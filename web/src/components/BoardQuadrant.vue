<script setup lang="ts">
import { computed, onBeforeUnmount, useTemplateRef, watch } from 'vue'

import GameMarble from './GameMarble.vue'
import {
  cellIndex,
  EMPTY,
  QUADRANT_NAMES,
  quadrantPositions,
  type Cell,
  type Position,
} from '@/game/board'
import { SIDES, type Side } from '@/game/sides'
import { Direction, type Quadrant } from '@/wasm/pentago.js'

/** What a click on the quadrant does: place a marble in a hole, select the quadrant, or nothing. */
export type QuadrantMode = 'place' | 'select' | 'idle'

const TWIST_MS = 420
const STALL_MARGIN_MS = 250

const props = defineProps<{
  quadrant: Quadrant
  /** The whole board; the quadrant picks its own nine cells. */
  cells: readonly Cell[]
  mode: QuadrantMode
  /** The side to move, previewed in the hole under the pointer. */
  current: Side
  selected: boolean
  /** Set while this quadrant must turn; `twisted` is emitted when it has. */
  twisting: Direction | null
}>()

const emit = defineEmits<{
  place: [position: Position]
  select: []
  twisted: []
}>()

const holes = computed(() =>
  quadrantPositions(props.quadrant).map((position) => {
    const cell = props.cells[cellIndex(position)] ?? EMPTY
    const side = cell === EMPTY ? null : SIDES[cell]
    return {
      key: cellIndex(position),
      position,
      side,
      open: props.mode === 'place' && side === null,
      label: `Row ${position.row + 1}, column ${position.col + 1}: ${side?.name ?? 'empty'}`,
    }
  }),
)

const tile = useTemplateRef('tile')
let animation: Animation | null = null

/** Whether turning the tile on screen is possible and wanted right now. */
function canAnimate(element: HTMLElement | null): element is HTMLElement {
  if (!element || typeof element.animate !== 'function') return false
  // Animations do not advance in a hidden tab: the twist would wait until the tab is shown.
  if (document.hidden) return false
  return !(window.matchMedia?.('(prefers-reduced-motion: reduce)').matches ?? false)
}

function wait(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

// The board still shows the cells from before the rotation, so the tile turns with its marbles.
// `flush: 'post'`: when `twisting` goes back to null the new cells are already drawn, and
// dropping the animation then leaves no frame where the old cells sit unrotated.
watch(
  () => props.twisting,
  async (direction) => {
    animation?.cancel()
    animation = null
    if (direction === null) return

    const element = tile.value
    if (!canAnimate(element)) {
      emit('twisted')
      return
    }

    const turn = direction === Direction.Clockwise ? '90deg' : '-90deg'
    const running = element.animate([{ rotate: '0deg' }, { rotate: turn }], {
      duration: TWIST_MS,
      easing: 'cubic-bezier(0.4, 0, 0.2, 1)',
      fill: 'forwards',
    })
    animation = running
    try {
      // The timer is a safety net: the game must go on even if the animation stalls, as it
      // does when the tab is hidden while the tile is turning.
      await Promise.race([running.finished, wait(TWIST_MS + STALL_MARGIN_MS)])
    } catch {
      // Cancelled: a new game was started mid-turn.
      return
    }
    if (animation === running) emit('twisted')
  },
  { flush: 'post' },
)

onBeforeUnmount(() => animation?.cancel())
</script>

<template>
  <div
    ref="tile"
    class="quadrant"
    :class="{ 'quadrant--selected': selected, 'quadrant--twisting': twisting !== null }"
  >
    <button
      v-for="hole in holes"
      :key="hole.key"
      class="hole"
      :class="[`hole--${current.tone}`, { 'hole--open': hole.open }]"
      type="button"
      :disabled="!hole.open"
      :aria-label="hole.label"
      @click="emit('place', hole.position)"
    >
      <GameMarble v-if="hole.side" :side="hole.side" />
    </button>

    <button
      v-if="mode === 'select'"
      class="quadrant__select"
      type="button"
      :aria-pressed="selected"
      :aria-label="`Select the ${QUADRANT_NAMES[quadrant]} quadrant`"
      @click="emit('select')"
    />
  </div>
</template>

<style scoped>
.quadrant {
  position: relative;
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 9.5%;
  padding: 5%;
  aspect-ratio: 1;
  border-radius: 9%;
  border: 1px solid var(--tile-edge);
  background: linear-gradient(145deg, #2f383c 0%, var(--tile) 55%, #242b2f 100%);
  box-shadow: 0 6px 14px rgb(0 0 0 / 0.35);
  transition:
    box-shadow 0.2s,
    outline-color 0.2s;
  outline: 2px solid transparent;
  outline-offset: 2px;
}

.quadrant--selected {
  outline-color: var(--amber);
  box-shadow: 0 10px 26px rgb(0 0 0 / 0.55);
}

/* Its corners sweep over the neighbouring tiles while it turns. */
.quadrant--twisting {
  z-index: 1;
}

.hole {
  position: relative;
  display: block;
  padding: 0;
  border: 0;
  aspect-ratio: 1;
  border-radius: 50%;
  background: var(--hole);
  box-shadow:
    inset 0 3px 6px rgb(0 0 0 / 0.65),
    0 1px 0 rgb(255 255 255 / 0.05);
}

.hole--open {
  cursor: pointer;
}

/* A faint marble of the side to move, in the hole about to be played. */
.hole--open::after {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 50%;
  opacity: 0;
  transition: opacity 0.12s;
}

.hole--open.hole--bone::after {
  background: radial-gradient(circle at 36% 30%, #ffffff, #cfc8b8 72%, #a39c8d);
}

.hole--open.hole--amber::after {
  background: radial-gradient(circle at 36% 30%, #ffcf96, #dd7717 68%, #8f4409);
}

.hole--open:hover::after,
.hole--open:focus-visible::after {
  opacity: 0.4;
}

.hole:focus-visible {
  outline: 2px solid var(--amber);
  outline-offset: 2px;
}

.quadrant__select {
  position: absolute;
  inset: 0;
  padding: 0;
  border: 0;
  border-radius: inherit;
  background: transparent;
  cursor: pointer;
}

.quadrant__select:hover {
  background: rgb(242 150 58 / 0.06);
}

.quadrant__select:focus-visible {
  outline: 2px solid var(--amber);
  outline-offset: 5px;
}
</style>
