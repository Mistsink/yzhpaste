import { createRouter, createWebHistory } from 'vue-router'
import ContentView from '../views/ContentView.vue'
import ConfigView from '../views/ConfigView.vue'
import { listen } from '@tauri-apps/api/event'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'content',
      component: () => import('../views/ConfigView.vue')
    },
    {
      path: '/config',
      name: 'config',
      component: ContentView
    }
  ]
})

export default router

listen('on-window-hide', () => router.push('/'))
