import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import App from '../App.vue'

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
  })
})
