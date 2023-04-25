import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { startLoop, stopLoop } from './stores/clipdoard'

import App from './App.vue'
import router from './router'

import './assets/main.css'

const app = createApp(App)

app.use(createPinia())
app.use(router)

startLoop(10000)


app.mount('#app')
