import { ref, computed } from 'vue';
import { Store } from '@tauri-apps/plugin-store';
import { info, error as logError } from '@tauri-apps/plugin-log';
import type { AppConfig } from '~/types';
import { DEFAULT_CONFIG } from '~/types';

const store = new Store('.settings.dat');

export function useConfigStore() {
  const config = ref<AppConfig>(DEFAULT_CONFIG);

  async function loadConfig() {
    try {
      const storedConfig = await store.get('config');
      if (storedConfig) {
        config.value = storedConfig as AppConfig;
      } else {
        await store.set('config', DEFAULT_CONFIG);
      }
      applyConfig();
    } catch (error) {
      await logError(`Error initializing config: ${error}`);
    }
  }

  async function saveConfig() {
    try {
      await store.set('config', config.value);
      await store.save();
    } catch (error) {
      await logError(`Error saving config: ${error}`);
    }
  }

  async function setFont(font: string) {
    config.value.font = font;
    applyConfig();
    await saveConfig();
  }

  function applyConfig() {
    document.body.style.fontFamily = config.value.font;
  }

  loadConfig().catch(error =>
      logError(`Error loading config: ${error}`)
  );

  return {
    config: computed(() => config.value),
    setFont,
  };
}