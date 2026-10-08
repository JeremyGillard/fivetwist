import { readonly, ref } from 'vue'

import endUrl from '@/assets/sounds/end.wav'
import placeUrl from '@/assets/sounds/place.wav'
import twistUrl from '@/assets/sounds/twist.wav'

const STORAGE_KEY = 'pentago.muted'

const URLS = { place: placeUrl, twist: twistUrl, end: endUrl } as const

export type SoundName = keyof typeof URLS

function loadMuted(): boolean {
  try {
    return localStorage.getItem(STORAGE_KEY) === 'true'
  } catch {
    return false
  }
}

/** Short sounds for the events of a game, with a mute switch kept across visits. */
export function useSound() {
  const muted = ref(loadMuted())
  const players = new Map<SoundName, HTMLAudioElement>()

  function play(name: SoundName) {
    if (muted.value) return
    try {
      let audio = players.get(name)
      if (!audio) {
        audio = new Audio(URLS[name])
        players.set(name, audio)
      }
      audio.currentTime = 0
      // Browsers refuse to play before the first click on the page; a missed sound is harmless.
      audio.play()?.catch(() => {})
    } catch {
      // No audio support: the game is silent.
    }
  }

  function toggleMuted() {
    muted.value = !muted.value
    if (muted.value) players.forEach((audio) => audio.pause())
    try {
      localStorage.setItem(STORAGE_KEY, String(muted.value))
    } catch {
      // The choice still applies to this visit.
    }
  }

  return { muted: readonly(muted), play, toggleMuted }
}
