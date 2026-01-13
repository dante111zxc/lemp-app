<script setup lang="ts">
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { AlertCircle } from 'lucide-vue-next'

interface Props {
  title?: string
  message?: string
  showRetry?: boolean
  variant?: 'card' | 'inline'
}

withDefaults(defineProps<Props>(), {
  title: 'Something went wrong',
  message: 'An error occurred while loading this content.',
  showRetry: true,
  variant: 'card',
})

const emit = defineEmits<{
  retry: []
}>()
</script>

<template>
  <Card v-if="variant === 'card'">
    <CardHeader class="text-center">
      <div class="flex justify-center mb-4">
        <div class="rounded-full bg-destructive/10 p-3">
          <AlertCircle class="h-8 w-8 text-destructive" />
        </div>
      </div>
      <CardTitle>{{ title }}</CardTitle>
      <CardDescription>{{ message }}</CardDescription>
    </CardHeader>
    <CardContent v-if="showRetry" class="flex justify-center">
      <Button @click="emit('retry')" variant="outline"> Try Again </Button>
    </CardContent>
  </Card>

  <div v-else class="flex flex-col items-center justify-center gap-4 py-12">
    <div class="rounded-full bg-destructive/10 p-3">
      <AlertCircle class="h-8 w-8 text-destructive" />
    </div>
    <div class="space-y-2 text-center">
      <h3 class="text-lg font-semibold">{{ title }}</h3>
      <p class="text-sm text-muted-foreground max-w-md">{{ message }}</p>
    </div>
    <Button v-if="showRetry" @click="emit('retry')" variant="outline"> Try Again </Button>
  </div>
</template>
