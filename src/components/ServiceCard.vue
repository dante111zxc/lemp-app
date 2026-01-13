<script setup lang="ts">
import { ref } from 'vue'
import type { Service } from '@/types/service'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Play, Square, RotateCcw, Settings } from 'lucide-vue-next'
import type { BadgeVariants } from '@/components/ui/badge'
import { EnumServiceStatus } from '@/enums/EnumServiceStatus'
import ServiceConfigureModal from './ServiceConfigureModal.vue'

interface Props {
  service: Service
}

const props = defineProps<Props>()
const emit = defineEmits<{
  start: [serviceId: string]
  stop: [serviceId: string]
  restart: [serviceId: string]
  configure: [serviceId: string]
}>()

const configureModalOpen = ref(false)

const getStatusVariant = (status: Service['status']): BadgeVariants['variant'] => {
  switch (status) {
    case EnumServiceStatus.ACTIVE:
      return 'default'
    case EnumServiceStatus.STOPPED:
      return 'secondary'
    case EnumServiceStatus.ERROR:
      return 'destructive'
    default:
      return 'outline'
  }
}

const getClassStatus = (status: Service['status']) => {
  switch (status) {
    case EnumServiceStatus.ACTIVE:
      return 'bg-green-500'
    case EnumServiceStatus.STOPPED:
      return 'bg-yellow-500 text-white'
    case EnumServiceStatus.ERROR:
      return 'bg-red-500'
    case EnumServiceStatus.STARTING:
    case EnumServiceStatus.STOPPING:
      return 'bg-green-300'
    default:
      return ''
  }
}

const getStatusText = (status: Service['status']) => {
  return EnumServiceStatus.getLabel(status)
}

const isActionDisabled = (action: 'start' | 'stop' | 'restart') => {
  const { status } = props.service

  // Disable all actions if service is not installed
  if (status === EnumServiceStatus.NOT_INSTALLED) return true

  if (status === EnumServiceStatus.STARTING || status === EnumServiceStatus.STOPPING) return true

  switch (action) {
    case 'start':
      return status === EnumServiceStatus.ACTIVE
    case 'stop':
      return status === EnumServiceStatus.STOPPED || status === EnumServiceStatus.ERROR || status === EnumServiceStatus.NOT_ACTIVE
    case 'restart':
      return status === EnumServiceStatus.STOPPED || status === EnumServiceStatus.ERROR || status === EnumServiceStatus.NOT_ACTIVE
    default:
      return false
  }
}

const handleConfigure = () => {
  configureModalOpen.value = true
}
</script>

<template>
  <Card class="transition-all duration-200 hover:shadow-md cursor-pointer">
    <CardHeader>
      <div class="flex items-start justify-between gap-2">
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2 mb-1 flex-wrap">
            <CardTitle class="text-lg">{{ service.displayName }}</CardTitle>
            <Badge
              :variant="getStatusVariant(service.status)"
              :class="getClassStatus(service.status)"
            >
              {{ getStatusText(service.status) }}
            </Badge>
          </div>
          <CardDescription>{{ service.description }}</CardDescription>
        </div>
        <Button
          variant="ghost"
          size="icon-sm"
          @click.stop="handleConfigure"
          aria-label="Configure service"
        >
          <Settings />
        </Button>
      </div>
    </CardHeader>
    <CardContent>
      <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
        <div class="space-y-1 w-full sm:w-auto">
          <div class="flex flex-wrap gap-x-4 gap-y-1 text-sm text-muted-foreground">
            <div v-if="service.port" class="flex items-center gap-1.5">
              <span>Port:</span>
              <code class="font-mono text-foreground">{{ service.port }}</code>
            </div>
            <div v-if="service.version" class="flex items-center gap-1.5">
              <span>Version:</span>
              <code class="font-mono text-xs text-foreground">{{ service.version }}</code>
            </div>
          </div>
        </div>

        <div class="flex flex-wrap items-center gap-2 w-full sm:w-auto">
          <Button
            size="sm"
            variant="outline"
            :disabled="isActionDisabled('start')"
            @click.stop="emit('start', service.id)"
            aria-label="Start service"
            class="flex-1 sm:flex-none"
          >
            <Play />
            <span class="hidden sm:inline">Start</span>
          </Button>
          <Button
            size="sm"
            variant="outline"
            :disabled="isActionDisabled('stop')"
            @click.stop="emit('stop', service.id)"
            aria-label="Stop service"
            class="flex-1 sm:flex-none"
          >
            <Square />
            <span class="hidden sm:inline">Stop</span>
          </Button>
          <Button
            size="sm"
            variant="outline"
            :disabled="isActionDisabled('restart')"
            @click.stop="emit('restart', service.id)"
            aria-label="Restart service"
            class="flex-1 sm:flex-none"
          >
            <RotateCcw />
            <span class="hidden sm:inline">Restart</span>
          </Button>
        </div>
      </div>
    </CardContent>
  </Card>

  <ServiceConfigureModal
    :service="service"
    :open="configureModalOpen"
    @update:open="configureModalOpen = $event"
    @install="(serviceId) => emit('configure', serviceId)"
    @start="(serviceId) => emit('start', serviceId)"
    @stop="(serviceId) => emit('stop', serviceId)"
    @restart="(serviceId) => emit('restart', serviceId)"
  />
</template>
