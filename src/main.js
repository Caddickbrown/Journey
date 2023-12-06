import { createApp } from 'vue';
import App from './App.vue';
import { initializeTauri } from './tauri.js'; // create a separate file for Tauri initialization

const app = createApp(App);
initializeTauri(app);

const vm = app.mount('#app');

// Use the mounted lifecycle hook to ensure Tauri is initialized
vm.$nextTick(() => {
  app.config.globalProperties.$tauri
    .invoke({
      cmd: 'new_entry',
    })
    .then((response) => {
      console.log(response);
    })
    .catch((error) => {
      console.error(error);
    });
});
