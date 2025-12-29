<template>
  <Sidebar collapsible="offcanvas">
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton as-child class="data-[slot=sidebar-menu-button]:p-1.5!">
            <a href="#">
              <SettingsIcon :size="32" />
              <span class="text-base uppercase font-semibold">Settings</span>
            </a>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>
    <SidebarContent>
      <AppNavMain />
    </SidebarContent>
    <SidebarFooter>
      <SidebarGroup>
        <SidebarGroupContent class="flex flex-col gap-2">
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton>
                <Button class="w-full" variant="destructive">
                  <CirclePause :size="40" />
                  <span>Stop All</span></Button
                >
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarFooter>
  </Sidebar>
</template>

<script lang="ts" setup>
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar'
import { Button } from '@/components/ui/button'
import { SettingsIcon, CirclePause } from 'lucide-vue-next'
import AppNavMain from '@/components/AppNavMain.vue'
</script>
