<script setup lang="ts">
import { ref } from 'vue'
import { Play, Square, RotateCcw, HardDrive } from 'lucide-vue-next'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'

const serviceStatus = ref<'running' | 'stopped' | 'error'>('running')
const servicePort = ref(6379)
const serviceVersion = ref('7.2.4')

const handleStart = () => console.log('Start Redis')
const handleStop = () => console.log('Stop Redis')
const handleRestart = () => console.log('Restart Redis')
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold">Redis</h1>
        <p class="text-muted-foreground">In-memory data store management</p>
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
          <CardTitle class="text-sm font-medium">Port</CardTitle>
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
          <div class="text-lg font-bold">{{ serviceVersion }}</div>
        </CardContent>
      </Card>
    </div>

    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2 text-sm font-medium">
          <HardDrive class="h-4 w-4" />
          Info
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="grid gap-4 md:grid-cols-3">
          <div>
            <p class="text-sm text-muted-foreground">Uptime</p>
            <p class="text-lg font-bold">12d 4h 32m</p>
          </div>
          <div>
            <p class="text-sm text-muted-foreground">Connected Clients</p>
            <p class="text-lg font-bold">3</p>
          </div>
          <div>
            <p class="text-sm text-muted-foreground">Memory Usage</p>
            <p class="text-lg font-bold">2.1 MB</p>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
