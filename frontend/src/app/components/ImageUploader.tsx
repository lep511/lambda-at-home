'use client';

import React, { useState, useCallback, useRef } from 'react';
import Image from 'next/image';
import {
  Upload,
  X,
  Check,
  FileText,
  AlertCircle,
  Image as ImageIcon,
  Loader2,
} from 'lucide-react';

interface FileError {
  fileName: string
  fileSize: string
  fileType: string
  error: string
}

interface FileData {
  id: string
  file: File
  preview: string
  metadata: {
    description: string
  }
  error: string | null
  uploaded: boolean
  uploadResult?: any
}
interface SampleImage {
  id: string;
  name: string;
  description: string;
  url: string;
}

interface SampleImagesSectionProps {
  onSampleSelect: (sample: SampleImage) => void;
  hasFile: boolean;
}

// Create placeholder images using data URLs or use placeholder services
const SAMPLE_IMAGES: SampleImage[] = [
  {
    id: 'sample-1',
    name: 'mountain-landscape.png',
    url: 'https://images.unsplash.com/photo-1635165213424-284652d40987?w=400&h=300&fit=crop',
    description: 'Beautiful mountain landscape with morning mist'
  },
  {
    id: 'sample-2',
    name: 'city-sunset.jpg',
    url: 'https://images.unsplash.com/photo-1506905925346-21bda4d32df4?w=400&h=300&fit=crop',
    description: 'Urban skyline during golden hour'
  },
  {
    id: 'sample-3',
    name: 'ocean-waves.jpg',
    url: 'https://images.unsplash.com/photo-1505142468610-359e7d316be0?w=400&h=300&fit=crop',
    description: 'Peaceful ocean waves at sunset'
  }
];

// Custom error type for upload errors
interface UploadError extends Error {
  showRemoveButton?: boolean
}

// ===== API GATEWAY SERVICE MODULE =====
interface UploadResponse {
  url: string;
  key: string;
  uploadId: string;
  timestamp: string;
  size: number;
  contentType: string;
  etag: string;
  metadata: any;
  s3_response: {
    bucket: string;
    key: string;
    uploaded: boolean;
  };
}

// ===== UTILITIES MODULE =====
const FileValidator = {
  validateFile: (file: File): string | null => {
    const validTypes = ['image/png', 'image/jpeg', 'image/webp'];
    const maxSize = 50 * 1024 * 1024; // 50MB
    
    if (!validTypes.includes(file.type)) {
      return 'Invalid file type. Please upload PNG, JPEG, or WEBP files.';
    }
    
    if (file.size > maxSize) {
      return 'File size too large. Maximum size is 50MB.';
    }
    
    return null;
  },

  formatFileSize: (bytes: number): string => {
    return (bytes / 1024 / 1024).toFixed(2);
  },

  createFileData: (file: File, metadata?: any): FileData => ({
    id: Math.random().toString(36).substr(2, 9),
    file,
    preview: URL.createObjectURL(file),
    metadata: metadata || {
      description: ''
    },
    error: null,
    uploaded: false
  }),

  // Updated utility to create a File object from a URL
  createFileFromURL: async (imageUrl: string, filename: string): Promise<File> => {
    try {
      const response = await fetch(imageUrl);
      const blob = await response.blob();
      
      // Determine MIME type from filename extension
      const extension = filename.toLowerCase().split('.').pop();
      let mimeType = 'image/jpeg'; // default
      
      if (extension === 'png') {
        mimeType = 'image/png';
      } else if (extension === 'webp') {
        mimeType = 'image/webp';
      }
      
      return new File([blob], filename, { type: mimeType });
    } catch (error) {
      console.error('Error creating file from URL:', error);
      throw new Error(`Failed to load image: ${filename}`);
    }
  }
};

// ===== API GATEWAY UPLOAD SERVICE =====
const APIGatewayService = {
  uploadToAPIGateway: async (file: File, metadata: any): Promise < UploadResponse > => {
    const apiUrl = process.env.NEXT_PUBLIC_API_BASE_URL;
    if (!apiUrl) {
      throw new Error('API_BASE_URL environment variable is not defined');
    }

    const metadataDesc = metadata.description || '';
    let metadataDescSanitize;

    if (metadataDesc !== '') {
      metadataDescSanitize = metadataDesc
          .split('\n')
          .map((line: string) => line.trim())
          .filter((line: string) => line !== '')
          .join(' ')
          .replace(/[^ \t!-~]/g, '') // Remove any character that's not space, tab, or printable ASCII
          .trim();
    } else {
        metadataDescSanitize = 'Null';
    }
   
    try {
      const params = new URLSearchParams({
        filename: file.name,
        content_type: file.type,
        description: metadataDescSanitize
      });
      const uploadUrl = `${apiUrl}?${params}`;
        
      const uploadUrlResponse = await fetch(uploadUrl, {
        method: 'GET',
        headers: {},
      });

      if (!uploadUrlResponse.ok) {
        throw new Error(`Error getting URL: ${uploadUrlResponse.status}`);
      }

      const { bucket_name, event_id, upload_url, file_key,  } = await uploadUrlResponse.json();
      console.log('Upload URL received:', upload_url);

      const uploadResponse = await fetch(upload_url, {
        method: 'PUT',
        body: file,
        headers: {
          'Content-Type': file.type,
          'x-amz-meta-file-name': file.name,
          'x-amz-meta-description': metadataDescSanitize
        }
      });

      if (!uploadResponse.ok) {
        throw new Error(`Error uploading file: ${uploadResponse.status}`);
      }

      console.log('File successfully uploaded');

      return {
        url: upload_url,
        key: file_key,
        uploadId: event_id,
        timestamp: new Date().toISOString(),
        size: file.size,
        contentType: file.type,
        metadata: metadata,
        etag: uploadResponse.headers.get('ETag') || '',
        s3_response: {
          bucket: bucket_name,
          key: file_key,
          uploaded: true
        }
      };
    } catch (error) {
      console.error('Upload error:', error);

      if (error instanceof Error) {
        const uploadError: UploadError = new Error(`Upload failed: ${error.message}`);
        uploadError.showRemoveButton = true;
        throw uploadError;
      } else {
        const uploadError: UploadError = new Error('Upload failed: Unknown error');
        uploadError.showRemoveButton = true;
        throw uploadError;
      }
    }
  },

  // Alternative method using ArrayBuffer for better binary control
  uploadWithArrayBuffer: async (file: File, metadata: any): Promise < UploadResponse > => {
    const apiUrl = process.env.NEXT_PUBLIC_API_BASE_URL;
    if (!apiUrl) {
      throw new Error('API_BASE_URL environment variable is not defined');
    }

    try {
      // Read file as ArrayBuffer to ensure binary integrity
      const arrayBuffer = await file.arrayBuffer();
      const uint8Array = new Uint8Array(arrayBuffer);

      console.log('File binary data:', {
        name: file.name,
        size: file.size,
        type: file.type,
        arrayBufferSize: arrayBuffer.byteLength,
        firstBytes: Array.from(uint8Array.slice(0, 16)).map(b => b.toString(16).padStart(2, '0')).join(' ')
      });

      // Create new File from ArrayBuffer to ensure binary integrity
      const binaryFile = new File([arrayBuffer], file.name, {
        type: file.type,
        lastModified: file.lastModified
      });

      const formData = new FormData();
      formData.append('file', binaryFile);
      formData.append('metadata', JSON.stringify(metadata));

      const response = await fetch(apiUrl, {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Upload failed with status: ${response.status} - ${errorText}`);
      }

      const result = await response.json();

      return {
        url: result.url || `${apiUrl}/files/${file.name}`,
        key: result.key || file.name,
        uploadId: result.uploadId,
        timestamp: result.timestamp,
        size: result.size,
        contentType: result.contentType,
        etag: result.etag,
        metadata: result.metadata,
        s3_response: result.s3_response,
        ...metadata,
        ...result
      };
    } catch (error) {
      console.error('Upload error:', error);

      if (error instanceof Error) {
        const uploadError: UploadError = new Error(`Upload failed: ${error.message}`);
        uploadError.showRemoveButton = true;
        throw uploadError;
      } else {
        const uploadError: UploadError = new Error('Upload failed: Unknown error');
        uploadError.showRemoveButton = true;
        throw uploadError;
      }
    }
  },

  // Method to verify file integrity after upload
  verifyUpload: async (originalFile: File, uploadedUrl: string): Promise < boolean > => {
    try {
      // Download the uploaded file
      const response = await fetch(uploadedUrl);
      if (!response.ok) {
        console.error('Failed to fetch uploaded file for verification');
        return false;
      }

      const uploadedArrayBuffer = await response.arrayBuffer();
      const originalArrayBuffer = await originalFile.arrayBuffer();

      // Compare sizes
      if (originalArrayBuffer.byteLength !== uploadedArrayBuffer.byteLength) {
        console.error('File size mismatch:', {
          original: originalArrayBuffer.byteLength,
          uploaded: uploadedArrayBuffer.byteLength
        });
        return false;
      }

      // Compare content (first and last 1KB for efficiency)
      const originalBytes = new Uint8Array(originalArrayBuffer);
      const uploadedBytes = new Uint8Array(uploadedArrayBuffer);

      const checkSize = Math.min(1024, originalBytes.length);

      // Check first 1KB
      for (let i = 0; i < checkSize; i++) {
        if (originalBytes[i] !== uploadedBytes[i]) {
          console.error('File content mismatch at byte:', i);
          return false;
        }
      }

      // Check last 1KB
      const startOffset = Math.max(0, originalBytes.length - checkSize);
      for (let i = 0; i < checkSize && startOffset + i < originalBytes.length; i++) {
        if (originalBytes[startOffset + i] !== uploadedBytes[startOffset + i]) {
          console.error('File content mismatch at byte:', startOffset + i);
          return false;
        }
      }

      console.log('File integrity verified successfully');
      return true;
    } catch (error) {
      console.error('Error verifying upload:', error);
      return false;
    }
  }
};

// ===== COMPONENTS MODULE =====
const AppHeader: React.FC = () => (
  <div className="text-center mb-12 pt-8">
    <h1 className="text-4xl font-bold text-gray-700 mb-4 tracking-tight">
      Image Upload Studio
    </h1>
    <p className="text-lg text-gray-600 font-light italic">
      Upload your image via API Gateway with rich metadata
    </p>
  </div>
);

// Updated Sample Images Section Component
interface SampleImagesSectionProps {
  onSampleSelect: (sample: SampleImage) => void
  hasFile: boolean
}

const SampleImagesSection: React.FC<SampleImagesSectionProps> = ({
  onSampleSelect,
  hasFile,
}) => {
  const [loadingStates, setLoadingStates] = useState<Record<string, boolean>>(
    Object.fromEntries(SAMPLE_IMAGES.map((img) => [img.id, true]))
  );
  const [errorStates, setErrorStates] = useState<Record<string, boolean>>({});

  const handleImageLoad = useCallback((id: string) => {
    setLoadingStates((prev) => ({ ...prev, [id]: false }));
  }, []);

  const handleImageError = useCallback((id: string) => {
    setLoadingStates((prev) => ({ ...prev, [id]: false }));
    setErrorStates((prev) => ({ ...prev, [id]: true }));
  }, []);

  const handleSampleClick = useCallback(
    (sample: SampleImage) => {
      if (!loadingStates[sample.id] && !errorStates[sample.id]) {
        onSampleSelect(sample);
      }
    },
    [loadingStates, errorStates, onSampleSelect]
  );

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent, sample: SampleImage) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        handleSampleClick(sample);
      }
    },
    [handleSampleClick]
  );

  if (hasFile) return null;

  return (
    <div className="mb-8">
      <h2 className="text-2xl font-bold text-center text-black mb-4 flex items-center justify-center">
        <ImageIcon className="w-6 h-6 mr-2" /> Try Sample Images
      </h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {SAMPLE_IMAGES.map((sample) => (
          <div
            key={sample.id}
            role="button"
            tabIndex={0}
            aria-label={sample.name}
            onClick={() => handleSampleClick(sample)}
            className={`
              bg-white rounded-xl shadow-lg overflow-hidden
              ${errorStates[sample.id] ? 'opacity-50 cursor-not-allowed' : ''}
              ${loadingStates[sample.id] ? 'cursor-wait' : 'cursor-pointer hover:scale-105'}
              transition-transform duration-200
            `}
          >
            <div className="relative h-48 w-full bg-gray-100">
              {loadingStates[sample.id] && (
                <div className="absolute inset-0 flex items-center justify-center z-10">
                  <Loader2 className="animate-spin w-8 h-8 text-blue-600" />
                </div>
              )}

              <Image
                src={sample.url}
                alt={sample.description}
                fill
                className={`
                  object-cover transition-opacity duration-300
                  ${loadingStates[sample.id] || errorStates[sample.id] ? 'opacity-0' : 'opacity-100'}
                `}
                onLoadingComplete={() => handleImageLoad(sample.id)}
                onError={() => handleImageError(sample.id)}
              />

              {!loadingStates[sample.id] && !errorStates[sample.id] && (
                <div className="absolute inset-0 flex items-center justify-center opacity-0 hover:opacity-100 transition-opacity">
                  <div className="bg-white bg-opacity-90 rounded-full p-3">
                    <Upload className="w-6 h-6 text-blue-600" />
                  </div>
                </div>
              )}
            </div>
            <div className="p-4">
              <h3 className="italic truncate text-black">{sample.name}</h3>
              <p className="text-sm text-black line-clamp-2">{sample.description}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

interface ValidationErrorWindowProps {
  validationError: FileError | null
  onClose: () => void
}

const ValidationErrorWindow: React.FC<ValidationErrorWindowProps> = ({ validationError, onClose }) => {
  if (!validationError) return null;

  return (
    <div className="mb-8 bg-red-50 border-l-4 border-red-400 rounded-r-xl shadow-lg">
      <div className="p-6">
        <div className="flex items-center mb-4">
          <AlertCircle className="w-6 h-6 text-red-600 mr-3" />
          <h3 className="text-lg font-bold text-red-800">
            Invalid File Detected
          </h3>
          <button
            onClick={onClose}
            className="btn btn-ghost btn-sm"
          >
            <X className="w-5 h-5" />
          </button>
        </div>
        
        <div className="bg-white rounded-lg p-4 border border-red-200 mb-4">
          <div className="flex justify-between items-start mb-2">
            <div className="font-bold text-red-800 truncate flex-1 mr-4">
              {validationError.fileName}
            </div>
            <div className="text-sm text-red-600 font-light">
              {validationError.fileSize} MB
            </div>
          </div>
          <div className="text-sm text-red-600 mb-2">
            <span className="italic">File type: </span>
            <span className="font-mono bg-red-100 px-2 py-1 rounded">
              {validationError.fileType}
            </span>
          </div>
          <div className="text-sm text-red-700 italic">
            {validationError.error}
          </div>
        </div>
        
        <SupportedFormatsInfo />
      </div>
    </div>
  );
};

const SupportedFormatsInfo: React.FC = () => (
  <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
    <h4 className="font-bold text-blue-800 mb-2 flex items-center">
      <FileText className="w-4 h-4 mr-2" />
      Supported File Formats
    </h4>
    <div className="grid grid-cols-1 md:grid-cols-3 gap-3 text-sm">
      <div className="flex items-center text-blue-700">
        <div className="w-2 h-2 bg-blue-500 rounded-full mr-2"></div>
        <span className="font-bold">PNG</span>
        <span className="italic ml-1">(.png)</span>
      </div>
      <div className="flex items-center text-blue-700">
        <div className="w-2 h-2 bg-blue-500 rounded-full mr-2"></div>
        <span className="font-bold">JPEG</span>
        <span className="italic ml-1">(.jpg, .jpeg)</span>
      </div>
      <div className="flex items-center text-blue-700">
        <div className="w-2 h-2 bg-blue-500 rounded-full mr-2"></div>
        <span className="font-bold">WEBP</span>
        <span className="italic ml-1">(.webp)</span>
      </div>
    </div>
    <div className="mt-3 text-xs text-blue-600 italic">
      Maximum file size: 10MB per image
    </div>
  </div>
);

interface UploadZoneProps {
  dragActive: boolean
  onDragHandlers: {
    handleDrag: (e: React.DragEvent) => void
    handleDrop: (e: React.DragEvent) => void
  }
  onFileSelect: (e: React.ChangeEvent<HTMLInputElement>) => void
  fileInputRef: React.RefObject<HTMLInputElement | null>
  hasFile: boolean
}

const UploadZone: React.FC<UploadZoneProps> = ({ dragActive, onDragHandlers, onFileSelect, fileInputRef, hasFile }) => {
  if (hasFile) return null;

  return (
    <div 
      className={`relative border-2 border-dashed rounded-2xl p-12 mb-8 text-center transition-all duration-300 cursor-pointer ${
        dragActive 
          ? 'border-blue-400 bg-blue-50 scale-102' 
          : 'border-gray-300 bg-white hover:border-gray-400 hover:bg-gray-50'
      }`}
      onDragEnter={onDragHandlers.handleDrag}
      onDragLeave={onDragHandlers.handleDrag}
      onDragOver={onDragHandlers.handleDrag}
      onDrop={onDragHandlers.handleDrop}
      onClick={() => fileInputRef.current?.click()}
    >
      <input
        ref={fileInputRef}
        type="file"
        accept=".png,.jpg,.jpeg,.webp"
        onChange={onFileSelect}
        className="hidden"
      />
      
      <div className={`transition-all duration-300 ${dragActive ? 'scale-110' : ''}`}>
        <Upload className="w-16 h-16 text-gray-800 mx-auto mb-4" />
        <h3 className="text-2xl font-bold text-black mb-2">
          Drop your image here
        </h3>
        <p className="text-gray-900 italic mb-4">
          or click to browse your files
        </p>
        <div className="text-sm text-gray-900 font-light">
          Supports PNG, JPEG, WEBP • Maximum 10MB per file
        </div>
      </div>
    </div>
  );
};

interface FileCardProps {
  fileData: FileData
  onUpdateMetadata: (field: string, value: string) => void
  onRemove: () => void
  onUpload: () => void
  isUploading: boolean
}

const FileCard: React.FC<FileCardProps> = ({ fileData, onUpdateMetadata, onRemove, onUpload, isUploading }) => {
  const { file, preview, metadata, error, uploaded } = fileData;

  return (
    <div className={`bg-white rounded-2xl shadow-lg p-6 transition-all duration-300 hover:shadow-xl mb-8 ${
      uploaded ? 'ring-2 ring-green-200' : error ? 'ring-2 ring-red-200' : ''
    }`}>
      <div className="flex flex-col lg:flex-row gap-6">
        <ImagePreview 
          preview={preview}
          fileName={file.name}
          fileSize={FileValidator.formatFileSize(file.size)}
          uploaded={uploaded}
          onRemove={onRemove}
        />
        
        <MetadataForm
          metadata={metadata}
          onUpdate={onUpdateMetadata}
          disabled={uploaded}
        />
        
        <FileStatus
          error={error}
          uploaded={uploaded}
          isUploading={isUploading}
          onUpload={onUpload}
          onRemove={onRemove}
        />
      </div>
    </div>
  );
};

interface ImagePreviewProps {
  preview: string
  fileName: string
  fileSize: string
  uploaded: boolean
  onRemove: () => void
}

const ImagePreview: React.FC<ImagePreviewProps> = ({ preview, fileName, fileSize, uploaded, onRemove }) => (
  <div className="lg:w-48 flex-shrink-0">
    <div className="relative group">
      <img
        src={preview}
        alt={fileName}
        className="w-full h-48 object-cover rounded-xl transition-transform duration-300 group-hover:scale-105"
      />
      <button
        onClick={onRemove}
        className="absolute top-2 right-2 bg-red-500 hover:bg-red-600 text-white rounded-full p-2 opacity-0 group-hover:opacity-100 transition-all duration-200 hover:scale-110"
      >
        <X className="w-4 h-4" />
      </button>
      {uploaded && (
        <div className="absolute inset-0 bg-green-500 bg-opacity-20 rounded-xl flex items-center justify-center">
          <Check className="w-12 h-12 text-green-600" />
        </div>
      )}
    </div>
    <div className="mt-3">
      <div className="font-bold text-gray-900 truncate">{fileName}</div>
      <div className="text-sm text-gray-800 font-light">{fileSize} MB</div>
    </div>
  </div>
);

interface MetadataFormProps {
  metadata: {
    description: string
  }
  onUpdate: (field: string, value: string) => void
  disabled: boolean
}

const MetadataForm: React.FC<MetadataFormProps> = ({ metadata, onUpdate, disabled }) => (
  <div className="flex-1 space-y-4">
    <div>
      <label className="flex items-center text-sm font-bold text-black mb-2">
        <FileText className="w-4 h-4 mr-2" />
        Description
      </label>
      <textarea
        value={metadata.description}
        onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => onUpdate('description', e.target.value)}
        placeholder="Describe your image..."
        className="w-full p-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 font-light placeholder:text-gray-500 text-black"
        rows={3}
        disabled={disabled}
      />
    </div>
  </div>
);

interface FileStatusProps {
  error: string | null
  uploaded: boolean
  isUploading: boolean
  onUpload: () => void
  onRemove: () => void
}

const FileStatus: React.FC<FileStatusProps> = ({ error, uploaded, isUploading, onUpload, onRemove }) => (
  <div className="flex justify-between items-center pt-4 lg:flex-col lg:justify-end lg:w-32">
    <div className="flex-1 lg:mb-4">
      {error && (
        <div className="flex items-center text-red-600 text-sm mb-2">
          <AlertCircle className="w-4 h-4 mr-2" />
          <span className="italic">{error}</span>
        </div>
      )}
      {uploaded && (
        <div className="flex items-center text-green-600 text-sm">
          <Check className="w-4 h-4 mr-2" />
          <span className="italic">Successfully uploaded</span>
        </div>
      )}
      {isUploading && (
        <div className="flex items-center text-blue-600 text-sm">
          <div className="animate-spin rounded-full h-4 w-4 border-2 border-blue-600 border-t-transparent mr-2"></div>
          <span className="italic">Uploading...</span>
        </div>
      )}
    </div>

    <div className="flex flex-col gap-2">
      {!uploaded && !error && (
        <button
          onClick={onUpload}
          disabled={isUploading}
          className={`btn btn-primary ${isUploading ? 'btn-loading' : ''}`}
        >
          {isUploading ? 'Uploading...' : 'Upload'}
        </button>
      )}
      
      {/* Add upload another button for successful uploads */}
      {uploaded && (
        <button
          onClick={onRemove}
          className="btn btn-success"
        >
          <Upload className="w-4 h-4 mr-1" />
          Upload Another
        </button>
      )}
      
      {error && (
        <button
          onClick={onRemove}
          className="btn btn-danger"
        >
          <X className="w-4 h-4 mr-1" />
          Remove
        </button>
      )}
    </div>
  </div>
);

interface UploadedFileDisplayProps {
  uploadedFile: FileData | null
}

const UploadedFileDisplay: React.FC<UploadedFileDisplayProps> = ({ uploadedFile }) => {
  if (!uploadedFile) return null;

  return (
    <div>
      <h2 className="text-2xl font-bold text-black mb-6">
        Successfully Uploaded
      </h2>
      <div className="bg-green-50 border border-green-200 rounded-xl p-4 flex items-center space-x-4">
        <Check className="w-6 h-6 text-green-600" />
        <div className="flex-1">
          <div className="font-bold text-green-800">{uploadedFile.file.name}</div>
          <div className="text-sm text-green-600 italic">
            {uploadedFile.metadata.description && `"${uploadedFile.metadata.description}"`}
          </div>
        </div>
      </div>
    </div>
  );
};

// ===== HOOKS MODULE =====
interface UseFileUploadReturn {
  file: FileData | null
  isUploading: boolean
  uploadedFile: FileData | null
  validationError: FileError | null
  setValidationError: React.Dispatch<React.SetStateAction<FileError | null>>
  handleFile: (newFile: File, metadata?: any) => void
  updateMetadata: (field: string, value: string) => void
  removeFile: () => void
  uploadFile: () => Promise<void>
  handleSampleSelect: (sample: SampleImage) => void
}

const useFileUpload = (): UseFileUploadReturn => {
  const [file, setFile] = useState<FileData | null>(null);
  const [isUploading, setIsUploading] = useState<boolean>(false);
  const [uploadedFile, setUploadedFile] = useState<FileData | null>(null);
  const [validationError, setValidationError] = useState<FileError | null>(null);

  const handleFile = useCallback((newFile: File, metadata?: any) => {
    const error = FileValidator.validateFile(newFile);
    if (error) {
      setValidationError({
        fileName: newFile.name,
        fileSize: FileValidator.formatFileSize(newFile.size),
        fileType: newFile.type || 'Unknown',
        error: error
      });
      setTimeout(() => setValidationError(null), 8000);
    } else {
      if (file) {
        URL.revokeObjectURL(file.preview);
      }
      setFile(FileValidator.createFileData(newFile, metadata));
      setValidationError(null);
    }
  }, [file]);

  const handleSampleSelect = useCallback(async (sample: SampleImage) => {
    try {
      const sampleFile = await FileValidator.createFileFromURL(sample.url, sample.name);
      const sampleMetadata = {
        description: sample.description
      };
      handleFile(sampleFile, sampleMetadata);
    } catch (error) {
      console.error('Error loading sample image:', error);
      setValidationError({
        fileName: sample.name,
        fileSize: 'Unknown',
        fileType: 'Unknown',
        error: 'Failed to load sample image. Please check your internet connection.'
      });
    }
  }, [handleFile]);

  const updateMetadata = useCallback((field: string, value: string) => {
    if (file) {
      setFile(prev => prev ? { ...prev, metadata: { ...prev.metadata, [field]: value } } : null);
    }
  }, [file]);

  const removeFile = useCallback(() => {
    if (file) {
      URL.revokeObjectURL(file.preview);
      setFile(null);
    }
    // Clear uploaded file when removing to show upload zone again
    setUploadedFile(null);
  }, [file]);

  const uploadFile = useCallback(async () => {
    if (!file) return;
    
    setIsUploading(true);
    setFile(prev => prev ? { ...prev, error: null } : null); // Clear previous errors
    
    try {
      const result = await APIGatewayService.uploadToAPIGateway(file.file, file.metadata);
      
      const uploadedFileData = { ...file, uploaded: true, uploadResult: result };
      setUploadedFile(uploadedFileData);
      setFile(uploadedFileData); // Update the card to show its success state
    } catch (error) {
      console.error('Upload failed:', error);
      const errorMessage = error instanceof Error ? error.message : 'An unknown upload error occurred.';
      setFile(prev => prev ? { ...prev, error: errorMessage } : null);
    } finally {
      setIsUploading(false);
    }
  }, [file]);

  return {
    file,
    isUploading,
    uploadedFile,
    validationError,
    setValidationError,
    handleFile,
    updateMetadata,
    removeFile,
    uploadFile,
    handleSampleSelect
  };
};

// ===== MAIN COMPONENT =====
const ImageUploader: React.FC = () => {
  const {
    file,
    isUploading,
    uploadedFile,
    validationError,
    setValidationError,
    handleFile,
    updateMetadata,
    removeFile,
    uploadFile,
    handleSampleSelect
  } = useFileUpload();

  const fileInputRef = useRef<HTMLInputElement>(null);
  const [dragActive, setDragActive] = useState(false);

  const handleDrag = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === 'dragenter' || e.type === 'dragover') {
      setDragActive(true);
    } else if (e.type === 'dragleave') {
      setDragActive(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      handleFile(e.dataTransfer.files[0]);
    }
  };

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    if (e.target.files && e.target.files[0]) {
      handleFile(e.target.files[0]);
      // Reset input value to allow re-uploading the same file
      if(e.target) e.target.value = '';
    }
  };

  return (
    <div className="bg-gray-50 min-h-screen">
      <div className="container mx-auto px-4 py-8 max-w-4xl">
        <AppHeader />
        
        <ValidationErrorWindow 
          validationError={validationError} 
          onClose={() => setValidationError(null)} 
        />
        
        <UploadZone
          dragActive={dragActive}
          onDragHandlers={{ handleDrag, handleDrop }}
          onFileSelect={handleFileSelect}
          fileInputRef={fileInputRef}
          hasFile={!!file}
        />

        <SampleImagesSection 
          onSampleSelect={handleSampleSelect}
          hasFile={!!file}
        />

        {file && (
          <FileCard
            fileData={file}
            onUpdateMetadata={updateMetadata}
            onRemove={removeFile}
            onUpload={uploadFile}
            isUploading={isUploading}
          />
        )}
        
        {uploadedFile && !file && (
           <UploadedFileDisplay uploadedFile={uploadedFile} />
        )}
      </div>
    </div>
  );
};

// ===== ROOT APP COMPONENT =====
const App: React.FC = () => {
  return <ImageUploader />;
};

export default App;