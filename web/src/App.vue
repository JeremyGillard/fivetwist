<script setup lang="ts">
import { computed, ref, watch } from 'vue'

import type { QuadrantMode } from './components/BoardQuadrant.vue'
import ConfirmDialog from './components/ConfirmDialog.vue'
import GameBoard from './components/GameBoard.vue'
import PlayerCard from './components/PlayerCard.vue'
import RulesDialog from './components/RulesDialog.vue'
import TurnPanel from './components/TurnPanel.vue'
import { useGame } from './composables/useGame'
import { usePlayers } from './composables/usePlayers'
import { useSound } from './composables/useSound'
import { quadrantOf, type Position, type Twist } from './game/board'
import { SIDE_LIST, SIDES } from './game/sides'
import { Color, Phase, type Direction, type Quadrant } from './wasm/pentago.js'

const { snapshot, error, placed, place, rotate, reset } = useGame()
const { names, rename } = usePlayers()
const { muted, play, toggleMuted } = useSound()

/** The quadrant the twist buttons act on. */
const selected = ref<Quadrant | null>(null)
/** The rotation being animated; the rules only hear about it once it has finished. */
const twist = ref<Twist | null>(null)

const rulesOpen = ref(false)
const confirmingNewGame = ref(false)

const over = computed(() => snapshot.value.phase === Phase.Over)

const mode = computed<QuadrantMode>(() => {
  if (twist.value !== null) return 'idle'
  if (snapshot.value.phase === Phase.Placement) return 'place'
  if (snapshot.value.phase === Phase.Rotation) return 'select'
  return 'idle'
})

const canTwist = computed(() => mode.value === 'select' && selected.value !== null)

/** A game with marbles on the board that nobody has finished yet. */
const inProgress = computed(
  () => !over.value && placed.value[Color.White] + placed.value[Color.Black] > 0,
)

watch(over, (isOver) => {
  if (isOver) play('end')
})

function onPlace(position: Position) {
  if (mode.value !== 'place') return
  place(position.col, position.row)
  if (error.value !== null) return

  play('place')
  // The quadrant just played is the likeliest one to twist.
  if (snapshot.value.phase === Phase.Rotation) selected.value = quadrantOf(position)
}

function onSelect(quadrant: Quadrant) {
  if (mode.value === 'select') selected.value = quadrant
}

function onTwist(direction: Direction) {
  if (!canTwist.value || selected.value === null) return
  twist.value = { quadrant: selected.value, direction }
  play('twist')
}

function onTwisted() {
  if (twist.value === null) return
  rotate(twist.value.quadrant, twist.value.direction)
  twist.value = null
  selected.value = null
}

function startNewGame() {
  confirmingNewGame.value = false
  twist.value = null
  selected.value = null
  reset()
}

/**
 * Safari does not focus a button when it is clicked. A dialog gives focus back, on closing, to
 * whatever had it before: focusing the button first makes that the button in every browser.
 */
function focusTrigger(event: MouseEvent) {
  if (event.currentTarget instanceof HTMLElement) event.currentTarget.focus()
}

function openRules(event: MouseEvent) {
  focusTrigger(event)
  rulesOpen.value = true
}

/** Asks first when there is a game to lose. */
function requestNewGame(event: MouseEvent) {
  if (!inProgress.value) return startNewGame()
  focusTrigger(event)
  confirmingNewGame.value = true
}
</script>

<template>
  <div class="page">
    <header class="header">
      <h1 class="logo">Fivetwist</h1>
      <nav class="header__actions" aria-label="Game">
        <button class="pill" type="button" @click="openRules">Rules</button>
        <button
          class="pill pill--icon"
          type="button"
          aria-label="Sound"
          :aria-pressed="!muted"
          :title="muted ? 'Turn the sound on' : 'Turn the sound off'"
          @click="toggleMuted"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M11 5 6 9H3v6h3l5 4z" />
            <template v-if="muted">
              <path d="m16 9 5 6" />
              <path d="m21 9-5 6" />
            </template>
            <template v-else>
              <path d="M15.5 8.5a5 5 0 0 1 0 7" />
              <path d="M18.5 5.5a9 9 0 0 1 0 13" />
            </template>
          </svg>
        </button>
        <button class="pill" type="button" @click="requestNewGame">New game</button>
      </nav>
    </header>

    <main class="table">
      <aside class="players" aria-label="Players">
        <PlayerCard
          v-for="side in SIDE_LIST"
          :key="side.color"
          :side="side"
          :name="names[side.color]"
          :placed="placed[side.color]"
          :to-move="!over && snapshot.current === side.color"
          @rename="rename(side.color, $event)"
        />
      </aside>

      <GameBoard
        class="table__board"
        :cells="snapshot.cells"
        :winning="snapshot.winning"
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
        :names="names"
        :can-twist="canTwist"
        :error="error"
        @twist="onTwist"
        @play-again="startNewGame"
      />
    </main>

    <footer class="hint">
      <p>Five in a row wins — across, down or diagonal.</p>
      <p class="hint__credit">An unofficial game based on the board game Pentago.</p>
    </footer>

    <RulesDialog :open="rulesOpen" @close="rulesOpen = false" />
    <ConfirmDialog
      :open="confirmingNewGame"
      title="Start a new game?"
      message="The game in progress will be lost."
      confirm-label="New game"
      cancel-label="Keep playing"
      @confirm="startNewGame"
      @cancel="confirmingNewGame = false"
    />
  </div>
</template>

<style scoped>
.page {
  display: grid;
  grid-template-rows: auto 1fr auto;
  min-height: 100dvh;
  padding: 2.25rem 3rem;
}

.header {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem 1rem;
}

.header__actions {
  display: flex;
  align-items: center;
  gap: 0.6rem;
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
  white-space: nowrap;
  cursor: pointer;
  transition:
    border-color 0.15s,
    background-color 0.15s;
}

.pill--icon {
  display: grid;
  place-items: center;
  width: 2.6rem;
  aspect-ratio: 1;
  padding: 0;
}

.pill--icon svg {
  width: 1.15rem;
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
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
  grid-template-columns: minmax(13rem, 19rem) minmax(0, 36rem) minmax(13rem, 19rem);
  grid-template-areas: 'players board turn';
  align-items: center;
  justify-content: space-between;
  gap: 3rem;
  padding: 2rem 0;
}

.players {
  grid-area: players;
  display: grid;
  gap: 1rem;
}

.table__board {
  grid-area: board;
  justify-self: center;
}

.table__turn {
  grid-area: turn;
  justify-self: start;
  padding-left: 2rem;
}

.hint {
  text-align: center;
  font-size: 0.9rem;
  color: var(--muted);
}

.hint p {
  margin: 0;
}

.hint__credit {
  margin-top: 0.35rem;
  font-size: 0.75rem;
  opacity: 0.7;
}

/* Tablets and phones: one column, in the order the eye and the thumb need. */
@media (max-width: 68rem) {
  .page {
    padding: 1.25rem 1rem;
  }

  .table {
    grid-template-columns: minmax(0, 36rem);
    grid-template-areas:
      'players'
      'board'
      'turn';
    justify-content: center;
    gap: 1.25rem;
    padding: 1.25rem 0;
  }

  .players {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
  }

  .table__turn {
    justify-self: stretch;
    padding-left: 0;
  }
}

@media (max-width: 30rem) {
  .logo {
    font-size: 1.05rem;
    letter-spacing: 0.22em;
  }

  .pill {
    padding: 0.5rem 0.85rem;
    font-size: 0.8rem;
  }

  .pill--icon {
    width: 2.25rem;
    padding: 0;
  }

  .hint {
    font-size: 0.8rem;
  }
}
</style>
