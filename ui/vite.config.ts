import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import solidSvg from "vite-plugin-solid-svg";

export default defineConfig({
  plugins: [solid(), solidSvg()],
  server: {
    proxy: {
      "/api/": "http://localhost:3000",
    },
  },
});
