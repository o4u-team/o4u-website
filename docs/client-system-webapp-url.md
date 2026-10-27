# WebApp URL theo client system và app

Trong Client Systems → Phân quyền App, mỗi app được chọn có thể cấu hình WebApp URL riêng (HTTP/HTTPS, tối đa 255 ký tự). Để trống để dùng `apps.webapp_url`.

Cấu hình được lưu tại `app_client_system.webapp_url`, độc lập cho từng cặp client system–app.

Hai API trả thêm `data.webapp_url`:

- `POST /api/app/client-systems/ensure`: áp dụng cả khi tạo mới và dùng lại client system. Header `X-App-Id` xác định app.
- `GET /api/app-info`: header `X-App-Id` và `X-Client-System-Id` xác định cặp cấu hình; vẫn yêu cầu `allow_get_info`.

URL riêng được ưu tiên, sau đó dùng URL của app. Nếu cả hai chưa cấu hình, trả `null`. Field `webapp_endpoint` hiện có không thay đổi.

Endpoint quản trị `POST /client-systems/{id}/sync-apps` nhận `app_ids` và `webapp_urls` (object có key là app ID). Gửi URL rỗng hoặc `null` để xóa cấu hình riêng; bỏ qua key để giữ nguyên cấu hình đã lưu.

Khi triển khai, chạy `php artisan migrate --force` trước khi sử dụng phiên bản mới.

## Phiên bản và store URL riêng

Ngoài `webapp_url`, mỗi cặp hỗ trợ `android_min_version`, `android_current_version`, `ios_min_version`, `ios_current_version`, `android_store_url`, `apple_store_url`. Mỗi trường để trống dùng giá trị từ app. Không có cấu hình riêng cho status/allow_public.

Ensure và app-info trả đủ các trường trên trong `data`. API `GET /api/app/version` nhận thêm header tùy chọn `X-Client-System-Id` để dùng cấu hình riêng và tính `force_update`; hệ thống sẽ kiểm tra client system active và app đã được cấp quyền. Không gửi header này thì dùng cấu hình app như trước.

Endpoint sync-apps nhận thêm `app_settings` (object key là app ID, value là object chứa các trường cấu hình). Bỏ qua trường để giữ giá trị đã lưu; gửi rỗng/null để dùng mặc định. Phiên bản có định dạng `x.y.z`, phiên bản tối thiểu không được lớn hơn hiện tại sau khi áp dụng giá trị mặc định. `webapp_urls` vẫn được hỗ trợ cho client quản trị cũ.
