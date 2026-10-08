import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'

import PlayerCard from '../PlayerCard.vue'
import { SIDES } from '@/game/sides'
import { Color } from '@/wasm/pentago.js'

function mountCard(name = 'Bone') {
  return mount(PlayerCard, {
    props: { side: SIDES[Color.White], name, placed: 3, toMove: true },
    attachTo: document.body,
  })
}

describe('PlayerCard', () => {
  it('shows the player, the marbles placed and whose turn it is', () => {
    const wrapper = mountCard('Ada')

    expect(wrapper.find('h2').text()).toBe('Ada')
    expect(wrapper.text()).toContain('3 marbles placed')
    expect(wrapper.text()).toContain('To move')
    expect(wrapper.find('.marble--bone').exists()).toBe(true)
    wrapper.unmount()
  })

  it('renames the player on Enter', async () => {
    const wrapper = mountCard()

    await wrapper.find('.card__rename').trigger('click')
    const input = wrapper.find('input')
    expect(input.element.value).toBe('Bone')
    expect(input.attributes('aria-label')).toBe('Name of the Bone player')

    await input.setValue('Ada')
    await input.trigger('keydown', { key: 'Enter' })

    expect(wrapper.emitted('rename')).toEqual([['Ada']])
    expect(wrapper.find('input').exists()).toBe(false)
    wrapper.unmount()
  })

  it('renames the player when the field loses focus', async () => {
    const wrapper = mountCard()

    await wrapper.find('.card__rename').trigger('click')
    await wrapper.find('input').setValue('Grace')
    await wrapper.find('input').trigger('blur')

    expect(wrapper.emitted('rename')).toEqual([['Grace']])
    wrapper.unmount()
  })

  it('keeps the name on Escape', async () => {
    const wrapper = mountCard()

    await wrapper.find('.card__rename').trigger('click')
    await wrapper.find('input').setValue('Nope')
    await wrapper.find('input').trigger('keydown', { key: 'Escape' })

    expect(wrapper.emitted('rename')).toBeUndefined()
    expect(wrapper.find('h2').text()).toBe('Bone')
    wrapper.unmount()
  })
})
