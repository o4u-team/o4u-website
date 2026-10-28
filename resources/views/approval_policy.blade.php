<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>O4U Approval - Chính sách quyền riêng tư &amp; Điều khoản sử dụng</title>

    <link href="https://cdn.jsdelivr.net/npm/normalize.css@8.0.1/normalize.min.css" rel="stylesheet">

    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", sans-serif;
            padding: 1em;
            background-color: #0a0f1e;
            color: #e8eaf0;
            line-height: 1.55;
        }

        .container {
            padding: 1.25em;
            border-radius: 14px;
            background-color: #121829;
            max-width: 980px;
            margin: 0 auto;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.35);
            border: 1px solid rgba(232, 234, 240, 0.08);
        }

        .center { text-align: center; }

        h1 {
            margin: 0 0 6px;
            font-size: 1.6rem;
            letter-spacing: -0.01em;
            color: #e8eaf0;
        }

        h2 {
            margin: 22px 0 8px;
            font-size: 1.08rem;
            color: #e8eaf0;
        }

        .subtitle {
            margin: 0 0 18px;
            color: rgba(232, 234, 240, 0.65);
            font-size: 0.98rem;
        }

        ul { padding-left: 1.15em; }
        li { margin: 6px 0; }

        a { color: #6ee7b7; }
        a:visited { color: #5eead4; }
        a:hover { color: #a7f3d0; }

        .meta {
            display: inline-block;
            padding: 6px 10px;
            border-radius: 999px;
            background: rgba(110, 231, 183, 0.1);
            border: 1px solid rgba(110, 231, 183, 0.25);
            color: #a7f3d0;
            font-size: 0.9rem;
            margin: 6px 0 0;
        }

        .section { margin-top: 16px; }
        .divider { height: 1px; background: rgba(232, 234, 240, 0.1); margin: 18px 0; }

        .notice {
            padding: 12px 14px;
            border-radius: 10px;
            background: rgba(110, 231, 183, 0.08);
            border: 1px solid rgba(110, 231, 183, 0.2);
            color: #e8eaf0;
            font-size: 0.95rem;
        }

        strong { color: #f3f4f6; }
    </style>
</head>
<body>
<main class="container">
    <div class="center">
        <h1>Chính sách quyền riêng tư &amp; Điều khoản sử dụng</h1>
        <p class="subtitle">Ứng dụng <strong>Phê duyệt</strong></p>
        <div class="meta">Cập nhật lần cuối: <strong>28/10/2026</strong></div>
    </div>
    <div class="divider"></div>
    <p><strong>Ứng dụng Phê duyệt</strong> (sau đây gọi là “Ứng dụng”) do <strong>O4U Team</strong> phát triển, giúp người dùng xem hồ sơ chờ duyệt, phê duyệt hoặc từ chối, theo dõi lịch sử xử lý và nhận thông báo từ hệ thống <strong>Odoo</strong> của doanh nghiệp.</p>
    <div class="notice section">Dữ liệu tài khoản và hồ sơ phê duyệt được xử lý trên hệ thống Odoo mà doanh nghiệp cấu hình cho Ứng dụng. Doanh nghiệp chịu trách nhiệm quản lý dữ liệu nghiệp vụ, phân quyền và thời gian lưu trữ. Ứng dụng cũng lưu một số dữ liệu trên thiết bị và sử dụng dịch vụ thông báo đẩy như mô tả bên dưới.</div>

    <section class="section">
        <h2>1. Vai trò của O4U Team và doanh nghiệp triển khai</h2>
        <ul>
            <li><strong>O4U Team:</strong> phát triển phần mềm và hỗ trợ kỹ thuật. Việc truy cập hệ thống doanh nghiệp để hỗ trợ được thực hiện theo phạm vi doanh nghiệp cho phép.</li>
            <li><strong>Doanh nghiệp triển khai:</strong> quản lý máy chủ Odoo, tài khoản, hồ sơ, quy trình phê duyệt, người được truy cập và việc lưu trữ dữ liệu. Vui lòng liên hệ bộ phận IT hoặc bộ phận quản lý nghiệp vụ để biết chính sách áp dụng tại doanh nghiệp của bạn.</li>
        </ul>
    </section>
    <section class="section">
        <h2>2. Dữ liệu được xử lý</h2>
        <ul>
            <li><strong>Kết nối và tài khoản:</strong> địa chỉ máy chủ Odoo, cơ sở dữ liệu, thông tin đăng nhập, mã phiên truy cập và làm mới phiên; hồ sơ tài khoản có thể gồm họ tên, email, số điện thoại, công ty, phòng ban và chức danh.</li>
            <li><strong>Dữ liệu nghiệp vụ:</strong> nội dung hồ sơ, người đề nghị, người phê duyệt, trạng thái, thời gian xử lý, quyết định duyệt/từ chối, lý do từ chối, mẫu lý do và tài liệu đính kèm được phép xem.</li>
            <li><strong>Thông báo:</strong> nội dung thông báo, trạng thái đã đọc, mã thông báo đẩy (FCM token), nền tảng thiết bị và thông tin liên kết thiết bị với tài khoản để chuyển thông báo đúng người dùng.</li>
            <li><strong>Đăng nhập nhanh:</strong> khi bạn bật sinh trắc học, hệ thống xử lý mã thiết bị và thông tin xác thực đăng nhập nhanh. Mẫu khuôn mặt hoặc vân tay do hệ điều hành quản lý; Ứng dụng chỉ nhận kết quả xác thực, không nhận hoặc gửi mẫu sinh trắc học về Odoo.</li>
            <li><strong>Mã QR và tệp:</strong> nội dung mã QR dùng để thiết lập kết nối; tài liệu được tải xuống hoặc chuyển tới ứng dụng khác khi bạn chủ động sử dụng tính năng chia sẻ.</li>
        </ul>
    </section>
    <section class="section">
        <h2>3. Mục đích sử dụng dữ liệu</h2>
        <ul>
            <li>Xác thực người dùng, duy trì phiên đăng nhập và hiển thị dữ liệu theo phân quyền.</li>
            <li>Thực hiện quyết định phê duyệt hoặc từ chối, tra cứu hồ sơ và lịch sử xử lý.</li>
            <li>Gửi thông báo, cập nhật trạng thái đã đọc và nhắc xử lý hồ sơ.</li>
            <li>Lưu tùy chọn tài khoản, cấu hình kết nối và hỗ trợ đăng nhập nhanh khi bạn bật.</li>
        </ul>
        <p>O4U Team không bán dữ liệu cá nhân hoặc sử dụng nội dung hồ sơ phê duyệt cho quảng cáo.</p>
    </section>
    <section class="section">
        <h2>4. Quyền truy cập trên thiết bị</h2>
        <ul>
            <li><strong>Internet:</strong> kết nối tới hệ thống Odoo và dịch vụ thông báo.</li>
            <li><strong>Thông báo:</strong> hiển thị thông báo về hồ sơ và nhắc xử lý. Bạn có thể tắt trong cài đặt thiết bị; nội dung thông báo có thể hiển thị trên màn hình khóa tùy cài đặt của bạn.</li>
            <li><strong>Camera:</strong> quét mã QR để cấu hình kết nối Odoo khi bạn mở chức năng quét.</li>
            <li><strong>Ảnh:</strong> nếu phiên bản ứng dụng cung cấp chọn ảnh QR, ảnh được chọn dùng để đọc mã kết nối.</li>
            <li><strong>Face ID / vân tay:</strong> xác nhận đăng nhập nhanh khi bạn tự bật tính năng trên thiết bị hỗ trợ. Bạn có thể tắt trong phần cài đặt tài khoản của Ứng dụng.</li>
            <li><strong>Tải và chia sẻ tệp:</strong> lưu tệp phục vụ tải xuống/chia sẻ và mở bảng chia sẻ của hệ điều hành theo thao tác của bạn.</li>
        </ul>
        <p>Bạn có thể từ chối hoặc thu hồi các quyền tùy chọn trong cài đặt thiết bị; tính năng tương ứng có thể không hoạt động. Phiên bản hiện tại không yêu cầu quyền vị trí, danh bạ hoặc micro.</p>
    </section>
    <section class="section">
        <h2>5. Lưu trữ trên thiết bị</h2>
        <p>Ứng dụng lưu cấu hình kết nối và dữ liệu duy trì phiên đăng nhập trên thiết bị, bao gồm mã truy cập và thông tin tài khoản. Thông tin xác thực đăng nhập nhanh được lưu trong kho bảo mật của hệ điều hành.</p>
        <p>Đăng xuất sẽ xóa dữ liệu phiên đăng nhập do Ứng dụng quản lý và yêu cầu gỡ liên kết thông báo với tài khoản. Nếu thiết bị mất mạng, việc gỡ liên kết phía máy chủ có thể chưa hoàn tất. Tắt đăng nhập nhanh riêng trong cài đặt tài khoản nếu muốn thu hồi chức năng này. Các tệp bạn đã lưu hoặc chia sẻ cần được quản lý tại nơi lưu hoặc ứng dụng nhận tệp.</p>
    </section>
    <section class="section">
        <h2>6. Bên nhận dữ liệu và dịch vụ bên thứ ba</h2>
        <ul>
            <li><strong>Hệ thống Odoo của doanh nghiệp:</strong> tiếp nhận thông tin đăng nhập và xử lý dữ liệu nghiệp vụ. Người dùng khác chỉ được truy cập theo phân quyền doanh nghiệp thiết lập.</li>
            <li><strong>Firebase Cloud Messaging (Google):</strong> cung cấp mã thông báo và chuyển thông báo đẩy. Dịch vụ có thể xử lý mã cài đặt, token và dữ liệu kỹ thuật phục vụ chuyển thông báo. Xem <a href="https://firebase.google.com/support/privacy" target="_blank" rel="noopener noreferrer">chính sách quyền riêng tư Firebase</a>.</li>
            <li><strong>Dịch vụ thông báo của Apple:</strong> được sử dụng để chuyển thông báo trên iOS. Xem <a href="https://www.apple.com/legal/privacy/" target="_blank" rel="noopener noreferrer">chính sách quyền riêng tư Apple</a>.</li>
            <li><strong>Ứng dụng nhận chia sẻ:</strong> nhận tài liệu bạn chủ động chia sẻ và xử lý theo chính sách của ứng dụng đó. Hãy kiểm tra người nhận trước khi chia sẻ tài liệu nội bộ.</li>
        </ul>
    </section>
    <section class="section">
        <h2>7. Thời gian lưu trữ và yêu cầu xóa dữ liệu</h2>
        <p>Tài khoản, hồ sơ và lịch sử phê duyệt được lưu theo chính sách doanh nghiệp và nhu cầu lưu vết nghiệp vụ. Đăng xuất hoặc gỡ cài đặt không tự động xóa dữ liệu trên Odoo.</p>
        <p>Để yêu cầu xem, chỉnh sửa, xóa dữ liệu, đóng tài khoản hoặc thu hồi liên kết thiết bị, hãy liên hệ quản trị viên Odoo / bộ phận IT của doanh nghiệp. Một số hồ sơ có thể cần được giữ lại theo nghĩa vụ lưu trữ áp dụng; doanh nghiệp sẽ giải thích phạm vi và thời hạn xử lý yêu cầu.</p>
        <p>Bạn cũng có thể gửi yêu cầu hỗ trợ tới <a href="mailto:nguyenvannhan0810@gmail.com">nguyenvannhan0810@gmail.com</a>, nêu tên ứng dụng, doanh nghiệp và tài khoản cần hỗ trợ. Không gửi mật khẩu, mã truy cập hoặc tài liệu nội bộ nhạy cảm qua email.</p>
    </section>
    <section class="section">
        <h2>8. Bảo mật</h2>
        <p>Doanh nghiệp cần cấu hình kết nối HTTPS, phân quyền phù hợp và quản lý an toàn máy chủ Odoo. Người dùng cần bảo vệ thiết bị, không chia sẻ thông tin đăng nhập và chỉ kết nối tới máy chủ đáng tin cậy. Kho bảo mật của hệ điều hành được dùng cho thông tin xác thực đăng nhập nhanh; việc xác thực khuôn mặt/vân tay do hệ điều hành thực hiện.</p>
        <p>Khi nghi ngờ tài khoản bị truy cập trái phép, hãy liên hệ quản trị viên để thu hồi phiên và thiết bị đăng nhập.</p>
    </section>
    <section class="section">
        <h2>9. Điều khoản sử dụng</h2>
        <ul>
            <li>Chỉ sử dụng tài khoản được doanh nghiệp cấp và thực hiện thao tác trong phạm vi được ủy quyền.</li>
            <li>Kiểm tra nội dung và tài liệu trước khi phê duyệt hoặc từ chối. Quyết định được ghi nhận vào quy trình của doanh nghiệp; yêu cầu điều chỉnh cần được xử lý theo quy trình nội bộ.</li>
            <li>Không truy cập trái phép, chia sẻ tài liệu ngoài phạm vi cho phép hoặc làm gián đoạn hệ thống.</li>
            <li>Khả năng sử dụng phụ thuộc kết nối mạng, máy chủ Odoo và cấu hình doanh nghiệp. Khi thao tác báo lỗi hoặc chưa rõ kết quả, hãy kiểm tra lại trạng thái hồ sơ.</li>
        </ul>
    </section>
    <section class="section">
        <h2>10. Trẻ em</h2>
        <p>Ứng dụng phục vụ công việc nội bộ doanh nghiệp, không hướng tới trẻ em. Nếu phát hiện dữ liệu trẻ em được đưa vào hệ thống không phù hợp, vui lòng liên hệ doanh nghiệp quản lý dữ liệu để được xử lý.</p>
    </section>
    <section class="section">
        <h2>11. Thay đổi chính sách</h2>
        <p>Chính sách được cập nhật khi tính năng hoặc cách xử lý dữ liệu thay đổi. Phiên bản mới và ngày cập nhật sẽ được công bố trên trang này. Thay đổi cần quyền thiết bị bổ sung sẽ được yêu cầu tại tính năng liên quan.</p>
    </section>
    <section class="section">
        <h2>12. Thông tin liên hệ</h2>
        <p><strong>Hỗ trợ kỹ thuật ứng dụng: O4U Team</strong><br>
            Email: <a href="mailto:nguyenvannhan0810@gmail.com">nguyenvannhan0810@gmail.com</a><br>
            Website: <a href="https://o4u.nvnhan0810.com">https://o4u.nvnhan0810.com</a></p>
        <p><strong>Dữ liệu cá nhân, tài khoản và hồ sơ phê duyệt:</strong> liên hệ quản trị viên Odoo hoặc bộ phận IT của doanh nghiệp triển khai.</p>
    </section>
</main>
</body>
</html>
