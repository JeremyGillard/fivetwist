<script setup lang="ts">
import { computed } from 'vue'

import type { Snapshot } from '@/composables/useGame'
import { Color, Direction, Outcome, Phase } from '@/wasm/pentago.js'

const props = defineProps<{
  snapshot: Snapshot
  /** The names of the two players. */
  names: Readonly<Record<Color, string>>
  /** Whether a quadrant is selected and can be twisted now. */
  canTwist: boolean
  /** Why the last action was refused, if it was. */
  error: string | null
}>()

const emit = defineEmits<{
  twist: [direction: Direction]
  playAgain: []
}>()

const WINNERS: Partial<Record<Outcome, Color>> = {
  [Outcome.WhiteWins]: Color.White,
  [Outcome.BlackWins]: Color.Black,
}

const playing = computed(() => props.snapshot.outcome === undefined)

const heading = computed(() => {
  const { outcome, current } = props.snapshot
  if (outcome === undefined) return `${props.names[current]}’s turn`

  const winner = WINNERS[outcome]
  return winner === undefined ? 'Draw' : `${props.names[winner]} wins`
})

/** How the game ended, in a sentence. */
const ending = computed(() => {
  const { outcome, winning } = props.snapshot
  if (outcome === undefined) return ''
  if (outcome !== Outcome.Draw) return 'Five in a row.'
  return winning.length > 0
    ? 'The last twist lined up five for both sides.'
    : 'The board is full and nobody lined up five.'
})

/** The two halves of a turn; a step is done, current or still to come. */
const steps = computed(() => {
  const rotating = props.snapshot.phase === Phase.Rotation
  return [
    { number: 1, label: 'Place a marble', state: rotating ? 'done' : 'current' },
    { number: 2, label: 'Twist a quadrant', state: rotating ? 'current' : 'upcoming' },
  ]
})

const hint = computed(() => {
  if (props.snapshot.phase !== Phase.Rotation) return 'Click an empty hole.'
  return 'Pick a quadrant, then choose a direction.'
})
</script>

<template>
  <section class="panel" :class="{ 'panel--over': !playing }">
    <h2 class="panel__heading" aria-live="polite">{{ heading }}</h2>

    <template v-if="playing">
      <ol class="steps">
        <li
          v-for="step in steps"
          :key="step.number"
          class="step"
          :class="`step--${step.state}`"
          :aria-current="step.state === 'current' ? 'step' : undefined"
        >
          <span class="step__badge" aria-hidden="true">{{
            step.state === 'done' ? '✓' : step.number
          }}</span>
          <span class="step__label">{{ step.label }}</span>
        </li>
      </ol>

      <div class="twist">
        <button
          class="twist__button"
          type="button"
          aria-label="Twist counter-clockwise"
          :disabled="!canTwist"
          @click="emit('twist', Direction.CounterClockwise)"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
            <path d="M3 3v5h5" />
          </svg>
        </button>
        <button
          class="twist__button twist__button--primary"
          type="button"
          aria-label="Twist clockwise"
          :disabled="!canTwist"
          @click="emit('twist', Direction.Clockwise)"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8" />
            <path d="M21 3v5h-5" />
          </svg>
        </button>
      </div>

      <p class="panel__hint">{{ hint }}</p>
    </template>

    <template v-else>
      <p class="panel__ending">{{ ending }}</p>
      <button class="panel__again" type="button" @click="emit('playAgain')">Play again</button>
    </template>

    <p v-if="error" class="panel__error" role="alert">{{ error }}</p>
  </section>
</template>

<style scoped>
.panel__heading {
  margin: 0 0 1.25rem;
  font-size: 0.85rem;
  font-weight: 600;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: var(--muted);
  overflow-wrap: anywhere;
}

.panel--over .panel__heading {
  margin-bottom: 0.5rem;
  font-size: 1.6rem;
  letter-spacing: 0.04em;
  text-transform: none;
  color: var(--text);
}

.panel__ending {
  margin: 0;
  color: var(--muted);
}

.panel__again {
  margin-top: 1.5rem;
  padding: 0.75rem 1.6rem;
  border: 1.5px solid var(--amber);
  border-radius: 999px;
  background: var(--amber);
  color: #1a1206;
  font-weight: 700;
  cursor: pointer;
}

.panel__again:focus-visible {
  outline: 2px solid var(--amber);
  outline-offset: 3px;
}

.steps {
  display: grid;
  gap: 0.9rem;
  margin: 0;
  padding: 0;
  list-style: none;
}

.step {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  color: var(--muted);
}

.step__badge {
  display: grid;
  place-items: center;
  flex: none;
  width: 2rem;
  aspect-ratio: 1;
  border: 1.5px solid var(--line);
  border-radius: 50%;
  font-size: 0.85rem;
  font-weight: 600;
}

.step--done .step__badge {
  border-color: #46525a;
  color: var(--text);
}

.step--current {
  color: var(--text);
  font-weight: 600;
}

.step--current .step__badge {
  border-color: var(--amber);
  background: var(--amber);
  color: #1a1206;
}

.twist {
  display: flex;
  gap: 0.75rem;
  margin-top: 1.5rem;
}

.twist__button {
  display: grid;
  place-items: center;
  width: 4rem;
  aspect-ratio: 1;
  padding: 0;
  border: 1.5px solid #46525a;
  border-radius: 50%;
  background: var(--surface);
  color: var(--text);
  cursor: pointer;
  transition:
    opacity 0.15s,
    transform 0.1s,
    border-color 0.15s;
}

.twist__button--primary {
  border-color: var(--amber);
  background: var(--amber);
  color: #1a1206;
}

.twist__button svg {
  width: 1.5rem;
  fill: none;
  stroke: currentColor;
  stroke-width: 2;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.twist__button:hover:not(:disabled) {
  border-color: var(--text);
}

.twist__button:active:not(:disabled) {
  transform: scale(0.95);
}

.twist__button:focus-visible {
  outline: 2px solid var(--amber);
  outline-offset: 3px;
}

.twist__button:disabled {
  opacity: 0.3;
  cursor: default;
}

.panel__hint {
  margin: 1.25rem 0 0;
  font-size: 0.85rem;
  color: var(--muted);
}

.panel__error {
  margin: 1rem 0 0;
  font-size: 0.85rem;
  color: #ff8f7a;
}

/* Stacked layout: steps on the left, twist buttons on the right, within thumb reach. */
@media (max-width: 68rem) {
  .panel:not(.panel--over) {
    display: grid;
    grid-template-columns: 1fr auto;
    grid-template-areas:
      'heading heading'
      'steps twist'
      'hint hint'
      'error error';
    align-items: center;
    column-gap: 1rem;
  }

  .panel__heading {
    grid-area: heading;
    margin-bottom: 0.9rem;
  }

  .steps {
    grid-area: steps;
  }

  .twist {
    grid-area: twist;
    margin-top: 0;
  }

  .panel__hint {
    grid-area: hint;
    margin-top: 0.9rem;
  }

  .panel__error {
    grid-area: error;
  }

  .panel--over {
    text-align: center;
  }
}
</style>
