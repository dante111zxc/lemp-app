import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue'),
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('../views/About.vue'),
  },
  // {
  //   path: '/dashboard',
  //   component: Home,
  //   children: [
  //     {
  //       path: '',
  //       name: 'Dashboard',
  //       component: () => import('../views/dashboard/Dashboard.vue'),
  //       meta: { title: 'Dashboard' },
  //     },
  //     {
  //       path: 'users',
  //       name: 'Users',
  //       component: () => import('../views/dashboard/Users.vue'),
  //       meta: { title: 'Users' },
  //     },
  //     {
  //       path: 'analytics',
  //       name: 'Analytics',
  //       component: () => import('../views/dashboard/Analytics.vue'),
  //       meta: { title: 'Analytics' },
  //     },
  //     {
  //       path: 'settings',
  //       name: 'Settings',
  //       component: () => import('../views/dashboard/Settings.vue'),
  //       meta: { title: 'Settings' },
  //     },
  //   ],
  // },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
