> Bản thiết kế ban đầu ngày 16/10/2026. Xem [README](../README.vi.md) và [STATUS](STATUS.md) cho trạng thái triển khai hiện tại; tài liệu này không phải nhật ký kết quả.

# Kế hoạch trình bày và ra mắt trên GitHub

[English](GITHUB.en.md) · [Roadmap](ROADMAP.vi.md) · [README mẫu](../README.vi.md)

**Bản kế hoạch ngày 16/10/2026.** Đây là kế hoạch nội dung và phát hành. README đi kèm phản ánh đúng giai đoạn lập kế hoạch; chưa có tuyên bố rằng package hay benchmark đã hoạt động.

## 1. Học gì từ REA

[README của REA](https://github.com/morluto/rea) dẫn người đọc từ lời giới thiệu, chuyển ngôn ngữ đến cài đặt, yêu cầu mẫu, quy trình, khả năng, ví dụ thực tế và đóng góp. Điểm đáng học là khoảng cách ngắn từ hiểu công cụ đến xem bằng chứng. Đây là quan sát bố cục, chưa chứng minh nguyên nhân tăng trưởng.

Đề xuất cho mình: một vấn đề migration cụ thể, một ví dụ tái hiện được, một hành động tiếp theo rõ ràng. Dùng nội dung, hình ảnh, bản ghi và fixture tự tạo. Không mượn thương hiệu, ảnh chụp hay độ phổ biến của REA làm uy tín cho dự án.

## 2. Định vị và màn hình đầu tiên

Tên repo dự kiến: **upgrade-witness**. Kiểm tra khả dụng trước khi đăng ký tên repo/package; kế hoạch này chưa giữ tên nào.

Mô tả tiếng Anh đề xuất:

> Evidence for frontend upgrades. An AI skill and CLI for checking behavior after dependency migrations. Starting with React + Tailwind.

Mô tả tiếng Việt đề xuất:

> Bằng chứng cho mỗi lần nâng cấp frontend. Skill AI và CLI kiểm tra hành vi sau migration, bắt đầu với React + Tailwind.

Màn hình đầu có chuyển ngôn ngữ, tên, lời giới thiệu ngắn, hình hero, trạng thái **Đang lập kế hoạch / Chưa phát hành**, hai liên kết **Xem roadmap** và **Đóng góp ca migration**. Giữ nội dung chính bên ngoài hình để đọc được trên điện thoại và bằng công nghệ hỗ trợ. Hero đi kèm là hình minh họa ý tưởng, không phải ảnh sản phẩm.

Dùng hướng Swiss/editorial sáng đã chọn: nền trắng ấm, chữ đậm màu mực, điểm nhấn xanh/chanh vừa phải, căn lề rõ và khoảng trống thoáng. Kiểm tra ở chiều rộng điện thoại và giao diện sáng/tối của GitHub. SVG sáng cần có nền riêng.

## 3. Thứ tự nội dung README

| Vị trí | Câu hỏi người đọc | Nội dung |
|---|---|---|
| 1 | Đây là gì? | Lời hứa, phạm vi, giai đoạn, chuyển ngôn ngữ |
| 2 | Vì sao cần? | Migration build thành công nhưng thay đổi luồng quan trọng |
| 3 | Đã dùng được chưa? | Bảng hiện có/dự kiến và trạng thái cài đặt |
| 4 | Hỏi agent thế nào? | Một yêu cầu thực tế |
| 5 | Nhận được gì? | Phát hiện, bằng chứng, cách chạy lại, phần chưa kiểm tra |
| 6 | Có hiệu quả không? | Case thực tế khi tái hiện được; liên kết phương pháp đo |
| 7 | Hỗ trợ gì? | Ma trận nhỏ với phiên bản đã kiểm tra |
| 8 | Đóng góp thế nào? | Việc nhỏ, hướng dẫn đóng góp, MIT |

Khi có alpha, thay lời mời xem kế hoạch bằng lệnh cài đã thử từ checkout mới. Gắn phiên bản và điều kiện chạy. Chỉ đưa một đường cài chính lên README, chuyển lựa chọn khác vào tài liệu. Chưa có package thì không đăng lệnh `npx` dự đoán.

## 4. Kịch bản demo 55 giây

Chỉ ghi hình khi lát cắt W2 tái hiện được lỗi thật. Chọn fixture được phép công bố; lưu commit, lockfile, phiên bản trình duyệt/runtime và báo cáo.

| Thời gian | Màn hình | Chú thích |
|---|---|---|
| 0–7 giây | UI trước/sau migration | “Build thành công. Menu đã thay đổi.” |
| 7–15 giây | Yêu cầu gửi agent | “Kiểm tra các luồng cần giữ nguyên.” |
| 15–28 giây | Hai trạng thái cùng viewport | “Một thay đổi quan sát được, kèm bằng chứng.” |
| 28–40 giây | Phát hiện nối với assertion DOM/CSS và diff | “Xem thay đổi và chạy lại.” |
| 40–49 giây | Bản sửa và lượt kiểm tra mới | “Cùng phép kiểm tra, giờ đã đạt.” |
| 49–55 giây | Báo cáo phạm vi kiểm tra | “Xem bằng chứng cùng giới hạn.” |

Đây là kịch bản dự kiến. Thay từng câu bằng điều fixture thực sự chứng minh. Ghi rõ khi cắt thời gian chờ. Có ảnh đại diện, phụ đề, bản chép lời, báo cáo tải về và cách tái hiện. Tránh đặt ảnh động lặp vô hạn trước đoạn nội dung hữu ích đầu tiên.

## 5. Hình ảnh, About và badge

Thiết kế social card riêng 1280 × 640: nền đặc, tên, lời giới thiệu ngắn và hình trước → sau → bằng chứng. GitHub nhận PNG/JPG/GIF dưới 1 MB; kích thước hiển thị tốt được khuyến nghị là 1280 × 640, tối thiểu khuyến nghị 640 × 320. SVG trong README và ảnh tải lên social preview là hai tài sản riêng. [Hướng dẫn GitHub](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/customizing-your-repositorys-social-media-preview)

Topics đề xuất ban đầu: `agent-skills`, `ai-agents`, `developer-tools`, `tailwindcss`, `react`, `migration`, `playwright`, `visual-regression`. Thêm `cli` khi chạy được; chỉ thêm topic client sau kiểm thử tích hợp. Chưa triển khai MCP thì chưa gắn `mcp`.

Dùng tối đa bốn badge hữu ích. MIT liên kết đến giấy phép; trạng thái có thể là chữ tĩnh. CI phải trỏ đến workflow thật, release phải trỏ đến tag thật. Chưa xác minh thì không gắn số lượt tải, coverage, hiệu năng, độ tương thích hay sao. Số benchmark phải dẫn đến dữ liệu và baseline cụ thể.

## 6. Cấu trúc repo mục tiêu

Đây là **cây thư mục tương lai**, không phải danh sách file đã có trong bộ kế hoạch:

```text
README.md / README.vi.md
LICENSE / CONTRIBUTING.md / CONTRIBUTING.vi.md
SECURITY.md / CODE_OF_CONDUCT.md / CHANGELOG.md
.github/ISSUE_TEMPLATE/ / pull_request_template.md / workflows/
skills/upgrade-witness/SKILL.md / references/
packages/cli/                 # module lõi nằm trong cùng package
fixtures/ / benchmarks/ / tests/
docs/en/ / docs/vi/ / docs/cases/
assets/hero.svg / hero-vi.svg / social-card.png / demo-poster.png
```

Khi tạo repo, chuyển kế hoạch vào `docs/en/` và `docs/vi/`, sửa liên kết `../ROADMAP.*`, `../ARCHITECTURE.*`, `../BENCHMARK.*` trong README mẫu. Không đưa toàn bộ browser artifact lớn vào lịch sử Git; chọn báo cáo đã loại dữ liệu nhạy cảm làm release asset hoặc đính kèm case study.

## 7. Cộng đồng và hai ngôn ngữ

Tiếng Anh là trang mặc định; tiếng Việt có đường đọc tài liệu đầy đủ. Cập nhật mục tính năng/trạng thái của hai bản trong cùng PR, giữ nguyên định danh kỹ thuật, kiểm tra lệnh một lần cho cả hai. Ghi ngày rà soát bản dịch. Nếu chậm cập nhật, chỉ rõ mục cũ và dẫn đến bản mới.

Tạo issue form cho lỗi tái hiện, fixture migration và đề xuất tính năng. Việc đầu tiên cho cộng tác viên: fixture nhỏ có giấy phép, assertion thất bại, chỉnh cách diễn đạt báo cáo hoặc đồng bộ bản dịch. Commit mới viết tiêu đề và nội dung tiếng Việt theo quy ước của người dùng. Khi có mã chạy, hướng dẫn đóng góp phải có cách kiểm tra cục bộ. MIT đã được chọn; dòng bản quyền là `Copyright (c) 2026 Upgrade Witness contributors`. Giữ thông báo bên thứ ba riêng khi áp dụng.

## 8. Ra mắt và kiểm tra trước phát hành

W1–2: chuẩn bị cách giới thiệu và mời ít reviewer phù hợp khi được phép. W3–4: chia sẻ alpha chạy được. W5: công bố phương pháp và toàn bộ kết quả benchmark, gồm thất bại. W6: bắt đầu pilot và xin phép dùng từng case. W7: chuẩn bị release candidate và nội dung ra mắt. W8: ghi nhận việc dùng lại trong 14 ngày, phát hành beta khi đạt cổng chất lượng. Kết quả dùng lại mang tính mô tả, kèm số người có cơ hội dùng lần hai; thiếu cơ hội migration tiếp theo không phải lý do loại release.

- [ ] Checkout mới cài và chạy ví dụ thành công trên nền tảng đã công bố.
- [ ] EN/VI thống nhất liên kết, trạng thái, hỗ trợ, giấy phép và ảnh.
- [ ] Demo từ lượt chạy có tag; không bịa người dùng, kết quả hoặc lời chứng thực.
- [ ] Release notes có phạm vi, giới hạn và lỗi đã biết.
- [ ] Maintainer có thể phân loại lỗi tái hiện và yêu cầu ngoài phạm vi.

Kênh có thể dùng: Discussion của repo, bài viết kỹ thuật về migration, cộng đồng React/Tailwind/agent cho phép giới thiệu dự án. Mỗi bài dựa vào một case thật, công khai vai trò tác giả, không quảng cáo vào issue không liên quan, không mua hoặc trao đổi sao. Đọc quy định hiện hành trước khi đăng. Kế hoạch này không tự cho phép xuất bản ra ngoài. Đo lượt chạy đầu thành công, báo cáo hữu ích, bản sửa được chấp nhận, người thử quay lại và chất lượng đóng góp; sao là tín hiệu khám phá phụ.
