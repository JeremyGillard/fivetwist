import { describe, expect, it } from 'vitest'

import { useGame } from '../useGame'
import { Color, Direction, Outcome, Phase, Quadrant } from '@/wasm/pentago.js'

/** A full turn that never disturbs the marbles: the bottom-right quadrant stays empty. */
function playTurn(game: ReturnType<typeof useGame>, col: number, row: number) {
  game.place(col, row)
  game.rotate(Quadrant.BottomRight, Direction.Clockwise)
}

describe('useGame', () => {
  it('starts with an empty board and Bone (White) to place', () => {
    const { snapshot, placed, error } = useGame()

    expect(snapshot.value.cells).toHaveLength(36)
    expect(snapshot.value.cells.every((cell) => cell === 0)).toBe(true)
    expect(snapshot.value.current).toBe(Color.White)
    expect(snapshot.value.phase).toBe(Phase.Placement)
    expect(snapshot.value.outcome).toBeUndefined()
    expect(snapshot.value.winning).toEqual([])
    expect(placed.value).toEqual({ [Color.White]: 0, [Color.Black]: 0 })
    expect(error.value).toBeNull()
  })

  it('refreshes the snapshot after a placement', () => {
    const { snapshot, placed, place } = useGame()
    const before = snapshot.value

    place(1, 2)

    expect(snapshot.value).not.toBe(before)
    expect(snapshot.value.cells[2 * 6 + 1]).toBe(Color.White)
    expect(snapshot.value.phase).toBe(Phase.Rotation)
    expect(placed.value[Color.White]).toBe(1)
  })

  it('passes the turn after a rotation', () => {
    const { snapshot, place, rotate } = useGame()

    place(0, 0)
    rotate(Quadrant.TopLeft, Direction.Clockwise)

    expect(snapshot.value.cells[2]).toBe(Color.White)
    expect(snapshot.value.current).toBe(Color.Black)
    expect(snapshot.value.phase).toBe(Phase.Placement)
  })

  it('reports why an action was refused and keeps the game playable', () => {
    const game = useGame()
    playTurn(game, 1, 1)

    game.place(1, 1)
    expect(game.error.value).toBe('this cell is already occupied')
    expect(game.snapshot.value.phase).toBe(Phase.Placement)

    game.place(0, 0)
    expect(game.error.value).toBeNull()
    expect(game.snapshot.value.cells[0]).toBe(Color.Black)
  })

  it('reports a position off the board', () => {
    const { error, place } = useGame()
    place(6, 0)
    expect(error.value).toBe('this position is off the board')
  })

  it('reports the outcome of a finished game', () => {
    const game = useGame()
    const white = [0, 1, 2, 3]
    const black: [number, number][] = [
      [0, 5],
      [1, 5],
      [2, 5],
      [0, 4],
    ]
    white.forEach((col, turn) => {
      playTurn(game, col, 0)
      playTurn(game, ...black[turn]!)
    })
    game.place(4, 0)

    expect(game.snapshot.value.phase).toBe(Phase.Over)
    expect(game.snapshot.value.outcome).toBe(Outcome.WhiteWins)
    expect(game.snapshot.value.winning).toEqual([0, 1, 2, 3, 4])
    expect(game.placed.value).toEqual({ [Color.White]: 5, [Color.Black]: 4 })
  })

  it('starts over on reset', () => {
    const game = useGame()
    playTurn(game, 1, 1)
    game.place(1, 1)

    game.reset()

    expect(game.snapshot.value.cells.every((cell) => cell === 0)).toBe(true)
    expect(game.snapshot.value.current).toBe(Color.White)
    expect(game.error.value).toBeNull()
  })
})
