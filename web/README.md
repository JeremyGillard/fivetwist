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
npm run build       # builds the WebAssembly package, type-checks, bundles into dist/
npm run wasm        # only rebuilds the WebAssembly package into src/wasm (gitignored)
npm run type-check  # needs src/wasm to exist: run `npm run wasm` once first
npm run lint
npm run format
```

After changing Rust code, restart `npm run dev` or run `npm run wasm`.

## Layout

- `src/composables/useGame.ts`: owns the game and exposes a snapshot of its state.
- `src/game/`: board geometry and how the two sides are displayed.
- `src/components/`: the board, player cards and turn panel.
- `design/`: the mockup the interface follows.
