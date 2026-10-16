# Security / Bảo mật

For a sensitive vulnerability, use [GitHub private vulnerability reporting](https://github.com/2274802010922/upgrade-witness/security/advisories/new). Private reporting is enabled for this repository. Do not put credentials or exploit details in a public issue. / Với lỗ hổng nhạy cảm, dùng kênh báo riêng tư GitHub, không đưa credential hoặc chi tiết khai thác vào issue công khai.

This early alpha runs project installation, build and dev-server scripts with the user's permissions. `--trust-project` is an explicit execution boundary, not a sandbox. Do not run an untrusted repository with secrets in the environment. / Alpha chạy script của project bằng quyền người dùng. Cờ tin cậy không tạo sandbox; không chạy repo chưa tin cậy với bí mật trong môi trường.

Use synthetic data and inspect reports before sharing. Automatic input masking and token redaction cannot identify every secret. The CLI has no upload or LLM API feature; a host agent's data handling is separate. / Dùng dữ liệu giả, xem báo cáo trước khi chia sẻ; che input và lọc token không nhận ra mọi bí mật. CLI không upload hay gọi API LLM; cách agent host xử lý dữ liệu là vấn đề riêng.

Only the latest alpha is maintained. Reports are handled on a best-effort basis; there is no response-time guarantee. / Chỉ duy trì alpha mới nhất, chưa có cam kết thời gian phản hồi.
