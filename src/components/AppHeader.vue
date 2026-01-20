<script setup lang="ts">
import { Sun, Moon } from 'lucide-vue-next'
import { Button } from '@/components/ui/button'
import { useTheme } from '@/composables/useTheme'

const { isDark, toggleTheme } = useTheme()
</script>

<template>
  <header
    class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
  >
    <div class="container mx-auto px-6 flex h-14 items-center">
      <div class="flex items-center gap-2">
        <div>
          <h1 class="text-lg font-semibold leading-none">Service Manager</h1>
          <p class="text-xs text-muted-foreground">Development Environment</p>
        </div>
      </div>

      <div class="ml-auto flex items-center gap-2">
        <Button variant="ghost" size="icon" @click="toggleTheme" aria-label="Toggle theme">
          <Sun v-if="isDark" class="h-5 w-5" />
          <Moon v-else class="h-5 w-5" />
        </Button>
      </div>
    </div>
  </header>
</template>
