import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

import './assets/main.css'
import { useStoreCfg, useWatchCfgStore } from './stores/config'
import { listen } from '@tauri-apps/api/event'

const app = createApp(App)

app.use(createPinia())
app.use(router)

app.mount('#app')
