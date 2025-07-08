(module $wasm_example.wasm
  (type (;0;) (func))
  (type (;1;) (func (param i64)))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32)))
  (type (;5;) (func (param i32 i32 i32) (result i32)))
  (type (;6;) (func (param i32) (result i32)))
  (type (;7;) (func (param i32 i32 i32)))
  (import "host" "host_hello" (func $_ZN12wasm_example10host_hello17hf80dfca2827f9a4aE (type 0)))
  (import "host" "log" (func $_ZN12wasm_example3log17h1fa654dedad2c66bE (type 1)))
  (func $_ZN4talc4talc13Talc$LT$O$GT$4free17h073dd3b8b315c148E (type 2) (param i32 i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 1
        i32.add
        call $_ZN4talc9ptr_utils8align_up17h9d1c55fc5f6d0117E
        local.tee 1
        i32.load
        local.tee 0
        local.get 1
        i32.gt_u
        br_if 0 (;@2;)
        local.get 0
        local.set 2
        local.get 1
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.load
      local.set 2
    end
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            call $_ZN4talc4talc3tag3Tag10chunk_base17hedc89b2f9f83b1d1E
            local.tee 1
            i32.const -4
            i32.add
            local.tee 3
            i32.load
            local.tee 4
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 1
            local.get 4
            i32.sub
            local.set 1
            local.get 4
            i32.const 64
            i32.lt_u
            br_if 1 (;@3;)
            block  ;; label = @5
              local.get 4
              i32.const 128
              i32.ge_u
              br_if 0 (;@5;)
              local.get 4
              i32.const 3
              i32.shr_u
              i32.const 5
              i32.add
              local.set 4
              br 3 (;@2;)
            end
            local.get 4
            i32.const 30
            local.get 4
            i32.clz
            local.tee 3
            i32.sub
            i32.shr_u
            local.get 3
            i32.const 1
            i32.shl
            i32.sub
            i32.const 67
            i32.add
            local.tee 4
            i32.const 63
            local.get 4
            i32.const 63
            i32.lt_u
            select
            local.set 4
            br 2 (;@2;)
          end
          local.get 3
          call $_ZN4talc4talc3tag3Tag14set_above_free17h735dea07043c1aedE
          br 2 (;@1;)
        end
        local.get 4
        i32.const -12
        i32.add
        i32.const 2
        i32.shr_u
        local.set 4
      end
      local.get 1
      i32.const 4
      i32.add
      i32.load
      local.tee 5
      local.get 1
      i32.load
      local.tee 3
      i32.store
      block  ;; label = @2
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 5
        i32.store offset=4
      end
      i32.const 0
      i32.load offset=1188
      local.get 4
      i32.const 2
      i32.shl
      i32.add
      i32.load
      br_if 0 (;@1;)
      i32.const 8
      i32.const 12
      local.get 4
      i32.const 32
      i32.lt_u
      select
      i32.const 1172
      i32.add
      local.tee 3
      local.get 3
      i32.load
      i32.const 1
      local.get 4
      i32.shl
      i32.xor
      i32.store
    end
    local.get 0
    i32.const 4
    i32.add
    local.set 4
    block  ;; label = @1
      local.get 2
      i32.const 2
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=12
          local.tee 2
          i32.const 64
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            i32.const 128
            i32.ge_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 3
            i32.shr_u
            i32.const 5
            i32.add
            local.set 3
            br 2 (;@2;)
          end
          local.get 2
          i32.const 30
          local.get 2
          i32.clz
          local.tee 3
          i32.sub
          i32.shr_u
          local.get 3
          i32.const 1
          i32.shl
          i32.sub
          i32.const 67
          i32.add
          local.tee 3
          i32.const 63
          local.get 3
          i32.const 63
          i32.lt_u
          select
          local.set 3
          br 1 (;@2;)
        end
        local.get 2
        i32.const -12
        i32.add
        i32.const 2
        i32.shr_u
        local.set 3
      end
      local.get 0
      i32.const 8
      i32.add
      i32.load
      local.tee 5
      local.get 0
      i32.load offset=4
      local.tee 0
      i32.store
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.get 5
        i32.store offset=4
      end
      block  ;; label = @2
        i32.const 0
        i32.load offset=1188
        local.get 3
        i32.const 2
        i32.shl
        i32.add
        i32.load
        br_if 0 (;@2;)
        i32.const 8
        i32.const 12
        local.get 3
        i32.const 32
        i32.lt_u
        select
        i32.const 1172
        i32.add
        local.tee 0
        local.get 0
        i32.load
        i32.const 1
        local.get 3
        i32.shl
        i32.xor
        i32.store
      end
      local.get 4
      local.get 2
      i32.add
      local.set 4
    end
    block  ;; label = @1
      block  ;; label = @2
        local.get 4
        local.get 1
        i32.sub
        local.tee 0
        i32.const 64
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          i32.const 128
          i32.ge_u
          br_if 0 (;@3;)
          local.get 0
          i32.const 3
          i32.shr_u
          i32.const 5
          i32.add
          local.set 3
          br 2 (;@1;)
        end
        local.get 0
        i32.const 30
        local.get 0
        i32.clz
        local.tee 2
        i32.sub
        i32.shr_u
        local.get 2
        i32.const 1
        i32.shl
        i32.sub
        i32.const 67
        i32.add
        local.tee 2
        i32.const 63
        local.get 2
        i32.const 63
        i32.lt_u
        select
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const -12
      i32.add
      i32.const 2
      i32.shr_u
      local.set 3
    end
    block  ;; label = @1
      block  ;; label = @2
        i32.const 0
        i32.load offset=1188
        local.get 3
        i32.const 2
        i32.shl
        i32.add
        local.tee 2
        i32.load
        local.tee 5
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.store offset=4
        local.get 1
        i32.const 0
        i32.store
        local.get 2
        local.get 1
        i32.store
        i32.const 8
        i32.const 12
        local.get 3
        i32.const 32
        i32.lt_u
        select
        i32.const 1172
        i32.add
        local.tee 2
        local.get 2
        i32.load
        i32.const 1
        local.get 3
        i32.shl
        i32.xor
        i32.store
        br 1 (;@1;)
      end
      local.get 1
      local.get 2
      i32.store offset=4
      local.get 1
      local.get 5
      i32.store
      local.get 2
      local.get 1
      i32.store
      local.get 5
      local.get 1
      i32.store offset=4
    end
    local.get 1
    local.get 0
    i32.store offset=8
    local.get 4
    i32.const -4
    i32.add
    local.get 0
    i32.store)
  (func $_ZN4talc4talc13Talc$LT$O$GT$6malloc17h971426038f3a44c2E (type 3) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 12
    local.get 1
    i32.const 7
    i32.add
    i32.const -4
    i32.and
    local.get 1
    i32.const 9
    i32.lt_u
    select
    local.tee 2
    i32.const 3
    i32.shr_u
    i32.const 5
    i32.add
    local.get 2
    i32.const -12
    i32.add
    i32.const 2
    i32.shr_u
    local.get 2
    i32.const 30
    local.get 2
    i32.clz
    local.tee 3
    i32.sub
    i32.shr_u
    local.get 3
    i32.const 1
    i32.shl
    i32.sub
    i32.const 67
    i32.add
    local.tee 3
    i32.const 63
    local.get 3
    i32.const 63
    i32.lt_u
    select
    local.get 2
    i32.const 64
    i32.lt_u
    select
    local.tee 4
    local.get 2
    i32.const -64
    i32.and
    i32.const 64
    i32.eq
    local.tee 3
    select
    local.set 5
    local.get 4
    i32.const 31
    i32.and
    local.set 6
    local.get 3
    local.get 4
    i32.const 32
    i32.lt_u
    i32.or
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.const 5
          i32.lt_u
          br_if 0 (;@3;)
          i32.const 0
          local.get 0
          i32.sub
          local.set 8
          local.get 0
          i32.const -1
          i32.add
          local.set 9
          local.get 4
          i32.const 64
          i32.lt_u
          local.set 10
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 7
                        br_if 0 (;@10;)
                        local.get 10
                        i32.eqz
                        br_if 5 (;@5;)
                        i32.const 0
                        i32.load offset=1184
                        local.tee 11
                        local.get 6
                        i32.shr_u
                        local.tee 3
                        i32.eqz
                        br_if 5 (;@5;)
                        local.get 3
                        i32.ctz
                        local.get 4
                        i32.add
                        local.set 12
                        i32.const 0
                        i32.load offset=1180
                        local.set 13
                        br 1 (;@9;)
                      end
                      i32.const 0
                      i32.load offset=1180
                      local.tee 13
                      local.get 5
                      i32.shr_u
                      local.tee 3
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 3
                      i32.ctz
                      local.get 5
                      i32.add
                      local.set 12
                      i32.const 0
                      i32.load offset=1184
                      local.set 11
                    end
                    i32.const 0
                    i32.load offset=1188
                    local.set 14
                    block  ;; label = @9
                      local.get 11
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 11
                      i32.ctz
                      i32.const 32
                      i32.or
                      local.set 15
                      br 2 (;@7;)
                    end
                    loop  ;; label = @9
                      block  ;; label = @10
                        local.get 14
                        local.get 12
                        i32.const 2
                        i32.shl
                        i32.add
                        local.tee 16
                        i32.load
                        local.tee 3
                        i32.eqz
                        br_if 0 (;@10;)
                        loop  ;; label = @11
                          local.get 3
                          i32.load
                          local.set 17
                          block  ;; label = @12
                            local.get 3
                            i32.load offset=8
                            local.tee 18
                            local.get 2
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 9
                            local.get 3
                            i32.add
                            local.get 8
                            i32.and
                            local.tee 19
                            local.get 1
                            i32.add
                            i32.const 4
                            i32.add
                            local.get 3
                            local.get 18
                            i32.add
                            local.tee 18
                            i32.le_u
                            br_if 6 (;@6;)
                          end
                          local.get 17
                          local.set 3
                          local.get 17
                          br_if 0 (;@11;)
                        end
                      end
                      local.get 12
                      i32.const 30
                      i32.gt_u
                      br_if 4 (;@5;)
                      local.get 13
                      local.get 12
                      i32.const 1
                      i32.add
                      local.tee 3
                      i32.shr_u
                      local.tee 17
                      i32.eqz
                      br_if 4 (;@5;)
                      local.get 17
                      i32.ctz
                      local.get 3
                      i32.add
                      local.set 12
                      br 0 (;@9;)
                    end
                  end
                  i32.const 0
                  i32.load offset=1184
                  local.tee 11
                  i32.eqz
                  br_if 2 (;@5;)
                  i32.const 0
                  i32.load offset=1188
                  local.set 14
                  local.get 11
                  i32.ctz
                  i32.const 32
                  i32.or
                  local.tee 15
                  local.set 12
                end
                loop  ;; label = @7
                  block  ;; label = @8
                    local.get 14
                    local.get 12
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 16
                    i32.load
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    loop  ;; label = @9
                      local.get 3
                      i32.load
                      local.set 17
                      block  ;; label = @10
                        local.get 3
                        i32.load offset=8
                        local.tee 18
                        local.get 2
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 9
                        local.get 3
                        i32.add
                        local.get 8
                        i32.and
                        local.tee 19
                        local.get 1
                        i32.add
                        i32.const 4
                        i32.add
                        local.get 3
                        local.get 18
                        i32.add
                        local.tee 18
                        i32.le_u
                        br_if 4 (;@6;)
                      end
                      local.get 17
                      local.set 3
                      local.get 17
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 12
                  i32.const 1
                  i32.add
                  local.set 3
                  block  ;; label = @8
                    local.get 12
                    i32.const 31
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 12
                    i32.const 63
                    i32.ge_u
                    br_if 3 (;@5;)
                    local.get 11
                    local.get 3
                    i32.shr_u
                    local.tee 17
                    i32.eqz
                    br_if 3 (;@5;)
                    local.get 17
                    i32.ctz
                    local.get 3
                    i32.add
                    local.set 12
                    br 1 (;@7;)
                  end
                  local.get 15
                  local.set 12
                  local.get 13
                  local.get 3
                  i32.shr_u
                  local.tee 17
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 17
                  i32.ctz
                  local.get 3
                  i32.add
                  local.set 12
                  br 0 (;@7;)
                end
              end
              local.get 3
              i32.const 4
              i32.add
              i32.load
              local.tee 2
              local.get 17
              i32.store
              block  ;; label = @6
                local.get 17
                i32.eqz
                br_if 0 (;@6;)
                local.get 17
                local.get 2
                i32.store offset=4
              end
              local.get 16
              i32.load
              br_if 3 (;@2;)
              i32.const 8
              i32.const 12
              local.get 12
              i32.const 32
              i32.lt_u
              select
              i32.const 1172
              i32.add
              local.tee 17
              local.get 17
              i32.load
              i32.const 1
              local.get 12
              i32.shl
              i32.xor
              i32.store
              br 3 (;@2;)
            end
            i32.const 0
            local.set 19
            i32.const 1172
            local.get 0
            local.get 1
            call $_ZN79_$LT$talc..oom_handler..ClaimOnOom$u20$as$u20$talc..oom_handler..OomHandler$GT$10handle_oom17hc9461f1e72b39853E
            i32.eqz
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 4
        i32.const 63
        i32.gt_u
        local.set 12
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 7
                      br_if 0 (;@9;)
                      local.get 12
                      br_if 4 (;@5;)
                      i32.const 0
                      i32.load offset=1184
                      local.tee 19
                      local.get 6
                      i32.shr_u
                      local.tee 3
                      i32.eqz
                      br_if 4 (;@5;)
                      local.get 3
                      i32.ctz
                      local.get 4
                      i32.add
                      local.set 18
                      i32.const 0
                      i32.load offset=1180
                      local.set 14
                      br 1 (;@8;)
                    end
                    block  ;; label = @9
                      i32.const 0
                      i32.load offset=1180
                      local.tee 14
                      local.get 5
                      i32.shr_u
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 3
                      i32.ctz
                      local.get 5
                      i32.add
                      local.set 18
                      i32.const 0
                      i32.load offset=1184
                      local.set 19
                      br 1 (;@8;)
                    end
                    i32.const 0
                    i32.load offset=1184
                    local.tee 19
                    i32.eqz
                    br_if 3 (;@5;)
                    i32.const 0
                    i32.load offset=1188
                    local.set 8
                    local.get 19
                    i32.ctz
                    i32.const 32
                    i32.or
                    local.tee 16
                    local.set 18
                    br 1 (;@7;)
                  end
                  i32.const 0
                  i32.load offset=1188
                  local.set 8
                  local.get 19
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 19
                  i32.ctz
                  i32.const 32
                  i32.or
                  local.set 16
                end
                loop  ;; label = @7
                  local.get 8
                  local.get 18
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 9
                  local.set 3
                  block  ;; label = @8
                    loop  ;; label = @9
                      local.get 3
                      i32.load
                      local.tee 3
                      i32.eqz
                      br_if 1 (;@8;)
                      local.get 3
                      i32.load offset=8
                      local.tee 17
                      local.get 2
                      i32.lt_u
                      br_if 0 (;@9;)
                      br 6 (;@3;)
                    end
                  end
                  local.get 18
                  i32.const 1
                  i32.add
                  local.set 3
                  block  ;; label = @8
                    local.get 18
                    i32.const 31
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 18
                    i32.const 62
                    i32.gt_u
                    br_if 3 (;@5;)
                    local.get 19
                    local.get 3
                    i32.shr_u
                    local.tee 17
                    i32.eqz
                    br_if 3 (;@5;)
                    local.get 17
                    i32.ctz
                    local.get 3
                    i32.add
                    local.set 18
                    br 1 (;@7;)
                  end
                  local.get 16
                  local.set 18
                  local.get 14
                  local.get 3
                  i32.shr_u
                  local.tee 17
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 17
                  i32.ctz
                  local.get 3
                  i32.add
                  local.set 18
                  br 0 (;@7;)
                end
              end
              loop  ;; label = @6
                local.get 8
                local.get 18
                i32.const 2
                i32.shl
                i32.add
                local.tee 9
                local.set 3
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 3
                    i32.load
                    local.tee 3
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 3
                    i32.load offset=8
                    local.tee 17
                    local.get 2
                    i32.ge_u
                    br_if 5 (;@3;)
                    br 0 (;@8;)
                  end
                end
                local.get 18
                i32.const 30
                i32.gt_u
                br_if 1 (;@5;)
                local.get 14
                local.get 18
                i32.const 1
                i32.add
                local.tee 3
                i32.shr_u
                local.tee 17
                i32.eqz
                br_if 1 (;@5;)
                local.get 17
                i32.ctz
                local.get 3
                i32.add
                local.set 18
                br 0 (;@6;)
              end
            end
            i32.const 0
            local.set 19
            i32.const 1172
            local.get 0
            local.get 1
            call $_ZN79_$LT$talc..oom_handler..ClaimOnOom$u20$as$u20$talc..oom_handler..OomHandler$GT$10handle_oom17hc9461f1e72b39853E
            i32.eqz
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 3
        i32.const 4
        i32.add
        i32.load
        local.tee 19
        local.get 3
        i32.load
        local.tee 2
        i32.store
        block  ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          local.get 19
          i32.store offset=4
        end
        block  ;; label = @3
          local.get 9
          i32.load
          br_if 0 (;@3;)
          i32.const 8
          i32.const 12
          local.get 18
          i32.const 32
          i32.lt_u
          select
          i32.const 1172
          i32.add
          local.tee 2
          local.get 2
          i32.load
          i32.const 1
          local.get 18
          i32.shl
          i32.xor
          i32.store
        end
        local.get 3
        local.get 17
        i32.add
        local.set 18
        local.get 3
        local.set 19
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 18
          i32.const -12
          i32.add
          local.tee 17
          local.get 19
          local.get 17
          local.get 19
          i32.lt_u
          select
          local.tee 17
          local.get 3
          i32.sub
          local.tee 2
          i32.const 11
          i32.gt_u
          br_if 0 (;@3;)
          local.get 3
          i32.const -4
          i32.add
          call $_ZN4talc4talc3tag3Tag16clear_above_free17hf1ae8af19ca1c7b2E
          local.get 3
          local.set 17
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 64
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 2
              i32.const 128
              i32.ge_u
              br_if 0 (;@5;)
              local.get 2
              i32.const 3
              i32.shr_u
              i32.const 5
              i32.add
              local.set 9
              br 2 (;@3;)
            end
            local.get 2
            i32.const 30
            local.get 2
            i32.clz
            local.tee 8
            i32.sub
            i32.shr_u
            local.get 8
            i32.const 1
            i32.shl
            i32.sub
            i32.const 67
            i32.add
            local.tee 8
            i32.const 63
            local.get 8
            i32.const 63
            i32.lt_u
            select
            local.set 9
            br 1 (;@3;)
          end
          local.get 2
          i32.const -12
          i32.add
          i32.const 2
          i32.shr_u
          local.set 9
        end
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1188
            local.get 9
            i32.const 2
            i32.shl
            i32.add
            local.tee 8
            i32.load
            local.tee 12
            br_if 0 (;@4;)
            local.get 3
            local.get 8
            i32.store offset=4
            local.get 3
            i32.const 0
            i32.store
            local.get 8
            local.get 3
            i32.store
            i32.const 8
            i32.const 12
            local.get 9
            i32.const 32
            i32.lt_u
            select
            i32.const 1172
            i32.add
            local.tee 8
            local.get 8
            i32.load
            i32.const 1
            local.get 9
            i32.shl
            i32.xor
            i32.store
            br 1 (;@3;)
          end
          local.get 3
          local.get 8
          i32.store offset=4
          local.get 3
          local.get 12
          i32.store
          local.get 8
          local.get 3
          i32.store
          local.get 12
          local.get 3
          i32.store offset=4
        end
        local.get 3
        local.get 2
        i32.store offset=8
        local.get 17
        i32.const -4
        i32.add
        local.get 2
        i32.store
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 18
          local.get 17
          i32.const 8
          i32.add
          local.tee 3
          local.get 19
          local.get 1
          i32.add
          call $_ZN4talc9ptr_utils8align_up17h9d1c55fc5f6d0117E
          local.tee 2
          local.get 2
          local.get 3
          i32.lt_u
          select
          local.tee 3
          i32.const 4
          i32.add
          local.tee 8
          i32.sub
          local.tee 1
          i32.const 11
          i32.gt_u
          br_if 0 (;@3;)
          local.get 18
          i32.const -4
          i32.add
          local.tee 3
          local.get 17
          i32.const 0
          call $_ZN4talc4talc3tag3Tag5write17h3d6192dca1f450d3E
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 64
            i32.lt_u
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 1
              i32.const 128
              i32.ge_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 3
              i32.shr_u
              i32.const 5
              i32.add
              local.set 12
              br 2 (;@3;)
            end
            local.get 1
            i32.const 30
            local.get 1
            i32.clz
            local.tee 9
            i32.sub
            i32.shr_u
            local.get 9
            i32.const 1
            i32.shl
            i32.sub
            i32.const 67
            i32.add
            local.tee 9
            i32.const 63
            local.get 9
            i32.const 63
            i32.lt_u
            select
            local.set 12
            br 1 (;@3;)
          end
          local.get 1
          i32.const -12
          i32.add
          i32.const 2
          i32.shr_u
          local.set 12
        end
        block  ;; label = @3
          block  ;; label = @4
            i32.const 0
            i32.load offset=1188
            local.get 12
            i32.const 2
            i32.shl
            i32.add
            local.tee 9
            i32.load
            local.tee 4
            br_if 0 (;@4;)
            local.get 3
            local.get 9
            i32.store offset=8
            local.get 3
            i32.const 0
            i32.store offset=4
            local.get 9
            local.get 8
            i32.store
            i32.const 8
            i32.const 12
            local.get 12
            i32.const 32
            i32.lt_u
            select
            i32.const 1172
            i32.add
            local.tee 8
            local.get 8
            i32.load
            i32.const 1
            local.get 12
            i32.shl
            i32.xor
            i32.store
            br 1 (;@3;)
          end
          local.get 3
          local.get 9
          i32.store offset=8
          local.get 3
          local.get 4
          i32.store offset=4
          local.get 9
          local.get 8
          i32.store
          local.get 4
          local.get 8
          i32.store offset=4
        end
        local.get 3
        local.get 1
        i32.store offset=12
        local.get 18
        i32.const -4
        i32.add
        local.get 1
        i32.store
        local.get 3
        local.get 17
        i32.const 1
        call $_ZN4talc4talc3tag3Tag5write17h3d6192dca1f450d3E
      end
      local.get 3
      local.get 2
      i32.eq
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      i32.store
      local.get 19
      return
    end
    local.get 19)
  (func $hello_from_host (type 0)
    call $_ZN12wasm_example10host_hello17hf80dfca2827f9a4aE)
  (func $static_alloc_and_log (type 0)
    i32.const 1160
    i64.extend_i32_u
    i64.const 32
    i64.shl
    i64.const 12
    i64.or
    call $_ZN12wasm_example3log17h1fa654dedad2c66bE)
  (func $dyn_alloc_and_log (type 4) (param i32)
    (local i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          br 1 (;@2;)
        end
        i32.const 0
        local.set 2
        local.get 0
        i32.const 0
        i32.lt_s
        br_if 1 (;@1;)
        call $_RNvCse3amHqTQ2bx_7___rustc35___rust_no_alloc_shim_is_unstable_v2
        i32.const 1
        local.set 2
        i32.const 1
        local.get 0
        call $_ZN4talc4talc13Talc$LT$O$GT$6malloc17h971426038f3a44c2E
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 1
        i32.const 38
        i32.store8
        local.get 0
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        i32.const 1
        local.set 2
        loop  ;; label = @3
          local.get 1
          local.get 2
          i32.add
          local.get 1
          local.get 2
          call $memcpy
          drop
          local.get 2
          i32.const 1
          i32.shl
          local.set 2
          local.get 3
          i32.const 4
          i32.lt_u
          local.set 4
          local.get 3
          i32.const 1
          i32.shr_u
          local.set 3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
        end
        local.get 0
        local.get 2
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.add
        local.get 1
        local.get 0
        local.get 2
        i32.sub
        call $memcpy
        drop
      end
      local.get 1
      i64.extend_i32_u
      i64.const 32
      i64.shl
      local.get 0
      i64.extend_i32_u
      i64.or
      call $_ZN12wasm_example3log17h1fa654dedad2c66bE
      block  ;; label = @2
        local.get 0
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        local.get 0
        call $_ZN4talc4talc13Talc$LT$O$GT$4free17h073dd3b8b315c148E
      end
      return
    end
    local.get 2
    local.get 0
    i32.const 1144
    call $_ZN5alloc7raw_vec12handle_error17hdbfd218b4a9ae84cE
    unreachable)
  (func $_RNvCse3amHqTQ2bx_7___rustc35___rust_no_alloc_shim_is_unstable_v2 (type 0)
    return)
  (func $_ZN79_$LT$talc..oom_handler..ClaimOnOom$u20$as$u20$talc..oom_handler..OomHandler$GT$10handle_oom17hc9461f1e72b39853E (type 5) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 4
      local.get 0
      i32.load
      local.tee 5
      i32.le_u
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 5
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const -5
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 4
                  i32.const -4
                  i32.and
                  local.tee 4
                  local.get 5
                  i32.const 3
                  i32.add
                  i32.const -4
                  i32.and
                  local.tee 5
                  i32.le_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 5
                  i32.sub
                  local.set 6
                  block  ;; label = @8
                    local.get 0
                    i32.load offset=16
                    local.tee 7
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 263
                    i32.le_u
                    br_if 7 (;@1;)
                    local.get 5
                    i32.const 1
                    i32.store
                    local.get 0
                    local.get 5
                    i32.const 4
                    i32.add
                    i32.const 0
                    i32.const 256
                    call $memset
                    local.tee 7
                    i32.store offset=16
                    block  ;; label = @9
                      local.get 4
                      local.get 5
                      i32.const 264
                      i32.add
                      local.tee 6
                      i32.sub
                      local.tee 3
                      i32.const 11
                      i32.gt_u
                      br_if 0 (;@9;)
                      block  ;; label = @10
                        local.get 4
                        i32.const -4
                        i32.add
                        local.tee 3
                        local.get 5
                        i32.const 260
                        i32.add
                        local.tee 4
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 4
                        local.get 3
                        i32.store
                      end
                      local.get 3
                      local.get 5
                      i32.const 1
                      i32.add
                      i32.store
                      br 7 (;@2;)
                    end
                    local.get 3
                    i32.const 64
                    i32.lt_u
                    br_if 2 (;@6;)
                    block  ;; label = @9
                      local.get 3
                      i32.const 128
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 3
                      i32.const 3
                      i32.shr_u
                      i32.const 5
                      i32.add
                      local.set 8
                      br 6 (;@3;)
                    end
                    local.get 3
                    i32.const 30
                    local.get 3
                    i32.clz
                    local.tee 8
                    i32.sub
                    i32.shr_u
                    local.get 8
                    i32.const 1
                    i32.shl
                    i32.sub
                    i32.const 67
                    i32.add
                    local.tee 8
                    i32.const 63
                    local.get 8
                    i32.const 63
                    i32.lt_u
                    select
                    local.set 8
                    br 5 (;@3;)
                  end
                  local.get 6
                  i32.const 15
                  i32.le_u
                  br_if 6 (;@1;)
                  local.get 5
                  i32.const 3
                  i32.store
                  local.get 4
                  local.get 5
                  i32.const 4
                  i32.add
                  local.tee 9
                  i32.sub
                  local.tee 3
                  i32.const 64
                  i32.lt_u
                  br_if 2 (;@5;)
                  block  ;; label = @8
                    local.get 3
                    i32.const 128
                    i32.ge_u
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const 3
                    i32.shr_u
                    i32.const 5
                    i32.add
                    local.set 8
                    br 4 (;@4;)
                  end
                  local.get 3
                  i32.const 30
                  local.get 3
                  i32.clz
                  local.tee 6
                  i32.sub
                  i32.shr_u
                  local.get 6
                  i32.const 1
                  i32.shl
                  i32.sub
                  i32.const 67
                  i32.add
                  local.tee 6
                  i32.const 63
                  local.get 6
                  i32.const 63
                  i32.lt_u
                  select
                  local.set 8
                  br 3 (;@4;)
                end
                unreachable
              end
              local.get 3
              i32.const -12
              i32.add
              i32.const 2
              i32.shr_u
              local.set 8
              br 2 (;@3;)
            end
            local.get 3
            i32.const -12
            i32.add
            i32.const 2
            i32.shr_u
            local.set 8
          end
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              local.get 8
              i32.const 2
              i32.shl
              i32.add
              local.tee 6
              i32.load
              local.tee 7
              br_if 0 (;@5;)
              local.get 5
              i32.const 8
              i32.add
              local.get 6
              i32.store
              local.get 5
              i32.const 4
              i32.add
              i32.const 0
              i32.store
              local.get 6
              local.get 9
              i32.store
              local.get 0
              i32.const 8
              i32.const 12
              local.get 8
              i32.const 32
              i32.lt_u
              select
              i32.add
              local.tee 6
              local.get 6
              i32.load
              i32.const 1
              local.get 8
              i32.shl
              i32.xor
              i32.store
              br 1 (;@4;)
            end
            local.get 5
            i32.const 8
            i32.add
            local.get 6
            i32.store
            local.get 5
            i32.const 4
            i32.add
            local.get 7
            i32.store
            local.get 6
            local.get 9
            i32.store
            local.get 7
            local.get 9
            i32.store offset=4
          end
          local.get 5
          i32.const 12
          i32.add
          local.get 3
          i32.store
          local.get 4
          i32.const -4
          i32.add
          local.get 3
          i32.store
          br 1 (;@2;)
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 7
            local.get 8
            i32.const 2
            i32.shl
            i32.add
            local.tee 7
            i32.load
            local.tee 9
            br_if 0 (;@4;)
            local.get 5
            local.get 7
            i32.store offset=268
            local.get 5
            i32.const 0
            i32.store offset=264
            local.get 7
            local.get 6
            i32.store
            local.get 0
            i32.const 8
            i32.const 12
            local.get 8
            i32.const 32
            i32.lt_u
            select
            i32.add
            local.tee 6
            local.get 6
            i32.load
            i32.const 1
            local.get 8
            i32.shl
            i32.xor
            i32.store
            br 1 (;@3;)
          end
          local.get 5
          local.get 7
          i32.store offset=268
          local.get 5
          local.get 9
          i32.store offset=264
          local.get 7
          local.get 6
          i32.store
          local.get 9
          local.get 6
          i32.store offset=4
        end
        local.get 5
        local.get 3
        i32.store offset=272
        local.get 4
        i32.const -4
        i32.add
        local.get 3
        i32.store
        local.get 5
        local.get 5
        i32.const 3
        i32.add
        i32.store offset=260
      end
      local.get 0
      i64.const 0
      i64.store align=4
      i32.const 0
      local.set 3
    end
    local.get 3)
  (func $_ZN4talc9ptr_utils8align_up17h9d1c55fc5f6d0117E (type 6) (param i32) (result i32)
    local.get 0
    i32.const 3
    i32.add
    i32.const -4
    i32.and)
  (func $_ZN4talc4talc3tag3Tag5write17h3d6192dca1f450d3E (type 7) (param i32 i32 i32)
    local.get 0
    local.get 1
    i32.const 3
    i32.const 1
    local.get 2
    select
    i32.add
    i32.store)
  (func $_ZN4talc4talc3tag3Tag10chunk_base17hedc89b2f9f83b1d1E (type 6) (param i32) (result i32)
    local.get 0
    i32.const -4
    i32.and)
  (func $_ZN4talc4talc3tag3Tag14set_above_free17h735dea07043c1aedE (type 4) (param i32)
    local.get 0
    local.get 0
    i32.load
    i32.const 2
    i32.add
    i32.store)
  (func $_ZN4talc4talc3tag3Tag16clear_above_free17hf1ae8af19ca1c7b2E (type 4) (param i32)
    local.get 0
    local.get 0
    i32.load
    i32.const -2
    i32.add
    i32.store)
  (func $_ZN5alloc7raw_vec12handle_error17hdbfd218b4a9ae84cE (type 7) (param i32 i32 i32)
    unreachable)
  (func $_ZN17compiler_builtins3mem6memcpy17h1df4ddd6a417309eE (type 5) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 4
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 0
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 5
        i32.add
        local.tee 6
        i32.ge_u
        br_if 0 (;@2;)
        local.get 5
        i32.const -1
        i32.add
        local.set 7
        local.get 0
        local.set 4
        local.get 1
        local.set 8
        block  ;; label = @3
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.set 9
          local.get 0
          local.set 4
          local.get 1
          local.set 8
          loop  ;; label = @4
            local.get 4
            local.get 8
            i32.load8_u
            i32.store8
            local.get 8
            i32.const 1
            i32.add
            local.set 8
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 9
            i32.const -1
            i32.add
            local.tee 9
            br_if 0 (;@4;)
          end
        end
        local.get 7
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 4
          local.get 8
          i32.load8_u
          i32.store8
          local.get 4
          i32.const 1
          i32.add
          local.get 8
          i32.const 1
          i32.add
          i32.load8_u
          i32.store8
          local.get 4
          i32.const 2
          i32.add
          local.get 8
          i32.const 2
          i32.add
          i32.load8_u
          i32.store8
          local.get 4
          i32.const 3
          i32.add
          local.get 8
          i32.const 3
          i32.add
          i32.load8_u
          i32.store8
          local.get 4
          i32.const 4
          i32.add
          local.get 8
          i32.const 4
          i32.add
          i32.load8_u
          i32.store8
          local.get 4
          i32.const 5
          i32.add
          local.get 8
          i32.const 5
          i32.add
          i32.load8_u
          i32.store8
          local.get 4
          i32.const 6
          i32.add
          local.get 8
          i32.const 6
          i32.add
          i32.load8_u
          i32.store8
          local.get 4
          i32.const 7
          i32.add
          local.get 8
          i32.const 7
          i32.add
          i32.load8_u
          i32.store8
          local.get 8
          i32.const 8
          i32.add
          local.set 8
          local.get 4
          i32.const 8
          i32.add
          local.tee 4
          local.get 6
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 6
      local.get 2
      local.get 5
      i32.sub
      local.tee 9
      i32.const -4
      i32.and
      local.tee 7
      i32.add
      local.set 4
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 5
          i32.add
          local.tee 8
          i32.const 3
          i32.and
          local.tee 1
          br_if 0 (;@3;)
          local.get 6
          local.get 4
          i32.ge_u
          br_if 1 (;@2;)
          local.get 8
          local.set 1
          loop  ;; label = @4
            local.get 6
            local.get 1
            i32.load
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 6
            i32.const 4
            i32.add
            local.tee 6
            local.get 4
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        i32.const 0
        local.set 2
        local.get 3
        i32.const 0
        i32.store offset=12
        local.get 3
        i32.const 12
        i32.add
        local.get 1
        i32.or
        local.set 5
        block  ;; label = @3
          i32.const 4
          local.get 1
          i32.sub
          local.tee 10
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 8
          i32.load8_u
          i32.store8
          i32.const 1
          local.set 2
        end
        block  ;; label = @3
          local.get 10
          i32.const 2
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          local.get 2
          i32.add
          local.get 8
          local.get 2
          i32.add
          i32.load16_u
          i32.store16
        end
        local.get 8
        local.get 1
        i32.sub
        local.set 2
        local.get 1
        i32.const 3
        i32.shl
        local.set 11
        local.get 3
        i32.load offset=12
        local.set 5
        block  ;; label = @3
          block  ;; label = @4
            local.get 6
            i32.const 4
            i32.add
            local.get 4
            i32.lt_u
            br_if 0 (;@4;)
            local.get 6
            local.set 12
            br 1 (;@3;)
          end
          i32.const 0
          local.get 11
          i32.sub
          i32.const 24
          i32.and
          local.set 13
          loop  ;; label = @4
            local.get 6
            local.get 5
            local.get 11
            i32.shr_u
            local.get 2
            i32.const 4
            i32.add
            local.tee 2
            i32.load
            local.tee 5
            local.get 13
            i32.shl
            i32.or
            i32.store
            local.get 6
            i32.const 8
            i32.add
            local.set 10
            local.get 6
            i32.const 4
            i32.add
            local.tee 12
            local.set 6
            local.get 10
            local.get 4
            i32.lt_u
            br_if 0 (;@4;)
          end
        end
        i32.const 0
        local.set 6
        local.get 3
        i32.const 0
        i32.store8 offset=8
        local.get 3
        i32.const 0
        i32.store8 offset=6
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 1
            i32.ne
            br_if 0 (;@4;)
            local.get 3
            i32.const 8
            i32.add
            local.set 13
            i32.const 0
            local.set 1
            i32.const 0
            local.set 10
            i32.const 0
            local.set 14
            br 1 (;@3;)
          end
          local.get 2
          i32.const 5
          i32.add
          i32.load8_u
          local.set 10
          local.get 3
          local.get 2
          i32.const 4
          i32.add
          i32.load8_u
          local.tee 1
          i32.store8 offset=8
          local.get 10
          i32.const 8
          i32.shl
          local.set 10
          i32.const 2
          local.set 14
          local.get 3
          i32.const 6
          i32.add
          local.set 13
        end
        block  ;; label = @3
          local.get 8
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 13
          local.get 2
          i32.const 4
          i32.add
          local.get 14
          i32.add
          i32.load8_u
          i32.store8
          local.get 3
          i32.load8_u offset=6
          i32.const 16
          i32.shl
          local.set 6
          local.get 3
          i32.load8_u offset=8
          local.set 1
        end
        local.get 12
        local.get 10
        local.get 6
        i32.or
        local.get 1
        i32.const 255
        i32.and
        i32.or
        i32.const 0
        local.get 11
        i32.sub
        i32.const 24
        i32.and
        i32.shl
        local.get 5
        local.get 11
        i32.shr_u
        i32.or
        i32.store
      end
      local.get 9
      i32.const 3
      i32.and
      local.set 2
      local.get 8
      local.get 7
      i32.add
      local.set 1
    end
    block  ;; label = @1
      local.get 4
      local.get 4
      local.get 2
      i32.add
      local.tee 6
      i32.ge_u
      br_if 0 (;@1;)
      local.get 2
      i32.const -1
      i32.add
      local.set 9
      block  ;; label = @2
        local.get 2
        i32.const 7
        i32.and
        local.tee 8
        i32.eqz
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 4
          local.get 1
          i32.load8_u
          i32.store8
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 4
          i32.const 1
          i32.add
          local.set 4
          local.get 8
          i32.const -1
          i32.add
          local.tee 8
          br_if 0 (;@3;)
        end
      end
      local.get 9
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 4
        local.get 1
        i32.load8_u
        i32.store8
        local.get 4
        i32.const 1
        i32.add
        local.get 1
        i32.const 1
        i32.add
        i32.load8_u
        i32.store8
        local.get 4
        i32.const 2
        i32.add
        local.get 1
        i32.const 2
        i32.add
        i32.load8_u
        i32.store8
        local.get 4
        i32.const 3
        i32.add
        local.get 1
        i32.const 3
        i32.add
        i32.load8_u
        i32.store8
        local.get 4
        i32.const 4
        i32.add
        local.get 1
        i32.const 4
        i32.add
        i32.load8_u
        i32.store8
        local.get 4
        i32.const 5
        i32.add
        local.get 1
        i32.const 5
        i32.add
        i32.load8_u
        i32.store8
        local.get 4
        i32.const 6
        i32.add
        local.get 1
        i32.const 6
        i32.add
        i32.load8_u
        i32.store8
        local.get 4
        i32.const 7
        i32.add
        local.get 1
        i32.const 7
        i32.add
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 8
        i32.add
        local.set 1
        local.get 4
        i32.const 8
        i32.add
        local.tee 4
        local.get 6
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcpy (type 5) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call $_ZN17compiler_builtins3mem6memcpy17h1df4ddd6a417309eE)
  (func $memset (type 5) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 0
        local.get 0
        i32.const 0
        local.get 0
        i32.sub
        i32.const 3
        i32.and
        local.tee 4
        i32.add
        local.tee 5
        i32.ge_u
        br_if 0 (;@2;)
        local.get 4
        i32.const -1
        i32.add
        local.set 6
        local.get 0
        local.set 3
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          local.set 7
          local.get 0
          local.set 3
          loop  ;; label = @4
            local.get 3
            local.get 1
            i32.store8
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            local.get 7
            i32.const -1
            i32.add
            local.tee 7
            br_if 0 (;@4;)
          end
        end
        local.get 6
        i32.const 7
        i32.lt_u
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.store8
          local.get 3
          i32.const 7
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 6
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 5
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 4
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 3
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 2
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.get 1
          i32.store8
          local.get 3
          i32.const 8
          i32.add
          local.tee 3
          local.get 5
          i32.ne
          br_if 0 (;@3;)
        end
      end
      block  ;; label = @2
        local.get 5
        local.get 5
        local.get 2
        local.get 4
        i32.sub
        local.tee 2
        i32.const -4
        i32.and
        i32.add
        local.tee 3
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.set 7
        loop  ;; label = @3
          local.get 5
          local.get 7
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.const 3
      i32.and
      local.set 2
    end
    block  ;; label = @1
      local.get 3
      local.get 3
      local.get 2
      i32.add
      local.tee 7
      i32.ge_u
      br_if 0 (;@1;)
      local.get 2
      i32.const -1
      i32.add
      local.set 4
      block  ;; label = @2
        local.get 2
        i32.const 7
        i32.and
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          local.get 5
          i32.const -1
          i32.add
          local.tee 5
          br_if 0 (;@3;)
        end
      end
      local.get 4
      i32.const 7
      i32.lt_u
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.store8
        local.get 3
        i32.const 7
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 6
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 5
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 4
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 3
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 2
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 8
        i32.add
        local.tee 3
        local.get 7
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 4096 (pagesize 1))
  (global $__stack_pointer (mut i32) (i32.const 1024))
  (global (;1;) i32 (i32.const 2216))
  (global (;2;) i32 (i32.const 2224))
  (export "memory" (memory 0))
  (export "hello_from_host" (func $hello_from_host))
  (export "static_alloc_and_log" (func $static_alloc_and_log))
  (export "dyn_alloc_and_log" (func $dyn_alloc_and_log))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data $.rodata (i32.const 1024) "/home/tribe11200675/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/slice.rs\00\00\04\00\00w\00\00\00\0c\02\00\00\17\00\00\00Hello World!")
  (data $.data (i32.const 1172) "\a8\04\00\00\a8\08\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))
