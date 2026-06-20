<script setup lang="ts">
import { ref } from 'vue'
import { Play, Square, RotateCcw } from 'lucide-vue-next'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'

const serviceStatus = ref<'running' | 'stopped' | 'error'>('running')
const servicePort = ref(1025)
const serviceWebPort = ref(8025)
const serviceVersion = ref('1.18.0')

const handleStart = () => console.log('Start Mailpit')
const handleStop = () => console.log('Stop Mailpit')
const handleRestart = () => console.log('Restart Mailpit')
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold">Mailpit</h1>
        <p class="text-muted-foreground">Email testing tool management</p>
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

    <div class="grid gap-4 md:grid-cols-4">
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
          <CardTitle class="text-sm font-medium">SMTP Port</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-lg font-bold">{{ servicePort }}</div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader class="pb-2">
          <CardTitle class="text-sm font-medium">Web UI Port</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-lg font-bold">{{ serviceWebPort }}</div>
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
  </div>
</template>
