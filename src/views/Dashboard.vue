<script setup lang="ts">
import { useSystemInfo } from '@/composables/useSystemInfo'
import { formatBytes } from '@/utils/format'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Activity, Cpu, HardDrive, MemoryStick, Monitor } from 'lucide-vue-next'

const { systemInfo, loading, error } = useSystemInfo(5000)
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
      <prE>{{ systemInfo }}</prE>
      <div class="grid gap-4 md:grid-cols-1 lg:grid-cols-1">
        <div class="w-full max-w-full rounded-md border p-2 h-22">
          <div class="flex">
            <div class="flex">
              <div class="text-sm font-medium">{{ systemInfo.cpus[0].brand }}</div>
              <div class="text-xs text-gray-500"></div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
