import {fileURLToPath} from 'url'
import {dirname, join} from 'path'

const currentDir = dirname(fileURLToPath(import.meta.url))


// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-04-03',
  devtools: {enabled: true},
  ssr: false,
  alias: {
    '@vue/devtools-api': '@vue/devtools-api',
  },
  modules: [
    '@pinia/nuxt',
    '@nuxtjs/tailwindcss',
    '@vueuse/nuxt',
    '@nuxtjs/color-mode',
    '@nuxt/eslint',
    '@vee-validate/nuxt',
    '@nuxt/fonts',
    'shadcn-nuxt',
  ],

  vite: {
    // Better support for Tauri CLI output
    clearScreen: false,
    // https://github.com/tauri-apps/tauri/security/advisories/GHSA-2rcp-jvr4-r259
    define: {
      TAURI_PLATFORM: JSON.stringify(process.env.TAURI_PLATFORM),
      TAURI_ARCH: JSON.stringify(process.env.TAURI_ARCH),
      TAURI_FAMILY: JSON.stringify(process.env.TAURI_FAMILY),
      TAURI_PLATFORM_VERSION: JSON.stringify(
          process.env.TAURI_PLATFORM_VERSION
      ),
      TAURI_PLATFORM_TYPE: JSON.stringify(process.env.TAURI_PLATFORM_TYPE),
      TAURI_DEBUG: JSON.stringify(process.env.TAURI_DEBUG),
    },
    // Dev server configurations
    server: {
      // Tauri requires a consistent port
      strictPort: true,
      // Enables the development server to be discoverable by other devices for mobile development
      host: '0.0.0.0',
      hmr: {
        // Use websocket for mobile hot reloading
        protocol: 'ws',
        // Make sure it's available on the network
        host: '0.0.0.0',
        // Use a specific port for hmr
        port: 5183,
      },
    },
    optimizeDeps: {
      exclude: ['vee-validate'],
    },
  },

  srcDir: 'src',
  components: [
    {
      path: join(currentDir, './src/components/ui'),
      // this is required else Nuxt will autoImport `.ts` file
      extensions: ['.vue'],
    },
  ],
  shadcn: {
    /**
     * Prefix for all the imported component
     */
    prefix: '',
    /**
     * Directory that the component lives in.
     * @default "./components/ui"
     */
    componentDir: 'src/components/ui'
  },

  pinia: {
    storesDirs: ['src/stores/**'],
  },

  colorMode: {
    classSuffix: '',
  },
  fonts: {
    families: [
      {
        name: 'Montserrat',
        provider: 'fontsource',
        weights: ['400', '500', '600', '700', '800', '900'],
      },
    ],
  },

})