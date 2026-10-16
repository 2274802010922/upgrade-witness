<p align="center"><a href="README.md">English</a> · <a href="README.vi.md">Tiếng Việt</a></p>

# Upgrade Witness

**Nâng cấp thư viện. Thấy rõ điều gì thay đổi.**

Skill AI và CLI chạy tại máy để đối chiếu hai revision frontend đã commit, tạo bằng chứng có thể review. Bắt đầu với **React + Vite, npm, Tailwind 3 → 4 và Chromium**.

![Minh họa ý tưởng nguyên gốc: bản gốc, bản nâng cấp và bằng chứng.](assets/hero-vi.svg)

[![CI](https://github.com/2274802010922/upgrade-witness/actions/workflows/ci.yml/badge.svg)](https://github.com/2274802010922/upgrade-witness/actions/workflows/ci.yml)
**Alpha ban đầu** · [MIT](LICENSE) · [Roadmap](docs/ROADMAP.vi.md) · [Đóng góp](CONTRIBUTING.vi.md)

## Chạy một ca trước–sau thật

Cần Git, Node.js 24 và npm. Kiểm tra phát triển/runtime hiện dùng Node 24.16.0. Setup sẽ tải dependency và browser; chỉ chạy project bạn tin cậy.

```sh
git clone https://github.com/2274802010922/upgrade-witness.git
cd upgrade-witness
npm ci
npx playwright install chromium
npm run demo
```

Trên Linux, dùng `npx playwright install --with-deps chromium` nếu thiếu thư viện hệ thống cho browser.

Demo tạo repo Git riêng, commit hai trạng thái Tailwind 3 và Tailwind 4 của ví dụ tổng hợp tự tạo, chạy bộ kiểm chứng và kiểm tra kết quả. Ví dụ cố ý tạo hai thay đổi: mất cấu hình cột và đổi `data-variant`. **Phát hiện regression là kết quả demo mong đợi.** Script demo chỉ báo thành công khi có đúng các phát hiện cần thiết và in đường dẫn báo cáo. Xem [ví dụ và nguồn gốc](examples/tailwind-migration/README.md).

Đây là ví dụ chạy được, chưa phải benchmark độ chính xác trên dự án thực tế hoặc đánh giá công cụ nâng cấp Tailwind hiện tại.

[Xem báo cáo và quan sát từ lượt demo thực tế](docs/cases/tailwind-demo.md).

## Dùng cùng agent

Repo có sẵn [skill](skills/upgrade-witness/SKILL.md). Đưa thư mục `skills/upgrade-witness` vào cơ chế skill mà agent của bạn hỗ trợ, hoặc yêu cầu agent đọc file đó. Host cần truy cập checkout này, shell và dependency của CLI. Skill không cài model, đăng ký MCP server hay tự cấp quyền mới.

> Dùng Upgrade Witness kiểm chứng lần nâng Tailwind này. Giữ hành vi điều hướng và bố cục đã xác định. Đưa bằng chứng chạy lại được và nêu phần chưa kiểm chứng.

Agent có thể hỗ trợ migration được yêu cầu bằng công cụ chính thức. CLI **kiểm chứng các revision đã commit**; nó không tự migration hay merge ứng dụng.

## Kiểm tra project của bạn

Bắt đầu từ [cấu hình mẫu](examples/tailwind-migration/witness.config.json). Xác định kỳ vọng từ bản gốc trước khi xem kết quả bản mới. Cả hai revision cần lockfile npm và script dev tương thích Vite.

```sh
npm run build
node dist/src/cli.js doctor
node dist/src/cli.js verify --repo /path/to/trusted-app --base BASE_COMMIT --head HEAD_COMMIT --config /path/to/witness.config.json --out .uw/my-first-run --trust-project
```

Đặt đường dẫn Windows có dấu cách trong dấu ngoặc kép. Thư mục `--out` phải chưa tồn tại. Cờ xác nhận tin cậy thể hiện bạn cho phép chạy script install, build và start của repo đã chọn.

Bộ kiểm chứng chốt SHA bất biến, tạo worktree riêng, chụp các tình huống trên bản gốc/bản mới và xuất `report.html`, `report.md`, `report.json`, ảnh cùng hash bằng chứng. Thay đổi chưa commit được loại khỏi đầu vào và giữ nguyên. [Cấu hình và lệnh](docs/USAGE.md)

| Mã thoát | Ý nghĩa |
|---|---|
| `0` | Đã chạy đủ kiểm tra, không có regression hoặc khác biệt chưa giải thích |
| `1` | Có ít nhất một assertion regression; vẫn có thể còn phát hiện chưa giải quyết |
| `2` | Chưa kết luận, baseline không ổn định, lỗi môi trường hoặc ảnh khác cần review |
| `64` | Cách gọi lệnh hoặc cấu hình không hợp lệ |

Đọc cả `complete` và độ bao phủ, không chỉ mã thoát. Ảnh khác nhau là mục cần review, không tự động là bug. Chấp nhận thay đổi một thuộc tính không đồng nghĩa chấp nhận mọi khác biệt ảnh của trang.

## Đã triển khai

- Worktree theo revision, cài bằng lockfile, kiểm tra server sẵn sàng và giới hạn thời gian tiến trình.
- Click, nhập dữ liệu, nhấn phím theo cấu hình; assertion visibility, text, attribute và computed style.
- Kiểm tra baseline ổn định, quyết định thay đổi có chủ đích, so sánh ảnh và hash bằng chứng.
- Báo cáo HTML/Markdown/JSON tách quan sát khỏi kết luận nguyên nhân.
- Demo migration tổng hợp tự tạo, kiểm thử tập trung và cấu hình CI Windows/Linux.

## Giới hạn cần biết

Alpha chỉ kiểm tra trang, trạng thái và assertion đã khai báo; không chứng minh toàn ứng dụng tương đương. Clock, số ngẫu nhiên và dữ liệu mạng trực tiếp chưa được tự đóng băng; hãy dùng fixture ổn định. Chưa hỗ trợ Next.js/SSR, monorepo, pnpm, MCP hoặc tự động hóa production.

Dùng dữ liệu giả không nhạy cảm. Ảnh che input và selector được cấu hình; chữ/ảnh khác trên trang và giá trị assertion được thu thập vẫn có thể chứa dữ liệu riêng. Lọc token chỉ là biện pháp hỗ trợ. Xem lại artifact trước khi chia sẻ. CLI không gọi model hoặc upload; agent host có thể gửi ngữ cảnh tới nhà cung cấp model. Worktree không phải sandbox bảo mật; server cố ý tách daemon có thể thoát cơ chế dọn dẹp.

Đánh giá trên bộ giữ kín độc lập và pilot bên ngoài **chưa hoàn thành**. [Quy trình đánh giá](docs/BENCHMARK.vi.md) chứa mục tiêu, không phải thành tích đã đo. [Trạng thái triển khai](docs/STATUS.md)

## Cùng phát triển

Đóng góp ca migration nhỏ, ca đối chứng sạch, assertion, cải thiện báo cáo hoặc bản dịch. Đọc [CONTRIBUTING](CONTRIBUTING.vi.md). Fixture tự tạo, bằng chứng trung thực và PR nhỏ đặc biệt hữu ích.

[Kiến trúc](docs/ARCHITECTURE.vi.md) · [Backlog song ngữ](docs/BACKLOG.md) · [Kế hoạch GitHub](docs/GITHUB.vi.md) · [Bảo mật](SECURITY.md)

[MIT](LICENSE) · Copyright (c) 2026 Upgrade Witness contributors. Thành phần bên thứ ba giữ điều khoản tương ứng.
