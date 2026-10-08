<script setup lang="ts">
import BaseDialog from './BaseDialog.vue'

defineProps<{ open: boolean }>()
const emit = defineEmits<{ close: [] }>()
</script>

<template>
  <BaseDialog :open="open" title="How to play" @close="emit('close')">
    <p>Be the first to line up five of your marbles: across, down or diagonally.</p>
    <ol class="rules">
      <li><strong>Place a marble</strong> in any empty hole.</li>
      <li>
        <strong>Twist a quadrant</strong> a quarter turn, clockwise or counter-clockwise. Any of the
        four quadrants, not only the one you played in. The twist is not optional.
      </li>
    </ol>
    <ul class="rules">
      <li>Five in a row as soon as you place your marble wins at once, without a twist.</li>
      <li>A twist can line up your opponent’s marbles: then they win.</li>
      <li>If a twist lines up both sides at the same time, the game is a draw.</li>
      <li>If the board fills up with no five in a row, the game is a draw.</li>
    </ul>
    <p class="credit">
      Fivetwist follows the rules of the board game Pentago. It is an unofficial project, not
      affiliated with or endorsed by the game’s publishers.
    </p>
    <template #actions>
      <button class="dialog__button dialog__button--primary" type="button" @click="emit('close')">
        Got it
      </button>
    </template>
  </BaseDialog>
</template>

<style scoped>
.rules {
  display: grid;
  gap: 0.5rem;
  margin: 1rem 0 0;
  padding-left: 1.25rem;
}

p {
  margin: 0;
}

.credit {
  margin-top: 1.25rem;
  font-size: 0.8rem;
  opacity: 0.75;
}

strong {
  color: var(--text);
}
</style>
