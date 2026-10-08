# Pentago web app

Vue 3 + Vite + TypeScript front-end. The rules run in WebAssembly, built from `crates/pentago-wasm`.

## Requirements

- Node `^22.18.0 || >=24.12.0`
- Rust with the `wasm32-unknown-unknown` target, and `wasm-pack`

## Commands

```sh
npm install
npm run dev         # builds the WebAssembly package, then starts the dev server
npm run test:unit   # builds the WebAssembly package, then runs Vitest
npm run test:e2e    # builds the app, serves it, and plays it in Chromium, Firefox and WebKit
npm run build       # builds the WebAssembly package, type-checks, bundles into dist/
npm run wasm        # only rebuilds the WebAssembly package into src/wasm (gitignored)
npm run type-check  # needs src/wasm to exist: run `npm run wasm` once first
npm run lint
npm run format
```

After changing Rust code, restart `npm run dev` or run `npm run wasm`.

## Tests

- **Unit and component tests** (`src/**/__tests__`, Vitest): fast, run in a simulated DOM. They
  cover the game composable against the real WebAssembly module and the components' logic.
- **End-to-end tests** (`e2e/`, Playwright): a few flows played with real clicks in real browsers,
  against the production build. They cover what a simulated DOM cannot: loading the WebAssembly
  module, the twist animation, and controls being reachable on screen.

The first time, download the browsers with `npx playwright install`. Useful variants:

```sh
npx playwright test --ui                 # interactive runner
npx playwright test --project=webkit     # one browser only
npx playwright show-trace <trace.zip>    # inspect a failure recorded on retry
```

## Layout

- `src/composables/useGame.ts`: owns the game and exposes a snapshot of its state.
- `src/composables/usePlayers.ts`, `useSound.ts`: player names and the mute switch, both kept in
  `localStorage`.
- `src/game/`: board geometry and how the two sides are displayed.
- `src/components/`: the board, player cards, turn panel and dialogs.
- `src/assets/sounds/`: placement, twist and end-of-game sounds.
- `e2e/`: end-to-end tests; `game.ts` describes the page as a player uses it.
- `design/`: the mockup the interface follows.
