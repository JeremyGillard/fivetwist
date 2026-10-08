<script setup lang="ts">
import GameMarble from './GameMarble.vue'
import type { Side } from '@/game/sides'

defineProps<{
  side: Side
  /** Marbles this side has on the board. */
  placed: number
  /** Whether it is this side's turn. */
  toMove: boolean
}>()
</script>

<template>
  <article class="card" :class="{ 'card--to-move': toMove }">
    <span class="card__marble"><GameMarble :side="side" /></span>
    <div class="card__text">
      <h2 class="card__name">{{ side.name }}</h2>
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

.card__count {
  margin: 0.1rem 0 0;
  font-size: 0.8rem;
  color: var(--muted);
}

.card__tag {
  font-size: 0.72rem;
  font-weight: 600;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: var(--amber);
}
</style>
