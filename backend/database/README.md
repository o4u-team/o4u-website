# Khởi tạo database O4U

`init.sql` là cấu trúc PostgreSQL xuất trực tiếp từ database đang sử dụng ngày 2026-11-03 bằng `pg_dump --schema-only --no-owner --no-privileges --no-tablespaces`. File giữ bảng, sequence, default, index và constraint; không có dữ liệu, role hay quyền truy cập riêng của server cũ.

## Khởi tạo database trống

Cần PostgreSQL/psql 18 hoặc phiên bản mới hơn tương thích. Tạo role và database theo môi trường triển khai trước, rồi import với role sở hữu database:

```sh
psql -h 127.0.0.1 -p 5432 -U o4u -d o4u --no-psqlrc --set ON_ERROR_STOP=1 --single-transaction --file backend/database/init.sql
```

Lệnh trên chạy từ thư mục gốc repository. Chỉ import vào database trống; file không phải migration nâng cấp và không được chạy lại trên database đang có bảng. Backend không tự import khi khởi động. Sau đó cấu hình `backend/.env` trỏ đến database vừa tạo.

Snapshot không chứa tài khoản quản trị, API key, cấu hình app hoặc lịch sử migration. Google login chỉ chấp nhận email đã tồn tại trong `users`; khi tạo database mới cần bổ sung tài khoản quản trị và dữ liệu cấu hình cần thiết. Nếu cần giữ dữ liệu hiện tại, phải có bản backup dữ liệu riêng.

## Xuất lại cấu trúc

```sh
python3 backend/scripts/export-schema.py
```

Script dùng `backend/.env` (`DATABASE_URL` hoặc `DB_*`), không phụ thuộc `.env`, PHP hay migration của source Laravel cũ. Biến môi trường của process được ưu tiên. Script chỉ đọc database; thay `init.sql` sau khi export thành công. Có thể chỉ định `--env-file /path/to/.env` và `--output /path/to/schema.sql`.

Khi thay đổi schema sau snapshot này, lưu migration SQL mới trong backend và áp dụng theo thứ tự; không dùng `init.sql` để cập nhật database hiện hữu.
