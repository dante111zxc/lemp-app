<template>
  <div class="about">
    <h1>Giới thiệu</h1>
    <p>Đây là trang giới thiệu của ứng dụng.</p>
    <router-link to="/">Quay lại trang chủ</router-link>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.about {
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
