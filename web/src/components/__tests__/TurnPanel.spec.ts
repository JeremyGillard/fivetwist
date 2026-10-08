import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import TurnPanel from '../TurnPanel.vue'
import type { Snapshot } from '@/composables/useGame'
import { Color, Outcome, Phase } from '@/wasm/pentago.js'

const names = { [Color.White]: 'Ada', [Color.Black]: 'Grace' }

function mountPanel(snapshot: Partial<Snapshot>, error: string | null = null) {
  return mount(TurnPanel, {
    props: {
      snapshot: {
        cells: [],
        current: Color.White,
        phase: Phase.Placement,
        outcome: undefined,
        winning: [],
        ...snapshot,
      },
      names,
      canTwist: false,
      error,
    },
  })
}

describe('TurnPanel', () => {
  it('names the player to move', () => {
    expect(mountPanel({ current: Color.Black }).find('h2').text()).toBe('Grace’s turn')
  })

  it('names the winner and offers another game', async () => {
    const wrapper = mountPanel({
      phase: Phase.Over,
      outcome: Outcome.BlackWins,
      winning: [0, 1, 2, 3, 4],
    })

    expect(wrapper.find('h2').text()).toBe('Grace wins')
    expect(wrapper.text()).toContain('Five in a row.')
    expect(wrapper.find('.steps').exists()).toBe(false)

    await wrapper.find('.panel__again').trigger('click')
    expect(wrapper.emitted('playAgain')).toHaveLength(1)
  })

  it('explains a draw where both sides lined up five', () => {
    const wrapper = mountPanel({
      phase: Phase.Over,
      outcome: Outcome.Draw,
      winning: [0, 1, 2, 3, 4, 6, 7, 8, 9, 10],
    })

    expect(wrapper.find('h2').text()).toBe('Draw')
    expect(wrapper.text()).toContain('The last twist lined up five for both sides.')
  })

  it('explains a draw on a full board', () => {
    const wrapper = mountPanel({ phase: Phase.Over, outcome: Outcome.Draw })

    expect(wrapper.text()).toContain('The board is full and nobody lined up five.')
  })

  it('shows why an action was refused', () => {
    const wrapper = mountPanel({}, 'this cell is already occupied')

    expect(wrapper.find('[role="alert"]').text()).toBe('this cell is already occupied')
  })
})
