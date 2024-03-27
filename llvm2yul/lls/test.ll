; ModuleID = 'llir.df3857a699fc1ae9-cgu.0'
source_filename = "llir.df3857a699fc1ae9-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nounwind nonlazybind
define void @_entry(i64 noundef %l) unnamed_addr #0 {
start:
  %_122 = icmp sgt i64 %l, 1
  br i1 %_122, label %bb5, label %bb7

bb7:                                              ; preds = %bb5, %start
  ret void

bb5:                                              ; preds = %start, %bb5
  %iter.sroa.0.03 = phi i64 [ %_0.i, %bb5 ], [ 1, %start ]
  %_0.i = add nuw nsw i64 %iter.sroa.0.03, 1
  %_2.i = tail call noundef i64 @build_int() #1
  tail call void @test(i64 noundef %iter.sroa.0.03) #1
  tail call void @test(i64 noundef %_2.i) #1
  %exitcond.not = icmp eq i64 %_0.i, %l
  br i1 %exitcond.not, label %bb7, label %bb5
}

; Function Attrs: nounwind nonlazybind
declare noundef i64 @build_int() unnamed_addr #0

; Function Attrs: nounwind nonlazybind
declare void @test(i64 noundef) unnamed_addr #0

attributes #0 = { nounwind nonlazybind "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.78.0-nightly (3cbb93223 2024-03-13)"}
