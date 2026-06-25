<template>
  <button
    class="inline-flex items-center justify-center rounded-lg px-2 py-2 transition-all duration-300"
    :class="[
      isDisabled
        ? 'cursor-not-allowed opacity-40 bg-gray-100 text-gray-400'
        : 'cursor-pointer hover:ring-4 hover:ring-green-200 hover:bg-green-50',
    ]"
    @click="handleClick"
    :disabled="isDisabled"
  >
    <Play class="h-4 w-4 transition-all" :class="[isDisabled ? 'text-black' : 'text-green-600']" />
  </button>
</template>

<script lang="ts" setup>
import { Play } from 'lucide-vue-next'
import { EnumServiceStatus } from '@/enums/EnumServiceStatus'
import { computed, ref } from 'vue'

const props = defineProps<{
  status: number
}>()

const serviceStatus = ref(props.status)

const emit = defineEmits<{
  (e: 'update', value: number): void
}>()

//nếu service đang chạy chưa cài đặt thì bị disabled
const isDisabled = computed(() => {
  return serviceStatus.value !== EnumServiceStatus.STOPPED
})

const handleClick = () => {
  console.log('handle start')
  emit('update', serviceStatus.value)
}
</script>
