import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
// Auto-detect Codespaces and set proxy target accordingly
const codespacesBackend = 'https://ominous-cod-r475jj7xj9jc695-8080.app.github.dev';
const isCodespaces = !!process.env.CODESPACES || !!process.env.GITHUB_CODESPACE_TOKEN || !!process.env.CODESPACE_NAME;
const backendTarget = isCodespaces ? codespacesBackend : 'http://localhost:8080';

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': backendTarget,
      '/health': backendTarget
    }
  }
})
