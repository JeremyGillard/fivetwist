<script setup lang="ts">
import BaseDialog from './BaseDialog.vue'

defineProps<{
  open: boolean
  title: string
  message: string
  confirmLabel: string
  cancelLabel: string
}>()

const emit = defineEmits<{ confirm: []; cancel: [] }>()
</script>

<template>
  <BaseDialog :open="open" :title="title" @close="emit('cancel')">
    <p class="message">{{ message }}</p>
    <template #actions>
      <button class="dialog__button" type="button" @click="emit('cancel')">
        {{ cancelLabel }}
      </button>
      <button class="dialog__button dialog__button--primary" type="button" @click="emit('confirm')">
        {{ confirmLabel }}
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.message {
  margin: 0;
}
</style>
