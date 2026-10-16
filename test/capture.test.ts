import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import { access, mkdtemp, readFile, rm } from 'node:fs/promises';
import { createServer } from 'node:http';
import { tmpdir } from 'node:os';
import { join, isAbsolute } from 'node:path';
import test from 'node:test';
import { chromium } from 'playwright';
import { PNG } from 'pngjs';
import { captureScenarios } from '../src/capture.js';
import { parseConfig } from '../src/schema.js';

const executable = process.env.UW_BROWSER_EXECUTABLE || chromium.executablePath();
const browserUnavailable = existsSync(executable) ? false : 'Install Playwright Chromium or set UW_BROWSER_EXECUTABLE to run real browser tests.';

const html = `<!doctype html><html><body style="margin:0">
<input id="password" type="password" value="synthetic-secret" style="position:absolute;left:10px;top:10px;width:100px;height:30px">
<div id="private" style="position:absolute;left:10px;top:60px;width:100px;height:30px;background:red">synthetic private</div>
<main style="margin-top:120px"><input id="entry"><button id="open">Open</button><span id="output"></span>
<div id="panel" style="display:none;color:green" data-state="closed">Panel</div><div id="hidden" hidden>Hidden control</div>
<span id="fresh"></span><span id="intent">after</span><span id="environment"></span></main>
<script>
document.querySelector('#fresh').textContent=localStorage.getItem('visited')?'reused':'fresh'; localStorage.setItem('visited','yes');
document.querySelector('#entry').addEventListener('keydown',event=>{if(event.key==='Enter') document.querySelector('#output').textContent=event.target.value});
document.querySelector('#open').addEventListener('click',()=>{const panel=document.querySelector('#panel');panel.style.display='block';panel.dataset.state='open'});
document.querySelector('#environment').textContent=[navigator.language,Intl.DateTimeFormat().resolvedOptions().timeZone,matchMedia('(prefers-reduced-motion: reduce)').matches,matchMedia('(prefers-color-scheme: light)').matches].join('|');
</script></body></html>`;

test('real Chromium captures actions, all assertion kinds, deterministic context, masking, and opt-out', { skip: browserUnavailable }, async t => {
  const dir = await mkdtemp(join(tmpdir(), 'uw-capture-'));
  t.after(() => rm(dir, { recursive: true, force: true }));
  const server = createServer((_request, response) => { response.writeHead(200, { 'Content-Type': 'text/html' }); response.end(html); });
  await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve));
  t.after(() => new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve())));
  const address = server.address();
  assert.ok(address && typeof address !== 'string');
  const config = parseConfig({ schemaVersion: 1, timeoutMs: 5000, scenarios: [
    { id: 'workflow', route: '/', viewport: { width: 640, height: 480 }, mask: ['#private'], actions: [{ type: 'fill', selector: '#entry', value: 'synthetic input' }, { type: 'press', selector: '#entry', value: 'Enter' }, { type: 'click', selector: '#open' }], assertions: [
      { id: 'output', selector: '#output', kind: 'text', expected: 'synthetic input' },
      { id: 'visible', selector: '#panel', kind: 'visible', expected: true },
      { id: 'state', selector: '#panel', kind: 'attribute', name: 'data-state', expected: 'open' },
      { id: 'color', selector: '#panel', kind: 'style', name: 'color', expected: 'rgb(0, 128, 0)' },
      { id: 'hidden', selector: '#hidden', kind: 'visible', expected: false },
      { id: 'environment', selector: '#environment', kind: 'text', expected: 'en-US|UTC|true|true' },
    ] },
    { id: 'fresh', route: '/', screenshot: false, assertions: [{ id: 'storage', selector: '#fresh', kind: 'text', expected: 'fresh' }] },
    { id: 'intended', route: '/', screenshot: false, assertions: [{ id: 'text', selector: '#intent', kind: 'text', expected: 'before' }] },
  ], intendedChanges: [{ scenarioId: 'intended', assertionId: 'text', expected: 'after', reason: 'The new copy is deliberately approved.' }] });
  const captures = await captureScenarios(`http://127.0.0.1:${address.port}`, config, dir, 'head');
  assert.equal(captures.length, 3);
  for (const capture of captures) {
    assert.equal(capture.error, undefined);
    assert.notEqual(capture.browserVersion, 'unavailable');
    for (const check of capture.checks) assert.equal(check.passed, true, `${capture.scenarioId}/${check.id}: ${check.error ?? check.actual}`);
  }
  assert.ok(captures[0].screenshotPath && isAbsolute(captures[0].screenshotPath));
  const image = PNG.sync.read(await readFile(captures[0].screenshotPath));
  const pixel = (x: number, y: number) => [...image.data.subarray((y * image.width + x) * 4, (y * image.width + x) * 4 + 4)];
  assert.deepEqual(pixel(40, 25), [0, 0, 0, 255], 'password masked before saved screenshot');
  assert.deepEqual(pixel(40, 75), [0, 0, 0, 255], 'user selector masked before saved screenshot');
  assert.equal(captures[1].screenshotPath, undefined);
  await assert.rejects(access(join(dir, 'fresh.png')));
});

test('navigation and selector failures are sanitized while missing elements are observations', { skip: browserUnavailable }, async t => {
  const dir = await mkdtemp(join(tmpdir(), 'uw-capture-errors-'));
  t.after(() => rm(dir, { recursive: true, force: true }));
  const server = createServer((request, response) => {
    response.writeHead(request.url?.startsWith('/missing') ? 404 : 200, { 'Content-Type': 'text/html' });
    response.end(html);
  });
  await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve));
  t.after(() => new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve())));
  const address = server.address();
  assert.ok(address && typeof address !== 'string');
  const config = parseConfig({ schemaVersion: 1, timeoutMs: 5000, scenarios: [
    { id: 'http', route: '/missing?token=DO_NOT_PRINT', screenshot: false, assertions: [{ id: 'text', selector: 'body', kind: 'text', expected: 'unavailable' }] },
    { id: 'selector', route: '/', screenshot: false, assertions: [{ id: 'bad', selector: ':DO_NOT_PRINT(', kind: 'text', expected: 'unavailable' }] },
    { id: 'missing-element', route: '/', screenshot: false, assertions: [{ id: 'missing', selector: '#absent', kind: 'visible', expected: true }, { id: 'absent-control', selector: '#absent', kind: 'visible', expected: false }] },
  ] });
  const captures = await captureScenarios(`http://127.0.0.1:${address.port}`, config, dir, 'base');
  assert.match(captures[0].error ?? '', /Page navigation/);
  assert.equal(captures[0].checks.length, 0);
  assert.match(captures[1].checks[0].error ?? '', /Assertion evaluation/);
  assert.equal(JSON.stringify(captures).includes('DO_NOT_PRINT'), false);
  assert.equal(captures[2].checks[0].actual, false);
  assert.equal(captures[2].checks[0].passed, false);
  assert.equal(captures[2].checks[0].error, undefined);
  assert.equal(captures[2].checks[1].passed, true);
});
