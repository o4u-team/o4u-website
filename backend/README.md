# O4U Backend

Rust workspace riêng, tham khảo cấu trúc `nvnhan0810.com/backend`. Chạy trực tiếp trên PostgreSQL O4U hiện tại. Không có migration, seeder hay DDL trong startup.

- `crates/domain`: quy tắc phiên bản, cấu hình app, resource, đăng ký ERP và lỗi nghiệp vụ.
- `crates/application`: command handlers, query/read port, write port và registration gateway.
- `crates/infrastructure`: PostgreSQL repositories, transaction, AES tương thích PHP, Google OAuth, Turnstile, Odoo và FCM.
- `apps/api`: Axum HTTP adapter, session, kiểm tra origin, CORS, health và worker.

Dependency đi từ HTTP/infrastructure vào application/domain; domain không phụ thuộc HTTP hoặc SQLx. CQRS tách read/write repository và handler; cùng dùng schema cũ, không yêu cầu event sourcing.

## Chạy local

Yêu cầu Rust stable và PostgreSQL hiện có. Từ thư mục source O4U:

```sh
python3 backend/scripts/prepare-env.py
cd backend
cargo run -p o4u-api
```

`backend/.env` đã được tạo trong lần chuyển source này, có quyền `0600`, bị git ignore. Script không ghi đè file tồn tại, chỉ đọc cấu hình cần thiết từ `.env` cũ và tạo `SESSION_SECRET` mới. Những lần checkout mới có thể dùng script hoặc `.env.example`. Không cần PHP khi chạy hai app mới; script chuẩn bị env và fixture test là công cụ phát triển.

API mặc định `127.0.0.1:8088`. Chạy frontend tại `http://localhost:5173`; dùng đúng host này để Origin và cookie khớp. `DATABASE_URL` được ưu tiên hơn `DB_*`. Mọi secret vẫn chỉ ở backend.

Google login yêu cầu `GOOGLE_REDIRECT_URI` khớp URL callback đã đăng ký với Google. Local: `http://localhost:5173/auth/callback`; production: `https://<domain>/auth/callback`. Chỉ user đã có trong bảng `users` được đăng nhập. Session mới dùng cookie HttpOnly riêng và bảng `sessions` cũ; đăng xuất thu hồi session phía server.

Nếu frontend SSR và backend dùng hai subdomain cùng domain cha, ví dụ `o4u.example.com` và `o4u-backend.example.com`, đặt callback Google ở backend, giữ `FRONTEND_URL` ở frontend và thêm `SESSION_COOKIE_DOMAIN=example.com`. Cookie phiên sẽ được gửi tới cả hai host, để SSR kiểm tra phiên khi về dashboard. OAuth state vẫn dùng cookie chỉ thuộc host backend. Domain cha phải bao gồm cả hai host và các subdomain của domain đó phải đáng tin cậy vì chúng cũng nhận cookie phiên. Để trống biến này khi chạy local hoặc cùng origin. Khởi động lại backend sau khi đổi cấu hình; frontend SSR cần `BACKEND_URL` trỏ đúng backend. Cookie phiên cũ chỉ thuộc host backend sẽ được xóa khi đăng nhập lại hoặc đăng xuất.

## FCM worker

```sh
cd backend
cargo run -p o4u-api -- worker
```

Worker dùng bảng `jobs`/`failed_jobs` cũ, queue riêng `o4u-rust-fcm`. `FIREBASE_CREDENTIALS` nhận trực tiếp chuỗi JSON service-account hoặc đường dẫn file JSON. Khi đặt JSON trong `.env`, dùng dấu nháy đơn bao ngoài và giữ escape `\n` trong `private_key` (xem `.env.example`). Không cho Laravel worker xử lý queue này. Có tối đa 3 lần thử; các lỗi tạm thời đợi 60 giây, job hết lượt vào `failed_jobs`. Queue là at-least-once: lỗi một phần của multicast có thể làm gửi lặp khi thử lại. Các token/payload bị Firebase từ chối 400/404 được ghi cảnh báo và bỏ qua; không retry mãi token hỏng.

## Kiểm tra

```sh
cd backend
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Integration test được đánh dấu `ignored` để `cargo test` thông thường không tự đụng DB. File `tests/fixtures/legacy-schema.sql` là schema-only export của DB hiện tại, không chứa dữ liệu. Chỉ nạp fixture vào PostgreSQL test riêng:

```sh
/usr/lib/postgresql/18/bin/initdb -D /tmp/o4u-test-postgres -A trust --no-locale -E UTF8
/usr/lib/postgresql/18/bin/pg_ctl -D /tmp/o4u-test-postgres -l /tmp/o4u-test-postgres.log -o '-k /tmp -p 55438 -h 127.0.0.1' start
bash scripts/test-postgres.sh
/usr/lib/postgresql/18/bin/pg_ctl -D /tmp/o4u-test-postgres stop
```

Script cố định cổng 55438 và DB `o4u_migration_test`, tạo lại **DB test này** mỗi lần chạy. Không dùng cổng đó cho DB production. Test từ chối `TEST_DATABASE_URL` có tên database khác. Nếu máy dùng PostgreSQL version khác, thay đường dẫn binary tương ứng. Trong môi trường giới hạn sccache có thể dùng `RUSTC_WRAPPER= cargo ...`.

## Triển khai

Build `cargo build --release -p o4u-api`; chạy `target/release/o4u-api` và `target/release/o4u-api worker` dưới process manager với working directory là `backend`. Frontend được build độc lập. Xem cấu hình reverse proxy mẫu tại `../frontend/deploy/nginx.conf` và [bản đồ chuyển source](docs/MIGRATION.md).

Đặt `FRONTEND_URL` bằng origin public, không có `/` cuối. HTTPS tự bật Secure cookie. Chỉ khai báo IP reverse proxy thực sự trong `TRUSTED_PROXY_IPS`; proxy phải ghi đè `X-Real-IP`. Các request quản trị thay đổi dữ liệu phải có `Origin` khớp `FRONTEND_URL`. Nên phục vụ frontend và API qua cùng origin như cấu hình mẫu.

Tham khảo API thư viện: [Axum](https://docs.rs/axum/0.8.9/axum/), [SQLx](https://docs.rs/sqlx/0.9.0/sqlx/).
