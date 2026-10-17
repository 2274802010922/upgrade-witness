# Đóng góp cho Upgrade Witness

[English](CONTRIBUTING.md) · [Giới thiệu dự án](README.vi.md) · [MIT](LICENSE)

Upgrade Witness v0.2.0-alpha.1 là CLI native Rust kèm skill AI. Đóng góp hữu ích gồm ca migration tối thiểu, kỳ vọng kiểm tra, triển khai Rust và tài liệu Anh–Việt tương đương.

## Thiết lập và kiểm tra thay đổi

Cài Rust stable hiện hành, linker/công cụ build của nền tảng, Git và Chrome/Chromium. Node.js/npm chỉ cần cho ứng dụng frontend được kiểm tra và fixture test/demo có chạy chúng. Phần lõi dùng browser driver Rust; không cần cài Playwright. Xem [USAGE](docs/USAGE.md) nếu cần chỉ định trình duyệt.

```sh
cargo build --locked
cargo run --locked -- doctor
cargo fmt --all -- --check
cargo test --locked
```

Bộ test đầy đủ có trình duyệt thật và fixture được phục vụ bằng npm. Chỉ compile thành công chưa chứng minh các kiểm tra đó đạt. Chạy `cargo run --locked -- demo` khi sửa capture/comparison, vòng đời workspace hoặc fixture migration. Xem cả báo cáo và mã thoát demo. Demo cố ý tạo assertion regression.

Giữ `Cargo.lock` trong quản lý phiên bản và cập nhật cùng thay đổi dependency. Dùng `cargo build --locked --release` để kiểm build phát hành. Ghi lệnh đã chạy, bỏ qua hoặc bị chặn; không quảng bá nền tảng/phiên bản chưa thử.

## Những đóng góp đầu tiên hữu ích

- Ví dụ React/Vite + npm nhỏ tự tạo, thể hiện thay đổi sau nâng Tailwind 3→4.
- Ca đối chứng sạch hoặc thay đổi có chủ đích với yêu cầu hành vi rõ.
- Assertion dùng CSS selector hoặc thao tác phân biệt hành vi đúng và sai.
- Bản sửa runner/báo cáo Rust có lỗi tái hiện được.
- Giới hạn còn thiếu hoặc cải thiện tương ứng Anh–Việt.

Trao đổi trước khi thay đổi lớn về phạm vi. Có thể đề xuất trực tiếp bản sửa nhỏ. Alpha hiện bao phủ một ứng dụng npm và Chromium; thêm framework hoặc package manager cần fixture và bằng chứng nghiệm thu riêng.

## Báo lỗi và fixture

Dùng [mẫu issue của repo](https://github.com/2274802010922/upgrade-witness/issues/new/choose). Ghi phiên bản công cụ/trình duyệt, OS, commit trước/sau, npm lockfile của ứng dụng, các bước, hành vi mong đợi/thực tế và bằng chứng tối thiểu đã lọc dữ liệu. Ghi lệnh chính xác và `doctor` có đạt không. Đánh dấu nguyên nhân nghi vấn là giả thuyết.

Dùng dữ liệu giả. Không gửi credential, source riêng, cookie hay ảnh riêng tư. Source công khai chưa mặc nhiên được đổi giấy phép: ghi nguồn, tác giả, giấy phép và thông báo giữ lại khi nhập nội dung. Ưu tiên ví dụ tối thiểu tự tạo.

## Pull request

Mỗi PR tập trung một kết quả. Mô tả hành vi trước/sau và giữ ca lỗi. Thay kỳ vọng hoặc ngưỡng ảnh cần lý do từ hành vi có chủ đích; không đổi baseline chỉ để giấu lỗi. So ảnh theo ngưỡng Rust trong [USAGE](docs/USAGE.md), không giả định pixelmatch cho kết quả giống hệt.

Cập nhật mục EN/VI tương ứng cùng lúc, giữ nguyên định danh kỹ thuật. Nêu rõ nếu cần hỗ trợ dịch. Tiêu đề và nội dung commit mới viết tiếng Việt; có thể giữ prefix Conventional Commits và tên kỹ thuật, ví dụ `fix: giữ nguyên data-variant khi nâng cấp`. Chào đón thảo luận issue/PR bằng tiếng Anh; maintainer có thể hỗ trợ câu chữ commit. Không viết lại lịch sử cũ chỉ để dịch.

Chấp nhận AI hỗ trợ. Công khai hỗ trợ AI đáng kể trong PR, hiểu thay đổi và xác minh bằng chứng. Ghi công người thực sự đóng góp; không bịa đồng tác giả hoặc kiểm tra.

## Giấy phép và review

Đóng góp chủ động gửi để tích hợp phải có thể phân phối theo [MIT](LICENSE) của dự án; người đóng góp vẫn giữ bản quyền của mình. Chỉ gửi nội dung bạn có quyền đóng góp. Thành phần bên thứ ba giữ điều khoản riêng. Dự án không yêu cầu quy trình ký CLA hoặc DCO riêng.

Thảo luận tôn trọng và cụ thể. Maintainer có thể đóng việc trùng/ngoài phạm vi kèm lý do; thời gian review tình nguyện không phải cam kết dịch vụ. Với vấn đề bảo mật, dùng [SECURITY.md](SECURITY.md), không công khai credential hay chi tiết khai thác.
