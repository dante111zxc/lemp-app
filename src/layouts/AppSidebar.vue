<script setup lang="ts">
import { useRoute, RouterLink } from 'vue-router'
import { Monitor, Server, Database, HardDrive, Mail, Globe, Sun, Moon, Cog } from 'lucide-vue-next'
import {
  SidebarHeader,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarRail,
} from '@/components/ui/sidebar'
import { Button } from '@/components/ui/button'
import { useTheme } from '@/composables/useTheme'
import { computed } from 'vue'
const route = useRoute()
const { isDark, toggleTheme } = useTheme()

const navItems = [{ title: 'Dashboard', icon: Monitor, to: '/' }]

const serviceItems = [
  { title: 'Nginx', icon: Globe, to: '/nginx' },
  { title: 'MySQL', icon: Database, to: '/mysql' },
  { title: 'Redis', icon: HardDrive, to: '/redis' },
  { title: 'Mailpit', icon: Mail, to: '/mailpit' },
  { title: 'PHP', icon: Cog, to: '/php' },
]

const isActive = (path: string) => {
  console.log('path', path)

  return route.path === path
}
</script>

<template>
  <SidebarHeader class="border-b px-4 py-3">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <Server class="h-5 w-5" />
        <span class="font-heading text-base font-semibold">LEMP Manager</span>
      </div>

      <Button variant="ghost" size="icon" @click="toggleTheme" class="h-8 w-8">
        <Sun v-if="isDark" class="h-4 w-4" />
        <Moon v-else class="h-4 w-4" />
      </Button>
    </div>
  </SidebarHeader>

  <SidebarContent>
    <SidebarGroup>
      <SidebarGroupLabel>System</SidebarGroupLabel>
      <SidebarGroupContent>
        <SidebarMenu>
          <SidebarMenuItem v-for="item in navItems" :key="item.title">
            <SidebarMenuButton asChild :isActive="isActive(item.to)">
              <RouterLink :to="item.to">
                <component :is="item.icon" />
                <span>{{ item.title }}</span>
              </RouterLink>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>

    <SidebarGroup>
      <SidebarGroupLabel>Services</SidebarGroupLabel>
      <SidebarGroupContent>
        <SidebarMenu>
          <SidebarMenuItem v-for="item in serviceItems" :key="item.title">
            <SidebarMenuButton asChild :isActive="isActive(item.to)" variant="default">
              <RouterLink :to="item.to" :activeClass="'!bg-black !text-white'">
                <component :is="item.icon" />
                <span>{{ item.title }}</span>
              </RouterLink>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  </SidebarContent>

  <SidebarFooter class="border-t p-2">
    <div class="px-2 py-1">
      <p class="text-xs text-muted-foreground">LEMP Stack Manager</p>
    </div>
  </SidebarFooter>

  <SidebarRail />
</template>
