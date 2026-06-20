<script setup lang="ts">
import { useTheme } from '@/composables/useTheme'
import {
  SidebarProvider,
  Sidebar,
  SidebarInset,
} from '@/components/ui/sidebar'
import AppSidebar from '@/layouts/AppSidebar.vue'

useTheme()
</script>

<template>
  <SidebarProvider>
    <Sidebar collapsible="icon">
      <AppSidebar />
    </Sidebar>
    <SidebarInset>
      <div class="flex flex-col h-full">
        <main class="flex-1 overflow-y-auto">
          <div class="container mx-auto p-6">
            <router-view />
          </div>
        </main>
      </div>
    </SidebarInset>
  </SidebarProvider>
</template>
