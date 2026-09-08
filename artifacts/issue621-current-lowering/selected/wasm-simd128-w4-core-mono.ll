define internal fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_block_monoB5_(ptr noalias noundef nonnull align 16 dereferenceable(1136) %self, ptr noalias noundef nonnull align 4 captures(address) %left_io.0, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef %frames) unnamed_addr #1 !dbg !14005 {
start:
  %_71.i711 = alloca [368 x i8], align 16
  %peaks_left.i713 = alloca [1024 x i8], align 4
  %scratch.i714 = alloca [32 x i8], align 4
  %hot_left.i716 = alloca [368 x i8], align 16
  %peaks_left.i434 = alloca [1024 x i8], align 4
  %scratch.i = alloca [32 x i8], align 4
  %hot_left.i436 = alloca [368 x i8], align 16
  %uniform_left.i33 = alloca [64 x i8], align 16
  %peaks_left.i34 = alloca [1024 x i8], align 4
  %hot_left.i36 = alloca [368 x i8], align 16
  %_101.i = alloca [368 x i8], align 16
  %uniform_left.i = alloca [64 x i8], align 16
  %peaks_left.i = alloca [1024 x i8], align 4
  %hot_left.i = alloca [368 x i8], align 16
  %shape = alloca [12 x i8], align 4
  %words = shl i32 %frames, 2, !dbg !14006
  %0 = getelementptr inbounds nuw i8, ptr %self, i32 1125, !dbg !14007
  %1 = load i8, ptr %0, align 1, !dbg !14007, !range !4765, !noundef !10
  %2 = getelementptr inbounds nuw i8, ptr %self, i32 904, !dbg !14009
  %3 = load i8, ptr %2, align 8, !dbg !14009, !range !4765, !noundef !10
  %_6 = icmp eq i8 %1, %3, !dbg !14007
  %4 = getelementptr inbounds nuw i8, ptr %self, i32 988
  %_68.0 = load ptr, ptr %4, align 4, !dbg !14010
  %5 = getelementptr inbounds nuw i8, ptr %self, i32 992
  %_68.1 = load i32, ptr %5, align 4, !dbg !14010
  br i1 %_6, label %bb1, label %bb11.thread, !dbg !14007

bb1:                                              ; preds = %start
  %_8.i3599 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_68.0, i32 %_68.1, !dbg !14011
  br label %bb1.i.i3600, !dbg !14016

bb1.i.i3600:                                      ; preds = %bb11.i.i3602, %bb1
  %_221.i.i = phi ptr [ %_22.i.i3603, %bb11.i.i3602 ], [ %_68.0, %bb1 ]
  %_12.i.i3601 = icmp eq ptr %_221.i.i, %_8.i3599, !dbg !14018
  br i1 %_12.i.i3601, label %bb3, label %bb11.i.i3602, !dbg !14021

bb11.i.i3602:                                     ; preds = %bb1.i.i3600
  %_22.i.i3603 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 16, !dbg !14022
  %6 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 12, !dbg !14024
  %_3.i.i.i = load i32, ptr %6, align 4, !dbg !14024, !alias.scope !14026, !noalias !14031, !noundef !10
  %7 = icmp eq i32 %_3.i.i.i, 0, !dbg !14024
  %_51.i.i.i = load i32, ptr %_221.i.i, align 4, !dbg !14024, !alias.scope !14026, !noalias !14031
  %8 = getelementptr inbounds nuw i8, ptr %_221.i.i, i32 4, !dbg !14024
  %_72.i.i.i = load i32, ptr %8, align 4, !dbg !14024, !alias.scope !14026, !noalias !14031
  %9 = icmp eq i32 %_51.i.i.i, %_72.i.i.i, !dbg !14024
  %_0.sroa.0.0.off0.i.i.i = select i1 %7, i1 %9, i1 false, !dbg !14024
  br i1 %_0.sroa.0.0.off0.i.i.i, label %bb1.i.i3600, label %bb11.thread, !dbg !14034

bb3:                                              ; preds = %bb1.i.i3600
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 996, !dbg !14035
  %_69.0 = load ptr, ptr %10, align 4, !dbg !14035, !nonnull !10, !noundef !10
  %11 = getelementptr inbounds nuw i8, ptr %self, i32 1000, !dbg !14035
  %_69.1 = load i32, ptr %11, align 4, !dbg !14035, !noundef !10
  %_8.i3604 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_69.0, i32 %_69.1, !dbg !14036
  br label %bb1.i.i3605, !dbg !14041

bb1.i.i3605:                                      ; preds = %bb11.i.i3608, %bb3
  %_221.i.i3606 = phi ptr [ %_22.i.i3609, %bb11.i.i3608 ], [ %_69.0, %bb3 ]
  %_12.i.i3607 = icmp eq ptr %_221.i.i3606, %_8.i3604, !dbg !14043
  br i1 %_12.i.i3607, label %bb5, label %bb11.i.i3608, !dbg !14046

bb11.i.i3608:                                     ; preds = %bb1.i.i3605
  %_22.i.i3609 = getelementptr inbounds nuw i8, ptr %_221.i.i3606, i32 16, !dbg !14047
  %12 = getelementptr inbounds nuw i8, ptr %_221.i.i3606, i32 12, !dbg !14049
  %_3.i.i.i3610 = load i32, ptr %12, align 4, !dbg !14049, !alias.scope !14051, !noalias !14056, !noundef !10
  %13 = icmp eq i32 %_3.i.i.i3610, 0, !dbg !14049
  %_51.i.i.i3611 = load i32, ptr %_221.i.i3606, align 4, !dbg !14049, !alias.scope !14051, !noalias !14056
  %14 = getelementptr inbounds nuw i8, ptr %_221.i.i3606, i32 4, !dbg !14049
  %_72.i.i.i3612 = load i32, ptr %14, align 4, !dbg !14049, !alias.scope !14051, !noalias !14056
  %15 = icmp eq i32 %_51.i.i.i3611, %_72.i.i.i3612, !dbg !14049
  %_0.sroa.0.0.off0.i.i.i3613 = select i1 %13, i1 %15, i1 false, !dbg !14049
  br i1 %_0.sroa.0.0.off0.i.i.i3613, label %bb1.i.i3605, label %bb11.thread, !dbg !14059

bb5:                                              ; preds = %bb1.i.i3605
  %_51.not = icmp ugt i32 %words, %left_io.1
  br i1 %_51.not, label %bb35, label %bb1.i3615, !dbg !14060, !prof !4694

bb11.thread:                                      ; preds = %bb11.i.i3602, %bb11.i.i3608, %start
  %16 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  br label %bb16, !dbg !14069

bb11:                                             ; preds = %bb1.i3615
  %17 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  %18 = load i8, ptr %17, align 4, !range !4765
  %_15 = trunc nuw i8 %18 to i1
  br i1 %_15, label %bb13, label %bb16, !dbg !14069

bb1.i3615:                                        ; preds = %bb5, %bb12.i3619
  %io.sroa.5.0.i = phi i32 [ %len.i.i.i, %bb12.i3619 ], [ %words, %bb5 ]
  %io.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb12.i3619 ], [ %left_io.0, %bb5 ]
  %19 = icmp eq i32 %io.sroa.5.0.i, 0, !dbg !14071
  br i1 %19, label %bb11, label %bb13.preheader.i, !dbg !14071

bb13.preheader.i:                                 ; preds = %bb1.i3615
  %spec.store.select.i3616 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i, i32 32), !dbg !14074
  %data.i.i.idx.i = shl nuw nsw i32 %spec.store.select.i3616, 2, !dbg !14077
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i, i32 %data.i.i.idx.i, !dbg !14077
  br label %bb13.i3617, !dbg !14082

bb13.i3617:                                       ; preds = %bb13.i3617, %bb13.preheader.i
  %iter.sroa.0.08.i = phi ptr [ %_35.i3618, %bb13.i3617 ], [ %io.sroa.0.0.i, %bb13.preheader.i ]
  %bits.sroa.0.07.i = phi i32 [ %20, %bb13.i3617 ], [ 0, %bb13.preheader.i ]
  %_35.i3618 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i, i32 4, !dbg !14084
  %_95.i = load i32, ptr %iter.sroa.0.08.i, align 4, !dbg !14086, !alias.scope !14087, !noundef !10
  %20 = or i32 %_95.i, %bits.sroa.0.07.i, !dbg !14090
  %_29.i = icmp eq ptr %_35.i3618, %data.i.i.i, !dbg !14091
  br i1 %_29.i, label %bb12.i3619, label %bb13.i3617, !dbg !14082

bb12.i3619:                                       ; preds = %bb13.i3617
  %len.i.i.i = sub nuw nsw i32 %io.sroa.5.0.i, %spec.store.select.i3616, !dbg !14093
  %21 = icmp eq i32 %20, 0, !dbg !14094
  br i1 %21, label %bb1.i3615, label %bb11.thread7996, !dbg !14094

bb11.thread7996:                                  ; preds = %bb12.i3619
  %22 = getelementptr inbounds nuw i8, ptr %self, i32 1124
  br label %bb16, !dbg !14069

bb35:                                             ; preds = %bb5
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_81e9b689ac7837328a8c5e199528fa13) #33, !dbg !14095
  unreachable, !dbg !14095

bb16:                                             ; preds = %bb11.thread7996, %bb11.thread, %bb11
  %23 = phi ptr [ %16, %bb11.thread ], [ %17, %bb11 ], [ %22, %bb11.thread7996 ]
  %quiet.sroa.0.0.off07995 = phi i1 [ false, %bb11.thread ], [ true, %bb11 ], [ false, %bb11.thread7996 ]
  %_22 = getelementptr inbounds nuw i8, ptr %self, i32 16, !dbg !14096
  %_23 = getelementptr inbounds nuw i8, ptr %self, i32 912, !dbg !14097
  %_24 = getelementptr inbounds nuw i8, ptr %self, i32 924, !dbg !14098
  %_25 = getelementptr inbounds nuw i8, ptr %self, i32 800, !dbg !14099
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14100), !dbg !14103
  %_8.i3620 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_68.0, i32 %_68.1, !dbg !14104
  br label %bb1.i.i3621, !dbg !14113

bb1.i.i3621:                                      ; preds = %bb11.i.i3624, %bb16
  %_221.i.i3622 = phi ptr [ %_22.i.i3625, %bb11.i.i3624 ], [ %_68.0, %bb16 ]
  %_12.i.i3623 = icmp eq ptr %_221.i.i3622, %_8.i3620, !dbg !14115
  br i1 %_12.i.i3623, label %bb12.i, label %bb11.i.i3624, !dbg !14118

bb11.i.i3624:                                     ; preds = %bb1.i.i3621
  %_22.i.i3625 = getelementptr inbounds nuw i8, ptr %_221.i.i3622, i32 16, !dbg !14119
  %24 = getelementptr inbounds nuw i8, ptr %_221.i.i3622, i32 12, !dbg !14121
  %_3.i.i.i3626 = load i32, ptr %24, align 4, !dbg !14121, !alias.scope !14123, !noalias !14128, !noundef !10
  %25 = icmp eq i32 %_3.i.i.i3626, 0, !dbg !14121
  %_51.i.i.i3627 = load i32, ptr %_221.i.i3622, align 4, !dbg !14121, !alias.scope !14123, !noalias !14128
  %26 = getelementptr inbounds nuw i8, ptr %_221.i.i3622, i32 4, !dbg !14121
  %_72.i.i.i3628 = load i32, ptr %26, align 4, !dbg !14121, !alias.scope !14123, !noalias !14128
  %27 = icmp eq i32 %_51.i.i.i3627, %_72.i.i.i3628, !dbg !14121
  %_0.sroa.0.0.off0.i.i.i3629 = select i1 %25, i1 %27, i1 false, !dbg !14121
  br i1 %_0.sroa.0.0.off0.i.i.i3629, label %bb1.i.i3621, label %bb13.i, !dbg !14135

bb13.i:                                           ; preds = %bb11.i.i3624
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14136), !dbg !14139
  %28 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !14141
  %_31.0.i = load ptr, ptr %28, align 4, !dbg !14141, !alias.scope !14136, !noalias !14143, !nonnull !10, !noundef !10
  %29 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !14141
  %_31.1.i = load i32, ptr %29, align 4, !dbg !14141, !alias.scope !14136, !noalias !14143, !noundef !10
  %_17.idx.i = mul nuw nsw i32 %_31.1.i, 12, !dbg !14144
  %_17.i = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 %_17.idx.i, !dbg !14144
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14148), !dbg !14151, !noalias !14143
  %_5.not.i.i.i = icmp eq i32 %_31.1.i, 0
  %30 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 4
  %31 = getelementptr inbounds nuw i8, ptr %_31.0.i, i32 8
  br i1 %_5.not.i.i.i, label %bb2.i3640, label %bb1.i.i3631

bb1.i.i3631:                                      ; preds = %bb13.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i
  %_224.i.i = phi ptr [ %_22.i.i3634, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i ], [ %_31.0.i, %bb13.i ]
  %_12.i.i3632 = icmp eq ptr %_224.i.i, %_17.i, !dbg !14152
  br i1 %_12.i.i3632, label %bb2.i3640, label %bb11.i.i3633, !dbg !14156

bb11.i.i3633:                                     ; preds = %bb1.i.i3631
  %_22.i.i3634 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 12, !dbg !14157
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14159), !dbg !14162, !noalias !14143
  %_9.i.i.i3635 = load i32, ptr %_224.i.i, align 4, !dbg !14163, !alias.scope !14159, !noalias !14166, !noundef !10
  %_10.i.i.i = load i32, ptr %_31.0.i, align 4, !dbg !14163, !alias.scope !14148, !noalias !14168, !noundef !10
  %_8.i.i.i = icmp eq i32 %_9.i.i.i3635, %_10.i.i.i, !dbg !14163
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb8.i, !dbg !14163

bb2.i.i.i:                                        ; preds = %bb11.i.i3633
  %32 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 4, !dbg !14163
  %_12.i.i.i3636 = load i32, ptr %32, align 4, !dbg !14163, !alias.scope !14159, !noalias !14166, !noundef !10
  %_13.i.i.i3637 = load i32, ptr %30, align 4, !dbg !14163, !alias.scope !14148, !noalias !14168, !noundef !10
  %_11.i.i.i = icmp eq i32 %_12.i.i.i3636, %_13.i.i.i3637, !dbg !14163
  br i1 %_11.i.i.i, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, label %bb8.i, !dbg !14163

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i: ; preds = %bb2.i.i.i
  %33 = getelementptr inbounds nuw i8, ptr %_224.i.i, i32 8, !dbg !14163
  %_14.i.i.i3638 = load i32, ptr %33, align 4, !dbg !14163, !alias.scope !14159, !noalias !14166, !noundef !10
  %_15.i.i.i3639 = load i32, ptr %31, align 4, !dbg !14163, !alias.scope !14148, !noalias !14168, !noundef !10
  %34 = icmp eq i32 %_14.i.i.i3638, %_15.i.i.i3639, !dbg !14163
  br i1 %34, label %bb1.i.i3631, label %bb8.i, !dbg !14162

bb2.i3640:                                        ; preds = %bb1.i.i3631, %bb13.i
  %35 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !14169
  %_32.0.i = load ptr, ptr %35, align 4, !dbg !14169, !alias.scope !14136, !noalias !14143, !nonnull !10, !noundef !10
  %36 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !14169
  %_32.1.i = load i32, ptr %36, align 4, !dbg !14169, !alias.scope !14136, !noalias !14143, !noundef !10
  %_26.idx.i = shl nuw nsw i32 %_32.1.i, 2, !dbg !14170
  %_26.i3641 = getelementptr inbounds nuw i8, ptr %_32.0.i, i32 %_26.idx.i, !dbg !14170
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14174), !dbg !14177, !noalias !14143
  %_6.not.i.i.i = icmp eq i32 %_32.1.i, 0
  br i1 %_6.not.i.i.i, label %bb4.i, label %bb1.i3.i

bb1.i3.i:                                         ; preds = %bb2.i3640, %bb11.i5.i
  %_223.i.i = phi ptr [ %_22.i6.i, %bb11.i5.i ], [ %_32.0.i, %bb2.i3640 ]
  %_12.i4.i = icmp eq ptr %_223.i.i, %_26.i3641, !dbg !14178
  br i1 %_12.i4.i, label %bb4.i, label %bb11.i5.i, !dbg !14182

bb11.i5.i:                                        ; preds = %bb1.i3.i
  %_22.i6.i = getelementptr inbounds nuw i8, ptr %_223.i.i, i32 4, !dbg !14183
  %ptr.val.i.i = load i32, ptr %_223.i.i, align 4, !dbg !14185, !noalias !14186
  %_4.i.i.i3642 = load i32, ptr %_32.0.i, align 4, !dbg !14188, !alias.scope !14174, !noalias !14190, !noundef !10
  %_0.i.i.i = icmp eq i32 %ptr.val.i.i, %_4.i.i.i3642, !dbg !14191
  br i1 %_0.i.i.i, label %bb1.i3.i, label %bb8.i, !dbg !14185

bb12.i:                                           ; preds = %bb1.i.i3621
  %37 = getelementptr inbounds nuw i8, ptr %self, i32 996, !dbg !14192
  %_20.0.i = load ptr, ptr %37, align 4, !dbg !14192, !alias.scope !14100, !noalias !14143, !nonnull !10, !noundef !10
  %38 = getelementptr inbounds nuw i8, ptr %self, i32 1000, !dbg !14192
  %_20.1.i = load i32, ptr %38, align 4, !dbg !14192, !alias.scope !14100, !noalias !14143, !noundef !10
  %_8.i3643 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_20.0.i, i32 %_20.1.i, !dbg !14193
  br label %bb1.i.i3644, !dbg !14198

bb1.i.i3644:                                      ; preds = %bb11.i.i3647, %bb12.i
  %_221.i.i3645 = phi ptr [ %_22.i.i3648, %bb11.i.i3647 ], [ %_20.0.i, %bb12.i ]
  %_12.i.i3646 = icmp eq ptr %_221.i.i3645, %_8.i3643, !dbg !14200
  br i1 %_12.i.i3646, label %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3653, label %bb11.i.i3647, !dbg !14203

bb11.i.i3647:                                     ; preds = %bb1.i.i3644
  %_22.i.i3648 = getelementptr inbounds nuw i8, ptr %_221.i.i3645, i32 16, !dbg !14204
  %39 = getelementptr inbounds nuw i8, ptr %_221.i.i3645, i32 12, !dbg !14206
  %_3.i.i.i3649 = load i32, ptr %39, align 4, !dbg !14206, !alias.scope !14208, !noalias !14213, !noundef !10
  %40 = icmp eq i32 %_3.i.i.i3649, 0, !dbg !14206
  %_51.i.i.i3650 = load i32, ptr %_221.i.i3645, align 4, !dbg !14206, !alias.scope !14208, !noalias !14213
  %41 = getelementptr inbounds nuw i8, ptr %_221.i.i3645, i32 4, !dbg !14206
  %_72.i.i.i3651 = load i32, ptr %41, align 4, !dbg !14206, !alias.scope !14208, !noalias !14213
  %42 = icmp eq i32 %_51.i.i.i3650, %_72.i.i.i3651, !dbg !14206
  %_0.sroa.0.0.off0.i.i.i3652 = select i1 %40, i1 %42, i1 false, !dbg !14206
  br i1 %_0.sroa.0.0.off0.i.i.i3652, label %bb1.i.i3644, label %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3653, !dbg !14216

_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3653: ; preds = %bb1.i.i3644, %bb11.i.i3647
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14217), !dbg !14139
  %43 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !14220
  %_31.0.i3654 = load ptr, ptr %43, align 4, !dbg !14220, !alias.scope !14217, !noalias !14143, !nonnull !10, !noundef !10
  %44 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !14220
  %_31.1.i3655 = load i32, ptr %44, align 4, !dbg !14220, !alias.scope !14217, !noalias !14143, !noundef !10
  %_17.idx.i3656 = mul nuw nsw i32 %_31.1.i3655, 12, !dbg !14222
  %_17.i3657 = getelementptr inbounds nuw i8, ptr %_31.0.i3654, i32 %_17.idx.i3656, !dbg !14222
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14226), !dbg !14229, !noalias !14143
  %_5.not.i.i.i3658 = icmp eq i32 %_31.1.i3655, 0
  %45 = getelementptr inbounds nuw i8, ptr %_31.0.i3654, i32 4
  %46 = getelementptr inbounds nuw i8, ptr %_31.0.i3654, i32 8
  br i1 %_5.not.i.i.i3658, label %bb2.i3675, label %bb1.i.i3659

bb1.i.i3659:                                      ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3653, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3672
  %_224.i.i3660 = phi ptr [ %_22.i.i3663, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3672 ], [ %_31.0.i3654, %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3653 ]
  %_12.i.i3661 = icmp eq ptr %_224.i.i3660, %_17.i3657, !dbg !14230
  br i1 %_12.i.i3661, label %bb2.i3675, label %bb11.i.i3662, !dbg !14234

bb11.i.i3662:                                     ; preds = %bb1.i.i3659
  %_22.i.i3663 = getelementptr inbounds nuw i8, ptr %_224.i.i3660, i32 12, !dbg !14235
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14237), !dbg !14240, !noalias !14143
  %_9.i.i.i3664 = load i32, ptr %_224.i.i3660, align 4, !dbg !14241, !alias.scope !14237, !noalias !14244, !noundef !10
  %_10.i.i.i3665 = load i32, ptr %_31.0.i3654, align 4, !dbg !14241, !alias.scope !14226, !noalias !14246, !noundef !10
  %_8.i.i.i3666 = icmp eq i32 %_9.i.i.i3664, %_10.i.i.i3665, !dbg !14241
  br i1 %_8.i.i.i3666, label %bb2.i.i.i3668, label %bb6.i, !dbg !14241

bb2.i.i.i3668:                                    ; preds = %bb11.i.i3662
  %47 = getelementptr inbounds nuw i8, ptr %_224.i.i3660, i32 4, !dbg !14241
  %_12.i.i.i3669 = load i32, ptr %47, align 4, !dbg !14241, !alias.scope !14237, !noalias !14244, !noundef !10
  %_13.i.i.i3670 = load i32, ptr %45, align 4, !dbg !14241, !alias.scope !14226, !noalias !14246, !noundef !10
  %_11.i.i.i3671 = icmp eq i32 %_12.i.i.i3669, %_13.i.i.i3670, !dbg !14241
  br i1 %_11.i.i.i3671, label %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3672, label %bb6.i, !dbg !14241

_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3672: ; preds = %bb2.i.i.i3668
  %48 = getelementptr inbounds nuw i8, ptr %_224.i.i3660, i32 8, !dbg !14241
  %_14.i.i.i3673 = load i32, ptr %48, align 4, !dbg !14241, !alias.scope !14237, !noalias !14244, !noundef !10
  %_15.i.i.i3674 = load i32, ptr %46, align 4, !dbg !14241, !alias.scope !14226, !noalias !14246, !noundef !10
  %49 = icmp eq i32 %_14.i.i.i3673, %_15.i.i.i3674, !dbg !14241
  br i1 %49, label %bb1.i.i3659, label %bb6.i, !dbg !14240

bb2.i3675:                                        ; preds = %bb1.i.i3659, %_RNvCsjLJhryqjeDL_17true_peak_limiter20ramps_are_stationary.exit3653
  %50 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !14247
  %_32.0.i3676 = load ptr, ptr %50, align 4, !dbg !14247, !alias.scope !14217, !noalias !14143, !nonnull !10, !noundef !10
  %51 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !14247
  %_32.1.i3677 = load i32, ptr %51, align 4, !dbg !14247, !alias.scope !14217, !noalias !14143, !noundef !10
  %_26.idx.i3678 = shl nuw nsw i32 %_32.1.i3677, 2, !dbg !14248
  %_26.i3679 = getelementptr inbounds nuw i8, ptr %_32.0.i3676, i32 %_26.idx.i3678, !dbg !14248
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14252), !dbg !14255, !noalias !14143
  %_6.not.i.i.i3680 = icmp eq i32 %_32.1.i3677, 0
  br i1 %_6.not.i.i.i3680, label %bb2.i, label %bb1.i3.i3681

bb1.i3.i3681:                                     ; preds = %bb2.i3675, %bb11.i5.i3684
  %_223.i.i3682 = phi ptr [ %_22.i6.i3685, %bb11.i5.i3684 ], [ %_32.0.i3676, %bb2.i3675 ]
  %_12.i4.i3683 = icmp eq ptr %_223.i.i3682, %_26.i3679, !dbg !14256
  br i1 %_12.i4.i3683, label %bb2.i, label %bb11.i5.i3684, !dbg !14260

bb11.i5.i3684:                                    ; preds = %bb1.i3.i3681
  %_22.i6.i3685 = getelementptr inbounds nuw i8, ptr %_223.i.i3682, i32 4, !dbg !14261
  %ptr.val.i.i3686 = load i32, ptr %_223.i.i3682, align 4, !dbg !14263, !noalias !14264
  %_4.i.i.i3687 = load i32, ptr %_32.0.i3676, align 4, !dbg !14266, !alias.scope !14252, !noalias !14268, !noundef !10
  %_0.i.i.i3688 = icmp eq i32 %ptr.val.i.i3686, %_4.i.i.i3687, !dbg !14269
  br i1 %_0.i.i.i3688, label %bb1.i3.i3681, label %bb6.i, !dbg !14263

bb8.i:                                            ; preds = %bb11.i.i3633, %bb2.i.i.i, %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i, %bb11.i5.i, %bb6.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14270), !dbg !14273
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14274), !dbg !14273
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14276), !dbg !14273
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i716), !dbg !14278, !noalias !14282
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i716, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #32, !dbg !14285, !noalias !14286
  %52 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !14287
  %53 = load i8, ptr %52, align 16, !dbg !14287, !range !4765, !alias.scope !14270, !noalias !14291, !noundef !10
  %54 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !14292
  %55 = load i8, ptr %54, align 1, !dbg !14292, !range !4765, !alias.scope !14270, !noalias !14291, !noundef !10
  %_25.i724 = load i32, ptr %_25, align 4, !dbg !14294, !alias.scope !14276, !noalias !14296, !noundef !10
  %56 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !14297
  %_27.i725 = load i32, ptr %56, align 4, !dbg !14297, !alias.scope !14276, !noalias !14296, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i714), !dbg !14299, !noalias !14282
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i714, i8 0, i32 32, i1 false), !noalias !14282
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i713), !dbg !14301, !noalias !14282
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i713, i8 0, i32 1024, i1 false), !noalias !14282
  %_74.not.i7369331 = icmp eq i32 %frames, 0, !dbg !14303
  br i1 %_74.not.i7369331, label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb29.i737.lr.ph, !dbg !14303

bb29.i737.lr.ph:                                  ; preds = %bb8.i
  %_23.i722 = trunc nuw i8 %55 to i1, !dbg !14292
  %spec.store.select19.i723 = select i1 %_23.i722, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !14292
  %_22.i720 = trunc nuw i8 %53 to i1, !dbg !14287
  %link.sroa.0.0.i721 = select i1 %_22.i720, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !14287
  %d9.i = lshr i32 %frames, 5, !dbg !14313
  %r2.i = and i32 %frames, 31, !dbg !14320
  %_19.not.i = icmp ne i32 %r2.i, 0, !dbg !14321
  %57 = zext i1 %_19.not.i to i32, !dbg !14321
  %yield_count.sroa.0.0.i = add nuw nsw i32 %d9.i, %57, !dbg !14321
  %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 16
  %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 32
  %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 48
  %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 64
  %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 80
  %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 96
  %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 112
  %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 128
  %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 144
  %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 160
  %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 176
  %58 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %59 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %60 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i.i904 = getelementptr inbounds nuw i8, ptr %self, i32 80
  %61 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %62 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %63 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i.i918 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %64 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %65 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %66 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i.i932 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %67 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %68 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %69 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i.i946 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %70 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %71 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %72 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i.i960 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %73 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %74 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %75 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i.i974 = getelementptr inbounds nuw i8, ptr %self, i32 400
  %76 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %77 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %78 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i.i988 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %79 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %80 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %81 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i.i1002 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %82 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %83 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %84 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i.i1016 = getelementptr inbounds nuw i8, ptr %self, i32 592
  %85 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %86 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %87 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i.i1030 = getelementptr inbounds nuw i8, ptr %self, i32 656
  %88 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %89 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %90 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i.i1044 = getelementptr inbounds nuw i8, ptr %self, i32 720
  %91 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %92 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %93 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %_47.i751 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 192
  %_48.i752 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 256
  %94 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 240
  %95 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 224
  %96 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 208
  %97 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 304
  %98 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 288
  %99 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 272
  %100 = bitcast <4 x i32> %link.sroa.0.0.i721 to <16 x i8>
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
  %113 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 336
  %114 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 352
  %115 = getelementptr inbounds nuw i8, ptr %hot_left.i716, i32 320
  %116 = getelementptr inbounds nuw i8, ptr %self, i32 936
  %117 = getelementptr inbounds nuw i8, ptr %self, i32 932
  %118 = bitcast <4 x i32> %spec.store.select19.i723 to <16 x i8>
  %119 = getelementptr inbounds nuw i8, ptr %self, i32 920
  %_13.i13918030 = load <16 x i8>, ptr %96, align 16
  %_13.i13788034 = load <16 x i8>, ptr %99, align 16
  %_62.i.i8288041 = load <4 x float>, ptr %114, align 16
  %iter.sroa.0.0.ptr.i.i7949205.1 = getelementptr inbounds nuw i8, ptr %scratch.i714, i32 4
  %iter.sroa.0.0.ptr.i.i7949205.2 = getelementptr inbounds nuw i8, ptr %scratch.i714, i32 8
  %iter.sroa.0.0.ptr.i.i7949205.3 = getelementptr inbounds nuw i8, ptr %scratch.i714, i32 12
  %iter.sroa.0.0.ptr.i.i7949205.4 = getelementptr inbounds nuw i8, ptr %scratch.i714, i32 16
  %iter.sroa.0.0.ptr.i.i7949205.5 = getelementptr inbounds nuw i8, ptr %scratch.i714, i32 20
  %iter.sroa.0.0.ptr.i.i7949205.6 = getelementptr inbounds nuw i8, ptr %scratch.i714, i32 24
  %iter.sroa.0.0.ptr.i.i7949205.7 = getelementptr inbounds nuw i8, ptr %scratch.i714, i32 28
  %hot_left.i716.promoted = load <4 x i32>, ptr %hot_left.i716, align 16, !noalias !14322
  %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !noalias !14322
  %.promoted12098 = load <4 x float>, ptr %94, align 16
  %.promoted = load <4 x float>, ptr %97, align 16
  %.promoted12139 = load <4 x float>, ptr %113, align 16
  br label %bb29.i737, !dbg !14303

bb12.i731.loopexit:                               ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742
  %.lcssa1186012140 = phi <4 x float> [ %.lcssa1186012141, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742 ], [ %299, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %.lcssa1175412119 = phi <4 x float> [ %.lcssa1175412120, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742 ], [ %260, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %.lcssa1177212099 = phi <4 x float> [ %.lcssa1177212100, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742 ], [ %257, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %ring_cursor.sroa.0.1.i745.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i7349334, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742 ], [ %spec.store.select8.i865, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ], !dbg !14327
  %main_cursor.sroa.0.1.i746.lcssa = phi i32 [ %main_cursor.sroa.0.0.i7359335, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742 ], [ %spec.store.select7.i863, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ], !dbg !14328
  %_74.not.i736 = icmp eq i32 %122, 0, !dbg !14303
  %indvars.iv.next = add i32 %indvars.iv, -32, !dbg !14303
  br i1 %_74.not.i736, label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, label %bb29.i737, !dbg !14303

bb29.i737:                                        ; preds = %bb29.i737.lr.ph, %bb12.i731.loopexit
  %.lcssa1186012141 = phi <4 x float> [ %.promoted12139, %bb29.i737.lr.ph ], [ %.lcssa1186012140, %bb12.i731.loopexit ]
  %.lcssa1175412120 = phi <4 x float> [ %.promoted, %bb29.i737.lr.ph ], [ %.lcssa1175412119, %bb12.i731.loopexit ]
  %.lcssa1177212100 = phi <4 x float> [ %.promoted12098, %bb29.i737.lr.ph ], [ %.lcssa1177212099, %bb12.i731.loopexit ]
  %history.i.i710.sroa.38.0.lcssa12079 = phi <4 x i32> [ %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.38.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.35.0.lcssa12060 = phi <4 x i32> [ %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.35.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.32.0.lcssa12041 = phi <4 x i32> [ %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.32.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.29.0.lcssa12022 = phi <4 x i32> [ %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.29.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.26.0.lcssa12003 = phi <4 x i32> [ %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.26.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.22.0.lcssa11984 = phi <4 x i32> [ %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.22.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.19.0.lcssa11965 = phi <4 x i32> [ %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.19.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.16.0.lcssa11946 = phi <4 x i32> [ %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.16.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.13.0.lcssa11927 = phi <4 x i32> [ %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.13.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.10.0.lcssa11908 = phi <4 x i32> [ %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.10.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.7.0.lcssa11889 = phi <4 x i32> [ %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.7.0.lcssa, %bb12.i731.loopexit ]
  %history.i.i710.sroa.0.0.lcssa11870 = phi <4 x i32> [ %hot_left.i716.promoted, %bb29.i737.lr.ph ], [ %history.i.i710.sroa.0.0.lcssa, %bb12.i731.loopexit ]
  %indvars.iv = phi i32 [ %frames, %bb29.i737.lr.ph ], [ %indvars.iv.next, %bb12.i731.loopexit ]
  %main_cursor.sroa.0.0.i7359335 = phi i32 [ %_25.i724, %bb29.i737.lr.ph ], [ %main_cursor.sroa.0.1.i746.lcssa, %bb12.i731.loopexit ]
  %ring_cursor.sroa.0.0.i7349334 = phi i32 [ %_27.i725, %bb29.i737.lr.ph ], [ %ring_cursor.sroa.0.1.i745.lcssa, %bb12.i731.loopexit ]
  %iter2.sroa.0.0.i7339333 = phi i32 [ %yield_count.sroa.0.0.i, %bb29.i737.lr.ph ], [ %122, %bb12.i731.loopexit ]
  %iter.sroa.0.0.i7329332 = phi i32 [ 0, %bb29.i737.lr.ph ], [ %121, %bb12.i731.loopexit ]
  %120 = call i32 @llvm.umax.i32(i32 %indvars.iv, i32 1), !dbg !14329
  %umax11132 = call i32 @llvm.umin.i32(i32 %120, i32 32), !dbg !14329
  %121 = add i32 %iter.sroa.0.0.i7329332, 32, !dbg !14329
  %122 = add nsw i32 %iter2.sroa.0.0.i7339333, -1, !dbg !14333
  %_20.i.i7419170.not = icmp eq i32 %frames, %iter.sroa.0.0.i7329332, !dbg !14334
  br i1 %_20.i.i7419170.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742, label %bb5.i.i873.lr.ph, !dbg !14341

bb5.i.i873.lr.ph:                                 ; preds = %bb29.i737
  %_11.i.i.i.i8928048 = load <4 x float>, ptr %_22, align 16
  %_14.i.i.i.i8958049 = load <4 x float>, ptr %58, align 16
  %_17.i.i.i.i8988050 = load <4 x float>, ptr %59, align 16
  %_20.i.i.i.i9018051 = load <4 x float>, ptr %60, align 16
  %_25.i.i.i.i9068052 = load <4 x float>, ptr %row1.i.i.i.i904, align 16
  %_28.i.i.i.i9098053 = load <4 x float>, ptr %61, align 16
  %_31.i.i.i.i9128054 = load <4 x float>, ptr %62, align 16
  %_34.i.i.i.i9158055 = load <4 x float>, ptr %63, align 16
  %_39.i.i.i.i9208056 = load <4 x float>, ptr %row3.i.i.i.i918, align 16
  %_42.i.i.i.i9238057 = load <4 x float>, ptr %64, align 16
  %_45.i.i.i.i9268058 = load <4 x float>, ptr %65, align 16
  %_48.i.i.i.i9298059 = load <4 x float>, ptr %66, align 16
  %_53.i.i.i.i9348060 = load <4 x float>, ptr %row5.i.i.i.i932, align 16
  %_56.i.i.i.i9378061 = load <4 x float>, ptr %67, align 16
  %_59.i.i.i.i9408062 = load <4 x float>, ptr %68, align 16
  %_62.i.i.i.i9438063 = load <4 x float>, ptr %69, align 16
  %_67.i.i.i.i9488064 = load <4 x float>, ptr %row7.i.i.i.i946, align 16
  %_70.i.i.i.i9518065 = load <4 x float>, ptr %70, align 16
  %_73.i.i.i.i9548066 = load <4 x float>, ptr %71, align 16
  %_76.i.i.i.i9578067 = load <4 x float>, ptr %72, align 16
  %_81.i.i.i.i9628068 = load <4 x float>, ptr %row9.i.i.i.i960, align 16
  %_84.i.i.i.i9658069 = load <4 x float>, ptr %73, align 16
  %_87.i.i.i.i9688070 = load <4 x float>, ptr %74, align 16
  %_90.i.i.i.i9718071 = load <4 x float>, ptr %75, align 16
  %_95.i.i.i.i9768072 = load <4 x float>, ptr %row11.i.i.i.i974, align 16
  %_98.i.i.i.i9798073 = load <4 x float>, ptr %76, align 16
  %_101.i.i.i.i9828074 = load <4 x float>, ptr %77, align 16
  %_104.i.i.i.i9858075 = load <4 x float>, ptr %78, align 16
  %_109.i.i.i.i9908076 = load <4 x float>, ptr %row13.i.i.i.i988, align 16
  %_112.i.i.i.i9938077 = load <4 x float>, ptr %79, align 16
  %_115.i.i.i.i9968078 = load <4 x float>, ptr %80, align 16
  %_118.i.i.i.i9998079 = load <4 x float>, ptr %81, align 16
  %_123.i.i.i.i10048080 = load <4 x float>, ptr %row15.i.i.i.i1002, align 16
  %_126.i.i.i.i10078081 = load <4 x float>, ptr %82, align 16
  %_129.i.i.i.i10108082 = load <4 x float>, ptr %83, align 16
  %_132.i.i.i.i10138083 = load <4 x float>, ptr %84, align 16
  %_137.i.i.i.i10188084 = load <4 x float>, ptr %row17.i.i.i.i1016, align 16
  %_140.i.i.i.i10218085 = load <4 x float>, ptr %85, align 16
  %_143.i.i.i.i10248086 = load <4 x float>, ptr %86, align 16
  %_146.i.i.i.i10278087 = load <4 x float>, ptr %87, align 16
  %_151.i.i.i.i10328088 = load <4 x float>, ptr %row19.i.i.i.i1030, align 16
  %_154.i.i.i.i10358089 = load <4 x float>, ptr %88, align 16
  %_157.i.i.i.i10388090 = load <4 x float>, ptr %89, align 16
  %_160.i.i.i.i10418091 = load <4 x float>, ptr %90, align 16
  %_165.i.i.i.i10468092 = load <4 x float>, ptr %row21.i.i.i.i1044, align 16
  %_168.i.i.i.i10498093 = load <4 x float>, ptr %91, align 16
  %_171.i.i.i.i10528094 = load <4 x float>, ptr %92, align 16
  %_174.i.i.i.i10558095 = load <4 x float>, ptr %93, align 16
  br label %bb5.i.i873, !dbg !14341

bb5.i.i873:                                       ; preds = %bb5.i.i873.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit
  %iter.sroa.0.0.i.i7409182 = phi i32 [ 0, %bb5.i.i873.lr.ph ], [ %123, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.35.09181 = phi <4 x i32> [ %history.i.i710.sroa.35.0.lcssa12060, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.32.09180, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.32.09180 = phi <4 x i32> [ %history.i.i710.sroa.32.0.lcssa12041, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.29.09179, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.29.09179 = phi <4 x i32> [ %history.i.i710.sroa.29.0.lcssa12022, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.26.09178, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.26.09178 = phi <4 x i32> [ %history.i.i710.sroa.26.0.lcssa12003, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.22.09177, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.22.09177 = phi <4 x i32> [ %history.i.i710.sroa.22.0.lcssa11984, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.19.09176, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.19.09176 = phi <4 x i32> [ %history.i.i710.sroa.19.0.lcssa11965, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.16.09175, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.16.09175 = phi <4 x i32> [ %history.i.i710.sroa.16.0.lcssa11946, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.13.09174, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.13.09174 = phi <4 x i32> [ %history.i.i710.sroa.13.0.lcssa11927, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.10.09173, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.10.09173 = phi <4 x i32> [ %history.i.i710.sroa.10.0.lcssa11908, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.7.09172, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.7.09172 = phi <4 x i32> [ %history.i.i710.sroa.7.0.lcssa11889, %bb5.i.i873.lr.ph ], [ %history.i.i710.sroa.0.09171, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %history.i.i710.sroa.0.09171 = phi <4 x i32> [ %history.i.i710.sroa.0.0.lcssa11870, %bb5.i.i873.lr.ph ], [ %lanes.i2974.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ]
  %123 = add nuw nsw i32 %iter.sroa.0.0.i.i7409182, 1, !dbg !14342
  %_11.i20.i = add nuw nsw i32 %iter.sroa.0.0.i.i7409182, %iter.sroa.0.0.i7329332, !dbg !14345
  %base.i.i874 = shl i32 %_11.i20.i, 2, !dbg !14345
  %_24.i.i875 = icmp ugt i32 %base.i.i874, %left_io.1, !dbg !14346
  br i1 %_24.i.i875, label %bb7.i.i1073, label %bb8.i.i876, !dbg !14346, !prof !902

bb8.i.i876:                                       ; preds = %bb5.i.i873
  %_27.i.i877 = sub nuw nsw i32 %left_io.1, %base.i.i874, !dbg !14349
  %_8.i2977 = icmp samesign ugt i32 %_27.i.i877, 3, !dbg !14350
  br i1 %_8.i2977, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit, label %bb2.i2978, !dbg !14350, !prof !1153

bb2.i2978:                                        ; preds = %bb8.i.i876
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa11870, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa11889, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa11908, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa11927, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa11946, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa11965, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa11984, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa12003, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa12022, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa12041, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa12060, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa12079, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i.i877, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !14356, !noalias !14357
  unreachable, !dbg !14356

bb7.i.i1073:                                      ; preds = %bb5.i.i873
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa11870, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa11889, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa11908, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa11927, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa11946, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa11965, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa11984, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa12003, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa12022, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa12041, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa12060, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa12079, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i.i874, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !14362, !noalias !14363
  unreachable, !dbg !14362

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit: ; preds = %bb8.i.i876
  %_31.i21.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i.i874, !dbg !14364
  %lanes.i2974.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i21.i, align 4, !dbg !14366, !alias.scope !14370, !noalias !14374
  %124 = bitcast <4 x i32> %history.i.i710.sroa.19.09176 to <4 x float>, !dbg !14376
  %125 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %124), !dbg !14381
  %126 = bitcast <4 x i32> %lanes.i2974.sroa.0.0.copyload to <4 x float>, !dbg !14382
  %127 = fmul <4 x float> %_11.i.i.i.i8928048, %126, !dbg !14387
  %128 = fadd <4 x float> %127, zeroinitializer, !dbg !14388
  %129 = fmul <4 x float> %_14.i.i.i.i8958049, %126, !dbg !14392
  %130 = fadd <4 x float> %129, zeroinitializer, !dbg !14396
  %131 = fmul <4 x float> %_17.i.i.i.i8988050, %126, !dbg !14400
  %132 = fadd <4 x float> %131, zeroinitializer, !dbg !14404
  %133 = fmul <4 x float> %_20.i.i.i.i9018051, %126, !dbg !14408
  %134 = fadd <4 x float> %133, zeroinitializer, !dbg !14412
  %135 = bitcast <4 x i32> %history.i.i710.sroa.0.09171 to <4 x float>, !dbg !14416
  %136 = fmul <4 x float> %_25.i.i.i.i9068052, %135, !dbg !14420
  %137 = fadd <4 x float> %128, %136, !dbg !14421
  %138 = fmul <4 x float> %_28.i.i.i.i9098053, %135, !dbg !14425
  %139 = fadd <4 x float> %130, %138, !dbg !14429
  %140 = fmul <4 x float> %_31.i.i.i.i9128054, %135, !dbg !14433
  %141 = fadd <4 x float> %132, %140, !dbg !14437
  %142 = fmul <4 x float> %_34.i.i.i.i9158055, %135, !dbg !14441
  %143 = fadd <4 x float> %134, %142, !dbg !14445
  %144 = bitcast <4 x i32> %history.i.i710.sroa.7.09172 to <4 x float>, !dbg !14449
  %145 = fmul <4 x float> %_39.i.i.i.i9208056, %144, !dbg !14453
  %146 = fadd <4 x float> %137, %145, !dbg !14454
  %147 = fmul <4 x float> %_42.i.i.i.i9238057, %144, !dbg !14458
  %148 = fadd <4 x float> %139, %147, !dbg !14462
  %149 = fmul <4 x float> %_45.i.i.i.i9268058, %144, !dbg !14466
  %150 = fadd <4 x float> %141, %149, !dbg !14470
  %151 = fmul <4 x float> %_48.i.i.i.i9298059, %144, !dbg !14474
  %152 = fadd <4 x float> %143, %151, !dbg !14478
  %153 = bitcast <4 x i32> %history.i.i710.sroa.10.09173 to <4 x float>, !dbg !14482
  %154 = fmul <4 x float> %_53.i.i.i.i9348060, %153, !dbg !14486
  %155 = fadd <4 x float> %146, %154, !dbg !14487
  %156 = fmul <4 x float> %_56.i.i.i.i9378061, %153, !dbg !14491
  %157 = fadd <4 x float> %148, %156, !dbg !14495
  %158 = fmul <4 x float> %_59.i.i.i.i9408062, %153, !dbg !14499
  %159 = fadd <4 x float> %150, %158, !dbg !14503
  %160 = fmul <4 x float> %_62.i.i.i.i9438063, %153, !dbg !14507
  %161 = fadd <4 x float> %152, %160, !dbg !14511
  %162 = bitcast <4 x i32> %history.i.i710.sroa.13.09174 to <4 x float>, !dbg !14515
  %163 = fmul <4 x float> %_67.i.i.i.i9488064, %162, !dbg !14519
  %164 = fadd <4 x float> %155, %163, !dbg !14520
  %165 = fmul <4 x float> %_70.i.i.i.i9518065, %162, !dbg !14524
  %166 = fadd <4 x float> %157, %165, !dbg !14528
  %167 = fmul <4 x float> %_73.i.i.i.i9548066, %162, !dbg !14532
  %168 = fadd <4 x float> %159, %167, !dbg !14536
  %169 = fmul <4 x float> %_76.i.i.i.i9578067, %162, !dbg !14540
  %170 = fadd <4 x float> %161, %169, !dbg !14544
  %171 = bitcast <4 x i32> %history.i.i710.sroa.16.09175 to <4 x float>, !dbg !14548
  %172 = fmul <4 x float> %_81.i.i.i.i9628068, %171, !dbg !14552
  %173 = fadd <4 x float> %164, %172, !dbg !14553
  %174 = fmul <4 x float> %_84.i.i.i.i9658069, %171, !dbg !14557
  %175 = fadd <4 x float> %166, %174, !dbg !14561
  %176 = fmul <4 x float> %_87.i.i.i.i9688070, %171, !dbg !14565
  %177 = fadd <4 x float> %168, %176, !dbg !14569
  %178 = fmul <4 x float> %_90.i.i.i.i9718071, %171, !dbg !14573
  %179 = fadd <4 x float> %170, %178, !dbg !14577
  %180 = fmul <4 x float> %_95.i.i.i.i9768072, %124, !dbg !14581
  %181 = fadd <4 x float> %173, %180, !dbg !14585
  %182 = fmul <4 x float> %_98.i.i.i.i9798073, %124, !dbg !14589
  %183 = fadd <4 x float> %175, %182, !dbg !14593
  %184 = fmul <4 x float> %_101.i.i.i.i9828074, %124, !dbg !14597
  %185 = fadd <4 x float> %177, %184, !dbg !14601
  %186 = fmul <4 x float> %_104.i.i.i.i9858075, %124, !dbg !14605
  %187 = fadd <4 x float> %179, %186, !dbg !14609
  %188 = bitcast <4 x i32> %history.i.i710.sroa.22.09177 to <4 x float>, !dbg !14613
  %189 = fmul <4 x float> %_109.i.i.i.i9908076, %188, !dbg !14617
  %190 = fadd <4 x float> %181, %189, !dbg !14618
  %191 = fmul <4 x float> %_112.i.i.i.i9938077, %188, !dbg !14622
  %192 = fadd <4 x float> %183, %191, !dbg !14626
  %193 = fmul <4 x float> %_115.i.i.i.i9968078, %188, !dbg !14630
  %194 = fadd <4 x float> %185, %193, !dbg !14634
  %195 = fmul <4 x float> %_118.i.i.i.i9998079, %188, !dbg !14638
  %196 = fadd <4 x float> %187, %195, !dbg !14642
  %197 = bitcast <4 x i32> %history.i.i710.sroa.26.09178 to <4 x float>, !dbg !14646
  %198 = fmul <4 x float> %_123.i.i.i.i10048080, %197, !dbg !14650
  %199 = fadd <4 x float> %190, %198, !dbg !14651
  %200 = fmul <4 x float> %_126.i.i.i.i10078081, %197, !dbg !14655
  %201 = fadd <4 x float> %192, %200, !dbg !14659
  %202 = fmul <4 x float> %_129.i.i.i.i10108082, %197, !dbg !14663
  %203 = fadd <4 x float> %194, %202, !dbg !14667
  %204 = fmul <4 x float> %_132.i.i.i.i10138083, %197, !dbg !14671
  %205 = fadd <4 x float> %196, %204, !dbg !14675
  %206 = bitcast <4 x i32> %history.i.i710.sroa.29.09179 to <4 x float>, !dbg !14679
  %207 = fmul <4 x float> %_137.i.i.i.i10188084, %206, !dbg !14683
  %208 = fadd <4 x float> %199, %207, !dbg !14684
  %209 = fmul <4 x float> %_140.i.i.i.i10218085, %206, !dbg !14688
  %210 = fadd <4 x float> %201, %209, !dbg !14692
  %211 = fmul <4 x float> %_143.i.i.i.i10248086, %206, !dbg !14696
  %212 = fadd <4 x float> %203, %211, !dbg !14700
  %213 = fmul <4 x float> %_146.i.i.i.i10278087, %206, !dbg !14704
  %214 = fadd <4 x float> %205, %213, !dbg !14708
  %215 = bitcast <4 x i32> %history.i.i710.sroa.32.09180 to <4 x float>, !dbg !14712
  %216 = fmul <4 x float> %_151.i.i.i.i10328088, %215, !dbg !14716
  %217 = fadd <4 x float> %208, %216, !dbg !14717
  %218 = fmul <4 x float> %_154.i.i.i.i10358089, %215, !dbg !14721
  %219 = fadd <4 x float> %210, %218, !dbg !14725
  %220 = fmul <4 x float> %_157.i.i.i.i10388090, %215, !dbg !14729
  %221 = fadd <4 x float> %212, %220, !dbg !14733
  %222 = fmul <4 x float> %_160.i.i.i.i10418091, %215, !dbg !14737
  %223 = fadd <4 x float> %214, %222, !dbg !14741
  %224 = bitcast <4 x i32> %history.i.i710.sroa.35.09181 to <4 x float>, !dbg !14745
  %225 = fmul <4 x float> %_165.i.i.i.i10468092, %224, !dbg !14749
  %226 = fadd <4 x float> %217, %225, !dbg !14750
  %227 = fmul <4 x float> %_168.i.i.i.i10498093, %224, !dbg !14754
  %228 = fadd <4 x float> %219, %227, !dbg !14758
  %229 = fmul <4 x float> %_171.i.i.i.i10528094, %224, !dbg !14762
  %230 = fadd <4 x float> %221, %229, !dbg !14766
  %231 = fmul <4 x float> %_174.i.i.i.i10558095, %224, !dbg !14770
  %232 = fadd <4 x float> %223, %231, !dbg !14774
  %233 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %226), !dbg !14778
  %234 = fcmp olt <4 x float> %233, %125, !dbg !14782
  %235 = select <4 x i1> %234, <4 x float> %125, <4 x float> %233, !dbg !14786
  %236 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %228), !dbg !14778
  %237 = fcmp olt <4 x float> %236, %235, !dbg !14782
  %238 = select <4 x i1> %237, <4 x float> %235, <4 x float> %236, !dbg !14786
  %239 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %230), !dbg !14778
  %240 = fcmp olt <4 x float> %239, %238, !dbg !14782
  %241 = select <4 x i1> %240, <4 x float> %238, <4 x float> %239, !dbg !14786
  %242 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %232), !dbg !14778
  %243 = fcmp olt <4 x float> %242, %241, !dbg !14782
  %244 = select <4 x i1> %243, <4 x float> %241, <4 x float> %242, !dbg !14786
  %_39.i.i1067.idx = shl i32 %iter.sroa.0.0.i.i7409182, 4, !dbg !14787
  %_39.i.i1067 = getelementptr inbounds nuw i8, ptr %peaks_left.i713, i32 %_39.i.i1067.idx, !dbg !14787
  store <4 x float> %244, ptr %_39.i.i1067, align 4, !dbg !14792, !alias.scope !14797, !noalias !14801
  %exitcond.not = icmp eq i32 %123, %umax11132, !dbg !14334
  br i1 %exitcond.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742, label %bb5.i.i873, !dbg !14341

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit, %bb29.i737
  %history.i.i710.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.0.0.lcssa11870, %bb29.i737 ], [ %lanes.i2974.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.7.0.lcssa11889, %bb29.i737 ], [ %history.i.i710.sroa.0.09171, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.10.0.lcssa11908, %bb29.i737 ], [ %history.i.i710.sroa.7.09172, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.13.0.lcssa11927, %bb29.i737 ], [ %history.i.i710.sroa.10.09173, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.16.0.lcssa11946, %bb29.i737 ], [ %history.i.i710.sroa.13.09174, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.19.0.lcssa11965, %bb29.i737 ], [ %history.i.i710.sroa.16.09175, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.22.0.lcssa11984, %bb29.i737 ], [ %history.i.i710.sroa.19.09176, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.26.0.lcssa12003, %bb29.i737 ], [ %history.i.i710.sroa.22.09177, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.29.0.lcssa12022, %bb29.i737 ], [ %history.i.i710.sroa.26.09178, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.32.0.lcssa12041, %bb29.i737 ], [ %history.i.i710.sroa.29.09179, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.35.0.lcssa12060, %bb29.i737 ], [ %history.i.i710.sroa.32.09180, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  %history.i.i710.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i710.sroa.38.0.lcssa12079, %bb29.i737 ], [ %history.i.i710.sroa.35.09181, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit ], !dbg !14355
  br i1 %_20.i.i7419170.not, label %bb12.i731.loopexit, label %bb34.i748.lr.ph, !dbg !14805

bb34.i748.lr.ph:                                  ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i742
  %_60.i770 = load i32, ptr %101, align 4
  %_67.i861 = load i32, ptr %119, align 4
  br label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3022, !dbg !14805

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3022: ; preds = %bb34.i748.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395
  %245 = phi <4 x float> [ %.lcssa1186012141, %bb34.i748.lr.ph ], [ %299, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %246 = phi <4 x float> [ %.lcssa1175412120, %bb34.i748.lr.ph ], [ %260, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %247 = phi <4 x float> [ %.lcssa1177212100, %bb34.i748.lr.ph ], [ %257, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %main_cursor.sroa.0.1.i7469209 = phi i32 [ %main_cursor.sroa.0.0.i7359335, %bb34.i748.lr.ph ], [ %spec.store.select7.i863, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %ring_cursor.sroa.0.1.i7459208 = phi i32 [ %ring_cursor.sroa.0.0.i7349334, %bb34.i748.lr.ph ], [ %spec.store.select8.i865, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %iter1.sroa.0.0.i7449207 = phi i32 [ 0, %bb34.i748.lr.ph ], [ %256, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395 ]
  %248 = fadd <4 x float> %247, splat (float -1.000000e+00), !dbg !14812
  %249 = fcmp ogt <4 x float> %248, zeroinitializer, !dbg !14820
  %250 = sext <4 x i1> %249 to <4 x i32>, !dbg !14824
  %_11.i13888029 = load <4 x float>, ptr %_47.i751, align 16, !dbg !14829, !alias.scope !14830, !noalias !14833
  %_12.i1389 = load <4 x i32>, ptr %95, align 16, !dbg !14838, !alias.scope !14830, !noalias !14833
  %251 = bitcast <4 x i32> %_12.i1389 to <4 x float>, !dbg !14839
  %252 = fadd <4 x float> %_11.i13888029, %251, !dbg !14843
  %253 = bitcast <4 x float> %252 to <16 x i8>, !dbg !14844
  %254 = bitcast <4 x i32> %250 to <16 x i8>, !dbg !14848
  %_4.i3695 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %253, <16 x i8> %_13.i13918030, <16 x i8> %254), !dbg !14849
  store <16 x i8> %_4.i3695, ptr %_47.i751, align 16, !dbg !14850, !alias.scope !14830, !noalias !14833
  %255 = bitcast <4 x i32> %_12.i1389 to <16 x i8>, !dbg !14851
  %_4.i3696 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %255, <16 x i8> zeroinitializer, <16 x i8> %254), !dbg !14855
  store <16 x i8> %_4.i3696, ptr %95, align 16, !dbg !14856, !alias.scope !14830, !noalias !14833
  %256 = add nuw nsw i32 %iter1.sroa.0.0.i7449207, 1, !dbg !14857
  %_43.i749 = add nuw nsw i32 %iter1.sroa.0.0.i7449207, %iter.sroa.0.0.i7329332, !dbg !14863
  %base.i750 = shl i32 %_43.i749, 2, !dbg !14863
  %257 = select <4 x i1> %249, <4 x float> %248, <4 x float> zeroinitializer, !dbg !14864
  %258 = fadd <4 x float> %246, splat (float -1.000000e+00), !dbg !14865
  %259 = fcmp ogt <4 x float> %258, zeroinitializer, !dbg !14870
  %260 = select <4 x i1> %259, <4 x float> %258, <4 x float> zeroinitializer, !dbg !14874
  %261 = sext <4 x i1> %259 to <4 x i32>, !dbg !14875
  %_11.i8033 = load <4 x float>, ptr %_48.i752, align 16, !dbg !14880, !alias.scope !14881, !noalias !14884
  %_12.i1376 = load <4 x i32>, ptr %98, align 16, !dbg !14886, !alias.scope !14881, !noalias !14884
  %262 = bitcast <4 x i32> %_12.i1376 to <4 x float>, !dbg !14887
  %263 = fadd <4 x float> %_11.i8033, %262, !dbg !14891
  %264 = bitcast <4 x float> %263 to <16 x i8>, !dbg !14892
  %265 = bitcast <4 x i32> %261 to <16 x i8>, !dbg !14896
  %_4.i3697 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %264, <16 x i8> %_13.i13788034, <16 x i8> %265), !dbg !14897
  store <16 x i8> %_4.i3697, ptr %_48.i752, align 16, !dbg !14898, !alias.scope !14881, !noalias !14884
  %266 = bitcast <4 x i32> %_12.i1376 to <16 x i8>, !dbg !14899
  %_4.i3698 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %266, <16 x i8> zeroinitializer, <16 x i8> %265), !dbg !14903
  store <16 x i8> %_4.i3698, ptr %98, align 16, !dbg !14904, !alias.scope !14881, !noalias !14884
  %_89.i761.idx = shl i32 %iter1.sroa.0.0.i7449207, 4, !dbg !14905
  %_89.i761 = getelementptr inbounds nuw i8, ptr %peaks_left.i713, i32 %_89.i761.idx, !dbg !14905
  %lanes.i3015.sroa.0.0.copyload = load <16 x i8>, ptr %_89.i761, align 4, !dbg !14917, !alias.scope !14922, !noalias !14926
  %_4.i3699 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %lanes.i3015.sroa.0.0.copyload, <16 x i8> %lanes.i3015.sroa.0.0.copyload, <16 x i8> %100), !dbg !14930
  %_90.i765 = icmp ugt i32 %base.i750, %left_io.1, !dbg !14936
  br i1 %_90.i765, label %bb38.i871, label %bb39.i766, !dbg !14936, !prof !902

bb39.i766:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3022
  %_93.i767 = sub nuw nsw i32 %left_io.1, %base.i750, !dbg !14941
  %_97.i768 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i750, !dbg !14942
  %_8.i3009 = icmp samesign ugt i32 %_93.i767, 3, !dbg !14947
  br i1 %_8.i3009, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3013, label %bb2.i3010, !dbg !14947, !prof !1153

bb2.i3010:                                        ; preds = %bb39.i766
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i767, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !14952, !noalias !14953
  unreachable, !dbg !14952

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3013: ; preds = %bb39.i766
  %lanes.i3006.sroa.0.0.copyload = load <4 x i32>, ptr %_97.i768, align 4, !dbg !14957, !alias.scope !14961, !noalias !14965
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14967), !dbg !14970
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14971), !dbg !14970
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14973), !dbg !14970
  %width.i.i772 = load i32, ptr %102, align 4, !dbg !14975, !alias.scope !14977, !noalias !14978, !noundef !10
  %267 = bitcast <16 x i8> %_4.i3699 to <4 x float>, !dbg !14985
  %268 = bitcast <16 x i8> %_4.i3695 to <4 x float>, !dbg !14990
  %269 = fcmp ogt <4 x float> %267, %268, !dbg !14991
  %270 = sext <4 x i1> %269 to <4 x i32>, !dbg !14991
  %271 = fdiv <4 x float> %268, %267, !dbg !14992
  %272 = bitcast <4 x float> %271 to <16 x i8>, !dbg !14996
  %273 = bitcast <4 x i32> %270 to <16 x i8>, !dbg !15000
  %_4.i3700 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %272, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %273), !dbg !15001
  %_158.1.i.i777 = load i32, ptr %103, align 4, !dbg !15002, !alias.scope !14977, !noalias !14978, !noundef !10
  %_22.i.i778 = mul i32 %width.i.i772, %ring_cursor.sroa.0.1.i7459208, !dbg !15003
  %_90.i.i779 = icmp ugt i32 %_22.i.i778, %_158.1.i.i777, !dbg !15004
  br i1 %_90.i.i779, label %bb34.i.i870, label %bb35.i.i780, !dbg !15004, !prof !902

bb35.i.i780:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3013
  %_93.i.i782 = sub nuw i32 %_158.1.i.i777, %_22.i.i778, !dbg !15007
  %_8.i3407 = icmp samesign ugt i32 %_93.i.i782, 3, !dbg !15008
  br i1 %_8.i3407, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3410, label %bb2.i3408, !dbg !15008, !prof !1153

bb2.i3408:                                        ; preds = %bb35.i.i780
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i.i782, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !15013, !noalias !15014
  unreachable, !dbg !15013

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3410: ; preds = %bb35.i.i780
  %_158.0.i.i781 = load ptr, ptr %104, align 4, !dbg !15002, !alias.scope !14977, !noalias !14978, !nonnull !10, !noundef !10
  %_97.i.i783 = getelementptr inbounds nuw float, ptr %_158.0.i.i781, i32 %_22.i.i778, !dbg !15018
  store <16 x i8> %_4.i3700, ptr %_97.i.i783, align 4, !dbg !15020, !alias.scope !15024, !noalias !15028
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15030), !dbg !15033
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15034), !dbg !15033
  %width.i = load i32, ptr %102, align 4, !dbg !15036, !alias.scope !15030, !noalias !15038, !noundef !10
  %274 = icmp eq i32 %width.i, 0, !dbg !15039
  br i1 %274, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i1187.lr.ph, !dbg !15039

bb29.i1187.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3410
  %_126.1.i = load i32, ptr %105, align 4, !alias.scope !15030, !noalias !15038, !noundef !10
  %_126.0.i = load ptr, ptr %106, align 4, !nonnull !10
  %275 = add i32 %ring_cursor.sroa.0.1.i7459208, 1
  %_21.not.i = icmp ult i32 %275, %_60.i770
  %276 = select i1 %_21.not.i, i32 0, i32 %_60.i770
  %start1.sroa.0.0.i = sub nuw i32 %275, %276
  %_128.1.i = load i32, ptr %103, align 4
  %_128.0.i = load ptr, ptr %104, align 4, !nonnull !10
  %_130.1.i = load i32, ptr %107, align 4
  %_130.0.i = load ptr, ptr %108, align 4, !nonnull !10
  %_132.1.i = load i32, ptr %109, align 4
  %_132.0.i = load ptr, ptr %110, align 4, !nonnull !10
  %_43.i1197 = mul i32 %width.i, %start1.sroa.0.0.i
  br label %bb29.i1187, !dbg !15039

bb29.i1187:                                       ; preds = %bb29.i1187.lr.ph, %bb28.i
  %iter.sroa.0.0.idx.i9200 = phi i32 [ 0, %bb29.i1187.lr.ph ], [ %iter.sroa.0.0.add.i, %bb28.i ]
  %iter.sroa.4.0.i9199 = phi i32 [ 0, %bb29.i1187.lr.ph ], [ %_102.0.i, %bb28.i ]
  %iter.sroa.7.0.i9198 = phi i32 [ %width.i, %bb29.i1187.lr.ph ], [ %277, %bb28.i ]
  %iter.sroa.0.0.ptr.i9201 = getelementptr inbounds nuw i8, ptr %scratch.i714, i32 %iter.sroa.0.0.idx.i9200, !dbg !15041
  %277 = add i32 %iter.sroa.7.0.i9198, -1, !dbg !15041
  %_109.i = icmp eq i32 %iter.sroa.0.0.idx.i9200, 32, !dbg !15042
  br i1 %_109.i, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb33.i, !dbg !15046

bb33.i:                                           ; preds = %bb29.i1187
  %iter.sroa.0.0.add.i = add nuw nsw i32 %iter.sroa.0.0.idx.i9200, 4, !dbg !15047
  %_102.0.i = add nuw nsw i32 %iter.sroa.4.0.i9199, 1, !dbg !15049
  %exitcond11124.not = icmp eq i32 %iter.sroa.4.0.i9199, %_126.1.i, !dbg !15050
  br i1 %exitcond11124.not, label %panic.i, label %bb2.i1189, !dbg !15050

bb2.i1189:                                        ; preds = %bb33.i
  %278 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i, i32 %iter.sroa.4.0.i9199, !dbg !15050
  %shape.i = load i32, ptr %278, align 4, !dbg !15050, !noalias !15051, !noundef !10
  %279 = getelementptr inbounds nuw i8, ptr %278, i32 4, !dbg !15050
  %shape3.i = load i32, ptr %279, align 4, !dbg !15050, !noalias !15051, !noundef !10
  %280 = add i32 %shape3.i, %ring_cursor.sroa.0.1.i7459208, !dbg !15052
  %_18.not.i = icmp ult i32 %280, %_60.i770, !dbg !15053
  %281 = select i1 %_18.not.i, i32 0, i32 %_60.i770, !dbg !15053
  %spec.select.i = sub nuw i32 %280, %281, !dbg !15053
  %_25.i1190 = mul i32 %spec.select.i, %width.i, !dbg !15054
  %_24.i = add i32 %_25.i1190, %iter.sroa.4.0.i9199, !dbg !15054
  %_28.i1191 = icmp ult i32 %_24.i, %_128.1.i, !dbg !15055
  br i1 %_28.i1191, label %bb9.i, label %panic5.i, !dbg !15055

panic.i:                                          ; preds = %bb33.i
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i, i32 noundef %_126.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !15050, !noalias !15051
  unreachable, !dbg !15050

bb9.i:                                            ; preds = %bb2.i1189
  %282 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_24.i, !dbg !15055
  %283 = load float, ptr %282, align 4, !dbg !15055, !noalias !15051, !noundef !10
  %exitcond11125.not = icmp eq i32 %iter.sroa.4.0.i9199, %_130.1.i, !dbg !15056
  br i1 %exitcond11125.not, label %panic6.i, label %bb10.i, !dbg !15056

panic5.i:                                         ; preds = %bb2.i1189
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !15055, !noalias !15051
  unreachable, !dbg !15055

bb10.i:                                           ; preds = %bb9.i
  %284 = getelementptr inbounds nuw i32, ptr %_130.0.i, i32 %iter.sroa.4.0.i9199, !dbg !15056
  %_30.i = load i32, ptr %284, align 4, !dbg !15056, !noalias !15051, !noundef !10
  %285 = icmp eq i32 %_30.i, 0, !dbg !15057
  br i1 %285, label %bb14.i1195, label %bb12.i1192, !dbg !15057

panic6.i:                                         ; preds = %bb9.i
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i, i32 noundef %_130.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !15056, !noalias !15051
  unreachable, !dbg !15056

bb12.i1192:                                       ; preds = %bb10.i
  %_35.i = icmp ult i32 %iter.sroa.4.0.i9199, %_132.1.i, !dbg !15058
  br i1 %_35.i, label %bb13.i1193, label %panic7.i, !dbg !15058

bb14.i1195:                                       ; preds = %bb34.i1211, %bb13.i1193, %bb10.i
  %newest.sroa.0.0.i = phi float [ %283, %bb10.i ], [ %_33.i, %bb34.i1211 ], [ %283, %bb13.i1193 ], !dbg !15059
  %exitcond11126.not = icmp eq i32 %iter.sroa.4.0.i9199, %_132.1.i, !dbg !15060
  br i1 %exitcond11126.not, label %panic8.i, label %bb15.i, !dbg !15060

bb13.i1193:                                       ; preds = %bb12.i1192
  %286 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i9199, !dbg !15058
  %_33.i = load float, ptr %286, align 4, !dbg !15058, !noalias !15051, !noundef !10
  %_116.i1194 = fcmp olt float %_33.i, %283, !dbg !15061
  br i1 %_116.i1194, label %bb34.i1211, label %bb14.i1195, !dbg !15061

panic7.i:                                         ; preds = %bb12.i1192
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i9199, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !15058, !noalias !15051
  unreachable, !dbg !15058

bb34.i1211:                                       ; preds = %bb13.i1193
  br label %bb14.i1195, !dbg !15063

bb15.i:                                           ; preds = %bb14.i1195
  %287 = getelementptr inbounds nuw float, ptr %_132.0.i, i32 %iter.sroa.4.0.i9199, !dbg !15060
  store float %newest.sroa.0.0.i, ptr %287, align 4, !dbg !15060, !noalias !15051
  %_40.i = add i32 %_30.i, 1, !dbg !15064
  %complete.i = icmp eq i32 %_40.i, %shape.i, !dbg !15064
  br i1 %complete.i, label %bb19.i, label %bb17.i1196, !dbg !15065

panic8.i:                                         ; preds = %bb14.i1195
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i, i32 noundef %_132.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !15060, !noalias !15051
  unreachable, !dbg !15060

bb17.i1196:                                       ; preds = %bb15.i
  %_42.i = add i32 %iter.sroa.4.0.i9199, %_43.i1197, !dbg !15066
  %_45.i = icmp ult i32 %_42.i, %_128.1.i, !dbg !15067
  br i1 %_45.i, label %bb27.i, label %panic9.i, !dbg !15067

panic9.i:                                         ; preds = %bb17.i1196
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !15067, !noalias !15051
  unreachable, !dbg !15067

bb27.i:                                           ; preds = %bb17.i1196
  %288 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_42.i, !dbg !15067
  %_41.i1198 = load float, ptr %288, align 4, !dbg !15067, !noalias !15051, !noundef !10
  %_117.i = fcmp olt float %_41.i1198, %newest.sroa.0.0.i, !dbg !15068
  %newest.sroa.0.1.i = select i1 %_117.i, float %_41.i1198, float %newest.sroa.0.0.i, !dbg !15068
  store float %newest.sroa.0.1.i, ptr %iter.sroa.0.0.ptr.i9201, align 4, !dbg !15070, !alias.scope !15034, !noalias !15071
  br label %bb28.i, !dbg !15072

bb28.i:                                           ; preds = %bb22.i, %bb19.i, %bb27.i
  %storemerge = phi i32 [ %_40.i, %bb27.i ], [ 0, %bb19.i ], [ 0, %bb22.i ], !dbg !15073
  store i32 %storemerge, ptr %284, align 4, !dbg !15073, !noalias !15051
  %289 = icmp eq i32 %277, 0, !dbg !15039
  br i1 %289, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit, label %bb29.i1187, !dbg !15039

bb19.i:                                           ; preds = %bb15.i
  store float %newest.sroa.0.0.i, ptr %iter.sroa.0.0.ptr.i9201, align 4, !dbg !15070, !alias.scope !15034, !noalias !15071
  %_118.i9194.not = icmp eq i32 %shape.i, 0, !dbg !15074
  br i1 %_118.i9194.not, label %bb28.i, label %bb40.i.preheader, !dbg !15078

bb40.i.preheader:                                 ; preds = %bb19.i
  %290 = load float, ptr %282, align 4, !dbg !15079, !noalias !15051, !noundef !10
  br label %bb40.i, !dbg !15080

bb40.i:                                           ; preds = %bb40.i.preheader, %bb22.i
  %iter2.sroa.0.0.i12029197 = phi i32 [ %_119.i1203, %bb22.i ], [ 0, %bb40.i.preheader ]
  %suffix.sroa.0.0.i9196 = phi float [ %suffix.sroa.0.1.i, %bb22.i ], [ %290, %bb40.i.preheader ]
  %end.sroa.0.1.i9195 = phi i32 [ %293, %bb22.i ], [ %spec.select.i, %bb40.i.preheader ]
  %_54.i1204 = mul i32 %end.sroa.0.1.i9195, %width.i, !dbg !15081
  %_53.i = add i32 %_54.i1204, %iter.sroa.4.0.i9199, !dbg !15081
  %_57.i1205 = icmp ult i32 %_53.i, %_128.1.i, !dbg !15080
  br i1 %_57.i1205, label %bb22.i, label %panic13.i, !dbg !15080

panic13.i:                                        ; preds = %bb40.i
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i, i32 noundef %_128.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !15080, !noalias !15051
  unreachable, !dbg !15080

bb22.i:                                           ; preds = %bb40.i
  %_119.i1203 = add nuw i32 %iter2.sroa.0.0.i12029197, 1, !dbg !15082
  %291 = getelementptr inbounds nuw float, ptr %_128.0.i, i32 %_53.i, !dbg !15080
  %_52.i1207 = load float, ptr %291, align 4, !dbg !15080, !noalias !15051, !noundef !10
  %_121.i1208 = fcmp olt float %suffix.sroa.0.0.i9196, %_52.i1207, !dbg !15085
  %suffix.sroa.0.1.i = select i1 %_121.i1208, float %suffix.sroa.0.0.i9196, float %_52.i1207, !dbg !15085
  store float %suffix.sroa.0.1.i, ptr %291, align 4, !dbg !15087, !noalias !15051
  %292 = icmp eq i32 %end.sroa.0.1.i9195, 0, !dbg !15088
  %spec.store.select.i1210 = select i1 %292, i32 %_60.i770, i32 %end.sroa.0.1.i9195, !dbg !15088
  %293 = add i32 %spec.store.select.i1210, -1, !dbg !15089
  %exitcond11123.not = icmp eq i32 %_119.i1203, %shape.i, !dbg !15074
  br i1 %exitcond11123.not, label %bb28.i, label %bb40.i, !dbg !15078

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit: ; preds = %bb29.i1187, %bb28.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3410
  %lanes.i2999.sroa.0.0.copyload = load <4 x float>, ptr %scratch.i714, align 4, !dbg !15090, !alias.scope !15095, !noalias !15099
  %294 = fmul <4 x float> %lanes.i2999.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !15103
  %295 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %294), !dbg !15107
  %296 = fmul <4 x float> %295, splat (float 0x3F10000000000000), !dbg !15111
  %297 = icmp eq i32 %width.i.i772, 0, !dbg !15115
  %_163.1.i.i821.pre = load i32, ptr %111, align 4, !dbg !15117, !alias.scope !14977, !noalias !14978
  br i1 %297, label %bb53.i.i816, label %bb36.i.i795.lr.ph, !dbg !15115

bb36.i.i795.lr.ph:                                ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %_159.1.i.i800 = load i32, ptr %105, align 4, !alias.scope !14977, !noalias !14978, !noundef !10
  %_159.0.i.i804 = load ptr, ptr %106, align 4, !nonnull !10
  %_161.0.i.i814 = load ptr, ptr %112, align 4, !nonnull !10
  %exitcond11129.not = icmp eq i32 %_159.1.i.i800, 0, !dbg !15118
  br i1 %exitcond11129.not, label %panic.i.i802, label %bb14.i.i803, !dbg !15118

bb34.i.i870:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3013
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i778, i32 noundef %_158.1.i.i777, i32 noundef %_158.1.i.i777, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !15119, !noalias !15120
  unreachable, !dbg !15119

bb53.i.i816.loopexit:                             ; preds = %bb18.i.i813.7, %bb18.i.i813.6, %bb18.i.i813.5, %bb18.i.i813.4, %bb18.i.i813.3, %bb18.i.i813.2, %bb18.i.i813.1, %bb18.i.i813
  %lanes.i2992.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i714, align 4, !dbg !15121, !alias.scope !15126, !noalias !15130
  br label %bb53.i.i816, !dbg !15134

bb53.i.i816:                                      ; preds = %bb53.i.i816.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit
  %lanes.i2992.sroa.0.0.copyload = phi <4 x float> [ %lanes.i2992.sroa.0.0.copyload.pre, %bb53.i.i816.loopexit ], [ %lanes.i2999.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit ], !dbg !15121
  %298 = fadd <4 x float> %296, %245, !dbg !15135
  %299 = fsub <4 x float> %298, %lanes.i2992.sroa.0.0.copyload, !dbg !15139
  %_123.i.i822 = icmp ugt i32 %_22.i.i778, %_163.1.i.i821.pre, !dbg !15143
  br i1 %_123.i.i822, label %bb41.i.i869, label %bb42.i.i823, !dbg !15143, !prof !902

bb14.i.i803:                                      ; preds = %bb36.i.i795.lr.ph
  %300 = getelementptr inbounds nuw i8, ptr %_159.0.i.i804, i32 8, !dbg !15118
  %_42.i.i805 = load i32, ptr %300, align 4, !dbg !15118, !noalias !15146, !noundef !10
  %301 = add i32 %_42.i.i805, %ring_cursor.sroa.0.1.i7459208, !dbg !15147
  %_45.not.i.i806 = icmp ult i32 %301, %_60.i770, !dbg !15148
  %302 = select i1 %_45.not.i.i806, i32 0, i32 %_60.i770, !dbg !15148
  %spec.select.i.i807 = sub nuw i32 %301, %302, !dbg !15148
  %_49.i.i808 = mul i32 %spec.select.i.i807, %width.i.i772, !dbg !15149
  %_51.i.i811 = icmp ult i32 %_49.i.i808, %_163.1.i.i821.pre, !dbg !15150
  br i1 %_51.i.i811, label %bb18.i.i813, label %panic1.i.i812, !dbg !15150

panic.i.i802:                                     ; preds = %bb36.i.i795.7, %bb36.i.i795.6, %bb36.i.i795.5, %bb36.i.i795.4, %bb36.i.i795.3, %bb36.i.i795.2, %bb36.i.i795.1, %bb36.i.i795.lr.ph
  %_159.1.i.i800.lcssa.ph = phi i32 [ 7, %bb36.i.i795.7 ], [ 6, %bb36.i.i795.6 ], [ 5, %bb36.i.i795.5 ], [ 4, %bb36.i.i795.4 ], [ 3, %bb36.i.i795.3 ], [ 2, %bb36.i.i795.2 ], [ 1, %bb36.i.i795.1 ], [ 0, %bb36.i.i795.lr.ph ]
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i800.lcssa.ph, i32 noundef %_159.1.i.i800.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !15118, !noalias !15146
  unreachable, !dbg !15118

bb18.i.i813:                                      ; preds = %bb14.i.i803
  %303 = getelementptr inbounds nuw float, ptr %_161.0.i.i814, i32 %_49.i.i808, !dbg !15150
  %_47.i.i815 = load float, ptr %303, align 4, !dbg !15150, !noalias !15146, !noundef !10
  store float %_47.i.i815, ptr %scratch.i714, align 4, !dbg !15151, !alias.scope !14973, !noalias !15152
  %304 = icmp eq i32 %width.i.i772, 1, !dbg !15115
  br i1 %304, label %bb53.i.i816.loopexit, label %bb36.i.i795.1, !dbg !15115

bb36.i.i795.1:                                    ; preds = %bb18.i.i813
  %exitcond11129.1.not = icmp eq i32 %_159.1.i.i800, 1, !dbg !15118
  br i1 %exitcond11129.1.not, label %panic.i.i802, label %bb14.i.i803.1, !dbg !15118

bb14.i.i803.1:                                    ; preds = %bb36.i.i795.1
  %305 = getelementptr inbounds nuw i8, ptr %_159.0.i.i804, i32 20, !dbg !15118
  %_42.i.i805.1 = load i32, ptr %305, align 4, !dbg !15118, !noalias !15146, !noundef !10
  %306 = add i32 %_42.i.i805.1, %ring_cursor.sroa.0.1.i7459208, !dbg !15147
  %_45.not.i.i806.1 = icmp ult i32 %306, %_60.i770, !dbg !15148
  %307 = select i1 %_45.not.i.i806.1, i32 0, i32 %_60.i770, !dbg !15148
  %spec.select.i.i807.1 = sub nuw i32 %306, %307, !dbg !15148
  %_49.i.i808.1 = mul i32 %spec.select.i.i807.1, %width.i.i772, !dbg !15149
  %_48.i.i809.1 = add i32 %_49.i.i808.1, 1, !dbg !15149
  %_51.i.i811.1 = icmp ult i32 %_48.i.i809.1, %_163.1.i.i821.pre, !dbg !15150
  br i1 %_51.i.i811.1, label %bb18.i.i813.1, label %panic1.i.i812, !dbg !15150

bb18.i.i813.1:                                    ; preds = %bb14.i.i803.1
  %308 = getelementptr inbounds nuw float, ptr %_161.0.i.i814, i32 %_48.i.i809.1, !dbg !15150
  %_47.i.i815.1 = load float, ptr %308, align 4, !dbg !15150, !noalias !15146, !noundef !10
  store float %_47.i.i815.1, ptr %iter.sroa.0.0.ptr.i.i7949205.1, align 4, !dbg !15151, !alias.scope !14973, !noalias !15152
  %309 = icmp eq i32 %width.i.i772, 2, !dbg !15115
  br i1 %309, label %bb53.i.i816.loopexit, label %bb36.i.i795.2, !dbg !15115

bb36.i.i795.2:                                    ; preds = %bb18.i.i813.1
  %exitcond11129.2.not = icmp eq i32 %_159.1.i.i800, 2, !dbg !15118
  br i1 %exitcond11129.2.not, label %panic.i.i802, label %bb14.i.i803.2, !dbg !15118

bb14.i.i803.2:                                    ; preds = %bb36.i.i795.2
  %310 = getelementptr inbounds nuw i8, ptr %_159.0.i.i804, i32 32, !dbg !15118
  %_42.i.i805.2 = load i32, ptr %310, align 4, !dbg !15118, !noalias !15146, !noundef !10
  %311 = add i32 %_42.i.i805.2, %ring_cursor.sroa.0.1.i7459208, !dbg !15147
  %_45.not.i.i806.2 = icmp ult i32 %311, %_60.i770, !dbg !15148
  %312 = select i1 %_45.not.i.i806.2, i32 0, i32 %_60.i770, !dbg !15148
  %spec.select.i.i807.2 = sub nuw i32 %311, %312, !dbg !15148
  %_49.i.i808.2 = mul i32 %spec.select.i.i807.2, %width.i.i772, !dbg !15149
  %_48.i.i809.2 = add i32 %_49.i.i808.2, 2, !dbg !15149
  %_51.i.i811.2 = icmp ult i32 %_48.i.i809.2, %_163.1.i.i821.pre, !dbg !15150
  br i1 %_51.i.i811.2, label %bb18.i.i813.2, label %panic1.i.i812, !dbg !15150

bb18.i.i813.2:                                    ; preds = %bb14.i.i803.2
  %313 = getelementptr inbounds nuw float, ptr %_161.0.i.i814, i32 %_48.i.i809.2, !dbg !15150
  %_47.i.i815.2 = load float, ptr %313, align 4, !dbg !15150, !noalias !15146, !noundef !10
  store float %_47.i.i815.2, ptr %iter.sroa.0.0.ptr.i.i7949205.2, align 4, !dbg !15151, !alias.scope !14973, !noalias !15152
  %314 = icmp eq i32 %width.i.i772, 3, !dbg !15115
  br i1 %314, label %bb53.i.i816.loopexit, label %bb36.i.i795.3, !dbg !15115

bb36.i.i795.3:                                    ; preds = %bb18.i.i813.2
  %exitcond11129.3.not = icmp eq i32 %_159.1.i.i800, 3, !dbg !15118
  br i1 %exitcond11129.3.not, label %panic.i.i802, label %bb14.i.i803.3, !dbg !15118

bb14.i.i803.3:                                    ; preds = %bb36.i.i795.3
  %315 = getelementptr inbounds nuw i8, ptr %_159.0.i.i804, i32 44, !dbg !15118
  %_42.i.i805.3 = load i32, ptr %315, align 4, !dbg !15118, !noalias !15146, !noundef !10
  %316 = add i32 %_42.i.i805.3, %ring_cursor.sroa.0.1.i7459208, !dbg !15147
  %_45.not.i.i806.3 = icmp ult i32 %316, %_60.i770, !dbg !15148
  %317 = select i1 %_45.not.i.i806.3, i32 0, i32 %_60.i770, !dbg !15148
  %spec.select.i.i807.3 = sub nuw i32 %316, %317, !dbg !15148
  %_49.i.i808.3 = mul i32 %spec.select.i.i807.3, %width.i.i772, !dbg !15149
  %_48.i.i809.3 = add i32 %_49.i.i808.3, 3, !dbg !15149
  %_51.i.i811.3 = icmp ult i32 %_48.i.i809.3, %_163.1.i.i821.pre, !dbg !15150
  br i1 %_51.i.i811.3, label %bb18.i.i813.3, label %panic1.i.i812, !dbg !15150

bb18.i.i813.3:                                    ; preds = %bb14.i.i803.3
  %318 = getelementptr inbounds nuw float, ptr %_161.0.i.i814, i32 %_48.i.i809.3, !dbg !15150
  %_47.i.i815.3 = load float, ptr %318, align 4, !dbg !15150, !noalias !15146, !noundef !10
  store float %_47.i.i815.3, ptr %iter.sroa.0.0.ptr.i.i7949205.3, align 4, !dbg !15151, !alias.scope !14973, !noalias !15152
  %319 = icmp eq i32 %width.i.i772, 4, !dbg !15115
  br i1 %319, label %bb53.i.i816.loopexit, label %bb36.i.i795.4, !dbg !15115

bb36.i.i795.4:                                    ; preds = %bb18.i.i813.3
  %exitcond11129.4.not = icmp eq i32 %_159.1.i.i800, 4, !dbg !15118
  br i1 %exitcond11129.4.not, label %panic.i.i802, label %bb14.i.i803.4, !dbg !15118

bb14.i.i803.4:                                    ; preds = %bb36.i.i795.4
  %320 = getelementptr inbounds nuw i8, ptr %_159.0.i.i804, i32 56, !dbg !15118
  %_42.i.i805.4 = load i32, ptr %320, align 4, !dbg !15118, !noalias !15146, !noundef !10
  %321 = add i32 %_42.i.i805.4, %ring_cursor.sroa.0.1.i7459208, !dbg !15147
  %_45.not.i.i806.4 = icmp ult i32 %321, %_60.i770, !dbg !15148
  %322 = select i1 %_45.not.i.i806.4, i32 0, i32 %_60.i770, !dbg !15148
  %spec.select.i.i807.4 = sub nuw i32 %321, %322, !dbg !15148
  %_49.i.i808.4 = mul i32 %spec.select.i.i807.4, %width.i.i772, !dbg !15149
  %_48.i.i809.4 = add i32 %_49.i.i808.4, 4, !dbg !15149
  %_51.i.i811.4 = icmp ult i32 %_48.i.i809.4, %_163.1.i.i821.pre, !dbg !15150
  br i1 %_51.i.i811.4, label %bb18.i.i813.4, label %panic1.i.i812, !dbg !15150

bb18.i.i813.4:                                    ; preds = %bb14.i.i803.4
  %323 = getelementptr inbounds nuw float, ptr %_161.0.i.i814, i32 %_48.i.i809.4, !dbg !15150
  %_47.i.i815.4 = load float, ptr %323, align 4, !dbg !15150, !noalias !15146, !noundef !10
  store float %_47.i.i815.4, ptr %iter.sroa.0.0.ptr.i.i7949205.4, align 4, !dbg !15151, !alias.scope !14973, !noalias !15152
  %324 = icmp eq i32 %width.i.i772, 5, !dbg !15115
  br i1 %324, label %bb53.i.i816.loopexit, label %bb36.i.i795.5, !dbg !15115

bb36.i.i795.5:                                    ; preds = %bb18.i.i813.4
  %exitcond11129.5.not = icmp eq i32 %_159.1.i.i800, 5, !dbg !15118
  br i1 %exitcond11129.5.not, label %panic.i.i802, label %bb14.i.i803.5, !dbg !15118

bb14.i.i803.5:                                    ; preds = %bb36.i.i795.5
  %325 = getelementptr inbounds nuw i8, ptr %_159.0.i.i804, i32 68, !dbg !15118
  %_42.i.i805.5 = load i32, ptr %325, align 4, !dbg !15118, !noalias !15146, !noundef !10
  %326 = add i32 %_42.i.i805.5, %ring_cursor.sroa.0.1.i7459208, !dbg !15147
  %_45.not.i.i806.5 = icmp ult i32 %326, %_60.i770, !dbg !15148
  %327 = select i1 %_45.not.i.i806.5, i32 0, i32 %_60.i770, !dbg !15148
  %spec.select.i.i807.5 = sub nuw i32 %326, %327, !dbg !15148
  %_49.i.i808.5 = mul i32 %spec.select.i.i807.5, %width.i.i772, !dbg !15149
  %_48.i.i809.5 = add i32 %_49.i.i808.5, 5, !dbg !15149
  %_51.i.i811.5 = icmp ult i32 %_48.i.i809.5, %_163.1.i.i821.pre, !dbg !15150
  br i1 %_51.i.i811.5, label %bb18.i.i813.5, label %panic1.i.i812, !dbg !15150

bb18.i.i813.5:                                    ; preds = %bb14.i.i803.5
  %328 = getelementptr inbounds nuw float, ptr %_161.0.i.i814, i32 %_48.i.i809.5, !dbg !15150
  %_47.i.i815.5 = load float, ptr %328, align 4, !dbg !15150, !noalias !15146, !noundef !10
  store float %_47.i.i815.5, ptr %iter.sroa.0.0.ptr.i.i7949205.5, align 4, !dbg !15151, !alias.scope !14973, !noalias !15152
  %329 = icmp eq i32 %width.i.i772, 6, !dbg !15115
  br i1 %329, label %bb53.i.i816.loopexit, label %bb36.i.i795.6, !dbg !15115

bb36.i.i795.6:                                    ; preds = %bb18.i.i813.5
  %exitcond11129.6.not = icmp eq i32 %_159.1.i.i800, 6, !dbg !15118
  br i1 %exitcond11129.6.not, label %panic.i.i802, label %bb14.i.i803.6, !dbg !15118

bb14.i.i803.6:                                    ; preds = %bb36.i.i795.6
  %330 = getelementptr inbounds nuw i8, ptr %_159.0.i.i804, i32 80, !dbg !15118
  %_42.i.i805.6 = load i32, ptr %330, align 4, !dbg !15118, !noalias !15146, !noundef !10
  %331 = add i32 %_42.i.i805.6, %ring_cursor.sroa.0.1.i7459208, !dbg !15147
  %_45.not.i.i806.6 = icmp ult i32 %331, %_60.i770, !dbg !15148
  %332 = select i1 %_45.not.i.i806.6, i32 0, i32 %_60.i770, !dbg !15148
  %spec.select.i.i807.6 = sub nuw i32 %331, %332, !dbg !15148
  %_49.i.i808.6 = mul i32 %spec.select.i.i807.6, %width.i.i772, !dbg !15149
  %_48.i.i809.6 = add i32 %_49.i.i808.6, 6, !dbg !15149
  %_51.i.i811.6 = icmp ult i32 %_48.i.i809.6, %_163.1.i.i821.pre, !dbg !15150
  br i1 %_51.i.i811.6, label %bb18.i.i813.6, label %panic1.i.i812, !dbg !15150

bb18.i.i813.6:                                    ; preds = %bb14.i.i803.6
  %333 = getelementptr inbounds nuw float, ptr %_161.0.i.i814, i32 %_48.i.i809.6, !dbg !15150
  %_47.i.i815.6 = load float, ptr %333, align 4, !dbg !15150, !noalias !15146, !noundef !10
  store float %_47.i.i815.6, ptr %iter.sroa.0.0.ptr.i.i7949205.6, align 4, !dbg !15151, !alias.scope !14973, !noalias !15152
  %334 = icmp eq i32 %width.i.i772, 7, !dbg !15115
  br i1 %334, label %bb53.i.i816.loopexit, label %bb36.i.i795.7, !dbg !15115

bb36.i.i795.7:                                    ; preds = %bb18.i.i813.6
  %exitcond11129.7.not = icmp eq i32 %_159.1.i.i800, 7, !dbg !15118
  br i1 %exitcond11129.7.not, label %panic.i.i802, label %bb14.i.i803.7, !dbg !15118

bb14.i.i803.7:                                    ; preds = %bb36.i.i795.7
  %335 = getelementptr inbounds nuw i8, ptr %_159.0.i.i804, i32 92, !dbg !15118
  %_42.i.i805.7 = load i32, ptr %335, align 4, !dbg !15118, !noalias !15146, !noundef !10
  %336 = add i32 %_42.i.i805.7, %ring_cursor.sroa.0.1.i7459208, !dbg !15147
  %_45.not.i.i806.7 = icmp ult i32 %336, %_60.i770, !dbg !15148
  %337 = select i1 %_45.not.i.i806.7, i32 0, i32 %_60.i770, !dbg !15148
  %spec.select.i.i807.7 = sub nuw i32 %336, %337, !dbg !15148
  %_49.i.i808.7 = mul i32 %spec.select.i.i807.7, %width.i.i772, !dbg !15149
  %_48.i.i809.7 = add i32 %_49.i.i808.7, 7, !dbg !15149
  %_51.i.i811.7 = icmp ult i32 %_48.i.i809.7, %_163.1.i.i821.pre, !dbg !15150
  br i1 %_51.i.i811.7, label %bb18.i.i813.7, label %panic1.i.i812, !dbg !15150

bb18.i.i813.7:                                    ; preds = %bb14.i.i803.7
  %338 = getelementptr inbounds nuw float, ptr %_161.0.i.i814, i32 %_48.i.i809.7, !dbg !15150
  %_47.i.i815.7 = load float, ptr %338, align 4, !dbg !15150, !noalias !15146, !noundef !10
  store float %_47.i.i815.7, ptr %iter.sroa.0.0.ptr.i.i7949205.7, align 4, !dbg !15151, !alias.scope !14973, !noalias !15152
  br label %bb53.i.i816.loopexit, !dbg !15115

panic1.i.i812:                                    ; preds = %bb14.i.i803.7, %bb14.i.i803.6, %bb14.i.i803.5, %bb14.i.i803.4, %bb14.i.i803.3, %bb14.i.i803.2, %bb14.i.i803.1, %bb14.i.i803
  %_48.i.i809.lcssa.ph = phi i32 [ %_48.i.i809.7, %bb14.i.i803.7 ], [ %_48.i.i809.6, %bb14.i.i803.6 ], [ %_48.i.i809.5, %bb14.i.i803.5 ], [ %_48.i.i809.4, %bb14.i.i803.4 ], [ %_48.i.i809.3, %bb14.i.i803.3 ], [ %_48.i.i809.2, %bb14.i.i803.2 ], [ %_48.i.i809.1, %bb14.i.i803.1 ], [ %_49.i.i808, %bb14.i.i803 ]
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i809.lcssa.ph, i32 noundef %_163.1.i.i821.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !15150, !noalias !15146
  unreachable, !dbg !15150

bb42.i.i823:                                      ; preds = %bb53.i.i816
  %_126.i.i825 = sub nuw i32 %_163.1.i.i821.pre, %_22.i.i778, !dbg !15153
  %_8.i3402 = icmp samesign ugt i32 %_126.i.i825, 3, !dbg !15154
  br i1 %_8.i3402, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3405, label %bb2.i3403, !dbg !15154, !prof !1153

bb2.i3403:                                        ; preds = %bb42.i.i823
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i.i825, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !15159, !noalias !15160
  unreachable, !dbg !15159

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3405: ; preds = %bb42.i.i823
  %_163.0.i.i824 = load ptr, ptr %112, align 4, !dbg !15117, !alias.scope !14977, !noalias !14978, !nonnull !10, !noundef !10
  %_130.i.i826 = getelementptr inbounds nuw float, ptr %_163.0.i.i824, i32 %_22.i.i778, !dbg !15164
  store <4 x float> %296, ptr %_130.i.i826, align 4, !dbg !15166, !alias.scope !15170, !noalias !15174
  %_66.i.i8318042 = load <4 x float>, ptr %115, align 16, !dbg !15176, !alias.scope !14967, !noalias !15177
  %339 = fdiv <4 x float> %299, %_62.i.i8288041, !dbg !15178
  %340 = fsub <4 x float> splat (float 1.000000e+00), %339, !dbg !15182
  %341 = fsub <4 x float> %340, %_66.i.i8318042, !dbg !15186
  %342 = bitcast <16 x i8> %_4.i3697 to <4 x float>, !dbg !15190
  %343 = fmul <4 x float> %341, %342, !dbg !15194
  %344 = fadd <4 x float> %_66.i.i8318042, %343, !dbg !15195
  %345 = fcmp olt <4 x float> %344, %340, !dbg !15198
  %346 = select <4 x i1> %345, <4 x float> %340, <4 x float> %344, !dbg !15202
  %347 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %346), !dbg !15203
  %348 = fcmp uge <4 x float> %347, splat (float 0x3BC79CA100000000), !dbg !15208
  %349 = bitcast <4 x float> %346 to <4 x i32>, !dbg !15213
  %350 = select <4 x i1> %348, <4 x i32> %349, <4 x i32> zeroinitializer, !dbg !15213
  store <4 x i32> %350, ptr %115, align 16, !dbg !15216, !alias.scope !14967, !noalias !15177
  %351 = bitcast <4 x i32> %350 to <4 x float>, !dbg !15217
  %352 = fsub <4 x float> splat (float 1.000000e+00), %351, !dbg !15221
  %_164.1.i.i841 = load i32, ptr %116, align 4, !dbg !15222, !alias.scope !14977, !noalias !14978, !noundef !10
  %_74.i.i842 = mul i32 %width.i.i772, %main_cursor.sroa.0.1.i7469209, !dbg !15223
  %_134.i.i843 = icmp ugt i32 %_74.i.i842, %_164.1.i.i841, !dbg !15224
  br i1 %_134.i.i843, label %bb47.i.i868, label %bb48.i.i844, !dbg !15224, !prof !902

bb41.i.i869:                                      ; preds = %bb53.i.i816
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i778, i32 noundef %_163.1.i.i821.pre, i32 noundef %_163.1.i.i821.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !15227, !noalias !15146
  unreachable, !dbg !15227

bb48.i.i844:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3405
  %_137.i.i846 = sub nuw i32 %_164.1.i.i841, %_74.i.i842, !dbg !15228
  %_8.i2986 = icmp samesign ugt i32 %_137.i.i846, 3, !dbg !15229
  br i1 %_8.i2986, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395, label %bb2.i2987, !dbg !15229, !prof !1153

bb2.i2987:                                        ; preds = %bb48.i.i844
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i.i846, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !15234, !noalias !15235
  unreachable, !dbg !15234

bb47.i.i868:                                      ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3405
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i842, i32 noundef %_164.1.i.i841, i32 noundef %_164.1.i.i841, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !15239, !noalias !15146
  unreachable, !dbg !15239

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3395: ; preds = %bb48.i.i844
  %_164.0.i.i845 = load ptr, ptr %117, align 4, !dbg !15222, !alias.scope !14977, !noalias !14978, !nonnull !10, !noundef !10
  %_141.i.i847 = getelementptr inbounds nuw float, ptr %_164.0.i.i845, i32 %_74.i.i842, !dbg !15240
  %lanes.i2983.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i.i847, align 4, !dbg !15242, !alias.scope !15246, !noalias !15250
  store <4 x i32> %lanes.i3006.sroa.0.0.copyload, ptr %_141.i.i847, align 4, !dbg !15252, !alias.scope !15257, !noalias !15261
  %353 = bitcast <4 x i32> %lanes.i2983.sroa.0.0.copyload to <4 x float>, !dbg !15265
  %354 = fmul <4 x float> %352, %353, !dbg !15269
  %355 = bitcast <4 x i32> %lanes.i2983.sroa.0.0.copyload to <16 x i8>, !dbg !15270
  %356 = bitcast <4 x float> %354 to <16 x i8>, !dbg !15274
  %_4.i3701 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %355, <16 x i8> %356, <16 x i8> %118), !dbg !15275
  store <16 x i8> %_4.i3701, ptr %_97.i768, align 4, !dbg !15276, !alias.scope !15281, !noalias !15285
  %357 = add i32 %main_cursor.sroa.0.1.i7469209, 1, !dbg !15289
  %_65.i862 = icmp eq i32 %357, %_67.i861, !dbg !15290
  %spec.store.select7.i863 = select i1 %_65.i862, i32 0, i32 %357, !dbg !15290
  %358 = add i32 %ring_cursor.sroa.0.1.i7459208, 1, !dbg !15291
  %_68.i864 = icmp eq i32 %358, %_60.i770, !dbg !15292
  %spec.store.select8.i865 = select i1 %_68.i864, i32 0, i32 %358, !dbg !15292
  %exitcond11133.not = icmp eq i32 %256, %umax11132, !dbg !15293
  br i1 %exitcond11133.not, label %bb12.i731.loopexit, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3022, !dbg !14805

bb38.i871:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3022
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212100, ptr %94, align 16
  store <4 x float> %.lcssa1175412120, ptr %97, align 16
  store <4 x float> %.lcssa1186012141, ptr %113, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i750, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8b3aff86bf6965bbce4da27800054d27) #33, !dbg !15296, !noalias !15297
  unreachable, !dbg !15296

_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit: ; preds = %bb12.i731.loopexit
  store <4 x i32> %history.i.i710.sroa.0.0.lcssa, ptr %hot_left.i716, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.7.0.lcssa, ptr %history.i.i710.sroa.7.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.10.0.lcssa, ptr %history.i.i710.sroa.10.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.13.0.lcssa, ptr %history.i.i710.sroa.13.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.16.0.lcssa, ptr %history.i.i710.sroa.16.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.19.0.lcssa, ptr %history.i.i710.sroa.19.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.22.0.lcssa, ptr %history.i.i710.sroa.22.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.26.0.lcssa, ptr %history.i.i710.sroa.26.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.29.0.lcssa, ptr %history.i.i710.sroa.29.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.32.0.lcssa, ptr %history.i.i710.sroa.32.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.35.0.lcssa, ptr %history.i.i710.sroa.35.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x i32> %history.i.i710.sroa.38.0.lcssa, ptr %history.i.i710.sroa.38.0.hot_left.i716.sroa_idx, align 16, !dbg !14355
  store <4 x float> %.lcssa1177212099, ptr %94, align 16
  store <4 x float> %.lcssa1175412119, ptr %97, align 16
  store <4 x float> %.lcssa1186012140, ptr %113, align 16
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !15298

_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, %bb8.i
  %ring_cursor.sroa.0.0.i734.lcssa = phi i32 [ %_27.i725, %bb8.i ], [ %ring_cursor.sroa.0.1.i745.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !14297
  %main_cursor.sroa.0.0.i735.lcssa = phi i32 [ %_25.i724, %bb8.i ], [ %main_cursor.sroa.0.1.i746.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !14294
  call void @llvm.lifetime.start.p0(ptr nonnull %_71.i711), !dbg !15298, !noalias !14282
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 16 dereferenceable(368) %_71.i711, ptr noundef nonnull align 16 dereferenceable(368) %hot_left.i716, i32 368, i1 false), !dbg !15298, !noalias !14282
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %_71.i711, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24) #32, !dbg !15299, !noalias !15297
  call void @llvm.lifetime.end.p0(ptr nonnull %_71.i711), !dbg !15300, !noalias !14282
  store i32 %main_cursor.sroa.0.0.i735.lcssa, ptr %_25, align 4, !dbg !15301, !alias.scope !14276, !noalias !14296
  store i32 %ring_cursor.sroa.0.0.i734.lcssa, ptr %56, align 4, !dbg !15302, !alias.scope !14276, !noalias !14296
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i713), !dbg !15303, !noalias !14282
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i714), !dbg !15304, !noalias !14282
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i716), !dbg !15305, !noalias !14282
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !14273

bb4.i:                                            ; preds = %bb1.i3.i, %bb2.i3640, %bb2.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15306), !dbg !15309
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15310), !dbg !15309
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15312), !dbg !15309
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15314), !dbg !15309
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i36, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #32, !dbg !15316
  %359 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !15320
  %360 = load i8, ptr %359, align 16, !dbg !15320, !range !4765, !alias.scope !15306, !noalias !15324, !noundef !10
  %361 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !15326
  %362 = load i8, ptr %361, align 1, !dbg !15326, !range !4765, !alias.scope !15306, !noalias !15324, !noundef !10
  %363 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !15328
  %ring.i44 = load i32, ptr %363, align 4, !dbg !15328, !alias.scope !15310, !noalias !15330, !noundef !10
  %364 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !15331
  %main.i45 = load i32, ptr %364, align 4, !dbg !15331, !alias.scope !15310, !noalias !15330, !noundef !10
  %_26.i46 = load i32, ptr %_25, align 4, !dbg !15333, !alias.scope !15314, !noalias !15335, !noundef !10
  %365 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !15336
  %_27.i47 = load i32, ptr %365, align 4, !dbg !15336, !alias.scope !15314, !noalias !15335, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i34), !dbg !15338, !noalias !15340
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i34, i8 0, i32 1024, i1 false), !noalias !15340
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i33), !dbg !15341, !noalias !15340
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_left.i33, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24, i32 %ring.i44, i32 %main.i45) #32, !dbg !15343
  %_106.not.i569575 = icmp eq i32 %frames, 0, !dbg !15344
  br i1 %_106.not.i569575, label %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge, label %bb32.i57.lr.ph, !dbg !15344

bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge: ; preds = %bb4.i
  %.phi.trans.insert11245 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 16
  %left_phase.i414.pre = load i32, ptr %.phi.trans.insert11245, align 16, !dbg !15354, !noalias !15340
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !15344

bb32.i57.lr.ph:                                   ; preds = %bb4.i
  %_23.i42 = trunc nuw i8 %362 to i1, !dbg !15326
  %spec.store.select17.i43 = select i1 %_23.i42, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !15326
  %_22.i40 = trunc nuw i8 %360 to i1, !dbg !15320
  %link.sroa.0.0.i41 = select i1 %_22.i40, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !15320
  %d9.i3702 = lshr i32 %frames, 5, !dbg !15355
  %r2.i3703 = and i32 %frames, 31, !dbg !15362
  %_19.not.i3704 = icmp ne i32 %r2.i3703, 0, !dbg !15363
  %366 = zext i1 %_19.not.i3704 to i32, !dbg !15363
  %yield_count.sroa.0.0.i3705 = add nuw nsw i32 %d9.i3702, %366, !dbg !15363
  %history.i.i20.sroa.7.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 16
  %history.i.i20.sroa.10.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 32
  %history.i.i20.sroa.13.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 48
  %history.i.i20.sroa.16.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 64
  %history.i.i20.sroa.19.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 80
  %history.i.i20.sroa.22.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 96
  %history.i.i20.sroa.26.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 112
  %history.i.i20.sroa.29.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 128
  %history.i.i20.sroa.32.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 144
  %history.i.i20.sroa.35.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 160
  %history.i.i20.sroa.38.0.hot_left.i36.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 176
  %367 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %368 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %369 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i.i242 = getelementptr inbounds nuw i8, ptr %self, i32 80
  %370 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %371 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %372 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i.i256 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %373 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %374 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %375 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i.i270 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %376 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %377 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %378 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i.i284 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %379 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %380 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %381 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i.i298 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %382 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %383 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %384 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i.i312 = getelementptr inbounds nuw i8, ptr %self, i32 400
  %385 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %386 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %387 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i.i326 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %388 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %389 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %390 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i.i340 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %391 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %392 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %393 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i.i354 = getelementptr inbounds nuw i8, ptr %self, i32 592
  %394 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %395 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %396 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i.i368 = getelementptr inbounds nuw i8, ptr %self, i32 656
  %397 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %398 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %399 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i.i382 = getelementptr inbounds nuw i8, ptr %self, i32 720
  %400 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %401 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %402 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %403 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 20
  %_50.i31.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 24
  %_50.i31.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 28
  %_78.i102 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 192
  %_79.i103 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 256
  %404 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 240
  %405 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 224
  %406 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 208
  %407 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 304
  %408 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 288
  %409 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 272
  %410 = bitcast <4 x i32> %link.sroa.0.0.i41 to <16 x i8>
  %411 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 32
  %412 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 36
  %_22.i.i138 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 16
  %413 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 40
  %414 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 44
  %415 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 336
  %416 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 352
  %417 = getelementptr inbounds nuw i8, ptr %hot_left.i36, i32 320
  %418 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 52
  %419 = getelementptr inbounds nuw i8, ptr %uniform_left.i33, i32 48
  %420 = bitcast <4 x i32> %spec.store.select17.i43 to <16 x i8>
  %_22.i.i138.promoted = load i32, ptr %_22.i.i138, align 4
  %history.i.i20.sroa.0.0.copyload.pre = load <4 x i32>, ptr %hot_left.i36, align 16, !dbg !15364
  %history.i.i20.sroa.7.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.7.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.10.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.10.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.13.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.13.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.16.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.16.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.19.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.19.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.22.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.22.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.26.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.26.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.29.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.29.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.32.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.32.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.35.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.35.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %history.i.i20.sroa.38.0.copyload.pre = load <4 x i32>, ptr %history.i.i20.sroa.38.0.hot_left.i36.sroa_idx, align 16, !dbg !15364
  %_50.i31.sroa.3.0.copyload = load i32, ptr %_50.i31.sroa.3.0..sroa_idx, align 4
  %_50.i31.sroa.4.0.copyload = load i32, ptr %_50.i31.sroa.4.0..sroa_idx, align 4
  %_54.0.i.i126 = load ptr, ptr %411, align 16, !nonnull !10, !align !10189
  %_54.1.i.i127 = load i32, ptr %412, align 4
  %_18.i19.i = load i32, ptr %403, align 4
  %_29.i.i1959482.not = icmp eq i32 %_18.i19.i, 0
  %_56.0.i.i149 = load ptr, ptr %413, align 8, !nonnull !10, !align !10189
  %_56.1.i.i150 = load i32, ptr %414, align 4
  %_58.1.i.i178 = load i32, ptr %418, align 4
  %_58.0.i.i177 = load ptr, ptr %419, align 16, !nonnull !10, !align !10189
  br label %bb32.i57, !dbg !15344

bb13.i51.loopexit.loopexit:                       ; preds = %bb48.i202
  store <4 x float> %.lcssa1144612419, ptr %404, align 16
  store <4 x float> %.lcssa1143812429, ptr %407, align 16
  store <4 x float> %.lcssa1146112439, ptr %415, align 16
  br label %bb13.i51.loopexit, !dbg !15344

bb13.i51.loopexit:                                ; preds = %bb13.i51.loopexit.loopexit, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i62
  %storemerge.i.i144.lcssa95309556.lcssa9582 = phi i32 [ %storemerge.i.i144.lcssa95309556.lcssa9583, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i62 ], [ %storemerge.i.i144.lcssa95309556, %bb13.i51.loopexit.loopexit ]
  %ring_cursor.sroa.0.1.i64.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i529576, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i62 ], [ %ring_cursor.sroa.0.2.i205, %bb13.i51.loopexit.loopexit ], !dbg !15368
  %main_cursor.sroa.0.1.i65.lcssa = phi i32 [ %main_cursor.sroa.0.0.i539577, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i62 ], [ %main_cursor.sroa.0.2.i208, %bb13.i51.loopexit.loopexit ], !dbg !15369
  %_106.not.i56 = icmp eq i32 %422, 0, !dbg !15344
  %indvars.iv.next11152 = add i32 %indvars.iv11151, -32, !dbg !15344
  br i1 %_106.not.i56, label %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb32.i57, !dbg !15344

bb32.i57:                                         ; preds = %bb32.i57.lr.ph, %bb13.i51.loopexit
  %history.i.i20.sroa.38.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.38.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.38.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.35.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.35.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.35.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.32.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.32.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.32.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.29.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.29.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.29.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.26.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.26.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.26.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.22.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.22.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.22.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.19.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.19.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.19.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.16.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.16.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.16.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.13.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.13.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.13.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.10.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.10.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.10.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.7.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.7.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.7.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %history.i.i20.sroa.0.0.copyload = phi <4 x i32> [ %history.i.i20.sroa.0.0.copyload.pre, %bb32.i57.lr.ph ], [ %history.i.i20.sroa.0.0.lcssa, %bb13.i51.loopexit ], !dbg !15364
  %indvars.iv11151 = phi i32 [ %frames, %bb32.i57.lr.ph ], [ %indvars.iv.next11152, %bb13.i51.loopexit ]
  %storemerge.i.i144.lcssa95309556.lcssa9583 = phi i32 [ %_22.i.i138.promoted, %bb32.i57.lr.ph ], [ %storemerge.i.i144.lcssa95309556.lcssa9582, %bb13.i51.loopexit ]
  %iter2.sroa.0.0.i559579 = phi i32 [ %yield_count.sroa.0.0.i3705, %bb32.i57.lr.ph ], [ %422, %bb13.i51.loopexit ]
  %iter1.sroa.0.0.i549578 = phi i32 [ 0, %bb32.i57.lr.ph ], [ %421, %bb13.i51.loopexit ]
  %main_cursor.sroa.0.0.i539577 = phi i32 [ %_26.i46, %bb32.i57.lr.ph ], [ %main_cursor.sroa.0.1.i65.lcssa, %bb13.i51.loopexit ]
  %ring_cursor.sroa.0.0.i529576 = phi i32 [ %_27.i47, %bb32.i57.lr.ph ], [ %ring_cursor.sroa.0.1.i64.lcssa, %bb13.i51.loopexit ]
  %umin11167 = call i32 @llvm.umin.i32(i32 %indvars.iv11151, i32 32), !dbg !15370
  %umax11154 = call i32 @llvm.umax.i32(i32 %umin11167, i32 1), !dbg !15370
  %421 = add i32 %iter1.sroa.0.0.i549578, 32, !dbg !15370
  %422 = add nsw i32 %iter2.sroa.0.0.i559579, -1, !dbg !15374
  %423 = sub i32 %frames, %iter1.sroa.0.0.i549578, !dbg !15375
  %spec.store.select.i58 = tail call i32 @llvm.umin.i32(i32 %423, i32 32), !dbg !15376
  %_20.i.i619457.not = icmp eq i32 %frames, %iter1.sroa.0.0.i549578, !dbg !15381
  br i1 %_20.i.i619457.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i62, label %bb5.i.i210.lr.ph, !dbg !15385

bb5.i.i210.lr.ph:                                 ; preds = %bb32.i57
  %_11.i.i.i.i2308192 = load <4 x float>, ptr %_22, align 16
  %_14.i.i.i.i2338193 = load <4 x float>, ptr %367, align 16
  %_17.i.i.i.i2368194 = load <4 x float>, ptr %368, align 16
  %_20.i.i.i.i2398195 = load <4 x float>, ptr %369, align 16
  %_25.i.i.i.i2448196 = load <4 x float>, ptr %row1.i.i.i.i242, align 16
  %_28.i.i.i.i2478197 = load <4 x float>, ptr %370, align 16
  %_31.i.i.i.i2508198 = load <4 x float>, ptr %371, align 16
  %_34.i.i.i.i2538199 = load <4 x float>, ptr %372, align 16
  %_39.i.i.i.i2588200 = load <4 x float>, ptr %row3.i.i.i.i256, align 16
  %_42.i.i.i.i2618201 = load <4 x float>, ptr %373, align 16
  %_45.i.i.i.i2648202 = load <4 x float>, ptr %374, align 16
  %_48.i.i.i.i2678203 = load <4 x float>, ptr %375, align 16
  %_53.i.i.i.i2728204 = load <4 x float>, ptr %row5.i.i.i.i270, align 16
  %_56.i.i.i.i2758205 = load <4 x float>, ptr %376, align 16
  %_59.i.i.i.i2788206 = load <4 x float>, ptr %377, align 16
  %_62.i.i.i.i2818207 = load <4 x float>, ptr %378, align 16
  %_67.i.i.i.i2868208 = load <4 x float>, ptr %row7.i.i.i.i284, align 16
  %_70.i.i.i.i2898209 = load <4 x float>, ptr %379, align 16
  %_73.i.i.i.i2928210 = load <4 x float>, ptr %380, align 16
  %_76.i.i.i.i2958211 = load <4 x float>, ptr %381, align 16
  %_81.i.i.i.i3008212 = load <4 x float>, ptr %row9.i.i.i.i298, align 16
  %_84.i.i.i.i3038213 = load <4 x float>, ptr %382, align 16
  %_87.i.i.i.i3068214 = load <4 x float>, ptr %383, align 16
  %_90.i.i.i.i3098215 = load <4 x float>, ptr %384, align 16
  %_95.i.i.i.i3148216 = load <4 x float>, ptr %row11.i.i.i.i312, align 16
  %_98.i.i.i.i3178217 = load <4 x float>, ptr %385, align 16
  %_101.i.i.i.i3208218 = load <4 x float>, ptr %386, align 16
  %_104.i.i.i.i3238219 = load <4 x float>, ptr %387, align 16
  %_109.i.i.i.i3288220 = load <4 x float>, ptr %row13.i.i.i.i326, align 16
  %_112.i.i.i.i3318221 = load <4 x float>, ptr %388, align 16
  %_115.i.i.i.i3348222 = load <4 x float>, ptr %389, align 16
  %_118.i.i.i.i3378223 = load <4 x float>, ptr %390, align 16
  %_123.i.i.i.i3428224 = load <4 x float>, ptr %row15.i.i.i.i340, align 16
  %_126.i.i.i.i3458225 = load <4 x float>, ptr %391, align 16
  %_129.i.i.i.i3488226 = load <4 x float>, ptr %392, align 16
  %_132.i.i.i.i3518227 = load <4 x float>, ptr %393, align 16
  %_137.i.i.i.i3568228 = load <4 x float>, ptr %row17.i.i.i.i354, align 16
  %_140.i.i.i.i3598229 = load <4 x float>, ptr %394, align 16
  %_143.i.i.i.i3628230 = load <4 x float>, ptr %395, align 16
  %_146.i.i.i.i3658231 = load <4 x float>, ptr %396, align 16
  %_151.i.i.i.i3708232 = load <4 x float>, ptr %row19.i.i.i.i368, align 16
  %_154.i.i.i.i3738233 = load <4 x float>, ptr %397, align 16
  %_157.i.i.i.i3768234 = load <4 x float>, ptr %398, align 16
  %_160.i.i.i.i3798235 = load <4 x float>, ptr %399, align 16
  %_165.i.i.i.i3848236 = load <4 x float>, ptr %row21.i.i.i.i382, align 16
  %_168.i.i.i.i3878237 = load <4 x float>, ptr %400, align 16
  %_171.i.i.i.i3908238 = load <4 x float>, ptr %401, align 16
  %_174.i.i.i.i3938239 = load <4 x float>, ptr %402, align 16
  br label %bb5.i.i210, !dbg !15385

bb5.i.i210:                                       ; preds = %bb5.i.i210.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445
  %iter.sroa.0.0.i.i609469 = phi i32 [ 0, %bb5.i.i210.lr.ph ], [ %424, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.35.09468 = phi <4 x i32> [ %history.i.i20.sroa.35.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.32.09467, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.32.09467 = phi <4 x i32> [ %history.i.i20.sroa.32.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.29.09466, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.29.09466 = phi <4 x i32> [ %history.i.i20.sroa.29.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.26.09465, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.26.09465 = phi <4 x i32> [ %history.i.i20.sroa.26.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.22.09464, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.22.09464 = phi <4 x i32> [ %history.i.i20.sroa.22.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.19.09463, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.19.09463 = phi <4 x i32> [ %history.i.i20.sroa.19.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.16.09462, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.16.09462 = phi <4 x i32> [ %history.i.i20.sroa.16.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.13.09461, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.13.09461 = phi <4 x i32> [ %history.i.i20.sroa.13.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.10.09460, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.10.09460 = phi <4 x i32> [ %history.i.i20.sroa.10.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.7.09459, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.7.09459 = phi <4 x i32> [ %history.i.i20.sroa.7.0.copyload, %bb5.i.i210.lr.ph ], [ %history.i.i20.sroa.0.09458, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %history.i.i20.sroa.0.09458 = phi <4 x i32> [ %history.i.i20.sroa.0.0.copyload, %bb5.i.i210.lr.ph ], [ %lanes.i3074.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ]
  %424 = add nuw nsw i32 %iter.sroa.0.0.i.i609469, 1, !dbg !15386
  %_11.i18.i = add nuw nsw i32 %iter.sroa.0.0.i.i609469, %iter1.sroa.0.0.i549578, !dbg !15389
  %base.i.i211 = shl i32 %_11.i18.i, 2, !dbg !15389
  %_24.i.i212 = icmp ugt i32 %base.i.i211, %left_io.1, !dbg !15390
  br i1 %_24.i.i212, label %bb7.i.i412, label %bb8.i.i213, !dbg !15390, !prof !902

bb8.i.i213:                                       ; preds = %bb5.i.i210
  %_27.i.i214 = sub nuw nsw i32 %left_io.1, %base.i.i211, !dbg !15393
  %_8.i3077 = icmp samesign ugt i32 %_27.i.i214, 3, !dbg !15394
  br i1 %_8.i3077, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445, label %bb2.i3078, !dbg !15394, !prof !1153

bb2.i3078:                                        ; preds = %bb8.i.i213
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i.i214, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !15399, !noalias !15400
  unreachable, !dbg !15399

bb7.i.i412:                                       ; preds = %bb5.i.i210
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i.i211, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !15407, !noalias !15408
  unreachable, !dbg !15407

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445: ; preds = %bb8.i.i213
  %_31.i.i215 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i.i211, !dbg !15409
  %lanes.i3074.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i.i215, align 4, !dbg !15411, !alias.scope !15415, !noalias !15419
  %425 = bitcast <4 x i32> %history.i.i20.sroa.19.09463 to <4 x float>, !dbg !15421
  %426 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %425), !dbg !15426
  %427 = bitcast <4 x i32> %lanes.i3074.sroa.0.0.copyload to <4 x float>, !dbg !15427
  %428 = fmul <4 x float> %_11.i.i.i.i2308192, %427, !dbg !15432
  %429 = fadd <4 x float> %428, zeroinitializer, !dbg !15433
  %430 = fmul <4 x float> %_14.i.i.i.i2338193, %427, !dbg !15437
  %431 = fadd <4 x float> %430, zeroinitializer, !dbg !15441
  %432 = fmul <4 x float> %_17.i.i.i.i2368194, %427, !dbg !15445
  %433 = fadd <4 x float> %432, zeroinitializer, !dbg !15449
  %434 = fmul <4 x float> %_20.i.i.i.i2398195, %427, !dbg !15453
  %435 = fadd <4 x float> %434, zeroinitializer, !dbg !15457
  %436 = bitcast <4 x i32> %history.i.i20.sroa.0.09458 to <4 x float>, !dbg !15461
  %437 = fmul <4 x float> %_25.i.i.i.i2448196, %436, !dbg !15465
  %438 = fadd <4 x float> %429, %437, !dbg !15466
  %439 = fmul <4 x float> %_28.i.i.i.i2478197, %436, !dbg !15470
  %440 = fadd <4 x float> %431, %439, !dbg !15474
  %441 = fmul <4 x float> %_31.i.i.i.i2508198, %436, !dbg !15478
  %442 = fadd <4 x float> %433, %441, !dbg !15482
  %443 = fmul <4 x float> %_34.i.i.i.i2538199, %436, !dbg !15486
  %444 = fadd <4 x float> %435, %443, !dbg !15490
  %445 = bitcast <4 x i32> %history.i.i20.sroa.7.09459 to <4 x float>, !dbg !15494
  %446 = fmul <4 x float> %_39.i.i.i.i2588200, %445, !dbg !15498
  %447 = fadd <4 x float> %438, %446, !dbg !15499
  %448 = fmul <4 x float> %_42.i.i.i.i2618201, %445, !dbg !15503
  %449 = fadd <4 x float> %440, %448, !dbg !15507
  %450 = fmul <4 x float> %_45.i.i.i.i2648202, %445, !dbg !15511
  %451 = fadd <4 x float> %442, %450, !dbg !15515
  %452 = fmul <4 x float> %_48.i.i.i.i2678203, %445, !dbg !15519
  %453 = fadd <4 x float> %444, %452, !dbg !15523
  %454 = bitcast <4 x i32> %history.i.i20.sroa.10.09460 to <4 x float>, !dbg !15527
  %455 = fmul <4 x float> %_53.i.i.i.i2728204, %454, !dbg !15531
  %456 = fadd <4 x float> %447, %455, !dbg !15532
  %457 = fmul <4 x float> %_56.i.i.i.i2758205, %454, !dbg !15536
  %458 = fadd <4 x float> %449, %457, !dbg !15540
  %459 = fmul <4 x float> %_59.i.i.i.i2788206, %454, !dbg !15544
  %460 = fadd <4 x float> %451, %459, !dbg !15548
  %461 = fmul <4 x float> %_62.i.i.i.i2818207, %454, !dbg !15552
  %462 = fadd <4 x float> %453, %461, !dbg !15556
  %463 = bitcast <4 x i32> %history.i.i20.sroa.13.09461 to <4 x float>, !dbg !15560
  %464 = fmul <4 x float> %_67.i.i.i.i2868208, %463, !dbg !15564
  %465 = fadd <4 x float> %456, %464, !dbg !15565
  %466 = fmul <4 x float> %_70.i.i.i.i2898209, %463, !dbg !15569
  %467 = fadd <4 x float> %458, %466, !dbg !15573
  %468 = fmul <4 x float> %_73.i.i.i.i2928210, %463, !dbg !15577
  %469 = fadd <4 x float> %460, %468, !dbg !15581
  %470 = fmul <4 x float> %_76.i.i.i.i2958211, %463, !dbg !15585
  %471 = fadd <4 x float> %462, %470, !dbg !15589
  %472 = bitcast <4 x i32> %history.i.i20.sroa.16.09462 to <4 x float>, !dbg !15593
  %473 = fmul <4 x float> %_81.i.i.i.i3008212, %472, !dbg !15597
  %474 = fadd <4 x float> %465, %473, !dbg !15598
  %475 = fmul <4 x float> %_84.i.i.i.i3038213, %472, !dbg !15602
  %476 = fadd <4 x float> %467, %475, !dbg !15606
  %477 = fmul <4 x float> %_87.i.i.i.i3068214, %472, !dbg !15610
  %478 = fadd <4 x float> %469, %477, !dbg !15614
  %479 = fmul <4 x float> %_90.i.i.i.i3098215, %472, !dbg !15618
  %480 = fadd <4 x float> %471, %479, !dbg !15622
  %481 = fmul <4 x float> %_95.i.i.i.i3148216, %425, !dbg !15626
  %482 = fadd <4 x float> %474, %481, !dbg !15630
  %483 = fmul <4 x float> %_98.i.i.i.i3178217, %425, !dbg !15634
  %484 = fadd <4 x float> %476, %483, !dbg !15638
  %485 = fmul <4 x float> %_101.i.i.i.i3208218, %425, !dbg !15642
  %486 = fadd <4 x float> %478, %485, !dbg !15646
  %487 = fmul <4 x float> %_104.i.i.i.i3238219, %425, !dbg !15650
  %488 = fadd <4 x float> %480, %487, !dbg !15654
  %489 = bitcast <4 x i32> %history.i.i20.sroa.22.09464 to <4 x float>, !dbg !15658
  %490 = fmul <4 x float> %_109.i.i.i.i3288220, %489, !dbg !15662
  %491 = fadd <4 x float> %482, %490, !dbg !15663
  %492 = fmul <4 x float> %_112.i.i.i.i3318221, %489, !dbg !15667
  %493 = fadd <4 x float> %484, %492, !dbg !15671
  %494 = fmul <4 x float> %_115.i.i.i.i3348222, %489, !dbg !15675
  %495 = fadd <4 x float> %486, %494, !dbg !15679
  %496 = fmul <4 x float> %_118.i.i.i.i3378223, %489, !dbg !15683
  %497 = fadd <4 x float> %488, %496, !dbg !15687
  %498 = bitcast <4 x i32> %history.i.i20.sroa.26.09465 to <4 x float>, !dbg !15691
  %499 = fmul <4 x float> %_123.i.i.i.i3428224, %498, !dbg !15695
  %500 = fadd <4 x float> %491, %499, !dbg !15696
  %501 = fmul <4 x float> %_126.i.i.i.i3458225, %498, !dbg !15700
  %502 = fadd <4 x float> %493, %501, !dbg !15704
  %503 = fmul <4 x float> %_129.i.i.i.i3488226, %498, !dbg !15708
  %504 = fadd <4 x float> %495, %503, !dbg !15712
  %505 = fmul <4 x float> %_132.i.i.i.i3518227, %498, !dbg !15716
  %506 = fadd <4 x float> %497, %505, !dbg !15720
  %507 = bitcast <4 x i32> %history.i.i20.sroa.29.09466 to <4 x float>, !dbg !15724
  %508 = fmul <4 x float> %_137.i.i.i.i3568228, %507, !dbg !15728
  %509 = fadd <4 x float> %500, %508, !dbg !15729
  %510 = fmul <4 x float> %_140.i.i.i.i3598229, %507, !dbg !15733
  %511 = fadd <4 x float> %502, %510, !dbg !15737
  %512 = fmul <4 x float> %_143.i.i.i.i3628230, %507, !dbg !15741
  %513 = fadd <4 x float> %504, %512, !dbg !15745
  %514 = fmul <4 x float> %_146.i.i.i.i3658231, %507, !dbg !15749
  %515 = fadd <4 x float> %506, %514, !dbg !15753
  %516 = bitcast <4 x i32> %history.i.i20.sroa.32.09467 to <4 x float>, !dbg !15757
  %517 = fmul <4 x float> %_151.i.i.i.i3708232, %516, !dbg !15761
  %518 = fadd <4 x float> %509, %517, !dbg !15762
  %519 = fmul <4 x float> %_154.i.i.i.i3738233, %516, !dbg !15766
  %520 = fadd <4 x float> %511, %519, !dbg !15770
  %521 = fmul <4 x float> %_157.i.i.i.i3768234, %516, !dbg !15774
  %522 = fadd <4 x float> %513, %521, !dbg !15778
  %523 = fmul <4 x float> %_160.i.i.i.i3798235, %516, !dbg !15782
  %524 = fadd <4 x float> %515, %523, !dbg !15786
  %525 = bitcast <4 x i32> %history.i.i20.sroa.35.09468 to <4 x float>, !dbg !15790
  %526 = fmul <4 x float> %_165.i.i.i.i3848236, %525, !dbg !15794
  %527 = fadd <4 x float> %518, %526, !dbg !15795
  %528 = fmul <4 x float> %_168.i.i.i.i3878237, %525, !dbg !15799
  %529 = fadd <4 x float> %520, %528, !dbg !15803
  %530 = fmul <4 x float> %_171.i.i.i.i3908238, %525, !dbg !15807
  %531 = fadd <4 x float> %522, %530, !dbg !15811
  %532 = fmul <4 x float> %_174.i.i.i.i3938239, %525, !dbg !15815
  %533 = fadd <4 x float> %524, %532, !dbg !15819
  %534 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %527), !dbg !15823
  %535 = fcmp olt <4 x float> %534, %426, !dbg !15827
  %536 = select <4 x i1> %535, <4 x float> %426, <4 x float> %534, !dbg !15831
  %537 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %529), !dbg !15823
  %538 = fcmp olt <4 x float> %537, %536, !dbg !15827
  %539 = select <4 x i1> %538, <4 x float> %536, <4 x float> %537, !dbg !15831
  %540 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %531), !dbg !15823
  %541 = fcmp olt <4 x float> %540, %539, !dbg !15827
  %542 = select <4 x i1> %541, <4 x float> %539, <4 x float> %540, !dbg !15831
  %543 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %533), !dbg !15823
  %544 = fcmp olt <4 x float> %543, %542, !dbg !15827
  %545 = select <4 x i1> %544, <4 x float> %542, <4 x float> %543, !dbg !15831
  %_39.i.i406.idx = shl i32 %iter.sroa.0.0.i.i609469, 4, !dbg !15832
  %_39.i.i406 = getelementptr inbounds nuw i8, ptr %peaks_left.i34, i32 %_39.i.i406.idx, !dbg !15832
  store <4 x float> %545, ptr %_39.i.i406, align 4, !dbg !15837, !alias.scope !15842, !noalias !15846
  %exitcond11155.not = icmp eq i32 %424, %umax11154, !dbg !15381
  br i1 %exitcond11155.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i62, label %bb5.i.i210, !dbg !15385

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i62: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445, %bb32.i57
  %history.i.i20.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.0.0.copyload, %bb32.i57 ], [ %lanes.i3074.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.7.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.0.09458, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.10.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.7.09459, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.13.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.10.09460, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.16.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.13.09461, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.19.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.16.09462, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.22.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.19.09463, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.26.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.22.09464, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.29.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.26.09465, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.32.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.29.09466, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.35.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.32.09467, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  %history.i.i20.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i20.sroa.38.0.copyload, %bb32.i57 ], [ %history.i.i20.sroa.35.09468, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3445 ], !dbg !15850
  store <4 x i32> %history.i.i20.sroa.0.0.lcssa, ptr %hot_left.i36, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.7.0.lcssa, ptr %history.i.i20.sroa.7.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.10.0.lcssa, ptr %history.i.i20.sroa.10.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.13.0.lcssa, ptr %history.i.i20.sroa.13.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.16.0.lcssa, ptr %history.i.i20.sroa.16.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.19.0.lcssa, ptr %history.i.i20.sroa.19.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.22.0.lcssa, ptr %history.i.i20.sroa.22.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.26.0.lcssa, ptr %history.i.i20.sroa.26.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.29.0.lcssa, ptr %history.i.i20.sroa.29.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.32.0.lcssa, ptr %history.i.i20.sroa.32.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.35.0.lcssa, ptr %history.i.i20.sroa.35.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  store <4 x i32> %history.i.i20.sroa.38.0.lcssa, ptr %history.i.i20.sroa.38.0.hot_left.i36.sroa_idx, align 16, !dbg !15851
  br i1 %_20.i.i619457.not, label %bb13.i51.loopexit, label %bb17.i68.lr.ph, !dbg !15852

bb17.i68.lr.ph:                                   ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i62
  %_13.i14178171 = load <16 x i8>, ptr %406, align 16
  %_13.i14048175 = load <16 x i8>, ptr %409, align 16
  %_37.i.i1648183 = load <4 x float>, ptr %416, align 16
  %.promoted12418 = load <4 x float>, ptr %404, align 16
  %.promoted12428 = load <4 x float>, ptr %407, align 16
  %.promoted12438 = load <4 x float>, ptr %415, align 16
  br label %bb17.i68, !dbg !15852

bb17.i68:                                         ; preds = %bb17.i68.lr.ph, %bb48.i202
  %.lcssa1146112440 = phi <4 x float> [ %.promoted12438, %bb17.i68.lr.ph ], [ %.lcssa1146112439, %bb48.i202 ]
  %.lcssa1143812430 = phi <4 x float> [ %.promoted12428, %bb17.i68.lr.ph ], [ %.lcssa1143812429, %bb48.i202 ]
  %.lcssa1144612420 = phi <4 x float> [ %.promoted12418, %bb17.i68.lr.ph ], [ %.lcssa1144612419, %bb48.i202 ]
  %storemerge.i.i144.lcssa95309557 = phi i32 [ %storemerge.i.i144.lcssa95309556.lcssa9583, %bb17.i68.lr.ph ], [ %storemerge.i.i144.lcssa95309556, %bb48.i202 ]
  %frame.sroa.0.0.i669552 = phi i32 [ 0, %bb17.i68.lr.ph ], [ %_64.i81, %bb48.i202 ]
  %main_cursor.sroa.0.1.i659551 = phi i32 [ %main_cursor.sroa.0.0.i539577, %bb17.i68.lr.ph ], [ %main_cursor.sroa.0.2.i208, %bb48.i202 ]
  %ring_cursor.sroa.0.1.i649550 = phi i32 [ %ring_cursor.sroa.0.0.i529576, %bb17.i68.lr.ph ], [ %ring_cursor.sroa.0.2.i205, %bb48.i202 ]
  %_47.i69 = sub nuw nsw i32 %spec.store.select.i58, %frame.sroa.0.0.i669552, !dbg !15854
  %ring.i1318 = load i32, ptr %363, align 4, !dbg !15855, !alias.scope !15857, !noalias !15860, !noundef !10
  %main.i1319 = load i32, ptr %364, align 4, !dbg !15864, !alias.scope !15857, !noalias !15860, !noundef !10
  %_10.i = add i32 %ring_cursor.sroa.0.1.i649550, 1, !dbg !15865
  %_38.not.i = icmp ult i32 %_10.i, %ring.i1318, !dbg !15866
  %546 = select i1 %_38.not.i, i32 0, i32 %ring.i1318, !dbg !15866
  %start1.sroa.0.0.i1320 = sub nuw i32 %_10.i, %546, !dbg !15866
  %_12.i1321 = add i32 %_50.i31.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i649550, !dbg !15868
  %_39.not.i = icmp ult i32 %_12.i1321, %ring.i1318, !dbg !15869
  %547 = select i1 %_39.not.i, i32 0, i32 %ring.i1318, !dbg !15869
  %left_end.sroa.0.0.i = sub nuw i32 %_12.i1321, %547, !dbg !15869
  %_18.i1325 = add i32 %_50.i31.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i649550, !dbg !15871
  %_41.not.i = icmp ult i32 %_18.i1325, %ring.i1318, !dbg !15872
  %548 = select i1 %_41.not.i, i32 0, i32 %ring.i1318, !dbg !15872
  %left_expiring.sroa.0.0.i = sub nuw i32 %_18.i1325, %548, !dbg !15872
  %549 = sub i32 %ring.i1318, %ring_cursor.sroa.0.1.i649550, !dbg !15874
  %spec.store.select.i1327 = tail call i32 @llvm.umin.i32(i32 %549, i32 %_47.i69), !dbg !15875
  %550 = sub i32 %main.i1319, %main_cursor.sroa.0.1.i659551, !dbg !15877
  %_24.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %550, i32 %spec.store.select.i1327), !dbg !15878
  %551 = sub i32 %ring.i1318, %start1.sroa.0.0.i1320, !dbg !15880
  %_25.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %551, i32 %_24.sroa.0.0.i), !dbg !15881
  %552 = sub i32 %ring.i1318, %left_end.sroa.0.0.i, !dbg !15883
  %_27.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %552, i32 %_25.sroa.0.0.i), !dbg !15884
  %553 = sub i32 %ring.i1318, %left_expiring.sroa.0.0.i, !dbg !15886
  %_31.sroa.0.0.i = tail call i32 @llvm.umin.i32(i32 %553, i32 %_27.sroa.0.0.i), !dbg !15887
  %_54.i71 = add i32 %frame.sroa.0.0.i669552, %iter1.sroa.0.0.i549578, !dbg !15889
  %base.i72 = shl i32 %_54.i71, 2, !dbg !15889
  %base.i728167 = add i32 %_31.sroa.0.0.i, %_54.i71, !dbg !15892
  %_58.i74 = shl i32 %base.i728167, 2, !dbg !15892
  %_116.i75 = icmp ult i32 %_58.i74, %base.i72, !dbg !15895
  %_110.not.i76 = icmp ugt i32 %_58.i74, %left_io.1
  %or.cond.i77 = or i1 %_116.i75, %_110.not.i76, !dbg !15895
  br i1 %or.cond.i77, label %bb39.i209, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !15895, !prof !4694

bb39.i209:                                        ; preds = %bb17.i68
  store <4 x float> %.lcssa1144612420, ptr %404, align 16
  store <4 x float> %.lcssa1143812430, ptr %407, align 16
  store <4 x float> %.lcssa1146112440, ptr %415, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i72, i32 noundef %_58.i74, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_192e368852d87f0f082761c8ef71c730) #33, !dbg !15902, !noalias !15903
  unreachable, !dbg !15902

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb17.i68
  %_119.i79 = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i72, !dbg !15904
  %_64.i81 = add nuw nsw i32 %_31.sroa.0.0.i, %frame.sroa.0.0.i669552, !dbg !15908
  %_128.i89.idx = shl nuw nsw i32 %frame.sroa.0.0.i669552, 4, !dbg !15910
  %_128.i89 = getelementptr inbounds nuw i8, ptr %peaks_left.i34, i32 %_128.i89.idx, !dbg !15910
  %_2.i37239485.not = icmp eq i32 %_31.sroa.0.0.i, 0, !dbg !15920
  br i1 %_2.i37239485.not, label %bb48.i202, label %bb49.i93.lr.ph, !dbg !15920

bb49.i93.lr.ph:                                   ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %umin11163 = call i32 @llvm.umin.i32(i32 %552, i32 %553), !dbg !15920
  %umin11164 = call i32 @llvm.umin.i32(i32 %umin11163, i32 %551), !dbg !15920
  %umin11165 = call i32 @llvm.umin.i32(i32 %umin11164, i32 %549), !dbg !15920
  %umin11166 = call i32 @llvm.umin.i32(i32 %umin11165, i32 %550), !dbg !15920
  %554 = sub nsw i32 %umin11167, %frame.sroa.0.0.i669552, !dbg !15920
  %umin11168 = call i32 @llvm.umin.i32(i32 %umin11166, i32 %554), !dbg !15920
  %555 = and i32 %umin11168, 1073741823, !dbg !15920
  %_11.i14148170.pre = load <4 x float>, ptr %_78.i102, align 16, !dbg !15930
  %_12.i1415.pre = load <4 x i32>, ptr %405, align 16, !dbg !15934
  %_11.i14018174.pre = load <4 x float>, ptr %_79.i103, align 16, !dbg !15935
  %_12.i1402.pre = load <4 x i32>, ptr %408, align 16, !dbg !15937
  br label %bb49.i93, !dbg !15920

bb49.i93:                                         ; preds = %bb49.i93.lr.ph, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097
  %_12.i1402 = phi <4 x i32> [ %_12.i1402.pre, %bb49.i93.lr.ph ], [ %582, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ], !dbg !15937
  %_11.i14018174 = phi <4 x float> [ %_11.i14018174.pre, %bb49.i93.lr.ph ], [ %581, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ], !dbg !15935
  %_12.i1415 = phi <4 x i32> [ %_12.i1415.pre, %bb49.i93.lr.ph ], [ %580, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ], !dbg !15934
  %_11.i14148170 = phi <4 x float> [ %_11.i14148170.pre, %bb49.i93.lr.ph ], [ %579, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ], !dbg !15930
  %556 = phi <4 x float> [ %.lcssa1146112440, %bb49.i93.lr.ph ], [ %614, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ]
  %storemerge.i.i1449520 = phi i32 [ %storemerge.i.i144.lcssa95309557, %bb49.i93.lr.ph ], [ %storemerge.i.i144, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ]
  %557 = phi <4 x float> [ %.lcssa1143812430, %bb49.i93.lr.ph ], [ %577, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ]
  %558 = phi <4 x float> [ %.lcssa1144612420, %bb49.i93.lr.ph ], [ %576, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ]
  %iter.i25.sroa.16.09487 = phi i32 [ 0, %bb49.i93.lr.ph ], [ %575, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097 ]
  %559 = fadd <4 x float> %558, splat (float -1.000000e+00), !dbg !15938
  %560 = fcmp ogt <4 x float> %559, zeroinitializer, !dbg !15942
  %561 = sext <4 x i1> %560 to <4 x i32>, !dbg !15946
  %562 = bitcast <4 x i32> %_12.i1415 to <4 x float>, !dbg !15951
  %563 = fadd <4 x float> %_11.i14148170, %562, !dbg !15955
  %564 = bitcast <4 x float> %563 to <16 x i8>, !dbg !15956
  %565 = bitcast <4 x i32> %561 to <16 x i8>, !dbg !15960
  %_4.i3727 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %564, <16 x i8> %_13.i14178171, <16 x i8> %565), !dbg !15961
  %566 = bitcast <4 x i32> %_12.i1415 to <16 x i8>, !dbg !15962
  %_4.i3728 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %566, <16 x i8> zeroinitializer, <16 x i8> %565), !dbg !15966
  %567 = fadd <4 x float> %557, splat (float -1.000000e+00), !dbg !15967
  %568 = fcmp ogt <4 x float> %567, zeroinitializer, !dbg !15971
  %569 = sext <4 x i1> %568 to <4 x i32>, !dbg !15975
  %570 = bitcast <4 x i32> %_12.i1402 to <4 x float>, !dbg !15980
  %571 = fadd <4 x float> %_11.i14018174, %570, !dbg !15984
  %572 = bitcast <4 x float> %571 to <16 x i8>, !dbg !15985
  %573 = bitcast <4 x i32> %569 to <16 x i8>, !dbg !15989
  %_4.i3729 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %572, <16 x i8> %_13.i14048175, <16 x i8> %573), !dbg !15990
  %574 = bitcast <4 x i32> %_12.i1402 to <16 x i8>, !dbg !15991
  %_4.i3730 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %574, <16 x i8> zeroinitializer, <16 x i8> %573), !dbg !15995
  %start1.i.i = shl i32 %iter.i25.sroa.16.09487, 2, !dbg !15996
  %data.i.i3725 = getelementptr inbounds nuw float, ptr %_119.i79, i32 %start1.i.i, !dbg !15999
  %lanes.i3083.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i3725, align 4, !dbg !16001, !alias.scope !16010, !noalias !16014
  %575 = add nuw nsw i32 %iter.i25.sroa.16.09487, 1, !dbg !16018
  %576 = select <4 x i1> %560, <4 x float> %559, <4 x float> zeroinitializer, !dbg !16019
  %577 = select <4 x i1> %568, <4 x float> %567, <4 x float> zeroinitializer, !dbg !16020
  %_140.i116 = add i32 %iter.i25.sroa.16.09487, %ring_cursor.sroa.0.1.i649550, !dbg !16021
  %_141.i117 = add i32 %iter.i25.sroa.16.09487, %main_cursor.sroa.0.1.i659551, !dbg !16025
  %_142.i118 = add i32 %iter.i25.sroa.16.09487, %left_end.sroa.0.0.i, !dbg !16026
  %_143.i119 = add i32 %iter.i25.sroa.16.09487, %start1.sroa.0.0.i1320, !dbg !16027
  %_144.i120 = add i32 %iter.i25.sroa.16.09487, %left_expiring.sroa.0.0.i, !dbg !16028
  %base.i9.i.i129 = shl i32 %_140.i116, 2, !dbg !16029
  %_7.i10.i.i130 = add i32 %base.i9.i.i129, 4, !dbg !16032
  %578 = or disjoint i32 %base.i9.i.i129, 3, !dbg !16033
  %or.cond.i13.i.i133.not = icmp ult i32 %578, %_54.1.i.i127, !dbg !16033
  %579 = bitcast <16 x i8> %_4.i3727 to <4 x float>, !dbg !16033
  %580 = bitcast <16 x i8> %_4.i3728 to <4 x i32>, !dbg !16033
  %581 = bitcast <16 x i8> %_4.i3729 to <4 x float>, !dbg !16033
  %582 = bitcast <16 x i8> %_4.i3730 to <4 x i32>, !dbg !16033
  br i1 %or.cond.i13.i.i133.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i134, label %bb4.i15.i.i201, !dbg !16033, !prof !10587

bb4.i15.i.i201:                                   ; preds = %bb49.i93
  store <4 x float> %.lcssa1144612420, ptr %404, align 16
  store <4 x float> %.lcssa1143812430, ptr %407, align 16
  store <4 x float> %.lcssa1146112440, ptr %415, align 16
  store <16 x i8> %_4.i3727, ptr %_78.i102, align 16, !dbg !16037
  store <16 x i8> %_4.i3728, ptr %405, align 16, !dbg !16038
  store <16 x i8> %_4.i3729, ptr %_79.i103, align 16, !dbg !16039
  store <16 x i8> %_4.i3730, ptr %408, align 16, !dbg !16040
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i.i129, i32 noundef %_7.i10.i.i130, i32 noundef range(i32 0, 536870912) %_54.1.i.i127, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !16041, !noalias !16042
  unreachable, !dbg !16041

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i134: ; preds = %bb49.i93
  %data.i4.i = getelementptr inbounds nuw float, ptr %_128.i89, i32 %start1.i.i, !dbg !16056
  %lanes.i3092.sroa.0.0.copyload = load <16 x i8>, ptr %data.i4.i, align 4, !dbg !16059, !alias.scope !16064, !noalias !16068
  %_4.i3731 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %lanes.i3092.sroa.0.0.copyload, <16 x i8> %lanes.i3092.sroa.0.0.copyload, <16 x i8> %410), !dbg !16072
  %583 = bitcast <16 x i8> %_4.i3727 to <4 x float>, !dbg !16076
  %584 = bitcast <16 x i8> %_4.i3731 to <4 x float>, !dbg !16081
  %585 = fdiv <4 x float> %583, %584, !dbg !16082
  %586 = bitcast <4 x float> %585 to <16 x i8>, !dbg !16086
  %587 = fcmp ogt <4 x float> %584, %583, !dbg !16090
  %588 = sext <4 x i1> %587 to <4 x i32>, !dbg !16090
  %589 = bitcast <4 x i32> %588 to <16 x i8>, !dbg !16091
  %_4.i3732 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %586, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %589), !dbg !16092
  %_17.i14.i.i135 = getelementptr inbounds nuw float, ptr %_54.0.i.i126, i32 %base.i9.i.i129, !dbg !16093
  store <16 x i8> %_4.i3732, ptr %_17.i14.i.i135, align 4, !dbg !16095, !alias.scope !16100, !noalias !16104
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16108), !dbg !16111
  %base.i1116 = shl i32 %_142.i118, 2, !dbg !16112
  %590 = or disjoint i32 %base.i1116, 3, !dbg !16115
  %or.cond.i1120.not = icmp ult i32 %590, %_54.1.i.i127, !dbg !16115
  br i1 %or.cond.i1120.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1124, label %bb4.i1123, !dbg !16115, !prof !10587

bb4.i1123:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i134
  store <4 x float> %.lcssa1144612420, ptr %404, align 16
  store <4 x float> %.lcssa1143812430, ptr %407, align 16
  store <4 x float> %.lcssa1146112440, ptr %415, align 16
  store <16 x i8> %_4.i3727, ptr %_78.i102, align 16, !dbg !16037
  store <16 x i8> %_4.i3728, ptr %405, align 16, !dbg !16038
  store <16 x i8> %_4.i3729, ptr %_79.i103, align 16, !dbg !16039
  store <16 x i8> %_4.i3730, ptr %408, align 16, !dbg !16040
  %_5.i1117 = add i32 %base.i1116, 4, !dbg !16119
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1116, i32 noundef %_5.i1117, i32 noundef range(i32 0, 536870912) %_54.1.i.i127, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !16120, !noalias !16121
  unreachable, !dbg !16120

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1124: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i134
  %_15.i1122 = getelementptr inbounds nuw float, ptr %_54.0.i.i126, i32 %base.i1116, !dbg !16127
  %lanes.i2932.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1122, align 4, !dbg !16129
  %591 = icmp eq i32 %storemerge.i.i1449520, 0, !dbg !16134
  %_12.i.i1418177 = load <4 x float>, ptr %uniform_left.i33, align 16, !dbg !16134
  %592 = bitcast <4 x i32> %lanes.i2932.sroa.0.0.copyload to <4 x float>, !dbg !16134
  %593 = fcmp olt <4 x float> %_12.i.i1418177, %592, !dbg !16134
  %594 = select <4 x i1> %593, <4 x float> %_12.i.i1418177, <4 x float> %592, !dbg !16134
  %595 = bitcast <4 x float> %594 to <4 x i32>, !dbg !16134
  %.sroa.04770.0 = select i1 %591, <4 x i32> %lanes.i2932.sroa.0.0.copyload, <4 x i32> %595, !dbg !16134
  store <4 x i32> %.sroa.04770.0, ptr %uniform_left.i33, align 16, !dbg !16135, !alias.scope !16108, !noalias !16136
  %_15.i31.i = add i32 %storemerge.i.i1449520, 1, !dbg !16138
  %complete.i.i142 = icmp eq i32 %_15.i31.i, %_18.i19.i, !dbg !16138
  br i1 %complete.i.i142, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088, label %bb7.i32.i, !dbg !16139

bb7.i32.i:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1124
  %base.i1107 = shl i32 %_143.i119, 2, !dbg !16140
  %596 = or disjoint i32 %base.i1107, 3, !dbg !16142
  %or.cond.i1111.not = icmp ult i32 %596, %_54.1.i.i127, !dbg !16142
  br i1 %or.cond.i1111.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115, label %bb4.i1114, !dbg !16142, !prof !10587

bb4.i1114:                                        ; preds = %bb7.i32.i
  store <4 x float> %.lcssa1144612420, ptr %404, align 16
  store <4 x float> %.lcssa1143812430, ptr %407, align 16
  store <4 x float> %.lcssa1146112440, ptr %415, align 16
  store <16 x i8> %_4.i3727, ptr %_78.i102, align 16, !dbg !16037
  store <16 x i8> %_4.i3728, ptr %405, align 16, !dbg !16038
  store <16 x i8> %_4.i3729, ptr %_79.i103, align 16, !dbg !16039
  store <16 x i8> %_4.i3730, ptr %408, align 16, !dbg !16040
  %_5.i1108 = add i32 %base.i1107, 4, !dbg !16146
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1107, i32 noundef %_5.i1108, i32 noundef range(i32 0, 536870912) %_54.1.i.i127, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !16147, !noalias !16148
  unreachable, !dbg !16147

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115: ; preds = %bb7.i32.i
  %_15.i1113 = getelementptr inbounds nuw float, ptr %_54.0.i.i126, i32 %base.i1107, !dbg !16152
  %lanes.i2939.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1113, align 4, !dbg !16154, !alias.scope !16159, !noalias !16163
  %597 = bitcast <4 x i32> %.sroa.04770.0 to <4 x float>, !dbg !16167
  %598 = fcmp olt <4 x float> %lanes.i2939.sroa.0.0.copyload, %597, !dbg !16171
  %599 = select <4 x i1> %598, <4 x float> %lanes.i2939.sroa.0.0.copyload, <4 x float> %597, !dbg !16172
  %600 = bitcast <4 x float> %599 to <4 x i32>, !dbg !16173
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i143, !dbg !16175

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1124
  %601 = bitcast <4 x i32> %lanes.i2932.sroa.0.0.copyload to <4 x float>, !dbg !16139
  br i1 %_29.i.i1959482.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i143, label %bb19.i.i196, !dbg !16176

bb19.i.i196:                                      ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %end.sroa.0.0.i.i1949484 = phi i32 [ %607, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ %_142.i118, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088 ]
  %iter.sroa.0.0.i35.i9483 = phi i32 [ %_30.i36.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088 ]
  %602 = phi <4 x float> [ %605, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ %601, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088 ]
  %base.i1076 = shl i32 %end.sroa.0.0.i.i1949484, 2, !dbg !16179
  %603 = or disjoint i32 %base.i1076, 3, !dbg !16181
  %or.cond.i1077.not = icmp ult i32 %603, %_54.1.i.i127, !dbg !16181
  br i1 %or.cond.i1077.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb4.i1079, !dbg !16181, !prof !10587

bb4.i1079:                                        ; preds = %bb19.i.i196
  store <4 x float> %.lcssa1144612420, ptr %404, align 16
  store <4 x float> %.lcssa1143812430, ptr %407, align 16
  store <4 x float> %.lcssa1146112440, ptr %415, align 16
  store <16 x i8> %_4.i3727, ptr %_78.i102, align 16, !dbg !16037
  store <16 x i8> %_4.i3728, ptr %405, align 16, !dbg !16038
  store <16 x i8> %_4.i3729, ptr %_79.i103, align 16, !dbg !16039
  store <16 x i8> %_4.i3730, ptr %408, align 16, !dbg !16040
  %_5.i = add i32 %base.i1076, 4, !dbg !16185
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1076, i32 noundef %_5.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i127, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !16186, !noalias !16187
  unreachable, !dbg !16186

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb19.i.i196
  %_30.i36.i = add nuw i32 %iter.sroa.0.0.i35.i9483, 1, !dbg !16191
  %_15.i = getelementptr inbounds nuw float, ptr %_54.0.i.i126, i32 %base.i1076, !dbg !16194
  %lanes.i2967.sroa.0.0.copyload = load <4 x float>, ptr %_15.i, align 4, !dbg !16196, !alias.scope !16201, !noalias !16205
  %604 = fcmp olt <4 x float> %602, %lanes.i2967.sroa.0.0.copyload, !dbg !16209
  %605 = select <4 x i1> %604, <4 x float> %602, <4 x float> %lanes.i2967.sroa.0.0.copyload, !dbg !16213
  store <4 x float> %605, ptr %_15.i, align 4, !dbg !16214, !alias.scope !16220, !noalias !16224
  %606 = icmp eq i32 %end.sroa.0.0.i.i1949484, 0, !dbg !16230
  %spec.store.select.i.i199 = select i1 %606, i32 %ring.i44, i32 %end.sroa.0.0.i.i1949484, !dbg !16230
  %607 = add i32 %spec.store.select.i.i199, -1, !dbg !16231
  %exitcond11159.not = icmp eq i32 %_30.i36.i, %_18.i19.i, !dbg !16232
  br i1 %exitcond11159.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i143, label %bb19.i.i196, !dbg !16176

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i143: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115
  %.sroa.04770.1 = phi <4 x i32> [ %600, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ], [ %.sroa.04770.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088 ], [ %.sroa.04770.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], !dbg !16234
  %storemerge.i.i144 = phi i32 [ %_15.i31.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1115 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1088 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], !dbg !16235
  %608 = bitcast <4 x i32> %.sroa.04770.1 to <4 x float>, !dbg !16236
  %609 = fmul <4 x float> %608, splat (float 1.638400e+04), !dbg !16240
  %610 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %609), !dbg !16241
  %611 = fmul <4 x float> %610, splat (float 0x3F10000000000000), !dbg !16245
  %base.i1098 = shl i32 %_144.i120, 2, !dbg !16249
  %612 = or disjoint i32 %base.i1098, 3, !dbg !16251
  %or.cond.i1102.not = icmp ult i32 %612, %_56.1.i.i150, !dbg !16251
  br i1 %or.cond.i1102.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106, label %bb4.i1105, !dbg !16251, !prof !10587

bb4.i1105:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i143
  store <4 x float> %.lcssa1144612420, ptr %404, align 16
  store <4 x float> %.lcssa1143812430, ptr %407, align 16
  store <4 x float> %.lcssa1146112440, ptr %415, align 16
  store <16 x i8> %_4.i3727, ptr %_78.i102, align 16, !dbg !16037
  store <16 x i8> %_4.i3728, ptr %405, align 16, !dbg !16038
  store <16 x i8> %_4.i3729, ptr %_79.i103, align 16, !dbg !16039
  store <16 x i8> %_4.i3730, ptr %408, align 16, !dbg !16040
  %_5.i1099 = add i32 %base.i1098, 4, !dbg !16255
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1098, i32 noundef %_5.i1099, i32 noundef range(i32 0, 536870912) %_56.1.i.i150, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !16256, !noalias !16257
  unreachable, !dbg !16256

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i143
  %_15.i1104 = getelementptr inbounds nuw float, ptr %_56.0.i.i149, i32 %base.i1098, !dbg !16261
  %lanes.i2946.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1104, align 4, !dbg !16263, !alias.scope !16268, !noalias !16272
  %613 = fadd <4 x float> %611, %556, !dbg !16276
  %614 = fsub <4 x float> %613, %lanes.i2946.sroa.0.0.copyload, !dbg !16280
  %_8.not.i4.i.i159 = icmp ugt i32 %_7.i10.i.i130, %_56.1.i.i150
  br i1 %_8.not.i4.i.i159, label %bb4.i7.i.i191, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i161, !dbg !16284, !prof !4694

bb4.i7.i.i191:                                    ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106
  store <4 x float> %.lcssa1144612420, ptr %404, align 16
  store <4 x float> %.lcssa1143812430, ptr %407, align 16
  store <4 x float> %.lcssa1146112440, ptr %415, align 16
  store <16 x i8> %_4.i3727, ptr %_78.i102, align 16, !dbg !16037
  store <16 x i8> %_4.i3728, ptr %405, align 16, !dbg !16038
  store <16 x i8> %_4.i3729, ptr %_79.i103, align 16, !dbg !16039
  store <16 x i8> %_4.i3730, ptr %408, align 16, !dbg !16040
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i.i129, i32 noundef %_7.i10.i.i130, i32 noundef range(i32 0, 536870912) %_56.1.i.i150, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !16289, !noalias !16290
  unreachable, !dbg !16289

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i161: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1106
  %_17.i6.i.i162 = getelementptr inbounds nuw float, ptr %_56.0.i.i149, i32 %base.i9.i.i129, !dbg !16294
  store <4 x float> %611, ptr %_17.i6.i.i162, align 4, !dbg !16296, !alias.scope !16301, !noalias !16305
  %_41.i.i1678184 = load <4 x float>, ptr %417, align 16, !dbg !16309
  %615 = fdiv <4 x float> %614, %_37.i.i1648183, !dbg !16310
  %616 = fsub <4 x float> splat (float 1.000000e+00), %615, !dbg !16314
  %617 = fsub <4 x float> %616, %_41.i.i1678184, !dbg !16318
  %618 = bitcast <16 x i8> %_4.i3729 to <4 x float>, !dbg !16322
  %619 = fmul <4 x float> %617, %618, !dbg !16326
  %620 = fadd <4 x float> %_41.i.i1678184, %619, !dbg !16327
  %621 = fcmp olt <4 x float> %620, %616, !dbg !16330
  %622 = select <4 x i1> %621, <4 x float> %616, <4 x float> %620, !dbg !16334
  %623 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %622), !dbg !16335
  %624 = fcmp uge <4 x float> %623, splat (float 0x3BC79CA100000000), !dbg !16340
  %625 = bitcast <4 x float> %622 to <4 x i32>, !dbg !16345
  %626 = select <4 x i1> %624, <4 x i32> %625, <4 x i32> zeroinitializer, !dbg !16345
  store <4 x i32> %626, ptr %417, align 16, !dbg !16348
  %base.i1089 = shl i32 %_141.i117, 2, !dbg !16349
  %627 = or disjoint i32 %base.i1089, 3, !dbg !16351
  %or.cond.i1093.not = icmp ult i32 %627, %_58.1.i.i178, !dbg !16351
  br i1 %or.cond.i1093.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097, label %bb4.i1096, !dbg !16351, !prof !10587

bb4.i1096:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i161
  store <4 x float> %.lcssa1144612420, ptr %404, align 16
  store <4 x float> %.lcssa1143812430, ptr %407, align 16
  store <4 x float> %.lcssa1146112440, ptr %415, align 16
  store <16 x i8> %_4.i3727, ptr %_78.i102, align 16, !dbg !16037
  store <16 x i8> %_4.i3728, ptr %405, align 16, !dbg !16038
  store <16 x i8> %_4.i3729, ptr %_79.i103, align 16, !dbg !16039
  store <16 x i8> %_4.i3730, ptr %408, align 16, !dbg !16040
  %_5.i1090 = add i32 %base.i1089, 4, !dbg !16355
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1089, i32 noundef %_5.i1090, i32 noundef range(i32 0, 536870912) %_58.1.i.i178, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !16356, !noalias !16357
  unreachable, !dbg !16356

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i161
  %628 = bitcast <4 x i32> %626 to <4 x float>, !dbg !16361
  %629 = fsub <4 x float> splat (float 1.000000e+00), %628, !dbg !16365
  %_15.i1095 = getelementptr inbounds nuw float, ptr %_58.0.i.i177, i32 %base.i1089, !dbg !16366
  %lanes.i2953.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1095, align 4, !dbg !16368, !alias.scope !16373, !noalias !16377
  store <4 x i32> %lanes.i3083.sroa.0.0.copyload, ptr %_15.i1095, align 4, !dbg !16381, !alias.scope !16387, !noalias !16391
  %630 = bitcast <4 x i32> %lanes.i2953.sroa.0.0.copyload to <4 x float>, !dbg !16397
  %631 = fmul <4 x float> %629, %630, !dbg !16401
  %632 = bitcast <4 x i32> %lanes.i2953.sroa.0.0.copyload to <16 x i8>, !dbg !16402
  %633 = bitcast <4 x float> %631 to <16 x i8>, !dbg !16406
  %_4.i3733 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %632, <16 x i8> %633, <16 x i8> %420), !dbg !16407
  store <16 x i8> %_4.i3733, ptr %data.i.i3725, align 4, !dbg !16408, !alias.scope !16413, !noalias !16417
  %exitcond11169.not = icmp eq i32 %575, %555, !dbg !15920
  br i1 %exitcond11169.not, label %bb21.i91.bb48.i202_crit_edge, label %bb49.i93, !dbg !15920

bb21.i91.bb48.i202_crit_edge:                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1097
  store <16 x i8> %_4.i3727, ptr %_78.i102, align 16, !dbg !16037
  store <16 x i8> %_4.i3728, ptr %405, align 16, !dbg !16038
  store <16 x i8> %_4.i3729, ptr %_79.i103, align 16, !dbg !16039
  store <16 x i8> %_4.i3730, ptr %408, align 16, !dbg !16040
  br label %bb48.i202, !dbg !15920

bb48.i202:                                        ; preds = %bb21.i91.bb48.i202_crit_edge, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit
  %.lcssa1146112439 = phi <4 x float> [ %614, %bb21.i91.bb48.i202_crit_edge ], [ %.lcssa1146112440, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %.lcssa1143812429 = phi <4 x float> [ %577, %bb21.i91.bb48.i202_crit_edge ], [ %.lcssa1143812430, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %.lcssa1144612419 = phi <4 x float> [ %576, %bb21.i91.bb48.i202_crit_edge ], [ %.lcssa1144612420, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %storemerge.i.i144.lcssa95309556 = phi i32 [ %storemerge.i.i144, %bb21.i91.bb48.i202_crit_edge ], [ %storemerge.i.i144.lcssa95309557, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit ]
  %_92.i203 = add i32 %_31.sroa.0.0.i, %ring_cursor.sroa.0.1.i649550, !dbg !16421
  %_139.not.i204 = icmp ult i32 %_92.i203, %ring.i44, !dbg !16422
  %634 = select i1 %_139.not.i204, i32 0, i32 %ring.i44, !dbg !16422
  %ring_cursor.sroa.0.2.i205 = sub nuw i32 %_92.i203, %634, !dbg !16422
  %_94.i206 = add i32 %_31.sroa.0.0.i, %main_cursor.sroa.0.1.i659551, !dbg !16425
  %_145.not.i207 = icmp ult i32 %_94.i206, %main.i45, !dbg !16426
  %635 = select i1 %_145.not.i207, i32 0, i32 %main.i45, !dbg !16426
  %main_cursor.sroa.0.2.i208 = sub nuw i32 %_94.i206, %635, !dbg !16426
  %_41.i67 = icmp ult i32 %_64.i81, %spec.store.select.i58, !dbg !15852
  br i1 %_41.i67, label %bb17.i68, label %bb13.i51.loopexit.loopexit, !dbg !15852

_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb13.i51.loopexit, %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge
  %left_phase.i414 = phi i32 [ %left_phase.i414.pre, %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %storemerge.i.i144.lcssa95309556.lcssa9582, %bb13.i51.loopexit ], !dbg !15354
  %ring_cursor.sroa.0.0.i52.lcssa = phi i32 [ %_27.i47, %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %ring_cursor.sroa.0.1.i64.lcssa, %bb13.i51.loopexit ], !dbg !15336
  %main_cursor.sroa.0.0.i53.lcssa = phi i32 [ %_26.i46, %bb4.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %main_cursor.sroa.0.1.i65.lcssa, %bb13.i51.loopexit ], !dbg !15333
  %left_prefix.i413 = load <4 x i32>, ptr %uniform_left.i33, align 16, !dbg !16428, !noalias !15340
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i33), !dbg !16429, !noalias !15340
  %636 = getelementptr inbounds nuw i8, ptr %self, i32 968, !dbg !16430
  %_152.1.i416 = load i32, ptr %636, align 4, !dbg !16430, !alias.scope !15312, !noalias !16432, !noundef !10
  %_8.i3437 = icmp samesign ugt i32 %_152.1.i416, 3, !dbg !16433
  br i1 %_8.i3437, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3440, label %bb2.i3438, !dbg !16433, !prof !1153

bb2.i3438:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_152.1.i416, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !16438, !noalias !16439
  unreachable, !dbg !16438

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3440: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %637 = getelementptr inbounds nuw i8, ptr %self, i32 964, !dbg !16430
  %_152.0.i415 = load ptr, ptr %637, align 4, !dbg !16430, !alias.scope !15312, !noalias !16432, !nonnull !10, !noundef !10
  store <4 x i32> %left_prefix.i413, ptr %_152.0.i415, align 4, !dbg !16443, !alias.scope !16447, !noalias !16451
  %638 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !16453
  %_153.0.i417 = load ptr, ptr %638, align 4, !dbg !16453, !alias.scope !15312, !noalias !16432, !nonnull !10, !noundef !10
  %639 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !16453
  %_153.1.i418 = load i32, ptr %639, align 4, !dbg !16453, !alias.scope !15312, !noalias !16432, !noundef !10
  %640 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i414), !dbg !16454
  br i1 %640, label %bb2.i3739, label %bb6.i3734, !dbg !16454

bb6.i3734:                                        ; preds = %bb2.i3739, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3440
  %end_or_len.idx.i = shl nuw nsw i32 %_153.1.i418, 2, !dbg !16458
  %end_or_len.i = getelementptr inbounds nuw i8, ptr %_153.0.i417, i32 %end_or_len.idx.i, !dbg !16458
  %_293.i = icmp eq i32 %_153.1.i418, 0, !dbg !16462
  br i1 %_293.i, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i3735, !dbg !16465

bb2.i3739:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3440
  %bytes1.sroa.0.0.zext.i = and i32 %left_phase.i414, 255, !dbg !16466
  %bytes1.sroa.0.0.isplat.i = mul nuw i32 %bytes1.sroa.0.0.zext.i, 16843009, !dbg !16466
  %_5.i3740 = icmp eq i32 %left_phase.i414, %bytes1.sroa.0.0.isplat.i, !dbg !16467
  br i1 %_5.i3740, label %bb3.i3741, label %bb6.i3734, !dbg !16467

bb3.i3741:                                        ; preds = %bb2.i3739
  %bytes.sroa.0.0.extract.trunc.i = trunc i32 %left_phase.i414 to i8, !dbg !16468
  %641 = shl nuw nsw i32 %_153.1.i418, 2, !dbg !16470
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_153.0.i417, i8 %bytes.sroa.0.0.extract.trunc.i, i32 %641, i1 false), !dbg !16470, !alias.scope !16471, !noalias !15903
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, !dbg !16474

bb10.i3735:                                       ; preds = %bb6.i3734, %bb10.i3735
  %iter.sroa.0.04.i = phi ptr [ %_38.i3736, %bb10.i3735 ], [ %_153.0.i417, %bb6.i3734 ]
  %_38.i3736 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i, i32 4, !dbg !16475
  store i32 %left_phase.i414, ptr %iter.sroa.0.04.i, align 4, !dbg !16477, !alias.scope !16471, !noalias !15903
  %_29.i3737 = icmp eq ptr %_38.i3736, %end_or_len.i, !dbg !16462
  br i1 %_29.i3737, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, label %bb10.i3735, !dbg !16465

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit: ; preds = %bb10.i3735, %bb6.i3734, %bb3.i3741
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i36, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24) #32, !dbg !16478
  store i32 %main_cursor.sroa.0.0.i53.lcssa, ptr %_25, align 4, !dbg !16479, !alias.scope !15314, !noalias !15335
  store i32 %ring_cursor.sroa.0.0.i52.lcssa, ptr %365, align 4, !dbg !16480, !alias.scope !15314, !noalias !15335
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i34), !dbg !16481, !noalias !15340
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !15309

bb6.i:                                            ; preds = %_RNCNvCsjLJhryqjeDL_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3672, %bb2.i.i.i3668, %bb11.i.i3662, %bb11.i5.i3684
  br i1 %_12.i.i3646, label %bb7.i, label %bb8.i, !dbg !16482

bb2.i:                                            ; preds = %bb1.i3.i3681, %bb2.i3675
  br i1 %_12.i.i3646, label %bb3.i, label %bb4.i, !dbg !16483

bb7.i:                                            ; preds = %bb6.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16484), !dbg !16487
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16488), !dbg !16487
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16490), !dbg !16487
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i436, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #32, !dbg !16492
  %642 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !16496
  %643 = load i8, ptr %642, align 16, !dbg !16496, !range !4765, !alias.scope !16484, !noalias !16500, !noundef !10
  %644 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !16503
  %645 = load i8, ptr %644, align 1, !dbg !16503, !range !4765, !alias.scope !16484, !noalias !16500, !noundef !10
  %_25.i = load i32, ptr %_25, align 4, !dbg !16505, !alias.scope !16490, !noalias !16507, !noundef !10
  %646 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !16508
  %_27.i443 = load i32, ptr %646, align 4, !dbg !16508, !alias.scope !16490, !noalias !16507, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i), !dbg !16510, !noalias !16512
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i, i8 0, i32 32, i1 false), !noalias !16512
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i434), !dbg !16513, !noalias !16512
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i434, i8 0, i32 1024, i1 false), !noalias !16512
  %hot_left.i436.promoted = load <4 x i32>, ptr %hot_left.i436, align 1
  %_74.not.i9446 = icmp eq i32 %frames, 0, !dbg !16515
  br i1 %_74.not.i9446, label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, label %bb29.i.lr.ph, !dbg !16515

bb29.i.lr.ph:                                     ; preds = %bb7.i
  %_23.i442 = trunc nuw i8 %645 to i1, !dbg !16503
  %spec.store.select19.i = select i1 %_23.i442, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !16503
  %_22.i440 = trunc nuw i8 %643 to i1, !dbg !16496
  %link.sroa.0.0.i441 = select i1 %_22.i440, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !16496
  %d9.i3742 = lshr i32 %frames, 5, !dbg !16525
  %r2.i3743 = and i32 %frames, 31, !dbg !16532
  %_19.not.i3744 = icmp ne i32 %r2.i3743, 0, !dbg !16533
  %647 = zext i1 %_19.not.i3744 to i32, !dbg !16533
  %yield_count.sroa.0.0.i3745 = add nuw nsw i32 %d9.i3742, %647, !dbg !16533
  %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 16
  %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 32
  %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 48
  %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 64
  %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 80
  %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 96
  %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 112
  %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 128
  %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 144
  %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 160
  %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 176
  %648 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %649 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %650 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i.i526 = getelementptr inbounds nuw i8, ptr %self, i32 80
  %651 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %652 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %653 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i.i540 = getelementptr inbounds nuw i8, ptr %self, i32 144
  %654 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %655 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %656 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i.i554 = getelementptr inbounds nuw i8, ptr %self, i32 208
  %657 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %658 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %659 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i.i568 = getelementptr inbounds nuw i8, ptr %self, i32 272
  %660 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %661 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %662 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i.i582 = getelementptr inbounds nuw i8, ptr %self, i32 336
  %663 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %664 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %665 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i.i596 = getelementptr inbounds nuw i8, ptr %self, i32 400
  %666 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %667 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %668 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i.i610 = getelementptr inbounds nuw i8, ptr %self, i32 464
  %669 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %670 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %671 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i.i624 = getelementptr inbounds nuw i8, ptr %self, i32 528
  %672 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %673 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %674 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i.i638 = getelementptr inbounds nuw i8, ptr %self, i32 592
  %675 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %676 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %677 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i.i652 = getelementptr inbounds nuw i8, ptr %self, i32 656
  %678 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %679 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %680 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i.i666 = getelementptr inbounds nuw i8, ptr %self, i32 720
  %681 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %682 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %683 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %_47.i460 = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 192
  %_48.i = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 256
  %684 = bitcast <4 x i32> %link.sroa.0.0.i441 to <16 x i8>
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
  %695 = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 336
  %696 = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 352
  %697 = getelementptr inbounds nuw i8, ptr %hot_left.i436, i32 320
  %698 = getelementptr inbounds nuw i8, ptr %self, i32 936
  %699 = getelementptr inbounds nuw i8, ptr %self, i32 932
  %700 = bitcast <4 x i32> %spec.store.select19.i to <16 x i8>
  %701 = getelementptr inbounds nuw i8, ptr %self, i32 920
  %history.i.i433.sroa.7.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.10.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.13.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.16.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.19.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.22.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.26.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.29.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.32.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.35.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %history.i.i433.sroa.38.0.copyload.pre = load <4 x i32>, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 16, !dbg !16534
  %_8.i.i4618099 = load <4 x float>, ptr %_47.i460, align 16
  %_9.i.i4628100 = load <4 x float>, ptr %_48.i, align 16
  %_62.i.i4868108 = load <4 x float>, ptr %696, align 16
  %iter.sroa.0.0.ptr.i.i9376.1 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 4
  %iter.sroa.0.0.ptr.i.i9376.2 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 8
  %iter.sroa.0.0.ptr.i.i9376.3 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 12
  %iter.sroa.0.0.ptr.i.i9376.4 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 16
  %iter.sroa.0.0.ptr.i.i9376.5 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 20
  %iter.sroa.0.0.ptr.i.i9376.6 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 24
  %iter.sroa.0.0.ptr.i.i9376.7 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 28
  %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1
  %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1
  %.promoted12369 = load <4 x float>, ptr %695, align 16
  br label %bb29.i, !dbg !16515

bb12.i446.loopexit:                               ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i454
  %.lcssa1164912370 = phi <4 x float> [ %.lcssa1164912371, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i454 ], [ %860, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420 ]
  %ring_cursor.sroa.0.1.i456.lcssa = phi i32 [ %ring_cursor.sroa.0.0.i4489450, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i454 ], [ %spec.store.select8.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420 ], !dbg !16538
  %main_cursor.sroa.0.1.i457.lcssa = phi i32 [ %main_cursor.sroa.0.0.i4499451, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i454 ], [ %spec.store.select7.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420 ], !dbg !16539
  %_74.not.i = icmp eq i32 %704, 0, !dbg !16515
  %indvars.iv.next11135 = add i32 %indvars.iv11134, -32, !dbg !16515
  br i1 %_74.not.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, label %bb29.i, !dbg !16515

bb29.i:                                           ; preds = %bb29.i.lr.ph, %bb12.i446.loopexit
  %.lcssa1164912371 = phi <4 x float> [ %.promoted12369, %bb29.i.lr.ph ], [ %.lcssa1164912370, %bb12.i446.loopexit ]
  %history.i.i433.sroa.38.0.lcssa12350 = phi <4 x i32> [ %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.38.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.35.0.lcssa12331 = phi <4 x i32> [ %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.35.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.32.0.lcssa12312 = phi <4 x i32> [ %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.32.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.29.0.lcssa12293 = phi <4 x i32> [ %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.29.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.26.0.lcssa12274 = phi <4 x i32> [ %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.26.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.22.0.lcssa12255 = phi <4 x i32> [ %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.22.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.19.0.lcssa12236 = phi <4 x i32> [ %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.19.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.16.0.lcssa12217 = phi <4 x i32> [ %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.16.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.13.0.lcssa12198 = phi <4 x i32> [ %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.13.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.10.0.lcssa12179 = phi <4 x i32> [ %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.10.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.7.0.lcssa12160 = phi <4 x i32> [ %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.7.0.lcssa, %bb12.i446.loopexit ]
  %history.i.i433.sroa.38.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.38.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.38.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.35.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.35.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.35.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.32.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.32.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.32.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.29.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.29.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.29.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.26.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.26.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.26.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.22.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.22.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.22.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.19.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.19.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.19.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.16.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.16.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.16.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.13.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.13.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.13.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.10.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.10.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.10.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %history.i.i433.sroa.7.0.copyload = phi <4 x i32> [ %history.i.i433.sroa.7.0.copyload.pre, %bb29.i.lr.ph ], [ %history.i.i433.sroa.7.0.lcssa, %bb12.i446.loopexit ], !dbg !16534
  %indvars.iv11134 = phi i32 [ %frames, %bb29.i.lr.ph ], [ %indvars.iv.next11135, %bb12.i446.loopexit ]
  %main_cursor.sroa.0.0.i4499451 = phi i32 [ %_25.i, %bb29.i.lr.ph ], [ %main_cursor.sroa.0.1.i457.lcssa, %bb12.i446.loopexit ]
  %ring_cursor.sroa.0.0.i4489450 = phi i32 [ %_27.i443, %bb29.i.lr.ph ], [ %ring_cursor.sroa.0.1.i456.lcssa, %bb12.i446.loopexit ]
  %iter2.sroa.0.0.i4479449 = phi i32 [ %yield_count.sroa.0.0.i3745, %bb29.i.lr.ph ], [ %704, %bb12.i446.loopexit ]
  %iter.sroa.0.0.i9448 = phi i32 [ 0, %bb29.i.lr.ph ], [ %703, %bb12.i446.loopexit ]
  %history.i.i433.sroa.0.0.lcssa94239447 = phi <4 x i32> [ %hot_left.i436.promoted, %bb29.i.lr.ph ], [ %history.i.i433.sroa.0.0.lcssa, %bb12.i446.loopexit ]
  %702 = call i32 @llvm.umax.i32(i32 %indvars.iv11134, i32 1), !dbg !16540
  %umax11149 = call i32 @llvm.umin.i32(i32 %702, i32 32), !dbg !16540
  %703 = add i32 %iter.sroa.0.0.i9448, 32, !dbg !16540
  %704 = add nsw i32 %iter2.sroa.0.0.i4479449, -1, !dbg !16544
  %_20.i.i4539340.not = icmp eq i32 %frames, %iter.sroa.0.0.i9448, !dbg !16545
  br i1 %_20.i.i4539340.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i454, label %bb5.i.i494.lr.ph, !dbg !16549

bb5.i.i494.lr.ph:                                 ; preds = %bb29.i
  %_11.i.i.i.i5148115 = load <4 x float>, ptr %_22, align 16
  %_14.i.i.i.i5178116 = load <4 x float>, ptr %648, align 16
  %_17.i.i.i.i5208117 = load <4 x float>, ptr %649, align 16
  %_20.i.i.i.i5238118 = load <4 x float>, ptr %650, align 16
  %_25.i.i.i.i5288119 = load <4 x float>, ptr %row1.i.i.i.i526, align 16
  %_28.i.i.i.i5318120 = load <4 x float>, ptr %651, align 16
  %_31.i.i.i.i5348121 = load <4 x float>, ptr %652, align 16
  %_34.i.i.i.i5378122 = load <4 x float>, ptr %653, align 16
  %_39.i.i.i.i5428123 = load <4 x float>, ptr %row3.i.i.i.i540, align 16
  %_42.i.i.i.i5458124 = load <4 x float>, ptr %654, align 16
  %_45.i.i.i.i5488125 = load <4 x float>, ptr %655, align 16
  %_48.i.i.i.i5518126 = load <4 x float>, ptr %656, align 16
  %_53.i.i.i.i5568127 = load <4 x float>, ptr %row5.i.i.i.i554, align 16
  %_56.i.i.i.i5598128 = load <4 x float>, ptr %657, align 16
  %_59.i.i.i.i5628129 = load <4 x float>, ptr %658, align 16
  %_62.i.i.i.i5658130 = load <4 x float>, ptr %659, align 16
  %_67.i.i.i.i5708131 = load <4 x float>, ptr %row7.i.i.i.i568, align 16
  %_70.i.i.i.i5738132 = load <4 x float>, ptr %660, align 16
  %_73.i.i.i.i5768133 = load <4 x float>, ptr %661, align 16
  %_76.i.i.i.i5798134 = load <4 x float>, ptr %662, align 16
  %_81.i.i.i.i5848135 = load <4 x float>, ptr %row9.i.i.i.i582, align 16
  %_84.i.i.i.i5878136 = load <4 x float>, ptr %663, align 16
  %_87.i.i.i.i5908137 = load <4 x float>, ptr %664, align 16
  %_90.i.i.i.i5938138 = load <4 x float>, ptr %665, align 16
  %_95.i.i.i.i5988139 = load <4 x float>, ptr %row11.i.i.i.i596, align 16
  %_98.i.i.i.i6018140 = load <4 x float>, ptr %666, align 16
  %_101.i.i.i.i6048141 = load <4 x float>, ptr %667, align 16
  %_104.i.i.i.i6078142 = load <4 x float>, ptr %668, align 16
  %_109.i.i.i.i6128143 = load <4 x float>, ptr %row13.i.i.i.i610, align 16
  %_112.i.i.i.i6158144 = load <4 x float>, ptr %669, align 16
  %_115.i.i.i.i6188145 = load <4 x float>, ptr %670, align 16
  %_118.i.i.i.i6218146 = load <4 x float>, ptr %671, align 16
  %_123.i.i.i.i6268147 = load <4 x float>, ptr %row15.i.i.i.i624, align 16
  %_126.i.i.i.i6298148 = load <4 x float>, ptr %672, align 16
  %_129.i.i.i.i6328149 = load <4 x float>, ptr %673, align 16
  %_132.i.i.i.i6358150 = load <4 x float>, ptr %674, align 16
  %_137.i.i.i.i6408151 = load <4 x float>, ptr %row17.i.i.i.i638, align 16
  %_140.i.i.i.i6438152 = load <4 x float>, ptr %675, align 16
  %_143.i.i.i.i6468153 = load <4 x float>, ptr %676, align 16
  %_146.i.i.i.i6498154 = load <4 x float>, ptr %677, align 16
  %_151.i.i.i.i6548155 = load <4 x float>, ptr %row19.i.i.i.i652, align 16
  %_154.i.i.i.i6578156 = load <4 x float>, ptr %678, align 16
  %_157.i.i.i.i6608157 = load <4 x float>, ptr %679, align 16
  %_160.i.i.i.i6638158 = load <4 x float>, ptr %680, align 16
  %_165.i.i.i.i6688159 = load <4 x float>, ptr %row21.i.i.i.i666, align 16
  %_168.i.i.i.i6718160 = load <4 x float>, ptr %681, align 16
  %_171.i.i.i.i6748161 = load <4 x float>, ptr %682, align 16
  %_174.i.i.i.i6778162 = load <4 x float>, ptr %683, align 16
  br label %bb5.i.i494, !dbg !16549

bb5.i.i494:                                       ; preds = %bb5.i.i494.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415
  %iter.sroa.0.0.i.i4529352 = phi i32 [ 0, %bb5.i.i494.lr.ph ], [ %705, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.35.09351 = phi <4 x i32> [ %history.i.i433.sroa.35.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.32.09350, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.32.09350 = phi <4 x i32> [ %history.i.i433.sroa.32.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.29.09349, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.29.09349 = phi <4 x i32> [ %history.i.i433.sroa.29.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.26.09348, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.26.09348 = phi <4 x i32> [ %history.i.i433.sroa.26.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.22.09347, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.22.09347 = phi <4 x i32> [ %history.i.i433.sroa.22.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.19.09346, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.19.09346 = phi <4 x i32> [ %history.i.i433.sroa.19.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.16.09345, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.16.09345 = phi <4 x i32> [ %history.i.i433.sroa.16.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.13.09344, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.13.09344 = phi <4 x i32> [ %history.i.i433.sroa.13.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.10.09343, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.10.09343 = phi <4 x i32> [ %history.i.i433.sroa.10.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.7.09342, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.7.09342 = phi <4 x i32> [ %history.i.i433.sroa.7.0.copyload, %bb5.i.i494.lr.ph ], [ %history.i.i433.sroa.0.09341, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %history.i.i433.sroa.0.09341 = phi <4 x i32> [ %history.i.i433.sroa.0.0.lcssa94239447, %bb5.i.i494.lr.ph ], [ %lanes.i3024.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ]
  %705 = add nuw nsw i32 %iter.sroa.0.0.i.i4529352, 1, !dbg !16550
  %_11.i.i495 = add nuw nsw i32 %iter.sroa.0.0.i.i4529352, %iter.sroa.0.0.i9448, !dbg !16553
  %base.i.i496 = shl i32 %_11.i.i495, 2, !dbg !16553
  %_24.i.i497 = icmp ugt i32 %base.i.i496, %left_io.1, !dbg !16554
  br i1 %_24.i.i497, label %bb7.i.i695, label %bb8.i.i498, !dbg !16554, !prof !902

bb8.i.i498:                                       ; preds = %bb5.i.i494
  %_27.i.i499 = sub nuw nsw i32 %left_io.1, %base.i.i496, !dbg !16557
  %_8.i3027 = icmp samesign ugt i32 %_27.i.i499, 3, !dbg !16558
  br i1 %_8.i3027, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415, label %bb2.i3028, !dbg !16558, !prof !1153

bb2.i3028:                                        ; preds = %bb8.i.i498
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa12160, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa12179, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa12198, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa12217, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa12236, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa12255, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa12274, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa12293, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa12312, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa12331, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa12350, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i.i499, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !16564, !noalias !16565
  unreachable, !dbg !16564

bb7.i.i695:                                       ; preds = %bb5.i.i494
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa12160, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa12179, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa12198, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa12217, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa12236, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa12255, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa12274, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa12293, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa12312, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa12331, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa12350, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i.i496, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !16572, !noalias !16573
  unreachable, !dbg !16572

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415: ; preds = %bb8.i.i498
  %_31.i20.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i.i496, !dbg !16574
  %lanes.i3024.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i20.i, align 4, !dbg !16576, !alias.scope !16580, !noalias !16584
  %706 = bitcast <4 x i32> %history.i.i433.sroa.19.09346 to <4 x float>, !dbg !16586
  %707 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %706), !dbg !16591
  %708 = bitcast <4 x i32> %lanes.i3024.sroa.0.0.copyload to <4 x float>, !dbg !16592
  %709 = fmul <4 x float> %_11.i.i.i.i5148115, %708, !dbg !16597
  %710 = fadd <4 x float> %709, zeroinitializer, !dbg !16598
  %711 = fmul <4 x float> %_14.i.i.i.i5178116, %708, !dbg !16602
  %712 = fadd <4 x float> %711, zeroinitializer, !dbg !16606
  %713 = fmul <4 x float> %_17.i.i.i.i5208117, %708, !dbg !16610
  %714 = fadd <4 x float> %713, zeroinitializer, !dbg !16614
  %715 = fmul <4 x float> %_20.i.i.i.i5238118, %708, !dbg !16618
  %716 = fadd <4 x float> %715, zeroinitializer, !dbg !16622
  %717 = bitcast <4 x i32> %history.i.i433.sroa.0.09341 to <4 x float>, !dbg !16626
  %718 = fmul <4 x float> %_25.i.i.i.i5288119, %717, !dbg !16630
  %719 = fadd <4 x float> %710, %718, !dbg !16631
  %720 = fmul <4 x float> %_28.i.i.i.i5318120, %717, !dbg !16635
  %721 = fadd <4 x float> %712, %720, !dbg !16639
  %722 = fmul <4 x float> %_31.i.i.i.i5348121, %717, !dbg !16643
  %723 = fadd <4 x float> %714, %722, !dbg !16647
  %724 = fmul <4 x float> %_34.i.i.i.i5378122, %717, !dbg !16651
  %725 = fadd <4 x float> %716, %724, !dbg !16655
  %726 = bitcast <4 x i32> %history.i.i433.sroa.7.09342 to <4 x float>, !dbg !16659
  %727 = fmul <4 x float> %_39.i.i.i.i5428123, %726, !dbg !16663
  %728 = fadd <4 x float> %719, %727, !dbg !16664
  %729 = fmul <4 x float> %_42.i.i.i.i5458124, %726, !dbg !16668
  %730 = fadd <4 x float> %721, %729, !dbg !16672
  %731 = fmul <4 x float> %_45.i.i.i.i5488125, %726, !dbg !16676
  %732 = fadd <4 x float> %723, %731, !dbg !16680
  %733 = fmul <4 x float> %_48.i.i.i.i5518126, %726, !dbg !16684
  %734 = fadd <4 x float> %725, %733, !dbg !16688
  %735 = bitcast <4 x i32> %history.i.i433.sroa.10.09343 to <4 x float>, !dbg !16692
  %736 = fmul <4 x float> %_53.i.i.i.i5568127, %735, !dbg !16696
  %737 = fadd <4 x float> %728, %736, !dbg !16697
  %738 = fmul <4 x float> %_56.i.i.i.i5598128, %735, !dbg !16701
  %739 = fadd <4 x float> %730, %738, !dbg !16705
  %740 = fmul <4 x float> %_59.i.i.i.i5628129, %735, !dbg !16709
  %741 = fadd <4 x float> %732, %740, !dbg !16713
  %742 = fmul <4 x float> %_62.i.i.i.i5658130, %735, !dbg !16717
  %743 = fadd <4 x float> %734, %742, !dbg !16721
  %744 = bitcast <4 x i32> %history.i.i433.sroa.13.09344 to <4 x float>, !dbg !16725
  %745 = fmul <4 x float> %_67.i.i.i.i5708131, %744, !dbg !16729
  %746 = fadd <4 x float> %737, %745, !dbg !16730
  %747 = fmul <4 x float> %_70.i.i.i.i5738132, %744, !dbg !16734
  %748 = fadd <4 x float> %739, %747, !dbg !16738
  %749 = fmul <4 x float> %_73.i.i.i.i5768133, %744, !dbg !16742
  %750 = fadd <4 x float> %741, %749, !dbg !16746
  %751 = fmul <4 x float> %_76.i.i.i.i5798134, %744, !dbg !16750
  %752 = fadd <4 x float> %743, %751, !dbg !16754
  %753 = bitcast <4 x i32> %history.i.i433.sroa.16.09345 to <4 x float>, !dbg !16758
  %754 = fmul <4 x float> %_81.i.i.i.i5848135, %753, !dbg !16762
  %755 = fadd <4 x float> %746, %754, !dbg !16763
  %756 = fmul <4 x float> %_84.i.i.i.i5878136, %753, !dbg !16767
  %757 = fadd <4 x float> %748, %756, !dbg !16771
  %758 = fmul <4 x float> %_87.i.i.i.i5908137, %753, !dbg !16775
  %759 = fadd <4 x float> %750, %758, !dbg !16779
  %760 = fmul <4 x float> %_90.i.i.i.i5938138, %753, !dbg !16783
  %761 = fadd <4 x float> %752, %760, !dbg !16787
  %762 = fmul <4 x float> %_95.i.i.i.i5988139, %706, !dbg !16791
  %763 = fadd <4 x float> %755, %762, !dbg !16795
  %764 = fmul <4 x float> %_98.i.i.i.i6018140, %706, !dbg !16799
  %765 = fadd <4 x float> %757, %764, !dbg !16803
  %766 = fmul <4 x float> %_101.i.i.i.i6048141, %706, !dbg !16807
  %767 = fadd <4 x float> %759, %766, !dbg !16811
  %768 = fmul <4 x float> %_104.i.i.i.i6078142, %706, !dbg !16815
  %769 = fadd <4 x float> %761, %768, !dbg !16819
  %770 = bitcast <4 x i32> %history.i.i433.sroa.22.09347 to <4 x float>, !dbg !16823
  %771 = fmul <4 x float> %_109.i.i.i.i6128143, %770, !dbg !16827
  %772 = fadd <4 x float> %763, %771, !dbg !16828
  %773 = fmul <4 x float> %_112.i.i.i.i6158144, %770, !dbg !16832
  %774 = fadd <4 x float> %765, %773, !dbg !16836
  %775 = fmul <4 x float> %_115.i.i.i.i6188145, %770, !dbg !16840
  %776 = fadd <4 x float> %767, %775, !dbg !16844
  %777 = fmul <4 x float> %_118.i.i.i.i6218146, %770, !dbg !16848
  %778 = fadd <4 x float> %769, %777, !dbg !16852
  %779 = bitcast <4 x i32> %history.i.i433.sroa.26.09348 to <4 x float>, !dbg !16856
  %780 = fmul <4 x float> %_123.i.i.i.i6268147, %779, !dbg !16860
  %781 = fadd <4 x float> %772, %780, !dbg !16861
  %782 = fmul <4 x float> %_126.i.i.i.i6298148, %779, !dbg !16865
  %783 = fadd <4 x float> %774, %782, !dbg !16869
  %784 = fmul <4 x float> %_129.i.i.i.i6328149, %779, !dbg !16873
  %785 = fadd <4 x float> %776, %784, !dbg !16877
  %786 = fmul <4 x float> %_132.i.i.i.i6358150, %779, !dbg !16881
  %787 = fadd <4 x float> %778, %786, !dbg !16885
  %788 = bitcast <4 x i32> %history.i.i433.sroa.29.09349 to <4 x float>, !dbg !16889
  %789 = fmul <4 x float> %_137.i.i.i.i6408151, %788, !dbg !16893
  %790 = fadd <4 x float> %781, %789, !dbg !16894
  %791 = fmul <4 x float> %_140.i.i.i.i6438152, %788, !dbg !16898
  %792 = fadd <4 x float> %783, %791, !dbg !16902
  %793 = fmul <4 x float> %_143.i.i.i.i6468153, %788, !dbg !16906
  %794 = fadd <4 x float> %785, %793, !dbg !16910
  %795 = fmul <4 x float> %_146.i.i.i.i6498154, %788, !dbg !16914
  %796 = fadd <4 x float> %787, %795, !dbg !16918
  %797 = bitcast <4 x i32> %history.i.i433.sroa.32.09350 to <4 x float>, !dbg !16922
  %798 = fmul <4 x float> %_151.i.i.i.i6548155, %797, !dbg !16926
  %799 = fadd <4 x float> %790, %798, !dbg !16927
  %800 = fmul <4 x float> %_154.i.i.i.i6578156, %797, !dbg !16931
  %801 = fadd <4 x float> %792, %800, !dbg !16935
  %802 = fmul <4 x float> %_157.i.i.i.i6608157, %797, !dbg !16939
  %803 = fadd <4 x float> %794, %802, !dbg !16943
  %804 = fmul <4 x float> %_160.i.i.i.i6638158, %797, !dbg !16947
  %805 = fadd <4 x float> %796, %804, !dbg !16951
  %806 = bitcast <4 x i32> %history.i.i433.sroa.35.09351 to <4 x float>, !dbg !16955
  %807 = fmul <4 x float> %_165.i.i.i.i6688159, %806, !dbg !16959
  %808 = fadd <4 x float> %799, %807, !dbg !16960
  %809 = fmul <4 x float> %_168.i.i.i.i6718160, %806, !dbg !16964
  %810 = fadd <4 x float> %801, %809, !dbg !16968
  %811 = fmul <4 x float> %_171.i.i.i.i6748161, %806, !dbg !16972
  %812 = fadd <4 x float> %803, %811, !dbg !16976
  %813 = fmul <4 x float> %_174.i.i.i.i6778162, %806, !dbg !16980
  %814 = fadd <4 x float> %805, %813, !dbg !16984
  %815 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %808), !dbg !16988
  %816 = fcmp olt <4 x float> %815, %707, !dbg !16992
  %817 = select <4 x i1> %816, <4 x float> %707, <4 x float> %815, !dbg !16996
  %818 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %810), !dbg !16988
  %819 = fcmp olt <4 x float> %818, %817, !dbg !16992
  %820 = select <4 x i1> %819, <4 x float> %817, <4 x float> %818, !dbg !16996
  %821 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %812), !dbg !16988
  %822 = fcmp olt <4 x float> %821, %820, !dbg !16992
  %823 = select <4 x i1> %822, <4 x float> %820, <4 x float> %821, !dbg !16996
  %824 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %814), !dbg !16988
  %825 = fcmp olt <4 x float> %824, %823, !dbg !16992
  %826 = select <4 x i1> %825, <4 x float> %823, <4 x float> %824, !dbg !16996
  %_39.i.i689.idx = shl i32 %iter.sroa.0.0.i.i4529352, 4, !dbg !16997
  %_39.i.i689 = getelementptr inbounds nuw i8, ptr %peaks_left.i434, i32 %_39.i.i689.idx, !dbg !16997
  store <4 x float> %826, ptr %_39.i.i689, align 4, !dbg !17002, !alias.scope !17007, !noalias !17011
  %exitcond11138.not = icmp eq i32 %705, %umax11149, !dbg !16545
  br i1 %exitcond11138.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i454, label %bb5.i.i494, !dbg !16549

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i454: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415, %bb29.i
  %history.i.i433.sroa.0.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.0.0.lcssa94239447, %bb29.i ], [ %lanes.i3024.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.7.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.7.0.copyload, %bb29.i ], [ %history.i.i433.sroa.0.09341, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.10.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.10.0.copyload, %bb29.i ], [ %history.i.i433.sroa.7.09342, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.13.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.13.0.copyload, %bb29.i ], [ %history.i.i433.sroa.10.09343, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.16.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.16.0.copyload, %bb29.i ], [ %history.i.i433.sroa.13.09344, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.19.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.19.0.copyload, %bb29.i ], [ %history.i.i433.sroa.16.09345, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.22.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.22.0.copyload, %bb29.i ], [ %history.i.i433.sroa.19.09346, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.26.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.26.0.copyload, %bb29.i ], [ %history.i.i433.sroa.22.09347, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.29.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.29.0.copyload, %bb29.i ], [ %history.i.i433.sroa.26.09348, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.32.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.32.0.copyload, %bb29.i ], [ %history.i.i433.sroa.29.09349, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.35.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.35.0.copyload, %bb29.i ], [ %history.i.i433.sroa.32.09350, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  %history.i.i433.sroa.38.0.lcssa = phi <4 x i32> [ %history.i.i433.sroa.38.0.copyload, %bb29.i ], [ %history.i.i433.sroa.35.09351, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3415 ], !dbg !17015
  br i1 %_20.i.i4539340.not, label %bb12.i446.loopexit, label %bb34.i.lr.ph, !dbg !17016

bb34.i.lr.ph:                                     ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i454
  %_60.i = load i32, ptr %685, align 4
  %_67.i492 = load i32, ptr %701, align 4
  br label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3072, !dbg !17016

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3072: ; preds = %bb34.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420
  %827 = phi <4 x float> [ %.lcssa1164912371, %bb34.i.lr.ph ], [ %860, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420 ]
  %main_cursor.sroa.0.1.i4579380 = phi i32 [ %main_cursor.sroa.0.0.i4499451, %bb34.i.lr.ph ], [ %spec.store.select7.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420 ]
  %ring_cursor.sroa.0.1.i4569379 = phi i32 [ %ring_cursor.sroa.0.0.i4489450, %bb34.i.lr.ph ], [ %spec.store.select8.i, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420 ]
  %iter1.sroa.0.0.i4559378 = phi i32 [ 0, %bb34.i.lr.ph ], [ %828, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420 ]
  %828 = add nuw nsw i32 %iter1.sroa.0.0.i4559378, 1, !dbg !17023
  %_43.i = add nuw nsw i32 %iter1.sroa.0.0.i4559378, %iter.sroa.0.0.i9448, !dbg !17029
  %base.i459 = shl i32 %_43.i, 2, !dbg !17029
  %_89.i466.idx = shl i32 %iter1.sroa.0.0.i4559378, 4, !dbg !17031
  %_89.i466 = getelementptr inbounds nuw i8, ptr %peaks_left.i434, i32 %_89.i466.idx, !dbg !17031
  %lanes.i3065.sroa.0.0.copyload = load <16 x i8>, ptr %_89.i466, align 4, !dbg !17044, !alias.scope !17049, !noalias !17053
  %_4.i3756 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %lanes.i3065.sroa.0.0.copyload, <16 x i8> %lanes.i3065.sroa.0.0.copyload, <16 x i8> %684), !dbg !17057
  %_90.i469 = icmp ugt i32 %base.i459, %left_io.1, !dbg !17063
  br i1 %_90.i469, label %bb38.i, label %bb39.i470, !dbg !17063, !prof !902

bb39.i470:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3072
  %_93.i = sub nuw nsw i32 %left_io.1, %base.i459, !dbg !17068
  %_97.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i459, !dbg !17069
  %_8.i3059 = icmp samesign ugt i32 %_93.i, 3, !dbg !17074
  br i1 %_8.i3059, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3063, label %bb2.i3060, !dbg !17074, !prof !1153

bb2.i3060:                                        ; preds = %bb39.i470
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !17079, !noalias !17080
  unreachable, !dbg !17079

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3063: ; preds = %bb39.i470
  %lanes.i3056.sroa.0.0.copyload = load <4 x i32>, ptr %_97.i, align 4, !dbg !17084, !alias.scope !17088, !noalias !17092
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17094), !dbg !17097
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17098), !dbg !17097
  %width.i.i = load i32, ptr %686, align 4, !dbg !17100, !alias.scope !17102, !noalias !17103, !noundef !10
  %829 = bitcast <16 x i8> %_4.i3756 to <4 x float>, !dbg !17111
  %830 = fcmp olt <4 x float> %_8.i.i4618099, %829, !dbg !17116
  %831 = sext <4 x i1> %830 to <4 x i32>, !dbg !17116
  %832 = fdiv <4 x float> %_8.i.i4618099, %829, !dbg !17117
  %833 = bitcast <4 x float> %832 to <16 x i8>, !dbg !17121
  %834 = bitcast <4 x i32> %831 to <16 x i8>, !dbg !17125
  %_4.i3757 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %833, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %834), !dbg !17126
  %_158.1.i.i = load i32, ptr %687, align 4, !dbg !17127, !alias.scope !17102, !noalias !17103, !noundef !10
  %_22.i.i475 = mul i32 %width.i.i, %ring_cursor.sroa.0.1.i4569379, !dbg !17128
  %_90.i.i = icmp ugt i32 %_22.i.i475, %_158.1.i.i, !dbg !17129
  br i1 %_90.i.i, label %bb34.i.i, label %bb35.i.i, !dbg !17129, !prof !902

bb35.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3063
  %_93.i.i = sub nuw i32 %_158.1.i.i, %_22.i.i475, !dbg !17132
  %_8.i3432 = icmp samesign ugt i32 %_93.i.i, 3, !dbg !17133
  br i1 %_8.i3432, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3435, label %bb2.i3433, !dbg !17133, !prof !1153

bb2.i3433:                                        ; preds = %bb35.i.i
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_93.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !17138, !noalias !17139
  unreachable, !dbg !17138

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3435: ; preds = %bb35.i.i
  %_158.0.i.i = load ptr, ptr %688, align 4, !dbg !17127, !alias.scope !17102, !noalias !17103, !nonnull !10, !noundef !10
  %_97.i.i = getelementptr inbounds nuw float, ptr %_158.0.i.i, i32 %_22.i.i475, !dbg !17143
  store <16 x i8> %_4.i3757, ptr %_97.i.i, align 4, !dbg !17145, !alias.scope !17149, !noalias !17153
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17155), !dbg !17158
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17159), !dbg !17158
  %width.i1212 = load i32, ptr %686, align 4, !dbg !17161, !alias.scope !17155, !noalias !17163, !noundef !10
  %835 = icmp eq i32 %width.i1212, 0, !dbg !17164
  br i1 %835, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1317, label %bb29.i1218.lr.ph, !dbg !17164

bb29.i1218.lr.ph:                                 ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3435
  %_126.1.i1223 = load i32, ptr %44, align 4, !alias.scope !17155, !noalias !17163, !noundef !10
  %_126.0.i1227 = load ptr, ptr %43, align 4, !nonnull !10
  %836 = add i32 %ring_cursor.sroa.0.1.i4569379, 1
  %_21.not.i1232 = icmp ult i32 %836, %_60.i
  %837 = select i1 %_21.not.i1232, i32 0, i32 %_60.i
  %start1.sroa.0.0.i1233 = sub nuw i32 %836, %837
  %_128.1.i1236 = load i32, ptr %687, align 4
  %_128.0.i1240 = load ptr, ptr %688, align 4, !nonnull !10
  %_130.1.i1241 = load i32, ptr %689, align 4
  %_130.0.i1245 = load ptr, ptr %690, align 4, !nonnull !10
  %_132.1.i1248 = load i32, ptr %691, align 4
  %_132.0.i1252 = load ptr, ptr %692, align 4, !nonnull !10
  %_43.i1265 = mul i32 %width.i1212, %start1.sroa.0.0.i1233
  br label %bb29.i1218, !dbg !17164

bb29.i1218:                                       ; preds = %bb29.i1218.lr.ph, %bb28.i1280
  %iter.sroa.0.0.idx.i12169371 = phi i32 [ 0, %bb29.i1218.lr.ph ], [ %iter.sroa.0.0.add.i1221, %bb28.i1280 ]
  %iter.sroa.4.0.i12159370 = phi i32 [ 0, %bb29.i1218.lr.ph ], [ %_102.0.i1222, %bb28.i1280 ]
  %iter.sroa.7.0.i12149369 = phi i32 [ %width.i1212, %bb29.i1218.lr.ph ], [ %838, %bb28.i1280 ]
  %iter.sroa.0.0.ptr.i12179372 = getelementptr inbounds nuw i8, ptr %scratch.i, i32 %iter.sroa.0.0.idx.i12169371, !dbg !17166
  %838 = add i32 %iter.sroa.7.0.i12149369, -1, !dbg !17166
  %_109.i1219 = icmp eq i32 %iter.sroa.0.0.idx.i12169371, 32, !dbg !17167
  br i1 %_109.i1219, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1317, label %bb33.i1220, !dbg !17171

bb33.i1220:                                       ; preds = %bb29.i1218
  %iter.sroa.0.0.add.i1221 = add nuw nsw i32 %iter.sroa.0.0.idx.i12169371, 4, !dbg !17172
  %_102.0.i1222 = add nuw nsw i32 %iter.sroa.4.0.i12159370, 1, !dbg !17174
  %exitcond11141.not = icmp eq i32 %iter.sroa.4.0.i12159370, %_126.1.i1223, !dbg !17175
  br i1 %exitcond11141.not, label %panic.i1225, label %bb2.i1226, !dbg !17175

bb2.i1226:                                        ; preds = %bb33.i1220
  %839 = getelementptr inbounds nuw %LaneShape, ptr %_126.0.i1227, i32 %iter.sroa.4.0.i12159370, !dbg !17175
  %shape.i1228 = load i32, ptr %839, align 4, !dbg !17175, !noalias !17176, !noundef !10
  %840 = getelementptr inbounds nuw i8, ptr %839, i32 4, !dbg !17175
  %shape3.i1229 = load i32, ptr %840, align 4, !dbg !17175, !noalias !17176, !noundef !10
  %841 = add i32 %shape3.i1229, %ring_cursor.sroa.0.1.i4569379, !dbg !17177
  %_18.not.i1230 = icmp ult i32 %841, %_60.i, !dbg !17178
  %842 = select i1 %_18.not.i1230, i32 0, i32 %_60.i, !dbg !17178
  %spec.select.i1231 = sub nuw i32 %841, %842, !dbg !17178
  %_25.i1234 = mul i32 %spec.select.i1231, %width.i1212, !dbg !17179
  %_24.i1235 = add i32 %_25.i1234, %iter.sroa.4.0.i12159370, !dbg !17179
  %_28.i1237 = icmp ult i32 %_24.i1235, %_128.1.i1236, !dbg !17180
  br i1 %_28.i1237, label %bb9.i1239, label %panic5.i1238, !dbg !17180

panic.i1225:                                      ; preds = %bb33.i1220
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_126.1.i1223, i32 noundef %_126.1.i1223, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_12856b5f9033764dbf23e04eb77c5c46) #33, !dbg !17175, !noalias !17176
  unreachable, !dbg !17175

bb9.i1239:                                        ; preds = %bb2.i1226
  %843 = getelementptr inbounds nuw float, ptr %_128.0.i1240, i32 %_24.i1235, !dbg !17180
  %844 = load float, ptr %843, align 4, !dbg !17180, !noalias !17176, !noundef !10
  %exitcond11142.not = icmp eq i32 %iter.sroa.4.0.i12159370, %_130.1.i1241, !dbg !17181
  br i1 %exitcond11142.not, label %panic6.i1243, label %bb10.i1244, !dbg !17181

panic5.i1238:                                     ; preds = %bb2.i1226
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_24.i1235, i32 noundef %_128.1.i1236, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70ebf0cf1c90c6161926611ccf249a32) #33, !dbg !17180, !noalias !17176
  unreachable, !dbg !17180

bb10.i1244:                                       ; preds = %bb9.i1239
  %845 = getelementptr inbounds nuw i32, ptr %_130.0.i1245, i32 %iter.sroa.4.0.i12159370, !dbg !17181
  %_30.i1246 = load i32, ptr %845, align 4, !dbg !17181, !noalias !17176, !noundef !10
  %846 = icmp eq i32 %_30.i1246, 0, !dbg !17182
  br i1 %846, label %bb14.i1255, label %bb12.i1247, !dbg !17182

panic6.i1243:                                     ; preds = %bb9.i1239
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_130.1.i1241, i32 noundef %_130.1.i1241, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_023d591aaad7f5cf482ac80a9401eca1) #33, !dbg !17181, !noalias !17176
  unreachable, !dbg !17181

bb12.i1247:                                       ; preds = %bb10.i1244
  %_35.i1249 = icmp ult i32 %iter.sroa.4.0.i12159370, %_132.1.i1248, !dbg !17183
  br i1 %_35.i1249, label %bb13.i1251, label %panic7.i1250, !dbg !17183

bb14.i1255:                                       ; preds = %bb34.i1316, %bb13.i1251, %bb10.i1244
  %newest.sroa.0.0.i1256 = phi float [ %844, %bb10.i1244 ], [ %_33.i1253, %bb34.i1316 ], [ %844, %bb13.i1251 ], !dbg !17184
  %exitcond11143.not = icmp eq i32 %iter.sroa.4.0.i12159370, %_132.1.i1248, !dbg !17185
  br i1 %exitcond11143.not, label %panic8.i1259, label %bb15.i1260, !dbg !17185

bb13.i1251:                                       ; preds = %bb12.i1247
  %847 = getelementptr inbounds nuw float, ptr %_132.0.i1252, i32 %iter.sroa.4.0.i12159370, !dbg !17183
  %_33.i1253 = load float, ptr %847, align 4, !dbg !17183, !noalias !17176, !noundef !10
  %_116.i1254 = fcmp olt float %_33.i1253, %844, !dbg !17186
  br i1 %_116.i1254, label %bb34.i1316, label %bb14.i1255, !dbg !17186

panic7.i1250:                                     ; preds = %bb12.i1247
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %iter.sroa.4.0.i12159370, i32 noundef %_132.1.i1248, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a228cac3279aae3a34886f59f51fbe9e) #33, !dbg !17183, !noalias !17176
  unreachable, !dbg !17183

bb34.i1316:                                       ; preds = %bb13.i1251
  br label %bb14.i1255, !dbg !17188

bb15.i1260:                                       ; preds = %bb14.i1255
  %848 = getelementptr inbounds nuw float, ptr %_132.0.i1252, i32 %iter.sroa.4.0.i12159370, !dbg !17185
  store float %newest.sroa.0.0.i1256, ptr %848, align 4, !dbg !17185, !noalias !17176
  %_40.i1262 = add i32 %_30.i1246, 1, !dbg !17189
  %complete.i1263 = icmp eq i32 %_40.i1262, %shape.i1228, !dbg !17189
  br i1 %complete.i1263, label %bb19.i1285, label %bb17.i1264, !dbg !17190

panic8.i1259:                                     ; preds = %bb14.i1255
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_132.1.i1248, i32 noundef %_132.1.i1248, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1fa5d88c1a2cc292a2767c48606a38fe) #33, !dbg !17185, !noalias !17176
  unreachable, !dbg !17185

bb17.i1264:                                       ; preds = %bb15.i1260
  %_42.i1266 = add i32 %iter.sroa.4.0.i12159370, %_43.i1265, !dbg !17191
  %_45.i1268 = icmp ult i32 %_42.i1266, %_128.1.i1236, !dbg !17192
  br i1 %_45.i1268, label %bb27.i1278, label %panic9.i1269, !dbg !17192

panic9.i1269:                                     ; preds = %bb17.i1264
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_42.i1266, i32 noundef %_128.1.i1236, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_70d40ae2302f3ea19fa0c4a65fe82786) #33, !dbg !17192, !noalias !17176
  unreachable, !dbg !17192

bb27.i1278:                                       ; preds = %bb17.i1264
  %849 = getelementptr inbounds nuw float, ptr %_128.0.i1240, i32 %_42.i1266, !dbg !17192
  %_41.i1272 = load float, ptr %849, align 4, !dbg !17192, !noalias !17176, !noundef !10
  %_117.i1273 = fcmp olt float %_41.i1272, %newest.sroa.0.0.i1256, !dbg !17193
  %newest.sroa.0.1.i1274 = select i1 %_117.i1273, float %_41.i1272, float %newest.sroa.0.0.i1256, !dbg !17193
  store float %newest.sroa.0.1.i1274, ptr %iter.sroa.0.0.ptr.i12179372, align 4, !dbg !17195, !alias.scope !17159, !noalias !17196
  br label %bb28.i1280, !dbg !17197

bb28.i1280:                                       ; preds = %bb22.i1313, %bb19.i1285, %bb27.i1278
  %storemerge8102 = phi i32 [ %_40.i1262, %bb27.i1278 ], [ 0, %bb19.i1285 ], [ 0, %bb22.i1313 ], !dbg !17198
  store i32 %storemerge8102, ptr %845, align 4, !dbg !17198, !noalias !17176
  %850 = icmp eq i32 %838, 0, !dbg !17164
  br i1 %850, label %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1317, label %bb29.i1218, !dbg !17164

bb19.i1285:                                       ; preds = %bb15.i1260
  store float %newest.sroa.0.0.i1256, ptr %iter.sroa.0.0.ptr.i12179372, align 4, !dbg !17195, !alias.scope !17159, !noalias !17196
  %_118.i12919365.not = icmp eq i32 %shape.i1228, 0, !dbg !17199
  br i1 %_118.i12919365.not, label %bb28.i1280, label %bb40.i1298.preheader, !dbg !17203

bb40.i1298.preheader:                             ; preds = %bb19.i1285
  %851 = load float, ptr %843, align 4, !dbg !17204, !noalias !17176, !noundef !10
  br label %bb40.i1298, !dbg !17205

bb40.i1298:                                       ; preds = %bb40.i1298.preheader, %bb22.i1313
  %iter2.sroa.0.0.i12909368 = phi i32 [ %_119.i1299, %bb22.i1313 ], [ 0, %bb40.i1298.preheader ]
  %suffix.sroa.0.0.i12899367 = phi float [ %suffix.sroa.0.1.i1309, %bb22.i1313 ], [ %851, %bb40.i1298.preheader ]
  %end.sroa.0.1.i12889366 = phi i32 [ %854, %bb22.i1313 ], [ %spec.select.i1231, %bb40.i1298.preheader ]
  %_54.i1300 = mul i32 %end.sroa.0.1.i12889366, %width.i1212, !dbg !17206
  %_53.i1301 = add i32 %_54.i1300, %iter.sroa.4.0.i12159370, !dbg !17206
  %_57.i1303 = icmp ult i32 %_53.i1301, %_128.1.i1236, !dbg !17205
  br i1 %_57.i1303, label %bb22.i1313, label %panic13.i1304, !dbg !17205

panic13.i1304:                                    ; preds = %bb40.i1298
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_53.i1301, i32 noundef %_128.1.i1236, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_a271bbb7fd83c3605ea58a06b7065fa4) #33, !dbg !17205, !noalias !17176
  unreachable, !dbg !17205

bb22.i1313:                                       ; preds = %bb40.i1298
  %_119.i1299 = add nuw i32 %iter2.sroa.0.0.i12909368, 1, !dbg !17207
  %852 = getelementptr inbounds nuw float, ptr %_128.0.i1240, i32 %_53.i1301, !dbg !17205
  %_52.i1307 = load float, ptr %852, align 4, !dbg !17205, !noalias !17176, !noundef !10
  %_121.i1308 = fcmp olt float %suffix.sroa.0.0.i12899367, %_52.i1307, !dbg !17210
  %suffix.sroa.0.1.i1309 = select i1 %_121.i1308, float %suffix.sroa.0.0.i12899367, float %_52.i1307, !dbg !17210
  store float %suffix.sroa.0.1.i1309, ptr %852, align 4, !dbg !17212, !noalias !17176
  %853 = icmp eq i32 %end.sroa.0.1.i12889366, 0, !dbg !17213
  %spec.store.select.i1315 = select i1 %853, i32 %_60.i, i32 %end.sroa.0.1.i12889366, !dbg !17213
  %854 = add i32 %spec.store.select.i1315, -1, !dbg !17214
  %exitcond11140.not = icmp eq i32 %_119.i1299, %shape.i1228, !dbg !17199
  br i1 %exitcond11140.not, label %bb28.i1280, label %bb40.i1298, !dbg !17203

_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1317: ; preds = %bb29.i1218, %bb28.i1280, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3435
  %lanes.i3049.sroa.0.0.copyload = load <4 x float>, ptr %scratch.i, align 4, !dbg !17215, !alias.scope !17220, !noalias !17224
  %855 = fmul <4 x float> %lanes.i3049.sroa.0.0.copyload, splat (float 1.638400e+04), !dbg !17228
  %856 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %855), !dbg !17232
  %857 = fmul <4 x float> %856, splat (float 0x3F10000000000000), !dbg !17236
  %858 = icmp eq i32 %width.i.i, 0, !dbg !17240
  %_163.1.i.i.pre = load i32, ptr %693, align 4, !dbg !17242, !alias.scope !17102, !noalias !17103
  br i1 %858, label %bb53.i.i, label %bb36.i.i.lr.ph, !dbg !17240

bb36.i.i.lr.ph:                                   ; preds = %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1317
  %_159.1.i.i = load i32, ptr %44, align 4, !alias.scope !17102, !noalias !17103, !noundef !10
  %_159.0.i.i = load ptr, ptr %43, align 4, !nonnull !10
  %_161.0.i.i = load ptr, ptr %694, align 4, !nonnull !10
  %exitcond11146.not = icmp eq i32 %_159.1.i.i, 0, !dbg !17243
  br i1 %exitcond11146.not, label %panic.i.i, label %bb14.i.i, !dbg !17243

bb34.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3063
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i475, i32 noundef %_158.1.i.i, i32 noundef %_158.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_db7c42041d7aaaba4839906f37294547) #33, !dbg !17244, !noalias !17245
  unreachable, !dbg !17244

bb53.i.i.loopexit:                                ; preds = %bb18.i.i.7, %bb18.i.i.6, %bb18.i.i.5, %bb18.i.i.4, %bb18.i.i.3, %bb18.i.i.2, %bb18.i.i.1, %bb18.i.i
  %lanes.i3042.sroa.0.0.copyload.pre = load <4 x float>, ptr %scratch.i, align 4, !dbg !17246, !alias.scope !17251, !noalias !17255
  br label %bb53.i.i, !dbg !17259

bb53.i.i:                                         ; preds = %bb53.i.i.loopexit, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1317
  %lanes.i3042.sroa.0.0.copyload = phi <4 x float> [ %lanes.i3042.sroa.0.0.copyload.pre, %bb53.i.i.loopexit ], [ %lanes.i3049.sroa.0.0.copyload, %_RNvCsjLJhryqjeDL_17true_peak_limiter15sliding_minimum.exit1317 ], !dbg !17246
  %859 = fadd <4 x float> %857, %827, !dbg !17260
  %860 = fsub <4 x float> %859, %lanes.i3042.sroa.0.0.copyload, !dbg !17264
  %_123.i.i = icmp ugt i32 %_22.i.i475, %_163.1.i.i.pre, !dbg !17268
  br i1 %_123.i.i, label %bb41.i.i, label %bb42.i.i, !dbg !17268, !prof !902

bb14.i.i:                                         ; preds = %bb36.i.i.lr.ph
  %861 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 8, !dbg !17243
  %_42.i.i482 = load i32, ptr %861, align 4, !dbg !17243, !noalias !17271, !noundef !10
  %862 = add i32 %_42.i.i482, %ring_cursor.sroa.0.1.i4569379, !dbg !17272
  %_45.not.i.i = icmp ult i32 %862, %_60.i, !dbg !17273
  %863 = select i1 %_45.not.i.i, i32 0, i32 %_60.i, !dbg !17273
  %spec.select.i.i = sub nuw i32 %862, %863, !dbg !17273
  %_49.i.i483 = mul i32 %spec.select.i.i, %width.i.i, !dbg !17274
  %_51.i.i = icmp ult i32 %_49.i.i483, %_163.1.i.i.pre, !dbg !17275
  br i1 %_51.i.i, label %bb18.i.i, label %panic1.i.i, !dbg !17275

panic.i.i:                                        ; preds = %bb36.i.i.7, %bb36.i.i.6, %bb36.i.i.5, %bb36.i.i.4, %bb36.i.i.3, %bb36.i.i.2, %bb36.i.i.1, %bb36.i.i.lr.ph
  %_159.1.i.i.lcssa.ph = phi i32 [ 7, %bb36.i.i.7 ], [ 6, %bb36.i.i.6 ], [ 5, %bb36.i.i.5 ], [ 4, %bb36.i.i.4 ], [ 3, %bb36.i.i.3 ], [ 2, %bb36.i.i.2 ], [ 1, %bb36.i.i.1 ], [ 0, %bb36.i.i.lr.ph ]
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_159.1.i.i.lcssa.ph, i32 noundef %_159.1.i.i.lcssa.ph, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_33746731a929bad63709ddedbca82f5f) #33, !dbg !17243, !noalias !17271
  unreachable, !dbg !17243

bb18.i.i:                                         ; preds = %bb14.i.i
  %864 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_49.i.i483, !dbg !17275
  %_47.i.i = load float, ptr %864, align 4, !dbg !17275, !noalias !17271, !noundef !10
  store float %_47.i.i, ptr %scratch.i, align 4, !dbg !17276, !alias.scope !17098, !noalias !17277
  %865 = icmp eq i32 %width.i.i, 1, !dbg !17240
  br i1 %865, label %bb53.i.i.loopexit, label %bb36.i.i.1, !dbg !17240

bb36.i.i.1:                                       ; preds = %bb18.i.i
  %exitcond11146.1.not = icmp eq i32 %_159.1.i.i, 1, !dbg !17243
  br i1 %exitcond11146.1.not, label %panic.i.i, label %bb14.i.i.1, !dbg !17243

bb14.i.i.1:                                       ; preds = %bb36.i.i.1
  %866 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 20, !dbg !17243
  %_42.i.i482.1 = load i32, ptr %866, align 4, !dbg !17243, !noalias !17271, !noundef !10
  %867 = add i32 %_42.i.i482.1, %ring_cursor.sroa.0.1.i4569379, !dbg !17272
  %_45.not.i.i.1 = icmp ult i32 %867, %_60.i, !dbg !17273
  %868 = select i1 %_45.not.i.i.1, i32 0, i32 %_60.i, !dbg !17273
  %spec.select.i.i.1 = sub nuw i32 %867, %868, !dbg !17273
  %_49.i.i483.1 = mul i32 %spec.select.i.i.1, %width.i.i, !dbg !17274
  %_48.i.i.1 = add i32 %_49.i.i483.1, 1, !dbg !17274
  %_51.i.i.1 = icmp ult i32 %_48.i.i.1, %_163.1.i.i.pre, !dbg !17275
  br i1 %_51.i.i.1, label %bb18.i.i.1, label %panic1.i.i, !dbg !17275

bb18.i.i.1:                                       ; preds = %bb14.i.i.1
  %869 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.1, !dbg !17275
  %_47.i.i.1 = load float, ptr %869, align 4, !dbg !17275, !noalias !17271, !noundef !10
  store float %_47.i.i.1, ptr %iter.sroa.0.0.ptr.i.i9376.1, align 4, !dbg !17276, !alias.scope !17098, !noalias !17277
  %870 = icmp eq i32 %width.i.i, 2, !dbg !17240
  br i1 %870, label %bb53.i.i.loopexit, label %bb36.i.i.2, !dbg !17240

bb36.i.i.2:                                       ; preds = %bb18.i.i.1
  %exitcond11146.2.not = icmp eq i32 %_159.1.i.i, 2, !dbg !17243
  br i1 %exitcond11146.2.not, label %panic.i.i, label %bb14.i.i.2, !dbg !17243

bb14.i.i.2:                                       ; preds = %bb36.i.i.2
  %871 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 32, !dbg !17243
  %_42.i.i482.2 = load i32, ptr %871, align 4, !dbg !17243, !noalias !17271, !noundef !10
  %872 = add i32 %_42.i.i482.2, %ring_cursor.sroa.0.1.i4569379, !dbg !17272
  %_45.not.i.i.2 = icmp ult i32 %872, %_60.i, !dbg !17273
  %873 = select i1 %_45.not.i.i.2, i32 0, i32 %_60.i, !dbg !17273
  %spec.select.i.i.2 = sub nuw i32 %872, %873, !dbg !17273
  %_49.i.i483.2 = mul i32 %spec.select.i.i.2, %width.i.i, !dbg !17274
  %_48.i.i.2 = add i32 %_49.i.i483.2, 2, !dbg !17274
  %_51.i.i.2 = icmp ult i32 %_48.i.i.2, %_163.1.i.i.pre, !dbg !17275
  br i1 %_51.i.i.2, label %bb18.i.i.2, label %panic1.i.i, !dbg !17275

bb18.i.i.2:                                       ; preds = %bb14.i.i.2
  %874 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.2, !dbg !17275
  %_47.i.i.2 = load float, ptr %874, align 4, !dbg !17275, !noalias !17271, !noundef !10
  store float %_47.i.i.2, ptr %iter.sroa.0.0.ptr.i.i9376.2, align 4, !dbg !17276, !alias.scope !17098, !noalias !17277
  %875 = icmp eq i32 %width.i.i, 3, !dbg !17240
  br i1 %875, label %bb53.i.i.loopexit, label %bb36.i.i.3, !dbg !17240

bb36.i.i.3:                                       ; preds = %bb18.i.i.2
  %exitcond11146.3.not = icmp eq i32 %_159.1.i.i, 3, !dbg !17243
  br i1 %exitcond11146.3.not, label %panic.i.i, label %bb14.i.i.3, !dbg !17243

bb14.i.i.3:                                       ; preds = %bb36.i.i.3
  %876 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 44, !dbg !17243
  %_42.i.i482.3 = load i32, ptr %876, align 4, !dbg !17243, !noalias !17271, !noundef !10
  %877 = add i32 %_42.i.i482.3, %ring_cursor.sroa.0.1.i4569379, !dbg !17272
  %_45.not.i.i.3 = icmp ult i32 %877, %_60.i, !dbg !17273
  %878 = select i1 %_45.not.i.i.3, i32 0, i32 %_60.i, !dbg !17273
  %spec.select.i.i.3 = sub nuw i32 %877, %878, !dbg !17273
  %_49.i.i483.3 = mul i32 %spec.select.i.i.3, %width.i.i, !dbg !17274
  %_48.i.i.3 = add i32 %_49.i.i483.3, 3, !dbg !17274
  %_51.i.i.3 = icmp ult i32 %_48.i.i.3, %_163.1.i.i.pre, !dbg !17275
  br i1 %_51.i.i.3, label %bb18.i.i.3, label %panic1.i.i, !dbg !17275

bb18.i.i.3:                                       ; preds = %bb14.i.i.3
  %879 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.3, !dbg !17275
  %_47.i.i.3 = load float, ptr %879, align 4, !dbg !17275, !noalias !17271, !noundef !10
  store float %_47.i.i.3, ptr %iter.sroa.0.0.ptr.i.i9376.3, align 4, !dbg !17276, !alias.scope !17098, !noalias !17277
  %880 = icmp eq i32 %width.i.i, 4, !dbg !17240
  br i1 %880, label %bb53.i.i.loopexit, label %bb36.i.i.4, !dbg !17240

bb36.i.i.4:                                       ; preds = %bb18.i.i.3
  %exitcond11146.4.not = icmp eq i32 %_159.1.i.i, 4, !dbg !17243
  br i1 %exitcond11146.4.not, label %panic.i.i, label %bb14.i.i.4, !dbg !17243

bb14.i.i.4:                                       ; preds = %bb36.i.i.4
  %881 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 56, !dbg !17243
  %_42.i.i482.4 = load i32, ptr %881, align 4, !dbg !17243, !noalias !17271, !noundef !10
  %882 = add i32 %_42.i.i482.4, %ring_cursor.sroa.0.1.i4569379, !dbg !17272
  %_45.not.i.i.4 = icmp ult i32 %882, %_60.i, !dbg !17273
  %883 = select i1 %_45.not.i.i.4, i32 0, i32 %_60.i, !dbg !17273
  %spec.select.i.i.4 = sub nuw i32 %882, %883, !dbg !17273
  %_49.i.i483.4 = mul i32 %spec.select.i.i.4, %width.i.i, !dbg !17274
  %_48.i.i.4 = add i32 %_49.i.i483.4, 4, !dbg !17274
  %_51.i.i.4 = icmp ult i32 %_48.i.i.4, %_163.1.i.i.pre, !dbg !17275
  br i1 %_51.i.i.4, label %bb18.i.i.4, label %panic1.i.i, !dbg !17275

bb18.i.i.4:                                       ; preds = %bb14.i.i.4
  %884 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.4, !dbg !17275
  %_47.i.i.4 = load float, ptr %884, align 4, !dbg !17275, !noalias !17271, !noundef !10
  store float %_47.i.i.4, ptr %iter.sroa.0.0.ptr.i.i9376.4, align 4, !dbg !17276, !alias.scope !17098, !noalias !17277
  %885 = icmp eq i32 %width.i.i, 5, !dbg !17240
  br i1 %885, label %bb53.i.i.loopexit, label %bb36.i.i.5, !dbg !17240

bb36.i.i.5:                                       ; preds = %bb18.i.i.4
  %exitcond11146.5.not = icmp eq i32 %_159.1.i.i, 5, !dbg !17243
  br i1 %exitcond11146.5.not, label %panic.i.i, label %bb14.i.i.5, !dbg !17243

bb14.i.i.5:                                       ; preds = %bb36.i.i.5
  %886 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 68, !dbg !17243
  %_42.i.i482.5 = load i32, ptr %886, align 4, !dbg !17243, !noalias !17271, !noundef !10
  %887 = add i32 %_42.i.i482.5, %ring_cursor.sroa.0.1.i4569379, !dbg !17272
  %_45.not.i.i.5 = icmp ult i32 %887, %_60.i, !dbg !17273
  %888 = select i1 %_45.not.i.i.5, i32 0, i32 %_60.i, !dbg !17273
  %spec.select.i.i.5 = sub nuw i32 %887, %888, !dbg !17273
  %_49.i.i483.5 = mul i32 %spec.select.i.i.5, %width.i.i, !dbg !17274
  %_48.i.i.5 = add i32 %_49.i.i483.5, 5, !dbg !17274
  %_51.i.i.5 = icmp ult i32 %_48.i.i.5, %_163.1.i.i.pre, !dbg !17275
  br i1 %_51.i.i.5, label %bb18.i.i.5, label %panic1.i.i, !dbg !17275

bb18.i.i.5:                                       ; preds = %bb14.i.i.5
  %889 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.5, !dbg !17275
  %_47.i.i.5 = load float, ptr %889, align 4, !dbg !17275, !noalias !17271, !noundef !10
  store float %_47.i.i.5, ptr %iter.sroa.0.0.ptr.i.i9376.5, align 4, !dbg !17276, !alias.scope !17098, !noalias !17277
  %890 = icmp eq i32 %width.i.i, 6, !dbg !17240
  br i1 %890, label %bb53.i.i.loopexit, label %bb36.i.i.6, !dbg !17240

bb36.i.i.6:                                       ; preds = %bb18.i.i.5
  %exitcond11146.6.not = icmp eq i32 %_159.1.i.i, 6, !dbg !17243
  br i1 %exitcond11146.6.not, label %panic.i.i, label %bb14.i.i.6, !dbg !17243

bb14.i.i.6:                                       ; preds = %bb36.i.i.6
  %891 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 80, !dbg !17243
  %_42.i.i482.6 = load i32, ptr %891, align 4, !dbg !17243, !noalias !17271, !noundef !10
  %892 = add i32 %_42.i.i482.6, %ring_cursor.sroa.0.1.i4569379, !dbg !17272
  %_45.not.i.i.6 = icmp ult i32 %892, %_60.i, !dbg !17273
  %893 = select i1 %_45.not.i.i.6, i32 0, i32 %_60.i, !dbg !17273
  %spec.select.i.i.6 = sub nuw i32 %892, %893, !dbg !17273
  %_49.i.i483.6 = mul i32 %spec.select.i.i.6, %width.i.i, !dbg !17274
  %_48.i.i.6 = add i32 %_49.i.i483.6, 6, !dbg !17274
  %_51.i.i.6 = icmp ult i32 %_48.i.i.6, %_163.1.i.i.pre, !dbg !17275
  br i1 %_51.i.i.6, label %bb18.i.i.6, label %panic1.i.i, !dbg !17275

bb18.i.i.6:                                       ; preds = %bb14.i.i.6
  %894 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.6, !dbg !17275
  %_47.i.i.6 = load float, ptr %894, align 4, !dbg !17275, !noalias !17271, !noundef !10
  store float %_47.i.i.6, ptr %iter.sroa.0.0.ptr.i.i9376.6, align 4, !dbg !17276, !alias.scope !17098, !noalias !17277
  %895 = icmp eq i32 %width.i.i, 7, !dbg !17240
  br i1 %895, label %bb53.i.i.loopexit, label %bb36.i.i.7, !dbg !17240

bb36.i.i.7:                                       ; preds = %bb18.i.i.6
  %exitcond11146.7.not = icmp eq i32 %_159.1.i.i, 7, !dbg !17243
  br i1 %exitcond11146.7.not, label %panic.i.i, label %bb14.i.i.7, !dbg !17243

bb14.i.i.7:                                       ; preds = %bb36.i.i.7
  %896 = getelementptr inbounds nuw i8, ptr %_159.0.i.i, i32 92, !dbg !17243
  %_42.i.i482.7 = load i32, ptr %896, align 4, !dbg !17243, !noalias !17271, !noundef !10
  %897 = add i32 %_42.i.i482.7, %ring_cursor.sroa.0.1.i4569379, !dbg !17272
  %_45.not.i.i.7 = icmp ult i32 %897, %_60.i, !dbg !17273
  %898 = select i1 %_45.not.i.i.7, i32 0, i32 %_60.i, !dbg !17273
  %spec.select.i.i.7 = sub nuw i32 %897, %898, !dbg !17273
  %_49.i.i483.7 = mul i32 %spec.select.i.i.7, %width.i.i, !dbg !17274
  %_48.i.i.7 = add i32 %_49.i.i483.7, 7, !dbg !17274
  %_51.i.i.7 = icmp ult i32 %_48.i.i.7, %_163.1.i.i.pre, !dbg !17275
  br i1 %_51.i.i.7, label %bb18.i.i.7, label %panic1.i.i, !dbg !17275

bb18.i.i.7:                                       ; preds = %bb14.i.i.7
  %899 = getelementptr inbounds nuw float, ptr %_161.0.i.i, i32 %_48.i.i.7, !dbg !17275
  %_47.i.i.7 = load float, ptr %899, align 4, !dbg !17275, !noalias !17271, !noundef !10
  store float %_47.i.i.7, ptr %iter.sroa.0.0.ptr.i.i9376.7, align 4, !dbg !17276, !alias.scope !17098, !noalias !17277
  br label %bb53.i.i.loopexit, !dbg !17240

panic1.i.i:                                       ; preds = %bb14.i.i.7, %bb14.i.i.6, %bb14.i.i.5, %bb14.i.i.4, %bb14.i.i.3, %bb14.i.i.2, %bb14.i.i.1, %bb14.i.i
  %_48.i.i.lcssa.ph = phi i32 [ %_48.i.i.7, %bb14.i.i.7 ], [ %_48.i.i.6, %bb14.i.i.6 ], [ %_48.i.i.5, %bb14.i.i.5 ], [ %_48.i.i.4, %bb14.i.i.4 ], [ %_48.i.i.3, %bb14.i.i.3 ], [ %_48.i.i.2, %bb14.i.i.2 ], [ %_48.i.i.1, %bb14.i.i.1 ], [ %_49.i.i483, %bb14.i.i ]
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_48.i.i.lcssa.ph, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_0e76a26fbb751dfb8cf8a069b5b0d39a) #33, !dbg !17275, !noalias !17271
  unreachable, !dbg !17275

bb42.i.i:                                         ; preds = %bb53.i.i
  %_126.i.i = sub nuw i32 %_163.1.i.i.pre, %_22.i.i475, !dbg !17278
  %_8.i3427 = icmp samesign ugt i32 %_126.i.i, 3, !dbg !17279
  br i1 %_8.i3427, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3430, label %bb2.i3428, !dbg !17279, !prof !1153

bb2.i3428:                                        ; preds = %bb42.i.i
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_126.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !17284, !noalias !17285
  unreachable, !dbg !17284

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3430: ; preds = %bb42.i.i
  %_163.0.i.i = load ptr, ptr %694, align 4, !dbg !17242, !alias.scope !17102, !noalias !17103, !nonnull !10, !noundef !10
  %_130.i.i = getelementptr inbounds nuw float, ptr %_163.0.i.i, i32 %_22.i.i475, !dbg !17289
  store <4 x float> %857, ptr %_130.i.i, align 4, !dbg !17291, !alias.scope !17295, !noalias !17299
  %_66.i.i8109 = load <4 x float>, ptr %697, align 16, !dbg !17301
  %900 = fdiv <4 x float> %860, %_62.i.i4868108, !dbg !17302
  %901 = fsub <4 x float> splat (float 1.000000e+00), %900, !dbg !17306
  %902 = fsub <4 x float> %901, %_66.i.i8109, !dbg !17310
  %903 = fmul <4 x float> %_9.i.i4628100, %902, !dbg !17314
  %904 = fadd <4 x float> %_66.i.i8109, %903, !dbg !17318
  %905 = fcmp olt <4 x float> %904, %901, !dbg !17321
  %906 = select <4 x i1> %905, <4 x float> %901, <4 x float> %904, !dbg !17325
  %907 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %906), !dbg !17326
  %908 = fcmp uge <4 x float> %907, splat (float 0x3BC79CA100000000), !dbg !17331
  %909 = bitcast <4 x float> %906 to <4 x i32>, !dbg !17336
  %910 = select <4 x i1> %908, <4 x i32> %909, <4 x i32> zeroinitializer, !dbg !17336
  store <4 x i32> %910, ptr %697, align 16, !dbg !17339
  %911 = bitcast <4 x i32> %910 to <4 x float>, !dbg !17340
  %912 = fsub <4 x float> splat (float 1.000000e+00), %911, !dbg !17344
  %_164.1.i.i = load i32, ptr %698, align 4, !dbg !17345, !alias.scope !17102, !noalias !17103, !noundef !10
  %_74.i.i = mul i32 %width.i.i, %main_cursor.sroa.0.1.i4579380, !dbg !17346
  %_134.i.i = icmp ugt i32 %_74.i.i, %_164.1.i.i, !dbg !17347
  br i1 %_134.i.i, label %bb47.i.i, label %bb48.i.i, !dbg !17347, !prof !902

bb41.i.i:                                         ; preds = %bb53.i.i
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_22.i.i475, i32 noundef %_163.1.i.i.pre, i32 noundef %_163.1.i.i.pre, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c2286ff41a33b8c11aa270fe215441de) #33, !dbg !17350, !noalias !17271
  unreachable, !dbg !17350

bb48.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3430
  %_137.i.i = sub nuw i32 %_164.1.i.i, %_74.i.i, !dbg !17351
  %_8.i3036 = icmp samesign ugt i32 %_137.i.i, 3, !dbg !17352
  br i1 %_8.i3036, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420, label %bb2.i3037, !dbg !17352, !prof !1153

bb2.i3037:                                        ; preds = %bb48.i.i
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_137.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !17357, !noalias !17358
  unreachable, !dbg !17357

bb47.i.i:                                         ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3430
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_74.i.i, i32 noundef %_164.1.i.i, i32 noundef %_164.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_2a3a21efd18b403ec25db545d522c479) #33, !dbg !17362, !noalias !17271
  unreachable, !dbg !17362

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3420: ; preds = %bb48.i.i
  %_164.0.i.i = load ptr, ptr %699, align 4, !dbg !17345, !alias.scope !17102, !noalias !17103, !nonnull !10, !noundef !10
  %_141.i.i = getelementptr inbounds nuw float, ptr %_164.0.i.i, i32 %_74.i.i, !dbg !17363
  %lanes.i3033.sroa.0.0.copyload = load <4 x i32>, ptr %_141.i.i, align 4, !dbg !17365, !alias.scope !17369, !noalias !17373
  store <4 x i32> %lanes.i3056.sroa.0.0.copyload, ptr %_141.i.i, align 4, !dbg !17375, !alias.scope !17380, !noalias !17384
  %913 = bitcast <4 x i32> %lanes.i3033.sroa.0.0.copyload to <4 x float>, !dbg !17388
  %914 = fmul <4 x float> %912, %913, !dbg !17392
  %915 = bitcast <4 x i32> %lanes.i3033.sroa.0.0.copyload to <16 x i8>, !dbg !17393
  %916 = bitcast <4 x float> %914 to <16 x i8>, !dbg !17397
  %_4.i3758 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %915, <16 x i8> %916, <16 x i8> %700), !dbg !17398
  store <16 x i8> %_4.i3758, ptr %_97.i, align 4, !dbg !17399, !alias.scope !17404, !noalias !17408
  %917 = add i32 %main_cursor.sroa.0.1.i4579380, 1, !dbg !17412
  %_65.i = icmp eq i32 %917, %_67.i492, !dbg !17413
  %spec.store.select7.i = select i1 %_65.i, i32 0, i32 %917, !dbg !17413
  %918 = add i32 %ring_cursor.sroa.0.1.i4569379, 1, !dbg !17414
  %_68.i493 = icmp eq i32 %918, %_60.i, !dbg !17415
  %spec.store.select8.i = select i1 %_68.i493, i32 0, i32 %918, !dbg !17415
  %exitcond11150.not = icmp eq i32 %828, %umax11149, !dbg !17416
  br i1 %exitcond11150.not, label %bb12.i446.loopexit, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3072, !dbg !17016

bb38.i:                                           ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane4load.exit3072
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912371, ptr %695, align 16
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i459, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_8b3aff86bf6965bbce4da27800054d27) #33, !dbg !17419, !noalias !17420
  unreachable, !dbg !17419

_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit: ; preds = %bb12.i446.loopexit
  store <4 x i32> %history.i.i433.sroa.7.0.lcssa, ptr %history.i.i433.sroa.7.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.10.0.lcssa, ptr %history.i.i433.sroa.10.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.13.0.lcssa, ptr %history.i.i433.sroa.13.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.16.0.lcssa, ptr %history.i.i433.sroa.16.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.19.0.lcssa, ptr %history.i.i433.sroa.19.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.22.0.lcssa, ptr %history.i.i433.sroa.22.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.26.0.lcssa, ptr %history.i.i433.sroa.26.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.29.0.lcssa, ptr %history.i.i433.sroa.29.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.32.0.lcssa, ptr %history.i.i433.sroa.32.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.35.0.lcssa, ptr %history.i.i433.sroa.35.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x i32> %history.i.i433.sroa.38.0.lcssa, ptr %history.i.i433.sroa.38.0.hot_left.i436.sroa_idx, align 1, !dbg !16563
  store <4 x float> %.lcssa1164912370, ptr %695, align 16
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !17015

_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit, %bb7.i
  %history.i.i433.sroa.0.0.lcssa9423.lcssa = phi <4 x i32> [ %hot_left.i436.promoted, %bb7.i ], [ %history.i.i433.sroa.0.0.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ]
  %ring_cursor.sroa.0.0.i448.lcssa = phi i32 [ %_27.i443, %bb7.i ], [ %ring_cursor.sroa.0.1.i456.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !16508
  %main_cursor.sroa.0.0.i449.lcssa = phi i32 [ %_25.i, %bb7.i ], [ %main_cursor.sroa.0.1.i457.lcssa, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.loopexit ], !dbg !16505
  store <4 x i32> %history.i.i433.sroa.0.0.lcssa9423.lcssa, ptr %hot_left.i436, align 1, !dbg !17015
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %hot_left.i436, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24) #32, !dbg !17421
  store i32 %main_cursor.sroa.0.0.i449.lcssa, ptr %_25, align 4, !dbg !17422, !alias.scope !16490, !noalias !16507
  store i32 %ring_cursor.sroa.0.0.i448.lcssa, ptr %646, align 4, !dbg !17423, !alias.scope !16490, !noalias !16507
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i434), !dbg !17424, !noalias !16512
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i), !dbg !17425, !noalias !16512
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !16487

bb3.i:                                            ; preds = %bb2.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17426), !dbg !17429
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17430), !dbg !17429
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17432), !dbg !17429
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17434), !dbg !17429
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i), !dbg !17436, !noalias !17440
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::load
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E4loadB5_(ptr noalias noundef align 16 captures(none) dereferenceable(368) %hot_left.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #32, !dbg !17442, !noalias !17443
  %919 = getelementptr inbounds nuw i8, ptr %self, i32 784, !dbg !17444
  %920 = load i8, ptr %919, align 16, !dbg !17444, !range !4765, !alias.scope !17426, !noalias !17448, !noundef !10
  %921 = getelementptr inbounds nuw i8, ptr %self, i32 785, !dbg !17449
  %922 = load i8, ptr %921, align 1, !dbg !17449, !range !4765, !alias.scope !17426, !noalias !17448, !noundef !10
  %923 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !17451
  %ring.i = load i32, ptr %923, align 4, !dbg !17451, !alias.scope !17430, !noalias !17453, !noundef !10
  %924 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !17454
  %main.i = load i32, ptr %924, align 4, !dbg !17454, !alias.scope !17430, !noalias !17453, !noundef !10
  %_26.i = load i32, ptr %_25, align 4, !dbg !17456, !alias.scope !17434, !noalias !17458, !noundef !10
  %925 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !17459
  %_27.i = load i32, ptr %925, align 4, !dbg !17459, !alias.scope !17434, !noalias !17458, !noundef !10
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i), !dbg !17461, !noalias !17440
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i, i8 0, i32 1024, i1 false), !noalias !17440
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i), !dbg !17463, !noalias !17440
; call <true_peak_limiter::UniformHot<wide::f32x4_::f32x4>>::new
  call fastcc void @_RNvMs6_CsjLJhryqjeDL_17true_peak_limiterINtB5_10UniformHotNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E3newB5_(ptr noalias noundef align 16 captures(none) dereferenceable(64) %uniform_left.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24, i32 %ring.i, i32 %main.i) #32, !dbg !17465
  %hot_left.i.promoted = load <4 x i32>, ptr %hot_left.i, align 1
  %_106.not.i9717 = icmp eq i32 %frames, 0, !dbg !17466
  br i1 %_106.not.i9717, label %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge, label %bb32.i.lr.ph, !dbg !17466

bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge: ; preds = %bb3.i
  %.phi.trans.insert11247 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16
  %left_phase.i.pre = load i32, ptr %.phi.trans.insert11247, align 16, !dbg !17476, !noalias !17440
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !17466

bb32.i.lr.ph:                                     ; preds = %bb3.i
  %_23.i = trunc nuw i8 %922 to i1, !dbg !17449
  %spec.store.select17.i = select i1 %_23.i, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !17449
  %_22.i = trunc nuw i8 %920 to i1, !dbg !17444
  %link.sroa.0.0.i = select i1 %_22.i, <4 x i32> splat (i32 -1), <4 x i32> zeroinitializer, !dbg !17444
  %d9.i3759 = lshr i32 %frames, 5, !dbg !17477
  %r2.i3760 = and i32 %frames, 31, !dbg !17484
  %_19.not.i3761 = icmp ne i32 %r2.i3760, 0, !dbg !17485
  %926 = zext i1 %_19.not.i3761 to i32, !dbg !17485
  %yield_count.sroa.0.0.i3762 = add nuw nsw i32 %d9.i3759, %926, !dbg !17485
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
  %927 = getelementptr inbounds nuw i8, ptr %self, i32 32
  %928 = getelementptr inbounds nuw i8, ptr %self, i32 48
  %929 = getelementptr inbounds nuw i8, ptr %self, i32 64
  %row1.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 80
  %930 = getelementptr inbounds nuw i8, ptr %self, i32 96
  %931 = getelementptr inbounds nuw i8, ptr %self, i32 112
  %932 = getelementptr inbounds nuw i8, ptr %self, i32 128
  %row3.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 144
  %933 = getelementptr inbounds nuw i8, ptr %self, i32 160
  %934 = getelementptr inbounds nuw i8, ptr %self, i32 176
  %935 = getelementptr inbounds nuw i8, ptr %self, i32 192
  %row5.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 208
  %936 = getelementptr inbounds nuw i8, ptr %self, i32 224
  %937 = getelementptr inbounds nuw i8, ptr %self, i32 240
  %938 = getelementptr inbounds nuw i8, ptr %self, i32 256
  %row7.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 272
  %939 = getelementptr inbounds nuw i8, ptr %self, i32 288
  %940 = getelementptr inbounds nuw i8, ptr %self, i32 304
  %941 = getelementptr inbounds nuw i8, ptr %self, i32 320
  %row9.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 336
  %942 = getelementptr inbounds nuw i8, ptr %self, i32 352
  %943 = getelementptr inbounds nuw i8, ptr %self, i32 368
  %944 = getelementptr inbounds nuw i8, ptr %self, i32 384
  %row11.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 400
  %945 = getelementptr inbounds nuw i8, ptr %self, i32 416
  %946 = getelementptr inbounds nuw i8, ptr %self, i32 432
  %947 = getelementptr inbounds nuw i8, ptr %self, i32 448
  %row13.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 464
  %948 = getelementptr inbounds nuw i8, ptr %self, i32 480
  %949 = getelementptr inbounds nuw i8, ptr %self, i32 496
  %950 = getelementptr inbounds nuw i8, ptr %self, i32 512
  %row15.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 528
  %951 = getelementptr inbounds nuw i8, ptr %self, i32 544
  %952 = getelementptr inbounds nuw i8, ptr %self, i32 560
  %953 = getelementptr inbounds nuw i8, ptr %self, i32 576
  %row17.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 592
  %954 = getelementptr inbounds nuw i8, ptr %self, i32 608
  %955 = getelementptr inbounds nuw i8, ptr %self, i32 624
  %956 = getelementptr inbounds nuw i8, ptr %self, i32 640
  %row19.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 656
  %957 = getelementptr inbounds nuw i8, ptr %self, i32 672
  %958 = getelementptr inbounds nuw i8, ptr %self, i32 688
  %959 = getelementptr inbounds nuw i8, ptr %self, i32 704
  %row21.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i32 720
  %960 = getelementptr inbounds nuw i8, ptr %self, i32 736
  %961 = getelementptr inbounds nuw i8, ptr %self, i32 752
  %962 = getelementptr inbounds nuw i8, ptr %self, i32 768
  %963 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 20
  %_50.i.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 24
  %_50.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 28
  %_78.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 192
  %_79.i = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 256
  %964 = bitcast <4 x i32> %link.sroa.0.0.i to <16 x i8>
  %965 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 32
  %966 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 36
  %_22.i.i = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 16
  %967 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 40
  %968 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 44
  %969 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 336
  %970 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 352
  %971 = getelementptr inbounds nuw i8, ptr %hot_left.i, i32 320
  %972 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 52
  %973 = getelementptr inbounds nuw i8, ptr %uniform_left.i, i32 48
  %974 = bitcast <4 x i32> %spec.store.select17.i to <16 x i8>
  %history.i.i.sroa.7.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.10.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.13.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.16.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.19.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.22.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.26.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx.promoted = load <4 x i32>, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !noalias !17486
  %_22.i.i.promoted = load i32, ptr %_22.i.i, align 4
  %.promoted9883 = load <4 x float>, ptr %969, align 16
  %_50.i.sroa.3.0.copyload = load i32, ptr %_50.i.sroa.3.0..sroa_idx, align 4
  %_50.i.sroa.4.0.copyload = load i32, ptr %_50.i.sroa.4.0..sroa_idx, align 4
  %_8.i.i8245 = load <4 x float>, ptr %_78.i, align 16
  %_9.i.i8246 = load <4 x float>, ptr %_79.i, align 16
  %_54.0.i.i = load ptr, ptr %965, align 16, !nonnull !10, !align !10189
  %_54.1.i.i = load i32, ptr %966, align 4
  %_18.i18.i = load i32, ptr %963, align 4
  %_29.i.i9623.not = icmp eq i32 %_18.i18.i, 0
  %_56.0.i.i = load ptr, ptr %967, align 8, !nonnull !10, !align !10189
  %_56.1.i.i = load i32, ptr %968, align 4
  %_37.i.i8254 = load <4 x float>, ptr %970, align 16
  %_58.1.i.i = load i32, ptr %972, align 4
  %_58.0.i.i = load ptr, ptr %973, align 16, !nonnull !10, !align !10189
  br label %bb32.i, !dbg !17466

bb13.i5.loopexit:                                 ; preds = %bb48.i, %bb32.i
  %history.i.i.sroa.0.0.lcssa11292 = phi <4 x i32> [ %lanes.i3101.sroa.0.0.copyload, %bb48.i ], [ %history.i.i.sroa.0.0.lcssa97049718, %bb32.i ]
  %history.i.i.sroa.7.0.lcssa11291 = phi <4 x i32> [ %history.i.i.sroa.0.09609, %bb48.i ], [ %history.i.i.sroa.7.0.lcssa9726, %bb32.i ]
  %history.i.i.sroa.10.0.lcssa11290 = phi <4 x i32> [ %history.i.i.sroa.7.09608, %bb48.i ], [ %history.i.i.sroa.10.0.lcssa9739, %bb32.i ]
  %history.i.i.sroa.13.0.lcssa11289 = phi <4 x i32> [ %history.i.i.sroa.10.09607, %bb48.i ], [ %history.i.i.sroa.13.0.lcssa9752, %bb32.i ]
  %history.i.i.sroa.16.0.lcssa11288 = phi <4 x i32> [ %history.i.i.sroa.13.09606, %bb48.i ], [ %history.i.i.sroa.16.0.lcssa9765, %bb32.i ]
  %history.i.i.sroa.19.0.lcssa11287 = phi <4 x i32> [ %history.i.i.sroa.16.09605, %bb48.i ], [ %history.i.i.sroa.19.0.lcssa9778, %bb32.i ]
  %history.i.i.sroa.22.0.lcssa11286 = phi <4 x i32> [ %history.i.i.sroa.19.09604, %bb48.i ], [ %history.i.i.sroa.22.0.lcssa9791, %bb32.i ]
  %history.i.i.sroa.26.0.lcssa11285 = phi <4 x i32> [ %history.i.i.sroa.22.09603, %bb48.i ], [ %history.i.i.sroa.26.0.lcssa9804, %bb32.i ]
  %history.i.i.sroa.29.0.lcssa11284 = phi <4 x i32> [ %history.i.i.sroa.26.09602, %bb48.i ], [ %history.i.i.sroa.29.0.lcssa9817, %bb32.i ]
  %history.i.i.sroa.32.0.lcssa11283 = phi <4 x i32> [ %history.i.i.sroa.29.09601, %bb48.i ], [ %history.i.i.sroa.32.0.lcssa9830, %bb32.i ]
  %history.i.i.sroa.35.0.lcssa11282 = phi <4 x i32> [ %history.i.i.sroa.32.09600, %bb48.i ], [ %history.i.i.sroa.35.0.lcssa9843, %bb32.i ]
  %history.i.i.sroa.38.0.lcssa11281 = phi <4 x i32> [ %history.i.i.sroa.35.09599, %bb48.i ], [ %history.i.i.sroa.38.0.lcssa9856, %bb32.i ]
  %.lcssa96559685.lcssa9884 = phi <4 x float> [ %.lcssa96559685, %bb48.i ], [ %.lcssa96559685.lcssa9885, %bb32.i ]
  %storemerge.i.i.lcssa96399665.lcssa9869 = phi i32 [ %storemerge.i.i.lcssa96399665, %bb48.i ], [ %storemerge.i.i.lcssa96399665.lcssa9870, %bb32.i ]
  %ring_cursor.sroa.0.1.i.lcssa = phi i32 [ %ring_cursor.sroa.0.2.i, %bb48.i ], [ %ring_cursor.sroa.0.0.i9719, %bb32.i ], !dbg !17491
  %main_cursor.sroa.0.1.i.lcssa = phi i32 [ %main_cursor.sroa.0.2.i, %bb48.i ], [ %main_cursor.sroa.0.0.i9720, %bb32.i ], !dbg !17492
  %_106.not.i = icmp eq i32 %976, 0, !dbg !17466
  %indvars.iv.next11171 = add i32 %indvars.iv11170, -32, !dbg !17466
  br i1 %_106.not.i, label %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge, label %bb32.i, !dbg !17466

bb32.i:                                           ; preds = %bb32.i.lr.ph, %bb13.i5.loopexit
  %indvars.iv11170 = phi i32 [ %frames, %bb32.i.lr.ph ], [ %indvars.iv.next11171, %bb13.i5.loopexit ]
  %.lcssa96559685.lcssa9885 = phi <4 x float> [ %.promoted9883, %bb32.i.lr.ph ], [ %.lcssa96559685.lcssa9884, %bb13.i5.loopexit ]
  %storemerge.i.i.lcssa96399665.lcssa9870 = phi i32 [ %_22.i.i.promoted, %bb32.i.lr.ph ], [ %storemerge.i.i.lcssa96399665.lcssa9869, %bb13.i5.loopexit ]
  %history.i.i.sroa.38.0.lcssa9856 = phi <4 x i32> [ %history.i.i.sroa.38.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.38.0.lcssa11281, %bb13.i5.loopexit ]
  %history.i.i.sroa.35.0.lcssa9843 = phi <4 x i32> [ %history.i.i.sroa.35.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.35.0.lcssa11282, %bb13.i5.loopexit ]
  %history.i.i.sroa.32.0.lcssa9830 = phi <4 x i32> [ %history.i.i.sroa.32.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.32.0.lcssa11283, %bb13.i5.loopexit ]
  %history.i.i.sroa.29.0.lcssa9817 = phi <4 x i32> [ %history.i.i.sroa.29.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.29.0.lcssa11284, %bb13.i5.loopexit ]
  %history.i.i.sroa.26.0.lcssa9804 = phi <4 x i32> [ %history.i.i.sroa.26.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.26.0.lcssa11285, %bb13.i5.loopexit ]
  %history.i.i.sroa.22.0.lcssa9791 = phi <4 x i32> [ %history.i.i.sroa.22.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.22.0.lcssa11286, %bb13.i5.loopexit ]
  %history.i.i.sroa.19.0.lcssa9778 = phi <4 x i32> [ %history.i.i.sroa.19.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.19.0.lcssa11287, %bb13.i5.loopexit ]
  %history.i.i.sroa.16.0.lcssa9765 = phi <4 x i32> [ %history.i.i.sroa.16.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.16.0.lcssa11288, %bb13.i5.loopexit ]
  %history.i.i.sroa.13.0.lcssa9752 = phi <4 x i32> [ %history.i.i.sroa.13.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.13.0.lcssa11289, %bb13.i5.loopexit ]
  %history.i.i.sroa.10.0.lcssa9739 = phi <4 x i32> [ %history.i.i.sroa.10.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.10.0.lcssa11290, %bb13.i5.loopexit ]
  %history.i.i.sroa.7.0.lcssa9726 = phi <4 x i32> [ %history.i.i.sroa.7.0.hot_left.i.sroa_idx.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.7.0.lcssa11291, %bb13.i5.loopexit ]
  %iter2.sroa.0.0.i9722 = phi i32 [ %yield_count.sroa.0.0.i3762, %bb32.i.lr.ph ], [ %976, %bb13.i5.loopexit ]
  %iter1.sroa.0.0.i9721 = phi i32 [ 0, %bb32.i.lr.ph ], [ %975, %bb13.i5.loopexit ]
  %main_cursor.sroa.0.0.i9720 = phi i32 [ %_26.i, %bb32.i.lr.ph ], [ %main_cursor.sroa.0.1.i.lcssa, %bb13.i5.loopexit ]
  %ring_cursor.sroa.0.0.i9719 = phi i32 [ %_27.i, %bb32.i.lr.ph ], [ %ring_cursor.sroa.0.1.i.lcssa, %bb13.i5.loopexit ]
  %history.i.i.sroa.0.0.lcssa97049718 = phi <4 x i32> [ %hot_left.i.promoted, %bb32.i.lr.ph ], [ %history.i.i.sroa.0.0.lcssa11292, %bb13.i5.loopexit ]
  %umin11184 = call i32 @llvm.umin.i32(i32 %indvars.iv11170, i32 32), !dbg !17493
  %umax11173 = call i32 @llvm.umax.i32(i32 %umin11184, i32 1), !dbg !17493
  %975 = add i32 %iter1.sroa.0.0.i9721, 32, !dbg !17493
  %976 = add nsw i32 %iter2.sroa.0.0.i9722, -1, !dbg !17497
  %977 = sub i32 %frames, %iter1.sroa.0.0.i9721, !dbg !17498
  %spec.store.select.i = tail call i32 @llvm.umin.i32(i32 %977, i32 32), !dbg !17500
  %_20.i.i9598.not = icmp eq i32 %frames, %iter1.sroa.0.0.i9721, !dbg !17505
  br i1 %_20.i.i9598.not, label %bb13.i5.loopexit, label %bb5.i.i.lr.ph, !dbg !17511

bb5.i.i.lr.ph:                                    ; preds = %bb32.i
  %_11.i.i.i.i8263 = load <4 x float>, ptr %_22, align 16
  %_14.i.i.i.i8264 = load <4 x float>, ptr %927, align 16
  %_17.i.i.i.i8265 = load <4 x float>, ptr %928, align 16
  %_20.i.i.i.i8266 = load <4 x float>, ptr %929, align 16
  %_25.i.i.i.i8267 = load <4 x float>, ptr %row1.i.i.i.i, align 16
  %_28.i.i.i.i8268 = load <4 x float>, ptr %930, align 16
  %_31.i.i.i.i8269 = load <4 x float>, ptr %931, align 16
  %_34.i.i.i.i8270 = load <4 x float>, ptr %932, align 16
  %_39.i.i.i.i8271 = load <4 x float>, ptr %row3.i.i.i.i, align 16
  %_42.i.i.i.i8272 = load <4 x float>, ptr %933, align 16
  %_45.i.i.i.i8273 = load <4 x float>, ptr %934, align 16
  %_48.i.i.i.i8274 = load <4 x float>, ptr %935, align 16
  %_53.i.i.i.i8275 = load <4 x float>, ptr %row5.i.i.i.i, align 16
  %_56.i.i.i.i8276 = load <4 x float>, ptr %936, align 16
  %_59.i.i.i.i8277 = load <4 x float>, ptr %937, align 16
  %_62.i.i.i.i8278 = load <4 x float>, ptr %938, align 16
  %_67.i.i.i.i8279 = load <4 x float>, ptr %row7.i.i.i.i, align 16
  %_70.i.i.i.i8280 = load <4 x float>, ptr %939, align 16
  %_73.i.i.i.i8281 = load <4 x float>, ptr %940, align 16
  %_76.i.i.i.i8282 = load <4 x float>, ptr %941, align 16
  %_81.i.i.i.i8283 = load <4 x float>, ptr %row9.i.i.i.i, align 16
  %_84.i.i.i.i8284 = load <4 x float>, ptr %942, align 16
  %_87.i.i.i.i8285 = load <4 x float>, ptr %943, align 16
  %_90.i.i.i.i8286 = load <4 x float>, ptr %944, align 16
  %_95.i.i.i.i8287 = load <4 x float>, ptr %row11.i.i.i.i, align 16
  %_98.i.i.i.i8288 = load <4 x float>, ptr %945, align 16
  %_101.i.i.i.i8289 = load <4 x float>, ptr %946, align 16
  %_104.i.i.i.i8290 = load <4 x float>, ptr %947, align 16
  %_109.i.i.i.i8291 = load <4 x float>, ptr %row13.i.i.i.i, align 16
  %_112.i.i.i.i8292 = load <4 x float>, ptr %948, align 16
  %_115.i.i.i.i8293 = load <4 x float>, ptr %949, align 16
  %_118.i.i.i.i8294 = load <4 x float>, ptr %950, align 16
  %_123.i.i.i.i8295 = load <4 x float>, ptr %row15.i.i.i.i, align 16
  %_126.i.i.i.i8296 = load <4 x float>, ptr %951, align 16
  %_129.i.i.i.i8297 = load <4 x float>, ptr %952, align 16
  %_132.i.i.i.i8298 = load <4 x float>, ptr %953, align 16
  %_137.i.i.i.i8299 = load <4 x float>, ptr %row17.i.i.i.i, align 16
  %_140.i.i.i.i8300 = load <4 x float>, ptr %954, align 16
  %_143.i.i.i.i8301 = load <4 x float>, ptr %955, align 16
  %_146.i.i.i.i8302 = load <4 x float>, ptr %956, align 16
  %_151.i.i.i.i8303 = load <4 x float>, ptr %row19.i.i.i.i, align 16
  %_154.i.i.i.i8304 = load <4 x float>, ptr %957, align 16
  %_157.i.i.i.i8305 = load <4 x float>, ptr %958, align 16
  %_160.i.i.i.i8306 = load <4 x float>, ptr %959, align 16
  %_165.i.i.i.i8307 = load <4 x float>, ptr %row21.i.i.i.i, align 16
  %_168.i.i.i.i8308 = load <4 x float>, ptr %960, align 16
  %_171.i.i.i.i8309 = load <4 x float>, ptr %961, align 16
  %_174.i.i.i.i8310 = load <4 x float>, ptr %962, align 16
  br label %bb5.i.i, !dbg !17511

bb5.i.i:                                          ; preds = %bb5.i.i.lr.ph, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468
  %iter.sroa.0.0.i.i9610 = phi i32 [ 0, %bb5.i.i.lr.ph ], [ %978, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.0.09609 = phi <4 x i32> [ %history.i.i.sroa.0.0.lcssa97049718, %bb5.i.i.lr.ph ], [ %lanes.i3101.sroa.0.0.copyload, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.7.09608 = phi <4 x i32> [ %history.i.i.sroa.7.0.lcssa9726, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.0.09609, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.10.09607 = phi <4 x i32> [ %history.i.i.sroa.10.0.lcssa9739, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.7.09608, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.13.09606 = phi <4 x i32> [ %history.i.i.sroa.13.0.lcssa9752, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.10.09607, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.16.09605 = phi <4 x i32> [ %history.i.i.sroa.16.0.lcssa9765, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.13.09606, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.19.09604 = phi <4 x i32> [ %history.i.i.sroa.19.0.lcssa9778, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.16.09605, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.22.09603 = phi <4 x i32> [ %history.i.i.sroa.22.0.lcssa9791, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.19.09604, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.26.09602 = phi <4 x i32> [ %history.i.i.sroa.26.0.lcssa9804, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.22.09603, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.29.09601 = phi <4 x i32> [ %history.i.i.sroa.29.0.lcssa9817, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.26.09602, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.32.09600 = phi <4 x i32> [ %history.i.i.sroa.32.0.lcssa9830, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.29.09601, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %history.i.i.sroa.35.09599 = phi <4 x i32> [ %history.i.i.sroa.35.0.lcssa9843, %bb5.i.i.lr.ph ], [ %history.i.i.sroa.32.09600, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468 ]
  %978 = add nuw nsw i32 %iter.sroa.0.0.i.i9610, 1, !dbg !17512
  %_11.i.i = add nuw nsw i32 %iter.sroa.0.0.i.i9610, %iter1.sroa.0.0.i9721, !dbg !17515
  %base.i.i = shl i32 %_11.i.i, 2, !dbg !17515
  %_24.i.i = icmp ugt i32 %base.i.i, %left_io.1, !dbg !17516
  br i1 %_24.i.i, label %bb7.i.i, label %bb8.i.i, !dbg !17516, !prof !902

bb8.i.i:                                          ; preds = %bb5.i.i
  %_27.i.i = sub nuw nsw i32 %left_io.1, %base.i.i, !dbg !17519
  %_8.i3104 = icmp samesign ugt i32 %_27.i.i, 3, !dbg !17520
  br i1 %_8.i3104, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468, label %bb2.i3105, !dbg !17520, !prof !1153

bb2.i3105:                                        ; preds = %bb8.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_27.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !17525, !noalias !17526
  unreachable, !dbg !17525

bb7.i.i:                                          ; preds = %bb5.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i.i, i32 noundef range(i32 0, 536870912) %left_io.1, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_d37239ff881951c49620cb5a9c000a8e) #33, !dbg !17531, !noalias !17532
  unreachable, !dbg !17531

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468: ; preds = %bb8.i.i
  %_31.i.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i.i, !dbg !17533
  %lanes.i3101.sroa.0.0.copyload = load <4 x i32>, ptr %_31.i.i, align 4, !dbg !17535, !alias.scope !17539, !noalias !17543
  %979 = bitcast <4 x i32> %history.i.i.sroa.19.09604 to <4 x float>, !dbg !17545
  %980 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %979), !dbg !17550
  %981 = bitcast <4 x i32> %lanes.i3101.sroa.0.0.copyload to <4 x float>, !dbg !17551
  %982 = fmul <4 x float> %_11.i.i.i.i8263, %981, !dbg !17556
  %983 = fadd <4 x float> %982, zeroinitializer, !dbg !17557
  %984 = fmul <4 x float> %_14.i.i.i.i8264, %981, !dbg !17561
  %985 = fadd <4 x float> %984, zeroinitializer, !dbg !17565
  %986 = fmul <4 x float> %_17.i.i.i.i8265, %981, !dbg !17569
  %987 = fadd <4 x float> %986, zeroinitializer, !dbg !17573
  %988 = fmul <4 x float> %_20.i.i.i.i8266, %981, !dbg !17577
  %989 = fadd <4 x float> %988, zeroinitializer, !dbg !17581
  %990 = bitcast <4 x i32> %history.i.i.sroa.0.09609 to <4 x float>, !dbg !17585
  %991 = fmul <4 x float> %_25.i.i.i.i8267, %990, !dbg !17589
  %992 = fadd <4 x float> %983, %991, !dbg !17590
  %993 = fmul <4 x float> %_28.i.i.i.i8268, %990, !dbg !17594
  %994 = fadd <4 x float> %985, %993, !dbg !17598
  %995 = fmul <4 x float> %_31.i.i.i.i8269, %990, !dbg !17602
  %996 = fadd <4 x float> %987, %995, !dbg !17606
  %997 = fmul <4 x float> %_34.i.i.i.i8270, %990, !dbg !17610
  %998 = fadd <4 x float> %989, %997, !dbg !17614
  %999 = bitcast <4 x i32> %history.i.i.sroa.7.09608 to <4 x float>, !dbg !17618
  %1000 = fmul <4 x float> %_39.i.i.i.i8271, %999, !dbg !17622
  %1001 = fadd <4 x float> %992, %1000, !dbg !17623
  %1002 = fmul <4 x float> %_42.i.i.i.i8272, %999, !dbg !17627
  %1003 = fadd <4 x float> %994, %1002, !dbg !17631
  %1004 = fmul <4 x float> %_45.i.i.i.i8273, %999, !dbg !17635
  %1005 = fadd <4 x float> %996, %1004, !dbg !17639
  %1006 = fmul <4 x float> %_48.i.i.i.i8274, %999, !dbg !17643
  %1007 = fadd <4 x float> %998, %1006, !dbg !17647
  %1008 = bitcast <4 x i32> %history.i.i.sroa.10.09607 to <4 x float>, !dbg !17651
  %1009 = fmul <4 x float> %_53.i.i.i.i8275, %1008, !dbg !17655
  %1010 = fadd <4 x float> %1001, %1009, !dbg !17656
  %1011 = fmul <4 x float> %_56.i.i.i.i8276, %1008, !dbg !17660
  %1012 = fadd <4 x float> %1003, %1011, !dbg !17664
  %1013 = fmul <4 x float> %_59.i.i.i.i8277, %1008, !dbg !17668
  %1014 = fadd <4 x float> %1005, %1013, !dbg !17672
  %1015 = fmul <4 x float> %_62.i.i.i.i8278, %1008, !dbg !17676
  %1016 = fadd <4 x float> %1007, %1015, !dbg !17680
  %1017 = bitcast <4 x i32> %history.i.i.sroa.13.09606 to <4 x float>, !dbg !17684
  %1018 = fmul <4 x float> %_67.i.i.i.i8279, %1017, !dbg !17688
  %1019 = fadd <4 x float> %1010, %1018, !dbg !17689
  %1020 = fmul <4 x float> %_70.i.i.i.i8280, %1017, !dbg !17693
  %1021 = fadd <4 x float> %1012, %1020, !dbg !17697
  %1022 = fmul <4 x float> %_73.i.i.i.i8281, %1017, !dbg !17701
  %1023 = fadd <4 x float> %1014, %1022, !dbg !17705
  %1024 = fmul <4 x float> %_76.i.i.i.i8282, %1017, !dbg !17709
  %1025 = fadd <4 x float> %1016, %1024, !dbg !17713
  %1026 = bitcast <4 x i32> %history.i.i.sroa.16.09605 to <4 x float>, !dbg !17717
  %1027 = fmul <4 x float> %_81.i.i.i.i8283, %1026, !dbg !17721
  %1028 = fadd <4 x float> %1019, %1027, !dbg !17722
  %1029 = fmul <4 x float> %_84.i.i.i.i8284, %1026, !dbg !17726
  %1030 = fadd <4 x float> %1021, %1029, !dbg !17730
  %1031 = fmul <4 x float> %_87.i.i.i.i8285, %1026, !dbg !17734
  %1032 = fadd <4 x float> %1023, %1031, !dbg !17738
  %1033 = fmul <4 x float> %_90.i.i.i.i8286, %1026, !dbg !17742
  %1034 = fadd <4 x float> %1025, %1033, !dbg !17746
  %1035 = fmul <4 x float> %_95.i.i.i.i8287, %979, !dbg !17750
  %1036 = fadd <4 x float> %1028, %1035, !dbg !17754
  %1037 = fmul <4 x float> %_98.i.i.i.i8288, %979, !dbg !17758
  %1038 = fadd <4 x float> %1030, %1037, !dbg !17762
  %1039 = fmul <4 x float> %_101.i.i.i.i8289, %979, !dbg !17766
  %1040 = fadd <4 x float> %1032, %1039, !dbg !17770
  %1041 = fmul <4 x float> %_104.i.i.i.i8290, %979, !dbg !17774
  %1042 = fadd <4 x float> %1034, %1041, !dbg !17778
  %1043 = bitcast <4 x i32> %history.i.i.sroa.22.09603 to <4 x float>, !dbg !17782
  %1044 = fmul <4 x float> %_109.i.i.i.i8291, %1043, !dbg !17786
  %1045 = fadd <4 x float> %1036, %1044, !dbg !17787
  %1046 = fmul <4 x float> %_112.i.i.i.i8292, %1043, !dbg !17791
  %1047 = fadd <4 x float> %1038, %1046, !dbg !17795
  %1048 = fmul <4 x float> %_115.i.i.i.i8293, %1043, !dbg !17799
  %1049 = fadd <4 x float> %1040, %1048, !dbg !17803
  %1050 = fmul <4 x float> %_118.i.i.i.i8294, %1043, !dbg !17807
  %1051 = fadd <4 x float> %1042, %1050, !dbg !17811
  %1052 = bitcast <4 x i32> %history.i.i.sroa.26.09602 to <4 x float>, !dbg !17815
  %1053 = fmul <4 x float> %_123.i.i.i.i8295, %1052, !dbg !17819
  %1054 = fadd <4 x float> %1045, %1053, !dbg !17820
  %1055 = fmul <4 x float> %_126.i.i.i.i8296, %1052, !dbg !17824
  %1056 = fadd <4 x float> %1047, %1055, !dbg !17828
  %1057 = fmul <4 x float> %_129.i.i.i.i8297, %1052, !dbg !17832
  %1058 = fadd <4 x float> %1049, %1057, !dbg !17836
  %1059 = fmul <4 x float> %_132.i.i.i.i8298, %1052, !dbg !17840
  %1060 = fadd <4 x float> %1051, %1059, !dbg !17844
  %1061 = bitcast <4 x i32> %history.i.i.sroa.29.09601 to <4 x float>, !dbg !17848
  %1062 = fmul <4 x float> %_137.i.i.i.i8299, %1061, !dbg !17852
  %1063 = fadd <4 x float> %1054, %1062, !dbg !17853
  %1064 = fmul <4 x float> %_140.i.i.i.i8300, %1061, !dbg !17857
  %1065 = fadd <4 x float> %1056, %1064, !dbg !17861
  %1066 = fmul <4 x float> %_143.i.i.i.i8301, %1061, !dbg !17865
  %1067 = fadd <4 x float> %1058, %1066, !dbg !17869
  %1068 = fmul <4 x float> %_146.i.i.i.i8302, %1061, !dbg !17873
  %1069 = fadd <4 x float> %1060, %1068, !dbg !17877
  %1070 = bitcast <4 x i32> %history.i.i.sroa.32.09600 to <4 x float>, !dbg !17881
  %1071 = fmul <4 x float> %_151.i.i.i.i8303, %1070, !dbg !17885
  %1072 = fadd <4 x float> %1063, %1071, !dbg !17886
  %1073 = fmul <4 x float> %_154.i.i.i.i8304, %1070, !dbg !17890
  %1074 = fadd <4 x float> %1065, %1073, !dbg !17894
  %1075 = fmul <4 x float> %_157.i.i.i.i8305, %1070, !dbg !17898
  %1076 = fadd <4 x float> %1067, %1075, !dbg !17902
  %1077 = fmul <4 x float> %_160.i.i.i.i8306, %1070, !dbg !17906
  %1078 = fadd <4 x float> %1069, %1077, !dbg !17910
  %1079 = bitcast <4 x i32> %history.i.i.sroa.35.09599 to <4 x float>, !dbg !17914
  %1080 = fmul <4 x float> %_165.i.i.i.i8307, %1079, !dbg !17918
  %1081 = fadd <4 x float> %1072, %1080, !dbg !17919
  %1082 = fmul <4 x float> %_168.i.i.i.i8308, %1079, !dbg !17923
  %1083 = fadd <4 x float> %1074, %1082, !dbg !17927
  %1084 = fmul <4 x float> %_171.i.i.i.i8309, %1079, !dbg !17931
  %1085 = fadd <4 x float> %1076, %1084, !dbg !17935
  %1086 = fmul <4 x float> %_174.i.i.i.i8310, %1079, !dbg !17939
  %1087 = fadd <4 x float> %1078, %1086, !dbg !17943
  %1088 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1081), !dbg !17947
  %1089 = fcmp olt <4 x float> %1088, %980, !dbg !17951
  %1090 = select <4 x i1> %1089, <4 x float> %980, <4 x float> %1088, !dbg !17955
  %1091 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1083), !dbg !17947
  %1092 = fcmp olt <4 x float> %1091, %1090, !dbg !17951
  %1093 = select <4 x i1> %1092, <4 x float> %1090, <4 x float> %1091, !dbg !17955
  %1094 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1085), !dbg !17947
  %1095 = fcmp olt <4 x float> %1094, %1093, !dbg !17951
  %1096 = select <4 x i1> %1095, <4 x float> %1093, <4 x float> %1094, !dbg !17955
  %1097 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1087), !dbg !17947
  %1098 = fcmp olt <4 x float> %1097, %1096, !dbg !17951
  %1099 = select <4 x i1> %1098, <4 x float> %1096, <4 x float> %1097, !dbg !17955
  %_39.i.i.idx = shl i32 %iter.sroa.0.0.i.i9610, 4, !dbg !17956
  %_39.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i, i32 %_39.i.i.idx, !dbg !17956
  store <4 x float> %1099, ptr %_39.i.i, align 4, !dbg !17961, !alias.scope !17966, !noalias !17970
  %exitcond11174.not = icmp eq i32 %978, %umax11173, !dbg !17505
  br i1 %exitcond11174.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb5.i.i, !dbg !17511

_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i: ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3468
  br label %bb17.i, !dbg !17974

bb17.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, %bb48.i
  %.lcssa96559686 = phi <4 x float> [ %.lcssa96559685, %bb48.i ], [ %.lcssa96559685.lcssa9885, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ]
  %storemerge.i.i.lcssa96399666 = phi i32 [ %storemerge.i.i.lcssa96399665, %bb48.i ], [ %storemerge.i.i.lcssa96399665.lcssa9870, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ]
  %frame.sroa.0.0.i9661 = phi i32 [ %_64.i, %bb48.i ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ]
  %main_cursor.sroa.0.1.i9660 = phi i32 [ %main_cursor.sroa.0.2.i, %bb48.i ], [ %main_cursor.sroa.0.0.i9720, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ]
  %ring_cursor.sroa.0.1.i9659 = phi i32 [ %ring_cursor.sroa.0.2.i, %bb48.i ], [ %ring_cursor.sroa.0.0.i9719, %_RINvCsjLJhryqjeDL_17true_peak_limiter14detector_chunkNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i ]
  %_47.i = sub nuw nsw i32 %spec.store.select.i, %frame.sroa.0.0.i9661, !dbg !17976
  %ring.i1333 = load i32, ptr %923, align 4, !dbg !17977, !alias.scope !17979, !noalias !17982, !noundef !10
  %main.i1334 = load i32, ptr %924, align 4, !dbg !17986, !alias.scope !17979, !noalias !17982, !noundef !10
  %_10.i1335 = add i32 %ring_cursor.sroa.0.1.i9659, 1, !dbg !17987
  %_38.not.i1336 = icmp ult i32 %_10.i1335, %ring.i1333, !dbg !17988
  %1100 = select i1 %_38.not.i1336, i32 0, i32 %ring.i1333, !dbg !17988
  %start1.sroa.0.0.i1337 = sub nuw i32 %_10.i1335, %1100, !dbg !17988
  %_12.i1339 = add i32 %_50.i.sroa.3.0.copyload, %ring_cursor.sroa.0.1.i9659, !dbg !17990
  %_39.not.i1340 = icmp ult i32 %_12.i1339, %ring.i1333, !dbg !17991
  %1101 = select i1 %_39.not.i1340, i32 0, i32 %ring.i1333, !dbg !17991
  %left_end.sroa.0.0.i1341 = sub nuw i32 %_12.i1339, %1101, !dbg !17991
  %_18.i1347 = add i32 %_50.i.sroa.4.0.copyload, %ring_cursor.sroa.0.1.i9659, !dbg !17993
  %_41.not.i1348 = icmp ult i32 %_18.i1347, %ring.i1333, !dbg !17994
  %1102 = select i1 %_41.not.i1348, i32 0, i32 %ring.i1333, !dbg !17994
  %left_expiring.sroa.0.0.i1349 = sub nuw i32 %_18.i1347, %1102, !dbg !17994
  %1103 = sub i32 %ring.i1333, %ring_cursor.sroa.0.1.i9659, !dbg !17996
  %spec.store.select.i1354 = tail call i32 @llvm.umin.i32(i32 %1103, i32 %_47.i), !dbg !17997
  %1104 = sub i32 %main.i1334, %main_cursor.sroa.0.1.i9660, !dbg !17999
  %_24.sroa.0.0.i1356 = tail call i32 @llvm.umin.i32(i32 %1104, i32 %spec.store.select.i1354), !dbg !18000
  %1105 = sub i32 %ring.i1333, %start1.sroa.0.0.i1337, !dbg !18002
  %_25.sroa.0.0.i1358 = tail call i32 @llvm.umin.i32(i32 %1105, i32 %_24.sroa.0.0.i1356), !dbg !18003
  %1106 = sub i32 %ring.i1333, %left_end.sroa.0.0.i1341, !dbg !18005
  %_27.sroa.0.0.i1360 = tail call i32 @llvm.umin.i32(i32 %1106, i32 %_25.sroa.0.0.i1358), !dbg !18006
  %1107 = sub i32 %ring.i1333, %left_expiring.sroa.0.0.i1349, !dbg !18008
  %_31.sroa.0.0.i1364 = tail call i32 @llvm.umin.i32(i32 %1107, i32 %_27.sroa.0.0.i1360), !dbg !18009
  %_54.i = add i32 %frame.sroa.0.0.i9661, %iter1.sroa.0.0.i9721, !dbg !18011
  %base.i = shl i32 %_54.i, 2, !dbg !18011
  %base.i8244 = add i32 %_31.sroa.0.0.i1364, %_54.i, !dbg !18014
  %_58.i = shl i32 %base.i8244, 2, !dbg !18014
  %_116.i = icmp ult i32 %_58.i, %base.i, !dbg !18017
  %_110.not.i = icmp ugt i32 %_58.i, %left_io.1
  %or.cond.i = or i1 %_116.i, %_110.not.i, !dbg !18017
  br i1 %or.cond.i, label %bb39.i, label %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3791, !dbg !18017, !prof !4694

bb39.i:                                           ; preds = %bb17.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i, i32 noundef %_58.i, i32 noundef range(i32 0, 536870912) %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_192e368852d87f0f082761c8ef71c730) #33, !dbg !18024, !noalias !18025
  unreachable, !dbg !18024

_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3791: ; preds = %bb17.i
  %_119.i = getelementptr inbounds nuw float, ptr %left_io.0, i32 %base.i, !dbg !18026
  %_64.i = add nuw nsw i32 %_31.sroa.0.0.i1364, %frame.sroa.0.0.i9661, !dbg !18030
  %_128.i.idx = shl nuw nsw i32 %frame.sroa.0.0.i9661, 4, !dbg !18032
  %_128.i = getelementptr inbounds nuw i8, ptr %peaks_left.i, i32 %_128.i.idx, !dbg !18032
  %_2.i37949626.not = icmp eq i32 %_31.sroa.0.0.i1364, 0, !dbg !18042
  br i1 %_2.i37949626.not, label %bb48.i, label %bb49.i.preheader, !dbg !18042

bb49.i.preheader:                                 ; preds = %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3791
  %umin11180 = call i32 @llvm.umin.i32(i32 %1106, i32 %1107)
  %umin11181 = call i32 @llvm.umin.i32(i32 %umin11180, i32 %1105)
  %umin11182 = call i32 @llvm.umin.i32(i32 %umin11181, i32 %1103)
  %umin11183 = call i32 @llvm.umin.i32(i32 %umin11182, i32 %1104)
  %1108 = sub nsw i32 %umin11184, %frame.sroa.0.0.i9661
  %umin11185 = call i32 @llvm.umin.i32(i32 %umin11183, i32 %1108)
  %1109 = and i32 %umin11185, 1073741823
  br label %bb49.i

bb49.i:                                           ; preds = %bb49.i.preheader, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151
  %1110 = phi <4 x float> [ %.lcssa96559686, %bb49.i.preheader ], [ %1143, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ]
  %storemerge.i.i9629 = phi i32 [ %storemerge.i.i.lcssa96399666, %bb49.i.preheader ], [ %storemerge.i.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ]
  %iter.i.sroa.16.09628 = phi i32 [ 0, %bb49.i.preheader ], [ %1111, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ]
  %start1.i.i3799 = shl i32 %iter.i.sroa.16.09628, 2, !dbg !18051
  %data.i4.i3805 = getelementptr inbounds nuw float, ptr %_128.i, i32 %start1.i.i3799, !dbg !18053
  %lanes.i3119.sroa.0.0.copyload = load <16 x i8>, ptr %data.i4.i3805, align 4, !dbg !18056, !alias.scope !18063, !noalias !18067
  %data.i.i3800 = getelementptr inbounds nuw float, ptr %_119.i, i32 %start1.i.i3799, !dbg !18071
  %lanes.i3110.sroa.0.0.copyload = load <4 x i32>, ptr %data.i.i3800, align 4, !dbg !18073, !alias.scope !18081, !noalias !18085
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18089), !dbg !18092
  %1111 = add nuw nsw i32 %iter.i.sroa.16.09628, 1, !dbg !18094
  %_140.i = add i32 %iter.i.sroa.16.09628, %ring_cursor.sroa.0.1.i9659, !dbg !18095
  %_141.i = add i32 %iter.i.sroa.16.09628, %main_cursor.sroa.0.1.i9660, !dbg !18098
  %_142.i = add i32 %iter.i.sroa.16.09628, %left_end.sroa.0.0.i1341, !dbg !18099
  %_143.i = add i32 %iter.i.sroa.16.09628, %start1.sroa.0.0.i1337, !dbg !18100
  %_144.i = add i32 %iter.i.sroa.16.09628, %left_expiring.sroa.0.0.i1349, !dbg !18101
  %base.i9.i.i = shl i32 %_140.i, 2, !dbg !18102
  %_7.i10.i.i = add i32 %base.i9.i.i, 4, !dbg !18105
  %1112 = or disjoint i32 %base.i9.i.i, 3, !dbg !18106
  %or.cond.i13.i.i.not = icmp ult i32 %1112, %_54.1.i.i, !dbg !18106
  br i1 %or.cond.i13.i.i.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i, label %bb4.i15.i.i, !dbg !18106, !prof !10587

bb4.i15.i.i:                                      ; preds = %bb49.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i.i, i32 noundef %_7.i10.i.i, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !18110, !noalias !18111
  unreachable, !dbg !18110

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i: ; preds = %bb49.i
  %_4.i3810 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %lanes.i3119.sroa.0.0.copyload, <16 x i8> %lanes.i3119.sroa.0.0.copyload, <16 x i8> %964), !dbg !18123
  %1113 = bitcast <16 x i8> %_4.i3810 to <4 x float>, !dbg !18127
  %1114 = fdiv <4 x float> %_8.i.i8245, %1113, !dbg !18132
  %1115 = bitcast <4 x float> %1114 to <16 x i8>, !dbg !18136
  %1116 = fcmp olt <4 x float> %_8.i.i8245, %1113, !dbg !18140
  %1117 = sext <4 x i1> %1116 to <4 x i32>, !dbg !18140
  %1118 = bitcast <4 x i32> %1117 to <16 x i8>, !dbg !18141
  %_4.i3811 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1115, <16 x i8> <i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63, i8 0, i8 0, i8 -128, i8 63>, <16 x i8> %1118), !dbg !18142
  %_17.i14.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i9.i.i, !dbg !18143
  store <16 x i8> %_4.i3811, ptr %_17.i14.i.i, align 4, !dbg !18145, !alias.scope !18150, !noalias !18154
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18158), !dbg !18161
  %base.i1170 = shl i32 %_142.i, 2, !dbg !18162
  %1119 = or disjoint i32 %base.i1170, 3, !dbg !18165
  %or.cond.i1174.not = icmp ult i32 %1119, %_54.1.i.i, !dbg !18165
  br i1 %or.cond.i1174.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1178, label %bb4.i1177, !dbg !18165, !prof !10587

bb4.i1177:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i
  %_5.i1171 = add i32 %base.i1170, 4, !dbg !18169
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1170, i32 noundef %_5.i1171, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !18170, !noalias !18171
  unreachable, !dbg !18170

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1178: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit16.i.i
  %_15.i1176 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1170, !dbg !18177
  %lanes.i2890.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1176, align 4, !dbg !18179
  %1120 = icmp eq i32 %storemerge.i.i9629, 0, !dbg !18184
  %_12.i.i8248 = load <4 x float>, ptr %uniform_left.i, align 16, !dbg !18184
  %1121 = bitcast <4 x i32> %lanes.i2890.sroa.0.0.copyload to <4 x float>, !dbg !18184
  %1122 = fcmp olt <4 x float> %_12.i.i8248, %1121, !dbg !18184
  %1123 = select <4 x i1> %1122, <4 x float> %_12.i.i8248, <4 x float> %1121, !dbg !18184
  %1124 = bitcast <4 x float> %1123 to <4 x i32>, !dbg !18184
  %.sroa.04286.0 = select i1 %1120, <4 x i32> %lanes.i2890.sroa.0.0.copyload, <4 x i32> %1124, !dbg !18184
  store <4 x i32> %.sroa.04286.0, ptr %uniform_left.i, align 16, !dbg !18185, !alias.scope !18158, !noalias !18186
  %_15.i30.i = add i32 %storemerge.i.i9629, 1, !dbg !18188
  %complete.i.i = icmp eq i32 %_15.i30.i, %_18.i18.i, !dbg !18188
  br i1 %complete.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142, label %bb7.i31.i, !dbg !18189

bb7.i31.i:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1178
  %base.i1161 = shl i32 %_143.i, 2, !dbg !18190
  %1125 = or disjoint i32 %base.i1161, 3, !dbg !18192
  %or.cond.i1165.not = icmp ult i32 %1125, %_54.1.i.i, !dbg !18192
  br i1 %or.cond.i1165.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169, label %bb4.i1168, !dbg !18192, !prof !10587

bb4.i1168:                                        ; preds = %bb7.i31.i
  %_5.i1162 = add i32 %base.i1161, 4, !dbg !18196
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1161, i32 noundef %_5.i1162, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !18197, !noalias !18198
  unreachable, !dbg !18197

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169: ; preds = %bb7.i31.i
  %_15.i1167 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1161, !dbg !18202
  %lanes.i2897.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1167, align 4, !dbg !18204, !alias.scope !18209, !noalias !18213
  %1126 = bitcast <4 x i32> %.sroa.04286.0 to <4 x float>, !dbg !18217
  %1127 = fcmp olt <4 x float> %lanes.i2897.sroa.0.0.copyload, %1126, !dbg !18221
  %1128 = select <4 x i1> %1127, <4 x float> %lanes.i2897.sroa.0.0.copyload, <4 x float> %1126, !dbg !18222
  %1129 = bitcast <4 x float> %1128 to <4 x i32>, !dbg !18223
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, !dbg !18225

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1178
  %1130 = bitcast <4 x i32> %lanes.i2890.sroa.0.0.copyload to <4 x float>, !dbg !18189
  br i1 %_29.i.i9623.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb19.i.i, !dbg !18226

bb19.i.i:                                         ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133
  %end.sroa.0.0.i.i9625 = phi i32 [ %1136, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133 ], [ %_142.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142 ]
  %iter.sroa.0.0.i34.i9624 = phi i32 [ %_30.i35.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142 ]
  %1131 = phi <4 x float> [ %1134, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133 ], [ %1130, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142 ]
  %base.i1125 = shl i32 %end.sroa.0.0.i.i9625, 2, !dbg !18229
  %1132 = or disjoint i32 %base.i1125, 3, !dbg !18231
  %or.cond.i1129.not = icmp ult i32 %1132, %_54.1.i.i, !dbg !18231
  br i1 %or.cond.i1129.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133, label %bb4.i1132, !dbg !18231, !prof !10587

bb4.i1132:                                        ; preds = %bb19.i.i
  %_5.i1126 = add i32 %base.i1125, 4, !dbg !18235
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1125, i32 noundef %_5.i1126, i32 noundef range(i32 0, 536870912) %_54.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !18236, !noalias !18237
  unreachable, !dbg !18236

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133: ; preds = %bb19.i.i
  %_30.i35.i = add nuw i32 %iter.sroa.0.0.i34.i9624, 1, !dbg !18241
  %_15.i1131 = getelementptr inbounds nuw float, ptr %_54.0.i.i, i32 %base.i1125, !dbg !18244
  %lanes.i2925.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1131, align 4, !dbg !18246, !alias.scope !18251, !noalias !18255
  %1133 = fcmp olt <4 x float> %1131, %lanes.i2925.sroa.0.0.copyload, !dbg !18259
  %1134 = select <4 x i1> %1133, <4 x float> %1131, <4 x float> %lanes.i2925.sroa.0.0.copyload, !dbg !18263
  store <4 x float> %1134, ptr %_15.i1131, align 4, !dbg !18264, !alias.scope !18270, !noalias !18274
  %1135 = icmp eq i32 %end.sroa.0.0.i.i9625, 0, !dbg !18280
  %spec.store.select.i.i = select i1 %1135, i32 %ring.i, i32 %end.sroa.0.0.i.i9625, !dbg !18280
  %1136 = add i32 %spec.store.select.i.i, -1, !dbg !18281
  %exitcond11176.not = icmp eq i32 %_30.i35.i, %_18.i18.i, !dbg !18282
  br i1 %exitcond11176.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i, label %bb19.i.i, !dbg !18226

_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169
  %.sroa.04286.1 = phi <4 x i32> [ %1129, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169 ], [ %.sroa.04286.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142 ], [ %.sroa.04286.0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133 ], !dbg !18284
  %storemerge.i.i = phi i32 [ %_15.i30.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1169 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1142 ], [ 0, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1133 ], !dbg !18285
  %1137 = bitcast <4 x i32> %.sroa.04286.1 to <4 x float>, !dbg !18286
  %1138 = fmul <4 x float> %1137, splat (float 1.638400e+04), !dbg !18290
  %1139 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %1138), !dbg !18291
  %1140 = fmul <4 x float> %1139, splat (float 0x3F10000000000000), !dbg !18295
  %base.i1152 = shl i32 %_144.i, 2, !dbg !18299
  %1141 = or disjoint i32 %base.i1152, 3, !dbg !18301
  %or.cond.i1156.not = icmp ult i32 %1141, %_56.1.i.i, !dbg !18301
  br i1 %or.cond.i1156.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160, label %bb4.i1159, !dbg !18301, !prof !10587

bb4.i1159:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i
  %_5.i1153 = add i32 %base.i1152, 4, !dbg !18305
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1152, i32 noundef %_5.i1153, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !18306, !noalias !18307
  unreachable, !dbg !18306

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter23sliding_minimum_uniformNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit.i
  %_15.i1158 = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %base.i1152, !dbg !18311
  %lanes.i2904.sroa.0.0.copyload = load <4 x float>, ptr %_15.i1158, align 4, !dbg !18313, !alias.scope !18318, !noalias !18322
  %1142 = fadd <4 x float> %1140, %1110, !dbg !18326
  %1143 = fsub <4 x float> %1142, %lanes.i2904.sroa.0.0.copyload, !dbg !18330
  %_8.not.i4.i.i = icmp ugt i32 %_7.i10.i.i, %_56.1.i.i
  br i1 %_8.not.i4.i.i, label %bb4.i7.i.i, label %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i, !dbg !18334, !prof !4694

bb4.i7.i.i:                                       ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i9.i.i, i32 noundef %_7.i10.i.i, i32 noundef range(i32 0, 536870912) %_56.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_77f249f9c0b1ac1e2d67f390a6ea84d4) #33, !dbg !18339, !noalias !18340
  unreachable, !dbg !18339

_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1160
  %_17.i6.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i, i32 %base.i9.i.i, !dbg !18344
  store <4 x float> %1140, ptr %_17.i6.i.i, align 4, !dbg !18346, !alias.scope !18351, !noalias !18355
  %_41.i.i8255 = load <4 x float>, ptr %971, align 16, !dbg !18359, !alias.scope !18089, !noalias !18360
  %1144 = fdiv <4 x float> %1143, %_37.i.i8254, !dbg !18361
  %1145 = fsub <4 x float> splat (float 1.000000e+00), %1144, !dbg !18365
  %1146 = fsub <4 x float> %1145, %_41.i.i8255, !dbg !18369
  %1147 = fmul <4 x float> %_9.i.i8246, %1146, !dbg !18373
  %1148 = fadd <4 x float> %_41.i.i8255, %1147, !dbg !18377
  %1149 = fcmp olt <4 x float> %1148, %1145, !dbg !18380
  %1150 = select <4 x i1> %1149, <4 x float> %1145, <4 x float> %1148, !dbg !18384
  %1151 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %1150), !dbg !18385
  %1152 = fcmp uge <4 x float> %1151, splat (float 0x3BC79CA100000000), !dbg !18390
  %1153 = bitcast <4 x float> %1150 to <4 x i32>, !dbg !18395
  %1154 = select <4 x i1> %1152, <4 x i32> %1153, <4 x i32> zeroinitializer, !dbg !18395
  store <4 x i32> %1154, ptr %971, align 16, !dbg !18398, !alias.scope !18089, !noalias !18360
  %base.i1143 = shl i32 %_141.i, 2, !dbg !18399
  %1155 = or disjoint i32 %base.i1143, 3, !dbg !18401
  %or.cond.i1147.not = icmp ult i32 %1155, %_58.1.i.i, !dbg !18401
  br i1 %or.cond.i1147.not, label %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151, label %bb4.i1150, !dbg !18401, !prof !10587

bb4.i1150:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i
  %_5.i1144 = add i32 %base.i1143, 4, !dbg !18405
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %base.i1143, i32 noundef %_5.i1144, i32 noundef range(i32 0, 536870912) %_58.1.i.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ecb3dab3c849990309e84d651d3f65d6) #33, !dbg !18406, !noalias !18407
  unreachable, !dbg !18406

_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter15store_ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit8.i.i
  %1156 = bitcast <4 x i32> %1154 to <4 x float>, !dbg !18411
  %1157 = fsub <4 x float> splat (float 1.000000e+00), %1156, !dbg !18415
  %_15.i1149 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i32 %base.i1143, !dbg !18416
  %lanes.i2911.sroa.0.0.copyload = load <4 x i32>, ptr %_15.i1149, align 4, !dbg !18418, !alias.scope !18423, !noalias !18427
  store <4 x i32> %lanes.i3110.sroa.0.0.copyload, ptr %_15.i1149, align 4, !dbg !18431, !alias.scope !18437, !noalias !18441
  %1158 = bitcast <4 x i32> %lanes.i2911.sroa.0.0.copyload to <4 x float>, !dbg !18447
  %1159 = fmul <4 x float> %1157, %1158, !dbg !18451
  %1160 = bitcast <4 x i32> %lanes.i2911.sroa.0.0.copyload to <16 x i8>, !dbg !18452
  %1161 = bitcast <4 x float> %1159 to <16 x i8>, !dbg !18456
  %_4.i3812 = tail call <16 x i8> @llvm.wasm.bitselect.v16i8(<16 x i8> %1160, <16 x i8> %1161, <16 x i8> %974), !dbg !18457
  store <16 x i8> %_4.i3812, ptr %data.i.i3800, align 4, !dbg !18458, !alias.scope !18463, !noalias !18467
  %exitcond11186.not = icmp eq i32 %1111, %1109, !dbg !18042
  br i1 %exitcond11186.not, label %bb48.i, label %bb49.i, !dbg !18042

bb48.i:                                           ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3791
  %.lcssa96559685 = phi <4 x float> [ %.lcssa96559686, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3791 ], [ %1143, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ]
  %storemerge.i.i.lcssa96399665 = phi i32 [ %storemerge.i.i.lcssa96399666, %_RNvXs3_NtNtNtCsdkdt1aaAg1T_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter14ChunksExactMutfEINtBZ_11ChunksExactfEEINtB5_7ZipImplBW_B1z_E3newCsjLJhryqjeDL_17true_peak_limiter.exit3791 ], [ %storemerge.i.i, %_RINvCsjLJhryqjeDL_17true_peak_limiter9ring_laneNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit1151 ]
  %_92.i = add i32 %_31.sroa.0.0.i1364, %ring_cursor.sroa.0.1.i9659, !dbg !18471
  %_139.not.i = icmp ult i32 %_92.i, %ring.i, !dbg !18472
  %1162 = select i1 %_139.not.i, i32 0, i32 %ring.i, !dbg !18472
  %ring_cursor.sroa.0.2.i = sub nuw i32 %_92.i, %1162, !dbg !18472
  %_94.i = add i32 %_31.sroa.0.0.i1364, %main_cursor.sroa.0.1.i9660, !dbg !18475
  %_145.not.i = icmp ult i32 %_94.i, %main.i, !dbg !18476
  %1163 = select i1 %_145.not.i, i32 0, i32 %main.i, !dbg !18476
  %main_cursor.sroa.0.2.i = sub nuw i32 %_94.i, %1163, !dbg !18476
  %_41.i = icmp ult i32 %_64.i, %spec.store.select.i, !dbg !17974
  br i1 %_41.i, label %bb17.i, label %bb13.i5.loopexit, !dbg !17974

bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge: ; preds = %bb13.i5.loopexit
  store <4 x i32> %history.i.i.sroa.7.0.lcssa11291, ptr %history.i.i.sroa.7.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.10.0.lcssa11290, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.13.0.lcssa11289, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.16.0.lcssa11288, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.19.0.lcssa11287, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.22.0.lcssa11286, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.26.0.lcssa11285, ptr %history.i.i.sroa.26.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.29.0.lcssa11284, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.32.0.lcssa11283, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.35.0.lcssa11282, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x i32> %history.i.i.sroa.38.0.lcssa11281, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx, align 16, !dbg !18478
  store <4 x float> %.lcssa96559685.lcssa9884, ptr %969, align 16
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !17466

_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge
  %left_phase.i = phi i32 [ %storemerge.i.i.lcssa96399665.lcssa9869, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %left_phase.i.pre, %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], !dbg !17476
  %history.i.i.sroa.0.0.lcssa9704.lcssa = phi <4 x i32> [ %history.i.i.sroa.0.0.lcssa11292, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %hot_left.i.promoted, %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ]
  %ring_cursor.sroa.0.0.i.lcssa = phi i32 [ %ring_cursor.sroa.0.1.i.lcssa, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %_27.i, %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], !dbg !17459
  %main_cursor.sroa.0.0.i.lcssa = phi i32 [ %main_cursor.sroa.0.1.i.lcssa, %bb13.i5._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], [ %_26.i, %bb3.i._RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit_crit_edge ], !dbg !17456
  store <4 x i32> %history.i.i.sroa.0.0.lcssa9704.lcssa, ptr %hot_left.i, align 1, !dbg !18478
  %left_prefix.i = load <4 x i32>, ptr %uniform_left.i, align 16, !dbg !18479, !noalias !17440
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i), !dbg !18480, !noalias !17440
  %1164 = getelementptr inbounds nuw i8, ptr %self, i32 968, !dbg !18481
  %_152.1.i = load i32, ptr %1164, align 4, !dbg !18481, !alias.scope !17432, !noalias !17443, !noundef !10
  %_8.i3460 = icmp samesign ugt i32 %_152.1.i, 3, !dbg !18483
  br i1 %_8.i3460, label %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3463, label %bb2.i3461, !dbg !18483, !prof !1153

bb2.i3461:                                        ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef 4, i32 noundef range(i32 0, 536870912) %_152.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_5f4e05826d69b5e832dc96c63f325058) #33, !dbg !18488, !noalias !18489
  unreachable, !dbg !18488

_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3463: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
  %1165 = getelementptr inbounds nuw i8, ptr %self, i32 964, !dbg !18481
  %_152.0.i = load ptr, ptr %1165, align 4, !dbg !18481, !alias.scope !17432, !noalias !17443, !nonnull !10, !noundef !10
  store <4 x i32> %left_prefix.i, ptr %_152.0.i, align 4, !dbg !18493, !alias.scope !18497, !noalias !18501
  %_153.0.i = load ptr, ptr %50, align 4, !dbg !18503, !alias.scope !17432, !noalias !17443, !nonnull !10, !noundef !10
  %_153.1.i = load i32, ptr %51, align 4, !dbg !18503, !alias.scope !17432, !noalias !17443, !noundef !10
  %1166 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i), !dbg !18504
  br i1 %1166, label %bb2.i3822, label %bb6.i3813, !dbg !18504

bb6.i3813:                                        ; preds = %bb2.i3822, %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3463
  %end_or_len.idx.i3814 = shl nuw nsw i32 %_153.1.i, 2, !dbg !18508
  %end_or_len.i3815 = getelementptr inbounds nuw i8, ptr %_153.0.i, i32 %end_or_len.idx.i3814, !dbg !18508
  %_293.i3816 = icmp eq i32 %_153.1.i, 0, !dbg !18512
  br i1 %_293.i3816, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3828, label %bb10.i3817, !dbg !18515

bb2.i3822:                                        ; preds = %_RNvXNtCsg5nzdbTILLT_4lane5simd4NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4NtB4_4Lane5store.exit3463
  %bytes1.sroa.0.0.zext.i3823 = and i32 %left_phase.i, 255, !dbg !18516
  %bytes1.sroa.0.0.isplat.i3824 = mul nuw i32 %bytes1.sroa.0.0.zext.i3823, 16843009, !dbg !18516
  %_5.i3825 = icmp eq i32 %left_phase.i, %bytes1.sroa.0.0.isplat.i3824, !dbg !18517
  br i1 %_5.i3825, label %bb3.i3826, label %bb6.i3813, !dbg !18517

bb3.i3826:                                        ; preds = %bb2.i3822
  %bytes.sroa.0.0.extract.trunc.i3827 = trunc i32 %left_phase.i to i8, !dbg !18518
  %1167 = shl nuw nsw i32 %_153.1.i, 2, !dbg !18520
  tail call void @llvm.memset.p0.i32(ptr nonnull align 4 %_153.0.i, i8 %bytes.sroa.0.0.extract.trunc.i3827, i32 %1167, i1 false), !dbg !18520, !alias.scope !18521, !noalias !18025
  br label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3828, !dbg !18524

bb10.i3817:                                       ; preds = %bb6.i3813, %bb10.i3817
  %iter.sroa.0.04.i3818 = phi ptr [ %_38.i3819, %bb10.i3817 ], [ %_153.0.i, %bb6.i3813 ]
  %_38.i3819 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i3818, i32 4, !dbg !18525
  store i32 %left_phase.i, ptr %iter.sroa.0.04.i3818, align 4, !dbg !18527, !alias.scope !18521, !noalias !18025
  %_29.i3820 = icmp eq ptr %_38.i3819, %end_or_len.i3815, !dbg !18512
  br i1 %_29.i3820, label %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3828, label %bb10.i3817, !dbg !18515

_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3828: ; preds = %bb10.i3817, %bb6.i3813, %bb3.i3826
  call void @llvm.lifetime.start.p0(ptr nonnull %_101.i), !dbg !18528, !noalias !17440
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 16 dereferenceable(368) %_101.i, ptr noundef nonnull align 16 dereferenceable(368) %hot_left.i, i32 368, i1 false), !dbg !18528, !noalias !17440
; call <true_peak_limiter::HotChannel<wide::f32x4_::f32x4>>::store
  call fastcc void @_RNvMs5_CsjLJhryqjeDL_17true_peak_limiterINtB5_10HotChannelNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E5storeB5_(ptr noalias noundef readonly align 16 captures(none) dereferenceable(368) %_101.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_24) #32, !dbg !18529, !noalias !18025
  call void @llvm.lifetime.end.p0(ptr nonnull %_101.i), !dbg !18530, !noalias !17440
  store i32 %main_cursor.sroa.0.0.i.lcssa, ptr %_25, align 4, !dbg !18531, !alias.scope !17434, !noalias !17458
  store i32 %ring_cursor.sroa.0.0.i.lcssa, ptr %925, align 4, !dbg !18532, !alias.scope !17434, !noalias !17458
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i), !dbg !18533, !noalias !17440
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i), !dbg !18534, !noalias !17440
  br label %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, !dbg !17429

_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit: ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit, %_RINvCsjLJhryqjeDL_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %_RNvXs4_NtNtCsdkdt1aaAg1T_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit3828
  br i1 %quiet.sroa.0.0.off07995, label %bb18, label %bb24, !dbg !18535

bb13:                                             ; preds = %bb11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18536), !dbg !18539
  %1168 = getelementptr inbounds nuw i8, ptr %self, i32 980, !dbg !18540
  %_40.0.i = load ptr, ptr %1168, align 4, !dbg !18540, !alias.scope !18536, !nonnull !10, !noundef !10
  %1169 = getelementptr inbounds nuw i8, ptr %self, i32 984, !dbg !18540
  %_40.1.i = load i32, ptr %1169, align 4, !dbg !18540, !alias.scope !18536, !noundef !10
  %1170 = getelementptr inbounds nuw i8, ptr %self, i32 1012, !dbg !18542
  %_41.0.i = load ptr, ptr %1170, align 4, !dbg !18542, !alias.scope !18536, !nonnull !10, !noundef !10
  %1171 = getelementptr inbounds nuw i8, ptr %self, i32 1016, !dbg !18542
  %_41.1.i = load i32, ptr %1171, align 4, !dbg !18542, !alias.scope !18536, !noundef !10
  %spec.store.select.i.i3829 = tail call i32 @llvm.umin.i32(i32 %_41.1.i, i32 %_40.1.i), !dbg !18543
  %_2.i6.not.i = icmp eq i32 %spec.store.select.i.i3829, 0, !dbg !18549
  br i1 %_2.i6.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i3830, !dbg !18549

bb3.i3830:                                        ; preds = %bb13, %bb5.i
  %iter.sroa.8.07.i = phi i32 [ %1172, %bb5.i ], [ 0, %bb13 ]
  %_3.i1.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i, i32 %iter.sroa.8.07.i, !dbg !18552
  %_14.i3831 = load i32, ptr %_3.i1.i.i, align 4, !dbg !18555, !noalias !18536, !noundef !10
  %_20.i = icmp eq i32 %_14.i3831, 0, !dbg !18556
  br i1 %_20.i, label %panic.i3838, label %bb5.i, !dbg !18556

bb5.i:                                            ; preds = %bb3.i3830
  %_3.i.i.i3832 = getelementptr inbounds nuw i32, ptr %_40.0.i, i32 %iter.sroa.8.07.i, !dbg !18557
  %1172 = add nuw i32 %iter.sroa.8.07.i, 1, !dbg !18560
  %_18.i3833 = load i32, ptr %_3.i.i.i3832, align 4, !dbg !18561, !noalias !18536, !noundef !10
  %_19.i3834 = urem i32 %frames, %_14.i3831, !dbg !18556
  %_16.i3835 = add i32 %_19.i3834, %_18.i3833, !dbg !18562
  %_15.i3836 = urem i32 %_16.i3835, %_14.i3831, !dbg !18563
  store i32 %_15.i3836, ptr %_3.i.i.i3832, align 4, !dbg !18564, !noalias !18536
  %exitcond.not.i = icmp eq i32 %1172, %spec.store.select.i.i3829, !dbg !18549
  br i1 %exitcond.not.i, label %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit, label %bb3.i3830, !dbg !18549

panic.i3838:                                      ; preds = %bb3.i3830
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f5b0427df9b659e554a697ca46ce8b5a) #33, !dbg !18556, !noalias !18536
  unreachable, !dbg !18556

_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit: ; preds = %bb5.i, %bb13
  %1173 = getelementptr inbounds nuw i8, ptr %self, i32 916, !dbg !18565
  %_20.val = load i32, ptr %1173, align 4, !dbg !18565
  %1174 = getelementptr inbounds nuw i8, ptr %self, i32 920, !dbg !18565
  %_20.val3598 = load i32, ptr %1174, align 4, !dbg !18565, !noundef !10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !18566), !dbg !18565
  %_10.i3839 = icmp eq i32 %_20.val3598, 0, !dbg !18569
  br i1 %_10.i3839, label %panic.i3851, label %bb1.i3840, !dbg !18569

bb1.i3840:                                        ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
  %_19 = getelementptr inbounds nuw i8, ptr %self, i32 800, !dbg !18571
  %_7.i3841 = load i32, ptr %_19, align 4, !dbg !18572, !alias.scope !18566, !noundef !10
  %_8.i3842 = urem i32 %frames, %_20.val3598, !dbg !18569
  %_5.i3843 = add i32 %_8.i3842, %_7.i3841, !dbg !18573
  %_4.i3844 = urem i32 %_5.i3843, %_20.val3598, !dbg !18574
  store i32 %_4.i3844, ptr %_19, align 4, !dbg !18575, !alias.scope !18566
  %_17.i3845 = icmp eq i32 %_20.val, 0, !dbg !18576
  br i1 %_17.i3845, label %panic2.i, label %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, !dbg !18576

panic.i3851:                                      ; preds = %_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_c7e64e9e8489bc8e648659d0098d136c) #33, !dbg !18569, !noalias !18566
  unreachable, !dbg !18569

panic2.i:                                         ; preds = %bb1.i3840
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_34b23597fad9d85cb3bee6d64ebf77d5) #33, !dbg !18576, !noalias !18566
  unreachable, !dbg !18576

_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit: ; preds = %bb1.i3840
  %1175 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !18577
  %_14.i3847 = load i32, ptr %1175, align 4, !dbg !18577, !alias.scope !18566, !noundef !10
  %_15.i3848 = urem i32 %frames, %_20.val, !dbg !18576
  %_12.i3849 = add i32 %_15.i3848, %_14.i3847, !dbg !18578
  %_11.i3850 = urem i32 %_12.i3849, %_20.val, !dbg !18579
  store i32 %_11.i3850, ptr %1175, align 4, !dbg !18580, !alias.scope !18566
  br label %bb32, !dbg !18581

bb18:                                             ; preds = %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_27 = tail call noundef zeroext i1 @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(100) %_24) #32, !dbg !18582
  br i1 %_27, label %bb20, label %bb24, !dbg !18583

bb20:                                             ; preds = %bb18
  %_59.not = icmp ugt i32 %words, %left_io.1
  br i1 %_59.not, label %bb40, label %bb1.i3858, !dbg !18584, !prof !4694

bb24:                                             ; preds = %bb12.i3872, %bb1.i3858, %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit, %bb18
  %_26.sroa.0.0.off0 = phi i1 [ false, %_RINvCsjLJhryqjeDL_17true_peak_limiter18limiter_block_monoNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4EB2_.exit ], [ false, %bb18 ], [ false, %bb12.i3872 ], [ true, %bb1.i3858 ]
  %1176 = zext i1 %_26.sroa.0.0.off0 to i8, !dbg !18592
  store i8 %1176, ptr %23, align 4, !dbg !18592
  %1177 = load i8, ptr %2, align 8, !dbg !18593, !range !4765, !noundef !10
  store i8 %1177, ptr %0, align 1, !dbg !18594
  %fst_len.i3853 = and i32 %left_io.1, 536870908, !dbg !18595
  %_22.not.i9898 = icmp eq i32 %fst_len.i3853, 0, !dbg !18599
  br i1 %_22.not.i9898, label %bb32, label %bb13.i1181, !dbg !18599

bb13.i1181:                                       ; preds = %bb24, %bb13.i1181
  %iter.sroa.0.0.i11809901 = phi ptr [ %_27.i1182, %bb13.i1181 ], [ %left_io.0, %bb24 ]
  %iter.sroa.5.0.i9900 = phi i32 [ %_28.i, %bb13.i1181 ], [ %fst_len.i3853, %bb24 ]
  %ok.i.sroa.0.09899 = phi <4 x i32> [ %1180, %bb13.i1181 ], [ splat (i32 -1), %bb24 ]
  %_27.i1182 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i11809901, i32 16, !dbg !18602
  %_28.i = add i32 %iter.sroa.5.0.i9900, -4, !dbg !18605
  %lanes.i.sroa.0.0.copyload = load <4 x float>, ptr %iter.sroa.0.0.i11809901, align 4, !dbg !18606, !alias.scope !18611, !noalias !18615
  %1178 = tail call <4 x float> @llvm.fabs.v4f32(<4 x float> %lanes.i.sroa.0.0.copyload), !dbg !18619
  %1179 = fcmp olt <4 x float> %1178, splat (float 0x46293E5940000000), !dbg !18623
  %1180 = select <4 x i1> %1179, <4 x i32> %ok.i.sroa.0.09899, <4 x i32> zeroinitializer, !dbg !18628
  %_22.not.i = icmp eq i32 %_28.i, 0, !dbg !18599
  br i1 %_22.not.i, label %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit, label %bb13.i1181, !dbg !18599

_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %bb13.i1181
  %1181 = bitcast <4 x i32> %1180 to <16 x i8>, !dbg !18632
  %1182 = xor <16 x i8> %1181, splat (i8 -1), !dbg !18632
  %1183 = tail call i32 @llvm.wasm.anytrue.v16i8(<16 x i8> %1182), !dbg !18636
  %1184 = icmp eq i32 %1183, 0, !dbg !18636
  br i1 %1184, label %bb32, label %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit, !dbg !18637

bb1.i3858:                                        ; preds = %bb20, %bb12.i3872
  %io.sroa.5.0.i3859 = phi i32 [ %len.i.i.i3865, %bb12.i3872 ], [ %words, %bb20 ]
  %io.sroa.0.0.i3860 = phi ptr [ %data.i.i.i3864, %bb12.i3872 ], [ %left_io.0, %bb20 ]
  %1185 = icmp eq i32 %io.sroa.5.0.i3859, 0, !dbg !18638
  br i1 %1185, label %bb24, label %bb13.preheader.i3861, !dbg !18638

bb13.preheader.i3861:                             ; preds = %bb1.i3858
  %spec.store.select.i3862 = tail call i32 @llvm.umin.i32(i32 %io.sroa.5.0.i3859, i32 32), !dbg !18641
  %data.i.i.idx.i3863 = shl nuw nsw i32 %spec.store.select.i3862, 2, !dbg !18644
  %data.i.i.i3864 = getelementptr inbounds nuw i8, ptr %io.sroa.0.0.i3860, i32 %data.i.i.idx.i3863, !dbg !18644
  br label %bb13.i3866, !dbg !18649

bb13.i3866:                                       ; preds = %bb13.i3866, %bb13.preheader.i3861
  %iter.sroa.0.08.i3867 = phi ptr [ %_35.i3869, %bb13.i3866 ], [ %io.sroa.0.0.i3860, %bb13.preheader.i3861 ]
  %bits.sroa.0.07.i3868 = phi i32 [ %1186, %bb13.i3866 ], [ 0, %bb13.preheader.i3861 ]
  %_35.i3869 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i3867, i32 4, !dbg !18651
  %_95.i3870 = load i32, ptr %iter.sroa.0.08.i3867, align 4, !dbg !18653, !alias.scope !18654, !noundef !10
  %1186 = or i32 %_95.i3870, %bits.sroa.0.07.i3868, !dbg !18657
  %_29.i3871 = icmp eq ptr %_35.i3869, %data.i.i.i3864, !dbg !18658
  br i1 %_29.i3871, label %bb12.i3872, label %bb13.i3866, !dbg !18649

bb12.i3872:                                       ; preds = %bb13.i3866
  %len.i.i.i3865 = sub nuw nsw i32 %io.sroa.5.0.i3859, %spec.store.select.i3862, !dbg !18660
  %1187 = icmp eq i32 %1186, 0, !dbg !18661
  br i1 %1187, label %bb1.i3858, label %bb24, !dbg !18661

bb40:                                             ; preds = %bb20
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef 0, i32 noundef %words, i32 noundef %left_io.1, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_ec68fe2eaf9f84061aee17781c0add97) #33, !dbg !18662
  unreachable, !dbg !18662

_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit: ; preds = %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit
; call effect_runtime::bank::nonfinite_lane_mask::<wide::f32x4_::f32x4>
  %_33 = tail call fastcc noundef i32 @_RINvNtCscUXHzrJquta_14effect_runtime4bank19nonfinite_lane_maskNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %left_io.0, i32 noundef %left_io.1) #32, !dbg !18663
  %1188 = getelementptr inbounds nuw i8, ptr %self, i32 8, !dbg !18664
  store i32 %_33, ptr %1188, align 8, !dbg !18664
  %_36 = load i64, ptr %self, align 16, !dbg !18665, !noundef !10
  %1189 = tail call i64 @llvm.uadd.sat.i64(i64 %_36, i64 1), !dbg !18666
  store i64 %1189, ptr %self, align 16, !dbg !18669
  %.idx.i = shl nuw nsw i32 %left_io.1, 2, !dbg !18670
  call void @llvm.memset.p0.i32(ptr nonnull align 4 %left_io.0, i8 0, i32 %.idx.i, i1 false), !dbg !18677, !alias.scope !18678
  call void @llvm.lifetime.start.p0(ptr nonnull %shape), !dbg !18681
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 4 dereferenceable(12) %shape, ptr noundef nonnull align 16 dereferenceable(12) %_23, i32 12, i1 false), !dbg !18682
  %1190 = getelementptr inbounds nuw i8, ptr %self, i32 880, !dbg !18683
  %rate = load i32, ptr %1190, align 8, !dbg !18683, !noundef !10
  %1191 = getelementptr inbounds nuw i8, ptr %self, i32 808, !dbg !18685
  %_70.0 = load ptr, ptr %1191, align 8, !dbg !18685, !nonnull !10, !noundef !10
  %1192 = getelementptr inbounds nuw i8, ptr %self, i32 812, !dbg !18685
  %_70.1 = load i32, ptr %1192, align 4, !dbg !18685, !noundef !10
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_24, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_70.0, i32 noundef %_70.1, i32 noundef %rate) #32, !dbg !18687
  %_44 = getelementptr inbounds nuw i8, ptr %self, i32 1024, !dbg !18688
  %1193 = getelementptr inbounds nuw i8, ptr %self, i32 816, !dbg !18689
  %_71.0 = load ptr, ptr %1193, align 16, !dbg !18689, !nonnull !10, !noundef !10
  %1194 = getelementptr inbounds nuw i8, ptr %self, i32 820, !dbg !18689
  %_71.1 = load i32, ptr %1194, align 4, !dbg !18689, !noundef !10
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsjLJhryqjeDL_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 4 dereferenceable(100) %_44, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %shape, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_71.0, i32 noundef %_71.1, i32 noundef %rate) #32, !dbg !18690
  store i32 0, ptr %_25, align 16, !dbg !18691
  %1195 = getelementptr inbounds nuw i8, ptr %self, i32 804, !dbg !18691
  store i32 0, ptr %1195, align 4, !dbg !18691
  call void @llvm.lifetime.end.p0(ptr nonnull %shape), !dbg !18692
  br label %bb32, !dbg !18693

bb32:                                             ; preds = %bb24, %_RNvMs1_CsjLJhryqjeDL_17true_peak_limiterNtB5_7Cursors7advance.exit, %_RINvNtCscUXHzrJquta_14effect_runtime4bank11check_blockNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ECsjLJhryqjeDL_17true_peak_limiter.exit, %_RNvXs_NtNtCsdkdt1aaAg1T_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsjLJhryqjeDL_17true_peak_limiter.exit
  ret void, !dbg !18693
}
