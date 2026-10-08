<script setup lang="ts">
import { onMounted, useId, useTemplateRef, watch } from 'vue'

const props = defineProps<{
  open: boolean
  title: string
}>()

const emit = defineEmits<{ close: [] }>()

const dialog = useTemplateRef('dialog')
const titleId = useId()

// The native modal dialog traps focus, closes on Escape and gives focus back on closing.
// The fallbacks are for environments without modal dialogs, such as the unit tests' DOM.
function sync(open: boolean) {
  const element = dialog.value
  if (!element || element.hasAttribute('open') === open) return
  if (open) {
    if (typeof element.showModal === 'function') element.showModal()
    else element.setAttribute('open', '')
  } else if (typeof element.close === 'function') {
    element.close()
  } else {
    element.removeAttribute('open')
  }
}

watch(() => props.open, sync, { flush: 'post' })
onMounted(() => sync(props.open))

/** A click on the dimmed area around the dialog lands on the dialog element itself. */
function onClick(event: MouseEvent) {
  if (event.target === dialog.value) emit('close')
}
</script>

<template>
  <dialog
    ref="dialog"
    class="dialog"
    :aria-labelledby="titleId"
    @close="emit('close')"
    @click="onClick"
  >
    <div class="dialog__body">
      <h2 :id="titleId" class="dialog__title">{{ title }}</h2>
      <slot />
      <div class="dialog__actions">
        <slot name="actions" />
      </div>
    </div>
  </dialog>
</template>

<style scoped>
.dialog {
  width: min(32rem, calc(100vw - 2rem));
  max-height: calc(100dvh - 2rem);
  padding: 0;
  border: 1px solid var(--line);
  border-radius: 1.25rem;
  background: var(--surface);
  color: var(--text);
  box-shadow: 0 30px 80px rgb(0 0 0 / 0.6);
}

.dialog::backdrop {
  background: rgb(5 8 10 / 0.7);
}

.dialog__body {
  padding: 1.75rem;
}

.dialog__title {
  margin: 0 0 1rem;
  font-size: 1.25rem;
  font-weight: 700;
}

.dialog__actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 0.75rem;
  margin-top: 1.5rem;
}

.dialog :deep(p),
.dialog :deep(li) {
  color: #c9cdd0;
}

.dialog :deep(.dialog__button) {
  padding: 0.6rem 1.25rem;
  border: 1.5px solid var(--line);
  border-radius: 999px;
  background: transparent;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
}

.dialog :deep(.dialog__button:hover) {
  border-color: var(--muted);
}

.dialog :deep(.dialog__button--primary) {
  border-color: var(--amber);
  background: var(--amber);
  color: #1a1206;
}

.dialog :deep(.dialog__button:focus-visible) {
  outline: 2px solid var(--amber);
  outline-offset: 2px;
}
</style>
