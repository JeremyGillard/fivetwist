import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import GameBoard from '../GameBoard.vue'
import type { Cell } from '@/game/board'
import { Color } from '@/wasm/pentago.js'

function boardWith(marbles: Record<number, Color>): Cell[] {
  return Array.from({ length: 36 }, (_, index) => marbles[index] ?? 0)
}

describe('GameBoard', () => {
  it('draws four quadrants of nine holes', () => {
    const wrapper = mount(GameBoard, { props: { cells: boardWith({}) } })

    expect(wrapper.findAll('.quadrant')).toHaveLength(4)
    expect(wrapper.findAll('.hole')).toHaveLength(36)
    expect(wrapper.findAll('.marble')).toHaveLength(0)
  })

  it('draws each marble in the color of its side', () => {
    const wrapper = mount(GameBoard, {
      props: { cells: boardWith({ 0: Color.White, 7: Color.Black, 35: Color.Black }) },
    })

    expect(wrapper.findAll('.marble--bone')).toHaveLength(1)
    expect(wrapper.findAll('.marble--amber')).toHaveLength(2)
  })

  it('puts each cell in the quadrant and hole it belongs to', () => {
    // Column 4, row 1 is index 1 * 6 + 4: top-right quadrant, second row, second hole.
    const wrapper = mount(GameBoard, { props: { cells: boardWith({ 10: Color.Black }) } })

    const quadrants = wrapper.findAll('.quadrant')
    const hole = quadrants[1]!.findAll('.hole')[4]!
    expect(hole.find('.marble--amber').exists()).toBe(true)
    expect(hole.attributes('aria-label')).toBe('Row 2, column 5: Amber')
    expect(wrapper.findAll('.marble')).toHaveLength(1)
  })
})
