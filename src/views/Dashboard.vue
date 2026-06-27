<script setup lang="ts">
import { useSystemInfo } from '@/composables/useSystemInfo'
import { formatBytes } from '@/utils/format'
import { Dot } from 'lucide-vue-next'
import { Progress } from '@/components/ui/progress'

const { systemInfo, loading, error } = useSystemInfo(3000)
</script>

<template>
  <div class="space-y-6">
    <div>
      <h1 class="text-2xl font-bold">Dashboard</h1>
      <p class="text-muted-foreground">System overview and resource usage</p>
    </div>

    <div v-if="loading" class="grid gap-4 md:grid-cols-1 lg:grid-cols-1">
      <div class="w-full max-w-full rounded-md border p-2 h-22">
        <div class="h-full w-full animate-pulse rounded bg-muted" />
      </div>

      <div class="w-full max-w-full rounded-md border p-2 h-22">
        <div class="h-full w-full animate-pulse rounded bg-muted" />
      </div>
    </div>

    <div
      v-else-if="error"
      class="rounded-lg border border-destructive/50 bg-destructive/10 p-4 text-destructive"
    >
      {{ error }}
    </div>

    <template v-else-if="systemInfo">
      <div class="grid gap-4 md:grid-cols-1 lg:grid-cols-1">
        <div class="w-full max-w-full rounded-md border p-2 h-22">
          <div class="flex justify-between mb-4">
            <div class="flex flex-col gap-y-1">
              <div class="text-sm font-medium">{{ systemInfo.cpu_brand }}</div>
              <div class="text-xs text-gray-500 flex gap-x-1 items-center">
                <span>{{ systemInfo.cpu_architecture }}</span>
                <span><Dot class="w-2 h-2" /></span>
                <span>{{ systemInfo.total_core }} cores</span>
              </div>
            </div>

            <!-- CPU usage -->
            <div>
              <span class="text-2xl font-semibold mr-1">{{
                systemInfo.total_cpu_usage.toFixed(2)
              }}</span>
              <span class="text-xs text-gray-500">%</span>
            </div>
            <!-- CPU usage -->
          </div>

          <Progress :model-value="systemInfo.total_cpu_usage" />
        </div>
      </div>
    </template>
  </div>
</template>
