import { test } from 'node:test';
import assert from 'node:assert/strict';
import path from 'node:path';
import { tmpdir } from 'node:os';
import { mkdtemp,readFile,rm } from 'node:fs/promises';
import { parseConfig } from '../src/schema.js';
import { escapeHtml,redact,relativeArtifact,renderHtml,writeReport,sha256 } from '../src/report.js';
import type { RunReport } from '../src/types.js';

const input=()=>({schemaVersion:1,scenarios:[{id:'main',route:'/',assertions:[{id:'title',selector:'h1',kind:'text',expected:'Hello'}]}]});
test('configuration rejects ambiguous assertions, duplicate ids and unknown intents',()=>{
  assert.equal(parseConfig(input()).scenarios[0].viewport.width,1440);
  assert.throws(()=>parseConfig({...input(),scenarios:[...input().scenarios,...input().scenarios]}),/unique/);
  assert.throws(()=>parseConfig({...input(),intendedChanges:[{scenarioId:'main',assertionId:'missing',expected:'New',reason:'Intentionally changed'}]}),/Unknown/);
  assert.throws(()=>parseConfig({...input(),scenarios:[{id:'main',route:'/',assertions:[{id:'a',selector:'h1',kind:'style',expected:'red'}]}]}));
  assert.throws(()=>parseConfig({...input(),scenarios:[{...input().scenarios[0],route:'//evil.example'}]}));
  assert.throws(()=>parseConfig({...input(),extra:'silently ignored'}));
});
test('artifact boundaries reject sibling and parent traversal',()=>{
  const root=path.resolve('run');
  assert.equal(relativeArtifact(root,path.join(root,'base','main.png')),'base/main.png');
  assert.throws(()=>relativeArtifact(root,path.join(root,'..','outside.png')),/within/);
});
test('sanitized reports escape HTML and remove known tokens without claiming universal redaction',async()=>{
  const root=await mkdtemp(path.join(tmpdir(),'uw-report-'));
  try{
    const token='ghp_123456789012345678901234567890';
    const report:RunReport={schemaVersion:'0.1',toolVersion:'test',runId:'run',createdAt:'2026-10-16T00:00:00.000Z',baseSha:'abc',headSha:'def',contractSha256:sha256('x'),config:parseConfig(input()),environment:{node:'test',platform:'test',browser:'test'},complete:false,exitCode:2,findings:[{id:'f',scenarioId:'main',kind:'needs-review',observation:`<script>alert(1)</script> ${token}`,evidenceIds:[]}],evidence:[{id:'unsafe',path:'javascript:alert(1)',sha256:'hash',mediaType:'image/png'}],coverage:{planned:2,passed:0,failed:0,notRun:2},limitations:['Coverage is bounded']};
    const html=renderHtml(report);
    assert.ok(html.includes('&lt;script&gt;'));
    assert.ok(!html.includes('<script>'));
    assert.ok(!html.includes(token));
    assert.ok(html.includes('#invalid-artifact'));
    assert.ok(!html.includes('src="javascript:'));
    report.evidence=[];
    await writeReport(report,root);
    const parsed=JSON.parse(await readFile(path.join(root,'report.json'),'utf8'));
    assert.ok(parsed.findings[0].observation.includes('[REDACTED]'));
    assert.equal(parsed.exitCode,2);
  }finally{await rm(root,{recursive:true,force:true});}
});
test('redaction covers bearer and URL query tokens; escaping keeps Unicode',()=>{
  assert.equal(redact('Bearer abc.def-secret'),'Bearer [REDACTED]');
  assert.equal(redact('https://x.test/?token=secret&ok=yes'),'https://x.test/?token=[REDACTED]&ok=yes');
  assert.equal(escapeHtml('Nâng cấp <x>'),'Nâng cấp &lt;x&gt;');
});
