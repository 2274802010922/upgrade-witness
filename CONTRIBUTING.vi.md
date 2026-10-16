# Đóng góp cho Upgrade Witness

[English](CONTRIBUTING.md) · [Giới thiệu dự án](README.vi.md) · [MIT](LICENSE)

Upgrade Witness là alpha ban đầu. Chào đón đóng góp về ca migration tối thiểu, kỳ vọng kiểm tra, triển khai và tài liệu Anh–Việt. Setup bằng npm ci và npx playwright install chromium. Chạy npm run typecheck và npm test; chạy npm run demo khi sửa pipeline hoặc fixture. Xem README cho điều kiện nền tảng.

## Những đóng góp đầu tiên hữu ích

- Ví dụ React/Vite + npm nhỏ thể hiện hành vi đổi sau nâng Tailwind 3→4.
- Ca nâng cấp sạch không được báo regression, hoặc thay đổi có chủ đích với yêu cầu rõ.
- Assertion DOM/CSS hoặc thao tác phân biệt được hành vi đúng và sai.
- Giải thích rõ hơn, bổ sung giới hạn hoặc sửa tương đương ở cả hai ngôn ngữ.

Trao đổi trước về thay đổi lớn để maintainer xác nhận phạm vi. Có thể gửi trực tiếp bản sửa tài liệu nhỏ.

## Báo lỗi hoặc đóng góp fixture

Dùng mẫu issue đi kèm sau khi đưa chúng vào repo thật. Ghi phiên bản chính xác, hệ điều hành, revision trước/sau, lockfile npm, các bước, hành vi kỳ vọng, hành vi quan sát và bằng chứng tối thiểu đã loại dữ liệu nhạy cảm. Nói rõ lệnh nào đã thực sự chạy. Phân biệt giả thuyết với kết luận. Báo lỗi vẫn hữu ích khi chưa biết nguyên nhân.

Dùng dữ liệu giả. Không gửi credential, source riêng tư, cookie phiên đăng nhập hoặc ảnh có dữ liệu riêng của người dùng. Repo công khai không đồng nghĩa được đổi giấy phép nội dung của họ; khi nhập tài liệu/code, ghi nguồn, tác giả, giấy phép và giữ thông báo áp dụng. Ưu tiên ví dụ tối thiểu tự tạo.

## Pull request và kiểm chứng

Mỗi PR tập trung một kết quả. Giải thích hành vi trước/sau cụ thể và liên kết issue nếu có. Giữ ca lỗi gốc, ghi kiểm tra đã chạy, bỏ qua hoặc bị chặn. Thay ảnh kỳ vọng hoặc assertion cần lý do gắn với hành vi có chủ đích; không âm thầm chấp nhận baseline mới để làm lỗi biến mất.

Cập nhật mục EN/VI tương ứng cùng nhau. Giữ nguyên định danh kỹ thuật. Nếu chưa dịch chính xác được, đánh dấu phần cần maintainer hỗ trợ thay vì tuyên bố đã đồng bộ.

Tiêu đề và nội dung commit mới dùng tiếng Việt. Có thể giữ prefix Conventional Commits và tên kỹ thuật, ví dụ `fix: giữ nguyên data-variant khi nâng cấp`. Chấp nhận trao đổi issue/PR bằng tiếng Anh; maintainer có thể hỗ trợ câu chữ commit. Không viết lại lịch sử cũ chỉ để dịch.

Chấp nhận AI hỗ trợ. Nêu hỗ trợ AI đáng kể trong PR, hiểu thay đổi gửi lên và xác minh bằng chứng. Ghi công người thực sự đóng góp; không tạo co-author giả hoặc tuyên bố đã chạy kiểm tra mà agent chưa chạy.

## Giấy phép và review

Khi chủ động gửi đóng góp để đưa vào dự án, bạn đồng ý cho phân phối đóng góp đó theo [MIT của dự án](LICENSE), đồng thời giữ bản quyền của mình. Chỉ gửi nội dung bạn có quyền đóng góp. Thành phần bên thứ ba giữ điều khoản tương ứng. Chính sách khởi đầu này không yêu cầu CLA riêng hay quy trình ký DCO.

Trao đổi tôn trọng và cụ thể. Thảo luận code và bằng chứng, không công kích cá nhân. Maintainer có thể đóng nội dung trùng hoặc ngoài phạm vi và giải thích lý do. Thời gian review tự nguyện không phải cam kết dịch vụ.

Với phát hiện liên quan bảo mật, tránh đăng công khai chi tiết khai thác hoặc credential. Dùng kênh báo riêng tư trong [SECURITY.md](SECURITY.md).
