# AGENTS.md

## Mục tiêu repo

Repo này dùng để học Rust theo hướng thực hành. Ưu tiên giải thích rõ khái niệm, viết ví dụ nhỏ, chạy thử, rồi rút ra bài học.

## Ngôn ngữ phản hồi

- Trả lời bằng tiếng Việt, trừ khi người dùng yêu cầu ngôn ngữ khác.
- Giải thích thuật ngữ Rust bằng tiếng Việt kèm tên gốc tiếng Anh khi hữu ích, ví dụ: quyền sở hữu (ownership), mượn (borrowing), vòng đời (lifetime).
- Khi sửa code, nói ngắn gọn: đã sửa gì, vì sao, chạy lệnh nào để kiểm tra.

## Quy ước học Rust

- Ưu tiên code đơn giản, dễ đọc, phù hợp người đang học.
- Không dùng crate ngoài nếu Rust standard library đủ dùng.
- Khi giới thiệu tính năng mới, thêm ví dụ nhỏ và giải thích lỗi thường gặp.
- Với bài tập, giữ thay đổi tập trung trong crate liên quan.
- Nếu code có lỗi biên dịch, giải thích nguyên nhân theo compiler message trước khi sửa.

## Cấu trúc hiện tại

- `hello_world/`: crate nhập môn, dùng cho ví dụ cơ bản.
- `rust_testing/`: crate dùng cho kiểm thử và ví dụ nâng cao hơn.

## Lệnh thường dùng

- `cargo run`: chạy crate hiện tại.
- `cargo test`: chạy test crate hiện tại.
- `cargo check`: kiểm tra biên dịch nhanh.
- `cargo fmt`: định dạng code Rust.
- `cargo clippy`: lint Rust nếu đã cài Clippy.

Chạy lệnh trong đúng thư mục crate, ví dụ:

```powershell
cd hello_world
cargo run
```

## Quy ước chỉnh sửa

- Giữ thay đổi nhỏ, rõ mục tiêu học.
- Không xóa bài cũ nếu người dùng chưa yêu cầu; nếu cần thay, hãy giải thích.
- Không tạo project mới nếu có thể dùng `hello_world/` hoặc `rust_testing/`.
- Không sửa file ngoài phạm vi bài học trừ khi cần thiết.

## Quy ước commit

- Chỉ commit khi người dùng yêu cầu rõ ràng.
- Người dùng có thể nói “commit”, “tự commit”, “cam kết”, hoặc “lưu commit”; tất cả đều nghĩa là tạo Git commit.
- Trước khi commit, luôn chạy `git status --short` và tóm tắt file sẽ commit.
- Nếu có thay đổi không liên quan tới yêu cầu hiện tại, hỏi người dùng trước khi đưa vào commit.
- Commit message dùng tiếng Việt, ngắn gọn, dạng: `học rust: <nội dung>`.
- Không push lên remote trừ khi người dùng yêu cầu rõ.

## Khi người dùng hỏi bài học

1. Nêu mục tiêu bài học.
2. Chỉ ra file sẽ sửa hoặc tạo.
3. Viết code mẫu tối thiểu.
4. Gợi ý lệnh chạy.
5. Giải thích kết quả mong đợi.
