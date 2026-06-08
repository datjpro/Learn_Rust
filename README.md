# Learn_Rust

Repo học Rust bằng thực hành.

## Mục tiêu

- Làm quen cú pháp Rust qua ví dụ nhỏ.
- Hiểu các khái niệm cốt lõi: ownership, borrowing, lifetimes, structs, enums, pattern matching, error handling.
- Tập viết test bằng `cargo test`.
- Ghi lại tiến độ bằng Git commit khi cần.

## Cấu trúc

- `hello_world/`: bài nhập môn và ví dụ cơ bản.
- `rust_testing/`: ví dụ về kiểm thử và bài tập có test.
- `AGENTS.md`: hướng dẫn cho Codex/agent khi hỗ trợ trong repo này.

## Bắt đầu nhanh

Yêu cầu cần có Rust toolchain. Kiểm tra bằng:

```powershell
rustc --version
cargo --version
```

Chạy project đầu tiên:

```powershell
cd hello_world
cargo run
```

Chạy test:

```powershell
cd rust_testing
cargo test
```

## Lộ trình đề xuất

1. `hello_world`: biến, kiểu dữ liệu, hàm, control flow.
2. `hello_world`: ownership, borrowing, reference.
3. `hello_world`: struct, enum, match.
4. `rust_testing`: viết function nhỏ kèm unit test.
5. `rust_testing`: xử lý lỗi với `Result` và `Option`.
6. `rust_testing`: module, visibility, integration test.

## Cách nhờ Codex học cùng

Ví dụ prompt:

- `Dạy tôi bài ownership bằng ví dụ trong hello_world`.
- `Tạo bài tập Rust về enum và match`.
- `Giải thích lỗi compiler này bằng tiếng Việt`.
- `Viết test cho hàm hiện tại`.
- `Tự commit thay đổi này`.

## Quy ước commit

- Chỉ commit khi bạn yêu cầu rõ.
- Từ “commit” trong repo này nghĩa là tạo Git commit; có thể hiểu là “cam kết/lưu mốc thay đổi”.
- Commit message nên dùng tiếng Việt, ví dụ: `học rust: thêm bài ownership`.
- Không push lên remote nếu chưa yêu cầu.
