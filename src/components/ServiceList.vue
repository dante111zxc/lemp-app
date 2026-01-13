<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Service } from '@/types/service'
import ServiceCard from '@/components/ServiceCard.vue'
import { Card, CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { Search, RefreshCw, Activity, CheckCircle2, XCircle } from 'lucide-vue-next'
import { EnumServiceStatus } from '@/enums/EnumServiceStatus'

const emit = defineEmits<{
  start: [serviceId: string]
  stop: [serviceId: string]
  restart: [serviceId: string]
  configure: [serviceId: string]
  refreshAll: []
}>()

// Mock data - will be replaced with real Tauri commands
const services = ref<Service[]>([
  {
    id: 'nginx',
    name: 'nginx',
    displayName: 'Nginx',
    description: 'Web server and reverse proxy',
    status: EnumServiceStatus.ERROR,
    port: 80,
    autoStart: true,
    version: '1.24.0',
  },
  {
    id: 'php',
    name: 'php',
    displayName: 'PHP',
    description: 'PHP-FPM & CLI for web and command line',
    status: EnumServiceStatus.ACTIVE,
    port: 9000,
    autoStart: true,
    version: '8.3',
  },
  {
    id: 'mysql',
    name: 'mysql',
    displayName: 'MySQL',
    description: 'Relational database management system',
    status: EnumServiceStatus.ACTIVE,
    port: 3306,
    autoStart: true,
    version: '8.0',
  },
  {
    id: 'redis',
    name: 'redis',
    displayName: 'Redis',
    description: 'In-memory data structure store',
    status: EnumServiceStatus.STOPPED,
    port: 6379,
    autoStart: false,
    version: '7.2.0',
  },
  {
    id: 'mailpit',
    name: 'mailpit',
    displayName: 'Mailpit',
    description: 'Email testing tool',
    status: EnumServiceStatus.ACTIVE,
    port: 8025,
    autoStart: true,
    version: '1.10.0',
  },
  {
    id: 'supervisord',
    name: 'supervisord',
    displayName: 'Supervisor',
    description: 'Process control system',
    status: EnumServiceStatus.NOT_ACTIVE,
    port: 9001,
    autoStart: true,
    version: '4.2.5',
  },
  {
    id: 'memcached',
    name: 'memcached',
    displayName: 'Memcached',
    description: 'Distributed memory caching system',
    status: EnumServiceStatus.NOT_INSTALLED,
    port: 11211,
    autoStart: false,
  },
])

const searchQuery = ref('')
const isRefreshing = ref(false)

const filteredServices = computed(() => {
  if (!searchQuery.value) return services.value

  const query = searchQuery.value.toLowerCase()
  return services.value.filter(
    service =>
      service.displayName.toLowerCase().includes(query) ||
      service.description.toLowerCase().includes(query) ||
      service.name.toLowerCase().includes(query)
  )
})

const runningCount = computed(
  () => services.value.filter(s => s.status === EnumServiceStatus.ACTIVE).length
)
const stoppedCount = computed(
  () => services.value.filter(s => s.status === EnumServiceStatus.STOPPED).length
)

const handleRefresh = async () => {
  isRefreshing.value = true
  emit('refreshAll')

  setTimeout(() => {
    isRefreshing.value = false
  }, 1000)
}
</script>

<template>
  <div class="space-y-6">
    <!-- Stats Overview -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <Card>
        <CardContent class="pt-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">Total Services</p>
              <h3 class="text-2xl font-bold mt-2">{{ services.length }}</h3>
            </div>
            <Activity class="h-8 w-8 text-muted-foreground" />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="pt-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">Running</p>
              <h3 class="text-2xl font-bold text-success mt-2">{{ runningCount }}</h3>
            </div>
            <CheckCircle2 class="h-8 w-8 text-success" />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardContent class="pt-6">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm font-medium text-muted-foreground">Stopped</p>
              <h3 class="text-2xl font-bold text-muted-foreground mt-2">{{ stoppedCount }}</h3>
            </div>
            <XCircle class="h-8 w-8 text-muted-foreground" />
          </div>
        </CardContent>
      </Card>
    </div>

    <Separator />

    <!-- Search and Actions -->
    <div class="flex items-center gap-2">
      <div class="relative flex-1">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
        <Input v-model="searchQuery" placeholder="Search services..." class="pl-9" />
      </div>
      <Button
        variant="outline"
        size="icon"
        @click="handleRefresh"
        :disabled="isRefreshing"
        aria-label="Refresh all services"
      >
        <RefreshCw :class="isRefreshing && 'animate-spin'" />
      </Button>
    </div>

    <!-- Services Grid -->
    <div class="grid gap-4">
      <ServiceCard
        v-for="service in filteredServices"
        :key="service.id"
        :service="service"
        @start="emit('start', $event)"
        @stop="emit('stop', $event)"
        @restart="emit('restart', $event)"
        @configure="emit('configure', $event)"
      />
    </div>

    <!-- Empty State -->
    <Card v-if="filteredServices.length === 0">
      <CardContent class="flex flex-col items-center justify-center py-12">
        <Search class="h-12 w-12 text-muted-foreground mb-4" />
        <h3 class="text-lg font-semibold mb-2">No services found</h3>
        <p class="text-sm text-muted-foreground">Try adjusting your search criteria</p>
      </CardContent>
    </Card>
  </div>
</template>
