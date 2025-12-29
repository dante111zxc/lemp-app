<template>
  <div class="home">
    <h1>Trang chủ</h1>
    <p>Chào mừng đến với ứng dụng Tauri + Vue 3!</p>
    <router-link to="/about">Đi đến trang About</router-link>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.home {
  padding: 20px;
}

h1 {
  color: #42b983;
}

a {
  color: #42b983;
  text-decoration: none;
  padding: 10px;
  display: inline-block;
}

a:hover {
  text-decoration: underline;
}
</style>
