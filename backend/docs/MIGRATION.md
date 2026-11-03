# Bản đồ chuyển source O4U

Chỉ bổ sung hai thư mục `backend/` và `frontend/`. Source Laravel/Vue cũ, file env, deploy script và cấu hình hosting cũ được giữ nguyên. Chưa chuyển traffic production. Hai source mới không dùng PHP làm proxy hay import runtime từ source cũ.

## Chức năng

| Source cũ | Source mới |
| --- | --- |
| Inertia Welcome / ERP SaaS | Vue Router, bản sao nội dung và CSS landing |
| Blade chính sách | Static HTML trong frontend/public/policies |
| Google Socialite | Rust OAuth code exchange, state có chữ ký, cookie HttpOnly |
| Clients / Apps / Client Systems CRUD | `/api/admin/{resource}` + các Vue pages |
| App assignment và pivot settings | Transaction sync-apps, kế thừa mặc định từng trường |
| App usage / connection logs | Transaction upsert device/link và append log; hai bảng phân trang |
| Mobile app version/access/info/ensure | Giữ đường dẫn và headers mobile |
| AES-128-CBC của PHP | Rust envelope iv/value/mac/tag, HMAC-SHA256, fixture PHP test |
| Device API cũ | Giữ client_users/client_user_devices và public-client fallback |
| Laravel FCM jobs | Queue `o4u-rust-fcm` trong jobs cũ, worker Rust |
| ERP register | Turnstile → Odoo registration → Telegram notifier |

## API mobile

| Method | Path | Xác thực |
| --- | --- | --- |
| GET | `/api/app/version` | X-App-Id UUID; optional X-Client-System-Id, X-Platform, X-App-Version |
| POST | `/api/app/permissions` | X-App-Id UUID |
| POST | `/api/app/check-access` | X-App-Id UUID |
| POST | `/api/app/client-systems/ensure` | X-App-Id UUID |
| GET | `/api/app-info` | X-App-Id UUID hoặc numeric ID + X-Client-System-Id |
| POST | `/api/data/decrypt` | X-App-Id UUID |
| POST | `/api/data/encrypt` | X-Authorization API key |
| POST | `/api/user/device` | X-App-Id UUID; optional X-Authorization |
| POST | `/api/fcm/send` | X-Authorization API key |
| POST | `/api/v1/fcm/send-multi` | App + client system liên kết |
| POST | `/api/v1/app-usage/log` | App + client system liên kết |
| POST | `/api/odoo-barcode/log`, `/api/odoo-approval/log` | Public diagnostic |

Admin API dùng JSON thay cho Inertia responses/redirect. `/api/admin/me`, `/logout`, `/clients`, `/apps`, `/client-systems`, `/client-systems/:id/apps`, `/client-systems/:id/sync-apps`, `/app-usage`. CRUD dùng GET/POST/PUT/DELETE. `/health` kiểm tra process; `/ready` kiểm tra kết nối DB.

## Tương thích database

Giữ nguyên primary key, UUID, foreign key và bảng pivot. `ensure` dùng advisory transaction lock vì schema hiện tại không còn unique constraint endpoint/db_name. `sync-apps` là atomic, bảo toàn setting bị bỏ qua bởi client cũ và kiểm tra phiên bản sau khi ghép giá trị mặc định. SQL identifier động chỉ đến từ Resource enum và allowlist, các giá trị đều dùng bind parameter.

Schema thật cho phép nhiều cột nullable hơn file Laravel migration. Frontend chuẩn hóa boolean null về false và chuỗi nullable về giá trị hiển thị hợp lệ. `module_app_features` dùng cột `feature`; Rust đọc đúng cột này thay cho thuộc tính `name` được controller PHP gọi trước đó.

Session mới dùng cookie `o4u_session`, payload riêng và ID prefix `o4u:` trong bảng sessions. Không tiếp tục cookie Laravel cũ; người dùng đăng nhập lại sau khi chuyển hệ thống. Rate limit đăng ký dùng key SHA256 prefix `o4u-rust-rate:` trong bảng cache cũ, hỗ trợ nhiều process và restart. Giới hạn mới tính mỗi lần thử đăng ký theo email, thay vì chỉ đếm đăng ký thành công như controller cũ.

Runtime không chạy migration, không import fixture, không đổi schema. `export-schema.py` chỉ dùng `pg_dump --schema-only --no-owner --no-privileges` để tạo fixture kiểm thử. Không chứa dữ liệu khách hàng hoặc credentials trong fixture.

## Xác minh

Đã chạy domain/use-case/crypto tests, integration test trên PostgreSQL riêng với bản sao schema thật, Clippy với warnings-as-errors, ESLint, Prettier, strict TypeScript, Vitest và Vite production build. Browser smoke kiểm tra 8 trang chính, tạo/sửa/xóa khách hàng, lưu và tải lại app overrides, 4 trang chính sách và preview desktop/mobile.

Test tích hợp dùng dữ liệu giả; không gọi Google/Cloudflare/Odoo/Firebase/Telegram thật. Cần kiểm tra các tích hợp bằng cấu hình môi trường triển khai trước khi chuyển traffic. Không có provisioning tenant hay gửi thông báo thật trong quá trình chuyển source.

## Chuyển traffic khi triển khai sau này

1. Build backend/frontend độc lập, đặt env mới trỏ đúng DB O4U cũ.
2. Kiểm tra Google callback, Turnstile, Odoo registration endpoint, Firebase credential path và Telegram config.
3. Chạy Rust API + Rust worker; giữ Laravel worker khỏi queue `o4u-rust-fcm`.
4. Cấu hình hosting phục vụ `frontend/dist`, proxy API/auth/register tới Rust như `frontend/deploy/nginx.conf`.
5. Kiểm tra `/ready`, Google login, CRUD với tài khoản test và API mobile; sau đó mới chuyển traffic.

Source cũ vẫn ở nguyên vị trí để đối chiếu và chạy lại với cấu hình hosting cũ. Nếu từng chạy hai hệ thống cùng lúc, chúng chia sẻ dữ liệu nghiệp vụ theo đúng yêu cầu; thao tác ghi từ app nào cũng nhìn thấy bởi app kia.
