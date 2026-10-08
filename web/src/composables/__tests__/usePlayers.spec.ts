import { describe, expect, it } from 'vitest'

import { MAX_NAME_LENGTH, usePlayers } from '../usePlayers'
import { Color } from '@/wasm/pentago.js'

describe('usePlayers', () => {
  it('names the players after their side by default', () => {
    const { names } = usePlayers()
    expect(names[Color.White]).toBe('Bone')
    expect(names[Color.Black]).toBe('Amber')
  })

  it('renames a player and remembers it on the next visit', () => {
    usePlayers().rename(Color.Black, 'Jeremy')

    const { names } = usePlayers()
    expect(names[Color.Black]).toBe('Jeremy')
    expect(names[Color.White]).toBe('Bone')
  })

  it('tidies the name that was typed', () => {
    const { names, rename } = usePlayers()

    rename(Color.White, '   Ada    Lovelace  ')
    expect(names[Color.White]).toBe('Ada Lovelace')

    rename(Color.White, 'x'.repeat(40))
    expect(names[Color.White]).toHaveLength(MAX_NAME_LENGTH)
  })

  it('goes back to the name of the side when the name is emptied', () => {
    const { names, rename } = usePlayers()
    rename(Color.Black, 'Jeremy')

    rename(Color.Black, '   ')

    expect(names[Color.Black]).toBe('Amber')
  })

  it('ignores stored names it cannot read', () => {
    localStorage.setItem('fivetwist.players', '{not json')
    expect(usePlayers().names[Color.White]).toBe('Bone')

    localStorage.setItem('fivetwist.players', JSON.stringify({ 1: 42, 2: '' }))
    const { names } = usePlayers()
    expect(names[Color.White]).toBe('Bone')
    expect(names[Color.Black]).toBe('Amber')
  })
})
