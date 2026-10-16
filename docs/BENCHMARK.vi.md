> Bản thiết kế ban đầu ngày 16/10/2026. Xem [README](../README.vi.md) và [STATUS](STATUS.md) cho trạng thái triển khai hiện tại; tài liệu này không phải nhật ký kết quả.

# Upgrade Witness — kế hoạch bằng chứng và kiểm chứng

[English](BENCHMARK.en.md) · Bản kế hoạch ngày 16/10/2026

**Trạng thái: quy trình đề xuất; chưa có kết quả benchmark hay thử nghiệm người dùng.** Phạm vi là skill AI cùng CLI nhỏ cho React/Vite, npm và Tailwind v3→v4. Cần đo khả năng giúp agent giữ hành vi giao diện và tạo bằng chứng chạy lại được. Kết quả không chứng minh toàn ứng dụng hoàn toàn đúng.

## Sổ bằng chứng

Đây là tín hiệu định tính, không phải khảo sát đại diện GitHub hay ước lượng quy mô thị trường. Trạng thái được kiểm tra ngày 16/10/2026; mỗi bản benchmark phải lưu liên kết và ngày tra cứu.

| Nguồn | Ngày và trạng thái đã kiểm tra | Điều nguồn hỗ trợ | Giới hạn |
|---|---|---|---|
| [Tailwind #20435](https://github.com/tailwindlabs/tailwindcss/issues/20435) | Mở 27/08/2026; đang Open | Công cụ nâng cấp có thể sửa giá trị `variant` trong biểu thức điều kiện. Prop có kiểu có thể bị typecheck bắt; data attribute có thể lọt qua. | Báo cáo của người dùng; phải tự tái hiện trên fixture. |
| [Tailwind #19555](https://github.com/tailwindlabs/tailwindcss/issues/19555) | Mở 12/01/2026; đang Open | Chuyển cấu hình `maxWidth` tùy chỉnh có thể ảnh hưởng bố cục cột. | Trường hợp cấu hình cụ thể, không phải mọi migration. |
| [MCP servers #4856](https://github.com/modelcontextprotocol/servers/issues/4856) | Mở 26/09/2026; đóng completed 07/10/2026 | Kế hoạch đổi SDK dùng test ghi nhận hành vi hiện tại để tránh thay đổi ngoài ý muốn. | Tín hiệu ở quy trình lân cận, ngoài MVP frontend; không phải lỗi chưa giải quyết. |

[Codemod AI](https://codemod.com/blog/npx-codemod-ai) đã kết hợp skill migration, công cụ và test. [Playwright](https://playwright.dev/docs/test-snapshots) đã có so sánh ảnh và hướng dẫn về ảnh hưởng của môi trường chạy. Tích hợp các khả năng đó. Giả thuyết cần kiểm tra là chọn tình huống, giải thích và chạy lại bằng chứng tốt hơn; không tuyên bố phát minh AI migration hay visual testing.

## Hai mươi fixture và đáp án độc lập

Tạo 10 fixture phát triển và 10 fixture giữ kín để đánh giá cuối. Mỗi nhóm có:

| Loại | Số lượng mỗi nhóm | Kết quả cần đạt |
|---|---:|---|
| Regression thật | 5 | Tìm một lỗi chính mỗi fixture, chứng minh bằng trạng thái trước/sau |
| Migration không lỗi | 2 | Không báo regression; nêu phạm vi đã kiểm tra |
| Thay đổi có chủ ý | 2 | Giải thích thay đổi đã được ghi nhận/chấp thuận, không báo lỗi |
| Hỏng hạ tầng | 1 | Kết luận chưa đủ dữ liệu, chỉ rõ điều kiện hỏng và cách khôi phục |

Nhóm lỗi dự kiến: cấu hình tác động bố cục, sửa sai prop/data attribute, tràn ngang responsive, trạng thái focus/tương tác và ngữ nghĩa CSS utility. Tự viết ví dụ tối thiểu; issue công khai là nguồn ý tưởng, không mặc nhiên cho phép sao chép repo. Có trường hợp static checks vẫn vượt qua. Ca hạ tầng dùng lỗi server hoặc dependency được kiểm soát; không đổi nhãn lỗi ứng dụng sau khi chạy thất bại.

Mỗi fixture lưu commit trước/sau, npm lockfile, yêu cầu route/trạng thái, thay đổi có chủ ý, assertion chuẩn và bảng đáp án riêng. Người thứ hai xác nhận hành vi đúng. Hai nhóm dùng component và cơ chế khác nhau; đổi tên một template chưa tạo được tập kiểm tra độc lập. Đặt nhãn lỗi, patch đáp án, test chấm điểm, lịch sử Git tiết lộ lỗi và hồ sơ chấm ngoài **toàn bộ phạm vi filesystem, công cụ và mạng mà agent có thể truy cập**, trong môi trường chấm riêng có kiểm soát quyền. Chỉ chuyển chúng sang thư mục cạnh checkout là chưa đủ. Yêu cầu hành vi mong muốn được cung cấp cho cả hai nhánh thử nghiệm.

## Thử nghiệm đối chiếu

Mỗi fixture chạy ba lần cho mỗi nhánh: **20 × 3 × 2 = 120 lượt**, gồm 60 lượt phát triển và 60 lượt đánh giá giữ kín.

- **A:** agent có shell, trình duyệt, Playwright, công cụ migration chính thức và prompt nhiệm vụ thống nhất.
- **B:** cùng cấu hình và công cụ nền, bổ sung skill cùng CLI Upgrade Witness.

Thiết kế đo đóng góp của cả gói, chưa tách riêng tác động của lời hướng dẫn. Thử nghiệm tiếp theo có thể cấp CLI cho cả hai nhánh và chỉ thay skill. Ghi phiên bản model/agent, thiết lập, công cụ, prompt, hash gói và dữ liệu tính phí. Mỗi lượt có phiên và checkout mới; đảo thứ tự đối chiếu ngẫu nhiên, cùng chính sách cache, không mang trí nhớ giữa lượt. Cho truy cập cùng tài liệu đã chốt, chặn đáp án benchmark và lời giải issue khi chấm.

Giới hạn đề xuất mỗi lượt: 20 phút và tổng 40.000 token đầu vào/đầu ra, tính cả nội dung skill và ngữ cảnh công cụ. Hai nhánh có cùng giới hạn; lưu cả lượt hết thời gian. Hiệu chỉnh bằng nhóm phát triển trước khi chốt, không đổi sau khi thấy kết quả giữ kín. Thử nghiệm chính kiểm tra hai trạng thái trước/sau được cung cấp. Chấm việc agent tự migration và sửa lỗi riêng để bản sửa tình cờ không che một lỗi chưa phát hiện.

Chốt OS image, trình duyệt, font, viewport, múi giờ, locale, dữ liệu mẫu và xử lý animation. Dùng 375/768/1024/1440px theo yêu cầu từng fixture. Định trước route, trạng thái và thao tác bắt buộc. Hai bản chạy trong môi trường tương đương; không tự động chấp nhận ảnh mới làm chuẩn.

## Cách chấm và điều kiện đề xuất ở tuần 5

Người chấm không biết nhánh thử nghiệm khi có thể; đối chiếu phát hiện với đáp án, gộp báo cáo trùng và kiểm tra bằng chứng. Khác ảnh chưa đủ xác nhận regression. Công bố từng lượt và kết quả đa số ba lượt của từng fixture; các lượt lặp không phải repo độc lập.

| Chỉ số | Cách tính | Mục tiêu giữ kín; chưa phải kết quả |
|---|---|---|
| Recall lỗi | Lỗi chính được xác nhận / 5 lỗi đã biết; lượt chưa kết luận vẫn giữ trong mẫu số | Bắt được ≥4/5 fixture ở đa số lượt |
| Precision phát hiện | Phát hiện regression đúng / tất cả phát hiện regression; không có phát hiện thì chưa xác định | ≥80%; công bố tử/mẫu từng lượt và tổng |
| Báo sai ở ca sạch | Fixture sạch có báo regression sai / 2 | 0/2 theo đa số |
| Thay đổi có chủ ý | Fixture được giải thích đúng là có chủ ý / 2 | 2/2 theo đa số |
| Xử lý hạ tầng | Lượt hạ tầng được ghi đúng là chưa đủ dữ liệu / 3 | 3/3; tuyệt đối không báo pass |
| Phạm vi thực sự kiểm tra | Điểm bắt buộc đã chạy ở cả hai bản / điểm đã định trong 9 fixture chạy được | ≥90%; liệt kê phần bỏ sót |
| Chạy lại bằng chứng | Gói bằng chứng tái hiện độc lập được / gói đã nộp | ≥90%, cần `ceil(0,9 × n)` lần thành công; nêu n |
| Giá trị tăng thêm | Đối chiếu phát hiện theo đa số và phân loại ca sạch với A | Cải thiện ≥1 fixture có lỗi, không giảm chất lượng phân loại ca sạch |

Công bố thêm số fixture/lượt chưa kết luận theo nguyên nhân, thời gian tới bằng chứng đầu tiên, tổng thời gian, phí token, phút hỗ trợ và lượt thất bại. Không có phát hiện hoặc gói bằng chứng thì không thể tự coi precision/replay là đạt. Luôn có tử/mẫu và khoảng bất định mô tả khi phù hợp; năm fixture có lỗi không đủ chứng minh ưu thế tổng quát. Nếu lấy mẫu lại, nhóm theo fixture thay vì từng lượt. Chốt quy trình trước tuần 5. Sau khi lộ, tập tuần 5 trở thành bộ kiểm tra regression. Nếu trượt điều kiện hoặc thay đổi đáng kể năng lực, cần sửa và dùng tập giữ kín mới trước khi tuyên bố hiệu quả trên ca chưa từng thấy; chạy lại thông thường không đòi hỏi tập mới. Lượt chạy và chi phí tập giữ kín mới nằm ngoài ngân sách 120 lượt ban đầu. Không luyện trên ca đã lộ rồi gọi là chưa từng thấy.

## Lịch, pilot và quyết định

Tuần 1 xác định giả thuyết và giấy phép. Tuần 2 tạo đáp án và quy trình chạy lại ổn định. Tuần 3–4 chỉ dùng fixture phát triển. Tuần 5 chốt bản rồi đánh giá giữ kín; phải đạt mọi điều kiện trước beta bên ngoài. Tuần 6 tuyển **năm dev tự nguyện, trên ít nhất ba repo** cho pilot riêng có hỗ trợ. Tuần 7 chuẩn bị release candidate. Tuần 8 chạy lại tập tuần 5 đã lộ để kiểm tra regression, không tạo bằng chứng mới trên ca chưa từng thấy. Chỉ mở beta công khai sau khi qua điều kiện tuần 5, chạy lại regression và xử lý vướng mắc pilot. Trượt điều kiện hoặc thay đổi đáng kể năng lực sẽ kéo dài lịch để đánh giá tập giữ kín mới.

Mục tiêu pilot: ≥4/5 dev tự thiết lập không cần maintainer can thiệp trực tiếp, có bằng chứng dùng được trong 20 phút; hỗ trợ trung vị ≤30 phút/dev; ≥3/5 tự nguyện dùng lại cho thay đổi khác trong 14 ngày, kiểm tra cuối tuần 8. Với dùng lại, ghi riêng số người có cơ hội phù hợp: chưa có migration thứ hai không phải thất bại. Nếu dưới năm người có thay đổi phù hợp, công bố mẫu số thực tế. Chỉ số này giúp quyết định cải tiến hay chuyển hướng; không tự chặn phát hành khi thiếu cơ hội sử dụng. Ghi cả lần bắt đầu thất bại và người rút lui. Năm người quen không chứng minh retention hoặc khả năng trả phí. Hỏi tính hữu ích, phần chưa kiểm tra và lý do không quay lại.

## Nguồn lực, quản trị và công bố

Lịch tám tuần danh nghĩa giả định một maintainer cùng AI, khoảng 180 giờ gốc ở mức 22,5 giờ/tuần. Cộng dự phòng 25% thành 225 giờ: cần dành **9–12 tuần lịch ở mức 20–25 giờ/tuần**, không cam kết đủ dự phòng trong tám tuần. Trần ban đầu cho 120 lượt benchmark là **40 giờ chạy cộng dồn và 4,8 triệu token tổng**; đây là giới hạn, chưa phải số đo. Replay tuần 8, chạy lại do thất bại, tập giữ kín mới và thí nghiệm bổ sung đều tăng lượt chạy và chi phí ngoài 120 lượt đó. Ước lượng riêng việc tạo fixture, cache/setup, review và hỗ trợ pilot. Tính tiền từ lượng input/output/cache thực tế cùng giá hiện hành của nhà cung cấp; không tự đặt kết quả chi phí bằng USD.

Mã dự án dùng MIT, copyright © 2026 Upgrade Witness contributors. Ghi nguồn, quyền sử dụng, giấy phép, ghi công và thông báo bên thứ ba cho từng fixture; MIT của dự án không đổi giấy phép dependency hay ví dụ sao chép. Ưu tiên fixture tự tạo. Chỉ công bố báo cáo đã được đồng ý và làm sạch; mặc định không tải mã riêng, credential, ảnh hoặc log lên công khai. Xử lý dữ liệu của nhà cung cấp agent tách biệt với chính sách telemetry của CLI.

Phát hành benchmark card song ngữ gồm phạm vi, phiên bản, đầy đủ dòng kết quả, thất bại, phương pháp, cách chạy lại và giới hạn. Trước khi đo, hiển thị **“Chưa đo”**. Đánh phiên bản các tập đã lộ để chạy lại regression; tạo tập giữ kín mới khi trượt điều kiện hoặc thay đổi đáng kể năng lực đòi hỏi đánh giá trên ca chưa từng thấy.
