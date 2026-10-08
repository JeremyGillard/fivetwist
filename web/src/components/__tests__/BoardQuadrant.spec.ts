import { flushPromises, mount } from '@vue/test-utils'
import { afterEach, describe, expect, it, vi } from 'vitest'

import BoardQuadrant from '../BoardQuadrant.vue'
import { SIDES } from '@/game/sides'
import { Color, Direction, Quadrant } from '@/wasm/pentago.js'

function mountQuadrant() {
  return mount(BoardQuadrant, {
    props: {
      quadrant: Quadrant.TopLeft,
      cells: Array.from({ length: 36 }, () => 0 as const),
      winning: [],
      mode: 'idle',
      current: SIDES[Color.White],
      selected: false,
      twisting: null,
    },
  })
}

/** A stand-in for the Web Animations API, which jsdom does not implement. */
function fakeAnimation() {
  let finish!: () => void
  let abort!: (reason: unknown) => void
  const finished = new Promise<void>((resolve, reject) => {
    finish = resolve
    abort = reject
  })
  const cancel = vi.fn<() => void>(() => abort(new DOMException('cancelled', 'AbortError')))
  return { animation: { finished, cancel }, finish }
}

/** Makes every element animate with `animation`, and records the calls. */
function installAnimate(animation: ReturnType<typeof fakeAnimation>['animation']) {
  const animate = vi.fn<HTMLElement['animate']>(() => animation as unknown as Animation)
  HTMLElement.prototype.animate = animate
  return animate
}

describe('BoardQuadrant twist', () => {
  afterEach(() => {
    // @ts-expect-error jsdom has no Element.animate; tests add and remove their own.
    delete HTMLElement.prototype.animate
  })

  it('reports the twist at once when it cannot be animated', async () => {
    const wrapper = mountQuadrant()

    await wrapper.setProps({ twisting: Direction.Clockwise })

    expect(wrapper.emitted('twisted')).toHaveLength(1)
  })

  it('turns a quarter of a turn, then reports the twist', async () => {
    const { animation, finish } = fakeAnimation()
    const animate = installAnimate(animation)
    const wrapper = mountQuadrant()

    await wrapper.setProps({ twisting: Direction.CounterClockwise })

    expect(animate).toHaveBeenCalledOnce()
    expect(animate.mock.calls[0]).toMatchObject([[{ rotate: '0deg' }, { rotate: '-90deg' }], {}])
    expect(wrapper.classes()).toContain('quadrant--twisting')
    expect(wrapper.emitted('twisted')).toBeUndefined()

    finish()
    await flushPromises()
    expect(wrapper.emitted('twisted')).toHaveLength(1)

    // The parent applies the rotation and clears the prop: the animation is dropped.
    await wrapper.setProps({ twisting: null })
    expect(animation.cancel).toHaveBeenCalledOnce()
    expect(wrapper.classes()).not.toContain('quadrant--twisting')
  })

  it('turns clockwise by a positive angle', async () => {
    const animate = installAnimate(fakeAnimation().animation)
    const wrapper = mountQuadrant()

    await wrapper.setProps({ twisting: Direction.Clockwise })

    expect(animate.mock.calls[0]).toMatchObject([[{ rotate: '0deg' }, { rotate: '90deg' }], {}])
  })

  it('reports the twist even if the animation never finishes', async () => {
    vi.useFakeTimers()
    try {
      installAnimate(fakeAnimation().animation)
      const wrapper = mountQuadrant()

      await wrapper.setProps({ twisting: Direction.Clockwise })
      expect(wrapper.emitted('twisted')).toBeUndefined()

      await vi.advanceTimersByTimeAsync(1000)
      expect(wrapper.emitted('twisted')).toHaveLength(1)
    } finally {
      vi.useRealTimers()
    }
  })

  it('does not animate in a hidden tab, where animations stand still', async () => {
    const hidden = vi.spyOn(document, 'hidden', 'get').mockReturnValue(true)
    const animate = installAnimate(fakeAnimation().animation)
    const wrapper = mountQuadrant()

    await wrapper.setProps({ twisting: Direction.Clockwise })

    expect(animate).not.toHaveBeenCalled()
    expect(wrapper.emitted('twisted')).toHaveLength(1)
    hidden.mockRestore()
  })

  it('does not report a twist that was cancelled', async () => {
    const { animation } = fakeAnimation()
    installAnimate(animation)
    const wrapper = mountQuadrant()

    await wrapper.setProps({ twisting: Direction.Clockwise })
    await wrapper.setProps({ twisting: null })
    await flushPromises()

    expect(animation.cancel).toHaveBeenCalledOnce()
    expect(wrapper.emitted('twisted')).toBeUndefined()
  })
})
