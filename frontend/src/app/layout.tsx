import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Image Upload Studio",
  description: "Upload your images to S3 with rich metadata",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased bg-gradient-to-br from-gray-50 to-gray-100 min-h-screen`}
      >
        {/* Mobile-first responsive container */}
        <div className="min-h-screen flex flex-col p-2 sm:p-4 md:p-6">
          <div className="bg-white rounded-lg sm:rounded-xl md:rounded-2xl shadow-lg sm:shadow-xl flex-1 flex flex-col overflow-hidden">
            
            {/* Main content area */}
            <main className="flex-1 px-4 py-6 sm:px-6 sm:py-8 md:px-8 md:py-10">
              <div className="max-w-4xl mx-auto">
                {children}
              </div>
            </main>
            
            {/* Footer */}
            <footer className="border-t border-gray-100 px-4 py-4 sm:px-6 sm:py-6 md:px-8">
              <div className="max-w-4xl mx-auto text-center">
                <p className="text-xs sm:text-sm text-gray-500 leading-relaxed">
                  Built with ❤️ using{' '}
                  <a 
                    href="https://nextjs.org" 
                    target="_blank" 
                    rel="noopener noreferrer" 
                    className="underline hover:text-gray-700 transition-colors"
                  >
                    Next.js
                  </a>{' '}
                  and{' '}
                  <a 
                    href="https://aws.amazon.com/s3/" 
                    target="_blank" 
                    rel="noopener noreferrer" 
                    className="underline hover:text-gray-700 transition-colors"
                  >
                    Amazon S3
                  </a>
                </p>
              </div>
            </footer>
          </div>
        </div>
      </body>
    </html>
  );
}
