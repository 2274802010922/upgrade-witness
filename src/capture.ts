import { mkdir } from 'node:fs/promises';
import { resolve } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';
import { chromium, type Browser, type Locator, type Page } from 'playwright';
import type { Assertion, Check, ScenarioCapture, WitnessConfig } from './types.js';

// Mask before encoding the screenshot. This does not redact explicitly requested assertion values.
const sensitiveElements = 'input:not([type="button"]):not([type="submit"]):not([type="reset"]), textarea, [contenteditable]:not([contenteditable="false"]), [data-uw-sensitive]';

function safeError(stage: string, error: unknown): string {
  const timeout = error instanceof Error && error.name === 'TimeoutError';
  return `${stage} ${timeout ? 'timed out' : 'could not be completed'}. Details omitted to avoid exposing page or process data.`;
}

async function readValue(locator: Locator, assertion: Assertion, timeout: number): Promise<string | boolean | null> {
  const count = await locator.count();
  if (count > 1) throw new Error('Ambiguous assertion selector');
  if (count === 0) return assertion.kind === 'visible' ? false : null;
  switch (assertion.kind) {
    case 'visible': return locator.isVisible();
    case 'text': return (await locator.textContent({ timeout }))?.trim() ?? '';
    case 'attribute': return locator.getAttribute(assertion.name!, { timeout });
    case 'style': return locator.evaluate((element, name) => getComputedStyle(element).getPropertyValue(name).trim(), assertion.name!, { timeout });
  }
}

async function captureCheck(page: Page, assertion: Assertion, expected: string | boolean, timeoutMs: number): Promise<Check> {
  const deadline = Date.now() + timeoutMs;
  let actual: Check['actual'] = null;
  try {
    const locator = page.locator(assertion.selector);
    do {
      actual = await readValue(locator, assertion, Math.max(1, deadline - Date.now()));
      if (actual === expected) return { id: assertion.id, expected, actual, passed: true };
      if (Date.now() >= deadline) break;
      await delay(Math.min(50, deadline - Date.now()));
    } while (Date.now() < deadline);
    return { id: assertion.id, expected, actual, passed: false };
  } catch (error) {
    return { id: assertion.id, expected, actual: null, passed: false, error: safeError('Assertion evaluation', error) };
  }
}

export async function captureScenarios(baseUrl: string, config: WitnessConfig, outDir: string, label: string): Promise<ScenarioCapture[]> {
  let browser: Browser;
  try {
    await mkdir(outDir, { recursive: true });
    browser = await chromium.launch({ executablePath: process.env.UW_BROWSER_EXECUTABLE || undefined, headless: true, timeout: config.timeoutMs });
  } catch (error) {
    return config.scenarios.map(scenario => ({ scenarioId: scenario.id, checks: [], browserVersion: 'unavailable', error: safeError('Browser or output initialization', error) }));
  }

  const captures: ScenarioCapture[] = [];
  try {
    for (const scenario of config.scenarios) {
      const capture: ScenarioCapture = { scenarioId: scenario.id, checks: [], browserVersion: browser.version() };
      captures.push(capture);
      let context;
      let stage = 'Scenario initialization';
      try {
        context = await browser.newContext({ viewport: scenario.viewport, locale: 'en-US', timezoneId: 'UTC', reducedMotion: 'reduce', colorScheme: 'light', deviceScaleFactor: 1, serviceWorkers: 'block' });
        context.setDefaultTimeout(config.timeoutMs);
        context.setDefaultNavigationTimeout(config.timeoutMs);
        const page = await context.newPage();
        const target = new URL(scenario.route, baseUrl);
        if (target.origin !== new URL(baseUrl).origin) throw new Error('Cross-origin route');
        stage = 'Page navigation';
        const response = await page.goto(target.href, { waitUntil: 'load' });
        if (!response || !response.ok()) throw new Error('Unsuccessful HTTP response');
        for (const [index, action] of scenario.actions.entries()) {
          stage = `Action ${index + 1}`;
          const locator = page.locator(action.selector);
          if (action.type === 'click') await locator.click();
          else if (action.type === 'fill') await locator.fill(action.value!);
          else await locator.press(action.value!);
        }
        stage = 'Font and rendering readiness';
        await page.waitForFunction(() => document.fonts.status === 'loaded', undefined, { timeout: config.timeoutMs });
        await page.evaluate(async () => {
          await document.fonts.ready;
          await new Promise<void>(done => requestAnimationFrame(() => requestAnimationFrame(() => done())));
        });
        for (const assertion of scenario.assertions) {
          const intent = label === 'head' || label === 'head-repeat' ? config.intendedChanges.find(item => item.scenarioId === scenario.id && item.assertionId === assertion.id) : undefined;
          capture.checks.push(await captureCheck(page, assertion, intent?.expected ?? assertion.expected, config.timeoutMs));
        }
        if (scenario.screenshot) {
          stage = 'Masked screenshot capture';
          const screenshotPath = resolve(outDir, `${scenario.id}.png`);
          await page.screenshot({ path: screenshotPath, fullPage: true, animations: 'disabled', caret: 'hide', mask: [page.locator(sensitiveElements), ...scenario.mask.map(selector => page.locator(selector))], maskColor: '#000000', timeout: config.timeoutMs });
          capture.screenshotPath = screenshotPath;
        }
      } catch (error) {
        capture.error = safeError(stage, error);
      } finally {
        await context?.close().catch(() => undefined);
      }
    }
  } finally {
    await browser.close().catch(() => undefined);
  }
  return captures;
}
