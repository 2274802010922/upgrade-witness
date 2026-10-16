> Initial design snapshot, 16 October 2026. See [README](../README.md) and [STATUS](STATUS.md) for current implementation status; this document is not an execution log.

# Backlog — English / Tiếng Việt

[Roadmap VI](ROADMAP.vi.md) · [Roadmap EN](ROADMAP.en.md)

These are proposed GitHub issues, not issues already opened. / Đây là danh sách issue đề xuất, chưa được tạo trên GitHub.

All tasks start **Not started / Chưa bắt đầu**. The completed planning documents and assets are inputs to these tasks, not evidence that product implementation has begun. Maintainer owns priorities/review; AI assists implementation. Estimates are included in the roadmap's weekly budget, not additional hours. Each row is one issue; split only if it cannot be reviewed independently.

| ID | Week · priority | Issue title / Tiêu đề | Depends on | Definition of done / Điều kiện hoàn thành |
|---|---|---|---|---|
| UW-01 | W1 · P0 | Validate migration pain / Kiểm chứng nhu cầu migration | — | 20 deduplicated issues, ≥3 repos, five consented conversations; discovery gate recorded / Ghi nguồn, phản hồi và quyết định tiếp tục |
| UW-02 | W1 · P0 | Freeze scope and naming / Chốt phạm vi và tên | UW-01 | Name availability checked; one stack, runtime pair and agent host chosen / Không công bố tương thích chưa thử |
| UW-03 | W1 · P0 | Prepare MIT contribution repo / Chuẩn bị repo MIT để cộng tác | UW-02 | Actual repo identity selected, LICENSE/CONTRIBUTING/templates added, links fixed / Repo công khai chỉ khi được yêu cầu xuất bản |
| UW-04 | W1 · P0 | Define fixture rights and ground truth / Xác định quyền sử dụng và đáp án fixture | UW-01 | Provenance registry, dev/holdout split, reviewer access defined / Agent không thấy đáp án giữ kín |
| UW-05 | W2 · P0 | Preflight one npm/Vite project / Kiểm tra điều kiện chạy | UW-02 | Runtime, lockfile, browser and start failures diagnosed explicitly / Lỗi setup không thành pass |
| UW-06 | W2 · P0 | Isolate revision workspaces / Tách bản làm việc theo revision | UW-05 | Two pinned SHAs; no dirty-tree changes; owned cleanup only / Không reset hoặc stash âm thầm |
| UW-07 | W2 · P0 | Replay one real migration case / Chạy lại một ca migration thật | UW-04,06 | One failure plus clean control, frozen scenario, shareable evidence / Dev khác chạy lại được |
| UW-08 | W3 · P0 | Freeze scenario contract / Cố định hợp đồng kiểm chứng | UW-07 | Versioned contract, hash, intended changes, coverage denominator / Không sửa oracle theo candidate |
| UW-09 | W3 · P0 | Stabilize browser capture / Ổn định dữ liệu browser | UW-08 | Same environment, readiness, fonts/data/clock rules, unstable-base detection / Ghi rõ vùng không ổn định |
| UW-10 | W3 · P0 | Compare DOM/CSS and interactions / Đối chiếu DOM/CSS và tương tác | UW-09 | Known positive/negative cases; pixels alone need review / Kết luận có assertion hỗ trợ |
| UW-11 | W4 · P0 | Render evidence and exit outcomes / Xuất báo cáo và mã kết quả | UW-10 | JSON/MD/HTML; facts separated from cause; incomplete visible / Có đường dẫn và lệnh chạy lại |
| UW-12 | W4 · P0 | Protect artifacts and processes / Bảo vệ artifact và tiến trình | UW-06,11 | Redaction before model/export, HTML escaping, timeout/cleanup tests / Không phát tán raw evidence nhạy cảm |
| UW-13 | W4 · P0 | Author and trial the skill / Viết và thử skill | UW-11,12 | Concise SKILL.md, focused references, implicit routing checked, independent task trial / Không sao chép hướng dẫn chung |
| UW-14 | W4 · P0 | Package alpha and test platforms / Đóng gói alpha và kiểm nền tảng | UW-13 | Archive/install/uninstall tested; Linux and Windows; 10 dev fixtures / Cài từ artifact thay vì chỉ source tree |
| UW-15 | W5 · P0 | Freeze and score held-out evaluation / Chốt và chấm đánh giá giữ kín | UW-04,14 | Same-agent A/B, fixed budgets, complete results and gates / Giữ cả timeout và thất bại |
| UW-16 | W6 · P0 | Run five consented pilots / Tổ chức năm lượt dùng thử tự nguyện | UW-15 | Three repos; setup/support/usefulness logs; 14-day follow-up scheduled by agreement / Không tự gửi tin nhắn |
| UW-17 | W7 · P0 | Resolve pilot blockers / Sửa lỗi cản trở pilot | UW-16 | Prioritize reproducible core failures, rerun affected checks / Không mở rộng framework để né lỗi |
| UW-18 | W7 · P1 | Record real demo and case studies / Ghi demo và case study thật | UW-17 | One 55s walkthrough, three reproducible examples, consent and transcripts / Không dùng concept như screenshot thật |
| UW-19 | W7 · P0 | Prepare bilingual release candidate / Chuẩn bị bản ứng viên Anh–Việt | UW-17 | Docs parity, real support matrix, packaged replay, notices, security reporting route / Badge chỉ lấy từ trạng thái thật |
| UW-20 | W8 · P0 | Decide beta and publish release / Quyết định và phát hành beta | UW-15,18,19 | Current quality gates, pilot blockers resolved, documented reuse observation, release decision / Công bố khi người dùng yêu cầu |
| UW-21 | Post-beta · P1 | Review first 30 days / Đánh giá 30 ngày đầu | UW-20 | Setup, useful reports, repeat opportunities, support and quality trends / Chốt sửa sâu hoặc thêm một adapter |
| UW-22 | Later · P2 | Evaluate expansion / Đánh giá mở rộng | UW-21 | Demand-backed decision for pnpm/Next.js/MCP, each separately scoped / Không cam kết mọi thứ cùng lúc |

## Issue body template / Mẫu nội dung issue

```text
User outcome / Kết quả cho người dùng:
Supported scenario / Tình huống thuộc phạm vi:
Dependencies / Phụ thuộc:
Acceptance evidence / Bằng chứng nghiệm thu:
Tests or manual checks / Kiểm tra cần chạy:
Out of scope / Ngoài phạm vi:
Estimate and owner / Ước lượng và phụ trách:
```

Use P0 for the core release path, P1 for important supporting work, P2 for later decisions. / P0 là việc bắt buộc trên đường phát hành, P1 hỗ trợ quan trọng, P2 để quyết định sau.

## First five work sessions / Năm buổi làm việc đầu tiên

1. **Scope:** review this plan, choose first host/runtime and verify name availability. / Chốt phạm vi, host/runtime đầu tiên và kiểm tra tên.
2. **Demand:** build the first ten rows of the issue registry; draft interview questions. / Ghi mười issue đầu và câu hỏi phỏng vấn.
3. **Fixture:** independently recreate one licensed, minimal Tailwind failure and one clean control. / Dựng một ca lỗi và ca đối chứng sạch có nguồn gốc rõ.
4. **Contract:** specify the scenario and expected outcome before writing comparison logic. / Ghi tình huống và kết quả kỳ vọng trước logic đối chiếu.
5. **Repository:** use the preview assets and MIT files to prepare the actual repo structure; replace planning-stage links deliberately. / Dùng asset và MIT để chuẩn bị repo thật, sửa liên kết theo cấu trúc mới.
