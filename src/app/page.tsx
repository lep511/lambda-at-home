// app/page.tsx
import dynamic from 'next/dynamic'
import Image from 'next/image'

// ISR: rebuild this page at most once every 60 seconds
export const revalidate = 60

// Only load the uploader on the client—never attempt to SSR it
const ImageUploader = dynamic(
  () => import('./components/ImageUploader')
)

export default function HomePage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex flex-col">
      {/* Header */}
      <header className="w-full px-4 sm:px-6 lg:px-8 py-4 sm:py-6">
        <div className="max-w-4xl mx-auto flex flex-col sm:flex-row items-center justify-center sm:justify-start gap-3 sm:gap-4">
          <Image
            src="/logo.png"
            alt="Logo"
            width={48}
            height={48}
            className="w-10 h-10 sm:w-12 sm:h-12"
            priority
          />
          <h1 className="text-2xl sm:text-3xl lg:text-4xl font-bold text-gray-800 text-center sm:text-left">
            Image Uploader
          </h1>
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 w-full px-4 sm:px-6 lg:px-8 py-4 sm:py-8">
        <div className="max-w-4xl mx-auto">
          <ImageUploader />
        </div>
      </main>

      <div className="h-4 sm:h-8" />
    </div>
  )
}
