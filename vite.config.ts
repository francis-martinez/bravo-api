import { defineConfig } from "vite";

export default defineConfig({
  server: {
    port: 3000,
    proxy: {
      "/api": {
        target: "https://bravo.bebalanced-dev.org",
        changeOrigin: true,
        secure: true,
      },
      "/auth-api": {
        target: "https://login.dev.paprwrk.co",
        changeOrigin: true,
        secure: true,
        rewrite: (path) => path.replace(/^\/auth-api/, ""),
      },
    },
  },
});
