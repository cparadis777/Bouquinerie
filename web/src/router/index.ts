import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: () => import('../views/DashboardView.vue'),
    },
    {
      path: '/books',
      name: 'books',
      component: () => import('../views/BooksView.vue'),
    },
    {
      path: '/books/:id',
      name: 'book-detail',
      component: () => import('../views/BookDetailView.vue'),
    },
    {
      path: '/authors',
      name: 'authors',
      component: () => import('../views/AuthorsView.vue'),
    },
    {
      path: '/authors/:id',
      name: 'author-detail',
      component: () => import('../views/AuthorDetailView.vue'),
    },
    {
      path: '/series',
      name: 'series',
      component: () => import('../views/SeriesView.vue'),
    },
    {
      path: '/series/:id',
      name: 'series-detail',
      component: () => import('../views/SeriesDetailView.vue'),
    },
  ],
})

export default router
