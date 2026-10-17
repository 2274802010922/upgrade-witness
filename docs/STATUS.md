# Implementation status / Trạng thái triển khai

Updated / Cập nhật: 17 October 2026 · `0.2.0-alpha.1` · Current source: native Rust.

| Area / Phần | Status / Trạng thái |
|---|---|
| Public GitHub repo, MIT, bilingual README and contribution workflow | Created / Đã tạo |
| Native Rust CLI, strict config, pinned Git worktrees and bounded target process lifecycle | Implemented locally / Đã triển khai local |
| Rust Chromium CDP capture and repeated baseline/candidate comparison | Implemented locally / Đã triển khai local |
| Zero-area visibility and top-layer screenshot-mask regressions found by review | Fixed with browser tests / Đã sửa và có test browser |
| Rust HTML/Markdown/JSON reports and evidence hashes | Implemented / Đã triển khai |
| Synthetic React/Vite/Tailwind 3→4 positive-control demo | 14 checkpoints: 12 pass, 2 intended assertion regressions, 0 not run / Đã chạy đủ 14 checkpoint |
| Rust format, clippy and local test suite | Passed on Windows with Chrome 155; 29 tests / Đạt trên Windows với Chrome 155; 29 test |
| Rust CI matrix (Ubuntu 24.04 and Windows 2025) | Configured. GitHub Actions on the current source commit is authoritative / Đã cấu hình; xem Actions của commit hiện tại |
| Independent 20-fixture held-out evaluation | Not performed / Chưa thực hiện |
| Five external-user pilots and 14-day reuse observation | Not performed / Chưa thực hiện |
| Public Rust beta / MCP / Next.js / additional package managers | Deferred / Chưa triển khai |

The initial TypeScript architecture/roadmap/GitHub materials are dated design snapshots, not the current implementation spec. See [the native Rust update](RUST-MIGRATION.md) and current README/source. Benchmark targets are unmeasured. The demo contains intentionally authored faults and proves only this bounded path, not general-world accuracy.

Roadmap work remaining: recruit willing pilot users, independently construct the held-out benchmark fixtures, run the paired evaluation and record results before deciding on a beta. Do not count missing participants as successful pilot outcomes.

Tài liệu thiết kế lưu đề xuất TypeScript ban đầu để làm lịch sử; phần đó đã được thay bằng alpha Rust. Mục tiêu benchmark chưa được đo. Demo chỉ minh họa luồng kiểm chứng. Công việc còn lại là pilot có người thật, bộ fixture giữ kín độc lập và đánh giá đối chiếu trước khi quyết định beta.
