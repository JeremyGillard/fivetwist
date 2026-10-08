<script setup lang="ts">
import BoardQuadrant from './BoardQuadrant.vue'
import { QUADRANTS, type Cell } from '@/game/board'

defineProps<{ cells: readonly Cell[] }>()
</script>

<template>
  <div class="board" role="group" aria-label="Board">
    <BoardQuadrant
      v-for="quadrant in QUADRANTS"
      :key="quadrant"
      :quadrant="quadrant"
      :cells="cells"
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
}
</style>
