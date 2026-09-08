define internal fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_block_monoB5_(ptr noalias noundef nonnull align 16 dereferenceable(1136) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef %frames) unnamed_addr #0 !dbg !14027 {
start:
  %_75.i728 = alloca [368 x i8], align 16
  %peaks_left.i730 = alloca [1024 x i8], align 4
  %scratch.i731 = alloca [32 x i8], align 4
  %hot_left.i733 = alloca [368 x i8], align 16
  %peaks_left.i453 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_left.i455 = alloca [368 x i8], align 16
  %uniform_left.i38 = alloca [64 x i8], align 16
  %peaks_left.i39 = alloca [1024 x i8], align 4
  %hot_left.i41 = alloca [368 x i8], align 16
  %_105.i = alloca [368 x i8], align 16
  %uniform_left.i = alloca [64 x i8], align 16
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_left.i = alloca [368 x i8], align 16
  %shape = alloca [12 x i8], align 4
  %words = shl i32 %frames, 2, !dbg !14028
  %0 = getelementptr inbounds nuw i8, ptr %self, i32 1125, !dbg !14029
  %1 = load i8, ptr %0, align 1, !dbg !14029, !range !4667, !noundef !10
  %2 = getelementptr inbounds nuw i8, ptr %self, i32 904, !dbg !14031
  %3 = load i8, ptr %2, align 8, !dbg !14031, !range !4667, !noundef !10
  %_6 = icmp eq i8 %1, %3, !dbg !14029
  %4 = getelementptr inbounds nuw i8, ptr %self, i32 988
  %_68.0 = load ptr, ptr %4, align 4, !dbg !14032
  %5 = getelementptr inbounds nuw i8, ptr %self, i32 992
  %_68.1 = load i32, ptr %5, align 4, !dbg !14032
  br i1 %_6, label %bb1, label %bb11.thread, !dbg !14029

bb1:                                              ; preds = %start
  %_8.i3613 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_68.0, i32 %_68.1, !dbg !14033
  br label %bb1.i.i, !dbg !14038

bb1.i.i:                                          ; preds = %bb11.i.i3615, %bb1
  %_221.i.i = phi ptr [ %_22.i.i3616, %bb11.i.i3615 ], [ %_68.0, %bb1 ]
  %_12.i.i3614 = icmp eq ptr %_221.i.i, %_8.i3613, !dbg !14040
  br i1 %_12.i.i3614, label %bb3, label %bb11.i.i3615, !dbg !14043

bb11.i.i3615:                                     ; preds = %bb1.i.i
  %_22.i.i3616 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 16, !dbg !14044
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 12, !dbg !14046
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !14046, !alias.scope !14048, !noalias !14053, !noundef !10
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !14046
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !14046, !alias.scope !14048, !noalias !14053
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 4, !dbg !14046
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !14046, !alias.scope !14048, !noalias !14053
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !14046
  %_0.sroa.0.0.off0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !14046
  br i1 %_0.sroa.0.0.off0.i.i.i, label %bb1.i.i, label %bb11.thread, !dbg !14056

bb3:                                              ; preds = %bb1.i.i
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 996, !dbg !14057
  %_69.0 = load ptr, ptr %10, align 4, !dbg !14057, !nonnull !10, !noundef !10
  %11 = getelementptr inbounds nuw i8, ptr %self, i32 1000, !dbg !14057
  %_69.1 = load i32, ptr %11, align 4, !dbg !14057, !noundef !10
  %_8.i3617 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_69.0, i32 %_69.1, !dbg !14058
  br label %bb1.i.i3618, !dbg !14063

bb1.i.i3618:                                      ; preds = %bb11.i.i3621, %bb3
  %_221.i.i3619 = phi ptr [ %_22.i.i3622, %bb11.i.i3621 ], [ %_69.0, %bb3 ]
  %_12.i.i3620 = icmp eq ptr %_221.i.i3619, %_8.i3617, !dbg !14065
  br i1 %_12.i.i3620, label %bb5, label %bb11.i.i3621, !dbg !14068

bb11.i.i3621:                                     ; preds = %bb1.i.i3618
  %_22.i.i3622 = getelementptr inbounds nuw i8, ptr %_221.i.i3619, i32 16, !dbg !14069
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i3619, i32 12, !dbg !14071
  %_3.i.i.i3623 = load i32, ptr %12, align 4, !dbg !14071, !alias.scope !14073, !noalias !14078, !noundef !10
  %13 = icmp eq i32 %_3.i.i.i3623, 0, !dbg !14071
  %_51.i.i.i3624 = load i32, ptr %_221.i.i3619, align 4, !dbg !14071, !alias.scope !14073, !noalias !14078
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i3619, i32 4, !dbg !14071
  %_72.i.i.i3625 = load i32, ptr %14, align 4, !dbg !14071, !alias.scope !14073, !noalias !14078
  %15 = icmp eq i32 %_51.i.i.i3624, %_72.i.i.i3625, !dbg !14071
  %_0.sroa.0.0.off0.i.i.i3626 = select i1 %13, i1 %15, i1 false, !dbg !14071
  br i1 %_0.sroa.0.0.off0.i.i.i3626, label %bb1.i.i3618, label %bb11.thread, !dbg !14081

bb5:                                              ; preds = %bb1.i.i3618
  %_51.not = icmp ugt i32 %words, %left_io.1
  br i1 %_51.not, label %bb35, label %bb1.i3628, !dbg !14082, !prof !4596

bb11.thread:                                      ; preds = %bb11.i.i3615, %bb11.i.i3621, %start
  %16 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  br label %bb16, !dbg !14091

bb11:                                             ; preds = %bb1.i3628
  %17 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  %18 = load i8, ptr %17, align 4, !range !4667
  %_15 = trunc nuw i8 %18 to i1
  br i1 %_15, label %bb13, label %bb16, !dbg !14091

bb1.i3628:                                        ; preds = %bb5, %bb12.i3632
  %io.sroa.5.0.i = phi i32 [ %len.i.i.i, %bb12.i3632 ], [ %words, %bb5 ]
  %io.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb12.i3632 ], [ %left_io.0, %bb5 ]
  %19 = icmp eq i32 %io.sroa.5.0.i, 0, !dbg !14093
  br i1 %19, label %bb11, label %bb13.preheader.i, !dbg !14093

bb13.preheader.i:                                 ; preds = %bb1.i3628
  %spec.store.select.i3629 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i, i32 32), !dbg !14096
  %data.i.i.idx.i = shl nuw nsw i32 %spec.store.select.i3629, 2, !dbg !14099
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i, i32 %data.i.i.idx.i, !dbg !14099
  br label %bb13.i3630, !dbg !14104

bb13.i3630:                                       ; preds = %bb13.i3630, %bb13.preheader.i
  %iter.sroa.0.08.i = phi ptr [ %_35.i3631, %bb13.i3630 ], [ %io.sroa.0.0.i, %bb13.preheader.i ]
  %bits.sroa.0.07.i = phi i32 [ %20, %bb13.i3630 ], [ 0, %bb13.preheader.i ]
  %_35.i3631 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i, i32 4, !dbg !14106
  %_95.i = load i32, ptr %iter.sroa.0.08.i, align 4, !dbg !14108, !alias.scope !14109, !noundef !10
  %20 = or i32 %_95.i, %bits.sroa.0.07.i, !dbg !14112
  %_29.i = icmp eq ptr %_35.i3631, %data.i.i.i, !dbg !14113
  br i1 %_29.i, label %bb12.i3632, label %bb13.i3630, !dbg !14104

bb12.i3632:                                       ; preds = %bb13.i3630
  %len.i.i.i = sub nuw nsw i32 %io.sroa.5.0.i, %spec.store.select.i3629, !dbg !14115
  %21 = icmp eq i32 %20, 0, !dbg !14116
  br i1 %21, label %bb1.i3628, label %bb11.thread8154, !dbg !14116

bb11.thread8154:                                  ; preds = %bb12.i3632
  %22 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  br label %bb16, !dbg !14091

bb35:                                             ; preds = %bb5
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a8b9f3e4701c8e7f14c8a14ff519a24) #32, !dbg !14117
  unreachable, !dbg !14117

bb16:                                             ; preds = %bb11.thread8154, %bb11.thread, %bb11
  %23 = phi ptr [ %16, %bb11.thread ], [ %17, %bb11 ], [ %22, %bb11.thread8154 ]
  %quiet.sroa.0.0.off08153 = phi i1 [ false, %bb11.thread ], [ true, %bb11 ], [ false, %bb11.thread8154 ]
  %_22 = getelementptr inbounds nuw i8, ptr %self, i32 16, !dbg !14118
  %_23 = getelementptr inbounds nuw i8, ptr %self, i32 912, !dbg !14119
  %_24 = getelementptr inbounds nuw i8, ptr %self, i32 924, !dbg !14120
  %_25 = getelementptr inbounds nuw i8, ptr %self, i32 800, !dbg !14121
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14122), !dbg !14125
  %_8.i3633 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_68.0, i32 %_68.1, !dbg !14126
  br label %bb1.i.i3634, !dbg !14135

bb1.i.i3634:                                      ; preds = %bb11.i.i3637, %bb16
  %_221.i.i3635 = phi ptr [ %_22.i.i3638, %bb11.i.i3637 ], [ %_68.0, %bb16 ]
  %_12.i.i3636 = icmp eq ptr %_221.i.i3635, %_8.i3633, !dbg !14137
  br i1 %_12.i.i3636, label %bb12.i, label %bb11.i.i3637, !dbg !14140

bb11.i.i3637:                                     ; preds = %bb1.i.i3634
  %_22.i.i3638 = getelementptr inbounds nuw i8, ptr %_221.i.i3635, i32 16, !dbg !14141
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i3635, i32 12, !dbg !14143
  %_3.i.i.i3639 = load i32, ptr %24, align 4, !dbg !14143, !alias.scope !14145, !noalias !14150, !noundef !10
  %25 = icmp eq i32 %_3.i.i.i3639, 0, !dbg !14143
  %_51.i.i.i3640 = load i32, ptr %_221.i.i3635, align 4, !dbg !14143, !alias.scope !14145, !noalias !14150
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i3635, i32 4, !dbg !14143
  %_72.i.i.i3641 = load i32, ptr %26, align 4, !dbg !14143, !alias.scope !14145, !noalias !14150
  %27 = icmp eq i32 %_51.i.i.i3640, %_72.i.i.i3641, !dbg !14143
  %_0.sroa.0.0.off0.i.i.i3642 = select i1 %25, i1 %27, i1 false, !dbg !14143
  br i1 %_0.sroa.0.0.off0.i.i.i3642, label %bb1.i.i3634, label %bb13.i, !dbg !14157

bb13.i:                                           ; preds = %bb11.i.i3637
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14158), !dbg !14161
  %28 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !14163
  %_31.0.i = load ptr, ptr %28, align 4, !dbg !14163, !alias.scope !14158, !noalias !14165, !nonnull !10, !noundef !10
  %29 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !14163
  %_31.1.i = load i32, ptr %29, align 4, !dbg !14163, !alias.scope !14158, !noalias !14165, !noundef !10
  %_17.idx.i = mul nuw nsw i32 %_31.1.i, 12, !dbg !14166
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 %_17.idx.i, !dbg !14166
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14170), !dbg !14173, !noalias !14165
  %_5.not.i.i.i = icmp eq i32 %_31.1.i, 0
  %30 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 4
  %31 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 8
  br i1 %_5.not.i.i.i, label %bb2.i3653, label %bb1.i.i3644

bb1.i.i3644:                                      ; preds = %bb13.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i3647, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %bb13.i ]
  %_12.i.i3645 = icmp eq ptr %_224.i.i, %_17.i, !dbg !14174
  br i1 %_12.i.i3645, label %bb2.i3653, label %bb11.i.i3646, !dbg !14178

bb11.i.i3646:                                     ; preds = %bb1.i.i3644
  %_22.i.i3647 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 12, !dbg !14179
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14181), !dbg !14184, !noalias !14165
  %_9.i.i.i3648 = load i32, ptr %_224.i.i, align 4, !dbg !14185, !alias.scope !14181, !noalias !14188, !noundef !10
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !14185, !alias.scope !14170, !noalias !14190, !noundef !10
  %_8.i.i.i = icmp eq i32 %_9.i.i.i3648, %_10.i.i.i, !dbg !14185
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb8.i, !dbg !14185

bb2.i.i.i:                                        ; preds = %bb11.i.i3646
  %32 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 4, !dbg !14185
  %_12.i.i.i3649 = load i32, ptr %32, align 4, !dbg !14185, !alias.scope !14181, !noalias !14188, !noundef !10
  %_13.i.i.i3650 = load i32, ptr %30, align 4, !dbg !14185, !alias.scope !14170, !noalias !14190, !noundef !10
  %_11.i.i.i = icmp eq i32 %_12.i.i.i3649, %_13.i.i.i3650, !dbg !14185
  br i1 %_11.i.i.i, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb8.i, !dbg !14185

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %33 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 8, !dbg !14185
  %_14.i.i.i3651 = load i32, ptr %33, align 4, !dbg !14185, !alias.scope !14181, !noalias !14188, !noundef !10
  %_15.i.i.i3652 = load i32, ptr %31, align 4, !dbg !14185, !alias.scope !14170, !noalias !14190, !noundef !10
  %34 = icmp eq i32 %_14.i.i.i3651, %_15.i.i.i3652, !dbg !14185
  br i1 %34, label %bb1.i.i3644, label %bb8.i, !dbg !14184

bb2.i3653:                                        ; preds = %bb1.i.i3644, %bb13.i
  %35 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !14191
  %_32.0.i = load ptr, ptr %35, align 4, !dbg !14191, !alias.scope !14158, !noalias !14165, !nonnull !10, !noundef !10
  %36 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !14191
  %_32.1.i = load i32, ptr %36, align 4, !dbg !14191, !alias.scope !14158, !noalias !14165, !noundef !10
  %_26.idx.i = shl nuw nsw i32 %_32.1.i, 2, !dbg !14192
  %_26.i3654 = getelementptr inbounds nuw i8, ptr %_32.0.i, i32 %_26.idx.i, !dbg !14192
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14196), !dbg !14199, !noalias !14165
  %_6.not.i.i.i = icmp eq i32 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb4.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i3653, %bb11.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb11.i5.i ], [ %_32.0.i, %bb2.i3653 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i3654, !dbg !14200
  br i1 %_12.i4.i, label %bb4.i, label %bb11.i5.i, !dbg !14204

bb11.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i32 4, !dbg !14205
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !14207, !noalias !14208
  %_4.i.i.i3655 = load i32, ptr %_32.0.i, align 4, !dbg !14210, !alias.scope !14196, !noalias !14212, !noundef !10
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i3655, !dbg !14213
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb8.i, !dbg !14207

bb12.i:                                           ; preds = %bb1.i.i3634
  %37 = getelementptr inbounds nuw i8, ptr %self, i32 996, !dbg !14214
  %_20.0.i = load ptr, ptr %37, align 4, !dbg !14214, !alias.scope !14122, !noalias !14165, !nonnull !10, !noundef !10
  %38 = getelementptr inbounds nuw i8, ptr %self, i32 1000, !dbg !14214
  %_20.1.i = load i32, ptr %38, align 4, !dbg !14214, !alias.scope !14122, !noalias !14165, !noundef !10
  %_8.i3656 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_20.0.i, i32 %_20.1.i, !dbg !14215
  br label %bb1.i.i3657, !dbg !14220

bb1.i.i3657:                                      ; preds = %bb11.i.i3660, %bb12.i
  %_221.i.i3658 = phi ptr [ %_22.i.i3661, %bb11.i.i3660 ], [ %_20.0.i, %bb12.i ]
  %_12.i.i3659 = icmp eq ptr %_221.i.i3658, %_8.i3656, !dbg !14222
  br i1 %_12.i.i3659, label %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3666, label %bb11.i.i3660, !dbg !14225

bb11.i.i3660:                                     ; preds = %bb1.i.i3657
  %_22.i.i3661 = getelementptr inbounds nuw i8, ptr %_221.i.i3658, i32 16, !dbg !14226
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i3658, i32 12, !dbg !14228
  %_3.i.i.i3662 = load i32, ptr %39, align 4, !dbg !14228, !alias.scope !14230, !noalias !14235, !noundef !10
  %40 = icmp eq i32 %_3.i.i.i3662, 0, !dbg !14228
  %_51.i.i.i3663 = load i32, ptr %_221.i.i3658, align 4, !dbg !14228, !alias.scope !14230, !noalias !14235
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i3658, i32 4, !dbg !14228
  %_72.i.i.i3664 = load i32, ptr %41, align 4, !dbg !14228, !alias.scope !14230, !noalias !14235
  %42 = icmp eq i32 %_51.i.i.i3663, %_72.i.i.i3664, !dbg !14228
  %_0.sroa.0.0.off0.i.i.i3665 = select i1 %40, i1 %42, i1 false, !dbg !14228
  br i1 %_0.sroa.0.0.off0.i.i.i3665, label %bb1.i.i3657, label %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3666, !dbg !14238

_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3666: ; preds = %bb1.i.i3657, %bb11.i.i3660
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14239), !dbg !14161
  %43 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !14242
  %_31.0.i3667 = load ptr, ptr %43, align 4, !dbg !14242, !alias.scope !14239, !noalias !14165, !nonnull !10, !noundef !10
  %44 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !14242
  %_31.1.i3668 = load i32, ptr %44, align 4, !dbg !14242, !alias.scope !14239, !noalias !14165, !noundef !10
  %_17.idx.i3669 = mul nuw nsw i32 %_31.1.i3668, 12, !dbg !14244
  %_17.i3670 = getelementptr inbounds nuw i8, ptr %_31.0.i3667, i32 %_17.idx.i3669, !dbg !14244
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14248), !dbg !14251, !noalias !14165
  %_5.not.i.i.i3671 = icmp eq i32 %_31.1.i3668, 0
  %45 = getelementptr inbounds nuw i8, ptr %_31.0.i3667, i32 4
  %46 = getelementptr inbounds nuw i8, ptr %_31.0.i3667, i32 8
  br i1 %_5.not.i.i.i3671, label %bb2.i3688, label %bb1.i.i3672

bb1.i.i3672:                                      ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3666, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3685
  %_224.i.i3673 = phi ptr [ %_22.i.i3676, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3685 ], [ %_31.0.i3667, %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3666 ]
  %_12.i.i3674 = icmp eq ptr %_224.i.i3673, %_17.i3670, !dbg !14252
  br i1 %_12.i.i3674, label %bb2.i3688, label %bb11.i.i3675, !dbg !14256

bb11.i.i3675:                                     ; preds = %bb1.i.i3672
  %_22.i.i3676 = getelementptr inbounds nuw i8, ptr %_224.i.i3673, i32 12, !dbg !14257
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14259), !dbg !14262, !noalias !14165
  %_9.i.i.i3677 = load i32, ptr %_224.i.i3673, align 4, !dbg !14263, !alias.scope !14259, !noalias !14266, !noundef !10
  %_10.i.i.i3678 = load i32, ptr %_31.0.i3667, align 4, !dbg !14263, !alias.scope !14248, !noalias !14268, !noundef !10
  %_8.i.i.i3679 = icmp eq i32 %_9.i.i.i3677, %_10.i.i.i3678, !dbg !14263
  br i1 %_8.i.i.i3679, label %bb2.i.i.i3681, label %bb6.i, !dbg !14263

bb2.i.i.i3681:                                    ; preds = %bb11.i.i3675
  %47 = getelementptr inbounds nuw i8, ptr %_224.i.i3673, i32 4, !dbg !14263
  %_12.i.i.i3682 = load i32, ptr %47, align 4, !dbg !14263, !alias.scope !14259, !noalias !14266, !noundef !10
  %_13.i.i.i3683 = load i32, ptr %45, align 4, !dbg !14263, !alias.scope !14248, !noalias !14268, !noundef !10
  %_11.i.i.i3684 = icmp eq i32 %_12.i.i.i3682, %_13.i.i.i3683, !dbg !14263
  br i1 %_11.i.i.i3684, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3685, label %bb6.i, !dbg !14263

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3685: ; preds = %bb2.i.i.i3681
  %48 = getelementptr inbounds nuw i8, ptr %_224.i.i3673, i32 8, !dbg !14263
  %_14.i.i.i3686 = load i32, ptr %48, align 4, !dbg !14263, !alias.scope !14259, !noalias !14266, !noundef !10
  %_15.i.i.i3687 = load i32, ptr %46, align 4, !dbg !14263, !alias.scope !14248, !noalias !14268, !noundef !10
  %49 = icmp eq i32 %_14.i.i.i3686, %_15.i.i.i3687, !dbg !14263
  br i1 %49, label %bb1.i.i3672, label %bb6.i, !dbg !14262

bb2.i3688:                                        ; preds = %bb1.i.i3672, %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3666
  %50 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !14269
  %_32.0.i3689 = load ptr, ptr %50, align 4, !dbg !14269, !alias.scope !14239, !noalias !14165, !nonnull !10, !noundef !10
  %51 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !14269
  %_32.1.i3690 = load i32, ptr %51, align 4, !dbg !14269, !alias.scope !14239, !noalias !14165, !noundef !10
  %_26.idx.i3691 = shl nuw nsw i32 %_32.1.i3690, 2, !dbg !14270
  %_26.i3692 = getelementptr inbounds nuw i8, ptr %_32.0.i3689, i32 %_26.idx.i3691, !dbg !14270
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14274), !dbg !14277, !noalias !14165
  %_6.not.i.i.i3693 = icmp eq i32 %_32.1.i3690, 0
  br i1 %_6.not.i.i.i3693, label %bb2.i, label %bb1.i3.i3694

bb1.i3.i3694:                                     ; preds = %bb2.i3688, %bb11.i5.i3697
  %_223.i.i3695 = phi ptr [ %_22.i6.i3698, %bb11.i5.i3697 ], [ %_32.0.i3689, %bb2.i3688 ]
  %_12.i4.i3696 = icmp eq ptr %_223.i.i3695, %_26.i3692, !dbg !14278
  br i1 %_12.i4.i3696, label %bb2.i, label %bb11.i5.i3697, !dbg !14282

bb11.i5.i3697:                                    ; preds = %bb1.i3.i3694
  %_22.i6.i3698 = getelementptr inbounds nuw i8, ptr %_223.i.i3695, i32 4, !dbg !14283
  %ptr.val.i.i3699 = load i32, ptr %_223.i.i3695, align 4, !dbg !14285, !noalias !14286
  %_4.i.i.i3700 = load i32, ptr %_32.0.i3689, align 4, !dbg !14288, !alias.scope !14274, !noalias !14290, !noundef !10
  %_0.i.i.i3701 = icmp eq i32 %ptr.val.i.i3699, %_4.i.i.i3700, !dbg !14291
  br i1 %_0.i.i.i3701, label %bb1.i3.i3694, label %bb6.i, !dbg !14285

bb8.i:                                            ; preds = %bb11.i.i3646, %bb2.i.i.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb11.i5.i, %bb6.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14292), !dbg !14295
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14296), !dbg !14295
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14298), !dbg !14295
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i733), !dbg !14300, !noalias !14304
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i733, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #31, !dbg !14307, !noalias !14308
  %52 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !14309
  %53 = load i8, ptr %52, align 16, !dbg !14309, !range !4667, !alias.scope !14292, !noalias !14313, !noundef !10
  %54 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !14314
  %55 = load i8, ptr %54, align 1, !dbg !14314, !range !4667, !alias.scope !14292, !noalias !14313, !noundef !10
  %_25.i741 = load i32, ptr %_25, align 4, !dbg !14316, !alias.scope !14298, !noalias !14318, !noundef !10
  %56 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !14319
  %_27.i742 = load i32, ptr %56, align 4, !dbg !14319, !alias.scope !14298, !noalias !14318, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i731), !dbg !14321, !noalias !14304
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i731, i8 0, i32 32, i1 false), !noalias !14304
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i730), !dbg !14323, !noalias !14304
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i730, i8 0, i32 1024, i1 false), !noalias !14304
  %_78.not.i7539412 = icmp eq i32 %frames, 0, !dbg !14325
  br i1 %_78.not.i7539412, label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb29.i754.lr.ph, !dbg !14325

bb29.i754.lr.ph:                                  ; preds = %bb8.i
  %_23.i739 = trunc nuw i8 %55 to i1, !dbg !14314
  %spec.store.select19.i740 = select i1 %_23.i739, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !14314
  %_22.i737 = trunc nuw i8 %53 to i1, !dbg !14309
  %link.sroa.0.0.i738 = select i1 %_22.i737, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !14309
  %d9.i = lshr i32 %frames, 5, !dbg !14335
  %r2.i = and i32 %frames, 31, !dbg !14342
  %_19.not.i = icmp ne i32 %r2.i, 0, !dbg !14343
  %57 = zext i1 %_19.not.i to i32, !dbg !14343
  %yield_count.sroa.0.0.i = add nuw nsw i32 %d9.i, %57, !dbg !14343
  %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 16
  %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 32
  %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 48
  %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 64
  %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 80
  %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 96
  %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 112
  %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 128
  %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 144
  %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 160
  %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 176
  %58 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %59 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %60 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i.i796 = getelementptr inbounds nuw i8, ptr %self, i32 80
  %61 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %62 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %63 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i.i810 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %64 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %65 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %66 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i.i824 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %67 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %68 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %69 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i.i838 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %70 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %71 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %72 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i.i852 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %73 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %74 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %75 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i.i866 = getelementptr inbounds nuw i8, ptr %self, i32 400
  %76 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %77 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %78 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i.i880 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %79 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %80 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %81 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i.i894 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %82 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %83 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %84 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i.i908 = getelementptr inbounds nuw i8, ptr %self, i32 592
  %85 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %86 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %87 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i.i922 = getelementptr inbounds nuw i8, ptr %self, i32 656
  %88 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %89 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %90 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i.i936 = getelementptr inbounds nuw i8, ptr %self, i32 720
  %91 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %92 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %93 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %_51.i969 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 192
  %_52.i970 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 256
  %94 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 240
  %95 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 224
  %96 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 208
  %97 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 304
  %98 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 288
  %99 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 272
  %100 = bitcast <4 x i32> %link.sroa.0.0.i738 to <16 x i8>
  %101 = getelementptr inbounds nuw i8, ptr %self, i32 916
  %102 = getelementptr inbounds nuw i8, ptr %self, i32 1020
  %103 = getelementptr inbounds nuw i8, ptr %self, i32 944
  %104 = getelementptr inbounds nuw i8, ptr %self, i32 940
  %105 = getelementptr inbounds nuw i8, ptr %self, i32 1016
  %106 = getelementptr inbounds nuw i8, ptr %self, i32 1012
  %107 = getelementptr inbounds nuw i8, ptr %self, i32 984
  %108 = getelementptr inbounds nuw i8, ptr %self, i32 980
  %109 = getelementptr inbounds nuw i8, ptr %self, i32 968
  %110 = getelementptr inbounds nuw i8, ptr %self, i32 964
  %111 = getelementptr inbounds nuw i8, ptr %self, i32 952
  %112 = getelementptr inbounds nuw i8, ptr %self, i32 948
  %113 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 336
  %114 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 352
  %115 = getelementptr inbounds nuw i8, ptr %hot_left.i733, i32 320
  %116 = getelementptr inbounds nuw i8, ptr %self, i32 936
  %117 = getelementptr inbounds nuw i8, ptr %self, i32 932
  %118 = bitcast <4 x i32> %spec.store.select19.i740 to <16 x i8>
  %119 = getelementptr inbounds nuw i8, ptr %self, i32 920
  %_11.i.i.i.i7848250 = load <4 x float>, ptr %_22, align 16
  %_14.i.i.i.i7878251 = load <4 x float>, ptr %58, align 16
  %_17.i.i.i.i7908252 = load <4 x float>, ptr %59, align 16
  %_20.i.i.i.i7938253 = load <4 x float>, ptr %60, align 16
  %_25.i.i.i.i7988254 = load <4 x float>, ptr %row1.i.i.i.i796, align 16
  %_28.i.i.i.i8018255 = load <4 x float>, ptr %61, align 16
  %_31.i.i.i.i8048256 = load <4 x float>, ptr %62, align 16
  %_34.i.i.i.i8078257 = load <4 x float>, ptr %63, align 16
  %_39.i.i.i.i8128258 = load <4 x float>, ptr %row3.i.i.i.i810, align 16
  %_42.i.i.i.i8158259 = load <4 x float>, ptr %64, align 16
  %_45.i.i.i.i8188260 = load <4 x float>, ptr %65, align 16
  %_48.i.i.i.i8218261 = load <4 x float>, ptr %66, align 16
  %_53.i.i.i.i8268262 = load <4 x float>, ptr %row5.i.i.i.i824, align 16
  %_56.i.i.i.i8298263 = load <4 x float>, ptr %67, align 16
  %_59.i.i.i.i8328264 = load <4 x float>, ptr %68, align 16
  %_62.i.i.i.i8358265 = load <4 x float>, ptr %69, align 16
  %_67.i.i.i.i8408266 = load <4 x float>, ptr %row7.i.i.i.i838, align 16
  %_70.i.i.i.i8438267 = load <4 x float>, ptr %70, align 16
  %_73.i.i.i.i8468268 = load <4 x float>, ptr %71, align 16
  %_76.i.i.i.i8498269 = load <4 x float>, ptr %72, align 16
  %_81.i.i.i.i8548270 = load <4 x float>, ptr %row9.i.i.i.i852, align 16
  %_84.i.i.i.i8578271 = load <4 x float>, ptr %73, align 16
  %_87.i.i.i.i8608272 = load <4 x float>, ptr %74, align 16
  %_90.i.i.i.i8638273 = load <4 x float>, ptr %75, align 16
  %_95.i.i.i.i8688274 = load <4 x float>, ptr %row11.i.i.i.i866, align 16
  %_98.i.i.i.i8718275 = load <4 x float>, ptr %76, align 16
  %_101.i.i.i.i8748276 = load <4 x float>, ptr %77, align 16
  %_104.i.i.i.i8778277 = load <4 x float>, ptr %78, align 16
  %_109.i.i.i.i8828278 = load <4 x float>, ptr %row13.i.i.i.i880, align 16
  %_112.i.i.i.i8858279 = load <4 x float>, ptr %79, align 16
  %_115.i.i.i.i8888280 = load <4 x float>, ptr %80, align 16
  %_118.i.i.i.i8918281 = load <4 x float>, ptr %81, align 16
  %_123.i.i.i.i8968282 = load <4 x float>, ptr %row15.i.i.i.i894, align 16
  %_126.i.i.i.i8998283 = load <4 x float>, ptr %82, align 16
  %_129.i.i.i.i9028284 = load <4 x float>, ptr %83, align 16
  %_132.i.i.i.i9058285 = load <4 x float>, ptr %84, align 16
  %_137.i.i.i.i9108286 = load <4 x float>, ptr %row17.i.i.i.i908, align 16
  %_140.i.i.i.i9138287 = load <4 x float>, ptr %85, align 16
  %_143.i.i.i.i9168288 = load <4 x float>, ptr %86, align 16
  %_146.i.i.i.i9198289 = load <4 x float>, ptr %87, align 16
  %_151.i.i.i.i9248290 = load <4 x float>, ptr %row19.i.i.i.i922, align 16
  %_154.i.i.i.i9278291 = load <4 x float>, ptr %88, align 16
  %_157.i.i.i.i9308292 = load <4 x float>, ptr %89, align 16
  %_160.i.i.i.i9338293 = load <4 x float>, ptr %90, align 16
  %_165.i.i.i.i9388294 = load <4 x float>, ptr %row21.i.i.i.i936, align 16
  %_168.i.i.i.i9418295 = load <4 x float>, ptr %91, align 16
  %_171.i.i.i.i9448296 = load <4 x float>, ptr %92, align 16
  %_174.i.i.i.i9478297 = load <4 x float>, ptr %93, align 16
  %_13.i14058232 = load <16 x i8>, ptr %96, align 16
  %_13.i13928236 = load <16 x i8>, ptr %99, align 16
  %_62.i.i10468243 = load <4 x float>, ptr %114, align 16
  %iter.sroa.0.0.ptr.i.i10129286.1 = getelementptr inbounds nuw i8, ptr %scratch.i731, i32 4
  %iter.sroa.0.0.ptr.i.i10129286.2 = getelementptr inbounds nuw i8, ptr %scratch.i731, i32 8
  %iter.sroa.0.0.ptr.i.i10129286.3 = getelementptr inbounds nuw i8, ptr %scratch.i731, i32 12
  %iter.sroa.0.0.ptr.i.i10129286.4 = getelementptr inbounds nuw i8, ptr %scratch.i731, i32 16
  %iter.sroa.0.0.ptr.i.i10129286.5 = getelementptr inbounds nuw i8, ptr %scratch.i731, i32 20
  %iter.sroa.0.0.ptr.i.i10129286.6 = getelementptr inbounds nuw i8, ptr %scratch.i731, i32 24
  %iter.sroa.0.0.ptr.i.i10129286.7 = getelementptr inbounds nuw i8, ptr %scratch.i731, i32 28
  %hot_left.i733.promoted = load <4 x i32>, ptr %hot_left.i733, align 16
  %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16
  %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16
  %.promoted11953 = load <4 x float>, ptr %94, align 16
  %.promoted = load <4 x float>, ptr %97, align 16
  %.promoted11992 = load <4 x float>, ptr %113, align 16
  br label %bb29.i754, !dbg !14325

bb12.i748.loopexit:                               ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960
  %.lcssa1169111993 = phi <4 x float> [ %.lcssa1169111994, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960 ], [ %300, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %.lcssa1158511973 = phi <4 x float> [ %.lcssa1158511974, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960 ], [ %261, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %.lcssa1160311954 = phi <4 x float> [ %.lcssa1160311955, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960 ], [ %258, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %ring_cursor.sroa.0.1.i963.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i7519415, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960 ], [ %spec.store.select8.i1083, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14344
  %main_cursor.sroa.0.1.i964.lcssa = phi i32 [ %main_cursor.sroa.0.0.i7529416, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960 ], [ %spec.store.select7.i1081, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14345
  %_78.not.i753 = icmp eq i32 %122, 0, !dbg !14325
  %indvars.iv.next = add i32 %indvars.iv, -32, !dbg !14325
  br i1 %_78.not.i753, label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, label %bb29.i754, !dbg !14325

bb29.i754:                                        ; preds = %bb29.i754.lr.ph, %bb12.i748.loopexit
  %.lcssa1169111994 = phi <4 x float> [ %.promoted11992, %bb29.i754.lr.ph ], [ %.lcssa1169111993, %bb12.i748.loopexit ]
  %.lcssa1158511974 = phi <4 x float> [ %.promoted, %bb29.i754.lr.ph ], [ %.lcssa1158511973, %bb12.i748.loopexit ]
  %.lcssa1160311955 = phi <4 x float> [ %.promoted11953, %bb29.i754.lr.ph ], [ %.lcssa1160311954, %bb12.i748.loopexit ]
  %history.i.i727.sroa.38.0.lcssa11935 = phi <4 x i32> [ %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.38.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.35.0.lcssa11917 = phi <4 x i32> [ %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.35.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.32.0.lcssa11899 = phi <4 x i32> [ %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.32.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.29.0.lcssa11881 = phi <4 x i32> [ %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.29.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.26.0.lcssa11863 = phi <4 x i32> [ %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.26.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.22.0.lcssa11845 = phi <4 x i32> [ %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.22.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.19.0.lcssa11827 = phi <4 x i32> [ %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.19.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.16.0.lcssa11809 = phi <4 x i32> [ %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.16.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.13.0.lcssa11791 = phi <4 x i32> [ %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.13.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.10.0.lcssa11773 = phi <4 x i32> [ %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.10.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.7.0.lcssa11755 = phi <4 x i32> [ %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.7.0.lcssa, %bb12.i748.loopexit ]
  %history.i.i727.sroa.0.0.lcssa11737 = phi <4 x i32> [ %hot_left.i733.promoted, %bb29.i754.lr.ph ], [ %history.i.i727.sroa.0.0.lcssa, %bb12.i748.loopexit ]
  %indvars.iv = phi i32 [ %frames, %bb29.i754.lr.ph ], [ %indvars.iv.next, %bb12.i748.loopexit ]
  %main_cursor.sroa.0.0.i7529416 = phi i32 [ %_25.i741, %bb29.i754.lr.ph ], [ %main_cursor.sroa.0.1.i964.lcssa, %bb12.i748.loopexit ]
  %ring_cursor.sroa.0.0.i7519415 = phi i32 [ %_27.i742, %bb29.i754.lr.ph ], [ %ring_cursor.sroa.0.1.i963.lcssa, %bb12.i748.loopexit ]
  %iter2.sroa.0.0.i7509414 = phi i32 [ %yield_count.sroa.0.0.i, %bb29.i754.lr.ph ], [ %122, %bb12.i748.loopexit ]
  %iter.sroa.0.0.i7499413 = phi i32 [ 0, %bb29.i754.lr.ph ], [ %121, %bb12.i748.loopexit ]
  %120 = call i32 @llvm.umax.i32(i32 %indvars.iv, i32 1), !dbg !14346
  %umax11018 = call i32 @llvm.umin.i32(i32 %120, i32 32), !dbg !14346
  %121 = add i32 %iter.sroa.0.0.i7499413, 32, !dbg !14346
  %122 = add nsw i32 %iter2.sroa.0.0.i7509414, -1, !dbg !14350
  %123 = sub i32 %frames, %iter.sroa.0.0.i7499413, !dbg !14351
  %spec.store.select.i755 = tail call i32 @llvm.umin.i32(i32 %123, i32 32), !dbg !14353
  %active_base.i756 = shl i32 %iter.sroa.0.0.i7499413, 2, !dbg !14358
  %active_base.i7568228 = add i32 %spec.store.select.i755, %iter.sroa.0.0.i7499413, !dbg !14360
  %_40.i758 = shl i32 %active_base.i7568228, 2, !dbg !14360
  %_88.i759 = icmp ult i32 %_40.i758, %active_base.i756, !dbg !14363
  %_82.not.i760 = icmp ugt i32 %_40.i758, %left_io.1
  %or.cond.i761 = or i1 %_88.i759, %_82.not.i760, !dbg !14363
  br i1 %or.cond.i761, label %bb35.i1091, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !14363, !prof !4596

bb35.i1091:                                       ; preds = %bb29.i754
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa11737, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa11755, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa11773, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa11791, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa11809, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa11827, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa11845, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa11863, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa11881, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa11899, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa11917, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa11935, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i756, i32 noundef %_40.i758, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_cab916602395946b4285251e481c2df8) #32, !dbg !14372, !noalias !14373
  unreachable, !dbg !14372

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb29.i754
  %_91.i763 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %active_base.i756, !dbg !14374
  %_2.i37109251.not = icmp eq i32 %frames, %iter.sroa.0.0.i7499413, !dbg !14378
  br i1 %_2.i37109251.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960, label %bb5.i.i766, !dbg !14378

bb5.i.i766:                                       ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, %bb5.i.i766
  %iter.i.i723.sroa.16.09263 = phi i32 [ %245, %bb5.i.i766 ], [ 0, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.35.09262 = phi <4 x i32> [ %history.i.i727.sroa.32.09261, %bb5.i.i766 ], [ %history.i.i727.sroa.35.0.lcssa11917, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.32.09261 = phi <4 x i32> [ %history.i.i727.sroa.29.09260, %bb5.i.i766 ], [ %history.i.i727.sroa.32.0.lcssa11899, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.29.09260 = phi <4 x i32> [ %history.i.i727.sroa.26.09259, %bb5.i.i766 ], [ %history.i.i727.sroa.29.0.lcssa11881, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.26.09259 = phi <4 x i32> [ %history.i.i727.sroa.22.09258, %bb5.i.i766 ], [ %history.i.i727.sroa.26.0.lcssa11863, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.22.09258 = phi <4 x i32> [ %history.i.i727.sroa.19.09257, %bb5.i.i766 ], [ %history.i.i727.sroa.22.0.lcssa11845, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.19.09257 = phi <4 x i32> [ %history.i.i727.sroa.16.09256, %bb5.i.i766 ], [ %history.i.i727.sroa.19.0.lcssa11827, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.16.09256 = phi <4 x i32> [ %history.i.i727.sroa.13.09255, %bb5.i.i766 ], [ %history.i.i727.sroa.16.0.lcssa11809, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.13.09255 = phi <4 x i32> [ %history.i.i727.sroa.10.09254, %bb5.i.i766 ], [ %history.i.i727.sroa.13.0.lcssa11791, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.10.09254 = phi <4 x i32> [ %history.i.i727.sroa.7.09253, %bb5.i.i766 ], [ %history.i.i727.sroa.10.0.lcssa11773, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.7.09253 = phi <4 x i32> [ %history.i.i727.sroa.0.09252, %bb5.i.i766 ], [ %history.i.i727.sroa.7.0.lcssa11755, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %history.i.i727.sroa.0.09252 = phi <4 x i32> [ %lanes.i3029.sroa.0.0.copyload, %bb5.i.i766 ], [ %history.i.i727.sroa.0.0.lcssa11737, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %start1.i.i = shl i32 %iter.i.i723.sroa.16.09263, 2, !dbg !14381
  %data.i.i3712 = getelementptr inbounds nuw float, ptr %_91.i763, i32 %start1.i.i, !dbg !14383
  %lanes.i3029.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i3712, align 4, !dbg !14385, !alias.scope !14390, !noalias !14394
  %124 = bitcast <4 x i32> %history.i.i727.sroa.19.09257 to <4 x float>, !dbg !14401
  %125 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %124), !dbg !14406
  %126 = bitcast <4 x i32> %lanes.i3029.sroa.0.0.copyload to <4 x float>, !dbg !14407
  %127 = fmul <4 x float> %_11.i.i.i.i7848250, %126, !dbg !14412
  %128 = fadd <4 x float> %127, zeroinitializer, !dbg !14413
  %129 = fmul <4 x float> %_14.i.i.i.i7878251, %126, !dbg !14417
  %130 = fadd <4 x float> %129, zeroinitializer, !dbg !14421
  %131 = fmul <4 x float> %_17.i.i.i.i7908252, %126, !dbg !14425
  %132 = fadd <4 x float> %131, zeroinitializer, !dbg !14429
  %133 = fmul <4 x float> %_20.i.i.i.i7938253, %126, !dbg !14433
  %134 = fadd <4 x float> %133, zeroinitializer, !dbg !14437
  %135 = bitcast <4 x i32> %history.i.i727.sroa.0.09252 to <4 x float>, !dbg !14441
  %136 = fmul <4 x float> %_25.i.i.i.i7988254, %135, !dbg !14445
  %137 = fadd <4 x float> %128, %136, !dbg !14446
  %138 = fmul <4 x float> %_28.i.i.i.i8018255, %135, !dbg !14450
  %139 = fadd <4 x float> %130, %138, !dbg !14454
  %140 = fmul <4 x float> %_31.i.i.i.i8048256, %135, !dbg !14458
  %141 = fadd <4 x float> %132, %140, !dbg !14462
  %142 = fmul <4 x float> %_34.i.i.i.i8078257, %135, !dbg !14466
  %143 = fadd <4 x float> %134, %142, !dbg !14470
  %144 = bitcast <4 x i32> %history.i.i727.sroa.7.09253 to <4 x float>, !dbg !14474
  %145 = fmul <4 x float> %_39.i.i.i.i8128258, %144, !dbg !14478
  %146 = fadd <4 x float> %137, %145, !dbg !14479
  %147 = fmul <4 x float> %_42.i.i.i.i8158259, %144, !dbg !14483
  %148 = fadd <4 x float> %139, %147, !dbg !14487
  %149 = fmul <4 x float> %_45.i.i.i.i8188260, %144, !dbg !14491
  %150 = fadd <4 x float> %141, %149, !dbg !14495
  %151 = fmul <4 x float> %_48.i.i.i.i8218261, %144, !dbg !14499
  %152 = fadd <4 x float> %143, %151, !dbg !14503
  %153 = bitcast <4 x i32> %history.i.i727.sroa.10.09254 to <4 x float>, !dbg !14507
  %154 = fmul <4 x float> %_53.i.i.i.i8268262, %153, !dbg !14511
  %155 = fadd <4 x float> %146, %154, !dbg !14512
  %156 = fmul <4 x float> %_56.i.i.i.i8298263, %153, !dbg !14516
  %157 = fadd <4 x float> %148, %156, !dbg !14520
  %158 = fmul <4 x float> %_59.i.i.i.i8328264, %153, !dbg !14524
  %159 = fadd <4 x float> %150, %158, !dbg !14528
  %160 = fmul <4 x float> %_62.i.i.i.i8358265, %153, !dbg !14532
  %161 = fadd <4 x float> %152, %160, !dbg !14536
  %162 = bitcast <4 x i32> %history.i.i727.sroa.13.09255 to <4 x float>, !dbg !14540
  %163 = fmul <4 x float> %_67.i.i.i.i8408266, %162, !dbg !14544
  %164 = fadd <4 x float> %155, %163, !dbg !14545
  %165 = fmul <4 x float> %_70.i.i.i.i8438267, %162, !dbg !14549
  %166 = fadd <4 x float> %157, %165, !dbg !14553
  %167 = fmul <4 x float> %_73.i.i.i.i8468268, %162, !dbg !14557
  %168 = fadd <4 x float> %159, %167, !dbg !14561
  %169 = fmul <4 x float> %_76.i.i.i.i8498269, %162, !dbg !14565
  %170 = fadd <4 x float> %161, %169, !dbg !14569
  %171 = bitcast <4 x i32> %history.i.i727.sroa.16.09256 to <4 x float>, !dbg !14573
  %172 = fmul <4 x float> %_81.i.i.i.i8548270, %171, !dbg !14577
  %173 = fadd <4 x float> %164, %172, !dbg !14578
  %174 = fmul <4 x float> %_84.i.i.i.i8578271, %171, !dbg !14582
  %175 = fadd <4 x float> %166, %174, !dbg !14586
  %176 = fmul <4 x float> %_87.i.i.i.i8608272, %171, !dbg !14590
  %177 = fadd <4 x float> %168, %176, !dbg !14594
  %178 = fmul <4 x float> %_90.i.i.i.i8638273, %171, !dbg !14598
  %179 = fadd <4 x float> %170, %178, !dbg !14602
  %180 = fmul <4 x float> %_95.i.i.i.i8688274, %124, !dbg !14606
  %181 = fadd <4 x float> %173, %180, !dbg !14610
  %182 = fmul <4 x float> %_98.i.i.i.i8718275, %124, !dbg !14614
  %183 = fadd <4 x float> %175, %182, !dbg !14618
  %184 = fmul <4 x float> %_101.i.i.i.i8748276, %124, !dbg !14622
  %185 = fadd <4 x float> %177, %184, !dbg !14626
  %186 = fmul <4 x float> %_104.i.i.i.i8778277, %124, !dbg !14630
  %187 = fadd <4 x float> %179, %186, !dbg !14634
  %188 = bitcast <4 x i32> %history.i.i727.sroa.22.09258 to <4 x float>, !dbg !14638
  %189 = fmul <4 x float> %_109.i.i.i.i8828278, %188, !dbg !14642
  %190 = fadd <4 x float> %181, %189, !dbg !14643
  %191 = fmul <4 x float> %_112.i.i.i.i8858279, %188, !dbg !14647
  %192 = fadd <4 x float> %183, %191, !dbg !14651
  %193 = fmul <4 x float> %_115.i.i.i.i8888280, %188, !dbg !14655
  %194 = fadd <4 x float> %185, %193, !dbg !14659
  %195 = fmul <4 x float> %_118.i.i.i.i8918281, %188, !dbg !14663
  %196 = fadd <4 x float> %187, %195, !dbg !14667
  %197 = bitcast <4 x i32> %history.i.i727.sroa.26.09259 to <4 x float>, !dbg !14671
  %198 = fmul <4 x float> %_123.i.i.i.i8968282, %197, !dbg !14675
  %199 = fadd <4 x float> %190, %198, !dbg !14676
  %200 = fmul <4 x float> %_126.i.i.i.i8998283, %197, !dbg !14680
  %201 = fadd <4 x float> %192, %200, !dbg !14684
  %202 = fmul <4 x float> %_129.i.i.i.i9028284, %197, !dbg !14688
  %203 = fadd <4 x float> %194, %202, !dbg !14692
  %204 = fmul <4 x float> %_132.i.i.i.i9058285, %197, !dbg !14696
  %205 = fadd <4 x float> %196, %204, !dbg !14700
  %206 = bitcast <4 x i32> %history.i.i727.sroa.29.09260 to <4 x float>, !dbg !14704
  %207 = fmul <4 x float> %_137.i.i.i.i9108286, %206, !dbg !14708
  %208 = fadd <4 x float> %199, %207, !dbg !14709
  %209 = fmul <4 x float> %_140.i.i.i.i9138287, %206, !dbg !14713
  %210 = fadd <4 x float> %201, %209, !dbg !14717
  %211 = fmul <4 x float> %_143.i.i.i.i9168288, %206, !dbg !14721
  %212 = fadd <4 x float> %203, %211, !dbg !14725
  %213 = fmul <4 x float> %_146.i.i.i.i9198289, %206, !dbg !14729
  %214 = fadd <4 x float> %205, %213, !dbg !14733
  %215 = bitcast <4 x i32> %history.i.i727.sroa.32.09261 to <4 x float>, !dbg !14737
  %216 = fmul <4 x float> %_151.i.i.i.i9248290, %215, !dbg !14741
  %217 = fadd <4 x float> %208, %216, !dbg !14742
  %218 = fmul <4 x float> %_154.i.i.i.i9278291, %215, !dbg !14746
  %219 = fadd <4 x float> %210, %218, !dbg !14750
  %220 = fmul <4 x float> %_157.i.i.i.i9308292, %215, !dbg !14754
  %221 = fadd <4 x float> %212, %220, !dbg !14758
  %222 = fmul <4 x float> %_160.i.i.i.i9338293, %215, !dbg !14762
  %223 = fadd <4 x float> %214, %222, !dbg !14766
  %224 = bitcast <4 x i32> %history.i.i727.sroa.35.09262 to <4 x float>, !dbg !14770
  %225 = fmul <4 x float> %_165.i.i.i.i9388294, %224, !dbg !14774
  %226 = fadd <4 x float> %217, %225, !dbg !14775
  %227 = fmul <4 x float> %_168.i.i.i.i9418295, %224, !dbg !14779
  %228 = fadd <4 x float> %219, %227, !dbg !14783
  %229 = fmul <4 x float> %_171.i.i.i.i9448296, %224, !dbg !14787
  %230 = fadd <4 x float> %221, %229, !dbg !14791
  %231 = fmul <4 x float> %_174.i.i.i.i9478297, %224, !dbg !14795
  %232 = fadd <4 x float> %223, %231, !dbg !14799
  %233 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %226), !dbg !14803
  %234 = fcmp olt <4 x float> %233, %125, !dbg !14807
  %235 = select <4 x i1> %234, <4 x float> %125, <4 x float> %233, !dbg !14811
  %236 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %228), !dbg !14803
  %237 = fcmp olt <4 x float> %236, %235, !dbg !14807
  %238 = select <4 x i1> %237, <4 x float> %235, <4 x float> %236, !dbg !14811
  %239 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %230), !dbg !14803
  %240 = fcmp olt <4 x float> %239, %238, !dbg !14807
  %241 = select <4 x i1> %240, <4 x float> %238, <4 x float> %239, !dbg !14811
  %242 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %232), !dbg !14803
  %243 = fcmp olt <4 x float> %242, %241, !dbg !14807
  %244 = select <4 x i1> %243, <4 x float> %241, <4 x float> %242, !dbg !14811
  %245 = add nuw nsw i32 %iter.i.i723.sroa.16.09263, 1, !dbg !14812
  %data.i4.i = getelementptr inbounds nuw float, ptr %peaks_left.i730, i32 %start1.i.i, !dbg !14813
  store <4 x float> %244, ptr %data.i4.i, align 4, !dbg !14816, !alias.scope !14821, !noalias !14825
  %exitcond.not = icmp eq i32 %245, %umax11018, !dbg !14378
  br i1 %exitcond.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960, label %bb5.i.i766, !dbg !14378

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960: ; preds = %bb5.i.i766, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %history.i.i727.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.0.0.lcssa11737, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %lanes.i3029.sroa.0.0.copyload, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.7.0.lcssa11755, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.0.09252, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.10.0.lcssa11773, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.7.09253, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.13.0.lcssa11791, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.10.09254, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.16.0.lcssa11809, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.13.09255, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.19.0.lcssa11827, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.16.09256, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.22.0.lcssa11845, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.19.09257, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.26.0.lcssa11863, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.22.09258, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.29.0.lcssa11881, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.26.09259, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.32.0.lcssa11899, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.29.09260, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.35.0.lcssa11917, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.32.09261, %bb5.i.i766 ], !dbg !14370
  %history.i.i727.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i727.sroa.38.0.lcssa11935, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ], [ %history.i.i727.sroa.35.09262, %bb5.i.i766 ], !dbg !14370
  br i1 %_2.i37109251.not, label %bb12.i748.loopexit, label %bb42.i966.lr.ph, !dbg !14829

bb42.i966.lr.ph:                                  ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i960
  %_64.i988 = load i32, ptr %101, align 4
  %_71.i1079 = load i32, ptr %119, align 4
  br label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3027, !dbg !14829

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3027: ; preds = %bb42.i966.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit
  %246 = phi <4 x float> [ %.lcssa1169111994, %bb42.i966.lr.ph ], [ %300, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %247 = phi <4 x float> [ %.lcssa1158511974, %bb42.i966.lr.ph ], [ %261, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %248 = phi <4 x float> [ %.lcssa1160311955, %bb42.i966.lr.ph ], [ %258, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %main_cursor.sroa.0.1.i9649290 = phi i32 [ %main_cursor.sroa.0.0.i7529416, %bb42.i966.lr.ph ], [ %spec.store.select7.i1081, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %ring_cursor.sroa.0.1.i9639289 = phi i32 [ %ring_cursor.sroa.0.0.i7519415, %bb42.i966.lr.ph ], [ %spec.store.select8.i1083, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %iter1.sroa.0.0.i9629288 = phi i32 [ 0, %bb42.i966.lr.ph ], [ %257, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %249 = fadd <4 x float> %248, splat (float -1.000000e+00), !dbg !14836
  %250 = fcmp ogt <4 x float> %249, zeroinitializer, !dbg !14844
  %251 = sext <4 x i1> %250 to <4 x i32>, !dbg !14848
  %_11.i14028231 = load <4 x float>, ptr %_51.i969, align 16, !dbg !14853, !alias.scope !14854, !noalias !14857
  %_12.i1403 = load <4 x i32>, ptr %95, align 16, !dbg !14862, !alias.scope !14854, !noalias !14857
  %252 = bitcast <4 x i32> %_12.i1403 to <4 x float>, !dbg !14863
  %253 = fadd <4 x float> %_11.i14028231, %252, !dbg !14867
  %254 = bitcast <4 x float> %253 to <16 x i8>, !dbg !14868
  %255 = bitcast <4 x i32> %251 to <16 x i8>, !dbg !14872
  %_4.i3718 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %254, <16 x i8> %_13.i14058232, <16 x i8> %255), !dbg !14873
  store <16 x i8> %_4.i3718, ptr %_51.i969, align 16, !dbg !14874, !alias.scope !14854, !noalias !14857
  %256 = bitcast <4 x i32> %_12.i1403 to <16 x i8>, !dbg !14875
  %_4.i3719 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %256, <16 x i8> zeroinitializer, <16 x i8> %255), !dbg !14879
  store <16 x i8> %_4.i3719, ptr %95, align 16, !dbg !14880, !alias.scope !14854, !noalias !14857
  %257 = add nuw nsw i32 %iter1.sroa.0.0.i9629288, 1, !dbg !14881
  %_47.i967 = add nuw nsw i32 %iter1.sroa.0.0.i9629288, %iter.sroa.0.0.i7499413, !dbg !14887
  %base.i968 = shl i32 %_47.i967, 2, !dbg !14887
  %258 = select <4 x i1> %250, <4 x float> %249, <4 x float> zeroinitializer, !dbg !14888
  %259 = fadd <4 x float> %247, splat (float -1.000000e+00), !dbg !14889
  %260 = fcmp ogt <4 x float> %259, zeroinitializer, !dbg !14894
  %261 = select <4 x i1> %260, <4 x float> %259, <4 x float> zeroinitializer, !dbg !14898
  %262 = sext <4 x i1> %260 to <4 x i32>, !dbg !14899
  %_11.i8235 = load <4 x float>, ptr %_52.i970, align 16, !dbg !14904, !alias.scope !14905, !noalias !14908
  %_12.i1390 = load <4 x i32>, ptr %98, align 16, !dbg !14910, !alias.scope !14905, !noalias !14908
  %263 = bitcast <4 x i32> %_12.i1390 to <4 x float>, !dbg !14911
  %264 = fadd <4 x float> %_11.i8235, %263, !dbg !14915
  %265 = bitcast <4 x float> %264 to <16 x i8>, !dbg !14916
  %266 = bitcast <4 x i32> %262 to <16 x i8>, !dbg !14920
  %_4.i3720 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %265, <16 x i8> %_13.i13928236, <16 x i8> %266), !dbg !14921
  store <16 x i8> %_4.i3720, ptr %_52.i970, align 16, !dbg !14922, !alias.scope !14905, !noalias !14908
  %267 = bitcast <4 x i32> %_12.i1390 to <16 x i8>, !dbg !14923
  %_4.i3721 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %267, <16 x i8> zeroinitializer, <16 x i8> %266), !dbg !14927
  store <16 x i8> %_4.i3721, ptr %98, align 16, !dbg !14928, !alias.scope !14905, !noalias !14908
  %_110.i979.idx = shl i32 %iter1.sroa.0.0.i9629288, 4, !dbg !14929
  %_110.i979 = getelementptr inbounds nuw i8, ptr %peaks_left.i730, i32 %_110.i979.idx, !dbg !14929
  %lanes.i3020.sroa.0.0.copyload = load <16 x i8>, ptr %_110.i979, align 4, !dbg !14941, !alias.scope !14946, !noalias !14950
  %_4.i3722 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %lanes.i3020.sroa.0.0.copyload, <16 x i8> %lanes.i3020.sroa.0.0.copyload, <16 x i8> %100), !dbg !14954
  %_111.i983 = icmp ugt i32 %base.i968, %left_io.1, !dbg !14960
  br i1 %_111.i983, label %bb46.i1089, label %bb47.i984, !dbg !14960, !prof !787

bb47.i984:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3027
  %_113.i985 = sub nuw nsw i32 %left_io.1, %base.i968, !dbg !14965
  %_117.i986 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i968, !dbg !14966
  %_8.i3014 = icmp samesign ugt i32 %_113.i985, 3, !dbg !14971
  br i1 %_8.i3014, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3018, label %bb2.i3015, !dbg !14971, !prof !1039

bb2.i3015:                                        ; preds = %bb47.i984
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_113.i985, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !14976, !noalias !14977
  unreachable, !dbg !14976

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3018: ; preds = %bb47.i984
  %lanes.i3011.sroa.0.0.copyload = load <4 x i32>, ptr %_117.i986, align 4, !dbg !14981, !alias.scope !14985, !noalias !14989
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14991), !dbg !14994
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14995), !dbg !14994
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14997), !dbg !14994
  %width.i.i990 = load i32, ptr %102, align 4, !dbg !14999, !alias.scope !15001, !noalias !15002, !noundef !10
  %268 = bitcast <16 x i8> %_4.i3722 to <4 x float>, !dbg !15009
  %269 = bitcast <16 x i8> %_4.i3718 to <4 x float>, !dbg !15014
  %270 = fcmp ogt <4 x float> %268, %269, !dbg !15015
  %271 = sext <4 x i1> %270 to <4 x i32>, !dbg !15015
  %272 = fdiv <4 x float> %269, %268, !dbg !15016
  %273 = bitcast <4 x float> %272 to <16 x i8>, !dbg !15020
  %274 = bitcast <4 x i32> %271 to <16 x i8>, !dbg !15024
  %_4.i3723 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %273, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %274), !dbg !15025
  %_158.1.i.i995 = load i32, ptr %103, align 4, !dbg !15026, !alias.scope !15001, !noalias !15002, !noundef !10
  %_22.i.i996 = mul i32 %width.i.i990, %ring_cursor.sroa.0.1.i9639289, !dbg !15027
  %_90.i.i997 = icmp ugt i32 %_22.i.i996, %_158.1.i.i995, !dbg !15028
  br i1 %_90.i.i997, label %bb34.i.i1088, label %bb35.i.i998, !dbg !15028, !prof !787

bb35.i.i998:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3018
  %_93.i.i1000 = sub nuw i32 %_158.1.i.i995, %_22.i.i996, !dbg !15031
  %_8.i3416 = icmp samesign ugt i32 %_93.i.i1000, 3, !dbg !15032
  br i1 %_8.i3416, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3419, label %bb2.i3417, !dbg !15032, !prof !1039

bb2.i3417:                                        ; preds = %bb35.i.i998
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i.i1000, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !15037, !noalias !15038
  unreachable, !dbg !15037

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3419: ; preds = %bb35.i.i998
  %_158.0.i.i999 = load ptr, ptr %104, align 4, !dbg !15026, !alias.scope !15001, !noalias !15002, !nonnull !10, !noundef !10
  %_97.i.i1001 = getelementptr inbounds nuw float, ptr %_158.0.i.i999, i32 %_22.i.i996, !dbg !15042
  store <16 x i8> %_4.i3723, ptr %_97.i.i1001, align 4, !dbg !15044, !alias.scope !15048, !noalias !15052
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15054), !dbg !15057
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15058), !dbg !15057
  %width.i = load i32, ptr %102, align 4, !dbg !15060, !alias.scope !15054, !noalias !15062, !noundef !10
  %275 = icmp eq i32 %width.i, 0, !dbg !15063
  br i1 %275, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i1205.lr.ph, !dbg !15063

bb29.i1205.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3419
  %_126.1.i = load i32, ptr %105, align 4, !alias.scope !15054, !noalias !15062, !noundef !10
  %_126.0.i = load ptr, ptr %106, align 4, !nonnull !10
  %276 = add i32 %ring_cursor.sroa.0.1.i9639289, 1
  %_21.not.i = icmp ult i32 %276, %_64.i988
  %277 = select i1 %_21.not.i, i32 0, i32 %_64.i988
  %start1.sroa.0.0.i = sub nuw i32 %276, %277
  %_128.1.i = load i32, ptr %103, align 4
  %_128.0.i = load ptr, ptr %104, align 4, !nonnull !10
  %_130.1.i = load i32, ptr %107, align 4
  %_130.0.i = load ptr, ptr %108, align 4, !nonnull !10
  %_132.1.i = load i32, ptr %109, align 4
  %_132.0.i = load ptr, ptr %110, align 4, !nonnull !10
  %_43.i = mul i32 %width.i, %start1.sroa.0.0.i
  br label %bb29.i1205, !dbg !15063

bb29.i1205:                                       ; preds = %bb29.i1205.lr.ph, %bb28.i
  %iter.sroa.0.0.idx.i9281 = phi i32 [ 0, %bb29.i1205.lr.ph ], [ %iter.sroa.0.0.add.i, %bb28.i ]
  %iter.sroa.4.0.i9280 = phi i32 [ 0, %bb29.i1205.lr.ph ], [ %_102.0.i, %bb28.i ]
  %iter.sroa.7.0.i9279 = phi i32 [ %width.i, %bb29.i1205.lr.ph ], [ %278, %bb28.i ]
  %iter.sroa.0.0.ptr.i9282 = getelementptr inbounds nuw i8, ptr %scratch.i731, i32 %iter.sroa.0.0.idx.i9281, !dbg !15065
  %278 = add i32 %iter.sroa.7.0.i9279, -1, !dbg !15065
  %_109.i = icmp eq i32 %iter.sroa.0.0.idx.i9281, 32, !dbg !15066
  br i1 %_109.i, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb33.i, !dbg !15070

bb33.i:                                           ; preds = %bb29.i1205
  %iter.sroa.0.0.add.i = add nuw nsw i32 %iter.sroa.0.0.idx.i9281, 4, !dbg !15071
  %_102.0.i = add nuw nsw i32 %iter.sroa.4.0.i9280, 1, !dbg !15073
  %exitcond11010.not = icmp eq i32 %iter.sroa.4.0.i9280, %_126.1.i, !dbg !15074
  br i1 %exitcond11010.not, label %panic.i, label %bb2.i1207, !dbg !15074

bb2.i1207:                                        ; preds = %bb33.i
  %279 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i, i32 %iter.sroa.4.0.i9280, !dbg !15074
  %shape.i = load i32, ptr %279, align 4, !dbg !15074, !noalias !15075, !noundef !10
  %280 = getelementptr inbounds nuw i8, ptr %279, i32 4, !dbg !15074
  %shape3.i = load i32, ptr %280, align 4, !dbg !15074, !noalias !15075, !noundef !10
  %281 = add i32 %shape3.i, %ring_cursor.sroa.0.1.i9639289, !dbg !15076
  %_18.not.i = icmp ult i32 %281, %_64.i988, !dbg !15077
  %282 = select i1 %_18.not.i, i32 0, i32 %_64.i988, !dbg !15077
  %spec.select.i = sub nuw i32 %281, %282, !dbg !15077
  %_25.i1208 = mul i32 %spec.select.i, %width.i, !dbg !15078
  %_24.i = add i32 %_25.i1208, %iter.sroa.4.0.i9280, !dbg !15078
  %_28.i1209 = icmp ult i32 %_24.i, %_128.1.i, !dbg !15079
  br i1 %_28.i1209, label %bb9.i, label %panic5.i, !dbg !15079

panic.i:                                          ; preds = %bb33.i
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i, i32 noundef %_126.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !15074, !noalias !15075
  unreachable, !dbg !15074

bb9.i:                                            ; preds = %bb2.i1207
  %283 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_24.i, !dbg !15079
  %284 = load float, ptr %283, align 4, !dbg !15079, !noalias !15075, !noundef !10
  %exitcond11011.not = icmp eq i32 %iter.sroa.4.0.i9280, %_130.1.i, !dbg !15080
  br i1 %exitcond11011.not, label %panic6.i, label %bb10.i, !dbg !15080

panic5.i:                                         ; preds = %bb2.i1207
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !15079, !noalias !15075
  unreachable, !dbg !15079

bb10.i:                                           ; preds = %bb9.i
  %285 = getelementptr inbounds nuw i32, ptr %_130.0.i, i32 %iter.sroa.4.0.i9280, !dbg !15080
  %_30.i = load i32, ptr %285, align 4, !dbg !15080, !noalias !15075, !noundef !10
  %286 = icmp eq i32 %_30.i, 0, !dbg !15081
  br i1 %286, label %bb14.i1212, label %bb12.i1210, !dbg !15081

panic6.i:                                         ; preds = %bb9.i
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i, i32 noundef %_130.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !15080, !noalias !15075
  unreachable, !dbg !15080

bb12.i1210:                                       ; preds = %bb10.i
  %_35.i = icmp ult i32 %iter.sroa.4.0.i9280, %_132.1.i, !dbg !15082
  br i1 %_35.i, label %bb13.i1211, label %panic7.i, !dbg !15082

bb14.i1212:                                       ; preds = %bb34.i, %bb13.i1211, %bb10.i
  %newest.sroa.0.0.i = phi float [ %284, %bb10.i ], [ %_33.i, %bb34.i ], [ %284, %bb13.i1211 ], !dbg !15083
  %exitcond11012.not = icmp eq i32 %iter.sroa.4.0.i9280, %_132.1.i, !dbg !15084
  br i1 %exitcond11012.not, label %panic8.i, label %bb15.i, !dbg !15084

bb13.i1211:                                       ; preds = %bb12.i1210
  %287 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i9280, !dbg !15082
  %_33.i = load float, ptr %287, align 4, !dbg !15082, !noalias !15075, !noundef !10
  %_116.i = fcmp olt float %_33.i, %284, !dbg !15085
  br i1 %_116.i, label %bb34.i, label %bb14.i1212, !dbg !15085

panic7.i:                                         ; preds = %bb12.i1210
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i9280, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !15082, !noalias !15075
  unreachable, !dbg !15082

bb34.i:                                           ; preds = %bb13.i1211
  br label %bb14.i1212, !dbg !15087

bb15.i:                                           ; preds = %bb14.i1212
  %288 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i9280, !dbg !15084
  store float %newest.sroa.0.0.i, ptr %288, align 4, !dbg !15084, !noalias !15075
  %_40.i1213 = add i32 %_30.i, 1, !dbg !15088
  %complete.i = icmp eq i32 %_40.i1213, %shape.i, !dbg !15088
  br i1 %complete.i, label %bb19.i, label %bb17.i1214, !dbg !15089

panic8.i:                                         ; preds = %bb14.i1212
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !15084, !noalias !15075
  unreachable, !dbg !15084

bb17.i1214:                                       ; preds = %bb15.i
  %_42.i = add i32 %iter.sroa.4.0.i9280, %_43.i, !dbg !15090
  %_45.i1215 = icmp ult i32 %_42.i, %_128.1.i, !dbg !15091
  br i1 %_45.i1215, label %bb27.i, label %panic9.i, !dbg !15091

panic9.i:                                         ; preds = %bb17.i1214
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !15091, !noalias !15075
  unreachable, !dbg !15091

bb27.i:                                           ; preds = %bb17.i1214
  %289 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_42.i, !dbg !15091
  %_41.i = load float, ptr %289, align 4, !dbg !15091, !noalias !15075, !noundef !10
  %_117.i1216 = fcmp olt float %_41.i, %newest.sroa.0.0.i, !dbg !15092
  %newest.sroa.0.1.i = select i1 %_117.i1216, float %_41.i, float %newest.sroa.0.0.i, !dbg !15092
  store float %newest.sroa.0.1.i, ptr %iter.sroa.0.0.ptr.i9282, align 4, !dbg !15094, !alias.scope !15058, !noalias !15095
  br label %bb28.i, !dbg !15096

bb28.i:                                           ; preds = %bb22.i, %bb19.i, %bb27.i
  %storemerge = phi i32 [ %_40.i1213, %bb27.i ], [ 0, %bb19.i ], [ 0, %bb22.i ], !dbg !15097
  store i32 %storemerge, ptr %285, align 4, !dbg !15097, !noalias !15075
  %290 = icmp eq i32 %278, 0, !dbg !15063
  br i1 %290, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i1205, !dbg !15063

bb19.i:                                           ; preds = %bb15.i
  store float %newest.sroa.0.0.i, ptr %iter.sroa.0.0.ptr.i9282, align 4, !dbg !15094, !alias.scope !15058, !noalias !15095
  %_118.i9275.not = icmp eq i32 %shape.i, 0, !dbg !15098
  br i1 %_118.i9275.not, label %bb28.i, label %bb40.i.preheader, !dbg !15102

bb40.i.preheader:                                 ; preds = %bb19.i
  %291 = load float, ptr %283, align 4, !dbg !15103, !noalias !15075, !noundef !10
  br label %bb40.i, !dbg !15104

bb40.i:                                           ; preds = %bb40.i.preheader, %bb22.i
  %iter2.sroa.0.0.i12209278 = phi i32 [ %_119.i, %bb22.i ], [ 0, %bb40.i.preheader ]
  %suffix.sroa.0.0.i9277 = phi float [ %suffix.sroa.0.1.i, %bb22.i ], [ %291, %bb40.i.preheader ]
  %end.sroa.0.1.i9276 = phi i32 [ %294, %bb22.i ], [ %spec.select.i, %bb40.i.preheader ]
  %_54.i1222 = mul i32 %end.sroa.0.1.i9276, %width.i, !dbg !15105
  %_53.i = add i32 %_54.i1222, %iter.sroa.4.0.i9280, !dbg !15105
  %_57.i = icmp ult i32 %_53.i, %_128.1.i, !dbg !15104
  br i1 %_57.i, label %bb22.i, label %panic13.i, !dbg !15104

panic13.i:                                        ; preds = %bb40.i
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !15104, !noalias !15075
  unreachable, !dbg !15104

bb22.i:                                           ; preds = %bb40.i
  %_119.i = add nuw i32 %iter2.sroa.0.0.i12209278, 1, !dbg !15106
  %292 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_53.i, !dbg !15104
  %_52.i1224 = load float, ptr %292, align 4, !dbg !15104, !noalias !15075, !noundef !10
  %_121.i = fcmp olt float %suffix.sroa.0.0.i9277, %_52.i1224, !dbg !15109
  %suffix.sroa.0.1.i = select i1 %_121.i, float %suffix.sroa.0.0.i9277, float %_52.i1224, !dbg !15109
  store float %suffix.sroa.0.1.i, ptr %292, align 4, !dbg !15111, !noalias !15075
  %293 = icmp eq i32 %end.sroa.0.1.i9276, 0, !dbg !15112
  %spec.store.select.i1225 = select i1 %293, i32 %_64.i988, i32 %end.sroa.0.1.i9276, !dbg !15112
  %294 = add i32 %spec.store.select.i1225, -1, !dbg !15113
  %exitcond11009.not = icmp eq i32 %_119.i, %shape.i, !dbg !15098
  br i1 %exitcond11009.not, label %bb28.i, label %bb40.i, !dbg !15102

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit: ; preds = %bb29.i1205, %bb28.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3419
  %lanes.i3004.sroa.0.0.copyload = load <4 x float>, ptr %scratch.i731, align 4, !dbg !15114, !alias.scope !15119, !noalias !15123
  %295 = fmul <4 x float> %lanes.i3004.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !15127
  %296 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %295), !dbg !15131
  %297 = fmul <4 x float> %296, splat (float 0x3F10000000000000), !dbg !15135
  %298 = icmp eq i32 %width.i.i990, 0, !dbg !15139
  %_163.1.i.i1039.pre = load i32, ptr %111, align 4, !dbg !15141, !alias.scope !15001, !noalias !15002
  br i1 %298, label %bb53.i.i1034, label %bb36.i.i1013.lr.ph, !dbg !15139

bb36.i.i1013.lr.ph:                               ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_159.1.i.i1018 = load i32, ptr %105, align 4, !alias.scope !15001, !noalias !15002, !noundef !10
  %_159.0.i.i1022 = load ptr, ptr %106, align 4, !nonnull !10
  %_161.0.i.i1032 = load ptr, ptr %112, align 4, !nonnull !10
  %exitcond11015.not = icmp eq i32 %_159.1.i.i1018, 0, !dbg !15142
  br i1 %exitcond11015.not, label %panic.i.i1020, label %bb14.i.i1021, !dbg !15142

bb34.i.i1088:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3018
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i996, i32 noundef %_158.1.i.i995, i32 noundef %_158.1.i.i995, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !15143, !noalias !15144
  unreachable, !dbg !15143

bb53.i.i1034.loopexit:                            ; preds = %bb18.i.i1031.7, %bb18.i.i1031.6, %bb18.i.i1031.5, %bb18.i.i1031.4, %bb18.i.i1031.3, %bb18.i.i1031.2, %bb18.i.i1031.1, %bb18.i.i1031
  %lanes.i2997.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i731, align 4, !dbg !15145, !alias.scope !15150, !noalias !15154
  br label %bb53.i.i1034, !dbg !15158

bb53.i.i1034:                                     ; preds = %bb53.i.i1034.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %lanes.i2997.sroa.0.0.copyload = phi <4 x float> [ %lanes.i2997.sroa.0.0.copyload.pre, %bb53.i.i1034.loopexit ], [ %lanes.i3004.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit ], !dbg !15145
  %299 = fadd <4 x float> %297, %246, !dbg !15159
  %300 = fsub <4 x float> %299, %lanes.i2997.sroa.0.0.copyload, !dbg !15163
  %_123.i.i1040 = icmp ugt i32 %_22.i.i996, %_163.1.i.i1039.pre, !dbg !15167
  br i1 %_123.i.i1040, label %bb41.i.i1087, label %bb42.i.i1041, !dbg !15167, !prof !787

bb14.i.i1021:                                     ; preds = %bb36.i.i1013.lr.ph
  %301 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1022, i32 8, !dbg !15142
  %_42.i.i1023 = load i32, ptr %301, align 4, !dbg !15142, !noalias !15170, !noundef !10
  %302 = add i32 %_42.i.i1023, %ring_cursor.sroa.0.1.i9639289, !dbg !15171
  %_45.not.i.i1024 = icmp ult i32 %302, %_64.i988, !dbg !15172
  %303 = select i1 %_45.not.i.i1024, i32 0, i32 %_64.i988, !dbg !15172
  %spec.select.i.i1025 = sub nuw i32 %302, %303, !dbg !15172
  %_49.i.i1026 = mul i32 %spec.select.i.i1025, %width.i.i990, !dbg !15173
  %_51.i.i1029 = icmp ult i32 %_49.i.i1026, %_163.1.i.i1039.pre, !dbg !15174
  br i1 %_51.i.i1029, label %bb18.i.i1031, label %panic1.i.i1030, !dbg !15174

panic.i.i1020:                                    ; preds = %bb36.i.i1013.7, %bb36.i.i1013.6, %bb36.i.i1013.5, %bb36.i.i1013.4, %bb36.i.i1013.3, %bb36.i.i1013.2, %bb36.i.i1013.1, %bb36.i.i1013.lr.ph
  %_159.1.i.i1018.lcssa.ph = phi i32 [ 7, %bb36.i.i1013.7 ], [ 6, %bb36.i.i1013.6 ], [ 5, %bb36.i.i1013.5 ], [ 4, %bb36.i.i1013.4 ], [ 3, %bb36.i.i1013.3 ], [ 2, %bb36.i.i1013.2 ], [ 1, %bb36.i.i1013.1 ], [ 0, %bb36.i.i1013.lr.ph ]
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i1018.lcssa.ph, i32 noundef %_159.1.i.i1018.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !15142, !noalias !15170
  unreachable, !dbg !15142

bb18.i.i1031:                                     ; preds = %bb14.i.i1021
  %304 = getelementptr inbounds nuw float, ptr %_161.0.i.i1032, i32 %_49.i.i1026, !dbg !15174
  %_47.i.i1033 = load float, ptr %304, align 4, !dbg !15174, !noalias !15170, !noundef !10
  store float %_47.i.i1033, ptr %scratch.i731, align 4, !dbg !15175, !alias.scope !14997, !noalias !15176
  %305 = icmp eq i32 %width.i.i990, 1, !dbg !15139
  br i1 %305, label %bb53.i.i1034.loopexit, label %bb36.i.i1013.1, !dbg !15139

bb36.i.i1013.1:                                   ; preds = %bb18.i.i1031
  %exitcond11015.1.not = icmp eq i32 %_159.1.i.i1018, 1, !dbg !15142
  br i1 %exitcond11015.1.not, label %panic.i.i1020, label %bb14.i.i1021.1, !dbg !15142

bb14.i.i1021.1:                                   ; preds = %bb36.i.i1013.1
  %306 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1022, i32 20, !dbg !15142
  %_42.i.i1023.1 = load i32, ptr %306, align 4, !dbg !15142, !noalias !15170, !noundef !10
  %307 = add i32 %_42.i.i1023.1, %ring_cursor.sroa.0.1.i9639289, !dbg !15171
  %_45.not.i.i1024.1 = icmp ult i32 %307, %_64.i988, !dbg !15172
  %308 = select i1 %_45.not.i.i1024.1, i32 0, i32 %_64.i988, !dbg !15172
  %spec.select.i.i1025.1 = sub nuw i32 %307, %308, !dbg !15172
  %_49.i.i1026.1 = mul i32 %spec.select.i.i1025.1, %width.i.i990, !dbg !15173
  %_48.i.i1027.1 = add i32 %_49.i.i1026.1, 1, !dbg !15173
  %_51.i.i1029.1 = icmp ult i32 %_48.i.i1027.1, %_163.1.i.i1039.pre, !dbg !15174
  br i1 %_51.i.i1029.1, label %bb18.i.i1031.1, label %panic1.i.i1030, !dbg !15174

bb18.i.i1031.1:                                   ; preds = %bb14.i.i1021.1
  %309 = getelementptr inbounds nuw float, ptr %_161.0.i.i1032, i32 %_48.i.i1027.1, !dbg !15174
  %_47.i.i1033.1 = load float, ptr %309, align 4, !dbg !15174, !noalias !15170, !noundef !10
  store float %_47.i.i1033.1, ptr %iter.sroa.0.0.ptr.i.i10129286.1, align 4, !dbg !15175, !alias.scope !14997, !noalias !15176
  %310 = icmp eq i32 %width.i.i990, 2, !dbg !15139
  br i1 %310, label %bb53.i.i1034.loopexit, label %bb36.i.i1013.2, !dbg !15139

bb36.i.i1013.2:                                   ; preds = %bb18.i.i1031.1
  %exitcond11015.2.not = icmp eq i32 %_159.1.i.i1018, 2, !dbg !15142
  br i1 %exitcond11015.2.not, label %panic.i.i1020, label %bb14.i.i1021.2, !dbg !15142

bb14.i.i1021.2:                                   ; preds = %bb36.i.i1013.2
  %311 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1022, i32 32, !dbg !15142
  %_42.i.i1023.2 = load i32, ptr %311, align 4, !dbg !15142, !noalias !15170, !noundef !10
  %312 = add i32 %_42.i.i1023.2, %ring_cursor.sroa.0.1.i9639289, !dbg !15171
  %_45.not.i.i1024.2 = icmp ult i32 %312, %_64.i988, !dbg !15172
  %313 = select i1 %_45.not.i.i1024.2, i32 0, i32 %_64.i988, !dbg !15172
  %spec.select.i.i1025.2 = sub nuw i32 %312, %313, !dbg !15172
  %_49.i.i1026.2 = mul i32 %spec.select.i.i1025.2, %width.i.i990, !dbg !15173
  %_48.i.i1027.2 = add i32 %_49.i.i1026.2, 2, !dbg !15173
  %_51.i.i1029.2 = icmp ult i32 %_48.i.i1027.2, %_163.1.i.i1039.pre, !dbg !15174
  br i1 %_51.i.i1029.2, label %bb18.i.i1031.2, label %panic1.i.i1030, !dbg !15174

bb18.i.i1031.2:                                   ; preds = %bb14.i.i1021.2
  %314 = getelementptr inbounds nuw float, ptr %_161.0.i.i1032, i32 %_48.i.i1027.2, !dbg !15174
  %_47.i.i1033.2 = load float, ptr %314, align 4, !dbg !15174, !noalias !15170, !noundef !10
  store float %_47.i.i1033.2, ptr %iter.sroa.0.0.ptr.i.i10129286.2, align 4, !dbg !15175, !alias.scope !14997, !noalias !15176
  %315 = icmp eq i32 %width.i.i990, 3, !dbg !15139
  br i1 %315, label %bb53.i.i1034.loopexit, label %bb36.i.i1013.3, !dbg !15139

bb36.i.i1013.3:                                   ; preds = %bb18.i.i1031.2
  %exitcond11015.3.not = icmp eq i32 %_159.1.i.i1018, 3, !dbg !15142
  br i1 %exitcond11015.3.not, label %panic.i.i1020, label %bb14.i.i1021.3, !dbg !15142

bb14.i.i1021.3:                                   ; preds = %bb36.i.i1013.3
  %316 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1022, i32 44, !dbg !15142
  %_42.i.i1023.3 = load i32, ptr %316, align 4, !dbg !15142, !noalias !15170, !noundef !10
  %317 = add i32 %_42.i.i1023.3, %ring_cursor.sroa.0.1.i9639289, !dbg !15171
  %_45.not.i.i1024.3 = icmp ult i32 %317, %_64.i988, !dbg !15172
  %318 = select i1 %_45.not.i.i1024.3, i32 0, i32 %_64.i988, !dbg !15172
  %spec.select.i.i1025.3 = sub nuw i32 %317, %318, !dbg !15172
  %_49.i.i1026.3 = mul i32 %spec.select.i.i1025.3, %width.i.i990, !dbg !15173
  %_48.i.i1027.3 = add i32 %_49.i.i1026.3, 3, !dbg !15173
  %_51.i.i1029.3 = icmp ult i32 %_48.i.i1027.3, %_163.1.i.i1039.pre, !dbg !15174
  br i1 %_51.i.i1029.3, label %bb18.i.i1031.3, label %panic1.i.i1030, !dbg !15174

bb18.i.i1031.3:                                   ; preds = %bb14.i.i1021.3
  %319 = getelementptr inbounds nuw float, ptr %_161.0.i.i1032, i32 %_48.i.i1027.3, !dbg !15174
  %_47.i.i1033.3 = load float, ptr %319, align 4, !dbg !15174, !noalias !15170, !noundef !10
  store float %_47.i.i1033.3, ptr %iter.sroa.0.0.ptr.i.i10129286.3, align 4, !dbg !15175, !alias.scope !14997, !noalias !15176
  %320 = icmp eq i32 %width.i.i990, 4, !dbg !15139
  br i1 %320, label %bb53.i.i1034.loopexit, label %bb36.i.i1013.4, !dbg !15139

bb36.i.i1013.4:                                   ; preds = %bb18.i.i1031.3
  %exitcond11015.4.not = icmp eq i32 %_159.1.i.i1018, 4, !dbg !15142
  br i1 %exitcond11015.4.not, label %panic.i.i1020, label %bb14.i.i1021.4, !dbg !15142

bb14.i.i1021.4:                                   ; preds = %bb36.i.i1013.4
  %321 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1022, i32 56, !dbg !15142
  %_42.i.i1023.4 = load i32, ptr %321, align 4, !dbg !15142, !noalias !15170, !noundef !10
  %322 = add i32 %_42.i.i1023.4, %ring_cursor.sroa.0.1.i9639289, !dbg !15171
  %_45.not.i.i1024.4 = icmp ult i32 %322, %_64.i988, !dbg !15172
  %323 = select i1 %_45.not.i.i1024.4, i32 0, i32 %_64.i988, !dbg !15172
  %spec.select.i.i1025.4 = sub nuw i32 %322, %323, !dbg !15172
  %_49.i.i1026.4 = mul i32 %spec.select.i.i1025.4, %width.i.i990, !dbg !15173
  %_48.i.i1027.4 = add i32 %_49.i.i1026.4, 4, !dbg !15173
  %_51.i.i1029.4 = icmp ult i32 %_48.i.i1027.4, %_163.1.i.i1039.pre, !dbg !15174
  br i1 %_51.i.i1029.4, label %bb18.i.i1031.4, label %panic1.i.i1030, !dbg !15174

bb18.i.i1031.4:                                   ; preds = %bb14.i.i1021.4
  %324 = getelementptr inbounds nuw float, ptr %_161.0.i.i1032, i32 %_48.i.i1027.4, !dbg !15174
  %_47.i.i1033.4 = load float, ptr %324, align 4, !dbg !15174, !noalias !15170, !noundef !10
  store float %_47.i.i1033.4, ptr %iter.sroa.0.0.ptr.i.i10129286.4, align 4, !dbg !15175, !alias.scope !14997, !noalias !15176
  %325 = icmp eq i32 %width.i.i990, 5, !dbg !15139
  br i1 %325, label %bb53.i.i1034.loopexit, label %bb36.i.i1013.5, !dbg !15139

bb36.i.i1013.5:                                   ; preds = %bb18.i.i1031.4
  %exitcond11015.5.not = icmp eq i32 %_159.1.i.i1018, 5, !dbg !15142
  br i1 %exitcond11015.5.not, label %panic.i.i1020, label %bb14.i.i1021.5, !dbg !15142

bb14.i.i1021.5:                                   ; preds = %bb36.i.i1013.5
  %326 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1022, i32 68, !dbg !15142
  %_42.i.i1023.5 = load i32, ptr %326, align 4, !dbg !15142, !noalias !15170, !noundef !10
  %327 = add i32 %_42.i.i1023.5, %ring_cursor.sroa.0.1.i9639289, !dbg !15171
  %_45.not.i.i1024.5 = icmp ult i32 %327, %_64.i988, !dbg !15172
  %328 = select i1 %_45.not.i.i1024.5, i32 0, i32 %_64.i988, !dbg !15172
  %spec.select.i.i1025.5 = sub nuw i32 %327, %328, !dbg !15172
  %_49.i.i1026.5 = mul i32 %spec.select.i.i1025.5, %width.i.i990, !dbg !15173
  %_48.i.i1027.5 = add i32 %_49.i.i1026.5, 5, !dbg !15173
  %_51.i.i1029.5 = icmp ult i32 %_48.i.i1027.5, %_163.1.i.i1039.pre, !dbg !15174
  br i1 %_51.i.i1029.5, label %bb18.i.i1031.5, label %panic1.i.i1030, !dbg !15174

bb18.i.i1031.5:                                   ; preds = %bb14.i.i1021.5
  %329 = getelementptr inbounds nuw float, ptr %_161.0.i.i1032, i32 %_48.i.i1027.5, !dbg !15174
  %_47.i.i1033.5 = load float, ptr %329, align 4, !dbg !15174, !noalias !15170, !noundef !10
  store float %_47.i.i1033.5, ptr %iter.sroa.0.0.ptr.i.i10129286.5, align 4, !dbg !15175, !alias.scope !14997, !noalias !15176
  %330 = icmp eq i32 %width.i.i990, 6, !dbg !15139
  br i1 %330, label %bb53.i.i1034.loopexit, label %bb36.i.i1013.6, !dbg !15139

bb36.i.i1013.6:                                   ; preds = %bb18.i.i1031.5
  %exitcond11015.6.not = icmp eq i32 %_159.1.i.i1018, 6, !dbg !15142
  br i1 %exitcond11015.6.not, label %panic.i.i1020, label %bb14.i.i1021.6, !dbg !15142

bb14.i.i1021.6:                                   ; preds = %bb36.i.i1013.6
  %331 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1022, i32 80, !dbg !15142
  %_42.i.i1023.6 = load i32, ptr %331, align 4, !dbg !15142, !noalias !15170, !noundef !10
  %332 = add i32 %_42.i.i1023.6, %ring_cursor.sroa.0.1.i9639289, !dbg !15171
  %_45.not.i.i1024.6 = icmp ult i32 %332, %_64.i988, !dbg !15172
  %333 = select i1 %_45.not.i.i1024.6, i32 0, i32 %_64.i988, !dbg !15172
  %spec.select.i.i1025.6 = sub nuw i32 %332, %333, !dbg !15172
  %_49.i.i1026.6 = mul i32 %spec.select.i.i1025.6, %width.i.i990, !dbg !15173
  %_48.i.i1027.6 = add i32 %_49.i.i1026.6, 6, !dbg !15173
  %_51.i.i1029.6 = icmp ult i32 %_48.i.i1027.6, %_163.1.i.i1039.pre, !dbg !15174
  br i1 %_51.i.i1029.6, label %bb18.i.i1031.6, label %panic1.i.i1030, !dbg !15174

bb18.i.i1031.6:                                   ; preds = %bb14.i.i1021.6
  %334 = getelementptr inbounds nuw float, ptr %_161.0.i.i1032, i32 %_48.i.i1027.6, !dbg !15174
  %_47.i.i1033.6 = load float, ptr %334, align 4, !dbg !15174, !noalias !15170, !noundef !10
  store float %_47.i.i1033.6, ptr %iter.sroa.0.0.ptr.i.i10129286.6, align 4, !dbg !15175, !alias.scope !14997, !noalias !15176
  %335 = icmp eq i32 %width.i.i990, 7, !dbg !15139
  br i1 %335, label %bb53.i.i1034.loopexit, label %bb36.i.i1013.7, !dbg !15139

bb36.i.i1013.7:                                   ; preds = %bb18.i.i1031.6
  %exitcond11015.7.not = icmp eq i32 %_159.1.i.i1018, 7, !dbg !15142
  br i1 %exitcond11015.7.not, label %panic.i.i1020, label %bb14.i.i1021.7, !dbg !15142

bb14.i.i1021.7:                                   ; preds = %bb36.i.i1013.7
  %336 = getelementptr inbounds nuw i8, ptr %_159.0.i.i1022, i32 92, !dbg !15142
  %_42.i.i1023.7 = load i32, ptr %336, align 4, !dbg !15142, !noalias !15170, !noundef !10
  %337 = add i32 %_42.i.i1023.7, %ring_cursor.sroa.0.1.i9639289, !dbg !15171
  %_45.not.i.i1024.7 = icmp ult i32 %337, %_64.i988, !dbg !15172
  %338 = select i1 %_45.not.i.i1024.7, i32 0, i32 %_64.i988, !dbg !15172
  %spec.select.i.i1025.7 = sub nuw i32 %337, %338, !dbg !15172
  %_49.i.i1026.7 = mul i32 %spec.select.i.i1025.7, %width.i.i990, !dbg !15173
  %_48.i.i1027.7 = add i32 %_49.i.i1026.7, 7, !dbg !15173
  %_51.i.i1029.7 = icmp ult i32 %_48.i.i1027.7, %_163.1.i.i1039.pre, !dbg !15174
  br i1 %_51.i.i1029.7, label %bb18.i.i1031.7, label %panic1.i.i1030, !dbg !15174

bb18.i.i1031.7:                                   ; preds = %bb14.i.i1021.7
  %339 = getelementptr inbounds nuw float, ptr %_161.0.i.i1032, i32 %_48.i.i1027.7, !dbg !15174
  %_47.i.i1033.7 = load float, ptr %339, align 4, !dbg !15174, !noalias !15170, !noundef !10
  store float %_47.i.i1033.7, ptr %iter.sroa.0.0.ptr.i.i10129286.7, align 4, !dbg !15175, !alias.scope !14997, !noalias !15176
  br label %bb53.i.i1034.loopexit, !dbg !15139

panic1.i.i1030:                                   ; preds = %bb14.i.i1021.7, %bb14.i.i1021.6, %bb14.i.i1021.5, %bb14.i.i1021.4, %bb14.i.i1021.3, %bb14.i.i1021.2, %bb14.i.i1021.1, %bb14.i.i1021
  %_48.i.i1027.lcssa.ph = phi i32 [ %_48.i.i1027.7, %bb14.i.i1021.7 ], [ %_48.i.i1027.6, %bb14.i.i1021.6 ], [ %_48.i.i1027.5, %bb14.i.i1021.5 ], [ %_48.i.i1027.4, %bb14.i.i1021.4 ], [ %_48.i.i1027.3, %bb14.i.i1021.3 ], [ %_48.i.i1027.2, %bb14.i.i1021.2 ], [ %_48.i.i1027.1, %bb14.i.i1021.1 ], [ %_49.i.i1026, %bb14.i.i1021 ]
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i1027.lcssa.ph, i32 noundef %_163.1.i.i1039.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !15174, !noalias !15170
  unreachable, !dbg !15174

bb42.i.i1041:                                     ; preds = %bb53.i.i1034
  %_126.i.i1043 = sub nuw i32 %_163.1.i.i1039.pre, %_22.i.i996, !dbg !15177
  %_8.i3411 = icmp samesign ugt i32 %_126.i.i1043, 3, !dbg !15178
  br i1 %_8.i3411, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3414, label %bb2.i3412, !dbg !15178, !prof !1039

bb2.i3412:                                        ; preds = %bb42.i.i1041
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i.i1043, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !15183, !noalias !15184
  unreachable, !dbg !15183

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3414: ; preds = %bb42.i.i1041
  %_163.0.i.i1042 = load ptr, ptr %112, align 4, !dbg !15141, !alias.scope !15001, !noalias !15002, !nonnull !10, !noundef !10
  %_130.i.i1044 = getelementptr inbounds nuw float, ptr %_163.0.i.i1042, i32 %_22.i.i996, !dbg !15188
  store <4 x float> %297, ptr %_130.i.i1044, align 4, !dbg !15190, !alias.scope !15194, !noalias !15198
  %_66.i.i10498244 = load <4 x float>, ptr %115, align 16, !dbg !15200, !alias.scope !14991, !noalias !15201
  %340 = fdiv <4 x float> %300, %_62.i.i10468243, !dbg !15202
  %341 = fsub <4 x float> splat (float 1.000000e+00), %340, !dbg !15206
  %342 = fsub <4 x float> %341, %_66.i.i10498244, !dbg !15210
  %343 = bitcast <16 x i8> %_4.i3720 to <4 x float>, !dbg !15214
  %344 = fmul <4 x float> %342, %343, !dbg !15218
  %345 = fadd <4 x float> %_66.i.i10498244, %344, !dbg !15219
  %346 = fcmp olt <4 x float> %345, %341, !dbg !15222
  %347 = select <4 x i1> %346, <4 x float> %341, <4 x float> %345, !dbg !15226
  %348 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %347), !dbg !15227
  %349 = fcmp uge <4 x float> %348, splat (float 0x3BC79CA100000000), !dbg !15232
  %350 = bitcast <4 x float> %347 to <4 x i32>, !dbg !15237
  %351 = select <4 x i1> %349, <4 x i32> %350, <4 x i32> zeroinitializer, !dbg !15237
  store <4 x i32> %351, ptr %115, align 16, !dbg !15240, !alias.scope !14991, !noalias !15201
  %352 = bitcast <4 x i32> %351 to <4 x float>, !dbg !15241
  %353 = fsub <4 x float> splat (float 1.000000e+00), %352, !dbg !15245
  %_164.1.i.i1059 = load i32, ptr %116, align 4, !dbg !15246, !alias.scope !15001, !noalias !15002, !noundef !10
  %_74.i.i1060 = mul i32 %width.i.i990, %main_cursor.sroa.0.1.i9649290, !dbg !15247
  %_134.i.i1061 = icmp ugt i32 %_74.i.i1060, %_164.1.i.i1059, !dbg !15248
  br i1 %_134.i.i1061, label %bb47.i.i1086, label %bb48.i.i1062, !dbg !15248, !prof !787

bb41.i.i1087:                                     ; preds = %bb53.i.i1034
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i996, i32 noundef %_163.1.i.i1039.pre, i32 noundef %_163.1.i.i1039.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !15251, !noalias !15170
  unreachable, !dbg !15251

bb48.i.i1062:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3414
  %_137.i.i1064 = sub nuw i32 %_164.1.i.i1059, %_74.i.i1060, !dbg !15252
  %_8.i2991 = icmp samesign ugt i32 %_137.i.i1064, 3, !dbg !15253
  br i1 %_8.i2991, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit, label %bb2.i2992, !dbg !15253, !prof !1039

bb2.i2992:                                        ; preds = %bb48.i.i1062
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i.i1064, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !15258, !noalias !15259
  unreachable, !dbg !15258

bb47.i.i1086:                                     ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3414
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i1060, i32 noundef %_164.1.i.i1059, i32 noundef %_164.1.i.i1059, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !15263, !noalias !15170
  unreachable, !dbg !15263

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit: ; preds = %bb48.i.i1062
  %_164.0.i.i1063 = load ptr, ptr %117, align 4, !dbg !15246, !alias.scope !15001, !noalias !15002, !nonnull !10, !noundef !10
  %_141.i.i1065 = getelementptr inbounds nuw float, ptr %_164.0.i.i1063, i32 %_74.i.i1060, !dbg !15264
  %lanes.i2988.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i.i1065, align 4, !dbg !15266, !alias.scope !15270, !noalias !15274
  store <4 x i32> %lanes.i3011.sroa.0.0.copyload, ptr %_141.i.i1065, align 4, !dbg !15276, !alias.scope !15281, !noalias !15285
  %354 = bitcast <4 x i32> %lanes.i2988.sroa.0.0.copyload to <4 x float>, !dbg !15289
  %355 = fmul <4 x float> %353, %354, !dbg !15293
  %356 = bitcast <4 x i32> %lanes.i2988.sroa.0.0.copyload to <16 x i8>, !dbg !15294
  %357 = bitcast <4 x float> %355 to <16 x i8>, !dbg !15298
  %_4.i3724 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %356, <16 x i8> %357, <16 x i8> %118), !dbg !15299
  store <16 x i8> %_4.i3724, ptr %_117.i986, align 4, !dbg !15300, !alias.scope !15305, !noalias !15309
  %358 = add i32 %main_cursor.sroa.0.1.i9649290, 1, !dbg !15313
  %_69.i1080 = icmp eq i32 %358, %_71.i1079, !dbg !15314
  %spec.store.select7.i1081 = select i1 %_69.i1080, i32 0, i32 %358, !dbg !15314
  %359 = add i32 %ring_cursor.sroa.0.1.i9639289, 1, !dbg !15315
  %_72.i1082 = icmp eq i32 %359, %_64.i988, !dbg !15316
  %spec.store.select8.i1083 = select i1 %_72.i1082, i32 0, i32 %359, !dbg !15316
  %exitcond11019.not = icmp eq i32 %257, %umax11018, !dbg !15317
  br i1 %exitcond11019.not, label %bb12.i748.loopexit, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3027, !dbg !14829

bb46.i1089:                                       ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3027
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311955, ptr %94, align 16
  store <4 x float> %.lcssa1158511974, ptr %97, align 16
  store <4 x float> %.lcssa1169111994, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i968, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_16fe79fe907693415948d189acefd4a9) #32, !dbg !15320, !noalias !14373
  unreachable, !dbg !15320

_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit: ; preds = %bb12.i748.loopexit
  store <4 x i32> %history.i.i727.sroa.0.0.lcssa, ptr %hot_left.i733, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.7.0.lcssa, ptr %history.i.i727.sroa.7.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.10.0.lcssa, ptr %history.i.i727.sroa.10.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.13.0.lcssa, ptr %history.i.i727.sroa.13.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.16.0.lcssa, ptr %history.i.i727.sroa.16.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.19.0.lcssa, ptr %history.i.i727.sroa.19.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.22.0.lcssa, ptr %history.i.i727.sroa.22.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.26.0.lcssa, ptr %history.i.i727.sroa.26.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.29.0.lcssa, ptr %history.i.i727.sroa.29.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.32.0.lcssa, ptr %history.i.i727.sroa.32.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.35.0.lcssa, ptr %history.i.i727.sroa.35.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x i32> %history.i.i727.sroa.38.0.lcssa, ptr %history.i.i727.sroa.38.0.hot_left.i733.sroa_idx, align 16, !dbg !14370
  store <4 x float> %.lcssa1160311954, ptr %94, align 16
  store <4 x float> %.lcssa1158511973, ptr %97, align 16
  store <4 x float> %.lcssa1169111993, ptr %113, align 16
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !15321

_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, %bb8.i
  %ring_cursor.sroa.0.0.i751.lcssa = phi i32 [ %_27.i742, %bb8.i ], [ %ring_cursor.sroa.0.1.i963.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !14319
  %main_cursor.sroa.0.0.i752.lcssa = phi i32 [ %_25.i741, %bb8.i ], [ %main_cursor.sroa.0.1.i964.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !14316
  call void @llvm.lifetime.start.p0(ptr nonnull %_75.i728), !dbg !15321, !noalias !14304
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 16 dereferenceable(368) %_75.i728, ptr noundef nonnull align 16 dereferenceable(368) %hot_left.i733, i32 368, i1 false), !dbg !15321, !noalias !14304
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %_75.i728, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24) #31, !dbg !15322, !noalias !14373
  call void @llvm.lifetime.end.p0(ptr nonnull %_75.i728), !dbg !15323, !noalias !14304
  store i32 %main_cursor.sroa.0.0.i752.lcssa, ptr %_25, align 4, !dbg !15324, !alias.scope !14298, !noalias !14318
  store i32 %ring_cursor.sroa.0.0.i751.lcssa, ptr %56, align 4, !dbg !15325, !alias.scope !14298, !noalias !14318
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i730), !dbg !15326, !noalias !14304
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i731), !dbg !15327, !noalias !14304
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i733), !dbg !15328, !noalias !14304
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !14295

bb4.i:                                            ; preds = %bb1.i3.i, %bb2.i3653, %bb2.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15329), !dbg !15332
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15333), !dbg !15332
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15335), !dbg !15332
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15337), !dbg !15332
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i41, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #31, !dbg !15339
  %360 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !15343
  %361 = load i8, ptr %360, align 16, !dbg !15343, !range !4667, !alias.scope !15329, !noalias !15347, !noundef !10
  %362 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !15349
  %363 = load i8, ptr %362, align 1, !dbg !15349, !range !4667, !alias.scope !15329, !noalias !15347, !noundef !10
  %364 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !15351
  %ring.i49 = load i32, ptr %364, align 4, !dbg !15351, !alias.scope !15333, !noalias !15353, !noundef !10
  %365 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !15354
  %main.i50 = load i32, ptr %365, align 4, !dbg !15354, !alias.scope !15333, !noalias !15353, !noundef !10
  %_26.i51 = load i32, ptr %_25, align 4, !dbg !15356, !alias.scope !15337, !noalias !15358, !noundef !10
  %366 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !15359
  %_27.i52 = load i32, ptr %366, align 4, !dbg !15359, !alias.scope !15337, !noalias !15358, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i39), !dbg !15361, !noalias !15363
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i39, i8 0, i32 1024, i1 false), !noalias !15363
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i38), !dbg !15364, !noalias !15363
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_left.i38, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24, i32 %ring.i49, i32 %main.i50) #31, !dbg !15366
  %_110.not.i619653 = icmp eq i32 %frames, 0, !dbg !15367
  br i1 %_110.not.i619653, label %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge, label %bb32.i62.lr.ph, !dbg !15367

bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge: ; preds = %bb4.i
  %.phi.trans.insert11108 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 16
  %left_phase.i428.pre = load i32, ptr %.phi.trans.insert11108, align 16, !dbg !15377, !noalias !15363
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !15367

bb32.i62.lr.ph:                                   ; preds = %bb4.i
  %_23.i47 = trunc nuw i8 %363 to i1, !dbg !15349
  %spec.store.select18.i48 = select i1 %_23.i47, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !15349
  %_22.i45 = trunc nuw i8 %361 to i1, !dbg !15343
  %link.sroa.0.0.i46 = select i1 %_22.i45, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !15343
  %d9.i3725 = lshr i32 %frames, 5, !dbg !15378
  %r2.i3726 = and i32 %frames, 31, !dbg !15385
  %_19.not.i3727 = icmp ne i32 %r2.i3726, 0, !dbg !15386
  %367 = zext i1 %_19.not.i3727 to i32, !dbg !15386
  %yield_count.sroa.0.0.i3728 = add nuw nsw i32 %d9.i3725, %367, !dbg !15386
  %history.i.i25.sroa.7.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 16
  %history.i.i25.sroa.10.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 32
  %history.i.i25.sroa.13.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 48
  %history.i.i25.sroa.16.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 64
  %history.i.i25.sroa.19.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 80
  %history.i.i25.sroa.22.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 96
  %history.i.i25.sroa.26.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 112
  %history.i.i25.sroa.29.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 128
  %history.i.i25.sroa.32.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 144
  %history.i.i25.sroa.35.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 160
  %history.i.i25.sroa.38.0.hot_left.i41.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 176
  %368 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %369 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %370 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i.i104 = getelementptr inbounds nuw i8, ptr %self, i32 80
  %371 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %372 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %373 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i.i118 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %374 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %375 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %376 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i.i132 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %377 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %378 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %379 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i.i146 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %380 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %381 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %382 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i.i160 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %383 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %384 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %385 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i.i174 = getelementptr inbounds nuw i8, ptr %self, i32 400
  %386 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %387 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %388 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i.i188 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %389 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %390 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %391 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i.i202 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %392 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %393 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %394 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i.i216 = getelementptr inbounds nuw i8, ptr %self, i32 592
  %395 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %396 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %397 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i.i230 = getelementptr inbounds nuw i8, ptr %self, i32 656
  %398 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %399 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %400 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i.i244 = getelementptr inbounds nuw i8, ptr %self, i32 720
  %401 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %402 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %403 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %404 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 20
  %_54.i36.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 24
  %_54.i36.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 28
  %_82.i308 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 192
  %_83.i309 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 256
  %405 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 240
  %406 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 224
  %407 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 208
  %408 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 304
  %409 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 288
  %410 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 272
  %411 = bitcast <4 x i32> %link.sroa.0.0.i46 to <16 x i8>
  %412 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 32
  %413 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 36
  %_22.i.i347 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 16
  %414 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 40
  %415 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 44
  %416 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 336
  %417 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 352
  %418 = getelementptr inbounds nuw i8, ptr %hot_left.i41, i32 320
  %419 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 52
  %420 = getelementptr inbounds nuw i8, ptr %uniform_left.i38, i32 48
  %421 = bitcast <4 x i32> %spec.store.select18.i48 to <16 x i8>
  %_22.i.i347.promoted = load i32, ptr %_22.i.i347, align 4
  %_11.i.i.i.i928398 = load <4 x float>, ptr %_22, align 16
  %_14.i.i.i.i958399 = load <4 x float>, ptr %368, align 16
  %_17.i.i.i.i988400 = load <4 x float>, ptr %369, align 16
  %_20.i.i.i.i1018401 = load <4 x float>, ptr %370, align 16
  %_25.i.i.i.i1068402 = load <4 x float>, ptr %row1.i.i.i.i104, align 16
  %_28.i.i.i.i1098403 = load <4 x float>, ptr %371, align 16
  %_31.i.i.i.i1128404 = load <4 x float>, ptr %372, align 16
  %_34.i.i.i.i1158405 = load <4 x float>, ptr %373, align 16
  %_39.i.i.i.i1208406 = load <4 x float>, ptr %row3.i.i.i.i118, align 16
  %_42.i.i.i.i1238407 = load <4 x float>, ptr %374, align 16
  %_45.i.i.i.i1268408 = load <4 x float>, ptr %375, align 16
  %_48.i.i.i.i1298409 = load <4 x float>, ptr %376, align 16
  %_53.i.i.i.i1348410 = load <4 x float>, ptr %row5.i.i.i.i132, align 16
  %_56.i.i.i.i1378411 = load <4 x float>, ptr %377, align 16
  %_59.i.i.i.i1408412 = load <4 x float>, ptr %378, align 16
  %_62.i.i.i.i1438413 = load <4 x float>, ptr %379, align 16
  %_67.i.i.i.i1488414 = load <4 x float>, ptr %row7.i.i.i.i146, align 16
  %_70.i.i.i.i1518415 = load <4 x float>, ptr %380, align 16
  %_73.i.i.i.i1548416 = load <4 x float>, ptr %381, align 16
  %_76.i.i.i.i1578417 = load <4 x float>, ptr %382, align 16
  %_81.i.i.i.i1628418 = load <4 x float>, ptr %row9.i.i.i.i160, align 16
  %_84.i.i.i.i1658419 = load <4 x float>, ptr %383, align 16
  %_87.i.i.i.i1688420 = load <4 x float>, ptr %384, align 16
  %_90.i.i.i.i1718421 = load <4 x float>, ptr %385, align 16
  %_95.i.i.i.i1768422 = load <4 x float>, ptr %row11.i.i.i.i174, align 16
  %_98.i.i.i.i1798423 = load <4 x float>, ptr %386, align 16
  %_101.i.i.i.i1828424 = load <4 x float>, ptr %387, align 16
  %_104.i.i.i.i1858425 = load <4 x float>, ptr %388, align 16
  %_109.i.i.i.i1908426 = load <4 x float>, ptr %row13.i.i.i.i188, align 16
  %_112.i.i.i.i1938427 = load <4 x float>, ptr %389, align 16
  %_115.i.i.i.i1968428 = load <4 x float>, ptr %390, align 16
  %_118.i.i.i.i1998429 = load <4 x float>, ptr %391, align 16
  %_123.i.i.i.i2048430 = load <4 x float>, ptr %row15.i.i.i.i202, align 16
  %_126.i.i.i.i2078431 = load <4 x float>, ptr %392, align 16
  %_129.i.i.i.i2108432 = load <4 x float>, ptr %393, align 16
  %_132.i.i.i.i2138433 = load <4 x float>, ptr %394, align 16
  %_137.i.i.i.i2188434 = load <4 x float>, ptr %row17.i.i.i.i216, align 16
  %_140.i.i.i.i2218435 = load <4 x float>, ptr %395, align 16
  %_143.i.i.i.i2248436 = load <4 x float>, ptr %396, align 16
  %_146.i.i.i.i2278437 = load <4 x float>, ptr %397, align 16
  %_151.i.i.i.i2328438 = load <4 x float>, ptr %row19.i.i.i.i230, align 16
  %_154.i.i.i.i2358439 = load <4 x float>, ptr %398, align 16
  %_157.i.i.i.i2388440 = load <4 x float>, ptr %399, align 16
  %_160.i.i.i.i2418441 = load <4 x float>, ptr %400, align 16
  %_165.i.i.i.i2468442 = load <4 x float>, ptr %row21.i.i.i.i244, align 16
  %_168.i.i.i.i2498443 = load <4 x float>, ptr %401, align 16
  %_171.i.i.i.i2528444 = load <4 x float>, ptr %402, align 16
  %_174.i.i.i.i2558445 = load <4 x float>, ptr %403, align 16
  %_54.i36.sroa.3.0.copyload = load i32, ptr %_54.i36.sroa.3.0..sroa_idx, align 4
  %_54.i36.sroa.4.0.copyload = load i32, ptr %_54.i36.sroa.4.0..sroa_idx, align 4
  %_54.0.i.i332 = load ptr, ptr %412, align 16, !nonnull !10, !align !10173
  %_54.1.i.i333 = load i32, ptr %413, align 4
  %_18.i.i344 = load i32, ptr %404, align 4
  %_29.i.i4119560.not = icmp eq i32 %_18.i.i344, 0
  %_56.0.i.i360 = load ptr, ptr %414, align 8, !nonnull !10, !align !10173
  %_56.1.i.i361 = load i32, ptr %415, align 4
  %_58.1.i.i391 = load i32, ptr %419, align 4
  %_58.0.i.i390 = load ptr, ptr %420, align 16, !nonnull !10, !align !10173
  br label %bb32.i62, !dbg !15367

bb13.i56.loopexit.loopexit:                       ; preds = %bb55.i418
  store <4 x float> %.lcssa1127312259, ptr %405, align 16
  store <4 x float> %.lcssa1126512269, ptr %408, align 16
  store <4 x float> %.lcssa1128812279, ptr %416, align 16
  br label %bb13.i56.loopexit, !dbg !15367

bb13.i56.loopexit:                                ; preds = %bb13.i56.loopexit.loopexit, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i268
  %storemerge.i.i353.lcssa96089634.lcssa9660 = phi i32 [ %storemerge.i.i353.lcssa96089634.lcssa9661, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i268 ], [ %storemerge.i.i353.lcssa96089634, %bb13.i56.loopexit.loopexit ]
  %ring_cursor.sroa.0.1.i270.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i579654, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i268 ], [ %ring_cursor.sroa.0.2.i421, %bb13.i56.loopexit.loopexit ], !dbg !15387
  %main_cursor.sroa.0.1.i271.lcssa = phi i32 [ %main_cursor.sroa.0.0.i589655, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i268 ], [ %main_cursor.sroa.0.2.i424, %bb13.i56.loopexit.loopexit ], !dbg !15388
  %_110.not.i61 = icmp eq i32 %423, 0, !dbg !15367
  %indvars.iv.next11038 = add i32 %indvars.iv11037, -32, !dbg !15367
  br i1 %_110.not.i61, label %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb32.i62, !dbg !15367

bb32.i62:                                         ; preds = %bb32.i62.lr.ph, %bb13.i56.loopexit
  %indvars.iv11037 = phi i32 [ %frames, %bb32.i62.lr.ph ], [ %indvars.iv.next11038, %bb13.i56.loopexit ]
  %storemerge.i.i353.lcssa96089634.lcssa9661 = phi i32 [ %_22.i.i347.promoted, %bb32.i62.lr.ph ], [ %storemerge.i.i353.lcssa96089634.lcssa9660, %bb13.i56.loopexit ]
  %iter2.sroa.0.0.i609657 = phi i32 [ %yield_count.sroa.0.0.i3728, %bb32.i62.lr.ph ], [ %423, %bb13.i56.loopexit ]
  %iter1.sroa.0.0.i599656 = phi i32 [ 0, %bb32.i62.lr.ph ], [ %422, %bb13.i56.loopexit ]
  %main_cursor.sroa.0.0.i589655 = phi i32 [ %_26.i51, %bb32.i62.lr.ph ], [ %main_cursor.sroa.0.1.i271.lcssa, %bb13.i56.loopexit ]
  %ring_cursor.sroa.0.0.i579654 = phi i32 [ %_27.i52, %bb32.i62.lr.ph ], [ %ring_cursor.sroa.0.1.i270.lcssa, %bb13.i56.loopexit ]
  %umin11053 = call i32 @llvm.umin.i32(i32 %indvars.iv11037, i32 32), !dbg !15389
  %umax11040 = call i32 @llvm.umax.i32(i32 %umin11053, i32 1), !dbg !15389
  %422 = add i32 %iter1.sroa.0.0.i599656, 32, !dbg !15389
  %423 = add nsw i32 %iter2.sroa.0.0.i609657, -1, !dbg !15393
  %424 = sub i32 %frames, %iter1.sroa.0.0.i599656, !dbg !15394
  %spec.store.select.i63 = tail call i32 @llvm.umin.i32(i32 %424, i32 32), !dbg !15396
  %active_base.i64 = shl i32 %iter1.sroa.0.0.i599656, 2, !dbg !15401
  %active_base.i648371 = add i32 %spec.store.select.i63, %iter1.sroa.0.0.i599656, !dbg !15403
  %_40.i66 = shl i32 %active_base.i648371, 2, !dbg !15403
  %_120.i67 = icmp ult i32 %_40.i66, %active_base.i64, !dbg !15406
  %_114.not.i68 = icmp ugt i32 %_40.i66, %left_io.1
  %or.cond.i69 = or i1 %_120.i67, %_114.not.i68, !dbg !15406
  br i1 %or.cond.i69, label %bb38.i426, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747, !dbg !15406, !prof !4596

bb38.i426:                                        ; preds = %bb32.i62
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i64, i32 noundef %_40.i66, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_bac0677bfb785f04669fb3d80741c988) #32, !dbg !15413, !noalias !15414
  unreachable, !dbg !15413

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747: ; preds = %bb32.i62
  %_123.i71 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %active_base.i64, !dbg !15415
  %history.i.i25.sroa.0.0.copyload = load <4 x i32>, ptr %hot_left.i41, align 16, !dbg !15419
  %history.i.i25.sroa.7.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.7.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.10.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.10.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.13.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.13.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.16.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.16.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.19.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.19.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.22.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.22.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.26.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.26.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.29.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.29.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.32.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.32.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.35.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.35.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %history.i.i25.sroa.38.0.copyload = load <4 x i32>, ptr %history.i.i25.sroa.38.0.hot_left.i41.sroa_idx, align 16, !dbg !15419
  %_2.i37509535.not = icmp eq i32 %frames, %iter1.sroa.0.0.i599656, !dbg !15421
  br i1 %_2.i37509535.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i268, label %bb5.i.i74, !dbg !15421

bb5.i.i74:                                        ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747, %bb5.i.i74
  %iter.i.i21.sroa.16.09547 = phi i32 [ %546, %bb5.i.i74 ], [ 0, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.35.09546 = phi <4 x i32> [ %history.i.i25.sroa.32.09545, %bb5.i.i74 ], [ %history.i.i25.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.32.09545 = phi <4 x i32> [ %history.i.i25.sroa.29.09544, %bb5.i.i74 ], [ %history.i.i25.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.29.09544 = phi <4 x i32> [ %history.i.i25.sroa.26.09543, %bb5.i.i74 ], [ %history.i.i25.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.26.09543 = phi <4 x i32> [ %history.i.i25.sroa.22.09542, %bb5.i.i74 ], [ %history.i.i25.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.22.09542 = phi <4 x i32> [ %history.i.i25.sroa.19.09541, %bb5.i.i74 ], [ %history.i.i25.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.19.09541 = phi <4 x i32> [ %history.i.i25.sroa.16.09540, %bb5.i.i74 ], [ %history.i.i25.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.16.09540 = phi <4 x i32> [ %history.i.i25.sroa.13.09539, %bb5.i.i74 ], [ %history.i.i25.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.13.09539 = phi <4 x i32> [ %history.i.i25.sroa.10.09538, %bb5.i.i74 ], [ %history.i.i25.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.10.09538 = phi <4 x i32> [ %history.i.i25.sroa.7.09537, %bb5.i.i74 ], [ %history.i.i25.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.7.09537 = phi <4 x i32> [ %history.i.i25.sroa.0.09536, %bb5.i.i74 ], [ %history.i.i25.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %history.i.i25.sroa.0.09536 = phi <4 x i32> [ %lanes.i3106.sroa.0.0.copyload, %bb5.i.i74 ], [ %history.i.i25.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ]
  %start1.i.i3755 = shl i32 %iter.i.i21.sroa.16.09547, 2, !dbg !15424
  %data.i.i3756 = getelementptr inbounds nuw float, ptr %_123.i71, i32 %start1.i.i3755, !dbg !15426
  %lanes.i3106.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i3756, align 4, !dbg !15428, !alias.scope !15433, !noalias !15437
  %425 = bitcast <4 x i32> %history.i.i25.sroa.19.09541 to <4 x float>, !dbg !15444
  %426 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %425), !dbg !15449
  %427 = bitcast <4 x i32> %lanes.i3106.sroa.0.0.copyload to <4 x float>, !dbg !15450
  %428 = fmul <4 x float> %_11.i.i.i.i928398, %427, !dbg !15455
  %429 = fadd <4 x float> %428, zeroinitializer, !dbg !15456
  %430 = fmul <4 x float> %_14.i.i.i.i958399, %427, !dbg !15460
  %431 = fadd <4 x float> %430, zeroinitializer, !dbg !15464
  %432 = fmul <4 x float> %_17.i.i.i.i988400, %427, !dbg !15468
  %433 = fadd <4 x float> %432, zeroinitializer, !dbg !15472
  %434 = fmul <4 x float> %_20.i.i.i.i1018401, %427, !dbg !15476
  %435 = fadd <4 x float> %434, zeroinitializer, !dbg !15480
  %436 = bitcast <4 x i32> %history.i.i25.sroa.0.09536 to <4 x float>, !dbg !15484
  %437 = fmul <4 x float> %_25.i.i.i.i1068402, %436, !dbg !15488
  %438 = fadd <4 x float> %429, %437, !dbg !15489
  %439 = fmul <4 x float> %_28.i.i.i.i1098403, %436, !dbg !15493
  %440 = fadd <4 x float> %431, %439, !dbg !15497
  %441 = fmul <4 x float> %_31.i.i.i.i1128404, %436, !dbg !15501
  %442 = fadd <4 x float> %433, %441, !dbg !15505
  %443 = fmul <4 x float> %_34.i.i.i.i1158405, %436, !dbg !15509
  %444 = fadd <4 x float> %435, %443, !dbg !15513
  %445 = bitcast <4 x i32> %history.i.i25.sroa.7.09537 to <4 x float>, !dbg !15517
  %446 = fmul <4 x float> %_39.i.i.i.i1208406, %445, !dbg !15521
  %447 = fadd <4 x float> %438, %446, !dbg !15522
  %448 = fmul <4 x float> %_42.i.i.i.i1238407, %445, !dbg !15526
  %449 = fadd <4 x float> %440, %448, !dbg !15530
  %450 = fmul <4 x float> %_45.i.i.i.i1268408, %445, !dbg !15534
  %451 = fadd <4 x float> %442, %450, !dbg !15538
  %452 = fmul <4 x float> %_48.i.i.i.i1298409, %445, !dbg !15542
  %453 = fadd <4 x float> %444, %452, !dbg !15546
  %454 = bitcast <4 x i32> %history.i.i25.sroa.10.09538 to <4 x float>, !dbg !15550
  %455 = fmul <4 x float> %_53.i.i.i.i1348410, %454, !dbg !15554
  %456 = fadd <4 x float> %447, %455, !dbg !15555
  %457 = fmul <4 x float> %_56.i.i.i.i1378411, %454, !dbg !15559
  %458 = fadd <4 x float> %449, %457, !dbg !15563
  %459 = fmul <4 x float> %_59.i.i.i.i1408412, %454, !dbg !15567
  %460 = fadd <4 x float> %451, %459, !dbg !15571
  %461 = fmul <4 x float> %_62.i.i.i.i1438413, %454, !dbg !15575
  %462 = fadd <4 x float> %453, %461, !dbg !15579
  %463 = bitcast <4 x i32> %history.i.i25.sroa.13.09539 to <4 x float>, !dbg !15583
  %464 = fmul <4 x float> %_67.i.i.i.i1488414, %463, !dbg !15587
  %465 = fadd <4 x float> %456, %464, !dbg !15588
  %466 = fmul <4 x float> %_70.i.i.i.i1518415, %463, !dbg !15592
  %467 = fadd <4 x float> %458, %466, !dbg !15596
  %468 = fmul <4 x float> %_73.i.i.i.i1548416, %463, !dbg !15600
  %469 = fadd <4 x float> %460, %468, !dbg !15604
  %470 = fmul <4 x float> %_76.i.i.i.i1578417, %463, !dbg !15608
  %471 = fadd <4 x float> %462, %470, !dbg !15612
  %472 = bitcast <4 x i32> %history.i.i25.sroa.16.09540 to <4 x float>, !dbg !15616
  %473 = fmul <4 x float> %_81.i.i.i.i1628418, %472, !dbg !15620
  %474 = fadd <4 x float> %465, %473, !dbg !15621
  %475 = fmul <4 x float> %_84.i.i.i.i1658419, %472, !dbg !15625
  %476 = fadd <4 x float> %467, %475, !dbg !15629
  %477 = fmul <4 x float> %_87.i.i.i.i1688420, %472, !dbg !15633
  %478 = fadd <4 x float> %469, %477, !dbg !15637
  %479 = fmul <4 x float> %_90.i.i.i.i1718421, %472, !dbg !15641
  %480 = fadd <4 x float> %471, %479, !dbg !15645
  %481 = fmul <4 x float> %_95.i.i.i.i1768422, %425, !dbg !15649
  %482 = fadd <4 x float> %474, %481, !dbg !15653
  %483 = fmul <4 x float> %_98.i.i.i.i1798423, %425, !dbg !15657
  %484 = fadd <4 x float> %476, %483, !dbg !15661
  %485 = fmul <4 x float> %_101.i.i.i.i1828424, %425, !dbg !15665
  %486 = fadd <4 x float> %478, %485, !dbg !15669
  %487 = fmul <4 x float> %_104.i.i.i.i1858425, %425, !dbg !15673
  %488 = fadd <4 x float> %480, %487, !dbg !15677
  %489 = bitcast <4 x i32> %history.i.i25.sroa.22.09542 to <4 x float>, !dbg !15681
  %490 = fmul <4 x float> %_109.i.i.i.i1908426, %489, !dbg !15685
  %491 = fadd <4 x float> %482, %490, !dbg !15686
  %492 = fmul <4 x float> %_112.i.i.i.i1938427, %489, !dbg !15690
  %493 = fadd <4 x float> %484, %492, !dbg !15694
  %494 = fmul <4 x float> %_115.i.i.i.i1968428, %489, !dbg !15698
  %495 = fadd <4 x float> %486, %494, !dbg !15702
  %496 = fmul <4 x float> %_118.i.i.i.i1998429, %489, !dbg !15706
  %497 = fadd <4 x float> %488, %496, !dbg !15710
  %498 = bitcast <4 x i32> %history.i.i25.sroa.26.09543 to <4 x float>, !dbg !15714
  %499 = fmul <4 x float> %_123.i.i.i.i2048430, %498, !dbg !15718
  %500 = fadd <4 x float> %491, %499, !dbg !15719
  %501 = fmul <4 x float> %_126.i.i.i.i2078431, %498, !dbg !15723
  %502 = fadd <4 x float> %493, %501, !dbg !15727
  %503 = fmul <4 x float> %_129.i.i.i.i2108432, %498, !dbg !15731
  %504 = fadd <4 x float> %495, %503, !dbg !15735
  %505 = fmul <4 x float> %_132.i.i.i.i2138433, %498, !dbg !15739
  %506 = fadd <4 x float> %497, %505, !dbg !15743
  %507 = bitcast <4 x i32> %history.i.i25.sroa.29.09544 to <4 x float>, !dbg !15747
  %508 = fmul <4 x float> %_137.i.i.i.i2188434, %507, !dbg !15751
  %509 = fadd <4 x float> %500, %508, !dbg !15752
  %510 = fmul <4 x float> %_140.i.i.i.i2218435, %507, !dbg !15756
  %511 = fadd <4 x float> %502, %510, !dbg !15760
  %512 = fmul <4 x float> %_143.i.i.i.i2248436, %507, !dbg !15764
  %513 = fadd <4 x float> %504, %512, !dbg !15768
  %514 = fmul <4 x float> %_146.i.i.i.i2278437, %507, !dbg !15772
  %515 = fadd <4 x float> %506, %514, !dbg !15776
  %516 = bitcast <4 x i32> %history.i.i25.sroa.32.09545 to <4 x float>, !dbg !15780
  %517 = fmul <4 x float> %_151.i.i.i.i2328438, %516, !dbg !15784
  %518 = fadd <4 x float> %509, %517, !dbg !15785
  %519 = fmul <4 x float> %_154.i.i.i.i2358439, %516, !dbg !15789
  %520 = fadd <4 x float> %511, %519, !dbg !15793
  %521 = fmul <4 x float> %_157.i.i.i.i2388440, %516, !dbg !15797
  %522 = fadd <4 x float> %513, %521, !dbg !15801
  %523 = fmul <4 x float> %_160.i.i.i.i2418441, %516, !dbg !15805
  %524 = fadd <4 x float> %515, %523, !dbg !15809
  %525 = bitcast <4 x i32> %history.i.i25.sroa.35.09546 to <4 x float>, !dbg !15813
  %526 = fmul <4 x float> %_165.i.i.i.i2468442, %525, !dbg !15817
  %527 = fadd <4 x float> %518, %526, !dbg !15818
  %528 = fmul <4 x float> %_168.i.i.i.i2498443, %525, !dbg !15822
  %529 = fadd <4 x float> %520, %528, !dbg !15826
  %530 = fmul <4 x float> %_171.i.i.i.i2528444, %525, !dbg !15830
  %531 = fadd <4 x float> %522, %530, !dbg !15834
  %532 = fmul <4 x float> %_174.i.i.i.i2558445, %525, !dbg !15838
  %533 = fadd <4 x float> %524, %532, !dbg !15842
  %534 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %527), !dbg !15846
  %535 = fcmp olt <4 x float> %534, %426, !dbg !15850
  %536 = select <4 x i1> %535, <4 x float> %426, <4 x float> %534, !dbg !15854
  %537 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %529), !dbg !15846
  %538 = fcmp olt <4 x float> %537, %536, !dbg !15850
  %539 = select <4 x i1> %538, <4 x float> %536, <4 x float> %537, !dbg !15854
  %540 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %531), !dbg !15846
  %541 = fcmp olt <4 x float> %540, %539, !dbg !15850
  %542 = select <4 x i1> %541, <4 x float> %539, <4 x float> %540, !dbg !15854
  %543 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %533), !dbg !15846
  %544 = fcmp olt <4 x float> %543, %542, !dbg !15850
  %545 = select <4 x i1> %544, <4 x float> %542, <4 x float> %543, !dbg !15854
  %546 = add nuw nsw i32 %iter.i.i21.sroa.16.09547, 1, !dbg !15855
  %data.i4.i3760 = getelementptr inbounds nuw float, ptr %peaks_left.i39, i32 %start1.i.i3755, !dbg !15856
  store <4 x float> %545, ptr %data.i4.i3760, align 4, !dbg !15859, !alias.scope !15864, !noalias !15868
  %exitcond11041.not = icmp eq i32 %546, %umax11040, !dbg !15421
  br i1 %exitcond11041.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i268, label %bb5.i.i74, !dbg !15421

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i268: ; preds = %bb5.i.i74, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747
  %history.i.i25.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.0.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %lanes.i3106.sroa.0.0.copyload, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.7.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.0.09536, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.10.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.7.09537, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.13.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.10.09538, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.16.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.13.09539, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.19.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.16.09540, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.22.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.19.09541, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.26.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.22.09542, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.29.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.26.09543, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.32.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.29.09544, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.35.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.32.09545, %bb5.i.i74 ], !dbg !15872
  %history.i.i25.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i25.sroa.38.0.copyload, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3747 ], [ %history.i.i25.sroa.35.09546, %bb5.i.i74 ], !dbg !15872
  store <4 x i32> %history.i.i25.sroa.0.0.lcssa, ptr %hot_left.i41, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.7.0.lcssa, ptr %history.i.i25.sroa.7.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.10.0.lcssa, ptr %history.i.i25.sroa.10.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.13.0.lcssa, ptr %history.i.i25.sroa.13.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.16.0.lcssa, ptr %history.i.i25.sroa.16.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.19.0.lcssa, ptr %history.i.i25.sroa.19.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.22.0.lcssa, ptr %history.i.i25.sroa.22.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.26.0.lcssa, ptr %history.i.i25.sroa.26.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.29.0.lcssa, ptr %history.i.i25.sroa.29.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.32.0.lcssa, ptr %history.i.i25.sroa.32.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.35.0.lcssa, ptr %history.i.i25.sroa.35.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  store <4 x i32> %history.i.i25.sroa.38.0.lcssa, ptr %history.i.i25.sroa.38.0.hot_left.i41.sroa_idx, align 16, !dbg !15873
  br i1 %_2.i37509535.not, label %bb13.i56.loopexit, label %bb17.i274.lr.ph, !dbg !15874

bb17.i274.lr.ph:                                  ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i268
  %_13.i14318377 = load <16 x i8>, ptr %407, align 16
  %_13.i14188381 = load <16 x i8>, ptr %410, align 16
  %_37.i.i3778389 = load <4 x float>, ptr %417, align 16
  %.promoted12258 = load <4 x float>, ptr %405, align 16
  %.promoted12268 = load <4 x float>, ptr %408, align 16
  %.promoted12278 = load <4 x float>, ptr %416, align 16
  br label %bb17.i274, !dbg !15874

bb17.i274:                                        ; preds = %bb17.i274.lr.ph, %bb55.i418
  %.lcssa1128812280 = phi <4 x float> [ %.promoted12278, %bb17.i274.lr.ph ], [ %.lcssa1128812279, %bb55.i418 ]
  %.lcssa1126512270 = phi <4 x float> [ %.promoted12268, %bb17.i274.lr.ph ], [ %.lcssa1126512269, %bb55.i418 ]
  %.lcssa1127312260 = phi <4 x float> [ %.promoted12258, %bb17.i274.lr.ph ], [ %.lcssa1127312259, %bb55.i418 ]
  %storemerge.i.i353.lcssa96089635 = phi i32 [ %storemerge.i.i353.lcssa96089634.lcssa9661, %bb17.i274.lr.ph ], [ %storemerge.i.i353.lcssa96089634, %bb55.i418 ]
  %frame.sroa.0.0.i2729630 = phi i32 [ 0, %bb17.i274.lr.ph ], [ %_68.i287, %bb55.i418 ]
  %main_cursor.sroa.0.1.i2719629 = phi i32 [ %main_cursor.sroa.0.0.i589655, %bb17.i274.lr.ph ], [ %main_cursor.sroa.0.2.i424, %bb55.i418 ]
  %ring_cursor.sroa.0.1.i2709628 = phi i32 [ %ring_cursor.sroa.0.0.i579654, %bb17.i274.lr.ph ], [ %ring_cursor.sroa.0.2.i421, %bb55.i418 ]
  %_51.i275 = sub nuw nsw i32 %spec.store.select.i63, %frame.sroa.0.0.i2729630, !dbg !15876
  %ring.i1334 = load i32, ptr %364, align 4, !dbg !15877, !alias.scope !15879, !noalias !15882, !noundef !10
  %main.i1335 = load i32, ptr %365, align 4, !dbg !15886, !alias.scope !15879, !noalias !15882, !noundef !10
  %_10.i = add i32 %ring_cursor.sroa.0.1.i2709628, 1, !dbg !15887
  %_38.not.i = icmp ult i32 %_10.i, %ring.i1334, !dbg !15888
  %547 = select i1 %_38.not.i, i32 0, i32 %ring.i1334, !dbg !15888
  %start1.sroa.0.0.i1336 = sub nuw i32 %_10.i, %547, !dbg !15888
  %_12.i1337 = add i32 %_54.i36.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i2709628, !dbg !15890
  %_39.not.i = icmp ult i32 %_12.i1337, %ring.i1334, !dbg !15891
  %548 = select i1 %_39.not.i, i32 0, i32 %ring.i1334, !dbg !15891
  %left_end.sroa.0.0.i = sub nuw i32 %_12.i1337, %548, !dbg !15891
  %_18.i1341 = add i32 %_54.i36.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i2709628, !dbg !15893
  %_41.not.i = icmp ult i32 %_18.i1341, %ring.i1334, !dbg !15894
  %549 = select i1 %_41.not.i, i32 0, i32 %ring.i1334, !dbg !15894
  %left_expiring.sroa.0.0.i = sub nuw i32 %_18.i1341, %549, !dbg !15894
  %550 = sub i32 %ring.i1334, %ring_cursor.sroa.0.1.i2709628, !dbg !15896
  %spec.store.select.i1343 = tail call i32 @llvm.umin.i32(i32 %550, i32 %_51.i275), !dbg !15897
  %551 = sub i32 %main.i1335, %main_cursor.sroa.0.1.i2719629, !dbg !15899
  %_24.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %551, i32 %spec.store.select.i1343), !dbg !15900
  %552 = sub i32 %ring.i1334, %start1.sroa.0.0.i1336, !dbg !15902
  %_25.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %552, i32 %_24.sroa.0.0.i), !dbg !15903
  %553 = sub i32 %ring.i1334, %left_end.sroa.0.0.i, !dbg !15905
  %_27.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %553, i32 %_25.sroa.0.0.i), !dbg !15906
  %554 = sub i32 %ring.i1334, %left_expiring.sroa.0.0.i, !dbg !15908
  %_31.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %554, i32 %_27.sroa.0.0.i), !dbg !15909
  %_58.i277 = add i32 %frame.sroa.0.0.i2729630, %iter1.sroa.0.0.i599656, !dbg !15911
  %base.i278 = shl i32 %_58.i277, 2, !dbg !15911
  %base.i2788373 = add i32 %_31.sroa.0.0.i, %_58.i277, !dbg !15914
  %_62.i280 = shl i32 %base.i2788373, 2, !dbg !15914
  %_135.i281 = icmp ult i32 %_62.i280, %base.i278, !dbg !15917
  %_131.not.i282 = icmp ugt i32 %_62.i280, %left_io.1
  %or.cond16.i283 = or i1 %_135.i281, %_131.not.i282, !dbg !15917
  br i1 %or.cond16.i283, label %bb46.i425, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !15917, !prof !4596

bb46.i425:                                        ; preds = %bb17.i274
  store <4 x float> %.lcssa1127312260, ptr %405, align 16
  store <4 x float> %.lcssa1126512270, ptr %408, align 16
  store <4 x float> %.lcssa1128812280, ptr %416, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i278, i32 noundef %_62.i280, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1eb3474cf2d0fe655b193b0c53e74d06) #32, !dbg !15925, !noalias !15414
  unreachable, !dbg !15925

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb17.i274
  %_138.i285 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i278, !dbg !15926
  %_68.i287 = add nuw nsw i32 %_31.sroa.0.0.i, %frame.sroa.0.0.i2729630, !dbg !15930
  %_147.i295.idx = shl nuw nsw i32 %frame.sroa.0.0.i2729630, 4, !dbg !15932
  %_147.i295 = getelementptr inbounds nuw i8, ptr %peaks_left.i39, i32 %_147.i295.idx, !dbg !15932
  %_2.i37939563.not = icmp eq i32 %_31.sroa.0.0.i, 0, !dbg !15941
  br i1 %_2.i37939563.not, label %bb55.i418, label %bb56.i299.lr.ph, !dbg !15941

bb56.i299.lr.ph:                                  ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %umin11049 = call i32 @llvm.umin.i32(i32 %553, i32 %554), !dbg !15941
  %umin11050 = call i32 @llvm.umin.i32(i32 %umin11049, i32 %552), !dbg !15941
  %umin11051 = call i32 @llvm.umin.i32(i32 %umin11050, i32 %550), !dbg !15941
  %umin11052 = call i32 @llvm.umin.i32(i32 %umin11051, i32 %551), !dbg !15941
  %555 = sub nsw i32 %umin11053, %frame.sroa.0.0.i2729630, !dbg !15941
  %umin11054 = call i32 @llvm.umin.i32(i32 %umin11052, i32 %555), !dbg !15941
  %556 = and i32 %umin11054, 1073741823, !dbg !15941
  %_11.i14288376.pre = load <4 x float>, ptr %_82.i308, align 16, !dbg !15951
  %_12.i1429.pre = load <4 x i32>, ptr %406, align 16, !dbg !15955
  %_11.i14158380.pre = load <4 x float>, ptr %_83.i309, align 16, !dbg !15956
  %_12.i1416.pre = load <4 x i32>, ptr %409, align 16, !dbg !15958
  br label %bb56.i299, !dbg !15941

bb56.i299:                                        ; preds = %bb56.i299.lr.ph, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115
  %_12.i1416 = phi <4 x i32> [ %_12.i1416.pre, %bb56.i299.lr.ph ], [ %582, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ], !dbg !15958
  %_11.i14158380 = phi <4 x float> [ %_11.i14158380.pre, %bb56.i299.lr.ph ], [ %581, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ], !dbg !15956
  %_12.i1429 = phi <4 x i32> [ %_12.i1429.pre, %bb56.i299.lr.ph ], [ %580, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ], !dbg !15955
  %_11.i14288376 = phi <4 x float> [ %_11.i14288376.pre, %bb56.i299.lr.ph ], [ %579, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ], !dbg !15951
  %557 = phi <4 x float> [ %.lcssa1128812280, %bb56.i299.lr.ph ], [ %614, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ]
  %storemerge.i.i3539598 = phi i32 [ %storemerge.i.i353.lcssa96089635, %bb56.i299.lr.ph ], [ %storemerge.i.i353, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ]
  %558 = phi <4 x float> [ %.lcssa1126512270, %bb56.i299.lr.ph ], [ %577, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ]
  %559 = phi <4 x float> [ %.lcssa1127312260, %bb56.i299.lr.ph ], [ %576, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ]
  %iter.i30.sroa.21.09565 = phi i32 [ 0, %bb56.i299.lr.ph ], [ %_152.0.i307, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ]
  %560 = fadd <4 x float> %559, splat (float -1.000000e+00), !dbg !15959
  %561 = fcmp ogt <4 x float> %560, zeroinitializer, !dbg !15963
  %562 = sext <4 x i1> %561 to <4 x i32>, !dbg !15967
  %563 = bitcast <4 x i32> %_12.i1429 to <4 x float>, !dbg !15972
  %564 = fadd <4 x float> %_11.i14288376, %563, !dbg !15976
  %565 = bitcast <4 x float> %564 to <16 x i8>, !dbg !15977
  %566 = bitcast <4 x i32> %562 to <16 x i8>, !dbg !15981
  %_4.i3806 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %565, <16 x i8> %_13.i14318377, <16 x i8> %566), !dbg !15982
  %567 = bitcast <4 x i32> %_12.i1429 to <16 x i8>, !dbg !15983
  %_4.i3807 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %567, <16 x i8> zeroinitializer, <16 x i8> %566), !dbg !15987
  %568 = fadd <4 x float> %558, splat (float -1.000000e+00), !dbg !15988
  %569 = fcmp ogt <4 x float> %568, zeroinitializer, !dbg !15992
  %570 = sext <4 x i1> %569 to <4 x i32>, !dbg !15996
  %571 = bitcast <4 x i32> %_12.i1416 to <4 x float>, !dbg !16001
  %572 = fadd <4 x float> %_11.i14158380, %571, !dbg !16005
  %573 = bitcast <4 x float> %572 to <16 x i8>, !dbg !16006
  %574 = bitcast <4 x i32> %570 to <16 x i8>, !dbg !16010
  %_4.i3808 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %573, <16 x i8> %_13.i14188381, <16 x i8> %574), !dbg !16011
  %575 = bitcast <4 x i32> %_12.i1416 to <16 x i8>, !dbg !16012
  %_4.i3809 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %575, <16 x i8> zeroinitializer, <16 x i8> %574), !dbg !16016
  %start1.i.i3797 = shl i32 %iter.i30.sroa.21.09565, 2, !dbg !16017
  %data.i.i3798 = getelementptr inbounds nuw float, ptr %_138.i285, i32 %start1.i.i3797, !dbg !16020
  %lanes.i3088.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i3798, align 4, !dbg !16022, !alias.scope !16031, !noalias !16035
  %_152.0.i307 = add nuw nsw i32 %iter.i30.sroa.21.09565, 1, !dbg !16039
  %576 = select <4 x i1> %561, <4 x float> %560, <4 x float> zeroinitializer, !dbg !16042
  %577 = select <4 x i1> %569, <4 x float> %568, <4 x float> zeroinitializer, !dbg !16043
  %_159.i322 = add i32 %iter.i30.sroa.21.09565, %ring_cursor.sroa.0.1.i2709628, !dbg !16044
  %_160.i323 = add i32 %iter.i30.sroa.21.09565, %main_cursor.sroa.0.1.i2719629, !dbg !16048
  %_161.i324 = add i32 %iter.i30.sroa.21.09565, %left_end.sroa.0.0.i, !dbg !16049
  %_162.i325 = add i32 %iter.i30.sroa.21.09565, %start1.sroa.0.0.i1336, !dbg !16050
  %_163.i326 = add i32 %iter.i30.sroa.21.09565, %left_expiring.sroa.0.0.i, !dbg !16051
  %base.i9.i.i335 = shl i32 %_159.i322, 2, !dbg !16052
  %_7.i10.i.i336 = add i32 %base.i9.i.i335, 4, !dbg !16055
  %578 = or disjoint i32 %base.i9.i.i335, 3, !dbg !16056
  %or.cond.i13.i.i339.not = icmp ult i32 %578, %_54.1.i.i333, !dbg !16056
  %579 = bitcast <16 x i8> %_4.i3806 to <4 x float>, !dbg !16056
  %580 = bitcast <16 x i8> %_4.i3807 to <4 x i32>, !dbg !16056
  %581 = bitcast <16 x i8> %_4.i3808 to <4 x float>, !dbg !16056
  %582 = bitcast <16 x i8> %_4.i3809 to <4 x i32>, !dbg !16056
  br i1 %or.cond.i13.i.i339.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i340, label %bb4.i15.i.i417, !dbg !16056, !prof !10564

bb4.i15.i.i417:                                   ; preds = %bb56.i299
  store <4 x float> %.lcssa1127312260, ptr %405, align 16
  store <4 x float> %.lcssa1126512270, ptr %408, align 16
  store <4 x float> %.lcssa1128812280, ptr %416, align 16
  store <16 x i8> %_4.i3806, ptr %_82.i308, align 16, !dbg !16060
  store <16 x i8> %_4.i3807, ptr %406, align 16, !dbg !16061
  store <16 x i8> %_4.i3808, ptr %_83.i309, align 16, !dbg !16062
  store <16 x i8> %_4.i3809, ptr %409, align 16, !dbg !16063
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i.i335, i32 noundef %_7.i10.i.i336, i32 noundef range(i32 0, 536870912) %_54.1.i.i333, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !16064, !noalias !16065
  unreachable, !dbg !16064

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i340: ; preds = %bb56.i299
  %data.i4.i3802 = getelementptr inbounds nuw float, ptr %_147.i295, i32 %start1.i.i3797, !dbg !16079
  %lanes.i3097.sroa.0.0.copyload = load <16 x i8>, ptr %data.i4.i3802, align 4, !dbg !16082, !alias.scope !16087, !noalias !16091
  %_4.i3810 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %lanes.i3097.sroa.0.0.copyload, <16 x i8> %lanes.i3097.sroa.0.0.copyload, <16 x i8> %411), !dbg !16095
  %583 = bitcast <16 x i8> %_4.i3806 to <4 x float>, !dbg !16099
  %584 = bitcast <16 x i8> %_4.i3810 to <4 x float>, !dbg !16104
  %585 = fdiv <4 x float> %583, %584, !dbg !16105
  %586 = bitcast <4 x float> %585 to <16 x i8>, !dbg !16109
  %587 = fcmp ogt <4 x float> %584, %583, !dbg !16113
  %588 = sext <4 x i1> %587 to <4 x i32>, !dbg !16113
  %589 = bitcast <4 x i32> %588 to <16 x i8>, !dbg !16114
  %_4.i3811 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %586, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %589), !dbg !16115
  %_17.i14.i.i341 = getelementptr inbounds nuw float, ptr %_54.0.i.i332, i32 %base.i9.i.i335, !dbg !16116
  store <16 x i8> %_4.i3811, ptr %_17.i14.i.i341, align 4, !dbg !16118, !alias.scope !16123, !noalias !16127
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16131), !dbg !16134
  %base.i1134 = shl i32 %_161.i324, 2, !dbg !16135
  %590 = or disjoint i32 %base.i1134, 3, !dbg !16138
  %or.cond.i1138.not = icmp ult i32 %590, %_54.1.i.i333, !dbg !16138
  br i1 %or.cond.i1138.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142, label %bb4.i1141, !dbg !16138, !prof !10564

bb4.i1141:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i340
  store <4 x float> %.lcssa1127312260, ptr %405, align 16
  store <4 x float> %.lcssa1126512270, ptr %408, align 16
  store <4 x float> %.lcssa1128812280, ptr %416, align 16
  store <16 x i8> %_4.i3806, ptr %_82.i308, align 16, !dbg !16060
  store <16 x i8> %_4.i3807, ptr %406, align 16, !dbg !16061
  store <16 x i8> %_4.i3808, ptr %_83.i309, align 16, !dbg !16062
  store <16 x i8> %_4.i3809, ptr %409, align 16, !dbg !16063
  %_5.i1135 = add i32 %base.i1134, 4, !dbg !16142
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1134, i32 noundef %_5.i1135, i32 noundef range(i32 0, 536870912) %_54.1.i.i333, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !16143, !noalias !16144
  unreachable, !dbg !16143

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i340
  %_15.i1140 = getelementptr inbounds nuw float, ptr %_54.0.i.i332, i32 %base.i1134, !dbg !16150
  %lanes.i2946.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1140, align 4, !dbg !16152
  %591 = icmp eq i32 %storemerge.i.i3539598, 0, !dbg !16157
  %_12.i27.i8383 = load <4 x float>, ptr %uniform_left.i38, align 16, !dbg !16157
  %592 = bitcast <4 x i32> %lanes.i2946.sroa.0.0.copyload to <4 x float>, !dbg !16157
  %593 = fcmp olt <4 x float> %_12.i27.i8383, %592, !dbg !16157
  %594 = select <4 x i1> %593, <4 x float> %_12.i27.i8383, <4 x float> %592, !dbg !16157
  %595 = bitcast <4 x float> %594 to <4 x i32>, !dbg !16157
  %.sroa.04928.0 = select i1 %591, <4 x i32> %lanes.i2946.sroa.0.0.copyload, <4 x i32> %595, !dbg !16157
  store <4 x i32> %.sroa.04928.0, ptr %uniform_left.i38, align 16, !dbg !16158, !alias.scope !16131, !noalias !16159
  %_15.i.i349 = add i32 %storemerge.i.i3539598, 1, !dbg !16161
  %complete.i.i350 = icmp eq i32 %_15.i.i349, %_18.i.i344, !dbg !16161
  br i1 %complete.i.i350, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106, label %bb7.i.i351, !dbg !16162

bb7.i.i351:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142
  %base.i1125 = shl i32 %_162.i325, 2, !dbg !16163
  %596 = or disjoint i32 %base.i1125, 3, !dbg !16165
  %or.cond.i1129.not = icmp ult i32 %596, %_54.1.i.i333, !dbg !16165
  br i1 %or.cond.i1129.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133, label %bb4.i1132, !dbg !16165, !prof !10564

bb4.i1132:                                        ; preds = %bb7.i.i351
  store <4 x float> %.lcssa1127312260, ptr %405, align 16
  store <4 x float> %.lcssa1126512270, ptr %408, align 16
  store <4 x float> %.lcssa1128812280, ptr %416, align 16
  store <16 x i8> %_4.i3806, ptr %_82.i308, align 16, !dbg !16060
  store <16 x i8> %_4.i3807, ptr %406, align 16, !dbg !16061
  store <16 x i8> %_4.i3808, ptr %_83.i309, align 16, !dbg !16062
  store <16 x i8> %_4.i3809, ptr %409, align 16, !dbg !16063
  %_5.i1126 = add i32 %base.i1125, 4, !dbg !16169
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1125, i32 noundef %_5.i1126, i32 noundef range(i32 0, 536870912) %_54.1.i.i333, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !16170, !noalias !16171
  unreachable, !dbg !16170

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133: ; preds = %bb7.i.i351
  %_15.i1131 = getelementptr inbounds nuw float, ptr %_54.0.i.i332, i32 %base.i1125, !dbg !16175
  %lanes.i2953.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1131, align 4, !dbg !16177, !alias.scope !16182, !noalias !16186
  %597 = bitcast <4 x i32> %.sroa.04928.0 to <4 x float>, !dbg !16190
  %598 = fcmp olt <4 x float> %lanes.i2953.sroa.0.0.copyload, %597, !dbg !16194
  %599 = select <4 x i1> %598, <4 x float> %lanes.i2953.sroa.0.0.copyload, <4 x float> %597, !dbg !16195
  %600 = bitcast <4 x float> %599 to <4 x i32>, !dbg !16196
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i352, !dbg !16198

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142
  %601 = bitcast <4 x i32> %lanes.i2946.sroa.0.0.copyload to <4 x float>, !dbg !16162
  br i1 %_29.i.i4119560.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i352, label %bb19.i.i412, !dbg !16199

bb19.i.i412:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %end.sroa.0.0.i.i4109562 = phi i32 [ %607, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ %_161.i324, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106 ]
  %iter.sroa.0.0.i.i4099561 = phi i32 [ %_30.i32.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106 ]
  %602 = phi <4 x float> [ %605, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ %601, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106 ]
  %base.i1094 = shl i32 %end.sroa.0.0.i.i4109562, 2, !dbg !16202
  %603 = or disjoint i32 %base.i1094, 3, !dbg !16204
  %or.cond.i1095.not = icmp ult i32 %603, %_54.1.i.i333, !dbg !16204
  br i1 %or.cond.i1095.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb4.i1097, !dbg !16204, !prof !10564

bb4.i1097:                                        ; preds = %bb19.i.i412
  store <4 x float> %.lcssa1127312260, ptr %405, align 16
  store <4 x float> %.lcssa1126512270, ptr %408, align 16
  store <4 x float> %.lcssa1128812280, ptr %416, align 16
  store <16 x i8> %_4.i3806, ptr %_82.i308, align 16, !dbg !16060
  store <16 x i8> %_4.i3807, ptr %406, align 16, !dbg !16061
  store <16 x i8> %_4.i3808, ptr %_83.i309, align 16, !dbg !16062
  store <16 x i8> %_4.i3809, ptr %409, align 16, !dbg !16063
  %_5.i = add i32 %base.i1094, 4, !dbg !16208
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1094, i32 noundef %_5.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i333, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !16209, !noalias !16210
  unreachable, !dbg !16209

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb19.i.i412
  %_30.i32.i = add nuw i32 %iter.sroa.0.0.i.i4099561, 1, !dbg !16214
  %_15.i = getelementptr inbounds nuw float, ptr %_54.0.i.i332, i32 %base.i1094, !dbg !16217
  %lanes.i2981.sroa.0.0.copyload = load <4 x float>, ptr %_15.i, align 4, !dbg !16219, !alias.scope !16224, !noalias !16228
  %604 = fcmp olt <4 x float> %602, %lanes.i2981.sroa.0.0.copyload, !dbg !16232
  %605 = select <4 x i1> %604, <4 x float> %602, <4 x float> %lanes.i2981.sroa.0.0.copyload, !dbg !16236
  store <4 x float> %605, ptr %_15.i, align 4, !dbg !16237, !alias.scope !16243, !noalias !16247
  %606 = icmp eq i32 %end.sroa.0.0.i.i4109562, 0, !dbg !16253
  %spec.store.select.i.i415 = select i1 %606, i32 %ring.i49, i32 %end.sroa.0.0.i.i4109562, !dbg !16253
  %607 = add i32 %spec.store.select.i.i415, -1, !dbg !16254
  %exitcond11045.not = icmp eq i32 %_30.i32.i, %_18.i.i344, !dbg !16255
  br i1 %exitcond11045.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i352, label %bb19.i.i412, !dbg !16199

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i352: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133
  %.sroa.04928.1 = phi <4 x i32> [ %600, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133 ], [ %.sroa.04928.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106 ], [ %.sroa.04928.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], !dbg !16257
  %storemerge.i.i353 = phi i32 [ %_15.i.i349, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], !dbg !16258
  %608 = bitcast <4 x i32> %.sroa.04928.1 to <4 x float>, !dbg !16259
  %609 = fmul <4 x float> %608, splat (float 1.638400e+04), !dbg !16263
  %610 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %609), !dbg !16264
  %611 = fmul <4 x float> %610, splat (float 0x3F10000000000000), !dbg !16268
  %base.i1116 = shl i32 %_163.i326, 2, !dbg !16272
  %612 = or disjoint i32 %base.i1116, 3, !dbg !16274
  %or.cond.i1120.not = icmp ult i32 %612, %_56.1.i.i361, !dbg !16274
  br i1 %or.cond.i1120.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1124, label %bb4.i1123, !dbg !16274, !prof !10564

bb4.i1123:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i352
  store <4 x float> %.lcssa1127312260, ptr %405, align 16
  store <4 x float> %.lcssa1126512270, ptr %408, align 16
  store <4 x float> %.lcssa1128812280, ptr %416, align 16
  store <16 x i8> %_4.i3806, ptr %_82.i308, align 16, !dbg !16060
  store <16 x i8> %_4.i3807, ptr %406, align 16, !dbg !16061
  store <16 x i8> %_4.i3808, ptr %_83.i309, align 16, !dbg !16062
  store <16 x i8> %_4.i3809, ptr %409, align 16, !dbg !16063
  %_5.i1117 = add i32 %base.i1116, 4, !dbg !16278
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1116, i32 noundef %_5.i1117, i32 noundef range(i32 0, 536870912) %_56.1.i.i361, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !16279, !noalias !16280
  unreachable, !dbg !16279

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1124: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i352
  %_15.i1122 = getelementptr inbounds nuw float, ptr %_56.0.i.i360, i32 %base.i1116, !dbg !16284
  %lanes.i2960.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1122, align 4, !dbg !16286, !alias.scope !16291, !noalias !16295
  %613 = fadd <4 x float> %611, %557, !dbg !16299
  %614 = fsub <4 x float> %613, %lanes.i2960.sroa.0.0.copyload, !dbg !16303
  %_8.not.i4.i.i372 = icmp ugt i32 %_7.i10.i.i336, %_56.1.i.i361
  br i1 %_8.not.i4.i.i372, label %bb4.i7.i.i406, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i374, !dbg !16307, !prof !4596

bb4.i7.i.i406:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1124
  store <4 x float> %.lcssa1127312260, ptr %405, align 16
  store <4 x float> %.lcssa1126512270, ptr %408, align 16
  store <4 x float> %.lcssa1128812280, ptr %416, align 16
  store <16 x i8> %_4.i3806, ptr %_82.i308, align 16, !dbg !16060
  store <16 x i8> %_4.i3807, ptr %406, align 16, !dbg !16061
  store <16 x i8> %_4.i3808, ptr %_83.i309, align 16, !dbg !16062
  store <16 x i8> %_4.i3809, ptr %409, align 16, !dbg !16063
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i.i335, i32 noundef %_7.i10.i.i336, i32 noundef range(i32 0, 536870912) %_56.1.i.i361, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !16312, !noalias !16313
  unreachable, !dbg !16312

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i374: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1124
  %_17.i6.i.i375 = getelementptr inbounds nuw float, ptr %_56.0.i.i360, i32 %base.i9.i.i335, !dbg !16317
  store <4 x float> %611, ptr %_17.i6.i.i375, align 4, !dbg !16319, !alias.scope !16324, !noalias !16328
  %_41.i.i3808390 = load <4 x float>, ptr %418, align 16, !dbg !16332
  %615 = fdiv <4 x float> %614, %_37.i.i3778389, !dbg !16333
  %616 = fsub <4 x float> splat (float 1.000000e+00), %615, !dbg !16337
  %617 = fsub <4 x float> %616, %_41.i.i3808390, !dbg !16341
  %618 = bitcast <16 x i8> %_4.i3808 to <4 x float>, !dbg !16345
  %619 = fmul <4 x float> %617, %618, !dbg !16349
  %620 = fadd <4 x float> %_41.i.i3808390, %619, !dbg !16350
  %621 = fcmp olt <4 x float> %620, %616, !dbg !16353
  %622 = select <4 x i1> %621, <4 x float> %616, <4 x float> %620, !dbg !16357
  %623 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %622), !dbg !16358
  %624 = fcmp uge <4 x float> %623, splat (float 0x3BC79CA100000000), !dbg !16363
  %625 = bitcast <4 x float> %622 to <4 x i32>, !dbg !16368
  %626 = select <4 x i1> %624, <4 x i32> %625, <4 x i32> zeroinitializer, !dbg !16368
  store <4 x i32> %626, ptr %418, align 16, !dbg !16371
  %base.i1107 = shl i32 %_160.i323, 2, !dbg !16372
  %627 = or disjoint i32 %base.i1107, 3, !dbg !16374
  %or.cond.i1111.not = icmp ult i32 %627, %_58.1.i.i391, !dbg !16374
  br i1 %or.cond.i1111.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115, label %bb4.i1114, !dbg !16374, !prof !10564

bb4.i1114:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i374
  store <4 x float> %.lcssa1127312260, ptr %405, align 16
  store <4 x float> %.lcssa1126512270, ptr %408, align 16
  store <4 x float> %.lcssa1128812280, ptr %416, align 16
  store <16 x i8> %_4.i3806, ptr %_82.i308, align 16, !dbg !16060
  store <16 x i8> %_4.i3807, ptr %406, align 16, !dbg !16061
  store <16 x i8> %_4.i3808, ptr %_83.i309, align 16, !dbg !16062
  store <16 x i8> %_4.i3809, ptr %409, align 16, !dbg !16063
  %_5.i1108 = add i32 %base.i1107, 4, !dbg !16378
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1107, i32 noundef %_5.i1108, i32 noundef range(i32 0, 536870912) %_58.1.i.i391, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !16379, !noalias !16380
  unreachable, !dbg !16379

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i374
  %628 = bitcast <4 x i32> %626 to <4 x float>, !dbg !16384
  %629 = fsub <4 x float> splat (float 1.000000e+00), %628, !dbg !16388
  %_15.i1113 = getelementptr inbounds nuw float, ptr %_58.0.i.i390, i32 %base.i1107, !dbg !16389
  %lanes.i2967.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1113, align 4, !dbg !16391, !alias.scope !16396, !noalias !16400
  store <4 x i32> %lanes.i3088.sroa.0.0.copyload, ptr %_15.i1113, align 4, !dbg !16404, !alias.scope !16410, !noalias !16414
  %630 = bitcast <4 x i32> %lanes.i2967.sroa.0.0.copyload to <4 x float>, !dbg !16420
  %631 = fmul <4 x float> %629, %630, !dbg !16424
  %632 = bitcast <4 x i32> %lanes.i2967.sroa.0.0.copyload to <16 x i8>, !dbg !16425
  %633 = bitcast <4 x float> %631 to <16 x i8>, !dbg !16429
  %_4.i3812 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %632, <16 x i8> %633, <16 x i8> %421), !dbg !16430
  store <16 x i8> %_4.i3812, ptr %data.i.i3798, align 4, !dbg !16431, !alias.scope !16436, !noalias !16440
  %exitcond11055.not = icmp eq i32 %_152.0.i307, %556, !dbg !15941
  br i1 %exitcond11055.not, label %bb21.i297.bb55.i418_crit_edge, label %bb56.i299, !dbg !15941

bb21.i297.bb55.i418_crit_edge:                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115
  store <16 x i8> %_4.i3806, ptr %_82.i308, align 16, !dbg !16060
  store <16 x i8> %_4.i3807, ptr %406, align 16, !dbg !16061
  store <16 x i8> %_4.i3808, ptr %_83.i309, align 16, !dbg !16062
  store <16 x i8> %_4.i3809, ptr %409, align 16, !dbg !16063
  br label %bb55.i418, !dbg !15941

bb55.i418:                                        ; preds = %bb21.i297.bb55.i418_crit_edge, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %.lcssa1128812279 = phi <4 x float> [ %614, %bb21.i297.bb55.i418_crit_edge ], [ %.lcssa1128812280, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %.lcssa1126512269 = phi <4 x float> [ %577, %bb21.i297.bb55.i418_crit_edge ], [ %.lcssa1126512270, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %.lcssa1127312259 = phi <4 x float> [ %576, %bb21.i297.bb55.i418_crit_edge ], [ %.lcssa1127312260, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %storemerge.i.i353.lcssa96089634 = phi i32 [ %storemerge.i.i353, %bb21.i297.bb55.i418_crit_edge ], [ %storemerge.i.i353.lcssa96089635, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %_96.i419 = add i32 %_31.sroa.0.0.i, %ring_cursor.sroa.0.1.i2709628, !dbg !16444
  %_158.not.i420 = icmp ult i32 %_96.i419, %ring.i49, !dbg !16445
  %634 = select i1 %_158.not.i420, i32 0, i32 %ring.i49, !dbg !16445
  %ring_cursor.sroa.0.2.i421 = sub nuw i32 %_96.i419, %634, !dbg !16445
  %_98.i422 = add i32 %_31.sroa.0.0.i, %main_cursor.sroa.0.1.i2719629, !dbg !16448
  %_164.not.i423 = icmp ult i32 %_98.i422, %main.i50, !dbg !16449
  %635 = select i1 %_164.not.i423, i32 0, i32 %main.i50, !dbg !16449
  %main_cursor.sroa.0.2.i424 = sub nuw i32 %_98.i422, %635, !dbg !16449
  %_45.i273 = icmp ult i32 %_68.i287, %spec.store.select.i63, !dbg !15874
  br i1 %_45.i273, label %bb17.i274, label %bb13.i56.loopexit.loopexit, !dbg !15874

_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb13.i56.loopexit, %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge
  %left_phase.i428 = phi i32 [ %left_phase.i428.pre, %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %storemerge.i.i353.lcssa96089634.lcssa9660, %bb13.i56.loopexit ], !dbg !15377
  %ring_cursor.sroa.0.0.i57.lcssa = phi i32 [ %_27.i52, %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %ring_cursor.sroa.0.1.i270.lcssa, %bb13.i56.loopexit ], !dbg !15359
  %main_cursor.sroa.0.0.i58.lcssa = phi i32 [ %_26.i51, %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %main_cursor.sroa.0.1.i271.lcssa, %bb13.i56.loopexit ], !dbg !15356
  %left_prefix.i427 = load <4 x i32>, ptr %uniform_left.i38, align 16, !dbg !16451, !noalias !15363
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i38), !dbg !16452, !noalias !15363
  %636 = getelementptr inbounds nuw i8, ptr %self, i32 968, !dbg !16453
  %_171.1.i430 = load i32, ptr %636, align 4, !dbg !16453, !alias.scope !15335, !noalias !16455, !noundef !10
  %_8.i3451 = icmp samesign ugt i32 %_171.1.i430, 3, !dbg !16456
  br i1 %_8.i3451, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3454, label %bb2.i3452, !dbg !16456, !prof !1039

bb2.i3452:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_171.1.i430, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !16461, !noalias !16462
  unreachable, !dbg !16461

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3454: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %637 = getelementptr inbounds nuw i8, ptr %self, i32 964, !dbg !16453
  %_171.0.i429 = load ptr, ptr %637, align 4, !dbg !16453, !alias.scope !15335, !noalias !16455, !nonnull !10, !noundef !10
  store <4 x i32> %left_prefix.i427, ptr %_171.0.i429, align 4, !dbg !16466, !alias.scope !16470, !noalias !16474
  %638 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !16476
  %_172.0.i431 = load ptr, ptr %638, align 4, !dbg !16476, !alias.scope !15335, !noalias !16455, !nonnull !10, !noundef !10
  %639 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !16476
  %_172.1.i432 = load i32, ptr %639, align 4, !dbg !16476, !alias.scope !15335, !noalias !16455, !noundef !10
  %640 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i428), !dbg !16477
  br i1 %640, label %bb2.i3818, label %bb6.i3813, !dbg !16477

bb6.i3813:                                        ; preds = %bb2.i3818, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3454
  %end_or_len.idx.i = shl nuw nsw i32 %_172.1.i432, 2, !dbg !16481
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_172.0.i431, i32 %end_or_len.idx.i, !dbg !16481
  %_293.i = icmp eq i32 %_172.1.i432, 0, !dbg !16485
  br i1 %_293.i, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i3814, !dbg !16488

bb2.i3818:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3454
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i428, 255, !dbg !16489
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !16489
  %_5.i3819 = icmp eq i32 %left_phase.i428, %bytes1.sroa.0.0.isplat.i, !dbg !16490
  br i1 %_5.i3819, label %bb3.i3820, label %bb6.i3813, !dbg !16490

bb3.i3820:                                        ; preds = %bb2.i3818
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i428 to i8, !dbg !16491
  %641 = shl nuw nsw i32 %_172.1.i432, 2, !dbg !16493
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_172.0.i431, i8 %bytes.sroa.0.0.extract.trunc.i, i32 %641, i1 false), !dbg !16493, !alias.scope !16494, !noalias !15414
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !16497

bb10.i3814:                                       ; preds = %bb6.i3813, %bb10.i3814
  %iter.sroa.0.04.i = phi ptr [ %_38.i3815, %bb10.i3814 ], [ %_172.0.i431, %bb6.i3813 ]
  %_38.i3815 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i32 4, !dbg !16498
  store i32 %left_phase.i428, ptr %iter.sroa.0.04.i, align 4, !dbg !16500, !alias.scope !16494, !noalias !15414
  %_29.i3816 = icmp eq ptr %_38.i3815, %end_or_len.i, !dbg !16485
  br i1 %_29.i3816, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i3814, !dbg !16488

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i3814, %bb6.i3813, %bb3.i3820
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i41, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24) #31, !dbg !16501
  store i32 %main_cursor.sroa.0.0.i58.lcssa, ptr %_25, align 4, !dbg !16502, !alias.scope !15337, !noalias !15358
  store i32 %ring_cursor.sroa.0.0.i57.lcssa, ptr %366, align 4, !dbg !16503, !alias.scope !15337, !noalias !15358
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i39), !dbg !16504, !noalias !15363
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !15332

bb6.i:                                            ; preds = %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3685, %bb2.i.i.i3681, %bb11.i.i3675, %bb11.i5.i3697
  br i1 %_12.i.i3659, label %bb7.i, label %bb8.i, !dbg !16505

bb2.i:                                            ; preds = %bb1.i3.i3694, %bb2.i3688
  br i1 %_12.i.i3659, label %bb3.i, label %bb4.i, !dbg !16506

bb7.i:                                            ; preds = %bb6.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16507), !dbg !16510
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16511), !dbg !16510
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16513), !dbg !16510
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i455, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #31, !dbg !16515
  %642 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !16519
  %643 = load i8, ptr %642, align 16, !dbg !16519, !range !4667, !alias.scope !16507, !noalias !16523, !noundef !10
  %644 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !16526
  %645 = load i8, ptr %644, align 1, !dbg !16526, !range !4667, !alias.scope !16507, !noalias !16523, !noundef !10
  %_25.i = load i32, ptr %_25, align 4, !dbg !16528, !alias.scope !16513, !noalias !16530, !noundef !10
  %646 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !16531
  %_27.i462 = load i32, ptr %646, align 4, !dbg !16531, !alias.scope !16513, !noalias !16530, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !16533, !noalias !16535
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i32 32, i1 false), !noalias !16535
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i453), !dbg !16536, !noalias !16535
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i453, i8 0, i32 1024, i1 false), !noalias !16535
  %hot_left.i455.promoted = load <4 x i32>, ptr %hot_left.i455, align 1
  %_78.not.i9524 = icmp eq i32 %frames, 0, !dbg !16538
  br i1 %_78.not.i9524, label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb29.i.lr.ph, !dbg !16538

bb29.i.lr.ph:                                     ; preds = %bb7.i
  %_23.i461 = trunc nuw i8 %645 to i1, !dbg !16526
  %spec.store.select19.i = select i1 %_23.i461, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !16526
  %_22.i459 = trunc nuw i8 %643 to i1, !dbg !16519
  %link.sroa.0.0.i460 = select i1 %_22.i459, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !16519
  %d9.i3821 = lshr i32 %frames, 5, !dbg !16548
  %r2.i3822 = and i32 %frames, 31, !dbg !16555
  %_19.not.i3823 = icmp ne i32 %r2.i3822, 0, !dbg !16556
  %647 = zext i1 %_19.not.i3823 to i32, !dbg !16556
  %yield_count.sroa.0.0.i3824 = add nuw nsw i32 %d9.i3821, %647, !dbg !16556
  %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 16
  %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 32
  %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 48
  %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 64
  %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 80
  %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 96
  %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 112
  %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 128
  %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 144
  %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 160
  %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 176
  %648 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %649 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %650 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i.i507 = getelementptr inbounds nuw i8, ptr %self, i32 80
  %651 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %652 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %653 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i.i521 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %654 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %655 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %656 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i.i535 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %657 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %658 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %659 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i.i549 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %660 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %661 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %662 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i.i563 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %663 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %664 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %665 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i.i577 = getelementptr inbounds nuw i8, ptr %self, i32 400
  %666 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %667 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %668 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i.i591 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %669 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %670 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %671 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i.i605 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %672 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %673 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %674 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i.i619 = getelementptr inbounds nuw i8, ptr %self, i32 592
  %675 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %676 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %677 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i.i633 = getelementptr inbounds nuw i8, ptr %self, i32 656
  %678 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %679 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %680 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i.i647 = getelementptr inbounds nuw i8, ptr %self, i32 720
  %681 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %682 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %683 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %_51.i676 = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 192
  %_52.i = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 256
  %684 = bitcast <4 x i32> %link.sroa.0.0.i460 to <16 x i8>
  %685 = getelementptr inbounds nuw i8, ptr %self, i32 916
  %686 = getelementptr inbounds nuw i8, ptr %self, i32 1020
  %687 = getelementptr inbounds nuw i8, ptr %self, i32 944
  %688 = getelementptr inbounds nuw i8, ptr %self, i32 940
  %689 = getelementptr inbounds nuw i8, ptr %self, i32 984
  %690 = getelementptr inbounds nuw i8, ptr %self, i32 980
  %691 = getelementptr inbounds nuw i8, ptr %self, i32 968
  %692 = getelementptr inbounds nuw i8, ptr %self, i32 964
  %693 = getelementptr inbounds nuw i8, ptr %self, i32 952
  %694 = getelementptr inbounds nuw i8, ptr %self, i32 948
  %695 = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 336
  %696 = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 352
  %697 = getelementptr inbounds nuw i8, ptr %hot_left.i455, i32 320
  %698 = getelementptr inbounds nuw i8, ptr %self, i32 936
  %699 = getelementptr inbounds nuw i8, ptr %self, i32 932
  %700 = bitcast <4 x i32> %spec.store.select19.i to <16 x i8>
  %701 = getelementptr inbounds nuw i8, ptr %self, i32 920
  %_11.i.i.i.i4958319 = load <4 x float>, ptr %_22, align 16
  %_14.i.i.i.i4988320 = load <4 x float>, ptr %648, align 16
  %_17.i.i.i.i5018321 = load <4 x float>, ptr %649, align 16
  %_20.i.i.i.i5048322 = load <4 x float>, ptr %650, align 16
  %_25.i.i.i.i5098323 = load <4 x float>, ptr %row1.i.i.i.i507, align 16
  %_28.i.i.i.i5128324 = load <4 x float>, ptr %651, align 16
  %_31.i.i.i.i5158325 = load <4 x float>, ptr %652, align 16
  %_34.i.i.i.i5188326 = load <4 x float>, ptr %653, align 16
  %_39.i.i.i.i5238327 = load <4 x float>, ptr %row3.i.i.i.i521, align 16
  %_42.i.i.i.i5268328 = load <4 x float>, ptr %654, align 16
  %_45.i.i.i.i5298329 = load <4 x float>, ptr %655, align 16
  %_48.i.i.i.i5328330 = load <4 x float>, ptr %656, align 16
  %_53.i.i.i.i5378331 = load <4 x float>, ptr %row5.i.i.i.i535, align 16
  %_56.i.i.i.i5408332 = load <4 x float>, ptr %657, align 16
  %_59.i.i.i.i5438333 = load <4 x float>, ptr %658, align 16
  %_62.i.i.i.i5468334 = load <4 x float>, ptr %659, align 16
  %_67.i.i.i.i5518335 = load <4 x float>, ptr %row7.i.i.i.i549, align 16
  %_70.i.i.i.i5548336 = load <4 x float>, ptr %660, align 16
  %_73.i.i.i.i5578337 = load <4 x float>, ptr %661, align 16
  %_76.i.i.i.i5608338 = load <4 x float>, ptr %662, align 16
  %_81.i.i.i.i5658339 = load <4 x float>, ptr %row9.i.i.i.i563, align 16
  %_84.i.i.i.i5688340 = load <4 x float>, ptr %663, align 16
  %_87.i.i.i.i5718341 = load <4 x float>, ptr %664, align 16
  %_90.i.i.i.i5748342 = load <4 x float>, ptr %665, align 16
  %_95.i.i.i.i5798343 = load <4 x float>, ptr %row11.i.i.i.i577, align 16
  %_98.i.i.i.i5828344 = load <4 x float>, ptr %666, align 16
  %_101.i.i.i.i5858345 = load <4 x float>, ptr %667, align 16
  %_104.i.i.i.i5888346 = load <4 x float>, ptr %668, align 16
  %_109.i.i.i.i5938347 = load <4 x float>, ptr %row13.i.i.i.i591, align 16
  %_112.i.i.i.i5968348 = load <4 x float>, ptr %669, align 16
  %_115.i.i.i.i5998349 = load <4 x float>, ptr %670, align 16
  %_118.i.i.i.i6028350 = load <4 x float>, ptr %671, align 16
  %_123.i.i.i.i6078351 = load <4 x float>, ptr %row15.i.i.i.i605, align 16
  %_126.i.i.i.i6108352 = load <4 x float>, ptr %672, align 16
  %_129.i.i.i.i6138353 = load <4 x float>, ptr %673, align 16
  %_132.i.i.i.i6168354 = load <4 x float>, ptr %674, align 16
  %_137.i.i.i.i6218355 = load <4 x float>, ptr %row17.i.i.i.i619, align 16
  %_140.i.i.i.i6248356 = load <4 x float>, ptr %675, align 16
  %_143.i.i.i.i6278357 = load <4 x float>, ptr %676, align 16
  %_146.i.i.i.i6308358 = load <4 x float>, ptr %677, align 16
  %_151.i.i.i.i6358359 = load <4 x float>, ptr %row19.i.i.i.i633, align 16
  %_154.i.i.i.i6388360 = load <4 x float>, ptr %678, align 16
  %_157.i.i.i.i6418361 = load <4 x float>, ptr %679, align 16
  %_160.i.i.i.i6448362 = load <4 x float>, ptr %680, align 16
  %_165.i.i.i.i6498363 = load <4 x float>, ptr %row21.i.i.i.i647, align 16
  %_168.i.i.i.i6528364 = load <4 x float>, ptr %681, align 16
  %_171.i.i.i.i6558365 = load <4 x float>, ptr %682, align 16
  %_174.i.i.i.i6588366 = load <4 x float>, ptr %683, align 16
  %_8.i.i6778303 = load <4 x float>, ptr %_51.i676, align 16
  %_9.i.i6788304 = load <4 x float>, ptr %_52.i, align 16
  %_62.i.i6998312 = load <4 x float>, ptr %696, align 16
  %iter.sroa.0.0.ptr.i.i9457.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i.i9457.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i.i9457.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i.i9457.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i.i9457.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i.i9457.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i.i9457.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16
  %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16
  %.promoted12210 = load <4 x float>, ptr %695, align 16
  br label %bb29.i, !dbg !16538

bb12.i465.loopexit:                               ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i671
  %.lcssa1146912211 = phi <4 x float> [ %.lcssa1146912212, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i671 ], [ %861, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429 ]
  %ring_cursor.sroa.0.1.i673.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i4679528, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i671 ], [ %spec.store.select8.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429 ], !dbg !16557
  %main_cursor.sroa.0.1.i674.lcssa = phi i32 [ %main_cursor.sroa.0.0.i4689529, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i671 ], [ %spec.store.select7.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429 ], !dbg !16558
  %_78.not.i = icmp eq i32 %704, 0, !dbg !16538
  %indvars.iv.next11021 = add i32 %indvars.iv11020, -32, !dbg !16538
  br i1 %_78.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, label %bb29.i, !dbg !16538

bb29.i:                                           ; preds = %bb29.i.lr.ph, %bb12.i465.loopexit
  %.lcssa1146912212 = phi <4 x float> [ %.promoted12210, %bb29.i.lr.ph ], [ %.lcssa1146912211, %bb12.i465.loopexit ]
  %history.i.i452.sroa.38.0.lcssa12192 = phi <4 x i32> [ %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.38.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.35.0.lcssa12174 = phi <4 x i32> [ %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.35.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.32.0.lcssa12156 = phi <4 x i32> [ %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.32.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.29.0.lcssa12138 = phi <4 x i32> [ %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.29.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.26.0.lcssa12120 = phi <4 x i32> [ %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.26.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.22.0.lcssa12102 = phi <4 x i32> [ %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.22.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.19.0.lcssa12084 = phi <4 x i32> [ %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.19.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.16.0.lcssa12066 = phi <4 x i32> [ %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.16.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.13.0.lcssa12048 = phi <4 x i32> [ %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.13.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.10.0.lcssa12030 = phi <4 x i32> [ %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.10.0.lcssa, %bb12.i465.loopexit ]
  %history.i.i452.sroa.7.0.lcssa12012 = phi <4 x i32> [ %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.7.0.lcssa, %bb12.i465.loopexit ]
  %indvars.iv11020 = phi i32 [ %frames, %bb29.i.lr.ph ], [ %indvars.iv.next11021, %bb12.i465.loopexit ]
  %main_cursor.sroa.0.0.i4689529 = phi i32 [ %_25.i, %bb29.i.lr.ph ], [ %main_cursor.sroa.0.1.i674.lcssa, %bb12.i465.loopexit ]
  %ring_cursor.sroa.0.0.i4679528 = phi i32 [ %_27.i462, %bb29.i.lr.ph ], [ %ring_cursor.sroa.0.1.i673.lcssa, %bb12.i465.loopexit ]
  %iter2.sroa.0.0.i4669527 = phi i32 [ %yield_count.sroa.0.0.i3824, %bb29.i.lr.ph ], [ %704, %bb12.i465.loopexit ]
  %iter.sroa.0.0.i9526 = phi i32 [ 0, %bb29.i.lr.ph ], [ %703, %bb12.i465.loopexit ]
  %history.i.i452.sroa.0.0.lcssa95049525 = phi <4 x i32> [ %hot_left.i455.promoted, %bb29.i.lr.ph ], [ %history.i.i452.sroa.0.0.lcssa, %bb12.i465.loopexit ]
  %702 = call i32 @llvm.umax.i32(i32 %indvars.iv11020, i32 1), !dbg !16559
  %umax11035 = call i32 @llvm.umin.i32(i32 %702, i32 32), !dbg !16559
  %703 = add i32 %iter.sroa.0.0.i9526, 32, !dbg !16559
  %704 = add nsw i32 %iter2.sroa.0.0.i4669527, -1, !dbg !16563
  %705 = sub i32 %frames, %iter.sroa.0.0.i9526, !dbg !16564
  %spec.store.select.i469 = tail call i32 @llvm.umin.i32(i32 %705, i32 32), !dbg !16566
  %active_base.i470 = shl i32 %iter.sroa.0.0.i9526, 2, !dbg !16571
  %active_base.i4708302 = add i32 %spec.store.select.i469, %iter.sroa.0.0.i9526, !dbg !16573
  %_40.i472 = shl i32 %active_base.i4708302, 2, !dbg !16573
  %_88.i = icmp ult i32 %_40.i472, %active_base.i470, !dbg !16576
  %_82.not.i = icmp ugt i32 %_40.i472, %left_io.1
  %or.cond.i473 = or i1 %_88.i, %_82.not.i, !dbg !16576
  br i1 %or.cond.i473, label %bb35.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844, !dbg !16576, !prof !4596

bb35.i:                                           ; preds = %bb29.i
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa12012, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa12030, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa12048, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa12066, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa12084, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa12102, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa12120, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa12138, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa12156, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa12174, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa12192, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i470, i32 noundef %_40.i472, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_cab916602395946b4285251e481c2df8) #32, !dbg !16585, !noalias !16586
  unreachable, !dbg !16585

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844: ; preds = %bb29.i
  %_91.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %active_base.i470, !dbg !16587
  %_2.i38479421.not = icmp eq i32 %frames, %iter.sroa.0.0.i9526, !dbg !16591
  br i1 %_2.i38479421.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i671, label %bb5.i.i477, !dbg !16591

bb5.i.i477:                                       ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844, %bb5.i.i477
  %iter.i.i448.sroa.16.09433 = phi i32 [ %827, %bb5.i.i477 ], [ 0, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.35.09432 = phi <4 x i32> [ %history.i.i452.sroa.32.09431, %bb5.i.i477 ], [ %history.i.i452.sroa.35.0.lcssa12174, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.32.09431 = phi <4 x i32> [ %history.i.i452.sroa.29.09430, %bb5.i.i477 ], [ %history.i.i452.sroa.32.0.lcssa12156, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.29.09430 = phi <4 x i32> [ %history.i.i452.sroa.26.09429, %bb5.i.i477 ], [ %history.i.i452.sroa.29.0.lcssa12138, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.26.09429 = phi <4 x i32> [ %history.i.i452.sroa.22.09428, %bb5.i.i477 ], [ %history.i.i452.sroa.26.0.lcssa12120, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.22.09428 = phi <4 x i32> [ %history.i.i452.sroa.19.09427, %bb5.i.i477 ], [ %history.i.i452.sroa.22.0.lcssa12102, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.19.09427 = phi <4 x i32> [ %history.i.i452.sroa.16.09426, %bb5.i.i477 ], [ %history.i.i452.sroa.19.0.lcssa12084, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.16.09426 = phi <4 x i32> [ %history.i.i452.sroa.13.09425, %bb5.i.i477 ], [ %history.i.i452.sroa.16.0.lcssa12066, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.13.09425 = phi <4 x i32> [ %history.i.i452.sroa.10.09424, %bb5.i.i477 ], [ %history.i.i452.sroa.13.0.lcssa12048, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.10.09424 = phi <4 x i32> [ %history.i.i452.sroa.7.09423, %bb5.i.i477 ], [ %history.i.i452.sroa.10.0.lcssa12030, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.7.09423 = phi <4 x i32> [ %history.i.i452.sroa.0.09422, %bb5.i.i477 ], [ %history.i.i452.sroa.7.0.lcssa12012, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %history.i.i452.sroa.0.09422 = phi <4 x i32> [ %lanes.i3079.sroa.0.0.copyload, %bb5.i.i477 ], [ %history.i.i452.sroa.0.0.lcssa95049525, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ]
  %start1.i.i3852 = shl i32 %iter.i.i448.sroa.16.09433, 2, !dbg !16594
  %data.i.i3853 = getelementptr inbounds nuw float, ptr %_91.i, i32 %start1.i.i3852, !dbg !16596
  %lanes.i3079.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i3853, align 4, !dbg !16598, !alias.scope !16603, !noalias !16607
  %706 = bitcast <4 x i32> %history.i.i452.sroa.19.09427 to <4 x float>, !dbg !16614
  %707 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %706), !dbg !16619
  %708 = bitcast <4 x i32> %lanes.i3079.sroa.0.0.copyload to <4 x float>, !dbg !16620
  %709 = fmul <4 x float> %_11.i.i.i.i4958319, %708, !dbg !16625
  %710 = fadd <4 x float> %709, zeroinitializer, !dbg !16626
  %711 = fmul <4 x float> %_14.i.i.i.i4988320, %708, !dbg !16630
  %712 = fadd <4 x float> %711, zeroinitializer, !dbg !16634
  %713 = fmul <4 x float> %_17.i.i.i.i5018321, %708, !dbg !16638
  %714 = fadd <4 x float> %713, zeroinitializer, !dbg !16642
  %715 = fmul <4 x float> %_20.i.i.i.i5048322, %708, !dbg !16646
  %716 = fadd <4 x float> %715, zeroinitializer, !dbg !16650
  %717 = bitcast <4 x i32> %history.i.i452.sroa.0.09422 to <4 x float>, !dbg !16654
  %718 = fmul <4 x float> %_25.i.i.i.i5098323, %717, !dbg !16658
  %719 = fadd <4 x float> %710, %718, !dbg !16659
  %720 = fmul <4 x float> %_28.i.i.i.i5128324, %717, !dbg !16663
  %721 = fadd <4 x float> %712, %720, !dbg !16667
  %722 = fmul <4 x float> %_31.i.i.i.i5158325, %717, !dbg !16671
  %723 = fadd <4 x float> %714, %722, !dbg !16675
  %724 = fmul <4 x float> %_34.i.i.i.i5188326, %717, !dbg !16679
  %725 = fadd <4 x float> %716, %724, !dbg !16683
  %726 = bitcast <4 x i32> %history.i.i452.sroa.7.09423 to <4 x float>, !dbg !16687
  %727 = fmul <4 x float> %_39.i.i.i.i5238327, %726, !dbg !16691
  %728 = fadd <4 x float> %719, %727, !dbg !16692
  %729 = fmul <4 x float> %_42.i.i.i.i5268328, %726, !dbg !16696
  %730 = fadd <4 x float> %721, %729, !dbg !16700
  %731 = fmul <4 x float> %_45.i.i.i.i5298329, %726, !dbg !16704
  %732 = fadd <4 x float> %723, %731, !dbg !16708
  %733 = fmul <4 x float> %_48.i.i.i.i5328330, %726, !dbg !16712
  %734 = fadd <4 x float> %725, %733, !dbg !16716
  %735 = bitcast <4 x i32> %history.i.i452.sroa.10.09424 to <4 x float>, !dbg !16720
  %736 = fmul <4 x float> %_53.i.i.i.i5378331, %735, !dbg !16724
  %737 = fadd <4 x float> %728, %736, !dbg !16725
  %738 = fmul <4 x float> %_56.i.i.i.i5408332, %735, !dbg !16729
  %739 = fadd <4 x float> %730, %738, !dbg !16733
  %740 = fmul <4 x float> %_59.i.i.i.i5438333, %735, !dbg !16737
  %741 = fadd <4 x float> %732, %740, !dbg !16741
  %742 = fmul <4 x float> %_62.i.i.i.i5468334, %735, !dbg !16745
  %743 = fadd <4 x float> %734, %742, !dbg !16749
  %744 = bitcast <4 x i32> %history.i.i452.sroa.13.09425 to <4 x float>, !dbg !16753
  %745 = fmul <4 x float> %_67.i.i.i.i5518335, %744, !dbg !16757
  %746 = fadd <4 x float> %737, %745, !dbg !16758
  %747 = fmul <4 x float> %_70.i.i.i.i5548336, %744, !dbg !16762
  %748 = fadd <4 x float> %739, %747, !dbg !16766
  %749 = fmul <4 x float> %_73.i.i.i.i5578337, %744, !dbg !16770
  %750 = fadd <4 x float> %741, %749, !dbg !16774
  %751 = fmul <4 x float> %_76.i.i.i.i5608338, %744, !dbg !16778
  %752 = fadd <4 x float> %743, %751, !dbg !16782
  %753 = bitcast <4 x i32> %history.i.i452.sroa.16.09426 to <4 x float>, !dbg !16786
  %754 = fmul <4 x float> %_81.i.i.i.i5658339, %753, !dbg !16790
  %755 = fadd <4 x float> %746, %754, !dbg !16791
  %756 = fmul <4 x float> %_84.i.i.i.i5688340, %753, !dbg !16795
  %757 = fadd <4 x float> %748, %756, !dbg !16799
  %758 = fmul <4 x float> %_87.i.i.i.i5718341, %753, !dbg !16803
  %759 = fadd <4 x float> %750, %758, !dbg !16807
  %760 = fmul <4 x float> %_90.i.i.i.i5748342, %753, !dbg !16811
  %761 = fadd <4 x float> %752, %760, !dbg !16815
  %762 = fmul <4 x float> %_95.i.i.i.i5798343, %706, !dbg !16819
  %763 = fadd <4 x float> %755, %762, !dbg !16823
  %764 = fmul <4 x float> %_98.i.i.i.i5828344, %706, !dbg !16827
  %765 = fadd <4 x float> %757, %764, !dbg !16831
  %766 = fmul <4 x float> %_101.i.i.i.i5858345, %706, !dbg !16835
  %767 = fadd <4 x float> %759, %766, !dbg !16839
  %768 = fmul <4 x float> %_104.i.i.i.i5888346, %706, !dbg !16843
  %769 = fadd <4 x float> %761, %768, !dbg !16847
  %770 = bitcast <4 x i32> %history.i.i452.sroa.22.09428 to <4 x float>, !dbg !16851
  %771 = fmul <4 x float> %_109.i.i.i.i5938347, %770, !dbg !16855
  %772 = fadd <4 x float> %763, %771, !dbg !16856
  %773 = fmul <4 x float> %_112.i.i.i.i5968348, %770, !dbg !16860
  %774 = fadd <4 x float> %765, %773, !dbg !16864
  %775 = fmul <4 x float> %_115.i.i.i.i5998349, %770, !dbg !16868
  %776 = fadd <4 x float> %767, %775, !dbg !16872
  %777 = fmul <4 x float> %_118.i.i.i.i6028350, %770, !dbg !16876
  %778 = fadd <4 x float> %769, %777, !dbg !16880
  %779 = bitcast <4 x i32> %history.i.i452.sroa.26.09429 to <4 x float>, !dbg !16884
  %780 = fmul <4 x float> %_123.i.i.i.i6078351, %779, !dbg !16888
  %781 = fadd <4 x float> %772, %780, !dbg !16889
  %782 = fmul <4 x float> %_126.i.i.i.i6108352, %779, !dbg !16893
  %783 = fadd <4 x float> %774, %782, !dbg !16897
  %784 = fmul <4 x float> %_129.i.i.i.i6138353, %779, !dbg !16901
  %785 = fadd <4 x float> %776, %784, !dbg !16905
  %786 = fmul <4 x float> %_132.i.i.i.i6168354, %779, !dbg !16909
  %787 = fadd <4 x float> %778, %786, !dbg !16913
  %788 = bitcast <4 x i32> %history.i.i452.sroa.29.09430 to <4 x float>, !dbg !16917
  %789 = fmul <4 x float> %_137.i.i.i.i6218355, %788, !dbg !16921
  %790 = fadd <4 x float> %781, %789, !dbg !16922
  %791 = fmul <4 x float> %_140.i.i.i.i6248356, %788, !dbg !16926
  %792 = fadd <4 x float> %783, %791, !dbg !16930
  %793 = fmul <4 x float> %_143.i.i.i.i6278357, %788, !dbg !16934
  %794 = fadd <4 x float> %785, %793, !dbg !16938
  %795 = fmul <4 x float> %_146.i.i.i.i6308358, %788, !dbg !16942
  %796 = fadd <4 x float> %787, %795, !dbg !16946
  %797 = bitcast <4 x i32> %history.i.i452.sroa.32.09431 to <4 x float>, !dbg !16950
  %798 = fmul <4 x float> %_151.i.i.i.i6358359, %797, !dbg !16954
  %799 = fadd <4 x float> %790, %798, !dbg !16955
  %800 = fmul <4 x float> %_154.i.i.i.i6388360, %797, !dbg !16959
  %801 = fadd <4 x float> %792, %800, !dbg !16963
  %802 = fmul <4 x float> %_157.i.i.i.i6418361, %797, !dbg !16967
  %803 = fadd <4 x float> %794, %802, !dbg !16971
  %804 = fmul <4 x float> %_160.i.i.i.i6448362, %797, !dbg !16975
  %805 = fadd <4 x float> %796, %804, !dbg !16979
  %806 = bitcast <4 x i32> %history.i.i452.sroa.35.09432 to <4 x float>, !dbg !16983
  %807 = fmul <4 x float> %_165.i.i.i.i6498363, %806, !dbg !16987
  %808 = fadd <4 x float> %799, %807, !dbg !16988
  %809 = fmul <4 x float> %_168.i.i.i.i6528364, %806, !dbg !16992
  %810 = fadd <4 x float> %801, %809, !dbg !16996
  %811 = fmul <4 x float> %_171.i.i.i.i6558365, %806, !dbg !17000
  %812 = fadd <4 x float> %803, %811, !dbg !17004
  %813 = fmul <4 x float> %_174.i.i.i.i6588366, %806, !dbg !17008
  %814 = fadd <4 x float> %805, %813, !dbg !17012
  %815 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %808), !dbg !17016
  %816 = fcmp olt <4 x float> %815, %707, !dbg !17020
  %817 = select <4 x i1> %816, <4 x float> %707, <4 x float> %815, !dbg !17024
  %818 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %810), !dbg !17016
  %819 = fcmp olt <4 x float> %818, %817, !dbg !17020
  %820 = select <4 x i1> %819, <4 x float> %817, <4 x float> %818, !dbg !17024
  %821 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %812), !dbg !17016
  %822 = fcmp olt <4 x float> %821, %820, !dbg !17020
  %823 = select <4 x i1> %822, <4 x float> %820, <4 x float> %821, !dbg !17024
  %824 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %814), !dbg !17016
  %825 = fcmp olt <4 x float> %824, %823, !dbg !17020
  %826 = select <4 x i1> %825, <4 x float> %823, <4 x float> %824, !dbg !17024
  %827 = add nuw nsw i32 %iter.i.i448.sroa.16.09433, 1, !dbg !17025
  %data.i4.i3857 = getelementptr inbounds nuw float, ptr %peaks_left.i453, i32 %start1.i.i3852, !dbg !17026
  store <4 x float> %826, ptr %data.i4.i3857, align 4, !dbg !17029, !alias.scope !17034, !noalias !17038
  %exitcond11024.not = icmp eq i32 %827, %umax11035, !dbg !16591
  br i1 %exitcond11024.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i671, label %bb5.i.i477, !dbg !16591

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i671: ; preds = %bb5.i.i477, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844
  %history.i.i452.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.0.0.lcssa95049525, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %lanes.i3079.sroa.0.0.copyload, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.7.0.lcssa12012, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.0.09422, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.10.0.lcssa12030, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.7.09423, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.13.0.lcssa12048, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.10.09424, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.16.0.lcssa12066, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.13.09425, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.19.0.lcssa12084, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.16.09426, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.22.0.lcssa12102, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.19.09427, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.26.0.lcssa12120, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.22.09428, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.29.0.lcssa12138, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.26.09429, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.32.0.lcssa12156, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.29.09430, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.35.0.lcssa12174, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.32.09431, %bb5.i.i477 ], !dbg !16583
  %history.i.i452.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i452.sroa.38.0.lcssa12192, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3844 ], [ %history.i.i452.sroa.35.09432, %bb5.i.i477 ], !dbg !16583
  br i1 %_2.i38479421.not, label %bb12.i465.loopexit, label %bb42.i.lr.ph, !dbg !17042

bb42.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i671
  %_64.i = load i32, ptr %685, align 4
  %_71.i705 = load i32, ptr %701, align 4
  br label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3077, !dbg !17042

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3077: ; preds = %bb42.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429
  %828 = phi <4 x float> [ %.lcssa1146912212, %bb42.i.lr.ph ], [ %861, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429 ]
  %main_cursor.sroa.0.1.i6749461 = phi i32 [ %main_cursor.sroa.0.0.i4689529, %bb42.i.lr.ph ], [ %spec.store.select7.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429 ]
  %ring_cursor.sroa.0.1.i6739460 = phi i32 [ %ring_cursor.sroa.0.0.i4679528, %bb42.i.lr.ph ], [ %spec.store.select8.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429 ]
  %iter1.sroa.0.0.i6729459 = phi i32 [ 0, %bb42.i.lr.ph ], [ %829, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429 ]
  %829 = add nuw nsw i32 %iter1.sroa.0.0.i6729459, 1, !dbg !17049
  %_47.i = add nuw nsw i32 %iter1.sroa.0.0.i6729459, %iter.sroa.0.0.i9526, !dbg !17055
  %base.i675 = shl i32 %_47.i, 2, !dbg !17055
  %_110.i.idx = shl i32 %iter1.sroa.0.0.i6729459, 4, !dbg !17057
  %_110.i = getelementptr inbounds nuw i8, ptr %peaks_left.i453, i32 %_110.i.idx, !dbg !17057
  %lanes.i3070.sroa.0.0.copyload = load <16 x i8>, ptr %_110.i, align 4, !dbg !17070, !alias.scope !17075, !noalias !17079
  %_4.i3872 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %lanes.i3070.sroa.0.0.copyload, <16 x i8> %lanes.i3070.sroa.0.0.copyload, <16 x i8> %684), !dbg !17083
  %_111.i = icmp ugt i32 %base.i675, %left_io.1, !dbg !17089
  br i1 %_111.i, label %bb46.i707, label %bb47.i, !dbg !17089, !prof !787

bb47.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3077
  %_113.i = sub nuw nsw i32 %left_io.1, %base.i675, !dbg !17094
  %_117.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i675, !dbg !17095
  %_8.i3064 = icmp samesign ugt i32 %_113.i, 3, !dbg !17100
  br i1 %_8.i3064, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3068, label %bb2.i3065, !dbg !17100, !prof !1039

bb2.i3065:                                        ; preds = %bb47.i
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_113.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !17105, !noalias !17106
  unreachable, !dbg !17105

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3068: ; preds = %bb47.i
  %lanes.i3061.sroa.0.0.copyload = load <4 x i32>, ptr %_117.i, align 4, !dbg !17110, !alias.scope !17114, !noalias !17118
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17120), !dbg !17123
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17124), !dbg !17123
  %width.i.i = load i32, ptr %686, align 4, !dbg !17126, !alias.scope !17128, !noalias !17129, !noundef !10
  %830 = bitcast <16 x i8> %_4.i3872 to <4 x float>, !dbg !17137
  %831 = fcmp olt <4 x float> %_8.i.i6778303, %830, !dbg !17142
  %832 = sext <4 x i1> %831 to <4 x i32>, !dbg !17142
  %833 = fdiv <4 x float> %_8.i.i6778303, %830, !dbg !17143
  %834 = bitcast <4 x float> %833 to <16 x i8>, !dbg !17147
  %835 = bitcast <4 x i32> %832 to <16 x i8>, !dbg !17151
  %_4.i3873 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %834, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %835), !dbg !17152
  %_158.1.i.i = load i32, ptr %687, align 4, !dbg !17153, !alias.scope !17128, !noalias !17129, !noundef !10
  %_22.i.i688 = mul i32 %width.i.i, %ring_cursor.sroa.0.1.i6739460, !dbg !17154
  %_90.i.i = icmp ugt i32 %_22.i.i688, %_158.1.i.i, !dbg !17155
  br i1 %_90.i.i, label %bb34.i.i, label %bb35.i.i, !dbg !17155, !prof !787

bb35.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3068
  %_93.i.i = sub nuw i32 %_158.1.i.i, %_22.i.i688, !dbg !17158
  %_8.i3441 = icmp samesign ugt i32 %_93.i.i, 3, !dbg !17159
  br i1 %_8.i3441, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3444, label %bb2.i3442, !dbg !17159, !prof !1039

bb2.i3442:                                        ; preds = %bb35.i.i
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !17164, !noalias !17165
  unreachable, !dbg !17164

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3444: ; preds = %bb35.i.i
  %_158.0.i.i = load ptr, ptr %688, align 4, !dbg !17153, !alias.scope !17128, !noalias !17129, !nonnull !10, !noundef !10
  %_97.i.i = getelementptr inbounds nuw float, ptr %_158.0.i.i, i32 %_22.i.i688, !dbg !17169
  store <16 x i8> %_4.i3873, ptr %_97.i.i, align 4, !dbg !17171, !alias.scope !17175, !noalias !17179
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17181), !dbg !17184
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17185), !dbg !17184
  %width.i1227 = load i32, ptr %686, align 4, !dbg !17187, !alias.scope !17181, !noalias !17189, !noundef !10
  %836 = icmp eq i32 %width.i1227, 0, !dbg !17190
  br i1 %836, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1333, label %bb29.i1233.lr.ph, !dbg !17190

bb29.i1233.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3444
  %_126.1.i1238 = load i32, ptr %44, align 4, !alias.scope !17181, !noalias !17189, !noundef !10
  %_126.0.i1242 = load ptr, ptr %43, align 4, !nonnull !10
  %837 = add i32 %ring_cursor.sroa.0.1.i6739460, 1
  %_21.not.i1247 = icmp ult i32 %837, %_64.i
  %838 = select i1 %_21.not.i1247, i32 0, i32 %_64.i
  %start1.sroa.0.0.i1248 = sub nuw i32 %837, %838
  %_128.1.i1251 = load i32, ptr %687, align 4
  %_128.0.i1255 = load ptr, ptr %688, align 4, !nonnull !10
  %_130.1.i1256 = load i32, ptr %689, align 4
  %_130.0.i1260 = load ptr, ptr %690, align 4, !nonnull !10
  %_132.1.i1263 = load i32, ptr %691, align 4
  %_132.0.i1267 = load ptr, ptr %692, align 4, !nonnull !10
  %_43.i1280 = mul i32 %width.i1227, %start1.sroa.0.0.i1248
  br label %bb29.i1233, !dbg !17190

bb29.i1233:                                       ; preds = %bb29.i1233.lr.ph, %bb28.i1295
  %iter.sroa.0.0.idx.i12319452 = phi i32 [ 0, %bb29.i1233.lr.ph ], [ %iter.sroa.0.0.add.i1236, %bb28.i1295 ]
  %iter.sroa.4.0.i12309451 = phi i32 [ 0, %bb29.i1233.lr.ph ], [ %_102.0.i1237, %bb28.i1295 ]
  %iter.sroa.7.0.i12299450 = phi i32 [ %width.i1227, %bb29.i1233.lr.ph ], [ %839, %bb28.i1295 ]
  %iter.sroa.0.0.ptr.i12329453 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i12319452, !dbg !17192
  %839 = add i32 %iter.sroa.7.0.i12299450, -1, !dbg !17192
  %_109.i1234 = icmp eq i32 %iter.sroa.0.0.idx.i12319452, 32, !dbg !17193
  br i1 %_109.i1234, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1333, label %bb33.i1235, !dbg !17197

bb33.i1235:                                       ; preds = %bb29.i1233
  %iter.sroa.0.0.add.i1236 = add nuw nsw i32 %iter.sroa.0.0.idx.i12319452, 4, !dbg !17198
  %_102.0.i1237 = add nuw nsw i32 %iter.sroa.4.0.i12309451, 1, !dbg !17200
  %exitcond11027.not = icmp eq i32 %iter.sroa.4.0.i12309451, %_126.1.i1238, !dbg !17201
  br i1 %exitcond11027.not, label %panic.i1240, label %bb2.i1241, !dbg !17201

bb2.i1241:                                        ; preds = %bb33.i1235
  %840 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1242, i32 %iter.sroa.4.0.i12309451, !dbg !17201
  %shape.i1243 = load i32, ptr %840, align 4, !dbg !17201, !noalias !17202, !noundef !10
  %841 = getelementptr inbounds nuw i8, ptr %840, i32 4, !dbg !17201
  %shape3.i1244 = load i32, ptr %841, align 4, !dbg !17201, !noalias !17202, !noundef !10
  %842 = add i32 %shape3.i1244, %ring_cursor.sroa.0.1.i6739460, !dbg !17203
  %_18.not.i1245 = icmp ult i32 %842, %_64.i, !dbg !17204
  %843 = select i1 %_18.not.i1245, i32 0, i32 %_64.i, !dbg !17204
  %spec.select.i1246 = sub nuw i32 %842, %843, !dbg !17204
  %_25.i1249 = mul i32 %spec.select.i1246, %width.i1227, !dbg !17205
  %_24.i1250 = add i32 %_25.i1249, %iter.sroa.4.0.i12309451, !dbg !17205
  %_28.i1252 = icmp ult i32 %_24.i1250, %_128.1.i1251, !dbg !17206
  br i1 %_28.i1252, label %bb9.i1254, label %panic5.i1253, !dbg !17206

panic.i1240:                                      ; preds = %bb33.i1235
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1238, i32 noundef %_126.1.i1238, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #32, !dbg !17201, !noalias !17202
  unreachable, !dbg !17201

bb9.i1254:                                        ; preds = %bb2.i1241
  %844 = getelementptr inbounds nuw float, ptr %_128.0.i1255, i32 %_24.i1250, !dbg !17206
  %845 = load float, ptr %844, align 4, !dbg !17206, !noalias !17202, !noundef !10
  %exitcond11028.not = icmp eq i32 %iter.sroa.4.0.i12309451, %_130.1.i1256, !dbg !17207
  br i1 %exitcond11028.not, label %panic6.i1258, label %bb10.i1259, !dbg !17207

panic5.i1253:                                     ; preds = %bb2.i1241
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1250, i32 noundef %_128.1.i1251, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #32, !dbg !17206, !noalias !17202
  unreachable, !dbg !17206

bb10.i1259:                                       ; preds = %bb9.i1254
  %846 = getelementptr inbounds nuw i32, ptr %_130.0.i1260, i32 %iter.sroa.4.0.i12309451, !dbg !17207
  %_30.i1261 = load i32, ptr %846, align 4, !dbg !17207, !noalias !17202, !noundef !10
  %847 = icmp eq i32 %_30.i1261, 0, !dbg !17208
  br i1 %847, label %bb14.i1270, label %bb12.i1262, !dbg !17208

panic6.i1258:                                     ; preds = %bb9.i1254
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1256, i32 noundef %_130.1.i1256, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #32, !dbg !17207, !noalias !17202
  unreachable, !dbg !17207

bb12.i1262:                                       ; preds = %bb10.i1259
  %_35.i1264 = icmp ult i32 %iter.sroa.4.0.i12309451, %_132.1.i1263, !dbg !17209
  br i1 %_35.i1264, label %bb13.i1266, label %panic7.i1265, !dbg !17209

bb14.i1270:                                       ; preds = %bb34.i1331, %bb13.i1266, %bb10.i1259
  %newest.sroa.0.0.i1271 = phi float [ %845, %bb10.i1259 ], [ %_33.i1268, %bb34.i1331 ], [ %845, %bb13.i1266 ], !dbg !17210
  %exitcond11029.not = icmp eq i32 %iter.sroa.4.0.i12309451, %_132.1.i1263, !dbg !17211
  br i1 %exitcond11029.not, label %panic8.i1274, label %bb15.i1275, !dbg !17211

bb13.i1266:                                       ; preds = %bb12.i1262
  %848 = getelementptr inbounds nuw float, ptr %_132.0.i1267, i32 %iter.sroa.4.0.i12309451, !dbg !17209
  %_33.i1268 = load float, ptr %848, align 4, !dbg !17209, !noalias !17202, !noundef !10
  %_116.i1269 = fcmp olt float %_33.i1268, %845, !dbg !17212
  br i1 %_116.i1269, label %bb34.i1331, label %bb14.i1270, !dbg !17212

panic7.i1265:                                     ; preds = %bb12.i1262
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i12309451, i32 noundef %_132.1.i1263, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #32, !dbg !17209, !noalias !17202
  unreachable, !dbg !17209

bb34.i1331:                                       ; preds = %bb13.i1266
  br label %bb14.i1270, !dbg !17214

bb15.i1275:                                       ; preds = %bb14.i1270
  %849 = getelementptr inbounds nuw float, ptr %_132.0.i1267, i32 %iter.sroa.4.0.i12309451, !dbg !17211
  store float %newest.sroa.0.0.i1271, ptr %849, align 4, !dbg !17211, !noalias !17202
  %_40.i1277 = add i32 %_30.i1261, 1, !dbg !17215
  %complete.i1278 = icmp eq i32 %_40.i1277, %shape.i1243, !dbg !17215
  br i1 %complete.i1278, label %bb19.i1300, label %bb17.i1279, !dbg !17216

panic8.i1274:                                     ; preds = %bb14.i1270
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1263, i32 noundef %_132.1.i1263, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #32, !dbg !17211, !noalias !17202
  unreachable, !dbg !17211

bb17.i1279:                                       ; preds = %bb15.i1275
  %_42.i1281 = add i32 %iter.sroa.4.0.i12309451, %_43.i1280, !dbg !17217
  %_45.i1283 = icmp ult i32 %_42.i1281, %_128.1.i1251, !dbg !17218
  br i1 %_45.i1283, label %bb27.i1293, label %panic9.i1284, !dbg !17218

panic9.i1284:                                     ; preds = %bb17.i1279
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1281, i32 noundef %_128.1.i1251, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #32, !dbg !17218, !noalias !17202
  unreachable, !dbg !17218

bb27.i1293:                                       ; preds = %bb17.i1279
  %850 = getelementptr inbounds nuw float, ptr %_128.0.i1255, i32 %_42.i1281, !dbg !17218
  %_41.i1287 = load float, ptr %850, align 4, !dbg !17218, !noalias !17202, !noundef !10
  %_117.i1288 = fcmp olt float %_41.i1287, %newest.sroa.0.0.i1271, !dbg !17219
  %newest.sroa.0.1.i1289 = select i1 %_117.i1288, float %_41.i1287, float %newest.sroa.0.0.i1271, !dbg !17219
  store float %newest.sroa.0.1.i1289, ptr %iter.sroa.0.0.ptr.i12329453, align 4, !dbg !17221, !alias.scope !17185, !noalias !17222
  br label %bb28.i1295, !dbg !17223

bb28.i1295:                                       ; preds = %bb22.i1328, %bb19.i1300, %bb27.i1293
  %storemerge8306 = phi i32 [ %_40.i1277, %bb27.i1293 ], [ 0, %bb19.i1300 ], [ 0, %bb22.i1328 ], !dbg !17224
  store i32 %storemerge8306, ptr %846, align 4, !dbg !17224, !noalias !17202
  %851 = icmp eq i32 %839, 0, !dbg !17190
  br i1 %851, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1333, label %bb29.i1233, !dbg !17190

bb19.i1300:                                       ; preds = %bb15.i1275
  store float %newest.sroa.0.0.i1271, ptr %iter.sroa.0.0.ptr.i12329453, align 4, !dbg !17221, !alias.scope !17185, !noalias !17222
  %_118.i13069446.not = icmp eq i32 %shape.i1243, 0, !dbg !17225
  br i1 %_118.i13069446.not, label %bb28.i1295, label %bb40.i1313.preheader, !dbg !17229

bb40.i1313.preheader:                             ; preds = %bb19.i1300
  %852 = load float, ptr %844, align 4, !dbg !17230, !noalias !17202, !noundef !10
  br label %bb40.i1313, !dbg !17231

bb40.i1313:                                       ; preds = %bb40.i1313.preheader, %bb22.i1328
  %iter2.sroa.0.0.i13059449 = phi i32 [ %_119.i1314, %bb22.i1328 ], [ 0, %bb40.i1313.preheader ]
  %suffix.sroa.0.0.i13049448 = phi float [ %suffix.sroa.0.1.i1324, %bb22.i1328 ], [ %852, %bb40.i1313.preheader ]
  %end.sroa.0.1.i13039447 = phi i32 [ %855, %bb22.i1328 ], [ %spec.select.i1246, %bb40.i1313.preheader ]
  %_54.i1315 = mul i32 %end.sroa.0.1.i13039447, %width.i1227, !dbg !17232
  %_53.i1316 = add i32 %_54.i1315, %iter.sroa.4.0.i12309451, !dbg !17232
  %_57.i1318 = icmp ult i32 %_53.i1316, %_128.1.i1251, !dbg !17231
  br i1 %_57.i1318, label %bb22.i1328, label %panic13.i1319, !dbg !17231

panic13.i1319:                                    ; preds = %bb40.i1313
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1316, i32 noundef %_128.1.i1251, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #32, !dbg !17231, !noalias !17202
  unreachable, !dbg !17231

bb22.i1328:                                       ; preds = %bb40.i1313
  %_119.i1314 = add nuw i32 %iter2.sroa.0.0.i13059449, 1, !dbg !17233
  %853 = getelementptr inbounds nuw float, ptr %_128.0.i1255, i32 %_53.i1316, !dbg !17231
  %_52.i1322 = load float, ptr %853, align 4, !dbg !17231, !noalias !17202, !noundef !10
  %_121.i1323 = fcmp olt float %suffix.sroa.0.0.i13049448, %_52.i1322, !dbg !17236
  %suffix.sroa.0.1.i1324 = select i1 %_121.i1323, float %suffix.sroa.0.0.i13049448, float %_52.i1322, !dbg !17236
  store float %suffix.sroa.0.1.i1324, ptr %853, align 4, !dbg !17238, !noalias !17202
  %854 = icmp eq i32 %end.sroa.0.1.i13039447, 0, !dbg !17239
  %spec.store.select.i1330 = select i1 %854, i32 %_64.i, i32 %end.sroa.0.1.i13039447, !dbg !17239
  %855 = add i32 %spec.store.select.i1330, -1, !dbg !17240
  %exitcond11026.not = icmp eq i32 %_119.i1314, %shape.i1243, !dbg !17225
  br i1 %exitcond11026.not, label %bb28.i1295, label %bb40.i1313, !dbg !17229

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1333: ; preds = %bb29.i1233, %bb28.i1295, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3444
  %lanes.i3054.sroa.0.0.copyload = load <4 x float>, ptr %scratch.i, align 4, !dbg !17241, !alias.scope !17246, !noalias !17250
  %856 = fmul <4 x float> %lanes.i3054.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !17254
  %857 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %856), !dbg !17258
  %858 = fmul <4 x float> %857, splat (float 0x3F10000000000000), !dbg !17262
  %859 = icmp eq i32 %width.i.i, 0, !dbg !17266
  %_163.1.i.i.pre = load i32, ptr %693, align 4, !dbg !17268, !alias.scope !17128, !noalias !17129
  br i1 %859, label %bb53.i.i, label %bb36.i.i.lr.ph, !dbg !17266

bb36.i.i.lr.ph:                                   ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1333
  %_159.1.i.i = load i32, ptr %44, align 4, !alias.scope !17128, !noalias !17129, !noundef !10
  %_159.0.i.i = load ptr, ptr %43, align 4, !nonnull !10
  %_161.0.i.i = load ptr, ptr %694, align 4, !nonnull !10
  %exitcond11032.not = icmp eq i32 %_159.1.i.i, 0, !dbg !17269
  br i1 %exitcond11032.not, label %panic.i.i, label %bb14.i.i, !dbg !17269

bb34.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3068
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i688, i32 noundef %_158.1.i.i, i32 noundef %_158.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #32, !dbg !17270, !noalias !17271
  unreachable, !dbg !17270

bb53.i.i.loopexit:                                ; preds = %bb18.i.i.7, %bb18.i.i.6, %bb18.i.i.5, %bb18.i.i.4, %bb18.i.i.3, %bb18.i.i.2, %bb18.i.i.1, %bb18.i.i
  %lanes.i3047.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i, align 4, !dbg !17272, !alias.scope !17277, !noalias !17281
  br label %bb53.i.i, !dbg !17285

bb53.i.i:                                         ; preds = %bb53.i.i.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1333
  %lanes.i3047.sroa.0.0.copyload = phi <4 x float> [ %lanes.i3047.sroa.0.0.copyload.pre, %bb53.i.i.loopexit ], [ %lanes.i3054.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1333 ], !dbg !17272
  %860 = fadd <4 x float> %858, %828, !dbg !17286
  %861 = fsub <4 x float> %860, %lanes.i3047.sroa.0.0.copyload, !dbg !17290
  %_123.i.i = icmp ugt i32 %_22.i.i688, %_163.1.i.i.pre, !dbg !17294
  br i1 %_123.i.i, label %bb41.i.i, label %bb42.i.i, !dbg !17294, !prof !787

bb14.i.i:                                         ; preds = %bb36.i.i.lr.ph
  %862 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 8, !dbg !17269
  %_42.i.i695 = load i32, ptr %862, align 4, !dbg !17269, !noalias !17297, !noundef !10
  %863 = add i32 %_42.i.i695, %ring_cursor.sroa.0.1.i6739460, !dbg !17298
  %_45.not.i.i = icmp ult i32 %863, %_64.i, !dbg !17299
  %864 = select i1 %_45.not.i.i, i32 0, i32 %_64.i, !dbg !17299
  %spec.select.i.i = sub nuw i32 %863, %864, !dbg !17299
  %_49.i.i696 = mul i32 %spec.select.i.i, %width.i.i, !dbg !17300
  %_51.i.i = icmp ult i32 %_49.i.i696, %_163.1.i.i.pre, !dbg !17301
  br i1 %_51.i.i, label %bb18.i.i, label %panic1.i.i, !dbg !17301

panic.i.i:                                        ; preds = %bb36.i.i.7, %bb36.i.i.6, %bb36.i.i.5, %bb36.i.i.4, %bb36.i.i.3, %bb36.i.i.2, %bb36.i.i.1, %bb36.i.i.lr.ph
  %_159.1.i.i.lcssa.ph = phi i32 [ 7, %bb36.i.i.7 ], [ 6, %bb36.i.i.6 ], [ 5, %bb36.i.i.5 ], [ 4, %bb36.i.i.4 ], [ 3, %bb36.i.i.3 ], [ 2, %bb36.i.i.2 ], [ 1, %bb36.i.i.1 ], [ 0, %bb36.i.i.lr.ph ]
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i.lcssa.ph, i32 noundef %_159.1.i.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #32, !dbg !17269, !noalias !17297
  unreachable, !dbg !17269

bb18.i.i:                                         ; preds = %bb14.i.i
  %865 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_49.i.i696, !dbg !17301
  %_47.i.i = load float, ptr %865, align 4, !dbg !17301, !noalias !17297, !noundef !10
  store float %_47.i.i, ptr %scratch.i, align 4, !dbg !17302, !alias.scope !17124, !noalias !17303
  %866 = icmp eq i32 %width.i.i, 1, !dbg !17266
  br i1 %866, label %bb53.i.i.loopexit, label %bb36.i.i.1, !dbg !17266

bb36.i.i.1:                                       ; preds = %bb18.i.i
  %exitcond11032.1.not = icmp eq i32 %_159.1.i.i, 1, !dbg !17269
  br i1 %exitcond11032.1.not, label %panic.i.i, label %bb14.i.i.1, !dbg !17269

bb14.i.i.1:                                       ; preds = %bb36.i.i.1
  %867 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 20, !dbg !17269
  %_42.i.i695.1 = load i32, ptr %867, align 4, !dbg !17269, !noalias !17297, !noundef !10
  %868 = add i32 %_42.i.i695.1, %ring_cursor.sroa.0.1.i6739460, !dbg !17298
  %_45.not.i.i.1 = icmp ult i32 %868, %_64.i, !dbg !17299
  %869 = select i1 %_45.not.i.i.1, i32 0, i32 %_64.i, !dbg !17299
  %spec.select.i.i.1 = sub nuw i32 %868, %869, !dbg !17299
  %_49.i.i696.1 = mul i32 %spec.select.i.i.1, %width.i.i, !dbg !17300
  %_48.i.i.1 = add i32 %_49.i.i696.1, 1, !dbg !17300
  %_51.i.i.1 = icmp ult i32 %_48.i.i.1, %_163.1.i.i.pre, !dbg !17301
  br i1 %_51.i.i.1, label %bb18.i.i.1, label %panic1.i.i, !dbg !17301

bb18.i.i.1:                                       ; preds = %bb14.i.i.1
  %870 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.1, !dbg !17301
  %_47.i.i.1 = load float, ptr %870, align 4, !dbg !17301, !noalias !17297, !noundef !10
  store float %_47.i.i.1, ptr %iter.sroa.0.0.ptr.i.i9457.1, align 4, !dbg !17302, !alias.scope !17124, !noalias !17303
  %871 = icmp eq i32 %width.i.i, 2, !dbg !17266
  br i1 %871, label %bb53.i.i.loopexit, label %bb36.i.i.2, !dbg !17266

bb36.i.i.2:                                       ; preds = %bb18.i.i.1
  %exitcond11032.2.not = icmp eq i32 %_159.1.i.i, 2, !dbg !17269
  br i1 %exitcond11032.2.not, label %panic.i.i, label %bb14.i.i.2, !dbg !17269

bb14.i.i.2:                                       ; preds = %bb36.i.i.2
  %872 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 32, !dbg !17269
  %_42.i.i695.2 = load i32, ptr %872, align 4, !dbg !17269, !noalias !17297, !noundef !10
  %873 = add i32 %_42.i.i695.2, %ring_cursor.sroa.0.1.i6739460, !dbg !17298
  %_45.not.i.i.2 = icmp ult i32 %873, %_64.i, !dbg !17299
  %874 = select i1 %_45.not.i.i.2, i32 0, i32 %_64.i, !dbg !17299
  %spec.select.i.i.2 = sub nuw i32 %873, %874, !dbg !17299
  %_49.i.i696.2 = mul i32 %spec.select.i.i.2, %width.i.i, !dbg !17300
  %_48.i.i.2 = add i32 %_49.i.i696.2, 2, !dbg !17300
  %_51.i.i.2 = icmp ult i32 %_48.i.i.2, %_163.1.i.i.pre, !dbg !17301
  br i1 %_51.i.i.2, label %bb18.i.i.2, label %panic1.i.i, !dbg !17301

bb18.i.i.2:                                       ; preds = %bb14.i.i.2
  %875 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.2, !dbg !17301
  %_47.i.i.2 = load float, ptr %875, align 4, !dbg !17301, !noalias !17297, !noundef !10
  store float %_47.i.i.2, ptr %iter.sroa.0.0.ptr.i.i9457.2, align 4, !dbg !17302, !alias.scope !17124, !noalias !17303
  %876 = icmp eq i32 %width.i.i, 3, !dbg !17266
  br i1 %876, label %bb53.i.i.loopexit, label %bb36.i.i.3, !dbg !17266

bb36.i.i.3:                                       ; preds = %bb18.i.i.2
  %exitcond11032.3.not = icmp eq i32 %_159.1.i.i, 3, !dbg !17269
  br i1 %exitcond11032.3.not, label %panic.i.i, label %bb14.i.i.3, !dbg !17269

bb14.i.i.3:                                       ; preds = %bb36.i.i.3
  %877 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 44, !dbg !17269
  %_42.i.i695.3 = load i32, ptr %877, align 4, !dbg !17269, !noalias !17297, !noundef !10
  %878 = add i32 %_42.i.i695.3, %ring_cursor.sroa.0.1.i6739460, !dbg !17298
  %_45.not.i.i.3 = icmp ult i32 %878, %_64.i, !dbg !17299
  %879 = select i1 %_45.not.i.i.3, i32 0, i32 %_64.i, !dbg !17299
  %spec.select.i.i.3 = sub nuw i32 %878, %879, !dbg !17299
  %_49.i.i696.3 = mul i32 %spec.select.i.i.3, %width.i.i, !dbg !17300
  %_48.i.i.3 = add i32 %_49.i.i696.3, 3, !dbg !17300
  %_51.i.i.3 = icmp ult i32 %_48.i.i.3, %_163.1.i.i.pre, !dbg !17301
  br i1 %_51.i.i.3, label %bb18.i.i.3, label %panic1.i.i, !dbg !17301

bb18.i.i.3:                                       ; preds = %bb14.i.i.3
  %880 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.3, !dbg !17301
  %_47.i.i.3 = load float, ptr %880, align 4, !dbg !17301, !noalias !17297, !noundef !10
  store float %_47.i.i.3, ptr %iter.sroa.0.0.ptr.i.i9457.3, align 4, !dbg !17302, !alias.scope !17124, !noalias !17303
  %881 = icmp eq i32 %width.i.i, 4, !dbg !17266
  br i1 %881, label %bb53.i.i.loopexit, label %bb36.i.i.4, !dbg !17266

bb36.i.i.4:                                       ; preds = %bb18.i.i.3
  %exitcond11032.4.not = icmp eq i32 %_159.1.i.i, 4, !dbg !17269
  br i1 %exitcond11032.4.not, label %panic.i.i, label %bb14.i.i.4, !dbg !17269

bb14.i.i.4:                                       ; preds = %bb36.i.i.4
  %882 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 56, !dbg !17269
  %_42.i.i695.4 = load i32, ptr %882, align 4, !dbg !17269, !noalias !17297, !noundef !10
  %883 = add i32 %_42.i.i695.4, %ring_cursor.sroa.0.1.i6739460, !dbg !17298
  %_45.not.i.i.4 = icmp ult i32 %883, %_64.i, !dbg !17299
  %884 = select i1 %_45.not.i.i.4, i32 0, i32 %_64.i, !dbg !17299
  %spec.select.i.i.4 = sub nuw i32 %883, %884, !dbg !17299
  %_49.i.i696.4 = mul i32 %spec.select.i.i.4, %width.i.i, !dbg !17300
  %_48.i.i.4 = add i32 %_49.i.i696.4, 4, !dbg !17300
  %_51.i.i.4 = icmp ult i32 %_48.i.i.4, %_163.1.i.i.pre, !dbg !17301
  br i1 %_51.i.i.4, label %bb18.i.i.4, label %panic1.i.i, !dbg !17301

bb18.i.i.4:                                       ; preds = %bb14.i.i.4
  %885 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.4, !dbg !17301
  %_47.i.i.4 = load float, ptr %885, align 4, !dbg !17301, !noalias !17297, !noundef !10
  store float %_47.i.i.4, ptr %iter.sroa.0.0.ptr.i.i9457.4, align 4, !dbg !17302, !alias.scope !17124, !noalias !17303
  %886 = icmp eq i32 %width.i.i, 5, !dbg !17266
  br i1 %886, label %bb53.i.i.loopexit, label %bb36.i.i.5, !dbg !17266

bb36.i.i.5:                                       ; preds = %bb18.i.i.4
  %exitcond11032.5.not = icmp eq i32 %_159.1.i.i, 5, !dbg !17269
  br i1 %exitcond11032.5.not, label %panic.i.i, label %bb14.i.i.5, !dbg !17269

bb14.i.i.5:                                       ; preds = %bb36.i.i.5
  %887 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 68, !dbg !17269
  %_42.i.i695.5 = load i32, ptr %887, align 4, !dbg !17269, !noalias !17297, !noundef !10
  %888 = add i32 %_42.i.i695.5, %ring_cursor.sroa.0.1.i6739460, !dbg !17298
  %_45.not.i.i.5 = icmp ult i32 %888, %_64.i, !dbg !17299
  %889 = select i1 %_45.not.i.i.5, i32 0, i32 %_64.i, !dbg !17299
  %spec.select.i.i.5 = sub nuw i32 %888, %889, !dbg !17299
  %_49.i.i696.5 = mul i32 %spec.select.i.i.5, %width.i.i, !dbg !17300
  %_48.i.i.5 = add i32 %_49.i.i696.5, 5, !dbg !17300
  %_51.i.i.5 = icmp ult i32 %_48.i.i.5, %_163.1.i.i.pre, !dbg !17301
  br i1 %_51.i.i.5, label %bb18.i.i.5, label %panic1.i.i, !dbg !17301

bb18.i.i.5:                                       ; preds = %bb14.i.i.5
  %890 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.5, !dbg !17301
  %_47.i.i.5 = load float, ptr %890, align 4, !dbg !17301, !noalias !17297, !noundef !10
  store float %_47.i.i.5, ptr %iter.sroa.0.0.ptr.i.i9457.5, align 4, !dbg !17302, !alias.scope !17124, !noalias !17303
  %891 = icmp eq i32 %width.i.i, 6, !dbg !17266
  br i1 %891, label %bb53.i.i.loopexit, label %bb36.i.i.6, !dbg !17266

bb36.i.i.6:                                       ; preds = %bb18.i.i.5
  %exitcond11032.6.not = icmp eq i32 %_159.1.i.i, 6, !dbg !17269
  br i1 %exitcond11032.6.not, label %panic.i.i, label %bb14.i.i.6, !dbg !17269

bb14.i.i.6:                                       ; preds = %bb36.i.i.6
  %892 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 80, !dbg !17269
  %_42.i.i695.6 = load i32, ptr %892, align 4, !dbg !17269, !noalias !17297, !noundef !10
  %893 = add i32 %_42.i.i695.6, %ring_cursor.sroa.0.1.i6739460, !dbg !17298
  %_45.not.i.i.6 = icmp ult i32 %893, %_64.i, !dbg !17299
  %894 = select i1 %_45.not.i.i.6, i32 0, i32 %_64.i, !dbg !17299
  %spec.select.i.i.6 = sub nuw i32 %893, %894, !dbg !17299
  %_49.i.i696.6 = mul i32 %spec.select.i.i.6, %width.i.i, !dbg !17300
  %_48.i.i.6 = add i32 %_49.i.i696.6, 6, !dbg !17300
  %_51.i.i.6 = icmp ult i32 %_48.i.i.6, %_163.1.i.i.pre, !dbg !17301
  br i1 %_51.i.i.6, label %bb18.i.i.6, label %panic1.i.i, !dbg !17301

bb18.i.i.6:                                       ; preds = %bb14.i.i.6
  %895 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.6, !dbg !17301
  %_47.i.i.6 = load float, ptr %895, align 4, !dbg !17301, !noalias !17297, !noundef !10
  store float %_47.i.i.6, ptr %iter.sroa.0.0.ptr.i.i9457.6, align 4, !dbg !17302, !alias.scope !17124, !noalias !17303
  %896 = icmp eq i32 %width.i.i, 7, !dbg !17266
  br i1 %896, label %bb53.i.i.loopexit, label %bb36.i.i.7, !dbg !17266

bb36.i.i.7:                                       ; preds = %bb18.i.i.6
  %exitcond11032.7.not = icmp eq i32 %_159.1.i.i, 7, !dbg !17269
  br i1 %exitcond11032.7.not, label %panic.i.i, label %bb14.i.i.7, !dbg !17269

bb14.i.i.7:                                       ; preds = %bb36.i.i.7
  %897 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 92, !dbg !17269
  %_42.i.i695.7 = load i32, ptr %897, align 4, !dbg !17269, !noalias !17297, !noundef !10
  %898 = add i32 %_42.i.i695.7, %ring_cursor.sroa.0.1.i6739460, !dbg !17298
  %_45.not.i.i.7 = icmp ult i32 %898, %_64.i, !dbg !17299
  %899 = select i1 %_45.not.i.i.7, i32 0, i32 %_64.i, !dbg !17299
  %spec.select.i.i.7 = sub nuw i32 %898, %899, !dbg !17299
  %_49.i.i696.7 = mul i32 %spec.select.i.i.7, %width.i.i, !dbg !17300
  %_48.i.i.7 = add i32 %_49.i.i696.7, 7, !dbg !17300
  %_51.i.i.7 = icmp ult i32 %_48.i.i.7, %_163.1.i.i.pre, !dbg !17301
  br i1 %_51.i.i.7, label %bb18.i.i.7, label %panic1.i.i, !dbg !17301

bb18.i.i.7:                                       ; preds = %bb14.i.i.7
  %900 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.7, !dbg !17301
  %_47.i.i.7 = load float, ptr %900, align 4, !dbg !17301, !noalias !17297, !noundef !10
  store float %_47.i.i.7, ptr %iter.sroa.0.0.ptr.i.i9457.7, align 4, !dbg !17302, !alias.scope !17124, !noalias !17303
  br label %bb53.i.i.loopexit, !dbg !17266

panic1.i.i:                                       ; preds = %bb14.i.i.7, %bb14.i.i.6, %bb14.i.i.5, %bb14.i.i.4, %bb14.i.i.3, %bb14.i.i.2, %bb14.i.i.1, %bb14.i.i
  %_48.i.i.lcssa.ph = phi i32 [ %_48.i.i.7, %bb14.i.i.7 ], [ %_48.i.i.6, %bb14.i.i.6 ], [ %_48.i.i.5, %bb14.i.i.5 ], [ %_48.i.i.4, %bb14.i.i.4 ], [ %_48.i.i.3, %bb14.i.i.3 ], [ %_48.i.i.2, %bb14.i.i.2 ], [ %_48.i.i.1, %bb14.i.i.1 ], [ %_49.i.i696, %bb14.i.i ]
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i.lcssa.ph, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #32, !dbg !17301, !noalias !17297
  unreachable, !dbg !17301

bb42.i.i:                                         ; preds = %bb53.i.i
  %_126.i.i = sub nuw i32 %_163.1.i.i.pre, %_22.i.i688, !dbg !17304
  %_8.i3436 = icmp samesign ugt i32 %_126.i.i, 3, !dbg !17305
  br i1 %_8.i3436, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3439, label %bb2.i3437, !dbg !17305, !prof !1039

bb2.i3437:                                        ; preds = %bb42.i.i
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !17310, !noalias !17311
  unreachable, !dbg !17310

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3439: ; preds = %bb42.i.i
  %_163.0.i.i = load ptr, ptr %694, align 4, !dbg !17268, !alias.scope !17128, !noalias !17129, !nonnull !10, !noundef !10
  %_130.i.i = getelementptr inbounds nuw float, ptr %_163.0.i.i, i32 %_22.i.i688, !dbg !17315
  store <4 x float> %858, ptr %_130.i.i, align 4, !dbg !17317, !alias.scope !17321, !noalias !17325
  %_66.i.i8313 = load <4 x float>, ptr %697, align 16, !dbg !17327
  %901 = fdiv <4 x float> %861, %_62.i.i6998312, !dbg !17328
  %902 = fsub <4 x float> splat (float 1.000000e+00), %901, !dbg !17332
  %903 = fsub <4 x float> %902, %_66.i.i8313, !dbg !17336
  %904 = fmul <4 x float> %_9.i.i6788304, %903, !dbg !17340
  %905 = fadd <4 x float> %_66.i.i8313, %904, !dbg !17344
  %906 = fcmp olt <4 x float> %905, %902, !dbg !17347
  %907 = select <4 x i1> %906, <4 x float> %902, <4 x float> %905, !dbg !17351
  %908 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %907), !dbg !17352
  %909 = fcmp uge <4 x float> %908, splat (float 0x3BC79CA100000000), !dbg !17357
  %910 = bitcast <4 x float> %907 to <4 x i32>, !dbg !17362
  %911 = select <4 x i1> %909, <4 x i32> %910, <4 x i32> zeroinitializer, !dbg !17362
  store <4 x i32> %911, ptr %697, align 16, !dbg !17365
  %912 = bitcast <4 x i32> %911 to <4 x float>, !dbg !17366
  %913 = fsub <4 x float> splat (float 1.000000e+00), %912, !dbg !17370
  %_164.1.i.i = load i32, ptr %698, align 4, !dbg !17371, !alias.scope !17128, !noalias !17129, !noundef !10
  %_74.i.i = mul i32 %width.i.i, %main_cursor.sroa.0.1.i6749461, !dbg !17372
  %_134.i.i = icmp ugt i32 %_74.i.i, %_164.1.i.i, !dbg !17373
  br i1 %_134.i.i, label %bb47.i.i, label %bb48.i.i, !dbg !17373, !prof !787

bb41.i.i:                                         ; preds = %bb53.i.i
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i688, i32 noundef %_163.1.i.i.pre, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #32, !dbg !17376, !noalias !17297
  unreachable, !dbg !17376

bb48.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3439
  %_137.i.i = sub nuw i32 %_164.1.i.i, %_74.i.i, !dbg !17377
  %_8.i3041 = icmp samesign ugt i32 %_137.i.i, 3, !dbg !17378
  br i1 %_8.i3041, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429, label %bb2.i3042, !dbg !17378, !prof !1039

bb2.i3042:                                        ; preds = %bb48.i.i
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !17383, !noalias !17384
  unreachable, !dbg !17383

bb47.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3439
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i, i32 noundef %_164.1.i.i, i32 noundef %_164.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #32, !dbg !17388, !noalias !17297
  unreachable, !dbg !17388

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3429: ; preds = %bb48.i.i
  %_164.0.i.i = load ptr, ptr %699, align 4, !dbg !17371, !alias.scope !17128, !noalias !17129, !nonnull !10, !noundef !10
  %_141.i.i = getelementptr inbounds nuw float, ptr %_164.0.i.i, i32 %_74.i.i, !dbg !17389
  %lanes.i3038.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i.i, align 4, !dbg !17391, !alias.scope !17395, !noalias !17399
  store <4 x i32> %lanes.i3061.sroa.0.0.copyload, ptr %_141.i.i, align 4, !dbg !17401, !alias.scope !17406, !noalias !17410
  %914 = bitcast <4 x i32> %lanes.i3038.sroa.0.0.copyload to <4 x float>, !dbg !17414
  %915 = fmul <4 x float> %913, %914, !dbg !17418
  %916 = bitcast <4 x i32> %lanes.i3038.sroa.0.0.copyload to <16 x i8>, !dbg !17419
  %917 = bitcast <4 x float> %915 to <16 x i8>, !dbg !17423
  %_4.i3874 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %916, <16 x i8> %917, <16 x i8> %700), !dbg !17424
  store <16 x i8> %_4.i3874, ptr %_117.i, align 4, !dbg !17425, !alias.scope !17430, !noalias !17434
  %918 = add i32 %main_cursor.sroa.0.1.i6749461, 1, !dbg !17438
  %_69.i = icmp eq i32 %918, %_71.i705, !dbg !17439
  %spec.store.select7.i = select i1 %_69.i, i32 0, i32 %918, !dbg !17439
  %919 = add i32 %ring_cursor.sroa.0.1.i6739460, 1, !dbg !17440
  %_72.i706 = icmp eq i32 %919, %_64.i, !dbg !17441
  %spec.store.select8.i = select i1 %_72.i706, i32 0, i32 %919, !dbg !17441
  %exitcond11036.not = icmp eq i32 %829, %umax11035, !dbg !17442
  br i1 %exitcond11036.not, label %bb12.i465.loopexit, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3077, !dbg !17042

bb46.i707:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3077
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912212, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i675, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_16fe79fe907693415948d189acefd4a9) #32, !dbg !17445, !noalias !16586
  unreachable, !dbg !17445

_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit: ; preds = %bb12.i465.loopexit
  store <4 x i32> %history.i.i452.sroa.7.0.lcssa, ptr %history.i.i452.sroa.7.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.10.0.lcssa, ptr %history.i.i452.sroa.10.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.13.0.lcssa, ptr %history.i.i452.sroa.13.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.16.0.lcssa, ptr %history.i.i452.sroa.16.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.19.0.lcssa, ptr %history.i.i452.sroa.19.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.22.0.lcssa, ptr %history.i.i452.sroa.22.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.26.0.lcssa, ptr %history.i.i452.sroa.26.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.29.0.lcssa, ptr %history.i.i452.sroa.29.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.32.0.lcssa, ptr %history.i.i452.sroa.32.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.35.0.lcssa, ptr %history.i.i452.sroa.35.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x i32> %history.i.i452.sroa.38.0.lcssa, ptr %history.i.i452.sroa.38.0.hot_left.i455.sroa_idx, align 16, !dbg !16583
  store <4 x float> %.lcssa1146912211, ptr %695, align 16
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !16583

_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, %bb7.i
  %history.i.i452.sroa.0.0.lcssa9504.lcssa = phi <4 x i32> [ %hot_left.i455.promoted, %bb7.i ], [ %history.i.i452.sroa.0.0.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ]
  %ring_cursor.sroa.0.0.i467.lcssa = phi i32 [ %_27.i462, %bb7.i ], [ %ring_cursor.sroa.0.1.i673.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !16531
  %main_cursor.sroa.0.0.i468.lcssa = phi i32 [ %_25.i, %bb7.i ], [ %main_cursor.sroa.0.1.i674.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !16528
  store <4 x i32> %history.i.i452.sroa.0.0.lcssa9504.lcssa, ptr %hot_left.i455, align 1, !dbg !16583
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i455, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24) #31, !dbg !17446
  store i32 %main_cursor.sroa.0.0.i468.lcssa, ptr %_25, align 4, !dbg !17447, !alias.scope !16513, !noalias !16530
  store i32 %ring_cursor.sroa.0.0.i467.lcssa, ptr %646, align 4, !dbg !17448, !alias.scope !16513, !noalias !16530
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i453), !dbg !17449, !noalias !16535
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !17450, !noalias !16535
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !16510

bb3.i:                                            ; preds = %bb2.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17451), !dbg !17454
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17455), !dbg !17454
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17457), !dbg !17454
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17459), !dbg !17454
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i), !dbg !17461, !noalias !17465
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #31, !dbg !17467, !noalias !17468
  %920 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !17469
  %921 = load i8, ptr %920, align 16, !dbg !17469, !range !4667, !alias.scope !17451, !noalias !17473, !noundef !10
  %922 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !17474
  %923 = load i8, ptr %922, align 1, !dbg !17474, !range !4667, !alias.scope !17451, !noalias !17473, !noundef !10
  %924 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !17476
  %ring.i = load i32, ptr %924, align 4, !dbg !17476, !alias.scope !17455, !noalias !17478, !noundef !10
  %925 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !17479
  %main.i = load i32, ptr %925, align 4, !dbg !17479, !alias.scope !17455, !noalias !17478, !noundef !10
  %_26.i = load i32, ptr %_25, align 4, !dbg !17481, !alias.scope !17459, !noalias !17483, !noundef !10
  %926 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !17484
  %_27.i = load i32, ptr %926, align 4, !dbg !17484, !alias.scope !17459, !noalias !17483, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !17486, !noalias !17465
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i32 1024, i1 false), !noalias !17465
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !17488, !noalias !17465
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24, i32 %ring.i, i32 %main.i) #31, !dbg !17490
  %hot_left.i.promoted = load <4 x i32>, ptr %hot_left.i, align 1
  %_110.not.i9789 = icmp eq i32 %frames, 0, !dbg !17491
  br i1 %_110.not.i9789, label %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge, label %bb32.i.lr.ph, !dbg !17491

bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge: ; preds = %bb3.i
  %.phi.trans.insert11110 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16
  %left_phase.i.pre = load i32, ptr %.phi.trans.insert11110, align 16, !dbg !17501, !noalias !17465
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !17491

bb32.i.lr.ph:                                     ; preds = %bb3.i
  %_23.i = trunc nuw i8 %923 to i1, !dbg !17474
  %spec.store.select18.i = select i1 %_23.i, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !17474
  %_22.i = trunc nuw i8 %921 to i1, !dbg !17469
  %link.sroa.0.0.i = select i1 %_22.i, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !17469
  %d9.i3875 = lshr i32 %frames, 5, !dbg !17502
  %r2.i3876 = and i32 %frames, 31, !dbg !17509
  %_19.not.i3877 = icmp ne i32 %r2.i3876, 0, !dbg !17510
  %927 = zext i1 %_19.not.i3877 to i32, !dbg !17510
  %yield_count.sroa.0.0.i3878 = add nuw nsw i32 %d9.i3875, %927, !dbg !17510
  %history.i.i.sroa.7.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 16
  %history.i.i.sroa.10.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 32
  %history.i.i.sroa.13.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 48
  %history.i.i.sroa.16.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 64
  %history.i.i.sroa.19.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 80
  %history.i.i.sroa.22.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 96
  %history.i.i.sroa.26.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 112
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 128
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 144
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 160
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 176
  %928 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %929 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %930 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 80
  %931 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %932 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %933 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %934 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %935 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %936 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %937 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %938 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %939 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %940 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %941 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %942 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 336
  %943 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %944 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %945 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 400
  %946 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %947 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %948 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 464
  %949 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %950 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %951 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 528
  %952 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %953 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %954 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 592
  %955 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %956 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %957 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 656
  %958 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %959 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %960 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 720
  %961 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %962 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %963 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %964 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 20
  %_54.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24
  %_54.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28
  %_82.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 192
  %_83.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 256
  %965 = bitcast <4 x i32> %link.sroa.0.0.i to <16 x i8>
  %966 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 32
  %967 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 36
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16
  %968 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 40
  %969 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 44
  %970 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 336
  %971 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 352
  %972 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 320
  %973 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 52
  %974 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 48
  %975 = bitcast <4 x i32> %spec.store.select18.i to <16 x i8>
  %history.i.i.sroa.7.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.7.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.26.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.26.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx, align 16
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx, align 16
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %.promoted9919 = load <4 x float>, ptr %970, align 16
  %_11.i.i.i.i8471 = load <4 x float>, ptr %_22, align 16
  %_14.i.i.i.i8472 = load <4 x float>, ptr %928, align 16
  %_17.i.i.i.i8473 = load <4 x float>, ptr %929, align 16
  %_20.i.i.i.i8474 = load <4 x float>, ptr %930, align 16
  %_25.i.i.i.i8475 = load <4 x float>, ptr %row1.i.i.i.i, align 16
  %_28.i.i.i.i8476 = load <4 x float>, ptr %931, align 16
  %_31.i.i.i.i8477 = load <4 x float>, ptr %932, align 16
  %_34.i.i.i.i8478 = load <4 x float>, ptr %933, align 16
  %_39.i.i.i.i8479 = load <4 x float>, ptr %row3.i.i.i.i, align 16
  %_42.i.i.i.i8480 = load <4 x float>, ptr %934, align 16
  %_45.i.i.i.i8481 = load <4 x float>, ptr %935, align 16
  %_48.i.i.i.i8482 = load <4 x float>, ptr %936, align 16
  %_53.i.i.i.i8483 = load <4 x float>, ptr %row5.i.i.i.i, align 16
  %_56.i.i.i.i8484 = load <4 x float>, ptr %937, align 16
  %_59.i.i.i.i8485 = load <4 x float>, ptr %938, align 16
  %_62.i.i.i.i8486 = load <4 x float>, ptr %939, align 16
  %_67.i.i.i.i8487 = load <4 x float>, ptr %row7.i.i.i.i, align 16
  %_70.i.i.i.i8488 = load <4 x float>, ptr %940, align 16
  %_73.i.i.i.i8489 = load <4 x float>, ptr %941, align 16
  %_76.i.i.i.i8490 = load <4 x float>, ptr %942, align 16
  %_81.i.i.i.i8491 = load <4 x float>, ptr %row9.i.i.i.i, align 16
  %_84.i.i.i.i8492 = load <4 x float>, ptr %943, align 16
  %_87.i.i.i.i8493 = load <4 x float>, ptr %944, align 16
  %_90.i.i.i.i8494 = load <4 x float>, ptr %945, align 16
  %_95.i.i.i.i8495 = load <4 x float>, ptr %row11.i.i.i.i, align 16
  %_98.i.i.i.i8496 = load <4 x float>, ptr %946, align 16
  %_101.i.i.i.i8497 = load <4 x float>, ptr %947, align 16
  %_104.i.i.i.i8498 = load <4 x float>, ptr %948, align 16
  %_109.i.i.i.i8499 = load <4 x float>, ptr %row13.i.i.i.i, align 16
  %_112.i.i.i.i8500 = load <4 x float>, ptr %949, align 16
  %_115.i.i.i.i8501 = load <4 x float>, ptr %950, align 16
  %_118.i.i.i.i8502 = load <4 x float>, ptr %951, align 16
  %_123.i.i.i.i8503 = load <4 x float>, ptr %row15.i.i.i.i, align 16
  %_126.i.i.i.i8504 = load <4 x float>, ptr %952, align 16
  %_129.i.i.i.i8505 = load <4 x float>, ptr %953, align 16
  %_132.i.i.i.i8506 = load <4 x float>, ptr %954, align 16
  %_137.i.i.i.i8507 = load <4 x float>, ptr %row17.i.i.i.i, align 16
  %_140.i.i.i.i8508 = load <4 x float>, ptr %955, align 16
  %_143.i.i.i.i8509 = load <4 x float>, ptr %956, align 16
  %_146.i.i.i.i8510 = load <4 x float>, ptr %957, align 16
  %_151.i.i.i.i8511 = load <4 x float>, ptr %row19.i.i.i.i, align 16
  %_154.i.i.i.i8512 = load <4 x float>, ptr %958, align 16
  %_157.i.i.i.i8513 = load <4 x float>, ptr %959, align 16
  %_160.i.i.i.i8514 = load <4 x float>, ptr %960, align 16
  %_165.i.i.i.i8515 = load <4 x float>, ptr %row21.i.i.i.i, align 16
  %_168.i.i.i.i8516 = load <4 x float>, ptr %961, align 16
  %_171.i.i.i.i8517 = load <4 x float>, ptr %962, align 16
  %_174.i.i.i.i8518 = load <4 x float>, ptr %963, align 16
  %_54.i.sroa.3.0.copyload = load i32, ptr %_54.i.sroa.3.0..sroa_idx, align 4
  %_54.i.sroa.4.0.copyload = load i32, ptr %_54.i.sroa.4.0..sroa_idx, align 4
  %_8.i.i8453 = load <4 x float>, ptr %_82.i, align 16
  %_9.i.i8454 = load <4 x float>, ptr %_83.i, align 16
  %_54.0.i.i = load ptr, ptr %966, align 16, !nonnull !10, !align !10173
  %_54.1.i.i = load i32, ptr %967, align 4
  %_18.i.i = load i32, ptr %964, align 4
  %_29.i.i9698.not = icmp eq i32 %_18.i.i, 0
  %_56.0.i.i = load ptr, ptr %968, align 8, !nonnull !10, !align !10173
  %_56.1.i.i = load i32, ptr %969, align 4
  %_37.i.i8462 = load <4 x float>, ptr %971, align 16
  %_58.1.i.i = load i32, ptr %973, align 4
  %_58.0.i.i = load ptr, ptr %974, align 16, !nonnull !10, !align !10173
  br label %bb32.i, !dbg !17491

bb13.i5.loopexit:                                 ; preds = %bb55.i, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898
  %history.i.i.sroa.0.0.lcssa11150 = phi <4 x i32> [ %history.i.i.sroa.0.0.lcssa97799790, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %lanes.i3133.sroa.0.0.copyload, %bb55.i ]
  %history.i.i.sroa.7.0.lcssa11149 = phi <4 x i32> [ %history.i.i.sroa.7.0.lcssa9798, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.0.09685, %bb55.i ]
  %history.i.i.sroa.10.0.lcssa11148 = phi <4 x i32> [ %history.i.i.sroa.10.0.lcssa9808, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.7.09684, %bb55.i ]
  %history.i.i.sroa.13.0.lcssa11147 = phi <4 x i32> [ %history.i.i.sroa.13.0.lcssa9818, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.10.09683, %bb55.i ]
  %history.i.i.sroa.16.0.lcssa11146 = phi <4 x i32> [ %history.i.i.sroa.16.0.lcssa9828, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.13.09682, %bb55.i ]
  %history.i.i.sroa.19.0.lcssa11145 = phi <4 x i32> [ %history.i.i.sroa.19.0.lcssa9838, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.16.09681, %bb55.i ]
  %history.i.i.sroa.22.0.lcssa11144 = phi <4 x i32> [ %history.i.i.sroa.22.0.lcssa9848, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.19.09680, %bb55.i ]
  %history.i.i.sroa.26.0.lcssa11143 = phi <4 x i32> [ %history.i.i.sroa.26.0.lcssa9858, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.22.09679, %bb55.i ]
  %history.i.i.sroa.29.0.lcssa11142 = phi <4 x i32> [ %history.i.i.sroa.29.0.lcssa9868, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.26.09678, %bb55.i ]
  %history.i.i.sroa.32.0.lcssa11141 = phi <4 x i32> [ %history.i.i.sroa.32.0.lcssa9878, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.29.09677, %bb55.i ]
  %history.i.i.sroa.35.0.lcssa11140 = phi <4 x i32> [ %history.i.i.sroa.35.0.lcssa9888, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.32.09676, %bb55.i ]
  %history.i.i.sroa.38.0.lcssa11139 = phi <4 x i32> [ %history.i.i.sroa.38.0.lcssa9898, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %history.i.i.sroa.35.09675, %bb55.i ]
  %.lcssa97309760.lcssa9920 = phi <4 x float> [ %.lcssa97309760.lcssa9921, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %.lcssa97309760, %bb55.i ]
  %storemerge.i.i.lcssa97149740.lcssa9908 = phi i32 [ %storemerge.i.i.lcssa97149740.lcssa9909, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %storemerge.i.i.lcssa97149740, %bb55.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i9791, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %ring_cursor.sroa.0.2.i, %bb55.i ], !dbg !17511
  %main_cursor.sroa.0.1.i.lcssa = phi i32 [ %main_cursor.sroa.0.0.i9792, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ], [ %main_cursor.sroa.0.2.i, %bb55.i ], !dbg !17512
  %_110.not.i = icmp eq i32 %977, 0, !dbg !17491
  %indvars.iv.next11057 = add i32 %indvars.iv11056, -32, !dbg !17491
  br i1 %_110.not.i, label %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge, label %bb32.i, !dbg !17491

bb32.i:                                           ; preds = %bb32.i.lr.ph, %bb13.i5.loopexit
  %indvars.iv11056 = phi i32 [ %frames, %bb32.i.lr.ph ], [ %indvars.iv.next11057, %bb13.i5.loopexit ]
  %.lcssa97309760.lcssa9921 = phi <4 x float> [ %.promoted9919, %bb32.i.lr.ph ], [ %.lcssa97309760.lcssa9920, %bb13.i5.loopexit ]
  %storemerge.i.i.lcssa97149740.lcssa9909 = phi i32 [ %_22.i.i.promoted, %bb32.i.lr.ph ], [ %storemerge.i.i.lcssa97149740.lcssa9908, %bb13.i5.loopexit ]
  %history.i.i.sroa.38.0.lcssa9898 = phi <4 x i32> [ %history.i.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa11139, %bb13.i5.loopexit ]
  %history.i.i.sroa.35.0.lcssa9888 = phi <4 x i32> [ %history.i.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa11140, %bb13.i5.loopexit ]
  %history.i.i.sroa.32.0.lcssa9878 = phi <4 x i32> [ %history.i.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa11141, %bb13.i5.loopexit ]
  %history.i.i.sroa.29.0.lcssa9868 = phi <4 x i32> [ %history.i.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa11142, %bb13.i5.loopexit ]
  %history.i.i.sroa.26.0.lcssa9858 = phi <4 x i32> [ %history.i.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa11143, %bb13.i5.loopexit ]
  %history.i.i.sroa.22.0.lcssa9848 = phi <4 x i32> [ %history.i.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa11144, %bb13.i5.loopexit ]
  %history.i.i.sroa.19.0.lcssa9838 = phi <4 x i32> [ %history.i.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa11145, %bb13.i5.loopexit ]
  %history.i.i.sroa.16.0.lcssa9828 = phi <4 x i32> [ %history.i.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa11146, %bb13.i5.loopexit ]
  %history.i.i.sroa.13.0.lcssa9818 = phi <4 x i32> [ %history.i.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa11147, %bb13.i5.loopexit ]
  %history.i.i.sroa.10.0.lcssa9808 = phi <4 x i32> [ %history.i.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa11148, %bb13.i5.loopexit ]
  %history.i.i.sroa.7.0.lcssa9798 = phi <4 x i32> [ %history.i.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa11149, %bb13.i5.loopexit ]
  %iter2.sroa.0.0.i9794 = phi i32 [ %yield_count.sroa.0.0.i3878, %bb32.i.lr.ph ], [ %977, %bb13.i5.loopexit ]
  %iter1.sroa.0.0.i9793 = phi i32 [ 0, %bb32.i.lr.ph ], [ %976, %bb13.i5.loopexit ]
  %main_cursor.sroa.0.0.i9792 = phi i32 [ %_26.i, %bb32.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb13.i5.loopexit ]
  %ring_cursor.sroa.0.0.i9791 = phi i32 [ %_27.i, %bb32.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb13.i5.loopexit ]
  %history.i.i.sroa.0.0.lcssa97799790 = phi <4 x i32> [ %hot_left.i.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa11150, %bb13.i5.loopexit ]
  %umin11070 = call i32 @llvm.umin.i32(i32 %indvars.iv11056, i32 32), !dbg !17513
  %umax11059 = call i32 @llvm.umax.i32(i32 %umin11070, i32 1), !dbg !17513
  %976 = add i32 %iter1.sroa.0.0.i9793, 32, !dbg !17513
  %977 = add nsw i32 %iter2.sroa.0.0.i9794, -1, !dbg !17517
  %978 = sub i32 %frames, %iter1.sroa.0.0.i9793, !dbg !17518
  %spec.store.select.i = tail call i32 @llvm.umin.i32(i32 %978, i32 32), !dbg !17520
  %active_base.i = shl i32 %iter1.sroa.0.0.i9793, 2, !dbg !17525
  %active_base.i8450 = add i32 %spec.store.select.i, %iter1.sroa.0.0.i9793, !dbg !17527
  %_40.i = shl i32 %active_base.i8450, 2, !dbg !17527
  %_120.i = icmp ult i32 %_40.i, %active_base.i, !dbg !17530
  %_114.not.i = icmp ugt i32 %_40.i, %left_io.1
  %or.cond.i = or i1 %_120.i, %_114.not.i, !dbg !17530
  br i1 %or.cond.i, label %bb38.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898, !dbg !17530, !prof !4596

bb38.i:                                           ; preds = %bb32.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %active_base.i, i32 noundef %_40.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_bac0677bfb785f04669fb3d80741c988) #32, !dbg !17537, !noalias !17538
  unreachable, !dbg !17537

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898: ; preds = %bb32.i
  %_123.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %active_base.i, !dbg !17539
  %_2.i39019673.not = icmp eq i32 %frames, %iter1.sroa.0.0.i9793, !dbg !17543
  br i1 %_2.i39019673.not, label %bb13.i5.loopexit, label %bb5.i.i, !dbg !17543

bb5.i.i:                                          ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898, %bb5.i.i
  %history.i.i.sroa.0.09685 = phi <4 x i32> [ %lanes.i3133.sroa.0.0.copyload, %bb5.i.i ], [ %history.i.i.sroa.0.0.lcssa97799790, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.7.09684 = phi <4 x i32> [ %history.i.i.sroa.0.09685, %bb5.i.i ], [ %history.i.i.sroa.7.0.lcssa9798, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.10.09683 = phi <4 x i32> [ %history.i.i.sroa.7.09684, %bb5.i.i ], [ %history.i.i.sroa.10.0.lcssa9808, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.13.09682 = phi <4 x i32> [ %history.i.i.sroa.10.09683, %bb5.i.i ], [ %history.i.i.sroa.13.0.lcssa9818, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.16.09681 = phi <4 x i32> [ %history.i.i.sroa.13.09682, %bb5.i.i ], [ %history.i.i.sroa.16.0.lcssa9828, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.19.09680 = phi <4 x i32> [ %history.i.i.sroa.16.09681, %bb5.i.i ], [ %history.i.i.sroa.19.0.lcssa9838, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.22.09679 = phi <4 x i32> [ %history.i.i.sroa.19.09680, %bb5.i.i ], [ %history.i.i.sroa.22.0.lcssa9848, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.26.09678 = phi <4 x i32> [ %history.i.i.sroa.22.09679, %bb5.i.i ], [ %history.i.i.sroa.26.0.lcssa9858, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.29.09677 = phi <4 x i32> [ %history.i.i.sroa.26.09678, %bb5.i.i ], [ %history.i.i.sroa.29.0.lcssa9868, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.32.09676 = phi <4 x i32> [ %history.i.i.sroa.29.09677, %bb5.i.i ], [ %history.i.i.sroa.32.0.lcssa9878, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %history.i.i.sroa.35.09675 = phi <4 x i32> [ %history.i.i.sroa.32.09676, %bb5.i.i ], [ %history.i.i.sroa.35.0.lcssa9888, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %iter.i.i.sroa.16.09674 = phi i32 [ %1100, %bb5.i.i ], [ 0, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter11ChunksExactfEINtBZ_14ChunksExactMutfEEINtB5_7ZipImplBW_B1w_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3898 ]
  %start1.i.i3906 = shl i32 %iter.i.i.sroa.16.09674, 2, !dbg !17547
  %data.i.i3907 = getelementptr inbounds nuw float, ptr %_123.i, i32 %start1.i.i3906, !dbg !17549
  %lanes.i3133.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i3907, align 4, !dbg !17551, !alias.scope !17556, !noalias !17560
  %979 = bitcast <4 x i32> %history.i.i.sroa.19.09680 to <4 x float>, !dbg !17567
  %980 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %979), !dbg !17572
  %981 = bitcast <4 x i32> %lanes.i3133.sroa.0.0.copyload to <4 x float>, !dbg !17573
  %982 = fmul <4 x float> %_11.i.i.i.i8471, %981, !dbg !17578
  %983 = fadd <4 x float> %982, zeroinitializer, !dbg !17579
  %984 = fmul <4 x float> %_14.i.i.i.i8472, %981, !dbg !17583
  %985 = fadd <4 x float> %984, zeroinitializer, !dbg !17587
  %986 = fmul <4 x float> %_17.i.i.i.i8473, %981, !dbg !17591
  %987 = fadd <4 x float> %986, zeroinitializer, !dbg !17595
  %988 = fmul <4 x float> %_20.i.i.i.i8474, %981, !dbg !17599
  %989 = fadd <4 x float> %988, zeroinitializer, !dbg !17603
  %990 = bitcast <4 x i32> %history.i.i.sroa.0.09685 to <4 x float>, !dbg !17607
  %991 = fmul <4 x float> %_25.i.i.i.i8475, %990, !dbg !17611
  %992 = fadd <4 x float> %983, %991, !dbg !17612
  %993 = fmul <4 x float> %_28.i.i.i.i8476, %990, !dbg !17616
  %994 = fadd <4 x float> %985, %993, !dbg !17620
  %995 = fmul <4 x float> %_31.i.i.i.i8477, %990, !dbg !17624
  %996 = fadd <4 x float> %987, %995, !dbg !17628
  %997 = fmul <4 x float> %_34.i.i.i.i8478, %990, !dbg !17632
  %998 = fadd <4 x float> %989, %997, !dbg !17636
  %999 = bitcast <4 x i32> %history.i.i.sroa.7.09684 to <4 x float>, !dbg !17640
  %1000 = fmul <4 x float> %_39.i.i.i.i8479, %999, !dbg !17644
  %1001 = fadd <4 x float> %992, %1000, !dbg !17645
  %1002 = fmul <4 x float> %_42.i.i.i.i8480, %999, !dbg !17649
  %1003 = fadd <4 x float> %994, %1002, !dbg !17653
  %1004 = fmul <4 x float> %_45.i.i.i.i8481, %999, !dbg !17657
  %1005 = fadd <4 x float> %996, %1004, !dbg !17661
  %1006 = fmul <4 x float> %_48.i.i.i.i8482, %999, !dbg !17665
  %1007 = fadd <4 x float> %998, %1006, !dbg !17669
  %1008 = bitcast <4 x i32> %history.i.i.sroa.10.09683 to <4 x float>, !dbg !17673
  %1009 = fmul <4 x float> %_53.i.i.i.i8483, %1008, !dbg !17677
  %1010 = fadd <4 x float> %1001, %1009, !dbg !17678
  %1011 = fmul <4 x float> %_56.i.i.i.i8484, %1008, !dbg !17682
  %1012 = fadd <4 x float> %1003, %1011, !dbg !17686
  %1013 = fmul <4 x float> %_59.i.i.i.i8485, %1008, !dbg !17690
  %1014 = fadd <4 x float> %1005, %1013, !dbg !17694
  %1015 = fmul <4 x float> %_62.i.i.i.i8486, %1008, !dbg !17698
  %1016 = fadd <4 x float> %1007, %1015, !dbg !17702
  %1017 = bitcast <4 x i32> %history.i.i.sroa.13.09682 to <4 x float>, !dbg !17706
  %1018 = fmul <4 x float> %_67.i.i.i.i8487, %1017, !dbg !17710
  %1019 = fadd <4 x float> %1010, %1018, !dbg !17711
  %1020 = fmul <4 x float> %_70.i.i.i.i8488, %1017, !dbg !17715
  %1021 = fadd <4 x float> %1012, %1020, !dbg !17719
  %1022 = fmul <4 x float> %_73.i.i.i.i8489, %1017, !dbg !17723
  %1023 = fadd <4 x float> %1014, %1022, !dbg !17727
  %1024 = fmul <4 x float> %_76.i.i.i.i8490, %1017, !dbg !17731
  %1025 = fadd <4 x float> %1016, %1024, !dbg !17735
  %1026 = bitcast <4 x i32> %history.i.i.sroa.16.09681 to <4 x float>, !dbg !17739
  %1027 = fmul <4 x float> %_81.i.i.i.i8491, %1026, !dbg !17743
  %1028 = fadd <4 x float> %1019, %1027, !dbg !17744
  %1029 = fmul <4 x float> %_84.i.i.i.i8492, %1026, !dbg !17748
  %1030 = fadd <4 x float> %1021, %1029, !dbg !17752
  %1031 = fmul <4 x float> %_87.i.i.i.i8493, %1026, !dbg !17756
  %1032 = fadd <4 x float> %1023, %1031, !dbg !17760
  %1033 = fmul <4 x float> %_90.i.i.i.i8494, %1026, !dbg !17764
  %1034 = fadd <4 x float> %1025, %1033, !dbg !17768
  %1035 = fmul <4 x float> %_95.i.i.i.i8495, %979, !dbg !17772
  %1036 = fadd <4 x float> %1028, %1035, !dbg !17776
  %1037 = fmul <4 x float> %_98.i.i.i.i8496, %979, !dbg !17780
  %1038 = fadd <4 x float> %1030, %1037, !dbg !17784
  %1039 = fmul <4 x float> %_101.i.i.i.i8497, %979, !dbg !17788
  %1040 = fadd <4 x float> %1032, %1039, !dbg !17792
  %1041 = fmul <4 x float> %_104.i.i.i.i8498, %979, !dbg !17796
  %1042 = fadd <4 x float> %1034, %1041, !dbg !17800
  %1043 = bitcast <4 x i32> %history.i.i.sroa.22.09679 to <4 x float>, !dbg !17804
  %1044 = fmul <4 x float> %_109.i.i.i.i8499, %1043, !dbg !17808
  %1045 = fadd <4 x float> %1036, %1044, !dbg !17809
  %1046 = fmul <4 x float> %_112.i.i.i.i8500, %1043, !dbg !17813
  %1047 = fadd <4 x float> %1038, %1046, !dbg !17817
  %1048 = fmul <4 x float> %_115.i.i.i.i8501, %1043, !dbg !17821
  %1049 = fadd <4 x float> %1040, %1048, !dbg !17825
  %1050 = fmul <4 x float> %_118.i.i.i.i8502, %1043, !dbg !17829
  %1051 = fadd <4 x float> %1042, %1050, !dbg !17833
  %1052 = bitcast <4 x i32> %history.i.i.sroa.26.09678 to <4 x float>, !dbg !17837
  %1053 = fmul <4 x float> %_123.i.i.i.i8503, %1052, !dbg !17841
  %1054 = fadd <4 x float> %1045, %1053, !dbg !17842
  %1055 = fmul <4 x float> %_126.i.i.i.i8504, %1052, !dbg !17846
  %1056 = fadd <4 x float> %1047, %1055, !dbg !17850
  %1057 = fmul <4 x float> %_129.i.i.i.i8505, %1052, !dbg !17854
  %1058 = fadd <4 x float> %1049, %1057, !dbg !17858
  %1059 = fmul <4 x float> %_132.i.i.i.i8506, %1052, !dbg !17862
  %1060 = fadd <4 x float> %1051, %1059, !dbg !17866
  %1061 = bitcast <4 x i32> %history.i.i.sroa.29.09677 to <4 x float>, !dbg !17870
  %1062 = fmul <4 x float> %_137.i.i.i.i8507, %1061, !dbg !17874
  %1063 = fadd <4 x float> %1054, %1062, !dbg !17875
  %1064 = fmul <4 x float> %_140.i.i.i.i8508, %1061, !dbg !17879
  %1065 = fadd <4 x float> %1056, %1064, !dbg !17883
  %1066 = fmul <4 x float> %_143.i.i.i.i8509, %1061, !dbg !17887
  %1067 = fadd <4 x float> %1058, %1066, !dbg !17891
  %1068 = fmul <4 x float> %_146.i.i.i.i8510, %1061, !dbg !17895
  %1069 = fadd <4 x float> %1060, %1068, !dbg !17899
  %1070 = bitcast <4 x i32> %history.i.i.sroa.32.09676 to <4 x float>, !dbg !17903
  %1071 = fmul <4 x float> %_151.i.i.i.i8511, %1070, !dbg !17907
  %1072 = fadd <4 x float> %1063, %1071, !dbg !17908
  %1073 = fmul <4 x float> %_154.i.i.i.i8512, %1070, !dbg !17912
  %1074 = fadd <4 x float> %1065, %1073, !dbg !17916
  %1075 = fmul <4 x float> %_157.i.i.i.i8513, %1070, !dbg !17920
  %1076 = fadd <4 x float> %1067, %1075, !dbg !17924
  %1077 = fmul <4 x float> %_160.i.i.i.i8514, %1070, !dbg !17928
  %1078 = fadd <4 x float> %1069, %1077, !dbg !17932
  %1079 = bitcast <4 x i32> %history.i.i.sroa.35.09675 to <4 x float>, !dbg !17936
  %1080 = fmul <4 x float> %_165.i.i.i.i8515, %1079, !dbg !17940
  %1081 = fadd <4 x float> %1072, %1080, !dbg !17941
  %1082 = fmul <4 x float> %_168.i.i.i.i8516, %1079, !dbg !17945
  %1083 = fadd <4 x float> %1074, %1082, !dbg !17949
  %1084 = fmul <4 x float> %_171.i.i.i.i8517, %1079, !dbg !17953
  %1085 = fadd <4 x float> %1076, %1084, !dbg !17957
  %1086 = fmul <4 x float> %_174.i.i.i.i8518, %1079, !dbg !17961
  %1087 = fadd <4 x float> %1078, %1086, !dbg !17965
  %1088 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1081), !dbg !17969
  %1089 = fcmp olt <4 x float> %1088, %980, !dbg !17973
  %1090 = select <4 x i1> %1089, <4 x float> %980, <4 x float> %1088, !dbg !17977
  %1091 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1083), !dbg !17969
  %1092 = fcmp olt <4 x float> %1091, %1090, !dbg !17973
  %1093 = select <4 x i1> %1092, <4 x float> %1090, <4 x float> %1091, !dbg !17977
  %1094 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1085), !dbg !17969
  %1095 = fcmp olt <4 x float> %1094, %1093, !dbg !17973
  %1096 = select <4 x i1> %1095, <4 x float> %1093, <4 x float> %1094, !dbg !17977
  %1097 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1087), !dbg !17969
  %1098 = fcmp olt <4 x float> %1097, %1096, !dbg !17973
  %1099 = select <4 x i1> %1098, <4 x float> %1096, <4 x float> %1097, !dbg !17977
  %1100 = add nuw nsw i32 %iter.i.i.sroa.16.09674, 1, !dbg !17978
  %data.i4.i3911 = getelementptr inbounds nuw float, ptr %peaks_left.i, i32 %start1.i.i3906, !dbg !17979
  store <4 x float> %1099, ptr %data.i4.i3911, align 4, !dbg !17982, !alias.scope !17987, !noalias !17991
  %exitcond11060.not = icmp eq i32 %1100, %umax11059, !dbg !17543
  br i1 %exitcond11060.not, label %bb17.i.lr.ph, label %bb5.i.i, !dbg !17543

bb17.i.lr.ph:                                     ; preds = %bb5.i.i
  br label %bb17.i, !dbg !17995

bb17.i:                                           ; preds = %bb17.i.lr.ph, %bb55.i
  %.lcssa97309761 = phi <4 x float> [ %.lcssa97309760.lcssa9921, %bb17.i.lr.ph ], [ %.lcssa97309760, %bb55.i ]
  %storemerge.i.i.lcssa97149741 = phi i32 [ %storemerge.i.i.lcssa97149740.lcssa9909, %bb17.i.lr.ph ], [ %storemerge.i.i.lcssa97149740, %bb55.i ]
  %frame.sroa.0.0.i9736 = phi i32 [ 0, %bb17.i.lr.ph ], [ %_68.i, %bb55.i ]
  %main_cursor.sroa.0.1.i9735 = phi i32 [ %main_cursor.sroa.0.0.i9792, %bb17.i.lr.ph ], [ %main_cursor.sroa.0.2.i, %bb55.i ]
  %ring_cursor.sroa.0.1.i9734 = phi i32 [ %ring_cursor.sroa.0.0.i9791, %bb17.i.lr.ph ], [ %ring_cursor.sroa.0.2.i, %bb55.i ]
  %_51.i = sub nuw nsw i32 %spec.store.select.i, %frame.sroa.0.0.i9736, !dbg !17997
  %ring.i1347 = load i32, ptr %924, align 4, !dbg !17998, !alias.scope !18000, !noalias !18003, !noundef !10
  %main.i1348 = load i32, ptr %925, align 4, !dbg !18007, !alias.scope !18000, !noalias !18003, !noundef !10
  %_10.i1349 = add i32 %ring_cursor.sroa.0.1.i9734, 1, !dbg !18008
  %_38.not.i1350 = icmp ult i32 %_10.i1349, %ring.i1347, !dbg !18009
  %1101 = select i1 %_38.not.i1350, i32 0, i32 %ring.i1347, !dbg !18009
  %start1.sroa.0.0.i1351 = sub nuw i32 %_10.i1349, %1101, !dbg !18009
  %_12.i1353 = add i32 %_54.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9734, !dbg !18011
  %_39.not.i1354 = icmp ult i32 %_12.i1353, %ring.i1347, !dbg !18012
  %1102 = select i1 %_39.not.i1354, i32 0, i32 %ring.i1347, !dbg !18012
  %left_end.sroa.0.0.i1355 = sub nuw i32 %_12.i1353, %1102, !dbg !18012
  %_18.i1361 = add i32 %_54.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9734, !dbg !18014
  %_41.not.i1362 = icmp ult i32 %_18.i1361, %ring.i1347, !dbg !18015
  %1103 = select i1 %_41.not.i1362, i32 0, i32 %ring.i1347, !dbg !18015
  %left_expiring.sroa.0.0.i1363 = sub nuw i32 %_18.i1361, %1103, !dbg !18015
  %1104 = sub i32 %ring.i1347, %ring_cursor.sroa.0.1.i9734, !dbg !18017
  %spec.store.select.i1368 = tail call i32 @llvm.umin.i32(i32 %1104, i32 %_51.i), !dbg !18018
  %1105 = sub i32 %main.i1348, %main_cursor.sroa.0.1.i9735, !dbg !18020
  %_24.sroa.0.0.i1370 = tail call i32 @llvm.umin.i32(i32 %1105, i32 %spec.store.select.i1368), !dbg !18021
  %1106 = sub i32 %ring.i1347, %start1.sroa.0.0.i1351, !dbg !18023
  %_25.sroa.0.0.i1372 = tail call i32 @llvm.umin.i32(i32 %1106, i32 %_24.sroa.0.0.i1370), !dbg !18024
  %1107 = sub i32 %ring.i1347, %left_end.sroa.0.0.i1355, !dbg !18026
  %_27.sroa.0.0.i1374 = tail call i32 @llvm.umin.i32(i32 %1107, i32 %_25.sroa.0.0.i1372), !dbg !18027
  %1108 = sub i32 %ring.i1347, %left_expiring.sroa.0.0.i1363, !dbg !18029
  %_31.sroa.0.0.i1378 = tail call i32 @llvm.umin.i32(i32 %1108, i32 %_27.sroa.0.0.i1374), !dbg !18030
  %_58.i = add i32 %frame.sroa.0.0.i9736, %iter1.sroa.0.0.i9793, !dbg !18032
  %base.i = shl i32 %_58.i, 2, !dbg !18032
  %base.i8452 = add i32 %_31.sroa.0.0.i1378, %_58.i, !dbg !18035
  %_62.i = shl i32 %base.i8452, 2, !dbg !18035
  %_135.i = icmp ult i32 %_62.i, %base.i, !dbg !18038
  %_131.not.i = icmp ugt i32 %_62.i, %left_io.1
  %or.cond16.i = or i1 %_135.i, %_131.not.i, !dbg !18038
  br i1 %or.cond16.i, label %bb46.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3945, !dbg !18038, !prof !4596

bb46.i:                                           ; preds = %bb17.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i, i32 noundef %_62.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1eb3474cf2d0fe655b193b0c53e74d06) #32, !dbg !18046, !noalias !17538
  unreachable, !dbg !18046

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3945: ; preds = %bb17.i
  %_138.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i, !dbg !18047
  %_68.i = add nuw nsw i32 %_31.sroa.0.0.i1378, %frame.sroa.0.0.i9736, !dbg !18051
  %_147.i.idx = shl nuw nsw i32 %frame.sroa.0.0.i9736, 4, !dbg !18053
  %_147.i = getelementptr inbounds nuw i8, ptr %peaks_left.i, i32 %_147.i.idx, !dbg !18053
  %_2.i39489701.not = icmp eq i32 %_31.sroa.0.0.i1378, 0, !dbg !18062
  br i1 %_2.i39489701.not, label %bb55.i, label %bb56.i.preheader, !dbg !18062

bb56.i.preheader:                                 ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3945
  %umin11066 = call i32 @llvm.umin.i32(i32 %1107, i32 %1108)
  %umin11067 = call i32 @llvm.umin.i32(i32 %umin11066, i32 %1106)
  %umin11068 = call i32 @llvm.umin.i32(i32 %umin11067, i32 %1104)
  %umin11069 = call i32 @llvm.umin.i32(i32 %umin11068, i32 %1105)
  %1109 = sub nsw i32 %umin11070, %frame.sroa.0.0.i9736
  %umin11071 = call i32 @llvm.umin.i32(i32 %umin11069, i32 %1109)
  %1110 = and i32 %umin11071, 1073741823
  br label %bb56.i

bb56.i:                                           ; preds = %bb56.i.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169
  %1111 = phi <4 x float> [ %.lcssa97309761, %bb56.i.preheader ], [ %1144, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169 ]
  %storemerge.i.i9704 = phi i32 [ %storemerge.i.i.lcssa97149741, %bb56.i.preheader ], [ %storemerge.i.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169 ]
  %iter.i.sroa.16.09703 = phi i32 [ 0, %bb56.i.preheader ], [ %1112, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169 ]
  %start1.i.i3953 = shl i32 %iter.i.sroa.16.09703, 2, !dbg !18071
  %data.i4.i3959 = getelementptr inbounds nuw float, ptr %_147.i, i32 %start1.i.i3953, !dbg !18073
  %lanes.i3124.sroa.0.0.copyload = load <16 x i8>, ptr %data.i4.i3959, align 4, !dbg !18076, !alias.scope !18083, !noalias !18087
  %data.i.i3954 = getelementptr inbounds nuw float, ptr %_138.i, i32 %start1.i.i3953, !dbg !18091
  %lanes.i3115.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i3954, align 4, !dbg !18093, !alias.scope !18101, !noalias !18105
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18109), !dbg !18112
  %1112 = add nuw nsw i32 %iter.i.sroa.16.09703, 1, !dbg !18114
  %_159.i = add i32 %iter.i.sroa.16.09703, %ring_cursor.sroa.0.1.i9734, !dbg !18115
  %_160.i = add i32 %iter.i.sroa.16.09703, %main_cursor.sroa.0.1.i9735, !dbg !18118
  %_161.i = add i32 %iter.i.sroa.16.09703, %left_end.sroa.0.0.i1355, !dbg !18119
  %_162.i = add i32 %iter.i.sroa.16.09703, %start1.sroa.0.0.i1351, !dbg !18120
  %_163.i = add i32 %iter.i.sroa.16.09703, %left_expiring.sroa.0.0.i1363, !dbg !18121
  %base.i9.i.i = shl i32 %_159.i, 2, !dbg !18122
  %_7.i10.i.i = add i32 %base.i9.i.i, 4, !dbg !18125
  %1113 = or disjoint i32 %base.i9.i.i, 3, !dbg !18126
  %or.cond.i13.i.i.not = icmp ult i32 %1113, %_54.1.i.i, !dbg !18126
  br i1 %or.cond.i13.i.i.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i, label %bb4.i15.i.i, !dbg !18126, !prof !10564

bb4.i15.i.i:                                      ; preds = %bb56.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i.i, i32 noundef %_7.i10.i.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !18130, !noalias !18131
  unreachable, !dbg !18130

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i: ; preds = %bb56.i
  %_4.i3964 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %lanes.i3124.sroa.0.0.copyload, <16 x i8> %lanes.i3124.sroa.0.0.copyload, <16 x i8> %965), !dbg !18143
  %1114 = bitcast <16 x i8> %_4.i3964 to <4 x float>, !dbg !18147
  %1115 = fdiv <4 x float> %_8.i.i8453, %1114, !dbg !18152
  %1116 = bitcast <4 x float> %1115 to <16 x i8>, !dbg !18156
  %1117 = fcmp olt <4 x float> %_8.i.i8453, %1114, !dbg !18160
  %1118 = sext <4 x i1> %1117 to <4 x i32>, !dbg !18160
  %1119 = bitcast <4 x i32> %1118 to <16 x i8>, !dbg !18161
  %_4.i3965 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1116, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1119), !dbg !18162
  %_17.i14.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i9.i.i, !dbg !18163
  store <16 x i8> %_4.i3965, ptr %_17.i14.i.i, align 4, !dbg !18165, !alias.scope !18170, !noalias !18174
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18178), !dbg !18181
  %base.i1188 = shl i32 %_161.i, 2, !dbg !18182
  %1120 = or disjoint i32 %base.i1188, 3, !dbg !18185
  %or.cond.i1192.not = icmp ult i32 %1120, %_54.1.i.i, !dbg !18185
  br i1 %or.cond.i1192.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1196, label %bb4.i1195, !dbg !18185, !prof !10564

bb4.i1195:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i
  %_5.i1189 = add i32 %base.i1188, 4, !dbg !18189
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1188, i32 noundef %_5.i1189, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !18190, !noalias !18191
  unreachable, !dbg !18190

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1196: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i
  %_15.i1194 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1188, !dbg !18197
  %lanes.i2904.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1194, align 4, !dbg !18199
  %1121 = icmp eq i32 %storemerge.i.i9704, 0, !dbg !18204
  %_12.i26.i8456 = load <4 x float>, ptr %uniform_left.i, align 16, !dbg !18204
  %1122 = bitcast <4 x i32> %lanes.i2904.sroa.0.0.copyload to <4 x float>, !dbg !18204
  %1123 = fcmp olt <4 x float> %_12.i26.i8456, %1122, !dbg !18204
  %1124 = select <4 x i1> %1123, <4 x float> %_12.i26.i8456, <4 x float> %1122, !dbg !18204
  %1125 = bitcast <4 x float> %1124 to <4 x i32>, !dbg !18204
  %.sroa.04444.0 = select i1 %1121, <4 x i32> %lanes.i2904.sroa.0.0.copyload, <4 x i32> %1125, !dbg !18204
  store <4 x i32> %.sroa.04444.0, ptr %uniform_left.i, align 16, !dbg !18205, !alias.scope !18178, !noalias !18206
  %_15.i.i = add i32 %storemerge.i.i9704, 1, !dbg !18208
  %complete.i.i = icmp eq i32 %_15.i.i, %_18.i.i, !dbg !18208
  br i1 %complete.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160, label %bb7.i.i, !dbg !18209

bb7.i.i:                                          ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1196
  %base.i1179 = shl i32 %_162.i, 2, !dbg !18210
  %1126 = or disjoint i32 %base.i1179, 3, !dbg !18212
  %or.cond.i1183.not = icmp ult i32 %1126, %_54.1.i.i, !dbg !18212
  br i1 %or.cond.i1183.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1187, label %bb4.i1186, !dbg !18212, !prof !10564

bb4.i1186:                                        ; preds = %bb7.i.i
  %_5.i1180 = add i32 %base.i1179, 4, !dbg !18216
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1179, i32 noundef %_5.i1180, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !18217, !noalias !18218
  unreachable, !dbg !18217

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1187: ; preds = %bb7.i.i
  %_15.i1185 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1179, !dbg !18222
  %lanes.i2911.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1185, align 4, !dbg !18224, !alias.scope !18229, !noalias !18233
  %1127 = bitcast <4 x i32> %.sroa.04444.0 to <4 x float>, !dbg !18237
  %1128 = fcmp olt <4 x float> %lanes.i2911.sroa.0.0.copyload, %1127, !dbg !18241
  %1129 = select <4 x i1> %1128, <4 x float> %lanes.i2911.sroa.0.0.copyload, <4 x float> %1127, !dbg !18242
  %1130 = bitcast <4 x float> %1129 to <4 x i32>, !dbg !18243
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, !dbg !18245

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1196
  %1131 = bitcast <4 x i32> %lanes.i2904.sroa.0.0.copyload to <4 x float>, !dbg !18209
  br i1 %_29.i.i9698.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb19.i.i, !dbg !18246

bb19.i.i:                                         ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151
  %end.sroa.0.0.i.i9700 = phi i32 [ %1137, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ], [ %_161.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160 ]
  %iter.sroa.0.0.i.i9699 = phi i32 [ %_30.i31.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160 ]
  %1132 = phi <4 x float> [ %1135, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ], [ %1131, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160 ]
  %base.i1143 = shl i32 %end.sroa.0.0.i.i9700, 2, !dbg !18249
  %1133 = or disjoint i32 %base.i1143, 3, !dbg !18251
  %or.cond.i1147.not = icmp ult i32 %1133, %_54.1.i.i, !dbg !18251
  br i1 %or.cond.i1147.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151, label %bb4.i1150, !dbg !18251, !prof !10564

bb4.i1150:                                        ; preds = %bb19.i.i
  %_5.i1144 = add i32 %base.i1143, 4, !dbg !18255
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1143, i32 noundef %_5.i1144, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !18256, !noalias !18257
  unreachable, !dbg !18256

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151: ; preds = %bb19.i.i
  %_30.i31.i = add nuw i32 %iter.sroa.0.0.i.i9699, 1, !dbg !18261
  %_15.i1149 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1143, !dbg !18264
  %lanes.i2939.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1149, align 4, !dbg !18266, !alias.scope !18271, !noalias !18275
  %1134 = fcmp olt <4 x float> %1132, %lanes.i2939.sroa.0.0.copyload, !dbg !18279
  %1135 = select <4 x i1> %1134, <4 x float> %1132, <4 x float> %lanes.i2939.sroa.0.0.copyload, !dbg !18283
  store <4 x float> %1135, ptr %_15.i1149, align 4, !dbg !18284, !alias.scope !18290, !noalias !18294
  %1136 = icmp eq i32 %end.sroa.0.0.i.i9700, 0, !dbg !18300
  %spec.store.select.i.i = select i1 %1136, i32 %ring.i, i32 %end.sroa.0.0.i.i9700, !dbg !18300
  %1137 = add i32 %spec.store.select.i.i, -1, !dbg !18301
  %exitcond11062.not = icmp eq i32 %_30.i31.i, %_18.i.i, !dbg !18302
  br i1 %exitcond11062.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb19.i.i, !dbg !18246

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1187
  %.sroa.04444.1 = phi <4 x i32> [ %1130, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1187 ], [ %.sroa.04444.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160 ], [ %.sroa.04444.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ], !dbg !18304
  %storemerge.i.i = phi i32 [ %_15.i.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1187 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ], !dbg !18305
  %1138 = bitcast <4 x i32> %.sroa.04444.1 to <4 x float>, !dbg !18306
  %1139 = fmul <4 x float> %1138, splat (float 1.638400e+04), !dbg !18310
  %1140 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1139), !dbg !18311
  %1141 = fmul <4 x float> %1140, splat (float 0x3F10000000000000), !dbg !18315
  %base.i1170 = shl i32 %_163.i, 2, !dbg !18319
  %1142 = or disjoint i32 %base.i1170, 3, !dbg !18321
  %or.cond.i1174.not = icmp ult i32 %1142, %_56.1.i.i, !dbg !18321
  br i1 %or.cond.i1174.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1178, label %bb4.i1177, !dbg !18321, !prof !10564

bb4.i1177:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i
  %_5.i1171 = add i32 %base.i1170, 4, !dbg !18325
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1170, i32 noundef %_5.i1171, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !18326, !noalias !18327
  unreachable, !dbg !18326

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1178: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i
  %_15.i1176 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %base.i1170, !dbg !18331
  %lanes.i2918.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1176, align 4, !dbg !18333, !alias.scope !18338, !noalias !18342
  %1143 = fadd <4 x float> %1141, %1111, !dbg !18346
  %1144 = fsub <4 x float> %1143, %lanes.i2918.sroa.0.0.copyload, !dbg !18350
  %_8.not.i4.i.i = icmp ugt i32 %_7.i10.i.i, %_56.1.i.i
  br i1 %_8.not.i4.i.i, label %bb4.i7.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i, !dbg !18354, !prof !4596

bb4.i7.i.i:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1178
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i.i, i32 noundef %_7.i10.i.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #32, !dbg !18359, !noalias !18360
  unreachable, !dbg !18359

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1178
  %_17.i6.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %base.i9.i.i, !dbg !18364
  store <4 x float> %1141, ptr %_17.i6.i.i, align 4, !dbg !18366, !alias.scope !18371, !noalias !18375
  %_41.i.i8463 = load <4 x float>, ptr %972, align 16, !dbg !18379, !alias.scope !18109, !noalias !18380
  %1145 = fdiv <4 x float> %1144, %_37.i.i8462, !dbg !18381
  %1146 = fsub <4 x float> splat (float 1.000000e+00), %1145, !dbg !18385
  %1147 = fsub <4 x float> %1146, %_41.i.i8463, !dbg !18389
  %1148 = fmul <4 x float> %_9.i.i8454, %1147, !dbg !18393
  %1149 = fadd <4 x float> %_41.i.i8463, %1148, !dbg !18397
  %1150 = fcmp olt <4 x float> %1149, %1146, !dbg !18400
  %1151 = select <4 x i1> %1150, <4 x float> %1146, <4 x float> %1149, !dbg !18404
  %1152 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1151), !dbg !18405
  %1153 = fcmp uge <4 x float> %1152, splat (float 0x3BC79CA100000000), !dbg !18410
  %1154 = bitcast <4 x float> %1151 to <4 x i32>, !dbg !18415
  %1155 = select <4 x i1> %1153, <4 x i32> %1154, <4 x i32> zeroinitializer, !dbg !18415
  store <4 x i32> %1155, ptr %972, align 16, !dbg !18418, !alias.scope !18109, !noalias !18380
  %base.i1161 = shl i32 %_160.i, 2, !dbg !18419
  %1156 = or disjoint i32 %base.i1161, 3, !dbg !18421
  %or.cond.i1165.not = icmp ult i32 %1156, %_58.1.i.i, !dbg !18421
  br i1 %or.cond.i1165.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169, label %bb4.i1168, !dbg !18421, !prof !10564

bb4.i1168:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i
  %_5.i1162 = add i32 %base.i1161, 4, !dbg !18425
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1161, i32 noundef %_5.i1162, i32 noundef range(i32 0, 536870912) %_58.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #32, !dbg !18426, !noalias !18427
  unreachable, !dbg !18426

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i
  %1157 = bitcast <4 x i32> %1155 to <4 x float>, !dbg !18431
  %1158 = fsub <4 x float> splat (float 1.000000e+00), %1157, !dbg !18435
  %_15.i1167 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i32 %base.i1161, !dbg !18436
  %lanes.i2925.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1167, align 4, !dbg !18438, !alias.scope !18443, !noalias !18447
  store <4 x i32> %lanes.i3115.sroa.0.0.copyload, ptr %_15.i1167, align 4, !dbg !18451, !alias.scope !18457, !noalias !18461
  %1159 = bitcast <4 x i32> %lanes.i2925.sroa.0.0.copyload to <4 x float>, !dbg !18467
  %1160 = fmul <4 x float> %1158, %1159, !dbg !18471
  %1161 = bitcast <4 x i32> %lanes.i2925.sroa.0.0.copyload to <16 x i8>, !dbg !18472
  %1162 = bitcast <4 x float> %1160 to <16 x i8>, !dbg !18476
  %_4.i3966 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1161, <16 x i8> %1162, <16 x i8> %975), !dbg !18477
  store <16 x i8> %_4.i3966, ptr %data.i.i3954, align 4, !dbg !18478, !alias.scope !18483, !noalias !18487
  %exitcond11072.not = icmp eq i32 %1112, %1110, !dbg !18062
  br i1 %exitcond11072.not, label %bb55.i, label %bb56.i, !dbg !18062

bb55.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3945
  %.lcssa97309760 = phi <4 x float> [ %.lcssa97309761, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3945 ], [ %1144, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169 ]
  %storemerge.i.i.lcssa97149740 = phi i32 [ %storemerge.i.i.lcssa97149741, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3945 ], [ %storemerge.i.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169 ]
  %_96.i = add i32 %_31.sroa.0.0.i1378, %ring_cursor.sroa.0.1.i9734, !dbg !18491
  %_158.not.i = icmp ult i32 %_96.i, %ring.i, !dbg !18492
  %1163 = select i1 %_158.not.i, i32 0, i32 %ring.i, !dbg !18492
  %ring_cursor.sroa.0.2.i = sub nuw i32 %_96.i, %1163, !dbg !18492
  %_98.i = add i32 %_31.sroa.0.0.i1378, %main_cursor.sroa.0.1.i9735, !dbg !18495
  %_164.not.i = icmp ult i32 %_98.i, %main.i, !dbg !18496
  %1164 = select i1 %_164.not.i, i32 0, i32 %main.i, !dbg !18496
  %main_cursor.sroa.0.2.i = sub nuw i32 %_98.i, %1164, !dbg !18496
  %_45.i = icmp ult i32 %_68.i, %spec.store.select.i, !dbg !17995
  br i1 %_45.i, label %bb17.i, label %bb13.i5.loopexit, !dbg !17995

bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge: ; preds = %bb13.i5.loopexit
  store <4 x i32> %history.i.i.sroa.7.0.lcssa11149, ptr %history.i.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.10.0.lcssa11148, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.13.0.lcssa11147, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.16.0.lcssa11146, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.19.0.lcssa11145, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.22.0.lcssa11144, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.26.0.lcssa11143, ptr %history.i.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.29.0.lcssa11142, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.32.0.lcssa11141, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.35.0.lcssa11140, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x i32> %history.i.i.sroa.38.0.lcssa11139, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !18498
  store <4 x float> %.lcssa97309760.lcssa9920, ptr %970, align 16
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !17491

_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge
  %left_phase.i = phi i32 [ %storemerge.i.i.lcssa97149740.lcssa9908, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %left_phase.i.pre, %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], !dbg !17501
  %history.i.i.sroa.0.0.lcssa9779.lcssa = phi <4 x i32> [ %history.i.i.sroa.0.0.lcssa11150, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %hot_left.i.promoted, %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ]
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %ring_cursor.sroa.0.1.i.lcssa, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %_27.i, %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], !dbg !17484
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %main_cursor.sroa.0.1.i.lcssa, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %_26.i, %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], !dbg !17481
  store <4 x i32> %history.i.i.sroa.0.0.lcssa9779.lcssa, ptr %hot_left.i, align 1, !dbg !18498
  %left_prefix.i = load <4 x i32>, ptr %uniform_left.i, align 16, !dbg !18499, !noalias !17465
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !18500, !noalias !17465
  %1165 = getelementptr inbounds nuw i8, ptr %self, i32 968, !dbg !18501
  %_171.1.i = load i32, ptr %1165, align 4, !dbg !18501, !alias.scope !17457, !noalias !17468, !noundef !10
  %_8.i3474 = icmp samesign ugt i32 %_171.1.i, 3, !dbg !18503
  br i1 %_8.i3474, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3477, label %bb2.i3475, !dbg !18503, !prof !1039

bb2.i3475:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_171.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #32, !dbg !18508, !noalias !18509
  unreachable, !dbg !18508

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3477: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %1166 = getelementptr inbounds nuw i8, ptr %self, i32 964, !dbg !18501
  %_171.0.i = load ptr, ptr %1166, align 4, !dbg !18501, !alias.scope !17457, !noalias !17468, !nonnull !10, !noundef !10
  store <4 x i32> %left_prefix.i, ptr %_171.0.i, align 4, !dbg !18513, !alias.scope !18517, !noalias !18521
  %_172.0.i = load ptr, ptr %50, align 4, !dbg !18523, !alias.scope !17457, !noalias !17468, !nonnull !10, !noundef !10
  %_172.1.i = load i32, ptr %51, align 4, !dbg !18523, !alias.scope !17457, !noalias !17468, !noundef !10
  %1167 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !18524
  br i1 %1167, label %bb2.i3976, label %bb6.i3967, !dbg !18524

bb6.i3967:                                        ; preds = %bb2.i3976, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3477
  %end_or_len.idx.i3968 = shl nuw nsw i32 %_172.1.i, 2, !dbg !18528
  %end_or_len.i3969 = getelementptr inbounds nuw i8, ptr %_172.0.i, i32 %end_or_len.idx.i3968, !dbg !18528
  %_293.i3970 = icmp eq i32 %_172.1.i, 0, !dbg !18532
  br i1 %_293.i3970, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3982, label %bb10.i3971, !dbg !18535

bb2.i3976:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3477
  %bytes1.sroa.0.0.zext.i3977 = and i32 %left_phase.i, 255, !dbg !18536
  %bytes1.sroa.0.0.isplat.i3978 = mul nuw i32 %bytes1.sroa.0.0.zext.i3977, 16843009, !dbg !18536
  %_5.i3979 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i3978, !dbg !18537
  br i1 %_5.i3979, label %bb3.i3980, label %bb6.i3967, !dbg !18537

bb3.i3980:                                        ; preds = %bb2.i3976
  %bytes.sroa.0.0.extract.trunc.i3981 = trunc i32 %left_phase.i to i8, !dbg !18538
  %1168 = shl nuw nsw i32 %_172.1.i, 2, !dbg !18540
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_172.0.i, i8 %bytes.sroa.0.0.extract.trunc.i3981, i32 %1168, i1 false), !dbg !18540, !alias.scope !18541, !noalias !17538
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3982, !dbg !18544

bb10.i3971:                                       ; preds = %bb6.i3967, %bb10.i3971
  %iter.sroa.0.04.i3972 = phi ptr [ %_38.i3973, %bb10.i3971 ], [ %_172.0.i, %bb6.i3967 ]
  %_38.i3973 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i3972, i32 4, !dbg !18545
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i3972, align 4, !dbg !18547, !alias.scope !18541, !noalias !17538
  %_29.i3974 = icmp eq ptr %_38.i3973, %end_or_len.i3969, !dbg !18532
  br i1 %_29.i3974, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3982, label %bb10.i3971, !dbg !18535

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3982: ; preds = %bb10.i3971, %bb6.i3967, %bb3.i3980
  call void @llvm.lifetime.start.p0(ptr nonnull %_105.i), !dbg !18548, !noalias !17465
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 16 dereferenceable(368) %_105.i, ptr noundef nonnull align 16 dereferenceable(368) %hot_left.i, i32 368, i1 false), !dbg !18548, !noalias !17465
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %_105.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24) #31, !dbg !18549, !noalias !17538
  call void @llvm.lifetime.end.p0(ptr nonnull %_105.i), !dbg !18550, !noalias !17465
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_25, align 4, !dbg !18551, !alias.scope !17459, !noalias !17483
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %926, align 4, !dbg !18552, !alias.scope !17459, !noalias !17483
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !18553, !noalias !17465
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i), !dbg !18554, !noalias !17465
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !17454

_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3982
  br i1 %quiet.sroa.0.0.off08153, label %bb18, label %bb24, !dbg !18555

bb13:                                             ; preds = %bb11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18556), !dbg !18559
  %1169 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !18560
  %_40.0.i = load ptr, ptr %1169, align 4, !dbg !18560, !alias.scope !18556, !nonnull !10, !noundef !10
  %1170 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !18560
  %_40.1.i = load i32, ptr %1170, align 4, !dbg !18560, !alias.scope !18556, !noundef !10
  %1171 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !18562
  %_41.0.i = load ptr, ptr %1171, align 4, !dbg !18562, !alias.scope !18556, !nonnull !10, !noundef !10
  %1172 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !18562
  %_41.1.i = load i32, ptr %1172, align 4, !dbg !18562, !alias.scope !18556, !noundef !10
  %spec.store.select.i.i3983 = tail call i32 @llvm.umin.i32(i32 %_41.1.i, i32 %_40.1.i), !dbg !18563
  %_2.i6.not.i = icmp eq i32 %spec.store.select.i.i3983, 0, !dbg !18569
  br i1 %_2.i6.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i3984, !dbg !18569

bb3.i3984:                                        ; preds = %bb13, %bb5.i
  %iter.sroa.8.07.i = phi i32 [ %1173, %bb5.i ], [ 0, %bb13 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i32 %iter.sroa.8.07.i, !dbg !18572
  %_14.i3985 = load i32, ptr %_3.i1.i.i, align 4, !dbg !18575, !noalias !18556, !noundef !10
  %_20.i = icmp eq i32 %_14.i3985, 0, !dbg !18576
  br i1 %_20.i, label %panic.i3992, label %bb5.i, !dbg !18576

bb5.i:                                            ; preds = %bb3.i3984
  %_3.i.i.i3986 = getelementptr inbounds nuw i32, ptr %_40.0.i, i32 %iter.sroa.8.07.i, !dbg !18577
  %1173 = add nuw i32 %iter.sroa.8.07.i, 1, !dbg !18580
  %_18.i3987 = load i32, ptr %_3.i.i.i3986, align 4, !dbg !18581, !noalias !18556, !noundef !10
  %_19.i3988 = urem i32 %frames, %_14.i3985, !dbg !18576
  %_16.i3989 = add i32 %_19.i3988, %_18.i3987, !dbg !18582
  %_15.i3990 = urem i32 %_16.i3989, %_14.i3985, !dbg !18583
  store i32 %_15.i3990, ptr %_3.i.i.i3986, align 4, !dbg !18584, !noalias !18556
  %exitcond.not.i = icmp eq i32 %1173, %spec.store.select.i.i3983, !dbg !18569
  br i1 %exitcond.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i3984, !dbg !18569

panic.i3992:                                      ; preds = %bb3.i3984
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #32, !dbg !18576, !noalias !18556
  unreachable, !dbg !18576

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb5.i, %bb13
  %1174 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !18585
  %_20.val = load i32, ptr %1174, align 4, !dbg !18585
  %1175 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !18585
  %_20.val3612 = load i32, ptr %1175, align 4, !dbg !18585, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18586), !dbg !18585
  %_10.i3993 = icmp eq i32 %_20.val3612, 0, !dbg !18589
  br i1 %_10.i3993, label %panic.i4005, label %bb1.i3994, !dbg !18589

bb1.i3994:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %_19 = getelementptr inbounds nuw i8, ptr %self, i32 800, !dbg !18591
  %_7.i3995 = load i32, ptr %_19, align 4, !dbg !18592, !alias.scope !18586, !noundef !10
  %_8.i3996 = urem i32 %frames, %_20.val3612, !dbg !18589
  %_5.i3997 = add i32 %_8.i3996, %_7.i3995, !dbg !18593
  %_4.i3998 = urem i32 %_5.i3997, %_20.val3612, !dbg !18594
  store i32 %_4.i3998, ptr %_19, align 4, !dbg !18595, !alias.scope !18586
  %_17.i3999 = icmp eq i32 %_20.val, 0, !dbg !18596
  br i1 %_17.i3999, label %panic2.i, label %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !18596

panic.i4005:                                      ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c7e64e9e8489bc8e648659d0098d136c) #32, !dbg !18589, !noalias !18586
  unreachable, !dbg !18589

panic2.i:                                         ; preds = %bb1.i3994
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_34b23597fad9d85cb3bee6d64ebf77d5) #32, !dbg !18596, !noalias !18586
  unreachable, !dbg !18596

_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i3994
  %1176 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !18597
  %_14.i4001 = load i32, ptr %1176, align 4, !dbg !18597, !alias.scope !18586, !noundef !10
  %_15.i4002 = urem i32 %frames, %_20.val, !dbg !18596
  %_12.i4003 = add i32 %_15.i4002, %_14.i4001, !dbg !18598
  %_11.i4004 = urem i32 %_12.i4003, %_20.val, !dbg !18599
  store i32 %_11.i4004, ptr %1176, align 4, !dbg !18600, !alias.scope !18586
  br label %bb32, !dbg !18601

bb18:                                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_27 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #31, !dbg !18602
  br i1 %_27, label %bb20, label %bb24, !dbg !18603

bb20:                                             ; preds = %bb18
  %_59.not = icmp ugt i32 %words, %left_io.1
  br i1 %_59.not, label %bb40, label %bb1.i4012, !dbg !18604, !prof !4596

bb24:                                             ; preds = %bb12.i4026, %bb1.i4012, %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %bb18
  %_26.sroa.0.0.off0 = phi i1 [ false, %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ false, %bb18 ], [ false, %bb12.i4026 ], [ true, %bb1.i4012 ]
  %1177 = zext i1 %_26.sroa.0.0.off0 to i8, !dbg !18612
  store i8 %1177, ptr %23, align 4, !dbg !18612
  %1178 = load i8, ptr %2, align 8, !dbg !18613, !range !4667, !noundef !10
  store i8 %1178, ptr %0, align 1, !dbg !18614
  %fst_len.i4007 = and i32 %left_io.1, 536870908, !dbg !18615
  %_22.not.i9931 = icmp eq i32 %fst_len.i4007, 0, !dbg !18619
  br i1 %_22.not.i9931, label %bb32, label %bb13.i1199, !dbg !18619

bb13.i1199:                                       ; preds = %bb24, %bb13.i1199
  %iter.sroa.0.0.i11989934 = phi ptr [ %_27.i1200, %bb13.i1199 ], [ %left_io.0, %bb24 ]
  %iter.sroa.5.0.i9933 = phi i32 [ %_28.i, %bb13.i1199 ], [ %fst_len.i4007, %bb24 ]
  %ok.i.sroa.0.09932 = phi <4 x i32> [ %1181, %bb13.i1199 ], [ splat (i32 -1), %bb24 ]
  %_27.i1200 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i11989934, i32 16, !dbg !18622
  %_28.i = add i32 %iter.sroa.5.0.i9933, -4, !dbg !18625
  %lanes.i.sroa.0.0.copyload = load <4 x float>, ptr %iter.sroa.0.0.i11989934, align 4, !dbg !18626, !alias.scope !18631, !noalias !18635
  %1179 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload), !dbg !18639
  %1180 = fcmp olt <4 x float> %1179, splat (float 0x46293E5940000000), !dbg !18643
  %1181 = select <4 x i1> %1180, <4 x i32> %ok.i.sroa.0.09932, <4 x i32> zeroinitializer, !dbg !18648
  %_22.not.i = icmp eq i32 %_28.i, 0, !dbg !18619
  br i1 %_22.not.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit, label %bb13.i1199, !dbg !18619

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb13.i1199
  %1182 = bitcast <4 x i32> %1181 to <16 x i8>, !dbg !18652
  %1183 = xor <16 x i8> %1182, splat (i8 -1), !dbg !18652
  %1184 = tail call i32 @llvm.wasm.anytrue.v16i8(<16 x i8> %1183), !dbg !18656
  %1185 = icmp eq i32 %1184, 0, !dbg !18656
  br i1 %1185, label %bb32, label %bb19.i4029, !dbg !18657

bb1.i4012:                                        ; preds = %bb20, %bb12.i4026
  %io.sroa.5.0.i4013 = phi i32 [ %len.i.i.i4019, %bb12.i4026 ], [ %words, %bb20 ]
  %io.sroa.0.0.i4014 = phi ptr [ %data.i.i.i4018, %bb12.i4026 ], [ %left_io.0, %bb20 ]
  %1186 = icmp eq i32 %io.sroa.5.0.i4013, 0, !dbg !18658
  br i1 %1186, label %bb24, label %bb13.preheader.i4015, !dbg !18658

bb13.preheader.i4015:                             ; preds = %bb1.i4012
  %spec.store.select.i4016 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i4013, i32 32), !dbg !18661
  %data.i.i.idx.i4017 = shl nuw nsw i32 %spec.store.select.i4016, 2, !dbg !18664
  %data.i.i.i4018 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i4014, i32 %data.i.i.idx.i4017, !dbg !18664
  br label %bb13.i4020, !dbg !18669

bb13.i4020:                                       ; preds = %bb13.i4020, %bb13.preheader.i4015
  %iter.sroa.0.08.i4021 = phi ptr [ %_35.i4023, %bb13.i4020 ], [ %io.sroa.0.0.i4014, %bb13.preheader.i4015 ]
  %bits.sroa.0.07.i4022 = phi i32 [ %1187, %bb13.i4020 ], [ 0, %bb13.preheader.i4015 ]
  %_35.i4023 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i4021, i32 4, !dbg !18671
  %_95.i4024 = load i32, ptr %iter.sroa.0.08.i4021, align 4, !dbg !18673, !alias.scope !18674, !noundef !10
  %1187 = or i32 %_95.i4024, %bits.sroa.0.07.i4022, !dbg !18677
  %_29.i4025 = icmp eq ptr %_35.i4023, %data.i.i.i4018, !dbg !18678
  br i1 %_29.i4025, label %bb12.i4026, label %bb13.i4020, !dbg !18669

bb12.i4026:                                       ; preds = %bb13.i4020
  %len.i.i.i4019 = sub nuw nsw i32 %io.sroa.5.0.i4013, %spec.store.select.i4016, !dbg !18680
  %1188 = icmp eq i32 %1187, 0, !dbg !18681
  br i1 %1188, label %bb1.i4012, label %bb24, !dbg !18681

bb40:                                             ; preds = %bb20
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_83306e04d21648aa3da9cd74a1a9d08b) #32, !dbg !18682
  unreachable, !dbg !18682

bb19.i4029:                                       ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit, %bb19.i4029
  %iter.sroa.0.089.i = phi ptr [ %_42.i4030, %bb19.i4029 ], [ %left_io.0, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit ]
  %iter.sroa.5.088.i = phi i32 [ %_43.i4031, %bb19.i4029 ], [ %fst_len.i4007, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit ]
  %ok.sroa.0.087.i = phi <4 x i32> [ %1191, %bb19.i4029 ], [ splat (i32 -1), %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit ]
  %_42.i4030 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i32 16, !dbg !18683
  %_43.i4031 = add i32 %iter.sroa.5.088.i, -4, !dbg !18689
  %lanes.i.sroa.0.0.copyload.i = load <4 x float>, ptr %iter.sroa.0.089.i, align 4, !dbg !18690, !alias.scope !18695, !noalias !18701
  %1189 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload.i), !dbg !18705
  %1190 = fcmp olt <4 x float> %1189, splat (float 0x46293E5940000000), !dbg !18709
  %1191 = select <4 x i1> %1190, <4 x i32> %ok.sroa.0.087.i, <4 x i32> zeroinitializer, !dbg !18714
  %_37.not.i = icmp eq i32 %_43.i4031, 0, !dbg !18718
  br i1 %_37.not.i, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit, label %bb19.i4029, !dbg !18718

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb19.i4029
  %1192 = bitcast <4 x i32> %1191 to <16 x i8>, !dbg !18719
  %_4.i39.i = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> zeroinitializer, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1192), !dbg !18723
  %1193 = bitcast <16 x i8> %_4.i39.i to <4 x i32>, !dbg !18724
  %words.sroa.0.0.vec.extract.i = extractelement <4 x i32> %1193, i64 0, !dbg !18729
  %1194 = icmp ne i32 %words.sroa.0.0.vec.extract.i, 0, !dbg !18729
  %1195 = zext i1 %1194 to i32, !dbg !18729
  %words.sroa.0.4.vec.extract.i = extractelement <4 x i32> %1193, i64 1, !dbg !18729
  %1196 = icmp eq i32 %words.sroa.0.4.vec.extract.i, 0, !dbg !18729
  %1197 = select i1 %1196, i32 0, i32 2, !dbg !18729
  %mask.sroa.0.1.1.i = or disjoint i32 %1197, %1195, !dbg !18729
  %words.sroa.0.8.vec.extract.i = extractelement <4 x i32> %1193, i64 2, !dbg !18729
  %1198 = icmp eq i32 %words.sroa.0.8.vec.extract.i, 0, !dbg !18729
  %1199 = select i1 %1198, i32 0, i32 4, !dbg !18729
  %mask.sroa.0.1.2.i = or disjoint i32 %mask.sroa.0.1.1.i, %1199, !dbg !18729
  %words.sroa.0.12.vec.extract.i = extractelement <4 x i32> %1193, i64 3, !dbg !18729
  %1200 = icmp eq i32 %words.sroa.0.12.vec.extract.i, 0, !dbg !18729
  %1201 = select i1 %1200, i32 0, i32 8, !dbg !18729
  %mask.sroa.0.1.3.i = or disjoint i32 %mask.sroa.0.1.2.i, %1201, !dbg !18729
  %1202 = getelementptr inbounds nuw i8, ptr %self, i32 8, !dbg !18730
  store i32 %mask.sroa.0.1.3.i, ptr %1202, align 8, !dbg !18730
  %_36 = load i64, ptr %self, align 16, !dbg !18731, !noundef !10
  %1203 = tail call i64 @llvm.uadd.sat.i64(i64 %_36, i64 1), !dbg !18732
  store i64 %1203, ptr %self, align 16, !dbg !18735
  %.idx.i = shl nuw nsw i32 %left_io.1, 2, !dbg !18736
  call void @llvm.memset.p0.i32(ptr nonnull align 4 %left_io.0, i8 0, i32 %.idx.i, i1 false), !dbg !18743, !alias.scope !18744
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !18747
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(12) %shape, ptr noundef nonnull align 16 dereferenceable(12) %_23, i32 12, i1 false), !dbg !18748
  %1204 = getelementptr inbounds nuw i8, ptr %self, i32 880, !dbg !18749
  %rate = load i32, ptr %1204, align 8, !dbg !18749, !noundef !10
  %1205 = getelementptr inbounds nuw i8, ptr %self, i32 808, !dbg !18751
  %_70.0 = load ptr, ptr %1205, align 8, !dbg !18751, !nonnull !10, !noundef !10
  %1206 = getelementptr inbounds nuw i8, ptr %self, i32 812, !dbg !18751
  %_70.1 = load i32, ptr %1206, align 4, !dbg !18751, !noundef !10
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_24, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_70.0, i32 noundef %_70.1, i32 noundef %rate) #31, !dbg !18753
  %_44 = getelementptr inbounds nuw i8, ptr %self, i32 1024, !dbg !18754
  %1207 = getelementptr inbounds nuw i8, ptr %self, i32 816, !dbg !18755
  %_71.0 = load ptr, ptr %1207, align 16, !dbg !18755, !nonnull !10, !noundef !10
  %1208 = getelementptr inbounds nuw i8, ptr %self, i32 820, !dbg !18755
  %_71.1 = load i32, ptr %1208, align 4, !dbg !18755, !noundef !10
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_44, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_71.0, i32 noundef %_71.1, i32 noundef %rate) #31, !dbg !18756
  store i32 0, ptr %_25, align 16, !dbg !18757
  %1209 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !18757
  store i32 0, ptr %1209, align 4, !dbg !18757
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !18758
  br label %bb32, !dbg !18759

bb32:                                             ; preds = %bb24, %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit
  ret void, !dbg !18759
}
