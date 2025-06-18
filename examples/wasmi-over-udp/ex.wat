(module $wasm_example.wasm
  (type (;0;) (func (param i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (import "host" "host_hello" (func $_ZN12wasm_example10host_hello17hb4200c7edf76a1d3E (type 0)))
  (func $add (type 1) (param i32 i32) (result i32)
    i32.const 32
    call $_ZN12wasm_example10host_hello17hb4200c7edf76a1d3E
    local.get 1
    local.get 0
    i32.add)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 2 3)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "add" (func $add))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
