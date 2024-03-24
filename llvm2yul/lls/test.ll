; ModuleID = 'llir.304ea5af71a91ec-cgu.0'
source_filename = "llir.304ea5af71a91ec-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: nonlazybind uwtable
define void @main(i64 noundef %l) unnamed_addr #0 {
start:
  %_102 = icmp sgt i64 %l, 1
  br i1 %_102, label %bb4, label %bb6

bb6:                                              ; preds = %"_ZN4core3num21_$LT$impl$u20$i64$GT$3pow17h9bc75a53b14f3f0eE.exit", %start
  ret void

bb4:                                              ; preds = %start, %"_ZN4core3num21_$LT$impl$u20$i64$GT$3pow17h9bc75a53b14f3f0eE.exit"
  %iter.sroa.0.03 = phi i64 [ %_0.i, %"_ZN4core3num21_$LT$impl$u20$i64$GT$3pow17h9bc75a53b14f3f0eE.exit" ], [ 1, %start ]
  %_0.i = add nuw nsw i64 %iter.sroa.0.03, 1
  %_9 = trunc i64 %iter.sroa.0.03 to i32
  switch i32 %_9, label %bb4.i [
    i32 0, label %"_ZN4core3num21_$LT$impl$u20$i64$GT$3pow17h9bc75a53b14f3f0eE.exit"
    i32 1, label %bb10.i
  ]

bb10.i:                                           ; preds = %bb4.i, %bb4
  %acc.0.lcssa.i = phi i64 [ 1, %bb4 ], [ %spec.select.i, %bb4.i ]
  %base.0.lcssa.i = phi i64 [ %iter.sroa.0.03, %bb4 ], [ %_15.0.i, %bb4.i ]
  %_18.0.i = mul i64 %base.0.lcssa.i, %acc.0.lcssa.i
  br label %"_ZN4core3num21_$LT$impl$u20$i64$GT$3pow17h9bc75a53b14f3f0eE.exit"

bb4.i:                                            ; preds = %bb4, %bb4.i
  %exp.011.i = phi i32 [ %0, %bb4.i ], [ %_9, %bb4 ]
  %base.010.i = phi i64 [ %_15.0.i, %bb4.i ], [ %iter.sroa.0.03, %bb4 ]
  %acc.09.i = phi i64 [ %spec.select.i, %bb4.i ], [ 1, %bb4 ]
  %_8.i = and i32 %exp.011.i, 1
  %.not.i = icmp eq i32 %_8.i, 0
  %_12.0.i = select i1 %.not.i, i64 1, i64 %base.010.i
  %spec.select.i = mul i64 %_12.0.i, %acc.09.i
  %0 = lshr i32 %exp.011.i, 1
  %_15.0.i = mul i64 %base.010.i, %base.010.i
  %_6.i = icmp ugt i32 %exp.011.i, 3
  br i1 %_6.i, label %bb4.i, label %bb10.i

"_ZN4core3num21_$LT$impl$u20$i64$GT$3pow17h9bc75a53b14f3f0eE.exit": ; preds = %bb4, %bb10.i
  %_0.0.i = phi i64 [ %_18.0.i, %bb10.i ], [ 1, %bb4 ]
  tail call void @test(i64 noundef %_0.0.i)
  %exitcond.not = icmp eq i64 %_0.i, %l
  br i1 %exitcond.not, label %bb6, label %bb4
}

; Function Attrs: nonlazybind uwtable
declare void @test(i64 noundef) unnamed_addr #0

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.78.0-nightly (3cbb93223 2024-03-13)"}
