# Bản cập nhật chuyển sang Rust

[README](../README.vi.md) · [English](RUST-MIGRATION.md) · [Trạng thái](STATUS.md)

Cập nhật ngày 17/10/2026. Alpha TypeScript ban đầu đã được thay thế bằng CLI và phần lõi Rust native trong v0.2.0. Code cũ chỉ còn trong lịch sử Git; không được cài hoặc gọi bởi CLI hiện tại. Commit mới giữ nguyên lịch sử trước đó.

## Kiến trúc hiện tại

```text
Agent AI — đọc skill/reference Markdown
             │ chạy lệnh `uw verify`
CLI Rust native: clap / serde / anyhow
 ├─ Git: worktree tách biệt, SHA không đổi
 ├─ std::process: npm chỉ phục vụ ứng dụng đích
 ├─ Rust CDP: browser Chrome context mới, thao tác và capture
 ├─ Rust: so ảnh, JSON, Markdown và báo cáo HTML
 └─ sha2: digest cấu hình và artifact

Fixture React/Vite tự tạo — ứng dụng mục tiêu có npm/Node riêng
```

CLI, kiểm tra cấu hình, quản lý Git/tiến trình, tự động hóa trình duyệt, so sánh, báo cáo và test được viết bằng Rust. Skill và tài liệu vẫn là Markdown; biểu mẫu/YAML và asset giữ định dạng tương ứng. JavaScript cùng lockfile npm chỉ nằm trong `examples/` vì đó là ứng dụng frontend cần kiểm chứng. Phần lõi không dùng Node, TypeScript hay Playwright.

## Lý do và giới hạn

Theo yêu cầu dùng Rust cho dự án, `headless_chrome` điều khiển Chrome/Chromium cài trên máy qua Chrome DevTools Protocol ngay trong tiến trình Rust. Người dùng cần Rust, Git và Chromium; Node/npm chỉ cần khi ứng dụng đích có dùng chúng. CLI không tải browser tự động. `doctor` mở browser thật để kiểm tra có thể khởi chạy hay không.

Các thao tác DOM dùng CDP cùng biểu thức JavaScript ngắn được thực thi ngay trong trang để đọc selector/style và che vùng nhạy cảm. Không có Node runtime, Playwright hay harness JavaScript trong sản phẩm. Ứng dụng đích là thành phần riêng, các script build/install/start chỉ chạy khi đã xác nhận `--trust-project`.

Ngưỡng ảnh đánh dấu pixel đổi khi một kênh RGBA lệch hơn 25/255 rồi so tỷ lệ với `maxDiffPixelRatio`. Đây là heuristic xác định đơn giản, không cam kết chính xác thị giác tổng quát. Không có thông tin HTTP chắc chắn thì báo chưa kết luận. Clock, ngẫu nhiên, mạng trực tiếp và font chưa tự đóng băng.

Ảnh/chỉ số assertion có thể giữ dữ liệu nhạy cảm. Trước khi capture, tool ẩn selector mặc định/tự chọn rồi tô đen vùng ảnh trong RAM trước khi ghi file. Dùng dữ liệu giả và xem lại report. Lọc token không nhận biết mọi bí mật. CLI không upload hay gọi model, nhưng agent host tuân chính sách nhà cung cấp của nó.

## Build và chạy lại

Cài Rust stable và linker theo hệ điều hành:

```sh
cargo build --locked
cargo run --locked -- doctor
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo run --locked -- demo --out .uw/rust-demo
```

`doctor` kiểm Git và khởi chạy/đóng Chrome. Test browser cần Chromium; test worktree và server fixture cần Git, Node vì chúng kiểm một ứng dụng mục tiêu bằng npm. Bản thân CLI được build và chạy bằng Rust. Demo tạo repo tạm trong thư mục output mới, ghi commit synthetic với identity được đặt tên tường minh và giữ report. Không trỏ demo vào repo người dùng.

Các file `scripts/demo.mjs`, `package.json`, CLI TypeScript và test TS đã bị gỡ khỏi source hiện hành. Fixture React/Vite vẫn còn tại `examples/tailwind-migration/`. Bản v0.1.0-alpha.1 chỉ xuất hiện trong lịch sử Git; chưa có package npm công khai.

## Kiểm chứng còn lại

Fixture positive control chạy trên Windows có 14 checkpoint: 12 đạt, hai assertion lỗi cố ý, không bỏ sót checkpoint. Hai test browser riêng kiểm phần tử 0×0 và input nằm trong modal top layer. Những test này chỉ kiểm tình huống cụ thể, không đo độ chính xác tổng quát.

Roadmap ban đầu vẫn giữ nguyên cổng đánh giá giữ kín 20 fixture và năm pilot tự nguyện có theo dõi 14 ngày trước khi quyết định beta. Hai việc này chưa làm. Chạy CI Rust trên Ubuntu và Windows cho commit Rust; kết quả CI TypeScript cũ không chứng minh thay đổi này.
