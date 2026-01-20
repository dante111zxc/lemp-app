<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { Service } from '@/types/service'
import { Button } from '@/components/ui/button'
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
} from '@/components/ui/drawer'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import {
  Download,
  FileText,
  XCircle,
  CheckCircle2,
  Play,
  Square,
  RotateCcw,
  ScrollText,
} from 'lucide-vue-next'
import { EnumServiceStatus } from '@/enums/EnumServiceStatus'
import { ScrollArea } from '@/components/ui/scroll-area'

interface Props {
  service: Service | null
  open: boolean
}

const props = withDefaults(defineProps<Props>(), {
  service: null,
  open: false,
})

const emit = defineEmits<{
  'update:open': [open: boolean]
  install: [serviceId: string, version: string]
  'view-logs': [serviceId: string]
  start: [serviceId: string]
  stop: [serviceId: string]
  restart: [serviceId: string]
}>()

const selectedVersion = ref<string>('')
const showLogs = ref(false)
const mockLogs = ref<string>(
  `[2026-01-13 10:45:23] Service started successfully
[2026-01-13 10:45:24] Listening on port 9000
[2026-01-13 10:45:25] Worker process 1 spawned
[2026-01-13 10:45:25] Worker process 2 spawned
[2026-01-13 10:45:26] Ready to accept connections
[2026-01-13 10:46:01] Request from 127.0.0.1
[2026-01-13 10:46:02] Request from 127.0.0.1`
)

// Available versions for each service that supports version selection
const versionMap: Record<string, string[]> = {
  php: ['7.4', '8.1', '8.2', '8.3', '8.4'],
  mysql: ['5', '8'],
  redis: ['6.2', '7.4', '8.0', '8.2', '8.4'],
}

// Check if service is installed
const isInstalled = computed(() => {
  if (!props.service) return false
  return props.service.status !== EnumServiceStatus.NOT_INSTALLED
})

// Check if service supports version selection
const hasVersionSelection = computed(() => {
  if (!props.service) return false
  return ['php', 'mysql', 'redis'].includes(props.service.id)
})

// Get available versions for current service
const availableVersions = computed(() => {
  if (!props.service) return []
  return versionMap[props.service.id] || []
})

// Check if service is running
const isRunning = computed(() => {
  if (!props.service) return false
  return props.service.status === EnumServiceStatus.ACTIVE
})

// Check if service is stopped
const isStopped = computed(() => {
  if (!props.service) return false
  return (
    props.service.status === EnumServiceStatus.STOPPED ||
    props.service.status === EnumServiceStatus.NOT_ACTIVE ||
    props.service.status === EnumServiceStatus.ERROR
  )
})

// Check if service is restarting
const isRestarting = computed(() => {
  if (!props.service) return false
  return (
    props.service.status === EnumServiceStatus.STARTING ||
    props.service.status === EnumServiceStatus.STOPPING
  )
})

// Button disabled states
const isStartDisabled = computed(() => !isInstalled.value || isRunning.value || isRestarting.value)
const isStopDisabled = computed(() => !isInstalled.value || isStopped.value || isRestarting.value)
const isRestartDisabled = computed(() => !isInstalled.value || isRestarting.value)

// Reset state when modal opens
watch(
  () => props.open,
  newVal => {
    if (newVal && props.service) {
      selectedVersion.value = props.service.version || availableVersions.value[0] || ''
      showLogs.value = false
    }
  }
)

const handleInstall = () => {
  if (props.service && selectedVersion.value) {
    emit('install', props.service.id, selectedVersion.value)
  }
}

const handleStart = () => {
  if (props.service) {
    emit('start', props.service.id)
  }
}

const handleStop = () => {
  if (props.service) {
    emit('stop', props.service.id)
  }
}

const handleRestart = () => {
  if (props.service) {
    emit('restart', props.service.id)
  }
}

const handleViewLogs = () => {
  if (props.service) {
    emit('view-logs', props.service.id)
    showLogs.value = true
  }
}
</script>

<template>
  <Drawer :open="open" direction="left" @update:open="val => emit('update:open', val)">
    <DrawerContent>
      <DrawerHeader>
        <DrawerTitle>{{ service?.displayName }}</DrawerTitle>
        <DrawerDescription>Quản lý cài đặt và cấu hình service</DrawerDescription>
      </DrawerHeader>

      <ScrollArea class="flex-1 px-4 py-4 max-h-[70vh]">
        <div class="space-y-4">
          <!-- Section 1: Service Not Installed -->
          <Card v-if="!isInstalled && service">
            <CardHeader class="pb-3">
              <div class="flex items-center gap-2">
                <XCircle class="h-5 w-5 text-red-500" />
                <CardTitle class="text-base text-red-600">Service chưa được cài đặt</CardTitle>
              </div>
              <CardDescription>
                {{ service.displayName }} chưa được cài đặt trên hệ thống
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
              <!-- Version selection for PHP, MySQL, Redis -->
              <div v-if="hasVersionSelection" class="space-y-3">
                <label class="text-sm font-medium">Chọn phiên bản để cài đặt:</label>
                <Select v-model="selectedVersion">
                  <SelectTrigger class="w-full">
                    <SelectValue :placeholder="`Chọn phiên bản ${service.displayName}`" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem
                      v-for="version in availableVersions"
                      :key="version"
                      :value="version"
                    >
                      {{ service.displayName }} {{ version }}
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <Button
                class="w-full"
                :disabled="hasVersionSelection && !selectedVersion"
                @click="handleInstall"
              >
                <Download class="mr-2 h-4 w-4" />
                Cài đặt {{ service.displayName }}
                <span v-if="selectedVersion"> {{ selectedVersion }}</span>
              </Button>
            </CardContent>
          </Card>

          <!-- Section 2: Service Information (only when installed) -->
          <Card v-if="isInstalled && service" class="gap-y-2">
            <CardHeader>
              <div class="flex items-center gap-2">
                <CheckCircle2 class="h-5 w-5 text-green-500" />
                <CardTitle class="text-base">Thông tin Service</CardTitle>
              </div>
            </CardHeader>
            <CardContent>
              <div class="grid grid-cols-2 gap-4 text-sm">
                <div class="space-y-1">
                  <span class="text-muted-foreground">Tên service</span>
                  <p class="font-medium">{{ service.displayName }}</p>
                </div>
                <div class="space-y-1 space-x-1">
                  <span class="text-muted-foreground">PID</span>
                  <span class="text-sm font-semibold">{{ isRunning ? '12345' : '-' }}</span>
                </div>
                <div class="space-y-1 space-x-1">
                  <span class="text-muted-foreground">Cổng</span>
                  <span class="text-sm font-semibold">{{ service.port || '-' }}</span>
                </div>
                <div class="space-y-1 space-x-1">
                  <span class="text-muted-foreground">Trạng thái</span>
                  <span class="text-sm font-semibold">{{
                    EnumServiceStatus.getLabel(service.status)
                  }}</span>
                </div>
                <div class="space-y-1 space-x-1">
                  <span class="text-muted-foreground">Phiên bản</span>
                  <span class="text-sm font-semibold">{{ service.version || '-' }}</span>
                </div>
                <div class="space-y-1 space-x-1">
                  <span class="text-muted-foreground">Tự khởi động</span>
                  <span class="text-sm font-semibold">{{ service.autoStart ? 'Bật' : 'Tắt' }}</span>
                </div>
              </div>
            </CardContent>
          </Card>

          <!-- Section 3: Service Controls (only when installed) -->
          <Card v-if="isInstalled && service">
            <CardHeader class="pb-3">
              <CardTitle class="text-base">Điều khiển Service</CardTitle>
              <CardDescription>Khởi động, dừng hoặc khởi động lại service</CardDescription>
            </CardHeader>
            <CardContent>
              <div class="grid grid-cols-2 gap-3">
                <Button
                  variant="outline"
                  :disabled="isStartDisabled"
                  @click="handleStart"
                  class="w-full"
                >
                  <Play class="mr-2 h-4 w-4" />
                  Start
                </Button>
                <Button
                  variant="outline"
                  :disabled="isStopDisabled"
                  @click="handleStop"
                  class="w-full"
                >
                  <Square class="mr-2 h-4 w-4" />
                  Stop
                </Button>
                <Button
                  variant="outline"
                  :disabled="isRestartDisabled"
                  @click="handleRestart"
                  class="w-full"
                >
                  <RotateCcw class="mr-2 h-4 w-4" />
                  Restart
                </Button>
                <Button variant="outline" @click="handleViewLogs" class="w-full">
                  <ScrollText class="mr-2 h-4 w-4" />
                  Xem Log
                </Button>
              </div>
            </CardContent>
          </Card>
        </div>
      </ScrollArea>

      <DrawerFooter>
        <DrawerClose as-child>
          <Button variant="outline" class="w-full">Đóng</Button>
        </DrawerClose>
      </DrawerFooter>

      <!-- Logs Drawer -->
      <Drawer :open="showLogs" @update:open="val => (showLogs = val)">
        <DrawerContent class="max-h-[85vh]">
          <DrawerHeader>
            <DrawerTitle>{{ service?.displayName }} Logs</DrawerTitle>
            <DrawerDescription>Xem log của service</DrawerDescription>
          </DrawerHeader>

          <div class="px-4 flex-1 overflow-hidden">
            <div class="bg-black/5 dark:bg-black/50 rounded-md p-4 h-[40vh] overflow-hidden">
              <ScrollArea class="h-full">
                <pre class="text-sm font-mono text-foreground whitespace-pre-wrap">{{
                  mockLogs
                }}</pre>
              </ScrollArea>
            </div>
          </div>

          <DrawerFooter>
            <DrawerClose as-child>
              <Button variant="outline" class="w-full">Đóng</Button>
            </DrawerClose>
          </DrawerFooter>
        </DrawerContent>
      </Drawer>
    </DrawerContent>
  </Drawer>
</template>
