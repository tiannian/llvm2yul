; ModuleID = 'store.dc'
source_filename = "llvm-link"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: noinline nounwind nonlazybind
define noundef ptr @__yul_allocate(i64 noundef %len) unnamed_addr #0 {
start:
  %ptr1 = tail call noundef i64 @__yul_mload(ptr noundef nonnull inttoptr (i64 64 to ptr)) #4
  %_6 = tail call noundef zeroext i1 @__yul_iszero(i64 noundef %ptr1) #4
  %.ptr1 = select i1 %_6, i64 96, i64 %ptr1
  %_8 = tail call noundef i64 @__yul_add(i64 noundef %.ptr1, i64 noundef %len) #4
  tail call void @__yul_mstore(ptr noundef nonnull inttoptr (i64 64 to ptr), i64 noundef %_8) #4
  %_0 = inttoptr i64 %.ptr1 to ptr
  ret ptr %_0
}

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_mload(ptr noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare noundef zeroext i1 @__yul_iszero(i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_add(i64 noundef, i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare void @__yul_mstore(ptr noundef, i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
define noundef i64 @_ZN10patine_std8selector8selector17h3fd282a322a88d55E() unnamed_addr #1 {
start:
  %p = tail call noundef i64 @__yul__ext_literal(i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0) #4
  %selector_word = tail call noundef i64 @__yul_calldataload(i64 noundef %p) #4
  %rhs = tail call noundef i64 @__yul__ext_literal(i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 224) #4
  %res = tail call noundef i64 @__yul_shr(i64 noundef %selector_word, i64 noundef %rhs) #4
  ret i64 %res
}

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul__ext_literal(i64 noundef, i64 noundef, i64 noundef, i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_calldataload(i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_shr(i64 noundef, i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
define void @_store() unnamed_addr #1 {
start:
  %position = tail call noundef i64 @__yul__ext_literal(i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0) #4
  %value = tail call noundef i64 @__yul_caller() #4
  tail call void @__yul_sstore(i64 noundef %position, i64 noundef %value) #4
  %_3 = tail call noundef i64 @__yul_datasize(ptr noundef nonnull @_store_deployed) #4
  %ptr.i = tail call noundef ptr @__yul_allocate(i64 noundef %_3) #4
  %offset = tail call noundef i64 @__yul_dataoffset(ptr noundef nonnull @_store_deployed) #4
  tail call void @__yul_datacopy(ptr noundef %ptr.i, i64 noundef %offset, i64 noundef %_3) #4
  tail call void @__yul_return(ptr noundef %ptr.i, i64 noundef %_3) #4
  ret void
}

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_caller() unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare void @__yul_sstore(i64 noundef, i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
define void @_store_deployed() unnamed_addr #1 {
start:
  %ret_arr.i = alloca [32 x i8], align 1
  %x = tail call noundef i64 @__yul_callvalue() #4
  %cond = tail call noundef zeroext i1 @__yul_iszero(i64 noundef %x) #4
  br i1 %cond, label %bb9, label %bb8

bb8:                                              ; preds = %bb9, %start
  %0 = tail call i64 @_ZN10patine_std8selector8selector17h3fd282a322a88d55E() #4
  switch i64 %0, label %bb2 [
    i64 778358465, label %bb3
    i64 1616328221, label %bb4
  ]

bb9:                                              ; preds = %start
  tail call void @__yul_revert(ptr noundef null, i64 noundef 0) #4
  br label %bb8

bb2:                                              ; preds = %bb8
  tail call void @__yul_revert(ptr noundef null, i64 noundef 0) #4
  br label %bb5

bb3:                                              ; preds = %bb8
  %position.i = tail call noundef i64 @__yul__ext_literal(i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0) #4
  %v.i = tail call noundef i64 @__yul_sload(i64 noundef %position.i) #4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %ret_arr.i)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 1 dereferenceable(32) %ret_arr.i, i8 0, i64 32, i1 false)
  call void @__yul_mstore(ptr noundef nonnull %ret_arr.i, i64 noundef %v.i) #4
  call void @__yul_return(ptr noundef nonnull %ret_arr.i, i64 noundef 32) #4
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %ret_arr.i)
  br label %bb5

bb4:                                              ; preds = %bb8
  %p.i = tail call noundef i64 @__yul__ext_literal(i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 4) #4
  %v.i1 = tail call noundef i64 @__yul_calldataload(i64 noundef %p.i) #4
  %position.i2 = tail call noundef i64 @__yul__ext_literal(i64 noundef 0, i64 noundef 0, i64 noundef 0, i64 noundef 0) #4
  tail call void @__yul_sstore(i64 noundef %position.i2, i64 noundef %v.i1) #4
  br label %bb5

bb5:                                              ; preds = %bb4, %bb3, %bb2
  ret void
}

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_datasize(ptr noundef nonnull) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_dataoffset(ptr noundef nonnull) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare void @__yul_datacopy(ptr noundef, i64 noundef, i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare void @__yul_return(ptr noundef, i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_callvalue() unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare void @__yul_revert(ptr noundef, i64 noundef) unnamed_addr #1

; Function Attrs: nounwind nonlazybind
declare noundef i64 @__yul_sload(i64 noundef) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr nocapture) #2

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr nocapture writeonly, i8, i64, i1 immarg) #3

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr nocapture) #2

attributes #0 = { noinline nounwind nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nounwind nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #4 = { nounwind }

!llvm.ident = !{!0, !0, !0}
!llvm.module.flags = !{!1, !2}

!0 = !{!"rustc version 1.79.0-nightly (c9f8f3438 2024-03-27)"}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 2, !"RtLibUseGOT", i32 1}
