<script setup lang="ts">
import { computed, ref } from 'vue'

import type { QuadrantMode } from './components/BoardQuadrant.vue'
import GameBoard from './components/GameBoard.vue'
import PlayerCard from './components/PlayerCard.vue'
import TurnPanel from './components/TurnPanel.vue'
import { useGame } from './composables/useGame'
import { quadrantOf, type Position, type Twist } from './game/board'
import { SIDE_LIST, SIDES } from './game/sides'
import { Phase, type Direction, type Quadrant } from './wasm/pentago.js'

const { snapshot, error, placed, place, rotate, reset } = useGame()

/** The quadrant the twist buttons act on. */
const selected = ref<Quadrant | null>(null)
/** The rotation being animated; the rules only hear about it once it has finished. */
const twist = ref<Twist | null>(null)

const mode = computed<QuadrantMode>(() => {
  if (twist.value !== null) return 'idle'
  if (snapshot.value.phase === Phase.Placement) return 'place'
  if (snapshot.value.phase === Phase.Rotation) return 'select'
  return 'idle'
})

const canTwist = computed(() => mode.value === 'select' && selected.value !== null)

function onPlace(position: Position) {
  if (mode.value !== 'place') return
  place(position.col, position.row)
  // The quadrant just played is the likeliest one to twist.
  if (snapshot.value.phase === Phase.Rotation) selected.value = quadrantOf(position)
}

function onSelect(quadrant: Quadrant) {
  if (mode.value === 'select') selected.value = quadrant
}

function onTwist(direction: Direction) {
  if (!canTwist.value || selected.value === null) return
  twist.value = { quadrant: selected.value, direction }
}

function onTwisted() {
  if (twist.value === null) return
  rotate(twist.value.quadrant, twist.value.direction)
  twist.value = null
  selected.value = null
}

function newGame() {
  twist.value = null
  selected.value = null
  reset()
}
</script>

<template>
  <div class="page">
    <header class="header">
      <h1 class="logo">Pentago</h1>
      <button class="pill" type="button" @click="newGame">New game</button>
    </header>

    <main class="table">
      <aside class="players" aria-label="Players">
        <PlayerCard
          v-for="side in SIDE_LIST"
          :key="side.color"
          :side="side"
          :placed="placed[side.color]"
          :to-move="snapshot.phase !== Phase.Over && snapshot.current === side.color"
        />
      </aside>

      <GameBoard
        class="table__board"
        :cells="snapshot.cells"
        :mode="mode"
        :current="SIDES[snapshot.current]"
        :selected="selected"
        :twist="twist"
        @place="onPlace"
        @select="onSelect"
        @twisted="onTwisted"
      />

      <TurnPanel
        class="table__turn"
        :snapshot="snapshot"
        :can-twist="canTwist"
        :error="error"
        @twist="onTwist"
      />
    </main>

    <footer class="hint">Five in a row wins — across, down or diagonal.</footer>
  </div>
</template>

<style scoped>
.page {
  display: grid;
  grid-template-rows: auto 1fr auto;
  min-height: 100vh;
  padding: 2.25rem 3rem;
}

.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
}

.logo {
  margin: 0;
  font-size: 1.35rem;
  font-weight: 800;
  font-stretch: 125%;
  letter-spacing: 0.32em;
  text-transform: uppercase;
}

.pill {
  padding: 0.6rem 1.25rem;
  border: 1.5px solid var(--line);
  border-radius: 999px;
  background: transparent;
  font-size: 0.9rem;
  font-weight: 600;
  cursor: pointer;
  transition:
    border-color 0.15s,
    background-color 0.15s;
}

.pill:hover {
  border-color: var(--muted);
  background: var(--surface);
}

.pill:focus-visible {
  outline: 2px solid var(--amber);
  outline-offset: 2px;
}

.table {
  display: grid;
  grid-template-columns: minmax(15rem, 19rem) minmax(0, 36rem) minmax(15rem, 19rem);
  align-items: center;
  justify-content: space-between;
  gap: 3rem;
  padding: 2rem 0;
}

.players {
  display: grid;
  gap: 1rem;
}

.table__board {
  justify-self: center;
}

.table__turn {
  justify-self: start;
  padding-left: 2rem;
}

.hint {
  text-align: center;
  font-size: 0.9rem;
  color: var(--muted);
}
</style>
