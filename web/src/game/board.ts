import { Color, Quadrant, type Direction } from '@/wasm/pentago.js'

/** A cell as `Game.cells()` encodes it: 0 when empty, otherwise the color of its marble. */
export type Cell = 0 | Color

export const EMPTY = 0
export const BOARD_SIZE = 6
export const QUADRANT_SIZE = BOARD_SIZE / 2

/** The four quadrants, in the order they are laid out on screen. */
export const QUADRANTS = [
  Quadrant.TopLeft,
  Quadrant.TopRight,
  Quadrant.BottomLeft,
  Quadrant.BottomRight,
] as const

export interface Position {
  col: number
  row: number
}

/** Index of a position in the list returned by `Game.cells()`. */
export function cellIndex({ col, row }: Position): number {
  return row * BOARD_SIZE + col
}

/** The nine positions of a quadrant, row by row. */
export function quadrantPositions(quadrant: Quadrant): Position[] {
  const originCol =
    quadrant === Quadrant.TopRight || quadrant === Quadrant.BottomRight ? QUADRANT_SIZE : 0
  const originRow =
    quadrant === Quadrant.BottomLeft || quadrant === Quadrant.BottomRight ? QUADRANT_SIZE : 0

  const positions: Position[] = []
  for (let row = 0; row < QUADRANT_SIZE; row++) {
    for (let col = 0; col < QUADRANT_SIZE; col++) {
      positions.push({ col: originCol + col, row: originRow + row })
    }
  }
  return positions
}

/** The quadrant a position belongs to. */
export function quadrantOf({ col, row }: Position): Quadrant {
  if (row < QUADRANT_SIZE) return col < QUADRANT_SIZE ? Quadrant.TopLeft : Quadrant.TopRight
  return col < QUADRANT_SIZE ? Quadrant.BottomLeft : Quadrant.BottomRight
}

/** How a quadrant is named to the player. */
export const QUADRANT_NAMES: Record<Quadrant, string> = {
  [Quadrant.TopLeft]: 'top-left',
  [Quadrant.TopRight]: 'top-right',
  [Quadrant.BottomLeft]: 'bottom-left',
  [Quadrant.BottomRight]: 'bottom-right',
}

/** A quadrant being turned: the rules are applied once the animation has finished. */
export interface Twist {
  quadrant: Quadrant
  direction: Direction
}
