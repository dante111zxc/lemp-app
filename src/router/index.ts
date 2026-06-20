import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Dashboard',
    component: () => import('../views/Dashboard.vue'),
    meta: { title: 'Dashboard' },
  },
  {
    path: '/nginx',
    name: 'Nginx',
    component: () => import('../views/Nginx.vue'),
    meta: { title: 'Nginx' },
  },
  {
    path: '/mysql',
    name: 'MySQL',
    component: () => import('../views/Mysql.vue'),
    meta: { title: 'MySQL' },
  },
  {
    path: '/redis',
    name: 'Redis',
    component: () => import('../views/Redis.vue'),
    meta: { title: 'Redis' },
  },
  {
    path: '/mailpit',
    name: 'Mailpit',
    component: () => import('../views/Mailpit.vue'),
    meta: { title: 'Mailpit' },
  },
  {
    path: '/php',
    name: 'PHP',
    component: () => import('../views/Php.vue'),
    meta: { title: 'PHP' },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
