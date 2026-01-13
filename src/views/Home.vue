<script setup lang="ts">
import AppHeader from '@/components/AppHeader.vue'
import ServiceList from '@/components/ServiceList.vue'

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
        <div class="mb-6">
          <h2 class="text-2xl font-bold mb-2">Services</h2>
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
