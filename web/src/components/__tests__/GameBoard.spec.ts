import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import type { QuadrantMode } from '../BoardQuadrant.vue'
import GameBoard from '../GameBoard.vue'
import type { Cell } from '@/game/board'
import { SIDES } from '@/game/sides'
import { Color, Quadrant } from '@/wasm/pentago.js'

function boardWith(marbles: Record<number, Color>): Cell[] {
  return Array.from({ length: 36 }, (_, index) => marbles[index] ?? 0)
}

function mountBoard(marbles: Record<number, Color>, mode: QuadrantMode = 'idle') {
  return mount(GameBoard, {
    props: {
      cells: boardWith(marbles),
      mode,
      current: SIDES[Color.White],
      selected: null,
      twist: null,
    },
  })
}

describe('GameBoard', () => {
  it('draws four quadrants of nine holes', () => {
    const wrapper = mountBoard({})

    expect(wrapper.findAll('.quadrant')).toHaveLength(4)
    expect(wrapper.findAll('.hole')).toHaveLength(36)
    expect(wrapper.findAll('.marble')).toHaveLength(0)
  })

  it('draws each marble in the color of its side', () => {
    const wrapper = mountBoard({ 0: Color.White, 7: Color.Black, 35: Color.Black })

    expect(wrapper.findAll('.marble--bone')).toHaveLength(1)
    expect(wrapper.findAll('.marble--amber')).toHaveLength(2)
  })

  it('puts each cell in the quadrant and hole it belongs to', () => {
    // Column 4, row 1 is index 1 * 6 + 4: top-right quadrant, second row, second hole.
    const wrapper = mountBoard({ 10: Color.Black })

    const quadrants = wrapper.findAll('.quadrant')
    const hole = quadrants[1]!.findAll('.hole')[4]!
    expect(hole.find('.marble--amber').exists()).toBe(true)
    expect(hole.attributes('aria-label')).toBe('Row 2, column 5: Amber')
    expect(wrapper.findAll('.marble')).toHaveLength(1)
  })

  it('lets only empty holes be played while placing', async () => {
    const wrapper = mountBoard({ 0: Color.White }, 'place')
    const holes = wrapper.findAll('.hole')

    expect(holes[0]!.attributes('disabled')).toBeDefined()
    expect(holes[1]!.attributes('disabled')).toBeUndefined()

    await holes[10]!.trigger('click')
    expect(wrapper.emitted('place')).toEqual([[{ col: 4, row: 0 }]])
    expect(wrapper.find('.quadrant__select').exists()).toBe(false)
  })

  it('lets a quadrant be selected, and no hole be played, while rotating', async () => {
    const wrapper = mountBoard({}, 'select')

    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(0)
    const selectors = wrapper.findAll('.quadrant__select')
    expect(selectors.map((selector) => selector.attributes('aria-label'))).toEqual([
      'Select the top-left quadrant',
      'Select the top-right quadrant',
      'Select the bottom-left quadrant',
      'Select the bottom-right quadrant',
    ])

    await selectors[2]!.trigger('click')
    expect(wrapper.emitted('select')).toEqual([[Quadrant.BottomLeft]])
  })

  it('is inert when idle', () => {
    const wrapper = mountBoard({}, 'idle')

    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(0)
    expect(wrapper.find('.quadrant__select').exists()).toBe(false)
  })

  it('marks the selected quadrant', () => {
    const wrapper = mountBoard({}, 'select')
    expect(wrapper.findAll('.quadrant--selected')).toHaveLength(0)

    return wrapper.setProps({ selected: Quadrant.TopRight }).then(() => {
      const quadrants = wrapper.findAll('.quadrant')
      expect(quadrants[1]!.classes()).toContain('quadrant--selected')
      expect(wrapper.findAll('.quadrant--selected')).toHaveLength(1)
      expect(wrapper.findAll('[aria-pressed="true"]')).toHaveLength(1)
    })
  })
})
