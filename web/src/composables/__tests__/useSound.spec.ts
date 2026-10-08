import { describe, expect, it, vi } from 'vitest'

import { useSound } from '../useSound'

const played = () => vi.mocked(HTMLMediaElement.prototype.play)

describe('useSound', () => {
  it('plays sounds unless muted', () => {
    const { muted, play, toggleMuted } = useSound()
    expect(muted.value).toBe(false)

    play('place')
    expect(played()).toHaveBeenCalledOnce()

    toggleMuted()
    play('twist')
    expect(muted.value).toBe(true)
    expect(played()).toHaveBeenCalledOnce()

    toggleMuted()
    play('end')
    expect(played()).toHaveBeenCalledTimes(2)
  })

  it('remembers the mute choice on the next visit', () => {
    useSound().toggleMuted()

    const { muted, play } = useSound()
    expect(muted.value).toBe(true)
    play('place')
    expect(played()).not.toHaveBeenCalled()
  })

  it('stays quiet when the browser refuses to play', async () => {
    played().mockRejectedValueOnce(new DOMException('blocked', 'NotAllowedError'))
    const { play } = useSound()

    expect(() => play('place')).not.toThrow()
    // An unhandled rejection would fail the run.
    await Promise.resolve()
  })
})
