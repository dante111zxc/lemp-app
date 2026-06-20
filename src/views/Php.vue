<script setup lang="ts">
import { ref } from 'vue'
import {
  Play, Square, RotateCcw, Cog, Puzzle, Check, X, Plus, Pencil, Save,
} from 'lucide-vue-next'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Tabs, TabsList, TabsTrigger, TabsContent } from '@/components/ui/tabs'
import { Table, TableHeader, TableRow, TableHead, TableBody, TableCell } from '@/components/ui/table'

const activeTab = ref('php')

const serviceStatus = ref<'running' | 'stopped' | 'error'>('running')
const serviceVersion = ref('8.3.6')
const servicePort = ref(9000)

const editingKey = ref<string | null>(null)
const editValue = ref<string>('')

const inputSettings = new Set([
  'memory_limit',
  'max_execution_time',
  'max_input_time',
  'max_input_vars',
  'max_post_size',
  'post_max_size',
  'upload_max_filesize',
])

const selectOptions: Record<string, string[]> = {
  'date.timezone': [
    'UTC', 'America/New_York', 'America/Chicago', 'America/Denver',
    'America/Los_Angeles', 'Europe/London', 'Europe/Paris', 'Europe/Berlin',
    'Europe/Moscow', 'Asia/Tokyo', 'Asia/Shanghai', 'Asia/Ho_Chi_Minh',
    'Asia/Singapore', 'Asia/Dubai', 'Australia/Sydney', 'Pacific/Auckland',
  ],
  'display_errors': ['On', 'Off', 'stderr', 'stdout'],
  'error_reporting': [
    'E_ALL', 'E_ALL & ~E_DEPRECATED & ~E_STRICT',
    'E_ALL & ~E_NOTICE', 'E_ALL & ~E_NOTICE & ~E_DEPRECATED',
    'E_ERROR', 'E_WARNING', 'E_PARSE', 'E_NOTICE', 'E_STRICT',
    'E_DEPRECATED', 'E_CORE_ERROR', 'E_COMPILE_ERROR', 'E_USER_ERROR',
  ],
}

const phpSettings = ref([
  { key: 'memory_limit', value: '256M' },
  { key: 'max_execution_time', value: '300' },
  { key: 'max_input_time', value: '300' },
  { key: 'max_input_vars', value: '1000' },
  { key: 'max_post_size', value: '128M' },
  { key: 'post_max_size', value: '128M' },
  { key: 'upload_max_filesize', value: '128M' },
  { key: 'date.timezone', value: 'UTC' },
  { key: 'display_errors', value: 'On' },
  { key: 'error_reporting', value: 'E_ALL' },
])

const extensions = ref([
  { name: 'bcmath', enabled: true },
  { name: 'curl', enabled: true },
  { name: 'gd', enabled: true },
  { name: 'intl', enabled: true },
  { name: 'mbstring', enabled: true },
  { name: 'mysql', enabled: true },
  { name: 'opcache', enabled: true },
  { name: 'pdo', enabled: true },
  { name: 'redis', enabled: true },
  { name: 'xml', enabled: true },
  { name: 'zip', enabled: true },
  { name: 'imagick', enabled: false, installed: false },
  { name: 'exif', enabled: false, installed: true },
  { name: 'sodium', enabled: false, installed: true },
  { name: 'xdebug', enabled: false, installed: false },
])

const handleStart = () => console.log('Start PHP')
const handleStop = () => console.log('Stop PHP')
const handleRestart = () => console.log('Restart PHP')

const handleEdit = (key: string) => {
  const setting = phpSettings.value.find(s => s.key === key)
  if (setting) {
    editValue.value = setting.value
    editingKey.value = key
  }
}

const handleSave = (key: string) => {
  const setting = phpSettings.value.find(s => s.key === key)
  if (setting) {
    setting.value = editValue.value
    console.log(`Saved ${key} = ${editValue.value}`)
  }
  editingKey.value = null
}

const handleCancel = () => {
  editingKey.value = null
}

const handleInstallExtension = (name: string) => console.log('Install extension:', name)
const handleToggleExtension = (name: string, enable: boolean) => console.log(enable ? 'Enable' : 'Disable', 'extension:', name)
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold">PHP</h1>
        <p class="text-muted-foreground">PHP-FPM management</p>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" size="sm" @click="handleStart" :disabled="serviceStatus === 'running'">
          <Play class="h-4 w-4 mr-1" /> Start
        </Button>
        <Button variant="outline" size="sm" @click="handleStop" :disabled="serviceStatus !== 'running'">
          <Square class="h-4 w-4 mr-1" /> Stop
        </Button>
        <Button variant="outline" size="sm" @click="handleRestart">
          <RotateCcw class="h-4 w-4 mr-1" /> Restart
        </Button>
      </div>
    </div>

    <div class="grid gap-4 md:grid-cols-3">
      <Card>
        <CardHeader class="pb-2">
          <CardTitle class="text-sm font-medium">Status</CardTitle>
        </CardHeader>
        <CardContent>
          <Badge
            :variant="serviceStatus === 'running' ? 'default' : 'secondary'"
            class="capitalize"
          >
            {{ serviceStatus }}
          </Badge>
        </CardContent>
      </Card>
      <Card>
        <CardHeader class="pb-2">
          <CardTitle class="text-sm font-medium">PHP-FPM Port</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-lg font-bold">{{ servicePort }}</div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader class="pb-2">
          <CardTitle class="text-sm font-medium">Version</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-lg font-bold">PHP {{ serviceVersion }}</div>
        </CardContent>
      </Card>
    </div>

    <Tabs default-value="php" v-model="activeTab">
      <TabsList>
        <TabsTrigger value="php" class="flex items-center gap-2">
          <Cog class="h-4 w-4" />
          PHP Settings
        </TabsTrigger>
        <TabsTrigger value="extensions" class="flex items-center gap-2">
          <Puzzle class="h-4 w-4" />
          Extensions
        </TabsTrigger>
      </TabsList>

      <TabsContent value="php" class="mt-4">
        <Card>
          <CardHeader>
            <CardTitle class="text-sm font-medium">PHP Configuration</CardTitle>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Directive</TableHead>
                  <TableHead>Value</TableHead>
                  <TableHead class="w-[120px]">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                <TableRow v-for="setting in phpSettings" :key="setting.key">
                  <TableCell class="font-mono text-sm">{{ setting.key }}</TableCell>
                  <TableCell class="font-mono text-sm">
                    <div v-if="editingKey === setting.key" class="flex items-center gap-2">
                      <Input
                        v-if="inputSettings.has(setting.key)"
                        v-model="editValue"
                        class="h-8 w-40"
                      />
                      <Select
                        v-else
                        :default-value="setting.value"
                        @update:model-value="(val) => editValue = String(val)"
                      >
                        <SelectTrigger class="w-56 h-8">
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem
                            v-for="opt in selectOptions[setting.key]"
                            :key="opt"
                            :value="opt"
                          >
                            {{ opt }}
                          </SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                    <span v-else>{{ setting.value }}</span>
                  </TableCell>
                  <TableCell>
                    <div v-if="editingKey === setting.key" class="flex gap-1">
                      <Button variant="ghost" size="sm" @click="handleSave(setting.key)">
                        <Save class="h-4 w-4" />
                      </Button>
                      <Button variant="ghost" size="sm" @click="handleCancel">
                        <X class="h-4 w-4" />
                      </Button>
                    </div>
                    <Button
                      v-else
                      variant="ghost"
                      size="sm"
                      @click="handleEdit(setting.key)"
                    >
                      <Pencil class="h-4 w-4" />
                    </Button>
                  </TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </TabsContent>

      <TabsContent value="extensions" class="mt-4">
        <Card>
          <CardHeader>
            <CardTitle class="text-sm font-medium">PHP Extensions</CardTitle>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Extension</TableHead>
                  <TableHead>Status</TableHead>
                  <TableHead class="w-[180px]">Actions</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                <TableRow v-for="ext in extensions" :key="ext.name">
                  <TableCell class="font-medium">{{ ext.name }}</TableCell>
                  <TableCell>
                    <Badge
                      v-if="ext.enabled"
                      variant="default"
                      class="gap-1"
                    >
                      <Check class="h-3 w-3" /> Active
                    </Badge>
                    <Badge
                      v-else-if="ext.installed"
                      variant="secondary"
                      class="gap-1"
                    >
                      <X class="h-3 w-3" /> Inactive
                    </Badge>
                    <Badge
                      v-else
                      variant="outline"
                      class="gap-1"
                    >
                      Not installed
                    </Badge>
                  </TableCell>
                  <TableCell>
                    <div class="flex gap-1">
                      <Button
                        v-if="ext.installed && !ext.enabled"
                        variant="ghost"
                        size="sm"
                        @click="handleToggleExtension(ext.name, true)"
                      >
                        Enable
                      </Button>
                      <Button
                        v-if="ext.enabled"
                        variant="ghost"
                        size="sm"
                        @click="handleToggleExtension(ext.name, false)"
                      >
                        Disable
                      </Button>
                      <Button
                        v-if="!ext.installed"
                        variant="outline"
                        size="sm"
                        @click="handleInstallExtension(ext.name)"
                      >
                        <Plus class="h-3 w-3 mr-1" /> Install
                      </Button>
                    </div>
                  </TableCell>
                </TableRow>
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      </TabsContent>
    </Tabs>
  </div>
</template>
