<script setup lang="ts">
import { nextTick, ref, useTemplateRef } from 'vue'

import GameMarble from './GameMarble.vue'
import { MAX_NAME_LENGTH } from '@/composables/usePlayers'
import type { Side } from '@/game/sides'

const props = defineProps<{
  side: Side
  /** The name of the player on this side. */
  name: string
  /** Marbles this side has on the board. */
  placed: number
  /** Whether it is this side's turn. */
  toMove: boolean
}>()

const emit = defineEmits<{ rename: [name: string] }>()

const editing = ref(false)
const draft = ref('')
const input = useTemplateRef('input')
const nameButton = useTemplateRef('nameButton')

async function startEditing() {
  draft.value = props.name
  editing.value = true
  await nextTick()
  input.value?.select()
}

async function stopEditing(save: boolean) {
  if (!editing.value) return
  editing.value = false
  if (save) emit('rename', draft.value)
  await nextTick()
  nameButton.value?.focus()
}
</script>

<template>
  <article class="card" :class="{ 'card--to-move': toMove }">
    <span class="card__marble"><GameMarble :side="side" /></span>
    <div class="card__text">
      <h2 class="card__name">
        <input
          v-if="editing"
          ref="input"
          v-model="draft"
          class="card__input"
          type="text"
          :maxlength="MAX_NAME_LENGTH"
          :aria-label="`Name of the ${side.name} player`"
          autocomplete="off"
          spellcheck="false"
          @keydown.enter.prevent="stopEditing(true)"
          @keydown.escape.prevent="stopEditing(false)"
          @blur="stopEditing(true)"
        />
        <button
          v-else
          ref="nameButton"
          class="card__rename"
          type="button"
          :title="`Rename the ${side.name} player`"
          @click="startEditing"
        >
          {{ name }}
        </button>
      </h2>
      <p class="card__count">{{ placed }} {{ placed === 1 ? 'marble' : 'marbles' }} placed</p>
    </div>
    <span v-if="toMove" class="card__tag">To move</span>
  </article>
</template>

<style scoped>
.card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.1rem 1.25rem;
  border: 2px solid transparent;
  border-radius: 1.1rem;
  background: var(--surface);
}

.card--to-move {
  border-color: var(--amber);
}

.card__marble {
  flex: none;
  width: 2.5rem;
}

.card__text {
  flex: 1;
  min-width: 0;
}

.card__name {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
}

.card__rename,
.card__input {
  display: block;
  width: 100%;
  margin: -0.15rem -0.35rem;
  padding: 0.15rem 0.35rem;
  border: 1px solid transparent;
  border-radius: 0.4rem;
  background: transparent;
  font: inherit;
  color: inherit;
  text-align: left;
}

.card__rename {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  cursor: text;
}

.card__rename:hover {
  border-color: var(--line);
}

.card__input {
  border-color: var(--amber);
  background: var(--well);
  outline: none;
}

.card__rename:focus-visible {
  outline: 2px solid var(--amber);
  outline-offset: 1px;
}

.card__count {
  margin: 0.1rem 0 0;
  font-size: 0.8rem;
  color: var(--muted);
}

.card__tag {
  flex: none;
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: var(--amber);
}

@media (max-width: 30rem) {
  .card {
    flex-wrap: wrap;
    gap: 0.6rem;
    padding: 0.75rem 0.85rem;
  }

  .card__marble {
    width: 1.75rem;
  }

  .card__tag {
    flex-basis: 100%;
    font-size: 0.65rem;
  }
}
</style>
