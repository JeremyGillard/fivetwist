import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

import { initSync } from '../wasm/pentago.js'

// The browser fetches the WebAssembly module; tests load it from disk before any spec runs.
initSync({ module: readFileSync(resolve(import.meta.dirname, '../wasm/pentago_bg.wasm')) })
