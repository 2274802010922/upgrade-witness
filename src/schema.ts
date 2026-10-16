import { z } from 'zod';
import type { WitnessConfig } from './types.js';

const id = z.string().regex(/^[a-zA-Z0-9][a-zA-Z0-9._-]{0,79}$/);
const assertion = z.object({
  id, selector: z.string().min(1).max(500),
  kind: z.enum(['visible', 'text', 'attribute', 'style']),
  name: z.string().min(1).optional(), expected: z.union([z.string(), z.boolean()]),
}).strict().superRefine((v, ctx) => {
  if (['attribute', 'style'].includes(v.kind) && !v.name) ctx.addIssue({code:'custom',message:'attribute/style assertions require name'});
  if (v.kind === 'visible' && typeof v.expected !== 'boolean') ctx.addIssue({code:'custom',message:'visible expects a boolean'});
  if (v.kind !== 'visible' && typeof v.expected !== 'string') ctx.addIssue({code:'custom',message:'text/attribute/style expect a string'});
});
const action = z.object({type:z.enum(['click','fill','press']),selector:z.string().min(1).max(500),value:z.string().optional()}).strict().superRefine((v,ctx)=>{
  if(v.type!=='click' && v.value===undefined) ctx.addIssue({code:'custom',message:'fill/press actions require value'});
});
const scenario = z.object({
  id, route:z.string().regex(/^\/(?!\/)/).refine(v=>!v.includes('\\'), 'Backslashes are not allowed in routes'),
  viewport:z.object({width:z.number().int().min(320).max(2560),height:z.number().int().min(200).max(2160)}).strict().default({width:1440,height:900}),
  actions:z.array(action).max(30).default([]), assertions:z.array(assertion).min(1).max(50),
  mask:z.array(z.string().min(1)).max(50).default([]), screenshot:z.boolean().default(true),
  maxDiffPixelRatio:z.number().min(0).max(1).default(0.001),
}).strict();
const schema = z.object({
  schemaVersion:z.literal(1),
  project:z.object({startScript:z.string().regex(/^[\w:.-]+$/).default('dev'),buildScript:z.string().regex(/^[\w:.-]+$/).optional()}).strict().default({startScript:'dev'}),
  scenarios:z.array(scenario).min(1).max(30),
  intendedChanges:z.array(z.object({scenarioId:id,assertionId:id,expected:z.union([z.string(),z.boolean()]),reason:z.string().min(10)}).strict()).default([]),
  timeoutMs:z.number().int().min(1000).max(120000).default(15000),
}).strict();

export function parseConfig(value: unknown): WitnessConfig {
  const config = schema.parse(value);
  const scenarios = new Map(config.scenarios.map(s=>[s.id,s]));
  if(scenarios.size!==config.scenarios.length) throw new Error('Scenario ids must be unique');
  for(const s of config.scenarios) if(new Set(s.assertions.map(a=>a.id)).size!==s.assertions.length) throw new Error(`Duplicate assertion in ${s.id}`);
  const intents = new Set<string>();
  for(const i of config.intendedChanges) {
    const a=scenarios.get(i.scenarioId)?.assertions.find(a=>a.id===i.assertionId);
    if(!a) throw new Error(`Unknown intended-change assertion: ${i.scenarioId}/${i.assertionId}`);
    if(typeof a.expected!==typeof i.expected) throw new Error('Intended-change expectation type must match original');
    const key=`${i.scenarioId}/${i.assertionId}`;
    if(intents.has(key)) throw new Error(`Duplicate intended change: ${key}`);
    intents.add(key);
  }
  return config;
}
