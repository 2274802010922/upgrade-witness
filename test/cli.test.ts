import {test} from 'node:test';
import assert from 'node:assert/strict';
import {execFileSync,spawnSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
const cli=fileURLToPath(new URL('../src/cli.js',import.meta.url));
test('packaged command exposes help and version without modifying a project',()=>{
  assert.match(execFileSync(process.execPath,[cli,'--help'],{encoding:'utf8'}),/trust-project/);
  assert.equal(execFileSync(process.execPath,[cli,'--version'],{encoding:'utf8'}).trim(),'0.1.0-alpha.1');
});
test('invalid CLI usage and missing trust return 64 before project execution',()=>{
  const missing=spawnSync(process.execPath,[cli,'verify'],{encoding:'utf8'});
  assert.equal(missing.status,64);
  assert.match(missing.stderr,/Missing --repo/);
  const untrusted=spawnSync(process.execPath,[cli,'verify','--repo','unused','--base','main','--head','main','--config','unused','--out','unused'],{encoding:'utf8'});
  assert.equal(untrusted.status,64);
  assert.match(untrusted.stderr,/trust-project/);
});
