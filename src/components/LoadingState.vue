<script setup lang="ts">
import { Card, CardContent } from '@/components/ui/card'
import { Skeleton } from '@/components/ui/skeleton'
import { Loader2 } from 'lucide-vue-next'

interface Props {
  message?: string
  fullScreen?: boolean
  variant?: 'card' | 'overlay' | 'skeleton'
}

withDefaults(defineProps<Props>(), {
  message: 'Loading...',
  fullScreen: false,
  variant: 'overlay',
})
</script>

<template>
  <div v-if="variant === 'skeleton'" class="space-y-4">
    <Skeleton class="h-24 w-full" />
    <Skeleton class="h-24 w-full" />
    <Skeleton class="h-24 w-full" />
  </div>

  <Card v-else-if="variant === 'card'">
    <CardContent class="flex flex-col items-center justify-center gap-4 py-12">
      <Loader2 class="h-8 w-8 animate-spin text-primary" />
      <p class="text-sm text-muted-foreground">{{ message }}</p>
    </CardContent>
  </Card>

  <div
    v-else
    :class="[
      'flex flex-col items-center justify-center gap-4',
      fullScreen ? 'fixed inset-0 bg-background/80 backdrop-blur-sm z-50' : 'py-12',
    ]"
  >
    <Loader2 class="h-8 w-8 animate-spin text-primary" />
    <p class="text-sm text-muted-foreground">{{ message }}</p>
  </div>
</template>
