define internal fastcc void @_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_(ptr noalias noundef nonnull align 8 dereferenceable(1232) %self, ptr noalias noundef nonnull align 4 %left.0, i64 noundef range(i64 0, 2305843009213693952) %left.1, ptr noalias noundef nonnull align 4 %right.0, i64 noundef range(i64 0, 2305843009213693952) %right.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %sidechain, i64 noundef %frames, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(320) %reports) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !2526 {
start:
  %iter.i = alloca [56 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 1220, !dbg !2527
  %_9 = load i32, ptr %0, align 4, !dbg !2527, !noundef !12
  %_8 = zext i32 %_9 to i64, !dbg !2529
  %..i = tail call noundef range(i64 0, 4294967296) i64 @llvm.umin.i64(i64 %frames, i64 range(i64 0, 4294967296) %_8), !dbg !2530
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
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i, i64 noundef %_32.sroa.4.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2154676760dbbfbf0360c56922cbcced) #24, !dbg !2562, !noalias !2563
  unreachable, !dbg !2562

bb1.i:                                            ; preds = %bb12
  %_32.sroa.5.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 16, !dbg !2541
  %_32.sroa.5.0.copyload = load ptr, ptr %_32.sroa.5.0.sidechain.sroa_idx, align 8, !dbg !2541, !nonnull !12, !noundef !12
  %_17.not.i = icmp ugt i64 %..i, %_32.sroa.6.0.copyload, !dbg !2567
  br i1 %_17.not.i, label %bb6.i, label %bb9, !dbg !2567, !prof !180

bb6.i:                                            ; preds = %bb1.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i, i64 noundef %_32.sroa.6.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc4bf442cf077b790fbd54430ab2bad8) #24, !dbg !2573, !noalias !2563
  unreachable, !dbg !2573

bb9:                                              ; preds = %bb2, %bb1.i
  %side.sroa.4.0 = phi ptr [ %_32.sroa.5.0.copyload, %bb1.i ], [ undef, %bb2 ]
  %_33.not = icmp samesign ugt i64 %..i, %left.1
  br i1 %_33.not, label %bb16, label %bb14, !dbg !2574, !prof !2561

bb16:                                             ; preds = %bb9
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i, i64 noundef %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bc75afababbf20974edb8b966373fd0c) #24, !dbg !2585
  unreachable, !dbg !2585

bb14:                                             ; preds = %bb9
  %_41.not = icmp samesign ugt i64 %..i, %right.1, !dbg !2586
  br i1 %_41.not, label %bb19, label %bb18, !dbg !2586, !prof !180

bb19:                                             ; preds = %bb14
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i, i64 noundef %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0e50f2d670fa7dbeabdb68abcb7ac10) #24, !dbg !2592
  unreachable, !dbg !2592

bb18:                                             ; preds = %bb14
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2593), !dbg !2596
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2597), !dbg !2596
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2599), !dbg !2596
  %_10.i = getelementptr inbounds nuw i8, ptr %self, i64 792, !dbg !2601
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !2604
  %_15.i = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !2618
  %data.i.i651.i = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !2620
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 744, !dbg !2631
  %_32.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !2635
  %_58.0.i = load ptr, ptr %_15.i, align 8, !dbg !2636, !alias.scope !2593, !noalias !2637, !nonnull !12, !noundef !12
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !2636
  %_58.1.i = load i64, ptr %3, align 8, !dbg !2636, !alias.scope !2593, !noalias !2637, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !2639
  %_59.0.i = load ptr, ptr %4, align 8, !dbg !2639, !alias.scope !2593, !noalias !2637, !nonnull !12, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !2639
  %_59.1.i = load i64, ptr %5, align 8, !dbg !2639, !alias.scope !2593, !noalias !2637, !noundef !12
  %_45.i = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !2640
  %_60.0.i = load ptr, ptr %data.i.i651.i, align 8, !dbg !2641, !alias.scope !2593, !noalias !2637, !nonnull !12, !noundef !12
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !2641
  %_60.1.i = load i64, ptr %6, align 8, !dbg !2641, !alias.scope !2593, !noalias !2637, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 184, !dbg !2642
  %_61.0.i = load ptr, ptr %7, align 8, !dbg !2642, !alias.scope !2593, !noalias !2637, !nonnull !12, !noundef !12
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 192, !dbg !2642
  %_61.1.i = load i64, ptr %8, align 8, !dbg !2642, !alias.scope !2593, !noalias !2637, !noundef !12
  %_50.i = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !2643
  %_51.i = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !2644
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !2645
  %_52.i = load i32, ptr %9, align 4, !dbg !2645, !alias.scope !2593, !noalias !2637, !noundef !12
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !2646
  %_53.i = load i32, ptr %10, align 8, !dbg !2646, !alias.scope !2593, !noalias !2637, !noundef !12
  br i1 %.not, label %bb37.i.i, label %bb39.i.i, !dbg !2647

bb39.i.i:                                         ; preds = %bb18
  %11 = icmp ne ptr %side.sroa.4.0, null
  tail call void @llvm.assume(i1 %11)
  br label %bb37.i.i, !dbg !2658

bb37.i.i:                                         ; preds = %bb39.i.i, %bb18
  %empty.sroa.6.0.i.i = phi i64 [ %..i, %bb39.i.i ], [ 0, %bb18 ], !dbg !2659
  %empty.sroa.0.0.i.i = phi ptr [ %side.sroa.4.0, %bb39.i.i ], [ inttoptr (i64 4 to ptr), %bb18 ], !dbg !2659
  %side_left.sroa.0.0.i.i = phi ptr [ %1, %bb39.i.i ], [ inttoptr (i64 4 to ptr), %bb18 ], !dbg !2660
  %base.i.i = load i32, ptr %_51.i, align 4, !dbg !2661, !alias.scope !2593, !noalias !2663, !noundef !12
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 856
  %13 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %15 = getelementptr inbounds nuw i8, ptr %self, i64 860
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %18 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %19 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 932
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 784
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 788
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 936
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 776
  %25 = getelementptr inbounds nuw i8, ptr %self, i64 940
  %26 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %27 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %injected.cond1226.not.i = icmp ugt i64 %_58.1.i, %_60.1.i
  br i1 %injected.cond1226.not.i, label %bb40.i.us.preheader.i, label %repeat_loop_next16.i.split.split.us.split.us.i

bb40.i.us.preheader.i:                            ; preds = %bb37.i.i
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i37.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %33 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i37.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %35 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %36 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i37.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %45 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %51 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br label %bb40.i.us.i, !dbg !2666

repeat_loop_next16.i.split.split.us.split.us.i:   ; preds = %bb37.i.i
  %injected.cond1327.not.i = icmp samesign ugt i64 %..i, %empty.sroa.6.0.i.i
  br i1 %injected.cond1327.not.i, label %bb40.i.us.us.preheader.i, label %repeat_loop_next16.i.split.split.us.split.us.split.us.i

bb40.i.us.us.preheader.i:                         ; preds = %repeat_loop_next16.i.split.split.us.split.us.i
  %52 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %53 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %54 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i37.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i37.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %60 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i37.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %64 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %65 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %72 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %73 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %74 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br label %bb42.i.us.us.i, !dbg !2666

repeat_loop_next16.i.split.split.us.split.us.split.us.i: ; preds = %repeat_loop_next16.i.split.split.us.split.us.i
  %injected.cond1422.not.i = icmp ugt i64 %_58.1.i, %_59.1.i
  br i1 %injected.cond1422.not.i, label %bb40.i.us.us.us.preheader.i, label %repeat_loop_next16.i.split.split.us.split.us.split.us.split.us.split.us.i

bb40.i.us.us.us.preheader.i:                      ; preds = %repeat_loop_next16.i.split.split.us.split.us.split.us.i
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i37.us.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i37.us.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %82 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %84 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i37.us.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %86 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %88 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.us.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %91 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.us.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.us.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br label %bb40.i.us.us.us.i, !dbg !2666

repeat_loop_next16.i.split.split.us.split.us.split.us.split.us.split.us.i: ; preds = %repeat_loop_next16.i.split.split.us.split.us.split.us.i
  %injected.cond1594.not.i = icmp ugt i64 %_58.1.i, %_61.1.i
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.us.us.us.us.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.us.us.us.us.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.us.us.us.us.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br i1 %injected.cond1594.not.i, label %bb40.i.us.us.us.us.us.i, label %bb40.i.us.us.us.us.us.us.us.i

bb40.i.us.us.us.us.us.us.us.i:                    ; preds = %repeat_loop_next16.i.split.split.us.split.us.split.us.split.us.split.us.i, %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i
  %iter.sroa.0.0.i1071.us.us.us.us.us.us.us.i = phi i64 [ %124, %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i ], [ 0, %repeat_loop_next16.i.split.split.us.split.us.split.us.split.us.split.us.i ]
  %124 = add nuw nsw i64 %iter.sroa.0.0.i1071.us.us.us.us.us.us.us.i, 1, !dbg !2678
  %_27.i.us.us.us.us.us.us.us.i = trunc i64 %iter.sroa.0.0.i1071.us.us.us.us.us.us.us.i to i32, !dbg !2689
  %now.i.us.us.us.us.us.us.us.i = add i32 %base.i.i, %_27.i.us.us.us.us.us.us.us.i, !dbg !2690
  %_30.i.us.us.us.us.us.us.us.i = and i32 %now.i.us.us.us.us.us.us.us.i, %_52.i, !dbg !2693
  %_29.i.us.us.us.us.us.us.us.i = zext i32 %_30.i.us.us.us.us.us.us.us.i to i64, !dbg !2694
  %exitcond.not.i = icmp eq i64 %iter.sroa.0.0.i1071.us.us.us.us.us.us.us.i, %..i, !dbg !2666
  br i1 %exitcond.not.i, label %bb43.i.i, label %bb42.i.us.us.us.us.us.us.us.i, !dbg !2666, !prof !180

bb42.i.us.us.us.us.us.us.us.i:                    ; preds = %bb40.i.us.us.us.us.us.us.us.i
  %_123.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i1071.us.us.us.us.us.us.us.i, !dbg !2695
  %_124.not.not.i.us.us.us.us.us.us.us.i = icmp ugt i64 %_58.1.i, %_29.i.us.us.us.us.us.us.us.i, !dbg !2699
  br i1 %_124.not.not.i.us.us.us.us.us.us.us.i, label %bb45.i.us.us.us.us.us.us.us.i, label %bb46.i.i, !dbg !2699, !prof !2704

bb45.i.us.us.us.us.us.us.us.i:                    ; preds = %bb42.i.us.us.us.us.us.us.us.i
  %_0.i319.us.us.us.us.us.us.us.i = load float, ptr %_123.i.us.us.us.us.us.us.us.i, align 4, !dbg !2705, !alias.scope !2711, !noalias !2714, !noundef !12
  %_133.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_29.i.us.us.us.us.us.us.us.i, !dbg !2715
  store float %_0.i319.us.us.us.us.us.us.us.i, ptr %_133.i.us.us.us.us.us.us.us.i, align 4, !dbg !2719, !alias.scope !2722, !noalias !2663
  %_141.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i1071.us.us.us.us.us.us.us.i, !dbg !2725
  %_0.i317.us.us.us.us.us.us.us.i = load float, ptr %_141.i.us.us.us.us.us.us.us.i, align 4, !dbg !2732, !alias.scope !2734, !noalias !2737, !noundef !12
  %_149.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_29.i.us.us.us.us.us.us.us.i, !dbg !2738
  store float %_0.i317.us.us.us.us.us.us.us.i, ptr %_149.i.us.us.us.us.us.us.us.i, align 4, !dbg !2745, !alias.scope !2747, !noalias !2663
  %_157.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.us.us.us.us.us.us.i, !dbg !2750
  %_0.i315.us.us.us.us.us.us.us.i = load float, ptr %_157.i.us.us.us.us.us.us.us.i, align 4, !dbg !2757, !alias.scope !2759, !noalias !2663, !noundef !12
  %_165.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_29.i.us.us.us.us.us.us.us.i, !dbg !2762
  store float %_0.i315.us.us.us.us.us.us.us.i, ptr %_165.i.us.us.us.us.us.us.us.i, align 4, !dbg !2769, !alias.scope !2771, !noalias !2663
  %_173.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.us.us.us.us.us.us.i, !dbg !2774
  %_0.i313.us.us.us.us.us.us.us.i = load float, ptr %_173.i.us.us.us.us.us.us.us.i, align 4, !dbg !2781, !alias.scope !2783, !noalias !2663, !noundef !12
  %_181.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_29.i.us.us.us.us.us.us.us.i, !dbg !2786
  store float %_0.i313.us.us.us.us.us.us.us.i, ptr %_181.i.us.us.us.us.us.us.us.i, align 4, !dbg !2793, !alias.scope !2795, !noalias !2663
  %_52.i.us.us.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.us.us.i, %_53.i, !dbg !2798
  %_51.i.us.us.us.us.us.us.us.i = and i32 %_52.i.us.us.us.us.us.us.us.i, %_52.i, !dbg !2801
  %_50.i.us.us.us.us.us.us.us.i = zext i32 %_51.i.us.us.us.us.us.us.us.i to i64, !dbg !2802
  %_182.not.not.i.us.us.us.us.us.us.us.i = icmp ugt i64 %_58.1.i, %_50.i.us.us.us.us.us.us.us.i, !dbg !2803
  br i1 %_182.not.not.i.us.us.us.us.us.us.us.i, label %bb60.i.us.us.us.us.us.us.us.i, label %bb61.i.i, !dbg !2803, !prof !2704

bb60.i.us.us.us.us.us.us.us.i:                    ; preds = %bb45.i.us.us.us.us.us.us.us.i
  %_189.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_50.i.us.us.us.us.us.us.us.i, !dbg !2808
  %_0.i311.us.us.us.us.us.us.us.i = load float, ptr %_189.i.us.us.us.us.us.us.us.i, align 4, !dbg !2812, !alias.scope !2814, !noalias !2663, !noundef !12
  %_195.i.us.us.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_50.i.us.us.us.us.us.us.us.i, !dbg !2817
  %_0.i309.us.us.us.us.us.us.us.i = load float, ptr %_195.i.us.us.us.us.us.us.us.i, align 4, !dbg !2825, !alias.scope !2827, !noalias !2663, !noundef !12
  %_67.i.us.us.us.us.us.us.us.i = load i32, ptr %_45.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_66.i.us.us.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.us.us.i, %_67.i.us.us.us.us.us.us.us.i
  %_65.i.us.us.us.us.us.us.us.i = and i32 %_66.i.us.us.us.us.us.us.us.i, %_52.i
  %_64.i.us.us.us.us.us.us.us.i = zext i32 %_65.i.us.us.us.us.us.us.us.i to i64
  %_75.i.us.us.us.us.us.us.us.i = load i32, ptr %_50.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_74.i.us.us.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.us.us.i, %_75.i.us.us.us.us.us.us.us.i
  %_73.i.us.us.us.us.us.us.us.i = and i32 %_74.i.us.us.us.us.us.us.us.i, %_52.i
  %_72.i.us.us.us.us.us.us.us.i = zext i32 %_73.i.us.us.us.us.us.us.us.i to i64
  %_80.i.us.us.us.us.us.us.us.i = icmp ugt i64 %_59.1.i, %_64.i.us.us.us.us.us.us.us.i
  %125 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_64.i.us.us.us.us.us.us.us.i
  %126 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_64.i.us.us.us.us.us.us.us.i
  %_86.i.us.us.us.us.us.us.us.i = icmp ugt i64 %_61.1.i, %_72.i.us.us.us.us.us.us.us.i
  %127 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_72.i.us.us.us.us.us.us.us.i
  %_88.i.us.us.us.us.us.us.us.i = icmp ugt i64 %_59.1.i, %_72.i.us.us.us.us.us.us.us.i
  %128 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_72.i.us.us.us.us.us.us.us.i
  br i1 %_80.i.us.us.us.us.us.us.us.i, label %bb63.i.split.us.us.us.us.us.us.us.us.i, label %bb63.i.split.i

bb63.i.split.us.us.us.us.us.us.us.us.i:           ; preds = %bb60.i.us.us.us.us.us.us.us.i
  %_84.i.us.us.us.us.us.us.us.i = icmp ugt i64 %_61.1.i, %_64.i.us.us.us.us.us.us.us.i
  br i1 %_84.i.us.us.us.us.us.us.us.i, label %bb24.i.us.lr.ph.us.us.us.us.us.us.us.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i, !dbg !2830

bb24.i.us.lr.ph.us.us.us.us.us.us.us.i:           ; preds = %bb63.i.split.us.us.us.us.us.us.us.us.i
  %_82.i.us.us.us.us.us.us.us.us.i = load float, ptr %126, align 4, !noalias !2663, !noundef !12
  br i1 %_86.i.us.us.us.us.us.us.us.i, label %bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i, label %bb24.i.us.lr.ph.split.i

bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i:  ; preds = %bb24.i.us.lr.ph.us.us.us.us.us.us.us.i
  br i1 %_88.i.us.us.us.us.us.us.us.i, label %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i, !dbg !2838

bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i:        ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i
  %_87.i.us.us.us.us.us.us.us.us.us.i = load float, ptr %128, align 4, !noalias !2663, !noundef !12
  %_85.i.us.us.le931.us.us.us.us.us.us.us.i = load float, ptr %127, align 4, !noalias !2663, !noundef !12
  %_78.i.us.le.us.us.us.us.us.us.us.i = load float, ptr %125, align 4, !noalias !2663, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2839), !dbg !2842
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2842
  %_12.i39.us.us.us.us.us.us.us.i = load float, ptr %100, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.us.us.us.us.i = fcmp ule float %_12.i39.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.us.us.us.us.i = fcmp une float %_12.i39.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.us.us.us.us.i = load float, ptr %_10.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.us.us.us.us.i = load float, ptr %101, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.us.us.us.us.i = fadd float %_16.i42.us.us.us.us.us.us.us.i, %_17.i43.us.us.us.us.us.us.us.i, !dbg !2864
  %_20.i45655658.us.us.us.us.us.us.us.i = load float, ptr %102, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %129 = select i1 %_3.i196.us.us.us.us.us.us.us.i, float %_0.i255.us.us.us.us.us.us.us.i, float %_20.i45655658.us.us.us.us.us.us.us.i, !dbg !2869
  %_0.i407.us.us.us.us.us.us.us.i = select i1 %_3.i224.us.us.us.us.us.us.us.i, float %_16.i42.us.us.us.us.us.us.us.i, float %129, !dbg !2872
  store float %_0.i407.us.us.us.us.us.us.us.i, ptr %_10.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.us.us.us.us.i = select i1 %_3.i196.us.us.us.us.us.us.us.i, float %_17.i43.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.us.us.us.us.i, ptr %101, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.us.us.us.us.i = fadd float %_12.i39.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.us.us.us.us.i = select i1 %_3.i224.us.us.us.us.us.us.us.i, float %_12.i39.us.us.us.us.us.us.us.i, float %_0.i293.us.us.us.us.us.us.us.i, !dbg !2881
  store float %_4.i393.v.us.us.us.us.us.us.us.i, ptr %100, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.us.us.us.us.1.i = load float, ptr %103, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.us.us.us.us.1.i = fcmp ule float %_12.i39.us.us.us.us.us.us.us.1.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.us.us.us.us.1.i = fcmp une float %_12.i39.us.us.us.us.us.us.us.1.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.us.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.1.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.us.us.us.us.1.i = load float, ptr %104, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.us.us.us.us.1.i = fadd float %_16.i42.us.us.us.us.us.us.us.1.i, %_17.i43.us.us.us.us.us.us.us.1.i, !dbg !2864
  %_20.i45655658.us.us.us.us.us.us.us.1.i = load float, ptr %105, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %130 = select i1 %_3.i196.us.us.us.us.us.us.us.1.i, float %_0.i255.us.us.us.us.us.us.us.1.i, float %_20.i45655658.us.us.us.us.us.us.us.1.i, !dbg !2869
  %_0.i407.us.us.us.us.us.us.us.1.i = select i1 %_3.i224.us.us.us.us.us.us.us.1.i, float %_16.i42.us.us.us.us.us.us.us.1.i, float %130, !dbg !2872
  store float %_0.i407.us.us.us.us.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.1.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.us.us.us.us.1.i = select i1 %_3.i196.us.us.us.us.us.us.us.1.i, float %_17.i43.us.us.us.us.us.us.us.1.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.us.us.us.us.1.i, ptr %104, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.us.us.us.us.1.i = fadd float %_12.i39.us.us.us.us.us.us.us.1.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.us.us.us.us.1.i = select i1 %_3.i224.us.us.us.us.us.us.us.1.i, float %_12.i39.us.us.us.us.us.us.us.1.i, float %_0.i293.us.us.us.us.us.us.us.1.i, !dbg !2881
  store float %_4.i393.v.us.us.us.us.us.us.us.1.i, ptr %103, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.us.us.us.us.2.i = load float, ptr %106, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.us.us.us.us.2.i = fcmp ule float %_12.i39.us.us.us.us.us.us.us.2.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.us.us.us.us.2.i = fcmp une float %_12.i39.us.us.us.us.us.us.us.2.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.us.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.2.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.us.us.us.us.2.i = load float, ptr %107, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.us.us.us.us.2.i = fadd float %_16.i42.us.us.us.us.us.us.us.2.i, %_17.i43.us.us.us.us.us.us.us.2.i, !dbg !2864
  %_20.i45655658.us.us.us.us.us.us.us.2.i = load float, ptr %108, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %131 = select i1 %_3.i196.us.us.us.us.us.us.us.2.i, float %_0.i255.us.us.us.us.us.us.us.2.i, float %_20.i45655658.us.us.us.us.us.us.us.2.i, !dbg !2869
  %_0.i407.us.us.us.us.us.us.us.2.i = select i1 %_3.i224.us.us.us.us.us.us.us.2.i, float %_16.i42.us.us.us.us.us.us.us.2.i, float %131, !dbg !2872
  store float %_0.i407.us.us.us.us.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.2.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.us.us.us.us.2.i = select i1 %_3.i196.us.us.us.us.us.us.us.2.i, float %_17.i43.us.us.us.us.us.us.us.2.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.us.us.us.us.2.i, ptr %107, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.us.us.us.us.2.i = fadd float %_12.i39.us.us.us.us.us.us.us.2.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.us.us.us.us.2.i = select i1 %_3.i224.us.us.us.us.us.us.us.2.i, float %_12.i39.us.us.us.us.us.us.us.2.i, float %_0.i293.us.us.us.us.us.us.us.2.i, !dbg !2881
  store float %_4.i393.v.us.us.us.us.us.us.us.2.i, ptr %106, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.us.us.us.us.3.i = load float, ptr %109, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.us.us.us.us.3.i = fcmp ule float %_12.i39.us.us.us.us.us.us.us.3.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.us.us.us.us.3.i = fcmp une float %_12.i39.us.us.us.us.us.us.us.3.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.us.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.3.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.us.us.us.us.3.i = load float, ptr %110, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.us.us.us.us.3.i = fadd float %_16.i42.us.us.us.us.us.us.us.3.i, %_17.i43.us.us.us.us.us.us.us.3.i, !dbg !2864
  %_20.i45655658.us.us.us.us.us.us.us.3.i = load float, ptr %111, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %132 = select i1 %_3.i196.us.us.us.us.us.us.us.3.i, float %_0.i255.us.us.us.us.us.us.us.3.i, float %_20.i45655658.us.us.us.us.us.us.us.3.i, !dbg !2869
  %_0.i407.us.us.us.us.us.us.us.3.i = select i1 %_3.i224.us.us.us.us.us.us.us.3.i, float %_16.i42.us.us.us.us.us.us.us.3.i, float %132, !dbg !2872
  store float %_0.i407.us.us.us.us.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.3.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.us.us.us.us.3.i = select i1 %_3.i196.us.us.us.us.us.us.us.3.i, float %_17.i43.us.us.us.us.us.us.us.3.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.us.us.us.us.3.i, ptr %110, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.us.us.us.us.3.i = fadd float %_12.i39.us.us.us.us.us.us.us.3.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.us.us.us.us.3.i = select i1 %_3.i224.us.us.us.us.us.us.us.3.i, float %_12.i39.us.us.us.us.us.us.us.3.i, float %_0.i293.us.us.us.us.us.us.us.3.i, !dbg !2881
  store float %_4.i393.v.us.us.us.us.us.us.us.3.i, ptr %109, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %133 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.us.us.us.us.i), !dbg !2884
  %134 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.us.us.us.us.i), !dbg !2891
  %_37.i60.us.us.us.us.us.us.us.i = load float, ptr %13, align 4, !dbg !2894, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i222.us.us.us.us.us.us.us.i = fcmp ule float %_37.i60.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !2899
  %_3.i.i560.us.us.us.us.us.us.us.i = fcmp ule float %133, %134, !dbg !2901
  %_6.i.i562.us.us.us.us.us.us.us.i = bitcast float %133 to i32, !dbg !2907
  %_8.i.i564.us.us.us.us.us.us.us.i = bitcast float %134 to i32, !dbg !2911
  %_4.i.i567.us.us.us.us.us.us.us.i = select i1 %_3.i.i560.us.us.us.us.us.us.us.i, i32 %_8.i.i564.us.us.us.us.us.us.us.i, i32 %_6.i.i562.us.us.us.us.us.us.us.i, !dbg !2913
  %_4.i386.us.us.us.us.us.us.us.i = select i1 %_3.i222.us.us.us.us.us.us.us.i, i32 %_6.i.i562.us.us.us.us.us.us.us.i, i32 %_4.i.i567.us.us.us.us.us.us.us.i, !dbg !2914
  %_41.i64.us.us.us.us.us.us.us.i = load float, ptr %14, align 4, !dbg !2916, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i220.us.us.us.us.us.us.us.i = fcmp ule float %_41.i64.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !2917
  %_0.i277.us.us.us.us.us.us.us.i = fmul float %133, 5.000000e-01, !dbg !2919
  %_0.i276.us.us.us.us.us.us.us.i = fmul float %134, 5.000000e-01, !dbg !2922
  %_0.i254.us.us.us.us.us.us.us.i = fadd float %_0.i276.us.us.us.us.us.us.us.i, %_0.i277.us.us.us.us.us.us.us.i, !dbg !2924
  %_6.i374.us.us.us.us.us.us.us.i = bitcast float %_0.i254.us.us.us.us.us.us.us.i to i32, !dbg !2926
  %_4.i379.us.us.us.us.us.us.us.i = select i1 %_3.i220.us.us.us.us.us.us.us.i, i32 %_4.i386.us.us.us.us.us.us.us.i, i32 %_6.i374.us.us.us.us.us.us.us.i, !dbg !2929
  %_0.i380.us.us.us.us.us.us.us.i = bitcast i32 %_4.i379.us.us.us.us.us.us.us.i to float, !dbg !2930
  %_3.i.i552.us.us.us.us.us.us.us.i = fcmp ule float %_0.i380.us.us.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !2933
  %_4.i.i558.us.us.us.us.us.us.us.i = select i1 %_3.i.i552.us.us.us.us.us.us.us.i, i32 841731191, i32 %_4.i379.us.us.us.us.us.us.us.i, !dbg !2936
  %_0.i.i559.us.us.us.us.us.us.us.i = bitcast i32 %_4.i.i558.us.us.us.us.us.us.us.i to float, !dbg !2938
  %_3.i.i512.us.us.us.us.us.us.us.i = fcmp ule float %_0.i.i559.us.us.us.us.us.us.us.i, 0x3810000000000000, !dbg !2940
  %_4.i.i518.us.us.us.us.us.us.us.i = select i1 %_3.i.i512.us.us.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i558.us.us.us.us.us.us.us.i, !dbg !2950
  %_5.i324.us.us.us.us.us.us.us.i = and i32 %_4.i.i518.us.us.us.us.us.us.us.i, 8388607, !dbg !2952
  %_4.i325.us.us.us.us.us.us.us.i = or disjoint i32 %_5.i324.us.us.us.us.us.us.us.i, 1065353216, !dbg !2952
  %significand.i326.us.us.us.us.us.us.us.i = bitcast i32 %_4.i325.us.us.us.us.us.us.us.i to float, !dbg !2957
  %_0.i285.us.us.us.us.us.us.us.i = fadd float %significand.i326.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !2960
  %_0.i265.us.us.us.us.us.us.us.i = fmul float %_0.i285.us.us.us.us.us.us.us.i, 0x3F9B17A960000000, !dbg !2963
  %135 = fsub float 0x3FBF9A8440000000, %_0.i265.us.us.us.us.us.us.us.i, !dbg !2968
  %_0.i265.us.us.us.us.us.us.us.1.i = fmul float %_0.i285.us.us.us.us.us.us.us.i, %135, !dbg !2963
  %_0.i249.us.us.us.us.us.us.us.1.i = fadd float %_0.i265.us.us.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !2968
  %_0.i265.us.us.us.us.us.us.us.2.i = fmul float %_0.i285.us.us.us.us.us.us.us.i, %_0.i249.us.us.us.us.us.us.us.1.i, !dbg !2963
  %_0.i249.us.us.us.us.us.us.us.2.i = fadd float %_0.i265.us.us.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !2968
  %_0.i265.us.us.us.us.us.us.us.3.i = fmul float %_0.i285.us.us.us.us.us.us.us.i, %_0.i249.us.us.us.us.us.us.us.2.i, !dbg !2963
  %_0.i249.us.us.us.us.us.us.us.3.i = fadd float %_0.i265.us.us.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !2968
  %_0.i265.us.us.us.us.us.us.us.4.i = fmul float %_0.i285.us.us.us.us.us.us.us.i, %_0.i249.us.us.us.us.us.us.us.3.i, !dbg !2963
  %_0.i249.us.us.us.us.us.us.us.4.i = fadd float %_0.i265.us.us.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !2968
  %_9.i327.us.us.us.us.us.us.us.i = lshr i32 %_4.i.i518.us.us.us.us.us.us.us.i, 23, !dbg !2970
  %_8.i328.us.us.us.us.us.us.us.i = or disjoint i32 %_9.i327.us.us.us.us.us.us.us.i, 1258291200, !dbg !2970
  %_7.i329.us.us.us.us.us.us.us.i = bitcast i32 %_8.i328.us.us.us.us.us.us.us.i to float, !dbg !2972
  %exponent.i330.us.us.us.us.us.us.us.i = fadd float %_7.i329.us.us.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !2974
  %_0.i264.us.us.us.us.us.us.us.i = fmul float %_0.i285.us.us.us.us.us.us.us.i, %_0.i249.us.us.us.us.us.us.us.4.i, !dbg !2975
  %_0.i248.us.us.us.us.us.us.us.i = fadd float %exponent.i330.us.us.us.us.us.us.us.i, %_0.i264.us.us.us.us.us.us.us.i, !dbg !2977
  %_0.i275.us.us.us.us.us.us.us.i = fmul float %_0.i248.us.us.us.us.us.us.us.i, 0x4018151820000000, !dbg !2979
  %_3.i.i627.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i275.us.us.us.us.us.us.us.i, 2.400000e+01, !dbg !2981
  %_0.i.i634.us.us.us.us.us.us.us.i = select i1 %_3.i.i627.us.us.us.us.us.us.us.inv.i, float %_0.i275.us.us.us.us.us.us.us.i, float 2.400000e+01, !dbg !2981
  %_3.i.i544.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i634.us.us.us.us.us.us.us.i, -1.600000e+02, !dbg !2985
  %_0.i.i551.us.us.us.us.us.us.us.i = select i1 %_3.i.i544.us.us.us.us.us.us.us.inv.i, float %_0.i.i634.us.us.us.us.us.us.us.i, float -1.600000e+02, !dbg !2985
  %_55.i81.us.us.us.us.us.us.us.i = load float, ptr %12, align 4, !dbg !2988, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i218.us.us.us.us.us.us.us.i = fcmp ule float %_55.i81.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !2990
  %_3.i204.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i551.us.us.us.us.us.us.us.i, %_0.i407.us.us.us.us.us.us.us.i, !dbg !2992
  %_0.i292.us.us.us.us.us.us.us.i = fsub float %_0.i407.us.us.us.us.us.us.us.i, %_0.i407.us.us.us.us.us.us.us.3.i, !dbg !2996
  %_3.i202.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i551.us.us.us.us.us.us.us.i, %_0.i292.us.us.us.us.us.us.us.i, !dbg !2999
  %..i203.us.us.us.us.us.us.us.i = sext i1 %_3.i202.us.us.us.us.us.us.us.i to i32, !dbg !3001
  %_0.i502.us.us.us.us.us.us.us.i = sext i1 %_3.i204.us.us.us.us.us.us.us.i to i32, !dbg !3004
  %_0.i496.us.us.us.us.us.us.us.i = select i1 %_3.i218.us.us.us.us.us.us.us.i, i32 %_0.i502.us.us.us.us.us.us.us.i, i32 %..i203.us.us.us.us.us.us.us.i, !dbg !3004
  %_0.i508.us.us.us.us.us.us.us.i = xor i32 %..i203.us.us.us.us.us.us.us.i, -1, !dbg !3010
  %_67.i91.us.us.us.us.us.us.us.i = load float, ptr %15, align 4, !dbg !3014, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i216.us.us.us.us.us.us.us.i = fcmp ogt float %_67.i91.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3015
  %_0.i501.us.us.us.us.us.us.us.i = select i1 %_3.i216.us.us.us.us.us.us.us.i, i32 %_0.i508.us.us.us.us.us.us.us.i, i32 0, !dbg !3017
  %_0.i500.us.us.us.us.us.us.us.i = select i1 %_3.i218.us.us.us.us.us.us.us.i, i32 0, i32 %_0.i501.us.us.us.us.us.us.us.i, !dbg !3020
  %_0.i495.us.us.us.us.us.us.us.i = or i32 %_0.i500.us.us.us.us.us.us.us.i, %_0.i496.us.us.us.us.us.us.us.i, !dbg !3022
  %_5.i369.us.us.us.us.us.us.us.i = and i32 %_0.i495.us.us.us.us.us.us.us.i, 1065353216, !dbg !3025
  %_0.i373.us.us.us.us.us.us.us.i = bitcast i32 %_5.i369.us.us.us.us.us.us.us.i to float, !dbg !3027
  %_71.i97667668.us.us.us.us.us.us.us.i = load float, ptr %16, align 4, !dbg !3029, !alias.scope !2897, !noalias !2898, !noundef !12
  %_0.i291.us.us.us.us.us.us.us.i = fadd float %_67.i91.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3031
  %136 = trunc nsw i32 %_0.i500.us.us.us.us.us.us.us.i to i1, !dbg !3033
  %_4.i367.v.us.us.us.us.us.us.us.i = select i1 %136, float %_0.i291.us.us.us.us.us.us.us.i, float %_67.i91.us.us.us.us.us.us.us.i, !dbg !3033
  %137 = trunc nsw i32 %_0.i496.us.us.us.us.us.us.us.i to i1, !dbg !3035
  %_0.i361.us.us.us.us.us.us.us.i = select i1 %137, float %_71.i97667668.us.us.us.us.us.us.us.i, float %_4.i367.v.us.us.us.us.us.us.us.i, !dbg !3035
  store float %_0.i361.us.us.us.us.us.us.us.i, ptr %15, align 4, !dbg !3037, !alias.scope !2852, !noalias !2853
  store i32 %_5.i369.us.us.us.us.us.us.us.i, ptr %12, align 4, !dbg !3038, !alias.scope !2852, !noalias !2853
  %_0.i290.us.us.us.us.us.us.us.i = fadd float %_0.i407.us.us.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3039
  %_0.i289.us.us.us.us.us.us.us.i = fsub float %_0.i.i551.us.us.us.us.us.us.us.i, %_0.i407.us.us.us.us.us.us.us.i, !dbg !3041
  %_0.i274.us.us.us.us.us.us.us.i = fmul float %_0.i290.us.us.us.us.us.us.us.i, %_0.i289.us.us.us.us.us.us.us.i, !dbg !3043
  %138 = fneg float %_0.i407.us.us.us.us.us.us.us.2.i, !dbg !3045
  %_3.i.i536.inv.us.us.us.us.us.us.us.i = fcmp ogt float %_0.i274.us.us.us.us.us.us.us.i, %138, !dbg !3048
  %_4.i.i542.v.us.us.us.us.us.us.us.i = select i1 %_3.i.i536.inv.us.us.us.us.us.us.us.i, float %_0.i274.us.us.us.us.us.us.us.i, float %138, !dbg !3048
  %_3.i.i619.us.us.us.us.us.us.us.i = fcmp olt float %_4.i.i542.v.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3051
  %139 = fcmp ule float %_0.i373.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3055
  %140 = select i1 %139, i1 %_3.i.i619.us.us.us.us.us.us.us.i, i1 false, !dbg !3058
  %_0.i354.us.us.us.us.us.us.us.i = select i1 %140, float %_4.i.i542.v.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3058
  %_86.i110.us.us.us.us.us.us.us.i = load float, ptr %17, align 4, !dbg !3059, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i212.us.us.us.us.us.us.us.i = fcmp ule float %_0.i354.us.us.us.us.us.us.us.i, %_86.i110.us.us.us.us.us.us.us.i, !dbg !3061
  %_87.i112670.us.us.us.us.us.us.us.i = load i32, ptr %2, align 4, !dbg !3063, !alias.scope !2897, !noalias !2898, !noundef !12
  %_88.i113671.us.us.us.us.us.us.us.i = load i32, ptr %18, align 4, !dbg !3064, !alias.scope !2897, !noalias !2898, !noundef !12
  %_4.i347.us.us.us.us.us.us.us.i = select i1 %_3.i212.us.us.us.us.us.us.us.i, i32 %_88.i113671.us.us.us.us.us.us.us.i, i32 %_87.i112670.us.us.us.us.us.us.us.i, !dbg !3065
  %_0.i348.us.us.us.us.us.us.us.i = bitcast i32 %_4.i347.us.us.us.us.us.us.us.i to float, !dbg !3067
  %_0.i288.us.us.us.us.us.us.us.i = fsub float %_0.i354.us.us.us.us.us.us.us.i, %_86.i110.us.us.us.us.us.us.us.i, !dbg !3069
  %_4.i258.us.us.us.us.us.us.us.i = fmul float %_0.i288.us.us.us.us.us.us.us.i, %_0.i348.us.us.us.us.us.us.us.i, !dbg !3072
  %_0.i259.us.us.us.us.us.us.us.i = fadd float %_86.i110.us.us.us.us.us.us.us.i, %_4.i258.us.us.us.us.us.us.us.i, !dbg !3072
  %141 = tail call noundef float @llvm.fabs.f32(float %_0.i259.us.us.us.us.us.us.us.i), !dbg !3075
  %142 = fcmp uge float %141, 0x3BC79CA100000000, !dbg !3079
  %_0.i332.us.us.us.us.us.us.us.i = select i1 %142, float %_0.i259.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3082
  store float %_0.i332.us.us.us.us.us.us.us.i, ptr %17, align 4, !dbg !3083, !alias.scope !2852, !noalias !2853
  %_0.i273.us.us.us.us.us.us.us.i = fmul float %_0.i332.us.us.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !3085
  %_3.i.i528.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i273.us.us.us.us.us.us.us.i, -1.260000e+02, !dbg !3089
  %_0.i.i535.us.us.us.us.us.us.us.i = select i1 %_3.i.i528.us.us.us.us.us.us.us.inv.i, float %_0.i273.us.us.us.us.us.us.us.i, float -1.260000e+02, !dbg !3089
  %_3.i.i611.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i535.us.us.us.us.us.us.us.i, 1.270000e+02, !dbg !3094
  %_0.i.i618.us.us.us.us.us.us.us.i = select i1 %_3.i.i611.us.us.us.us.us.us.us.inv.i, float %_0.i.i535.us.us.us.us.us.us.us.i, float 1.270000e+02, !dbg !3094
  %143 = tail call noundef float @llvm.floor.f32(float %_0.i.i618.us.us.us.us.us.us.us.i), !dbg !3097
  %_0.i287.us.us.us.us.us.us.us.i = fsub float %_0.i.i618.us.us.us.us.us.us.us.i, %143, !dbg !3109
  %_98.i126.us.us.us.us.us.us.us.i = load float, ptr %19, align 4, !dbg !3112, !alias.scope !2897, !noalias !2898, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3114), !dbg !3117
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3119), !dbg !3117
  %_12.i.us.us.us.us.us.us.us.i = load float, ptr %112, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.us.us.us.us.i = fcmp ule float %_12.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.us.us.us.us.i = fcmp une float %_12.i.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.us.us.us.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.us.us.us.us.i = load float, ptr %113, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.us.us.us.us.i = fadd float %_16.i.us.us.us.us.us.us.us.i, %_17.i.us.us.us.us.us.us.us.i, !dbg !3131
  %_20.i677680.us.us.us.us.us.us.us.i = load float, ptr %114, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %144 = select i1 %_3.i200.us.us.us.us.us.us.us.i, float %_0.i257.us.us.us.us.us.us.us.i, float %_20.i677680.us.us.us.us.us.us.us.i, !dbg !3134
  %_0.i486.us.us.us.us.us.us.us.i = select i1 %_3.i240.us.us.us.us.us.us.us.i, float %_16.i.us.us.us.us.us.us.us.i, float %144, !dbg !3136
  store float %_0.i486.us.us.us.us.us.us.us.i, ptr %data.i.i.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.us.us.us.us.i = select i1 %_3.i200.us.us.us.us.us.us.us.i, float %_17.i.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.us.us.us.us.i, ptr %113, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.us.us.us.us.i = fadd float %_12.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.us.us.us.us.i = select i1 %_3.i240.us.us.us.us.us.us.us.i, float %_12.i.us.us.us.us.us.us.us.i, float %_0.i299.us.us.us.us.us.us.us.i, !dbg !3144
  store float %_4.i472.v.us.us.us.us.us.us.us.i, ptr %112, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.us.us.us.us.1.i = load float, ptr %115, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.us.us.us.us.1.i = fcmp ule float %_12.i.us.us.us.us.us.us.us.1.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.us.us.us.us.1.i = fcmp une float %_12.i.us.us.us.us.us.us.us.1.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.us.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.1.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.us.us.us.us.1.i = load float, ptr %116, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.us.us.us.us.1.i = fadd float %_16.i.us.us.us.us.us.us.us.1.i, %_17.i.us.us.us.us.us.us.us.1.i, !dbg !3131
  %_20.i677680.us.us.us.us.us.us.us.1.i = load float, ptr %117, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %145 = select i1 %_3.i200.us.us.us.us.us.us.us.1.i, float %_0.i257.us.us.us.us.us.us.us.1.i, float %_20.i677680.us.us.us.us.us.us.us.1.i, !dbg !3134
  %_0.i486.us.us.us.us.us.us.us.1.i = select i1 %_3.i240.us.us.us.us.us.us.us.1.i, float %_16.i.us.us.us.us.us.us.us.1.i, float %145, !dbg !3136
  store float %_0.i486.us.us.us.us.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.1.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.us.us.us.us.1.i = select i1 %_3.i200.us.us.us.us.us.us.us.1.i, float %_17.i.us.us.us.us.us.us.us.1.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.us.us.us.us.1.i, ptr %116, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.us.us.us.us.1.i = fadd float %_12.i.us.us.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.us.us.us.us.1.i = select i1 %_3.i240.us.us.us.us.us.us.us.1.i, float %_12.i.us.us.us.us.us.us.us.1.i, float %_0.i299.us.us.us.us.us.us.us.1.i, !dbg !3144
  store float %_4.i472.v.us.us.us.us.us.us.us.1.i, ptr %115, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.us.us.us.us.2.i = load float, ptr %118, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.us.us.us.us.2.i = fcmp ule float %_12.i.us.us.us.us.us.us.us.2.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.us.us.us.us.2.i = fcmp une float %_12.i.us.us.us.us.us.us.us.2.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.us.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.2.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.us.us.us.us.2.i = load float, ptr %119, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.us.us.us.us.2.i = fadd float %_16.i.us.us.us.us.us.us.us.2.i, %_17.i.us.us.us.us.us.us.us.2.i, !dbg !3131
  %_20.i677680.us.us.us.us.us.us.us.2.i = load float, ptr %120, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %146 = select i1 %_3.i200.us.us.us.us.us.us.us.2.i, float %_0.i257.us.us.us.us.us.us.us.2.i, float %_20.i677680.us.us.us.us.us.us.us.2.i, !dbg !3134
  %_0.i486.us.us.us.us.us.us.us.2.i = select i1 %_3.i240.us.us.us.us.us.us.us.2.i, float %_16.i.us.us.us.us.us.us.us.2.i, float %146, !dbg !3136
  store float %_0.i486.us.us.us.us.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.2.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.us.us.us.us.2.i = select i1 %_3.i200.us.us.us.us.us.us.us.2.i, float %_17.i.us.us.us.us.us.us.us.2.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.us.us.us.us.2.i, ptr %119, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.us.us.us.us.2.i = fadd float %_12.i.us.us.us.us.us.us.us.2.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.us.us.us.us.2.i = select i1 %_3.i240.us.us.us.us.us.us.us.2.i, float %_12.i.us.us.us.us.us.us.us.2.i, float %_0.i299.us.us.us.us.us.us.us.2.i, !dbg !3144
  store float %_4.i472.v.us.us.us.us.us.us.us.2.i, ptr %118, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.us.us.us.us.3.i = load float, ptr %121, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.us.us.us.us.3.i = fcmp ule float %_12.i.us.us.us.us.us.us.us.3.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.us.us.us.us.3.i = fcmp une float %_12.i.us.us.us.us.us.us.us.3.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.us.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.3.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.us.us.us.us.3.i = load float, ptr %122, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.us.us.us.us.3.i = fadd float %_16.i.us.us.us.us.us.us.us.3.i, %_17.i.us.us.us.us.us.us.us.3.i, !dbg !3131
  %_20.i677680.us.us.us.us.us.us.us.3.i = load float, ptr %123, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %147 = select i1 %_3.i200.us.us.us.us.us.us.us.3.i, float %_0.i257.us.us.us.us.us.us.us.3.i, float %_20.i677680.us.us.us.us.us.us.us.3.i, !dbg !3134
  %_0.i486.us.us.us.us.us.us.us.3.i = select i1 %_3.i240.us.us.us.us.us.us.us.3.i, float %_16.i.us.us.us.us.us.us.us.3.i, float %147, !dbg !3136
  store float %_0.i486.us.us.us.us.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.3.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.us.us.us.us.3.i = select i1 %_3.i200.us.us.us.us.us.us.us.3.i, float %_17.i.us.us.us.us.us.us.us.3.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.us.us.us.us.3.i, ptr %122, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.us.us.us.us.3.i = fadd float %_12.i.us.us.us.us.us.us.us.3.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.us.us.us.us.3.i = select i1 %_3.i240.us.us.us.us.us.us.us.3.i, float %_12.i.us.us.us.us.us.us.us.3.i, float %_0.i299.us.us.us.us.us.us.us.3.i, !dbg !3144
  store float %_4.i472.v.us.us.us.us.us.us.us.3.i, ptr %121, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %148 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le931.us.us.us.us.us.us.us.i), !dbg !3147
  %149 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.us.us.us.us.i), !dbg !3149
  %_37.i.us.us.us.us.us.us.us.i = load float, ptr %21, align 4, !dbg !3151, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i238.us.us.us.us.us.us.us.i = fcmp ule float %_37.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3154
  %_3.i.i594.us.us.us.us.us.us.us.i = fcmp ule float %148, %149, !dbg !3156
  %_6.i.i596.us.us.us.us.us.us.us.i = bitcast float %148 to i32, !dbg !3159
  %_8.i.i598.us.us.us.us.us.us.us.i = bitcast float %149 to i32, !dbg !3162
  %_4.i.i601.us.us.us.us.us.us.us.i = select i1 %_3.i.i594.us.us.us.us.us.us.us.i, i32 %_8.i.i598.us.us.us.us.us.us.us.i, i32 %_6.i.i596.us.us.us.us.us.us.us.i, !dbg !3164
  %_4.i465.us.us.us.us.us.us.us.i = select i1 %_3.i238.us.us.us.us.us.us.us.i, i32 %_6.i.i596.us.us.us.us.us.us.us.i, i32 %_4.i.i601.us.us.us.us.us.us.us.i, !dbg !3165
  %_41.i.us.us.us.us.us.us.us.i = load float, ptr %22, align 4, !dbg !3167, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i236.us.us.us.us.us.us.us.i = fcmp ule float %_41.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3168
  %_0.i283.us.us.us.us.us.us.us.i = fmul float %148, 5.000000e-01, !dbg !3170
  %_0.i282.us.us.us.us.us.us.us.i = fmul float %149, 5.000000e-01, !dbg !3172
  %_0.i256.us.us.us.us.us.us.us.i = fadd float %_0.i282.us.us.us.us.us.us.us.i, %_0.i283.us.us.us.us.us.us.us.i, !dbg !3174
  %_6.i453.us.us.us.us.us.us.us.i = bitcast float %_0.i256.us.us.us.us.us.us.us.i to i32, !dbg !3176
  %_4.i458.us.us.us.us.us.us.us.i = select i1 %_3.i236.us.us.us.us.us.us.us.i, i32 %_4.i465.us.us.us.us.us.us.us.i, i32 %_6.i453.us.us.us.us.us.us.us.i, !dbg !3179
  %_0.i459.us.us.us.us.us.us.us.i = bitcast i32 %_4.i458.us.us.us.us.us.us.us.i to float, !dbg !3180
  %_3.i.i586.us.us.us.us.us.us.us.i = fcmp ule float %_0.i459.us.us.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !3182
  %_4.i.i592.us.us.us.us.us.us.us.i = select i1 %_3.i.i586.us.us.us.us.us.us.us.i, i32 841731191, i32 %_4.i458.us.us.us.us.us.us.us.i, !dbg !3185
  %_0.i.i593.us.us.us.us.us.us.us.i = bitcast i32 %_4.i.i592.us.us.us.us.us.us.us.i to float, !dbg !3187
  %_3.i.i.us.us.us.us.us.us.us.i = fcmp ule float %_0.i.i593.us.us.us.us.us.us.us.i, 0x3810000000000000, !dbg !3189
  %_4.i.i.us.us.us.us.us.us.us.i = select i1 %_3.i.i.us.us.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i592.us.us.us.us.us.us.us.i, !dbg !3194
  %_5.i320.us.us.us.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.us.us.us.i, 8388607, !dbg !3196
  %_4.i321.us.us.us.us.us.us.us.i = or disjoint i32 %_5.i320.us.us.us.us.us.us.us.i, 1065353216, !dbg !3196
  %significand.i.us.us.us.us.us.us.us.i = bitcast i32 %_4.i321.us.us.us.us.us.us.us.i to float, !dbg !3198
  %_0.i284.us.us.us.us.us.us.us.i = fadd float %significand.i.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3200
  %_0.i263.us.us.us.us.us.us.us.i = fmul float %_0.i284.us.us.us.us.us.us.us.i, 0x3F9B17A960000000, !dbg !3202
  %150 = fsub float 0x3FBF9A8440000000, %_0.i263.us.us.us.us.us.us.us.i, !dbg !3204
  %_0.i263.us.us.us.us.us.us.us.1.i = fmul float %_0.i284.us.us.us.us.us.us.us.i, %150, !dbg !3202
  %_0.i247.us.us.us.us.us.us.us.1.i = fadd float %_0.i263.us.us.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3204
  %_0.i263.us.us.us.us.us.us.us.2.i = fmul float %_0.i284.us.us.us.us.us.us.us.i, %_0.i247.us.us.us.us.us.us.us.1.i, !dbg !3202
  %_0.i247.us.us.us.us.us.us.us.2.i = fadd float %_0.i263.us.us.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3204
  %_0.i263.us.us.us.us.us.us.us.3.i = fmul float %_0.i284.us.us.us.us.us.us.us.i, %_0.i247.us.us.us.us.us.us.us.2.i, !dbg !3202
  %_0.i247.us.us.us.us.us.us.us.3.i = fadd float %_0.i263.us.us.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3204
  %_0.i263.us.us.us.us.us.us.us.4.i = fmul float %_0.i284.us.us.us.us.us.us.us.i, %_0.i247.us.us.us.us.us.us.us.3.i, !dbg !3202
  %_0.i247.us.us.us.us.us.us.us.4.i = fadd float %_0.i263.us.us.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3204
  %_9.i.us.us.us.us.us.us.us.i = lshr i32 %_4.i.i.us.us.us.us.us.us.us.i, 23, !dbg !3206
  %_8.i322.us.us.us.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.us.us.us.i, 1258291200, !dbg !3206
  %_7.i.us.us.us.us.us.us.us.i = bitcast i32 %_8.i322.us.us.us.us.us.us.us.i to float, !dbg !3207
  %exponent.i.us.us.us.us.us.us.us.i = fadd float %_7.i.us.us.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !3209
  %_0.i262.us.us.us.us.us.us.us.i = fmul float %_0.i284.us.us.us.us.us.us.us.i, %_0.i247.us.us.us.us.us.us.us.4.i, !dbg !3210
  %_0.i246.us.us.us.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.us.us.us.i, %_0.i262.us.us.us.us.us.us.us.i, !dbg !3212
  %_0.i281.us.us.us.us.us.us.us.i = fmul float %_0.i246.us.us.us.us.us.us.us.i, 0x4018151820000000, !dbg !3214
  %_3.i.i643.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i281.us.us.us.us.us.us.us.i, 2.400000e+01, !dbg !3216
  %_0.i.i650.us.us.us.us.us.us.us.i = select i1 %_3.i.i643.us.us.us.us.us.us.us.inv.i, float %_0.i281.us.us.us.us.us.us.us.i, float 2.400000e+01, !dbg !3216
  %_3.i.i578.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i650.us.us.us.us.us.us.us.i, -1.600000e+02, !dbg !3219
  %_0.i.i585.us.us.us.us.us.us.us.i = select i1 %_3.i.i578.us.us.us.us.us.us.us.inv.i, float %_0.i.i650.us.us.us.us.us.us.us.i, float -1.600000e+02, !dbg !3219
  %_55.i12.us.us.us.us.us.us.us.i = load float, ptr %20, align 4, !dbg !3222, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i234.us.us.us.us.us.us.us.i = fcmp ule float %_55.i12.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3223
  %_3.i208.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i585.us.us.us.us.us.us.us.i, %_0.i486.us.us.us.us.us.us.us.i, !dbg !3225
  %_0.i298.us.us.us.us.us.us.us.i = fsub float %_0.i486.us.us.us.us.us.us.us.i, %_0.i486.us.us.us.us.us.us.us.3.i, !dbg !3227
  %_3.i206.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i585.us.us.us.us.us.us.us.i, %_0.i298.us.us.us.us.us.us.us.i, !dbg !3229
  %..i207.us.us.us.us.us.us.us.i = sext i1 %_3.i206.us.us.us.us.us.us.us.i to i32, !dbg !3231
  %_0.i506.us.us.us.us.us.us.us.i = sext i1 %_3.i208.us.us.us.us.us.us.us.i to i32, !dbg !3233
  %_0.i499.us.us.us.us.us.us.us.i = select i1 %_3.i234.us.us.us.us.us.us.us.i, i32 %_0.i506.us.us.us.us.us.us.us.i, i32 %..i207.us.us.us.us.us.us.us.i, !dbg !3233
  %_0.i510.us.us.us.us.us.us.us.i = xor i32 %..i207.us.us.us.us.us.us.us.i, -1, !dbg !3235
  %_67.i14.us.us.us.us.us.us.us.i = load float, ptr %23, align 4, !dbg !3237, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i232.us.us.us.us.us.us.us.i = fcmp ogt float %_67.i14.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3238
  %_0.i505.us.us.us.us.us.us.us.i = select i1 %_3.i232.us.us.us.us.us.us.us.i, i32 %_0.i510.us.us.us.us.us.us.us.i, i32 0, !dbg !3240
  %_0.i504.us.us.us.us.us.us.us.i = select i1 %_3.i234.us.us.us.us.us.us.us.i, i32 0, i32 %_0.i505.us.us.us.us.us.us.us.i, !dbg !3242
  %_0.i498.us.us.us.us.us.us.us.i = or i32 %_0.i504.us.us.us.us.us.us.us.i, %_0.i499.us.us.us.us.us.us.us.i, !dbg !3244
  %_5.i448.us.us.us.us.us.us.us.i = and i32 %_0.i498.us.us.us.us.us.us.us.i, 1065353216, !dbg !3246
  %_0.i452.us.us.us.us.us.us.us.i = bitcast i32 %_5.i448.us.us.us.us.us.us.us.i to float, !dbg !3248
  %_71.i689690.us.us.us.us.us.us.us.i = load float, ptr %24, align 4, !dbg !3250, !alias.scope !3152, !noalias !3153, !noundef !12
  %_0.i297.us.us.us.us.us.us.us.i = fadd float %_67.i14.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !3251
  %151 = trunc nsw i32 %_0.i504.us.us.us.us.us.us.us.i to i1, !dbg !3253
  %_4.i446.v.us.us.us.us.us.us.us.i = select i1 %151, float %_0.i297.us.us.us.us.us.us.us.i, float %_67.i14.us.us.us.us.us.us.us.i, !dbg !3253
  %152 = trunc nsw i32 %_0.i499.us.us.us.us.us.us.us.i to i1, !dbg !3255
  %_0.i440.us.us.us.us.us.us.us.i = select i1 %152, float %_71.i689690.us.us.us.us.us.us.us.i, float %_4.i446.v.us.us.us.us.us.us.us.i, !dbg !3255
  store float %_0.i440.us.us.us.us.us.us.us.i, ptr %23, align 4, !dbg !3257, !alias.scope !3123, !noalias !3124
  store i32 %_5.i448.us.us.us.us.us.us.us.i, ptr %20, align 4, !dbg !3258, !alias.scope !3123, !noalias !3124
  %_0.i296.us.us.us.us.us.us.us.i = fadd float %_0.i486.us.us.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3259
  %_0.i295.us.us.us.us.us.us.us.i = fsub float %_0.i.i585.us.us.us.us.us.us.us.i, %_0.i486.us.us.us.us.us.us.us.i, !dbg !3261
  %_0.i280.us.us.us.us.us.us.us.i = fmul float %_0.i296.us.us.us.us.us.us.us.i, %_0.i295.us.us.us.us.us.us.us.i, !dbg !3263
  %153 = fneg float %_0.i486.us.us.us.us.us.us.us.2.i, !dbg !3265
  %_3.i.i569.inv.us.us.us.us.us.us.us.i = fcmp ogt float %_0.i280.us.us.us.us.us.us.us.i, %153, !dbg !3267
  %_4.i.i576.v.us.us.us.us.us.us.us.i = select i1 %_3.i.i569.inv.us.us.us.us.us.us.us.i, float %_0.i280.us.us.us.us.us.us.us.i, float %153, !dbg !3267
  %_3.i.i635.us.us.us.us.us.us.us.i = fcmp olt float %_4.i.i576.v.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3270
  %154 = fcmp ule float %_0.i452.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3273
  %155 = select i1 %154, i1 %_3.i.i635.us.us.us.us.us.us.us.i, i1 false, !dbg !3275
  %_0.i433.us.us.us.us.us.us.us.i = select i1 %155, float %_4.i.i576.v.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3275
  %_86.i22.us.us.us.us.us.us.us.i = load float, ptr %25, align 4, !dbg !3276, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i228.us.us.us.us.us.us.us.i = fcmp ule float %_0.i433.us.us.us.us.us.us.us.i, %_86.i22.us.us.us.us.us.us.us.i, !dbg !3277
  %_87.i24692.us.us.us.us.us.us.us.i = load i32, ptr %_32.i, align 4, !dbg !3279, !alias.scope !3152, !noalias !3153, !noundef !12
  %_88.i25693.us.us.us.us.us.us.us.i = load i32, ptr %26, align 4, !dbg !3280, !alias.scope !3152, !noalias !3153, !noundef !12
  %_4.i427.us.us.us.us.us.us.us.i = select i1 %_3.i228.us.us.us.us.us.us.us.i, i32 %_88.i25693.us.us.us.us.us.us.us.i, i32 %_87.i24692.us.us.us.us.us.us.us.i, !dbg !3281
  %_0.i.us.us.us.us.us.us.us.i = bitcast i32 %_4.i427.us.us.us.us.us.us.us.i to float, !dbg !3283
  %_0.i294.us.us.us.us.us.us.us.i = fsub float %_0.i433.us.us.us.us.us.us.us.i, %_86.i22.us.us.us.us.us.us.us.i, !dbg !3285
  %_4.i260.us.us.us.us.us.us.us.i = fmul float %_0.i294.us.us.us.us.us.us.us.i, %_0.i.us.us.us.us.us.us.us.i, !dbg !3287
  %_0.i261.us.us.us.us.us.us.us.i = fadd float %_86.i22.us.us.us.us.us.us.us.i, %_4.i260.us.us.us.us.us.us.us.i, !dbg !3287
  %156 = tail call noundef float @llvm.fabs.f32(float %_0.i261.us.us.us.us.us.us.us.i), !dbg !3289
  %157 = fcmp uge float %156, 0x3BC79CA100000000, !dbg !3292
  %_0.i336.us.us.us.us.us.us.us.i = select i1 %157, float %_0.i261.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !3294
  store float %_0.i336.us.us.us.us.us.us.us.i, ptr %25, align 4, !dbg !3295, !alias.scope !3123, !noalias !3124
  %_0.i279.us.us.us.us.us.us.us.i = fmul float %_0.i336.us.us.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !3296
  %_3.i.i520.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i279.us.us.us.us.us.us.us.i, -1.260000e+02, !dbg !3299
  %_0.i.i527.us.us.us.us.us.us.us.i = select i1 %_3.i.i520.us.us.us.us.us.us.us.inv.i, float %_0.i279.us.us.us.us.us.us.us.i, float -1.260000e+02, !dbg !3299
  %_3.i.i603.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i527.us.us.us.us.us.us.us.i, 1.270000e+02, !dbg !3303
  %_0.i.i610.us.us.us.us.us.us.us.i = select i1 %_3.i.i603.us.us.us.us.us.us.us.inv.i, float %_0.i.i527.us.us.us.us.us.us.us.i, float 1.270000e+02, !dbg !3303
  %158 = tail call noundef float @llvm.floor.f32(float %_0.i.i610.us.us.us.us.us.us.us.i), !dbg !3306
  %_0.i286.us.us.us.us.us.us.us.i = fsub float %_0.i.i610.us.us.us.us.us.us.us.i, %158, !dbg !3310
  %_0.i268.us.us.us.us.us.us.us.i = fmul float %_0.i286.us.us.us.us.us.us.us.i, 0x3F5E974FA0000000, !dbg !3312
  %_0.i251.us.us.us.us.us.us.us.i = fadd float %_0.i268.us.us.us.us.us.us.us.i, 0x3F82778560000000, !dbg !3317
  %_0.i268.us.us.us.us.us.us.us.1.i = fmul float %_0.i286.us.us.us.us.us.us.us.i, %_0.i251.us.us.us.us.us.us.us.i, !dbg !3312
  %_0.i251.us.us.us.us.us.us.us.1.i = fadd float %_0.i268.us.us.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3317
  %_0.i268.us.us.us.us.us.us.us.2.i = fmul float %_0.i286.us.us.us.us.us.us.us.i, %_0.i251.us.us.us.us.us.us.us.1.i, !dbg !3312
  %_0.i251.us.us.us.us.us.us.us.2.i = fadd float %_0.i268.us.us.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3317
  %_0.i268.us.us.us.us.us.us.us.3.i = fmul float %_0.i286.us.us.us.us.us.us.us.i, %_0.i251.us.us.us.us.us.us.us.2.i, !dbg !3312
  %_0.i251.us.us.us.us.us.us.us.3.i = fadd float %_0.i268.us.us.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3317
  %_0.i271.us.us.us.us.us.us.us.i = fmul float %_0.i287.us.us.us.us.us.us.us.i, 0x3F5E974FA0000000, !dbg !3319
  %_0.i253.us.us.us.us.us.us.us.i = fadd float %_0.i271.us.us.us.us.us.us.us.i, 0x3F82778560000000, !dbg !3321
  %_0.i271.us.us.us.us.us.us.us.1.i = fmul float %_0.i287.us.us.us.us.us.us.us.i, %_0.i253.us.us.us.us.us.us.us.i, !dbg !3319
  %_0.i253.us.us.us.us.us.us.us.1.i = fadd float %_0.i271.us.us.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3321
  %_0.i271.us.us.us.us.us.us.us.2.i = fmul float %_0.i287.us.us.us.us.us.us.us.i, %_0.i253.us.us.us.us.us.us.us.1.i, !dbg !3319
  %_0.i253.us.us.us.us.us.us.us.2.i = fadd float %_0.i271.us.us.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3321
  %_0.i271.us.us.us.us.us.us.us.3.i = fmul float %_0.i287.us.us.us.us.us.us.us.i, %_0.i253.us.us.us.us.us.us.us.2.i, !dbg !3319
  %_0.i253.us.us.us.us.us.us.us.3.i = fadd float %_0.i271.us.us.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3321
  %_0.i270.us.us.us.us.us.us.us.i = fmul float %_0.i287.us.us.us.us.us.us.us.i, %_0.i253.us.us.us.us.us.us.us.3.i, !dbg !3323
  %_0.i252.us.us.us.us.us.us.us.i = fadd float %_0.i270.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !3325
  %biased.i191.us.us.us.us.us.us.us.i = fadd float %143, 0x4160000FE0000000, !dbg !3327
  %_4.i192.us.us.us.us.us.us.us.i = bitcast float %biased.i191.us.us.us.us.us.us.us.i to i32, !dbg !3331
  %_3.i193.us.us.us.us.us.us.us.i = shl i32 %_4.i192.us.us.us.us.us.us.us.i, 23, !dbg !3335
  %_0.i194.us.us.us.us.us.us.us.i = bitcast i32 %_3.i193.us.us.us.us.us.us.us.i to float, !dbg !3336
  %_0.i269.us.us.us.us.us.us.us.i = fmul float %_0.i252.us.us.us.us.us.us.us.i, %_0.i194.us.us.us.us.us.us.us.i, !dbg !3339
  %_3.i195.us.us.us.us.us.us.us.i = fcmp une float %_0.i332.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3341
  %_3.i210.us.us.us.us.us.us.us.i = fcmp ule float %_98.i126.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3343
  %_0.i494675.not.us.us.us.us.us.us.us.i = and i1 %_3.i210.us.us.us.us.us.us.us.i, %_3.i195.us.us.us.us.us.us.us.i, !dbg !3345
  %_0.i272.us.us.us.us.us.us.us.i = fmul float %_0.i311.us.us.us.us.us.us.us.i, %_0.i269.us.us.us.us.us.us.us.i, !dbg !3345
  %_4.i340.v.us.us.us.us.us.us.us.i = select i1 %_0.i494675.not.us.us.us.us.us.us.us.i, float %_0.i272.us.us.us.us.us.us.us.i, float %_0.i311.us.us.us.us.us.us.us.i, !dbg !3348
  %_0.i267.us.us.us.us.us.us.us.i = fmul float %_0.i286.us.us.us.us.us.us.us.i, %_0.i251.us.us.us.us.us.us.us.3.i, !dbg !3350
  %_0.i250.us.us.us.us.us.us.us.i = fadd float %_0.i267.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !3352
  %biased.i.us.us.us.us.us.us.us.i = fadd float %158, 0x4160000FE0000000, !dbg !3354
  %_4.i188.us.us.us.us.us.us.us.i = bitcast float %biased.i.us.us.us.us.us.us.us.i to i32, !dbg !3356
  %_3.i189.us.us.us.us.us.us.us.i = shl i32 %_4.i188.us.us.us.us.us.us.us.i, 23, !dbg !3358
  %_0.i190.us.us.us.us.us.us.us.i = bitcast i32 %_3.i189.us.us.us.us.us.us.us.i to float, !dbg !3359
  %_0.i266.us.us.us.us.us.us.us.i = fmul float %_0.i250.us.us.us.us.us.us.us.i, %_0.i190.us.us.us.us.us.us.us.i, !dbg !3361
  %_3.i198.us.us.us.us.us.us.us.i = fcmp une float %_0.i336.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3363
  %_98.i.us.us.us.us.us.us.us.i = load float, ptr %27, align 4, !dbg !3365, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i226.us.us.us.us.us.us.us.i = fcmp ule float %_98.i.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !3366
  %_0.i497697.not.us.us.us.us.us.us.us.i = and i1 %_3.i226.us.us.us.us.us.us.us.i, %_3.i198.us.us.us.us.us.us.us.i, !dbg !3368
  %_0.i278.us.us.us.us.us.us.us.i = fmul float %_0.i309.us.us.us.us.us.us.us.i, %_0.i266.us.us.us.us.us.us.us.i, !dbg !3368
  %_4.i420.v.us.us.us.us.us.us.us.i = select i1 %_0.i497697.not.us.us.us.us.us.us.us.i, float %_0.i278.us.us.us.us.us.us.us.i, float %_0.i309.us.us.us.us.us.us.us.i, !dbg !3370
  store float %_4.i340.v.us.us.us.us.us.us.us.i, ptr %_123.i.us.us.us.us.us.us.us.i, align 4, !dbg !3372, !alias.scope !3375, !noalias !2714
  store float %_4.i420.v.us.us.us.us.us.us.us.i, ptr %_141.i.us.us.us.us.us.us.us.i, align 4, !dbg !3378, !alias.scope !3380, !noalias !2737
  %exitcond2621.not.i = icmp eq i64 %124, %..i, !dbg !3383
  br i1 %exitcond2621.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb40.i.us.us.us.us.us.us.us.i, !dbg !3386

bb40.i.us.us.us.us.us.i:                          ; preds = %repeat_loop_next16.i.split.split.us.split.us.split.us.split.us.split.us.i, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i
  %iter.sroa.0.0.i1071.us.us.us.us.us.i = phi i64 [ %159, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i ], [ 0, %repeat_loop_next16.i.split.split.us.split.us.split.us.split.us.split.us.i ]
  %159 = add nuw nsw i64 %iter.sroa.0.0.i1071.us.us.us.us.us.i, 1, !dbg !2678
  %_27.i.us.us.us.us.us.i = trunc i64 %iter.sroa.0.0.i1071.us.us.us.us.us.i to i32, !dbg !2689
  %now.i.us.us.us.us.us.i = add i32 %base.i.i, %_27.i.us.us.us.us.us.i, !dbg !2690
  %_30.i.us.us.us.us.us.i = and i32 %now.i.us.us.us.us.us.i, %_52.i, !dbg !2693
  %_29.i.us.us.us.us.us.i = zext i32 %_30.i.us.us.us.us.us.i to i64, !dbg !2694
  %exitcond2624.not.i = icmp eq i64 %iter.sroa.0.0.i1071.us.us.us.us.us.i, %..i, !dbg !2666
  br i1 %exitcond2624.not.i, label %bb43.i.i, label %bb42.i.us.us.us.us.us.i, !dbg !2666, !prof !180

bb42.i.us.us.us.us.us.i:                          ; preds = %bb40.i.us.us.us.us.us.i
  %_123.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i1071.us.us.us.us.us.i, !dbg !2695
  %_124.not.not.i.us.us.us.us.us.i = icmp ugt i64 %_58.1.i, %_29.i.us.us.us.us.us.i, !dbg !2699
  br i1 %_124.not.not.i.us.us.us.us.us.i, label %bb45.i.us.us.us.us.us.i, label %bb46.i.i, !dbg !2699, !prof !2704

bb45.i.us.us.us.us.us.i:                          ; preds = %bb42.i.us.us.us.us.us.i
  %_0.i319.us.us.us.us.us.i = load float, ptr %_123.i.us.us.us.us.us.i, align 4, !dbg !2705, !alias.scope !2711, !noalias !2714, !noundef !12
  %_133.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_29.i.us.us.us.us.us.i, !dbg !2715
  store float %_0.i319.us.us.us.us.us.i, ptr %_133.i.us.us.us.us.us.i, align 4, !dbg !2719, !alias.scope !2722, !noalias !2663
  %_141.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i1071.us.us.us.us.us.i, !dbg !2725
  %_0.i317.us.us.us.us.us.i = load float, ptr %_141.i.us.us.us.us.us.i, align 4, !dbg !2732, !alias.scope !2734, !noalias !2737, !noundef !12
  %_149.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_29.i.us.us.us.us.us.i, !dbg !2738
  store float %_0.i317.us.us.us.us.us.i, ptr %_149.i.us.us.us.us.us.i, align 4, !dbg !2745, !alias.scope !2747, !noalias !2663
  %_157.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.us.us.us.us.i, !dbg !2750
  %_0.i315.us.us.us.us.us.i = load float, ptr %_157.i.us.us.us.us.us.i, align 4, !dbg !2757, !alias.scope !2759, !noalias !2663, !noundef !12
  %_165.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_29.i.us.us.us.us.us.i, !dbg !2762
  store float %_0.i315.us.us.us.us.us.i, ptr %_165.i.us.us.us.us.us.i, align 4, !dbg !2769, !alias.scope !2771, !noalias !2663
  %_174.not.not.i.us.us.us.us.us.i = icmp ugt i64 %_61.1.i, %_29.i.us.us.us.us.us.i, !dbg !3387
  br i1 %_174.not.not.i.us.us.us.us.us.i, label %bb58.i.us.us.us.us.us.i, label %bb59.i.i, !dbg !3387, !prof !2704

bb58.i.us.us.us.us.us.i:                          ; preds = %bb45.i.us.us.us.us.us.i
  %_173.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.us.us.us.us.i, !dbg !2774
  %_0.i313.us.us.us.us.us.i = load float, ptr %_173.i.us.us.us.us.us.i, align 4, !dbg !2781, !alias.scope !2783, !noalias !2663, !noundef !12
  %_181.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_29.i.us.us.us.us.us.i, !dbg !2786
  store float %_0.i313.us.us.us.us.us.i, ptr %_181.i.us.us.us.us.us.i, align 4, !dbg !2793, !alias.scope !2795, !noalias !2663
  %_52.i.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.i, %_53.i, !dbg !2798
  %_51.i.us.us.us.us.us.i = and i32 %_52.i.us.us.us.us.us.i, %_52.i, !dbg !2801
  %_50.i.us.us.us.us.us.i = zext i32 %_51.i.us.us.us.us.us.i to i64, !dbg !2802
  %_182.not.not.i.us.us.us.us.us.i = icmp ugt i64 %_58.1.i, %_50.i.us.us.us.us.us.i, !dbg !2803
  br i1 %_182.not.not.i.us.us.us.us.us.i, label %bb60.i.us.us.us.us.us.i, label %bb61.i.i, !dbg !2803, !prof !2704

bb60.i.us.us.us.us.us.i:                          ; preds = %bb58.i.us.us.us.us.us.i
  %_189.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_50.i.us.us.us.us.us.i, !dbg !2808
  %_0.i311.us.us.us.us.us.i = load float, ptr %_189.i.us.us.us.us.us.i, align 4, !dbg !2812, !alias.scope !2814, !noalias !2663, !noundef !12
  %_195.i.us.us.us.us.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_50.i.us.us.us.us.us.i, !dbg !2817
  %_0.i309.us.us.us.us.us.i = load float, ptr %_195.i.us.us.us.us.us.i, align 4, !dbg !2825, !alias.scope !2827, !noalias !2663, !noundef !12
  %_67.i.us.us.us.us.us.i = load i32, ptr %_45.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_66.i.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.i, %_67.i.us.us.us.us.us.i
  %_65.i.us.us.us.us.us.i = and i32 %_66.i.us.us.us.us.us.i, %_52.i
  %_64.i.us.us.us.us.us.i = zext i32 %_65.i.us.us.us.us.us.i to i64
  %_75.i.us.us.us.us.us.i = load i32, ptr %_50.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_74.i.us.us.us.us.us.i = sub i32 %now.i.us.us.us.us.us.i, %_75.i.us.us.us.us.us.i
  %_73.i.us.us.us.us.us.i = and i32 %_74.i.us.us.us.us.us.i, %_52.i
  %_72.i.us.us.us.us.us.i = zext i32 %_73.i.us.us.us.us.us.i to i64
  %_80.i.us.us.us.us.us.i = icmp ugt i64 %_59.1.i, %_64.i.us.us.us.us.us.i
  %160 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_64.i.us.us.us.us.us.i
  %161 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_64.i.us.us.us.us.us.i
  %_86.i.us.us.us.us.us.i = icmp ugt i64 %_61.1.i, %_72.i.us.us.us.us.us.i
  %162 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_72.i.us.us.us.us.us.i
  %163 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_72.i.us.us.us.us.us.i
  br i1 %_80.i.us.us.us.us.us.i, label %bb63.i.split.us.us.us.us.us.us.i, label %bb63.i.split.i

bb63.i.split.us.us.us.us.us.us.i:                 ; preds = %bb60.i.us.us.us.us.us.i
  %_84.i.us.us.us.us.us.i = icmp ugt i64 %_61.1.i, %_64.i.us.us.us.us.us.i
  br i1 %_84.i.us.us.us.us.us.i, label %bb24.i.us.lr.ph.us.us.us.us.us.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i, !dbg !2830

bb24.i.us.lr.ph.us.us.us.us.us.i:                 ; preds = %bb63.i.split.us.us.us.us.us.us.i
  br i1 %_86.i.us.us.us.us.us.i, label %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i, label %bb24.i.us.lr.ph.split.i

bb24.i.us.lr.ph.split.us.us.us.us.us.us.i:        ; preds = %bb24.i.us.lr.ph.us.us.us.us.us.i
  %_82.i.us.us.us.us.us.us.i = load float, ptr %161, align 4, !noalias !2663, !noundef !12
  %_87.i.us.us.us.us.us.us.us.i = load float, ptr %163, align 4, !noalias !2663, !noundef !12
  %_85.i.us.us.le931.us.us.us.us.us.i = load float, ptr %162, align 4, !noalias !2663, !noundef !12
  %_78.i.us.le.us.us.us.us.us.i = load float, ptr %160, align 4, !noalias !2663, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2839), !dbg !2842
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2842
  %_12.i39.us.us.us.us.us.i = load float, ptr %100, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.us.us.i = fcmp ule float %_12.i39.us.us.us.us.us.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.us.us.i = fcmp une float %_12.i39.us.us.us.us.us.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.us.us.i = load float, ptr %_10.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.us.us.i = load float, ptr %101, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.us.us.i = fadd float %_16.i42.us.us.us.us.us.i, %_17.i43.us.us.us.us.us.i, !dbg !2864
  %_20.i45655658.us.us.us.us.us.i = load float, ptr %102, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %164 = select i1 %_3.i196.us.us.us.us.us.i, float %_0.i255.us.us.us.us.us.i, float %_20.i45655658.us.us.us.us.us.i, !dbg !2869
  %_0.i407.us.us.us.us.us.i = select i1 %_3.i224.us.us.us.us.us.i, float %_16.i42.us.us.us.us.us.i, float %164, !dbg !2872
  store float %_0.i407.us.us.us.us.us.i, ptr %_10.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.us.us.i = select i1 %_3.i196.us.us.us.us.us.i, float %_17.i43.us.us.us.us.us.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.us.us.i, ptr %101, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.us.us.i = fadd float %_12.i39.us.us.us.us.us.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.us.us.i = select i1 %_3.i224.us.us.us.us.us.i, float %_12.i39.us.us.us.us.us.i, float %_0.i293.us.us.us.us.us.i, !dbg !2881
  store float %_4.i393.v.us.us.us.us.us.i, ptr %100, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.us.us.1.i = load float, ptr %103, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.us.us.1.i = fcmp ule float %_12.i39.us.us.us.us.us.1.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.us.us.1.i = fcmp une float %_12.i39.us.us.us.us.us.1.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.1.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.us.us.1.i = load float, ptr %104, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.us.us.1.i = fadd float %_16.i42.us.us.us.us.us.1.i, %_17.i43.us.us.us.us.us.1.i, !dbg !2864
  %_20.i45655658.us.us.us.us.us.1.i = load float, ptr %105, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %165 = select i1 %_3.i196.us.us.us.us.us.1.i, float %_0.i255.us.us.us.us.us.1.i, float %_20.i45655658.us.us.us.us.us.1.i, !dbg !2869
  %_0.i407.us.us.us.us.us.1.i = select i1 %_3.i224.us.us.us.us.us.1.i, float %_16.i42.us.us.us.us.us.1.i, float %165, !dbg !2872
  store float %_0.i407.us.us.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.1.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.us.us.1.i = select i1 %_3.i196.us.us.us.us.us.1.i, float %_17.i43.us.us.us.us.us.1.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.us.us.1.i, ptr %104, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.us.us.1.i = fadd float %_12.i39.us.us.us.us.us.1.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.us.us.1.i = select i1 %_3.i224.us.us.us.us.us.1.i, float %_12.i39.us.us.us.us.us.1.i, float %_0.i293.us.us.us.us.us.1.i, !dbg !2881
  store float %_4.i393.v.us.us.us.us.us.1.i, ptr %103, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.us.us.2.i = load float, ptr %106, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.us.us.2.i = fcmp ule float %_12.i39.us.us.us.us.us.2.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.us.us.2.i = fcmp une float %_12.i39.us.us.us.us.us.2.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.2.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.us.us.2.i = load float, ptr %107, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.us.us.2.i = fadd float %_16.i42.us.us.us.us.us.2.i, %_17.i43.us.us.us.us.us.2.i, !dbg !2864
  %_20.i45655658.us.us.us.us.us.2.i = load float, ptr %108, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %166 = select i1 %_3.i196.us.us.us.us.us.2.i, float %_0.i255.us.us.us.us.us.2.i, float %_20.i45655658.us.us.us.us.us.2.i, !dbg !2869
  %_0.i407.us.us.us.us.us.2.i = select i1 %_3.i224.us.us.us.us.us.2.i, float %_16.i42.us.us.us.us.us.2.i, float %166, !dbg !2872
  store float %_0.i407.us.us.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.2.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.us.us.2.i = select i1 %_3.i196.us.us.us.us.us.2.i, float %_17.i43.us.us.us.us.us.2.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.us.us.2.i, ptr %107, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.us.us.2.i = fadd float %_12.i39.us.us.us.us.us.2.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.us.us.2.i = select i1 %_3.i224.us.us.us.us.us.2.i, float %_12.i39.us.us.us.us.us.2.i, float %_0.i293.us.us.us.us.us.2.i, !dbg !2881
  store float %_4.i393.v.us.us.us.us.us.2.i, ptr %106, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.us.us.3.i = load float, ptr %109, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.us.us.3.i = fcmp ule float %_12.i39.us.us.us.us.us.3.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.us.us.3.i = fcmp une float %_12.i39.us.us.us.us.us.3.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.3.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.us.us.3.i = load float, ptr %110, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.us.us.3.i = fadd float %_16.i42.us.us.us.us.us.3.i, %_17.i43.us.us.us.us.us.3.i, !dbg !2864
  %_20.i45655658.us.us.us.us.us.3.i = load float, ptr %111, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %167 = select i1 %_3.i196.us.us.us.us.us.3.i, float %_0.i255.us.us.us.us.us.3.i, float %_20.i45655658.us.us.us.us.us.3.i, !dbg !2869
  %_0.i407.us.us.us.us.us.3.i = select i1 %_3.i224.us.us.us.us.us.3.i, float %_16.i42.us.us.us.us.us.3.i, float %167, !dbg !2872
  store float %_0.i407.us.us.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.us.us.3.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.us.us.3.i = select i1 %_3.i196.us.us.us.us.us.3.i, float %_17.i43.us.us.us.us.us.3.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.us.us.3.i, ptr %110, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.us.us.3.i = fadd float %_12.i39.us.us.us.us.us.3.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.us.us.3.i = select i1 %_3.i224.us.us.us.us.us.3.i, float %_12.i39.us.us.us.us.us.3.i, float %_0.i293.us.us.us.us.us.3.i, !dbg !2881
  store float %_4.i393.v.us.us.us.us.us.3.i, ptr %109, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %168 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.us.us.i), !dbg !2884
  %169 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.us.us.i), !dbg !2891
  %_37.i60.us.us.us.us.us.i = load float, ptr %13, align 4, !dbg !2894, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i222.us.us.us.us.us.i = fcmp ule float %_37.i60.us.us.us.us.us.i, 0.000000e+00, !dbg !2899
  %_3.i.i560.us.us.us.us.us.i = fcmp ule float %168, %169, !dbg !2901
  %_6.i.i562.us.us.us.us.us.i = bitcast float %168 to i32, !dbg !2907
  %_8.i.i564.us.us.us.us.us.i = bitcast float %169 to i32, !dbg !2911
  %_4.i.i567.us.us.us.us.us.i = select i1 %_3.i.i560.us.us.us.us.us.i, i32 %_8.i.i564.us.us.us.us.us.i, i32 %_6.i.i562.us.us.us.us.us.i, !dbg !2913
  %_4.i386.us.us.us.us.us.i = select i1 %_3.i222.us.us.us.us.us.i, i32 %_6.i.i562.us.us.us.us.us.i, i32 %_4.i.i567.us.us.us.us.us.i, !dbg !2914
  %_41.i64.us.us.us.us.us.i = load float, ptr %14, align 4, !dbg !2916, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i220.us.us.us.us.us.i = fcmp ule float %_41.i64.us.us.us.us.us.i, 0.000000e+00, !dbg !2917
  %_0.i277.us.us.us.us.us.i = fmul float %168, 5.000000e-01, !dbg !2919
  %_0.i276.us.us.us.us.us.i = fmul float %169, 5.000000e-01, !dbg !2922
  %_0.i254.us.us.us.us.us.i = fadd float %_0.i276.us.us.us.us.us.i, %_0.i277.us.us.us.us.us.i, !dbg !2924
  %_6.i374.us.us.us.us.us.i = bitcast float %_0.i254.us.us.us.us.us.i to i32, !dbg !2926
  %_4.i379.us.us.us.us.us.i = select i1 %_3.i220.us.us.us.us.us.i, i32 %_4.i386.us.us.us.us.us.i, i32 %_6.i374.us.us.us.us.us.i, !dbg !2929
  %_0.i380.us.us.us.us.us.i = bitcast i32 %_4.i379.us.us.us.us.us.i to float, !dbg !2930
  %_3.i.i552.us.us.us.us.us.i = fcmp ule float %_0.i380.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !2933
  %_4.i.i558.us.us.us.us.us.i = select i1 %_3.i.i552.us.us.us.us.us.i, i32 841731191, i32 %_4.i379.us.us.us.us.us.i, !dbg !2936
  %_0.i.i559.us.us.us.us.us.i = bitcast i32 %_4.i.i558.us.us.us.us.us.i to float, !dbg !2938
  %_3.i.i512.us.us.us.us.us.i = fcmp ule float %_0.i.i559.us.us.us.us.us.i, 0x3810000000000000, !dbg !2940
  %_4.i.i518.us.us.us.us.us.i = select i1 %_3.i.i512.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i558.us.us.us.us.us.i, !dbg !2950
  %_5.i324.us.us.us.us.us.i = and i32 %_4.i.i518.us.us.us.us.us.i, 8388607, !dbg !2952
  %_4.i325.us.us.us.us.us.i = or disjoint i32 %_5.i324.us.us.us.us.us.i, 1065353216, !dbg !2952
  %significand.i326.us.us.us.us.us.i = bitcast i32 %_4.i325.us.us.us.us.us.i to float, !dbg !2957
  %_0.i285.us.us.us.us.us.i = fadd float %significand.i326.us.us.us.us.us.i, -1.000000e+00, !dbg !2960
  %_0.i265.us.us.us.us.us.i = fmul float %_0.i285.us.us.us.us.us.i, 0x3F9B17A960000000, !dbg !2963
  %170 = fsub float 0x3FBF9A8440000000, %_0.i265.us.us.us.us.us.i, !dbg !2968
  %_0.i265.us.us.us.us.us.1.i = fmul float %_0.i285.us.us.us.us.us.i, %170, !dbg !2963
  %_0.i249.us.us.us.us.us.1.i = fadd float %_0.i265.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !2968
  %_0.i265.us.us.us.us.us.2.i = fmul float %_0.i285.us.us.us.us.us.i, %_0.i249.us.us.us.us.us.1.i, !dbg !2963
  %_0.i249.us.us.us.us.us.2.i = fadd float %_0.i265.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !2968
  %_0.i265.us.us.us.us.us.3.i = fmul float %_0.i285.us.us.us.us.us.i, %_0.i249.us.us.us.us.us.2.i, !dbg !2963
  %_0.i249.us.us.us.us.us.3.i = fadd float %_0.i265.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !2968
  %_0.i265.us.us.us.us.us.4.i = fmul float %_0.i285.us.us.us.us.us.i, %_0.i249.us.us.us.us.us.3.i, !dbg !2963
  %_0.i249.us.us.us.us.us.4.i = fadd float %_0.i265.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !2968
  %_9.i327.us.us.us.us.us.i = lshr i32 %_4.i.i518.us.us.us.us.us.i, 23, !dbg !2970
  %_8.i328.us.us.us.us.us.i = or disjoint i32 %_9.i327.us.us.us.us.us.i, 1258291200, !dbg !2970
  %_7.i329.us.us.us.us.us.i = bitcast i32 %_8.i328.us.us.us.us.us.i to float, !dbg !2972
  %exponent.i330.us.us.us.us.us.i = fadd float %_7.i329.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !2974
  %_0.i264.us.us.us.us.us.i = fmul float %_0.i285.us.us.us.us.us.i, %_0.i249.us.us.us.us.us.4.i, !dbg !2975
  %_0.i248.us.us.us.us.us.i = fadd float %exponent.i330.us.us.us.us.us.i, %_0.i264.us.us.us.us.us.i, !dbg !2977
  %_0.i275.us.us.us.us.us.i = fmul float %_0.i248.us.us.us.us.us.i, 0x4018151820000000, !dbg !2979
  %_3.i.i627.us.us.us.us.us.inv.i = fcmp olt float %_0.i275.us.us.us.us.us.i, 2.400000e+01, !dbg !2981
  %_0.i.i634.us.us.us.us.us.i = select i1 %_3.i.i627.us.us.us.us.us.inv.i, float %_0.i275.us.us.us.us.us.i, float 2.400000e+01, !dbg !2981
  %_3.i.i544.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i634.us.us.us.us.us.i, -1.600000e+02, !dbg !2985
  %_0.i.i551.us.us.us.us.us.i = select i1 %_3.i.i544.us.us.us.us.us.inv.i, float %_0.i.i634.us.us.us.us.us.i, float -1.600000e+02, !dbg !2985
  %_55.i81.us.us.us.us.us.i = load float, ptr %12, align 4, !dbg !2988, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i218.us.us.us.us.us.i = fcmp ule float %_55.i81.us.us.us.us.us.i, 0.000000e+00, !dbg !2990
  %_3.i204.us.us.us.us.us.i = fcmp oge float %_0.i.i551.us.us.us.us.us.i, %_0.i407.us.us.us.us.us.i, !dbg !2992
  %_0.i292.us.us.us.us.us.i = fsub float %_0.i407.us.us.us.us.us.i, %_0.i407.us.us.us.us.us.3.i, !dbg !2996
  %_3.i202.us.us.us.us.us.i = fcmp oge float %_0.i.i551.us.us.us.us.us.i, %_0.i292.us.us.us.us.us.i, !dbg !2999
  %..i203.us.us.us.us.us.i = sext i1 %_3.i202.us.us.us.us.us.i to i32, !dbg !3001
  %_0.i502.us.us.us.us.us.i = sext i1 %_3.i204.us.us.us.us.us.i to i32, !dbg !3004
  %_0.i496.us.us.us.us.us.i = select i1 %_3.i218.us.us.us.us.us.i, i32 %_0.i502.us.us.us.us.us.i, i32 %..i203.us.us.us.us.us.i, !dbg !3004
  %_0.i508.us.us.us.us.us.i = xor i32 %..i203.us.us.us.us.us.i, -1, !dbg !3010
  %_67.i91.us.us.us.us.us.i = load float, ptr %15, align 4, !dbg !3014, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i216.us.us.us.us.us.i = fcmp ogt float %_67.i91.us.us.us.us.us.i, 0.000000e+00, !dbg !3015
  %_0.i501.us.us.us.us.us.i = select i1 %_3.i216.us.us.us.us.us.i, i32 %_0.i508.us.us.us.us.us.i, i32 0, !dbg !3017
  %_0.i500.us.us.us.us.us.i = select i1 %_3.i218.us.us.us.us.us.i, i32 0, i32 %_0.i501.us.us.us.us.us.i, !dbg !3020
  %_0.i495.us.us.us.us.us.i = or i32 %_0.i500.us.us.us.us.us.i, %_0.i496.us.us.us.us.us.i, !dbg !3022
  %_5.i369.us.us.us.us.us.i = and i32 %_0.i495.us.us.us.us.us.i, 1065353216, !dbg !3025
  %_0.i373.us.us.us.us.us.i = bitcast i32 %_5.i369.us.us.us.us.us.i to float, !dbg !3027
  %_71.i97667668.us.us.us.us.us.i = load float, ptr %16, align 4, !dbg !3029, !alias.scope !2897, !noalias !2898, !noundef !12
  %_0.i291.us.us.us.us.us.i = fadd float %_67.i91.us.us.us.us.us.i, -1.000000e+00, !dbg !3031
  %171 = trunc nsw i32 %_0.i500.us.us.us.us.us.i to i1, !dbg !3033
  %_4.i367.v.us.us.us.us.us.i = select i1 %171, float %_0.i291.us.us.us.us.us.i, float %_67.i91.us.us.us.us.us.i, !dbg !3033
  %172 = trunc nsw i32 %_0.i496.us.us.us.us.us.i to i1, !dbg !3035
  %_0.i361.us.us.us.us.us.i = select i1 %172, float %_71.i97667668.us.us.us.us.us.i, float %_4.i367.v.us.us.us.us.us.i, !dbg !3035
  store float %_0.i361.us.us.us.us.us.i, ptr %15, align 4, !dbg !3037, !alias.scope !2852, !noalias !2853
  store i32 %_5.i369.us.us.us.us.us.i, ptr %12, align 4, !dbg !3038, !alias.scope !2852, !noalias !2853
  %_0.i290.us.us.us.us.us.i = fadd float %_0.i407.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3039
  %_0.i289.us.us.us.us.us.i = fsub float %_0.i.i551.us.us.us.us.us.i, %_0.i407.us.us.us.us.us.i, !dbg !3041
  %_0.i274.us.us.us.us.us.i = fmul float %_0.i290.us.us.us.us.us.i, %_0.i289.us.us.us.us.us.i, !dbg !3043
  %173 = fneg float %_0.i407.us.us.us.us.us.2.i, !dbg !3045
  %_3.i.i536.inv.us.us.us.us.us.i = fcmp ogt float %_0.i274.us.us.us.us.us.i, %173, !dbg !3048
  %_4.i.i542.v.us.us.us.us.us.i = select i1 %_3.i.i536.inv.us.us.us.us.us.i, float %_0.i274.us.us.us.us.us.i, float %173, !dbg !3048
  %_3.i.i619.us.us.us.us.us.i = fcmp olt float %_4.i.i542.v.us.us.us.us.us.i, 0.000000e+00, !dbg !3051
  %174 = fcmp ule float %_0.i373.us.us.us.us.us.i, 0.000000e+00, !dbg !3055
  %175 = select i1 %174, i1 %_3.i.i619.us.us.us.us.us.i, i1 false, !dbg !3058
  %_0.i354.us.us.us.us.us.i = select i1 %175, float %_4.i.i542.v.us.us.us.us.us.i, float 0.000000e+00, !dbg !3058
  %_86.i110.us.us.us.us.us.i = load float, ptr %17, align 4, !dbg !3059, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i212.us.us.us.us.us.i = fcmp ule float %_0.i354.us.us.us.us.us.i, %_86.i110.us.us.us.us.us.i, !dbg !3061
  %_87.i112670.us.us.us.us.us.i = load i32, ptr %2, align 4, !dbg !3063, !alias.scope !2897, !noalias !2898, !noundef !12
  %_88.i113671.us.us.us.us.us.i = load i32, ptr %18, align 4, !dbg !3064, !alias.scope !2897, !noalias !2898, !noundef !12
  %_4.i347.us.us.us.us.us.i = select i1 %_3.i212.us.us.us.us.us.i, i32 %_88.i113671.us.us.us.us.us.i, i32 %_87.i112670.us.us.us.us.us.i, !dbg !3065
  %_0.i348.us.us.us.us.us.i = bitcast i32 %_4.i347.us.us.us.us.us.i to float, !dbg !3067
  %_0.i288.us.us.us.us.us.i = fsub float %_0.i354.us.us.us.us.us.i, %_86.i110.us.us.us.us.us.i, !dbg !3069
  %_4.i258.us.us.us.us.us.i = fmul float %_0.i288.us.us.us.us.us.i, %_0.i348.us.us.us.us.us.i, !dbg !3072
  %_0.i259.us.us.us.us.us.i = fadd float %_86.i110.us.us.us.us.us.i, %_4.i258.us.us.us.us.us.i, !dbg !3072
  %176 = tail call noundef float @llvm.fabs.f32(float %_0.i259.us.us.us.us.us.i), !dbg !3075
  %177 = fcmp uge float %176, 0x3BC79CA100000000, !dbg !3079
  %_0.i332.us.us.us.us.us.i = select i1 %177, float %_0.i259.us.us.us.us.us.i, float 0.000000e+00, !dbg !3082
  store float %_0.i332.us.us.us.us.us.i, ptr %17, align 4, !dbg !3083, !alias.scope !2852, !noalias !2853
  %_0.i273.us.us.us.us.us.i = fmul float %_0.i332.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !3085
  %_3.i.i528.us.us.us.us.us.inv.i = fcmp ogt float %_0.i273.us.us.us.us.us.i, -1.260000e+02, !dbg !3089
  %_0.i.i535.us.us.us.us.us.i = select i1 %_3.i.i528.us.us.us.us.us.inv.i, float %_0.i273.us.us.us.us.us.i, float -1.260000e+02, !dbg !3089
  %_3.i.i611.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i535.us.us.us.us.us.i, 1.270000e+02, !dbg !3094
  %_0.i.i618.us.us.us.us.us.i = select i1 %_3.i.i611.us.us.us.us.us.inv.i, float %_0.i.i535.us.us.us.us.us.i, float 1.270000e+02, !dbg !3094
  %178 = tail call noundef float @llvm.floor.f32(float %_0.i.i618.us.us.us.us.us.i), !dbg !3097
  %_0.i287.us.us.us.us.us.i = fsub float %_0.i.i618.us.us.us.us.us.i, %178, !dbg !3109
  %_98.i126.us.us.us.us.us.i = load float, ptr %19, align 4, !dbg !3112, !alias.scope !2897, !noalias !2898, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3114), !dbg !3117
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3119), !dbg !3117
  %_12.i.us.us.us.us.us.i = load float, ptr %112, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.us.us.i = fcmp ule float %_12.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.us.us.i = fcmp une float %_12.i.us.us.us.us.us.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.us.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.us.us.i = load float, ptr %113, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.us.us.i = fadd float %_16.i.us.us.us.us.us.i, %_17.i.us.us.us.us.us.i, !dbg !3131
  %_20.i677680.us.us.us.us.us.i = load float, ptr %114, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %179 = select i1 %_3.i200.us.us.us.us.us.i, float %_0.i257.us.us.us.us.us.i, float %_20.i677680.us.us.us.us.us.i, !dbg !3134
  %_0.i486.us.us.us.us.us.i = select i1 %_3.i240.us.us.us.us.us.i, float %_16.i.us.us.us.us.us.i, float %179, !dbg !3136
  store float %_0.i486.us.us.us.us.us.i, ptr %data.i.i.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.us.us.i = select i1 %_3.i200.us.us.us.us.us.i, float %_17.i.us.us.us.us.us.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.us.us.i, ptr %113, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.us.us.i = fadd float %_12.i.us.us.us.us.us.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.us.us.i = select i1 %_3.i240.us.us.us.us.us.i, float %_12.i.us.us.us.us.us.i, float %_0.i299.us.us.us.us.us.i, !dbg !3144
  store float %_4.i472.v.us.us.us.us.us.i, ptr %112, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.us.us.1.i = load float, ptr %115, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.us.us.1.i = fcmp ule float %_12.i.us.us.us.us.us.1.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.us.us.1.i = fcmp une float %_12.i.us.us.us.us.us.1.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.1.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.us.us.1.i = load float, ptr %116, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.us.us.1.i = fadd float %_16.i.us.us.us.us.us.1.i, %_17.i.us.us.us.us.us.1.i, !dbg !3131
  %_20.i677680.us.us.us.us.us.1.i = load float, ptr %117, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %180 = select i1 %_3.i200.us.us.us.us.us.1.i, float %_0.i257.us.us.us.us.us.1.i, float %_20.i677680.us.us.us.us.us.1.i, !dbg !3134
  %_0.i486.us.us.us.us.us.1.i = select i1 %_3.i240.us.us.us.us.us.1.i, float %_16.i.us.us.us.us.us.1.i, float %180, !dbg !3136
  store float %_0.i486.us.us.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.1.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.us.us.1.i = select i1 %_3.i200.us.us.us.us.us.1.i, float %_17.i.us.us.us.us.us.1.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.us.us.1.i, ptr %116, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.us.us.1.i = fadd float %_12.i.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.us.us.1.i = select i1 %_3.i240.us.us.us.us.us.1.i, float %_12.i.us.us.us.us.us.1.i, float %_0.i299.us.us.us.us.us.1.i, !dbg !3144
  store float %_4.i472.v.us.us.us.us.us.1.i, ptr %115, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.us.us.2.i = load float, ptr %118, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.us.us.2.i = fcmp ule float %_12.i.us.us.us.us.us.2.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.us.us.2.i = fcmp une float %_12.i.us.us.us.us.us.2.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.2.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.us.us.2.i = load float, ptr %119, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.us.us.2.i = fadd float %_16.i.us.us.us.us.us.2.i, %_17.i.us.us.us.us.us.2.i, !dbg !3131
  %_20.i677680.us.us.us.us.us.2.i = load float, ptr %120, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %181 = select i1 %_3.i200.us.us.us.us.us.2.i, float %_0.i257.us.us.us.us.us.2.i, float %_20.i677680.us.us.us.us.us.2.i, !dbg !3134
  %_0.i486.us.us.us.us.us.2.i = select i1 %_3.i240.us.us.us.us.us.2.i, float %_16.i.us.us.us.us.us.2.i, float %181, !dbg !3136
  store float %_0.i486.us.us.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.2.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.us.us.2.i = select i1 %_3.i200.us.us.us.us.us.2.i, float %_17.i.us.us.us.us.us.2.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.us.us.2.i, ptr %119, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.us.us.2.i = fadd float %_12.i.us.us.us.us.us.2.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.us.us.2.i = select i1 %_3.i240.us.us.us.us.us.2.i, float %_12.i.us.us.us.us.us.2.i, float %_0.i299.us.us.us.us.us.2.i, !dbg !3144
  store float %_4.i472.v.us.us.us.us.us.2.i, ptr %118, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.us.us.3.i = load float, ptr %121, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.us.us.3.i = fcmp ule float %_12.i.us.us.us.us.us.3.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.us.us.3.i = fcmp une float %_12.i.us.us.us.us.us.3.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.3.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.us.us.3.i = load float, ptr %122, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.us.us.3.i = fadd float %_16.i.us.us.us.us.us.3.i, %_17.i.us.us.us.us.us.3.i, !dbg !3131
  %_20.i677680.us.us.us.us.us.3.i = load float, ptr %123, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %182 = select i1 %_3.i200.us.us.us.us.us.3.i, float %_0.i257.us.us.us.us.us.3.i, float %_20.i677680.us.us.us.us.us.3.i, !dbg !3134
  %_0.i486.us.us.us.us.us.3.i = select i1 %_3.i240.us.us.us.us.us.3.i, float %_16.i.us.us.us.us.us.3.i, float %182, !dbg !3136
  store float %_0.i486.us.us.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.us.us.3.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.us.us.3.i = select i1 %_3.i200.us.us.us.us.us.3.i, float %_17.i.us.us.us.us.us.3.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.us.us.3.i, ptr %122, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.us.us.3.i = fadd float %_12.i.us.us.us.us.us.3.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.us.us.3.i = select i1 %_3.i240.us.us.us.us.us.3.i, float %_12.i.us.us.us.us.us.3.i, float %_0.i299.us.us.us.us.us.3.i, !dbg !3144
  store float %_4.i472.v.us.us.us.us.us.3.i, ptr %121, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %183 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le931.us.us.us.us.us.i), !dbg !3147
  %184 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.us.us.i), !dbg !3149
  %_37.i.us.us.us.us.us.i = load float, ptr %21, align 4, !dbg !3151, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i238.us.us.us.us.us.i = fcmp ule float %_37.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3154
  %_3.i.i594.us.us.us.us.us.i = fcmp ule float %183, %184, !dbg !3156
  %_6.i.i596.us.us.us.us.us.i = bitcast float %183 to i32, !dbg !3159
  %_8.i.i598.us.us.us.us.us.i = bitcast float %184 to i32, !dbg !3162
  %_4.i.i601.us.us.us.us.us.i = select i1 %_3.i.i594.us.us.us.us.us.i, i32 %_8.i.i598.us.us.us.us.us.i, i32 %_6.i.i596.us.us.us.us.us.i, !dbg !3164
  %_4.i465.us.us.us.us.us.i = select i1 %_3.i238.us.us.us.us.us.i, i32 %_6.i.i596.us.us.us.us.us.i, i32 %_4.i.i601.us.us.us.us.us.i, !dbg !3165
  %_41.i.us.us.us.us.us.i = load float, ptr %22, align 4, !dbg !3167, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i236.us.us.us.us.us.i = fcmp ule float %_41.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3168
  %_0.i283.us.us.us.us.us.i = fmul float %183, 5.000000e-01, !dbg !3170
  %_0.i282.us.us.us.us.us.i = fmul float %184, 5.000000e-01, !dbg !3172
  %_0.i256.us.us.us.us.us.i = fadd float %_0.i282.us.us.us.us.us.i, %_0.i283.us.us.us.us.us.i, !dbg !3174
  %_6.i453.us.us.us.us.us.i = bitcast float %_0.i256.us.us.us.us.us.i to i32, !dbg !3176
  %_4.i458.us.us.us.us.us.i = select i1 %_3.i236.us.us.us.us.us.i, i32 %_4.i465.us.us.us.us.us.i, i32 %_6.i453.us.us.us.us.us.i, !dbg !3179
  %_0.i459.us.us.us.us.us.i = bitcast i32 %_4.i458.us.us.us.us.us.i to float, !dbg !3180
  %_3.i.i586.us.us.us.us.us.i = fcmp ule float %_0.i459.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !3182
  %_4.i.i592.us.us.us.us.us.i = select i1 %_3.i.i586.us.us.us.us.us.i, i32 841731191, i32 %_4.i458.us.us.us.us.us.i, !dbg !3185
  %_0.i.i593.us.us.us.us.us.i = bitcast i32 %_4.i.i592.us.us.us.us.us.i to float, !dbg !3187
  %_3.i.i.us.us.us.us.us.i = fcmp ule float %_0.i.i593.us.us.us.us.us.i, 0x3810000000000000, !dbg !3189
  %_4.i.i.us.us.us.us.us.i = select i1 %_3.i.i.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i592.us.us.us.us.us.i, !dbg !3194
  %_5.i320.us.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.us.i, 8388607, !dbg !3196
  %_4.i321.us.us.us.us.us.i = or disjoint i32 %_5.i320.us.us.us.us.us.i, 1065353216, !dbg !3196
  %significand.i.us.us.us.us.us.i = bitcast i32 %_4.i321.us.us.us.us.us.i to float, !dbg !3198
  %_0.i284.us.us.us.us.us.i = fadd float %significand.i.us.us.us.us.us.i, -1.000000e+00, !dbg !3200
  %_0.i263.us.us.us.us.us.i = fmul float %_0.i284.us.us.us.us.us.i, 0x3F9B17A960000000, !dbg !3202
  %185 = fsub float 0x3FBF9A8440000000, %_0.i263.us.us.us.us.us.i, !dbg !3204
  %_0.i263.us.us.us.us.us.1.i = fmul float %_0.i284.us.us.us.us.us.i, %185, !dbg !3202
  %_0.i247.us.us.us.us.us.1.i = fadd float %_0.i263.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3204
  %_0.i263.us.us.us.us.us.2.i = fmul float %_0.i284.us.us.us.us.us.i, %_0.i247.us.us.us.us.us.1.i, !dbg !3202
  %_0.i247.us.us.us.us.us.2.i = fadd float %_0.i263.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3204
  %_0.i263.us.us.us.us.us.3.i = fmul float %_0.i284.us.us.us.us.us.i, %_0.i247.us.us.us.us.us.2.i, !dbg !3202
  %_0.i247.us.us.us.us.us.3.i = fadd float %_0.i263.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3204
  %_0.i263.us.us.us.us.us.4.i = fmul float %_0.i284.us.us.us.us.us.i, %_0.i247.us.us.us.us.us.3.i, !dbg !3202
  %_0.i247.us.us.us.us.us.4.i = fadd float %_0.i263.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3204
  %_9.i.us.us.us.us.us.i = lshr i32 %_4.i.i.us.us.us.us.us.i, 23, !dbg !3206
  %_8.i322.us.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.us.i, 1258291200, !dbg !3206
  %_7.i.us.us.us.us.us.i = bitcast i32 %_8.i322.us.us.us.us.us.i to float, !dbg !3207
  %exponent.i.us.us.us.us.us.i = fadd float %_7.i.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !3209
  %_0.i262.us.us.us.us.us.i = fmul float %_0.i284.us.us.us.us.us.i, %_0.i247.us.us.us.us.us.4.i, !dbg !3210
  %_0.i246.us.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.us.i, %_0.i262.us.us.us.us.us.i, !dbg !3212
  %_0.i281.us.us.us.us.us.i = fmul float %_0.i246.us.us.us.us.us.i, 0x4018151820000000, !dbg !3214
  %_3.i.i643.us.us.us.us.us.inv.i = fcmp olt float %_0.i281.us.us.us.us.us.i, 2.400000e+01, !dbg !3216
  %_0.i.i650.us.us.us.us.us.i = select i1 %_3.i.i643.us.us.us.us.us.inv.i, float %_0.i281.us.us.us.us.us.i, float 2.400000e+01, !dbg !3216
  %_3.i.i578.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i650.us.us.us.us.us.i, -1.600000e+02, !dbg !3219
  %_0.i.i585.us.us.us.us.us.i = select i1 %_3.i.i578.us.us.us.us.us.inv.i, float %_0.i.i650.us.us.us.us.us.i, float -1.600000e+02, !dbg !3219
  %_55.i12.us.us.us.us.us.i = load float, ptr %20, align 4, !dbg !3222, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i234.us.us.us.us.us.i = fcmp ule float %_55.i12.us.us.us.us.us.i, 0.000000e+00, !dbg !3223
  %_3.i208.us.us.us.us.us.i = fcmp oge float %_0.i.i585.us.us.us.us.us.i, %_0.i486.us.us.us.us.us.i, !dbg !3225
  %_0.i298.us.us.us.us.us.i = fsub float %_0.i486.us.us.us.us.us.i, %_0.i486.us.us.us.us.us.3.i, !dbg !3227
  %_3.i206.us.us.us.us.us.i = fcmp oge float %_0.i.i585.us.us.us.us.us.i, %_0.i298.us.us.us.us.us.i, !dbg !3229
  %..i207.us.us.us.us.us.i = sext i1 %_3.i206.us.us.us.us.us.i to i32, !dbg !3231
  %_0.i506.us.us.us.us.us.i = sext i1 %_3.i208.us.us.us.us.us.i to i32, !dbg !3233
  %_0.i499.us.us.us.us.us.i = select i1 %_3.i234.us.us.us.us.us.i, i32 %_0.i506.us.us.us.us.us.i, i32 %..i207.us.us.us.us.us.i, !dbg !3233
  %_0.i510.us.us.us.us.us.i = xor i32 %..i207.us.us.us.us.us.i, -1, !dbg !3235
  %_67.i14.us.us.us.us.us.i = load float, ptr %23, align 4, !dbg !3237, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i232.us.us.us.us.us.i = fcmp ogt float %_67.i14.us.us.us.us.us.i, 0.000000e+00, !dbg !3238
  %_0.i505.us.us.us.us.us.i = select i1 %_3.i232.us.us.us.us.us.i, i32 %_0.i510.us.us.us.us.us.i, i32 0, !dbg !3240
  %_0.i504.us.us.us.us.us.i = select i1 %_3.i234.us.us.us.us.us.i, i32 0, i32 %_0.i505.us.us.us.us.us.i, !dbg !3242
  %_0.i498.us.us.us.us.us.i = or i32 %_0.i504.us.us.us.us.us.i, %_0.i499.us.us.us.us.us.i, !dbg !3244
  %_5.i448.us.us.us.us.us.i = and i32 %_0.i498.us.us.us.us.us.i, 1065353216, !dbg !3246
  %_0.i452.us.us.us.us.us.i = bitcast i32 %_5.i448.us.us.us.us.us.i to float, !dbg !3248
  %_71.i689690.us.us.us.us.us.i = load float, ptr %24, align 4, !dbg !3250, !alias.scope !3152, !noalias !3153, !noundef !12
  %_0.i297.us.us.us.us.us.i = fadd float %_67.i14.us.us.us.us.us.i, -1.000000e+00, !dbg !3251
  %186 = trunc nsw i32 %_0.i504.us.us.us.us.us.i to i1, !dbg !3253
  %_4.i446.v.us.us.us.us.us.i = select i1 %186, float %_0.i297.us.us.us.us.us.i, float %_67.i14.us.us.us.us.us.i, !dbg !3253
  %187 = trunc nsw i32 %_0.i499.us.us.us.us.us.i to i1, !dbg !3255
  %_0.i440.us.us.us.us.us.i = select i1 %187, float %_71.i689690.us.us.us.us.us.i, float %_4.i446.v.us.us.us.us.us.i, !dbg !3255
  store float %_0.i440.us.us.us.us.us.i, ptr %23, align 4, !dbg !3257, !alias.scope !3123, !noalias !3124
  store i32 %_5.i448.us.us.us.us.us.i, ptr %20, align 4, !dbg !3258, !alias.scope !3123, !noalias !3124
  %_0.i296.us.us.us.us.us.i = fadd float %_0.i486.us.us.us.us.us.1.i, -1.000000e+00, !dbg !3259
  %_0.i295.us.us.us.us.us.i = fsub float %_0.i.i585.us.us.us.us.us.i, %_0.i486.us.us.us.us.us.i, !dbg !3261
  %_0.i280.us.us.us.us.us.i = fmul float %_0.i296.us.us.us.us.us.i, %_0.i295.us.us.us.us.us.i, !dbg !3263
  %188 = fneg float %_0.i486.us.us.us.us.us.2.i, !dbg !3265
  %_3.i.i569.inv.us.us.us.us.us.i = fcmp ogt float %_0.i280.us.us.us.us.us.i, %188, !dbg !3267
  %_4.i.i576.v.us.us.us.us.us.i = select i1 %_3.i.i569.inv.us.us.us.us.us.i, float %_0.i280.us.us.us.us.us.i, float %188, !dbg !3267
  %_3.i.i635.us.us.us.us.us.i = fcmp olt float %_4.i.i576.v.us.us.us.us.us.i, 0.000000e+00, !dbg !3270
  %189 = fcmp ule float %_0.i452.us.us.us.us.us.i, 0.000000e+00, !dbg !3273
  %190 = select i1 %189, i1 %_3.i.i635.us.us.us.us.us.i, i1 false, !dbg !3275
  %_0.i433.us.us.us.us.us.i = select i1 %190, float %_4.i.i576.v.us.us.us.us.us.i, float 0.000000e+00, !dbg !3275
  %_86.i22.us.us.us.us.us.i = load float, ptr %25, align 4, !dbg !3276, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i228.us.us.us.us.us.i = fcmp ule float %_0.i433.us.us.us.us.us.i, %_86.i22.us.us.us.us.us.i, !dbg !3277
  %_87.i24692.us.us.us.us.us.i = load i32, ptr %_32.i, align 4, !dbg !3279, !alias.scope !3152, !noalias !3153, !noundef !12
  %_88.i25693.us.us.us.us.us.i = load i32, ptr %26, align 4, !dbg !3280, !alias.scope !3152, !noalias !3153, !noundef !12
  %_4.i427.us.us.us.us.us.i = select i1 %_3.i228.us.us.us.us.us.i, i32 %_88.i25693.us.us.us.us.us.i, i32 %_87.i24692.us.us.us.us.us.i, !dbg !3281
  %_0.i.us.us.us.us.us.i = bitcast i32 %_4.i427.us.us.us.us.us.i to float, !dbg !3283
  %_0.i294.us.us.us.us.us.i = fsub float %_0.i433.us.us.us.us.us.i, %_86.i22.us.us.us.us.us.i, !dbg !3285
  %_4.i260.us.us.us.us.us.i = fmul float %_0.i294.us.us.us.us.us.i, %_0.i.us.us.us.us.us.i, !dbg !3287
  %_0.i261.us.us.us.us.us.i = fadd float %_86.i22.us.us.us.us.us.i, %_4.i260.us.us.us.us.us.i, !dbg !3287
  %191 = tail call noundef float @llvm.fabs.f32(float %_0.i261.us.us.us.us.us.i), !dbg !3289
  %192 = fcmp uge float %191, 0x3BC79CA100000000, !dbg !3292
  %_0.i336.us.us.us.us.us.i = select i1 %192, float %_0.i261.us.us.us.us.us.i, float 0.000000e+00, !dbg !3294
  store float %_0.i336.us.us.us.us.us.i, ptr %25, align 4, !dbg !3295, !alias.scope !3123, !noalias !3124
  %_0.i279.us.us.us.us.us.i = fmul float %_0.i336.us.us.us.us.us.i, 0x3FC542A5A0000000, !dbg !3296
  %_3.i.i520.us.us.us.us.us.inv.i = fcmp ogt float %_0.i279.us.us.us.us.us.i, -1.260000e+02, !dbg !3299
  %_0.i.i527.us.us.us.us.us.i = select i1 %_3.i.i520.us.us.us.us.us.inv.i, float %_0.i279.us.us.us.us.us.i, float -1.260000e+02, !dbg !3299
  %_3.i.i603.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i527.us.us.us.us.us.i, 1.270000e+02, !dbg !3303
  %_0.i.i610.us.us.us.us.us.i = select i1 %_3.i.i603.us.us.us.us.us.inv.i, float %_0.i.i527.us.us.us.us.us.i, float 1.270000e+02, !dbg !3303
  %193 = tail call noundef float @llvm.floor.f32(float %_0.i.i610.us.us.us.us.us.i), !dbg !3306
  %_0.i286.us.us.us.us.us.i = fsub float %_0.i.i610.us.us.us.us.us.i, %193, !dbg !3310
  %_0.i268.us.us.us.us.us.i = fmul float %_0.i286.us.us.us.us.us.i, 0x3F5E974FA0000000, !dbg !3312
  %_0.i251.us.us.us.us.us.i = fadd float %_0.i268.us.us.us.us.us.i, 0x3F82778560000000, !dbg !3317
  %_0.i268.us.us.us.us.us.1.i = fmul float %_0.i286.us.us.us.us.us.i, %_0.i251.us.us.us.us.us.i, !dbg !3312
  %_0.i251.us.us.us.us.us.1.i = fadd float %_0.i268.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3317
  %_0.i268.us.us.us.us.us.2.i = fmul float %_0.i286.us.us.us.us.us.i, %_0.i251.us.us.us.us.us.1.i, !dbg !3312
  %_0.i251.us.us.us.us.us.2.i = fadd float %_0.i268.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3317
  %_0.i268.us.us.us.us.us.3.i = fmul float %_0.i286.us.us.us.us.us.i, %_0.i251.us.us.us.us.us.2.i, !dbg !3312
  %_0.i251.us.us.us.us.us.3.i = fadd float %_0.i268.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3317
  %_0.i271.us.us.us.us.us.i = fmul float %_0.i287.us.us.us.us.us.i, 0x3F5E974FA0000000, !dbg !3319
  %_0.i253.us.us.us.us.us.i = fadd float %_0.i271.us.us.us.us.us.i, 0x3F82778560000000, !dbg !3321
  %_0.i271.us.us.us.us.us.1.i = fmul float %_0.i287.us.us.us.us.us.i, %_0.i253.us.us.us.us.us.i, !dbg !3319
  %_0.i253.us.us.us.us.us.1.i = fadd float %_0.i271.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3321
  %_0.i271.us.us.us.us.us.2.i = fmul float %_0.i287.us.us.us.us.us.i, %_0.i253.us.us.us.us.us.1.i, !dbg !3319
  %_0.i253.us.us.us.us.us.2.i = fadd float %_0.i271.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3321
  %_0.i271.us.us.us.us.us.3.i = fmul float %_0.i287.us.us.us.us.us.i, %_0.i253.us.us.us.us.us.2.i, !dbg !3319
  %_0.i253.us.us.us.us.us.3.i = fadd float %_0.i271.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3321
  %_0.i270.us.us.us.us.us.i = fmul float %_0.i287.us.us.us.us.us.i, %_0.i253.us.us.us.us.us.3.i, !dbg !3323
  %_0.i252.us.us.us.us.us.i = fadd float %_0.i270.us.us.us.us.us.i, 1.000000e+00, !dbg !3325
  %biased.i191.us.us.us.us.us.i = fadd float %178, 0x4160000FE0000000, !dbg !3327
  %_4.i192.us.us.us.us.us.i = bitcast float %biased.i191.us.us.us.us.us.i to i32, !dbg !3331
  %_3.i193.us.us.us.us.us.i = shl i32 %_4.i192.us.us.us.us.us.i, 23, !dbg !3335
  %_0.i194.us.us.us.us.us.i = bitcast i32 %_3.i193.us.us.us.us.us.i to float, !dbg !3336
  %_0.i269.us.us.us.us.us.i = fmul float %_0.i252.us.us.us.us.us.i, %_0.i194.us.us.us.us.us.i, !dbg !3339
  %_3.i195.us.us.us.us.us.i = fcmp une float %_0.i332.us.us.us.us.us.i, 0.000000e+00, !dbg !3341
  %_3.i210.us.us.us.us.us.i = fcmp ule float %_98.i126.us.us.us.us.us.i, 0.000000e+00, !dbg !3343
  %_0.i494675.not.us.us.us.us.us.i = and i1 %_3.i210.us.us.us.us.us.i, %_3.i195.us.us.us.us.us.i, !dbg !3345
  %_0.i272.us.us.us.us.us.i = fmul float %_0.i311.us.us.us.us.us.i, %_0.i269.us.us.us.us.us.i, !dbg !3345
  %_4.i340.v.us.us.us.us.us.i = select i1 %_0.i494675.not.us.us.us.us.us.i, float %_0.i272.us.us.us.us.us.i, float %_0.i311.us.us.us.us.us.i, !dbg !3348
  %_0.i267.us.us.us.us.us.i = fmul float %_0.i286.us.us.us.us.us.i, %_0.i251.us.us.us.us.us.3.i, !dbg !3350
  %_0.i250.us.us.us.us.us.i = fadd float %_0.i267.us.us.us.us.us.i, 1.000000e+00, !dbg !3352
  %biased.i.us.us.us.us.us.i = fadd float %193, 0x4160000FE0000000, !dbg !3354
  %_4.i188.us.us.us.us.us.i = bitcast float %biased.i.us.us.us.us.us.i to i32, !dbg !3356
  %_3.i189.us.us.us.us.us.i = shl i32 %_4.i188.us.us.us.us.us.i, 23, !dbg !3358
  %_0.i190.us.us.us.us.us.i = bitcast i32 %_3.i189.us.us.us.us.us.i to float, !dbg !3359
  %_0.i266.us.us.us.us.us.i = fmul float %_0.i250.us.us.us.us.us.i, %_0.i190.us.us.us.us.us.i, !dbg !3361
  %_3.i198.us.us.us.us.us.i = fcmp une float %_0.i336.us.us.us.us.us.i, 0.000000e+00, !dbg !3363
  %_98.i.us.us.us.us.us.i = load float, ptr %27, align 4, !dbg !3365, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i226.us.us.us.us.us.i = fcmp ule float %_98.i.us.us.us.us.us.i, 0.000000e+00, !dbg !3366
  %_0.i497697.not.us.us.us.us.us.i = and i1 %_3.i226.us.us.us.us.us.i, %_3.i198.us.us.us.us.us.i, !dbg !3368
  %_0.i278.us.us.us.us.us.i = fmul float %_0.i309.us.us.us.us.us.i, %_0.i266.us.us.us.us.us.i, !dbg !3368
  %_4.i420.v.us.us.us.us.us.i = select i1 %_0.i497697.not.us.us.us.us.us.i, float %_0.i278.us.us.us.us.us.i, float %_0.i309.us.us.us.us.us.i, !dbg !3370
  store float %_4.i340.v.us.us.us.us.us.i, ptr %_123.i.us.us.us.us.us.i, align 4, !dbg !3372, !alias.scope !3375, !noalias !2714
  store float %_4.i420.v.us.us.us.us.us.i, ptr %_141.i.us.us.us.us.us.i, align 4, !dbg !3378, !alias.scope !3380, !noalias !2737
  %exitcond2625.not.i = icmp eq i64 %159, %..i, !dbg !3383
  br i1 %exitcond2625.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb40.i.us.us.us.us.us.i, !dbg !3386, !llvm.loop !3388

bb40.i.us.us.us.i:                                ; preds = %bb28.i.us.us.lr.ph.us.us.us.i, %bb40.i.us.us.us.preheader.i
  %iter.sroa.0.0.i1071.us.us.us.i = phi i64 [ %194, %bb28.i.us.us.lr.ph.us.us.us.i ], [ 0, %bb40.i.us.us.us.preheader.i ]
  %194 = add nuw nsw i64 %iter.sroa.0.0.i1071.us.us.us.i, 1, !dbg !2678
  %_27.i.us.us.us.i = trunc i64 %iter.sroa.0.0.i1071.us.us.us.i to i32, !dbg !2689
  %now.i.us.us.us.i = add i32 %base.i.i, %_27.i.us.us.us.i, !dbg !2690
  %_30.i.us.us.us.i = and i32 %now.i.us.us.us.i, %_52.i, !dbg !2693
  %_29.i.us.us.us.i = zext i32 %_30.i.us.us.us.i to i64, !dbg !2694
  %exitcond2628.not.i = icmp eq i64 %iter.sroa.0.0.i1071.us.us.us.i, %..i, !dbg !2666
  br i1 %exitcond2628.not.i, label %bb43.i.i, label %bb42.i.us.us.us.i, !dbg !2666, !prof !180

bb42.i.us.us.us.i:                                ; preds = %bb40.i.us.us.us.i
  %_123.i.us.us.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i1071.us.us.us.i, !dbg !2695
  %_124.not.not.i.us.us.us.i = icmp ugt i64 %_58.1.i, %_29.i.us.us.us.i, !dbg !2699
  br i1 %_124.not.not.i.us.us.us.i, label %bb45.i.us.us.us.i, label %bb46.i.i, !dbg !2699, !prof !2704

bb45.i.us.us.us.i:                                ; preds = %bb42.i.us.us.us.i
  %_0.i319.us.us.us.i = load float, ptr %_123.i.us.us.us.i, align 4, !dbg !2705, !alias.scope !2711, !noalias !2714, !noundef !12
  %_133.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_29.i.us.us.us.i, !dbg !2715
  store float %_0.i319.us.us.us.i, ptr %_133.i.us.us.us.i, align 4, !dbg !2719, !alias.scope !2722, !noalias !2663
  %_141.i.us.us.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i1071.us.us.us.i, !dbg !2725
  %_0.i317.us.us.us.i = load float, ptr %_141.i.us.us.us.i, align 4, !dbg !2732, !alias.scope !2734, !noalias !2737, !noundef !12
  %_149.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_29.i.us.us.us.i, !dbg !2738
  store float %_0.i317.us.us.us.i, ptr %_149.i.us.us.us.i, align 4, !dbg !2745, !alias.scope !2747, !noalias !2663
  %_158.not.not.i.us.us.us.i = icmp ugt i64 %_59.1.i, %_29.i.us.us.us.i, !dbg !3390
  br i1 %_158.not.not.i.us.us.us.i, label %bb54.i.us.us.us.i, label %bb55.i.i, !dbg !3390, !prof !2704

bb54.i.us.us.us.i:                                ; preds = %bb45.i.us.us.us.i
  %_157.i.us.us.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.us.us.i, !dbg !2750
  %_0.i315.us.us.us.i = load float, ptr %_157.i.us.us.us.i, align 4, !dbg !2757, !alias.scope !2759, !noalias !2663, !noundef !12
  %_165.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_29.i.us.us.us.i, !dbg !2762
  store float %_0.i315.us.us.us.i, ptr %_165.i.us.us.us.i, align 4, !dbg !2769, !alias.scope !2771, !noalias !2663
  %_174.not.not.i.us.us.us.i = icmp ugt i64 %_61.1.i, %_29.i.us.us.us.i, !dbg !3387
  br i1 %_174.not.not.i.us.us.us.i, label %bb58.i.us.us.us.i, label %bb59.i.i, !dbg !3387, !prof !2704

bb58.i.us.us.us.i:                                ; preds = %bb54.i.us.us.us.i
  %_173.i.us.us.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.us.us.i, !dbg !2774
  %_0.i313.us.us.us.i = load float, ptr %_173.i.us.us.us.i, align 4, !dbg !2781, !alias.scope !2783, !noalias !2663, !noundef !12
  %_181.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_29.i.us.us.us.i, !dbg !2786
  store float %_0.i313.us.us.us.i, ptr %_181.i.us.us.us.i, align 4, !dbg !2793, !alias.scope !2795, !noalias !2663
  %_52.i.us.us.us.i = sub i32 %now.i.us.us.us.i, %_53.i, !dbg !2798
  %_51.i.us.us.us.i = and i32 %_52.i.us.us.us.i, %_52.i, !dbg !2801
  %_50.i.us.us.us.i = zext i32 %_51.i.us.us.us.i to i64, !dbg !2802
  %_182.not.not.i.us.us.us.i = icmp ugt i64 %_58.1.i, %_50.i.us.us.us.i, !dbg !2803
  br i1 %_182.not.not.i.us.us.us.i, label %bb60.i.us.us.us.i, label %bb61.i.i, !dbg !2803, !prof !2704

bb60.i.us.us.us.i:                                ; preds = %bb58.i.us.us.us.i
  %_189.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_50.i.us.us.us.i, !dbg !2808
  %_0.i311.us.us.us.i = load float, ptr %_189.i.us.us.us.i, align 4, !dbg !2812, !alias.scope !2814, !noalias !2663, !noundef !12
  %_195.i.us.us.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_50.i.us.us.us.i, !dbg !2817
  %_0.i309.us.us.us.i = load float, ptr %_195.i.us.us.us.i, align 4, !dbg !2825, !alias.scope !2827, !noalias !2663, !noundef !12
  %_67.i.us.us.us.i = load i32, ptr %_45.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_66.i.us.us.us.i = sub i32 %now.i.us.us.us.i, %_67.i.us.us.us.i
  %_65.i.us.us.us.i = and i32 %_66.i.us.us.us.i, %_52.i
  %_64.i.us.us.us.i = zext i32 %_65.i.us.us.us.i to i64
  %_75.i.us.us.us.i = load i32, ptr %_50.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_74.i.us.us.us.i = sub i32 %now.i.us.us.us.i, %_75.i.us.us.us.i
  %_73.i.us.us.us.i = and i32 %_74.i.us.us.us.i, %_52.i
  %_72.i.us.us.us.i = zext i32 %_73.i.us.us.us.i to i64
  %_80.i.us.us.us.i = icmp ugt i64 %_59.1.i, %_64.i.us.us.us.i
  %195 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_64.i.us.us.us.i
  %196 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_64.i.us.us.us.i
  %_86.i.us.us.us.i = icmp ugt i64 %_61.1.i, %_72.i.us.us.us.i
  %197 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_72.i.us.us.us.i
  %_88.i.us.us.us.i = icmp ugt i64 %_59.1.i, %_72.i.us.us.us.i
  %198 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_72.i.us.us.us.i
  br i1 %_80.i.us.us.us.i, label %bb63.i.split.us.us.us.us.i, label %bb63.i.split.i

bb63.i.split.us.us.us.us.i:                       ; preds = %bb60.i.us.us.us.i
  %_84.i.us.us.us.i = icmp ugt i64 %_61.1.i, %_64.i.us.us.us.i
  br i1 %_84.i.us.us.us.i, label %bb24.i.us.lr.ph.us.us.us.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i, !dbg !2830

bb24.i.us.lr.ph.us.us.us.i:                       ; preds = %bb63.i.split.us.us.us.us.i
  %_82.i.us.us.us.us.i = load float, ptr %196, align 4, !noalias !2663, !noundef !12
  br i1 %_86.i.us.us.us.i, label %bb24.i.us.lr.ph.split.us.us.us.us.i, label %bb24.i.us.lr.ph.split.i

bb24.i.us.lr.ph.split.us.us.us.us.i:              ; preds = %bb24.i.us.lr.ph.us.us.us.i
  br i1 %_88.i.us.us.us.i, label %bb28.i.us.us.lr.ph.us.us.us.i, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i, !dbg !2838

bb28.i.us.us.lr.ph.us.us.us.i:                    ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.i
  %_87.i.us.us.us.us.us.i = load float, ptr %198, align 4, !noalias !2663, !noundef !12
  %_85.i.us.us.le931.us.us.us.i = load float, ptr %197, align 4, !noalias !2663, !noundef !12
  %_78.i.us.le.us.us.us.i = load float, ptr %195, align 4, !noalias !2663, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2839), !dbg !2842
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2842
  %_12.i39.us.us.us.i = load float, ptr %76, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.i = fcmp ule float %_12.i39.us.us.us.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.i = fcmp une float %_12.i39.us.us.us.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.i = load float, ptr %_10.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.i = load float, ptr %77, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.i = fadd float %_16.i42.us.us.us.i, %_17.i43.us.us.us.i, !dbg !2864
  %_20.i45655658.us.us.us.i = load float, ptr %78, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %199 = select i1 %_3.i196.us.us.us.i, float %_0.i255.us.us.us.i, float %_20.i45655658.us.us.us.i, !dbg !2869
  %_0.i407.us.us.us.i = select i1 %_3.i224.us.us.us.i, float %_16.i42.us.us.us.i, float %199, !dbg !2872
  store float %_0.i407.us.us.us.i, ptr %_10.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.i = select i1 %_3.i196.us.us.us.i, float %_17.i43.us.us.us.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.i, ptr %77, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.i = fadd float %_12.i39.us.us.us.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.i = select i1 %_3.i224.us.us.us.i, float %_12.i39.us.us.us.i, float %_0.i293.us.us.us.i, !dbg !2881
  store float %_4.i393.v.us.us.us.i, ptr %76, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.1.i = load float, ptr %79, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.1.i = fcmp ule float %_12.i39.us.us.us.1.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.1.i = fcmp une float %_12.i39.us.us.us.1.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.1.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.1.i = load float, ptr %80, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.1.i = fadd float %_16.i42.us.us.us.1.i, %_17.i43.us.us.us.1.i, !dbg !2864
  %_20.i45655658.us.us.us.1.i = load float, ptr %81, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %200 = select i1 %_3.i196.us.us.us.1.i, float %_0.i255.us.us.us.1.i, float %_20.i45655658.us.us.us.1.i, !dbg !2869
  %_0.i407.us.us.us.1.i = select i1 %_3.i224.us.us.us.1.i, float %_16.i42.us.us.us.1.i, float %200, !dbg !2872
  store float %_0.i407.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.1.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.1.i = select i1 %_3.i196.us.us.us.1.i, float %_17.i43.us.us.us.1.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.1.i, ptr %80, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.1.i = fadd float %_12.i39.us.us.us.1.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.1.i = select i1 %_3.i224.us.us.us.1.i, float %_12.i39.us.us.us.1.i, float %_0.i293.us.us.us.1.i, !dbg !2881
  store float %_4.i393.v.us.us.us.1.i, ptr %79, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.2.i = load float, ptr %82, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.2.i = fcmp ule float %_12.i39.us.us.us.2.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.2.i = fcmp une float %_12.i39.us.us.us.2.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.2.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.2.i = load float, ptr %83, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.2.i = fadd float %_16.i42.us.us.us.2.i, %_17.i43.us.us.us.2.i, !dbg !2864
  %_20.i45655658.us.us.us.2.i = load float, ptr %84, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %201 = select i1 %_3.i196.us.us.us.2.i, float %_0.i255.us.us.us.2.i, float %_20.i45655658.us.us.us.2.i, !dbg !2869
  %_0.i407.us.us.us.2.i = select i1 %_3.i224.us.us.us.2.i, float %_16.i42.us.us.us.2.i, float %201, !dbg !2872
  store float %_0.i407.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.2.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.2.i = select i1 %_3.i196.us.us.us.2.i, float %_17.i43.us.us.us.2.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.2.i, ptr %83, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.2.i = fadd float %_12.i39.us.us.us.2.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.2.i = select i1 %_3.i224.us.us.us.2.i, float %_12.i39.us.us.us.2.i, float %_0.i293.us.us.us.2.i, !dbg !2881
  store float %_4.i393.v.us.us.us.2.i, ptr %82, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.us.3.i = load float, ptr %85, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.us.3.i = fcmp ule float %_12.i39.us.us.us.3.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.us.3.i = fcmp une float %_12.i39.us.us.us.3.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.3.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.us.3.i = load float, ptr %86, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.us.3.i = fadd float %_16.i42.us.us.us.3.i, %_17.i43.us.us.us.3.i, !dbg !2864
  %_20.i45655658.us.us.us.3.i = load float, ptr %87, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %202 = select i1 %_3.i196.us.us.us.3.i, float %_0.i255.us.us.us.3.i, float %_20.i45655658.us.us.us.3.i, !dbg !2869
  %_0.i407.us.us.us.3.i = select i1 %_3.i224.us.us.us.3.i, float %_16.i42.us.us.us.3.i, float %202, !dbg !2872
  store float %_0.i407.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.us.3.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.us.3.i = select i1 %_3.i196.us.us.us.3.i, float %_17.i43.us.us.us.3.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.us.3.i, ptr %86, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.us.3.i = fadd float %_12.i39.us.us.us.3.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.us.3.i = select i1 %_3.i224.us.us.us.3.i, float %_12.i39.us.us.us.3.i, float %_0.i293.us.us.us.3.i, !dbg !2881
  store float %_4.i393.v.us.us.us.3.i, ptr %85, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %203 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.i), !dbg !2884
  %204 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.i), !dbg !2891
  %_37.i60.us.us.us.i = load float, ptr %13, align 4, !dbg !2894, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i222.us.us.us.i = fcmp ule float %_37.i60.us.us.us.i, 0.000000e+00, !dbg !2899
  %_3.i.i560.us.us.us.i = fcmp ule float %203, %204, !dbg !2901
  %_6.i.i562.us.us.us.i = bitcast float %203 to i32, !dbg !2907
  %_8.i.i564.us.us.us.i = bitcast float %204 to i32, !dbg !2911
  %_4.i.i567.us.us.us.i = select i1 %_3.i.i560.us.us.us.i, i32 %_8.i.i564.us.us.us.i, i32 %_6.i.i562.us.us.us.i, !dbg !2913
  %_4.i386.us.us.us.i = select i1 %_3.i222.us.us.us.i, i32 %_6.i.i562.us.us.us.i, i32 %_4.i.i567.us.us.us.i, !dbg !2914
  %_41.i64.us.us.us.i = load float, ptr %14, align 4, !dbg !2916, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i220.us.us.us.i = fcmp ule float %_41.i64.us.us.us.i, 0.000000e+00, !dbg !2917
  %_0.i277.us.us.us.i = fmul float %203, 5.000000e-01, !dbg !2919
  %_0.i276.us.us.us.i = fmul float %204, 5.000000e-01, !dbg !2922
  %_0.i254.us.us.us.i = fadd float %_0.i276.us.us.us.i, %_0.i277.us.us.us.i, !dbg !2924
  %_6.i374.us.us.us.i = bitcast float %_0.i254.us.us.us.i to i32, !dbg !2926
  %_4.i379.us.us.us.i = select i1 %_3.i220.us.us.us.i, i32 %_4.i386.us.us.us.i, i32 %_6.i374.us.us.us.i, !dbg !2929
  %_0.i380.us.us.us.i = bitcast i32 %_4.i379.us.us.us.i to float, !dbg !2930
  %_3.i.i552.us.us.us.i = fcmp ule float %_0.i380.us.us.us.i, 0x3E45798EE0000000, !dbg !2933
  %_4.i.i558.us.us.us.i = select i1 %_3.i.i552.us.us.us.i, i32 841731191, i32 %_4.i379.us.us.us.i, !dbg !2936
  %_0.i.i559.us.us.us.i = bitcast i32 %_4.i.i558.us.us.us.i to float, !dbg !2938
  %_3.i.i512.us.us.us.i = fcmp ule float %_0.i.i559.us.us.us.i, 0x3810000000000000, !dbg !2940
  %_4.i.i518.us.us.us.i = select i1 %_3.i.i512.us.us.us.i, i32 8388608, i32 %_4.i.i558.us.us.us.i, !dbg !2950
  %_5.i324.us.us.us.i = and i32 %_4.i.i518.us.us.us.i, 8388607, !dbg !2952
  %_4.i325.us.us.us.i = or disjoint i32 %_5.i324.us.us.us.i, 1065353216, !dbg !2952
  %significand.i326.us.us.us.i = bitcast i32 %_4.i325.us.us.us.i to float, !dbg !2957
  %_0.i285.us.us.us.i = fadd float %significand.i326.us.us.us.i, -1.000000e+00, !dbg !2960
  %_0.i265.us.us.us.i = fmul float %_0.i285.us.us.us.i, 0x3F9B17A960000000, !dbg !2963
  %205 = fsub float 0x3FBF9A8440000000, %_0.i265.us.us.us.i, !dbg !2968
  %_0.i265.us.us.us.1.i = fmul float %_0.i285.us.us.us.i, %205, !dbg !2963
  %_0.i249.us.us.us.1.i = fadd float %_0.i265.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !2968
  %_0.i265.us.us.us.2.i = fmul float %_0.i285.us.us.us.i, %_0.i249.us.us.us.1.i, !dbg !2963
  %_0.i249.us.us.us.2.i = fadd float %_0.i265.us.us.us.2.i, 0x3FDD544F20000000, !dbg !2968
  %_0.i265.us.us.us.3.i = fmul float %_0.i285.us.us.us.i, %_0.i249.us.us.us.2.i, !dbg !2963
  %_0.i249.us.us.us.3.i = fadd float %_0.i265.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !2968
  %_0.i265.us.us.us.4.i = fmul float %_0.i285.us.us.us.i, %_0.i249.us.us.us.3.i, !dbg !2963
  %_0.i249.us.us.us.4.i = fadd float %_0.i265.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !2968
  %_9.i327.us.us.us.i = lshr i32 %_4.i.i518.us.us.us.i, 23, !dbg !2970
  %_8.i328.us.us.us.i = or disjoint i32 %_9.i327.us.us.us.i, 1258291200, !dbg !2970
  %_7.i329.us.us.us.i = bitcast i32 %_8.i328.us.us.us.i to float, !dbg !2972
  %exponent.i330.us.us.us.i = fadd float %_7.i329.us.us.us.i, 0xC160000FE0000000, !dbg !2974
  %_0.i264.us.us.us.i = fmul float %_0.i285.us.us.us.i, %_0.i249.us.us.us.4.i, !dbg !2975
  %_0.i248.us.us.us.i = fadd float %exponent.i330.us.us.us.i, %_0.i264.us.us.us.i, !dbg !2977
  %_0.i275.us.us.us.i = fmul float %_0.i248.us.us.us.i, 0x4018151820000000, !dbg !2979
  %_3.i.i627.us.us.us.inv.i = fcmp olt float %_0.i275.us.us.us.i, 2.400000e+01, !dbg !2981
  %_0.i.i634.us.us.us.i = select i1 %_3.i.i627.us.us.us.inv.i, float %_0.i275.us.us.us.i, float 2.400000e+01, !dbg !2981
  %_3.i.i544.us.us.us.inv.i = fcmp ogt float %_0.i.i634.us.us.us.i, -1.600000e+02, !dbg !2985
  %_0.i.i551.us.us.us.i = select i1 %_3.i.i544.us.us.us.inv.i, float %_0.i.i634.us.us.us.i, float -1.600000e+02, !dbg !2985
  %_55.i81.us.us.us.i = load float, ptr %12, align 4, !dbg !2988, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i218.us.us.us.i = fcmp ule float %_55.i81.us.us.us.i, 0.000000e+00, !dbg !2990
  %_3.i204.us.us.us.i = fcmp oge float %_0.i.i551.us.us.us.i, %_0.i407.us.us.us.i, !dbg !2992
  %_0.i292.us.us.us.i = fsub float %_0.i407.us.us.us.i, %_0.i407.us.us.us.3.i, !dbg !2996
  %_3.i202.us.us.us.i = fcmp oge float %_0.i.i551.us.us.us.i, %_0.i292.us.us.us.i, !dbg !2999
  %..i203.us.us.us.i = sext i1 %_3.i202.us.us.us.i to i32, !dbg !3001
  %_0.i502.us.us.us.i = sext i1 %_3.i204.us.us.us.i to i32, !dbg !3004
  %_0.i496.us.us.us.i = select i1 %_3.i218.us.us.us.i, i32 %_0.i502.us.us.us.i, i32 %..i203.us.us.us.i, !dbg !3004
  %_0.i508.us.us.us.i = xor i32 %..i203.us.us.us.i, -1, !dbg !3010
  %_67.i91.us.us.us.i = load float, ptr %15, align 4, !dbg !3014, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i216.us.us.us.i = fcmp ogt float %_67.i91.us.us.us.i, 0.000000e+00, !dbg !3015
  %_0.i501.us.us.us.i = select i1 %_3.i216.us.us.us.i, i32 %_0.i508.us.us.us.i, i32 0, !dbg !3017
  %_0.i500.us.us.us.i = select i1 %_3.i218.us.us.us.i, i32 0, i32 %_0.i501.us.us.us.i, !dbg !3020
  %_0.i495.us.us.us.i = or i32 %_0.i500.us.us.us.i, %_0.i496.us.us.us.i, !dbg !3022
  %_5.i369.us.us.us.i = and i32 %_0.i495.us.us.us.i, 1065353216, !dbg !3025
  %_0.i373.us.us.us.i = bitcast i32 %_5.i369.us.us.us.i to float, !dbg !3027
  %_71.i97667668.us.us.us.i = load float, ptr %16, align 4, !dbg !3029, !alias.scope !2897, !noalias !2898, !noundef !12
  %_0.i291.us.us.us.i = fadd float %_67.i91.us.us.us.i, -1.000000e+00, !dbg !3031
  %206 = trunc nsw i32 %_0.i500.us.us.us.i to i1, !dbg !3033
  %_4.i367.v.us.us.us.i = select i1 %206, float %_0.i291.us.us.us.i, float %_67.i91.us.us.us.i, !dbg !3033
  %207 = trunc nsw i32 %_0.i496.us.us.us.i to i1, !dbg !3035
  %_0.i361.us.us.us.i = select i1 %207, float %_71.i97667668.us.us.us.i, float %_4.i367.v.us.us.us.i, !dbg !3035
  store float %_0.i361.us.us.us.i, ptr %15, align 4, !dbg !3037, !alias.scope !2852, !noalias !2853
  store i32 %_5.i369.us.us.us.i, ptr %12, align 4, !dbg !3038, !alias.scope !2852, !noalias !2853
  %_0.i290.us.us.us.i = fadd float %_0.i407.us.us.us.1.i, -1.000000e+00, !dbg !3039
  %_0.i289.us.us.us.i = fsub float %_0.i.i551.us.us.us.i, %_0.i407.us.us.us.i, !dbg !3041
  %_0.i274.us.us.us.i = fmul float %_0.i290.us.us.us.i, %_0.i289.us.us.us.i, !dbg !3043
  %208 = fneg float %_0.i407.us.us.us.2.i, !dbg !3045
  %_3.i.i536.inv.us.us.us.i = fcmp ogt float %_0.i274.us.us.us.i, %208, !dbg !3048
  %_4.i.i542.v.us.us.us.i = select i1 %_3.i.i536.inv.us.us.us.i, float %_0.i274.us.us.us.i, float %208, !dbg !3048
  %_3.i.i619.us.us.us.i = fcmp olt float %_4.i.i542.v.us.us.us.i, 0.000000e+00, !dbg !3051
  %209 = fcmp ule float %_0.i373.us.us.us.i, 0.000000e+00, !dbg !3055
  %210 = select i1 %209, i1 %_3.i.i619.us.us.us.i, i1 false, !dbg !3058
  %_0.i354.us.us.us.i = select i1 %210, float %_4.i.i542.v.us.us.us.i, float 0.000000e+00, !dbg !3058
  %_86.i110.us.us.us.i = load float, ptr %17, align 4, !dbg !3059, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i212.us.us.us.i = fcmp ule float %_0.i354.us.us.us.i, %_86.i110.us.us.us.i, !dbg !3061
  %_87.i112670.us.us.us.i = load i32, ptr %2, align 4, !dbg !3063, !alias.scope !2897, !noalias !2898, !noundef !12
  %_88.i113671.us.us.us.i = load i32, ptr %18, align 4, !dbg !3064, !alias.scope !2897, !noalias !2898, !noundef !12
  %_4.i347.us.us.us.i = select i1 %_3.i212.us.us.us.i, i32 %_88.i113671.us.us.us.i, i32 %_87.i112670.us.us.us.i, !dbg !3065
  %_0.i348.us.us.us.i = bitcast i32 %_4.i347.us.us.us.i to float, !dbg !3067
  %_0.i288.us.us.us.i = fsub float %_0.i354.us.us.us.i, %_86.i110.us.us.us.i, !dbg !3069
  %_4.i258.us.us.us.i = fmul float %_0.i288.us.us.us.i, %_0.i348.us.us.us.i, !dbg !3072
  %_0.i259.us.us.us.i = fadd float %_86.i110.us.us.us.i, %_4.i258.us.us.us.i, !dbg !3072
  %211 = tail call noundef float @llvm.fabs.f32(float %_0.i259.us.us.us.i), !dbg !3075
  %212 = fcmp uge float %211, 0x3BC79CA100000000, !dbg !3079
  %_0.i332.us.us.us.i = select i1 %212, float %_0.i259.us.us.us.i, float 0.000000e+00, !dbg !3082
  store float %_0.i332.us.us.us.i, ptr %17, align 4, !dbg !3083, !alias.scope !2852, !noalias !2853
  %_0.i273.us.us.us.i = fmul float %_0.i332.us.us.us.i, 0x3FC542A5A0000000, !dbg !3085
  %_3.i.i528.us.us.us.inv.i = fcmp ogt float %_0.i273.us.us.us.i, -1.260000e+02, !dbg !3089
  %_0.i.i535.us.us.us.i = select i1 %_3.i.i528.us.us.us.inv.i, float %_0.i273.us.us.us.i, float -1.260000e+02, !dbg !3089
  %_3.i.i611.us.us.us.inv.i = fcmp olt float %_0.i.i535.us.us.us.i, 1.270000e+02, !dbg !3094
  %_0.i.i618.us.us.us.i = select i1 %_3.i.i611.us.us.us.inv.i, float %_0.i.i535.us.us.us.i, float 1.270000e+02, !dbg !3094
  %213 = tail call noundef float @llvm.floor.f32(float %_0.i.i618.us.us.us.i), !dbg !3097
  %_0.i287.us.us.us.i = fsub float %_0.i.i618.us.us.us.i, %213, !dbg !3109
  %_98.i126.us.us.us.i = load float, ptr %19, align 4, !dbg !3112, !alias.scope !2897, !noalias !2898, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3114), !dbg !3117
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3119), !dbg !3117
  %_12.i.us.us.us.i = load float, ptr %88, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.i = fcmp ule float %_12.i.us.us.us.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.i = fcmp une float %_12.i.us.us.us.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.i = load float, ptr %89, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.i = fadd float %_16.i.us.us.us.i, %_17.i.us.us.us.i, !dbg !3131
  %_20.i677680.us.us.us.i = load float, ptr %90, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %214 = select i1 %_3.i200.us.us.us.i, float %_0.i257.us.us.us.i, float %_20.i677680.us.us.us.i, !dbg !3134
  %_0.i486.us.us.us.i = select i1 %_3.i240.us.us.us.i, float %_16.i.us.us.us.i, float %214, !dbg !3136
  store float %_0.i486.us.us.us.i, ptr %data.i.i.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.i = select i1 %_3.i200.us.us.us.i, float %_17.i.us.us.us.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.i, ptr %89, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.i = fadd float %_12.i.us.us.us.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.i = select i1 %_3.i240.us.us.us.i, float %_12.i.us.us.us.i, float %_0.i299.us.us.us.i, !dbg !3144
  store float %_4.i472.v.us.us.us.i, ptr %88, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.1.i = load float, ptr %91, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.1.i = fcmp ule float %_12.i.us.us.us.1.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.1.i = fcmp une float %_12.i.us.us.us.1.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.1.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.1.i = load float, ptr %92, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.1.i = fadd float %_16.i.us.us.us.1.i, %_17.i.us.us.us.1.i, !dbg !3131
  %_20.i677680.us.us.us.1.i = load float, ptr %93, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %215 = select i1 %_3.i200.us.us.us.1.i, float %_0.i257.us.us.us.1.i, float %_20.i677680.us.us.us.1.i, !dbg !3134
  %_0.i486.us.us.us.1.i = select i1 %_3.i240.us.us.us.1.i, float %_16.i.us.us.us.1.i, float %215, !dbg !3136
  store float %_0.i486.us.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.1.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.1.i = select i1 %_3.i200.us.us.us.1.i, float %_17.i.us.us.us.1.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.1.i, ptr %92, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.1.i = fadd float %_12.i.us.us.us.1.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.1.i = select i1 %_3.i240.us.us.us.1.i, float %_12.i.us.us.us.1.i, float %_0.i299.us.us.us.1.i, !dbg !3144
  store float %_4.i472.v.us.us.us.1.i, ptr %91, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.2.i = load float, ptr %94, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.2.i = fcmp ule float %_12.i.us.us.us.2.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.2.i = fcmp une float %_12.i.us.us.us.2.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.2.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.2.i = load float, ptr %95, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.2.i = fadd float %_16.i.us.us.us.2.i, %_17.i.us.us.us.2.i, !dbg !3131
  %_20.i677680.us.us.us.2.i = load float, ptr %96, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %216 = select i1 %_3.i200.us.us.us.2.i, float %_0.i257.us.us.us.2.i, float %_20.i677680.us.us.us.2.i, !dbg !3134
  %_0.i486.us.us.us.2.i = select i1 %_3.i240.us.us.us.2.i, float %_16.i.us.us.us.2.i, float %216, !dbg !3136
  store float %_0.i486.us.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.2.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.2.i = select i1 %_3.i200.us.us.us.2.i, float %_17.i.us.us.us.2.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.2.i, ptr %95, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.2.i = fadd float %_12.i.us.us.us.2.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.2.i = select i1 %_3.i240.us.us.us.2.i, float %_12.i.us.us.us.2.i, float %_0.i299.us.us.us.2.i, !dbg !3144
  store float %_4.i472.v.us.us.us.2.i, ptr %94, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.us.3.i = load float, ptr %97, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.us.3.i = fcmp ule float %_12.i.us.us.us.3.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.us.3.i = fcmp une float %_12.i.us.us.us.3.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.us.3.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.us.3.i = load float, ptr %98, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.us.3.i = fadd float %_16.i.us.us.us.3.i, %_17.i.us.us.us.3.i, !dbg !3131
  %_20.i677680.us.us.us.3.i = load float, ptr %99, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %217 = select i1 %_3.i200.us.us.us.3.i, float %_0.i257.us.us.us.3.i, float %_20.i677680.us.us.us.3.i, !dbg !3134
  %_0.i486.us.us.us.3.i = select i1 %_3.i240.us.us.us.3.i, float %_16.i.us.us.us.3.i, float %217, !dbg !3136
  store float %_0.i486.us.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i.us.us.us.3.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.us.3.i = select i1 %_3.i200.us.us.us.3.i, float %_17.i.us.us.us.3.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.us.3.i, ptr %98, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.us.3.i = fadd float %_12.i.us.us.us.3.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.us.3.i = select i1 %_3.i240.us.us.us.3.i, float %_12.i.us.us.us.3.i, float %_0.i299.us.us.us.3.i, !dbg !3144
  store float %_4.i472.v.us.us.us.3.i, ptr %97, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %218 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le931.us.us.us.i), !dbg !3147
  %219 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.i), !dbg !3149
  %_37.i.us.us.us.i = load float, ptr %21, align 4, !dbg !3151, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i238.us.us.us.i = fcmp ule float %_37.i.us.us.us.i, 0.000000e+00, !dbg !3154
  %_3.i.i594.us.us.us.i = fcmp ule float %218, %219, !dbg !3156
  %_6.i.i596.us.us.us.i = bitcast float %218 to i32, !dbg !3159
  %_8.i.i598.us.us.us.i = bitcast float %219 to i32, !dbg !3162
  %_4.i.i601.us.us.us.i = select i1 %_3.i.i594.us.us.us.i, i32 %_8.i.i598.us.us.us.i, i32 %_6.i.i596.us.us.us.i, !dbg !3164
  %_4.i465.us.us.us.i = select i1 %_3.i238.us.us.us.i, i32 %_6.i.i596.us.us.us.i, i32 %_4.i.i601.us.us.us.i, !dbg !3165
  %_41.i.us.us.us.i = load float, ptr %22, align 4, !dbg !3167, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i236.us.us.us.i = fcmp ule float %_41.i.us.us.us.i, 0.000000e+00, !dbg !3168
  %_0.i283.us.us.us.i = fmul float %218, 5.000000e-01, !dbg !3170
  %_0.i282.us.us.us.i = fmul float %219, 5.000000e-01, !dbg !3172
  %_0.i256.us.us.us.i = fadd float %_0.i282.us.us.us.i, %_0.i283.us.us.us.i, !dbg !3174
  %_6.i453.us.us.us.i = bitcast float %_0.i256.us.us.us.i to i32, !dbg !3176
  %_4.i458.us.us.us.i = select i1 %_3.i236.us.us.us.i, i32 %_4.i465.us.us.us.i, i32 %_6.i453.us.us.us.i, !dbg !3179
  %_0.i459.us.us.us.i = bitcast i32 %_4.i458.us.us.us.i to float, !dbg !3180
  %_3.i.i586.us.us.us.i = fcmp ule float %_0.i459.us.us.us.i, 0x3E45798EE0000000, !dbg !3182
  %_4.i.i592.us.us.us.i = select i1 %_3.i.i586.us.us.us.i, i32 841731191, i32 %_4.i458.us.us.us.i, !dbg !3185
  %_0.i.i593.us.us.us.i = bitcast i32 %_4.i.i592.us.us.us.i to float, !dbg !3187
  %_3.i.i.us.us.us.i = fcmp ule float %_0.i.i593.us.us.us.i, 0x3810000000000000, !dbg !3189
  %_4.i.i.us.us.us.i = select i1 %_3.i.i.us.us.us.i, i32 8388608, i32 %_4.i.i592.us.us.us.i, !dbg !3194
  %_5.i320.us.us.us.i = and i32 %_4.i.i.us.us.us.i, 8388607, !dbg !3196
  %_4.i321.us.us.us.i = or disjoint i32 %_5.i320.us.us.us.i, 1065353216, !dbg !3196
  %significand.i.us.us.us.i = bitcast i32 %_4.i321.us.us.us.i to float, !dbg !3198
  %_0.i284.us.us.us.i = fadd float %significand.i.us.us.us.i, -1.000000e+00, !dbg !3200
  %_0.i263.us.us.us.i = fmul float %_0.i284.us.us.us.i, 0x3F9B17A960000000, !dbg !3202
  %220 = fsub float 0x3FBF9A8440000000, %_0.i263.us.us.us.i, !dbg !3204
  %_0.i263.us.us.us.1.i = fmul float %_0.i284.us.us.us.i, %220, !dbg !3202
  %_0.i247.us.us.us.1.i = fadd float %_0.i263.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !3204
  %_0.i263.us.us.us.2.i = fmul float %_0.i284.us.us.us.i, %_0.i247.us.us.us.1.i, !dbg !3202
  %_0.i247.us.us.us.2.i = fadd float %_0.i263.us.us.us.2.i, 0x3FDD544F20000000, !dbg !3204
  %_0.i263.us.us.us.3.i = fmul float %_0.i284.us.us.us.i, %_0.i247.us.us.us.2.i, !dbg !3202
  %_0.i247.us.us.us.3.i = fadd float %_0.i263.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3204
  %_0.i263.us.us.us.4.i = fmul float %_0.i284.us.us.us.i, %_0.i247.us.us.us.3.i, !dbg !3202
  %_0.i247.us.us.us.4.i = fadd float %_0.i263.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !3204
  %_9.i.us.us.us.i = lshr i32 %_4.i.i.us.us.us.i, 23, !dbg !3206
  %_8.i322.us.us.us.i = or disjoint i32 %_9.i.us.us.us.i, 1258291200, !dbg !3206
  %_7.i.us.us.us.i = bitcast i32 %_8.i322.us.us.us.i to float, !dbg !3207
  %exponent.i.us.us.us.i = fadd float %_7.i.us.us.us.i, 0xC160000FE0000000, !dbg !3209
  %_0.i262.us.us.us.i = fmul float %_0.i284.us.us.us.i, %_0.i247.us.us.us.4.i, !dbg !3210
  %_0.i246.us.us.us.i = fadd float %exponent.i.us.us.us.i, %_0.i262.us.us.us.i, !dbg !3212
  %_0.i281.us.us.us.i = fmul float %_0.i246.us.us.us.i, 0x4018151820000000, !dbg !3214
  %_3.i.i643.us.us.us.inv.i = fcmp olt float %_0.i281.us.us.us.i, 2.400000e+01, !dbg !3216
  %_0.i.i650.us.us.us.i = select i1 %_3.i.i643.us.us.us.inv.i, float %_0.i281.us.us.us.i, float 2.400000e+01, !dbg !3216
  %_3.i.i578.us.us.us.inv.i = fcmp ogt float %_0.i.i650.us.us.us.i, -1.600000e+02, !dbg !3219
  %_0.i.i585.us.us.us.i = select i1 %_3.i.i578.us.us.us.inv.i, float %_0.i.i650.us.us.us.i, float -1.600000e+02, !dbg !3219
  %_55.i12.us.us.us.i = load float, ptr %20, align 4, !dbg !3222, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i234.us.us.us.i = fcmp ule float %_55.i12.us.us.us.i, 0.000000e+00, !dbg !3223
  %_3.i208.us.us.us.i = fcmp oge float %_0.i.i585.us.us.us.i, %_0.i486.us.us.us.i, !dbg !3225
  %_0.i298.us.us.us.i = fsub float %_0.i486.us.us.us.i, %_0.i486.us.us.us.3.i, !dbg !3227
  %_3.i206.us.us.us.i = fcmp oge float %_0.i.i585.us.us.us.i, %_0.i298.us.us.us.i, !dbg !3229
  %..i207.us.us.us.i = sext i1 %_3.i206.us.us.us.i to i32, !dbg !3231
  %_0.i506.us.us.us.i = sext i1 %_3.i208.us.us.us.i to i32, !dbg !3233
  %_0.i499.us.us.us.i = select i1 %_3.i234.us.us.us.i, i32 %_0.i506.us.us.us.i, i32 %..i207.us.us.us.i, !dbg !3233
  %_0.i510.us.us.us.i = xor i32 %..i207.us.us.us.i, -1, !dbg !3235
  %_67.i14.us.us.us.i = load float, ptr %23, align 4, !dbg !3237, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i232.us.us.us.i = fcmp ogt float %_67.i14.us.us.us.i, 0.000000e+00, !dbg !3238
  %_0.i505.us.us.us.i = select i1 %_3.i232.us.us.us.i, i32 %_0.i510.us.us.us.i, i32 0, !dbg !3240
  %_0.i504.us.us.us.i = select i1 %_3.i234.us.us.us.i, i32 0, i32 %_0.i505.us.us.us.i, !dbg !3242
  %_0.i498.us.us.us.i = or i32 %_0.i504.us.us.us.i, %_0.i499.us.us.us.i, !dbg !3244
  %_5.i448.us.us.us.i = and i32 %_0.i498.us.us.us.i, 1065353216, !dbg !3246
  %_0.i452.us.us.us.i = bitcast i32 %_5.i448.us.us.us.i to float, !dbg !3248
  %_71.i689690.us.us.us.i = load float, ptr %24, align 4, !dbg !3250, !alias.scope !3152, !noalias !3153, !noundef !12
  %_0.i297.us.us.us.i = fadd float %_67.i14.us.us.us.i, -1.000000e+00, !dbg !3251
  %221 = trunc nsw i32 %_0.i504.us.us.us.i to i1, !dbg !3253
  %_4.i446.v.us.us.us.i = select i1 %221, float %_0.i297.us.us.us.i, float %_67.i14.us.us.us.i, !dbg !3253
  %222 = trunc nsw i32 %_0.i499.us.us.us.i to i1, !dbg !3255
  %_0.i440.us.us.us.i = select i1 %222, float %_71.i689690.us.us.us.i, float %_4.i446.v.us.us.us.i, !dbg !3255
  store float %_0.i440.us.us.us.i, ptr %23, align 4, !dbg !3257, !alias.scope !3123, !noalias !3124
  store i32 %_5.i448.us.us.us.i, ptr %20, align 4, !dbg !3258, !alias.scope !3123, !noalias !3124
  %_0.i296.us.us.us.i = fadd float %_0.i486.us.us.us.1.i, -1.000000e+00, !dbg !3259
  %_0.i295.us.us.us.i = fsub float %_0.i.i585.us.us.us.i, %_0.i486.us.us.us.i, !dbg !3261
  %_0.i280.us.us.us.i = fmul float %_0.i296.us.us.us.i, %_0.i295.us.us.us.i, !dbg !3263
  %223 = fneg float %_0.i486.us.us.us.2.i, !dbg !3265
  %_3.i.i569.inv.us.us.us.i = fcmp ogt float %_0.i280.us.us.us.i, %223, !dbg !3267
  %_4.i.i576.v.us.us.us.i = select i1 %_3.i.i569.inv.us.us.us.i, float %_0.i280.us.us.us.i, float %223, !dbg !3267
  %_3.i.i635.us.us.us.i = fcmp olt float %_4.i.i576.v.us.us.us.i, 0.000000e+00, !dbg !3270
  %224 = fcmp ule float %_0.i452.us.us.us.i, 0.000000e+00, !dbg !3273
  %225 = select i1 %224, i1 %_3.i.i635.us.us.us.i, i1 false, !dbg !3275
  %_0.i433.us.us.us.i = select i1 %225, float %_4.i.i576.v.us.us.us.i, float 0.000000e+00, !dbg !3275
  %_86.i22.us.us.us.i = load float, ptr %25, align 4, !dbg !3276, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i228.us.us.us.i = fcmp ule float %_0.i433.us.us.us.i, %_86.i22.us.us.us.i, !dbg !3277
  %_87.i24692.us.us.us.i = load i32, ptr %_32.i, align 4, !dbg !3279, !alias.scope !3152, !noalias !3153, !noundef !12
  %_88.i25693.us.us.us.i = load i32, ptr %26, align 4, !dbg !3280, !alias.scope !3152, !noalias !3153, !noundef !12
  %_4.i427.us.us.us.i = select i1 %_3.i228.us.us.us.i, i32 %_88.i25693.us.us.us.i, i32 %_87.i24692.us.us.us.i, !dbg !3281
  %_0.i.us.us.us.i = bitcast i32 %_4.i427.us.us.us.i to float, !dbg !3283
  %_0.i294.us.us.us.i = fsub float %_0.i433.us.us.us.i, %_86.i22.us.us.us.i, !dbg !3285
  %_4.i260.us.us.us.i = fmul float %_0.i294.us.us.us.i, %_0.i.us.us.us.i, !dbg !3287
  %_0.i261.us.us.us.i = fadd float %_86.i22.us.us.us.i, %_4.i260.us.us.us.i, !dbg !3287
  %226 = tail call noundef float @llvm.fabs.f32(float %_0.i261.us.us.us.i), !dbg !3289
  %227 = fcmp uge float %226, 0x3BC79CA100000000, !dbg !3292
  %_0.i336.us.us.us.i = select i1 %227, float %_0.i261.us.us.us.i, float 0.000000e+00, !dbg !3294
  store float %_0.i336.us.us.us.i, ptr %25, align 4, !dbg !3295, !alias.scope !3123, !noalias !3124
  %_0.i279.us.us.us.i = fmul float %_0.i336.us.us.us.i, 0x3FC542A5A0000000, !dbg !3296
  %_3.i.i520.us.us.us.inv.i = fcmp ogt float %_0.i279.us.us.us.i, -1.260000e+02, !dbg !3299
  %_0.i.i527.us.us.us.i = select i1 %_3.i.i520.us.us.us.inv.i, float %_0.i279.us.us.us.i, float -1.260000e+02, !dbg !3299
  %_3.i.i603.us.us.us.inv.i = fcmp olt float %_0.i.i527.us.us.us.i, 1.270000e+02, !dbg !3303
  %_0.i.i610.us.us.us.i = select i1 %_3.i.i603.us.us.us.inv.i, float %_0.i.i527.us.us.us.i, float 1.270000e+02, !dbg !3303
  %228 = tail call noundef float @llvm.floor.f32(float %_0.i.i610.us.us.us.i), !dbg !3306
  %_0.i286.us.us.us.i = fsub float %_0.i.i610.us.us.us.i, %228, !dbg !3310
  %_0.i268.us.us.us.i = fmul float %_0.i286.us.us.us.i, 0x3F5E974FA0000000, !dbg !3312
  %_0.i251.us.us.us.i = fadd float %_0.i268.us.us.us.i, 0x3F82778560000000, !dbg !3317
  %_0.i268.us.us.us.1.i = fmul float %_0.i286.us.us.us.i, %_0.i251.us.us.us.i, !dbg !3312
  %_0.i251.us.us.us.1.i = fadd float %_0.i268.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3317
  %_0.i268.us.us.us.2.i = fmul float %_0.i286.us.us.us.i, %_0.i251.us.us.us.1.i, !dbg !3312
  %_0.i251.us.us.us.2.i = fadd float %_0.i268.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3317
  %_0.i268.us.us.us.3.i = fmul float %_0.i286.us.us.us.i, %_0.i251.us.us.us.2.i, !dbg !3312
  %_0.i251.us.us.us.3.i = fadd float %_0.i268.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3317
  %_0.i271.us.us.us.i = fmul float %_0.i287.us.us.us.i, 0x3F5E974FA0000000, !dbg !3319
  %_0.i253.us.us.us.i = fadd float %_0.i271.us.us.us.i, 0x3F82778560000000, !dbg !3321
  %_0.i271.us.us.us.1.i = fmul float %_0.i287.us.us.us.i, %_0.i253.us.us.us.i, !dbg !3319
  %_0.i253.us.us.us.1.i = fadd float %_0.i271.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !3321
  %_0.i271.us.us.us.2.i = fmul float %_0.i287.us.us.us.i, %_0.i253.us.us.us.1.i, !dbg !3319
  %_0.i253.us.us.us.2.i = fadd float %_0.i271.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !3321
  %_0.i271.us.us.us.3.i = fmul float %_0.i287.us.us.us.i, %_0.i253.us.us.us.2.i, !dbg !3319
  %_0.i253.us.us.us.3.i = fadd float %_0.i271.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3321
  %_0.i270.us.us.us.i = fmul float %_0.i287.us.us.us.i, %_0.i253.us.us.us.3.i, !dbg !3323
  %_0.i252.us.us.us.i = fadd float %_0.i270.us.us.us.i, 1.000000e+00, !dbg !3325
  %biased.i191.us.us.us.i = fadd float %213, 0x4160000FE0000000, !dbg !3327
  %_4.i192.us.us.us.i = bitcast float %biased.i191.us.us.us.i to i32, !dbg !3331
  %_3.i193.us.us.us.i = shl i32 %_4.i192.us.us.us.i, 23, !dbg !3335
  %_0.i194.us.us.us.i = bitcast i32 %_3.i193.us.us.us.i to float, !dbg !3336
  %_0.i269.us.us.us.i = fmul float %_0.i252.us.us.us.i, %_0.i194.us.us.us.i, !dbg !3339
  %_3.i195.us.us.us.i = fcmp une float %_0.i332.us.us.us.i, 0.000000e+00, !dbg !3341
  %_3.i210.us.us.us.i = fcmp ule float %_98.i126.us.us.us.i, 0.000000e+00, !dbg !3343
  %_0.i494675.not.us.us.us.i = and i1 %_3.i210.us.us.us.i, %_3.i195.us.us.us.i, !dbg !3345
  %_0.i272.us.us.us.i = fmul float %_0.i311.us.us.us.i, %_0.i269.us.us.us.i, !dbg !3345
  %_4.i340.v.us.us.us.i = select i1 %_0.i494675.not.us.us.us.i, float %_0.i272.us.us.us.i, float %_0.i311.us.us.us.i, !dbg !3348
  %_0.i267.us.us.us.i = fmul float %_0.i286.us.us.us.i, %_0.i251.us.us.us.3.i, !dbg !3350
  %_0.i250.us.us.us.i = fadd float %_0.i267.us.us.us.i, 1.000000e+00, !dbg !3352
  %biased.i.us.us.us.i = fadd float %228, 0x4160000FE0000000, !dbg !3354
  %_4.i188.us.us.us.i = bitcast float %biased.i.us.us.us.i to i32, !dbg !3356
  %_3.i189.us.us.us.i = shl i32 %_4.i188.us.us.us.i, 23, !dbg !3358
  %_0.i190.us.us.us.i = bitcast i32 %_3.i189.us.us.us.i to float, !dbg !3359
  %_0.i266.us.us.us.i = fmul float %_0.i250.us.us.us.i, %_0.i190.us.us.us.i, !dbg !3361
  %_3.i198.us.us.us.i = fcmp une float %_0.i336.us.us.us.i, 0.000000e+00, !dbg !3363
  %_98.i.us.us.us.i = load float, ptr %27, align 4, !dbg !3365, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i226.us.us.us.i = fcmp ule float %_98.i.us.us.us.i, 0.000000e+00, !dbg !3366
  %_0.i497697.not.us.us.us.i = and i1 %_3.i226.us.us.us.i, %_3.i198.us.us.us.i, !dbg !3368
  %_0.i278.us.us.us.i = fmul float %_0.i309.us.us.us.i, %_0.i266.us.us.us.i, !dbg !3368
  %_4.i420.v.us.us.us.i = select i1 %_0.i497697.not.us.us.us.i, float %_0.i278.us.us.us.i, float %_0.i309.us.us.us.i, !dbg !3370
  store float %_4.i340.v.us.us.us.i, ptr %_123.i.us.us.us.i, align 4, !dbg !3372, !alias.scope !3375, !noalias !2714
  store float %_4.i420.v.us.us.us.i, ptr %_141.i.us.us.us.i, align 4, !dbg !3378, !alias.scope !3380, !noalias !2737
  %exitcond2630.not.i = icmp eq i64 %194, %..i, !dbg !3383
  br i1 %exitcond2630.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb40.i.us.us.us.i, !dbg !3386, !llvm.loop !3391

bb42.i.us.us.i:                                   ; preds = %bb28.i.us.us.lr.ph.us.us.i, %bb40.i.us.us.preheader.i
  %iter.sroa.0.0.i1071.us.us.i = phi i64 [ %229, %bb28.i.us.us.lr.ph.us.us.i ], [ 0, %bb40.i.us.us.preheader.i ]
  %229 = add nuw nsw i64 %iter.sroa.0.0.i1071.us.us.i, 1, !dbg !2678
  %_27.i.us.us.i = trunc i64 %iter.sroa.0.0.i1071.us.us.i to i32, !dbg !2689
  %now.i.us.us.i = add i32 %base.i.i, %_27.i.us.us.i, !dbg !2690
  %_30.i.us.us.i = and i32 %now.i.us.us.i, %_52.i, !dbg !2693
  %_29.i.us.us.i = zext i32 %_30.i.us.us.i to i64, !dbg !2694
  %_123.i.us.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i1071.us.us.i, !dbg !2695
  %_124.not.not.i.us.us.i = icmp ugt i64 %_58.1.i, %_29.i.us.us.i, !dbg !2699
  br i1 %_124.not.not.i.us.us.i, label %bb45.i.us.us.i, label %bb46.i.i, !dbg !2699, !prof !2704

bb45.i.us.us.i:                                   ; preds = %bb42.i.us.us.i
  %_0.i319.us.us.i = load float, ptr %_123.i.us.us.i, align 4, !dbg !2705, !alias.scope !2711, !noalias !2714, !noundef !12
  %_133.i.us.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_29.i.us.us.i, !dbg !2715
  store float %_0.i319.us.us.i, ptr %_133.i.us.us.i, align 4, !dbg !2719, !alias.scope !2722, !noalias !2663
  %_141.i.us.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i1071.us.us.i, !dbg !2725
  %_0.i317.us.us.i = load float, ptr %_141.i.us.us.i, align 4, !dbg !2732, !alias.scope !2734, !noalias !2737, !noundef !12
  %_149.i.us.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_29.i.us.us.i, !dbg !2738
  store float %_0.i317.us.us.i, ptr %_149.i.us.us.i, align 4, !dbg !2745, !alias.scope !2747, !noalias !2663
  %exitcond2631.not.i = icmp eq i64 %iter.sroa.0.0.i1071.us.us.i, %empty.sroa.6.0.i.i, !dbg !3392
  br i1 %exitcond2631.not.i, label %bb53.i.i, label %bb52.i.us.us.i, !dbg !3392, !prof !180

bb52.i.us.us.i:                                   ; preds = %bb45.i.us.us.i
  %_158.not.not.i.us.us.i = icmp ugt i64 %_59.1.i, %_29.i.us.us.i, !dbg !3390
  br i1 %_158.not.not.i.us.us.i, label %bb54.i.us.us.i, label %bb55.i.i, !dbg !3390, !prof !2704

bb54.i.us.us.i:                                   ; preds = %bb52.i.us.us.i
  %_157.i.us.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.us.i, !dbg !2750
  %_0.i315.us.us.i = load float, ptr %_157.i.us.us.i, align 4, !dbg !2757, !alias.scope !2759, !noalias !2663, !noundef !12
  %_165.i.us.us.i = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_29.i.us.us.i, !dbg !2762
  store float %_0.i315.us.us.i, ptr %_165.i.us.us.i, align 4, !dbg !2769, !alias.scope !2771, !noalias !2663
  %_174.not.not.i.us.us.i = icmp ugt i64 %_61.1.i, %_29.i.us.us.i, !dbg !3387
  br i1 %_174.not.not.i.us.us.i, label %bb58.i.us.us.i, label %bb59.i.i, !dbg !3387, !prof !2704

bb58.i.us.us.i:                                   ; preds = %bb54.i.us.us.i
  %_173.i.us.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.us.i, !dbg !2774
  %_0.i313.us.us.i = load float, ptr %_173.i.us.us.i, align 4, !dbg !2781, !alias.scope !2783, !noalias !2663, !noundef !12
  %_181.i.us.us.i = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_29.i.us.us.i, !dbg !2786
  store float %_0.i313.us.us.i, ptr %_181.i.us.us.i, align 4, !dbg !2793, !alias.scope !2795, !noalias !2663
  %_52.i.us.us.i = sub i32 %now.i.us.us.i, %_53.i, !dbg !2798
  %_51.i.us.us.i = and i32 %_52.i.us.us.i, %_52.i, !dbg !2801
  %_50.i.us.us.i = zext i32 %_51.i.us.us.i to i64, !dbg !2802
  %_182.not.not.i.us.us.i = icmp ugt i64 %_58.1.i, %_50.i.us.us.i, !dbg !2803
  br i1 %_182.not.not.i.us.us.i, label %bb60.i.us.us.i, label %bb61.i.i, !dbg !2803, !prof !2704

bb60.i.us.us.i:                                   ; preds = %bb58.i.us.us.i
  %_189.i.us.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_50.i.us.us.i, !dbg !2808
  %_0.i311.us.us.i = load float, ptr %_189.i.us.us.i, align 4, !dbg !2812, !alias.scope !2814, !noalias !2663, !noundef !12
  %_195.i.us.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_50.i.us.us.i, !dbg !2817
  %_0.i309.us.us.i = load float, ptr %_195.i.us.us.i, align 4, !dbg !2825, !alias.scope !2827, !noalias !2663, !noundef !12
  %_67.i.us.us.i = load i32, ptr %_45.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_66.i.us.us.i = sub i32 %now.i.us.us.i, %_67.i.us.us.i
  %_65.i.us.us.i = and i32 %_66.i.us.us.i, %_52.i
  %_64.i.us.us.i = zext i32 %_65.i.us.us.i to i64
  %_75.i.us.us.i = load i32, ptr %_50.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_74.i.us.us.i = sub i32 %now.i.us.us.i, %_75.i.us.us.i
  %_73.i.us.us.i = and i32 %_74.i.us.us.i, %_52.i
  %_72.i.us.us.i = zext i32 %_73.i.us.us.i to i64
  %_80.i.us.us.i = icmp ugt i64 %_59.1.i, %_64.i.us.us.i
  %230 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_64.i.us.us.i
  %231 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_64.i.us.us.i
  %_86.i.us.us.i = icmp ugt i64 %_61.1.i, %_72.i.us.us.i
  %232 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_72.i.us.us.i
  %_88.i.us.us.i = icmp ugt i64 %_59.1.i, %_72.i.us.us.i
  %233 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_72.i.us.us.i
  br i1 %_80.i.us.us.i, label %bb63.i.split.us.us.us.i, label %bb63.i.split.i

bb63.i.split.us.us.us.i:                          ; preds = %bb60.i.us.us.i
  %_84.i.us.us.i = icmp ugt i64 %_61.1.i, %_64.i.us.us.i
  br i1 %_84.i.us.us.i, label %bb24.i.us.lr.ph.us.us.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i, !dbg !2830

bb24.i.us.lr.ph.us.us.i:                          ; preds = %bb63.i.split.us.us.us.i
  %_82.i.us.us.us.i = load float, ptr %231, align 4, !noalias !2663, !noundef !12
  br i1 %_86.i.us.us.i, label %bb24.i.us.lr.ph.split.us.us.us.i, label %bb24.i.us.lr.ph.split.i

bb24.i.us.lr.ph.split.us.us.us.i:                 ; preds = %bb24.i.us.lr.ph.us.us.i
  br i1 %_88.i.us.us.i, label %bb28.i.us.us.lr.ph.us.us.i, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i, !dbg !2838

bb28.i.us.us.lr.ph.us.us.i:                       ; preds = %bb24.i.us.lr.ph.split.us.us.us.i
  %_87.i.us.us.us.us.i = load float, ptr %233, align 4, !noalias !2663, !noundef !12
  %_85.i.us.us.le931.us.us.i = load float, ptr %232, align 4, !noalias !2663, !noundef !12
  %_78.i.us.le.us.us.i = load float, ptr %230, align 4, !noalias !2663, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2839), !dbg !2842
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2842
  %_12.i39.us.us.i = load float, ptr %52, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.i = fcmp ule float %_12.i39.us.us.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.i = fcmp une float %_12.i39.us.us.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.i = load float, ptr %_10.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.i = load float, ptr %53, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.i = fadd float %_16.i42.us.us.i, %_17.i43.us.us.i, !dbg !2864
  %_20.i45655658.us.us.i = load float, ptr %54, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %234 = select i1 %_3.i196.us.us.i, float %_0.i255.us.us.i, float %_20.i45655658.us.us.i, !dbg !2869
  %_0.i407.us.us.i = select i1 %_3.i224.us.us.i, float %_16.i42.us.us.i, float %234, !dbg !2872
  store float %_0.i407.us.us.i, ptr %_10.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.i = select i1 %_3.i196.us.us.i, float %_17.i43.us.us.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.i, ptr %53, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.i = fadd float %_12.i39.us.us.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.i = select i1 %_3.i224.us.us.i, float %_12.i39.us.us.i, float %_0.i293.us.us.i, !dbg !2881
  store float %_4.i393.v.us.us.i, ptr %52, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.1.i = load float, ptr %55, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.1.i = fcmp ule float %_12.i39.us.us.1.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.1.i = fcmp une float %_12.i39.us.us.1.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.1.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.1.i = load float, ptr %56, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.1.i = fadd float %_16.i42.us.us.1.i, %_17.i43.us.us.1.i, !dbg !2864
  %_20.i45655658.us.us.1.i = load float, ptr %57, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %235 = select i1 %_3.i196.us.us.1.i, float %_0.i255.us.us.1.i, float %_20.i45655658.us.us.1.i, !dbg !2869
  %_0.i407.us.us.1.i = select i1 %_3.i224.us.us.1.i, float %_16.i42.us.us.1.i, float %235, !dbg !2872
  store float %_0.i407.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.1.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.1.i = select i1 %_3.i196.us.us.1.i, float %_17.i43.us.us.1.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.1.i, ptr %56, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.1.i = fadd float %_12.i39.us.us.1.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.1.i = select i1 %_3.i224.us.us.1.i, float %_12.i39.us.us.1.i, float %_0.i293.us.us.1.i, !dbg !2881
  store float %_4.i393.v.us.us.1.i, ptr %55, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.2.i = load float, ptr %58, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.2.i = fcmp ule float %_12.i39.us.us.2.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.2.i = fcmp une float %_12.i39.us.us.2.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.2.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.2.i = load float, ptr %59, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.2.i = fadd float %_16.i42.us.us.2.i, %_17.i43.us.us.2.i, !dbg !2864
  %_20.i45655658.us.us.2.i = load float, ptr %60, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %236 = select i1 %_3.i196.us.us.2.i, float %_0.i255.us.us.2.i, float %_20.i45655658.us.us.2.i, !dbg !2869
  %_0.i407.us.us.2.i = select i1 %_3.i224.us.us.2.i, float %_16.i42.us.us.2.i, float %236, !dbg !2872
  store float %_0.i407.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.2.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.2.i = select i1 %_3.i196.us.us.2.i, float %_17.i43.us.us.2.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.2.i, ptr %59, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.2.i = fadd float %_12.i39.us.us.2.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.2.i = select i1 %_3.i224.us.us.2.i, float %_12.i39.us.us.2.i, float %_0.i293.us.us.2.i, !dbg !2881
  store float %_4.i393.v.us.us.2.i, ptr %58, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.us.3.i = load float, ptr %61, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.us.3.i = fcmp ule float %_12.i39.us.us.3.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.us.3.i = fcmp une float %_12.i39.us.us.3.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.us.3.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.us.3.i = load float, ptr %62, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.us.3.i = fadd float %_16.i42.us.us.3.i, %_17.i43.us.us.3.i, !dbg !2864
  %_20.i45655658.us.us.3.i = load float, ptr %63, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %237 = select i1 %_3.i196.us.us.3.i, float %_0.i255.us.us.3.i, float %_20.i45655658.us.us.3.i, !dbg !2869
  %_0.i407.us.us.3.i = select i1 %_3.i224.us.us.3.i, float %_16.i42.us.us.3.i, float %237, !dbg !2872
  store float %_0.i407.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i37.us.us.3.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.us.3.i = select i1 %_3.i196.us.us.3.i, float %_17.i43.us.us.3.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.us.3.i, ptr %62, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.us.3.i = fadd float %_12.i39.us.us.3.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.us.3.i = select i1 %_3.i224.us.us.3.i, float %_12.i39.us.us.3.i, float %_0.i293.us.us.3.i, !dbg !2881
  store float %_4.i393.v.us.us.3.i, ptr %61, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %238 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.i), !dbg !2884
  %239 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.i), !dbg !2891
  %_37.i60.us.us.i = load float, ptr %13, align 4, !dbg !2894, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i222.us.us.i = fcmp ule float %_37.i60.us.us.i, 0.000000e+00, !dbg !2899
  %_3.i.i560.us.us.i = fcmp ule float %238, %239, !dbg !2901
  %_6.i.i562.us.us.i = bitcast float %238 to i32, !dbg !2907
  %_8.i.i564.us.us.i = bitcast float %239 to i32, !dbg !2911
  %_4.i.i567.us.us.i = select i1 %_3.i.i560.us.us.i, i32 %_8.i.i564.us.us.i, i32 %_6.i.i562.us.us.i, !dbg !2913
  %_4.i386.us.us.i = select i1 %_3.i222.us.us.i, i32 %_6.i.i562.us.us.i, i32 %_4.i.i567.us.us.i, !dbg !2914
  %_41.i64.us.us.i = load float, ptr %14, align 4, !dbg !2916, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i220.us.us.i = fcmp ule float %_41.i64.us.us.i, 0.000000e+00, !dbg !2917
  %_0.i277.us.us.i = fmul float %238, 5.000000e-01, !dbg !2919
  %_0.i276.us.us.i = fmul float %239, 5.000000e-01, !dbg !2922
  %_0.i254.us.us.i = fadd float %_0.i276.us.us.i, %_0.i277.us.us.i, !dbg !2924
  %_6.i374.us.us.i = bitcast float %_0.i254.us.us.i to i32, !dbg !2926
  %_4.i379.us.us.i = select i1 %_3.i220.us.us.i, i32 %_4.i386.us.us.i, i32 %_6.i374.us.us.i, !dbg !2929
  %_0.i380.us.us.i = bitcast i32 %_4.i379.us.us.i to float, !dbg !2930
  %_3.i.i552.us.us.i = fcmp ule float %_0.i380.us.us.i, 0x3E45798EE0000000, !dbg !2933
  %_4.i.i558.us.us.i = select i1 %_3.i.i552.us.us.i, i32 841731191, i32 %_4.i379.us.us.i, !dbg !2936
  %_0.i.i559.us.us.i = bitcast i32 %_4.i.i558.us.us.i to float, !dbg !2938
  %_3.i.i512.us.us.i = fcmp ule float %_0.i.i559.us.us.i, 0x3810000000000000, !dbg !2940
  %_4.i.i518.us.us.i = select i1 %_3.i.i512.us.us.i, i32 8388608, i32 %_4.i.i558.us.us.i, !dbg !2950
  %_5.i324.us.us.i = and i32 %_4.i.i518.us.us.i, 8388607, !dbg !2952
  %_4.i325.us.us.i = or disjoint i32 %_5.i324.us.us.i, 1065353216, !dbg !2952
  %significand.i326.us.us.i = bitcast i32 %_4.i325.us.us.i to float, !dbg !2957
  %_0.i285.us.us.i = fadd float %significand.i326.us.us.i, -1.000000e+00, !dbg !2960
  %_0.i265.us.us.i = fmul float %_0.i285.us.us.i, 0x3F9B17A960000000, !dbg !2963
  %240 = fsub float 0x3FBF9A8440000000, %_0.i265.us.us.i, !dbg !2968
  %_0.i265.us.us.1.i = fmul float %_0.i285.us.us.i, %240, !dbg !2963
  %_0.i249.us.us.1.i = fadd float %_0.i265.us.us.1.i, 0xBFD1E3F400000000, !dbg !2968
  %_0.i265.us.us.2.i = fmul float %_0.i285.us.us.i, %_0.i249.us.us.1.i, !dbg !2963
  %_0.i249.us.us.2.i = fadd float %_0.i265.us.us.2.i, 0x3FDD544F20000000, !dbg !2968
  %_0.i265.us.us.3.i = fmul float %_0.i285.us.us.i, %_0.i249.us.us.2.i, !dbg !2963
  %_0.i249.us.us.3.i = fadd float %_0.i265.us.us.3.i, 0xBFE6FC2A60000000, !dbg !2968
  %_0.i265.us.us.4.i = fmul float %_0.i285.us.us.i, %_0.i249.us.us.3.i, !dbg !2963
  %_0.i249.us.us.4.i = fadd float %_0.i265.us.us.4.i, 0x3FF714B2A0000000, !dbg !2968
  %_9.i327.us.us.i = lshr i32 %_4.i.i518.us.us.i, 23, !dbg !2970
  %_8.i328.us.us.i = or disjoint i32 %_9.i327.us.us.i, 1258291200, !dbg !2970
  %_7.i329.us.us.i = bitcast i32 %_8.i328.us.us.i to float, !dbg !2972
  %exponent.i330.us.us.i = fadd float %_7.i329.us.us.i, 0xC160000FE0000000, !dbg !2974
  %_0.i264.us.us.i = fmul float %_0.i285.us.us.i, %_0.i249.us.us.4.i, !dbg !2975
  %_0.i248.us.us.i = fadd float %exponent.i330.us.us.i, %_0.i264.us.us.i, !dbg !2977
  %_0.i275.us.us.i = fmul float %_0.i248.us.us.i, 0x4018151820000000, !dbg !2979
  %_3.i.i627.us.us.inv.i = fcmp olt float %_0.i275.us.us.i, 2.400000e+01, !dbg !2981
  %_0.i.i634.us.us.i = select i1 %_3.i.i627.us.us.inv.i, float %_0.i275.us.us.i, float 2.400000e+01, !dbg !2981
  %_3.i.i544.us.us.inv.i = fcmp ogt float %_0.i.i634.us.us.i, -1.600000e+02, !dbg !2985
  %_0.i.i551.us.us.i = select i1 %_3.i.i544.us.us.inv.i, float %_0.i.i634.us.us.i, float -1.600000e+02, !dbg !2985
  %_55.i81.us.us.i = load float, ptr %12, align 4, !dbg !2988, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i218.us.us.i = fcmp ule float %_55.i81.us.us.i, 0.000000e+00, !dbg !2990
  %_3.i204.us.us.i = fcmp oge float %_0.i.i551.us.us.i, %_0.i407.us.us.i, !dbg !2992
  %_0.i292.us.us.i = fsub float %_0.i407.us.us.i, %_0.i407.us.us.3.i, !dbg !2996
  %_3.i202.us.us.i = fcmp oge float %_0.i.i551.us.us.i, %_0.i292.us.us.i, !dbg !2999
  %..i203.us.us.i = sext i1 %_3.i202.us.us.i to i32, !dbg !3001
  %_0.i502.us.us.i = sext i1 %_3.i204.us.us.i to i32, !dbg !3004
  %_0.i496.us.us.i = select i1 %_3.i218.us.us.i, i32 %_0.i502.us.us.i, i32 %..i203.us.us.i, !dbg !3004
  %_0.i508.us.us.i = xor i32 %..i203.us.us.i, -1, !dbg !3010
  %_67.i91.us.us.i = load float, ptr %15, align 4, !dbg !3014, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i216.us.us.i = fcmp ogt float %_67.i91.us.us.i, 0.000000e+00, !dbg !3015
  %_0.i501.us.us.i = select i1 %_3.i216.us.us.i, i32 %_0.i508.us.us.i, i32 0, !dbg !3017
  %_0.i500.us.us.i = select i1 %_3.i218.us.us.i, i32 0, i32 %_0.i501.us.us.i, !dbg !3020
  %_0.i495.us.us.i = or i32 %_0.i500.us.us.i, %_0.i496.us.us.i, !dbg !3022
  %_5.i369.us.us.i = and i32 %_0.i495.us.us.i, 1065353216, !dbg !3025
  %_0.i373.us.us.i = bitcast i32 %_5.i369.us.us.i to float, !dbg !3027
  %_71.i97667668.us.us.i = load float, ptr %16, align 4, !dbg !3029, !alias.scope !2897, !noalias !2898, !noundef !12
  %_0.i291.us.us.i = fadd float %_67.i91.us.us.i, -1.000000e+00, !dbg !3031
  %241 = trunc nsw i32 %_0.i500.us.us.i to i1, !dbg !3033
  %_4.i367.v.us.us.i = select i1 %241, float %_0.i291.us.us.i, float %_67.i91.us.us.i, !dbg !3033
  %242 = trunc nsw i32 %_0.i496.us.us.i to i1, !dbg !3035
  %_0.i361.us.us.i = select i1 %242, float %_71.i97667668.us.us.i, float %_4.i367.v.us.us.i, !dbg !3035
  store float %_0.i361.us.us.i, ptr %15, align 4, !dbg !3037, !alias.scope !2852, !noalias !2853
  store i32 %_5.i369.us.us.i, ptr %12, align 4, !dbg !3038, !alias.scope !2852, !noalias !2853
  %_0.i290.us.us.i = fadd float %_0.i407.us.us.1.i, -1.000000e+00, !dbg !3039
  %_0.i289.us.us.i = fsub float %_0.i.i551.us.us.i, %_0.i407.us.us.i, !dbg !3041
  %_0.i274.us.us.i = fmul float %_0.i290.us.us.i, %_0.i289.us.us.i, !dbg !3043
  %243 = fneg float %_0.i407.us.us.2.i, !dbg !3045
  %_3.i.i536.inv.us.us.i = fcmp ogt float %_0.i274.us.us.i, %243, !dbg !3048
  %_4.i.i542.v.us.us.i = select i1 %_3.i.i536.inv.us.us.i, float %_0.i274.us.us.i, float %243, !dbg !3048
  %_3.i.i619.us.us.i = fcmp olt float %_4.i.i542.v.us.us.i, 0.000000e+00, !dbg !3051
  %244 = fcmp ule float %_0.i373.us.us.i, 0.000000e+00, !dbg !3055
  %245 = select i1 %244, i1 %_3.i.i619.us.us.i, i1 false, !dbg !3058
  %_0.i354.us.us.i = select i1 %245, float %_4.i.i542.v.us.us.i, float 0.000000e+00, !dbg !3058
  %_86.i110.us.us.i = load float, ptr %17, align 4, !dbg !3059, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i212.us.us.i = fcmp ule float %_0.i354.us.us.i, %_86.i110.us.us.i, !dbg !3061
  %_87.i112670.us.us.i = load i32, ptr %2, align 4, !dbg !3063, !alias.scope !2897, !noalias !2898, !noundef !12
  %_88.i113671.us.us.i = load i32, ptr %18, align 4, !dbg !3064, !alias.scope !2897, !noalias !2898, !noundef !12
  %_4.i347.us.us.i = select i1 %_3.i212.us.us.i, i32 %_88.i113671.us.us.i, i32 %_87.i112670.us.us.i, !dbg !3065
  %_0.i348.us.us.i = bitcast i32 %_4.i347.us.us.i to float, !dbg !3067
  %_0.i288.us.us.i = fsub float %_0.i354.us.us.i, %_86.i110.us.us.i, !dbg !3069
  %_4.i258.us.us.i = fmul float %_0.i288.us.us.i, %_0.i348.us.us.i, !dbg !3072
  %_0.i259.us.us.i = fadd float %_86.i110.us.us.i, %_4.i258.us.us.i, !dbg !3072
  %246 = tail call noundef float @llvm.fabs.f32(float %_0.i259.us.us.i), !dbg !3075
  %247 = fcmp uge float %246, 0x3BC79CA100000000, !dbg !3079
  %_0.i332.us.us.i = select i1 %247, float %_0.i259.us.us.i, float 0.000000e+00, !dbg !3082
  store float %_0.i332.us.us.i, ptr %17, align 4, !dbg !3083, !alias.scope !2852, !noalias !2853
  %_0.i273.us.us.i = fmul float %_0.i332.us.us.i, 0x3FC542A5A0000000, !dbg !3085
  %_3.i.i528.us.us.inv.i = fcmp ogt float %_0.i273.us.us.i, -1.260000e+02, !dbg !3089
  %_0.i.i535.us.us.i = select i1 %_3.i.i528.us.us.inv.i, float %_0.i273.us.us.i, float -1.260000e+02, !dbg !3089
  %_3.i.i611.us.us.inv.i = fcmp olt float %_0.i.i535.us.us.i, 1.270000e+02, !dbg !3094
  %_0.i.i618.us.us.i = select i1 %_3.i.i611.us.us.inv.i, float %_0.i.i535.us.us.i, float 1.270000e+02, !dbg !3094
  %248 = tail call noundef float @llvm.floor.f32(float %_0.i.i618.us.us.i), !dbg !3097
  %_0.i287.us.us.i = fsub float %_0.i.i618.us.us.i, %248, !dbg !3109
  %_98.i126.us.us.i = load float, ptr %19, align 4, !dbg !3112, !alias.scope !2897, !noalias !2898, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3114), !dbg !3117
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3119), !dbg !3117
  %_12.i.us.us.i = load float, ptr %64, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.i = fcmp ule float %_12.i.us.us.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.i = fcmp une float %_12.i.us.us.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.i = load float, ptr %65, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.i = fadd float %_16.i.us.us.i, %_17.i.us.us.i, !dbg !3131
  %_20.i677680.us.us.i = load float, ptr %66, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %249 = select i1 %_3.i200.us.us.i, float %_0.i257.us.us.i, float %_20.i677680.us.us.i, !dbg !3134
  %_0.i486.us.us.i = select i1 %_3.i240.us.us.i, float %_16.i.us.us.i, float %249, !dbg !3136
  store float %_0.i486.us.us.i, ptr %data.i.i.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.i = select i1 %_3.i200.us.us.i, float %_17.i.us.us.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.i, ptr %65, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.i = fadd float %_12.i.us.us.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.i = select i1 %_3.i240.us.us.i, float %_12.i.us.us.i, float %_0.i299.us.us.i, !dbg !3144
  store float %_4.i472.v.us.us.i, ptr %64, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.1.i = load float, ptr %67, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.1.i = fcmp ule float %_12.i.us.us.1.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.1.i = fcmp une float %_12.i.us.us.1.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.1.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.1.i = load float, ptr %68, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.1.i = fadd float %_16.i.us.us.1.i, %_17.i.us.us.1.i, !dbg !3131
  %_20.i677680.us.us.1.i = load float, ptr %69, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %250 = select i1 %_3.i200.us.us.1.i, float %_0.i257.us.us.1.i, float %_20.i677680.us.us.1.i, !dbg !3134
  %_0.i486.us.us.1.i = select i1 %_3.i240.us.us.1.i, float %_16.i.us.us.1.i, float %250, !dbg !3136
  store float %_0.i486.us.us.1.i, ptr %iter1.sroa.0.0.ptr.i.us.us.1.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.1.i = select i1 %_3.i200.us.us.1.i, float %_17.i.us.us.1.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.1.i, ptr %68, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.1.i = fadd float %_12.i.us.us.1.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.1.i = select i1 %_3.i240.us.us.1.i, float %_12.i.us.us.1.i, float %_0.i299.us.us.1.i, !dbg !3144
  store float %_4.i472.v.us.us.1.i, ptr %67, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.2.i = load float, ptr %70, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.2.i = fcmp ule float %_12.i.us.us.2.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.2.i = fcmp une float %_12.i.us.us.2.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.2.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.2.i = load float, ptr %71, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.2.i = fadd float %_16.i.us.us.2.i, %_17.i.us.us.2.i, !dbg !3131
  %_20.i677680.us.us.2.i = load float, ptr %72, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %251 = select i1 %_3.i200.us.us.2.i, float %_0.i257.us.us.2.i, float %_20.i677680.us.us.2.i, !dbg !3134
  %_0.i486.us.us.2.i = select i1 %_3.i240.us.us.2.i, float %_16.i.us.us.2.i, float %251, !dbg !3136
  store float %_0.i486.us.us.2.i, ptr %iter1.sroa.0.0.ptr.i.us.us.2.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.2.i = select i1 %_3.i200.us.us.2.i, float %_17.i.us.us.2.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.2.i, ptr %71, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.2.i = fadd float %_12.i.us.us.2.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.2.i = select i1 %_3.i240.us.us.2.i, float %_12.i.us.us.2.i, float %_0.i299.us.us.2.i, !dbg !3144
  store float %_4.i472.v.us.us.2.i, ptr %70, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.us.3.i = load float, ptr %73, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.us.3.i = fcmp ule float %_12.i.us.us.3.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.us.3.i = fcmp une float %_12.i.us.us.3.i, 1.000000e+00, !dbg !3127
  %_16.i.us.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.us.3.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.us.3.i = load float, ptr %74, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.us.3.i = fadd float %_16.i.us.us.3.i, %_17.i.us.us.3.i, !dbg !3131
  %_20.i677680.us.us.3.i = load float, ptr %75, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %252 = select i1 %_3.i200.us.us.3.i, float %_0.i257.us.us.3.i, float %_20.i677680.us.us.3.i, !dbg !3134
  %_0.i486.us.us.3.i = select i1 %_3.i240.us.us.3.i, float %_16.i.us.us.3.i, float %252, !dbg !3136
  store float %_0.i486.us.us.3.i, ptr %iter1.sroa.0.0.ptr.i.us.us.3.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.us.3.i = select i1 %_3.i200.us.us.3.i, float %_17.i.us.us.3.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.us.3.i, ptr %74, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.us.3.i = fadd float %_12.i.us.us.3.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.us.3.i = select i1 %_3.i240.us.us.3.i, float %_12.i.us.us.3.i, float %_0.i299.us.us.3.i, !dbg !3144
  store float %_4.i472.v.us.us.3.i, ptr %73, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %253 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le931.us.us.i), !dbg !3147
  %254 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.i), !dbg !3149
  %_37.i.us.us.i = load float, ptr %21, align 4, !dbg !3151, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i238.us.us.i = fcmp ule float %_37.i.us.us.i, 0.000000e+00, !dbg !3154
  %_3.i.i594.us.us.i = fcmp ule float %253, %254, !dbg !3156
  %_6.i.i596.us.us.i = bitcast float %253 to i32, !dbg !3159
  %_8.i.i598.us.us.i = bitcast float %254 to i32, !dbg !3162
  %_4.i.i601.us.us.i = select i1 %_3.i.i594.us.us.i, i32 %_8.i.i598.us.us.i, i32 %_6.i.i596.us.us.i, !dbg !3164
  %_4.i465.us.us.i = select i1 %_3.i238.us.us.i, i32 %_6.i.i596.us.us.i, i32 %_4.i.i601.us.us.i, !dbg !3165
  %_41.i.us.us.i = load float, ptr %22, align 4, !dbg !3167, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i236.us.us.i = fcmp ule float %_41.i.us.us.i, 0.000000e+00, !dbg !3168
  %_0.i283.us.us.i = fmul float %253, 5.000000e-01, !dbg !3170
  %_0.i282.us.us.i = fmul float %254, 5.000000e-01, !dbg !3172
  %_0.i256.us.us.i = fadd float %_0.i282.us.us.i, %_0.i283.us.us.i, !dbg !3174
  %_6.i453.us.us.i = bitcast float %_0.i256.us.us.i to i32, !dbg !3176
  %_4.i458.us.us.i = select i1 %_3.i236.us.us.i, i32 %_4.i465.us.us.i, i32 %_6.i453.us.us.i, !dbg !3179
  %_0.i459.us.us.i = bitcast i32 %_4.i458.us.us.i to float, !dbg !3180
  %_3.i.i586.us.us.i = fcmp ule float %_0.i459.us.us.i, 0x3E45798EE0000000, !dbg !3182
  %_4.i.i592.us.us.i = select i1 %_3.i.i586.us.us.i, i32 841731191, i32 %_4.i458.us.us.i, !dbg !3185
  %_0.i.i593.us.us.i = bitcast i32 %_4.i.i592.us.us.i to float, !dbg !3187
  %_3.i.i.us.us.i = fcmp ule float %_0.i.i593.us.us.i, 0x3810000000000000, !dbg !3189
  %_4.i.i.us.us.i = select i1 %_3.i.i.us.us.i, i32 8388608, i32 %_4.i.i592.us.us.i, !dbg !3194
  %_5.i320.us.us.i = and i32 %_4.i.i.us.us.i, 8388607, !dbg !3196
  %_4.i321.us.us.i = or disjoint i32 %_5.i320.us.us.i, 1065353216, !dbg !3196
  %significand.i.us.us.i = bitcast i32 %_4.i321.us.us.i to float, !dbg !3198
  %_0.i284.us.us.i = fadd float %significand.i.us.us.i, -1.000000e+00, !dbg !3200
  %_0.i263.us.us.i = fmul float %_0.i284.us.us.i, 0x3F9B17A960000000, !dbg !3202
  %255 = fsub float 0x3FBF9A8440000000, %_0.i263.us.us.i, !dbg !3204
  %_0.i263.us.us.1.i = fmul float %_0.i284.us.us.i, %255, !dbg !3202
  %_0.i247.us.us.1.i = fadd float %_0.i263.us.us.1.i, 0xBFD1E3F400000000, !dbg !3204
  %_0.i263.us.us.2.i = fmul float %_0.i284.us.us.i, %_0.i247.us.us.1.i, !dbg !3202
  %_0.i247.us.us.2.i = fadd float %_0.i263.us.us.2.i, 0x3FDD544F20000000, !dbg !3204
  %_0.i263.us.us.3.i = fmul float %_0.i284.us.us.i, %_0.i247.us.us.2.i, !dbg !3202
  %_0.i247.us.us.3.i = fadd float %_0.i263.us.us.3.i, 0xBFE6FC2A60000000, !dbg !3204
  %_0.i263.us.us.4.i = fmul float %_0.i284.us.us.i, %_0.i247.us.us.3.i, !dbg !3202
  %_0.i247.us.us.4.i = fadd float %_0.i263.us.us.4.i, 0x3FF714B2A0000000, !dbg !3204
  %_9.i.us.us.i = lshr i32 %_4.i.i.us.us.i, 23, !dbg !3206
  %_8.i322.us.us.i = or disjoint i32 %_9.i.us.us.i, 1258291200, !dbg !3206
  %_7.i.us.us.i = bitcast i32 %_8.i322.us.us.i to float, !dbg !3207
  %exponent.i.us.us.i = fadd float %_7.i.us.us.i, 0xC160000FE0000000, !dbg !3209
  %_0.i262.us.us.i = fmul float %_0.i284.us.us.i, %_0.i247.us.us.4.i, !dbg !3210
  %_0.i246.us.us.i = fadd float %exponent.i.us.us.i, %_0.i262.us.us.i, !dbg !3212
  %_0.i281.us.us.i = fmul float %_0.i246.us.us.i, 0x4018151820000000, !dbg !3214
  %_3.i.i643.us.us.inv.i = fcmp olt float %_0.i281.us.us.i, 2.400000e+01, !dbg !3216
  %_0.i.i650.us.us.i = select i1 %_3.i.i643.us.us.inv.i, float %_0.i281.us.us.i, float 2.400000e+01, !dbg !3216
  %_3.i.i578.us.us.inv.i = fcmp ogt float %_0.i.i650.us.us.i, -1.600000e+02, !dbg !3219
  %_0.i.i585.us.us.i = select i1 %_3.i.i578.us.us.inv.i, float %_0.i.i650.us.us.i, float -1.600000e+02, !dbg !3219
  %_55.i12.us.us.i = load float, ptr %20, align 4, !dbg !3222, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i234.us.us.i = fcmp ule float %_55.i12.us.us.i, 0.000000e+00, !dbg !3223
  %_3.i208.us.us.i = fcmp oge float %_0.i.i585.us.us.i, %_0.i486.us.us.i, !dbg !3225
  %_0.i298.us.us.i = fsub float %_0.i486.us.us.i, %_0.i486.us.us.3.i, !dbg !3227
  %_3.i206.us.us.i = fcmp oge float %_0.i.i585.us.us.i, %_0.i298.us.us.i, !dbg !3229
  %..i207.us.us.i = sext i1 %_3.i206.us.us.i to i32, !dbg !3231
  %_0.i506.us.us.i = sext i1 %_3.i208.us.us.i to i32, !dbg !3233
  %_0.i499.us.us.i = select i1 %_3.i234.us.us.i, i32 %_0.i506.us.us.i, i32 %..i207.us.us.i, !dbg !3233
  %_0.i510.us.us.i = xor i32 %..i207.us.us.i, -1, !dbg !3235
  %_67.i14.us.us.i = load float, ptr %23, align 4, !dbg !3237, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i232.us.us.i = fcmp ogt float %_67.i14.us.us.i, 0.000000e+00, !dbg !3238
  %_0.i505.us.us.i = select i1 %_3.i232.us.us.i, i32 %_0.i510.us.us.i, i32 0, !dbg !3240
  %_0.i504.us.us.i = select i1 %_3.i234.us.us.i, i32 0, i32 %_0.i505.us.us.i, !dbg !3242
  %_0.i498.us.us.i = or i32 %_0.i504.us.us.i, %_0.i499.us.us.i, !dbg !3244
  %_5.i448.us.us.i = and i32 %_0.i498.us.us.i, 1065353216, !dbg !3246
  %_0.i452.us.us.i = bitcast i32 %_5.i448.us.us.i to float, !dbg !3248
  %_71.i689690.us.us.i = load float, ptr %24, align 4, !dbg !3250, !alias.scope !3152, !noalias !3153, !noundef !12
  %_0.i297.us.us.i = fadd float %_67.i14.us.us.i, -1.000000e+00, !dbg !3251
  %256 = trunc nsw i32 %_0.i504.us.us.i to i1, !dbg !3253
  %_4.i446.v.us.us.i = select i1 %256, float %_0.i297.us.us.i, float %_67.i14.us.us.i, !dbg !3253
  %257 = trunc nsw i32 %_0.i499.us.us.i to i1, !dbg !3255
  %_0.i440.us.us.i = select i1 %257, float %_71.i689690.us.us.i, float %_4.i446.v.us.us.i, !dbg !3255
  store float %_0.i440.us.us.i, ptr %23, align 4, !dbg !3257, !alias.scope !3123, !noalias !3124
  store i32 %_5.i448.us.us.i, ptr %20, align 4, !dbg !3258, !alias.scope !3123, !noalias !3124
  %_0.i296.us.us.i = fadd float %_0.i486.us.us.1.i, -1.000000e+00, !dbg !3259
  %_0.i295.us.us.i = fsub float %_0.i.i585.us.us.i, %_0.i486.us.us.i, !dbg !3261
  %_0.i280.us.us.i = fmul float %_0.i296.us.us.i, %_0.i295.us.us.i, !dbg !3263
  %258 = fneg float %_0.i486.us.us.2.i, !dbg !3265
  %_3.i.i569.inv.us.us.i = fcmp ogt float %_0.i280.us.us.i, %258, !dbg !3267
  %_4.i.i576.v.us.us.i = select i1 %_3.i.i569.inv.us.us.i, float %_0.i280.us.us.i, float %258, !dbg !3267
  %_3.i.i635.us.us.i = fcmp olt float %_4.i.i576.v.us.us.i, 0.000000e+00, !dbg !3270
  %259 = fcmp ule float %_0.i452.us.us.i, 0.000000e+00, !dbg !3273
  %260 = select i1 %259, i1 %_3.i.i635.us.us.i, i1 false, !dbg !3275
  %_0.i433.us.us.i = select i1 %260, float %_4.i.i576.v.us.us.i, float 0.000000e+00, !dbg !3275
  %_86.i22.us.us.i = load float, ptr %25, align 4, !dbg !3276, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i228.us.us.i = fcmp ule float %_0.i433.us.us.i, %_86.i22.us.us.i, !dbg !3277
  %_87.i24692.us.us.i = load i32, ptr %_32.i, align 4, !dbg !3279, !alias.scope !3152, !noalias !3153, !noundef !12
  %_88.i25693.us.us.i = load i32, ptr %26, align 4, !dbg !3280, !alias.scope !3152, !noalias !3153, !noundef !12
  %_4.i427.us.us.i = select i1 %_3.i228.us.us.i, i32 %_88.i25693.us.us.i, i32 %_87.i24692.us.us.i, !dbg !3281
  %_0.i.us.us.i = bitcast i32 %_4.i427.us.us.i to float, !dbg !3283
  %_0.i294.us.us.i = fsub float %_0.i433.us.us.i, %_86.i22.us.us.i, !dbg !3285
  %_4.i260.us.us.i = fmul float %_0.i294.us.us.i, %_0.i.us.us.i, !dbg !3287
  %_0.i261.us.us.i = fadd float %_86.i22.us.us.i, %_4.i260.us.us.i, !dbg !3287
  %261 = tail call noundef float @llvm.fabs.f32(float %_0.i261.us.us.i), !dbg !3289
  %262 = fcmp uge float %261, 0x3BC79CA100000000, !dbg !3292
  %_0.i336.us.us.i = select i1 %262, float %_0.i261.us.us.i, float 0.000000e+00, !dbg !3294
  store float %_0.i336.us.us.i, ptr %25, align 4, !dbg !3295, !alias.scope !3123, !noalias !3124
  %_0.i279.us.us.i = fmul float %_0.i336.us.us.i, 0x3FC542A5A0000000, !dbg !3296
  %_3.i.i520.us.us.inv.i = fcmp ogt float %_0.i279.us.us.i, -1.260000e+02, !dbg !3299
  %_0.i.i527.us.us.i = select i1 %_3.i.i520.us.us.inv.i, float %_0.i279.us.us.i, float -1.260000e+02, !dbg !3299
  %_3.i.i603.us.us.inv.i = fcmp olt float %_0.i.i527.us.us.i, 1.270000e+02, !dbg !3303
  %_0.i.i610.us.us.i = select i1 %_3.i.i603.us.us.inv.i, float %_0.i.i527.us.us.i, float 1.270000e+02, !dbg !3303
  %263 = tail call noundef float @llvm.floor.f32(float %_0.i.i610.us.us.i), !dbg !3306
  %_0.i286.us.us.i = fsub float %_0.i.i610.us.us.i, %263, !dbg !3310
  %_0.i268.us.us.i = fmul float %_0.i286.us.us.i, 0x3F5E974FA0000000, !dbg !3312
  %_0.i251.us.us.i = fadd float %_0.i268.us.us.i, 0x3F82778560000000, !dbg !3317
  %_0.i268.us.us.1.i = fmul float %_0.i286.us.us.i, %_0.i251.us.us.i, !dbg !3312
  %_0.i251.us.us.1.i = fadd float %_0.i268.us.us.1.i, 0x3FAC91CE60000000, !dbg !3317
  %_0.i268.us.us.2.i = fmul float %_0.i286.us.us.i, %_0.i251.us.us.1.i, !dbg !3312
  %_0.i251.us.us.2.i = fadd float %_0.i268.us.us.2.i, 0x3FCEBDB560000000, !dbg !3317
  %_0.i268.us.us.3.i = fmul float %_0.i286.us.us.i, %_0.i251.us.us.2.i, !dbg !3312
  %_0.i251.us.us.3.i = fadd float %_0.i268.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3317
  %_0.i271.us.us.i = fmul float %_0.i287.us.us.i, 0x3F5E974FA0000000, !dbg !3319
  %_0.i253.us.us.i = fadd float %_0.i271.us.us.i, 0x3F82778560000000, !dbg !3321
  %_0.i271.us.us.1.i = fmul float %_0.i287.us.us.i, %_0.i253.us.us.i, !dbg !3319
  %_0.i253.us.us.1.i = fadd float %_0.i271.us.us.1.i, 0x3FAC91CE60000000, !dbg !3321
  %_0.i271.us.us.2.i = fmul float %_0.i287.us.us.i, %_0.i253.us.us.1.i, !dbg !3319
  %_0.i253.us.us.2.i = fadd float %_0.i271.us.us.2.i, 0x3FCEBDB560000000, !dbg !3321
  %_0.i271.us.us.3.i = fmul float %_0.i287.us.us.i, %_0.i253.us.us.2.i, !dbg !3319
  %_0.i253.us.us.3.i = fadd float %_0.i271.us.us.3.i, 0x3FE62E4BA0000000, !dbg !3321
  %_0.i270.us.us.i = fmul float %_0.i287.us.us.i, %_0.i253.us.us.3.i, !dbg !3323
  %_0.i252.us.us.i = fadd float %_0.i270.us.us.i, 1.000000e+00, !dbg !3325
  %biased.i191.us.us.i = fadd float %248, 0x4160000FE0000000, !dbg !3327
  %_4.i192.us.us.i = bitcast float %biased.i191.us.us.i to i32, !dbg !3331
  %_3.i193.us.us.i = shl i32 %_4.i192.us.us.i, 23, !dbg !3335
  %_0.i194.us.us.i = bitcast i32 %_3.i193.us.us.i to float, !dbg !3336
  %_0.i269.us.us.i = fmul float %_0.i252.us.us.i, %_0.i194.us.us.i, !dbg !3339
  %_3.i195.us.us.i = fcmp une float %_0.i332.us.us.i, 0.000000e+00, !dbg !3341
  %_3.i210.us.us.i = fcmp ule float %_98.i126.us.us.i, 0.000000e+00, !dbg !3343
  %_0.i494675.not.us.us.i = and i1 %_3.i210.us.us.i, %_3.i195.us.us.i, !dbg !3345
  %_0.i272.us.us.i = fmul float %_0.i311.us.us.i, %_0.i269.us.us.i, !dbg !3345
  %_4.i340.v.us.us.i = select i1 %_0.i494675.not.us.us.i, float %_0.i272.us.us.i, float %_0.i311.us.us.i, !dbg !3348
  %_0.i267.us.us.i = fmul float %_0.i286.us.us.i, %_0.i251.us.us.3.i, !dbg !3350
  %_0.i250.us.us.i = fadd float %_0.i267.us.us.i, 1.000000e+00, !dbg !3352
  %biased.i.us.us.i = fadd float %263, 0x4160000FE0000000, !dbg !3354
  %_4.i188.us.us.i = bitcast float %biased.i.us.us.i to i32, !dbg !3356
  %_3.i189.us.us.i = shl i32 %_4.i188.us.us.i, 23, !dbg !3358
  %_0.i190.us.us.i = bitcast i32 %_3.i189.us.us.i to float, !dbg !3359
  %_0.i266.us.us.i = fmul float %_0.i250.us.us.i, %_0.i190.us.us.i, !dbg !3361
  %_3.i198.us.us.i = fcmp une float %_0.i336.us.us.i, 0.000000e+00, !dbg !3363
  %_98.i.us.us.i = load float, ptr %27, align 4, !dbg !3365, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i226.us.us.i = fcmp ule float %_98.i.us.us.i, 0.000000e+00, !dbg !3366
  %_0.i497697.not.us.us.i = and i1 %_3.i226.us.us.i, %_3.i198.us.us.i, !dbg !3368
  %_0.i278.us.us.i = fmul float %_0.i309.us.us.i, %_0.i266.us.us.i, !dbg !3368
  %_4.i420.v.us.us.i = select i1 %_0.i497697.not.us.us.i, float %_0.i278.us.us.i, float %_0.i309.us.us.i, !dbg !3370
  store float %_4.i340.v.us.us.i, ptr %_123.i.us.us.i, align 4, !dbg !3372, !alias.scope !3375, !noalias !2714
  store float %_4.i420.v.us.us.i, ptr %_141.i.us.us.i, align 4, !dbg !3378, !alias.scope !3380, !noalias !2737
  %exitcond2633.not.i = icmp eq i64 %229, %..i, !dbg !3383
  br i1 %exitcond2633.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb42.i.us.us.i, !dbg !3386, !llvm.loop !3393

bb40.i.us.i:                                      ; preds = %bb28.i.us.us.lr.ph.us.i, %bb40.i.us.preheader.i
  %iter.sroa.0.0.i1071.us.i = phi i64 [ %264, %bb28.i.us.us.lr.ph.us.i ], [ 0, %bb40.i.us.preheader.i ]
  %264 = add nuw nsw i64 %iter.sroa.0.0.i1071.us.i, 1, !dbg !2678
  %_27.i.us.i = trunc i64 %iter.sroa.0.0.i1071.us.i to i32, !dbg !2689
  %now.i.us.i = add i32 %base.i.i, %_27.i.us.i, !dbg !2690
  %_30.i.us.i = and i32 %now.i.us.i, %_52.i, !dbg !2693
  %_29.i.us.i = zext i32 %_30.i.us.i to i64, !dbg !2694
  %exitcond2634.not.i = icmp eq i64 %iter.sroa.0.0.i1071.us.i, %..i, !dbg !2666
  br i1 %exitcond2634.not.i, label %bb43.i.i, label %bb42.i.us.i, !dbg !2666, !prof !180

bb42.i.us.i:                                      ; preds = %bb40.i.us.i
  %_123.i.us.i = getelementptr inbounds nuw float, ptr %left.0, i64 %iter.sroa.0.0.i1071.us.i, !dbg !2695
  %_124.not.not.i.us.i = icmp ugt i64 %_58.1.i, %_29.i.us.i, !dbg !2699
  br i1 %_124.not.not.i.us.i, label %bb45.i.us.i, label %bb46.i.i, !dbg !2699, !prof !2704

bb45.i.us.i:                                      ; preds = %bb42.i.us.i
  %_0.i319.us.i = load float, ptr %_123.i.us.i, align 4, !dbg !2705, !alias.scope !2711, !noalias !2714, !noundef !12
  %_133.i.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_29.i.us.i, !dbg !2715
  store float %_0.i319.us.i, ptr %_133.i.us.i, align 4, !dbg !2719, !alias.scope !2722, !noalias !2663
  %_141.i.us.i = getelementptr inbounds nuw float, ptr %right.0, i64 %iter.sroa.0.0.i1071.us.i, !dbg !2725
  %_142.not.not.i.us.i = icmp ugt i64 %_60.1.i, %_29.i.us.i, !dbg !3394
  br i1 %_142.not.not.i.us.i, label %bb50.i.us.i, label %bb51.i.i, !dbg !3394, !prof !2704

bb50.i.us.i:                                      ; preds = %bb45.i.us.i
  %_0.i317.us.i = load float, ptr %_141.i.us.i, align 4, !dbg !2732, !alias.scope !2734, !noalias !2737, !noundef !12
  %_149.i.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_29.i.us.i, !dbg !2738
  store float %_0.i317.us.i, ptr %_149.i.us.i, align 4, !dbg !2745, !alias.scope !2747, !noalias !2663
  %exitcond2635.not.i = icmp eq i64 %iter.sroa.0.0.i1071.us.i, %empty.sroa.6.0.i.i, !dbg !3392
  br i1 %exitcond2635.not.i, label %bb53.i.i, label %bb52.i.us.i, !dbg !3392, !prof !180

bb52.i.us.i:                                      ; preds = %bb50.i.us.i
  %_158.not.not.i.us.i = icmp ugt i64 %_59.1.i, %_29.i.us.i, !dbg !3390
  br i1 %_158.not.not.i.us.i, label %bb54.i.us.i, label %bb55.i.i, !dbg !3390, !prof !2704

bb54.i.us.i:                                      ; preds = %bb52.i.us.i
  %_157.i.us.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.i, !dbg !2750
  %_0.i315.us.i = load float, ptr %_157.i.us.i, align 4, !dbg !2757, !alias.scope !2759, !noalias !2663, !noundef !12
  %_165.i.us.i = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_29.i.us.i, !dbg !2762
  store float %_0.i315.us.i, ptr %_165.i.us.i, align 4, !dbg !2769, !alias.scope !2771, !noalias !2663
  %_174.not.not.i.us.i = icmp ugt i64 %_61.1.i, %_29.i.us.i, !dbg !3387
  br i1 %_174.not.not.i.us.i, label %bb58.i.us.i, label %bb59.i.i, !dbg !3387, !prof !2704

bb58.i.us.i:                                      ; preds = %bb54.i.us.i
  %_173.i.us.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i, i64 %iter.sroa.0.0.i1071.us.i, !dbg !2774
  %_0.i313.us.i = load float, ptr %_173.i.us.i, align 4, !dbg !2781, !alias.scope !2783, !noalias !2663, !noundef !12
  %_181.i.us.i = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_29.i.us.i, !dbg !2786
  store float %_0.i313.us.i, ptr %_181.i.us.i, align 4, !dbg !2793, !alias.scope !2795, !noalias !2663
  %_52.i.us.i = sub i32 %now.i.us.i, %_53.i, !dbg !2798
  %_51.i.us.i = and i32 %_52.i.us.i, %_52.i, !dbg !2801
  %_50.i.us.i = zext i32 %_51.i.us.i to i64, !dbg !2802
  %_182.not.not.i.us.i = icmp ugt i64 %_58.1.i, %_50.i.us.i, !dbg !2803
  br i1 %_182.not.not.i.us.i, label %bb60.i.us.i, label %bb61.i.i, !dbg !2803, !prof !2704

bb60.i.us.i:                                      ; preds = %bb58.i.us.i
  %_189.i.us.i = getelementptr inbounds nuw float, ptr %_58.0.i, i64 %_50.i.us.i, !dbg !2808
  %_0.i311.us.i = load float, ptr %_189.i.us.i, align 4, !dbg !2812, !alias.scope !2814, !noalias !2663, !noundef !12
  %_190.not.not.i.us.i = icmp ugt i64 %_60.1.i, %_50.i.us.i, !dbg !3395
  br i1 %_190.not.not.i.us.i, label %bb63.i.us.i, label %bb64.i.i, !dbg !3395, !prof !2704

bb63.i.us.i:                                      ; preds = %bb60.i.us.i
  %_195.i.us.i = getelementptr inbounds nuw float, ptr %_60.0.i, i64 %_50.i.us.i, !dbg !2817
  %_0.i309.us.i = load float, ptr %_195.i.us.i, align 4, !dbg !2825, !alias.scope !2827, !noalias !2663, !noundef !12
  %_67.i.us.i = load i32, ptr %_45.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_66.i.us.i = sub i32 %now.i.us.i, %_67.i.us.i
  %_65.i.us.i = and i32 %_66.i.us.i, %_52.i
  %_64.i.us.i = zext i32 %_65.i.us.i to i64
  %_75.i.us.i = load i32, ptr %_50.i, align 4, !alias.scope !2593, !noalias !2663, !noundef !12
  %_74.i.us.i = sub i32 %now.i.us.i, %_75.i.us.i
  %_73.i.us.i = and i32 %_74.i.us.i, %_52.i
  %_72.i.us.i = zext i32 %_73.i.us.i to i64
  %_80.i.us.i = icmp ugt i64 %_59.1.i, %_64.i.us.i
  %265 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_64.i.us.i
  %266 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_64.i.us.i
  %_86.i.us.i = icmp ugt i64 %_61.1.i, %_72.i.us.i
  %267 = getelementptr inbounds nuw float, ptr %_61.0.i, i64 %_72.i.us.i
  %_88.i.us.i = icmp ugt i64 %_59.1.i, %_72.i.us.i
  %268 = getelementptr inbounds nuw float, ptr %_59.0.i, i64 %_72.i.us.i
  br i1 %_80.i.us.i, label %bb63.i.split.us.us.i, label %bb63.i.split.i

bb63.i.split.us.us.i:                             ; preds = %bb63.i.us.i
  %_84.i.us.i = icmp ugt i64 %_61.1.i, %_64.i.us.i
  br i1 %_84.i.us.i, label %bb24.i.us.lr.ph.us.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i, !dbg !2830

bb24.i.us.lr.ph.us.i:                             ; preds = %bb63.i.split.us.us.i
  %_82.i.us.us.i = load float, ptr %266, align 4, !noalias !2663, !noundef !12
  br i1 %_86.i.us.i, label %bb24.i.us.lr.ph.split.us.us.i, label %bb24.i.us.lr.ph.split.i

bb24.i.us.lr.ph.split.us.us.i:                    ; preds = %bb24.i.us.lr.ph.us.i
  br i1 %_88.i.us.i, label %bb28.i.us.us.lr.ph.us.i, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i, !dbg !2838

bb28.i.us.us.lr.ph.us.i:                          ; preds = %bb24.i.us.lr.ph.split.us.us.i
  %_87.i.us.us.us.i = load float, ptr %268, align 4, !noalias !2663, !noundef !12
  %_85.i.us.us.le931.us.i = load float, ptr %267, align 4, !noalias !2663, !noundef !12
  %_78.i.us.le.us.i = load float, ptr %265, align 4, !noalias !2663, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2839), !dbg !2842
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2843), !dbg !2842
  %_12.i39.us.i = load float, ptr %28, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.i = fcmp ule float %_12.i39.us.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.i = fcmp une float %_12.i39.us.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.i = load float, ptr %_10.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.i = load float, ptr %29, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.i = fadd float %_16.i42.us.i, %_17.i43.us.i, !dbg !2864
  %_20.i45655658.us.i = load float, ptr %30, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %269 = select i1 %_3.i196.us.i, float %_0.i255.us.i, float %_20.i45655658.us.i, !dbg !2869
  %_0.i407.us.i = select i1 %_3.i224.us.i, float %_16.i42.us.i, float %269, !dbg !2872
  store float %_0.i407.us.i, ptr %_10.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.i = select i1 %_3.i196.us.i, float %_17.i43.us.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.i, ptr %29, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.i = fadd float %_12.i39.us.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.i = select i1 %_3.i224.us.i, float %_12.i39.us.i, float %_0.i293.us.i, !dbg !2881
  store float %_4.i393.v.us.i, ptr %28, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.1.i = load float, ptr %31, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.1.i = fcmp ule float %_12.i39.us.1.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.1.i = fcmp une float %_12.i39.us.1.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.1.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.1.i = load float, ptr %32, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.1.i = fadd float %_16.i42.us.1.i, %_17.i43.us.1.i, !dbg !2864
  %_20.i45655658.us.1.i = load float, ptr %33, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %270 = select i1 %_3.i196.us.1.i, float %_0.i255.us.1.i, float %_20.i45655658.us.1.i, !dbg !2869
  %_0.i407.us.1.i = select i1 %_3.i224.us.1.i, float %_16.i42.us.1.i, float %270, !dbg !2872
  store float %_0.i407.us.1.i, ptr %iter1.sroa.0.0.ptr.i37.us.1.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.1.i = select i1 %_3.i196.us.1.i, float %_17.i43.us.1.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.1.i, ptr %32, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.1.i = fadd float %_12.i39.us.1.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.1.i = select i1 %_3.i224.us.1.i, float %_12.i39.us.1.i, float %_0.i293.us.1.i, !dbg !2881
  store float %_4.i393.v.us.1.i, ptr %31, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.2.i = load float, ptr %34, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.2.i = fcmp ule float %_12.i39.us.2.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.2.i = fcmp une float %_12.i39.us.2.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.2.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.2.i = load float, ptr %35, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.2.i = fadd float %_16.i42.us.2.i, %_17.i43.us.2.i, !dbg !2864
  %_20.i45655658.us.2.i = load float, ptr %36, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %271 = select i1 %_3.i196.us.2.i, float %_0.i255.us.2.i, float %_20.i45655658.us.2.i, !dbg !2869
  %_0.i407.us.2.i = select i1 %_3.i224.us.2.i, float %_16.i42.us.2.i, float %271, !dbg !2872
  store float %_0.i407.us.2.i, ptr %iter1.sroa.0.0.ptr.i37.us.2.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.2.i = select i1 %_3.i196.us.2.i, float %_17.i43.us.2.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.2.i, ptr %35, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.2.i = fadd float %_12.i39.us.2.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.2.i = select i1 %_3.i224.us.2.i, float %_12.i39.us.2.i, float %_0.i293.us.2.i, !dbg !2881
  store float %_4.i393.v.us.2.i, ptr %34, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %_12.i39.us.3.i = load float, ptr %37, align 4, !dbg !2845, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i224.us.3.i = fcmp ule float %_12.i39.us.3.i, 0.000000e+00, !dbg !2854
  %_3.i196.us.3.i = fcmp une float %_12.i39.us.3.i, 1.000000e+00, !dbg !2857
  %_16.i42.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i37.us.3.i, align 4, !dbg !2861, !alias.scope !2852, !noalias !2853, !noundef !12
  %_17.i43.us.3.i = load float, ptr %38, align 4, !dbg !2863, !alias.scope !2852, !noalias !2853, !noundef !12
  %_0.i255.us.3.i = fadd float %_16.i42.us.3.i, %_17.i43.us.3.i, !dbg !2864
  %_20.i45655658.us.3.i = load float, ptr %39, align 4, !dbg !2867, !alias.scope !2852, !noalias !2853, !noundef !12
  %272 = select i1 %_3.i196.us.3.i, float %_0.i255.us.3.i, float %_20.i45655658.us.3.i, !dbg !2869
  %_0.i407.us.3.i = select i1 %_3.i224.us.3.i, float %_16.i42.us.3.i, float %272, !dbg !2872
  store float %_0.i407.us.3.i, ptr %iter1.sroa.0.0.ptr.i37.us.3.i, align 4, !dbg !2874, !alias.scope !2852, !noalias !2853
  %_0.i400.us.3.i = select i1 %_3.i196.us.3.i, float %_17.i43.us.3.i, float 0.000000e+00, !dbg !2875
  store float %_0.i400.us.3.i, ptr %38, align 4, !dbg !2877, !alias.scope !2852, !noalias !2853
  %_0.i293.us.3.i = fadd float %_12.i39.us.3.i, -1.000000e+00, !dbg !2878
  %_4.i393.v.us.3.i = select i1 %_3.i224.us.3.i, float %_12.i39.us.3.i, float %_0.i293.us.3.i, !dbg !2881
  store float %_4.i393.v.us.3.i, ptr %37, align 4, !dbg !2883, !alias.scope !2852, !noalias !2853
  %273 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.i), !dbg !2884
  %274 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.i), !dbg !2891
  %_37.i60.us.i = load float, ptr %13, align 4, !dbg !2894, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i222.us.i = fcmp ule float %_37.i60.us.i, 0.000000e+00, !dbg !2899
  %_3.i.i560.us.i = fcmp ule float %273, %274, !dbg !2901
  %_6.i.i562.us.i = bitcast float %273 to i32, !dbg !2907
  %_8.i.i564.us.i = bitcast float %274 to i32, !dbg !2911
  %_4.i.i567.us.i = select i1 %_3.i.i560.us.i, i32 %_8.i.i564.us.i, i32 %_6.i.i562.us.i, !dbg !2913
  %_4.i386.us.i = select i1 %_3.i222.us.i, i32 %_6.i.i562.us.i, i32 %_4.i.i567.us.i, !dbg !2914
  %_41.i64.us.i = load float, ptr %14, align 4, !dbg !2916, !alias.scope !2897, !noalias !2898, !noundef !12
  %_3.i220.us.i = fcmp ule float %_41.i64.us.i, 0.000000e+00, !dbg !2917
  %_0.i277.us.i = fmul float %273, 5.000000e-01, !dbg !2919
  %_0.i276.us.i = fmul float %274, 5.000000e-01, !dbg !2922
  %_0.i254.us.i = fadd float %_0.i276.us.i, %_0.i277.us.i, !dbg !2924
  %_6.i374.us.i = bitcast float %_0.i254.us.i to i32, !dbg !2926
  %_4.i379.us.i = select i1 %_3.i220.us.i, i32 %_4.i386.us.i, i32 %_6.i374.us.i, !dbg !2929
  %_0.i380.us.i = bitcast i32 %_4.i379.us.i to float, !dbg !2930
  %_3.i.i552.us.i = fcmp ule float %_0.i380.us.i, 0x3E45798EE0000000, !dbg !2933
  %_4.i.i558.us.i = select i1 %_3.i.i552.us.i, i32 841731191, i32 %_4.i379.us.i, !dbg !2936
  %_0.i.i559.us.i = bitcast i32 %_4.i.i558.us.i to float, !dbg !2938
  %_3.i.i512.us.i = fcmp ule float %_0.i.i559.us.i, 0x3810000000000000, !dbg !2940
  %_4.i.i518.us.i = select i1 %_3.i.i512.us.i, i32 8388608, i32 %_4.i.i558.us.i, !dbg !2950
  %_5.i324.us.i = and i32 %_4.i.i518.us.i, 8388607, !dbg !2952
  %_4.i325.us.i = or disjoint i32 %_5.i324.us.i, 1065353216, !dbg !2952
  %significand.i326.us.i = bitcast i32 %_4.i325.us.i to float, !dbg !2957
  %_0.i285.us.i = fadd float %significand.i326.us.i, -1.000000e+00, !dbg !2960
  %_0.i265.us.i = fmul float %_0.i285.us.i, 0x3F9B17A960000000, !dbg !2963
  %275 = fsub float 0x3FBF9A8440000000, %_0.i265.us.i, !dbg !2968
  %_0.i265.us.1.i = fmul float %_0.i285.us.i, %275, !dbg !2963
  %_0.i249.us.1.i = fadd float %_0.i265.us.1.i, 0xBFD1E3F400000000, !dbg !2968
  %_0.i265.us.2.i = fmul float %_0.i285.us.i, %_0.i249.us.1.i, !dbg !2963
  %_0.i249.us.2.i = fadd float %_0.i265.us.2.i, 0x3FDD544F20000000, !dbg !2968
  %_0.i265.us.3.i = fmul float %_0.i285.us.i, %_0.i249.us.2.i, !dbg !2963
  %_0.i249.us.3.i = fadd float %_0.i265.us.3.i, 0xBFE6FC2A60000000, !dbg !2968
  %_0.i265.us.4.i = fmul float %_0.i285.us.i, %_0.i249.us.3.i, !dbg !2963
  %_0.i249.us.4.i = fadd float %_0.i265.us.4.i, 0x3FF714B2A0000000, !dbg !2968
  %_9.i327.us.i = lshr i32 %_4.i.i518.us.i, 23, !dbg !2970
  %_8.i328.us.i = or disjoint i32 %_9.i327.us.i, 1258291200, !dbg !2970
  %_7.i329.us.i = bitcast i32 %_8.i328.us.i to float, !dbg !2972
  %exponent.i330.us.i = fadd float %_7.i329.us.i, 0xC160000FE0000000, !dbg !2974
  %_0.i264.us.i = fmul float %_0.i285.us.i, %_0.i249.us.4.i, !dbg !2975
  %_0.i248.us.i = fadd float %exponent.i330.us.i, %_0.i264.us.i, !dbg !2977
  %_0.i275.us.i = fmul float %_0.i248.us.i, 0x4018151820000000, !dbg !2979
  %_3.i.i627.us.inv.i = fcmp olt float %_0.i275.us.i, 2.400000e+01, !dbg !2981
  %_0.i.i634.us.i = select i1 %_3.i.i627.us.inv.i, float %_0.i275.us.i, float 2.400000e+01, !dbg !2981
  %_3.i.i544.us.inv.i = fcmp ogt float %_0.i.i634.us.i, -1.600000e+02, !dbg !2985
  %_0.i.i551.us.i = select i1 %_3.i.i544.us.inv.i, float %_0.i.i634.us.i, float -1.600000e+02, !dbg !2985
  %_55.i81.us.i = load float, ptr %12, align 4, !dbg !2988, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i218.us.i = fcmp ule float %_55.i81.us.i, 0.000000e+00, !dbg !2990
  %_3.i204.us.i = fcmp oge float %_0.i.i551.us.i, %_0.i407.us.i, !dbg !2992
  %_0.i292.us.i = fsub float %_0.i407.us.i, %_0.i407.us.3.i, !dbg !2996
  %_3.i202.us.i = fcmp oge float %_0.i.i551.us.i, %_0.i292.us.i, !dbg !2999
  %..i203.us.i = sext i1 %_3.i202.us.i to i32, !dbg !3001
  %_0.i502.us.i = sext i1 %_3.i204.us.i to i32, !dbg !3004
  %_0.i496.us.i = select i1 %_3.i218.us.i, i32 %_0.i502.us.i, i32 %..i203.us.i, !dbg !3004
  %_0.i508.us.i = xor i32 %..i203.us.i, -1, !dbg !3010
  %_67.i91.us.i = load float, ptr %15, align 4, !dbg !3014, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i216.us.i = fcmp ogt float %_67.i91.us.i, 0.000000e+00, !dbg !3015
  %_0.i501.us.i = select i1 %_3.i216.us.i, i32 %_0.i508.us.i, i32 0, !dbg !3017
  %_0.i500.us.i = select i1 %_3.i218.us.i, i32 0, i32 %_0.i501.us.i, !dbg !3020
  %_0.i495.us.i = or i32 %_0.i500.us.i, %_0.i496.us.i, !dbg !3022
  %_5.i369.us.i = and i32 %_0.i495.us.i, 1065353216, !dbg !3025
  %_0.i373.us.i = bitcast i32 %_5.i369.us.i to float, !dbg !3027
  %_71.i97667668.us.i = load float, ptr %16, align 4, !dbg !3029, !alias.scope !2897, !noalias !2898, !noundef !12
  %_0.i291.us.i = fadd float %_67.i91.us.i, -1.000000e+00, !dbg !3031
  %276 = trunc nsw i32 %_0.i500.us.i to i1, !dbg !3033
  %_4.i367.v.us.i = select i1 %276, float %_0.i291.us.i, float %_67.i91.us.i, !dbg !3033
  %277 = trunc nsw i32 %_0.i496.us.i to i1, !dbg !3035
  %_0.i361.us.i = select i1 %277, float %_71.i97667668.us.i, float %_4.i367.v.us.i, !dbg !3035
  store float %_0.i361.us.i, ptr %15, align 4, !dbg !3037, !alias.scope !2852, !noalias !2853
  store i32 %_5.i369.us.i, ptr %12, align 4, !dbg !3038, !alias.scope !2852, !noalias !2853
  %_0.i290.us.i = fadd float %_0.i407.us.1.i, -1.000000e+00, !dbg !3039
  %_0.i289.us.i = fsub float %_0.i.i551.us.i, %_0.i407.us.i, !dbg !3041
  %_0.i274.us.i = fmul float %_0.i290.us.i, %_0.i289.us.i, !dbg !3043
  %278 = fneg float %_0.i407.us.2.i, !dbg !3045
  %_3.i.i536.inv.us.i = fcmp ogt float %_0.i274.us.i, %278, !dbg !3048
  %_4.i.i542.v.us.i = select i1 %_3.i.i536.inv.us.i, float %_0.i274.us.i, float %278, !dbg !3048
  %_3.i.i619.us.i = fcmp olt float %_4.i.i542.v.us.i, 0.000000e+00, !dbg !3051
  %279 = fcmp ule float %_0.i373.us.i, 0.000000e+00, !dbg !3055
  %280 = select i1 %279, i1 %_3.i.i619.us.i, i1 false, !dbg !3058
  %_0.i354.us.i = select i1 %280, float %_4.i.i542.v.us.i, float 0.000000e+00, !dbg !3058
  %_86.i110.us.i = load float, ptr %17, align 4, !dbg !3059, !alias.scope !2852, !noalias !2853, !noundef !12
  %_3.i212.us.i = fcmp ule float %_0.i354.us.i, %_86.i110.us.i, !dbg !3061
  %_87.i112670.us.i = load i32, ptr %2, align 4, !dbg !3063, !alias.scope !2897, !noalias !2898, !noundef !12
  %_88.i113671.us.i = load i32, ptr %18, align 4, !dbg !3064, !alias.scope !2897, !noalias !2898, !noundef !12
  %_4.i347.us.i = select i1 %_3.i212.us.i, i32 %_88.i113671.us.i, i32 %_87.i112670.us.i, !dbg !3065
  %_0.i348.us.i = bitcast i32 %_4.i347.us.i to float, !dbg !3067
  %_0.i288.us.i = fsub float %_0.i354.us.i, %_86.i110.us.i, !dbg !3069
  %_4.i258.us.i = fmul float %_0.i288.us.i, %_0.i348.us.i, !dbg !3072
  %_0.i259.us.i = fadd float %_86.i110.us.i, %_4.i258.us.i, !dbg !3072
  %281 = tail call noundef float @llvm.fabs.f32(float %_0.i259.us.i), !dbg !3075
  %282 = fcmp uge float %281, 0x3BC79CA100000000, !dbg !3079
  %_0.i332.us.i = select i1 %282, float %_0.i259.us.i, float 0.000000e+00, !dbg !3082
  store float %_0.i332.us.i, ptr %17, align 4, !dbg !3083, !alias.scope !2852, !noalias !2853
  %_0.i273.us.i = fmul float %_0.i332.us.i, 0x3FC542A5A0000000, !dbg !3085
  %_3.i.i528.us.inv.i = fcmp ogt float %_0.i273.us.i, -1.260000e+02, !dbg !3089
  %_0.i.i535.us.i = select i1 %_3.i.i528.us.inv.i, float %_0.i273.us.i, float -1.260000e+02, !dbg !3089
  %_3.i.i611.us.inv.i = fcmp olt float %_0.i.i535.us.i, 1.270000e+02, !dbg !3094
  %_0.i.i618.us.i = select i1 %_3.i.i611.us.inv.i, float %_0.i.i535.us.i, float 1.270000e+02, !dbg !3094
  %283 = tail call noundef float @llvm.floor.f32(float %_0.i.i618.us.i), !dbg !3097
  %_0.i287.us.i = fsub float %_0.i.i618.us.i, %283, !dbg !3109
  %_98.i126.us.i = load float, ptr %19, align 4, !dbg !3112, !alias.scope !2897, !noalias !2898, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3114), !dbg !3117
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3119), !dbg !3117
  %_12.i.us.i = load float, ptr %40, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.i = fcmp ule float %_12.i.us.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.i = fcmp une float %_12.i.us.i, 1.000000e+00, !dbg !3127
  %_16.i.us.i = load float, ptr %data.i.i.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.i = load float, ptr %41, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.i = fadd float %_16.i.us.i, %_17.i.us.i, !dbg !3131
  %_20.i677680.us.i = load float, ptr %42, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %284 = select i1 %_3.i200.us.i, float %_0.i257.us.i, float %_20.i677680.us.i, !dbg !3134
  %_0.i486.us.i = select i1 %_3.i240.us.i, float %_16.i.us.i, float %284, !dbg !3136
  store float %_0.i486.us.i, ptr %data.i.i.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.i = select i1 %_3.i200.us.i, float %_17.i.us.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.i, ptr %41, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.i = fadd float %_12.i.us.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.i = select i1 %_3.i240.us.i, float %_12.i.us.i, float %_0.i299.us.i, !dbg !3144
  store float %_4.i472.v.us.i, ptr %40, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.1.i = load float, ptr %43, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.1.i = fcmp ule float %_12.i.us.1.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.1.i = fcmp une float %_12.i.us.1.i, 1.000000e+00, !dbg !3127
  %_16.i.us.1.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.1.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.1.i = load float, ptr %44, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.1.i = fadd float %_16.i.us.1.i, %_17.i.us.1.i, !dbg !3131
  %_20.i677680.us.1.i = load float, ptr %45, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %285 = select i1 %_3.i200.us.1.i, float %_0.i257.us.1.i, float %_20.i677680.us.1.i, !dbg !3134
  %_0.i486.us.1.i = select i1 %_3.i240.us.1.i, float %_16.i.us.1.i, float %285, !dbg !3136
  store float %_0.i486.us.1.i, ptr %iter1.sroa.0.0.ptr.i.us.1.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.1.i = select i1 %_3.i200.us.1.i, float %_17.i.us.1.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.1.i, ptr %44, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.1.i = fadd float %_12.i.us.1.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.1.i = select i1 %_3.i240.us.1.i, float %_12.i.us.1.i, float %_0.i299.us.1.i, !dbg !3144
  store float %_4.i472.v.us.1.i, ptr %43, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.2.i = load float, ptr %46, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.2.i = fcmp ule float %_12.i.us.2.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.2.i = fcmp une float %_12.i.us.2.i, 1.000000e+00, !dbg !3127
  %_16.i.us.2.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.2.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.2.i = load float, ptr %47, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.2.i = fadd float %_16.i.us.2.i, %_17.i.us.2.i, !dbg !3131
  %_20.i677680.us.2.i = load float, ptr %48, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %286 = select i1 %_3.i200.us.2.i, float %_0.i257.us.2.i, float %_20.i677680.us.2.i, !dbg !3134
  %_0.i486.us.2.i = select i1 %_3.i240.us.2.i, float %_16.i.us.2.i, float %286, !dbg !3136
  store float %_0.i486.us.2.i, ptr %iter1.sroa.0.0.ptr.i.us.2.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.2.i = select i1 %_3.i200.us.2.i, float %_17.i.us.2.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.2.i, ptr %47, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.2.i = fadd float %_12.i.us.2.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.2.i = select i1 %_3.i240.us.2.i, float %_12.i.us.2.i, float %_0.i299.us.2.i, !dbg !3144
  store float %_4.i472.v.us.2.i, ptr %46, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %_12.i.us.3.i = load float, ptr %49, align 4, !dbg !3121, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i240.us.3.i = fcmp ule float %_12.i.us.3.i, 0.000000e+00, !dbg !3125
  %_3.i200.us.3.i = fcmp une float %_12.i.us.3.i, 1.000000e+00, !dbg !3127
  %_16.i.us.3.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.3.i, align 4, !dbg !3129, !alias.scope !3123, !noalias !3124, !noundef !12
  %_17.i.us.3.i = load float, ptr %50, align 4, !dbg !3130, !alias.scope !3123, !noalias !3124, !noundef !12
  %_0.i257.us.3.i = fadd float %_16.i.us.3.i, %_17.i.us.3.i, !dbg !3131
  %_20.i677680.us.3.i = load float, ptr %51, align 4, !dbg !3133, !alias.scope !3123, !noalias !3124, !noundef !12
  %287 = select i1 %_3.i200.us.3.i, float %_0.i257.us.3.i, float %_20.i677680.us.3.i, !dbg !3134
  %_0.i486.us.3.i = select i1 %_3.i240.us.3.i, float %_16.i.us.3.i, float %287, !dbg !3136
  store float %_0.i486.us.3.i, ptr %iter1.sroa.0.0.ptr.i.us.3.i, align 4, !dbg !3138, !alias.scope !3123, !noalias !3124
  %_0.i479.us.3.i = select i1 %_3.i200.us.3.i, float %_17.i.us.3.i, float 0.000000e+00, !dbg !3139
  store float %_0.i479.us.3.i, ptr %50, align 4, !dbg !3141, !alias.scope !3123, !noalias !3124
  %_0.i299.us.3.i = fadd float %_12.i.us.3.i, -1.000000e+00, !dbg !3142
  %_4.i472.v.us.3.i = select i1 %_3.i240.us.3.i, float %_12.i.us.3.i, float %_0.i299.us.3.i, !dbg !3144
  store float %_4.i472.v.us.3.i, ptr %49, align 4, !dbg !3146, !alias.scope !3123, !noalias !3124
  %288 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le931.us.i), !dbg !3147
  %289 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.i), !dbg !3149
  %_37.i.us.i = load float, ptr %21, align 4, !dbg !3151, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i238.us.i = fcmp ule float %_37.i.us.i, 0.000000e+00, !dbg !3154
  %_3.i.i594.us.i = fcmp ule float %288, %289, !dbg !3156
  %_6.i.i596.us.i = bitcast float %288 to i32, !dbg !3159
  %_8.i.i598.us.i = bitcast float %289 to i32, !dbg !3162
  %_4.i.i601.us.i = select i1 %_3.i.i594.us.i, i32 %_8.i.i598.us.i, i32 %_6.i.i596.us.i, !dbg !3164
  %_4.i465.us.i = select i1 %_3.i238.us.i, i32 %_6.i.i596.us.i, i32 %_4.i.i601.us.i, !dbg !3165
  %_41.i.us.i = load float, ptr %22, align 4, !dbg !3167, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i236.us.i = fcmp ule float %_41.i.us.i, 0.000000e+00, !dbg !3168
  %_0.i283.us.i = fmul float %288, 5.000000e-01, !dbg !3170
  %_0.i282.us.i = fmul float %289, 5.000000e-01, !dbg !3172
  %_0.i256.us.i = fadd float %_0.i282.us.i, %_0.i283.us.i, !dbg !3174
  %_6.i453.us.i = bitcast float %_0.i256.us.i to i32, !dbg !3176
  %_4.i458.us.i = select i1 %_3.i236.us.i, i32 %_4.i465.us.i, i32 %_6.i453.us.i, !dbg !3179
  %_0.i459.us.i = bitcast i32 %_4.i458.us.i to float, !dbg !3180
  %_3.i.i586.us.i = fcmp ule float %_0.i459.us.i, 0x3E45798EE0000000, !dbg !3182
  %_4.i.i592.us.i = select i1 %_3.i.i586.us.i, i32 841731191, i32 %_4.i458.us.i, !dbg !3185
  %_0.i.i593.us.i = bitcast i32 %_4.i.i592.us.i to float, !dbg !3187
  %_3.i.i.us.i = fcmp ule float %_0.i.i593.us.i, 0x3810000000000000, !dbg !3189
  %_4.i.i.us.i = select i1 %_3.i.i.us.i, i32 8388608, i32 %_4.i.i592.us.i, !dbg !3194
  %_5.i320.us.i = and i32 %_4.i.i.us.i, 8388607, !dbg !3196
  %_4.i321.us.i = or disjoint i32 %_5.i320.us.i, 1065353216, !dbg !3196
  %significand.i.us.i = bitcast i32 %_4.i321.us.i to float, !dbg !3198
  %_0.i284.us.i = fadd float %significand.i.us.i, -1.000000e+00, !dbg !3200
  %_0.i263.us.i = fmul float %_0.i284.us.i, 0x3F9B17A960000000, !dbg !3202
  %290 = fsub float 0x3FBF9A8440000000, %_0.i263.us.i, !dbg !3204
  %_0.i263.us.1.i = fmul float %_0.i284.us.i, %290, !dbg !3202
  %_0.i247.us.1.i = fadd float %_0.i263.us.1.i, 0xBFD1E3F400000000, !dbg !3204
  %_0.i263.us.2.i = fmul float %_0.i284.us.i, %_0.i247.us.1.i, !dbg !3202
  %_0.i247.us.2.i = fadd float %_0.i263.us.2.i, 0x3FDD544F20000000, !dbg !3204
  %_0.i263.us.3.i = fmul float %_0.i284.us.i, %_0.i247.us.2.i, !dbg !3202
  %_0.i247.us.3.i = fadd float %_0.i263.us.3.i, 0xBFE6FC2A60000000, !dbg !3204
  %_0.i263.us.4.i = fmul float %_0.i284.us.i, %_0.i247.us.3.i, !dbg !3202
  %_0.i247.us.4.i = fadd float %_0.i263.us.4.i, 0x3FF714B2A0000000, !dbg !3204
  %_9.i.us.i = lshr i32 %_4.i.i.us.i, 23, !dbg !3206
  %_8.i322.us.i = or disjoint i32 %_9.i.us.i, 1258291200, !dbg !3206
  %_7.i.us.i = bitcast i32 %_8.i322.us.i to float, !dbg !3207
  %exponent.i.us.i = fadd float %_7.i.us.i, 0xC160000FE0000000, !dbg !3209
  %_0.i262.us.i = fmul float %_0.i284.us.i, %_0.i247.us.4.i, !dbg !3210
  %_0.i246.us.i = fadd float %exponent.i.us.i, %_0.i262.us.i, !dbg !3212
  %_0.i281.us.i = fmul float %_0.i246.us.i, 0x4018151820000000, !dbg !3214
  %_3.i.i643.us.inv.i = fcmp olt float %_0.i281.us.i, 2.400000e+01, !dbg !3216
  %_0.i.i650.us.i = select i1 %_3.i.i643.us.inv.i, float %_0.i281.us.i, float 2.400000e+01, !dbg !3216
  %_3.i.i578.us.inv.i = fcmp ogt float %_0.i.i650.us.i, -1.600000e+02, !dbg !3219
  %_0.i.i585.us.i = select i1 %_3.i.i578.us.inv.i, float %_0.i.i650.us.i, float -1.600000e+02, !dbg !3219
  %_55.i12.us.i = load float, ptr %20, align 4, !dbg !3222, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i234.us.i = fcmp ule float %_55.i12.us.i, 0.000000e+00, !dbg !3223
  %_3.i208.us.i = fcmp oge float %_0.i.i585.us.i, %_0.i486.us.i, !dbg !3225
  %_0.i298.us.i = fsub float %_0.i486.us.i, %_0.i486.us.3.i, !dbg !3227
  %_3.i206.us.i = fcmp oge float %_0.i.i585.us.i, %_0.i298.us.i, !dbg !3229
  %..i207.us.i = sext i1 %_3.i206.us.i to i32, !dbg !3231
  %_0.i506.us.i = sext i1 %_3.i208.us.i to i32, !dbg !3233
  %_0.i499.us.i = select i1 %_3.i234.us.i, i32 %_0.i506.us.i, i32 %..i207.us.i, !dbg !3233
  %_0.i510.us.i = xor i32 %..i207.us.i, -1, !dbg !3235
  %_67.i14.us.i = load float, ptr %23, align 4, !dbg !3237, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i232.us.i = fcmp ogt float %_67.i14.us.i, 0.000000e+00, !dbg !3238
  %_0.i505.us.i = select i1 %_3.i232.us.i, i32 %_0.i510.us.i, i32 0, !dbg !3240
  %_0.i504.us.i = select i1 %_3.i234.us.i, i32 0, i32 %_0.i505.us.i, !dbg !3242
  %_0.i498.us.i = or i32 %_0.i504.us.i, %_0.i499.us.i, !dbg !3244
  %_5.i448.us.i = and i32 %_0.i498.us.i, 1065353216, !dbg !3246
  %_0.i452.us.i = bitcast i32 %_5.i448.us.i to float, !dbg !3248
  %_71.i689690.us.i = load float, ptr %24, align 4, !dbg !3250, !alias.scope !3152, !noalias !3153, !noundef !12
  %_0.i297.us.i = fadd float %_67.i14.us.i, -1.000000e+00, !dbg !3251
  %291 = trunc nsw i32 %_0.i504.us.i to i1, !dbg !3253
  %_4.i446.v.us.i = select i1 %291, float %_0.i297.us.i, float %_67.i14.us.i, !dbg !3253
  %292 = trunc nsw i32 %_0.i499.us.i to i1, !dbg !3255
  %_0.i440.us.i = select i1 %292, float %_71.i689690.us.i, float %_4.i446.v.us.i, !dbg !3255
  store float %_0.i440.us.i, ptr %23, align 4, !dbg !3257, !alias.scope !3123, !noalias !3124
  store i32 %_5.i448.us.i, ptr %20, align 4, !dbg !3258, !alias.scope !3123, !noalias !3124
  %_0.i296.us.i = fadd float %_0.i486.us.1.i, -1.000000e+00, !dbg !3259
  %_0.i295.us.i = fsub float %_0.i.i585.us.i, %_0.i486.us.i, !dbg !3261
  %_0.i280.us.i = fmul float %_0.i296.us.i, %_0.i295.us.i, !dbg !3263
  %293 = fneg float %_0.i486.us.2.i, !dbg !3265
  %_3.i.i569.inv.us.i = fcmp ogt float %_0.i280.us.i, %293, !dbg !3267
  %_4.i.i576.v.us.i = select i1 %_3.i.i569.inv.us.i, float %_0.i280.us.i, float %293, !dbg !3267
  %_3.i.i635.us.i = fcmp olt float %_4.i.i576.v.us.i, 0.000000e+00, !dbg !3270
  %294 = fcmp ule float %_0.i452.us.i, 0.000000e+00, !dbg !3273
  %295 = select i1 %294, i1 %_3.i.i635.us.i, i1 false, !dbg !3275
  %_0.i433.us.i = select i1 %295, float %_4.i.i576.v.us.i, float 0.000000e+00, !dbg !3275
  %_86.i22.us.i = load float, ptr %25, align 4, !dbg !3276, !alias.scope !3123, !noalias !3124, !noundef !12
  %_3.i228.us.i = fcmp ule float %_0.i433.us.i, %_86.i22.us.i, !dbg !3277
  %_87.i24692.us.i = load i32, ptr %_32.i, align 4, !dbg !3279, !alias.scope !3152, !noalias !3153, !noundef !12
  %_88.i25693.us.i = load i32, ptr %26, align 4, !dbg !3280, !alias.scope !3152, !noalias !3153, !noundef !12
  %_4.i427.us.i = select i1 %_3.i228.us.i, i32 %_88.i25693.us.i, i32 %_87.i24692.us.i, !dbg !3281
  %_0.i.us.i = bitcast i32 %_4.i427.us.i to float, !dbg !3283
  %_0.i294.us.i = fsub float %_0.i433.us.i, %_86.i22.us.i, !dbg !3285
  %_4.i260.us.i = fmul float %_0.i294.us.i, %_0.i.us.i, !dbg !3287
  %_0.i261.us.i = fadd float %_86.i22.us.i, %_4.i260.us.i, !dbg !3287
  %296 = tail call noundef float @llvm.fabs.f32(float %_0.i261.us.i), !dbg !3289
  %297 = fcmp uge float %296, 0x3BC79CA100000000, !dbg !3292
  %_0.i336.us.i = select i1 %297, float %_0.i261.us.i, float 0.000000e+00, !dbg !3294
  store float %_0.i336.us.i, ptr %25, align 4, !dbg !3295, !alias.scope !3123, !noalias !3124
  %_0.i279.us.i = fmul float %_0.i336.us.i, 0x3FC542A5A0000000, !dbg !3296
  %_3.i.i520.us.inv.i = fcmp ogt float %_0.i279.us.i, -1.260000e+02, !dbg !3299
  %_0.i.i527.us.i = select i1 %_3.i.i520.us.inv.i, float %_0.i279.us.i, float -1.260000e+02, !dbg !3299
  %_3.i.i603.us.inv.i = fcmp olt float %_0.i.i527.us.i, 1.270000e+02, !dbg !3303
  %_0.i.i610.us.i = select i1 %_3.i.i603.us.inv.i, float %_0.i.i527.us.i, float 1.270000e+02, !dbg !3303
  %298 = tail call noundef float @llvm.floor.f32(float %_0.i.i610.us.i), !dbg !3306
  %_0.i286.us.i = fsub float %_0.i.i610.us.i, %298, !dbg !3310
  %_0.i268.us.i = fmul float %_0.i286.us.i, 0x3F5E974FA0000000, !dbg !3312
  %_0.i251.us.i = fadd float %_0.i268.us.i, 0x3F82778560000000, !dbg !3317
  %_0.i268.us.1.i = fmul float %_0.i286.us.i, %_0.i251.us.i, !dbg !3312
  %_0.i251.us.1.i = fadd float %_0.i268.us.1.i, 0x3FAC91CE60000000, !dbg !3317
  %_0.i268.us.2.i = fmul float %_0.i286.us.i, %_0.i251.us.1.i, !dbg !3312
  %_0.i251.us.2.i = fadd float %_0.i268.us.2.i, 0x3FCEBDB560000000, !dbg !3317
  %_0.i268.us.3.i = fmul float %_0.i286.us.i, %_0.i251.us.2.i, !dbg !3312
  %_0.i251.us.3.i = fadd float %_0.i268.us.3.i, 0x3FE62E4BA0000000, !dbg !3317
  %_0.i271.us.i = fmul float %_0.i287.us.i, 0x3F5E974FA0000000, !dbg !3319
  %_0.i253.us.i = fadd float %_0.i271.us.i, 0x3F82778560000000, !dbg !3321
  %_0.i271.us.1.i = fmul float %_0.i287.us.i, %_0.i253.us.i, !dbg !3319
  %_0.i253.us.1.i = fadd float %_0.i271.us.1.i, 0x3FAC91CE60000000, !dbg !3321
  %_0.i271.us.2.i = fmul float %_0.i287.us.i, %_0.i253.us.1.i, !dbg !3319
  %_0.i253.us.2.i = fadd float %_0.i271.us.2.i, 0x3FCEBDB560000000, !dbg !3321
  %_0.i271.us.3.i = fmul float %_0.i287.us.i, %_0.i253.us.2.i, !dbg !3319
  %_0.i253.us.3.i = fadd float %_0.i271.us.3.i, 0x3FE62E4BA0000000, !dbg !3321
  %_0.i270.us.i = fmul float %_0.i287.us.i, %_0.i253.us.3.i, !dbg !3323
  %_0.i252.us.i = fadd float %_0.i270.us.i, 1.000000e+00, !dbg !3325
  %biased.i191.us.i = fadd float %283, 0x4160000FE0000000, !dbg !3327
  %_4.i192.us.i = bitcast float %biased.i191.us.i to i32, !dbg !3331
  %_3.i193.us.i = shl i32 %_4.i192.us.i, 23, !dbg !3335
  %_0.i194.us.i = bitcast i32 %_3.i193.us.i to float, !dbg !3336
  %_0.i269.us.i = fmul float %_0.i252.us.i, %_0.i194.us.i, !dbg !3339
  %_3.i195.us.i = fcmp une float %_0.i332.us.i, 0.000000e+00, !dbg !3341
  %_3.i210.us.i = fcmp ule float %_98.i126.us.i, 0.000000e+00, !dbg !3343
  %_0.i494675.not.us.i = and i1 %_3.i210.us.i, %_3.i195.us.i, !dbg !3345
  %_0.i272.us.i = fmul float %_0.i311.us.i, %_0.i269.us.i, !dbg !3345
  %_4.i340.v.us.i = select i1 %_0.i494675.not.us.i, float %_0.i272.us.i, float %_0.i311.us.i, !dbg !3348
  %_0.i267.us.i = fmul float %_0.i286.us.i, %_0.i251.us.3.i, !dbg !3350
  %_0.i250.us.i = fadd float %_0.i267.us.i, 1.000000e+00, !dbg !3352
  %biased.i.us.i = fadd float %298, 0x4160000FE0000000, !dbg !3354
  %_4.i188.us.i = bitcast float %biased.i.us.i to i32, !dbg !3356
  %_3.i189.us.i = shl i32 %_4.i188.us.i, 23, !dbg !3358
  %_0.i190.us.i = bitcast i32 %_3.i189.us.i to float, !dbg !3359
  %_0.i266.us.i = fmul float %_0.i250.us.i, %_0.i190.us.i, !dbg !3361
  %_3.i198.us.i = fcmp une float %_0.i336.us.i, 0.000000e+00, !dbg !3363
  %_98.i.us.i = load float, ptr %27, align 4, !dbg !3365, !alias.scope !3152, !noalias !3153, !noundef !12
  %_3.i226.us.i = fcmp ule float %_98.i.us.i, 0.000000e+00, !dbg !3366
  %_0.i497697.not.us.i = and i1 %_3.i226.us.i, %_3.i198.us.i, !dbg !3368
  %_0.i278.us.i = fmul float %_0.i309.us.i, %_0.i266.us.i, !dbg !3368
  %_4.i420.v.us.i = select i1 %_0.i497697.not.us.i, float %_0.i278.us.i, float %_0.i309.us.i, !dbg !3370
  store float %_4.i340.v.us.i, ptr %_123.i.us.i, align 4, !dbg !3372, !alias.scope !3375, !noalias !2714
  store float %_4.i420.v.us.i, ptr %_141.i.us.i, align 4, !dbg !3378, !alias.scope !3380, !noalias !2737
  %exitcond2637.not.i = icmp eq i64 %264, %..i, !dbg !3383
  br i1 %exitcond2637.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit, label %bb40.i.us.i, !dbg !3386, !llvm.loop !3396

bb43.i.i:                                         ; preds = %bb40.i.us.us.us.us.us.us.us.i, %bb40.i.us.us.us.us.us.i, %bb40.i.us.us.us.i, %bb40.i.us.i
  %.us-phi1124.i = phi i64 [ %264, %bb40.i.us.i ], [ %194, %bb40.i.us.us.us.i ], [ %159, %bb40.i.us.us.us.us.us.i ], [ %124, %bb40.i.us.us.us.us.us.us.us.i ]
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 1, 2305843009213693952) %..i, i64 noundef %.us-phi1124.i, i64 noundef range(i64 1, 2305843009213693952) %..i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_97ed0782c9e7425b1b215f66cb64a347) #24, !dbg !3397, !noalias !2663
  unreachable, !dbg !3397

bb46.i.i:                                         ; preds = %bb42.i.us.us.us.us.us.us.us.i, %bb42.i.us.us.us.us.us.i, %bb42.i.us.us.us.i, %bb42.i.us.us.i, %bb42.i.us.i
  %.us-phi1130.i = phi i64 [ %_29.i.us.i, %bb42.i.us.i ], [ %_29.i.us.us.i, %bb42.i.us.us.i ], [ %_29.i.us.us.us.us.us.i, %bb42.i.us.us.us.us.us.i ], [ %_29.i.us.us.us.i, %bb42.i.us.us.us.i ], [ %_29.i.us.us.us.us.us.us.us.i, %bb42.i.us.us.us.us.us.us.us.i ]
  %_36.i.le1052.i = add nuw nsw i64 %.us-phi1130.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1130.i, i64 noundef %_36.i.le1052.i, i64 noundef %_58.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cad5a7b24766ffcdda9cbfe066eb4078) #24, !dbg !3398, !noalias !2663
  unreachable, !dbg !3398

bb51.i.i:                                         ; preds = %bb45.i.us.i
  %_36.i.le1050.i = add nuw nsw i64 %_29.i.us.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_29.i.us.i, i64 noundef %_36.i.le1050.i, i64 noundef %_60.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_72e24c5578221343e8a784545a3910d2) #24, !dbg !3399, !noalias !2663
  unreachable, !dbg !3399

bb53.i.i:                                         ; preds = %bb45.i.us.us.i, %bb50.i.us.i
  %.us-phi1142.i = phi i64 [ %264, %bb50.i.us.i ], [ %229, %bb45.i.us.us.i ]
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %empty.sroa.6.0.i.i, i64 noundef %.us-phi1142.i, i64 noundef %empty.sroa.6.0.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2f4f8781efb7cd41cc45e538dfda195) #24, !dbg !3400, !noalias !2663
  unreachable, !dbg !3400

bb55.i.i:                                         ; preds = %bb45.i.us.us.us.i, %bb52.i.us.us.i, %bb52.i.us.i
  %.us-phi1148.i = phi i64 [ %_29.i.us.i, %bb52.i.us.i ], [ %_29.i.us.us.i, %bb52.i.us.us.i ], [ %_29.i.us.us.us.i, %bb45.i.us.us.us.i ]
  %_36.i.le1048.i = add nuw nsw i64 %.us-phi1148.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1148.i, i64 noundef %_36.i.le1048.i, i64 noundef %_59.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8b70dfa50e86ad5a1ce9afbe0da1688) #24, !dbg !3401, !noalias !2663
  unreachable, !dbg !3401

bb59.i.i:                                         ; preds = %bb45.i.us.us.us.us.us.i, %bb54.i.us.us.us.i, %bb54.i.us.us.i, %bb54.i.us.i
  %.us-phi1160.i = phi i64 [ %_29.i.us.us.i, %bb54.i.us.us.i ], [ %_29.i.us.us.us.i, %bb54.i.us.us.us.i ], [ %_29.i.us.i, %bb54.i.us.i ], [ %_29.i.us.us.us.us.us.i, %bb45.i.us.us.us.us.us.i ]
  %_36.i.le.i = add nuw nsw i64 %.us-phi1160.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1160.i, i64 noundef %_36.i.le.i, i64 noundef %_61.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ad8d9ef662eaae13c96be161fe1fd2e1) #24, !dbg !3402, !noalias !2663
  unreachable, !dbg !3402

bb61.i.i:                                         ; preds = %bb45.i.us.us.us.us.us.us.us.i, %bb58.i.us.us.us.us.us.i, %bb58.i.us.us.us.i, %bb58.i.us.us.i, %bb58.i.us.i
  %.us-phi1166.i = phi i64 [ %_50.i.us.i, %bb58.i.us.i ], [ %_50.i.us.us.i, %bb58.i.us.us.i ], [ %_50.i.us.us.us.us.us.i, %bb58.i.us.us.us.us.us.i ], [ %_50.i.us.us.us.i, %bb58.i.us.us.us.i ], [ %_50.i.us.us.us.us.us.us.us.i, %bb45.i.us.us.us.us.us.us.us.i ]
  %_55.i.le1046.i = add nuw nsw i64 %.us-phi1166.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1166.i, i64 noundef %_55.i.le1046.i, i64 noundef %_58.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba99eeeb3482270ebe1c674b4013dd6a) #24, !dbg !3403, !noalias !2663
  unreachable, !dbg !3403

bb64.i.i:                                         ; preds = %bb60.i.us.i
  %_55.i.le.i = add nuw nsw i64 %_50.i.us.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_50.i.us.i, i64 noundef %_55.i.le.i, i64 noundef %_60.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f4c27429e4885e1b619f1e4a3587acc) #24, !dbg !3404, !noalias !2663
  unreachable, !dbg !3404

bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i: ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i, %bb24.i.us.lr.ph.split.us.us.us.us.i, %bb24.i.us.lr.ph.split.us.us.us.i, %bb24.i.us.lr.ph.split.us.us.i
  %.us-phi1225.i = phi i64 [ %_72.i.us.us.us.i, %bb24.i.us.lr.ph.split.us.us.us.us.i ], [ %_72.i.us.i, %bb24.i.us.lr.ph.split.us.us.i ], [ %_72.i.us.us.i, %bb24.i.us.lr.ph.split.us.us.us.i ], [ %_72.i.us.us.us.us.us.us.us.i, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1225.i, i64 noundef %_59.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5ae5decb60097d7e1183cb43cae6b3f) #24, !dbg !2838, !noalias !2663
  unreachable, !dbg !2838

bb24.i.us.lr.ph.split.i:                          ; preds = %bb24.i.us.lr.ph.us.us.us.us.us.us.us.i, %bb24.i.us.lr.ph.us.us.us.us.us.i, %bb24.i.us.lr.ph.us.us.us.i, %bb24.i.us.lr.ph.us.us.i, %bb24.i.us.lr.ph.us.i
  %.us-phi1207.i = phi i64 [ %_72.i.us.i, %bb24.i.us.lr.ph.us.i ], [ %_72.i.us.us.i, %bb24.i.us.lr.ph.us.us.i ], [ %_72.i.us.us.us.us.us.i, %bb24.i.us.lr.ph.us.us.us.us.us.i ], [ %_72.i.us.us.us.i, %bb24.i.us.lr.ph.us.us.us.i ], [ %_72.i.us.us.us.us.us.us.us.i, %bb24.i.us.lr.ph.us.us.us.us.us.us.us.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1207.i, i64 noundef %_61.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5833dd3f10f38ea96ec24c6054ff083c) #24, !dbg !3405, !noalias !2663
  unreachable, !dbg !3405

bb63.i.split.us.panic20.i.split.us_crit_edge.i:   ; preds = %bb63.i.split.us.us.us.us.us.us.us.us.i, %bb63.i.split.us.us.us.us.us.us.i, %bb63.i.split.us.us.us.us.i, %bb63.i.split.us.us.us.i, %bb63.i.split.us.us.i
  %.us-phi1201.i = phi i64 [ %_64.i.us.i, %bb63.i.split.us.us.i ], [ %_64.i.us.us.i, %bb63.i.split.us.us.us.i ], [ %_64.i.us.us.us.us.us.i, %bb63.i.split.us.us.us.us.us.us.i ], [ %_64.i.us.us.us.i, %bb63.i.split.us.us.us.us.i ], [ %_64.i.us.us.us.us.us.us.us.i, %bb63.i.split.us.us.us.us.us.us.us.us.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1201.i, i64 noundef %_61.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30bf8a301493a607abcc78dbf93977e0) #24, !dbg !2830, !noalias !2663
  unreachable, !dbg !2830

bb63.i.split.i:                                   ; preds = %bb60.i.us.us.us.us.us.us.us.i, %bb60.i.us.us.us.us.us.i, %bb60.i.us.us.us.i, %bb60.i.us.us.i, %bb63.i.us.i
  %.us-phi1179.i = phi i64 [ %_64.i.us.i, %bb63.i.us.i ], [ %_64.i.us.us.i, %bb60.i.us.us.i ], [ %_64.i.us.us.us.us.us.i, %bb60.i.us.us.us.us.us.i ], [ %_64.i.us.us.us.i, %bb60.i.us.us.us.i ], [ %_64.i.us.us.us.us.us.us.us.i, %bb60.i.us.us.us.us.us.us.us.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1179.i, i64 noundef %_59.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b53d553f9945f7702333f7af20204159) #24, !dbg !3406, !noalias !2663
  unreachable, !dbg !3406

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKBS_EB3_.exit: ; preds = %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i, %bb28.i.us.us.lr.ph.us.us.us.i, %bb28.i.us.us.lr.ph.us.us.i, %bb28.i.us.us.lr.ph.us.i
  %_108.i.i = trunc nuw i64 %..i to i32, !dbg !3407
  %_107.i.i = add i32 %base.i.i, %_108.i.i, !dbg !3408
  store i32 %_107.i.i, ptr %_51.i, align 4, !dbg !3410, !alias.scope !2593, !noalias !2663
  br label %bb4, !dbg !3411

bb7:                                              ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, %bb4
  %_27 = trunc nuw i64 %..i to i32, !dbg !3412
  %299 = load i32, ptr %0, align 4, !dbg !3413, !noundef !12
  %300 = sub i32 %299, %_27, !dbg !3413
  store i32 %300, ptr %0, align 4, !dbg !3413
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3414), !dbg !3417
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3418), !dbg !3417
  call void @llvm.lifetime.start.p0(ptr nonnull %iter.i), !dbg !3420, !noalias !3425
  %_7.sroa.0.sroa.3.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 16, !dbg !3420
  store ptr %left.0, ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i, align 8, !dbg !3420, !noalias !3425
  %_7.sroa.0.sroa.4.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 24, !dbg !3420
  store i64 %left.1, ptr %_7.sroa.0.sroa.4.0.iter.sroa_idx.i, align 8, !dbg !3420, !noalias !3425
  %_7.sroa.0.sroa.5.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 32, !dbg !3420
  store ptr %right.0, ptr %_7.sroa.0.sroa.5.0.iter.sroa_idx.i, align 8, !dbg !3420, !noalias !3425
  %_7.sroa.0.sroa.6.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 40, !dbg !3420
  store i64 %right.1, ptr %_7.sroa.0.sroa.6.0.iter.sroa_idx.i, align 8, !dbg !3420, !noalias !3425
  %_7184.not.i = icmp eq i64 %frames, 0
  %301 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %302 = getelementptr inbounds nuw i8, ptr %self, i64 104
  %303 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %304 = getelementptr inbounds nuw i8, ptr %self, i64 944
  %305 = getelementptr inbounds nuw i8, ptr %self, i64 72
  %sample_rate.i.i.i = load i32, ptr %305, align 8, !alias.scope !3414, !noalias !3428
  %_32.i.i.i = uitofp i32 %sample_rate.i.i.i to double
  %306 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %307 = getelementptr inbounds nuw i8, ptr %self, i64 744
  %308 = getelementptr inbounds nuw i8, ptr %self, i64 792
  %slots.i.i = load i64, ptr %301, align 8, !alias.scope !3414, !noalias !3428
  %_197.not.i.i = icmp eq i64 %slots.i.i, 0
  %_12.i.i.i = load i32, ptr %306, align 8, !alias.scope !3414, !noalias !3428
  br label %bb6.i5, !dbg !3429

bb6.i5:                                           ; preds = %bb1.backedge.i, %bb7
  %counter.sroa.0.0.v.i = phi i64 [ 24, %bb7 ], [ 32, %bb1.backedge.i ]
  %_5.not.i.i.i.i = phi i1 [ false, %bb7 ], [ true, %bb1.backedge.i ]
  %309 = phi i64 [ 0, %bb7 ], [ 1, %bb1.backedge.i ]
  %self3.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<&mut [f32]>", ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i, i64 %309, !dbg !3449
  %_14.0.i.i.i.i = load ptr, ptr %self3.i.i.i.i, align 8, !dbg !3462, !alias.scope !3475, !noalias !3482, !nonnull !12, !align !3484, !noundef !12
  %310 = getelementptr inbounds nuw i8, ptr %self3.i.i.i.i, i64 8, !dbg !3462
  %_14.1.i.i.i.i = load i64, ptr %310, align 8, !dbg !3462, !alias.scope !3475, !noalias !3482, !noundef !12
  %311 = getelementptr inbounds nuw %"kernel::GateState<f32>", ptr %self, i64 %309, !dbg !3485
  %312 = getelementptr inbounds nuw i8, ptr %311, i64 864, !dbg !3485
  %_17.i = load float, ptr %312, align 4, !dbg !3485, !alias.scope !3414, !noalias !3428, !noundef !12
  %_22.not.i2177.i = icmp eq i64 %_14.1.i.i.i.i, 0, !dbg !3487
  br i1 %_22.not.i2177.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !3487

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i: ; preds = %bb6.i5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %ok.sroa.0.0.i2080.i = phi i32 [ %_0.i40.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ -1, %bb6.i5 ]
  %iter.sroa.0.0.i1979.i = phi ptr [ %_27.i23.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %_14.0.i.i.i.i, %bb6.i5 ]
  %iter.sroa.5.0.i1878.i = phi i64 [ %_28.i24.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i ], [ %_14.1.i.i.i.i, %bb6.i5 ]
  %_27.i23.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1979.i, i64 4, !dbg !3502
  %_28.i24.i = add i64 %iter.sroa.5.0.i1878.i, -1, !dbg !3509
  %_0.i35.i = load float, ptr %iter.sroa.0.0.i1979.i, align 4, !dbg !3510, !alias.scope !3513, !noalias !3516, !noundef !12
  %313 = tail call noundef float @llvm.fabs.f32(float %_0.i35.i), !dbg !3517
  %_3.i.i = fcmp olt float %313, 0x46293E5940000000, !dbg !3520
  %_0.i40.i = select i1 %_3.i.i, i32 %ok.sroa.0.0.i2080.i, i32 0, !dbg !3522
  %_22.not.i21.i = icmp eq i64 %_28.i24.i, 0, !dbg !3487
  br i1 %_22.not.i21.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i, !dbg !3487

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i
  %314 = icmp eq i32 %_0.i40.i, -1, !dbg !3524
  %315 = tail call float @llvm.fabs.f32(float %_17.i)
  %_3.i33120.i = fcmp olt float %315, 0x46293E5940000000
  %or.cond.i = and i1 %_3.i33120.i, %314, !dbg !3527
  br i1 %or.cond.i, label %bb1.backedge.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !3527

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i: ; preds = %bb6.i5
  %316 = tail call noundef float @llvm.fabs.f32(float %_17.i), !dbg !3528
  %_3.i33.i = fcmp olt float %316, 0x46293E5940000000, !dbg !3531
  br i1 %_3.i33.i, label %bb1.backedge.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i, !dbg !3533

bb1.backedge.i:                                   ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i
  br i1 %_5.not.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E12finish_blockB2_.exit, label %bb6.i5, !dbg !3429

bb24.loopexit.i.i:                                ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %317 = and i32 %_0.i7.i.i, 1065353216, !dbg !3534
  %318 = icmp ne i32 %317, 1065353216, !dbg !3541
  %319 = zext i1 %318 to i32, !dbg !3541
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i, !dbg !3545

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %ok.sroa.0.017.i.i = phi i32 [ %_0.i7.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i ]
  %iter.sroa.0.016.i.i = phi ptr [ %_45.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %_14.0.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i ]
  %iter.sroa.5.015.i.i = phi i64 [ %_46.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %_14.1.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i ]
  %_45.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i.i, i64 4, !dbg !3546
  %_46.i.i = add nsw i64 %iter.sroa.5.015.i.i, -1, !dbg !3559
  %_0.i.i.i = load float, ptr %iter.sroa.0.016.i.i, align 4, !dbg !3560, !alias.scope !3563, !noalias !3516, !noundef !12
  %320 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i), !dbg !3568
  %_3.i.i.i = fcmp olt float %320, 0x46293E5940000000, !dbg !3571
  %_0.i7.i.i = select i1 %_3.i.i.i, i32 %ok.sroa.0.017.i.i, i32 0, !dbg !3573
  %_40.not.i.i = icmp eq i64 %_46.i.i, 0, !dbg !3575
  br i1 %_40.not.i.i, label %bb24.loopexit.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !3575

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i: ; preds = %bb24.loopexit.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i
  %.pre-phi = phi float [ %315, %bb24.loopexit.i.i ], [ %316, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i ], !dbg !3576
  %ok.sroa.0.0.lcssa.i.i = phi i32 [ %319, %bb24.loopexit.i.i ], [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i ], !dbg !3579
  %_3.i.i54.i = fcmp uge float %.pre-phi, 0x46293E5940000000, !dbg !3580
  %321 = zext i1 %_3.i.i54.i to i32, !dbg !3582
  %failed.i = or i32 %ok.sroa.0.0.lcssa.i.i, %321, !dbg !3583
  %322 = icmp eq i32 %failed.i, 0
  %ring.i.i = getelementptr inbounds nuw %Ring, ptr %302, i64 %309
  %323 = getelementptr inbounds nuw i8, ptr %ring.i.i, i64 8
  %324 = getelementptr inbounds nuw i8, ptr %ring.i.i, i64 24
  %325 = getelementptr inbounds nuw i8, ptr %ring.i.i, i64 16
  %326 = getelementptr inbounds nuw [8 x float], ptr %303, i64 %309
  %values.sroa.5.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %326, i64 4
  %values.sroa.6.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %326, i64 8
  %values.sroa.7.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %326, i64 12
  %values.sroa.8.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %326, i64 16
  %values.sroa.9.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %326, i64 20
  %values.sroa.10.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %326, i64 24
  %values.sroa.11.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %326, i64 28
  %327 = getelementptr inbounds nuw %LaneTiming, ptr %304, i64 %309
  %_7.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %327, i64 4
  %_7.sroa.5.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %327, i64 8
  %_7.sroa.6.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %327, i64 12
  %328 = getelementptr inbounds nuw %Ring, ptr %self, i64 %309
  %329 = getelementptr inbounds nuw i8, ptr %328, i64 136
  %330 = getelementptr inbounds nuw %"kernel::GateCoef<f32>", ptr %307, i64 %309
  %_19.i.i.i = getelementptr inbounds nuw i8, ptr %330, i64 8
  %_25.i.i.i = getelementptr inbounds nuw i8, ptr %330, i64 4
  %331 = getelementptr inbounds nuw %"kernel::GateState<f32>", ptr %308, i64 %309
  %_17.i.i = getelementptr inbounds nuw i8, ptr %331, i64 72
  %_19.i.i = getelementptr inbounds nuw i8, ptr %331, i64 64
  %_21.i.i = getelementptr inbounds nuw i8, ptr %331, i64 68
  %_37.i.i = getelementptr inbounds nuw i8, ptr %331, i64 4
  %_39.i.i = getelementptr inbounds nuw i8, ptr %331, i64 8
  %_41.i.i = getelementptr inbounds nuw i8, ptr %331, i64 12
  %iter.sroa.0.0.ptr24.1.i.i = getelementptr inbounds nuw i8, ptr %331, i64 16
  %_37.1.i.i = getelementptr inbounds nuw i8, ptr %331, i64 20
  %_39.1.i.i = getelementptr inbounds nuw i8, ptr %331, i64 24
  %_41.1.i.i = getelementptr inbounds nuw i8, ptr %331, i64 28
  %iter.sroa.0.0.ptr24.2.i.i = getelementptr inbounds nuw i8, ptr %331, i64 32
  %_37.2.i.i = getelementptr inbounds nuw i8, ptr %331, i64 36
  %_39.2.i.i = getelementptr inbounds nuw i8, ptr %331, i64 40
  %_41.2.i.i = getelementptr inbounds nuw i8, ptr %331, i64 44
  %iter.sroa.0.0.ptr24.3.i.i = getelementptr inbounds nuw i8, ptr %331, i64 48
  %_37.3.i.i = getelementptr inbounds nuw i8, ptr %331, i64 52
  %_39.3.i.i = getelementptr inbounds nuw i8, ptr %331, i64 56
  %_41.3.i.i = getelementptr inbounds nuw i8, ptr %331, i64 60
  %counter.sroa.0.0.i = getelementptr inbounds nuw i8, ptr %reports, i64 %counter.sroa.0.0.v.i
  br i1 %322, label %bb1.backedge.i, label %bb30.i

bb30.i:                                           ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i
  br i1 %_7184.not.i, label %bb33.i, label %bb32.i, !dbg !3584

bb33.i:                                           ; preds = %bb21.i, %bb30.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3595), !dbg !3598
  br i1 %_197.not.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i, label %bb7.lr.ph.i.i, !dbg !3601

bb7.lr.ph.i.i:                                    ; preds = %bb33.i
  %_23.1.i.i = load i64, ptr %323, align 8, !alias.scope !3613, !noalias !3428, !noundef !12
  br label %bb7.i.i, !dbg !3601

bb7.i.i:                                          ; preds = %bb5.i.i, %bb7.lr.ph.i.i
  %iter.sroa.0.08.i.i = phi i64 [ 0, %bb7.lr.ph.i.i ], [ %332, %bb5.i.i ]
  %332 = add nuw i64 %iter.sroa.0.08.i.i, 1, !dbg !3614
  %exitcond.not.i.i = icmp eq i64 %iter.sroa.0.08.i.i, %_23.1.i.i, !dbg !3620
  br i1 %exitcond.not.i.i, label %panic1.i.i, label %bb3.i.i, !dbg !3620

bb3.i.i:                                          ; preds = %bb7.i.i
  %_23.0.i.i = load ptr, ptr %ring.i.i, align 8, !dbg !3620, !alias.scope !3613, !noalias !3428, !nonnull !12, !noundef !12
  %333 = getelementptr inbounds nuw float, ptr %_23.0.i.i, i64 %iter.sroa.0.08.i.i, !dbg !3620
  store float 0.000000e+00, ptr %333, align 4, !dbg !3620, !noalias !3622
  %_25.1.i.i = load i64, ptr %324, align 8, !dbg !3623, !alias.scope !3613, !noalias !3428, !noundef !12
  %_14.i.i = icmp ult i64 %iter.sroa.0.08.i.i, %_25.1.i.i, !dbg !3623
  br i1 %_14.i.i, label %bb5.i.i, label %panic2.i.i, !dbg !3623

panic1.i.i:                                       ; preds = %bb7.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.1.i.i, i64 noundef %_23.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f8f0512af3f0ba047152c3c522a44b59) #24, !dbg !3620, !noalias !3622
  unreachable, !dbg !3620

bb5.i.i:                                          ; preds = %bb3.i.i
  %_25.0.i.i = load ptr, ptr %325, align 8, !dbg !3623, !alias.scope !3613, !noalias !3428, !nonnull !12, !noundef !12
  %334 = getelementptr inbounds nuw float, ptr %_25.0.i.i, i64 %iter.sroa.0.08.i.i, !dbg !3623
  store float 0.000000e+00, ptr %334, align 4, !dbg !3623, !noalias !3622
  %exitcond12.not.i.i = icmp eq i64 %332, %slots.i.i, !dbg !3624
  br i1 %exitcond12.not.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i, label %bb7.i.i, !dbg !3601

panic2.i.i:                                       ; preds = %bb3.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.sroa.0.08.i.i, i64 noundef %_25.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_25eed8128df6f66da1727b7a67d9a9d8) #24, !dbg !3623, !noalias !3622
  unreachable, !dbg !3623

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i: ; preds = %bb5.i.i, %bb33.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3627), !dbg !3630
  %values.sroa.0.0.copyload.i.i = load float, ptr %326, align 8, !dbg !3631, !alias.scope !3634, !noalias !3428
  %values.sroa.5.0.copyload.i.i = load float, ptr %values.sroa.5.0..sroa_idx.i.i, align 4, !dbg !3631, !alias.scope !3634, !noalias !3428
  %values.sroa.6.0.copyload.i.i = load float, ptr %values.sroa.6.0..sroa_idx.i.i, align 8, !dbg !3631, !alias.scope !3634, !noalias !3428
  %values.sroa.7.0.copyload.i.i = load float, ptr %values.sroa.7.0..sroa_idx.i.i, align 4, !dbg !3631, !alias.scope !3634, !noalias !3428
  %values.sroa.8.0.copyload.i.i = load float, ptr %values.sroa.8.0..sroa_idx.i.i, align 8, !dbg !3631, !alias.scope !3634, !noalias !3428
  %values.sroa.9.0.copyload.i.i = load float, ptr %values.sroa.9.0..sroa_idx.i.i, align 4, !dbg !3631, !alias.scope !3634, !noalias !3428
  %values.sroa.10.0.copyload.i.i = load float, ptr %values.sroa.10.0..sroa_idx.i.i, align 8, !dbg !3631, !alias.scope !3634, !noalias !3428
  %values.sroa.11.0.copyload.i.i = load float, ptr %values.sroa.11.0..sroa_idx.i.i, align 4, !dbg !3631, !alias.scope !3634, !noalias !3428
  store float %values.sroa.11.0.copyload.i.i, ptr %327, align 8, !dbg !3635, !alias.scope !3634, !noalias !3428
  store float %values.sroa.8.0.copyload.i.i, ptr %_7.sroa.4.0..sroa_idx.i.i, align 4, !dbg !3635, !alias.scope !3634, !noalias !3428
  store float %values.sroa.9.0.copyload.i.i, ptr %_7.sroa.5.0..sroa_idx.i.i, align 8, !dbg !3635, !alias.scope !3634, !noalias !3428
  store float %values.sroa.10.0.copyload.i.i, ptr %_7.sroa.6.0..sroa_idx.i.i, align 4, !dbg !3635, !alias.scope !3634, !noalias !3428
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3637), !dbg !3640
  %_31.i.i.i = fpext float %values.sroa.11.0.copyload.i.i to double, !dbg !3641
  %_30.i.i.i = fmul double %_32.i.i.i, %_31.i.i.i, !dbg !3645
  %_29.i.i.i = fdiv double %_30.i.i.i, 1.000000e+03, !dbg !3645
  %_28.i.i.i = fadd double %_29.i.i.i, 5.000000e-01, !dbg !3646
  %335 = tail call double @llvm.floor.f64(double %_28.i.i.i), !dbg !3647
  %or.cond.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %335, i32 527), !dbg !3650
  %_35.i.i.i = fcmp ogt double %335, 0x41EFFFFFFFE00000
  %or.cond10.i.i.i = or i1 %or.cond.i.i.i, %_35.i.i.i, !dbg !3650
  br i1 %or.cond10.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i, label %bb19.i.i.i, !dbg !3650

bb19.i.i.i:                                       ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i
  %_45.i.i.i = fpext float %values.sroa.9.0.copyload.i.i to double, !dbg !3651
  %_44.i.i.i = fmul double %_32.i.i.i, %_45.i.i.i, !dbg !3654
  %_43.i.i.i = fdiv double %_44.i.i.i, 1.000000e+03, !dbg !3654
  %_42.i.i.i = fadd double %_43.i.i.i, 5.000000e-01, !dbg !3655
  %336 = tail call double @llvm.floor.f64(double %_42.i.i.i), !dbg !3656
  %or.cond11.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %336, i32 527), !dbg !3659
  %_48.i.i.i = fcmp ogt double %336, 0x41EFFFFFFFE00000
  %or.cond12.i.i.i = or i1 %or.cond11.i.i.i, %_48.i.i.i, !dbg !3659
  br i1 %or.cond12.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i, label %bb20.3.i.i, !dbg !3659

bb20.3.i.i:                                       ; preds = %bb19.i.i.i
  %_36.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %335), !dbg !3660
  %_49.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %336), !dbg !3661
  %337 = tail call i32 @llvm.usub.sat.i32(i32 %_12.i.i.i, i32 %_36.i.i.i), !dbg !3662
  store i32 %337, ptr %329, align 4, !dbg !3662, !alias.scope !3663, !noalias !3428
  %_20.i.i.i = uitofp i32 %_49.i.i.i to float, !dbg !3664
  store float %_20.i.i.i, ptr %_19.i.i.i, align 4, !dbg !3665, !alias.scope !3667, !noalias !3428
; call effect_runtime::envelope::attack_release_coefficient
  %_23.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.8.0.copyload.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !3670, !noalias !3671
  store float %_23.i.i.i, ptr %330, align 4, !dbg !3672, !alias.scope !3674, !noalias !3428
; call effect_runtime::envelope::attack_release_coefficient
  %_26.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.10.0.copyload.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !3677, !noalias !3671
  store float %_26.i.i.i, ptr %_25.i.i.i, align 4, !dbg !3678, !alias.scope !3680, !noalias !3428
  store float 0.000000e+00, ptr %_17.i.i, align 4, !dbg !3683, !alias.scope !3686, !noalias !3428
  store float 1.000000e+00, ptr %_19.i.i, align 4, !dbg !3689, !alias.scope !3691, !noalias !3428
  store float %_20.i.i.i, ptr %_21.i.i, align 4, !dbg !3694, !alias.scope !3696, !noalias !3428
  store float %values.sroa.0.0.copyload.i.i, ptr %331, align 4, !dbg !3699, !alias.scope !3703, !noalias !3428
  store float %values.sroa.0.0.copyload.i.i, ptr %_37.i.i, align 4, !dbg !3706, !alias.scope !3708, !noalias !3428
  store float 0.000000e+00, ptr %_39.i.i, align 4, !dbg !3711, !alias.scope !3713, !noalias !3428
  store float 0.000000e+00, ptr %_41.i.i, align 4, !dbg !3716, !alias.scope !3718, !noalias !3428
  store float %values.sroa.5.0.copyload.i.i, ptr %iter.sroa.0.0.ptr24.1.i.i, align 4, !dbg !3699, !alias.scope !3703, !noalias !3428
  store float %values.sroa.5.0.copyload.i.i, ptr %_37.1.i.i, align 4, !dbg !3706, !alias.scope !3708, !noalias !3428
  store float 0.000000e+00, ptr %_39.1.i.i, align 4, !dbg !3711, !alias.scope !3713, !noalias !3428
  store float 0.000000e+00, ptr %_41.1.i.i, align 4, !dbg !3716, !alias.scope !3718, !noalias !3428
  store float %values.sroa.6.0.copyload.i.i, ptr %iter.sroa.0.0.ptr24.2.i.i, align 4, !dbg !3699, !alias.scope !3703, !noalias !3428
  store float %values.sroa.6.0.copyload.i.i, ptr %_37.2.i.i, align 4, !dbg !3706, !alias.scope !3708, !noalias !3428
  store float 0.000000e+00, ptr %_39.2.i.i, align 4, !dbg !3711, !alias.scope !3713, !noalias !3428
  store float 0.000000e+00, ptr %_41.2.i.i, align 4, !dbg !3716, !alias.scope !3718, !noalias !3428
  store float %values.sroa.7.0.copyload.i.i, ptr %iter.sroa.0.0.ptr24.3.i.i, align 4, !dbg !3699, !alias.scope !3703, !noalias !3428
  store float %values.sroa.7.0.copyload.i.i, ptr %_37.3.i.i, align 4, !dbg !3706, !alias.scope !3708, !noalias !3428
  store float 0.000000e+00, ptr %_39.3.i.i, align 4, !dbg !3711, !alias.scope !3713, !noalias !3428
  store float 0.000000e+00, ptr %_41.3.i.i, align 4, !dbg !3716, !alias.scope !3718, !noalias !3428
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i, !dbg !3721

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9seed_laneB2_.exit.i: ; preds = %bb20.3.i.i, %bb19.i.i.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16clear_lane_ringsB2_.exit.i
  %_46.i = load i64, ptr %counter.sroa.0.0.i, align 8, !dbg !3722, !alias.scope !3418, !noalias !3725, !noundef !12
  %338 = tail call i64 @llvm.uadd.sat.i64(i64 %_46.i, i64 %frames), !dbg !3726
  store i64 %338, ptr %counter.sroa.0.0.i, align 8, !dbg !3730, !alias.scope !3418, !noalias !3725
  br label %bb1.backedge.i, !dbg !3429

bb32.i:                                           ; preds = %bb30.i, %bb21.i
  %iter2.sroa.0.085.i = phi i64 [ %339, %bb21.i ], [ 0, %bb30.i ]
  %exitcond.not.i6 = icmp eq i64 %iter2.sroa.0.085.i, %_14.1.i.i.i.i, !dbg !3731
  br i1 %exitcond.not.i6, label %panic4.i, label %bb21.i, !dbg !3731

bb21.i:                                           ; preds = %bb32.i
  %339 = add nuw i64 %iter2.sroa.0.085.i, 1, !dbg !3733
  %340 = getelementptr inbounds nuw float, ptr %_14.0.i.i.i.i, i64 %iter2.sroa.0.085.i, !dbg !3731
  store float 0.000000e+00, ptr %340, align 4, !dbg !3731, !noalias !3516
  %exitcond115.not.i = icmp eq i64 %339, %frames, !dbg !3741
  br i1 %exitcond115.not.i, label %bb33.i, label %bb32.i, !dbg !3584

panic4.i:                                         ; preds = %bb32.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_14.1.i.i.i.i, i64 noundef %_14.1.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30d570589940b68d7f77af9df77eb2ee) #24, !dbg !3731, !noalias !3516
  unreachable, !dbg !3731

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E12finish_blockB2_.exit: ; preds = %bb1.backedge.i
  call void @llvm.lifetime.end.p0(ptr nonnull %iter.i), !dbg !3745, !noalias !3425
  ret void, !dbg !3746

bb5:                                              ; preds = %bb4
  %341 = load ptr, ptr %sidechain, align 8, !dbg !3747, !noundef !12
  %.not3 = icmp eq ptr %341, null, !dbg !3747
  br i1 %.not3, label %bb20, label %bb22, !dbg !3751

bb22:                                             ; preds = %bb5
  %_51.sroa.4.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 8, !dbg !3752
  %_51.sroa.4.0.copyload = load i64, ptr %_51.sroa.4.0.sidechain.sroa_idx, align 8, !dbg !3752
  %_51.sroa.5.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 16, !dbg !3752
  %_51.sroa.5.0.copyload = load ptr, ptr %_51.sroa.5.0.sidechain.sroa_idx, align 8, !dbg !3752, !nonnull !12, !noundef !12
  %_51.sroa.6.0.sidechain.sroa_idx = getelementptr inbounds nuw i8, ptr %sidechain, i64 24, !dbg !3752
  %_51.sroa.6.0.copyload = load i64, ptr %_51.sroa.6.0.sidechain.sroa_idx, align 8, !dbg !3752
  %_9.i = icmp ugt i64 %..i, %_51.sroa.4.0.copyload, !dbg !3754
  br i1 %_9.i, label %bb1.i13, label %bb2.i, !dbg !3754, !prof !180

bb2.i:                                            ; preds = %bb22
  %_17.i10 = icmp ugt i64 %..i, %_51.sroa.6.0.copyload, !dbg !3763
  br i1 %_17.i10, label %bb3.i12, label %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit, !dbg !3763, !prof !180

bb1.i13:                                          ; preds = %bb22
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i, i64 noundef %_51.sroa.4.0.copyload, i64 noundef %_51.sroa.4.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30a58d326fa295091a02a55f77ea093b) #24, !dbg !3767, !noalias !3768
  unreachable, !dbg !3767

bb3.i12:                                          ; preds = %bb2.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i, i64 noundef %_51.sroa.6.0.copyload, i64 noundef %_51.sroa.6.0.copyload, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9ed981539d8cc9a26fc3fd872195988f) #24, !dbg !3772, !noalias !3768
  unreachable, !dbg !3772

_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit: ; preds = %bb2.i
  %_16.i = getelementptr inbounds nuw float, ptr %341, i64 %..i, !dbg !3773
  %_12.i = sub nuw i64 %_51.sroa.4.0.copyload, %..i, !dbg !3778
  %_20.i = sub nuw i64 %_51.sroa.6.0.copyload, %..i, !dbg !3779
  %_24.i = getelementptr inbounds nuw float, ptr %_51.sroa.5.0.copyload, i64 %..i, !dbg !3780
  br label %bb20, !dbg !3785

bb20:                                             ; preds = %bb5, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit
  %side2.sroa.3.0 = phi i64 [ %_12.i, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit ], [ undef, %bb5 ]
  %side2.sroa.0.0 = phi ptr [ %_16.i, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit ], [ null, %bb5 ], !dbg !3786
  %side2.sroa.4.0 = phi ptr [ %_24.i, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit ], [ undef, %bb5 ]
  %side2.sroa.5.0 = phi i64 [ %_20.i, %_RNCNvMCsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_E9run_blocks_0B4_.exit ], [ undef, %bb5 ]
  %_52 = icmp samesign ugt i64 %..i, %left.1, !dbg !3787
  br i1 %_52, label %bb24, label %bb25, !dbg !3787, !prof !180

bb25:                                             ; preds = %bb20
  %_60 = icmp samesign ugt i64 %..i, %right.1, !dbg !3794
  br i1 %_60, label %bb26, label %bb27, !dbg !3794, !prof !180

bb24:                                             ; preds = %bb20
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i, i64 noundef %left.1, i64 noundef %left.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e6adad6d678d00eb6b02b8162f35ebb4) #24, !dbg !3798
  unreachable, !dbg !3798

bb27:                                             ; preds = %bb25
  %_59 = getelementptr inbounds nuw float, ptr %left.0, i64 %..i, !dbg !3799
  %_55 = sub nuw nsw i64 %left.1, %..i, !dbg !3804
  %_63 = sub nuw nsw i64 %right.1, %..i, !dbg !3805
  %_67 = getelementptr inbounds nuw float, ptr %right.0, i64 %..i, !dbg !3806
  %_26 = sub i64 %frames, %..i, !dbg !3811
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3812), !dbg !3815
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3816), !dbg !3815
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3818), !dbg !3815
  %_10.i15 = getelementptr inbounds nuw i8, ptr %self, i64 792, !dbg !3820
  %data.i.i.i16 = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !3823
  %_15.i17 = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !3828
  %data.i.i557.i = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !3830
  %342 = getelementptr inbounds nuw i8, ptr %self, i64 744, !dbg !3835
  %_32.i18 = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !3839
  %_58.0.i19 = load ptr, ptr %_15.i17, align 8, !dbg !3840, !alias.scope !3812, !noalias !3841, !nonnull !12, !noundef !12
  %343 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !3840
  %_58.1.i20 = load i64, ptr %343, align 8, !dbg !3840, !alias.scope !3812, !noalias !3841, !noundef !12
  %344 = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !3843
  %_59.0.i21 = load ptr, ptr %344, align 8, !dbg !3843, !alias.scope !3812, !noalias !3841, !nonnull !12, !noundef !12
  %345 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !3843
  %_59.1.i22 = load i64, ptr %345, align 8, !dbg !3843, !alias.scope !3812, !noalias !3841, !noundef !12
  %_45.i23 = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !3844
  %_60.0.i24 = load ptr, ptr %data.i.i557.i, align 8, !dbg !3845, !alias.scope !3812, !noalias !3841, !nonnull !12, !noundef !12
  %346 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !3845
  %_60.1.i25 = load i64, ptr %346, align 8, !dbg !3845, !alias.scope !3812, !noalias !3841, !noundef !12
  %347 = getelementptr inbounds nuw i8, ptr %self, i64 184, !dbg !3846
  %_61.0.i26 = load ptr, ptr %347, align 8, !dbg !3846, !alias.scope !3812, !noalias !3841, !nonnull !12, !noundef !12
  %348 = getelementptr inbounds nuw i8, ptr %self, i64 192, !dbg !3846
  %_61.1.i27 = load i64, ptr %348, align 8, !dbg !3846, !alias.scope !3812, !noalias !3841, !noundef !12
  %_50.i28 = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !3847
  %_51.i29 = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !3848
  %349 = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !3849
  %_52.i30 = load i32, ptr %349, align 4, !dbg !3849, !alias.scope !3812, !noalias !3841, !noundef !12
  %350 = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !3850
  %_53.i31 = load i32, ptr %350, align 8, !dbg !3850, !alias.scope !3812, !noalias !3841, !noundef !12
  %.not.i.i33 = icmp eq ptr %side2.sroa.0.0, null, !dbg !3851
  br i1 %.not.i.i33, label %bb37.i.i41, label %bb39.i.i34, !dbg !3862

bb39.i.i34:                                       ; preds = %bb27
  %351 = icmp ne ptr %side2.sroa.4.0, null
  tail call void @llvm.assume(i1 %351)
  %352 = freeze i64 %side2.sroa.3.0
  %353 = freeze i64 %side2.sroa.5.0
  br label %bb37.i.i41, !dbg !3863

bb37.i.i41:                                       ; preds = %bb39.i.i34, %bb27
  %empty.sroa.6.0.i.i42 = phi i64 [ %353, %bb39.i.i34 ], [ 0, %bb27 ], !dbg !3864
  %empty.sroa.0.0.i.i43 = phi ptr [ %side2.sroa.4.0, %bb39.i.i34 ], [ inttoptr (i64 4 to ptr), %bb27 ], !dbg !3864
  %side_left.sroa.0.0.i.i44 = phi ptr [ %side2.sroa.0.0, %bb39.i.i34 ], [ inttoptr (i64 4 to ptr), %bb27 ], !dbg !3865
  %side_left.sroa.5.0.i.i45 = phi i64 [ %352, %bb39.i.i34 ], [ 0, %bb27 ], !dbg !3865
  %base.i.i46 = load i32, ptr %_51.i29, align 4, !dbg !3866, !alias.scope !3812, !noalias !3868, !noundef !12
  %_67.i.i = load i32, ptr %_45.i23, align 4, !alias.scope !3812, !noalias !3841
  %_75.i.i = load i32, ptr %_50.i28, align 4, !alias.scope !3812, !noalias !3841
  %threshold.i32.i = load float, ptr %_10.i15, align 4, !alias.scope !3812, !noalias !3841
  %354 = getelementptr inbounds nuw i8, ptr %self, i64 808
  %ratio.i33.i = load float, ptr %354, align 4, !alias.scope !3812, !noalias !3841
  %355 = getelementptr inbounds nuw i8, ptr %self, i64 824
  %range.i34.i = load float, ptr %355, align 4, !alias.scope !3812, !noalias !3841
  %356 = getelementptr inbounds nuw i8, ptr %self, i64 840
  %hysteresis.i35.i = load float, ptr %356, align 4, !alias.scope !3812, !noalias !3841
  %357 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %_37.i38.i = load float, ptr %357, align 4, !alias.scope !3812, !noalias !3841
  %_3.i190.i = fcmp ule float %_37.i38.i, 0.000000e+00
  %358 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %_41.i42.i = load float, ptr %358, align 4, !alias.scope !3812, !noalias !3841
  %_3.i188.i = fcmp ule float %_41.i42.i, 0.000000e+00
  %359 = getelementptr inbounds nuw i8, ptr %self, i64 856
  %_0.i254.i = fsub float %threshold.i32.i, %hysteresis.i35.i
  %360 = getelementptr inbounds nuw i8, ptr %self, i64 860
  %361 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %_71.i75568569.i = load float, ptr %361, align 4, !alias.scope !3812, !noalias !3841
  %_0.i252.i = fadd float %ratio.i33.i, -1.000000e+00
  %362 = fneg float %range.i34.i
  %363 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %_87.i90571.i = load i32, ptr %342, align 4, !alias.scope !3812, !noalias !3841
  %364 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %_88.i91572.i = load i32, ptr %364, align 4, !alias.scope !3812, !noalias !3841
  %365 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %_98.i104.i = load float, ptr %365, align 4, !alias.scope !3812, !noalias !3841
  %_3.i178.i = fcmp ule float %_98.i104.i, 0.000000e+00
  %threshold.i.i = load float, ptr %data.i.i.i16, align 4, !alias.scope !3812, !noalias !3841
  %366 = getelementptr inbounds nuw i8, ptr %self, i64 884
  %ratio.i.i = load float, ptr %366, align 4, !alias.scope !3812, !noalias !3841
  %367 = getelementptr inbounds nuw i8, ptr %self, i64 900
  %range.i.i = load float, ptr %367, align 4, !alias.scope !3812, !noalias !3841
  %368 = getelementptr inbounds nuw i8, ptr %self, i64 916
  %hysteresis.i.i = load float, ptr %368, align 4, !alias.scope !3812, !noalias !3841
  %369 = getelementptr inbounds nuw i8, ptr %self, i64 784
  %_37.i.i47 = load float, ptr %369, align 4, !alias.scope !3812, !noalias !3841
  %_3.i204.i = fcmp ule float %_37.i.i47, 0.000000e+00
  %370 = getelementptr inbounds nuw i8, ptr %self, i64 788
  %_41.i.i48 = load float, ptr %370, align 4, !alias.scope !3812, !noalias !3841
  %_3.i202.i = fcmp ule float %_41.i.i48, 0.000000e+00
  %371 = getelementptr inbounds nuw i8, ptr %self, i64 932
  %_0.i259.i = fsub float %threshold.i.i, %hysteresis.i.i
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 936
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 776
  %_71.i585586.i = load float, ptr %373, align 4, !alias.scope !3812, !noalias !3841
  %_0.i257.i = fadd float %ratio.i.i, -1.000000e+00
  %374 = fneg float %range.i.i
  %375 = getelementptr inbounds nuw i8, ptr %self, i64 940
  %_87.i23588.i = load i32, ptr %_32.i18, align 4, !alias.scope !3812, !noalias !3841
  %376 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %_88.i24589.i = load i32, ptr %376, align 4, !alias.scope !3812, !noalias !3841
  %377 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %_98.i.i = load float, ptr %377, align 4, !alias.scope !3812, !noalias !3841
  %_3.i192.i = fcmp ule float %_98.i.i, 0.000000e+00
  %.promoted1000.i = load float, ptr %360, align 4, !alias.scope !3812, !noalias !3841
  %.promoted1002.i = load float, ptr %363, align 4, !alias.scope !3812, !noalias !3841
  %.promoted1004.i = load float, ptr %372, align 4, !alias.scope !3812, !noalias !3841
  %.promoted1006.i = load float, ptr %375, align 4, !alias.scope !3812, !noalias !3841
  %injected.cond.not.i = icmp samesign ugt i64 %left.1, %right.1
  br i1 %injected.cond.not.i, label %bb42.i.i, label %bb40.i.lr.ph.split.us.i

bb40.i.lr.ph.split.us.i:                          ; preds = %bb37.i.i41
  %injected.cond1130.not.i = icmp ugt i64 %_58.1.i20, %_60.1.i25
  br i1 %injected.cond1130.not.i, label %bb40.i.us.i433, label %bb40.i.lr.ph.split.us.split.us.i

bb40.i.lr.ph.split.us.split.us.i:                 ; preds = %bb40.i.lr.ph.split.us.i
  %injected.cond1231.not.i = icmp ugt i64 %_55, %side_left.sroa.5.0.i.i45
  br i1 %injected.cond1231.not.i, label %bb42.i.us.us.i356, label %bb40.i.lr.ph.split.us.split.us.split.us.i

bb40.i.lr.ph.split.us.split.us.split.us.i:        ; preds = %bb40.i.lr.ph.split.us.split.us.i
  %injected.cond1326.not.i = icmp ugt i64 %_58.1.i20, %_59.1.i22
  br i1 %injected.cond1326.not.i, label %bb40.i.us.us.us.i279, label %bb40.i.lr.ph.split.us.split.us.split.us.split.us.i

bb40.i.lr.ph.split.us.split.us.split.us.split.us.i: ; preds = %bb40.i.lr.ph.split.us.split.us.split.us.i
  %injected.cond1415.not.i = icmp ugt i64 %_55, %empty.sroa.6.0.i.i42
  br i1 %injected.cond1415.not.i, label %bb42.i.us.us.us.us.i205, label %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i

bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i: ; preds = %bb40.i.lr.ph.split.us.split.us.split.us.split.us.i
  %injected.cond1498.not.i = icmp ugt i64 %_58.1.i20, %_61.1.i27
  br i1 %injected.cond1498.not.i, label %bb40.i.us.us.us.us.us.i128, label %bb40.i.us.us.us.us.us.us.us.i49

bb40.i.us.us.us.us.us.us.us.i49:                  ; preds = %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i, %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99
  %_86.i211007.us.us.us.us.us.us.us.i = phi float [ %_0.i296.us.us.us.us.us.us.us.i122, %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99 ], [ %.promoted1006.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %_67.i131005.us.us.us.us.us.us.us.i = phi float [ %_0.i373.us.us.us.us.us.us.us.i118, %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99 ], [ %.promoted1004.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %_86.i881003.us.us.us.us.us.us.us.i = phi float [ %_0.i292.us.us.us.us.us.us.us.i107, %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99 ], [ %.promoted1002.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %_67.i691001.us.us.us.us.us.us.us.i = phi float [ %_0.i321.us.us.us.us.us.us.us.i, %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99 ], [ %.promoted1000.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %iter.sroa.0.0.i966.us.us.us.us.us.us.us.i = phi i64 [ %378, %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99 ], [ 0, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %378 = add nuw nsw i64 %iter.sroa.0.0.i966.us.us.us.us.us.us.us.i, 1, !dbg !3871
  %_27.i.us.us.us.us.us.us.us.i50 = trunc i64 %iter.sroa.0.0.i966.us.us.us.us.us.us.us.i to i32, !dbg !3885
  %now.i.us.us.us.us.us.us.us.i51 = add i32 %base.i.i46, %_27.i.us.us.us.us.us.us.us.i50, !dbg !3888
  %_30.i.us.us.us.us.us.us.us.i52 = and i32 %now.i.us.us.us.us.us.us.us.i51, %_52.i30, !dbg !3891
  %_29.i.us.us.us.us.us.us.us.i53 = zext i32 %_30.i.us.us.us.us.us.us.us.i52 to i64, !dbg !3893
  %exitcond.not.i54 = icmp eq i64 %iter.sroa.0.0.i966.us.us.us.us.us.us.us.i, %_55, !dbg !3894
  br i1 %exitcond.not.i54, label %bb43.i.i127, label %bb42.i.us.us.us.us.us.us.us.i55, !dbg !3894, !prof !180

bb42.i.us.us.us.us.us.us.us.i55:                  ; preds = %bb40.i.us.us.us.us.us.us.us.i49
  %_123.i.us.us.us.us.us.us.us.i56 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i966.us.us.us.us.us.us.us.i, !dbg !3900
  %_124.not.not.i.us.us.us.us.us.us.us.i57 = icmp ugt i64 %_58.1.i20, %_29.i.us.us.us.us.us.us.us.i53, !dbg !3904
  br i1 %_124.not.not.i.us.us.us.us.us.us.us.i57, label %bb45.i.us.us.us.us.us.us.us.i59, label %bb46.i.i58, !dbg !3904, !prof !2704

bb45.i.us.us.us.us.us.us.us.i59:                  ; preds = %bb42.i.us.us.us.us.us.us.us.i55
  %_0.i279.us.us.us.us.us.us.us.i60 = load float, ptr %_123.i.us.us.us.us.us.us.us.i56, align 4, !dbg !3909, !alias.scope !3911, !noalias !3914, !noundef !12
  %_133.i.us.us.us.us.us.us.us.i61 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_29.i.us.us.us.us.us.us.us.i53, !dbg !3915
  store float %_0.i279.us.us.us.us.us.us.us.i60, ptr %_133.i.us.us.us.us.us.us.us.i61, align 4, !dbg !3919, !alias.scope !3921, !noalias !3868
  %_141.i.us.us.us.us.us.us.us.i62 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i966.us.us.us.us.us.us.us.i, !dbg !3924
  %_0.i277.us.us.us.us.us.us.us.i63 = load float, ptr %_141.i.us.us.us.us.us.us.us.i62, align 4, !dbg !3931, !alias.scope !3933, !noalias !3936, !noundef !12
  %_149.i.us.us.us.us.us.us.us.i64 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_29.i.us.us.us.us.us.us.us.i53, !dbg !3937
  store float %_0.i277.us.us.us.us.us.us.us.i63, ptr %_149.i.us.us.us.us.us.us.us.i64, align 4, !dbg !3944, !alias.scope !3946, !noalias !3868
  %_157.i.us.us.us.us.us.us.us.i65 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i44, i64 %iter.sroa.0.0.i966.us.us.us.us.us.us.us.i, !dbg !3949
  %_0.i275.us.us.us.us.us.us.us.i66 = load float, ptr %_157.i.us.us.us.us.us.us.us.i65, align 4, !dbg !3956, !alias.scope !3958, !noalias !3868, !noundef !12
  %_165.i.us.us.us.us.us.us.us.i67 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_29.i.us.us.us.us.us.us.us.i53, !dbg !3961
  store float %_0.i275.us.us.us.us.us.us.us.i66, ptr %_165.i.us.us.us.us.us.us.us.i67, align 4, !dbg !3968, !alias.scope !3970, !noalias !3868
  %_173.i.us.us.us.us.us.us.us.i68 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i43, i64 %iter.sroa.0.0.i966.us.us.us.us.us.us.us.i, !dbg !3973
  %_0.i273.us.us.us.us.us.us.us.i69 = load float, ptr %_173.i.us.us.us.us.us.us.us.i68, align 4, !dbg !3980, !alias.scope !3982, !noalias !3868, !noundef !12
  %_181.i.us.us.us.us.us.us.us.i70 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_29.i.us.us.us.us.us.us.us.i53, !dbg !3985
  store float %_0.i273.us.us.us.us.us.us.us.i69, ptr %_181.i.us.us.us.us.us.us.us.i70, align 4, !dbg !3992, !alias.scope !3994, !noalias !3868
  %_52.i.us.us.us.us.us.us.us.i71 = sub i32 %now.i.us.us.us.us.us.us.us.i51, %_53.i31, !dbg !3997
  %_51.i.us.us.us.us.us.us.us.i72 = and i32 %_52.i.us.us.us.us.us.us.us.i71, %_52.i30, !dbg !4000
  %_50.i.us.us.us.us.us.us.us.i73 = zext i32 %_51.i.us.us.us.us.us.us.us.i72 to i64, !dbg !4001
  %_182.not.not.i.us.us.us.us.us.us.us.i74 = icmp ugt i64 %_58.1.i20, %_50.i.us.us.us.us.us.us.us.i73, !dbg !4002
  br i1 %_182.not.not.i.us.us.us.us.us.us.us.i74, label %bb60.i.us.us.us.us.us.us.us.i76, label %bb61.i.i75, !dbg !4002, !prof !2704

bb60.i.us.us.us.us.us.us.us.i76:                  ; preds = %bb45.i.us.us.us.us.us.us.us.i59
  %_189.i.us.us.us.us.us.us.us.i77 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_50.i.us.us.us.us.us.us.us.i73, !dbg !4007
  %_0.i271.us.us.us.us.us.us.us.i78 = load float, ptr %_189.i.us.us.us.us.us.us.us.i77, align 4, !dbg !4011, !alias.scope !4013, !noalias !3868, !noundef !12
  %_195.i.us.us.us.us.us.us.us.i79 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_50.i.us.us.us.us.us.us.us.i73, !dbg !4016
  %_0.i269.us.us.us.us.us.us.us.i80 = load float, ptr %_195.i.us.us.us.us.us.us.us.i79, align 4, !dbg !4024, !alias.scope !4026, !noalias !3868, !noundef !12
  %_66.i.us.us.us.us.us.us.us.i81 = sub i32 %now.i.us.us.us.us.us.us.us.i51, %_67.i.i
  %_65.i.us.us.us.us.us.us.us.i82 = and i32 %_66.i.us.us.us.us.us.us.us.i81, %_52.i30
  %_64.i.us.us.us.us.us.us.us.i83 = zext i32 %_65.i.us.us.us.us.us.us.us.i82 to i64
  %_74.i.us.us.us.us.us.us.us.i84 = sub i32 %now.i.us.us.us.us.us.us.us.i51, %_75.i.i
  %_73.i.us.us.us.us.us.us.us.i85 = and i32 %_74.i.us.us.us.us.us.us.us.i84, %_52.i30
  %_72.i.us.us.us.us.us.us.us.i86 = zext i32 %_73.i.us.us.us.us.us.us.us.i85 to i64
  %_80.i.us.us.us.us.us.us.us.i87 = icmp ugt i64 %_59.1.i22, %_64.i.us.us.us.us.us.us.us.i83
  %379 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_64.i.us.us.us.us.us.us.us.i83
  %380 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_64.i.us.us.us.us.us.us.us.i83
  %_86.i.us.us.us.us.us.us.us.i88 = icmp ugt i64 %_61.1.i27, %_72.i.us.us.us.us.us.us.us.i86
  %381 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_72.i.us.us.us.us.us.us.us.i86
  %_88.i.us.us.us.us.us.us.us.i89 = icmp ugt i64 %_59.1.i22, %_72.i.us.us.us.us.us.us.us.i86
  %382 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_72.i.us.us.us.us.us.us.us.i86
  br i1 %_80.i.us.us.us.us.us.us.us.i87, label %bb63.i.split.us.us.us.us.us.us.us.us.i91, label %bb63.i.split.i90

bb63.i.split.us.us.us.us.us.us.us.us.i91:         ; preds = %bb60.i.us.us.us.us.us.us.us.i76
  %_84.i.us.us.us.us.us.us.us.i92 = icmp ugt i64 %_61.1.i27, %_64.i.us.us.us.us.us.us.us.i83
  br i1 %_84.i.us.us.us.us.us.us.us.i92, label %bb24.i.us.lr.ph.us.us.us.us.us.us.us.i94, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i93, !dbg !4029

bb24.i.us.lr.ph.us.us.us.us.us.us.us.i94:         ; preds = %bb63.i.split.us.us.us.us.us.us.us.us.i91
  %_82.i.us.us.us.us.us.us.us.us.i95 = load float, ptr %380, align 4, !noalias !3868, !noundef !12
  br i1 %_86.i.us.us.us.us.us.us.us.i88, label %bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i97, label %bb24.i.us.lr.ph.split.i96

bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i97: ; preds = %bb24.i.us.lr.ph.us.us.us.us.us.us.us.i94
  br i1 %_88.i.us.us.us.us.us.us.us.i89, label %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i98, !dbg !4037

bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99:      ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i97
  %_87.i.us.us.us.us.us.us.us.us.us.i100 = load float, ptr %382, align 4, !noalias !3868, !noundef !12
  %_85.i.us.us.le827.us.us.us.us.us.us.us.i = load float, ptr %381, align 4, !noalias !3868, !noundef !12
  %_78.i.us.le.us.us.us.us.us.us.us.i101 = load float, ptr %379, align 4, !noalias !3868, !noundef !12
  %383 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.us.us.us.us.i101), !dbg !4038
  %384 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.us.us.us.us.i95), !dbg !4048
  %_3.i.i466.us.us.us.us.us.us.us.i = fcmp ule float %383, %384, !dbg !4051
  %_6.i.i468.us.us.us.us.us.us.us.i = bitcast float %383 to i32, !dbg !4056
  %_8.i.i470.us.us.us.us.us.us.us.i = bitcast float %384 to i32, !dbg !4059
  %_4.i.i473.us.us.us.us.us.us.us.i = select i1 %_3.i.i466.us.us.us.us.us.us.us.i, i32 %_8.i.i470.us.us.us.us.us.us.us.i, i32 %_6.i.i468.us.us.us.us.us.us.us.i, !dbg !4061
  %_4.i346.us.us.us.us.us.us.us.i = select i1 %_3.i190.i, i32 %_6.i.i468.us.us.us.us.us.us.us.i, i32 %_4.i.i473.us.us.us.us.us.us.us.i, !dbg !4062
  %_0.i239.us.us.us.us.us.us.us.i = fmul float %383, 5.000000e-01, !dbg !4064
  %_0.i238.us.us.us.us.us.us.us.i = fmul float %384, 5.000000e-01, !dbg !4066
  %_0.i218.us.us.us.us.us.us.us.i = fadd float %_0.i238.us.us.us.us.us.us.us.i, %_0.i239.us.us.us.us.us.us.us.i, !dbg !4068
  %_6.i334.us.us.us.us.us.us.us.i = bitcast float %_0.i218.us.us.us.us.us.us.us.i to i32, !dbg !4070
  %_4.i339.us.us.us.us.us.us.us.i = select i1 %_3.i188.i, i32 %_4.i346.us.us.us.us.us.us.us.i, i32 %_6.i334.us.us.us.us.us.us.us.i, !dbg !4073
  %_0.i340.us.us.us.us.us.us.us.i = bitcast i32 %_4.i339.us.us.us.us.us.us.us.i to float, !dbg !4074
  %_3.i.i458.us.us.us.us.us.us.us.i = fcmp ule float %_0.i340.us.us.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4076
  %_4.i.i464.us.us.us.us.us.us.us.i = select i1 %_3.i.i458.us.us.us.us.us.us.us.i, i32 841731191, i32 %_4.i339.us.us.us.us.us.us.us.i, !dbg !4079
  %_0.i.i465.us.us.us.us.us.us.us.i = bitcast i32 %_4.i.i464.us.us.us.us.us.us.us.i to float, !dbg !4081
  %_3.i.i418.us.us.us.us.us.us.us.i = fcmp ule float %_0.i.i465.us.us.us.us.us.us.us.i, 0x3810000000000000, !dbg !4083
  %_4.i.i424.us.us.us.us.us.us.us.i = select i1 %_3.i.i418.us.us.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i464.us.us.us.us.us.us.us.i, !dbg !4089
  %_5.i284.us.us.us.us.us.us.us.i = and i32 %_4.i.i424.us.us.us.us.us.us.us.i, 8388607, !dbg !4091
  %_4.i285.us.us.us.us.us.us.us.i = or disjoint i32 %_5.i284.us.us.us.us.us.us.us.i, 1065353216, !dbg !4091
  %significand.i286.us.us.us.us.us.us.us.i = bitcast i32 %_4.i285.us.us.us.us.us.us.us.i to float, !dbg !4093
  %_0.i247.us.us.us.us.us.us.us.i102 = fadd float %significand.i286.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !4095
  %_0.i227.us.us.us.us.us.us.us.i = fmul float %_0.i247.us.us.us.us.us.us.us.i102, 0x3F9B17A960000000, !dbg !4097
  %385 = fsub float 0x3FBF9A8440000000, %_0.i227.us.us.us.us.us.us.us.i, !dbg !4099
  %_0.i227.us.us.us.us.us.us.us.1.i = fmul float %_0.i247.us.us.us.us.us.us.us.i102, %385, !dbg !4097
  %_0.i213.us.us.us.us.us.us.us.1.i = fadd float %_0.i227.us.us.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !4099
  %_0.i227.us.us.us.us.us.us.us.2.i = fmul float %_0.i247.us.us.us.us.us.us.us.i102, %_0.i213.us.us.us.us.us.us.us.1.i, !dbg !4097
  %_0.i213.us.us.us.us.us.us.us.2.i = fadd float %_0.i227.us.us.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !4099
  %_0.i227.us.us.us.us.us.us.us.3.i = fmul float %_0.i247.us.us.us.us.us.us.us.i102, %_0.i213.us.us.us.us.us.us.us.2.i, !dbg !4097
  %_0.i213.us.us.us.us.us.us.us.3.i = fadd float %_0.i227.us.us.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4099
  %_0.i227.us.us.us.us.us.us.us.4.i = fmul float %_0.i247.us.us.us.us.us.us.us.i102, %_0.i213.us.us.us.us.us.us.us.3.i, !dbg !4097
  %_0.i213.us.us.us.us.us.us.us.4.i = fadd float %_0.i227.us.us.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4099
  %_9.i287.us.us.us.us.us.us.us.i = lshr i32 %_4.i.i424.us.us.us.us.us.us.us.i, 23, !dbg !4101
  %_8.i288.us.us.us.us.us.us.us.i = or disjoint i32 %_9.i287.us.us.us.us.us.us.us.i, 1258291200, !dbg !4101
  %_7.i289.us.us.us.us.us.us.us.i = bitcast i32 %_8.i288.us.us.us.us.us.us.us.i to float, !dbg !4102
  %exponent.i290.us.us.us.us.us.us.us.i = fadd float %_7.i289.us.us.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !4104
  %_0.i226.us.us.us.us.us.us.us.i = fmul float %_0.i247.us.us.us.us.us.us.us.i102, %_0.i213.us.us.us.us.us.us.us.4.i, !dbg !4105
  %_0.i212.us.us.us.us.us.us.us.i = fadd float %exponent.i290.us.us.us.us.us.us.us.i, %_0.i226.us.us.us.us.us.us.us.i, !dbg !4107
  %_0.i237.us.us.us.us.us.us.us.i = fmul float %_0.i212.us.us.us.us.us.us.us.i, 0x4018151820000000, !dbg !4109
  %_3.i.i533.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i237.us.us.us.us.us.us.us.i, 2.400000e+01, !dbg !4111
  %_0.i.i540.us.us.us.us.us.us.us.i = select i1 %_3.i.i533.us.us.us.us.us.us.us.inv.i, float %_0.i237.us.us.us.us.us.us.us.i, float 2.400000e+01, !dbg !4111
  %_3.i.i450.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i540.us.us.us.us.us.us.us.i, -1.600000e+02, !dbg !4114
  %_0.i.i457.us.us.us.us.us.us.us.i = select i1 %_3.i.i450.us.us.us.us.us.us.us.inv.i, float %_0.i.i540.us.us.us.us.us.us.us.i, float -1.600000e+02, !dbg !4114
  %_55.i59.us.us.us.us.us.us.us.i = load float, ptr %359, align 4, !dbg !4117, !alias.scope !4119, !noalias !4122, !noundef !12
  %_3.i186.us.us.us.us.us.us.us.i = fcmp ule float %_55.i59.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4124
  %_3.i172.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i457.us.us.us.us.us.us.us.i, %threshold.i32.i, !dbg !4126
  %_3.i170.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i457.us.us.us.us.us.us.us.i, %_0.i254.i, !dbg !4129
  %..i171.us.us.us.us.us.us.us.i = sext i1 %_3.i170.us.us.us.us.us.us.us.i to i32, !dbg !4132
  %_0.i408.us.us.us.us.us.us.us.i = sext i1 %_3.i172.us.us.us.us.us.us.us.i to i32, !dbg !4134
  %_0.i402.us.us.us.us.us.us.us.i = select i1 %_3.i186.us.us.us.us.us.us.us.i, i32 %_0.i408.us.us.us.us.us.us.us.i, i32 %..i171.us.us.us.us.us.us.us.i, !dbg !4134
  %_0.i414.us.us.us.us.us.us.us.i = xor i32 %..i171.us.us.us.us.us.us.us.i, -1, !dbg !4139
  %_3.i184.us.us.us.us.us.us.us.i = fcmp ogt float %_67.i691001.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4142
  %_0.i407.us.us.us.us.us.us.us.i103 = select i1 %_3.i184.us.us.us.us.us.us.us.i, i32 %_0.i414.us.us.us.us.us.us.us.i, i32 0, !dbg !4144
  %_0.i406.us.us.us.us.us.us.us.i = select i1 %_3.i186.us.us.us.us.us.us.us.i, i32 0, i32 %_0.i407.us.us.us.us.us.us.us.i103, !dbg !4146
  %_0.i401.us.us.us.us.us.us.us.i = or i32 %_0.i406.us.us.us.us.us.us.us.i, %_0.i402.us.us.us.us.us.us.us.i, !dbg !4148
  %_5.i329.us.us.us.us.us.us.us.i = and i32 %_0.i401.us.us.us.us.us.us.us.i, 1065353216, !dbg !4151
  %_0.i333.us.us.us.us.us.us.us.i = bitcast i32 %_5.i329.us.us.us.us.us.us.us.i to float, !dbg !4153
  %_0.i253.us.us.us.us.us.us.us.i104 = fadd float %_67.i691001.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !4155
  %386 = trunc nsw i32 %_0.i406.us.us.us.us.us.us.us.i to i1, !dbg !4158
  %_4.i327.v.us.us.us.us.us.us.us.i = select i1 %386, float %_0.i253.us.us.us.us.us.us.us.i104, float %_67.i691001.us.us.us.us.us.us.us.i, !dbg !4158
  %387 = trunc nsw i32 %_0.i402.us.us.us.us.us.us.us.i to i1, !dbg !4160
  %_0.i321.us.us.us.us.us.us.us.i = select i1 %387, float %_71.i75568569.i, float %_4.i327.v.us.us.us.us.us.us.us.i, !dbg !4160
  store float %_0.i321.us.us.us.us.us.us.us.i, ptr %360, align 4, !dbg !4162, !alias.scope !4119, !noalias !4122
  store i32 %_5.i329.us.us.us.us.us.us.us.i, ptr %359, align 4, !dbg !4163, !alias.scope !4119, !noalias !4122
  %_0.i251.us.us.us.us.us.us.us.i105 = fsub float %_0.i.i457.us.us.us.us.us.us.us.i, %threshold.i32.i, !dbg !4164
  %_0.i236.us.us.us.us.us.us.us.i = fmul float %_0.i252.i, %_0.i251.us.us.us.us.us.us.us.i105, !dbg !4166
  %_3.i.i442.inv.us.us.us.us.us.us.us.i = fcmp ogt float %_0.i236.us.us.us.us.us.us.us.i, %362, !dbg !4168
  %_4.i.i448.v.us.us.us.us.us.us.us.i = select i1 %_3.i.i442.inv.us.us.us.us.us.us.us.i, float %_0.i236.us.us.us.us.us.us.us.i, float %362, !dbg !4168
  %_3.i.i525.us.us.us.us.us.us.us.i = fcmp olt float %_4.i.i448.v.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4171
  %388 = fcmp ule float %_0.i333.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4174
  %389 = select i1 %388, i1 %_3.i.i525.us.us.us.us.us.us.us.i, i1 false, !dbg !4177
  %_0.i314.us.us.us.us.us.us.us.i = select i1 %389, float %_4.i.i448.v.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !4177
  %_3.i180.us.us.us.us.us.us.us.i = fcmp ule float %_0.i314.us.us.us.us.us.us.us.i, %_86.i881003.us.us.us.us.us.us.us.i, !dbg !4178
  %_4.i307.us.us.us.us.us.us.us.i = select i1 %_3.i180.us.us.us.us.us.us.us.i, i32 %_88.i91572.i, i32 %_87.i90571.i, !dbg !4181
  %_0.i308.us.us.us.us.us.us.us.i = bitcast i32 %_4.i307.us.us.us.us.us.us.us.i to float, !dbg !4183
  %_0.i250.us.us.us.us.us.us.us.i106 = fsub float %_0.i314.us.us.us.us.us.us.us.i, %_86.i881003.us.us.us.us.us.us.us.i, !dbg !4185
  %_4.i220.us.us.us.us.us.us.us.i = fmul float %_0.i250.us.us.us.us.us.us.us.i106, %_0.i308.us.us.us.us.us.us.us.i, !dbg !4188
  %_0.i221.us.us.us.us.us.us.us.i = fadd float %_86.i881003.us.us.us.us.us.us.us.i, %_4.i220.us.us.us.us.us.us.us.i, !dbg !4188
  %390 = tail call noundef float @llvm.fabs.f32(float %_0.i221.us.us.us.us.us.us.us.i), !dbg !4190
  %391 = fcmp uge float %390, 0x3BC79CA100000000, !dbg !4194
  %_0.i292.us.us.us.us.us.us.us.i107 = select i1 %391, float %_0.i221.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !4196
  store float %_0.i292.us.us.us.us.us.us.us.i107, ptr %363, align 4, !dbg !4197, !alias.scope !4119, !noalias !4122
  %_0.i235.us.us.us.us.us.us.us.i = fmul float %_0.i292.us.us.us.us.us.us.us.i107, 0x3FC542A5A0000000, !dbg !4199
  %_3.i.i434.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i235.us.us.us.us.us.us.us.i, -1.260000e+02, !dbg !4203
  %_0.i.i441.us.us.us.us.us.us.us.i = select i1 %_3.i.i434.us.us.us.us.us.us.us.inv.i, float %_0.i235.us.us.us.us.us.us.us.i, float -1.260000e+02, !dbg !4203
  %_3.i.i517.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i441.us.us.us.us.us.us.us.i, 1.270000e+02, !dbg !4207
  %_0.i.i524.us.us.us.us.us.us.us.i = select i1 %_3.i.i517.us.us.us.us.us.us.us.inv.i, float %_0.i.i441.us.us.us.us.us.us.us.i, float 1.270000e+02, !dbg !4207
  %392 = tail call noundef float @llvm.floor.f32(float %_0.i.i524.us.us.us.us.us.us.us.i), !dbg !4210
  %_0.i249.us.us.us.us.us.us.us.i108 = fsub float %_0.i.i524.us.us.us.us.us.us.us.i, %392, !dbg !4214
  %393 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le827.us.us.us.us.us.us.us.i), !dbg !4216
  %394 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.us.us.us.us.i100), !dbg !4220
  %_3.i.i500.us.us.us.us.us.us.us.i = fcmp ule float %393, %394, !dbg !4222
  %_6.i.i502.us.us.us.us.us.us.us.i = bitcast float %393 to i32, !dbg !4225
  %_8.i.i504.us.us.us.us.us.us.us.i = bitcast float %394 to i32, !dbg !4228
  %_4.i.i507.us.us.us.us.us.us.us.i = select i1 %_3.i.i500.us.us.us.us.us.us.us.i, i32 %_8.i.i504.us.us.us.us.us.us.us.i, i32 %_6.i.i502.us.us.us.us.us.us.us.i, !dbg !4230
  %_4.i398.us.us.us.us.us.us.us.i = select i1 %_3.i204.i, i32 %_6.i.i502.us.us.us.us.us.us.us.i, i32 %_4.i.i507.us.us.us.us.us.us.us.i, !dbg !4231
  %_0.i245.us.us.us.us.us.us.us.i = fmul float %393, 5.000000e-01, !dbg !4233
  %_0.i244.us.us.us.us.us.us.us.i = fmul float %394, 5.000000e-01, !dbg !4235
  %_0.i219.us.us.us.us.us.us.us.i = fadd float %_0.i244.us.us.us.us.us.us.us.i, %_0.i245.us.us.us.us.us.us.us.i, !dbg !4237
  %_6.i386.us.us.us.us.us.us.us.i = bitcast float %_0.i219.us.us.us.us.us.us.us.i to i32, !dbg !4239
  %_4.i391.us.us.us.us.us.us.us.i = select i1 %_3.i202.i, i32 %_4.i398.us.us.us.us.us.us.us.i, i32 %_6.i386.us.us.us.us.us.us.us.i, !dbg !4242
  %_0.i392.us.us.us.us.us.us.us.i = bitcast i32 %_4.i391.us.us.us.us.us.us.us.i to float, !dbg !4243
  %_3.i.i492.us.us.us.us.us.us.us.i = fcmp ule float %_0.i392.us.us.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4245
  %_4.i.i498.us.us.us.us.us.us.us.i = select i1 %_3.i.i492.us.us.us.us.us.us.us.i, i32 841731191, i32 %_4.i391.us.us.us.us.us.us.us.i, !dbg !4248
  %_0.i.i499.us.us.us.us.us.us.us.i = bitcast i32 %_4.i.i498.us.us.us.us.us.us.us.i to float, !dbg !4250
  %_3.i.i.us.us.us.us.us.us.us.i109 = fcmp ule float %_0.i.i499.us.us.us.us.us.us.us.i, 0x3810000000000000, !dbg !4252
  %_4.i.i.us.us.us.us.us.us.us.i110 = select i1 %_3.i.i.us.us.us.us.us.us.us.i109, i32 8388608, i32 %_4.i.i498.us.us.us.us.us.us.us.i, !dbg !4257
  %_5.i280.us.us.us.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.us.us.us.i110, 8388607, !dbg !4259
  %_4.i281.us.us.us.us.us.us.us.i = or disjoint i32 %_5.i280.us.us.us.us.us.us.us.i, 1065353216, !dbg !4259
  %significand.i.us.us.us.us.us.us.us.i111 = bitcast i32 %_4.i281.us.us.us.us.us.us.us.i to float, !dbg !4261
  %_0.i246.us.us.us.us.us.us.us.i112 = fadd float %significand.i.us.us.us.us.us.us.us.i111, -1.000000e+00, !dbg !4263
  %_0.i225.us.us.us.us.us.us.us.i = fmul float %_0.i246.us.us.us.us.us.us.us.i112, 0x3F9B17A960000000, !dbg !4265
  %395 = fsub float 0x3FBF9A8440000000, %_0.i225.us.us.us.us.us.us.us.i, !dbg !4267
  %_0.i225.us.us.us.us.us.us.us.1.i = fmul float %_0.i246.us.us.us.us.us.us.us.i112, %395, !dbg !4265
  %_0.i211.us.us.us.us.us.us.us.1.i = fadd float %_0.i225.us.us.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !4267
  %_0.i225.us.us.us.us.us.us.us.2.i = fmul float %_0.i246.us.us.us.us.us.us.us.i112, %_0.i211.us.us.us.us.us.us.us.1.i, !dbg !4265
  %_0.i211.us.us.us.us.us.us.us.2.i = fadd float %_0.i225.us.us.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !4267
  %_0.i225.us.us.us.us.us.us.us.3.i = fmul float %_0.i246.us.us.us.us.us.us.us.i112, %_0.i211.us.us.us.us.us.us.us.2.i, !dbg !4265
  %_0.i211.us.us.us.us.us.us.us.3.i = fadd float %_0.i225.us.us.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4267
  %_0.i225.us.us.us.us.us.us.us.4.i = fmul float %_0.i246.us.us.us.us.us.us.us.i112, %_0.i211.us.us.us.us.us.us.us.3.i, !dbg !4265
  %_0.i211.us.us.us.us.us.us.us.4.i = fadd float %_0.i225.us.us.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4267
  %_9.i.us.us.us.us.us.us.us.i113 = lshr i32 %_4.i.i.us.us.us.us.us.us.us.i110, 23, !dbg !4269
  %_8.i282.us.us.us.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.us.us.us.i113, 1258291200, !dbg !4269
  %_7.i.us.us.us.us.us.us.us.i114 = bitcast i32 %_8.i282.us.us.us.us.us.us.us.i to float, !dbg !4270
  %exponent.i.us.us.us.us.us.us.us.i115 = fadd float %_7.i.us.us.us.us.us.us.us.i114, 0xC160000FE0000000, !dbg !4272
  %_0.i224.us.us.us.us.us.us.us.i = fmul float %_0.i246.us.us.us.us.us.us.us.i112, %_0.i211.us.us.us.us.us.us.us.4.i, !dbg !4273
  %_0.i210.us.us.us.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.us.us.us.i115, %_0.i224.us.us.us.us.us.us.us.i, !dbg !4275
  %_0.i243.us.us.us.us.us.us.us.i = fmul float %_0.i210.us.us.us.us.us.us.us.i, 0x4018151820000000, !dbg !4277
  %_3.i.i549.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i243.us.us.us.us.us.us.us.i, 2.400000e+01, !dbg !4279
  %_0.i.i556.us.us.us.us.us.us.us.i = select i1 %_3.i.i549.us.us.us.us.us.us.us.inv.i, float %_0.i243.us.us.us.us.us.us.us.i, float 2.400000e+01, !dbg !4279
  %_3.i.i484.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i556.us.us.us.us.us.us.us.i, -1.600000e+02, !dbg !4282
  %_0.i.i491.us.us.us.us.us.us.us.i = select i1 %_3.i.i484.us.us.us.us.us.us.us.inv.i, float %_0.i.i556.us.us.us.us.us.us.us.i, float -1.600000e+02, !dbg !4282
  %_55.i11.us.us.us.us.us.us.us.i = load float, ptr %371, align 4, !dbg !4285, !alias.scope !4286, !noalias !4289, !noundef !12
  %_3.i200.us.us.us.us.us.us.us.i116 = fcmp ule float %_55.i11.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4291
  %_3.i176.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i491.us.us.us.us.us.us.us.i, %threshold.i.i, !dbg !4293
  %_3.i174.us.us.us.us.us.us.us.i = fcmp oge float %_0.i.i491.us.us.us.us.us.us.us.i, %_0.i259.i, !dbg !4295
  %..i175.us.us.us.us.us.us.us.i = sext i1 %_3.i174.us.us.us.us.us.us.us.i to i32, !dbg !4297
  %_0.i412.us.us.us.us.us.us.us.i = sext i1 %_3.i176.us.us.us.us.us.us.us.i to i32, !dbg !4299
  %_0.i405.us.us.us.us.us.us.us.i = select i1 %_3.i200.us.us.us.us.us.us.us.i116, i32 %_0.i412.us.us.us.us.us.us.us.i, i32 %..i175.us.us.us.us.us.us.us.i, !dbg !4299
  %_0.i416.us.us.us.us.us.us.us.i = xor i32 %..i175.us.us.us.us.us.us.us.i, -1, !dbg !4301
  %_3.i198.us.us.us.us.us.us.us.i117 = fcmp ogt float %_67.i131005.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4303
  %_0.i411.us.us.us.us.us.us.us.i = select i1 %_3.i198.us.us.us.us.us.us.us.i117, i32 %_0.i416.us.us.us.us.us.us.us.i, i32 0, !dbg !4305
  %_0.i410.us.us.us.us.us.us.us.i = select i1 %_3.i200.us.us.us.us.us.us.us.i116, i32 0, i32 %_0.i411.us.us.us.us.us.us.us.i, !dbg !4307
  %_0.i404.us.us.us.us.us.us.us.i = or i32 %_0.i410.us.us.us.us.us.us.us.i, %_0.i405.us.us.us.us.us.us.us.i, !dbg !4309
  %_5.i381.us.us.us.us.us.us.us.i = and i32 %_0.i404.us.us.us.us.us.us.us.i, 1065353216, !dbg !4311
  %_0.i385.us.us.us.us.us.us.us.i = bitcast i32 %_5.i381.us.us.us.us.us.us.us.i to float, !dbg !4313
  %_0.i258.us.us.us.us.us.us.us.i = fadd float %_67.i131005.us.us.us.us.us.us.us.i, -1.000000e+00, !dbg !4315
  %396 = trunc nsw i32 %_0.i410.us.us.us.us.us.us.us.i to i1, !dbg !4317
  %_4.i379.v.us.us.us.us.us.us.us.i = select i1 %396, float %_0.i258.us.us.us.us.us.us.us.i, float %_67.i131005.us.us.us.us.us.us.us.i, !dbg !4317
  %397 = trunc nsw i32 %_0.i405.us.us.us.us.us.us.us.i to i1, !dbg !4319
  %_0.i373.us.us.us.us.us.us.us.i118 = select i1 %397, float %_71.i585586.i, float %_4.i379.v.us.us.us.us.us.us.us.i, !dbg !4319
  store float %_0.i373.us.us.us.us.us.us.us.i118, ptr %372, align 4, !dbg !4321, !alias.scope !4286, !noalias !4289
  store i32 %_5.i381.us.us.us.us.us.us.us.i, ptr %371, align 4, !dbg !4322, !alias.scope !4286, !noalias !4289
  %_0.i256.us.us.us.us.us.us.us.i119 = fsub float %_0.i.i491.us.us.us.us.us.us.us.i, %threshold.i.i, !dbg !4323
  %_0.i242.us.us.us.us.us.us.us.i = fmul float %_0.i257.i, %_0.i256.us.us.us.us.us.us.us.i119, !dbg !4325
  %_3.i.i475.inv.us.us.us.us.us.us.us.i = fcmp ogt float %_0.i242.us.us.us.us.us.us.us.i, %374, !dbg !4327
  %_4.i.i482.v.us.us.us.us.us.us.us.i = select i1 %_3.i.i475.inv.us.us.us.us.us.us.us.i, float %_0.i242.us.us.us.us.us.us.us.i, float %374, !dbg !4327
  %_3.i.i541.us.us.us.us.us.us.us.i = fcmp olt float %_4.i.i482.v.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4330
  %398 = fcmp ule float %_0.i385.us.us.us.us.us.us.us.i, 0.000000e+00, !dbg !4333
  %399 = select i1 %398, i1 %_3.i.i541.us.us.us.us.us.us.us.i, i1 false, !dbg !4335
  %_0.i366.us.us.us.us.us.us.us.i = select i1 %399, float %_4.i.i482.v.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !4335
  %_3.i194.us.us.us.us.us.us.us.i = fcmp ule float %_0.i366.us.us.us.us.us.us.us.i, %_86.i211007.us.us.us.us.us.us.us.i, !dbg !4336
  %_4.i360.us.us.us.us.us.us.us.i = select i1 %_3.i194.us.us.us.us.us.us.us.i, i32 %_88.i24589.i, i32 %_87.i23588.i, !dbg !4338
  %_0.i.us.us.us.us.us.us.us.i120 = bitcast i32 %_4.i360.us.us.us.us.us.us.us.i to float, !dbg !4340
  %_0.i255.us.us.us.us.us.us.us.i121 = fsub float %_0.i366.us.us.us.us.us.us.us.i, %_86.i211007.us.us.us.us.us.us.us.i, !dbg !4342
  %_4.i222.us.us.us.us.us.us.us.i = fmul float %_0.i255.us.us.us.us.us.us.us.i121, %_0.i.us.us.us.us.us.us.us.i120, !dbg !4344
  %_0.i223.us.us.us.us.us.us.us.i = fadd float %_86.i211007.us.us.us.us.us.us.us.i, %_4.i222.us.us.us.us.us.us.us.i, !dbg !4344
  %400 = tail call noundef float @llvm.fabs.f32(float %_0.i223.us.us.us.us.us.us.us.i), !dbg !4346
  %401 = fcmp uge float %400, 0x3BC79CA100000000, !dbg !4349
  %_0.i296.us.us.us.us.us.us.us.i122 = select i1 %401, float %_0.i223.us.us.us.us.us.us.us.i, float 0.000000e+00, !dbg !4351
  store float %_0.i296.us.us.us.us.us.us.us.i122, ptr %375, align 4, !dbg !4352, !alias.scope !4286, !noalias !4289
  %_0.i241.us.us.us.us.us.us.us.i = fmul float %_0.i296.us.us.us.us.us.us.us.i122, 0x3FC542A5A0000000, !dbg !4353
  %_3.i.i426.us.us.us.us.us.us.us.inv.i = fcmp ogt float %_0.i241.us.us.us.us.us.us.us.i, -1.260000e+02, !dbg !4356
  %_0.i.i433.us.us.us.us.us.us.us.i = select i1 %_3.i.i426.us.us.us.us.us.us.us.inv.i, float %_0.i241.us.us.us.us.us.us.us.i, float -1.260000e+02, !dbg !4356
  %_3.i.i509.us.us.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i433.us.us.us.us.us.us.us.i, 1.270000e+02, !dbg !4360
  %_0.i.i516.us.us.us.us.us.us.us.i = select i1 %_3.i.i509.us.us.us.us.us.us.us.inv.i, float %_0.i.i433.us.us.us.us.us.us.us.i, float 1.270000e+02, !dbg !4360
  %402 = tail call noundef float @llvm.floor.f32(float %_0.i.i516.us.us.us.us.us.us.us.i), !dbg !4363
  %_0.i248.us.us.us.us.us.us.us.i123 = fsub float %_0.i.i516.us.us.us.us.us.us.us.i, %402, !dbg !4367
  %_0.i230.us.us.us.us.us.us.us.i = fmul float %_0.i248.us.us.us.us.us.us.us.i123, 0x3F5E974FA0000000, !dbg !4369
  %_0.i215.us.us.us.us.us.us.us.i = fadd float %_0.i230.us.us.us.us.us.us.us.i, 0x3F82778560000000, !dbg !4371
  %_0.i230.us.us.us.us.us.us.us.1.i = fmul float %_0.i248.us.us.us.us.us.us.us.i123, %_0.i215.us.us.us.us.us.us.us.i, !dbg !4369
  %_0.i215.us.us.us.us.us.us.us.1.i = fadd float %_0.i230.us.us.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4371
  %_0.i230.us.us.us.us.us.us.us.2.i = fmul float %_0.i248.us.us.us.us.us.us.us.i123, %_0.i215.us.us.us.us.us.us.us.1.i, !dbg !4369
  %_0.i215.us.us.us.us.us.us.us.2.i = fadd float %_0.i230.us.us.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4371
  %_0.i230.us.us.us.us.us.us.us.3.i = fmul float %_0.i248.us.us.us.us.us.us.us.i123, %_0.i215.us.us.us.us.us.us.us.2.i, !dbg !4369
  %_0.i215.us.us.us.us.us.us.us.3.i = fadd float %_0.i230.us.us.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4371
  %_0.i233.us.us.us.us.us.us.us.i = fmul float %_0.i249.us.us.us.us.us.us.us.i108, 0x3F5E974FA0000000, !dbg !4373
  %_0.i217.us.us.us.us.us.us.us.i = fadd float %_0.i233.us.us.us.us.us.us.us.i, 0x3F82778560000000, !dbg !4375
  %_0.i233.us.us.us.us.us.us.us.1.i = fmul float %_0.i249.us.us.us.us.us.us.us.i108, %_0.i217.us.us.us.us.us.us.us.i, !dbg !4373
  %_0.i217.us.us.us.us.us.us.us.1.i = fadd float %_0.i233.us.us.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4375
  %_0.i233.us.us.us.us.us.us.us.2.i = fmul float %_0.i249.us.us.us.us.us.us.us.i108, %_0.i217.us.us.us.us.us.us.us.1.i, !dbg !4373
  %_0.i217.us.us.us.us.us.us.us.2.i = fadd float %_0.i233.us.us.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4375
  %_0.i233.us.us.us.us.us.us.us.3.i = fmul float %_0.i249.us.us.us.us.us.us.us.i108, %_0.i217.us.us.us.us.us.us.us.2.i, !dbg !4373
  %_0.i217.us.us.us.us.us.us.us.3.i = fadd float %_0.i233.us.us.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4375
  %_0.i232.us.us.us.us.us.us.us.i = fmul float %_0.i249.us.us.us.us.us.us.us.i108, %_0.i217.us.us.us.us.us.us.us.3.i, !dbg !4377
  %_0.i216.us.us.us.us.us.us.us.i = fadd float %_0.i232.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !4379
  %biased.i163.us.us.us.us.us.us.us.i = fadd float %392, 0x4160000FE0000000, !dbg !4381
  %_4.i164.us.us.us.us.us.us.us.i = bitcast float %biased.i163.us.us.us.us.us.us.us.i to i32, !dbg !4383
  %_3.i165.us.us.us.us.us.us.us.i = shl i32 %_4.i164.us.us.us.us.us.us.us.i, 23, !dbg !4385
  %_0.i166.us.us.us.us.us.us.us.i = bitcast i32 %_3.i165.us.us.us.us.us.us.us.i to float, !dbg !4386
  %_0.i231.us.us.us.us.us.us.us.i = fmul float %_0.i216.us.us.us.us.us.us.us.i, %_0.i166.us.us.us.us.us.us.us.i, !dbg !4388
  %_3.i167.us.us.us.us.us.us.us.i = fcmp une float %_0.i292.us.us.us.us.us.us.us.i107, 0.000000e+00, !dbg !4390
  %_0.i400576.not.us.us.us.us.us.us.us.i = and i1 %_3.i178.i, %_3.i167.us.us.us.us.us.us.us.i, !dbg !4393
  %_0.i234.us.us.us.us.us.us.us.i = fmul float %_0.i271.us.us.us.us.us.us.us.i78, %_0.i231.us.us.us.us.us.us.us.i, !dbg !4393
  %_4.i300.v.us.us.us.us.us.us.us.i = select i1 %_0.i400576.not.us.us.us.us.us.us.us.i, float %_0.i234.us.us.us.us.us.us.us.i, float %_0.i271.us.us.us.us.us.us.us.i78, !dbg !4396
  %_0.i229.us.us.us.us.us.us.us.i = fmul float %_0.i248.us.us.us.us.us.us.us.i123, %_0.i215.us.us.us.us.us.us.us.3.i, !dbg !4398
  %_0.i214.us.us.us.us.us.us.us.i = fadd float %_0.i229.us.us.us.us.us.us.us.i, 1.000000e+00, !dbg !4400
  %biased.i.us.us.us.us.us.us.us.i124 = fadd float %402, 0x4160000FE0000000, !dbg !4402
  %_4.i160.us.us.us.us.us.us.us.i = bitcast float %biased.i.us.us.us.us.us.us.us.i124 to i32, !dbg !4404
  %_3.i161.us.us.us.us.us.us.us.i = shl i32 %_4.i160.us.us.us.us.us.us.us.i, 23, !dbg !4406
  %_0.i162.us.us.us.us.us.us.us.i = bitcast i32 %_3.i161.us.us.us.us.us.us.us.i to float, !dbg !4407
  %_0.i228.us.us.us.us.us.us.us.i = fmul float %_0.i214.us.us.us.us.us.us.us.i, %_0.i162.us.us.us.us.us.us.us.i, !dbg !4409
  %_3.i168.us.us.us.us.us.us.us.i = fcmp une float %_0.i296.us.us.us.us.us.us.us.i122, 0.000000e+00, !dbg !4411
  %_0.i403593.not.us.us.us.us.us.us.us.i = and i1 %_3.i192.i, %_3.i168.us.us.us.us.us.us.us.i, !dbg !4413
  %_0.i240.us.us.us.us.us.us.us.i = fmul float %_0.i269.us.us.us.us.us.us.us.i80, %_0.i228.us.us.us.us.us.us.us.i, !dbg !4413
  %_4.i353.v.us.us.us.us.us.us.us.i = select i1 %_0.i403593.not.us.us.us.us.us.us.us.i, float %_0.i240.us.us.us.us.us.us.us.i, float %_0.i269.us.us.us.us.us.us.us.i80, !dbg !4415
  store float %_4.i300.v.us.us.us.us.us.us.us.i, ptr %_123.i.us.us.us.us.us.us.us.i56, align 4, !dbg !4417, !alias.scope !4420, !noalias !3914
  store float %_4.i353.v.us.us.us.us.us.us.us.i, ptr %_141.i.us.us.us.us.us.us.us.i62, align 4, !dbg !4423, !alias.scope !4425, !noalias !3936
  %exitcond2525.not.i = icmp eq i64 %378, %_26, !dbg !4428
  br i1 %exitcond2525.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb40.i.us.us.us.us.us.us.us.i49, !dbg !4431

bb40.i.us.us.us.us.us.i128:                       ; preds = %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178
  %_86.i211007.us.us.us.us.us.i = phi float [ %_0.i296.us.us.us.us.us.i202, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178 ], [ %.promoted1006.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %_67.i131005.us.us.us.us.us.i = phi float [ %_0.i373.us.us.us.us.us.i198, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178 ], [ %.promoted1004.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %_86.i881003.us.us.us.us.us.i = phi float [ %_0.i292.us.us.us.us.us.i187, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178 ], [ %.promoted1002.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %_67.i691001.us.us.us.us.us.i = phi float [ %_0.i321.us.us.us.us.us.i, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178 ], [ %.promoted1000.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %iter.sroa.0.0.i966.us.us.us.us.us.i = phi i64 [ %403, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178 ], [ 0, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.split.us.i ]
  %403 = add nuw nsw i64 %iter.sroa.0.0.i966.us.us.us.us.us.i, 1, !dbg !3871
  %_27.i.us.us.us.us.us.i129 = trunc i64 %iter.sroa.0.0.i966.us.us.us.us.us.i to i32, !dbg !3885
  %now.i.us.us.us.us.us.i130 = add i32 %base.i.i46, %_27.i.us.us.us.us.us.i129, !dbg !3888
  %_30.i.us.us.us.us.us.i131 = and i32 %now.i.us.us.us.us.us.i130, %_52.i30, !dbg !3891
  %_29.i.us.us.us.us.us.i132 = zext i32 %_30.i.us.us.us.us.us.i131 to i64, !dbg !3893
  %exitcond2528.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.us.us.us.us.i, %_55, !dbg !3894
  br i1 %exitcond2528.not.i, label %bb43.i.i127, label %bb42.i.us.us.us.us.us.i133, !dbg !3894, !prof !180

bb42.i.us.us.us.us.us.i133:                       ; preds = %bb40.i.us.us.us.us.us.i128
  %_123.i.us.us.us.us.us.i134 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i966.us.us.us.us.us.i, !dbg !3900
  %_124.not.not.i.us.us.us.us.us.i135 = icmp ugt i64 %_58.1.i20, %_29.i.us.us.us.us.us.i132, !dbg !3904
  br i1 %_124.not.not.i.us.us.us.us.us.i135, label %bb45.i.us.us.us.us.us.i136, label %bb46.i.i58, !dbg !3904, !prof !2704

bb45.i.us.us.us.us.us.i136:                       ; preds = %bb42.i.us.us.us.us.us.i133
  %_0.i279.us.us.us.us.us.i137 = load float, ptr %_123.i.us.us.us.us.us.i134, align 4, !dbg !3909, !alias.scope !3911, !noalias !3914, !noundef !12
  %_133.i.us.us.us.us.us.i138 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_29.i.us.us.us.us.us.i132, !dbg !3915
  store float %_0.i279.us.us.us.us.us.i137, ptr %_133.i.us.us.us.us.us.i138, align 4, !dbg !3919, !alias.scope !3921, !noalias !3868
  %_141.i.us.us.us.us.us.i139 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i966.us.us.us.us.us.i, !dbg !3924
  %_0.i277.us.us.us.us.us.i140 = load float, ptr %_141.i.us.us.us.us.us.i139, align 4, !dbg !3931, !alias.scope !3933, !noalias !3936, !noundef !12
  %_149.i.us.us.us.us.us.i141 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_29.i.us.us.us.us.us.i132, !dbg !3937
  store float %_0.i277.us.us.us.us.us.i140, ptr %_149.i.us.us.us.us.us.i141, align 4, !dbg !3944, !alias.scope !3946, !noalias !3868
  %_157.i.us.us.us.us.us.i142 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i44, i64 %iter.sroa.0.0.i966.us.us.us.us.us.i, !dbg !3949
  %_0.i275.us.us.us.us.us.i143 = load float, ptr %_157.i.us.us.us.us.us.i142, align 4, !dbg !3956, !alias.scope !3958, !noalias !3868, !noundef !12
  %_165.i.us.us.us.us.us.i144 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_29.i.us.us.us.us.us.i132, !dbg !3961
  store float %_0.i275.us.us.us.us.us.i143, ptr %_165.i.us.us.us.us.us.i144, align 4, !dbg !3968, !alias.scope !3970, !noalias !3868
  %_174.not.not.i.us.us.us.us.us.i145 = icmp ugt i64 %_61.1.i27, %_29.i.us.us.us.us.us.i132, !dbg !4432
  br i1 %_174.not.not.i.us.us.us.us.us.i145, label %bb58.i.us.us.us.us.us.i148, label %bb59.i.i146, !dbg !4432, !prof !2704

bb58.i.us.us.us.us.us.i148:                       ; preds = %bb45.i.us.us.us.us.us.i136
  %_173.i.us.us.us.us.us.i149 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i43, i64 %iter.sroa.0.0.i966.us.us.us.us.us.i, !dbg !3973
  %_0.i273.us.us.us.us.us.i150 = load float, ptr %_173.i.us.us.us.us.us.i149, align 4, !dbg !3980, !alias.scope !3982, !noalias !3868, !noundef !12
  %_181.i.us.us.us.us.us.i151 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_29.i.us.us.us.us.us.i132, !dbg !3985
  store float %_0.i273.us.us.us.us.us.i150, ptr %_181.i.us.us.us.us.us.i151, align 4, !dbg !3992, !alias.scope !3994, !noalias !3868
  %_52.i.us.us.us.us.us.i152 = sub i32 %now.i.us.us.us.us.us.i130, %_53.i31, !dbg !3997
  %_51.i.us.us.us.us.us.i153 = and i32 %_52.i.us.us.us.us.us.i152, %_52.i30, !dbg !4000
  %_50.i.us.us.us.us.us.i154 = zext i32 %_51.i.us.us.us.us.us.i153 to i64, !dbg !4001
  %_182.not.not.i.us.us.us.us.us.i155 = icmp ugt i64 %_58.1.i20, %_50.i.us.us.us.us.us.i154, !dbg !4002
  br i1 %_182.not.not.i.us.us.us.us.us.i155, label %bb60.i.us.us.us.us.us.i156, label %bb61.i.i75, !dbg !4002, !prof !2704

bb60.i.us.us.us.us.us.i156:                       ; preds = %bb58.i.us.us.us.us.us.i148
  %_189.i.us.us.us.us.us.i157 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_50.i.us.us.us.us.us.i154, !dbg !4007
  %_0.i271.us.us.us.us.us.i158 = load float, ptr %_189.i.us.us.us.us.us.i157, align 4, !dbg !4011, !alias.scope !4013, !noalias !3868, !noundef !12
  %_195.i.us.us.us.us.us.i163 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_50.i.us.us.us.us.us.i154, !dbg !4016
  %_0.i269.us.us.us.us.us.i164 = load float, ptr %_195.i.us.us.us.us.us.i163, align 4, !dbg !4024, !alias.scope !4026, !noalias !3868, !noundef !12
  %_66.i.us.us.us.us.us.i165 = sub i32 %now.i.us.us.us.us.us.i130, %_67.i.i
  %_65.i.us.us.us.us.us.i166 = and i32 %_66.i.us.us.us.us.us.i165, %_52.i30
  %_64.i.us.us.us.us.us.i167 = zext i32 %_65.i.us.us.us.us.us.i166 to i64
  %_74.i.us.us.us.us.us.i168 = sub i32 %now.i.us.us.us.us.us.i130, %_75.i.i
  %_73.i.us.us.us.us.us.i169 = and i32 %_74.i.us.us.us.us.us.i168, %_52.i30
  %_72.i.us.us.us.us.us.i170 = zext i32 %_73.i.us.us.us.us.us.i169 to i64
  %_80.i.us.us.us.us.us.i171 = icmp ugt i64 %_59.1.i22, %_64.i.us.us.us.us.us.i167
  %404 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_64.i.us.us.us.us.us.i167
  %405 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_64.i.us.us.us.us.us.i167
  %_86.i.us.us.us.us.us.i172 = icmp ugt i64 %_61.1.i27, %_72.i.us.us.us.us.us.i170
  %406 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_72.i.us.us.us.us.us.i170
  %407 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_72.i.us.us.us.us.us.i170
  br i1 %_80.i.us.us.us.us.us.i171, label %bb63.i.split.us.us.us.us.us.us.i174, label %bb63.i.split.i90

bb63.i.split.us.us.us.us.us.us.i174:              ; preds = %bb60.i.us.us.us.us.us.i156
  %_84.i.us.us.us.us.us.i175 = icmp ugt i64 %_61.1.i27, %_64.i.us.us.us.us.us.i167
  br i1 %_84.i.us.us.us.us.us.i175, label %bb24.i.us.lr.ph.us.us.us.us.us.i176, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i93, !dbg !4029

bb24.i.us.lr.ph.us.us.us.us.us.i176:              ; preds = %bb63.i.split.us.us.us.us.us.us.i174
  br i1 %_86.i.us.us.us.us.us.i172, label %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178, label %bb24.i.us.lr.ph.split.i96

bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178:     ; preds = %bb24.i.us.lr.ph.us.us.us.us.us.i176
  %_82.i.us.us.us.us.us.us.i177 = load float, ptr %405, align 4, !noalias !3868, !noundef !12
  %_87.i.us.us.us.us.us.us.us.i180 = load float, ptr %407, align 4, !noalias !3868, !noundef !12
  %_85.i.us.us.le827.us.us.us.us.us.i = load float, ptr %406, align 4, !noalias !3868, !noundef !12
  %_78.i.us.le.us.us.us.us.us.i181 = load float, ptr %404, align 4, !noalias !3868, !noundef !12
  %408 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.us.us.i181), !dbg !4038
  %409 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.us.us.i177), !dbg !4048
  %_3.i.i466.us.us.us.us.us.i = fcmp ule float %408, %409, !dbg !4051
  %_6.i.i468.us.us.us.us.us.i = bitcast float %408 to i32, !dbg !4056
  %_8.i.i470.us.us.us.us.us.i = bitcast float %409 to i32, !dbg !4059
  %_4.i.i473.us.us.us.us.us.i = select i1 %_3.i.i466.us.us.us.us.us.i, i32 %_8.i.i470.us.us.us.us.us.i, i32 %_6.i.i468.us.us.us.us.us.i, !dbg !4061
  %_4.i346.us.us.us.us.us.i = select i1 %_3.i190.i, i32 %_6.i.i468.us.us.us.us.us.i, i32 %_4.i.i473.us.us.us.us.us.i, !dbg !4062
  %_0.i239.us.us.us.us.us.i = fmul float %408, 5.000000e-01, !dbg !4064
  %_0.i238.us.us.us.us.us.i = fmul float %409, 5.000000e-01, !dbg !4066
  %_0.i218.us.us.us.us.us.i = fadd float %_0.i238.us.us.us.us.us.i, %_0.i239.us.us.us.us.us.i, !dbg !4068
  %_6.i334.us.us.us.us.us.i = bitcast float %_0.i218.us.us.us.us.us.i to i32, !dbg !4070
  %_4.i339.us.us.us.us.us.i = select i1 %_3.i188.i, i32 %_4.i346.us.us.us.us.us.i, i32 %_6.i334.us.us.us.us.us.i, !dbg !4073
  %_0.i340.us.us.us.us.us.i = bitcast i32 %_4.i339.us.us.us.us.us.i to float, !dbg !4074
  %_3.i.i458.us.us.us.us.us.i = fcmp ule float %_0.i340.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4076
  %_4.i.i464.us.us.us.us.us.i = select i1 %_3.i.i458.us.us.us.us.us.i, i32 841731191, i32 %_4.i339.us.us.us.us.us.i, !dbg !4079
  %_0.i.i465.us.us.us.us.us.i = bitcast i32 %_4.i.i464.us.us.us.us.us.i to float, !dbg !4081
  %_3.i.i418.us.us.us.us.us.i = fcmp ule float %_0.i.i465.us.us.us.us.us.i, 0x3810000000000000, !dbg !4083
  %_4.i.i424.us.us.us.us.us.i = select i1 %_3.i.i418.us.us.us.us.us.i, i32 8388608, i32 %_4.i.i464.us.us.us.us.us.i, !dbg !4089
  %_5.i284.us.us.us.us.us.i = and i32 %_4.i.i424.us.us.us.us.us.i, 8388607, !dbg !4091
  %_4.i285.us.us.us.us.us.i = or disjoint i32 %_5.i284.us.us.us.us.us.i, 1065353216, !dbg !4091
  %significand.i286.us.us.us.us.us.i = bitcast i32 %_4.i285.us.us.us.us.us.i to float, !dbg !4093
  %_0.i247.us.us.us.us.us.i182 = fadd float %significand.i286.us.us.us.us.us.i, -1.000000e+00, !dbg !4095
  %_0.i227.us.us.us.us.us.i = fmul float %_0.i247.us.us.us.us.us.i182, 0x3F9B17A960000000, !dbg !4097
  %410 = fsub float 0x3FBF9A8440000000, %_0.i227.us.us.us.us.us.i, !dbg !4099
  %_0.i227.us.us.us.us.us.1.i = fmul float %_0.i247.us.us.us.us.us.i182, %410, !dbg !4097
  %_0.i213.us.us.us.us.us.1.i = fadd float %_0.i227.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !4099
  %_0.i227.us.us.us.us.us.2.i = fmul float %_0.i247.us.us.us.us.us.i182, %_0.i213.us.us.us.us.us.1.i, !dbg !4097
  %_0.i213.us.us.us.us.us.2.i = fadd float %_0.i227.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !4099
  %_0.i227.us.us.us.us.us.3.i = fmul float %_0.i247.us.us.us.us.us.i182, %_0.i213.us.us.us.us.us.2.i, !dbg !4097
  %_0.i213.us.us.us.us.us.3.i = fadd float %_0.i227.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4099
  %_0.i227.us.us.us.us.us.4.i = fmul float %_0.i247.us.us.us.us.us.i182, %_0.i213.us.us.us.us.us.3.i, !dbg !4097
  %_0.i213.us.us.us.us.us.4.i = fadd float %_0.i227.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4099
  %_9.i287.us.us.us.us.us.i = lshr i32 %_4.i.i424.us.us.us.us.us.i, 23, !dbg !4101
  %_8.i288.us.us.us.us.us.i = or disjoint i32 %_9.i287.us.us.us.us.us.i, 1258291200, !dbg !4101
  %_7.i289.us.us.us.us.us.i = bitcast i32 %_8.i288.us.us.us.us.us.i to float, !dbg !4102
  %exponent.i290.us.us.us.us.us.i = fadd float %_7.i289.us.us.us.us.us.i, 0xC160000FE0000000, !dbg !4104
  %_0.i226.us.us.us.us.us.i = fmul float %_0.i247.us.us.us.us.us.i182, %_0.i213.us.us.us.us.us.4.i, !dbg !4105
  %_0.i212.us.us.us.us.us.i = fadd float %exponent.i290.us.us.us.us.us.i, %_0.i226.us.us.us.us.us.i, !dbg !4107
  %_0.i237.us.us.us.us.us.i = fmul float %_0.i212.us.us.us.us.us.i, 0x4018151820000000, !dbg !4109
  %_3.i.i533.us.us.us.us.us.inv.i = fcmp olt float %_0.i237.us.us.us.us.us.i, 2.400000e+01, !dbg !4111
  %_0.i.i540.us.us.us.us.us.i = select i1 %_3.i.i533.us.us.us.us.us.inv.i, float %_0.i237.us.us.us.us.us.i, float 2.400000e+01, !dbg !4111
  %_3.i.i450.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i540.us.us.us.us.us.i, -1.600000e+02, !dbg !4114
  %_0.i.i457.us.us.us.us.us.i = select i1 %_3.i.i450.us.us.us.us.us.inv.i, float %_0.i.i540.us.us.us.us.us.i, float -1.600000e+02, !dbg !4114
  %_55.i59.us.us.us.us.us.i = load float, ptr %359, align 4, !dbg !4117, !alias.scope !4119, !noalias !4122, !noundef !12
  %_3.i186.us.us.us.us.us.i = fcmp ule float %_55.i59.us.us.us.us.us.i, 0.000000e+00, !dbg !4124
  %_3.i172.us.us.us.us.us.i = fcmp oge float %_0.i.i457.us.us.us.us.us.i, %threshold.i32.i, !dbg !4126
  %_3.i170.us.us.us.us.us.i = fcmp oge float %_0.i.i457.us.us.us.us.us.i, %_0.i254.i, !dbg !4129
  %..i171.us.us.us.us.us.i = sext i1 %_3.i170.us.us.us.us.us.i to i32, !dbg !4132
  %_0.i408.us.us.us.us.us.i = sext i1 %_3.i172.us.us.us.us.us.i to i32, !dbg !4134
  %_0.i402.us.us.us.us.us.i = select i1 %_3.i186.us.us.us.us.us.i, i32 %_0.i408.us.us.us.us.us.i, i32 %..i171.us.us.us.us.us.i, !dbg !4134
  %_0.i414.us.us.us.us.us.i = xor i32 %..i171.us.us.us.us.us.i, -1, !dbg !4139
  %_3.i184.us.us.us.us.us.i = fcmp ogt float %_67.i691001.us.us.us.us.us.i, 0.000000e+00, !dbg !4142
  %_0.i407.us.us.us.us.us.i183 = select i1 %_3.i184.us.us.us.us.us.i, i32 %_0.i414.us.us.us.us.us.i, i32 0, !dbg !4144
  %_0.i406.us.us.us.us.us.i = select i1 %_3.i186.us.us.us.us.us.i, i32 0, i32 %_0.i407.us.us.us.us.us.i183, !dbg !4146
  %_0.i401.us.us.us.us.us.i = or i32 %_0.i406.us.us.us.us.us.i, %_0.i402.us.us.us.us.us.i, !dbg !4148
  %_5.i329.us.us.us.us.us.i = and i32 %_0.i401.us.us.us.us.us.i, 1065353216, !dbg !4151
  %_0.i333.us.us.us.us.us.i = bitcast i32 %_5.i329.us.us.us.us.us.i to float, !dbg !4153
  %_0.i253.us.us.us.us.us.i184 = fadd float %_67.i691001.us.us.us.us.us.i, -1.000000e+00, !dbg !4155
  %411 = trunc nsw i32 %_0.i406.us.us.us.us.us.i to i1, !dbg !4158
  %_4.i327.v.us.us.us.us.us.i = select i1 %411, float %_0.i253.us.us.us.us.us.i184, float %_67.i691001.us.us.us.us.us.i, !dbg !4158
  %412 = trunc nsw i32 %_0.i402.us.us.us.us.us.i to i1, !dbg !4160
  %_0.i321.us.us.us.us.us.i = select i1 %412, float %_71.i75568569.i, float %_4.i327.v.us.us.us.us.us.i, !dbg !4160
  store float %_0.i321.us.us.us.us.us.i, ptr %360, align 4, !dbg !4162, !alias.scope !4119, !noalias !4122
  store i32 %_5.i329.us.us.us.us.us.i, ptr %359, align 4, !dbg !4163, !alias.scope !4119, !noalias !4122
  %_0.i251.us.us.us.us.us.i185 = fsub float %_0.i.i457.us.us.us.us.us.i, %threshold.i32.i, !dbg !4164
  %_0.i236.us.us.us.us.us.i = fmul float %_0.i252.i, %_0.i251.us.us.us.us.us.i185, !dbg !4166
  %_3.i.i442.inv.us.us.us.us.us.i = fcmp ogt float %_0.i236.us.us.us.us.us.i, %362, !dbg !4168
  %_4.i.i448.v.us.us.us.us.us.i = select i1 %_3.i.i442.inv.us.us.us.us.us.i, float %_0.i236.us.us.us.us.us.i, float %362, !dbg !4168
  %_3.i.i525.us.us.us.us.us.i = fcmp olt float %_4.i.i448.v.us.us.us.us.us.i, 0.000000e+00, !dbg !4171
  %413 = fcmp ule float %_0.i333.us.us.us.us.us.i, 0.000000e+00, !dbg !4174
  %414 = select i1 %413, i1 %_3.i.i525.us.us.us.us.us.i, i1 false, !dbg !4177
  %_0.i314.us.us.us.us.us.i = select i1 %414, float %_4.i.i448.v.us.us.us.us.us.i, float 0.000000e+00, !dbg !4177
  %_3.i180.us.us.us.us.us.i = fcmp ule float %_0.i314.us.us.us.us.us.i, %_86.i881003.us.us.us.us.us.i, !dbg !4178
  %_4.i307.us.us.us.us.us.i = select i1 %_3.i180.us.us.us.us.us.i, i32 %_88.i91572.i, i32 %_87.i90571.i, !dbg !4181
  %_0.i308.us.us.us.us.us.i = bitcast i32 %_4.i307.us.us.us.us.us.i to float, !dbg !4183
  %_0.i250.us.us.us.us.us.i186 = fsub float %_0.i314.us.us.us.us.us.i, %_86.i881003.us.us.us.us.us.i, !dbg !4185
  %_4.i220.us.us.us.us.us.i = fmul float %_0.i250.us.us.us.us.us.i186, %_0.i308.us.us.us.us.us.i, !dbg !4188
  %_0.i221.us.us.us.us.us.i = fadd float %_86.i881003.us.us.us.us.us.i, %_4.i220.us.us.us.us.us.i, !dbg !4188
  %415 = tail call noundef float @llvm.fabs.f32(float %_0.i221.us.us.us.us.us.i), !dbg !4190
  %416 = fcmp uge float %415, 0x3BC79CA100000000, !dbg !4194
  %_0.i292.us.us.us.us.us.i187 = select i1 %416, float %_0.i221.us.us.us.us.us.i, float 0.000000e+00, !dbg !4196
  store float %_0.i292.us.us.us.us.us.i187, ptr %363, align 4, !dbg !4197, !alias.scope !4119, !noalias !4122
  %_0.i235.us.us.us.us.us.i = fmul float %_0.i292.us.us.us.us.us.i187, 0x3FC542A5A0000000, !dbg !4199
  %_3.i.i434.us.us.us.us.us.inv.i = fcmp ogt float %_0.i235.us.us.us.us.us.i, -1.260000e+02, !dbg !4203
  %_0.i.i441.us.us.us.us.us.i = select i1 %_3.i.i434.us.us.us.us.us.inv.i, float %_0.i235.us.us.us.us.us.i, float -1.260000e+02, !dbg !4203
  %_3.i.i517.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i441.us.us.us.us.us.i, 1.270000e+02, !dbg !4207
  %_0.i.i524.us.us.us.us.us.i = select i1 %_3.i.i517.us.us.us.us.us.inv.i, float %_0.i.i441.us.us.us.us.us.i, float 1.270000e+02, !dbg !4207
  %417 = tail call noundef float @llvm.floor.f32(float %_0.i.i524.us.us.us.us.us.i), !dbg !4210
  %_0.i249.us.us.us.us.us.i188 = fsub float %_0.i.i524.us.us.us.us.us.i, %417, !dbg !4214
  %418 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le827.us.us.us.us.us.i), !dbg !4216
  %419 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.us.us.i180), !dbg !4220
  %_3.i.i500.us.us.us.us.us.i = fcmp ule float %418, %419, !dbg !4222
  %_6.i.i502.us.us.us.us.us.i = bitcast float %418 to i32, !dbg !4225
  %_8.i.i504.us.us.us.us.us.i = bitcast float %419 to i32, !dbg !4228
  %_4.i.i507.us.us.us.us.us.i = select i1 %_3.i.i500.us.us.us.us.us.i, i32 %_8.i.i504.us.us.us.us.us.i, i32 %_6.i.i502.us.us.us.us.us.i, !dbg !4230
  %_4.i398.us.us.us.us.us.i = select i1 %_3.i204.i, i32 %_6.i.i502.us.us.us.us.us.i, i32 %_4.i.i507.us.us.us.us.us.i, !dbg !4231
  %_0.i245.us.us.us.us.us.i = fmul float %418, 5.000000e-01, !dbg !4233
  %_0.i244.us.us.us.us.us.i = fmul float %419, 5.000000e-01, !dbg !4235
  %_0.i219.us.us.us.us.us.i = fadd float %_0.i244.us.us.us.us.us.i, %_0.i245.us.us.us.us.us.i, !dbg !4237
  %_6.i386.us.us.us.us.us.i = bitcast float %_0.i219.us.us.us.us.us.i to i32, !dbg !4239
  %_4.i391.us.us.us.us.us.i = select i1 %_3.i202.i, i32 %_4.i398.us.us.us.us.us.i, i32 %_6.i386.us.us.us.us.us.i, !dbg !4242
  %_0.i392.us.us.us.us.us.i = bitcast i32 %_4.i391.us.us.us.us.us.i to float, !dbg !4243
  %_3.i.i492.us.us.us.us.us.i = fcmp ule float %_0.i392.us.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4245
  %_4.i.i498.us.us.us.us.us.i = select i1 %_3.i.i492.us.us.us.us.us.i, i32 841731191, i32 %_4.i391.us.us.us.us.us.i, !dbg !4248
  %_0.i.i499.us.us.us.us.us.i = bitcast i32 %_4.i.i498.us.us.us.us.us.i to float, !dbg !4250
  %_3.i.i.us.us.us.us.us.i189 = fcmp ule float %_0.i.i499.us.us.us.us.us.i, 0x3810000000000000, !dbg !4252
  %_4.i.i.us.us.us.us.us.i190 = select i1 %_3.i.i.us.us.us.us.us.i189, i32 8388608, i32 %_4.i.i498.us.us.us.us.us.i, !dbg !4257
  %_5.i280.us.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.us.i190, 8388607, !dbg !4259
  %_4.i281.us.us.us.us.us.i = or disjoint i32 %_5.i280.us.us.us.us.us.i, 1065353216, !dbg !4259
  %significand.i.us.us.us.us.us.i191 = bitcast i32 %_4.i281.us.us.us.us.us.i to float, !dbg !4261
  %_0.i246.us.us.us.us.us.i192 = fadd float %significand.i.us.us.us.us.us.i191, -1.000000e+00, !dbg !4263
  %_0.i225.us.us.us.us.us.i = fmul float %_0.i246.us.us.us.us.us.i192, 0x3F9B17A960000000, !dbg !4265
  %420 = fsub float 0x3FBF9A8440000000, %_0.i225.us.us.us.us.us.i, !dbg !4267
  %_0.i225.us.us.us.us.us.1.i = fmul float %_0.i246.us.us.us.us.us.i192, %420, !dbg !4265
  %_0.i211.us.us.us.us.us.1.i = fadd float %_0.i225.us.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !4267
  %_0.i225.us.us.us.us.us.2.i = fmul float %_0.i246.us.us.us.us.us.i192, %_0.i211.us.us.us.us.us.1.i, !dbg !4265
  %_0.i211.us.us.us.us.us.2.i = fadd float %_0.i225.us.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !4267
  %_0.i225.us.us.us.us.us.3.i = fmul float %_0.i246.us.us.us.us.us.i192, %_0.i211.us.us.us.us.us.2.i, !dbg !4265
  %_0.i211.us.us.us.us.us.3.i = fadd float %_0.i225.us.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4267
  %_0.i225.us.us.us.us.us.4.i = fmul float %_0.i246.us.us.us.us.us.i192, %_0.i211.us.us.us.us.us.3.i, !dbg !4265
  %_0.i211.us.us.us.us.us.4.i = fadd float %_0.i225.us.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4267
  %_9.i.us.us.us.us.us.i193 = lshr i32 %_4.i.i.us.us.us.us.us.i190, 23, !dbg !4269
  %_8.i282.us.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.us.i193, 1258291200, !dbg !4269
  %_7.i.us.us.us.us.us.i194 = bitcast i32 %_8.i282.us.us.us.us.us.i to float, !dbg !4270
  %exponent.i.us.us.us.us.us.i195 = fadd float %_7.i.us.us.us.us.us.i194, 0xC160000FE0000000, !dbg !4272
  %_0.i224.us.us.us.us.us.i = fmul float %_0.i246.us.us.us.us.us.i192, %_0.i211.us.us.us.us.us.4.i, !dbg !4273
  %_0.i210.us.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.us.i195, %_0.i224.us.us.us.us.us.i, !dbg !4275
  %_0.i243.us.us.us.us.us.i = fmul float %_0.i210.us.us.us.us.us.i, 0x4018151820000000, !dbg !4277
  %_3.i.i549.us.us.us.us.us.inv.i = fcmp olt float %_0.i243.us.us.us.us.us.i, 2.400000e+01, !dbg !4279
  %_0.i.i556.us.us.us.us.us.i = select i1 %_3.i.i549.us.us.us.us.us.inv.i, float %_0.i243.us.us.us.us.us.i, float 2.400000e+01, !dbg !4279
  %_3.i.i484.us.us.us.us.us.inv.i = fcmp ogt float %_0.i.i556.us.us.us.us.us.i, -1.600000e+02, !dbg !4282
  %_0.i.i491.us.us.us.us.us.i = select i1 %_3.i.i484.us.us.us.us.us.inv.i, float %_0.i.i556.us.us.us.us.us.i, float -1.600000e+02, !dbg !4282
  %_55.i11.us.us.us.us.us.i = load float, ptr %371, align 4, !dbg !4285, !alias.scope !4286, !noalias !4289, !noundef !12
  %_3.i200.us.us.us.us.us.i196 = fcmp ule float %_55.i11.us.us.us.us.us.i, 0.000000e+00, !dbg !4291
  %_3.i176.us.us.us.us.us.i = fcmp oge float %_0.i.i491.us.us.us.us.us.i, %threshold.i.i, !dbg !4293
  %_3.i174.us.us.us.us.us.i = fcmp oge float %_0.i.i491.us.us.us.us.us.i, %_0.i259.i, !dbg !4295
  %..i175.us.us.us.us.us.i = sext i1 %_3.i174.us.us.us.us.us.i to i32, !dbg !4297
  %_0.i412.us.us.us.us.us.i = sext i1 %_3.i176.us.us.us.us.us.i to i32, !dbg !4299
  %_0.i405.us.us.us.us.us.i = select i1 %_3.i200.us.us.us.us.us.i196, i32 %_0.i412.us.us.us.us.us.i, i32 %..i175.us.us.us.us.us.i, !dbg !4299
  %_0.i416.us.us.us.us.us.i = xor i32 %..i175.us.us.us.us.us.i, -1, !dbg !4301
  %_3.i198.us.us.us.us.us.i197 = fcmp ogt float %_67.i131005.us.us.us.us.us.i, 0.000000e+00, !dbg !4303
  %_0.i411.us.us.us.us.us.i = select i1 %_3.i198.us.us.us.us.us.i197, i32 %_0.i416.us.us.us.us.us.i, i32 0, !dbg !4305
  %_0.i410.us.us.us.us.us.i = select i1 %_3.i200.us.us.us.us.us.i196, i32 0, i32 %_0.i411.us.us.us.us.us.i, !dbg !4307
  %_0.i404.us.us.us.us.us.i = or i32 %_0.i410.us.us.us.us.us.i, %_0.i405.us.us.us.us.us.i, !dbg !4309
  %_5.i381.us.us.us.us.us.i = and i32 %_0.i404.us.us.us.us.us.i, 1065353216, !dbg !4311
  %_0.i385.us.us.us.us.us.i = bitcast i32 %_5.i381.us.us.us.us.us.i to float, !dbg !4313
  %_0.i258.us.us.us.us.us.i = fadd float %_67.i131005.us.us.us.us.us.i, -1.000000e+00, !dbg !4315
  %421 = trunc nsw i32 %_0.i410.us.us.us.us.us.i to i1, !dbg !4317
  %_4.i379.v.us.us.us.us.us.i = select i1 %421, float %_0.i258.us.us.us.us.us.i, float %_67.i131005.us.us.us.us.us.i, !dbg !4317
  %422 = trunc nsw i32 %_0.i405.us.us.us.us.us.i to i1, !dbg !4319
  %_0.i373.us.us.us.us.us.i198 = select i1 %422, float %_71.i585586.i, float %_4.i379.v.us.us.us.us.us.i, !dbg !4319
  store float %_0.i373.us.us.us.us.us.i198, ptr %372, align 4, !dbg !4321, !alias.scope !4286, !noalias !4289
  store i32 %_5.i381.us.us.us.us.us.i, ptr %371, align 4, !dbg !4322, !alias.scope !4286, !noalias !4289
  %_0.i256.us.us.us.us.us.i199 = fsub float %_0.i.i491.us.us.us.us.us.i, %threshold.i.i, !dbg !4323
  %_0.i242.us.us.us.us.us.i = fmul float %_0.i257.i, %_0.i256.us.us.us.us.us.i199, !dbg !4325
  %_3.i.i475.inv.us.us.us.us.us.i = fcmp ogt float %_0.i242.us.us.us.us.us.i, %374, !dbg !4327
  %_4.i.i482.v.us.us.us.us.us.i = select i1 %_3.i.i475.inv.us.us.us.us.us.i, float %_0.i242.us.us.us.us.us.i, float %374, !dbg !4327
  %_3.i.i541.us.us.us.us.us.i = fcmp olt float %_4.i.i482.v.us.us.us.us.us.i, 0.000000e+00, !dbg !4330
  %423 = fcmp ule float %_0.i385.us.us.us.us.us.i, 0.000000e+00, !dbg !4333
  %424 = select i1 %423, i1 %_3.i.i541.us.us.us.us.us.i, i1 false, !dbg !4335
  %_0.i366.us.us.us.us.us.i = select i1 %424, float %_4.i.i482.v.us.us.us.us.us.i, float 0.000000e+00, !dbg !4335
  %_3.i194.us.us.us.us.us.i = fcmp ule float %_0.i366.us.us.us.us.us.i, %_86.i211007.us.us.us.us.us.i, !dbg !4336
  %_4.i360.us.us.us.us.us.i = select i1 %_3.i194.us.us.us.us.us.i, i32 %_88.i24589.i, i32 %_87.i23588.i, !dbg !4338
  %_0.i.us.us.us.us.us.i200 = bitcast i32 %_4.i360.us.us.us.us.us.i to float, !dbg !4340
  %_0.i255.us.us.us.us.us.i201 = fsub float %_0.i366.us.us.us.us.us.i, %_86.i211007.us.us.us.us.us.i, !dbg !4342
  %_4.i222.us.us.us.us.us.i = fmul float %_0.i255.us.us.us.us.us.i201, %_0.i.us.us.us.us.us.i200, !dbg !4344
  %_0.i223.us.us.us.us.us.i = fadd float %_86.i211007.us.us.us.us.us.i, %_4.i222.us.us.us.us.us.i, !dbg !4344
  %425 = tail call noundef float @llvm.fabs.f32(float %_0.i223.us.us.us.us.us.i), !dbg !4346
  %426 = fcmp uge float %425, 0x3BC79CA100000000, !dbg !4349
  %_0.i296.us.us.us.us.us.i202 = select i1 %426, float %_0.i223.us.us.us.us.us.i, float 0.000000e+00, !dbg !4351
  store float %_0.i296.us.us.us.us.us.i202, ptr %375, align 4, !dbg !4352, !alias.scope !4286, !noalias !4289
  %_0.i241.us.us.us.us.us.i = fmul float %_0.i296.us.us.us.us.us.i202, 0x3FC542A5A0000000, !dbg !4353
  %_3.i.i426.us.us.us.us.us.inv.i = fcmp ogt float %_0.i241.us.us.us.us.us.i, -1.260000e+02, !dbg !4356
  %_0.i.i433.us.us.us.us.us.i = select i1 %_3.i.i426.us.us.us.us.us.inv.i, float %_0.i241.us.us.us.us.us.i, float -1.260000e+02, !dbg !4356
  %_3.i.i509.us.us.us.us.us.inv.i = fcmp olt float %_0.i.i433.us.us.us.us.us.i, 1.270000e+02, !dbg !4360
  %_0.i.i516.us.us.us.us.us.i = select i1 %_3.i.i509.us.us.us.us.us.inv.i, float %_0.i.i433.us.us.us.us.us.i, float 1.270000e+02, !dbg !4360
  %427 = tail call noundef float @llvm.floor.f32(float %_0.i.i516.us.us.us.us.us.i), !dbg !4363
  %_0.i248.us.us.us.us.us.i203 = fsub float %_0.i.i516.us.us.us.us.us.i, %427, !dbg !4367
  %_0.i230.us.us.us.us.us.i = fmul float %_0.i248.us.us.us.us.us.i203, 0x3F5E974FA0000000, !dbg !4369
  %_0.i215.us.us.us.us.us.i = fadd float %_0.i230.us.us.us.us.us.i, 0x3F82778560000000, !dbg !4371
  %_0.i230.us.us.us.us.us.1.i = fmul float %_0.i248.us.us.us.us.us.i203, %_0.i215.us.us.us.us.us.i, !dbg !4369
  %_0.i215.us.us.us.us.us.1.i = fadd float %_0.i230.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4371
  %_0.i230.us.us.us.us.us.2.i = fmul float %_0.i248.us.us.us.us.us.i203, %_0.i215.us.us.us.us.us.1.i, !dbg !4369
  %_0.i215.us.us.us.us.us.2.i = fadd float %_0.i230.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4371
  %_0.i230.us.us.us.us.us.3.i = fmul float %_0.i248.us.us.us.us.us.i203, %_0.i215.us.us.us.us.us.2.i, !dbg !4369
  %_0.i215.us.us.us.us.us.3.i = fadd float %_0.i230.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4371
  %_0.i233.us.us.us.us.us.i = fmul float %_0.i249.us.us.us.us.us.i188, 0x3F5E974FA0000000, !dbg !4373
  %_0.i217.us.us.us.us.us.i = fadd float %_0.i233.us.us.us.us.us.i, 0x3F82778560000000, !dbg !4375
  %_0.i233.us.us.us.us.us.1.i = fmul float %_0.i249.us.us.us.us.us.i188, %_0.i217.us.us.us.us.us.i, !dbg !4373
  %_0.i217.us.us.us.us.us.1.i = fadd float %_0.i233.us.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4375
  %_0.i233.us.us.us.us.us.2.i = fmul float %_0.i249.us.us.us.us.us.i188, %_0.i217.us.us.us.us.us.1.i, !dbg !4373
  %_0.i217.us.us.us.us.us.2.i = fadd float %_0.i233.us.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4375
  %_0.i233.us.us.us.us.us.3.i = fmul float %_0.i249.us.us.us.us.us.i188, %_0.i217.us.us.us.us.us.2.i, !dbg !4373
  %_0.i217.us.us.us.us.us.3.i = fadd float %_0.i233.us.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4375
  %_0.i232.us.us.us.us.us.i = fmul float %_0.i249.us.us.us.us.us.i188, %_0.i217.us.us.us.us.us.3.i, !dbg !4377
  %_0.i216.us.us.us.us.us.i = fadd float %_0.i232.us.us.us.us.us.i, 1.000000e+00, !dbg !4379
  %biased.i163.us.us.us.us.us.i = fadd float %417, 0x4160000FE0000000, !dbg !4381
  %_4.i164.us.us.us.us.us.i = bitcast float %biased.i163.us.us.us.us.us.i to i32, !dbg !4383
  %_3.i165.us.us.us.us.us.i = shl i32 %_4.i164.us.us.us.us.us.i, 23, !dbg !4385
  %_0.i166.us.us.us.us.us.i = bitcast i32 %_3.i165.us.us.us.us.us.i to float, !dbg !4386
  %_0.i231.us.us.us.us.us.i = fmul float %_0.i216.us.us.us.us.us.i, %_0.i166.us.us.us.us.us.i, !dbg !4388
  %_3.i167.us.us.us.us.us.i = fcmp une float %_0.i292.us.us.us.us.us.i187, 0.000000e+00, !dbg !4390
  %_0.i400576.not.us.us.us.us.us.i = and i1 %_3.i178.i, %_3.i167.us.us.us.us.us.i, !dbg !4393
  %_0.i234.us.us.us.us.us.i = fmul float %_0.i271.us.us.us.us.us.i158, %_0.i231.us.us.us.us.us.i, !dbg !4393
  %_4.i300.v.us.us.us.us.us.i = select i1 %_0.i400576.not.us.us.us.us.us.i, float %_0.i234.us.us.us.us.us.i, float %_0.i271.us.us.us.us.us.i158, !dbg !4396
  %_0.i229.us.us.us.us.us.i = fmul float %_0.i248.us.us.us.us.us.i203, %_0.i215.us.us.us.us.us.3.i, !dbg !4398
  %_0.i214.us.us.us.us.us.i = fadd float %_0.i229.us.us.us.us.us.i, 1.000000e+00, !dbg !4400
  %biased.i.us.us.us.us.us.i204 = fadd float %427, 0x4160000FE0000000, !dbg !4402
  %_4.i160.us.us.us.us.us.i = bitcast float %biased.i.us.us.us.us.us.i204 to i32, !dbg !4404
  %_3.i161.us.us.us.us.us.i = shl i32 %_4.i160.us.us.us.us.us.i, 23, !dbg !4406
  %_0.i162.us.us.us.us.us.i = bitcast i32 %_3.i161.us.us.us.us.us.i to float, !dbg !4407
  %_0.i228.us.us.us.us.us.i = fmul float %_0.i214.us.us.us.us.us.i, %_0.i162.us.us.us.us.us.i, !dbg !4409
  %_3.i168.us.us.us.us.us.i = fcmp une float %_0.i296.us.us.us.us.us.i202, 0.000000e+00, !dbg !4411
  %_0.i403593.not.us.us.us.us.us.i = and i1 %_3.i192.i, %_3.i168.us.us.us.us.us.i, !dbg !4413
  %_0.i240.us.us.us.us.us.i = fmul float %_0.i269.us.us.us.us.us.i164, %_0.i228.us.us.us.us.us.i, !dbg !4413
  %_4.i353.v.us.us.us.us.us.i = select i1 %_0.i403593.not.us.us.us.us.us.i, float %_0.i240.us.us.us.us.us.i, float %_0.i269.us.us.us.us.us.i164, !dbg !4415
  store float %_4.i300.v.us.us.us.us.us.i, ptr %_123.i.us.us.us.us.us.i134, align 4, !dbg !4417, !alias.scope !4420, !noalias !3914
  store float %_4.i353.v.us.us.us.us.us.i, ptr %_141.i.us.us.us.us.us.i139, align 4, !dbg !4423, !alias.scope !4425, !noalias !3936
  %exitcond2529.not.i = icmp eq i64 %403, %_26, !dbg !4428
  br i1 %exitcond2529.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb40.i.us.us.us.us.us.i128, !dbg !4431, !llvm.loop !4433

bb42.i.us.us.us.us.i205:                          ; preds = %bb40.i.lr.ph.split.us.split.us.split.us.split.us.i, %bb28.i.us.us.lr.ph.us.us.us.us.i252
  %_86.i211007.us.us.us.us.i = phi float [ %_0.i296.us.us.us.us.i275, %bb28.i.us.us.lr.ph.us.us.us.us.i252 ], [ %.promoted1006.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.i ]
  %_67.i131005.us.us.us.us.i = phi float [ %_0.i373.us.us.us.us.i271, %bb28.i.us.us.lr.ph.us.us.us.us.i252 ], [ %.promoted1004.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.i ]
  %_86.i881003.us.us.us.us.i = phi float [ %_0.i292.us.us.us.us.i260, %bb28.i.us.us.lr.ph.us.us.us.us.i252 ], [ %.promoted1002.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.i ]
  %_67.i691001.us.us.us.us.i = phi float [ %_0.i321.us.us.us.us.i, %bb28.i.us.us.lr.ph.us.us.us.us.i252 ], [ %.promoted1000.i, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.i ]
  %iter.sroa.0.0.i966.us.us.us.us.i = phi i64 [ %428, %bb28.i.us.us.lr.ph.us.us.us.us.i252 ], [ 0, %bb40.i.lr.ph.split.us.split.us.split.us.split.us.i ]
  %428 = add nuw nsw i64 %iter.sroa.0.0.i966.us.us.us.us.i, 1, !dbg !3871
  %_27.i.us.us.us.us.i206 = trunc i64 %iter.sroa.0.0.i966.us.us.us.us.i to i32, !dbg !3885
  %now.i.us.us.us.us.i207 = add i32 %base.i.i46, %_27.i.us.us.us.us.i206, !dbg !3888
  %_30.i.us.us.us.us.i208 = and i32 %now.i.us.us.us.us.i207, %_52.i30, !dbg !3891
  %_29.i.us.us.us.us.i209 = zext i32 %_30.i.us.us.us.us.i208 to i64, !dbg !3893
  %_123.i.us.us.us.us.i210 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i966.us.us.us.us.i, !dbg !3900
  %_124.not.not.i.us.us.us.us.i211 = icmp ugt i64 %_58.1.i20, %_29.i.us.us.us.us.i209, !dbg !3904
  br i1 %_124.not.not.i.us.us.us.us.i211, label %bb45.i.us.us.us.us.i212, label %bb46.i.i58, !dbg !3904, !prof !2704

bb45.i.us.us.us.us.i212:                          ; preds = %bb42.i.us.us.us.us.i205
  %_0.i279.us.us.us.us.i213 = load float, ptr %_123.i.us.us.us.us.i210, align 4, !dbg !3909, !alias.scope !3911, !noalias !3914, !noundef !12
  %_133.i.us.us.us.us.i214 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_29.i.us.us.us.us.i209, !dbg !3915
  store float %_0.i279.us.us.us.us.i213, ptr %_133.i.us.us.us.us.i214, align 4, !dbg !3919, !alias.scope !3921, !noalias !3868
  %_141.i.us.us.us.us.i215 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i966.us.us.us.us.i, !dbg !3924
  %_0.i277.us.us.us.us.i216 = load float, ptr %_141.i.us.us.us.us.i215, align 4, !dbg !3931, !alias.scope !3933, !noalias !3936, !noundef !12
  %_149.i.us.us.us.us.i217 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_29.i.us.us.us.us.i209, !dbg !3937
  store float %_0.i277.us.us.us.us.i216, ptr %_149.i.us.us.us.us.i217, align 4, !dbg !3944, !alias.scope !3946, !noalias !3868
  %_157.i.us.us.us.us.i218 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i44, i64 %iter.sroa.0.0.i966.us.us.us.us.i, !dbg !3949
  %_0.i275.us.us.us.us.i219 = load float, ptr %_157.i.us.us.us.us.i218, align 4, !dbg !3956, !alias.scope !3958, !noalias !3868, !noundef !12
  %_165.i.us.us.us.us.i220 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_29.i.us.us.us.us.i209, !dbg !3961
  store float %_0.i275.us.us.us.us.i219, ptr %_165.i.us.us.us.us.i220, align 4, !dbg !3968, !alias.scope !3970, !noalias !3868
  %exitcond2530.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.us.us.us.i, %empty.sroa.6.0.i.i42, !dbg !4434
  br i1 %exitcond2530.not.i, label %bb57.i.i278, label %bb56.i.us.us.us.us.i221, !dbg !4434, !prof !180

bb56.i.us.us.us.us.i221:                          ; preds = %bb45.i.us.us.us.us.i212
  %_174.not.not.i.us.us.us.us.i222 = icmp ugt i64 %_61.1.i27, %_29.i.us.us.us.us.i209, !dbg !4432
  br i1 %_174.not.not.i.us.us.us.us.i222, label %bb58.i.us.us.us.us.i223, label %bb59.i.i146, !dbg !4432, !prof !2704

bb58.i.us.us.us.us.i223:                          ; preds = %bb56.i.us.us.us.us.i221
  %_173.i.us.us.us.us.i224 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i43, i64 %iter.sroa.0.0.i966.us.us.us.us.i, !dbg !3973
  %_0.i273.us.us.us.us.i225 = load float, ptr %_173.i.us.us.us.us.i224, align 4, !dbg !3980, !alias.scope !3982, !noalias !3868, !noundef !12
  %_181.i.us.us.us.us.i226 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_29.i.us.us.us.us.i209, !dbg !3985
  store float %_0.i273.us.us.us.us.i225, ptr %_181.i.us.us.us.us.i226, align 4, !dbg !3992, !alias.scope !3994, !noalias !3868
  %_52.i.us.us.us.us.i227 = sub i32 %now.i.us.us.us.us.i207, %_53.i31, !dbg !3997
  %_51.i.us.us.us.us.i228 = and i32 %_52.i.us.us.us.us.i227, %_52.i30, !dbg !4000
  %_50.i.us.us.us.us.i229 = zext i32 %_51.i.us.us.us.us.i228 to i64, !dbg !4001
  %_182.not.not.i.us.us.us.us.i230 = icmp ugt i64 %_58.1.i20, %_50.i.us.us.us.us.i229, !dbg !4002
  br i1 %_182.not.not.i.us.us.us.us.i230, label %bb60.i.us.us.us.us.i231, label %bb61.i.i75, !dbg !4002, !prof !2704

bb60.i.us.us.us.us.i231:                          ; preds = %bb58.i.us.us.us.us.i223
  %_189.i.us.us.us.us.i232 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_50.i.us.us.us.us.i229, !dbg !4007
  %_0.i271.us.us.us.us.i233 = load float, ptr %_189.i.us.us.us.us.i232, align 4, !dbg !4011, !alias.scope !4013, !noalias !3868, !noundef !12
  %_195.i.us.us.us.us.i236 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_50.i.us.us.us.us.i229, !dbg !4016
  %_0.i269.us.us.us.us.i237 = load float, ptr %_195.i.us.us.us.us.i236, align 4, !dbg !4024, !alias.scope !4026, !noalias !3868, !noundef !12
  %_66.i.us.us.us.us.i238 = sub i32 %now.i.us.us.us.us.i207, %_67.i.i
  %_65.i.us.us.us.us.i239 = and i32 %_66.i.us.us.us.us.i238, %_52.i30
  %_64.i.us.us.us.us.i240 = zext i32 %_65.i.us.us.us.us.i239 to i64
  %_74.i.us.us.us.us.i241 = sub i32 %now.i.us.us.us.us.i207, %_75.i.i
  %_73.i.us.us.us.us.i242 = and i32 %_74.i.us.us.us.us.i241, %_52.i30
  %_72.i.us.us.us.us.i243 = zext i32 %_73.i.us.us.us.us.i242 to i64
  %_80.i.us.us.us.us.i244 = icmp ugt i64 %_59.1.i22, %_64.i.us.us.us.us.i240
  %429 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_64.i.us.us.us.us.i240
  %430 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_64.i.us.us.us.us.i240
  %_86.i.us.us.us.us.i245 = icmp ugt i64 %_61.1.i27, %_72.i.us.us.us.us.i243
  %431 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_72.i.us.us.us.us.i243
  %_88.i.us.us.us.us.i246 = icmp ugt i64 %_59.1.i22, %_72.i.us.us.us.us.i243
  %432 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_72.i.us.us.us.us.i243
  br i1 %_80.i.us.us.us.us.i244, label %bb63.i.split.us.us.us.us.us.i247, label %bb63.i.split.i90

bb63.i.split.us.us.us.us.us.i247:                 ; preds = %bb60.i.us.us.us.us.i231
  %_84.i.us.us.us.us.i248 = icmp ugt i64 %_61.1.i27, %_64.i.us.us.us.us.i240
  br i1 %_84.i.us.us.us.us.i248, label %bb24.i.us.lr.ph.us.us.us.us.i249, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i93, !dbg !4029

bb24.i.us.lr.ph.us.us.us.us.i249:                 ; preds = %bb63.i.split.us.us.us.us.us.i247
  %_82.i.us.us.us.us.us.i250 = load float, ptr %430, align 4, !noalias !3868, !noundef !12
  br i1 %_86.i.us.us.us.us.i245, label %bb24.i.us.lr.ph.split.us.us.us.us.us.i251, label %bb24.i.us.lr.ph.split.i96

bb24.i.us.lr.ph.split.us.us.us.us.us.i251:        ; preds = %bb24.i.us.lr.ph.us.us.us.us.i249
  br i1 %_88.i.us.us.us.us.i246, label %bb28.i.us.us.lr.ph.us.us.us.us.i252, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i98, !dbg !4037

bb28.i.us.us.lr.ph.us.us.us.us.i252:              ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.us.i251
  %_87.i.us.us.us.us.us.us.i253 = load float, ptr %432, align 4, !noalias !3868, !noundef !12
  %_85.i.us.us.le827.us.us.us.us.i = load float, ptr %431, align 4, !noalias !3868, !noundef !12
  %_78.i.us.le.us.us.us.us.i254 = load float, ptr %429, align 4, !noalias !3868, !noundef !12
  %433 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.us.i254), !dbg !4038
  %434 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.us.i250), !dbg !4048
  %_3.i.i466.us.us.us.us.i = fcmp ule float %433, %434, !dbg !4051
  %_6.i.i468.us.us.us.us.i = bitcast float %433 to i32, !dbg !4056
  %_8.i.i470.us.us.us.us.i = bitcast float %434 to i32, !dbg !4059
  %_4.i.i473.us.us.us.us.i = select i1 %_3.i.i466.us.us.us.us.i, i32 %_8.i.i470.us.us.us.us.i, i32 %_6.i.i468.us.us.us.us.i, !dbg !4061
  %_4.i346.us.us.us.us.i = select i1 %_3.i190.i, i32 %_6.i.i468.us.us.us.us.i, i32 %_4.i.i473.us.us.us.us.i, !dbg !4062
  %_0.i239.us.us.us.us.i = fmul float %433, 5.000000e-01, !dbg !4064
  %_0.i238.us.us.us.us.i = fmul float %434, 5.000000e-01, !dbg !4066
  %_0.i218.us.us.us.us.i = fadd float %_0.i238.us.us.us.us.i, %_0.i239.us.us.us.us.i, !dbg !4068
  %_6.i334.us.us.us.us.i = bitcast float %_0.i218.us.us.us.us.i to i32, !dbg !4070
  %_4.i339.us.us.us.us.i = select i1 %_3.i188.i, i32 %_4.i346.us.us.us.us.i, i32 %_6.i334.us.us.us.us.i, !dbg !4073
  %_0.i340.us.us.us.us.i = bitcast i32 %_4.i339.us.us.us.us.i to float, !dbg !4074
  %_3.i.i458.us.us.us.us.i = fcmp ule float %_0.i340.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4076
  %_4.i.i464.us.us.us.us.i = select i1 %_3.i.i458.us.us.us.us.i, i32 841731191, i32 %_4.i339.us.us.us.us.i, !dbg !4079
  %_0.i.i465.us.us.us.us.i = bitcast i32 %_4.i.i464.us.us.us.us.i to float, !dbg !4081
  %_3.i.i418.us.us.us.us.i = fcmp ule float %_0.i.i465.us.us.us.us.i, 0x3810000000000000, !dbg !4083
  %_4.i.i424.us.us.us.us.i = select i1 %_3.i.i418.us.us.us.us.i, i32 8388608, i32 %_4.i.i464.us.us.us.us.i, !dbg !4089
  %_5.i284.us.us.us.us.i = and i32 %_4.i.i424.us.us.us.us.i, 8388607, !dbg !4091
  %_4.i285.us.us.us.us.i = or disjoint i32 %_5.i284.us.us.us.us.i, 1065353216, !dbg !4091
  %significand.i286.us.us.us.us.i = bitcast i32 %_4.i285.us.us.us.us.i to float, !dbg !4093
  %_0.i247.us.us.us.us.i255 = fadd float %significand.i286.us.us.us.us.i, -1.000000e+00, !dbg !4095
  %_0.i227.us.us.us.us.i = fmul float %_0.i247.us.us.us.us.i255, 0x3F9B17A960000000, !dbg !4097
  %435 = fsub float 0x3FBF9A8440000000, %_0.i227.us.us.us.us.i, !dbg !4099
  %_0.i227.us.us.us.us.1.i = fmul float %_0.i247.us.us.us.us.i255, %435, !dbg !4097
  %_0.i213.us.us.us.us.1.i = fadd float %_0.i227.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !4099
  %_0.i227.us.us.us.us.2.i = fmul float %_0.i247.us.us.us.us.i255, %_0.i213.us.us.us.us.1.i, !dbg !4097
  %_0.i213.us.us.us.us.2.i = fadd float %_0.i227.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !4099
  %_0.i227.us.us.us.us.3.i = fmul float %_0.i247.us.us.us.us.i255, %_0.i213.us.us.us.us.2.i, !dbg !4097
  %_0.i213.us.us.us.us.3.i = fadd float %_0.i227.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4099
  %_0.i227.us.us.us.us.4.i = fmul float %_0.i247.us.us.us.us.i255, %_0.i213.us.us.us.us.3.i, !dbg !4097
  %_0.i213.us.us.us.us.4.i = fadd float %_0.i227.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4099
  %_9.i287.us.us.us.us.i = lshr i32 %_4.i.i424.us.us.us.us.i, 23, !dbg !4101
  %_8.i288.us.us.us.us.i = or disjoint i32 %_9.i287.us.us.us.us.i, 1258291200, !dbg !4101
  %_7.i289.us.us.us.us.i = bitcast i32 %_8.i288.us.us.us.us.i to float, !dbg !4102
  %exponent.i290.us.us.us.us.i = fadd float %_7.i289.us.us.us.us.i, 0xC160000FE0000000, !dbg !4104
  %_0.i226.us.us.us.us.i = fmul float %_0.i247.us.us.us.us.i255, %_0.i213.us.us.us.us.4.i, !dbg !4105
  %_0.i212.us.us.us.us.i = fadd float %exponent.i290.us.us.us.us.i, %_0.i226.us.us.us.us.i, !dbg !4107
  %_0.i237.us.us.us.us.i = fmul float %_0.i212.us.us.us.us.i, 0x4018151820000000, !dbg !4109
  %_3.i.i533.us.us.us.us.inv.i = fcmp olt float %_0.i237.us.us.us.us.i, 2.400000e+01, !dbg !4111
  %_0.i.i540.us.us.us.us.i = select i1 %_3.i.i533.us.us.us.us.inv.i, float %_0.i237.us.us.us.us.i, float 2.400000e+01, !dbg !4111
  %_3.i.i450.us.us.us.us.inv.i = fcmp ogt float %_0.i.i540.us.us.us.us.i, -1.600000e+02, !dbg !4114
  %_0.i.i457.us.us.us.us.i = select i1 %_3.i.i450.us.us.us.us.inv.i, float %_0.i.i540.us.us.us.us.i, float -1.600000e+02, !dbg !4114
  %_55.i59.us.us.us.us.i = load float, ptr %359, align 4, !dbg !4117, !alias.scope !4119, !noalias !4122, !noundef !12
  %_3.i186.us.us.us.us.i = fcmp ule float %_55.i59.us.us.us.us.i, 0.000000e+00, !dbg !4124
  %_3.i172.us.us.us.us.i = fcmp oge float %_0.i.i457.us.us.us.us.i, %threshold.i32.i, !dbg !4126
  %_3.i170.us.us.us.us.i = fcmp oge float %_0.i.i457.us.us.us.us.i, %_0.i254.i, !dbg !4129
  %..i171.us.us.us.us.i = sext i1 %_3.i170.us.us.us.us.i to i32, !dbg !4132
  %_0.i408.us.us.us.us.i = sext i1 %_3.i172.us.us.us.us.i to i32, !dbg !4134
  %_0.i402.us.us.us.us.i = select i1 %_3.i186.us.us.us.us.i, i32 %_0.i408.us.us.us.us.i, i32 %..i171.us.us.us.us.i, !dbg !4134
  %_0.i414.us.us.us.us.i = xor i32 %..i171.us.us.us.us.i, -1, !dbg !4139
  %_3.i184.us.us.us.us.i = fcmp ogt float %_67.i691001.us.us.us.us.i, 0.000000e+00, !dbg !4142
  %_0.i407.us.us.us.us.i256 = select i1 %_3.i184.us.us.us.us.i, i32 %_0.i414.us.us.us.us.i, i32 0, !dbg !4144
  %_0.i406.us.us.us.us.i = select i1 %_3.i186.us.us.us.us.i, i32 0, i32 %_0.i407.us.us.us.us.i256, !dbg !4146
  %_0.i401.us.us.us.us.i = or i32 %_0.i406.us.us.us.us.i, %_0.i402.us.us.us.us.i, !dbg !4148
  %_5.i329.us.us.us.us.i = and i32 %_0.i401.us.us.us.us.i, 1065353216, !dbg !4151
  %_0.i333.us.us.us.us.i = bitcast i32 %_5.i329.us.us.us.us.i to float, !dbg !4153
  %_0.i253.us.us.us.us.i257 = fadd float %_67.i691001.us.us.us.us.i, -1.000000e+00, !dbg !4155
  %436 = trunc nsw i32 %_0.i406.us.us.us.us.i to i1, !dbg !4158
  %_4.i327.v.us.us.us.us.i = select i1 %436, float %_0.i253.us.us.us.us.i257, float %_67.i691001.us.us.us.us.i, !dbg !4158
  %437 = trunc nsw i32 %_0.i402.us.us.us.us.i to i1, !dbg !4160
  %_0.i321.us.us.us.us.i = select i1 %437, float %_71.i75568569.i, float %_4.i327.v.us.us.us.us.i, !dbg !4160
  store float %_0.i321.us.us.us.us.i, ptr %360, align 4, !dbg !4162, !alias.scope !4119, !noalias !4122
  store i32 %_5.i329.us.us.us.us.i, ptr %359, align 4, !dbg !4163, !alias.scope !4119, !noalias !4122
  %_0.i251.us.us.us.us.i258 = fsub float %_0.i.i457.us.us.us.us.i, %threshold.i32.i, !dbg !4164
  %_0.i236.us.us.us.us.i = fmul float %_0.i252.i, %_0.i251.us.us.us.us.i258, !dbg !4166
  %_3.i.i442.inv.us.us.us.us.i = fcmp ogt float %_0.i236.us.us.us.us.i, %362, !dbg !4168
  %_4.i.i448.v.us.us.us.us.i = select i1 %_3.i.i442.inv.us.us.us.us.i, float %_0.i236.us.us.us.us.i, float %362, !dbg !4168
  %_3.i.i525.us.us.us.us.i = fcmp olt float %_4.i.i448.v.us.us.us.us.i, 0.000000e+00, !dbg !4171
  %438 = fcmp ule float %_0.i333.us.us.us.us.i, 0.000000e+00, !dbg !4174
  %439 = select i1 %438, i1 %_3.i.i525.us.us.us.us.i, i1 false, !dbg !4177
  %_0.i314.us.us.us.us.i = select i1 %439, float %_4.i.i448.v.us.us.us.us.i, float 0.000000e+00, !dbg !4177
  %_3.i180.us.us.us.us.i = fcmp ule float %_0.i314.us.us.us.us.i, %_86.i881003.us.us.us.us.i, !dbg !4178
  %_4.i307.us.us.us.us.i = select i1 %_3.i180.us.us.us.us.i, i32 %_88.i91572.i, i32 %_87.i90571.i, !dbg !4181
  %_0.i308.us.us.us.us.i = bitcast i32 %_4.i307.us.us.us.us.i to float, !dbg !4183
  %_0.i250.us.us.us.us.i259 = fsub float %_0.i314.us.us.us.us.i, %_86.i881003.us.us.us.us.i, !dbg !4185
  %_4.i220.us.us.us.us.i = fmul float %_0.i250.us.us.us.us.i259, %_0.i308.us.us.us.us.i, !dbg !4188
  %_0.i221.us.us.us.us.i = fadd float %_86.i881003.us.us.us.us.i, %_4.i220.us.us.us.us.i, !dbg !4188
  %440 = tail call noundef float @llvm.fabs.f32(float %_0.i221.us.us.us.us.i), !dbg !4190
  %441 = fcmp uge float %440, 0x3BC79CA100000000, !dbg !4194
  %_0.i292.us.us.us.us.i260 = select i1 %441, float %_0.i221.us.us.us.us.i, float 0.000000e+00, !dbg !4196
  store float %_0.i292.us.us.us.us.i260, ptr %363, align 4, !dbg !4197, !alias.scope !4119, !noalias !4122
  %_0.i235.us.us.us.us.i = fmul float %_0.i292.us.us.us.us.i260, 0x3FC542A5A0000000, !dbg !4199
  %_3.i.i434.us.us.us.us.inv.i = fcmp ogt float %_0.i235.us.us.us.us.i, -1.260000e+02, !dbg !4203
  %_0.i.i441.us.us.us.us.i = select i1 %_3.i.i434.us.us.us.us.inv.i, float %_0.i235.us.us.us.us.i, float -1.260000e+02, !dbg !4203
  %_3.i.i517.us.us.us.us.inv.i = fcmp olt float %_0.i.i441.us.us.us.us.i, 1.270000e+02, !dbg !4207
  %_0.i.i524.us.us.us.us.i = select i1 %_3.i.i517.us.us.us.us.inv.i, float %_0.i.i441.us.us.us.us.i, float 1.270000e+02, !dbg !4207
  %442 = tail call noundef float @llvm.floor.f32(float %_0.i.i524.us.us.us.us.i), !dbg !4210
  %_0.i249.us.us.us.us.i261 = fsub float %_0.i.i524.us.us.us.us.i, %442, !dbg !4214
  %443 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le827.us.us.us.us.i), !dbg !4216
  %444 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.us.i253), !dbg !4220
  %_3.i.i500.us.us.us.us.i = fcmp ule float %443, %444, !dbg !4222
  %_6.i.i502.us.us.us.us.i = bitcast float %443 to i32, !dbg !4225
  %_8.i.i504.us.us.us.us.i = bitcast float %444 to i32, !dbg !4228
  %_4.i.i507.us.us.us.us.i = select i1 %_3.i.i500.us.us.us.us.i, i32 %_8.i.i504.us.us.us.us.i, i32 %_6.i.i502.us.us.us.us.i, !dbg !4230
  %_4.i398.us.us.us.us.i = select i1 %_3.i204.i, i32 %_6.i.i502.us.us.us.us.i, i32 %_4.i.i507.us.us.us.us.i, !dbg !4231
  %_0.i245.us.us.us.us.i = fmul float %443, 5.000000e-01, !dbg !4233
  %_0.i244.us.us.us.us.i = fmul float %444, 5.000000e-01, !dbg !4235
  %_0.i219.us.us.us.us.i = fadd float %_0.i244.us.us.us.us.i, %_0.i245.us.us.us.us.i, !dbg !4237
  %_6.i386.us.us.us.us.i = bitcast float %_0.i219.us.us.us.us.i to i32, !dbg !4239
  %_4.i391.us.us.us.us.i = select i1 %_3.i202.i, i32 %_4.i398.us.us.us.us.i, i32 %_6.i386.us.us.us.us.i, !dbg !4242
  %_0.i392.us.us.us.us.i = bitcast i32 %_4.i391.us.us.us.us.i to float, !dbg !4243
  %_3.i.i492.us.us.us.us.i = fcmp ule float %_0.i392.us.us.us.us.i, 0x3E45798EE0000000, !dbg !4245
  %_4.i.i498.us.us.us.us.i = select i1 %_3.i.i492.us.us.us.us.i, i32 841731191, i32 %_4.i391.us.us.us.us.i, !dbg !4248
  %_0.i.i499.us.us.us.us.i = bitcast i32 %_4.i.i498.us.us.us.us.i to float, !dbg !4250
  %_3.i.i.us.us.us.us.i262 = fcmp ule float %_0.i.i499.us.us.us.us.i, 0x3810000000000000, !dbg !4252
  %_4.i.i.us.us.us.us.i263 = select i1 %_3.i.i.us.us.us.us.i262, i32 8388608, i32 %_4.i.i498.us.us.us.us.i, !dbg !4257
  %_5.i280.us.us.us.us.i = and i32 %_4.i.i.us.us.us.us.i263, 8388607, !dbg !4259
  %_4.i281.us.us.us.us.i = or disjoint i32 %_5.i280.us.us.us.us.i, 1065353216, !dbg !4259
  %significand.i.us.us.us.us.i264 = bitcast i32 %_4.i281.us.us.us.us.i to float, !dbg !4261
  %_0.i246.us.us.us.us.i265 = fadd float %significand.i.us.us.us.us.i264, -1.000000e+00, !dbg !4263
  %_0.i225.us.us.us.us.i = fmul float %_0.i246.us.us.us.us.i265, 0x3F9B17A960000000, !dbg !4265
  %445 = fsub float 0x3FBF9A8440000000, %_0.i225.us.us.us.us.i, !dbg !4267
  %_0.i225.us.us.us.us.1.i = fmul float %_0.i246.us.us.us.us.i265, %445, !dbg !4265
  %_0.i211.us.us.us.us.1.i = fadd float %_0.i225.us.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !4267
  %_0.i225.us.us.us.us.2.i = fmul float %_0.i246.us.us.us.us.i265, %_0.i211.us.us.us.us.1.i, !dbg !4265
  %_0.i211.us.us.us.us.2.i = fadd float %_0.i225.us.us.us.us.2.i, 0x3FDD544F20000000, !dbg !4267
  %_0.i225.us.us.us.us.3.i = fmul float %_0.i246.us.us.us.us.i265, %_0.i211.us.us.us.us.2.i, !dbg !4265
  %_0.i211.us.us.us.us.3.i = fadd float %_0.i225.us.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4267
  %_0.i225.us.us.us.us.4.i = fmul float %_0.i246.us.us.us.us.i265, %_0.i211.us.us.us.us.3.i, !dbg !4265
  %_0.i211.us.us.us.us.4.i = fadd float %_0.i225.us.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4267
  %_9.i.us.us.us.us.i266 = lshr i32 %_4.i.i.us.us.us.us.i263, 23, !dbg !4269
  %_8.i282.us.us.us.us.i = or disjoint i32 %_9.i.us.us.us.us.i266, 1258291200, !dbg !4269
  %_7.i.us.us.us.us.i267 = bitcast i32 %_8.i282.us.us.us.us.i to float, !dbg !4270
  %exponent.i.us.us.us.us.i268 = fadd float %_7.i.us.us.us.us.i267, 0xC160000FE0000000, !dbg !4272
  %_0.i224.us.us.us.us.i = fmul float %_0.i246.us.us.us.us.i265, %_0.i211.us.us.us.us.4.i, !dbg !4273
  %_0.i210.us.us.us.us.i = fadd float %exponent.i.us.us.us.us.i268, %_0.i224.us.us.us.us.i, !dbg !4275
  %_0.i243.us.us.us.us.i = fmul float %_0.i210.us.us.us.us.i, 0x4018151820000000, !dbg !4277
  %_3.i.i549.us.us.us.us.inv.i = fcmp olt float %_0.i243.us.us.us.us.i, 2.400000e+01, !dbg !4279
  %_0.i.i556.us.us.us.us.i = select i1 %_3.i.i549.us.us.us.us.inv.i, float %_0.i243.us.us.us.us.i, float 2.400000e+01, !dbg !4279
  %_3.i.i484.us.us.us.us.inv.i = fcmp ogt float %_0.i.i556.us.us.us.us.i, -1.600000e+02, !dbg !4282
  %_0.i.i491.us.us.us.us.i = select i1 %_3.i.i484.us.us.us.us.inv.i, float %_0.i.i556.us.us.us.us.i, float -1.600000e+02, !dbg !4282
  %_55.i11.us.us.us.us.i = load float, ptr %371, align 4, !dbg !4285, !alias.scope !4286, !noalias !4289, !noundef !12
  %_3.i200.us.us.us.us.i269 = fcmp ule float %_55.i11.us.us.us.us.i, 0.000000e+00, !dbg !4291
  %_3.i176.us.us.us.us.i = fcmp oge float %_0.i.i491.us.us.us.us.i, %threshold.i.i, !dbg !4293
  %_3.i174.us.us.us.us.i = fcmp oge float %_0.i.i491.us.us.us.us.i, %_0.i259.i, !dbg !4295
  %..i175.us.us.us.us.i = sext i1 %_3.i174.us.us.us.us.i to i32, !dbg !4297
  %_0.i412.us.us.us.us.i = sext i1 %_3.i176.us.us.us.us.i to i32, !dbg !4299
  %_0.i405.us.us.us.us.i = select i1 %_3.i200.us.us.us.us.i269, i32 %_0.i412.us.us.us.us.i, i32 %..i175.us.us.us.us.i, !dbg !4299
  %_0.i416.us.us.us.us.i = xor i32 %..i175.us.us.us.us.i, -1, !dbg !4301
  %_3.i198.us.us.us.us.i270 = fcmp ogt float %_67.i131005.us.us.us.us.i, 0.000000e+00, !dbg !4303
  %_0.i411.us.us.us.us.i = select i1 %_3.i198.us.us.us.us.i270, i32 %_0.i416.us.us.us.us.i, i32 0, !dbg !4305
  %_0.i410.us.us.us.us.i = select i1 %_3.i200.us.us.us.us.i269, i32 0, i32 %_0.i411.us.us.us.us.i, !dbg !4307
  %_0.i404.us.us.us.us.i = or i32 %_0.i410.us.us.us.us.i, %_0.i405.us.us.us.us.i, !dbg !4309
  %_5.i381.us.us.us.us.i = and i32 %_0.i404.us.us.us.us.i, 1065353216, !dbg !4311
  %_0.i385.us.us.us.us.i = bitcast i32 %_5.i381.us.us.us.us.i to float, !dbg !4313
  %_0.i258.us.us.us.us.i = fadd float %_67.i131005.us.us.us.us.i, -1.000000e+00, !dbg !4315
  %446 = trunc nsw i32 %_0.i410.us.us.us.us.i to i1, !dbg !4317
  %_4.i379.v.us.us.us.us.i = select i1 %446, float %_0.i258.us.us.us.us.i, float %_67.i131005.us.us.us.us.i, !dbg !4317
  %447 = trunc nsw i32 %_0.i405.us.us.us.us.i to i1, !dbg !4319
  %_0.i373.us.us.us.us.i271 = select i1 %447, float %_71.i585586.i, float %_4.i379.v.us.us.us.us.i, !dbg !4319
  store float %_0.i373.us.us.us.us.i271, ptr %372, align 4, !dbg !4321, !alias.scope !4286, !noalias !4289
  store i32 %_5.i381.us.us.us.us.i, ptr %371, align 4, !dbg !4322, !alias.scope !4286, !noalias !4289
  %_0.i256.us.us.us.us.i272 = fsub float %_0.i.i491.us.us.us.us.i, %threshold.i.i, !dbg !4323
  %_0.i242.us.us.us.us.i = fmul float %_0.i257.i, %_0.i256.us.us.us.us.i272, !dbg !4325
  %_3.i.i475.inv.us.us.us.us.i = fcmp ogt float %_0.i242.us.us.us.us.i, %374, !dbg !4327
  %_4.i.i482.v.us.us.us.us.i = select i1 %_3.i.i475.inv.us.us.us.us.i, float %_0.i242.us.us.us.us.i, float %374, !dbg !4327
  %_3.i.i541.us.us.us.us.i = fcmp olt float %_4.i.i482.v.us.us.us.us.i, 0.000000e+00, !dbg !4330
  %448 = fcmp ule float %_0.i385.us.us.us.us.i, 0.000000e+00, !dbg !4333
  %449 = select i1 %448, i1 %_3.i.i541.us.us.us.us.i, i1 false, !dbg !4335
  %_0.i366.us.us.us.us.i = select i1 %449, float %_4.i.i482.v.us.us.us.us.i, float 0.000000e+00, !dbg !4335
  %_3.i194.us.us.us.us.i = fcmp ule float %_0.i366.us.us.us.us.i, %_86.i211007.us.us.us.us.i, !dbg !4336
  %_4.i360.us.us.us.us.i = select i1 %_3.i194.us.us.us.us.i, i32 %_88.i24589.i, i32 %_87.i23588.i, !dbg !4338
  %_0.i.us.us.us.us.i273 = bitcast i32 %_4.i360.us.us.us.us.i to float, !dbg !4340
  %_0.i255.us.us.us.us.i274 = fsub float %_0.i366.us.us.us.us.i, %_86.i211007.us.us.us.us.i, !dbg !4342
  %_4.i222.us.us.us.us.i = fmul float %_0.i255.us.us.us.us.i274, %_0.i.us.us.us.us.i273, !dbg !4344
  %_0.i223.us.us.us.us.i = fadd float %_86.i211007.us.us.us.us.i, %_4.i222.us.us.us.us.i, !dbg !4344
  %450 = tail call noundef float @llvm.fabs.f32(float %_0.i223.us.us.us.us.i), !dbg !4346
  %451 = fcmp uge float %450, 0x3BC79CA100000000, !dbg !4349
  %_0.i296.us.us.us.us.i275 = select i1 %451, float %_0.i223.us.us.us.us.i, float 0.000000e+00, !dbg !4351
  store float %_0.i296.us.us.us.us.i275, ptr %375, align 4, !dbg !4352, !alias.scope !4286, !noalias !4289
  %_0.i241.us.us.us.us.i = fmul float %_0.i296.us.us.us.us.i275, 0x3FC542A5A0000000, !dbg !4353
  %_3.i.i426.us.us.us.us.inv.i = fcmp ogt float %_0.i241.us.us.us.us.i, -1.260000e+02, !dbg !4356
  %_0.i.i433.us.us.us.us.i = select i1 %_3.i.i426.us.us.us.us.inv.i, float %_0.i241.us.us.us.us.i, float -1.260000e+02, !dbg !4356
  %_3.i.i509.us.us.us.us.inv.i = fcmp olt float %_0.i.i433.us.us.us.us.i, 1.270000e+02, !dbg !4360
  %_0.i.i516.us.us.us.us.i = select i1 %_3.i.i509.us.us.us.us.inv.i, float %_0.i.i433.us.us.us.us.i, float 1.270000e+02, !dbg !4360
  %452 = tail call noundef float @llvm.floor.f32(float %_0.i.i516.us.us.us.us.i), !dbg !4363
  %_0.i248.us.us.us.us.i276 = fsub float %_0.i.i516.us.us.us.us.i, %452, !dbg !4367
  %_0.i230.us.us.us.us.i = fmul float %_0.i248.us.us.us.us.i276, 0x3F5E974FA0000000, !dbg !4369
  %_0.i215.us.us.us.us.i = fadd float %_0.i230.us.us.us.us.i, 0x3F82778560000000, !dbg !4371
  %_0.i230.us.us.us.us.1.i = fmul float %_0.i248.us.us.us.us.i276, %_0.i215.us.us.us.us.i, !dbg !4369
  %_0.i215.us.us.us.us.1.i = fadd float %_0.i230.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4371
  %_0.i230.us.us.us.us.2.i = fmul float %_0.i248.us.us.us.us.i276, %_0.i215.us.us.us.us.1.i, !dbg !4369
  %_0.i215.us.us.us.us.2.i = fadd float %_0.i230.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4371
  %_0.i230.us.us.us.us.3.i = fmul float %_0.i248.us.us.us.us.i276, %_0.i215.us.us.us.us.2.i, !dbg !4369
  %_0.i215.us.us.us.us.3.i = fadd float %_0.i230.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4371
  %_0.i233.us.us.us.us.i = fmul float %_0.i249.us.us.us.us.i261, 0x3F5E974FA0000000, !dbg !4373
  %_0.i217.us.us.us.us.i = fadd float %_0.i233.us.us.us.us.i, 0x3F82778560000000, !dbg !4375
  %_0.i233.us.us.us.us.1.i = fmul float %_0.i249.us.us.us.us.i261, %_0.i217.us.us.us.us.i, !dbg !4373
  %_0.i217.us.us.us.us.1.i = fadd float %_0.i233.us.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4375
  %_0.i233.us.us.us.us.2.i = fmul float %_0.i249.us.us.us.us.i261, %_0.i217.us.us.us.us.1.i, !dbg !4373
  %_0.i217.us.us.us.us.2.i = fadd float %_0.i233.us.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4375
  %_0.i233.us.us.us.us.3.i = fmul float %_0.i249.us.us.us.us.i261, %_0.i217.us.us.us.us.2.i, !dbg !4373
  %_0.i217.us.us.us.us.3.i = fadd float %_0.i233.us.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4375
  %_0.i232.us.us.us.us.i = fmul float %_0.i249.us.us.us.us.i261, %_0.i217.us.us.us.us.3.i, !dbg !4377
  %_0.i216.us.us.us.us.i = fadd float %_0.i232.us.us.us.us.i, 1.000000e+00, !dbg !4379
  %biased.i163.us.us.us.us.i = fadd float %442, 0x4160000FE0000000, !dbg !4381
  %_4.i164.us.us.us.us.i = bitcast float %biased.i163.us.us.us.us.i to i32, !dbg !4383
  %_3.i165.us.us.us.us.i = shl i32 %_4.i164.us.us.us.us.i, 23, !dbg !4385
  %_0.i166.us.us.us.us.i = bitcast i32 %_3.i165.us.us.us.us.i to float, !dbg !4386
  %_0.i231.us.us.us.us.i = fmul float %_0.i216.us.us.us.us.i, %_0.i166.us.us.us.us.i, !dbg !4388
  %_3.i167.us.us.us.us.i = fcmp une float %_0.i292.us.us.us.us.i260, 0.000000e+00, !dbg !4390
  %_0.i400576.not.us.us.us.us.i = and i1 %_3.i178.i, %_3.i167.us.us.us.us.i, !dbg !4393
  %_0.i234.us.us.us.us.i = fmul float %_0.i271.us.us.us.us.i233, %_0.i231.us.us.us.us.i, !dbg !4393
  %_4.i300.v.us.us.us.us.i = select i1 %_0.i400576.not.us.us.us.us.i, float %_0.i234.us.us.us.us.i, float %_0.i271.us.us.us.us.i233, !dbg !4396
  %_0.i229.us.us.us.us.i = fmul float %_0.i248.us.us.us.us.i276, %_0.i215.us.us.us.us.3.i, !dbg !4398
  %_0.i214.us.us.us.us.i = fadd float %_0.i229.us.us.us.us.i, 1.000000e+00, !dbg !4400
  %biased.i.us.us.us.us.i277 = fadd float %452, 0x4160000FE0000000, !dbg !4402
  %_4.i160.us.us.us.us.i = bitcast float %biased.i.us.us.us.us.i277 to i32, !dbg !4404
  %_3.i161.us.us.us.us.i = shl i32 %_4.i160.us.us.us.us.i, 23, !dbg !4406
  %_0.i162.us.us.us.us.i = bitcast i32 %_3.i161.us.us.us.us.i to float, !dbg !4407
  %_0.i228.us.us.us.us.i = fmul float %_0.i214.us.us.us.us.i, %_0.i162.us.us.us.us.i, !dbg !4409
  %_3.i168.us.us.us.us.i = fcmp une float %_0.i296.us.us.us.us.i275, 0.000000e+00, !dbg !4411
  %_0.i403593.not.us.us.us.us.i = and i1 %_3.i192.i, %_3.i168.us.us.us.us.i, !dbg !4413
  %_0.i240.us.us.us.us.i = fmul float %_0.i269.us.us.us.us.i237, %_0.i228.us.us.us.us.i, !dbg !4413
  %_4.i353.v.us.us.us.us.i = select i1 %_0.i403593.not.us.us.us.us.i, float %_0.i240.us.us.us.us.i, float %_0.i269.us.us.us.us.i237, !dbg !4415
  store float %_4.i300.v.us.us.us.us.i, ptr %_123.i.us.us.us.us.i210, align 4, !dbg !4417, !alias.scope !4420, !noalias !3914
  store float %_4.i353.v.us.us.us.us.i, ptr %_141.i.us.us.us.us.i215, align 4, !dbg !4423, !alias.scope !4425, !noalias !3936
  %exitcond2531.not.i = icmp eq i64 %428, %_26, !dbg !4428
  br i1 %exitcond2531.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb42.i.us.us.us.us.i205, !dbg !4431, !llvm.loop !4435

bb40.i.us.us.us.i279:                             ; preds = %bb40.i.lr.ph.split.us.split.us.split.us.i, %bb28.i.us.us.lr.ph.us.us.us.i330
  %_86.i211007.us.us.us.i = phi float [ %_0.i296.us.us.us.i353, %bb28.i.us.us.lr.ph.us.us.us.i330 ], [ %.promoted1006.i, %bb40.i.lr.ph.split.us.split.us.split.us.i ]
  %_67.i131005.us.us.us.i = phi float [ %_0.i373.us.us.us.i349, %bb28.i.us.us.lr.ph.us.us.us.i330 ], [ %.promoted1004.i, %bb40.i.lr.ph.split.us.split.us.split.us.i ]
  %_86.i881003.us.us.us.i = phi float [ %_0.i292.us.us.us.i338, %bb28.i.us.us.lr.ph.us.us.us.i330 ], [ %.promoted1002.i, %bb40.i.lr.ph.split.us.split.us.split.us.i ]
  %_67.i691001.us.us.us.i = phi float [ %_0.i321.us.us.us.i, %bb28.i.us.us.lr.ph.us.us.us.i330 ], [ %.promoted1000.i, %bb40.i.lr.ph.split.us.split.us.split.us.i ]
  %iter.sroa.0.0.i966.us.us.us.i = phi i64 [ %453, %bb28.i.us.us.lr.ph.us.us.us.i330 ], [ 0, %bb40.i.lr.ph.split.us.split.us.split.us.i ]
  %453 = add nuw nsw i64 %iter.sroa.0.0.i966.us.us.us.i, 1, !dbg !3871
  %_27.i.us.us.us.i280 = trunc i64 %iter.sroa.0.0.i966.us.us.us.i to i32, !dbg !3885
  %now.i.us.us.us.i281 = add i32 %base.i.i46, %_27.i.us.us.us.i280, !dbg !3888
  %_30.i.us.us.us.i282 = and i32 %now.i.us.us.us.i281, %_52.i30, !dbg !3891
  %_29.i.us.us.us.i283 = zext i32 %_30.i.us.us.us.i282 to i64, !dbg !3893
  %exitcond2532.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.us.us.i, %_55, !dbg !3894
  br i1 %exitcond2532.not.i, label %bb43.i.i127, label %bb42.i.us.us.us.i284, !dbg !3894, !prof !180

bb42.i.us.us.us.i284:                             ; preds = %bb40.i.us.us.us.i279
  %_123.i.us.us.us.i285 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i966.us.us.us.i, !dbg !3900
  %_124.not.not.i.us.us.us.i286 = icmp ugt i64 %_58.1.i20, %_29.i.us.us.us.i283, !dbg !3904
  br i1 %_124.not.not.i.us.us.us.i286, label %bb45.i.us.us.us.i287, label %bb46.i.i58, !dbg !3904, !prof !2704

bb45.i.us.us.us.i287:                             ; preds = %bb42.i.us.us.us.i284
  %_0.i279.us.us.us.i288 = load float, ptr %_123.i.us.us.us.i285, align 4, !dbg !3909, !alias.scope !3911, !noalias !3914, !noundef !12
  %_133.i.us.us.us.i289 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_29.i.us.us.us.i283, !dbg !3915
  store float %_0.i279.us.us.us.i288, ptr %_133.i.us.us.us.i289, align 4, !dbg !3919, !alias.scope !3921, !noalias !3868
  %_141.i.us.us.us.i290 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i966.us.us.us.i, !dbg !3924
  %_0.i277.us.us.us.i291 = load float, ptr %_141.i.us.us.us.i290, align 4, !dbg !3931, !alias.scope !3933, !noalias !3936, !noundef !12
  %_149.i.us.us.us.i292 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_29.i.us.us.us.i283, !dbg !3937
  store float %_0.i277.us.us.us.i291, ptr %_149.i.us.us.us.i292, align 4, !dbg !3944, !alias.scope !3946, !noalias !3868
  %_158.not.not.i.us.us.us.i293 = icmp ugt i64 %_59.1.i22, %_29.i.us.us.us.i283, !dbg !4436
  br i1 %_158.not.not.i.us.us.us.i293, label %bb54.i.us.us.us.i295, label %bb55.i.i294, !dbg !4436, !prof !2704

bb54.i.us.us.us.i295:                             ; preds = %bb45.i.us.us.us.i287
  %_157.i.us.us.us.i296 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i44, i64 %iter.sroa.0.0.i966.us.us.us.i, !dbg !3949
  %_0.i275.us.us.us.i297 = load float, ptr %_157.i.us.us.us.i296, align 4, !dbg !3956, !alias.scope !3958, !noalias !3868, !noundef !12
  %_165.i.us.us.us.i298 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_29.i.us.us.us.i283, !dbg !3961
  store float %_0.i275.us.us.us.i297, ptr %_165.i.us.us.us.i298, align 4, !dbg !3968, !alias.scope !3970, !noalias !3868
  %exitcond2533.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.us.us.i, %empty.sroa.6.0.i.i42, !dbg !4434
  br i1 %exitcond2533.not.i, label %bb57.i.i278, label %bb56.i.us.us.us.i299, !dbg !4434, !prof !180

bb56.i.us.us.us.i299:                             ; preds = %bb54.i.us.us.us.i295
  %_174.not.not.i.us.us.us.i300 = icmp ugt i64 %_61.1.i27, %_29.i.us.us.us.i283, !dbg !4432
  br i1 %_174.not.not.i.us.us.us.i300, label %bb58.i.us.us.us.i301, label %bb59.i.i146, !dbg !4432, !prof !2704

bb58.i.us.us.us.i301:                             ; preds = %bb56.i.us.us.us.i299
  %_173.i.us.us.us.i302 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i43, i64 %iter.sroa.0.0.i966.us.us.us.i, !dbg !3973
  %_0.i273.us.us.us.i303 = load float, ptr %_173.i.us.us.us.i302, align 4, !dbg !3980, !alias.scope !3982, !noalias !3868, !noundef !12
  %_181.i.us.us.us.i304 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_29.i.us.us.us.i283, !dbg !3985
  store float %_0.i273.us.us.us.i303, ptr %_181.i.us.us.us.i304, align 4, !dbg !3992, !alias.scope !3994, !noalias !3868
  %_52.i.us.us.us.i305 = sub i32 %now.i.us.us.us.i281, %_53.i31, !dbg !3997
  %_51.i.us.us.us.i306 = and i32 %_52.i.us.us.us.i305, %_52.i30, !dbg !4000
  %_50.i.us.us.us.i307 = zext i32 %_51.i.us.us.us.i306 to i64, !dbg !4001
  %_182.not.not.i.us.us.us.i308 = icmp ugt i64 %_58.1.i20, %_50.i.us.us.us.i307, !dbg !4002
  br i1 %_182.not.not.i.us.us.us.i308, label %bb60.i.us.us.us.i309, label %bb61.i.i75, !dbg !4002, !prof !2704

bb60.i.us.us.us.i309:                             ; preds = %bb58.i.us.us.us.i301
  %_189.i.us.us.us.i310 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_50.i.us.us.us.i307, !dbg !4007
  %_0.i271.us.us.us.i311 = load float, ptr %_189.i.us.us.us.i310, align 4, !dbg !4011, !alias.scope !4013, !noalias !3868, !noundef !12
  %_195.i.us.us.us.i314 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_50.i.us.us.us.i307, !dbg !4016
  %_0.i269.us.us.us.i315 = load float, ptr %_195.i.us.us.us.i314, align 4, !dbg !4024, !alias.scope !4026, !noalias !3868, !noundef !12
  %_66.i.us.us.us.i316 = sub i32 %now.i.us.us.us.i281, %_67.i.i
  %_65.i.us.us.us.i317 = and i32 %_66.i.us.us.us.i316, %_52.i30
  %_64.i.us.us.us.i318 = zext i32 %_65.i.us.us.us.i317 to i64
  %_74.i.us.us.us.i319 = sub i32 %now.i.us.us.us.i281, %_75.i.i
  %_73.i.us.us.us.i320 = and i32 %_74.i.us.us.us.i319, %_52.i30
  %_72.i.us.us.us.i321 = zext i32 %_73.i.us.us.us.i320 to i64
  %_80.i.us.us.us.i322 = icmp ugt i64 %_59.1.i22, %_64.i.us.us.us.i318
  %454 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_64.i.us.us.us.i318
  %455 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_64.i.us.us.us.i318
  %_86.i.us.us.us.i323 = icmp ugt i64 %_61.1.i27, %_72.i.us.us.us.i321
  %456 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_72.i.us.us.us.i321
  %_88.i.us.us.us.i324 = icmp ugt i64 %_59.1.i22, %_72.i.us.us.us.i321
  %457 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_72.i.us.us.us.i321
  br i1 %_80.i.us.us.us.i322, label %bb63.i.split.us.us.us.us.i325, label %bb63.i.split.i90

bb63.i.split.us.us.us.us.i325:                    ; preds = %bb60.i.us.us.us.i309
  %_84.i.us.us.us.i326 = icmp ugt i64 %_61.1.i27, %_64.i.us.us.us.i318
  br i1 %_84.i.us.us.us.i326, label %bb24.i.us.lr.ph.us.us.us.i327, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i93, !dbg !4029

bb24.i.us.lr.ph.us.us.us.i327:                    ; preds = %bb63.i.split.us.us.us.us.i325
  %_82.i.us.us.us.us.i328 = load float, ptr %455, align 4, !noalias !3868, !noundef !12
  br i1 %_86.i.us.us.us.i323, label %bb24.i.us.lr.ph.split.us.us.us.us.i329, label %bb24.i.us.lr.ph.split.i96

bb24.i.us.lr.ph.split.us.us.us.us.i329:           ; preds = %bb24.i.us.lr.ph.us.us.us.i327
  br i1 %_88.i.us.us.us.i324, label %bb28.i.us.us.lr.ph.us.us.us.i330, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i98, !dbg !4037

bb28.i.us.us.lr.ph.us.us.us.i330:                 ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.i329
  %_87.i.us.us.us.us.us.i331 = load float, ptr %457, align 4, !noalias !3868, !noundef !12
  %_85.i.us.us.le827.us.us.us.i = load float, ptr %456, align 4, !noalias !3868, !noundef !12
  %_78.i.us.le.us.us.us.i332 = load float, ptr %454, align 4, !noalias !3868, !noundef !12
  %458 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.i332), !dbg !4038
  %459 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.i328), !dbg !4048
  %_3.i.i466.us.us.us.i = fcmp ule float %458, %459, !dbg !4051
  %_6.i.i468.us.us.us.i = bitcast float %458 to i32, !dbg !4056
  %_8.i.i470.us.us.us.i = bitcast float %459 to i32, !dbg !4059
  %_4.i.i473.us.us.us.i = select i1 %_3.i.i466.us.us.us.i, i32 %_8.i.i470.us.us.us.i, i32 %_6.i.i468.us.us.us.i, !dbg !4061
  %_4.i346.us.us.us.i = select i1 %_3.i190.i, i32 %_6.i.i468.us.us.us.i, i32 %_4.i.i473.us.us.us.i, !dbg !4062
  %_0.i239.us.us.us.i = fmul float %458, 5.000000e-01, !dbg !4064
  %_0.i238.us.us.us.i = fmul float %459, 5.000000e-01, !dbg !4066
  %_0.i218.us.us.us.i = fadd float %_0.i238.us.us.us.i, %_0.i239.us.us.us.i, !dbg !4068
  %_6.i334.us.us.us.i = bitcast float %_0.i218.us.us.us.i to i32, !dbg !4070
  %_4.i339.us.us.us.i = select i1 %_3.i188.i, i32 %_4.i346.us.us.us.i, i32 %_6.i334.us.us.us.i, !dbg !4073
  %_0.i340.us.us.us.i = bitcast i32 %_4.i339.us.us.us.i to float, !dbg !4074
  %_3.i.i458.us.us.us.i = fcmp ule float %_0.i340.us.us.us.i, 0x3E45798EE0000000, !dbg !4076
  %_4.i.i464.us.us.us.i = select i1 %_3.i.i458.us.us.us.i, i32 841731191, i32 %_4.i339.us.us.us.i, !dbg !4079
  %_0.i.i465.us.us.us.i = bitcast i32 %_4.i.i464.us.us.us.i to float, !dbg !4081
  %_3.i.i418.us.us.us.i = fcmp ule float %_0.i.i465.us.us.us.i, 0x3810000000000000, !dbg !4083
  %_4.i.i424.us.us.us.i = select i1 %_3.i.i418.us.us.us.i, i32 8388608, i32 %_4.i.i464.us.us.us.i, !dbg !4089
  %_5.i284.us.us.us.i = and i32 %_4.i.i424.us.us.us.i, 8388607, !dbg !4091
  %_4.i285.us.us.us.i = or disjoint i32 %_5.i284.us.us.us.i, 1065353216, !dbg !4091
  %significand.i286.us.us.us.i = bitcast i32 %_4.i285.us.us.us.i to float, !dbg !4093
  %_0.i247.us.us.us.i333 = fadd float %significand.i286.us.us.us.i, -1.000000e+00, !dbg !4095
  %_0.i227.us.us.us.i = fmul float %_0.i247.us.us.us.i333, 0x3F9B17A960000000, !dbg !4097
  %460 = fsub float 0x3FBF9A8440000000, %_0.i227.us.us.us.i, !dbg !4099
  %_0.i227.us.us.us.1.i = fmul float %_0.i247.us.us.us.i333, %460, !dbg !4097
  %_0.i213.us.us.us.1.i = fadd float %_0.i227.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !4099
  %_0.i227.us.us.us.2.i = fmul float %_0.i247.us.us.us.i333, %_0.i213.us.us.us.1.i, !dbg !4097
  %_0.i213.us.us.us.2.i = fadd float %_0.i227.us.us.us.2.i, 0x3FDD544F20000000, !dbg !4099
  %_0.i227.us.us.us.3.i = fmul float %_0.i247.us.us.us.i333, %_0.i213.us.us.us.2.i, !dbg !4097
  %_0.i213.us.us.us.3.i = fadd float %_0.i227.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4099
  %_0.i227.us.us.us.4.i = fmul float %_0.i247.us.us.us.i333, %_0.i213.us.us.us.3.i, !dbg !4097
  %_0.i213.us.us.us.4.i = fadd float %_0.i227.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4099
  %_9.i287.us.us.us.i = lshr i32 %_4.i.i424.us.us.us.i, 23, !dbg !4101
  %_8.i288.us.us.us.i = or disjoint i32 %_9.i287.us.us.us.i, 1258291200, !dbg !4101
  %_7.i289.us.us.us.i = bitcast i32 %_8.i288.us.us.us.i to float, !dbg !4102
  %exponent.i290.us.us.us.i = fadd float %_7.i289.us.us.us.i, 0xC160000FE0000000, !dbg !4104
  %_0.i226.us.us.us.i = fmul float %_0.i247.us.us.us.i333, %_0.i213.us.us.us.4.i, !dbg !4105
  %_0.i212.us.us.us.i = fadd float %exponent.i290.us.us.us.i, %_0.i226.us.us.us.i, !dbg !4107
  %_0.i237.us.us.us.i = fmul float %_0.i212.us.us.us.i, 0x4018151820000000, !dbg !4109
  %_3.i.i533.us.us.us.inv.i = fcmp olt float %_0.i237.us.us.us.i, 2.400000e+01, !dbg !4111
  %_0.i.i540.us.us.us.i = select i1 %_3.i.i533.us.us.us.inv.i, float %_0.i237.us.us.us.i, float 2.400000e+01, !dbg !4111
  %_3.i.i450.us.us.us.inv.i = fcmp ogt float %_0.i.i540.us.us.us.i, -1.600000e+02, !dbg !4114
  %_0.i.i457.us.us.us.i = select i1 %_3.i.i450.us.us.us.inv.i, float %_0.i.i540.us.us.us.i, float -1.600000e+02, !dbg !4114
  %_55.i59.us.us.us.i = load float, ptr %359, align 4, !dbg !4117, !alias.scope !4119, !noalias !4122, !noundef !12
  %_3.i186.us.us.us.i = fcmp ule float %_55.i59.us.us.us.i, 0.000000e+00, !dbg !4124
  %_3.i172.us.us.us.i = fcmp oge float %_0.i.i457.us.us.us.i, %threshold.i32.i, !dbg !4126
  %_3.i170.us.us.us.i = fcmp oge float %_0.i.i457.us.us.us.i, %_0.i254.i, !dbg !4129
  %..i171.us.us.us.i = sext i1 %_3.i170.us.us.us.i to i32, !dbg !4132
  %_0.i408.us.us.us.i = sext i1 %_3.i172.us.us.us.i to i32, !dbg !4134
  %_0.i402.us.us.us.i = select i1 %_3.i186.us.us.us.i, i32 %_0.i408.us.us.us.i, i32 %..i171.us.us.us.i, !dbg !4134
  %_0.i414.us.us.us.i = xor i32 %..i171.us.us.us.i, -1, !dbg !4139
  %_3.i184.us.us.us.i = fcmp ogt float %_67.i691001.us.us.us.i, 0.000000e+00, !dbg !4142
  %_0.i407.us.us.us.i334 = select i1 %_3.i184.us.us.us.i, i32 %_0.i414.us.us.us.i, i32 0, !dbg !4144
  %_0.i406.us.us.us.i = select i1 %_3.i186.us.us.us.i, i32 0, i32 %_0.i407.us.us.us.i334, !dbg !4146
  %_0.i401.us.us.us.i = or i32 %_0.i406.us.us.us.i, %_0.i402.us.us.us.i, !dbg !4148
  %_5.i329.us.us.us.i = and i32 %_0.i401.us.us.us.i, 1065353216, !dbg !4151
  %_0.i333.us.us.us.i = bitcast i32 %_5.i329.us.us.us.i to float, !dbg !4153
  %_0.i253.us.us.us.i335 = fadd float %_67.i691001.us.us.us.i, -1.000000e+00, !dbg !4155
  %461 = trunc nsw i32 %_0.i406.us.us.us.i to i1, !dbg !4158
  %_4.i327.v.us.us.us.i = select i1 %461, float %_0.i253.us.us.us.i335, float %_67.i691001.us.us.us.i, !dbg !4158
  %462 = trunc nsw i32 %_0.i402.us.us.us.i to i1, !dbg !4160
  %_0.i321.us.us.us.i = select i1 %462, float %_71.i75568569.i, float %_4.i327.v.us.us.us.i, !dbg !4160
  store float %_0.i321.us.us.us.i, ptr %360, align 4, !dbg !4162, !alias.scope !4119, !noalias !4122
  store i32 %_5.i329.us.us.us.i, ptr %359, align 4, !dbg !4163, !alias.scope !4119, !noalias !4122
  %_0.i251.us.us.us.i336 = fsub float %_0.i.i457.us.us.us.i, %threshold.i32.i, !dbg !4164
  %_0.i236.us.us.us.i = fmul float %_0.i252.i, %_0.i251.us.us.us.i336, !dbg !4166
  %_3.i.i442.inv.us.us.us.i = fcmp ogt float %_0.i236.us.us.us.i, %362, !dbg !4168
  %_4.i.i448.v.us.us.us.i = select i1 %_3.i.i442.inv.us.us.us.i, float %_0.i236.us.us.us.i, float %362, !dbg !4168
  %_3.i.i525.us.us.us.i = fcmp olt float %_4.i.i448.v.us.us.us.i, 0.000000e+00, !dbg !4171
  %463 = fcmp ule float %_0.i333.us.us.us.i, 0.000000e+00, !dbg !4174
  %464 = select i1 %463, i1 %_3.i.i525.us.us.us.i, i1 false, !dbg !4177
  %_0.i314.us.us.us.i = select i1 %464, float %_4.i.i448.v.us.us.us.i, float 0.000000e+00, !dbg !4177
  %_3.i180.us.us.us.i = fcmp ule float %_0.i314.us.us.us.i, %_86.i881003.us.us.us.i, !dbg !4178
  %_4.i307.us.us.us.i = select i1 %_3.i180.us.us.us.i, i32 %_88.i91572.i, i32 %_87.i90571.i, !dbg !4181
  %_0.i308.us.us.us.i = bitcast i32 %_4.i307.us.us.us.i to float, !dbg !4183
  %_0.i250.us.us.us.i337 = fsub float %_0.i314.us.us.us.i, %_86.i881003.us.us.us.i, !dbg !4185
  %_4.i220.us.us.us.i = fmul float %_0.i250.us.us.us.i337, %_0.i308.us.us.us.i, !dbg !4188
  %_0.i221.us.us.us.i = fadd float %_86.i881003.us.us.us.i, %_4.i220.us.us.us.i, !dbg !4188
  %465 = tail call noundef float @llvm.fabs.f32(float %_0.i221.us.us.us.i), !dbg !4190
  %466 = fcmp uge float %465, 0x3BC79CA100000000, !dbg !4194
  %_0.i292.us.us.us.i338 = select i1 %466, float %_0.i221.us.us.us.i, float 0.000000e+00, !dbg !4196
  store float %_0.i292.us.us.us.i338, ptr %363, align 4, !dbg !4197, !alias.scope !4119, !noalias !4122
  %_0.i235.us.us.us.i = fmul float %_0.i292.us.us.us.i338, 0x3FC542A5A0000000, !dbg !4199
  %_3.i.i434.us.us.us.inv.i = fcmp ogt float %_0.i235.us.us.us.i, -1.260000e+02, !dbg !4203
  %_0.i.i441.us.us.us.i = select i1 %_3.i.i434.us.us.us.inv.i, float %_0.i235.us.us.us.i, float -1.260000e+02, !dbg !4203
  %_3.i.i517.us.us.us.inv.i = fcmp olt float %_0.i.i441.us.us.us.i, 1.270000e+02, !dbg !4207
  %_0.i.i524.us.us.us.i = select i1 %_3.i.i517.us.us.us.inv.i, float %_0.i.i441.us.us.us.i, float 1.270000e+02, !dbg !4207
  %467 = tail call noundef float @llvm.floor.f32(float %_0.i.i524.us.us.us.i), !dbg !4210
  %_0.i249.us.us.us.i339 = fsub float %_0.i.i524.us.us.us.i, %467, !dbg !4214
  %468 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le827.us.us.us.i), !dbg !4216
  %469 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.i331), !dbg !4220
  %_3.i.i500.us.us.us.i = fcmp ule float %468, %469, !dbg !4222
  %_6.i.i502.us.us.us.i = bitcast float %468 to i32, !dbg !4225
  %_8.i.i504.us.us.us.i = bitcast float %469 to i32, !dbg !4228
  %_4.i.i507.us.us.us.i = select i1 %_3.i.i500.us.us.us.i, i32 %_8.i.i504.us.us.us.i, i32 %_6.i.i502.us.us.us.i, !dbg !4230
  %_4.i398.us.us.us.i = select i1 %_3.i204.i, i32 %_6.i.i502.us.us.us.i, i32 %_4.i.i507.us.us.us.i, !dbg !4231
  %_0.i245.us.us.us.i = fmul float %468, 5.000000e-01, !dbg !4233
  %_0.i244.us.us.us.i = fmul float %469, 5.000000e-01, !dbg !4235
  %_0.i219.us.us.us.i = fadd float %_0.i244.us.us.us.i, %_0.i245.us.us.us.i, !dbg !4237
  %_6.i386.us.us.us.i = bitcast float %_0.i219.us.us.us.i to i32, !dbg !4239
  %_4.i391.us.us.us.i = select i1 %_3.i202.i, i32 %_4.i398.us.us.us.i, i32 %_6.i386.us.us.us.i, !dbg !4242
  %_0.i392.us.us.us.i = bitcast i32 %_4.i391.us.us.us.i to float, !dbg !4243
  %_3.i.i492.us.us.us.i = fcmp ule float %_0.i392.us.us.us.i, 0x3E45798EE0000000, !dbg !4245
  %_4.i.i498.us.us.us.i = select i1 %_3.i.i492.us.us.us.i, i32 841731191, i32 %_4.i391.us.us.us.i, !dbg !4248
  %_0.i.i499.us.us.us.i = bitcast i32 %_4.i.i498.us.us.us.i to float, !dbg !4250
  %_3.i.i.us.us.us.i340 = fcmp ule float %_0.i.i499.us.us.us.i, 0x3810000000000000, !dbg !4252
  %_4.i.i.us.us.us.i341 = select i1 %_3.i.i.us.us.us.i340, i32 8388608, i32 %_4.i.i498.us.us.us.i, !dbg !4257
  %_5.i280.us.us.us.i = and i32 %_4.i.i.us.us.us.i341, 8388607, !dbg !4259
  %_4.i281.us.us.us.i = or disjoint i32 %_5.i280.us.us.us.i, 1065353216, !dbg !4259
  %significand.i.us.us.us.i342 = bitcast i32 %_4.i281.us.us.us.i to float, !dbg !4261
  %_0.i246.us.us.us.i343 = fadd float %significand.i.us.us.us.i342, -1.000000e+00, !dbg !4263
  %_0.i225.us.us.us.i = fmul float %_0.i246.us.us.us.i343, 0x3F9B17A960000000, !dbg !4265
  %470 = fsub float 0x3FBF9A8440000000, %_0.i225.us.us.us.i, !dbg !4267
  %_0.i225.us.us.us.1.i = fmul float %_0.i246.us.us.us.i343, %470, !dbg !4265
  %_0.i211.us.us.us.1.i = fadd float %_0.i225.us.us.us.1.i, 0xBFD1E3F400000000, !dbg !4267
  %_0.i225.us.us.us.2.i = fmul float %_0.i246.us.us.us.i343, %_0.i211.us.us.us.1.i, !dbg !4265
  %_0.i211.us.us.us.2.i = fadd float %_0.i225.us.us.us.2.i, 0x3FDD544F20000000, !dbg !4267
  %_0.i225.us.us.us.3.i = fmul float %_0.i246.us.us.us.i343, %_0.i211.us.us.us.2.i, !dbg !4265
  %_0.i211.us.us.us.3.i = fadd float %_0.i225.us.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4267
  %_0.i225.us.us.us.4.i = fmul float %_0.i246.us.us.us.i343, %_0.i211.us.us.us.3.i, !dbg !4265
  %_0.i211.us.us.us.4.i = fadd float %_0.i225.us.us.us.4.i, 0x3FF714B2A0000000, !dbg !4267
  %_9.i.us.us.us.i344 = lshr i32 %_4.i.i.us.us.us.i341, 23, !dbg !4269
  %_8.i282.us.us.us.i = or disjoint i32 %_9.i.us.us.us.i344, 1258291200, !dbg !4269
  %_7.i.us.us.us.i345 = bitcast i32 %_8.i282.us.us.us.i to float, !dbg !4270
  %exponent.i.us.us.us.i346 = fadd float %_7.i.us.us.us.i345, 0xC160000FE0000000, !dbg !4272
  %_0.i224.us.us.us.i = fmul float %_0.i246.us.us.us.i343, %_0.i211.us.us.us.4.i, !dbg !4273
  %_0.i210.us.us.us.i = fadd float %exponent.i.us.us.us.i346, %_0.i224.us.us.us.i, !dbg !4275
  %_0.i243.us.us.us.i = fmul float %_0.i210.us.us.us.i, 0x4018151820000000, !dbg !4277
  %_3.i.i549.us.us.us.inv.i = fcmp olt float %_0.i243.us.us.us.i, 2.400000e+01, !dbg !4279
  %_0.i.i556.us.us.us.i = select i1 %_3.i.i549.us.us.us.inv.i, float %_0.i243.us.us.us.i, float 2.400000e+01, !dbg !4279
  %_3.i.i484.us.us.us.inv.i = fcmp ogt float %_0.i.i556.us.us.us.i, -1.600000e+02, !dbg !4282
  %_0.i.i491.us.us.us.i = select i1 %_3.i.i484.us.us.us.inv.i, float %_0.i.i556.us.us.us.i, float -1.600000e+02, !dbg !4282
  %_55.i11.us.us.us.i = load float, ptr %371, align 4, !dbg !4285, !alias.scope !4286, !noalias !4289, !noundef !12
  %_3.i200.us.us.us.i347 = fcmp ule float %_55.i11.us.us.us.i, 0.000000e+00, !dbg !4291
  %_3.i176.us.us.us.i = fcmp oge float %_0.i.i491.us.us.us.i, %threshold.i.i, !dbg !4293
  %_3.i174.us.us.us.i = fcmp oge float %_0.i.i491.us.us.us.i, %_0.i259.i, !dbg !4295
  %..i175.us.us.us.i = sext i1 %_3.i174.us.us.us.i to i32, !dbg !4297
  %_0.i412.us.us.us.i = sext i1 %_3.i176.us.us.us.i to i32, !dbg !4299
  %_0.i405.us.us.us.i = select i1 %_3.i200.us.us.us.i347, i32 %_0.i412.us.us.us.i, i32 %..i175.us.us.us.i, !dbg !4299
  %_0.i416.us.us.us.i = xor i32 %..i175.us.us.us.i, -1, !dbg !4301
  %_3.i198.us.us.us.i348 = fcmp ogt float %_67.i131005.us.us.us.i, 0.000000e+00, !dbg !4303
  %_0.i411.us.us.us.i = select i1 %_3.i198.us.us.us.i348, i32 %_0.i416.us.us.us.i, i32 0, !dbg !4305
  %_0.i410.us.us.us.i = select i1 %_3.i200.us.us.us.i347, i32 0, i32 %_0.i411.us.us.us.i, !dbg !4307
  %_0.i404.us.us.us.i = or i32 %_0.i410.us.us.us.i, %_0.i405.us.us.us.i, !dbg !4309
  %_5.i381.us.us.us.i = and i32 %_0.i404.us.us.us.i, 1065353216, !dbg !4311
  %_0.i385.us.us.us.i = bitcast i32 %_5.i381.us.us.us.i to float, !dbg !4313
  %_0.i258.us.us.us.i = fadd float %_67.i131005.us.us.us.i, -1.000000e+00, !dbg !4315
  %471 = trunc nsw i32 %_0.i410.us.us.us.i to i1, !dbg !4317
  %_4.i379.v.us.us.us.i = select i1 %471, float %_0.i258.us.us.us.i, float %_67.i131005.us.us.us.i, !dbg !4317
  %472 = trunc nsw i32 %_0.i405.us.us.us.i to i1, !dbg !4319
  %_0.i373.us.us.us.i349 = select i1 %472, float %_71.i585586.i, float %_4.i379.v.us.us.us.i, !dbg !4319
  store float %_0.i373.us.us.us.i349, ptr %372, align 4, !dbg !4321, !alias.scope !4286, !noalias !4289
  store i32 %_5.i381.us.us.us.i, ptr %371, align 4, !dbg !4322, !alias.scope !4286, !noalias !4289
  %_0.i256.us.us.us.i350 = fsub float %_0.i.i491.us.us.us.i, %threshold.i.i, !dbg !4323
  %_0.i242.us.us.us.i = fmul float %_0.i257.i, %_0.i256.us.us.us.i350, !dbg !4325
  %_3.i.i475.inv.us.us.us.i = fcmp ogt float %_0.i242.us.us.us.i, %374, !dbg !4327
  %_4.i.i482.v.us.us.us.i = select i1 %_3.i.i475.inv.us.us.us.i, float %_0.i242.us.us.us.i, float %374, !dbg !4327
  %_3.i.i541.us.us.us.i = fcmp olt float %_4.i.i482.v.us.us.us.i, 0.000000e+00, !dbg !4330
  %473 = fcmp ule float %_0.i385.us.us.us.i, 0.000000e+00, !dbg !4333
  %474 = select i1 %473, i1 %_3.i.i541.us.us.us.i, i1 false, !dbg !4335
  %_0.i366.us.us.us.i = select i1 %474, float %_4.i.i482.v.us.us.us.i, float 0.000000e+00, !dbg !4335
  %_3.i194.us.us.us.i = fcmp ule float %_0.i366.us.us.us.i, %_86.i211007.us.us.us.i, !dbg !4336
  %_4.i360.us.us.us.i = select i1 %_3.i194.us.us.us.i, i32 %_88.i24589.i, i32 %_87.i23588.i, !dbg !4338
  %_0.i.us.us.us.i351 = bitcast i32 %_4.i360.us.us.us.i to float, !dbg !4340
  %_0.i255.us.us.us.i352 = fsub float %_0.i366.us.us.us.i, %_86.i211007.us.us.us.i, !dbg !4342
  %_4.i222.us.us.us.i = fmul float %_0.i255.us.us.us.i352, %_0.i.us.us.us.i351, !dbg !4344
  %_0.i223.us.us.us.i = fadd float %_86.i211007.us.us.us.i, %_4.i222.us.us.us.i, !dbg !4344
  %475 = tail call noundef float @llvm.fabs.f32(float %_0.i223.us.us.us.i), !dbg !4346
  %476 = fcmp uge float %475, 0x3BC79CA100000000, !dbg !4349
  %_0.i296.us.us.us.i353 = select i1 %476, float %_0.i223.us.us.us.i, float 0.000000e+00, !dbg !4351
  store float %_0.i296.us.us.us.i353, ptr %375, align 4, !dbg !4352, !alias.scope !4286, !noalias !4289
  %_0.i241.us.us.us.i = fmul float %_0.i296.us.us.us.i353, 0x3FC542A5A0000000, !dbg !4353
  %_3.i.i426.us.us.us.inv.i = fcmp ogt float %_0.i241.us.us.us.i, -1.260000e+02, !dbg !4356
  %_0.i.i433.us.us.us.i = select i1 %_3.i.i426.us.us.us.inv.i, float %_0.i241.us.us.us.i, float -1.260000e+02, !dbg !4356
  %_3.i.i509.us.us.us.inv.i = fcmp olt float %_0.i.i433.us.us.us.i, 1.270000e+02, !dbg !4360
  %_0.i.i516.us.us.us.i = select i1 %_3.i.i509.us.us.us.inv.i, float %_0.i.i433.us.us.us.i, float 1.270000e+02, !dbg !4360
  %477 = tail call noundef float @llvm.floor.f32(float %_0.i.i516.us.us.us.i), !dbg !4363
  %_0.i248.us.us.us.i354 = fsub float %_0.i.i516.us.us.us.i, %477, !dbg !4367
  %_0.i230.us.us.us.i = fmul float %_0.i248.us.us.us.i354, 0x3F5E974FA0000000, !dbg !4369
  %_0.i215.us.us.us.i = fadd float %_0.i230.us.us.us.i, 0x3F82778560000000, !dbg !4371
  %_0.i230.us.us.us.1.i = fmul float %_0.i248.us.us.us.i354, %_0.i215.us.us.us.i, !dbg !4369
  %_0.i215.us.us.us.1.i = fadd float %_0.i230.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4371
  %_0.i230.us.us.us.2.i = fmul float %_0.i248.us.us.us.i354, %_0.i215.us.us.us.1.i, !dbg !4369
  %_0.i215.us.us.us.2.i = fadd float %_0.i230.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4371
  %_0.i230.us.us.us.3.i = fmul float %_0.i248.us.us.us.i354, %_0.i215.us.us.us.2.i, !dbg !4369
  %_0.i215.us.us.us.3.i = fadd float %_0.i230.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4371
  %_0.i233.us.us.us.i = fmul float %_0.i249.us.us.us.i339, 0x3F5E974FA0000000, !dbg !4373
  %_0.i217.us.us.us.i = fadd float %_0.i233.us.us.us.i, 0x3F82778560000000, !dbg !4375
  %_0.i233.us.us.us.1.i = fmul float %_0.i249.us.us.us.i339, %_0.i217.us.us.us.i, !dbg !4373
  %_0.i217.us.us.us.1.i = fadd float %_0.i233.us.us.us.1.i, 0x3FAC91CE60000000, !dbg !4375
  %_0.i233.us.us.us.2.i = fmul float %_0.i249.us.us.us.i339, %_0.i217.us.us.us.1.i, !dbg !4373
  %_0.i217.us.us.us.2.i = fadd float %_0.i233.us.us.us.2.i, 0x3FCEBDB560000000, !dbg !4375
  %_0.i233.us.us.us.3.i = fmul float %_0.i249.us.us.us.i339, %_0.i217.us.us.us.2.i, !dbg !4373
  %_0.i217.us.us.us.3.i = fadd float %_0.i233.us.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4375
  %_0.i232.us.us.us.i = fmul float %_0.i249.us.us.us.i339, %_0.i217.us.us.us.3.i, !dbg !4377
  %_0.i216.us.us.us.i = fadd float %_0.i232.us.us.us.i, 1.000000e+00, !dbg !4379
  %biased.i163.us.us.us.i = fadd float %467, 0x4160000FE0000000, !dbg !4381
  %_4.i164.us.us.us.i = bitcast float %biased.i163.us.us.us.i to i32, !dbg !4383
  %_3.i165.us.us.us.i = shl i32 %_4.i164.us.us.us.i, 23, !dbg !4385
  %_0.i166.us.us.us.i = bitcast i32 %_3.i165.us.us.us.i to float, !dbg !4386
  %_0.i231.us.us.us.i = fmul float %_0.i216.us.us.us.i, %_0.i166.us.us.us.i, !dbg !4388
  %_3.i167.us.us.us.i = fcmp une float %_0.i292.us.us.us.i338, 0.000000e+00, !dbg !4390
  %_0.i400576.not.us.us.us.i = and i1 %_3.i178.i, %_3.i167.us.us.us.i, !dbg !4393
  %_0.i234.us.us.us.i = fmul float %_0.i271.us.us.us.i311, %_0.i231.us.us.us.i, !dbg !4393
  %_4.i300.v.us.us.us.i = select i1 %_0.i400576.not.us.us.us.i, float %_0.i234.us.us.us.i, float %_0.i271.us.us.us.i311, !dbg !4396
  %_0.i229.us.us.us.i = fmul float %_0.i248.us.us.us.i354, %_0.i215.us.us.us.3.i, !dbg !4398
  %_0.i214.us.us.us.i = fadd float %_0.i229.us.us.us.i, 1.000000e+00, !dbg !4400
  %biased.i.us.us.us.i355 = fadd float %477, 0x4160000FE0000000, !dbg !4402
  %_4.i160.us.us.us.i = bitcast float %biased.i.us.us.us.i355 to i32, !dbg !4404
  %_3.i161.us.us.us.i = shl i32 %_4.i160.us.us.us.i, 23, !dbg !4406
  %_0.i162.us.us.us.i = bitcast i32 %_3.i161.us.us.us.i to float, !dbg !4407
  %_0.i228.us.us.us.i = fmul float %_0.i214.us.us.us.i, %_0.i162.us.us.us.i, !dbg !4409
  %_3.i168.us.us.us.i = fcmp une float %_0.i296.us.us.us.i353, 0.000000e+00, !dbg !4411
  %_0.i403593.not.us.us.us.i = and i1 %_3.i192.i, %_3.i168.us.us.us.i, !dbg !4413
  %_0.i240.us.us.us.i = fmul float %_0.i269.us.us.us.i315, %_0.i228.us.us.us.i, !dbg !4413
  %_4.i353.v.us.us.us.i = select i1 %_0.i403593.not.us.us.us.i, float %_0.i240.us.us.us.i, float %_0.i269.us.us.us.i315, !dbg !4415
  store float %_4.i300.v.us.us.us.i, ptr %_123.i.us.us.us.i285, align 4, !dbg !4417, !alias.scope !4420, !noalias !3914
  store float %_4.i353.v.us.us.us.i, ptr %_141.i.us.us.us.i290, align 4, !dbg !4423, !alias.scope !4425, !noalias !3936
  %exitcond2534.not.i = icmp eq i64 %453, %_26, !dbg !4428
  br i1 %exitcond2534.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb40.i.us.us.us.i279, !dbg !4431, !llvm.loop !4437

bb42.i.us.us.i356:                                ; preds = %bb40.i.lr.ph.split.us.split.us.i, %bb28.i.us.us.lr.ph.us.us.i406
  %_86.i211007.us.us.i = phi float [ %_0.i296.us.us.i429, %bb28.i.us.us.lr.ph.us.us.i406 ], [ %.promoted1006.i, %bb40.i.lr.ph.split.us.split.us.i ]
  %_67.i131005.us.us.i = phi float [ %_0.i373.us.us.i425, %bb28.i.us.us.lr.ph.us.us.i406 ], [ %.promoted1004.i, %bb40.i.lr.ph.split.us.split.us.i ]
  %_86.i881003.us.us.i = phi float [ %_0.i292.us.us.i414, %bb28.i.us.us.lr.ph.us.us.i406 ], [ %.promoted1002.i, %bb40.i.lr.ph.split.us.split.us.i ]
  %_67.i691001.us.us.i = phi float [ %_0.i321.us.us.i, %bb28.i.us.us.lr.ph.us.us.i406 ], [ %.promoted1000.i, %bb40.i.lr.ph.split.us.split.us.i ]
  %iter.sroa.0.0.i966.us.us.i = phi i64 [ %478, %bb28.i.us.us.lr.ph.us.us.i406 ], [ 0, %bb40.i.lr.ph.split.us.split.us.i ]
  %478 = add nuw nsw i64 %iter.sroa.0.0.i966.us.us.i, 1, !dbg !3871
  %_27.i.us.us.i357 = trunc i64 %iter.sroa.0.0.i966.us.us.i to i32, !dbg !3885
  %now.i.us.us.i358 = add i32 %base.i.i46, %_27.i.us.us.i357, !dbg !3888
  %_30.i.us.us.i359 = and i32 %now.i.us.us.i358, %_52.i30, !dbg !3891
  %_29.i.us.us.i360 = zext i32 %_30.i.us.us.i359 to i64, !dbg !3893
  %_123.i.us.us.i361 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i966.us.us.i, !dbg !3900
  %_124.not.not.i.us.us.i362 = icmp ugt i64 %_58.1.i20, %_29.i.us.us.i360, !dbg !3904
  br i1 %_124.not.not.i.us.us.i362, label %bb45.i.us.us.i363, label %bb46.i.i58, !dbg !3904, !prof !2704

bb45.i.us.us.i363:                                ; preds = %bb42.i.us.us.i356
  %_0.i279.us.us.i364 = load float, ptr %_123.i.us.us.i361, align 4, !dbg !3909, !alias.scope !3911, !noalias !3914, !noundef !12
  %_133.i.us.us.i365 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_29.i.us.us.i360, !dbg !3915
  store float %_0.i279.us.us.i364, ptr %_133.i.us.us.i365, align 4, !dbg !3919, !alias.scope !3921, !noalias !3868
  %_141.i.us.us.i366 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i966.us.us.i, !dbg !3924
  %_0.i277.us.us.i367 = load float, ptr %_141.i.us.us.i366, align 4, !dbg !3931, !alias.scope !3933, !noalias !3936, !noundef !12
  %_149.i.us.us.i368 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_29.i.us.us.i360, !dbg !3937
  store float %_0.i277.us.us.i367, ptr %_149.i.us.us.i368, align 4, !dbg !3944, !alias.scope !3946, !noalias !3868
  %exitcond2535.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.us.i, %side_left.sroa.5.0.i.i45, !dbg !4438
  br i1 %exitcond2535.not.i, label %bb53.i.i432, label %bb52.i.us.us.i369, !dbg !4438, !prof !180

bb52.i.us.us.i369:                                ; preds = %bb45.i.us.us.i363
  %_158.not.not.i.us.us.i370 = icmp ugt i64 %_59.1.i22, %_29.i.us.us.i360, !dbg !4436
  br i1 %_158.not.not.i.us.us.i370, label %bb54.i.us.us.i371, label %bb55.i.i294, !dbg !4436, !prof !2704

bb54.i.us.us.i371:                                ; preds = %bb52.i.us.us.i369
  %_157.i.us.us.i372 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i44, i64 %iter.sroa.0.0.i966.us.us.i, !dbg !3949
  %_0.i275.us.us.i373 = load float, ptr %_157.i.us.us.i372, align 4, !dbg !3956, !alias.scope !3958, !noalias !3868, !noundef !12
  %_165.i.us.us.i374 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_29.i.us.us.i360, !dbg !3961
  store float %_0.i275.us.us.i373, ptr %_165.i.us.us.i374, align 4, !dbg !3968, !alias.scope !3970, !noalias !3868
  %exitcond2536.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.us.i, %empty.sroa.6.0.i.i42, !dbg !4434
  br i1 %exitcond2536.not.i, label %bb57.i.i278, label %bb56.i.us.us.i375, !dbg !4434, !prof !180

bb56.i.us.us.i375:                                ; preds = %bb54.i.us.us.i371
  %_174.not.not.i.us.us.i376 = icmp ugt i64 %_61.1.i27, %_29.i.us.us.i360, !dbg !4432
  br i1 %_174.not.not.i.us.us.i376, label %bb58.i.us.us.i377, label %bb59.i.i146, !dbg !4432, !prof !2704

bb58.i.us.us.i377:                                ; preds = %bb56.i.us.us.i375
  %_173.i.us.us.i378 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i43, i64 %iter.sroa.0.0.i966.us.us.i, !dbg !3973
  %_0.i273.us.us.i379 = load float, ptr %_173.i.us.us.i378, align 4, !dbg !3980, !alias.scope !3982, !noalias !3868, !noundef !12
  %_181.i.us.us.i380 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_29.i.us.us.i360, !dbg !3985
  store float %_0.i273.us.us.i379, ptr %_181.i.us.us.i380, align 4, !dbg !3992, !alias.scope !3994, !noalias !3868
  %_52.i.us.us.i381 = sub i32 %now.i.us.us.i358, %_53.i31, !dbg !3997
  %_51.i.us.us.i382 = and i32 %_52.i.us.us.i381, %_52.i30, !dbg !4000
  %_50.i.us.us.i383 = zext i32 %_51.i.us.us.i382 to i64, !dbg !4001
  %_182.not.not.i.us.us.i384 = icmp ugt i64 %_58.1.i20, %_50.i.us.us.i383, !dbg !4002
  br i1 %_182.not.not.i.us.us.i384, label %bb60.i.us.us.i385, label %bb61.i.i75, !dbg !4002, !prof !2704

bb60.i.us.us.i385:                                ; preds = %bb58.i.us.us.i377
  %_189.i.us.us.i386 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_50.i.us.us.i383, !dbg !4007
  %_0.i271.us.us.i387 = load float, ptr %_189.i.us.us.i386, align 4, !dbg !4011, !alias.scope !4013, !noalias !3868, !noundef !12
  %_195.i.us.us.i390 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_50.i.us.us.i383, !dbg !4016
  %_0.i269.us.us.i391 = load float, ptr %_195.i.us.us.i390, align 4, !dbg !4024, !alias.scope !4026, !noalias !3868, !noundef !12
  %_66.i.us.us.i392 = sub i32 %now.i.us.us.i358, %_67.i.i
  %_65.i.us.us.i393 = and i32 %_66.i.us.us.i392, %_52.i30
  %_64.i.us.us.i394 = zext i32 %_65.i.us.us.i393 to i64
  %_74.i.us.us.i395 = sub i32 %now.i.us.us.i358, %_75.i.i
  %_73.i.us.us.i396 = and i32 %_74.i.us.us.i395, %_52.i30
  %_72.i.us.us.i397 = zext i32 %_73.i.us.us.i396 to i64
  %_80.i.us.us.i398 = icmp ugt i64 %_59.1.i22, %_64.i.us.us.i394
  %479 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_64.i.us.us.i394
  %480 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_64.i.us.us.i394
  %_86.i.us.us.i399 = icmp ugt i64 %_61.1.i27, %_72.i.us.us.i397
  %481 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_72.i.us.us.i397
  %_88.i.us.us.i400 = icmp ugt i64 %_59.1.i22, %_72.i.us.us.i397
  %482 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_72.i.us.us.i397
  br i1 %_80.i.us.us.i398, label %bb63.i.split.us.us.us.i401, label %bb63.i.split.i90

bb63.i.split.us.us.us.i401:                       ; preds = %bb60.i.us.us.i385
  %_84.i.us.us.i402 = icmp ugt i64 %_61.1.i27, %_64.i.us.us.i394
  br i1 %_84.i.us.us.i402, label %bb24.i.us.lr.ph.us.us.i403, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i93, !dbg !4029

bb24.i.us.lr.ph.us.us.i403:                       ; preds = %bb63.i.split.us.us.us.i401
  %_82.i.us.us.us.i404 = load float, ptr %480, align 4, !noalias !3868, !noundef !12
  br i1 %_86.i.us.us.i399, label %bb24.i.us.lr.ph.split.us.us.us.i405, label %bb24.i.us.lr.ph.split.i96

bb24.i.us.lr.ph.split.us.us.us.i405:              ; preds = %bb24.i.us.lr.ph.us.us.i403
  br i1 %_88.i.us.us.i400, label %bb28.i.us.us.lr.ph.us.us.i406, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i98, !dbg !4037

bb28.i.us.us.lr.ph.us.us.i406:                    ; preds = %bb24.i.us.lr.ph.split.us.us.us.i405
  %_87.i.us.us.us.us.i407 = load float, ptr %482, align 4, !noalias !3868, !noundef !12
  %_85.i.us.us.le827.us.us.i = load float, ptr %481, align 4, !noalias !3868, !noundef !12
  %_78.i.us.le.us.us.i408 = load float, ptr %479, align 4, !noalias !3868, !noundef !12
  %483 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.i408), !dbg !4038
  %484 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.i404), !dbg !4048
  %_3.i.i466.us.us.i = fcmp ule float %483, %484, !dbg !4051
  %_6.i.i468.us.us.i = bitcast float %483 to i32, !dbg !4056
  %_8.i.i470.us.us.i = bitcast float %484 to i32, !dbg !4059
  %_4.i.i473.us.us.i = select i1 %_3.i.i466.us.us.i, i32 %_8.i.i470.us.us.i, i32 %_6.i.i468.us.us.i, !dbg !4061
  %_4.i346.us.us.i = select i1 %_3.i190.i, i32 %_6.i.i468.us.us.i, i32 %_4.i.i473.us.us.i, !dbg !4062
  %_0.i239.us.us.i = fmul float %483, 5.000000e-01, !dbg !4064
  %_0.i238.us.us.i = fmul float %484, 5.000000e-01, !dbg !4066
  %_0.i218.us.us.i = fadd float %_0.i238.us.us.i, %_0.i239.us.us.i, !dbg !4068
  %_6.i334.us.us.i = bitcast float %_0.i218.us.us.i to i32, !dbg !4070
  %_4.i339.us.us.i = select i1 %_3.i188.i, i32 %_4.i346.us.us.i, i32 %_6.i334.us.us.i, !dbg !4073
  %_0.i340.us.us.i = bitcast i32 %_4.i339.us.us.i to float, !dbg !4074
  %_3.i.i458.us.us.i = fcmp ule float %_0.i340.us.us.i, 0x3E45798EE0000000, !dbg !4076
  %_4.i.i464.us.us.i = select i1 %_3.i.i458.us.us.i, i32 841731191, i32 %_4.i339.us.us.i, !dbg !4079
  %_0.i.i465.us.us.i = bitcast i32 %_4.i.i464.us.us.i to float, !dbg !4081
  %_3.i.i418.us.us.i = fcmp ule float %_0.i.i465.us.us.i, 0x3810000000000000, !dbg !4083
  %_4.i.i424.us.us.i = select i1 %_3.i.i418.us.us.i, i32 8388608, i32 %_4.i.i464.us.us.i, !dbg !4089
  %_5.i284.us.us.i = and i32 %_4.i.i424.us.us.i, 8388607, !dbg !4091
  %_4.i285.us.us.i = or disjoint i32 %_5.i284.us.us.i, 1065353216, !dbg !4091
  %significand.i286.us.us.i = bitcast i32 %_4.i285.us.us.i to float, !dbg !4093
  %_0.i247.us.us.i409 = fadd float %significand.i286.us.us.i, -1.000000e+00, !dbg !4095
  %_0.i227.us.us.i = fmul float %_0.i247.us.us.i409, 0x3F9B17A960000000, !dbg !4097
  %485 = fsub float 0x3FBF9A8440000000, %_0.i227.us.us.i, !dbg !4099
  %_0.i227.us.us.1.i = fmul float %_0.i247.us.us.i409, %485, !dbg !4097
  %_0.i213.us.us.1.i = fadd float %_0.i227.us.us.1.i, 0xBFD1E3F400000000, !dbg !4099
  %_0.i227.us.us.2.i = fmul float %_0.i247.us.us.i409, %_0.i213.us.us.1.i, !dbg !4097
  %_0.i213.us.us.2.i = fadd float %_0.i227.us.us.2.i, 0x3FDD544F20000000, !dbg !4099
  %_0.i227.us.us.3.i = fmul float %_0.i247.us.us.i409, %_0.i213.us.us.2.i, !dbg !4097
  %_0.i213.us.us.3.i = fadd float %_0.i227.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4099
  %_0.i227.us.us.4.i = fmul float %_0.i247.us.us.i409, %_0.i213.us.us.3.i, !dbg !4097
  %_0.i213.us.us.4.i = fadd float %_0.i227.us.us.4.i, 0x3FF714B2A0000000, !dbg !4099
  %_9.i287.us.us.i = lshr i32 %_4.i.i424.us.us.i, 23, !dbg !4101
  %_8.i288.us.us.i = or disjoint i32 %_9.i287.us.us.i, 1258291200, !dbg !4101
  %_7.i289.us.us.i = bitcast i32 %_8.i288.us.us.i to float, !dbg !4102
  %exponent.i290.us.us.i = fadd float %_7.i289.us.us.i, 0xC160000FE0000000, !dbg !4104
  %_0.i226.us.us.i = fmul float %_0.i247.us.us.i409, %_0.i213.us.us.4.i, !dbg !4105
  %_0.i212.us.us.i = fadd float %exponent.i290.us.us.i, %_0.i226.us.us.i, !dbg !4107
  %_0.i237.us.us.i = fmul float %_0.i212.us.us.i, 0x4018151820000000, !dbg !4109
  %_3.i.i533.us.us.inv.i = fcmp olt float %_0.i237.us.us.i, 2.400000e+01, !dbg !4111
  %_0.i.i540.us.us.i = select i1 %_3.i.i533.us.us.inv.i, float %_0.i237.us.us.i, float 2.400000e+01, !dbg !4111
  %_3.i.i450.us.us.inv.i = fcmp ogt float %_0.i.i540.us.us.i, -1.600000e+02, !dbg !4114
  %_0.i.i457.us.us.i = select i1 %_3.i.i450.us.us.inv.i, float %_0.i.i540.us.us.i, float -1.600000e+02, !dbg !4114
  %_55.i59.us.us.i = load float, ptr %359, align 4, !dbg !4117, !alias.scope !4119, !noalias !4122, !noundef !12
  %_3.i186.us.us.i = fcmp ule float %_55.i59.us.us.i, 0.000000e+00, !dbg !4124
  %_3.i172.us.us.i = fcmp oge float %_0.i.i457.us.us.i, %threshold.i32.i, !dbg !4126
  %_3.i170.us.us.i = fcmp oge float %_0.i.i457.us.us.i, %_0.i254.i, !dbg !4129
  %..i171.us.us.i = sext i1 %_3.i170.us.us.i to i32, !dbg !4132
  %_0.i408.us.us.i = sext i1 %_3.i172.us.us.i to i32, !dbg !4134
  %_0.i402.us.us.i = select i1 %_3.i186.us.us.i, i32 %_0.i408.us.us.i, i32 %..i171.us.us.i, !dbg !4134
  %_0.i414.us.us.i = xor i32 %..i171.us.us.i, -1, !dbg !4139
  %_3.i184.us.us.i = fcmp ogt float %_67.i691001.us.us.i, 0.000000e+00, !dbg !4142
  %_0.i407.us.us.i410 = select i1 %_3.i184.us.us.i, i32 %_0.i414.us.us.i, i32 0, !dbg !4144
  %_0.i406.us.us.i = select i1 %_3.i186.us.us.i, i32 0, i32 %_0.i407.us.us.i410, !dbg !4146
  %_0.i401.us.us.i = or i32 %_0.i406.us.us.i, %_0.i402.us.us.i, !dbg !4148
  %_5.i329.us.us.i = and i32 %_0.i401.us.us.i, 1065353216, !dbg !4151
  %_0.i333.us.us.i = bitcast i32 %_5.i329.us.us.i to float, !dbg !4153
  %_0.i253.us.us.i411 = fadd float %_67.i691001.us.us.i, -1.000000e+00, !dbg !4155
  %486 = trunc nsw i32 %_0.i406.us.us.i to i1, !dbg !4158
  %_4.i327.v.us.us.i = select i1 %486, float %_0.i253.us.us.i411, float %_67.i691001.us.us.i, !dbg !4158
  %487 = trunc nsw i32 %_0.i402.us.us.i to i1, !dbg !4160
  %_0.i321.us.us.i = select i1 %487, float %_71.i75568569.i, float %_4.i327.v.us.us.i, !dbg !4160
  store float %_0.i321.us.us.i, ptr %360, align 4, !dbg !4162, !alias.scope !4119, !noalias !4122
  store i32 %_5.i329.us.us.i, ptr %359, align 4, !dbg !4163, !alias.scope !4119, !noalias !4122
  %_0.i251.us.us.i412 = fsub float %_0.i.i457.us.us.i, %threshold.i32.i, !dbg !4164
  %_0.i236.us.us.i = fmul float %_0.i252.i, %_0.i251.us.us.i412, !dbg !4166
  %_3.i.i442.inv.us.us.i = fcmp ogt float %_0.i236.us.us.i, %362, !dbg !4168
  %_4.i.i448.v.us.us.i = select i1 %_3.i.i442.inv.us.us.i, float %_0.i236.us.us.i, float %362, !dbg !4168
  %_3.i.i525.us.us.i = fcmp olt float %_4.i.i448.v.us.us.i, 0.000000e+00, !dbg !4171
  %488 = fcmp ule float %_0.i333.us.us.i, 0.000000e+00, !dbg !4174
  %489 = select i1 %488, i1 %_3.i.i525.us.us.i, i1 false, !dbg !4177
  %_0.i314.us.us.i = select i1 %489, float %_4.i.i448.v.us.us.i, float 0.000000e+00, !dbg !4177
  %_3.i180.us.us.i = fcmp ule float %_0.i314.us.us.i, %_86.i881003.us.us.i, !dbg !4178
  %_4.i307.us.us.i = select i1 %_3.i180.us.us.i, i32 %_88.i91572.i, i32 %_87.i90571.i, !dbg !4181
  %_0.i308.us.us.i = bitcast i32 %_4.i307.us.us.i to float, !dbg !4183
  %_0.i250.us.us.i413 = fsub float %_0.i314.us.us.i, %_86.i881003.us.us.i, !dbg !4185
  %_4.i220.us.us.i = fmul float %_0.i250.us.us.i413, %_0.i308.us.us.i, !dbg !4188
  %_0.i221.us.us.i = fadd float %_86.i881003.us.us.i, %_4.i220.us.us.i, !dbg !4188
  %490 = tail call noundef float @llvm.fabs.f32(float %_0.i221.us.us.i), !dbg !4190
  %491 = fcmp uge float %490, 0x3BC79CA100000000, !dbg !4194
  %_0.i292.us.us.i414 = select i1 %491, float %_0.i221.us.us.i, float 0.000000e+00, !dbg !4196
  store float %_0.i292.us.us.i414, ptr %363, align 4, !dbg !4197, !alias.scope !4119, !noalias !4122
  %_0.i235.us.us.i = fmul float %_0.i292.us.us.i414, 0x3FC542A5A0000000, !dbg !4199
  %_3.i.i434.us.us.inv.i = fcmp ogt float %_0.i235.us.us.i, -1.260000e+02, !dbg !4203
  %_0.i.i441.us.us.i = select i1 %_3.i.i434.us.us.inv.i, float %_0.i235.us.us.i, float -1.260000e+02, !dbg !4203
  %_3.i.i517.us.us.inv.i = fcmp olt float %_0.i.i441.us.us.i, 1.270000e+02, !dbg !4207
  %_0.i.i524.us.us.i = select i1 %_3.i.i517.us.us.inv.i, float %_0.i.i441.us.us.i, float 1.270000e+02, !dbg !4207
  %492 = tail call noundef float @llvm.floor.f32(float %_0.i.i524.us.us.i), !dbg !4210
  %_0.i249.us.us.i415 = fsub float %_0.i.i524.us.us.i, %492, !dbg !4214
  %493 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le827.us.us.i), !dbg !4216
  %494 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.i407), !dbg !4220
  %_3.i.i500.us.us.i = fcmp ule float %493, %494, !dbg !4222
  %_6.i.i502.us.us.i = bitcast float %493 to i32, !dbg !4225
  %_8.i.i504.us.us.i = bitcast float %494 to i32, !dbg !4228
  %_4.i.i507.us.us.i = select i1 %_3.i.i500.us.us.i, i32 %_8.i.i504.us.us.i, i32 %_6.i.i502.us.us.i, !dbg !4230
  %_4.i398.us.us.i = select i1 %_3.i204.i, i32 %_6.i.i502.us.us.i, i32 %_4.i.i507.us.us.i, !dbg !4231
  %_0.i245.us.us.i = fmul float %493, 5.000000e-01, !dbg !4233
  %_0.i244.us.us.i = fmul float %494, 5.000000e-01, !dbg !4235
  %_0.i219.us.us.i = fadd float %_0.i244.us.us.i, %_0.i245.us.us.i, !dbg !4237
  %_6.i386.us.us.i = bitcast float %_0.i219.us.us.i to i32, !dbg !4239
  %_4.i391.us.us.i = select i1 %_3.i202.i, i32 %_4.i398.us.us.i, i32 %_6.i386.us.us.i, !dbg !4242
  %_0.i392.us.us.i = bitcast i32 %_4.i391.us.us.i to float, !dbg !4243
  %_3.i.i492.us.us.i = fcmp ule float %_0.i392.us.us.i, 0x3E45798EE0000000, !dbg !4245
  %_4.i.i498.us.us.i = select i1 %_3.i.i492.us.us.i, i32 841731191, i32 %_4.i391.us.us.i, !dbg !4248
  %_0.i.i499.us.us.i = bitcast i32 %_4.i.i498.us.us.i to float, !dbg !4250
  %_3.i.i.us.us.i416 = fcmp ule float %_0.i.i499.us.us.i, 0x3810000000000000, !dbg !4252
  %_4.i.i.us.us.i417 = select i1 %_3.i.i.us.us.i416, i32 8388608, i32 %_4.i.i498.us.us.i, !dbg !4257
  %_5.i280.us.us.i = and i32 %_4.i.i.us.us.i417, 8388607, !dbg !4259
  %_4.i281.us.us.i = or disjoint i32 %_5.i280.us.us.i, 1065353216, !dbg !4259
  %significand.i.us.us.i418 = bitcast i32 %_4.i281.us.us.i to float, !dbg !4261
  %_0.i246.us.us.i419 = fadd float %significand.i.us.us.i418, -1.000000e+00, !dbg !4263
  %_0.i225.us.us.i = fmul float %_0.i246.us.us.i419, 0x3F9B17A960000000, !dbg !4265
  %495 = fsub float 0x3FBF9A8440000000, %_0.i225.us.us.i, !dbg !4267
  %_0.i225.us.us.1.i = fmul float %_0.i246.us.us.i419, %495, !dbg !4265
  %_0.i211.us.us.1.i = fadd float %_0.i225.us.us.1.i, 0xBFD1E3F400000000, !dbg !4267
  %_0.i225.us.us.2.i = fmul float %_0.i246.us.us.i419, %_0.i211.us.us.1.i, !dbg !4265
  %_0.i211.us.us.2.i = fadd float %_0.i225.us.us.2.i, 0x3FDD544F20000000, !dbg !4267
  %_0.i225.us.us.3.i = fmul float %_0.i246.us.us.i419, %_0.i211.us.us.2.i, !dbg !4265
  %_0.i211.us.us.3.i = fadd float %_0.i225.us.us.3.i, 0xBFE6FC2A60000000, !dbg !4267
  %_0.i225.us.us.4.i = fmul float %_0.i246.us.us.i419, %_0.i211.us.us.3.i, !dbg !4265
  %_0.i211.us.us.4.i = fadd float %_0.i225.us.us.4.i, 0x3FF714B2A0000000, !dbg !4267
  %_9.i.us.us.i420 = lshr i32 %_4.i.i.us.us.i417, 23, !dbg !4269
  %_8.i282.us.us.i = or disjoint i32 %_9.i.us.us.i420, 1258291200, !dbg !4269
  %_7.i.us.us.i421 = bitcast i32 %_8.i282.us.us.i to float, !dbg !4270
  %exponent.i.us.us.i422 = fadd float %_7.i.us.us.i421, 0xC160000FE0000000, !dbg !4272
  %_0.i224.us.us.i = fmul float %_0.i246.us.us.i419, %_0.i211.us.us.4.i, !dbg !4273
  %_0.i210.us.us.i = fadd float %exponent.i.us.us.i422, %_0.i224.us.us.i, !dbg !4275
  %_0.i243.us.us.i = fmul float %_0.i210.us.us.i, 0x4018151820000000, !dbg !4277
  %_3.i.i549.us.us.inv.i = fcmp olt float %_0.i243.us.us.i, 2.400000e+01, !dbg !4279
  %_0.i.i556.us.us.i = select i1 %_3.i.i549.us.us.inv.i, float %_0.i243.us.us.i, float 2.400000e+01, !dbg !4279
  %_3.i.i484.us.us.inv.i = fcmp ogt float %_0.i.i556.us.us.i, -1.600000e+02, !dbg !4282
  %_0.i.i491.us.us.i = select i1 %_3.i.i484.us.us.inv.i, float %_0.i.i556.us.us.i, float -1.600000e+02, !dbg !4282
  %_55.i11.us.us.i = load float, ptr %371, align 4, !dbg !4285, !alias.scope !4286, !noalias !4289, !noundef !12
  %_3.i200.us.us.i423 = fcmp ule float %_55.i11.us.us.i, 0.000000e+00, !dbg !4291
  %_3.i176.us.us.i = fcmp oge float %_0.i.i491.us.us.i, %threshold.i.i, !dbg !4293
  %_3.i174.us.us.i = fcmp oge float %_0.i.i491.us.us.i, %_0.i259.i, !dbg !4295
  %..i175.us.us.i = sext i1 %_3.i174.us.us.i to i32, !dbg !4297
  %_0.i412.us.us.i = sext i1 %_3.i176.us.us.i to i32, !dbg !4299
  %_0.i405.us.us.i = select i1 %_3.i200.us.us.i423, i32 %_0.i412.us.us.i, i32 %..i175.us.us.i, !dbg !4299
  %_0.i416.us.us.i = xor i32 %..i175.us.us.i, -1, !dbg !4301
  %_3.i198.us.us.i424 = fcmp ogt float %_67.i131005.us.us.i, 0.000000e+00, !dbg !4303
  %_0.i411.us.us.i = select i1 %_3.i198.us.us.i424, i32 %_0.i416.us.us.i, i32 0, !dbg !4305
  %_0.i410.us.us.i = select i1 %_3.i200.us.us.i423, i32 0, i32 %_0.i411.us.us.i, !dbg !4307
  %_0.i404.us.us.i = or i32 %_0.i410.us.us.i, %_0.i405.us.us.i, !dbg !4309
  %_5.i381.us.us.i = and i32 %_0.i404.us.us.i, 1065353216, !dbg !4311
  %_0.i385.us.us.i = bitcast i32 %_5.i381.us.us.i to float, !dbg !4313
  %_0.i258.us.us.i = fadd float %_67.i131005.us.us.i, -1.000000e+00, !dbg !4315
  %496 = trunc nsw i32 %_0.i410.us.us.i to i1, !dbg !4317
  %_4.i379.v.us.us.i = select i1 %496, float %_0.i258.us.us.i, float %_67.i131005.us.us.i, !dbg !4317
  %497 = trunc nsw i32 %_0.i405.us.us.i to i1, !dbg !4319
  %_0.i373.us.us.i425 = select i1 %497, float %_71.i585586.i, float %_4.i379.v.us.us.i, !dbg !4319
  store float %_0.i373.us.us.i425, ptr %372, align 4, !dbg !4321, !alias.scope !4286, !noalias !4289
  store i32 %_5.i381.us.us.i, ptr %371, align 4, !dbg !4322, !alias.scope !4286, !noalias !4289
  %_0.i256.us.us.i426 = fsub float %_0.i.i491.us.us.i, %threshold.i.i, !dbg !4323
  %_0.i242.us.us.i = fmul float %_0.i257.i, %_0.i256.us.us.i426, !dbg !4325
  %_3.i.i475.inv.us.us.i = fcmp ogt float %_0.i242.us.us.i, %374, !dbg !4327
  %_4.i.i482.v.us.us.i = select i1 %_3.i.i475.inv.us.us.i, float %_0.i242.us.us.i, float %374, !dbg !4327
  %_3.i.i541.us.us.i = fcmp olt float %_4.i.i482.v.us.us.i, 0.000000e+00, !dbg !4330
  %498 = fcmp ule float %_0.i385.us.us.i, 0.000000e+00, !dbg !4333
  %499 = select i1 %498, i1 %_3.i.i541.us.us.i, i1 false, !dbg !4335
  %_0.i366.us.us.i = select i1 %499, float %_4.i.i482.v.us.us.i, float 0.000000e+00, !dbg !4335
  %_3.i194.us.us.i = fcmp ule float %_0.i366.us.us.i, %_86.i211007.us.us.i, !dbg !4336
  %_4.i360.us.us.i = select i1 %_3.i194.us.us.i, i32 %_88.i24589.i, i32 %_87.i23588.i, !dbg !4338
  %_0.i.us.us.i427 = bitcast i32 %_4.i360.us.us.i to float, !dbg !4340
  %_0.i255.us.us.i428 = fsub float %_0.i366.us.us.i, %_86.i211007.us.us.i, !dbg !4342
  %_4.i222.us.us.i = fmul float %_0.i255.us.us.i428, %_0.i.us.us.i427, !dbg !4344
  %_0.i223.us.us.i = fadd float %_86.i211007.us.us.i, %_4.i222.us.us.i, !dbg !4344
  %500 = tail call noundef float @llvm.fabs.f32(float %_0.i223.us.us.i), !dbg !4346
  %501 = fcmp uge float %500, 0x3BC79CA100000000, !dbg !4349
  %_0.i296.us.us.i429 = select i1 %501, float %_0.i223.us.us.i, float 0.000000e+00, !dbg !4351
  store float %_0.i296.us.us.i429, ptr %375, align 4, !dbg !4352, !alias.scope !4286, !noalias !4289
  %_0.i241.us.us.i = fmul float %_0.i296.us.us.i429, 0x3FC542A5A0000000, !dbg !4353
  %_3.i.i426.us.us.inv.i = fcmp ogt float %_0.i241.us.us.i, -1.260000e+02, !dbg !4356
  %_0.i.i433.us.us.i = select i1 %_3.i.i426.us.us.inv.i, float %_0.i241.us.us.i, float -1.260000e+02, !dbg !4356
  %_3.i.i509.us.us.inv.i = fcmp olt float %_0.i.i433.us.us.i, 1.270000e+02, !dbg !4360
  %_0.i.i516.us.us.i = select i1 %_3.i.i509.us.us.inv.i, float %_0.i.i433.us.us.i, float 1.270000e+02, !dbg !4360
  %502 = tail call noundef float @llvm.floor.f32(float %_0.i.i516.us.us.i), !dbg !4363
  %_0.i248.us.us.i430 = fsub float %_0.i.i516.us.us.i, %502, !dbg !4367
  %_0.i230.us.us.i = fmul float %_0.i248.us.us.i430, 0x3F5E974FA0000000, !dbg !4369
  %_0.i215.us.us.i = fadd float %_0.i230.us.us.i, 0x3F82778560000000, !dbg !4371
  %_0.i230.us.us.1.i = fmul float %_0.i248.us.us.i430, %_0.i215.us.us.i, !dbg !4369
  %_0.i215.us.us.1.i = fadd float %_0.i230.us.us.1.i, 0x3FAC91CE60000000, !dbg !4371
  %_0.i230.us.us.2.i = fmul float %_0.i248.us.us.i430, %_0.i215.us.us.1.i, !dbg !4369
  %_0.i215.us.us.2.i = fadd float %_0.i230.us.us.2.i, 0x3FCEBDB560000000, !dbg !4371
  %_0.i230.us.us.3.i = fmul float %_0.i248.us.us.i430, %_0.i215.us.us.2.i, !dbg !4369
  %_0.i215.us.us.3.i = fadd float %_0.i230.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4371
  %_0.i233.us.us.i = fmul float %_0.i249.us.us.i415, 0x3F5E974FA0000000, !dbg !4373
  %_0.i217.us.us.i = fadd float %_0.i233.us.us.i, 0x3F82778560000000, !dbg !4375
  %_0.i233.us.us.1.i = fmul float %_0.i249.us.us.i415, %_0.i217.us.us.i, !dbg !4373
  %_0.i217.us.us.1.i = fadd float %_0.i233.us.us.1.i, 0x3FAC91CE60000000, !dbg !4375
  %_0.i233.us.us.2.i = fmul float %_0.i249.us.us.i415, %_0.i217.us.us.1.i, !dbg !4373
  %_0.i217.us.us.2.i = fadd float %_0.i233.us.us.2.i, 0x3FCEBDB560000000, !dbg !4375
  %_0.i233.us.us.3.i = fmul float %_0.i249.us.us.i415, %_0.i217.us.us.2.i, !dbg !4373
  %_0.i217.us.us.3.i = fadd float %_0.i233.us.us.3.i, 0x3FE62E4BA0000000, !dbg !4375
  %_0.i232.us.us.i = fmul float %_0.i249.us.us.i415, %_0.i217.us.us.3.i, !dbg !4377
  %_0.i216.us.us.i = fadd float %_0.i232.us.us.i, 1.000000e+00, !dbg !4379
  %biased.i163.us.us.i = fadd float %492, 0x4160000FE0000000, !dbg !4381
  %_4.i164.us.us.i = bitcast float %biased.i163.us.us.i to i32, !dbg !4383
  %_3.i165.us.us.i = shl i32 %_4.i164.us.us.i, 23, !dbg !4385
  %_0.i166.us.us.i = bitcast i32 %_3.i165.us.us.i to float, !dbg !4386
  %_0.i231.us.us.i = fmul float %_0.i216.us.us.i, %_0.i166.us.us.i, !dbg !4388
  %_3.i167.us.us.i = fcmp une float %_0.i292.us.us.i414, 0.000000e+00, !dbg !4390
  %_0.i400576.not.us.us.i = and i1 %_3.i178.i, %_3.i167.us.us.i, !dbg !4393
  %_0.i234.us.us.i = fmul float %_0.i271.us.us.i387, %_0.i231.us.us.i, !dbg !4393
  %_4.i300.v.us.us.i = select i1 %_0.i400576.not.us.us.i, float %_0.i234.us.us.i, float %_0.i271.us.us.i387, !dbg !4396
  %_0.i229.us.us.i = fmul float %_0.i248.us.us.i430, %_0.i215.us.us.3.i, !dbg !4398
  %_0.i214.us.us.i = fadd float %_0.i229.us.us.i, 1.000000e+00, !dbg !4400
  %biased.i.us.us.i431 = fadd float %502, 0x4160000FE0000000, !dbg !4402
  %_4.i160.us.us.i = bitcast float %biased.i.us.us.i431 to i32, !dbg !4404
  %_3.i161.us.us.i = shl i32 %_4.i160.us.us.i, 23, !dbg !4406
  %_0.i162.us.us.i = bitcast i32 %_3.i161.us.us.i to float, !dbg !4407
  %_0.i228.us.us.i = fmul float %_0.i214.us.us.i, %_0.i162.us.us.i, !dbg !4409
  %_3.i168.us.us.i = fcmp une float %_0.i296.us.us.i429, 0.000000e+00, !dbg !4411
  %_0.i403593.not.us.us.i = and i1 %_3.i192.i, %_3.i168.us.us.i, !dbg !4413
  %_0.i240.us.us.i = fmul float %_0.i269.us.us.i391, %_0.i228.us.us.i, !dbg !4413
  %_4.i353.v.us.us.i = select i1 %_0.i403593.not.us.us.i, float %_0.i240.us.us.i, float %_0.i269.us.us.i391, !dbg !4415
  store float %_4.i300.v.us.us.i, ptr %_123.i.us.us.i361, align 4, !dbg !4417, !alias.scope !4420, !noalias !3914
  store float %_4.i353.v.us.us.i, ptr %_141.i.us.us.i366, align 4, !dbg !4423, !alias.scope !4425, !noalias !3936
  %exitcond2537.not.i = icmp eq i64 %478, %_26, !dbg !4428
  br i1 %exitcond2537.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb42.i.us.us.i356, !dbg !4431, !llvm.loop !4439

bb40.i.us.i433:                                   ; preds = %bb40.i.lr.ph.split.us.i, %bb28.i.us.us.lr.ph.us.i487
  %_86.i211007.us.i = phi float [ %_0.i296.us.i510, %bb28.i.us.us.lr.ph.us.i487 ], [ %.promoted1006.i, %bb40.i.lr.ph.split.us.i ]
  %_67.i131005.us.i = phi float [ %_0.i373.us.i506, %bb28.i.us.us.lr.ph.us.i487 ], [ %.promoted1004.i, %bb40.i.lr.ph.split.us.i ]
  %_86.i881003.us.i = phi float [ %_0.i292.us.i495, %bb28.i.us.us.lr.ph.us.i487 ], [ %.promoted1002.i, %bb40.i.lr.ph.split.us.i ]
  %_67.i691001.us.i = phi float [ %_0.i321.us.i, %bb28.i.us.us.lr.ph.us.i487 ], [ %.promoted1000.i, %bb40.i.lr.ph.split.us.i ]
  %iter.sroa.0.0.i966.us.i = phi i64 [ %503, %bb28.i.us.us.lr.ph.us.i487 ], [ 0, %bb40.i.lr.ph.split.us.i ]
  %503 = add nuw nsw i64 %iter.sroa.0.0.i966.us.i, 1, !dbg !3871
  %_27.i.us.i434 = trunc i64 %iter.sroa.0.0.i966.us.i to i32, !dbg !3885
  %now.i.us.i435 = add i32 %base.i.i46, %_27.i.us.i434, !dbg !3888
  %_30.i.us.i436 = and i32 %now.i.us.i435, %_52.i30, !dbg !3891
  %_29.i.us.i437 = zext i32 %_30.i.us.i436 to i64, !dbg !3893
  %exitcond2538.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.i, %_55, !dbg !3894
  br i1 %exitcond2538.not.i, label %bb43.i.i127, label %bb42.i.us.i438, !dbg !3894, !prof !180

bb42.i.us.i438:                                   ; preds = %bb40.i.us.i433
  %_123.i.us.i439 = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i966.us.i, !dbg !3900
  %_124.not.not.i.us.i440 = icmp ugt i64 %_58.1.i20, %_29.i.us.i437, !dbg !3904
  br i1 %_124.not.not.i.us.i440, label %bb45.i.us.i441, label %bb46.i.i58, !dbg !3904, !prof !2704

bb45.i.us.i441:                                   ; preds = %bb42.i.us.i438
  %_0.i279.us.i442 = load float, ptr %_123.i.us.i439, align 4, !dbg !3909, !alias.scope !3911, !noalias !3914, !noundef !12
  %_133.i.us.i443 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_29.i.us.i437, !dbg !3915
  store float %_0.i279.us.i442, ptr %_133.i.us.i443, align 4, !dbg !3919, !alias.scope !3921, !noalias !3868
  %_141.i.us.i444 = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i966.us.i, !dbg !3924
  %_142.not.not.i.us.i445 = icmp ugt i64 %_60.1.i25, %_29.i.us.i437, !dbg !4440
  br i1 %_142.not.not.i.us.i445, label %bb50.i.us.i447, label %bb51.i.i446, !dbg !4440, !prof !2704

bb50.i.us.i447:                                   ; preds = %bb45.i.us.i441
  %_0.i277.us.i448 = load float, ptr %_141.i.us.i444, align 4, !dbg !3931, !alias.scope !3933, !noalias !3936, !noundef !12
  %_149.i.us.i449 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_29.i.us.i437, !dbg !3937
  store float %_0.i277.us.i448, ptr %_149.i.us.i449, align 4, !dbg !3944, !alias.scope !3946, !noalias !3868
  %exitcond2539.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.i, %side_left.sroa.5.0.i.i45, !dbg !4438
  br i1 %exitcond2539.not.i, label %bb53.i.i432, label %bb52.i.us.i450, !dbg !4438, !prof !180

bb52.i.us.i450:                                   ; preds = %bb50.i.us.i447
  %_158.not.not.i.us.i451 = icmp ugt i64 %_59.1.i22, %_29.i.us.i437, !dbg !4436
  br i1 %_158.not.not.i.us.i451, label %bb54.i.us.i452, label %bb55.i.i294, !dbg !4436, !prof !2704

bb54.i.us.i452:                                   ; preds = %bb52.i.us.i450
  %_157.i.us.i453 = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i44, i64 %iter.sroa.0.0.i966.us.i, !dbg !3949
  %_0.i275.us.i454 = load float, ptr %_157.i.us.i453, align 4, !dbg !3956, !alias.scope !3958, !noalias !3868, !noundef !12
  %_165.i.us.i455 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_29.i.us.i437, !dbg !3961
  store float %_0.i275.us.i454, ptr %_165.i.us.i455, align 4, !dbg !3968, !alias.scope !3970, !noalias !3868
  %exitcond2540.not.i = icmp eq i64 %iter.sroa.0.0.i966.us.i, %empty.sroa.6.0.i.i42, !dbg !4434
  br i1 %exitcond2540.not.i, label %bb57.i.i278, label %bb56.i.us.i456, !dbg !4434, !prof !180

bb56.i.us.i456:                                   ; preds = %bb54.i.us.i452
  %_174.not.not.i.us.i457 = icmp ugt i64 %_61.1.i27, %_29.i.us.i437, !dbg !4432
  br i1 %_174.not.not.i.us.i457, label %bb58.i.us.i458, label %bb59.i.i146, !dbg !4432, !prof !2704

bb58.i.us.i458:                                   ; preds = %bb56.i.us.i456
  %_173.i.us.i459 = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i43, i64 %iter.sroa.0.0.i966.us.i, !dbg !3973
  %_0.i273.us.i460 = load float, ptr %_173.i.us.i459, align 4, !dbg !3980, !alias.scope !3982, !noalias !3868, !noundef !12
  %_181.i.us.i461 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_29.i.us.i437, !dbg !3985
  store float %_0.i273.us.i460, ptr %_181.i.us.i461, align 4, !dbg !3992, !alias.scope !3994, !noalias !3868
  %_52.i.us.i462 = sub i32 %now.i.us.i435, %_53.i31, !dbg !3997
  %_51.i.us.i463 = and i32 %_52.i.us.i462, %_52.i30, !dbg !4000
  %_50.i.us.i464 = zext i32 %_51.i.us.i463 to i64, !dbg !4001
  %_182.not.not.i.us.i465 = icmp ugt i64 %_58.1.i20, %_50.i.us.i464, !dbg !4002
  br i1 %_182.not.not.i.us.i465, label %bb60.i.us.i466, label %bb61.i.i75, !dbg !4002, !prof !2704

bb60.i.us.i466:                                   ; preds = %bb58.i.us.i458
  %_189.i.us.i467 = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_50.i.us.i464, !dbg !4007
  %_0.i271.us.i468 = load float, ptr %_189.i.us.i467, align 4, !dbg !4011, !alias.scope !4013, !noalias !3868, !noundef !12
  %_190.not.not.i.us.i469 = icmp ugt i64 %_60.1.i25, %_50.i.us.i464, !dbg !4441
  br i1 %_190.not.not.i.us.i469, label %bb63.i.us.i470, label %bb64.i.i160, !dbg !4441, !prof !2704

bb63.i.us.i470:                                   ; preds = %bb60.i.us.i466
  %_195.i.us.i471 = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_50.i.us.i464, !dbg !4016
  %_0.i269.us.i472 = load float, ptr %_195.i.us.i471, align 4, !dbg !4024, !alias.scope !4026, !noalias !3868, !noundef !12
  %_66.i.us.i473 = sub i32 %now.i.us.i435, %_67.i.i
  %_65.i.us.i474 = and i32 %_66.i.us.i473, %_52.i30
  %_64.i.us.i475 = zext i32 %_65.i.us.i474 to i64
  %_74.i.us.i476 = sub i32 %now.i.us.i435, %_75.i.i
  %_73.i.us.i477 = and i32 %_74.i.us.i476, %_52.i30
  %_72.i.us.i478 = zext i32 %_73.i.us.i477 to i64
  %_80.i.us.i479 = icmp ugt i64 %_59.1.i22, %_64.i.us.i475
  %504 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_64.i.us.i475
  %505 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_64.i.us.i475
  %_86.i.us.i480 = icmp ugt i64 %_61.1.i27, %_72.i.us.i478
  %506 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_72.i.us.i478
  %_88.i.us.i481 = icmp ugt i64 %_59.1.i22, %_72.i.us.i478
  %507 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_72.i.us.i478
  br i1 %_80.i.us.i479, label %bb63.i.split.us.us.i482, label %bb63.i.split.i90

bb63.i.split.us.us.i482:                          ; preds = %bb63.i.us.i470
  %_84.i.us.i483 = icmp ugt i64 %_61.1.i27, %_64.i.us.i475
  br i1 %_84.i.us.i483, label %bb24.i.us.lr.ph.us.i484, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i93, !dbg !4029

bb24.i.us.lr.ph.us.i484:                          ; preds = %bb63.i.split.us.us.i482
  %_82.i.us.us.i485 = load float, ptr %505, align 4, !noalias !3868, !noundef !12
  br i1 %_86.i.us.i480, label %bb24.i.us.lr.ph.split.us.us.i486, label %bb24.i.us.lr.ph.split.i96

bb24.i.us.lr.ph.split.us.us.i486:                 ; preds = %bb24.i.us.lr.ph.us.i484
  br i1 %_88.i.us.i481, label %bb28.i.us.us.lr.ph.us.i487, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i98, !dbg !4037

bb28.i.us.us.lr.ph.us.i487:                       ; preds = %bb24.i.us.lr.ph.split.us.us.i486
  %_87.i.us.us.us.i488 = load float, ptr %507, align 4, !noalias !3868, !noundef !12
  %_85.i.us.us.le827.us.i = load float, ptr %506, align 4, !noalias !3868, !noundef !12
  %_78.i.us.le.us.i489 = load float, ptr %504, align 4, !noalias !3868, !noundef !12
  %508 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.i489), !dbg !4038
  %509 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.i485), !dbg !4048
  %_3.i.i466.us.i = fcmp ule float %508, %509, !dbg !4051
  %_6.i.i468.us.i = bitcast float %508 to i32, !dbg !4056
  %_8.i.i470.us.i = bitcast float %509 to i32, !dbg !4059
  %_4.i.i473.us.i = select i1 %_3.i.i466.us.i, i32 %_8.i.i470.us.i, i32 %_6.i.i468.us.i, !dbg !4061
  %_4.i346.us.i = select i1 %_3.i190.i, i32 %_6.i.i468.us.i, i32 %_4.i.i473.us.i, !dbg !4062
  %_0.i239.us.i = fmul float %508, 5.000000e-01, !dbg !4064
  %_0.i238.us.i = fmul float %509, 5.000000e-01, !dbg !4066
  %_0.i218.us.i = fadd float %_0.i238.us.i, %_0.i239.us.i, !dbg !4068
  %_6.i334.us.i = bitcast float %_0.i218.us.i to i32, !dbg !4070
  %_4.i339.us.i = select i1 %_3.i188.i, i32 %_4.i346.us.i, i32 %_6.i334.us.i, !dbg !4073
  %_0.i340.us.i = bitcast i32 %_4.i339.us.i to float, !dbg !4074
  %_3.i.i458.us.i = fcmp ule float %_0.i340.us.i, 0x3E45798EE0000000, !dbg !4076
  %_4.i.i464.us.i = select i1 %_3.i.i458.us.i, i32 841731191, i32 %_4.i339.us.i, !dbg !4079
  %_0.i.i465.us.i = bitcast i32 %_4.i.i464.us.i to float, !dbg !4081
  %_3.i.i418.us.i = fcmp ule float %_0.i.i465.us.i, 0x3810000000000000, !dbg !4083
  %_4.i.i424.us.i = select i1 %_3.i.i418.us.i, i32 8388608, i32 %_4.i.i464.us.i, !dbg !4089
  %_5.i284.us.i = and i32 %_4.i.i424.us.i, 8388607, !dbg !4091
  %_4.i285.us.i = or disjoint i32 %_5.i284.us.i, 1065353216, !dbg !4091
  %significand.i286.us.i = bitcast i32 %_4.i285.us.i to float, !dbg !4093
  %_0.i247.us.i490 = fadd float %significand.i286.us.i, -1.000000e+00, !dbg !4095
  %_0.i227.us.i = fmul float %_0.i247.us.i490, 0x3F9B17A960000000, !dbg !4097
  %510 = fsub float 0x3FBF9A8440000000, %_0.i227.us.i, !dbg !4099
  %_0.i227.us.1.i = fmul float %_0.i247.us.i490, %510, !dbg !4097
  %_0.i213.us.1.i = fadd float %_0.i227.us.1.i, 0xBFD1E3F400000000, !dbg !4099
  %_0.i227.us.2.i = fmul float %_0.i247.us.i490, %_0.i213.us.1.i, !dbg !4097
  %_0.i213.us.2.i = fadd float %_0.i227.us.2.i, 0x3FDD544F20000000, !dbg !4099
  %_0.i227.us.3.i = fmul float %_0.i247.us.i490, %_0.i213.us.2.i, !dbg !4097
  %_0.i213.us.3.i = fadd float %_0.i227.us.3.i, 0xBFE6FC2A60000000, !dbg !4099
  %_0.i227.us.4.i = fmul float %_0.i247.us.i490, %_0.i213.us.3.i, !dbg !4097
  %_0.i213.us.4.i = fadd float %_0.i227.us.4.i, 0x3FF714B2A0000000, !dbg !4099
  %_9.i287.us.i = lshr i32 %_4.i.i424.us.i, 23, !dbg !4101
  %_8.i288.us.i = or disjoint i32 %_9.i287.us.i, 1258291200, !dbg !4101
  %_7.i289.us.i = bitcast i32 %_8.i288.us.i to float, !dbg !4102
  %exponent.i290.us.i = fadd float %_7.i289.us.i, 0xC160000FE0000000, !dbg !4104
  %_0.i226.us.i = fmul float %_0.i247.us.i490, %_0.i213.us.4.i, !dbg !4105
  %_0.i212.us.i = fadd float %exponent.i290.us.i, %_0.i226.us.i, !dbg !4107
  %_0.i237.us.i = fmul float %_0.i212.us.i, 0x4018151820000000, !dbg !4109
  %_3.i.i533.us.inv.i = fcmp olt float %_0.i237.us.i, 2.400000e+01, !dbg !4111
  %_0.i.i540.us.i = select i1 %_3.i.i533.us.inv.i, float %_0.i237.us.i, float 2.400000e+01, !dbg !4111
  %_3.i.i450.us.inv.i = fcmp ogt float %_0.i.i540.us.i, -1.600000e+02, !dbg !4114
  %_0.i.i457.us.i = select i1 %_3.i.i450.us.inv.i, float %_0.i.i540.us.i, float -1.600000e+02, !dbg !4114
  %_55.i59.us.i = load float, ptr %359, align 4, !dbg !4117, !alias.scope !4119, !noalias !4122, !noundef !12
  %_3.i186.us.i = fcmp ule float %_55.i59.us.i, 0.000000e+00, !dbg !4124
  %_3.i172.us.i = fcmp oge float %_0.i.i457.us.i, %threshold.i32.i, !dbg !4126
  %_3.i170.us.i = fcmp oge float %_0.i.i457.us.i, %_0.i254.i, !dbg !4129
  %..i171.us.i = sext i1 %_3.i170.us.i to i32, !dbg !4132
  %_0.i408.us.i = sext i1 %_3.i172.us.i to i32, !dbg !4134
  %_0.i402.us.i = select i1 %_3.i186.us.i, i32 %_0.i408.us.i, i32 %..i171.us.i, !dbg !4134
  %_0.i414.us.i = xor i32 %..i171.us.i, -1, !dbg !4139
  %_3.i184.us.i = fcmp ogt float %_67.i691001.us.i, 0.000000e+00, !dbg !4142
  %_0.i407.us.i491 = select i1 %_3.i184.us.i, i32 %_0.i414.us.i, i32 0, !dbg !4144
  %_0.i406.us.i = select i1 %_3.i186.us.i, i32 0, i32 %_0.i407.us.i491, !dbg !4146
  %_0.i401.us.i = or i32 %_0.i406.us.i, %_0.i402.us.i, !dbg !4148
  %_5.i329.us.i = and i32 %_0.i401.us.i, 1065353216, !dbg !4151
  %_0.i333.us.i = bitcast i32 %_5.i329.us.i to float, !dbg !4153
  %_0.i253.us.i492 = fadd float %_67.i691001.us.i, -1.000000e+00, !dbg !4155
  %511 = trunc nsw i32 %_0.i406.us.i to i1, !dbg !4158
  %_4.i327.v.us.i = select i1 %511, float %_0.i253.us.i492, float %_67.i691001.us.i, !dbg !4158
  %512 = trunc nsw i32 %_0.i402.us.i to i1, !dbg !4160
  %_0.i321.us.i = select i1 %512, float %_71.i75568569.i, float %_4.i327.v.us.i, !dbg !4160
  store float %_0.i321.us.i, ptr %360, align 4, !dbg !4162, !alias.scope !4119, !noalias !4122
  store i32 %_5.i329.us.i, ptr %359, align 4, !dbg !4163, !alias.scope !4119, !noalias !4122
  %_0.i251.us.i493 = fsub float %_0.i.i457.us.i, %threshold.i32.i, !dbg !4164
  %_0.i236.us.i = fmul float %_0.i252.i, %_0.i251.us.i493, !dbg !4166
  %_3.i.i442.inv.us.i = fcmp ogt float %_0.i236.us.i, %362, !dbg !4168
  %_4.i.i448.v.us.i = select i1 %_3.i.i442.inv.us.i, float %_0.i236.us.i, float %362, !dbg !4168
  %_3.i.i525.us.i = fcmp olt float %_4.i.i448.v.us.i, 0.000000e+00, !dbg !4171
  %513 = fcmp ule float %_0.i333.us.i, 0.000000e+00, !dbg !4174
  %514 = select i1 %513, i1 %_3.i.i525.us.i, i1 false, !dbg !4177
  %_0.i314.us.i = select i1 %514, float %_4.i.i448.v.us.i, float 0.000000e+00, !dbg !4177
  %_3.i180.us.i = fcmp ule float %_0.i314.us.i, %_86.i881003.us.i, !dbg !4178
  %_4.i307.us.i = select i1 %_3.i180.us.i, i32 %_88.i91572.i, i32 %_87.i90571.i, !dbg !4181
  %_0.i308.us.i = bitcast i32 %_4.i307.us.i to float, !dbg !4183
  %_0.i250.us.i494 = fsub float %_0.i314.us.i, %_86.i881003.us.i, !dbg !4185
  %_4.i220.us.i = fmul float %_0.i250.us.i494, %_0.i308.us.i, !dbg !4188
  %_0.i221.us.i = fadd float %_86.i881003.us.i, %_4.i220.us.i, !dbg !4188
  %515 = tail call noundef float @llvm.fabs.f32(float %_0.i221.us.i), !dbg !4190
  %516 = fcmp uge float %515, 0x3BC79CA100000000, !dbg !4194
  %_0.i292.us.i495 = select i1 %516, float %_0.i221.us.i, float 0.000000e+00, !dbg !4196
  store float %_0.i292.us.i495, ptr %363, align 4, !dbg !4197, !alias.scope !4119, !noalias !4122
  %_0.i235.us.i = fmul float %_0.i292.us.i495, 0x3FC542A5A0000000, !dbg !4199
  %_3.i.i434.us.inv.i = fcmp ogt float %_0.i235.us.i, -1.260000e+02, !dbg !4203
  %_0.i.i441.us.i = select i1 %_3.i.i434.us.inv.i, float %_0.i235.us.i, float -1.260000e+02, !dbg !4203
  %_3.i.i517.us.inv.i = fcmp olt float %_0.i.i441.us.i, 1.270000e+02, !dbg !4207
  %_0.i.i524.us.i = select i1 %_3.i.i517.us.inv.i, float %_0.i.i441.us.i, float 1.270000e+02, !dbg !4207
  %517 = tail call noundef float @llvm.floor.f32(float %_0.i.i524.us.i), !dbg !4210
  %_0.i249.us.i496 = fsub float %_0.i.i524.us.i, %517, !dbg !4214
  %518 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le827.us.i), !dbg !4216
  %519 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.i488), !dbg !4220
  %_3.i.i500.us.i = fcmp ule float %518, %519, !dbg !4222
  %_6.i.i502.us.i = bitcast float %518 to i32, !dbg !4225
  %_8.i.i504.us.i = bitcast float %519 to i32, !dbg !4228
  %_4.i.i507.us.i = select i1 %_3.i.i500.us.i, i32 %_8.i.i504.us.i, i32 %_6.i.i502.us.i, !dbg !4230
  %_4.i398.us.i = select i1 %_3.i204.i, i32 %_6.i.i502.us.i, i32 %_4.i.i507.us.i, !dbg !4231
  %_0.i245.us.i = fmul float %518, 5.000000e-01, !dbg !4233
  %_0.i244.us.i = fmul float %519, 5.000000e-01, !dbg !4235
  %_0.i219.us.i = fadd float %_0.i244.us.i, %_0.i245.us.i, !dbg !4237
  %_6.i386.us.i = bitcast float %_0.i219.us.i to i32, !dbg !4239
  %_4.i391.us.i = select i1 %_3.i202.i, i32 %_4.i398.us.i, i32 %_6.i386.us.i, !dbg !4242
  %_0.i392.us.i = bitcast i32 %_4.i391.us.i to float, !dbg !4243
  %_3.i.i492.us.i = fcmp ule float %_0.i392.us.i, 0x3E45798EE0000000, !dbg !4245
  %_4.i.i498.us.i = select i1 %_3.i.i492.us.i, i32 841731191, i32 %_4.i391.us.i, !dbg !4248
  %_0.i.i499.us.i = bitcast i32 %_4.i.i498.us.i to float, !dbg !4250
  %_3.i.i.us.i497 = fcmp ule float %_0.i.i499.us.i, 0x3810000000000000, !dbg !4252
  %_4.i.i.us.i498 = select i1 %_3.i.i.us.i497, i32 8388608, i32 %_4.i.i498.us.i, !dbg !4257
  %_5.i280.us.i = and i32 %_4.i.i.us.i498, 8388607, !dbg !4259
  %_4.i281.us.i = or disjoint i32 %_5.i280.us.i, 1065353216, !dbg !4259
  %significand.i.us.i499 = bitcast i32 %_4.i281.us.i to float, !dbg !4261
  %_0.i246.us.i500 = fadd float %significand.i.us.i499, -1.000000e+00, !dbg !4263
  %_0.i225.us.i = fmul float %_0.i246.us.i500, 0x3F9B17A960000000, !dbg !4265
  %520 = fsub float 0x3FBF9A8440000000, %_0.i225.us.i, !dbg !4267
  %_0.i225.us.1.i = fmul float %_0.i246.us.i500, %520, !dbg !4265
  %_0.i211.us.1.i = fadd float %_0.i225.us.1.i, 0xBFD1E3F400000000, !dbg !4267
  %_0.i225.us.2.i = fmul float %_0.i246.us.i500, %_0.i211.us.1.i, !dbg !4265
  %_0.i211.us.2.i = fadd float %_0.i225.us.2.i, 0x3FDD544F20000000, !dbg !4267
  %_0.i225.us.3.i = fmul float %_0.i246.us.i500, %_0.i211.us.2.i, !dbg !4265
  %_0.i211.us.3.i = fadd float %_0.i225.us.3.i, 0xBFE6FC2A60000000, !dbg !4267
  %_0.i225.us.4.i = fmul float %_0.i246.us.i500, %_0.i211.us.3.i, !dbg !4265
  %_0.i211.us.4.i = fadd float %_0.i225.us.4.i, 0x3FF714B2A0000000, !dbg !4267
  %_9.i.us.i501 = lshr i32 %_4.i.i.us.i498, 23, !dbg !4269
  %_8.i282.us.i = or disjoint i32 %_9.i.us.i501, 1258291200, !dbg !4269
  %_7.i.us.i502 = bitcast i32 %_8.i282.us.i to float, !dbg !4270
  %exponent.i.us.i503 = fadd float %_7.i.us.i502, 0xC160000FE0000000, !dbg !4272
  %_0.i224.us.i = fmul float %_0.i246.us.i500, %_0.i211.us.4.i, !dbg !4273
  %_0.i210.us.i = fadd float %exponent.i.us.i503, %_0.i224.us.i, !dbg !4275
  %_0.i243.us.i = fmul float %_0.i210.us.i, 0x4018151820000000, !dbg !4277
  %_3.i.i549.us.inv.i = fcmp olt float %_0.i243.us.i, 2.400000e+01, !dbg !4279
  %_0.i.i556.us.i = select i1 %_3.i.i549.us.inv.i, float %_0.i243.us.i, float 2.400000e+01, !dbg !4279
  %_3.i.i484.us.inv.i = fcmp ogt float %_0.i.i556.us.i, -1.600000e+02, !dbg !4282
  %_0.i.i491.us.i = select i1 %_3.i.i484.us.inv.i, float %_0.i.i556.us.i, float -1.600000e+02, !dbg !4282
  %_55.i11.us.i = load float, ptr %371, align 4, !dbg !4285, !alias.scope !4286, !noalias !4289, !noundef !12
  %_3.i200.us.i504 = fcmp ule float %_55.i11.us.i, 0.000000e+00, !dbg !4291
  %_3.i176.us.i = fcmp oge float %_0.i.i491.us.i, %threshold.i.i, !dbg !4293
  %_3.i174.us.i = fcmp oge float %_0.i.i491.us.i, %_0.i259.i, !dbg !4295
  %..i175.us.i = sext i1 %_3.i174.us.i to i32, !dbg !4297
  %_0.i412.us.i = sext i1 %_3.i176.us.i to i32, !dbg !4299
  %_0.i405.us.i = select i1 %_3.i200.us.i504, i32 %_0.i412.us.i, i32 %..i175.us.i, !dbg !4299
  %_0.i416.us.i = xor i32 %..i175.us.i, -1, !dbg !4301
  %_3.i198.us.i505 = fcmp ogt float %_67.i131005.us.i, 0.000000e+00, !dbg !4303
  %_0.i411.us.i = select i1 %_3.i198.us.i505, i32 %_0.i416.us.i, i32 0, !dbg !4305
  %_0.i410.us.i = select i1 %_3.i200.us.i504, i32 0, i32 %_0.i411.us.i, !dbg !4307
  %_0.i404.us.i = or i32 %_0.i410.us.i, %_0.i405.us.i, !dbg !4309
  %_5.i381.us.i = and i32 %_0.i404.us.i, 1065353216, !dbg !4311
  %_0.i385.us.i = bitcast i32 %_5.i381.us.i to float, !dbg !4313
  %_0.i258.us.i = fadd float %_67.i131005.us.i, -1.000000e+00, !dbg !4315
  %521 = trunc nsw i32 %_0.i410.us.i to i1, !dbg !4317
  %_4.i379.v.us.i = select i1 %521, float %_0.i258.us.i, float %_67.i131005.us.i, !dbg !4317
  %522 = trunc nsw i32 %_0.i405.us.i to i1, !dbg !4319
  %_0.i373.us.i506 = select i1 %522, float %_71.i585586.i, float %_4.i379.v.us.i, !dbg !4319
  store float %_0.i373.us.i506, ptr %372, align 4, !dbg !4321, !alias.scope !4286, !noalias !4289
  store i32 %_5.i381.us.i, ptr %371, align 4, !dbg !4322, !alias.scope !4286, !noalias !4289
  %_0.i256.us.i507 = fsub float %_0.i.i491.us.i, %threshold.i.i, !dbg !4323
  %_0.i242.us.i = fmul float %_0.i257.i, %_0.i256.us.i507, !dbg !4325
  %_3.i.i475.inv.us.i = fcmp ogt float %_0.i242.us.i, %374, !dbg !4327
  %_4.i.i482.v.us.i = select i1 %_3.i.i475.inv.us.i, float %_0.i242.us.i, float %374, !dbg !4327
  %_3.i.i541.us.i = fcmp olt float %_4.i.i482.v.us.i, 0.000000e+00, !dbg !4330
  %523 = fcmp ule float %_0.i385.us.i, 0.000000e+00, !dbg !4333
  %524 = select i1 %523, i1 %_3.i.i541.us.i, i1 false, !dbg !4335
  %_0.i366.us.i = select i1 %524, float %_4.i.i482.v.us.i, float 0.000000e+00, !dbg !4335
  %_3.i194.us.i = fcmp ule float %_0.i366.us.i, %_86.i211007.us.i, !dbg !4336
  %_4.i360.us.i = select i1 %_3.i194.us.i, i32 %_88.i24589.i, i32 %_87.i23588.i, !dbg !4338
  %_0.i.us.i508 = bitcast i32 %_4.i360.us.i to float, !dbg !4340
  %_0.i255.us.i509 = fsub float %_0.i366.us.i, %_86.i211007.us.i, !dbg !4342
  %_4.i222.us.i = fmul float %_0.i255.us.i509, %_0.i.us.i508, !dbg !4344
  %_0.i223.us.i = fadd float %_86.i211007.us.i, %_4.i222.us.i, !dbg !4344
  %525 = tail call noundef float @llvm.fabs.f32(float %_0.i223.us.i), !dbg !4346
  %526 = fcmp uge float %525, 0x3BC79CA100000000, !dbg !4349
  %_0.i296.us.i510 = select i1 %526, float %_0.i223.us.i, float 0.000000e+00, !dbg !4351
  store float %_0.i296.us.i510, ptr %375, align 4, !dbg !4352, !alias.scope !4286, !noalias !4289
  %_0.i241.us.i = fmul float %_0.i296.us.i510, 0x3FC542A5A0000000, !dbg !4353
  %_3.i.i426.us.inv.i = fcmp ogt float %_0.i241.us.i, -1.260000e+02, !dbg !4356
  %_0.i.i433.us.i = select i1 %_3.i.i426.us.inv.i, float %_0.i241.us.i, float -1.260000e+02, !dbg !4356
  %_3.i.i509.us.inv.i = fcmp olt float %_0.i.i433.us.i, 1.270000e+02, !dbg !4360
  %_0.i.i516.us.i = select i1 %_3.i.i509.us.inv.i, float %_0.i.i433.us.i, float 1.270000e+02, !dbg !4360
  %527 = tail call noundef float @llvm.floor.f32(float %_0.i.i516.us.i), !dbg !4363
  %_0.i248.us.i511 = fsub float %_0.i.i516.us.i, %527, !dbg !4367
  %_0.i230.us.i = fmul float %_0.i248.us.i511, 0x3F5E974FA0000000, !dbg !4369
  %_0.i215.us.i = fadd float %_0.i230.us.i, 0x3F82778560000000, !dbg !4371
  %_0.i230.us.1.i = fmul float %_0.i248.us.i511, %_0.i215.us.i, !dbg !4369
  %_0.i215.us.1.i = fadd float %_0.i230.us.1.i, 0x3FAC91CE60000000, !dbg !4371
  %_0.i230.us.2.i = fmul float %_0.i248.us.i511, %_0.i215.us.1.i, !dbg !4369
  %_0.i215.us.2.i = fadd float %_0.i230.us.2.i, 0x3FCEBDB560000000, !dbg !4371
  %_0.i230.us.3.i = fmul float %_0.i248.us.i511, %_0.i215.us.2.i, !dbg !4369
  %_0.i215.us.3.i = fadd float %_0.i230.us.3.i, 0x3FE62E4BA0000000, !dbg !4371
  %_0.i233.us.i = fmul float %_0.i249.us.i496, 0x3F5E974FA0000000, !dbg !4373
  %_0.i217.us.i = fadd float %_0.i233.us.i, 0x3F82778560000000, !dbg !4375
  %_0.i233.us.1.i = fmul float %_0.i249.us.i496, %_0.i217.us.i, !dbg !4373
  %_0.i217.us.1.i = fadd float %_0.i233.us.1.i, 0x3FAC91CE60000000, !dbg !4375
  %_0.i233.us.2.i = fmul float %_0.i249.us.i496, %_0.i217.us.1.i, !dbg !4373
  %_0.i217.us.2.i = fadd float %_0.i233.us.2.i, 0x3FCEBDB560000000, !dbg !4375
  %_0.i233.us.3.i = fmul float %_0.i249.us.i496, %_0.i217.us.2.i, !dbg !4373
  %_0.i217.us.3.i = fadd float %_0.i233.us.3.i, 0x3FE62E4BA0000000, !dbg !4375
  %_0.i232.us.i = fmul float %_0.i249.us.i496, %_0.i217.us.3.i, !dbg !4377
  %_0.i216.us.i = fadd float %_0.i232.us.i, 1.000000e+00, !dbg !4379
  %biased.i163.us.i = fadd float %517, 0x4160000FE0000000, !dbg !4381
  %_4.i164.us.i = bitcast float %biased.i163.us.i to i32, !dbg !4383
  %_3.i165.us.i = shl i32 %_4.i164.us.i, 23, !dbg !4385
  %_0.i166.us.i = bitcast i32 %_3.i165.us.i to float, !dbg !4386
  %_0.i231.us.i = fmul float %_0.i216.us.i, %_0.i166.us.i, !dbg !4388
  %_3.i167.us.i = fcmp une float %_0.i292.us.i495, 0.000000e+00, !dbg !4390
  %_0.i400576.not.us.i = and i1 %_3.i178.i, %_3.i167.us.i, !dbg !4393
  %_0.i234.us.i = fmul float %_0.i271.us.i468, %_0.i231.us.i, !dbg !4393
  %_4.i300.v.us.i = select i1 %_0.i400576.not.us.i, float %_0.i234.us.i, float %_0.i271.us.i468, !dbg !4396
  %_0.i229.us.i = fmul float %_0.i248.us.i511, %_0.i215.us.3.i, !dbg !4398
  %_0.i214.us.i = fadd float %_0.i229.us.i, 1.000000e+00, !dbg !4400
  %biased.i.us.i512 = fadd float %527, 0x4160000FE0000000, !dbg !4402
  %_4.i160.us.i = bitcast float %biased.i.us.i512 to i32, !dbg !4404
  %_3.i161.us.i = shl i32 %_4.i160.us.i, 23, !dbg !4406
  %_0.i162.us.i = bitcast i32 %_3.i161.us.i to float, !dbg !4407
  %_0.i228.us.i = fmul float %_0.i214.us.i, %_0.i162.us.i, !dbg !4409
  %_3.i168.us.i = fcmp une float %_0.i296.us.i510, 0.000000e+00, !dbg !4411
  %_0.i403593.not.us.i = and i1 %_3.i192.i, %_3.i168.us.i, !dbg !4413
  %_0.i240.us.i = fmul float %_0.i269.us.i472, %_0.i228.us.i, !dbg !4413
  %_4.i353.v.us.i = select i1 %_0.i403593.not.us.i, float %_0.i240.us.i, float %_0.i269.us.i472, !dbg !4415
  store float %_4.i300.v.us.i, ptr %_123.i.us.i439, align 4, !dbg !4417, !alias.scope !4420, !noalias !3914
  store float %_4.i353.v.us.i, ptr %_141.i.us.i444, align 4, !dbg !4423, !alias.scope !4425, !noalias !3936
  %exitcond2541.not.i = icmp eq i64 %503, %_26, !dbg !4428
  br i1 %exitcond2541.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb40.i.us.i433, !dbg !4431, !llvm.loop !4442

bb43.i.i127:                                      ; preds = %bb40.i.us.us.us.us.us.us.us.i49, %bb40.i.us.us.us.us.us.i128, %bb40.i.us.us.us.i279, %bb40.i.us.i433
  %.us-phi1028.i = phi i64 [ %503, %bb40.i.us.i433 ], [ %453, %bb40.i.us.us.us.i279 ], [ %403, %bb40.i.us.us.us.us.us.i128 ], [ %378, %bb40.i.us.us.us.us.us.us.us.i49 ]
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_55, i64 noundef %.us-phi1028.i, i64 noundef range(i64 0, 2305843009213693952) %_55, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_97ed0782c9e7425b1b215f66cb64a347) #24, !dbg !4443, !noalias !3868
  unreachable, !dbg !4443

bb42.i.i:                                         ; preds = %bb37.i.i41, %bb28.i.us.us.lr.ph.i
  %_86.i211007.i = phi float [ %_0.i296.i, %bb28.i.us.us.lr.ph.i ], [ %.promoted1006.i, %bb37.i.i41 ]
  %_67.i131005.i = phi float [ %_0.i373.i, %bb28.i.us.us.lr.ph.i ], [ %.promoted1004.i, %bb37.i.i41 ]
  %_86.i881003.i = phi float [ %_0.i292.i, %bb28.i.us.us.lr.ph.i ], [ %.promoted1002.i, %bb37.i.i41 ]
  %_67.i691001.i = phi float [ %_0.i321.i, %bb28.i.us.us.lr.ph.i ], [ %.promoted1000.i, %bb37.i.i41 ]
  %iter.sroa.0.0.i966.i = phi i64 [ %528, %bb28.i.us.us.lr.ph.i ], [ 0, %bb37.i.i41 ]
  %528 = add nuw nsw i64 %iter.sroa.0.0.i966.i, 1, !dbg !3871
  %_27.i.i = trunc i64 %iter.sroa.0.0.i966.i to i32, !dbg !3885
  %now.i.i = add i32 %base.i.i46, %_27.i.i, !dbg !3888
  %_30.i.i = and i32 %now.i.i, %_52.i30, !dbg !3891
  %_29.i.i = zext i32 %_30.i.i to i64, !dbg !3893
  %_123.i.i = getelementptr inbounds nuw float, ptr %_59, i64 %iter.sroa.0.0.i966.i, !dbg !3900
  %_124.not.not.i.i = icmp ugt i64 %_58.1.i20, %_29.i.i, !dbg !3904
  br i1 %_124.not.not.i.i, label %bb45.i.i, label %bb46.i.i58, !dbg !3904, !prof !2704

bb46.i.i58:                                       ; preds = %bb42.i.us.us.us.us.us.us.us.i55, %bb42.i.us.us.us.us.us.i133, %bb42.i.us.us.us.us.i205, %bb42.i.us.us.us.i284, %bb42.i.us.us.i356, %bb42.i.us.i438, %bb42.i.i
  %.us-phi1034.i = phi i64 [ %_29.i.us.us.us.i283, %bb42.i.us.us.us.i284 ], [ %_29.i.us.us.us.us.i209, %bb42.i.us.us.us.us.i205 ], [ %_29.i.us.us.us.us.us.i132, %bb42.i.us.us.us.us.us.i133 ], [ %_29.i.i, %bb42.i.i ], [ %_29.i.us.i437, %bb42.i.us.i438 ], [ %_29.i.us.us.i360, %bb42.i.us.us.i356 ], [ %_29.i.us.us.us.us.us.us.us.i53, %bb42.i.us.us.us.us.us.us.us.i55 ]
  %_36.i.le946.i = add nuw nsw i64 %.us-phi1034.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1034.i, i64 noundef %_36.i.le946.i, i64 noundef %_58.1.i20, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cad5a7b24766ffcdda9cbfe066eb4078) #24, !dbg !4444, !noalias !3868
  unreachable, !dbg !4444

bb45.i.i:                                         ; preds = %bb42.i.i
  %_0.i279.i = load float, ptr %_123.i.i, align 4, !dbg !3909, !alias.scope !3911, !noalias !3914, !noundef !12
  %_133.i.i = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_29.i.i, !dbg !3915
  store float %_0.i279.i, ptr %_133.i.i, align 4, !dbg !3919, !alias.scope !3921, !noalias !3868
  %exitcond2542.not.i = icmp eq i64 %iter.sroa.0.0.i966.i, %_63, !dbg !4445
  br i1 %exitcond2542.not.i, label %bb49.i.i, label %bb48.i.i, !dbg !4445, !prof !180

bb49.i.i:                                         ; preds = %bb45.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_63, i64 noundef %528, i64 noundef range(i64 0, 2305843009213693952) %_63, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aebeae245abdbd708a3d39b73262e641) #24, !dbg !4446, !noalias !3868
  unreachable, !dbg !4446

bb48.i.i:                                         ; preds = %bb45.i.i
  %_141.i.i = getelementptr inbounds nuw float, ptr %_67, i64 %iter.sroa.0.0.i966.i, !dbg !3924
  %_142.not.not.i.i = icmp ugt i64 %_60.1.i25, %_29.i.i, !dbg !4440
  br i1 %_142.not.not.i.i, label %bb50.i.i, label %bb51.i.i446, !dbg !4440, !prof !2704

bb51.i.i446:                                      ; preds = %bb45.i.us.i441, %bb48.i.i
  %.us-phi1040.i = phi i64 [ %_29.i.i, %bb48.i.i ], [ %_29.i.us.i437, %bb45.i.us.i441 ]
  %_36.i.le944.i = add nuw nsw i64 %.us-phi1040.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1040.i, i64 noundef %_36.i.le944.i, i64 noundef %_60.1.i25, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_72e24c5578221343e8a784545a3910d2) #24, !dbg !4447, !noalias !3868
  unreachable, !dbg !4447

bb50.i.i:                                         ; preds = %bb48.i.i
  %_0.i277.i = load float, ptr %_141.i.i, align 4, !dbg !3931, !alias.scope !3933, !noalias !3936, !noundef !12
  %_149.i.i = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_29.i.i, !dbg !3937
  store float %_0.i277.i, ptr %_149.i.i, align 4, !dbg !3944, !alias.scope !3946, !noalias !3868
  %exitcond2543.not.i = icmp eq i64 %iter.sroa.0.0.i966.i, %side_left.sroa.5.0.i.i45, !dbg !4438
  br i1 %exitcond2543.not.i, label %bb53.i.i432, label %bb52.i.i, !dbg !4438, !prof !180

bb53.i.i432:                                      ; preds = %bb45.i.us.us.i363, %bb50.i.us.i447, %bb50.i.i
  %.us-phi1046.i = phi i64 [ %503, %bb50.i.us.i447 ], [ %528, %bb50.i.i ], [ %478, %bb45.i.us.us.i363 ]
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %side_left.sroa.5.0.i.i45, i64 noundef %.us-phi1046.i, i64 noundef %side_left.sroa.5.0.i.i45, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2f4f8781efb7cd41cc45e538dfda195) #24, !dbg !4448, !noalias !3868
  unreachable, !dbg !4448

bb52.i.i:                                         ; preds = %bb50.i.i
  %_158.not.not.i.i = icmp ugt i64 %_59.1.i22, %_29.i.i, !dbg !4436
  br i1 %_158.not.not.i.i, label %bb54.i.i, label %bb55.i.i294, !dbg !4436, !prof !2704

bb55.i.i294:                                      ; preds = %bb45.i.us.us.us.i287, %bb52.i.us.us.i369, %bb52.i.us.i450, %bb52.i.i
  %.us-phi1052.i = phi i64 [ %_29.i.i, %bb52.i.i ], [ %_29.i.us.i437, %bb52.i.us.i450 ], [ %_29.i.us.us.i360, %bb52.i.us.us.i369 ], [ %_29.i.us.us.us.i283, %bb45.i.us.us.us.i287 ]
  %_36.i.le942.i = add nuw nsw i64 %.us-phi1052.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1052.i, i64 noundef %_36.i.le942.i, i64 noundef %_59.1.i22, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8b70dfa50e86ad5a1ce9afbe0da1688) #24, !dbg !4449, !noalias !3868
  unreachable, !dbg !4449

bb54.i.i:                                         ; preds = %bb52.i.i
  %_157.i.i = getelementptr inbounds nuw float, ptr %side_left.sroa.0.0.i.i44, i64 %iter.sroa.0.0.i966.i, !dbg !3949
  %_0.i275.i = load float, ptr %_157.i.i, align 4, !dbg !3956, !alias.scope !3958, !noalias !3868, !noundef !12
  %_165.i.i = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_29.i.i, !dbg !3961
  store float %_0.i275.i, ptr %_165.i.i, align 4, !dbg !3968, !alias.scope !3970, !noalias !3868
  %exitcond2544.not.i = icmp eq i64 %iter.sroa.0.0.i966.i, %empty.sroa.6.0.i.i42, !dbg !4434
  br i1 %exitcond2544.not.i, label %bb57.i.i278, label %bb56.i.i, !dbg !4434, !prof !180

bb57.i.i278:                                      ; preds = %bb45.i.us.us.us.us.i212, %bb54.i.us.us.us.i295, %bb54.i.us.us.i371, %bb54.i.us.i452, %bb54.i.i
  %.us-phi1058.i = phi i64 [ %453, %bb54.i.us.us.us.i295 ], [ %528, %bb54.i.i ], [ %503, %bb54.i.us.i452 ], [ %478, %bb54.i.us.us.i371 ], [ %428, %bb45.i.us.us.us.us.i212 ]
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %empty.sroa.6.0.i.i42, i64 noundef %.us-phi1058.i, i64 noundef %empty.sroa.6.0.i.i42, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_202abc45ab97a16825849449dde94dfa) #24, !dbg !4450, !noalias !3868
  unreachable, !dbg !4450

bb56.i.i:                                         ; preds = %bb54.i.i
  %_174.not.not.i.i = icmp ugt i64 %_61.1.i27, %_29.i.i, !dbg !4432
  br i1 %_174.not.not.i.i, label %bb58.i.i, label %bb59.i.i146, !dbg !4432, !prof !2704

bb59.i.i146:                                      ; preds = %bb45.i.us.us.us.us.us.i136, %bb56.i.us.us.us.us.i221, %bb56.i.us.us.us.i299, %bb56.i.us.us.i375, %bb56.i.us.i456, %bb56.i.i
  %.us-phi1064.i = phi i64 [ %_29.i.us.i437, %bb56.i.us.i456 ], [ %_29.i.us.us.i360, %bb56.i.us.us.i375 ], [ %_29.i.us.us.us.i283, %bb56.i.us.us.us.i299 ], [ %_29.i.us.us.us.us.i209, %bb56.i.us.us.us.us.i221 ], [ %_29.i.i, %bb56.i.i ], [ %_29.i.us.us.us.us.us.i132, %bb45.i.us.us.us.us.us.i136 ]
  %_36.i.le.i147 = add nuw nsw i64 %.us-phi1064.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1064.i, i64 noundef %_36.i.le.i147, i64 noundef %_61.1.i27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ad8d9ef662eaae13c96be161fe1fd2e1) #24, !dbg !4451, !noalias !3868
  unreachable, !dbg !4451

bb58.i.i:                                         ; preds = %bb56.i.i
  %_173.i.i = getelementptr inbounds nuw float, ptr %empty.sroa.0.0.i.i43, i64 %iter.sroa.0.0.i966.i, !dbg !3973
  %_0.i273.i = load float, ptr %_173.i.i, align 4, !dbg !3980, !alias.scope !3982, !noalias !3868, !noundef !12
  %_181.i.i = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_29.i.i, !dbg !3985
  store float %_0.i273.i, ptr %_181.i.i, align 4, !dbg !3992, !alias.scope !3994, !noalias !3868
  %_52.i.i = sub i32 %now.i.i, %_53.i31, !dbg !3997
  %_51.i.i = and i32 %_52.i.i, %_52.i30, !dbg !4000
  %_50.i.i = zext i32 %_51.i.i to i64, !dbg !4001
  %_182.not.not.i.i = icmp ugt i64 %_58.1.i20, %_50.i.i, !dbg !4002
  br i1 %_182.not.not.i.i, label %bb60.i.i, label %bb61.i.i75, !dbg !4002, !prof !2704

bb61.i.i75:                                       ; preds = %bb45.i.us.us.us.us.us.us.us.i59, %bb58.i.us.us.us.us.us.i148, %bb58.i.us.us.us.us.i223, %bb58.i.us.us.us.i301, %bb58.i.us.us.i377, %bb58.i.us.i458, %bb58.i.i
  %.us-phi1070.i = phi i64 [ %_50.i.us.us.us.i307, %bb58.i.us.us.us.i301 ], [ %_50.i.us.us.us.us.i229, %bb58.i.us.us.us.us.i223 ], [ %_50.i.us.us.us.us.us.i154, %bb58.i.us.us.us.us.us.i148 ], [ %_50.i.i, %bb58.i.i ], [ %_50.i.us.i464, %bb58.i.us.i458 ], [ %_50.i.us.us.i383, %bb58.i.us.us.i377 ], [ %_50.i.us.us.us.us.us.us.us.i73, %bb45.i.us.us.us.us.us.us.us.i59 ]
  %_55.i.le940.i = add nuw nsw i64 %.us-phi1070.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1070.i, i64 noundef %_55.i.le940.i, i64 noundef %_58.1.i20, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba99eeeb3482270ebe1c674b4013dd6a) #24, !dbg !4452, !noalias !3868
  unreachable, !dbg !4452

bb60.i.i:                                         ; preds = %bb58.i.i
  %_189.i.i = getelementptr inbounds nuw float, ptr %_58.0.i19, i64 %_50.i.i, !dbg !4007
  %_0.i271.i = load float, ptr %_189.i.i, align 4, !dbg !4011, !alias.scope !4013, !noalias !3868, !noundef !12
  %_190.not.not.i.i = icmp ugt i64 %_60.1.i25, %_50.i.i, !dbg !4441
  br i1 %_190.not.not.i.i, label %bb63.i.i, label %bb64.i.i160, !dbg !4441, !prof !2704

bb64.i.i160:                                      ; preds = %bb60.i.us.i466, %bb60.i.i
  %.us-phi1076.i = phi i64 [ %_50.i.i, %bb60.i.i ], [ %_50.i.us.i464, %bb60.i.us.i466 ]
  %_55.i.le.i161 = add nuw nsw i64 %.us-phi1076.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1076.i, i64 noundef %_55.i.le.i161, i64 noundef %_60.1.i25, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f4c27429e4885e1b619f1e4a3587acc) #24, !dbg !4453, !noalias !3868
  unreachable, !dbg !4453

bb63.i.i:                                         ; preds = %bb60.i.i
  %_195.i.i = getelementptr inbounds nuw float, ptr %_60.0.i24, i64 %_50.i.i, !dbg !4016
  %_0.i269.i = load float, ptr %_195.i.i, align 4, !dbg !4024, !alias.scope !4026, !noalias !3868, !noundef !12
  %_66.i.i = sub i32 %now.i.i, %_67.i.i
  %_65.i.i = and i32 %_66.i.i, %_52.i30
  %_64.i.i = zext i32 %_65.i.i to i64
  %_74.i.i = sub i32 %now.i.i, %_75.i.i
  %_73.i.i = and i32 %_74.i.i, %_52.i30
  %_72.i.i = zext i32 %_73.i.i to i64
  %_80.i.i = icmp ugt i64 %_59.1.i22, %_64.i.i
  %529 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_64.i.i
  %530 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_64.i.i
  %_86.i.i = icmp ugt i64 %_61.1.i27, %_72.i.i
  %531 = getelementptr inbounds nuw float, ptr %_61.0.i26, i64 %_72.i.i
  %_88.i.i = icmp ugt i64 %_59.1.i22, %_72.i.i
  %532 = getelementptr inbounds nuw float, ptr %_59.0.i21, i64 %_72.i.i
  br i1 %_80.i.i, label %bb63.i.split.us.i, label %bb63.i.split.i90

bb63.i.split.us.i:                                ; preds = %bb63.i.i
  %_84.i.i = icmp ugt i64 %_61.1.i27, %_64.i.i
  br i1 %_84.i.i, label %bb24.i.us.lr.ph.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i93, !dbg !4029

bb24.i.us.lr.ph.i:                                ; preds = %bb63.i.split.us.i
  %_82.i.us.i = load float, ptr %530, align 4, !noalias !3868, !noundef !12
  br i1 %_86.i.i, label %bb24.i.us.lr.ph.split.us.i, label %bb24.i.us.lr.ph.split.i96

bb24.i.us.lr.ph.split.us.i:                       ; preds = %bb24.i.us.lr.ph.i
  br i1 %_88.i.i, label %bb28.i.us.us.lr.ph.i, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i98, !dbg !4037

bb28.i.us.us.lr.ph.i:                             ; preds = %bb24.i.us.lr.ph.split.us.i
  %_87.i.us.us.i = load float, ptr %532, align 4, !noalias !3868, !noundef !12
  %_85.i.us.us.le827.i = load float, ptr %531, align 4, !noalias !3868, !noundef !12
  %_78.i.us.le.i = load float, ptr %529, align 4, !noalias !3868, !noundef !12
  %533 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.i), !dbg !4038
  %534 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.i), !dbg !4048
  %_3.i.i466.i = fcmp ule float %533, %534, !dbg !4051
  %_6.i.i468.i = bitcast float %533 to i32, !dbg !4056
  %_8.i.i470.i = bitcast float %534 to i32, !dbg !4059
  %_4.i.i473.i = select i1 %_3.i.i466.i, i32 %_8.i.i470.i, i32 %_6.i.i468.i, !dbg !4061
  %_4.i346.i = select i1 %_3.i190.i, i32 %_6.i.i468.i, i32 %_4.i.i473.i, !dbg !4062
  %_0.i239.i = fmul float %533, 5.000000e-01, !dbg !4064
  %_0.i238.i = fmul float %534, 5.000000e-01, !dbg !4066
  %_0.i218.i = fadd float %_0.i238.i, %_0.i239.i, !dbg !4068
  %_6.i334.i = bitcast float %_0.i218.i to i32, !dbg !4070
  %_4.i339.i = select i1 %_3.i188.i, i32 %_4.i346.i, i32 %_6.i334.i, !dbg !4073
  %_0.i340.i = bitcast i32 %_4.i339.i to float, !dbg !4074
  %_3.i.i458.i = fcmp ule float %_0.i340.i, 0x3E45798EE0000000, !dbg !4076
  %_4.i.i464.i = select i1 %_3.i.i458.i, i32 841731191, i32 %_4.i339.i, !dbg !4079
  %_0.i.i465.i = bitcast i32 %_4.i.i464.i to float, !dbg !4081
  %_3.i.i418.i = fcmp ule float %_0.i.i465.i, 0x3810000000000000, !dbg !4083
  %_4.i.i424.i = select i1 %_3.i.i418.i, i32 8388608, i32 %_4.i.i464.i, !dbg !4089
  %_5.i284.i = and i32 %_4.i.i424.i, 8388607, !dbg !4091
  %_4.i285.i = or disjoint i32 %_5.i284.i, 1065353216, !dbg !4091
  %significand.i286.i = bitcast i32 %_4.i285.i to float, !dbg !4093
  %_0.i247.i = fadd float %significand.i286.i, -1.000000e+00, !dbg !4095
  %_0.i227.i = fmul float %_0.i247.i, 0x3F9B17A960000000, !dbg !4097
  %535 = fsub float 0x3FBF9A8440000000, %_0.i227.i, !dbg !4099
  %_0.i227.1.i = fmul float %_0.i247.i, %535, !dbg !4097
  %_0.i213.1.i = fadd float %_0.i227.1.i, 0xBFD1E3F400000000, !dbg !4099
  %_0.i227.2.i = fmul float %_0.i247.i, %_0.i213.1.i, !dbg !4097
  %_0.i213.2.i = fadd float %_0.i227.2.i, 0x3FDD544F20000000, !dbg !4099
  %_0.i227.3.i = fmul float %_0.i247.i, %_0.i213.2.i, !dbg !4097
  %_0.i213.3.i = fadd float %_0.i227.3.i, 0xBFE6FC2A60000000, !dbg !4099
  %_0.i227.4.i = fmul float %_0.i247.i, %_0.i213.3.i, !dbg !4097
  %_0.i213.4.i = fadd float %_0.i227.4.i, 0x3FF714B2A0000000, !dbg !4099
  %_9.i287.i = lshr i32 %_4.i.i424.i, 23, !dbg !4101
  %_8.i288.i = or disjoint i32 %_9.i287.i, 1258291200, !dbg !4101
  %_7.i289.i = bitcast i32 %_8.i288.i to float, !dbg !4102
  %exponent.i290.i = fadd float %_7.i289.i, 0xC160000FE0000000, !dbg !4104
  %_0.i226.i = fmul float %_0.i247.i, %_0.i213.4.i, !dbg !4105
  %_0.i212.i = fadd float %exponent.i290.i, %_0.i226.i, !dbg !4107
  %_0.i237.i = fmul float %_0.i212.i, 0x4018151820000000, !dbg !4109
  %_3.i.i533.inv.i = fcmp olt float %_0.i237.i, 2.400000e+01, !dbg !4111
  %_0.i.i540.i = select i1 %_3.i.i533.inv.i, float %_0.i237.i, float 2.400000e+01, !dbg !4111
  %_3.i.i450.inv.i = fcmp ogt float %_0.i.i540.i, -1.600000e+02, !dbg !4114
  %_0.i.i457.i = select i1 %_3.i.i450.inv.i, float %_0.i.i540.i, float -1.600000e+02, !dbg !4114
  %_55.i59.i = load float, ptr %359, align 4, !dbg !4117, !alias.scope !4119, !noalias !4122, !noundef !12
  %_3.i186.i = fcmp ule float %_55.i59.i, 0.000000e+00, !dbg !4124
  %_3.i172.i = fcmp oge float %_0.i.i457.i, %threshold.i32.i, !dbg !4126
  %_3.i170.i = fcmp oge float %_0.i.i457.i, %_0.i254.i, !dbg !4129
  %..i171.i = sext i1 %_3.i170.i to i32, !dbg !4132
  %_0.i408.i = sext i1 %_3.i172.i to i32, !dbg !4134
  %_0.i402.i = select i1 %_3.i186.i, i32 %_0.i408.i, i32 %..i171.i, !dbg !4134
  %_0.i414.i = xor i32 %..i171.i, -1, !dbg !4139
  %_3.i184.i = fcmp ogt float %_67.i691001.i, 0.000000e+00, !dbg !4142
  %_0.i407.i = select i1 %_3.i184.i, i32 %_0.i414.i, i32 0, !dbg !4144
  %_0.i406.i = select i1 %_3.i186.i, i32 0, i32 %_0.i407.i, !dbg !4146
  %_0.i401.i = or i32 %_0.i406.i, %_0.i402.i, !dbg !4148
  %_5.i329.i = and i32 %_0.i401.i, 1065353216, !dbg !4151
  %_0.i333.i = bitcast i32 %_5.i329.i to float, !dbg !4153
  %_0.i253.i = fadd float %_67.i691001.i, -1.000000e+00, !dbg !4155
  %536 = trunc nsw i32 %_0.i406.i to i1, !dbg !4158
  %_4.i327.v.i = select i1 %536, float %_0.i253.i, float %_67.i691001.i, !dbg !4158
  %537 = trunc nsw i32 %_0.i402.i to i1, !dbg !4160
  %_0.i321.i = select i1 %537, float %_71.i75568569.i, float %_4.i327.v.i, !dbg !4160
  store float %_0.i321.i, ptr %360, align 4, !dbg !4162, !alias.scope !4119, !noalias !4122
  store i32 %_5.i329.i, ptr %359, align 4, !dbg !4163, !alias.scope !4119, !noalias !4122
  %_0.i251.i = fsub float %_0.i.i457.i, %threshold.i32.i, !dbg !4164
  %_0.i236.i = fmul float %_0.i252.i, %_0.i251.i, !dbg !4166
  %_3.i.i442.inv.i = fcmp ogt float %_0.i236.i, %362, !dbg !4168
  %_4.i.i448.v.i = select i1 %_3.i.i442.inv.i, float %_0.i236.i, float %362, !dbg !4168
  %_3.i.i525.i = fcmp olt float %_4.i.i448.v.i, 0.000000e+00, !dbg !4171
  %538 = fcmp ule float %_0.i333.i, 0.000000e+00, !dbg !4174
  %539 = select i1 %538, i1 %_3.i.i525.i, i1 false, !dbg !4177
  %_0.i314.i = select i1 %539, float %_4.i.i448.v.i, float 0.000000e+00, !dbg !4177
  %_3.i180.i = fcmp ule float %_0.i314.i, %_86.i881003.i, !dbg !4178
  %_4.i307.i = select i1 %_3.i180.i, i32 %_88.i91572.i, i32 %_87.i90571.i, !dbg !4181
  %_0.i308.i = bitcast i32 %_4.i307.i to float, !dbg !4183
  %_0.i250.i = fsub float %_0.i314.i, %_86.i881003.i, !dbg !4185
  %_4.i220.i = fmul float %_0.i250.i, %_0.i308.i, !dbg !4188
  %_0.i221.i = fadd float %_86.i881003.i, %_4.i220.i, !dbg !4188
  %540 = tail call noundef float @llvm.fabs.f32(float %_0.i221.i), !dbg !4190
  %541 = fcmp uge float %540, 0x3BC79CA100000000, !dbg !4194
  %_0.i292.i = select i1 %541, float %_0.i221.i, float 0.000000e+00, !dbg !4196
  store float %_0.i292.i, ptr %363, align 4, !dbg !4197, !alias.scope !4119, !noalias !4122
  %_0.i235.i = fmul float %_0.i292.i, 0x3FC542A5A0000000, !dbg !4199
  %_3.i.i434.inv.i = fcmp ogt float %_0.i235.i, -1.260000e+02, !dbg !4203
  %_0.i.i441.i = select i1 %_3.i.i434.inv.i, float %_0.i235.i, float -1.260000e+02, !dbg !4203
  %_3.i.i517.inv.i = fcmp olt float %_0.i.i441.i, 1.270000e+02, !dbg !4207
  %_0.i.i524.i = select i1 %_3.i.i517.inv.i, float %_0.i.i441.i, float 1.270000e+02, !dbg !4207
  %542 = tail call noundef float @llvm.floor.f32(float %_0.i.i524.i), !dbg !4210
  %_0.i249.i = fsub float %_0.i.i524.i, %542, !dbg !4214
  %543 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le827.i), !dbg !4216
  %544 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.i), !dbg !4220
  %_3.i.i500.i = fcmp ule float %543, %544, !dbg !4222
  %_6.i.i502.i = bitcast float %543 to i32, !dbg !4225
  %_8.i.i504.i = bitcast float %544 to i32, !dbg !4228
  %_4.i.i507.i = select i1 %_3.i.i500.i, i32 %_8.i.i504.i, i32 %_6.i.i502.i, !dbg !4230
  %_4.i398.i = select i1 %_3.i204.i, i32 %_6.i.i502.i, i32 %_4.i.i507.i, !dbg !4231
  %_0.i245.i = fmul float %543, 5.000000e-01, !dbg !4233
  %_0.i244.i = fmul float %544, 5.000000e-01, !dbg !4235
  %_0.i219.i = fadd float %_0.i244.i, %_0.i245.i, !dbg !4237
  %_6.i386.i = bitcast float %_0.i219.i to i32, !dbg !4239
  %_4.i391.i = select i1 %_3.i202.i, i32 %_4.i398.i, i32 %_6.i386.i, !dbg !4242
  %_0.i392.i = bitcast i32 %_4.i391.i to float, !dbg !4243
  %_3.i.i492.i = fcmp ule float %_0.i392.i, 0x3E45798EE0000000, !dbg !4245
  %_4.i.i498.i = select i1 %_3.i.i492.i, i32 841731191, i32 %_4.i391.i, !dbg !4248
  %_0.i.i499.i = bitcast i32 %_4.i.i498.i to float, !dbg !4250
  %_3.i.i.i513 = fcmp ule float %_0.i.i499.i, 0x3810000000000000, !dbg !4252
  %_4.i.i.i = select i1 %_3.i.i.i513, i32 8388608, i32 %_4.i.i498.i, !dbg !4257
  %_5.i280.i = and i32 %_4.i.i.i, 8388607, !dbg !4259
  %_4.i281.i = or disjoint i32 %_5.i280.i, 1065353216, !dbg !4259
  %significand.i.i = bitcast i32 %_4.i281.i to float, !dbg !4261
  %_0.i246.i = fadd float %significand.i.i, -1.000000e+00, !dbg !4263
  %_0.i225.i = fmul float %_0.i246.i, 0x3F9B17A960000000, !dbg !4265
  %545 = fsub float 0x3FBF9A8440000000, %_0.i225.i, !dbg !4267
  %_0.i225.1.i = fmul float %_0.i246.i, %545, !dbg !4265
  %_0.i211.1.i = fadd float %_0.i225.1.i, 0xBFD1E3F400000000, !dbg !4267
  %_0.i225.2.i = fmul float %_0.i246.i, %_0.i211.1.i, !dbg !4265
  %_0.i211.2.i = fadd float %_0.i225.2.i, 0x3FDD544F20000000, !dbg !4267
  %_0.i225.3.i = fmul float %_0.i246.i, %_0.i211.2.i, !dbg !4265
  %_0.i211.3.i = fadd float %_0.i225.3.i, 0xBFE6FC2A60000000, !dbg !4267
  %_0.i225.4.i = fmul float %_0.i246.i, %_0.i211.3.i, !dbg !4265
  %_0.i211.4.i = fadd float %_0.i225.4.i, 0x3FF714B2A0000000, !dbg !4267
  %_9.i.i = lshr i32 %_4.i.i.i, 23, !dbg !4269
  %_8.i282.i = or disjoint i32 %_9.i.i, 1258291200, !dbg !4269
  %_7.i.i = bitcast i32 %_8.i282.i to float, !dbg !4270
  %exponent.i.i = fadd float %_7.i.i, 0xC160000FE0000000, !dbg !4272
  %_0.i224.i = fmul float %_0.i246.i, %_0.i211.4.i, !dbg !4273
  %_0.i210.i = fadd float %exponent.i.i, %_0.i224.i, !dbg !4275
  %_0.i243.i = fmul float %_0.i210.i, 0x4018151820000000, !dbg !4277
  %_3.i.i549.inv.i = fcmp olt float %_0.i243.i, 2.400000e+01, !dbg !4279
  %_0.i.i556.i = select i1 %_3.i.i549.inv.i, float %_0.i243.i, float 2.400000e+01, !dbg !4279
  %_3.i.i484.inv.i = fcmp ogt float %_0.i.i556.i, -1.600000e+02, !dbg !4282
  %_0.i.i491.i = select i1 %_3.i.i484.inv.i, float %_0.i.i556.i, float -1.600000e+02, !dbg !4282
  %_55.i11.i = load float, ptr %371, align 4, !dbg !4285, !alias.scope !4286, !noalias !4289, !noundef !12
  %_3.i200.i = fcmp ule float %_55.i11.i, 0.000000e+00, !dbg !4291
  %_3.i176.i = fcmp oge float %_0.i.i491.i, %threshold.i.i, !dbg !4293
  %_3.i174.i = fcmp oge float %_0.i.i491.i, %_0.i259.i, !dbg !4295
  %..i175.i = sext i1 %_3.i174.i to i32, !dbg !4297
  %_0.i412.i = sext i1 %_3.i176.i to i32, !dbg !4299
  %_0.i405.i = select i1 %_3.i200.i, i32 %_0.i412.i, i32 %..i175.i, !dbg !4299
  %_0.i416.i = xor i32 %..i175.i, -1, !dbg !4301
  %_3.i198.i = fcmp ogt float %_67.i131005.i, 0.000000e+00, !dbg !4303
  %_0.i411.i = select i1 %_3.i198.i, i32 %_0.i416.i, i32 0, !dbg !4305
  %_0.i410.i = select i1 %_3.i200.i, i32 0, i32 %_0.i411.i, !dbg !4307
  %_0.i404.i = or i32 %_0.i410.i, %_0.i405.i, !dbg !4309
  %_5.i381.i = and i32 %_0.i404.i, 1065353216, !dbg !4311
  %_0.i385.i = bitcast i32 %_5.i381.i to float, !dbg !4313
  %_0.i258.i = fadd float %_67.i131005.i, -1.000000e+00, !dbg !4315
  %546 = trunc nsw i32 %_0.i410.i to i1, !dbg !4317
  %_4.i379.v.i = select i1 %546, float %_0.i258.i, float %_67.i131005.i, !dbg !4317
  %547 = trunc nsw i32 %_0.i405.i to i1, !dbg !4319
  %_0.i373.i = select i1 %547, float %_71.i585586.i, float %_4.i379.v.i, !dbg !4319
  store float %_0.i373.i, ptr %372, align 4, !dbg !4321, !alias.scope !4286, !noalias !4289
  store i32 %_5.i381.i, ptr %371, align 4, !dbg !4322, !alias.scope !4286, !noalias !4289
  %_0.i256.i = fsub float %_0.i.i491.i, %threshold.i.i, !dbg !4323
  %_0.i242.i = fmul float %_0.i257.i, %_0.i256.i, !dbg !4325
  %_3.i.i475.inv.i = fcmp ogt float %_0.i242.i, %374, !dbg !4327
  %_4.i.i482.v.i = select i1 %_3.i.i475.inv.i, float %_0.i242.i, float %374, !dbg !4327
  %_3.i.i541.i = fcmp olt float %_4.i.i482.v.i, 0.000000e+00, !dbg !4330
  %548 = fcmp ule float %_0.i385.i, 0.000000e+00, !dbg !4333
  %549 = select i1 %548, i1 %_3.i.i541.i, i1 false, !dbg !4335
  %_0.i366.i = select i1 %549, float %_4.i.i482.v.i, float 0.000000e+00, !dbg !4335
  %_3.i194.i = fcmp ule float %_0.i366.i, %_86.i211007.i, !dbg !4336
  %_4.i360.i = select i1 %_3.i194.i, i32 %_88.i24589.i, i32 %_87.i23588.i, !dbg !4338
  %_0.i.i = bitcast i32 %_4.i360.i to float, !dbg !4340
  %_0.i255.i = fsub float %_0.i366.i, %_86.i211007.i, !dbg !4342
  %_4.i222.i = fmul float %_0.i255.i, %_0.i.i, !dbg !4344
  %_0.i223.i = fadd float %_86.i211007.i, %_4.i222.i, !dbg !4344
  %550 = tail call noundef float @llvm.fabs.f32(float %_0.i223.i), !dbg !4346
  %551 = fcmp uge float %550, 0x3BC79CA100000000, !dbg !4349
  %_0.i296.i = select i1 %551, float %_0.i223.i, float 0.000000e+00, !dbg !4351
  store float %_0.i296.i, ptr %375, align 4, !dbg !4352, !alias.scope !4286, !noalias !4289
  %_0.i241.i = fmul float %_0.i296.i, 0x3FC542A5A0000000, !dbg !4353
  %_3.i.i426.inv.i = fcmp ogt float %_0.i241.i, -1.260000e+02, !dbg !4356
  %_0.i.i433.i = select i1 %_3.i.i426.inv.i, float %_0.i241.i, float -1.260000e+02, !dbg !4356
  %_3.i.i509.inv.i = fcmp olt float %_0.i.i433.i, 1.270000e+02, !dbg !4360
  %_0.i.i516.i = select i1 %_3.i.i509.inv.i, float %_0.i.i433.i, float 1.270000e+02, !dbg !4360
  %552 = tail call noundef float @llvm.floor.f32(float %_0.i.i516.i), !dbg !4363
  %_0.i248.i = fsub float %_0.i.i516.i, %552, !dbg !4367
  %_0.i230.i = fmul float %_0.i248.i, 0x3F5E974FA0000000, !dbg !4369
  %_0.i215.i = fadd float %_0.i230.i, 0x3F82778560000000, !dbg !4371
  %_0.i230.1.i = fmul float %_0.i248.i, %_0.i215.i, !dbg !4369
  %_0.i215.1.i = fadd float %_0.i230.1.i, 0x3FAC91CE60000000, !dbg !4371
  %_0.i230.2.i = fmul float %_0.i248.i, %_0.i215.1.i, !dbg !4369
  %_0.i215.2.i = fadd float %_0.i230.2.i, 0x3FCEBDB560000000, !dbg !4371
  %_0.i230.3.i = fmul float %_0.i248.i, %_0.i215.2.i, !dbg !4369
  %_0.i215.3.i = fadd float %_0.i230.3.i, 0x3FE62E4BA0000000, !dbg !4371
  %_0.i233.i = fmul float %_0.i249.i, 0x3F5E974FA0000000, !dbg !4373
  %_0.i217.i = fadd float %_0.i233.i, 0x3F82778560000000, !dbg !4375
  %_0.i233.1.i = fmul float %_0.i249.i, %_0.i217.i, !dbg !4373
  %_0.i217.1.i = fadd float %_0.i233.1.i, 0x3FAC91CE60000000, !dbg !4375
  %_0.i233.2.i = fmul float %_0.i249.i, %_0.i217.1.i, !dbg !4373
  %_0.i217.2.i = fadd float %_0.i233.2.i, 0x3FCEBDB560000000, !dbg !4375
  %_0.i233.3.i = fmul float %_0.i249.i, %_0.i217.2.i, !dbg !4373
  %_0.i217.3.i = fadd float %_0.i233.3.i, 0x3FE62E4BA0000000, !dbg !4375
  %_0.i232.i = fmul float %_0.i249.i, %_0.i217.3.i, !dbg !4377
  %_0.i216.i = fadd float %_0.i232.i, 1.000000e+00, !dbg !4379
  %biased.i163.i = fadd float %542, 0x4160000FE0000000, !dbg !4381
  %_4.i164.i = bitcast float %biased.i163.i to i32, !dbg !4383
  %_3.i165.i = shl i32 %_4.i164.i, 23, !dbg !4385
  %_0.i166.i = bitcast i32 %_3.i165.i to float, !dbg !4386
  %_0.i231.i = fmul float %_0.i216.i, %_0.i166.i, !dbg !4388
  %_3.i167.i = fcmp une float %_0.i292.i, 0.000000e+00, !dbg !4390
  %_0.i400576.not.i = and i1 %_3.i178.i, %_3.i167.i, !dbg !4393
  %_0.i234.i = fmul float %_0.i271.i, %_0.i231.i, !dbg !4393
  %_4.i300.v.i = select i1 %_0.i400576.not.i, float %_0.i234.i, float %_0.i271.i, !dbg !4396
  %_0.i229.i = fmul float %_0.i248.i, %_0.i215.3.i, !dbg !4398
  %_0.i214.i = fadd float %_0.i229.i, 1.000000e+00, !dbg !4400
  %biased.i.i = fadd float %552, 0x4160000FE0000000, !dbg !4402
  %_4.i160.i = bitcast float %biased.i.i to i32, !dbg !4404
  %_3.i161.i = shl i32 %_4.i160.i, 23, !dbg !4406
  %_0.i162.i = bitcast i32 %_3.i161.i to float, !dbg !4407
  %_0.i228.i = fmul float %_0.i214.i, %_0.i162.i, !dbg !4409
  %_3.i168.i = fcmp une float %_0.i296.i, 0.000000e+00, !dbg !4411
  %_0.i403593.not.i = and i1 %_3.i192.i, %_3.i168.i, !dbg !4413
  %_0.i240.i = fmul float %_0.i269.i, %_0.i228.i, !dbg !4413
  %_4.i353.v.i = select i1 %_0.i403593.not.i, float %_0.i240.i, float %_0.i269.i, !dbg !4415
  store float %_4.i300.v.i, ptr %_123.i.i, align 4, !dbg !4417, !alias.scope !4420, !noalias !3914
  store float %_4.i353.v.i, ptr %_141.i.i, align 4, !dbg !4423, !alias.scope !4425, !noalias !3936
  %exitcond2545.not.i = icmp eq i64 %528, %_26, !dbg !4428
  br i1 %exitcond2545.not.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit, label %bb42.i.i, !dbg !4431, !llvm.loop !4454

bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i98: ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i97, %bb24.i.us.lr.ph.split.us.us.us.us.us.i251, %bb24.i.us.lr.ph.split.us.us.us.us.i329, %bb24.i.us.lr.ph.split.us.us.us.i405, %bb24.i.us.lr.ph.split.us.us.i486, %bb24.i.us.lr.ph.split.us.i
  %.us-phi1129.i = phi i64 [ %_72.i.us.us.i397, %bb24.i.us.lr.ph.split.us.us.us.i405 ], [ %_72.i.us.us.us.i321, %bb24.i.us.lr.ph.split.us.us.us.us.i329 ], [ %_72.i.us.us.us.us.i243, %bb24.i.us.lr.ph.split.us.us.us.us.us.i251 ], [ %_72.i.i, %bb24.i.us.lr.ph.split.us.i ], [ %_72.i.us.i478, %bb24.i.us.lr.ph.split.us.us.i486 ], [ %_72.i.us.us.us.us.us.us.us.i86, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.us.us.i97 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1129.i, i64 noundef %_59.1.i22, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5ae5decb60097d7e1183cb43cae6b3f) #24, !dbg !4037, !noalias !3868
  unreachable, !dbg !4037

bb24.i.us.lr.ph.split.i96:                        ; preds = %bb24.i.us.lr.ph.us.us.us.us.us.us.us.i94, %bb24.i.us.lr.ph.us.us.us.us.us.i176, %bb24.i.us.lr.ph.us.us.us.us.i249, %bb24.i.us.lr.ph.us.us.us.i327, %bb24.i.us.lr.ph.us.us.i403, %bb24.i.us.lr.ph.us.i484, %bb24.i.us.lr.ph.i
  %.us-phi1111.i = phi i64 [ %_72.i.us.us.us.i321, %bb24.i.us.lr.ph.us.us.us.i327 ], [ %_72.i.us.us.us.us.i243, %bb24.i.us.lr.ph.us.us.us.us.i249 ], [ %_72.i.us.us.us.us.us.i170, %bb24.i.us.lr.ph.us.us.us.us.us.i176 ], [ %_72.i.i, %bb24.i.us.lr.ph.i ], [ %_72.i.us.i478, %bb24.i.us.lr.ph.us.i484 ], [ %_72.i.us.us.i397, %bb24.i.us.lr.ph.us.us.i403 ], [ %_72.i.us.us.us.us.us.us.us.i86, %bb24.i.us.lr.ph.us.us.us.us.us.us.us.i94 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1111.i, i64 noundef %_61.1.i27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5833dd3f10f38ea96ec24c6054ff083c) #24, !dbg !4455, !noalias !3868
  unreachable, !dbg !4455

bb63.i.split.us.panic20.i.split.us_crit_edge.i93: ; preds = %bb63.i.split.us.us.us.us.us.us.us.us.i91, %bb63.i.split.us.us.us.us.us.us.i174, %bb63.i.split.us.us.us.us.us.i247, %bb63.i.split.us.us.us.us.i325, %bb63.i.split.us.us.us.i401, %bb63.i.split.us.us.i482, %bb63.i.split.us.i
  %.us-phi1105.i = phi i64 [ %_64.i.us.us.us.i318, %bb63.i.split.us.us.us.us.i325 ], [ %_64.i.us.us.us.us.i240, %bb63.i.split.us.us.us.us.us.i247 ], [ %_64.i.us.us.us.us.us.i167, %bb63.i.split.us.us.us.us.us.us.i174 ], [ %_64.i.i, %bb63.i.split.us.i ], [ %_64.i.us.i475, %bb63.i.split.us.us.i482 ], [ %_64.i.us.us.i394, %bb63.i.split.us.us.us.i401 ], [ %_64.i.us.us.us.us.us.us.us.i83, %bb63.i.split.us.us.us.us.us.us.us.us.i91 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1105.i, i64 noundef %_61.1.i27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30bf8a301493a607abcc78dbf93977e0) #24, !dbg !4029, !noalias !3868
  unreachable, !dbg !4029

bb63.i.split.i90:                                 ; preds = %bb60.i.us.us.us.us.us.us.us.i76, %bb60.i.us.us.us.us.us.i156, %bb60.i.us.us.us.us.i231, %bb60.i.us.us.us.i309, %bb60.i.us.us.i385, %bb63.i.us.i470, %bb63.i.i
  %.us-phi1083.i = phi i64 [ %_64.i.us.us.us.i318, %bb60.i.us.us.us.i309 ], [ %_64.i.us.us.us.us.i240, %bb60.i.us.us.us.us.i231 ], [ %_64.i.us.us.us.us.us.i167, %bb60.i.us.us.us.us.us.i156 ], [ %_64.i.i, %bb63.i.i ], [ %_64.i.us.i475, %bb63.i.us.i470 ], [ %_64.i.us.us.i394, %bb60.i.us.us.i385 ], [ %_64.i.us.us.us.us.us.us.us.i83, %bb60.i.us.us.us.us.us.us.us.i76 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1083.i, i64 noundef %_59.1.i22, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b53d553f9945f7702333f7af20204159) #24, !dbg !4456, !noalias !3868
  unreachable, !dbg !4456

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb1_E11run_segmentKb0_EB3_.exit: ; preds = %bb28.i.us.us.lr.ph.us.us.us.us.us.us.us.i99, %bb24.i.us.lr.ph.split.us.us.us.us.us.us.i178, %bb28.i.us.us.lr.ph.us.us.us.us.i252, %bb28.i.us.us.lr.ph.us.us.us.i330, %bb28.i.us.us.lr.ph.us.us.i406, %bb28.i.us.us.lr.ph.us.i487, %bb28.i.us.us.lr.ph.i
  %_108.i.i125 = trunc i64 %_26 to i32, !dbg !4457
  %_107.i.i126 = add i32 %base.i.i46, %_108.i.i125, !dbg !4458
  store i32 %_107.i.i126, ptr %_51.i29, align 4, !dbg !4460, !alias.scope !3812, !noalias !3868
  br label %bb7, !dbg !4461

bb26:                                             ; preds = %bb25
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i, i64 noundef %right.1, i64 noundef %right.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9c634077742a23e4340a846355ecc484) #24, !dbg !4462
  unreachable, !dbg !4462
}
