import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

import { beforeEach, vi } from 'vitest'

import { initSync } from '../wasm/pentago.js'

// The browser fetches the WebAssembly module; tests load it from disk before any spec runs.
initSync({ module: readFileSync(resolve(import.meta.dirname, '../wasm/pentago_bg.wasm')) })

// jsdom has no audio playback. Specs that care about sound spy on these.
HTMLMediaElement.prototype.play = vi.fn<HTMLMediaElement['play']>(() => Promise.resolve())
HTMLMediaElement.prototype.pause = vi.fn<HTMLMediaElement['pause']>()

beforeEach(() => {
  localStorage.clear()
  vi.mocked(HTMLMediaElement.prototype.play).mockClear()
  vi.mocked(HTMLMediaElement.prototype.pause).mockClear()
})
