import { createApp } from 'vue'

import './assets/main.css'
import App from './App.vue'
import init from './wasm/pentago.js'

// The rules live in the WebAssembly module: nothing can be played until it is loaded.
try {
  await init()
  createApp(App).mount('#app')
} catch (error) {
  console.error(error)
  const message = document.createElement('p')
  message.className = 'load-error'
  message.textContent = 'The game could not be loaded. Please reload the page.'
  document.querySelector('#app')?.replaceChildren(message)
}
