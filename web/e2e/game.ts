import { expect, type Locator, type Page } from '@playwright/test'

export type Side = 'Bone' | 'Amber'
export type QuadrantName = 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right'
export type TwistDirection = 'clockwise' | 'counter-clockwise'

/**
 * The game as a player sees and uses it: everything is found by its accessible name, and every
 * action is a real click, so a control hidden under another one makes the test fail.
 */
export class GamePage {
  /** Uncaught errors thrown by the page since it was opened. */
  readonly errors: Error[] = []

  constructor(readonly page: Page) {
    page.on('pageerror', (error) => this.errors.push(error))
  }

  async open() {
    await this.page.goto('/')
    // The board only appears once the WebAssembly module has been fetched and started.
    await expect(this.heading).toBeVisible()
  }

  /** "Bone's turn", "Amber wins", "Draw"... */
  get heading(): Locator {
    return this.page.locator('.panel__heading')
  }

  get currentStep(): Locator {
    return this.page.locator('[aria-current="step"]')
  }

  get newGame(): Locator {
    return this.page.getByRole('button', { name: 'New game' })
  }

  /** Every hole of the board, whatever it holds. Columns and rows start at 1, as on screen. */
  get holes(): Locator {
    return this.page.getByRole('button', { name: /^Row \d, column \d:/ })
  }

  get marbles(): Locator {
    return this.page.getByRole('button', { name: /^Row \d, column \d: (Bone|Amber)$/ })
  }

  hole(column: number, row: number, content: Side | 'empty' = 'empty'): Locator {
    return this.page.getByRole('button', {
      name: `Row ${row}, column ${column}: ${content}`,
      exact: true,
    })
  }

  quadrant(name: QuadrantName): Locator {
    return this.page.getByRole('button', { name: `Select the ${name} quadrant` })
  }

  twistButton(direction: TwistDirection): Locator {
    return this.page.getByRole('button', { name: `Twist ${direction}`, exact: true })
  }

  /** The tiles that are turning right now. */
  get twistingTiles(): Locator {
    return this.page.locator('.quadrant--twisting')
  }

  async place(column: number, row: number) {
    await this.hole(column, row).click()
  }

  /** Selects a quadrant, twists it, and waits until it has stopped turning. */
  async twist(name: QuadrantName, direction: TwistDirection = 'clockwise') {
    await this.quadrant(name).click()
    await this.twistButton(direction).click()
    await expect(this.twistingTiles).toHaveCount(0)
  }

  /** A whole turn that leaves the marbles where they are: the bottom-right quadrant stays empty. */
  async playTurn(column: number, row: number) {
    await this.place(column, row)
    await this.twist('bottom-right')
  }
}
