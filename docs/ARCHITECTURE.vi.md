> Bản thiết kế ban đầu ngày 16/10/2026. Xem [README](../README.vi.md) và [STATUS](STATUS.md) cho trạng thái triển khai hiện tại; tài liệu này không phải nhật ký kết quả.

# Upgrade Witness — kiến trúc đề xuất

[English](ARCHITECTURE.en.md) · Thiết kế ngày 16/10/2026

**Trạng thái: bản thiết kế.** CLI, package, phạm vi tương thích và benchmark trong tài liệu này chưa phải phần mềm đã phát hành. Các lệnh minh họa mô tả giao diện dự kiến xây dựng.

## 1. Ranh giới sản phẩm

Upgrade Witness giúp lập trình viên trả lời: “Sau nâng cấp, hành vi quan sát được nào đã thay đổi, thay đổi nào vi phạm yêu cầu, và người khác có thể tái hiện bằng chứng không?” Phạm vi đầu tiên nên là **React + Vite + Tailwind CSS 3 → 4, npm, một ứng dụng, Chromium**. Linux CI và Windows local là hai môi trường hỗ trợ riêng; mỗi phép so sánh phải chạy cả hai revision trong cùng môi trường. Next.js, monorepo, pnpm, trình duyệt khác, client AI khác và MCP được bổ sung theo nhu cầu đã kiểm chứng.

Có hai điểm bắt đầu: hỗ trợ thực hiện nâng cấp, hoặc kiểm chứng bản nâng cấp đã commit. Cả hai đều quy về hai revision Git cố định và một bộ yêu cầu kiểm chứng đã khóa. Giữ test của người dùng, dùng agent hiện có để đọc code, đề xuất kịch bản, sửa trong phạm vi được giao và giải thích phát hiện. CLI phụ trách thao tác cần lặp lại chính xác. Chưa xây bộ định tuyến model, vòng lặp LLM tự trị hay framework codemod tổng quát.

Hướng dẫn chính thức của Tailwind đề nghị xem lại diff và kiểm tra trong trình duyệt; tài liệu cũng liệt kê các thay đổi có thể làm giao diện khác trước một cách hợp lệ. Đây là dữ liệu điều tra, chưa phải bằng chứng rằng chủ sản phẩm chấp nhận thay đổi đó. [Hướng dẫn nâng cấp chính thức](https://tailwindcss.com/docs/upgrade-guide)

## 2. Thành phần và trách nhiệm

```text
Yêu cầu của lập trình viên
      │
Agent lập trình hiện có + skill
      │ các lệnh có cấu trúc dự kiến
TypeScript CLI
  ├─ kiểm tra điều kiện và quản lý workspace theo revision
  ├─ adapter React/Vite + npm
  ├─ chạy kịch bản đã khóa bằng Playwright
  ├─ so sánh và tạo manifest bằng chứng
  └─ báo cáo JSON + HTML tĩnh + Markdown đã che dữ liệu
```

Giữ `skills/upgrade-witness/SKILL.md` ngắn: tiêu chí kích hoạt, hai chế độ, quy tắc bằng chứng, điều kiện dừng và tham chiếu chỉ đọc khi cần. Đặt hợp đồng kịch bản, quy trình điều tra và quy tắc Tailwind trong các tài liệu tham chiếu riêng. Logic thực thi nằm trong một package nhỏ `packages/cli`, chia module nội bộ; chưa cần framework nhiều package.

Phần lõi quản lý revision, kiểm tra hợp đồng, điều phối trình duyệt, băm artifact và tạo báo cáo. Adapter nhận diện dự án, kiểm tra phiên bản hỗ trợ, cung cấp thao tác install/build/serve, kiểm tra ứng dụng sẵn sàng và gợi ý rủi ro đặc thù migration. Kết quả của adapter là dữ liệu có cấu trúc; không ghép shell command từ nội dung issue. Ưu tiên truyền mảng đối số cho process. Cấu hình chưa hỗ trợ phải được báo rõ. Dự án dùng giấy phép MIT đã chọn; giữ thông báo bản quyền bên thứ ba và ghi nguồn gốc fixture.

## 3. Điều kiện đầu vào và tính bất biến

Yêu cầu Git, cặp phiên bản Node/npm cố định đã hỗ trợ, npm lockfile ở mỗi revision, lệnh chạy local có thể lặp lại và dữ liệu fixture ổn định. Chọn và kiểm tra phiên bản cụ thể trong tuần 1; chưa quảng bá ma trận chưa thử nghiệm. Ghi lại phiên bản CLI, revision trình duyệt, image hệ điều hành, phiên bản dependency, hash lockfile, digest cấu hình và hai commit SHA đầy đủ.

Chỉ resolve ref một lần. Tạo detached worktree tạm trong thư mục riêng của từng lần chạy và cài dependency độc lập. Git worktree cho phép có working tree riêng mà không thay thế checkout người dùng đang làm việc. [Tài liệu Git worktree](https://git-scm.com/docs/git-worktree)

Checkout có thay đổi chưa commit phải được giữ nguyên. Có thể kiểm chứng hai ref đã commit được chỉ định rõ, đồng thời liệt kê những file chưa commit bị loại khỏi đầu vào. Nếu bản cần kiểm chứng chỉ tồn tại dưới dạng thay đổi chưa commit, giải thích rằng còn thiếu đầu vào bất biến và dừng nhánh xử lý này. Một phiên bản sau có thể tạo snapshot khi được yêu cầu, không sửa index của người dùng; v0.1 không âm thầm stash, reset, commit, clean hoặc sao chép secret bị ignore. Cleanup chỉ xóa đường dẫn thuộc thư mục lần chạy đã kiểm tra, từ chối mọi đường dẫn bên ngoài.

Chạy `npm ci` trong checkout tạm. Manifest và lockfile không khớp là lỗi điều kiện đầu vào, không được tự “sửa” trong lúc đo candidate. Lifecycle script của dependency và lệnh khởi động đều thực thi code dự án; chỉ chạy trong ngữ cảnh phát triển được người dùng tin cậy và không cấp secret cho code pull request không tin cậy trên CI. [Tài liệu npm ci](https://docs.npmjs.com/cli/v11/commands/npm-ci/)

## 4. Quy trình đầu cuối

1. **Khảo sát:** xác định migration, stack hỗ trợ, route, tương tác quan trọng, test hiện có, thay đổi có chủ đích và ràng buộc môi trường.
2. **Khóa kỳ vọng:** tạo file kịch bản có phiên bản từ bản base và yêu cầu của người dùng. Ghi fixture, viewport, bước tương tác, assertion, vùng ảnh, vùng che, ngưỡng sai lệch và thay đổi được chấp nhận. Băm hợp đồng này trước migration hoặc trước khi agent kiểm chứng đọc kết quả candidate.
3. **Kiểm tra base:** chạy mỗi kịch bản bắt buộc hai lần. Assertion đã lỗi, thiếu font, pixel bất ổn vượt ngưỡng khai báo, thiếu dữ liệu hoặc ứng dụng chưa sẵn sàng khiến so sánh thiếu tin cậy. Báo riêng các vấn đề của baseline.
4. **Tạo hoặc chọn candidate:** với nâng cấp mới, agent chạy migrator chính thức ở phiên bản đã cố định và sửa trong workspace candidate tách biệt. Chỉ kiểm chứng khi có SHA candidate bất biến. Với nâng cấp có sẵn, chọn SHA đó; công bố rằng kỳ vọng được dựng lại sau nâng cấp và không dùng kết quả head để xây hợp đồng baseline.
5. **Chạy hai phía:** dùng cùng kịch bản đã khóa và assertion ban đầu. Test khám phá thêm trên candidate có ích nhưng không được thay oracle cũ hoặc biến một kiểm tra lỗi thành đạt.
6. **So sánh và điều tra:** chạy lại kiểm tra lỗi một lần, đối chiếu bằng chứng với diff code và hướng dẫn migration; tách điều quan sát được khỏi cách giải thích.
7. **Xem xét và sửa khi được yêu cầu:** đề xuất patch tối thiểu. Mỗi lần sửa tạo danh tính candidate và báo cáo mới. Thay kỳ vọng phải có lý do được ghi nhận rõ; giữ kết quả lỗi trước đó.
8. **Xuất báo cáo:** gồm độ bao phủ, phát hiện, giới hạn và lệnh tái hiện kèm manifest toàn vẹn. Không kết luận rằng toàn ứng dụng an toàn.

Không tự cập nhật baseline trong lúc kiểm chứng. Một thay đổi được upstream ghi nhận chỉ là ứng viên để xem xét chấp nhận, không phải quyền tự nới test.

## 5. Giao thức trình duyệt ổn định

Cố định phiên bản Playwright và Chromium đi kèm. Giữ nhất quán viewport, device scale, locale, múi giờ, chế độ màu, reduced motion, font, đồng hồ và dữ liệu test khi có thể. Dùng fixture cho phản hồi mạng; liệt kê dependency live như một giới hạn. Chờ tín hiệu sẵn sàng của ứng dụng và font tải xong. Tắt animation nhất quán. Chạy hai build tuần tự trên cùng origin local, tạo storage sạch và dừng server giữa hai lượt.

Chỉ so ảnh trong cùng cấu hình hệ điều hành/trình duyệt. Playwright lưu ý kết quả render phụ thuộc môi trường; baseline Linux và Windows không thể thay thế cho nhau. [So sánh hình ảnh với Playwright](https://playwright.dev/docs/test-snapshots)

Bắt đầu với hai chiều rộng 375 và 1440 CSS pixel cùng một nhóm nhỏ luồng đã khai báo. Bổ sung 768/1024 và trạng thái VI/EN cần thiết trong alpha nếu ứng dụng pilot có sử dụng. Ưu tiên locator theo role hoặc test ID ổn định. Thu thập computed CSS và thuộc tính DOM quanh assertion lỗi, không dump cả ứng dụng. Sai khác pixel chỉ mang nhãn “cần xem xét”, trừ khi vi phạm yêu cầu hình ảnh đã khóa hoặc đã có quyết định đánh giá rõ ràng.

## 6. Mô hình phát hiện và bằng chứng

Cấu trúc JSON đề xuất:

```json
{
  "schemaVersion": "0.1",
  "runId": "local-run-id",
  "baseSha": "full-sha",
  "headSha": "full-sha",
  "contractSha256": "digest",
  "complete": false,
  "findings": [{
    "id": "UW-001",
    "scenarioId": "navigation.mobile",
    "kind": "regression",
    "observation": "Menu mở trên base nhưng vẫn bị ẩn trên head.",
    "assertionId": "menu.visible-after-click",
    "suspectedCause": {"text": "Hành vi hidden/display thay đổi", "status": "unverified"},
    "evidenceIds": ["base-dom", "head-dom", "head-shot"],
    "intentDecision": null,
    "reproducedRuns": 2
  }],
  "coverage": {"planned": 8, "passed": 6, "failed": 1, "notRun": 1}
}
```

Các loại phát hiện: `regression`, `intended-change`, `needs-review`, `baseline-issue`, `inconclusive`. Observation là sự kiện đã đo. Nguyên nhân nghi vấn được đánh dấu riêng và liên kết tới code candidate. Thay đổi có chủ đích phải kèm quyết định chấp nhận và phạm vi. Coverage ghi mọi kết quả theo kịch bản/môi trường và các mục loại trừ; tỷ lệ luôn có mẫu số rõ ràng. Assertion lỗi đã tái hiện chứng minh hành vi suy giảm quan sát được, chưa chứng minh nguyên nhân gốc.

Mỗi bằng chứng ghi đường dẫn tương đối, media type, SHA-256 của byte đã che dữ liệu, bước thu thập, revision nguồn và digest môi trường. Băm riêng hợp đồng đã khóa và manifest cuối. Hash giúp phát hiện dữ liệu bị sửa, không chứng minh dữ liệu được thu thập trung thực hay quan hệ nhân quả với code.

## 7. Giao diện lệnh và CI đề xuất

Các lệnh sau **chỉ minh họa thiết kế, chưa sử dụng được trước khi triển khai**:

```text
uw doctor --config witness.config.json
uw plan --base <sha> --out <run-directory>
uw capture --run <run-directory> --revision base
uw compare --run <run-directory> --head <sha>
uw report --run <run-directory> --format html,markdown,json
```

`plan` khóa hợp đồng base; `compare` từ chối hợp đồng bị đổi và ghi danh tính head. Mỗi lệnh có kết quả ổn định cho máy đọc. Exit code đề xuất: **0** = mọi kiểm tra bắt buộc hoàn thành, không còn regression hay khác biệt chưa xử lý; **1** = có ít nhất một regression xác nhận; **2** = chưa kết luận, chưa chạy đủ, bất ổn, chưa hỗ trợ hoặc lỗi đầu vào khi chưa có regression xác nhận; **64** = dùng CLI sai. Nếu vừa có regression vừa thiếu kiểm tra, trả 1 và `complete:false`; không giấu phần còn thiếu. CI luôn lưu artifact khi lỗi, chặn với 1/2/64 và xem cập nhật baseline như một thay đổi cần review riêng.

## 8. Quyền riêng tư và các mốc kiểm chứng

Mặc định dùng fixture, tắt HAR/full tracing. Che selector nhạy cảm trước khi chụp; chỉ cho phép các trường DOM cần thiết; lọc log, đường dẫn và URL trước khi agent đọc hoặc xuất báo cáo. Mặc định chỉ lưu artifact đã che rồi mới tính hash. Lỗi che dữ liệu phải chặn export. Escape nội dung không tin cậy trong HTML, không thực thi nội dung báo cáo. Ghi rõ thời gian giữ file local và cách xóa; mặc định tắt telemetry.

CLI chạy local. Agent chủ có thể gửi ngữ cảnh hoặc artifact được chọn tới nhà cung cấp model theo cấu hình client. Vì vậy không quảng bá “code không bao giờ rời máy” chỉ vì runner chạy local. Nếu sau này thêm chẩn đoán thô, cần một quy trình chỉ lưu local riêng và quy tắc xử lý rõ ràng.

Ước tính tám tuần, khoảng 180 giờ làm việc ở nhịp 20–25 giờ mỗi tuần, cộng 25% dự phòng tương đương 45 giờ. Nếu dùng hết 225 giờ thì lịch kéo dài khoảng 9–12 tuần với cùng nhịp làm việc. Nếu chưa qua điều kiện kiểm chứng, giảm phạm vi hoặc dời phát hành; lịch kết thúc không tự chứng minh sản phẩm đã sẵn sàng.

| Giai đoạn | Điều kiện kỹ thuật để đi tiếp |
|---|---|
| W1 khảo sát | Chốt phiên bản hỗ trợ, ba kịch bản thực tế, hợp đồng và ranh giới adapter. |
| W2 lát cắt đầu cuối | Một fixture trước/sau thực tế tạo báo cáo lặp lại được trong một môi trường chính. |
| W3–4 alpha | Hoàn thiện schema, che dữ liệu, bảo vệ checkout đang sửa, quyết định thay đổi có chủ đích, timeout và cleanup; kiểm chứng cả Windows local và Linux CI. |
| W5 đánh giá | Bộ lỗi giữ riêng và nâng cấp đúng; so cùng agent khi có/không skill; đo phát hiện, báo động sai, hoàn thành, thời gian và chi phí. |
| W6 pilot | Ba dự án bên ngoài hoặc thử nghiệm cùng maintainer; ghi lỗi setup và thời gian tới phát hiện hữu ích đầu tiên. |
| W7 release candidate | Tái hiện từ máy sạch, chuẩn bị kết quả và giới hạn, kiểm tra hướng dẫn song ngữ. |
| W8 public beta có điều kiện | Chỉ phát hành khi qua tiêu chí sẵn sàng và trong ma trận đã thử; nếu chưa đạt thì kéo dài kiểm chứng hoặc thu hẹp phạm vi. |

Unit test tập trung schema, ưu tiên exit code, giới hạn đường dẫn và che dữ liệu. Integration test bao gồm base lỗi, output bất ổn, nâng cấp không đổi hành vi, regression thật và thay đổi có chủ đích. Kiểm tra cleanup process và đường dẫn Windows có dấu cách. Agent đánh giá độc lập chỉ nhận skill và fixture gốc, không được biết trước lỗi ẩn. Chỉ đưa MCP và client bổ sung vào triển khai khi hợp đồng CLI và báo cáo lõi đã ổn định.
