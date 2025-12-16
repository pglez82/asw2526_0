import { setWorldConstructor, Before, After, setDefaultTimeout } from '@cucumber/cucumber'
import { chromium } from 'playwright'

setDefaultTimeout(60_000)

class CustomWorld {
  browser = null
  page = null
}

setWorldConstructor(CustomWorld)

Before(async function () {
  // Allow turning off headless mode and enabling slow motion/devtools via env vars
  const headless = process.env.PLAYWRIGHT_HEADLESS !== 'false'
  const slowMo = process.env.PLAYWRIGHT_SLOWMO ? Number(process.env.PLAYWRIGHT_SLOWMO) : 0
  const devtools = process.env.PLAYWRIGHT_DEVTOOLS === 'true'

  this.browser = await chromium.launch({ headless, slowMo, devtools })
  this.page = await this.browser.newPage()
})

After(async function () {
  if (this.page) await this.page.close()
  if (this.browser) await this.browser.close()
})
