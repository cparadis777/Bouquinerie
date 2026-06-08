import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

let authInitialized = false

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/login',
      name: 'login',
      meta: { blank: true },
      component: () => import('../views/LoginView.vue'),
    },
    {
      path: '/register',
      name: 'register',
      meta: { blank: true },
      component: () => import('../views/RegisterView.vue'),
    },
    {
      path: '/',
      name: 'dashboard',
      component: () => import('../views/DashboardView.vue'),
    },
    {
      path: '/books',
      name: 'books',
      component: () => import('../views/DashboardView.vue'),
    },
    {
      path: '/books/:id',
      name: 'book-detail',
      component: () => import('../views/BookDetail.vue'),
    },
    {
      path: '/authors',
      name: 'authors',
      component: () => import('../views/DashboardView.vue'),
    },
    // {
    //   path: '/authors/:id',
    //   name: 'author-detail',
    //   component: () => import('../views/AuthorDetailView.vue'),
    // },
    {
      path: '/series',
      name: 'series',
      component: () => import('../views/DashboardView.vue'),
    },
    // {
    //   path: '/series/:id',
    //   name: 'series-detail',
    //   component: () => import('../views/SeriesDetailView.vue'),
    // },
    {
      path: '/dev/palettes',
      name: 'palettes',
      component: () => import('../views/dev/Palette.vue'),
    },
  ],
})

router.beforeEach(async (to, _from) => {
  const auth = useAuthStore()

  if (!authInitialized) {
    authInitialized = true
    await auth.checkAuth()
  }

  if (auth.isAuthenticated && (to.name === 'login' || to.name === 'register')) {
    return { name: 'dashboard' }
  }

  if (!auth.isAuthenticated && to.name !== 'login' && to.name !== 'register') {
    return { name: 'login' }
  }
})

export default router
