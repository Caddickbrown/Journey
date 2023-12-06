import { ref } from 'vue';

const initializeTauri = (app) => {
  // Set up Tauri and expose it to the global properties
  app.config.globalProperties.$tauri = ref(window.__TAURI__);
};

export { initializeTauri };
