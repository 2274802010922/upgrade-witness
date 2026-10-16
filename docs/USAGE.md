# Configuration and commands / Cấu hình và lệnh

[English README](../README.md) · [README tiếng Việt](../README.vi.md)

## Start / Bắt đầu

Build with `npm ci`, `npx playwright install chromium`, and `npm run build`. Run `node dist/src/cli.js doctor`; it launches and closes Chromium to check readiness. On Linux, install required browser libraries with `npx playwright install --with-deps chromium`.

Cài dependency/browser và build theo các lệnh trên. `doctor` thực sự mở rồi đóng Chromium, không chỉ kiểm file tồn tại. Chỉ chạy script của repo bạn tin cậy; việc cài đặt và build có thực thi code.

```sh
node dist/src/cli.js verify --repo /path/to/app --base BASE_SHA --head HEAD_SHA --config /path/to/witness.config.json --out .uw/run-001 --trust-project
```

Replace placeholders and quote Windows paths with spaces. Both revisions must be committed and contain `package.json` and `package-lock.json`. The output directory must be new. Detached temporary worktrees are removed afterward; source changes are preserved. / Thay giá trị mẫu, đặt đường dẫn có dấu cách trong ngoặc kép. Hai revision phải có commit và manifest/lockfile npm. Thư mục đầu ra phải mới. Worktree tạm được dọn sau lượt chạy; giữ thay đổi ở repo gốc.

## Minimal contract / Hợp đồng tối thiểu

```json
{
  "schemaVersion": 1,
  "project": { "startScript": "dev", "buildScript": "build" },
  "timeoutMs": 15000,
  "scenarios": [{
    "id": "menu-mobile",
    "route": "/",
    "viewport": { "width": 390, "height": 844 },
    "actions": [{ "type": "click", "selector": "[data-testid=menu-toggle]" }],
    "assertions": [{
      "id": "menu-open", "selector": "[data-testid=menu-panel]",
      "kind": "visible", "expected": true
    }],
    "mask": ["[data-private]"],
    "screenshot": true,
    "maxDiffPixelRatio": 0.001
  }],
  "intendedChanges": []
}
```

| Field / Trường | Behavior / Hành vi |
|---|---|
| `project.startScript` | npm script receiving Vite flags `--host`, `--port`, `--strictPort`; default `dev` / Script tương thích cờ Vite |
| `project.buildScript` | Optional build before serving / Build tùy chọn trước chạy |
| `timeoutMs` | 1,000–120,000ms per operation/assertion; install capped at 5min, build at 3min / Giới hạn từng thao tác, không phải tổng lượt |
| `route` | Same-origin path starting with `/` / Đường dẫn cùng origin |
| `actions` | `click`, `fill`, `press`; latter two require `value` / Hai loại sau cần giá trị |
| `assertions` | `visible` boolean; `text`, `attribute`, `style` string; last two require `name` / `style` là computed CSS |
| `mask` | Extra selectors hidden before screenshot encoding / Selector che trước khi ghi ảnh |
| `screenshot` | Default true; false permits assertion-only checks / Có thể chỉ kiểm assertion |
| `maxDiffPixelRatio` | Visual review threshold, not semantic assertion tolerance / Ngưỡng review ảnh, không nới assertion |

Require one matching element per assertion. Missing elements yield false/null observations; ambiguous selectors are inconclusive. Scenarios and assertions need unique IDs. Unknown config keys are rejected. / Mỗi assertion cần một element phù hợp. Element thiếu tạo quan sát false/null; selector nhiều kết quả khiến chưa kết luận. ID phải duy nhất; từ chối key cấu hình lạ.

An intended change is explicit: `{"scenarioId":"menu-mobile","assertionId":"menu-open","expected":false,"reason":"The reviewed design intentionally removes this menu."}`. The baseline still must meet its original expectation. The candidate must meet the new expectation. This decision never waives unrelated visual differences. / Thay đổi có chủ đích vẫn yêu cầu baseline đúng kỳ vọng cũ và candidate đúng kỳ vọng mới; không miễn kiểm tra các khác biệt ảnh khác.

## Evidence and replay / Bằng chứng và chạy lại

Both baseline and candidate are captured twice with fresh browser contexts and the same local origin. Locale, timezone, color scheme, reduced motion and device scale are fixed. Clock/random/network data are not automatically frozen. / Mỗi bên chạy hai lần trong context mới và cùng origin local. Cố định locale, timezone, theme, reduced motion và scale; chưa tự cố định clock/random/network.

The run contains reports, `contract.json`, `inputs.json`, per-side checks and screenshots. `inputs.json` records commit identities, lockfile hashes, runtime and browser-tool version. Evidence hashes cover emitted bytes. The contract digest binds normalized input; the exported contract may be redacted, so keep the original configuration for replay. / Giữ cấu hình gốc để chạy lại; bản xuất có thể đã che dữ liệu. Hash xác định byte, không chứng minh nguyên nhân lỗi.

Reports return `0` clean, `1` regression, `2` unresolved/incomplete, `64` invalid invocation/configuration. A regression can coexist with `complete:false`. Coverage counts assertion checkpoints on both revisions; repeat captures are stability checks, not additional coverage. / Một regression có thể đi cùng kết quả chưa hoàn chỉnh. Lượt lặp kiểm độ ổn định, không tăng giả độ bao phủ.

## Privacy and troubleshooting / Dữ liệu và xử lý lỗi

The runner masks inputs, textareas, editable regions and `[data-uw-sensitive]` in screenshots, plus your selectors. Assertions can still record sensitive values. Token filtering is best effort; arbitrary text/images are not automatically private. Use synthetic data and review every export. / Dùng dữ liệu giả và xem lại phần xuất; che ảnh không che mọi giá trị assertion.

- Missing browser: rerun browser installation and `doctor`. / Thiếu browser: cài và kiểm lại.
- Install/build failure: run the exact project script in a trusted disposable checkout to inspect its output; raw process logs are intentionally excluded from reports. / Chạy riêng script để xem lỗi, báo cáo không chứa raw log.
- Baseline issue: fix the scenario/environment or document the existing bug; do not relabel it as a new regression. / Sửa tình huống/môi trường hoặc ghi lỗi cũ.
- Visual review: inspect images and refine a semantic requirement; do not raise tolerance simply to hide a defect. / Xem ảnh và làm rõ yêu cầu, không tăng ngưỡng để giấu lỗi.

`UW_BROWSER_EXECUTABLE` may select an installed Chromium-compatible binary. Record and compare the same binary version; this is an override, not a compatibility guarantee. / Có thể chọn binary khác nhưng phải ghi phiên bản; không mặc nhiên được hỗ trợ.
