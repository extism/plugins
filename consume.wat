;; Lovingly hand-crafted WAT "consume" module. Exports a single function that consumes
;; all bytes from extism input and writes them to local memory. Returns an empty string.
(module
    (import "extism:host/env" "input_length" (func $input_length (result i64)))
    (import "extism:host/env" "input_load_u64" (func $input_load_u64 (param i64) (result i64)))
    (import "extism:host/env" "input_load_u8" (func $input_load_u8 (param i64) (result i32)))
    (import "extism:host/env" "store_u64" (func $store_u64 (param i64 i64)))
    (import "extism:host/env" "store_u8" (func $store_u8 (param i64 i32)))
    (import "extism:host/env" "alloc" (func $alloc (param i64) (result i64)))
    (import "extism:host/env" "output_set" (func $output_set (param i64 i64)))

    (memory (export "memory") 0)
    (func (export "consume") (result i32)
        (local $len i64)
        (local $offset i64)
        (local $len64 i64)
        (local $output i64)

        (local.set $offset (i64.const 0))
        (local.set $len (call $input_length))

        (memory.grow
            (i32.wrap_i64
              (i64.add
                  (i64.shr_u (local.get $len) (i64.const 16))
                  (i64.extend_i32_u (i64.ne (i64.and (local.get $len) (i64.const 65535)) (i64.const 0)))
              )
            )
        )
        drop ;; we're optimistically assuming that growing memory worked

        (local.set $len64 (i64.shl (i64.shr_u (local.get $len) (i64.const 16)) (i64.const 16)))
        (local.set $output (call $alloc (i64.const 0)))
        (call $output_set (local.get $output) (i64.const 0))

        (loop $load_u64s
            (i64.store (i32.wrap_i64 (local.get $offset)) (call $input_load_u64 (local.get $offset)))
            (local.set $offset (i64.add (local.get $offset) (i64.const 8)))
            (br_if $load_u64s (i64.lt_u (local.get $offset) (local.get $len64)))
        )

        (if (i64.ne (local.get $len64) (local.get $len)) (then
          (loop $load_u8s
              (i32.store8 (i32.wrap_i64 (local.get $offset)) (call $input_load_u8 (local.get $offset)))
              (local.set $offset (i64.add (i64.const 1) (local.get $offset)))
              (br_if $load_u8s (i64.lt_u (local.get $offset) (local.get $len)))
          )
        ))

        i32.const 0
    )
)
