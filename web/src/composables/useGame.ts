import { computed, getCurrentScope, onScopeDispose, readonly, ref, shallowRef } from 'vue'

import { EMPTY, type Cell } from '@/game/board'
import { Color, Outcome, Phase, Game, type Direction, type Quadrant } from '@/wasm/pentago.js'

/** A copy of the game state, taken after every action. */
export interface Snapshot {
  /** The 36 cells, row by row from the top-left corner. */
  cells: readonly Cell[]
  current: Color
  phase: Phase
  outcome: Outcome | undefined
  /** Indexes in `cells` of the marbles forming a winning line; empty unless the game was won. */
  winning: readonly number[]
}

function read(game: Game): Snapshot {
  return {
    cells: Array.from(game.cells()) as Cell[],
    current: game.current(),
    phase: game.phase(),
    outcome: game.outcome(),
    winning: Array.from(game.winningCells()),
  }
}

/**
 * One game of Pentago.
 *
 * The game itself lives in WebAssembly memory, which Vue cannot observe. So every action goes
 * through the rules, then a fresh snapshot replaces the previous one; the page is drawn from it.
 */
export function useGame() {
  let game = new Game()
  const snapshot = shallowRef(read(game))
  /** Why the last action was refused, if it was. */
  const error = ref<string | null>(null)

  function attempt(action: () => void) {
    try {
      action()
      error.value = null
    } catch (refusal) {
      error.value = refusal instanceof Error ? refusal.message : String(refusal)
    }
    snapshot.value = read(game)
  }

  function place(col: number, row: number) {
    attempt(() => game.place(col, row))
  }

  function rotate(quadrant: Quadrant, direction: Direction) {
    attempt(() => game.rotate(quadrant, direction))
  }

  function reset() {
    // The game is Rust memory: the garbage collector does not reclaim it promptly on its own.
    game.free()
    game = new Game()
    error.value = null
    snapshot.value = read(game)
  }

  /** How many marbles each side has on the board. */
  const placed = computed<Record<Color, number>>(() => {
    const counts = { [Color.White]: 0, [Color.Black]: 0 }
    for (const cell of snapshot.value.cells) {
      if (cell !== EMPTY) counts[cell]++
    }
    return counts
  })

  if (getCurrentScope()) {
    onScopeDispose(() => game.free())
  }

  return { snapshot: readonly(snapshot), error: readonly(error), placed, place, rotate, reset }
}
