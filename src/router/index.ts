import { createRouter, createWebHistory } from 'vue-router'
import ContentView from '../views/ContentView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'content',
      component: ContentView
    },
    {
      path: '/config',
      name: 'config',
      component: () => import('../views/ConfigView.vue')
    }
  ]
})

export default router
