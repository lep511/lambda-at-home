import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Environment variables configuration
  env: {
    APP_VERSION: process.env.npm_package_version || '1.0.0',
  }  
};

export default nextConfig;

module.exports = {
  allowedDevOrigins: ['local-origin.dev', '*.local-origin.dev'],
  images: {
    domains: ['images.unsplash.com'],
  },
}