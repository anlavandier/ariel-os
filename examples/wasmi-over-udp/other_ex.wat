(module $wasm_example.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $main (type 0) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 3
      i32.and
      local.set 2
      i32.const 0
      local.set 3
      block  ;; label = @2
        local.get 0
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        i32.const -4
        i32.and
        local.set 4
        i32.const 0
        local.set 3
        loop  ;; label = @3
          local.get 1
          local.get 3
          i32.store offset=12
          local.get 1
          i32.const 12
          i32.add
          local.set 0
          local.get 1
          local.get 3
          i32.const 1
          i32.add
          i32.store offset=12
          local.get 1
          local.get 3
          i32.const 2
          i32.add
          i32.store offset=12
          local.get 1
          local.get 3
          i32.const 3
          i32.add
          i32.store offset=12
          local.get 3
          i32.const 4
          i32.add
          local.tee 3
          local.get 4
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 1
        local.get 3
        i32.store offset=12
        local.get 1
        i32.const 12
        i32.add
        local.set 0
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        br_if 0 (;@2;)
      end
    end
    i32.const 0)
  (memory (;0;) 1)
  (global $__stack_pointer (mut i32) (i32.const 16))
  (global (;1;) i32 (i32.const 16))
  (global (;2;) i32 (i32.const 16))
  (export "memory" (memory 0))
  (export "main" (func $main))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
