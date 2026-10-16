import { createHash } from 'node:crypto';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import type { Evidence, RunReport, ScenarioCapture } from './types.js';

export const sha256 = (value: string | Buffer): string => createHash('sha256').update(value).digest('hex');
export function redact(value: string): string {
  return value
    .replace(/\bBearer\s+[A-Za-z0-9._~+\/-]+=*/gi, 'Bearer [REDACTED]')
    .replace(/\b(?:gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9_-]{20,})\b/g, '[REDACTED]')
    .replace(/([?&](?:token|access_token|api_key|secret|password)=)[^&#\s]+/gi, '$1[REDACTED]');
}
export function sanitize<T>(value:T):T {
  if(typeof value==='string') return redact(value) as T;
  if(Array.isArray(value)) return value.map(sanitize) as T;
  if(value && typeof value==='object') return Object.fromEntries(Object.entries(value).map(([k,v])=>[k,sanitize(v)])) as T;
  return value;
}
export function escapeHtml(value:unknown):string {
  return String(value??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]!));
}
function cell(value:unknown):string { return String(value??'—').replace(/\|/g,'\\|').replace(/[\r\n]+/g,' '); }
function artifactHref(value:string):string {
  return /^[A-Za-z0-9._/-]+$/.test(value)&&!value.startsWith('/')&&!value.split('/').includes('..')?encodeURI(value):'#invalid-artifact';
}
export function relativeArtifact(root:string,filename:string):string {
  const relative=path.relative(path.resolve(root),path.resolve(filename));
  if(!relative || relative==='..' || relative.startsWith(`..${path.sep}`) || path.isAbsolute(relative)) throw new Error('Evidence path must stay within the report directory');
  return relative.split(path.sep).join('/');
}
export async function collectEvidence(root:string, groups:Record<string,ScenarioCapture[]>):Promise<Evidence[]> {
  const evidence:Evidence[]=[];
  for(const [label,captures] of Object.entries(groups)) {
    for(const c of captures) {
      if(c.screenshotPath) {
        const relative=relativeArtifact(root,c.screenshotPath);
        evidence.push({id:`${label}-${c.scenarioId}`,path:relative,sha256:sha256(await readFile(c.screenshotPath)),mediaType:'image/png'});
      }
    }
    const filename=`${label}/checks.json`;
    await mkdir(path.join(root,label),{recursive:true});
    const content=JSON.stringify(sanitize(captures.map(c=>({...c,screenshotPath:c.screenshotPath?relativeArtifact(root,c.screenshotPath):undefined}))),null,2)+'\n';
    await writeFile(path.join(root,filename),content);
    evidence.push({id:`${label}-checks`,path:filename,sha256:sha256(content),mediaType:'application/json'});
  }
  return evidence;
}

export function renderMarkdown(raw:RunReport):string {
  const r=sanitize(raw);
  return `# Upgrade Witness report\n\nRun: \`${r.runId}\` · Exit: **${r.exitCode}** · Complete: **${r.complete}**\n\n`+
    `Base: \`${r.baseSha||'unresolved'}\`\n\nHead: \`${r.headSha||'unresolved'}\`\n\n`+
    `Coverage: ${r.coverage.passed} passed, ${r.coverage.failed} failed, ${r.coverage.notRun} not run / ${r.coverage.planned} planned checkpoints.\n\n`+
    `| Kind | Scenario | Assertion | Observation |\n|---|---|---|---|\n`+
    r.findings.map(f=>`| ${cell(f.kind)} | ${cell(f.scenarioId)} | ${cell(f.assertionId)} | ${cell(f.observation)} |`).join('\n')+
    `\n\n## Evidence\n\n`+r.evidence.map(e=>`- [${cell(e.id)}](${artifactHref(e.path)}) — SHA-256 \`${e.sha256}\``).join('\n')+
    `\n\n## Limits\n\n`+r.limitations.map(l=>`- ${l}`).join('\n')+'\n';
}

export function renderHtml(raw:RunReport):string {
  const r=sanitize(raw), h=escapeHtml;
  const images=r.evidence.filter(e=>e.mediaType==='image/png'&&!e.id.startsWith('base-repeat-')&&!e.id.startsWith('head-repeat-')).sort((a,b)=>a.id.replace(/^(base|head)-/,'').localeCompare(b.id.replace(/^(base|head)-/,''))||a.id.localeCompare(b.id));
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src 'self' data:; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'"><title>Upgrade Witness report</title><style>
*{box-sizing:border-box}body{overflow-wrap:anywhere;margin:0;background:#f7f6f1;color:#091426;font:16px/1.6 system-ui,sans-serif}main{max-width:1200px;margin:auto;padding:40px 24px}header{border-bottom:1px solid #ccd2cc;padding-bottom:24px}h1{font-size:clamp(32px,5vw,56px);line-height:1.1;letter-spacing:-1.5px;margin:12px 0}h2{margin:36px 0 12px;font-size:24px}code{overflow-wrap:anywhere;font:13px/1.6 ui-monospace,monospace}a{color:#1749bf}a:focus-visible{outline:3px solid #2456e6;outline-offset:4px}.eyebrow{font-weight:700;letter-spacing:2px;font-size:13px}.status{display:inline-block;background:#e4e9de;border:1px solid #ccd2cc;padding:5px 12px;border-radius:4px;margin:6px 8px 6px 0}.grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(min(100%,330px),1fr));gap:16px}.grid>*{min-width:0}.finding,figure{margin:0;background:white;border:1px solid #ccd2cc;border-radius:6px;padding:20px;overflow-wrap:anywhere}.kind{font-size:13px;font-weight:700;color:#1749bf}.regression{border-left:5px solid #b92a31}.needs-review,.inconclusive,.baseline-issue{border-left:5px solid #a36600}.intended-change{border-left:5px solid #6c8e15}.finding h3{margin:8px 0}.muted{color:#485366}img{width:100%;height:auto;border:1px solid #dde1dc}figcaption{margin-bottom:12px;font-weight:600}pre{white-space:pre-wrap;overflow-wrap:anywhere;background:#f1f3ed;padding:12px;font-size:13px}li{margin:8px 0}.proof{font-size:12px}footer{margin-top:40px;padding-top:16px;border-top:1px solid #ccd2cc}
</style></head><body><main><header><div class="eyebrow">UPGRADE WITNESS / ALPHA</div><h1>See what changed.</h1><p class="muted">Observed evidence for the scenarios in this run. This is not a whole-application correctness guarantee.</p><span class="status">Exit ${r.exitCode}</span><span class="status">${r.complete?'Complete':'Incomplete'}</span><span class="status">${r.coverage.passed}/${r.coverage.planned} checkpoints passed</span><p><strong>Base</strong> <code>${h(r.baseSha||'unresolved')}</code><br><strong>Head</strong> <code>${h(r.headSha||'unresolved')}</code></p></header>
<h2>Findings</h2><div class="grid">${r.findings.length?r.findings.map(f=>`<article class="finding ${h(f.kind)}"><span class="kind">${h(f.kind.toUpperCase())}</span><h3>${h(f.scenarioId)}${f.assertionId?' / '+h(f.assertionId):''}</h3><p>${h(f.observation)}</p>${f.base!==undefined?`<pre>Base: ${h(f.base)}\nHead: ${h(f.head)}\nExpected: ${h(f.expected)}</pre>`:''}${f.reason?`<p>${h(f.reason)}</p>`:''}<p class="proof">Evidence: ${f.evidenceIds.map(h).join(', ')||'See run checks'}</p></article>`).join(''):'<article class="finding"><h3>No regression observed</h3><p>This statement applies only to the completed declared checks.</p></article>'}</div>
<h2>Before and after</h2><div class="grid">${images.map(e=>`<figure><figcaption>${h(e.id)}</figcaption><a href="${h(artifactHref(e.path))}"><img src="${h(artifactHref(e.path))}" alt="Captured ${h(e.id)}" loading="lazy"></a><p class="proof"><code>${h(e.sha256)}</code></p></figure>`).join('')}</div>
<h2>Coverage and limits</h2><p>${r.coverage.passed} passed · ${r.coverage.failed} failed · ${r.coverage.notRun} not run · ${r.coverage.planned} planned checkpoints.</p><ul>${r.limitations.map(l=>`<li>${h(l)}</li>`).join('')}</ul><footer><p><a href="report.json">JSON report</a> · <a href="report.md">Markdown report</a></p><p class="muted"><code>${h(r.runId)}</code> · ${h(r.createdAt)} · Node ${h(r.environment.node)} · ${h(r.environment.platform)} · Chromium ${h(r.environment.browser)}</p></footer></main></body></html>`;
}

export async function writeReport(raw:RunReport,outDir:string):Promise<void> {
  const report=sanitize(raw);
  for(const e of report.evidence) relativeArtifact(outDir,path.resolve(outDir,e.path));
  await mkdir(outDir,{recursive:true});
  await writeFile(path.join(outDir,'report.json'),JSON.stringify(report,null,2)+'\n');
  await writeFile(path.join(outDir,'report.md'),renderMarkdown(report));
  await writeFile(path.join(outDir,'report.html'),renderHtml(report));
}
