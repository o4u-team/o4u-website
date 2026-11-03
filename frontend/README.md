# O4U Frontend

Vue 3 SPA độc lập với Vue Router, TypeScript strict, Vuetify, Vite, ESLint, Prettier và Vitest. Tham khảo cách chia lớp ở `nvnhan0810.com/foreign` và `nvnhan0810.com/wallets`.

- `src/domain`: entity/types và quy tắc marketing, không phụ thuộc Vue/HTTP.
- `src/application`: use case, repository port, pagination cho danh sách lựa chọn.
- `src/infrastructure`: HTTP repository, kiểm tra response bằng Zod.
- `src/presentation`: pages, layouts, components và composables.
- `src/app`: composition root, router, dependency wiring, Vuetify.

ESLint chặn import Vue hoặc infrastructure từ domain/application. TypeScript bật `strict`, `noUncheckedIndexedAccess`, `noUnusedLocals`, `noUnusedParameters`; không dùng `any` hoặc `@ts-ignore`. Response API được kiểm tra runtime; dữ liệu nullable trong schema cũ được chuẩn hóa tại HTTP boundary.

## Chạy SSR

```sh
cd frontend
npm ci
npm run dev
```

Yêu cầu Node 22.18+ hoặc 24.12+. Node server render Vue thành HTML cho từng request rồi hydrate trang trong trình duyệt. Mở **http://localhost:5173** và chạy Rust backend tại `127.0.0.1:8088`. Vite proxy `/api`, `/auth`, `/erp-saas/register` tới backend. Có thể copy `.env.example` thành `.env` để đổi `BACKEND_URL`.

Chỉ đặt `VITE_API_BASE_URL` khi cần base URL API riêng. Không đặt DB password, OAuth secret hoặc khóa tích hợp vào biến `VITE_*`; các biến này được đóng gói vào JavaScript public. Cấu hình cùng origin được ưu tiên để session cookie hoạt động nhất quán.

## Kiểm tra và build

```sh
npm run check
npm run format
npm run build
```

`npm run build` tạo client assets trong `dist/` và SSR bundle trong `dist-ssr/`. Chạy production bằng `npm start` (Node SSR server). Reverse proxy chuyển page requests tới Node server và có thể phục vụ assets trực tiếp từ `dist/`.

Các trang: `/`, `/erp-saas`, `/login`, `/dashboard`, `/clients`, `/apps`, `/client-systems`, `/client-systems/:id/app-info`, `/app-usage`. Các trang chính sách giữ nội dung cũ tại `/kho-barcode`, `/cong-dat-hang`, `/hr-cham-cong`, `/approval`; HTML bản sao ở `public/policies`.

Landing, ERP và các component Zalo được chuyển từ Vue cũ. Quản trị dùng REST/use case; không còn Inertia, Laravel Vite plugin hoặc PHP runtime. Các trang danh sách hỗ trợ lọc/phân trang; form hiển thị lỗi API; thao tác xóa cần xác nhận; cấu hình app cho phép kế thừa mặc định theo từng trường.

Tham khảo [Vue + TypeScript](https://vuejs.org/guide/typescript/overview.html).
