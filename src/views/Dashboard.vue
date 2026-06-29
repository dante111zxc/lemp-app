<script setup lang="ts">
import { useSystemInfo } from '@/composables/useSystemInfo'
import { Dot, Zap, AudioLinesIcon, Cpu, ChartPieIcon } from 'lucide-vue-next'
import { Progress } from '@/components/ui/progress'
import { formatCpuFrequency } from '@/utils/format'

const { systemInfo, loading, error, cpuMhz } = useSystemInfo()
</script>

<template>
  <div class="space-y-6">
    <div>
      <h1 class="text-2xl font-bold">Dashboard</h1>
      <p class="text-muted-foreground">System overview and resource usage</p>
    </div>

    <div v-if="loading" class="grid gap-4 md:grid-cols-1 lg:grid-cols-1">
      <div class="w-full max-w-full rounded-md border p-2 h-40">
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
        <div class="w-full max-w-full rounded-md border p-2 min-h-22">
          <div class="flex justify-between mb-4">
            <div class="flex flex-col gap-y-1">
              <div class="flex items-center gap-x-2">
                <div class="border border-indigo-500 rounded-lg p-3 bg-indigo-200">
                  <Cpu :size="20" class="text-indigo-500"></Cpu>
                </div>

                <div>
                  <div class="text-sm font-medium">{{ systemInfo.cpu_brand }}</div>
                  <div class="text-xs text-gray-500 flex gap-x-1 items-center">
                    <span>{{ systemInfo.cpu_architecture }}</span>
                    <span><Dot class="w-2 h-2" /></span>
                    <span>{{ systemInfo.total_core }} cores</span>
                  </div>
                </div>
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

          <Progress :model-value="systemInfo.total_cpu_usage" class="mb-4" />

          <div class="grid grid-cols-11 md:grid-cols-3">
            <div>
              <div class="flex gap-x-2 items-center">
                <Zap class="text-gray-500" :size="12"></Zap>
                <span class="text-xs text-gray-500">Frequency</span>
              </div>

              <div>
                <span class="text-lg font-medium mr-1">
                  {{ formatCpuFrequency(cpuMhz) }}
                </span>

                <span class="text-xs text-gray-500" v-if="cpuMhz > 1000">Ghz</span>
                <span class="text-xs text-gray-500" v-else>Mhz</span>
              </div>
            </div>

            <div>
              <div class="flex gap-x-2 items-center">
                <AudioLinesIcon class="text-gray-500" :size="12"></AudioLinesIcon>
                <span class="text-xs text-gray-500">Usage</span>
              </div>

              <div>
                <span class="text-lg font-medium mr-1">{{
                  systemInfo.total_cpu_usage.toFixed(2)
                }}</span>
                <span class="text-xs text-gray-500">%</span>
              </div>
            </div>

            <div>
              <div class="flex gap-x-2 items-center">
                <ChartPieIcon class="text-gray-500" :size="12" />
                <span class="text-xs text-gray-500">Memory usage</span>
              </div>

              <div>
                <span class="text-lg font-medium mr-1"
                  >{{ systemInfo.used_memory_gb.toFixed(1) }}/{{
                    Math.ceil(systemInfo.total_memory_gb)
                  }}</span
                >
                <span class="text-xs text-gray-500">GB</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
