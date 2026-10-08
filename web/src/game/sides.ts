import { Color } from '@/wasm/pentago.js'

/** How a side is shown. The rules call the sides White and Black; the design shows Bone and Amber. */
export interface Side {
  color: Color
  name: string
  /** Suffix of the CSS classes that paint this side. */
  tone: 'bone' | 'amber'
}

export const SIDES: Record<Color, Side> = {
  [Color.White]: { color: Color.White, name: 'Bone', tone: 'bone' },
  [Color.Black]: { color: Color.Black, name: 'Amber', tone: 'amber' },
}

/** Both sides, in playing order. */
export const SIDE_LIST: readonly Side[] = [SIDES[Color.White], SIDES[Color.Black]]
