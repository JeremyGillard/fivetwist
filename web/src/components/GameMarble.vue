<script setup lang="ts">
import type { Side } from '@/game/sides'

defineProps<{
  side: Side
  /** Part of a winning line: drawn with a bright ring. */
  winning?: boolean
  /** The game was won elsewhere: drawn faded, so the winning line stands out. */
  faded?: boolean
}>()
</script>

<template>
  <span
    class="marble"
    :class="[`marble--${side.tone}`, { 'marble--winning': winning, 'marble--faded': faded }]"
  />
</template>

<style scoped>
.marble {
  display: block;
  width: 100%;
  aspect-ratio: 1;
  border-radius: 50%;
  border: 2px solid #0c0f11;
  box-shadow:
    0 3px 6px rgb(0 0 0 / 0.55),
    inset 0 -3px 6px rgb(0 0 0 / 0.25);
  transition: opacity 0.3s;
}

.marble--bone {
  background: radial-gradient(
    circle at 36% 30%,
    #ffffff 0%,
    #f1ece0 38%,
    #cfc8b8 72%,
    #a39c8d 100%
  );
}

.marble--amber {
  background: radial-gradient(
    circle at 36% 30%,
    #ffcf96 0%,
    #f6993a 36%,
    #dd7717 68%,
    #8f4409 100%
  );
}

.marble--winning {
  box-shadow:
    0 0 0 3px var(--text),
    0 0 18px 4px rgb(255 255 255 / 0.35),
    inset 0 -3px 6px rgb(0 0 0 / 0.25);
}

.marble--faded {
  opacity: 0.4;
}
</style>
