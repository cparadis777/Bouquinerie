<template>
  <nav>
    <ul>
      <li>
        <RouterLink to="/">
          <Icon icon="mdi-view-dashboard" class="nav-icon" />
          <span>Dashboard</span>
        </RouterLink>
      </li>
      <li>
        <RouterLink to="/books">
          <Icon icon="mdi-book-open-variant" class="nav-icon" />
          <span>Books</span>
        </RouterLink>
      </li>
      <li>
        <RouterLink to="/authors">
          <Icon icon="mdi-account-group" class="nav-icon" />
          <span>Authors</span>
        </RouterLink>
      </li>
      <li>
        <RouterLink to="/series">
          <Icon icon="mdi-book-multiple" class="nav-icon" />
          <span>Series</span>
        </RouterLink>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
  import { RouterLink } from 'vue-router';
  import { Icon } from '@iconify/vue';
</script>

<style lang="css" scoped>
  ul {
    list-style: none;
    padding-left: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  li a {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 12px 16px;
    border-radius: 8px;
    text-decoration: none;
    color: inherit;
    font-weight: 600;
    transition: background 0.15s, opacity 0.15s;
    opacity: 0.6;
  }

  li a:hover {
    color: var(--primary);
    opacity: 0.8;
  }

  li a.router-link-active {
    color: var(--accent);
    opacity: 1;
  }

  .nav-icon {
    font-size: 24px;
  }
</style>
