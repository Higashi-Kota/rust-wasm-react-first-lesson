import react from '@vitejs/plugin-react'
import { resolve } from 'node:path'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      // 開発時は utils のソースを直接参照
      '@internal/utils': resolve(__dirname, '../utils/src/index.ts'),
    },
  },
  optimizeDeps: {
    exclude: ['lucide-react', '@internal/utils'],
  },
  server: {
    fs: {
      // モノレポのファイルアクセスを許可
      allow: ['..'],
    },
  },
})
