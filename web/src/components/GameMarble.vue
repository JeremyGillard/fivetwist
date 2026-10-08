<script setup lang="ts">
import type { Side } from '@/game/sides'

defineProps<{ side: Side }>()
</script>

<template>
  <span class="marble" :class="`marble--${side.tone}`" />
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
</style>
