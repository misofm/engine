define void @_RNvXse_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank(ptr dead_on_unwind noalias noundef writable writeonly sret([328 x i8]) align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef align 32 dereferenceable(2656) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(112) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !12529 {
start:
  %words.i65.i = alloca [32 x i8], align 32
  %words.i64.i = alloca [32 x i8], align 32
  %words.i63.i = alloca [32 x i8], align 32
  %words.i.i = alloca [32 x i8], align 32
  %_84.i = alloca [32 x i8], align 32
  %pending.i = alloca [64 x i8], align 4
  %words.i18.i.i.i = alloca [32 x i8], align 32
  %words.i17.i143.i.i = alloca [32 x i8], align 32
  %words.i.i144.i.i = alloca [32 x i8], align 32
  %words.i17.i.i.i = alloca [32 x i8], align 32
  %words.i16.i.i.i = alloca [32 x i8], align 32
  %words.i15.i.i.i = alloca [32 x i8], align 32
  %words.i14.i.i.i = alloca [32 x i8], align 32
  %words.i13.i.i.i = alloca [32 x i8], align 32
  %words.i12.i.i.i = alloca [32 x i8], align 32
  %words.i.i.i.i = alloca [32 x i8], align 32
  %_15.i141.i.i = alloca [32 x i8], align 32
  %iter.i.i = alloca [56 x i8], align 8
  %reports = alloca [320 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 2624, !dbg !12531
  %1 = load i8, ptr %0, align 32, !dbg !12531, !range !1313, !noundef !12
  %.not = icmp eq i8 %1, 2, !dbg !12532
  br i1 %.not, label %bb13, label %bb14, !dbg !12535, !prof !180

bb14:                                             ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %block, i64 108, !dbg !12536
  %3 = load i8, ptr %2, align 4, !dbg !12536, !range !1328, !noundef !12
  %_44.not = icmp eq i8 %3, %1, !dbg !12543
  %4 = getelementptr inbounds nuw i8, ptr %block, i64 64
  %5 = load ptr, ptr %4, align 8
  %.not7 = icmp eq ptr %5, null
  %or.cond = select i1 %_44.not, i1 %.not7, i1 false, !dbg !12540
  br i1 %or.cond, label %bb4, label %bb3, !dbg !12540

bb13:                                             ; preds = %start
; call core::option::expect_failed
  tail call void @_RNvNtCs4NRVxsYgnAr_4core6option13expect_failed(ptr noalias noundef nonnull readonly captures(address, read_provenance) @alloc_376120b9c5efdf3d59386c16952a74b7, i64 noundef 31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d54ae23796fd62146400a9560f58d006) #24, !dbg !12546
  unreachable, !dbg !12546

bb3:                                              ; preds = %bb14
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %_0, i8 0, i64 320, i1 false), !dbg !12547
  %report.sroa.7.0._0.sroa_idx16 = getelementptr inbounds nuw i8, ptr %_0, i64 320, !dbg !12547
  store i8 %1, ptr %report.sroa.7.0._0.sroa_idx16, align 8, !dbg !12547
  br label %bb12, !dbg !12548

bb4:                                              ; preds = %bb14
  call void @llvm.lifetime.start.p0(ptr nonnull %reports), !dbg !12549
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %reports, i8 0, i64 320, i1 false), !dbg !12550
  %6 = getelementptr inbounds nuw i8, ptr %block, i64 48
  %_36.0 = load ptr, ptr %6, align 8, !nonnull !12, !align !3533, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %block, i64 56
  %_36.1 = load i64, ptr %7, align 8, !noundef !12
  %8 = getelementptr inbounds nuw i8, ptr %block, i64 32
  %_39.0 = load ptr, ptr %8, align 8, !nonnull !12, !align !4797
  %9 = getelementptr inbounds nuw i8, ptr %block, i64 40
  %_39.1 = load i64, ptr %9, align 8
  %10 = getelementptr inbounds nuw i8, ptr %block, i64 96
  %_23 = load i64, ptr %10, align 8
  %_7.sroa.5124.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 8
  %_7.sroa.6127.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 16
  %_7.sroa.7130.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 24
  %11 = getelementptr inbounds nuw i8, ptr %pending.i, i64 32
  %_7.sroa.5124.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 40
  %_7.sroa.6127.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 48
  %_7.sroa.7130.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 56
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 2596
  %13 = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 2620
  %15 = call i64 @llvm.usub.sat.i64(i64 %_36.1, i64 1), !dbg !12551
  br label %bb15, !dbg !12551

bb16:                                             ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit
  %16 = getelementptr inbounds nuw i8, ptr %block, i64 104, !dbg !12559
  %_27 = load i32, ptr %16, align 8, !dbg !12559, !noundef !12
  %frames = zext i32 %_27 to i64, !dbg !12559
  %_37.0 = load ptr, ptr %block, align 8, !dbg !12560, !nonnull !12, !align !3533, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !12560
  %_37.1 = load i64, ptr %17, align 8, !dbg !12560, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !12562
  %_38.0 = load ptr, ptr %18, align 8, !dbg !12562, !nonnull !12, !align !3533, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !12562
  %_38.1 = load i64, ptr %19, align 8, !dbg !12562, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12563), !dbg !12566
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12567), !dbg !12566
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12569), !dbg !12566
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12571), !dbg !12566
  %_9.i = load i32, ptr %14, align 4, !dbg !12573, !alias.scope !12563, !noalias !12577, !noundef !12
  %20 = tail call i32 @llvm.umin.i32(i32 %_27, i32 %_9.i), !dbg !12579
  %..i.i = zext i32 %20 to i64, !dbg !12579
  %_10.not.i = icmp eq i32 %20, 0, !dbg !12581
  br i1 %_10.not.i, label %bb4.i, label %bb9.i, !dbg !12581

bb4.i:                                            ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKb1_EB3_.exit.i, %bb16
  %_18.i = icmp ugt i32 %_27, %_9.i, !dbg !12583
  br i1 %_18.i, label %bb20.i, label %bb7.i, !dbg !12583

bb9.i:                                            ; preds = %bb16
  %21 = shl nuw nsw i64 %..i.i, 3, !dbg !12584
  %_33.not.i = icmp samesign ugt i64 %21, %_37.1
  br i1 %_33.not.i, label %bb16.i, label %bb14.i, !dbg !12585, !prof !2561

bb16.i:                                           ; preds = %bb9.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %21, i64 noundef range(i64 0, 2305843009213693952) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_dd5f55065f566218c9f31cb2a4357231) #24, !dbg !12596, !noalias !12577
  unreachable, !dbg !12596

bb14.i:                                           ; preds = %bb9.i
  %_41.not.i = icmp samesign ugt i64 %21, %_38.1, !dbg !12597
  br i1 %_41.not.i, label %bb19.i, label %bb18.i, !dbg !12597, !prof !180

bb19.i:                                           ; preds = %bb14.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %21, i64 noundef range(i64 0, 2305843009213693952) %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1d7cc6e40c752396aa7def7556a6c433) #24, !dbg !12603, !noalias !12577
  unreachable, !dbg !12603

bb18.i:                                           ; preds = %bb14.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12604), !dbg !12607
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12608), !dbg !12607
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12610), !dbg !12607
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1888, !dbg !12612
  %_15.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1152, !dbg !12625
  %data.i.i887.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !12627
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 2572, !dbg !12632
  %_29.i.i = load i32, ptr %22, align 4, !dbg !12632, !range !1335, !alias.scope !12636, !noalias !12637, !noundef !12
  %_31.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1184, !dbg !12639
  %_33.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1248, !dbg !12640
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12641), !dbg !12644
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12645), !dbg !12644
  %23 = icmp eq i32 %_29.i.i, 1, !dbg !12647
  br i1 %23, label %bb7.i.i, label %bb1.i.i.preheader.i.i, !dbg !12647

bb1.i.i.preheader.i.i:                            ; preds = %bb18.i
  %.val.i.i.i.i = load i32, ptr %_31.i.i, align 4, !dbg !12649, !alias.scope !12652, !noalias !12653, !noundef !12
  %.val1.i.i.i.i = load i32, ptr %_33.i.i, align 4, !dbg !12649, !alias.scope !12656, !noalias !12657, !noundef !12
  %_0.i.i.not.i.i.i.i = icmp eq i32 %.val.i.i.i.i, %.val1.i.i.i.i, !dbg !12658
  br i1 %_0.i.i.not.i.i.i.i, label %bb1.i.i.1.i.i, label %bb7.i.i, !dbg !12649

bb1.i.i.1.i.i:                                    ; preds = %bb1.i.i.preheader.i.i
  %_3.i.i.i.i.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1188, !dbg !12663
  %_3.i1.i.i.i.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1252, !dbg !12668
  %.val.i.i.1.i.i = load i32, ptr %_3.i.i.i.i.i.1.i.i, align 4, !dbg !12649, !alias.scope !12652, !noalias !12653, !noundef !12
  %.val1.i.i.1.i.i = load i32, ptr %_3.i1.i.i.i.i.1.i.i, align 4, !dbg !12649, !alias.scope !12656, !noalias !12657, !noundef !12
  %_0.i.i.not.i.i.1.i.i = icmp eq i32 %.val.i.i.1.i.i, %.val1.i.i.1.i.i, !dbg !12658
  br i1 %_0.i.i.not.i.i.1.i.i, label %bb1.i.i.2.i.i, label %bb7.i.i, !dbg !12649

bb1.i.i.2.i.i:                                    ; preds = %bb1.i.i.1.i.i
  %_3.i.i.i.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1192, !dbg !12663
  %_3.i1.i.i.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1256, !dbg !12668
  %.val.i.i.2.i.i = load i32, ptr %_3.i.i.i.i.i.2.i.i, align 4, !dbg !12649, !alias.scope !12652, !noalias !12653, !noundef !12
  %.val1.i.i.2.i.i = load i32, ptr %_3.i1.i.i.i.i.2.i.i, align 4, !dbg !12649, !alias.scope !12656, !noalias !12657, !noundef !12
  %_0.i.i.not.i.i.2.i.i = icmp eq i32 %.val.i.i.2.i.i, %.val1.i.i.2.i.i, !dbg !12658
  br i1 %_0.i.i.not.i.i.2.i.i, label %bb1.i.i.3.i.i, label %bb7.i.i, !dbg !12649

bb1.i.i.3.i.i:                                    ; preds = %bb1.i.i.2.i.i
  %_3.i.i.i.i.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1196, !dbg !12663
  %_3.i1.i.i.i.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1260, !dbg !12668
  %.val.i.i.3.i.i = load i32, ptr %_3.i.i.i.i.i.3.i.i, align 4, !dbg !12649, !alias.scope !12652, !noalias !12653, !noundef !12
  %.val1.i.i.3.i.i = load i32, ptr %_3.i1.i.i.i.i.3.i.i, align 4, !dbg !12649, !alias.scope !12656, !noalias !12657, !noundef !12
  %_0.i.i.not.i.i.3.i.i = icmp eq i32 %.val.i.i.3.i.i, %.val1.i.i.3.i.i, !dbg !12658
  br i1 %_0.i.i.not.i.i.3.i.i, label %bb1.i.i.4.i.i, label %bb7.i.i, !dbg !12649

bb1.i.i.4.i.i:                                    ; preds = %bb1.i.i.3.i.i
  %_3.i.i.i.i.i.4.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1200, !dbg !12663
  %_3.i1.i.i.i.i.4.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1264, !dbg !12668
  %.val.i.i.4.i.i = load i32, ptr %_3.i.i.i.i.i.4.i.i, align 4, !dbg !12649, !alias.scope !12652, !noalias !12653, !noundef !12
  %.val1.i.i.4.i.i = load i32, ptr %_3.i1.i.i.i.i.4.i.i, align 4, !dbg !12649, !alias.scope !12656, !noalias !12657, !noundef !12
  %_0.i.i.not.i.i.4.i.i = icmp eq i32 %.val.i.i.4.i.i, %.val1.i.i.4.i.i, !dbg !12658
  br i1 %_0.i.i.not.i.i.4.i.i, label %bb1.i.i.5.i.i, label %bb7.i.i, !dbg !12649

bb1.i.i.5.i.i:                                    ; preds = %bb1.i.i.4.i.i
  %_3.i.i.i.i.i.5.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1204, !dbg !12663
  %_3.i1.i.i.i.i.5.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1268, !dbg !12668
  %.val.i.i.5.i.i = load i32, ptr %_3.i.i.i.i.i.5.i.i, align 4, !dbg !12649, !alias.scope !12652, !noalias !12653, !noundef !12
  %.val1.i.i.5.i.i = load i32, ptr %_3.i1.i.i.i.i.5.i.i, align 4, !dbg !12649, !alias.scope !12656, !noalias !12657, !noundef !12
  %_0.i.i.not.i.i.5.i.i = icmp eq i32 %.val.i.i.5.i.i, %.val1.i.i.5.i.i, !dbg !12658
  br i1 %_0.i.i.not.i.i.5.i.i, label %bb1.i.i.6.i.i, label %bb7.i.i, !dbg !12649

bb1.i.i.6.i.i:                                    ; preds = %bb1.i.i.5.i.i
  %_3.i.i.i.i.i.6.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !12663
  %_3.i1.i.i.i.i.6.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1272, !dbg !12668
  %.val.i.i.6.i.i = load i32, ptr %_3.i.i.i.i.i.6.i.i, align 4, !dbg !12649, !alias.scope !12652, !noalias !12653, !noundef !12
  %.val1.i.i.6.i.i = load i32, ptr %_3.i1.i.i.i.i.6.i.i, align 4, !dbg !12649, !alias.scope !12656, !noalias !12657, !noundef !12
  %_0.i.i.not.i.i.6.i.i = icmp eq i32 %.val.i.i.6.i.i, %.val1.i.i.6.i.i, !dbg !12658
  br i1 %_0.i.i.not.i.i.6.i.i, label %bb1.i.i.7.i.i, label %bb7.i.i, !dbg !12649

bb1.i.i.7.i.i:                                    ; preds = %bb1.i.i.6.i.i
  %_3.i.i.i.i.i.7.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !12663
  %_3.i1.i.i.i.i.7.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1276, !dbg !12668
  %.val.i.i.7.i.i = load i32, ptr %_3.i.i.i.i.i.7.i.i, align 4, !dbg !12649, !alias.scope !12652, !noalias !12653, !noundef !12
  %.val1.i.i.7.i.i = load i32, ptr %_3.i1.i.i.i.i.7.i.i, align 4, !dbg !12649, !alias.scope !12656, !noalias !12657, !noundef !12
  %_0.i.i.not.i.i.7.i.i = icmp eq i32 %.val.i.i.7.i.i, %.val1.i.i.7.i.i, !dbg !12658
  %spec.select.i.i = select i1 %_0.i.i.not.i.i.7.i.i, i8 1, i8 2, !dbg !12649
  br label %bb7.i.i, !dbg !12649

bb7.i.i:                                          ; preds = %bb1.i.i.7.i.i, %bb1.i.i.6.i.i, %bb1.i.i.5.i.i, %bb1.i.i.4.i.i, %bb1.i.i.3.i.i, %bb1.i.i.2.i.i, %bb1.i.i.1.i.i, %bb1.i.i.preheader.i.i, %bb18.i
  %_0.sroa.0.0.i892.i.i = phi i8 [ 0, %bb18.i ], [ 2, %bb1.i.i.preheader.i.i ], [ 2, %bb1.i.i.4.i.i ], [ 2, %bb1.i.i.6.i.i ], [ 2, %bb1.i.i.1.i.i ], [ %spec.select.i.i, %bb1.i.i.7.i.i ], [ 2, %bb1.i.i.2.i.i ], [ 2, %bb1.i.i.5.i.i ], [ 2, %bb1.i.i.3.i.i ], !dbg !12671
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !12672
  %_38.i.i = getelementptr inbounds nuw i8, ptr %self, i64 960, !dbg !12674
  %_64.0.i.i = load ptr, ptr %_15.i.i, align 8, !dbg !12675, !alias.scope !12636, !noalias !12637, !nonnull !12, !noundef !12
  %25 = getelementptr inbounds nuw i8, ptr %self, i64 1160, !dbg !12675
  %_64.1.i.i = load i64, ptr %25, align 8, !dbg !12675, !alias.scope !12636, !noalias !12637, !noundef !12
  %_66.0.i.i = load ptr, ptr %data.i.i887.i.i, align 8, !dbg !12676, !alias.scope !12636, !noalias !12637, !nonnull !12, !noundef !12
  %26 = getelementptr inbounds nuw i8, ptr %self, i64 1224, !dbg !12676
  %_66.1.i.i = load i64, ptr %26, align 8, !dbg !12676, !alias.scope !12636, !noalias !12637, !noundef !12
  %_57.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2608, !dbg !12677
  %27 = getelementptr inbounds nuw i8, ptr %self, i64 2612, !dbg !12678
  %_58.i.i = load i32, ptr %27, align 4, !dbg !12678, !alias.scope !12636, !noalias !12637, !noundef !12
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 2616, !dbg !12679
  %_59.i.i = load i32, ptr %28, align 8, !dbg !12679, !alias.scope !12636, !noalias !12637, !noundef !12
  %base.i.i.i = load i32, ptr %_57.i.i, align 4, !dbg !12680, !alias.scope !12636, !noalias !12690, !noundef !12
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 1792
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 1824
  %33 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 1856
  %35 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %36 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 2400
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 2432
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 2464
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %45 = getelementptr inbounds nuw i8, ptr %self, i64 1188
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 1252
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 1192
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 1256
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 1196
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 1260
  %51 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %52 = getelementptr inbounds nuw i8, ptr %self, i64 1264
  %53 = getelementptr inbounds nuw i8, ptr %self, i64 1204
  %54 = getelementptr inbounds nuw i8, ptr %self, i64 1268
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1208
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 1272
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 1212
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 1276
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %60 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %iter1.sroa.0.0.ptr.i120.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %64 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %iter1.sroa.0.0.ptr.i120.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1536
  %65 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 1600
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 1568
  %iter1.sroa.0.0.ptr.i120.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 1760
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 1728
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 1984
  %72 = getelementptr inbounds nuw i8, ptr %self, i64 1952
  %73 = getelementptr inbounds nuw i8, ptr %self, i64 1920
  %iter1.sroa.0.0.ptr.i.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2016
  %74 = getelementptr inbounds nuw i8, ptr %self, i64 2112
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 2080
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 2048
  %iter1.sroa.0.0.ptr.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2144
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 2240
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 2208
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 2176
  %iter1.sroa.0.0.ptr.i.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2272
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 2368
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 2336
  %82 = getelementptr inbounds nuw i8, ptr %self, i64 2304
  br label %bb30.i.i.i, !dbg !12693

bb30.i.i.i:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i, %bb7.i.i
  %iter.sroa.0.0.i1825.i.i = phi i64 [ 0, %bb7.i.i ], [ %83, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i ]
  %83 = add nuw nsw i64 %iter.sroa.0.0.i1825.i.i, 1, !dbg !12702
  %span.i.i.i = shl i64 %iter.sroa.0.0.i1825.i.i, 3, !dbg !12708
  %_28.i.i.i = trunc i64 %iter.sroa.0.0.i1825.i.i to i32, !dbg !12710
  %now.i.i.i = add i32 %base.i.i.i, %_28.i.i.i, !dbg !12712
  %_31.i.i.i = and i32 %now.i.i.i, %_58.i.i, !dbg !12715
  %_30.i.i.i = zext i32 %_31.i.i.i to i64, !dbg !12717
  %write.i.i.i = shl nuw nsw i64 %_30.i.i.i, 3, !dbg !12717
  %exitcond.not.i.i = icmp eq i64 %iter.sroa.0.0.i1825.i.i, %..i.i, !dbg !12718
  br i1 %exitcond.not.i.i, label %bb33.i.i.i, label %bb32.i.i.i, !dbg !12718, !prof !2561

bb33.i.i.i:                                       ; preds = %bb30.i.i.i
  %_35.i.i.i = add nuw nsw i64 %span.i.i.i, 8, !dbg !12726
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i.i, i64 noundef %_35.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %21, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d0dbd696a058609bd94bbe1fa75d0723) #24, !dbg !12727, !noalias !12690
  unreachable, !dbg !12727

bb32.i.i.i:                                       ; preds = %bb30.i.i.i
  %_97.i.i.i = getelementptr inbounds nuw float, ptr %_37.0, i64 %span.i.i.i, !dbg !12728
  %_37.i.i.i = add nuw nsw i64 %write.i.i.i, 8, !dbg !12732
  %_98.not.i.i.i = icmp ugt i64 %_37.i.i.i, %_64.1.i.i, !dbg !12733
  br i1 %_98.not.i.i.i, label %bb36.i.i.i, label %bb38.i.i.i, !dbg !12733, !prof !180

bb36.i.i.i:                                       ; preds = %bb32.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i.i, i64 noundef %_37.i.i.i, i64 noundef %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1eeef3195352adc58f4bfdb015316fef) #24, !dbg !12738, !noalias !12690
  unreachable, !dbg !12738

bb38.i.i.i:                                       ; preds = %bb32.i.i.i
  %_107.i.i.i = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %write.i.i.i, !dbg !12739
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_107.i.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_97.i.i.i, i64 32, i1 false), !dbg !12743, !noalias !12752
  %_115.i.i.i = getelementptr inbounds nuw float, ptr %_38.0, i64 %span.i.i.i, !dbg !12753
  %_116.not.i.i.i = icmp ugt i64 %_37.i.i.i, %_66.1.i.i, !dbg !12760
  br i1 %_116.not.i.i.i, label %bb41.i.i.i, label %bb40.i.i.i, !dbg !12760, !prof !180

bb41.i.i.i:                                       ; preds = %bb38.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i.i, i64 noundef %_37.i.i.i, i64 noundef %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9d56679ca30f2caa500ddf2e89d00cb) #24, !dbg !12764, !noalias !12690
  unreachable, !dbg !12764

bb40.i.i.i:                                       ; preds = %bb38.i.i.i
  %_123.i.i.i = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %write.i.i.i, !dbg !12765
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_123.i.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_115.i.i.i, i64 32, i1 false), !dbg !12769, !noalias !12774
  %_53.i.i.i = sub i32 %now.i.i.i, %_59.i.i, !dbg !12775
  %_52.i.i.i = and i32 %_53.i.i.i, %_58.i.i, !dbg !12778
  %_51.i.i.i = zext i32 %_52.i.i.i to i64, !dbg !12779
  %read.i.i.i = shl nuw nsw i64 %_51.i.i.i, 3, !dbg !12779
  %_56.i.i.i = add nuw nsw i64 %read.i.i.i, 8, !dbg !12780
  %_156.not.i.i.i = icmp ugt i64 %_56.i.i.i, %_64.1.i.i, !dbg !12782
  br i1 %_156.not.i.i.i, label %bb51.i.i.i, label %bb50.i.i.i, !dbg !12782, !prof !180

bb51.i.i.i:                                       ; preds = %bb40.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i.i, i64 noundef %_56.i.i.i, i64 noundef %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7) #24, !dbg !12786, !noalias !12690
  unreachable, !dbg !12786

bb50.i.i.i:                                       ; preds = %bb40.i.i.i
  %_163.i.i.i = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %read.i.i.i, !dbg !12787
  %lanes.i471.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_163.i.i.i, align 4, !dbg !12791, !alias.scope !12799, !noalias !12803
  %_164.not.i.i.i = icmp ugt i64 %_56.i.i.i, %_66.1.i.i, !dbg !12807
  br i1 %_164.not.i.i.i, label %bb54.i.i.i, label %bb53.i.i.i, !dbg !12807, !prof !180

bb54.i.i.i:                                       ; preds = %bb50.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i.i, i64 noundef %_56.i.i.i, i64 noundef %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f198228fd40f4c04a48a745576c5c5ee) #24, !dbg !12812, !noalias !12690
  unreachable, !dbg !12812

bb53.i.i.i:                                       ; preds = %bb50.i.i.i
  %_169.i.i.i = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %read.i.i.i, !dbg !12813
  %lanes.i465.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_169.i.i.i, align 4, !dbg !12817, !alias.scope !12822, !noalias !12826
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12830), !dbg !12833
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12836), !dbg !12833
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12838), !dbg !12833
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12840), !dbg !12833
  %_18.i71.i.i = load i32, ptr %_31.i.i, align 4, !dbg !12842, !alias.scope !12844, !noalias !12845, !noundef !12
  %_17.i.i.i = sub i32 %now.i.i.i, %_18.i71.i.i, !dbg !12847
  %_16.i72.i.i = and i32 %_17.i.i.i, %_58.i.i, !dbg !12842
  %_15.i73.i.i = zext i32 %_16.i72.i.i to i64, !dbg !12842
  %_14.i.i.i = shl nuw nsw i64 %_15.i73.i.i, 3, !dbg !12842
  %_26.i.i.i = load i32, ptr %_33.i.i, align 4, !dbg !12842, !alias.scope !12849, !noalias !12850, !noundef !12
  %_25.i.i.i = sub i32 %now.i.i.i, %_26.i.i.i, !dbg !12847
  %_24.i.i.i = and i32 %_25.i.i.i, %_58.i.i, !dbg !12842
  %_23.i76.i.i = zext i32 %_24.i.i.i to i64, !dbg !12842
  %_22.i.i.i = shl nuw nsw i64 %_23.i76.i.i, 3, !dbg !12842
  %_31.i77.i.i = icmp samesign ult i64 %_14.i.i.i, %_64.1.i.i, !dbg !12842
  switch i8 %_0.sroa.0.0.i892.i.i, label %bb53.i.i.i.unreachabledefault [
    i8 0, label %bb7.i75.preheader.i.i
    i8 1, label %bb16.i.preheader.i.i
    i8 2, label %bb25.i.preheader.i.i
  ], !dbg !12851

bb25.i.preheader.i.i:                             ; preds = %bb53.i.i.i
  br i1 %_31.i77.i.i, label %bb27.i58.i.i, label %panic28.i.i.i, !dbg !12852

bb16.i.preheader.i.i:                             ; preds = %bb53.i.i.i
  br i1 %_31.i77.i.i, label %bb17.i.i.i, label %panic15.i.i.i, !dbg !12853

bb7.i75.preheader.i.i:                            ; preds = %bb53.i.i.i
  br i1 %_31.i77.i.i, label %bb8.i78.i.i, label %panic4.i.i.i, !dbg !12854

bb53.i.i.i.unreachabledefault:                    ; preds = %bb53.i.i.i
  unreachable

default.unreachable:                              ; preds = %bb4.i10, %bb53.i.i74.i
  unreachable

bb8.i78.i.i:                                      ; preds = %bb7.i75.preheader.i.i
  %_34.i.i.i = icmp samesign ult i64 %_22.i.i.i, %_66.1.i.i, !dbg !12855
  br i1 %_34.i.i.i, label %bb10.i79.i.i, label %panic5.i.i.i, !dbg !12855

panic4.i.i.i:                                     ; preds = %bb10.i79.6.i.i, %bb10.i79.5.i.i, %bb10.i79.4.i.i, %bb10.i79.3.i.i, %bb10.i79.2.i.i, %bb10.i79.1.i.i, %bb10.i79.i.i, %bb7.i75.preheader.i.i
  %left.i.lcssa.i.i = phi i64 [ %_14.i.i.i, %bb7.i75.preheader.i.i ], [ %left.i.1.i.i, %bb10.i79.i.i ], [ %left.i.2.i.i, %bb10.i79.1.i.i ], [ %left.i.3.i.i, %bb10.i79.2.i.i ], [ %left.i.4.i.i, %bb10.i79.3.i.i ], [ %left.i.5.i.i, %bb10.i79.4.i.i ], [ %left.i.6.i.i, %bb10.i79.5.i.i ], [ %left.i.7.i.i, %bb10.i79.6.i.i ], !dbg !12856
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left.i.lcssa.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cba26bb3a04aaba20f9c249edc5e133d) #24, !dbg !12854, !noalias !12857
  unreachable, !dbg !12854

panic5.i.i.i:                                     ; preds = %bb8.i78.7.i.i, %bb8.i78.6.i.i, %bb8.i78.5.i.i, %bb8.i78.4.i.i, %bb8.i78.3.i.i, %bb8.i78.2.i.i, %bb8.i78.1.i.i, %bb8.i78.i.i
  %right.i.lcssa1840.i.i = phi i64 [ %_22.i.i.i, %bb8.i78.i.i ], [ %right.i.1.i.i, %bb8.i78.1.i.i ], [ %right.i.2.i.i, %bb8.i78.2.i.i ], [ %right.i.3.i.i, %bb8.i78.3.i.i ], [ %right.i.4.i.i, %bb8.i78.4.i.i ], [ %right.i.5.i.i, %bb8.i78.5.i.i ], [ %right.i.6.i.i, %bb8.i78.6.i.i ], [ %right.i.7.i.i, %bb8.i78.7.i.i ], !dbg !12858
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right.i.lcssa1840.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2ad2d313de59c36d62f4a7d75017a71f) #24, !dbg !12855, !noalias !12857
  unreachable, !dbg !12855

bb10.i79.i.i:                                     ; preds = %bb8.i78.i.i
  %84 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_14.i.i.i, !dbg !12854
  %left_own.i.i.i = load float, ptr %84, align 4, !dbg !12854, !alias.scope !12838, !noalias !12859, !noundef !12
  %85 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_22.i.i.i, !dbg !12855
  %right_own.i.i.i = load float, ptr %85, align 4, !dbg !12855, !alias.scope !12840, !noalias !12860, !noundef !12
  %_18.i71.1.i.i = load i32, ptr %45, align 4, !dbg !12861, !alias.scope !12844, !noalias !12845, !noundef !12
  %_17.i.1.i.i = sub i32 %now.i.i.i, %_18.i71.1.i.i, !dbg !12862
  %_16.i72.1.i.i = and i32 %_17.i.1.i.i, %_58.i.i, !dbg !12864
  %_15.i73.1.i.i = zext i32 %_16.i72.1.i.i to i64, !dbg !12856
  %_14.i.1.i.i = shl nuw nsw i64 %_15.i73.1.i.i, 3, !dbg !12856
  %left.i.1.i.i = or disjoint i64 %_14.i.1.i.i, 1, !dbg !12856
  %_26.i.1.i.i = load i32, ptr %46, align 4, !dbg !12865, !alias.scope !12849, !noalias !12850, !noundef !12
  %_25.i.1.i.i = sub i32 %now.i.i.i, %_26.i.1.i.i, !dbg !12866
  %_24.i.1.i.i = and i32 %_25.i.1.i.i, %_58.i.i, !dbg !12868
  %_23.i76.1.i.i = zext i32 %_24.i.1.i.i to i64, !dbg !12858
  %_22.i.1.i.i = shl nuw nsw i64 %_23.i76.1.i.i, 3, !dbg !12858
  %right.i.1.i.i = or disjoint i64 %_22.i.1.i.i, 1, !dbg !12858
  %_31.i77.1.i.i = icmp samesign ult i64 %left.i.1.i.i, %_64.1.i.i, !dbg !12854
  br i1 %_31.i77.1.i.i, label %bb8.i78.1.i.i, label %panic4.i.i.i, !dbg !12854

bb8.i78.1.i.i:                                    ; preds = %bb10.i79.i.i
  %_34.i.1.i.i = icmp samesign ult i64 %right.i.1.i.i, %_66.1.i.i, !dbg !12855
  br i1 %_34.i.1.i.i, label %bb10.i79.1.i.i, label %panic5.i.i.i, !dbg !12855

bb10.i79.1.i.i:                                   ; preds = %bb8.i78.1.i.i
  %86 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.1.i.i, !dbg !12854
  %left_own.i.1.i.i = load float, ptr %86, align 4, !dbg !12854, !alias.scope !12838, !noalias !12859, !noundef !12
  %87 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.1.i.i, !dbg !12855
  %right_own.i.1.i.i = load float, ptr %87, align 4, !dbg !12855, !alias.scope !12840, !noalias !12860, !noundef !12
  %_18.i71.2.i.i = load i32, ptr %47, align 4, !dbg !12861, !alias.scope !12844, !noalias !12845, !noundef !12
  %_17.i.2.i.i = sub i32 %now.i.i.i, %_18.i71.2.i.i, !dbg !12862
  %_16.i72.2.i.i = and i32 %_17.i.2.i.i, %_58.i.i, !dbg !12864
  %_15.i73.2.i.i = zext i32 %_16.i72.2.i.i to i64, !dbg !12856
  %_14.i.2.i.i = shl nuw nsw i64 %_15.i73.2.i.i, 3, !dbg !12856
  %left.i.2.i.i = or disjoint i64 %_14.i.2.i.i, 2, !dbg !12856
  %_26.i.2.i.i = load i32, ptr %48, align 4, !dbg !12865, !alias.scope !12849, !noalias !12850, !noundef !12
  %_25.i.2.i.i = sub i32 %now.i.i.i, %_26.i.2.i.i, !dbg !12866
  %_24.i.2.i.i = and i32 %_25.i.2.i.i, %_58.i.i, !dbg !12868
  %_23.i76.2.i.i = zext i32 %_24.i.2.i.i to i64, !dbg !12858
  %_22.i.2.i.i = shl nuw nsw i64 %_23.i76.2.i.i, 3, !dbg !12858
  %right.i.2.i.i = or disjoint i64 %_22.i.2.i.i, 2, !dbg !12858
  %_31.i77.2.i.i = icmp samesign ult i64 %left.i.2.i.i, %_64.1.i.i, !dbg !12854
  br i1 %_31.i77.2.i.i, label %bb8.i78.2.i.i, label %panic4.i.i.i, !dbg !12854

bb8.i78.2.i.i:                                    ; preds = %bb10.i79.1.i.i
  %_34.i.2.i.i = icmp samesign ult i64 %right.i.2.i.i, %_66.1.i.i, !dbg !12855
  br i1 %_34.i.2.i.i, label %bb10.i79.2.i.i, label %panic5.i.i.i, !dbg !12855

bb10.i79.2.i.i:                                   ; preds = %bb8.i78.2.i.i
  %88 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.2.i.i, !dbg !12854
  %left_own.i.2.i.i = load float, ptr %88, align 4, !dbg !12854, !alias.scope !12838, !noalias !12859, !noundef !12
  %89 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.2.i.i, !dbg !12855
  %right_own.i.2.i.i = load float, ptr %89, align 4, !dbg !12855, !alias.scope !12840, !noalias !12860, !noundef !12
  %_18.i71.3.i.i = load i32, ptr %49, align 4, !dbg !12861, !alias.scope !12844, !noalias !12845, !noundef !12
  %_17.i.3.i.i = sub i32 %now.i.i.i, %_18.i71.3.i.i, !dbg !12862
  %_16.i72.3.i.i = and i32 %_17.i.3.i.i, %_58.i.i, !dbg !12864
  %_15.i73.3.i.i = zext i32 %_16.i72.3.i.i to i64, !dbg !12856
  %_14.i.3.i.i = shl nuw nsw i64 %_15.i73.3.i.i, 3, !dbg !12856
  %left.i.3.i.i = or disjoint i64 %_14.i.3.i.i, 3, !dbg !12856
  %_26.i.3.i.i = load i32, ptr %50, align 4, !dbg !12865, !alias.scope !12849, !noalias !12850, !noundef !12
  %_25.i.3.i.i = sub i32 %now.i.i.i, %_26.i.3.i.i, !dbg !12866
  %_24.i.3.i.i = and i32 %_25.i.3.i.i, %_58.i.i, !dbg !12868
  %_23.i76.3.i.i = zext i32 %_24.i.3.i.i to i64, !dbg !12858
  %_22.i.3.i.i = shl nuw nsw i64 %_23.i76.3.i.i, 3, !dbg !12858
  %right.i.3.i.i = or disjoint i64 %_22.i.3.i.i, 3, !dbg !12858
  %_31.i77.3.i.i = icmp samesign ult i64 %left.i.3.i.i, %_64.1.i.i, !dbg !12854
  br i1 %_31.i77.3.i.i, label %bb8.i78.3.i.i, label %panic4.i.i.i, !dbg !12854

bb8.i78.3.i.i:                                    ; preds = %bb10.i79.2.i.i
  %_34.i.3.i.i = icmp samesign ult i64 %right.i.3.i.i, %_66.1.i.i, !dbg !12855
  br i1 %_34.i.3.i.i, label %bb10.i79.3.i.i, label %panic5.i.i.i, !dbg !12855

bb10.i79.3.i.i:                                   ; preds = %bb8.i78.3.i.i
  %90 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.3.i.i, !dbg !12854
  %left_own.i.3.i.i = load float, ptr %90, align 4, !dbg !12854, !alias.scope !12838, !noalias !12859, !noundef !12
  %91 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.3.i.i, !dbg !12855
  %right_own.i.3.i.i = load float, ptr %91, align 4, !dbg !12855, !alias.scope !12840, !noalias !12860, !noundef !12
  %_18.i71.4.i.i = load i32, ptr %51, align 4, !dbg !12861, !alias.scope !12844, !noalias !12845, !noundef !12
  %_17.i.4.i.i = sub i32 %now.i.i.i, %_18.i71.4.i.i, !dbg !12862
  %_16.i72.4.i.i = and i32 %_17.i.4.i.i, %_58.i.i, !dbg !12864
  %_15.i73.4.i.i = zext i32 %_16.i72.4.i.i to i64, !dbg !12856
  %_14.i.4.i.i = shl nuw nsw i64 %_15.i73.4.i.i, 3, !dbg !12856
  %left.i.4.i.i = or disjoint i64 %_14.i.4.i.i, 4, !dbg !12856
  %_26.i.4.i.i = load i32, ptr %52, align 4, !dbg !12865, !alias.scope !12849, !noalias !12850, !noundef !12
  %_25.i.4.i.i = sub i32 %now.i.i.i, %_26.i.4.i.i, !dbg !12866
  %_24.i.4.i.i = and i32 %_25.i.4.i.i, %_58.i.i, !dbg !12868
  %_23.i76.4.i.i = zext i32 %_24.i.4.i.i to i64, !dbg !12858
  %_22.i.4.i.i = shl nuw nsw i64 %_23.i76.4.i.i, 3, !dbg !12858
  %right.i.4.i.i = or disjoint i64 %_22.i.4.i.i, 4, !dbg !12858
  %_31.i77.4.i.i = icmp samesign ult i64 %left.i.4.i.i, %_64.1.i.i, !dbg !12854
  br i1 %_31.i77.4.i.i, label %bb8.i78.4.i.i, label %panic4.i.i.i, !dbg !12854

bb8.i78.4.i.i:                                    ; preds = %bb10.i79.3.i.i
  %_34.i.4.i.i = icmp samesign ult i64 %right.i.4.i.i, %_66.1.i.i, !dbg !12855
  br i1 %_34.i.4.i.i, label %bb10.i79.4.i.i, label %panic5.i.i.i, !dbg !12855

bb10.i79.4.i.i:                                   ; preds = %bb8.i78.4.i.i
  %92 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.4.i.i, !dbg !12854
  %left_own.i.4.i.i = load float, ptr %92, align 4, !dbg !12854, !alias.scope !12838, !noalias !12859, !noundef !12
  %93 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.4.i.i, !dbg !12855
  %right_own.i.4.i.i = load float, ptr %93, align 4, !dbg !12855, !alias.scope !12840, !noalias !12860, !noundef !12
  %_18.i71.5.i.i = load i32, ptr %53, align 4, !dbg !12861, !alias.scope !12844, !noalias !12845, !noundef !12
  %_17.i.5.i.i = sub i32 %now.i.i.i, %_18.i71.5.i.i, !dbg !12862
  %_16.i72.5.i.i = and i32 %_17.i.5.i.i, %_58.i.i, !dbg !12864
  %_15.i73.5.i.i = zext i32 %_16.i72.5.i.i to i64, !dbg !12856
  %_14.i.5.i.i = shl nuw nsw i64 %_15.i73.5.i.i, 3, !dbg !12856
  %left.i.5.i.i = or disjoint i64 %_14.i.5.i.i, 5, !dbg !12856
  %_26.i.5.i.i = load i32, ptr %54, align 4, !dbg !12865, !alias.scope !12849, !noalias !12850, !noundef !12
  %_25.i.5.i.i = sub i32 %now.i.i.i, %_26.i.5.i.i, !dbg !12866
  %_24.i.5.i.i = and i32 %_25.i.5.i.i, %_58.i.i, !dbg !12868
  %_23.i76.5.i.i = zext i32 %_24.i.5.i.i to i64, !dbg !12858
  %_22.i.5.i.i = shl nuw nsw i64 %_23.i76.5.i.i, 3, !dbg !12858
  %right.i.5.i.i = or disjoint i64 %_22.i.5.i.i, 5, !dbg !12858
  %_31.i77.5.i.i = icmp samesign ult i64 %left.i.5.i.i, %_64.1.i.i, !dbg !12854
  br i1 %_31.i77.5.i.i, label %bb8.i78.5.i.i, label %panic4.i.i.i, !dbg !12854

bb8.i78.5.i.i:                                    ; preds = %bb10.i79.4.i.i
  %_34.i.5.i.i = icmp samesign ult i64 %right.i.5.i.i, %_66.1.i.i, !dbg !12855
  br i1 %_34.i.5.i.i, label %bb10.i79.5.i.i, label %panic5.i.i.i, !dbg !12855

bb10.i79.5.i.i:                                   ; preds = %bb8.i78.5.i.i
  %94 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.5.i.i, !dbg !12854
  %left_own.i.5.i.i = load float, ptr %94, align 4, !dbg !12854, !alias.scope !12838, !noalias !12859, !noundef !12
  %95 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.5.i.i, !dbg !12855
  %right_own.i.5.i.i = load float, ptr %95, align 4, !dbg !12855, !alias.scope !12840, !noalias !12860, !noundef !12
  %_18.i71.6.i.i = load i32, ptr %55, align 4, !dbg !12861, !alias.scope !12844, !noalias !12845, !noundef !12
  %_17.i.6.i.i = sub i32 %now.i.i.i, %_18.i71.6.i.i, !dbg !12862
  %_16.i72.6.i.i = and i32 %_17.i.6.i.i, %_58.i.i, !dbg !12864
  %_15.i73.6.i.i = zext i32 %_16.i72.6.i.i to i64, !dbg !12856
  %_14.i.6.i.i = shl nuw nsw i64 %_15.i73.6.i.i, 3, !dbg !12856
  %left.i.6.i.i = or disjoint i64 %_14.i.6.i.i, 6, !dbg !12856
  %_26.i.6.i.i = load i32, ptr %56, align 4, !dbg !12865, !alias.scope !12849, !noalias !12850, !noundef !12
  %_25.i.6.i.i = sub i32 %now.i.i.i, %_26.i.6.i.i, !dbg !12866
  %_24.i.6.i.i = and i32 %_25.i.6.i.i, %_58.i.i, !dbg !12868
  %_23.i76.6.i.i = zext i32 %_24.i.6.i.i to i64, !dbg !12858
  %_22.i.6.i.i = shl nuw nsw i64 %_23.i76.6.i.i, 3, !dbg !12858
  %right.i.6.i.i = or disjoint i64 %_22.i.6.i.i, 6, !dbg !12858
  %_31.i77.6.i.i = icmp samesign ult i64 %left.i.6.i.i, %_64.1.i.i, !dbg !12854
  br i1 %_31.i77.6.i.i, label %bb8.i78.6.i.i, label %panic4.i.i.i, !dbg !12854

bb8.i78.6.i.i:                                    ; preds = %bb10.i79.5.i.i
  %_34.i.6.i.i = icmp samesign ult i64 %right.i.6.i.i, %_66.1.i.i, !dbg !12855
  br i1 %_34.i.6.i.i, label %bb10.i79.6.i.i, label %panic5.i.i.i, !dbg !12855

bb10.i79.6.i.i:                                   ; preds = %bb8.i78.6.i.i
  %96 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.6.i.i, !dbg !12854
  %left_own.i.6.i.i = load float, ptr %96, align 4, !dbg !12854, !alias.scope !12838, !noalias !12859, !noundef !12
  %97 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.6.i.i, !dbg !12855
  %right_own.i.6.i.i = load float, ptr %97, align 4, !dbg !12855, !alias.scope !12840, !noalias !12860, !noundef !12
  %_18.i71.7.i.i = load i32, ptr %57, align 4, !dbg !12861, !alias.scope !12844, !noalias !12845, !noundef !12
  %_17.i.7.i.i = sub i32 %now.i.i.i, %_18.i71.7.i.i, !dbg !12862
  %_16.i72.7.i.i = and i32 %_17.i.7.i.i, %_58.i.i, !dbg !12864
  %_15.i73.7.i.i = zext i32 %_16.i72.7.i.i to i64, !dbg !12856
  %_14.i.7.i.i = shl nuw nsw i64 %_15.i73.7.i.i, 3, !dbg !12856
  %left.i.7.i.i = or disjoint i64 %_14.i.7.i.i, 7, !dbg !12856
  %_26.i.7.i.i = load i32, ptr %58, align 4, !dbg !12865, !alias.scope !12849, !noalias !12850, !noundef !12
  %_25.i.7.i.i = sub i32 %now.i.i.i, %_26.i.7.i.i, !dbg !12866
  %_24.i.7.i.i = and i32 %_25.i.7.i.i, %_58.i.i, !dbg !12868
  %_23.i76.7.i.i = zext i32 %_24.i.7.i.i to i64, !dbg !12858
  %_22.i.7.i.i = shl nuw nsw i64 %_23.i76.7.i.i, 3, !dbg !12858
  %right.i.7.i.i = or disjoint i64 %_22.i.7.i.i, 7, !dbg !12858
  %_31.i77.7.i.i = icmp samesign ult i64 %left.i.7.i.i, %_64.1.i.i, !dbg !12854
  br i1 %_31.i77.7.i.i, label %bb8.i78.7.i.i, label %panic4.i.i.i, !dbg !12854

bb8.i78.7.i.i:                                    ; preds = %bb10.i79.6.i.i
  %_34.i.7.i.i = icmp samesign ult i64 %right.i.7.i.i, %_66.1.i.i, !dbg !12855
  br i1 %_34.i.7.i.i, label %bb10.i79.7.i.i, label %panic5.i.i.i, !dbg !12855

bb10.i79.7.i.i:                                   ; preds = %bb8.i78.7.i.i
  %98 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.7.i.i, !dbg !12854
  %left_own.i.7.i.i = load float, ptr %98, align 4, !dbg !12854, !alias.scope !12838, !noalias !12859, !noundef !12
  %99 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.7.i.i, !dbg !12855
  %right_own.i.7.i.i = load float, ptr %99, align 4, !dbg !12855, !alias.scope !12840, !noalias !12860, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !12869

bb17.i.i.i:                                       ; preds = %bb16.i.preheader.i.i
  %_59.i.i.i = icmp samesign ult i64 %_22.i.i.i, %_66.1.i.i, !dbg !12872
  br i1 %_59.i.i.i, label %bb19.i.i.i, label %panic17.i.i.i, !dbg !12872

panic15.i.i.i:                                    ; preds = %bb19.i.6.i.i, %bb19.i.5.i.i, %bb19.i.4.i.i, %bb19.i.3.i.i, %bb19.i.2.i.i, %bb19.i.1.i.i, %bb19.i.i.i, %bb16.i.preheader.i.i
  %left12.i.lcssa.i.i = phi i64 [ %_14.i.i.i, %bb16.i.preheader.i.i ], [ %left12.i.1.i.i, %bb19.i.i.i ], [ %left12.i.2.i.i, %bb19.i.1.i.i ], [ %left12.i.3.i.i, %bb19.i.2.i.i ], [ %left12.i.4.i.i, %bb19.i.3.i.i ], [ %left12.i.5.i.i, %bb19.i.4.i.i ], [ %left12.i.6.i.i, %bb19.i.5.i.i ], [ %left12.i.7.i.i, %bb19.i.6.i.i ], !dbg !12873
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left12.i.lcssa.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2e01e5f964095ee8e5aa091c7d92b88) #24, !dbg !12853, !noalias !12857
  unreachable, !dbg !12853

panic17.i.i.i:                                    ; preds = %bb17.i.7.i.i, %bb17.i.6.i.i, %bb17.i.5.i.i, %bb17.i.4.i.i, %bb17.i.3.i.i, %bb17.i.2.i.i, %bb17.i.1.i.i, %bb17.i.i.i
  %right14.i.lcssa1836.i.i = phi i64 [ %_22.i.i.i, %bb17.i.i.i ], [ %right14.i.1.i.i, %bb17.i.1.i.i ], [ %right14.i.2.i.i, %bb17.i.2.i.i ], [ %right14.i.3.i.i, %bb17.i.3.i.i ], [ %right14.i.4.i.i, %bb17.i.4.i.i ], [ %right14.i.5.i.i, %bb17.i.5.i.i ], [ %right14.i.6.i.i, %bb17.i.6.i.i ], [ %right14.i.7.i.i, %bb17.i.7.i.i ], !dbg !12874
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right14.i.lcssa1836.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_48dca199019b49040caac979cf1ddc5a) #24, !dbg !12872, !noalias !12857
  unreachable, !dbg !12872

bb19.i.i.i:                                       ; preds = %bb17.i.i.i
  %100 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_14.i.i.i, !dbg !12853
  %left_own16.i.i.i = load float, ptr %100, align 4, !dbg !12853, !alias.scope !12838, !noalias !12859, !noundef !12
  %101 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_22.i.i.i, !dbg !12872
  %right_own18.i.i.i = load float, ptr %101, align 4, !dbg !12872, !alias.scope !12840, !noalias !12860, !noundef !12
  %_43.i.1.i.i = load i32, ptr %45, align 4, !dbg !12875, !alias.scope !12844, !noalias !12845, !noundef !12
  %_42.i.1.i.i = sub i32 %now.i.i.i, %_43.i.1.i.i, !dbg !12876
  %_41.i.1.i.i = and i32 %_42.i.1.i.i, %_58.i.i, !dbg !12878
  %_40.i.1.i.i = zext i32 %_41.i.1.i.i to i64, !dbg !12873
  %_39.i63.1.i.i = shl nuw nsw i64 %_40.i.1.i.i, 3, !dbg !12873
  %left12.i.1.i.i = or disjoint i64 %_39.i63.1.i.i, 1, !dbg !12873
  %_51.i65.1.i.i = load i32, ptr %46, align 4, !dbg !12879, !alias.scope !12849, !noalias !12850, !noundef !12
  %_50.i.1.i.i = sub i32 %now.i.i.i, %_51.i65.1.i.i, !dbg !12880
  %_49.i.1.i.i = and i32 %_50.i.1.i.i, %_58.i.i, !dbg !12882
  %_48.i.1.i.i = zext i32 %_49.i.1.i.i to i64, !dbg !12874
  %_47.i.1.i.i = shl nuw nsw i64 %_48.i.1.i.i, 3, !dbg !12874
  %right14.i.1.i.i = or disjoint i64 %_47.i.1.i.i, 1, !dbg !12874
  %_56.i66.1.i.i = icmp samesign ult i64 %left12.i.1.i.i, %_64.1.i.i, !dbg !12853
  br i1 %_56.i66.1.i.i, label %bb17.i.1.i.i, label %panic15.i.i.i, !dbg !12853

bb17.i.1.i.i:                                     ; preds = %bb19.i.i.i
  %_59.i.1.i.i = icmp samesign ult i64 %right14.i.1.i.i, %_66.1.i.i, !dbg !12872
  br i1 %_59.i.1.i.i, label %bb19.i.1.i.i, label %panic17.i.i.i, !dbg !12872

bb19.i.1.i.i:                                     ; preds = %bb17.i.1.i.i
  %102 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.1.i.i, !dbg !12853
  %left_own16.i.1.i.i = load float, ptr %102, align 4, !dbg !12853, !alias.scope !12838, !noalias !12859, !noundef !12
  %103 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.1.i.i, !dbg !12872
  %right_own18.i.1.i.i = load float, ptr %103, align 4, !dbg !12872, !alias.scope !12840, !noalias !12860, !noundef !12
  %_43.i.2.i.i = load i32, ptr %47, align 4, !dbg !12875, !alias.scope !12844, !noalias !12845, !noundef !12
  %_42.i.2.i.i = sub i32 %now.i.i.i, %_43.i.2.i.i, !dbg !12876
  %_41.i.2.i.i = and i32 %_42.i.2.i.i, %_58.i.i, !dbg !12878
  %_40.i.2.i.i = zext i32 %_41.i.2.i.i to i64, !dbg !12873
  %_39.i63.2.i.i = shl nuw nsw i64 %_40.i.2.i.i, 3, !dbg !12873
  %left12.i.2.i.i = or disjoint i64 %_39.i63.2.i.i, 2, !dbg !12873
  %_51.i65.2.i.i = load i32, ptr %48, align 4, !dbg !12879, !alias.scope !12849, !noalias !12850, !noundef !12
  %_50.i.2.i.i = sub i32 %now.i.i.i, %_51.i65.2.i.i, !dbg !12880
  %_49.i.2.i.i = and i32 %_50.i.2.i.i, %_58.i.i, !dbg !12882
  %_48.i.2.i.i = zext i32 %_49.i.2.i.i to i64, !dbg !12874
  %_47.i.2.i.i = shl nuw nsw i64 %_48.i.2.i.i, 3, !dbg !12874
  %right14.i.2.i.i = or disjoint i64 %_47.i.2.i.i, 2, !dbg !12874
  %_56.i66.2.i.i = icmp samesign ult i64 %left12.i.2.i.i, %_64.1.i.i, !dbg !12853
  br i1 %_56.i66.2.i.i, label %bb17.i.2.i.i, label %panic15.i.i.i, !dbg !12853

bb17.i.2.i.i:                                     ; preds = %bb19.i.1.i.i
  %_59.i.2.i.i = icmp samesign ult i64 %right14.i.2.i.i, %_66.1.i.i, !dbg !12872
  br i1 %_59.i.2.i.i, label %bb19.i.2.i.i, label %panic17.i.i.i, !dbg !12872

bb19.i.2.i.i:                                     ; preds = %bb17.i.2.i.i
  %104 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.2.i.i, !dbg !12853
  %left_own16.i.2.i.i = load float, ptr %104, align 4, !dbg !12853, !alias.scope !12838, !noalias !12859, !noundef !12
  %105 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.2.i.i, !dbg !12872
  %right_own18.i.2.i.i = load float, ptr %105, align 4, !dbg !12872, !alias.scope !12840, !noalias !12860, !noundef !12
  %_43.i.3.i.i = load i32, ptr %49, align 4, !dbg !12875, !alias.scope !12844, !noalias !12845, !noundef !12
  %_42.i.3.i.i = sub i32 %now.i.i.i, %_43.i.3.i.i, !dbg !12876
  %_41.i.3.i.i = and i32 %_42.i.3.i.i, %_58.i.i, !dbg !12878
  %_40.i.3.i.i = zext i32 %_41.i.3.i.i to i64, !dbg !12873
  %_39.i63.3.i.i = shl nuw nsw i64 %_40.i.3.i.i, 3, !dbg !12873
  %left12.i.3.i.i = or disjoint i64 %_39.i63.3.i.i, 3, !dbg !12873
  %_51.i65.3.i.i = load i32, ptr %50, align 4, !dbg !12879, !alias.scope !12849, !noalias !12850, !noundef !12
  %_50.i.3.i.i = sub i32 %now.i.i.i, %_51.i65.3.i.i, !dbg !12880
  %_49.i.3.i.i = and i32 %_50.i.3.i.i, %_58.i.i, !dbg !12882
  %_48.i.3.i.i = zext i32 %_49.i.3.i.i to i64, !dbg !12874
  %_47.i.3.i.i = shl nuw nsw i64 %_48.i.3.i.i, 3, !dbg !12874
  %right14.i.3.i.i = or disjoint i64 %_47.i.3.i.i, 3, !dbg !12874
  %_56.i66.3.i.i = icmp samesign ult i64 %left12.i.3.i.i, %_64.1.i.i, !dbg !12853
  br i1 %_56.i66.3.i.i, label %bb17.i.3.i.i, label %panic15.i.i.i, !dbg !12853

bb17.i.3.i.i:                                     ; preds = %bb19.i.2.i.i
  %_59.i.3.i.i = icmp samesign ult i64 %right14.i.3.i.i, %_66.1.i.i, !dbg !12872
  br i1 %_59.i.3.i.i, label %bb19.i.3.i.i, label %panic17.i.i.i, !dbg !12872

bb19.i.3.i.i:                                     ; preds = %bb17.i.3.i.i
  %106 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.3.i.i, !dbg !12853
  %left_own16.i.3.i.i = load float, ptr %106, align 4, !dbg !12853, !alias.scope !12838, !noalias !12859, !noundef !12
  %107 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.3.i.i, !dbg !12872
  %right_own18.i.3.i.i = load float, ptr %107, align 4, !dbg !12872, !alias.scope !12840, !noalias !12860, !noundef !12
  %_43.i.4.i.i = load i32, ptr %51, align 4, !dbg !12875, !alias.scope !12844, !noalias !12845, !noundef !12
  %_42.i.4.i.i = sub i32 %now.i.i.i, %_43.i.4.i.i, !dbg !12876
  %_41.i.4.i.i = and i32 %_42.i.4.i.i, %_58.i.i, !dbg !12878
  %_40.i.4.i.i = zext i32 %_41.i.4.i.i to i64, !dbg !12873
  %_39.i63.4.i.i = shl nuw nsw i64 %_40.i.4.i.i, 3, !dbg !12873
  %left12.i.4.i.i = or disjoint i64 %_39.i63.4.i.i, 4, !dbg !12873
  %_51.i65.4.i.i = load i32, ptr %52, align 4, !dbg !12879, !alias.scope !12849, !noalias !12850, !noundef !12
  %_50.i.4.i.i = sub i32 %now.i.i.i, %_51.i65.4.i.i, !dbg !12880
  %_49.i.4.i.i = and i32 %_50.i.4.i.i, %_58.i.i, !dbg !12882
  %_48.i.4.i.i = zext i32 %_49.i.4.i.i to i64, !dbg !12874
  %_47.i.4.i.i = shl nuw nsw i64 %_48.i.4.i.i, 3, !dbg !12874
  %right14.i.4.i.i = or disjoint i64 %_47.i.4.i.i, 4, !dbg !12874
  %_56.i66.4.i.i = icmp samesign ult i64 %left12.i.4.i.i, %_64.1.i.i, !dbg !12853
  br i1 %_56.i66.4.i.i, label %bb17.i.4.i.i, label %panic15.i.i.i, !dbg !12853

bb17.i.4.i.i:                                     ; preds = %bb19.i.3.i.i
  %_59.i.4.i.i = icmp samesign ult i64 %right14.i.4.i.i, %_66.1.i.i, !dbg !12872
  br i1 %_59.i.4.i.i, label %bb19.i.4.i.i, label %panic17.i.i.i, !dbg !12872

bb19.i.4.i.i:                                     ; preds = %bb17.i.4.i.i
  %108 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.4.i.i, !dbg !12853
  %left_own16.i.4.i.i = load float, ptr %108, align 4, !dbg !12853, !alias.scope !12838, !noalias !12859, !noundef !12
  %109 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.4.i.i, !dbg !12872
  %right_own18.i.4.i.i = load float, ptr %109, align 4, !dbg !12872, !alias.scope !12840, !noalias !12860, !noundef !12
  %_43.i.5.i.i = load i32, ptr %53, align 4, !dbg !12875, !alias.scope !12844, !noalias !12845, !noundef !12
  %_42.i.5.i.i = sub i32 %now.i.i.i, %_43.i.5.i.i, !dbg !12876
  %_41.i.5.i.i = and i32 %_42.i.5.i.i, %_58.i.i, !dbg !12878
  %_40.i.5.i.i = zext i32 %_41.i.5.i.i to i64, !dbg !12873
  %_39.i63.5.i.i = shl nuw nsw i64 %_40.i.5.i.i, 3, !dbg !12873
  %left12.i.5.i.i = or disjoint i64 %_39.i63.5.i.i, 5, !dbg !12873
  %_51.i65.5.i.i = load i32, ptr %54, align 4, !dbg !12879, !alias.scope !12849, !noalias !12850, !noundef !12
  %_50.i.5.i.i = sub i32 %now.i.i.i, %_51.i65.5.i.i, !dbg !12880
  %_49.i.5.i.i = and i32 %_50.i.5.i.i, %_58.i.i, !dbg !12882
  %_48.i.5.i.i = zext i32 %_49.i.5.i.i to i64, !dbg !12874
  %_47.i.5.i.i = shl nuw nsw i64 %_48.i.5.i.i, 3, !dbg !12874
  %right14.i.5.i.i = or disjoint i64 %_47.i.5.i.i, 5, !dbg !12874
  %_56.i66.5.i.i = icmp samesign ult i64 %left12.i.5.i.i, %_64.1.i.i, !dbg !12853
  br i1 %_56.i66.5.i.i, label %bb17.i.5.i.i, label %panic15.i.i.i, !dbg !12853

bb17.i.5.i.i:                                     ; preds = %bb19.i.4.i.i
  %_59.i.5.i.i = icmp samesign ult i64 %right14.i.5.i.i, %_66.1.i.i, !dbg !12872
  br i1 %_59.i.5.i.i, label %bb19.i.5.i.i, label %panic17.i.i.i, !dbg !12872

bb19.i.5.i.i:                                     ; preds = %bb17.i.5.i.i
  %110 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.5.i.i, !dbg !12853
  %left_own16.i.5.i.i = load float, ptr %110, align 4, !dbg !12853, !alias.scope !12838, !noalias !12859, !noundef !12
  %111 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.5.i.i, !dbg !12872
  %right_own18.i.5.i.i = load float, ptr %111, align 4, !dbg !12872, !alias.scope !12840, !noalias !12860, !noundef !12
  %_43.i.6.i.i = load i32, ptr %55, align 4, !dbg !12875, !alias.scope !12844, !noalias !12845, !noundef !12
  %_42.i.6.i.i = sub i32 %now.i.i.i, %_43.i.6.i.i, !dbg !12876
  %_41.i.6.i.i = and i32 %_42.i.6.i.i, %_58.i.i, !dbg !12878
  %_40.i.6.i.i = zext i32 %_41.i.6.i.i to i64, !dbg !12873
  %_39.i63.6.i.i = shl nuw nsw i64 %_40.i.6.i.i, 3, !dbg !12873
  %left12.i.6.i.i = or disjoint i64 %_39.i63.6.i.i, 6, !dbg !12873
  %_51.i65.6.i.i = load i32, ptr %56, align 4, !dbg !12879, !alias.scope !12849, !noalias !12850, !noundef !12
  %_50.i.6.i.i = sub i32 %now.i.i.i, %_51.i65.6.i.i, !dbg !12880
  %_49.i.6.i.i = and i32 %_50.i.6.i.i, %_58.i.i, !dbg !12882
  %_48.i.6.i.i = zext i32 %_49.i.6.i.i to i64, !dbg !12874
  %_47.i.6.i.i = shl nuw nsw i64 %_48.i.6.i.i, 3, !dbg !12874
  %right14.i.6.i.i = or disjoint i64 %_47.i.6.i.i, 6, !dbg !12874
  %_56.i66.6.i.i = icmp samesign ult i64 %left12.i.6.i.i, %_64.1.i.i, !dbg !12853
  br i1 %_56.i66.6.i.i, label %bb17.i.6.i.i, label %panic15.i.i.i, !dbg !12853

bb17.i.6.i.i:                                     ; preds = %bb19.i.5.i.i
  %_59.i.6.i.i = icmp samesign ult i64 %right14.i.6.i.i, %_66.1.i.i, !dbg !12872
  br i1 %_59.i.6.i.i, label %bb19.i.6.i.i, label %panic17.i.i.i, !dbg !12872

bb19.i.6.i.i:                                     ; preds = %bb17.i.6.i.i
  %112 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.6.i.i, !dbg !12853
  %left_own16.i.6.i.i = load float, ptr %112, align 4, !dbg !12853, !alias.scope !12838, !noalias !12859, !noundef !12
  %113 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.6.i.i, !dbg !12872
  %right_own18.i.6.i.i = load float, ptr %113, align 4, !dbg !12872, !alias.scope !12840, !noalias !12860, !noundef !12
  %_43.i.7.i.i = load i32, ptr %57, align 4, !dbg !12875, !alias.scope !12844, !noalias !12845, !noundef !12
  %_42.i.7.i.i = sub i32 %now.i.i.i, %_43.i.7.i.i, !dbg !12876
  %_41.i.7.i.i = and i32 %_42.i.7.i.i, %_58.i.i, !dbg !12878
  %_40.i.7.i.i = zext i32 %_41.i.7.i.i to i64, !dbg !12873
  %_39.i63.7.i.i = shl nuw nsw i64 %_40.i.7.i.i, 3, !dbg !12873
  %left12.i.7.i.i = or disjoint i64 %_39.i63.7.i.i, 7, !dbg !12873
  %_51.i65.7.i.i = load i32, ptr %58, align 4, !dbg !12879, !alias.scope !12849, !noalias !12850, !noundef !12
  %_50.i.7.i.i = sub i32 %now.i.i.i, %_51.i65.7.i.i, !dbg !12880
  %_49.i.7.i.i = and i32 %_50.i.7.i.i, %_58.i.i, !dbg !12882
  %_48.i.7.i.i = zext i32 %_49.i.7.i.i to i64, !dbg !12874
  %_47.i.7.i.i = shl nuw nsw i64 %_48.i.7.i.i, 3, !dbg !12874
  %right14.i.7.i.i = or disjoint i64 %_47.i.7.i.i, 7, !dbg !12874
  %_56.i66.7.i.i = icmp samesign ult i64 %left12.i.7.i.i, %_64.1.i.i, !dbg !12853
  br i1 %_56.i66.7.i.i, label %bb17.i.7.i.i, label %panic15.i.i.i, !dbg !12853

bb17.i.7.i.i:                                     ; preds = %bb19.i.6.i.i
  %_59.i.7.i.i = icmp samesign ult i64 %right14.i.7.i.i, %_66.1.i.i, !dbg !12872
  br i1 %_59.i.7.i.i, label %bb19.i.7.i.i, label %panic17.i.i.i, !dbg !12872

bb19.i.7.i.i:                                     ; preds = %bb17.i.7.i.i
  %114 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.7.i.i, !dbg !12853
  %left_own16.i.7.i.i = load float, ptr %114, align 4, !dbg !12853, !alias.scope !12838, !noalias !12859, !noundef !12
  %115 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.7.i.i, !dbg !12872
  %right_own18.i.7.i.i = load float, ptr %115, align 4, !dbg !12872, !alias.scope !12840, !noalias !12860, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !12869

panic28.i.i.i:                                    ; preds = %bb33.i60.6.i.i, %bb33.i60.5.i.i, %bb33.i60.4.i.i, %bb33.i60.3.i.i, %bb33.i60.2.i.i, %bb33.i60.1.i.i, %bb33.i60.i.i, %bb25.i.preheader.i.i
  %left25.i.lcssa.i.i = phi i64 [ %_14.i.i.i, %bb25.i.preheader.i.i ], [ %left25.i.1.i.i, %bb33.i60.i.i ], [ %left25.i.2.i.i, %bb33.i60.1.i.i ], [ %left25.i.3.i.i, %bb33.i60.2.i.i ], [ %left25.i.4.i.i, %bb33.i60.3.i.i ], [ %left25.i.5.i.i, %bb33.i60.4.i.i ], [ %left25.i.6.i.i, %bb33.i60.5.i.i ], [ %left25.i.7.i.i, %bb33.i60.6.i.i ], !dbg !12883
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left25.i.lcssa.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4b7b2eb7fa1b19ad0451b09b71313122) #24, !dbg !12852, !noalias !12857
  unreachable, !dbg !12852

bb27.i58.i.i:                                     ; preds = %bb25.i.preheader.i.i
  %116 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_14.i.i.i, !dbg !12852
  %_79.i.i.i = load float, ptr %116, align 4, !dbg !12852, !alias.scope !12838, !noalias !12859, !noundef !12
  %_85.i.i.i = icmp samesign ult i64 %_14.i.i.i, %_66.1.i.i, !dbg !12884
  br i1 %_85.i.i.i, label %bb29.i59.i.i, label %panic30.i.i.i, !dbg !12884

panic30.i.i.i:                                    ; preds = %bb27.i58.7.i.i, %bb27.i58.6.i.i, %bb27.i58.5.i.i, %bb27.i58.4.i.i, %bb27.i58.3.i.i, %bb27.i58.2.i.i, %bb27.i58.1.i.i, %bb27.i58.i.i
  %left25.i.lcssa1832.i.i = phi i64 [ %_14.i.i.i, %bb27.i58.i.i ], [ %left25.i.1.i.i, %bb27.i58.1.i.i ], [ %left25.i.2.i.i, %bb27.i58.2.i.i ], [ %left25.i.3.i.i, %bb27.i58.3.i.i ], [ %left25.i.4.i.i, %bb27.i58.4.i.i ], [ %left25.i.5.i.i, %bb27.i58.5.i.i ], [ %left25.i.6.i.i, %bb27.i58.6.i.i ], [ %left25.i.7.i.i, %bb27.i58.7.i.i ], !dbg !12883
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left25.i.lcssa1832.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60256fc2b51ee5bb69caa4304418caff) #24, !dbg !12884, !noalias !12857
  unreachable, !dbg !12884

bb29.i59.i.i:                                     ; preds = %bb27.i58.i.i
  %117 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_14.i.i.i, !dbg !12884
  %_83.i.i.i = load float, ptr %117, align 4, !dbg !12884, !alias.scope !12840, !noalias !12860, !noundef !12
  %_87.i.i.i = icmp samesign ult i64 %_22.i.i.i, %_66.1.i.i, !dbg !12885
  br i1 %_87.i.i.i, label %bb31.i.i.i, label %panic32.i.i.i, !dbg !12885

panic32.i.i.i:                                    ; preds = %bb29.i59.7.i.i, %bb29.i59.6.i.i, %bb29.i59.5.i.i, %bb29.i59.4.i.i, %bb29.i59.3.i.i, %bb29.i59.2.i.i, %bb29.i59.1.i.i, %bb29.i59.i.i
  %right27.i.lcssa1829.i.i = phi i64 [ %_22.i.i.i, %bb29.i59.i.i ], [ %right27.i.1.i.i, %bb29.i59.1.i.i ], [ %right27.i.2.i.i, %bb29.i59.2.i.i ], [ %right27.i.3.i.i, %bb29.i59.3.i.i ], [ %right27.i.4.i.i, %bb29.i59.4.i.i ], [ %right27.i.5.i.i, %bb29.i59.5.i.i ], [ %right27.i.6.i.i, %bb29.i59.6.i.i ], [ %right27.i.7.i.i, %bb29.i59.7.i.i ], !dbg !12886
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right27.i.lcssa1829.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_03bcac171bcf0d517141d8cd3ac9ded0) #24, !dbg !12885, !noalias !12857
  unreachable, !dbg !12885

bb31.i.i.i:                                       ; preds = %bb29.i59.i.i
  %118 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_22.i.i.i, !dbg !12885
  %_86.i.i.i = load float, ptr %118, align 4, !dbg !12885, !alias.scope !12840, !noalias !12860, !noundef !12
  %_89.i.i.i = icmp samesign ult i64 %_22.i.i.i, %_64.1.i.i, !dbg !12887
  br i1 %_89.i.i.i, label %bb33.i60.i.i, label %panic34.i.i.i, !dbg !12887

panic34.i.i.i:                                    ; preds = %bb31.i.7.i.i, %bb31.i.6.i.i, %bb31.i.5.i.i, %bb31.i.4.i.i, %bb31.i.3.i.i, %bb31.i.2.i.i, %bb31.i.1.i.i, %bb31.i.i.i
  %right27.i.lcssa1830.i.i = phi i64 [ %_22.i.i.i, %bb31.i.i.i ], [ %right27.i.1.i.i, %bb31.i.1.i.i ], [ %right27.i.2.i.i, %bb31.i.2.i.i ], [ %right27.i.3.i.i, %bb31.i.3.i.i ], [ %right27.i.4.i.i, %bb31.i.4.i.i ], [ %right27.i.5.i.i, %bb31.i.5.i.i ], [ %right27.i.6.i.i, %bb31.i.6.i.i ], [ %right27.i.7.i.i, %bb31.i.7.i.i ], !dbg !12886
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right27.i.lcssa1830.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ed84bd2438b7b7e3dfb2147bac09e923) #24, !dbg !12887, !noalias !12857
  unreachable, !dbg !12887

bb33.i60.i.i:                                     ; preds = %bb31.i.i.i
  %119 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_22.i.i.i, !dbg !12887
  %_88.i.i.i = load float, ptr %119, align 4, !dbg !12887, !alias.scope !12838, !noalias !12859, !noundef !12
  %_68.i.1.i.i = load i32, ptr %45, align 4, !dbg !12888, !alias.scope !12844, !noalias !12845, !noundef !12
  %_67.i52.1.i.i = sub i32 %now.i.i.i, %_68.i.1.i.i, !dbg !12889
  %_66.i.1.i.i = and i32 %_67.i52.1.i.i, %_58.i.i, !dbg !12891
  %_65.i.1.i.i = zext i32 %_66.i.1.i.i to i64, !dbg !12883
  %_64.i53.1.i.i = shl nuw nsw i64 %_65.i.1.i.i, 3, !dbg !12883
  %left25.i.1.i.i = or disjoint i64 %_64.i53.1.i.i, 1, !dbg !12883
  %_76.i54.1.i.i = load i32, ptr %46, align 4, !dbg !12892, !alias.scope !12849, !noalias !12850, !noundef !12
  %_75.i.1.i.i = sub i32 %now.i.i.i, %_76.i54.1.i.i, !dbg !12893
  %_74.i55.1.i.i = and i32 %_75.i.1.i.i, %_58.i.i, !dbg !12895
  %_73.i56.1.i.i = zext i32 %_74.i55.1.i.i to i64, !dbg !12886
  %_72.i.1.i.i = shl nuw nsw i64 %_73.i56.1.i.i, 3, !dbg !12886
  %right27.i.1.i.i = or disjoint i64 %_72.i.1.i.i, 1, !dbg !12886
  %_81.i57.1.i.i = icmp samesign ult i64 %left25.i.1.i.i, %_64.1.i.i, !dbg !12852
  br i1 %_81.i57.1.i.i, label %bb27.i58.1.i.i, label %panic28.i.i.i, !dbg !12852

bb27.i58.1.i.i:                                   ; preds = %bb33.i60.i.i
  %120 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.1.i.i, !dbg !12852
  %_79.i.1.i.i = load float, ptr %120, align 4, !dbg !12852, !alias.scope !12838, !noalias !12859, !noundef !12
  %_85.i.1.i.i = icmp samesign ult i64 %left25.i.1.i.i, %_66.1.i.i, !dbg !12884
  br i1 %_85.i.1.i.i, label %bb29.i59.1.i.i, label %panic30.i.i.i, !dbg !12884

bb29.i59.1.i.i:                                   ; preds = %bb27.i58.1.i.i
  %121 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.1.i.i, !dbg !12884
  %_83.i.1.i.i = load float, ptr %121, align 4, !dbg !12884, !alias.scope !12840, !noalias !12860, !noundef !12
  %_87.i.1.i.i = icmp samesign ult i64 %right27.i.1.i.i, %_66.1.i.i, !dbg !12885
  br i1 %_87.i.1.i.i, label %bb31.i.1.i.i, label %panic32.i.i.i, !dbg !12885

bb31.i.1.i.i:                                     ; preds = %bb29.i59.1.i.i
  %122 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.1.i.i, !dbg !12885
  %_86.i.1.i.i = load float, ptr %122, align 4, !dbg !12885, !alias.scope !12840, !noalias !12860, !noundef !12
  %_89.i.1.i.i = icmp samesign ult i64 %right27.i.1.i.i, %_64.1.i.i, !dbg !12887
  br i1 %_89.i.1.i.i, label %bb33.i60.1.i.i, label %panic34.i.i.i, !dbg !12887

bb33.i60.1.i.i:                                   ; preds = %bb31.i.1.i.i
  %123 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.1.i.i, !dbg !12887
  %_88.i.1.i.i = load float, ptr %123, align 4, !dbg !12887, !alias.scope !12838, !noalias !12859, !noundef !12
  %_68.i.2.i.i = load i32, ptr %47, align 4, !dbg !12888, !alias.scope !12844, !noalias !12845, !noundef !12
  %_67.i52.2.i.i = sub i32 %now.i.i.i, %_68.i.2.i.i, !dbg !12889
  %_66.i.2.i.i = and i32 %_67.i52.2.i.i, %_58.i.i, !dbg !12891
  %_65.i.2.i.i = zext i32 %_66.i.2.i.i to i64, !dbg !12883
  %_64.i53.2.i.i = shl nuw nsw i64 %_65.i.2.i.i, 3, !dbg !12883
  %left25.i.2.i.i = or disjoint i64 %_64.i53.2.i.i, 2, !dbg !12883
  %_76.i54.2.i.i = load i32, ptr %48, align 4, !dbg !12892, !alias.scope !12849, !noalias !12850, !noundef !12
  %_75.i.2.i.i = sub i32 %now.i.i.i, %_76.i54.2.i.i, !dbg !12893
  %_74.i55.2.i.i = and i32 %_75.i.2.i.i, %_58.i.i, !dbg !12895
  %_73.i56.2.i.i = zext i32 %_74.i55.2.i.i to i64, !dbg !12886
  %_72.i.2.i.i = shl nuw nsw i64 %_73.i56.2.i.i, 3, !dbg !12886
  %right27.i.2.i.i = or disjoint i64 %_72.i.2.i.i, 2, !dbg !12886
  %_81.i57.2.i.i = icmp samesign ult i64 %left25.i.2.i.i, %_64.1.i.i, !dbg !12852
  br i1 %_81.i57.2.i.i, label %bb27.i58.2.i.i, label %panic28.i.i.i, !dbg !12852

bb27.i58.2.i.i:                                   ; preds = %bb33.i60.1.i.i
  %124 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.2.i.i, !dbg !12852
  %_79.i.2.i.i = load float, ptr %124, align 4, !dbg !12852, !alias.scope !12838, !noalias !12859, !noundef !12
  %_85.i.2.i.i = icmp samesign ult i64 %left25.i.2.i.i, %_66.1.i.i, !dbg !12884
  br i1 %_85.i.2.i.i, label %bb29.i59.2.i.i, label %panic30.i.i.i, !dbg !12884

bb29.i59.2.i.i:                                   ; preds = %bb27.i58.2.i.i
  %125 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.2.i.i, !dbg !12884
  %_83.i.2.i.i = load float, ptr %125, align 4, !dbg !12884, !alias.scope !12840, !noalias !12860, !noundef !12
  %_87.i.2.i.i = icmp samesign ult i64 %right27.i.2.i.i, %_66.1.i.i, !dbg !12885
  br i1 %_87.i.2.i.i, label %bb31.i.2.i.i, label %panic32.i.i.i, !dbg !12885

bb31.i.2.i.i:                                     ; preds = %bb29.i59.2.i.i
  %126 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.2.i.i, !dbg !12885
  %_86.i.2.i.i = load float, ptr %126, align 4, !dbg !12885, !alias.scope !12840, !noalias !12860, !noundef !12
  %_89.i.2.i.i = icmp samesign ult i64 %right27.i.2.i.i, %_64.1.i.i, !dbg !12887
  br i1 %_89.i.2.i.i, label %bb33.i60.2.i.i, label %panic34.i.i.i, !dbg !12887

bb33.i60.2.i.i:                                   ; preds = %bb31.i.2.i.i
  %127 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.2.i.i, !dbg !12887
  %_88.i.2.i.i = load float, ptr %127, align 4, !dbg !12887, !alias.scope !12838, !noalias !12859, !noundef !12
  %_68.i.3.i.i = load i32, ptr %49, align 4, !dbg !12888, !alias.scope !12844, !noalias !12845, !noundef !12
  %_67.i52.3.i.i = sub i32 %now.i.i.i, %_68.i.3.i.i, !dbg !12889
  %_66.i.3.i.i = and i32 %_67.i52.3.i.i, %_58.i.i, !dbg !12891
  %_65.i.3.i.i = zext i32 %_66.i.3.i.i to i64, !dbg !12883
  %_64.i53.3.i.i = shl nuw nsw i64 %_65.i.3.i.i, 3, !dbg !12883
  %left25.i.3.i.i = or disjoint i64 %_64.i53.3.i.i, 3, !dbg !12883
  %_76.i54.3.i.i = load i32, ptr %50, align 4, !dbg !12892, !alias.scope !12849, !noalias !12850, !noundef !12
  %_75.i.3.i.i = sub i32 %now.i.i.i, %_76.i54.3.i.i, !dbg !12893
  %_74.i55.3.i.i = and i32 %_75.i.3.i.i, %_58.i.i, !dbg !12895
  %_73.i56.3.i.i = zext i32 %_74.i55.3.i.i to i64, !dbg !12886
  %_72.i.3.i.i = shl nuw nsw i64 %_73.i56.3.i.i, 3, !dbg !12886
  %right27.i.3.i.i = or disjoint i64 %_72.i.3.i.i, 3, !dbg !12886
  %_81.i57.3.i.i = icmp samesign ult i64 %left25.i.3.i.i, %_64.1.i.i, !dbg !12852
  br i1 %_81.i57.3.i.i, label %bb27.i58.3.i.i, label %panic28.i.i.i, !dbg !12852

bb27.i58.3.i.i:                                   ; preds = %bb33.i60.2.i.i
  %128 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.3.i.i, !dbg !12852
  %_79.i.3.i.i = load float, ptr %128, align 4, !dbg !12852, !alias.scope !12838, !noalias !12859, !noundef !12
  %_85.i.3.i.i = icmp samesign ult i64 %left25.i.3.i.i, %_66.1.i.i, !dbg !12884
  br i1 %_85.i.3.i.i, label %bb29.i59.3.i.i, label %panic30.i.i.i, !dbg !12884

bb29.i59.3.i.i:                                   ; preds = %bb27.i58.3.i.i
  %129 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.3.i.i, !dbg !12884
  %_83.i.3.i.i = load float, ptr %129, align 4, !dbg !12884, !alias.scope !12840, !noalias !12860, !noundef !12
  %_87.i.3.i.i = icmp samesign ult i64 %right27.i.3.i.i, %_66.1.i.i, !dbg !12885
  br i1 %_87.i.3.i.i, label %bb31.i.3.i.i, label %panic32.i.i.i, !dbg !12885

bb31.i.3.i.i:                                     ; preds = %bb29.i59.3.i.i
  %130 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.3.i.i, !dbg !12885
  %_86.i.3.i.i = load float, ptr %130, align 4, !dbg !12885, !alias.scope !12840, !noalias !12860, !noundef !12
  %_89.i.3.i.i = icmp samesign ult i64 %right27.i.3.i.i, %_64.1.i.i, !dbg !12887
  br i1 %_89.i.3.i.i, label %bb33.i60.3.i.i, label %panic34.i.i.i, !dbg !12887

bb33.i60.3.i.i:                                   ; preds = %bb31.i.3.i.i
  %131 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.3.i.i, !dbg !12887
  %_88.i.3.i.i = load float, ptr %131, align 4, !dbg !12887, !alias.scope !12838, !noalias !12859, !noundef !12
  %_68.i.4.i.i = load i32, ptr %51, align 4, !dbg !12888, !alias.scope !12844, !noalias !12845, !noundef !12
  %_67.i52.4.i.i = sub i32 %now.i.i.i, %_68.i.4.i.i, !dbg !12889
  %_66.i.4.i.i = and i32 %_67.i52.4.i.i, %_58.i.i, !dbg !12891
  %_65.i.4.i.i = zext i32 %_66.i.4.i.i to i64, !dbg !12883
  %_64.i53.4.i.i = shl nuw nsw i64 %_65.i.4.i.i, 3, !dbg !12883
  %left25.i.4.i.i = or disjoint i64 %_64.i53.4.i.i, 4, !dbg !12883
  %_76.i54.4.i.i = load i32, ptr %52, align 4, !dbg !12892, !alias.scope !12849, !noalias !12850, !noundef !12
  %_75.i.4.i.i = sub i32 %now.i.i.i, %_76.i54.4.i.i, !dbg !12893
  %_74.i55.4.i.i = and i32 %_75.i.4.i.i, %_58.i.i, !dbg !12895
  %_73.i56.4.i.i = zext i32 %_74.i55.4.i.i to i64, !dbg !12886
  %_72.i.4.i.i = shl nuw nsw i64 %_73.i56.4.i.i, 3, !dbg !12886
  %right27.i.4.i.i = or disjoint i64 %_72.i.4.i.i, 4, !dbg !12886
  %_81.i57.4.i.i = icmp samesign ult i64 %left25.i.4.i.i, %_64.1.i.i, !dbg !12852
  br i1 %_81.i57.4.i.i, label %bb27.i58.4.i.i, label %panic28.i.i.i, !dbg !12852

bb27.i58.4.i.i:                                   ; preds = %bb33.i60.3.i.i
  %132 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.4.i.i, !dbg !12852
  %_79.i.4.i.i = load float, ptr %132, align 4, !dbg !12852, !alias.scope !12838, !noalias !12859, !noundef !12
  %_85.i.4.i.i = icmp samesign ult i64 %left25.i.4.i.i, %_66.1.i.i, !dbg !12884
  br i1 %_85.i.4.i.i, label %bb29.i59.4.i.i, label %panic30.i.i.i, !dbg !12884

bb29.i59.4.i.i:                                   ; preds = %bb27.i58.4.i.i
  %133 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.4.i.i, !dbg !12884
  %_83.i.4.i.i = load float, ptr %133, align 4, !dbg !12884, !alias.scope !12840, !noalias !12860, !noundef !12
  %_87.i.4.i.i = icmp samesign ult i64 %right27.i.4.i.i, %_66.1.i.i, !dbg !12885
  br i1 %_87.i.4.i.i, label %bb31.i.4.i.i, label %panic32.i.i.i, !dbg !12885

bb31.i.4.i.i:                                     ; preds = %bb29.i59.4.i.i
  %134 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.4.i.i, !dbg !12885
  %_86.i.4.i.i = load float, ptr %134, align 4, !dbg !12885, !alias.scope !12840, !noalias !12860, !noundef !12
  %_89.i.4.i.i = icmp samesign ult i64 %right27.i.4.i.i, %_64.1.i.i, !dbg !12887
  br i1 %_89.i.4.i.i, label %bb33.i60.4.i.i, label %panic34.i.i.i, !dbg !12887

bb33.i60.4.i.i:                                   ; preds = %bb31.i.4.i.i
  %135 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.4.i.i, !dbg !12887
  %_88.i.4.i.i = load float, ptr %135, align 4, !dbg !12887, !alias.scope !12838, !noalias !12859, !noundef !12
  %_68.i.5.i.i = load i32, ptr %53, align 4, !dbg !12888, !alias.scope !12844, !noalias !12845, !noundef !12
  %_67.i52.5.i.i = sub i32 %now.i.i.i, %_68.i.5.i.i, !dbg !12889
  %_66.i.5.i.i = and i32 %_67.i52.5.i.i, %_58.i.i, !dbg !12891
  %_65.i.5.i.i = zext i32 %_66.i.5.i.i to i64, !dbg !12883
  %_64.i53.5.i.i = shl nuw nsw i64 %_65.i.5.i.i, 3, !dbg !12883
  %left25.i.5.i.i = or disjoint i64 %_64.i53.5.i.i, 5, !dbg !12883
  %_76.i54.5.i.i = load i32, ptr %54, align 4, !dbg !12892, !alias.scope !12849, !noalias !12850, !noundef !12
  %_75.i.5.i.i = sub i32 %now.i.i.i, %_76.i54.5.i.i, !dbg !12893
  %_74.i55.5.i.i = and i32 %_75.i.5.i.i, %_58.i.i, !dbg !12895
  %_73.i56.5.i.i = zext i32 %_74.i55.5.i.i to i64, !dbg !12886
  %_72.i.5.i.i = shl nuw nsw i64 %_73.i56.5.i.i, 3, !dbg !12886
  %right27.i.5.i.i = or disjoint i64 %_72.i.5.i.i, 5, !dbg !12886
  %_81.i57.5.i.i = icmp samesign ult i64 %left25.i.5.i.i, %_64.1.i.i, !dbg !12852
  br i1 %_81.i57.5.i.i, label %bb27.i58.5.i.i, label %panic28.i.i.i, !dbg !12852

bb27.i58.5.i.i:                                   ; preds = %bb33.i60.4.i.i
  %136 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.5.i.i, !dbg !12852
  %_79.i.5.i.i = load float, ptr %136, align 4, !dbg !12852, !alias.scope !12838, !noalias !12859, !noundef !12
  %_85.i.5.i.i = icmp samesign ult i64 %left25.i.5.i.i, %_66.1.i.i, !dbg !12884
  br i1 %_85.i.5.i.i, label %bb29.i59.5.i.i, label %panic30.i.i.i, !dbg !12884

bb29.i59.5.i.i:                                   ; preds = %bb27.i58.5.i.i
  %137 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.5.i.i, !dbg !12884
  %_83.i.5.i.i = load float, ptr %137, align 4, !dbg !12884, !alias.scope !12840, !noalias !12860, !noundef !12
  %_87.i.5.i.i = icmp samesign ult i64 %right27.i.5.i.i, %_66.1.i.i, !dbg !12885
  br i1 %_87.i.5.i.i, label %bb31.i.5.i.i, label %panic32.i.i.i, !dbg !12885

bb31.i.5.i.i:                                     ; preds = %bb29.i59.5.i.i
  %138 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.5.i.i, !dbg !12885
  %_86.i.5.i.i = load float, ptr %138, align 4, !dbg !12885, !alias.scope !12840, !noalias !12860, !noundef !12
  %_89.i.5.i.i = icmp samesign ult i64 %right27.i.5.i.i, %_64.1.i.i, !dbg !12887
  br i1 %_89.i.5.i.i, label %bb33.i60.5.i.i, label %panic34.i.i.i, !dbg !12887

bb33.i60.5.i.i:                                   ; preds = %bb31.i.5.i.i
  %139 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.5.i.i, !dbg !12887
  %_88.i.5.i.i = load float, ptr %139, align 4, !dbg !12887, !alias.scope !12838, !noalias !12859, !noundef !12
  %_68.i.6.i.i = load i32, ptr %55, align 4, !dbg !12888, !alias.scope !12844, !noalias !12845, !noundef !12
  %_67.i52.6.i.i = sub i32 %now.i.i.i, %_68.i.6.i.i, !dbg !12889
  %_66.i.6.i.i = and i32 %_67.i52.6.i.i, %_58.i.i, !dbg !12891
  %_65.i.6.i.i = zext i32 %_66.i.6.i.i to i64, !dbg !12883
  %_64.i53.6.i.i = shl nuw nsw i64 %_65.i.6.i.i, 3, !dbg !12883
  %left25.i.6.i.i = or disjoint i64 %_64.i53.6.i.i, 6, !dbg !12883
  %_76.i54.6.i.i = load i32, ptr %56, align 4, !dbg !12892, !alias.scope !12849, !noalias !12850, !noundef !12
  %_75.i.6.i.i = sub i32 %now.i.i.i, %_76.i54.6.i.i, !dbg !12893
  %_74.i55.6.i.i = and i32 %_75.i.6.i.i, %_58.i.i, !dbg !12895
  %_73.i56.6.i.i = zext i32 %_74.i55.6.i.i to i64, !dbg !12886
  %_72.i.6.i.i = shl nuw nsw i64 %_73.i56.6.i.i, 3, !dbg !12886
  %right27.i.6.i.i = or disjoint i64 %_72.i.6.i.i, 6, !dbg !12886
  %_81.i57.6.i.i = icmp samesign ult i64 %left25.i.6.i.i, %_64.1.i.i, !dbg !12852
  br i1 %_81.i57.6.i.i, label %bb27.i58.6.i.i, label %panic28.i.i.i, !dbg !12852

bb27.i58.6.i.i:                                   ; preds = %bb33.i60.5.i.i
  %140 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.6.i.i, !dbg !12852
  %_79.i.6.i.i = load float, ptr %140, align 4, !dbg !12852, !alias.scope !12838, !noalias !12859, !noundef !12
  %_85.i.6.i.i = icmp samesign ult i64 %left25.i.6.i.i, %_66.1.i.i, !dbg !12884
  br i1 %_85.i.6.i.i, label %bb29.i59.6.i.i, label %panic30.i.i.i, !dbg !12884

bb29.i59.6.i.i:                                   ; preds = %bb27.i58.6.i.i
  %141 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.6.i.i, !dbg !12884
  %_83.i.6.i.i = load float, ptr %141, align 4, !dbg !12884, !alias.scope !12840, !noalias !12860, !noundef !12
  %_87.i.6.i.i = icmp samesign ult i64 %right27.i.6.i.i, %_66.1.i.i, !dbg !12885
  br i1 %_87.i.6.i.i, label %bb31.i.6.i.i, label %panic32.i.i.i, !dbg !12885

bb31.i.6.i.i:                                     ; preds = %bb29.i59.6.i.i
  %142 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.6.i.i, !dbg !12885
  %_86.i.6.i.i = load float, ptr %142, align 4, !dbg !12885, !alias.scope !12840, !noalias !12860, !noundef !12
  %_89.i.6.i.i = icmp samesign ult i64 %right27.i.6.i.i, %_64.1.i.i, !dbg !12887
  br i1 %_89.i.6.i.i, label %bb33.i60.6.i.i, label %panic34.i.i.i, !dbg !12887

bb33.i60.6.i.i:                                   ; preds = %bb31.i.6.i.i
  %143 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.6.i.i, !dbg !12887
  %_88.i.6.i.i = load float, ptr %143, align 4, !dbg !12887, !alias.scope !12838, !noalias !12859, !noundef !12
  %_68.i.7.i.i = load i32, ptr %57, align 4, !dbg !12888, !alias.scope !12844, !noalias !12845, !noundef !12
  %_67.i52.7.i.i = sub i32 %now.i.i.i, %_68.i.7.i.i, !dbg !12889
  %_66.i.7.i.i = and i32 %_67.i52.7.i.i, %_58.i.i, !dbg !12891
  %_65.i.7.i.i = zext i32 %_66.i.7.i.i to i64, !dbg !12883
  %_64.i53.7.i.i = shl nuw nsw i64 %_65.i.7.i.i, 3, !dbg !12883
  %left25.i.7.i.i = or disjoint i64 %_64.i53.7.i.i, 7, !dbg !12883
  %_76.i54.7.i.i = load i32, ptr %58, align 4, !dbg !12892, !alias.scope !12849, !noalias !12850, !noundef !12
  %_75.i.7.i.i = sub i32 %now.i.i.i, %_76.i54.7.i.i, !dbg !12893
  %_74.i55.7.i.i = and i32 %_75.i.7.i.i, %_58.i.i, !dbg !12895
  %_73.i56.7.i.i = zext i32 %_74.i55.7.i.i to i64, !dbg !12886
  %_72.i.7.i.i = shl nuw nsw i64 %_73.i56.7.i.i, 3, !dbg !12886
  %right27.i.7.i.i = or disjoint i64 %_72.i.7.i.i, 7, !dbg !12886
  %_81.i57.7.i.i = icmp samesign ult i64 %left25.i.7.i.i, %_64.1.i.i, !dbg !12852
  br i1 %_81.i57.7.i.i, label %bb27.i58.7.i.i, label %panic28.i.i.i, !dbg !12852

bb27.i58.7.i.i:                                   ; preds = %bb33.i60.6.i.i
  %144 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.7.i.i, !dbg !12852
  %_79.i.7.i.i = load float, ptr %144, align 4, !dbg !12852, !alias.scope !12838, !noalias !12859, !noundef !12
  %_85.i.7.i.i = icmp samesign ult i64 %left25.i.7.i.i, %_66.1.i.i, !dbg !12884
  br i1 %_85.i.7.i.i, label %bb29.i59.7.i.i, label %panic30.i.i.i, !dbg !12884

bb29.i59.7.i.i:                                   ; preds = %bb27.i58.7.i.i
  %145 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.7.i.i, !dbg !12884
  %_83.i.7.i.i = load float, ptr %145, align 4, !dbg !12884, !alias.scope !12840, !noalias !12860, !noundef !12
  %_87.i.7.i.i = icmp samesign ult i64 %right27.i.7.i.i, %_66.1.i.i, !dbg !12885
  br i1 %_87.i.7.i.i, label %bb31.i.7.i.i, label %panic32.i.i.i, !dbg !12885

bb31.i.7.i.i:                                     ; preds = %bb29.i59.7.i.i
  %_89.i.7.i.i = icmp samesign ult i64 %right27.i.7.i.i, %_64.1.i.i, !dbg !12887
  br i1 %_89.i.7.i.i, label %bb33.i60.7.i.i, label %panic34.i.i.i, !dbg !12887

bb33.i60.7.i.i:                                   ; preds = %bb31.i.7.i.i
  %146 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.7.i.i, !dbg !12885
  %_86.i.7.i.i = load float, ptr %146, align 4, !dbg !12885, !alias.scope !12840, !noalias !12860, !noundef !12
  %147 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.7.i.i, !dbg !12887
  %_88.i.7.i.i = load float, ptr %147, align 4, !dbg !12887, !alias.scope !12838, !noalias !12859, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !12869

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i: ; preds = %bb33.i60.7.i.i, %bb19.i.7.i.i, %bb10.i79.7.i.i
  %taps.i.sroa.103.0.i.i = phi float [ %right_own.i.7.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.7.i.i, %bb19.i.7.i.i ], [ %_88.i.7.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.100.0.i.i = phi float [ %right_own.i.6.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.6.i.i, %bb19.i.7.i.i ], [ %_88.i.6.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.97.0.i.i = phi float [ %right_own.i.5.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.5.i.i, %bb19.i.7.i.i ], [ %_88.i.5.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.94.0.i.i = phi float [ %right_own.i.4.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.4.i.i, %bb19.i.7.i.i ], [ %_88.i.4.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.91.0.i.i = phi float [ %right_own.i.3.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.3.i.i, %bb19.i.7.i.i ], [ %_88.i.3.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.88.0.i.i = phi float [ %right_own.i.2.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.2.i.i, %bb19.i.7.i.i ], [ %_88.i.2.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.85.0.i.i = phi float [ %right_own.i.1.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.1.i.i, %bb19.i.7.i.i ], [ %_88.i.1.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.80.0.i.i = phi float [ %right_own.i.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.i.i, %bb19.i.7.i.i ], [ %_88.i.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.77.0.i.i = phi float [ %right_own.i.7.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.7.i.i, %bb19.i.7.i.i ], [ %_86.i.7.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.74.0.i.i = phi float [ %right_own.i.6.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.6.i.i, %bb19.i.7.i.i ], [ %_86.i.6.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.71.0.i.i = phi float [ %right_own.i.5.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.5.i.i, %bb19.i.7.i.i ], [ %_86.i.5.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.68.0.i.i = phi float [ %right_own.i.4.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.4.i.i, %bb19.i.7.i.i ], [ %_86.i.4.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.65.0.i.i = phi float [ %right_own.i.3.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.3.i.i, %bb19.i.7.i.i ], [ %_86.i.3.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.62.0.i.i = phi float [ %right_own.i.2.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.2.i.i, %bb19.i.7.i.i ], [ %_86.i.2.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.59.0.i.i = phi float [ %right_own.i.1.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.1.i.i, %bb19.i.7.i.i ], [ %_86.i.1.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.54.0.i.i = phi float [ %right_own.i.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.i.i, %bb19.i.7.i.i ], [ %_86.i.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.51.0.i.i = phi float [ %left_own.i.7.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.7.i.i, %bb19.i.7.i.i ], [ %_83.i.7.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.48.0.i.i = phi float [ %left_own.i.6.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.6.i.i, %bb19.i.7.i.i ], [ %_83.i.6.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.45.0.i.i = phi float [ %left_own.i.5.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.5.i.i, %bb19.i.7.i.i ], [ %_83.i.5.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.42.0.i.i = phi float [ %left_own.i.4.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.4.i.i, %bb19.i.7.i.i ], [ %_83.i.4.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.39.0.i.i = phi float [ %left_own.i.3.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.3.i.i, %bb19.i.7.i.i ], [ %_83.i.3.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.36.0.i.i = phi float [ %left_own.i.2.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.2.i.i, %bb19.i.7.i.i ], [ %_83.i.2.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.33.0.i.i = phi float [ %left_own.i.1.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.1.i.i, %bb19.i.7.i.i ], [ %_83.i.1.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.28.0.i.i = phi float [ %left_own.i.i.i, %bb10.i79.7.i.i ], [ %right_own18.i.i.i, %bb19.i.7.i.i ], [ %_83.i.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.25.0.i.i = phi float [ %left_own.i.7.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.7.i.i, %bb19.i.7.i.i ], [ %_79.i.7.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.22.0.i.i = phi float [ %left_own.i.6.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.6.i.i, %bb19.i.7.i.i ], [ %_79.i.6.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.19.0.i.i = phi float [ %left_own.i.5.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.5.i.i, %bb19.i.7.i.i ], [ %_79.i.5.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.16.0.i.i = phi float [ %left_own.i.4.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.4.i.i, %bb19.i.7.i.i ], [ %_79.i.4.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.13.0.i.i = phi float [ %left_own.i.3.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.3.i.i, %bb19.i.7.i.i ], [ %_79.i.3.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.10.0.i.i = phi float [ %left_own.i.2.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.2.i.i, %bb19.i.7.i.i ], [ %_79.i.2.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.7.0.i.i = phi float [ %left_own.i.1.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.1.i.i, %bb19.i.7.i.i ], [ %_79.i.1.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %taps.i.sroa.0.0.i.i = phi float [ %left_own.i.i.i, %bb10.i79.7.i.i ], [ %left_own16.i.i.i, %bb19.i.7.i.i ], [ %_79.i.i.i, %bb33.i60.7.i.i ], !dbg !12842
  %_12.i112.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %59, align 32, !dbg !12896, !alias.scope !12636, !noalias !12903
  %148 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i112.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12910
  %149 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i112.i.sroa.0.0.copyload.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12929
  %_16.i108.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %13, align 32, !dbg !12942, !alias.scope !12636, !noalias !12903
  %_17.i107.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %60, align 32, !dbg !12944, !alias.scope !12636, !noalias !12903
  %150 = fadd <8 x float> %_16.i108.i.sroa.0.0.copyload.i.i, %_17.i107.i.sroa.0.0.copyload.i.i, !dbg !12945
  %_20.i104.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %61, align 32, !dbg !12955, !alias.scope !12636, !noalias !12903
  %151 = bitcast <8 x float> %149 to <8 x i32>, !dbg !12957
  %152 = icmp slt <8 x i32> %151, zeroinitializer, !dbg !12964
  %153 = select <8 x i1> %152, <8 x float> %_20.i104.i.sroa.0.0.copyload.i.i, <8 x float> %150, !dbg !12964
  %154 = bitcast <8 x float> %148 to <8 x i32>, !dbg !12968
  %155 = icmp slt <8 x i32> %154, zeroinitializer, !dbg !12972
  %156 = select <8 x i1> %155, <8 x float> %153, <8 x float> %_16.i108.i.sroa.0.0.copyload.i.i, !dbg !12972
  store <8 x float> %156, ptr %13, align 32, !dbg !12974, !alias.scope !12636, !noalias !12903
  %157 = select <8 x i1> %152, <8 x float> zeroinitializer, <8 x float> %_17.i107.i.sroa.0.0.copyload.i.i, !dbg !12975
  store <8 x float> %157, ptr %60, align 32, !dbg !12980, !alias.scope !12636, !noalias !12903
  %158 = fadd <8 x float> %_12.i112.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !12981
  %159 = select <8 x i1> %155, <8 x float> %158, <8 x float> %_12.i112.i.sroa.0.0.copyload.i.i, !dbg !12991
  store <8 x float> %159, ptr %59, align 32, !dbg !12996, !alias.scope !12636, !noalias !12903
  %_12.i112.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %62, align 32, !dbg !12896, !alias.scope !12636, !noalias !12903
  %160 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i112.i.sroa.0.0.copyload.1.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12910
  %161 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i112.i.sroa.0.0.copyload.1.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12929
  %_16.i108.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i120.i.1.i.i, align 32, !dbg !12942, !alias.scope !12636, !noalias !12903
  %_17.i107.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %63, align 32, !dbg !12944, !alias.scope !12636, !noalias !12903
  %162 = fadd <8 x float> %_16.i108.i.sroa.0.0.copyload.1.i.i, %_17.i107.i.sroa.0.0.copyload.1.i.i, !dbg !12945
  %_20.i104.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %64, align 32, !dbg !12955, !alias.scope !12636, !noalias !12903
  %163 = bitcast <8 x float> %161 to <8 x i32>, !dbg !12957
  %164 = icmp slt <8 x i32> %163, zeroinitializer, !dbg !12964
  %165 = select <8 x i1> %164, <8 x float> %_20.i104.i.sroa.0.0.copyload.1.i.i, <8 x float> %162, !dbg !12964
  %166 = bitcast <8 x float> %160 to <8 x i32>, !dbg !12968
  %167 = icmp slt <8 x i32> %166, zeroinitializer, !dbg !12972
  %168 = select <8 x i1> %167, <8 x float> %165, <8 x float> %_16.i108.i.sroa.0.0.copyload.1.i.i, !dbg !12972
  store <8 x float> %168, ptr %iter1.sroa.0.0.ptr.i120.i.1.i.i, align 32, !dbg !12974, !alias.scope !12636, !noalias !12903
  %169 = select <8 x i1> %164, <8 x float> zeroinitializer, <8 x float> %_17.i107.i.sroa.0.0.copyload.1.i.i, !dbg !12975
  store <8 x float> %169, ptr %63, align 32, !dbg !12980, !alias.scope !12636, !noalias !12903
  %170 = fadd <8 x float> %_12.i112.i.sroa.0.0.copyload.1.i.i, splat (float -1.000000e+00), !dbg !12981
  %171 = select <8 x i1> %167, <8 x float> %170, <8 x float> %_12.i112.i.sroa.0.0.copyload.1.i.i, !dbg !12991
  store <8 x float> %171, ptr %62, align 32, !dbg !12996, !alias.scope !12636, !noalias !12903
  %_12.i112.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %65, align 32, !dbg !12896, !alias.scope !12636, !noalias !12903
  %172 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i112.i.sroa.0.0.copyload.2.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12910
  %173 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i112.i.sroa.0.0.copyload.2.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12929
  %_16.i108.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i120.i.2.i.i, align 32, !dbg !12942, !alias.scope !12636, !noalias !12903
  %_17.i107.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %66, align 32, !dbg !12944, !alias.scope !12636, !noalias !12903
  %174 = fadd <8 x float> %_16.i108.i.sroa.0.0.copyload.2.i.i, %_17.i107.i.sroa.0.0.copyload.2.i.i, !dbg !12945
  %_20.i104.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %67, align 32, !dbg !12955, !alias.scope !12636, !noalias !12903
  %175 = bitcast <8 x float> %173 to <8 x i32>, !dbg !12957
  %176 = icmp slt <8 x i32> %175, zeroinitializer, !dbg !12964
  %177 = select <8 x i1> %176, <8 x float> %_20.i104.i.sroa.0.0.copyload.2.i.i, <8 x float> %174, !dbg !12964
  %178 = bitcast <8 x float> %172 to <8 x i32>, !dbg !12968
  %179 = icmp slt <8 x i32> %178, zeroinitializer, !dbg !12972
  %180 = select <8 x i1> %179, <8 x float> %177, <8 x float> %_16.i108.i.sroa.0.0.copyload.2.i.i, !dbg !12972
  store <8 x float> %180, ptr %iter1.sroa.0.0.ptr.i120.i.2.i.i, align 32, !dbg !12974, !alias.scope !12636, !noalias !12903
  %181 = select <8 x i1> %176, <8 x float> zeroinitializer, <8 x float> %_17.i107.i.sroa.0.0.copyload.2.i.i, !dbg !12975
  store <8 x float> %181, ptr %66, align 32, !dbg !12980, !alias.scope !12636, !noalias !12903
  %182 = fadd <8 x float> %_12.i112.i.sroa.0.0.copyload.2.i.i, splat (float -1.000000e+00), !dbg !12981
  %183 = select <8 x i1> %179, <8 x float> %182, <8 x float> %_12.i112.i.sroa.0.0.copyload.2.i.i, !dbg !12991
  store <8 x float> %183, ptr %65, align 32, !dbg !12996, !alias.scope !12636, !noalias !12903
  %_12.i112.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %68, align 32, !dbg !12896, !alias.scope !12636, !noalias !12903
  %184 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i112.i.sroa.0.0.copyload.3.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12910
  %185 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i112.i.sroa.0.0.copyload.3.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12929
  %_16.i108.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i120.i.3.i.i, align 32, !dbg !12942, !alias.scope !12636, !noalias !12903
  %_17.i107.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %69, align 32, !dbg !12944, !alias.scope !12636, !noalias !12903
  %186 = fadd <8 x float> %_16.i108.i.sroa.0.0.copyload.3.i.i, %_17.i107.i.sroa.0.0.copyload.3.i.i, !dbg !12945
  %_20.i104.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %70, align 32, !dbg !12955, !alias.scope !12636, !noalias !12903
  %187 = bitcast <8 x float> %185 to <8 x i32>, !dbg !12957
  %188 = icmp slt <8 x i32> %187, zeroinitializer, !dbg !12964
  %189 = select <8 x i1> %188, <8 x float> %_20.i104.i.sroa.0.0.copyload.3.i.i, <8 x float> %186, !dbg !12964
  %190 = bitcast <8 x float> %184 to <8 x i32>, !dbg !12968
  %191 = icmp slt <8 x i32> %190, zeroinitializer, !dbg !12972
  %192 = select <8 x i1> %191, <8 x float> %189, <8 x float> %_16.i108.i.sroa.0.0.copyload.3.i.i, !dbg !12972
  store <8 x float> %192, ptr %iter1.sroa.0.0.ptr.i120.i.3.i.i, align 32, !dbg !12974, !alias.scope !12636, !noalias !12903
  %193 = select <8 x i1> %188, <8 x float> zeroinitializer, <8 x float> %_17.i107.i.sroa.0.0.copyload.3.i.i, !dbg !12975
  store <8 x float> %193, ptr %69, align 32, !dbg !12980, !alias.scope !12636, !noalias !12903
  %194 = fadd <8 x float> %_12.i112.i.sroa.0.0.copyload.3.i.i, splat (float -1.000000e+00), !dbg !12981
  %195 = select <8 x i1> %191, <8 x float> %194, <8 x float> %_12.i112.i.sroa.0.0.copyload.3.i.i, !dbg !12991
  store <8 x float> %195, ptr %68, align 32, !dbg !12996, !alias.scope !12636, !noalias !12903
  %_41.i83.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %31, align 32, !dbg !12997, !alias.scope !12636, !noalias !13005
  %196 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_41.i83.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13007
  %197 = bitcast <8 x float> %196 to <8 x i32>, !dbg !13013
  %198 = icmp slt <8 x i32> %197, zeroinitializer, !dbg !13017
  %199 = insertelement <8 x float> poison, float %taps.i.sroa.0.0.i.i, i64 0, !dbg !13019
  %200 = insertelement <8 x float> %199, float %taps.i.sroa.7.0.i.i, i64 1, !dbg !13019
  %201 = insertelement <8 x float> %200, float %taps.i.sroa.10.0.i.i, i64 2, !dbg !13019
  %202 = insertelement <8 x float> %201, float %taps.i.sroa.13.0.i.i, i64 3, !dbg !13019
  %203 = insertelement <8 x float> %202, float %taps.i.sroa.16.0.i.i, i64 4, !dbg !13019
  %204 = insertelement <8 x float> %203, float %taps.i.sroa.19.0.i.i, i64 5, !dbg !13019
  %205 = insertelement <8 x float> %204, float %taps.i.sroa.22.0.i.i, i64 6, !dbg !13019
  %206 = insertelement <8 x float> %205, float %taps.i.sroa.25.0.i.i, i64 7, !dbg !13019
  %207 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %206), !dbg !13023
  %208 = fmul <8 x float> %207, splat (float 5.000000e-01), !dbg !13038
  %209 = insertelement <8 x float> poison, float %taps.i.sroa.28.0.i.i, i64 0, !dbg !13048
  %210 = insertelement <8 x float> %209, float %taps.i.sroa.33.0.i.i, i64 1, !dbg !13048
  %211 = insertelement <8 x float> %210, float %taps.i.sroa.36.0.i.i, i64 2, !dbg !13048
  %212 = insertelement <8 x float> %211, float %taps.i.sroa.39.0.i.i, i64 3, !dbg !13048
  %213 = insertelement <8 x float> %212, float %taps.i.sroa.42.0.i.i, i64 4, !dbg !13048
  %214 = insertelement <8 x float> %213, float %taps.i.sroa.45.0.i.i, i64 5, !dbg !13048
  %215 = insertelement <8 x float> %214, float %taps.i.sroa.48.0.i.i, i64 6, !dbg !13048
  %216 = insertelement <8 x float> %215, float %taps.i.sroa.51.0.i.i, i64 7, !dbg !13048
  %217 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %216), !dbg !13053
  %218 = fmul <8 x float> %217, splat (float 5.000000e-01), !dbg !13059
  %219 = fadd <8 x float> %218, %208, !dbg !13064
  %_37.i87.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %30, align 32, !dbg !13069, !alias.scope !12636, !noalias !13005
  %220 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_37.i87.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13070
  %221 = bitcast <8 x float> %220 to <8 x i32>, !dbg !13076
  %222 = icmp slt <8 x i32> %221, zeroinitializer, !dbg !13080
  %223 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %207, <8 x float> %217), !dbg !13082
  %224 = select <8 x i1> %222, <8 x float> %223, <8 x float> %207, !dbg !13080
  %225 = select <8 x i1> %198, <8 x float> %219, <8 x float> %224, !dbg !13017
  %226 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %225, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !13091
  %227 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %226, <8 x float> splat (float 0x3810000000000000)), !dbg !13096
  %228 = bitcast <8 x float> %227 to <4 x i64>, !dbg !13105
  %229 = and <4 x i64> %228, splat (i64 36028792732385279), !dbg !13106
  %230 = or disjoint <4 x i64> %229, splat (i64 4575657222473777152), !dbg !13124
  %231 = bitcast <4 x i64> %230 to <8 x float>, !dbg !13133
  %232 = fadd <8 x float> %231, splat (float -1.000000e+00), !dbg !13134
  %233 = fmul <8 x float> %232, splat (float 0x3F9B17A960000000), !dbg !13140
  %234 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %233, !dbg !13148
  %235 = fmul <8 x float> %232, %234, !dbg !13140
  %236 = fadd <8 x float> %235, splat (float 0xBFD1E3F400000000), !dbg !13148
  %237 = fmul <8 x float> %232, %236, !dbg !13140
  %238 = fadd <8 x float> %237, splat (float 0x3FDD544F20000000), !dbg !13148
  %239 = fmul <8 x float> %232, %238, !dbg !13140
  %240 = fadd <8 x float> %239, splat (float 0xBFE6FC2A60000000), !dbg !13148
  %241 = fmul <8 x float> %232, %240, !dbg !13140
  %242 = fadd <8 x float> %241, splat (float 0x3FF714B2A0000000), !dbg !13148
  %243 = bitcast <8 x float> %227 to <8 x i32>, !dbg !13153
  %_3.i901.i.i = lshr <8 x i32> %243, splat (i32 23), !dbg !13163
  %244 = or disjoint <8 x i32> %_3.i901.i.i, splat (i32 1258291200), !dbg !13164
  %245 = bitcast <8 x i32> %244 to <8 x float>, !dbg !13170
  %246 = fadd <8 x float> %245, splat (float 0xC160000FE0000000), !dbg !13171
  %247 = fmul <8 x float> %232, %242, !dbg !13177
  %248 = fadd <8 x float> %246, %247, !dbg !13182
  %249 = fmul <8 x float> %248, splat (float 0x4018151820000000), !dbg !13187
  %250 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %249, <8 x float> splat (float 2.400000e+01)), !dbg !13192
  %251 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %250, <8 x float> splat (float -1.600000e+02)), !dbg !13201
  %_55.i69.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %29, align 32, !dbg !13206, !alias.scope !12636, !noalias !12903
  %252 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_55.i69.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13208
  %253 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %251, <8 x float> %156, i8 29), !dbg !13214
  %254 = fsub <8 x float> %156, %192, !dbg !13227
  %255 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %251, <8 x float> %254, i8 29), !dbg !13233
  %256 = bitcast <8 x float> %252 to <8 x i32>, !dbg !13239
  %257 = xor <8 x i32> %256, splat (i32 -1), !dbg !13255
  %258 = bitcast <8 x float> %253 to <8 x i32>, !dbg !13260
  %259 = and <8 x i32> %258, %257, !dbg !13267
  %260 = bitcast <8 x float> %255 to <8 x i32>, !dbg !13269
  %261 = and <8 x i32> %260, %256, !dbg !13274
  %262 = or <8 x i32> %261, %259, !dbg !13276
  %263 = xor <8 x i32> %260, splat (i32 -1), !dbg !13289
  %_67.i57.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %32, align 32, !dbg !13297, !alias.scope !12636, !noalias !12903
  %264 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_67.i57.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13298
  %265 = bitcast <8 x float> %264 to <8 x i32>, !dbg !13304
  %266 = and <8 x i32> %263, %265, !dbg !13308
  %267 = and <8 x i32> %266, %256, !dbg !13308
  %268 = or <8 x i32> %267, %262, !dbg !13313
  %269 = icmp slt <8 x i32> %268, zeroinitializer, !dbg !13319
  %270 = select <8 x i1> %269, <8 x float> splat (float 1.000000e+00), <8 x float> zeroinitializer, !dbg !13319
  %_71.i53.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %33, align 32, !dbg !13324, !alias.scope !12636, !noalias !13005
  %271 = fadd <8 x float> %_67.i57.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !13326
  %272 = icmp slt <8 x i32> %267, zeroinitializer, !dbg !13331
  %273 = select <8 x i1> %272, <8 x float> %271, <8 x float> %_67.i57.i.sroa.0.0.copyload.i.i, !dbg !13331
  %274 = icmp slt <8 x i32> %262, zeroinitializer, !dbg !13336
  %275 = select <8 x i1> %274, <8 x float> %_71.i53.i.sroa.0.0.copyload.i.i, <8 x float> %273, !dbg !13336
  store <8 x float> %275, ptr %32, align 32, !dbg !13341, !alias.scope !12636, !noalias !12903
  store <8 x float> %270, ptr %29, align 32, !dbg !13342, !alias.scope !12636, !noalias !12903
  %_86.i38.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %34, align 32, !dbg !13343, !alias.scope !12636, !noalias !12903
  %_88.i37.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %35, align 32, !dbg !13346, !alias.scope !12636, !noalias !13005
  %_7.i783.i.i = load <8 x float>, ptr %24, align 32, !dbg !13347, !alias.scope !13349, !noalias !13352
  %276 = fadd <8 x float> %168, splat (float -1.000000e+00), !dbg !13356
  %277 = fsub <8 x float> %251, %156, !dbg !13361
  %278 = fmul <8 x float> %276, %277, !dbg !13366
  %279 = fneg <8 x float> %180, !dbg !13371
  %280 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %278, <8 x float> %279), !dbg !13380
  %281 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %280, <8 x float> zeroinitializer), !dbg !13385
  %282 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %270, <8 x float> zeroinitializer, i8 30), !dbg !13390
  %283 = bitcast <8 x float> %282 to <8 x i32>, !dbg !13396
  %284 = icmp slt <8 x i32> %283, zeroinitializer, !dbg !13400
  %285 = select <8 x i1> %284, <8 x float> zeroinitializer, <8 x float> %281, !dbg !13400
  %286 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %285, <8 x float> %_86.i38.i.sroa.0.0.copyload.i.i, i8 30), !dbg !13402
  %287 = bitcast <8 x float> %286 to <8 x i32>, !dbg !13408
  %288 = icmp slt <8 x i32> %287, zeroinitializer, !dbg !13411
  %289 = select <8 x i1> %288, <8 x float> %_7.i783.i.i, <8 x float> %_88.i37.i.sroa.0.0.copyload.i.i, !dbg !13411
  %290 = fsub <8 x float> %285, %_86.i38.i.sroa.0.0.copyload.i.i, !dbg !13413
  %291 = fmul <8 x float> %290, %289, !dbg !13419
  %292 = fadd <8 x float> %_86.i38.i.sroa.0.0.copyload.i.i, %291, !dbg !13427
  %293 = bitcast <8 x float> %292 to <8 x i32>, !dbg !13434
  %294 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %292), !dbg !13441
  %295 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %294, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !13443
  %296 = bitcast <8 x float> %295 to <8 x i32>, !dbg !13455
  %297 = xor <8 x i32> %296, splat (i32 -1), !dbg !13467
  %298 = and <8 x i32> %293, %297, !dbg !13469
  store <8 x i32> %298, ptr %34, align 32, !dbg !13475, !alias.scope !12636, !noalias !12903
  %299 = bitcast <8 x i32> %298 to <8 x float>, !dbg !13477
  %300 = fmul <8 x float> %299, splat (float 0x3FC542A5A0000000), !dbg !13478
  %301 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %300, <8 x float> splat (float -1.260000e+02)), !dbg !13485
  %302 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %301, <8 x float> splat (float 1.270000e+02)), !dbg !13492
  %303 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %302), !dbg !13497
  %304 = fsub <8 x float> %302, %303, !dbg !13507
  %305 = fmul <8 x float> %304, splat (float 0x3F5E974FA0000000), !dbg !13513
  %306 = fadd <8 x float> %305, splat (float 0x3F82778560000000), !dbg !13521
  %307 = fmul <8 x float> %304, %306, !dbg !13513
  %308 = fadd <8 x float> %307, splat (float 0x3FAC91CE60000000), !dbg !13521
  %309 = fmul <8 x float> %304, %308, !dbg !13513
  %310 = fadd <8 x float> %309, splat (float 0x3FCEBDB560000000), !dbg !13521
  %311 = fmul <8 x float> %304, %310, !dbg !13513
  %312 = fadd <8 x float> %311, splat (float 0x3FE62E4BA0000000), !dbg !13521
  %_98.i27.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %36, align 32, !dbg !13526, !alias.scope !12636, !noalias !13005
  %_12.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %71, align 32, !dbg !13528, !alias.scope !12636, !noalias !13531
  %313 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13538
  %314 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.i.sroa.0.0.copyload.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !13544
  %_16.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %data.i.i.i.i, align 32, !dbg !13550, !alias.scope !12636, !noalias !13531
  %_17.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %72, align 32, !dbg !13551, !alias.scope !12636, !noalias !13531
  %315 = fadd <8 x float> %_16.i.i.sroa.0.0.copyload.i.i, %_17.i.i.sroa.0.0.copyload.i.i, !dbg !13552
  %_20.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %73, align 32, !dbg !13557, !alias.scope !12636, !noalias !13531
  %316 = bitcast <8 x float> %314 to <8 x i32>, !dbg !13558
  %317 = icmp slt <8 x i32> %316, zeroinitializer, !dbg !13562
  %318 = select <8 x i1> %317, <8 x float> %_20.i.i.sroa.0.0.copyload.i.i, <8 x float> %315, !dbg !13562
  %319 = bitcast <8 x float> %313 to <8 x i32>, !dbg !13564
  %320 = icmp slt <8 x i32> %319, zeroinitializer, !dbg !13568
  %321 = select <8 x i1> %320, <8 x float> %318, <8 x float> %_16.i.i.sroa.0.0.copyload.i.i, !dbg !13568
  store <8 x float> %321, ptr %data.i.i.i.i, align 32, !dbg !13570, !alias.scope !12636, !noalias !13531
  %322 = select <8 x i1> %317, <8 x float> zeroinitializer, <8 x float> %_17.i.i.sroa.0.0.copyload.i.i, !dbg !13571
  store <8 x float> %322, ptr %72, align 32, !dbg !13576, !alias.scope !12636, !noalias !13531
  %323 = fadd <8 x float> %_12.i.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !13577
  %324 = select <8 x i1> %320, <8 x float> %323, <8 x float> %_12.i.i.sroa.0.0.copyload.i.i, !dbg !13582
  store <8 x float> %324, ptr %71, align 32, !dbg !13587, !alias.scope !12636, !noalias !13531
  %_12.i.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %74, align 32, !dbg !13528, !alias.scope !12636, !noalias !13531
  %325 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.i.sroa.0.0.copyload.1.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13538
  %326 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.i.sroa.0.0.copyload.1.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !13544
  %_16.i.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i.i.1.i.i, align 32, !dbg !13550, !alias.scope !12636, !noalias !13531
  %_17.i.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %75, align 32, !dbg !13551, !alias.scope !12636, !noalias !13531
  %327 = fadd <8 x float> %_16.i.i.sroa.0.0.copyload.1.i.i, %_17.i.i.sroa.0.0.copyload.1.i.i, !dbg !13552
  %_20.i.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %76, align 32, !dbg !13557, !alias.scope !12636, !noalias !13531
  %328 = bitcast <8 x float> %326 to <8 x i32>, !dbg !13558
  %329 = icmp slt <8 x i32> %328, zeroinitializer, !dbg !13562
  %330 = select <8 x i1> %329, <8 x float> %_20.i.i.sroa.0.0.copyload.1.i.i, <8 x float> %327, !dbg !13562
  %331 = bitcast <8 x float> %325 to <8 x i32>, !dbg !13564
  %332 = icmp slt <8 x i32> %331, zeroinitializer, !dbg !13568
  %333 = select <8 x i1> %332, <8 x float> %330, <8 x float> %_16.i.i.sroa.0.0.copyload.1.i.i, !dbg !13568
  store <8 x float> %333, ptr %iter1.sroa.0.0.ptr.i.i.1.i.i, align 32, !dbg !13570, !alias.scope !12636, !noalias !13531
  %334 = select <8 x i1> %329, <8 x float> zeroinitializer, <8 x float> %_17.i.i.sroa.0.0.copyload.1.i.i, !dbg !13571
  store <8 x float> %334, ptr %75, align 32, !dbg !13576, !alias.scope !12636, !noalias !13531
  %335 = fadd <8 x float> %_12.i.i.sroa.0.0.copyload.1.i.i, splat (float -1.000000e+00), !dbg !13577
  %336 = select <8 x i1> %332, <8 x float> %335, <8 x float> %_12.i.i.sroa.0.0.copyload.1.i.i, !dbg !13582
  store <8 x float> %336, ptr %74, align 32, !dbg !13587, !alias.scope !12636, !noalias !13531
  %_12.i.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %77, align 32, !dbg !13528, !alias.scope !12636, !noalias !13531
  %337 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.i.sroa.0.0.copyload.2.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13538
  %338 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.i.sroa.0.0.copyload.2.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !13544
  %_16.i.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i.i.2.i.i, align 32, !dbg !13550, !alias.scope !12636, !noalias !13531
  %_17.i.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %78, align 32, !dbg !13551, !alias.scope !12636, !noalias !13531
  %339 = fadd <8 x float> %_16.i.i.sroa.0.0.copyload.2.i.i, %_17.i.i.sroa.0.0.copyload.2.i.i, !dbg !13552
  %_20.i.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %79, align 32, !dbg !13557, !alias.scope !12636, !noalias !13531
  %340 = bitcast <8 x float> %338 to <8 x i32>, !dbg !13558
  %341 = icmp slt <8 x i32> %340, zeroinitializer, !dbg !13562
  %342 = select <8 x i1> %341, <8 x float> %_20.i.i.sroa.0.0.copyload.2.i.i, <8 x float> %339, !dbg !13562
  %343 = bitcast <8 x float> %337 to <8 x i32>, !dbg !13564
  %344 = icmp slt <8 x i32> %343, zeroinitializer, !dbg !13568
  %345 = select <8 x i1> %344, <8 x float> %342, <8 x float> %_16.i.i.sroa.0.0.copyload.2.i.i, !dbg !13568
  store <8 x float> %345, ptr %iter1.sroa.0.0.ptr.i.i.2.i.i, align 32, !dbg !13570, !alias.scope !12636, !noalias !13531
  %346 = select <8 x i1> %341, <8 x float> zeroinitializer, <8 x float> %_17.i.i.sroa.0.0.copyload.2.i.i, !dbg !13571
  store <8 x float> %346, ptr %78, align 32, !dbg !13576, !alias.scope !12636, !noalias !13531
  %347 = fadd <8 x float> %_12.i.i.sroa.0.0.copyload.2.i.i, splat (float -1.000000e+00), !dbg !13577
  %348 = select <8 x i1> %344, <8 x float> %347, <8 x float> %_12.i.i.sroa.0.0.copyload.2.i.i, !dbg !13582
  store <8 x float> %348, ptr %77, align 32, !dbg !13587, !alias.scope !12636, !noalias !13531
  %_12.i.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %80, align 32, !dbg !13528, !alias.scope !12636, !noalias !13531
  %349 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.i.sroa.0.0.copyload.3.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13538
  %350 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.i.sroa.0.0.copyload.3.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !13544
  %_16.i.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i.i.3.i.i, align 32, !dbg !13550, !alias.scope !12636, !noalias !13531
  %_17.i.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %81, align 32, !dbg !13551, !alias.scope !12636, !noalias !13531
  %351 = fadd <8 x float> %_16.i.i.sroa.0.0.copyload.3.i.i, %_17.i.i.sroa.0.0.copyload.3.i.i, !dbg !13552
  %_20.i.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %82, align 32, !dbg !13557, !alias.scope !12636, !noalias !13531
  %352 = bitcast <8 x float> %350 to <8 x i32>, !dbg !13558
  %353 = icmp slt <8 x i32> %352, zeroinitializer, !dbg !13562
  %354 = select <8 x i1> %353, <8 x float> %_20.i.i.sroa.0.0.copyload.3.i.i, <8 x float> %351, !dbg !13562
  %355 = bitcast <8 x float> %349 to <8 x i32>, !dbg !13564
  %356 = icmp slt <8 x i32> %355, zeroinitializer, !dbg !13568
  %357 = select <8 x i1> %356, <8 x float> %354, <8 x float> %_16.i.i.sroa.0.0.copyload.3.i.i, !dbg !13568
  store <8 x float> %357, ptr %iter1.sroa.0.0.ptr.i.i.3.i.i, align 32, !dbg !13570, !alias.scope !12636, !noalias !13531
  %358 = select <8 x i1> %353, <8 x float> zeroinitializer, <8 x float> %_17.i.i.sroa.0.0.copyload.3.i.i, !dbg !13571
  store <8 x float> %358, ptr %81, align 32, !dbg !13576, !alias.scope !12636, !noalias !13531
  %359 = fadd <8 x float> %_12.i.i.sroa.0.0.copyload.3.i.i, splat (float -1.000000e+00), !dbg !13577
  %360 = select <8 x i1> %356, <8 x float> %359, <8 x float> %_12.i.i.sroa.0.0.copyload.3.i.i, !dbg !13582
  store <8 x float> %360, ptr %80, align 32, !dbg !13587, !alias.scope !12636, !noalias !13531
  %361 = fmul <8 x float> %304, %312, !dbg !13588
  %362 = fadd <8 x float> %361, splat (float 1.000000e+00), !dbg !13593
  %363 = fadd <8 x float> %303, splat (float 0x4160000FE0000000), !dbg !13598
  %364 = bitcast <8 x float> %363 to <8 x i32>, !dbg !13607
  %_3.i902.i.i = shl <8 x i32> %364, splat (i32 23), !dbg !13617
  %365 = bitcast <8 x i32> %_3.i902.i.i to <8 x float>, !dbg !13618
  %366 = fmul <8 x float> %362, %365, !dbg !13620
  %367 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %299, <8 x float> zeroinitializer, i8 0), !dbg !13624
  %368 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_98.i27.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13630
  %369 = bitcast <8 x float> %367 to <8 x i32>, !dbg !13636
  %370 = bitcast <8 x float> %368 to <8 x i32>, !dbg !13636
  %371 = or <8 x i32> %370, %369, !dbg !13640
  %372 = fmul <8 x float> %lanes.i471.sroa.0.0.copyload.i.i, %366, !dbg !13642
  %373 = icmp slt <8 x i32> %371, zeroinitializer, !dbg !13648
  %374 = select <8 x i1> %373, <8 x float> %lanes.i471.sroa.0.0.copyload.i.i, <8 x float> %372, !dbg !13648
  %_41.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %39, align 32, !dbg !13653, !alias.scope !12636, !noalias !13654
  %375 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_41.i.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13656
  %376 = bitcast <8 x float> %375 to <8 x i32>, !dbg !13662
  %377 = icmp slt <8 x i32> %376, zeroinitializer, !dbg !13666
  %378 = insertelement <8 x float> poison, float %taps.i.sroa.54.0.i.i, i64 0, !dbg !13668
  %379 = insertelement <8 x float> %378, float %taps.i.sroa.59.0.i.i, i64 1, !dbg !13668
  %380 = insertelement <8 x float> %379, float %taps.i.sroa.62.0.i.i, i64 2, !dbg !13668
  %381 = insertelement <8 x float> %380, float %taps.i.sroa.65.0.i.i, i64 3, !dbg !13668
  %382 = insertelement <8 x float> %381, float %taps.i.sroa.68.0.i.i, i64 4, !dbg !13668
  %383 = insertelement <8 x float> %382, float %taps.i.sroa.71.0.i.i, i64 5, !dbg !13668
  %384 = insertelement <8 x float> %383, float %taps.i.sroa.74.0.i.i, i64 6, !dbg !13668
  %385 = insertelement <8 x float> %384, float %taps.i.sroa.77.0.i.i, i64 7, !dbg !13668
  %386 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %385), !dbg !13673
  %387 = fmul <8 x float> %386, splat (float 5.000000e-01), !dbg !13679
  %388 = insertelement <8 x float> poison, float %taps.i.sroa.80.0.i.i, i64 0, !dbg !13684
  %389 = insertelement <8 x float> %388, float %taps.i.sroa.85.0.i.i, i64 1, !dbg !13684
  %390 = insertelement <8 x float> %389, float %taps.i.sroa.88.0.i.i, i64 2, !dbg !13684
  %391 = insertelement <8 x float> %390, float %taps.i.sroa.91.0.i.i, i64 3, !dbg !13684
  %392 = insertelement <8 x float> %391, float %taps.i.sroa.94.0.i.i, i64 4, !dbg !13684
  %393 = insertelement <8 x float> %392, float %taps.i.sroa.97.0.i.i, i64 5, !dbg !13684
  %394 = insertelement <8 x float> %393, float %taps.i.sroa.100.0.i.i, i64 6, !dbg !13684
  %395 = insertelement <8 x float> %394, float %taps.i.sroa.103.0.i.i, i64 7, !dbg !13684
  %396 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %395), !dbg !13689
  %397 = fmul <8 x float> %396, splat (float 5.000000e-01), !dbg !13695
  %398 = fadd <8 x float> %397, %387, !dbg !13700
  %_37.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %38, align 32, !dbg !13705, !alias.scope !12636, !noalias !13654
  %399 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_37.i.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13706
  %400 = bitcast <8 x float> %399 to <8 x i32>, !dbg !13712
  %401 = icmp slt <8 x i32> %400, zeroinitializer, !dbg !13716
  %402 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %386, <8 x float> %396), !dbg !13718
  %403 = select <8 x i1> %401, <8 x float> %402, <8 x float> %386, !dbg !13716
  %404 = select <8 x i1> %377, <8 x float> %398, <8 x float> %403, !dbg !13666
  %405 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %404, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !13723
  %406 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %405, <8 x float> splat (float 0x3810000000000000)), !dbg !13728
  %407 = bitcast <8 x float> %406 to <4 x i64>, !dbg !13735
  %408 = and <4 x i64> %407, splat (i64 36028792732385279), !dbg !13736
  %409 = or disjoint <4 x i64> %408, splat (i64 4575657222473777152), !dbg !13741
  %410 = bitcast <4 x i64> %409 to <8 x float>, !dbg !13745
  %411 = fadd <8 x float> %410, splat (float -1.000000e+00), !dbg !13746
  %412 = fmul <8 x float> %411, splat (float 0x3F9B17A960000000), !dbg !13751
  %413 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %412, !dbg !13756
  %414 = fmul <8 x float> %411, %413, !dbg !13751
  %415 = fadd <8 x float> %414, splat (float 0xBFD1E3F400000000), !dbg !13756
  %416 = fmul <8 x float> %411, %415, !dbg !13751
  %417 = fadd <8 x float> %416, splat (float 0x3FDD544F20000000), !dbg !13756
  %418 = fmul <8 x float> %411, %417, !dbg !13751
  %419 = fadd <8 x float> %418, splat (float 0xBFE6FC2A60000000), !dbg !13756
  %420 = fmul <8 x float> %411, %419, !dbg !13751
  %421 = fadd <8 x float> %420, splat (float 0x3FF714B2A0000000), !dbg !13756
  %422 = bitcast <8 x float> %406 to <8 x i32>, !dbg !13761
  %_3.i905.i.i = lshr <8 x i32> %422, splat (i32 23), !dbg !13765
  %423 = or disjoint <8 x i32> %_3.i905.i.i, splat (i32 1258291200), !dbg !13766
  %424 = bitcast <8 x i32> %423 to <8 x float>, !dbg !13770
  %425 = fadd <8 x float> %424, splat (float 0xC160000FE0000000), !dbg !13771
  %426 = fmul <8 x float> %411, %421, !dbg !13775
  %427 = fadd <8 x float> %425, %426, !dbg !13780
  %428 = fmul <8 x float> %427, splat (float 0x4018151820000000), !dbg !13785
  %429 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %428, <8 x float> splat (float 2.400000e+01)), !dbg !13790
  %430 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %429, <8 x float> splat (float -1.600000e+02)), !dbg !13795
  %_55.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %37, align 32, !dbg !13800, !alias.scope !12636, !noalias !13531
  %431 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_55.i.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13801
  %432 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %430, <8 x float> %321, i8 29), !dbg !13807
  %433 = fsub <8 x float> %321, %357, !dbg !13813
  %434 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %430, <8 x float> %433, i8 29), !dbg !13818
  %435 = bitcast <8 x float> %431 to <8 x i32>, !dbg !13824
  %436 = xor <8 x i32> %435, splat (i32 -1), !dbg !13830
  %437 = bitcast <8 x float> %432 to <8 x i32>, !dbg !13832
  %438 = and <8 x i32> %437, %436, !dbg !13836
  %439 = bitcast <8 x float> %434 to <8 x i32>, !dbg !13838
  %440 = and <8 x i32> %439, %435, !dbg !13842
  %441 = or <8 x i32> %440, %438, !dbg !13844
  %442 = xor <8 x i32> %439, splat (i32 -1), !dbg !13849
  %_67.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %40, align 32, !dbg !13856, !alias.scope !12636, !noalias !13531
  %443 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_67.i.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13857
  %444 = bitcast <8 x float> %443 to <8 x i32>, !dbg !13863
  %445 = and <8 x i32> %442, %444, !dbg !13867
  %446 = and <8 x i32> %445, %435, !dbg !13867
  %447 = or <8 x i32> %446, %441, !dbg !13872
  %448 = icmp slt <8 x i32> %447, zeroinitializer, !dbg !13877
  %449 = select <8 x i1> %448, <8 x float> splat (float 1.000000e+00), <8 x float> zeroinitializer, !dbg !13877
  %_71.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %41, align 32, !dbg !13882, !alias.scope !12636, !noalias !13654
  %450 = fadd <8 x float> %_67.i.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !13883
  %451 = icmp slt <8 x i32> %446, zeroinitializer, !dbg !13888
  %452 = select <8 x i1> %451, <8 x float> %450, <8 x float> %_67.i.i.sroa.0.0.copyload.i.i, !dbg !13888
  %453 = icmp slt <8 x i32> %441, zeroinitializer, !dbg !13893
  %454 = select <8 x i1> %453, <8 x float> %_71.i.i.sroa.0.0.copyload.i.i, <8 x float> %452, !dbg !13893
  store <8 x float> %454, ptr %40, align 32, !dbg !13898, !alias.scope !12636, !noalias !13531
  store <8 x float> %449, ptr %37, align 32, !dbg !13899, !alias.scope !12636, !noalias !13531
  %_86.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %42, align 32, !dbg !13900, !alias.scope !12636, !noalias !13531
  %_88.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %43, align 32, !dbg !13901, !alias.scope !12636, !noalias !13654
  %_7.i735.i.i = load <8 x float>, ptr %_38.i.i, align 32, !dbg !13902, !alias.scope !13904, !noalias !13907
  %455 = fadd <8 x float> %333, splat (float -1.000000e+00), !dbg !13911
  %456 = fsub <8 x float> %430, %321, !dbg !13916
  %457 = fmul <8 x float> %455, %456, !dbg !13921
  %458 = fneg <8 x float> %345, !dbg !13926
  %459 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %457, <8 x float> %458), !dbg !13931
  %460 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %459, <8 x float> zeroinitializer), !dbg !13936
  %461 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %449, <8 x float> zeroinitializer, i8 30), !dbg !13941
  %462 = bitcast <8 x float> %461 to <8 x i32>, !dbg !13947
  %463 = icmp slt <8 x i32> %462, zeroinitializer, !dbg !13951
  %464 = select <8 x i1> %463, <8 x float> zeroinitializer, <8 x float> %460, !dbg !13951
  %465 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %464, <8 x float> %_86.i.i.sroa.0.0.copyload.i.i, i8 30), !dbg !13953
  %466 = bitcast <8 x float> %465 to <8 x i32>, !dbg !13959
  %467 = icmp slt <8 x i32> %466, zeroinitializer, !dbg !13962
  %468 = select <8 x i1> %467, <8 x float> %_7.i735.i.i, <8 x float> %_88.i.i.sroa.0.0.copyload.i.i, !dbg !13962
  %469 = fsub <8 x float> %464, %_86.i.i.sroa.0.0.copyload.i.i, !dbg !13964
  %470 = fmul <8 x float> %469, %468, !dbg !13969
  %471 = fadd <8 x float> %_86.i.i.sroa.0.0.copyload.i.i, %470, !dbg !13974
  %472 = bitcast <8 x float> %471 to <8 x i32>, !dbg !13978
  %473 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %471), !dbg !13984
  %474 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %473, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !13986
  %475 = bitcast <8 x float> %474 to <8 x i32>, !dbg !13992
  %476 = xor <8 x i32> %475, splat (i32 -1), !dbg !13998
  %477 = and <8 x i32> %472, %476, !dbg !14000
  store <8 x i32> %477, ptr %42, align 32, !dbg !14004, !alias.scope !12636, !noalias !13531
  %478 = bitcast <8 x i32> %477 to <8 x float>, !dbg !14005
  %479 = fmul <8 x float> %478, splat (float 0x3FC542A5A0000000), !dbg !14006
  %480 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %479, <8 x float> splat (float -1.260000e+02)), !dbg !14012
  %481 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %480, <8 x float> splat (float 1.270000e+02)), !dbg !14018
  %482 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %481), !dbg !14023
  %483 = fsub <8 x float> %481, %482, !dbg !14028
  %484 = fmul <8 x float> %483, splat (float 0x3F5E974FA0000000), !dbg !14033
  %485 = fadd <8 x float> %484, splat (float 0x3F82778560000000), !dbg !14038
  %486 = fmul <8 x float> %483, %485, !dbg !14033
  %487 = fadd <8 x float> %486, splat (float 0x3FAC91CE60000000), !dbg !14038
  %488 = fmul <8 x float> %483, %487, !dbg !14033
  %489 = fadd <8 x float> %488, splat (float 0x3FCEBDB560000000), !dbg !14038
  %490 = fmul <8 x float> %483, %489, !dbg !14033
  %491 = fadd <8 x float> %490, splat (float 0x3FE62E4BA0000000), !dbg !14038
  %492 = fmul <8 x float> %483, %491, !dbg !14043
  %493 = fadd <8 x float> %492, splat (float 1.000000e+00), !dbg !14048
  %494 = fadd <8 x float> %482, splat (float 0x4160000FE0000000), !dbg !14053
  %495 = bitcast <8 x float> %494 to <8 x i32>, !dbg !14058
  %_3.i906.i.i = shl <8 x i32> %495, splat (i32 23), !dbg !14062
  %496 = bitcast <8 x i32> %_3.i906.i.i to <8 x float>, !dbg !14063
  %497 = fmul <8 x float> %493, %496, !dbg !14065
  %498 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %478, <8 x float> zeroinitializer, i8 0), !dbg !14069
  %_98.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %44, align 32, !dbg !14075, !alias.scope !12636, !noalias !13654
  %499 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_98.i.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !14076
  %500 = bitcast <8 x float> %498 to <8 x i32>, !dbg !14082
  %501 = bitcast <8 x float> %499 to <8 x i32>, !dbg !14082
  %502 = or <8 x i32> %501, %500, !dbg !14086
  %503 = fmul <8 x float> %lanes.i465.sroa.0.0.copyload.i.i, %497, !dbg !14088
  %504 = icmp slt <8 x i32> %502, zeroinitializer, !dbg !14093
  %505 = select <8 x i1> %504, <8 x float> %lanes.i465.sroa.0.0.copyload.i.i, <8 x float> %503, !dbg !14093
  store <8 x float> %374, ptr %_97.i.i.i, align 4, !dbg !14098, !alias.scope !14104, !noalias !14108
  store <8 x float> %505, ptr %_115.i.i.i, align 4, !dbg !14112, !alias.scope !14117, !noalias !14121
  %exitcond1961.not.i.i = icmp eq i64 %83, %..i.i, !dbg !14125
  br i1 %exitcond1961.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKb1_EB3_.exit.i, label %bb30.i.i.i, !dbg !12693

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKb1_EB3_.exit.i: ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i
  %_81.i.i.i = add i32 %base.i.i.i, %20, !dbg !14128
  store i32 %_81.i.i.i, ptr %_57.i.i, align 4, !dbg !14130, !alias.scope !12636, !noalias !12690
  br label %bb4.i, !dbg !14131

bb7.i:                                            ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKB1r_EB3_.exit.i, %bb4.i
  %506 = load i32, ptr %14, align 4, !dbg !14132, !alias.scope !12563, !noalias !12577, !noundef !12
  %507 = sub i32 %506, %20, !dbg !14132
  store i32 %507, ptr %14, align 4, !dbg !14132, !alias.scope !12563, !noalias !12577
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14133), !dbg !14136
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14137), !dbg !14136
  call void @llvm.lifetime.start.p0(ptr nonnull %iter.i.i), !dbg !14139, !noalias !14144
  store i64 0, ptr %iter.i.i, align 8, !dbg !14139, !noalias !14144
  %_7.sroa.0.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 8, !dbg !14139
  store i64 2, ptr %_7.sroa.0.sroa.2.0.iter.sroa_idx.i.i, align 8, !dbg !14139, !noalias !14144
  %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 16, !dbg !14139
  store ptr %_37.0, ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, align 8, !dbg !14139, !noalias !14144
  %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 24, !dbg !14139
  store i64 %_37.1, ptr %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i, align 8, !dbg !14139, !noalias !14144
  %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 32, !dbg !14139
  store ptr %_38.0, ptr %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i, align 8, !dbg !14139, !noalias !14144
  %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 40, !dbg !14139
  store i64 %_38.1, ptr %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i, align 8, !dbg !14139, !noalias !14144
  %_7.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 48, !dbg !14139
  store i64 0, ptr %_7.sroa.2.0.iter.sroa_idx.i.i, align 8, !dbg !14139, !noalias !14144
  %_71228.not.i.i = icmp eq i32 %_27, 0
  %508 = getelementptr inbounds nuw i8, ptr %self, i64 2496
  %509 = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %510 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %511 = getelementptr inbounds nuw i8, ptr %self, i64 2576
  %512 = getelementptr inbounds nuw i8, ptr %self, i64 2616
  %513 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %514 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0)
  %515 = bitcast <8 x float> %514 to <8 x i32>
  br label %bb6.i5.i, !dbg !14147

bb6.i5.i:                                         ; preds = %bb1.backedge.i.i, %bb7.i
  %516 = phi i64 [ 24, %bb7.i ], [ 32, %bb1.backedge.i.i ]
  %_5.not.i.i.i.i.i = phi i1 [ false, %bb7.i ], [ true, %bb1.backedge.i.i ]
  %517 = phi i64 [ 0, %bb7.i ], [ 1, %bb1.backedge.i.i ]
  %self3.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<&mut [f32]>", ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, i64 %517, !dbg !14153
  %_14.0.i.i.i.i.i = load ptr, ptr %self3.i.i.i.i.i, align 8, !dbg !14158, !alias.scope !14162, !noalias !14169, !nonnull !12, !align !3533, !noundef !12
  %518 = getelementptr inbounds nuw i8, ptr %self3.i.i.i.i.i, i64 8, !dbg !14158
  %_14.1.i.i.i.i.i = load i64, ptr %518, align 8, !dbg !14158, !alias.scope !14162, !noalias !14169, !noundef !12
  %519 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x8_::f32x8>", ptr %self, i64 %517, !dbg !14171
  %520 = getelementptr inbounds nuw i8, ptr %519, i64 1856, !dbg !14171
  %_17.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %520, align 32, !dbg !14171, !alias.scope !14173, !noalias !14174
  %fst_len.i.i.i = and i64 %_14.1.i.i.i.i.i, -8, !dbg !14175
  %_22.not.i26221.i.i = icmp eq i64 %fst_len.i.i.i, 0, !dbg !14182
  br i1 %_22.not.i26221.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %bb13.i27.i.i, !dbg !14182

bb13.i27.i.i:                                     ; preds = %bb6.i5.i, %bb13.i27.i.i
  %iter.sroa.0.0.i25224.i.i = phi ptr [ %_27.i28.i.i, %bb13.i27.i.i ], [ %_14.0.i.i.i.i.i, %bb6.i5.i ]
  %iter.sroa.5.0.i24223.i.i = phi i64 [ %_28.i29.i.i, %bb13.i27.i.i ], [ %fst_len.i.i.i, %bb6.i5.i ]
  %ok.i16.sroa.0.0222.i.i = phi <8 x i32> [ %525, %bb13.i27.i.i ], [ %515, %bb6.i5.i ]
  %lanes.i.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %iter.sroa.0.0.i25224.i.i, align 4, !dbg !14189, !alias.scope !14195, !noalias !14199
  %_27.i28.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i25224.i.i, i64 32, !dbg !14203
  %_28.i29.i.i = add i64 %iter.sroa.5.0.i24223.i.i, -8, !dbg !14210
  %521 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !14211
  %522 = bitcast <8 x i32> %521 to <8 x float>, !dbg !14218
  %523 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %522, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !14219
  %524 = bitcast <8 x float> %523 to <8 x i32>, !dbg !14225
  %525 = and <8 x i32> %ok.i16.sroa.0.0222.i.i, %524, !dbg !14229
  %_22.not.i26.i.i = icmp eq i64 %_28.i29.i.i, 0, !dbg !14182
  br i1 %_22.not.i26.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %bb13.i27.i.i, !dbg !14182

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i: ; preds = %bb13.i27.i.i, %bb6.i5.i
  %ok.i16.sroa.0.0.lcssa.i.i = phi <8 x i32> [ %515, %bb6.i5.i ], [ %525, %bb13.i27.i.i ], !dbg !14231
  %526 = icmp sgt <8 x i32> %ok.i16.sroa.0.0.lcssa.i.i, splat (i32 -1), !dbg !14232
  %527 = bitcast <8 x i1> %526 to i8, !dbg !14232
  %_0.i98.not.i.i = icmp eq i8 %527, 0, !dbg !14242
  br i1 %_0.i98.not.i.i, label %bb9.i.i, label %bb14.i.i, !dbg !14243

bb9.i.i:                                          ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  %528 = and <8 x i32> %_17.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !14244
  %529 = bitcast <8 x i32> %528 to <8 x float>, !dbg !14251
  %530 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %529, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !14252
  %531 = bitcast <8 x float> %530 to <8 x i32>, !dbg !14258
  %532 = and <8 x i32> %531, %515, !dbg !14262
  %533 = icmp sgt <8 x i32> %532, splat (i32 -1), !dbg !14264
  %534 = bitcast <8 x i1> %533 to i8, !dbg !14264
  %_0.i101.not.i.i = icmp eq i8 %534, 0, !dbg !14269
  br i1 %_0.i101.not.i.i, label %bb1.backedge.i.i, label %bb14.i.i, !dbg !14270

bb1.backedge.i.i:                                 ; preds = %bb17.backedge.i.i, %bb9.i.i
  br i1 %_5.not.i.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9run_blockB2_.exit, label %bb6.i5.i, !dbg !14147

bb14.i.i:                                         ; preds = %bb9.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  %fst_len.i.i.i.i = and i64 %_14.1.i.i.i.i.i, 2305843009213693944, !dbg !14271
  %_40.not71.i.i.i = icmp eq i64 %fst_len.i.i.i.i, 0, !dbg !14278
  br i1 %_40.not71.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i, label %bb23.i.i.i, !dbg !14278

bb23.i.i.i:                                       ; preds = %bb14.i.i, %bb23.i.i.i
  %iter.sroa.0.074.i.i.i = phi ptr [ %_45.i.i.i, %bb23.i.i.i ], [ %_14.0.i.i.i.i.i, %bb14.i.i ]
  %iter.sroa.5.073.i.i.i = phi i64 [ %_46.i.i.i, %bb23.i.i.i ], [ %fst_len.i.i.i.i, %bb14.i.i ]
  %ok.sroa.0.072.i.i.i = phi <8 x i32> [ %539, %bb23.i.i.i ], [ %515, %bb14.i.i ]
  %lanes.i.sroa.0.0.copyload.i.i.i = load <8 x i32>, ptr %iter.sroa.0.074.i.i.i, align 4, !dbg !14285, !alias.scope !14291, !noalias !14297
  %_45.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i.i, i64 32, !dbg !14301
  %_46.i.i.i = add i64 %iter.sroa.5.073.i.i.i, -8, !dbg !14308
  %535 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i.i, splat (i32 2147483647), !dbg !14309
  %536 = bitcast <8 x i32> %535 to <8 x float>, !dbg !14316
  %537 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %536, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !14317
  %538 = bitcast <8 x float> %537 to <8 x i32>, !dbg !14323
  %539 = and <8 x i32> %ok.sroa.0.072.i.i.i, %538, !dbg !14327
  %_40.not.i.i.i = icmp eq i64 %_46.i.i.i, 0, !dbg !14278
  br i1 %_40.not.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i, label %bb23.i.i.i, !dbg !14278

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i: ; preds = %bb23.i.i.i, %bb14.i.i
  %ok.sroa.0.0.lcssa.i.i.i = phi <8 x i32> [ %515, %bb14.i.i ], [ %539, %bb23.i.i.i ], !dbg !14329
  %540 = and <8 x i32> %_17.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !14330
  %541 = bitcast <8 x i32> %540 to <8 x float>, !dbg !14337
  %542 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %541, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !14338
  %543 = bitcast <8 x float> %542 to <8 x i32>, !dbg !14344
  %544 = and <8 x i32> %543, %515, !dbg !14348
  %545 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i.i.i, zeroinitializer, !dbg !14350
  %bc.i.i.i = select <8 x i1> %545, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !14350
  %546 = extractelement <8 x i32> %bc.i.i.i, i64 0, !dbg !14356
  %547 = icmp ne i32 %546, 0, !dbg !14356
  %548 = zext i1 %547 to i32, !dbg !14356
  %549 = extractelement <8 x i32> %bc.i.i.i, i64 1, !dbg !14356
  %550 = icmp eq i32 %549, 0, !dbg !14356
  %551 = select i1 %550, i32 0, i32 2, !dbg !14356
  %552 = extractelement <8 x i32> %bc.i.i.i, i64 2, !dbg !14356
  %553 = icmp eq i32 %552, 0, !dbg !14356
  %554 = select i1 %553, i32 0, i32 4, !dbg !14356
  %555 = extractelement <8 x i32> %bc.i.i.i, i64 3, !dbg !14356
  %556 = icmp eq i32 %555, 0, !dbg !14356
  %557 = select i1 %556, i32 0, i32 8, !dbg !14356
  %558 = extractelement <8 x i32> %bc.i.i.i, i64 4, !dbg !14356
  %559 = icmp eq i32 %558, 0, !dbg !14356
  %560 = select i1 %559, i32 0, i32 16, !dbg !14356
  %561 = extractelement <8 x i32> %bc.i.i.i, i64 5, !dbg !14356
  %562 = icmp eq i32 %561, 0, !dbg !14356
  %563 = select i1 %562, i32 0, i32 32, !dbg !14356
  %564 = extractelement <8 x i32> %bc.i.i.i, i64 6, !dbg !14356
  %565 = icmp eq i32 %564, 0, !dbg !14356
  %566 = select i1 %565, i32 0, i32 64, !dbg !14356
  %567 = extractelement <8 x i32> %bc.i.i.i, i64 7, !dbg !14356
  %568 = icmp eq i32 %567, 0, !dbg !14356
  %569 = select i1 %568, i32 0, i32 128, !dbg !14356
  %570 = icmp slt <8 x i32> %544, zeroinitializer, !dbg !14360
  %bc.i123.i.i = select <8 x i1> %570, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !14360
  %571 = extractelement <8 x i32> %bc.i123.i.i, i64 0, !dbg !14365
  %572 = icmp ne i32 %571, 0, !dbg !14365
  %573 = zext i1 %572 to i32, !dbg !14365
  %574 = extractelement <8 x i32> %bc.i123.i.i, i64 1, !dbg !14365
  %575 = icmp eq i32 %574, 0, !dbg !14365
  %576 = select i1 %575, i32 0, i32 2, !dbg !14365
  %577 = extractelement <8 x i32> %bc.i123.i.i, i64 2, !dbg !14365
  %578 = icmp eq i32 %577, 0, !dbg !14365
  %579 = select i1 %578, i32 0, i32 4, !dbg !14365
  %580 = extractelement <8 x i32> %bc.i123.i.i, i64 3, !dbg !14365
  %581 = icmp eq i32 %580, 0, !dbg !14365
  %582 = select i1 %581, i32 0, i32 8, !dbg !14365
  %583 = extractelement <8 x i32> %bc.i123.i.i, i64 4, !dbg !14365
  %584 = icmp eq i32 %583, 0, !dbg !14365
  %585 = select i1 %584, i32 0, i32 16, !dbg !14365
  %586 = extractelement <8 x i32> %bc.i123.i.i, i64 5, !dbg !14365
  %587 = icmp eq i32 %586, 0, !dbg !14365
  %588 = select i1 %587, i32 0, i32 32, !dbg !14365
  %589 = extractelement <8 x i32> %bc.i123.i.i, i64 6, !dbg !14365
  %590 = icmp eq i32 %589, 0, !dbg !14365
  %591 = select i1 %590, i32 0, i32 64, !dbg !14365
  %592 = extractelement <8 x i32> %bc.i123.i.i, i64 7, !dbg !14365
  %593 = icmp eq i32 %592, 0, !dbg !14365
  %594 = select i1 %593, i32 0, i32 128, !dbg !14365
  %mask.sroa.0.1.1.i125.i.i = or disjoint i32 %551, %548, !dbg !14365
  %mask.sroa.0.1.2.i127.i.i = or disjoint i32 %mask.sroa.0.1.1.i125.i.i, %554, !dbg !14365
  %mask.sroa.0.1.3.i129.i.i = or disjoint i32 %mask.sroa.0.1.2.i127.i.i, %557, !dbg !14365
  %mask.sroa.0.1.4.i131.i.i = or disjoint i32 %mask.sroa.0.1.3.i129.i.i, %560, !dbg !14365
  %mask.sroa.0.1.5.i133.i.i = or disjoint i32 %mask.sroa.0.1.4.i131.i.i, %563, !dbg !14365
  %mask.sroa.0.1.6.i135.i.i = or i32 %mask.sroa.0.1.5.i133.i.i, %566, !dbg !14365
  %mask.sroa.0.1.7.i137.i.i = or i32 %mask.sroa.0.1.6.i135.i.i, %569, !dbg !14365
  %mask.sroa.0.1.1.i.i.i = or i32 %mask.sroa.0.1.7.i137.i.i, %573, !dbg !14356
  %mask.sroa.0.1.2.i.i.i = or i32 %mask.sroa.0.1.1.i.i.i, %576, !dbg !14356
  %mask.sroa.0.1.3.i.i.i = or i32 %mask.sroa.0.1.2.i.i.i, %579, !dbg !14356
  %mask.sroa.0.1.4.i.i.i = or i32 %mask.sroa.0.1.3.i.i.i, %582, !dbg !14356
  %mask.sroa.0.1.5.i.i.i = or i32 %mask.sroa.0.1.4.i.i.i, %585, !dbg !14356
  %mask.sroa.0.1.6.i.i.i = or i32 %mask.sroa.0.1.5.i.i.i, %588, !dbg !14356
  %mask.sroa.0.1.7.i.i.i = or i32 %mask.sroa.0.1.6.i.i.i, %591, !dbg !14356
  %failed.i.i = or i32 %mask.sroa.0.1.7.i.i.i, %594, !dbg !14366
  %invariant.gep.i.i = getelementptr [8 x float], ptr %self, i64 %517, !dbg !14367
  %ring.i.i.i = getelementptr inbounds nuw %Ring, ptr %509, i64 %517
  %595 = getelementptr inbounds nuw i8, ptr %ring.i.i.i, i64 8
  %invariant.gep231.i.i = getelementptr %LaneTiming, ptr %510, i64 %517, !dbg !14370
  %596 = getelementptr inbounds nuw %Ring, ptr %self, i64 %517
  %597 = getelementptr inbounds nuw i8, ptr %596, i64 1184
  %598 = getelementptr inbounds nuw %"kernel::GateCoef<wide::f32x8_::f32x8>", ptr %513, i64 %517
  %_19.i148.i.i = getelementptr inbounds nuw i8, ptr %598, i64 64
  %_25.i.i6.i = getelementptr inbounds nuw i8, ptr %598, i64 32
  %599 = getelementptr inbounds nuw %"kernel::GateCoef<wide::f32x8_::f32x8>", ptr %self, i64 %517
  %600 = getelementptr inbounds nuw i8, ptr %599, i64 832
  %601 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x8_::f32x8>", ptr %13, i64 %517
  %_17.i.i7.i = getelementptr inbounds nuw i8, ptr %601, i64 576
  %_19.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 512
  %_21.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 544
  %_37.i.i8.i = getelementptr inbounds nuw i8, ptr %601, i64 32
  %_39.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 64
  %_41.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 96
  %iter.sroa.0.0.ptr26.1.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 128
  %_37.1.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 160
  %_39.1.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 192
  %_41.1.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 224
  %iter.sroa.0.0.ptr26.2.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 256
  %_37.2.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 288
  %_39.2.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 320
  %_41.2.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 352
  %iter.sroa.0.0.ptr26.3.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 384
  %_37.3.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 416
  %_39.3.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 448
  %_41.3.i.i.i = getelementptr inbounds nuw i8, ptr %601, i64 480
  %602 = getelementptr inbounds nuw i8, ptr %reports, i64 %516
  br label %bb30.i.i, !dbg !14370

bb30.i.i:                                         ; preds = %bb17.backedge.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i
  %iter1.sroa.0.0230.i.i = phi i64 [ 0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i ], [ %603, %bb17.backedge.i.i ]
  %603 = add nuw nsw i64 %iter1.sroa.0.0230.i.i, 1, !dbg !14376
  %604 = trunc nuw nsw i64 %iter1.sroa.0.0230.i.i to i32, !dbg !14382
  %_35.i.i = shl nuw nsw i32 1, %604, !dbg !14382
  %_34.i.i = and i32 %_35.i.i, %failed.i.i, !dbg !14384
  %605 = icmp eq i32 %_34.i.i, 0, !dbg !14384
  br i1 %605, label %bb17.backedge.i.i, label %bb20.preheader.i.i, !dbg !14384

bb20.preheader.i.i:                               ; preds = %bb30.i.i
  br i1 %_71228.not.i.i, label %bb33.i.i, label %bb32.i.i, !dbg !14385

bb33.i.i:                                         ; preds = %bb21.i.i, %bb20.preheader.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14391), !dbg !14394
  %slots.i.i.i = load i64, ptr %508, align 32, !dbg !14397, !alias.scope !14401, !noalias !14174, !noundef !12
  %_193.not.i.i.i = icmp eq i64 %slots.i.i.i, 0, !dbg !14402
  br i1 %_193.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.lr.ph.i.i.i, !dbg !14413

bb7.lr.ph.i.i.i:                                  ; preds = %bb33.i.i
  %_23.1.i.i.i = load i64, ptr %595, align 8, !alias.scope !14401, !noalias !14174, !noundef !12
  br label %bb7.i.i.i, !dbg !14413

bb7.i.i.i:                                        ; preds = %bb3.i.i.i, %bb7.lr.ph.i.i.i
  %iter.sroa.0.04.i.i.i = phi i64 [ 0, %bb7.lr.ph.i.i.i ], [ %606, %bb3.i.i.i ]
  %_10.i.i.i = shl i64 %iter.sroa.0.04.i.i.i, 3, !dbg !14414
  %_9.i139.i.i = add nuw nsw i64 %_10.i.i.i, %iter1.sroa.0.0230.i.i, !dbg !14414
  %_12.i.i.i = icmp ult i64 %_9.i139.i.i, %_23.1.i.i.i, !dbg !14416
  br i1 %_12.i.i.i, label %bb3.i.i.i, label %panic1.i.i.i, !dbg !14416

bb3.i.i.i:                                        ; preds = %bb7.i.i.i
  %_23.0.i.i.i = load ptr, ptr %ring.i.i.i, align 8, !dbg !14416, !alias.scope !14401, !noalias !14174, !nonnull !12, !noundef !12
  %606 = add nuw i64 %iter.sroa.0.04.i.i.i, 1, !dbg !14417
  %607 = getelementptr inbounds nuw float, ptr %_23.0.i.i.i, i64 %_9.i139.i.i, !dbg !14416
  store float 0.000000e+00, ptr %607, align 4, !dbg !14416, !noalias !14423
  %exitcond.not.i.i.i = icmp eq i64 %606, %slots.i.i.i, !dbg !14402
  br i1 %exitcond.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.i.i.i, !dbg !14413

panic1.i.i.i:                                     ; preds = %bb7.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_9.i139.i.i, i64 noundef %_23.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_835aafef72e8508601474e7b1f4172a9) #24, !dbg !14416, !noalias !14423
  unreachable, !dbg !14416

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i: ; preds = %bb3.i.i.i, %bb33.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14424), !dbg !14427
  %gep.i.i = getelementptr [2 x [8 x float]], ptr %invariant.gep.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14428
  %values.sroa.0.0.copyload.i.i.i = load float, ptr %gep.i.i, align 32, !dbg !14428, !alias.scope !14430, !noalias !14174
  %values.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 4, !dbg !14428
  %values.sroa.5.0.copyload.i.i.i = load float, ptr %values.sroa.5.0..sroa_idx.i.i.i, align 4, !dbg !14428, !alias.scope !14430, !noalias !14174
  %values.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 8, !dbg !14428
  %values.sroa.6.0.copyload.i.i.i = load float, ptr %values.sroa.6.0..sroa_idx.i.i.i, align 8, !dbg !14428, !alias.scope !14430, !noalias !14174
  %values.sroa.7.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 12, !dbg !14428
  %values.sroa.7.0.copyload.i.i.i = load float, ptr %values.sroa.7.0..sroa_idx.i.i.i, align 4, !dbg !14428, !alias.scope !14430, !noalias !14174
  %values.sroa.8.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 16, !dbg !14428
  %values.sroa.8.0.copyload.i.i.i = load float, ptr %values.sroa.8.0..sroa_idx.i.i.i, align 16, !dbg !14428, !alias.scope !14430, !noalias !14174
  %values.sroa.9.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 20, !dbg !14428
  %values.sroa.9.0.copyload.i.i.i = load float, ptr %values.sroa.9.0..sroa_idx.i.i.i, align 4, !dbg !14428, !alias.scope !14430, !noalias !14174
  %values.sroa.10.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 24, !dbg !14428
  %values.sroa.10.0.copyload.i.i.i = load float, ptr %values.sroa.10.0..sroa_idx.i.i.i, align 8, !dbg !14428, !alias.scope !14430, !noalias !14174
  %values.sroa.11.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 28, !dbg !14428
  %values.sroa.11.0.copyload.i.i.i = load float, ptr %values.sroa.11.0..sroa_idx.i.i.i, align 4, !dbg !14428, !alias.scope !14430, !noalias !14174
  %gep232.i.i = getelementptr [2 x %LaneTiming], ptr %invariant.gep231.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14431
  store float %values.sroa.11.0.copyload.i.i.i, ptr %gep232.i.i, align 16, !dbg !14431, !alias.scope !14430, !noalias !14174
  %_7.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep232.i.i, i64 4, !dbg !14431
  store float %values.sroa.8.0.copyload.i.i.i, ptr %_7.sroa.4.0..sroa_idx.i.i.i, align 4, !dbg !14431, !alias.scope !14430, !noalias !14174
  %_7.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep232.i.i, i64 8, !dbg !14431
  store float %values.sroa.9.0.copyload.i.i.i, ptr %_7.sroa.5.0..sroa_idx.i.i.i, align 8, !dbg !14431, !alias.scope !14430, !noalias !14174
  %_7.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep232.i.i, i64 12, !dbg !14431
  store float %values.sroa.10.0.copyload.i.i.i, ptr %_7.sroa.6.0..sroa_idx.i.i.i, align 4, !dbg !14431, !alias.scope !14430, !noalias !14174
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14432), !dbg !14435
  %sample_rate.i.i.i = load i32, ptr %511, align 8, !dbg !14436, !alias.scope !14438, !noalias !14174, !noundef !12
  %_31.i.i11.i = fpext float %values.sroa.11.0.copyload.i.i.i to double, !dbg !14439
  %_32.i.i.i = uitofp i32 %sample_rate.i.i.i to double, !dbg !14442
  %_30.i.i12.i = fmul double %_31.i.i11.i, %_32.i.i.i, !dbg !14444
  %_29.i.i.i = fdiv double %_30.i.i12.i, 1.000000e+03, !dbg !14444
  %_28.i145.i.i = fadd double %_29.i.i.i, 5.000000e-01, !dbg !14445
  %608 = tail call double @llvm.floor.f64(double %_28.i145.i.i), !dbg !14446
  %or.cond.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %608, i32 527), !dbg !14449
  %_35.i.i13.i = fcmp ogt double %608, 0x41EFFFFFFFE00000
  %or.cond10.i.i.i = or i1 %or.cond.i.i.i, %_35.i.i13.i, !dbg !14449
  br i1 %or.cond10.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i, label %bb19.i.i14.i, !dbg !14449

bb19.i.i14.i:                                     ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i
  %_45.i146.i.i = fpext float %values.sroa.9.0.copyload.i.i.i to double, !dbg !14450
  %_44.i.i.i = fmul double %_45.i146.i.i, %_32.i.i.i, !dbg !14453
  %_43.i.i.i = fdiv double %_44.i.i.i, 1.000000e+03, !dbg !14453
  %_42.i.i.i = fadd double %_43.i.i.i, 5.000000e-01, !dbg !14454
  %609 = tail call double @llvm.floor.f64(double %_42.i.i.i), !dbg !14455
  %or.cond11.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %609, i32 527), !dbg !14458
  %_48.i.i.i = fcmp ogt double %609, 0x41EFFFFFFFE00000
  %or.cond12.i.i.i = or i1 %or.cond11.i.i.i, %_48.i.i.i, !dbg !14458
  br i1 %or.cond12.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i, label %bb20.3.i.i.i, !dbg !14458

bb20.3.i.i.i:                                     ; preds = %bb19.i.i14.i
  %_12.i147.i.i = load i32, ptr %512, align 8, !dbg !14459, !alias.scope !14438, !noalias !14174, !noundef !12
  %_36.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %608), !dbg !14460
  %_49.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %609), !dbg !14461
  %610 = getelementptr inbounds nuw i32, ptr %597, i64 %iter1.sroa.0.0230.i.i, !dbg !14462
  %611 = tail call i32 @llvm.usub.sat.i32(i32 %_12.i147.i.i, i32 %_36.i.i.i), !dbg !14462
  store i32 %611, ptr %610, align 4, !dbg !14462, !alias.scope !14438, !noalias !14174
  %_20.i.i.i = uitofp i32 %_49.i.i.i to float, !dbg !14463
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i144.i.i), !dbg !14464, !noalias !14466
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i144.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_19.i148.i.i, i64 32, i1 false), !dbg !14464, !noalias !14174
  %612 = getelementptr inbounds nuw float, ptr %words.i.i144.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14467
  store float %_20.i.i.i, ptr %612, align 4, !dbg !14467, !noalias !14468
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_19.i148.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i144.i.i, i64 32, i1 false), !dbg !14471, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i144.i.i), !dbg !14472, !noalias !14466
; call effect_runtime::envelope::attack_release_coefficient
  %_23.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.8.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !14473, !noalias !14474
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i143.i.i), !dbg !14475, !noalias !14466
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i143.i.i, ptr noundef nonnull align 32 dereferenceable(32) %598, i64 32, i1 false), !dbg !14475, !noalias !14174
  %613 = getelementptr inbounds nuw float, ptr %words.i17.i143.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14477
  store float %_23.i.i.i, ptr %613, align 4, !dbg !14477, !noalias !14478
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %598, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i143.i.i, i64 32, i1 false), !dbg !14481, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i143.i.i), !dbg !14482, !noalias !14466
; call effect_runtime::envelope::attack_release_coefficient
  %_26.i.i15.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.10.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !14483, !noalias !14474
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !14484, !noalias !14466
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i18.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_25.i.i6.i, i64 32, i1 false), !dbg !14484, !noalias !14174
  %614 = getelementptr inbounds nuw float, ptr %words.i18.i.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14486
  store float %_26.i.i15.i, ptr %614, align 4, !dbg !14486, !noalias !14487
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_25.i.i6.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i18.i.i.i, i64 32, i1 false), !dbg !14490, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !14491, !noalias !14466
  call void @llvm.lifetime.start.p0(ptr nonnull %_15.i141.i.i), !dbg !14492, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_15.i141.i.i, ptr noundef nonnull align 32 dereferenceable(32) %600, i64 32, i1 false), !dbg !14492, !noalias !14174
  %615 = getelementptr inbounds nuw float, ptr %_15.i141.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14494
  %_0.i.i.i.i = load float, ptr %615, align 4, !dbg !14494, !alias.scope !14496, !noalias !14493, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_15.i141.i.i), !dbg !14499, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i.i.i), !dbg !14500, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_17.i.i7.i, i64 32, i1 false), !dbg !14500, !noalias !14174
  %616 = getelementptr inbounds nuw float, ptr %words.i.i.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14502
  store float 0.000000e+00, ptr %616, align 4, !dbg !14502, !noalias !14503
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_17.i.i7.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i.i.i, i64 32, i1 false), !dbg !14506, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i.i.i), !dbg !14507, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i12.i.i.i), !dbg !14508, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i12.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_19.i.i.i, i64 32, i1 false), !dbg !14508, !noalias !14174
  %617 = getelementptr inbounds nuw float, ptr %words.i12.i.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14510
  store float 1.000000e+00, ptr %617, align 4, !dbg !14510, !noalias !14511
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_19.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i12.i.i.i, i64 32, i1 false), !dbg !14514, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i12.i.i.i), !dbg !14515, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i13.i.i.i), !dbg !14516, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i13.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_21.i.i.i, i64 32, i1 false), !dbg !14516, !noalias !14174
  %618 = getelementptr inbounds nuw float, ptr %words.i13.i.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !14518
  store float %_0.i.i.i.i, ptr %618, align 4, !dbg !14518, !noalias !14519
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_21.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i13.i.i.i, i64 32, i1 false), !dbg !14522, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i13.i.i.i), !dbg !14523, !noalias !14493
  %619 = getelementptr inbounds nuw float, ptr %words.i14.i.i.i, i64 %iter1.sroa.0.0230.i.i
  %620 = getelementptr inbounds nuw float, ptr %words.i15.i.i.i, i64 %iter1.sroa.0.0230.i.i
  %621 = getelementptr inbounds nuw float, ptr %words.i16.i.i.i, i64 %iter1.sroa.0.0230.i.i
  %622 = getelementptr inbounds nuw float, ptr %words.i17.i.i.i, i64 %iter1.sroa.0.0230.i.i
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !14524, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %601, i64 32, i1 false), !dbg !14524, !noalias !14174
  store float %values.sroa.0.0.copyload.i.i.i, ptr %619, align 4, !dbg !14526, !noalias !14527
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %601, ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, i64 32, i1 false), !dbg !14530, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !14531, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !14532, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_37.i.i8.i, i64 32, i1 false), !dbg !14532, !noalias !14174
  store float %values.sroa.0.0.copyload.i.i.i, ptr %620, align 4, !dbg !14534, !noalias !14535
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_37.i.i8.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, i64 32, i1 false), !dbg !14538, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !14539, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !14540, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_39.i.i.i, i64 32, i1 false), !dbg !14540, !noalias !14174
  store float 0.000000e+00, ptr %621, align 4, !dbg !14542, !noalias !14543
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_39.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, i64 32, i1 false), !dbg !14546, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !14547, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !14548, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_41.i.i.i, i64 32, i1 false), !dbg !14548, !noalias !14174
  store float 0.000000e+00, ptr %622, align 4, !dbg !14550, !noalias !14551
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_41.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, i64 32, i1 false), !dbg !14554, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !14555, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !14524, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.1.i.i.i, i64 32, i1 false), !dbg !14524, !noalias !14174
  store float %values.sroa.5.0.copyload.i.i.i, ptr %619, align 4, !dbg !14526, !noalias !14527
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.1.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, i64 32, i1 false), !dbg !14530, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !14531, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !14532, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_37.1.i.i.i, i64 32, i1 false), !dbg !14532, !noalias !14174
  store float %values.sroa.5.0.copyload.i.i.i, ptr %620, align 4, !dbg !14534, !noalias !14535
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_37.1.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, i64 32, i1 false), !dbg !14538, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !14539, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !14540, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_39.1.i.i.i, i64 32, i1 false), !dbg !14540, !noalias !14174
  store float 0.000000e+00, ptr %621, align 4, !dbg !14542, !noalias !14543
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_39.1.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, i64 32, i1 false), !dbg !14546, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !14547, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !14548, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_41.1.i.i.i, i64 32, i1 false), !dbg !14548, !noalias !14174
  store float 0.000000e+00, ptr %622, align 4, !dbg !14550, !noalias !14551
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_41.1.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, i64 32, i1 false), !dbg !14554, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !14555, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !14524, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.2.i.i.i, i64 32, i1 false), !dbg !14524, !noalias !14174
  store float %values.sroa.6.0.copyload.i.i.i, ptr %619, align 4, !dbg !14526, !noalias !14527
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.2.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, i64 32, i1 false), !dbg !14530, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !14531, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !14532, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_37.2.i.i.i, i64 32, i1 false), !dbg !14532, !noalias !14174
  store float %values.sroa.6.0.copyload.i.i.i, ptr %620, align 4, !dbg !14534, !noalias !14535
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_37.2.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, i64 32, i1 false), !dbg !14538, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !14539, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !14540, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_39.2.i.i.i, i64 32, i1 false), !dbg !14540, !noalias !14174
  store float 0.000000e+00, ptr %621, align 4, !dbg !14542, !noalias !14543
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_39.2.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, i64 32, i1 false), !dbg !14546, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !14547, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !14548, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_41.2.i.i.i, i64 32, i1 false), !dbg !14548, !noalias !14174
  store float 0.000000e+00, ptr %622, align 4, !dbg !14550, !noalias !14551
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_41.2.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, i64 32, i1 false), !dbg !14554, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !14555, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !14524, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.3.i.i.i, i64 32, i1 false), !dbg !14524, !noalias !14174
  store float %values.sroa.7.0.copyload.i.i.i, ptr %619, align 4, !dbg !14526, !noalias !14527
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.3.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, i64 32, i1 false), !dbg !14530, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !14531, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !14532, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_37.3.i.i.i, i64 32, i1 false), !dbg !14532, !noalias !14174
  store float %values.sroa.7.0.copyload.i.i.i, ptr %620, align 4, !dbg !14534, !noalias !14535
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_37.3.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, i64 32, i1 false), !dbg !14538, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !14539, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !14540, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_39.3.i.i.i, i64 32, i1 false), !dbg !14540, !noalias !14174
  store float 0.000000e+00, ptr %621, align 4, !dbg !14542, !noalias !14543
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_39.3.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, i64 32, i1 false), !dbg !14546, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !14547, !noalias !14493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !14548, !noalias !14493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_41.3.i.i.i, i64 32, i1 false), !dbg !14548, !noalias !14174
  store float 0.000000e+00, ptr %622, align 4, !dbg !14550, !noalias !14551
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_41.3.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, i64 32, i1 false), !dbg !14554, !noalias !14174
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !14555, !noalias !14493
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i, !dbg !14556

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i: ; preds = %bb20.3.i.i.i, %bb19.i.i14.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i
  %gep234.i.i = getelementptr inbounds nuw %"effect_contract::ProcessReport", ptr %602, i64 %iter1.sroa.0.0230.i.i, !dbg !14557
  %_46.i.i = load i64, ptr %gep234.i.i, align 8, !dbg !14559, !alias.scope !14561, !noalias !14562, !noundef !12
  %623 = tail call i64 @llvm.uadd.sat.i64(i64 %_46.i.i, i64 range(i64 0, 4294967296) %frames), !dbg !14563
  store i64 %623, ptr %gep234.i.i, align 8, !dbg !14566, !alias.scope !14561, !noalias !14562
  br label %bb17.backedge.i.i, !dbg !14367

bb17.backedge.i.i:                                ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i, %bb30.i.i
  %exitcond259.not.i.i = icmp eq i64 %603, 8, !dbg !14567
  br i1 %exitcond259.not.i.i, label %bb1.backedge.i.i, label %bb30.i.i, !dbg !14370

bb32.i.i:                                         ; preds = %bb20.preheader.i.i, %bb21.i.i
  %iter2.sroa.0.0229.i.i = phi i64 [ %624, %bb21.i.i ], [ 0, %bb20.preheader.i.i ]
  %_39.i.i = shl nuw nsw i64 %iter2.sroa.0.0229.i.i, 3, !dbg !14570
  %_38.i9.i = add nuw nsw i64 %_39.i.i, %iter1.sroa.0.0230.i.i, !dbg !14570
  %_41.i.i = icmp ult i64 %_38.i9.i, %_14.1.i.i.i.i.i, !dbg !14572
  br i1 %_41.i.i, label %bb21.i.i, label %panic4.i.i, !dbg !14572

bb21.i.i:                                         ; preds = %bb32.i.i
  %624 = add nuw nsw i64 %iter2.sroa.0.0229.i.i, 1, !dbg !14573
  %625 = getelementptr inbounds nuw float, ptr %_14.0.i.i.i.i.i, i64 %_38.i9.i, !dbg !14572
  store float 0.000000e+00, ptr %625, align 4, !dbg !14572, !noalias !14579
  %exitcond.not.i10.i = icmp eq i64 %624, %frames, !dbg !14580
  br i1 %exitcond.not.i10.i, label %bb33.i.i, label %bb32.i.i, !dbg !14385

panic4.i.i:                                       ; preds = %bb32.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_38.i9.i, i64 noundef %_14.1.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6797264598a169e4722ae66c7bc497b8) #24, !dbg !14572, !noalias !14579
  unreachable, !dbg !14572

bb20.i:                                           ; preds = %bb4.i
  %626 = shl nuw nsw i64 %..i.i, 3, !dbg !14583
  %_52.i = icmp samesign ugt i64 %626, %_37.1, !dbg !14584
  br i1 %_52.i, label %bb24.i, label %bb25.i, !dbg !14584, !prof !180

bb25.i:                                           ; preds = %bb20.i
  %_60.i = icmp samesign ugt i64 %626, %_38.1, !dbg !14592
  br i1 %_60.i, label %bb26.i, label %bb27.i, !dbg !14592, !prof !180

bb24.i:                                           ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %626, i64 noundef range(i64 0, 2305843009213693952) %_37.1, i64 noundef range(i64 0, 2305843009213693952) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e79f4c3c2f015f90ab54e70b61044ab) #24, !dbg !14596, !noalias !12577
  unreachable, !dbg !14596

bb27.i:                                           ; preds = %bb25.i
  %_59.i = getelementptr inbounds nuw float, ptr %_37.0, i64 %626, !dbg !14597
  %_55.i = sub nuw nsw i64 %_37.1, %626, !dbg !14602
  %_63.i = sub nuw nsw i64 %_38.1, %626, !dbg !14603
  %_67.i = getelementptr inbounds nuw float, ptr %_38.0, i64 %626, !dbg !14604
  %_26.i = sub nsw i64 %frames, %..i.i, !dbg !14609
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14610), !dbg !14613
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14614), !dbg !14613
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14616), !dbg !14613
  %data.i.i.i23.i = getelementptr inbounds nuw i8, ptr %self, i64 1888, !dbg !14618
  %_15.i24.i = getelementptr inbounds nuw i8, ptr %self, i64 1152, !dbg !14625
  %data.i.i831.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !14627
  %627 = getelementptr inbounds nuw i8, ptr %self, i64 2572, !dbg !14632
  %_29.i25.i = load i32, ptr %627, align 4, !dbg !14632, !range !1335, !alias.scope !14636, !noalias !14637, !noundef !12
  %_31.i26.i = getelementptr inbounds nuw i8, ptr %self, i64 1184, !dbg !14639
  %_33.i27.i = getelementptr inbounds nuw i8, ptr %self, i64 1248, !dbg !14640
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14641), !dbg !14644
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14645), !dbg !14644
  %628 = icmp eq i32 %_29.i25.i, 1, !dbg !14647
  br i1 %628, label %bb7.i32.i, label %bb1.i.i.preheader.i28.i, !dbg !14647

bb1.i.i.preheader.i28.i:                          ; preds = %bb27.i
  %.val.i.i.i29.i = load i32, ptr %_31.i26.i, align 4, !dbg !14649, !alias.scope !14652, !noalias !14653, !noundef !12
  %.val1.i.i.i30.i = load i32, ptr %_33.i27.i, align 4, !dbg !14649, !alias.scope !14656, !noalias !14657, !noundef !12
  %_0.i.i.not.i.i.i31.i = icmp eq i32 %.val.i.i.i29.i, %.val1.i.i.i30.i, !dbg !14658
  br i1 %_0.i.i.not.i.i.i31.i, label %bb1.i.i.1.i601.i, label %bb7.i32.i, !dbg !14649

bb1.i.i.1.i601.i:                                 ; preds = %bb1.i.i.preheader.i28.i
  %_3.i.i.i.i.i.1.i602.i = getelementptr inbounds nuw i8, ptr %self, i64 1188, !dbg !14663
  %_3.i1.i.i.i.i.1.i603.i = getelementptr inbounds nuw i8, ptr %self, i64 1252, !dbg !14668
  %.val.i.i.1.i604.i = load i32, ptr %_3.i.i.i.i.i.1.i602.i, align 4, !dbg !14649, !alias.scope !14652, !noalias !14653, !noundef !12
  %.val1.i.i.1.i605.i = load i32, ptr %_3.i1.i.i.i.i.1.i603.i, align 4, !dbg !14649, !alias.scope !14656, !noalias !14657, !noundef !12
  %_0.i.i.not.i.i.1.i606.i = icmp eq i32 %.val.i.i.1.i604.i, %.val1.i.i.1.i605.i, !dbg !14658
  br i1 %_0.i.i.not.i.i.1.i606.i, label %bb1.i.i.2.i607.i, label %bb7.i32.i, !dbg !14649

bb1.i.i.2.i607.i:                                 ; preds = %bb1.i.i.1.i601.i
  %_3.i.i.i.i.i.2.i608.i = getelementptr inbounds nuw i8, ptr %self, i64 1192, !dbg !14663
  %_3.i1.i.i.i.i.2.i609.i = getelementptr inbounds nuw i8, ptr %self, i64 1256, !dbg !14668
  %.val.i.i.2.i610.i = load i32, ptr %_3.i.i.i.i.i.2.i608.i, align 4, !dbg !14649, !alias.scope !14652, !noalias !14653, !noundef !12
  %.val1.i.i.2.i611.i = load i32, ptr %_3.i1.i.i.i.i.2.i609.i, align 4, !dbg !14649, !alias.scope !14656, !noalias !14657, !noundef !12
  %_0.i.i.not.i.i.2.i612.i = icmp eq i32 %.val.i.i.2.i610.i, %.val1.i.i.2.i611.i, !dbg !14658
  br i1 %_0.i.i.not.i.i.2.i612.i, label %bb1.i.i.3.i613.i, label %bb7.i32.i, !dbg !14649

bb1.i.i.3.i613.i:                                 ; preds = %bb1.i.i.2.i607.i
  %_3.i.i.i.i.i.3.i614.i = getelementptr inbounds nuw i8, ptr %self, i64 1196, !dbg !14663
  %_3.i1.i.i.i.i.3.i615.i = getelementptr inbounds nuw i8, ptr %self, i64 1260, !dbg !14668
  %.val.i.i.3.i616.i = load i32, ptr %_3.i.i.i.i.i.3.i614.i, align 4, !dbg !14649, !alias.scope !14652, !noalias !14653, !noundef !12
  %.val1.i.i.3.i617.i = load i32, ptr %_3.i1.i.i.i.i.3.i615.i, align 4, !dbg !14649, !alias.scope !14656, !noalias !14657, !noundef !12
  %_0.i.i.not.i.i.3.i618.i = icmp eq i32 %.val.i.i.3.i616.i, %.val1.i.i.3.i617.i, !dbg !14658
  br i1 %_0.i.i.not.i.i.3.i618.i, label %bb1.i.i.4.i619.i, label %bb7.i32.i, !dbg !14649

bb1.i.i.4.i619.i:                                 ; preds = %bb1.i.i.3.i613.i
  %_3.i.i.i.i.i.4.i620.i = getelementptr inbounds nuw i8, ptr %self, i64 1200, !dbg !14663
  %_3.i1.i.i.i.i.4.i621.i = getelementptr inbounds nuw i8, ptr %self, i64 1264, !dbg !14668
  %.val.i.i.4.i622.i = load i32, ptr %_3.i.i.i.i.i.4.i620.i, align 4, !dbg !14649, !alias.scope !14652, !noalias !14653, !noundef !12
  %.val1.i.i.4.i623.i = load i32, ptr %_3.i1.i.i.i.i.4.i621.i, align 4, !dbg !14649, !alias.scope !14656, !noalias !14657, !noundef !12
  %_0.i.i.not.i.i.4.i624.i = icmp eq i32 %.val.i.i.4.i622.i, %.val1.i.i.4.i623.i, !dbg !14658
  br i1 %_0.i.i.not.i.i.4.i624.i, label %bb1.i.i.5.i625.i, label %bb7.i32.i, !dbg !14649

bb1.i.i.5.i625.i:                                 ; preds = %bb1.i.i.4.i619.i
  %_3.i.i.i.i.i.5.i626.i = getelementptr inbounds nuw i8, ptr %self, i64 1204, !dbg !14663
  %_3.i1.i.i.i.i.5.i627.i = getelementptr inbounds nuw i8, ptr %self, i64 1268, !dbg !14668
  %.val.i.i.5.i628.i = load i32, ptr %_3.i.i.i.i.i.5.i626.i, align 4, !dbg !14649, !alias.scope !14652, !noalias !14653, !noundef !12
  %.val1.i.i.5.i629.i = load i32, ptr %_3.i1.i.i.i.i.5.i627.i, align 4, !dbg !14649, !alias.scope !14656, !noalias !14657, !noundef !12
  %_0.i.i.not.i.i.5.i630.i = icmp eq i32 %.val.i.i.5.i628.i, %.val1.i.i.5.i629.i, !dbg !14658
  br i1 %_0.i.i.not.i.i.5.i630.i, label %bb1.i.i.6.i631.i, label %bb7.i32.i, !dbg !14649

bb1.i.i.6.i631.i:                                 ; preds = %bb1.i.i.5.i625.i
  %_3.i.i.i.i.i.6.i632.i = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !14663
  %_3.i1.i.i.i.i.6.i633.i = getelementptr inbounds nuw i8, ptr %self, i64 1272, !dbg !14668
  %.val.i.i.6.i634.i = load i32, ptr %_3.i.i.i.i.i.6.i632.i, align 4, !dbg !14649, !alias.scope !14652, !noalias !14653, !noundef !12
  %.val1.i.i.6.i635.i = load i32, ptr %_3.i1.i.i.i.i.6.i633.i, align 4, !dbg !14649, !alias.scope !14656, !noalias !14657, !noundef !12
  %_0.i.i.not.i.i.6.i636.i = icmp eq i32 %.val.i.i.6.i634.i, %.val1.i.i.6.i635.i, !dbg !14658
  br i1 %_0.i.i.not.i.i.6.i636.i, label %bb1.i.i.7.i637.i, label %bb7.i32.i, !dbg !14649

bb1.i.i.7.i637.i:                                 ; preds = %bb1.i.i.6.i631.i
  %_3.i.i.i.i.i.7.i638.i = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !14663
  %_3.i1.i.i.i.i.7.i639.i = getelementptr inbounds nuw i8, ptr %self, i64 1276, !dbg !14668
  %.val.i.i.7.i640.i = load i32, ptr %_3.i.i.i.i.i.7.i638.i, align 4, !dbg !14649, !alias.scope !14652, !noalias !14653, !noundef !12
  %.val1.i.i.7.i641.i = load i32, ptr %_3.i1.i.i.i.i.7.i639.i, align 4, !dbg !14649, !alias.scope !14656, !noalias !14657, !noundef !12
  %_0.i.i.not.i.i.7.i642.i = icmp eq i32 %.val.i.i.7.i640.i, %.val1.i.i.7.i641.i, !dbg !14658
  %spec.select.i643.i = select i1 %_0.i.i.not.i.i.7.i642.i, i8 1, i8 2, !dbg !14649
  br label %bb7.i32.i, !dbg !14649

bb7.i32.i:                                        ; preds = %bb1.i.i.7.i637.i, %bb1.i.i.6.i631.i, %bb1.i.i.5.i625.i, %bb1.i.i.4.i619.i, %bb1.i.i.3.i613.i, %bb1.i.i.2.i607.i, %bb1.i.i.1.i601.i, %bb1.i.i.preheader.i28.i, %bb27.i
  %_0.sroa.0.0.i836.i.i = phi i8 [ 0, %bb27.i ], [ 2, %bb1.i.i.preheader.i28.i ], [ 2, %bb1.i.i.4.i619.i ], [ 2, %bb1.i.i.6.i631.i ], [ 2, %bb1.i.i.1.i601.i ], [ %spec.select.i643.i, %bb1.i.i.7.i637.i ], [ 2, %bb1.i.i.2.i607.i ], [ 2, %bb1.i.i.5.i625.i ], [ 2, %bb1.i.i.3.i613.i ], !dbg !14671
  %629 = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !14672
  %_38.i33.i = getelementptr inbounds nuw i8, ptr %self, i64 960, !dbg !14674
  %_64.0.i34.i = load ptr, ptr %_15.i24.i, align 8, !dbg !14675, !alias.scope !14636, !noalias !14637, !nonnull !12, !noundef !12
  %630 = getelementptr inbounds nuw i8, ptr %self, i64 1160, !dbg !14675
  %_64.1.i35.i = load i64, ptr %630, align 8, !dbg !14675, !alias.scope !14636, !noalias !14637, !noundef !12
  %_66.0.i36.i = load ptr, ptr %data.i.i831.i.i, align 8, !dbg !14676, !alias.scope !14636, !noalias !14637, !nonnull !12, !noundef !12
  %631 = getelementptr inbounds nuw i8, ptr %self, i64 1224, !dbg !14676
  %_66.1.i37.i = load i64, ptr %631, align 8, !dbg !14676, !alias.scope !14636, !noalias !14637, !noundef !12
  %_57.i38.i = getelementptr inbounds nuw i8, ptr %self, i64 2608, !dbg !14677
  %632 = getelementptr inbounds nuw i8, ptr %self, i64 2612, !dbg !14678
  %_58.i39.i = load i32, ptr %632, align 4, !dbg !14678, !alias.scope !14636, !noalias !14637, !noundef !12
  %633 = getelementptr inbounds nuw i8, ptr %self, i64 2616, !dbg !14679
  %_59.i40.i = load i32, ptr %633, align 8, !dbg !14679, !alias.scope !14636, !noalias !14637, !noundef !12
  %base.i.i45.i = load i32, ptr %_57.i38.i, align 4, !dbg !14680, !alias.scope !14636, !noalias !14690, !noundef !12
  %634 = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %635 = getelementptr inbounds nuw i8, ptr %self, i64 1536
  %636 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %637 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %638 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %639 = getelementptr inbounds nuw i8, ptr %self, i64 1792
  %640 = getelementptr inbounds nuw i8, ptr %self, i64 1824
  %641 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %642 = getelementptr inbounds nuw i8, ptr %self, i64 1856
  %643 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %644 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %645 = getelementptr inbounds nuw i8, ptr %self, i64 2016
  %646 = getelementptr inbounds nuw i8, ptr %self, i64 2144
  %647 = getelementptr inbounds nuw i8, ptr %self, i64 2272
  %648 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %649 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %650 = getelementptr inbounds nuw i8, ptr %self, i64 2400
  %651 = getelementptr inbounds nuw i8, ptr %self, i64 2432
  %652 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %653 = getelementptr inbounds nuw i8, ptr %self, i64 2464
  %654 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %655 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %656 = lshr i64 %_63.i, 3, !dbg !14693
  %657 = lshr i64 %_55.i, 3, !dbg !14693
  %658 = getelementptr inbounds nuw i8, ptr %self, i64 1188
  %659 = getelementptr inbounds nuw i8, ptr %self, i64 1252
  %660 = getelementptr inbounds nuw i8, ptr %self, i64 1192
  %661 = getelementptr inbounds nuw i8, ptr %self, i64 1256
  %662 = getelementptr inbounds nuw i8, ptr %self, i64 1196
  %663 = getelementptr inbounds nuw i8, ptr %self, i64 1260
  %664 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %665 = getelementptr inbounds nuw i8, ptr %self, i64 1264
  %666 = getelementptr inbounds nuw i8, ptr %self, i64 1204
  %667 = getelementptr inbounds nuw i8, ptr %self, i64 1268
  %668 = getelementptr inbounds nuw i8, ptr %self, i64 1208
  %669 = getelementptr inbounds nuw i8, ptr %self, i64 1272
  %670 = getelementptr inbounds nuw i8, ptr %self, i64 1212
  %671 = getelementptr inbounds nuw i8, ptr %self, i64 1276
  br label %bb30.i.i46.i, !dbg !14693

bb30.i.i46.i:                                     ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i272.i, %bb7.i32.i
  %iter.sroa.0.0.i1684.i.i = phi i64 [ 0, %bb7.i32.i ], [ %672, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i272.i ]
  %672 = add nuw nsw i64 %iter.sroa.0.0.i1684.i.i, 1, !dbg !14702
  %span.i.i47.i = shl i64 %iter.sroa.0.0.i1684.i.i, 3, !dbg !14708
  %_28.i.i48.i = trunc i64 %iter.sroa.0.0.i1684.i.i to i32, !dbg !14710
  %now.i.i49.i = add i32 %base.i.i45.i, %_28.i.i48.i, !dbg !14712
  %_31.i.i50.i = and i32 %now.i.i49.i, %_58.i39.i, !dbg !14715
  %_30.i.i51.i = zext i32 %_31.i.i50.i to i64, !dbg !14717
  %write.i.i52.i = shl nuw nsw i64 %_30.i.i51.i, 3, !dbg !14717
  %exitcond.not.i53.i = icmp eq i64 %iter.sroa.0.0.i1684.i.i, %657, !dbg !14718
  br i1 %exitcond.not.i53.i, label %bb33.i.i599.i, label %bb32.i.i54.i, !dbg !14718, !prof !2561

bb33.i.i599.i:                                    ; preds = %bb30.i.i46.i
  %_35.i.i600.i = add nuw nsw i64 %span.i.i47.i, 8, !dbg !14726
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i47.i, i64 noundef %_35.i.i600.i, i64 noundef range(i64 0, 2305843009213693952) %_55.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d0dbd696a058609bd94bbe1fa75d0723) #24, !dbg !14727, !noalias !14690
  unreachable, !dbg !14727

bb32.i.i54.i:                                     ; preds = %bb30.i.i46.i
  %_97.i.i55.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %span.i.i47.i, !dbg !14728
  %_37.i.i56.i = add nuw nsw i64 %write.i.i52.i, 8, !dbg !14732
  %_98.not.i.i57.i = icmp ugt i64 %_37.i.i56.i, %_64.1.i35.i, !dbg !14733
  br i1 %_98.not.i.i57.i, label %bb36.i.i598.i, label %bb35.i.i58.i, !dbg !14733, !prof !180

bb36.i.i598.i:                                    ; preds = %bb32.i.i54.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i52.i, i64 noundef %_37.i.i56.i, i64 noundef %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1eeef3195352adc58f4bfdb015316fef) #24, !dbg !14738, !noalias !14690
  unreachable, !dbg !14738

bb35.i.i58.i:                                     ; preds = %bb32.i.i54.i
  %_107.i.i59.i = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %write.i.i52.i, !dbg !14739
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_107.i.i59.i, ptr noundef nonnull align 4 dereferenceable(32) %_97.i.i55.i, i64 32, i1 false), !dbg !14743, !noalias !14748
  %exitcond1819.i.i = icmp eq i64 %iter.sroa.0.0.i1684.i.i, %656, !dbg !14749
  br i1 %exitcond1819.i.i, label %bb39.i.i597.i, label %bb38.i.i60.i, !dbg !14749, !prof !180

bb39.i.i597.i:                                    ; preds = %bb35.i.i58.i
  %673 = and i64 %_63.i, 2305843009213693944, !dbg !14693
  %674 = add nuw nsw i64 %673, 8, !dbg !14693
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i47.i, i64 noundef %674, i64 noundef range(i64 0, 2305843009213693952) %_63.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d9529ff5ddc99dd60299cff5ff3cd676) #24, !dbg !14753, !noalias !14690
  unreachable, !dbg !14753

bb38.i.i60.i:                                     ; preds = %bb35.i.i58.i
  %_115.i.i61.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %span.i.i47.i, !dbg !14754
  %_116.not.i.i62.i = icmp ugt i64 %_37.i.i56.i, %_66.1.i37.i, !dbg !14758
  br i1 %_116.not.i.i62.i, label %bb41.i.i596.i, label %bb40.i.i63.i, !dbg !14758, !prof !180

bb41.i.i596.i:                                    ; preds = %bb38.i.i60.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i52.i, i64 noundef %_37.i.i56.i, i64 noundef %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9d56679ca30f2caa500ddf2e89d00cb) #24, !dbg !14762, !noalias !14690
  unreachable, !dbg !14762

bb40.i.i63.i:                                     ; preds = %bb38.i.i60.i
  %_123.i.i64.i = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %write.i.i52.i, !dbg !14763
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_123.i.i64.i, ptr noundef nonnull align 4 dereferenceable(32) %_115.i.i61.i, i64 32, i1 false), !dbg !14767, !noalias !14772
  %_53.i.i65.i = sub i32 %now.i.i49.i, %_59.i40.i, !dbg !14773
  %_52.i.i66.i = and i32 %_53.i.i65.i, %_58.i39.i, !dbg !14776
  %_51.i.i67.i = zext i32 %_52.i.i66.i to i64, !dbg !14777
  %read.i.i68.i = shl nuw nsw i64 %_51.i.i67.i, 3, !dbg !14777
  %_56.i.i69.i = add nuw nsw i64 %read.i.i68.i, 8, !dbg !14778
  %_156.not.i.i70.i = icmp ugt i64 %_56.i.i69.i, %_64.1.i35.i, !dbg !14780
  br i1 %_156.not.i.i70.i, label %bb51.i.i595.i, label %bb50.i.i71.i, !dbg !14780, !prof !180

bb51.i.i595.i:                                    ; preds = %bb40.i.i63.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i68.i, i64 noundef %_56.i.i69.i, i64 noundef %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7) #24, !dbg !14784, !noalias !14690
  unreachable, !dbg !14784

bb50.i.i71.i:                                     ; preds = %bb40.i.i63.i
  %_163.i.i72.i = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %read.i.i68.i, !dbg !14785
  %lanes.i447.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_163.i.i72.i, align 4, !dbg !14789, !alias.scope !14794, !noalias !14798
  %_164.not.i.i73.i = icmp ugt i64 %_56.i.i69.i, %_66.1.i37.i, !dbg !14802
  br i1 %_164.not.i.i73.i, label %bb54.i.i594.i, label %bb53.i.i74.i, !dbg !14802, !prof !180

bb54.i.i594.i:                                    ; preds = %bb50.i.i71.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i68.i, i64 noundef %_56.i.i69.i, i64 noundef %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f198228fd40f4c04a48a745576c5c5ee) #24, !dbg !14807, !noalias !14690
  unreachable, !dbg !14807

bb53.i.i74.i:                                     ; preds = %bb50.i.i71.i
  %_169.i.i75.i = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %read.i.i68.i, !dbg !14808
  %lanes.i441.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_169.i.i75.i, align 4, !dbg !14812, !alias.scope !14817, !noalias !14821
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14825), !dbg !14828
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14831), !dbg !14828
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14833), !dbg !14828
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14835), !dbg !14828
  %_18.i71.i76.i = load i32, ptr %_31.i26.i, align 4, !dbg !14837, !alias.scope !14839, !noalias !14840, !noundef !12
  %_17.i.i77.i = sub i32 %now.i.i49.i, %_18.i71.i76.i, !dbg !14842
  %_16.i72.i78.i = and i32 %_17.i.i77.i, %_58.i39.i, !dbg !14837
  %_15.i73.i79.i = zext i32 %_16.i72.i78.i to i64, !dbg !14837
  %_14.i.i80.i = shl nuw nsw i64 %_15.i73.i79.i, 3, !dbg !14837
  %_26.i.i81.i = load i32, ptr %_33.i27.i, align 4, !dbg !14837, !alias.scope !14844, !noalias !14845, !noundef !12
  %_25.i.i82.i = sub i32 %now.i.i49.i, %_26.i.i81.i, !dbg !14842
  %_24.i.i83.i = and i32 %_25.i.i82.i, %_58.i39.i, !dbg !14837
  %_23.i76.i84.i = zext i32 %_24.i.i83.i to i64, !dbg !14837
  %_22.i.i85.i = shl nuw nsw i64 %_23.i76.i84.i, 3, !dbg !14837
  %_31.i77.i86.i = icmp samesign ult i64 %_14.i.i80.i, %_64.1.i35.i, !dbg !14837
  switch i8 %_0.sroa.0.0.i836.i.i, label %default.unreachable [
    i8 0, label %bb7.i75.preheader.i458.i
    i8 1, label %bb16.i.preheader.i323.i
    i8 2, label %bb25.i.preheader.i87.i
  ], !dbg !14846

bb25.i.preheader.i87.i:                           ; preds = %bb53.i.i74.i
  br i1 %_31.i77.i86.i, label %bb27.i58.i90.i, label %panic28.i.i88.i, !dbg !14847

bb16.i.preheader.i323.i:                          ; preds = %bb53.i.i74.i
  br i1 %_31.i77.i86.i, label %bb17.i.i326.i, label %panic15.i.i324.i, !dbg !14848

bb7.i75.preheader.i458.i:                         ; preds = %bb53.i.i74.i
  br i1 %_31.i77.i86.i, label %bb8.i78.i461.i, label %panic4.i.i459.i, !dbg !14849

bb8.i78.i461.i:                                   ; preds = %bb7.i75.preheader.i458.i
  %_34.i.i462.i = icmp samesign ult i64 %_22.i.i85.i, %_66.1.i37.i, !dbg !14850
  br i1 %_34.i.i462.i, label %bb10.i79.i464.i, label %panic5.i.i463.i, !dbg !14850

panic4.i.i459.i:                                  ; preds = %bb10.i79.6.i572.i, %bb10.i79.5.i554.i, %bb10.i79.4.i536.i, %bb10.i79.3.i518.i, %bb10.i79.2.i500.i, %bb10.i79.1.i482.i, %bb10.i79.i464.i, %bb7.i75.preheader.i458.i
  %left.i.lcssa.i460.i = phi i64 [ %_14.i.i80.i, %bb7.i75.preheader.i458.i ], [ %left.i.1.i472.i, %bb10.i79.i464.i ], [ %left.i.2.i490.i, %bb10.i79.1.i482.i ], [ %left.i.3.i508.i, %bb10.i79.2.i500.i ], [ %left.i.4.i526.i, %bb10.i79.3.i518.i ], [ %left.i.5.i544.i, %bb10.i79.4.i536.i ], [ %left.i.6.i562.i, %bb10.i79.5.i554.i ], [ %left.i.7.i580.i, %bb10.i79.6.i572.i ], !dbg !14851
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left.i.lcssa.i460.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cba26bb3a04aaba20f9c249edc5e133d) #24, !dbg !14849, !noalias !14852
  unreachable, !dbg !14849

panic5.i.i463.i:                                  ; preds = %bb8.i78.7.i588.i, %bb8.i78.6.i570.i, %bb8.i78.5.i552.i, %bb8.i78.4.i534.i, %bb8.i78.3.i516.i, %bb8.i78.2.i498.i, %bb8.i78.1.i480.i, %bb8.i78.i461.i
  %right.i.lcssa1699.i.i = phi i64 [ %_22.i.i85.i, %bb8.i78.i461.i ], [ %right.i.1.i478.i, %bb8.i78.1.i480.i ], [ %right.i.2.i496.i, %bb8.i78.2.i498.i ], [ %right.i.3.i514.i, %bb8.i78.3.i516.i ], [ %right.i.4.i532.i, %bb8.i78.4.i534.i ], [ %right.i.5.i550.i, %bb8.i78.5.i552.i ], [ %right.i.6.i568.i, %bb8.i78.6.i570.i ], [ %right.i.7.i586.i, %bb8.i78.7.i588.i ], !dbg !14853
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right.i.lcssa1699.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2ad2d313de59c36d62f4a7d75017a71f) #24, !dbg !14850, !noalias !14852
  unreachable, !dbg !14850

bb10.i79.i464.i:                                  ; preds = %bb8.i78.i461.i
  %675 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %_14.i.i80.i, !dbg !14849
  %left_own.i.i465.i = load float, ptr %675, align 4, !dbg !14849, !alias.scope !14833, !noalias !14854, !noundef !12
  %676 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %_22.i.i85.i, !dbg !14850
  %right_own.i.i466.i = load float, ptr %676, align 4, !dbg !14850, !alias.scope !14835, !noalias !14855, !noundef !12
  %_18.i71.1.i467.i = load i32, ptr %658, align 4, !dbg !14856, !alias.scope !14839, !noalias !14840, !noundef !12
  %_17.i.1.i468.i = sub i32 %now.i.i49.i, %_18.i71.1.i467.i, !dbg !14857
  %_16.i72.1.i469.i = and i32 %_17.i.1.i468.i, %_58.i39.i, !dbg !14859
  %_15.i73.1.i470.i = zext i32 %_16.i72.1.i469.i to i64, !dbg !14851
  %_14.i.1.i471.i = shl nuw nsw i64 %_15.i73.1.i470.i, 3, !dbg !14851
  %left.i.1.i472.i = or disjoint i64 %_14.i.1.i471.i, 1, !dbg !14851
  %_26.i.1.i473.i = load i32, ptr %659, align 4, !dbg !14860, !alias.scope !14844, !noalias !14845, !noundef !12
  %_25.i.1.i474.i = sub i32 %now.i.i49.i, %_26.i.1.i473.i, !dbg !14861
  %_24.i.1.i475.i = and i32 %_25.i.1.i474.i, %_58.i39.i, !dbg !14863
  %_23.i76.1.i476.i = zext i32 %_24.i.1.i475.i to i64, !dbg !14853
  %_22.i.1.i477.i = shl nuw nsw i64 %_23.i76.1.i476.i, 3, !dbg !14853
  %right.i.1.i478.i = or disjoint i64 %_22.i.1.i477.i, 1, !dbg !14853
  %_31.i77.1.i479.i = icmp samesign ult i64 %left.i.1.i472.i, %_64.1.i35.i, !dbg !14849
  br i1 %_31.i77.1.i479.i, label %bb8.i78.1.i480.i, label %panic4.i.i459.i, !dbg !14849

bb8.i78.1.i480.i:                                 ; preds = %bb10.i79.i464.i
  %_34.i.1.i481.i = icmp samesign ult i64 %right.i.1.i478.i, %_66.1.i37.i, !dbg !14850
  br i1 %_34.i.1.i481.i, label %bb10.i79.1.i482.i, label %panic5.i.i463.i, !dbg !14850

bb10.i79.1.i482.i:                                ; preds = %bb8.i78.1.i480.i
  %677 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.1.i472.i, !dbg !14849
  %left_own.i.1.i483.i = load float, ptr %677, align 4, !dbg !14849, !alias.scope !14833, !noalias !14854, !noundef !12
  %678 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.1.i478.i, !dbg !14850
  %right_own.i.1.i484.i = load float, ptr %678, align 4, !dbg !14850, !alias.scope !14835, !noalias !14855, !noundef !12
  %_18.i71.2.i485.i = load i32, ptr %660, align 4, !dbg !14856, !alias.scope !14839, !noalias !14840, !noundef !12
  %_17.i.2.i486.i = sub i32 %now.i.i49.i, %_18.i71.2.i485.i, !dbg !14857
  %_16.i72.2.i487.i = and i32 %_17.i.2.i486.i, %_58.i39.i, !dbg !14859
  %_15.i73.2.i488.i = zext i32 %_16.i72.2.i487.i to i64, !dbg !14851
  %_14.i.2.i489.i = shl nuw nsw i64 %_15.i73.2.i488.i, 3, !dbg !14851
  %left.i.2.i490.i = or disjoint i64 %_14.i.2.i489.i, 2, !dbg !14851
  %_26.i.2.i491.i = load i32, ptr %661, align 4, !dbg !14860, !alias.scope !14844, !noalias !14845, !noundef !12
  %_25.i.2.i492.i = sub i32 %now.i.i49.i, %_26.i.2.i491.i, !dbg !14861
  %_24.i.2.i493.i = and i32 %_25.i.2.i492.i, %_58.i39.i, !dbg !14863
  %_23.i76.2.i494.i = zext i32 %_24.i.2.i493.i to i64, !dbg !14853
  %_22.i.2.i495.i = shl nuw nsw i64 %_23.i76.2.i494.i, 3, !dbg !14853
  %right.i.2.i496.i = or disjoint i64 %_22.i.2.i495.i, 2, !dbg !14853
  %_31.i77.2.i497.i = icmp samesign ult i64 %left.i.2.i490.i, %_64.1.i35.i, !dbg !14849
  br i1 %_31.i77.2.i497.i, label %bb8.i78.2.i498.i, label %panic4.i.i459.i, !dbg !14849

bb8.i78.2.i498.i:                                 ; preds = %bb10.i79.1.i482.i
  %_34.i.2.i499.i = icmp samesign ult i64 %right.i.2.i496.i, %_66.1.i37.i, !dbg !14850
  br i1 %_34.i.2.i499.i, label %bb10.i79.2.i500.i, label %panic5.i.i463.i, !dbg !14850

bb10.i79.2.i500.i:                                ; preds = %bb8.i78.2.i498.i
  %679 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.2.i490.i, !dbg !14849
  %left_own.i.2.i501.i = load float, ptr %679, align 4, !dbg !14849, !alias.scope !14833, !noalias !14854, !noundef !12
  %680 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.2.i496.i, !dbg !14850
  %right_own.i.2.i502.i = load float, ptr %680, align 4, !dbg !14850, !alias.scope !14835, !noalias !14855, !noundef !12
  %_18.i71.3.i503.i = load i32, ptr %662, align 4, !dbg !14856, !alias.scope !14839, !noalias !14840, !noundef !12
  %_17.i.3.i504.i = sub i32 %now.i.i49.i, %_18.i71.3.i503.i, !dbg !14857
  %_16.i72.3.i505.i = and i32 %_17.i.3.i504.i, %_58.i39.i, !dbg !14859
  %_15.i73.3.i506.i = zext i32 %_16.i72.3.i505.i to i64, !dbg !14851
  %_14.i.3.i507.i = shl nuw nsw i64 %_15.i73.3.i506.i, 3, !dbg !14851
  %left.i.3.i508.i = or disjoint i64 %_14.i.3.i507.i, 3, !dbg !14851
  %_26.i.3.i509.i = load i32, ptr %663, align 4, !dbg !14860, !alias.scope !14844, !noalias !14845, !noundef !12
  %_25.i.3.i510.i = sub i32 %now.i.i49.i, %_26.i.3.i509.i, !dbg !14861
  %_24.i.3.i511.i = and i32 %_25.i.3.i510.i, %_58.i39.i, !dbg !14863
  %_23.i76.3.i512.i = zext i32 %_24.i.3.i511.i to i64, !dbg !14853
  %_22.i.3.i513.i = shl nuw nsw i64 %_23.i76.3.i512.i, 3, !dbg !14853
  %right.i.3.i514.i = or disjoint i64 %_22.i.3.i513.i, 3, !dbg !14853
  %_31.i77.3.i515.i = icmp samesign ult i64 %left.i.3.i508.i, %_64.1.i35.i, !dbg !14849
  br i1 %_31.i77.3.i515.i, label %bb8.i78.3.i516.i, label %panic4.i.i459.i, !dbg !14849

bb8.i78.3.i516.i:                                 ; preds = %bb10.i79.2.i500.i
  %_34.i.3.i517.i = icmp samesign ult i64 %right.i.3.i514.i, %_66.1.i37.i, !dbg !14850
  br i1 %_34.i.3.i517.i, label %bb10.i79.3.i518.i, label %panic5.i.i463.i, !dbg !14850

bb10.i79.3.i518.i:                                ; preds = %bb8.i78.3.i516.i
  %681 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.3.i508.i, !dbg !14849
  %left_own.i.3.i519.i = load float, ptr %681, align 4, !dbg !14849, !alias.scope !14833, !noalias !14854, !noundef !12
  %682 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.3.i514.i, !dbg !14850
  %right_own.i.3.i520.i = load float, ptr %682, align 4, !dbg !14850, !alias.scope !14835, !noalias !14855, !noundef !12
  %_18.i71.4.i521.i = load i32, ptr %664, align 4, !dbg !14856, !alias.scope !14839, !noalias !14840, !noundef !12
  %_17.i.4.i522.i = sub i32 %now.i.i49.i, %_18.i71.4.i521.i, !dbg !14857
  %_16.i72.4.i523.i = and i32 %_17.i.4.i522.i, %_58.i39.i, !dbg !14859
  %_15.i73.4.i524.i = zext i32 %_16.i72.4.i523.i to i64, !dbg !14851
  %_14.i.4.i525.i = shl nuw nsw i64 %_15.i73.4.i524.i, 3, !dbg !14851
  %left.i.4.i526.i = or disjoint i64 %_14.i.4.i525.i, 4, !dbg !14851
  %_26.i.4.i527.i = load i32, ptr %665, align 4, !dbg !14860, !alias.scope !14844, !noalias !14845, !noundef !12
  %_25.i.4.i528.i = sub i32 %now.i.i49.i, %_26.i.4.i527.i, !dbg !14861
  %_24.i.4.i529.i = and i32 %_25.i.4.i528.i, %_58.i39.i, !dbg !14863
  %_23.i76.4.i530.i = zext i32 %_24.i.4.i529.i to i64, !dbg !14853
  %_22.i.4.i531.i = shl nuw nsw i64 %_23.i76.4.i530.i, 3, !dbg !14853
  %right.i.4.i532.i = or disjoint i64 %_22.i.4.i531.i, 4, !dbg !14853
  %_31.i77.4.i533.i = icmp samesign ult i64 %left.i.4.i526.i, %_64.1.i35.i, !dbg !14849
  br i1 %_31.i77.4.i533.i, label %bb8.i78.4.i534.i, label %panic4.i.i459.i, !dbg !14849

bb8.i78.4.i534.i:                                 ; preds = %bb10.i79.3.i518.i
  %_34.i.4.i535.i = icmp samesign ult i64 %right.i.4.i532.i, %_66.1.i37.i, !dbg !14850
  br i1 %_34.i.4.i535.i, label %bb10.i79.4.i536.i, label %panic5.i.i463.i, !dbg !14850

bb10.i79.4.i536.i:                                ; preds = %bb8.i78.4.i534.i
  %683 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.4.i526.i, !dbg !14849
  %left_own.i.4.i537.i = load float, ptr %683, align 4, !dbg !14849, !alias.scope !14833, !noalias !14854, !noundef !12
  %684 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.4.i532.i, !dbg !14850
  %right_own.i.4.i538.i = load float, ptr %684, align 4, !dbg !14850, !alias.scope !14835, !noalias !14855, !noundef !12
  %_18.i71.5.i539.i = load i32, ptr %666, align 4, !dbg !14856, !alias.scope !14839, !noalias !14840, !noundef !12
  %_17.i.5.i540.i = sub i32 %now.i.i49.i, %_18.i71.5.i539.i, !dbg !14857
  %_16.i72.5.i541.i = and i32 %_17.i.5.i540.i, %_58.i39.i, !dbg !14859
  %_15.i73.5.i542.i = zext i32 %_16.i72.5.i541.i to i64, !dbg !14851
  %_14.i.5.i543.i = shl nuw nsw i64 %_15.i73.5.i542.i, 3, !dbg !14851
  %left.i.5.i544.i = or disjoint i64 %_14.i.5.i543.i, 5, !dbg !14851
  %_26.i.5.i545.i = load i32, ptr %667, align 4, !dbg !14860, !alias.scope !14844, !noalias !14845, !noundef !12
  %_25.i.5.i546.i = sub i32 %now.i.i49.i, %_26.i.5.i545.i, !dbg !14861
  %_24.i.5.i547.i = and i32 %_25.i.5.i546.i, %_58.i39.i, !dbg !14863
  %_23.i76.5.i548.i = zext i32 %_24.i.5.i547.i to i64, !dbg !14853
  %_22.i.5.i549.i = shl nuw nsw i64 %_23.i76.5.i548.i, 3, !dbg !14853
  %right.i.5.i550.i = or disjoint i64 %_22.i.5.i549.i, 5, !dbg !14853
  %_31.i77.5.i551.i = icmp samesign ult i64 %left.i.5.i544.i, %_64.1.i35.i, !dbg !14849
  br i1 %_31.i77.5.i551.i, label %bb8.i78.5.i552.i, label %panic4.i.i459.i, !dbg !14849

bb8.i78.5.i552.i:                                 ; preds = %bb10.i79.4.i536.i
  %_34.i.5.i553.i = icmp samesign ult i64 %right.i.5.i550.i, %_66.1.i37.i, !dbg !14850
  br i1 %_34.i.5.i553.i, label %bb10.i79.5.i554.i, label %panic5.i.i463.i, !dbg !14850

bb10.i79.5.i554.i:                                ; preds = %bb8.i78.5.i552.i
  %685 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.5.i544.i, !dbg !14849
  %left_own.i.5.i555.i = load float, ptr %685, align 4, !dbg !14849, !alias.scope !14833, !noalias !14854, !noundef !12
  %686 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.5.i550.i, !dbg !14850
  %right_own.i.5.i556.i = load float, ptr %686, align 4, !dbg !14850, !alias.scope !14835, !noalias !14855, !noundef !12
  %_18.i71.6.i557.i = load i32, ptr %668, align 4, !dbg !14856, !alias.scope !14839, !noalias !14840, !noundef !12
  %_17.i.6.i558.i = sub i32 %now.i.i49.i, %_18.i71.6.i557.i, !dbg !14857
  %_16.i72.6.i559.i = and i32 %_17.i.6.i558.i, %_58.i39.i, !dbg !14859
  %_15.i73.6.i560.i = zext i32 %_16.i72.6.i559.i to i64, !dbg !14851
  %_14.i.6.i561.i = shl nuw nsw i64 %_15.i73.6.i560.i, 3, !dbg !14851
  %left.i.6.i562.i = or disjoint i64 %_14.i.6.i561.i, 6, !dbg !14851
  %_26.i.6.i563.i = load i32, ptr %669, align 4, !dbg !14860, !alias.scope !14844, !noalias !14845, !noundef !12
  %_25.i.6.i564.i = sub i32 %now.i.i49.i, %_26.i.6.i563.i, !dbg !14861
  %_24.i.6.i565.i = and i32 %_25.i.6.i564.i, %_58.i39.i, !dbg !14863
  %_23.i76.6.i566.i = zext i32 %_24.i.6.i565.i to i64, !dbg !14853
  %_22.i.6.i567.i = shl nuw nsw i64 %_23.i76.6.i566.i, 3, !dbg !14853
  %right.i.6.i568.i = or disjoint i64 %_22.i.6.i567.i, 6, !dbg !14853
  %_31.i77.6.i569.i = icmp samesign ult i64 %left.i.6.i562.i, %_64.1.i35.i, !dbg !14849
  br i1 %_31.i77.6.i569.i, label %bb8.i78.6.i570.i, label %panic4.i.i459.i, !dbg !14849

bb8.i78.6.i570.i:                                 ; preds = %bb10.i79.5.i554.i
  %_34.i.6.i571.i = icmp samesign ult i64 %right.i.6.i568.i, %_66.1.i37.i, !dbg !14850
  br i1 %_34.i.6.i571.i, label %bb10.i79.6.i572.i, label %panic5.i.i463.i, !dbg !14850

bb10.i79.6.i572.i:                                ; preds = %bb8.i78.6.i570.i
  %687 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.6.i562.i, !dbg !14849
  %left_own.i.6.i573.i = load float, ptr %687, align 4, !dbg !14849, !alias.scope !14833, !noalias !14854, !noundef !12
  %688 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.6.i568.i, !dbg !14850
  %right_own.i.6.i574.i = load float, ptr %688, align 4, !dbg !14850, !alias.scope !14835, !noalias !14855, !noundef !12
  %_18.i71.7.i575.i = load i32, ptr %670, align 4, !dbg !14856, !alias.scope !14839, !noalias !14840, !noundef !12
  %_17.i.7.i576.i = sub i32 %now.i.i49.i, %_18.i71.7.i575.i, !dbg !14857
  %_16.i72.7.i577.i = and i32 %_17.i.7.i576.i, %_58.i39.i, !dbg !14859
  %_15.i73.7.i578.i = zext i32 %_16.i72.7.i577.i to i64, !dbg !14851
  %_14.i.7.i579.i = shl nuw nsw i64 %_15.i73.7.i578.i, 3, !dbg !14851
  %left.i.7.i580.i = or disjoint i64 %_14.i.7.i579.i, 7, !dbg !14851
  %_26.i.7.i581.i = load i32, ptr %671, align 4, !dbg !14860, !alias.scope !14844, !noalias !14845, !noundef !12
  %_25.i.7.i582.i = sub i32 %now.i.i49.i, %_26.i.7.i581.i, !dbg !14861
  %_24.i.7.i583.i = and i32 %_25.i.7.i582.i, %_58.i39.i, !dbg !14863
  %_23.i76.7.i584.i = zext i32 %_24.i.7.i583.i to i64, !dbg !14853
  %_22.i.7.i585.i = shl nuw nsw i64 %_23.i76.7.i584.i, 3, !dbg !14853
  %right.i.7.i586.i = or disjoint i64 %_22.i.7.i585.i, 7, !dbg !14853
  %_31.i77.7.i587.i = icmp samesign ult i64 %left.i.7.i580.i, %_64.1.i35.i, !dbg !14849
  br i1 %_31.i77.7.i587.i, label %bb8.i78.7.i588.i, label %panic4.i.i459.i, !dbg !14849

bb8.i78.7.i588.i:                                 ; preds = %bb10.i79.6.i572.i
  %_34.i.7.i589.i = icmp samesign ult i64 %right.i.7.i586.i, %_66.1.i37.i, !dbg !14850
  br i1 %_34.i.7.i589.i, label %bb10.i79.7.i590.i, label %panic5.i.i463.i, !dbg !14850

bb10.i79.7.i590.i:                                ; preds = %bb8.i78.7.i588.i
  %689 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.7.i580.i, !dbg !14849
  %left_own.i.7.i591.i = load float, ptr %689, align 4, !dbg !14849, !alias.scope !14833, !noalias !14854, !noundef !12
  %690 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.7.i586.i, !dbg !14850
  %right_own.i.7.i592.i = load float, ptr %690, align 4, !dbg !14850, !alias.scope !14835, !noalias !14855, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i272.i, !dbg !14864

bb17.i.i326.i:                                    ; preds = %bb16.i.preheader.i323.i
  %_59.i.i327.i = icmp samesign ult i64 %_22.i.i85.i, %_66.1.i37.i, !dbg !14867
  br i1 %_59.i.i327.i, label %bb19.i.i329.i, label %panic17.i.i328.i, !dbg !14867

panic15.i.i324.i:                                 ; preds = %bb19.i.6.i437.i, %bb19.i.5.i419.i, %bb19.i.4.i401.i, %bb19.i.3.i383.i, %bb19.i.2.i365.i, %bb19.i.1.i347.i, %bb19.i.i329.i, %bb16.i.preheader.i323.i
  %left12.i.lcssa.i325.i = phi i64 [ %_14.i.i80.i, %bb16.i.preheader.i323.i ], [ %left12.i.1.i337.i, %bb19.i.i329.i ], [ %left12.i.2.i355.i, %bb19.i.1.i347.i ], [ %left12.i.3.i373.i, %bb19.i.2.i365.i ], [ %left12.i.4.i391.i, %bb19.i.3.i383.i ], [ %left12.i.5.i409.i, %bb19.i.4.i401.i ], [ %left12.i.6.i427.i, %bb19.i.5.i419.i ], [ %left12.i.7.i445.i, %bb19.i.6.i437.i ], !dbg !14868
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left12.i.lcssa.i325.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2e01e5f964095ee8e5aa091c7d92b88) #24, !dbg !14848, !noalias !14852
  unreachable, !dbg !14848

panic17.i.i328.i:                                 ; preds = %bb17.i.7.i453.i, %bb17.i.6.i435.i, %bb17.i.5.i417.i, %bb17.i.4.i399.i, %bb17.i.3.i381.i, %bb17.i.2.i363.i, %bb17.i.1.i345.i, %bb17.i.i326.i
  %right14.i.lcssa1695.i.i = phi i64 [ %_22.i.i85.i, %bb17.i.i326.i ], [ %right14.i.1.i343.i, %bb17.i.1.i345.i ], [ %right14.i.2.i361.i, %bb17.i.2.i363.i ], [ %right14.i.3.i379.i, %bb17.i.3.i381.i ], [ %right14.i.4.i397.i, %bb17.i.4.i399.i ], [ %right14.i.5.i415.i, %bb17.i.5.i417.i ], [ %right14.i.6.i433.i, %bb17.i.6.i435.i ], [ %right14.i.7.i451.i, %bb17.i.7.i453.i ], !dbg !14869
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right14.i.lcssa1695.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_48dca199019b49040caac979cf1ddc5a) #24, !dbg !14867, !noalias !14852
  unreachable, !dbg !14867

bb19.i.i329.i:                                    ; preds = %bb17.i.i326.i
  %691 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %_14.i.i80.i, !dbg !14848
  %left_own16.i.i330.i = load float, ptr %691, align 4, !dbg !14848, !alias.scope !14833, !noalias !14854, !noundef !12
  %692 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %_22.i.i85.i, !dbg !14867
  %right_own18.i.i331.i = load float, ptr %692, align 4, !dbg !14867, !alias.scope !14835, !noalias !14855, !noundef !12
  %_43.i.1.i332.i = load i32, ptr %658, align 4, !dbg !14870, !alias.scope !14839, !noalias !14840, !noundef !12
  %_42.i.1.i333.i = sub i32 %now.i.i49.i, %_43.i.1.i332.i, !dbg !14871
  %_41.i.1.i334.i = and i32 %_42.i.1.i333.i, %_58.i39.i, !dbg !14873
  %_40.i.1.i335.i = zext i32 %_41.i.1.i334.i to i64, !dbg !14868
  %_39.i63.1.i336.i = shl nuw nsw i64 %_40.i.1.i335.i, 3, !dbg !14868
  %left12.i.1.i337.i = or disjoint i64 %_39.i63.1.i336.i, 1, !dbg !14868
  %_51.i65.1.i338.i = load i32, ptr %659, align 4, !dbg !14874, !alias.scope !14844, !noalias !14845, !noundef !12
  %_50.i.1.i339.i = sub i32 %now.i.i49.i, %_51.i65.1.i338.i, !dbg !14875
  %_49.i.1.i340.i = and i32 %_50.i.1.i339.i, %_58.i39.i, !dbg !14877
  %_48.i.1.i341.i = zext i32 %_49.i.1.i340.i to i64, !dbg !14869
  %_47.i.1.i342.i = shl nuw nsw i64 %_48.i.1.i341.i, 3, !dbg !14869
  %right14.i.1.i343.i = or disjoint i64 %_47.i.1.i342.i, 1, !dbg !14869
  %_56.i66.1.i344.i = icmp samesign ult i64 %left12.i.1.i337.i, %_64.1.i35.i, !dbg !14848
  br i1 %_56.i66.1.i344.i, label %bb17.i.1.i345.i, label %panic15.i.i324.i, !dbg !14848

bb17.i.1.i345.i:                                  ; preds = %bb19.i.i329.i
  %_59.i.1.i346.i = icmp samesign ult i64 %right14.i.1.i343.i, %_66.1.i37.i, !dbg !14867
  br i1 %_59.i.1.i346.i, label %bb19.i.1.i347.i, label %panic17.i.i328.i, !dbg !14867

bb19.i.1.i347.i:                                  ; preds = %bb17.i.1.i345.i
  %693 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.1.i337.i, !dbg !14848
  %left_own16.i.1.i348.i = load float, ptr %693, align 4, !dbg !14848, !alias.scope !14833, !noalias !14854, !noundef !12
  %694 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.1.i343.i, !dbg !14867
  %right_own18.i.1.i349.i = load float, ptr %694, align 4, !dbg !14867, !alias.scope !14835, !noalias !14855, !noundef !12
  %_43.i.2.i350.i = load i32, ptr %660, align 4, !dbg !14870, !alias.scope !14839, !noalias !14840, !noundef !12
  %_42.i.2.i351.i = sub i32 %now.i.i49.i, %_43.i.2.i350.i, !dbg !14871
  %_41.i.2.i352.i = and i32 %_42.i.2.i351.i, %_58.i39.i, !dbg !14873
  %_40.i.2.i353.i = zext i32 %_41.i.2.i352.i to i64, !dbg !14868
  %_39.i63.2.i354.i = shl nuw nsw i64 %_40.i.2.i353.i, 3, !dbg !14868
  %left12.i.2.i355.i = or disjoint i64 %_39.i63.2.i354.i, 2, !dbg !14868
  %_51.i65.2.i356.i = load i32, ptr %661, align 4, !dbg !14874, !alias.scope !14844, !noalias !14845, !noundef !12
  %_50.i.2.i357.i = sub i32 %now.i.i49.i, %_51.i65.2.i356.i, !dbg !14875
  %_49.i.2.i358.i = and i32 %_50.i.2.i357.i, %_58.i39.i, !dbg !14877
  %_48.i.2.i359.i = zext i32 %_49.i.2.i358.i to i64, !dbg !14869
  %_47.i.2.i360.i = shl nuw nsw i64 %_48.i.2.i359.i, 3, !dbg !14869
  %right14.i.2.i361.i = or disjoint i64 %_47.i.2.i360.i, 2, !dbg !14869
  %_56.i66.2.i362.i = icmp samesign ult i64 %left12.i.2.i355.i, %_64.1.i35.i, !dbg !14848
  br i1 %_56.i66.2.i362.i, label %bb17.i.2.i363.i, label %panic15.i.i324.i, !dbg !14848

bb17.i.2.i363.i:                                  ; preds = %bb19.i.1.i347.i
  %_59.i.2.i364.i = icmp samesign ult i64 %right14.i.2.i361.i, %_66.1.i37.i, !dbg !14867
  br i1 %_59.i.2.i364.i, label %bb19.i.2.i365.i, label %panic17.i.i328.i, !dbg !14867

bb19.i.2.i365.i:                                  ; preds = %bb17.i.2.i363.i
  %695 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.2.i355.i, !dbg !14848
  %left_own16.i.2.i366.i = load float, ptr %695, align 4, !dbg !14848, !alias.scope !14833, !noalias !14854, !noundef !12
  %696 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.2.i361.i, !dbg !14867
  %right_own18.i.2.i367.i = load float, ptr %696, align 4, !dbg !14867, !alias.scope !14835, !noalias !14855, !noundef !12
  %_43.i.3.i368.i = load i32, ptr %662, align 4, !dbg !14870, !alias.scope !14839, !noalias !14840, !noundef !12
  %_42.i.3.i369.i = sub i32 %now.i.i49.i, %_43.i.3.i368.i, !dbg !14871
  %_41.i.3.i370.i = and i32 %_42.i.3.i369.i, %_58.i39.i, !dbg !14873
  %_40.i.3.i371.i = zext i32 %_41.i.3.i370.i to i64, !dbg !14868
  %_39.i63.3.i372.i = shl nuw nsw i64 %_40.i.3.i371.i, 3, !dbg !14868
  %left12.i.3.i373.i = or disjoint i64 %_39.i63.3.i372.i, 3, !dbg !14868
  %_51.i65.3.i374.i = load i32, ptr %663, align 4, !dbg !14874, !alias.scope !14844, !noalias !14845, !noundef !12
  %_50.i.3.i375.i = sub i32 %now.i.i49.i, %_51.i65.3.i374.i, !dbg !14875
  %_49.i.3.i376.i = and i32 %_50.i.3.i375.i, %_58.i39.i, !dbg !14877
  %_48.i.3.i377.i = zext i32 %_49.i.3.i376.i to i64, !dbg !14869
  %_47.i.3.i378.i = shl nuw nsw i64 %_48.i.3.i377.i, 3, !dbg !14869
  %right14.i.3.i379.i = or disjoint i64 %_47.i.3.i378.i, 3, !dbg !14869
  %_56.i66.3.i380.i = icmp samesign ult i64 %left12.i.3.i373.i, %_64.1.i35.i, !dbg !14848
  br i1 %_56.i66.3.i380.i, label %bb17.i.3.i381.i, label %panic15.i.i324.i, !dbg !14848

bb17.i.3.i381.i:                                  ; preds = %bb19.i.2.i365.i
  %_59.i.3.i382.i = icmp samesign ult i64 %right14.i.3.i379.i, %_66.1.i37.i, !dbg !14867
  br i1 %_59.i.3.i382.i, label %bb19.i.3.i383.i, label %panic17.i.i328.i, !dbg !14867

bb19.i.3.i383.i:                                  ; preds = %bb17.i.3.i381.i
  %697 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.3.i373.i, !dbg !14848
  %left_own16.i.3.i384.i = load float, ptr %697, align 4, !dbg !14848, !alias.scope !14833, !noalias !14854, !noundef !12
  %698 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.3.i379.i, !dbg !14867
  %right_own18.i.3.i385.i = load float, ptr %698, align 4, !dbg !14867, !alias.scope !14835, !noalias !14855, !noundef !12
  %_43.i.4.i386.i = load i32, ptr %664, align 4, !dbg !14870, !alias.scope !14839, !noalias !14840, !noundef !12
  %_42.i.4.i387.i = sub i32 %now.i.i49.i, %_43.i.4.i386.i, !dbg !14871
  %_41.i.4.i388.i = and i32 %_42.i.4.i387.i, %_58.i39.i, !dbg !14873
  %_40.i.4.i389.i = zext i32 %_41.i.4.i388.i to i64, !dbg !14868
  %_39.i63.4.i390.i = shl nuw nsw i64 %_40.i.4.i389.i, 3, !dbg !14868
  %left12.i.4.i391.i = or disjoint i64 %_39.i63.4.i390.i, 4, !dbg !14868
  %_51.i65.4.i392.i = load i32, ptr %665, align 4, !dbg !14874, !alias.scope !14844, !noalias !14845, !noundef !12
  %_50.i.4.i393.i = sub i32 %now.i.i49.i, %_51.i65.4.i392.i, !dbg !14875
  %_49.i.4.i394.i = and i32 %_50.i.4.i393.i, %_58.i39.i, !dbg !14877
  %_48.i.4.i395.i = zext i32 %_49.i.4.i394.i to i64, !dbg !14869
  %_47.i.4.i396.i = shl nuw nsw i64 %_48.i.4.i395.i, 3, !dbg !14869
  %right14.i.4.i397.i = or disjoint i64 %_47.i.4.i396.i, 4, !dbg !14869
  %_56.i66.4.i398.i = icmp samesign ult i64 %left12.i.4.i391.i, %_64.1.i35.i, !dbg !14848
  br i1 %_56.i66.4.i398.i, label %bb17.i.4.i399.i, label %panic15.i.i324.i, !dbg !14848

bb17.i.4.i399.i:                                  ; preds = %bb19.i.3.i383.i
  %_59.i.4.i400.i = icmp samesign ult i64 %right14.i.4.i397.i, %_66.1.i37.i, !dbg !14867
  br i1 %_59.i.4.i400.i, label %bb19.i.4.i401.i, label %panic17.i.i328.i, !dbg !14867

bb19.i.4.i401.i:                                  ; preds = %bb17.i.4.i399.i
  %699 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.4.i391.i, !dbg !14848
  %left_own16.i.4.i402.i = load float, ptr %699, align 4, !dbg !14848, !alias.scope !14833, !noalias !14854, !noundef !12
  %700 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.4.i397.i, !dbg !14867
  %right_own18.i.4.i403.i = load float, ptr %700, align 4, !dbg !14867, !alias.scope !14835, !noalias !14855, !noundef !12
  %_43.i.5.i404.i = load i32, ptr %666, align 4, !dbg !14870, !alias.scope !14839, !noalias !14840, !noundef !12
  %_42.i.5.i405.i = sub i32 %now.i.i49.i, %_43.i.5.i404.i, !dbg !14871
  %_41.i.5.i406.i = and i32 %_42.i.5.i405.i, %_58.i39.i, !dbg !14873
  %_40.i.5.i407.i = zext i32 %_41.i.5.i406.i to i64, !dbg !14868
  %_39.i63.5.i408.i = shl nuw nsw i64 %_40.i.5.i407.i, 3, !dbg !14868
  %left12.i.5.i409.i = or disjoint i64 %_39.i63.5.i408.i, 5, !dbg !14868
  %_51.i65.5.i410.i = load i32, ptr %667, align 4, !dbg !14874, !alias.scope !14844, !noalias !14845, !noundef !12
  %_50.i.5.i411.i = sub i32 %now.i.i49.i, %_51.i65.5.i410.i, !dbg !14875
  %_49.i.5.i412.i = and i32 %_50.i.5.i411.i, %_58.i39.i, !dbg !14877
  %_48.i.5.i413.i = zext i32 %_49.i.5.i412.i to i64, !dbg !14869
  %_47.i.5.i414.i = shl nuw nsw i64 %_48.i.5.i413.i, 3, !dbg !14869
  %right14.i.5.i415.i = or disjoint i64 %_47.i.5.i414.i, 5, !dbg !14869
  %_56.i66.5.i416.i = icmp samesign ult i64 %left12.i.5.i409.i, %_64.1.i35.i, !dbg !14848
  br i1 %_56.i66.5.i416.i, label %bb17.i.5.i417.i, label %panic15.i.i324.i, !dbg !14848

bb17.i.5.i417.i:                                  ; preds = %bb19.i.4.i401.i
  %_59.i.5.i418.i = icmp samesign ult i64 %right14.i.5.i415.i, %_66.1.i37.i, !dbg !14867
  br i1 %_59.i.5.i418.i, label %bb19.i.5.i419.i, label %panic17.i.i328.i, !dbg !14867

bb19.i.5.i419.i:                                  ; preds = %bb17.i.5.i417.i
  %701 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.5.i409.i, !dbg !14848
  %left_own16.i.5.i420.i = load float, ptr %701, align 4, !dbg !14848, !alias.scope !14833, !noalias !14854, !noundef !12
  %702 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.5.i415.i, !dbg !14867
  %right_own18.i.5.i421.i = load float, ptr %702, align 4, !dbg !14867, !alias.scope !14835, !noalias !14855, !noundef !12
  %_43.i.6.i422.i = load i32, ptr %668, align 4, !dbg !14870, !alias.scope !14839, !noalias !14840, !noundef !12
  %_42.i.6.i423.i = sub i32 %now.i.i49.i, %_43.i.6.i422.i, !dbg !14871
  %_41.i.6.i424.i = and i32 %_42.i.6.i423.i, %_58.i39.i, !dbg !14873
  %_40.i.6.i425.i = zext i32 %_41.i.6.i424.i to i64, !dbg !14868
  %_39.i63.6.i426.i = shl nuw nsw i64 %_40.i.6.i425.i, 3, !dbg !14868
  %left12.i.6.i427.i = or disjoint i64 %_39.i63.6.i426.i, 6, !dbg !14868
  %_51.i65.6.i428.i = load i32, ptr %669, align 4, !dbg !14874, !alias.scope !14844, !noalias !14845, !noundef !12
  %_50.i.6.i429.i = sub i32 %now.i.i49.i, %_51.i65.6.i428.i, !dbg !14875
  %_49.i.6.i430.i = and i32 %_50.i.6.i429.i, %_58.i39.i, !dbg !14877
  %_48.i.6.i431.i = zext i32 %_49.i.6.i430.i to i64, !dbg !14869
  %_47.i.6.i432.i = shl nuw nsw i64 %_48.i.6.i431.i, 3, !dbg !14869
  %right14.i.6.i433.i = or disjoint i64 %_47.i.6.i432.i, 6, !dbg !14869
  %_56.i66.6.i434.i = icmp samesign ult i64 %left12.i.6.i427.i, %_64.1.i35.i, !dbg !14848
  br i1 %_56.i66.6.i434.i, label %bb17.i.6.i435.i, label %panic15.i.i324.i, !dbg !14848

bb17.i.6.i435.i:                                  ; preds = %bb19.i.5.i419.i
  %_59.i.6.i436.i = icmp samesign ult i64 %right14.i.6.i433.i, %_66.1.i37.i, !dbg !14867
  br i1 %_59.i.6.i436.i, label %bb19.i.6.i437.i, label %panic17.i.i328.i, !dbg !14867

bb19.i.6.i437.i:                                  ; preds = %bb17.i.6.i435.i
  %703 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.6.i427.i, !dbg !14848
  %left_own16.i.6.i438.i = load float, ptr %703, align 4, !dbg !14848, !alias.scope !14833, !noalias !14854, !noundef !12
  %704 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.6.i433.i, !dbg !14867
  %right_own18.i.6.i439.i = load float, ptr %704, align 4, !dbg !14867, !alias.scope !14835, !noalias !14855, !noundef !12
  %_43.i.7.i440.i = load i32, ptr %670, align 4, !dbg !14870, !alias.scope !14839, !noalias !14840, !noundef !12
  %_42.i.7.i441.i = sub i32 %now.i.i49.i, %_43.i.7.i440.i, !dbg !14871
  %_41.i.7.i442.i = and i32 %_42.i.7.i441.i, %_58.i39.i, !dbg !14873
  %_40.i.7.i443.i = zext i32 %_41.i.7.i442.i to i64, !dbg !14868
  %_39.i63.7.i444.i = shl nuw nsw i64 %_40.i.7.i443.i, 3, !dbg !14868
  %left12.i.7.i445.i = or disjoint i64 %_39.i63.7.i444.i, 7, !dbg !14868
  %_51.i65.7.i446.i = load i32, ptr %671, align 4, !dbg !14874, !alias.scope !14844, !noalias !14845, !noundef !12
  %_50.i.7.i447.i = sub i32 %now.i.i49.i, %_51.i65.7.i446.i, !dbg !14875
  %_49.i.7.i448.i = and i32 %_50.i.7.i447.i, %_58.i39.i, !dbg !14877
  %_48.i.7.i449.i = zext i32 %_49.i.7.i448.i to i64, !dbg !14869
  %_47.i.7.i450.i = shl nuw nsw i64 %_48.i.7.i449.i, 3, !dbg !14869
  %right14.i.7.i451.i = or disjoint i64 %_47.i.7.i450.i, 7, !dbg !14869
  %_56.i66.7.i452.i = icmp samesign ult i64 %left12.i.7.i445.i, %_64.1.i35.i, !dbg !14848
  br i1 %_56.i66.7.i452.i, label %bb17.i.7.i453.i, label %panic15.i.i324.i, !dbg !14848

bb17.i.7.i453.i:                                  ; preds = %bb19.i.6.i437.i
  %_59.i.7.i454.i = icmp samesign ult i64 %right14.i.7.i451.i, %_66.1.i37.i, !dbg !14867
  br i1 %_59.i.7.i454.i, label %bb19.i.7.i455.i, label %panic17.i.i328.i, !dbg !14867

bb19.i.7.i455.i:                                  ; preds = %bb17.i.7.i453.i
  %705 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.7.i445.i, !dbg !14848
  %left_own16.i.7.i456.i = load float, ptr %705, align 4, !dbg !14848, !alias.scope !14833, !noalias !14854, !noundef !12
  %706 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.7.i451.i, !dbg !14867
  %right_own18.i.7.i457.i = load float, ptr %706, align 4, !dbg !14867, !alias.scope !14835, !noalias !14855, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i272.i, !dbg !14864

panic28.i.i88.i:                                  ; preds = %bb33.i60.6.i246.i, %bb33.i60.5.i222.i, %bb33.i60.4.i198.i, %bb33.i60.3.i174.i, %bb33.i60.2.i150.i, %bb33.i60.1.i126.i, %bb33.i60.i102.i, %bb25.i.preheader.i87.i
  %left25.i.lcssa.i89.i = phi i64 [ %_14.i.i80.i, %bb25.i.preheader.i87.i ], [ %left25.i.1.i109.i, %bb33.i60.i102.i ], [ %left25.i.2.i133.i, %bb33.i60.1.i126.i ], [ %left25.i.3.i157.i, %bb33.i60.2.i150.i ], [ %left25.i.4.i181.i, %bb33.i60.3.i174.i ], [ %left25.i.5.i205.i, %bb33.i60.4.i198.i ], [ %left25.i.6.i229.i, %bb33.i60.5.i222.i ], [ %left25.i.7.i253.i, %bb33.i60.6.i246.i ], !dbg !14878
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left25.i.lcssa.i89.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4b7b2eb7fa1b19ad0451b09b71313122) #24, !dbg !14847, !noalias !14852
  unreachable, !dbg !14847

bb27.i58.i90.i:                                   ; preds = %bb25.i.preheader.i87.i
  %707 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %_14.i.i80.i, !dbg !14847
  %_79.i.i91.i = load float, ptr %707, align 4, !dbg !14847, !alias.scope !14833, !noalias !14854, !noundef !12
  %_85.i.i92.i = icmp samesign ult i64 %_14.i.i80.i, %_66.1.i37.i, !dbg !14879
  br i1 %_85.i.i92.i, label %bb29.i59.i94.i, label %panic30.i.i93.i, !dbg !14879

panic30.i.i93.i:                                  ; preds = %bb27.i58.7.i261.i, %bb27.i58.6.i237.i, %bb27.i58.5.i213.i, %bb27.i58.4.i189.i, %bb27.i58.3.i165.i, %bb27.i58.2.i141.i, %bb27.i58.1.i117.i, %bb27.i58.i90.i
  %left25.i.lcssa1691.i.i = phi i64 [ %_14.i.i80.i, %bb27.i58.i90.i ], [ %left25.i.1.i109.i, %bb27.i58.1.i117.i ], [ %left25.i.2.i133.i, %bb27.i58.2.i141.i ], [ %left25.i.3.i157.i, %bb27.i58.3.i165.i ], [ %left25.i.4.i181.i, %bb27.i58.4.i189.i ], [ %left25.i.5.i205.i, %bb27.i58.5.i213.i ], [ %left25.i.6.i229.i, %bb27.i58.6.i237.i ], [ %left25.i.7.i253.i, %bb27.i58.7.i261.i ], !dbg !14878
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left25.i.lcssa1691.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60256fc2b51ee5bb69caa4304418caff) #24, !dbg !14879, !noalias !14852
  unreachable, !dbg !14879

bb29.i59.i94.i:                                   ; preds = %bb27.i58.i90.i
  %708 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %_14.i.i80.i, !dbg !14879
  %_83.i.i95.i = load float, ptr %708, align 4, !dbg !14879, !alias.scope !14835, !noalias !14855, !noundef !12
  %_87.i.i96.i = icmp samesign ult i64 %_22.i.i85.i, %_66.1.i37.i, !dbg !14880
  br i1 %_87.i.i96.i, label %bb31.i.i98.i, label %panic32.i.i97.i, !dbg !14880

panic32.i.i97.i:                                  ; preds = %bb29.i59.7.i264.i, %bb29.i59.6.i240.i, %bb29.i59.5.i216.i, %bb29.i59.4.i192.i, %bb29.i59.3.i168.i, %bb29.i59.2.i144.i, %bb29.i59.1.i120.i, %bb29.i59.i94.i
  %right27.i.lcssa1688.i.i = phi i64 [ %_22.i.i85.i, %bb29.i59.i94.i ], [ %right27.i.1.i115.i, %bb29.i59.1.i120.i ], [ %right27.i.2.i139.i, %bb29.i59.2.i144.i ], [ %right27.i.3.i163.i, %bb29.i59.3.i168.i ], [ %right27.i.4.i187.i, %bb29.i59.4.i192.i ], [ %right27.i.5.i211.i, %bb29.i59.5.i216.i ], [ %right27.i.6.i235.i, %bb29.i59.6.i240.i ], [ %right27.i.7.i259.i, %bb29.i59.7.i264.i ], !dbg !14881
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right27.i.lcssa1688.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_03bcac171bcf0d517141d8cd3ac9ded0) #24, !dbg !14880, !noalias !14852
  unreachable, !dbg !14880

bb31.i.i98.i:                                     ; preds = %bb29.i59.i94.i
  %709 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %_22.i.i85.i, !dbg !14880
  %_86.i.i99.i = load float, ptr %709, align 4, !dbg !14880, !alias.scope !14835, !noalias !14855, !noundef !12
  %_89.i.i100.i = icmp samesign ult i64 %_22.i.i85.i, %_64.1.i35.i, !dbg !14882
  br i1 %_89.i.i100.i, label %bb33.i60.i102.i, label %panic34.i.i101.i, !dbg !14882

panic34.i.i101.i:                                 ; preds = %bb31.i.7.i267.i, %bb31.i.6.i243.i, %bb31.i.5.i219.i, %bb31.i.4.i195.i, %bb31.i.3.i171.i, %bb31.i.2.i147.i, %bb31.i.1.i123.i, %bb31.i.i98.i
  %right27.i.lcssa1689.i.i = phi i64 [ %_22.i.i85.i, %bb31.i.i98.i ], [ %right27.i.1.i115.i, %bb31.i.1.i123.i ], [ %right27.i.2.i139.i, %bb31.i.2.i147.i ], [ %right27.i.3.i163.i, %bb31.i.3.i171.i ], [ %right27.i.4.i187.i, %bb31.i.4.i195.i ], [ %right27.i.5.i211.i, %bb31.i.5.i219.i ], [ %right27.i.6.i235.i, %bb31.i.6.i243.i ], [ %right27.i.7.i259.i, %bb31.i.7.i267.i ], !dbg !14881
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right27.i.lcssa1689.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ed84bd2438b7b7e3dfb2147bac09e923) #24, !dbg !14882, !noalias !14852
  unreachable, !dbg !14882

bb33.i60.i102.i:                                  ; preds = %bb31.i.i98.i
  %710 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %_22.i.i85.i, !dbg !14882
  %_88.i.i103.i = load float, ptr %710, align 4, !dbg !14882, !alias.scope !14833, !noalias !14854, !noundef !12
  %_68.i.1.i104.i = load i32, ptr %658, align 4, !dbg !14883, !alias.scope !14839, !noalias !14840, !noundef !12
  %_67.i52.1.i105.i = sub i32 %now.i.i49.i, %_68.i.1.i104.i, !dbg !14884
  %_66.i.1.i106.i = and i32 %_67.i52.1.i105.i, %_58.i39.i, !dbg !14886
  %_65.i.1.i107.i = zext i32 %_66.i.1.i106.i to i64, !dbg !14878
  %_64.i53.1.i108.i = shl nuw nsw i64 %_65.i.1.i107.i, 3, !dbg !14878
  %left25.i.1.i109.i = or disjoint i64 %_64.i53.1.i108.i, 1, !dbg !14878
  %_76.i54.1.i110.i = load i32, ptr %659, align 4, !dbg !14887, !alias.scope !14844, !noalias !14845, !noundef !12
  %_75.i.1.i111.i = sub i32 %now.i.i49.i, %_76.i54.1.i110.i, !dbg !14888
  %_74.i55.1.i112.i = and i32 %_75.i.1.i111.i, %_58.i39.i, !dbg !14890
  %_73.i56.1.i113.i = zext i32 %_74.i55.1.i112.i to i64, !dbg !14881
  %_72.i.1.i114.i = shl nuw nsw i64 %_73.i56.1.i113.i, 3, !dbg !14881
  %right27.i.1.i115.i = or disjoint i64 %_72.i.1.i114.i, 1, !dbg !14881
  %_81.i57.1.i116.i = icmp samesign ult i64 %left25.i.1.i109.i, %_64.1.i35.i, !dbg !14847
  br i1 %_81.i57.1.i116.i, label %bb27.i58.1.i117.i, label %panic28.i.i88.i, !dbg !14847

bb27.i58.1.i117.i:                                ; preds = %bb33.i60.i102.i
  %711 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.1.i109.i, !dbg !14847
  %_79.i.1.i118.i = load float, ptr %711, align 4, !dbg !14847, !alias.scope !14833, !noalias !14854, !noundef !12
  %_85.i.1.i119.i = icmp samesign ult i64 %left25.i.1.i109.i, %_66.1.i37.i, !dbg !14879
  br i1 %_85.i.1.i119.i, label %bb29.i59.1.i120.i, label %panic30.i.i93.i, !dbg !14879

bb29.i59.1.i120.i:                                ; preds = %bb27.i58.1.i117.i
  %712 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.1.i109.i, !dbg !14879
  %_83.i.1.i121.i = load float, ptr %712, align 4, !dbg !14879, !alias.scope !14835, !noalias !14855, !noundef !12
  %_87.i.1.i122.i = icmp samesign ult i64 %right27.i.1.i115.i, %_66.1.i37.i, !dbg !14880
  br i1 %_87.i.1.i122.i, label %bb31.i.1.i123.i, label %panic32.i.i97.i, !dbg !14880

bb31.i.1.i123.i:                                  ; preds = %bb29.i59.1.i120.i
  %713 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.1.i115.i, !dbg !14880
  %_86.i.1.i124.i = load float, ptr %713, align 4, !dbg !14880, !alias.scope !14835, !noalias !14855, !noundef !12
  %_89.i.1.i125.i = icmp samesign ult i64 %right27.i.1.i115.i, %_64.1.i35.i, !dbg !14882
  br i1 %_89.i.1.i125.i, label %bb33.i60.1.i126.i, label %panic34.i.i101.i, !dbg !14882

bb33.i60.1.i126.i:                                ; preds = %bb31.i.1.i123.i
  %714 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.1.i115.i, !dbg !14882
  %_88.i.1.i127.i = load float, ptr %714, align 4, !dbg !14882, !alias.scope !14833, !noalias !14854, !noundef !12
  %_68.i.2.i128.i = load i32, ptr %660, align 4, !dbg !14883, !alias.scope !14839, !noalias !14840, !noundef !12
  %_67.i52.2.i129.i = sub i32 %now.i.i49.i, %_68.i.2.i128.i, !dbg !14884
  %_66.i.2.i130.i = and i32 %_67.i52.2.i129.i, %_58.i39.i, !dbg !14886
  %_65.i.2.i131.i = zext i32 %_66.i.2.i130.i to i64, !dbg !14878
  %_64.i53.2.i132.i = shl nuw nsw i64 %_65.i.2.i131.i, 3, !dbg !14878
  %left25.i.2.i133.i = or disjoint i64 %_64.i53.2.i132.i, 2, !dbg !14878
  %_76.i54.2.i134.i = load i32, ptr %661, align 4, !dbg !14887, !alias.scope !14844, !noalias !14845, !noundef !12
  %_75.i.2.i135.i = sub i32 %now.i.i49.i, %_76.i54.2.i134.i, !dbg !14888
  %_74.i55.2.i136.i = and i32 %_75.i.2.i135.i, %_58.i39.i, !dbg !14890
  %_73.i56.2.i137.i = zext i32 %_74.i55.2.i136.i to i64, !dbg !14881
  %_72.i.2.i138.i = shl nuw nsw i64 %_73.i56.2.i137.i, 3, !dbg !14881
  %right27.i.2.i139.i = or disjoint i64 %_72.i.2.i138.i, 2, !dbg !14881
  %_81.i57.2.i140.i = icmp samesign ult i64 %left25.i.2.i133.i, %_64.1.i35.i, !dbg !14847
  br i1 %_81.i57.2.i140.i, label %bb27.i58.2.i141.i, label %panic28.i.i88.i, !dbg !14847

bb27.i58.2.i141.i:                                ; preds = %bb33.i60.1.i126.i
  %715 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.2.i133.i, !dbg !14847
  %_79.i.2.i142.i = load float, ptr %715, align 4, !dbg !14847, !alias.scope !14833, !noalias !14854, !noundef !12
  %_85.i.2.i143.i = icmp samesign ult i64 %left25.i.2.i133.i, %_66.1.i37.i, !dbg !14879
  br i1 %_85.i.2.i143.i, label %bb29.i59.2.i144.i, label %panic30.i.i93.i, !dbg !14879

bb29.i59.2.i144.i:                                ; preds = %bb27.i58.2.i141.i
  %716 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.2.i133.i, !dbg !14879
  %_83.i.2.i145.i = load float, ptr %716, align 4, !dbg !14879, !alias.scope !14835, !noalias !14855, !noundef !12
  %_87.i.2.i146.i = icmp samesign ult i64 %right27.i.2.i139.i, %_66.1.i37.i, !dbg !14880
  br i1 %_87.i.2.i146.i, label %bb31.i.2.i147.i, label %panic32.i.i97.i, !dbg !14880

bb31.i.2.i147.i:                                  ; preds = %bb29.i59.2.i144.i
  %717 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.2.i139.i, !dbg !14880
  %_86.i.2.i148.i = load float, ptr %717, align 4, !dbg !14880, !alias.scope !14835, !noalias !14855, !noundef !12
  %_89.i.2.i149.i = icmp samesign ult i64 %right27.i.2.i139.i, %_64.1.i35.i, !dbg !14882
  br i1 %_89.i.2.i149.i, label %bb33.i60.2.i150.i, label %panic34.i.i101.i, !dbg !14882

bb33.i60.2.i150.i:                                ; preds = %bb31.i.2.i147.i
  %718 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.2.i139.i, !dbg !14882
  %_88.i.2.i151.i = load float, ptr %718, align 4, !dbg !14882, !alias.scope !14833, !noalias !14854, !noundef !12
  %_68.i.3.i152.i = load i32, ptr %662, align 4, !dbg !14883, !alias.scope !14839, !noalias !14840, !noundef !12
  %_67.i52.3.i153.i = sub i32 %now.i.i49.i, %_68.i.3.i152.i, !dbg !14884
  %_66.i.3.i154.i = and i32 %_67.i52.3.i153.i, %_58.i39.i, !dbg !14886
  %_65.i.3.i155.i = zext i32 %_66.i.3.i154.i to i64, !dbg !14878
  %_64.i53.3.i156.i = shl nuw nsw i64 %_65.i.3.i155.i, 3, !dbg !14878
  %left25.i.3.i157.i = or disjoint i64 %_64.i53.3.i156.i, 3, !dbg !14878
  %_76.i54.3.i158.i = load i32, ptr %663, align 4, !dbg !14887, !alias.scope !14844, !noalias !14845, !noundef !12
  %_75.i.3.i159.i = sub i32 %now.i.i49.i, %_76.i54.3.i158.i, !dbg !14888
  %_74.i55.3.i160.i = and i32 %_75.i.3.i159.i, %_58.i39.i, !dbg !14890
  %_73.i56.3.i161.i = zext i32 %_74.i55.3.i160.i to i64, !dbg !14881
  %_72.i.3.i162.i = shl nuw nsw i64 %_73.i56.3.i161.i, 3, !dbg !14881
  %right27.i.3.i163.i = or disjoint i64 %_72.i.3.i162.i, 3, !dbg !14881
  %_81.i57.3.i164.i = icmp samesign ult i64 %left25.i.3.i157.i, %_64.1.i35.i, !dbg !14847
  br i1 %_81.i57.3.i164.i, label %bb27.i58.3.i165.i, label %panic28.i.i88.i, !dbg !14847

bb27.i58.3.i165.i:                                ; preds = %bb33.i60.2.i150.i
  %719 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.3.i157.i, !dbg !14847
  %_79.i.3.i166.i = load float, ptr %719, align 4, !dbg !14847, !alias.scope !14833, !noalias !14854, !noundef !12
  %_85.i.3.i167.i = icmp samesign ult i64 %left25.i.3.i157.i, %_66.1.i37.i, !dbg !14879
  br i1 %_85.i.3.i167.i, label %bb29.i59.3.i168.i, label %panic30.i.i93.i, !dbg !14879

bb29.i59.3.i168.i:                                ; preds = %bb27.i58.3.i165.i
  %720 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.3.i157.i, !dbg !14879
  %_83.i.3.i169.i = load float, ptr %720, align 4, !dbg !14879, !alias.scope !14835, !noalias !14855, !noundef !12
  %_87.i.3.i170.i = icmp samesign ult i64 %right27.i.3.i163.i, %_66.1.i37.i, !dbg !14880
  br i1 %_87.i.3.i170.i, label %bb31.i.3.i171.i, label %panic32.i.i97.i, !dbg !14880

bb31.i.3.i171.i:                                  ; preds = %bb29.i59.3.i168.i
  %721 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.3.i163.i, !dbg !14880
  %_86.i.3.i172.i = load float, ptr %721, align 4, !dbg !14880, !alias.scope !14835, !noalias !14855, !noundef !12
  %_89.i.3.i173.i = icmp samesign ult i64 %right27.i.3.i163.i, %_64.1.i35.i, !dbg !14882
  br i1 %_89.i.3.i173.i, label %bb33.i60.3.i174.i, label %panic34.i.i101.i, !dbg !14882

bb33.i60.3.i174.i:                                ; preds = %bb31.i.3.i171.i
  %722 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.3.i163.i, !dbg !14882
  %_88.i.3.i175.i = load float, ptr %722, align 4, !dbg !14882, !alias.scope !14833, !noalias !14854, !noundef !12
  %_68.i.4.i176.i = load i32, ptr %664, align 4, !dbg !14883, !alias.scope !14839, !noalias !14840, !noundef !12
  %_67.i52.4.i177.i = sub i32 %now.i.i49.i, %_68.i.4.i176.i, !dbg !14884
  %_66.i.4.i178.i = and i32 %_67.i52.4.i177.i, %_58.i39.i, !dbg !14886
  %_65.i.4.i179.i = zext i32 %_66.i.4.i178.i to i64, !dbg !14878
  %_64.i53.4.i180.i = shl nuw nsw i64 %_65.i.4.i179.i, 3, !dbg !14878
  %left25.i.4.i181.i = or disjoint i64 %_64.i53.4.i180.i, 4, !dbg !14878
  %_76.i54.4.i182.i = load i32, ptr %665, align 4, !dbg !14887, !alias.scope !14844, !noalias !14845, !noundef !12
  %_75.i.4.i183.i = sub i32 %now.i.i49.i, %_76.i54.4.i182.i, !dbg !14888
  %_74.i55.4.i184.i = and i32 %_75.i.4.i183.i, %_58.i39.i, !dbg !14890
  %_73.i56.4.i185.i = zext i32 %_74.i55.4.i184.i to i64, !dbg !14881
  %_72.i.4.i186.i = shl nuw nsw i64 %_73.i56.4.i185.i, 3, !dbg !14881
  %right27.i.4.i187.i = or disjoint i64 %_72.i.4.i186.i, 4, !dbg !14881
  %_81.i57.4.i188.i = icmp samesign ult i64 %left25.i.4.i181.i, %_64.1.i35.i, !dbg !14847
  br i1 %_81.i57.4.i188.i, label %bb27.i58.4.i189.i, label %panic28.i.i88.i, !dbg !14847

bb27.i58.4.i189.i:                                ; preds = %bb33.i60.3.i174.i
  %723 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.4.i181.i, !dbg !14847
  %_79.i.4.i190.i = load float, ptr %723, align 4, !dbg !14847, !alias.scope !14833, !noalias !14854, !noundef !12
  %_85.i.4.i191.i = icmp samesign ult i64 %left25.i.4.i181.i, %_66.1.i37.i, !dbg !14879
  br i1 %_85.i.4.i191.i, label %bb29.i59.4.i192.i, label %panic30.i.i93.i, !dbg !14879

bb29.i59.4.i192.i:                                ; preds = %bb27.i58.4.i189.i
  %724 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.4.i181.i, !dbg !14879
  %_83.i.4.i193.i = load float, ptr %724, align 4, !dbg !14879, !alias.scope !14835, !noalias !14855, !noundef !12
  %_87.i.4.i194.i = icmp samesign ult i64 %right27.i.4.i187.i, %_66.1.i37.i, !dbg !14880
  br i1 %_87.i.4.i194.i, label %bb31.i.4.i195.i, label %panic32.i.i97.i, !dbg !14880

bb31.i.4.i195.i:                                  ; preds = %bb29.i59.4.i192.i
  %725 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.4.i187.i, !dbg !14880
  %_86.i.4.i196.i = load float, ptr %725, align 4, !dbg !14880, !alias.scope !14835, !noalias !14855, !noundef !12
  %_89.i.4.i197.i = icmp samesign ult i64 %right27.i.4.i187.i, %_64.1.i35.i, !dbg !14882
  br i1 %_89.i.4.i197.i, label %bb33.i60.4.i198.i, label %panic34.i.i101.i, !dbg !14882

bb33.i60.4.i198.i:                                ; preds = %bb31.i.4.i195.i
  %726 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.4.i187.i, !dbg !14882
  %_88.i.4.i199.i = load float, ptr %726, align 4, !dbg !14882, !alias.scope !14833, !noalias !14854, !noundef !12
  %_68.i.5.i200.i = load i32, ptr %666, align 4, !dbg !14883, !alias.scope !14839, !noalias !14840, !noundef !12
  %_67.i52.5.i201.i = sub i32 %now.i.i49.i, %_68.i.5.i200.i, !dbg !14884
  %_66.i.5.i202.i = and i32 %_67.i52.5.i201.i, %_58.i39.i, !dbg !14886
  %_65.i.5.i203.i = zext i32 %_66.i.5.i202.i to i64, !dbg !14878
  %_64.i53.5.i204.i = shl nuw nsw i64 %_65.i.5.i203.i, 3, !dbg !14878
  %left25.i.5.i205.i = or disjoint i64 %_64.i53.5.i204.i, 5, !dbg !14878
  %_76.i54.5.i206.i = load i32, ptr %667, align 4, !dbg !14887, !alias.scope !14844, !noalias !14845, !noundef !12
  %_75.i.5.i207.i = sub i32 %now.i.i49.i, %_76.i54.5.i206.i, !dbg !14888
  %_74.i55.5.i208.i = and i32 %_75.i.5.i207.i, %_58.i39.i, !dbg !14890
  %_73.i56.5.i209.i = zext i32 %_74.i55.5.i208.i to i64, !dbg !14881
  %_72.i.5.i210.i = shl nuw nsw i64 %_73.i56.5.i209.i, 3, !dbg !14881
  %right27.i.5.i211.i = or disjoint i64 %_72.i.5.i210.i, 5, !dbg !14881
  %_81.i57.5.i212.i = icmp samesign ult i64 %left25.i.5.i205.i, %_64.1.i35.i, !dbg !14847
  br i1 %_81.i57.5.i212.i, label %bb27.i58.5.i213.i, label %panic28.i.i88.i, !dbg !14847

bb27.i58.5.i213.i:                                ; preds = %bb33.i60.4.i198.i
  %727 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.5.i205.i, !dbg !14847
  %_79.i.5.i214.i = load float, ptr %727, align 4, !dbg !14847, !alias.scope !14833, !noalias !14854, !noundef !12
  %_85.i.5.i215.i = icmp samesign ult i64 %left25.i.5.i205.i, %_66.1.i37.i, !dbg !14879
  br i1 %_85.i.5.i215.i, label %bb29.i59.5.i216.i, label %panic30.i.i93.i, !dbg !14879

bb29.i59.5.i216.i:                                ; preds = %bb27.i58.5.i213.i
  %728 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.5.i205.i, !dbg !14879
  %_83.i.5.i217.i = load float, ptr %728, align 4, !dbg !14879, !alias.scope !14835, !noalias !14855, !noundef !12
  %_87.i.5.i218.i = icmp samesign ult i64 %right27.i.5.i211.i, %_66.1.i37.i, !dbg !14880
  br i1 %_87.i.5.i218.i, label %bb31.i.5.i219.i, label %panic32.i.i97.i, !dbg !14880

bb31.i.5.i219.i:                                  ; preds = %bb29.i59.5.i216.i
  %729 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.5.i211.i, !dbg !14880
  %_86.i.5.i220.i = load float, ptr %729, align 4, !dbg !14880, !alias.scope !14835, !noalias !14855, !noundef !12
  %_89.i.5.i221.i = icmp samesign ult i64 %right27.i.5.i211.i, %_64.1.i35.i, !dbg !14882
  br i1 %_89.i.5.i221.i, label %bb33.i60.5.i222.i, label %panic34.i.i101.i, !dbg !14882

bb33.i60.5.i222.i:                                ; preds = %bb31.i.5.i219.i
  %730 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.5.i211.i, !dbg !14882
  %_88.i.5.i223.i = load float, ptr %730, align 4, !dbg !14882, !alias.scope !14833, !noalias !14854, !noundef !12
  %_68.i.6.i224.i = load i32, ptr %668, align 4, !dbg !14883, !alias.scope !14839, !noalias !14840, !noundef !12
  %_67.i52.6.i225.i = sub i32 %now.i.i49.i, %_68.i.6.i224.i, !dbg !14884
  %_66.i.6.i226.i = and i32 %_67.i52.6.i225.i, %_58.i39.i, !dbg !14886
  %_65.i.6.i227.i = zext i32 %_66.i.6.i226.i to i64, !dbg !14878
  %_64.i53.6.i228.i = shl nuw nsw i64 %_65.i.6.i227.i, 3, !dbg !14878
  %left25.i.6.i229.i = or disjoint i64 %_64.i53.6.i228.i, 6, !dbg !14878
  %_76.i54.6.i230.i = load i32, ptr %669, align 4, !dbg !14887, !alias.scope !14844, !noalias !14845, !noundef !12
  %_75.i.6.i231.i = sub i32 %now.i.i49.i, %_76.i54.6.i230.i, !dbg !14888
  %_74.i55.6.i232.i = and i32 %_75.i.6.i231.i, %_58.i39.i, !dbg !14890
  %_73.i56.6.i233.i = zext i32 %_74.i55.6.i232.i to i64, !dbg !14881
  %_72.i.6.i234.i = shl nuw nsw i64 %_73.i56.6.i233.i, 3, !dbg !14881
  %right27.i.6.i235.i = or disjoint i64 %_72.i.6.i234.i, 6, !dbg !14881
  %_81.i57.6.i236.i = icmp samesign ult i64 %left25.i.6.i229.i, %_64.1.i35.i, !dbg !14847
  br i1 %_81.i57.6.i236.i, label %bb27.i58.6.i237.i, label %panic28.i.i88.i, !dbg !14847

bb27.i58.6.i237.i:                                ; preds = %bb33.i60.5.i222.i
  %731 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.6.i229.i, !dbg !14847
  %_79.i.6.i238.i = load float, ptr %731, align 4, !dbg !14847, !alias.scope !14833, !noalias !14854, !noundef !12
  %_85.i.6.i239.i = icmp samesign ult i64 %left25.i.6.i229.i, %_66.1.i37.i, !dbg !14879
  br i1 %_85.i.6.i239.i, label %bb29.i59.6.i240.i, label %panic30.i.i93.i, !dbg !14879

bb29.i59.6.i240.i:                                ; preds = %bb27.i58.6.i237.i
  %732 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.6.i229.i, !dbg !14879
  %_83.i.6.i241.i = load float, ptr %732, align 4, !dbg !14879, !alias.scope !14835, !noalias !14855, !noundef !12
  %_87.i.6.i242.i = icmp samesign ult i64 %right27.i.6.i235.i, %_66.1.i37.i, !dbg !14880
  br i1 %_87.i.6.i242.i, label %bb31.i.6.i243.i, label %panic32.i.i97.i, !dbg !14880

bb31.i.6.i243.i:                                  ; preds = %bb29.i59.6.i240.i
  %733 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.6.i235.i, !dbg !14880
  %_86.i.6.i244.i = load float, ptr %733, align 4, !dbg !14880, !alias.scope !14835, !noalias !14855, !noundef !12
  %_89.i.6.i245.i = icmp samesign ult i64 %right27.i.6.i235.i, %_64.1.i35.i, !dbg !14882
  br i1 %_89.i.6.i245.i, label %bb33.i60.6.i246.i, label %panic34.i.i101.i, !dbg !14882

bb33.i60.6.i246.i:                                ; preds = %bb31.i.6.i243.i
  %734 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.6.i235.i, !dbg !14882
  %_88.i.6.i247.i = load float, ptr %734, align 4, !dbg !14882, !alias.scope !14833, !noalias !14854, !noundef !12
  %_68.i.7.i248.i = load i32, ptr %670, align 4, !dbg !14883, !alias.scope !14839, !noalias !14840, !noundef !12
  %_67.i52.7.i249.i = sub i32 %now.i.i49.i, %_68.i.7.i248.i, !dbg !14884
  %_66.i.7.i250.i = and i32 %_67.i52.7.i249.i, %_58.i39.i, !dbg !14886
  %_65.i.7.i251.i = zext i32 %_66.i.7.i250.i to i64, !dbg !14878
  %_64.i53.7.i252.i = shl nuw nsw i64 %_65.i.7.i251.i, 3, !dbg !14878
  %left25.i.7.i253.i = or disjoint i64 %_64.i53.7.i252.i, 7, !dbg !14878
  %_76.i54.7.i254.i = load i32, ptr %671, align 4, !dbg !14887, !alias.scope !14844, !noalias !14845, !noundef !12
  %_75.i.7.i255.i = sub i32 %now.i.i49.i, %_76.i54.7.i254.i, !dbg !14888
  %_74.i55.7.i256.i = and i32 %_75.i.7.i255.i, %_58.i39.i, !dbg !14890
  %_73.i56.7.i257.i = zext i32 %_74.i55.7.i256.i to i64, !dbg !14881
  %_72.i.7.i258.i = shl nuw nsw i64 %_73.i56.7.i257.i, 3, !dbg !14881
  %right27.i.7.i259.i = or disjoint i64 %_72.i.7.i258.i, 7, !dbg !14881
  %_81.i57.7.i260.i = icmp samesign ult i64 %left25.i.7.i253.i, %_64.1.i35.i, !dbg !14847
  br i1 %_81.i57.7.i260.i, label %bb27.i58.7.i261.i, label %panic28.i.i88.i, !dbg !14847

bb27.i58.7.i261.i:                                ; preds = %bb33.i60.6.i246.i
  %735 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.7.i253.i, !dbg !14847
  %_79.i.7.i262.i = load float, ptr %735, align 4, !dbg !14847, !alias.scope !14833, !noalias !14854, !noundef !12
  %_85.i.7.i263.i = icmp samesign ult i64 %left25.i.7.i253.i, %_66.1.i37.i, !dbg !14879
  br i1 %_85.i.7.i263.i, label %bb29.i59.7.i264.i, label %panic30.i.i93.i, !dbg !14879

bb29.i59.7.i264.i:                                ; preds = %bb27.i58.7.i261.i
  %736 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.7.i253.i, !dbg !14879
  %_83.i.7.i265.i = load float, ptr %736, align 4, !dbg !14879, !alias.scope !14835, !noalias !14855, !noundef !12
  %_87.i.7.i266.i = icmp samesign ult i64 %right27.i.7.i259.i, %_66.1.i37.i, !dbg !14880
  br i1 %_87.i.7.i266.i, label %bb31.i.7.i267.i, label %panic32.i.i97.i, !dbg !14880

bb31.i.7.i267.i:                                  ; preds = %bb29.i59.7.i264.i
  %_89.i.7.i268.i = icmp samesign ult i64 %right27.i.7.i259.i, %_64.1.i35.i, !dbg !14882
  br i1 %_89.i.7.i268.i, label %bb33.i60.7.i269.i, label %panic34.i.i101.i, !dbg !14882

bb33.i60.7.i269.i:                                ; preds = %bb31.i.7.i267.i
  %737 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.7.i259.i, !dbg !14880
  %_86.i.7.i270.i = load float, ptr %737, align 4, !dbg !14880, !alias.scope !14835, !noalias !14855, !noundef !12
  %738 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.7.i259.i, !dbg !14882
  %_88.i.7.i271.i = load float, ptr %738, align 4, !dbg !14882, !alias.scope !14833, !noalias !14854, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i272.i, !dbg !14864

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i272.i: ; preds = %bb33.i60.7.i269.i, %bb19.i.7.i455.i, %bb10.i79.7.i590.i
  %taps.i.sroa.103.0.i273.i = phi float [ %right_own.i.7.i592.i, %bb10.i79.7.i590.i ], [ %left_own16.i.7.i456.i, %bb19.i.7.i455.i ], [ %_88.i.7.i271.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.100.0.i274.i = phi float [ %right_own.i.6.i574.i, %bb10.i79.7.i590.i ], [ %left_own16.i.6.i438.i, %bb19.i.7.i455.i ], [ %_88.i.6.i247.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.97.0.i275.i = phi float [ %right_own.i.5.i556.i, %bb10.i79.7.i590.i ], [ %left_own16.i.5.i420.i, %bb19.i.7.i455.i ], [ %_88.i.5.i223.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.94.0.i276.i = phi float [ %right_own.i.4.i538.i, %bb10.i79.7.i590.i ], [ %left_own16.i.4.i402.i, %bb19.i.7.i455.i ], [ %_88.i.4.i199.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.91.0.i277.i = phi float [ %right_own.i.3.i520.i, %bb10.i79.7.i590.i ], [ %left_own16.i.3.i384.i, %bb19.i.7.i455.i ], [ %_88.i.3.i175.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.88.0.i278.i = phi float [ %right_own.i.2.i502.i, %bb10.i79.7.i590.i ], [ %left_own16.i.2.i366.i, %bb19.i.7.i455.i ], [ %_88.i.2.i151.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.85.0.i279.i = phi float [ %right_own.i.1.i484.i, %bb10.i79.7.i590.i ], [ %left_own16.i.1.i348.i, %bb19.i.7.i455.i ], [ %_88.i.1.i127.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.80.0.i280.i = phi float [ %right_own.i.i466.i, %bb10.i79.7.i590.i ], [ %left_own16.i.i330.i, %bb19.i.7.i455.i ], [ %_88.i.i103.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.77.0.i281.i = phi float [ %right_own.i.7.i592.i, %bb10.i79.7.i590.i ], [ %right_own18.i.7.i457.i, %bb19.i.7.i455.i ], [ %_86.i.7.i270.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.74.0.i282.i = phi float [ %right_own.i.6.i574.i, %bb10.i79.7.i590.i ], [ %right_own18.i.6.i439.i, %bb19.i.7.i455.i ], [ %_86.i.6.i244.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.71.0.i283.i = phi float [ %right_own.i.5.i556.i, %bb10.i79.7.i590.i ], [ %right_own18.i.5.i421.i, %bb19.i.7.i455.i ], [ %_86.i.5.i220.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.68.0.i284.i = phi float [ %right_own.i.4.i538.i, %bb10.i79.7.i590.i ], [ %right_own18.i.4.i403.i, %bb19.i.7.i455.i ], [ %_86.i.4.i196.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.65.0.i285.i = phi float [ %right_own.i.3.i520.i, %bb10.i79.7.i590.i ], [ %right_own18.i.3.i385.i, %bb19.i.7.i455.i ], [ %_86.i.3.i172.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.62.0.i286.i = phi float [ %right_own.i.2.i502.i, %bb10.i79.7.i590.i ], [ %right_own18.i.2.i367.i, %bb19.i.7.i455.i ], [ %_86.i.2.i148.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.59.0.i287.i = phi float [ %right_own.i.1.i484.i, %bb10.i79.7.i590.i ], [ %right_own18.i.1.i349.i, %bb19.i.7.i455.i ], [ %_86.i.1.i124.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.54.0.i288.i = phi float [ %right_own.i.i466.i, %bb10.i79.7.i590.i ], [ %right_own18.i.i331.i, %bb19.i.7.i455.i ], [ %_86.i.i99.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.51.0.i289.i = phi float [ %left_own.i.7.i591.i, %bb10.i79.7.i590.i ], [ %right_own18.i.7.i457.i, %bb19.i.7.i455.i ], [ %_83.i.7.i265.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.48.0.i290.i = phi float [ %left_own.i.6.i573.i, %bb10.i79.7.i590.i ], [ %right_own18.i.6.i439.i, %bb19.i.7.i455.i ], [ %_83.i.6.i241.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.45.0.i291.i = phi float [ %left_own.i.5.i555.i, %bb10.i79.7.i590.i ], [ %right_own18.i.5.i421.i, %bb19.i.7.i455.i ], [ %_83.i.5.i217.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.42.0.i292.i = phi float [ %left_own.i.4.i537.i, %bb10.i79.7.i590.i ], [ %right_own18.i.4.i403.i, %bb19.i.7.i455.i ], [ %_83.i.4.i193.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.39.0.i293.i = phi float [ %left_own.i.3.i519.i, %bb10.i79.7.i590.i ], [ %right_own18.i.3.i385.i, %bb19.i.7.i455.i ], [ %_83.i.3.i169.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.36.0.i294.i = phi float [ %left_own.i.2.i501.i, %bb10.i79.7.i590.i ], [ %right_own18.i.2.i367.i, %bb19.i.7.i455.i ], [ %_83.i.2.i145.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.33.0.i295.i = phi float [ %left_own.i.1.i483.i, %bb10.i79.7.i590.i ], [ %right_own18.i.1.i349.i, %bb19.i.7.i455.i ], [ %_83.i.1.i121.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.28.0.i296.i = phi float [ %left_own.i.i465.i, %bb10.i79.7.i590.i ], [ %right_own18.i.i331.i, %bb19.i.7.i455.i ], [ %_83.i.i95.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.25.0.i297.i = phi float [ %left_own.i.7.i591.i, %bb10.i79.7.i590.i ], [ %left_own16.i.7.i456.i, %bb19.i.7.i455.i ], [ %_79.i.7.i262.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.22.0.i298.i = phi float [ %left_own.i.6.i573.i, %bb10.i79.7.i590.i ], [ %left_own16.i.6.i438.i, %bb19.i.7.i455.i ], [ %_79.i.6.i238.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.19.0.i299.i = phi float [ %left_own.i.5.i555.i, %bb10.i79.7.i590.i ], [ %left_own16.i.5.i420.i, %bb19.i.7.i455.i ], [ %_79.i.5.i214.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.16.0.i300.i = phi float [ %left_own.i.4.i537.i, %bb10.i79.7.i590.i ], [ %left_own16.i.4.i402.i, %bb19.i.7.i455.i ], [ %_79.i.4.i190.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.13.0.i301.i = phi float [ %left_own.i.3.i519.i, %bb10.i79.7.i590.i ], [ %left_own16.i.3.i384.i, %bb19.i.7.i455.i ], [ %_79.i.3.i166.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.10.0.i302.i = phi float [ %left_own.i.2.i501.i, %bb10.i79.7.i590.i ], [ %left_own16.i.2.i366.i, %bb19.i.7.i455.i ], [ %_79.i.2.i142.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.7.0.i303.i = phi float [ %left_own.i.1.i483.i, %bb10.i79.7.i590.i ], [ %left_own16.i.1.i348.i, %bb19.i.7.i455.i ], [ %_79.i.1.i118.i, %bb33.i60.7.i269.i ], !dbg !14837
  %taps.i.sroa.0.0.i304.i = phi float [ %left_own.i.i465.i, %bb10.i79.7.i590.i ], [ %left_own16.i.i330.i, %bb19.i.7.i455.i ], [ %_79.i.i91.i, %bb33.i60.7.i269.i ], !dbg !14837
  %_41.i83.i.sroa.0.0.copyload.i305.i = load <8 x float>, ptr %638, align 32, !dbg !14891, !alias.scope !14636, !noalias !14903
  %739 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_41.i83.i.sroa.0.0.copyload.i305.i, <8 x float> zeroinitializer, i8 30), !dbg !14910
  %740 = bitcast <8 x float> %739 to <8 x i32>, !dbg !14916
  %741 = icmp slt <8 x i32> %740, zeroinitializer, !dbg !14920
  %742 = insertelement <8 x float> poison, float %taps.i.sroa.0.0.i304.i, i64 0, !dbg !14922
  %743 = insertelement <8 x float> %742, float %taps.i.sroa.7.0.i303.i, i64 1, !dbg !14922
  %744 = insertelement <8 x float> %743, float %taps.i.sroa.10.0.i302.i, i64 2, !dbg !14922
  %745 = insertelement <8 x float> %744, float %taps.i.sroa.13.0.i301.i, i64 3, !dbg !14922
  %746 = insertelement <8 x float> %745, float %taps.i.sroa.16.0.i300.i, i64 4, !dbg !14922
  %747 = insertelement <8 x float> %746, float %taps.i.sroa.19.0.i299.i, i64 5, !dbg !14922
  %748 = insertelement <8 x float> %747, float %taps.i.sroa.22.0.i298.i, i64 6, !dbg !14922
  %749 = insertelement <8 x float> %748, float %taps.i.sroa.25.0.i297.i, i64 7, !dbg !14922
  %750 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %749), !dbg !14926
  %751 = fmul <8 x float> %750, splat (float 5.000000e-01), !dbg !14932
  %752 = insertelement <8 x float> poison, float %taps.i.sroa.28.0.i296.i, i64 0, !dbg !14937
  %753 = insertelement <8 x float> %752, float %taps.i.sroa.33.0.i295.i, i64 1, !dbg !14937
  %754 = insertelement <8 x float> %753, float %taps.i.sroa.36.0.i294.i, i64 2, !dbg !14937
  %755 = insertelement <8 x float> %754, float %taps.i.sroa.39.0.i293.i, i64 3, !dbg !14937
  %756 = insertelement <8 x float> %755, float %taps.i.sroa.42.0.i292.i, i64 4, !dbg !14937
  %757 = insertelement <8 x float> %756, float %taps.i.sroa.45.0.i291.i, i64 5, !dbg !14937
  %758 = insertelement <8 x float> %757, float %taps.i.sroa.48.0.i290.i, i64 6, !dbg !14937
  %759 = insertelement <8 x float> %758, float %taps.i.sroa.51.0.i289.i, i64 7, !dbg !14937
  %760 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %759), !dbg !14942
  %761 = fmul <8 x float> %760, splat (float 5.000000e-01), !dbg !14948
  %762 = fadd <8 x float> %761, %751, !dbg !14953
  %_37.i87.i.sroa.0.0.copyload.i306.i = load <8 x float>, ptr %637, align 32, !dbg !14958, !alias.scope !14636, !noalias !14903
  %763 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_37.i87.i.sroa.0.0.copyload.i306.i, <8 x float> zeroinitializer, i8 30), !dbg !14959
  %764 = bitcast <8 x float> %763 to <8 x i32>, !dbg !14965
  %765 = icmp slt <8 x i32> %764, zeroinitializer, !dbg !14969
  %766 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %750, <8 x float> %760), !dbg !14971
  %767 = select <8 x i1> %765, <8 x float> %766, <8 x float> %750, !dbg !14969
  %768 = select <8 x i1> %741, <8 x float> %762, <8 x float> %767, !dbg !14920
  %769 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %768, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !14976
  %770 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %769, <8 x float> splat (float 0x3810000000000000)), !dbg !14981
  %771 = bitcast <8 x float> %770 to <4 x i64>, !dbg !14989
  %772 = and <4 x i64> %771, splat (i64 36028792732385279), !dbg !14990
  %773 = or disjoint <4 x i64> %772, splat (i64 4575657222473777152), !dbg !14995
  %774 = bitcast <4 x i64> %773 to <8 x float>, !dbg !14999
  %775 = fadd <8 x float> %774, splat (float -1.000000e+00), !dbg !15000
  %776 = fmul <8 x float> %775, splat (float 0x3F9B17A960000000), !dbg !15005
  %777 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %776, !dbg !15010
  %778 = fmul <8 x float> %775, %777, !dbg !15005
  %779 = fadd <8 x float> %778, splat (float 0xBFD1E3F400000000), !dbg !15010
  %780 = fmul <8 x float> %775, %779, !dbg !15005
  %781 = fadd <8 x float> %780, splat (float 0x3FDD544F20000000), !dbg !15010
  %782 = fmul <8 x float> %775, %781, !dbg !15005
  %783 = fadd <8 x float> %782, splat (float 0xBFE6FC2A60000000), !dbg !15010
  %784 = fmul <8 x float> %775, %783, !dbg !15005
  %785 = fadd <8 x float> %784, splat (float 0x3FF714B2A0000000), !dbg !15010
  %786 = bitcast <8 x float> %770 to <8 x i32>, !dbg !15015
  %_3.i845.i.i = lshr <8 x i32> %786, splat (i32 23), !dbg !15019
  %787 = or disjoint <8 x i32> %_3.i845.i.i, splat (i32 1258291200), !dbg !15020
  %788 = bitcast <8 x i32> %787 to <8 x float>, !dbg !15024
  %789 = fadd <8 x float> %788, splat (float 0xC160000FE0000000), !dbg !15025
  %hysteresis.i93.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %636, align 32, !dbg !15029, !alias.scope !14636, !noalias !15030
  %range.i94.i.sroa.0.0.copyload1540.i.i = load <8 x i32>, ptr %635, align 32, !dbg !15032, !alias.scope !14636, !noalias !15030
  %ratio.i95.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %634, align 32, !dbg !15033, !alias.scope !14636, !noalias !15030
  %threshold.i96.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %13, align 32, !dbg !15034, !alias.scope !14636, !noalias !15030
  %790 = fmul <8 x float> %775, %785, !dbg !15035
  %791 = fadd <8 x float> %789, %790, !dbg !15040
  %792 = fmul <8 x float> %791, splat (float 0x4018151820000000), !dbg !15045
  %793 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %792, <8 x float> splat (float 2.400000e+01)), !dbg !15050
  %794 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %793, <8 x float> splat (float -1.600000e+02)), !dbg !15055
  %_55.i69.i.sroa.0.0.copyload.i307.i = load <8 x float>, ptr %639, align 32, !dbg !15060, !alias.scope !14636, !noalias !15030
  %795 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_55.i69.i.sroa.0.0.copyload.i307.i, <8 x float> zeroinitializer, i8 30), !dbg !15062
  %796 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %794, <8 x float> %threshold.i96.i.sroa.0.0.copyload.i.i, i8 29), !dbg !15068
  %797 = fsub <8 x float> %threshold.i96.i.sroa.0.0.copyload.i.i, %hysteresis.i93.i.sroa.0.0.copyload.i.i, !dbg !15075
  %798 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %794, <8 x float> %797, i8 29), !dbg !15081
  %799 = bitcast <8 x float> %795 to <8 x i32>, !dbg !15087
  %800 = xor <8 x i32> %799, splat (i32 -1), !dbg !15094
  %801 = bitcast <8 x float> %796 to <8 x i32>, !dbg !15096
  %802 = and <8 x i32> %801, %800, !dbg !15100
  %803 = bitcast <8 x float> %798 to <8 x i32>, !dbg !15102
  %804 = and <8 x i32> %803, %799, !dbg !15107
  %805 = or <8 x i32> %804, %802, !dbg !15109
  %806 = xor <8 x i32> %803, splat (i32 -1), !dbg !15115
  %_67.i57.i.sroa.0.0.copyload.i308.i = load <8 x float>, ptr %640, align 32, !dbg !15123, !alias.scope !14636, !noalias !15030
  %807 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_67.i57.i.sroa.0.0.copyload.i308.i, <8 x float> zeroinitializer, i8 30), !dbg !15124
  %808 = bitcast <8 x float> %807 to <8 x i32>, !dbg !15130
  %809 = and <8 x i32> %806, %808, !dbg !15134
  %810 = and <8 x i32> %809, %799, !dbg !15134
  %811 = or <8 x i32> %810, %805, !dbg !15139
  %812 = icmp slt <8 x i32> %811, zeroinitializer, !dbg !15145
  %813 = select <8 x i1> %812, <8 x float> splat (float 1.000000e+00), <8 x float> zeroinitializer, !dbg !15145
  %_71.i53.i.sroa.0.0.copyload.i309.i = load <8 x float>, ptr %641, align 32, !dbg !15150, !alias.scope !14636, !noalias !14903
  %814 = fadd <8 x float> %_67.i57.i.sroa.0.0.copyload.i308.i, splat (float -1.000000e+00), !dbg !15152
  %815 = icmp slt <8 x i32> %810, zeroinitializer, !dbg !15157
  %816 = select <8 x i1> %815, <8 x float> %814, <8 x float> %_67.i57.i.sroa.0.0.copyload.i308.i, !dbg !15157
  %817 = icmp slt <8 x i32> %805, zeroinitializer, !dbg !15162
  %818 = select <8 x i1> %817, <8 x float> %_71.i53.i.sroa.0.0.copyload.i309.i, <8 x float> %816, !dbg !15162
  store <8 x float> %818, ptr %640, align 32, !dbg !15167, !alias.scope !14636, !noalias !15030
  store <8 x float> %813, ptr %639, align 32, !dbg !15168, !alias.scope !14636, !noalias !15030
  %_86.i38.i.sroa.0.0.copyload.i310.i = load <8 x float>, ptr %642, align 32, !dbg !15169, !alias.scope !14636, !noalias !15030
  %_88.i37.i.sroa.0.0.copyload.i311.i = load <8 x float>, ptr %643, align 32, !dbg !15172, !alias.scope !14636, !noalias !14903
  %_7.i743.i.i = load <8 x float>, ptr %629, align 32, !dbg !15173, !alias.scope !15175, !noalias !15178
  %819 = fadd <8 x float> %ratio.i95.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !15182
  %820 = fsub <8 x float> %794, %threshold.i96.i.sroa.0.0.copyload.i.i, !dbg !15187
  %821 = fmul <8 x float> %819, %820, !dbg !15192
  %822 = xor <8 x i32> %range.i94.i.sroa.0.0.copyload1540.i.i, splat (i32 -2147483648), !dbg !15197
  %823 = bitcast <8 x i32> %822 to <8 x float>, !dbg !15202
  %824 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %821, <8 x float> %823), !dbg !15203
  %825 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %824, <8 x float> zeroinitializer), !dbg !15208
  %826 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %813, <8 x float> zeroinitializer, i8 30), !dbg !15213
  %827 = bitcast <8 x float> %826 to <8 x i32>, !dbg !15219
  %828 = icmp slt <8 x i32> %827, zeroinitializer, !dbg !15223
  %829 = select <8 x i1> %828, <8 x float> zeroinitializer, <8 x float> %825, !dbg !15223
  %830 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %829, <8 x float> %_86.i38.i.sroa.0.0.copyload.i310.i, i8 30), !dbg !15225
  %831 = bitcast <8 x float> %830 to <8 x i32>, !dbg !15231
  %832 = icmp slt <8 x i32> %831, zeroinitializer, !dbg !15234
  %833 = select <8 x i1> %832, <8 x float> %_7.i743.i.i, <8 x float> %_88.i37.i.sroa.0.0.copyload.i311.i, !dbg !15234
  %834 = fsub <8 x float> %829, %_86.i38.i.sroa.0.0.copyload.i310.i, !dbg !15236
  %835 = fmul <8 x float> %834, %833, !dbg !15242
  %836 = fadd <8 x float> %_86.i38.i.sroa.0.0.copyload.i310.i, %835, !dbg !15247
  %837 = bitcast <8 x float> %836 to <8 x i32>, !dbg !15251
  %838 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %836), !dbg !15258
  %839 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %838, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !15260
  %840 = bitcast <8 x float> %839 to <8 x i32>, !dbg !15266
  %841 = xor <8 x i32> %840, splat (i32 -1), !dbg !15272
  %842 = and <8 x i32> %837, %841, !dbg !15274
  store <8 x i32> %842, ptr %642, align 32, !dbg !15278, !alias.scope !14636, !noalias !15030
  %843 = bitcast <8 x i32> %842 to <8 x float>, !dbg !15280
  %844 = fmul <8 x float> %843, splat (float 0x3FC542A5A0000000), !dbg !15281
  %845 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %844, <8 x float> splat (float -1.260000e+02)), !dbg !15288
  %846 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %845, <8 x float> splat (float 1.270000e+02)), !dbg !15294
  %847 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %846), !dbg !15299
  %848 = fsub <8 x float> %846, %847, !dbg !15304
  %849 = fmul <8 x float> %848, splat (float 0x3F5E974FA0000000), !dbg !15309
  %850 = fadd <8 x float> %849, splat (float 0x3F82778560000000), !dbg !15314
  %851 = fmul <8 x float> %848, %850, !dbg !15309
  %852 = fadd <8 x float> %851, splat (float 0x3FAC91CE60000000), !dbg !15314
  %853 = fmul <8 x float> %848, %852, !dbg !15309
  %854 = fadd <8 x float> %853, splat (float 0x3FCEBDB560000000), !dbg !15314
  %855 = fmul <8 x float> %848, %854, !dbg !15309
  %856 = fadd <8 x float> %855, splat (float 0x3FE62E4BA0000000), !dbg !15314
  %_98.i27.i.sroa.0.0.copyload.i312.i = load <8 x float>, ptr %644, align 32, !dbg !15319, !alias.scope !14636, !noalias !14903
  %857 = fmul <8 x float> %848, %856, !dbg !15321
  %858 = fadd <8 x float> %857, splat (float 1.000000e+00), !dbg !15326
  %859 = fadd <8 x float> %847, splat (float 0x4160000FE0000000), !dbg !15331
  %860 = bitcast <8 x float> %859 to <8 x i32>, !dbg !15336
  %_3.i846.i.i = shl <8 x i32> %860, splat (i32 23), !dbg !15340
  %861 = bitcast <8 x i32> %_3.i846.i.i to <8 x float>, !dbg !15341
  %862 = fmul <8 x float> %858, %861, !dbg !15343
  %863 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %843, <8 x float> zeroinitializer, i8 0), !dbg !15347
  %864 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_98.i27.i.sroa.0.0.copyload.i312.i, <8 x float> zeroinitializer, i8 30), !dbg !15353
  %865 = bitcast <8 x float> %863 to <8 x i32>, !dbg !15359
  %866 = bitcast <8 x float> %864 to <8 x i32>, !dbg !15359
  %867 = or <8 x i32> %866, %865, !dbg !15363
  %868 = fmul <8 x float> %lanes.i447.sroa.0.0.copyload.i.i, %862, !dbg !15365
  %869 = icmp slt <8 x i32> %867, zeroinitializer, !dbg !15371
  %870 = select <8 x i1> %869, <8 x float> %lanes.i447.sroa.0.0.copyload.i.i, <8 x float> %868, !dbg !15371
  %_41.i.i.sroa.0.0.copyload.i313.i = load <8 x float>, ptr %649, align 32, !dbg !15376, !alias.scope !14636, !noalias !15379
  %871 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_41.i.i.sroa.0.0.copyload.i313.i, <8 x float> zeroinitializer, i8 30), !dbg !15386
  %872 = bitcast <8 x float> %871 to <8 x i32>, !dbg !15392
  %873 = icmp slt <8 x i32> %872, zeroinitializer, !dbg !15396
  %874 = insertelement <8 x float> poison, float %taps.i.sroa.54.0.i288.i, i64 0, !dbg !15398
  %875 = insertelement <8 x float> %874, float %taps.i.sroa.59.0.i287.i, i64 1, !dbg !15398
  %876 = insertelement <8 x float> %875, float %taps.i.sroa.62.0.i286.i, i64 2, !dbg !15398
  %877 = insertelement <8 x float> %876, float %taps.i.sroa.65.0.i285.i, i64 3, !dbg !15398
  %878 = insertelement <8 x float> %877, float %taps.i.sroa.68.0.i284.i, i64 4, !dbg !15398
  %879 = insertelement <8 x float> %878, float %taps.i.sroa.71.0.i283.i, i64 5, !dbg !15398
  %880 = insertelement <8 x float> %879, float %taps.i.sroa.74.0.i282.i, i64 6, !dbg !15398
  %881 = insertelement <8 x float> %880, float %taps.i.sroa.77.0.i281.i, i64 7, !dbg !15398
  %882 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %881), !dbg !15403
  %883 = fmul <8 x float> %882, splat (float 5.000000e-01), !dbg !15409
  %884 = insertelement <8 x float> poison, float %taps.i.sroa.80.0.i280.i, i64 0, !dbg !15414
  %885 = insertelement <8 x float> %884, float %taps.i.sroa.85.0.i279.i, i64 1, !dbg !15414
  %886 = insertelement <8 x float> %885, float %taps.i.sroa.88.0.i278.i, i64 2, !dbg !15414
  %887 = insertelement <8 x float> %886, float %taps.i.sroa.91.0.i277.i, i64 3, !dbg !15414
  %888 = insertelement <8 x float> %887, float %taps.i.sroa.94.0.i276.i, i64 4, !dbg !15414
  %889 = insertelement <8 x float> %888, float %taps.i.sroa.97.0.i275.i, i64 5, !dbg !15414
  %890 = insertelement <8 x float> %889, float %taps.i.sroa.100.0.i274.i, i64 6, !dbg !15414
  %891 = insertelement <8 x float> %890, float %taps.i.sroa.103.0.i273.i, i64 7, !dbg !15414
  %892 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %891), !dbg !15419
  %893 = fmul <8 x float> %892, splat (float 5.000000e-01), !dbg !15425
  %894 = fadd <8 x float> %893, %883, !dbg !15430
  %_37.i.i.sroa.0.0.copyload.i314.i = load <8 x float>, ptr %648, align 32, !dbg !15435, !alias.scope !14636, !noalias !15379
  %895 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_37.i.i.sroa.0.0.copyload.i314.i, <8 x float> zeroinitializer, i8 30), !dbg !15436
  %896 = bitcast <8 x float> %895 to <8 x i32>, !dbg !15442
  %897 = icmp slt <8 x i32> %896, zeroinitializer, !dbg !15446
  %898 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %882, <8 x float> %892), !dbg !15448
  %899 = select <8 x i1> %897, <8 x float> %898, <8 x float> %882, !dbg !15446
  %900 = select <8 x i1> %873, <8 x float> %894, <8 x float> %899, !dbg !15396
  %901 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %900, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !15453
  %902 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %901, <8 x float> splat (float 0x3810000000000000)), !dbg !15458
  %903 = bitcast <8 x float> %902 to <4 x i64>, !dbg !15465
  %904 = and <4 x i64> %903, splat (i64 36028792732385279), !dbg !15466
  %905 = or disjoint <4 x i64> %904, splat (i64 4575657222473777152), !dbg !15471
  %906 = bitcast <4 x i64> %905 to <8 x float>, !dbg !15475
  %907 = fadd <8 x float> %906, splat (float -1.000000e+00), !dbg !15476
  %908 = fmul <8 x float> %907, splat (float 0x3F9B17A960000000), !dbg !15481
  %909 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %908, !dbg !15486
  %910 = fmul <8 x float> %907, %909, !dbg !15481
  %911 = fadd <8 x float> %910, splat (float 0xBFD1E3F400000000), !dbg !15486
  %912 = fmul <8 x float> %907, %911, !dbg !15481
  %913 = fadd <8 x float> %912, splat (float 0x3FDD544F20000000), !dbg !15486
  %914 = fmul <8 x float> %907, %913, !dbg !15481
  %915 = fadd <8 x float> %914, splat (float 0xBFE6FC2A60000000), !dbg !15486
  %916 = fmul <8 x float> %907, %915, !dbg !15481
  %917 = fadd <8 x float> %916, splat (float 0x3FF714B2A0000000), !dbg !15486
  %918 = bitcast <8 x float> %902 to <8 x i32>, !dbg !15491
  %_3.i849.i.i = lshr <8 x i32> %918, splat (i32 23), !dbg !15495
  %919 = or disjoint <8 x i32> %_3.i849.i.i, splat (i32 1258291200), !dbg !15496
  %920 = bitcast <8 x i32> %919 to <8 x float>, !dbg !15500
  %921 = fadd <8 x float> %920, splat (float 0xC160000FE0000000), !dbg !15501
  %hysteresis.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %647, align 32, !dbg !15505, !alias.scope !14636, !noalias !15506
  %range.i.i.sroa.0.0.copyload1550.i.i = load <8 x i32>, ptr %646, align 32, !dbg !15508, !alias.scope !14636, !noalias !15506
  %ratio.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %645, align 32, !dbg !15509, !alias.scope !14636, !noalias !15506
  %threshold.i.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %data.i.i.i23.i, align 32, !dbg !15510, !alias.scope !14636, !noalias !15506
  %922 = fmul <8 x float> %907, %917, !dbg !15511
  %923 = fadd <8 x float> %921, %922, !dbg !15516
  %924 = fmul <8 x float> %923, splat (float 0x4018151820000000), !dbg !15521
  %925 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %924, <8 x float> splat (float 2.400000e+01)), !dbg !15526
  %926 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %925, <8 x float> splat (float -1.600000e+02)), !dbg !15531
  %_55.i.i.sroa.0.0.copyload.i315.i = load <8 x float>, ptr %650, align 32, !dbg !15536, !alias.scope !14636, !noalias !15506
  %927 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_55.i.i.sroa.0.0.copyload.i315.i, <8 x float> zeroinitializer, i8 30), !dbg !15537
  %928 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %926, <8 x float> %threshold.i.i.sroa.0.0.copyload.i.i, i8 29), !dbg !15543
  %929 = fsub <8 x float> %threshold.i.i.sroa.0.0.copyload.i.i, %hysteresis.i.i.sroa.0.0.copyload.i.i, !dbg !15549
  %930 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %926, <8 x float> %929, i8 29), !dbg !15554
  %931 = bitcast <8 x float> %927 to <8 x i32>, !dbg !15560
  %932 = xor <8 x i32> %931, splat (i32 -1), !dbg !15566
  %933 = bitcast <8 x float> %928 to <8 x i32>, !dbg !15568
  %934 = and <8 x i32> %933, %932, !dbg !15572
  %935 = bitcast <8 x float> %930 to <8 x i32>, !dbg !15574
  %936 = and <8 x i32> %935, %931, !dbg !15578
  %937 = or <8 x i32> %936, %934, !dbg !15580
  %938 = xor <8 x i32> %935, splat (i32 -1), !dbg !15585
  %_67.i.i.sroa.0.0.copyload.i316.i = load <8 x float>, ptr %651, align 32, !dbg !15592, !alias.scope !14636, !noalias !15506
  %939 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_67.i.i.sroa.0.0.copyload.i316.i, <8 x float> zeroinitializer, i8 30), !dbg !15593
  %940 = bitcast <8 x float> %939 to <8 x i32>, !dbg !15599
  %941 = and <8 x i32> %938, %940, !dbg !15603
  %942 = and <8 x i32> %941, %931, !dbg !15603
  %943 = or <8 x i32> %942, %937, !dbg !15608
  %944 = icmp slt <8 x i32> %943, zeroinitializer, !dbg !15613
  %945 = select <8 x i1> %944, <8 x float> splat (float 1.000000e+00), <8 x float> zeroinitializer, !dbg !15613
  %_71.i.i.sroa.0.0.copyload.i317.i = load <8 x float>, ptr %652, align 32, !dbg !15618, !alias.scope !14636, !noalias !15379
  %946 = fadd <8 x float> %_67.i.i.sroa.0.0.copyload.i316.i, splat (float -1.000000e+00), !dbg !15619
  %947 = icmp slt <8 x i32> %942, zeroinitializer, !dbg !15624
  %948 = select <8 x i1> %947, <8 x float> %946, <8 x float> %_67.i.i.sroa.0.0.copyload.i316.i, !dbg !15624
  %949 = icmp slt <8 x i32> %937, zeroinitializer, !dbg !15629
  %950 = select <8 x i1> %949, <8 x float> %_71.i.i.sroa.0.0.copyload.i317.i, <8 x float> %948, !dbg !15629
  store <8 x float> %950, ptr %651, align 32, !dbg !15634, !alias.scope !14636, !noalias !15506
  store <8 x float> %945, ptr %650, align 32, !dbg !15635, !alias.scope !14636, !noalias !15506
  %_86.i.i.sroa.0.0.copyload.i318.i = load <8 x float>, ptr %653, align 32, !dbg !15636, !alias.scope !14636, !noalias !15506
  %_88.i.i.sroa.0.0.copyload.i319.i = load <8 x float>, ptr %654, align 32, !dbg !15637, !alias.scope !14636, !noalias !15379
  %_7.i711.i.i = load <8 x float>, ptr %_38.i33.i, align 32, !dbg !15638, !alias.scope !15640, !noalias !15643
  %951 = fadd <8 x float> %ratio.i.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !15647
  %952 = fsub <8 x float> %926, %threshold.i.i.sroa.0.0.copyload.i.i, !dbg !15652
  %953 = fmul <8 x float> %951, %952, !dbg !15657
  %954 = xor <8 x i32> %range.i.i.sroa.0.0.copyload1550.i.i, splat (i32 -2147483648), !dbg !15662
  %955 = bitcast <8 x i32> %954 to <8 x float>, !dbg !15667
  %956 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %953, <8 x float> %955), !dbg !15668
  %957 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %956, <8 x float> zeroinitializer), !dbg !15673
  %958 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %945, <8 x float> zeroinitializer, i8 30), !dbg !15678
  %959 = bitcast <8 x float> %958 to <8 x i32>, !dbg !15684
  %960 = icmp slt <8 x i32> %959, zeroinitializer, !dbg !15688
  %961 = select <8 x i1> %960, <8 x float> zeroinitializer, <8 x float> %957, !dbg !15688
  %962 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %961, <8 x float> %_86.i.i.sroa.0.0.copyload.i318.i, i8 30), !dbg !15690
  %963 = bitcast <8 x float> %962 to <8 x i32>, !dbg !15696
  %964 = icmp slt <8 x i32> %963, zeroinitializer, !dbg !15699
  %965 = select <8 x i1> %964, <8 x float> %_7.i711.i.i, <8 x float> %_88.i.i.sroa.0.0.copyload.i319.i, !dbg !15699
  %966 = fsub <8 x float> %961, %_86.i.i.sroa.0.0.copyload.i318.i, !dbg !15701
  %967 = fmul <8 x float> %966, %965, !dbg !15706
  %968 = fadd <8 x float> %_86.i.i.sroa.0.0.copyload.i318.i, %967, !dbg !15711
  %969 = bitcast <8 x float> %968 to <8 x i32>, !dbg !15715
  %970 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %968), !dbg !15721
  %971 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %970, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !15723
  %972 = bitcast <8 x float> %971 to <8 x i32>, !dbg !15729
  %973 = xor <8 x i32> %972, splat (i32 -1), !dbg !15735
  %974 = and <8 x i32> %969, %973, !dbg !15737
  store <8 x i32> %974, ptr %653, align 32, !dbg !15741, !alias.scope !14636, !noalias !15506
  %975 = bitcast <8 x i32> %974 to <8 x float>, !dbg !15742
  %976 = fmul <8 x float> %975, splat (float 0x3FC542A5A0000000), !dbg !15743
  %977 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %976, <8 x float> splat (float -1.260000e+02)), !dbg !15749
  %978 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %977, <8 x float> splat (float 1.270000e+02)), !dbg !15755
  %979 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %978), !dbg !15760
  %980 = fsub <8 x float> %978, %979, !dbg !15765
  %981 = fmul <8 x float> %980, splat (float 0x3F5E974FA0000000), !dbg !15770
  %982 = fadd <8 x float> %981, splat (float 0x3F82778560000000), !dbg !15775
  %983 = fmul <8 x float> %980, %982, !dbg !15770
  %984 = fadd <8 x float> %983, splat (float 0x3FAC91CE60000000), !dbg !15775
  %985 = fmul <8 x float> %980, %984, !dbg !15770
  %986 = fadd <8 x float> %985, splat (float 0x3FCEBDB560000000), !dbg !15775
  %987 = fmul <8 x float> %980, %986, !dbg !15770
  %988 = fadd <8 x float> %987, splat (float 0x3FE62E4BA0000000), !dbg !15775
  %989 = fmul <8 x float> %980, %988, !dbg !15780
  %990 = fadd <8 x float> %989, splat (float 1.000000e+00), !dbg !15785
  %991 = fadd <8 x float> %979, splat (float 0x4160000FE0000000), !dbg !15790
  %992 = bitcast <8 x float> %991 to <8 x i32>, !dbg !15795
  %_3.i850.i.i = shl <8 x i32> %992, splat (i32 23), !dbg !15799
  %993 = bitcast <8 x i32> %_3.i850.i.i to <8 x float>, !dbg !15800
  %994 = fmul <8 x float> %990, %993, !dbg !15802
  %995 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %975, <8 x float> zeroinitializer, i8 0), !dbg !15806
  %_98.i.i.sroa.0.0.copyload.i320.i = load <8 x float>, ptr %655, align 32, !dbg !15812, !alias.scope !14636, !noalias !15379
  %996 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_98.i.i.sroa.0.0.copyload.i320.i, <8 x float> zeroinitializer, i8 30), !dbg !15813
  %997 = bitcast <8 x float> %995 to <8 x i32>, !dbg !15819
  %998 = bitcast <8 x float> %996 to <8 x i32>, !dbg !15819
  %999 = or <8 x i32> %998, %997, !dbg !15823
  %1000 = fmul <8 x float> %lanes.i441.sroa.0.0.copyload.i.i, %994, !dbg !15825
  %1001 = icmp slt <8 x i32> %999, zeroinitializer, !dbg !15830
  %1002 = select <8 x i1> %1001, <8 x float> %lanes.i441.sroa.0.0.copyload.i.i, <8 x float> %1000, !dbg !15830
  store <8 x float> %870, ptr %_97.i.i55.i, align 4, !dbg !15835, !alias.scope !15841, !noalias !15845
  store <8 x float> %1002, ptr %_115.i.i61.i, align 4, !dbg !15849, !alias.scope !15854, !noalias !15858
  %exitcond1820.not.i.i = icmp eq i64 %672, %_26.i, !dbg !15862
  br i1 %exitcond1820.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKB1r_EB3_.exit.i, label %bb30.i.i46.i, !dbg !14693

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKB1r_EB3_.exit.i: ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i272.i
  %_82.i.i321.i = trunc i64 %_26.i to i32, !dbg !15865
  %_81.i.i322.i = add i32 %base.i.i45.i, %_82.i.i321.i, !dbg !15866
  store i32 %_81.i.i322.i, ptr %_57.i38.i, align 4, !dbg !15868, !alias.scope !14636, !noalias !14690
  br label %bb7.i, !dbg !15869

bb26.i:                                           ; preds = %bb25.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %626, i64 noundef range(i64 0, 2305843009213693952) %_38.1, i64 noundef range(i64 0, 2305843009213693952) %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6d4388bd1c2ee005f6a969a0e3ca0f4) #24, !dbg !15870, !noalias !12577
  unreachable, !dbg !15870

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9run_blockB2_.exit: ; preds = %bb1.backedge.i.i
  call void @llvm.lifetime.end.p0(ptr nonnull %iter.i.i), !dbg !15871, !noalias !14144
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %_0, ptr noundef nonnull align 8 dereferenceable(320) %reports, i64 320, i1 false), !dbg !15872
  %report.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 320, !dbg !15872
  store i8 %1, ptr %report.sroa.7.0._0.sroa_idx, align 8, !dbg !15872
  call void @llvm.lifetime.end.p0(ptr nonnull %reports), !dbg !15873
  br label %bb12, !dbg !12548

bb15:                                             ; preds = %bb4, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit
  %iter.sroa.0.0181 = phi i64 [ 0, %bb4 ], [ %1003, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit ]
  %1003 = add nuw nsw i64 %iter.sroa.0.0181, 1, !dbg !15874
  %exitcond.not = icmp eq i64 %iter.sroa.0.0181, %_36.1, !dbg !15880
  br i1 %exitcond.not, label %panic, label %bb7, !dbg !15880

bb12:                                             ; preds = %bb3, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9run_blockB2_.exit
  ret void, !dbg !12548

bb7:                                              ; preds = %bb15
  %1004 = getelementptr inbounds nuw i32, ptr %_36.0, i64 %iter.sroa.0.0181, !dbg !15880
  %_14 = load i32, ptr %1004, align 4, !dbg !15880, !noundef !12
  %start1 = zext i32 %_14 to i64, !dbg !15880
  %exitcond337.not = icmp eq i64 %iter.sroa.0.0181, %15, !dbg !15882
  br i1 %exitcond337.not, label %panic2, label %bb8, !dbg !15882

panic:                                            ; preds = %bb15
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_36.1, i64 noundef %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d54ae23796fd62146400a9560f58d006) #24, !dbg !15880
  unreachable, !dbg !15880

bb8:                                              ; preds = %bb7
  %1005 = getelementptr inbounds nuw i32, ptr %_36.0, i64 %1003, !dbg !15882
  %_18 = load i32, ptr %1005, align 4, !dbg !15882, !noundef !12
  %end = zext i32 %_18 to i64, !dbg !15882
  %_58 = icmp ult i32 %_18, %_14, !dbg !15884
  %_52.not = icmp ult i64 %_39.1, %end
  %or.cond24 = or i1 %_58, %_52.not, !dbg !15884
  br i1 %or.cond24, label %bb19, label %bb9, !dbg !15884, !prof !2561

panic2:                                           ; preds = %bb7
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %1003, i64 noundef %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d54ae23796fd62146400a9560f58d006) #24, !dbg !15882
  unreachable, !dbg !15882

bb19:                                             ; preds = %bb8
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1, i64 noundef %end, i64 noundef %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d54ae23796fd62146400a9560f58d006) #24, !dbg !15892
  unreachable, !dbg !15892

bb9:                                              ; preds = %bb8
  %_59 = sub nuw nsw i64 %end, %start1, !dbg !15893
  %_61 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_39.0, i64 %start1, !dbg !15894
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15898), !dbg !15901
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15902), !dbg !15901
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15904), !dbg !15901
  call void @llvm.lifetime.start.p0(ptr nonnull %pending.i), !dbg !15906, !noalias !15909
  store i32 0, ptr %pending.i, align 4, !noalias !15909
  store i32 0, ptr %_7.sroa.5124.0.pending.sroa_idx.i, align 4, !noalias !15909
  store i32 0, ptr %_7.sroa.6127.0.pending.sroa_idx.i, align 4, !noalias !15909
  store i32 0, ptr %_7.sroa.7130.0.pending.sroa_idx.i, align 4, !noalias !15909
  store i32 0, ptr %11, align 4, !noalias !15909
  store i32 0, ptr %_7.sroa.5124.0..sroa_idx.i, align 4, !noalias !15909
  store i32 0, ptr %_7.sroa.6127.0..sroa_idx.i, align 4, !noalias !15909
  store i32 0, ptr %_7.sroa.7130.0..sroa_idx.i, align 4, !noalias !15909
  %_106.idx.i = mul nuw nsw i64 %_59, 40, !dbg !15910
  %_106.i = getelementptr inbounds nuw i8, ptr %_61, i64 %_106.idx.i, !dbg !15910
  %_6.i.i8790.i = icmp eq i32 %_18, %_14, !dbg !15921
  br i1 %_6.i.i8790.i, label %bb36.preheader.i, label %bb4.lr.ph.lr.ph.i, !dbg !15926

bb4.lr.ph.lr.ph.i:                                ; preds = %bb9
  %_24 = getelementptr inbounds nuw %"effect_contract::ProcessReport", ptr %reports, i64 %iter.sroa.0.0181, !dbg !15927
  %1006 = getelementptr inbounds nuw i8, ptr %_24, i64 16
  %_31.i = load i32, ptr %12, align 4, !alias.scope !15898, !noalias !15928
  %_30.i = zext i32 %_31.i to i64
  %.promoted95.i = load i64, ptr %1006, align 8, !alias.scope !15904, !noalias !15929
  br label %bb4.lr.ph.i, !dbg !15926

bb4.lr.ph.i:                                      ; preds = %bb31.i, %bb4.lr.ph.lr.ph.i
  %.promoted97.i = phi i64 [ %.promoted95.i, %bb4.lr.ph.lr.ph.i ], [ %.promoted96.i, %bb31.i ]
  %last_order.sroa.3.0.ph94.i = phi i32 [ undef, %bb4.lr.ph.lr.ph.i ], [ %_127.0.i, %bb31.i ]
  %last_order.sroa.0.0.ph93.not.i = phi i1 [ true, %bb4.lr.ph.lr.ph.i ], [ false, %bb31.i ]
  %iter.sroa.0.0.ph92.i = phi ptr [ %_61, %bb4.lr.ph.lr.ph.i ], [ %_16.i.i.i, %bb31.i ]
  %iter.sroa.7.0.ph91.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %bb4.i10, !dbg !15926

bb36.preheader.i:                                 ; preds = %bb31.i, %bb35.i, %bb9
  %1007 = getelementptr inbounds nuw float, ptr %_84.i, i64 %iter.sroa.0.0181
  %1008 = getelementptr inbounds nuw float, ptr %words.i.i, i64 %iter.sroa.0.0181
  %1009 = getelementptr inbounds nuw float, ptr %words.i63.i, i64 %iter.sroa.0.0181
  %1010 = getelementptr inbounds nuw float, ptr %words.i64.i, i64 %iter.sroa.0.0181
  %1011 = getelementptr inbounds nuw float, ptr %words.i65.i, i64 %iter.sroa.0.0181
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, !dbg !15930

bb4.i10:                                          ; preds = %bb35.i, %bb4.lr.ph.i
  %.promoted96.i = phi i64 [ %.promoted97.i, %bb4.lr.ph.i ], [ %1058, %bb35.i ]
  %iter.sroa.0.089.i = phi ptr [ %iter.sroa.0.0.ph92.i, %bb4.lr.ph.i ], [ %_16.i.i.i, %bb35.i ]
  %iter.sroa.7.088.i = phi i64 [ %iter.sroa.7.0.ph91.i, %bb4.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 40, !dbg !15934
  %_9.0.i.i = add i64 %iter.sroa.7.088.i, 1, !dbg !15936
  %1012 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 32, !dbg !15937
  %_17.i = load i32, ptr %1012, align 8, !dbg !15937, !range !1335, !alias.scope !15902, !noalias !15939, !noundef !12
  switch i32 %_17.i, label %default.unreachable [
    i32 1, label %bb9.i12
    i32 2, label %bb7.i11
    i32 3, label %bb35.i
  ], !dbg !15940

bb36.loopexit.i:                                  ; preds = %bb46.us.3.i, %bb40.backedge.us.2.i
  %_6.i.i44.i = icmp eq i64 %iter1.sroa.0.0.add.i, 64, !dbg !15941
  br i1 %_6.i.i44.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, !dbg !15930

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i: ; preds = %bb36.loopexit.i, %bb36.preheader.i
  %iter1.sroa.0.0.idx107.i = phi i64 [ 0, %bb36.preheader.i ], [ %iter1.sroa.0.0.add.i, %bb36.loopexit.i ]
  %iter1.sroa.7.0106.i = phi i64 [ 0, %bb36.preheader.i ], [ %_9.0.i48.i, %bb36.loopexit.i ]
  %iter1.sroa.0.0.ptr.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 %iter1.sroa.0.0.idx107.i, !dbg !15941
  %iter1.sroa.0.0.add.i = add nuw nsw i64 %iter1.sroa.0.0.idx107.i, 32, !dbg !15943
  %_9.0.i48.i = add nuw nsw i64 %iter1.sroa.7.0106.i, 1, !dbg !15945
  %1013 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x8_::f32x8>", ptr %13, i64 %iter1.sroa.7.0106.i
  %1014 = load i32, ptr %iter1.sroa.0.0.ptr.i, align 4, !dbg !15946, !range !5852, !noalias !15909, !noundef !12
  %1015 = trunc nuw i32 %1014 to i1, !dbg !15950
  br i1 %1015, label %bb46.us.i, label %bb40.backedge.us.i, !dbg !15950

bb46.us.i:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %1016 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 4, !dbg !15946
  %value.us.i = load float, ptr %1016, align 4, !dbg !15951, !noalias !15909, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %_84.i), !dbg !15952, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_84.i, ptr noundef nonnull align 32 dereferenceable(32) %1013, i64 32, i1 false), !dbg !15952, !noalias !15928
  %_0.i.us.i = load float, ptr %1007, align 4, !dbg !15955, !alias.scope !15957, !noalias !15909, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_84.i), !dbg !15960, !noalias !15909
  %1017 = getelementptr inbounds nuw i8, ptr %1013, i64 32, !dbg !15961
  %1018 = getelementptr inbounds nuw i8, ptr %1013, i64 64, !dbg !15962
  %1019 = getelementptr inbounds nuw i8, ptr %1013, i64 96, !dbg !15963
  %_148.us.i = bitcast float %_0.i.us.i to i32, !dbg !15964
  %_150.us.i = bitcast float %value.us.i to i32, !dbg !15972
  %_149.us.i = icmp ne i32 %_148.us.i, %_150.us.i, !dbg !15975
  %1020 = icmp eq i32 %_148.us.i, -2147483648
  %or.cond.us.i = or i1 %_149.us.i, %1020, !dbg !15975
  %1021 = tail call float @llvm.fabs.f32(float %_0.i.us.i)
  %_144.us.i = fcmp ueq float %1021, 0x7FF0000000000000
  %or.cond40.us.i = or i1 %_144.us.i, %or.cond.us.i, !dbg !15975
  %_146.us.i = fsub float %value.us.i, %_0.i.us.i, !dbg !15975
  %1022 = fmul float %_146.us.i, 1.562500e-02, !dbg !15975
  %ramp.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %_0.i.us.i, float %value.us.i, !dbg !15975
  %ramp4.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %1022, float 0.000000e+00, !dbg !15975
  %ramp5.sroa.0.0.us.i = select i1 %or.cond40.us.i, float 6.400000e+01, float 0.000000e+00, !dbg !15975
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !15976, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, ptr noundef nonnull align 32 dereferenceable(32) %1013, i64 32, i1 false), !dbg !15976, !noalias !15928
  store float %ramp.sroa.0.0.us.i, ptr %1008, align 4, !dbg !15978, !noalias !15979
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1013, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, i64 32, i1 false), !dbg !15982, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !15983, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i63.i), !dbg !15984, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, ptr noundef nonnull align 32 dereferenceable(32) %1017, i64 32, i1 false), !dbg !15984, !noalias !15928
  store float %value.us.i, ptr %1009, align 4, !dbg !15986, !noalias !15987
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1017, ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, i64 32, i1 false), !dbg !15990, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i63.i), !dbg !15991, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i64.i), !dbg !15992, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, ptr noundef nonnull align 32 dereferenceable(32) %1018, i64 32, i1 false), !dbg !15992, !noalias !15928
  store float %ramp4.sroa.0.0.us.i, ptr %1010, align 4, !dbg !15994, !noalias !15995
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1018, ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, i64 32, i1 false), !dbg !15998, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i64.i), !dbg !15999, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i65.i), !dbg !16000, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, ptr noundef nonnull align 32 dereferenceable(32) %1019, i64 32, i1 false), !dbg !16000, !noalias !15928
  store float %ramp5.sroa.0.0.us.i, ptr %1011, align 4, !dbg !16002, !noalias !16003
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1019, ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, i64 32, i1 false), !dbg !16006, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i65.i), !dbg !16007, !noalias !15909
  store i32 64, ptr %14, align 4, !dbg !16008, !alias.scope !15898, !noalias !15928
  br label %bb40.backedge.us.i, !dbg !16009

bb40.backedge.us.i:                               ; preds = %bb46.us.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %iter2.sroa.0.0.ptr102.us.1.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 8, !dbg !16010
  %1023 = load i32, ptr %iter2.sroa.0.0.ptr102.us.1.i, align 4, !dbg !15946, !range !5852, !noalias !15909, !noundef !12
  %1024 = trunc nuw i32 %1023 to i1, !dbg !15950
  br i1 %1024, label %bb46.us.1.i, label %bb40.backedge.us.1.i, !dbg !15950

bb46.us.1.i:                                      ; preds = %bb40.backedge.us.i
  %1025 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 12, !dbg !15946
  %value.us.1.i = load float, ptr %1025, align 4, !dbg !15951, !noalias !15909, !noundef !12
  %slot.us.1.i = getelementptr inbounds nuw i8, ptr %1013, i64 128, !dbg !16014
  call void @llvm.lifetime.start.p0(ptr nonnull %_84.i), !dbg !15952, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_84.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.1.i, i64 32, i1 false), !dbg !15952, !noalias !15928
  %_0.i.us.1.i = load float, ptr %1007, align 4, !dbg !15955, !alias.scope !15957, !noalias !15909, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_84.i), !dbg !15960, !noalias !15909
  %1026 = getelementptr inbounds nuw i8, ptr %1013, i64 160, !dbg !15961
  %1027 = getelementptr inbounds nuw i8, ptr %1013, i64 192, !dbg !15962
  %1028 = getelementptr inbounds nuw i8, ptr %1013, i64 224, !dbg !15963
  %_148.us.1.i = bitcast float %_0.i.us.1.i to i32, !dbg !15964
  %_150.us.1.i = bitcast float %value.us.1.i to i32, !dbg !15972
  %_149.us.1.i = icmp ne i32 %_148.us.1.i, %_150.us.1.i, !dbg !15975
  %1029 = icmp eq i32 %_148.us.1.i, -2147483648
  %or.cond.us.1.i = or i1 %_149.us.1.i, %1029, !dbg !15975
  %1030 = tail call float @llvm.fabs.f32(float %_0.i.us.1.i)
  %_144.us.1.i = fcmp ueq float %1030, 0x7FF0000000000000
  %or.cond40.us.1.i = or i1 %_144.us.1.i, %or.cond.us.1.i, !dbg !15975
  %_146.us.1.i = fsub float %value.us.1.i, %_0.i.us.1.i, !dbg !15975
  %1031 = fmul float %_146.us.1.i, 1.562500e-02, !dbg !15975
  %ramp.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %_0.i.us.1.i, float %value.us.1.i, !dbg !15975
  %ramp4.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %1031, float 0.000000e+00, !dbg !15975
  %ramp5.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !15975
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !15976, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.1.i, i64 32, i1 false), !dbg !15976, !noalias !15928
  store float %ramp.sroa.0.0.us.1.i, ptr %1008, align 4, !dbg !15978, !noalias !15979
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %slot.us.1.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, i64 32, i1 false), !dbg !15982, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !15983, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i63.i), !dbg !15984, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, ptr noundef nonnull align 32 dereferenceable(32) %1026, i64 32, i1 false), !dbg !15984, !noalias !15928
  store float %value.us.1.i, ptr %1009, align 4, !dbg !15986, !noalias !15987
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1026, ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, i64 32, i1 false), !dbg !15990, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i63.i), !dbg !15991, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i64.i), !dbg !15992, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, ptr noundef nonnull align 32 dereferenceable(32) %1027, i64 32, i1 false), !dbg !15992, !noalias !15928
  store float %ramp4.sroa.0.0.us.1.i, ptr %1010, align 4, !dbg !15994, !noalias !15995
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1027, ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, i64 32, i1 false), !dbg !15998, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i64.i), !dbg !15999, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i65.i), !dbg !16000, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, ptr noundef nonnull align 32 dereferenceable(32) %1028, i64 32, i1 false), !dbg !16000, !noalias !15928
  store float %ramp5.sroa.0.0.us.1.i, ptr %1011, align 4, !dbg !16002, !noalias !16003
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1028, ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, i64 32, i1 false), !dbg !16006, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i65.i), !dbg !16007, !noalias !15909
  store i32 64, ptr %14, align 4, !dbg !16008, !alias.scope !15898, !noalias !15928
  br label %bb40.backedge.us.1.i, !dbg !16009

bb40.backedge.us.1.i:                             ; preds = %bb46.us.1.i, %bb40.backedge.us.i
  %iter2.sroa.0.0.ptr102.us.2.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 16, !dbg !16010
  %1032 = load i32, ptr %iter2.sroa.0.0.ptr102.us.2.i, align 4, !dbg !15946, !range !5852, !noalias !15909, !noundef !12
  %1033 = trunc nuw i32 %1032 to i1, !dbg !15950
  br i1 %1033, label %bb46.us.2.i, label %bb40.backedge.us.2.i, !dbg !15950

bb46.us.2.i:                                      ; preds = %bb40.backedge.us.1.i
  %1034 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 20, !dbg !15946
  %value.us.2.i = load float, ptr %1034, align 4, !dbg !15951, !noalias !15909, !noundef !12
  %slot.us.2.i = getelementptr inbounds nuw i8, ptr %1013, i64 256, !dbg !16014
  call void @llvm.lifetime.start.p0(ptr nonnull %_84.i), !dbg !15952, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_84.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.2.i, i64 32, i1 false), !dbg !15952, !noalias !15928
  %_0.i.us.2.i = load float, ptr %1007, align 4, !dbg !15955, !alias.scope !15957, !noalias !15909, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_84.i), !dbg !15960, !noalias !15909
  %1035 = getelementptr inbounds nuw i8, ptr %1013, i64 288, !dbg !15961
  %1036 = getelementptr inbounds nuw i8, ptr %1013, i64 320, !dbg !15962
  %1037 = getelementptr inbounds nuw i8, ptr %1013, i64 352, !dbg !15963
  %_148.us.2.i = bitcast float %_0.i.us.2.i to i32, !dbg !15964
  %_150.us.2.i = bitcast float %value.us.2.i to i32, !dbg !15972
  %_149.us.2.i = icmp ne i32 %_148.us.2.i, %_150.us.2.i, !dbg !15975
  %1038 = icmp eq i32 %_148.us.2.i, -2147483648
  %or.cond.us.2.i = or i1 %_149.us.2.i, %1038, !dbg !15975
  %1039 = tail call float @llvm.fabs.f32(float %_0.i.us.2.i)
  %_144.us.2.i = fcmp ueq float %1039, 0x7FF0000000000000
  %or.cond40.us.2.i = or i1 %_144.us.2.i, %or.cond.us.2.i, !dbg !15975
  %_146.us.2.i = fsub float %value.us.2.i, %_0.i.us.2.i, !dbg !15975
  %1040 = fmul float %_146.us.2.i, 1.562500e-02, !dbg !15975
  %ramp.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %_0.i.us.2.i, float %value.us.2.i, !dbg !15975
  %ramp4.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %1040, float 0.000000e+00, !dbg !15975
  %ramp5.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float 6.400000e+01, float 0.000000e+00, !dbg !15975
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !15976, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.2.i, i64 32, i1 false), !dbg !15976, !noalias !15928
  store float %ramp.sroa.0.0.us.2.i, ptr %1008, align 4, !dbg !15978, !noalias !15979
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %slot.us.2.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, i64 32, i1 false), !dbg !15982, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !15983, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i63.i), !dbg !15984, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, ptr noundef nonnull align 32 dereferenceable(32) %1035, i64 32, i1 false), !dbg !15984, !noalias !15928
  store float %value.us.2.i, ptr %1009, align 4, !dbg !15986, !noalias !15987
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1035, ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, i64 32, i1 false), !dbg !15990, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i63.i), !dbg !15991, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i64.i), !dbg !15992, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, ptr noundef nonnull align 32 dereferenceable(32) %1036, i64 32, i1 false), !dbg !15992, !noalias !15928
  store float %ramp4.sroa.0.0.us.2.i, ptr %1010, align 4, !dbg !15994, !noalias !15995
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1036, ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, i64 32, i1 false), !dbg !15998, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i64.i), !dbg !15999, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i65.i), !dbg !16000, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, ptr noundef nonnull align 32 dereferenceable(32) %1037, i64 32, i1 false), !dbg !16000, !noalias !15928
  store float %ramp5.sroa.0.0.us.2.i, ptr %1011, align 4, !dbg !16002, !noalias !16003
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1037, ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, i64 32, i1 false), !dbg !16006, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i65.i), !dbg !16007, !noalias !15909
  store i32 64, ptr %14, align 4, !dbg !16008, !alias.scope !15898, !noalias !15928
  br label %bb40.backedge.us.2.i, !dbg !16009

bb40.backedge.us.2.i:                             ; preds = %bb46.us.2.i, %bb40.backedge.us.1.i
  %iter2.sroa.0.0.ptr102.us.3.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 24, !dbg !16010
  %1041 = load i32, ptr %iter2.sroa.0.0.ptr102.us.3.i, align 4, !dbg !15946, !range !5852, !noalias !15909, !noundef !12
  %1042 = trunc nuw i32 %1041 to i1, !dbg !15950
  br i1 %1042, label %bb46.us.3.i, label %bb36.loopexit.i, !dbg !15950

bb46.us.3.i:                                      ; preds = %bb40.backedge.us.2.i
  %1043 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 28, !dbg !15946
  %value.us.3.i = load float, ptr %1043, align 4, !dbg !15951, !noalias !15909, !noundef !12
  %slot.us.3.i = getelementptr inbounds nuw i8, ptr %1013, i64 384, !dbg !16014
  call void @llvm.lifetime.start.p0(ptr nonnull %_84.i), !dbg !15952, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_84.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.3.i, i64 32, i1 false), !dbg !15952, !noalias !15928
  %_0.i.us.3.i = load float, ptr %1007, align 4, !dbg !15955, !alias.scope !15957, !noalias !15909, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_84.i), !dbg !15960, !noalias !15909
  %1044 = getelementptr inbounds nuw i8, ptr %1013, i64 416, !dbg !15961
  %1045 = getelementptr inbounds nuw i8, ptr %1013, i64 448, !dbg !15962
  %1046 = getelementptr inbounds nuw i8, ptr %1013, i64 480, !dbg !15963
  %_148.us.3.i = bitcast float %_0.i.us.3.i to i32, !dbg !15964
  %_150.us.3.i = bitcast float %value.us.3.i to i32, !dbg !15972
  %_149.us.3.i = icmp ne i32 %_148.us.3.i, %_150.us.3.i, !dbg !15975
  %1047 = icmp eq i32 %_148.us.3.i, -2147483648
  %or.cond.us.3.i = or i1 %_149.us.3.i, %1047, !dbg !15975
  %1048 = tail call float @llvm.fabs.f32(float %_0.i.us.3.i)
  %_144.us.3.i = fcmp ueq float %1048, 0x7FF0000000000000
  %or.cond40.us.3.i = or i1 %_144.us.3.i, %or.cond.us.3.i, !dbg !15975
  %_146.us.3.i = fsub float %value.us.3.i, %_0.i.us.3.i, !dbg !15975
  %1049 = fmul float %_146.us.3.i, 1.562500e-02, !dbg !15975
  %ramp.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %_0.i.us.3.i, float %value.us.3.i, !dbg !15975
  %ramp4.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %1049, float 0.000000e+00, !dbg !15975
  %ramp5.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float 6.400000e+01, float 0.000000e+00, !dbg !15975
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !15976, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.3.i, i64 32, i1 false), !dbg !15976, !noalias !15928
  store float %ramp.sroa.0.0.us.3.i, ptr %1008, align 4, !dbg !15978, !noalias !15979
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %slot.us.3.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, i64 32, i1 false), !dbg !15982, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !15983, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i63.i), !dbg !15984, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, ptr noundef nonnull align 32 dereferenceable(32) %1044, i64 32, i1 false), !dbg !15984, !noalias !15928
  store float %value.us.3.i, ptr %1009, align 4, !dbg !15986, !noalias !15987
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1044, ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, i64 32, i1 false), !dbg !15990, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i63.i), !dbg !15991, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i64.i), !dbg !15992, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, ptr noundef nonnull align 32 dereferenceable(32) %1045, i64 32, i1 false), !dbg !15992, !noalias !15928
  store float %ramp4.sroa.0.0.us.3.i, ptr %1010, align 4, !dbg !15994, !noalias !15995
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1045, ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, i64 32, i1 false), !dbg !15998, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i64.i), !dbg !15999, !noalias !15909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i65.i), !dbg !16000, !noalias !15909
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, ptr noundef nonnull align 32 dereferenceable(32) %1046, i64 32, i1 false), !dbg !16000, !noalias !15928
  store float %ramp5.sroa.0.0.us.3.i, ptr %1011, align 4, !dbg !16002, !noalias !16003
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %1046, ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, i64 32, i1 false), !dbg !16006, !noalias !15928
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i65.i), !dbg !16007, !noalias !15909
  store i32 64, ptr %14, align 4, !dbg !16008, !alias.scope !15898, !noalias !15928
  br label %bb36.loopexit.i, !dbg !16009

bb7.i11:                                          ; preds = %bb4.i10
  br label %bb9.i12, !dbg !16015

bb9.i12:                                          ; preds = %bb7.i11, %bb4.i10
  %channel.sroa.0.0.sroa.phi28.i = phi ptr [ %11, %bb7.i11 ], [ %pending.i, %bb4.i10 ], !dbg !16016
  %channel.sroa.0.0.i = phi i32 [ 1, %bb7.i11 ], [ 0, %bb4.i10 ], !dbg !16016
  %1050 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 16, !dbg !16017
  %_21.i = load i32, ptr %1050, align 8, !dbg !16017, !alias.scope !15902, !noalias !15939, !noundef !12
  %parameter_index.i = zext i32 %_21.i to i64, !dbg !16017
  %_121.1.i = icmp slt i32 %_21.i, 0, !dbg !16019
  br i1 %_121.1.i, label %bb35.i, label %bb59.i, !dbg !16025, !prof !180

bb59.i:                                           ; preds = %bb9.i12
  %_121.0.i = shl nuw i32 %_21.i, 1, !dbg !16019
  %_127.0.i = or disjoint i32 %_121.0.i, %channel.sroa.0.0.i, !dbg !16029
  %_29.i = icmp ult i64 %iter.sroa.7.088.i, %_30.i, !dbg !16037
  %_32.i = icmp samesign ult i32 %_21.i, 4
  %or.cond16.i = and i1 %_29.i, %_32.i, !dbg !16037
  br i1 %or.cond16.i, label %bb11.i, label %bb35.i, !dbg !16037

bb11.i:                                           ; preds = %bb59.i
  %1051 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 28, !dbg !16039
  %_130.i = load i32, ptr %1051, align 4, !dbg !16039, !range !6307, !alias.scope !15902, !noalias !15939, !noundef !12
  %1052 = icmp eq i32 %_130.i, 1, !dbg !16042
  br i1 %1052, label %bb12.i13, label %bb35.i, !dbg !16042

bb12.i13:                                         ; preds = %bb11.i
  %_34.i = load i64, ptr %iter.sroa.0.089.i, align 8, !dbg !16043, !alias.scope !15902, !noalias !15939, !noundef !12
  %_33.i = icmp eq i64 %_34.i, %_23, !dbg !16043
  br i1 %_33.i, label %bb13.i, label %bb35.i, !dbg !16043

bb13.i:                                           ; preds = %bb12.i13
  %1053 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 8, !dbg !16044
  %_36.i = load i64, ptr %1053, align 8, !dbg !16044, !alias.scope !15902, !noalias !15939, !noundef !12
  %_35.i = icmp eq i64 %_36.i, %_23, !dbg !16044
  br i1 %_35.i, label %bb14.i14, label %bb35.i, !dbg !16044

bb14.i14:                                         ; preds = %bb13.i
  %1054 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 20, !dbg !16045
  %_39.i = load float, ptr %1054, align 4, !dbg !16045, !alias.scope !15902, !noalias !15939, !noundef !12
  %_38.i = bitcast float %_39.i to i32, !dbg !16046
  %1055 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 24, !dbg !16048
  %_4139.i = load i32, ptr %1055, align 8, !dbg !16048, !alias.scope !15902, !noalias !15939, !noundef !12
  %_37.i = icmp eq i32 %_4139.i, %_38.i, !dbg !16045
  br i1 %_37.i, label %bb16.i15, label %bb35.i, !dbg !16045

bb16.i15:                                         ; preds = %bb14.i14
  %_43.i = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_d0058eaee52f1ba8b2e5577cef0c3c7f, i64 %parameter_index.i, !dbg !16049
; call effect_runtime::params::parameter_value_valid
  %_42.i = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_43.i, float noundef %_39.i) #23, !dbg !16050, !noalias !15909
  %_45.i = icmp ugt i32 %_127.0.i, %last_order.sroa.3.0.ph94.i
  %or.cond17.not.not108.i = select i1 %last_order.sroa.0.0.ph93.not.i, i1 true, i1 %_45.i, !dbg !16050
  %or.cond41.not.i = select i1 %_42.i, i1 %or.cond17.not.not108.i, i1 false, !dbg !16050
  br i1 %or.cond41.not.i, label %bb29.i, label %bb35.i, !dbg !16050

bb29.i:                                           ; preds = %bb16.i15
  %_47.i = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i, !dbg !16051
  %1056 = load i32, ptr %_47.i, align 4, !dbg !16052, !range !5852, !noalias !15909, !noundef !12
  %_133.not.i = icmp eq i32 %1056, 0, !dbg !16059
  br i1 %_133.not.i, label %bb31.i, label %bb35.i, !dbg !16060

bb31.i:                                           ; preds = %bb29.i
  %_47.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i
  %_135.i = fcmp oeq float %_39.i, 0.000000e+00, !dbg !16062
  %_55.sroa.0.0.i = select i1 %_135.i, float 0.000000e+00, float %_39.i, !dbg !16062
  store i32 1, ptr %_47.i.le, align 4, !dbg !16065, !noalias !15909
  %1057 = getelementptr inbounds nuw i8, ptr %_47.i.le, i64 4, !dbg !16065
  store float %_55.sroa.0.0.i, ptr %1057, align 4, !dbg !16065, !noalias !15909
  %_6.i.i87.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !15921
  br i1 %_6.i.i87.i, label %bb36.preheader.i, label %bb4.lr.ph.i, !dbg !15926

bb35.i:                                           ; preds = %bb29.i, %bb16.i15, %bb14.i14, %bb13.i, %bb12.i13, %bb11.i, %bb59.i, %bb9.i12, %bb4.i10
  %1058 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted96.i, i64 1), !dbg !16066
  store i64 %1058, ptr %1006, align 8, !dbg !16016, !alias.scope !15904, !noalias !15929
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !15921
  br i1 %_6.i.i.i, label %bb36.preheader.i, label %bb4.i10, !dbg !15926

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit: ; preds = %bb36.loopexit.i
  call void @llvm.lifetime.end.p0(ptr nonnull %pending.i), !dbg !16069, !noalias !15909
  %exitcond338.not = icmp eq i64 %1003, 8, !dbg !16070
  br i1 %exitcond338.not, label %bb16, label %bb15, !dbg !12551
}
