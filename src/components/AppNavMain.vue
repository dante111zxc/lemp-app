<template>
  <SidebarGroup>
    <SidebarGroupContent class="flex flex-col gap-2">
      <SidebarMenu>
        <SidebarMenuItem v-for="item in items" :key="item.title">
          <SidebarMenuButton :tooltip="item.title">
            <component :is="item.icon" v-if="item.icon" />
            <span>{{ item.title }}</span>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarGroupContent>
  </SidebarGroup>
</template>

<script setup lang="ts">
import { ref, type Component } from 'vue'
import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar'

import { LucideLayoutDashboard, Database, Server, DatabaseZap } from 'lucide-vue-next'

interface NavItem {
  title: string
  url: string
  icon?: Component
}

const items = ref<Array<NavItem>>([
  {
    title: 'Web Server',
    url: '#',
    icon: Server,
  },
  {
    title: 'PHP Versions',
    url: '#',
    icon: LucideLayoutDashboard,
  },
  {
    title: 'Databases',
    url: '#',
    icon: Database,
  },
  {
    title: 'Cache',
    url: '#',
    icon: DatabaseZap,
  },
])
</script>
