<template>
  <div class="p-2">
    <div class="flex items-center gap-2 justify-center mb-4">
      <img class="block max-w-8 h-auto aspect-square" src="@/assets/logo.png" alt="logo" />
      <span class="text-base font-semibold uppercase">Local Box</span>
    </div>

    <Table class="border">
      <TableCaption>A list of your services.</TableCaption>
      <TableHeader>
        <TableRow>
          <TableHead class="w-25 font-semibold"> Service </TableHead>
          <TableHead class="font-semibold">Version</TableHead>
          <TableHead class="font-semibold">Status</TableHead>
          <TableHead class="font-semibold"> Action </TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow v-for="service in services" :key="service.name">
          <TableCell class="font-medium">
            {{ service.name }}
          </TableCell>
          <TableCell>{{ service.version }}</TableCell>
          <TableCell>
            <span
              v-if="service.status === 1"
              class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800"
            >
              Running
            </span>
            <span
              v-if="service.status === 2"
              class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800"
            >
              Stopped
            </span>

            <span
              v-if="service.status === 3"
              class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800"
            >
              Not installed
            </span>
          </TableCell>
          <TableCell>
            <ButtonGroup v-if="service.status !== 3">
              <Button variant="outline" size="sm">
                <Play v-if="service.status === 2" class="w-4 h-4 mr-2" />
                <StopCircle v-else class="w-4 h-4 mr-2" />
                {{ service.status === 2 ? 'Start' : 'Stop' }}
              </Button>
              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button variant="outline" size="sm" aria-label="More Options">
                    <EllipsisVertical />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" class="w-52">
                  <DropdownMenuGroup>
                    <DropdownMenuItem> Install </DropdownMenuItem>
                    <DropdownMenuItem> Change Version </DropdownMenuItem>
                    <DropdownMenuItem> Config </DropdownMenuItem>
                    <DropdownMenuItem> Logs </DropdownMenuItem>
                    <DropdownMenuItem> System Status </DropdownMenuItem>
                  </DropdownMenuGroup>
                </DropdownMenuContent>
              </DropdownMenu>
            </ButtonGroup>
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  </div>
</template>

<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { ButtonGroup } from '@/components/ui/button-group'
import { Play, StopCircle, EllipsisVertical } from 'lucide-vue-next'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'

interface Services {
  name: string
  version: string
  status: number
  action: string
}

const services: Services[] = [
  {
    name: 'PHP',
    version: '8.4',
    status: 1,
    action: 'Update',
  },
  {
    name: 'Nginx',
    version: '8.4',
    status: 2,
    action: 'Update',
  },
  {
    name: 'Mysql',
    version: '8.4',
    status: 2,
    action: 'Update',
  },
  {
    name: 'Supervisord',
    version: '8.4',
    status: 1,
    action: 'Update',
  },
  {
    name: 'Redis',
    version: '8.4',
    status: 3,
    action: 'Update',
  },
  {
    name: 'MailPit',
    version: '8.4',
    status: 3,
    action: 'Update',
  },
]
</script>
