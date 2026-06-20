<script setup lang="ts">
import { useSystemInfo } from '@/composables/useSystemInfo'
import { formatBytes } from '@/types/system'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import {
  Activity,
  Cpu,
  HardDrive,
  MemoryStick,
  Monitor,
} from 'lucide-vue-next'

const { systemInfo, loading, error } = useSystemInfo(5000)
</script>

<template>
  <div class="space-y-6">
    <div>
      <h1 class="text-2xl font-bold">Dashboard</h1>
      <p class="text-muted-foreground">System overview and resource usage</p>
    </div>

    <div v-if="loading" class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card v-for="i in 4" :key="i">
        <CardHeader class="pb-2">
          <CardTitle class="text-sm font-medium">&nbsp;</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="h-8 w-24 animate-pulse rounded bg-muted" />
        </CardContent>
      </Card>
    </div>

    <div v-else-if="error" class="rounded-lg border border-destructive/50 bg-destructive/10 p-4 text-destructive">
      {{ error }}
    </div>

    <template v-else-if="systemInfo">
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader class="flex flex-row items-center justify-between pb-2">
            <CardTitle class="text-sm font-medium">Operating System</CardTitle>
            <Monitor class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div class="text-lg font-bold">{{ systemInfo.os_name }}</div>
            <p class="text-xs text-muted-foreground">{{ systemInfo.os_version }}</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader class="flex flex-row items-center justify-between pb-2">
            <CardTitle class="text-sm font-medium">CPU</CardTitle>
            <Cpu class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div class="text-lg font-bold">{{ systemInfo.cpu_cores }} cores</div>
            <p class="text-xs text-muted-foreground truncate">{{ systemInfo.cpu_name }}</p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader class="flex flex-row items-center justify-between pb-2">
            <CardTitle class="text-sm font-medium">Memory</CardTitle>
            <MemoryStick class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div class="text-lg font-bold">
              {{ formatBytes(systemInfo.used_memory) }} / {{ formatBytes(systemInfo.total_memory) }}
            </div>
            <div class="mt-2 h-2 w-full rounded-full bg-muted">
              <div
                class="h-2 rounded-full bg-primary transition-all"
                :style="{ width: ((systemInfo.used_memory / systemInfo.total_memory) * 100) + '%' }"
              />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader class="flex flex-row items-center justify-between pb-2">
            <CardTitle class="text-sm font-medium">Disk</CardTitle>
            <HardDrive class="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div class="text-lg font-bold">
              {{ formatBytes(systemInfo.used_disk) }} / {{ formatBytes(systemInfo.total_disk) }}
            </div>
            <div class="mt-2 h-2 w-full rounded-full bg-muted">
              <div
                class="h-2 rounded-full bg-primary transition-all"
                :style="{ width: ((systemInfo.used_disk / systemInfo.total_disk) * 100) + '%' }"
              />
            </div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle class="flex items-center gap-2 text-sm font-medium">
            <Activity class="h-4 w-4" />
            Resource Usage
          </CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <div>
            <div class="mb-1 flex justify-between text-sm">
              <span class="text-muted-foreground">CPU Usage</span>
              <span>{{ systemInfo.cpu_usage }}%</span>
            </div>
            <div class="h-2.5 w-full rounded-full bg-muted">
              <div
                class="h-2.5 rounded-full transition-all"
                :class="systemInfo.cpu_usage > 80 ? 'bg-destructive' : 'bg-primary'"
                :style="{ width: systemInfo.cpu_usage + '%' }"
              />
            </div>
          </div>
          <div>
            <div class="mb-1 flex justify-between text-sm">
              <span class="text-muted-foreground">Memory Usage</span>
              <span>{{ ((systemInfo.used_memory / systemInfo.total_memory) * 100).toFixed(1) }}%</span>
            </div>
            <div class="h-2.5 w-full rounded-full bg-muted">
              <div
                class="h-2.5 rounded-full transition-all"
                :class="(systemInfo.used_memory / systemInfo.total_memory) > 0.8 ? 'bg-destructive' : 'bg-primary'"
                :style="{ width: ((systemInfo.used_memory / systemInfo.total_memory) * 100) + '%' }"
              />
            </div>
          </div>
          <div>
            <div class="mb-1 flex justify-between text-sm">
              <span class="text-muted-foreground">Disk Usage</span>
              <span>{{ ((systemInfo.used_disk / systemInfo.total_disk) * 100).toFixed(1) }}%</span>
            </div>
            <div class="h-2.5 w-full rounded-full bg-muted">
              <div
                class="h-2.5 rounded-full transition-all"
                :class="(systemInfo.used_disk / systemInfo.total_disk) > 0.85 ? 'bg-destructive' : 'bg-primary'"
                :style="{ width: ((systemInfo.used_disk / systemInfo.total_disk) * 100) + '%' }"
              />
            </div>
          </div>
        </CardContent>
      </Card>
    </template>
  </div>
</template>
