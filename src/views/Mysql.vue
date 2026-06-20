<script setup lang="ts">
import { ref } from 'vue'
import { Play, Square, RotateCcw, Database } from 'lucide-vue-next'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Table as UITable, TableHeader, TableRow, TableHead, TableBody, TableCell } from '@/components/ui/table'

const serviceStatus = ref<'running' | 'stopped' | 'error'>('running')
const servicePort = ref(3306)
const serviceVersion = ref('8.0.33')

const databases = ref([
  { name: 'lemp_app', size: '12.5 MB', users: 1 },
  { name: 'test_db', size: '2.3 MB', users: 1 },
  { name: 'wordpress', size: '45.8 MB', users: 2 },
])

const handleStart = () => console.log('Start MySQL')
const handleStop = () => console.log('Stop MySQL')
const handleRestart = () => console.log('Restart MySQL')
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold">MySQL</h1>
        <p class="text-muted-foreground">Database server management</p>
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
          <Database class="h-4 w-4" />
          Databases
        </CardTitle>
      </CardHeader>
      <CardContent>
        <UITable>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>Size</TableHead>
              <TableHead>Users</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="db in databases" :key="db.name">
              <TableCell class="font-medium">{{ db.name }}</TableCell>
              <TableCell class="text-muted-foreground">{{ db.size }}</TableCell>
              <TableCell>{{ db.users }}</TableCell>
            </TableRow>
          </TableBody>
        </UITable>
      </CardContent>
    </Card>
  </div>
</template>
