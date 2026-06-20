<script setup lang="ts">
import { TabsRoot } from 'reka-ui'
import type { TabsRootEmits, TabsRootProps } from 'reka-ui'

const props = defineProps<TabsRootProps>()
const emits = defineEmits<TabsRootEmits>()
</script>

<template>
  <TabsRoot v-bind="props" @update:model-value="(val) => emits('update:modelValue', val)">
    <slot />
  </TabsRoot>
</template>
