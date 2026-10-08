import { expect, test, type Page } from '@playwright/test'

import { GamePage } from './game'

let game: GamePage

test.beforeEach(async ({ page }) => {
  game = new GamePage(page)
  await game.open()
})

test.afterEach(() => {
  expect(game.errors).toEqual([])
})

test('loads the rules and shows a new game', async ({ page }) => {
  await expect(page).toHaveTitle('Pentago')
  await expect(game.heading).toHaveText('Bone’s turn')
  await expect(game.currentStep).toContainText('Place a marble')
  await expect(game.holes).toHaveCount(36)
  await expect(game.marbles).toHaveCount(0)
  await expect(game.twistButton('clockwise')).toBeDisabled()
  await expect(game.twistButton('counter-clockwise')).toBeDisabled()
})

test('plays a turn: place a marble, then twist the quadrant it is in', async () => {
  await game.place(1, 1)

  await expect(game.hole(1, 1, 'Bone')).toBeVisible()
  await expect(game.currentStep).toContainText('Twist a quadrant')
  await expect(game.quadrant('top-left')).toHaveAttribute('aria-pressed', 'true')

  await game.twistButton('clockwise').click()

  // The tile turns on screen before the rules are told: the turn has not passed yet.
  await expect(game.twistingTiles).toHaveCount(1)
  await expect(game.heading).toHaveText('Bone’s turn')

  await expect(game.heading).toHaveText('Amber’s turn')
  await expect(game.twistingTiles).toHaveCount(0)
  // Clockwise, the top-left corner of the quadrant ends up in its top-right corner.
  await expect(game.hole(3, 1, 'Bone')).toBeVisible()
  await expect(game.hole(1, 1)).toBeEnabled()
  await expect(game.marbles).toHaveCount(1)
  await expect(game.currentStep).toContainText('Place a marble')
})

test('twists any quadrant, in either direction', async () => {
  await game.place(4, 4)
  await expect(game.quadrant('bottom-right')).toHaveAttribute('aria-pressed', 'true')

  await game.quadrant('top-left').click()
  await expect(game.quadrant('top-left')).toHaveAttribute('aria-pressed', 'true')
  await expect(game.quadrant('bottom-right')).toHaveAttribute('aria-pressed', 'false')
  await game.twistButton('counter-clockwise').click()

  // The marble is elsewhere, so it has not moved.
  await expect(game.heading).toHaveText('Amber’s turn')
  await expect(game.hole(4, 4, 'Bone')).toBeVisible()

  await game.place(4, 1)
  await game.twist('top-right', 'counter-clockwise')

  // Counter-clockwise, the top-left corner of the quadrant ends up in its bottom-left corner.
  await expect(game.hole(4, 3, 'Amber')).toBeVisible()
  await expect(game.heading).toHaveText('Bone’s turn')
})

test('does not let a marble be placed on another, or during the twist', async () => {
  await game.place(2, 2)

  // Every hole is locked until the twist is done.
  await expect(game.hole(1, 1)).toBeDisabled()
  await game.twist('bottom-right')

  await expect(game.hole(2, 2, 'Bone')).toBeDisabled()
  await expect(game.hole(1, 1)).toBeEnabled()
})

/** Bone fills the top row while Amber plays the bottom-left corner; Bone wins on the ninth marble. */
async function playUntilBoneWins(game: GamePage) {
  const turns = [
    { bone: [1, 1], amber: [1, 6] },
    { bone: [2, 1], amber: [2, 6] },
    { bone: [3, 1], amber: [3, 6] },
    { bone: [4, 1], amber: [1, 5] },
  ] as const

  for (const { bone, amber } of turns) {
    await game.playTurn(bone[0], bone[1])
    await game.playTurn(amber[0], amber[1])
  }
  await game.place(5, 1)
}

test('ends the game on five in a row and shows the winning line', async () => {
  await playUntilBoneWins(game)

  await expect(game.heading).toHaveText('Bone wins')
  await expect(game.marbles).toHaveCount(9)
  await expect(game.winningMarbles).toHaveCount(5)
  await expect(game.hole(5, 1, 'Bone')).toHaveCount(0)
  await expect(game.hole(6, 1)).toBeDisabled()
  await expect(game.quadrant('top-left')).toHaveCount(0)
  await expect(game.twistButton('clockwise')).toHaveCount(0)

  // Nothing is lost by starting again now, so no question is asked.
  await game.playAgain.click()

  await expect(game.dialog).toHaveCount(0)
  await expect(game.heading).toHaveText('Bone’s turn')
  await expect(game.marbles).toHaveCount(0)
  await expect(game.hole(6, 1)).toBeEnabled()
})

test('asks before discarding a game in progress', async () => {
  await game.playTurn(2, 2)

  await game.newGame.click()

  await expect(game.dialog).toBeVisible()
  await expect(game.dialog).toContainText('Start a new game?')
  await game.dialog.getByRole('button', { name: 'Keep playing' }).click()
  await expect(game.dialog).toHaveCount(0)
  await expect(game.marbles).toHaveCount(1)

  await game.newGame.click()
  await game.dialog.getByRole('button', { name: 'New game' }).click()

  await expect(game.dialog).toHaveCount(0)
  await expect(game.marbles).toHaveCount(0)
  await expect(game.heading).toHaveText('Bone’s turn')
})

test('starts a new game at once on an untouched board', async () => {
  await game.newGame.click()

  await expect(game.dialog).toHaveCount(0)
  await expect(game.heading).toHaveText('Bone’s turn')
})

test('starts over cleanly when a new game is confirmed during a twist', async ({ page }) => {
  await game.place(1, 1)
  await game.twistButton('clockwise').click()
  await expect(game.twistingTiles).toHaveCount(1)

  await game.newGame.click()
  await game.dialog.getByRole('button', { name: 'New game' }).click()

  await expect(game.twistingTiles).toHaveCount(0)
  await expect(game.marbles).toHaveCount(0)
  await expect(game.heading).toHaveText('Bone’s turn')
  await expect(game.currentStep).toContainText('Place a marble')

  // A twist still pending from the old game must not land on the new one.
  await page.waitForTimeout(1000)
  await expect(game.marbles).toHaveCount(0)
  await expect(game.heading).toHaveText('Bone’s turn')
  await expect(game.hole(1, 1)).toBeEnabled()
})

test('explains the rules in a dialog that Escape closes', async ({ page }) => {
  await game.rules.click()

  await expect(game.dialog).toBeVisible()
  await expect(game.dialog).toContainText('How to play')
  await expect(game.dialog).toContainText('If a twist lines up both sides at the same time')

  await page.keyboard.press('Escape')

  await expect(game.dialog).toHaveCount(0)
  await expect(game.rules).toBeFocused()

  // A click beside the dialog closes it, and does not reach the board underneath.
  await game.rules.click()
  await expect(game.dialog).toBeVisible()
  await page.mouse.click(5, 5)
  await expect(game.dialog).toHaveCount(0)
  await expect(game.marbles).toHaveCount(0)

  await game.rules.click()
  await game.dialog.getByRole('button', { name: 'Got it' }).click()
  await expect(game.dialog).toHaveCount(0)
  await game.place(1, 1)
  await expect(game.hole(1, 1, 'Bone')).toBeVisible()
})

test('names the players, and remembers the names', async ({ page }) => {
  await game.rename('Bone', 'Ada')
  await game.rename('Amber', 'Grace')

  await expect(game.heading).toHaveText('Ada’s turn')
  await game.playTurn(1, 1)
  await expect(game.heading).toHaveText('Grace’s turn')

  await page.reload()

  await expect(game.heading).toHaveText('Ada’s turn')
  await expect(game.playerName('Amber')).toHaveText('Grace')
})

test.describe('sound', () => {
  /** The sound files the page asked to play, in order: "place", "twist" or "end". */
  async function played(page: Page): Promise<string[]> {
    return page.evaluate(() => (window as unknown as { played: string[] }).played)
  }

  test.beforeEach(async ({ page }) => {
    // Browsers under test have no speakers to check: record what the page asks to play.
    await page.addInitScript(() => {
      const record: string[] = []
      ;(window as unknown as { played: string[] }).played = record
      HTMLMediaElement.prototype.play = function () {
        record.push(new URL(this.src).pathname.split('/').pop()!.split(/[-.]/)[0]!)
        return Promise.resolve()
      }
    })
    await game.open()
  })

  test('plays for a placement, a twist and the end of the game', async ({ page }) => {
    await game.playTurn(1, 6)
    expect(await played(page)).toEqual(['place', 'twist'])

    await game.newGame.click()
    await game.dialog.getByRole('button', { name: 'New game' }).click()
    await playUntilBoneWins(game)

    await expect(game.heading).toHaveText('Bone wins')
    expect((await played(page)).slice(-2)).toEqual(['place', 'end'])
  })

  test('stays silent once turned off, and remembers it', async ({ page }) => {
    await expect(game.sound).toHaveAttribute('aria-pressed', 'true')

    await game.sound.click()
    await game.playTurn(1, 1)

    await expect(game.sound).toHaveAttribute('aria-pressed', 'false')
    expect(await played(page)).toEqual([])

    await page.reload()
    await expect(game.sound).toHaveAttribute('aria-pressed', 'false')
  })
})

for (const [device, viewport] of [
  ['phone', { width: 390, height: 844 }],
  ['small phone', { width: 320, height: 568 }],
  ['tablet', { width: 820, height: 1180 }],
] as const) {
  test.describe(`on a ${device}`, () => {
    test.use({ viewport })

    test('fits the screen and can be played', async ({ page }) => {
      // Nothing may stick out sideways.
      const overflow = await page.evaluate(
        () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
      )
      expect(overflow).toBe(0)

      const board = await page.getByRole('group', { name: 'Board' }).boundingBox()
      expect(board!.x).toBeGreaterThanOrEqual(0)
      expect(board!.x + board!.width).toBeLessThanOrEqual(viewport.width)
      expect(board!.width).toBeGreaterThan(viewport.width * 0.6)

      await game.place(6, 6)
      await game.twist('top-left', 'counter-clockwise')

      await expect(game.heading).toHaveText('Amber’s turn')
      await expect(game.hole(6, 6, 'Bone')).toBeVisible()

      await game.rules.click()
      await expect(game.dialog.getByRole('button', { name: 'Got it' })).toBeInViewport()
    })
  })
}

test.describe('with reduced motion', () => {
  test.use({ reducedMotion: 'reduce' })

  test('twists without animating', async () => {
    await game.place(1, 1)
    await game.twistButton('clockwise').click()

    await expect(game.heading).toHaveText('Amber’s turn')
    await expect(game.hole(3, 1, 'Bone')).toBeVisible()
    await expect(game.twistingTiles).toHaveCount(0)
  })
})
