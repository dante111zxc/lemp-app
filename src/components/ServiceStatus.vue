<template>
  <span
    class="inline-flex items-center gap-2 border rounded-full px-4 py-2 text-sm font-medium transition-all duration-300 hover:-translate-y-0.5"
    :class="btnClass"
  >
    <span class="relative flex h-2.5 w-2.5">
      <span
        class="animate-ping absolute inline-flex h-full w-full rounded-full opacity-75"
        :class="bgClass"
      ></span>
      <span class="relative inline-flex rounded-full h-2.5 w-2.5" :class="bgClass"></span>
    </span>
    <span
      class="text-xs"
      :class="{ 'text-green-500': props.status === 1, 'text-red-500': props.status === 0 }"
      >{{ text }}</span
    >
  </span>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
const props = defineProps<{
  status: number
}>()

const btnClass = computed(() => {
  return props.status === 0
    ? 'bg-red-50 text-red border-red-200 hover:bg-red-100'
    : 'bg-green-50 text-green border-green-200 hover:bg-green-100'
})

const bgClass = computed(() => {
  return props.status == 0 ? 'bg-red-500' : 'bg-green-500'
})

const text = computed(() => {
  return props.status === 0 ? 'Stop' : 'Running'
})
</script>
