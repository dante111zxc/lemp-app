<script setup lang="ts">
import AppHeader from '@/components/AppHeader.vue'
import ServiceList from '@/components/ServiceList.vue'
import { useSystemInfo } from '@/composables/useSystemInfo'
import { formatBytes } from '@/types/system'

const { systemInfo, loading, error } = useSystemInfo(5000)

const handleStartService = (serviceId: string) => {
  console.log('Starting service:', serviceId)
  // TODO: Implement Tauri command to start service
  // await invoke('start_service', { serviceId })
}

const handleStopService = (serviceId: string) => {
  console.log('Stopping service:', serviceId)
  // TODO: Implement Tauri command to stop service
  // await invoke('stop_service', { serviceId })
}

const handleRestartService = (serviceId: string) => {
  console.log('Restarting service:', serviceId)
  // TODO: Implement Tauri command to restart service
  // await invoke('restart_service', { serviceId })
}

const handleConfigureService = (serviceId: string) => {
  console.log('Configuring service:', serviceId)
  // TODO: Implement service configuration dialog
}

const handleRefreshAll = () => {
  console.log('Refreshing all services')
  // TODO: Implement Tauri command to refresh service status
  // await invoke('get_all_services')
}
</script>

<template>
  <div class="flex flex-col h-screen">
    <AppHeader />

    <main class="flex-1 overflow-y-auto">
      <div class="container mx-auto p-6">
        <div class="mb-5">
          <h2 class="text-2xl font-bold mb-2">System information</h2>

          <div v-if="loading">Loading...</div>
          <div v-else-if="systemInfo">
            <p>OS: {{ systemInfo.os_name }} {{ systemInfo.os_version }}</p>
            <p>CPU: {{ systemInfo.cpu_name }} ({{ systemInfo.cpu_cores }} cores)</p>
            <p>
              RAM: {{ formatBytes(systemInfo.used_memory) }} /
              {{ formatBytes(systemInfo.total_memory) }}
            </p>
            <p>
              Disk: {{ formatBytes(systemInfo.free_disk) }} /
              {{ formatBytes(systemInfo.total_disk) }}
            </p>
          </div>
        </div>

        <div class="mb-6">
          <h2 class="text-2xl font-bold">Services</h2>
          <p class="text-muted-foreground">Manage your local development environment services</p>
        </div>

        <ServiceList
          @start="handleStartService"
          @stop="handleStopService"
          @restart="handleRestartService"
          @configure="handleConfigureService"
          @refresh-all="handleRefreshAll"
        />
      </div>
    </main>
  </div>
</template>
