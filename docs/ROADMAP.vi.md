> Bản thiết kế ban đầu ngày 16/10/2026. Xem [README](../README.vi.md) và [STATUS](STATUS.md) cho trạng thái triển khai hiện tại; tài liệu này không phải nhật ký kết quả.

# Upgrade Witness — roadmap từ ý tưởng đến phát hành

[English](ROADMAP.en.md) · [Bắt đầu tại đây](../README.md) · Mốc lập kế hoạch: 16/10/2026

**Quyết định:** xây một skill AI chuyên sâu, kèm bộ công cụ thực thi nhỏ, giúp dev kiểm chứng hành vi giao diện sau khi nâng cấp dependency. Phạm vi đầu tiên: React + Vite + Tailwind CSS v3 → v4, npm và Chromium. Đã chọn giấy phép MIT. Tên sản phẩm là tên làm việc; chưa kiểm tra khả dụng tên repo, package và tên miền.

**Trạng thái:** bộ tài liệu này gồm kế hoạch và tài sản trình bày repo. Chưa có bộ kiểm chứng được triển khai, package phát hành, kết quả benchmark đã đo hay repo GitHub đã xuất bản.

## Cập nhật triển khai: alpha Rust native — 17/10/2026

Kế hoạch W1–W8 bên dưới được viết ngày 16/10, trước yêu cầu triển khai và đổi sang Rust. Roadmap code đã chuyển hướng theo đó. Upgrade Witness hiện là repo MIT công khai, gồm CLI Rust native, agent skill Markdown, fixture mục tiêu React/Vite/npm, report, test và bộ tài liệu đóng góp song ngữ. Nhánh hiện tại đã gỡ lõi TypeScript; prototype 0.1 vẫn nằm trong lịch sử Git nhưng chưa từng phát hành lên npm.

| Hạng mục | Bằng chứng/trạng thái hiện tại |
|---|---|
| W1 khám phá và xác định vấn đề | Đã ghi repo, phạm vi và yêu cầu gốc. Chưa phỏng vấn người dùng. |
| W2 vertical slice và W3–4 alpha | Có CLI Rust, detached revision, capture Chromium lặp, assertion hành vi, review pixel, report có hash và demo positive-control. 29 test Rust, strict Clippy và format đã pass local trên Windows/Chrome 155. |
| W5 đánh giá độc lập | Chưa chạy. Bộ benchmark đề xuất 20 fixture/120 lượt so sánh chưa được thực hiện. |
| W6 pilot tự nguyện | Chưa thực hiện. Chưa có buổi dùng thử hay kết quả dùng lại 14 ngày; không khẳng định đã có pilot. |
| W7 release candidate | Chỉ chuẩn bị sau khi đạt ngưỡng đánh giá và xử lý vướng mắc pilot có đồng thuận; không suy ra mức sẵn sàng từ demo tổng hợp. |
| W8 beta công khai | Chưa lên lịch hoặc phát hành. Cần đánh giá độc lập và review pilot trước beta. |

Bằng chứng demo Rust: 14 checkpoint; 12 đạt, hai regression cố ý, không bỏ sót; ảnh còn cần review nên `complete:false`. Đây chỉ là positive-control xác định, không phải phép đo độ chính xác. CI Rust của repo kiểm commit trên Ubuntu/Windows. Xem [trạng thái hiện tại](STATUS.md) và [kiến trúc/chuyển sang Rust](RUST-MIGRATION.md) để biết runtime/giới hạn.

## 1. Lời hứa sản phẩm và người dùng đầu tiên

**Tiếng Việt:** “Nâng cấp thư viện. Thấy rõ điều gì thay đổi.”

**English:** “Upgrade your dependencies. See what changed.”

Người dùng đầu tiên là maintainer của ứng dụng React quy mô nhỏ hoặc vừa, cần nâng Tailwind nhưng chưa có đủ test giao diện hoặc test xuyên suốt luồng sử dụng. Công việc đầu tiên là đối chiếu một tập trang và trạng thái component đã thống nhất, trước và sau nâng cấp, rồi đưa ra phát hiện mà người review chạy lại được. Người dùng thứ hai là dev đang review PR cập nhật dependency.

Báo cáo phải tách biệt: khác biệt quan sát được, thay đổi có chủ đích, nghi ngờ regression, regression đã xác minh và vùng chưa kiểm tra. Build thành công chưa đủ chứng minh hành vi được giữ nguyên. Ảnh khác nhau cũng chưa đủ kết luận có bug.

Giả thuyết khác biệt nằm ở việc nối rủi ro đặc thù migration với phép kiểm có mục tiêu và bằng chứng để review. [Codemod AI](https://codemod.com/blog/npx-codemod-ai) đã có skill, công cụ và test cho migration; [Playwright](https://playwright.dev/docs/test-snapshots) đã có so sánh ảnh. Ta tích hợp khả năng đó và đo giá trị bổ sung, không tuyên bố tạo ra một lĩnh vực mới hay bảo đảm ứng dụng không có lỗi.

## 2. Bằng chứng nhu cầu và cách kiểm chứng

Bắt đầu từ danh mục có nguồn trong [BENCHMARK](BENCHMARK.vi.md). [Tailwind #20435](https://github.com/tailwindlabs/tailwindcss/issues/20435) ghi nhận migration sửa nhầm giá trị prop/data-attribute; [#19555](https://github.com/tailwindlabs/tailwindcss/issues/19555) ghi nhận bố cục cột đổi sau migration cấu hình. Đây là các dạng lỗi cụ thể, chưa phải bằng chứng về quy mô thị trường.

Tuần 1: đọc 20 issue liên quan trên ít nhất ba repo, lưu ngày, trạng thái, môi trường, cách xử lý tạm, phản hồi maintainer và khả năng chạy lại bằng chứng. Gộp các báo cáo trùng triệu chứng. Tìm năm dev đồng ý trao đổi qua quan hệ sẵn có hoặc kênh cộng đồng cho phép; không bình luận quảng cáo hàng loạt vào issue.

Hỏi về lần nâng cấp gần nhất: lỗi gì, phát hiện bằng cách nào, mất bao lâu, đang dùng kiểm tra nào, có thể chia sẻ ví dụ đã loại dữ liệu nhạy cảm không, và báo cáo đề xuất có giúp họ quyết định merge không. Ghi hành vi thực tế riêng với lời khen dành cho ý tưởng.

**Điều kiện qua bước khám phá:** ít nhất 3/5 người kể được một vấn đề kiểm chứng cụ thể gần đây; ít nhất hai người cung cấp project hoặc fixture được phép dùng thử. Đây là ngưỡng đề xuất. Nếu chưa đạt, tiếp tục khám phá hoặc thu hẹp bài toán trước khi mở rộng triển khai.

## 3. Quyết định phạm vi

| Có trong bản đầu | Để sau khi có bằng chứng cần thiết |
|---|---|
| React/Vite, Tailwind v3→v4; một luồng npm được ghim phiên bản | Next.js SSR/RSC, Vue, Angular, migration backend hoặc database |
| Hai revision có sẵn hoặc nâng cấp trong bản làm việc riêng | Ghi vào production, tự merge, nâng cấp đồng loạt nhiều repo |
| Trang/trạng thái đã chọn, dữ liệu cố định, kiểm tra DOM/CSS, ảnh và tương tác | Tự khám phá mọi trạng thái, cam kết hiệu năng |
| Báo cáo Markdown/HTML và bằng chứng JSON có phiên bản | Dashboard lưu trên máy chủ, tài khoản, thanh toán, telemetry từ xa |
| Skill ngắn và CLI TypeScript nhỏ; kiểm chứng một agent host trước | MCP server, nhiều agent, ma trận pnpm/yarn |
| Kiểm thử Linux CI và Windows local riêng biệt | Tuyên bố ảnh chụp so sánh trực tiếp được giữa các hệ điều hành |

Chọn agent host đầu tiên khi bắt đầu dựa trên công cụ sẵn có; chỉ công bố phiên bản hỗ trợ sau khi chạy thực tế. Kiến trúc không phụ thuộc riêng một host. Mỗi tích hợp mới cần kiểm thử nghiệm thu riêng.

## 4. Hành trình sử dụng và kết quả cần đạt

1. Dev cung cấp repo, yêu cầu nâng cấp hoặc hai revision, cùng trang/trạng thái quan trọng.
2. Skill kiểm tra môi trường và xác định phạm vi kiểm chứng; ghi rõ điều kiện chưa hỗ trợ và lỗi đã tồn tại.
3. Chạy bản trước nâng cấp, ghi nhận và cố định hành vi kỳ vọng. Giữ phép kiểm độc lập với cách triển khai của bản mới.
4. Thực hiện migration trong bản sao riêng, hoặc đọc revision đã nâng cấp. Không tự chuyển branch người dùng hay bỏ thay đổi chưa commit.
5. Chạy cùng tình huống trên hai revision trong cùng môi trường. Kiểm tra lại tín hiệu không ổn định trước khi kết luận.
6. Xuất báo cáo có vị trí code, bằng chứng trước/sau, lệnh chạy lại, nguồn giải thích thay đổi dự kiến và khoảng trống kiểm tra.
7. Nếu người dùng yêu cầu sửa, tạo patch nhỏ nhất và chạy lại phép kiểm đã cố định ban đầu; giữ bằng chứng lỗi gốc.

Trải nghiệm thành công đầu tiên là một phát hiện chạy lại được hoặc báo cáo “chưa quan sát thấy regression trong phạm vi đã kiểm tra”. Lỗi setup phải trả kết quả chưa thể kết luận, không được báo thành công. Xem hợp đồng dữ liệu và mã thoát trong [ARCHITECTURE](ARCHITECTURE.vi.md).

## 5. Kế hoạch triển khai tám tuần

Giả định một maintainer làm cùng AI, 20–25 giờ tập trung mỗi tuần. Ước lượng **180 giờ, cộng 25% dự phòng = 225 giờ**. Kế hoạch gốc hướng tới tám tuần; dùng hết dự phòng sẽ kéo lịch thành khoảng 9–12 tuần ở nhịp này. Đây là dự toán lập kế hoạch, chưa phải năng suất đo thực tế. Tuần tính từ ngày thực sự bắt đầu; bộ kế hoạch không tạo lịch tự động hay hạn chót.

| Tuần / công sức | Kết quả và công việc | Điều kiện hoàn thành | Phụ thuộc |
|---|---|---|---|
| W1 / 20 giờ | Kiểm chứng nhu cầu, định vị, danh mục fixture, kiểm tra tên; thiết lập MIT và hướng dẫn đóng góp | Đạt điều kiện khám phá; ghi phạm vi và rủi ro chính | Không |
| W2 / 24 giờ | Luồng chạy xuyên suốt: một fixture, hai revision, một tình huống, một phát hiện có bằng chứng | Dev khác chạy lại được từ checkout sạch | W1 |
| W3 / 24 giờ | Hợp đồng baseline, runner, capture ổn định, kiểm DOM/CSS và tương tác | Bắt được lỗi đã biết; ca sạch không bị báo sai; lỗi setup chưa thể kết luận | W2 |
| W4 / 24 giờ | Workflow skill, renderer báo cáo, loại dữ liệu nhạy cảm, manifest, kiểm Windows/Linux, sửa và kiểm tra lại | Chạy mười fixture phát triển; thử được từ gói đóng gói | W3 |
| W5 / 24 giờ | Chốt phiên bản skill/công cụ; benchmark trên bộ giữ kín, đối chiếu cùng agent không dùng skill | Đạt ngưỡng BENCHMARK; giữ số đếm gốc và các lần chạy thất bại | W4 |
| W6 / 20 giờ | Năm dev dùng thử tự nguyện trên ba repo; đo setup và giá trị báo cáo | Ghi nhận kết quả ban đầu; bắt đầu cửa sổ theo dõi 14 ngày | W5 |
| W7 / 20 giờ | Sửa lỗi cản trở pilot, đóng gói release candidate, demo thật, tài liệu EN/VI, diễn tập phát hành | Cài mới và chạy lại được; mọi tuyên bố công khai có bằng chứng | W6 |
| W8 / 24 giờ | Hoàn tất theo dõi pilot, chạy lại bộ W5 đã công bố trên bản ứng viên, phát hành beta khi đủ điều kiện | Ghi quyết định chất lượng/pilot; có artifact gắn tag, tài liệu và quy trình hỗ trợ | W7 và theo dõi pilot |

Lượt W8 kiểm tra regression trên bộ đã công bố, không phải bằng chứng giữ kín mới. Nếu trượt ngưỡng chất lượng hoặc thay đổi năng lực đáng kể, cần bộ giữ kín mới cùng dự toán lượt chạy, review và chi phí bổ sung ngoài thử nghiệm 120 lượt ban đầu.

Maintainer chịu trách nhiệm ưu tiên, review và quyết định phát hành. AI hỗ trợ triển khai, kiểm tra và chạy thử; mọi kết luận vẫn cần bằng chứng quan sát được. Nên có người review không viết phần triển khai kiểm tra một phần phát hiện và cách chấm bộ giữ kín.

Nếu không đạt điều kiện, sửa luồng đang lỗi, ghi quyết định và dời phát hành. Không bù bằng cách thêm framework hoặc làm trang giới thiệu đẹp hơn. Beta là kết quả của nghiệm thu, không tự đến khi hết tuần tám.

## 6. Thứ tự triển khai và bảng công việc

Dùng các việc đủ nhỏ để tạo issue trong [BACKLOG](BACKLOG.md). Cột đề xuất: Backlog → Ready → In progress → Review → Verified → Done. Với một maintainer, giới hạn tối đa một việc triển khai lõi và một việc tài liệu đang làm cùng lúc.

Mỗi issue có kết quả cho người dùng, phạm vi hỗ trợ, ID phụ thuộc, bằng chứng nghiệm thu và ước lượng. Hoàn thành nghĩa là kết quả đã được kiểm chứng và tài liệu khớp thực tế. Milestone: `discovery`, `vertical-slice`, `alpha`, `evaluation`, `pilot`, `public-beta`; chỉ tạo phiên bản phát hành khi có artifact.

Workflow branch đề xuất: branch nhỏ → kiểm tra liên quan → review → squash merge. Tiêu đề và nội dung commit mới dùng tiếng Việt, giữ tên kỹ thuật và prefix Conventional Commits; ví dụ `feat: thêm đối chiếu thuộc tính DOM`. Contributor quốc tế có thể mô tả PR bằng tiếng Anh; maintainer hỗ trợ chuẩn bị nội dung commit tiếng Việt.

## 7. Chất lượng, chi phí và điểm cần dừng

BENCHMARK là nơi quy định ngưỡng số để tránh tài liệu mâu thuẫn. Kiểm tra fixture, lỗi runner, sai lệch môi trường, cảnh báo sai, thay đổi có chủ đích, bằng chứng và khả năng chạy lại; không chỉ kiểm tra câu chữ đầu ra. So sánh cùng model, công cụ và ngân sách tác vụ khi có/không có skill; không để đáp án test lọt vào prompt hay references.

Theo dõi riêng giờ phát triển, mức dùng model/API, phút CI, dung lượng artifact và hỗ trợ pilot. Chạy thử ba fixture để hiệu chuẩn chi phí rồi đặt trần mỗi lượt trước khi chạy benchmark đầy đủ. Kiến trúc cơ bản đề xuất không bắt buộc dịch vụ trả phí. Hạn mức tài khoản, giá model và nơi lưu trữ phụ thuộc cách triển khai, cần kiểm tra khi bắt đầu.

| Rủi ro | Xử lý sớm | Dừng hoặc thu hẹp khi |
|---|---|---|
| Nhiễu ảnh lấn át phát hiện hữu ích | Cùng OS/browser/font/dữ liệu; ưu tiên assertion ổn định trước pixel diff | Ca sạch liên tục bị báo sai |
| Agent sửa test cho khớp lỗi | Cố định phép kiểm baseline; lưu quyết định thay đổi kỳ vọng | Phép kiểm lỗi bị âm thầm nới lỏng |
| Lộ dữ liệu riêng tư | Dữ liệu giả, lọc trước khi gửi agent/export, không đưa raw evidence vào Git | Không thể xem xét hoặc làm sạch phần xuất |
| Setup tốn hơn giá trị nhận được | Một stack/package manager; chẩn đoán điều kiện chạy rõ | Hỗ trợ pilot chiếm phần lớn công sức |
| Skill ít giá trị hơn agent thông thường | Đánh giá đối chiếu và tách từng thành phần | Không có phát hiện hữu ích bổ sung với chi phí chấp nhận được |
| Quá tải maintainer | Ma trận hỗ trợ hẹp, mẫu issue, nhịp phát hành phù hợp | Hỗ trợ vượt thời gian duy trì mỗi tuần |

Script cài dependency của repo là code được thực thi. Với project chưa tin cậy, dùng môi trường dùng một lần phù hợp; Git worktree không phải sandbox bảo mật. Chạy tại máy không đồng nghĩa dữ liệu gửi cho nhà cung cấp model cũng ở lại máy.

## 8. GitHub, MIT và vòng đời đóng góp

README tiếng Anh là điểm vào chính, có liên kết tiếng Việt dễ thấy. Đặt kết quả cho người dùng, trạng thái trung thực và một demo thật trước chi tiết kiến trúc. [GITHUB](GITHUB.vi.md) quy định bố cục README, kịch bản demo, About/topics, ảnh chia sẻ, badge, phát hành và đồng bộ ngôn ngữ. [Bản xem trước repo](../README.vi.md) dùng được ngay cho giai đoạn lập kế hoạch.

[Giấy phép MIT](../LICENSE) đã chọn cho phép sử dụng lại và đóng góp theo điều khoản của nó. Giữ nguyên văn bản tiếng Anh chuẩn; phần tiếng Việt giải thích mà không thay thế giấy phép. Dòng bản quyền: `Copyright (c) 2026 Upgrade Witness contributors`. Giữ thông báo và giấy phép bên thứ ba; MIT của dự án không đổi giấy phép dependency hoặc ví dụ nhập vào. Nguồn: [OSI MIT](https://opensource.org/license/mit).

Chào đón báo lỗi, fixture tối thiểu, tài liệu, bản dịch và adapter thuộc phạm vi hỗ trợ. Khởi đầu bằng CONTRIBUTING và mẫu issue/PR song ngữ được cung cấp. Maintainer review nguồn gốc/quyền đóng góp và bằng chứng kiểm thử. Chấp nhận đóng góp có AI hỗ trợ khi người gửi hiểu và xác minh nội dung; ghi công người thực sự đóng góp.

Trước phát hành: chọn owner/repo, kiểm tra tên, đưa tài liệu vào cấu trúc repo đã chọn và sửa liên kết, thiết lập CI thật, kiểm gói cài đặt, cấu hình kênh báo lỗ hổng riêng tư, ghi ba demo thật và diễn tập phát hành. Bộ kế hoạch này không tạo repo hay thực hiện xuất bản.

## 9. Phát hành, 30 ngày đầu và hướng mở rộng

Chỉ phát hành beta khi qua nghiệm thu. Kèm phiên bản đã hỗ trợ, giới hạn, case study chạy lại được, checksum, hướng dẫn cài/gỡ đã thử trên môi trường sạch và changelog gắn commit thật. Có cách thao tác thủ công nếu installer lỗi. Không hiển thị badge thành công hoặc lệnh cài npm cho artifact chưa tồn tại.

Trong 30 ngày đầu, phân loại issue theo nhịp duy trì được, phản hồi báo lỗi có reproducer, theo dõi setup và dùng lại từ phản hồi tự nguyện, phát hành bản sửa tập trung. Chạy lại fixture liên quan khi đổi capture, assertion, dependency hoặc cơ chế chọn skill. Đặt phiên bản cho JSON report và manifest fixture. Cập nhật tài liệu Anh–Việt cùng lúc khi hành vi cho người dùng đổi.

Ngày 30, quyết định tăng độ bao phủ Tailwind, thêm pnpm hay kiểm chứng adapter Next.js. Chỉ thêm MCP khi người dùng cần giao diện công cụ mà CLI phục vụ chưa thuận tiện. Chỉ cân nhắc dịch vụ cộng tác có máy chủ khi xuất hiện nhu cầu lặp lại; phần đó nằm ngoài cam kết triển khai của roadmap này.

**Thành công là dev ra quyết định migration tốt hơn nhờ bằng chứng chạy lại được.** Sao và lượt tải hỗ trợ khám phá sản phẩm, nhưng không thay thế thước đo đó.
