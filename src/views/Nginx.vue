<script setup lang="ts">
import { Globe, Settings, LoaderCircle, Ban } from 'lucide-vue-next'
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
import NginxConfigEditor from '@/components/NginxConfigEditor.vue'
import { useNginx } from '@/composables/useNginx'
const {
  nginxRes,
  websites,
  configContent,
  editingWebsite,
  loading,
  error,
  start,
  stop,
  restart,
  show,
  update,
  closeEditor,
} = useNginx()
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div class="flex gap-x-2">
        <span class="text-xl font-bold">Nginx</span>
        <span v-if="nginxRes?.data?.version" class="text-muted-foreground text-xs"
          >v{{ nginxRes?.data.version }}</span
        >

        <span v-if="loading"><LoaderCircle class="h-4 w-4 animate-spin"></LoaderCircle></span>
      </div>
      <div class="flex gap-2">
        <ServiceStatus
          v-if="nginxRes?.data.status !== EnumServiceStatus.NOT_INSTALLED"
          :status="nginxRes?.data.status as number"
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

    <div
      v-if="error"
      class="rounded-lg border border-destructive/50 bg-destructive/10 p-4 text-destructive text-sm"
    >
      {{ error }}
    </div>

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
                <Button variant="ghost" size="sm" @click="show(site)">
                  <Settings class="h-4 w-4 mr-1" /> Edit
                </Button>
              </TableCell>
            </TableRow>

            <TableRow v-if="websites.length === 0 && !loading">
              <TableCell colspan="4" class="text-center text-muted-foreground py-6">
                <Ban class="h-4 w-4 inline mr-2" />
                No websites found
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>

    <NginxConfigEditor
      :website="editingWebsite"
      :content="configContent"
      :loading="loading"
      @save="(content: string) => update(content)"
      @close="closeEditor"
    />
  </div>
</template>
