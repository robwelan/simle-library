/**
 * scripts/env.js
 * Centralized Environment Detection and Path Management
 */

export const ENV = {
  // 1. Local Node.js / NPM Dev Server
  isLocal:
    window.location.hostname === 'localhost' ||
    window.location.hostname === '127.0.0.1',

  // 2. GitHub Pages
  isGitHubPages:
    window.location.hostname.includes('github.io'),

  // 3. Dedicated App (Tauri, Electron, Capacitor, etc.)
  isNativeApp:
    !!window.__TAURI__ ||
    !!window.process?.versions?.electron ||
    !!window.Capacitor ||
    window.location.protocol === 'file:' ||
    window.location.protocol === 'app:',

  /**
   * Logic to determine where the WASM folder lives relative to index.html
   */
  getWasmDir() {
    if (this.isNativeApp) return './wasm';
    return this.isLocal ? '../wasm' : './wasm';
  },

  /**
   * Logic to determine where the books/content folder lives
   */
  getContentBase(projectPaths) {
    if (this.isLocal || this.isNativeApp) {
      return projectPaths.web_paths.local_books_base;
    }
    return projectPaths.web_paths.server_books_base;
  }
};
