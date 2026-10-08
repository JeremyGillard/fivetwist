import { mount, type VueWrapper } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import App from '../App.vue'

// jsdom has no Web Animations API, so a twist is applied as soon as it is asked for.

function hole(wrapper: VueWrapper, col: number, row: number) {
  return wrapper.find(`.hole[aria-label^="Row ${row + 1}, column ${col + 1}:"]`)
}

const twistClockwise = '[aria-label="Twist clockwise"]'
const twistCounterClockwise = '[aria-label="Twist counter-clockwise"]'

/** Places at (`col`, `row`), then twists the bottom-right quadrant, which the scripts leave empty. */
async function playTurn(wrapper: VueWrapper, col: number, row: number) {
  await hole(wrapper, col, row).trigger('click')
  await wrapper.find('[aria-label="Select the bottom-right quadrant"]').trigger('click')
  await wrapper.find(twistClockwise).trigger('click')
}

describe('App', () => {
  it('shows a new game: empty board, Bone to move, first step current', () => {
    const wrapper = mount(App)

    expect(wrapper.find('h1').text()).toBe('Pentago')
    expect(wrapper.findAll('.hole')).toHaveLength(36)
    expect(wrapper.findAll('.hole .marble')).toHaveLength(0)

    const cards = wrapper.findAll('.card')
    expect(cards.map((card) => card.find('h2').text())).toEqual(['Bone', 'Amber'])
    expect(cards[0]!.text()).toContain('0 marbles placed')
    expect(cards[0]!.text()).toContain('To move')
    expect(cards[1]!.text()).not.toContain('To move')

    expect(wrapper.find('.panel h2').text()).toBe("Bone's turn")
    expect(wrapper.find('[aria-current="step"]').text()).toContain('Place a marble')
    expect(wrapper.find(twistClockwise).attributes('disabled')).toBeDefined()
    expect(wrapper.find(twistCounterClockwise).attributes('disabled')).toBeDefined()
  })

  it('places a marble on a click and moves on to the twist', async () => {
    const wrapper = mount(App)

    await hole(wrapper, 4, 1).trigger('click')

    expect(hole(wrapper, 4, 1).find('.marble--bone').exists()).toBe(true)
    expect(wrapper.findAll('.card')[0]!.text()).toContain('1 marble placed')
    expect(wrapper.find('[aria-current="step"]').text()).toContain('Twist a quadrant')
    expect(wrapper.find('.panel h2').text()).toBe("Bone's turn")
    // No hole can be played until the quadrant has been twisted.
    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(0)
  })

  it('preselects the quadrant that was just played', async () => {
    const wrapper = mount(App)

    await hole(wrapper, 4, 1).trigger('click')

    const pressed = wrapper.findAll('[aria-pressed="true"]')
    expect(pressed.map((button) => button.attributes('aria-label'))).toEqual([
      'Select the top-right quadrant',
    ])
    expect(wrapper.find(twistClockwise).attributes('disabled')).toBeUndefined()
  })

  it('twists the selected quadrant and passes the turn', async () => {
    const wrapper = mount(App)

    await hole(wrapper, 0, 0).trigger('click')
    await wrapper.find(twistClockwise).trigger('click')

    // Clockwise, the top-left corner of the quadrant goes to its top-right corner.
    expect(hole(wrapper, 0, 0).find('.marble').exists()).toBe(false)
    expect(hole(wrapper, 2, 0).find('.marble--bone').exists()).toBe(true)
    expect(wrapper.find('.panel h2').text()).toBe("Amber's turn")
    expect(wrapper.findAll('.card')[1]!.text()).toContain('To move')
    expect(wrapper.find('[aria-current="step"]').text()).toContain('Place a marble')
    expect(wrapper.findAll('.quadrant--selected')).toHaveLength(0)
  })

  it('twists counter-clockwise with the other button', async () => {
    const wrapper = mount(App)

    await hole(wrapper, 0, 0).trigger('click')
    await wrapper.find(twistCounterClockwise).trigger('click')

    expect(hole(wrapper, 0, 2).find('.marble--bone').exists()).toBe(true)
  })

  it('twists another quadrant when one is picked', async () => {
    const wrapper = mount(App)

    await hole(wrapper, 0, 0).trigger('click')
    await wrapper.find('[aria-label="Select the bottom-left quadrant"]').trigger('click')
    await wrapper.find(twistClockwise).trigger('click')

    expect(hole(wrapper, 0, 0).find('.marble--bone').exists()).toBe(true)
    expect(wrapper.find('.panel h2').text()).toBe("Amber's turn")
  })

  it('does not let an occupied hole be played', async () => {
    const wrapper = mount(App)
    await playTurn(wrapper, 1, 1)

    expect(hole(wrapper, 1, 1).attributes('disabled')).toBeDefined()
    await hole(wrapper, 1, 1).trigger('click')

    expect(hole(wrapper, 1, 1).find('.marble--bone').exists()).toBe(true)
    expect(wrapper.find('.panel h2').text()).toBe("Amber's turn")
    expect(wrapper.find('[role="alert"]').exists()).toBe(false)
  })

  it('alternates the two sides', async () => {
    const wrapper = mount(App)

    await playTurn(wrapper, 0, 0)
    await playTurn(wrapper, 1, 0)

    expect(hole(wrapper, 0, 0).find('.marble--bone').exists()).toBe(true)
    expect(hole(wrapper, 1, 0).find('.marble--amber').exists()).toBe(true)
    expect(wrapper.find('.panel h2').text()).toBe("Bone's turn")
  })

  it('ends the game on five in a row and freezes the board', async () => {
    const wrapper = mount(App)
    const bone = [0, 1, 2, 3]
    const amber: [number, number][] = [
      [0, 5],
      [1, 5],
      [2, 5],
      [0, 4],
    ]
    for (const [turn, col] of bone.entries()) {
      await playTurn(wrapper, col, 0)
      await playTurn(wrapper, ...amber[turn]!)
    }

    await hole(wrapper, 4, 0).trigger('click')

    expect(wrapper.find('.panel h2').text()).toBe('Bone wins')
    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(0)
    expect(wrapper.find('.quadrant__select').exists()).toBe(false)
    expect(wrapper.find(twistClockwise).exists()).toBe(false)
    expect(wrapper.text()).not.toContain('To move')
  })

  it('starts over with New game', async () => {
    const wrapper = mount(App)
    await playTurn(wrapper, 2, 2)
    await hole(wrapper, 3, 3).trigger('click')

    await wrapper.find('.pill').trigger('click')

    expect(wrapper.findAll('.hole .marble')).toHaveLength(0)
    expect(wrapper.find('.panel h2').text()).toBe("Bone's turn")
    expect(wrapper.find('[aria-current="step"]').text()).toContain('Place a marble')
    expect(wrapper.findAll('.quadrant--selected')).toHaveLength(0)
    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(36)
  })
})
