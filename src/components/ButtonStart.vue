<template>
  <Play
    class="h-4 w-4 mr-1 text-green-600 cursor-pointer"
    @click="handleStart"
    :disabled="
      props.status === EnumServiceStatus.NOT_INSTALLED || props.status === EnumServiceStatus.RUNNING
    "
  />
</template>

<script lang="ts" setup>
import { Play } from 'lucide-vue-next'
import { EnumServiceStatus } from '@/enums/EnumServiceStatus'

const props = defineProps<{
  status: number
}>()

const handleStart = () => {
  console.log('handle start')
}
</script>
