<template>
  <Button variant="outline" size="sm" @click="handleStart" :disabled="serviceStatus === 'running'">
    <Play class="h-4 w-4 mr-1" /> Start
  </Button>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { Play, Square, RotateCcw, Globe, Settings } from 'lucide-vue-next'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'

const serviceStatus = ref('running')

const handleStart = () => {
  console.log('handle start')
}
</script>
