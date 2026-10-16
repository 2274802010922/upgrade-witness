#!/usr/bin/env node
import { parseArgs } from 'node:util';
import { randomUUID } from 'node:crypto';
import { access, mkdir, readFile, writeFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { chromium } from 'playwright';
import { parseConfig } from './schema.js';
import { createRevisionWorkspaces, prepareAndStart } from './workspace.js';
import { captureScenarios } from './capture.js';
import { compareCaptures } from './compare.js';
import { collectEvidence, redact, sanitize, sha256, writeReport } from './report.js';
import type { RunReport, ScenarioCapture } from './types.js';

const VERSION='0.1.0-alpha.1';
const help=`Upgrade Witness ${VERSION}\n\nuw doctor\nuw verify --repo PATH --base REF --head REF --config FILE --out NEW_DIRECTORY --trust-project\n\nVerify two committed revisions of a trusted React/Vite npm project.\nProject installation/build scripts execute code. The explicit trust flag is required.\nExit: 0 completed without unresolved differences; 1 regression; 2 inconclusive; 64 usage.\n`;
async function exists(p:string):Promise<boolean>{try{await access(p);return true;}catch{return false;}}
async function main():Promise<number>{
  const {values,positionals}=parseArgs({options:{repo:{type:'string'},base:{type:'string'},head:{type:'string'},config:{type:'string'},out:{type:'string'},'trust-project':{type:'boolean'},help:{type:'boolean'},version:{type:'boolean'}},allowPositionals:true,strict:true});
  if(values.version){console.log(VERSION);return 0;}
  if(values.help||positionals.length===0){console.log(help);return 0;}
  if(positionals.length!==1) throw new Error('Exactly one command is required');
  if(positionals[0]==='doctor'){
    const browserPath=process.env.UW_BROWSER_EXECUTABLE||chromium.executablePath();
    const git=spawnSync('git',['--version'],{encoding:'utf8',windowsHide:true});
    const browserInstalled=await exists(browserPath);
    let browserReady=false,browserVersion='unavailable';
    try{const browser=await chromium.launch({executablePath:process.env.UW_BROWSER_EXECUTABLE||undefined,headless:true,timeout:10000});browserVersion=browser.version();await browser.close();browserReady=true;}catch{/* Surface readiness without exposing host paths or raw process logs. */}
    const ready=git.status===0&&browserReady;
    console.log(JSON.stringify({version:VERSION,node:process.version,platform:process.platform,git:git.status===0?git.stdout.trim():'unavailable',browserInstalled,browserReady,browserVersion,ready,next:ready?'Ready to verify trusted projects':'Install Git and run npx playwright install chromium (Linux may need --with-deps), or set UW_BROWSER_EXECUTABLE to a tested Chromium binary'},null,2));
    return ready?0:2;
  }
  if(positionals[0]!=='verify') throw new Error(`Unknown command ${positionals[0]}`);
  for(const key of ['repo','base','head','config','out'] as const) if(!values[key]) throw new Error(`Missing --${key}`);
  if(!values['trust-project']) throw new Error('Use --trust-project only for a project you authorize to install, build and run');
  const config=parseConfig(JSON.parse(await readFile(path.resolve(values.config!),'utf8')));
  const out=path.resolve(values.out!);
  if(await exists(out)) throw new Error('--out must be a new directory; existing runs are never overwritten');
  await mkdir(out,{recursive:true});
  const contract=JSON.stringify(config,null,2)+'\n';
  const digest=sha256(contract);
  // Keep exported expectations sanitized; the digest binds the original input supplied by the user.
  const reviewContract=JSON.stringify(sanitize(config),null,2)+'\n';
  await writeFile(path.join(out,'contract.json'),reviewContract);
  const report:RunReport={schemaVersion:'0.1',toolVersion:VERSION,runId:randomUUID(),createdAt:new Date().toISOString(),baseSha:'',headSha:'',contractSha256:digest,config,environment:{node:process.version,platform:process.platform,browser:'not started'},complete:false,exitCode:2,findings:[],evidence:[],coverage:{planned:2*config.scenarios.reduce((n,s)=>n+s.assertions.length,0),passed:0,failed:0,notRun:2*config.scenarios.reduce((n,s)=>n+s.assertions.length,0)},limitations:[
    'Alpha verifies only configured routes, viewports, actions and assertions; it does not prove whole-application correctness.',
    'Screenshot differences require review; a failed baseline or unstable capture cannot establish a regression.',
    'Use synthetic non-sensitive data. Input fields and configured masks are hidden in screenshots; arbitrary text/images may still contain private information. Review artifacts before sharing.',
    'Token redaction is best effort and does not identify every secret. Explicit assertion values may collect sensitive data.',
    'contractSha256 binds the normalized original configuration. The exported contract may be redacted; replay requires the original config.',
    'This CLI does not run a model or upload artifacts. A host agent may send selected data to its model provider.',
  ]};
  let workspace:Awaited<ReturnType<typeof createRevisionWorkspaces>>|undefined;
  let server:Awaited<ReturnType<typeof prepareAndStart>>|undefined;
  let base:ScenarioCapture[]=[],repeat:ScenarioCapture[]=[],head:ScenarioCapture[]=[],headRepeat:ScenarioCapture[]=[];
  let originPort:number|undefined;
  let cleanupError:string|undefined;
  try{
    workspace=await createRevisionWorkspaces(path.resolve(values.repo!),values.base!,values.head!,out);
    report.baseSha=workspace.baseSha;report.headSha=workspace.headSha;
    report.limitations.push(`Replay with the original repository/config: uw verify --repo <repository> --base ${workspace.baseSha} --head ${workspace.headSha} --config <original-config> --out <new-directory> --trust-project`);
    const locks={base:sha256(await readFile(path.join(workspace.baseDir,'package-lock.json'))),head:sha256(await readFile(path.join(workspace.headDir,'package-lock.json')))};
    await writeFile(path.join(out,'inputs.json'),JSON.stringify({baseSha:workspace.baseSha,headSha:workspace.headSha,lockfileSha256:locks,contractSha256:digest,node:process.version,platform:process.platform,arch:process.arch,playwright:'1.64.1'},null,2)+'\n');
    if(workspace.dirtyFiles.length) report.limitations.push(`${workspace.dirtyFiles.length} uncommitted source entries were excluded; the source checkout was left untouched.`);
    console.error('Capturing baseline (two runs)...');
    server=await prepareAndStart(workspace.baseDir,config);
    originPort=Number(new URL(server.url).port);
    base=await captureScenarios(server.url,config,path.join(out,'base'),'base');
    repeat=await captureScenarios(server.url,config,path.join(out,'base-repeat'),'base-repeat');
    await server.stop();server=undefined;
    console.error('Capturing candidate...');
    server=await prepareAndStart(workspace.headDir,config,{port:originPort});
    head=await captureScenarios(server.url,config,path.join(out,'head'),'head');
    headRepeat=await captureScenarios(server.url,config,path.join(out,'head-repeat'),'head-repeat');
    await server.stop();server=undefined;
    Object.assign(report,await compareCaptures(config,base,head,repeat,headRepeat));
  }catch(error){
    if(base.length||head.length) Object.assign(report,await compareCaptures(config,base,head,repeat,headRepeat));
    report.findings.push({id:'infrastructure',scenarioId:'run',kind:'inconclusive',observation:redact(error instanceof Error?error.message:'Execution failed'),evidenceIds:[]});
    report.complete=false;report.exitCode=report.findings.some(f=>f.kind==='regression')?1:2;
  }finally{
    if(server) try{await server.stop();}catch{cleanupError='A started server could not be confirmed stopped; inspect owned processes.';}
    if(workspace) try{await workspace.cleanup();}catch{cleanupError='Temporary worktree cleanup did not complete; inspect the run workspaces before manual cleanup.';}
  }
  if(cleanupError){report.limitations.push(cleanupError);report.complete=false;if(report.exitCode===0)report.exitCode=2;}
  report.environment.browser=base[0]?.browserVersion||head[0]?.browserVersion||'not started';
  report.evidence=await collectEvidence(out,{'base':base,'base-repeat':repeat,'head':head,'head-repeat':headRepeat});
  report.evidence.push({id:'contract',path:'contract.json',sha256:sha256(reviewContract),mediaType:'application/json'});
  if(await exists(path.join(out,'inputs.json'))) report.evidence.push({id:'inputs',path:'inputs.json',sha256:sha256(await readFile(path.join(out,'inputs.json'))),mediaType:'application/json'});
  await writeReport(report,out);
  console.log(JSON.stringify({exitCode:report.exitCode,complete:report.complete,findings:report.findings.length,report:path.join(out,'report.html')},null,2));
  return report.exitCode;
}
main().then(code=>{process.exitCode=code;}).catch(error=>{console.error(redact(error instanceof Error?error.message:String(error)));process.exitCode=64;});
