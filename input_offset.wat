;; Lovingly hand-crafted WAT input_offset_length module. Reads the input_offset, queries
;; the length, allocates 8 bytes, stores the length, and outputs.
(module
    (import "extism:host/env" "input_offset" (func $input_offset (result i64)))
    (import "extism:host/env" "length" (func $length (param i64) (result i64)))
    (import "extism:host/env" "store_u64" (func $store_u64 (param i64 i64)))
    (import "extism:host/env" "alloc" (func $alloc (param i64) (result i64)))
    (import "extism:host/env" "output_set" (func $output_set (param i64 i64)))

    (memory (export "memory") 0)
    (func (export "input_offset_length") (result i32)
        (local $output i64)
        (local.set $output (call $alloc (i64.const 8)))
        (call $store_u64 (local.get $output) (call $length (call $input_offset)))
        (call $output_set (local.get $output) (i64.const 8))
        i32.const 0
    )
)
