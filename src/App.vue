<script setup lang="ts">
import { useTheme } from '@/composables/useTheme'

// Initialize theme
useTheme()
</script>

<template>
  <div id="app" class="min-h-screen bg-background text-foreground">
    <router-view />
  </div>
</template>
