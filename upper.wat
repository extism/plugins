;; Lovingly hand-crafted WAT "uppercase" module. Exports a single function that takes a
;; pointer to extism memory, consumes all bytes from that position, then returns them uppercased.
(module
    (import "extism:host/env" "length" (func $length (param i64) (result i64)))
    (import "extism:host/env" "load_u64" (func $load_u64 (param i64) (result i64)))
    (import "extism:host/env" "load_u8" (func $load_u8 (param i64) (result i32)))
    (import "extism:host/env" "store_u64" (func $store_u64 (param i64 i64)))
    (import "extism:host/env" "store_u8" (func $store_u8 (param i64 i32)))

    (func (export "host_reflect") (param $extism_offset i64) (result i64)
        (local $len i64)
        (local $offset i64)
        (local $len64 i64)
        (local $charas i64)

        (local.set $offset (i64.const 0))
        (local.set $len (call $length (local.get $extism_offset)))

        (local.set $len64 (i64.shl (i64.shr_u (local.get $len) (i64.const 16)) (i64.const 16)))

        (loop $to_upper
            (local.set $charas (call $load_u64 (i64.add (local.get $extism_offset) (local.get $offset))))

            ;; nobody cared what charas these were... until they put on the mask
            ;; seriously though. grab each byte, subtract 96, div_u by 27 so that 0 == needs mask and all other numbers don't, eqz to flip the 0 to a 1, then
            ;; shl the result to the proper position. finally, invert
            (call $store_u64
              (i64.add (local.get $extism_offset) (local.get $offset))
              (i64.and
                (local.get $charas)
                (i64.xor
                  (i64.or
                    (i64.shl
                      (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (local.get $charas)) (i64.const 96)) (i64.const 27))))
                      (i64.const 5)) ;; 5
                    (i64.or
                      (i64.shl
                        (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (i64.shr_u (local.get $charas) (i64.const 8))) (i64.const 96)) (i64.const 27))))
                        (i64.const 13)) ;; 5 + 8
                      (i64.or
                        (i64.shl
                          (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (i64.shr_u (local.get $charas) (i64.const 16))) (i64.const 96)) (i64.const 27))))
                          (i64.const 21)) ;; 5 + 16
                        (i64.or
                          (i64.shl
                            (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (i64.shr_u (local.get $charas) (i64.const 24))) (i64.const 96)) (i64.const 27))))
                            (i64.const 29)) ;; 5 + 24
                          (i64.or
                            (i64.shl
                              (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (i64.shr_u (local.get $charas) (i64.const 32))) (i64.const 96)) (i64.const 27))))
                              (i64.const 37)) ;; 5 + 32
                            (i64.or
                              (i64.shl
                                (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (i64.shr_u (local.get $charas) (i64.const 40))) (i64.const 96)) (i64.const 27))))
                                (i64.const 45)) ;; 5 + 40
                              (i64.or
                                (i64.shl
                                  (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (i64.shr_u (local.get $charas) (i64.const 48))) (i64.const 96)) (i64.const 27))))
                                  (i64.const 53)) ;; 5 + 48
                                (i64.shl
                                  (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (i64.shr_u (local.get $charas) (i64.const 56))) (i64.const 96)) (i64.const 27))))
                                  (i64.const 61)) ;; 5 + 56
                              )
                            )
                          )
                        )
                      )
                    )
                  )
                  (i64.const -1)
                )
              )
            )

            (local.set $offset (i64.add (local.get $offset) (i64.const 8)))
            (br_if $to_upper (i64.lt_u (local.get $offset) (local.get $len64)))
        )

        (if (i64.ne (local.get $len64) (local.get $len)) (then
          (loop $to_upper
              (local.set $charas (i64.extend_i32_u (call $load_u8 (i64.add (local.get $extism_offset) (local.get $offset)))))
              (call $store_u8
                (i64.add (local.get $extism_offset) (local.get $offset))
                (i32.wrap_i64 (i64.and
                  (local.get $charas)
                  (i64.xor
                    (i64.shl
                      (i64.extend_i32_u (i64.eqz (i64.div_u (i64.sub (i64.and (i64.const 0xff) (local.get $charas)) (i64.const 96)) (i64.const 27))))
                      (i64.const 5) ;; 5
                    )
                    (i64.const -1)
                  )
                ))
              )

              (local.set $offset (i64.add (i64.const 1) (local.get $offset)))
              (br_if $to_upper (i64.lt_u (local.get $offset) (local.get $len)))
          )
        ))
        local.get $extism_offset
    )
)
