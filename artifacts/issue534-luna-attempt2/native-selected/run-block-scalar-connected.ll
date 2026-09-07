define internal fastcc void @_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_(ptr noalias noundef nonnull align 8 dereferenceable(1232) %self, ptr noalias noundef nonnull align 4 %left.0, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef nonnull align 4 %right.0, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %sidechain, i64 noundef %frames, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(320) %reports) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !2526 {
start:
  %iter.i = alloca [56 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 1220, !dbg !2527
  %_9 = load i32, ptr %0, align 4, !dbg !2527, !noundef !12
  %_8 = zext i32 %_9 to i64, !dbg !2529
  %..i = tail call noundef i64 @llvm.umin.i64(i64 %frames, i64 %_8), !dbg !2530
  %_10.not = icmp eq i64 %..i, 0, !dbg !2533
  br i1 %_10.not, label %bb4, label %bb2, !dbg !2533

bb4:                                              ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, %start
  %_18 = icmp ugt i64 %frames, %_8, !dbg !2535
  br i1 %_18, label %bb5, label %bb7, !dbg !2535

bb2:                                              ; preds = %start
  %1 = load ptr, ptr %sidechain, align 8, !dbg !2536, !noundef !12
  %.not = icmp eq ptr %1, null, !dbg !2536
  br i1 %.not, label %bb9, label %bb12, !dbg !2540

bb12:                                             ; preds = %bb2
  %_32.sroa.4.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 8, !dbg !2541
  %_32.sroa.4.0.copyload = load i64, ptr %_32.sroa.4.0.sidechain.sroa_idx, align 8, !dbg !2541
  %_32.sroa.6.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 24, !dbg !2541
  %_32.sroa.6.0.copyload = load i64, ptr %_32.sroa.6.0.sidechain.sroa_idx, align 8, !dbg !2541
  %_9.not.i = icmp ugt i64 %..i, %_32.sroa.4.0.copyload
  br i1 %_9.not.i, label %bb3.i, label %bb1.i, !dbg !2543, !prof !2561

bb3.i:                                            ; preds = %bb12
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i, i64 noundef %_32.sroa.4.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f518473f2bd55505cd4207a72f88500) #24, !dbg !2562, !noalias !2563
  unreachable, !dbg !2562

bb1.i:                                            ; preds = %bb12
  %_32.sroa.5.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 16, !dbg !2541
  %_32.sroa.5.0.copyload = load ptr, ptr %_32.sroa.5.0.sidechain.sroa_idx, align 8, !dbg !2541, !nonnull !12, !noundef !12
  %_17.not.i = icmp ugt i64 %..i, %_32.sroa.6.0.copyload, !dbg !2567
  br i1 %_17.not.i, label %bb6.i, label %bb9, !dbg !2567, !prof !180

bb6.i:                                            ; preds = %bb1.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i, i64 noundef %_32.sroa.6.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_22dea5e023495603116a6a1e47cd356c) #24, !dbg !2573, !noalias !2563
  unreachable, !dbg !2573

bb9:                                              ; preds = %bb2, %bb1.i
  %side.sroa.4.0 = phi ptr [ %_32.sroa.5.0.copyload, %bb1.i ], [ undef, %bb2 ]
  %_33.not = icmp samesign ugt i64 %..i, %left.1
  br i1 %_33.not, label %bb16, label %bb14, !dbg !2574, !prof !2561

bb16:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i, i64 noundef %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_dd5f55065f566218c9f31cb2a4357231) #24, !dbg !2585
  unreachable, !dbg !2585

bb14:                                             ; preds = %bb9
  %_41.not = icmp samesign ugt i64 %..i, %right.1, !dbg !2586
  br i1 %_41.not, label %bb19, label %bb18, !dbg !2586, !prof !180

bb19:                                             ; preds = %bb14
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i, i64 noundef %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1d7cc6e40c752396aa7def7556a6c433) #24, !dbg !2592
  unreachable, !dbg !2592

bb18:                                             ; preds = %bb14
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2593), !dbg !2596
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2597), !dbg !2596
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2599), !dbg !2596
  %_10.i = getelementptr inbounds nuw i8, ptr %self, i64 792, !dbg !2601
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !2604
  %_15.i = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !2618
  %data.i.i546.i = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !2620
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 68, !dbg !2631
  %_29.i = load i32, ptr %2, align 4, !dbg !2631, !range !1335, !alias.scope !2593, !noalias !2635, !noundef !12
  %_31.i = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !2637
  %_33.i = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !2638
  %3 = icmp eq i32 %_29.i, 1, !dbg !2639
  %.val.i.i.i = load i32, ptr %_31.i, align 4, !dbg !2639, !alias.scope !2593, !noalias !2635
  %.val1.i.i.i = load i32, ptr %_33.i, align 4, !dbg !2639, !alias.scope !2593, !noalias !2635
  %_0.i.i.not.i.i.i = icmp eq i32 %.val.i.i.i, %.val1.i.i.i, !dbg !2639
  %spec.select.i = select i1 %_0.i.i.not.i.i.i, i8 1, i8 2, !dbg !2639
  %_0.sroa.0.0.i551.i = select i1 %3, i8 0, i8 %spec.select.i, !dbg !2639
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 744, !dbg !2642
  %_38.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !2644
  %_64.0.i = load ptr, ptr %_15.i, align 8, !dbg !2645, !alias.scope !2593, !noalias !2635, !nonnull !12, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !2645
  %_64.1.i = load i64, ptr %5, align 8, !dbg !2645, !alias.scope !2593, !noalias !2635, !noundef !12
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !2646
  %_65.0.i = load ptr, ptr %6, align 8, !dbg !2646, !alias.scope !2593, !noalias !2635, !nonnull !12, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !2646
  %_65.1.i = load i64, ptr %7, align 8, !dbg !2646, !alias.scope !2593, !noalias !2635, !noundef !12
  %_66.0.i = load ptr, ptr %data.i.i546.i, align 8, !dbg !2647, !alias.scope !2593, !noalias !2635, !nonnull !12, !noundef !12
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !2647
  %_66.1.i = load i64, ptr %8, align 8, !dbg !2647, !alias.scope !2593, !noalias !2635, !noundef !12
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 184, !dbg !2648
  %_67.0.i = load ptr, ptr %9, align 8, !dbg !2648, !alias.scope !2593, !noalias !2635, !nonnull !12, !noundef !12
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 192, !dbg !2648
  %_67.1.i = load i64, ptr %10, align 8, !dbg !2648, !alias.scope !2593, !noalias !2635, !noundef !12
  %_57.i = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !2649
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !2650
  %_58.i = load i32, ptr %11, align 4, !dbg !2650, !alias.scope !2593, !noalias !2635, !noundef !12
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !2651
  %_59.i = load i32, ptr %12, align 8, !dbg !2651, !alias.scope !2593, !noalias !2635, !noundef !12
  br i1 %.not, label %bb27.i.i, label %bb29.i.i, !dbg !2652

bb29.i.i:                                         ; preds = %bb18
  %13 = icmp ne ptr %side.sroa.4.0, null
  tail call void @llvm.assume(i1 %13)
  br label %bb27.i.i, !dbg !2663

bb27.i.i:                                         ; preds = %bb29.i.i, %bb18
  %empty.sroa.0.0.i.i = phi ptr [ %side.sroa.4.0, %bb29.i.i ], [ inttoptr (i64 4 to ptr), %bb18 ], !dbg !2664
  %empty.sroa.6.0.i.i = phi i64 [ %..i, %bb29.i.i ], [ 0, %bb18 ], !dbg !2664
  %side_left.sroa.0.0.i.i = phi ptr [ %1, %bb29.i.i ], [ inttoptr (i64 4 to ptr), %bb18 ], !dbg !2665
  %base.i.i = load i32, ptr %_57.i, align 4, !dbg !2666, !alias.scope !2593, !noalias !2668, !noundef !12
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 856
  %15 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 860
  %18 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %19 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 932
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 784
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 788
  %25 = getelementptr inbounds nuw i8, ptr %self, i64 936
  %26 = getelementptr inbounds nuw i8, ptr %self, i64 776
  %27 = getelementptr inbounds nuw i8, ptr %self, i64 940
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %injected.cond740.not.i = icmp ugt i64 %_64.1.i, %_66.1.i
  br i1 %injected.cond740.not.i, label %bb30.i.us.preheader.i, label %repeat_loop_next15.i.split.us.split.us.i

bb30.i.us.preheader.i:                            ; preds = %bb27.i.i
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i34.i.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %33 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %35 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i34.i.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %36 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i34.i.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.i.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %45 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.i.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.i.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %51 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %52 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %53 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br label %bb30.i.us.i, !dbg !2671

repeat_loop_next15.i.split.us.split.us.i:         ; preds = %bb27.i.i
  %injected.cond765.not.i = icmp samesign ugt i64 %..i, %empty.sroa.6.0.i.i
  br i1 %injected.cond765.not.i, label %bb30.i.us.us.preheader.i, label %repeat_loop_next15.i.split.us.split.us.split.us.i

bb30.i.us.us.preheader.i:                         ; preds = %repeat_loop_next15.i.split.us.split.us.i
  %54 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i34.i.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i34.i.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %60 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i34.i.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %64 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %65 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.i.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.i.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %72 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %73 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %74 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.i.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br label %bb32.i.us.us.i, !dbg !2671

repeat_loop_next15.i.split.us.split.us.split.us.i: ; preds = %repeat_loop_next15.i.split.us.split.us.i
  %injected.cond788.not.i = icmp ugt i64 %_64.1.i, %_65.1.i
  br i1 %injected.cond788.not.i, label %bb30.i.us.us.us.preheader.i, label %repeat_loop_next15.i.split.us.split.us.split.us.split.us.split.us.i

bb30.i.us.us.us.preheader.i:                      ; preds = %repeat_loop_next15.i.split.us.split.us.split.us.i
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i34.i.us.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %82 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i34.i.us.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %84 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %86 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i34.i.us.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %88 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %91 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.i.us.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.i.us.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.i.us.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br label %bb30.i.us.us.us.i, !dbg !2671

repeat_loop_next15.i.split.us.split.us.split.us.split.us.split.us.i: ; preds = %repeat_loop_next15.i.split.us.split.us.split.us.i
  %injected.cond828.not.i = icmp ugt i64 %_64.1.i, %_67.1.i
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %125 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br i1 %injected.cond828.not.i, label %bb30.i.us.us.us.us.us.i, label %bb30.i.us.us.us.us.us.us.us.i

bb30.i.us.us.us.us.us.us.us.i:                    ; preds = %repeat_loop_next15.i.split.us.split.us.split.us.split.us.split.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i
  %iter.sroa.0.0.i714.us.us.us.us.us.us.us.i = phi i64 [ %126, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i ], [ 0, %repeat_loop_next15.i.split.us.split.us.split.us.split.us.split.us.i ]
  %126 = add nuw nsw i64 %iter.sroa.0.0.i714.us.us.us.us.us.us.us.i, 1, !dbg !2683
  %_28.i.us.us.us.us.us.us.us.i = trunc i64 %iter.sroa.0.0.i714.us.us.us.us.us.us.us.i to i32, !dbg !2694
  %now.i.us.us.us.us.us.us.us.i = add i32 %base.i.i, %_28.i.us.us.us.us.us.us.us.i, !dbg !2695
  %_31.i.us.us.us.us.us.us.us.i = and i32 %now.i.us.us.us.us.us.us.us.i, %_58.i, !dbg !2698
  %_30.i.us.us.us.us.us.us.us.i = zext i32 %_31.i.us.us.us.us.us.us.us.i to i64, !dbg !2699
  %exitcond.not.i = icmp eq i64 %iter.sroa.0.0.i714.us.us.us.us.us.us.us.i, %..i, !dbg !2671
  br i1 %exitcond.not.i, label %bb33.i.i, label %bb32.i.us.us.us.us.us.us.us.i, !dbg !2671, !prof !180

bb32.i.us.us.us.us.us.us.us.i:                    ; preds = %bb30.i.us.us.us.us.us.us.us.i
  %_97.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i714.us.us.us.us.us.us.us.i, !dbg !2700
  %_98.not.not.i.us.us.us.us.us.us.us.i = icmp ugt i64 %_64.1.i, %_30.i.us.us.us.us.us.us.us.i, !dbg !2704
  br i1 %_98.not.not.i.us.us.us.us.us.us.us.i, label %bb35.i.us.us.us.us.us.us.us.i, label %bb36.i.i, !dbg !2704, !prof !2709

bb35.i.us.us.us.us.us.us.us.i:                    ; preds = %bb32.i.us.us.us.us.us.us.us.i
  %_0.i213.us.us.us.us.us.us.us.i = load float, ptr %_97.i.us.us.us.us.us.us.us.i, align 4, !dbg !2710, !alias.scope !2716, !noalias !2719, !noundef !12
  %_107.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_30.i.us.us.us.us.us.us.us.i, !dbg !2720
  store float %_0.i213.us.us.us.us.us.us.us.i, ptr %_107.i.us.us.us.us.us.us.us.i, align 4, !dbg !2724, !alias.scope !2727, !noalias !2668
  %_115.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i714.us.us.us.us.us.us.us.i, !dbg !2730
  %_0.i211.us.us.us.us.us.us.us.i = load float, ptr %_115.i.us.us.us.us.us.us.us.i, align 4, !dbg !2737, !alias.scope !2739, !noalias !2742, !noundef !12
  %_123.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_30.i.us.us.us.us.us.us.us.i, !dbg !2743
  store float %_0.i211.us.us.us.us.us.us.us.i, ptr %_123.i.us.us.us.us.us.us.us.i, align 4, !dbg !2750, !alias.scope !2752, !noalias !2668
  %_131.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.us.us.us.us.us.us.i, !dbg !2755
  %_0.i209.us.us.us.us.us.us.us.i = load float, ptr %_131.i.us.us.us.us.us.us.us.i, align 4, !dbg !2762, !alias.scope !2764, !noalias !2668, !noundef !12
  %_139.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_30.i.us.us.us.us.us.us.us.i, !dbg !2767
  store float %_0.i209.us.us.us.us.us.us.us.i, ptr %_139.i.us.us.us.us.us.us.us.i, align 4, !dbg !2774, !alias.scope !2776, !noalias !2668
  %_147.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.us.us.us.us.us.us.i, !dbg !2779
  %_0.i207.us.us.us.us.us.us.us.i = load float, ptr %_147.i.us.us.us.us.us.us.us.i, align 4, !dbg !2786, !alias.scope !2788, !noalias !2668, !noundef !12
  %_155.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_30.i.us.us.us.us.us.us.us.i, !dbg !2791
  store float %_0.i207.us.us.us.us.us.us.us.i, ptr %_155.i.us.us.us.us.us.us.us.i, align 4, !dbg !2798, !alias.scope !2800, !noalias !2668
  %_53.i.us.us.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.us.us.i, %_59.i, !dbg !2803
  %_52.i.us.us.us.us.us.us.us.i = and i32 %_53.i.us.us.us.us.us.us.us.i, %_58.i, !dbg !2806
  %_51.i.us.us.us.us.us.us.us.i = zext i32 %_52.i.us.us.us.us.us.us.us.i to i64, !dbg !2807
  %_156.not.not.i.us.us.us.us.us.us.us.i = icmp ugt i64 %_64.1.i, %_51.i.us.us.us.us.us.us.us.i, !dbg !2808
  br i1 %_156.not.not.i.us.us.us.us.us.us.us.i, label %bb50.i.us.us.us.us.us.us.us.i, label %bb51.i.i, !dbg !2808, !prof !2709

bb50.i.us.us.us.us.us.us.us.i:                    ; preds = %bb35.i.us.us.us.us.us.us.us.i
  %_163.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_51.i.us.us.us.us.us.us.us.i, !dbg !2813
  %_0.i205.us.us.us.us.us.us.us.i = load float, ptr %_163.i.us.us.us.us.us.us.us.i, align 4, !dbg !2817, !alias.scope !2819, !noalias !2668, !noundef !12
  %_169.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_51.i.us.us.us.us.us.us.us.i, !dbg !2822
  %_0.i203.us.us.us.us.us.us.us.i = load float, ptr %_169.i.us.us.us.us.us.us.us.i, align 4, !dbg !2830, !alias.scope !2832, !noalias !2668, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2835), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2841), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2845), !dbg !2838
  %_18.i74.us.us.us.us.us.us.us.i = load i32, ptr %_31.i, align 4, !dbg !2847, !alias.scope !2850, !noalias !2851, !noundef !12
  %_17.i.us.us.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.us.us.i, %_18.i74.us.us.us.us.us.us.us.i, !dbg !2853
  %_16.i75.us.us.us.us.us.us.us.i = and i32 %_17.i.us.us.us.us.us.us.us.i, %_58.i, !dbg !2847
  %_15.i76.us.us.us.us.us.us.us.i = zext i32 %_16.i75.us.us.us.us.us.us.us.i to i64, !dbg !2847
  %_26.i.us.us.us.us.us.us.us.i = load i32, ptr %_33.i, align 4, !dbg !2847, !alias.scope !2856, !noalias !2857, !noundef !12
  %_25.i.us.us.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.us.us.i, %_26.i.us.us.us.us.us.us.us.i, !dbg !2853
  %_24.i.us.us.us.us.us.us.us.i = and i32 %_25.i.us.us.us.us.us.us.us.i, %_58.i, !dbg !2847
  %_23.i79.us.us.us.us.us.us.us.i = zext i32 %_24.i.us.us.us.us.us.us.us.i to i64, !dbg !2847
  %_31.i80.us.us.us.us.us.us.us.i = icmp samesign ugt i64 %_65.1.i, %_15.i76.us.us.us.us.us.us.us.i, !dbg !2847
  switch i8 %_0.sroa.0.0.i551.i, label %bb50.i.us.us.us.us.us.us.us.i.unreachabledefault [
    i8 0, label %bb5.i.preheader.us.us.us.us.us.us.us.i
    i8 1, label %bb14.i63.preheader.us.us.us.us.us.us.us.i
    i8 2, label %bb23.i.preheader.us.us.us.us.us.us.us.i
  ], !dbg !2858

bb27.i60.us.us.us.us.us.us.us.i:                  ; preds = %bb23.i.preheader.us.us.us.us.us.us.us.i
  %127 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.us.us.us.us.i, !dbg !2859
  %_79.i.us.us.us.us.us.us.us.i = load float, ptr %127, align 4, !dbg !2859, !alias.scope !2843, !noalias !2865, !noundef !12
  %_85.i.us.us.us.us.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_15.i76.us.us.us.us.us.us.us.i, !dbg !2866
  br i1 %_85.i.us.us.us.us.us.us.us.i, label %bb29.i61.us.us.us.us.us.us.us.i, label %panic30.i.i, !dbg !2866

bb29.i61.us.us.us.us.us.us.us.i:                  ; preds = %bb27.i60.us.us.us.us.us.us.us.i
  %128 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_15.i76.us.us.us.us.us.us.us.i, !dbg !2866
  %_83.i.us.us.us.us.us.us.us.i = load float, ptr %128, align 4, !dbg !2866, !alias.scope !2845, !noalias !2867, !noundef !12
  %_87.i.us.us.us.us.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.us.us.us.us.i, !dbg !2868
  br i1 %_87.i.us.us.us.us.us.us.us.i, label %bb31.i.us.us.us.us.us.us.us.i, label %panic32.i.i, !dbg !2868

bb31.i.us.us.us.us.us.us.us.i:                    ; preds = %bb29.i61.us.us.us.us.us.us.us.i
  %_89.i.us.us.us.us.us.us.us.i = icmp samesign ugt i64 %_65.1.i, %_23.i79.us.us.us.us.us.us.us.i, !dbg !2869
  br i1 %_89.i.us.us.us.us.us.us.us.i, label %bb33.i62.us.us.us.us.us.us.us.i, label %panic34.i.i, !dbg !2869

bb33.i62.us.us.us.us.us.us.us.i:                  ; preds = %bb31.i.us.us.us.us.us.us.us.i
  %129 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.us.us.us.us.i, !dbg !2868
  %_86.i.us.us.us.us.us.us.us.i = load float, ptr %129, align 4, !dbg !2868, !alias.scope !2845, !noalias !2867, !noundef !12
  %130 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_23.i79.us.us.us.us.us.us.us.i, !dbg !2869
  %_88.i.us.us.us.us.us.us.us.i = load float, ptr %130, align 4, !dbg !2869, !alias.scope !2843, !noalias !2865, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i, !dbg !2870

bb17.i.us.us.us.us.us.us.us.i:                    ; preds = %bb14.i63.preheader.us.us.us.us.us.us.us.i
  %_59.i.us.us.us.us.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.us.us.us.us.i, !dbg !2873
  br i1 %_59.i.us.us.us.us.us.us.us.i, label %bb19.i.us.us.us.us.us.us.us.i, label %panic17.i.i, !dbg !2873

bb19.i.us.us.us.us.us.us.us.i:                    ; preds = %bb17.i.us.us.us.us.us.us.us.i
  %131 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.us.us.us.us.i, !dbg !2880
  %left_own16.i.us.us.us.us.us.us.us.i = load float, ptr %131, align 4, !dbg !2880, !alias.scope !2843, !noalias !2865, !noundef !12
  %132 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.us.us.us.us.i, !dbg !2873
  %right_own18.i.us.us.us.us.us.us.us.i = load float, ptr %132, align 4, !dbg !2873, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i, !dbg !2870

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i: ; preds = %bb10.i82.us.us.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.us.us.i, %bb33.i62.us.us.us.us.us.us.us.i
  %taps.i.sroa.1011534.7.i = phi float [ %right_own.i.us.us.us.us.us.us.us.i, %bb10.i82.us.us.us.us.us.us.us.i ], [ %left_own16.i.us.us.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.us.us.i ], [ %_88.i.us.us.us.us.us.us.us.i, %bb33.i62.us.us.us.us.us.us.us.i ], !dbg !2847
  %taps.i.sroa.681533.7.i = phi float [ %right_own.i.us.us.us.us.us.us.us.i, %bb10.i82.us.us.us.us.us.us.us.i ], [ %right_own18.i.us.us.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.us.us.i ], [ %_86.i.us.us.us.us.us.us.us.i, %bb33.i62.us.us.us.us.us.us.us.i ], !dbg !2847
  %taps.i.sroa.351532.7.i = phi float [ %left_own.i.us.us.us.us.us.us.us.i, %bb10.i82.us.us.us.us.us.us.us.i ], [ %right_own18.i.us.us.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.us.us.i ], [ %_83.i.us.us.us.us.us.us.us.i, %bb33.i62.us.us.us.us.us.us.us.i ], !dbg !2847
  %taps.i.sroa.0.7.i = phi float [ %left_own.i.us.us.us.us.us.us.us.i, %bb10.i82.us.us.us.us.us.us.us.i ], [ %left_own16.i.us.us.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.us.us.i ], [ %_79.i.us.us.us.us.us.us.us.i, %bb33.i62.us.us.us.us.us.us.us.i ], !dbg !2847
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2881), !dbg !2884
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2885), !dbg !2884
  %_12.i36.i.us.us.us.us.us.us.us.i = load float, ptr %102, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.us.us.us.us.i = fcmp ule float %_12.i36.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.us.us.us.us.i = fcmp une float %_12.i36.i.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.us.us.us.us.i = load float, ptr %_10.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.us.us.us.us.i = load float, ptr %103, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.us.us.us.us.i = fadd float %_16.i39.i.us.us.us.us.us.us.us.i, %_17.i40.i.us.us.us.us.us.us.us.i, !dbg !2906
  %_20.i42.i552555.us.us.us.us.us.us.us.i = load float, ptr %104, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %133 = select i1 %_3.i95.us.us.us.us.us.us.us.i, float %_0.i152.us.us.us.us.us.us.us.i, float %_20.i42.i552555.us.us.us.us.us.us.us.i, !dbg !2911
  %_0.i381.us.us.us.us.us.us.us.i = select i1 %_3.i135.us.us.us.us.us.us.us.i, float %_16.i39.i.us.us.us.us.us.us.us.i, float %133, !dbg !2914
  store float %_0.i381.us.us.us.us.us.us.us.i, ptr %_10.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.us.us.us.us.i = select i1 %_3.i95.us.us.us.us.us.us.us.i, float %_17.i40.i.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.us.us.us.us.i, ptr %103, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.us.us.us.us.i = fadd float %_12.i36.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.us.us.us.us.i = select i1 %_3.i135.us.us.us.us.us.us.us.i, float %_12.i36.i.us.us.us.us.us.us.us.i, float %_0.i193.us.us.us.us.us.us.us.i, !dbg !2923
  store float %_4.i367.v.us.us.us.us.us.us.us.i, ptr %102, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.us.us.us.us.1.i = load float, ptr %105, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.us.us.us.us.1.i = fcmp ule float %_12.i36.i.us.us.us.us.us.us.us.1.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.us.us.us.us.1.i = fcmp une float %_12.i36.i.us.us.us.us.us.us.us.1.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.us.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.1.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.us.us.us.us.1.i = load float, ptr %106, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.us.us.us.us.1.i = fadd float %_16.i39.i.us.us.us.us.us.us.us.1.i, %_17.i40.i.us.us.us.us.us.us.us.1.i, !dbg !2906
  %_20.i42.i552555.us.us.us.us.us.us.us.1.i = load float, ptr %107, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %134 = select i1 %_3.i95.us.us.us.us.us.us.us.1.i, float %_0.i152.us.us.us.us.us.us.us.1.i, float %_20.i42.i552555.us.us.us.us.us.us.us.1.i, !dbg !2911
  %_0.i381.us.us.us.us.us.us.us.1.i = select i1 %_3.i135.us.us.us.us.us.us.us.1.i, float %_16.i39.i.us.us.us.us.us.us.us.1.i, float %134, !dbg !2914
  store float %_0.i381.us.us.us.us.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.1.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.us.us.us.us.1.i = select i1 %_3.i95.us.us.us.us.us.us.us.1.i, float %_17.i40.i.us.us.us.us.us.us.us.1.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.us.us.us.us.1.i, ptr %106, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.us.us.us.us.1.i = fadd float %_12.i36.i.us.us.us.us.us.us.us.1.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.us.us.us.us.1.i = select i1 %_3.i135.us.us.us.us.us.us.us.1.i, float %_12.i36.i.us.us.us.us.us.us.us.1.i, float %_0.i193.us.us.us.us.us.us.us.1.i, !dbg !2923
  store float %_4.i367.v.us.us.us.us.us.us.us.1.i, ptr %105, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.us.us.us.us.2.i = load float, ptr %108, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.us.us.us.us.2.i = fcmp ule float %_12.i36.i.us.us.us.us.us.us.us.2.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.us.us.us.us.2.i = fcmp une float %_12.i36.i.us.us.us.us.us.us.us.2.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.us.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.2.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.us.us.us.us.2.i = load float, ptr %109, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.us.us.us.us.2.i = fadd float %_16.i39.i.us.us.us.us.us.us.us.2.i, %_17.i40.i.us.us.us.us.us.us.us.2.i, !dbg !2906
  %_20.i42.i552555.us.us.us.us.us.us.us.2.i = load float, ptr %110, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %135 = select i1 %_3.i95.us.us.us.us.us.us.us.2.i, float %_0.i152.us.us.us.us.us.us.us.2.i, float %_20.i42.i552555.us.us.us.us.us.us.us.2.i, !dbg !2911
  %_0.i381.us.us.us.us.us.us.us.2.i = select i1 %_3.i135.us.us.us.us.us.us.us.2.i, float %_16.i39.i.us.us.us.us.us.us.us.2.i, float %135, !dbg !2914
  store float %_0.i381.us.us.us.us.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.2.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.us.us.us.us.2.i = select i1 %_3.i95.us.us.us.us.us.us.us.2.i, float %_17.i40.i.us.us.us.us.us.us.us.2.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.us.us.us.us.2.i, ptr %109, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.us.us.us.us.2.i = fadd float %_12.i36.i.us.us.us.us.us.us.us.2.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.us.us.us.us.2.i = select i1 %_3.i135.us.us.us.us.us.us.us.2.i, float %_12.i36.i.us.us.us.us.us.us.us.2.i, float %_0.i193.us.us.us.us.us.us.us.2.i, !dbg !2923
  store float %_4.i367.v.us.us.us.us.us.us.us.2.i, ptr %108, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.us.us.us.us.3.i = load float, ptr %111, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.us.us.us.us.3.i = fcmp ule float %_12.i36.i.us.us.us.us.us.us.us.3.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.us.us.us.us.3.i = fcmp une float %_12.i36.i.us.us.us.us.us.us.us.3.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.us.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.3.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.us.us.us.us.3.i = load float, ptr %112, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.us.us.us.us.3.i = fadd float %_16.i39.i.us.us.us.us.us.us.us.3.i, %_17.i40.i.us.us.us.us.us.us.us.3.i, !dbg !2906
  %_20.i42.i552555.us.us.us.us.us.us.us.3.i = load float, ptr %113, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %136 = select i1 %_3.i95.us.us.us.us.us.us.us.3.i, float %_0.i152.us.us.us.us.us.us.us.3.i, float %_20.i42.i552555.us.us.us.us.us.us.us.3.i, !dbg !2911
  %_0.i381.us.us.us.us.us.us.us.3.i = select i1 %_3.i135.us.us.us.us.us.us.us.3.i, float %_16.i39.i.us.us.us.us.us.us.us.3.i, float %136, !dbg !2914
  store float %_0.i381.us.us.us.us.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.3.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.us.us.us.us.3.i = select i1 %_3.i95.us.us.us.us.us.us.us.3.i, float %_17.i40.i.us.us.us.us.us.us.us.3.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.us.us.us.us.3.i, ptr %112, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.us.us.us.us.3.i = fadd float %_12.i36.i.us.us.us.us.us.us.us.3.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.us.us.us.us.3.i = select i1 %_3.i135.us.us.us.us.us.us.us.3.i, float %_12.i36.i.us.us.us.us.us.us.us.3.i, float %_0.i193.us.us.us.us.us.us.us.3.i, !dbg !2923
  store float %_4.i367.v.us.us.us.us.us.us.us.3.i, ptr %111, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %137 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.7.i), !dbg !2926
  %138 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351532.7.i), !dbg !2933
  %_37.i57.i.us.us.us.us.us.us.us.i = load float, ptr %15, align 4, !dbg !2936, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i133.us.us.us.us.us.us.us.i = fcmp ule float %_37.i57.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !2941
  %_3.i.i489.us.us.us.us.us.us.us.i = fcmp ule float %137, %138, !dbg !2943
  %_6.i.i491.us.us.us.us.us.us.us.i = bitcast float %137 to i32, !dbg !2949
  %_8.i.i493.us.us.us.us.us.us.us.i = bitcast float %138 to i32, !dbg !2953
  %_4.i.i496.us.us.us.us.us.us.us.i = select i1 %_3.i.i489.us.us.us.us.us.us.us.i, i32 %_8.i.i493.us.us.us.us.us.us.us.i, i32 %_6.i.i491.us.us.us.us.us.us.us.i, !dbg !2955
  %_4.i360.us.us.us.us.us.us.us.i = select i1 %_3.i133.us.us.us.us.us.us.us.i, i32 %_6.i.i491.us.us.us.us.us.us.us.i, i32 %_4.i.i496.us.us.us.us.us.us.us.i, !dbg !2956
  %_41.i61.i.us.us.us.us.us.us.us.i = load float, ptr %16, align 4, !dbg !2958, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i131.us.us.us.us.us.us.us.i = fcmp ule float %_41.i61.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !2959
  %_0.i177.us.us.us.us.us.us.us.i = fmul float %137, 5.000000e-01, !dbg !2961
  %_0.i176.us.us.us.us.us.us.us.i = fmul float %138, 5.000000e-01, !dbg !2964
  %_0.i151.us.us.us.us.us.us.us.i = fadd float %_0.i176.us.us.us.us.us.us.us.i, %_0.i177.us.us.us.us.us.us.us.i, !dbg !2966
  %_6.i348.us.us.us.us.us.us.us.i = bitcast float %_0.i151.us.us.us.us.us.us.us.i to i32, !dbg !2968
  %_4.i353.us.us.us.us.us.us.us.i = select i1 %_3.i131.us.us.us.us.us.us.us.i, i32 %_4.i360.us.us.us.us.us.us.us.i, i32 %_6.i348.us.us.us.us.us.us.us.i, !dbg !2971
  %_0.i354.us.us.us.us.us.us.us.i = bitcast i32 %_4.i353.us.us.us.us.us.us.us.i to float, !dbg !2972
  %_3.i.i481.us.us.us.us.us.us.us.i = fcmp ule float %_0.i354.us.us.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !2975
  %_4.i.i487.us.us.us.us.us.us.us.i = select i1 %_3.i.i481.us.us.us.us.us.us.us.i, i32 841731191, i32 %_4.i353.us.us.us.us.us.us.us.i, !dbg !2978
  %_0.i.i488.us.us.us.us.us.us.us.i = bitcast i32 %_4.i.i487.us.us.us.us.us.us.us.i to float, !dbg !2980
  %_3.i.i.us.us.us.us.us.us.us.i = fcmp ule float %_0.i.i488.us.us.us.us.us.us.us.i, 0x3810000000000000, !dbg !2982
  %_4.i.i.us.us.us.us.us.us.us.i = select i1 %_3.i.i.us.us.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i487.us.us.us.us.us.us.us.i, !dbg !2992
  %_5.i214.us.us.us.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.us.us.us.i, 8388607, !dbg !2994
  %_4.i215.us.us.us.us.us.us.us.i = or disjoint i32 %_5.i214.us.us.us.us.us.us.us.i, 1065353216, !dbg !2994
  %significand.i.us.us.us.us.us.us.us.i = bitcast i32 %_4.i215.us.us.us.us.us.us.us.i to float, !dbg !2999
  %_0.i178.us.us.us.us.us.us.us.i = fadd float %significand.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3002
  %_0.i158.us.us.us.us.us.us.us.i = fmul float %_0.i178.us.us.us.us.us.us.us.i, 0x3F9B17A960000000, !dbg !3005
  %139 = fsub float 0x3FBF9A8440000000, %_0.i158.us.us.us.us.us.us.us.i, !dbg !3010
  %_0.i158.us.us.us.us.us.us.us.1.i = fmul float %_0.i178.us.us.us.us.us.us.us.i, %139, !dbg !3005
  %_0.i142.us.us.us.us.us.us.us.1.i = fadd float %_0.i158.us.us.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3010
  %_0.i158.us.us.us.us.us.us.us.2.i = fmul float %_0.i178.us.us.us.us.us.us.us.i, %_0.i142.us.us.us.us.us.us.us.1.i, !dbg !3005
  %_0.i142.us.us.us.us.us.us.us.2.i = fadd float %_0.i158.us.us.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3010
  %_0.i158.us.us.us.us.us.us.us.3.i = fmul float %_0.i178.us.us.us.us.us.us.us.i, %_0.i142.us.us.us.us.us.us.us.2.i, !dbg !3005
  %_0.i142.us.us.us.us.us.us.us.3.i = fadd float %_0.i158.us.us.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3010
  %_0.i158.us.us.us.us.us.us.us.4.i = fmul float %_0.i178.us.us.us.us.us.us.us.i, %_0.i142.us.us.us.us.us.us.us.3.i, !dbg !3005
  %_0.i142.us.us.us.us.us.us.us.4.i = fadd float %_0.i158.us.us.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3010
  %_9.i.us.us.us.us.us.us.us.i = lshr i32 %_4.i.i.us.us.us.us.us.us.us.i, 23, !dbg !3012
  %_8.i216.us.us.us.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.us.us.us.i, 1258291200, !dbg !3012
  %_7.i.us.us.us.us.us.us.us.i = bitcast i32 %_8.i216.us.us.us.us.us.us.us.i to float, !dbg !3014
  %exponent.i.us.us.us.us.us.us.us.i = fadd float %_7.i.us.us.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !3016
  %_0.i157.us.us.us.us.us.us.us.i = fmul float %_0.i178.us.us.us.us.us.us.us.i, %_0.i142.us.us.us.us.us.us.us.4.i, !dbg !3017
  %_0.i141.us.us.us.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.us.us.us.i, %_0.i157.us.us.us.us.us.us.us.i, !dbg !3019
  %_0.i175.us.us.us.us.us.us.us.i = fmul float %_0.i141.us.us.us.us.us.us.us.i, 0x4018151820000000, !dbg !3021
  %_3.i.i538.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i175.us.us.us.us.us.us.us.i, 2.400000e+01, !dbg !3023
  %_0.i.i545.us.us.us.us.us.us.us.i = select i1 %_3.i.i538.us.us.us.us.us.us.us.inv.i, float %_0.i175.us.us.us.us.us.us.us.i, float 2.400000e+01, !dbg !3023
  %_3.i.i473.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i545.us.us.us.us.us.us.us.i, -1.600000e+02, !dbg !3027
  %_0.i.i480.us.us.us.us.us.us.us.i = select i1 %_3.i.i473.us.us.us.us.us.us.us.inv.i, float %_0.i.i545.us.us.us.us.us.us.us.i, float -1.600000e+02, !dbg !3027
  %_55.i78.i.us.us.us.us.us.us.us.i = load float, ptr %14, align 4, !dbg !3030, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i129.us.us.us.us.us.us.us.i = fcmp ule float %_55.i78.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3032
  %_3.i103.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i480.us.us.us.us.us.us.us.i, %_0.i381.us.us.us.us.us.us.us.i, !dbg !3034
  %_0.i192.us.us.us.us.us.us.us.i = fsub float %_0.i381.us.us.us.us.us.us.us.i, %_0.i381.us.us.us.us.us.us.us.3.i, !dbg !3038
  %_3.i101.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i480.us.us.us.us.us.us.us.i, %_0.i192.us.us.us.us.us.us.us.i, !dbg !3041
  %..i102.us.us.us.us.us.us.us.i = sext i1 %_3.i101.us.us.us.us.us.us.us.i to i32, !dbg !3043
  %_0.i401.us.us.us.us.us.us.us.i = sext i1 %_3.i103.us.us.us.us.us.us.us.i to i32, !dbg !3046
  %_0.i394.us.us.us.us.us.us.us.i = select i1 %_3.i129.us.us.us.us.us.us.us.i, i32 %_0.i401.us.us.us.us.us.us.us.i, i32 %..i102.us.us.us.us.us.us.us.i, !dbg !3046
  %_0.i405.us.us.us.us.us.us.us.i = xor i32 %..i102.us.us.us.us.us.us.us.i, -1, !dbg !3052
  %_67.i88.i.us.us.us.us.us.us.us.i = load float, ptr %17, align 4, !dbg !3056, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i127.us.us.us.us.us.us.us.i = fcmp ogt float %_67.i88.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3057
  %_0.i400.us.us.us.us.us.us.us.i = select i1 %_3.i127.us.us.us.us.us.us.us.i, i32 %_0.i405.us.us.us.us.us.us.us.i, i32 0, !dbg !3059
  %_0.i399.us.us.us.us.us.us.us.i = select i1 %_3.i129.us.us.us.us.us.us.us.i, i32 0, i32 %_0.i400.us.us.us.us.us.us.us.i, !dbg !3062
  %_0.i393.us.us.us.us.us.us.us.i = or i32 %_0.i399.us.us.us.us.us.us.us.i, %_0.i394.us.us.us.us.us.us.us.i, !dbg !3064
  %_5.i343.us.us.us.us.us.us.us.i = and i32 %_0.i393.us.us.us.us.us.us.us.i, 1065353216, !dbg !3067
  %_0.i347.us.us.us.us.us.us.us.i = bitcast i32 %_5.i343.us.us.us.us.us.us.us.i to float, !dbg !3069
  %_71.i94.i564565.us.us.us.us.us.us.us.i = load float, ptr %18, align 4, !dbg !3071, !alias.scope !2939, !noalias !2940, !noundef !12
  %_0.i191.us.us.us.us.us.us.us.i = fadd float %_67.i88.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3073
  %140 = trunc nsw i32 %_0.i399.us.us.us.us.us.us.us.i to i1, !dbg !3075
  %_4.i341.v.us.us.us.us.us.us.us.i = select i1 %140, float %_0.i191.us.us.us.us.us.us.us.i, float %_67.i88.i.us.us.us.us.us.us.us.i, !dbg !3075
  %141 = trunc nsw i32 %_0.i394.us.us.us.us.us.us.us.i to i1, !dbg !3077
  %_0.i335.us.us.us.us.us.us.us.i = select i1 %141, float %_71.i94.i564565.us.us.us.us.us.us.us.i, float %_4.i341.v.us.us.us.us.us.us.us.i, !dbg !3077
  store float %_0.i335.us.us.us.us.us.us.us.i, ptr %17, align 4, !dbg !3079, !alias.scope !2894, !noalias !2895
  store i32 %_5.i343.us.us.us.us.us.us.us.i, ptr %14, align 4, !dbg !3080, !alias.scope !2894, !noalias !2895
  %_0.i190.us.us.us.us.us.us.us.i = fadd float %_0.i381.us.us.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3081
  %_0.i189.us.us.us.us.us.us.us.i = fsub float %_0.i.i480.us.us.us.us.us.us.us.i, %_0.i381.us.us.us.us.us.us.us.i, !dbg !3083
  %_0.i174.us.us.us.us.us.us.us.i = fmul float %_0.i190.us.us.us.us.us.us.us.i, %_0.i189.us.us.us.us.us.us.us.i, !dbg !3085
  %142 = fneg float %_0.i381.us.us.us.us.us.us.us.2.i, !dbg !3087
  %_3.i.i464.inv.us.us.us.us.us.us.us.i = fcmp ogt float %_0.i174.us.us.us.us.us.us.us.i, %142, !dbg !3090
  %_4.i.i471.v.us.us.us.us.us.us.us.i = select i1 %_3.i.i464.inv.us.us.us.us.us.us.us.i, float %_0.i174.us.us.us.us.us.us.us.i, float %142, !dbg !3090
  %_3.i.i530.us.us.us.us.us.us.us.i = fcmp olt float %_4.i.i471.v.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3093
  %143 = fcmp ule float %_0.i347.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3097
  %144 = select i1 %143, i1 %_3.i.i530.us.us.us.us.us.us.us.i, i1 false, !dbg !3100
  %_0.i328.us.us.us.us.us.us.us.i = select i1 %144, float %_4.i.i471.v.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3100
  %_86.i107.i.us.us.us.us.us.us.us.i = load float, ptr %19, align 4, !dbg !3101, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i123.us.us.us.us.us.us.us.i = fcmp ule float %_0.i328.us.us.us.us.us.us.us.i, %_86.i107.i.us.us.us.us.us.us.us.i, !dbg !3103
  %_87.i109.i567.us.us.us.us.us.us.us.i = load i32, ptr %4, align 4, !dbg !3105, !alias.scope !2939, !noalias !2940, !noundef !12
  %_88.i110.i568.us.us.us.us.us.us.us.i = load i32, ptr %20, align 4, !dbg !3106, !alias.scope !2939, !noalias !2940, !noundef !12
  %_4.i321.us.us.us.us.us.us.us.i = select i1 %_3.i123.us.us.us.us.us.us.us.i, i32 %_88.i110.i568.us.us.us.us.us.us.us.i, i32 %_87.i109.i567.us.us.us.us.us.us.us.i, !dbg !3107
  %_0.i322.us.us.us.us.us.us.us.i = bitcast i32 %_4.i321.us.us.us.us.us.us.us.i to float, !dbg !3109
  %_0.i188.us.us.us.us.us.us.us.i = fsub float %_0.i328.us.us.us.us.us.us.us.i, %_86.i107.i.us.us.us.us.us.us.us.i, !dbg !3111
  %_4.i155.us.us.us.us.us.us.us.i = fmul float %_0.i188.us.us.us.us.us.us.us.i, %_0.i322.us.us.us.us.us.us.us.i, !dbg !3114
  %_0.i156.us.us.us.us.us.us.us.i = fadd float %_86.i107.i.us.us.us.us.us.us.us.i, %_4.i155.us.us.us.us.us.us.us.i, !dbg !3114
  %145 = tail call noundef float @llvm.fabs.f32(float %_0.i156.us.us.us.us.us.us.us.i), !dbg !3117
  %146 = fcmp uge float %145, 0x3BC79CA100000000, !dbg !3121
  %_0.i230.us.us.us.us.us.us.us.i = select i1 %146, float %_0.i156.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3124
  store float %_0.i230.us.us.us.us.us.us.us.i, ptr %19, align 4, !dbg !3125, !alias.scope !2894, !noalias !2895
  %_0.i173.us.us.us.us.us.us.us.i = fmul float %_0.i230.us.us.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !3127
  %_3.i.i415.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i173.us.us.us.us.us.us.us.i, -1.260000e+02, !dbg !3131
  %_0.i.i422.us.us.us.us.us.us.us.i = select i1 %_3.i.i415.us.us.us.us.us.us.us.inv.i, float %_0.i173.us.us.us.us.us.us.us.i, float -1.260000e+02, !dbg !3131
  %_3.i.i498.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i422.us.us.us.us.us.us.us.i, 1.270000e+02, !dbg !3136
  %_0.i.i505.us.us.us.us.us.us.us.i = select i1 %_3.i.i498.us.us.us.us.us.us.us.inv.i, float %_0.i.i422.us.us.us.us.us.us.us.i, float 1.270000e+02, !dbg !3136
  %147 = tail call noundef float @llvm.floor.f32(float %_0.i.i505.us.us.us.us.us.us.us.i), !dbg !3139
  %_0.i180.us.us.us.us.us.us.us.i = fsub float %_0.i.i505.us.us.us.us.us.us.us.i, %147, !dbg !3151
  %_98.i123.i.us.us.us.us.us.us.us.i = load float, ptr %21, align 4, !dbg !3154, !alias.scope !2939, !noalias !2940, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3156), !dbg !3159
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3161), !dbg !3159
  %_12.i.i.us.us.us.us.us.us.us.i = load float, ptr %114, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.us.us.us.us.i = fcmp ule float %_12.i.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.us.us.us.us.i = fcmp une float %_12.i.i.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.us.us.us.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.us.us.us.us.i = load float, ptr %115, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.us.us.us.us.i = fadd float %_16.i.i.us.us.us.us.us.us.us.i, %_17.i.i.us.us.us.us.us.us.us.i, !dbg !3173
  %_20.i.i574577.us.us.us.us.us.us.us.i = load float, ptr %116, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %148 = select i1 %_3.i91.us.us.us.us.us.us.us.i, float %_0.i150.us.us.us.us.us.us.us.i, float %_20.i.i574577.us.us.us.us.us.us.us.i, !dbg !3176
  %_0.i301.us.us.us.us.us.us.us.i = select i1 %_3.i119.us.us.us.us.us.us.us.i, float %_16.i.i.us.us.us.us.us.us.us.i, float %148, !dbg !3178
  store float %_0.i301.us.us.us.us.us.us.us.i, ptr %data.i.i.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.us.us.us.us.i = select i1 %_3.i91.us.us.us.us.us.us.us.i, float %_17.i.i.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.us.us.us.us.i, ptr %115, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.us.us.us.us.i = fadd float %_12.i.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.us.us.us.us.i = select i1 %_3.i119.us.us.us.us.us.us.us.i, float %_12.i.i.us.us.us.us.us.us.us.i, float %_0.i187.us.us.us.us.us.us.us.i, !dbg !3186
  store float %_4.i287.v.us.us.us.us.us.us.us.i, ptr %114, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.us.us.us.us.1.i = load float, ptr %117, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.us.us.us.us.1.i = fcmp ule float %_12.i.i.us.us.us.us.us.us.us.1.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.us.us.us.us.1.i = fcmp une float %_12.i.i.us.us.us.us.us.us.us.1.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.us.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.1.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.us.us.us.us.1.i = load float, ptr %118, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.us.us.us.us.1.i = fadd float %_16.i.i.us.us.us.us.us.us.us.1.i, %_17.i.i.us.us.us.us.us.us.us.1.i, !dbg !3173
  %_20.i.i574577.us.us.us.us.us.us.us.1.i = load float, ptr %119, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %149 = select i1 %_3.i91.us.us.us.us.us.us.us.1.i, float %_0.i150.us.us.us.us.us.us.us.1.i, float %_20.i.i574577.us.us.us.us.us.us.us.1.i, !dbg !3176
  %_0.i301.us.us.us.us.us.us.us.1.i = select i1 %_3.i119.us.us.us.us.us.us.us.1.i, float %_16.i.i.us.us.us.us.us.us.us.1.i, float %149, !dbg !3178
  store float %_0.i301.us.us.us.us.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.1.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.us.us.us.us.1.i = select i1 %_3.i91.us.us.us.us.us.us.us.1.i, float %_17.i.i.us.us.us.us.us.us.us.1.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.us.us.us.us.1.i, ptr %118, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.us.us.us.us.1.i = fadd float %_12.i.i.us.us.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.us.us.us.us.1.i = select i1 %_3.i119.us.us.us.us.us.us.us.1.i, float %_12.i.i.us.us.us.us.us.us.us.1.i, float %_0.i187.us.us.us.us.us.us.us.1.i, !dbg !3186
  store float %_4.i287.v.us.us.us.us.us.us.us.1.i, ptr %117, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.us.us.us.us.2.i = load float, ptr %120, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.us.us.us.us.2.i = fcmp ule float %_12.i.i.us.us.us.us.us.us.us.2.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.us.us.us.us.2.i = fcmp une float %_12.i.i.us.us.us.us.us.us.us.2.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.us.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.2.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.us.us.us.us.2.i = load float, ptr %121, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.us.us.us.us.2.i = fadd float %_16.i.i.us.us.us.us.us.us.us.2.i, %_17.i.i.us.us.us.us.us.us.us.2.i, !dbg !3173
  %_20.i.i574577.us.us.us.us.us.us.us.2.i = load float, ptr %122, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %150 = select i1 %_3.i91.us.us.us.us.us.us.us.2.i, float %_0.i150.us.us.us.us.us.us.us.2.i, float %_20.i.i574577.us.us.us.us.us.us.us.2.i, !dbg !3176
  %_0.i301.us.us.us.us.us.us.us.2.i = select i1 %_3.i119.us.us.us.us.us.us.us.2.i, float %_16.i.i.us.us.us.us.us.us.us.2.i, float %150, !dbg !3178
  store float %_0.i301.us.us.us.us.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.2.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.us.us.us.us.2.i = select i1 %_3.i91.us.us.us.us.us.us.us.2.i, float %_17.i.i.us.us.us.us.us.us.us.2.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.us.us.us.us.2.i, ptr %121, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.us.us.us.us.2.i = fadd float %_12.i.i.us.us.us.us.us.us.us.2.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.us.us.us.us.2.i = select i1 %_3.i119.us.us.us.us.us.us.us.2.i, float %_12.i.i.us.us.us.us.us.us.us.2.i, float %_0.i187.us.us.us.us.us.us.us.2.i, !dbg !3186
  store float %_4.i287.v.us.us.us.us.us.us.us.2.i, ptr %120, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.us.us.us.us.3.i = load float, ptr %123, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.us.us.us.us.3.i = fcmp ule float %_12.i.i.us.us.us.us.us.us.us.3.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.us.us.us.us.3.i = fcmp une float %_12.i.i.us.us.us.us.us.us.us.3.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.us.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.3.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.us.us.us.us.3.i = load float, ptr %124, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.us.us.us.us.3.i = fadd float %_16.i.i.us.us.us.us.us.us.us.3.i, %_17.i.i.us.us.us.us.us.us.us.3.i, !dbg !3173
  %_20.i.i574577.us.us.us.us.us.us.us.3.i = load float, ptr %125, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %151 = select i1 %_3.i91.us.us.us.us.us.us.us.3.i, float %_0.i150.us.us.us.us.us.us.us.3.i, float %_20.i.i574577.us.us.us.us.us.us.us.3.i, !dbg !3176
  %_0.i301.us.us.us.us.us.us.us.3.i = select i1 %_3.i119.us.us.us.us.us.us.us.3.i, float %_16.i.i.us.us.us.us.us.us.us.3.i, float %151, !dbg !3178
  store float %_0.i301.us.us.us.us.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.3.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.us.us.us.us.3.i = select i1 %_3.i91.us.us.us.us.us.us.us.3.i, float %_17.i.i.us.us.us.us.us.us.us.3.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.us.us.us.us.3.i, ptr %124, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.us.us.us.us.3.i = fadd float %_12.i.i.us.us.us.us.us.us.us.3.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.us.us.us.us.3.i = select i1 %_3.i119.us.us.us.us.us.us.us.3.i, float %_12.i.i.us.us.us.us.us.us.us.3.i, float %_0.i187.us.us.us.us.us.us.us.3.i, !dbg !3186
  store float %_4.i287.v.us.us.us.us.us.us.us.3.i, ptr %123, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %152 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681533.7.i), !dbg !3189
  %153 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011534.7.i), !dbg !3191
  %_37.i.i.us.us.us.us.us.us.us.i = load float, ptr %23, align 4, !dbg !3193, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i117.us.us.us.us.us.us.us.i = fcmp ule float %_37.i.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3196
  %_3.i.i455.us.us.us.us.us.us.us.i = fcmp ule float %152, %153, !dbg !3198
  %_6.i.i457.us.us.us.us.us.us.us.i = bitcast float %152 to i32, !dbg !3201
  %_8.i.i459.us.us.us.us.us.us.us.i = bitcast float %153 to i32, !dbg !3204
  %_4.i.i462.us.us.us.us.us.us.us.i = select i1 %_3.i.i455.us.us.us.us.us.us.us.i, i32 %_8.i.i459.us.us.us.us.us.us.us.i, i32 %_6.i.i457.us.us.us.us.us.us.us.i, !dbg !3206
  %_4.i280.us.us.us.us.us.us.us.i = select i1 %_3.i117.us.us.us.us.us.us.us.i, i32 %_6.i.i457.us.us.us.us.us.us.us.i, i32 %_4.i.i462.us.us.us.us.us.us.us.i, !dbg !3207
  %_41.i.i.us.us.us.us.us.us.us.i = load float, ptr %24, align 4, !dbg !3209, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i115.us.us.us.us.us.us.us.i = fcmp ule float %_41.i.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3210
  %_0.i171.us.us.us.us.us.us.us.i = fmul float %152, 5.000000e-01, !dbg !3212
  %_0.i170.us.us.us.us.us.us.us.i = fmul float %153, 5.000000e-01, !dbg !3214
  %_0.i149.us.us.us.us.us.us.us.i = fadd float %_0.i170.us.us.us.us.us.us.us.i, %_0.i171.us.us.us.us.us.us.us.i, !dbg !3216
  %_6.i268.us.us.us.us.us.us.us.i = bitcast float %_0.i149.us.us.us.us.us.us.us.i to i32, !dbg !3218
  %_4.i273.us.us.us.us.us.us.us.i = select i1 %_3.i115.us.us.us.us.us.us.us.i, i32 %_4.i280.us.us.us.us.us.us.us.i, i32 %_6.i268.us.us.us.us.us.us.us.i, !dbg !3221
  %_0.i274.us.us.us.us.us.us.us.i = bitcast i32 %_4.i273.us.us.us.us.us.us.us.i to float, !dbg !3222
  %_3.i.i447.us.us.us.us.us.us.us.i = fcmp ule float %_0.i274.us.us.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !3224
  %_4.i.i453.us.us.us.us.us.us.us.i = select i1 %_3.i.i447.us.us.us.us.us.us.us.i, i32 841731191, i32 %_4.i273.us.us.us.us.us.us.us.i, !dbg !3227
  %_0.i.i454.us.us.us.us.us.us.us.i = bitcast i32 %_4.i.i453.us.us.us.us.us.us.us.i to float, !dbg !3229
  %_3.i.i407.us.us.us.us.us.us.us.i = fcmp ule float %_0.i.i454.us.us.us.us.us.us.us.i, 0x3810000000000000, !dbg !3231
  %_4.i.i413.us.us.us.us.us.us.us.i = select i1 %_3.i.i407.us.us.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i453.us.us.us.us.us.us.us.i, !dbg !3236
  %_5.i218.us.us.us.us.us.us.us.i = and i32 %_4.i.i413.us.us.us.us.us.us.us.i, 8388607, !dbg !3238
  %_4.i219.us.us.us.us.us.us.us.i = or disjoint i32 %_5.i218.us.us.us.us.us.us.us.i, 1065353216, !dbg !3238
  %significand.i220.us.us.us.us.us.us.us.i = bitcast i32 %_4.i219.us.us.us.us.us.us.us.i to float, !dbg !3240
  %_0.i179.us.us.us.us.us.us.us.i = fadd float %significand.i220.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3242
  %_0.i160.us.us.us.us.us.us.us.i = fmul float %_0.i179.us.us.us.us.us.us.us.i, 0x3F9B17A960000000, !dbg !3244
  %154 = fsub float 0x3FBF9A8440000000, %_0.i160.us.us.us.us.us.us.us.i, !dbg !3246
  %_0.i160.us.us.us.us.us.us.us.1.i = fmul float %_0.i179.us.us.us.us.us.us.us.i, %154, !dbg !3244
  %_0.i144.us.us.us.us.us.us.us.1.i = fadd float %_0.i160.us.us.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3246
  %_0.i160.us.us.us.us.us.us.us.2.i = fmul float %_0.i179.us.us.us.us.us.us.us.i, %_0.i144.us.us.us.us.us.us.us.1.i, !dbg !3244
  %_0.i144.us.us.us.us.us.us.us.2.i = fadd float %_0.i160.us.us.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3246
  %_0.i160.us.us.us.us.us.us.us.3.i = fmul float %_0.i179.us.us.us.us.us.us.us.i, %_0.i144.us.us.us.us.us.us.us.2.i, !dbg !3244
  %_0.i144.us.us.us.us.us.us.us.3.i = fadd float %_0.i160.us.us.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3246
  %_0.i160.us.us.us.us.us.us.us.4.i = fmul float %_0.i179.us.us.us.us.us.us.us.i, %_0.i144.us.us.us.us.us.us.us.3.i, !dbg !3244
  %_0.i144.us.us.us.us.us.us.us.4.i = fadd float %_0.i160.us.us.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3246
  %_9.i221.us.us.us.us.us.us.us.i = lshr i32 %_4.i.i413.us.us.us.us.us.us.us.i, 23, !dbg !3248
  %_8.i222.us.us.us.us.us.us.us.i = or disjoint i32 %_9.i221.us.us.us.us.us.us.us.i, 1258291200, !dbg !3248
  %_7.i223.us.us.us.us.us.us.us.i = bitcast i32 %_8.i222.us.us.us.us.us.us.us.i to float, !dbg !3249
  %exponent.i224.us.us.us.us.us.us.us.i = fadd float %_7.i223.us.us.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !3251
  %_0.i159.us.us.us.us.us.us.us.i = fmul float %_0.i179.us.us.us.us.us.us.us.i, %_0.i144.us.us.us.us.us.us.us.4.i, !dbg !3252
  %_0.i143.us.us.us.us.us.us.us.i = fadd float %exponent.i224.us.us.us.us.us.us.us.i, %_0.i159.us.us.us.us.us.us.us.i, !dbg !3254
  %_0.i169.us.us.us.us.us.us.us.i = fmul float %_0.i143.us.us.us.us.us.us.us.i, 0x4018151820000000, !dbg !3256
  %_3.i.i522.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i169.us.us.us.us.us.us.us.i, 2.400000e+01, !dbg !3258
  %_0.i.i529.us.us.us.us.us.us.us.i = select i1 %_3.i.i522.us.us.us.us.us.us.us.inv.i, float %_0.i169.us.us.us.us.us.us.us.i, float 2.400000e+01, !dbg !3258
  %_3.i.i439.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i529.us.us.us.us.us.us.us.i, -1.600000e+02, !dbg !3261
  %_0.i.i446.us.us.us.us.us.us.us.i = select i1 %_3.i.i439.us.us.us.us.us.us.us.inv.i, float %_0.i.i529.us.us.us.us.us.us.us.i, float -1.600000e+02, !dbg !3261
  %_55.i.i.us.us.us.us.us.us.us.i = load float, ptr %22, align 4, !dbg !3264, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i113.us.us.us.us.us.us.us.i = fcmp ule float %_55.i.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3265
  %_3.i99.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i446.us.us.us.us.us.us.us.i, %_0.i301.us.us.us.us.us.us.us.i, !dbg !3267
  %_0.i186.us.us.us.us.us.us.us.i = fsub float %_0.i301.us.us.us.us.us.us.us.i, %_0.i301.us.us.us.us.us.us.us.3.i, !dbg !3269
  %_3.i97.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i446.us.us.us.us.us.us.us.i, %_0.i186.us.us.us.us.us.us.us.i, !dbg !3271
  %..i98.us.us.us.us.us.us.us.i = sext i1 %_3.i97.us.us.us.us.us.us.us.i to i32, !dbg !3273
  %_0.i397.us.us.us.us.us.us.us.i = sext i1 %_3.i99.us.us.us.us.us.us.us.i to i32, !dbg !3275
  %_0.i391.us.us.us.us.us.us.us.i = select i1 %_3.i113.us.us.us.us.us.us.us.i, i32 %_0.i397.us.us.us.us.us.us.us.i, i32 %..i98.us.us.us.us.us.us.us.i, !dbg !3275
  %_0.i403.us.us.us.us.us.us.us.i = xor i32 %..i98.us.us.us.us.us.us.us.i, -1, !dbg !3277
  %_67.i.i.us.us.us.us.us.us.us.i = load float, ptr %25, align 4, !dbg !3279, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i111.us.us.us.us.us.us.us.i = fcmp ogt float %_67.i.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3280
  %_0.i396.us.us.us.us.us.us.us.i = select i1 %_3.i111.us.us.us.us.us.us.us.i, i32 %_0.i403.us.us.us.us.us.us.us.i, i32 0, !dbg !3282
  %_0.i395.us.us.us.us.us.us.us.i = select i1 %_3.i113.us.us.us.us.us.us.us.i, i32 0, i32 %_0.i396.us.us.us.us.us.us.us.i, !dbg !3284
  %_0.i390.us.us.us.us.us.us.us.i = or i32 %_0.i395.us.us.us.us.us.us.us.i, %_0.i391.us.us.us.us.us.us.us.i, !dbg !3286
  %_5.i263.us.us.us.us.us.us.us.i = and i32 %_0.i390.us.us.us.us.us.us.us.i, 1065353216, !dbg !3288
  %_0.i267.us.us.us.us.us.us.us.i = bitcast i32 %_5.i263.us.us.us.us.us.us.us.i to float, !dbg !3290
  %_71.i.i586587.us.us.us.us.us.us.us.i = load float, ptr %26, align 4, !dbg !3292, !alias.scope !3194, !noalias !3195, !noundef !12
  %_0.i185.us.us.us.us.us.us.us.i = fadd float %_67.i.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3293
  %155 = trunc nsw i32 %_0.i395.us.us.us.us.us.us.us.i to i1, !dbg !3295
  %_4.i261.v.us.us.us.us.us.us.us.i = select i1 %155, float %_0.i185.us.us.us.us.us.us.us.i, float %_67.i.i.us.us.us.us.us.us.us.i, !dbg !3295
  %156 = trunc nsw i32 %_0.i391.us.us.us.us.us.us.us.i to i1, !dbg !3297
  %_0.i255.us.us.us.us.us.us.us.i = select i1 %156, float %_71.i.i586587.us.us.us.us.us.us.us.i, float %_4.i261.v.us.us.us.us.us.us.us.i, !dbg !3297
  store float %_0.i255.us.us.us.us.us.us.us.i, ptr %25, align 4, !dbg !3299, !alias.scope !3165, !noalias !3166
  store i32 %_5.i263.us.us.us.us.us.us.us.i, ptr %22, align 4, !dbg !3300, !alias.scope !3165, !noalias !3166
  %_0.i184.us.us.us.us.us.us.us.i = fadd float %_0.i301.us.us.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3301
  %_0.i183.us.us.us.us.us.us.us.i = fsub float %_0.i.i446.us.us.us.us.us.us.us.i, %_0.i301.us.us.us.us.us.us.us.i, !dbg !3303
  %_0.i168.us.us.us.us.us.us.us.i = fmul float %_0.i184.us.us.us.us.us.us.us.i, %_0.i183.us.us.us.us.us.us.us.i, !dbg !3305
  %157 = fneg float %_0.i301.us.us.us.us.us.us.us.2.i, !dbg !3307
  %_3.i.i431.inv.us.us.us.us.us.us.us.i = fcmp ogt float %_0.i168.us.us.us.us.us.us.us.i, %157, !dbg !3309
  %_4.i.i437.v.us.us.us.us.us.us.us.i = select i1 %_3.i.i431.inv.us.us.us.us.us.us.us.i, float %_0.i168.us.us.us.us.us.us.us.i, float %157, !dbg !3309
  %_3.i.i514.us.us.us.us.us.us.us.i = fcmp olt float %_4.i.i437.v.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3312
  %158 = fcmp ule float %_0.i267.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3315
  %159 = select i1 %158, i1 %_3.i.i514.us.us.us.us.us.us.us.i, i1 false, !dbg !3317
  %_0.i248.us.us.us.us.us.us.us.i = select i1 %159, float %_4.i.i437.v.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3317
  %_86.i.i.us.us.us.us.us.us.us.i = load float, ptr %27, align 4, !dbg !3318, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i107.us.us.us.us.us.us.us.i = fcmp ule float %_0.i248.us.us.us.us.us.us.us.i, %_86.i.i.us.us.us.us.us.us.us.i, !dbg !3319
  %_87.i.i589.us.us.us.us.us.us.us.i = load i32, ptr %_38.i, align 4, !dbg !3321, !alias.scope !3194, !noalias !3195, !noundef !12
  %_88.i.i590.us.us.us.us.us.us.us.i = load i32, ptr %28, align 4, !dbg !3322, !alias.scope !3194, !noalias !3195, !noundef !12
  %_4.i241.us.us.us.us.us.us.us.i = select i1 %_3.i107.us.us.us.us.us.us.us.i, i32 %_88.i.i590.us.us.us.us.us.us.us.i, i32 %_87.i.i589.us.us.us.us.us.us.us.i, !dbg !3323
  %_0.i242.us.us.us.us.us.us.us.i = bitcast i32 %_4.i241.us.us.us.us.us.us.us.i to float, !dbg !3325
  %_0.i182.us.us.us.us.us.us.us.i = fsub float %_0.i248.us.us.us.us.us.us.us.i, %_86.i.i.us.us.us.us.us.us.us.i, !dbg !3327
  %_4.i153.us.us.us.us.us.us.us.i = fmul float %_0.i182.us.us.us.us.us.us.us.i, %_0.i242.us.us.us.us.us.us.us.i, !dbg !3329
  %_0.i154.us.us.us.us.us.us.us.i = fadd float %_86.i.i.us.us.us.us.us.us.us.i, %_4.i153.us.us.us.us.us.us.us.i, !dbg !3329
  %160 = tail call noundef float @llvm.fabs.f32(float %_0.i154.us.us.us.us.us.us.us.i), !dbg !3331
  %161 = fcmp uge float %160, 0x3BC79CA100000000, !dbg !3334
  %_0.i226.us.us.us.us.us.us.us.i = select i1 %161, float %_0.i154.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3336
  store float %_0.i226.us.us.us.us.us.us.us.i, ptr %27, align 4, !dbg !3337, !alias.scope !3165, !noalias !3166
  %_0.i167.us.us.us.us.us.us.us.i = fmul float %_0.i226.us.us.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !3338
  %_3.i.i423.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i167.us.us.us.us.us.us.us.i, -1.260000e+02, !dbg !3341
  %_0.i.i430.us.us.us.us.us.us.us.i = select i1 %_3.i.i423.us.us.us.us.us.us.us.inv.i, float %_0.i167.us.us.us.us.us.us.us.i, float -1.260000e+02, !dbg !3341
  %_3.i.i506.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i430.us.us.us.us.us.us.us.i, 1.270000e+02, !dbg !3345
  %_0.i.i513.us.us.us.us.us.us.us.i = select i1 %_3.i.i506.us.us.us.us.us.us.us.inv.i, float %_0.i.i430.us.us.us.us.us.us.us.i, float 1.270000e+02, !dbg !3345
  %162 = tail call noundef float @llvm.floor.f32(float %_0.i.i513.us.us.us.us.us.us.us.i), !dbg !3348
  %_0.i181.us.us.us.us.us.us.us.i = fsub float %_0.i.i513.us.us.us.us.us.us.us.i, %162, !dbg !3352
  %_0.i165.us.us.us.us.us.us.us.i = fmul float %_0.i181.us.us.us.us.us.us.us.i, 0x3F5E974FA0000000, !dbg !3354
  %_0.i148.us.us.us.us.us.us.us.i = fadd float %_0.i165.us.us.us.us.us.us.us.i, 0x3F82778560000000, !dbg !3359
  %_0.i165.us.us.us.us.us.us.us.1.i = fmul float %_0.i181.us.us.us.us.us.us.us.i, %_0.i148.us.us.us.us.us.us.us.i, !dbg !3354
  %_0.i148.us.us.us.us.us.us.us.1.i = fadd float %_0.i165.us.us.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3359
  %_0.i165.us.us.us.us.us.us.us.2.i = fmul float %_0.i181.us.us.us.us.us.us.us.i, %_0.i148.us.us.us.us.us.us.us.1.i, !dbg !3354
  %_0.i148.us.us.us.us.us.us.us.2.i = fadd float %_0.i165.us.us.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3359
  %_0.i165.us.us.us.us.us.us.us.3.i = fmul float %_0.i181.us.us.us.us.us.us.us.i, %_0.i148.us.us.us.us.us.us.us.2.i, !dbg !3354
  %_0.i148.us.us.us.us.us.us.us.3.i = fadd float %_0.i165.us.us.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3359
  %_0.i163.us.us.us.us.us.us.us.i = fmul float %_0.i180.us.us.us.us.us.us.us.i, 0x3F5E974FA0000000, !dbg !3361
  %_0.i146.us.us.us.us.us.us.us.i = fadd float %_0.i163.us.us.us.us.us.us.us.i, 0x3F82778560000000, !dbg !3363
  %_0.i163.us.us.us.us.us.us.us.1.i = fmul float %_0.i180.us.us.us.us.us.us.us.i, %_0.i146.us.us.us.us.us.us.us.i, !dbg !3361
  %_0.i146.us.us.us.us.us.us.us.1.i = fadd float %_0.i163.us.us.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3363
  %_0.i163.us.us.us.us.us.us.us.2.i = fmul float %_0.i180.us.us.us.us.us.us.us.i, %_0.i146.us.us.us.us.us.us.us.1.i, !dbg !3361
  %_0.i146.us.us.us.us.us.us.us.2.i = fadd float %_0.i163.us.us.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3363
  %_0.i163.us.us.us.us.us.us.us.3.i = fmul float %_0.i180.us.us.us.us.us.us.us.i, %_0.i146.us.us.us.us.us.us.us.2.i, !dbg !3361
  %_0.i146.us.us.us.us.us.us.us.3.i = fadd float %_0.i163.us.us.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3363
  %_0.i162.us.us.us.us.us.us.us.i = fmul float %_0.i180.us.us.us.us.us.us.us.i, %_0.i146.us.us.us.us.us.us.us.3.i, !dbg !3365
  %_0.i145.us.us.us.us.us.us.us.i = fadd float %_0.i162.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !3367
  %biased.i.us.us.us.us.us.us.us.i = fadd float %147, 0x4160000FE0000000, !dbg !3369
  %_4.i83.us.us.us.us.us.us.us.i = bitcast float %biased.i.us.us.us.us.us.us.us.i to i32, !dbg !3373
  %_3.i84.us.us.us.us.us.us.us.i = shl i32 %_4.i83.us.us.us.us.us.us.us.i, 23, !dbg !3377
  %_0.i85.us.us.us.us.us.us.us.i = bitcast i32 %_3.i84.us.us.us.us.us.us.us.i to float, !dbg !3378
  %_0.i161.us.us.us.us.us.us.us.i = fmul float %_0.i145.us.us.us.us.us.us.us.i, %_0.i85.us.us.us.us.us.us.us.i, !dbg !3381
  %_3.i93.us.us.us.us.us.us.us.i = fcmp une float %_0.i230.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3383
  %_3.i121.us.us.us.us.us.us.us.i = fcmp ule float %_98.i123.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3385
  %_0.i392572.not.us.us.us.us.us.us.us.i = and i1 %_3.i121.us.us.us.us.us.us.us.i, %_3.i93.us.us.us.us.us.us.us.i, !dbg !3387
  %_0.i172.us.us.us.us.us.us.us.i = fmul float %_0.i205.us.us.us.us.us.us.us.i, %_0.i161.us.us.us.us.us.us.us.i, !dbg !3387
  %_4.i314.v.us.us.us.us.us.us.us.i = select i1 %_0.i392572.not.us.us.us.us.us.us.us.i, float %_0.i172.us.us.us.us.us.us.us.i, float %_0.i205.us.us.us.us.us.us.us.i, !dbg !3390
  %_0.i.us.us.us.us.us.us.us.i = fmul float %_0.i181.us.us.us.us.us.us.us.i, %_0.i148.us.us.us.us.us.us.us.3.i, !dbg !3392
  %_0.i147.us.us.us.us.us.us.us.i = fadd float %_0.i.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !3394
  %biased.i86.us.us.us.us.us.us.us.i = fadd float %162, 0x4160000FE0000000, !dbg !3396
  %_4.i87.us.us.us.us.us.us.us.i = bitcast float %biased.i86.us.us.us.us.us.us.us.i to i32, !dbg !3398
  %_3.i88.us.us.us.us.us.us.us.i = shl i32 %_4.i87.us.us.us.us.us.us.us.i, 23, !dbg !3400
  %_0.i89.us.us.us.us.us.us.us.i = bitcast i32 %_3.i88.us.us.us.us.us.us.us.i to float, !dbg !3401
  %_0.i164.us.us.us.us.us.us.us.i = fmul float %_0.i147.us.us.us.us.us.us.us.i, %_0.i89.us.us.us.us.us.us.us.i, !dbg !3403
  %_3.i90.us.us.us.us.us.us.us.i = fcmp une float %_0.i226.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3405
  %_98.i.i.us.us.us.us.us.us.us.i = load float, ptr %29, align 4, !dbg !3407, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i105.us.us.us.us.us.us.us.i = fcmp ule float %_98.i.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3408
  %_0.i389594.not.us.us.us.us.us.us.us.i = and i1 %_3.i105.us.us.us.us.us.us.us.i, %_3.i90.us.us.us.us.us.us.us.i, !dbg !3410
  %_0.i166.us.us.us.us.us.us.us.i = fmul float %_0.i203.us.us.us.us.us.us.us.i, %_0.i164.us.us.us.us.us.us.us.i, !dbg !3410
  %_4.i234.v.us.us.us.us.us.us.us.i = select i1 %_0.i389594.not.us.us.us.us.us.us.us.i, float %_0.i166.us.us.us.us.us.us.us.i, float %_0.i203.us.us.us.us.us.us.us.i, !dbg !3412
  store float %_4.i314.v.us.us.us.us.us.us.us.i, ptr %_97.i.us.us.us.us.us.us.us.i, align 4, !dbg !3414, !alias.scope !3417, !noalias !2719
  store float %_4.i234.v.us.us.us.us.us.us.us.i, ptr %_115.i.us.us.us.us.us.us.us.i, align 4, !dbg !3420, !alias.scope !3422, !noalias !2742
  %exitcond1511.not.i = icmp eq i64 %126, %..i, !dbg !3425
  br i1 %exitcond1511.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb30.i.us.us.us.us.us.us.us.i, !dbg !3428

bb8.i81.us.us.us.us.us.us.us.i:                   ; preds = %bb5.i.preheader.us.us.us.us.us.us.us.i
  %_34.i.us.us.us.us.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.us.us.us.us.i, !dbg !3429
  br i1 %_34.i.us.us.us.us.us.us.us.i, label %bb10.i82.us.us.us.us.us.us.us.i, label %panic5.i.i, !dbg !3429

bb10.i82.us.us.us.us.us.us.us.i:                  ; preds = %bb8.i81.us.us.us.us.us.us.us.i
  %163 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.us.us.us.us.i, !dbg !3436
  %left_own.i.us.us.us.us.us.us.us.i = load float, ptr %163, align 4, !dbg !3436, !alias.scope !2843, !noalias !2865, !noundef !12
  %164 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.us.us.us.us.i, !dbg !3429
  %right_own.i.us.us.us.us.us.us.us.i = load float, ptr %164, align 4, !dbg !3429, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i, !dbg !2870

bb5.i.preheader.us.us.us.us.us.us.us.i:           ; preds = %bb50.i.us.us.us.us.us.us.us.i
  br i1 %_31.i80.us.us.us.us.us.us.us.i, label %bb8.i81.us.us.us.us.us.us.us.i, label %panic4.i.i, !dbg !3436

bb14.i63.preheader.us.us.us.us.us.us.us.i:        ; preds = %bb50.i.us.us.us.us.us.us.us.i
  br i1 %_31.i80.us.us.us.us.us.us.us.i, label %bb17.i.us.us.us.us.us.us.us.i, label %panic15.i.i, !dbg !2880

bb23.i.preheader.us.us.us.us.us.us.us.i:          ; preds = %bb50.i.us.us.us.us.us.us.us.i
  br i1 %_31.i80.us.us.us.us.us.us.us.i, label %bb27.i60.us.us.us.us.us.us.us.i, label %panic28.i.i, !dbg !2859

bb50.i.us.us.us.us.us.us.us.i.unreachabledefault: ; preds = %bb50.i.us.us.us.us.us.us.us.i
  unreachable

default.unreachable:                              ; preds = %bb50.i.us.us.us.us.us.i, %bb50.i.us.us.us.i, %bb50.i.us.us.i, %bb53.i.us.i, %bb50.i.us.us.us.us.us.us.us.i73, %bb50.i.us.us.us.us.us.i237, %bb50.i.us.us.us.us.i391, %bb50.i.us.us.us.i547, %bb50.i.us.us.i701, %bb53.i.us.i864, %bb53.i.i
  unreachable

bb30.i.us.us.us.us.us.i:                          ; preds = %repeat_loop_next15.i.split.us.split.us.split.us.split.us.split.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i
  %iter.sroa.0.0.i714.us.us.us.us.us.i = phi i64 [ %165, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i ], [ 0, %repeat_loop_next15.i.split.us.split.us.split.us.split.us.split.us.i ]
  %165 = add nuw nsw i64 %iter.sroa.0.0.i714.us.us.us.us.us.i, 1, !dbg !2683
  %_28.i.us.us.us.us.us.i = trunc i64 %iter.sroa.0.0.i714.us.us.us.us.us.i to i32, !dbg !2694
  %now.i.us.us.us.us.us.i = add i32 %base.i.i, %_28.i.us.us.us.us.us.i, !dbg !2695
  %_31.i.us.us.us.us.us.i = and i32 %now.i.us.us.us.us.us.i, %_58.i, !dbg !2698
  %_30.i.us.us.us.us.us.i = zext i32 %_31.i.us.us.us.us.us.i to i64, !dbg !2699
  %exitcond1514.not.i = icmp eq i64 %iter.sroa.0.0.i714.us.us.us.us.us.i, %..i, !dbg !2671
  br i1 %exitcond1514.not.i, label %bb33.i.i, label %bb32.i.us.us.us.us.us.i, !dbg !2671, !prof !180

bb32.i.us.us.us.us.us.i:                          ; preds = %bb30.i.us.us.us.us.us.i
  %_97.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i714.us.us.us.us.us.i, !dbg !2700
  %_98.not.not.i.us.us.us.us.us.i = icmp ugt i64 %_64.1.i, %_30.i.us.us.us.us.us.i, !dbg !2704
  br i1 %_98.not.not.i.us.us.us.us.us.i, label %bb35.i.us.us.us.us.us.i, label %bb36.i.i, !dbg !2704, !prof !2709

bb35.i.us.us.us.us.us.i:                          ; preds = %bb32.i.us.us.us.us.us.i
  %_0.i213.us.us.us.us.us.i = load float, ptr %_97.i.us.us.us.us.us.i, align 4, !dbg !2710, !alias.scope !2716, !noalias !2719, !noundef !12
  %_107.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_30.i.us.us.us.us.us.i, !dbg !2720
  store float %_0.i213.us.us.us.us.us.i, ptr %_107.i.us.us.us.us.us.i, align 4, !dbg !2724, !alias.scope !2727, !noalias !2668
  %_115.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i714.us.us.us.us.us.i, !dbg !2730
  %_0.i211.us.us.us.us.us.i = load float, ptr %_115.i.us.us.us.us.us.i, align 4, !dbg !2737, !alias.scope !2739, !noalias !2742, !noundef !12
  %_123.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_30.i.us.us.us.us.us.i, !dbg !2743
  store float %_0.i211.us.us.us.us.us.i, ptr %_123.i.us.us.us.us.us.i, align 4, !dbg !2750, !alias.scope !2752, !noalias !2668
  %_131.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.us.us.us.us.i, !dbg !2755
  %_0.i209.us.us.us.us.us.i = load float, ptr %_131.i.us.us.us.us.us.i, align 4, !dbg !2762, !alias.scope !2764, !noalias !2668, !noundef !12
  %_139.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_30.i.us.us.us.us.us.i, !dbg !2767
  store float %_0.i209.us.us.us.us.us.i, ptr %_139.i.us.us.us.us.us.i, align 4, !dbg !2774, !alias.scope !2776, !noalias !2668
  %_148.not.not.i.us.us.us.us.us.i = icmp ugt i64 %_67.1.i, %_30.i.us.us.us.us.us.i, !dbg !3437
  br i1 %_148.not.not.i.us.us.us.us.us.i, label %bb48.i.us.us.us.us.us.i, label %bb49.i.i, !dbg !3437, !prof !2709

bb48.i.us.us.us.us.us.i:                          ; preds = %bb35.i.us.us.us.us.us.i
  %_147.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.us.us.us.us.i, !dbg !2779
  %_0.i207.us.us.us.us.us.i = load float, ptr %_147.i.us.us.us.us.us.i, align 4, !dbg !2786, !alias.scope !2788, !noalias !2668, !noundef !12
  %_155.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_30.i.us.us.us.us.us.i, !dbg !2791
  store float %_0.i207.us.us.us.us.us.i, ptr %_155.i.us.us.us.us.us.i, align 4, !dbg !2798, !alias.scope !2800, !noalias !2668
  %_53.i.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.i, %_59.i, !dbg !2803
  %_52.i.us.us.us.us.us.i = and i32 %_53.i.us.us.us.us.us.i, %_58.i, !dbg !2806
  %_51.i.us.us.us.us.us.i = zext i32 %_52.i.us.us.us.us.us.i to i64, !dbg !2807
  %_156.not.not.i.us.us.us.us.us.i = icmp ugt i64 %_64.1.i, %_51.i.us.us.us.us.us.i, !dbg !2808
  br i1 %_156.not.not.i.us.us.us.us.us.i, label %bb50.i.us.us.us.us.us.i, label %bb51.i.i, !dbg !2808, !prof !2709

bb50.i.us.us.us.us.us.i:                          ; preds = %bb48.i.us.us.us.us.us.i
  %_163.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_51.i.us.us.us.us.us.i, !dbg !2813
  %_0.i205.us.us.us.us.us.i = load float, ptr %_163.i.us.us.us.us.us.i, align 4, !dbg !2817, !alias.scope !2819, !noalias !2668, !noundef !12
  %_169.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_51.i.us.us.us.us.us.i, !dbg !2822
  %_0.i203.us.us.us.us.us.i = load float, ptr %_169.i.us.us.us.us.us.i, align 4, !dbg !2830, !alias.scope !2832, !noalias !2668, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2835), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2841), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2845), !dbg !2838
  %_18.i74.us.us.us.us.us.i = load i32, ptr %_31.i, align 4, !dbg !2847, !alias.scope !2850, !noalias !2851, !noundef !12
  %_17.i.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.i, %_18.i74.us.us.us.us.us.i, !dbg !2853
  %_16.i75.us.us.us.us.us.i = and i32 %_17.i.us.us.us.us.us.i, %_58.i, !dbg !2847
  %_15.i76.us.us.us.us.us.i = zext i32 %_16.i75.us.us.us.us.us.i to i64, !dbg !2847
  %_26.i.us.us.us.us.us.i = load i32, ptr %_33.i, align 4, !dbg !2847, !alias.scope !2856, !noalias !2857, !noundef !12
  %_25.i.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.i, %_26.i.us.us.us.us.us.i, !dbg !2853
  %_24.i.us.us.us.us.us.i = and i32 %_25.i.us.us.us.us.us.i, %_58.i, !dbg !2847
  %_23.i79.us.us.us.us.us.i = zext i32 %_24.i.us.us.us.us.us.i to i64, !dbg !2847
  %_31.i80.us.us.us.us.us.i = icmp samesign ugt i64 %_65.1.i, %_15.i76.us.us.us.us.us.i, !dbg !2847
  switch i8 %_0.sroa.0.0.i551.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.us.us.us.i
    i8 1, label %bb14.i63.preheader.us.us.us.us.us.i
    i8 2, label %bb23.i.preheader.us.us.us.us.us.i
  ], !dbg !2858

bb27.i60.us.us.us.us.us.i:                        ; preds = %bb23.i.preheader.us.us.us.us.us.i
  %166 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.us.us.i, !dbg !2859
  %_79.i.us.us.us.us.us.i = load float, ptr %166, align 4, !dbg !2859, !alias.scope !2843, !noalias !2865, !noundef !12
  %_85.i.us.us.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_15.i76.us.us.us.us.us.i, !dbg !2866
  br i1 %_85.i.us.us.us.us.us.i, label %bb29.i61.us.us.us.us.us.i, label %panic30.i.i, !dbg !2866

bb29.i61.us.us.us.us.us.i:                        ; preds = %bb27.i60.us.us.us.us.us.i
  %_87.i.us.us.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.us.us.i, !dbg !2868
  br i1 %_87.i.us.us.us.us.us.i, label %bb31.i.us.us.us.us.us.i, label %panic32.i.i, !dbg !2868

bb31.i.us.us.us.us.us.i:                          ; preds = %bb29.i61.us.us.us.us.us.i
  %167 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_15.i76.us.us.us.us.us.i, !dbg !2866
  %_83.i.us.us.us.us.us.i = load float, ptr %167, align 4, !dbg !2866, !alias.scope !2845, !noalias !2867, !noundef !12
  %168 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.us.us.i, !dbg !2868
  %_86.i.us.us.us.us.us.i = load float, ptr %168, align 4, !dbg !2868, !alias.scope !2845, !noalias !2867, !noundef !12
  %169 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_23.i79.us.us.us.us.us.i, !dbg !2869
  %_88.i.us.us.us.us.us.i = load float, ptr %169, align 4, !dbg !2869, !alias.scope !2843, !noalias !2865, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i, !dbg !2870

bb17.i.us.us.us.us.us.i:                          ; preds = %bb14.i63.preheader.us.us.us.us.us.i
  %_59.i.us.us.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.us.us.i, !dbg !2873
  br i1 %_59.i.us.us.us.us.us.i, label %bb19.i.us.us.us.us.us.i, label %panic17.i.i, !dbg !2873

bb19.i.us.us.us.us.us.i:                          ; preds = %bb17.i.us.us.us.us.us.i
  %170 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.us.us.i, !dbg !2880
  %left_own16.i.us.us.us.us.us.i = load float, ptr %170, align 4, !dbg !2880, !alias.scope !2843, !noalias !2865, !noundef !12
  %171 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.us.us.i, !dbg !2873
  %right_own18.i.us.us.us.us.us.i = load float, ptr %171, align 4, !dbg !2873, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i, !dbg !2870

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i: ; preds = %bb10.i82.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.i, %bb31.i.us.us.us.us.us.i
  %taps.i.sroa.1011534.5.i = phi float [ %right_own.i.us.us.us.us.us.i, %bb10.i82.us.us.us.us.us.i ], [ %left_own16.i.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.i ], [ %_88.i.us.us.us.us.us.i, %bb31.i.us.us.us.us.us.i ], !dbg !2847
  %taps.i.sroa.681533.5.i = phi float [ %right_own.i.us.us.us.us.us.i, %bb10.i82.us.us.us.us.us.i ], [ %right_own18.i.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.i ], [ %_86.i.us.us.us.us.us.i, %bb31.i.us.us.us.us.us.i ], !dbg !2847
  %taps.i.sroa.351532.5.i = phi float [ %left_own.i.us.us.us.us.us.i, %bb10.i82.us.us.us.us.us.i ], [ %right_own18.i.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.i ], [ %_83.i.us.us.us.us.us.i, %bb31.i.us.us.us.us.us.i ], !dbg !2847
  %taps.i.sroa.0.5.i = phi float [ %left_own.i.us.us.us.us.us.i, %bb10.i82.us.us.us.us.us.i ], [ %left_own16.i.us.us.us.us.us.i, %bb19.i.us.us.us.us.us.i ], [ %_79.i.us.us.us.us.us.i, %bb31.i.us.us.us.us.us.i ], !dbg !2847
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2881), !dbg !2884
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2885), !dbg !2884
  %_12.i36.i.us.us.us.us.us.i = load float, ptr %102, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.us.us.i = fcmp ule float %_12.i36.i.us.us.us.us.us.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.us.us.i = fcmp une float %_12.i36.i.us.us.us.us.us.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.us.us.i = load float, ptr %_10.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.us.us.i = load float, ptr %103, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.us.us.i = fadd float %_16.i39.i.us.us.us.us.us.i, %_17.i40.i.us.us.us.us.us.i, !dbg !2906
  %_20.i42.i552555.us.us.us.us.us.i = load float, ptr %104, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %172 = select i1 %_3.i95.us.us.us.us.us.i, float %_0.i152.us.us.us.us.us.i, float %_20.i42.i552555.us.us.us.us.us.i, !dbg !2911
  %_0.i381.us.us.us.us.us.i = select i1 %_3.i135.us.us.us.us.us.i, float %_16.i39.i.us.us.us.us.us.i, float %172, !dbg !2914
  store float %_0.i381.us.us.us.us.us.i, ptr %_10.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.us.us.i = select i1 %_3.i95.us.us.us.us.us.i, float %_17.i40.i.us.us.us.us.us.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.us.us.i, ptr %103, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.us.us.i = fadd float %_12.i36.i.us.us.us.us.us.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.us.us.i = select i1 %_3.i135.us.us.us.us.us.i, float %_12.i36.i.us.us.us.us.us.i, float %_0.i193.us.us.us.us.us.i, !dbg !2923
  store float %_4.i367.v.us.us.us.us.us.i, ptr %102, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.us.us.1.i = load float, ptr %105, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.us.us.1.i = fcmp ule float %_12.i36.i.us.us.us.us.us.1.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.us.us.1.i = fcmp une float %_12.i36.i.us.us.us.us.us.1.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.1.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.us.us.1.i = load float, ptr %106, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.us.us.1.i = fadd float %_16.i39.i.us.us.us.us.us.1.i, %_17.i40.i.us.us.us.us.us.1.i, !dbg !2906
  %_20.i42.i552555.us.us.us.us.us.1.i = load float, ptr %107, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %173 = select i1 %_3.i95.us.us.us.us.us.1.i, float %_0.i152.us.us.us.us.us.1.i, float %_20.i42.i552555.us.us.us.us.us.1.i, !dbg !2911
  %_0.i381.us.us.us.us.us.1.i = select i1 %_3.i135.us.us.us.us.us.1.i, float %_16.i39.i.us.us.us.us.us.1.i, float %173, !dbg !2914
  store float %_0.i381.us.us.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.1.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.us.us.1.i = select i1 %_3.i95.us.us.us.us.us.1.i, float %_17.i40.i.us.us.us.us.us.1.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.us.us.1.i, ptr %106, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.us.us.1.i = fadd float %_12.i36.i.us.us.us.us.us.1.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.us.us.1.i = select i1 %_3.i135.us.us.us.us.us.1.i, float %_12.i36.i.us.us.us.us.us.1.i, float %_0.i193.us.us.us.us.us.1.i, !dbg !2923
  store float %_4.i367.v.us.us.us.us.us.1.i, ptr %105, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.us.us.2.i = load float, ptr %108, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.us.us.2.i = fcmp ule float %_12.i36.i.us.us.us.us.us.2.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.us.us.2.i = fcmp une float %_12.i36.i.us.us.us.us.us.2.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.2.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.us.us.2.i = load float, ptr %109, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.us.us.2.i = fadd float %_16.i39.i.us.us.us.us.us.2.i, %_17.i40.i.us.us.us.us.us.2.i, !dbg !2906
  %_20.i42.i552555.us.us.us.us.us.2.i = load float, ptr %110, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %174 = select i1 %_3.i95.us.us.us.us.us.2.i, float %_0.i152.us.us.us.us.us.2.i, float %_20.i42.i552555.us.us.us.us.us.2.i, !dbg !2911
  %_0.i381.us.us.us.us.us.2.i = select i1 %_3.i135.us.us.us.us.us.2.i, float %_16.i39.i.us.us.us.us.us.2.i, float %174, !dbg !2914
  store float %_0.i381.us.us.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.2.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.us.us.2.i = select i1 %_3.i95.us.us.us.us.us.2.i, float %_17.i40.i.us.us.us.us.us.2.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.us.us.2.i, ptr %109, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.us.us.2.i = fadd float %_12.i36.i.us.us.us.us.us.2.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.us.us.2.i = select i1 %_3.i135.us.us.us.us.us.2.i, float %_12.i36.i.us.us.us.us.us.2.i, float %_0.i193.us.us.us.us.us.2.i, !dbg !2923
  store float %_4.i367.v.us.us.us.us.us.2.i, ptr %108, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.us.us.3.i = load float, ptr %111, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.us.us.3.i = fcmp ule float %_12.i36.i.us.us.us.us.us.3.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.us.us.3.i = fcmp une float %_12.i36.i.us.us.us.us.us.3.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.3.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.us.us.3.i = load float, ptr %112, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.us.us.3.i = fadd float %_16.i39.i.us.us.us.us.us.3.i, %_17.i40.i.us.us.us.us.us.3.i, !dbg !2906
  %_20.i42.i552555.us.us.us.us.us.3.i = load float, ptr %113, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %175 = select i1 %_3.i95.us.us.us.us.us.3.i, float %_0.i152.us.us.us.us.us.3.i, float %_20.i42.i552555.us.us.us.us.us.3.i, !dbg !2911
  %_0.i381.us.us.us.us.us.3.i = select i1 %_3.i135.us.us.us.us.us.3.i, float %_16.i39.i.us.us.us.us.us.3.i, float %175, !dbg !2914
  store float %_0.i381.us.us.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.us.us.3.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.us.us.3.i = select i1 %_3.i95.us.us.us.us.us.3.i, float %_17.i40.i.us.us.us.us.us.3.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.us.us.3.i, ptr %112, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.us.us.3.i = fadd float %_12.i36.i.us.us.us.us.us.3.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.us.us.3.i = select i1 %_3.i135.us.us.us.us.us.3.i, float %_12.i36.i.us.us.us.us.us.3.i, float %_0.i193.us.us.us.us.us.3.i, !dbg !2923
  store float %_4.i367.v.us.us.us.us.us.3.i, ptr %111, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %176 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.5.i), !dbg !2926
  %177 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351532.5.i), !dbg !2933
  %_37.i57.i.us.us.us.us.us.i = load float, ptr %15, align 4, !dbg !2936, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i133.us.us.us.us.us.i = fcmp ule float %_37.i57.i.us.us.us.us.us.i, 0.000000e+00, !dbg !2941
  %_3.i.i489.us.us.us.us.us.i = fcmp ule float %176, %177, !dbg !2943
  %_6.i.i491.us.us.us.us.us.i = bitcast float %176 to i32, !dbg !2949
  %_8.i.i493.us.us.us.us.us.i = bitcast float %177 to i32, !dbg !2953
  %_4.i.i496.us.us.us.us.us.i = select i1 %_3.i.i489.us.us.us.us.us.i, i32 %_8.i.i493.us.us.us.us.us.i, i32 %_6.i.i491.us.us.us.us.us.i, !dbg !2955
  %_4.i360.us.us.us.us.us.i = select i1 %_3.i133.us.us.us.us.us.i, i32 %_6.i.i491.us.us.us.us.us.i, i32 %_4.i.i496.us.us.us.us.us.i, !dbg !2956
  %_41.i61.i.us.us.us.us.us.i = load float, ptr %16, align 4, !dbg !2958, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i131.us.us.us.us.us.i = fcmp ule float %_41.i61.i.us.us.us.us.us.i, 0.000000e+00, !dbg !2959
  %_0.i177.us.us.us.us.us.i = fmul float %176, 5.000000e-01, !dbg !2961
  %_0.i176.us.us.us.us.us.i = fmul float %177, 5.000000e-01, !dbg !2964
  %_0.i151.us.us.us.us.us.i = fadd float %_0.i176.us.us.us.us.us.i, %_0.i177.us.us.us.us.us.i, !dbg !2966
  %_6.i348.us.us.us.us.us.i = bitcast float %_0.i151.us.us.us.us.us.i to i32, !dbg !2968
  %_4.i353.us.us.us.us.us.i = select i1 %_3.i131.us.us.us.us.us.i, i32 %_4.i360.us.us.us.us.us.i, i32 %_6.i348.us.us.us.us.us.i, !dbg !2971
  %_0.i354.us.us.us.us.us.i = bitcast i32 %_4.i353.us.us.us.us.us.i to float, !dbg !2972
  %_3.i.i481.us.us.us.us.us.i = fcmp ule float %_0.i354.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !2975
  %_4.i.i487.us.us.us.us.us.i = select i1 %_3.i.i481.us.us.us.us.us.i, i32 841731191, i32 %_4.i353.us.us.us.us.us.i, !dbg !2978
  %_0.i.i488.us.us.us.us.us.i = bitcast i32 %_4.i.i487.us.us.us.us.us.i to float, !dbg !2980
  %_3.i.i.us.us.us.us.us.i = fcmp ule float %_0.i.i488.us.us.us.us.us.i, 0x3810000000000000, !dbg !2982
  %_4.i.i.us.us.us.us.us.i = select i1 %_3.i.i.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i487.us.us.us.us.us.i, !dbg !2992
  %_5.i214.us.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.us.i, 8388607, !dbg !2994
  %_4.i215.us.us.us.us.us.i = or disjoint i32 %_5.i214.us.us.us.us.us.i, 1065353216, !dbg !2994
  %significand.i.us.us.us.us.us.i = bitcast i32 %_4.i215.us.us.us.us.us.i to float, !dbg !2999
  %_0.i178.us.us.us.us.us.i = fadd float %significand.i.us.us.us.us.us.i, -1.000000e+00, !dbg !3002
  %_0.i158.us.us.us.us.us.i = fmul float %_0.i178.us.us.us.us.us.i, 0x3F9B17A960000000, !dbg !3005
  %178 = fsub float 0x3FBF9A8440000000, %_0.i158.us.us.us.us.us.i, !dbg !3010
  %_0.i158.us.us.us.us.us.1.i = fmul float %_0.i178.us.us.us.us.us.i, %178, !dbg !3005
  %_0.i142.us.us.us.us.us.1.i = fadd float %_0.i158.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3010
  %_0.i158.us.us.us.us.us.2.i = fmul float %_0.i178.us.us.us.us.us.i, %_0.i142.us.us.us.us.us.1.i, !dbg !3005
  %_0.i142.us.us.us.us.us.2.i = fadd float %_0.i158.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3010
  %_0.i158.us.us.us.us.us.3.i = fmul float %_0.i178.us.us.us.us.us.i, %_0.i142.us.us.us.us.us.2.i, !dbg !3005
  %_0.i142.us.us.us.us.us.3.i = fadd float %_0.i158.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3010
  %_0.i158.us.us.us.us.us.4.i = fmul float %_0.i178.us.us.us.us.us.i, %_0.i142.us.us.us.us.us.3.i, !dbg !3005
  %_0.i142.us.us.us.us.us.4.i = fadd float %_0.i158.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3010
  %_9.i.us.us.us.us.us.i = lshr i32 %_4.i.i.us.us.us.us.us.i, 23, !dbg !3012
  %_8.i216.us.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.us.i, 1258291200, !dbg !3012
  %_7.i.us.us.us.us.us.i = bitcast i32 %_8.i216.us.us.us.us.us.i to float, !dbg !3014
  %exponent.i.us.us.us.us.us.i = fadd float %_7.i.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !3016
  %_0.i157.us.us.us.us.us.i = fmul float %_0.i178.us.us.us.us.us.i, %_0.i142.us.us.us.us.us.4.i, !dbg !3017
  %_0.i141.us.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.us.i, %_0.i157.us.us.us.us.us.i, !dbg !3019
  %_0.i175.us.us.us.us.us.i = fmul float %_0.i141.us.us.us.us.us.i, 0x4018151820000000, !dbg !3021
  %_3.i.i538.us.us.us.us.us.inv.i = fcmp olt float %_0.i175.us.us.us.us.us.i, 2.400000e+01, !dbg !3023
  %_0.i.i545.us.us.us.us.us.i = select i1 %_3.i.i538.us.us.us.us.us.inv.i, float %_0.i175.us.us.us.us.us.i, float 2.400000e+01, !dbg !3023
  %_3.i.i473.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i545.us.us.us.us.us.i, -1.600000e+02, !dbg !3027
  %_0.i.i480.us.us.us.us.us.i = select i1 %_3.i.i473.us.us.us.us.us.inv.i, float %_0.i.i545.us.us.us.us.us.i, float -1.600000e+02, !dbg !3027
  %_55.i78.i.us.us.us.us.us.i = load float, ptr %14, align 4, !dbg !3030, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i129.us.us.us.us.us.i = fcmp ule float %_55.i78.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3032
  %_3.i103.us.us.us.us.us.i = fcmp oge float %_0.i.i480.us.us.us.us.us.i, %_0.i381.us.us.us.us.us.i, !dbg !3034
  %_0.i192.us.us.us.us.us.i = fsub float %_0.i381.us.us.us.us.us.i, %_0.i381.us.us.us.us.us.3.i, !dbg !3038
  %_3.i101.us.us.us.us.us.i = fcmp oge float %_0.i.i480.us.us.us.us.us.i, %_0.i192.us.us.us.us.us.i, !dbg !3041
  %..i102.us.us.us.us.us.i = sext i1 %_3.i101.us.us.us.us.us.i to i32, !dbg !3043
  %_0.i401.us.us.us.us.us.i = sext i1 %_3.i103.us.us.us.us.us.i to i32, !dbg !3046
  %_0.i394.us.us.us.us.us.i = select i1 %_3.i129.us.us.us.us.us.i, i32 %_0.i401.us.us.us.us.us.i, i32 %..i102.us.us.us.us.us.i, !dbg !3046
  %_0.i405.us.us.us.us.us.i = xor i32 %..i102.us.us.us.us.us.i, -1, !dbg !3052
  %_67.i88.i.us.us.us.us.us.i = load float, ptr %17, align 4, !dbg !3056, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i127.us.us.us.us.us.i = fcmp ogt float %_67.i88.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3057
  %_0.i400.us.us.us.us.us.i = select i1 %_3.i127.us.us.us.us.us.i, i32 %_0.i405.us.us.us.us.us.i, i32 0, !dbg !3059
  %_0.i399.us.us.us.us.us.i = select i1 %_3.i129.us.us.us.us.us.i, i32 0, i32 %_0.i400.us.us.us.us.us.i, !dbg !3062
  %_0.i393.us.us.us.us.us.i = or i32 %_0.i399.us.us.us.us.us.i, %_0.i394.us.us.us.us.us.i, !dbg !3064
  %_5.i343.us.us.us.us.us.i = and i32 %_0.i393.us.us.us.us.us.i, 1065353216, !dbg !3067
  %_0.i347.us.us.us.us.us.i = bitcast i32 %_5.i343.us.us.us.us.us.i to float, !dbg !3069
  %_71.i94.i564565.us.us.us.us.us.i = load float, ptr %18, align 4, !dbg !3071, !alias.scope !2939, !noalias !2940, !noundef !12
  %_0.i191.us.us.us.us.us.i = fadd float %_67.i88.i.us.us.us.us.us.i, -1.000000e+00, !dbg !3073
  %179 = trunc nsw i32 %_0.i399.us.us.us.us.us.i to i1, !dbg !3075
  %_4.i341.v.us.us.us.us.us.i = select i1 %179, float %_0.i191.us.us.us.us.us.i, float %_67.i88.i.us.us.us.us.us.i, !dbg !3075
  %180 = trunc nsw i32 %_0.i394.us.us.us.us.us.i to i1, !dbg !3077
  %_0.i335.us.us.us.us.us.i = select i1 %180, float %_71.i94.i564565.us.us.us.us.us.i, float %_4.i341.v.us.us.us.us.us.i, !dbg !3077
  store float %_0.i335.us.us.us.us.us.i, ptr %17, align 4, !dbg !3079, !alias.scope !2894, !noalias !2895
  store i32 %_5.i343.us.us.us.us.us.i, ptr %14, align 4, !dbg !3080, !alias.scope !2894, !noalias !2895
  %_0.i190.us.us.us.us.us.i = fadd float %_0.i381.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3081
  %_0.i189.us.us.us.us.us.i = fsub float %_0.i.i480.us.us.us.us.us.i, %_0.i381.us.us.us.us.us.i, !dbg !3083
  %_0.i174.us.us.us.us.us.i = fmul float %_0.i190.us.us.us.us.us.i, %_0.i189.us.us.us.us.us.i, !dbg !3085
  %181 = fneg float %_0.i381.us.us.us.us.us.2.i, !dbg !3087
  %_3.i.i464.inv.us.us.us.us.us.i = fcmp ogt float %_0.i174.us.us.us.us.us.i, %181, !dbg !3090
  %_4.i.i471.v.us.us.us.us.us.i = select i1 %_3.i.i464.inv.us.us.us.us.us.i, float %_0.i174.us.us.us.us.us.i, float %181, !dbg !3090
  %_3.i.i530.us.us.us.us.us.i = fcmp olt float %_4.i.i471.v.us.us.us.us.us.i, 0.000000e+00, !dbg !3093
  %182 = fcmp ule float %_0.i347.us.us.us.us.us.i, 0.000000e+00, !dbg !3097
  %183 = select i1 %182, i1 %_3.i.i530.us.us.us.us.us.i, i1 false, !dbg !3100
  %_0.i328.us.us.us.us.us.i = select i1 %183, float %_4.i.i471.v.us.us.us.us.us.i, float 0.000000e+00, !dbg !3100
  %_86.i107.i.us.us.us.us.us.i = load float, ptr %19, align 4, !dbg !3101, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i123.us.us.us.us.us.i = fcmp ule float %_0.i328.us.us.us.us.us.i, %_86.i107.i.us.us.us.us.us.i, !dbg !3103
  %_87.i109.i567.us.us.us.us.us.i = load i32, ptr %4, align 4, !dbg !3105, !alias.scope !2939, !noalias !2940, !noundef !12
  %_88.i110.i568.us.us.us.us.us.i = load i32, ptr %20, align 4, !dbg !3106, !alias.scope !2939, !noalias !2940, !noundef !12
  %_4.i321.us.us.us.us.us.i = select i1 %_3.i123.us.us.us.us.us.i, i32 %_88.i110.i568.us.us.us.us.us.i, i32 %_87.i109.i567.us.us.us.us.us.i, !dbg !3107
  %_0.i322.us.us.us.us.us.i = bitcast i32 %_4.i321.us.us.us.us.us.i to float, !dbg !3109
  %_0.i188.us.us.us.us.us.i = fsub float %_0.i328.us.us.us.us.us.i, %_86.i107.i.us.us.us.us.us.i, !dbg !3111
  %_4.i155.us.us.us.us.us.i = fmul float %_0.i188.us.us.us.us.us.i, %_0.i322.us.us.us.us.us.i, !dbg !3114
  %_0.i156.us.us.us.us.us.i = fadd float %_86.i107.i.us.us.us.us.us.i, %_4.i155.us.us.us.us.us.i, !dbg !3114
  %184 = tail call noundef float @llvm.fabs.f32(float %_0.i156.us.us.us.us.us.i), !dbg !3117
  %185 = fcmp uge float %184, 0x3BC79CA100000000, !dbg !3121
  %_0.i230.us.us.us.us.us.i = select i1 %185, float %_0.i156.us.us.us.us.us.i, float 0.000000e+00, !dbg !3124
  store float %_0.i230.us.us.us.us.us.i, ptr %19, align 4, !dbg !3125, !alias.scope !2894, !noalias !2895
  %_0.i173.us.us.us.us.us.i = fmul float %_0.i230.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !3127
  %_3.i.i415.us.us.us.us.us.inv.i = fcmp ogt float %_0.i173.us.us.us.us.us.i, -1.260000e+02, !dbg !3131
  %_0.i.i422.us.us.us.us.us.i = select i1 %_3.i.i415.us.us.us.us.us.inv.i, float %_0.i173.us.us.us.us.us.i, float -1.260000e+02, !dbg !3131
  %_3.i.i498.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i422.us.us.us.us.us.i, 1.270000e+02, !dbg !3136
  %_0.i.i505.us.us.us.us.us.i = select i1 %_3.i.i498.us.us.us.us.us.inv.i, float %_0.i.i422.us.us.us.us.us.i, float 1.270000e+02, !dbg !3136
  %186 = tail call noundef float @llvm.floor.f32(float %_0.i.i505.us.us.us.us.us.i), !dbg !3139
  %_0.i180.us.us.us.us.us.i = fsub float %_0.i.i505.us.us.us.us.us.i, %186, !dbg !3151
  %_98.i123.i.us.us.us.us.us.i = load float, ptr %21, align 4, !dbg !3154, !alias.scope !2939, !noalias !2940, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3156), !dbg !3159
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3161), !dbg !3159
  %_12.i.i.us.us.us.us.us.i = load float, ptr %114, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.us.us.i = fcmp ule float %_12.i.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.us.us.i = fcmp une float %_12.i.i.us.us.us.us.us.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.us.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.us.us.i = load float, ptr %115, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.us.us.i = fadd float %_16.i.i.us.us.us.us.us.i, %_17.i.i.us.us.us.us.us.i, !dbg !3173
  %_20.i.i574577.us.us.us.us.us.i = load float, ptr %116, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %187 = select i1 %_3.i91.us.us.us.us.us.i, float %_0.i150.us.us.us.us.us.i, float %_20.i.i574577.us.us.us.us.us.i, !dbg !3176
  %_0.i301.us.us.us.us.us.i = select i1 %_3.i119.us.us.us.us.us.i, float %_16.i.i.us.us.us.us.us.i, float %187, !dbg !3178
  store float %_0.i301.us.us.us.us.us.i, ptr %data.i.i.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.us.us.i = select i1 %_3.i91.us.us.us.us.us.i, float %_17.i.i.us.us.us.us.us.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.us.us.i, ptr %115, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.us.us.i = fadd float %_12.i.i.us.us.us.us.us.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.us.us.i = select i1 %_3.i119.us.us.us.us.us.i, float %_12.i.i.us.us.us.us.us.i, float %_0.i187.us.us.us.us.us.i, !dbg !3186
  store float %_4.i287.v.us.us.us.us.us.i, ptr %114, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.us.us.1.i = load float, ptr %117, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.us.us.1.i = fcmp ule float %_12.i.i.us.us.us.us.us.1.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.us.us.1.i = fcmp une float %_12.i.i.us.us.us.us.us.1.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.1.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.us.us.1.i = load float, ptr %118, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.us.us.1.i = fadd float %_16.i.i.us.us.us.us.us.1.i, %_17.i.i.us.us.us.us.us.1.i, !dbg !3173
  %_20.i.i574577.us.us.us.us.us.1.i = load float, ptr %119, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %188 = select i1 %_3.i91.us.us.us.us.us.1.i, float %_0.i150.us.us.us.us.us.1.i, float %_20.i.i574577.us.us.us.us.us.1.i, !dbg !3176
  %_0.i301.us.us.us.us.us.1.i = select i1 %_3.i119.us.us.us.us.us.1.i, float %_16.i.i.us.us.us.us.us.1.i, float %188, !dbg !3178
  store float %_0.i301.us.us.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.1.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.us.us.1.i = select i1 %_3.i91.us.us.us.us.us.1.i, float %_17.i.i.us.us.us.us.us.1.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.us.us.1.i, ptr %118, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.us.us.1.i = fadd float %_12.i.i.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.us.us.1.i = select i1 %_3.i119.us.us.us.us.us.1.i, float %_12.i.i.us.us.us.us.us.1.i, float %_0.i187.us.us.us.us.us.1.i, !dbg !3186
  store float %_4.i287.v.us.us.us.us.us.1.i, ptr %117, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.us.us.2.i = load float, ptr %120, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.us.us.2.i = fcmp ule float %_12.i.i.us.us.us.us.us.2.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.us.us.2.i = fcmp une float %_12.i.i.us.us.us.us.us.2.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.2.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.us.us.2.i = load float, ptr %121, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.us.us.2.i = fadd float %_16.i.i.us.us.us.us.us.2.i, %_17.i.i.us.us.us.us.us.2.i, !dbg !3173
  %_20.i.i574577.us.us.us.us.us.2.i = load float, ptr %122, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %189 = select i1 %_3.i91.us.us.us.us.us.2.i, float %_0.i150.us.us.us.us.us.2.i, float %_20.i.i574577.us.us.us.us.us.2.i, !dbg !3176
  %_0.i301.us.us.us.us.us.2.i = select i1 %_3.i119.us.us.us.us.us.2.i, float %_16.i.i.us.us.us.us.us.2.i, float %189, !dbg !3178
  store float %_0.i301.us.us.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.2.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.us.us.2.i = select i1 %_3.i91.us.us.us.us.us.2.i, float %_17.i.i.us.us.us.us.us.2.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.us.us.2.i, ptr %121, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.us.us.2.i = fadd float %_12.i.i.us.us.us.us.us.2.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.us.us.2.i = select i1 %_3.i119.us.us.us.us.us.2.i, float %_12.i.i.us.us.us.us.us.2.i, float %_0.i187.us.us.us.us.us.2.i, !dbg !3186
  store float %_4.i287.v.us.us.us.us.us.2.i, ptr %120, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.us.us.3.i = load float, ptr %123, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.us.us.3.i = fcmp ule float %_12.i.i.us.us.us.us.us.3.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.us.us.3.i = fcmp une float %_12.i.i.us.us.us.us.us.3.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.3.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.us.us.3.i = load float, ptr %124, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.us.us.3.i = fadd float %_16.i.i.us.us.us.us.us.3.i, %_17.i.i.us.us.us.us.us.3.i, !dbg !3173
  %_20.i.i574577.us.us.us.us.us.3.i = load float, ptr %125, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %190 = select i1 %_3.i91.us.us.us.us.us.3.i, float %_0.i150.us.us.us.us.us.3.i, float %_20.i.i574577.us.us.us.us.us.3.i, !dbg !3176
  %_0.i301.us.us.us.us.us.3.i = select i1 %_3.i119.us.us.us.us.us.3.i, float %_16.i.i.us.us.us.us.us.3.i, float %190, !dbg !3178
  store float %_0.i301.us.us.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.us.us.3.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.us.us.3.i = select i1 %_3.i91.us.us.us.us.us.3.i, float %_17.i.i.us.us.us.us.us.3.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.us.us.3.i, ptr %124, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.us.us.3.i = fadd float %_12.i.i.us.us.us.us.us.3.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.us.us.3.i = select i1 %_3.i119.us.us.us.us.us.3.i, float %_12.i.i.us.us.us.us.us.3.i, float %_0.i187.us.us.us.us.us.3.i, !dbg !3186
  store float %_4.i287.v.us.us.us.us.us.3.i, ptr %123, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %191 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681533.5.i), !dbg !3189
  %192 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011534.5.i), !dbg !3191
  %_37.i.i.us.us.us.us.us.i = load float, ptr %23, align 4, !dbg !3193, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i117.us.us.us.us.us.i = fcmp ule float %_37.i.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3196
  %_3.i.i455.us.us.us.us.us.i = fcmp ule float %191, %192, !dbg !3198
  %_6.i.i457.us.us.us.us.us.i = bitcast float %191 to i32, !dbg !3201
  %_8.i.i459.us.us.us.us.us.i = bitcast float %192 to i32, !dbg !3204
  %_4.i.i462.us.us.us.us.us.i = select i1 %_3.i.i455.us.us.us.us.us.i, i32 %_8.i.i459.us.us.us.us.us.i, i32 %_6.i.i457.us.us.us.us.us.i, !dbg !3206
  %_4.i280.us.us.us.us.us.i = select i1 %_3.i117.us.us.us.us.us.i, i32 %_6.i.i457.us.us.us.us.us.i, i32 %_4.i.i462.us.us.us.us.us.i, !dbg !3207
  %_41.i.i.us.us.us.us.us.i = load float, ptr %24, align 4, !dbg !3209, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i115.us.us.us.us.us.i = fcmp ule float %_41.i.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3210
  %_0.i171.us.us.us.us.us.i = fmul float %191, 5.000000e-01, !dbg !3212
  %_0.i170.us.us.us.us.us.i = fmul float %192, 5.000000e-01, !dbg !3214
  %_0.i149.us.us.us.us.us.i = fadd float %_0.i170.us.us.us.us.us.i, %_0.i171.us.us.us.us.us.i, !dbg !3216
  %_6.i268.us.us.us.us.us.i = bitcast float %_0.i149.us.us.us.us.us.i to i32, !dbg !3218
  %_4.i273.us.us.us.us.us.i = select i1 %_3.i115.us.us.us.us.us.i, i32 %_4.i280.us.us.us.us.us.i, i32 %_6.i268.us.us.us.us.us.i, !dbg !3221
  %_0.i274.us.us.us.us.us.i = bitcast i32 %_4.i273.us.us.us.us.us.i to float, !dbg !3222
  %_3.i.i447.us.us.us.us.us.i = fcmp ule float %_0.i274.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !3224
  %_4.i.i453.us.us.us.us.us.i = select i1 %_3.i.i447.us.us.us.us.us.i, i32 841731191, i32 %_4.i273.us.us.us.us.us.i, !dbg !3227
  %_0.i.i454.us.us.us.us.us.i = bitcast i32 %_4.i.i453.us.us.us.us.us.i to float, !dbg !3229
  %_3.i.i407.us.us.us.us.us.i = fcmp ule float %_0.i.i454.us.us.us.us.us.i, 0x3810000000000000, !dbg !3231
  %_4.i.i413.us.us.us.us.us.i = select i1 %_3.i.i407.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i453.us.us.us.us.us.i, !dbg !3236
  %_5.i218.us.us.us.us.us.i = and i32 %_4.i.i413.us.us.us.us.us.i, 8388607, !dbg !3238
  %_4.i219.us.us.us.us.us.i = or disjoint i32 %_5.i218.us.us.us.us.us.i, 1065353216, !dbg !3238
  %significand.i220.us.us.us.us.us.i = bitcast i32 %_4.i219.us.us.us.us.us.i to float, !dbg !3240
  %_0.i179.us.us.us.us.us.i = fadd float %significand.i220.us.us.us.us.us.i, -1.000000e+00, !dbg !3242
  %_0.i160.us.us.us.us.us.i = fmul float %_0.i179.us.us.us.us.us.i, 0x3F9B17A960000000, !dbg !3244
  %193 = fsub float 0x3FBF9A8440000000, %_0.i160.us.us.us.us.us.i, !dbg !3246
  %_0.i160.us.us.us.us.us.1.i = fmul float %_0.i179.us.us.us.us.us.i, %193, !dbg !3244
  %_0.i144.us.us.us.us.us.1.i = fadd float %_0.i160.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3246
  %_0.i160.us.us.us.us.us.2.i = fmul float %_0.i179.us.us.us.us.us.i, %_0.i144.us.us.us.us.us.1.i, !dbg !3244
  %_0.i144.us.us.us.us.us.2.i = fadd float %_0.i160.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3246
  %_0.i160.us.us.us.us.us.3.i = fmul float %_0.i179.us.us.us.us.us.i, %_0.i144.us.us.us.us.us.2.i, !dbg !3244
  %_0.i144.us.us.us.us.us.3.i = fadd float %_0.i160.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3246
  %_0.i160.us.us.us.us.us.4.i = fmul float %_0.i179.us.us.us.us.us.i, %_0.i144.us.us.us.us.us.3.i, !dbg !3244
  %_0.i144.us.us.us.us.us.4.i = fadd float %_0.i160.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3246
  %_9.i221.us.us.us.us.us.i = lshr i32 %_4.i.i413.us.us.us.us.us.i, 23, !dbg !3248
  %_8.i222.us.us.us.us.us.i = or disjoint i32 %_9.i221.us.us.us.us.us.i, 1258291200, !dbg !3248
  %_7.i223.us.us.us.us.us.i = bitcast i32 %_8.i222.us.us.us.us.us.i to float, !dbg !3249
  %exponent.i224.us.us.us.us.us.i = fadd float %_7.i223.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !3251
  %_0.i159.us.us.us.us.us.i = fmul float %_0.i179.us.us.us.us.us.i, %_0.i144.us.us.us.us.us.4.i, !dbg !3252
  %_0.i143.us.us.us.us.us.i = fadd float %exponent.i224.us.us.us.us.us.i, %_0.i159.us.us.us.us.us.i, !dbg !3254
  %_0.i169.us.us.us.us.us.i = fmul float %_0.i143.us.us.us.us.us.i, 0x4018151820000000, !dbg !3256
  %_3.i.i522.us.us.us.us.us.inv.i = fcmp olt float %_0.i169.us.us.us.us.us.i, 2.400000e+01, !dbg !3258
  %_0.i.i529.us.us.us.us.us.i = select i1 %_3.i.i522.us.us.us.us.us.inv.i, float %_0.i169.us.us.us.us.us.i, float 2.400000e+01, !dbg !3258
  %_3.i.i439.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i529.us.us.us.us.us.i, -1.600000e+02, !dbg !3261
  %_0.i.i446.us.us.us.us.us.i = select i1 %_3.i.i439.us.us.us.us.us.inv.i, float %_0.i.i529.us.us.us.us.us.i, float -1.600000e+02, !dbg !3261
  %_55.i.i.us.us.us.us.us.i = load float, ptr %22, align 4, !dbg !3264, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i113.us.us.us.us.us.i = fcmp ule float %_55.i.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3265
  %_3.i99.us.us.us.us.us.i = fcmp oge float %_0.i.i446.us.us.us.us.us.i, %_0.i301.us.us.us.us.us.i, !dbg !3267
  %_0.i186.us.us.us.us.us.i = fsub float %_0.i301.us.us.us.us.us.i, %_0.i301.us.us.us.us.us.3.i, !dbg !3269
  %_3.i97.us.us.us.us.us.i = fcmp oge float %_0.i.i446.us.us.us.us.us.i, %_0.i186.us.us.us.us.us.i, !dbg !3271
  %..i98.us.us.us.us.us.i = sext i1 %_3.i97.us.us.us.us.us.i to i32, !dbg !3273
  %_0.i397.us.us.us.us.us.i = sext i1 %_3.i99.us.us.us.us.us.i to i32, !dbg !3275
  %_0.i391.us.us.us.us.us.i = select i1 %_3.i113.us.us.us.us.us.i, i32 %_0.i397.us.us.us.us.us.i, i32 %..i98.us.us.us.us.us.i, !dbg !3275
  %_0.i403.us.us.us.us.us.i = xor i32 %..i98.us.us.us.us.us.i, -1, !dbg !3277
  %_67.i.i.us.us.us.us.us.i = load float, ptr %25, align 4, !dbg !3279, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i111.us.us.us.us.us.i = fcmp ogt float %_67.i.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3280
  %_0.i396.us.us.us.us.us.i = select i1 %_3.i111.us.us.us.us.us.i, i32 %_0.i403.us.us.us.us.us.i, i32 0, !dbg !3282
  %_0.i395.us.us.us.us.us.i = select i1 %_3.i113.us.us.us.us.us.i, i32 0, i32 %_0.i396.us.us.us.us.us.i, !dbg !3284
  %_0.i390.us.us.us.us.us.i = or i32 %_0.i395.us.us.us.us.us.i, %_0.i391.us.us.us.us.us.i, !dbg !3286
  %_5.i263.us.us.us.us.us.i = and i32 %_0.i390.us.us.us.us.us.i, 1065353216, !dbg !3288
  %_0.i267.us.us.us.us.us.i = bitcast i32 %_5.i263.us.us.us.us.us.i to float, !dbg !3290
  %_71.i.i586587.us.us.us.us.us.i = load float, ptr %26, align 4, !dbg !3292, !alias.scope !3194, !noalias !3195, !noundef !12
  %_0.i185.us.us.us.us.us.i = fadd float %_67.i.i.us.us.us.us.us.i, -1.000000e+00, !dbg !3293
  %194 = trunc nsw i32 %_0.i395.us.us.us.us.us.i to i1, !dbg !3295
  %_4.i261.v.us.us.us.us.us.i = select i1 %194, float %_0.i185.us.us.us.us.us.i, float %_67.i.i.us.us.us.us.us.i, !dbg !3295
  %195 = trunc nsw i32 %_0.i391.us.us.us.us.us.i to i1, !dbg !3297
  %_0.i255.us.us.us.us.us.i = select i1 %195, float %_71.i.i586587.us.us.us.us.us.i, float %_4.i261.v.us.us.us.us.us.i, !dbg !3297
  store float %_0.i255.us.us.us.us.us.i, ptr %25, align 4, !dbg !3299, !alias.scope !3165, !noalias !3166
  store i32 %_5.i263.us.us.us.us.us.i, ptr %22, align 4, !dbg !3300, !alias.scope !3165, !noalias !3166
  %_0.i184.us.us.us.us.us.i = fadd float %_0.i301.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3301
  %_0.i183.us.us.us.us.us.i = fsub float %_0.i.i446.us.us.us.us.us.i, %_0.i301.us.us.us.us.us.i, !dbg !3303
  %_0.i168.us.us.us.us.us.i = fmul float %_0.i184.us.us.us.us.us.i, %_0.i183.us.us.us.us.us.i, !dbg !3305
  %196 = fneg float %_0.i301.us.us.us.us.us.2.i, !dbg !3307
  %_3.i.i431.inv.us.us.us.us.us.i = fcmp ogt float %_0.i168.us.us.us.us.us.i, %196, !dbg !3309
  %_4.i.i437.v.us.us.us.us.us.i = select i1 %_3.i.i431.inv.us.us.us.us.us.i, float %_0.i168.us.us.us.us.us.i, float %196, !dbg !3309
  %_3.i.i514.us.us.us.us.us.i = fcmp olt float %_4.i.i437.v.us.us.us.us.us.i, 0.000000e+00, !dbg !3312
  %197 = fcmp ule float %_0.i267.us.us.us.us.us.i, 0.000000e+00, !dbg !3315
  %198 = select i1 %197, i1 %_3.i.i514.us.us.us.us.us.i, i1 false, !dbg !3317
  %_0.i248.us.us.us.us.us.i = select i1 %198, float %_4.i.i437.v.us.us.us.us.us.i, float 0.000000e+00, !dbg !3317
  %_86.i.i.us.us.us.us.us.i = load float, ptr %27, align 4, !dbg !3318, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i107.us.us.us.us.us.i = fcmp ule float %_0.i248.us.us.us.us.us.i, %_86.i.i.us.us.us.us.us.i, !dbg !3319
  %_87.i.i589.us.us.us.us.us.i = load i32, ptr %_38.i, align 4, !dbg !3321, !alias.scope !3194, !noalias !3195, !noundef !12
  %_88.i.i590.us.us.us.us.us.i = load i32, ptr %28, align 4, !dbg !3322, !alias.scope !3194, !noalias !3195, !noundef !12
  %_4.i241.us.us.us.us.us.i = select i1 %_3.i107.us.us.us.us.us.i, i32 %_88.i.i590.us.us.us.us.us.i, i32 %_87.i.i589.us.us.us.us.us.i, !dbg !3323
  %_0.i242.us.us.us.us.us.i = bitcast i32 %_4.i241.us.us.us.us.us.i to float, !dbg !3325
  %_0.i182.us.us.us.us.us.i = fsub float %_0.i248.us.us.us.us.us.i, %_86.i.i.us.us.us.us.us.i, !dbg !3327
  %_4.i153.us.us.us.us.us.i = fmul float %_0.i182.us.us.us.us.us.i, %_0.i242.us.us.us.us.us.i, !dbg !3329
  %_0.i154.us.us.us.us.us.i = fadd float %_86.i.i.us.us.us.us.us.i, %_4.i153.us.us.us.us.us.i, !dbg !3329
  %199 = tail call noundef float @llvm.fabs.f32(float %_0.i154.us.us.us.us.us.i), !dbg !3331
  %200 = fcmp uge float %199, 0x3BC79CA100000000, !dbg !3334
  %_0.i226.us.us.us.us.us.i = select i1 %200, float %_0.i154.us.us.us.us.us.i, float 0.000000e+00, !dbg !3336
  store float %_0.i226.us.us.us.us.us.i, ptr %27, align 4, !dbg !3337, !alias.scope !3165, !noalias !3166
  %_0.i167.us.us.us.us.us.i = fmul float %_0.i226.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !3338
  %_3.i.i423.us.us.us.us.us.inv.i = fcmp ogt float %_0.i167.us.us.us.us.us.i, -1.260000e+02, !dbg !3341
  %_0.i.i430.us.us.us.us.us.i = select i1 %_3.i.i423.us.us.us.us.us.inv.i, float %_0.i167.us.us.us.us.us.i, float -1.260000e+02, !dbg !3341
  %_3.i.i506.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i430.us.us.us.us.us.i, 1.270000e+02, !dbg !3345
  %_0.i.i513.us.us.us.us.us.i = select i1 %_3.i.i506.us.us.us.us.us.inv.i, float %_0.i.i430.us.us.us.us.us.i, float 1.270000e+02, !dbg !3345
  %201 = tail call noundef float @llvm.floor.f32(float %_0.i.i513.us.us.us.us.us.i), !dbg !3348
  %_0.i181.us.us.us.us.us.i = fsub float %_0.i.i513.us.us.us.us.us.i, %201, !dbg !3352
  %_0.i165.us.us.us.us.us.i = fmul float %_0.i181.us.us.us.us.us.i, 0x3F5E974FA0000000, !dbg !3354
  %_0.i148.us.us.us.us.us.i = fadd float %_0.i165.us.us.us.us.us.i, 0x3F82778560000000, !dbg !3359
  %_0.i165.us.us.us.us.us.1.i = fmul float %_0.i181.us.us.us.us.us.i, %_0.i148.us.us.us.us.us.i, !dbg !3354
  %_0.i148.us.us.us.us.us.1.i = fadd float %_0.i165.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3359
  %_0.i165.us.us.us.us.us.2.i = fmul float %_0.i181.us.us.us.us.us.i, %_0.i148.us.us.us.us.us.1.i, !dbg !3354
  %_0.i148.us.us.us.us.us.2.i = fadd float %_0.i165.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3359
  %_0.i165.us.us.us.us.us.3.i = fmul float %_0.i181.us.us.us.us.us.i, %_0.i148.us.us.us.us.us.2.i, !dbg !3354
  %_0.i148.us.us.us.us.us.3.i = fadd float %_0.i165.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3359
  %_0.i163.us.us.us.us.us.i = fmul float %_0.i180.us.us.us.us.us.i, 0x3F5E974FA0000000, !dbg !3361
  %_0.i146.us.us.us.us.us.i = fadd float %_0.i163.us.us.us.us.us.i, 0x3F82778560000000, !dbg !3363
  %_0.i163.us.us.us.us.us.1.i = fmul float %_0.i180.us.us.us.us.us.i, %_0.i146.us.us.us.us.us.i, !dbg !3361
  %_0.i146.us.us.us.us.us.1.i = fadd float %_0.i163.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3363
  %_0.i163.us.us.us.us.us.2.i = fmul float %_0.i180.us.us.us.us.us.i, %_0.i146.us.us.us.us.us.1.i, !dbg !3361
  %_0.i146.us.us.us.us.us.2.i = fadd float %_0.i163.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3363
  %_0.i163.us.us.us.us.us.3.i = fmul float %_0.i180.us.us.us.us.us.i, %_0.i146.us.us.us.us.us.2.i, !dbg !3361
  %_0.i146.us.us.us.us.us.3.i = fadd float %_0.i163.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3363
  %_0.i162.us.us.us.us.us.i = fmul float %_0.i180.us.us.us.us.us.i, %_0.i146.us.us.us.us.us.3.i, !dbg !3365
  %_0.i145.us.us.us.us.us.i = fadd float %_0.i162.us.us.us.us.us.i, 1.000000e+00, !dbg !3367
  %biased.i.us.us.us.us.us.i = fadd float %186, 0x4160000FE0000000, !dbg !3369
  %_4.i83.us.us.us.us.us.i = bitcast float %biased.i.us.us.us.us.us.i to i32, !dbg !3373
  %_3.i84.us.us.us.us.us.i = shl i32 %_4.i83.us.us.us.us.us.i, 23, !dbg !3377
  %_0.i85.us.us.us.us.us.i = bitcast i32 %_3.i84.us.us.us.us.us.i to float, !dbg !3378
  %_0.i161.us.us.us.us.us.i = fmul float %_0.i145.us.us.us.us.us.i, %_0.i85.us.us.us.us.us.i, !dbg !3381
  %_3.i93.us.us.us.us.us.i = fcmp une float %_0.i230.us.us.us.us.us.i, 0.000000e+00, !dbg !3383
  %_3.i121.us.us.us.us.us.i = fcmp ule float %_98.i123.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3385
  %_0.i392572.not.us.us.us.us.us.i = and i1 %_3.i121.us.us.us.us.us.i, %_3.i93.us.us.us.us.us.i, !dbg !3387
  %_0.i172.us.us.us.us.us.i = fmul float %_0.i205.us.us.us.us.us.i, %_0.i161.us.us.us.us.us.i, !dbg !3387
  %_4.i314.v.us.us.us.us.us.i = select i1 %_0.i392572.not.us.us.us.us.us.i, float %_0.i172.us.us.us.us.us.i, float %_0.i205.us.us.us.us.us.i, !dbg !3390
  %_0.i.us.us.us.us.us.i = fmul float %_0.i181.us.us.us.us.us.i, %_0.i148.us.us.us.us.us.3.i, !dbg !3392
  %_0.i147.us.us.us.us.us.i = fadd float %_0.i.us.us.us.us.us.i, 1.000000e+00, !dbg !3394
  %biased.i86.us.us.us.us.us.i = fadd float %201, 0x4160000FE0000000, !dbg !3396
  %_4.i87.us.us.us.us.us.i = bitcast float %biased.i86.us.us.us.us.us.i to i32, !dbg !3398
  %_3.i88.us.us.us.us.us.i = shl i32 %_4.i87.us.us.us.us.us.i, 23, !dbg !3400
  %_0.i89.us.us.us.us.us.i = bitcast i32 %_3.i88.us.us.us.us.us.i to float, !dbg !3401
  %_0.i164.us.us.us.us.us.i = fmul float %_0.i147.us.us.us.us.us.i, %_0.i89.us.us.us.us.us.i, !dbg !3403
  %_3.i90.us.us.us.us.us.i = fcmp une float %_0.i226.us.us.us.us.us.i, 0.000000e+00, !dbg !3405
  %_98.i.i.us.us.us.us.us.i = load float, ptr %29, align 4, !dbg !3407, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i105.us.us.us.us.us.i = fcmp ule float %_98.i.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3408
  %_0.i389594.not.us.us.us.us.us.i = and i1 %_3.i105.us.us.us.us.us.i, %_3.i90.us.us.us.us.us.i, !dbg !3410
  %_0.i166.us.us.us.us.us.i = fmul float %_0.i203.us.us.us.us.us.i, %_0.i164.us.us.us.us.us.i, !dbg !3410
  %_4.i234.v.us.us.us.us.us.i = select i1 %_0.i389594.not.us.us.us.us.us.i, float %_0.i166.us.us.us.us.us.i, float %_0.i203.us.us.us.us.us.i, !dbg !3412
  store float %_4.i314.v.us.us.us.us.us.i, ptr %_97.i.us.us.us.us.us.i, align 4, !dbg !3414, !alias.scope !3417, !noalias !2719
  store float %_4.i234.v.us.us.us.us.us.i, ptr %_115.i.us.us.us.us.us.i, align 4, !dbg !3420, !alias.scope !3422, !noalias !2742
  %exitcond1515.not.i = icmp eq i64 %165, %..i, !dbg !3425
  br i1 %exitcond1515.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb30.i.us.us.us.us.us.i, !dbg !3428, !llvm.loop !3438

bb8.i81.us.us.us.us.us.i:                         ; preds = %bb5.i.preheader.us.us.us.us.us.i
  %_34.i.us.us.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.us.us.i, !dbg !3429
  br i1 %_34.i.us.us.us.us.us.i, label %bb10.i82.us.us.us.us.us.i, label %panic5.i.i, !dbg !3429

bb10.i82.us.us.us.us.us.i:                        ; preds = %bb8.i81.us.us.us.us.us.i
  %202 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.us.us.i, !dbg !3436
  %left_own.i.us.us.us.us.us.i = load float, ptr %202, align 4, !dbg !3436, !alias.scope !2843, !noalias !2865, !noundef !12
  %203 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.us.us.i, !dbg !3429
  %right_own.i.us.us.us.us.us.i = load float, ptr %203, align 4, !dbg !3429, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i, !dbg !2870

bb5.i.preheader.us.us.us.us.us.i:                 ; preds = %bb50.i.us.us.us.us.us.i
  br i1 %_31.i80.us.us.us.us.us.i, label %bb8.i81.us.us.us.us.us.i, label %panic4.i.i, !dbg !3436

bb14.i63.preheader.us.us.us.us.us.i:              ; preds = %bb50.i.us.us.us.us.us.i
  br i1 %_31.i80.us.us.us.us.us.i, label %bb17.i.us.us.us.us.us.i, label %panic15.i.i, !dbg !2880

bb23.i.preheader.us.us.us.us.us.i:                ; preds = %bb50.i.us.us.us.us.us.i
  br i1 %_31.i80.us.us.us.us.us.i, label %bb27.i60.us.us.us.us.us.i, label %panic28.i.i, !dbg !2859

bb30.i.us.us.us.i:                                ; preds = %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i, %bb30.i.us.us.us.preheader.i
  %iter.sroa.0.0.i714.us.us.us.i = phi i64 [ %204, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i ], [ 0, %bb30.i.us.us.us.preheader.i ]
  %204 = add nuw nsw i64 %iter.sroa.0.0.i714.us.us.us.i, 1, !dbg !2683
  %_28.i.us.us.us.i = trunc i64 %iter.sroa.0.0.i714.us.us.us.i to i32, !dbg !2694
  %now.i.us.us.us.i = add i32 %base.i.i, %_28.i.us.us.us.i, !dbg !2695
  %_31.i.us.us.us.i = and i32 %now.i.us.us.us.i, %_58.i, !dbg !2698
  %_30.i.us.us.us.i = zext i32 %_31.i.us.us.us.i to i64, !dbg !2699
  %exitcond1518.not.i = icmp eq i64 %iter.sroa.0.0.i714.us.us.us.i, %..i, !dbg !2671
  br i1 %exitcond1518.not.i, label %bb33.i.i, label %bb32.i.us.us.us.i, !dbg !2671, !prof !180

bb32.i.us.us.us.i:                                ; preds = %bb30.i.us.us.us.i
  %_97.i.us.us.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i714.us.us.us.i, !dbg !2700
  %_98.not.not.i.us.us.us.i = icmp ugt i64 %_64.1.i, %_30.i.us.us.us.i, !dbg !2704
  br i1 %_98.not.not.i.us.us.us.i, label %bb35.i.us.us.us.i, label %bb36.i.i, !dbg !2704, !prof !2709

bb35.i.us.us.us.i:                                ; preds = %bb32.i.us.us.us.i
  %_0.i213.us.us.us.i = load float, ptr %_97.i.us.us.us.i, align 4, !dbg !2710, !alias.scope !2716, !noalias !2719, !noundef !12
  %_107.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_30.i.us.us.us.i, !dbg !2720
  store float %_0.i213.us.us.us.i, ptr %_107.i.us.us.us.i, align 4, !dbg !2724, !alias.scope !2727, !noalias !2668
  %_115.i.us.us.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i714.us.us.us.i, !dbg !2730
  %_0.i211.us.us.us.i = load float, ptr %_115.i.us.us.us.i, align 4, !dbg !2737, !alias.scope !2739, !noalias !2742, !noundef !12
  %_123.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_30.i.us.us.us.i, !dbg !2743
  store float %_0.i211.us.us.us.i, ptr %_123.i.us.us.us.i, align 4, !dbg !2750, !alias.scope !2752, !noalias !2668
  %_132.not.not.i.us.us.us.i = icmp ugt i64 %_65.1.i, %_30.i.us.us.us.i, !dbg !3440
  br i1 %_132.not.not.i.us.us.us.i, label %bb44.i.us.us.us.i, label %bb45.i.i, !dbg !3440, !prof !2709

bb44.i.us.us.us.i:                                ; preds = %bb35.i.us.us.us.i
  %_131.i.us.us.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.us.us.i, !dbg !2755
  %_0.i209.us.us.us.i = load float, ptr %_131.i.us.us.us.i, align 4, !dbg !2762, !alias.scope !2764, !noalias !2668, !noundef !12
  %_139.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_30.i.us.us.us.i, !dbg !2767
  store float %_0.i209.us.us.us.i, ptr %_139.i.us.us.us.i, align 4, !dbg !2774, !alias.scope !2776, !noalias !2668
  %_148.not.not.i.us.us.us.i = icmp ugt i64 %_67.1.i, %_30.i.us.us.us.i, !dbg !3437
  br i1 %_148.not.not.i.us.us.us.i, label %bb48.i.us.us.us.i, label %bb49.i.i, !dbg !3437, !prof !2709

bb48.i.us.us.us.i:                                ; preds = %bb44.i.us.us.us.i
  %_147.i.us.us.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.us.us.i, !dbg !2779
  %_0.i207.us.us.us.i = load float, ptr %_147.i.us.us.us.i, align 4, !dbg !2786, !alias.scope !2788, !noalias !2668, !noundef !12
  %_155.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_30.i.us.us.us.i, !dbg !2791
  store float %_0.i207.us.us.us.i, ptr %_155.i.us.us.us.i, align 4, !dbg !2798, !alias.scope !2800, !noalias !2668
  %_53.i.us.us.us.i = sub i32 %now.i.us.us.us.i, %_59.i, !dbg !2803
  %_52.i.us.us.us.i = and i32 %_53.i.us.us.us.i, %_58.i, !dbg !2806
  %_51.i.us.us.us.i = zext i32 %_52.i.us.us.us.i to i64, !dbg !2807
  %_156.not.not.i.us.us.us.i = icmp ugt i64 %_64.1.i, %_51.i.us.us.us.i, !dbg !2808
  br i1 %_156.not.not.i.us.us.us.i, label %bb50.i.us.us.us.i, label %bb51.i.i, !dbg !2808, !prof !2709

bb50.i.us.us.us.i:                                ; preds = %bb48.i.us.us.us.i
  %_163.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_51.i.us.us.us.i, !dbg !2813
  %_0.i205.us.us.us.i = load float, ptr %_163.i.us.us.us.i, align 4, !dbg !2817, !alias.scope !2819, !noalias !2668, !noundef !12
  %_169.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_51.i.us.us.us.i, !dbg !2822
  %_0.i203.us.us.us.i = load float, ptr %_169.i.us.us.us.i, align 4, !dbg !2830, !alias.scope !2832, !noalias !2668, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2835), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2841), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2845), !dbg !2838
  %_18.i74.us.us.us.i = load i32, ptr %_31.i, align 4, !dbg !2847, !alias.scope !2850, !noalias !2851, !noundef !12
  %_17.i.us.us.us.i = sub i32 %now.i.us.us.us.i, %_18.i74.us.us.us.i, !dbg !2853
  %_16.i75.us.us.us.i = and i32 %_17.i.us.us.us.i, %_58.i, !dbg !2847
  %_15.i76.us.us.us.i = zext i32 %_16.i75.us.us.us.i to i64, !dbg !2847
  %_26.i.us.us.us.i = load i32, ptr %_33.i, align 4, !dbg !2847, !alias.scope !2856, !noalias !2857, !noundef !12
  %_25.i.us.us.us.i = sub i32 %now.i.us.us.us.i, %_26.i.us.us.us.i, !dbg !2853
  %_24.i.us.us.us.i = and i32 %_25.i.us.us.us.i, %_58.i, !dbg !2847
  %_23.i79.us.us.us.i = zext i32 %_24.i.us.us.us.i to i64, !dbg !2847
  %_31.i80.us.us.us.i = icmp samesign ugt i64 %_65.1.i, %_15.i76.us.us.us.i, !dbg !2847
  switch i8 %_0.sroa.0.0.i551.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.us.i
    i8 1, label %bb14.i63.preheader.us.us.us.i
    i8 2, label %bb23.i.preheader.us.us.us.i
  ], !dbg !2858

bb27.i60.us.us.us.i:                              ; preds = %bb23.i.preheader.us.us.us.i
  %205 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.i, !dbg !2859
  %_79.i.us.us.us.i = load float, ptr %205, align 4, !dbg !2859, !alias.scope !2843, !noalias !2865, !noundef !12
  %_85.i.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_15.i76.us.us.us.i, !dbg !2866
  br i1 %_85.i.us.us.us.i, label %bb29.i61.us.us.us.i, label %panic30.i.i, !dbg !2866

bb29.i61.us.us.us.i:                              ; preds = %bb27.i60.us.us.us.i
  %206 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_15.i76.us.us.us.i, !dbg !2866
  %_83.i.us.us.us.i = load float, ptr %206, align 4, !dbg !2866, !alias.scope !2845, !noalias !2867, !noundef !12
  %_87.i.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.i, !dbg !2868
  br i1 %_87.i.us.us.us.i, label %bb31.i.us.us.us.i, label %panic32.i.i, !dbg !2868

bb31.i.us.us.us.i:                                ; preds = %bb29.i61.us.us.us.i
  %_89.i.us.us.us.i = icmp samesign ugt i64 %_65.1.i, %_23.i79.us.us.us.i, !dbg !2869
  br i1 %_89.i.us.us.us.i, label %bb33.i62.us.us.us.i, label %panic34.i.i, !dbg !2869

bb33.i62.us.us.us.i:                              ; preds = %bb31.i.us.us.us.i
  %207 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.i, !dbg !2868
  %_86.i.us.us.us.i = load float, ptr %207, align 4, !dbg !2868, !alias.scope !2845, !noalias !2867, !noundef !12
  %208 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_23.i79.us.us.us.i, !dbg !2869
  %_88.i.us.us.us.i = load float, ptr %208, align 4, !dbg !2869, !alias.scope !2843, !noalias !2865, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i, !dbg !2870

bb17.i.us.us.us.i:                                ; preds = %bb14.i63.preheader.us.us.us.i
  %_59.i.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.i, !dbg !2873
  br i1 %_59.i.us.us.us.i, label %bb19.i.us.us.us.i, label %panic17.i.i, !dbg !2873

bb19.i.us.us.us.i:                                ; preds = %bb17.i.us.us.us.i
  %209 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.i, !dbg !2880
  %left_own16.i.us.us.us.i = load float, ptr %209, align 4, !dbg !2880, !alias.scope !2843, !noalias !2865, !noundef !12
  %210 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.i, !dbg !2873
  %right_own18.i.us.us.us.i = load float, ptr %210, align 4, !dbg !2873, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i, !dbg !2870

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i: ; preds = %bb10.i82.us.us.us.i, %bb19.i.us.us.us.i, %bb33.i62.us.us.us.i
  %taps.i.sroa.1011534.3.i = phi float [ %right_own.i.us.us.us.i, %bb10.i82.us.us.us.i ], [ %left_own16.i.us.us.us.i, %bb19.i.us.us.us.i ], [ %_88.i.us.us.us.i, %bb33.i62.us.us.us.i ], !dbg !2847
  %taps.i.sroa.681533.3.i = phi float [ %right_own.i.us.us.us.i, %bb10.i82.us.us.us.i ], [ %right_own18.i.us.us.us.i, %bb19.i.us.us.us.i ], [ %_86.i.us.us.us.i, %bb33.i62.us.us.us.i ], !dbg !2847
  %taps.i.sroa.351532.3.i = phi float [ %left_own.i.us.us.us.i, %bb10.i82.us.us.us.i ], [ %right_own18.i.us.us.us.i, %bb19.i.us.us.us.i ], [ %_83.i.us.us.us.i, %bb33.i62.us.us.us.i ], !dbg !2847
  %taps.i.sroa.0.3.i = phi float [ %left_own.i.us.us.us.i, %bb10.i82.us.us.us.i ], [ %left_own16.i.us.us.us.i, %bb19.i.us.us.us.i ], [ %_79.i.us.us.us.i, %bb33.i62.us.us.us.i ], !dbg !2847
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2881), !dbg !2884
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2885), !dbg !2884
  %_12.i36.i.us.us.us.i = load float, ptr %78, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.i = fcmp ule float %_12.i36.i.us.us.us.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.i = fcmp une float %_12.i36.i.us.us.us.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.i = load float, ptr %_10.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.i = load float, ptr %79, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.i = fadd float %_16.i39.i.us.us.us.i, %_17.i40.i.us.us.us.i, !dbg !2906
  %_20.i42.i552555.us.us.us.i = load float, ptr %80, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %211 = select i1 %_3.i95.us.us.us.i, float %_0.i152.us.us.us.i, float %_20.i42.i552555.us.us.us.i, !dbg !2911
  %_0.i381.us.us.us.i = select i1 %_3.i135.us.us.us.i, float %_16.i39.i.us.us.us.i, float %211, !dbg !2914
  store float %_0.i381.us.us.us.i, ptr %_10.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.i = select i1 %_3.i95.us.us.us.i, float %_17.i40.i.us.us.us.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.i, ptr %79, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.i = fadd float %_12.i36.i.us.us.us.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.i = select i1 %_3.i135.us.us.us.i, float %_12.i36.i.us.us.us.i, float %_0.i193.us.us.us.i, !dbg !2923
  store float %_4.i367.v.us.us.us.i, ptr %78, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.1.i = load float, ptr %81, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.1.i = fcmp ule float %_12.i36.i.us.us.us.1.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.1.i = fcmp une float %_12.i36.i.us.us.us.1.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.1.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.1.i = load float, ptr %82, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.1.i = fadd float %_16.i39.i.us.us.us.1.i, %_17.i40.i.us.us.us.1.i, !dbg !2906
  %_20.i42.i552555.us.us.us.1.i = load float, ptr %83, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %212 = select i1 %_3.i95.us.us.us.1.i, float %_0.i152.us.us.us.1.i, float %_20.i42.i552555.us.us.us.1.i, !dbg !2911
  %_0.i381.us.us.us.1.i = select i1 %_3.i135.us.us.us.1.i, float %_16.i39.i.us.us.us.1.i, float %212, !dbg !2914
  store float %_0.i381.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.1.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.1.i = select i1 %_3.i95.us.us.us.1.i, float %_17.i40.i.us.us.us.1.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.1.i, ptr %82, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.1.i = fadd float %_12.i36.i.us.us.us.1.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.1.i = select i1 %_3.i135.us.us.us.1.i, float %_12.i36.i.us.us.us.1.i, float %_0.i193.us.us.us.1.i, !dbg !2923
  store float %_4.i367.v.us.us.us.1.i, ptr %81, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.2.i = load float, ptr %84, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.2.i = fcmp ule float %_12.i36.i.us.us.us.2.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.2.i = fcmp une float %_12.i36.i.us.us.us.2.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.2.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.2.i = load float, ptr %85, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.2.i = fadd float %_16.i39.i.us.us.us.2.i, %_17.i40.i.us.us.us.2.i, !dbg !2906
  %_20.i42.i552555.us.us.us.2.i = load float, ptr %86, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %213 = select i1 %_3.i95.us.us.us.2.i, float %_0.i152.us.us.us.2.i, float %_20.i42.i552555.us.us.us.2.i, !dbg !2911
  %_0.i381.us.us.us.2.i = select i1 %_3.i135.us.us.us.2.i, float %_16.i39.i.us.us.us.2.i, float %213, !dbg !2914
  store float %_0.i381.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.2.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.2.i = select i1 %_3.i95.us.us.us.2.i, float %_17.i40.i.us.us.us.2.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.2.i, ptr %85, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.2.i = fadd float %_12.i36.i.us.us.us.2.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.2.i = select i1 %_3.i135.us.us.us.2.i, float %_12.i36.i.us.us.us.2.i, float %_0.i193.us.us.us.2.i, !dbg !2923
  store float %_4.i367.v.us.us.us.2.i, ptr %84, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.us.3.i = load float, ptr %87, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.us.3.i = fcmp ule float %_12.i36.i.us.us.us.3.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.us.3.i = fcmp une float %_12.i36.i.us.us.us.3.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.3.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.us.3.i = load float, ptr %88, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.us.3.i = fadd float %_16.i39.i.us.us.us.3.i, %_17.i40.i.us.us.us.3.i, !dbg !2906
  %_20.i42.i552555.us.us.us.3.i = load float, ptr %89, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %214 = select i1 %_3.i95.us.us.us.3.i, float %_0.i152.us.us.us.3.i, float %_20.i42.i552555.us.us.us.3.i, !dbg !2911
  %_0.i381.us.us.us.3.i = select i1 %_3.i135.us.us.us.3.i, float %_16.i39.i.us.us.us.3.i, float %214, !dbg !2914
  store float %_0.i381.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.us.3.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.us.3.i = select i1 %_3.i95.us.us.us.3.i, float %_17.i40.i.us.us.us.3.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.us.3.i, ptr %88, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.us.3.i = fadd float %_12.i36.i.us.us.us.3.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.us.3.i = select i1 %_3.i135.us.us.us.3.i, float %_12.i36.i.us.us.us.3.i, float %_0.i193.us.us.us.3.i, !dbg !2923
  store float %_4.i367.v.us.us.us.3.i, ptr %87, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %215 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.3.i), !dbg !2926
  %216 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351532.3.i), !dbg !2933
  %_37.i57.i.us.us.us.i = load float, ptr %15, align 4, !dbg !2936, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i133.us.us.us.i = fcmp ule float %_37.i57.i.us.us.us.i, 0.000000e+00, !dbg !2941
  %_3.i.i489.us.us.us.i = fcmp ule float %215, %216, !dbg !2943
  %_6.i.i491.us.us.us.i = bitcast float %215 to i32, !dbg !2949
  %_8.i.i493.us.us.us.i = bitcast float %216 to i32, !dbg !2953
  %_4.i.i496.us.us.us.i = select i1 %_3.i.i489.us.us.us.i, i32 %_8.i.i493.us.us.us.i, i32 %_6.i.i491.us.us.us.i, !dbg !2955
  %_4.i360.us.us.us.i = select i1 %_3.i133.us.us.us.i, i32 %_6.i.i491.us.us.us.i, i32 %_4.i.i496.us.us.us.i, !dbg !2956
  %_41.i61.i.us.us.us.i = load float, ptr %16, align 4, !dbg !2958, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i131.us.us.us.i = fcmp ule float %_41.i61.i.us.us.us.i, 0.000000e+00, !dbg !2959
  %_0.i177.us.us.us.i = fmul float %215, 5.000000e-01, !dbg !2961
  %_0.i176.us.us.us.i = fmul float %216, 5.000000e-01, !dbg !2964
  %_0.i151.us.us.us.i = fadd float %_0.i176.us.us.us.i, %_0.i177.us.us.us.i, !dbg !2966
  %_6.i348.us.us.us.i = bitcast float %_0.i151.us.us.us.i to i32, !dbg !2968
  %_4.i353.us.us.us.i = select i1 %_3.i131.us.us.us.i, i32 %_4.i360.us.us.us.i, i32 %_6.i348.us.us.us.i, !dbg !2971
  %_0.i354.us.us.us.i = bitcast i32 %_4.i353.us.us.us.i to float, !dbg !2972
  %_3.i.i481.us.us.us.i = fcmp ule float %_0.i354.us.us.us.i, 0x3E45798EE0000000, !dbg !2975
  %_4.i.i487.us.us.us.i = select i1 %_3.i.i481.us.us.us.i, i32 841731191, i32 %_4.i353.us.us.us.i, !dbg !2978
  %_0.i.i488.us.us.us.i = bitcast i32 %_4.i.i487.us.us.us.i to float, !dbg !2980
  %_3.i.i.us.us.us.i = fcmp ule float %_0.i.i488.us.us.us.i, 0x3810000000000000, !dbg !2982
  %_4.i.i.us.us.us.i = select i1 %_3.i.i.us.us.us.i, i32 8388608, i32 %_4.i.i487.us.us.us.i, !dbg !2992
  %_5.i214.us.us.us.i = and i32 %_4.i.i.us.us.us.i, 8388607, !dbg !2994
  %_4.i215.us.us.us.i = or disjoint i32 %_5.i214.us.us.us.i, 1065353216, !dbg !2994
  %significand.i.us.us.us.i = bitcast i32 %_4.i215.us.us.us.i to float, !dbg !2999
  %_0.i178.us.us.us.i = fadd float %significand.i.us.us.us.i, -1.000000e+00, !dbg !3002
  %_0.i158.us.us.us.i = fmul float %_0.i178.us.us.us.i, 0x3F9B17A960000000, !dbg !3005
  %217 = fsub float 0x3FBF9A8440000000, %_0.i158.us.us.us.i, !dbg !3010
  %_0.i158.us.us.us.1.i = fmul float %_0.i178.us.us.us.i, %217, !dbg !3005
  %_0.i142.us.us.us.1.i = fadd float %_0.i158.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3010
  %_0.i158.us.us.us.2.i = fmul float %_0.i178.us.us.us.i, %_0.i142.us.us.us.1.i, !dbg !3005
  %_0.i142.us.us.us.2.i = fadd float %_0.i158.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3010
  %_0.i158.us.us.us.3.i = fmul float %_0.i178.us.us.us.i, %_0.i142.us.us.us.2.i, !dbg !3005
  %_0.i142.us.us.us.3.i = fadd float %_0.i158.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3010
  %_0.i158.us.us.us.4.i = fmul float %_0.i178.us.us.us.i, %_0.i142.us.us.us.3.i, !dbg !3005
  %_0.i142.us.us.us.4.i = fadd float %_0.i158.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3010
  %_9.i.us.us.us.i = lshr i32 %_4.i.i.us.us.us.i, 23, !dbg !3012
  %_8.i216.us.us.us.i = or disjoint i32 %_9.i.us.us.us.i, 1258291200, !dbg !3012
  %_7.i.us.us.us.i = bitcast i32 %_8.i216.us.us.us.i to float, !dbg !3014
  %exponent.i.us.us.us.i = fadd float %_7.i.us.us.us.i, 0xC160000FE0000000, !dbg !3016
  %_0.i157.us.us.us.i = fmul float %_0.i178.us.us.us.i, %_0.i142.us.us.us.4.i, !dbg !3017
  %_0.i141.us.us.us.i = fadd float %exponent.i.us.us.us.i, %_0.i157.us.us.us.i, !dbg !3019
  %_0.i175.us.us.us.i = fmul float %_0.i141.us.us.us.i, 0x4018151820000000, !dbg !3021
  %_3.i.i538.us.us.us.inv.i = fcmp olt float %_0.i175.us.us.us.i, 2.400000e+01, !dbg !3023
  %_0.i.i545.us.us.us.i = select i1 %_3.i.i538.us.us.us.inv.i, float %_0.i175.us.us.us.i, float 2.400000e+01, !dbg !3023
  %_3.i.i473.us.us.us.inv.i = fcmp ogt float %_0.i.i545.us.us.us.i, -1.600000e+02, !dbg !3027
  %_0.i.i480.us.us.us.i = select i1 %_3.i.i473.us.us.us.inv.i, float %_0.i.i545.us.us.us.i, float -1.600000e+02, !dbg !3027
  %_55.i78.i.us.us.us.i = load float, ptr %14, align 4, !dbg !3030, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i129.us.us.us.i = fcmp ule float %_55.i78.i.us.us.us.i, 0.000000e+00, !dbg !3032
  %_3.i103.us.us.us.i = fcmp oge float %_0.i.i480.us.us.us.i, %_0.i381.us.us.us.i, !dbg !3034
  %_0.i192.us.us.us.i = fsub float %_0.i381.us.us.us.i, %_0.i381.us.us.us.3.i, !dbg !3038
  %_3.i101.us.us.us.i = fcmp oge float %_0.i.i480.us.us.us.i, %_0.i192.us.us.us.i, !dbg !3041
  %..i102.us.us.us.i = sext i1 %_3.i101.us.us.us.i to i32, !dbg !3043
  %_0.i401.us.us.us.i = sext i1 %_3.i103.us.us.us.i to i32, !dbg !3046
  %_0.i394.us.us.us.i = select i1 %_3.i129.us.us.us.i, i32 %_0.i401.us.us.us.i, i32 %..i102.us.us.us.i, !dbg !3046
  %_0.i405.us.us.us.i = xor i32 %..i102.us.us.us.i, -1, !dbg !3052
  %_67.i88.i.us.us.us.i = load float, ptr %17, align 4, !dbg !3056, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i127.us.us.us.i = fcmp ogt float %_67.i88.i.us.us.us.i, 0.000000e+00, !dbg !3057
  %_0.i400.us.us.us.i = select i1 %_3.i127.us.us.us.i, i32 %_0.i405.us.us.us.i, i32 0, !dbg !3059
  %_0.i399.us.us.us.i = select i1 %_3.i129.us.us.us.i, i32 0, i32 %_0.i400.us.us.us.i, !dbg !3062
  %_0.i393.us.us.us.i = or i32 %_0.i399.us.us.us.i, %_0.i394.us.us.us.i, !dbg !3064
  %_5.i343.us.us.us.i = and i32 %_0.i393.us.us.us.i, 1065353216, !dbg !3067
  %_0.i347.us.us.us.i = bitcast i32 %_5.i343.us.us.us.i to float, !dbg !3069
  %_71.i94.i564565.us.us.us.i = load float, ptr %18, align 4, !dbg !3071, !alias.scope !2939, !noalias !2940, !noundef !12
  %_0.i191.us.us.us.i = fadd float %_67.i88.i.us.us.us.i, -1.000000e+00, !dbg !3073
  %218 = trunc nsw i32 %_0.i399.us.us.us.i to i1, !dbg !3075
  %_4.i341.v.us.us.us.i = select i1 %218, float %_0.i191.us.us.us.i, float %_67.i88.i.us.us.us.i, !dbg !3075
  %219 = trunc nsw i32 %_0.i394.us.us.us.i to i1, !dbg !3077
  %_0.i335.us.us.us.i = select i1 %219, float %_71.i94.i564565.us.us.us.i, float %_4.i341.v.us.us.us.i, !dbg !3077
  store float %_0.i335.us.us.us.i, ptr %17, align 4, !dbg !3079, !alias.scope !2894, !noalias !2895
  store i32 %_5.i343.us.us.us.i, ptr %14, align 4, !dbg !3080, !alias.scope !2894, !noalias !2895
  %_0.i190.us.us.us.i = fadd float %_0.i381.us.us.us.1.i, -1.000000e+00, !dbg !3081
  %_0.i189.us.us.us.i = fsub float %_0.i.i480.us.us.us.i, %_0.i381.us.us.us.i, !dbg !3083
  %_0.i174.us.us.us.i = fmul float %_0.i190.us.us.us.i, %_0.i189.us.us.us.i, !dbg !3085
  %220 = fneg float %_0.i381.us.us.us.2.i, !dbg !3087
  %_3.i.i464.inv.us.us.us.i = fcmp ogt float %_0.i174.us.us.us.i, %220, !dbg !3090
  %_4.i.i471.v.us.us.us.i = select i1 %_3.i.i464.inv.us.us.us.i, float %_0.i174.us.us.us.i, float %220, !dbg !3090
  %_3.i.i530.us.us.us.i = fcmp olt float %_4.i.i471.v.us.us.us.i, 0.000000e+00, !dbg !3093
  %221 = fcmp ule float %_0.i347.us.us.us.i, 0.000000e+00, !dbg !3097
  %222 = select i1 %221, i1 %_3.i.i530.us.us.us.i, i1 false, !dbg !3100
  %_0.i328.us.us.us.i = select i1 %222, float %_4.i.i471.v.us.us.us.i, float 0.000000e+00, !dbg !3100
  %_86.i107.i.us.us.us.i = load float, ptr %19, align 4, !dbg !3101, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i123.us.us.us.i = fcmp ule float %_0.i328.us.us.us.i, %_86.i107.i.us.us.us.i, !dbg !3103
  %_87.i109.i567.us.us.us.i = load i32, ptr %4, align 4, !dbg !3105, !alias.scope !2939, !noalias !2940, !noundef !12
  %_88.i110.i568.us.us.us.i = load i32, ptr %20, align 4, !dbg !3106, !alias.scope !2939, !noalias !2940, !noundef !12
  %_4.i321.us.us.us.i = select i1 %_3.i123.us.us.us.i, i32 %_88.i110.i568.us.us.us.i, i32 %_87.i109.i567.us.us.us.i, !dbg !3107
  %_0.i322.us.us.us.i = bitcast i32 %_4.i321.us.us.us.i to float, !dbg !3109
  %_0.i188.us.us.us.i = fsub float %_0.i328.us.us.us.i, %_86.i107.i.us.us.us.i, !dbg !3111
  %_4.i155.us.us.us.i = fmul float %_0.i188.us.us.us.i, %_0.i322.us.us.us.i, !dbg !3114
  %_0.i156.us.us.us.i = fadd float %_86.i107.i.us.us.us.i, %_4.i155.us.us.us.i, !dbg !3114
  %223 = tail call noundef float @llvm.fabs.f32(float %_0.i156.us.us.us.i), !dbg !3117
  %224 = fcmp uge float %223, 0x3BC79CA100000000, !dbg !3121
  %_0.i230.us.us.us.i = select i1 %224, float %_0.i156.us.us.us.i, float 0.000000e+00, !dbg !3124
  store float %_0.i230.us.us.us.i, ptr %19, align 4, !dbg !3125, !alias.scope !2894, !noalias !2895
  %_0.i173.us.us.us.i = fmul float %_0.i230.us.us.us.i, 0x3FC542A5A0000000, !dbg !3127
  %_3.i.i415.us.us.us.inv.i = fcmp ogt float %_0.i173.us.us.us.i, -1.260000e+02, !dbg !3131
  %_0.i.i422.us.us.us.i = select i1 %_3.i.i415.us.us.us.inv.i, float %_0.i173.us.us.us.i, float -1.260000e+02, !dbg !3131
  %_3.i.i498.us.us.us.inv.i = fcmp olt float %_0.i.i422.us.us.us.i, 1.270000e+02, !dbg !3136
  %_0.i.i505.us.us.us.i = select i1 %_3.i.i498.us.us.us.inv.i, float %_0.i.i422.us.us.us.i, float 1.270000e+02, !dbg !3136
  %225 = tail call noundef float @llvm.floor.f32(float %_0.i.i505.us.us.us.i), !dbg !3139
  %_0.i180.us.us.us.i = fsub float %_0.i.i505.us.us.us.i, %225, !dbg !3151
  %_98.i123.i.us.us.us.i = load float, ptr %21, align 4, !dbg !3154, !alias.scope !2939, !noalias !2940, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3156), !dbg !3159
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3161), !dbg !3159
  %_12.i.i.us.us.us.i = load float, ptr %90, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.i = fcmp ule float %_12.i.i.us.us.us.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.i = fcmp une float %_12.i.i.us.us.us.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.i = load float, ptr %91, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.i = fadd float %_16.i.i.us.us.us.i, %_17.i.i.us.us.us.i, !dbg !3173
  %_20.i.i574577.us.us.us.i = load float, ptr %92, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %226 = select i1 %_3.i91.us.us.us.i, float %_0.i150.us.us.us.i, float %_20.i.i574577.us.us.us.i, !dbg !3176
  %_0.i301.us.us.us.i = select i1 %_3.i119.us.us.us.i, float %_16.i.i.us.us.us.i, float %226, !dbg !3178
  store float %_0.i301.us.us.us.i, ptr %data.i.i.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.i = select i1 %_3.i91.us.us.us.i, float %_17.i.i.us.us.us.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.i, ptr %91, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.i = fadd float %_12.i.i.us.us.us.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.i = select i1 %_3.i119.us.us.us.i, float %_12.i.i.us.us.us.i, float %_0.i187.us.us.us.i, !dbg !3186
  store float %_4.i287.v.us.us.us.i, ptr %90, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.1.i = load float, ptr %93, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.1.i = fcmp ule float %_12.i.i.us.us.us.1.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.1.i = fcmp une float %_12.i.i.us.us.us.1.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.1.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.1.i = load float, ptr %94, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.1.i = fadd float %_16.i.i.us.us.us.1.i, %_17.i.i.us.us.us.1.i, !dbg !3173
  %_20.i.i574577.us.us.us.1.i = load float, ptr %95, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %227 = select i1 %_3.i91.us.us.us.1.i, float %_0.i150.us.us.us.1.i, float %_20.i.i574577.us.us.us.1.i, !dbg !3176
  %_0.i301.us.us.us.1.i = select i1 %_3.i119.us.us.us.1.i, float %_16.i.i.us.us.us.1.i, float %227, !dbg !3178
  store float %_0.i301.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.1.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.1.i = select i1 %_3.i91.us.us.us.1.i, float %_17.i.i.us.us.us.1.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.1.i, ptr %94, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.1.i = fadd float %_12.i.i.us.us.us.1.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.1.i = select i1 %_3.i119.us.us.us.1.i, float %_12.i.i.us.us.us.1.i, float %_0.i187.us.us.us.1.i, !dbg !3186
  store float %_4.i287.v.us.us.us.1.i, ptr %93, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.2.i = load float, ptr %96, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.2.i = fcmp ule float %_12.i.i.us.us.us.2.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.2.i = fcmp une float %_12.i.i.us.us.us.2.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.2.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.2.i = load float, ptr %97, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.2.i = fadd float %_16.i.i.us.us.us.2.i, %_17.i.i.us.us.us.2.i, !dbg !3173
  %_20.i.i574577.us.us.us.2.i = load float, ptr %98, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %228 = select i1 %_3.i91.us.us.us.2.i, float %_0.i150.us.us.us.2.i, float %_20.i.i574577.us.us.us.2.i, !dbg !3176
  %_0.i301.us.us.us.2.i = select i1 %_3.i119.us.us.us.2.i, float %_16.i.i.us.us.us.2.i, float %228, !dbg !3178
  store float %_0.i301.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.2.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.2.i = select i1 %_3.i91.us.us.us.2.i, float %_17.i.i.us.us.us.2.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.2.i, ptr %97, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.2.i = fadd float %_12.i.i.us.us.us.2.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.2.i = select i1 %_3.i119.us.us.us.2.i, float %_12.i.i.us.us.us.2.i, float %_0.i187.us.us.us.2.i, !dbg !3186
  store float %_4.i287.v.us.us.us.2.i, ptr %96, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.us.3.i = load float, ptr %99, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.us.3.i = fcmp ule float %_12.i.i.us.us.us.3.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.us.3.i = fcmp une float %_12.i.i.us.us.us.3.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.3.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.us.3.i = load float, ptr %100, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.us.3.i = fadd float %_16.i.i.us.us.us.3.i, %_17.i.i.us.us.us.3.i, !dbg !3173
  %_20.i.i574577.us.us.us.3.i = load float, ptr %101, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %229 = select i1 %_3.i91.us.us.us.3.i, float %_0.i150.us.us.us.3.i, float %_20.i.i574577.us.us.us.3.i, !dbg !3176
  %_0.i301.us.us.us.3.i = select i1 %_3.i119.us.us.us.3.i, float %_16.i.i.us.us.us.3.i, float %229, !dbg !3178
  store float %_0.i301.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.us.3.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.us.3.i = select i1 %_3.i91.us.us.us.3.i, float %_17.i.i.us.us.us.3.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.us.3.i, ptr %100, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.us.3.i = fadd float %_12.i.i.us.us.us.3.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.us.3.i = select i1 %_3.i119.us.us.us.3.i, float %_12.i.i.us.us.us.3.i, float %_0.i187.us.us.us.3.i, !dbg !3186
  store float %_4.i287.v.us.us.us.3.i, ptr %99, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %230 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681533.3.i), !dbg !3189
  %231 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011534.3.i), !dbg !3191
  %_37.i.i.us.us.us.i = load float, ptr %23, align 4, !dbg !3193, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i117.us.us.us.i = fcmp ule float %_37.i.i.us.us.us.i, 0.000000e+00, !dbg !3196
  %_3.i.i455.us.us.us.i = fcmp ule float %230, %231, !dbg !3198
  %_6.i.i457.us.us.us.i = bitcast float %230 to i32, !dbg !3201
  %_8.i.i459.us.us.us.i = bitcast float %231 to i32, !dbg !3204
  %_4.i.i462.us.us.us.i = select i1 %_3.i.i455.us.us.us.i, i32 %_8.i.i459.us.us.us.i, i32 %_6.i.i457.us.us.us.i, !dbg !3206
  %_4.i280.us.us.us.i = select i1 %_3.i117.us.us.us.i, i32 %_6.i.i457.us.us.us.i, i32 %_4.i.i462.us.us.us.i, !dbg !3207
  %_41.i.i.us.us.us.i = load float, ptr %24, align 4, !dbg !3209, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i115.us.us.us.i = fcmp ule float %_41.i.i.us.us.us.i, 0.000000e+00, !dbg !3210
  %_0.i171.us.us.us.i = fmul float %230, 5.000000e-01, !dbg !3212
  %_0.i170.us.us.us.i = fmul float %231, 5.000000e-01, !dbg !3214
  %_0.i149.us.us.us.i = fadd float %_0.i170.us.us.us.i, %_0.i171.us.us.us.i, !dbg !3216
  %_6.i268.us.us.us.i = bitcast float %_0.i149.us.us.us.i to i32, !dbg !3218
  %_4.i273.us.us.us.i = select i1 %_3.i115.us.us.us.i, i32 %_4.i280.us.us.us.i, i32 %_6.i268.us.us.us.i, !dbg !3221
  %_0.i274.us.us.us.i = bitcast i32 %_4.i273.us.us.us.i to float, !dbg !3222
  %_3.i.i447.us.us.us.i = fcmp ule float %_0.i274.us.us.us.i, 0x3E45798EE0000000, !dbg !3224
  %_4.i.i453.us.us.us.i = select i1 %_3.i.i447.us.us.us.i, i32 841731191, i32 %_4.i273.us.us.us.i, !dbg !3227
  %_0.i.i454.us.us.us.i = bitcast i32 %_4.i.i453.us.us.us.i to float, !dbg !3229
  %_3.i.i407.us.us.us.i = fcmp ule float %_0.i.i454.us.us.us.i, 0x3810000000000000, !dbg !3231
  %_4.i.i413.us.us.us.i = select i1 %_3.i.i407.us.us.us.i, i32 8388608, i32 %_4.i.i453.us.us.us.i, !dbg !3236
  %_5.i218.us.us.us.i = and i32 %_4.i.i413.us.us.us.i, 8388607, !dbg !3238
  %_4.i219.us.us.us.i = or disjoint i32 %_5.i218.us.us.us.i, 1065353216, !dbg !3238
  %significand.i220.us.us.us.i = bitcast i32 %_4.i219.us.us.us.i to float, !dbg !3240
  %_0.i179.us.us.us.i = fadd float %significand.i220.us.us.us.i, -1.000000e+00, !dbg !3242
  %_0.i160.us.us.us.i = fmul float %_0.i179.us.us.us.i, 0x3F9B17A960000000, !dbg !3244
  %232 = fsub float 0x3FBF9A8440000000, %_0.i160.us.us.us.i, !dbg !3246
  %_0.i160.us.us.us.1.i = fmul float %_0.i179.us.us.us.i, %232, !dbg !3244
  %_0.i144.us.us.us.1.i = fadd float %_0.i160.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3246
  %_0.i160.us.us.us.2.i = fmul float %_0.i179.us.us.us.i, %_0.i144.us.us.us.1.i, !dbg !3244
  %_0.i144.us.us.us.2.i = fadd float %_0.i160.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3246
  %_0.i160.us.us.us.3.i = fmul float %_0.i179.us.us.us.i, %_0.i144.us.us.us.2.i, !dbg !3244
  %_0.i144.us.us.us.3.i = fadd float %_0.i160.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3246
  %_0.i160.us.us.us.4.i = fmul float %_0.i179.us.us.us.i, %_0.i144.us.us.us.3.i, !dbg !3244
  %_0.i144.us.us.us.4.i = fadd float %_0.i160.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3246
  %_9.i221.us.us.us.i = lshr i32 %_4.i.i413.us.us.us.i, 23, !dbg !3248
  %_8.i222.us.us.us.i = or disjoint i32 %_9.i221.us.us.us.i, 1258291200, !dbg !3248
  %_7.i223.us.us.us.i = bitcast i32 %_8.i222.us.us.us.i to float, !dbg !3249
  %exponent.i224.us.us.us.i = fadd float %_7.i223.us.us.us.i, 0xC160000FE0000000, !dbg !3251
  %_0.i159.us.us.us.i = fmul float %_0.i179.us.us.us.i, %_0.i144.us.us.us.4.i, !dbg !3252
  %_0.i143.us.us.us.i = fadd float %exponent.i224.us.us.us.i, %_0.i159.us.us.us.i, !dbg !3254
  %_0.i169.us.us.us.i = fmul float %_0.i143.us.us.us.i, 0x4018151820000000, !dbg !3256
  %_3.i.i522.us.us.us.inv.i = fcmp olt float %_0.i169.us.us.us.i, 2.400000e+01, !dbg !3258
  %_0.i.i529.us.us.us.i = select i1 %_3.i.i522.us.us.us.inv.i, float %_0.i169.us.us.us.i, float 2.400000e+01, !dbg !3258
  %_3.i.i439.us.us.us.inv.i = fcmp ogt float %_0.i.i529.us.us.us.i, -1.600000e+02, !dbg !3261
  %_0.i.i446.us.us.us.i = select i1 %_3.i.i439.us.us.us.inv.i, float %_0.i.i529.us.us.us.i, float -1.600000e+02, !dbg !3261
  %_55.i.i.us.us.us.i = load float, ptr %22, align 4, !dbg !3264, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i113.us.us.us.i = fcmp ule float %_55.i.i.us.us.us.i, 0.000000e+00, !dbg !3265
  %_3.i99.us.us.us.i = fcmp oge float %_0.i.i446.us.us.us.i, %_0.i301.us.us.us.i, !dbg !3267
  %_0.i186.us.us.us.i = fsub float %_0.i301.us.us.us.i, %_0.i301.us.us.us.3.i, !dbg !3269
  %_3.i97.us.us.us.i = fcmp oge float %_0.i.i446.us.us.us.i, %_0.i186.us.us.us.i, !dbg !3271
  %..i98.us.us.us.i = sext i1 %_3.i97.us.us.us.i to i32, !dbg !3273
  %_0.i397.us.us.us.i = sext i1 %_3.i99.us.us.us.i to i32, !dbg !3275
  %_0.i391.us.us.us.i = select i1 %_3.i113.us.us.us.i, i32 %_0.i397.us.us.us.i, i32 %..i98.us.us.us.i, !dbg !3275
  %_0.i403.us.us.us.i = xor i32 %..i98.us.us.us.i, -1, !dbg !3277
  %_67.i.i.us.us.us.i = load float, ptr %25, align 4, !dbg !3279, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i111.us.us.us.i = fcmp ogt float %_67.i.i.us.us.us.i, 0.000000e+00, !dbg !3280
  %_0.i396.us.us.us.i = select i1 %_3.i111.us.us.us.i, i32 %_0.i403.us.us.us.i, i32 0, !dbg !3282
  %_0.i395.us.us.us.i = select i1 %_3.i113.us.us.us.i, i32 0, i32 %_0.i396.us.us.us.i, !dbg !3284
  %_0.i390.us.us.us.i = or i32 %_0.i395.us.us.us.i, %_0.i391.us.us.us.i, !dbg !3286
  %_5.i263.us.us.us.i = and i32 %_0.i390.us.us.us.i, 1065353216, !dbg !3288
  %_0.i267.us.us.us.i = bitcast i32 %_5.i263.us.us.us.i to float, !dbg !3290
  %_71.i.i586587.us.us.us.i = load float, ptr %26, align 4, !dbg !3292, !alias.scope !3194, !noalias !3195, !noundef !12
  %_0.i185.us.us.us.i = fadd float %_67.i.i.us.us.us.i, -1.000000e+00, !dbg !3293
  %233 = trunc nsw i32 %_0.i395.us.us.us.i to i1, !dbg !3295
  %_4.i261.v.us.us.us.i = select i1 %233, float %_0.i185.us.us.us.i, float %_67.i.i.us.us.us.i, !dbg !3295
  %234 = trunc nsw i32 %_0.i391.us.us.us.i to i1, !dbg !3297
  %_0.i255.us.us.us.i = select i1 %234, float %_71.i.i586587.us.us.us.i, float %_4.i261.v.us.us.us.i, !dbg !3297
  store float %_0.i255.us.us.us.i, ptr %25, align 4, !dbg !3299, !alias.scope !3165, !noalias !3166
  store i32 %_5.i263.us.us.us.i, ptr %22, align 4, !dbg !3300, !alias.scope !3165, !noalias !3166
  %_0.i184.us.us.us.i = fadd float %_0.i301.us.us.us.1.i, -1.000000e+00, !dbg !3301
  %_0.i183.us.us.us.i = fsub float %_0.i.i446.us.us.us.i, %_0.i301.us.us.us.i, !dbg !3303
  %_0.i168.us.us.us.i = fmul float %_0.i184.us.us.us.i, %_0.i183.us.us.us.i, !dbg !3305
  %235 = fneg float %_0.i301.us.us.us.2.i, !dbg !3307
  %_3.i.i431.inv.us.us.us.i = fcmp ogt float %_0.i168.us.us.us.i, %235, !dbg !3309
  %_4.i.i437.v.us.us.us.i = select i1 %_3.i.i431.inv.us.us.us.i, float %_0.i168.us.us.us.i, float %235, !dbg !3309
  %_3.i.i514.us.us.us.i = fcmp olt float %_4.i.i437.v.us.us.us.i, 0.000000e+00, !dbg !3312
  %236 = fcmp ule float %_0.i267.us.us.us.i, 0.000000e+00, !dbg !3315
  %237 = select i1 %236, i1 %_3.i.i514.us.us.us.i, i1 false, !dbg !3317
  %_0.i248.us.us.us.i = select i1 %237, float %_4.i.i437.v.us.us.us.i, float 0.000000e+00, !dbg !3317
  %_86.i.i.us.us.us.i = load float, ptr %27, align 4, !dbg !3318, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i107.us.us.us.i = fcmp ule float %_0.i248.us.us.us.i, %_86.i.i.us.us.us.i, !dbg !3319
  %_87.i.i589.us.us.us.i = load i32, ptr %_38.i, align 4, !dbg !3321, !alias.scope !3194, !noalias !3195, !noundef !12
  %_88.i.i590.us.us.us.i = load i32, ptr %28, align 4, !dbg !3322, !alias.scope !3194, !noalias !3195, !noundef !12
  %_4.i241.us.us.us.i = select i1 %_3.i107.us.us.us.i, i32 %_88.i.i590.us.us.us.i, i32 %_87.i.i589.us.us.us.i, !dbg !3323
  %_0.i242.us.us.us.i = bitcast i32 %_4.i241.us.us.us.i to float, !dbg !3325
  %_0.i182.us.us.us.i = fsub float %_0.i248.us.us.us.i, %_86.i.i.us.us.us.i, !dbg !3327
  %_4.i153.us.us.us.i = fmul float %_0.i182.us.us.us.i, %_0.i242.us.us.us.i, !dbg !3329
  %_0.i154.us.us.us.i = fadd float %_86.i.i.us.us.us.i, %_4.i153.us.us.us.i, !dbg !3329
  %238 = tail call noundef float @llvm.fabs.f32(float %_0.i154.us.us.us.i), !dbg !3331
  %239 = fcmp uge float %238, 0x3BC79CA100000000, !dbg !3334
  %_0.i226.us.us.us.i = select i1 %239, float %_0.i154.us.us.us.i, float 0.000000e+00, !dbg !3336
  store float %_0.i226.us.us.us.i, ptr %27, align 4, !dbg !3337, !alias.scope !3165, !noalias !3166
  %_0.i167.us.us.us.i = fmul float %_0.i226.us.us.us.i, 0x3FC542A5A0000000, !dbg !3338
  %_3.i.i423.us.us.us.inv.i = fcmp ogt float %_0.i167.us.us.us.i, -1.260000e+02, !dbg !3341
  %_0.i.i430.us.us.us.i = select i1 %_3.i.i423.us.us.us.inv.i, float %_0.i167.us.us.us.i, float -1.260000e+02, !dbg !3341
  %_3.i.i506.us.us.us.inv.i = fcmp olt float %_0.i.i430.us.us.us.i, 1.270000e+02, !dbg !3345
  %_0.i.i513.us.us.us.i = select i1 %_3.i.i506.us.us.us.inv.i, float %_0.i.i430.us.us.us.i, float 1.270000e+02, !dbg !3345
  %240 = tail call noundef float @llvm.floor.f32(float %_0.i.i513.us.us.us.i), !dbg !3348
  %_0.i181.us.us.us.i = fsub float %_0.i.i513.us.us.us.i, %240, !dbg !3352
  %_0.i165.us.us.us.i = fmul float %_0.i181.us.us.us.i, 0x3F5E974FA0000000, !dbg !3354
  %_0.i148.us.us.us.i = fadd float %_0.i165.us.us.us.i, 0x3F82778560000000, !dbg !3359
  %_0.i165.us.us.us.1.i = fmul float %_0.i181.us.us.us.i, %_0.i148.us.us.us.i, !dbg !3354
  %_0.i148.us.us.us.1.i = fadd float %_0.i165.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3359
  %_0.i165.us.us.us.2.i = fmul float %_0.i181.us.us.us.i, %_0.i148.us.us.us.1.i, !dbg !3354
  %_0.i148.us.us.us.2.i = fadd float %_0.i165.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3359
  %_0.i165.us.us.us.3.i = fmul float %_0.i181.us.us.us.i, %_0.i148.us.us.us.2.i, !dbg !3354
  %_0.i148.us.us.us.3.i = fadd float %_0.i165.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3359
  %_0.i163.us.us.us.i = fmul float %_0.i180.us.us.us.i, 0x3F5E974FA0000000, !dbg !3361
  %_0.i146.us.us.us.i = fadd float %_0.i163.us.us.us.i, 0x3F82778560000000, !dbg !3363
  %_0.i163.us.us.us.1.i = fmul float %_0.i180.us.us.us.i, %_0.i146.us.us.us.i, !dbg !3361
  %_0.i146.us.us.us.1.i = fadd float %_0.i163.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3363
  %_0.i163.us.us.us.2.i = fmul float %_0.i180.us.us.us.i, %_0.i146.us.us.us.1.i, !dbg !3361
  %_0.i146.us.us.us.2.i = fadd float %_0.i163.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3363
  %_0.i163.us.us.us.3.i = fmul float %_0.i180.us.us.us.i, %_0.i146.us.us.us.2.i, !dbg !3361
  %_0.i146.us.us.us.3.i = fadd float %_0.i163.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3363
  %_0.i162.us.us.us.i = fmul float %_0.i180.us.us.us.i, %_0.i146.us.us.us.3.i, !dbg !3365
  %_0.i145.us.us.us.i = fadd float %_0.i162.us.us.us.i, 1.000000e+00, !dbg !3367
  %biased.i.us.us.us.i = fadd float %225, 0x4160000FE0000000, !dbg !3369
  %_4.i83.us.us.us.i = bitcast float %biased.i.us.us.us.i to i32, !dbg !3373
  %_3.i84.us.us.us.i = shl i32 %_4.i83.us.us.us.i, 23, !dbg !3377
  %_0.i85.us.us.us.i = bitcast i32 %_3.i84.us.us.us.i to float, !dbg !3378
  %_0.i161.us.us.us.i = fmul float %_0.i145.us.us.us.i, %_0.i85.us.us.us.i, !dbg !3381
  %_3.i93.us.us.us.i = fcmp une float %_0.i230.us.us.us.i, 0.000000e+00, !dbg !3383
  %_3.i121.us.us.us.i = fcmp ule float %_98.i123.i.us.us.us.i, 0.000000e+00, !dbg !3385
  %_0.i392572.not.us.us.us.i = and i1 %_3.i121.us.us.us.i, %_3.i93.us.us.us.i, !dbg !3387
  %_0.i172.us.us.us.i = fmul float %_0.i205.us.us.us.i, %_0.i161.us.us.us.i, !dbg !3387
  %_4.i314.v.us.us.us.i = select i1 %_0.i392572.not.us.us.us.i, float %_0.i172.us.us.us.i, float %_0.i205.us.us.us.i, !dbg !3390
  %_0.i.us.us.us.i = fmul float %_0.i181.us.us.us.i, %_0.i148.us.us.us.3.i, !dbg !3392
  %_0.i147.us.us.us.i = fadd float %_0.i.us.us.us.i, 1.000000e+00, !dbg !3394
  %biased.i86.us.us.us.i = fadd float %240, 0x4160000FE0000000, !dbg !3396
  %_4.i87.us.us.us.i = bitcast float %biased.i86.us.us.us.i to i32, !dbg !3398
  %_3.i88.us.us.us.i = shl i32 %_4.i87.us.us.us.i, 23, !dbg !3400
  %_0.i89.us.us.us.i = bitcast i32 %_3.i88.us.us.us.i to float, !dbg !3401
  %_0.i164.us.us.us.i = fmul float %_0.i147.us.us.us.i, %_0.i89.us.us.us.i, !dbg !3403
  %_3.i90.us.us.us.i = fcmp une float %_0.i226.us.us.us.i, 0.000000e+00, !dbg !3405
  %_98.i.i.us.us.us.i = load float, ptr %29, align 4, !dbg !3407, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i105.us.us.us.i = fcmp ule float %_98.i.i.us.us.us.i, 0.000000e+00, !dbg !3408
  %_0.i389594.not.us.us.us.i = and i1 %_3.i105.us.us.us.i, %_3.i90.us.us.us.i, !dbg !3410
  %_0.i166.us.us.us.i = fmul float %_0.i203.us.us.us.i, %_0.i164.us.us.us.i, !dbg !3410
  %_4.i234.v.us.us.us.i = select i1 %_0.i389594.not.us.us.us.i, float %_0.i166.us.us.us.i, float %_0.i203.us.us.us.i, !dbg !3412
  store float %_4.i314.v.us.us.us.i, ptr %_97.i.us.us.us.i, align 4, !dbg !3414, !alias.scope !3417, !noalias !2719
  store float %_4.i234.v.us.us.us.i, ptr %_115.i.us.us.us.i, align 4, !dbg !3420, !alias.scope !3422, !noalias !2742
  %exitcond1520.not.i = icmp eq i64 %204, %..i, !dbg !3425
  br i1 %exitcond1520.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb30.i.us.us.us.i, !dbg !3428, !llvm.loop !3441

bb8.i81.us.us.us.i:                               ; preds = %bb5.i.preheader.us.us.us.i
  %_34.i.us.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.us.i, !dbg !3429
  br i1 %_34.i.us.us.us.i, label %bb10.i82.us.us.us.i, label %panic5.i.i, !dbg !3429

bb10.i82.us.us.us.i:                              ; preds = %bb8.i81.us.us.us.i
  %241 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.us.i, !dbg !3436
  %left_own.i.us.us.us.i = load float, ptr %241, align 4, !dbg !3436, !alias.scope !2843, !noalias !2865, !noundef !12
  %242 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.us.i, !dbg !3429
  %right_own.i.us.us.us.i = load float, ptr %242, align 4, !dbg !3429, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i, !dbg !2870

bb5.i.preheader.us.us.us.i:                       ; preds = %bb50.i.us.us.us.i
  br i1 %_31.i80.us.us.us.i, label %bb8.i81.us.us.us.i, label %panic4.i.i, !dbg !3436

bb14.i63.preheader.us.us.us.i:                    ; preds = %bb50.i.us.us.us.i
  br i1 %_31.i80.us.us.us.i, label %bb17.i.us.us.us.i, label %panic15.i.i, !dbg !2880

bb23.i.preheader.us.us.us.i:                      ; preds = %bb50.i.us.us.us.i
  br i1 %_31.i80.us.us.us.i, label %bb27.i60.us.us.us.i, label %panic28.i.i, !dbg !2859

bb32.i.us.us.i:                                   ; preds = %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i, %bb30.i.us.us.preheader.i
  %iter.sroa.0.0.i714.us.us.i = phi i64 [ %243, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i ], [ 0, %bb30.i.us.us.preheader.i ]
  %243 = add nuw nsw i64 %iter.sroa.0.0.i714.us.us.i, 1, !dbg !2683
  %_28.i.us.us.i = trunc i64 %iter.sroa.0.0.i714.us.us.i to i32, !dbg !2694
  %now.i.us.us.i = add i32 %base.i.i, %_28.i.us.us.i, !dbg !2695
  %_31.i.us.us.i = and i32 %now.i.us.us.i, %_58.i, !dbg !2698
  %_30.i.us.us.i = zext i32 %_31.i.us.us.i to i64, !dbg !2699
  %_97.i.us.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i714.us.us.i, !dbg !2700
  %_98.not.not.i.us.us.i = icmp ugt i64 %_64.1.i, %_30.i.us.us.i, !dbg !2704
  br i1 %_98.not.not.i.us.us.i, label %bb35.i.us.us.i, label %bb36.i.i, !dbg !2704, !prof !2709

bb35.i.us.us.i:                                   ; preds = %bb32.i.us.us.i
  %_0.i213.us.us.i = load float, ptr %_97.i.us.us.i, align 4, !dbg !2710, !alias.scope !2716, !noalias !2719, !noundef !12
  %_107.i.us.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_30.i.us.us.i, !dbg !2720
  store float %_0.i213.us.us.i, ptr %_107.i.us.us.i, align 4, !dbg !2724, !alias.scope !2727, !noalias !2668
  %_115.i.us.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i714.us.us.i, !dbg !2730
  %_0.i211.us.us.i = load float, ptr %_115.i.us.us.i, align 4, !dbg !2737, !alias.scope !2739, !noalias !2742, !noundef !12
  %_123.i.us.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_30.i.us.us.i, !dbg !2743
  store float %_0.i211.us.us.i, ptr %_123.i.us.us.i, align 4, !dbg !2750, !alias.scope !2752, !noalias !2668
  %exitcond1521.not.i = icmp eq i64 %iter.sroa.0.0.i714.us.us.i, %empty.sroa.6.0.i.i, !dbg !3442
  br i1 %exitcond1521.not.i, label %bb43.i.i, label %bb42.i.us.us.i, !dbg !3442, !prof !180

bb42.i.us.us.i:                                   ; preds = %bb35.i.us.us.i
  %_132.not.not.i.us.us.i = icmp ugt i64 %_65.1.i, %_30.i.us.us.i, !dbg !3440
  br i1 %_132.not.not.i.us.us.i, label %bb44.i.us.us.i, label %bb45.i.i, !dbg !3440, !prof !2709

bb44.i.us.us.i:                                   ; preds = %bb42.i.us.us.i
  %_131.i.us.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.us.i, !dbg !2755
  %_0.i209.us.us.i = load float, ptr %_131.i.us.us.i, align 4, !dbg !2762, !alias.scope !2764, !noalias !2668, !noundef !12
  %_139.i.us.us.i = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_30.i.us.us.i, !dbg !2767
  store float %_0.i209.us.us.i, ptr %_139.i.us.us.i, align 4, !dbg !2774, !alias.scope !2776, !noalias !2668
  %_148.not.not.i.us.us.i = icmp ugt i64 %_67.1.i, %_30.i.us.us.i, !dbg !3437
  br i1 %_148.not.not.i.us.us.i, label %bb48.i.us.us.i, label %bb49.i.i, !dbg !3437, !prof !2709

bb48.i.us.us.i:                                   ; preds = %bb44.i.us.us.i
  %_147.i.us.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.us.i, !dbg !2779
  %_0.i207.us.us.i = load float, ptr %_147.i.us.us.i, align 4, !dbg !2786, !alias.scope !2788, !noalias !2668, !noundef !12
  %_155.i.us.us.i = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_30.i.us.us.i, !dbg !2791
  store float %_0.i207.us.us.i, ptr %_155.i.us.us.i, align 4, !dbg !2798, !alias.scope !2800, !noalias !2668
  %_53.i.us.us.i = sub i32 %now.i.us.us.i, %_59.i, !dbg !2803
  %_52.i.us.us.i = and i32 %_53.i.us.us.i, %_58.i, !dbg !2806
  %_51.i.us.us.i = zext i32 %_52.i.us.us.i to i64, !dbg !2807
  %_156.not.not.i.us.us.i = icmp ugt i64 %_64.1.i, %_51.i.us.us.i, !dbg !2808
  br i1 %_156.not.not.i.us.us.i, label %bb50.i.us.us.i, label %bb51.i.i, !dbg !2808, !prof !2709

bb50.i.us.us.i:                                   ; preds = %bb48.i.us.us.i
  %_163.i.us.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_51.i.us.us.i, !dbg !2813
  %_0.i205.us.us.i = load float, ptr %_163.i.us.us.i, align 4, !dbg !2817, !alias.scope !2819, !noalias !2668, !noundef !12
  %_169.i.us.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_51.i.us.us.i, !dbg !2822
  %_0.i203.us.us.i = load float, ptr %_169.i.us.us.i, align 4, !dbg !2830, !alias.scope !2832, !noalias !2668, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2835), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2841), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2845), !dbg !2838
  %_18.i74.us.us.i = load i32, ptr %_31.i, align 4, !dbg !2847, !alias.scope !2850, !noalias !2851, !noundef !12
  %_17.i.us.us.i = sub i32 %now.i.us.us.i, %_18.i74.us.us.i, !dbg !2853
  %_16.i75.us.us.i = and i32 %_17.i.us.us.i, %_58.i, !dbg !2847
  %_15.i76.us.us.i = zext i32 %_16.i75.us.us.i to i64, !dbg !2847
  %_26.i.us.us.i = load i32, ptr %_33.i, align 4, !dbg !2847, !alias.scope !2856, !noalias !2857, !noundef !12
  %_25.i.us.us.i = sub i32 %now.i.us.us.i, %_26.i.us.us.i, !dbg !2853
  %_24.i.us.us.i = and i32 %_25.i.us.us.i, %_58.i, !dbg !2847
  %_23.i79.us.us.i = zext i32 %_24.i.us.us.i to i64, !dbg !2847
  %_31.i80.us.us.i = icmp samesign ugt i64 %_65.1.i, %_15.i76.us.us.i, !dbg !2847
  switch i8 %_0.sroa.0.0.i551.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.i
    i8 1, label %bb14.i63.preheader.us.us.i
    i8 2, label %bb23.i.preheader.us.us.i
  ], !dbg !2858

bb27.i60.us.us.i:                                 ; preds = %bb23.i.preheader.us.us.i
  %244 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.i, !dbg !2859
  %_79.i.us.us.i = load float, ptr %244, align 4, !dbg !2859, !alias.scope !2843, !noalias !2865, !noundef !12
  %_85.i.us.us.i = icmp samesign ugt i64 %_67.1.i, %_15.i76.us.us.i, !dbg !2866
  br i1 %_85.i.us.us.i, label %bb29.i61.us.us.i, label %panic30.i.i, !dbg !2866

bb29.i61.us.us.i:                                 ; preds = %bb27.i60.us.us.i
  %245 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_15.i76.us.us.i, !dbg !2866
  %_83.i.us.us.i = load float, ptr %245, align 4, !dbg !2866, !alias.scope !2845, !noalias !2867, !noundef !12
  %_87.i.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.i, !dbg !2868
  br i1 %_87.i.us.us.i, label %bb31.i.us.us.i, label %panic32.i.i, !dbg !2868

bb31.i.us.us.i:                                   ; preds = %bb29.i61.us.us.i
  %_89.i.us.us.i = icmp samesign ugt i64 %_65.1.i, %_23.i79.us.us.i, !dbg !2869
  br i1 %_89.i.us.us.i, label %bb33.i62.us.us.i, label %panic34.i.i, !dbg !2869

bb33.i62.us.us.i:                                 ; preds = %bb31.i.us.us.i
  %246 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.i, !dbg !2868
  %_86.i.us.us.i = load float, ptr %246, align 4, !dbg !2868, !alias.scope !2845, !noalias !2867, !noundef !12
  %247 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_23.i79.us.us.i, !dbg !2869
  %_88.i.us.us.i = load float, ptr %247, align 4, !dbg !2869, !alias.scope !2843, !noalias !2865, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i, !dbg !2870

bb17.i.us.us.i:                                   ; preds = %bb14.i63.preheader.us.us.i
  %_59.i.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.i, !dbg !2873
  br i1 %_59.i.us.us.i, label %bb19.i.us.us.i, label %panic17.i.i, !dbg !2873

bb19.i.us.us.i:                                   ; preds = %bb17.i.us.us.i
  %248 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.i, !dbg !2880
  %left_own16.i.us.us.i = load float, ptr %248, align 4, !dbg !2880, !alias.scope !2843, !noalias !2865, !noundef !12
  %249 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.i, !dbg !2873
  %right_own18.i.us.us.i = load float, ptr %249, align 4, !dbg !2873, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i, !dbg !2870

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i: ; preds = %bb10.i82.us.us.i, %bb19.i.us.us.i, %bb33.i62.us.us.i
  %taps.i.sroa.1011534.2.i = phi float [ %right_own.i.us.us.i, %bb10.i82.us.us.i ], [ %left_own16.i.us.us.i, %bb19.i.us.us.i ], [ %_88.i.us.us.i, %bb33.i62.us.us.i ], !dbg !2847
  %taps.i.sroa.681533.2.i = phi float [ %right_own.i.us.us.i, %bb10.i82.us.us.i ], [ %right_own18.i.us.us.i, %bb19.i.us.us.i ], [ %_86.i.us.us.i, %bb33.i62.us.us.i ], !dbg !2847
  %taps.i.sroa.351532.2.i = phi float [ %left_own.i.us.us.i, %bb10.i82.us.us.i ], [ %right_own18.i.us.us.i, %bb19.i.us.us.i ], [ %_83.i.us.us.i, %bb33.i62.us.us.i ], !dbg !2847
  %taps.i.sroa.0.2.i = phi float [ %left_own.i.us.us.i, %bb10.i82.us.us.i ], [ %left_own16.i.us.us.i, %bb19.i.us.us.i ], [ %_79.i.us.us.i, %bb33.i62.us.us.i ], !dbg !2847
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2881), !dbg !2884
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2885), !dbg !2884
  %_12.i36.i.us.us.i = load float, ptr %54, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.i = fcmp ule float %_12.i36.i.us.us.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.i = fcmp une float %_12.i36.i.us.us.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.i = load float, ptr %_10.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.i = load float, ptr %55, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.i = fadd float %_16.i39.i.us.us.i, %_17.i40.i.us.us.i, !dbg !2906
  %_20.i42.i552555.us.us.i = load float, ptr %56, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %250 = select i1 %_3.i95.us.us.i, float %_0.i152.us.us.i, float %_20.i42.i552555.us.us.i, !dbg !2911
  %_0.i381.us.us.i = select i1 %_3.i135.us.us.i, float %_16.i39.i.us.us.i, float %250, !dbg !2914
  store float %_0.i381.us.us.i, ptr %_10.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.i = select i1 %_3.i95.us.us.i, float %_17.i40.i.us.us.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.i, ptr %55, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.i = fadd float %_12.i36.i.us.us.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.i = select i1 %_3.i135.us.us.i, float %_12.i36.i.us.us.i, float %_0.i193.us.us.i, !dbg !2923
  store float %_4.i367.v.us.us.i, ptr %54, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.1.i = load float, ptr %57, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.1.i = fcmp ule float %_12.i36.i.us.us.1.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.1.i = fcmp une float %_12.i36.i.us.us.1.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.1.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.1.i = load float, ptr %58, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.1.i = fadd float %_16.i39.i.us.us.1.i, %_17.i40.i.us.us.1.i, !dbg !2906
  %_20.i42.i552555.us.us.1.i = load float, ptr %59, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %251 = select i1 %_3.i95.us.us.1.i, float %_0.i152.us.us.1.i, float %_20.i42.i552555.us.us.1.i, !dbg !2911
  %_0.i381.us.us.1.i = select i1 %_3.i135.us.us.1.i, float %_16.i39.i.us.us.1.i, float %251, !dbg !2914
  store float %_0.i381.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.1.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.1.i = select i1 %_3.i95.us.us.1.i, float %_17.i40.i.us.us.1.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.1.i, ptr %58, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.1.i = fadd float %_12.i36.i.us.us.1.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.1.i = select i1 %_3.i135.us.us.1.i, float %_12.i36.i.us.us.1.i, float %_0.i193.us.us.1.i, !dbg !2923
  store float %_4.i367.v.us.us.1.i, ptr %57, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.2.i = load float, ptr %60, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.2.i = fcmp ule float %_12.i36.i.us.us.2.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.2.i = fcmp une float %_12.i36.i.us.us.2.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.2.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.2.i = load float, ptr %61, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.2.i = fadd float %_16.i39.i.us.us.2.i, %_17.i40.i.us.us.2.i, !dbg !2906
  %_20.i42.i552555.us.us.2.i = load float, ptr %62, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %252 = select i1 %_3.i95.us.us.2.i, float %_0.i152.us.us.2.i, float %_20.i42.i552555.us.us.2.i, !dbg !2911
  %_0.i381.us.us.2.i = select i1 %_3.i135.us.us.2.i, float %_16.i39.i.us.us.2.i, float %252, !dbg !2914
  store float %_0.i381.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.2.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.2.i = select i1 %_3.i95.us.us.2.i, float %_17.i40.i.us.us.2.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.2.i, ptr %61, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.2.i = fadd float %_12.i36.i.us.us.2.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.2.i = select i1 %_3.i135.us.us.2.i, float %_12.i36.i.us.us.2.i, float %_0.i193.us.us.2.i, !dbg !2923
  store float %_4.i367.v.us.us.2.i, ptr %60, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.us.3.i = load float, ptr %63, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.us.3.i = fcmp ule float %_12.i36.i.us.us.3.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.us.3.i = fcmp une float %_12.i36.i.us.us.3.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.3.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.us.3.i = load float, ptr %64, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.us.3.i = fadd float %_16.i39.i.us.us.3.i, %_17.i40.i.us.us.3.i, !dbg !2906
  %_20.i42.i552555.us.us.3.i = load float, ptr %65, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %253 = select i1 %_3.i95.us.us.3.i, float %_0.i152.us.us.3.i, float %_20.i42.i552555.us.us.3.i, !dbg !2911
  %_0.i381.us.us.3.i = select i1 %_3.i135.us.us.3.i, float %_16.i39.i.us.us.3.i, float %253, !dbg !2914
  store float %_0.i381.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.us.3.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.us.3.i = select i1 %_3.i95.us.us.3.i, float %_17.i40.i.us.us.3.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.us.3.i, ptr %64, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.us.3.i = fadd float %_12.i36.i.us.us.3.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.us.3.i = select i1 %_3.i135.us.us.3.i, float %_12.i36.i.us.us.3.i, float %_0.i193.us.us.3.i, !dbg !2923
  store float %_4.i367.v.us.us.3.i, ptr %63, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %254 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.2.i), !dbg !2926
  %255 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351532.2.i), !dbg !2933
  %_37.i57.i.us.us.i = load float, ptr %15, align 4, !dbg !2936, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i133.us.us.i = fcmp ule float %_37.i57.i.us.us.i, 0.000000e+00, !dbg !2941
  %_3.i.i489.us.us.i = fcmp ule float %254, %255, !dbg !2943
  %_6.i.i491.us.us.i = bitcast float %254 to i32, !dbg !2949
  %_8.i.i493.us.us.i = bitcast float %255 to i32, !dbg !2953
  %_4.i.i496.us.us.i = select i1 %_3.i.i489.us.us.i, i32 %_8.i.i493.us.us.i, i32 %_6.i.i491.us.us.i, !dbg !2955
  %_4.i360.us.us.i = select i1 %_3.i133.us.us.i, i32 %_6.i.i491.us.us.i, i32 %_4.i.i496.us.us.i, !dbg !2956
  %_41.i61.i.us.us.i = load float, ptr %16, align 4, !dbg !2958, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i131.us.us.i = fcmp ule float %_41.i61.i.us.us.i, 0.000000e+00, !dbg !2959
  %_0.i177.us.us.i = fmul float %254, 5.000000e-01, !dbg !2961
  %_0.i176.us.us.i = fmul float %255, 5.000000e-01, !dbg !2964
  %_0.i151.us.us.i = fadd float %_0.i176.us.us.i, %_0.i177.us.us.i, !dbg !2966
  %_6.i348.us.us.i = bitcast float %_0.i151.us.us.i to i32, !dbg !2968
  %_4.i353.us.us.i = select i1 %_3.i131.us.us.i, i32 %_4.i360.us.us.i, i32 %_6.i348.us.us.i, !dbg !2971
  %_0.i354.us.us.i = bitcast i32 %_4.i353.us.us.i to float, !dbg !2972
  %_3.i.i481.us.us.i = fcmp ule float %_0.i354.us.us.i, 0x3E45798EE0000000, !dbg !2975
  %_4.i.i487.us.us.i = select i1 %_3.i.i481.us.us.i, i32 841731191, i32 %_4.i353.us.us.i, !dbg !2978
  %_0.i.i488.us.us.i = bitcast i32 %_4.i.i487.us.us.i to float, !dbg !2980
  %_3.i.i.us.us.i = fcmp ule float %_0.i.i488.us.us.i, 0x3810000000000000, !dbg !2982
  %_4.i.i.us.us.i = select i1 %_3.i.i.us.us.i, i32 8388608, i32 %_4.i.i487.us.us.i, !dbg !2992
  %_5.i214.us.us.i = and i32 %_4.i.i.us.us.i, 8388607, !dbg !2994
  %_4.i215.us.us.i = or disjoint i32 %_5.i214.us.us.i, 1065353216, !dbg !2994
  %significand.i.us.us.i = bitcast i32 %_4.i215.us.us.i to float, !dbg !2999
  %_0.i178.us.us.i = fadd float %significand.i.us.us.i, -1.000000e+00, !dbg !3002
  %_0.i158.us.us.i = fmul float %_0.i178.us.us.i, 0x3F9B17A960000000, !dbg !3005
  %256 = fsub float 0x3FBF9A8440000000, %_0.i158.us.us.i, !dbg !3010
  %_0.i158.us.us.1.i = fmul float %_0.i178.us.us.i, %256, !dbg !3005
  %_0.i142.us.us.1.i = fadd float %_0.i158.us.us.1.i, 0xBFD1E3F400000000, !dbg !3010
  %_0.i158.us.us.2.i = fmul float %_0.i178.us.us.i, %_0.i142.us.us.1.i, !dbg !3005
  %_0.i142.us.us.2.i = fadd float %_0.i158.us.us.2.i, 0x3FDD544F20000000, !dbg !3010
  %_0.i158.us.us.3.i = fmul float %_0.i178.us.us.i, %_0.i142.us.us.2.i, !dbg !3005
  %_0.i142.us.us.3.i = fadd float %_0.i158.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3010
  %_0.i158.us.us.4.i = fmul float %_0.i178.us.us.i, %_0.i142.us.us.3.i, !dbg !3005
  %_0.i142.us.us.4.i = fadd float %_0.i158.us.us.4.i, 0x3FF714B2A0000000, !dbg !3010
  %_9.i.us.us.i = lshr i32 %_4.i.i.us.us.i, 23, !dbg !3012
  %_8.i216.us.us.i = or disjoint i32 %_9.i.us.us.i, 1258291200, !dbg !3012
  %_7.i.us.us.i = bitcast i32 %_8.i216.us.us.i to float, !dbg !3014
  %exponent.i.us.us.i = fadd float %_7.i.us.us.i, 0xC160000FE0000000, !dbg !3016
  %_0.i157.us.us.i = fmul float %_0.i178.us.us.i, %_0.i142.us.us.4.i, !dbg !3017
  %_0.i141.us.us.i = fadd float %exponent.i.us.us.i, %_0.i157.us.us.i, !dbg !3019
  %_0.i175.us.us.i = fmul float %_0.i141.us.us.i, 0x4018151820000000, !dbg !3021
  %_3.i.i538.us.us.inv.i = fcmp olt float %_0.i175.us.us.i, 2.400000e+01, !dbg !3023
  %_0.i.i545.us.us.i = select i1 %_3.i.i538.us.us.inv.i, float %_0.i175.us.us.i, float 2.400000e+01, !dbg !3023
  %_3.i.i473.us.us.inv.i = fcmp ogt float %_0.i.i545.us.us.i, -1.600000e+02, !dbg !3027
  %_0.i.i480.us.us.i = select i1 %_3.i.i473.us.us.inv.i, float %_0.i.i545.us.us.i, float -1.600000e+02, !dbg !3027
  %_55.i78.i.us.us.i = load float, ptr %14, align 4, !dbg !3030, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i129.us.us.i = fcmp ule float %_55.i78.i.us.us.i, 0.000000e+00, !dbg !3032
  %_3.i103.us.us.i = fcmp oge float %_0.i.i480.us.us.i, %_0.i381.us.us.i, !dbg !3034
  %_0.i192.us.us.i = fsub float %_0.i381.us.us.i, %_0.i381.us.us.3.i, !dbg !3038
  %_3.i101.us.us.i = fcmp oge float %_0.i.i480.us.us.i, %_0.i192.us.us.i, !dbg !3041
  %..i102.us.us.i = sext i1 %_3.i101.us.us.i to i32, !dbg !3043
  %_0.i401.us.us.i = sext i1 %_3.i103.us.us.i to i32, !dbg !3046
  %_0.i394.us.us.i = select i1 %_3.i129.us.us.i, i32 %_0.i401.us.us.i, i32 %..i102.us.us.i, !dbg !3046
  %_0.i405.us.us.i = xor i32 %..i102.us.us.i, -1, !dbg !3052
  %_67.i88.i.us.us.i = load float, ptr %17, align 4, !dbg !3056, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i127.us.us.i = fcmp ogt float %_67.i88.i.us.us.i, 0.000000e+00, !dbg !3057
  %_0.i400.us.us.i = select i1 %_3.i127.us.us.i, i32 %_0.i405.us.us.i, i32 0, !dbg !3059
  %_0.i399.us.us.i = select i1 %_3.i129.us.us.i, i32 0, i32 %_0.i400.us.us.i, !dbg !3062
  %_0.i393.us.us.i = or i32 %_0.i399.us.us.i, %_0.i394.us.us.i, !dbg !3064
  %_5.i343.us.us.i = and i32 %_0.i393.us.us.i, 1065353216, !dbg !3067
  %_0.i347.us.us.i = bitcast i32 %_5.i343.us.us.i to float, !dbg !3069
  %_71.i94.i564565.us.us.i = load float, ptr %18, align 4, !dbg !3071, !alias.scope !2939, !noalias !2940, !noundef !12
  %_0.i191.us.us.i = fadd float %_67.i88.i.us.us.i, -1.000000e+00, !dbg !3073
  %257 = trunc nsw i32 %_0.i399.us.us.i to i1, !dbg !3075
  %_4.i341.v.us.us.i = select i1 %257, float %_0.i191.us.us.i, float %_67.i88.i.us.us.i, !dbg !3075
  %258 = trunc nsw i32 %_0.i394.us.us.i to i1, !dbg !3077
  %_0.i335.us.us.i = select i1 %258, float %_71.i94.i564565.us.us.i, float %_4.i341.v.us.us.i, !dbg !3077
  store float %_0.i335.us.us.i, ptr %17, align 4, !dbg !3079, !alias.scope !2894, !noalias !2895
  store i32 %_5.i343.us.us.i, ptr %14, align 4, !dbg !3080, !alias.scope !2894, !noalias !2895
  %_0.i190.us.us.i = fadd float %_0.i381.us.us.1.i, -1.000000e+00, !dbg !3081
  %_0.i189.us.us.i = fsub float %_0.i.i480.us.us.i, %_0.i381.us.us.i, !dbg !3083
  %_0.i174.us.us.i = fmul float %_0.i190.us.us.i, %_0.i189.us.us.i, !dbg !3085
  %259 = fneg float %_0.i381.us.us.2.i, !dbg !3087
  %_3.i.i464.inv.us.us.i = fcmp ogt float %_0.i174.us.us.i, %259, !dbg !3090
  %_4.i.i471.v.us.us.i = select i1 %_3.i.i464.inv.us.us.i, float %_0.i174.us.us.i, float %259, !dbg !3090
  %_3.i.i530.us.us.i = fcmp olt float %_4.i.i471.v.us.us.i, 0.000000e+00, !dbg !3093
  %260 = fcmp ule float %_0.i347.us.us.i, 0.000000e+00, !dbg !3097
  %261 = select i1 %260, i1 %_3.i.i530.us.us.i, i1 false, !dbg !3100
  %_0.i328.us.us.i = select i1 %261, float %_4.i.i471.v.us.us.i, float 0.000000e+00, !dbg !3100
  %_86.i107.i.us.us.i = load float, ptr %19, align 4, !dbg !3101, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i123.us.us.i = fcmp ule float %_0.i328.us.us.i, %_86.i107.i.us.us.i, !dbg !3103
  %_87.i109.i567.us.us.i = load i32, ptr %4, align 4, !dbg !3105, !alias.scope !2939, !noalias !2940, !noundef !12
  %_88.i110.i568.us.us.i = load i32, ptr %20, align 4, !dbg !3106, !alias.scope !2939, !noalias !2940, !noundef !12
  %_4.i321.us.us.i = select i1 %_3.i123.us.us.i, i32 %_88.i110.i568.us.us.i, i32 %_87.i109.i567.us.us.i, !dbg !3107
  %_0.i322.us.us.i = bitcast i32 %_4.i321.us.us.i to float, !dbg !3109
  %_0.i188.us.us.i = fsub float %_0.i328.us.us.i, %_86.i107.i.us.us.i, !dbg !3111
  %_4.i155.us.us.i = fmul float %_0.i188.us.us.i, %_0.i322.us.us.i, !dbg !3114
  %_0.i156.us.us.i = fadd float %_86.i107.i.us.us.i, %_4.i155.us.us.i, !dbg !3114
  %262 = tail call noundef float @llvm.fabs.f32(float %_0.i156.us.us.i), !dbg !3117
  %263 = fcmp uge float %262, 0x3BC79CA100000000, !dbg !3121
  %_0.i230.us.us.i = select i1 %263, float %_0.i156.us.us.i, float 0.000000e+00, !dbg !3124
  store float %_0.i230.us.us.i, ptr %19, align 4, !dbg !3125, !alias.scope !2894, !noalias !2895
  %_0.i173.us.us.i = fmul float %_0.i230.us.us.i, 0x3FC542A5A0000000, !dbg !3127
  %_3.i.i415.us.us.inv.i = fcmp ogt float %_0.i173.us.us.i, -1.260000e+02, !dbg !3131
  %_0.i.i422.us.us.i = select i1 %_3.i.i415.us.us.inv.i, float %_0.i173.us.us.i, float -1.260000e+02, !dbg !3131
  %_3.i.i498.us.us.inv.i = fcmp olt float %_0.i.i422.us.us.i, 1.270000e+02, !dbg !3136
  %_0.i.i505.us.us.i = select i1 %_3.i.i498.us.us.inv.i, float %_0.i.i422.us.us.i, float 1.270000e+02, !dbg !3136
  %264 = tail call noundef float @llvm.floor.f32(float %_0.i.i505.us.us.i), !dbg !3139
  %_0.i180.us.us.i = fsub float %_0.i.i505.us.us.i, %264, !dbg !3151
  %_98.i123.i.us.us.i = load float, ptr %21, align 4, !dbg !3154, !alias.scope !2939, !noalias !2940, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3156), !dbg !3159
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3161), !dbg !3159
  %_12.i.i.us.us.i = load float, ptr %66, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.i = fcmp ule float %_12.i.i.us.us.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.i = fcmp une float %_12.i.i.us.us.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.i = load float, ptr %67, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.i = fadd float %_16.i.i.us.us.i, %_17.i.i.us.us.i, !dbg !3173
  %_20.i.i574577.us.us.i = load float, ptr %68, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %265 = select i1 %_3.i91.us.us.i, float %_0.i150.us.us.i, float %_20.i.i574577.us.us.i, !dbg !3176
  %_0.i301.us.us.i = select i1 %_3.i119.us.us.i, float %_16.i.i.us.us.i, float %265, !dbg !3178
  store float %_0.i301.us.us.i, ptr %data.i.i.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.i = select i1 %_3.i91.us.us.i, float %_17.i.i.us.us.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.i, ptr %67, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.i = fadd float %_12.i.i.us.us.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.i = select i1 %_3.i119.us.us.i, float %_12.i.i.us.us.i, float %_0.i187.us.us.i, !dbg !3186
  store float %_4.i287.v.us.us.i, ptr %66, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.1.i = load float, ptr %69, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.1.i = fcmp ule float %_12.i.i.us.us.1.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.1.i = fcmp une float %_12.i.i.us.us.1.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.1.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.1.i = load float, ptr %70, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.1.i = fadd float %_16.i.i.us.us.1.i, %_17.i.i.us.us.1.i, !dbg !3173
  %_20.i.i574577.us.us.1.i = load float, ptr %71, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %266 = select i1 %_3.i91.us.us.1.i, float %_0.i150.us.us.1.i, float %_20.i.i574577.us.us.1.i, !dbg !3176
  %_0.i301.us.us.1.i = select i1 %_3.i119.us.us.1.i, float %_16.i.i.us.us.1.i, float %266, !dbg !3178
  store float %_0.i301.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.1.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.1.i = select i1 %_3.i91.us.us.1.i, float %_17.i.i.us.us.1.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.1.i, ptr %70, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.1.i = fadd float %_12.i.i.us.us.1.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.1.i = select i1 %_3.i119.us.us.1.i, float %_12.i.i.us.us.1.i, float %_0.i187.us.us.1.i, !dbg !3186
  store float %_4.i287.v.us.us.1.i, ptr %69, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.2.i = load float, ptr %72, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.2.i = fcmp ule float %_12.i.i.us.us.2.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.2.i = fcmp une float %_12.i.i.us.us.2.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.2.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.2.i = load float, ptr %73, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.2.i = fadd float %_16.i.i.us.us.2.i, %_17.i.i.us.us.2.i, !dbg !3173
  %_20.i.i574577.us.us.2.i = load float, ptr %74, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %267 = select i1 %_3.i91.us.us.2.i, float %_0.i150.us.us.2.i, float %_20.i.i574577.us.us.2.i, !dbg !3176
  %_0.i301.us.us.2.i = select i1 %_3.i119.us.us.2.i, float %_16.i.i.us.us.2.i, float %267, !dbg !3178
  store float %_0.i301.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.2.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.2.i = select i1 %_3.i91.us.us.2.i, float %_17.i.i.us.us.2.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.2.i, ptr %73, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.2.i = fadd float %_12.i.i.us.us.2.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.2.i = select i1 %_3.i119.us.us.2.i, float %_12.i.i.us.us.2.i, float %_0.i187.us.us.2.i, !dbg !3186
  store float %_4.i287.v.us.us.2.i, ptr %72, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.us.3.i = load float, ptr %75, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.us.3.i = fcmp ule float %_12.i.i.us.us.3.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.us.3.i = fcmp une float %_12.i.i.us.us.3.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.us.3.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.us.3.i = load float, ptr %76, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.us.3.i = fadd float %_16.i.i.us.us.3.i, %_17.i.i.us.us.3.i, !dbg !3173
  %_20.i.i574577.us.us.3.i = load float, ptr %77, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %268 = select i1 %_3.i91.us.us.3.i, float %_0.i150.us.us.3.i, float %_20.i.i574577.us.us.3.i, !dbg !3176
  %_0.i301.us.us.3.i = select i1 %_3.i119.us.us.3.i, float %_16.i.i.us.us.3.i, float %268, !dbg !3178
  store float %_0.i301.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i.i.us.us.3.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.us.3.i = select i1 %_3.i91.us.us.3.i, float %_17.i.i.us.us.3.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.us.3.i, ptr %76, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.us.3.i = fadd float %_12.i.i.us.us.3.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.us.3.i = select i1 %_3.i119.us.us.3.i, float %_12.i.i.us.us.3.i, float %_0.i187.us.us.3.i, !dbg !3186
  store float %_4.i287.v.us.us.3.i, ptr %75, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %269 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681533.2.i), !dbg !3189
  %270 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011534.2.i), !dbg !3191
  %_37.i.i.us.us.i = load float, ptr %23, align 4, !dbg !3193, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i117.us.us.i = fcmp ule float %_37.i.i.us.us.i, 0.000000e+00, !dbg !3196
  %_3.i.i455.us.us.i = fcmp ule float %269, %270, !dbg !3198
  %_6.i.i457.us.us.i = bitcast float %269 to i32, !dbg !3201
  %_8.i.i459.us.us.i = bitcast float %270 to i32, !dbg !3204
  %_4.i.i462.us.us.i = select i1 %_3.i.i455.us.us.i, i32 %_8.i.i459.us.us.i, i32 %_6.i.i457.us.us.i, !dbg !3206
  %_4.i280.us.us.i = select i1 %_3.i117.us.us.i, i32 %_6.i.i457.us.us.i, i32 %_4.i.i462.us.us.i, !dbg !3207
  %_41.i.i.us.us.i = load float, ptr %24, align 4, !dbg !3209, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i115.us.us.i = fcmp ule float %_41.i.i.us.us.i, 0.000000e+00, !dbg !3210
  %_0.i171.us.us.i = fmul float %269, 5.000000e-01, !dbg !3212
  %_0.i170.us.us.i = fmul float %270, 5.000000e-01, !dbg !3214
  %_0.i149.us.us.i = fadd float %_0.i170.us.us.i, %_0.i171.us.us.i, !dbg !3216
  %_6.i268.us.us.i = bitcast float %_0.i149.us.us.i to i32, !dbg !3218
  %_4.i273.us.us.i = select i1 %_3.i115.us.us.i, i32 %_4.i280.us.us.i, i32 %_6.i268.us.us.i, !dbg !3221
  %_0.i274.us.us.i = bitcast i32 %_4.i273.us.us.i to float, !dbg !3222
  %_3.i.i447.us.us.i = fcmp ule float %_0.i274.us.us.i, 0x3E45798EE0000000, !dbg !3224
  %_4.i.i453.us.us.i = select i1 %_3.i.i447.us.us.i, i32 841731191, i32 %_4.i273.us.us.i, !dbg !3227
  %_0.i.i454.us.us.i = bitcast i32 %_4.i.i453.us.us.i to float, !dbg !3229
  %_3.i.i407.us.us.i = fcmp ule float %_0.i.i454.us.us.i, 0x3810000000000000, !dbg !3231
  %_4.i.i413.us.us.i = select i1 %_3.i.i407.us.us.i, i32 8388608, i32 %_4.i.i453.us.us.i, !dbg !3236
  %_5.i218.us.us.i = and i32 %_4.i.i413.us.us.i, 8388607, !dbg !3238
  %_4.i219.us.us.i = or disjoint i32 %_5.i218.us.us.i, 1065353216, !dbg !3238
  %significand.i220.us.us.i = bitcast i32 %_4.i219.us.us.i to float, !dbg !3240
  %_0.i179.us.us.i = fadd float %significand.i220.us.us.i, -1.000000e+00, !dbg !3242
  %_0.i160.us.us.i = fmul float %_0.i179.us.us.i, 0x3F9B17A960000000, !dbg !3244
  %271 = fsub float 0x3FBF9A8440000000, %_0.i160.us.us.i, !dbg !3246
  %_0.i160.us.us.1.i = fmul float %_0.i179.us.us.i, %271, !dbg !3244
  %_0.i144.us.us.1.i = fadd float %_0.i160.us.us.1.i, 0xBFD1E3F400000000, !dbg !3246
  %_0.i160.us.us.2.i = fmul float %_0.i179.us.us.i, %_0.i144.us.us.1.i, !dbg !3244
  %_0.i144.us.us.2.i = fadd float %_0.i160.us.us.2.i, 0x3FDD544F20000000, !dbg !3246
  %_0.i160.us.us.3.i = fmul float %_0.i179.us.us.i, %_0.i144.us.us.2.i, !dbg !3244
  %_0.i144.us.us.3.i = fadd float %_0.i160.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3246
  %_0.i160.us.us.4.i = fmul float %_0.i179.us.us.i, %_0.i144.us.us.3.i, !dbg !3244
  %_0.i144.us.us.4.i = fadd float %_0.i160.us.us.4.i, 0x3FF714B2A0000000, !dbg !3246
  %_9.i221.us.us.i = lshr i32 %_4.i.i413.us.us.i, 23, !dbg !3248
  %_8.i222.us.us.i = or disjoint i32 %_9.i221.us.us.i, 1258291200, !dbg !3248
  %_7.i223.us.us.i = bitcast i32 %_8.i222.us.us.i to float, !dbg !3249
  %exponent.i224.us.us.i = fadd float %_7.i223.us.us.i, 0xC160000FE0000000, !dbg !3251
  %_0.i159.us.us.i = fmul float %_0.i179.us.us.i, %_0.i144.us.us.4.i, !dbg !3252
  %_0.i143.us.us.i = fadd float %exponent.i224.us.us.i, %_0.i159.us.us.i, !dbg !3254
  %_0.i169.us.us.i = fmul float %_0.i143.us.us.i, 0x4018151820000000, !dbg !3256
  %_3.i.i522.us.us.inv.i = fcmp olt float %_0.i169.us.us.i, 2.400000e+01, !dbg !3258
  %_0.i.i529.us.us.i = select i1 %_3.i.i522.us.us.inv.i, float %_0.i169.us.us.i, float 2.400000e+01, !dbg !3258
  %_3.i.i439.us.us.inv.i = fcmp ogt float %_0.i.i529.us.us.i, -1.600000e+02, !dbg !3261
  %_0.i.i446.us.us.i = select i1 %_3.i.i439.us.us.inv.i, float %_0.i.i529.us.us.i, float -1.600000e+02, !dbg !3261
  %_55.i.i.us.us.i = load float, ptr %22, align 4, !dbg !3264, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i113.us.us.i = fcmp ule float %_55.i.i.us.us.i, 0.000000e+00, !dbg !3265
  %_3.i99.us.us.i = fcmp oge float %_0.i.i446.us.us.i, %_0.i301.us.us.i, !dbg !3267
  %_0.i186.us.us.i = fsub float %_0.i301.us.us.i, %_0.i301.us.us.3.i, !dbg !3269
  %_3.i97.us.us.i = fcmp oge float %_0.i.i446.us.us.i, %_0.i186.us.us.i, !dbg !3271
  %..i98.us.us.i = sext i1 %_3.i97.us.us.i to i32, !dbg !3273
  %_0.i397.us.us.i = sext i1 %_3.i99.us.us.i to i32, !dbg !3275
  %_0.i391.us.us.i = select i1 %_3.i113.us.us.i, i32 %_0.i397.us.us.i, i32 %..i98.us.us.i, !dbg !3275
  %_0.i403.us.us.i = xor i32 %..i98.us.us.i, -1, !dbg !3277
  %_67.i.i.us.us.i = load float, ptr %25, align 4, !dbg !3279, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i111.us.us.i = fcmp ogt float %_67.i.i.us.us.i, 0.000000e+00, !dbg !3280
  %_0.i396.us.us.i = select i1 %_3.i111.us.us.i, i32 %_0.i403.us.us.i, i32 0, !dbg !3282
  %_0.i395.us.us.i = select i1 %_3.i113.us.us.i, i32 0, i32 %_0.i396.us.us.i, !dbg !3284
  %_0.i390.us.us.i = or i32 %_0.i395.us.us.i, %_0.i391.us.us.i, !dbg !3286
  %_5.i263.us.us.i = and i32 %_0.i390.us.us.i, 1065353216, !dbg !3288
  %_0.i267.us.us.i = bitcast i32 %_5.i263.us.us.i to float, !dbg !3290
  %_71.i.i586587.us.us.i = load float, ptr %26, align 4, !dbg !3292, !alias.scope !3194, !noalias !3195, !noundef !12
  %_0.i185.us.us.i = fadd float %_67.i.i.us.us.i, -1.000000e+00, !dbg !3293
  %272 = trunc nsw i32 %_0.i395.us.us.i to i1, !dbg !3295
  %_4.i261.v.us.us.i = select i1 %272, float %_0.i185.us.us.i, float %_67.i.i.us.us.i, !dbg !3295
  %273 = trunc nsw i32 %_0.i391.us.us.i to i1, !dbg !3297
  %_0.i255.us.us.i = select i1 %273, float %_71.i.i586587.us.us.i, float %_4.i261.v.us.us.i, !dbg !3297
  store float %_0.i255.us.us.i, ptr %25, align 4, !dbg !3299, !alias.scope !3165, !noalias !3166
  store i32 %_5.i263.us.us.i, ptr %22, align 4, !dbg !3300, !alias.scope !3165, !noalias !3166
  %_0.i184.us.us.i = fadd float %_0.i301.us.us.1.i, -1.000000e+00, !dbg !3301
  %_0.i183.us.us.i = fsub float %_0.i.i446.us.us.i, %_0.i301.us.us.i, !dbg !3303
  %_0.i168.us.us.i = fmul float %_0.i184.us.us.i, %_0.i183.us.us.i, !dbg !3305
  %274 = fneg float %_0.i301.us.us.2.i, !dbg !3307
  %_3.i.i431.inv.us.us.i = fcmp ogt float %_0.i168.us.us.i, %274, !dbg !3309
  %_4.i.i437.v.us.us.i = select i1 %_3.i.i431.inv.us.us.i, float %_0.i168.us.us.i, float %274, !dbg !3309
  %_3.i.i514.us.us.i = fcmp olt float %_4.i.i437.v.us.us.i, 0.000000e+00, !dbg !3312
  %275 = fcmp ule float %_0.i267.us.us.i, 0.000000e+00, !dbg !3315
  %276 = select i1 %275, i1 %_3.i.i514.us.us.i, i1 false, !dbg !3317
  %_0.i248.us.us.i = select i1 %276, float %_4.i.i437.v.us.us.i, float 0.000000e+00, !dbg !3317
  %_86.i.i.us.us.i = load float, ptr %27, align 4, !dbg !3318, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i107.us.us.i = fcmp ule float %_0.i248.us.us.i, %_86.i.i.us.us.i, !dbg !3319
  %_87.i.i589.us.us.i = load i32, ptr %_38.i, align 4, !dbg !3321, !alias.scope !3194, !noalias !3195, !noundef !12
  %_88.i.i590.us.us.i = load i32, ptr %28, align 4, !dbg !3322, !alias.scope !3194, !noalias !3195, !noundef !12
  %_4.i241.us.us.i = select i1 %_3.i107.us.us.i, i32 %_88.i.i590.us.us.i, i32 %_87.i.i589.us.us.i, !dbg !3323
  %_0.i242.us.us.i = bitcast i32 %_4.i241.us.us.i to float, !dbg !3325
  %_0.i182.us.us.i = fsub float %_0.i248.us.us.i, %_86.i.i.us.us.i, !dbg !3327
  %_4.i153.us.us.i = fmul float %_0.i182.us.us.i, %_0.i242.us.us.i, !dbg !3329
  %_0.i154.us.us.i = fadd float %_86.i.i.us.us.i, %_4.i153.us.us.i, !dbg !3329
  %277 = tail call noundef float @llvm.fabs.f32(float %_0.i154.us.us.i), !dbg !3331
  %278 = fcmp uge float %277, 0x3BC79CA100000000, !dbg !3334
  %_0.i226.us.us.i = select i1 %278, float %_0.i154.us.us.i, float 0.000000e+00, !dbg !3336
  store float %_0.i226.us.us.i, ptr %27, align 4, !dbg !3337, !alias.scope !3165, !noalias !3166
  %_0.i167.us.us.i = fmul float %_0.i226.us.us.i, 0x3FC542A5A0000000, !dbg !3338
  %_3.i.i423.us.us.inv.i = fcmp ogt float %_0.i167.us.us.i, -1.260000e+02, !dbg !3341
  %_0.i.i430.us.us.i = select i1 %_3.i.i423.us.us.inv.i, float %_0.i167.us.us.i, float -1.260000e+02, !dbg !3341
  %_3.i.i506.us.us.inv.i = fcmp olt float %_0.i.i430.us.us.i, 1.270000e+02, !dbg !3345
  %_0.i.i513.us.us.i = select i1 %_3.i.i506.us.us.inv.i, float %_0.i.i430.us.us.i, float 1.270000e+02, !dbg !3345
  %279 = tail call noundef float @llvm.floor.f32(float %_0.i.i513.us.us.i), !dbg !3348
  %_0.i181.us.us.i = fsub float %_0.i.i513.us.us.i, %279, !dbg !3352
  %_0.i165.us.us.i = fmul float %_0.i181.us.us.i, 0x3F5E974FA0000000, !dbg !3354
  %_0.i148.us.us.i = fadd float %_0.i165.us.us.i, 0x3F82778560000000, !dbg !3359
  %_0.i165.us.us.1.i = fmul float %_0.i181.us.us.i, %_0.i148.us.us.i, !dbg !3354
  %_0.i148.us.us.1.i = fadd float %_0.i165.us.us.1.i, 0x3FAC91CE60000000, !dbg !3359
  %_0.i165.us.us.2.i = fmul float %_0.i181.us.us.i, %_0.i148.us.us.1.i, !dbg !3354
  %_0.i148.us.us.2.i = fadd float %_0.i165.us.us.2.i, 0x3FCEBDB560000000, !dbg !3359
  %_0.i165.us.us.3.i = fmul float %_0.i181.us.us.i, %_0.i148.us.us.2.i, !dbg !3354
  %_0.i148.us.us.3.i = fadd float %_0.i165.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3359
  %_0.i163.us.us.i = fmul float %_0.i180.us.us.i, 0x3F5E974FA0000000, !dbg !3361
  %_0.i146.us.us.i = fadd float %_0.i163.us.us.i, 0x3F82778560000000, !dbg !3363
  %_0.i163.us.us.1.i = fmul float %_0.i180.us.us.i, %_0.i146.us.us.i, !dbg !3361
  %_0.i146.us.us.1.i = fadd float %_0.i163.us.us.1.i, 0x3FAC91CE60000000, !dbg !3363
  %_0.i163.us.us.2.i = fmul float %_0.i180.us.us.i, %_0.i146.us.us.1.i, !dbg !3361
  %_0.i146.us.us.2.i = fadd float %_0.i163.us.us.2.i, 0x3FCEBDB560000000, !dbg !3363
  %_0.i163.us.us.3.i = fmul float %_0.i180.us.us.i, %_0.i146.us.us.2.i, !dbg !3361
  %_0.i146.us.us.3.i = fadd float %_0.i163.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3363
  %_0.i162.us.us.i = fmul float %_0.i180.us.us.i, %_0.i146.us.us.3.i, !dbg !3365
  %_0.i145.us.us.i = fadd float %_0.i162.us.us.i, 1.000000e+00, !dbg !3367
  %biased.i.us.us.i = fadd float %264, 0x4160000FE0000000, !dbg !3369
  %_4.i83.us.us.i = bitcast float %biased.i.us.us.i to i32, !dbg !3373
  %_3.i84.us.us.i = shl i32 %_4.i83.us.us.i, 23, !dbg !3377
  %_0.i85.us.us.i = bitcast i32 %_3.i84.us.us.i to float, !dbg !3378
  %_0.i161.us.us.i = fmul float %_0.i145.us.us.i, %_0.i85.us.us.i, !dbg !3381
  %_3.i93.us.us.i = fcmp une float %_0.i230.us.us.i, 0.000000e+00, !dbg !3383
  %_3.i121.us.us.i = fcmp ule float %_98.i123.i.us.us.i, 0.000000e+00, !dbg !3385
  %_0.i392572.not.us.us.i = and i1 %_3.i121.us.us.i, %_3.i93.us.us.i, !dbg !3387
  %_0.i172.us.us.i = fmul float %_0.i205.us.us.i, %_0.i161.us.us.i, !dbg !3387
  %_4.i314.v.us.us.i = select i1 %_0.i392572.not.us.us.i, float %_0.i172.us.us.i, float %_0.i205.us.us.i, !dbg !3390
  %_0.i.us.us.i = fmul float %_0.i181.us.us.i, %_0.i148.us.us.3.i, !dbg !3392
  %_0.i147.us.us.i = fadd float %_0.i.us.us.i, 1.000000e+00, !dbg !3394
  %biased.i86.us.us.i = fadd float %279, 0x4160000FE0000000, !dbg !3396
  %_4.i87.us.us.i = bitcast float %biased.i86.us.us.i to i32, !dbg !3398
  %_3.i88.us.us.i = shl i32 %_4.i87.us.us.i, 23, !dbg !3400
  %_0.i89.us.us.i = bitcast i32 %_3.i88.us.us.i to float, !dbg !3401
  %_0.i164.us.us.i = fmul float %_0.i147.us.us.i, %_0.i89.us.us.i, !dbg !3403
  %_3.i90.us.us.i = fcmp une float %_0.i226.us.us.i, 0.000000e+00, !dbg !3405
  %_98.i.i.us.us.i = load float, ptr %29, align 4, !dbg !3407, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i105.us.us.i = fcmp ule float %_98.i.i.us.us.i, 0.000000e+00, !dbg !3408
  %_0.i389594.not.us.us.i = and i1 %_3.i105.us.us.i, %_3.i90.us.us.i, !dbg !3410
  %_0.i166.us.us.i = fmul float %_0.i203.us.us.i, %_0.i164.us.us.i, !dbg !3410
  %_4.i234.v.us.us.i = select i1 %_0.i389594.not.us.us.i, float %_0.i166.us.us.i, float %_0.i203.us.us.i, !dbg !3412
  store float %_4.i314.v.us.us.i, ptr %_97.i.us.us.i, align 4, !dbg !3414, !alias.scope !3417, !noalias !2719
  store float %_4.i234.v.us.us.i, ptr %_115.i.us.us.i, align 4, !dbg !3420, !alias.scope !3422, !noalias !2742
  %exitcond1523.not.i = icmp eq i64 %243, %..i, !dbg !3425
  br i1 %exitcond1523.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb32.i.us.us.i, !dbg !3428, !llvm.loop !3443

bb8.i81.us.us.i:                                  ; preds = %bb5.i.preheader.us.us.i
  %_34.i.us.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.us.i, !dbg !3429
  br i1 %_34.i.us.us.i, label %bb10.i82.us.us.i, label %panic5.i.i, !dbg !3429

bb10.i82.us.us.i:                                 ; preds = %bb8.i81.us.us.i
  %280 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.us.i, !dbg !3436
  %left_own.i.us.us.i = load float, ptr %280, align 4, !dbg !3436, !alias.scope !2843, !noalias !2865, !noundef !12
  %281 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.us.i, !dbg !3429
  %right_own.i.us.us.i = load float, ptr %281, align 4, !dbg !3429, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i, !dbg !2870

bb5.i.preheader.us.us.i:                          ; preds = %bb50.i.us.us.i
  br i1 %_31.i80.us.us.i, label %bb8.i81.us.us.i, label %panic4.i.i, !dbg !3436

bb14.i63.preheader.us.us.i:                       ; preds = %bb50.i.us.us.i
  br i1 %_31.i80.us.us.i, label %bb17.i.us.us.i, label %panic15.i.i, !dbg !2880

bb23.i.preheader.us.us.i:                         ; preds = %bb50.i.us.us.i
  br i1 %_31.i80.us.us.i, label %bb27.i60.us.us.i, label %panic28.i.i, !dbg !2859

bb30.i.us.i:                                      ; preds = %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i, %bb30.i.us.preheader.i
  %iter.sroa.0.0.i714.us.i = phi i64 [ %282, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i ], [ 0, %bb30.i.us.preheader.i ]
  %282 = add nuw nsw i64 %iter.sroa.0.0.i714.us.i, 1, !dbg !2683
  %_28.i.us.i = trunc i64 %iter.sroa.0.0.i714.us.i to i32, !dbg !2694
  %now.i.us.i = add i32 %base.i.i, %_28.i.us.i, !dbg !2695
  %_31.i.us.i = and i32 %now.i.us.i, %_58.i, !dbg !2698
  %_30.i.us.i = zext i32 %_31.i.us.i to i64, !dbg !2699
  %exitcond1524.not.i = icmp eq i64 %iter.sroa.0.0.i714.us.i, %..i, !dbg !2671
  br i1 %exitcond1524.not.i, label %bb33.i.i, label %bb32.i.us.i, !dbg !2671, !prof !180

bb32.i.us.i:                                      ; preds = %bb30.i.us.i
  %_97.i.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i714.us.i, !dbg !2700
  %_98.not.not.i.us.i = icmp ugt i64 %_64.1.i, %_30.i.us.i, !dbg !2704
  br i1 %_98.not.not.i.us.i, label %bb35.i.us.i, label %bb36.i.i, !dbg !2704, !prof !2709

bb35.i.us.i:                                      ; preds = %bb32.i.us.i
  %_0.i213.us.i = load float, ptr %_97.i.us.i, align 4, !dbg !2710, !alias.scope !2716, !noalias !2719, !noundef !12
  %_107.i.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_30.i.us.i, !dbg !2720
  store float %_0.i213.us.i, ptr %_107.i.us.i, align 4, !dbg !2724, !alias.scope !2727, !noalias !2668
  %_115.i.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i714.us.i, !dbg !2730
  %_116.not.not.i.us.i = icmp ugt i64 %_66.1.i, %_30.i.us.i, !dbg !3444
  br i1 %_116.not.not.i.us.i, label %bb40.i.us.i, label %bb41.i.i, !dbg !3444, !prof !2709

bb40.i.us.i:                                      ; preds = %bb35.i.us.i
  %_0.i211.us.i = load float, ptr %_115.i.us.i, align 4, !dbg !2737, !alias.scope !2739, !noalias !2742, !noundef !12
  %_123.i.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_30.i.us.i, !dbg !2743
  store float %_0.i211.us.i, ptr %_123.i.us.i, align 4, !dbg !2750, !alias.scope !2752, !noalias !2668
  %exitcond1525.not.i = icmp eq i64 %iter.sroa.0.0.i714.us.i, %empty.sroa.6.0.i.i, !dbg !3442
  br i1 %exitcond1525.not.i, label %bb43.i.i, label %bb42.i.us.i, !dbg !3442, !prof !180

bb42.i.us.i:                                      ; preds = %bb40.i.us.i
  %_132.not.not.i.us.i = icmp ugt i64 %_65.1.i, %_30.i.us.i, !dbg !3440
  br i1 %_132.not.not.i.us.i, label %bb44.i.us.i, label %bb45.i.i, !dbg !3440, !prof !2709

bb44.i.us.i:                                      ; preds = %bb42.i.us.i
  %_131.i.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.i, !dbg !2755
  %_0.i209.us.i = load float, ptr %_131.i.us.i, align 4, !dbg !2762, !alias.scope !2764, !noalias !2668, !noundef !12
  %_139.i.us.i = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_30.i.us.i, !dbg !2767
  store float %_0.i209.us.i, ptr %_139.i.us.i, align 4, !dbg !2774, !alias.scope !2776, !noalias !2668
  %_148.not.not.i.us.i = icmp ugt i64 %_67.1.i, %_30.i.us.i, !dbg !3437
  br i1 %_148.not.not.i.us.i, label %bb48.i.us.i, label %bb49.i.i, !dbg !3437, !prof !2709

bb48.i.us.i:                                      ; preds = %bb44.i.us.i
  %_147.i.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i714.us.i, !dbg !2779
  %_0.i207.us.i = load float, ptr %_147.i.us.i, align 4, !dbg !2786, !alias.scope !2788, !noalias !2668, !noundef !12
  %_155.i.us.i = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_30.i.us.i, !dbg !2791
  store float %_0.i207.us.i, ptr %_155.i.us.i, align 4, !dbg !2798, !alias.scope !2800, !noalias !2668
  %_53.i.us.i = sub i32 %now.i.us.i, %_59.i, !dbg !2803
  %_52.i.us.i = and i32 %_53.i.us.i, %_58.i, !dbg !2806
  %_51.i.us.i = zext i32 %_52.i.us.i to i64, !dbg !2807
  %_156.not.not.i.us.i = icmp ugt i64 %_64.1.i, %_51.i.us.i, !dbg !2808
  br i1 %_156.not.not.i.us.i, label %bb50.i.us.i, label %bb51.i.i, !dbg !2808, !prof !2709

bb50.i.us.i:                                      ; preds = %bb48.i.us.i
  %_163.i.us.i = getelementptr inbounds nuw float, ptr %_64.0.i, i64 %_51.i.us.i, !dbg !2813
  %_0.i205.us.i = load float, ptr %_163.i.us.i, align 4, !dbg !2817, !alias.scope !2819, !noalias !2668, !noundef !12
  %_164.not.not.i.us.i = icmp ugt i64 %_66.1.i, %_51.i.us.i, !dbg !3445
  br i1 %_164.not.not.i.us.i, label %bb53.i.us.i, label %bb54.i.i, !dbg !3445, !prof !2709

bb53.i.us.i:                                      ; preds = %bb50.i.us.i
  %_169.i.us.i = getelementptr inbounds nuw float, ptr %_66.0.i, i64 %_51.i.us.i, !dbg !2822
  %_0.i203.us.i = load float, ptr %_169.i.us.i, align 4, !dbg !2830, !alias.scope !2832, !noalias !2668, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2835), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2841), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2838
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2845), !dbg !2838
  %_18.i74.us.i = load i32, ptr %_31.i, align 4, !dbg !2847, !alias.scope !2850, !noalias !2851, !noundef !12
  %_17.i.us.i = sub i32 %now.i.us.i, %_18.i74.us.i, !dbg !2853
  %_16.i75.us.i = and i32 %_17.i.us.i, %_58.i, !dbg !2847
  %_15.i76.us.i = zext i32 %_16.i75.us.i to i64, !dbg !2847
  %_26.i.us.i = load i32, ptr %_33.i, align 4, !dbg !2847, !alias.scope !2856, !noalias !2857, !noundef !12
  %_25.i.us.i = sub i32 %now.i.us.i, %_26.i.us.i, !dbg !2853
  %_24.i.us.i = and i32 %_25.i.us.i, %_58.i, !dbg !2847
  %_23.i79.us.i = zext i32 %_24.i.us.i to i64, !dbg !2847
  %_31.i80.us.i = icmp samesign ugt i64 %_65.1.i, %_15.i76.us.i, !dbg !2847
  switch i8 %_0.sroa.0.0.i551.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.i
    i8 1, label %bb14.i63.preheader.us.i
    i8 2, label %bb23.i.preheader.us.i
  ], !dbg !2858

bb27.i60.us.i:                                    ; preds = %bb23.i.preheader.us.i
  %283 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.i, !dbg !2859
  %_79.i.us.i = load float, ptr %283, align 4, !dbg !2859, !alias.scope !2843, !noalias !2865, !noundef !12
  %_85.i.us.i = icmp samesign ugt i64 %_67.1.i, %_15.i76.us.i, !dbg !2866
  br i1 %_85.i.us.i, label %bb29.i61.us.i, label %panic30.i.i, !dbg !2866

bb29.i61.us.i:                                    ; preds = %bb27.i60.us.i
  %284 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_15.i76.us.i, !dbg !2866
  %_83.i.us.i = load float, ptr %284, align 4, !dbg !2866, !alias.scope !2845, !noalias !2867, !noundef !12
  %_87.i.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.i, !dbg !2868
  br i1 %_87.i.us.i, label %bb31.i.us.i, label %panic32.i.i, !dbg !2868

bb31.i.us.i:                                      ; preds = %bb29.i61.us.i
  %_89.i.us.i = icmp samesign ugt i64 %_65.1.i, %_23.i79.us.i, !dbg !2869
  br i1 %_89.i.us.i, label %bb33.i62.us.i, label %panic34.i.i, !dbg !2869

bb33.i62.us.i:                                    ; preds = %bb31.i.us.i
  %285 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.i, !dbg !2868
  %_86.i.us.i = load float, ptr %285, align 4, !dbg !2868, !alias.scope !2845, !noalias !2867, !noundef !12
  %286 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_23.i79.us.i, !dbg !2869
  %_88.i.us.i = load float, ptr %286, align 4, !dbg !2869, !alias.scope !2843, !noalias !2865, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i, !dbg !2870

bb17.i.us.i:                                      ; preds = %bb14.i63.preheader.us.i
  %_59.i.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.i, !dbg !2873
  br i1 %_59.i.us.i, label %bb19.i.us.i, label %panic17.i.i, !dbg !2873

bb19.i.us.i:                                      ; preds = %bb17.i.us.i
  %287 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.i, !dbg !2880
  %left_own16.i.us.i = load float, ptr %287, align 4, !dbg !2880, !alias.scope !2843, !noalias !2865, !noundef !12
  %288 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.i, !dbg !2873
  %right_own18.i.us.i = load float, ptr %288, align 4, !dbg !2873, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i, !dbg !2870

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i: ; preds = %bb10.i82.us.i, %bb19.i.us.i, %bb33.i62.us.i
  %taps.i.sroa.1011534.1.i = phi float [ %right_own.i.us.i, %bb10.i82.us.i ], [ %left_own16.i.us.i, %bb19.i.us.i ], [ %_88.i.us.i, %bb33.i62.us.i ], !dbg !2847
  %taps.i.sroa.681533.1.i = phi float [ %right_own.i.us.i, %bb10.i82.us.i ], [ %right_own18.i.us.i, %bb19.i.us.i ], [ %_86.i.us.i, %bb33.i62.us.i ], !dbg !2847
  %taps.i.sroa.351532.1.i = phi float [ %left_own.i.us.i, %bb10.i82.us.i ], [ %right_own18.i.us.i, %bb19.i.us.i ], [ %_83.i.us.i, %bb33.i62.us.i ], !dbg !2847
  %taps.i.sroa.0.1.i = phi float [ %left_own.i.us.i, %bb10.i82.us.i ], [ %left_own16.i.us.i, %bb19.i.us.i ], [ %_79.i.us.i, %bb33.i62.us.i ], !dbg !2847
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2881), !dbg !2884
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2885), !dbg !2884
  %_12.i36.i.us.i = load float, ptr %30, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.i = fcmp ule float %_12.i36.i.us.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.i = fcmp une float %_12.i36.i.us.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.i = load float, ptr %_10.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.i = load float, ptr %31, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.i = fadd float %_16.i39.i.us.i, %_17.i40.i.us.i, !dbg !2906
  %_20.i42.i552555.us.i = load float, ptr %32, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %289 = select i1 %_3.i95.us.i, float %_0.i152.us.i, float %_20.i42.i552555.us.i, !dbg !2911
  %_0.i381.us.i = select i1 %_3.i135.us.i, float %_16.i39.i.us.i, float %289, !dbg !2914
  store float %_0.i381.us.i, ptr %_10.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.i = select i1 %_3.i95.us.i, float %_17.i40.i.us.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.i, ptr %31, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.i = fadd float %_12.i36.i.us.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.i = select i1 %_3.i135.us.i, float %_12.i36.i.us.i, float %_0.i193.us.i, !dbg !2923
  store float %_4.i367.v.us.i, ptr %30, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.1.i = load float, ptr %33, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.1.i = fcmp ule float %_12.i36.i.us.1.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.1.i = fcmp une float %_12.i36.i.us.1.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.1.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.1.i = load float, ptr %34, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.1.i = fadd float %_16.i39.i.us.1.i, %_17.i40.i.us.1.i, !dbg !2906
  %_20.i42.i552555.us.1.i = load float, ptr %35, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %290 = select i1 %_3.i95.us.1.i, float %_0.i152.us.1.i, float %_20.i42.i552555.us.1.i, !dbg !2911
  %_0.i381.us.1.i = select i1 %_3.i135.us.1.i, float %_16.i39.i.us.1.i, float %290, !dbg !2914
  store float %_0.i381.us.1.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.1.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.1.i = select i1 %_3.i95.us.1.i, float %_17.i40.i.us.1.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.1.i, ptr %34, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.1.i = fadd float %_12.i36.i.us.1.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.1.i = select i1 %_3.i135.us.1.i, float %_12.i36.i.us.1.i, float %_0.i193.us.1.i, !dbg !2923
  store float %_4.i367.v.us.1.i, ptr %33, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.2.i = load float, ptr %36, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.2.i = fcmp ule float %_12.i36.i.us.2.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.2.i = fcmp une float %_12.i36.i.us.2.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.2.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.2.i = load float, ptr %37, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.2.i = fadd float %_16.i39.i.us.2.i, %_17.i40.i.us.2.i, !dbg !2906
  %_20.i42.i552555.us.2.i = load float, ptr %38, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %291 = select i1 %_3.i95.us.2.i, float %_0.i152.us.2.i, float %_20.i42.i552555.us.2.i, !dbg !2911
  %_0.i381.us.2.i = select i1 %_3.i135.us.2.i, float %_16.i39.i.us.2.i, float %291, !dbg !2914
  store float %_0.i381.us.2.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.2.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.2.i = select i1 %_3.i95.us.2.i, float %_17.i40.i.us.2.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.2.i, ptr %37, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.2.i = fadd float %_12.i36.i.us.2.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.2.i = select i1 %_3.i135.us.2.i, float %_12.i36.i.us.2.i, float %_0.i193.us.2.i, !dbg !2923
  store float %_4.i367.v.us.2.i, ptr %36, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %_12.i36.i.us.3.i = load float, ptr %39, align 4, !dbg !2887, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i135.us.3.i = fcmp ule float %_12.i36.i.us.3.i, 0.000000e+00, !dbg !2896
  %_3.i95.us.3.i = fcmp une float %_12.i36.i.us.3.i, 1.000000e+00, !dbg !2899
  %_16.i39.i.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i34.i.us.3.i, align 4, !dbg !2903, !alias.scope !2894, !noalias !2895, !noundef !12
  %_17.i40.i.us.3.i = load float, ptr %40, align 4, !dbg !2905, !alias.scope !2894, !noalias !2895, !noundef !12
  %_0.i152.us.3.i = fadd float %_16.i39.i.us.3.i, %_17.i40.i.us.3.i, !dbg !2906
  %_20.i42.i552555.us.3.i = load float, ptr %41, align 4, !dbg !2909, !alias.scope !2894, !noalias !2895, !noundef !12
  %292 = select i1 %_3.i95.us.3.i, float %_0.i152.us.3.i, float %_20.i42.i552555.us.3.i, !dbg !2911
  %_0.i381.us.3.i = select i1 %_3.i135.us.3.i, float %_16.i39.i.us.3.i, float %292, !dbg !2914
  store float %_0.i381.us.3.i, ptr %iter1.sroa.0.0.ptr.i34.i.us.3.i, align 4, !dbg !2916, !alias.scope !2894, !noalias !2895
  %_0.i374.us.3.i = select i1 %_3.i95.us.3.i, float %_17.i40.i.us.3.i, float 0.000000e+00, !dbg !2917
  store float %_0.i374.us.3.i, ptr %40, align 4, !dbg !2919, !alias.scope !2894, !noalias !2895
  %_0.i193.us.3.i = fadd float %_12.i36.i.us.3.i, -1.000000e+00, !dbg !2920
  %_4.i367.v.us.3.i = select i1 %_3.i135.us.3.i, float %_12.i36.i.us.3.i, float %_0.i193.us.3.i, !dbg !2923
  store float %_4.i367.v.us.3.i, ptr %39, align 4, !dbg !2925, !alias.scope !2894, !noalias !2895
  %293 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.1.i), !dbg !2926
  %294 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351532.1.i), !dbg !2933
  %_37.i57.i.us.i = load float, ptr %15, align 4, !dbg !2936, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i133.us.i = fcmp ule float %_37.i57.i.us.i, 0.000000e+00, !dbg !2941
  %_3.i.i489.us.i = fcmp ule float %293, %294, !dbg !2943
  %_6.i.i491.us.i = bitcast float %293 to i32, !dbg !2949
  %_8.i.i493.us.i = bitcast float %294 to i32, !dbg !2953
  %_4.i.i496.us.i = select i1 %_3.i.i489.us.i, i32 %_8.i.i493.us.i, i32 %_6.i.i491.us.i, !dbg !2955
  %_4.i360.us.i = select i1 %_3.i133.us.i, i32 %_6.i.i491.us.i, i32 %_4.i.i496.us.i, !dbg !2956
  %_41.i61.i.us.i = load float, ptr %16, align 4, !dbg !2958, !alias.scope !2939, !noalias !2940, !noundef !12
  %_3.i131.us.i = fcmp ule float %_41.i61.i.us.i, 0.000000e+00, !dbg !2959
  %_0.i177.us.i = fmul float %293, 5.000000e-01, !dbg !2961
  %_0.i176.us.i = fmul float %294, 5.000000e-01, !dbg !2964
  %_0.i151.us.i = fadd float %_0.i176.us.i, %_0.i177.us.i, !dbg !2966
  %_6.i348.us.i = bitcast float %_0.i151.us.i to i32, !dbg !2968
  %_4.i353.us.i = select i1 %_3.i131.us.i, i32 %_4.i360.us.i, i32 %_6.i348.us.i, !dbg !2971
  %_0.i354.us.i = bitcast i32 %_4.i353.us.i to float, !dbg !2972
  %_3.i.i481.us.i = fcmp ule float %_0.i354.us.i, 0x3E45798EE0000000, !dbg !2975
  %_4.i.i487.us.i = select i1 %_3.i.i481.us.i, i32 841731191, i32 %_4.i353.us.i, !dbg !2978
  %_0.i.i488.us.i = bitcast i32 %_4.i.i487.us.i to float, !dbg !2980
  %_3.i.i.us.i = fcmp ule float %_0.i.i488.us.i, 0x3810000000000000, !dbg !2982
  %_4.i.i.us.i = select i1 %_3.i.i.us.i, i32 8388608, i32 %_4.i.i487.us.i, !dbg !2992
  %_5.i214.us.i = and i32 %_4.i.i.us.i, 8388607, !dbg !2994
  %_4.i215.us.i = or disjoint i32 %_5.i214.us.i, 1065353216, !dbg !2994
  %significand.i.us.i = bitcast i32 %_4.i215.us.i to float, !dbg !2999
  %_0.i178.us.i = fadd float %significand.i.us.i, -1.000000e+00, !dbg !3002
  %_0.i158.us.i = fmul float %_0.i178.us.i, 0x3F9B17A960000000, !dbg !3005
  %295 = fsub float 0x3FBF9A8440000000, %_0.i158.us.i, !dbg !3010
  %_0.i158.us.1.i = fmul float %_0.i178.us.i, %295, !dbg !3005
  %_0.i142.us.1.i = fadd float %_0.i158.us.1.i, 0xBFD1E3F400000000, !dbg !3010
  %_0.i158.us.2.i = fmul float %_0.i178.us.i, %_0.i142.us.1.i, !dbg !3005
  %_0.i142.us.2.i = fadd float %_0.i158.us.2.i, 0x3FDD544F20000000, !dbg !3010
  %_0.i158.us.3.i = fmul float %_0.i178.us.i, %_0.i142.us.2.i, !dbg !3005
  %_0.i142.us.3.i = fadd float %_0.i158.us.3.i, 0xBFE6FC2A60000000, !dbg !3010
  %_0.i158.us.4.i = fmul float %_0.i178.us.i, %_0.i142.us.3.i, !dbg !3005
  %_0.i142.us.4.i = fadd float %_0.i158.us.4.i, 0x3FF714B2A0000000, !dbg !3010
  %_9.i.us.i = lshr i32 %_4.i.i.us.i, 23, !dbg !3012
  %_8.i216.us.i = or disjoint i32 %_9.i.us.i, 1258291200, !dbg !3012
  %_7.i.us.i = bitcast i32 %_8.i216.us.i to float, !dbg !3014
  %exponent.i.us.i = fadd float %_7.i.us.i, 0xC160000FE0000000, !dbg !3016
  %_0.i157.us.i = fmul float %_0.i178.us.i, %_0.i142.us.4.i, !dbg !3017
  %_0.i141.us.i = fadd float %exponent.i.us.i, %_0.i157.us.i, !dbg !3019
  %_0.i175.us.i = fmul float %_0.i141.us.i, 0x4018151820000000, !dbg !3021
  %_3.i.i538.us.inv.i = fcmp olt float %_0.i175.us.i, 2.400000e+01, !dbg !3023
  %_0.i.i545.us.i = select i1 %_3.i.i538.us.inv.i, float %_0.i175.us.i, float 2.400000e+01, !dbg !3023
  %_3.i.i473.us.inv.i = fcmp ogt float %_0.i.i545.us.i, -1.600000e+02, !dbg !3027
  %_0.i.i480.us.i = select i1 %_3.i.i473.us.inv.i, float %_0.i.i545.us.i, float -1.600000e+02, !dbg !3027
  %_55.i78.i.us.i = load float, ptr %14, align 4, !dbg !3030, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i129.us.i = fcmp ule float %_55.i78.i.us.i, 0.000000e+00, !dbg !3032
  %_3.i103.us.i = fcmp oge float %_0.i.i480.us.i, %_0.i381.us.i, !dbg !3034
  %_0.i192.us.i = fsub float %_0.i381.us.i, %_0.i381.us.3.i, !dbg !3038
  %_3.i101.us.i = fcmp oge float %_0.i.i480.us.i, %_0.i192.us.i, !dbg !3041
  %..i102.us.i = sext i1 %_3.i101.us.i to i32, !dbg !3043
  %_0.i401.us.i = sext i1 %_3.i103.us.i to i32, !dbg !3046
  %_0.i394.us.i = select i1 %_3.i129.us.i, i32 %_0.i401.us.i, i32 %..i102.us.i, !dbg !3046
  %_0.i405.us.i = xor i32 %..i102.us.i, -1, !dbg !3052
  %_67.i88.i.us.i = load float, ptr %17, align 4, !dbg !3056, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i127.us.i = fcmp ogt float %_67.i88.i.us.i, 0.000000e+00, !dbg !3057
  %_0.i400.us.i = select i1 %_3.i127.us.i, i32 %_0.i405.us.i, i32 0, !dbg !3059
  %_0.i399.us.i = select i1 %_3.i129.us.i, i32 0, i32 %_0.i400.us.i, !dbg !3062
  %_0.i393.us.i = or i32 %_0.i399.us.i, %_0.i394.us.i, !dbg !3064
  %_5.i343.us.i = and i32 %_0.i393.us.i, 1065353216, !dbg !3067
  %_0.i347.us.i = bitcast i32 %_5.i343.us.i to float, !dbg !3069
  %_71.i94.i564565.us.i = load float, ptr %18, align 4, !dbg !3071, !alias.scope !2939, !noalias !2940, !noundef !12
  %_0.i191.us.i = fadd float %_67.i88.i.us.i, -1.000000e+00, !dbg !3073
  %296 = trunc nsw i32 %_0.i399.us.i to i1, !dbg !3075
  %_4.i341.v.us.i = select i1 %296, float %_0.i191.us.i, float %_67.i88.i.us.i, !dbg !3075
  %297 = trunc nsw i32 %_0.i394.us.i to i1, !dbg !3077
  %_0.i335.us.i = select i1 %297, float %_71.i94.i564565.us.i, float %_4.i341.v.us.i, !dbg !3077
  store float %_0.i335.us.i, ptr %17, align 4, !dbg !3079, !alias.scope !2894, !noalias !2895
  store i32 %_5.i343.us.i, ptr %14, align 4, !dbg !3080, !alias.scope !2894, !noalias !2895
  %_0.i190.us.i = fadd float %_0.i381.us.1.i, -1.000000e+00, !dbg !3081
  %_0.i189.us.i = fsub float %_0.i.i480.us.i, %_0.i381.us.i, !dbg !3083
  %_0.i174.us.i = fmul float %_0.i190.us.i, %_0.i189.us.i, !dbg !3085
  %298 = fneg float %_0.i381.us.2.i, !dbg !3087
  %_3.i.i464.inv.us.i = fcmp ogt float %_0.i174.us.i, %298, !dbg !3090
  %_4.i.i471.v.us.i = select i1 %_3.i.i464.inv.us.i, float %_0.i174.us.i, float %298, !dbg !3090
  %_3.i.i530.us.i = fcmp olt float %_4.i.i471.v.us.i, 0.000000e+00, !dbg !3093
  %299 = fcmp ule float %_0.i347.us.i, 0.000000e+00, !dbg !3097
  %300 = select i1 %299, i1 %_3.i.i530.us.i, i1 false, !dbg !3100
  %_0.i328.us.i = select i1 %300, float %_4.i.i471.v.us.i, float 0.000000e+00, !dbg !3100
  %_86.i107.i.us.i = load float, ptr %19, align 4, !dbg !3101, !alias.scope !2894, !noalias !2895, !noundef !12
  %_3.i123.us.i = fcmp ule float %_0.i328.us.i, %_86.i107.i.us.i, !dbg !3103
  %_87.i109.i567.us.i = load i32, ptr %4, align 4, !dbg !3105, !alias.scope !2939, !noalias !2940, !noundef !12
  %_88.i110.i568.us.i = load i32, ptr %20, align 4, !dbg !3106, !alias.scope !2939, !noalias !2940, !noundef !12
  %_4.i321.us.i = select i1 %_3.i123.us.i, i32 %_88.i110.i568.us.i, i32 %_87.i109.i567.us.i, !dbg !3107
  %_0.i322.us.i = bitcast i32 %_4.i321.us.i to float, !dbg !3109
  %_0.i188.us.i = fsub float %_0.i328.us.i, %_86.i107.i.us.i, !dbg !3111
  %_4.i155.us.i = fmul float %_0.i188.us.i, %_0.i322.us.i, !dbg !3114
  %_0.i156.us.i = fadd float %_86.i107.i.us.i, %_4.i155.us.i, !dbg !3114
  %301 = tail call noundef float @llvm.fabs.f32(float %_0.i156.us.i), !dbg !3117
  %302 = fcmp uge float %301, 0x3BC79CA100000000, !dbg !3121
  %_0.i230.us.i = select i1 %302, float %_0.i156.us.i, float 0.000000e+00, !dbg !3124
  store float %_0.i230.us.i, ptr %19, align 4, !dbg !3125, !alias.scope !2894, !noalias !2895
  %_0.i173.us.i = fmul float %_0.i230.us.i, 0x3FC542A5A0000000, !dbg !3127
  %_3.i.i415.us.inv.i = fcmp ogt float %_0.i173.us.i, -1.260000e+02, !dbg !3131
  %_0.i.i422.us.i = select i1 %_3.i.i415.us.inv.i, float %_0.i173.us.i, float -1.260000e+02, !dbg !3131
  %_3.i.i498.us.inv.i = fcmp olt float %_0.i.i422.us.i, 1.270000e+02, !dbg !3136
  %_0.i.i505.us.i = select i1 %_3.i.i498.us.inv.i, float %_0.i.i422.us.i, float 1.270000e+02, !dbg !3136
  %303 = tail call noundef float @llvm.floor.f32(float %_0.i.i505.us.i), !dbg !3139
  %_0.i180.us.i = fsub float %_0.i.i505.us.i, %303, !dbg !3151
  %_98.i123.i.us.i = load float, ptr %21, align 4, !dbg !3154, !alias.scope !2939, !noalias !2940, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3156), !dbg !3159
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3161), !dbg !3159
  %_12.i.i.us.i = load float, ptr %42, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.i = fcmp ule float %_12.i.i.us.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.i = fcmp une float %_12.i.i.us.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.i = load float, ptr %43, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.i = fadd float %_16.i.i.us.i, %_17.i.i.us.i, !dbg !3173
  %_20.i.i574577.us.i = load float, ptr %44, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %304 = select i1 %_3.i91.us.i, float %_0.i150.us.i, float %_20.i.i574577.us.i, !dbg !3176
  %_0.i301.us.i = select i1 %_3.i119.us.i, float %_16.i.i.us.i, float %304, !dbg !3178
  store float %_0.i301.us.i, ptr %data.i.i.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.i = select i1 %_3.i91.us.i, float %_17.i.i.us.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.i, ptr %43, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.i = fadd float %_12.i.i.us.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.i = select i1 %_3.i119.us.i, float %_12.i.i.us.i, float %_0.i187.us.i, !dbg !3186
  store float %_4.i287.v.us.i, ptr %42, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.1.i = load float, ptr %45, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.1.i = fcmp ule float %_12.i.i.us.1.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.1.i = fcmp une float %_12.i.i.us.1.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.1.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.1.i = load float, ptr %46, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.1.i = fadd float %_16.i.i.us.1.i, %_17.i.i.us.1.i, !dbg !3173
  %_20.i.i574577.us.1.i = load float, ptr %47, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %305 = select i1 %_3.i91.us.1.i, float %_0.i150.us.1.i, float %_20.i.i574577.us.1.i, !dbg !3176
  %_0.i301.us.1.i = select i1 %_3.i119.us.1.i, float %_16.i.i.us.1.i, float %305, !dbg !3178
  store float %_0.i301.us.1.i, ptr %iter1.sroa.0.0.ptr.i.i.us.1.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.1.i = select i1 %_3.i91.us.1.i, float %_17.i.i.us.1.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.1.i, ptr %46, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.1.i = fadd float %_12.i.i.us.1.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.1.i = select i1 %_3.i119.us.1.i, float %_12.i.i.us.1.i, float %_0.i187.us.1.i, !dbg !3186
  store float %_4.i287.v.us.1.i, ptr %45, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.2.i = load float, ptr %48, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.2.i = fcmp ule float %_12.i.i.us.2.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.2.i = fcmp une float %_12.i.i.us.2.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.2.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.2.i = load float, ptr %49, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.2.i = fadd float %_16.i.i.us.2.i, %_17.i.i.us.2.i, !dbg !3173
  %_20.i.i574577.us.2.i = load float, ptr %50, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %306 = select i1 %_3.i91.us.2.i, float %_0.i150.us.2.i, float %_20.i.i574577.us.2.i, !dbg !3176
  %_0.i301.us.2.i = select i1 %_3.i119.us.2.i, float %_16.i.i.us.2.i, float %306, !dbg !3178
  store float %_0.i301.us.2.i, ptr %iter1.sroa.0.0.ptr.i.i.us.2.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.2.i = select i1 %_3.i91.us.2.i, float %_17.i.i.us.2.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.2.i, ptr %49, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.2.i = fadd float %_12.i.i.us.2.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.2.i = select i1 %_3.i119.us.2.i, float %_12.i.i.us.2.i, float %_0.i187.us.2.i, !dbg !3186
  store float %_4.i287.v.us.2.i, ptr %48, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %_12.i.i.us.3.i = load float, ptr %51, align 4, !dbg !3163, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i119.us.3.i = fcmp ule float %_12.i.i.us.3.i, 0.000000e+00, !dbg !3167
  %_3.i91.us.3.i = fcmp une float %_12.i.i.us.3.i, 1.000000e+00, !dbg !3169
  %_16.i.i.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.3.i, align 4, !dbg !3171, !alias.scope !3165, !noalias !3166, !noundef !12
  %_17.i.i.us.3.i = load float, ptr %52, align 4, !dbg !3172, !alias.scope !3165, !noalias !3166, !noundef !12
  %_0.i150.us.3.i = fadd float %_16.i.i.us.3.i, %_17.i.i.us.3.i, !dbg !3173
  %_20.i.i574577.us.3.i = load float, ptr %53, align 4, !dbg !3175, !alias.scope !3165, !noalias !3166, !noundef !12
  %307 = select i1 %_3.i91.us.3.i, float %_0.i150.us.3.i, float %_20.i.i574577.us.3.i, !dbg !3176
  %_0.i301.us.3.i = select i1 %_3.i119.us.3.i, float %_16.i.i.us.3.i, float %307, !dbg !3178
  store float %_0.i301.us.3.i, ptr %iter1.sroa.0.0.ptr.i.i.us.3.i, align 4, !dbg !3180, !alias.scope !3165, !noalias !3166
  %_0.i294.us.3.i = select i1 %_3.i91.us.3.i, float %_17.i.i.us.3.i, float 0.000000e+00, !dbg !3181
  store float %_0.i294.us.3.i, ptr %52, align 4, !dbg !3183, !alias.scope !3165, !noalias !3166
  %_0.i187.us.3.i = fadd float %_12.i.i.us.3.i, -1.000000e+00, !dbg !3184
  %_4.i287.v.us.3.i = select i1 %_3.i119.us.3.i, float %_12.i.i.us.3.i, float %_0.i187.us.3.i, !dbg !3186
  store float %_4.i287.v.us.3.i, ptr %51, align 4, !dbg !3188, !alias.scope !3165, !noalias !3166
  %308 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681533.1.i), !dbg !3189
  %309 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011534.1.i), !dbg !3191
  %_37.i.i.us.i = load float, ptr %23, align 4, !dbg !3193, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i117.us.i = fcmp ule float %_37.i.i.us.i, 0.000000e+00, !dbg !3196
  %_3.i.i455.us.i = fcmp ule float %308, %309, !dbg !3198
  %_6.i.i457.us.i = bitcast float %308 to i32, !dbg !3201
  %_8.i.i459.us.i = bitcast float %309 to i32, !dbg !3204
  %_4.i.i462.us.i = select i1 %_3.i.i455.us.i, i32 %_8.i.i459.us.i, i32 %_6.i.i457.us.i, !dbg !3206
  %_4.i280.us.i = select i1 %_3.i117.us.i, i32 %_6.i.i457.us.i, i32 %_4.i.i462.us.i, !dbg !3207
  %_41.i.i.us.i = load float, ptr %24, align 4, !dbg !3209, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i115.us.i = fcmp ule float %_41.i.i.us.i, 0.000000e+00, !dbg !3210
  %_0.i171.us.i = fmul float %308, 5.000000e-01, !dbg !3212
  %_0.i170.us.i = fmul float %309, 5.000000e-01, !dbg !3214
  %_0.i149.us.i = fadd float %_0.i170.us.i, %_0.i171.us.i, !dbg !3216
  %_6.i268.us.i = bitcast float %_0.i149.us.i to i32, !dbg !3218
  %_4.i273.us.i = select i1 %_3.i115.us.i, i32 %_4.i280.us.i, i32 %_6.i268.us.i, !dbg !3221
  %_0.i274.us.i = bitcast i32 %_4.i273.us.i to float, !dbg !3222
  %_3.i.i447.us.i = fcmp ule float %_0.i274.us.i, 0x3E45798EE0000000, !dbg !3224
  %_4.i.i453.us.i = select i1 %_3.i.i447.us.i, i32 841731191, i32 %_4.i273.us.i, !dbg !3227
  %_0.i.i454.us.i = bitcast i32 %_4.i.i453.us.i to float, !dbg !3229
  %_3.i.i407.us.i = fcmp ule float %_0.i.i454.us.i, 0x3810000000000000, !dbg !3231
  %_4.i.i413.us.i = select i1 %_3.i.i407.us.i, i32 8388608, i32 %_4.i.i453.us.i, !dbg !3236
  %_5.i218.us.i = and i32 %_4.i.i413.us.i, 8388607, !dbg !3238
  %_4.i219.us.i = or disjoint i32 %_5.i218.us.i, 1065353216, !dbg !3238
  %significand.i220.us.i = bitcast i32 %_4.i219.us.i to float, !dbg !3240
  %_0.i179.us.i = fadd float %significand.i220.us.i, -1.000000e+00, !dbg !3242
  %_0.i160.us.i = fmul float %_0.i179.us.i, 0x3F9B17A960000000, !dbg !3244
  %310 = fsub float 0x3FBF9A8440000000, %_0.i160.us.i, !dbg !3246
  %_0.i160.us.1.i = fmul float %_0.i179.us.i, %310, !dbg !3244
  %_0.i144.us.1.i = fadd float %_0.i160.us.1.i, 0xBFD1E3F400000000, !dbg !3246
  %_0.i160.us.2.i = fmul float %_0.i179.us.i, %_0.i144.us.1.i, !dbg !3244
  %_0.i144.us.2.i = fadd float %_0.i160.us.2.i, 0x3FDD544F20000000, !dbg !3246
  %_0.i160.us.3.i = fmul float %_0.i179.us.i, %_0.i144.us.2.i, !dbg !3244
  %_0.i144.us.3.i = fadd float %_0.i160.us.3.i, 0xBFE6FC2A60000000, !dbg !3246
  %_0.i160.us.4.i = fmul float %_0.i179.us.i, %_0.i144.us.3.i, !dbg !3244
  %_0.i144.us.4.i = fadd float %_0.i160.us.4.i, 0x3FF714B2A0000000, !dbg !3246
  %_9.i221.us.i = lshr i32 %_4.i.i413.us.i, 23, !dbg !3248
  %_8.i222.us.i = or disjoint i32 %_9.i221.us.i, 1258291200, !dbg !3248
  %_7.i223.us.i = bitcast i32 %_8.i222.us.i to float, !dbg !3249
  %exponent.i224.us.i = fadd float %_7.i223.us.i, 0xC160000FE0000000, !dbg !3251
  %_0.i159.us.i = fmul float %_0.i179.us.i, %_0.i144.us.4.i, !dbg !3252
  %_0.i143.us.i = fadd float %exponent.i224.us.i, %_0.i159.us.i, !dbg !3254
  %_0.i169.us.i = fmul float %_0.i143.us.i, 0x4018151820000000, !dbg !3256
  %_3.i.i522.us.inv.i = fcmp olt float %_0.i169.us.i, 2.400000e+01, !dbg !3258
  %_0.i.i529.us.i = select i1 %_3.i.i522.us.inv.i, float %_0.i169.us.i, float 2.400000e+01, !dbg !3258
  %_3.i.i439.us.inv.i = fcmp ogt float %_0.i.i529.us.i, -1.600000e+02, !dbg !3261
  %_0.i.i446.us.i = select i1 %_3.i.i439.us.inv.i, float %_0.i.i529.us.i, float -1.600000e+02, !dbg !3261
  %_55.i.i.us.i = load float, ptr %22, align 4, !dbg !3264, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i113.us.i = fcmp ule float %_55.i.i.us.i, 0.000000e+00, !dbg !3265
  %_3.i99.us.i = fcmp oge float %_0.i.i446.us.i, %_0.i301.us.i, !dbg !3267
  %_0.i186.us.i = fsub float %_0.i301.us.i, %_0.i301.us.3.i, !dbg !3269
  %_3.i97.us.i = fcmp oge float %_0.i.i446.us.i, %_0.i186.us.i, !dbg !3271
  %..i98.us.i = sext i1 %_3.i97.us.i to i32, !dbg !3273
  %_0.i397.us.i = sext i1 %_3.i99.us.i to i32, !dbg !3275
  %_0.i391.us.i = select i1 %_3.i113.us.i, i32 %_0.i397.us.i, i32 %..i98.us.i, !dbg !3275
  %_0.i403.us.i = xor i32 %..i98.us.i, -1, !dbg !3277
  %_67.i.i.us.i = load float, ptr %25, align 4, !dbg !3279, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i111.us.i = fcmp ogt float %_67.i.i.us.i, 0.000000e+00, !dbg !3280
  %_0.i396.us.i = select i1 %_3.i111.us.i, i32 %_0.i403.us.i, i32 0, !dbg !3282
  %_0.i395.us.i = select i1 %_3.i113.us.i, i32 0, i32 %_0.i396.us.i, !dbg !3284
  %_0.i390.us.i = or i32 %_0.i395.us.i, %_0.i391.us.i, !dbg !3286
  %_5.i263.us.i = and i32 %_0.i390.us.i, 1065353216, !dbg !3288
  %_0.i267.us.i = bitcast i32 %_5.i263.us.i to float, !dbg !3290
  %_71.i.i586587.us.i = load float, ptr %26, align 4, !dbg !3292, !alias.scope !3194, !noalias !3195, !noundef !12
  %_0.i185.us.i = fadd float %_67.i.i.us.i, -1.000000e+00, !dbg !3293
  %311 = trunc nsw i32 %_0.i395.us.i to i1, !dbg !3295
  %_4.i261.v.us.i = select i1 %311, float %_0.i185.us.i, float %_67.i.i.us.i, !dbg !3295
  %312 = trunc nsw i32 %_0.i391.us.i to i1, !dbg !3297
  %_0.i255.us.i = select i1 %312, float %_71.i.i586587.us.i, float %_4.i261.v.us.i, !dbg !3297
  store float %_0.i255.us.i, ptr %25, align 4, !dbg !3299, !alias.scope !3165, !noalias !3166
  store i32 %_5.i263.us.i, ptr %22, align 4, !dbg !3300, !alias.scope !3165, !noalias !3166
  %_0.i184.us.i = fadd float %_0.i301.us.1.i, -1.000000e+00, !dbg !3301
  %_0.i183.us.i = fsub float %_0.i.i446.us.i, %_0.i301.us.i, !dbg !3303
  %_0.i168.us.i = fmul float %_0.i184.us.i, %_0.i183.us.i, !dbg !3305
  %313 = fneg float %_0.i301.us.2.i, !dbg !3307
  %_3.i.i431.inv.us.i = fcmp ogt float %_0.i168.us.i, %313, !dbg !3309
  %_4.i.i437.v.us.i = select i1 %_3.i.i431.inv.us.i, float %_0.i168.us.i, float %313, !dbg !3309
  %_3.i.i514.us.i = fcmp olt float %_4.i.i437.v.us.i, 0.000000e+00, !dbg !3312
  %314 = fcmp ule float %_0.i267.us.i, 0.000000e+00, !dbg !3315
  %315 = select i1 %314, i1 %_3.i.i514.us.i, i1 false, !dbg !3317
  %_0.i248.us.i = select i1 %315, float %_4.i.i437.v.us.i, float 0.000000e+00, !dbg !3317
  %_86.i.i.us.i = load float, ptr %27, align 4, !dbg !3318, !alias.scope !3165, !noalias !3166, !noundef !12
  %_3.i107.us.i = fcmp ule float %_0.i248.us.i, %_86.i.i.us.i, !dbg !3319
  %_87.i.i589.us.i = load i32, ptr %_38.i, align 4, !dbg !3321, !alias.scope !3194, !noalias !3195, !noundef !12
  %_88.i.i590.us.i = load i32, ptr %28, align 4, !dbg !3322, !alias.scope !3194, !noalias !3195, !noundef !12
  %_4.i241.us.i = select i1 %_3.i107.us.i, i32 %_88.i.i590.us.i, i32 %_87.i.i589.us.i, !dbg !3323
  %_0.i242.us.i = bitcast i32 %_4.i241.us.i to float, !dbg !3325
  %_0.i182.us.i = fsub float %_0.i248.us.i, %_86.i.i.us.i, !dbg !3327
  %_4.i153.us.i = fmul float %_0.i182.us.i, %_0.i242.us.i, !dbg !3329
  %_0.i154.us.i = fadd float %_86.i.i.us.i, %_4.i153.us.i, !dbg !3329
  %316 = tail call noundef float @llvm.fabs.f32(float %_0.i154.us.i), !dbg !3331
  %317 = fcmp uge float %316, 0x3BC79CA100000000, !dbg !3334
  %_0.i226.us.i = select i1 %317, float %_0.i154.us.i, float 0.000000e+00, !dbg !3336
  store float %_0.i226.us.i, ptr %27, align 4, !dbg !3337, !alias.scope !3165, !noalias !3166
  %_0.i167.us.i = fmul float %_0.i226.us.i, 0x3FC542A5A0000000, !dbg !3338
  %_3.i.i423.us.inv.i = fcmp ogt float %_0.i167.us.i, -1.260000e+02, !dbg !3341
  %_0.i.i430.us.i = select i1 %_3.i.i423.us.inv.i, float %_0.i167.us.i, float -1.260000e+02, !dbg !3341
  %_3.i.i506.us.inv.i = fcmp olt float %_0.i.i430.us.i, 1.270000e+02, !dbg !3345
  %_0.i.i513.us.i = select i1 %_3.i.i506.us.inv.i, float %_0.i.i430.us.i, float 1.270000e+02, !dbg !3345
  %318 = tail call noundef float @llvm.floor.f32(float %_0.i.i513.us.i), !dbg !3348
  %_0.i181.us.i = fsub float %_0.i.i513.us.i, %318, !dbg !3352
  %_0.i165.us.i = fmul float %_0.i181.us.i, 0x3F5E974FA0000000, !dbg !3354
  %_0.i148.us.i = fadd float %_0.i165.us.i, 0x3F82778560000000, !dbg !3359
  %_0.i165.us.1.i = fmul float %_0.i181.us.i, %_0.i148.us.i, !dbg !3354
  %_0.i148.us.1.i = fadd float %_0.i165.us.1.i, 0x3FAC91CE60000000, !dbg !3359
  %_0.i165.us.2.i = fmul float %_0.i181.us.i, %_0.i148.us.1.i, !dbg !3354
  %_0.i148.us.2.i = fadd float %_0.i165.us.2.i, 0x3FCEBDB560000000, !dbg !3359
  %_0.i165.us.3.i = fmul float %_0.i181.us.i, %_0.i148.us.2.i, !dbg !3354
  %_0.i148.us.3.i = fadd float %_0.i165.us.3.i, 0x3FE62E4BA0000000, !dbg !3359
  %_0.i163.us.i = fmul float %_0.i180.us.i, 0x3F5E974FA0000000, !dbg !3361
  %_0.i146.us.i = fadd float %_0.i163.us.i, 0x3F82778560000000, !dbg !3363
  %_0.i163.us.1.i = fmul float %_0.i180.us.i, %_0.i146.us.i, !dbg !3361
  %_0.i146.us.1.i = fadd float %_0.i163.us.1.i, 0x3FAC91CE60000000, !dbg !3363
  %_0.i163.us.2.i = fmul float %_0.i180.us.i, %_0.i146.us.1.i, !dbg !3361
  %_0.i146.us.2.i = fadd float %_0.i163.us.2.i, 0x3FCEBDB560000000, !dbg !3363
  %_0.i163.us.3.i = fmul float %_0.i180.us.i, %_0.i146.us.2.i, !dbg !3361
  %_0.i146.us.3.i = fadd float %_0.i163.us.3.i, 0x3FE62E4BA0000000, !dbg !3363
  %_0.i162.us.i = fmul float %_0.i180.us.i, %_0.i146.us.3.i, !dbg !3365
  %_0.i145.us.i = fadd float %_0.i162.us.i, 1.000000e+00, !dbg !3367
  %biased.i.us.i = fadd float %303, 0x4160000FE0000000, !dbg !3369
  %_4.i83.us.i = bitcast float %biased.i.us.i to i32, !dbg !3373
  %_3.i84.us.i = shl i32 %_4.i83.us.i, 23, !dbg !3377
  %_0.i85.us.i = bitcast i32 %_3.i84.us.i to float, !dbg !3378
  %_0.i161.us.i = fmul float %_0.i145.us.i, %_0.i85.us.i, !dbg !3381
  %_3.i93.us.i = fcmp une float %_0.i230.us.i, 0.000000e+00, !dbg !3383
  %_3.i121.us.i = fcmp ule float %_98.i123.i.us.i, 0.000000e+00, !dbg !3385
  %_0.i392572.not.us.i = and i1 %_3.i121.us.i, %_3.i93.us.i, !dbg !3387
  %_0.i172.us.i = fmul float %_0.i205.us.i, %_0.i161.us.i, !dbg !3387
  %_4.i314.v.us.i = select i1 %_0.i392572.not.us.i, float %_0.i172.us.i, float %_0.i205.us.i, !dbg !3390
  %_0.i.us.i = fmul float %_0.i181.us.i, %_0.i148.us.3.i, !dbg !3392
  %_0.i147.us.i = fadd float %_0.i.us.i, 1.000000e+00, !dbg !3394
  %biased.i86.us.i = fadd float %318, 0x4160000FE0000000, !dbg !3396
  %_4.i87.us.i = bitcast float %biased.i86.us.i to i32, !dbg !3398
  %_3.i88.us.i = shl i32 %_4.i87.us.i, 23, !dbg !3400
  %_0.i89.us.i = bitcast i32 %_3.i88.us.i to float, !dbg !3401
  %_0.i164.us.i = fmul float %_0.i147.us.i, %_0.i89.us.i, !dbg !3403
  %_3.i90.us.i = fcmp une float %_0.i226.us.i, 0.000000e+00, !dbg !3405
  %_98.i.i.us.i = load float, ptr %29, align 4, !dbg !3407, !alias.scope !3194, !noalias !3195, !noundef !12
  %_3.i105.us.i = fcmp ule float %_98.i.i.us.i, 0.000000e+00, !dbg !3408
  %_0.i389594.not.us.i = and i1 %_3.i105.us.i, %_3.i90.us.i, !dbg !3410
  %_0.i166.us.i = fmul float %_0.i203.us.i, %_0.i164.us.i, !dbg !3410
  %_4.i234.v.us.i = select i1 %_0.i389594.not.us.i, float %_0.i166.us.i, float %_0.i203.us.i, !dbg !3412
  store float %_4.i314.v.us.i, ptr %_97.i.us.i, align 4, !dbg !3414, !alias.scope !3417, !noalias !2719
  store float %_4.i234.v.us.i, ptr %_115.i.us.i, align 4, !dbg !3420, !alias.scope !3422, !noalias !2742
  %exitcond1527.not.i = icmp eq i64 %282, %..i, !dbg !3425
  br i1 %exitcond1527.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb30.i.us.i, !dbg !3428, !llvm.loop !3446

bb8.i81.us.i:                                     ; preds = %bb5.i.preheader.us.i
  %_34.i.us.i = icmp samesign ugt i64 %_67.1.i, %_23.i79.us.i, !dbg !3429
  br i1 %_34.i.us.i, label %bb10.i82.us.i, label %panic5.i.i, !dbg !3429

bb10.i82.us.i:                                    ; preds = %bb8.i81.us.i
  %319 = getelementptr inbounds nuw float, ptr %_65.0.i, i64 %_15.i76.us.i, !dbg !3436
  %left_own.i.us.i = load float, ptr %319, align 4, !dbg !3436, !alias.scope !2843, !noalias !2865, !noundef !12
  %320 = getelementptr inbounds nuw float, ptr %_67.0.i, i64 %_23.i79.us.i, !dbg !3429
  %right_own.i.us.i = load float, ptr %320, align 4, !dbg !3429, !alias.scope !2845, !noalias !2867, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i, !dbg !2870

bb5.i.preheader.us.i:                             ; preds = %bb53.i.us.i
  br i1 %_31.i80.us.i, label %bb8.i81.us.i, label %panic4.i.i, !dbg !3436

bb14.i63.preheader.us.i:                          ; preds = %bb53.i.us.i
  br i1 %_31.i80.us.i, label %bb17.i.us.i, label %panic15.i.i, !dbg !2880

bb23.i.preheader.us.i:                            ; preds = %bb53.i.us.i
  br i1 %_31.i80.us.i, label %bb27.i60.us.i, label %panic28.i.i, !dbg !2859

bb33.i.i:                                         ; preds = %bb30.i.us.us.us.us.us.us.us.i, %bb30.i.us.us.us.us.us.i, %bb30.i.us.us.us.i, %bb30.i.us.i
  %.us-phi.i = phi i64 [ %282, %bb30.i.us.i ], [ %204, %bb30.i.us.us.us.i ], [ %165, %bb30.i.us.us.us.us.us.i ], [ %126, %bb30.i.us.us.us.us.us.us.us.i ], !dbg !3447
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 1, 2305843009213693952) %..i, i64 noundef %.us-phi.i, i64 noundef range(i64 1, 2305843009213693952) %..i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d0dbd696a058609bd94bbe1fa75d0723) #24, !dbg !3447, !noalias !2668
  unreachable, !dbg !3447

bb36.i.i:                                         ; preds = %bb32.i.us.us.us.us.us.us.us.i, %bb32.i.us.us.us.us.us.i, %bb32.i.us.us.us.i, %bb32.i.us.us.i, %bb32.i.us.i
  %.us-phi716.i = phi i64 [ %_30.i.us.i, %bb32.i.us.i ], [ %_30.i.us.us.i, %bb32.i.us.us.i ], [ %_30.i.us.us.us.us.us.i, %bb32.i.us.us.us.us.us.i ], [ %_30.i.us.us.us.i, %bb32.i.us.us.us.i ], [ %_30.i.us.us.us.us.us.us.us.i, %bb32.i.us.us.us.us.us.us.us.i ]
  %_37.i.le712.i = add nuw nsw i64 %.us-phi716.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi716.i, i64 noundef %_37.i.le712.i, i64 noundef %_64.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1eeef3195352adc58f4bfdb015316fef) #24, !dbg !3448, !noalias !2668
  unreachable, !dbg !3448

bb41.i.i:                                         ; preds = %bb35.i.us.i
  %_37.i.le710.i = add nuw nsw i64 %_30.i.us.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_30.i.us.i, i64 noundef %_37.i.le710.i, i64 noundef %_66.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9d56679ca30f2caa500ddf2e89d00cb) #24, !dbg !3449, !noalias !2668
  unreachable, !dbg !3449

bb43.i.i:                                         ; preds = %bb35.i.us.us.i, %bb40.i.us.i
  %.us-phi720.i = phi i64 [ %282, %bb40.i.us.i ], [ %243, %bb35.i.us.us.i ], !dbg !3450
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %empty.sroa.6.0.i.i, i64 noundef %.us-phi720.i, i64 noundef %empty.sroa.6.0.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_31945e5cade31a240e4e50932dbe7a26) #24, !dbg !3450, !noalias !2668
  unreachable, !dbg !3450

bb45.i.i:                                         ; preds = %bb35.i.us.us.us.i, %bb42.i.us.us.i, %bb42.i.us.i
  %.us-phi722.i = phi i64 [ %_30.i.us.i, %bb42.i.us.i ], [ %_30.i.us.us.i, %bb42.i.us.us.i ], [ %_30.i.us.us.us.i, %bb35.i.us.us.us.i ]
  %_37.i.le708.i = add nuw nsw i64 %.us-phi722.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi722.i, i64 noundef %_37.i.le708.i, i64 noundef %_65.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9b3ed103a299f61e5929b9f3196be990) #24, !dbg !3451, !noalias !2668
  unreachable, !dbg !3451

bb49.i.i:                                         ; preds = %bb35.i.us.us.us.us.us.i, %bb44.i.us.us.us.i, %bb44.i.us.us.i, %bb44.i.us.i
  %.us-phi726.i = phi i64 [ %_30.i.us.us.i, %bb44.i.us.us.i ], [ %_30.i.us.us.us.i, %bb44.i.us.us.us.i ], [ %_30.i.us.i, %bb44.i.us.i ], [ %_30.i.us.us.us.us.us.i, %bb35.i.us.us.us.us.us.i ]
  %_37.i.le.i = add nuw nsw i64 %.us-phi726.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi726.i, i64 noundef %_37.i.le.i, i64 noundef %_67.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a44e1bcb659c545f1ffe97492fde2ec9) #24, !dbg !3452, !noalias !2668
  unreachable, !dbg !3452

bb51.i.i:                                         ; preds = %bb35.i.us.us.us.us.us.us.us.i, %bb48.i.us.us.us.us.us.i, %bb48.i.us.us.us.i, %bb48.i.us.us.i, %bb48.i.us.i
  %.us-phi728.i = phi i64 [ %_51.i.us.i, %bb48.i.us.i ], [ %_51.i.us.us.i, %bb48.i.us.us.i ], [ %_51.i.us.us.us.us.us.i, %bb48.i.us.us.us.us.us.i ], [ %_51.i.us.us.us.i, %bb48.i.us.us.us.i ], [ %_51.i.us.us.us.us.us.us.us.i, %bb35.i.us.us.us.us.us.us.us.i ]
  %_56.i.le706.i = add nuw nsw i64 %.us-phi728.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi728.i, i64 noundef %_56.i.le706.i, i64 noundef %_64.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7) #24, !dbg !3453, !noalias !2668
  unreachable, !dbg !3453

bb54.i.i:                                         ; preds = %bb50.i.us.i
  %_56.i.le.i = add nuw nsw i64 %_51.i.us.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_51.i.us.i, i64 noundef %_56.i.le.i, i64 noundef %_66.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f198228fd40f4c04a48a745576c5c5ee) #24, !dbg !3454, !noalias !2668
  unreachable, !dbg !3454

panic4.i.i:                                       ; preds = %bb5.i.preheader.us.us.us.us.us.us.us.i, %bb5.i.preheader.us.us.us.us.us.i, %bb5.i.preheader.us.us.us.i, %bb5.i.preheader.us.us.i, %bb5.i.preheader.us.i
  %.us-phi738.i = phi i64 [ %_15.i76.us.i, %bb5.i.preheader.us.i ], [ %_15.i76.us.us.i, %bb5.i.preheader.us.us.i ], [ %_15.i76.us.us.us.us.us.i, %bb5.i.preheader.us.us.us.us.us.i ], [ %_15.i76.us.us.us.i, %bb5.i.preheader.us.us.us.i ], [ %_15.i76.us.us.us.us.us.us.us.i, %bb5.i.preheader.us.us.us.us.us.us.us.i ], !dbg !3436
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi738.i, i64 noundef range(i64 1, 2305843009213693952) %_65.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cba26bb3a04aaba20f9c249edc5e133d) #24, !dbg !3436, !noalias !3455
  unreachable, !dbg !3436

panic5.i.i:                                       ; preds = %bb8.i81.us.us.us.us.us.us.us.i, %bb8.i81.us.us.us.us.us.i, %bb8.i81.us.us.us.i, %bb8.i81.us.us.i, %bb8.i81.us.i
  %.us-phi739.i = phi i64 [ %_23.i79.us.i, %bb8.i81.us.i ], [ %_23.i79.us.us.i, %bb8.i81.us.us.i ], [ %_23.i79.us.us.us.us.us.i, %bb8.i81.us.us.us.us.us.i ], [ %_23.i79.us.us.us.i, %bb8.i81.us.us.us.i ], [ %_23.i79.us.us.us.us.us.us.us.i, %bb8.i81.us.us.us.us.us.us.us.i ], !dbg !3429
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi739.i, i64 noundef range(i64 1, 2305843009213693952) %_67.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2ad2d313de59c36d62f4a7d75017a71f) #24, !dbg !3429, !noalias !3455
  unreachable, !dbg !3429

panic15.i.i:                                      ; preds = %bb14.i63.preheader.us.us.us.us.us.us.us.i, %bb14.i63.preheader.us.us.us.us.us.i, %bb14.i63.preheader.us.us.us.i, %bb14.i63.preheader.us.us.i, %bb14.i63.preheader.us.i
  %.us-phi736.i = phi i64 [ %_15.i76.us.i, %bb14.i63.preheader.us.i ], [ %_15.i76.us.us.i, %bb14.i63.preheader.us.us.i ], [ %_15.i76.us.us.us.us.us.i, %bb14.i63.preheader.us.us.us.us.us.i ], [ %_15.i76.us.us.us.i, %bb14.i63.preheader.us.us.us.i ], [ %_15.i76.us.us.us.us.us.us.us.i, %bb14.i63.preheader.us.us.us.us.us.us.us.i ], !dbg !2880
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi736.i, i64 noundef range(i64 1, 2305843009213693952) %_65.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2e01e5f964095ee8e5aa091c7d92b88) #24, !dbg !2880, !noalias !3455
  unreachable, !dbg !2880

panic17.i.i:                                      ; preds = %bb17.i.us.us.us.us.us.us.us.i, %bb17.i.us.us.us.us.us.i, %bb17.i.us.us.us.i, %bb17.i.us.us.i, %bb17.i.us.i
  %.us-phi737.i = phi i64 [ %_23.i79.us.i, %bb17.i.us.i ], [ %_23.i79.us.us.i, %bb17.i.us.us.i ], [ %_23.i79.us.us.us.us.us.i, %bb17.i.us.us.us.us.us.i ], [ %_23.i79.us.us.us.i, %bb17.i.us.us.us.i ], [ %_23.i79.us.us.us.us.us.us.us.i, %bb17.i.us.us.us.us.us.us.us.i ], !dbg !2873
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi737.i, i64 noundef range(i64 1, 2305843009213693952) %_67.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_48dca199019b49040caac979cf1ddc5a) #24, !dbg !2873, !noalias !3455
  unreachable, !dbg !2873

panic28.i.i:                                      ; preds = %bb23.i.preheader.us.us.us.us.us.us.us.i, %bb23.i.preheader.us.us.us.us.us.i, %bb23.i.preheader.us.us.us.i, %bb23.i.preheader.us.us.i, %bb23.i.preheader.us.i
  %.us-phi732.i = phi i64 [ %_15.i76.us.i, %bb23.i.preheader.us.i ], [ %_15.i76.us.us.i, %bb23.i.preheader.us.us.i ], [ %_15.i76.us.us.us.us.us.i, %bb23.i.preheader.us.us.us.us.us.i ], [ %_15.i76.us.us.us.i, %bb23.i.preheader.us.us.us.i ], [ %_15.i76.us.us.us.us.us.us.us.i, %bb23.i.preheader.us.us.us.us.us.us.us.i ], !dbg !2859
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi732.i, i64 noundef range(i64 1, 2305843009213693952) %_65.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4b7b2eb7fa1b19ad0451b09b71313122) #24, !dbg !2859, !noalias !3455
  unreachable, !dbg !2859

panic30.i.i:                                      ; preds = %bb27.i60.us.us.us.us.us.us.us.i, %bb27.i60.us.us.us.us.us.i, %bb27.i60.us.us.us.i, %bb27.i60.us.us.i, %bb27.i60.us.i
  %.us-phi733.i = phi i64 [ %_15.i76.us.i, %bb27.i60.us.i ], [ %_15.i76.us.us.i, %bb27.i60.us.us.i ], [ %_15.i76.us.us.us.us.us.i, %bb27.i60.us.us.us.us.us.i ], [ %_15.i76.us.us.us.i, %bb27.i60.us.us.us.i ], [ %_15.i76.us.us.us.us.us.us.us.i, %bb27.i60.us.us.us.us.us.us.us.i ], !dbg !2866
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi733.i, i64 noundef range(i64 1, 2305843009213693952) %_67.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60256fc2b51ee5bb69caa4304418caff) #24, !dbg !2866, !noalias !3455
  unreachable, !dbg !2866

panic32.i.i:                                      ; preds = %bb29.i61.us.us.us.us.us.us.us.i, %bb29.i61.us.us.us.us.us.i, %bb29.i61.us.us.us.i, %bb29.i61.us.us.i, %bb29.i61.us.i
  %.us-phi734.i = phi i64 [ %_23.i79.us.i, %bb29.i61.us.i ], [ %_23.i79.us.us.i, %bb29.i61.us.us.i ], [ %_23.i79.us.us.us.us.us.i, %bb29.i61.us.us.us.us.us.i ], [ %_23.i79.us.us.us.i, %bb29.i61.us.us.us.i ], [ %_23.i79.us.us.us.us.us.us.us.i, %bb29.i61.us.us.us.us.us.us.us.i ], !dbg !2868
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi734.i, i64 noundef range(i64 1, 2305843009213693952) %_67.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_03bcac171bcf0d517141d8cd3ac9ded0) #24, !dbg !2868, !noalias !3455
  unreachable, !dbg !2868

panic34.i.i:                                      ; preds = %bb31.i.us.us.us.us.us.us.us.i, %bb31.i.us.us.us.i, %bb31.i.us.us.i, %bb31.i.us.i
  %.us-phi735.i = phi i64 [ %_23.i79.us.us.us.i, %bb31.i.us.us.us.i ], [ %_23.i79.us.i, %bb31.i.us.i ], [ %_23.i79.us.us.i, %bb31.i.us.us.i ], [ %_23.i79.us.us.us.us.us.us.us.i, %bb31.i.us.us.us.us.us.us.us.i ], !dbg !2869
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi735.i, i64 noundef range(i64 1, 2305843009213693952) %_65.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ed84bd2438b7b7e3dfb2147bac09e923) #24, !dbg !2869, !noalias !3455
  unreachable, !dbg !2869

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit: ; preds = %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i
  %_82.i.i = trunc nuw i64 %..i to i32, !dbg !3456
  %_81.i.i = add i32 %base.i.i, %_82.i.i, !dbg !3457
  store i32 %_81.i.i, ptr %_57.i, align 4, !dbg !3459, !alias.scope !2593, !noalias !2668
  br label %bb4, !dbg !3460

bb7:                                              ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, %bb4
  %_27 = trunc nuw i64 %..i to i32, !dbg !3461
  %321 = load i32, ptr %0, align 4, !dbg !3462, !noundef !12
  %322 = sub i32 %321, %_27, !dbg !3462
  store i32 %322, ptr %0, align 4, !dbg !3462
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3463), !dbg !3466
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3467), !dbg !3466
  call void @llvm.lifetime.start.p0(ptr nonnull %iter.i), !dbg !3469, !noalias !3474
  %_7.sroa.0.sroa.3.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 16, !dbg !3469
  store ptr %left.0, ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i, align 8, !dbg !3469, !noalias !3474
  %_7.sroa.0.sroa.4.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 24, !dbg !3469
  store i64 %left.1, ptr %_7.sroa.0.sroa.4.0.iter.sroa_idx.i, align 8, !dbg !3469, !noalias !3474
  %_7.sroa.0.sroa.5.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 32, !dbg !3469
  store ptr %right.0, ptr %_7.sroa.0.sroa.5.0.iter.sroa_idx.i, align 8, !dbg !3469, !noalias !3474
  %_7.sroa.0.sroa.6.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 40, !dbg !3469
  store i64 %right.1, ptr %_7.sroa.0.sroa.6.0.iter.sroa_idx.i, align 8, !dbg !3469, !noalias !3474
  %_7184.not.i = icmp eq i64 %frames, 0
  %323 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %324 = getelementptr inbounds nuw i8, ptr %self, i64 104
  %325 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %326 = getelementptr inbounds nuw i8, ptr %self, i64 944
  %327 = getelementptr inbounds nuw i8, ptr %self, i64 72
  %sample_rate.i.i.i = load i32, ptr %327, align 8, !alias.scope !3463, !noalias !3477
  %_32.i.i.i = uitofp i32 %sample_rate.i.i.i to double
  %328 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %329 = getelementptr inbounds nuw i8, ptr %self, i64 744
  %330 = getelementptr inbounds nuw i8, ptr %self, i64 792
  %slots.i.i = load i64, ptr %323, align 8, !alias.scope !3463, !noalias !3477
  %_197.not.i.i = icmp eq i64 %slots.i.i, 0
  %_12.i.i.i = load i32, ptr %328, align 8, !alias.scope !3463, !noalias !3477
  br label %bb6.i4, !dbg !3478

bb6.i4:                                           ; preds = %bb1.backedge.i, %bb7
  %counter.sroa.0.0.v.i = phi i64 [ 24, %bb7 ], [ 32, %bb1.backedge.i ]
  %_5.not.i.i.i.i = phi i1 [ false, %bb7 ], [ true, %bb1.backedge.i ]
  %331 = phi i64 [ 0, %bb7 ], [ 1, %bb1.backedge.i ]
  %self3.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<&mut [f32]>", ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i, i64 %331, !dbg !3498
  %_14.0.i.i.i.i = load ptr, ptr %self3.i.i.i.i, align 8, !dbg !3511, !alias.scope !3524, !noalias !3531, !nonnull !12, !align !3533, !noundef !12
  %332 = getelementptr inbounds nuw i8, ptr %self3.i.i.i.i, i64 8, !dbg !3511
  %_14.1.i.i.i.i = load i64, ptr %332, align 8, !dbg !3511, !alias.scope !3524, !noalias !3531, !noundef !12
  %333 = getelementptr inbounds nuw %"kernel::GateState<f32>", ptr %self, i64 %331, !dbg !3534
  %334 = getelementptr inbounds nuw i8, ptr %333, i64 864, !dbg !3534
  %_17.i = load float, ptr %334, align 4, !dbg !3534, !alias.scope !3463, !noalias !3477, !noundef !12
  %_22.not.i2177.i = icmp eq i64 %_14.1.i.i.i.i, 0, !dbg !3536
  br i1 %_22.not.i2177.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !3536

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb6.i4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %ok.sroa.0.0.i2080.i = phi i32 [ %_0.i40.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ -1, %bb6.i4 ]
  %iter.sroa.0.0.i1979.i = phi ptr [ %_27.i23.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %_14.0.i.i.i.i, %bb6.i4 ]
  %iter.sroa.5.0.i1878.i = phi i64 [ %_28.i24.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %_14.1.i.i.i.i, %bb6.i4 ]
  %_27.i23.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1979.i, i64 4, !dbg !3551
  %_28.i24.i = add i64 %iter.sroa.5.0.i1878.i, -1, !dbg !3558
  %_0.i35.i = load float, ptr %iter.sroa.0.0.i1979.i, align 4, !dbg !3559, !alias.scope !3562, !noalias !3565, !noundef !12
  %335 = tail call noundef float @llvm.fabs.f32(float %_0.i35.i), !dbg !3566
  %_3.i.i = fcmp olt float %335, 0x46293E5940000000, !dbg !3569
  %_0.i40.i = select i1 %_3.i.i, i32 %ok.sroa.0.0.i2080.i, i32 0, !dbg !3571
  %_22.not.i21.i = icmp eq i64 %_28.i24.i, 0, !dbg !3536
  br i1 %_22.not.i21.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !3536

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %336 = icmp eq i32 %_0.i40.i, -1, !dbg !3573
  %337 = tail call float @llvm.fabs.f32(float %_17.i)
  %_3.i33120.i = fcmp olt float %337, 0x46293E5940000000
  %or.cond.i = and i1 %_3.i33120.i, %336, !dbg !3576
  br i1 %or.cond.i, label %bb1.backedge.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !3576

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i: ; preds = %bb6.i4
  %338 = tail call noundef float @llvm.fabs.f32(float %_17.i), !dbg !3577
  %_3.i33.i = fcmp olt float %338, 0x46293E5940000000, !dbg !3580
  br i1 %_3.i33.i, label %bb1.backedge.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i, !dbg !3582

bb1.backedge.i:                                   ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i
  br i1 %_5.not.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E12finish_blockB2_.exit, label %bb6.i4, !dbg !3478

bb24.loopexit.i.i:                                ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %339 = and i32 %_0.i7.i.i, 1065353216, !dbg !3583
  %340 = icmp ne i32 %339, 1065353216, !dbg !3590
  %341 = zext i1 %340 to i32, !dbg !3590
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i, !dbg !3594

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %ok.sroa.0.017.i.i = phi i32 [ %_0.i7.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i ]
  %iter.sroa.0.016.i.i = phi ptr [ %_45.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %_14.0.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i ]
  %iter.sroa.5.015.i.i = phi i64 [ %_46.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %_14.1.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i ]
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i.i, i64 4, !dbg !3595
  %_46.i.i = add nsw i64 %iter.sroa.5.015.i.i, -1, !dbg !3608
  %_0.i.i.i = load float, ptr %iter.sroa.0.016.i.i, align 4, !dbg !3609, !alias.scope !3612, !noalias !3565, !noundef !12
  %342 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i), !dbg !3617
  %_3.i.i.i = fcmp olt float %342, 0x46293E5940000000, !dbg !3620
  %_0.i7.i.i = select i1 %_3.i.i.i, i32 %ok.sroa.0.017.i.i, i32 0, !dbg !3622
  %_40.not.i.i = icmp eq i64 %_46.i.i, 0, !dbg !3624
  br i1 %_40.not.i.i, label %bb24.loopexit.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !3624

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i: ; preds = %bb24.loopexit.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i
  %.pre-phi = phi float [ %337, %bb24.loopexit.i.i ], [ %338, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i ], !dbg !3625
  %ok.sroa.0.0.lcssa.i.i = phi i32 [ %341, %bb24.loopexit.i.i ], [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i ], !dbg !3628
  %_3.i.i54.i = fcmp uge float %.pre-phi, 0x46293E5940000000, !dbg !3629
  %343 = zext i1 %_3.i.i54.i to i32, !dbg !3631
  %failed.i = or i32 %ok.sroa.0.0.lcssa.i.i, %343, !dbg !3632
  %344 = icmp eq i32 %failed.i, 0
  %ring.i.i = getelementptr inbounds nuw %Ring, ptr %324, i64 %331
  %345 = getelementptr inbounds nuw i8, ptr %ring.i.i, i64 8
  %346 = getelementptr inbounds nuw i8, ptr %ring.i.i, i64 24
  %347 = getelementptr inbounds nuw i8, ptr %ring.i.i, i64 16
  %348 = getelementptr inbounds nuw [8 x float], ptr %325, i64 %331
  %values.sroa.5.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %348, i64 4
  %values.sroa.6.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %348, i64 8
  %values.sroa.7.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %348, i64 12
  %values.sroa.8.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %348, i64 16
  %values.sroa.9.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %348, i64 20
  %values.sroa.10.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %348, i64 24
  %values.sroa.11.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %348, i64 28
  %349 = getelementptr inbounds nuw %LaneTiming, ptr %326, i64 %331
  %_7.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %349, i64 4
  %_7.sroa.5.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %349, i64 8
  %_7.sroa.6.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %349, i64 12
  %350 = getelementptr inbounds nuw %Ring, ptr %self, i64 %331
  %351 = getelementptr inbounds nuw i8, ptr %350, i64 136
  %352 = getelementptr inbounds nuw %"kernel::GateCoef<f32>", ptr %329, i64 %331
  %_19.i.i.i = getelementptr inbounds nuw i8, ptr %352, i64 8
  %_25.i.i.i = getelementptr inbounds nuw i8, ptr %352, i64 4
  %353 = getelementptr inbounds nuw %"kernel::GateState<f32>", ptr %330, i64 %331
  %_17.i.i = getelementptr inbounds nuw i8, ptr %353, i64 72
  %_19.i.i = getelementptr inbounds nuw i8, ptr %353, i64 64
  %_21.i.i = getelementptr inbounds nuw i8, ptr %353, i64 68
  %_37.i.i = getelementptr inbounds nuw i8, ptr %353, i64 4
  %_39.i.i = getelementptr inbounds nuw i8, ptr %353, i64 8
  %_41.i.i = getelementptr inbounds nuw i8, ptr %353, i64 12
  %iter.sroa.0.0.ptr24.1.i.i = getelementptr inbounds nuw i8, ptr %353, i64 16
  %_37.1.i.i = getelementptr inbounds nuw i8, ptr %353, i64 20
  %_39.1.i.i = getelementptr inbounds nuw i8, ptr %353, i64 24
  %_41.1.i.i = getelementptr inbounds nuw i8, ptr %353, i64 28
  %iter.sroa.0.0.ptr24.2.i.i = getelementptr inbounds nuw i8, ptr %353, i64 32
  %_37.2.i.i = getelementptr inbounds nuw i8, ptr %353, i64 36
  %_39.2.i.i = getelementptr inbounds nuw i8, ptr %353, i64 40
  %_41.2.i.i = getelementptr inbounds nuw i8, ptr %353, i64 44
  %iter.sroa.0.0.ptr24.3.i.i = getelementptr inbounds nuw i8, ptr %353, i64 48
  %_37.3.i.i = getelementptr inbounds nuw i8, ptr %353, i64 52
  %_39.3.i.i = getelementptr inbounds nuw i8, ptr %353, i64 56
  %_41.3.i.i = getelementptr inbounds nuw i8, ptr %353, i64 60
  %counter.sroa.0.0.i = getelementptr inbounds nuw i8, ptr %reports, i64 %counter.sroa.0.0.v.i
  br i1 %344, label %bb1.backedge.i, label %bb30.i

bb30.i:                                           ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i
  br i1 %_7184.not.i, label %bb33.i, label %bb32.i, !dbg !3633

bb33.i:                                           ; preds = %bb21.i, %bb30.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3644), !dbg !3647
  br i1 %_197.not.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i, label %bb7.lr.ph.i.i, !dbg !3650

bb7.lr.ph.i.i:                                    ; preds = %bb33.i
  %_23.1.i.i = load i64, ptr %345, align 8, !alias.scope !3662, !noalias !3477, !noundef !12
  br label %bb7.i.i, !dbg !3650

bb7.i.i:                                          ; preds = %bb5.i.i, %bb7.lr.ph.i.i
  %iter.sroa.0.08.i.i = phi i64 [ 0, %bb7.lr.ph.i.i ], [ %354, %bb5.i.i ]
  %354 = add nuw i64 %iter.sroa.0.08.i.i, 1, !dbg !3663
  %exitcond.not.i.i = icmp eq i64 %iter.sroa.0.08.i.i, %_23.1.i.i, !dbg !3669
  br i1 %exitcond.not.i.i, label %panic1.i.i, label %bb3.i.i, !dbg !3669

bb3.i.i:                                          ; preds = %bb7.i.i
  %_23.0.i.i = load ptr, ptr %ring.i.i, align 8, !dbg !3669, !alias.scope !3662, !noalias !3477, !nonnull !12, !noundef !12
  %355 = getelementptr inbounds nuw float, ptr %_23.0.i.i, i64 %iter.sroa.0.08.i.i, !dbg !3669
  store float 0.000000e+00, ptr %355, align 4, !dbg !3669, !noalias !3671
  %_25.1.i.i = load i64, ptr %346, align 8, !dbg !3672, !alias.scope !3662, !noalias !3477, !noundef !12
  %_14.i.i = icmp ult i64 %iter.sroa.0.08.i.i, %_25.1.i.i, !dbg !3672
  br i1 %_14.i.i, label %bb5.i.i, label %panic2.i.i, !dbg !3672

panic1.i.i:                                       ; preds = %bb7.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.1.i.i, i64 noundef %_23.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_835aafef72e8508601474e7b1f4172a9) #24, !dbg !3669, !noalias !3671
  unreachable, !dbg !3669

bb5.i.i:                                          ; preds = %bb3.i.i
  %_25.0.i.i = load ptr, ptr %347, align 8, !dbg !3672, !alias.scope !3662, !noalias !3477, !nonnull !12, !noundef !12
  %356 = getelementptr inbounds nuw float, ptr %_25.0.i.i, i64 %iter.sroa.0.08.i.i, !dbg !3672
  store float 0.000000e+00, ptr %356, align 4, !dbg !3672, !noalias !3671
  %exitcond12.not.i.i = icmp eq i64 %354, %slots.i.i, !dbg !3673
  br i1 %exitcond12.not.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i, label %bb7.i.i, !dbg !3650

panic2.i.i:                                       ; preds = %bb3.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.sroa.0.08.i.i, i64 noundef %_25.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9043cc21280ce18935f79be264d94710) #24, !dbg !3672, !noalias !3671
  unreachable, !dbg !3672

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i: ; preds = %bb5.i.i, %bb33.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3676), !dbg !3679
  %values.sroa.0.0.copyload.i.i = load float, ptr %348, align 8, !dbg !3680, !alias.scope !3683, !noalias !3477
  %values.sroa.5.0.copyload.i.i = load float, ptr %values.sroa.5.0..sroa_idx.i.i, align 4, !dbg !3680, !alias.scope !3683, !noalias !3477
  %values.sroa.6.0.copyload.i.i = load float, ptr %values.sroa.6.0..sroa_idx.i.i, align 8, !dbg !3680, !alias.scope !3683, !noalias !3477
  %values.sroa.7.0.copyload.i.i = load float, ptr %values.sroa.7.0..sroa_idx.i.i, align 4, !dbg !3680, !alias.scope !3683, !noalias !3477
  %values.sroa.8.0.copyload.i.i = load float, ptr %values.sroa.8.0..sroa_idx.i.i, align 8, !dbg !3680, !alias.scope !3683, !noalias !3477
  %values.sroa.9.0.copyload.i.i = load float, ptr %values.sroa.9.0..sroa_idx.i.i, align 4, !dbg !3680, !alias.scope !3683, !noalias !3477
  %values.sroa.10.0.copyload.i.i = load float, ptr %values.sroa.10.0..sroa_idx.i.i, align 8, !dbg !3680, !alias.scope !3683, !noalias !3477
  %values.sroa.11.0.copyload.i.i = load float, ptr %values.sroa.11.0..sroa_idx.i.i, align 4, !dbg !3680, !alias.scope !3683, !noalias !3477
  store float %values.sroa.11.0.copyload.i.i, ptr %349, align 8, !dbg !3684, !alias.scope !3683, !noalias !3477
  store float %values.sroa.8.0.copyload.i.i, ptr %_7.sroa.4.0..sroa_idx.i.i, align 4, !dbg !3684, !alias.scope !3683, !noalias !3477
  store float %values.sroa.9.0.copyload.i.i, ptr %_7.sroa.5.0..sroa_idx.i.i, align 8, !dbg !3684, !alias.scope !3683, !noalias !3477
  store float %values.sroa.10.0.copyload.i.i, ptr %_7.sroa.6.0..sroa_idx.i.i, align 4, !dbg !3684, !alias.scope !3683, !noalias !3477
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3686), !dbg !3689
  %_31.i.i.i = fpext float %values.sroa.11.0.copyload.i.i to double, !dbg !3690
  %_30.i.i.i = fmul double %_32.i.i.i, %_31.i.i.i, !dbg !3694
  %_29.i.i.i = fdiv double %_30.i.i.i, 1.000000e+03, !dbg !3694
  %_28.i.i.i = fadd double %_29.i.i.i, 5.000000e-01, !dbg !3695
  %357 = tail call double @llvm.floor.f64(double %_28.i.i.i), !dbg !3696
  %or.cond.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %357, i32 527), !dbg !3699
  %_35.i.i.i = fcmp ogt double %357, 0x41EFFFFFFFE00000
  %or.cond10.i.i.i = or i1 %or.cond.i.i.i, %_35.i.i.i, !dbg !3699
  br i1 %or.cond10.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i, label %bb19.i.i.i, !dbg !3699

bb19.i.i.i:                                       ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i
  %_45.i.i.i = fpext float %values.sroa.9.0.copyload.i.i to double, !dbg !3700
  %_44.i.i.i = fmul double %_32.i.i.i, %_45.i.i.i, !dbg !3703
  %_43.i.i.i = fdiv double %_44.i.i.i, 1.000000e+03, !dbg !3703
  %_42.i.i.i = fadd double %_43.i.i.i, 5.000000e-01, !dbg !3704
  %358 = tail call double @llvm.floor.f64(double %_42.i.i.i), !dbg !3705
  %or.cond11.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %358, i32 527), !dbg !3708
  %_48.i.i.i = fcmp ogt double %358, 0x41EFFFFFFFE00000
  %or.cond12.i.i.i = or i1 %or.cond11.i.i.i, %_48.i.i.i, !dbg !3708
  br i1 %or.cond12.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i, label %bb20.3.i.i, !dbg !3708

bb20.3.i.i:                                       ; preds = %bb19.i.i.i
  %_36.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %357), !dbg !3709
  %_49.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %358), !dbg !3710
  %359 = tail call i32 @llvm.usub.sat.i32(i32 %_12.i.i.i, i32 %_36.i.i.i), !dbg !3711
  store i32 %359, ptr %351, align 4, !dbg !3711, !alias.scope !3712, !noalias !3477
  %_20.i.i.i = uitofp i32 %_49.i.i.i to float, !dbg !3713
  store float %_20.i.i.i, ptr %_19.i.i.i, align 4, !dbg !3714, !alias.scope !3716, !noalias !3477
; call effect_runtime::envelope::attack_release_coefficient
  %_23.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.8.0.copyload.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !3719, !noalias !3720
  store float %_23.i.i.i, ptr %352, align 4, !dbg !3721, !alias.scope !3723, !noalias !3477
; call effect_runtime::envelope::attack_release_coefficient
  %_26.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.10.0.copyload.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !3726, !noalias !3720
  store float %_26.i.i.i, ptr %_25.i.i.i, align 4, !dbg !3727, !alias.scope !3729, !noalias !3477
  store float 0.000000e+00, ptr %_17.i.i, align 4, !dbg !3732, !alias.scope !3735, !noalias !3477
  store float 1.000000e+00, ptr %_19.i.i, align 4, !dbg !3738, !alias.scope !3740, !noalias !3477
  store float %_20.i.i.i, ptr %_21.i.i, align 4, !dbg !3743, !alias.scope !3745, !noalias !3477
  store float %values.sroa.0.0.copyload.i.i, ptr %353, align 4, !dbg !3748, !alias.scope !3752, !noalias !3477
  store float %values.sroa.0.0.copyload.i.i, ptr %_37.i.i, align 4, !dbg !3755, !alias.scope !3757, !noalias !3477
  store float 0.000000e+00, ptr %_39.i.i, align 4, !dbg !3760, !alias.scope !3762, !noalias !3477
  store float 0.000000e+00, ptr %_41.i.i, align 4, !dbg !3765, !alias.scope !3767, !noalias !3477
  store float %values.sroa.5.0.copyload.i.i, ptr %iter.sroa.0.0.ptr24.1.i.i, align 4, !dbg !3748, !alias.scope !3752, !noalias !3477
  store float %values.sroa.5.0.copyload.i.i, ptr %_37.1.i.i, align 4, !dbg !3755, !alias.scope !3757, !noalias !3477
  store float 0.000000e+00, ptr %_39.1.i.i, align 4, !dbg !3760, !alias.scope !3762, !noalias !3477
  store float 0.000000e+00, ptr %_41.1.i.i, align 4, !dbg !3765, !alias.scope !3767, !noalias !3477
  store float %values.sroa.6.0.copyload.i.i, ptr %iter.sroa.0.0.ptr24.2.i.i, align 4, !dbg !3748, !alias.scope !3752, !noalias !3477
  store float %values.sroa.6.0.copyload.i.i, ptr %_37.2.i.i, align 4, !dbg !3755, !alias.scope !3757, !noalias !3477
  store float 0.000000e+00, ptr %_39.2.i.i, align 4, !dbg !3760, !alias.scope !3762, !noalias !3477
  store float 0.000000e+00, ptr %_41.2.i.i, align 4, !dbg !3765, !alias.scope !3767, !noalias !3477
  store float %values.sroa.7.0.copyload.i.i, ptr %iter.sroa.0.0.ptr24.3.i.i, align 4, !dbg !3748, !alias.scope !3752, !noalias !3477
  store float %values.sroa.7.0.copyload.i.i, ptr %_37.3.i.i, align 4, !dbg !3755, !alias.scope !3757, !noalias !3477
  store float 0.000000e+00, ptr %_39.3.i.i, align 4, !dbg !3760, !alias.scope !3762, !noalias !3477
  store float 0.000000e+00, ptr %_41.3.i.i, align 4, !dbg !3765, !alias.scope !3767, !noalias !3477
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i, !dbg !3770

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i: ; preds = %bb20.3.i.i, %bb19.i.i.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i
  %_46.i = load i64, ptr %counter.sroa.0.0.i, align 8, !dbg !3771, !alias.scope !3467, !noalias !3774, !noundef !12
  %360 = tail call i64 @llvm.uadd.sat.i64(i64 %_46.i, i64 %frames), !dbg !3775
  store i64 %360, ptr %counter.sroa.0.0.i, align 8, !dbg !3779, !alias.scope !3467, !noalias !3774
  br label %bb1.backedge.i, !dbg !3478

bb32.i:                                           ; preds = %bb30.i, %bb21.i
  %iter2.sroa.0.085.i = phi i64 [ %361, %bb21.i ], [ 0, %bb30.i ]
  %exitcond.not.i5 = icmp eq i64 %iter2.sroa.0.085.i, %_14.1.i.i.i.i, !dbg !3780
  br i1 %exitcond.not.i5, label %panic4.i, label %bb21.i, !dbg !3780

bb21.i:                                           ; preds = %bb32.i
  %361 = add nuw i64 %iter2.sroa.0.085.i, 1, !dbg !3782
  %362 = getelementptr inbounds nuw float, ptr %_14.0.i.i.i.i, i64 %iter2.sroa.0.085.i, !dbg !3780
  store float 0.000000e+00, ptr %362, align 4, !dbg !3780, !noalias !3565
  %exitcond115.not.i = icmp eq i64 %361, %frames, !dbg !3790
  br i1 %exitcond115.not.i, label %bb33.i, label %bb32.i, !dbg !3633

panic4.i:                                         ; preds = %bb32.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_14.1.i.i.i.i, i64 noundef %_14.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6797264598a169e4722ae66c7bc497b8) #24, !dbg !3780, !noalias !3565
  unreachable, !dbg !3780

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E12finish_blockB2_.exit: ; preds = %bb1.backedge.i
  call void @llvm.lifetime.end.p0(ptr nonnull %iter.i), !dbg !3794, !noalias !3474
  ret void, !dbg !3795

bb5:                                              ; preds = %bb4
  %363 = load ptr, ptr %sidechain, align 8, !dbg !3796, !noundef !12
  %.not3 = icmp eq ptr %363, null, !dbg !3796
  br i1 %.not3, label %bb20, label %bb22, !dbg !3800

bb22:                                             ; preds = %bb5
  %_51.sroa.4.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 8, !dbg !3801
  %_51.sroa.4.0.copyload = load i64, ptr %_51.sroa.4.0.sidechain.sroa_idx, align 8, !dbg !3801
  %_51.sroa.5.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 16, !dbg !3801
  %_51.sroa.5.0.copyload = load ptr, ptr %_51.sroa.5.0.sidechain.sroa_idx, align 8, !dbg !3801, !nonnull !12, !noundef !12
  %_51.sroa.6.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 24, !dbg !3801
  %_51.sroa.6.0.copyload = load i64, ptr %_51.sroa.6.0.sidechain.sroa_idx, align 8, !dbg !3801
  %_9.i = icmp ugt i64 %..i, %_51.sroa.4.0.copyload, !dbg !3803
  br i1 %_9.i, label %bb1.i12, label %bb2.i, !dbg !3803, !prof !180

bb2.i:                                            ; preds = %bb22
  %_17.i9 = icmp ugt i64 %..i, %_51.sroa.6.0.copyload, !dbg !3812
  br i1 %_17.i9, label %bb3.i11, label %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit, !dbg !3812, !prof !180

bb1.i12:                                          ; preds = %bb22
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i, i64 noundef %_51.sroa.4.0.copyload, i64 noundef %_51.sroa.4.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b376fc9587b2ace3f5c69e612169ea0b) #24, !dbg !3816, !noalias !3817
  unreachable, !dbg !3816

bb3.i11:                                          ; preds = %bb2.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i, i64 noundef %_51.sroa.6.0.copyload, i64 noundef %_51.sroa.6.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_23ac76c781f7ef32d3975251265b95f9) #24, !dbg !3821, !noalias !3817
  unreachable, !dbg !3821

_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit: ; preds = %bb2.i
  %_16.i = getelementptr inbounds nuw float, ptr %363, i64 %..i, !dbg !3822
  %_12.i = sub nuw i64 %_51.sroa.4.0.copyload, %..i, !dbg !3827
  %_20.i = sub nuw i64 %_51.sroa.6.0.copyload, %..i, !dbg !3828
  %_24.i = getelementptr inbounds nuw float, ptr %_51.sroa.5.0.copyload, i64 %..i, !dbg !3829
  br label %bb20, !dbg !3834

bb20:                                             ; preds = %bb5, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit
  %side2.sroa.3.0 = phi i64 [ %_12.i, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit ], [ undef, %bb5 ]
  %side2.sroa.0.0 = phi ptr [ %_16.i, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit ], [ null, %bb5 ], !dbg !3835
  %side2.sroa.4.0 = phi ptr [ %_24.i, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit ], [ undef, %bb5 ]
  %side2.sroa.5.0 = phi i64 [ %_20.i, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit ], [ undef, %bb5 ]
  %_52 = icmp samesign ugt i64 %..i, %left.1, !dbg !3836
  br i1 %_52, label %bb24, label %bb25, !dbg !3836, !prof !180

bb25:                                             ; preds = %bb20
  %_60 = icmp samesign ugt i64 %..i, %right.1, !dbg !3843
  br i1 %_60, label %bb26, label %bb27, !dbg !3843, !prof !180

bb24:                                             ; preds = %bb20
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i, i64 noundef %left.1, i64 noundef %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e79f4c3c2f015f90ab54e70b61044ab) #24, !dbg !3847
  unreachable, !dbg !3847

bb27:                                             ; preds = %bb25
  %_59 = getelementptr inbounds nuw float, ptr %left.0, i64 %..i, !dbg !3848
  %_55 = sub nuw nsw i64 %left.1, %..i, !dbg !3853
  %_63 = sub nuw nsw i64 %right.1, %..i, !dbg !3854
  %_67 = getelementptr inbounds nuw float, ptr %right.0, i64 %..i, !dbg !3855
  %_26 = sub i64 %frames, %..i, !dbg !3860
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3861), !dbg !3864
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3865), !dbg !3864
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3867), !dbg !3864
  %_10.i13 = getelementptr inbounds nuw i8, ptr %self, i64 792, !dbg !3869
  %data.i.i.i14 = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !3872
  %_15.i15 = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !3877
  %data.i.i480.i = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !3879
  %364 = getelementptr inbounds nuw i8, ptr %self, i64 68, !dbg !3884
  %_29.i16 = load i32, ptr %364, align 4, !dbg !3884, !range !1335, !alias.scope !3861, !noalias !3888, !noundef !12
  %_31.i17 = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !3890
  %_33.i18 = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !3891
  %365 = icmp eq i32 %_29.i16, 1, !dbg !3892
  %.val.i.i.i19 = load i32, ptr %_31.i17, align 4, !dbg !3892, !alias.scope !3861, !noalias !3888
  %.val1.i.i.i20 = load i32, ptr %_33.i18, align 4, !dbg !3892, !alias.scope !3861, !noalias !3888
  %_0.i.i.not.i.i.i21 = icmp eq i32 %.val.i.i.i19, %.val1.i.i.i20, !dbg !3892
  %spec.select.i22 = select i1 %_0.i.i.not.i.i.i21, i8 1, i8 2, !dbg !3892
  %_0.sroa.0.0.i485.i = select i1 %365, i8 0, i8 %spec.select.i22, !dbg !3892
  %366 = getelementptr inbounds nuw i8, ptr %self, i64 744, !dbg !3894
  %_38.i23 = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !3896
  %_64.0.i24 = load ptr, ptr %_15.i15, align 8, !dbg !3897, !alias.scope !3861, !noalias !3888, !nonnull !12, !noundef !12
  %367 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !3897
  %_64.1.i25 = load i64, ptr %367, align 8, !dbg !3897, !alias.scope !3861, !noalias !3888, !noundef !12
  %368 = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !3898
  %_65.0.i26 = load ptr, ptr %368, align 8, !dbg !3898, !alias.scope !3861, !noalias !3888, !nonnull !12, !noundef !12
  %369 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !3898
  %_65.1.i27 = load i64, ptr %369, align 8, !dbg !3898, !alias.scope !3861, !noalias !3888, !noundef !12
  %_66.0.i28 = load ptr, ptr %data.i.i480.i, align 8, !dbg !3899, !alias.scope !3861, !noalias !3888, !nonnull !12, !noundef !12
  %370 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !3899
  %_66.1.i29 = load i64, ptr %370, align 8, !dbg !3899, !alias.scope !3861, !noalias !3888, !noundef !12
  %371 = getelementptr inbounds nuw i8, ptr %self, i64 184, !dbg !3900
  %_67.0.i30 = load ptr, ptr %371, align 8, !dbg !3900, !alias.scope !3861, !noalias !3888, !nonnull !12, !noundef !12
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 192, !dbg !3900
  %_67.1.i31 = load i64, ptr %372, align 8, !dbg !3900, !alias.scope !3861, !noalias !3888, !noundef !12
  %_57.i32 = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !3901
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !3902
  %_58.i33 = load i32, ptr %373, align 4, !dbg !3902, !alias.scope !3861, !noalias !3888, !noundef !12
  %374 = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !3903
  %_59.i34 = load i32, ptr %374, align 8, !dbg !3903, !alias.scope !3861, !noalias !3888, !noundef !12
  %.not.i.i36 = icmp eq ptr %side2.sroa.0.0, null, !dbg !3904
  br i1 %.not.i.i36, label %bb27.i.i44, label %bb29.i.i37, !dbg !3915

bb29.i.i37:                                       ; preds = %bb27
  %375 = icmp ne ptr %side2.sroa.4.0, null
  tail call void @llvm.assume(i1 %375)
  %376 = freeze i64 %side2.sroa.3.0
  %377 = freeze i64 %side2.sroa.5.0
  br label %bb27.i.i44, !dbg !3916

bb27.i.i44:                                       ; preds = %bb29.i.i37, %bb27
  %empty.sroa.0.0.i.i45 = phi ptr [ %side2.sroa.4.0, %bb29.i.i37 ], [ inttoptr (i64 4 to ptr), %bb27 ], !dbg !3917
  %empty.sroa.6.0.i.i46 = phi i64 [ %377, %bb29.i.i37 ], [ 0, %bb27 ], !dbg !3917
  %side_left.sroa.0.0.i.i47 = phi ptr [ %side2.sroa.0.0, %bb29.i.i37 ], [ inttoptr (i64 4 to ptr), %bb27 ], !dbg !3918
  %side_left.sroa.5.0.i.i48 = phi i64 [ %376, %bb29.i.i37 ], [ 0, %bb27 ], !dbg !3918
  %base.i.i49 = load i32, ptr %_57.i32, align 4, !dbg !3919, !alias.scope !3861, !noalias !3921, !noundef !12
  %378 = getelementptr inbounds nuw i8, ptr %self, i64 808
  %379 = getelementptr inbounds nuw i8, ptr %self, i64 824
  %380 = getelementptr inbounds nuw i8, ptr %self, i64 840
  %381 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %382 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %383 = getelementptr inbounds nuw i8, ptr %self, i64 856
  %384 = getelementptr inbounds nuw i8, ptr %self, i64 860
  %385 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %386 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %387 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %388 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %389 = getelementptr inbounds nuw i8, ptr %self, i64 884
  %390 = getelementptr inbounds nuw i8, ptr %self, i64 900
  %391 = getelementptr inbounds nuw i8, ptr %self, i64 916
  %392 = getelementptr inbounds nuw i8, ptr %self, i64 784
  %393 = getelementptr inbounds nuw i8, ptr %self, i64 788
  %394 = getelementptr inbounds nuw i8, ptr %self, i64 932
  %395 = getelementptr inbounds nuw i8, ptr %self, i64 936
  %396 = getelementptr inbounds nuw i8, ptr %self, i64 776
  %397 = getelementptr inbounds nuw i8, ptr %self, i64 940
  %398 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %399 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %injected.cond.not.i = icmp samesign ugt i64 %left.1, %right.1
  br i1 %injected.cond.not.i, label %bb32.i.i, label %bb30.i.lr.ph.split.us.i

bb30.i.lr.ph.split.us.i:                          ; preds = %bb27.i.i44
  %injected.cond663.not.i = icmp ugt i64 %_64.1.i25, %_66.1.i29
  br i1 %injected.cond663.not.i, label %bb30.i.us.i831, label %bb30.i.lr.ph.split.us.split.us.i

bb30.i.lr.ph.split.us.split.us.i:                 ; preds = %bb30.i.lr.ph.split.us.i
  %injected.cond688.not.i = icmp ugt i64 %_55, %side_left.sroa.5.0.i.i48
  br i1 %injected.cond688.not.i, label %bb32.i.us.us.i676, label %bb30.i.lr.ph.split.us.split.us.split.us.i

bb30.i.lr.ph.split.us.split.us.split.us.i:        ; preds = %bb30.i.lr.ph.split.us.split.us.i
  %injected.cond711.not.i = icmp ugt i64 %_64.1.i25, %_65.1.i27
  br i1 %injected.cond711.not.i, label %bb30.i.us.us.us.i521, label %bb30.i.lr.ph.split.us.split.us.split.us.split.us.i

bb30.i.lr.ph.split.us.split.us.split.us.split.us.i: ; preds = %bb30.i.lr.ph.split.us.split.us.split.us.i
  %injected.cond732.not.i = icmp ugt i64 %_55, %empty.sroa.6.0.i.i46
  br i1 %injected.cond732.not.i, label %bb32.i.us.us.us.us.i369, label %bb30.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i

bb30.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i: ; preds = %bb30.i.lr.ph.split.us.split.us.split.us.split.us.i
  %injected.cond751.not.i = icmp ugt i64 %_64.1.i25, %_67.1.i31
  br i1 %injected.cond751.not.i, label %bb30.i.us.us.us.us.us.i213, label %bb30.i.us.us.us.us.us.us.us.i50

bb30.i.us.us.us.us.us.us.us.i50:                  ; preds = %bb30.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i103
  %iter.sroa.0.0.i637.us.us.us.us.us.us.us.i = phi i64 [ %400, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i103 ], [ 0, %bb30.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %400 = add nuw nsw i64 %iter.sroa.0.0.i637.us.us.us.us.us.us.us.i, 1, !dbg !3924
  %_28.i.us.us.us.us.us.us.us.i51 = trunc i64 %iter.sroa.0.0.i637.us.us.us.us.us.us.us.i to i32, !dbg !3938
  %now.i.us.us.us.us.us.us.us.i52 = add i32 %base.i.i49, %_28.i.us.us.us.us.us.us.us.i51, !dbg !3941
  %_31.i.us.us.us.us.us.us.us.i53 = and i32 %now.i.us.us.us.us.us.us.us.i52, %_58.i33, !dbg !3944
  %_30.i.us.us.us.us.us.us.us.i54 = zext i32 %_31.i.us.us.us.us.us.us.us.i53 to i64, !dbg !3946
  %exitcond.not.i55 = icmp eq i64 %iter.sroa.0.0.i637.us.us.us.us.us.us.us.i, %_55, !dbg !3947
  br i1 %exitcond.not.i55, label %bb33.i.i211, label %bb32.i.us.us.us.us.us.us.us.i56, !dbg !3947, !prof !180

bb32.i.us.us.us.us.us.us.us.i56:                  ; preds = %bb30.i.us.us.us.us.us.us.us.i50
  %_97.i.us.us.us.us.us.us.us.i57 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i637.us.us.us.us.us.us.us.i, !dbg !3953
  %_98.not.not.i.us.us.us.us.us.us.us.i58 = icmp ugt i64 %_64.1.i25, %_30.i.us.us.us.us.us.us.us.i54, !dbg !3957
  br i1 %_98.not.not.i.us.us.us.us.us.us.us.i58, label %bb35.i.us.us.us.us.us.us.us.i60, label %bb36.i.i59, !dbg !3957, !prof !2709

bb35.i.us.us.us.us.us.us.us.i60:                  ; preds = %bb32.i.us.us.us.us.us.us.us.i56
  %_0.i201.us.us.us.us.us.us.us.i = load float, ptr %_97.i.us.us.us.us.us.us.us.i57, align 4, !dbg !3962, !alias.scope !3964, !noalias !3967, !noundef !12
  %_107.i.us.us.us.us.us.us.us.i61 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_30.i.us.us.us.us.us.us.us.i54, !dbg !3968
  store float %_0.i201.us.us.us.us.us.us.us.i, ptr %_107.i.us.us.us.us.us.us.us.i61, align 4, !dbg !3972, !alias.scope !3974, !noalias !3921
  %_115.i.us.us.us.us.us.us.us.i62 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i637.us.us.us.us.us.us.us.i, !dbg !3977
  %_0.i199.us.us.us.us.us.us.us.i = load float, ptr %_115.i.us.us.us.us.us.us.us.i62, align 4, !dbg !3984, !alias.scope !3986, !noalias !3989, !noundef !12
  %_123.i.us.us.us.us.us.us.us.i63 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_30.i.us.us.us.us.us.us.us.i54, !dbg !3990
  store float %_0.i199.us.us.us.us.us.us.us.i, ptr %_123.i.us.us.us.us.us.us.us.i63, align 4, !dbg !3997, !alias.scope !3999, !noalias !3921
  %_131.i.us.us.us.us.us.us.us.i64 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i47, i64 %iter.sroa.0.0.i637.us.us.us.us.us.us.us.i, !dbg !4002
  %_0.i197.us.us.us.us.us.us.us.i = load float, ptr %_131.i.us.us.us.us.us.us.us.i64, align 4, !dbg !4009, !alias.scope !4011, !noalias !3921, !noundef !12
  %_139.i.us.us.us.us.us.us.us.i65 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_30.i.us.us.us.us.us.us.us.i54, !dbg !4014
  store float %_0.i197.us.us.us.us.us.us.us.i, ptr %_139.i.us.us.us.us.us.us.us.i65, align 4, !dbg !4021, !alias.scope !4023, !noalias !3921
  %_147.i.us.us.us.us.us.us.us.i66 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i45, i64 %iter.sroa.0.0.i637.us.us.us.us.us.us.us.i, !dbg !4026
  %_0.i195.us.us.us.us.us.us.us.i = load float, ptr %_147.i.us.us.us.us.us.us.us.i66, align 4, !dbg !4033, !alias.scope !4035, !noalias !3921, !noundef !12
  %_155.i.us.us.us.us.us.us.us.i67 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_30.i.us.us.us.us.us.us.us.i54, !dbg !4038
  store float %_0.i195.us.us.us.us.us.us.us.i, ptr %_155.i.us.us.us.us.us.us.us.i67, align 4, !dbg !4045, !alias.scope !4047, !noalias !3921
  %_53.i.us.us.us.us.us.us.us.i68 = sub i32 %now.i.us.us.us.us.us.us.us.i52, %_59.i34, !dbg !4050
  %_52.i.us.us.us.us.us.us.us.i69 = and i32 %_53.i.us.us.us.us.us.us.us.i68, %_58.i33, !dbg !4053
  %_51.i.us.us.us.us.us.us.us.i70 = zext i32 %_52.i.us.us.us.us.us.us.us.i69 to i64, !dbg !4054
  %_156.not.not.i.us.us.us.us.us.us.us.i71 = icmp ugt i64 %_64.1.i25, %_51.i.us.us.us.us.us.us.us.i70, !dbg !4055
  br i1 %_156.not.not.i.us.us.us.us.us.us.us.i71, label %bb50.i.us.us.us.us.us.us.us.i73, label %bb51.i.i72, !dbg !4055, !prof !2709

bb50.i.us.us.us.us.us.us.us.i73:                  ; preds = %bb35.i.us.us.us.us.us.us.us.i60
  %_163.i.us.us.us.us.us.us.us.i74 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_51.i.us.us.us.us.us.us.us.i70, !dbg !4060
  %_0.i193.us.us.us.us.us.us.us.i75 = load float, ptr %_163.i.us.us.us.us.us.us.us.i74, align 4, !dbg !4064, !alias.scope !4066, !noalias !3921, !noundef !12
  %_169.i.us.us.us.us.us.us.us.i76 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_51.i.us.us.us.us.us.us.us.i70, !dbg !4069
  %_0.i191.us.us.us.us.us.us.us.i77 = load float, ptr %_169.i.us.us.us.us.us.us.us.i76, align 4, !dbg !4077, !alias.scope !4079, !noalias !3921, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4082), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4088), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4090), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4092), !dbg !4085
  %_18.i74.us.us.us.us.us.us.us.i78 = load i32, ptr %_31.i17, align 4, !dbg !4094, !alias.scope !4096, !noalias !4097, !noundef !12
  %_17.i.us.us.us.us.us.us.us.i79 = sub i32 %now.i.us.us.us.us.us.us.us.i52, %_18.i74.us.us.us.us.us.us.us.i78, !dbg !4099
  %_16.i75.us.us.us.us.us.us.us.i80 = and i32 %_17.i.us.us.us.us.us.us.us.i79, %_58.i33, !dbg !4094
  %_15.i76.us.us.us.us.us.us.us.i81 = zext i32 %_16.i75.us.us.us.us.us.us.us.i80 to i64, !dbg !4094
  %_26.i.us.us.us.us.us.us.us.i82 = load i32, ptr %_33.i18, align 4, !dbg !4094, !alias.scope !4101, !noalias !4102, !noundef !12
  %_25.i.us.us.us.us.us.us.us.i83 = sub i32 %now.i.us.us.us.us.us.us.us.i52, %_26.i.us.us.us.us.us.us.us.i82, !dbg !4099
  %_24.i.us.us.us.us.us.us.us.i84 = and i32 %_25.i.us.us.us.us.us.us.us.i83, %_58.i33, !dbg !4094
  %_23.i79.us.us.us.us.us.us.us.i85 = zext i32 %_24.i.us.us.us.us.us.us.us.i84 to i64, !dbg !4094
  %_31.i80.us.us.us.us.us.us.us.i86 = icmp samesign ugt i64 %_65.1.i27, %_15.i76.us.us.us.us.us.us.us.i81, !dbg !4094
  switch i8 %_0.sroa.0.0.i485.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.us.us.us.us.us.i202
    i8 1, label %bb14.i63.preheader.us.us.us.us.us.us.us.i194
    i8 2, label %bb23.i.preheader.us.us.us.us.us.us.us.i87
  ], !dbg !4103

bb27.i60.us.us.us.us.us.us.us.i89:                ; preds = %bb23.i.preheader.us.us.us.us.us.us.us.i87
  %401 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.us.us.us.i81, !dbg !4104
  %_79.i.us.us.us.us.us.us.us.i90 = load float, ptr %401, align 4, !dbg !4104, !alias.scope !4090, !noalias !4105, !noundef !12
  %_85.i.us.us.us.us.us.us.us.i91 = icmp samesign ugt i64 %_67.1.i31, %_15.i76.us.us.us.us.us.us.us.i81, !dbg !4106
  br i1 %_85.i.us.us.us.us.us.us.us.i91, label %bb29.i61.us.us.us.us.us.us.us.i93, label %panic30.i.i92, !dbg !4106

bb29.i61.us.us.us.us.us.us.us.i93:                ; preds = %bb27.i60.us.us.us.us.us.us.us.i89
  %402 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_15.i76.us.us.us.us.us.us.us.i81, !dbg !4106
  %_83.i.us.us.us.us.us.us.us.i94 = load float, ptr %402, align 4, !dbg !4106, !alias.scope !4092, !noalias !4107, !noundef !12
  %_87.i.us.us.us.us.us.us.us.i95 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.us.us.us.i85, !dbg !4108
  br i1 %_87.i.us.us.us.us.us.us.us.i95, label %bb31.i.us.us.us.us.us.us.us.i97, label %panic32.i.i96, !dbg !4108

bb31.i.us.us.us.us.us.us.us.i97:                  ; preds = %bb29.i61.us.us.us.us.us.us.us.i93
  %_89.i.us.us.us.us.us.us.us.i98 = icmp samesign ugt i64 %_65.1.i27, %_23.i79.us.us.us.us.us.us.us.i85, !dbg !4109
  br i1 %_89.i.us.us.us.us.us.us.us.i98, label %bb33.i62.us.us.us.us.us.us.us.i100, label %panic34.i.i99, !dbg !4109

bb33.i62.us.us.us.us.us.us.us.i100:               ; preds = %bb31.i.us.us.us.us.us.us.us.i97
  %403 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.us.us.us.i85, !dbg !4108
  %_86.i.us.us.us.us.us.us.us.i101 = load float, ptr %403, align 4, !dbg !4108, !alias.scope !4092, !noalias !4107, !noundef !12
  %404 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_23.i79.us.us.us.us.us.us.us.i85, !dbg !4109
  %_88.i.us.us.us.us.us.us.us.i102 = load float, ptr %404, align 4, !dbg !4109, !alias.scope !4090, !noalias !4105, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i103, !dbg !4110

bb17.i.us.us.us.us.us.us.us.i196:                 ; preds = %bb14.i63.preheader.us.us.us.us.us.us.us.i194
  %_59.i.us.us.us.us.us.us.us.i197 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.us.us.us.i85, !dbg !4113
  br i1 %_59.i.us.us.us.us.us.us.us.i197, label %bb19.i.us.us.us.us.us.us.us.i199, label %panic17.i.i198, !dbg !4113

bb19.i.us.us.us.us.us.us.us.i199:                 ; preds = %bb17.i.us.us.us.us.us.us.us.i196
  %405 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.us.us.us.i81, !dbg !4114
  %left_own16.i.us.us.us.us.us.us.us.i200 = load float, ptr %405, align 4, !dbg !4114, !alias.scope !4090, !noalias !4105, !noundef !12
  %406 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.us.us.us.i85, !dbg !4113
  %right_own18.i.us.us.us.us.us.us.us.i201 = load float, ptr %406, align 4, !dbg !4113, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i103, !dbg !4110

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i103: ; preds = %bb10.i82.us.us.us.us.us.us.us.i207, %bb19.i.us.us.us.us.us.us.us.i199, %bb33.i62.us.us.us.us.us.us.us.i100
  %taps.i.sroa.1011457.7.i = phi float [ %right_own.i.us.us.us.us.us.us.us.i209, %bb10.i82.us.us.us.us.us.us.us.i207 ], [ %left_own16.i.us.us.us.us.us.us.us.i200, %bb19.i.us.us.us.us.us.us.us.i199 ], [ %_88.i.us.us.us.us.us.us.us.i102, %bb33.i62.us.us.us.us.us.us.us.i100 ], !dbg !4094
  %taps.i.sroa.681456.7.i = phi float [ %right_own.i.us.us.us.us.us.us.us.i209, %bb10.i82.us.us.us.us.us.us.us.i207 ], [ %right_own18.i.us.us.us.us.us.us.us.i201, %bb19.i.us.us.us.us.us.us.us.i199 ], [ %_86.i.us.us.us.us.us.us.us.i101, %bb33.i62.us.us.us.us.us.us.us.i100 ], !dbg !4094
  %taps.i.sroa.351455.7.i = phi float [ %left_own.i.us.us.us.us.us.us.us.i208, %bb10.i82.us.us.us.us.us.us.us.i207 ], [ %right_own18.i.us.us.us.us.us.us.us.i201, %bb19.i.us.us.us.us.us.us.us.i199 ], [ %_83.i.us.us.us.us.us.us.us.i94, %bb33.i62.us.us.us.us.us.us.us.i100 ], !dbg !4094
  %taps.i.sroa.0.7.i104 = phi float [ %left_own.i.us.us.us.us.us.us.us.i208, %bb10.i82.us.us.us.us.us.us.us.i207 ], [ %left_own16.i.us.us.us.us.us.us.us.i200, %bb19.i.us.us.us.us.us.us.us.i199 ], [ %_79.i.us.us.us.us.us.us.us.i90, %bb33.i62.us.us.us.us.us.us.us.i100 ], !dbg !4094
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4115), !dbg !4118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4119), !dbg !4118
  %threshold.i30.i.us.us.us.us.us.us.us.i = load float, ptr %_10.i13, align 4, !dbg !4121, !alias.scope !4126, !noalias !4127, !noundef !12
  %ratio.i31.i.us.us.us.us.us.us.us.i = load float, ptr %378, align 4, !dbg !4128, !alias.scope !4126, !noalias !4127, !noundef !12
  %range.i32.i.us.us.us.us.us.us.us.i = load float, ptr %379, align 4, !dbg !4130, !alias.scope !4126, !noalias !4127, !noundef !12
  %hysteresis.i33.i.us.us.us.us.us.us.us.i = load float, ptr %380, align 4, !dbg !4132, !alias.scope !4126, !noalias !4127, !noundef !12
  %407 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.7.i104), !dbg !4134
  %408 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351455.7.i), !dbg !4137
  %_37.i36.i.us.us.us.us.us.us.us.i = load float, ptr %381, align 4, !dbg !4140, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i127.us.us.us.us.us.us.us.i105 = fcmp ule float %_37.i36.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4145
  %_3.i.i423.us.us.us.us.us.us.us.i = fcmp ule float %407, %408, !dbg !4147
  %_6.i.i425.us.us.us.us.us.us.us.i = bitcast float %407 to i32, !dbg !4150
  %_8.i.i427.us.us.us.us.us.us.us.i = bitcast float %408 to i32, !dbg !4153
  %_4.i.i430.us.us.us.us.us.us.us.i = select i1 %_3.i.i423.us.us.us.us.us.us.us.i, i32 %_8.i.i427.us.us.us.us.us.us.us.i, i32 %_6.i.i425.us.us.us.us.us.us.us.i, !dbg !4155
  %_4.i321.us.us.us.us.us.us.us.i106 = select i1 %_3.i127.us.us.us.us.us.us.us.i105, i32 %_6.i.i425.us.us.us.us.us.us.us.i, i32 %_4.i.i430.us.us.us.us.us.us.us.i, !dbg !4156
  %_41.i40.i.us.us.us.us.us.us.us.i = load float, ptr %382, align 4, !dbg !4158, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i125.us.us.us.us.us.us.us.i = fcmp ule float %_41.i40.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4159
  %_0.i167.us.us.us.us.us.us.us.i107 = fmul float %407, 5.000000e-01, !dbg !4161
  %_0.i166.us.us.us.us.us.us.us.i108 = fmul float %408, 5.000000e-01, !dbg !4163
  %_0.i142.us.us.us.us.us.us.us.i109 = fadd float %_0.i166.us.us.us.us.us.us.us.i108, %_0.i167.us.us.us.us.us.us.us.i107, !dbg !4165
  %_6.i309.us.us.us.us.us.us.us.i = bitcast float %_0.i142.us.us.us.us.us.us.us.i109 to i32, !dbg !4167
  %_4.i314.us.us.us.us.us.us.us.i = select i1 %_3.i125.us.us.us.us.us.us.us.i, i32 %_4.i321.us.us.us.us.us.us.us.i106, i32 %_6.i309.us.us.us.us.us.us.us.i, !dbg !4170
  %_0.i315.us.us.us.us.us.us.us.i = bitcast i32 %_4.i314.us.us.us.us.us.us.us.i to float, !dbg !4171
  %_3.i.i415.us.us.us.us.us.us.us.i = fcmp ule float %_0.i315.us.us.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4173
  %_4.i.i421.us.us.us.us.us.us.us.i = select i1 %_3.i.i415.us.us.us.us.us.us.us.i, i32 841731191, i32 %_4.i314.us.us.us.us.us.us.us.i, !dbg !4176
  %_0.i.i422.us.us.us.us.us.us.us.i110 = bitcast i32 %_4.i.i421.us.us.us.us.us.us.us.i to float, !dbg !4178
  %_3.i.i.us.us.us.us.us.us.us.i111 = fcmp ule float %_0.i.i422.us.us.us.us.us.us.us.i110, 0x3810000000000000, !dbg !4180
  %_4.i.i.us.us.us.us.us.us.us.i112 = select i1 %_3.i.i.us.us.us.us.us.us.us.i111, i32 8388608, i32 %_4.i.i421.us.us.us.us.us.us.us.i, !dbg !4186
  %_5.i202.us.us.us.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.us.us.us.i112, 8388607, !dbg !4188
  %_4.i203.us.us.us.us.us.us.us.i = or disjoint i32 %_5.i202.us.us.us.us.us.us.us.i, 1065353216, !dbg !4188
  %significand.i.us.us.us.us.us.us.us.i113 = bitcast i32 %_4.i203.us.us.us.us.us.us.us.i to float, !dbg !4190
  %_0.i168.us.us.us.us.us.us.us.i114 = fadd float %significand.i.us.us.us.us.us.us.us.i113, -1.000000e+00, !dbg !4192
  %_0.i148.us.us.us.us.us.us.us.i115 = fmul float %_0.i168.us.us.us.us.us.us.us.i114, 0x3F9B17A960000000, !dbg !4194
  %409 = fsub float 0x3FBF9A8440000000, %_0.i148.us.us.us.us.us.us.us.i115, !dbg !4196
  %_0.i148.us.us.us.us.us.us.us.1.i116 = fmul float %_0.i168.us.us.us.us.us.us.us.i114, %409, !dbg !4194
  %_0.i134.us.us.us.us.us.us.us.1.i = fadd float %_0.i148.us.us.us.us.us.us.us.1.i116, 0xBFD1E3F400000000, !dbg !4196
  %_0.i148.us.us.us.us.us.us.us.2.i117 = fmul float %_0.i168.us.us.us.us.us.us.us.i114, %_0.i134.us.us.us.us.us.us.us.1.i, !dbg !4194
  %_0.i134.us.us.us.us.us.us.us.2.i = fadd float %_0.i148.us.us.us.us.us.us.us.2.i117, 0x3FDD544F20000000, !dbg !4196
  %_0.i148.us.us.us.us.us.us.us.3.i118 = fmul float %_0.i168.us.us.us.us.us.us.us.i114, %_0.i134.us.us.us.us.us.us.us.2.i, !dbg !4194
  %_0.i134.us.us.us.us.us.us.us.3.i = fadd float %_0.i148.us.us.us.us.us.us.us.3.i118, 0xBFE6FC2A60000000, !dbg !4196
  %_0.i148.us.us.us.us.us.us.us.4.i = fmul float %_0.i168.us.us.us.us.us.us.us.i114, %_0.i134.us.us.us.us.us.us.us.3.i, !dbg !4194
  %_0.i134.us.us.us.us.us.us.us.4.i = fadd float %_0.i148.us.us.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4196
  %_9.i.us.us.us.us.us.us.us.i119 = lshr i32 %_4.i.i.us.us.us.us.us.us.us.i112, 23, !dbg !4198
  %_8.i204.us.us.us.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.us.us.us.i119, 1258291200, !dbg !4198
  %_7.i.us.us.us.us.us.us.us.i120 = bitcast i32 %_8.i204.us.us.us.us.us.us.us.i to float, !dbg !4199
  %exponent.i.us.us.us.us.us.us.us.i121 = fadd float %_7.i.us.us.us.us.us.us.us.i120, 0xC160000FE0000000, !dbg !4201
  %_0.i147.us.us.us.us.us.us.us.i122 = fmul float %_0.i168.us.us.us.us.us.us.us.i114, %_0.i134.us.us.us.us.us.us.us.4.i, !dbg !4202
  %_0.i133.us.us.us.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.us.us.us.i121, %_0.i147.us.us.us.us.us.us.us.i122, !dbg !4204
  %_0.i165.us.us.us.us.us.us.us.i123 = fmul float %_0.i133.us.us.us.us.us.us.us.i, 0x4018151820000000, !dbg !4206
  %_3.i.i472.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i165.us.us.us.us.us.us.us.i123, 2.400000e+01, !dbg !4208
  %_0.i.i479.us.us.us.us.us.us.us.i = select i1 %_3.i.i472.us.us.us.us.us.us.us.inv.i, float %_0.i165.us.us.us.us.us.us.us.i123, float 2.400000e+01, !dbg !4208
  %_3.i.i407.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i479.us.us.us.us.us.us.us.i, -1.600000e+02, !dbg !4211
  %_0.i.i414.us.us.us.us.us.us.us.i = select i1 %_3.i.i407.us.us.us.us.us.us.us.inv.i, float %_0.i.i479.us.us.us.us.us.us.us.i, float -1.600000e+02, !dbg !4211
  %_55.i57.i.us.us.us.us.us.us.us.i = load float, ptr %383, align 4, !dbg !4214, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i123.us.us.us.us.us.us.us.i124 = fcmp ule float %_55.i57.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4216
  %_3.i99.us.us.us.us.us.us.us.i125 = fcmp oge float %_0.i.i414.us.us.us.us.us.us.us.i, %threshold.i30.i.us.us.us.us.us.us.us.i, !dbg !4218
  %_0.i181.us.us.us.us.us.us.us.i126 = fsub float %threshold.i30.i.us.us.us.us.us.us.us.i, %hysteresis.i33.i.us.us.us.us.us.us.us.i, !dbg !4221
  %_3.i97.us.us.us.us.us.us.us.i127 = fcmp oge float %_0.i.i414.us.us.us.us.us.us.us.i, %_0.i181.us.us.us.us.us.us.us.i126, !dbg !4224
  %..i98.us.us.us.us.us.us.us.i128 = sext i1 %_3.i97.us.us.us.us.us.us.us.i127 to i32, !dbg !4226
  %_0.i335.us.us.us.us.us.us.us.i129 = sext i1 %_3.i99.us.us.us.us.us.us.us.i125 to i32, !dbg !4228
  %_0.i328.us.us.us.us.us.us.us.i130 = select i1 %_3.i123.us.us.us.us.us.us.us.i124, i32 %_0.i335.us.us.us.us.us.us.us.i129, i32 %..i98.us.us.us.us.us.us.us.i128, !dbg !4228
  %_0.i339.us.us.us.us.us.us.us.i = xor i32 %..i98.us.us.us.us.us.us.us.i128, -1, !dbg !4233
  %_67.i67.i.us.us.us.us.us.us.us.i = load float, ptr %384, align 4, !dbg !4236, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i121.us.us.us.us.us.us.us.i131 = fcmp ogt float %_67.i67.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4237
  %_0.i334.us.us.us.us.us.us.us.i = select i1 %_3.i121.us.us.us.us.us.us.us.i131, i32 %_0.i339.us.us.us.us.us.us.us.i, i32 0, !dbg !4239
  %_0.i333.us.us.us.us.us.us.us.i = select i1 %_3.i123.us.us.us.us.us.us.us.i124, i32 0, i32 %_0.i334.us.us.us.us.us.us.us.i, !dbg !4241
  %_0.i327.us.us.us.us.us.us.us.i = or i32 %_0.i333.us.us.us.us.us.us.us.i, %_0.i328.us.us.us.us.us.us.us.i130, !dbg !4243
  %_5.i304.us.us.us.us.us.us.us.i = and i32 %_0.i327.us.us.us.us.us.us.us.i, 1065353216, !dbg !4246
  %_0.i308.us.us.us.us.us.us.us.i = bitcast i32 %_5.i304.us.us.us.us.us.us.us.i to float, !dbg !4248
  %_71.i73.i493494.us.us.us.us.us.us.us.i = load float, ptr %385, align 4, !dbg !4250, !alias.scope !4143, !noalias !4144, !noundef !12
  %_0.i180.us.us.us.us.us.us.us.i132 = fadd float %_67.i67.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !4252
  %410 = trunc nsw i32 %_0.i333.us.us.us.us.us.us.us.i to i1, !dbg !4254
  %_4.i302.v.us.us.us.us.us.us.us.i = select i1 %410, float %_0.i180.us.us.us.us.us.us.us.i132, float %_67.i67.i.us.us.us.us.us.us.us.i, !dbg !4254
  %411 = trunc nsw i32 %_0.i328.us.us.us.us.us.us.us.i130 to i1, !dbg !4256
  %_0.i296.us.us.us.us.us.us.us.i = select i1 %411, float %_71.i73.i493494.us.us.us.us.us.us.us.i, float %_4.i302.v.us.us.us.us.us.us.us.i, !dbg !4256
  store float %_0.i296.us.us.us.us.us.us.us.i, ptr %384, align 4, !dbg !4258, !alias.scope !4126, !noalias !4127
  store i32 %_5.i304.us.us.us.us.us.us.us.i, ptr %383, align 4, !dbg !4259, !alias.scope !4126, !noalias !4127
  %_0.i179.us.us.us.us.us.us.us.i133 = fadd float %ratio.i31.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !4260
  %_0.i178.us.us.us.us.us.us.us.i134 = fsub float %_0.i.i414.us.us.us.us.us.us.us.i, %threshold.i30.i.us.us.us.us.us.us.us.i, !dbg !4262
  %_0.i164.us.us.us.us.us.us.us.i135 = fmul float %_0.i179.us.us.us.us.us.us.us.i133, %_0.i178.us.us.us.us.us.us.us.i134, !dbg !4264
  %412 = fneg float %range.i32.i.us.us.us.us.us.us.us.i, !dbg !4266
  %_3.i.i398.inv.us.us.us.us.us.us.us.i = fcmp ogt float %_0.i164.us.us.us.us.us.us.us.i135, %412, !dbg !4268
  %_4.i.i405.v.us.us.us.us.us.us.us.i = select i1 %_3.i.i398.inv.us.us.us.us.us.us.us.i, float %_0.i164.us.us.us.us.us.us.us.i135, float %412, !dbg !4268
  %_3.i.i464.us.us.us.us.us.us.us.i = fcmp olt float %_4.i.i405.v.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4271
  %413 = fcmp ule float %_0.i308.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4274
  %414 = select i1 %413, i1 %_3.i.i464.us.us.us.us.us.us.us.i, i1 false, !dbg !4277
  %_0.i289.us.us.us.us.us.us.us.i = select i1 %414, float %_4.i.i405.v.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !4277
  %_86.i86.i.us.us.us.us.us.us.us.i = load float, ptr %386, align 4, !dbg !4278, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i117.us.us.us.us.us.us.us.i136 = fcmp ule float %_0.i289.us.us.us.us.us.us.us.i, %_86.i86.i.us.us.us.us.us.us.us.i, !dbg !4280
  %_87.i88.i496.us.us.us.us.us.us.us.i = load i32, ptr %366, align 4, !dbg !4282, !alias.scope !4143, !noalias !4144, !noundef !12
  %_88.i89.i497.us.us.us.us.us.us.us.i = load i32, ptr %387, align 4, !dbg !4283, !alias.scope !4143, !noalias !4144, !noundef !12
  %_4.i282.us.us.us.us.us.us.us.i = select i1 %_3.i117.us.us.us.us.us.us.us.i136, i32 %_88.i89.i497.us.us.us.us.us.us.us.i, i32 %_87.i88.i496.us.us.us.us.us.us.us.i, !dbg !4284
  %_0.i283.us.us.us.us.us.us.us.i = bitcast i32 %_4.i282.us.us.us.us.us.us.us.i to float, !dbg !4286
  %_0.i177.us.us.us.us.us.us.us.i137 = fsub float %_0.i289.us.us.us.us.us.us.us.i, %_86.i86.i.us.us.us.us.us.us.us.i, !dbg !4288
  %_4.i145.us.us.us.us.us.us.us.i = fmul float %_0.i177.us.us.us.us.us.us.us.i137, %_0.i283.us.us.us.us.us.us.us.i, !dbg !4291
  %_0.i146.us.us.us.us.us.us.us.i138 = fadd float %_86.i86.i.us.us.us.us.us.us.us.i, %_4.i145.us.us.us.us.us.us.us.i, !dbg !4291
  %415 = tail call noundef float @llvm.fabs.f32(float %_0.i146.us.us.us.us.us.us.us.i138), !dbg !4293
  %416 = fcmp uge float %415, 0x3BC79CA100000000, !dbg !4297
  %_0.i218.us.us.us.us.us.us.us.i = select i1 %416, float %_0.i146.us.us.us.us.us.us.us.i138, float 0.000000e+00, !dbg !4299
  store float %_0.i218.us.us.us.us.us.us.us.i, ptr %386, align 4, !dbg !4300, !alias.scope !4126, !noalias !4127
  %_0.i163.us.us.us.us.us.us.us.i139 = fmul float %_0.i218.us.us.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !4302
  %_3.i.i349.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i163.us.us.us.us.us.us.us.i139, -1.260000e+02, !dbg !4306
  %_0.i.i356.us.us.us.us.us.us.us.i = select i1 %_3.i.i349.us.us.us.us.us.us.us.inv.i, float %_0.i163.us.us.us.us.us.us.us.i139, float -1.260000e+02, !dbg !4306
  %_3.i.i432.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i356.us.us.us.us.us.us.us.i, 1.270000e+02, !dbg !4310
  %_0.i.i439.us.us.us.us.us.us.us.i = select i1 %_3.i.i432.us.us.us.us.us.us.us.inv.i, float %_0.i.i356.us.us.us.us.us.us.us.i, float 1.270000e+02, !dbg !4310
  %417 = tail call noundef float @llvm.floor.f32(float %_0.i.i439.us.us.us.us.us.us.us.i), !dbg !4313
  %_0.i170.us.us.us.us.us.us.us.i140 = fsub float %_0.i.i439.us.us.us.us.us.us.us.i, %417, !dbg !4317
  %_98.i102.i.us.us.us.us.us.us.us.i = load float, ptr %388, align 4, !dbg !4319, !alias.scope !4143, !noalias !4144, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4321), !dbg !4324
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4326), !dbg !4324
  %threshold.i.i.us.us.us.us.us.us.us.i = load float, ptr %data.i.i.i14, align 4, !dbg !4328, !alias.scope !4330, !noalias !4331, !noundef !12
  %ratio.i.i.us.us.us.us.us.us.us.i = load float, ptr %389, align 4, !dbg !4332, !alias.scope !4330, !noalias !4331, !noundef !12
  %range.i.i.us.us.us.us.us.us.us.i = load float, ptr %390, align 4, !dbg !4333, !alias.scope !4330, !noalias !4331, !noundef !12
  %hysteresis.i.i.us.us.us.us.us.us.us.i = load float, ptr %391, align 4, !dbg !4334, !alias.scope !4330, !noalias !4331, !noundef !12
  %418 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681456.7.i), !dbg !4335
  %419 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011457.7.i), !dbg !4337
  %_37.i.i.us.us.us.us.us.us.us.i141 = load float, ptr %392, align 4, !dbg !4339, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i113.us.us.us.us.us.us.us.i142 = fcmp ule float %_37.i.i.us.us.us.us.us.us.us.i141, 0.000000e+00, !dbg !4342
  %_3.i.i389.us.us.us.us.us.us.us.i = fcmp ule float %418, %419, !dbg !4344
  %_6.i.i391.us.us.us.us.us.us.us.i = bitcast float %418 to i32, !dbg !4347
  %_8.i.i393.us.us.us.us.us.us.us.i = bitcast float %419 to i32, !dbg !4350
  %_4.i.i396.us.us.us.us.us.us.us.i = select i1 %_3.i.i389.us.us.us.us.us.us.us.i, i32 %_8.i.i393.us.us.us.us.us.us.us.i, i32 %_6.i.i391.us.us.us.us.us.us.us.i, !dbg !4352
  %_4.i268.us.us.us.us.us.us.us.i = select i1 %_3.i113.us.us.us.us.us.us.us.i142, i32 %_6.i.i391.us.us.us.us.us.us.us.i, i32 %_4.i.i396.us.us.us.us.us.us.us.i, !dbg !4353
  %_41.i.i.us.us.us.us.us.us.us.i143 = load float, ptr %393, align 4, !dbg !4355, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i111.us.us.us.us.us.us.us.i144 = fcmp ule float %_41.i.i.us.us.us.us.us.us.us.i143, 0.000000e+00, !dbg !4356
  %_0.i161.us.us.us.us.us.us.us.i145 = fmul float %418, 5.000000e-01, !dbg !4358
  %_0.i160.us.us.us.us.us.us.us.i146 = fmul float %419, 5.000000e-01, !dbg !4360
  %_0.i141.us.us.us.us.us.us.us.i147 = fadd float %_0.i160.us.us.us.us.us.us.us.i146, %_0.i161.us.us.us.us.us.us.us.i145, !dbg !4362
  %_6.i256.us.us.us.us.us.us.us.i = bitcast float %_0.i141.us.us.us.us.us.us.us.i147 to i32, !dbg !4364
  %_4.i261.us.us.us.us.us.us.us.i = select i1 %_3.i111.us.us.us.us.us.us.us.i144, i32 %_4.i268.us.us.us.us.us.us.us.i, i32 %_6.i256.us.us.us.us.us.us.us.i, !dbg !4367
  %_0.i262.us.us.us.us.us.us.us.i = bitcast i32 %_4.i261.us.us.us.us.us.us.us.i to float, !dbg !4368
  %_3.i.i381.us.us.us.us.us.us.us.i = fcmp ule float %_0.i262.us.us.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4370
  %_4.i.i387.us.us.us.us.us.us.us.i = select i1 %_3.i.i381.us.us.us.us.us.us.us.i, i32 841731191, i32 %_4.i261.us.us.us.us.us.us.us.i, !dbg !4373
  %_0.i.i388.us.us.us.us.us.us.us.i = bitcast i32 %_4.i.i387.us.us.us.us.us.us.us.i to float, !dbg !4375
  %_3.i.i341.us.us.us.us.us.us.us.i = fcmp ule float %_0.i.i388.us.us.us.us.us.us.us.i, 0x3810000000000000, !dbg !4377
  %_4.i.i347.us.us.us.us.us.us.us.i = select i1 %_3.i.i341.us.us.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i387.us.us.us.us.us.us.us.i, !dbg !4382
  %_5.i206.us.us.us.us.us.us.us.i = and i32 %_4.i.i347.us.us.us.us.us.us.us.i, 8388607, !dbg !4384
  %_4.i207.us.us.us.us.us.us.us.i = or disjoint i32 %_5.i206.us.us.us.us.us.us.us.i, 1065353216, !dbg !4384
  %significand.i208.us.us.us.us.us.us.us.i = bitcast i32 %_4.i207.us.us.us.us.us.us.us.i to float, !dbg !4386
  %_0.i169.us.us.us.us.us.us.us.i148 = fadd float %significand.i208.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !4388
  %_0.i150.us.us.us.us.us.us.us.i149 = fmul float %_0.i169.us.us.us.us.us.us.us.i148, 0x3F9B17A960000000, !dbg !4390
  %420 = fsub float 0x3FBF9A8440000000, %_0.i150.us.us.us.us.us.us.us.i149, !dbg !4392
  %_0.i150.us.us.us.us.us.us.us.1.i150 = fmul float %_0.i169.us.us.us.us.us.us.us.i148, %420, !dbg !4390
  %_0.i136.us.us.us.us.us.us.us.1.i = fadd float %_0.i150.us.us.us.us.us.us.us.1.i150, 0xBFD1E3F400000000, !dbg !4392
  %_0.i150.us.us.us.us.us.us.us.2.i151 = fmul float %_0.i169.us.us.us.us.us.us.us.i148, %_0.i136.us.us.us.us.us.us.us.1.i, !dbg !4390
  %_0.i136.us.us.us.us.us.us.us.2.i = fadd float %_0.i150.us.us.us.us.us.us.us.2.i151, 0x3FDD544F20000000, !dbg !4392
  %_0.i150.us.us.us.us.us.us.us.3.i152 = fmul float %_0.i169.us.us.us.us.us.us.us.i148, %_0.i136.us.us.us.us.us.us.us.2.i, !dbg !4390
  %_0.i136.us.us.us.us.us.us.us.3.i = fadd float %_0.i150.us.us.us.us.us.us.us.3.i152, 0xBFE6FC2A60000000, !dbg !4392
  %_0.i150.us.us.us.us.us.us.us.4.i = fmul float %_0.i169.us.us.us.us.us.us.us.i148, %_0.i136.us.us.us.us.us.us.us.3.i, !dbg !4390
  %_0.i136.us.us.us.us.us.us.us.4.i = fadd float %_0.i150.us.us.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4392
  %_9.i209.us.us.us.us.us.us.us.i = lshr i32 %_4.i.i347.us.us.us.us.us.us.us.i, 23, !dbg !4394
  %_8.i210.us.us.us.us.us.us.us.i = or disjoint i32 %_9.i209.us.us.us.us.us.us.us.i, 1258291200, !dbg !4394
  %_7.i211.us.us.us.us.us.us.us.i = bitcast i32 %_8.i210.us.us.us.us.us.us.us.i to float, !dbg !4395
  %exponent.i212.us.us.us.us.us.us.us.i = fadd float %_7.i211.us.us.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !4397
  %_0.i149.us.us.us.us.us.us.us.i153 = fmul float %_0.i169.us.us.us.us.us.us.us.i148, %_0.i136.us.us.us.us.us.us.us.4.i, !dbg !4398
  %_0.i135.us.us.us.us.us.us.us.i = fadd float %exponent.i212.us.us.us.us.us.us.us.i, %_0.i149.us.us.us.us.us.us.us.i153, !dbg !4400
  %_0.i159.us.us.us.us.us.us.us.i154 = fmul float %_0.i135.us.us.us.us.us.us.us.i, 0x4018151820000000, !dbg !4402
  %_3.i.i456.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i159.us.us.us.us.us.us.us.i154, 2.400000e+01, !dbg !4404
  %_0.i.i463.us.us.us.us.us.us.us.i = select i1 %_3.i.i456.us.us.us.us.us.us.us.inv.i, float %_0.i159.us.us.us.us.us.us.us.i154, float 2.400000e+01, !dbg !4404
  %_3.i.i373.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i463.us.us.us.us.us.us.us.i, -1.600000e+02, !dbg !4407
  %_0.i.i380.us.us.us.us.us.us.us.i = select i1 %_3.i.i373.us.us.us.us.us.us.us.inv.i, float %_0.i.i463.us.us.us.us.us.us.us.i, float -1.600000e+02, !dbg !4407
  %_55.i.i.us.us.us.us.us.us.us.i155 = load float, ptr %394, align 4, !dbg !4410, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i109.us.us.us.us.us.us.us.i = fcmp ule float %_55.i.i.us.us.us.us.us.us.us.i155, 0.000000e+00, !dbg !4411
  %_3.i95.us.us.us.us.us.us.us.i156 = fcmp oge float %_0.i.i380.us.us.us.us.us.us.us.i, %threshold.i.i.us.us.us.us.us.us.us.i, !dbg !4413
  %_0.i176.us.us.us.us.us.us.us.i157 = fsub float %threshold.i.i.us.us.us.us.us.us.us.i, %hysteresis.i.i.us.us.us.us.us.us.us.i, !dbg !4415
  %_3.i93.us.us.us.us.us.us.us.i158 = fcmp oge float %_0.i.i380.us.us.us.us.us.us.us.i, %_0.i176.us.us.us.us.us.us.us.i157, !dbg !4417
  %..i94.us.us.us.us.us.us.us.i = sext i1 %_3.i93.us.us.us.us.us.us.us.i158 to i32, !dbg !4419
  %_0.i331.us.us.us.us.us.us.us.i = sext i1 %_3.i95.us.us.us.us.us.us.us.i156 to i32, !dbg !4421
  %_0.i325.us.us.us.us.us.us.us.i = select i1 %_3.i109.us.us.us.us.us.us.us.i, i32 %_0.i331.us.us.us.us.us.us.us.i, i32 %..i94.us.us.us.us.us.us.us.i, !dbg !4421
  %_0.i337.us.us.us.us.us.us.us.i = xor i32 %..i94.us.us.us.us.us.us.us.i, -1, !dbg !4423
  %_67.i.i.us.us.us.us.us.us.us.i159 = load float, ptr %395, align 4, !dbg !4425, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i107.us.us.us.us.us.us.us.i160 = fcmp ogt float %_67.i.i.us.us.us.us.us.us.us.i159, 0.000000e+00, !dbg !4426
  %_0.i330.us.us.us.us.us.us.us.i = select i1 %_3.i107.us.us.us.us.us.us.us.i160, i32 %_0.i337.us.us.us.us.us.us.us.i, i32 0, !dbg !4428
  %_0.i329.us.us.us.us.us.us.us.i = select i1 %_3.i109.us.us.us.us.us.us.us.i, i32 0, i32 %_0.i330.us.us.us.us.us.us.us.i, !dbg !4430
  %_0.i324.us.us.us.us.us.us.us.i = or i32 %_0.i329.us.us.us.us.us.us.us.i, %_0.i325.us.us.us.us.us.us.us.i, !dbg !4432
  %_5.i251.us.us.us.us.us.us.us.i = and i32 %_0.i324.us.us.us.us.us.us.us.i, 1065353216, !dbg !4434
  %_0.i255.us.us.us.us.us.us.us.i161 = bitcast i32 %_5.i251.us.us.us.us.us.us.us.i to float, !dbg !4436
  %_71.i.i510511.us.us.us.us.us.us.us.i = load float, ptr %396, align 4, !dbg !4438, !alias.scope !4340, !noalias !4341, !noundef !12
  %_0.i175.us.us.us.us.us.us.us.i162 = fadd float %_67.i.i.us.us.us.us.us.us.us.i159, -1.000000e+00, !dbg !4439
  %421 = trunc nsw i32 %_0.i329.us.us.us.us.us.us.us.i to i1, !dbg !4441
  %_4.i249.v.us.us.us.us.us.us.us.i = select i1 %421, float %_0.i175.us.us.us.us.us.us.us.i162, float %_67.i.i.us.us.us.us.us.us.us.i159, !dbg !4441
  %422 = trunc nsw i32 %_0.i325.us.us.us.us.us.us.us.i to i1, !dbg !4443
  %_0.i243.us.us.us.us.us.us.us.i = select i1 %422, float %_71.i.i510511.us.us.us.us.us.us.us.i, float %_4.i249.v.us.us.us.us.us.us.us.i, !dbg !4443
  store float %_0.i243.us.us.us.us.us.us.us.i, ptr %395, align 4, !dbg !4445, !alias.scope !4330, !noalias !4331
  store i32 %_5.i251.us.us.us.us.us.us.us.i, ptr %394, align 4, !dbg !4446, !alias.scope !4330, !noalias !4331
  %_0.i174.us.us.us.us.us.us.us.i163 = fadd float %ratio.i.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !4447
  %_0.i173.us.us.us.us.us.us.us.i164 = fsub float %_0.i.i380.us.us.us.us.us.us.us.i, %threshold.i.i.us.us.us.us.us.us.us.i, !dbg !4449
  %_0.i158.us.us.us.us.us.us.us.i165 = fmul float %_0.i174.us.us.us.us.us.us.us.i163, %_0.i173.us.us.us.us.us.us.us.i164, !dbg !4451
  %423 = fneg float %range.i.i.us.us.us.us.us.us.us.i, !dbg !4453
  %_3.i.i365.inv.us.us.us.us.us.us.us.i = fcmp ogt float %_0.i158.us.us.us.us.us.us.us.i165, %423, !dbg !4455
  %_4.i.i371.v.us.us.us.us.us.us.us.i = select i1 %_3.i.i365.inv.us.us.us.us.us.us.us.i, float %_0.i158.us.us.us.us.us.us.us.i165, float %423, !dbg !4455
  %_3.i.i448.us.us.us.us.us.us.us.i = fcmp olt float %_4.i.i371.v.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4458
  %424 = fcmp ule float %_0.i255.us.us.us.us.us.us.us.i161, 0.000000e+00, !dbg !4461
  %425 = select i1 %424, i1 %_3.i.i448.us.us.us.us.us.us.us.i, i1 false, !dbg !4463
  %_0.i236.us.us.us.us.us.us.us.i = select i1 %425, float %_4.i.i371.v.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !4463
  %_86.i.i.us.us.us.us.us.us.us.i166 = load float, ptr %397, align 4, !dbg !4464, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i103.us.us.us.us.us.us.us.i167 = fcmp ule float %_0.i236.us.us.us.us.us.us.us.i, %_86.i.i.us.us.us.us.us.us.us.i166, !dbg !4465
  %_87.i.i513.us.us.us.us.us.us.us.i = load i32, ptr %_38.i23, align 4, !dbg !4467, !alias.scope !4340, !noalias !4341, !noundef !12
  %_88.i.i514.us.us.us.us.us.us.us.i = load i32, ptr %398, align 4, !dbg !4468, !alias.scope !4340, !noalias !4341, !noundef !12
  %_4.i229.us.us.us.us.us.us.us.i = select i1 %_3.i103.us.us.us.us.us.us.us.i167, i32 %_88.i.i514.us.us.us.us.us.us.us.i, i32 %_87.i.i513.us.us.us.us.us.us.us.i, !dbg !4469
  %_0.i230.us.us.us.us.us.us.us.i168 = bitcast i32 %_4.i229.us.us.us.us.us.us.us.i to float, !dbg !4471
  %_0.i172.us.us.us.us.us.us.us.i169 = fsub float %_0.i236.us.us.us.us.us.us.us.i, %_86.i.i.us.us.us.us.us.us.us.i166, !dbg !4473
  %_4.i143.us.us.us.us.us.us.us.i = fmul float %_0.i172.us.us.us.us.us.us.us.i169, %_0.i230.us.us.us.us.us.us.us.i168, !dbg !4475
  %_0.i144.us.us.us.us.us.us.us.i170 = fadd float %_86.i.i.us.us.us.us.us.us.us.i166, %_4.i143.us.us.us.us.us.us.us.i, !dbg !4475
  %426 = tail call noundef float @llvm.fabs.f32(float %_0.i144.us.us.us.us.us.us.us.i170), !dbg !4477
  %427 = fcmp uge float %426, 0x3BC79CA100000000, !dbg !4480
  %_0.i214.us.us.us.us.us.us.us.i = select i1 %427, float %_0.i144.us.us.us.us.us.us.us.i170, float 0.000000e+00, !dbg !4482
  store float %_0.i214.us.us.us.us.us.us.us.i, ptr %397, align 4, !dbg !4483, !alias.scope !4330, !noalias !4331
  %_0.i157.us.us.us.us.us.us.us.i171 = fmul float %_0.i214.us.us.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !4484
  %_3.i.i357.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i157.us.us.us.us.us.us.us.i171, -1.260000e+02, !dbg !4487
  %_0.i.i364.us.us.us.us.us.us.us.i = select i1 %_3.i.i357.us.us.us.us.us.us.us.inv.i, float %_0.i157.us.us.us.us.us.us.us.i171, float -1.260000e+02, !dbg !4487
  %_3.i.i440.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i364.us.us.us.us.us.us.us.i, 1.270000e+02, !dbg !4491
  %_0.i.i447.us.us.us.us.us.us.us.i = select i1 %_3.i.i440.us.us.us.us.us.us.us.inv.i, float %_0.i.i364.us.us.us.us.us.us.us.i, float 1.270000e+02, !dbg !4491
  %428 = tail call noundef float @llvm.floor.f32(float %_0.i.i447.us.us.us.us.us.us.us.i), !dbg !4494
  %_0.i171.us.us.us.us.us.us.us.i172 = fsub float %_0.i.i447.us.us.us.us.us.us.us.i, %428, !dbg !4498
  %_0.i155.us.us.us.us.us.us.us.i = fmul float %_0.i171.us.us.us.us.us.us.us.i172, 0x3F5E974FA0000000, !dbg !4500
  %_0.i140.us.us.us.us.us.us.us.i = fadd float %_0.i155.us.us.us.us.us.us.us.i, 0x3F82778560000000, !dbg !4502
  %_0.i155.us.us.us.us.us.us.us.1.i = fmul float %_0.i171.us.us.us.us.us.us.us.i172, %_0.i140.us.us.us.us.us.us.us.i, !dbg !4500
  %_0.i140.us.us.us.us.us.us.us.1.i = fadd float %_0.i155.us.us.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4502
  %_0.i155.us.us.us.us.us.us.us.2.i = fmul float %_0.i171.us.us.us.us.us.us.us.i172, %_0.i140.us.us.us.us.us.us.us.1.i, !dbg !4500
  %_0.i140.us.us.us.us.us.us.us.2.i = fadd float %_0.i155.us.us.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4502
  %_0.i155.us.us.us.us.us.us.us.3.i = fmul float %_0.i171.us.us.us.us.us.us.us.i172, %_0.i140.us.us.us.us.us.us.us.2.i, !dbg !4500
  %_0.i140.us.us.us.us.us.us.us.3.i = fadd float %_0.i155.us.us.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4502
  %_0.i153.us.us.us.us.us.us.us.i = fmul float %_0.i170.us.us.us.us.us.us.us.i140, 0x3F5E974FA0000000, !dbg !4504
  %_0.i138.us.us.us.us.us.us.us.i = fadd float %_0.i153.us.us.us.us.us.us.us.i, 0x3F82778560000000, !dbg !4506
  %_0.i153.us.us.us.us.us.us.us.1.i = fmul float %_0.i170.us.us.us.us.us.us.us.i140, %_0.i138.us.us.us.us.us.us.us.i, !dbg !4504
  %_0.i138.us.us.us.us.us.us.us.1.i = fadd float %_0.i153.us.us.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4506
  %_0.i153.us.us.us.us.us.us.us.2.i = fmul float %_0.i170.us.us.us.us.us.us.us.i140, %_0.i138.us.us.us.us.us.us.us.1.i, !dbg !4504
  %_0.i138.us.us.us.us.us.us.us.2.i = fadd float %_0.i153.us.us.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4506
  %_0.i153.us.us.us.us.us.us.us.3.i = fmul float %_0.i170.us.us.us.us.us.us.us.i140, %_0.i138.us.us.us.us.us.us.us.2.i, !dbg !4504
  %_0.i138.us.us.us.us.us.us.us.3.i = fadd float %_0.i153.us.us.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4506
  %_0.i152.us.us.us.us.us.us.us.i173 = fmul float %_0.i170.us.us.us.us.us.us.us.i140, %_0.i138.us.us.us.us.us.us.us.3.i, !dbg !4508
  %_0.i137.us.us.us.us.us.us.us.i = fadd float %_0.i152.us.us.us.us.us.us.us.i173, 1.000000e+00, !dbg !4510
  %biased.i.us.us.us.us.us.us.us.i174 = fadd float %417, 0x4160000FE0000000, !dbg !4512
  %_4.i83.us.us.us.us.us.us.us.i175 = bitcast float %biased.i.us.us.us.us.us.us.us.i174 to i32, !dbg !4514
  %_3.i84.us.us.us.us.us.us.us.i176 = shl i32 %_4.i83.us.us.us.us.us.us.us.i175, 23, !dbg !4516
  %_0.i85.us.us.us.us.us.us.us.i177 = bitcast i32 %_3.i84.us.us.us.us.us.us.us.i176 to float, !dbg !4517
  %_0.i151.us.us.us.us.us.us.us.i178 = fmul float %_0.i137.us.us.us.us.us.us.us.i, %_0.i85.us.us.us.us.us.us.us.i177, !dbg !4519
  %_3.i91.us.us.us.us.us.us.us.i179 = fcmp une float %_0.i218.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4521
  %_3.i115.us.us.us.us.us.us.us.i180 = fcmp ule float %_98.i102.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4523
  %_0.i326501.not.us.us.us.us.us.us.us.i = and i1 %_3.i115.us.us.us.us.us.us.us.i180, %_3.i91.us.us.us.us.us.us.us.i179, !dbg !4525
  %_0.i162.us.us.us.us.us.us.us.i181 = fmul float %_0.i193.us.us.us.us.us.us.us.i75, %_0.i151.us.us.us.us.us.us.us.i178, !dbg !4525
  %_4.i275.v.us.us.us.us.us.us.us.i = select i1 %_0.i326501.not.us.us.us.us.us.us.us.i, float %_0.i162.us.us.us.us.us.us.us.i181, float %_0.i193.us.us.us.us.us.us.us.i75, !dbg !4528
  %_0.i.us.us.us.us.us.us.us.i182 = fmul float %_0.i171.us.us.us.us.us.us.us.i172, %_0.i140.us.us.us.us.us.us.us.3.i, !dbg !4530
  %_0.i139.us.us.us.us.us.us.us.i = fadd float %_0.i.us.us.us.us.us.us.us.i182, 1.000000e+00, !dbg !4532
  %biased.i86.us.us.us.us.us.us.us.i183 = fadd float %428, 0x4160000FE0000000, !dbg !4534
  %_4.i87.us.us.us.us.us.us.us.i184 = bitcast float %biased.i86.us.us.us.us.us.us.us.i183 to i32, !dbg !4536
  %_3.i88.us.us.us.us.us.us.us.i185 = shl i32 %_4.i87.us.us.us.us.us.us.us.i184, 23, !dbg !4538
  %_0.i89.us.us.us.us.us.us.us.i186 = bitcast i32 %_3.i88.us.us.us.us.us.us.us.i185 to float, !dbg !4539
  %_0.i154.us.us.us.us.us.us.us.i187 = fmul float %_0.i139.us.us.us.us.us.us.us.i, %_0.i89.us.us.us.us.us.us.us.i186, !dbg !4541
  %_3.i90.us.us.us.us.us.us.us.i188 = fcmp une float %_0.i214.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4543
  %_98.i.i.us.us.us.us.us.us.us.i189 = load float, ptr %399, align 4, !dbg !4545, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i101.us.us.us.us.us.us.us.i190 = fcmp ule float %_98.i.i.us.us.us.us.us.us.us.i189, 0.000000e+00, !dbg !4546
  %_0.i323518.not.us.us.us.us.us.us.us.i = and i1 %_3.i101.us.us.us.us.us.us.us.i190, %_3.i90.us.us.us.us.us.us.us.i188, !dbg !4548
  %_0.i156.us.us.us.us.us.us.us.i191 = fmul float %_0.i191.us.us.us.us.us.us.us.i77, %_0.i154.us.us.us.us.us.us.us.i187, !dbg !4548
  %_4.i222.v.us.us.us.us.us.us.us.i = select i1 %_0.i323518.not.us.us.us.us.us.us.us.i, float %_0.i156.us.us.us.us.us.us.us.i191, float %_0.i191.us.us.us.us.us.us.us.i77, !dbg !4550
  store float %_4.i275.v.us.us.us.us.us.us.us.i, ptr %_97.i.us.us.us.us.us.us.us.i57, align 4, !dbg !4552, !alias.scope !4555, !noalias !3967
  store float %_4.i222.v.us.us.us.us.us.us.us.i, ptr %_115.i.us.us.us.us.us.us.us.i62, align 4, !dbg !4558, !alias.scope !4560, !noalias !3989
  %exitcond1434.not.i = icmp eq i64 %400, %_26, !dbg !4563
  br i1 %exitcond1434.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb30.i.us.us.us.us.us.us.us.i50, !dbg !4566

bb8.i81.us.us.us.us.us.us.us.i204:                ; preds = %bb5.i.preheader.us.us.us.us.us.us.us.i202
  %_34.i.us.us.us.us.us.us.us.i205 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.us.us.us.i85, !dbg !4567
  br i1 %_34.i.us.us.us.us.us.us.us.i205, label %bb10.i82.us.us.us.us.us.us.us.i207, label %panic5.i.i206, !dbg !4567

bb10.i82.us.us.us.us.us.us.us.i207:               ; preds = %bb8.i81.us.us.us.us.us.us.us.i204
  %429 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.us.us.us.i81, !dbg !4568
  %left_own.i.us.us.us.us.us.us.us.i208 = load float, ptr %429, align 4, !dbg !4568, !alias.scope !4090, !noalias !4105, !noundef !12
  %430 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.us.us.us.i85, !dbg !4567
  %right_own.i.us.us.us.us.us.us.us.i209 = load float, ptr %430, align 4, !dbg !4567, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i103, !dbg !4110

bb5.i.preheader.us.us.us.us.us.us.us.i202:        ; preds = %bb50.i.us.us.us.us.us.us.us.i73
  br i1 %_31.i80.us.us.us.us.us.us.us.i86, label %bb8.i81.us.us.us.us.us.us.us.i204, label %panic4.i.i203, !dbg !4568

bb14.i63.preheader.us.us.us.us.us.us.us.i194:     ; preds = %bb50.i.us.us.us.us.us.us.us.i73
  br i1 %_31.i80.us.us.us.us.us.us.us.i86, label %bb17.i.us.us.us.us.us.us.us.i196, label %panic15.i.i195, !dbg !4114

bb23.i.preheader.us.us.us.us.us.us.us.i87:        ; preds = %bb50.i.us.us.us.us.us.us.us.i73
  br i1 %_31.i80.us.us.us.us.us.us.us.i86, label %bb27.i60.us.us.us.us.us.us.us.i89, label %panic28.i.i88, !dbg !4104

bb30.i.us.us.us.us.us.i213:                       ; preds = %bb30.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i267
  %iter.sroa.0.0.i637.us.us.us.us.us.i = phi i64 [ %431, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i267 ], [ 0, %bb30.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %431 = add nuw nsw i64 %iter.sroa.0.0.i637.us.us.us.us.us.i, 1, !dbg !3924
  %_28.i.us.us.us.us.us.i214 = trunc i64 %iter.sroa.0.0.i637.us.us.us.us.us.i to i32, !dbg !3938
  %now.i.us.us.us.us.us.i215 = add i32 %base.i.i49, %_28.i.us.us.us.us.us.i214, !dbg !3941
  %_31.i.us.us.us.us.us.i216 = and i32 %now.i.us.us.us.us.us.i215, %_58.i33, !dbg !3944
  %_30.i.us.us.us.us.us.i217 = zext i32 %_31.i.us.us.us.us.us.i216 to i64, !dbg !3946
  %exitcond1437.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.us.us.us.us.i, %_55, !dbg !3947
  br i1 %exitcond1437.not.i, label %bb33.i.i211, label %bb32.i.us.us.us.us.us.i218, !dbg !3947, !prof !180

bb32.i.us.us.us.us.us.i218:                       ; preds = %bb30.i.us.us.us.us.us.i213
  %_97.i.us.us.us.us.us.i219 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i637.us.us.us.us.us.i, !dbg !3953
  %_98.not.not.i.us.us.us.us.us.i220 = icmp ugt i64 %_64.1.i25, %_30.i.us.us.us.us.us.i217, !dbg !3957
  br i1 %_98.not.not.i.us.us.us.us.us.i220, label %bb35.i.us.us.us.us.us.i221, label %bb36.i.i59, !dbg !3957, !prof !2709

bb35.i.us.us.us.us.us.i221:                       ; preds = %bb32.i.us.us.us.us.us.i218
  %_0.i201.us.us.us.us.us.i = load float, ptr %_97.i.us.us.us.us.us.i219, align 4, !dbg !3962, !alias.scope !3964, !noalias !3967, !noundef !12
  %_107.i.us.us.us.us.us.i222 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_30.i.us.us.us.us.us.i217, !dbg !3968
  store float %_0.i201.us.us.us.us.us.i, ptr %_107.i.us.us.us.us.us.i222, align 4, !dbg !3972, !alias.scope !3974, !noalias !3921
  %_115.i.us.us.us.us.us.i223 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i637.us.us.us.us.us.i, !dbg !3977
  %_0.i199.us.us.us.us.us.i = load float, ptr %_115.i.us.us.us.us.us.i223, align 4, !dbg !3984, !alias.scope !3986, !noalias !3989, !noundef !12
  %_123.i.us.us.us.us.us.i224 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_30.i.us.us.us.us.us.i217, !dbg !3990
  store float %_0.i199.us.us.us.us.us.i, ptr %_123.i.us.us.us.us.us.i224, align 4, !dbg !3997, !alias.scope !3999, !noalias !3921
  %_131.i.us.us.us.us.us.i225 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i47, i64 %iter.sroa.0.0.i637.us.us.us.us.us.i, !dbg !4002
  %_0.i197.us.us.us.us.us.i = load float, ptr %_131.i.us.us.us.us.us.i225, align 4, !dbg !4009, !alias.scope !4011, !noalias !3921, !noundef !12
  %_139.i.us.us.us.us.us.i226 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_30.i.us.us.us.us.us.i217, !dbg !4014
  store float %_0.i197.us.us.us.us.us.i, ptr %_139.i.us.us.us.us.us.i226, align 4, !dbg !4021, !alias.scope !4023, !noalias !3921
  %_148.not.not.i.us.us.us.us.us.i227 = icmp ugt i64 %_67.1.i31, %_30.i.us.us.us.us.us.i217, !dbg !4569
  br i1 %_148.not.not.i.us.us.us.us.us.i227, label %bb48.i.us.us.us.us.us.i230, label %bb49.i.i228, !dbg !4569, !prof !2709

bb48.i.us.us.us.us.us.i230:                       ; preds = %bb35.i.us.us.us.us.us.i221
  %_147.i.us.us.us.us.us.i231 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i45, i64 %iter.sroa.0.0.i637.us.us.us.us.us.i, !dbg !4026
  %_0.i195.us.us.us.us.us.i = load float, ptr %_147.i.us.us.us.us.us.i231, align 4, !dbg !4033, !alias.scope !4035, !noalias !3921, !noundef !12
  %_155.i.us.us.us.us.us.i232 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_30.i.us.us.us.us.us.i217, !dbg !4038
  store float %_0.i195.us.us.us.us.us.i, ptr %_155.i.us.us.us.us.us.i232, align 4, !dbg !4045, !alias.scope !4047, !noalias !3921
  %_53.i.us.us.us.us.us.i233 = sub i32 %now.i.us.us.us.us.us.i215, %_59.i34, !dbg !4050
  %_52.i.us.us.us.us.us.i234 = and i32 %_53.i.us.us.us.us.us.i233, %_58.i33, !dbg !4053
  %_51.i.us.us.us.us.us.i235 = zext i32 %_52.i.us.us.us.us.us.i234 to i64, !dbg !4054
  %_156.not.not.i.us.us.us.us.us.i236 = icmp ugt i64 %_64.1.i25, %_51.i.us.us.us.us.us.i235, !dbg !4055
  br i1 %_156.not.not.i.us.us.us.us.us.i236, label %bb50.i.us.us.us.us.us.i237, label %bb51.i.i72, !dbg !4055, !prof !2709

bb50.i.us.us.us.us.us.i237:                       ; preds = %bb48.i.us.us.us.us.us.i230
  %_163.i.us.us.us.us.us.i238 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_51.i.us.us.us.us.us.i235, !dbg !4060
  %_0.i193.us.us.us.us.us.i239 = load float, ptr %_163.i.us.us.us.us.us.i238, align 4, !dbg !4064, !alias.scope !4066, !noalias !3921, !noundef !12
  %_169.i.us.us.us.us.us.i244 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_51.i.us.us.us.us.us.i235, !dbg !4069
  %_0.i191.us.us.us.us.us.i245 = load float, ptr %_169.i.us.us.us.us.us.i244, align 4, !dbg !4077, !alias.scope !4079, !noalias !3921, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4082), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4088), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4090), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4092), !dbg !4085
  %_18.i74.us.us.us.us.us.i246 = load i32, ptr %_31.i17, align 4, !dbg !4094, !alias.scope !4096, !noalias !4097, !noundef !12
  %_17.i.us.us.us.us.us.i247 = sub i32 %now.i.us.us.us.us.us.i215, %_18.i74.us.us.us.us.us.i246, !dbg !4099
  %_16.i75.us.us.us.us.us.i248 = and i32 %_17.i.us.us.us.us.us.i247, %_58.i33, !dbg !4094
  %_15.i76.us.us.us.us.us.i249 = zext i32 %_16.i75.us.us.us.us.us.i248 to i64, !dbg !4094
  %_26.i.us.us.us.us.us.i250 = load i32, ptr %_33.i18, align 4, !dbg !4094, !alias.scope !4101, !noalias !4102, !noundef !12
  %_25.i.us.us.us.us.us.i251 = sub i32 %now.i.us.us.us.us.us.i215, %_26.i.us.us.us.us.us.i250, !dbg !4099
  %_24.i.us.us.us.us.us.i252 = and i32 %_25.i.us.us.us.us.us.i251, %_58.i33, !dbg !4094
  %_23.i79.us.us.us.us.us.i253 = zext i32 %_24.i.us.us.us.us.us.i252 to i64, !dbg !4094
  %_31.i80.us.us.us.us.us.i254 = icmp samesign ugt i64 %_65.1.i27, %_15.i76.us.us.us.us.us.i249, !dbg !4094
  switch i8 %_0.sroa.0.0.i485.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.us.us.us.i362
    i8 1, label %bb14.i63.preheader.us.us.us.us.us.i356
    i8 2, label %bb23.i.preheader.us.us.us.us.us.i255
  ], !dbg !4103

bb27.i60.us.us.us.us.us.i256:                     ; preds = %bb23.i.preheader.us.us.us.us.us.i255
  %432 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.us.i249, !dbg !4104
  %_79.i.us.us.us.us.us.i257 = load float, ptr %432, align 4, !dbg !4104, !alias.scope !4090, !noalias !4105, !noundef !12
  %_85.i.us.us.us.us.us.i258 = icmp samesign ugt i64 %_67.1.i31, %_15.i76.us.us.us.us.us.i249, !dbg !4106
  br i1 %_85.i.us.us.us.us.us.i258, label %bb29.i61.us.us.us.us.us.i259, label %panic30.i.i92, !dbg !4106

bb29.i61.us.us.us.us.us.i259:                     ; preds = %bb27.i60.us.us.us.us.us.i256
  %_87.i.us.us.us.us.us.i261 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.us.i253, !dbg !4108
  br i1 %_87.i.us.us.us.us.us.i261, label %bb31.i.us.us.us.us.us.i262, label %panic32.i.i96, !dbg !4108

bb31.i.us.us.us.us.us.i262:                       ; preds = %bb29.i61.us.us.us.us.us.i259
  %433 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_15.i76.us.us.us.us.us.i249, !dbg !4106
  %_83.i.us.us.us.us.us.i260 = load float, ptr %433, align 4, !dbg !4106, !alias.scope !4092, !noalias !4107, !noundef !12
  %434 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.us.i253, !dbg !4108
  %_86.i.us.us.us.us.us.i265 = load float, ptr %434, align 4, !dbg !4108, !alias.scope !4092, !noalias !4107, !noundef !12
  %435 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_23.i79.us.us.us.us.us.i253, !dbg !4109
  %_88.i.us.us.us.us.us.i266 = load float, ptr %435, align 4, !dbg !4109, !alias.scope !4090, !noalias !4105, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i267, !dbg !4110

bb17.i.us.us.us.us.us.i357:                       ; preds = %bb14.i63.preheader.us.us.us.us.us.i356
  %_59.i.us.us.us.us.us.i358 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.us.i253, !dbg !4113
  br i1 %_59.i.us.us.us.us.us.i358, label %bb19.i.us.us.us.us.us.i359, label %panic17.i.i198, !dbg !4113

bb19.i.us.us.us.us.us.i359:                       ; preds = %bb17.i.us.us.us.us.us.i357
  %436 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.us.i249, !dbg !4114
  %left_own16.i.us.us.us.us.us.i360 = load float, ptr %436, align 4, !dbg !4114, !alias.scope !4090, !noalias !4105, !noundef !12
  %437 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.us.i253, !dbg !4113
  %right_own18.i.us.us.us.us.us.i361 = load float, ptr %437, align 4, !dbg !4113, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i267, !dbg !4110

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i267: ; preds = %bb10.i82.us.us.us.us.us.i365, %bb19.i.us.us.us.us.us.i359, %bb31.i.us.us.us.us.us.i262
  %taps.i.sroa.1011457.5.i = phi float [ %right_own.i.us.us.us.us.us.i367, %bb10.i82.us.us.us.us.us.i365 ], [ %left_own16.i.us.us.us.us.us.i360, %bb19.i.us.us.us.us.us.i359 ], [ %_88.i.us.us.us.us.us.i266, %bb31.i.us.us.us.us.us.i262 ], !dbg !4094
  %taps.i.sroa.681456.5.i = phi float [ %right_own.i.us.us.us.us.us.i367, %bb10.i82.us.us.us.us.us.i365 ], [ %right_own18.i.us.us.us.us.us.i361, %bb19.i.us.us.us.us.us.i359 ], [ %_86.i.us.us.us.us.us.i265, %bb31.i.us.us.us.us.us.i262 ], !dbg !4094
  %taps.i.sroa.351455.5.i = phi float [ %left_own.i.us.us.us.us.us.i366, %bb10.i82.us.us.us.us.us.i365 ], [ %right_own18.i.us.us.us.us.us.i361, %bb19.i.us.us.us.us.us.i359 ], [ %_83.i.us.us.us.us.us.i260, %bb31.i.us.us.us.us.us.i262 ], !dbg !4094
  %taps.i.sroa.0.5.i268 = phi float [ %left_own.i.us.us.us.us.us.i366, %bb10.i82.us.us.us.us.us.i365 ], [ %left_own16.i.us.us.us.us.us.i360, %bb19.i.us.us.us.us.us.i359 ], [ %_79.i.us.us.us.us.us.i257, %bb31.i.us.us.us.us.us.i262 ], !dbg !4094
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4115), !dbg !4118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4119), !dbg !4118
  %threshold.i30.i.us.us.us.us.us.i = load float, ptr %_10.i13, align 4, !dbg !4121, !alias.scope !4126, !noalias !4127, !noundef !12
  %ratio.i31.i.us.us.us.us.us.i = load float, ptr %378, align 4, !dbg !4128, !alias.scope !4126, !noalias !4127, !noundef !12
  %range.i32.i.us.us.us.us.us.i = load float, ptr %379, align 4, !dbg !4130, !alias.scope !4126, !noalias !4127, !noundef !12
  %hysteresis.i33.i.us.us.us.us.us.i = load float, ptr %380, align 4, !dbg !4132, !alias.scope !4126, !noalias !4127, !noundef !12
  %438 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.5.i268), !dbg !4134
  %439 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351455.5.i), !dbg !4137
  %_37.i36.i.us.us.us.us.us.i = load float, ptr %381, align 4, !dbg !4140, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i127.us.us.us.us.us.i269 = fcmp ule float %_37.i36.i.us.us.us.us.us.i, 0.000000e+00, !dbg !4145
  %_3.i.i423.us.us.us.us.us.i = fcmp ule float %438, %439, !dbg !4147
  %_6.i.i425.us.us.us.us.us.i = bitcast float %438 to i32, !dbg !4150
  %_8.i.i427.us.us.us.us.us.i = bitcast float %439 to i32, !dbg !4153
  %_4.i.i430.us.us.us.us.us.i = select i1 %_3.i.i423.us.us.us.us.us.i, i32 %_8.i.i427.us.us.us.us.us.i, i32 %_6.i.i425.us.us.us.us.us.i, !dbg !4155
  %_4.i321.us.us.us.us.us.i270 = select i1 %_3.i127.us.us.us.us.us.i269, i32 %_6.i.i425.us.us.us.us.us.i, i32 %_4.i.i430.us.us.us.us.us.i, !dbg !4156
  %_41.i40.i.us.us.us.us.us.i = load float, ptr %382, align 4, !dbg !4158, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i125.us.us.us.us.us.i = fcmp ule float %_41.i40.i.us.us.us.us.us.i, 0.000000e+00, !dbg !4159
  %_0.i167.us.us.us.us.us.i271 = fmul float %438, 5.000000e-01, !dbg !4161
  %_0.i166.us.us.us.us.us.i272 = fmul float %439, 5.000000e-01, !dbg !4163
  %_0.i142.us.us.us.us.us.i273 = fadd float %_0.i166.us.us.us.us.us.i272, %_0.i167.us.us.us.us.us.i271, !dbg !4165
  %_6.i309.us.us.us.us.us.i = bitcast float %_0.i142.us.us.us.us.us.i273 to i32, !dbg !4167
  %_4.i314.us.us.us.us.us.i = select i1 %_3.i125.us.us.us.us.us.i, i32 %_4.i321.us.us.us.us.us.i270, i32 %_6.i309.us.us.us.us.us.i, !dbg !4170
  %_0.i315.us.us.us.us.us.i = bitcast i32 %_4.i314.us.us.us.us.us.i to float, !dbg !4171
  %_3.i.i415.us.us.us.us.us.i = fcmp ule float %_0.i315.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4173
  %_4.i.i421.us.us.us.us.us.i = select i1 %_3.i.i415.us.us.us.us.us.i, i32 841731191, i32 %_4.i314.us.us.us.us.us.i, !dbg !4176
  %_0.i.i422.us.us.us.us.us.i274 = bitcast i32 %_4.i.i421.us.us.us.us.us.i to float, !dbg !4178
  %_3.i.i.us.us.us.us.us.i275 = fcmp ule float %_0.i.i422.us.us.us.us.us.i274, 0x3810000000000000, !dbg !4180
  %_4.i.i.us.us.us.us.us.i276 = select i1 %_3.i.i.us.us.us.us.us.i275, i32 8388608, i32 %_4.i.i421.us.us.us.us.us.i, !dbg !4186
  %_5.i202.us.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.us.i276, 8388607, !dbg !4188
  %_4.i203.us.us.us.us.us.i = or disjoint i32 %_5.i202.us.us.us.us.us.i, 1065353216, !dbg !4188
  %significand.i.us.us.us.us.us.i277 = bitcast i32 %_4.i203.us.us.us.us.us.i to float, !dbg !4190
  %_0.i168.us.us.us.us.us.i278 = fadd float %significand.i.us.us.us.us.us.i277, -1.000000e+00, !dbg !4192
  %_0.i148.us.us.us.us.us.i279 = fmul float %_0.i168.us.us.us.us.us.i278, 0x3F9B17A960000000, !dbg !4194
  %440 = fsub float 0x3FBF9A8440000000, %_0.i148.us.us.us.us.us.i279, !dbg !4196
  %_0.i148.us.us.us.us.us.1.i280 = fmul float %_0.i168.us.us.us.us.us.i278, %440, !dbg !4194
  %_0.i134.us.us.us.us.us.1.i = fadd float %_0.i148.us.us.us.us.us.1.i280, 0xBFD1E3F400000000, !dbg !4196
  %_0.i148.us.us.us.us.us.2.i281 = fmul float %_0.i168.us.us.us.us.us.i278, %_0.i134.us.us.us.us.us.1.i, !dbg !4194
  %_0.i134.us.us.us.us.us.2.i = fadd float %_0.i148.us.us.us.us.us.2.i281, 0x3FDD544F20000000, !dbg !4196
  %_0.i148.us.us.us.us.us.3.i282 = fmul float %_0.i168.us.us.us.us.us.i278, %_0.i134.us.us.us.us.us.2.i, !dbg !4194
  %_0.i134.us.us.us.us.us.3.i = fadd float %_0.i148.us.us.us.us.us.3.i282, 0xBFE6FC2A60000000, !dbg !4196
  %_0.i148.us.us.us.us.us.4.i = fmul float %_0.i168.us.us.us.us.us.i278, %_0.i134.us.us.us.us.us.3.i, !dbg !4194
  %_0.i134.us.us.us.us.us.4.i = fadd float %_0.i148.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4196
  %_9.i.us.us.us.us.us.i283 = lshr i32 %_4.i.i.us.us.us.us.us.i276, 23, !dbg !4198
  %_8.i204.us.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.us.i283, 1258291200, !dbg !4198
  %_7.i.us.us.us.us.us.i284 = bitcast i32 %_8.i204.us.us.us.us.us.i to float, !dbg !4199
  %exponent.i.us.us.us.us.us.i285 = fadd float %_7.i.us.us.us.us.us.i284, 0xC160000FE0000000, !dbg !4201
  %_0.i147.us.us.us.us.us.i286 = fmul float %_0.i168.us.us.us.us.us.i278, %_0.i134.us.us.us.us.us.4.i, !dbg !4202
  %_0.i133.us.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.us.i285, %_0.i147.us.us.us.us.us.i286, !dbg !4204
  %_0.i165.us.us.us.us.us.i287 = fmul float %_0.i133.us.us.us.us.us.i, 0x4018151820000000, !dbg !4206
  %_3.i.i472.us.us.us.us.us.inv.i = fcmp olt float %_0.i165.us.us.us.us.us.i287, 2.400000e+01, !dbg !4208
  %_0.i.i479.us.us.us.us.us.i = select i1 %_3.i.i472.us.us.us.us.us.inv.i, float %_0.i165.us.us.us.us.us.i287, float 2.400000e+01, !dbg !4208
  %_3.i.i407.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i479.us.us.us.us.us.i, -1.600000e+02, !dbg !4211
  %_0.i.i414.us.us.us.us.us.i = select i1 %_3.i.i407.us.us.us.us.us.inv.i, float %_0.i.i479.us.us.us.us.us.i, float -1.600000e+02, !dbg !4211
  %_55.i57.i.us.us.us.us.us.i = load float, ptr %383, align 4, !dbg !4214, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i123.us.us.us.us.us.i288 = fcmp ule float %_55.i57.i.us.us.us.us.us.i, 0.000000e+00, !dbg !4216
  %_3.i99.us.us.us.us.us.i289 = fcmp oge float %_0.i.i414.us.us.us.us.us.i, %threshold.i30.i.us.us.us.us.us.i, !dbg !4218
  %_0.i181.us.us.us.us.us.i290 = fsub float %threshold.i30.i.us.us.us.us.us.i, %hysteresis.i33.i.us.us.us.us.us.i, !dbg !4221
  %_3.i97.us.us.us.us.us.i291 = fcmp oge float %_0.i.i414.us.us.us.us.us.i, %_0.i181.us.us.us.us.us.i290, !dbg !4224
  %..i98.us.us.us.us.us.i292 = sext i1 %_3.i97.us.us.us.us.us.i291 to i32, !dbg !4226
  %_0.i335.us.us.us.us.us.i293 = sext i1 %_3.i99.us.us.us.us.us.i289 to i32, !dbg !4228
  %_0.i328.us.us.us.us.us.i294 = select i1 %_3.i123.us.us.us.us.us.i288, i32 %_0.i335.us.us.us.us.us.i293, i32 %..i98.us.us.us.us.us.i292, !dbg !4228
  %_0.i339.us.us.us.us.us.i = xor i32 %..i98.us.us.us.us.us.i292, -1, !dbg !4233
  %_67.i67.i.us.us.us.us.us.i = load float, ptr %384, align 4, !dbg !4236, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i121.us.us.us.us.us.i295 = fcmp ogt float %_67.i67.i.us.us.us.us.us.i, 0.000000e+00, !dbg !4237
  %_0.i334.us.us.us.us.us.i = select i1 %_3.i121.us.us.us.us.us.i295, i32 %_0.i339.us.us.us.us.us.i, i32 0, !dbg !4239
  %_0.i333.us.us.us.us.us.i = select i1 %_3.i123.us.us.us.us.us.i288, i32 0, i32 %_0.i334.us.us.us.us.us.i, !dbg !4241
  %_0.i327.us.us.us.us.us.i = or i32 %_0.i333.us.us.us.us.us.i, %_0.i328.us.us.us.us.us.i294, !dbg !4243
  %_5.i304.us.us.us.us.us.i = and i32 %_0.i327.us.us.us.us.us.i, 1065353216, !dbg !4246
  %_0.i308.us.us.us.us.us.i = bitcast i32 %_5.i304.us.us.us.us.us.i to float, !dbg !4248
  %_71.i73.i493494.us.us.us.us.us.i = load float, ptr %385, align 4, !dbg !4250, !alias.scope !4143, !noalias !4144, !noundef !12
  %_0.i180.us.us.us.us.us.i296 = fadd float %_67.i67.i.us.us.us.us.us.i, -1.000000e+00, !dbg !4252
  %441 = trunc nsw i32 %_0.i333.us.us.us.us.us.i to i1, !dbg !4254
  %_4.i302.v.us.us.us.us.us.i = select i1 %441, float %_0.i180.us.us.us.us.us.i296, float %_67.i67.i.us.us.us.us.us.i, !dbg !4254
  %442 = trunc nsw i32 %_0.i328.us.us.us.us.us.i294 to i1, !dbg !4256
  %_0.i296.us.us.us.us.us.i = select i1 %442, float %_71.i73.i493494.us.us.us.us.us.i, float %_4.i302.v.us.us.us.us.us.i, !dbg !4256
  store float %_0.i296.us.us.us.us.us.i, ptr %384, align 4, !dbg !4258, !alias.scope !4126, !noalias !4127
  store i32 %_5.i304.us.us.us.us.us.i, ptr %383, align 4, !dbg !4259, !alias.scope !4126, !noalias !4127
  %_0.i179.us.us.us.us.us.i297 = fadd float %ratio.i31.i.us.us.us.us.us.i, -1.000000e+00, !dbg !4260
  %_0.i178.us.us.us.us.us.i298 = fsub float %_0.i.i414.us.us.us.us.us.i, %threshold.i30.i.us.us.us.us.us.i, !dbg !4262
  %_0.i164.us.us.us.us.us.i299 = fmul float %_0.i179.us.us.us.us.us.i297, %_0.i178.us.us.us.us.us.i298, !dbg !4264
  %443 = fneg float %range.i32.i.us.us.us.us.us.i, !dbg !4266
  %_3.i.i398.inv.us.us.us.us.us.i = fcmp ogt float %_0.i164.us.us.us.us.us.i299, %443, !dbg !4268
  %_4.i.i405.v.us.us.us.us.us.i = select i1 %_3.i.i398.inv.us.us.us.us.us.i, float %_0.i164.us.us.us.us.us.i299, float %443, !dbg !4268
  %_3.i.i464.us.us.us.us.us.i = fcmp olt float %_4.i.i405.v.us.us.us.us.us.i, 0.000000e+00, !dbg !4271
  %444 = fcmp ule float %_0.i308.us.us.us.us.us.i, 0.000000e+00, !dbg !4274
  %445 = select i1 %444, i1 %_3.i.i464.us.us.us.us.us.i, i1 false, !dbg !4277
  %_0.i289.us.us.us.us.us.i = select i1 %445, float %_4.i.i405.v.us.us.us.us.us.i, float 0.000000e+00, !dbg !4277
  %_86.i86.i.us.us.us.us.us.i = load float, ptr %386, align 4, !dbg !4278, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i117.us.us.us.us.us.i300 = fcmp ule float %_0.i289.us.us.us.us.us.i, %_86.i86.i.us.us.us.us.us.i, !dbg !4280
  %_87.i88.i496.us.us.us.us.us.i = load i32, ptr %366, align 4, !dbg !4282, !alias.scope !4143, !noalias !4144, !noundef !12
  %_88.i89.i497.us.us.us.us.us.i = load i32, ptr %387, align 4, !dbg !4283, !alias.scope !4143, !noalias !4144, !noundef !12
  %_4.i282.us.us.us.us.us.i = select i1 %_3.i117.us.us.us.us.us.i300, i32 %_88.i89.i497.us.us.us.us.us.i, i32 %_87.i88.i496.us.us.us.us.us.i, !dbg !4284
  %_0.i283.us.us.us.us.us.i = bitcast i32 %_4.i282.us.us.us.us.us.i to float, !dbg !4286
  %_0.i177.us.us.us.us.us.i301 = fsub float %_0.i289.us.us.us.us.us.i, %_86.i86.i.us.us.us.us.us.i, !dbg !4288
  %_4.i145.us.us.us.us.us.i = fmul float %_0.i177.us.us.us.us.us.i301, %_0.i283.us.us.us.us.us.i, !dbg !4291
  %_0.i146.us.us.us.us.us.i302 = fadd float %_86.i86.i.us.us.us.us.us.i, %_4.i145.us.us.us.us.us.i, !dbg !4291
  %446 = tail call noundef float @llvm.fabs.f32(float %_0.i146.us.us.us.us.us.i302), !dbg !4293
  %447 = fcmp uge float %446, 0x3BC79CA100000000, !dbg !4297
  %_0.i218.us.us.us.us.us.i = select i1 %447, float %_0.i146.us.us.us.us.us.i302, float 0.000000e+00, !dbg !4299
  store float %_0.i218.us.us.us.us.us.i, ptr %386, align 4, !dbg !4300, !alias.scope !4126, !noalias !4127
  %_0.i163.us.us.us.us.us.i303 = fmul float %_0.i218.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !4302
  %_3.i.i349.us.us.us.us.us.inv.i = fcmp ogt float %_0.i163.us.us.us.us.us.i303, -1.260000e+02, !dbg !4306
  %_0.i.i356.us.us.us.us.us.i = select i1 %_3.i.i349.us.us.us.us.us.inv.i, float %_0.i163.us.us.us.us.us.i303, float -1.260000e+02, !dbg !4306
  %_3.i.i432.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i356.us.us.us.us.us.i, 1.270000e+02, !dbg !4310
  %_0.i.i439.us.us.us.us.us.i = select i1 %_3.i.i432.us.us.us.us.us.inv.i, float %_0.i.i356.us.us.us.us.us.i, float 1.270000e+02, !dbg !4310
  %448 = tail call noundef float @llvm.floor.f32(float %_0.i.i439.us.us.us.us.us.i), !dbg !4313
  %_0.i170.us.us.us.us.us.i304 = fsub float %_0.i.i439.us.us.us.us.us.i, %448, !dbg !4317
  %_98.i102.i.us.us.us.us.us.i = load float, ptr %388, align 4, !dbg !4319, !alias.scope !4143, !noalias !4144, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4321), !dbg !4324
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4326), !dbg !4324
  %threshold.i.i.us.us.us.us.us.i = load float, ptr %data.i.i.i14, align 4, !dbg !4328, !alias.scope !4330, !noalias !4331, !noundef !12
  %ratio.i.i.us.us.us.us.us.i = load float, ptr %389, align 4, !dbg !4332, !alias.scope !4330, !noalias !4331, !noundef !12
  %range.i.i.us.us.us.us.us.i = load float, ptr %390, align 4, !dbg !4333, !alias.scope !4330, !noalias !4331, !noundef !12
  %hysteresis.i.i.us.us.us.us.us.i = load float, ptr %391, align 4, !dbg !4334, !alias.scope !4330, !noalias !4331, !noundef !12
  %449 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681456.5.i), !dbg !4335
  %450 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011457.5.i), !dbg !4337
  %_37.i.i.us.us.us.us.us.i305 = load float, ptr %392, align 4, !dbg !4339, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i113.us.us.us.us.us.i306 = fcmp ule float %_37.i.i.us.us.us.us.us.i305, 0.000000e+00, !dbg !4342
  %_3.i.i389.us.us.us.us.us.i = fcmp ule float %449, %450, !dbg !4344
  %_6.i.i391.us.us.us.us.us.i = bitcast float %449 to i32, !dbg !4347
  %_8.i.i393.us.us.us.us.us.i = bitcast float %450 to i32, !dbg !4350
  %_4.i.i396.us.us.us.us.us.i = select i1 %_3.i.i389.us.us.us.us.us.i, i32 %_8.i.i393.us.us.us.us.us.i, i32 %_6.i.i391.us.us.us.us.us.i, !dbg !4352
  %_4.i268.us.us.us.us.us.i = select i1 %_3.i113.us.us.us.us.us.i306, i32 %_6.i.i391.us.us.us.us.us.i, i32 %_4.i.i396.us.us.us.us.us.i, !dbg !4353
  %_41.i.i.us.us.us.us.us.i307 = load float, ptr %393, align 4, !dbg !4355, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i111.us.us.us.us.us.i308 = fcmp ule float %_41.i.i.us.us.us.us.us.i307, 0.000000e+00, !dbg !4356
  %_0.i161.us.us.us.us.us.i309 = fmul float %449, 5.000000e-01, !dbg !4358
  %_0.i160.us.us.us.us.us.i310 = fmul float %450, 5.000000e-01, !dbg !4360
  %_0.i141.us.us.us.us.us.i311 = fadd float %_0.i160.us.us.us.us.us.i310, %_0.i161.us.us.us.us.us.i309, !dbg !4362
  %_6.i256.us.us.us.us.us.i = bitcast float %_0.i141.us.us.us.us.us.i311 to i32, !dbg !4364
  %_4.i261.us.us.us.us.us.i = select i1 %_3.i111.us.us.us.us.us.i308, i32 %_4.i268.us.us.us.us.us.i, i32 %_6.i256.us.us.us.us.us.i, !dbg !4367
  %_0.i262.us.us.us.us.us.i = bitcast i32 %_4.i261.us.us.us.us.us.i to float, !dbg !4368
  %_3.i.i381.us.us.us.us.us.i = fcmp ule float %_0.i262.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4370
  %_4.i.i387.us.us.us.us.us.i = select i1 %_3.i.i381.us.us.us.us.us.i, i32 841731191, i32 %_4.i261.us.us.us.us.us.i, !dbg !4373
  %_0.i.i388.us.us.us.us.us.i = bitcast i32 %_4.i.i387.us.us.us.us.us.i to float, !dbg !4375
  %_3.i.i341.us.us.us.us.us.i = fcmp ule float %_0.i.i388.us.us.us.us.us.i, 0x3810000000000000, !dbg !4377
  %_4.i.i347.us.us.us.us.us.i = select i1 %_3.i.i341.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i387.us.us.us.us.us.i, !dbg !4382
  %_5.i206.us.us.us.us.us.i = and i32 %_4.i.i347.us.us.us.us.us.i, 8388607, !dbg !4384
  %_4.i207.us.us.us.us.us.i = or disjoint i32 %_5.i206.us.us.us.us.us.i, 1065353216, !dbg !4384
  %significand.i208.us.us.us.us.us.i = bitcast i32 %_4.i207.us.us.us.us.us.i to float, !dbg !4386
  %_0.i169.us.us.us.us.us.i312 = fadd float %significand.i208.us.us.us.us.us.i, -1.000000e+00, !dbg !4388
  %_0.i150.us.us.us.us.us.i313 = fmul float %_0.i169.us.us.us.us.us.i312, 0x3F9B17A960000000, !dbg !4390
  %451 = fsub float 0x3FBF9A8440000000, %_0.i150.us.us.us.us.us.i313, !dbg !4392
  %_0.i150.us.us.us.us.us.1.i314 = fmul float %_0.i169.us.us.us.us.us.i312, %451, !dbg !4390
  %_0.i136.us.us.us.us.us.1.i = fadd float %_0.i150.us.us.us.us.us.1.i314, 0xBFD1E3F400000000, !dbg !4392
  %_0.i150.us.us.us.us.us.2.i315 = fmul float %_0.i169.us.us.us.us.us.i312, %_0.i136.us.us.us.us.us.1.i, !dbg !4390
  %_0.i136.us.us.us.us.us.2.i = fadd float %_0.i150.us.us.us.us.us.2.i315, 0x3FDD544F20000000, !dbg !4392
  %_0.i150.us.us.us.us.us.3.i316 = fmul float %_0.i169.us.us.us.us.us.i312, %_0.i136.us.us.us.us.us.2.i, !dbg !4390
  %_0.i136.us.us.us.us.us.3.i = fadd float %_0.i150.us.us.us.us.us.3.i316, 0xBFE6FC2A60000000, !dbg !4392
  %_0.i150.us.us.us.us.us.4.i = fmul float %_0.i169.us.us.us.us.us.i312, %_0.i136.us.us.us.us.us.3.i, !dbg !4390
  %_0.i136.us.us.us.us.us.4.i = fadd float %_0.i150.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4392
  %_9.i209.us.us.us.us.us.i = lshr i32 %_4.i.i347.us.us.us.us.us.i, 23, !dbg !4394
  %_8.i210.us.us.us.us.us.i = or disjoint i32 %_9.i209.us.us.us.us.us.i, 1258291200, !dbg !4394
  %_7.i211.us.us.us.us.us.i = bitcast i32 %_8.i210.us.us.us.us.us.i to float, !dbg !4395
  %exponent.i212.us.us.us.us.us.i = fadd float %_7.i211.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !4397
  %_0.i149.us.us.us.us.us.i317 = fmul float %_0.i169.us.us.us.us.us.i312, %_0.i136.us.us.us.us.us.4.i, !dbg !4398
  %_0.i135.us.us.us.us.us.i = fadd float %exponent.i212.us.us.us.us.us.i, %_0.i149.us.us.us.us.us.i317, !dbg !4400
  %_0.i159.us.us.us.us.us.i318 = fmul float %_0.i135.us.us.us.us.us.i, 0x4018151820000000, !dbg !4402
  %_3.i.i456.us.us.us.us.us.inv.i = fcmp olt float %_0.i159.us.us.us.us.us.i318, 2.400000e+01, !dbg !4404
  %_0.i.i463.us.us.us.us.us.i = select i1 %_3.i.i456.us.us.us.us.us.inv.i, float %_0.i159.us.us.us.us.us.i318, float 2.400000e+01, !dbg !4404
  %_3.i.i373.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i463.us.us.us.us.us.i, -1.600000e+02, !dbg !4407
  %_0.i.i380.us.us.us.us.us.i = select i1 %_3.i.i373.us.us.us.us.us.inv.i, float %_0.i.i463.us.us.us.us.us.i, float -1.600000e+02, !dbg !4407
  %_55.i.i.us.us.us.us.us.i319 = load float, ptr %394, align 4, !dbg !4410, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i109.us.us.us.us.us.i = fcmp ule float %_55.i.i.us.us.us.us.us.i319, 0.000000e+00, !dbg !4411
  %_3.i95.us.us.us.us.us.i320 = fcmp oge float %_0.i.i380.us.us.us.us.us.i, %threshold.i.i.us.us.us.us.us.i, !dbg !4413
  %_0.i176.us.us.us.us.us.i321 = fsub float %threshold.i.i.us.us.us.us.us.i, %hysteresis.i.i.us.us.us.us.us.i, !dbg !4415
  %_3.i93.us.us.us.us.us.i322 = fcmp oge float %_0.i.i380.us.us.us.us.us.i, %_0.i176.us.us.us.us.us.i321, !dbg !4417
  %..i94.us.us.us.us.us.i = sext i1 %_3.i93.us.us.us.us.us.i322 to i32, !dbg !4419
  %_0.i331.us.us.us.us.us.i = sext i1 %_3.i95.us.us.us.us.us.i320 to i32, !dbg !4421
  %_0.i325.us.us.us.us.us.i = select i1 %_3.i109.us.us.us.us.us.i, i32 %_0.i331.us.us.us.us.us.i, i32 %..i94.us.us.us.us.us.i, !dbg !4421
  %_0.i337.us.us.us.us.us.i = xor i32 %..i94.us.us.us.us.us.i, -1, !dbg !4423
  %_67.i.i.us.us.us.us.us.i323 = load float, ptr %395, align 4, !dbg !4425, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i107.us.us.us.us.us.i324 = fcmp ogt float %_67.i.i.us.us.us.us.us.i323, 0.000000e+00, !dbg !4426
  %_0.i330.us.us.us.us.us.i = select i1 %_3.i107.us.us.us.us.us.i324, i32 %_0.i337.us.us.us.us.us.i, i32 0, !dbg !4428
  %_0.i329.us.us.us.us.us.i = select i1 %_3.i109.us.us.us.us.us.i, i32 0, i32 %_0.i330.us.us.us.us.us.i, !dbg !4430
  %_0.i324.us.us.us.us.us.i = or i32 %_0.i329.us.us.us.us.us.i, %_0.i325.us.us.us.us.us.i, !dbg !4432
  %_5.i251.us.us.us.us.us.i = and i32 %_0.i324.us.us.us.us.us.i, 1065353216, !dbg !4434
  %_0.i255.us.us.us.us.us.i325 = bitcast i32 %_5.i251.us.us.us.us.us.i to float, !dbg !4436
  %_71.i.i510511.us.us.us.us.us.i = load float, ptr %396, align 4, !dbg !4438, !alias.scope !4340, !noalias !4341, !noundef !12
  %_0.i175.us.us.us.us.us.i326 = fadd float %_67.i.i.us.us.us.us.us.i323, -1.000000e+00, !dbg !4439
  %452 = trunc nsw i32 %_0.i329.us.us.us.us.us.i to i1, !dbg !4441
  %_4.i249.v.us.us.us.us.us.i = select i1 %452, float %_0.i175.us.us.us.us.us.i326, float %_67.i.i.us.us.us.us.us.i323, !dbg !4441
  %453 = trunc nsw i32 %_0.i325.us.us.us.us.us.i to i1, !dbg !4443
  %_0.i243.us.us.us.us.us.i = select i1 %453, float %_71.i.i510511.us.us.us.us.us.i, float %_4.i249.v.us.us.us.us.us.i, !dbg !4443
  store float %_0.i243.us.us.us.us.us.i, ptr %395, align 4, !dbg !4445, !alias.scope !4330, !noalias !4331
  store i32 %_5.i251.us.us.us.us.us.i, ptr %394, align 4, !dbg !4446, !alias.scope !4330, !noalias !4331
  %_0.i174.us.us.us.us.us.i327 = fadd float %ratio.i.i.us.us.us.us.us.i, -1.000000e+00, !dbg !4447
  %_0.i173.us.us.us.us.us.i328 = fsub float %_0.i.i380.us.us.us.us.us.i, %threshold.i.i.us.us.us.us.us.i, !dbg !4449
  %_0.i158.us.us.us.us.us.i329 = fmul float %_0.i174.us.us.us.us.us.i327, %_0.i173.us.us.us.us.us.i328, !dbg !4451
  %454 = fneg float %range.i.i.us.us.us.us.us.i, !dbg !4453
  %_3.i.i365.inv.us.us.us.us.us.i = fcmp ogt float %_0.i158.us.us.us.us.us.i329, %454, !dbg !4455
  %_4.i.i371.v.us.us.us.us.us.i = select i1 %_3.i.i365.inv.us.us.us.us.us.i, float %_0.i158.us.us.us.us.us.i329, float %454, !dbg !4455
  %_3.i.i448.us.us.us.us.us.i = fcmp olt float %_4.i.i371.v.us.us.us.us.us.i, 0.000000e+00, !dbg !4458
  %455 = fcmp ule float %_0.i255.us.us.us.us.us.i325, 0.000000e+00, !dbg !4461
  %456 = select i1 %455, i1 %_3.i.i448.us.us.us.us.us.i, i1 false, !dbg !4463
  %_0.i236.us.us.us.us.us.i = select i1 %456, float %_4.i.i371.v.us.us.us.us.us.i, float 0.000000e+00, !dbg !4463
  %_86.i.i.us.us.us.us.us.i330 = load float, ptr %397, align 4, !dbg !4464, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i103.us.us.us.us.us.i331 = fcmp ule float %_0.i236.us.us.us.us.us.i, %_86.i.i.us.us.us.us.us.i330, !dbg !4465
  %_87.i.i513.us.us.us.us.us.i = load i32, ptr %_38.i23, align 4, !dbg !4467, !alias.scope !4340, !noalias !4341, !noundef !12
  %_88.i.i514.us.us.us.us.us.i = load i32, ptr %398, align 4, !dbg !4468, !alias.scope !4340, !noalias !4341, !noundef !12
  %_4.i229.us.us.us.us.us.i = select i1 %_3.i103.us.us.us.us.us.i331, i32 %_88.i.i514.us.us.us.us.us.i, i32 %_87.i.i513.us.us.us.us.us.i, !dbg !4469
  %_0.i230.us.us.us.us.us.i332 = bitcast i32 %_4.i229.us.us.us.us.us.i to float, !dbg !4471
  %_0.i172.us.us.us.us.us.i333 = fsub float %_0.i236.us.us.us.us.us.i, %_86.i.i.us.us.us.us.us.i330, !dbg !4473
  %_4.i143.us.us.us.us.us.i = fmul float %_0.i172.us.us.us.us.us.i333, %_0.i230.us.us.us.us.us.i332, !dbg !4475
  %_0.i144.us.us.us.us.us.i334 = fadd float %_86.i.i.us.us.us.us.us.i330, %_4.i143.us.us.us.us.us.i, !dbg !4475
  %457 = tail call noundef float @llvm.fabs.f32(float %_0.i144.us.us.us.us.us.i334), !dbg !4477
  %458 = fcmp uge float %457, 0x3BC79CA100000000, !dbg !4480
  %_0.i214.us.us.us.us.us.i = select i1 %458, float %_0.i144.us.us.us.us.us.i334, float 0.000000e+00, !dbg !4482
  store float %_0.i214.us.us.us.us.us.i, ptr %397, align 4, !dbg !4483, !alias.scope !4330, !noalias !4331
  %_0.i157.us.us.us.us.us.i335 = fmul float %_0.i214.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !4484
  %_3.i.i357.us.us.us.us.us.inv.i = fcmp ogt float %_0.i157.us.us.us.us.us.i335, -1.260000e+02, !dbg !4487
  %_0.i.i364.us.us.us.us.us.i = select i1 %_3.i.i357.us.us.us.us.us.inv.i, float %_0.i157.us.us.us.us.us.i335, float -1.260000e+02, !dbg !4487
  %_3.i.i440.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i364.us.us.us.us.us.i, 1.270000e+02, !dbg !4491
  %_0.i.i447.us.us.us.us.us.i = select i1 %_3.i.i440.us.us.us.us.us.inv.i, float %_0.i.i364.us.us.us.us.us.i, float 1.270000e+02, !dbg !4491
  %459 = tail call noundef float @llvm.floor.f32(float %_0.i.i447.us.us.us.us.us.i), !dbg !4494
  %_0.i171.us.us.us.us.us.i336 = fsub float %_0.i.i447.us.us.us.us.us.i, %459, !dbg !4498
  %_0.i155.us.us.us.us.us.i = fmul float %_0.i171.us.us.us.us.us.i336, 0x3F5E974FA0000000, !dbg !4500
  %_0.i140.us.us.us.us.us.i = fadd float %_0.i155.us.us.us.us.us.i, 0x3F82778560000000, !dbg !4502
  %_0.i155.us.us.us.us.us.1.i = fmul float %_0.i171.us.us.us.us.us.i336, %_0.i140.us.us.us.us.us.i, !dbg !4500
  %_0.i140.us.us.us.us.us.1.i = fadd float %_0.i155.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4502
  %_0.i155.us.us.us.us.us.2.i = fmul float %_0.i171.us.us.us.us.us.i336, %_0.i140.us.us.us.us.us.1.i, !dbg !4500
  %_0.i140.us.us.us.us.us.2.i = fadd float %_0.i155.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4502
  %_0.i155.us.us.us.us.us.3.i = fmul float %_0.i171.us.us.us.us.us.i336, %_0.i140.us.us.us.us.us.2.i, !dbg !4500
  %_0.i140.us.us.us.us.us.3.i = fadd float %_0.i155.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4502
  %_0.i153.us.us.us.us.us.i = fmul float %_0.i170.us.us.us.us.us.i304, 0x3F5E974FA0000000, !dbg !4504
  %_0.i138.us.us.us.us.us.i = fadd float %_0.i153.us.us.us.us.us.i, 0x3F82778560000000, !dbg !4506
  %_0.i153.us.us.us.us.us.1.i = fmul float %_0.i170.us.us.us.us.us.i304, %_0.i138.us.us.us.us.us.i, !dbg !4504
  %_0.i138.us.us.us.us.us.1.i = fadd float %_0.i153.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4506
  %_0.i153.us.us.us.us.us.2.i = fmul float %_0.i170.us.us.us.us.us.i304, %_0.i138.us.us.us.us.us.1.i, !dbg !4504
  %_0.i138.us.us.us.us.us.2.i = fadd float %_0.i153.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4506
  %_0.i153.us.us.us.us.us.3.i = fmul float %_0.i170.us.us.us.us.us.i304, %_0.i138.us.us.us.us.us.2.i, !dbg !4504
  %_0.i138.us.us.us.us.us.3.i = fadd float %_0.i153.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4506
  %_0.i152.us.us.us.us.us.i337 = fmul float %_0.i170.us.us.us.us.us.i304, %_0.i138.us.us.us.us.us.3.i, !dbg !4508
  %_0.i137.us.us.us.us.us.i = fadd float %_0.i152.us.us.us.us.us.i337, 1.000000e+00, !dbg !4510
  %biased.i.us.us.us.us.us.i338 = fadd float %448, 0x4160000FE0000000, !dbg !4512
  %_4.i83.us.us.us.us.us.i339 = bitcast float %biased.i.us.us.us.us.us.i338 to i32, !dbg !4514
  %_3.i84.us.us.us.us.us.i340 = shl i32 %_4.i83.us.us.us.us.us.i339, 23, !dbg !4516
  %_0.i85.us.us.us.us.us.i341 = bitcast i32 %_3.i84.us.us.us.us.us.i340 to float, !dbg !4517
  %_0.i151.us.us.us.us.us.i342 = fmul float %_0.i137.us.us.us.us.us.i, %_0.i85.us.us.us.us.us.i341, !dbg !4519
  %_3.i91.us.us.us.us.us.i343 = fcmp une float %_0.i218.us.us.us.us.us.i, 0.000000e+00, !dbg !4521
  %_3.i115.us.us.us.us.us.i344 = fcmp ule float %_98.i102.i.us.us.us.us.us.i, 0.000000e+00, !dbg !4523
  %_0.i326501.not.us.us.us.us.us.i = and i1 %_3.i115.us.us.us.us.us.i344, %_3.i91.us.us.us.us.us.i343, !dbg !4525
  %_0.i162.us.us.us.us.us.i345 = fmul float %_0.i193.us.us.us.us.us.i239, %_0.i151.us.us.us.us.us.i342, !dbg !4525
  %_4.i275.v.us.us.us.us.us.i = select i1 %_0.i326501.not.us.us.us.us.us.i, float %_0.i162.us.us.us.us.us.i345, float %_0.i193.us.us.us.us.us.i239, !dbg !4528
  %_0.i.us.us.us.us.us.i346 = fmul float %_0.i171.us.us.us.us.us.i336, %_0.i140.us.us.us.us.us.3.i, !dbg !4530
  %_0.i139.us.us.us.us.us.i = fadd float %_0.i.us.us.us.us.us.i346, 1.000000e+00, !dbg !4532
  %biased.i86.us.us.us.us.us.i347 = fadd float %459, 0x4160000FE0000000, !dbg !4534
  %_4.i87.us.us.us.us.us.i348 = bitcast float %biased.i86.us.us.us.us.us.i347 to i32, !dbg !4536
  %_3.i88.us.us.us.us.us.i349 = shl i32 %_4.i87.us.us.us.us.us.i348, 23, !dbg !4538
  %_0.i89.us.us.us.us.us.i350 = bitcast i32 %_3.i88.us.us.us.us.us.i349 to float, !dbg !4539
  %_0.i154.us.us.us.us.us.i351 = fmul float %_0.i139.us.us.us.us.us.i, %_0.i89.us.us.us.us.us.i350, !dbg !4541
  %_3.i90.us.us.us.us.us.i352 = fcmp une float %_0.i214.us.us.us.us.us.i, 0.000000e+00, !dbg !4543
  %_98.i.i.us.us.us.us.us.i353 = load float, ptr %399, align 4, !dbg !4545, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i101.us.us.us.us.us.i354 = fcmp ule float %_98.i.i.us.us.us.us.us.i353, 0.000000e+00, !dbg !4546
  %_0.i323518.not.us.us.us.us.us.i = and i1 %_3.i101.us.us.us.us.us.i354, %_3.i90.us.us.us.us.us.i352, !dbg !4548
  %_0.i156.us.us.us.us.us.i355 = fmul float %_0.i191.us.us.us.us.us.i245, %_0.i154.us.us.us.us.us.i351, !dbg !4548
  %_4.i222.v.us.us.us.us.us.i = select i1 %_0.i323518.not.us.us.us.us.us.i, float %_0.i156.us.us.us.us.us.i355, float %_0.i191.us.us.us.us.us.i245, !dbg !4550
  store float %_4.i275.v.us.us.us.us.us.i, ptr %_97.i.us.us.us.us.us.i219, align 4, !dbg !4552, !alias.scope !4555, !noalias !3967
  store float %_4.i222.v.us.us.us.us.us.i, ptr %_115.i.us.us.us.us.us.i223, align 4, !dbg !4558, !alias.scope !4560, !noalias !3989
  %exitcond1438.not.i = icmp eq i64 %431, %_26, !dbg !4563
  br i1 %exitcond1438.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb30.i.us.us.us.us.us.i213, !dbg !4566, !llvm.loop !4570

bb8.i81.us.us.us.us.us.i363:                      ; preds = %bb5.i.preheader.us.us.us.us.us.i362
  %_34.i.us.us.us.us.us.i364 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.us.i253, !dbg !4567
  br i1 %_34.i.us.us.us.us.us.i364, label %bb10.i82.us.us.us.us.us.i365, label %panic5.i.i206, !dbg !4567

bb10.i82.us.us.us.us.us.i365:                     ; preds = %bb8.i81.us.us.us.us.us.i363
  %460 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.us.i249, !dbg !4568
  %left_own.i.us.us.us.us.us.i366 = load float, ptr %460, align 4, !dbg !4568, !alias.scope !4090, !noalias !4105, !noundef !12
  %461 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.us.i253, !dbg !4567
  %right_own.i.us.us.us.us.us.i367 = load float, ptr %461, align 4, !dbg !4567, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i267, !dbg !4110

bb5.i.preheader.us.us.us.us.us.i362:              ; preds = %bb50.i.us.us.us.us.us.i237
  br i1 %_31.i80.us.us.us.us.us.i254, label %bb8.i81.us.us.us.us.us.i363, label %panic4.i.i203, !dbg !4568

bb14.i63.preheader.us.us.us.us.us.i356:           ; preds = %bb50.i.us.us.us.us.us.i237
  br i1 %_31.i80.us.us.us.us.us.i254, label %bb17.i.us.us.us.us.us.i357, label %panic15.i.i195, !dbg !4114

bb23.i.preheader.us.us.us.us.us.i255:             ; preds = %bb50.i.us.us.us.us.us.i237
  br i1 %_31.i80.us.us.us.us.us.i254, label %bb27.i60.us.us.us.us.us.i256, label %panic28.i.i88, !dbg !4104

bb32.i.us.us.us.us.i369:                          ; preds = %bb30.i.lr.ph.split.us.split.us.split.us.split.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.i419
  %iter.sroa.0.0.i637.us.us.us.us.i = phi i64 [ %462, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.i419 ], [ 0, %bb30.i.lr.ph.split.us.split.us.split.us.split.us.i ]
  %462 = add nuw nsw i64 %iter.sroa.0.0.i637.us.us.us.us.i, 1, !dbg !3924
  %_28.i.us.us.us.us.i370 = trunc i64 %iter.sroa.0.0.i637.us.us.us.us.i to i32, !dbg !3938
  %now.i.us.us.us.us.i371 = add i32 %base.i.i49, %_28.i.us.us.us.us.i370, !dbg !3941
  %_31.i.us.us.us.us.i372 = and i32 %now.i.us.us.us.us.i371, %_58.i33, !dbg !3944
  %_30.i.us.us.us.us.i373 = zext i32 %_31.i.us.us.us.us.i372 to i64, !dbg !3946
  %_97.i.us.us.us.us.i374 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i637.us.us.us.us.i, !dbg !3953
  %_98.not.not.i.us.us.us.us.i375 = icmp ugt i64 %_64.1.i25, %_30.i.us.us.us.us.i373, !dbg !3957
  br i1 %_98.not.not.i.us.us.us.us.i375, label %bb35.i.us.us.us.us.i376, label %bb36.i.i59, !dbg !3957, !prof !2709

bb35.i.us.us.us.us.i376:                          ; preds = %bb32.i.us.us.us.us.i369
  %_0.i201.us.us.us.us.i = load float, ptr %_97.i.us.us.us.us.i374, align 4, !dbg !3962, !alias.scope !3964, !noalias !3967, !noundef !12
  %_107.i.us.us.us.us.i377 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_30.i.us.us.us.us.i373, !dbg !3968
  store float %_0.i201.us.us.us.us.i, ptr %_107.i.us.us.us.us.i377, align 4, !dbg !3972, !alias.scope !3974, !noalias !3921
  %_115.i.us.us.us.us.i378 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i637.us.us.us.us.i, !dbg !3977
  %_0.i199.us.us.us.us.i = load float, ptr %_115.i.us.us.us.us.i378, align 4, !dbg !3984, !alias.scope !3986, !noalias !3989, !noundef !12
  %_123.i.us.us.us.us.i379 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_30.i.us.us.us.us.i373, !dbg !3990
  store float %_0.i199.us.us.us.us.i, ptr %_123.i.us.us.us.us.i379, align 4, !dbg !3997, !alias.scope !3999, !noalias !3921
  %_131.i.us.us.us.us.i380 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i47, i64 %iter.sroa.0.0.i637.us.us.us.us.i, !dbg !4002
  %_0.i197.us.us.us.us.i = load float, ptr %_131.i.us.us.us.us.i380, align 4, !dbg !4009, !alias.scope !4011, !noalias !3921, !noundef !12
  %_139.i.us.us.us.us.i381 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_30.i.us.us.us.us.i373, !dbg !4014
  store float %_0.i197.us.us.us.us.i, ptr %_139.i.us.us.us.us.i381, align 4, !dbg !4021, !alias.scope !4023, !noalias !3921
  %exitcond1439.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.us.us.us.i, %empty.sroa.6.0.i.i46, !dbg !4571
  br i1 %exitcond1439.not.i, label %bb47.i.i520, label %bb46.i.us.us.us.us.i382, !dbg !4571, !prof !180

bb46.i.us.us.us.us.i382:                          ; preds = %bb35.i.us.us.us.us.i376
  %_148.not.not.i.us.us.us.us.i383 = icmp ugt i64 %_67.1.i31, %_30.i.us.us.us.us.i373, !dbg !4569
  br i1 %_148.not.not.i.us.us.us.us.i383, label %bb48.i.us.us.us.us.i384, label %bb49.i.i228, !dbg !4569, !prof !2709

bb48.i.us.us.us.us.i384:                          ; preds = %bb46.i.us.us.us.us.i382
  %_147.i.us.us.us.us.i385 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i45, i64 %iter.sroa.0.0.i637.us.us.us.us.i, !dbg !4026
  %_0.i195.us.us.us.us.i = load float, ptr %_147.i.us.us.us.us.i385, align 4, !dbg !4033, !alias.scope !4035, !noalias !3921, !noundef !12
  %_155.i.us.us.us.us.i386 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_30.i.us.us.us.us.i373, !dbg !4038
  store float %_0.i195.us.us.us.us.i, ptr %_155.i.us.us.us.us.i386, align 4, !dbg !4045, !alias.scope !4047, !noalias !3921
  %_53.i.us.us.us.us.i387 = sub i32 %now.i.us.us.us.us.i371, %_59.i34, !dbg !4050
  %_52.i.us.us.us.us.i388 = and i32 %_53.i.us.us.us.us.i387, %_58.i33, !dbg !4053
  %_51.i.us.us.us.us.i389 = zext i32 %_52.i.us.us.us.us.i388 to i64, !dbg !4054
  %_156.not.not.i.us.us.us.us.i390 = icmp ugt i64 %_64.1.i25, %_51.i.us.us.us.us.i389, !dbg !4055
  br i1 %_156.not.not.i.us.us.us.us.i390, label %bb50.i.us.us.us.us.i391, label %bb51.i.i72, !dbg !4055, !prof !2709

bb50.i.us.us.us.us.i391:                          ; preds = %bb48.i.us.us.us.us.i384
  %_163.i.us.us.us.us.i392 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_51.i.us.us.us.us.i389, !dbg !4060
  %_0.i193.us.us.us.us.i393 = load float, ptr %_163.i.us.us.us.us.i392, align 4, !dbg !4064, !alias.scope !4066, !noalias !3921, !noundef !12
  %_169.i.us.us.us.us.i396 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_51.i.us.us.us.us.i389, !dbg !4069
  %_0.i191.us.us.us.us.i397 = load float, ptr %_169.i.us.us.us.us.i396, align 4, !dbg !4077, !alias.scope !4079, !noalias !3921, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4082), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4088), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4090), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4092), !dbg !4085
  %_18.i74.us.us.us.us.i398 = load i32, ptr %_31.i17, align 4, !dbg !4094, !alias.scope !4096, !noalias !4097, !noundef !12
  %_17.i.us.us.us.us.i399 = sub i32 %now.i.us.us.us.us.i371, %_18.i74.us.us.us.us.i398, !dbg !4099
  %_16.i75.us.us.us.us.i400 = and i32 %_17.i.us.us.us.us.i399, %_58.i33, !dbg !4094
  %_15.i76.us.us.us.us.i401 = zext i32 %_16.i75.us.us.us.us.i400 to i64, !dbg !4094
  %_26.i.us.us.us.us.i402 = load i32, ptr %_33.i18, align 4, !dbg !4094, !alias.scope !4101, !noalias !4102, !noundef !12
  %_25.i.us.us.us.us.i403 = sub i32 %now.i.us.us.us.us.i371, %_26.i.us.us.us.us.i402, !dbg !4099
  %_24.i.us.us.us.us.i404 = and i32 %_25.i.us.us.us.us.i403, %_58.i33, !dbg !4094
  %_23.i79.us.us.us.us.i405 = zext i32 %_24.i.us.us.us.us.i404 to i64, !dbg !4094
  %_31.i80.us.us.us.us.i406 = icmp samesign ugt i64 %_65.1.i27, %_15.i76.us.us.us.us.i401, !dbg !4094
  switch i8 %_0.sroa.0.0.i485.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.us.us.i514
    i8 1, label %bb14.i63.preheader.us.us.us.us.i508
    i8 2, label %bb23.i.preheader.us.us.us.us.i407
  ], !dbg !4103

bb27.i60.us.us.us.us.i408:                        ; preds = %bb23.i.preheader.us.us.us.us.i407
  %463 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.i401, !dbg !4104
  %_79.i.us.us.us.us.i409 = load float, ptr %463, align 4, !dbg !4104, !alias.scope !4090, !noalias !4105, !noundef !12
  %_85.i.us.us.us.us.i410 = icmp samesign ugt i64 %_67.1.i31, %_15.i76.us.us.us.us.i401, !dbg !4106
  br i1 %_85.i.us.us.us.us.i410, label %bb29.i61.us.us.us.us.i411, label %panic30.i.i92, !dbg !4106

bb29.i61.us.us.us.us.i411:                        ; preds = %bb27.i60.us.us.us.us.i408
  %464 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_15.i76.us.us.us.us.i401, !dbg !4106
  %_83.i.us.us.us.us.i412 = load float, ptr %464, align 4, !dbg !4106, !alias.scope !4092, !noalias !4107, !noundef !12
  %_87.i.us.us.us.us.i413 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.i405, !dbg !4108
  br i1 %_87.i.us.us.us.us.i413, label %bb31.i.us.us.us.us.i414, label %panic32.i.i96, !dbg !4108

bb31.i.us.us.us.us.i414:                          ; preds = %bb29.i61.us.us.us.us.i411
  %_89.i.us.us.us.us.i415 = icmp samesign ugt i64 %_65.1.i27, %_23.i79.us.us.us.us.i405, !dbg !4109
  br i1 %_89.i.us.us.us.us.i415, label %bb33.i62.us.us.us.us.i416, label %panic34.i.i99, !dbg !4109

bb33.i62.us.us.us.us.i416:                        ; preds = %bb31.i.us.us.us.us.i414
  %465 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.i405, !dbg !4108
  %_86.i.us.us.us.us.i417 = load float, ptr %465, align 4, !dbg !4108, !alias.scope !4092, !noalias !4107, !noundef !12
  %466 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_23.i79.us.us.us.us.i405, !dbg !4109
  %_88.i.us.us.us.us.i418 = load float, ptr %466, align 4, !dbg !4109, !alias.scope !4090, !noalias !4105, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.i419, !dbg !4110

bb17.i.us.us.us.us.i509:                          ; preds = %bb14.i63.preheader.us.us.us.us.i508
  %_59.i.us.us.us.us.i510 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.i405, !dbg !4113
  br i1 %_59.i.us.us.us.us.i510, label %bb19.i.us.us.us.us.i511, label %panic17.i.i198, !dbg !4113

bb19.i.us.us.us.us.i511:                          ; preds = %bb17.i.us.us.us.us.i509
  %467 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.i401, !dbg !4114
  %left_own16.i.us.us.us.us.i512 = load float, ptr %467, align 4, !dbg !4114, !alias.scope !4090, !noalias !4105, !noundef !12
  %468 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.i405, !dbg !4113
  %right_own18.i.us.us.us.us.i513 = load float, ptr %468, align 4, !dbg !4113, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.i419, !dbg !4110

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.i419: ; preds = %bb10.i82.us.us.us.us.i517, %bb19.i.us.us.us.us.i511, %bb33.i62.us.us.us.us.i416
  %taps.i.sroa.1011457.4.i = phi float [ %right_own.i.us.us.us.us.i519, %bb10.i82.us.us.us.us.i517 ], [ %left_own16.i.us.us.us.us.i512, %bb19.i.us.us.us.us.i511 ], [ %_88.i.us.us.us.us.i418, %bb33.i62.us.us.us.us.i416 ], !dbg !4094
  %taps.i.sroa.681456.4.i = phi float [ %right_own.i.us.us.us.us.i519, %bb10.i82.us.us.us.us.i517 ], [ %right_own18.i.us.us.us.us.i513, %bb19.i.us.us.us.us.i511 ], [ %_86.i.us.us.us.us.i417, %bb33.i62.us.us.us.us.i416 ], !dbg !4094
  %taps.i.sroa.351455.4.i = phi float [ %left_own.i.us.us.us.us.i518, %bb10.i82.us.us.us.us.i517 ], [ %right_own18.i.us.us.us.us.i513, %bb19.i.us.us.us.us.i511 ], [ %_83.i.us.us.us.us.i412, %bb33.i62.us.us.us.us.i416 ], !dbg !4094
  %taps.i.sroa.0.4.i420 = phi float [ %left_own.i.us.us.us.us.i518, %bb10.i82.us.us.us.us.i517 ], [ %left_own16.i.us.us.us.us.i512, %bb19.i.us.us.us.us.i511 ], [ %_79.i.us.us.us.us.i409, %bb33.i62.us.us.us.us.i416 ], !dbg !4094
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4115), !dbg !4118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4119), !dbg !4118
  %threshold.i30.i.us.us.us.us.i = load float, ptr %_10.i13, align 4, !dbg !4121, !alias.scope !4126, !noalias !4127, !noundef !12
  %ratio.i31.i.us.us.us.us.i = load float, ptr %378, align 4, !dbg !4128, !alias.scope !4126, !noalias !4127, !noundef !12
  %range.i32.i.us.us.us.us.i = load float, ptr %379, align 4, !dbg !4130, !alias.scope !4126, !noalias !4127, !noundef !12
  %hysteresis.i33.i.us.us.us.us.i = load float, ptr %380, align 4, !dbg !4132, !alias.scope !4126, !noalias !4127, !noundef !12
  %469 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.4.i420), !dbg !4134
  %470 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351455.4.i), !dbg !4137
  %_37.i36.i.us.us.us.us.i = load float, ptr %381, align 4, !dbg !4140, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i127.us.us.us.us.i421 = fcmp ule float %_37.i36.i.us.us.us.us.i, 0.000000e+00, !dbg !4145
  %_3.i.i423.us.us.us.us.i = fcmp ule float %469, %470, !dbg !4147
  %_6.i.i425.us.us.us.us.i = bitcast float %469 to i32, !dbg !4150
  %_8.i.i427.us.us.us.us.i = bitcast float %470 to i32, !dbg !4153
  %_4.i.i430.us.us.us.us.i = select i1 %_3.i.i423.us.us.us.us.i, i32 %_8.i.i427.us.us.us.us.i, i32 %_6.i.i425.us.us.us.us.i, !dbg !4155
  %_4.i321.us.us.us.us.i422 = select i1 %_3.i127.us.us.us.us.i421, i32 %_6.i.i425.us.us.us.us.i, i32 %_4.i.i430.us.us.us.us.i, !dbg !4156
  %_41.i40.i.us.us.us.us.i = load float, ptr %382, align 4, !dbg !4158, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i125.us.us.us.us.i = fcmp ule float %_41.i40.i.us.us.us.us.i, 0.000000e+00, !dbg !4159
  %_0.i167.us.us.us.us.i423 = fmul float %469, 5.000000e-01, !dbg !4161
  %_0.i166.us.us.us.us.i424 = fmul float %470, 5.000000e-01, !dbg !4163
  %_0.i142.us.us.us.us.i425 = fadd float %_0.i166.us.us.us.us.i424, %_0.i167.us.us.us.us.i423, !dbg !4165
  %_6.i309.us.us.us.us.i = bitcast float %_0.i142.us.us.us.us.i425 to i32, !dbg !4167
  %_4.i314.us.us.us.us.i = select i1 %_3.i125.us.us.us.us.i, i32 %_4.i321.us.us.us.us.i422, i32 %_6.i309.us.us.us.us.i, !dbg !4170
  %_0.i315.us.us.us.us.i = bitcast i32 %_4.i314.us.us.us.us.i to float, !dbg !4171
  %_3.i.i415.us.us.us.us.i = fcmp ule float %_0.i315.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4173
  %_4.i.i421.us.us.us.us.i = select i1 %_3.i.i415.us.us.us.us.i, i32 841731191, i32 %_4.i314.us.us.us.us.i, !dbg !4176
  %_0.i.i422.us.us.us.us.i426 = bitcast i32 %_4.i.i421.us.us.us.us.i to float, !dbg !4178
  %_3.i.i.us.us.us.us.i427 = fcmp ule float %_0.i.i422.us.us.us.us.i426, 0x3810000000000000, !dbg !4180
  %_4.i.i.us.us.us.us.i428 = select i1 %_3.i.i.us.us.us.us.i427, i32 8388608, i32 %_4.i.i421.us.us.us.us.i, !dbg !4186
  %_5.i202.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.i428, 8388607, !dbg !4188
  %_4.i203.us.us.us.us.i = or disjoint i32 %_5.i202.us.us.us.us.i, 1065353216, !dbg !4188
  %significand.i.us.us.us.us.i429 = bitcast i32 %_4.i203.us.us.us.us.i to float, !dbg !4190
  %_0.i168.us.us.us.us.i430 = fadd float %significand.i.us.us.us.us.i429, -1.000000e+00, !dbg !4192
  %_0.i148.us.us.us.us.i431 = fmul float %_0.i168.us.us.us.us.i430, 0x3F9B17A960000000, !dbg !4194
  %471 = fsub float 0x3FBF9A8440000000, %_0.i148.us.us.us.us.i431, !dbg !4196
  %_0.i148.us.us.us.us.1.i432 = fmul float %_0.i168.us.us.us.us.i430, %471, !dbg !4194
  %_0.i134.us.us.us.us.1.i = fadd float %_0.i148.us.us.us.us.1.i432, 0xBFD1E3F400000000, !dbg !4196
  %_0.i148.us.us.us.us.2.i433 = fmul float %_0.i168.us.us.us.us.i430, %_0.i134.us.us.us.us.1.i, !dbg !4194
  %_0.i134.us.us.us.us.2.i = fadd float %_0.i148.us.us.us.us.2.i433, 0x3FDD544F20000000, !dbg !4196
  %_0.i148.us.us.us.us.3.i434 = fmul float %_0.i168.us.us.us.us.i430, %_0.i134.us.us.us.us.2.i, !dbg !4194
  %_0.i134.us.us.us.us.3.i = fadd float %_0.i148.us.us.us.us.3.i434, 0xBFE6FC2A60000000, !dbg !4196
  %_0.i148.us.us.us.us.4.i = fmul float %_0.i168.us.us.us.us.i430, %_0.i134.us.us.us.us.3.i, !dbg !4194
  %_0.i134.us.us.us.us.4.i = fadd float %_0.i148.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4196
  %_9.i.us.us.us.us.i435 = lshr i32 %_4.i.i.us.us.us.us.i428, 23, !dbg !4198
  %_8.i204.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.i435, 1258291200, !dbg !4198
  %_7.i.us.us.us.us.i436 = bitcast i32 %_8.i204.us.us.us.us.i to float, !dbg !4199
  %exponent.i.us.us.us.us.i437 = fadd float %_7.i.us.us.us.us.i436, 0xC160000FE0000000, !dbg !4201
  %_0.i147.us.us.us.us.i438 = fmul float %_0.i168.us.us.us.us.i430, %_0.i134.us.us.us.us.4.i, !dbg !4202
  %_0.i133.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.i437, %_0.i147.us.us.us.us.i438, !dbg !4204
  %_0.i165.us.us.us.us.i439 = fmul float %_0.i133.us.us.us.us.i, 0x4018151820000000, !dbg !4206
  %_3.i.i472.us.us.us.us.inv.i = fcmp olt float %_0.i165.us.us.us.us.i439, 2.400000e+01, !dbg !4208
  %_0.i.i479.us.us.us.us.i = select i1 %_3.i.i472.us.us.us.us.inv.i, float %_0.i165.us.us.us.us.i439, float 2.400000e+01, !dbg !4208
  %_3.i.i407.us.us.us.us.inv.i = fcmp ogt float %_0.i.i479.us.us.us.us.i, -1.600000e+02, !dbg !4211
  %_0.i.i414.us.us.us.us.i = select i1 %_3.i.i407.us.us.us.us.inv.i, float %_0.i.i479.us.us.us.us.i, float -1.600000e+02, !dbg !4211
  %_55.i57.i.us.us.us.us.i = load float, ptr %383, align 4, !dbg !4214, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i123.us.us.us.us.i440 = fcmp ule float %_55.i57.i.us.us.us.us.i, 0.000000e+00, !dbg !4216
  %_3.i99.us.us.us.us.i441 = fcmp oge float %_0.i.i414.us.us.us.us.i, %threshold.i30.i.us.us.us.us.i, !dbg !4218
  %_0.i181.us.us.us.us.i442 = fsub float %threshold.i30.i.us.us.us.us.i, %hysteresis.i33.i.us.us.us.us.i, !dbg !4221
  %_3.i97.us.us.us.us.i443 = fcmp oge float %_0.i.i414.us.us.us.us.i, %_0.i181.us.us.us.us.i442, !dbg !4224
  %..i98.us.us.us.us.i444 = sext i1 %_3.i97.us.us.us.us.i443 to i32, !dbg !4226
  %_0.i335.us.us.us.us.i445 = sext i1 %_3.i99.us.us.us.us.i441 to i32, !dbg !4228
  %_0.i328.us.us.us.us.i446 = select i1 %_3.i123.us.us.us.us.i440, i32 %_0.i335.us.us.us.us.i445, i32 %..i98.us.us.us.us.i444, !dbg !4228
  %_0.i339.us.us.us.us.i = xor i32 %..i98.us.us.us.us.i444, -1, !dbg !4233
  %_67.i67.i.us.us.us.us.i = load float, ptr %384, align 4, !dbg !4236, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i121.us.us.us.us.i447 = fcmp ogt float %_67.i67.i.us.us.us.us.i, 0.000000e+00, !dbg !4237
  %_0.i334.us.us.us.us.i = select i1 %_3.i121.us.us.us.us.i447, i32 %_0.i339.us.us.us.us.i, i32 0, !dbg !4239
  %_0.i333.us.us.us.us.i = select i1 %_3.i123.us.us.us.us.i440, i32 0, i32 %_0.i334.us.us.us.us.i, !dbg !4241
  %_0.i327.us.us.us.us.i = or i32 %_0.i333.us.us.us.us.i, %_0.i328.us.us.us.us.i446, !dbg !4243
  %_5.i304.us.us.us.us.i = and i32 %_0.i327.us.us.us.us.i, 1065353216, !dbg !4246
  %_0.i308.us.us.us.us.i = bitcast i32 %_5.i304.us.us.us.us.i to float, !dbg !4248
  %_71.i73.i493494.us.us.us.us.i = load float, ptr %385, align 4, !dbg !4250, !alias.scope !4143, !noalias !4144, !noundef !12
  %_0.i180.us.us.us.us.i448 = fadd float %_67.i67.i.us.us.us.us.i, -1.000000e+00, !dbg !4252
  %472 = trunc nsw i32 %_0.i333.us.us.us.us.i to i1, !dbg !4254
  %_4.i302.v.us.us.us.us.i = select i1 %472, float %_0.i180.us.us.us.us.i448, float %_67.i67.i.us.us.us.us.i, !dbg !4254
  %473 = trunc nsw i32 %_0.i328.us.us.us.us.i446 to i1, !dbg !4256
  %_0.i296.us.us.us.us.i = select i1 %473, float %_71.i73.i493494.us.us.us.us.i, float %_4.i302.v.us.us.us.us.i, !dbg !4256
  store float %_0.i296.us.us.us.us.i, ptr %384, align 4, !dbg !4258, !alias.scope !4126, !noalias !4127
  store i32 %_5.i304.us.us.us.us.i, ptr %383, align 4, !dbg !4259, !alias.scope !4126, !noalias !4127
  %_0.i179.us.us.us.us.i449 = fadd float %ratio.i31.i.us.us.us.us.i, -1.000000e+00, !dbg !4260
  %_0.i178.us.us.us.us.i450 = fsub float %_0.i.i414.us.us.us.us.i, %threshold.i30.i.us.us.us.us.i, !dbg !4262
  %_0.i164.us.us.us.us.i451 = fmul float %_0.i179.us.us.us.us.i449, %_0.i178.us.us.us.us.i450, !dbg !4264
  %474 = fneg float %range.i32.i.us.us.us.us.i, !dbg !4266
  %_3.i.i398.inv.us.us.us.us.i = fcmp ogt float %_0.i164.us.us.us.us.i451, %474, !dbg !4268
  %_4.i.i405.v.us.us.us.us.i = select i1 %_3.i.i398.inv.us.us.us.us.i, float %_0.i164.us.us.us.us.i451, float %474, !dbg !4268
  %_3.i.i464.us.us.us.us.i = fcmp olt float %_4.i.i405.v.us.us.us.us.i, 0.000000e+00, !dbg !4271
  %475 = fcmp ule float %_0.i308.us.us.us.us.i, 0.000000e+00, !dbg !4274
  %476 = select i1 %475, i1 %_3.i.i464.us.us.us.us.i, i1 false, !dbg !4277
  %_0.i289.us.us.us.us.i = select i1 %476, float %_4.i.i405.v.us.us.us.us.i, float 0.000000e+00, !dbg !4277
  %_86.i86.i.us.us.us.us.i = load float, ptr %386, align 4, !dbg !4278, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i117.us.us.us.us.i452 = fcmp ule float %_0.i289.us.us.us.us.i, %_86.i86.i.us.us.us.us.i, !dbg !4280
  %_87.i88.i496.us.us.us.us.i = load i32, ptr %366, align 4, !dbg !4282, !alias.scope !4143, !noalias !4144, !noundef !12
  %_88.i89.i497.us.us.us.us.i = load i32, ptr %387, align 4, !dbg !4283, !alias.scope !4143, !noalias !4144, !noundef !12
  %_4.i282.us.us.us.us.i = select i1 %_3.i117.us.us.us.us.i452, i32 %_88.i89.i497.us.us.us.us.i, i32 %_87.i88.i496.us.us.us.us.i, !dbg !4284
  %_0.i283.us.us.us.us.i = bitcast i32 %_4.i282.us.us.us.us.i to float, !dbg !4286
  %_0.i177.us.us.us.us.i453 = fsub float %_0.i289.us.us.us.us.i, %_86.i86.i.us.us.us.us.i, !dbg !4288
  %_4.i145.us.us.us.us.i = fmul float %_0.i177.us.us.us.us.i453, %_0.i283.us.us.us.us.i, !dbg !4291
  %_0.i146.us.us.us.us.i454 = fadd float %_86.i86.i.us.us.us.us.i, %_4.i145.us.us.us.us.i, !dbg !4291
  %477 = tail call noundef float @llvm.fabs.f32(float %_0.i146.us.us.us.us.i454), !dbg !4293
  %478 = fcmp uge float %477, 0x3BC79CA100000000, !dbg !4297
  %_0.i218.us.us.us.us.i = select i1 %478, float %_0.i146.us.us.us.us.i454, float 0.000000e+00, !dbg !4299
  store float %_0.i218.us.us.us.us.i, ptr %386, align 4, !dbg !4300, !alias.scope !4126, !noalias !4127
  %_0.i163.us.us.us.us.i455 = fmul float %_0.i218.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !4302
  %_3.i.i349.us.us.us.us.inv.i = fcmp ogt float %_0.i163.us.us.us.us.i455, -1.260000e+02, !dbg !4306
  %_0.i.i356.us.us.us.us.i = select i1 %_3.i.i349.us.us.us.us.inv.i, float %_0.i163.us.us.us.us.i455, float -1.260000e+02, !dbg !4306
  %_3.i.i432.us.us.us.us.inv.i = fcmp olt float %_0.i.i356.us.us.us.us.i, 1.270000e+02, !dbg !4310
  %_0.i.i439.us.us.us.us.i = select i1 %_3.i.i432.us.us.us.us.inv.i, float %_0.i.i356.us.us.us.us.i, float 1.270000e+02, !dbg !4310
  %479 = tail call noundef float @llvm.floor.f32(float %_0.i.i439.us.us.us.us.i), !dbg !4313
  %_0.i170.us.us.us.us.i456 = fsub float %_0.i.i439.us.us.us.us.i, %479, !dbg !4317
  %_98.i102.i.us.us.us.us.i = load float, ptr %388, align 4, !dbg !4319, !alias.scope !4143, !noalias !4144, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4321), !dbg !4324
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4326), !dbg !4324
  %threshold.i.i.us.us.us.us.i = load float, ptr %data.i.i.i14, align 4, !dbg !4328, !alias.scope !4330, !noalias !4331, !noundef !12
  %ratio.i.i.us.us.us.us.i = load float, ptr %389, align 4, !dbg !4332, !alias.scope !4330, !noalias !4331, !noundef !12
  %range.i.i.us.us.us.us.i = load float, ptr %390, align 4, !dbg !4333, !alias.scope !4330, !noalias !4331, !noundef !12
  %hysteresis.i.i.us.us.us.us.i = load float, ptr %391, align 4, !dbg !4334, !alias.scope !4330, !noalias !4331, !noundef !12
  %480 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681456.4.i), !dbg !4335
  %481 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011457.4.i), !dbg !4337
  %_37.i.i.us.us.us.us.i457 = load float, ptr %392, align 4, !dbg !4339, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i113.us.us.us.us.i458 = fcmp ule float %_37.i.i.us.us.us.us.i457, 0.000000e+00, !dbg !4342
  %_3.i.i389.us.us.us.us.i = fcmp ule float %480, %481, !dbg !4344
  %_6.i.i391.us.us.us.us.i = bitcast float %480 to i32, !dbg !4347
  %_8.i.i393.us.us.us.us.i = bitcast float %481 to i32, !dbg !4350
  %_4.i.i396.us.us.us.us.i = select i1 %_3.i.i389.us.us.us.us.i, i32 %_8.i.i393.us.us.us.us.i, i32 %_6.i.i391.us.us.us.us.i, !dbg !4352
  %_4.i268.us.us.us.us.i = select i1 %_3.i113.us.us.us.us.i458, i32 %_6.i.i391.us.us.us.us.i, i32 %_4.i.i396.us.us.us.us.i, !dbg !4353
  %_41.i.i.us.us.us.us.i459 = load float, ptr %393, align 4, !dbg !4355, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i111.us.us.us.us.i460 = fcmp ule float %_41.i.i.us.us.us.us.i459, 0.000000e+00, !dbg !4356
  %_0.i161.us.us.us.us.i461 = fmul float %480, 5.000000e-01, !dbg !4358
  %_0.i160.us.us.us.us.i462 = fmul float %481, 5.000000e-01, !dbg !4360
  %_0.i141.us.us.us.us.i463 = fadd float %_0.i160.us.us.us.us.i462, %_0.i161.us.us.us.us.i461, !dbg !4362
  %_6.i256.us.us.us.us.i = bitcast float %_0.i141.us.us.us.us.i463 to i32, !dbg !4364
  %_4.i261.us.us.us.us.i = select i1 %_3.i111.us.us.us.us.i460, i32 %_4.i268.us.us.us.us.i, i32 %_6.i256.us.us.us.us.i, !dbg !4367
  %_0.i262.us.us.us.us.i = bitcast i32 %_4.i261.us.us.us.us.i to float, !dbg !4368
  %_3.i.i381.us.us.us.us.i = fcmp ule float %_0.i262.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4370
  %_4.i.i387.us.us.us.us.i = select i1 %_3.i.i381.us.us.us.us.i, i32 841731191, i32 %_4.i261.us.us.us.us.i, !dbg !4373
  %_0.i.i388.us.us.us.us.i = bitcast i32 %_4.i.i387.us.us.us.us.i to float, !dbg !4375
  %_3.i.i341.us.us.us.us.i = fcmp ule float %_0.i.i388.us.us.us.us.i, 0x3810000000000000, !dbg !4377
  %_4.i.i347.us.us.us.us.i = select i1 %_3.i.i341.us.us.us.us.i, i32 8388608, i32 %_4.i.i387.us.us.us.us.i, !dbg !4382
  %_5.i206.us.us.us.us.i = and i32 %_4.i.i347.us.us.us.us.i, 8388607, !dbg !4384
  %_4.i207.us.us.us.us.i = or disjoint i32 %_5.i206.us.us.us.us.i, 1065353216, !dbg !4384
  %significand.i208.us.us.us.us.i = bitcast i32 %_4.i207.us.us.us.us.i to float, !dbg !4386
  %_0.i169.us.us.us.us.i464 = fadd float %significand.i208.us.us.us.us.i, -1.000000e+00, !dbg !4388
  %_0.i150.us.us.us.us.i465 = fmul float %_0.i169.us.us.us.us.i464, 0x3F9B17A960000000, !dbg !4390
  %482 = fsub float 0x3FBF9A8440000000, %_0.i150.us.us.us.us.i465, !dbg !4392
  %_0.i150.us.us.us.us.1.i466 = fmul float %_0.i169.us.us.us.us.i464, %482, !dbg !4390
  %_0.i136.us.us.us.us.1.i = fadd float %_0.i150.us.us.us.us.1.i466, 0xBFD1E3F400000000, !dbg !4392
  %_0.i150.us.us.us.us.2.i467 = fmul float %_0.i169.us.us.us.us.i464, %_0.i136.us.us.us.us.1.i, !dbg !4390
  %_0.i136.us.us.us.us.2.i = fadd float %_0.i150.us.us.us.us.2.i467, 0x3FDD544F20000000, !dbg !4392
  %_0.i150.us.us.us.us.3.i468 = fmul float %_0.i169.us.us.us.us.i464, %_0.i136.us.us.us.us.2.i, !dbg !4390
  %_0.i136.us.us.us.us.3.i = fadd float %_0.i150.us.us.us.us.3.i468, 0xBFE6FC2A60000000, !dbg !4392
  %_0.i150.us.us.us.us.4.i = fmul float %_0.i169.us.us.us.us.i464, %_0.i136.us.us.us.us.3.i, !dbg !4390
  %_0.i136.us.us.us.us.4.i = fadd float %_0.i150.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4392
  %_9.i209.us.us.us.us.i = lshr i32 %_4.i.i347.us.us.us.us.i, 23, !dbg !4394
  %_8.i210.us.us.us.us.i = or disjoint i32 %_9.i209.us.us.us.us.i, 1258291200, !dbg !4394
  %_7.i211.us.us.us.us.i = bitcast i32 %_8.i210.us.us.us.us.i to float, !dbg !4395
  %exponent.i212.us.us.us.us.i = fadd float %_7.i211.us.us.us.us.i, 0xC160000FE0000000, !dbg !4397
  %_0.i149.us.us.us.us.i469 = fmul float %_0.i169.us.us.us.us.i464, %_0.i136.us.us.us.us.4.i, !dbg !4398
  %_0.i135.us.us.us.us.i = fadd float %exponent.i212.us.us.us.us.i, %_0.i149.us.us.us.us.i469, !dbg !4400
  %_0.i159.us.us.us.us.i470 = fmul float %_0.i135.us.us.us.us.i, 0x4018151820000000, !dbg !4402
  %_3.i.i456.us.us.us.us.inv.i = fcmp olt float %_0.i159.us.us.us.us.i470, 2.400000e+01, !dbg !4404
  %_0.i.i463.us.us.us.us.i = select i1 %_3.i.i456.us.us.us.us.inv.i, float %_0.i159.us.us.us.us.i470, float 2.400000e+01, !dbg !4404
  %_3.i.i373.us.us.us.us.inv.i = fcmp ogt float %_0.i.i463.us.us.us.us.i, -1.600000e+02, !dbg !4407
  %_0.i.i380.us.us.us.us.i = select i1 %_3.i.i373.us.us.us.us.inv.i, float %_0.i.i463.us.us.us.us.i, float -1.600000e+02, !dbg !4407
  %_55.i.i.us.us.us.us.i471 = load float, ptr %394, align 4, !dbg !4410, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i109.us.us.us.us.i = fcmp ule float %_55.i.i.us.us.us.us.i471, 0.000000e+00, !dbg !4411
  %_3.i95.us.us.us.us.i472 = fcmp oge float %_0.i.i380.us.us.us.us.i, %threshold.i.i.us.us.us.us.i, !dbg !4413
  %_0.i176.us.us.us.us.i473 = fsub float %threshold.i.i.us.us.us.us.i, %hysteresis.i.i.us.us.us.us.i, !dbg !4415
  %_3.i93.us.us.us.us.i474 = fcmp oge float %_0.i.i380.us.us.us.us.i, %_0.i176.us.us.us.us.i473, !dbg !4417
  %..i94.us.us.us.us.i = sext i1 %_3.i93.us.us.us.us.i474 to i32, !dbg !4419
  %_0.i331.us.us.us.us.i = sext i1 %_3.i95.us.us.us.us.i472 to i32, !dbg !4421
  %_0.i325.us.us.us.us.i = select i1 %_3.i109.us.us.us.us.i, i32 %_0.i331.us.us.us.us.i, i32 %..i94.us.us.us.us.i, !dbg !4421
  %_0.i337.us.us.us.us.i = xor i32 %..i94.us.us.us.us.i, -1, !dbg !4423
  %_67.i.i.us.us.us.us.i475 = load float, ptr %395, align 4, !dbg !4425, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i107.us.us.us.us.i476 = fcmp ogt float %_67.i.i.us.us.us.us.i475, 0.000000e+00, !dbg !4426
  %_0.i330.us.us.us.us.i = select i1 %_3.i107.us.us.us.us.i476, i32 %_0.i337.us.us.us.us.i, i32 0, !dbg !4428
  %_0.i329.us.us.us.us.i = select i1 %_3.i109.us.us.us.us.i, i32 0, i32 %_0.i330.us.us.us.us.i, !dbg !4430
  %_0.i324.us.us.us.us.i = or i32 %_0.i329.us.us.us.us.i, %_0.i325.us.us.us.us.i, !dbg !4432
  %_5.i251.us.us.us.us.i = and i32 %_0.i324.us.us.us.us.i, 1065353216, !dbg !4434
  %_0.i255.us.us.us.us.i477 = bitcast i32 %_5.i251.us.us.us.us.i to float, !dbg !4436
  %_71.i.i510511.us.us.us.us.i = load float, ptr %396, align 4, !dbg !4438, !alias.scope !4340, !noalias !4341, !noundef !12
  %_0.i175.us.us.us.us.i478 = fadd float %_67.i.i.us.us.us.us.i475, -1.000000e+00, !dbg !4439
  %483 = trunc nsw i32 %_0.i329.us.us.us.us.i to i1, !dbg !4441
  %_4.i249.v.us.us.us.us.i = select i1 %483, float %_0.i175.us.us.us.us.i478, float %_67.i.i.us.us.us.us.i475, !dbg !4441
  %484 = trunc nsw i32 %_0.i325.us.us.us.us.i to i1, !dbg !4443
  %_0.i243.us.us.us.us.i = select i1 %484, float %_71.i.i510511.us.us.us.us.i, float %_4.i249.v.us.us.us.us.i, !dbg !4443
  store float %_0.i243.us.us.us.us.i, ptr %395, align 4, !dbg !4445, !alias.scope !4330, !noalias !4331
  store i32 %_5.i251.us.us.us.us.i, ptr %394, align 4, !dbg !4446, !alias.scope !4330, !noalias !4331
  %_0.i174.us.us.us.us.i479 = fadd float %ratio.i.i.us.us.us.us.i, -1.000000e+00, !dbg !4447
  %_0.i173.us.us.us.us.i480 = fsub float %_0.i.i380.us.us.us.us.i, %threshold.i.i.us.us.us.us.i, !dbg !4449
  %_0.i158.us.us.us.us.i481 = fmul float %_0.i174.us.us.us.us.i479, %_0.i173.us.us.us.us.i480, !dbg !4451
  %485 = fneg float %range.i.i.us.us.us.us.i, !dbg !4453
  %_3.i.i365.inv.us.us.us.us.i = fcmp ogt float %_0.i158.us.us.us.us.i481, %485, !dbg !4455
  %_4.i.i371.v.us.us.us.us.i = select i1 %_3.i.i365.inv.us.us.us.us.i, float %_0.i158.us.us.us.us.i481, float %485, !dbg !4455
  %_3.i.i448.us.us.us.us.i = fcmp olt float %_4.i.i371.v.us.us.us.us.i, 0.000000e+00, !dbg !4458
  %486 = fcmp ule float %_0.i255.us.us.us.us.i477, 0.000000e+00, !dbg !4461
  %487 = select i1 %486, i1 %_3.i.i448.us.us.us.us.i, i1 false, !dbg !4463
  %_0.i236.us.us.us.us.i = select i1 %487, float %_4.i.i371.v.us.us.us.us.i, float 0.000000e+00, !dbg !4463
  %_86.i.i.us.us.us.us.i482 = load float, ptr %397, align 4, !dbg !4464, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i103.us.us.us.us.i483 = fcmp ule float %_0.i236.us.us.us.us.i, %_86.i.i.us.us.us.us.i482, !dbg !4465
  %_87.i.i513.us.us.us.us.i = load i32, ptr %_38.i23, align 4, !dbg !4467, !alias.scope !4340, !noalias !4341, !noundef !12
  %_88.i.i514.us.us.us.us.i = load i32, ptr %398, align 4, !dbg !4468, !alias.scope !4340, !noalias !4341, !noundef !12
  %_4.i229.us.us.us.us.i = select i1 %_3.i103.us.us.us.us.i483, i32 %_88.i.i514.us.us.us.us.i, i32 %_87.i.i513.us.us.us.us.i, !dbg !4469
  %_0.i230.us.us.us.us.i484 = bitcast i32 %_4.i229.us.us.us.us.i to float, !dbg !4471
  %_0.i172.us.us.us.us.i485 = fsub float %_0.i236.us.us.us.us.i, %_86.i.i.us.us.us.us.i482, !dbg !4473
  %_4.i143.us.us.us.us.i = fmul float %_0.i172.us.us.us.us.i485, %_0.i230.us.us.us.us.i484, !dbg !4475
  %_0.i144.us.us.us.us.i486 = fadd float %_86.i.i.us.us.us.us.i482, %_4.i143.us.us.us.us.i, !dbg !4475
  %488 = tail call noundef float @llvm.fabs.f32(float %_0.i144.us.us.us.us.i486), !dbg !4477
  %489 = fcmp uge float %488, 0x3BC79CA100000000, !dbg !4480
  %_0.i214.us.us.us.us.i = select i1 %489, float %_0.i144.us.us.us.us.i486, float 0.000000e+00, !dbg !4482
  store float %_0.i214.us.us.us.us.i, ptr %397, align 4, !dbg !4483, !alias.scope !4330, !noalias !4331
  %_0.i157.us.us.us.us.i487 = fmul float %_0.i214.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !4484
  %_3.i.i357.us.us.us.us.inv.i = fcmp ogt float %_0.i157.us.us.us.us.i487, -1.260000e+02, !dbg !4487
  %_0.i.i364.us.us.us.us.i = select i1 %_3.i.i357.us.us.us.us.inv.i, float %_0.i157.us.us.us.us.i487, float -1.260000e+02, !dbg !4487
  %_3.i.i440.us.us.us.us.inv.i = fcmp olt float %_0.i.i364.us.us.us.us.i, 1.270000e+02, !dbg !4491
  %_0.i.i447.us.us.us.us.i = select i1 %_3.i.i440.us.us.us.us.inv.i, float %_0.i.i364.us.us.us.us.i, float 1.270000e+02, !dbg !4491
  %490 = tail call noundef float @llvm.floor.f32(float %_0.i.i447.us.us.us.us.i), !dbg !4494
  %_0.i171.us.us.us.us.i488 = fsub float %_0.i.i447.us.us.us.us.i, %490, !dbg !4498
  %_0.i155.us.us.us.us.i = fmul float %_0.i171.us.us.us.us.i488, 0x3F5E974FA0000000, !dbg !4500
  %_0.i140.us.us.us.us.i = fadd float %_0.i155.us.us.us.us.i, 0x3F82778560000000, !dbg !4502
  %_0.i155.us.us.us.us.1.i = fmul float %_0.i171.us.us.us.us.i488, %_0.i140.us.us.us.us.i, !dbg !4500
  %_0.i140.us.us.us.us.1.i = fadd float %_0.i155.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4502
  %_0.i155.us.us.us.us.2.i = fmul float %_0.i171.us.us.us.us.i488, %_0.i140.us.us.us.us.1.i, !dbg !4500
  %_0.i140.us.us.us.us.2.i = fadd float %_0.i155.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4502
  %_0.i155.us.us.us.us.3.i = fmul float %_0.i171.us.us.us.us.i488, %_0.i140.us.us.us.us.2.i, !dbg !4500
  %_0.i140.us.us.us.us.3.i = fadd float %_0.i155.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4502
  %_0.i153.us.us.us.us.i = fmul float %_0.i170.us.us.us.us.i456, 0x3F5E974FA0000000, !dbg !4504
  %_0.i138.us.us.us.us.i = fadd float %_0.i153.us.us.us.us.i, 0x3F82778560000000, !dbg !4506
  %_0.i153.us.us.us.us.1.i = fmul float %_0.i170.us.us.us.us.i456, %_0.i138.us.us.us.us.i, !dbg !4504
  %_0.i138.us.us.us.us.1.i = fadd float %_0.i153.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4506
  %_0.i153.us.us.us.us.2.i = fmul float %_0.i170.us.us.us.us.i456, %_0.i138.us.us.us.us.1.i, !dbg !4504
  %_0.i138.us.us.us.us.2.i = fadd float %_0.i153.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4506
  %_0.i153.us.us.us.us.3.i = fmul float %_0.i170.us.us.us.us.i456, %_0.i138.us.us.us.us.2.i, !dbg !4504
  %_0.i138.us.us.us.us.3.i = fadd float %_0.i153.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4506
  %_0.i152.us.us.us.us.i489 = fmul float %_0.i170.us.us.us.us.i456, %_0.i138.us.us.us.us.3.i, !dbg !4508
  %_0.i137.us.us.us.us.i = fadd float %_0.i152.us.us.us.us.i489, 1.000000e+00, !dbg !4510
  %biased.i.us.us.us.us.i490 = fadd float %479, 0x4160000FE0000000, !dbg !4512
  %_4.i83.us.us.us.us.i491 = bitcast float %biased.i.us.us.us.us.i490 to i32, !dbg !4514
  %_3.i84.us.us.us.us.i492 = shl i32 %_4.i83.us.us.us.us.i491, 23, !dbg !4516
  %_0.i85.us.us.us.us.i493 = bitcast i32 %_3.i84.us.us.us.us.i492 to float, !dbg !4517
  %_0.i151.us.us.us.us.i494 = fmul float %_0.i137.us.us.us.us.i, %_0.i85.us.us.us.us.i493, !dbg !4519
  %_3.i91.us.us.us.us.i495 = fcmp une float %_0.i218.us.us.us.us.i, 0.000000e+00, !dbg !4521
  %_3.i115.us.us.us.us.i496 = fcmp ule float %_98.i102.i.us.us.us.us.i, 0.000000e+00, !dbg !4523
  %_0.i326501.not.us.us.us.us.i = and i1 %_3.i115.us.us.us.us.i496, %_3.i91.us.us.us.us.i495, !dbg !4525
  %_0.i162.us.us.us.us.i497 = fmul float %_0.i193.us.us.us.us.i393, %_0.i151.us.us.us.us.i494, !dbg !4525
  %_4.i275.v.us.us.us.us.i = select i1 %_0.i326501.not.us.us.us.us.i, float %_0.i162.us.us.us.us.i497, float %_0.i193.us.us.us.us.i393, !dbg !4528
  %_0.i.us.us.us.us.i498 = fmul float %_0.i171.us.us.us.us.i488, %_0.i140.us.us.us.us.3.i, !dbg !4530
  %_0.i139.us.us.us.us.i = fadd float %_0.i.us.us.us.us.i498, 1.000000e+00, !dbg !4532
  %biased.i86.us.us.us.us.i499 = fadd float %490, 0x4160000FE0000000, !dbg !4534
  %_4.i87.us.us.us.us.i500 = bitcast float %biased.i86.us.us.us.us.i499 to i32, !dbg !4536
  %_3.i88.us.us.us.us.i501 = shl i32 %_4.i87.us.us.us.us.i500, 23, !dbg !4538
  %_0.i89.us.us.us.us.i502 = bitcast i32 %_3.i88.us.us.us.us.i501 to float, !dbg !4539
  %_0.i154.us.us.us.us.i503 = fmul float %_0.i139.us.us.us.us.i, %_0.i89.us.us.us.us.i502, !dbg !4541
  %_3.i90.us.us.us.us.i504 = fcmp une float %_0.i214.us.us.us.us.i, 0.000000e+00, !dbg !4543
  %_98.i.i.us.us.us.us.i505 = load float, ptr %399, align 4, !dbg !4545, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i101.us.us.us.us.i506 = fcmp ule float %_98.i.i.us.us.us.us.i505, 0.000000e+00, !dbg !4546
  %_0.i323518.not.us.us.us.us.i = and i1 %_3.i101.us.us.us.us.i506, %_3.i90.us.us.us.us.i504, !dbg !4548
  %_0.i156.us.us.us.us.i507 = fmul float %_0.i191.us.us.us.us.i397, %_0.i154.us.us.us.us.i503, !dbg !4548
  %_4.i222.v.us.us.us.us.i = select i1 %_0.i323518.not.us.us.us.us.i, float %_0.i156.us.us.us.us.i507, float %_0.i191.us.us.us.us.i397, !dbg !4550
  store float %_4.i275.v.us.us.us.us.i, ptr %_97.i.us.us.us.us.i374, align 4, !dbg !4552, !alias.scope !4555, !noalias !3967
  store float %_4.i222.v.us.us.us.us.i, ptr %_115.i.us.us.us.us.i378, align 4, !dbg !4558, !alias.scope !4560, !noalias !3989
  %exitcond1440.not.i = icmp eq i64 %462, %_26, !dbg !4563
  br i1 %exitcond1440.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb32.i.us.us.us.us.i369, !dbg !4566, !llvm.loop !4572

bb8.i81.us.us.us.us.i515:                         ; preds = %bb5.i.preheader.us.us.us.us.i514
  %_34.i.us.us.us.us.i516 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.us.i405, !dbg !4567
  br i1 %_34.i.us.us.us.us.i516, label %bb10.i82.us.us.us.us.i517, label %panic5.i.i206, !dbg !4567

bb10.i82.us.us.us.us.i517:                        ; preds = %bb8.i81.us.us.us.us.i515
  %491 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.us.i401, !dbg !4568
  %left_own.i.us.us.us.us.i518 = load float, ptr %491, align 4, !dbg !4568, !alias.scope !4090, !noalias !4105, !noundef !12
  %492 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.us.i405, !dbg !4567
  %right_own.i.us.us.us.us.i519 = load float, ptr %492, align 4, !dbg !4567, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.i419, !dbg !4110

bb5.i.preheader.us.us.us.us.i514:                 ; preds = %bb50.i.us.us.us.us.i391
  br i1 %_31.i80.us.us.us.us.i406, label %bb8.i81.us.us.us.us.i515, label %panic4.i.i203, !dbg !4568

bb14.i63.preheader.us.us.us.us.i508:              ; preds = %bb50.i.us.us.us.us.i391
  br i1 %_31.i80.us.us.us.us.i406, label %bb17.i.us.us.us.us.i509, label %panic15.i.i195, !dbg !4114

bb23.i.preheader.us.us.us.us.i407:                ; preds = %bb50.i.us.us.us.us.i391
  br i1 %_31.i80.us.us.us.us.i406, label %bb27.i60.us.us.us.us.i408, label %panic28.i.i88, !dbg !4104

bb30.i.us.us.us.i521:                             ; preds = %bb30.i.lr.ph.split.us.split.us.split.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i575
  %iter.sroa.0.0.i637.us.us.us.i = phi i64 [ %493, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i575 ], [ 0, %bb30.i.lr.ph.split.us.split.us.split.us.i ]
  %493 = add nuw nsw i64 %iter.sroa.0.0.i637.us.us.us.i, 1, !dbg !3924
  %_28.i.us.us.us.i522 = trunc i64 %iter.sroa.0.0.i637.us.us.us.i to i32, !dbg !3938
  %now.i.us.us.us.i523 = add i32 %base.i.i49, %_28.i.us.us.us.i522, !dbg !3941
  %_31.i.us.us.us.i524 = and i32 %now.i.us.us.us.i523, %_58.i33, !dbg !3944
  %_30.i.us.us.us.i525 = zext i32 %_31.i.us.us.us.i524 to i64, !dbg !3946
  %exitcond1441.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.us.us.i, %_55, !dbg !3947
  br i1 %exitcond1441.not.i, label %bb33.i.i211, label %bb32.i.us.us.us.i526, !dbg !3947, !prof !180

bb32.i.us.us.us.i526:                             ; preds = %bb30.i.us.us.us.i521
  %_97.i.us.us.us.i527 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i637.us.us.us.i, !dbg !3953
  %_98.not.not.i.us.us.us.i528 = icmp ugt i64 %_64.1.i25, %_30.i.us.us.us.i525, !dbg !3957
  br i1 %_98.not.not.i.us.us.us.i528, label %bb35.i.us.us.us.i529, label %bb36.i.i59, !dbg !3957, !prof !2709

bb35.i.us.us.us.i529:                             ; preds = %bb32.i.us.us.us.i526
  %_0.i201.us.us.us.i = load float, ptr %_97.i.us.us.us.i527, align 4, !dbg !3962, !alias.scope !3964, !noalias !3967, !noundef !12
  %_107.i.us.us.us.i530 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_30.i.us.us.us.i525, !dbg !3968
  store float %_0.i201.us.us.us.i, ptr %_107.i.us.us.us.i530, align 4, !dbg !3972, !alias.scope !3974, !noalias !3921
  %_115.i.us.us.us.i531 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i637.us.us.us.i, !dbg !3977
  %_0.i199.us.us.us.i = load float, ptr %_115.i.us.us.us.i531, align 4, !dbg !3984, !alias.scope !3986, !noalias !3989, !noundef !12
  %_123.i.us.us.us.i532 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_30.i.us.us.us.i525, !dbg !3990
  store float %_0.i199.us.us.us.i, ptr %_123.i.us.us.us.i532, align 4, !dbg !3997, !alias.scope !3999, !noalias !3921
  %_132.not.not.i.us.us.us.i533 = icmp ugt i64 %_65.1.i27, %_30.i.us.us.us.i525, !dbg !4573
  br i1 %_132.not.not.i.us.us.us.i533, label %bb44.i.us.us.us.i535, label %bb45.i.i534, !dbg !4573, !prof !2709

bb44.i.us.us.us.i535:                             ; preds = %bb35.i.us.us.us.i529
  %_131.i.us.us.us.i536 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i47, i64 %iter.sroa.0.0.i637.us.us.us.i, !dbg !4002
  %_0.i197.us.us.us.i = load float, ptr %_131.i.us.us.us.i536, align 4, !dbg !4009, !alias.scope !4011, !noalias !3921, !noundef !12
  %_139.i.us.us.us.i537 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_30.i.us.us.us.i525, !dbg !4014
  store float %_0.i197.us.us.us.i, ptr %_139.i.us.us.us.i537, align 4, !dbg !4021, !alias.scope !4023, !noalias !3921
  %exitcond1442.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.us.us.i, %empty.sroa.6.0.i.i46, !dbg !4571
  br i1 %exitcond1442.not.i, label %bb47.i.i520, label %bb46.i.us.us.us.i538, !dbg !4571, !prof !180

bb46.i.us.us.us.i538:                             ; preds = %bb44.i.us.us.us.i535
  %_148.not.not.i.us.us.us.i539 = icmp ugt i64 %_67.1.i31, %_30.i.us.us.us.i525, !dbg !4569
  br i1 %_148.not.not.i.us.us.us.i539, label %bb48.i.us.us.us.i540, label %bb49.i.i228, !dbg !4569, !prof !2709

bb48.i.us.us.us.i540:                             ; preds = %bb46.i.us.us.us.i538
  %_147.i.us.us.us.i541 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i45, i64 %iter.sroa.0.0.i637.us.us.us.i, !dbg !4026
  %_0.i195.us.us.us.i = load float, ptr %_147.i.us.us.us.i541, align 4, !dbg !4033, !alias.scope !4035, !noalias !3921, !noundef !12
  %_155.i.us.us.us.i542 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_30.i.us.us.us.i525, !dbg !4038
  store float %_0.i195.us.us.us.i, ptr %_155.i.us.us.us.i542, align 4, !dbg !4045, !alias.scope !4047, !noalias !3921
  %_53.i.us.us.us.i543 = sub i32 %now.i.us.us.us.i523, %_59.i34, !dbg !4050
  %_52.i.us.us.us.i544 = and i32 %_53.i.us.us.us.i543, %_58.i33, !dbg !4053
  %_51.i.us.us.us.i545 = zext i32 %_52.i.us.us.us.i544 to i64, !dbg !4054
  %_156.not.not.i.us.us.us.i546 = icmp ugt i64 %_64.1.i25, %_51.i.us.us.us.i545, !dbg !4055
  br i1 %_156.not.not.i.us.us.us.i546, label %bb50.i.us.us.us.i547, label %bb51.i.i72, !dbg !4055, !prof !2709

bb50.i.us.us.us.i547:                             ; preds = %bb48.i.us.us.us.i540
  %_163.i.us.us.us.i548 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_51.i.us.us.us.i545, !dbg !4060
  %_0.i193.us.us.us.i549 = load float, ptr %_163.i.us.us.us.i548, align 4, !dbg !4064, !alias.scope !4066, !noalias !3921, !noundef !12
  %_169.i.us.us.us.i552 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_51.i.us.us.us.i545, !dbg !4069
  %_0.i191.us.us.us.i553 = load float, ptr %_169.i.us.us.us.i552, align 4, !dbg !4077, !alias.scope !4079, !noalias !3921, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4082), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4088), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4090), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4092), !dbg !4085
  %_18.i74.us.us.us.i554 = load i32, ptr %_31.i17, align 4, !dbg !4094, !alias.scope !4096, !noalias !4097, !noundef !12
  %_17.i.us.us.us.i555 = sub i32 %now.i.us.us.us.i523, %_18.i74.us.us.us.i554, !dbg !4099
  %_16.i75.us.us.us.i556 = and i32 %_17.i.us.us.us.i555, %_58.i33, !dbg !4094
  %_15.i76.us.us.us.i557 = zext i32 %_16.i75.us.us.us.i556 to i64, !dbg !4094
  %_26.i.us.us.us.i558 = load i32, ptr %_33.i18, align 4, !dbg !4094, !alias.scope !4101, !noalias !4102, !noundef !12
  %_25.i.us.us.us.i559 = sub i32 %now.i.us.us.us.i523, %_26.i.us.us.us.i558, !dbg !4099
  %_24.i.us.us.us.i560 = and i32 %_25.i.us.us.us.i559, %_58.i33, !dbg !4094
  %_23.i79.us.us.us.i561 = zext i32 %_24.i.us.us.us.i560 to i64, !dbg !4094
  %_31.i80.us.us.us.i562 = icmp samesign ugt i64 %_65.1.i27, %_15.i76.us.us.us.i557, !dbg !4094
  switch i8 %_0.sroa.0.0.i485.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.us.i670
    i8 1, label %bb14.i63.preheader.us.us.us.i664
    i8 2, label %bb23.i.preheader.us.us.us.i563
  ], !dbg !4103

bb27.i60.us.us.us.i564:                           ; preds = %bb23.i.preheader.us.us.us.i563
  %494 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.i557, !dbg !4104
  %_79.i.us.us.us.i565 = load float, ptr %494, align 4, !dbg !4104, !alias.scope !4090, !noalias !4105, !noundef !12
  %_85.i.us.us.us.i566 = icmp samesign ugt i64 %_67.1.i31, %_15.i76.us.us.us.i557, !dbg !4106
  br i1 %_85.i.us.us.us.i566, label %bb29.i61.us.us.us.i567, label %panic30.i.i92, !dbg !4106

bb29.i61.us.us.us.i567:                           ; preds = %bb27.i60.us.us.us.i564
  %495 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_15.i76.us.us.us.i557, !dbg !4106
  %_83.i.us.us.us.i568 = load float, ptr %495, align 4, !dbg !4106, !alias.scope !4092, !noalias !4107, !noundef !12
  %_87.i.us.us.us.i569 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.i561, !dbg !4108
  br i1 %_87.i.us.us.us.i569, label %bb31.i.us.us.us.i570, label %panic32.i.i96, !dbg !4108

bb31.i.us.us.us.i570:                             ; preds = %bb29.i61.us.us.us.i567
  %_89.i.us.us.us.i571 = icmp samesign ugt i64 %_65.1.i27, %_23.i79.us.us.us.i561, !dbg !4109
  br i1 %_89.i.us.us.us.i571, label %bb33.i62.us.us.us.i572, label %panic34.i.i99, !dbg !4109

bb33.i62.us.us.us.i572:                           ; preds = %bb31.i.us.us.us.i570
  %496 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.i561, !dbg !4108
  %_86.i.us.us.us.i573 = load float, ptr %496, align 4, !dbg !4108, !alias.scope !4092, !noalias !4107, !noundef !12
  %497 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_23.i79.us.us.us.i561, !dbg !4109
  %_88.i.us.us.us.i574 = load float, ptr %497, align 4, !dbg !4109, !alias.scope !4090, !noalias !4105, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i575, !dbg !4110

bb17.i.us.us.us.i665:                             ; preds = %bb14.i63.preheader.us.us.us.i664
  %_59.i.us.us.us.i666 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.i561, !dbg !4113
  br i1 %_59.i.us.us.us.i666, label %bb19.i.us.us.us.i667, label %panic17.i.i198, !dbg !4113

bb19.i.us.us.us.i667:                             ; preds = %bb17.i.us.us.us.i665
  %498 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.i557, !dbg !4114
  %left_own16.i.us.us.us.i668 = load float, ptr %498, align 4, !dbg !4114, !alias.scope !4090, !noalias !4105, !noundef !12
  %499 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.i561, !dbg !4113
  %right_own18.i.us.us.us.i669 = load float, ptr %499, align 4, !dbg !4113, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i575, !dbg !4110

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i575: ; preds = %bb10.i82.us.us.us.i673, %bb19.i.us.us.us.i667, %bb33.i62.us.us.us.i572
  %taps.i.sroa.1011457.3.i = phi float [ %right_own.i.us.us.us.i675, %bb10.i82.us.us.us.i673 ], [ %left_own16.i.us.us.us.i668, %bb19.i.us.us.us.i667 ], [ %_88.i.us.us.us.i574, %bb33.i62.us.us.us.i572 ], !dbg !4094
  %taps.i.sroa.681456.3.i = phi float [ %right_own.i.us.us.us.i675, %bb10.i82.us.us.us.i673 ], [ %right_own18.i.us.us.us.i669, %bb19.i.us.us.us.i667 ], [ %_86.i.us.us.us.i573, %bb33.i62.us.us.us.i572 ], !dbg !4094
  %taps.i.sroa.351455.3.i = phi float [ %left_own.i.us.us.us.i674, %bb10.i82.us.us.us.i673 ], [ %right_own18.i.us.us.us.i669, %bb19.i.us.us.us.i667 ], [ %_83.i.us.us.us.i568, %bb33.i62.us.us.us.i572 ], !dbg !4094
  %taps.i.sroa.0.3.i576 = phi float [ %left_own.i.us.us.us.i674, %bb10.i82.us.us.us.i673 ], [ %left_own16.i.us.us.us.i668, %bb19.i.us.us.us.i667 ], [ %_79.i.us.us.us.i565, %bb33.i62.us.us.us.i572 ], !dbg !4094
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4115), !dbg !4118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4119), !dbg !4118
  %threshold.i30.i.us.us.us.i = load float, ptr %_10.i13, align 4, !dbg !4121, !alias.scope !4126, !noalias !4127, !noundef !12
  %ratio.i31.i.us.us.us.i = load float, ptr %378, align 4, !dbg !4128, !alias.scope !4126, !noalias !4127, !noundef !12
  %range.i32.i.us.us.us.i = load float, ptr %379, align 4, !dbg !4130, !alias.scope !4126, !noalias !4127, !noundef !12
  %hysteresis.i33.i.us.us.us.i = load float, ptr %380, align 4, !dbg !4132, !alias.scope !4126, !noalias !4127, !noundef !12
  %500 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.3.i576), !dbg !4134
  %501 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351455.3.i), !dbg !4137
  %_37.i36.i.us.us.us.i = load float, ptr %381, align 4, !dbg !4140, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i127.us.us.us.i577 = fcmp ule float %_37.i36.i.us.us.us.i, 0.000000e+00, !dbg !4145
  %_3.i.i423.us.us.us.i = fcmp ule float %500, %501, !dbg !4147
  %_6.i.i425.us.us.us.i = bitcast float %500 to i32, !dbg !4150
  %_8.i.i427.us.us.us.i = bitcast float %501 to i32, !dbg !4153
  %_4.i.i430.us.us.us.i = select i1 %_3.i.i423.us.us.us.i, i32 %_8.i.i427.us.us.us.i, i32 %_6.i.i425.us.us.us.i, !dbg !4155
  %_4.i321.us.us.us.i578 = select i1 %_3.i127.us.us.us.i577, i32 %_6.i.i425.us.us.us.i, i32 %_4.i.i430.us.us.us.i, !dbg !4156
  %_41.i40.i.us.us.us.i = load float, ptr %382, align 4, !dbg !4158, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i125.us.us.us.i = fcmp ule float %_41.i40.i.us.us.us.i, 0.000000e+00, !dbg !4159
  %_0.i167.us.us.us.i579 = fmul float %500, 5.000000e-01, !dbg !4161
  %_0.i166.us.us.us.i580 = fmul float %501, 5.000000e-01, !dbg !4163
  %_0.i142.us.us.us.i581 = fadd float %_0.i166.us.us.us.i580, %_0.i167.us.us.us.i579, !dbg !4165
  %_6.i309.us.us.us.i = bitcast float %_0.i142.us.us.us.i581 to i32, !dbg !4167
  %_4.i314.us.us.us.i = select i1 %_3.i125.us.us.us.i, i32 %_4.i321.us.us.us.i578, i32 %_6.i309.us.us.us.i, !dbg !4170
  %_0.i315.us.us.us.i = bitcast i32 %_4.i314.us.us.us.i to float, !dbg !4171
  %_3.i.i415.us.us.us.i = fcmp ule float %_0.i315.us.us.us.i, 0x3E45798EE0000000, !dbg !4173
  %_4.i.i421.us.us.us.i = select i1 %_3.i.i415.us.us.us.i, i32 841731191, i32 %_4.i314.us.us.us.i, !dbg !4176
  %_0.i.i422.us.us.us.i582 = bitcast i32 %_4.i.i421.us.us.us.i to float, !dbg !4178
  %_3.i.i.us.us.us.i583 = fcmp ule float %_0.i.i422.us.us.us.i582, 0x3810000000000000, !dbg !4180
  %_4.i.i.us.us.us.i584 = select i1 %_3.i.i.us.us.us.i583, i32 8388608, i32 %_4.i.i421.us.us.us.i, !dbg !4186
  %_5.i202.us.us.us.i = and i32 %_4.i.i.us.us.us.i584, 8388607, !dbg !4188
  %_4.i203.us.us.us.i = or disjoint i32 %_5.i202.us.us.us.i, 1065353216, !dbg !4188
  %significand.i.us.us.us.i585 = bitcast i32 %_4.i203.us.us.us.i to float, !dbg !4190
  %_0.i168.us.us.us.i586 = fadd float %significand.i.us.us.us.i585, -1.000000e+00, !dbg !4192
  %_0.i148.us.us.us.i587 = fmul float %_0.i168.us.us.us.i586, 0x3F9B17A960000000, !dbg !4194
  %502 = fsub float 0x3FBF9A8440000000, %_0.i148.us.us.us.i587, !dbg !4196
  %_0.i148.us.us.us.1.i588 = fmul float %_0.i168.us.us.us.i586, %502, !dbg !4194
  %_0.i134.us.us.us.1.i = fadd float %_0.i148.us.us.us.1.i588, 0xBFD1E3F400000000, !dbg !4196
  %_0.i148.us.us.us.2.i589 = fmul float %_0.i168.us.us.us.i586, %_0.i134.us.us.us.1.i, !dbg !4194
  %_0.i134.us.us.us.2.i = fadd float %_0.i148.us.us.us.2.i589, 0x3FDD544F20000000, !dbg !4196
  %_0.i148.us.us.us.3.i590 = fmul float %_0.i168.us.us.us.i586, %_0.i134.us.us.us.2.i, !dbg !4194
  %_0.i134.us.us.us.3.i = fadd float %_0.i148.us.us.us.3.i590, 0xBFE6FC2A60000000, !dbg !4196
  %_0.i148.us.us.us.4.i = fmul float %_0.i168.us.us.us.i586, %_0.i134.us.us.us.3.i, !dbg !4194
  %_0.i134.us.us.us.4.i = fadd float %_0.i148.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4196
  %_9.i.us.us.us.i591 = lshr i32 %_4.i.i.us.us.us.i584, 23, !dbg !4198
  %_8.i204.us.us.us.i = or disjoint i32 %_9.i.us.us.us.i591, 1258291200, !dbg !4198
  %_7.i.us.us.us.i592 = bitcast i32 %_8.i204.us.us.us.i to float, !dbg !4199
  %exponent.i.us.us.us.i593 = fadd float %_7.i.us.us.us.i592, 0xC160000FE0000000, !dbg !4201
  %_0.i147.us.us.us.i594 = fmul float %_0.i168.us.us.us.i586, %_0.i134.us.us.us.4.i, !dbg !4202
  %_0.i133.us.us.us.i = fadd float %exponent.i.us.us.us.i593, %_0.i147.us.us.us.i594, !dbg !4204
  %_0.i165.us.us.us.i595 = fmul float %_0.i133.us.us.us.i, 0x4018151820000000, !dbg !4206
  %_3.i.i472.us.us.us.inv.i = fcmp olt float %_0.i165.us.us.us.i595, 2.400000e+01, !dbg !4208
  %_0.i.i479.us.us.us.i = select i1 %_3.i.i472.us.us.us.inv.i, float %_0.i165.us.us.us.i595, float 2.400000e+01, !dbg !4208
  %_3.i.i407.us.us.us.inv.i = fcmp ogt float %_0.i.i479.us.us.us.i, -1.600000e+02, !dbg !4211
  %_0.i.i414.us.us.us.i = select i1 %_3.i.i407.us.us.us.inv.i, float %_0.i.i479.us.us.us.i, float -1.600000e+02, !dbg !4211
  %_55.i57.i.us.us.us.i = load float, ptr %383, align 4, !dbg !4214, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i123.us.us.us.i596 = fcmp ule float %_55.i57.i.us.us.us.i, 0.000000e+00, !dbg !4216
  %_3.i99.us.us.us.i597 = fcmp oge float %_0.i.i414.us.us.us.i, %threshold.i30.i.us.us.us.i, !dbg !4218
  %_0.i181.us.us.us.i598 = fsub float %threshold.i30.i.us.us.us.i, %hysteresis.i33.i.us.us.us.i, !dbg !4221
  %_3.i97.us.us.us.i599 = fcmp oge float %_0.i.i414.us.us.us.i, %_0.i181.us.us.us.i598, !dbg !4224
  %..i98.us.us.us.i600 = sext i1 %_3.i97.us.us.us.i599 to i32, !dbg !4226
  %_0.i335.us.us.us.i601 = sext i1 %_3.i99.us.us.us.i597 to i32, !dbg !4228
  %_0.i328.us.us.us.i602 = select i1 %_3.i123.us.us.us.i596, i32 %_0.i335.us.us.us.i601, i32 %..i98.us.us.us.i600, !dbg !4228
  %_0.i339.us.us.us.i = xor i32 %..i98.us.us.us.i600, -1, !dbg !4233
  %_67.i67.i.us.us.us.i = load float, ptr %384, align 4, !dbg !4236, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i121.us.us.us.i603 = fcmp ogt float %_67.i67.i.us.us.us.i, 0.000000e+00, !dbg !4237
  %_0.i334.us.us.us.i = select i1 %_3.i121.us.us.us.i603, i32 %_0.i339.us.us.us.i, i32 0, !dbg !4239
  %_0.i333.us.us.us.i = select i1 %_3.i123.us.us.us.i596, i32 0, i32 %_0.i334.us.us.us.i, !dbg !4241
  %_0.i327.us.us.us.i = or i32 %_0.i333.us.us.us.i, %_0.i328.us.us.us.i602, !dbg !4243
  %_5.i304.us.us.us.i = and i32 %_0.i327.us.us.us.i, 1065353216, !dbg !4246
  %_0.i308.us.us.us.i = bitcast i32 %_5.i304.us.us.us.i to float, !dbg !4248
  %_71.i73.i493494.us.us.us.i = load float, ptr %385, align 4, !dbg !4250, !alias.scope !4143, !noalias !4144, !noundef !12
  %_0.i180.us.us.us.i604 = fadd float %_67.i67.i.us.us.us.i, -1.000000e+00, !dbg !4252
  %503 = trunc nsw i32 %_0.i333.us.us.us.i to i1, !dbg !4254
  %_4.i302.v.us.us.us.i = select i1 %503, float %_0.i180.us.us.us.i604, float %_67.i67.i.us.us.us.i, !dbg !4254
  %504 = trunc nsw i32 %_0.i328.us.us.us.i602 to i1, !dbg !4256
  %_0.i296.us.us.us.i = select i1 %504, float %_71.i73.i493494.us.us.us.i, float %_4.i302.v.us.us.us.i, !dbg !4256
  store float %_0.i296.us.us.us.i, ptr %384, align 4, !dbg !4258, !alias.scope !4126, !noalias !4127
  store i32 %_5.i304.us.us.us.i, ptr %383, align 4, !dbg !4259, !alias.scope !4126, !noalias !4127
  %_0.i179.us.us.us.i605 = fadd float %ratio.i31.i.us.us.us.i, -1.000000e+00, !dbg !4260
  %_0.i178.us.us.us.i606 = fsub float %_0.i.i414.us.us.us.i, %threshold.i30.i.us.us.us.i, !dbg !4262
  %_0.i164.us.us.us.i607 = fmul float %_0.i179.us.us.us.i605, %_0.i178.us.us.us.i606, !dbg !4264
  %505 = fneg float %range.i32.i.us.us.us.i, !dbg !4266
  %_3.i.i398.inv.us.us.us.i = fcmp ogt float %_0.i164.us.us.us.i607, %505, !dbg !4268
  %_4.i.i405.v.us.us.us.i = select i1 %_3.i.i398.inv.us.us.us.i, float %_0.i164.us.us.us.i607, float %505, !dbg !4268
  %_3.i.i464.us.us.us.i = fcmp olt float %_4.i.i405.v.us.us.us.i, 0.000000e+00, !dbg !4271
  %506 = fcmp ule float %_0.i308.us.us.us.i, 0.000000e+00, !dbg !4274
  %507 = select i1 %506, i1 %_3.i.i464.us.us.us.i, i1 false, !dbg !4277
  %_0.i289.us.us.us.i = select i1 %507, float %_4.i.i405.v.us.us.us.i, float 0.000000e+00, !dbg !4277
  %_86.i86.i.us.us.us.i = load float, ptr %386, align 4, !dbg !4278, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i117.us.us.us.i608 = fcmp ule float %_0.i289.us.us.us.i, %_86.i86.i.us.us.us.i, !dbg !4280
  %_87.i88.i496.us.us.us.i = load i32, ptr %366, align 4, !dbg !4282, !alias.scope !4143, !noalias !4144, !noundef !12
  %_88.i89.i497.us.us.us.i = load i32, ptr %387, align 4, !dbg !4283, !alias.scope !4143, !noalias !4144, !noundef !12
  %_4.i282.us.us.us.i = select i1 %_3.i117.us.us.us.i608, i32 %_88.i89.i497.us.us.us.i, i32 %_87.i88.i496.us.us.us.i, !dbg !4284
  %_0.i283.us.us.us.i = bitcast i32 %_4.i282.us.us.us.i to float, !dbg !4286
  %_0.i177.us.us.us.i609 = fsub float %_0.i289.us.us.us.i, %_86.i86.i.us.us.us.i, !dbg !4288
  %_4.i145.us.us.us.i = fmul float %_0.i177.us.us.us.i609, %_0.i283.us.us.us.i, !dbg !4291
  %_0.i146.us.us.us.i610 = fadd float %_86.i86.i.us.us.us.i, %_4.i145.us.us.us.i, !dbg !4291
  %508 = tail call noundef float @llvm.fabs.f32(float %_0.i146.us.us.us.i610), !dbg !4293
  %509 = fcmp uge float %508, 0x3BC79CA100000000, !dbg !4297
  %_0.i218.us.us.us.i = select i1 %509, float %_0.i146.us.us.us.i610, float 0.000000e+00, !dbg !4299
  store float %_0.i218.us.us.us.i, ptr %386, align 4, !dbg !4300, !alias.scope !4126, !noalias !4127
  %_0.i163.us.us.us.i611 = fmul float %_0.i218.us.us.us.i, 0x3FC542A5A0000000, !dbg !4302
  %_3.i.i349.us.us.us.inv.i = fcmp ogt float %_0.i163.us.us.us.i611, -1.260000e+02, !dbg !4306
  %_0.i.i356.us.us.us.i = select i1 %_3.i.i349.us.us.us.inv.i, float %_0.i163.us.us.us.i611, float -1.260000e+02, !dbg !4306
  %_3.i.i432.us.us.us.inv.i = fcmp olt float %_0.i.i356.us.us.us.i, 1.270000e+02, !dbg !4310
  %_0.i.i439.us.us.us.i = select i1 %_3.i.i432.us.us.us.inv.i, float %_0.i.i356.us.us.us.i, float 1.270000e+02, !dbg !4310
  %510 = tail call noundef float @llvm.floor.f32(float %_0.i.i439.us.us.us.i), !dbg !4313
  %_0.i170.us.us.us.i612 = fsub float %_0.i.i439.us.us.us.i, %510, !dbg !4317
  %_98.i102.i.us.us.us.i = load float, ptr %388, align 4, !dbg !4319, !alias.scope !4143, !noalias !4144, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4321), !dbg !4324
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4326), !dbg !4324
  %threshold.i.i.us.us.us.i = load float, ptr %data.i.i.i14, align 4, !dbg !4328, !alias.scope !4330, !noalias !4331, !noundef !12
  %ratio.i.i.us.us.us.i = load float, ptr %389, align 4, !dbg !4332, !alias.scope !4330, !noalias !4331, !noundef !12
  %range.i.i.us.us.us.i = load float, ptr %390, align 4, !dbg !4333, !alias.scope !4330, !noalias !4331, !noundef !12
  %hysteresis.i.i.us.us.us.i = load float, ptr %391, align 4, !dbg !4334, !alias.scope !4330, !noalias !4331, !noundef !12
  %511 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681456.3.i), !dbg !4335
  %512 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011457.3.i), !dbg !4337
  %_37.i.i.us.us.us.i613 = load float, ptr %392, align 4, !dbg !4339, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i113.us.us.us.i614 = fcmp ule float %_37.i.i.us.us.us.i613, 0.000000e+00, !dbg !4342
  %_3.i.i389.us.us.us.i = fcmp ule float %511, %512, !dbg !4344
  %_6.i.i391.us.us.us.i = bitcast float %511 to i32, !dbg !4347
  %_8.i.i393.us.us.us.i = bitcast float %512 to i32, !dbg !4350
  %_4.i.i396.us.us.us.i = select i1 %_3.i.i389.us.us.us.i, i32 %_8.i.i393.us.us.us.i, i32 %_6.i.i391.us.us.us.i, !dbg !4352
  %_4.i268.us.us.us.i = select i1 %_3.i113.us.us.us.i614, i32 %_6.i.i391.us.us.us.i, i32 %_4.i.i396.us.us.us.i, !dbg !4353
  %_41.i.i.us.us.us.i615 = load float, ptr %393, align 4, !dbg !4355, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i111.us.us.us.i616 = fcmp ule float %_41.i.i.us.us.us.i615, 0.000000e+00, !dbg !4356
  %_0.i161.us.us.us.i617 = fmul float %511, 5.000000e-01, !dbg !4358
  %_0.i160.us.us.us.i618 = fmul float %512, 5.000000e-01, !dbg !4360
  %_0.i141.us.us.us.i619 = fadd float %_0.i160.us.us.us.i618, %_0.i161.us.us.us.i617, !dbg !4362
  %_6.i256.us.us.us.i = bitcast float %_0.i141.us.us.us.i619 to i32, !dbg !4364
  %_4.i261.us.us.us.i = select i1 %_3.i111.us.us.us.i616, i32 %_4.i268.us.us.us.i, i32 %_6.i256.us.us.us.i, !dbg !4367
  %_0.i262.us.us.us.i = bitcast i32 %_4.i261.us.us.us.i to float, !dbg !4368
  %_3.i.i381.us.us.us.i = fcmp ule float %_0.i262.us.us.us.i, 0x3E45798EE0000000, !dbg !4370
  %_4.i.i387.us.us.us.i = select i1 %_3.i.i381.us.us.us.i, i32 841731191, i32 %_4.i261.us.us.us.i, !dbg !4373
  %_0.i.i388.us.us.us.i = bitcast i32 %_4.i.i387.us.us.us.i to float, !dbg !4375
  %_3.i.i341.us.us.us.i = fcmp ule float %_0.i.i388.us.us.us.i, 0x3810000000000000, !dbg !4377
  %_4.i.i347.us.us.us.i = select i1 %_3.i.i341.us.us.us.i, i32 8388608, i32 %_4.i.i387.us.us.us.i, !dbg !4382
  %_5.i206.us.us.us.i = and i32 %_4.i.i347.us.us.us.i, 8388607, !dbg !4384
  %_4.i207.us.us.us.i = or disjoint i32 %_5.i206.us.us.us.i, 1065353216, !dbg !4384
  %significand.i208.us.us.us.i = bitcast i32 %_4.i207.us.us.us.i to float, !dbg !4386
  %_0.i169.us.us.us.i620 = fadd float %significand.i208.us.us.us.i, -1.000000e+00, !dbg !4388
  %_0.i150.us.us.us.i621 = fmul float %_0.i169.us.us.us.i620, 0x3F9B17A960000000, !dbg !4390
  %513 = fsub float 0x3FBF9A8440000000, %_0.i150.us.us.us.i621, !dbg !4392
  %_0.i150.us.us.us.1.i622 = fmul float %_0.i169.us.us.us.i620, %513, !dbg !4390
  %_0.i136.us.us.us.1.i = fadd float %_0.i150.us.us.us.1.i622, 0xBFD1E3F400000000, !dbg !4392
  %_0.i150.us.us.us.2.i623 = fmul float %_0.i169.us.us.us.i620, %_0.i136.us.us.us.1.i, !dbg !4390
  %_0.i136.us.us.us.2.i = fadd float %_0.i150.us.us.us.2.i623, 0x3FDD544F20000000, !dbg !4392
  %_0.i150.us.us.us.3.i624 = fmul float %_0.i169.us.us.us.i620, %_0.i136.us.us.us.2.i, !dbg !4390
  %_0.i136.us.us.us.3.i = fadd float %_0.i150.us.us.us.3.i624, 0xBFE6FC2A60000000, !dbg !4392
  %_0.i150.us.us.us.4.i = fmul float %_0.i169.us.us.us.i620, %_0.i136.us.us.us.3.i, !dbg !4390
  %_0.i136.us.us.us.4.i = fadd float %_0.i150.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4392
  %_9.i209.us.us.us.i = lshr i32 %_4.i.i347.us.us.us.i, 23, !dbg !4394
  %_8.i210.us.us.us.i = or disjoint i32 %_9.i209.us.us.us.i, 1258291200, !dbg !4394
  %_7.i211.us.us.us.i = bitcast i32 %_8.i210.us.us.us.i to float, !dbg !4395
  %exponent.i212.us.us.us.i = fadd float %_7.i211.us.us.us.i, 0xC160000FE0000000, !dbg !4397
  %_0.i149.us.us.us.i625 = fmul float %_0.i169.us.us.us.i620, %_0.i136.us.us.us.4.i, !dbg !4398
  %_0.i135.us.us.us.i = fadd float %exponent.i212.us.us.us.i, %_0.i149.us.us.us.i625, !dbg !4400
  %_0.i159.us.us.us.i626 = fmul float %_0.i135.us.us.us.i, 0x4018151820000000, !dbg !4402
  %_3.i.i456.us.us.us.inv.i = fcmp olt float %_0.i159.us.us.us.i626, 2.400000e+01, !dbg !4404
  %_0.i.i463.us.us.us.i = select i1 %_3.i.i456.us.us.us.inv.i, float %_0.i159.us.us.us.i626, float 2.400000e+01, !dbg !4404
  %_3.i.i373.us.us.us.inv.i = fcmp ogt float %_0.i.i463.us.us.us.i, -1.600000e+02, !dbg !4407
  %_0.i.i380.us.us.us.i = select i1 %_3.i.i373.us.us.us.inv.i, float %_0.i.i463.us.us.us.i, float -1.600000e+02, !dbg !4407
  %_55.i.i.us.us.us.i627 = load float, ptr %394, align 4, !dbg !4410, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i109.us.us.us.i = fcmp ule float %_55.i.i.us.us.us.i627, 0.000000e+00, !dbg !4411
  %_3.i95.us.us.us.i628 = fcmp oge float %_0.i.i380.us.us.us.i, %threshold.i.i.us.us.us.i, !dbg !4413
  %_0.i176.us.us.us.i629 = fsub float %threshold.i.i.us.us.us.i, %hysteresis.i.i.us.us.us.i, !dbg !4415
  %_3.i93.us.us.us.i630 = fcmp oge float %_0.i.i380.us.us.us.i, %_0.i176.us.us.us.i629, !dbg !4417
  %..i94.us.us.us.i = sext i1 %_3.i93.us.us.us.i630 to i32, !dbg !4419
  %_0.i331.us.us.us.i = sext i1 %_3.i95.us.us.us.i628 to i32, !dbg !4421
  %_0.i325.us.us.us.i = select i1 %_3.i109.us.us.us.i, i32 %_0.i331.us.us.us.i, i32 %..i94.us.us.us.i, !dbg !4421
  %_0.i337.us.us.us.i = xor i32 %..i94.us.us.us.i, -1, !dbg !4423
  %_67.i.i.us.us.us.i631 = load float, ptr %395, align 4, !dbg !4425, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i107.us.us.us.i632 = fcmp ogt float %_67.i.i.us.us.us.i631, 0.000000e+00, !dbg !4426
  %_0.i330.us.us.us.i = select i1 %_3.i107.us.us.us.i632, i32 %_0.i337.us.us.us.i, i32 0, !dbg !4428
  %_0.i329.us.us.us.i = select i1 %_3.i109.us.us.us.i, i32 0, i32 %_0.i330.us.us.us.i, !dbg !4430
  %_0.i324.us.us.us.i = or i32 %_0.i329.us.us.us.i, %_0.i325.us.us.us.i, !dbg !4432
  %_5.i251.us.us.us.i = and i32 %_0.i324.us.us.us.i, 1065353216, !dbg !4434
  %_0.i255.us.us.us.i633 = bitcast i32 %_5.i251.us.us.us.i to float, !dbg !4436
  %_71.i.i510511.us.us.us.i = load float, ptr %396, align 4, !dbg !4438, !alias.scope !4340, !noalias !4341, !noundef !12
  %_0.i175.us.us.us.i634 = fadd float %_67.i.i.us.us.us.i631, -1.000000e+00, !dbg !4439
  %514 = trunc nsw i32 %_0.i329.us.us.us.i to i1, !dbg !4441
  %_4.i249.v.us.us.us.i = select i1 %514, float %_0.i175.us.us.us.i634, float %_67.i.i.us.us.us.i631, !dbg !4441
  %515 = trunc nsw i32 %_0.i325.us.us.us.i to i1, !dbg !4443
  %_0.i243.us.us.us.i = select i1 %515, float %_71.i.i510511.us.us.us.i, float %_4.i249.v.us.us.us.i, !dbg !4443
  store float %_0.i243.us.us.us.i, ptr %395, align 4, !dbg !4445, !alias.scope !4330, !noalias !4331
  store i32 %_5.i251.us.us.us.i, ptr %394, align 4, !dbg !4446, !alias.scope !4330, !noalias !4331
  %_0.i174.us.us.us.i635 = fadd float %ratio.i.i.us.us.us.i, -1.000000e+00, !dbg !4447
  %_0.i173.us.us.us.i636 = fsub float %_0.i.i380.us.us.us.i, %threshold.i.i.us.us.us.i, !dbg !4449
  %_0.i158.us.us.us.i637 = fmul float %_0.i174.us.us.us.i635, %_0.i173.us.us.us.i636, !dbg !4451
  %516 = fneg float %range.i.i.us.us.us.i, !dbg !4453
  %_3.i.i365.inv.us.us.us.i = fcmp ogt float %_0.i158.us.us.us.i637, %516, !dbg !4455
  %_4.i.i371.v.us.us.us.i = select i1 %_3.i.i365.inv.us.us.us.i, float %_0.i158.us.us.us.i637, float %516, !dbg !4455
  %_3.i.i448.us.us.us.i = fcmp olt float %_4.i.i371.v.us.us.us.i, 0.000000e+00, !dbg !4458
  %517 = fcmp ule float %_0.i255.us.us.us.i633, 0.000000e+00, !dbg !4461
  %518 = select i1 %517, i1 %_3.i.i448.us.us.us.i, i1 false, !dbg !4463
  %_0.i236.us.us.us.i = select i1 %518, float %_4.i.i371.v.us.us.us.i, float 0.000000e+00, !dbg !4463
  %_86.i.i.us.us.us.i638 = load float, ptr %397, align 4, !dbg !4464, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i103.us.us.us.i639 = fcmp ule float %_0.i236.us.us.us.i, %_86.i.i.us.us.us.i638, !dbg !4465
  %_87.i.i513.us.us.us.i = load i32, ptr %_38.i23, align 4, !dbg !4467, !alias.scope !4340, !noalias !4341, !noundef !12
  %_88.i.i514.us.us.us.i = load i32, ptr %398, align 4, !dbg !4468, !alias.scope !4340, !noalias !4341, !noundef !12
  %_4.i229.us.us.us.i = select i1 %_3.i103.us.us.us.i639, i32 %_88.i.i514.us.us.us.i, i32 %_87.i.i513.us.us.us.i, !dbg !4469
  %_0.i230.us.us.us.i640 = bitcast i32 %_4.i229.us.us.us.i to float, !dbg !4471
  %_0.i172.us.us.us.i641 = fsub float %_0.i236.us.us.us.i, %_86.i.i.us.us.us.i638, !dbg !4473
  %_4.i143.us.us.us.i = fmul float %_0.i172.us.us.us.i641, %_0.i230.us.us.us.i640, !dbg !4475
  %_0.i144.us.us.us.i642 = fadd float %_86.i.i.us.us.us.i638, %_4.i143.us.us.us.i, !dbg !4475
  %519 = tail call noundef float @llvm.fabs.f32(float %_0.i144.us.us.us.i642), !dbg !4477
  %520 = fcmp uge float %519, 0x3BC79CA100000000, !dbg !4480
  %_0.i214.us.us.us.i = select i1 %520, float %_0.i144.us.us.us.i642, float 0.000000e+00, !dbg !4482
  store float %_0.i214.us.us.us.i, ptr %397, align 4, !dbg !4483, !alias.scope !4330, !noalias !4331
  %_0.i157.us.us.us.i643 = fmul float %_0.i214.us.us.us.i, 0x3FC542A5A0000000, !dbg !4484
  %_3.i.i357.us.us.us.inv.i = fcmp ogt float %_0.i157.us.us.us.i643, -1.260000e+02, !dbg !4487
  %_0.i.i364.us.us.us.i = select i1 %_3.i.i357.us.us.us.inv.i, float %_0.i157.us.us.us.i643, float -1.260000e+02, !dbg !4487
  %_3.i.i440.us.us.us.inv.i = fcmp olt float %_0.i.i364.us.us.us.i, 1.270000e+02, !dbg !4491
  %_0.i.i447.us.us.us.i = select i1 %_3.i.i440.us.us.us.inv.i, float %_0.i.i364.us.us.us.i, float 1.270000e+02, !dbg !4491
  %521 = tail call noundef float @llvm.floor.f32(float %_0.i.i447.us.us.us.i), !dbg !4494
  %_0.i171.us.us.us.i644 = fsub float %_0.i.i447.us.us.us.i, %521, !dbg !4498
  %_0.i155.us.us.us.i = fmul float %_0.i171.us.us.us.i644, 0x3F5E974FA0000000, !dbg !4500
  %_0.i140.us.us.us.i = fadd float %_0.i155.us.us.us.i, 0x3F82778560000000, !dbg !4502
  %_0.i155.us.us.us.1.i = fmul float %_0.i171.us.us.us.i644, %_0.i140.us.us.us.i, !dbg !4500
  %_0.i140.us.us.us.1.i = fadd float %_0.i155.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4502
  %_0.i155.us.us.us.2.i = fmul float %_0.i171.us.us.us.i644, %_0.i140.us.us.us.1.i, !dbg !4500
  %_0.i140.us.us.us.2.i = fadd float %_0.i155.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4502
  %_0.i155.us.us.us.3.i = fmul float %_0.i171.us.us.us.i644, %_0.i140.us.us.us.2.i, !dbg !4500
  %_0.i140.us.us.us.3.i = fadd float %_0.i155.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4502
  %_0.i153.us.us.us.i = fmul float %_0.i170.us.us.us.i612, 0x3F5E974FA0000000, !dbg !4504
  %_0.i138.us.us.us.i = fadd float %_0.i153.us.us.us.i, 0x3F82778560000000, !dbg !4506
  %_0.i153.us.us.us.1.i = fmul float %_0.i170.us.us.us.i612, %_0.i138.us.us.us.i, !dbg !4504
  %_0.i138.us.us.us.1.i = fadd float %_0.i153.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4506
  %_0.i153.us.us.us.2.i = fmul float %_0.i170.us.us.us.i612, %_0.i138.us.us.us.1.i, !dbg !4504
  %_0.i138.us.us.us.2.i = fadd float %_0.i153.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4506
  %_0.i153.us.us.us.3.i = fmul float %_0.i170.us.us.us.i612, %_0.i138.us.us.us.2.i, !dbg !4504
  %_0.i138.us.us.us.3.i = fadd float %_0.i153.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4506
  %_0.i152.us.us.us.i645 = fmul float %_0.i170.us.us.us.i612, %_0.i138.us.us.us.3.i, !dbg !4508
  %_0.i137.us.us.us.i = fadd float %_0.i152.us.us.us.i645, 1.000000e+00, !dbg !4510
  %biased.i.us.us.us.i646 = fadd float %510, 0x4160000FE0000000, !dbg !4512
  %_4.i83.us.us.us.i647 = bitcast float %biased.i.us.us.us.i646 to i32, !dbg !4514
  %_3.i84.us.us.us.i648 = shl i32 %_4.i83.us.us.us.i647, 23, !dbg !4516
  %_0.i85.us.us.us.i649 = bitcast i32 %_3.i84.us.us.us.i648 to float, !dbg !4517
  %_0.i151.us.us.us.i650 = fmul float %_0.i137.us.us.us.i, %_0.i85.us.us.us.i649, !dbg !4519
  %_3.i91.us.us.us.i651 = fcmp une float %_0.i218.us.us.us.i, 0.000000e+00, !dbg !4521
  %_3.i115.us.us.us.i652 = fcmp ule float %_98.i102.i.us.us.us.i, 0.000000e+00, !dbg !4523
  %_0.i326501.not.us.us.us.i = and i1 %_3.i115.us.us.us.i652, %_3.i91.us.us.us.i651, !dbg !4525
  %_0.i162.us.us.us.i653 = fmul float %_0.i193.us.us.us.i549, %_0.i151.us.us.us.i650, !dbg !4525
  %_4.i275.v.us.us.us.i = select i1 %_0.i326501.not.us.us.us.i, float %_0.i162.us.us.us.i653, float %_0.i193.us.us.us.i549, !dbg !4528
  %_0.i.us.us.us.i654 = fmul float %_0.i171.us.us.us.i644, %_0.i140.us.us.us.3.i, !dbg !4530
  %_0.i139.us.us.us.i = fadd float %_0.i.us.us.us.i654, 1.000000e+00, !dbg !4532
  %biased.i86.us.us.us.i655 = fadd float %521, 0x4160000FE0000000, !dbg !4534
  %_4.i87.us.us.us.i656 = bitcast float %biased.i86.us.us.us.i655 to i32, !dbg !4536
  %_3.i88.us.us.us.i657 = shl i32 %_4.i87.us.us.us.i656, 23, !dbg !4538
  %_0.i89.us.us.us.i658 = bitcast i32 %_3.i88.us.us.us.i657 to float, !dbg !4539
  %_0.i154.us.us.us.i659 = fmul float %_0.i139.us.us.us.i, %_0.i89.us.us.us.i658, !dbg !4541
  %_3.i90.us.us.us.i660 = fcmp une float %_0.i214.us.us.us.i, 0.000000e+00, !dbg !4543
  %_98.i.i.us.us.us.i661 = load float, ptr %399, align 4, !dbg !4545, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i101.us.us.us.i662 = fcmp ule float %_98.i.i.us.us.us.i661, 0.000000e+00, !dbg !4546
  %_0.i323518.not.us.us.us.i = and i1 %_3.i101.us.us.us.i662, %_3.i90.us.us.us.i660, !dbg !4548
  %_0.i156.us.us.us.i663 = fmul float %_0.i191.us.us.us.i553, %_0.i154.us.us.us.i659, !dbg !4548
  %_4.i222.v.us.us.us.i = select i1 %_0.i323518.not.us.us.us.i, float %_0.i156.us.us.us.i663, float %_0.i191.us.us.us.i553, !dbg !4550
  store float %_4.i275.v.us.us.us.i, ptr %_97.i.us.us.us.i527, align 4, !dbg !4552, !alias.scope !4555, !noalias !3967
  store float %_4.i222.v.us.us.us.i, ptr %_115.i.us.us.us.i531, align 4, !dbg !4558, !alias.scope !4560, !noalias !3989
  %exitcond1443.not.i = icmp eq i64 %493, %_26, !dbg !4563
  br i1 %exitcond1443.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb30.i.us.us.us.i521, !dbg !4566, !llvm.loop !4574

bb8.i81.us.us.us.i671:                            ; preds = %bb5.i.preheader.us.us.us.i670
  %_34.i.us.us.us.i672 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.us.i561, !dbg !4567
  br i1 %_34.i.us.us.us.i672, label %bb10.i82.us.us.us.i673, label %panic5.i.i206, !dbg !4567

bb10.i82.us.us.us.i673:                           ; preds = %bb8.i81.us.us.us.i671
  %522 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.us.i557, !dbg !4568
  %left_own.i.us.us.us.i674 = load float, ptr %522, align 4, !dbg !4568, !alias.scope !4090, !noalias !4105, !noundef !12
  %523 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.us.i561, !dbg !4567
  %right_own.i.us.us.us.i675 = load float, ptr %523, align 4, !dbg !4567, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i575, !dbg !4110

bb5.i.preheader.us.us.us.i670:                    ; preds = %bb50.i.us.us.us.i547
  br i1 %_31.i80.us.us.us.i562, label %bb8.i81.us.us.us.i671, label %panic4.i.i203, !dbg !4568

bb14.i63.preheader.us.us.us.i664:                 ; preds = %bb50.i.us.us.us.i547
  br i1 %_31.i80.us.us.us.i562, label %bb17.i.us.us.us.i665, label %panic15.i.i195, !dbg !4114

bb23.i.preheader.us.us.us.i563:                   ; preds = %bb50.i.us.us.us.i547
  br i1 %_31.i80.us.us.us.i562, label %bb27.i60.us.us.us.i564, label %panic28.i.i88, !dbg !4104

bb32.i.us.us.i676:                                ; preds = %bb30.i.lr.ph.split.us.split.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i729
  %iter.sroa.0.0.i637.us.us.i = phi i64 [ %524, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i729 ], [ 0, %bb30.i.lr.ph.split.us.split.us.i ]
  %524 = add nuw nsw i64 %iter.sroa.0.0.i637.us.us.i, 1, !dbg !3924
  %_28.i.us.us.i677 = trunc i64 %iter.sroa.0.0.i637.us.us.i to i32, !dbg !3938
  %now.i.us.us.i678 = add i32 %base.i.i49, %_28.i.us.us.i677, !dbg !3941
  %_31.i.us.us.i679 = and i32 %now.i.us.us.i678, %_58.i33, !dbg !3944
  %_30.i.us.us.i680 = zext i32 %_31.i.us.us.i679 to i64, !dbg !3946
  %_97.i.us.us.i681 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i637.us.us.i, !dbg !3953
  %_98.not.not.i.us.us.i682 = icmp ugt i64 %_64.1.i25, %_30.i.us.us.i680, !dbg !3957
  br i1 %_98.not.not.i.us.us.i682, label %bb35.i.us.us.i683, label %bb36.i.i59, !dbg !3957, !prof !2709

bb35.i.us.us.i683:                                ; preds = %bb32.i.us.us.i676
  %_0.i201.us.us.i = load float, ptr %_97.i.us.us.i681, align 4, !dbg !3962, !alias.scope !3964, !noalias !3967, !noundef !12
  %_107.i.us.us.i684 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_30.i.us.us.i680, !dbg !3968
  store float %_0.i201.us.us.i, ptr %_107.i.us.us.i684, align 4, !dbg !3972, !alias.scope !3974, !noalias !3921
  %_115.i.us.us.i685 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i637.us.us.i, !dbg !3977
  %_0.i199.us.us.i = load float, ptr %_115.i.us.us.i685, align 4, !dbg !3984, !alias.scope !3986, !noalias !3989, !noundef !12
  %_123.i.us.us.i686 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_30.i.us.us.i680, !dbg !3990
  store float %_0.i199.us.us.i, ptr %_123.i.us.us.i686, align 4, !dbg !3997, !alias.scope !3999, !noalias !3921
  %exitcond1444.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.us.i, %side_left.sroa.5.0.i.i48, !dbg !4575
  br i1 %exitcond1444.not.i, label %bb43.i.i830, label %bb42.i.us.us.i687, !dbg !4575, !prof !180

bb42.i.us.us.i687:                                ; preds = %bb35.i.us.us.i683
  %_132.not.not.i.us.us.i688 = icmp ugt i64 %_65.1.i27, %_30.i.us.us.i680, !dbg !4573
  br i1 %_132.not.not.i.us.us.i688, label %bb44.i.us.us.i689, label %bb45.i.i534, !dbg !4573, !prof !2709

bb44.i.us.us.i689:                                ; preds = %bb42.i.us.us.i687
  %_131.i.us.us.i690 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i47, i64 %iter.sroa.0.0.i637.us.us.i, !dbg !4002
  %_0.i197.us.us.i = load float, ptr %_131.i.us.us.i690, align 4, !dbg !4009, !alias.scope !4011, !noalias !3921, !noundef !12
  %_139.i.us.us.i691 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_30.i.us.us.i680, !dbg !4014
  store float %_0.i197.us.us.i, ptr %_139.i.us.us.i691, align 4, !dbg !4021, !alias.scope !4023, !noalias !3921
  %exitcond1445.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.us.i, %empty.sroa.6.0.i.i46, !dbg !4571
  br i1 %exitcond1445.not.i, label %bb47.i.i520, label %bb46.i.us.us.i692, !dbg !4571, !prof !180

bb46.i.us.us.i692:                                ; preds = %bb44.i.us.us.i689
  %_148.not.not.i.us.us.i693 = icmp ugt i64 %_67.1.i31, %_30.i.us.us.i680, !dbg !4569
  br i1 %_148.not.not.i.us.us.i693, label %bb48.i.us.us.i694, label %bb49.i.i228, !dbg !4569, !prof !2709

bb48.i.us.us.i694:                                ; preds = %bb46.i.us.us.i692
  %_147.i.us.us.i695 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i45, i64 %iter.sroa.0.0.i637.us.us.i, !dbg !4026
  %_0.i195.us.us.i = load float, ptr %_147.i.us.us.i695, align 4, !dbg !4033, !alias.scope !4035, !noalias !3921, !noundef !12
  %_155.i.us.us.i696 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_30.i.us.us.i680, !dbg !4038
  store float %_0.i195.us.us.i, ptr %_155.i.us.us.i696, align 4, !dbg !4045, !alias.scope !4047, !noalias !3921
  %_53.i.us.us.i697 = sub i32 %now.i.us.us.i678, %_59.i34, !dbg !4050
  %_52.i.us.us.i698 = and i32 %_53.i.us.us.i697, %_58.i33, !dbg !4053
  %_51.i.us.us.i699 = zext i32 %_52.i.us.us.i698 to i64, !dbg !4054
  %_156.not.not.i.us.us.i700 = icmp ugt i64 %_64.1.i25, %_51.i.us.us.i699, !dbg !4055
  br i1 %_156.not.not.i.us.us.i700, label %bb50.i.us.us.i701, label %bb51.i.i72, !dbg !4055, !prof !2709

bb50.i.us.us.i701:                                ; preds = %bb48.i.us.us.i694
  %_163.i.us.us.i702 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_51.i.us.us.i699, !dbg !4060
  %_0.i193.us.us.i703 = load float, ptr %_163.i.us.us.i702, align 4, !dbg !4064, !alias.scope !4066, !noalias !3921, !noundef !12
  %_169.i.us.us.i706 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_51.i.us.us.i699, !dbg !4069
  %_0.i191.us.us.i707 = load float, ptr %_169.i.us.us.i706, align 4, !dbg !4077, !alias.scope !4079, !noalias !3921, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4082), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4088), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4090), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4092), !dbg !4085
  %_18.i74.us.us.i708 = load i32, ptr %_31.i17, align 4, !dbg !4094, !alias.scope !4096, !noalias !4097, !noundef !12
  %_17.i.us.us.i709 = sub i32 %now.i.us.us.i678, %_18.i74.us.us.i708, !dbg !4099
  %_16.i75.us.us.i710 = and i32 %_17.i.us.us.i709, %_58.i33, !dbg !4094
  %_15.i76.us.us.i711 = zext i32 %_16.i75.us.us.i710 to i64, !dbg !4094
  %_26.i.us.us.i712 = load i32, ptr %_33.i18, align 4, !dbg !4094, !alias.scope !4101, !noalias !4102, !noundef !12
  %_25.i.us.us.i713 = sub i32 %now.i.us.us.i678, %_26.i.us.us.i712, !dbg !4099
  %_24.i.us.us.i714 = and i32 %_25.i.us.us.i713, %_58.i33, !dbg !4094
  %_23.i79.us.us.i715 = zext i32 %_24.i.us.us.i714 to i64, !dbg !4094
  %_31.i80.us.us.i716 = icmp samesign ugt i64 %_65.1.i27, %_15.i76.us.us.i711, !dbg !4094
  switch i8 %_0.sroa.0.0.i485.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.i824
    i8 1, label %bb14.i63.preheader.us.us.i818
    i8 2, label %bb23.i.preheader.us.us.i717
  ], !dbg !4103

bb27.i60.us.us.i718:                              ; preds = %bb23.i.preheader.us.us.i717
  %525 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.i711, !dbg !4104
  %_79.i.us.us.i719 = load float, ptr %525, align 4, !dbg !4104, !alias.scope !4090, !noalias !4105, !noundef !12
  %_85.i.us.us.i720 = icmp samesign ugt i64 %_67.1.i31, %_15.i76.us.us.i711, !dbg !4106
  br i1 %_85.i.us.us.i720, label %bb29.i61.us.us.i721, label %panic30.i.i92, !dbg !4106

bb29.i61.us.us.i721:                              ; preds = %bb27.i60.us.us.i718
  %526 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_15.i76.us.us.i711, !dbg !4106
  %_83.i.us.us.i722 = load float, ptr %526, align 4, !dbg !4106, !alias.scope !4092, !noalias !4107, !noundef !12
  %_87.i.us.us.i723 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.i715, !dbg !4108
  br i1 %_87.i.us.us.i723, label %bb31.i.us.us.i724, label %panic32.i.i96, !dbg !4108

bb31.i.us.us.i724:                                ; preds = %bb29.i61.us.us.i721
  %_89.i.us.us.i725 = icmp samesign ugt i64 %_65.1.i27, %_23.i79.us.us.i715, !dbg !4109
  br i1 %_89.i.us.us.i725, label %bb33.i62.us.us.i726, label %panic34.i.i99, !dbg !4109

bb33.i62.us.us.i726:                              ; preds = %bb31.i.us.us.i724
  %527 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.i715, !dbg !4108
  %_86.i.us.us.i727 = load float, ptr %527, align 4, !dbg !4108, !alias.scope !4092, !noalias !4107, !noundef !12
  %528 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_23.i79.us.us.i715, !dbg !4109
  %_88.i.us.us.i728 = load float, ptr %528, align 4, !dbg !4109, !alias.scope !4090, !noalias !4105, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i729, !dbg !4110

bb17.i.us.us.i819:                                ; preds = %bb14.i63.preheader.us.us.i818
  %_59.i.us.us.i820 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.i715, !dbg !4113
  br i1 %_59.i.us.us.i820, label %bb19.i.us.us.i821, label %panic17.i.i198, !dbg !4113

bb19.i.us.us.i821:                                ; preds = %bb17.i.us.us.i819
  %529 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.i711, !dbg !4114
  %left_own16.i.us.us.i822 = load float, ptr %529, align 4, !dbg !4114, !alias.scope !4090, !noalias !4105, !noundef !12
  %530 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.i715, !dbg !4113
  %right_own18.i.us.us.i823 = load float, ptr %530, align 4, !dbg !4113, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i729, !dbg !4110

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i729: ; preds = %bb10.i82.us.us.i827, %bb19.i.us.us.i821, %bb33.i62.us.us.i726
  %taps.i.sroa.1011457.2.i = phi float [ %right_own.i.us.us.i829, %bb10.i82.us.us.i827 ], [ %left_own16.i.us.us.i822, %bb19.i.us.us.i821 ], [ %_88.i.us.us.i728, %bb33.i62.us.us.i726 ], !dbg !4094
  %taps.i.sroa.681456.2.i = phi float [ %right_own.i.us.us.i829, %bb10.i82.us.us.i827 ], [ %right_own18.i.us.us.i823, %bb19.i.us.us.i821 ], [ %_86.i.us.us.i727, %bb33.i62.us.us.i726 ], !dbg !4094
  %taps.i.sroa.351455.2.i = phi float [ %left_own.i.us.us.i828, %bb10.i82.us.us.i827 ], [ %right_own18.i.us.us.i823, %bb19.i.us.us.i821 ], [ %_83.i.us.us.i722, %bb33.i62.us.us.i726 ], !dbg !4094
  %taps.i.sroa.0.2.i730 = phi float [ %left_own.i.us.us.i828, %bb10.i82.us.us.i827 ], [ %left_own16.i.us.us.i822, %bb19.i.us.us.i821 ], [ %_79.i.us.us.i719, %bb33.i62.us.us.i726 ], !dbg !4094
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4115), !dbg !4118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4119), !dbg !4118
  %threshold.i30.i.us.us.i = load float, ptr %_10.i13, align 4, !dbg !4121, !alias.scope !4126, !noalias !4127, !noundef !12
  %ratio.i31.i.us.us.i = load float, ptr %378, align 4, !dbg !4128, !alias.scope !4126, !noalias !4127, !noundef !12
  %range.i32.i.us.us.i = load float, ptr %379, align 4, !dbg !4130, !alias.scope !4126, !noalias !4127, !noundef !12
  %hysteresis.i33.i.us.us.i = load float, ptr %380, align 4, !dbg !4132, !alias.scope !4126, !noalias !4127, !noundef !12
  %531 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.2.i730), !dbg !4134
  %532 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351455.2.i), !dbg !4137
  %_37.i36.i.us.us.i = load float, ptr %381, align 4, !dbg !4140, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i127.us.us.i731 = fcmp ule float %_37.i36.i.us.us.i, 0.000000e+00, !dbg !4145
  %_3.i.i423.us.us.i = fcmp ule float %531, %532, !dbg !4147
  %_6.i.i425.us.us.i = bitcast float %531 to i32, !dbg !4150
  %_8.i.i427.us.us.i = bitcast float %532 to i32, !dbg !4153
  %_4.i.i430.us.us.i = select i1 %_3.i.i423.us.us.i, i32 %_8.i.i427.us.us.i, i32 %_6.i.i425.us.us.i, !dbg !4155
  %_4.i321.us.us.i732 = select i1 %_3.i127.us.us.i731, i32 %_6.i.i425.us.us.i, i32 %_4.i.i430.us.us.i, !dbg !4156
  %_41.i40.i.us.us.i = load float, ptr %382, align 4, !dbg !4158, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i125.us.us.i = fcmp ule float %_41.i40.i.us.us.i, 0.000000e+00, !dbg !4159
  %_0.i167.us.us.i733 = fmul float %531, 5.000000e-01, !dbg !4161
  %_0.i166.us.us.i734 = fmul float %532, 5.000000e-01, !dbg !4163
  %_0.i142.us.us.i735 = fadd float %_0.i166.us.us.i734, %_0.i167.us.us.i733, !dbg !4165
  %_6.i309.us.us.i = bitcast float %_0.i142.us.us.i735 to i32, !dbg !4167
  %_4.i314.us.us.i = select i1 %_3.i125.us.us.i, i32 %_4.i321.us.us.i732, i32 %_6.i309.us.us.i, !dbg !4170
  %_0.i315.us.us.i = bitcast i32 %_4.i314.us.us.i to float, !dbg !4171
  %_3.i.i415.us.us.i = fcmp ule float %_0.i315.us.us.i, 0x3E45798EE0000000, !dbg !4173
  %_4.i.i421.us.us.i = select i1 %_3.i.i415.us.us.i, i32 841731191, i32 %_4.i314.us.us.i, !dbg !4176
  %_0.i.i422.us.us.i736 = bitcast i32 %_4.i.i421.us.us.i to float, !dbg !4178
  %_3.i.i.us.us.i737 = fcmp ule float %_0.i.i422.us.us.i736, 0x3810000000000000, !dbg !4180
  %_4.i.i.us.us.i738 = select i1 %_3.i.i.us.us.i737, i32 8388608, i32 %_4.i.i421.us.us.i, !dbg !4186
  %_5.i202.us.us.i = and i32 %_4.i.i.us.us.i738, 8388607, !dbg !4188
  %_4.i203.us.us.i = or disjoint i32 %_5.i202.us.us.i, 1065353216, !dbg !4188
  %significand.i.us.us.i739 = bitcast i32 %_4.i203.us.us.i to float, !dbg !4190
  %_0.i168.us.us.i740 = fadd float %significand.i.us.us.i739, -1.000000e+00, !dbg !4192
  %_0.i148.us.us.i741 = fmul float %_0.i168.us.us.i740, 0x3F9B17A960000000, !dbg !4194
  %533 = fsub float 0x3FBF9A8440000000, %_0.i148.us.us.i741, !dbg !4196
  %_0.i148.us.us.1.i742 = fmul float %_0.i168.us.us.i740, %533, !dbg !4194
  %_0.i134.us.us.1.i = fadd float %_0.i148.us.us.1.i742, 0xBFD1E3F400000000, !dbg !4196
  %_0.i148.us.us.2.i743 = fmul float %_0.i168.us.us.i740, %_0.i134.us.us.1.i, !dbg !4194
  %_0.i134.us.us.2.i = fadd float %_0.i148.us.us.2.i743, 0x3FDD544F20000000, !dbg !4196
  %_0.i148.us.us.3.i744 = fmul float %_0.i168.us.us.i740, %_0.i134.us.us.2.i, !dbg !4194
  %_0.i134.us.us.3.i = fadd float %_0.i148.us.us.3.i744, 0xBFE6FC2A60000000, !dbg !4196
  %_0.i148.us.us.4.i = fmul float %_0.i168.us.us.i740, %_0.i134.us.us.3.i, !dbg !4194
  %_0.i134.us.us.4.i = fadd float %_0.i148.us.us.4.i, 0x3FF714B2A0000000, !dbg !4196
  %_9.i.us.us.i745 = lshr i32 %_4.i.i.us.us.i738, 23, !dbg !4198
  %_8.i204.us.us.i = or disjoint i32 %_9.i.us.us.i745, 1258291200, !dbg !4198
  %_7.i.us.us.i746 = bitcast i32 %_8.i204.us.us.i to float, !dbg !4199
  %exponent.i.us.us.i747 = fadd float %_7.i.us.us.i746, 0xC160000FE0000000, !dbg !4201
  %_0.i147.us.us.i748 = fmul float %_0.i168.us.us.i740, %_0.i134.us.us.4.i, !dbg !4202
  %_0.i133.us.us.i = fadd float %exponent.i.us.us.i747, %_0.i147.us.us.i748, !dbg !4204
  %_0.i165.us.us.i749 = fmul float %_0.i133.us.us.i, 0x4018151820000000, !dbg !4206
  %_3.i.i472.us.us.inv.i = fcmp olt float %_0.i165.us.us.i749, 2.400000e+01, !dbg !4208
  %_0.i.i479.us.us.i = select i1 %_3.i.i472.us.us.inv.i, float %_0.i165.us.us.i749, float 2.400000e+01, !dbg !4208
  %_3.i.i407.us.us.inv.i = fcmp ogt float %_0.i.i479.us.us.i, -1.600000e+02, !dbg !4211
  %_0.i.i414.us.us.i = select i1 %_3.i.i407.us.us.inv.i, float %_0.i.i479.us.us.i, float -1.600000e+02, !dbg !4211
  %_55.i57.i.us.us.i = load float, ptr %383, align 4, !dbg !4214, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i123.us.us.i750 = fcmp ule float %_55.i57.i.us.us.i, 0.000000e+00, !dbg !4216
  %_3.i99.us.us.i751 = fcmp oge float %_0.i.i414.us.us.i, %threshold.i30.i.us.us.i, !dbg !4218
  %_0.i181.us.us.i752 = fsub float %threshold.i30.i.us.us.i, %hysteresis.i33.i.us.us.i, !dbg !4221
  %_3.i97.us.us.i753 = fcmp oge float %_0.i.i414.us.us.i, %_0.i181.us.us.i752, !dbg !4224
  %..i98.us.us.i754 = sext i1 %_3.i97.us.us.i753 to i32, !dbg !4226
  %_0.i335.us.us.i755 = sext i1 %_3.i99.us.us.i751 to i32, !dbg !4228
  %_0.i328.us.us.i756 = select i1 %_3.i123.us.us.i750, i32 %_0.i335.us.us.i755, i32 %..i98.us.us.i754, !dbg !4228
  %_0.i339.us.us.i = xor i32 %..i98.us.us.i754, -1, !dbg !4233
  %_67.i67.i.us.us.i = load float, ptr %384, align 4, !dbg !4236, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i121.us.us.i757 = fcmp ogt float %_67.i67.i.us.us.i, 0.000000e+00, !dbg !4237
  %_0.i334.us.us.i = select i1 %_3.i121.us.us.i757, i32 %_0.i339.us.us.i, i32 0, !dbg !4239
  %_0.i333.us.us.i = select i1 %_3.i123.us.us.i750, i32 0, i32 %_0.i334.us.us.i, !dbg !4241
  %_0.i327.us.us.i = or i32 %_0.i333.us.us.i, %_0.i328.us.us.i756, !dbg !4243
  %_5.i304.us.us.i = and i32 %_0.i327.us.us.i, 1065353216, !dbg !4246
  %_0.i308.us.us.i = bitcast i32 %_5.i304.us.us.i to float, !dbg !4248
  %_71.i73.i493494.us.us.i = load float, ptr %385, align 4, !dbg !4250, !alias.scope !4143, !noalias !4144, !noundef !12
  %_0.i180.us.us.i758 = fadd float %_67.i67.i.us.us.i, -1.000000e+00, !dbg !4252
  %534 = trunc nsw i32 %_0.i333.us.us.i to i1, !dbg !4254
  %_4.i302.v.us.us.i = select i1 %534, float %_0.i180.us.us.i758, float %_67.i67.i.us.us.i, !dbg !4254
  %535 = trunc nsw i32 %_0.i328.us.us.i756 to i1, !dbg !4256
  %_0.i296.us.us.i = select i1 %535, float %_71.i73.i493494.us.us.i, float %_4.i302.v.us.us.i, !dbg !4256
  store float %_0.i296.us.us.i, ptr %384, align 4, !dbg !4258, !alias.scope !4126, !noalias !4127
  store i32 %_5.i304.us.us.i, ptr %383, align 4, !dbg !4259, !alias.scope !4126, !noalias !4127
  %_0.i179.us.us.i759 = fadd float %ratio.i31.i.us.us.i, -1.000000e+00, !dbg !4260
  %_0.i178.us.us.i760 = fsub float %_0.i.i414.us.us.i, %threshold.i30.i.us.us.i, !dbg !4262
  %_0.i164.us.us.i761 = fmul float %_0.i179.us.us.i759, %_0.i178.us.us.i760, !dbg !4264
  %536 = fneg float %range.i32.i.us.us.i, !dbg !4266
  %_3.i.i398.inv.us.us.i = fcmp ogt float %_0.i164.us.us.i761, %536, !dbg !4268
  %_4.i.i405.v.us.us.i = select i1 %_3.i.i398.inv.us.us.i, float %_0.i164.us.us.i761, float %536, !dbg !4268
  %_3.i.i464.us.us.i = fcmp olt float %_4.i.i405.v.us.us.i, 0.000000e+00, !dbg !4271
  %537 = fcmp ule float %_0.i308.us.us.i, 0.000000e+00, !dbg !4274
  %538 = select i1 %537, i1 %_3.i.i464.us.us.i, i1 false, !dbg !4277
  %_0.i289.us.us.i = select i1 %538, float %_4.i.i405.v.us.us.i, float 0.000000e+00, !dbg !4277
  %_86.i86.i.us.us.i = load float, ptr %386, align 4, !dbg !4278, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i117.us.us.i762 = fcmp ule float %_0.i289.us.us.i, %_86.i86.i.us.us.i, !dbg !4280
  %_87.i88.i496.us.us.i = load i32, ptr %366, align 4, !dbg !4282, !alias.scope !4143, !noalias !4144, !noundef !12
  %_88.i89.i497.us.us.i = load i32, ptr %387, align 4, !dbg !4283, !alias.scope !4143, !noalias !4144, !noundef !12
  %_4.i282.us.us.i = select i1 %_3.i117.us.us.i762, i32 %_88.i89.i497.us.us.i, i32 %_87.i88.i496.us.us.i, !dbg !4284
  %_0.i283.us.us.i = bitcast i32 %_4.i282.us.us.i to float, !dbg !4286
  %_0.i177.us.us.i763 = fsub float %_0.i289.us.us.i, %_86.i86.i.us.us.i, !dbg !4288
  %_4.i145.us.us.i = fmul float %_0.i177.us.us.i763, %_0.i283.us.us.i, !dbg !4291
  %_0.i146.us.us.i764 = fadd float %_86.i86.i.us.us.i, %_4.i145.us.us.i, !dbg !4291
  %539 = tail call noundef float @llvm.fabs.f32(float %_0.i146.us.us.i764), !dbg !4293
  %540 = fcmp uge float %539, 0x3BC79CA100000000, !dbg !4297
  %_0.i218.us.us.i = select i1 %540, float %_0.i146.us.us.i764, float 0.000000e+00, !dbg !4299
  store float %_0.i218.us.us.i, ptr %386, align 4, !dbg !4300, !alias.scope !4126, !noalias !4127
  %_0.i163.us.us.i765 = fmul float %_0.i218.us.us.i, 0x3FC542A5A0000000, !dbg !4302
  %_3.i.i349.us.us.inv.i = fcmp ogt float %_0.i163.us.us.i765, -1.260000e+02, !dbg !4306
  %_0.i.i356.us.us.i = select i1 %_3.i.i349.us.us.inv.i, float %_0.i163.us.us.i765, float -1.260000e+02, !dbg !4306
  %_3.i.i432.us.us.inv.i = fcmp olt float %_0.i.i356.us.us.i, 1.270000e+02, !dbg !4310
  %_0.i.i439.us.us.i = select i1 %_3.i.i432.us.us.inv.i, float %_0.i.i356.us.us.i, float 1.270000e+02, !dbg !4310
  %541 = tail call noundef float @llvm.floor.f32(float %_0.i.i439.us.us.i), !dbg !4313
  %_0.i170.us.us.i766 = fsub float %_0.i.i439.us.us.i, %541, !dbg !4317
  %_98.i102.i.us.us.i = load float, ptr %388, align 4, !dbg !4319, !alias.scope !4143, !noalias !4144, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4321), !dbg !4324
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4326), !dbg !4324
  %threshold.i.i.us.us.i = load float, ptr %data.i.i.i14, align 4, !dbg !4328, !alias.scope !4330, !noalias !4331, !noundef !12
  %ratio.i.i.us.us.i = load float, ptr %389, align 4, !dbg !4332, !alias.scope !4330, !noalias !4331, !noundef !12
  %range.i.i.us.us.i = load float, ptr %390, align 4, !dbg !4333, !alias.scope !4330, !noalias !4331, !noundef !12
  %hysteresis.i.i.us.us.i = load float, ptr %391, align 4, !dbg !4334, !alias.scope !4330, !noalias !4331, !noundef !12
  %542 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681456.2.i), !dbg !4335
  %543 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011457.2.i), !dbg !4337
  %_37.i.i.us.us.i767 = load float, ptr %392, align 4, !dbg !4339, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i113.us.us.i768 = fcmp ule float %_37.i.i.us.us.i767, 0.000000e+00, !dbg !4342
  %_3.i.i389.us.us.i = fcmp ule float %542, %543, !dbg !4344
  %_6.i.i391.us.us.i = bitcast float %542 to i32, !dbg !4347
  %_8.i.i393.us.us.i = bitcast float %543 to i32, !dbg !4350
  %_4.i.i396.us.us.i = select i1 %_3.i.i389.us.us.i, i32 %_8.i.i393.us.us.i, i32 %_6.i.i391.us.us.i, !dbg !4352
  %_4.i268.us.us.i = select i1 %_3.i113.us.us.i768, i32 %_6.i.i391.us.us.i, i32 %_4.i.i396.us.us.i, !dbg !4353
  %_41.i.i.us.us.i769 = load float, ptr %393, align 4, !dbg !4355, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i111.us.us.i770 = fcmp ule float %_41.i.i.us.us.i769, 0.000000e+00, !dbg !4356
  %_0.i161.us.us.i771 = fmul float %542, 5.000000e-01, !dbg !4358
  %_0.i160.us.us.i772 = fmul float %543, 5.000000e-01, !dbg !4360
  %_0.i141.us.us.i773 = fadd float %_0.i160.us.us.i772, %_0.i161.us.us.i771, !dbg !4362
  %_6.i256.us.us.i = bitcast float %_0.i141.us.us.i773 to i32, !dbg !4364
  %_4.i261.us.us.i = select i1 %_3.i111.us.us.i770, i32 %_4.i268.us.us.i, i32 %_6.i256.us.us.i, !dbg !4367
  %_0.i262.us.us.i = bitcast i32 %_4.i261.us.us.i to float, !dbg !4368
  %_3.i.i381.us.us.i = fcmp ule float %_0.i262.us.us.i, 0x3E45798EE0000000, !dbg !4370
  %_4.i.i387.us.us.i = select i1 %_3.i.i381.us.us.i, i32 841731191, i32 %_4.i261.us.us.i, !dbg !4373
  %_0.i.i388.us.us.i = bitcast i32 %_4.i.i387.us.us.i to float, !dbg !4375
  %_3.i.i341.us.us.i = fcmp ule float %_0.i.i388.us.us.i, 0x3810000000000000, !dbg !4377
  %_4.i.i347.us.us.i = select i1 %_3.i.i341.us.us.i, i32 8388608, i32 %_4.i.i387.us.us.i, !dbg !4382
  %_5.i206.us.us.i = and i32 %_4.i.i347.us.us.i, 8388607, !dbg !4384
  %_4.i207.us.us.i = or disjoint i32 %_5.i206.us.us.i, 1065353216, !dbg !4384
  %significand.i208.us.us.i = bitcast i32 %_4.i207.us.us.i to float, !dbg !4386
  %_0.i169.us.us.i774 = fadd float %significand.i208.us.us.i, -1.000000e+00, !dbg !4388
  %_0.i150.us.us.i775 = fmul float %_0.i169.us.us.i774, 0x3F9B17A960000000, !dbg !4390
  %544 = fsub float 0x3FBF9A8440000000, %_0.i150.us.us.i775, !dbg !4392
  %_0.i150.us.us.1.i776 = fmul float %_0.i169.us.us.i774, %544, !dbg !4390
  %_0.i136.us.us.1.i = fadd float %_0.i150.us.us.1.i776, 0xBFD1E3F400000000, !dbg !4392
  %_0.i150.us.us.2.i777 = fmul float %_0.i169.us.us.i774, %_0.i136.us.us.1.i, !dbg !4390
  %_0.i136.us.us.2.i = fadd float %_0.i150.us.us.2.i777, 0x3FDD544F20000000, !dbg !4392
  %_0.i150.us.us.3.i778 = fmul float %_0.i169.us.us.i774, %_0.i136.us.us.2.i, !dbg !4390
  %_0.i136.us.us.3.i = fadd float %_0.i150.us.us.3.i778, 0xBFE6FC2A60000000, !dbg !4392
  %_0.i150.us.us.4.i = fmul float %_0.i169.us.us.i774, %_0.i136.us.us.3.i, !dbg !4390
  %_0.i136.us.us.4.i = fadd float %_0.i150.us.us.4.i, 0x3FF714B2A0000000, !dbg !4392
  %_9.i209.us.us.i = lshr i32 %_4.i.i347.us.us.i, 23, !dbg !4394
  %_8.i210.us.us.i = or disjoint i32 %_9.i209.us.us.i, 1258291200, !dbg !4394
  %_7.i211.us.us.i = bitcast i32 %_8.i210.us.us.i to float, !dbg !4395
  %exponent.i212.us.us.i = fadd float %_7.i211.us.us.i, 0xC160000FE0000000, !dbg !4397
  %_0.i149.us.us.i779 = fmul float %_0.i169.us.us.i774, %_0.i136.us.us.4.i, !dbg !4398
  %_0.i135.us.us.i = fadd float %exponent.i212.us.us.i, %_0.i149.us.us.i779, !dbg !4400
  %_0.i159.us.us.i780 = fmul float %_0.i135.us.us.i, 0x4018151820000000, !dbg !4402
  %_3.i.i456.us.us.inv.i = fcmp olt float %_0.i159.us.us.i780, 2.400000e+01, !dbg !4404
  %_0.i.i463.us.us.i = select i1 %_3.i.i456.us.us.inv.i, float %_0.i159.us.us.i780, float 2.400000e+01, !dbg !4404
  %_3.i.i373.us.us.inv.i = fcmp ogt float %_0.i.i463.us.us.i, -1.600000e+02, !dbg !4407
  %_0.i.i380.us.us.i = select i1 %_3.i.i373.us.us.inv.i, float %_0.i.i463.us.us.i, float -1.600000e+02, !dbg !4407
  %_55.i.i.us.us.i781 = load float, ptr %394, align 4, !dbg !4410, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i109.us.us.i = fcmp ule float %_55.i.i.us.us.i781, 0.000000e+00, !dbg !4411
  %_3.i95.us.us.i782 = fcmp oge float %_0.i.i380.us.us.i, %threshold.i.i.us.us.i, !dbg !4413
  %_0.i176.us.us.i783 = fsub float %threshold.i.i.us.us.i, %hysteresis.i.i.us.us.i, !dbg !4415
  %_3.i93.us.us.i784 = fcmp oge float %_0.i.i380.us.us.i, %_0.i176.us.us.i783, !dbg !4417
  %..i94.us.us.i = sext i1 %_3.i93.us.us.i784 to i32, !dbg !4419
  %_0.i331.us.us.i = sext i1 %_3.i95.us.us.i782 to i32, !dbg !4421
  %_0.i325.us.us.i = select i1 %_3.i109.us.us.i, i32 %_0.i331.us.us.i, i32 %..i94.us.us.i, !dbg !4421
  %_0.i337.us.us.i = xor i32 %..i94.us.us.i, -1, !dbg !4423
  %_67.i.i.us.us.i785 = load float, ptr %395, align 4, !dbg !4425, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i107.us.us.i786 = fcmp ogt float %_67.i.i.us.us.i785, 0.000000e+00, !dbg !4426
  %_0.i330.us.us.i = select i1 %_3.i107.us.us.i786, i32 %_0.i337.us.us.i, i32 0, !dbg !4428
  %_0.i329.us.us.i = select i1 %_3.i109.us.us.i, i32 0, i32 %_0.i330.us.us.i, !dbg !4430
  %_0.i324.us.us.i = or i32 %_0.i329.us.us.i, %_0.i325.us.us.i, !dbg !4432
  %_5.i251.us.us.i = and i32 %_0.i324.us.us.i, 1065353216, !dbg !4434
  %_0.i255.us.us.i787 = bitcast i32 %_5.i251.us.us.i to float, !dbg !4436
  %_71.i.i510511.us.us.i = load float, ptr %396, align 4, !dbg !4438, !alias.scope !4340, !noalias !4341, !noundef !12
  %_0.i175.us.us.i788 = fadd float %_67.i.i.us.us.i785, -1.000000e+00, !dbg !4439
  %545 = trunc nsw i32 %_0.i329.us.us.i to i1, !dbg !4441
  %_4.i249.v.us.us.i = select i1 %545, float %_0.i175.us.us.i788, float %_67.i.i.us.us.i785, !dbg !4441
  %546 = trunc nsw i32 %_0.i325.us.us.i to i1, !dbg !4443
  %_0.i243.us.us.i = select i1 %546, float %_71.i.i510511.us.us.i, float %_4.i249.v.us.us.i, !dbg !4443
  store float %_0.i243.us.us.i, ptr %395, align 4, !dbg !4445, !alias.scope !4330, !noalias !4331
  store i32 %_5.i251.us.us.i, ptr %394, align 4, !dbg !4446, !alias.scope !4330, !noalias !4331
  %_0.i174.us.us.i789 = fadd float %ratio.i.i.us.us.i, -1.000000e+00, !dbg !4447
  %_0.i173.us.us.i790 = fsub float %_0.i.i380.us.us.i, %threshold.i.i.us.us.i, !dbg !4449
  %_0.i158.us.us.i791 = fmul float %_0.i174.us.us.i789, %_0.i173.us.us.i790, !dbg !4451
  %547 = fneg float %range.i.i.us.us.i, !dbg !4453
  %_3.i.i365.inv.us.us.i = fcmp ogt float %_0.i158.us.us.i791, %547, !dbg !4455
  %_4.i.i371.v.us.us.i = select i1 %_3.i.i365.inv.us.us.i, float %_0.i158.us.us.i791, float %547, !dbg !4455
  %_3.i.i448.us.us.i = fcmp olt float %_4.i.i371.v.us.us.i, 0.000000e+00, !dbg !4458
  %548 = fcmp ule float %_0.i255.us.us.i787, 0.000000e+00, !dbg !4461
  %549 = select i1 %548, i1 %_3.i.i448.us.us.i, i1 false, !dbg !4463
  %_0.i236.us.us.i = select i1 %549, float %_4.i.i371.v.us.us.i, float 0.000000e+00, !dbg !4463
  %_86.i.i.us.us.i792 = load float, ptr %397, align 4, !dbg !4464, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i103.us.us.i793 = fcmp ule float %_0.i236.us.us.i, %_86.i.i.us.us.i792, !dbg !4465
  %_87.i.i513.us.us.i = load i32, ptr %_38.i23, align 4, !dbg !4467, !alias.scope !4340, !noalias !4341, !noundef !12
  %_88.i.i514.us.us.i = load i32, ptr %398, align 4, !dbg !4468, !alias.scope !4340, !noalias !4341, !noundef !12
  %_4.i229.us.us.i = select i1 %_3.i103.us.us.i793, i32 %_88.i.i514.us.us.i, i32 %_87.i.i513.us.us.i, !dbg !4469
  %_0.i230.us.us.i794 = bitcast i32 %_4.i229.us.us.i to float, !dbg !4471
  %_0.i172.us.us.i795 = fsub float %_0.i236.us.us.i, %_86.i.i.us.us.i792, !dbg !4473
  %_4.i143.us.us.i = fmul float %_0.i172.us.us.i795, %_0.i230.us.us.i794, !dbg !4475
  %_0.i144.us.us.i796 = fadd float %_86.i.i.us.us.i792, %_4.i143.us.us.i, !dbg !4475
  %550 = tail call noundef float @llvm.fabs.f32(float %_0.i144.us.us.i796), !dbg !4477
  %551 = fcmp uge float %550, 0x3BC79CA100000000, !dbg !4480
  %_0.i214.us.us.i = select i1 %551, float %_0.i144.us.us.i796, float 0.000000e+00, !dbg !4482
  store float %_0.i214.us.us.i, ptr %397, align 4, !dbg !4483, !alias.scope !4330, !noalias !4331
  %_0.i157.us.us.i797 = fmul float %_0.i214.us.us.i, 0x3FC542A5A0000000, !dbg !4484
  %_3.i.i357.us.us.inv.i = fcmp ogt float %_0.i157.us.us.i797, -1.260000e+02, !dbg !4487
  %_0.i.i364.us.us.i = select i1 %_3.i.i357.us.us.inv.i, float %_0.i157.us.us.i797, float -1.260000e+02, !dbg !4487
  %_3.i.i440.us.us.inv.i = fcmp olt float %_0.i.i364.us.us.i, 1.270000e+02, !dbg !4491
  %_0.i.i447.us.us.i = select i1 %_3.i.i440.us.us.inv.i, float %_0.i.i364.us.us.i, float 1.270000e+02, !dbg !4491
  %552 = tail call noundef float @llvm.floor.f32(float %_0.i.i447.us.us.i), !dbg !4494
  %_0.i171.us.us.i798 = fsub float %_0.i.i447.us.us.i, %552, !dbg !4498
  %_0.i155.us.us.i = fmul float %_0.i171.us.us.i798, 0x3F5E974FA0000000, !dbg !4500
  %_0.i140.us.us.i = fadd float %_0.i155.us.us.i, 0x3F82778560000000, !dbg !4502
  %_0.i155.us.us.1.i = fmul float %_0.i171.us.us.i798, %_0.i140.us.us.i, !dbg !4500
  %_0.i140.us.us.1.i = fadd float %_0.i155.us.us.1.i, 0x3FAC91CE60000000, !dbg !4502
  %_0.i155.us.us.2.i = fmul float %_0.i171.us.us.i798, %_0.i140.us.us.1.i, !dbg !4500
  %_0.i140.us.us.2.i = fadd float %_0.i155.us.us.2.i, 0x3FCEBDB560000000, !dbg !4502
  %_0.i155.us.us.3.i = fmul float %_0.i171.us.us.i798, %_0.i140.us.us.2.i, !dbg !4500
  %_0.i140.us.us.3.i = fadd float %_0.i155.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4502
  %_0.i153.us.us.i = fmul float %_0.i170.us.us.i766, 0x3F5E974FA0000000, !dbg !4504
  %_0.i138.us.us.i = fadd float %_0.i153.us.us.i, 0x3F82778560000000, !dbg !4506
  %_0.i153.us.us.1.i = fmul float %_0.i170.us.us.i766, %_0.i138.us.us.i, !dbg !4504
  %_0.i138.us.us.1.i = fadd float %_0.i153.us.us.1.i, 0x3FAC91CE60000000, !dbg !4506
  %_0.i153.us.us.2.i = fmul float %_0.i170.us.us.i766, %_0.i138.us.us.1.i, !dbg !4504
  %_0.i138.us.us.2.i = fadd float %_0.i153.us.us.2.i, 0x3FCEBDB560000000, !dbg !4506
  %_0.i153.us.us.3.i = fmul float %_0.i170.us.us.i766, %_0.i138.us.us.2.i, !dbg !4504
  %_0.i138.us.us.3.i = fadd float %_0.i153.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4506
  %_0.i152.us.us.i799 = fmul float %_0.i170.us.us.i766, %_0.i138.us.us.3.i, !dbg !4508
  %_0.i137.us.us.i = fadd float %_0.i152.us.us.i799, 1.000000e+00, !dbg !4510
  %biased.i.us.us.i800 = fadd float %541, 0x4160000FE0000000, !dbg !4512
  %_4.i83.us.us.i801 = bitcast float %biased.i.us.us.i800 to i32, !dbg !4514
  %_3.i84.us.us.i802 = shl i32 %_4.i83.us.us.i801, 23, !dbg !4516
  %_0.i85.us.us.i803 = bitcast i32 %_3.i84.us.us.i802 to float, !dbg !4517
  %_0.i151.us.us.i804 = fmul float %_0.i137.us.us.i, %_0.i85.us.us.i803, !dbg !4519
  %_3.i91.us.us.i805 = fcmp une float %_0.i218.us.us.i, 0.000000e+00, !dbg !4521
  %_3.i115.us.us.i806 = fcmp ule float %_98.i102.i.us.us.i, 0.000000e+00, !dbg !4523
  %_0.i326501.not.us.us.i = and i1 %_3.i115.us.us.i806, %_3.i91.us.us.i805, !dbg !4525
  %_0.i162.us.us.i807 = fmul float %_0.i193.us.us.i703, %_0.i151.us.us.i804, !dbg !4525
  %_4.i275.v.us.us.i = select i1 %_0.i326501.not.us.us.i, float %_0.i162.us.us.i807, float %_0.i193.us.us.i703, !dbg !4528
  %_0.i.us.us.i808 = fmul float %_0.i171.us.us.i798, %_0.i140.us.us.3.i, !dbg !4530
  %_0.i139.us.us.i = fadd float %_0.i.us.us.i808, 1.000000e+00, !dbg !4532
  %biased.i86.us.us.i809 = fadd float %552, 0x4160000FE0000000, !dbg !4534
  %_4.i87.us.us.i810 = bitcast float %biased.i86.us.us.i809 to i32, !dbg !4536
  %_3.i88.us.us.i811 = shl i32 %_4.i87.us.us.i810, 23, !dbg !4538
  %_0.i89.us.us.i812 = bitcast i32 %_3.i88.us.us.i811 to float, !dbg !4539
  %_0.i154.us.us.i813 = fmul float %_0.i139.us.us.i, %_0.i89.us.us.i812, !dbg !4541
  %_3.i90.us.us.i814 = fcmp une float %_0.i214.us.us.i, 0.000000e+00, !dbg !4543
  %_98.i.i.us.us.i815 = load float, ptr %399, align 4, !dbg !4545, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i101.us.us.i816 = fcmp ule float %_98.i.i.us.us.i815, 0.000000e+00, !dbg !4546
  %_0.i323518.not.us.us.i = and i1 %_3.i101.us.us.i816, %_3.i90.us.us.i814, !dbg !4548
  %_0.i156.us.us.i817 = fmul float %_0.i191.us.us.i707, %_0.i154.us.us.i813, !dbg !4548
  %_4.i222.v.us.us.i = select i1 %_0.i323518.not.us.us.i, float %_0.i156.us.us.i817, float %_0.i191.us.us.i707, !dbg !4550
  store float %_4.i275.v.us.us.i, ptr %_97.i.us.us.i681, align 4, !dbg !4552, !alias.scope !4555, !noalias !3967
  store float %_4.i222.v.us.us.i, ptr %_115.i.us.us.i685, align 4, !dbg !4558, !alias.scope !4560, !noalias !3989
  %exitcond1446.not.i = icmp eq i64 %524, %_26, !dbg !4563
  br i1 %exitcond1446.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb32.i.us.us.i676, !dbg !4566, !llvm.loop !4576

bb8.i81.us.us.i825:                               ; preds = %bb5.i.preheader.us.us.i824
  %_34.i.us.us.i826 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.us.i715, !dbg !4567
  br i1 %_34.i.us.us.i826, label %bb10.i82.us.us.i827, label %panic5.i.i206, !dbg !4567

bb10.i82.us.us.i827:                              ; preds = %bb8.i81.us.us.i825
  %553 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.us.i711, !dbg !4568
  %left_own.i.us.us.i828 = load float, ptr %553, align 4, !dbg !4568, !alias.scope !4090, !noalias !4105, !noundef !12
  %554 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.us.i715, !dbg !4567
  %right_own.i.us.us.i829 = load float, ptr %554, align 4, !dbg !4567, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i729, !dbg !4110

bb5.i.preheader.us.us.i824:                       ; preds = %bb50.i.us.us.i701
  br i1 %_31.i80.us.us.i716, label %bb8.i81.us.us.i825, label %panic4.i.i203, !dbg !4568

bb14.i63.preheader.us.us.i818:                    ; preds = %bb50.i.us.us.i701
  br i1 %_31.i80.us.us.i716, label %bb17.i.us.us.i819, label %panic15.i.i195, !dbg !4114

bb23.i.preheader.us.us.i717:                      ; preds = %bb50.i.us.us.i701
  br i1 %_31.i80.us.us.i716, label %bb27.i60.us.us.i718, label %panic28.i.i88, !dbg !4104

bb30.i.us.i831:                                   ; preds = %bb30.i.lr.ph.split.us.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i888
  %iter.sroa.0.0.i637.us.i = phi i64 [ %555, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i888 ], [ 0, %bb30.i.lr.ph.split.us.i ]
  %555 = add nuw nsw i64 %iter.sroa.0.0.i637.us.i, 1, !dbg !3924
  %_28.i.us.i832 = trunc i64 %iter.sroa.0.0.i637.us.i to i32, !dbg !3938
  %now.i.us.i833 = add i32 %base.i.i49, %_28.i.us.i832, !dbg !3941
  %_31.i.us.i834 = and i32 %now.i.us.i833, %_58.i33, !dbg !3944
  %_30.i.us.i835 = zext i32 %_31.i.us.i834 to i64, !dbg !3946
  %exitcond1447.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.i, %_55, !dbg !3947
  br i1 %exitcond1447.not.i, label %bb33.i.i211, label %bb32.i.us.i836, !dbg !3947, !prof !180

bb32.i.us.i836:                                   ; preds = %bb30.i.us.i831
  %_97.i.us.i837 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i637.us.i, !dbg !3953
  %_98.not.not.i.us.i838 = icmp ugt i64 %_64.1.i25, %_30.i.us.i835, !dbg !3957
  br i1 %_98.not.not.i.us.i838, label %bb35.i.us.i839, label %bb36.i.i59, !dbg !3957, !prof !2709

bb35.i.us.i839:                                   ; preds = %bb32.i.us.i836
  %_0.i201.us.i = load float, ptr %_97.i.us.i837, align 4, !dbg !3962, !alias.scope !3964, !noalias !3967, !noundef !12
  %_107.i.us.i840 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_30.i.us.i835, !dbg !3968
  store float %_0.i201.us.i, ptr %_107.i.us.i840, align 4, !dbg !3972, !alias.scope !3974, !noalias !3921
  %_115.i.us.i841 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i637.us.i, !dbg !3977
  %_116.not.not.i.us.i842 = icmp ugt i64 %_66.1.i29, %_30.i.us.i835, !dbg !4577
  br i1 %_116.not.not.i.us.i842, label %bb40.i.us.i844, label %bb41.i.i843, !dbg !4577, !prof !2709

bb40.i.us.i844:                                   ; preds = %bb35.i.us.i839
  %_0.i199.us.i = load float, ptr %_115.i.us.i841, align 4, !dbg !3984, !alias.scope !3986, !noalias !3989, !noundef !12
  %_123.i.us.i845 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_30.i.us.i835, !dbg !3990
  store float %_0.i199.us.i, ptr %_123.i.us.i845, align 4, !dbg !3997, !alias.scope !3999, !noalias !3921
  %exitcond1448.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.i, %side_left.sroa.5.0.i.i48, !dbg !4575
  br i1 %exitcond1448.not.i, label %bb43.i.i830, label %bb42.i.us.i846, !dbg !4575, !prof !180

bb42.i.us.i846:                                   ; preds = %bb40.i.us.i844
  %_132.not.not.i.us.i847 = icmp ugt i64 %_65.1.i27, %_30.i.us.i835, !dbg !4573
  br i1 %_132.not.not.i.us.i847, label %bb44.i.us.i848, label %bb45.i.i534, !dbg !4573, !prof !2709

bb44.i.us.i848:                                   ; preds = %bb42.i.us.i846
  %_131.i.us.i849 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i47, i64 %iter.sroa.0.0.i637.us.i, !dbg !4002
  %_0.i197.us.i = load float, ptr %_131.i.us.i849, align 4, !dbg !4009, !alias.scope !4011, !noalias !3921, !noundef !12
  %_139.i.us.i850 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_30.i.us.i835, !dbg !4014
  store float %_0.i197.us.i, ptr %_139.i.us.i850, align 4, !dbg !4021, !alias.scope !4023, !noalias !3921
  %exitcond1449.not.i = icmp eq i64 %iter.sroa.0.0.i637.us.i, %empty.sroa.6.0.i.i46, !dbg !4571
  br i1 %exitcond1449.not.i, label %bb47.i.i520, label %bb46.i.us.i851, !dbg !4571, !prof !180

bb46.i.us.i851:                                   ; preds = %bb44.i.us.i848
  %_148.not.not.i.us.i852 = icmp ugt i64 %_67.1.i31, %_30.i.us.i835, !dbg !4569
  br i1 %_148.not.not.i.us.i852, label %bb48.i.us.i853, label %bb49.i.i228, !dbg !4569, !prof !2709

bb48.i.us.i853:                                   ; preds = %bb46.i.us.i851
  %_147.i.us.i854 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i45, i64 %iter.sroa.0.0.i637.us.i, !dbg !4026
  %_0.i195.us.i = load float, ptr %_147.i.us.i854, align 4, !dbg !4033, !alias.scope !4035, !noalias !3921, !noundef !12
  %_155.i.us.i855 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_30.i.us.i835, !dbg !4038
  store float %_0.i195.us.i, ptr %_155.i.us.i855, align 4, !dbg !4045, !alias.scope !4047, !noalias !3921
  %_53.i.us.i856 = sub i32 %now.i.us.i833, %_59.i34, !dbg !4050
  %_52.i.us.i857 = and i32 %_53.i.us.i856, %_58.i33, !dbg !4053
  %_51.i.us.i858 = zext i32 %_52.i.us.i857 to i64, !dbg !4054
  %_156.not.not.i.us.i859 = icmp ugt i64 %_64.1.i25, %_51.i.us.i858, !dbg !4055
  br i1 %_156.not.not.i.us.i859, label %bb50.i.us.i860, label %bb51.i.i72, !dbg !4055, !prof !2709

bb50.i.us.i860:                                   ; preds = %bb48.i.us.i853
  %_163.i.us.i861 = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_51.i.us.i858, !dbg !4060
  %_0.i193.us.i862 = load float, ptr %_163.i.us.i861, align 4, !dbg !4064, !alias.scope !4066, !noalias !3921, !noundef !12
  %_164.not.not.i.us.i863 = icmp ugt i64 %_66.1.i29, %_51.i.us.i858, !dbg !4578
  br i1 %_164.not.not.i.us.i863, label %bb53.i.us.i864, label %bb54.i.i241, !dbg !4578, !prof !2709

bb53.i.us.i864:                                   ; preds = %bb50.i.us.i860
  %_169.i.us.i865 = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_51.i.us.i858, !dbg !4069
  %_0.i191.us.i866 = load float, ptr %_169.i.us.i865, align 4, !dbg !4077, !alias.scope !4079, !noalias !3921, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4082), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4088), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4090), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4092), !dbg !4085
  %_18.i74.us.i867 = load i32, ptr %_31.i17, align 4, !dbg !4094, !alias.scope !4096, !noalias !4097, !noundef !12
  %_17.i.us.i868 = sub i32 %now.i.us.i833, %_18.i74.us.i867, !dbg !4099
  %_16.i75.us.i869 = and i32 %_17.i.us.i868, %_58.i33, !dbg !4094
  %_15.i76.us.i870 = zext i32 %_16.i75.us.i869 to i64, !dbg !4094
  %_26.i.us.i871 = load i32, ptr %_33.i18, align 4, !dbg !4094, !alias.scope !4101, !noalias !4102, !noundef !12
  %_25.i.us.i872 = sub i32 %now.i.us.i833, %_26.i.us.i871, !dbg !4099
  %_24.i.us.i873 = and i32 %_25.i.us.i872, %_58.i33, !dbg !4094
  %_23.i79.us.i874 = zext i32 %_24.i.us.i873 to i64, !dbg !4094
  %_31.i80.us.i875 = icmp samesign ugt i64 %_65.1.i27, %_15.i76.us.i870, !dbg !4094
  switch i8 %_0.sroa.0.0.i485.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.i983
    i8 1, label %bb14.i63.preheader.us.i977
    i8 2, label %bb23.i.preheader.us.i876
  ], !dbg !4103

bb27.i60.us.i877:                                 ; preds = %bb23.i.preheader.us.i876
  %556 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.i870, !dbg !4104
  %_79.i.us.i878 = load float, ptr %556, align 4, !dbg !4104, !alias.scope !4090, !noalias !4105, !noundef !12
  %_85.i.us.i879 = icmp samesign ugt i64 %_67.1.i31, %_15.i76.us.i870, !dbg !4106
  br i1 %_85.i.us.i879, label %bb29.i61.us.i880, label %panic30.i.i92, !dbg !4106

bb29.i61.us.i880:                                 ; preds = %bb27.i60.us.i877
  %557 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_15.i76.us.i870, !dbg !4106
  %_83.i.us.i881 = load float, ptr %557, align 4, !dbg !4106, !alias.scope !4092, !noalias !4107, !noundef !12
  %_87.i.us.i882 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.i874, !dbg !4108
  br i1 %_87.i.us.i882, label %bb31.i.us.i883, label %panic32.i.i96, !dbg !4108

bb31.i.us.i883:                                   ; preds = %bb29.i61.us.i880
  %_89.i.us.i884 = icmp samesign ugt i64 %_65.1.i27, %_23.i79.us.i874, !dbg !4109
  br i1 %_89.i.us.i884, label %bb33.i62.us.i885, label %panic34.i.i99, !dbg !4109

bb33.i62.us.i885:                                 ; preds = %bb31.i.us.i883
  %558 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.i874, !dbg !4108
  %_86.i.us.i886 = load float, ptr %558, align 4, !dbg !4108, !alias.scope !4092, !noalias !4107, !noundef !12
  %559 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_23.i79.us.i874, !dbg !4109
  %_88.i.us.i887 = load float, ptr %559, align 4, !dbg !4109, !alias.scope !4090, !noalias !4105, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i888, !dbg !4110

bb17.i.us.i978:                                   ; preds = %bb14.i63.preheader.us.i977
  %_59.i.us.i979 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.i874, !dbg !4113
  br i1 %_59.i.us.i979, label %bb19.i.us.i980, label %panic17.i.i198, !dbg !4113

bb19.i.us.i980:                                   ; preds = %bb17.i.us.i978
  %560 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.i870, !dbg !4114
  %left_own16.i.us.i981 = load float, ptr %560, align 4, !dbg !4114, !alias.scope !4090, !noalias !4105, !noundef !12
  %561 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.i874, !dbg !4113
  %right_own18.i.us.i982 = load float, ptr %561, align 4, !dbg !4113, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i888, !dbg !4110

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i888: ; preds = %bb10.i82.us.i986, %bb19.i.us.i980, %bb33.i62.us.i885
  %taps.i.sroa.1011457.1.i = phi float [ %right_own.i.us.i988, %bb10.i82.us.i986 ], [ %left_own16.i.us.i981, %bb19.i.us.i980 ], [ %_88.i.us.i887, %bb33.i62.us.i885 ], !dbg !4094
  %taps.i.sroa.681456.1.i = phi float [ %right_own.i.us.i988, %bb10.i82.us.i986 ], [ %right_own18.i.us.i982, %bb19.i.us.i980 ], [ %_86.i.us.i886, %bb33.i62.us.i885 ], !dbg !4094
  %taps.i.sroa.351455.1.i = phi float [ %left_own.i.us.i987, %bb10.i82.us.i986 ], [ %right_own18.i.us.i982, %bb19.i.us.i980 ], [ %_83.i.us.i881, %bb33.i62.us.i885 ], !dbg !4094
  %taps.i.sroa.0.1.i889 = phi float [ %left_own.i.us.i987, %bb10.i82.us.i986 ], [ %left_own16.i.us.i981, %bb19.i.us.i980 ], [ %_79.i.us.i878, %bb33.i62.us.i885 ], !dbg !4094
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4115), !dbg !4118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4119), !dbg !4118
  %threshold.i30.i.us.i = load float, ptr %_10.i13, align 4, !dbg !4121, !alias.scope !4126, !noalias !4127, !noundef !12
  %ratio.i31.i.us.i = load float, ptr %378, align 4, !dbg !4128, !alias.scope !4126, !noalias !4127, !noundef !12
  %range.i32.i.us.i = load float, ptr %379, align 4, !dbg !4130, !alias.scope !4126, !noalias !4127, !noundef !12
  %hysteresis.i33.i.us.i = load float, ptr %380, align 4, !dbg !4132, !alias.scope !4126, !noalias !4127, !noundef !12
  %562 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.1.i889), !dbg !4134
  %563 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351455.1.i), !dbg !4137
  %_37.i36.i.us.i = load float, ptr %381, align 4, !dbg !4140, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i127.us.i890 = fcmp ule float %_37.i36.i.us.i, 0.000000e+00, !dbg !4145
  %_3.i.i423.us.i = fcmp ule float %562, %563, !dbg !4147
  %_6.i.i425.us.i = bitcast float %562 to i32, !dbg !4150
  %_8.i.i427.us.i = bitcast float %563 to i32, !dbg !4153
  %_4.i.i430.us.i = select i1 %_3.i.i423.us.i, i32 %_8.i.i427.us.i, i32 %_6.i.i425.us.i, !dbg !4155
  %_4.i321.us.i891 = select i1 %_3.i127.us.i890, i32 %_6.i.i425.us.i, i32 %_4.i.i430.us.i, !dbg !4156
  %_41.i40.i.us.i = load float, ptr %382, align 4, !dbg !4158, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i125.us.i = fcmp ule float %_41.i40.i.us.i, 0.000000e+00, !dbg !4159
  %_0.i167.us.i892 = fmul float %562, 5.000000e-01, !dbg !4161
  %_0.i166.us.i893 = fmul float %563, 5.000000e-01, !dbg !4163
  %_0.i142.us.i894 = fadd float %_0.i166.us.i893, %_0.i167.us.i892, !dbg !4165
  %_6.i309.us.i = bitcast float %_0.i142.us.i894 to i32, !dbg !4167
  %_4.i314.us.i = select i1 %_3.i125.us.i, i32 %_4.i321.us.i891, i32 %_6.i309.us.i, !dbg !4170
  %_0.i315.us.i = bitcast i32 %_4.i314.us.i to float, !dbg !4171
  %_3.i.i415.us.i = fcmp ule float %_0.i315.us.i, 0x3E45798EE0000000, !dbg !4173
  %_4.i.i421.us.i = select i1 %_3.i.i415.us.i, i32 841731191, i32 %_4.i314.us.i, !dbg !4176
  %_0.i.i422.us.i895 = bitcast i32 %_4.i.i421.us.i to float, !dbg !4178
  %_3.i.i.us.i896 = fcmp ule float %_0.i.i422.us.i895, 0x3810000000000000, !dbg !4180
  %_4.i.i.us.i897 = select i1 %_3.i.i.us.i896, i32 8388608, i32 %_4.i.i421.us.i, !dbg !4186
  %_5.i202.us.i = and i32 %_4.i.i.us.i897, 8388607, !dbg !4188
  %_4.i203.us.i = or disjoint i32 %_5.i202.us.i, 1065353216, !dbg !4188
  %significand.i.us.i898 = bitcast i32 %_4.i203.us.i to float, !dbg !4190
  %_0.i168.us.i899 = fadd float %significand.i.us.i898, -1.000000e+00, !dbg !4192
  %_0.i148.us.i900 = fmul float %_0.i168.us.i899, 0x3F9B17A960000000, !dbg !4194
  %564 = fsub float 0x3FBF9A8440000000, %_0.i148.us.i900, !dbg !4196
  %_0.i148.us.1.i901 = fmul float %_0.i168.us.i899, %564, !dbg !4194
  %_0.i134.us.1.i = fadd float %_0.i148.us.1.i901, 0xBFD1E3F400000000, !dbg !4196
  %_0.i148.us.2.i902 = fmul float %_0.i168.us.i899, %_0.i134.us.1.i, !dbg !4194
  %_0.i134.us.2.i = fadd float %_0.i148.us.2.i902, 0x3FDD544F20000000, !dbg !4196
  %_0.i148.us.3.i903 = fmul float %_0.i168.us.i899, %_0.i134.us.2.i, !dbg !4194
  %_0.i134.us.3.i = fadd float %_0.i148.us.3.i903, 0xBFE6FC2A60000000, !dbg !4196
  %_0.i148.us.4.i = fmul float %_0.i168.us.i899, %_0.i134.us.3.i, !dbg !4194
  %_0.i134.us.4.i = fadd float %_0.i148.us.4.i, 0x3FF714B2A0000000, !dbg !4196
  %_9.i.us.i904 = lshr i32 %_4.i.i.us.i897, 23, !dbg !4198
  %_8.i204.us.i = or disjoint i32 %_9.i.us.i904, 1258291200, !dbg !4198
  %_7.i.us.i905 = bitcast i32 %_8.i204.us.i to float, !dbg !4199
  %exponent.i.us.i906 = fadd float %_7.i.us.i905, 0xC160000FE0000000, !dbg !4201
  %_0.i147.us.i907 = fmul float %_0.i168.us.i899, %_0.i134.us.4.i, !dbg !4202
  %_0.i133.us.i = fadd float %exponent.i.us.i906, %_0.i147.us.i907, !dbg !4204
  %_0.i165.us.i908 = fmul float %_0.i133.us.i, 0x4018151820000000, !dbg !4206
  %_3.i.i472.us.inv.i = fcmp olt float %_0.i165.us.i908, 2.400000e+01, !dbg !4208
  %_0.i.i479.us.i = select i1 %_3.i.i472.us.inv.i, float %_0.i165.us.i908, float 2.400000e+01, !dbg !4208
  %_3.i.i407.us.inv.i = fcmp ogt float %_0.i.i479.us.i, -1.600000e+02, !dbg !4211
  %_0.i.i414.us.i = select i1 %_3.i.i407.us.inv.i, float %_0.i.i479.us.i, float -1.600000e+02, !dbg !4211
  %_55.i57.i.us.i = load float, ptr %383, align 4, !dbg !4214, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i123.us.i909 = fcmp ule float %_55.i57.i.us.i, 0.000000e+00, !dbg !4216
  %_3.i99.us.i910 = fcmp oge float %_0.i.i414.us.i, %threshold.i30.i.us.i, !dbg !4218
  %_0.i181.us.i911 = fsub float %threshold.i30.i.us.i, %hysteresis.i33.i.us.i, !dbg !4221
  %_3.i97.us.i912 = fcmp oge float %_0.i.i414.us.i, %_0.i181.us.i911, !dbg !4224
  %..i98.us.i913 = sext i1 %_3.i97.us.i912 to i32, !dbg !4226
  %_0.i335.us.i914 = sext i1 %_3.i99.us.i910 to i32, !dbg !4228
  %_0.i328.us.i915 = select i1 %_3.i123.us.i909, i32 %_0.i335.us.i914, i32 %..i98.us.i913, !dbg !4228
  %_0.i339.us.i = xor i32 %..i98.us.i913, -1, !dbg !4233
  %_67.i67.i.us.i = load float, ptr %384, align 4, !dbg !4236, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i121.us.i916 = fcmp ogt float %_67.i67.i.us.i, 0.000000e+00, !dbg !4237
  %_0.i334.us.i = select i1 %_3.i121.us.i916, i32 %_0.i339.us.i, i32 0, !dbg !4239
  %_0.i333.us.i = select i1 %_3.i123.us.i909, i32 0, i32 %_0.i334.us.i, !dbg !4241
  %_0.i327.us.i = or i32 %_0.i333.us.i, %_0.i328.us.i915, !dbg !4243
  %_5.i304.us.i = and i32 %_0.i327.us.i, 1065353216, !dbg !4246
  %_0.i308.us.i = bitcast i32 %_5.i304.us.i to float, !dbg !4248
  %_71.i73.i493494.us.i = load float, ptr %385, align 4, !dbg !4250, !alias.scope !4143, !noalias !4144, !noundef !12
  %_0.i180.us.i917 = fadd float %_67.i67.i.us.i, -1.000000e+00, !dbg !4252
  %565 = trunc nsw i32 %_0.i333.us.i to i1, !dbg !4254
  %_4.i302.v.us.i = select i1 %565, float %_0.i180.us.i917, float %_67.i67.i.us.i, !dbg !4254
  %566 = trunc nsw i32 %_0.i328.us.i915 to i1, !dbg !4256
  %_0.i296.us.i = select i1 %566, float %_71.i73.i493494.us.i, float %_4.i302.v.us.i, !dbg !4256
  store float %_0.i296.us.i, ptr %384, align 4, !dbg !4258, !alias.scope !4126, !noalias !4127
  store i32 %_5.i304.us.i, ptr %383, align 4, !dbg !4259, !alias.scope !4126, !noalias !4127
  %_0.i179.us.i918 = fadd float %ratio.i31.i.us.i, -1.000000e+00, !dbg !4260
  %_0.i178.us.i919 = fsub float %_0.i.i414.us.i, %threshold.i30.i.us.i, !dbg !4262
  %_0.i164.us.i920 = fmul float %_0.i179.us.i918, %_0.i178.us.i919, !dbg !4264
  %567 = fneg float %range.i32.i.us.i, !dbg !4266
  %_3.i.i398.inv.us.i = fcmp ogt float %_0.i164.us.i920, %567, !dbg !4268
  %_4.i.i405.v.us.i = select i1 %_3.i.i398.inv.us.i, float %_0.i164.us.i920, float %567, !dbg !4268
  %_3.i.i464.us.i = fcmp olt float %_4.i.i405.v.us.i, 0.000000e+00, !dbg !4271
  %568 = fcmp ule float %_0.i308.us.i, 0.000000e+00, !dbg !4274
  %569 = select i1 %568, i1 %_3.i.i464.us.i, i1 false, !dbg !4277
  %_0.i289.us.i = select i1 %569, float %_4.i.i405.v.us.i, float 0.000000e+00, !dbg !4277
  %_86.i86.i.us.i = load float, ptr %386, align 4, !dbg !4278, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i117.us.i921 = fcmp ule float %_0.i289.us.i, %_86.i86.i.us.i, !dbg !4280
  %_87.i88.i496.us.i = load i32, ptr %366, align 4, !dbg !4282, !alias.scope !4143, !noalias !4144, !noundef !12
  %_88.i89.i497.us.i = load i32, ptr %387, align 4, !dbg !4283, !alias.scope !4143, !noalias !4144, !noundef !12
  %_4.i282.us.i = select i1 %_3.i117.us.i921, i32 %_88.i89.i497.us.i, i32 %_87.i88.i496.us.i, !dbg !4284
  %_0.i283.us.i = bitcast i32 %_4.i282.us.i to float, !dbg !4286
  %_0.i177.us.i922 = fsub float %_0.i289.us.i, %_86.i86.i.us.i, !dbg !4288
  %_4.i145.us.i = fmul float %_0.i177.us.i922, %_0.i283.us.i, !dbg !4291
  %_0.i146.us.i923 = fadd float %_86.i86.i.us.i, %_4.i145.us.i, !dbg !4291
  %570 = tail call noundef float @llvm.fabs.f32(float %_0.i146.us.i923), !dbg !4293
  %571 = fcmp uge float %570, 0x3BC79CA100000000, !dbg !4297
  %_0.i218.us.i = select i1 %571, float %_0.i146.us.i923, float 0.000000e+00, !dbg !4299
  store float %_0.i218.us.i, ptr %386, align 4, !dbg !4300, !alias.scope !4126, !noalias !4127
  %_0.i163.us.i924 = fmul float %_0.i218.us.i, 0x3FC542A5A0000000, !dbg !4302
  %_3.i.i349.us.inv.i = fcmp ogt float %_0.i163.us.i924, -1.260000e+02, !dbg !4306
  %_0.i.i356.us.i = select i1 %_3.i.i349.us.inv.i, float %_0.i163.us.i924, float -1.260000e+02, !dbg !4306
  %_3.i.i432.us.inv.i = fcmp olt float %_0.i.i356.us.i, 1.270000e+02, !dbg !4310
  %_0.i.i439.us.i = select i1 %_3.i.i432.us.inv.i, float %_0.i.i356.us.i, float 1.270000e+02, !dbg !4310
  %572 = tail call noundef float @llvm.floor.f32(float %_0.i.i439.us.i), !dbg !4313
  %_0.i170.us.i925 = fsub float %_0.i.i439.us.i, %572, !dbg !4317
  %_98.i102.i.us.i = load float, ptr %388, align 4, !dbg !4319, !alias.scope !4143, !noalias !4144, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4321), !dbg !4324
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4326), !dbg !4324
  %threshold.i.i.us.i = load float, ptr %data.i.i.i14, align 4, !dbg !4328, !alias.scope !4330, !noalias !4331, !noundef !12
  %ratio.i.i.us.i = load float, ptr %389, align 4, !dbg !4332, !alias.scope !4330, !noalias !4331, !noundef !12
  %range.i.i.us.i = load float, ptr %390, align 4, !dbg !4333, !alias.scope !4330, !noalias !4331, !noundef !12
  %hysteresis.i.i.us.i = load float, ptr %391, align 4, !dbg !4334, !alias.scope !4330, !noalias !4331, !noundef !12
  %573 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681456.1.i), !dbg !4335
  %574 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011457.1.i), !dbg !4337
  %_37.i.i.us.i926 = load float, ptr %392, align 4, !dbg !4339, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i113.us.i927 = fcmp ule float %_37.i.i.us.i926, 0.000000e+00, !dbg !4342
  %_3.i.i389.us.i = fcmp ule float %573, %574, !dbg !4344
  %_6.i.i391.us.i = bitcast float %573 to i32, !dbg !4347
  %_8.i.i393.us.i = bitcast float %574 to i32, !dbg !4350
  %_4.i.i396.us.i = select i1 %_3.i.i389.us.i, i32 %_8.i.i393.us.i, i32 %_6.i.i391.us.i, !dbg !4352
  %_4.i268.us.i = select i1 %_3.i113.us.i927, i32 %_6.i.i391.us.i, i32 %_4.i.i396.us.i, !dbg !4353
  %_41.i.i.us.i928 = load float, ptr %393, align 4, !dbg !4355, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i111.us.i929 = fcmp ule float %_41.i.i.us.i928, 0.000000e+00, !dbg !4356
  %_0.i161.us.i930 = fmul float %573, 5.000000e-01, !dbg !4358
  %_0.i160.us.i931 = fmul float %574, 5.000000e-01, !dbg !4360
  %_0.i141.us.i932 = fadd float %_0.i160.us.i931, %_0.i161.us.i930, !dbg !4362
  %_6.i256.us.i = bitcast float %_0.i141.us.i932 to i32, !dbg !4364
  %_4.i261.us.i = select i1 %_3.i111.us.i929, i32 %_4.i268.us.i, i32 %_6.i256.us.i, !dbg !4367
  %_0.i262.us.i = bitcast i32 %_4.i261.us.i to float, !dbg !4368
  %_3.i.i381.us.i = fcmp ule float %_0.i262.us.i, 0x3E45798EE0000000, !dbg !4370
  %_4.i.i387.us.i = select i1 %_3.i.i381.us.i, i32 841731191, i32 %_4.i261.us.i, !dbg !4373
  %_0.i.i388.us.i = bitcast i32 %_4.i.i387.us.i to float, !dbg !4375
  %_3.i.i341.us.i = fcmp ule float %_0.i.i388.us.i, 0x3810000000000000, !dbg !4377
  %_4.i.i347.us.i = select i1 %_3.i.i341.us.i, i32 8388608, i32 %_4.i.i387.us.i, !dbg !4382
  %_5.i206.us.i = and i32 %_4.i.i347.us.i, 8388607, !dbg !4384
  %_4.i207.us.i = or disjoint i32 %_5.i206.us.i, 1065353216, !dbg !4384
  %significand.i208.us.i = bitcast i32 %_4.i207.us.i to float, !dbg !4386
  %_0.i169.us.i933 = fadd float %significand.i208.us.i, -1.000000e+00, !dbg !4388
  %_0.i150.us.i934 = fmul float %_0.i169.us.i933, 0x3F9B17A960000000, !dbg !4390
  %575 = fsub float 0x3FBF9A8440000000, %_0.i150.us.i934, !dbg !4392
  %_0.i150.us.1.i935 = fmul float %_0.i169.us.i933, %575, !dbg !4390
  %_0.i136.us.1.i = fadd float %_0.i150.us.1.i935, 0xBFD1E3F400000000, !dbg !4392
  %_0.i150.us.2.i936 = fmul float %_0.i169.us.i933, %_0.i136.us.1.i, !dbg !4390
  %_0.i136.us.2.i = fadd float %_0.i150.us.2.i936, 0x3FDD544F20000000, !dbg !4392
  %_0.i150.us.3.i937 = fmul float %_0.i169.us.i933, %_0.i136.us.2.i, !dbg !4390
  %_0.i136.us.3.i = fadd float %_0.i150.us.3.i937, 0xBFE6FC2A60000000, !dbg !4392
  %_0.i150.us.4.i = fmul float %_0.i169.us.i933, %_0.i136.us.3.i, !dbg !4390
  %_0.i136.us.4.i = fadd float %_0.i150.us.4.i, 0x3FF714B2A0000000, !dbg !4392
  %_9.i209.us.i = lshr i32 %_4.i.i347.us.i, 23, !dbg !4394
  %_8.i210.us.i = or disjoint i32 %_9.i209.us.i, 1258291200, !dbg !4394
  %_7.i211.us.i = bitcast i32 %_8.i210.us.i to float, !dbg !4395
  %exponent.i212.us.i = fadd float %_7.i211.us.i, 0xC160000FE0000000, !dbg !4397
  %_0.i149.us.i938 = fmul float %_0.i169.us.i933, %_0.i136.us.4.i, !dbg !4398
  %_0.i135.us.i = fadd float %exponent.i212.us.i, %_0.i149.us.i938, !dbg !4400
  %_0.i159.us.i939 = fmul float %_0.i135.us.i, 0x4018151820000000, !dbg !4402
  %_3.i.i456.us.inv.i = fcmp olt float %_0.i159.us.i939, 2.400000e+01, !dbg !4404
  %_0.i.i463.us.i = select i1 %_3.i.i456.us.inv.i, float %_0.i159.us.i939, float 2.400000e+01, !dbg !4404
  %_3.i.i373.us.inv.i = fcmp ogt float %_0.i.i463.us.i, -1.600000e+02, !dbg !4407
  %_0.i.i380.us.i = select i1 %_3.i.i373.us.inv.i, float %_0.i.i463.us.i, float -1.600000e+02, !dbg !4407
  %_55.i.i.us.i940 = load float, ptr %394, align 4, !dbg !4410, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i109.us.i = fcmp ule float %_55.i.i.us.i940, 0.000000e+00, !dbg !4411
  %_3.i95.us.i941 = fcmp oge float %_0.i.i380.us.i, %threshold.i.i.us.i, !dbg !4413
  %_0.i176.us.i942 = fsub float %threshold.i.i.us.i, %hysteresis.i.i.us.i, !dbg !4415
  %_3.i93.us.i943 = fcmp oge float %_0.i.i380.us.i, %_0.i176.us.i942, !dbg !4417
  %..i94.us.i = sext i1 %_3.i93.us.i943 to i32, !dbg !4419
  %_0.i331.us.i = sext i1 %_3.i95.us.i941 to i32, !dbg !4421
  %_0.i325.us.i = select i1 %_3.i109.us.i, i32 %_0.i331.us.i, i32 %..i94.us.i, !dbg !4421
  %_0.i337.us.i = xor i32 %..i94.us.i, -1, !dbg !4423
  %_67.i.i.us.i944 = load float, ptr %395, align 4, !dbg !4425, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i107.us.i945 = fcmp ogt float %_67.i.i.us.i944, 0.000000e+00, !dbg !4426
  %_0.i330.us.i = select i1 %_3.i107.us.i945, i32 %_0.i337.us.i, i32 0, !dbg !4428
  %_0.i329.us.i = select i1 %_3.i109.us.i, i32 0, i32 %_0.i330.us.i, !dbg !4430
  %_0.i324.us.i = or i32 %_0.i329.us.i, %_0.i325.us.i, !dbg !4432
  %_5.i251.us.i = and i32 %_0.i324.us.i, 1065353216, !dbg !4434
  %_0.i255.us.i946 = bitcast i32 %_5.i251.us.i to float, !dbg !4436
  %_71.i.i510511.us.i = load float, ptr %396, align 4, !dbg !4438, !alias.scope !4340, !noalias !4341, !noundef !12
  %_0.i175.us.i947 = fadd float %_67.i.i.us.i944, -1.000000e+00, !dbg !4439
  %576 = trunc nsw i32 %_0.i329.us.i to i1, !dbg !4441
  %_4.i249.v.us.i = select i1 %576, float %_0.i175.us.i947, float %_67.i.i.us.i944, !dbg !4441
  %577 = trunc nsw i32 %_0.i325.us.i to i1, !dbg !4443
  %_0.i243.us.i = select i1 %577, float %_71.i.i510511.us.i, float %_4.i249.v.us.i, !dbg !4443
  store float %_0.i243.us.i, ptr %395, align 4, !dbg !4445, !alias.scope !4330, !noalias !4331
  store i32 %_5.i251.us.i, ptr %394, align 4, !dbg !4446, !alias.scope !4330, !noalias !4331
  %_0.i174.us.i948 = fadd float %ratio.i.i.us.i, -1.000000e+00, !dbg !4447
  %_0.i173.us.i949 = fsub float %_0.i.i380.us.i, %threshold.i.i.us.i, !dbg !4449
  %_0.i158.us.i950 = fmul float %_0.i174.us.i948, %_0.i173.us.i949, !dbg !4451
  %578 = fneg float %range.i.i.us.i, !dbg !4453
  %_3.i.i365.inv.us.i = fcmp ogt float %_0.i158.us.i950, %578, !dbg !4455
  %_4.i.i371.v.us.i = select i1 %_3.i.i365.inv.us.i, float %_0.i158.us.i950, float %578, !dbg !4455
  %_3.i.i448.us.i = fcmp olt float %_4.i.i371.v.us.i, 0.000000e+00, !dbg !4458
  %579 = fcmp ule float %_0.i255.us.i946, 0.000000e+00, !dbg !4461
  %580 = select i1 %579, i1 %_3.i.i448.us.i, i1 false, !dbg !4463
  %_0.i236.us.i = select i1 %580, float %_4.i.i371.v.us.i, float 0.000000e+00, !dbg !4463
  %_86.i.i.us.i951 = load float, ptr %397, align 4, !dbg !4464, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i103.us.i952 = fcmp ule float %_0.i236.us.i, %_86.i.i.us.i951, !dbg !4465
  %_87.i.i513.us.i = load i32, ptr %_38.i23, align 4, !dbg !4467, !alias.scope !4340, !noalias !4341, !noundef !12
  %_88.i.i514.us.i = load i32, ptr %398, align 4, !dbg !4468, !alias.scope !4340, !noalias !4341, !noundef !12
  %_4.i229.us.i = select i1 %_3.i103.us.i952, i32 %_88.i.i514.us.i, i32 %_87.i.i513.us.i, !dbg !4469
  %_0.i230.us.i953 = bitcast i32 %_4.i229.us.i to float, !dbg !4471
  %_0.i172.us.i954 = fsub float %_0.i236.us.i, %_86.i.i.us.i951, !dbg !4473
  %_4.i143.us.i = fmul float %_0.i172.us.i954, %_0.i230.us.i953, !dbg !4475
  %_0.i144.us.i955 = fadd float %_86.i.i.us.i951, %_4.i143.us.i, !dbg !4475
  %581 = tail call noundef float @llvm.fabs.f32(float %_0.i144.us.i955), !dbg !4477
  %582 = fcmp uge float %581, 0x3BC79CA100000000, !dbg !4480
  %_0.i214.us.i = select i1 %582, float %_0.i144.us.i955, float 0.000000e+00, !dbg !4482
  store float %_0.i214.us.i, ptr %397, align 4, !dbg !4483, !alias.scope !4330, !noalias !4331
  %_0.i157.us.i956 = fmul float %_0.i214.us.i, 0x3FC542A5A0000000, !dbg !4484
  %_3.i.i357.us.inv.i = fcmp ogt float %_0.i157.us.i956, -1.260000e+02, !dbg !4487
  %_0.i.i364.us.i = select i1 %_3.i.i357.us.inv.i, float %_0.i157.us.i956, float -1.260000e+02, !dbg !4487
  %_3.i.i440.us.inv.i = fcmp olt float %_0.i.i364.us.i, 1.270000e+02, !dbg !4491
  %_0.i.i447.us.i = select i1 %_3.i.i440.us.inv.i, float %_0.i.i364.us.i, float 1.270000e+02, !dbg !4491
  %583 = tail call noundef float @llvm.floor.f32(float %_0.i.i447.us.i), !dbg !4494
  %_0.i171.us.i957 = fsub float %_0.i.i447.us.i, %583, !dbg !4498
  %_0.i155.us.i = fmul float %_0.i171.us.i957, 0x3F5E974FA0000000, !dbg !4500
  %_0.i140.us.i = fadd float %_0.i155.us.i, 0x3F82778560000000, !dbg !4502
  %_0.i155.us.1.i = fmul float %_0.i171.us.i957, %_0.i140.us.i, !dbg !4500
  %_0.i140.us.1.i = fadd float %_0.i155.us.1.i, 0x3FAC91CE60000000, !dbg !4502
  %_0.i155.us.2.i = fmul float %_0.i171.us.i957, %_0.i140.us.1.i, !dbg !4500
  %_0.i140.us.2.i = fadd float %_0.i155.us.2.i, 0x3FCEBDB560000000, !dbg !4502
  %_0.i155.us.3.i = fmul float %_0.i171.us.i957, %_0.i140.us.2.i, !dbg !4500
  %_0.i140.us.3.i = fadd float %_0.i155.us.3.i, 0x3FE62E4BA0000000, !dbg !4502
  %_0.i153.us.i = fmul float %_0.i170.us.i925, 0x3F5E974FA0000000, !dbg !4504
  %_0.i138.us.i = fadd float %_0.i153.us.i, 0x3F82778560000000, !dbg !4506
  %_0.i153.us.1.i = fmul float %_0.i170.us.i925, %_0.i138.us.i, !dbg !4504
  %_0.i138.us.1.i = fadd float %_0.i153.us.1.i, 0x3FAC91CE60000000, !dbg !4506
  %_0.i153.us.2.i = fmul float %_0.i170.us.i925, %_0.i138.us.1.i, !dbg !4504
  %_0.i138.us.2.i = fadd float %_0.i153.us.2.i, 0x3FCEBDB560000000, !dbg !4506
  %_0.i153.us.3.i = fmul float %_0.i170.us.i925, %_0.i138.us.2.i, !dbg !4504
  %_0.i138.us.3.i = fadd float %_0.i153.us.3.i, 0x3FE62E4BA0000000, !dbg !4506
  %_0.i152.us.i958 = fmul float %_0.i170.us.i925, %_0.i138.us.3.i, !dbg !4508
  %_0.i137.us.i = fadd float %_0.i152.us.i958, 1.000000e+00, !dbg !4510
  %biased.i.us.i959 = fadd float %572, 0x4160000FE0000000, !dbg !4512
  %_4.i83.us.i960 = bitcast float %biased.i.us.i959 to i32, !dbg !4514
  %_3.i84.us.i961 = shl i32 %_4.i83.us.i960, 23, !dbg !4516
  %_0.i85.us.i962 = bitcast i32 %_3.i84.us.i961 to float, !dbg !4517
  %_0.i151.us.i963 = fmul float %_0.i137.us.i, %_0.i85.us.i962, !dbg !4519
  %_3.i91.us.i964 = fcmp une float %_0.i218.us.i, 0.000000e+00, !dbg !4521
  %_3.i115.us.i965 = fcmp ule float %_98.i102.i.us.i, 0.000000e+00, !dbg !4523
  %_0.i326501.not.us.i = and i1 %_3.i115.us.i965, %_3.i91.us.i964, !dbg !4525
  %_0.i162.us.i966 = fmul float %_0.i193.us.i862, %_0.i151.us.i963, !dbg !4525
  %_4.i275.v.us.i = select i1 %_0.i326501.not.us.i, float %_0.i162.us.i966, float %_0.i193.us.i862, !dbg !4528
  %_0.i.us.i967 = fmul float %_0.i171.us.i957, %_0.i140.us.3.i, !dbg !4530
  %_0.i139.us.i = fadd float %_0.i.us.i967, 1.000000e+00, !dbg !4532
  %biased.i86.us.i968 = fadd float %583, 0x4160000FE0000000, !dbg !4534
  %_4.i87.us.i969 = bitcast float %biased.i86.us.i968 to i32, !dbg !4536
  %_3.i88.us.i970 = shl i32 %_4.i87.us.i969, 23, !dbg !4538
  %_0.i89.us.i971 = bitcast i32 %_3.i88.us.i970 to float, !dbg !4539
  %_0.i154.us.i972 = fmul float %_0.i139.us.i, %_0.i89.us.i971, !dbg !4541
  %_3.i90.us.i973 = fcmp une float %_0.i214.us.i, 0.000000e+00, !dbg !4543
  %_98.i.i.us.i974 = load float, ptr %399, align 4, !dbg !4545, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i101.us.i975 = fcmp ule float %_98.i.i.us.i974, 0.000000e+00, !dbg !4546
  %_0.i323518.not.us.i = and i1 %_3.i101.us.i975, %_3.i90.us.i973, !dbg !4548
  %_0.i156.us.i976 = fmul float %_0.i191.us.i866, %_0.i154.us.i972, !dbg !4548
  %_4.i222.v.us.i = select i1 %_0.i323518.not.us.i, float %_0.i156.us.i976, float %_0.i191.us.i866, !dbg !4550
  store float %_4.i275.v.us.i, ptr %_97.i.us.i837, align 4, !dbg !4552, !alias.scope !4555, !noalias !3967
  store float %_4.i222.v.us.i, ptr %_115.i.us.i841, align 4, !dbg !4558, !alias.scope !4560, !noalias !3989
  %exitcond1450.not.i = icmp eq i64 %555, %_26, !dbg !4563
  br i1 %exitcond1450.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb30.i.us.i831, !dbg !4566, !llvm.loop !4579

bb8.i81.us.i984:                                  ; preds = %bb5.i.preheader.us.i983
  %_34.i.us.i985 = icmp samesign ugt i64 %_67.1.i31, %_23.i79.us.i874, !dbg !4567
  br i1 %_34.i.us.i985, label %bb10.i82.us.i986, label %panic5.i.i206, !dbg !4567

bb10.i82.us.i986:                                 ; preds = %bb8.i81.us.i984
  %584 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.us.i870, !dbg !4568
  %left_own.i.us.i987 = load float, ptr %584, align 4, !dbg !4568, !alias.scope !4090, !noalias !4105, !noundef !12
  %585 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.us.i874, !dbg !4567
  %right_own.i.us.i988 = load float, ptr %585, align 4, !dbg !4567, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i888, !dbg !4110

bb5.i.preheader.us.i983:                          ; preds = %bb53.i.us.i864
  br i1 %_31.i80.us.i875, label %bb8.i81.us.i984, label %panic4.i.i203, !dbg !4568

bb14.i63.preheader.us.i977:                       ; preds = %bb53.i.us.i864
  br i1 %_31.i80.us.i875, label %bb17.i.us.i978, label %panic15.i.i195, !dbg !4114

bb23.i.preheader.us.i876:                         ; preds = %bb53.i.us.i864
  br i1 %_31.i80.us.i875, label %bb27.i60.us.i877, label %panic28.i.i88, !dbg !4104

bb33.i.i211:                                      ; preds = %bb30.i.us.us.us.us.us.us.us.i50, %bb30.i.us.us.us.us.us.i213, %bb30.i.us.us.us.i521, %bb30.i.us.i831
  %.us-phi.i212 = phi i64 [ %555, %bb30.i.us.i831 ], [ %493, %bb30.i.us.us.us.i521 ], [ %431, %bb30.i.us.us.us.us.us.i213 ], [ %400, %bb30.i.us.us.us.us.us.us.us.i50 ], !dbg !4580
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_55, i64 noundef %.us-phi.i212, i64 noundef range(i64 0, 2305843009213693952) %_55, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d0dbd696a058609bd94bbe1fa75d0723) #24, !dbg !4580, !noalias !3921
  unreachable, !dbg !4580

bb32.i.i:                                         ; preds = %bb27.i.i44, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i
  %iter.sroa.0.0.i637.i = phi i64 [ %586, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i ], [ 0, %bb27.i.i44 ]
  %586 = add nuw nsw i64 %iter.sroa.0.0.i637.i, 1, !dbg !3924
  %_28.i.i = trunc i64 %iter.sroa.0.0.i637.i to i32, !dbg !3938
  %now.i.i = add i32 %base.i.i49, %_28.i.i, !dbg !3941
  %_31.i.i = and i32 %now.i.i, %_58.i33, !dbg !3944
  %_30.i.i = zext i32 %_31.i.i to i64, !dbg !3946
  %_97.i.i = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i637.i, !dbg !3953
  %_98.not.not.i.i = icmp ugt i64 %_64.1.i25, %_30.i.i, !dbg !3957
  br i1 %_98.not.not.i.i, label %bb35.i.i, label %bb36.i.i59, !dbg !3957, !prof !2709

bb36.i.i59:                                       ; preds = %bb32.i.us.us.us.us.us.us.us.i56, %bb32.i.us.us.us.us.us.i218, %bb32.i.us.us.us.us.i369, %bb32.i.us.us.us.i526, %bb32.i.us.us.i676, %bb32.i.us.i836, %bb32.i.i
  %.us-phi639.i = phi i64 [ %_30.i.us.us.us.i525, %bb32.i.us.us.us.i526 ], [ %_30.i.us.us.us.us.i373, %bb32.i.us.us.us.us.i369 ], [ %_30.i.us.us.us.us.us.i217, %bb32.i.us.us.us.us.us.i218 ], [ %_30.i.i, %bb32.i.i ], [ %_30.i.us.i835, %bb32.i.us.i836 ], [ %_30.i.us.us.i680, %bb32.i.us.us.i676 ], [ %_30.i.us.us.us.us.us.us.us.i54, %bb32.i.us.us.us.us.us.us.us.i56 ]
  %_37.i.le634.i = add nuw nsw i64 %.us-phi639.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi639.i, i64 noundef %_37.i.le634.i, i64 noundef %_64.1.i25, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1eeef3195352adc58f4bfdb015316fef) #24, !dbg !4581, !noalias !3921
  unreachable, !dbg !4581

bb35.i.i:                                         ; preds = %bb32.i.i
  %_0.i201.i = load float, ptr %_97.i.i, align 4, !dbg !3962, !alias.scope !3964, !noalias !3967, !noundef !12
  %_107.i.i = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_30.i.i, !dbg !3968
  store float %_0.i201.i, ptr %_107.i.i, align 4, !dbg !3972, !alias.scope !3974, !noalias !3921
  %exitcond1451.not.i = icmp eq i64 %iter.sroa.0.0.i637.i, %_63, !dbg !4582
  br i1 %exitcond1451.not.i, label %bb39.i.i, label %bb38.i.i, !dbg !4582, !prof !180

bb39.i.i:                                         ; preds = %bb35.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_63, i64 noundef %586, i64 noundef range(i64 0, 2305843009213693952) %_63, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d9529ff5ddc99dd60299cff5ff3cd676) #24, !dbg !4583, !noalias !3921
  unreachable, !dbg !4583

bb38.i.i:                                         ; preds = %bb35.i.i
  %_115.i.i = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i637.i, !dbg !3977
  %_116.not.not.i.i = icmp ugt i64 %_66.1.i29, %_30.i.i, !dbg !4577
  br i1 %_116.not.not.i.i, label %bb40.i.i, label %bb41.i.i843, !dbg !4577, !prof !2709

bb41.i.i843:                                      ; preds = %bb35.i.us.i839, %bb38.i.i
  %.us-phi641.i = phi i64 [ %_30.i.i, %bb38.i.i ], [ %_30.i.us.i835, %bb35.i.us.i839 ]
  %_37.i.le632.i = add nuw nsw i64 %.us-phi641.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi641.i, i64 noundef %_37.i.le632.i, i64 noundef %_66.1.i29, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9d56679ca30f2caa500ddf2e89d00cb) #24, !dbg !4584, !noalias !3921
  unreachable, !dbg !4584

bb40.i.i:                                         ; preds = %bb38.i.i
  %_0.i199.i = load float, ptr %_115.i.i, align 4, !dbg !3984, !alias.scope !3986, !noalias !3989, !noundef !12
  %_123.i.i = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_30.i.i, !dbg !3990
  store float %_0.i199.i, ptr %_123.i.i, align 4, !dbg !3997, !alias.scope !3999, !noalias !3921
  %exitcond1452.not.i = icmp eq i64 %iter.sroa.0.0.i637.i, %side_left.sroa.5.0.i.i48, !dbg !4575
  br i1 %exitcond1452.not.i, label %bb43.i.i830, label %bb42.i.i, !dbg !4575, !prof !180

bb43.i.i830:                                      ; preds = %bb35.i.us.us.i683, %bb40.i.us.i844, %bb40.i.i
  %.us-phi643.i = phi i64 [ %555, %bb40.i.us.i844 ], [ %586, %bb40.i.i ], [ %524, %bb35.i.us.us.i683 ], !dbg !4585
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %side_left.sroa.5.0.i.i48, i64 noundef %.us-phi643.i, i64 noundef %side_left.sroa.5.0.i.i48, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_31945e5cade31a240e4e50932dbe7a26) #24, !dbg !4585, !noalias !3921
  unreachable, !dbg !4585

bb42.i.i:                                         ; preds = %bb40.i.i
  %_132.not.not.i.i = icmp ugt i64 %_65.1.i27, %_30.i.i, !dbg !4573
  br i1 %_132.not.not.i.i, label %bb44.i.i, label %bb45.i.i534, !dbg !4573, !prof !2709

bb45.i.i534:                                      ; preds = %bb35.i.us.us.us.i529, %bb42.i.us.us.i687, %bb42.i.us.i846, %bb42.i.i
  %.us-phi645.i = phi i64 [ %_30.i.i, %bb42.i.i ], [ %_30.i.us.i835, %bb42.i.us.i846 ], [ %_30.i.us.us.i680, %bb42.i.us.us.i687 ], [ %_30.i.us.us.us.i525, %bb35.i.us.us.us.i529 ]
  %_37.i.le630.i = add nuw nsw i64 %.us-phi645.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi645.i, i64 noundef %_37.i.le630.i, i64 noundef %_65.1.i27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9b3ed103a299f61e5929b9f3196be990) #24, !dbg !4586, !noalias !3921
  unreachable, !dbg !4586

bb44.i.i:                                         ; preds = %bb42.i.i
  %_131.i.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i47, i64 %iter.sroa.0.0.i637.i, !dbg !4002
  %_0.i197.i = load float, ptr %_131.i.i, align 4, !dbg !4009, !alias.scope !4011, !noalias !3921, !noundef !12
  %_139.i.i = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_30.i.i, !dbg !4014
  store float %_0.i197.i, ptr %_139.i.i, align 4, !dbg !4021, !alias.scope !4023, !noalias !3921
  %exitcond1453.not.i = icmp eq i64 %iter.sroa.0.0.i637.i, %empty.sroa.6.0.i.i46, !dbg !4571
  br i1 %exitcond1453.not.i, label %bb47.i.i520, label %bb46.i.i, !dbg !4571, !prof !180

bb47.i.i520:                                      ; preds = %bb35.i.us.us.us.us.i376, %bb44.i.us.us.us.i535, %bb44.i.us.us.i689, %bb44.i.us.i848, %bb44.i.i
  %.us-phi647.i = phi i64 [ %493, %bb44.i.us.us.us.i535 ], [ %586, %bb44.i.i ], [ %555, %bb44.i.us.i848 ], [ %524, %bb44.i.us.us.i689 ], [ %462, %bb35.i.us.us.us.us.i376 ], !dbg !4587
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %empty.sroa.6.0.i.i46, i64 noundef %.us-phi647.i, i64 noundef %empty.sroa.6.0.i.i46, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b346da41cf48210ef0dcce3f5e66934c) #24, !dbg !4587, !noalias !3921
  unreachable, !dbg !4587

bb46.i.i:                                         ; preds = %bb44.i.i
  %_148.not.not.i.i = icmp ugt i64 %_67.1.i31, %_30.i.i, !dbg !4569
  br i1 %_148.not.not.i.i, label %bb48.i.i, label %bb49.i.i228, !dbg !4569, !prof !2709

bb49.i.i228:                                      ; preds = %bb35.i.us.us.us.us.us.i221, %bb46.i.us.us.us.us.i382, %bb46.i.us.us.us.i538, %bb46.i.us.us.i692, %bb46.i.us.i851, %bb46.i.i
  %.us-phi649.i = phi i64 [ %_30.i.us.i835, %bb46.i.us.i851 ], [ %_30.i.us.us.i680, %bb46.i.us.us.i692 ], [ %_30.i.us.us.us.i525, %bb46.i.us.us.us.i538 ], [ %_30.i.us.us.us.us.i373, %bb46.i.us.us.us.us.i382 ], [ %_30.i.i, %bb46.i.i ], [ %_30.i.us.us.us.us.us.i217, %bb35.i.us.us.us.us.us.i221 ]
  %_37.i.le.i229 = add nuw nsw i64 %.us-phi649.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi649.i, i64 noundef %_37.i.le.i229, i64 noundef %_67.1.i31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a44e1bcb659c545f1ffe97492fde2ec9) #24, !dbg !4588, !noalias !3921
  unreachable, !dbg !4588

bb48.i.i:                                         ; preds = %bb46.i.i
  %_147.i.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i45, i64 %iter.sroa.0.0.i637.i, !dbg !4026
  %_0.i195.i = load float, ptr %_147.i.i, align 4, !dbg !4033, !alias.scope !4035, !noalias !3921, !noundef !12
  %_155.i.i = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_30.i.i, !dbg !4038
  store float %_0.i195.i, ptr %_155.i.i, align 4, !dbg !4045, !alias.scope !4047, !noalias !3921
  %_53.i.i = sub i32 %now.i.i, %_59.i34, !dbg !4050
  %_52.i.i = and i32 %_53.i.i, %_58.i33, !dbg !4053
  %_51.i.i = zext i32 %_52.i.i to i64, !dbg !4054
  %_156.not.not.i.i = icmp ugt i64 %_64.1.i25, %_51.i.i, !dbg !4055
  br i1 %_156.not.not.i.i, label %bb50.i.i, label %bb51.i.i72, !dbg !4055, !prof !2709

bb51.i.i72:                                       ; preds = %bb35.i.us.us.us.us.us.us.us.i60, %bb48.i.us.us.us.us.us.i230, %bb48.i.us.us.us.us.i384, %bb48.i.us.us.us.i540, %bb48.i.us.us.i694, %bb48.i.us.i853, %bb48.i.i
  %.us-phi651.i = phi i64 [ %_51.i.us.us.us.i545, %bb48.i.us.us.us.i540 ], [ %_51.i.us.us.us.us.i389, %bb48.i.us.us.us.us.i384 ], [ %_51.i.us.us.us.us.us.i235, %bb48.i.us.us.us.us.us.i230 ], [ %_51.i.i, %bb48.i.i ], [ %_51.i.us.i858, %bb48.i.us.i853 ], [ %_51.i.us.us.i699, %bb48.i.us.us.i694 ], [ %_51.i.us.us.us.us.us.us.us.i70, %bb35.i.us.us.us.us.us.us.us.i60 ]
  %_56.i.le628.i = add nuw nsw i64 %.us-phi651.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi651.i, i64 noundef %_56.i.le628.i, i64 noundef %_64.1.i25, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7) #24, !dbg !4589, !noalias !3921
  unreachable, !dbg !4589

bb50.i.i:                                         ; preds = %bb48.i.i
  %_163.i.i = getelementptr inbounds nuw float, ptr %_64.0.i24, i64 %_51.i.i, !dbg !4060
  %_0.i193.i = load float, ptr %_163.i.i, align 4, !dbg !4064, !alias.scope !4066, !noalias !3921, !noundef !12
  %_164.not.not.i.i = icmp ugt i64 %_66.1.i29, %_51.i.i, !dbg !4578
  br i1 %_164.not.not.i.i, label %bb53.i.i, label %bb54.i.i241, !dbg !4578, !prof !2709

bb54.i.i241:                                      ; preds = %bb50.i.us.i860, %bb50.i.i
  %.us-phi653.i = phi i64 [ %_51.i.i, %bb50.i.i ], [ %_51.i.us.i858, %bb50.i.us.i860 ]
  %_56.i.le.i242 = add nuw nsw i64 %.us-phi653.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi653.i, i64 noundef %_56.i.le.i242, i64 noundef %_66.1.i29, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f198228fd40f4c04a48a745576c5c5ee) #24, !dbg !4590, !noalias !3921
  unreachable, !dbg !4590

bb53.i.i:                                         ; preds = %bb50.i.i
  %_169.i.i = getelementptr inbounds nuw float, ptr %_66.0.i28, i64 %_51.i.i, !dbg !4069
  %_0.i191.i = load float, ptr %_169.i.i, align 4, !dbg !4077, !alias.scope !4079, !noalias !3921, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4082), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4088), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4090), !dbg !4085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4092), !dbg !4085
  %_18.i74.i = load i32, ptr %_31.i17, align 4, !dbg !4094, !alias.scope !4096, !noalias !4097, !noundef !12
  %_17.i.i989 = sub i32 %now.i.i, %_18.i74.i, !dbg !4099
  %_16.i75.i = and i32 %_17.i.i989, %_58.i33, !dbg !4094
  %_15.i76.i = zext i32 %_16.i75.i to i64, !dbg !4094
  %_26.i.i = load i32, ptr %_33.i18, align 4, !dbg !4094, !alias.scope !4101, !noalias !4102, !noundef !12
  %_25.i.i = sub i32 %now.i.i, %_26.i.i, !dbg !4099
  %_24.i.i = and i32 %_25.i.i, %_58.i33, !dbg !4094
  %_23.i79.i = zext i32 %_24.i.i to i64, !dbg !4094
  %_31.i80.i = icmp samesign ugt i64 %_65.1.i27, %_15.i76.i, !dbg !4094
  switch i8 %_0.sroa.0.0.i485.i, label %default.unreachable [
    i8 0, label %bb7.i78.i
    i8 1, label %bb16.i.i
    i8 2, label %bb25.i.i
  ], !dbg !4103

bb7.i78.i:                                        ; preds = %bb53.i.i
  br i1 %_31.i80.i, label %bb8.i81.i, label %panic4.i.i203, !dbg !4568

bb8.i81.i:                                        ; preds = %bb7.i78.i
  %_34.i.i = icmp samesign ugt i64 %_67.1.i31, %_23.i79.i, !dbg !4567
  br i1 %_34.i.i, label %bb10.i82.i, label %panic5.i.i206, !dbg !4567

panic4.i.i203:                                    ; preds = %bb5.i.preheader.us.us.us.us.us.us.us.i202, %bb5.i.preheader.us.us.us.us.us.i362, %bb5.i.preheader.us.us.us.us.i514, %bb5.i.preheader.us.us.us.i670, %bb5.i.preheader.us.us.i824, %bb5.i.preheader.us.i983, %bb7.i78.i
  %.us-phi661.i = phi i64 [ %_15.i76.us.us.us.i557, %bb5.i.preheader.us.us.us.i670 ], [ %_15.i76.us.us.us.us.i401, %bb5.i.preheader.us.us.us.us.i514 ], [ %_15.i76.us.us.us.us.us.i249, %bb5.i.preheader.us.us.us.us.us.i362 ], [ %_15.i76.i, %bb7.i78.i ], [ %_15.i76.us.i870, %bb5.i.preheader.us.i983 ], [ %_15.i76.us.us.i711, %bb5.i.preheader.us.us.i824 ], [ %_15.i76.us.us.us.us.us.us.us.i81, %bb5.i.preheader.us.us.us.us.us.us.us.i202 ], !dbg !4568
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi661.i, i64 noundef range(i64 1, 2305843009213693952) %_65.1.i27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cba26bb3a04aaba20f9c249edc5e133d) #24, !dbg !4568, !noalias !4591
  unreachable, !dbg !4568

panic5.i.i206:                                    ; preds = %bb8.i81.us.us.us.us.us.us.us.i204, %bb8.i81.us.us.us.us.us.i363, %bb8.i81.us.us.us.us.i515, %bb8.i81.us.us.us.i671, %bb8.i81.us.us.i825, %bb8.i81.us.i984, %bb8.i81.i
  %.us-phi662.i = phi i64 [ %_23.i79.us.us.us.i561, %bb8.i81.us.us.us.i671 ], [ %_23.i79.us.us.us.us.i405, %bb8.i81.us.us.us.us.i515 ], [ %_23.i79.us.us.us.us.us.i253, %bb8.i81.us.us.us.us.us.i363 ], [ %_23.i79.i, %bb8.i81.i ], [ %_23.i79.us.i874, %bb8.i81.us.i984 ], [ %_23.i79.us.us.i715, %bb8.i81.us.us.i825 ], [ %_23.i79.us.us.us.us.us.us.us.i85, %bb8.i81.us.us.us.us.us.us.us.i204 ], !dbg !4567
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi662.i, i64 noundef range(i64 1, 2305843009213693952) %_67.1.i31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2ad2d313de59c36d62f4a7d75017a71f) #24, !dbg !4567, !noalias !4591
  unreachable, !dbg !4567

bb10.i82.i:                                       ; preds = %bb8.i81.i
  %587 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.i, !dbg !4568
  %left_own.i.i = load float, ptr %587, align 4, !dbg !4568, !alias.scope !4090, !noalias !4105, !noundef !12
  %588 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.i, !dbg !4567
  %right_own.i.i = load float, ptr %588, align 4, !dbg !4567, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i, !dbg !4110

bb16.i.i:                                         ; preds = %bb53.i.i
  br i1 %_31.i80.i, label %bb17.i.i, label %panic15.i.i195, !dbg !4114

bb17.i.i:                                         ; preds = %bb16.i.i
  %_59.i.i = icmp samesign ugt i64 %_67.1.i31, %_23.i79.i, !dbg !4113
  br i1 %_59.i.i, label %bb19.i.i, label %panic17.i.i198, !dbg !4113

panic15.i.i195:                                   ; preds = %bb14.i63.preheader.us.us.us.us.us.us.us.i194, %bb14.i63.preheader.us.us.us.us.us.i356, %bb14.i63.preheader.us.us.us.us.i508, %bb14.i63.preheader.us.us.us.i664, %bb14.i63.preheader.us.us.i818, %bb14.i63.preheader.us.i977, %bb16.i.i
  %.us-phi659.i = phi i64 [ %_15.i76.us.us.us.i557, %bb14.i63.preheader.us.us.us.i664 ], [ %_15.i76.us.us.us.us.i401, %bb14.i63.preheader.us.us.us.us.i508 ], [ %_15.i76.us.us.us.us.us.i249, %bb14.i63.preheader.us.us.us.us.us.i356 ], [ %_15.i76.i, %bb16.i.i ], [ %_15.i76.us.i870, %bb14.i63.preheader.us.i977 ], [ %_15.i76.us.us.i711, %bb14.i63.preheader.us.us.i818 ], [ %_15.i76.us.us.us.us.us.us.us.i81, %bb14.i63.preheader.us.us.us.us.us.us.us.i194 ], !dbg !4114
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi659.i, i64 noundef range(i64 1, 2305843009213693952) %_65.1.i27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2e01e5f964095ee8e5aa091c7d92b88) #24, !dbg !4114, !noalias !4591
  unreachable, !dbg !4114

panic17.i.i198:                                   ; preds = %bb17.i.us.us.us.us.us.us.us.i196, %bb17.i.us.us.us.us.us.i357, %bb17.i.us.us.us.us.i509, %bb17.i.us.us.us.i665, %bb17.i.us.us.i819, %bb17.i.us.i978, %bb17.i.i
  %.us-phi660.i = phi i64 [ %_23.i79.us.us.us.i561, %bb17.i.us.us.us.i665 ], [ %_23.i79.us.us.us.us.i405, %bb17.i.us.us.us.us.i509 ], [ %_23.i79.us.us.us.us.us.i253, %bb17.i.us.us.us.us.us.i357 ], [ %_23.i79.i, %bb17.i.i ], [ %_23.i79.us.i874, %bb17.i.us.i978 ], [ %_23.i79.us.us.i715, %bb17.i.us.us.i819 ], [ %_23.i79.us.us.us.us.us.us.us.i85, %bb17.i.us.us.us.us.us.us.us.i196 ], !dbg !4113
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi660.i, i64 noundef range(i64 1, 2305843009213693952) %_67.1.i31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_48dca199019b49040caac979cf1ddc5a) #24, !dbg !4113, !noalias !4591
  unreachable, !dbg !4113

bb19.i.i:                                         ; preds = %bb17.i.i
  %589 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.i, !dbg !4114
  %left_own16.i.i = load float, ptr %589, align 4, !dbg !4114, !alias.scope !4090, !noalias !4105, !noundef !12
  %590 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.i, !dbg !4113
  %right_own18.i.i = load float, ptr %590, align 4, !dbg !4113, !alias.scope !4092, !noalias !4107, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i, !dbg !4110

bb25.i.i:                                         ; preds = %bb53.i.i
  br i1 %_31.i80.i, label %bb27.i60.i, label %panic28.i.i88, !dbg !4104

panic28.i.i88:                                    ; preds = %bb23.i.preheader.us.us.us.us.us.us.us.i87, %bb23.i.preheader.us.us.us.us.us.i255, %bb23.i.preheader.us.us.us.us.i407, %bb23.i.preheader.us.us.us.i563, %bb23.i.preheader.us.us.i717, %bb23.i.preheader.us.i876, %bb25.i.i
  %.us-phi655.i = phi i64 [ %_15.i76.us.us.us.i557, %bb23.i.preheader.us.us.us.i563 ], [ %_15.i76.us.us.us.us.i401, %bb23.i.preheader.us.us.us.us.i407 ], [ %_15.i76.us.us.us.us.us.i249, %bb23.i.preheader.us.us.us.us.us.i255 ], [ %_15.i76.i, %bb25.i.i ], [ %_15.i76.us.i870, %bb23.i.preheader.us.i876 ], [ %_15.i76.us.us.i711, %bb23.i.preheader.us.us.i717 ], [ %_15.i76.us.us.us.us.us.us.us.i81, %bb23.i.preheader.us.us.us.us.us.us.us.i87 ], !dbg !4104
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi655.i, i64 noundef range(i64 1, 2305843009213693952) %_65.1.i27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4b7b2eb7fa1b19ad0451b09b71313122) #24, !dbg !4104, !noalias !4591
  unreachable, !dbg !4104

bb27.i60.i:                                       ; preds = %bb25.i.i
  %591 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_15.i76.i, !dbg !4104
  %_79.i.i = load float, ptr %591, align 4, !dbg !4104, !alias.scope !4090, !noalias !4105, !noundef !12
  %_85.i.i = icmp samesign ugt i64 %_67.1.i31, %_15.i76.i, !dbg !4106
  br i1 %_85.i.i, label %bb29.i61.i, label %panic30.i.i92, !dbg !4106

panic30.i.i92:                                    ; preds = %bb27.i60.us.us.us.us.us.us.us.i89, %bb27.i60.us.us.us.us.us.i256, %bb27.i60.us.us.us.us.i408, %bb27.i60.us.us.us.i564, %bb27.i60.us.us.i718, %bb27.i60.us.i877, %bb27.i60.i
  %.us-phi656.i = phi i64 [ %_15.i76.us.us.us.i557, %bb27.i60.us.us.us.i564 ], [ %_15.i76.us.us.us.us.i401, %bb27.i60.us.us.us.us.i408 ], [ %_15.i76.us.us.us.us.us.i249, %bb27.i60.us.us.us.us.us.i256 ], [ %_15.i76.i, %bb27.i60.i ], [ %_15.i76.us.i870, %bb27.i60.us.i877 ], [ %_15.i76.us.us.i711, %bb27.i60.us.us.i718 ], [ %_15.i76.us.us.us.us.us.us.us.i81, %bb27.i60.us.us.us.us.us.us.us.i89 ], !dbg !4106
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi656.i, i64 noundef range(i64 1, 2305843009213693952) %_67.1.i31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60256fc2b51ee5bb69caa4304418caff) #24, !dbg !4106, !noalias !4591
  unreachable, !dbg !4106

bb29.i61.i:                                       ; preds = %bb27.i60.i
  %592 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_15.i76.i, !dbg !4106
  %_83.i.i = load float, ptr %592, align 4, !dbg !4106, !alias.scope !4092, !noalias !4107, !noundef !12
  %_87.i.i = icmp samesign ugt i64 %_67.1.i31, %_23.i79.i, !dbg !4108
  br i1 %_87.i.i, label %bb31.i.i, label %panic32.i.i96, !dbg !4108

panic32.i.i96:                                    ; preds = %bb29.i61.us.us.us.us.us.us.us.i93, %bb29.i61.us.us.us.us.us.i259, %bb29.i61.us.us.us.us.i411, %bb29.i61.us.us.us.i567, %bb29.i61.us.us.i721, %bb29.i61.us.i880, %bb29.i61.i
  %.us-phi657.i = phi i64 [ %_23.i79.us.us.us.i561, %bb29.i61.us.us.us.i567 ], [ %_23.i79.us.us.us.us.i405, %bb29.i61.us.us.us.us.i411 ], [ %_23.i79.us.us.us.us.us.i253, %bb29.i61.us.us.us.us.us.i259 ], [ %_23.i79.i, %bb29.i61.i ], [ %_23.i79.us.i874, %bb29.i61.us.i880 ], [ %_23.i79.us.us.i715, %bb29.i61.us.us.i721 ], [ %_23.i79.us.us.us.us.us.us.us.i85, %bb29.i61.us.us.us.us.us.us.us.i93 ], !dbg !4108
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi657.i, i64 noundef range(i64 1, 2305843009213693952) %_67.1.i31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_03bcac171bcf0d517141d8cd3ac9ded0) #24, !dbg !4108, !noalias !4591
  unreachable, !dbg !4108

bb31.i.i:                                         ; preds = %bb29.i61.i
  %_89.i.i = icmp samesign ugt i64 %_65.1.i27, %_23.i79.i, !dbg !4109
  br i1 %_89.i.i, label %bb33.i62.i, label %panic34.i.i99, !dbg !4109

panic34.i.i99:                                    ; preds = %bb31.i.us.us.us.us.us.us.us.i97, %bb31.i.us.us.us.us.i414, %bb31.i.us.us.us.i570, %bb31.i.us.us.i724, %bb31.i.us.i883, %bb31.i.i
  %.us-phi658.i = phi i64 [ %_23.i79.us.us.i715, %bb31.i.us.us.i724 ], [ %_23.i79.us.us.us.i561, %bb31.i.us.us.us.i570 ], [ %_23.i79.us.us.us.us.i405, %bb31.i.us.us.us.us.i414 ], [ %_23.i79.i, %bb31.i.i ], [ %_23.i79.us.i874, %bb31.i.us.i883 ], [ %_23.i79.us.us.us.us.us.us.us.i85, %bb31.i.us.us.us.us.us.us.us.i97 ], !dbg !4109
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi658.i, i64 noundef range(i64 1, 2305843009213693952) %_65.1.i27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ed84bd2438b7b7e3dfb2147bac09e923) #24, !dbg !4109, !noalias !4591
  unreachable, !dbg !4109

bb33.i62.i:                                       ; preds = %bb31.i.i
  %593 = getelementptr inbounds nuw float, ptr %_67.0.i30, i64 %_23.i79.i, !dbg !4108
  %_86.i.i = load float, ptr %593, align 4, !dbg !4108, !alias.scope !4092, !noalias !4107, !noundef !12
  %594 = getelementptr inbounds nuw float, ptr %_65.0.i26, i64 %_23.i79.i, !dbg !4109
  %_88.i.i = load float, ptr %594, align 4, !dbg !4109, !alias.scope !4090, !noalias !4105, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i, !dbg !4110

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i: ; preds = %bb33.i62.i, %bb19.i.i, %bb10.i82.i
  %taps.i.sroa.1011457.0.i = phi float [ %right_own.i.i, %bb10.i82.i ], [ %left_own16.i.i, %bb19.i.i ], [ %_88.i.i, %bb33.i62.i ], !dbg !4094
  %taps.i.sroa.681456.0.i = phi float [ %right_own.i.i, %bb10.i82.i ], [ %right_own18.i.i, %bb19.i.i ], [ %_86.i.i, %bb33.i62.i ], !dbg !4094
  %taps.i.sroa.351455.0.i = phi float [ %left_own.i.i, %bb10.i82.i ], [ %right_own18.i.i, %bb19.i.i ], [ %_83.i.i, %bb33.i62.i ], !dbg !4094
  %taps.i.sroa.0.0.i = phi float [ %left_own.i.i, %bb10.i82.i ], [ %left_own16.i.i, %bb19.i.i ], [ %_79.i.i, %bb33.i62.i ], !dbg !4094
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4115), !dbg !4118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4119), !dbg !4118
  %threshold.i30.i.i = load float, ptr %_10.i13, align 4, !dbg !4121, !alias.scope !4126, !noalias !4127, !noundef !12
  %ratio.i31.i.i = load float, ptr %378, align 4, !dbg !4128, !alias.scope !4126, !noalias !4127, !noundef !12
  %range.i32.i.i = load float, ptr %379, align 4, !dbg !4130, !alias.scope !4126, !noalias !4127, !noundef !12
  %hysteresis.i33.i.i = load float, ptr %380, align 4, !dbg !4132, !alias.scope !4126, !noalias !4127, !noundef !12
  %595 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.0.i), !dbg !4134
  %596 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.351455.0.i), !dbg !4137
  %_37.i36.i.i = load float, ptr %381, align 4, !dbg !4140, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i127.i = fcmp ule float %_37.i36.i.i, 0.000000e+00, !dbg !4145
  %_3.i.i423.i = fcmp ule float %595, %596, !dbg !4147
  %_6.i.i425.i = bitcast float %595 to i32, !dbg !4150
  %_8.i.i427.i = bitcast float %596 to i32, !dbg !4153
  %_4.i.i430.i = select i1 %_3.i.i423.i, i32 %_8.i.i427.i, i32 %_6.i.i425.i, !dbg !4155
  %_4.i321.i = select i1 %_3.i127.i, i32 %_6.i.i425.i, i32 %_4.i.i430.i, !dbg !4156
  %_41.i40.i.i = load float, ptr %382, align 4, !dbg !4158, !alias.scope !4143, !noalias !4144, !noundef !12
  %_3.i125.i = fcmp ule float %_41.i40.i.i, 0.000000e+00, !dbg !4159
  %_0.i167.i = fmul float %595, 5.000000e-01, !dbg !4161
  %_0.i166.i = fmul float %596, 5.000000e-01, !dbg !4163
  %_0.i142.i = fadd float %_0.i166.i, %_0.i167.i, !dbg !4165
  %_6.i309.i = bitcast float %_0.i142.i to i32, !dbg !4167
  %_4.i314.i = select i1 %_3.i125.i, i32 %_4.i321.i, i32 %_6.i309.i, !dbg !4170
  %_0.i315.i = bitcast i32 %_4.i314.i to float, !dbg !4171
  %_3.i.i415.i = fcmp ule float %_0.i315.i, 0x3E45798EE0000000, !dbg !4173
  %_4.i.i421.i = select i1 %_3.i.i415.i, i32 841731191, i32 %_4.i314.i, !dbg !4176
  %_0.i.i422.i = bitcast i32 %_4.i.i421.i to float, !dbg !4178
  %_3.i.i.i990 = fcmp ule float %_0.i.i422.i, 0x3810000000000000, !dbg !4180
  %_4.i.i.i = select i1 %_3.i.i.i990, i32 8388608, i32 %_4.i.i421.i, !dbg !4186
  %_5.i202.i = and i32 %_4.i.i.i, 8388607, !dbg !4188
  %_4.i203.i = or disjoint i32 %_5.i202.i, 1065353216, !dbg !4188
  %significand.i.i = bitcast i32 %_4.i203.i to float, !dbg !4190
  %_0.i168.i = fadd float %significand.i.i, -1.000000e+00, !dbg !4192
  %_0.i148.i = fmul float %_0.i168.i, 0x3F9B17A960000000, !dbg !4194
  %597 = fsub float 0x3FBF9A8440000000, %_0.i148.i, !dbg !4196
  %_0.i148.1.i = fmul float %_0.i168.i, %597, !dbg !4194
  %_0.i134.1.i = fadd float %_0.i148.1.i, 0xBFD1E3F400000000, !dbg !4196
  %_0.i148.2.i = fmul float %_0.i168.i, %_0.i134.1.i, !dbg !4194
  %_0.i134.2.i = fadd float %_0.i148.2.i, 0x3FDD544F20000000, !dbg !4196
  %_0.i148.3.i = fmul float %_0.i168.i, %_0.i134.2.i, !dbg !4194
  %_0.i134.3.i = fadd float %_0.i148.3.i, 0xBFE6FC2A60000000, !dbg !4196
  %_0.i148.4.i = fmul float %_0.i168.i, %_0.i134.3.i, !dbg !4194
  %_0.i134.4.i = fadd float %_0.i148.4.i, 0x3FF714B2A0000000, !dbg !4196
  %_9.i.i = lshr i32 %_4.i.i.i, 23, !dbg !4198
  %_8.i204.i = or disjoint i32 %_9.i.i, 1258291200, !dbg !4198
  %_7.i.i = bitcast i32 %_8.i204.i to float, !dbg !4199
  %exponent.i.i = fadd float %_7.i.i, 0xC160000FE0000000, !dbg !4201
  %_0.i147.i = fmul float %_0.i168.i, %_0.i134.4.i, !dbg !4202
  %_0.i133.i = fadd float %exponent.i.i, %_0.i147.i, !dbg !4204
  %_0.i165.i = fmul float %_0.i133.i, 0x4018151820000000, !dbg !4206
  %_3.i.i472.inv.i = fcmp olt float %_0.i165.i, 2.400000e+01, !dbg !4208
  %_0.i.i479.i = select i1 %_3.i.i472.inv.i, float %_0.i165.i, float 2.400000e+01, !dbg !4208
  %_3.i.i407.inv.i = fcmp ogt float %_0.i.i479.i, -1.600000e+02, !dbg !4211
  %_0.i.i414.i = select i1 %_3.i.i407.inv.i, float %_0.i.i479.i, float -1.600000e+02, !dbg !4211
  %_55.i57.i.i = load float, ptr %383, align 4, !dbg !4214, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i123.i = fcmp ule float %_55.i57.i.i, 0.000000e+00, !dbg !4216
  %_3.i99.i = fcmp oge float %_0.i.i414.i, %threshold.i30.i.i, !dbg !4218
  %_0.i181.i = fsub float %threshold.i30.i.i, %hysteresis.i33.i.i, !dbg !4221
  %_3.i97.i = fcmp oge float %_0.i.i414.i, %_0.i181.i, !dbg !4224
  %..i98.i = sext i1 %_3.i97.i to i32, !dbg !4226
  %_0.i335.i = sext i1 %_3.i99.i to i32, !dbg !4228
  %_0.i328.i = select i1 %_3.i123.i, i32 %_0.i335.i, i32 %..i98.i, !dbg !4228
  %_0.i339.i = xor i32 %..i98.i, -1, !dbg !4233
  %_67.i67.i.i = load float, ptr %384, align 4, !dbg !4236, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i121.i = fcmp ogt float %_67.i67.i.i, 0.000000e+00, !dbg !4237
  %_0.i334.i = select i1 %_3.i121.i, i32 %_0.i339.i, i32 0, !dbg !4239
  %_0.i333.i = select i1 %_3.i123.i, i32 0, i32 %_0.i334.i, !dbg !4241
  %_0.i327.i = or i32 %_0.i333.i, %_0.i328.i, !dbg !4243
  %_5.i304.i = and i32 %_0.i327.i, 1065353216, !dbg !4246
  %_0.i308.i = bitcast i32 %_5.i304.i to float, !dbg !4248
  %_71.i73.i493494.i = load float, ptr %385, align 4, !dbg !4250, !alias.scope !4143, !noalias !4144, !noundef !12
  %_0.i180.i = fadd float %_67.i67.i.i, -1.000000e+00, !dbg !4252
  %598 = trunc nsw i32 %_0.i333.i to i1, !dbg !4254
  %_4.i302.v.i = select i1 %598, float %_0.i180.i, float %_67.i67.i.i, !dbg !4254
  %599 = trunc nsw i32 %_0.i328.i to i1, !dbg !4256
  %_0.i296.i = select i1 %599, float %_71.i73.i493494.i, float %_4.i302.v.i, !dbg !4256
  store float %_0.i296.i, ptr %384, align 4, !dbg !4258, !alias.scope !4126, !noalias !4127
  store i32 %_5.i304.i, ptr %383, align 4, !dbg !4259, !alias.scope !4126, !noalias !4127
  %_0.i179.i = fadd float %ratio.i31.i.i, -1.000000e+00, !dbg !4260
  %_0.i178.i = fsub float %_0.i.i414.i, %threshold.i30.i.i, !dbg !4262
  %_0.i164.i = fmul float %_0.i179.i, %_0.i178.i, !dbg !4264
  %600 = fneg float %range.i32.i.i, !dbg !4266
  %_3.i.i398.inv.i = fcmp ogt float %_0.i164.i, %600, !dbg !4268
  %_4.i.i405.v.i = select i1 %_3.i.i398.inv.i, float %_0.i164.i, float %600, !dbg !4268
  %_3.i.i464.i = fcmp olt float %_4.i.i405.v.i, 0.000000e+00, !dbg !4271
  %601 = fcmp ule float %_0.i308.i, 0.000000e+00, !dbg !4274
  %602 = select i1 %601, i1 %_3.i.i464.i, i1 false, !dbg !4277
  %_0.i289.i = select i1 %602, float %_4.i.i405.v.i, float 0.000000e+00, !dbg !4277
  %_86.i86.i.i = load float, ptr %386, align 4, !dbg !4278, !alias.scope !4126, !noalias !4127, !noundef !12
  %_3.i117.i = fcmp ule float %_0.i289.i, %_86.i86.i.i, !dbg !4280
  %_87.i88.i496.i = load i32, ptr %366, align 4, !dbg !4282, !alias.scope !4143, !noalias !4144, !noundef !12
  %_88.i89.i497.i = load i32, ptr %387, align 4, !dbg !4283, !alias.scope !4143, !noalias !4144, !noundef !12
  %_4.i282.i = select i1 %_3.i117.i, i32 %_88.i89.i497.i, i32 %_87.i88.i496.i, !dbg !4284
  %_0.i283.i = bitcast i32 %_4.i282.i to float, !dbg !4286
  %_0.i177.i = fsub float %_0.i289.i, %_86.i86.i.i, !dbg !4288
  %_4.i145.i = fmul float %_0.i177.i, %_0.i283.i, !dbg !4291
  %_0.i146.i = fadd float %_86.i86.i.i, %_4.i145.i, !dbg !4291
  %603 = tail call noundef float @llvm.fabs.f32(float %_0.i146.i), !dbg !4293
  %604 = fcmp uge float %603, 0x3BC79CA100000000, !dbg !4297
  %_0.i218.i = select i1 %604, float %_0.i146.i, float 0.000000e+00, !dbg !4299
  store float %_0.i218.i, ptr %386, align 4, !dbg !4300, !alias.scope !4126, !noalias !4127
  %_0.i163.i = fmul float %_0.i218.i, 0x3FC542A5A0000000, !dbg !4302
  %_3.i.i349.inv.i = fcmp ogt float %_0.i163.i, -1.260000e+02, !dbg !4306
  %_0.i.i356.i = select i1 %_3.i.i349.inv.i, float %_0.i163.i, float -1.260000e+02, !dbg !4306
  %_3.i.i432.inv.i = fcmp olt float %_0.i.i356.i, 1.270000e+02, !dbg !4310
  %_0.i.i439.i = select i1 %_3.i.i432.inv.i, float %_0.i.i356.i, float 1.270000e+02, !dbg !4310
  %605 = tail call noundef float @llvm.floor.f32(float %_0.i.i439.i), !dbg !4313
  %_0.i170.i = fsub float %_0.i.i439.i, %605, !dbg !4317
  %_98.i102.i.i = load float, ptr %388, align 4, !dbg !4319, !alias.scope !4143, !noalias !4144, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4321), !dbg !4324
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4326), !dbg !4324
  %threshold.i.i.i = load float, ptr %data.i.i.i14, align 4, !dbg !4328, !alias.scope !4330, !noalias !4331, !noundef !12
  %ratio.i.i.i = load float, ptr %389, align 4, !dbg !4332, !alias.scope !4330, !noalias !4331, !noundef !12
  %range.i.i.i = load float, ptr %390, align 4, !dbg !4333, !alias.scope !4330, !noalias !4331, !noundef !12
  %hysteresis.i.i.i = load float, ptr %391, align 4, !dbg !4334, !alias.scope !4330, !noalias !4331, !noundef !12
  %606 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.681456.0.i), !dbg !4335
  %607 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.1011457.0.i), !dbg !4337
  %_37.i.i.i = load float, ptr %392, align 4, !dbg !4339, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i113.i = fcmp ule float %_37.i.i.i, 0.000000e+00, !dbg !4342
  %_3.i.i389.i = fcmp ule float %606, %607, !dbg !4344
  %_6.i.i391.i = bitcast float %606 to i32, !dbg !4347
  %_8.i.i393.i = bitcast float %607 to i32, !dbg !4350
  %_4.i.i396.i = select i1 %_3.i.i389.i, i32 %_8.i.i393.i, i32 %_6.i.i391.i, !dbg !4352
  %_4.i268.i = select i1 %_3.i113.i, i32 %_6.i.i391.i, i32 %_4.i.i396.i, !dbg !4353
  %_41.i.i.i = load float, ptr %393, align 4, !dbg !4355, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i111.i = fcmp ule float %_41.i.i.i, 0.000000e+00, !dbg !4356
  %_0.i161.i = fmul float %606, 5.000000e-01, !dbg !4358
  %_0.i160.i = fmul float %607, 5.000000e-01, !dbg !4360
  %_0.i141.i = fadd float %_0.i160.i, %_0.i161.i, !dbg !4362
  %_6.i256.i = bitcast float %_0.i141.i to i32, !dbg !4364
  %_4.i261.i = select i1 %_3.i111.i, i32 %_4.i268.i, i32 %_6.i256.i, !dbg !4367
  %_0.i262.i = bitcast i32 %_4.i261.i to float, !dbg !4368
  %_3.i.i381.i = fcmp ule float %_0.i262.i, 0x3E45798EE0000000, !dbg !4370
  %_4.i.i387.i = select i1 %_3.i.i381.i, i32 841731191, i32 %_4.i261.i, !dbg !4373
  %_0.i.i388.i = bitcast i32 %_4.i.i387.i to float, !dbg !4375
  %_3.i.i341.i = fcmp ule float %_0.i.i388.i, 0x3810000000000000, !dbg !4377
  %_4.i.i347.i = select i1 %_3.i.i341.i, i32 8388608, i32 %_4.i.i387.i, !dbg !4382
  %_5.i206.i = and i32 %_4.i.i347.i, 8388607, !dbg !4384
  %_4.i207.i = or disjoint i32 %_5.i206.i, 1065353216, !dbg !4384
  %significand.i208.i = bitcast i32 %_4.i207.i to float, !dbg !4386
  %_0.i169.i = fadd float %significand.i208.i, -1.000000e+00, !dbg !4388
  %_0.i150.i = fmul float %_0.i169.i, 0x3F9B17A960000000, !dbg !4390
  %608 = fsub float 0x3FBF9A8440000000, %_0.i150.i, !dbg !4392
  %_0.i150.1.i = fmul float %_0.i169.i, %608, !dbg !4390
  %_0.i136.1.i = fadd float %_0.i150.1.i, 0xBFD1E3F400000000, !dbg !4392
  %_0.i150.2.i = fmul float %_0.i169.i, %_0.i136.1.i, !dbg !4390
  %_0.i136.2.i = fadd float %_0.i150.2.i, 0x3FDD544F20000000, !dbg !4392
  %_0.i150.3.i = fmul float %_0.i169.i, %_0.i136.2.i, !dbg !4390
  %_0.i136.3.i = fadd float %_0.i150.3.i, 0xBFE6FC2A60000000, !dbg !4392
  %_0.i150.4.i = fmul float %_0.i169.i, %_0.i136.3.i, !dbg !4390
  %_0.i136.4.i = fadd float %_0.i150.4.i, 0x3FF714B2A0000000, !dbg !4392
  %_9.i209.i = lshr i32 %_4.i.i347.i, 23, !dbg !4394
  %_8.i210.i = or disjoint i32 %_9.i209.i, 1258291200, !dbg !4394
  %_7.i211.i = bitcast i32 %_8.i210.i to float, !dbg !4395
  %exponent.i212.i = fadd float %_7.i211.i, 0xC160000FE0000000, !dbg !4397
  %_0.i149.i = fmul float %_0.i169.i, %_0.i136.4.i, !dbg !4398
  %_0.i135.i = fadd float %exponent.i212.i, %_0.i149.i, !dbg !4400
  %_0.i159.i = fmul float %_0.i135.i, 0x4018151820000000, !dbg !4402
  %_3.i.i456.inv.i = fcmp olt float %_0.i159.i, 2.400000e+01, !dbg !4404
  %_0.i.i463.i = select i1 %_3.i.i456.inv.i, float %_0.i159.i, float 2.400000e+01, !dbg !4404
  %_3.i.i373.inv.i = fcmp ogt float %_0.i.i463.i, -1.600000e+02, !dbg !4407
  %_0.i.i380.i = select i1 %_3.i.i373.inv.i, float %_0.i.i463.i, float -1.600000e+02, !dbg !4407
  %_55.i.i.i = load float, ptr %394, align 4, !dbg !4410, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i109.i = fcmp ule float %_55.i.i.i, 0.000000e+00, !dbg !4411
  %_3.i95.i = fcmp oge float %_0.i.i380.i, %threshold.i.i.i, !dbg !4413
  %_0.i176.i = fsub float %threshold.i.i.i, %hysteresis.i.i.i, !dbg !4415
  %_3.i93.i = fcmp oge float %_0.i.i380.i, %_0.i176.i, !dbg !4417
  %..i94.i = sext i1 %_3.i93.i to i32, !dbg !4419
  %_0.i331.i = sext i1 %_3.i95.i to i32, !dbg !4421
  %_0.i325.i = select i1 %_3.i109.i, i32 %_0.i331.i, i32 %..i94.i, !dbg !4421
  %_0.i337.i = xor i32 %..i94.i, -1, !dbg !4423
  %_67.i.i.i = load float, ptr %395, align 4, !dbg !4425, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i107.i = fcmp ogt float %_67.i.i.i, 0.000000e+00, !dbg !4426
  %_0.i330.i = select i1 %_3.i107.i, i32 %_0.i337.i, i32 0, !dbg !4428
  %_0.i329.i = select i1 %_3.i109.i, i32 0, i32 %_0.i330.i, !dbg !4430
  %_0.i324.i = or i32 %_0.i329.i, %_0.i325.i, !dbg !4432
  %_5.i251.i = and i32 %_0.i324.i, 1065353216, !dbg !4434
  %_0.i255.i = bitcast i32 %_5.i251.i to float, !dbg !4436
  %_71.i.i510511.i = load float, ptr %396, align 4, !dbg !4438, !alias.scope !4340, !noalias !4341, !noundef !12
  %_0.i175.i = fadd float %_67.i.i.i, -1.000000e+00, !dbg !4439
  %609 = trunc nsw i32 %_0.i329.i to i1, !dbg !4441
  %_4.i249.v.i = select i1 %609, float %_0.i175.i, float %_67.i.i.i, !dbg !4441
  %610 = trunc nsw i32 %_0.i325.i to i1, !dbg !4443
  %_0.i243.i = select i1 %610, float %_71.i.i510511.i, float %_4.i249.v.i, !dbg !4443
  store float %_0.i243.i, ptr %395, align 4, !dbg !4445, !alias.scope !4330, !noalias !4331
  store i32 %_5.i251.i, ptr %394, align 4, !dbg !4446, !alias.scope !4330, !noalias !4331
  %_0.i174.i = fadd float %ratio.i.i.i, -1.000000e+00, !dbg !4447
  %_0.i173.i = fsub float %_0.i.i380.i, %threshold.i.i.i, !dbg !4449
  %_0.i158.i = fmul float %_0.i174.i, %_0.i173.i, !dbg !4451
  %611 = fneg float %range.i.i.i, !dbg !4453
  %_3.i.i365.inv.i = fcmp ogt float %_0.i158.i, %611, !dbg !4455
  %_4.i.i371.v.i = select i1 %_3.i.i365.inv.i, float %_0.i158.i, float %611, !dbg !4455
  %_3.i.i448.i = fcmp olt float %_4.i.i371.v.i, 0.000000e+00, !dbg !4458
  %612 = fcmp ule float %_0.i255.i, 0.000000e+00, !dbg !4461
  %613 = select i1 %612, i1 %_3.i.i448.i, i1 false, !dbg !4463
  %_0.i236.i = select i1 %613, float %_4.i.i371.v.i, float 0.000000e+00, !dbg !4463
  %_86.i.i.i = load float, ptr %397, align 4, !dbg !4464, !alias.scope !4330, !noalias !4331, !noundef !12
  %_3.i103.i = fcmp ule float %_0.i236.i, %_86.i.i.i, !dbg !4465
  %_87.i.i513.i = load i32, ptr %_38.i23, align 4, !dbg !4467, !alias.scope !4340, !noalias !4341, !noundef !12
  %_88.i.i514.i = load i32, ptr %398, align 4, !dbg !4468, !alias.scope !4340, !noalias !4341, !noundef !12
  %_4.i229.i = select i1 %_3.i103.i, i32 %_88.i.i514.i, i32 %_87.i.i513.i, !dbg !4469
  %_0.i230.i = bitcast i32 %_4.i229.i to float, !dbg !4471
  %_0.i172.i = fsub float %_0.i236.i, %_86.i.i.i, !dbg !4473
  %_4.i143.i = fmul float %_0.i172.i, %_0.i230.i, !dbg !4475
  %_0.i144.i = fadd float %_86.i.i.i, %_4.i143.i, !dbg !4475
  %614 = tail call noundef float @llvm.fabs.f32(float %_0.i144.i), !dbg !4477
  %615 = fcmp uge float %614, 0x3BC79CA100000000, !dbg !4480
  %_0.i214.i = select i1 %615, float %_0.i144.i, float 0.000000e+00, !dbg !4482
  store float %_0.i214.i, ptr %397, align 4, !dbg !4483, !alias.scope !4330, !noalias !4331
  %_0.i157.i = fmul float %_0.i214.i, 0x3FC542A5A0000000, !dbg !4484
  %_3.i.i357.inv.i = fcmp ogt float %_0.i157.i, -1.260000e+02, !dbg !4487
  %_0.i.i364.i = select i1 %_3.i.i357.inv.i, float %_0.i157.i, float -1.260000e+02, !dbg !4487
  %_3.i.i440.inv.i = fcmp olt float %_0.i.i364.i, 1.270000e+02, !dbg !4491
  %_0.i.i447.i = select i1 %_3.i.i440.inv.i, float %_0.i.i364.i, float 1.270000e+02, !dbg !4491
  %616 = tail call noundef float @llvm.floor.f32(float %_0.i.i447.i), !dbg !4494
  %_0.i171.i = fsub float %_0.i.i447.i, %616, !dbg !4498
  %_0.i155.i = fmul float %_0.i171.i, 0x3F5E974FA0000000, !dbg !4500
  %_0.i140.i = fadd float %_0.i155.i, 0x3F82778560000000, !dbg !4502
  %_0.i155.1.i = fmul float %_0.i171.i, %_0.i140.i, !dbg !4500
  %_0.i140.1.i = fadd float %_0.i155.1.i, 0x3FAC91CE60000000, !dbg !4502
  %_0.i155.2.i = fmul float %_0.i171.i, %_0.i140.1.i, !dbg !4500
  %_0.i140.2.i = fadd float %_0.i155.2.i, 0x3FCEBDB560000000, !dbg !4502
  %_0.i155.3.i = fmul float %_0.i171.i, %_0.i140.2.i, !dbg !4500
  %_0.i140.3.i = fadd float %_0.i155.3.i, 0x3FE62E4BA0000000, !dbg !4502
  %_0.i153.i = fmul float %_0.i170.i, 0x3F5E974FA0000000, !dbg !4504
  %_0.i138.i = fadd float %_0.i153.i, 0x3F82778560000000, !dbg !4506
  %_0.i153.1.i = fmul float %_0.i170.i, %_0.i138.i, !dbg !4504
  %_0.i138.1.i = fadd float %_0.i153.1.i, 0x3FAC91CE60000000, !dbg !4506
  %_0.i153.2.i = fmul float %_0.i170.i, %_0.i138.1.i, !dbg !4504
  %_0.i138.2.i = fadd float %_0.i153.2.i, 0x3FCEBDB560000000, !dbg !4506
  %_0.i153.3.i = fmul float %_0.i170.i, %_0.i138.2.i, !dbg !4504
  %_0.i138.3.i = fadd float %_0.i153.3.i, 0x3FE62E4BA0000000, !dbg !4506
  %_0.i152.i = fmul float %_0.i170.i, %_0.i138.3.i, !dbg !4508
  %_0.i137.i = fadd float %_0.i152.i, 1.000000e+00, !dbg !4510
  %biased.i.i = fadd float %605, 0x4160000FE0000000, !dbg !4512
  %_4.i83.i = bitcast float %biased.i.i to i32, !dbg !4514
  %_3.i84.i = shl i32 %_4.i83.i, 23, !dbg !4516
  %_0.i85.i = bitcast i32 %_3.i84.i to float, !dbg !4517
  %_0.i151.i = fmul float %_0.i137.i, %_0.i85.i, !dbg !4519
  %_3.i91.i = fcmp une float %_0.i218.i, 0.000000e+00, !dbg !4521
  %_3.i115.i = fcmp ule float %_98.i102.i.i, 0.000000e+00, !dbg !4523
  %_0.i326501.not.i = and i1 %_3.i115.i, %_3.i91.i, !dbg !4525
  %_0.i162.i = fmul float %_0.i193.i, %_0.i151.i, !dbg !4525
  %_4.i275.v.i = select i1 %_0.i326501.not.i, float %_0.i162.i, float %_0.i193.i, !dbg !4528
  %_0.i.i = fmul float %_0.i171.i, %_0.i140.3.i, !dbg !4530
  %_0.i139.i = fadd float %_0.i.i, 1.000000e+00, !dbg !4532
  %biased.i86.i = fadd float %616, 0x4160000FE0000000, !dbg !4534
  %_4.i87.i = bitcast float %biased.i86.i to i32, !dbg !4536
  %_3.i88.i = shl i32 %_4.i87.i, 23, !dbg !4538
  %_0.i89.i = bitcast i32 %_3.i88.i to float, !dbg !4539
  %_0.i154.i = fmul float %_0.i139.i, %_0.i89.i, !dbg !4541
  %_3.i90.i = fcmp une float %_0.i214.i, 0.000000e+00, !dbg !4543
  %_98.i.i.i = load float, ptr %399, align 4, !dbg !4545, !alias.scope !4340, !noalias !4341, !noundef !12
  %_3.i101.i = fcmp ule float %_98.i.i.i, 0.000000e+00, !dbg !4546
  %_0.i323518.not.i = and i1 %_3.i101.i, %_3.i90.i, !dbg !4548
  %_0.i156.i = fmul float %_0.i191.i, %_0.i154.i, !dbg !4548
  %_4.i222.v.i = select i1 %_0.i323518.not.i, float %_0.i156.i, float %_0.i191.i, !dbg !4550
  store float %_4.i275.v.i, ptr %_97.i.i, align 4, !dbg !4552, !alias.scope !4555, !noalias !3967
  store float %_4.i222.v.i, ptr %_115.i.i, align 4, !dbg !4558, !alias.scope !4560, !noalias !3989
  %exitcond1454.not.i = icmp eq i64 %586, %_26, !dbg !4563
  br i1 %exitcond1454.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb32.i.i, !dbg !4566, !llvm.loop !4592

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit: ; preds = %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.us.us.i103, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.us.i267, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.us.i419, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i575, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.i729, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i888, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i
  %_82.i.i192 = trunc i64 %_26 to i32, !dbg !4593
  %_81.i.i193 = add i32 %base.i.i49, %_82.i.i192, !dbg !4594
  store i32 %_81.i.i193, ptr %_57.i32, align 4, !dbg !4596, !alias.scope !3861, !noalias !3921
  br label %bb7, !dbg !4597

bb26:                                             ; preds = %bb25
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i, i64 noundef %right.1, i64 noundef %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6d4388bd1c2ee005f6a969a0e3ca0f4) #24, !dbg !4598
  unreachable, !dbg !4598
}
