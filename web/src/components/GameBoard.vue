<script setup lang="ts">
import BoardQuadrant, { type QuadrantMode } from './BoardQuadrant.vue'
import { QUADRANTS, type Cell, type Position, type Twist } from '@/game/board'
import type { Side } from '@/game/sides'
import type { Quadrant } from '@/wasm/pentago.js'

defineProps<{
  cells: readonly Cell[]
  /** Indexes in `cells` of the marbles forming a winning line. */
  winning: readonly number[]
  mode: QuadrantMode
  /** The side to move. */
  current: Side
  selected: Quadrant | null
  twist: Twist | null
}>()

const emit = defineEmits<{
  place: [position: Position]
  select: [quadrant: Quadrant]
  twisted: []
}>()
</script>

<template>
  <div class="board" role="group" aria-label="Board">
    <BoardQuadrant
      v-for="quadrant in QUADRANTS"
      :key="quadrant"
      :quadrant="quadrant"
      :cells="cells"
      :winning="winning"
      :mode="mode"
      :current="current"
      :selected="selected === quadrant"
      :twisting="twist?.quadrant === quadrant ? twist.direction : null"
      @place="emit('place', $event)"
      @select="emit('select', quadrant)"
      @twisted="emit('twisted')"
    />
    <span class="pivot" aria-hidden="true" />
  </div>
</template>

<style scoped>
.board {
  position: relative;
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 2.2%;
  width: min(100%, 36rem);
  /* Keep the whole board on screen on a short, wide window. */
  max-width: max(17rem, calc(100dvh - 14rem));
  padding: 2.6%;
  aspect-ratio: 1;
  border-radius: 6%;
  background: var(--well);
  box-shadow: 0 24px 60px rgb(0 0 0 / 0.45);
}

.pivot {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 5.2%;
  aspect-ratio: 1;
  border-radius: 50%;
  translate: -50% -50%;
  background: #a9772f;
  pointer-events: none;
}
</style>
