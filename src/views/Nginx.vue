<script setup lang="ts">
import { ref } from 'vue'
import { Globe, Settings, LoaderCircle } from 'lucide-vue-next'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import ButtonStart from '@/components/ButtonStart.vue'
import ServiceStatus from '@/components/ServiceStatus.vue'
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from '@/components/ui/table'
import { EnumServiceStatus } from '@/enums/EnumServiceStatus'
import ButtonStop from '@/components/ButtonStop.vue'
import ButtonRestart from '@/components/ButtonRestart.vue'
import { useNginx } from '@/composables/useNginx'
const { nginxRes, start, stop, restart, loading } = useNginx()
const websites = ref([
  { name: 'localhost', root: '/var/www/html', enabled: true },
  { name: 'lemp-app.test', root: '/var/www/html/lemp-app', enabled: true },
  { name: 'example.test', root: '/var/www/html/example', enabled: false },
])

const handleEditConfig = (name: string) => console.log('Edit config for:', name)
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div class="flex gap-x-2">
        <span class="text-xl font-bold">Nginx</span>
        <span class="text-muted-foreground text-xs">v{{ nginxRes?.data.version }}</span>
        <span><LoaderCircle class="h-4 w-4 animate-spin"></LoaderCircle></span>
      </div>
      <div class="flex gap-2">
        <ServiceStatus
          v-if="
            nginxRes?.data.status !== undefined &&
            nginxRes.data.status !== EnumServiceStatus.NOT_INSTALLED
          "
          :status="nginxRes?.data.status"
        />

        <template v-if="nginxRes?.data.status === EnumServiceStatus.RUNNING">
          <ButtonStop :status="EnumServiceStatus.RUNNING" @click="stop" :loading="loading" />
          <ButtonRestart :status="EnumServiceStatus.RUNNING" @click="restart" :loading="loading" />
        </template>

        <template v-else-if="nginxRes?.data.status === EnumServiceStatus.STOPPED">
          <ButtonStart :status="EnumServiceStatus.STOPPED" @click="start" :loading="loading" />
        </template>
      </div>
    </div>

    <!--data table-->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2 text-sm font-medium">
          <Globe class="h-4 w-4" />
          Websites
        </CardTitle>
      </CardHeader>
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
              <TableHead>Root</TableHead>
              <TableHead>Status</TableHead>
              <TableHead class="w-[120px]">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="site in websites" :key="site.name">
              <TableCell class="font-medium">{{ site.name }}</TableCell>
              <TableCell class="text-muted-foreground">{{ site.root }}</TableCell>
              <TableCell>
                <Badge :variant="site.enabled ? 'default' : 'secondary'" class="capitalize">
                  {{ site.enabled ? 'Enabled' : 'Disabled' }}
                </Badge>
              </TableCell>
              <TableCell>
                <Button variant="ghost" size="sm" @click="handleEditConfig(site.name)">
                  <Settings class="h-4 w-4 mr-1" /> Edit
                </Button>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
