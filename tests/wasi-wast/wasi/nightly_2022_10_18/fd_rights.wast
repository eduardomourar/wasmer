;; This file was generated by https://github.com/wasmerio/wasi-tests

(wasi_test "fd_rights.wasm"
  (assert_return (i64.const 101))
  (assert_stderr "thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 30, kind: ReadOnlyFilesystem, message: \"Read-only file system\" }', /Users/fs/Development/wasmer/tests/wasi-wast/wasi/tests/fd_rights.rs:9:10\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n")
)
