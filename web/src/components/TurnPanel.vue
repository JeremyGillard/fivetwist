<script setup lang="ts">
import { computed } from 'vue'

import type { Snapshot } from '@/composables/useGame'
import { SIDES } from '@/game/sides'
import { Color, Outcome, Phase } from '@/wasm/pentago.js'

const props = defineProps<{ snapshot: Snapshot }>()

const WINNERS: Partial<Record<Outcome, Color>> = {
  [Outcome.WhiteWins]: Color.White,
  [Outcome.BlackWins]: Color.Black,
}

const heading = computed(() => {
  const { outcome, current } = props.snapshot
  if (outcome === undefined) return `${SIDES[current].name}'s turn`

  const winner = WINNERS[outcome]
  return winner === undefined ? 'Draw' : `${SIDES[winner].name} wins`
})

/** The two halves of a turn; a step is done, current or still to come. */
const steps = computed(() => {
  const rotating = props.snapshot.phase === Phase.Rotation
  return [
    { number: 1, label: 'Place a marble', state: rotating ? 'done' : 'current' },
    { number: 2, label: 'Twist a quadrant', state: rotating ? 'current' : 'upcoming' },
  ]
})

const playing = computed(() => props.snapshot.phase !== Phase.Over)
</script>

<template>
  <section class="panel" aria-live="polite">
    <h2 class="panel__heading">{{ heading }}</h2>
    <ol v-if="playing" class="steps">
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
</style>
