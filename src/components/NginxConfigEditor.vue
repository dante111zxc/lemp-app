<script setup lang="ts">
import { ref, watch } from 'vue'
import { VueMonacoEditor } from '@guolao/vue-monaco-editor'
import { Button } from '@/components/ui/button'
import { LoaderCircle, X } from 'lucide-vue-next'
import type { NginxWebsite } from '@/types/nginx'

const props = defineProps<{
  website: NginxWebsite | null
  content: string
  loading: boolean
}>()

const emit = defineEmits<{
  save: [content: string]
  close: []
}>()

const editorContent = ref(props.content)
const isSaving = ref(false)

watch(
  () => props.content,
  val => {
    editorContent.value = val
  }
)

const handleSave = () => {
  isSaving.value = true
  emit('save', editorContent.value)
  setTimeout(() => {
    isSaving.value = false
  }, 1000)
}

const handleClose = () => {
  emit('close')
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="!!website"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/50"
      @click.self="handleClose"
    >
      <div class="bg-background rounded-lg shadow-lg w-full max-w-4xl h-[95vh] flex flex-col m-4">
        <div class="flex items-center justify-between px-6 py-2 border-b">
          <div class="flex items-center gap-2 min-w-0">
            <span class="font-semibold truncate">Edit Config: {{ website?.name }}</span>
            <span class="text-xs text-muted-foreground truncate hidden sm:inline">
              {{ website?.file_path }}
            </span>
            <LoaderCircle v-if="loading" class="h-4 w-4 animate-spin shrink-0" />
          </div>
          <Button variant="ghost" size="sm" @click="handleClose">
            <X class="h-4 w-4" />
          </Button>
        </div>

        <div class="flex-1 min-h-0 border-b mx-6 my-2 rounded overflow-hidden">
          <VueMonacoEditor
            v-if="!!website"
            v-model:value="editorContent"
            language="nginx"
            theme="vs-dark"
            :options="{
              minimap: { enabled: false },
              fontSize: 14,
              lineNumbers: 'on',
              scrollBeyondLastLine: false,
              wordWrap: 'on',
              automaticLayout: true,
            }"
            class="h-full w-full"
          />
        </div>

        <div class="flex justify-end gap-2 px-4 py-2 border-t">
          <Button variant="outline" @click="handleClose">Cancel</Button>
          <Button @click="handleSave" :disabled="isSaving">
            <LoaderCircle v-if="isSaving" class="h-4 w-4 mr-1 animate-spin" />
            Save
          </Button>
        </div>
      </div>
    </div>
  </Teleport>
</template>
