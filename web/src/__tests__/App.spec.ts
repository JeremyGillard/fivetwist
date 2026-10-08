import { mount, type VueWrapper } from '@vue/test-utils'
import { describe, expect, it, vi } from 'vitest'

import App from '../App.vue'

// jsdom has no Web Animations API, so a twist is applied as soon as it is asked for.

function hole(wrapper: VueWrapper, col: number, row: number) {
  return wrapper.find(`.hole[aria-label^="Row ${row + 1}, column ${col + 1}:"]`)
}

function button(wrapper: VueWrapper, label: string) {
  const found = wrapper.findAll('button').find((candidate) => candidate.text() === label)
  if (!found) throw new Error(`no button labelled "${label}"`)
  return found
}

const twistClockwise = '[aria-label="Twist clockwise"]'
const twistCounterClockwise = '[aria-label="Twist counter-clockwise"]'
const soundsPlayed = () => vi.mocked(HTMLMediaElement.prototype.play).mock.calls.length

/** Places at (`col`, `row`), then twists the bottom-right quadrant, which the scripts leave empty. */
async function playTurn(wrapper: VueWrapper, col: number, row: number) {
  await hole(wrapper, col, row).trigger('click')
  await wrapper.find('[aria-label="Select the bottom-right quadrant"]').trigger('click')
  await wrapper.find(twistClockwise).trigger('click')
}

/** Bone takes the top row while Amber plays the bottom-left corner. */
async function playUntilBoneWins(wrapper: VueWrapper) {
  const amber: [number, number][] = [
    [0, 5],
    [1, 5],
    [2, 5],
    [0, 4],
  ]
  for (const [col, reply] of amber.entries()) {
    await playTurn(wrapper, col, 0)
    await playTurn(wrapper, reply[0], reply[1])
  }
  await hole(wrapper, 4, 0).trigger('click')
}

describe('App', () => {
  it('shows a new game: empty board, Bone to move, first step current', () => {
    const wrapper = mount(App)

    expect(wrapper.find('h1').text()).toBe('Fivetwist')
    expect(wrapper.findAll('.hole')).toHaveLength(36)
    expect(wrapper.findAll('.hole .marble')).toHaveLength(0)

    const cards = wrapper.findAll('.card')
    expect(cards.map((card) => card.find('h2').text())).toEqual(['Bone', 'Amber'])
    expect(cards[0]!.text()).toContain('0 marbles placed')
    expect(cards[0]!.text()).toContain('To move')
    expect(cards[1]!.text()).not.toContain('To move')

    expect(wrapper.find('.panel h2').text()).toBe('Bone’s turn')
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
    expect(wrapper.find('.panel h2').text()).toBe('Bone’s turn')
    // No hole can be played until the quadrant has been twisted.
    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(0)
  })

  it('preselects the quadrant that was just played', async () => {
    const wrapper = mount(App)

    await hole(wrapper, 4, 1).trigger('click')

    const pressed = wrapper.findAll('.quadrant__select[aria-pressed="true"]')
    expect(pressed.map((selector) => selector.attributes('aria-label'))).toEqual([
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
    expect(wrapper.find('.panel h2').text()).toBe('Amber’s turn')
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
    expect(wrapper.find('.panel h2').text()).toBe('Amber’s turn')
  })

  it('does not let an occupied hole be played', async () => {
    const wrapper = mount(App)
    await playTurn(wrapper, 1, 1)

    expect(hole(wrapper, 1, 1).attributes('disabled')).toBeDefined()
    await hole(wrapper, 1, 1).trigger('click')

    expect(hole(wrapper, 1, 1).find('.marble--bone').exists()).toBe(true)
    expect(wrapper.find('.panel h2').text()).toBe('Amber’s turn')
    expect(wrapper.find('[role="alert"]').exists()).toBe(false)
  })

  it('alternates the two sides', async () => {
    const wrapper = mount(App)

    await playTurn(wrapper, 0, 0)
    await playTurn(wrapper, 1, 0)

    expect(hole(wrapper, 0, 0).find('.marble--bone').exists()).toBe(true)
    expect(hole(wrapper, 1, 0).find('.marble--amber').exists()).toBe(true)
    expect(wrapper.find('.panel h2').text()).toBe('Bone’s turn')
  })

  it('ends the game on five in a row: winner, winning line, frozen board', async () => {
    const wrapper = mount(App)

    await playUntilBoneWins(wrapper)

    expect(wrapper.find('.panel h2').text()).toBe('Bone wins')
    expect(wrapper.find('.panel').text()).toContain('Five in a row.')
    expect(wrapper.findAll('.marble--winning')).toHaveLength(5)
    expect(wrapper.findAll('.marble--faded')).toHaveLength(4)
    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(0)
    expect(wrapper.find('.quadrant__select').exists()).toBe(false)
    expect(wrapper.find(twistClockwise).exists()).toBe(false)
    expect(wrapper.text()).not.toContain('To move')
  })

  it('starts another game from the end screen without asking', async () => {
    const wrapper = mount(App)
    await playUntilBoneWins(wrapper)

    await button(wrapper, 'Play again').trigger('click')

    expect(wrapper.findAll('.hole .marble')).toHaveLength(0)
    expect(wrapper.find('.panel h2').text()).toBe('Bone’s turn')
    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(36)
  })

  it('starts a new game at once when there is nothing to lose', async () => {
    const wrapper = mount(App)

    await button(wrapper, 'New game').trigger('click')

    expect(wrapper.findAll('dialog[open]')).toHaveLength(0)
    expect(wrapper.find('.panel h2').text()).toBe('Bone’s turn')
  })

  it('asks before discarding a game in progress', async () => {
    const wrapper = mount(App)
    await playTurn(wrapper, 2, 2)
    await hole(wrapper, 3, 3).trigger('click')

    await button(wrapper, 'New game').trigger('click')

    const dialog = wrapper.find('dialog[open]')
    expect(dialog.text()).toContain('Start a new game?')
    expect(wrapper.findAll('.hole .marble')).toHaveLength(2)

    await button(wrapper, 'Keep playing').trigger('click')

    expect(wrapper.findAll('dialog[open]')).toHaveLength(0)
    expect(wrapper.findAll('.hole .marble')).toHaveLength(2)
    expect(wrapper.find('[aria-current="step"]').text()).toContain('Twist a quadrant')
  })

  it('starts over once the new game is confirmed', async () => {
    const wrapper = mount(App)
    await playTurn(wrapper, 2, 2)
    await hole(wrapper, 3, 3).trigger('click')

    await button(wrapper, 'New game').trigger('click')
    await wrapper.find('dialog[open] .dialog__button--primary').trigger('click')

    expect(wrapper.findAll('dialog[open]')).toHaveLength(0)
    expect(wrapper.findAll('.hole .marble')).toHaveLength(0)
    expect(wrapper.find('.panel h2').text()).toBe('Bone’s turn')
    expect(wrapper.find('[aria-current="step"]').text()).toContain('Place a marble')
    expect(wrapper.findAll('.quadrant--selected')).toHaveLength(0)
    expect(wrapper.findAll('.hole:not([disabled])')).toHaveLength(36)
  })

  it('opens and closes the rules', async () => {
    const wrapper = mount(App)
    expect(wrapper.findAll('dialog[open]')).toHaveLength(0)

    await button(wrapper, 'Rules').trigger('click')

    const dialog = wrapper.find('dialog[open]')
    expect(dialog.text()).toContain('How to play')
    expect(dialog.text()).toContain('Twist a quadrant')

    await button(wrapper, 'Got it').trigger('click')
    expect(wrapper.findAll('dialog[open]')).toHaveLength(0)
  })

  it('uses the names of the players, and remembers them', async () => {
    const wrapper = mount(App)

    await wrapper.findAll('.card__rename')[0]!.trigger('click')
    await wrapper.find('.card input').setValue('Ada')
    await wrapper.find('.card input').trigger('keydown', { key: 'Enter' })

    expect(wrapper.findAll('.card h2')[0]!.text()).toBe('Ada')
    expect(wrapper.find('.panel h2').text()).toBe('Ada’s turn')
    // Marbles keep the name of their color.
    await hole(wrapper, 0, 0).trigger('click')
    expect(hole(wrapper, 0, 0).attributes('aria-label')).toBe('Row 1, column 1: Bone')

    expect(mount(App).findAll('.card h2')[0]!.text()).toBe('Ada')
  })

  it('plays a sound for a placement, a twist and the end of the game', async () => {
    const wrapper = mount(App)

    await hole(wrapper, 0, 0).trigger('click')
    expect(soundsPlayed()).toBe(1)

    await wrapper.find('[aria-label="Select the bottom-right quadrant"]').trigger('click')
    expect(soundsPlayed()).toBe(1)
    await wrapper.find(twistClockwise).trigger('click')
    expect(soundsPlayed()).toBe(2)

    const fresh = mount(App)
    vi.mocked(HTMLMediaElement.prototype.play).mockClear()
    await playUntilBoneWins(fresh)
    // Nine placements, eight twists, and the end of the game.
    expect(soundsPlayed()).toBe(18)
  })

  it('plays nothing once the sound is turned off, and remembers it', async () => {
    const wrapper = mount(App)
    const sound = wrapper.find('[aria-label="Sound"]')
    expect(sound.attributes('aria-pressed')).toBe('true')

    await sound.trigger('click')
    await playTurn(wrapper, 0, 0)

    expect(sound.attributes('aria-pressed')).toBe('false')
    expect(soundsPlayed()).toBe(0)
    expect(mount(App).find('[aria-label="Sound"]').attributes('aria-pressed')).toBe('false')
  })
})
