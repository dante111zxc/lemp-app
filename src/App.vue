<script setup lang="ts"></script>

<template>
  <div id="app">
    <nav class="navbar">
      <div class="nav-links">
        <router-link to="/">Trang chủ</router-link>
        <router-link to="/about">Giới thiệu</router-link>
      </div>
    </nav>
    <main class="content">
      <router-view />
    </main>
  </div>
</template>
