<template>
  <SidebarProvider
    :style="{
      '--sidebar-width': 'calc(var(--spacing) * 72)',
      '--header-height': 'calc(var(--spacing) * 12)',
    }"
  >
    <AppSidebar variant="inset" />
    <SidebarInset>
      <AppHeader />
      <div class="flex flex-1 flex-col">
        <div class="@container/main flex flex-1 flex-col gap-2">
          <div class="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
            <!-- <SectionCards /> -->
            <div class="px-4 lg:px-6">
              <!-- <ChartAreaInteractive /> -->
            </div>
            <!-- <DataTable :data="data" /> -->
          </div>
        </div>
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
<script setup lang="ts">
import AppHeader from './layouts/AppHeader.vue'
import AppFooter from './layouts/AppFooter.vue'
import AppSidebar from './layouts/AppSidebar.vue'
import { SidebarInset, SidebarProvider } from '@/components/ui/sidebar'
</script>
