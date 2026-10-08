<script setup lang="ts">
import GameBoard from './components/GameBoard.vue'
import PlayerCard from './components/PlayerCard.vue'
import TurnPanel from './components/TurnPanel.vue'
import { useGame } from './composables/useGame'
import { SIDE_LIST } from './game/sides'
import { Phase } from './wasm/pentago.js'

const { snapshot, placed, reset } = useGame()
</script>

<template>
  <div class="page">
    <header class="header">
      <h1 class="logo">Pentago</h1>
      <button class="pill" type="button" @click="reset">New game</button>
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

      <GameBoard class="table__board" :cells="snapshot.cells" />

      <TurnPanel class="table__turn" :snapshot="snapshot" />
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
