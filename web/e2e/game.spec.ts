import { expect, test } from '@playwright/test'

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
  await expect(game.heading).toHaveText("Bone's turn")
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
  await expect(game.heading).toHaveText("Bone's turn")

  await expect(game.heading).toHaveText("Amber's turn")
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
  await expect(game.heading).toHaveText("Amber's turn")
  await expect(game.hole(4, 4, 'Bone')).toBeVisible()

  await game.place(4, 1)
  await game.twist('top-right', 'counter-clockwise')

  // Counter-clockwise, the top-left corner of the quadrant ends up in its bottom-left corner.
  await expect(game.hole(4, 3, 'Amber')).toBeVisible()
  await expect(game.heading).toHaveText("Bone's turn")
})

test('does not let a marble be placed on another, or during the twist', async () => {
  await game.place(2, 2)

  // Every hole is locked until the twist is done.
  await expect(game.hole(1, 1)).toBeDisabled()
  await game.twist('bottom-right')

  await expect(game.hole(2, 2, 'Bone')).toBeDisabled()
  await expect(game.hole(1, 1)).toBeEnabled()
})

test('ends the game on five in a row', async () => {
  // Each row: where Bone plays, then where Amber plays. Bone fills the top row.
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

  await expect(game.heading).toHaveText('Bone wins')
  await expect(game.marbles).toHaveCount(9)
  await expect(game.hole(6, 1)).toBeDisabled()
  await expect(game.quadrant('top-left')).toHaveCount(0)
  await expect(game.twistButton('clockwise')).toHaveCount(0)

  await game.newGame.click()

  await expect(game.heading).toHaveText("Bone's turn")
  await expect(game.marbles).toHaveCount(0)
  await expect(game.hole(6, 1)).toBeEnabled()
})

test('starts over cleanly when New game is clicked during a twist', async ({ page }) => {
  await game.place(1, 1)
  await game.twistButton('clockwise').click()
  await expect(game.twistingTiles).toHaveCount(1)

  await game.newGame.click()

  await expect(game.twistingTiles).toHaveCount(0)
  await expect(game.marbles).toHaveCount(0)
  await expect(game.heading).toHaveText("Bone's turn")
  await expect(game.currentStep).toContainText('Place a marble')

  // The interrupted twist must not be applied to the new game once its animation would have ended.
  await page.waitForTimeout(1000)
  await expect(game.marbles).toHaveCount(0)
  await expect(game.heading).toHaveText("Bone's turn")
  await expect(game.hole(1, 1)).toBeEnabled()
})

test.describe('with reduced motion', () => {
  test.use({ reducedMotion: 'reduce' })

  test('twists without animating', async () => {
    await game.place(1, 1)
    await game.twistButton('clockwise').click()

    await expect(game.heading).toHaveText("Amber's turn")
    await expect(game.hole(3, 1, 'Bone')).toBeVisible()
    await expect(game.twistingTiles).toHaveCount(0)
  })
})
