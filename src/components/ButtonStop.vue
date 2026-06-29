<template>
  <button
    class="inline-flex items-center justify-center rounded-lg px-2 py-2 transition-all duration-300"
    :class="[
      isDisabled
        ? 'cursor-not-allowed opacity-40 bg-gray-100 text-gray-400'
        : 'cursor-pointer hover:ring-4 hover:ring-red-200 hover:bg-red-50',
    ]"
    @click="handleClick"
    :disabled="isDisabled"
  >
    <Square class="h-4 w-4 transition-all" :class="[isDisabled ? 'text-black' : 'text-red-600']" />
  </button>
</template>

<script lang="ts" setup>
import { EnumServiceStatus } from '@/enums/EnumServiceStatus'
import { computed } from 'vue'
import { Square } from 'lucide-vue-next'

const props = defineProps<{
  status: number
  loading: boolean
}>()

const emit = defineEmits<{
  (e: 'update', value: number): void
}>()

const isDisabled = computed(() => {
  return props.status !== EnumServiceStatus.RUNNING || props.loading
})

const handleClick = () => {
  emit('update', props.status)
}
</script>
