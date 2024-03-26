; ModuleID = 'store.4c37db01f9004d91-cgu.0'
source_filename = "store.4c37db01f9004d91-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nonlazybind uwtable
define void @_entry() unnamed_addr #0 {
start:
  %sender = tail call noundef i32 @__yul_caller()
  %idx = tail call noundef i32 @__yul__ext_literal(i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0)
  tail call void @__yul_sstore(i32 noundef %idx, i32 noundef %sender)
  %codeoffset = tail call noundef i32 @__yul_dataoffset(ptr noundef nonnull @_deployed_entry)
  %codesize = tail call noundef i32 @__yul_datasize(ptr noundef nonnull @_deployed_entry)
  %idx0 = tail call noundef i32 @__yul__ext_literal(i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0)
  tail call void @__yul_datacopy(i32 noundef %idx0, i32 noundef %codeoffset, i32 noundef %codesize)
  tail call void @__yul_return(i32 noundef %idx0, i32 noundef %codesize)
  ret void
}

; Function Attrs: nonlazybind uwtable
define void @_deployed_entry() unnamed_addr #0 {
start:
  %v0 = tail call noundef i32 @__yul__ext_literal(i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0, i32 noundef 0)
  tail call void @__yul_revert(i32 noundef %v0, i32 noundef %v0)
  ret void
}

; Function Attrs: nonlazybind uwtable
declare noundef i32 @__yul_caller() unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare noundef i32 @__yul__ext_literal(i32 noundef, i32 noundef, i32 noundef, i32 noundef, i32 noundef, i32 noundef, i32 noundef, i32 noundef) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare void @__yul_sstore(i32 noundef, i32 noundef) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare noundef i32 @__yul_dataoffset(ptr noundef nonnull) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare noundef i32 @__yul_datasize(ptr noundef nonnull) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare void @__yul_datacopy(i32 noundef, i32 noundef, i32 noundef) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare void @__yul_return(i32 noundef, i32 noundef) unnamed_addr #0

; Function Attrs: nonlazybind uwtable
declare void @__yul_revert(i32 noundef, i32 noundef) unnamed_addr #0

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.78.0-nightly (3cbb93223 2024-03-13)"}
