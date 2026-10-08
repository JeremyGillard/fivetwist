import { reactive, readonly } from 'vue'

import { SIDES } from '@/game/sides'
import { Color } from '@/wasm/pentago.js'

const STORAGE_KEY = 'fivetwist.players'
export const MAX_NAME_LENGTH = 16

type Names = Record<Color, string>

function defaults(): Names {
  return { [Color.White]: SIDES[Color.White].name, [Color.Black]: SIDES[Color.Black].name }
}

/** Trims a typed name; an empty one falls back to the name of the side. */
function clean(name: unknown, color: Color): string {
  const trimmed = typeof name === 'string' ? name.trim().replace(/\s+/g, ' ') : ''
  return trimmed.slice(0, MAX_NAME_LENGTH).trim() || SIDES[color].name
}

function load(): Names {
  const names = defaults()
  try {
    const stored: unknown = JSON.parse(localStorage.getItem(STORAGE_KEY) ?? 'null')
    if (stored && typeof stored === 'object') {
      const record = stored as Record<string, unknown>
      names[Color.White] = clean(record[Color.White], Color.White)
      names[Color.Black] = clean(record[Color.Black], Color.Black)
    }
  } catch {
    // Unreadable or blocked storage: play with the default names.
  }
  return names
}

/** The names of the two players, kept across games and visits. */
export function usePlayers() {
  const names = reactive(load())

  function rename(color: Color, name: string) {
    names[color] = clean(name, color)
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(names))
    } catch {
      // The name still applies to this visit.
    }
  }

  return { names: readonly(names), rename }
}
