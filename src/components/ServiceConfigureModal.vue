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
import { Download, FileText, AlertCircle, CheckCircle2, Terminal, Globe } from 'lucide-vue-next'
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
  install: [serviceId: string]
  'select-version': [serviceId: string, version: string]
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

// Available versions for each service
const versionMap: Record<string, string[]> = {
  php: ['7.4', '8.1', '8.2', '8.3', '8.4'],
  mysql: ['5.7', '8.0'],
  nginx: ['1.24.0'],
  redis: ['7.2.0'],
  mailpit: ['1.10.0'],
  supervisord: ['4.2.5'],
}

const isInstalled = computed(() => {
  if (!props.service) return false
  return props.service.status !== EnumServiceStatus.NOT_INSTALLED
})

const availableVersions = computed(() => {
  if (!props.service) return []
  return versionMap[props.service.id] || []
})

const isVersionSelectable = computed(() => {
  if (!props.service) return false
  return ['php', 'mysql'].includes(props.service.id)
})

// Check if this is PHP service (to show FPM + CLI info)
const isPHPService = computed(() => {
  return props.service?.id === 'php'
})

const canStartStopRestart = computed(() => {
  return isInstalled.value
})

const canStop = computed(() => {
  if (!props.service) return false
  return props.service.status !== EnumServiceStatus.STOPPED && isInstalled.value
})

watch(
  () => props.open,
  newVal => {
    if (newVal && props.service) {
      selectedVersion.value = props.service.version || ''
      showLogs.value = false
    }
  }
)

const handleInstall = () => {
  if (props.service) {
    emit('install', props.service.id)
  }
}

const handleVersionChange = (version: any) => {
  if (props.service && version) {
    selectedVersion.value = String(version)
    emit('select-version', props.service.id, String(version))
  }
}

const handleViewLogs = () => {
  if (props.service) {
    emit('view-logs', props.service.id)
    showLogs.value = true
  }
}

const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <Drawer :open="open" @update:open="handleClose">
    <DrawerContent class="max-h-[90vh]">
      <DrawerHeader>
        <DrawerTitle>Configure {{ service?.displayName }}</DrawerTitle>
        <DrawerDescription
          >Manage installation, version, and settings for this service</DrawerDescription
        >
      </DrawerHeader>

      <ScrollArea class="flex-1 px-4 py-4 max-h-[60vh]">
        <div class="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle class="text-base">Installation Status</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4">
              <div class="flex items-center gap-3">
                <div v-if="isInstalled" class="flex items-center gap-2 text-green-600">
                  <CheckCircle2 class="h-5 w-5" />
                  <span class="font-medium">Installed</span>
                </div>
                <div v-else class="flex items-center gap-2 text-red-600">
                  <AlertCircle class="h-5 w-5" />
                  <span class="font-medium">Not Installed</span>
                </div>
              </div>

              <div v-if="!isInstalled" class="pt-2">
                <Button @click="handleInstall" class="w-full sm:w-auto">
                  <Download class="mr-2 h-4 w-4" />
                  Install {{ service?.displayName }}
                </Button>
              </div>
            </CardContent>
          </Card>

          <Card v-if="isInstalled">
            <CardHeader>
              <CardTitle class="text-base">Version Management</CardTitle>
              <CardDescription>
                {{
                  isVersionSelectable
                    ? `Select preferred version (current: ${service?.version})`
                    : `Current version: ${service?.version}`
                }}
              </CardDescription>
            </CardHeader>
            <CardContent class="space-y-4">
              <div v-if="isVersionSelectable" class="w-full">
                <Select :model-value="selectedVersion" @update:model-value="handleVersionChange">
                  <SelectTrigger class="w-full">
                    <SelectValue :placeholder="`Select ${service?.displayName} version`" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem
                      v-for="version in availableVersions"
                      :key="version"
                      :value="version"
                    >
                      {{ service?.displayName }} {{ version }}
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div v-else class="flex items-center gap-2">
                <Badge variant="outline">{{ service?.version }}</Badge>
                <span class="text-sm text-muted-foreground">Latest version in use</span>
              </div>

              <!-- PHP specific: Show FPM and CLI info -->
              <div v-if="isPHPService && isInstalled" class="border-t pt-4 mt-4">
                <p class="text-sm font-medium mb-3">Components included:</p>
                <div class="space-y-3">
                  <div class="flex items-center gap-3 p-3 bg-muted/50 rounded-lg">
                    <Globe class="h-5 w-5 text-blue-500" />
                    <div class="flex-1">
                      <p class="text-sm font-medium">PHP-FPM</p>
                      <p class="text-xs text-muted-foreground">
                        FastCGI Process Manager for web requests
                      </p>
                    </div>
                    <Badge variant="secondary" class="text-xs">Port 9000</Badge>
                  </div>
                  <div class="flex items-center gap-3 p-3 bg-muted/50 rounded-lg">
                    <Terminal class="h-5 w-5 text-green-500" />
                    <div class="flex-1">
                      <p class="text-sm font-medium">PHP-CLI</p>
                      <p class="text-xs text-muted-foreground">
                        Command line interface for scripts & artisan
                      </p>
                    </div>
                    <Badge variant="secondary" class="text-xs font-mono">/usr/bin/php</Badge>
                  </div>
                </div>
                <p class="text-xs text-muted-foreground mt-3">
                  Both components use PHP {{ selectedVersion || service?.version }} simultaneously
                </p>
              </div>
            </CardContent>
          </Card>

          <Card v-if="isInstalled">
            <CardHeader>
              <CardTitle class="text-base">Service Controls</CardTitle>
              <CardDescription>
                {{
                  isPHPService
                    ? 'Start, stop, and manage PHP-FPM service'
                    : 'Start, stop, and manage this service'
                }}
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                <Button
                  variant="outline"
                  :disabled="!canStartStopRestart"
                  @click="service && emit('start', service.id)"
                  class="w-full"
                >
                  Start
                </Button>
                <Button
                  variant="outline"
                  :disabled="!canStop"
                  @click="service && emit('stop', service.id)"
                  class="w-full"
                >
                  Stop
                </Button>
                <Button
                  variant="outline"
                  :disabled="!canStartStopRestart"
                  @click="service && emit('restart', service.id)"
                  class="w-full"
                >
                  Restart
                </Button>
              </div>
              <p v-if="isPHPService" class="text-xs text-muted-foreground mt-3">
                Note: PHP-CLI doesn't require a running service. Only PHP-FPM needs to be started
                for web requests.
              </p>
            </CardContent>
          </Card>

          <Card v-if="isInstalled">
            <CardHeader>
              <CardTitle class="text-base">Service Logs</CardTitle>
              <CardDescription>
                {{
                  isPHPService
                    ? 'View PHP-FPM error and access logs'
                    : 'View and monitor service logs'
                }}
              </CardDescription>
            </CardHeader>
            <CardContent>
              <Button @click="handleViewLogs" variant="outline" class="w-full sm:w-auto">
                <FileText class="mr-2 h-4 w-4" />
                View Logs
              </Button>

              <!-- Logs Drawer -->
              <Drawer :open="showLogs" @update:open="val => (showLogs = val)">
                <DrawerContent class="max-h-[85vh]">
                  <DrawerHeader>
                    <DrawerTitle>{{ service?.displayName }} Logs</DrawerTitle>
                    <DrawerDescription>
                      {{
                        isPHPService ? 'PHP-FPM error and access logs' : 'Real-time service logs'
                      }}
                    </DrawerDescription>
                  </DrawerHeader>

                  <div class="px-4 flex-1 overflow-hidden">
                    <div
                      class="bg-black/5 dark:bg-black/50 rounded-md p-4 h-[40vh] overflow-hidden"
                    >
                      <ScrollArea class="h-full">
                        <pre class="text-sm font-mono text-foreground whitespace-pre-wrap">{{
                          mockLogs
                        }}</pre>
                      </ScrollArea>
                    </div>
                  </div>

                  <DrawerFooter>
                    <DrawerClose as-child>
                      <Button variant="outline" class="w-full"> Close </Button>
                    </DrawerClose>
                  </DrawerFooter>
                </DrawerContent>
              </Drawer>
            </CardContent>
          </Card>

          <Card v-if="service">
            <CardHeader>
              <CardTitle class="text-base">Service Information</CardTitle>
            </CardHeader>
            <CardContent class="space-y-2 text-sm">
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <span class="text-muted-foreground">Service ID:</span>
                  <p class="font-mono text-xs">{{ service.id }}</p>
                </div>
                <div v-if="service.port">
                  <span class="text-muted-foreground">Port:</span>
                  <p class="font-mono">{{ service.port }}</p>
                </div>
                <div>
                  <span class="text-muted-foreground">Status:</span>
                  <p>{{ EnumServiceStatus.getLabel(service.status) }}</p>
                </div>
                <div>
                  <span class="text-muted-foreground">Auto Start:</span>
                  <p>{{ service.autoStart ? 'Enabled' : 'Disabled' }}</p>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </ScrollArea>

      <DrawerFooter>
        <DrawerClose as-child>
          <Button variant="outline" class="w-full">Close</Button>
        </DrawerClose>
      </DrawerFooter>
    </DrawerContent>
  </Drawer>
</template>
