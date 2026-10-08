<script setup lang="ts">
import { computed } from 'vue'

import GameMarble from './GameMarble.vue'
import { cellIndex, EMPTY, quadrantPositions, type Cell } from '@/game/board'
import { SIDES } from '@/game/sides'
import type { Quadrant } from '@/wasm/pentago.js'

const props = defineProps<{
  quadrant: Quadrant
  /** The whole board; the quadrant picks its own nine cells. */
  cells: readonly Cell[]
}>()

const holes = computed(() =>
  quadrantPositions(props.quadrant).map((position) => {
    const cell = props.cells[cellIndex(position)] ?? EMPTY
    const side = cell === EMPTY ? null : SIDES[cell]
    return {
      key: cellIndex(position),
      side,
      label: `Row ${position.row + 1}, column ${position.col + 1}: ${side?.name ?? 'empty'}`,
    }
  }),
)
</script>

<template>
  <div class="quadrant">
    <div v-for="hole in holes" :key="hole.key" class="hole" role="img" :aria-label="hole.label">
      <GameMarble v-if="hole.side" :side="hole.side" />
    </div>
  </div>
</template>

<style scoped>
.quadrant {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 9.5%;
  padding: 5%;
  aspect-ratio: 1;
  border-radius: 9%;
  border: 1px solid var(--tile-edge);
  background: linear-gradient(145deg, #2f383c 0%, var(--tile) 55%, #242b2f 100%);
  box-shadow: 0 6px 14px rgb(0 0 0 / 0.35);
}

.hole {
  aspect-ratio: 1;
  border-radius: 50%;
  background: var(--hole);
  box-shadow:
    inset 0 3px 6px rgb(0 0 0 / 0.65),
    0 1px 0 rgb(255 255 255 / 0.05);
}
</style>
