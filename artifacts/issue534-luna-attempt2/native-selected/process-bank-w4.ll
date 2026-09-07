define void @_RNvXsd_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank(ptr dead_on_unwind noalias noundef writable writeonly sret([328 x i8]) align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef align 16 dereferenceable(1840) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(112) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !8399 {
start:
  %words.i69.i = alloca [32 x i8], align 16
  %words.i68.i = alloca [32 x i8], align 16
  %words.i67.i = alloca [32 x i8], align 16
  %words.i66.i = alloca [32 x i8], align 16
  %words.i.i = alloca [32 x i8], align 4
  %pending.i = alloca [64 x i8], align 4
  %words.i18.i127.i.i = alloca [32 x i8], align 16
  %words.i17.i128.i.i = alloca [32 x i8], align 16
  %words.i.i129.i.i = alloca [32 x i8], align 16
  %words.i18.i.i.i = alloca [32 x i8], align 16
  %words.i17.i.i.i = alloca [32 x i8], align 16
  %words.i16.i.i.i = alloca [32 x i8], align 16
  %words.i15.i.i.i = alloca [32 x i8], align 16
  %words.i14.i.i.i = alloca [32 x i8], align 16
  %words.i13.i.i.i = alloca [32 x i8], align 16
  %words.i12.i.i.i = alloca [32 x i8], align 16
  %words.i.i.i.i = alloca [32 x i8], align 16
  %iter.i.i = alloca [56 x i8], align 8
  %reports = alloca [320 x i8], align 8
  %report.sroa.0 = alloca [320 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !8401
  %1 = load i8, ptr %0, align 16, !dbg !8401, !range !1313, !noundef !12
  %.not = icmp eq i8 %1, 2, !dbg !8402
  br i1 %.not, label %bb13, label %bb14, !dbg !8405, !prof !180

bb14:                                             ; preds = %start
  call void @llvm.lifetime.start.p0(ptr nonnull %report.sroa.0), !dbg !8406
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %report.sroa.0, i8 0, i64 320, i1 false), !dbg !8408
  %2 = getelementptr inbounds nuw i8, ptr %block, i64 108, !dbg !8412
  %3 = load i8, ptr %2, align 4, !dbg !8412, !range !1328, !noundef !12
  %_44.not = icmp eq i8 %3, %1, !dbg !8419
  %4 = getelementptr inbounds nuw i8, ptr %block, i64 64
  %5 = load ptr, ptr %4, align 8
  %.not7 = icmp eq ptr %5, null
  %or.cond = select i1 %_44.not, i1 %.not7, i1 false, !dbg !8417
  br i1 %or.cond, label %bb4, label %bb3, !dbg !8417

bb13:                                             ; preds = %start
; call core::option::expect_failed
  tail call void @_RNvNtCs4NRVxsYgnAr_4core6option13expect_failed(ptr noalias noundef nonnull readonly captures(address, read_provenance) @alloc_376120b9c5efdf3d59386c16952a74b7, i64 noundef 31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_99ab90c4f654517a4481f4facf03e67c) #24, !dbg !8422
  unreachable, !dbg !8422

bb3:                                              ; preds = %bb14
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %_0, i8 0, i64 320, i1 false), !dbg !8423
  %report.sroa.7.0._0.sroa_idx16 = getelementptr inbounds nuw i8, ptr %_0, i64 320, !dbg !8423
  store i8 %1, ptr %report.sroa.7.0._0.sroa_idx16, align 8, !dbg !8423
  br label %bb12, !dbg !8424

bb4:                                              ; preds = %bb14
  call void @llvm.lifetime.start.p0(ptr nonnull %reports), !dbg !8425
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %reports, i8 0, i64 320, i1 false), !dbg !8426
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
  %_7.sroa.5128.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 8
  %_7.sroa.6131.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 16
  %_7.sroa.7134.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 24
  %11 = getelementptr inbounds nuw i8, ptr %pending.i, i64 32
  %_7.sroa.5128.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 40
  %_7.sroa.6131.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 48
  %_7.sroa.7134.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 56
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 1796
  %13 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 1820
  %15 = call i64 @llvm.usub.sat.i64(i64 %_36.1, i64 1), !dbg !8427
  br label %bb15, !dbg !8427

bb16:                                             ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit
  %16 = getelementptr inbounds nuw i8, ptr %block, i64 104, !dbg !8435
  %_27 = load i32, ptr %16, align 8, !dbg !8435, !noundef !12
  %frames = zext i32 %_27 to i64, !dbg !8435
  %_37.0 = load ptr, ptr %block, align 8, !dbg !8436, !nonnull !12, !align !3533, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !8436
  %_37.1 = load i64, ptr %17, align 8, !dbg !8436, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !8438
  %_38.0 = load ptr, ptr %18, align 8, !dbg !8438, !nonnull !12, !align !3533, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !8438
  %_38.1 = load i64, ptr %19, align 8, !dbg !8438, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8439), !dbg !8442
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8443), !dbg !8442
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8445), !dbg !8442
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8447), !dbg !8442
  %_9.i = load i32, ptr %14, align 4, !dbg !8449, !alias.scope !8439, !noalias !8453, !noundef !12
  %20 = tail call i32 @llvm.umin.i32(i32 %_27, i32 %_9.i), !dbg !8455
  %..i.i = zext i32 %20 to i64, !dbg !8455
  %_10.not.i = icmp eq i32 %20, 0, !dbg !8457
  br i1 %_10.not.i, label %bb4.i, label %bb9.i, !dbg !8457

bb4.i:                                            ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKb1_EB3_.exit.i, %bb16
  %_18.i = icmp ugt i32 %_27, %_9.i, !dbg !8459
  br i1 %_18.i, label %bb20.i, label %bb7.i, !dbg !8459

bb9.i:                                            ; preds = %bb16
  %21 = shl nuw nsw i64 %..i.i, 2, !dbg !8460
  %_33.not.i = icmp samesign ugt i64 %21, %_37.1
  br i1 %_33.not.i, label %bb16.i, label %bb14.i, !dbg !8461, !prof !2561

bb16.i:                                           ; preds = %bb9.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %21, i64 noundef range(i64 0, 2305843009213693952) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_dd5f55065f566218c9f31cb2a4357231) #24, !dbg !8472, !noalias !8453
  unreachable, !dbg !8472

bb14.i:                                           ; preds = %bb9.i
  %_41.not.i = icmp samesign ugt i64 %21, %_38.1, !dbg !8473
  br i1 %_41.not.i, label %bb19.i, label %bb18.i, !dbg !8473, !prof !180

bb19.i:                                           ; preds = %bb14.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %21, i64 noundef range(i64 0, 2305843009213693952) %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1d7cc6e40c752396aa7def7556a6c433) #24, !dbg !8479, !noalias !8453
  unreachable, !dbg !8479

bb18.i:                                           ; preds = %bb14.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8480), !dbg !8483
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8484), !dbg !8483
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8486), !dbg !8483
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1392, !dbg !8488
  %_15.i.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !8501
  %data.i.i873.i.i = getelementptr inbounds nuw i8, ptr %self, i64 832, !dbg !8503
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 1772, !dbg !8508
  %_29.i.i = load i32, ptr %22, align 4, !dbg !8508, !range !1335, !alias.scope !8512, !noalias !8513, !noundef !12
  %_31.i.i = getelementptr inbounds nuw i8, ptr %self, i64 800, !dbg !8515
  %_33.i.i = getelementptr inbounds nuw i8, ptr %self, i64 864, !dbg !8516
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8517), !dbg !8520
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8521), !dbg !8520
  %23 = icmp eq i32 %_29.i.i, 1, !dbg !8523
  br i1 %23, label %bb7.i.i, label %bb1.i.i.preheader.i.i, !dbg !8523

bb1.i.i.preheader.i.i:                            ; preds = %bb18.i
  %.val.i.i.i.i = load i32, ptr %_31.i.i, align 4, !dbg !8525, !alias.scope !8536, !noalias !8537, !noundef !12
  %.val1.i.i.i.i = load i32, ptr %_33.i.i, align 4, !dbg !8525, !alias.scope !8540, !noalias !8541, !noundef !12
  %_0.i.i.not.i.i.i.i = icmp eq i32 %.val.i.i.i.i, %.val1.i.i.i.i, !dbg !8542
  br i1 %_0.i.i.not.i.i.i.i, label %bb1.i.i.1.i.i, label %bb7.i.i, !dbg !8525

bb1.i.i.1.i.i:                                    ; preds = %bb1.i.i.preheader.i.i
  %_3.i.i.i.i.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 804, !dbg !8556
  %_3.i1.i.i.i.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !8570
  %.val.i.i.1.i.i = load i32, ptr %_3.i.i.i.i.i.1.i.i, align 4, !dbg !8525, !alias.scope !8536, !noalias !8537, !noundef !12
  %.val1.i.i.1.i.i = load i32, ptr %_3.i1.i.i.i.i.1.i.i, align 4, !dbg !8525, !alias.scope !8540, !noalias !8541, !noundef !12
  %_0.i.i.not.i.i.1.i.i = icmp eq i32 %.val.i.i.1.i.i, %.val1.i.i.1.i.i, !dbg !8542
  br i1 %_0.i.i.not.i.i.1.i.i, label %bb1.i.i.2.i.i, label %bb7.i.i, !dbg !8525

bb1.i.i.2.i.i:                                    ; preds = %bb1.i.i.1.i.i
  %_3.i.i.i.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 808, !dbg !8556
  %_3.i1.i.i.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 872, !dbg !8570
  %.val.i.i.2.i.i = load i32, ptr %_3.i.i.i.i.i.2.i.i, align 4, !dbg !8525, !alias.scope !8536, !noalias !8537, !noundef !12
  %.val1.i.i.2.i.i = load i32, ptr %_3.i1.i.i.i.i.2.i.i, align 4, !dbg !8525, !alias.scope !8540, !noalias !8541, !noundef !12
  %_0.i.i.not.i.i.2.i.i = icmp eq i32 %.val.i.i.2.i.i, %.val1.i.i.2.i.i, !dbg !8542
  br i1 %_0.i.i.not.i.i.2.i.i, label %bb1.i.i.3.i.i, label %bb7.i.i, !dbg !8525

bb1.i.i.3.i.i:                                    ; preds = %bb1.i.i.2.i.i
  %_3.i.i.i.i.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 812, !dbg !8556
  %_3.i1.i.i.i.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 876, !dbg !8570
  %.val.i.i.3.i.i = load i32, ptr %_3.i.i.i.i.i.3.i.i, align 4, !dbg !8525, !alias.scope !8536, !noalias !8537, !noundef !12
  %.val1.i.i.3.i.i = load i32, ptr %_3.i1.i.i.i.i.3.i.i, align 4, !dbg !8525, !alias.scope !8540, !noalias !8541, !noundef !12
  %_0.i.i.not.i.i.3.i.i = icmp eq i32 %.val.i.i.3.i.i, %.val1.i.i.3.i.i, !dbg !8542
  %spec.select.i.i = select i1 %_0.i.i.not.i.i.3.i.i, i8 1, i8 2, !dbg !8525
  br label %bb7.i.i, !dbg !8525

bb7.i.i:                                          ; preds = %bb1.i.i.3.i.i, %bb1.i.i.2.i.i, %bb1.i.i.1.i.i, %bb1.i.i.preheader.i.i, %bb18.i
  %_0.sroa.0.0.i878.i.i = phi i8 [ 0, %bb18.i ], [ 2, %bb1.i.i.preheader.i.i ], [ 2, %bb1.i.i.2.i.i ], [ %spec.select.i.i, %bb1.i.i.3.i.i ], [ 2, %bb1.i.i.1.i.i ], !dbg !8573
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 896, !dbg !8574
  %_38.i.i = getelementptr inbounds nuw i8, ptr %self, i64 992, !dbg !8576
  %_64.0.i.i = load ptr, ptr %_15.i.i, align 8, !dbg !8577, !alias.scope !8512, !noalias !8513, !nonnull !12, !noundef !12
  %25 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !8577
  %_64.1.i.i = load i64, ptr %25, align 8, !dbg !8577, !alias.scope !8512, !noalias !8513, !noundef !12
  %_66.0.i.i = load ptr, ptr %data.i.i873.i.i, align 8, !dbg !8578, !alias.scope !8512, !noalias !8513, !nonnull !12, !noundef !12
  %26 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !8578
  %_66.1.i.i = load i64, ptr %26, align 8, !dbg !8578, !alias.scope !8512, !noalias !8513, !noundef !12
  %_57.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1808, !dbg !8579
  %27 = getelementptr inbounds nuw i8, ptr %self, i64 1812, !dbg !8580
  %_58.i.i = load i32, ptr %27, align 4, !dbg !8580, !alias.scope !8512, !noalias !8513, !noundef !12
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 1816, !dbg !8581
  %_59.i.i = load i32, ptr %28, align 8, !dbg !8581, !alias.scope !8512, !noalias !8513, !noundef !12
  %base.i.i.i = load i32, ptr %_57.i.i, align 4, !dbg !8582, !alias.scope !8512, !noalias !8592, !noundef !12
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 976
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 1360
  %33 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %35 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %36 = getelementptr inbounds nuw i8, ptr %self, i64 944
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 1648
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 1072
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 1008
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 1040
  %45 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 868
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 808
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %51 = getelementptr inbounds nuw i8, ptr %self, i64 1136
  %52 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %53 = getelementptr inbounds nuw i8, ptr %self, i64 1104
  %iter1.sroa.0.0.ptr.i120.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %54 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 1168
  %iter1.sroa.0.0.ptr.i120.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 1264
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 1232
  %iter1.sroa.0.0.ptr.i120.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %60 = getelementptr inbounds nuw i8, ptr %self, i64 1328
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 1296
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %64 = getelementptr inbounds nuw i8, ptr %self, i64 1424
  %65 = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %iter1.sroa.0.0.ptr.i.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1456
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 1488
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %iter1.sroa.0.0.ptr.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1520
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 1568
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 1552
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 1536
  %iter1.sroa.0.0.ptr.i.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1584
  %72 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %73 = getelementptr inbounds nuw i8, ptr %self, i64 1616
  %74 = getelementptr inbounds nuw i8, ptr %self, i64 1600
  br label %bb30.i.i.i, !dbg !8595

bb30.i.i.i:                                       ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i, %bb7.i.i
  %iter.sroa.0.0.i1771.i.i = phi i64 [ 0, %bb7.i.i ], [ %75, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i ]
  %75 = add nuw nsw i64 %iter.sroa.0.0.i1771.i.i, 1, !dbg !8604
  %span.i.i.i = shl i64 %iter.sroa.0.0.i1771.i.i, 2, !dbg !8610
  %_28.i.i.i = trunc i64 %iter.sroa.0.0.i1771.i.i to i32, !dbg !8612
  %now.i.i.i = add i32 %base.i.i.i, %_28.i.i.i, !dbg !8614
  %_31.i.i.i = and i32 %now.i.i.i, %_58.i.i, !dbg !8617
  %_30.i.i.i = zext i32 %_31.i.i.i to i64, !dbg !8619
  %write.i.i.i = shl nuw nsw i64 %_30.i.i.i, 2, !dbg !8619
  %exitcond.not.i.i = icmp eq i64 %iter.sroa.0.0.i1771.i.i, %..i.i, !dbg !8620
  br i1 %exitcond.not.i.i, label %bb33.i.i.i, label %bb32.i.i.i, !dbg !8620, !prof !2561

bb33.i.i.i:                                       ; preds = %bb30.i.i.i
  %_35.i.i.i = add nuw nsw i64 %span.i.i.i, 4, !dbg !8628
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i.i, i64 noundef %_35.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %21, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d0dbd696a058609bd94bbe1fa75d0723) #24, !dbg !8629, !noalias !8592
  unreachable, !dbg !8629

bb32.i.i.i:                                       ; preds = %bb30.i.i.i
  %_97.i.i.i = getelementptr inbounds nuw float, ptr %_37.0, i64 %span.i.i.i, !dbg !8630
  %_37.i.i.i = add nuw nsw i64 %write.i.i.i, 4, !dbg !8634
  %_98.not.i.i.i = icmp ugt i64 %_37.i.i.i, %_64.1.i.i, !dbg !8635
  br i1 %_98.not.i.i.i, label %bb36.i.i.i, label %bb38.i.i.i, !dbg !8635, !prof !180

bb36.i.i.i:                                       ; preds = %bb32.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i.i, i64 noundef %_37.i.i.i, i64 noundef %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1eeef3195352adc58f4bfdb015316fef) #24, !dbg !8640, !noalias !8592
  unreachable, !dbg !8640

bb38.i.i.i:                                       ; preds = %bb32.i.i.i
  %_107.i.i.i = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %write.i.i.i, !dbg !8641
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %_107.i.i.i, ptr noundef nonnull align 4 dereferenceable(16) %_97.i.i.i, i64 16, i1 false), !dbg !8645, !noalias !8650
  %_115.i.i.i = getelementptr inbounds nuw float, ptr %_38.0, i64 %span.i.i.i, !dbg !8651
  %_116.not.i.i.i = icmp ugt i64 %_37.i.i.i, %_66.1.i.i, !dbg !8658
  br i1 %_116.not.i.i.i, label %bb41.i.i.i, label %bb40.i.i.i, !dbg !8658, !prof !180

bb41.i.i.i:                                       ; preds = %bb38.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i.i, i64 noundef %_37.i.i.i, i64 noundef %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9d56679ca30f2caa500ddf2e89d00cb) #24, !dbg !8662, !noalias !8592
  unreachable, !dbg !8662

bb40.i.i.i:                                       ; preds = %bb38.i.i.i
  %_123.i.i.i = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %write.i.i.i, !dbg !8663
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %_123.i.i.i, ptr noundef nonnull align 4 dereferenceable(16) %_115.i.i.i, i64 16, i1 false), !dbg !8667, !noalias !8672
  %_53.i.i.i = sub i32 %now.i.i.i, %_59.i.i, !dbg !8673
  %_52.i.i.i = and i32 %_53.i.i.i, %_58.i.i, !dbg !8676
  %_51.i.i.i = zext i32 %_52.i.i.i to i64, !dbg !8677
  %read.i.i.i = shl nuw nsw i64 %_51.i.i.i, 2, !dbg !8677
  %_56.i.i.i = add nuw nsw i64 %read.i.i.i, 4, !dbg !8678
  %_156.not.i.i.i = icmp ugt i64 %_56.i.i.i, %_64.1.i.i, !dbg !8680
  br i1 %_156.not.i.i.i, label %bb51.i.i.i, label %bb50.i.i.i, !dbg !8680, !prof !180

bb51.i.i.i:                                       ; preds = %bb40.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i.i, i64 noundef %_56.i.i.i, i64 noundef %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7) #24, !dbg !8684, !noalias !8592
  unreachable, !dbg !8684

bb50.i.i.i:                                       ; preds = %bb40.i.i.i
  %_163.i.i.i = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %read.i.i.i, !dbg !8685
  %lanes.i471.sroa.0.0.copyload.i.i = load <4 x float>, ptr %_163.i.i.i, align 4, !dbg !8689, !alias.scope !8697, !noalias !8701
  %_164.not.i.i.i = icmp ugt i64 %_56.i.i.i, %_66.1.i.i, !dbg !8705
  br i1 %_164.not.i.i.i, label %bb54.i.i.i, label %bb53.i.i.i, !dbg !8705, !prof !180

bb54.i.i.i:                                       ; preds = %bb50.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i.i, i64 noundef %_56.i.i.i, i64 noundef %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f198228fd40f4c04a48a745576c5c5ee) #24, !dbg !8710, !noalias !8592
  unreachable, !dbg !8710

bb53.i.i.i:                                       ; preds = %bb50.i.i.i
  %_169.i.i.i = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %read.i.i.i, !dbg !8711
  %lanes.i465.sroa.0.0.copyload.i.i = load <4 x float>, ptr %_169.i.i.i, align 4, !dbg !8715, !alias.scope !8720, !noalias !8724
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8728), !dbg !8731
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8734), !dbg !8731
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8736), !dbg !8731
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8738), !dbg !8731
  %_18.i71.i.i = load i32, ptr %_31.i.i, align 4, !dbg !8740, !alias.scope !8742, !noalias !8743, !noundef !12
  %_17.i.i.i = sub i32 %now.i.i.i, %_18.i71.i.i, !dbg !8745
  %_16.i72.i.i = and i32 %_17.i.i.i, %_58.i.i, !dbg !8740
  %_15.i73.i.i = zext i32 %_16.i72.i.i to i64, !dbg !8740
  %_14.i.i.i = shl nuw nsw i64 %_15.i73.i.i, 2, !dbg !8740
  %_26.i.i.i = load i32, ptr %_33.i.i, align 4, !dbg !8740, !alias.scope !8747, !noalias !8748, !noundef !12
  %_25.i.i.i = sub i32 %now.i.i.i, %_26.i.i.i, !dbg !8745
  %_24.i.i.i = and i32 %_25.i.i.i, %_58.i.i, !dbg !8740
  %_23.i76.i.i = zext i32 %_24.i.i.i to i64, !dbg !8740
  %_22.i.i.i = shl nuw nsw i64 %_23.i76.i.i, 2, !dbg !8740
  %_31.i77.i.i = icmp samesign ult i64 %_14.i.i.i, %_64.1.i.i, !dbg !8740
  switch i8 %_0.sroa.0.0.i878.i.i, label %bb53.i.i.i.unreachabledefault [
    i8 0, label %bb7.i75.preheader.i.i
    i8 1, label %bb16.i.preheader.i.i
    i8 2, label %bb25.i.preheader.i.i
  ], !dbg !8749

bb25.i.preheader.i.i:                             ; preds = %bb53.i.i.i
  br i1 %_31.i77.i.i, label %bb27.i58.i.i, label %panic28.i.i.i, !dbg !8750

bb16.i.preheader.i.i:                             ; preds = %bb53.i.i.i
  br i1 %_31.i77.i.i, label %bb17.i.i.i, label %panic15.i.i.i, !dbg !8751

bb7.i75.preheader.i.i:                            ; preds = %bb53.i.i.i
  br i1 %_31.i77.i.i, label %bb8.i78.i.i, label %panic4.i.i.i, !dbg !8752

bb53.i.i.i.unreachabledefault:                    ; preds = %bb53.i.i.i
  unreachable

default.unreachable:                              ; preds = %bb4.i10, %bb53.i.i74.i
  unreachable

bb8.i78.i.i:                                      ; preds = %bb7.i75.preheader.i.i
  %_34.i.i.i = icmp samesign ult i64 %_22.i.i.i, %_66.1.i.i, !dbg !8753
  br i1 %_34.i.i.i, label %bb10.i79.i.i, label %panic5.i.i.i, !dbg !8753

panic4.i.i.i:                                     ; preds = %bb10.i79.2.i.i, %bb10.i79.1.i.i, %bb10.i79.i.i, %bb7.i75.preheader.i.i
  %left.i.lcssa.i.i = phi i64 [ %_14.i.i.i, %bb7.i75.preheader.i.i ], [ %left.i.1.i.i, %bb10.i79.i.i ], [ %left.i.2.i.i, %bb10.i79.1.i.i ], [ %left.i.3.i.i, %bb10.i79.2.i.i ], !dbg !8754
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left.i.lcssa.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cba26bb3a04aaba20f9c249edc5e133d) #24, !dbg !8752, !noalias !8755
  unreachable, !dbg !8752

panic5.i.i.i:                                     ; preds = %bb8.i78.3.i.i, %bb8.i78.2.i.i, %bb8.i78.1.i.i, %bb8.i78.i.i
  %right.i.lcssa1786.i.i = phi i64 [ %_22.i.i.i, %bb8.i78.i.i ], [ %right.i.1.i.i, %bb8.i78.1.i.i ], [ %right.i.2.i.i, %bb8.i78.2.i.i ], [ %right.i.3.i.i, %bb8.i78.3.i.i ], !dbg !8756
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right.i.lcssa1786.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2ad2d313de59c36d62f4a7d75017a71f) #24, !dbg !8753, !noalias !8755
  unreachable, !dbg !8753

bb10.i79.i.i:                                     ; preds = %bb8.i78.i.i
  %76 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_14.i.i.i, !dbg !8752
  %left_own.i.i.i = load float, ptr %76, align 4, !dbg !8752, !alias.scope !8736, !noalias !8757, !noundef !12
  %77 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_22.i.i.i, !dbg !8753
  %right_own.i.i.i = load float, ptr %77, align 4, !dbg !8753, !alias.scope !8738, !noalias !8758, !noundef !12
  %_18.i71.1.i.i = load i32, ptr %45, align 4, !dbg !8759, !alias.scope !8742, !noalias !8743, !noundef !12
  %_17.i.1.i.i = sub i32 %now.i.i.i, %_18.i71.1.i.i, !dbg !8760
  %_16.i72.1.i.i = and i32 %_17.i.1.i.i, %_58.i.i, !dbg !8762
  %_15.i73.1.i.i = zext i32 %_16.i72.1.i.i to i64, !dbg !8754
  %_14.i.1.i.i = shl nuw nsw i64 %_15.i73.1.i.i, 2, !dbg !8754
  %left.i.1.i.i = or disjoint i64 %_14.i.1.i.i, 1, !dbg !8754
  %_26.i.1.i.i = load i32, ptr %46, align 4, !dbg !8763, !alias.scope !8747, !noalias !8748, !noundef !12
  %_25.i.1.i.i = sub i32 %now.i.i.i, %_26.i.1.i.i, !dbg !8764
  %_24.i.1.i.i = and i32 %_25.i.1.i.i, %_58.i.i, !dbg !8766
  %_23.i76.1.i.i = zext i32 %_24.i.1.i.i to i64, !dbg !8756
  %_22.i.1.i.i = shl nuw nsw i64 %_23.i76.1.i.i, 2, !dbg !8756
  %right.i.1.i.i = or disjoint i64 %_22.i.1.i.i, 1, !dbg !8756
  %_31.i77.1.i.i = icmp samesign ult i64 %left.i.1.i.i, %_64.1.i.i, !dbg !8752
  br i1 %_31.i77.1.i.i, label %bb8.i78.1.i.i, label %panic4.i.i.i, !dbg !8752

bb8.i78.1.i.i:                                    ; preds = %bb10.i79.i.i
  %_34.i.1.i.i = icmp samesign ult i64 %right.i.1.i.i, %_66.1.i.i, !dbg !8753
  br i1 %_34.i.1.i.i, label %bb10.i79.1.i.i, label %panic5.i.i.i, !dbg !8753

bb10.i79.1.i.i:                                   ; preds = %bb8.i78.1.i.i
  %78 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.1.i.i, !dbg !8752
  %left_own.i.1.i.i = load float, ptr %78, align 4, !dbg !8752, !alias.scope !8736, !noalias !8757, !noundef !12
  %79 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.1.i.i, !dbg !8753
  %right_own.i.1.i.i = load float, ptr %79, align 4, !dbg !8753, !alias.scope !8738, !noalias !8758, !noundef !12
  %_18.i71.2.i.i = load i32, ptr %47, align 4, !dbg !8759, !alias.scope !8742, !noalias !8743, !noundef !12
  %_17.i.2.i.i = sub i32 %now.i.i.i, %_18.i71.2.i.i, !dbg !8760
  %_16.i72.2.i.i = and i32 %_17.i.2.i.i, %_58.i.i, !dbg !8762
  %_15.i73.2.i.i = zext i32 %_16.i72.2.i.i to i64, !dbg !8754
  %_14.i.2.i.i = shl nuw nsw i64 %_15.i73.2.i.i, 2, !dbg !8754
  %left.i.2.i.i = or disjoint i64 %_14.i.2.i.i, 2, !dbg !8754
  %_26.i.2.i.i = load i32, ptr %48, align 4, !dbg !8763, !alias.scope !8747, !noalias !8748, !noundef !12
  %_25.i.2.i.i = sub i32 %now.i.i.i, %_26.i.2.i.i, !dbg !8764
  %_24.i.2.i.i = and i32 %_25.i.2.i.i, %_58.i.i, !dbg !8766
  %_23.i76.2.i.i = zext i32 %_24.i.2.i.i to i64, !dbg !8756
  %_22.i.2.i.i = shl nuw nsw i64 %_23.i76.2.i.i, 2, !dbg !8756
  %right.i.2.i.i = or disjoint i64 %_22.i.2.i.i, 2, !dbg !8756
  %_31.i77.2.i.i = icmp samesign ult i64 %left.i.2.i.i, %_64.1.i.i, !dbg !8752
  br i1 %_31.i77.2.i.i, label %bb8.i78.2.i.i, label %panic4.i.i.i, !dbg !8752

bb8.i78.2.i.i:                                    ; preds = %bb10.i79.1.i.i
  %_34.i.2.i.i = icmp samesign ult i64 %right.i.2.i.i, %_66.1.i.i, !dbg !8753
  br i1 %_34.i.2.i.i, label %bb10.i79.2.i.i, label %panic5.i.i.i, !dbg !8753

bb10.i79.2.i.i:                                   ; preds = %bb8.i78.2.i.i
  %80 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.2.i.i, !dbg !8752
  %left_own.i.2.i.i = load float, ptr %80, align 4, !dbg !8752, !alias.scope !8736, !noalias !8757, !noundef !12
  %81 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.2.i.i, !dbg !8753
  %right_own.i.2.i.i = load float, ptr %81, align 4, !dbg !8753, !alias.scope !8738, !noalias !8758, !noundef !12
  %_18.i71.3.i.i = load i32, ptr %49, align 4, !dbg !8759, !alias.scope !8742, !noalias !8743, !noundef !12
  %_17.i.3.i.i = sub i32 %now.i.i.i, %_18.i71.3.i.i, !dbg !8760
  %_16.i72.3.i.i = and i32 %_17.i.3.i.i, %_58.i.i, !dbg !8762
  %_15.i73.3.i.i = zext i32 %_16.i72.3.i.i to i64, !dbg !8754
  %_14.i.3.i.i = shl nuw nsw i64 %_15.i73.3.i.i, 2, !dbg !8754
  %left.i.3.i.i = or disjoint i64 %_14.i.3.i.i, 3, !dbg !8754
  %_26.i.3.i.i = load i32, ptr %50, align 4, !dbg !8763, !alias.scope !8747, !noalias !8748, !noundef !12
  %_25.i.3.i.i = sub i32 %now.i.i.i, %_26.i.3.i.i, !dbg !8764
  %_24.i.3.i.i = and i32 %_25.i.3.i.i, %_58.i.i, !dbg !8766
  %_23.i76.3.i.i = zext i32 %_24.i.3.i.i to i64, !dbg !8756
  %_22.i.3.i.i = shl nuw nsw i64 %_23.i76.3.i.i, 2, !dbg !8756
  %right.i.3.i.i = or disjoint i64 %_22.i.3.i.i, 3, !dbg !8756
  %_31.i77.3.i.i = icmp samesign ult i64 %left.i.3.i.i, %_64.1.i.i, !dbg !8752
  br i1 %_31.i77.3.i.i, label %bb8.i78.3.i.i, label %panic4.i.i.i, !dbg !8752

bb8.i78.3.i.i:                                    ; preds = %bb10.i79.2.i.i
  %_34.i.3.i.i = icmp samesign ult i64 %right.i.3.i.i, %_66.1.i.i, !dbg !8753
  br i1 %_34.i.3.i.i, label %bb10.i79.3.i.i, label %panic5.i.i.i, !dbg !8753

bb10.i79.3.i.i:                                   ; preds = %bb8.i78.3.i.i
  %82 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left.i.3.i.i, !dbg !8752
  %left_own.i.3.i.i = load float, ptr %82, align 4, !dbg !8752, !alias.scope !8736, !noalias !8757, !noundef !12
  %83 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right.i.3.i.i, !dbg !8753
  %right_own.i.3.i.i = load float, ptr %83, align 4, !dbg !8753, !alias.scope !8738, !noalias !8758, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !8767

bb17.i.i.i:                                       ; preds = %bb16.i.preheader.i.i
  %_59.i.i.i = icmp samesign ult i64 %_22.i.i.i, %_66.1.i.i, !dbg !8770
  br i1 %_59.i.i.i, label %bb19.i.i.i, label %panic17.i.i.i, !dbg !8770

panic15.i.i.i:                                    ; preds = %bb19.i.2.i.i, %bb19.i.1.i.i, %bb19.i.i.i, %bb16.i.preheader.i.i
  %left12.i.lcssa.i.i = phi i64 [ %_14.i.i.i, %bb16.i.preheader.i.i ], [ %left12.i.1.i.i, %bb19.i.i.i ], [ %left12.i.2.i.i, %bb19.i.1.i.i ], [ %left12.i.3.i.i, %bb19.i.2.i.i ], !dbg !8771
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left12.i.lcssa.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2e01e5f964095ee8e5aa091c7d92b88) #24, !dbg !8751, !noalias !8755
  unreachable, !dbg !8751

panic17.i.i.i:                                    ; preds = %bb17.i.3.i.i, %bb17.i.2.i.i, %bb17.i.1.i.i, %bb17.i.i.i
  %right14.i.lcssa1782.i.i = phi i64 [ %_22.i.i.i, %bb17.i.i.i ], [ %right14.i.1.i.i, %bb17.i.1.i.i ], [ %right14.i.2.i.i, %bb17.i.2.i.i ], [ %right14.i.3.i.i, %bb17.i.3.i.i ], !dbg !8772
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right14.i.lcssa1782.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_48dca199019b49040caac979cf1ddc5a) #24, !dbg !8770, !noalias !8755
  unreachable, !dbg !8770

bb19.i.i.i:                                       ; preds = %bb17.i.i.i
  %84 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_14.i.i.i, !dbg !8751
  %left_own16.i.i.i = load float, ptr %84, align 4, !dbg !8751, !alias.scope !8736, !noalias !8757, !noundef !12
  %85 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_22.i.i.i, !dbg !8770
  %right_own18.i.i.i = load float, ptr %85, align 4, !dbg !8770, !alias.scope !8738, !noalias !8758, !noundef !12
  %_43.i.1.i.i = load i32, ptr %45, align 4, !dbg !8773, !alias.scope !8742, !noalias !8743, !noundef !12
  %_42.i.1.i.i = sub i32 %now.i.i.i, %_43.i.1.i.i, !dbg !8774
  %_41.i.1.i.i = and i32 %_42.i.1.i.i, %_58.i.i, !dbg !8776
  %_40.i.1.i.i = zext i32 %_41.i.1.i.i to i64, !dbg !8771
  %_39.i63.1.i.i = shl nuw nsw i64 %_40.i.1.i.i, 2, !dbg !8771
  %left12.i.1.i.i = or disjoint i64 %_39.i63.1.i.i, 1, !dbg !8771
  %_51.i65.1.i.i = load i32, ptr %46, align 4, !dbg !8777, !alias.scope !8747, !noalias !8748, !noundef !12
  %_50.i.1.i.i = sub i32 %now.i.i.i, %_51.i65.1.i.i, !dbg !8778
  %_49.i.1.i.i = and i32 %_50.i.1.i.i, %_58.i.i, !dbg !8780
  %_48.i.1.i.i = zext i32 %_49.i.1.i.i to i64, !dbg !8772
  %_47.i.1.i.i = shl nuw nsw i64 %_48.i.1.i.i, 2, !dbg !8772
  %right14.i.1.i.i = or disjoint i64 %_47.i.1.i.i, 1, !dbg !8772
  %_56.i66.1.i.i = icmp samesign ult i64 %left12.i.1.i.i, %_64.1.i.i, !dbg !8751
  br i1 %_56.i66.1.i.i, label %bb17.i.1.i.i, label %panic15.i.i.i, !dbg !8751

bb17.i.1.i.i:                                     ; preds = %bb19.i.i.i
  %_59.i.1.i.i = icmp samesign ult i64 %right14.i.1.i.i, %_66.1.i.i, !dbg !8770
  br i1 %_59.i.1.i.i, label %bb19.i.1.i.i, label %panic17.i.i.i, !dbg !8770

bb19.i.1.i.i:                                     ; preds = %bb17.i.1.i.i
  %86 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.1.i.i, !dbg !8751
  %left_own16.i.1.i.i = load float, ptr %86, align 4, !dbg !8751, !alias.scope !8736, !noalias !8757, !noundef !12
  %87 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.1.i.i, !dbg !8770
  %right_own18.i.1.i.i = load float, ptr %87, align 4, !dbg !8770, !alias.scope !8738, !noalias !8758, !noundef !12
  %_43.i.2.i.i = load i32, ptr %47, align 4, !dbg !8773, !alias.scope !8742, !noalias !8743, !noundef !12
  %_42.i.2.i.i = sub i32 %now.i.i.i, %_43.i.2.i.i, !dbg !8774
  %_41.i.2.i.i = and i32 %_42.i.2.i.i, %_58.i.i, !dbg !8776
  %_40.i.2.i.i = zext i32 %_41.i.2.i.i to i64, !dbg !8771
  %_39.i63.2.i.i = shl nuw nsw i64 %_40.i.2.i.i, 2, !dbg !8771
  %left12.i.2.i.i = or disjoint i64 %_39.i63.2.i.i, 2, !dbg !8771
  %_51.i65.2.i.i = load i32, ptr %48, align 4, !dbg !8777, !alias.scope !8747, !noalias !8748, !noundef !12
  %_50.i.2.i.i = sub i32 %now.i.i.i, %_51.i65.2.i.i, !dbg !8778
  %_49.i.2.i.i = and i32 %_50.i.2.i.i, %_58.i.i, !dbg !8780
  %_48.i.2.i.i = zext i32 %_49.i.2.i.i to i64, !dbg !8772
  %_47.i.2.i.i = shl nuw nsw i64 %_48.i.2.i.i, 2, !dbg !8772
  %right14.i.2.i.i = or disjoint i64 %_47.i.2.i.i, 2, !dbg !8772
  %_56.i66.2.i.i = icmp samesign ult i64 %left12.i.2.i.i, %_64.1.i.i, !dbg !8751
  br i1 %_56.i66.2.i.i, label %bb17.i.2.i.i, label %panic15.i.i.i, !dbg !8751

bb17.i.2.i.i:                                     ; preds = %bb19.i.1.i.i
  %_59.i.2.i.i = icmp samesign ult i64 %right14.i.2.i.i, %_66.1.i.i, !dbg !8770
  br i1 %_59.i.2.i.i, label %bb19.i.2.i.i, label %panic17.i.i.i, !dbg !8770

bb19.i.2.i.i:                                     ; preds = %bb17.i.2.i.i
  %88 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.2.i.i, !dbg !8751
  %left_own16.i.2.i.i = load float, ptr %88, align 4, !dbg !8751, !alias.scope !8736, !noalias !8757, !noundef !12
  %89 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.2.i.i, !dbg !8770
  %right_own18.i.2.i.i = load float, ptr %89, align 4, !dbg !8770, !alias.scope !8738, !noalias !8758, !noundef !12
  %_43.i.3.i.i = load i32, ptr %49, align 4, !dbg !8773, !alias.scope !8742, !noalias !8743, !noundef !12
  %_42.i.3.i.i = sub i32 %now.i.i.i, %_43.i.3.i.i, !dbg !8774
  %_41.i.3.i.i = and i32 %_42.i.3.i.i, %_58.i.i, !dbg !8776
  %_40.i.3.i.i = zext i32 %_41.i.3.i.i to i64, !dbg !8771
  %_39.i63.3.i.i = shl nuw nsw i64 %_40.i.3.i.i, 2, !dbg !8771
  %left12.i.3.i.i = or disjoint i64 %_39.i63.3.i.i, 3, !dbg !8771
  %_51.i65.3.i.i = load i32, ptr %50, align 4, !dbg !8777, !alias.scope !8747, !noalias !8748, !noundef !12
  %_50.i.3.i.i = sub i32 %now.i.i.i, %_51.i65.3.i.i, !dbg !8778
  %_49.i.3.i.i = and i32 %_50.i.3.i.i, %_58.i.i, !dbg !8780
  %_48.i.3.i.i = zext i32 %_49.i.3.i.i to i64, !dbg !8772
  %_47.i.3.i.i = shl nuw nsw i64 %_48.i.3.i.i, 2, !dbg !8772
  %right14.i.3.i.i = or disjoint i64 %_47.i.3.i.i, 3, !dbg !8772
  %_56.i66.3.i.i = icmp samesign ult i64 %left12.i.3.i.i, %_64.1.i.i, !dbg !8751
  br i1 %_56.i66.3.i.i, label %bb17.i.3.i.i, label %panic15.i.i.i, !dbg !8751

bb17.i.3.i.i:                                     ; preds = %bb19.i.2.i.i
  %_59.i.3.i.i = icmp samesign ult i64 %right14.i.3.i.i, %_66.1.i.i, !dbg !8770
  br i1 %_59.i.3.i.i, label %bb19.i.3.i.i, label %panic17.i.i.i, !dbg !8770

bb19.i.3.i.i:                                     ; preds = %bb17.i.3.i.i
  %90 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left12.i.3.i.i, !dbg !8751
  %left_own16.i.3.i.i = load float, ptr %90, align 4, !dbg !8751, !alias.scope !8736, !noalias !8757, !noundef !12
  %91 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right14.i.3.i.i, !dbg !8770
  %right_own18.i.3.i.i = load float, ptr %91, align 4, !dbg !8770, !alias.scope !8738, !noalias !8758, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !8767

panic28.i.i.i:                                    ; preds = %bb33.i60.2.i.i, %bb33.i60.1.i.i, %bb33.i60.i.i, %bb25.i.preheader.i.i
  %left25.i.lcssa.i.i = phi i64 [ %_14.i.i.i, %bb25.i.preheader.i.i ], [ %left25.i.1.i.i, %bb33.i60.i.i ], [ %left25.i.2.i.i, %bb33.i60.1.i.i ], [ %left25.i.3.i.i, %bb33.i60.2.i.i ], !dbg !8781
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left25.i.lcssa.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4b7b2eb7fa1b19ad0451b09b71313122) #24, !dbg !8750, !noalias !8755
  unreachable, !dbg !8750

bb27.i58.i.i:                                     ; preds = %bb25.i.preheader.i.i
  %92 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_14.i.i.i, !dbg !8750
  %_79.i.i.i = load float, ptr %92, align 4, !dbg !8750, !alias.scope !8736, !noalias !8757, !noundef !12
  %_85.i.i.i = icmp samesign ult i64 %_14.i.i.i, %_66.1.i.i, !dbg !8782
  br i1 %_85.i.i.i, label %bb29.i59.i.i, label %panic30.i.i.i, !dbg !8782

panic30.i.i.i:                                    ; preds = %bb27.i58.3.i.i, %bb27.i58.2.i.i, %bb27.i58.1.i.i, %bb27.i58.i.i
  %left25.i.lcssa1778.i.i = phi i64 [ %_14.i.i.i, %bb27.i58.i.i ], [ %left25.i.1.i.i, %bb27.i58.1.i.i ], [ %left25.i.2.i.i, %bb27.i58.2.i.i ], [ %left25.i.3.i.i, %bb27.i58.3.i.i ], !dbg !8781
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left25.i.lcssa1778.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60256fc2b51ee5bb69caa4304418caff) #24, !dbg !8782, !noalias !8755
  unreachable, !dbg !8782

bb29.i59.i.i:                                     ; preds = %bb27.i58.i.i
  %93 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_14.i.i.i, !dbg !8782
  %_83.i.i.i = load float, ptr %93, align 4, !dbg !8782, !alias.scope !8738, !noalias !8758, !noundef !12
  %_87.i.i.i = icmp samesign ult i64 %_22.i.i.i, %_66.1.i.i, !dbg !8783
  br i1 %_87.i.i.i, label %bb31.i.i.i, label %panic32.i.i.i, !dbg !8783

panic32.i.i.i:                                    ; preds = %bb29.i59.3.i.i, %bb29.i59.2.i.i, %bb29.i59.1.i.i, %bb29.i59.i.i
  %right27.i.lcssa1775.i.i = phi i64 [ %_22.i.i.i, %bb29.i59.i.i ], [ %right27.i.1.i.i, %bb29.i59.1.i.i ], [ %right27.i.2.i.i, %bb29.i59.2.i.i ], [ %right27.i.3.i.i, %bb29.i59.3.i.i ], !dbg !8784
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right27.i.lcssa1775.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_03bcac171bcf0d517141d8cd3ac9ded0) #24, !dbg !8783, !noalias !8755
  unreachable, !dbg !8783

bb31.i.i.i:                                       ; preds = %bb29.i59.i.i
  %94 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_22.i.i.i, !dbg !8783
  %_86.i.i.i = load float, ptr %94, align 4, !dbg !8783, !alias.scope !8738, !noalias !8758, !noundef !12
  %_89.i.i.i = icmp samesign ult i64 %_22.i.i.i, %_64.1.i.i, !dbg !8785
  br i1 %_89.i.i.i, label %bb33.i60.i.i, label %panic34.i.i.i, !dbg !8785

panic34.i.i.i:                                    ; preds = %bb31.i.3.i.i, %bb31.i.2.i.i, %bb31.i.1.i.i, %bb31.i.i.i
  %right27.i.lcssa1776.i.i = phi i64 [ %_22.i.i.i, %bb31.i.i.i ], [ %right27.i.1.i.i, %bb31.i.1.i.i ], [ %right27.i.2.i.i, %bb31.i.2.i.i ], [ %right27.i.3.i.i, %bb31.i.3.i.i ], !dbg !8784
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right27.i.lcssa1776.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ed84bd2438b7b7e3dfb2147bac09e923) #24, !dbg !8785, !noalias !8755
  unreachable, !dbg !8785

bb33.i60.i.i:                                     ; preds = %bb31.i.i.i
  %95 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_22.i.i.i, !dbg !8785
  %_88.i.i.i = load float, ptr %95, align 4, !dbg !8785, !alias.scope !8736, !noalias !8757, !noundef !12
  %_68.i.1.i.i = load i32, ptr %45, align 4, !dbg !8786, !alias.scope !8742, !noalias !8743, !noundef !12
  %_67.i52.1.i.i = sub i32 %now.i.i.i, %_68.i.1.i.i, !dbg !8787
  %_66.i.1.i.i = and i32 %_67.i52.1.i.i, %_58.i.i, !dbg !8789
  %_65.i.1.i.i = zext i32 %_66.i.1.i.i to i64, !dbg !8781
  %_64.i53.1.i.i = shl nuw nsw i64 %_65.i.1.i.i, 2, !dbg !8781
  %left25.i.1.i.i = or disjoint i64 %_64.i53.1.i.i, 1, !dbg !8781
  %_76.i54.1.i.i = load i32, ptr %46, align 4, !dbg !8790, !alias.scope !8747, !noalias !8748, !noundef !12
  %_75.i.1.i.i = sub i32 %now.i.i.i, %_76.i54.1.i.i, !dbg !8791
  %_74.i55.1.i.i = and i32 %_75.i.1.i.i, %_58.i.i, !dbg !8793
  %_73.i56.1.i.i = zext i32 %_74.i55.1.i.i to i64, !dbg !8784
  %_72.i.1.i.i = shl nuw nsw i64 %_73.i56.1.i.i, 2, !dbg !8784
  %right27.i.1.i.i = or disjoint i64 %_72.i.1.i.i, 1, !dbg !8784
  %_81.i57.1.i.i = icmp samesign ult i64 %left25.i.1.i.i, %_64.1.i.i, !dbg !8750
  br i1 %_81.i57.1.i.i, label %bb27.i58.1.i.i, label %panic28.i.i.i, !dbg !8750

bb27.i58.1.i.i:                                   ; preds = %bb33.i60.i.i
  %96 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.1.i.i, !dbg !8750
  %_79.i.1.i.i = load float, ptr %96, align 4, !dbg !8750, !alias.scope !8736, !noalias !8757, !noundef !12
  %_85.i.1.i.i = icmp samesign ult i64 %left25.i.1.i.i, %_66.1.i.i, !dbg !8782
  br i1 %_85.i.1.i.i, label %bb29.i59.1.i.i, label %panic30.i.i.i, !dbg !8782

bb29.i59.1.i.i:                                   ; preds = %bb27.i58.1.i.i
  %97 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.1.i.i, !dbg !8782
  %_83.i.1.i.i = load float, ptr %97, align 4, !dbg !8782, !alias.scope !8738, !noalias !8758, !noundef !12
  %_87.i.1.i.i = icmp samesign ult i64 %right27.i.1.i.i, %_66.1.i.i, !dbg !8783
  br i1 %_87.i.1.i.i, label %bb31.i.1.i.i, label %panic32.i.i.i, !dbg !8783

bb31.i.1.i.i:                                     ; preds = %bb29.i59.1.i.i
  %98 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.1.i.i, !dbg !8783
  %_86.i.1.i.i = load float, ptr %98, align 4, !dbg !8783, !alias.scope !8738, !noalias !8758, !noundef !12
  %_89.i.1.i.i = icmp samesign ult i64 %right27.i.1.i.i, %_64.1.i.i, !dbg !8785
  br i1 %_89.i.1.i.i, label %bb33.i60.1.i.i, label %panic34.i.i.i, !dbg !8785

bb33.i60.1.i.i:                                   ; preds = %bb31.i.1.i.i
  %99 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.1.i.i, !dbg !8785
  %_88.i.1.i.i = load float, ptr %99, align 4, !dbg !8785, !alias.scope !8736, !noalias !8757, !noundef !12
  %_68.i.2.i.i = load i32, ptr %47, align 4, !dbg !8786, !alias.scope !8742, !noalias !8743, !noundef !12
  %_67.i52.2.i.i = sub i32 %now.i.i.i, %_68.i.2.i.i, !dbg !8787
  %_66.i.2.i.i = and i32 %_67.i52.2.i.i, %_58.i.i, !dbg !8789
  %_65.i.2.i.i = zext i32 %_66.i.2.i.i to i64, !dbg !8781
  %_64.i53.2.i.i = shl nuw nsw i64 %_65.i.2.i.i, 2, !dbg !8781
  %left25.i.2.i.i = or disjoint i64 %_64.i53.2.i.i, 2, !dbg !8781
  %_76.i54.2.i.i = load i32, ptr %48, align 4, !dbg !8790, !alias.scope !8747, !noalias !8748, !noundef !12
  %_75.i.2.i.i = sub i32 %now.i.i.i, %_76.i54.2.i.i, !dbg !8791
  %_74.i55.2.i.i = and i32 %_75.i.2.i.i, %_58.i.i, !dbg !8793
  %_73.i56.2.i.i = zext i32 %_74.i55.2.i.i to i64, !dbg !8784
  %_72.i.2.i.i = shl nuw nsw i64 %_73.i56.2.i.i, 2, !dbg !8784
  %right27.i.2.i.i = or disjoint i64 %_72.i.2.i.i, 2, !dbg !8784
  %_81.i57.2.i.i = icmp samesign ult i64 %left25.i.2.i.i, %_64.1.i.i, !dbg !8750
  br i1 %_81.i57.2.i.i, label %bb27.i58.2.i.i, label %panic28.i.i.i, !dbg !8750

bb27.i58.2.i.i:                                   ; preds = %bb33.i60.1.i.i
  %100 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.2.i.i, !dbg !8750
  %_79.i.2.i.i = load float, ptr %100, align 4, !dbg !8750, !alias.scope !8736, !noalias !8757, !noundef !12
  %_85.i.2.i.i = icmp samesign ult i64 %left25.i.2.i.i, %_66.1.i.i, !dbg !8782
  br i1 %_85.i.2.i.i, label %bb29.i59.2.i.i, label %panic30.i.i.i, !dbg !8782

bb29.i59.2.i.i:                                   ; preds = %bb27.i58.2.i.i
  %101 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.2.i.i, !dbg !8782
  %_83.i.2.i.i = load float, ptr %101, align 4, !dbg !8782, !alias.scope !8738, !noalias !8758, !noundef !12
  %_87.i.2.i.i = icmp samesign ult i64 %right27.i.2.i.i, %_66.1.i.i, !dbg !8783
  br i1 %_87.i.2.i.i, label %bb31.i.2.i.i, label %panic32.i.i.i, !dbg !8783

bb31.i.2.i.i:                                     ; preds = %bb29.i59.2.i.i
  %102 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.2.i.i, !dbg !8783
  %_86.i.2.i.i = load float, ptr %102, align 4, !dbg !8783, !alias.scope !8738, !noalias !8758, !noundef !12
  %_89.i.2.i.i = icmp samesign ult i64 %right27.i.2.i.i, %_64.1.i.i, !dbg !8785
  br i1 %_89.i.2.i.i, label %bb33.i60.2.i.i, label %panic34.i.i.i, !dbg !8785

bb33.i60.2.i.i:                                   ; preds = %bb31.i.2.i.i
  %103 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.2.i.i, !dbg !8785
  %_88.i.2.i.i = load float, ptr %103, align 4, !dbg !8785, !alias.scope !8736, !noalias !8757, !noundef !12
  %_68.i.3.i.i = load i32, ptr %49, align 4, !dbg !8786, !alias.scope !8742, !noalias !8743, !noundef !12
  %_67.i52.3.i.i = sub i32 %now.i.i.i, %_68.i.3.i.i, !dbg !8787
  %_66.i.3.i.i = and i32 %_67.i52.3.i.i, %_58.i.i, !dbg !8789
  %_65.i.3.i.i = zext i32 %_66.i.3.i.i to i64, !dbg !8781
  %_64.i53.3.i.i = shl nuw nsw i64 %_65.i.3.i.i, 2, !dbg !8781
  %left25.i.3.i.i = or disjoint i64 %_64.i53.3.i.i, 3, !dbg !8781
  %_76.i54.3.i.i = load i32, ptr %50, align 4, !dbg !8790, !alias.scope !8747, !noalias !8748, !noundef !12
  %_75.i.3.i.i = sub i32 %now.i.i.i, %_76.i54.3.i.i, !dbg !8791
  %_74.i55.3.i.i = and i32 %_75.i.3.i.i, %_58.i.i, !dbg !8793
  %_73.i56.3.i.i = zext i32 %_74.i55.3.i.i to i64, !dbg !8784
  %_72.i.3.i.i = shl nuw nsw i64 %_73.i56.3.i.i, 2, !dbg !8784
  %right27.i.3.i.i = or disjoint i64 %_72.i.3.i.i, 3, !dbg !8784
  %_81.i57.3.i.i = icmp samesign ult i64 %left25.i.3.i.i, %_64.1.i.i, !dbg !8750
  br i1 %_81.i57.3.i.i, label %bb27.i58.3.i.i, label %panic28.i.i.i, !dbg !8750

bb27.i58.3.i.i:                                   ; preds = %bb33.i60.2.i.i
  %104 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %left25.i.3.i.i, !dbg !8750
  %_79.i.3.i.i = load float, ptr %104, align 4, !dbg !8750, !alias.scope !8736, !noalias !8757, !noundef !12
  %_85.i.3.i.i = icmp samesign ult i64 %left25.i.3.i.i, %_66.1.i.i, !dbg !8782
  br i1 %_85.i.3.i.i, label %bb29.i59.3.i.i, label %panic30.i.i.i, !dbg !8782

bb29.i59.3.i.i:                                   ; preds = %bb27.i58.3.i.i
  %105 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %left25.i.3.i.i, !dbg !8782
  %_83.i.3.i.i = load float, ptr %105, align 4, !dbg !8782, !alias.scope !8738, !noalias !8758, !noundef !12
  %_87.i.3.i.i = icmp samesign ult i64 %right27.i.3.i.i, %_66.1.i.i, !dbg !8783
  br i1 %_87.i.3.i.i, label %bb31.i.3.i.i, label %panic32.i.i.i, !dbg !8783

bb31.i.3.i.i:                                     ; preds = %bb29.i59.3.i.i
  %_89.i.3.i.i = icmp samesign ult i64 %right27.i.3.i.i, %_64.1.i.i, !dbg !8785
  br i1 %_89.i.3.i.i, label %bb33.i60.3.i.i, label %panic34.i.i.i, !dbg !8785

bb33.i60.3.i.i:                                   ; preds = %bb31.i.3.i.i
  %106 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %right27.i.3.i.i, !dbg !8783
  %_86.i.3.i.i = load float, ptr %106, align 4, !dbg !8783, !alias.scope !8738, !noalias !8758, !noundef !12
  %107 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %right27.i.3.i.i, !dbg !8785
  %_88.i.3.i.i = load float, ptr %107, align 4, !dbg !8785, !alias.scope !8736, !noalias !8757, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !8767

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i: ; preds = %bb33.i60.3.i.i, %bb19.i.3.i.i, %bb10.i79.3.i.i
  %taps.i.sroa.55.0.i.i = phi float [ %right_own.i.3.i.i, %bb10.i79.3.i.i ], [ %left_own16.i.3.i.i, %bb19.i.3.i.i ], [ %_88.i.3.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.52.0.i.i = phi float [ %right_own.i.2.i.i, %bb10.i79.3.i.i ], [ %left_own16.i.2.i.i, %bb19.i.3.i.i ], [ %_88.i.2.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.49.0.i.i = phi float [ %right_own.i.1.i.i, %bb10.i79.3.i.i ], [ %left_own16.i.1.i.i, %bb19.i.3.i.i ], [ %_88.i.1.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.441922.0.i.i = phi float [ %right_own.i.i.i, %bb10.i79.3.i.i ], [ %left_own16.i.i.i, %bb19.i.3.i.i ], [ %_88.i.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.41.0.i.i = phi float [ %right_own.i.3.i.i, %bb10.i79.3.i.i ], [ %right_own18.i.3.i.i, %bb19.i.3.i.i ], [ %_86.i.3.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.38.0.i.i = phi float [ %right_own.i.2.i.i, %bb10.i79.3.i.i ], [ %right_own18.i.2.i.i, %bb19.i.3.i.i ], [ %_86.i.2.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.35.0.i.i = phi float [ %right_own.i.1.i.i, %bb10.i79.3.i.i ], [ %right_own18.i.1.i.i, %bb19.i.3.i.i ], [ %_86.i.1.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.301917.0.i.i = phi float [ %right_own.i.i.i, %bb10.i79.3.i.i ], [ %right_own18.i.i.i, %bb19.i.3.i.i ], [ %_86.i.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.27.0.i.i = phi float [ %left_own.i.3.i.i, %bb10.i79.3.i.i ], [ %right_own18.i.3.i.i, %bb19.i.3.i.i ], [ %_83.i.3.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.24.0.i.i = phi float [ %left_own.i.2.i.i, %bb10.i79.3.i.i ], [ %right_own18.i.2.i.i, %bb19.i.3.i.i ], [ %_83.i.2.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.21.0.i.i = phi float [ %left_own.i.1.i.i, %bb10.i79.3.i.i ], [ %right_own18.i.1.i.i, %bb19.i.3.i.i ], [ %_83.i.1.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.161912.0.i.i = phi float [ %left_own.i.i.i, %bb10.i79.3.i.i ], [ %right_own18.i.i.i, %bb19.i.3.i.i ], [ %_83.i.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.13.0.i.i = phi float [ %left_own.i.3.i.i, %bb10.i79.3.i.i ], [ %left_own16.i.3.i.i, %bb19.i.3.i.i ], [ %_79.i.3.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.10.0.i.i = phi float [ %left_own.i.2.i.i, %bb10.i79.3.i.i ], [ %left_own16.i.2.i.i, %bb19.i.3.i.i ], [ %_79.i.2.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.7.0.i.i = phi float [ %left_own.i.1.i.i, %bb10.i79.3.i.i ], [ %left_own16.i.1.i.i, %bb19.i.3.i.i ], [ %_79.i.1.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %taps.i.sroa.0.0.i.i = phi float [ %left_own.i.i.i, %bb10.i79.3.i.i ], [ %left_own16.i.i.i, %bb19.i.3.i.i ], [ %_79.i.i.i, %bb33.i60.3.i.i ], !dbg !8740
  %_12.i112.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %51, align 16, !dbg !8794, !alias.scope !8512, !noalias !8801
  %108 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i112.i.sroa.0.0.copyload.i.i, i8 1), !dbg !8808
  %109 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i112.i.sroa.0.0.copyload.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !8833
  %_16.i108.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %13, align 16, !dbg !8846, !alias.scope !8512, !noalias !8801
  %_17.i107.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %52, align 16, !dbg !8848, !alias.scope !8512, !noalias !8801
  %110 = fadd <4 x float> %_16.i108.i.sroa.0.0.copyload.i.i, %_17.i107.i.sroa.0.0.copyload.i.i, !dbg !8849
  %_20.i104.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %53, align 16, !dbg !8859, !alias.scope !8512, !noalias !8801
  %111 = bitcast <4 x float> %109 to <4 x i32>, !dbg !8861
  %112 = icmp slt <4 x i32> %111, zeroinitializer, !dbg !8870
  %113 = select <4 x i1> %112, <4 x float> %_20.i104.i.sroa.0.0.copyload.i.i, <4 x float> %110, !dbg !8870
  %114 = bitcast <4 x float> %108 to <4 x i32>, !dbg !8876
  %115 = icmp slt <4 x i32> %114, zeroinitializer, !dbg !8880
  %116 = select <4 x i1> %115, <4 x float> %113, <4 x float> %_16.i108.i.sroa.0.0.copyload.i.i, !dbg !8880
  store <4 x float> %116, ptr %13, align 16, !dbg !8882, !alias.scope !8512, !noalias !8801
  %117 = select <4 x i1> %112, <4 x float> zeroinitializer, <4 x float> %_17.i107.i.sroa.0.0.copyload.i.i, !dbg !8883
  store <4 x float> %117, ptr %52, align 16, !dbg !8888, !alias.scope !8512, !noalias !8801
  %118 = fadd <4 x float> %_12.i112.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !8889
  %119 = select <4 x i1> %115, <4 x float> %118, <4 x float> %_12.i112.i.sroa.0.0.copyload.i.i, !dbg !8899
  store <4 x float> %119, ptr %51, align 16, !dbg !8904, !alias.scope !8512, !noalias !8801
  %_12.i112.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %54, align 16, !dbg !8794, !alias.scope !8512, !noalias !8801
  %120 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i112.i.sroa.0.0.copyload.1.i.i, i8 1), !dbg !8808
  %121 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i112.i.sroa.0.0.copyload.1.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !8833
  %_16.i108.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i120.i.1.i.i, align 16, !dbg !8846, !alias.scope !8512, !noalias !8801
  %_17.i107.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %55, align 16, !dbg !8848, !alias.scope !8512, !noalias !8801
  %122 = fadd <4 x float> %_16.i108.i.sroa.0.0.copyload.1.i.i, %_17.i107.i.sroa.0.0.copyload.1.i.i, !dbg !8849
  %_20.i104.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %56, align 16, !dbg !8859, !alias.scope !8512, !noalias !8801
  %123 = bitcast <4 x float> %121 to <4 x i32>, !dbg !8861
  %124 = icmp slt <4 x i32> %123, zeroinitializer, !dbg !8870
  %125 = select <4 x i1> %124, <4 x float> %_20.i104.i.sroa.0.0.copyload.1.i.i, <4 x float> %122, !dbg !8870
  %126 = bitcast <4 x float> %120 to <4 x i32>, !dbg !8876
  %127 = icmp slt <4 x i32> %126, zeroinitializer, !dbg !8880
  %128 = select <4 x i1> %127, <4 x float> %125, <4 x float> %_16.i108.i.sroa.0.0.copyload.1.i.i, !dbg !8880
  store <4 x float> %128, ptr %iter1.sroa.0.0.ptr.i120.i.1.i.i, align 16, !dbg !8882, !alias.scope !8512, !noalias !8801
  %129 = select <4 x i1> %124, <4 x float> zeroinitializer, <4 x float> %_17.i107.i.sroa.0.0.copyload.1.i.i, !dbg !8883
  store <4 x float> %129, ptr %55, align 16, !dbg !8888, !alias.scope !8512, !noalias !8801
  %130 = fadd <4 x float> %_12.i112.i.sroa.0.0.copyload.1.i.i, splat (float -1.000000e+00), !dbg !8889
  %131 = select <4 x i1> %127, <4 x float> %130, <4 x float> %_12.i112.i.sroa.0.0.copyload.1.i.i, !dbg !8899
  store <4 x float> %131, ptr %54, align 16, !dbg !8904, !alias.scope !8512, !noalias !8801
  %_12.i112.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %57, align 16, !dbg !8794, !alias.scope !8512, !noalias !8801
  %132 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i112.i.sroa.0.0.copyload.2.i.i, i8 1), !dbg !8808
  %133 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i112.i.sroa.0.0.copyload.2.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !8833
  %_16.i108.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i120.i.2.i.i, align 16, !dbg !8846, !alias.scope !8512, !noalias !8801
  %_17.i107.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %58, align 16, !dbg !8848, !alias.scope !8512, !noalias !8801
  %134 = fadd <4 x float> %_16.i108.i.sroa.0.0.copyload.2.i.i, %_17.i107.i.sroa.0.0.copyload.2.i.i, !dbg !8849
  %_20.i104.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %59, align 16, !dbg !8859, !alias.scope !8512, !noalias !8801
  %135 = bitcast <4 x float> %133 to <4 x i32>, !dbg !8861
  %136 = icmp slt <4 x i32> %135, zeroinitializer, !dbg !8870
  %137 = select <4 x i1> %136, <4 x float> %_20.i104.i.sroa.0.0.copyload.2.i.i, <4 x float> %134, !dbg !8870
  %138 = bitcast <4 x float> %132 to <4 x i32>, !dbg !8876
  %139 = icmp slt <4 x i32> %138, zeroinitializer, !dbg !8880
  %140 = select <4 x i1> %139, <4 x float> %137, <4 x float> %_16.i108.i.sroa.0.0.copyload.2.i.i, !dbg !8880
  store <4 x float> %140, ptr %iter1.sroa.0.0.ptr.i120.i.2.i.i, align 16, !dbg !8882, !alias.scope !8512, !noalias !8801
  %141 = select <4 x i1> %136, <4 x float> zeroinitializer, <4 x float> %_17.i107.i.sroa.0.0.copyload.2.i.i, !dbg !8883
  store <4 x float> %141, ptr %58, align 16, !dbg !8888, !alias.scope !8512, !noalias !8801
  %142 = fadd <4 x float> %_12.i112.i.sroa.0.0.copyload.2.i.i, splat (float -1.000000e+00), !dbg !8889
  %143 = select <4 x i1> %139, <4 x float> %142, <4 x float> %_12.i112.i.sroa.0.0.copyload.2.i.i, !dbg !8899
  store <4 x float> %143, ptr %57, align 16, !dbg !8904, !alias.scope !8512, !noalias !8801
  %_12.i112.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %60, align 16, !dbg !8794, !alias.scope !8512, !noalias !8801
  %144 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i112.i.sroa.0.0.copyload.3.i.i, i8 1), !dbg !8808
  %145 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i112.i.sroa.0.0.copyload.3.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !8833
  %_16.i108.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i120.i.3.i.i, align 16, !dbg !8846, !alias.scope !8512, !noalias !8801
  %_17.i107.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %61, align 16, !dbg !8848, !alias.scope !8512, !noalias !8801
  %146 = fadd <4 x float> %_16.i108.i.sroa.0.0.copyload.3.i.i, %_17.i107.i.sroa.0.0.copyload.3.i.i, !dbg !8849
  %_20.i104.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %62, align 16, !dbg !8859, !alias.scope !8512, !noalias !8801
  %147 = bitcast <4 x float> %145 to <4 x i32>, !dbg !8861
  %148 = icmp slt <4 x i32> %147, zeroinitializer, !dbg !8870
  %149 = select <4 x i1> %148, <4 x float> %_20.i104.i.sroa.0.0.copyload.3.i.i, <4 x float> %146, !dbg !8870
  %150 = bitcast <4 x float> %144 to <4 x i32>, !dbg !8876
  %151 = icmp slt <4 x i32> %150, zeroinitializer, !dbg !8880
  %152 = select <4 x i1> %151, <4 x float> %149, <4 x float> %_16.i108.i.sroa.0.0.copyload.3.i.i, !dbg !8880
  store <4 x float> %152, ptr %iter1.sroa.0.0.ptr.i120.i.3.i.i, align 16, !dbg !8882, !alias.scope !8512, !noalias !8801
  %153 = select <4 x i1> %148, <4 x float> zeroinitializer, <4 x float> %_17.i107.i.sroa.0.0.copyload.3.i.i, !dbg !8883
  store <4 x float> %153, ptr %61, align 16, !dbg !8888, !alias.scope !8512, !noalias !8801
  %154 = fadd <4 x float> %_12.i112.i.sroa.0.0.copyload.3.i.i, splat (float -1.000000e+00), !dbg !8889
  %155 = select <4 x i1> %151, <4 x float> %154, <4 x float> %_12.i112.i.sroa.0.0.copyload.3.i.i, !dbg !8899
  store <4 x float> %155, ptr %60, align 16, !dbg !8904, !alias.scope !8512, !noalias !8801
  %_41.i83.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %31, align 16, !dbg !8905, !alias.scope !8512, !noalias !8913
  %156 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_41.i83.i.sroa.0.0.copyload.i.i, i8 1), !dbg !8915
  %157 = bitcast <4 x float> %156 to <4 x i32>, !dbg !8921
  %158 = icmp slt <4 x i32> %157, zeroinitializer, !dbg !8925
  %159 = insertelement <4 x float> poison, float %taps.i.sroa.0.0.i.i, i64 0, !dbg !8927
  %160 = insertelement <4 x float> %159, float %taps.i.sroa.7.0.i.i, i64 1, !dbg !8927
  %161 = insertelement <4 x float> %160, float %taps.i.sroa.10.0.i.i, i64 2, !dbg !8927
  %162 = insertelement <4 x float> %161, float %taps.i.sroa.13.0.i.i, i64 3, !dbg !8927
  %163 = bitcast <4 x float> %162 to <2 x i64>, !dbg !8931
  %164 = and <2 x i64> %163, splat (i64 9223372034707292159), !dbg !8932
  %165 = bitcast <2 x i64> %164 to <4 x float>, !dbg !8947
  %166 = fmul <4 x float> %165, splat (float 5.000000e-01), !dbg !8948
  %167 = insertelement <4 x float> poison, float %taps.i.sroa.161912.0.i.i, i64 0, !dbg !8958
  %168 = insertelement <4 x float> %167, float %taps.i.sroa.21.0.i.i, i64 1, !dbg !8958
  %169 = insertelement <4 x float> %168, float %taps.i.sroa.24.0.i.i, i64 2, !dbg !8958
  %170 = insertelement <4 x float> %169, float %taps.i.sroa.27.0.i.i, i64 3, !dbg !8958
  %171 = bitcast <4 x float> %170 to <2 x i64>, !dbg !8963
  %172 = and <2 x i64> %171, splat (i64 9223372034707292159), !dbg !8964
  %173 = bitcast <2 x i64> %172 to <4 x float>, !dbg !8970
  %174 = fmul <4 x float> %173, splat (float 5.000000e-01), !dbg !8971
  %175 = fadd <4 x float> %174, %166, !dbg !8976
  %_37.i87.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %30, align 16, !dbg !8981, !alias.scope !8512, !noalias !8913
  %176 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_37.i87.i.sroa.0.0.copyload.i.i, i8 1), !dbg !8982
  %177 = bitcast <4 x float> %176 to <4 x i32>, !dbg !8988
  %178 = icmp slt <4 x i32> %177, zeroinitializer, !dbg !8992
  %179 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %165, <4 x float> %173), !dbg !8994
  %180 = select <4 x i1> %178, <4 x float> %179, <4 x float> %165, !dbg !8992
  %181 = select <4 x i1> %158, <4 x float> %175, <4 x float> %180, !dbg !8925
  %182 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %181, <4 x float> splat (float 0x3E45798EE0000000)), !dbg !9003
  %183 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %182, <4 x float> splat (float 0x3810000000000000)), !dbg !9008
  %184 = bitcast <4 x float> %183 to <2 x i64>, !dbg !9017
  %185 = and <2 x i64> %184, splat (i64 36028792732385279), !dbg !9018
  %186 = or disjoint <2 x i64> %185, splat (i64 4575657222473777152), !dbg !9036
  %187 = bitcast <2 x i64> %186 to <4 x float>, !dbg !9045
  %188 = fadd <4 x float> %187, splat (float -1.000000e+00), !dbg !9046
  %189 = fmul <4 x float> %188, splat (float 0x3F9B17A960000000), !dbg !9052
  %190 = fsub <4 x float> splat (float 0x3FBF9A8440000000), %189, !dbg !9060
  %191 = fmul <4 x float> %188, %190, !dbg !9052
  %192 = fadd <4 x float> %191, splat (float 0xBFD1E3F400000000), !dbg !9060
  %193 = fmul <4 x float> %188, %192, !dbg !9052
  %194 = fadd <4 x float> %193, splat (float 0x3FDD544F20000000), !dbg !9060
  %195 = fmul <4 x float> %188, %194, !dbg !9052
  %196 = fadd <4 x float> %195, splat (float 0xBFE6FC2A60000000), !dbg !9060
  %197 = fmul <4 x float> %188, %196, !dbg !9052
  %198 = fadd <4 x float> %197, splat (float 0x3FF714B2A0000000), !dbg !9060
  %199 = bitcast <4 x float> %183 to <4 x i32>, !dbg !9065
  %_3.i887.i.i = lshr <4 x i32> %199, splat (i32 23), !dbg !9075
  %200 = bitcast <4 x i32> %_3.i887.i.i to <2 x i64>, !dbg !9076
  %201 = or disjoint <2 x i64> %200, splat (i64 5404319554102886400), !dbg !9077
  %202 = bitcast <2 x i64> %201 to <4 x float>, !dbg !9083
  %203 = fadd <4 x float> %202, splat (float 0xC160000FE0000000), !dbg !9084
  %204 = bitcast <4 x float> %140 to <2 x i64>, !dbg !9090
  %205 = fmul <4 x float> %188, %198, !dbg !9091
  %206 = fadd <4 x float> %203, %205, !dbg !9096
  %207 = fmul <4 x float> %206, splat (float 0x4018151820000000), !dbg !9101
  %208 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %207, <4 x float> splat (float 2.400000e+01)), !dbg !9106
  %209 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %208, <4 x float> splat (float -1.600000e+02)), !dbg !9115
  %_55.i69.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %29, align 16, !dbg !9120, !alias.scope !8512, !noalias !8801
  %210 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_55.i69.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9122
  %211 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %116, <4 x float> %209, i8 2), !dbg !9128
  %212 = fsub <4 x float> %116, %152, !dbg !9141
  %213 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %212, <4 x float> %209, i8 2), !dbg !9147
  %214 = bitcast <4 x float> %210 to <2 x i64>, !dbg !9153
  %215 = xor <2 x i64> %214, splat (i64 -1), !dbg !9165
  %216 = bitcast <4 x float> %211 to <2 x i64>, !dbg !9170
  %217 = and <2 x i64> %216, %215, !dbg !9177
  %218 = bitcast <4 x float> %213 to <2 x i64>, !dbg !9179
  %219 = and <2 x i64> %218, %214, !dbg !9184
  %220 = or <2 x i64> %219, %217, !dbg !9186
  %221 = xor <2 x i64> %218, splat (i64 -1), !dbg !9199
  %_67.i57.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %32, align 16, !dbg !9206, !alias.scope !8512, !noalias !8801
  %222 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_67.i57.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9207
  %223 = bitcast <4 x float> %222 to <2 x i64>, !dbg !9213
  %224 = and <2 x i64> %221, %223, !dbg !9217
  %225 = and <2 x i64> %224, %214, !dbg !9217
  %226 = or <2 x i64> %225, %220, !dbg !9222
  %227 = bitcast <2 x i64> %226 to <4 x i32>, !dbg !9228
  %228 = icmp slt <4 x i32> %227, zeroinitializer, !dbg !9232
  %229 = select <4 x i1> %228, <4 x float> splat (float 1.000000e+00), <4 x float> zeroinitializer, !dbg !9232
  %_71.i53.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %33, align 16, !dbg !9234, !alias.scope !8512, !noalias !8913
  %230 = fadd <4 x float> %_67.i57.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !9236
  %231 = bitcast <2 x i64> %225 to <4 x i32>, !dbg !9241
  %232 = icmp slt <4 x i32> %231, zeroinitializer, !dbg !9245
  %233 = select <4 x i1> %232, <4 x float> %230, <4 x float> %_67.i57.i.sroa.0.0.copyload.i.i, !dbg !9245
  %234 = bitcast <2 x i64> %220 to <4 x i32>, !dbg !9247
  %235 = icmp slt <4 x i32> %234, zeroinitializer, !dbg !9251
  %236 = select <4 x i1> %235, <4 x float> %_71.i53.i.sroa.0.0.copyload.i.i, <4 x float> %233, !dbg !9251
  store <4 x float> %236, ptr %32, align 16, !dbg !9253, !alias.scope !8512, !noalias !8801
  store <4 x float> %229, ptr %29, align 16, !dbg !9254, !alias.scope !8512, !noalias !8801
  %_86.i38.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %34, align 16, !dbg !9255, !alias.scope !8512, !noalias !8801
  %_88.i37.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %35, align 16, !dbg !9258, !alias.scope !8512, !noalias !8913
  %_7.i787.i.i = load <4 x float>, ptr %24, align 16, !dbg !9259, !alias.scope !9261, !noalias !9264
  %237 = fadd <4 x float> %128, splat (float -1.000000e+00), !dbg !9268
  %238 = fsub <4 x float> %209, %116, !dbg !9273
  %239 = fmul <4 x float> %237, %238, !dbg !9278
  %240 = xor <2 x i64> %204, splat (i64 -9223372034707292160), !dbg !9283
  %241 = bitcast <2 x i64> %240 to <4 x float>, !dbg !9292
  %242 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %239, <4 x float> %241), !dbg !9293
  %243 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %242, <4 x float> zeroinitializer), !dbg !9298
  %244 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %229, i8 1), !dbg !9303
  %245 = bitcast <4 x float> %244 to <4 x i32>, !dbg !9309
  %246 = icmp slt <4 x i32> %245, zeroinitializer, !dbg !9313
  %247 = select <4 x i1> %246, <4 x float> zeroinitializer, <4 x float> %243, !dbg !9313
  %248 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_86.i38.i.sroa.0.0.copyload.i.i, <4 x float> %247, i8 1), !dbg !9315
  %249 = bitcast <4 x float> %248 to <4 x i32>, !dbg !9321
  %250 = icmp slt <4 x i32> %249, zeroinitializer, !dbg !9324
  %251 = select <4 x i1> %250, <4 x float> %_7.i787.i.i, <4 x float> %_88.i37.i.sroa.0.0.copyload.i.i, !dbg !9324
  %252 = fsub <4 x float> %247, %_86.i38.i.sroa.0.0.copyload.i.i, !dbg !9326
  %253 = fmul <4 x float> %252, %251, !dbg !9332
  %254 = fadd <4 x float> %_86.i38.i.sroa.0.0.copyload.i.i, %253, !dbg !9340
  %255 = bitcast <4 x float> %254 to <2 x i64>, !dbg !9347
  %256 = and <2 x i64> %255, splat (i64 9223372034707292159), !dbg !9354
  %257 = bitcast <2 x i64> %256 to <4 x float>, !dbg !9347
  %258 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %257, <4 x float> splat (float 0x3BC79CA100000000), i8 1), !dbg !9356
  %259 = bitcast <4 x float> %258 to <2 x i64>, !dbg !9368
  %260 = xor <2 x i64> %259, splat (i64 -1), !dbg !9377
  %261 = and <2 x i64> %255, %260, !dbg !9379
  store <2 x i64> %261, ptr %34, align 16, !dbg !9385, !alias.scope !8512, !noalias !8801
  %262 = bitcast <2 x i64> %261 to <4 x float>, !dbg !9387
  %263 = fmul <4 x float> %262, splat (float 0x3FC542A5A0000000), !dbg !9388
  %264 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %263, <4 x float> splat (float -1.260000e+02)), !dbg !9395
  %265 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %264, <4 x float> splat (float 1.270000e+02)), !dbg !9402
  %266 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %265), !dbg !9407
  %267 = fsub <4 x float> %265, %266, !dbg !9417
  %268 = fmul <4 x float> %267, splat (float 0x3F5E974FA0000000), !dbg !9423
  %269 = fadd <4 x float> %268, splat (float 0x3F82778560000000), !dbg !9431
  %270 = fmul <4 x float> %267, %269, !dbg !9423
  %271 = fadd <4 x float> %270, splat (float 0x3FAC91CE60000000), !dbg !9431
  %272 = fmul <4 x float> %267, %271, !dbg !9423
  %273 = fadd <4 x float> %272, splat (float 0x3FCEBDB560000000), !dbg !9431
  %274 = fmul <4 x float> %267, %273, !dbg !9423
  %275 = fadd <4 x float> %274, splat (float 0x3FE62E4BA0000000), !dbg !9431
  %_98.i27.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %36, align 16, !dbg !9436, !alias.scope !8512, !noalias !8913
  %_12.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %63, align 16, !dbg !9438, !alias.scope !8512, !noalias !9441
  %276 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9448
  %277 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i.i.sroa.0.0.copyload.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !9454
  %_16.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %data.i.i.i.i, align 16, !dbg !9460, !alias.scope !8512, !noalias !9441
  %_17.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %64, align 16, !dbg !9461, !alias.scope !8512, !noalias !9441
  %278 = fadd <4 x float> %_16.i.i.sroa.0.0.copyload.i.i, %_17.i.i.sroa.0.0.copyload.i.i, !dbg !9462
  %_20.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %65, align 16, !dbg !9467, !alias.scope !8512, !noalias !9441
  %279 = bitcast <4 x float> %277 to <4 x i32>, !dbg !9468
  %280 = icmp slt <4 x i32> %279, zeroinitializer, !dbg !9472
  %281 = select <4 x i1> %280, <4 x float> %_20.i.i.sroa.0.0.copyload.i.i, <4 x float> %278, !dbg !9472
  %282 = bitcast <4 x float> %276 to <4 x i32>, !dbg !9474
  %283 = icmp slt <4 x i32> %282, zeroinitializer, !dbg !9478
  %284 = select <4 x i1> %283, <4 x float> %281, <4 x float> %_16.i.i.sroa.0.0.copyload.i.i, !dbg !9478
  store <4 x float> %284, ptr %data.i.i.i.i, align 16, !dbg !9480, !alias.scope !8512, !noalias !9441
  %285 = select <4 x i1> %280, <4 x float> zeroinitializer, <4 x float> %_17.i.i.sroa.0.0.copyload.i.i, !dbg !9481
  store <4 x float> %285, ptr %64, align 16, !dbg !9486, !alias.scope !8512, !noalias !9441
  %286 = fadd <4 x float> %_12.i.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !9487
  %287 = select <4 x i1> %283, <4 x float> %286, <4 x float> %_12.i.i.sroa.0.0.copyload.i.i, !dbg !9492
  store <4 x float> %287, ptr %63, align 16, !dbg !9497, !alias.scope !8512, !noalias !9441
  %_12.i.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %66, align 16, !dbg !9438, !alias.scope !8512, !noalias !9441
  %288 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i.i.sroa.0.0.copyload.1.i.i, i8 1), !dbg !9448
  %289 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i.i.sroa.0.0.copyload.1.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !9454
  %_16.i.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i.i.1.i.i, align 16, !dbg !9460, !alias.scope !8512, !noalias !9441
  %_17.i.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %67, align 16, !dbg !9461, !alias.scope !8512, !noalias !9441
  %290 = fadd <4 x float> %_16.i.i.sroa.0.0.copyload.1.i.i, %_17.i.i.sroa.0.0.copyload.1.i.i, !dbg !9462
  %_20.i.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %68, align 16, !dbg !9467, !alias.scope !8512, !noalias !9441
  %291 = bitcast <4 x float> %289 to <4 x i32>, !dbg !9468
  %292 = icmp slt <4 x i32> %291, zeroinitializer, !dbg !9472
  %293 = select <4 x i1> %292, <4 x float> %_20.i.i.sroa.0.0.copyload.1.i.i, <4 x float> %290, !dbg !9472
  %294 = bitcast <4 x float> %288 to <4 x i32>, !dbg !9474
  %295 = icmp slt <4 x i32> %294, zeroinitializer, !dbg !9478
  %296 = select <4 x i1> %295, <4 x float> %293, <4 x float> %_16.i.i.sroa.0.0.copyload.1.i.i, !dbg !9478
  store <4 x float> %296, ptr %iter1.sroa.0.0.ptr.i.i.1.i.i, align 16, !dbg !9480, !alias.scope !8512, !noalias !9441
  %297 = select <4 x i1> %292, <4 x float> zeroinitializer, <4 x float> %_17.i.i.sroa.0.0.copyload.1.i.i, !dbg !9481
  store <4 x float> %297, ptr %67, align 16, !dbg !9486, !alias.scope !8512, !noalias !9441
  %298 = fadd <4 x float> %_12.i.i.sroa.0.0.copyload.1.i.i, splat (float -1.000000e+00), !dbg !9487
  %299 = select <4 x i1> %295, <4 x float> %298, <4 x float> %_12.i.i.sroa.0.0.copyload.1.i.i, !dbg !9492
  store <4 x float> %299, ptr %66, align 16, !dbg !9497, !alias.scope !8512, !noalias !9441
  %_12.i.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %69, align 16, !dbg !9438, !alias.scope !8512, !noalias !9441
  %300 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i.i.sroa.0.0.copyload.2.i.i, i8 1), !dbg !9448
  %301 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i.i.sroa.0.0.copyload.2.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !9454
  %_16.i.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i.i.2.i.i, align 16, !dbg !9460, !alias.scope !8512, !noalias !9441
  %_17.i.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %70, align 16, !dbg !9461, !alias.scope !8512, !noalias !9441
  %302 = fadd <4 x float> %_16.i.i.sroa.0.0.copyload.2.i.i, %_17.i.i.sroa.0.0.copyload.2.i.i, !dbg !9462
  %_20.i.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %71, align 16, !dbg !9467, !alias.scope !8512, !noalias !9441
  %303 = bitcast <4 x float> %301 to <4 x i32>, !dbg !9468
  %304 = icmp slt <4 x i32> %303, zeroinitializer, !dbg !9472
  %305 = select <4 x i1> %304, <4 x float> %_20.i.i.sroa.0.0.copyload.2.i.i, <4 x float> %302, !dbg !9472
  %306 = bitcast <4 x float> %300 to <4 x i32>, !dbg !9474
  %307 = icmp slt <4 x i32> %306, zeroinitializer, !dbg !9478
  %308 = select <4 x i1> %307, <4 x float> %305, <4 x float> %_16.i.i.sroa.0.0.copyload.2.i.i, !dbg !9478
  store <4 x float> %308, ptr %iter1.sroa.0.0.ptr.i.i.2.i.i, align 16, !dbg !9480, !alias.scope !8512, !noalias !9441
  %309 = select <4 x i1> %304, <4 x float> zeroinitializer, <4 x float> %_17.i.i.sroa.0.0.copyload.2.i.i, !dbg !9481
  store <4 x float> %309, ptr %70, align 16, !dbg !9486, !alias.scope !8512, !noalias !9441
  %310 = fadd <4 x float> %_12.i.i.sroa.0.0.copyload.2.i.i, splat (float -1.000000e+00), !dbg !9487
  %311 = select <4 x i1> %307, <4 x float> %310, <4 x float> %_12.i.i.sroa.0.0.copyload.2.i.i, !dbg !9492
  store <4 x float> %311, ptr %69, align 16, !dbg !9497, !alias.scope !8512, !noalias !9441
  %_12.i.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %72, align 16, !dbg !9438, !alias.scope !8512, !noalias !9441
  %312 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i.i.sroa.0.0.copyload.3.i.i, i8 1), !dbg !9448
  %313 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i.i.sroa.0.0.copyload.3.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !9454
  %_16.i.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i.i.3.i.i, align 16, !dbg !9460, !alias.scope !8512, !noalias !9441
  %_17.i.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %73, align 16, !dbg !9461, !alias.scope !8512, !noalias !9441
  %314 = fadd <4 x float> %_16.i.i.sroa.0.0.copyload.3.i.i, %_17.i.i.sroa.0.0.copyload.3.i.i, !dbg !9462
  %_20.i.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %74, align 16, !dbg !9467, !alias.scope !8512, !noalias !9441
  %315 = bitcast <4 x float> %313 to <4 x i32>, !dbg !9468
  %316 = icmp slt <4 x i32> %315, zeroinitializer, !dbg !9472
  %317 = select <4 x i1> %316, <4 x float> %_20.i.i.sroa.0.0.copyload.3.i.i, <4 x float> %314, !dbg !9472
  %318 = bitcast <4 x float> %312 to <4 x i32>, !dbg !9474
  %319 = icmp slt <4 x i32> %318, zeroinitializer, !dbg !9478
  %320 = select <4 x i1> %319, <4 x float> %317, <4 x float> %_16.i.i.sroa.0.0.copyload.3.i.i, !dbg !9478
  store <4 x float> %320, ptr %iter1.sroa.0.0.ptr.i.i.3.i.i, align 16, !dbg !9480, !alias.scope !8512, !noalias !9441
  %321 = select <4 x i1> %316, <4 x float> zeroinitializer, <4 x float> %_17.i.i.sroa.0.0.copyload.3.i.i, !dbg !9481
  store <4 x float> %321, ptr %73, align 16, !dbg !9486, !alias.scope !8512, !noalias !9441
  %322 = fadd <4 x float> %_12.i.i.sroa.0.0.copyload.3.i.i, splat (float -1.000000e+00), !dbg !9487
  %323 = select <4 x i1> %319, <4 x float> %322, <4 x float> %_12.i.i.sroa.0.0.copyload.3.i.i, !dbg !9492
  store <4 x float> %323, ptr %72, align 16, !dbg !9497, !alias.scope !8512, !noalias !9441
  %324 = fmul <4 x float> %267, %275, !dbg !9498
  %325 = fadd <4 x float> %324, splat (float 1.000000e+00), !dbg !9503
  %326 = fadd <4 x float> %266, splat (float 0x4160000FE0000000), !dbg !9508
  %327 = bitcast <4 x float> %326 to <4 x i32>, !dbg !9517
  %_3.i888.i.i = shl <4 x i32> %327, splat (i32 23), !dbg !9527
  %328 = bitcast <4 x i32> %_3.i888.i.i to <4 x float>, !dbg !9528
  %329 = fmul <4 x float> %325, %328, !dbg !9530
  %330 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %262, <4 x float> zeroinitializer, i8 0), !dbg !9534
  %331 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_98.i27.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9540
  %332 = bitcast <4 x float> %330 to <2 x i64>, !dbg !9546
  %333 = bitcast <4 x float> %331 to <2 x i64>, !dbg !9546
  %334 = or <2 x i64> %333, %332, !dbg !9550
  %335 = fmul <4 x float> %lanes.i471.sroa.0.0.copyload.i.i, %329, !dbg !9552
  %336 = bitcast <2 x i64> %334 to <4 x i32>, !dbg !9558
  %337 = icmp slt <4 x i32> %336, zeroinitializer, !dbg !9562
  %338 = select <4 x i1> %337, <4 x float> %lanes.i471.sroa.0.0.copyload.i.i, <4 x float> %335, !dbg !9562
  %_41.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %39, align 16, !dbg !9564, !alias.scope !8512, !noalias !9565
  %339 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_41.i.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9567
  %340 = bitcast <4 x float> %339 to <4 x i32>, !dbg !9573
  %341 = icmp slt <4 x i32> %340, zeroinitializer, !dbg !9577
  %342 = insertelement <4 x float> poison, float %taps.i.sroa.301917.0.i.i, i64 0, !dbg !9579
  %343 = insertelement <4 x float> %342, float %taps.i.sroa.35.0.i.i, i64 1, !dbg !9579
  %344 = insertelement <4 x float> %343, float %taps.i.sroa.38.0.i.i, i64 2, !dbg !9579
  %345 = insertelement <4 x float> %344, float %taps.i.sroa.41.0.i.i, i64 3, !dbg !9579
  %346 = bitcast <4 x float> %345 to <2 x i64>, !dbg !9584
  %347 = and <2 x i64> %346, splat (i64 9223372034707292159), !dbg !9585
  %348 = bitcast <2 x i64> %347 to <4 x float>, !dbg !9591
  %349 = fmul <4 x float> %348, splat (float 5.000000e-01), !dbg !9592
  %350 = insertelement <4 x float> poison, float %taps.i.sroa.441922.0.i.i, i64 0, !dbg !9597
  %351 = insertelement <4 x float> %350, float %taps.i.sroa.49.0.i.i, i64 1, !dbg !9597
  %352 = insertelement <4 x float> %351, float %taps.i.sroa.52.0.i.i, i64 2, !dbg !9597
  %353 = insertelement <4 x float> %352, float %taps.i.sroa.55.0.i.i, i64 3, !dbg !9597
  %354 = bitcast <4 x float> %353 to <2 x i64>, !dbg !9602
  %355 = and <2 x i64> %354, splat (i64 9223372034707292159), !dbg !9603
  %356 = bitcast <2 x i64> %355 to <4 x float>, !dbg !9609
  %357 = fmul <4 x float> %356, splat (float 5.000000e-01), !dbg !9610
  %358 = fadd <4 x float> %357, %349, !dbg !9615
  %_37.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %38, align 16, !dbg !9620, !alias.scope !8512, !noalias !9565
  %359 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_37.i.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9621
  %360 = bitcast <4 x float> %359 to <4 x i32>, !dbg !9627
  %361 = icmp slt <4 x i32> %360, zeroinitializer, !dbg !9631
  %362 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %348, <4 x float> %356), !dbg !9633
  %363 = select <4 x i1> %361, <4 x float> %362, <4 x float> %348, !dbg !9631
  %364 = select <4 x i1> %341, <4 x float> %358, <4 x float> %363, !dbg !9577
  %365 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %364, <4 x float> splat (float 0x3E45798EE0000000)), !dbg !9638
  %366 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %365, <4 x float> splat (float 0x3810000000000000)), !dbg !9643
  %367 = bitcast <4 x float> %366 to <2 x i64>, !dbg !9650
  %368 = and <2 x i64> %367, splat (i64 36028792732385279), !dbg !9651
  %369 = or disjoint <2 x i64> %368, splat (i64 4575657222473777152), !dbg !9656
  %370 = bitcast <2 x i64> %369 to <4 x float>, !dbg !9660
  %371 = fadd <4 x float> %370, splat (float -1.000000e+00), !dbg !9661
  %372 = fmul <4 x float> %371, splat (float 0x3F9B17A960000000), !dbg !9666
  %373 = fsub <4 x float> splat (float 0x3FBF9A8440000000), %372, !dbg !9671
  %374 = fmul <4 x float> %371, %373, !dbg !9666
  %375 = fadd <4 x float> %374, splat (float 0xBFD1E3F400000000), !dbg !9671
  %376 = fmul <4 x float> %371, %375, !dbg !9666
  %377 = fadd <4 x float> %376, splat (float 0x3FDD544F20000000), !dbg !9671
  %378 = fmul <4 x float> %371, %377, !dbg !9666
  %379 = fadd <4 x float> %378, splat (float 0xBFE6FC2A60000000), !dbg !9671
  %380 = fmul <4 x float> %371, %379, !dbg !9666
  %381 = fadd <4 x float> %380, splat (float 0x3FF714B2A0000000), !dbg !9671
  %382 = bitcast <4 x float> %366 to <4 x i32>, !dbg !9676
  %_3.i891.i.i = lshr <4 x i32> %382, splat (i32 23), !dbg !9680
  %383 = bitcast <4 x i32> %_3.i891.i.i to <2 x i64>, !dbg !9681
  %384 = or disjoint <2 x i64> %383, splat (i64 5404319554102886400), !dbg !9682
  %385 = bitcast <2 x i64> %384 to <4 x float>, !dbg !9686
  %386 = fadd <4 x float> %385, splat (float 0xC160000FE0000000), !dbg !9687
  %387 = bitcast <4 x float> %308 to <2 x i64>, !dbg !9691
  %388 = fmul <4 x float> %371, %381, !dbg !9692
  %389 = fadd <4 x float> %386, %388, !dbg !9697
  %390 = fmul <4 x float> %389, splat (float 0x4018151820000000), !dbg !9702
  %391 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %390, <4 x float> splat (float 2.400000e+01)), !dbg !9707
  %392 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %391, <4 x float> splat (float -1.600000e+02)), !dbg !9712
  %_55.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %37, align 16, !dbg !9717, !alias.scope !8512, !noalias !9441
  %393 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_55.i.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9718
  %394 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %284, <4 x float> %392, i8 2), !dbg !9724
  %395 = fsub <4 x float> %284, %320, !dbg !9730
  %396 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %395, <4 x float> %392, i8 2), !dbg !9735
  %397 = bitcast <4 x float> %393 to <2 x i64>, !dbg !9741
  %398 = xor <2 x i64> %397, splat (i64 -1), !dbg !9746
  %399 = bitcast <4 x float> %394 to <2 x i64>, !dbg !9748
  %400 = and <2 x i64> %399, %398, !dbg !9752
  %401 = bitcast <4 x float> %396 to <2 x i64>, !dbg !9754
  %402 = and <2 x i64> %401, %397, !dbg !9758
  %403 = or <2 x i64> %402, %400, !dbg !9760
  %404 = xor <2 x i64> %401, splat (i64 -1), !dbg !9765
  %_67.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %40, align 16, !dbg !9771, !alias.scope !8512, !noalias !9441
  %405 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_67.i.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9772
  %406 = bitcast <4 x float> %405 to <2 x i64>, !dbg !9778
  %407 = and <2 x i64> %404, %406, !dbg !9782
  %408 = and <2 x i64> %407, %397, !dbg !9782
  %409 = or <2 x i64> %408, %403, !dbg !9787
  %410 = bitcast <2 x i64> %409 to <4 x i32>, !dbg !9792
  %411 = icmp slt <4 x i32> %410, zeroinitializer, !dbg !9796
  %412 = select <4 x i1> %411, <4 x float> splat (float 1.000000e+00), <4 x float> zeroinitializer, !dbg !9796
  %_71.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %41, align 16, !dbg !9798, !alias.scope !8512, !noalias !9565
  %413 = fadd <4 x float> %_67.i.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !9799
  %414 = bitcast <2 x i64> %408 to <4 x i32>, !dbg !9804
  %415 = icmp slt <4 x i32> %414, zeroinitializer, !dbg !9808
  %416 = select <4 x i1> %415, <4 x float> %413, <4 x float> %_67.i.i.sroa.0.0.copyload.i.i, !dbg !9808
  %417 = bitcast <2 x i64> %403 to <4 x i32>, !dbg !9810
  %418 = icmp slt <4 x i32> %417, zeroinitializer, !dbg !9814
  %419 = select <4 x i1> %418, <4 x float> %_71.i.i.sroa.0.0.copyload.i.i, <4 x float> %416, !dbg !9814
  store <4 x float> %419, ptr %40, align 16, !dbg !9816, !alias.scope !8512, !noalias !9441
  store <4 x float> %412, ptr %37, align 16, !dbg !9817, !alias.scope !8512, !noalias !9441
  %_86.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %42, align 16, !dbg !9818, !alias.scope !8512, !noalias !9441
  %_88.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %43, align 16, !dbg !9819, !alias.scope !8512, !noalias !9565
  %_7.i739.i.i = load <4 x float>, ptr %_38.i.i, align 16, !dbg !9820, !alias.scope !9822, !noalias !9825
  %420 = fadd <4 x float> %296, splat (float -1.000000e+00), !dbg !9829
  %421 = fsub <4 x float> %392, %284, !dbg !9834
  %422 = fmul <4 x float> %420, %421, !dbg !9839
  %423 = xor <2 x i64> %387, splat (i64 -9223372034707292160), !dbg !9844
  %424 = bitcast <2 x i64> %423 to <4 x float>, !dbg !9849
  %425 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %422, <4 x float> %424), !dbg !9850
  %426 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %425, <4 x float> zeroinitializer), !dbg !9855
  %427 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %412, i8 1), !dbg !9860
  %428 = bitcast <4 x float> %427 to <4 x i32>, !dbg !9866
  %429 = icmp slt <4 x i32> %428, zeroinitializer, !dbg !9870
  %430 = select <4 x i1> %429, <4 x float> zeroinitializer, <4 x float> %426, !dbg !9870
  %431 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_86.i.i.sroa.0.0.copyload.i.i, <4 x float> %430, i8 1), !dbg !9872
  %432 = bitcast <4 x float> %431 to <4 x i32>, !dbg !9878
  %433 = icmp slt <4 x i32> %432, zeroinitializer, !dbg !9881
  %434 = select <4 x i1> %433, <4 x float> %_7.i739.i.i, <4 x float> %_88.i.i.sroa.0.0.copyload.i.i, !dbg !9881
  %435 = fsub <4 x float> %430, %_86.i.i.sroa.0.0.copyload.i.i, !dbg !9883
  %436 = fmul <4 x float> %435, %434, !dbg !9888
  %437 = fadd <4 x float> %_86.i.i.sroa.0.0.copyload.i.i, %436, !dbg !9893
  %438 = bitcast <4 x float> %437 to <2 x i64>, !dbg !9897
  %439 = and <2 x i64> %438, splat (i64 9223372034707292159), !dbg !9903
  %440 = bitcast <2 x i64> %439 to <4 x float>, !dbg !9897
  %441 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %440, <4 x float> splat (float 0x3BC79CA100000000), i8 1), !dbg !9905
  %442 = bitcast <4 x float> %441 to <2 x i64>, !dbg !9911
  %443 = xor <2 x i64> %442, splat (i64 -1), !dbg !9916
  %444 = and <2 x i64> %438, %443, !dbg !9918
  store <2 x i64> %444, ptr %42, align 16, !dbg !9922, !alias.scope !8512, !noalias !9441
  %445 = bitcast <2 x i64> %444 to <4 x float>, !dbg !9923
  %446 = fmul <4 x float> %445, splat (float 0x3FC542A5A0000000), !dbg !9924
  %447 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %446, <4 x float> splat (float -1.260000e+02)), !dbg !9930
  %448 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %447, <4 x float> splat (float 1.270000e+02)), !dbg !9936
  %449 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %448), !dbg !9941
  %450 = fsub <4 x float> %448, %449, !dbg !9946
  %451 = fmul <4 x float> %450, splat (float 0x3F5E974FA0000000), !dbg !9951
  %452 = fadd <4 x float> %451, splat (float 0x3F82778560000000), !dbg !9956
  %453 = fmul <4 x float> %450, %452, !dbg !9951
  %454 = fadd <4 x float> %453, splat (float 0x3FAC91CE60000000), !dbg !9956
  %455 = fmul <4 x float> %450, %454, !dbg !9951
  %456 = fadd <4 x float> %455, splat (float 0x3FCEBDB560000000), !dbg !9956
  %457 = fmul <4 x float> %450, %456, !dbg !9951
  %458 = fadd <4 x float> %457, splat (float 0x3FE62E4BA0000000), !dbg !9956
  %459 = fmul <4 x float> %450, %458, !dbg !9961
  %460 = fadd <4 x float> %459, splat (float 1.000000e+00), !dbg !9966
  %461 = fadd <4 x float> %449, splat (float 0x4160000FE0000000), !dbg !9971
  %462 = bitcast <4 x float> %461 to <4 x i32>, !dbg !9976
  %_3.i892.i.i = shl <4 x i32> %462, splat (i32 23), !dbg !9980
  %463 = bitcast <4 x i32> %_3.i892.i.i to <4 x float>, !dbg !9981
  %464 = fmul <4 x float> %460, %463, !dbg !9983
  %465 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %445, <4 x float> zeroinitializer, i8 0), !dbg !9987
  %_98.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %44, align 16, !dbg !9993, !alias.scope !8512, !noalias !9565
  %466 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_98.i.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9994
  %467 = bitcast <4 x float> %465 to <2 x i64>, !dbg !10000
  %468 = bitcast <4 x float> %466 to <2 x i64>, !dbg !10000
  %469 = or <2 x i64> %468, %467, !dbg !10004
  %470 = fmul <4 x float> %lanes.i465.sroa.0.0.copyload.i.i, %464, !dbg !10006
  %471 = bitcast <2 x i64> %469 to <4 x i32>, !dbg !10011
  %472 = icmp slt <4 x i32> %471, zeroinitializer, !dbg !10015
  %473 = select <4 x i1> %472, <4 x float> %lanes.i465.sroa.0.0.copyload.i.i, <4 x float> %470, !dbg !10015
  store <4 x float> %338, ptr %_97.i.i.i, align 4, !dbg !10017, !alias.scope !10023, !noalias !10027
  store <4 x float> %473, ptr %_115.i.i.i, align 4, !dbg !10031, !alias.scope !10036, !noalias !10040
  %exitcond1907.not.i.i = icmp eq i64 %75, %..i.i, !dbg !10044
  br i1 %exitcond1907.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKb1_EB3_.exit.i, label %bb30.i.i.i, !dbg !8595

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKb1_EB3_.exit.i: ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i
  %_81.i.i.i = add i32 %base.i.i.i, %20, !dbg !10047
  store i32 %_81.i.i.i, ptr %_57.i.i, align 4, !dbg !10049, !alias.scope !8512, !noalias !8592
  br label %bb4.i, !dbg !10050

bb7.i:                                            ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKB1r_EB3_.exit.i, %bb4.i
  %474 = load i32, ptr %14, align 4, !dbg !10051, !alias.scope !8439, !noalias !8453, !noundef !12
  %475 = sub i32 %474, %20, !dbg !10051
  store i32 %475, ptr %14, align 4, !dbg !10051, !alias.scope !8439, !noalias !8453
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10052), !dbg !10055
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10056), !dbg !10055
  call void @llvm.lifetime.start.p0(ptr nonnull %iter.i.i), !dbg !10058, !noalias !10063
  store i64 0, ptr %iter.i.i, align 8, !dbg !10058, !noalias !10063
  %_7.sroa.0.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 8, !dbg !10058
  store i64 2, ptr %_7.sroa.0.sroa.2.0.iter.sroa_idx.i.i, align 8, !dbg !10058, !noalias !10063
  %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 16, !dbg !10058
  store ptr %_37.0, ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, align 8, !dbg !10058, !noalias !10063
  %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 24, !dbg !10058
  store i64 %_37.1, ptr %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i, align 8, !dbg !10058, !noalias !10063
  %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 32, !dbg !10058
  store ptr %_38.0, ptr %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i, align 8, !dbg !10058, !noalias !10063
  %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 40, !dbg !10058
  store i64 %_38.1, ptr %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i, align 8, !dbg !10058, !noalias !10063
  %_7.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 48, !dbg !10058
  store i64 0, ptr %_7.sroa.2.0.iter.sroa_idx.i.i, align 8, !dbg !10058, !noalias !10063
  %_71197.not.i.i = icmp eq i32 %_27, 0
  %476 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %477 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %478 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %479 = getelementptr inbounds nuw i8, ptr %self, i64 1776
  %480 = getelementptr inbounds nuw i8, ptr %self, i64 1816
  %481 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %482 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> zeroinitializer, i8 0)
  %483 = bitcast <4 x float> %482 to <2 x i64>
  br label %bb6.i5.i, !dbg !10066

bb6.i5.i:                                         ; preds = %bb1.backedge.i.i, %bb7.i
  %484 = phi i64 [ 24, %bb7.i ], [ 32, %bb1.backedge.i.i ]
  %_5.not.i.i.i.i.i = phi i1 [ false, %bb7.i ], [ true, %bb1.backedge.i.i ]
  %485 = phi i64 [ 0, %bb7.i ], [ 1, %bb1.backedge.i.i ]
  %self3.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<&mut [f32]>", ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, i64 %485, !dbg !10072
  %_14.0.i.i.i.i.i = load ptr, ptr %self3.i.i.i.i.i, align 8, !dbg !10077, !alias.scope !10081, !noalias !10088, !nonnull !12, !align !3533, !noundef !12
  %486 = getelementptr inbounds nuw i8, ptr %self3.i.i.i.i.i, i64 8, !dbg !10077
  %_14.1.i.i.i.i.i = load i64, ptr %486, align 8, !dbg !10077, !alias.scope !10081, !noalias !10088, !noundef !12
  %487 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x4_::f32x4>", ptr %self, i64 %485, !dbg !10090
  %488 = getelementptr inbounds nuw i8, ptr %487, i64 1376, !dbg !10090
  %_17.sroa.0.0.copyload.i.i = load <2 x i64>, ptr %488, align 16, !dbg !10090, !alias.scope !10092, !noalias !10093
  %fst_len.i.i.i = and i64 %_14.1.i.i.i.i.i, -4, !dbg !10094
  %_22.not.i26190.i.i = icmp eq i64 %fst_len.i.i.i, 0, !dbg !10105
  br i1 %_22.not.i26190.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %bb13.i27.i.i, !dbg !10105

bb13.i27.i.i:                                     ; preds = %bb6.i5.i, %bb13.i27.i.i
  %iter.sroa.0.0.i25193.i.i = phi ptr [ %_27.i28.i.i, %bb13.i27.i.i ], [ %_14.0.i.i.i.i.i, %bb6.i5.i ]
  %iter.sroa.5.0.i24192.i.i = phi i64 [ %_28.i29.i.i, %bb13.i27.i.i ], [ %fst_len.i.i.i, %bb6.i5.i ]
  %ok.i16.sroa.0.0191.i.i = phi <2 x i64> [ %493, %bb13.i27.i.i ], [ %483, %bb6.i5.i ]
  %lanes.i.sroa.0.0.copyload.i.i = load <2 x i64>, ptr %iter.sroa.0.0.i25193.i.i, align 4, !dbg !10112, !alias.scope !10118, !noalias !10122
  %_27.i28.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i25193.i.i, i64 16, !dbg !10126
  %_28.i29.i.i = add i64 %iter.sroa.5.0.i24192.i.i, -4, !dbg !10133
  %489 = and <2 x i64> %lanes.i.sroa.0.0.copyload.i.i, splat (i64 9223372034707292159), !dbg !10134
  %490 = bitcast <2 x i64> %489 to <4 x float>, !dbg !10141
  %491 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %490, <4 x float> splat (float 0x46293E5940000000), i8 1), !dbg !10142
  %492 = bitcast <4 x float> %491 to <2 x i64>, !dbg !10148
  %493 = and <2 x i64> %ok.i16.sroa.0.0191.i.i, %492, !dbg !10152
  %_22.not.i26.i.i = icmp eq i64 %_28.i29.i.i, 0, !dbg !10105
  br i1 %_22.not.i26.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %bb13.i27.i.i, !dbg !10105

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i: ; preds = %bb13.i27.i.i, %bb6.i5.i
  %ok.i16.sroa.0.0.lcssa.i.i = phi <2 x i64> [ %483, %bb6.i5.i ], [ %493, %bb13.i27.i.i ], !dbg !10154
  %494 = bitcast <2 x i64> %ok.i16.sroa.0.0.lcssa.i.i to <4 x i32>, !dbg !10155
  %495 = icmp sgt <4 x i32> %494, splat (i32 -1), !dbg !10164
  %496 = bitcast <4 x i1> %495 to i4, !dbg !10164
  %_0.i98.not.i.i = icmp eq i4 %496, 0, !dbg !10168
  br i1 %_0.i98.not.i.i, label %bb9.i.i, label %bb14.i.i, !dbg !10169

bb9.i.i:                                          ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  %497 = and <2 x i64> %_17.sroa.0.0.copyload.i.i, splat (i64 9223372034707292159), !dbg !10170
  %498 = bitcast <2 x i64> %497 to <4 x float>, !dbg !10177
  %499 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %498, <4 x float> splat (float 0x46293E5940000000), i8 1), !dbg !10178
  %500 = bitcast <4 x float> %499 to <2 x i64>, !dbg !10184
  %501 = and <2 x i64> %500, %483, !dbg !10188
  %502 = bitcast <2 x i64> %501 to <4 x i32>, !dbg !10190
  %503 = icmp sgt <4 x i32> %502, splat (i32 -1), !dbg !10195
  %504 = bitcast <4 x i1> %503 to i4, !dbg !10195
  %_0.i101.not.i.i = icmp eq i4 %504, 0, !dbg !10197
  br i1 %_0.i101.not.i.i, label %bb1.backedge.i.i, label %bb14.i.i, !dbg !10198

bb1.backedge.i.i:                                 ; preds = %bb17.backedge.i.i, %bb9.i.i
  br i1 %_5.not.i.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9run_blockB2_.exit, label %bb6.i5.i, !dbg !10066

bb14.i.i:                                         ; preds = %bb9.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  %fst_len.i.i.i.i = and i64 %_14.1.i.i.i.i.i, 2305843009213693948, !dbg !10199
  %_40.not71.i.i.i = icmp eq i64 %fst_len.i.i.i.i, 0, !dbg !10206
  br i1 %_40.not71.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i, label %bb23.i.i.i, !dbg !10206

bb24.loopexit.i.i.i:                              ; preds = %bb23.i.i.i
  %505 = bitcast <2 x i64> %510 to <4 x float>, !dbg !10213
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !10219

bb23.i.i.i:                                       ; preds = %bb14.i.i, %bb23.i.i.i
  %iter.sroa.0.074.i.i.i = phi ptr [ %_45.i.i.i, %bb23.i.i.i ], [ %_14.0.i.i.i.i.i, %bb14.i.i ]
  %iter.sroa.5.073.i.i.i = phi i64 [ %_46.i.i.i, %bb23.i.i.i ], [ %fst_len.i.i.i.i, %bb14.i.i ]
  %ok.sroa.0.072.i.i.i = phi <2 x i64> [ %510, %bb23.i.i.i ], [ %483, %bb14.i.i ]
  %lanes.i.sroa.0.0.copyload.i.i.i = load <2 x i64>, ptr %iter.sroa.0.074.i.i.i, align 4, !dbg !10220, !alias.scope !10225, !noalias !10231
  %_45.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i.i, i64 16, !dbg !10235
  %_46.i.i.i = add i64 %iter.sroa.5.073.i.i.i, -4, !dbg !10242
  %506 = and <2 x i64> %lanes.i.sroa.0.0.copyload.i.i.i, splat (i64 9223372034707292159), !dbg !10243
  %507 = bitcast <2 x i64> %506 to <4 x float>, !dbg !10249
  %508 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %507, <4 x float> splat (float 0x46293E5940000000), i8 1), !dbg !10250
  %509 = bitcast <4 x float> %508 to <2 x i64>, !dbg !10213
  %510 = and <2 x i64> %ok.sroa.0.072.i.i.i, %509, !dbg !10256
  %_40.not.i.i.i = icmp eq i64 %_46.i.i.i, 0, !dbg !10206
  br i1 %_40.not.i.i.i, label %bb24.loopexit.i.i.i, label %bb23.i.i.i, !dbg !10206

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i: ; preds = %bb24.loopexit.i.i.i, %bb14.i.i
  %ok.sroa.0.0.lcssa.i.i.i = phi <4 x float> [ %482, %bb14.i.i ], [ %505, %bb24.loopexit.i.i.i ], !dbg !10258
  %511 = and <2 x i64> %_17.sroa.0.0.copyload.i.i, splat (i64 9223372034707292159), !dbg !10259
  %512 = bitcast <2 x i64> %511 to <4 x float>, !dbg !10266
  %513 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %512, <4 x float> splat (float 0x46293E5940000000), i8 1), !dbg !10267
  %514 = bitcast <4 x float> %513 to <2 x i64>, !dbg !10273
  %515 = and <2 x i64> %514, %483, !dbg !10277
  %516 = bitcast <4 x float> %ok.sroa.0.0.lcssa.i.i.i to <4 x i32>, !dbg !10279
  %517 = icmp slt <4 x i32> %516, zeroinitializer, !dbg !10284
  %bc.i.i.i = select <4 x i1> %517, <4 x i32> zeroinitializer, <4 x i32> splat (i32 1065353216), !dbg !10284
  %518 = extractelement <4 x i32> %bc.i.i.i, i64 0, !dbg !10286
  %519 = icmp ne i32 %518, 0, !dbg !10286
  %520 = zext i1 %519 to i32, !dbg !10286
  %521 = extractelement <4 x i32> %bc.i.i.i, i64 1, !dbg !10286
  %522 = icmp eq i32 %521, 0, !dbg !10286
  %523 = select i1 %522, i32 0, i32 2, !dbg !10286
  %524 = extractelement <4 x i32> %bc.i.i.i, i64 2, !dbg !10286
  %525 = icmp eq i32 %524, 0, !dbg !10286
  %526 = select i1 %525, i32 0, i32 4, !dbg !10286
  %527 = extractelement <4 x i32> %bc.i.i.i, i64 3, !dbg !10286
  %528 = icmp eq i32 %527, 0, !dbg !10286
  %529 = select i1 %528, i32 0, i32 8, !dbg !10286
  %530 = bitcast <2 x i64> %515 to <4 x i32>, !dbg !10290
  %531 = icmp slt <4 x i32> %530, zeroinitializer, !dbg !10294
  %bc.i116.i.i = select <4 x i1> %531, <4 x i32> zeroinitializer, <4 x i32> splat (i32 1065353216), !dbg !10294
  %532 = extractelement <4 x i32> %bc.i116.i.i, i64 0, !dbg !10296
  %533 = icmp ne i32 %532, 0, !dbg !10296
  %534 = zext i1 %533 to i32, !dbg !10296
  %535 = extractelement <4 x i32> %bc.i116.i.i, i64 1, !dbg !10296
  %536 = icmp eq i32 %535, 0, !dbg !10296
  %537 = select i1 %536, i32 0, i32 2, !dbg !10296
  %538 = extractelement <4 x i32> %bc.i116.i.i, i64 2, !dbg !10296
  %539 = icmp eq i32 %538, 0, !dbg !10296
  %540 = select i1 %539, i32 0, i32 4, !dbg !10296
  %541 = extractelement <4 x i32> %bc.i116.i.i, i64 3, !dbg !10296
  %542 = icmp eq i32 %541, 0, !dbg !10296
  %543 = select i1 %542, i32 0, i32 8, !dbg !10296
  %mask.sroa.0.1.1.i118.i.i = or disjoint i32 %523, %520, !dbg !10296
  %mask.sroa.0.1.2.i120.i.i = or disjoint i32 %mask.sroa.0.1.1.i118.i.i, %526, !dbg !10296
  %mask.sroa.0.1.3.i122.i.i = or disjoint i32 %mask.sroa.0.1.2.i120.i.i, %529, !dbg !10296
  %mask.sroa.0.1.1.i.i.i = or i32 %mask.sroa.0.1.3.i122.i.i, %534, !dbg !10286
  %mask.sroa.0.1.2.i.i.i = or i32 %mask.sroa.0.1.1.i.i.i, %537, !dbg !10286
  %mask.sroa.0.1.3.i.i.i = or i32 %mask.sroa.0.1.2.i.i.i, %540, !dbg !10286
  %failed.i.i = or i32 %mask.sroa.0.1.3.i.i.i, %543, !dbg !10297
  %invariant.gep.i.i = getelementptr [8 x float], ptr %self, i64 %485, !dbg !10298
  %ring.i.i.i = getelementptr inbounds nuw %Ring, ptr %477, i64 %485
  %544 = getelementptr inbounds nuw i8, ptr %ring.i.i.i, i64 8
  %invariant.gep200.i.i = getelementptr %LaneTiming, ptr %478, i64 %485, !dbg !10301
  %545 = getelementptr inbounds nuw %Ring, ptr %self, i64 %485
  %546 = getelementptr inbounds nuw i8, ptr %545, i64 800
  %547 = getelementptr inbounds nuw %"kernel::GateCoef<wide::f32x4_::f32x4>", ptr %481, i64 %485
  %_19.i133.i.i = getelementptr inbounds nuw i8, ptr %547, i64 32
  %_25.i.i6.i = getelementptr inbounds nuw i8, ptr %547, i64 16
  %548 = getelementptr inbounds nuw %"kernel::GateCoef<wide::f32x4_::f32x4>", ptr %self, i64 %485
  %549 = getelementptr inbounds nuw i8, ptr %548, i64 928
  %550 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x4_::f32x4>", ptr %13, i64 %485
  %_17.i.i7.i = getelementptr inbounds nuw i8, ptr %550, i64 288
  %_19.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 256
  %_21.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 272
  %_37.i.i8.i = getelementptr inbounds nuw i8, ptr %550, i64 16
  %_39.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 32
  %_41.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 48
  %iter.sroa.0.0.ptr27.1.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 64
  %_37.1.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 80
  %_39.1.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 96
  %_41.1.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 112
  %iter.sroa.0.0.ptr27.2.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 128
  %_37.2.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 144
  %_39.2.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 160
  %_41.2.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 176
  %iter.sroa.0.0.ptr27.3.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 192
  %_37.3.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 208
  %_39.3.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 224
  %_41.3.i.i.i = getelementptr inbounds nuw i8, ptr %550, i64 240
  %551 = getelementptr inbounds nuw i8, ptr %reports, i64 %484
  br label %bb30.i.i, !dbg !10301

bb30.i.i:                                         ; preds = %bb17.backedge.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i
  %iter1.sroa.0.0199.i.i = phi i64 [ 0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i ], [ %552, %bb17.backedge.i.i ]
  %552 = add nuw nsw i64 %iter1.sroa.0.0199.i.i, 1, !dbg !10307
  %553 = trunc nuw nsw i64 %iter1.sroa.0.0199.i.i to i32, !dbg !10313
  %_35.i.i = shl nuw nsw i32 1, %553, !dbg !10313
  %_34.i.i = and i32 %_35.i.i, %failed.i.i, !dbg !10315
  %554 = icmp eq i32 %_34.i.i, 0, !dbg !10315
  br i1 %554, label %bb17.backedge.i.i, label %bb20.preheader.i.i, !dbg !10315

bb20.preheader.i.i:                               ; preds = %bb30.i.i
  br i1 %_71197.not.i.i, label %bb33.i.i, label %bb32.i.i, !dbg !10316

bb33.i.i:                                         ; preds = %bb21.i.i, %bb20.preheader.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10322), !dbg !10325
  %slots.i.i.i = load i64, ptr %476, align 16, !dbg !10328, !alias.scope !10332, !noalias !10093, !noundef !12
  %_193.not.i.i.i = icmp eq i64 %slots.i.i.i, 0, !dbg !10333
  br i1 %_193.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.lr.ph.i.i.i, !dbg !10344

bb7.lr.ph.i.i.i:                                  ; preds = %bb33.i.i
  %_23.1.i.i.i = load i64, ptr %544, align 8, !alias.scope !10332, !noalias !10093, !noundef !12
  br label %bb7.i.i.i, !dbg !10344

bb7.i.i.i:                                        ; preds = %bb3.i.i.i, %bb7.lr.ph.i.i.i
  %iter.sroa.0.04.i.i.i = phi i64 [ 0, %bb7.lr.ph.i.i.i ], [ %555, %bb3.i.i.i ]
  %_10.i.i.i = shl i64 %iter.sroa.0.04.i.i.i, 2, !dbg !10345
  %_9.i124.i.i = add nuw nsw i64 %_10.i.i.i, %iter1.sroa.0.0199.i.i, !dbg !10345
  %_12.i.i.i = icmp ult i64 %_9.i124.i.i, %_23.1.i.i.i, !dbg !10347
  br i1 %_12.i.i.i, label %bb3.i.i.i, label %panic1.i.i.i, !dbg !10347

bb3.i.i.i:                                        ; preds = %bb7.i.i.i
  %_23.0.i.i.i = load ptr, ptr %ring.i.i.i, align 8, !dbg !10347, !alias.scope !10332, !noalias !10093, !nonnull !12, !noundef !12
  %555 = add nuw i64 %iter.sroa.0.04.i.i.i, 1, !dbg !10348
  %556 = getelementptr inbounds nuw float, ptr %_23.0.i.i.i, i64 %_9.i124.i.i, !dbg !10347
  store float 0.000000e+00, ptr %556, align 4, !dbg !10347, !noalias !10354
  %exitcond.not.i.i.i = icmp eq i64 %555, %slots.i.i.i, !dbg !10333
  br i1 %exitcond.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.i.i.i, !dbg !10344

panic1.i.i.i:                                     ; preds = %bb7.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_9.i124.i.i, i64 noundef %_23.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_835aafef72e8508601474e7b1f4172a9) #24, !dbg !10347, !noalias !10354
  unreachable, !dbg !10347

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i: ; preds = %bb3.i.i.i, %bb33.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10355), !dbg !10358
  %gep.i.i = getelementptr [2 x [8 x float]], ptr %invariant.gep.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10359
  %values.sroa.0.0.copyload.i.i.i = load float, ptr %gep.i.i, align 16, !dbg !10359, !alias.scope !10362, !noalias !10093
  %values.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 4, !dbg !10359
  %values.sroa.5.0.copyload.i.i.i = load float, ptr %values.sroa.5.0..sroa_idx.i.i.i, align 4, !dbg !10359, !alias.scope !10362, !noalias !10093
  %values.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 8, !dbg !10359
  %values.sroa.6.0.copyload.i.i.i = load float, ptr %values.sroa.6.0..sroa_idx.i.i.i, align 8, !dbg !10359, !alias.scope !10362, !noalias !10093
  %values.sroa.7.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 12, !dbg !10359
  %values.sroa.7.0.copyload.i.i.i = load float, ptr %values.sroa.7.0..sroa_idx.i.i.i, align 4, !dbg !10359, !alias.scope !10362, !noalias !10093
  %values.sroa.8.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 16, !dbg !10359
  %values.sroa.8.0.copyload.i.i.i = load float, ptr %values.sroa.8.0..sroa_idx.i.i.i, align 16, !dbg !10359, !alias.scope !10362, !noalias !10093
  %values.sroa.9.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 20, !dbg !10359
  %values.sroa.9.0.copyload.i.i.i = load float, ptr %values.sroa.9.0..sroa_idx.i.i.i, align 4, !dbg !10359, !alias.scope !10362, !noalias !10093
  %values.sroa.10.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 24, !dbg !10359
  %values.sroa.10.0.copyload.i.i.i = load float, ptr %values.sroa.10.0..sroa_idx.i.i.i, align 8, !dbg !10359, !alias.scope !10362, !noalias !10093
  %values.sroa.11.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 28, !dbg !10359
  %values.sroa.11.0.copyload.i.i.i = load float, ptr %values.sroa.11.0..sroa_idx.i.i.i, align 4, !dbg !10359, !alias.scope !10362, !noalias !10093
  %gep201.i.i = getelementptr [2 x %LaneTiming], ptr %invariant.gep200.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10363
  store float %values.sroa.11.0.copyload.i.i.i, ptr %gep201.i.i, align 16, !dbg !10363, !alias.scope !10362, !noalias !10093
  %_7.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep201.i.i, i64 4, !dbg !10363
  store float %values.sroa.8.0.copyload.i.i.i, ptr %_7.sroa.4.0..sroa_idx.i.i.i, align 4, !dbg !10363, !alias.scope !10362, !noalias !10093
  %_7.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep201.i.i, i64 8, !dbg !10363
  store float %values.sroa.9.0.copyload.i.i.i, ptr %_7.sroa.5.0..sroa_idx.i.i.i, align 8, !dbg !10363, !alias.scope !10362, !noalias !10093
  %_7.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep201.i.i, i64 12, !dbg !10363
  store float %values.sroa.10.0.copyload.i.i.i, ptr %_7.sroa.6.0..sroa_idx.i.i.i, align 4, !dbg !10363, !alias.scope !10362, !noalias !10093
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10365), !dbg !10368
  %sample_rate.i.i.i = load i32, ptr %479, align 8, !dbg !10369, !alias.scope !10371, !noalias !10093, !noundef !12
  %_31.i.i11.i = fpext float %values.sroa.11.0.copyload.i.i.i to double, !dbg !10372
  %_32.i.i.i = uitofp i32 %sample_rate.i.i.i to double, !dbg !10375
  %_30.i.i12.i = fmul double %_31.i.i11.i, %_32.i.i.i, !dbg !10377
  %_29.i.i.i = fdiv double %_30.i.i12.i, 1.000000e+03, !dbg !10377
  %_28.i130.i.i = fadd double %_29.i.i.i, 5.000000e-01, !dbg !10378
  %557 = tail call double @llvm.floor.f64(double %_28.i130.i.i), !dbg !10379
  %or.cond.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %557, i32 527), !dbg !10382
  %_35.i.i13.i = fcmp ogt double %557, 0x41EFFFFFFFE00000
  %or.cond10.i.i.i = or i1 %or.cond.i.i.i, %_35.i.i13.i, !dbg !10382
  br i1 %or.cond10.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i, label %bb19.i.i14.i, !dbg !10382

bb19.i.i14.i:                                     ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i
  %_45.i131.i.i = fpext float %values.sroa.9.0.copyload.i.i.i to double, !dbg !10383
  %_44.i.i.i = fmul double %_45.i131.i.i, %_32.i.i.i, !dbg !10386
  %_43.i.i.i = fdiv double %_44.i.i.i, 1.000000e+03, !dbg !10386
  %_42.i.i.i = fadd double %_43.i.i.i, 5.000000e-01, !dbg !10387
  %558 = tail call double @llvm.floor.f64(double %_42.i.i.i), !dbg !10388
  %or.cond11.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %558, i32 527), !dbg !10391
  %_48.i.i.i = fcmp ogt double %558, 0x41EFFFFFFFE00000
  %or.cond12.i.i.i = or i1 %or.cond11.i.i.i, %_48.i.i.i, !dbg !10391
  br i1 %or.cond12.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i, label %bb20.3.i.i.i, !dbg !10391

bb20.3.i.i.i:                                     ; preds = %bb19.i.i14.i
  %_12.i132.i.i = load i32, ptr %480, align 8, !dbg !10392, !alias.scope !10371, !noalias !10093, !noundef !12
  %_36.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %557), !dbg !10393
  %_49.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %558), !dbg !10394
  %559 = getelementptr inbounds nuw i32, ptr %546, i64 %iter1.sroa.0.0199.i.i, !dbg !10395
  %560 = tail call i32 @llvm.usub.sat.i32(i32 %_12.i132.i.i, i32 %_36.i.i.i), !dbg !10395
  store i32 %560, ptr %559, align 4, !dbg !10395, !alias.scope !10371, !noalias !10093
  %_20.i.i.i = uitofp i32 %_49.i.i.i to float, !dbg !10396
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i129.i.i), !dbg !10397, !noalias !10399
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i.i129.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_19.i133.i.i, i64 16, i1 false), !dbg !10402, !noalias !10093
  %561 = getelementptr inbounds nuw float, ptr %words.i.i129.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10403
  store float %_20.i.i.i, ptr %561, align 4, !dbg !10403, !noalias !10399
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_19.i133.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i.i129.i.i, i64 16, i1 false), !dbg !10404, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i129.i.i), !dbg !10405, !noalias !10399
; call effect_runtime::envelope::attack_release_coefficient
  %_23.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.8.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !10406, !noalias !10407
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i128.i.i), !dbg !10408, !noalias !10410
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i128.i.i, ptr noundef nonnull align 16 dereferenceable(16) %547, i64 16, i1 false), !dbg !10413, !noalias !10093
  %562 = getelementptr inbounds nuw float, ptr %words.i17.i128.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10414
  store float %_23.i.i.i, ptr %562, align 4, !dbg !10414, !noalias !10410
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %547, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i128.i.i, i64 16, i1 false), !dbg !10415, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i128.i.i), !dbg !10416, !noalias !10410
; call effect_runtime::envelope::attack_release_coefficient
  %_26.i.i15.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.10.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !10417, !noalias !10407
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i127.i.i), !dbg !10418, !noalias !10420
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i127.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_25.i.i6.i, i64 16, i1 false), !dbg !10423, !noalias !10093
  %563 = getelementptr inbounds nuw float, ptr %words.i18.i127.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10424
  store float %_26.i.i15.i, ptr %563, align 4, !dbg !10424, !noalias !10420
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_25.i.i6.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i127.i.i, i64 16, i1 false), !dbg !10425, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i127.i.i), !dbg !10426, !noalias !10420
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i.i.i), !dbg !10427, !noalias !10429
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %549, i64 16, i1 false), !dbg !10432, !noalias !10093
  %564 = getelementptr inbounds nuw float, ptr %words.i.i.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10433
  %_0.i.i.i.i = load float, ptr %564, align 4, !dbg !10433, !noalias !10429, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i.i.i), !dbg !10434, !noalias !10429
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i12.i.i.i), !dbg !10435, !noalias !10438
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i12.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_17.i.i7.i, i64 16, i1 false), !dbg !10441, !noalias !10093
  %565 = getelementptr inbounds nuw float, ptr %words.i12.i.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10442
  store float 0.000000e+00, ptr %565, align 4, !dbg !10442, !noalias !10438
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_17.i.i7.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i12.i.i.i, i64 16, i1 false), !dbg !10443, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i12.i.i.i), !dbg !10444, !noalias !10438
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i13.i.i.i), !dbg !10445, !noalias !10447
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i13.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_19.i.i.i, i64 16, i1 false), !dbg !10450, !noalias !10093
  %566 = getelementptr inbounds nuw float, ptr %words.i13.i.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10451
  store float 1.000000e+00, ptr %566, align 4, !dbg !10451, !noalias !10447
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_19.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i13.i.i.i, i64 16, i1 false), !dbg !10452, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i13.i.i.i), !dbg !10453, !noalias !10447
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !10454, !noalias !10456
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i14.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_21.i.i.i, i64 16, i1 false), !dbg !10459, !noalias !10093
  %567 = getelementptr inbounds nuw float, ptr %words.i14.i.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10460
  store float %_0.i.i.i.i, ptr %567, align 4, !dbg !10460, !noalias !10456
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_21.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i14.i.i.i, i64 16, i1 false), !dbg !10461, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !10462, !noalias !10456
  %568 = getelementptr inbounds nuw float, ptr %words.i15.i.i.i, i64 %iter1.sroa.0.0199.i.i
  %569 = getelementptr inbounds nuw float, ptr %words.i16.i.i.i, i64 %iter1.sroa.0.0199.i.i
  %570 = getelementptr inbounds nuw float, ptr %words.i17.i.i.i, i64 %iter1.sroa.0.0199.i.i
  %571 = getelementptr inbounds nuw float, ptr %words.i18.i.i.i, i64 %iter1.sroa.0.0199.i.i
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !10463, !noalias !10467
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %550, i64 16, i1 false), !dbg !10470, !noalias !10093
  store float %values.sroa.0.0.copyload.i.i.i, ptr %568, align 4, !dbg !10471, !noalias !10467
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %550, ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, i64 16, i1 false), !dbg !10472, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !10473, !noalias !10467
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !10474, !noalias !10476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_37.i.i8.i, i64 16, i1 false), !dbg !10479, !noalias !10093
  store float %values.sroa.0.0.copyload.i.i.i, ptr %569, align 4, !dbg !10480, !noalias !10476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_37.i.i8.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, i64 16, i1 false), !dbg !10481, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !10482, !noalias !10476
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !10483, !noalias !10485
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_39.i.i.i, i64 16, i1 false), !dbg !10488, !noalias !10093
  store float 0.000000e+00, ptr %570, align 4, !dbg !10489, !noalias !10485
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_39.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, i64 16, i1 false), !dbg !10490, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !10491, !noalias !10485
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !10492, !noalias !10494
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_41.i.i.i, i64 16, i1 false), !dbg !10497, !noalias !10093
  store float 0.000000e+00, ptr %571, align 4, !dbg !10498, !noalias !10494
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_41.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, i64 16, i1 false), !dbg !10499, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !10500, !noalias !10494
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !10463, !noalias !10467
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.1.i.i.i, i64 16, i1 false), !dbg !10470, !noalias !10093
  store float %values.sroa.5.0.copyload.i.i.i, ptr %568, align 4, !dbg !10471, !noalias !10467
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.1.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, i64 16, i1 false), !dbg !10472, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !10473, !noalias !10467
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !10474, !noalias !10476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_37.1.i.i.i, i64 16, i1 false), !dbg !10479, !noalias !10093
  store float %values.sroa.5.0.copyload.i.i.i, ptr %569, align 4, !dbg !10480, !noalias !10476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_37.1.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, i64 16, i1 false), !dbg !10481, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !10482, !noalias !10476
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !10483, !noalias !10485
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_39.1.i.i.i, i64 16, i1 false), !dbg !10488, !noalias !10093
  store float 0.000000e+00, ptr %570, align 4, !dbg !10489, !noalias !10485
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_39.1.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, i64 16, i1 false), !dbg !10490, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !10491, !noalias !10485
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !10492, !noalias !10494
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_41.1.i.i.i, i64 16, i1 false), !dbg !10497, !noalias !10093
  store float 0.000000e+00, ptr %571, align 4, !dbg !10498, !noalias !10494
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_41.1.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, i64 16, i1 false), !dbg !10499, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !10500, !noalias !10494
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !10463, !noalias !10467
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.2.i.i.i, i64 16, i1 false), !dbg !10470, !noalias !10093
  store float %values.sroa.6.0.copyload.i.i.i, ptr %568, align 4, !dbg !10471, !noalias !10467
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.2.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, i64 16, i1 false), !dbg !10472, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !10473, !noalias !10467
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !10474, !noalias !10476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_37.2.i.i.i, i64 16, i1 false), !dbg !10479, !noalias !10093
  store float %values.sroa.6.0.copyload.i.i.i, ptr %569, align 4, !dbg !10480, !noalias !10476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_37.2.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, i64 16, i1 false), !dbg !10481, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !10482, !noalias !10476
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !10483, !noalias !10485
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_39.2.i.i.i, i64 16, i1 false), !dbg !10488, !noalias !10093
  store float 0.000000e+00, ptr %570, align 4, !dbg !10489, !noalias !10485
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_39.2.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, i64 16, i1 false), !dbg !10490, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !10491, !noalias !10485
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !10492, !noalias !10494
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_41.2.i.i.i, i64 16, i1 false), !dbg !10497, !noalias !10093
  store float 0.000000e+00, ptr %571, align 4, !dbg !10498, !noalias !10494
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_41.2.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, i64 16, i1 false), !dbg !10499, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !10500, !noalias !10494
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !10463, !noalias !10467
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.3.i.i.i, i64 16, i1 false), !dbg !10470, !noalias !10093
  store float %values.sroa.7.0.copyload.i.i.i, ptr %568, align 4, !dbg !10471, !noalias !10467
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.3.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, i64 16, i1 false), !dbg !10472, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !10473, !noalias !10467
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !10474, !noalias !10476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_37.3.i.i.i, i64 16, i1 false), !dbg !10479, !noalias !10093
  store float %values.sroa.7.0.copyload.i.i.i, ptr %569, align 4, !dbg !10480, !noalias !10476
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_37.3.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, i64 16, i1 false), !dbg !10481, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !10482, !noalias !10476
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !10483, !noalias !10485
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_39.3.i.i.i, i64 16, i1 false), !dbg !10488, !noalias !10093
  store float 0.000000e+00, ptr %570, align 4, !dbg !10489, !noalias !10485
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_39.3.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, i64 16, i1 false), !dbg !10490, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !10491, !noalias !10485
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !10492, !noalias !10494
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_41.3.i.i.i, i64 16, i1 false), !dbg !10497, !noalias !10093
  store float 0.000000e+00, ptr %571, align 4, !dbg !10498, !noalias !10494
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_41.3.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, i64 16, i1 false), !dbg !10499, !noalias !10093
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !10500, !noalias !10494
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i, !dbg !10501

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i: ; preds = %bb20.3.i.i.i, %bb19.i.i14.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i
  %gep203.i.i = getelementptr inbounds nuw %"effect_contract::ProcessReport", ptr %551, i64 %iter1.sroa.0.0199.i.i, !dbg !10502
  %_46.i.i = load i64, ptr %gep203.i.i, align 8, !dbg !10504, !alias.scope !10506, !noalias !10507, !noundef !12
  %572 = tail call i64 @llvm.uadd.sat.i64(i64 %_46.i.i, i64 range(i64 0, 4294967296) %frames), !dbg !10508
  store i64 %572, ptr %gep203.i.i, align 8, !dbg !10511, !alias.scope !10506, !noalias !10507
  br label %bb17.backedge.i.i, !dbg !10298

bb17.backedge.i.i:                                ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i, %bb30.i.i
  %exitcond228.not.i.i = icmp eq i64 %552, 4, !dbg !10512
  br i1 %exitcond228.not.i.i, label %bb1.backedge.i.i, label %bb30.i.i, !dbg !10301

bb32.i.i:                                         ; preds = %bb20.preheader.i.i, %bb21.i.i
  %iter2.sroa.0.0198.i.i = phi i64 [ %573, %bb21.i.i ], [ 0, %bb20.preheader.i.i ]
  %_39.i.i = shl nuw nsw i64 %iter2.sroa.0.0198.i.i, 2, !dbg !10515
  %_38.i9.i = add nuw nsw i64 %_39.i.i, %iter1.sroa.0.0199.i.i, !dbg !10515
  %_41.i.i = icmp ult i64 %_38.i9.i, %_14.1.i.i.i.i.i, !dbg !10517
  br i1 %_41.i.i, label %bb21.i.i, label %panic4.i.i, !dbg !10517

bb21.i.i:                                         ; preds = %bb32.i.i
  %573 = add nuw nsw i64 %iter2.sroa.0.0198.i.i, 1, !dbg !10518
  %574 = getelementptr inbounds nuw float, ptr %_14.0.i.i.i.i.i, i64 %_38.i9.i, !dbg !10517
  store float 0.000000e+00, ptr %574, align 4, !dbg !10517, !noalias !10524
  %exitcond.not.i10.i = icmp eq i64 %573, %frames, !dbg !10525
  br i1 %exitcond.not.i10.i, label %bb33.i.i, label %bb32.i.i, !dbg !10316

panic4.i.i:                                       ; preds = %bb32.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_38.i9.i, i64 noundef %_14.1.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6797264598a169e4722ae66c7bc497b8) #24, !dbg !10517, !noalias !10524
  unreachable, !dbg !10517

bb20.i:                                           ; preds = %bb4.i
  %575 = shl nuw nsw i64 %..i.i, 2, !dbg !10528
  %_52.i = icmp samesign ugt i64 %575, %_37.1, !dbg !10529
  br i1 %_52.i, label %bb24.i, label %bb25.i, !dbg !10529, !prof !180

bb25.i:                                           ; preds = %bb20.i
  %_60.i = icmp samesign ugt i64 %575, %_38.1, !dbg !10537
  br i1 %_60.i, label %bb26.i, label %bb27.i, !dbg !10537, !prof !180

bb24.i:                                           ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %575, i64 noundef range(i64 0, 2305843009213693952) %_37.1, i64 noundef range(i64 0, 2305843009213693952) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e79f4c3c2f015f90ab54e70b61044ab) #24, !dbg !10541, !noalias !8453
  unreachable, !dbg !10541

bb27.i:                                           ; preds = %bb25.i
  %_59.i = getelementptr inbounds nuw float, ptr %_37.0, i64 %575, !dbg !10542
  %_55.i = sub nuw nsw i64 %_37.1, %575, !dbg !10547
  %_63.i = sub nuw nsw i64 %_38.1, %575, !dbg !10548
  %_67.i = getelementptr inbounds nuw float, ptr %_38.0, i64 %575, !dbg !10549
  %_26.i = sub nsw i64 %frames, %..i.i, !dbg !10554
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10555), !dbg !10558
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10559), !dbg !10558
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10561), !dbg !10558
  %data.i.i.i23.i = getelementptr inbounds nuw i8, ptr %self, i64 1392, !dbg !10563
  %_15.i24.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !10570
  %data.i.i817.i.i = getelementptr inbounds nuw i8, ptr %self, i64 832, !dbg !10572
  %576 = getelementptr inbounds nuw i8, ptr %self, i64 1772, !dbg !10577
  %_29.i25.i = load i32, ptr %576, align 4, !dbg !10577, !range !1335, !alias.scope !10581, !noalias !10582, !noundef !12
  %_31.i26.i = getelementptr inbounds nuw i8, ptr %self, i64 800, !dbg !10584
  %_33.i27.i = getelementptr inbounds nuw i8, ptr %self, i64 864, !dbg !10585
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10586), !dbg !10589
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10590), !dbg !10589
  %577 = icmp eq i32 %_29.i25.i, 1, !dbg !10592
  br i1 %577, label %bb7.i32.i, label %bb1.i.i.preheader.i28.i, !dbg !10592

bb1.i.i.preheader.i28.i:                          ; preds = %bb27.i
  %.val.i.i.i29.i = load i32, ptr %_31.i26.i, align 4, !dbg !10594, !alias.scope !10597, !noalias !10598, !noundef !12
  %.val1.i.i.i30.i = load i32, ptr %_33.i27.i, align 4, !dbg !10594, !alias.scope !10601, !noalias !10602, !noundef !12
  %_0.i.i.not.i.i.i31.i = icmp eq i32 %.val.i.i.i29.i, %.val1.i.i.i30.i, !dbg !10603
  br i1 %_0.i.i.not.i.i.i31.i, label %bb1.i.i.1.i342.i, label %bb7.i32.i, !dbg !10594

bb1.i.i.1.i342.i:                                 ; preds = %bb1.i.i.preheader.i28.i
  %_3.i.i.i.i.i.1.i343.i = getelementptr inbounds nuw i8, ptr %self, i64 804, !dbg !10608
  %_3.i1.i.i.i.i.1.i344.i = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !10613
  %.val.i.i.1.i345.i = load i32, ptr %_3.i.i.i.i.i.1.i343.i, align 4, !dbg !10594, !alias.scope !10597, !noalias !10598, !noundef !12
  %.val1.i.i.1.i346.i = load i32, ptr %_3.i1.i.i.i.i.1.i344.i, align 4, !dbg !10594, !alias.scope !10601, !noalias !10602, !noundef !12
  %_0.i.i.not.i.i.1.i347.i = icmp eq i32 %.val.i.i.1.i345.i, %.val1.i.i.1.i346.i, !dbg !10603
  br i1 %_0.i.i.not.i.i.1.i347.i, label %bb1.i.i.2.i348.i, label %bb7.i32.i, !dbg !10594

bb1.i.i.2.i348.i:                                 ; preds = %bb1.i.i.1.i342.i
  %_3.i.i.i.i.i.2.i349.i = getelementptr inbounds nuw i8, ptr %self, i64 808, !dbg !10608
  %_3.i1.i.i.i.i.2.i350.i = getelementptr inbounds nuw i8, ptr %self, i64 872, !dbg !10613
  %.val.i.i.2.i351.i = load i32, ptr %_3.i.i.i.i.i.2.i349.i, align 4, !dbg !10594, !alias.scope !10597, !noalias !10598, !noundef !12
  %.val1.i.i.2.i352.i = load i32, ptr %_3.i1.i.i.i.i.2.i350.i, align 4, !dbg !10594, !alias.scope !10601, !noalias !10602, !noundef !12
  %_0.i.i.not.i.i.2.i353.i = icmp eq i32 %.val.i.i.2.i351.i, %.val1.i.i.2.i352.i, !dbg !10603
  br i1 %_0.i.i.not.i.i.2.i353.i, label %bb1.i.i.3.i354.i, label %bb7.i32.i, !dbg !10594

bb1.i.i.3.i354.i:                                 ; preds = %bb1.i.i.2.i348.i
  %_3.i.i.i.i.i.3.i355.i = getelementptr inbounds nuw i8, ptr %self, i64 812, !dbg !10608
  %_3.i1.i.i.i.i.3.i356.i = getelementptr inbounds nuw i8, ptr %self, i64 876, !dbg !10613
  %.val.i.i.3.i357.i = load i32, ptr %_3.i.i.i.i.i.3.i355.i, align 4, !dbg !10594, !alias.scope !10597, !noalias !10598, !noundef !12
  %.val1.i.i.3.i358.i = load i32, ptr %_3.i1.i.i.i.i.3.i356.i, align 4, !dbg !10594, !alias.scope !10601, !noalias !10602, !noundef !12
  %_0.i.i.not.i.i.3.i359.i = icmp eq i32 %.val.i.i.3.i357.i, %.val1.i.i.3.i358.i, !dbg !10603
  %spec.select.i360.i = select i1 %_0.i.i.not.i.i.3.i359.i, i8 1, i8 2, !dbg !10594
  br label %bb7.i32.i, !dbg !10594

bb7.i32.i:                                        ; preds = %bb1.i.i.3.i354.i, %bb1.i.i.2.i348.i, %bb1.i.i.1.i342.i, %bb1.i.i.preheader.i28.i, %bb27.i
  %_0.sroa.0.0.i822.i.i = phi i8 [ 0, %bb27.i ], [ 2, %bb1.i.i.preheader.i28.i ], [ 2, %bb1.i.i.2.i348.i ], [ %spec.select.i360.i, %bb1.i.i.3.i354.i ], [ 2, %bb1.i.i.1.i342.i ], !dbg !10616
  %578 = getelementptr inbounds nuw i8, ptr %self, i64 896, !dbg !10617
  %_38.i33.i = getelementptr inbounds nuw i8, ptr %self, i64 992, !dbg !10619
  %_64.0.i34.i = load ptr, ptr %_15.i24.i, align 8, !dbg !10620, !alias.scope !10581, !noalias !10582, !nonnull !12, !noundef !12
  %579 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !10620
  %_64.1.i35.i = load i64, ptr %579, align 8, !dbg !10620, !alias.scope !10581, !noalias !10582, !noundef !12
  %_66.0.i36.i = load ptr, ptr %data.i.i817.i.i, align 8, !dbg !10621, !alias.scope !10581, !noalias !10582, !nonnull !12, !noundef !12
  %580 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !10621
  %_66.1.i37.i = load i64, ptr %580, align 8, !dbg !10621, !alias.scope !10581, !noalias !10582, !noundef !12
  %_57.i38.i = getelementptr inbounds nuw i8, ptr %self, i64 1808, !dbg !10622
  %581 = getelementptr inbounds nuw i8, ptr %self, i64 1812, !dbg !10623
  %_58.i39.i = load i32, ptr %581, align 4, !dbg !10623, !alias.scope !10581, !noalias !10582, !noundef !12
  %582 = getelementptr inbounds nuw i8, ptr %self, i64 1816, !dbg !10624
  %_59.i40.i = load i32, ptr %582, align 8, !dbg !10624, !alias.scope !10581, !noalias !10582, !noundef !12
  %base.i.i45.i = load i32, ptr %_57.i38.i, align 4, !dbg !10625, !alias.scope !10581, !noalias !10635, !noundef !12
  %583 = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %584 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %585 = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %586 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %587 = getelementptr inbounds nuw i8, ptr %self, i64 976
  %588 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %589 = getelementptr inbounds nuw i8, ptr %self, i64 1360
  %590 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %591 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %592 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %593 = getelementptr inbounds nuw i8, ptr %self, i64 944
  %594 = getelementptr inbounds nuw i8, ptr %self, i64 1456
  %595 = getelementptr inbounds nuw i8, ptr %self, i64 1520
  %596 = getelementptr inbounds nuw i8, ptr %self, i64 1584
  %597 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %598 = getelementptr inbounds nuw i8, ptr %self, i64 1072
  %599 = getelementptr inbounds nuw i8, ptr %self, i64 1648
  %600 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %601 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %602 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %603 = getelementptr inbounds nuw i8, ptr %self, i64 1008
  %604 = getelementptr inbounds nuw i8, ptr %self, i64 1040
  %605 = lshr i64 %_63.i, 2, !dbg !10638
  %606 = lshr i64 %_55.i, 2, !dbg !10638
  %607 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %608 = getelementptr inbounds nuw i8, ptr %self, i64 868
  %609 = getelementptr inbounds nuw i8, ptr %self, i64 808
  %610 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %611 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %612 = getelementptr inbounds nuw i8, ptr %self, i64 876
  br label %bb30.i.i46.i, !dbg !10638

bb30.i.i46.i:                                     ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i176.i, %bb7.i32.i
  %iter.sroa.0.0.i1630.i.i = phi i64 [ 0, %bb7.i32.i ], [ %613, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i176.i ]
  %613 = add nuw nsw i64 %iter.sroa.0.0.i1630.i.i, 1, !dbg !10647
  %span.i.i47.i = shl i64 %iter.sroa.0.0.i1630.i.i, 2, !dbg !10653
  %_28.i.i48.i = trunc i64 %iter.sroa.0.0.i1630.i.i to i32, !dbg !10655
  %now.i.i49.i = add i32 %base.i.i45.i, %_28.i.i48.i, !dbg !10657
  %_31.i.i50.i = and i32 %now.i.i49.i, %_58.i39.i, !dbg !10660
  %_30.i.i51.i = zext i32 %_31.i.i50.i to i64, !dbg !10662
  %write.i.i52.i = shl nuw nsw i64 %_30.i.i51.i, 2, !dbg !10662
  %exitcond.not.i53.i = icmp eq i64 %iter.sroa.0.0.i1630.i.i, %606, !dbg !10663
  br i1 %exitcond.not.i53.i, label %bb33.i.i340.i, label %bb32.i.i54.i, !dbg !10663, !prof !2561

bb33.i.i340.i:                                    ; preds = %bb30.i.i46.i
  %_35.i.i341.i = add nuw nsw i64 %span.i.i47.i, 4, !dbg !10671
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i47.i, i64 noundef %_35.i.i341.i, i64 noundef range(i64 0, 2305843009213693952) %_55.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d0dbd696a058609bd94bbe1fa75d0723) #24, !dbg !10672, !noalias !10635
  unreachable, !dbg !10672

bb32.i.i54.i:                                     ; preds = %bb30.i.i46.i
  %_97.i.i55.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %span.i.i47.i, !dbg !10673
  %_37.i.i56.i = add nuw nsw i64 %write.i.i52.i, 4, !dbg !10677
  %_98.not.i.i57.i = icmp ugt i64 %_37.i.i56.i, %_64.1.i35.i, !dbg !10678
  br i1 %_98.not.i.i57.i, label %bb36.i.i339.i, label %bb35.i.i58.i, !dbg !10678, !prof !180

bb36.i.i339.i:                                    ; preds = %bb32.i.i54.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i52.i, i64 noundef %_37.i.i56.i, i64 noundef %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1eeef3195352adc58f4bfdb015316fef) #24, !dbg !10683, !noalias !10635
  unreachable, !dbg !10683

bb35.i.i58.i:                                     ; preds = %bb32.i.i54.i
  %_107.i.i59.i = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %write.i.i52.i, !dbg !10684
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %_107.i.i59.i, ptr noundef nonnull align 4 dereferenceable(16) %_97.i.i55.i, i64 16, i1 false), !dbg !10688, !noalias !10693
  %exitcond1765.i.i = icmp eq i64 %iter.sroa.0.0.i1630.i.i, %605, !dbg !10694
  br i1 %exitcond1765.i.i, label %bb39.i.i338.i, label %bb38.i.i60.i, !dbg !10694, !prof !180

bb39.i.i338.i:                                    ; preds = %bb35.i.i58.i
  %614 = and i64 %_63.i, 2305843009213693948, !dbg !10638
  %615 = add nuw nsw i64 %614, 4, !dbg !10638
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i47.i, i64 noundef %615, i64 noundef range(i64 0, 2305843009213693952) %_63.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d9529ff5ddc99dd60299cff5ff3cd676) #24, !dbg !10698, !noalias !10635
  unreachable, !dbg !10698

bb38.i.i60.i:                                     ; preds = %bb35.i.i58.i
  %_115.i.i61.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %span.i.i47.i, !dbg !10699
  %_116.not.i.i62.i = icmp ugt i64 %_37.i.i56.i, %_66.1.i37.i, !dbg !10703
  br i1 %_116.not.i.i62.i, label %bb41.i.i337.i, label %bb40.i.i63.i, !dbg !10703, !prof !180

bb41.i.i337.i:                                    ; preds = %bb38.i.i60.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i52.i, i64 noundef %_37.i.i56.i, i64 noundef %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9d56679ca30f2caa500ddf2e89d00cb) #24, !dbg !10707, !noalias !10635
  unreachable, !dbg !10707

bb40.i.i63.i:                                     ; preds = %bb38.i.i60.i
  %_123.i.i64.i = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %write.i.i52.i, !dbg !10708
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %_123.i.i64.i, ptr noundef nonnull align 4 dereferenceable(16) %_115.i.i61.i, i64 16, i1 false), !dbg !10712, !noalias !10717
  %_53.i.i65.i = sub i32 %now.i.i49.i, %_59.i40.i, !dbg !10718
  %_52.i.i66.i = and i32 %_53.i.i65.i, %_58.i39.i, !dbg !10721
  %_51.i.i67.i = zext i32 %_52.i.i66.i to i64, !dbg !10722
  %read.i.i68.i = shl nuw nsw i64 %_51.i.i67.i, 2, !dbg !10722
  %_56.i.i69.i = add nuw nsw i64 %read.i.i68.i, 4, !dbg !10723
  %_156.not.i.i70.i = icmp ugt i64 %_56.i.i69.i, %_64.1.i35.i, !dbg !10725
  br i1 %_156.not.i.i70.i, label %bb51.i.i336.i, label %bb50.i.i71.i, !dbg !10725, !prof !180

bb51.i.i336.i:                                    ; preds = %bb40.i.i63.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i68.i, i64 noundef %_56.i.i69.i, i64 noundef %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7) #24, !dbg !10729, !noalias !10635
  unreachable, !dbg !10729

bb50.i.i71.i:                                     ; preds = %bb40.i.i63.i
  %_163.i.i72.i = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %read.i.i68.i, !dbg !10730
  %lanes.i447.sroa.0.0.copyload.i.i = load <4 x float>, ptr %_163.i.i72.i, align 4, !dbg !10734, !alias.scope !10739, !noalias !10743
  %_164.not.i.i73.i = icmp ugt i64 %_56.i.i69.i, %_66.1.i37.i, !dbg !10747
  br i1 %_164.not.i.i73.i, label %bb54.i.i335.i, label %bb53.i.i74.i, !dbg !10747, !prof !180

bb54.i.i335.i:                                    ; preds = %bb50.i.i71.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i68.i, i64 noundef %_56.i.i69.i, i64 noundef %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f198228fd40f4c04a48a745576c5c5ee) #24, !dbg !10752, !noalias !10635
  unreachable, !dbg !10752

bb53.i.i74.i:                                     ; preds = %bb50.i.i71.i
  %_169.i.i75.i = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %read.i.i68.i, !dbg !10753
  %lanes.i441.sroa.0.0.copyload.i.i = load <4 x float>, ptr %_169.i.i75.i, align 4, !dbg !10757, !alias.scope !10762, !noalias !10766
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10770), !dbg !10773
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10776), !dbg !10773
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10778), !dbg !10773
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10780), !dbg !10773
  %_18.i71.i76.i = load i32, ptr %_31.i26.i, align 4, !dbg !10782, !alias.scope !10784, !noalias !10785, !noundef !12
  %_17.i.i77.i = sub i32 %now.i.i49.i, %_18.i71.i76.i, !dbg !10787
  %_16.i72.i78.i = and i32 %_17.i.i77.i, %_58.i39.i, !dbg !10782
  %_15.i73.i79.i = zext i32 %_16.i72.i78.i to i64, !dbg !10782
  %_14.i.i80.i = shl nuw nsw i64 %_15.i73.i79.i, 2, !dbg !10782
  %_26.i.i81.i = load i32, ptr %_33.i27.i, align 4, !dbg !10782, !alias.scope !10789, !noalias !10790, !noundef !12
  %_25.i.i82.i = sub i32 %now.i.i49.i, %_26.i.i81.i, !dbg !10787
  %_24.i.i83.i = and i32 %_25.i.i82.i, %_58.i39.i, !dbg !10782
  %_23.i76.i84.i = zext i32 %_24.i.i83.i to i64, !dbg !10782
  %_22.i.i85.i = shl nuw nsw i64 %_23.i76.i84.i, 2, !dbg !10782
  %_31.i77.i86.i = icmp samesign ult i64 %_14.i.i80.i, %_64.1.i35.i, !dbg !10782
  switch i8 %_0.sroa.0.0.i822.i.i, label %default.unreachable [
    i8 0, label %bb7.i75.preheader.i271.i
    i8 1, label %bb16.i.preheader.i208.i
    i8 2, label %bb25.i.preheader.i87.i
  ], !dbg !10791

bb25.i.preheader.i87.i:                           ; preds = %bb53.i.i74.i
  br i1 %_31.i77.i86.i, label %bb27.i58.i90.i, label %panic28.i.i88.i, !dbg !10792

bb16.i.preheader.i208.i:                          ; preds = %bb53.i.i74.i
  br i1 %_31.i77.i86.i, label %bb17.i.i211.i, label %panic15.i.i209.i, !dbg !10793

bb7.i75.preheader.i271.i:                         ; preds = %bb53.i.i74.i
  br i1 %_31.i77.i86.i, label %bb8.i78.i274.i, label %panic4.i.i272.i, !dbg !10794

bb8.i78.i274.i:                                   ; preds = %bb7.i75.preheader.i271.i
  %_34.i.i275.i = icmp samesign ult i64 %_22.i.i85.i, %_66.1.i37.i, !dbg !10795
  br i1 %_34.i.i275.i, label %bb10.i79.i277.i, label %panic5.i.i276.i, !dbg !10795

panic4.i.i272.i:                                  ; preds = %bb10.i79.2.i313.i, %bb10.i79.1.i295.i, %bb10.i79.i277.i, %bb7.i75.preheader.i271.i
  %left.i.lcssa.i273.i = phi i64 [ %_14.i.i80.i, %bb7.i75.preheader.i271.i ], [ %left.i.1.i285.i, %bb10.i79.i277.i ], [ %left.i.2.i303.i, %bb10.i79.1.i295.i ], [ %left.i.3.i321.i, %bb10.i79.2.i313.i ], !dbg !10796
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left.i.lcssa.i273.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cba26bb3a04aaba20f9c249edc5e133d) #24, !dbg !10794, !noalias !10797
  unreachable, !dbg !10794

panic5.i.i276.i:                                  ; preds = %bb8.i78.3.i329.i, %bb8.i78.2.i311.i, %bb8.i78.1.i293.i, %bb8.i78.i274.i
  %right.i.lcssa1645.i.i = phi i64 [ %_22.i.i85.i, %bb8.i78.i274.i ], [ %right.i.1.i291.i, %bb8.i78.1.i293.i ], [ %right.i.2.i309.i, %bb8.i78.2.i311.i ], [ %right.i.3.i327.i, %bb8.i78.3.i329.i ], !dbg !10798
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right.i.lcssa1645.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2ad2d313de59c36d62f4a7d75017a71f) #24, !dbg !10795, !noalias !10797
  unreachable, !dbg !10795

bb10.i79.i277.i:                                  ; preds = %bb8.i78.i274.i
  %616 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %_14.i.i80.i, !dbg !10794
  %left_own.i.i278.i = load float, ptr %616, align 4, !dbg !10794, !alias.scope !10778, !noalias !10799, !noundef !12
  %617 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %_22.i.i85.i, !dbg !10795
  %right_own.i.i279.i = load float, ptr %617, align 4, !dbg !10795, !alias.scope !10780, !noalias !10800, !noundef !12
  %_18.i71.1.i280.i = load i32, ptr %607, align 4, !dbg !10801, !alias.scope !10784, !noalias !10785, !noundef !12
  %_17.i.1.i281.i = sub i32 %now.i.i49.i, %_18.i71.1.i280.i, !dbg !10802
  %_16.i72.1.i282.i = and i32 %_17.i.1.i281.i, %_58.i39.i, !dbg !10804
  %_15.i73.1.i283.i = zext i32 %_16.i72.1.i282.i to i64, !dbg !10796
  %_14.i.1.i284.i = shl nuw nsw i64 %_15.i73.1.i283.i, 2, !dbg !10796
  %left.i.1.i285.i = or disjoint i64 %_14.i.1.i284.i, 1, !dbg !10796
  %_26.i.1.i286.i = load i32, ptr %608, align 4, !dbg !10805, !alias.scope !10789, !noalias !10790, !noundef !12
  %_25.i.1.i287.i = sub i32 %now.i.i49.i, %_26.i.1.i286.i, !dbg !10806
  %_24.i.1.i288.i = and i32 %_25.i.1.i287.i, %_58.i39.i, !dbg !10808
  %_23.i76.1.i289.i = zext i32 %_24.i.1.i288.i to i64, !dbg !10798
  %_22.i.1.i290.i = shl nuw nsw i64 %_23.i76.1.i289.i, 2, !dbg !10798
  %right.i.1.i291.i = or disjoint i64 %_22.i.1.i290.i, 1, !dbg !10798
  %_31.i77.1.i292.i = icmp samesign ult i64 %left.i.1.i285.i, %_64.1.i35.i, !dbg !10794
  br i1 %_31.i77.1.i292.i, label %bb8.i78.1.i293.i, label %panic4.i.i272.i, !dbg !10794

bb8.i78.1.i293.i:                                 ; preds = %bb10.i79.i277.i
  %_34.i.1.i294.i = icmp samesign ult i64 %right.i.1.i291.i, %_66.1.i37.i, !dbg !10795
  br i1 %_34.i.1.i294.i, label %bb10.i79.1.i295.i, label %panic5.i.i276.i, !dbg !10795

bb10.i79.1.i295.i:                                ; preds = %bb8.i78.1.i293.i
  %618 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.1.i285.i, !dbg !10794
  %left_own.i.1.i296.i = load float, ptr %618, align 4, !dbg !10794, !alias.scope !10778, !noalias !10799, !noundef !12
  %619 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.1.i291.i, !dbg !10795
  %right_own.i.1.i297.i = load float, ptr %619, align 4, !dbg !10795, !alias.scope !10780, !noalias !10800, !noundef !12
  %_18.i71.2.i298.i = load i32, ptr %609, align 4, !dbg !10801, !alias.scope !10784, !noalias !10785, !noundef !12
  %_17.i.2.i299.i = sub i32 %now.i.i49.i, %_18.i71.2.i298.i, !dbg !10802
  %_16.i72.2.i300.i = and i32 %_17.i.2.i299.i, %_58.i39.i, !dbg !10804
  %_15.i73.2.i301.i = zext i32 %_16.i72.2.i300.i to i64, !dbg !10796
  %_14.i.2.i302.i = shl nuw nsw i64 %_15.i73.2.i301.i, 2, !dbg !10796
  %left.i.2.i303.i = or disjoint i64 %_14.i.2.i302.i, 2, !dbg !10796
  %_26.i.2.i304.i = load i32, ptr %610, align 4, !dbg !10805, !alias.scope !10789, !noalias !10790, !noundef !12
  %_25.i.2.i305.i = sub i32 %now.i.i49.i, %_26.i.2.i304.i, !dbg !10806
  %_24.i.2.i306.i = and i32 %_25.i.2.i305.i, %_58.i39.i, !dbg !10808
  %_23.i76.2.i307.i = zext i32 %_24.i.2.i306.i to i64, !dbg !10798
  %_22.i.2.i308.i = shl nuw nsw i64 %_23.i76.2.i307.i, 2, !dbg !10798
  %right.i.2.i309.i = or disjoint i64 %_22.i.2.i308.i, 2, !dbg !10798
  %_31.i77.2.i310.i = icmp samesign ult i64 %left.i.2.i303.i, %_64.1.i35.i, !dbg !10794
  br i1 %_31.i77.2.i310.i, label %bb8.i78.2.i311.i, label %panic4.i.i272.i, !dbg !10794

bb8.i78.2.i311.i:                                 ; preds = %bb10.i79.1.i295.i
  %_34.i.2.i312.i = icmp samesign ult i64 %right.i.2.i309.i, %_66.1.i37.i, !dbg !10795
  br i1 %_34.i.2.i312.i, label %bb10.i79.2.i313.i, label %panic5.i.i276.i, !dbg !10795

bb10.i79.2.i313.i:                                ; preds = %bb8.i78.2.i311.i
  %620 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.2.i303.i, !dbg !10794
  %left_own.i.2.i314.i = load float, ptr %620, align 4, !dbg !10794, !alias.scope !10778, !noalias !10799, !noundef !12
  %621 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.2.i309.i, !dbg !10795
  %right_own.i.2.i315.i = load float, ptr %621, align 4, !dbg !10795, !alias.scope !10780, !noalias !10800, !noundef !12
  %_18.i71.3.i316.i = load i32, ptr %611, align 4, !dbg !10801, !alias.scope !10784, !noalias !10785, !noundef !12
  %_17.i.3.i317.i = sub i32 %now.i.i49.i, %_18.i71.3.i316.i, !dbg !10802
  %_16.i72.3.i318.i = and i32 %_17.i.3.i317.i, %_58.i39.i, !dbg !10804
  %_15.i73.3.i319.i = zext i32 %_16.i72.3.i318.i to i64, !dbg !10796
  %_14.i.3.i320.i = shl nuw nsw i64 %_15.i73.3.i319.i, 2, !dbg !10796
  %left.i.3.i321.i = or disjoint i64 %_14.i.3.i320.i, 3, !dbg !10796
  %_26.i.3.i322.i = load i32, ptr %612, align 4, !dbg !10805, !alias.scope !10789, !noalias !10790, !noundef !12
  %_25.i.3.i323.i = sub i32 %now.i.i49.i, %_26.i.3.i322.i, !dbg !10806
  %_24.i.3.i324.i = and i32 %_25.i.3.i323.i, %_58.i39.i, !dbg !10808
  %_23.i76.3.i325.i = zext i32 %_24.i.3.i324.i to i64, !dbg !10798
  %_22.i.3.i326.i = shl nuw nsw i64 %_23.i76.3.i325.i, 2, !dbg !10798
  %right.i.3.i327.i = or disjoint i64 %_22.i.3.i326.i, 3, !dbg !10798
  %_31.i77.3.i328.i = icmp samesign ult i64 %left.i.3.i321.i, %_64.1.i35.i, !dbg !10794
  br i1 %_31.i77.3.i328.i, label %bb8.i78.3.i329.i, label %panic4.i.i272.i, !dbg !10794

bb8.i78.3.i329.i:                                 ; preds = %bb10.i79.2.i313.i
  %_34.i.3.i330.i = icmp samesign ult i64 %right.i.3.i327.i, %_66.1.i37.i, !dbg !10795
  br i1 %_34.i.3.i330.i, label %bb10.i79.3.i331.i, label %panic5.i.i276.i, !dbg !10795

bb10.i79.3.i331.i:                                ; preds = %bb8.i78.3.i329.i
  %622 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left.i.3.i321.i, !dbg !10794
  %left_own.i.3.i332.i = load float, ptr %622, align 4, !dbg !10794, !alias.scope !10778, !noalias !10799, !noundef !12
  %623 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right.i.3.i327.i, !dbg !10795
  %right_own.i.3.i333.i = load float, ptr %623, align 4, !dbg !10795, !alias.scope !10780, !noalias !10800, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i176.i, !dbg !10809

bb17.i.i211.i:                                    ; preds = %bb16.i.preheader.i208.i
  %_59.i.i212.i = icmp samesign ult i64 %_22.i.i85.i, %_66.1.i37.i, !dbg !10812
  br i1 %_59.i.i212.i, label %bb19.i.i214.i, label %panic17.i.i213.i, !dbg !10812

panic15.i.i209.i:                                 ; preds = %bb19.i.2.i250.i, %bb19.i.1.i232.i, %bb19.i.i214.i, %bb16.i.preheader.i208.i
  %left12.i.lcssa.i210.i = phi i64 [ %_14.i.i80.i, %bb16.i.preheader.i208.i ], [ %left12.i.1.i222.i, %bb19.i.i214.i ], [ %left12.i.2.i240.i, %bb19.i.1.i232.i ], [ %left12.i.3.i258.i, %bb19.i.2.i250.i ], !dbg !10813
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left12.i.lcssa.i210.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2e01e5f964095ee8e5aa091c7d92b88) #24, !dbg !10793, !noalias !10797
  unreachable, !dbg !10793

panic17.i.i213.i:                                 ; preds = %bb17.i.3.i266.i, %bb17.i.2.i248.i, %bb17.i.1.i230.i, %bb17.i.i211.i
  %right14.i.lcssa1641.i.i = phi i64 [ %_22.i.i85.i, %bb17.i.i211.i ], [ %right14.i.1.i228.i, %bb17.i.1.i230.i ], [ %right14.i.2.i246.i, %bb17.i.2.i248.i ], [ %right14.i.3.i264.i, %bb17.i.3.i266.i ], !dbg !10814
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right14.i.lcssa1641.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_48dca199019b49040caac979cf1ddc5a) #24, !dbg !10812, !noalias !10797
  unreachable, !dbg !10812

bb19.i.i214.i:                                    ; preds = %bb17.i.i211.i
  %624 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %_14.i.i80.i, !dbg !10793
  %left_own16.i.i215.i = load float, ptr %624, align 4, !dbg !10793, !alias.scope !10778, !noalias !10799, !noundef !12
  %625 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %_22.i.i85.i, !dbg !10812
  %right_own18.i.i216.i = load float, ptr %625, align 4, !dbg !10812, !alias.scope !10780, !noalias !10800, !noundef !12
  %_43.i.1.i217.i = load i32, ptr %607, align 4, !dbg !10815, !alias.scope !10784, !noalias !10785, !noundef !12
  %_42.i.1.i218.i = sub i32 %now.i.i49.i, %_43.i.1.i217.i, !dbg !10816
  %_41.i.1.i219.i = and i32 %_42.i.1.i218.i, %_58.i39.i, !dbg !10818
  %_40.i.1.i220.i = zext i32 %_41.i.1.i219.i to i64, !dbg !10813
  %_39.i63.1.i221.i = shl nuw nsw i64 %_40.i.1.i220.i, 2, !dbg !10813
  %left12.i.1.i222.i = or disjoint i64 %_39.i63.1.i221.i, 1, !dbg !10813
  %_51.i65.1.i223.i = load i32, ptr %608, align 4, !dbg !10819, !alias.scope !10789, !noalias !10790, !noundef !12
  %_50.i.1.i224.i = sub i32 %now.i.i49.i, %_51.i65.1.i223.i, !dbg !10820
  %_49.i.1.i225.i = and i32 %_50.i.1.i224.i, %_58.i39.i, !dbg !10822
  %_48.i.1.i226.i = zext i32 %_49.i.1.i225.i to i64, !dbg !10814
  %_47.i.1.i227.i = shl nuw nsw i64 %_48.i.1.i226.i, 2, !dbg !10814
  %right14.i.1.i228.i = or disjoint i64 %_47.i.1.i227.i, 1, !dbg !10814
  %_56.i66.1.i229.i = icmp samesign ult i64 %left12.i.1.i222.i, %_64.1.i35.i, !dbg !10793
  br i1 %_56.i66.1.i229.i, label %bb17.i.1.i230.i, label %panic15.i.i209.i, !dbg !10793

bb17.i.1.i230.i:                                  ; preds = %bb19.i.i214.i
  %_59.i.1.i231.i = icmp samesign ult i64 %right14.i.1.i228.i, %_66.1.i37.i, !dbg !10812
  br i1 %_59.i.1.i231.i, label %bb19.i.1.i232.i, label %panic17.i.i213.i, !dbg !10812

bb19.i.1.i232.i:                                  ; preds = %bb17.i.1.i230.i
  %626 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.1.i222.i, !dbg !10793
  %left_own16.i.1.i233.i = load float, ptr %626, align 4, !dbg !10793, !alias.scope !10778, !noalias !10799, !noundef !12
  %627 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.1.i228.i, !dbg !10812
  %right_own18.i.1.i234.i = load float, ptr %627, align 4, !dbg !10812, !alias.scope !10780, !noalias !10800, !noundef !12
  %_43.i.2.i235.i = load i32, ptr %609, align 4, !dbg !10815, !alias.scope !10784, !noalias !10785, !noundef !12
  %_42.i.2.i236.i = sub i32 %now.i.i49.i, %_43.i.2.i235.i, !dbg !10816
  %_41.i.2.i237.i = and i32 %_42.i.2.i236.i, %_58.i39.i, !dbg !10818
  %_40.i.2.i238.i = zext i32 %_41.i.2.i237.i to i64, !dbg !10813
  %_39.i63.2.i239.i = shl nuw nsw i64 %_40.i.2.i238.i, 2, !dbg !10813
  %left12.i.2.i240.i = or disjoint i64 %_39.i63.2.i239.i, 2, !dbg !10813
  %_51.i65.2.i241.i = load i32, ptr %610, align 4, !dbg !10819, !alias.scope !10789, !noalias !10790, !noundef !12
  %_50.i.2.i242.i = sub i32 %now.i.i49.i, %_51.i65.2.i241.i, !dbg !10820
  %_49.i.2.i243.i = and i32 %_50.i.2.i242.i, %_58.i39.i, !dbg !10822
  %_48.i.2.i244.i = zext i32 %_49.i.2.i243.i to i64, !dbg !10814
  %_47.i.2.i245.i = shl nuw nsw i64 %_48.i.2.i244.i, 2, !dbg !10814
  %right14.i.2.i246.i = or disjoint i64 %_47.i.2.i245.i, 2, !dbg !10814
  %_56.i66.2.i247.i = icmp samesign ult i64 %left12.i.2.i240.i, %_64.1.i35.i, !dbg !10793
  br i1 %_56.i66.2.i247.i, label %bb17.i.2.i248.i, label %panic15.i.i209.i, !dbg !10793

bb17.i.2.i248.i:                                  ; preds = %bb19.i.1.i232.i
  %_59.i.2.i249.i = icmp samesign ult i64 %right14.i.2.i246.i, %_66.1.i37.i, !dbg !10812
  br i1 %_59.i.2.i249.i, label %bb19.i.2.i250.i, label %panic17.i.i213.i, !dbg !10812

bb19.i.2.i250.i:                                  ; preds = %bb17.i.2.i248.i
  %628 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.2.i240.i, !dbg !10793
  %left_own16.i.2.i251.i = load float, ptr %628, align 4, !dbg !10793, !alias.scope !10778, !noalias !10799, !noundef !12
  %629 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.2.i246.i, !dbg !10812
  %right_own18.i.2.i252.i = load float, ptr %629, align 4, !dbg !10812, !alias.scope !10780, !noalias !10800, !noundef !12
  %_43.i.3.i253.i = load i32, ptr %611, align 4, !dbg !10815, !alias.scope !10784, !noalias !10785, !noundef !12
  %_42.i.3.i254.i = sub i32 %now.i.i49.i, %_43.i.3.i253.i, !dbg !10816
  %_41.i.3.i255.i = and i32 %_42.i.3.i254.i, %_58.i39.i, !dbg !10818
  %_40.i.3.i256.i = zext i32 %_41.i.3.i255.i to i64, !dbg !10813
  %_39.i63.3.i257.i = shl nuw nsw i64 %_40.i.3.i256.i, 2, !dbg !10813
  %left12.i.3.i258.i = or disjoint i64 %_39.i63.3.i257.i, 3, !dbg !10813
  %_51.i65.3.i259.i = load i32, ptr %612, align 4, !dbg !10819, !alias.scope !10789, !noalias !10790, !noundef !12
  %_50.i.3.i260.i = sub i32 %now.i.i49.i, %_51.i65.3.i259.i, !dbg !10820
  %_49.i.3.i261.i = and i32 %_50.i.3.i260.i, %_58.i39.i, !dbg !10822
  %_48.i.3.i262.i = zext i32 %_49.i.3.i261.i to i64, !dbg !10814
  %_47.i.3.i263.i = shl nuw nsw i64 %_48.i.3.i262.i, 2, !dbg !10814
  %right14.i.3.i264.i = or disjoint i64 %_47.i.3.i263.i, 3, !dbg !10814
  %_56.i66.3.i265.i = icmp samesign ult i64 %left12.i.3.i258.i, %_64.1.i35.i, !dbg !10793
  br i1 %_56.i66.3.i265.i, label %bb17.i.3.i266.i, label %panic15.i.i209.i, !dbg !10793

bb17.i.3.i266.i:                                  ; preds = %bb19.i.2.i250.i
  %_59.i.3.i267.i = icmp samesign ult i64 %right14.i.3.i264.i, %_66.1.i37.i, !dbg !10812
  br i1 %_59.i.3.i267.i, label %bb19.i.3.i268.i, label %panic17.i.i213.i, !dbg !10812

bb19.i.3.i268.i:                                  ; preds = %bb17.i.3.i266.i
  %630 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left12.i.3.i258.i, !dbg !10793
  %left_own16.i.3.i269.i = load float, ptr %630, align 4, !dbg !10793, !alias.scope !10778, !noalias !10799, !noundef !12
  %631 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right14.i.3.i264.i, !dbg !10812
  %right_own18.i.3.i270.i = load float, ptr %631, align 4, !dbg !10812, !alias.scope !10780, !noalias !10800, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i176.i, !dbg !10809

panic28.i.i88.i:                                  ; preds = %bb33.i60.2.i150.i, %bb33.i60.1.i126.i, %bb33.i60.i102.i, %bb25.i.preheader.i87.i
  %left25.i.lcssa.i89.i = phi i64 [ %_14.i.i80.i, %bb25.i.preheader.i87.i ], [ %left25.i.1.i109.i, %bb33.i60.i102.i ], [ %left25.i.2.i133.i, %bb33.i60.1.i126.i ], [ %left25.i.3.i157.i, %bb33.i60.2.i150.i ], !dbg !10823
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left25.i.lcssa.i89.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4b7b2eb7fa1b19ad0451b09b71313122) #24, !dbg !10792, !noalias !10797
  unreachable, !dbg !10792

bb27.i58.i90.i:                                   ; preds = %bb25.i.preheader.i87.i
  %632 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %_14.i.i80.i, !dbg !10792
  %_79.i.i91.i = load float, ptr %632, align 4, !dbg !10792, !alias.scope !10778, !noalias !10799, !noundef !12
  %_85.i.i92.i = icmp samesign ult i64 %_14.i.i80.i, %_66.1.i37.i, !dbg !10824
  br i1 %_85.i.i92.i, label %bb29.i59.i94.i, label %panic30.i.i93.i, !dbg !10824

panic30.i.i93.i:                                  ; preds = %bb27.i58.3.i165.i, %bb27.i58.2.i141.i, %bb27.i58.1.i117.i, %bb27.i58.i90.i
  %left25.i.lcssa1637.i.i = phi i64 [ %_14.i.i80.i, %bb27.i58.i90.i ], [ %left25.i.1.i109.i, %bb27.i58.1.i117.i ], [ %left25.i.2.i133.i, %bb27.i58.2.i141.i ], [ %left25.i.3.i157.i, %bb27.i58.3.i165.i ], !dbg !10823
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %left25.i.lcssa1637.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60256fc2b51ee5bb69caa4304418caff) #24, !dbg !10824, !noalias !10797
  unreachable, !dbg !10824

bb29.i59.i94.i:                                   ; preds = %bb27.i58.i90.i
  %633 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %_14.i.i80.i, !dbg !10824
  %_83.i.i95.i = load float, ptr %633, align 4, !dbg !10824, !alias.scope !10780, !noalias !10800, !noundef !12
  %_87.i.i96.i = icmp samesign ult i64 %_22.i.i85.i, %_66.1.i37.i, !dbg !10825
  br i1 %_87.i.i96.i, label %bb31.i.i98.i, label %panic32.i.i97.i, !dbg !10825

panic32.i.i97.i:                                  ; preds = %bb29.i59.3.i168.i, %bb29.i59.2.i144.i, %bb29.i59.1.i120.i, %bb29.i59.i94.i
  %right27.i.lcssa1634.i.i = phi i64 [ %_22.i.i85.i, %bb29.i59.i94.i ], [ %right27.i.1.i115.i, %bb29.i59.1.i120.i ], [ %right27.i.2.i139.i, %bb29.i59.2.i144.i ], [ %right27.i.3.i163.i, %bb29.i59.3.i168.i ], !dbg !10826
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right27.i.lcssa1634.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i37.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_03bcac171bcf0d517141d8cd3ac9ded0) #24, !dbg !10825, !noalias !10797
  unreachable, !dbg !10825

bb31.i.i98.i:                                     ; preds = %bb29.i59.i94.i
  %634 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %_22.i.i85.i, !dbg !10825
  %_86.i.i99.i = load float, ptr %634, align 4, !dbg !10825, !alias.scope !10780, !noalias !10800, !noundef !12
  %_89.i.i100.i = icmp samesign ult i64 %_22.i.i85.i, %_64.1.i35.i, !dbg !10827
  br i1 %_89.i.i100.i, label %bb33.i60.i102.i, label %panic34.i.i101.i, !dbg !10827

panic34.i.i101.i:                                 ; preds = %bb31.i.3.i171.i, %bb31.i.2.i147.i, %bb31.i.1.i123.i, %bb31.i.i98.i
  %right27.i.lcssa1635.i.i = phi i64 [ %_22.i.i85.i, %bb31.i.i98.i ], [ %right27.i.1.i115.i, %bb31.i.1.i123.i ], [ %right27.i.2.i139.i, %bb31.i.2.i147.i ], [ %right27.i.3.i163.i, %bb31.i.3.i171.i ], !dbg !10826
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %right27.i.lcssa1635.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i35.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ed84bd2438b7b7e3dfb2147bac09e923) #24, !dbg !10827, !noalias !10797
  unreachable, !dbg !10827

bb33.i60.i102.i:                                  ; preds = %bb31.i.i98.i
  %635 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %_22.i.i85.i, !dbg !10827
  %_88.i.i103.i = load float, ptr %635, align 4, !dbg !10827, !alias.scope !10778, !noalias !10799, !noundef !12
  %_68.i.1.i104.i = load i32, ptr %607, align 4, !dbg !10828, !alias.scope !10784, !noalias !10785, !noundef !12
  %_67.i52.1.i105.i = sub i32 %now.i.i49.i, %_68.i.1.i104.i, !dbg !10829
  %_66.i.1.i106.i = and i32 %_67.i52.1.i105.i, %_58.i39.i, !dbg !10831
  %_65.i.1.i107.i = zext i32 %_66.i.1.i106.i to i64, !dbg !10823
  %_64.i53.1.i108.i = shl nuw nsw i64 %_65.i.1.i107.i, 2, !dbg !10823
  %left25.i.1.i109.i = or disjoint i64 %_64.i53.1.i108.i, 1, !dbg !10823
  %_76.i54.1.i110.i = load i32, ptr %608, align 4, !dbg !10832, !alias.scope !10789, !noalias !10790, !noundef !12
  %_75.i.1.i111.i = sub i32 %now.i.i49.i, %_76.i54.1.i110.i, !dbg !10833
  %_74.i55.1.i112.i = and i32 %_75.i.1.i111.i, %_58.i39.i, !dbg !10835
  %_73.i56.1.i113.i = zext i32 %_74.i55.1.i112.i to i64, !dbg !10826
  %_72.i.1.i114.i = shl nuw nsw i64 %_73.i56.1.i113.i, 2, !dbg !10826
  %right27.i.1.i115.i = or disjoint i64 %_72.i.1.i114.i, 1, !dbg !10826
  %_81.i57.1.i116.i = icmp samesign ult i64 %left25.i.1.i109.i, %_64.1.i35.i, !dbg !10792
  br i1 %_81.i57.1.i116.i, label %bb27.i58.1.i117.i, label %panic28.i.i88.i, !dbg !10792

bb27.i58.1.i117.i:                                ; preds = %bb33.i60.i102.i
  %636 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.1.i109.i, !dbg !10792
  %_79.i.1.i118.i = load float, ptr %636, align 4, !dbg !10792, !alias.scope !10778, !noalias !10799, !noundef !12
  %_85.i.1.i119.i = icmp samesign ult i64 %left25.i.1.i109.i, %_66.1.i37.i, !dbg !10824
  br i1 %_85.i.1.i119.i, label %bb29.i59.1.i120.i, label %panic30.i.i93.i, !dbg !10824

bb29.i59.1.i120.i:                                ; preds = %bb27.i58.1.i117.i
  %637 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.1.i109.i, !dbg !10824
  %_83.i.1.i121.i = load float, ptr %637, align 4, !dbg !10824, !alias.scope !10780, !noalias !10800, !noundef !12
  %_87.i.1.i122.i = icmp samesign ult i64 %right27.i.1.i115.i, %_66.1.i37.i, !dbg !10825
  br i1 %_87.i.1.i122.i, label %bb31.i.1.i123.i, label %panic32.i.i97.i, !dbg !10825

bb31.i.1.i123.i:                                  ; preds = %bb29.i59.1.i120.i
  %638 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.1.i115.i, !dbg !10825
  %_86.i.1.i124.i = load float, ptr %638, align 4, !dbg !10825, !alias.scope !10780, !noalias !10800, !noundef !12
  %_89.i.1.i125.i = icmp samesign ult i64 %right27.i.1.i115.i, %_64.1.i35.i, !dbg !10827
  br i1 %_89.i.1.i125.i, label %bb33.i60.1.i126.i, label %panic34.i.i101.i, !dbg !10827

bb33.i60.1.i126.i:                                ; preds = %bb31.i.1.i123.i
  %639 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.1.i115.i, !dbg !10827
  %_88.i.1.i127.i = load float, ptr %639, align 4, !dbg !10827, !alias.scope !10778, !noalias !10799, !noundef !12
  %_68.i.2.i128.i = load i32, ptr %609, align 4, !dbg !10828, !alias.scope !10784, !noalias !10785, !noundef !12
  %_67.i52.2.i129.i = sub i32 %now.i.i49.i, %_68.i.2.i128.i, !dbg !10829
  %_66.i.2.i130.i = and i32 %_67.i52.2.i129.i, %_58.i39.i, !dbg !10831
  %_65.i.2.i131.i = zext i32 %_66.i.2.i130.i to i64, !dbg !10823
  %_64.i53.2.i132.i = shl nuw nsw i64 %_65.i.2.i131.i, 2, !dbg !10823
  %left25.i.2.i133.i = or disjoint i64 %_64.i53.2.i132.i, 2, !dbg !10823
  %_76.i54.2.i134.i = load i32, ptr %610, align 4, !dbg !10832, !alias.scope !10789, !noalias !10790, !noundef !12
  %_75.i.2.i135.i = sub i32 %now.i.i49.i, %_76.i54.2.i134.i, !dbg !10833
  %_74.i55.2.i136.i = and i32 %_75.i.2.i135.i, %_58.i39.i, !dbg !10835
  %_73.i56.2.i137.i = zext i32 %_74.i55.2.i136.i to i64, !dbg !10826
  %_72.i.2.i138.i = shl nuw nsw i64 %_73.i56.2.i137.i, 2, !dbg !10826
  %right27.i.2.i139.i = or disjoint i64 %_72.i.2.i138.i, 2, !dbg !10826
  %_81.i57.2.i140.i = icmp samesign ult i64 %left25.i.2.i133.i, %_64.1.i35.i, !dbg !10792
  br i1 %_81.i57.2.i140.i, label %bb27.i58.2.i141.i, label %panic28.i.i88.i, !dbg !10792

bb27.i58.2.i141.i:                                ; preds = %bb33.i60.1.i126.i
  %640 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.2.i133.i, !dbg !10792
  %_79.i.2.i142.i = load float, ptr %640, align 4, !dbg !10792, !alias.scope !10778, !noalias !10799, !noundef !12
  %_85.i.2.i143.i = icmp samesign ult i64 %left25.i.2.i133.i, %_66.1.i37.i, !dbg !10824
  br i1 %_85.i.2.i143.i, label %bb29.i59.2.i144.i, label %panic30.i.i93.i, !dbg !10824

bb29.i59.2.i144.i:                                ; preds = %bb27.i58.2.i141.i
  %641 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.2.i133.i, !dbg !10824
  %_83.i.2.i145.i = load float, ptr %641, align 4, !dbg !10824, !alias.scope !10780, !noalias !10800, !noundef !12
  %_87.i.2.i146.i = icmp samesign ult i64 %right27.i.2.i139.i, %_66.1.i37.i, !dbg !10825
  br i1 %_87.i.2.i146.i, label %bb31.i.2.i147.i, label %panic32.i.i97.i, !dbg !10825

bb31.i.2.i147.i:                                  ; preds = %bb29.i59.2.i144.i
  %642 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.2.i139.i, !dbg !10825
  %_86.i.2.i148.i = load float, ptr %642, align 4, !dbg !10825, !alias.scope !10780, !noalias !10800, !noundef !12
  %_89.i.2.i149.i = icmp samesign ult i64 %right27.i.2.i139.i, %_64.1.i35.i, !dbg !10827
  br i1 %_89.i.2.i149.i, label %bb33.i60.2.i150.i, label %panic34.i.i101.i, !dbg !10827

bb33.i60.2.i150.i:                                ; preds = %bb31.i.2.i147.i
  %643 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.2.i139.i, !dbg !10827
  %_88.i.2.i151.i = load float, ptr %643, align 4, !dbg !10827, !alias.scope !10778, !noalias !10799, !noundef !12
  %_68.i.3.i152.i = load i32, ptr %611, align 4, !dbg !10828, !alias.scope !10784, !noalias !10785, !noundef !12
  %_67.i52.3.i153.i = sub i32 %now.i.i49.i, %_68.i.3.i152.i, !dbg !10829
  %_66.i.3.i154.i = and i32 %_67.i52.3.i153.i, %_58.i39.i, !dbg !10831
  %_65.i.3.i155.i = zext i32 %_66.i.3.i154.i to i64, !dbg !10823
  %_64.i53.3.i156.i = shl nuw nsw i64 %_65.i.3.i155.i, 2, !dbg !10823
  %left25.i.3.i157.i = or disjoint i64 %_64.i53.3.i156.i, 3, !dbg !10823
  %_76.i54.3.i158.i = load i32, ptr %612, align 4, !dbg !10832, !alias.scope !10789, !noalias !10790, !noundef !12
  %_75.i.3.i159.i = sub i32 %now.i.i49.i, %_76.i54.3.i158.i, !dbg !10833
  %_74.i55.3.i160.i = and i32 %_75.i.3.i159.i, %_58.i39.i, !dbg !10835
  %_73.i56.3.i161.i = zext i32 %_74.i55.3.i160.i to i64, !dbg !10826
  %_72.i.3.i162.i = shl nuw nsw i64 %_73.i56.3.i161.i, 2, !dbg !10826
  %right27.i.3.i163.i = or disjoint i64 %_72.i.3.i162.i, 3, !dbg !10826
  %_81.i57.3.i164.i = icmp samesign ult i64 %left25.i.3.i157.i, %_64.1.i35.i, !dbg !10792
  br i1 %_81.i57.3.i164.i, label %bb27.i58.3.i165.i, label %panic28.i.i88.i, !dbg !10792

bb27.i58.3.i165.i:                                ; preds = %bb33.i60.2.i150.i
  %644 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %left25.i.3.i157.i, !dbg !10792
  %_79.i.3.i166.i = load float, ptr %644, align 4, !dbg !10792, !alias.scope !10778, !noalias !10799, !noundef !12
  %_85.i.3.i167.i = icmp samesign ult i64 %left25.i.3.i157.i, %_66.1.i37.i, !dbg !10824
  br i1 %_85.i.3.i167.i, label %bb29.i59.3.i168.i, label %panic30.i.i93.i, !dbg !10824

bb29.i59.3.i168.i:                                ; preds = %bb27.i58.3.i165.i
  %645 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %left25.i.3.i157.i, !dbg !10824
  %_83.i.3.i169.i = load float, ptr %645, align 4, !dbg !10824, !alias.scope !10780, !noalias !10800, !noundef !12
  %_87.i.3.i170.i = icmp samesign ult i64 %right27.i.3.i163.i, %_66.1.i37.i, !dbg !10825
  br i1 %_87.i.3.i170.i, label %bb31.i.3.i171.i, label %panic32.i.i97.i, !dbg !10825

bb31.i.3.i171.i:                                  ; preds = %bb29.i59.3.i168.i
  %_89.i.3.i172.i = icmp samesign ult i64 %right27.i.3.i163.i, %_64.1.i35.i, !dbg !10827
  br i1 %_89.i.3.i172.i, label %bb33.i60.3.i173.i, label %panic34.i.i101.i, !dbg !10827

bb33.i60.3.i173.i:                                ; preds = %bb31.i.3.i171.i
  %646 = getelementptr inbounds nuw float, ptr %_66.0.i36.i, i64 %right27.i.3.i163.i, !dbg !10825
  %_86.i.3.i174.i = load float, ptr %646, align 4, !dbg !10825, !alias.scope !10780, !noalias !10800, !noundef !12
  %647 = getelementptr inbounds nuw float, ptr %_64.0.i34.i, i64 %right27.i.3.i163.i, !dbg !10827
  %_88.i.3.i175.i = load float, ptr %647, align 4, !dbg !10827, !alias.scope !10778, !noalias !10799, !noundef !12
  br label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i176.i, !dbg !10809

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i176.i: ; preds = %bb33.i60.3.i173.i, %bb19.i.3.i268.i, %bb10.i79.3.i331.i
  %taps.i.sroa.55.0.i177.i = phi float [ %right_own.i.3.i333.i, %bb10.i79.3.i331.i ], [ %left_own16.i.3.i269.i, %bb19.i.3.i268.i ], [ %_88.i.3.i175.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.52.0.i178.i = phi float [ %right_own.i.2.i315.i, %bb10.i79.3.i331.i ], [ %left_own16.i.2.i251.i, %bb19.i.3.i268.i ], [ %_88.i.2.i151.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.49.0.i179.i = phi float [ %right_own.i.1.i297.i, %bb10.i79.3.i331.i ], [ %left_own16.i.1.i233.i, %bb19.i.3.i268.i ], [ %_88.i.1.i127.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.441781.0.i.i = phi float [ %right_own.i.i279.i, %bb10.i79.3.i331.i ], [ %left_own16.i.i215.i, %bb19.i.3.i268.i ], [ %_88.i.i103.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.41.0.i180.i = phi float [ %right_own.i.3.i333.i, %bb10.i79.3.i331.i ], [ %right_own18.i.3.i270.i, %bb19.i.3.i268.i ], [ %_86.i.3.i174.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.38.0.i181.i = phi float [ %right_own.i.2.i315.i, %bb10.i79.3.i331.i ], [ %right_own18.i.2.i252.i, %bb19.i.3.i268.i ], [ %_86.i.2.i148.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.35.0.i182.i = phi float [ %right_own.i.1.i297.i, %bb10.i79.3.i331.i ], [ %right_own18.i.1.i234.i, %bb19.i.3.i268.i ], [ %_86.i.1.i124.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.301776.0.i.i = phi float [ %right_own.i.i279.i, %bb10.i79.3.i331.i ], [ %right_own18.i.i216.i, %bb19.i.3.i268.i ], [ %_86.i.i99.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.27.0.i183.i = phi float [ %left_own.i.3.i332.i, %bb10.i79.3.i331.i ], [ %right_own18.i.3.i270.i, %bb19.i.3.i268.i ], [ %_83.i.3.i169.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.24.0.i184.i = phi float [ %left_own.i.2.i314.i, %bb10.i79.3.i331.i ], [ %right_own18.i.2.i252.i, %bb19.i.3.i268.i ], [ %_83.i.2.i145.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.21.0.i185.i = phi float [ %left_own.i.1.i296.i, %bb10.i79.3.i331.i ], [ %right_own18.i.1.i234.i, %bb19.i.3.i268.i ], [ %_83.i.1.i121.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.161771.0.i.i = phi float [ %left_own.i.i278.i, %bb10.i79.3.i331.i ], [ %right_own18.i.i216.i, %bb19.i.3.i268.i ], [ %_83.i.i95.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.13.0.i186.i = phi float [ %left_own.i.3.i332.i, %bb10.i79.3.i331.i ], [ %left_own16.i.3.i269.i, %bb19.i.3.i268.i ], [ %_79.i.3.i166.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.10.0.i187.i = phi float [ %left_own.i.2.i314.i, %bb10.i79.3.i331.i ], [ %left_own16.i.2.i251.i, %bb19.i.3.i268.i ], [ %_79.i.2.i142.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.7.0.i188.i = phi float [ %left_own.i.1.i296.i, %bb10.i79.3.i331.i ], [ %left_own16.i.1.i233.i, %bb19.i.3.i268.i ], [ %_79.i.1.i118.i, %bb33.i60.3.i173.i ], !dbg !10782
  %taps.i.sroa.0.0.i189.i = phi float [ %left_own.i.i278.i, %bb10.i79.3.i331.i ], [ %left_own16.i.i215.i, %bb19.i.3.i268.i ], [ %_79.i.i91.i, %bb33.i60.3.i173.i ], !dbg !10782
  %_41.i83.i.sroa.0.0.copyload.i190.i = load <4 x float>, ptr %587, align 16, !dbg !10836, !alias.scope !10581, !noalias !10848
  %648 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_41.i83.i.sroa.0.0.copyload.i190.i, i8 1), !dbg !10855
  %649 = bitcast <4 x float> %648 to <4 x i32>, !dbg !10861
  %650 = icmp slt <4 x i32> %649, zeroinitializer, !dbg !10865
  %651 = insertelement <4 x float> poison, float %taps.i.sroa.0.0.i189.i, i64 0, !dbg !10867
  %652 = insertelement <4 x float> %651, float %taps.i.sroa.7.0.i188.i, i64 1, !dbg !10867
  %653 = insertelement <4 x float> %652, float %taps.i.sroa.10.0.i187.i, i64 2, !dbg !10867
  %654 = insertelement <4 x float> %653, float %taps.i.sroa.13.0.i186.i, i64 3, !dbg !10867
  %655 = bitcast <4 x float> %654 to <2 x i64>, !dbg !10871
  %656 = and <2 x i64> %655, splat (i64 9223372034707292159), !dbg !10872
  %657 = bitcast <2 x i64> %656 to <4 x float>, !dbg !10878
  %658 = fmul <4 x float> %657, splat (float 5.000000e-01), !dbg !10879
  %659 = insertelement <4 x float> poison, float %taps.i.sroa.161771.0.i.i, i64 0, !dbg !10884
  %660 = insertelement <4 x float> %659, float %taps.i.sroa.21.0.i185.i, i64 1, !dbg !10884
  %661 = insertelement <4 x float> %660, float %taps.i.sroa.24.0.i184.i, i64 2, !dbg !10884
  %662 = insertelement <4 x float> %661, float %taps.i.sroa.27.0.i183.i, i64 3, !dbg !10884
  %663 = bitcast <4 x float> %662 to <2 x i64>, !dbg !10889
  %664 = and <2 x i64> %663, splat (i64 9223372034707292159), !dbg !10890
  %665 = bitcast <2 x i64> %664 to <4 x float>, !dbg !10896
  %666 = fmul <4 x float> %665, splat (float 5.000000e-01), !dbg !10897
  %667 = fadd <4 x float> %666, %658, !dbg !10902
  %_37.i87.i.sroa.0.0.copyload.i191.i = load <4 x float>, ptr %586, align 16, !dbg !10907, !alias.scope !10581, !noalias !10848
  %668 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_37.i87.i.sroa.0.0.copyload.i191.i, i8 1), !dbg !10908
  %669 = bitcast <4 x float> %668 to <4 x i32>, !dbg !10914
  %670 = icmp slt <4 x i32> %669, zeroinitializer, !dbg !10918
  %671 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %657, <4 x float> %665), !dbg !10920
  %672 = select <4 x i1> %670, <4 x float> %671, <4 x float> %657, !dbg !10918
  %673 = select <4 x i1> %650, <4 x float> %667, <4 x float> %672, !dbg !10865
  %674 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %673, <4 x float> splat (float 0x3E45798EE0000000)), !dbg !10925
  %675 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %674, <4 x float> splat (float 0x3810000000000000)), !dbg !10930
  %676 = bitcast <4 x float> %675 to <2 x i64>, !dbg !10938
  %677 = and <2 x i64> %676, splat (i64 36028792732385279), !dbg !10939
  %678 = or disjoint <2 x i64> %677, splat (i64 4575657222473777152), !dbg !10944
  %679 = bitcast <2 x i64> %678 to <4 x float>, !dbg !10948
  %680 = fadd <4 x float> %679, splat (float -1.000000e+00), !dbg !10949
  %681 = fmul <4 x float> %680, splat (float 0x3F9B17A960000000), !dbg !10954
  %682 = fsub <4 x float> splat (float 0x3FBF9A8440000000), %681, !dbg !10959
  %683 = fmul <4 x float> %680, %682, !dbg !10954
  %684 = fadd <4 x float> %683, splat (float 0xBFD1E3F400000000), !dbg !10959
  %685 = fmul <4 x float> %680, %684, !dbg !10954
  %686 = fadd <4 x float> %685, splat (float 0x3FDD544F20000000), !dbg !10959
  %687 = fmul <4 x float> %680, %686, !dbg !10954
  %688 = fadd <4 x float> %687, splat (float 0xBFE6FC2A60000000), !dbg !10959
  %689 = fmul <4 x float> %680, %688, !dbg !10954
  %690 = fadd <4 x float> %689, splat (float 0x3FF714B2A0000000), !dbg !10959
  %691 = bitcast <4 x float> %675 to <4 x i32>, !dbg !10964
  %_3.i831.i.i = lshr <4 x i32> %691, splat (i32 23), !dbg !10968
  %692 = bitcast <4 x i32> %_3.i831.i.i to <2 x i64>, !dbg !10969
  %693 = or disjoint <2 x i64> %692, splat (i64 5404319554102886400), !dbg !10970
  %694 = bitcast <2 x i64> %693 to <4 x float>, !dbg !10974
  %695 = fadd <4 x float> %694, splat (float 0xC160000FE0000000), !dbg !10975
  %hysteresis.i93.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %585, align 16, !dbg !10979, !alias.scope !10581, !noalias !10980
  %range.i94.i.sroa.0.0.copyload1492.i.i = load <2 x i64>, ptr %584, align 16, !dbg !10982, !alias.scope !10581, !noalias !10980
  %ratio.i95.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %583, align 16, !dbg !10983, !alias.scope !10581, !noalias !10980
  %threshold.i96.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %13, align 16, !dbg !10984, !alias.scope !10581, !noalias !10980
  %696 = fmul <4 x float> %680, %690, !dbg !10985
  %697 = fadd <4 x float> %695, %696, !dbg !10990
  %698 = fmul <4 x float> %697, splat (float 0x4018151820000000), !dbg !10995
  %699 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %698, <4 x float> splat (float 2.400000e+01)), !dbg !11000
  %700 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %699, <4 x float> splat (float -1.600000e+02)), !dbg !11005
  %_55.i69.i.sroa.0.0.copyload.i192.i = load <4 x float>, ptr %588, align 16, !dbg !11010, !alias.scope !10581, !noalias !10980
  %701 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_55.i69.i.sroa.0.0.copyload.i192.i, i8 1), !dbg !11012
  %702 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %threshold.i96.i.sroa.0.0.copyload.i.i, <4 x float> %700, i8 2), !dbg !11018
  %703 = fsub <4 x float> %threshold.i96.i.sroa.0.0.copyload.i.i, %hysteresis.i93.i.sroa.0.0.copyload.i.i, !dbg !11025
  %704 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %703, <4 x float> %700, i8 2), !dbg !11031
  %705 = bitcast <4 x float> %701 to <2 x i64>, !dbg !11037
  %706 = xor <2 x i64> %705, splat (i64 -1), !dbg !11043
  %707 = bitcast <4 x float> %702 to <2 x i64>, !dbg !11045
  %708 = and <2 x i64> %707, %706, !dbg !11049
  %709 = bitcast <4 x float> %704 to <2 x i64>, !dbg !11051
  %710 = and <2 x i64> %709, %705, !dbg !11056
  %711 = or <2 x i64> %710, %708, !dbg !11058
  %712 = xor <2 x i64> %709, splat (i64 -1), !dbg !11064
  %_67.i57.i.sroa.0.0.copyload.i193.i = load <4 x float>, ptr %589, align 16, !dbg !11071, !alias.scope !10581, !noalias !10980
  %713 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_67.i57.i.sroa.0.0.copyload.i193.i, i8 1), !dbg !11072
  %714 = bitcast <4 x float> %713 to <2 x i64>, !dbg !11078
  %715 = and <2 x i64> %712, %714, !dbg !11082
  %716 = and <2 x i64> %715, %705, !dbg !11082
  %717 = or <2 x i64> %716, %711, !dbg !11087
  %718 = bitcast <2 x i64> %717 to <4 x i32>, !dbg !11093
  %719 = icmp slt <4 x i32> %718, zeroinitializer, !dbg !11097
  %720 = select <4 x i1> %719, <4 x float> splat (float 1.000000e+00), <4 x float> zeroinitializer, !dbg !11097
  %_71.i53.i.sroa.0.0.copyload.i194.i = load <4 x float>, ptr %590, align 16, !dbg !11099, !alias.scope !10581, !noalias !10848
  %721 = fadd <4 x float> %_67.i57.i.sroa.0.0.copyload.i193.i, splat (float -1.000000e+00), !dbg !11101
  %722 = bitcast <2 x i64> %716 to <4 x i32>, !dbg !11106
  %723 = icmp slt <4 x i32> %722, zeroinitializer, !dbg !11110
  %724 = select <4 x i1> %723, <4 x float> %721, <4 x float> %_67.i57.i.sroa.0.0.copyload.i193.i, !dbg !11110
  %725 = bitcast <2 x i64> %711 to <4 x i32>, !dbg !11112
  %726 = icmp slt <4 x i32> %725, zeroinitializer, !dbg !11116
  %727 = select <4 x i1> %726, <4 x float> %_71.i53.i.sroa.0.0.copyload.i194.i, <4 x float> %724, !dbg !11116
  store <4 x float> %727, ptr %589, align 16, !dbg !11118, !alias.scope !10581, !noalias !10980
  store <4 x float> %720, ptr %588, align 16, !dbg !11119, !alias.scope !10581, !noalias !10980
  %_86.i38.i.sroa.0.0.copyload.i195.i = load <4 x float>, ptr %591, align 16, !dbg !11120, !alias.scope !10581, !noalias !10980
  %_88.i37.i.sroa.0.0.copyload.i196.i = load <4 x float>, ptr %592, align 16, !dbg !11123, !alias.scope !10581, !noalias !10848
  %_7.i747.i.i = load <4 x float>, ptr %578, align 16, !dbg !11124, !alias.scope !11126, !noalias !11129
  %728 = fadd <4 x float> %ratio.i95.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !11133
  %729 = fsub <4 x float> %700, %threshold.i96.i.sroa.0.0.copyload.i.i, !dbg !11138
  %730 = fmul <4 x float> %728, %729, !dbg !11143
  %731 = xor <2 x i64> %range.i94.i.sroa.0.0.copyload1492.i.i, splat (i64 -9223372034707292160), !dbg !11148
  %732 = bitcast <2 x i64> %731 to <4 x float>, !dbg !11153
  %733 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %730, <4 x float> %732), !dbg !11154
  %734 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %733, <4 x float> zeroinitializer), !dbg !11159
  %735 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %720, i8 1), !dbg !11164
  %736 = bitcast <4 x float> %735 to <4 x i32>, !dbg !11170
  %737 = icmp slt <4 x i32> %736, zeroinitializer, !dbg !11174
  %738 = select <4 x i1> %737, <4 x float> zeroinitializer, <4 x float> %734, !dbg !11174
  %739 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_86.i38.i.sroa.0.0.copyload.i195.i, <4 x float> %738, i8 1), !dbg !11176
  %740 = bitcast <4 x float> %739 to <4 x i32>, !dbg !11182
  %741 = icmp slt <4 x i32> %740, zeroinitializer, !dbg !11185
  %742 = select <4 x i1> %741, <4 x float> %_7.i747.i.i, <4 x float> %_88.i37.i.sroa.0.0.copyload.i196.i, !dbg !11185
  %743 = fsub <4 x float> %738, %_86.i38.i.sroa.0.0.copyload.i195.i, !dbg !11187
  %744 = fmul <4 x float> %743, %742, !dbg !11193
  %745 = fadd <4 x float> %_86.i38.i.sroa.0.0.copyload.i195.i, %744, !dbg !11198
  %746 = bitcast <4 x float> %745 to <2 x i64>, !dbg !11202
  %747 = and <2 x i64> %746, splat (i64 9223372034707292159), !dbg !11209
  %748 = bitcast <2 x i64> %747 to <4 x float>, !dbg !11202
  %749 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %748, <4 x float> splat (float 0x3BC79CA100000000), i8 1), !dbg !11211
  %750 = bitcast <4 x float> %749 to <2 x i64>, !dbg !11217
  %751 = xor <2 x i64> %750, splat (i64 -1), !dbg !11222
  %752 = and <2 x i64> %746, %751, !dbg !11224
  store <2 x i64> %752, ptr %591, align 16, !dbg !11228, !alias.scope !10581, !noalias !10980
  %753 = bitcast <2 x i64> %752 to <4 x float>, !dbg !11230
  %754 = fmul <4 x float> %753, splat (float 0x3FC542A5A0000000), !dbg !11231
  %755 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %754, <4 x float> splat (float -1.260000e+02)), !dbg !11238
  %756 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %755, <4 x float> splat (float 1.270000e+02)), !dbg !11244
  %757 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %756), !dbg !11249
  %758 = fsub <4 x float> %756, %757, !dbg !11254
  %759 = fmul <4 x float> %758, splat (float 0x3F5E974FA0000000), !dbg !11259
  %760 = fadd <4 x float> %759, splat (float 0x3F82778560000000), !dbg !11264
  %761 = fmul <4 x float> %758, %760, !dbg !11259
  %762 = fadd <4 x float> %761, splat (float 0x3FAC91CE60000000), !dbg !11264
  %763 = fmul <4 x float> %758, %762, !dbg !11259
  %764 = fadd <4 x float> %763, splat (float 0x3FCEBDB560000000), !dbg !11264
  %765 = fmul <4 x float> %758, %764, !dbg !11259
  %766 = fadd <4 x float> %765, splat (float 0x3FE62E4BA0000000), !dbg !11264
  %_98.i27.i.sroa.0.0.copyload.i197.i = load <4 x float>, ptr %593, align 16, !dbg !11269, !alias.scope !10581, !noalias !10848
  %767 = fmul <4 x float> %758, %766, !dbg !11271
  %768 = fadd <4 x float> %767, splat (float 1.000000e+00), !dbg !11276
  %769 = fadd <4 x float> %757, splat (float 0x4160000FE0000000), !dbg !11281
  %770 = bitcast <4 x float> %769 to <4 x i32>, !dbg !11286
  %_3.i832.i.i = shl <4 x i32> %770, splat (i32 23), !dbg !11290
  %771 = bitcast <4 x i32> %_3.i832.i.i to <4 x float>, !dbg !11291
  %772 = fmul <4 x float> %768, %771, !dbg !11293
  %773 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %753, <4 x float> zeroinitializer, i8 0), !dbg !11297
  %774 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_98.i27.i.sroa.0.0.copyload.i197.i, i8 1), !dbg !11303
  %775 = bitcast <4 x float> %773 to <2 x i64>, !dbg !11309
  %776 = bitcast <4 x float> %774 to <2 x i64>, !dbg !11309
  %777 = or <2 x i64> %776, %775, !dbg !11313
  %778 = fmul <4 x float> %lanes.i447.sroa.0.0.copyload.i.i, %772, !dbg !11315
  %779 = bitcast <2 x i64> %777 to <4 x i32>, !dbg !11321
  %780 = icmp slt <4 x i32> %779, zeroinitializer, !dbg !11325
  %781 = select <4 x i1> %780, <4 x float> %lanes.i447.sroa.0.0.copyload.i.i, <4 x float> %778, !dbg !11325
  %_41.i.i.sroa.0.0.copyload.i198.i = load <4 x float>, ptr %598, align 16, !dbg !11327, !alias.scope !10581, !noalias !11330
  %782 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_41.i.i.sroa.0.0.copyload.i198.i, i8 1), !dbg !11337
  %783 = bitcast <4 x float> %782 to <4 x i32>, !dbg !11343
  %784 = icmp slt <4 x i32> %783, zeroinitializer, !dbg !11347
  %785 = insertelement <4 x float> poison, float %taps.i.sroa.301776.0.i.i, i64 0, !dbg !11349
  %786 = insertelement <4 x float> %785, float %taps.i.sroa.35.0.i182.i, i64 1, !dbg !11349
  %787 = insertelement <4 x float> %786, float %taps.i.sroa.38.0.i181.i, i64 2, !dbg !11349
  %788 = insertelement <4 x float> %787, float %taps.i.sroa.41.0.i180.i, i64 3, !dbg !11349
  %789 = bitcast <4 x float> %788 to <2 x i64>, !dbg !11354
  %790 = and <2 x i64> %789, splat (i64 9223372034707292159), !dbg !11355
  %791 = bitcast <2 x i64> %790 to <4 x float>, !dbg !11361
  %792 = fmul <4 x float> %791, splat (float 5.000000e-01), !dbg !11362
  %793 = insertelement <4 x float> poison, float %taps.i.sroa.441781.0.i.i, i64 0, !dbg !11367
  %794 = insertelement <4 x float> %793, float %taps.i.sroa.49.0.i179.i, i64 1, !dbg !11367
  %795 = insertelement <4 x float> %794, float %taps.i.sroa.52.0.i178.i, i64 2, !dbg !11367
  %796 = insertelement <4 x float> %795, float %taps.i.sroa.55.0.i177.i, i64 3, !dbg !11367
  %797 = bitcast <4 x float> %796 to <2 x i64>, !dbg !11372
  %798 = and <2 x i64> %797, splat (i64 9223372034707292159), !dbg !11373
  %799 = bitcast <2 x i64> %798 to <4 x float>, !dbg !11379
  %800 = fmul <4 x float> %799, splat (float 5.000000e-01), !dbg !11380
  %801 = fadd <4 x float> %800, %792, !dbg !11385
  %_37.i.i.sroa.0.0.copyload.i199.i = load <4 x float>, ptr %597, align 16, !dbg !11390, !alias.scope !10581, !noalias !11330
  %802 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_37.i.i.sroa.0.0.copyload.i199.i, i8 1), !dbg !11391
  %803 = bitcast <4 x float> %802 to <4 x i32>, !dbg !11397
  %804 = icmp slt <4 x i32> %803, zeroinitializer, !dbg !11401
  %805 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %791, <4 x float> %799), !dbg !11403
  %806 = select <4 x i1> %804, <4 x float> %805, <4 x float> %791, !dbg !11401
  %807 = select <4 x i1> %784, <4 x float> %801, <4 x float> %806, !dbg !11347
  %808 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %807, <4 x float> splat (float 0x3E45798EE0000000)), !dbg !11408
  %809 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %808, <4 x float> splat (float 0x3810000000000000)), !dbg !11413
  %810 = bitcast <4 x float> %809 to <2 x i64>, !dbg !11420
  %811 = and <2 x i64> %810, splat (i64 36028792732385279), !dbg !11421
  %812 = or disjoint <2 x i64> %811, splat (i64 4575657222473777152), !dbg !11426
  %813 = bitcast <2 x i64> %812 to <4 x float>, !dbg !11430
  %814 = fadd <4 x float> %813, splat (float -1.000000e+00), !dbg !11431
  %815 = fmul <4 x float> %814, splat (float 0x3F9B17A960000000), !dbg !11436
  %816 = fsub <4 x float> splat (float 0x3FBF9A8440000000), %815, !dbg !11441
  %817 = fmul <4 x float> %814, %816, !dbg !11436
  %818 = fadd <4 x float> %817, splat (float 0xBFD1E3F400000000), !dbg !11441
  %819 = fmul <4 x float> %814, %818, !dbg !11436
  %820 = fadd <4 x float> %819, splat (float 0x3FDD544F20000000), !dbg !11441
  %821 = fmul <4 x float> %814, %820, !dbg !11436
  %822 = fadd <4 x float> %821, splat (float 0xBFE6FC2A60000000), !dbg !11441
  %823 = fmul <4 x float> %814, %822, !dbg !11436
  %824 = fadd <4 x float> %823, splat (float 0x3FF714B2A0000000), !dbg !11441
  %825 = bitcast <4 x float> %809 to <4 x i32>, !dbg !11446
  %_3.i835.i.i = lshr <4 x i32> %825, splat (i32 23), !dbg !11450
  %826 = bitcast <4 x i32> %_3.i835.i.i to <2 x i64>, !dbg !11451
  %827 = or disjoint <2 x i64> %826, splat (i64 5404319554102886400), !dbg !11452
  %828 = bitcast <2 x i64> %827 to <4 x float>, !dbg !11456
  %829 = fadd <4 x float> %828, splat (float 0xC160000FE0000000), !dbg !11457
  %hysteresis.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %596, align 16, !dbg !11461, !alias.scope !10581, !noalias !11462
  %range.i.i.sroa.0.0.copyload1499.i.i = load <2 x i64>, ptr %595, align 16, !dbg !11464, !alias.scope !10581, !noalias !11462
  %ratio.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %594, align 16, !dbg !11465, !alias.scope !10581, !noalias !11462
  %threshold.i.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %data.i.i.i23.i, align 16, !dbg !11466, !alias.scope !10581, !noalias !11462
  %830 = fmul <4 x float> %814, %824, !dbg !11467
  %831 = fadd <4 x float> %829, %830, !dbg !11472
  %832 = fmul <4 x float> %831, splat (float 0x4018151820000000), !dbg !11477
  %833 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %832, <4 x float> splat (float 2.400000e+01)), !dbg !11482
  %834 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %833, <4 x float> splat (float -1.600000e+02)), !dbg !11487
  %_55.i.i.sroa.0.0.copyload.i200.i = load <4 x float>, ptr %599, align 16, !dbg !11492, !alias.scope !10581, !noalias !11462
  %835 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_55.i.i.sroa.0.0.copyload.i200.i, i8 1), !dbg !11493
  %836 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %threshold.i.i.sroa.0.0.copyload.i.i, <4 x float> %834, i8 2), !dbg !11499
  %837 = fsub <4 x float> %threshold.i.i.sroa.0.0.copyload.i.i, %hysteresis.i.i.sroa.0.0.copyload.i.i, !dbg !11505
  %838 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %837, <4 x float> %834, i8 2), !dbg !11510
  %839 = bitcast <4 x float> %835 to <2 x i64>, !dbg !11516
  %840 = xor <2 x i64> %839, splat (i64 -1), !dbg !11521
  %841 = bitcast <4 x float> %836 to <2 x i64>, !dbg !11523
  %842 = and <2 x i64> %841, %840, !dbg !11527
  %843 = bitcast <4 x float> %838 to <2 x i64>, !dbg !11529
  %844 = and <2 x i64> %843, %839, !dbg !11533
  %845 = or <2 x i64> %844, %842, !dbg !11535
  %846 = xor <2 x i64> %843, splat (i64 -1), !dbg !11540
  %_67.i.i.sroa.0.0.copyload.i201.i = load <4 x float>, ptr %600, align 16, !dbg !11546, !alias.scope !10581, !noalias !11462
  %847 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_67.i.i.sroa.0.0.copyload.i201.i, i8 1), !dbg !11547
  %848 = bitcast <4 x float> %847 to <2 x i64>, !dbg !11553
  %849 = and <2 x i64> %846, %848, !dbg !11557
  %850 = and <2 x i64> %849, %839, !dbg !11557
  %851 = or <2 x i64> %850, %845, !dbg !11562
  %852 = bitcast <2 x i64> %851 to <4 x i32>, !dbg !11567
  %853 = icmp slt <4 x i32> %852, zeroinitializer, !dbg !11571
  %854 = select <4 x i1> %853, <4 x float> splat (float 1.000000e+00), <4 x float> zeroinitializer, !dbg !11571
  %_71.i.i.sroa.0.0.copyload.i202.i = load <4 x float>, ptr %601, align 16, !dbg !11573, !alias.scope !10581, !noalias !11330
  %855 = fadd <4 x float> %_67.i.i.sroa.0.0.copyload.i201.i, splat (float -1.000000e+00), !dbg !11574
  %856 = bitcast <2 x i64> %850 to <4 x i32>, !dbg !11579
  %857 = icmp slt <4 x i32> %856, zeroinitializer, !dbg !11583
  %858 = select <4 x i1> %857, <4 x float> %855, <4 x float> %_67.i.i.sroa.0.0.copyload.i201.i, !dbg !11583
  %859 = bitcast <2 x i64> %845 to <4 x i32>, !dbg !11585
  %860 = icmp slt <4 x i32> %859, zeroinitializer, !dbg !11589
  %861 = select <4 x i1> %860, <4 x float> %_71.i.i.sroa.0.0.copyload.i202.i, <4 x float> %858, !dbg !11589
  store <4 x float> %861, ptr %600, align 16, !dbg !11591, !alias.scope !10581, !noalias !11462
  store <4 x float> %854, ptr %599, align 16, !dbg !11592, !alias.scope !10581, !noalias !11462
  %_86.i.i.sroa.0.0.copyload.i203.i = load <4 x float>, ptr %602, align 16, !dbg !11593, !alias.scope !10581, !noalias !11462
  %_88.i.i.sroa.0.0.copyload.i204.i = load <4 x float>, ptr %603, align 16, !dbg !11594, !alias.scope !10581, !noalias !11330
  %_7.i715.i.i = load <4 x float>, ptr %_38.i33.i, align 16, !dbg !11595, !alias.scope !11597, !noalias !11600
  %862 = fadd <4 x float> %ratio.i.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !11604
  %863 = fsub <4 x float> %834, %threshold.i.i.sroa.0.0.copyload.i.i, !dbg !11609
  %864 = fmul <4 x float> %862, %863, !dbg !11614
  %865 = xor <2 x i64> %range.i.i.sroa.0.0.copyload1499.i.i, splat (i64 -9223372034707292160), !dbg !11619
  %866 = bitcast <2 x i64> %865 to <4 x float>, !dbg !11624
  %867 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %864, <4 x float> %866), !dbg !11625
  %868 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %867, <4 x float> zeroinitializer), !dbg !11630
  %869 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %854, i8 1), !dbg !11635
  %870 = bitcast <4 x float> %869 to <4 x i32>, !dbg !11641
  %871 = icmp slt <4 x i32> %870, zeroinitializer, !dbg !11645
  %872 = select <4 x i1> %871, <4 x float> zeroinitializer, <4 x float> %868, !dbg !11645
  %873 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_86.i.i.sroa.0.0.copyload.i203.i, <4 x float> %872, i8 1), !dbg !11647
  %874 = bitcast <4 x float> %873 to <4 x i32>, !dbg !11653
  %875 = icmp slt <4 x i32> %874, zeroinitializer, !dbg !11656
  %876 = select <4 x i1> %875, <4 x float> %_7.i715.i.i, <4 x float> %_88.i.i.sroa.0.0.copyload.i204.i, !dbg !11656
  %877 = fsub <4 x float> %872, %_86.i.i.sroa.0.0.copyload.i203.i, !dbg !11658
  %878 = fmul <4 x float> %877, %876, !dbg !11663
  %879 = fadd <4 x float> %_86.i.i.sroa.0.0.copyload.i203.i, %878, !dbg !11668
  %880 = bitcast <4 x float> %879 to <2 x i64>, !dbg !11672
  %881 = and <2 x i64> %880, splat (i64 9223372034707292159), !dbg !11678
  %882 = bitcast <2 x i64> %881 to <4 x float>, !dbg !11672
  %883 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %882, <4 x float> splat (float 0x3BC79CA100000000), i8 1), !dbg !11680
  %884 = bitcast <4 x float> %883 to <2 x i64>, !dbg !11686
  %885 = xor <2 x i64> %884, splat (i64 -1), !dbg !11691
  %886 = and <2 x i64> %880, %885, !dbg !11693
  store <2 x i64> %886, ptr %602, align 16, !dbg !11697, !alias.scope !10581, !noalias !11462
  %887 = bitcast <2 x i64> %886 to <4 x float>, !dbg !11698
  %888 = fmul <4 x float> %887, splat (float 0x3FC542A5A0000000), !dbg !11699
  %889 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %888, <4 x float> splat (float -1.260000e+02)), !dbg !11705
  %890 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %889, <4 x float> splat (float 1.270000e+02)), !dbg !11711
  %891 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %890), !dbg !11716
  %892 = fsub <4 x float> %890, %891, !dbg !11721
  %893 = fmul <4 x float> %892, splat (float 0x3F5E974FA0000000), !dbg !11726
  %894 = fadd <4 x float> %893, splat (float 0x3F82778560000000), !dbg !11731
  %895 = fmul <4 x float> %892, %894, !dbg !11726
  %896 = fadd <4 x float> %895, splat (float 0x3FAC91CE60000000), !dbg !11731
  %897 = fmul <4 x float> %892, %896, !dbg !11726
  %898 = fadd <4 x float> %897, splat (float 0x3FCEBDB560000000), !dbg !11731
  %899 = fmul <4 x float> %892, %898, !dbg !11726
  %900 = fadd <4 x float> %899, splat (float 0x3FE62E4BA0000000), !dbg !11731
  %901 = fmul <4 x float> %892, %900, !dbg !11736
  %902 = fadd <4 x float> %901, splat (float 1.000000e+00), !dbg !11741
  %903 = fadd <4 x float> %891, splat (float 0x4160000FE0000000), !dbg !11746
  %904 = bitcast <4 x float> %903 to <4 x i32>, !dbg !11751
  %_3.i836.i.i = shl <4 x i32> %904, splat (i32 23), !dbg !11755
  %905 = bitcast <4 x i32> %_3.i836.i.i to <4 x float>, !dbg !11756
  %906 = fmul <4 x float> %902, %905, !dbg !11758
  %907 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %887, <4 x float> zeroinitializer, i8 0), !dbg !11762
  %_98.i.i.sroa.0.0.copyload.i205.i = load <4 x float>, ptr %604, align 16, !dbg !11768, !alias.scope !10581, !noalias !11330
  %908 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_98.i.i.sroa.0.0.copyload.i205.i, i8 1), !dbg !11769
  %909 = bitcast <4 x float> %907 to <2 x i64>, !dbg !11775
  %910 = bitcast <4 x float> %908 to <2 x i64>, !dbg !11775
  %911 = or <2 x i64> %910, %909, !dbg !11779
  %912 = fmul <4 x float> %lanes.i441.sroa.0.0.copyload.i.i, %906, !dbg !11781
  %913 = bitcast <2 x i64> %911 to <4 x i32>, !dbg !11786
  %914 = icmp slt <4 x i32> %913, zeroinitializer, !dbg !11790
  %915 = select <4 x i1> %914, <4 x float> %lanes.i441.sroa.0.0.copyload.i.i, <4 x float> %912, !dbg !11790
  store <4 x float> %781, ptr %_97.i.i55.i, align 4, !dbg !11792, !alias.scope !11798, !noalias !11802
  store <4 x float> %915, ptr %_115.i.i61.i, align 4, !dbg !11806, !alias.scope !11811, !noalias !11815
  %exitcond1766.not.i.i = icmp eq i64 %613, %_26.i, !dbg !11819
  br i1 %exitcond1766.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKB1r_EB3_.exit.i, label %bb30.i.i46.i, !dbg !10638

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKB1r_EB3_.exit.i: ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i176.i
  %_82.i.i206.i = trunc i64 %_26.i to i32, !dbg !11822
  %_81.i.i207.i = add i32 %base.i.i45.i, %_82.i.i206.i, !dbg !11823
  store i32 %_81.i.i207.i, ptr %_57.i38.i, align 4, !dbg !11825, !alias.scope !10581, !noalias !10635
  br label %bb7.i, !dbg !11826

bb26.i:                                           ; preds = %bb25.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %575, i64 noundef range(i64 0, 2305843009213693952) %_38.1, i64 noundef range(i64 0, 2305843009213693952) %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6d4388bd1c2ee005f6a969a0e3ca0f4) #24, !dbg !11827, !noalias !8453
  unreachable, !dbg !11827

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9run_blockB2_.exit: ; preds = %bb1.backedge.i.i
  call void @llvm.lifetime.end.p0(ptr nonnull %iter.i.i), !dbg !11828, !noalias !10063
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(160) %report.sroa.0, ptr noundef nonnull align 8 dereferenceable(160) %reports, i64 160, i1 false), !dbg !11829, !alias.scope !11836, !noalias !11840
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %_0, ptr noundef nonnull align 8 dereferenceable(320) %report.sroa.0, i64 320, i1 false), !dbg !11842
  %report.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 320, !dbg !11842
  store i8 %1, ptr %report.sroa.7.0._0.sroa_idx, align 8, !dbg !11842
  call void @llvm.lifetime.end.p0(ptr nonnull %reports), !dbg !11843
  br label %bb12, !dbg !8424

bb15:                                             ; preds = %bb4, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit
  %iter.sroa.0.0181 = phi i64 [ 0, %bb4 ], [ %916, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit ]
  %916 = add nuw nsw i64 %iter.sroa.0.0181, 1, !dbg !11844
  %exitcond.not = icmp eq i64 %iter.sroa.0.0181, %_36.1, !dbg !11850
  br i1 %exitcond.not, label %panic, label %bb7, !dbg !11850

bb12:                                             ; preds = %bb3, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9run_blockB2_.exit
  call void @llvm.lifetime.end.p0(ptr nonnull %report.sroa.0), !dbg !11852
  ret void, !dbg !8424

bb7:                                              ; preds = %bb15
  %917 = getelementptr inbounds nuw i32, ptr %_36.0, i64 %iter.sroa.0.0181, !dbg !11850
  %_14 = load i32, ptr %917, align 4, !dbg !11850, !noundef !12
  %start1 = zext i32 %_14 to i64, !dbg !11850
  %exitcond337.not = icmp eq i64 %iter.sroa.0.0181, %15, !dbg !11853
  br i1 %exitcond337.not, label %panic2, label %bb8, !dbg !11853

panic:                                            ; preds = %bb15
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_36.1, i64 noundef %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_99ab90c4f654517a4481f4facf03e67c) #24, !dbg !11850
  unreachable, !dbg !11850

bb8:                                              ; preds = %bb7
  %918 = getelementptr inbounds nuw i32, ptr %_36.0, i64 %916, !dbg !11853
  %_18 = load i32, ptr %918, align 4, !dbg !11853, !noundef !12
  %end = zext i32 %_18 to i64, !dbg !11853
  %_58 = icmp ult i32 %_18, %_14, !dbg !11855
  %_52.not = icmp ult i64 %_39.1, %end
  %or.cond24 = or i1 %_58, %_52.not, !dbg !11855
  br i1 %or.cond24, label %bb19, label %bb9, !dbg !11855, !prof !2561

panic2:                                           ; preds = %bb7
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %916, i64 noundef %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_99ab90c4f654517a4481f4facf03e67c) #24, !dbg !11853
  unreachable, !dbg !11853

bb19:                                             ; preds = %bb8
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1, i64 noundef %end, i64 noundef %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_99ab90c4f654517a4481f4facf03e67c) #24, !dbg !11863
  unreachable, !dbg !11863

bb9:                                              ; preds = %bb8
  %_59 = sub nuw nsw i64 %end, %start1, !dbg !11864
  %_61 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_39.0, i64 %start1, !dbg !11865
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11869), !dbg !11872
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11873), !dbg !11872
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11875), !dbg !11872
  call void @llvm.lifetime.start.p0(ptr nonnull %pending.i), !dbg !11877, !noalias !11880
  store i32 0, ptr %pending.i, align 4, !noalias !11880
  store i32 0, ptr %_7.sroa.5128.0.pending.sroa_idx.i, align 4, !noalias !11880
  store i32 0, ptr %_7.sroa.6131.0.pending.sroa_idx.i, align 4, !noalias !11880
  store i32 0, ptr %_7.sroa.7134.0.pending.sroa_idx.i, align 4, !noalias !11880
  store i32 0, ptr %11, align 4, !noalias !11880
  store i32 0, ptr %_7.sroa.5128.0..sroa_idx.i, align 4, !noalias !11880
  store i32 0, ptr %_7.sroa.6131.0..sroa_idx.i, align 4, !noalias !11880
  store i32 0, ptr %_7.sroa.7134.0..sroa_idx.i, align 4, !noalias !11880
  %_106.idx.i = mul nuw nsw i64 %_59, 40, !dbg !11881
  %_106.i = getelementptr inbounds nuw i8, ptr %_61, i64 %_106.idx.i, !dbg !11881
  %_6.i.i9194.i = icmp eq i32 %_18, %_14, !dbg !11892
  br i1 %_6.i.i9194.i, label %bb36.preheader.i, label %bb4.lr.ph.lr.ph.i, !dbg !11897

bb4.lr.ph.lr.ph.i:                                ; preds = %bb9
  %_24 = getelementptr inbounds nuw %"effect_contract::ProcessReport", ptr %reports, i64 %iter.sroa.0.0181, !dbg !11898
  %919 = getelementptr inbounds nuw i8, ptr %_24, i64 16
  %_31.i = load i32, ptr %12, align 4, !alias.scope !11869, !noalias !11899
  %_30.i = zext i32 %_31.i to i64
  %.promoted99.i = load i64, ptr %919, align 8, !alias.scope !11875, !noalias !11900
  br label %bb4.lr.ph.i, !dbg !11897

bb4.lr.ph.i:                                      ; preds = %bb31.i, %bb4.lr.ph.lr.ph.i
  %.promoted101.i = phi i64 [ %.promoted99.i, %bb4.lr.ph.lr.ph.i ], [ %.promoted100.i, %bb31.i ]
  %last_order.sroa.3.0.ph98.i = phi i32 [ undef, %bb4.lr.ph.lr.ph.i ], [ %_127.0.i, %bb31.i ]
  %last_order.sroa.0.0.ph97.not.i = phi i1 [ true, %bb4.lr.ph.lr.ph.i ], [ false, %bb31.i ]
  %iter.sroa.0.0.ph96.i = phi ptr [ %_61, %bb4.lr.ph.lr.ph.i ], [ %_16.i.i.i, %bb31.i ]
  %iter.sroa.7.0.ph95.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %bb4.i10, !dbg !11897

bb36.preheader.i:                                 ; preds = %bb31.i, %bb35.i, %bb9
  %920 = getelementptr inbounds nuw float, ptr %words.i.i, i64 %iter.sroa.0.0181
  %921 = getelementptr inbounds nuw float, ptr %words.i66.i, i64 %iter.sroa.0.0181
  %922 = getelementptr inbounds nuw float, ptr %words.i67.i, i64 %iter.sroa.0.0181
  %923 = getelementptr inbounds nuw float, ptr %words.i68.i, i64 %iter.sroa.0.0181
  %924 = getelementptr inbounds nuw float, ptr %words.i69.i, i64 %iter.sroa.0.0181
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, !dbg !11901

bb4.i10:                                          ; preds = %bb35.i, %bb4.lr.ph.i
  %.promoted100.i = phi i64 [ %.promoted101.i, %bb4.lr.ph.i ], [ %971, %bb35.i ]
  %iter.sroa.0.093.i = phi ptr [ %iter.sroa.0.0.ph96.i, %bb4.lr.ph.i ], [ %_16.i.i.i, %bb35.i ]
  %iter.sroa.7.092.i = phi i64 [ %iter.sroa.7.0.ph95.i, %bb4.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 40, !dbg !11909
  %_9.0.i.i = add i64 %iter.sroa.7.092.i, 1, !dbg !11911
  %925 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 32, !dbg !11912
  %_17.i = load i32, ptr %925, align 8, !dbg !11912, !range !1335, !alias.scope !11873, !noalias !11914, !noundef !12
  switch i32 %_17.i, label %default.unreachable [
    i32 1, label %bb9.i12
    i32 2, label %bb7.i11
    i32 3, label %bb35.i
  ], !dbg !11915

bb36.loopexit.i:                                  ; preds = %bb46.us.3.i, %bb40.backedge.us.2.i
  %_6.i.i44.i = icmp eq i64 %iter1.sroa.0.0.add.i, 64, !dbg !11916
  br i1 %_6.i.i44.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, !dbg !11901

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i: ; preds = %bb36.loopexit.i, %bb36.preheader.i
  %iter1.sroa.0.0.idx111.i = phi i64 [ 0, %bb36.preheader.i ], [ %iter1.sroa.0.0.add.i, %bb36.loopexit.i ]
  %iter1.sroa.7.0110.i = phi i64 [ 0, %bb36.preheader.i ], [ %_9.0.i48.i, %bb36.loopexit.i ]
  %iter1.sroa.0.0.ptr.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 %iter1.sroa.0.0.idx111.i, !dbg !11916
  %iter1.sroa.0.0.add.i = add nuw nsw i64 %iter1.sroa.0.0.idx111.i, 32, !dbg !11919
  %_9.0.i48.i = add nuw nsw i64 %iter1.sroa.7.0110.i, 1, !dbg !11922
  %926 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x4_::f32x4>", ptr %13, i64 %iter1.sroa.7.0110.i
  %927 = load i32, ptr %iter1.sroa.0.0.ptr.i, align 4, !dbg !11925, !range !5852, !noalias !11880, !noundef !12
  %928 = trunc nuw i32 %927 to i1, !dbg !11929
  br i1 %928, label %bb46.us.i, label %bb40.backedge.us.i, !dbg !11929

bb46.us.i:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %929 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 4, !dbg !11925
  %value.us.i = load float, ptr %929, align 4, !dbg !11930, !noalias !11880, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !11931, !noalias !11935
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %words.i.i, ptr noundef nonnull align 16 dereferenceable(16) %926, i64 16, i1 false), !dbg !11938, !noalias !11899
  %_0.i.us.i = load float, ptr %920, align 4, !dbg !11943, !noalias !11935, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !11944, !noalias !11935
  %930 = getelementptr inbounds nuw i8, ptr %926, i64 16, !dbg !11945
  %931 = getelementptr inbounds nuw i8, ptr %926, i64 32, !dbg !11946
  %932 = getelementptr inbounds nuw i8, ptr %926, i64 48, !dbg !11947
  %_148.us.i = bitcast float %_0.i.us.i to i32, !dbg !11948
  %_150.us.i = bitcast float %value.us.i to i32, !dbg !11956
  %_149.us.i = icmp ne i32 %_148.us.i, %_150.us.i, !dbg !11959
  %933 = icmp eq i32 %_148.us.i, -2147483648
  %or.cond.us.i = or i1 %_149.us.i, %933, !dbg !11959
  %934 = tail call float @llvm.fabs.f32(float %_0.i.us.i)
  %_144.us.i = fcmp ueq float %934, 0x7FF0000000000000
  %or.cond40.us.i = or i1 %_144.us.i, %or.cond.us.i, !dbg !11959
  %_146.us.i = fsub float %value.us.i, %_0.i.us.i, !dbg !11959
  %935 = fmul float %_146.us.i, 1.562500e-02, !dbg !11959
  %ramp.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %_0.i.us.i, float %value.us.i, !dbg !11959
  %ramp4.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %935, float 0.000000e+00, !dbg !11959
  %ramp5.sroa.0.0.us.i = select i1 %or.cond40.us.i, float 6.400000e+01, float 0.000000e+00, !dbg !11959
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i66.i), !dbg !11960, !noalias !11962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, ptr noundef nonnull align 16 dereferenceable(16) %926, i64 16, i1 false), !dbg !11965, !noalias !11899
  store float %ramp.sroa.0.0.us.i, ptr %921, align 4, !dbg !11966, !noalias !11962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %926, ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, i64 16, i1 false), !dbg !11967, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i66.i), !dbg !11968, !noalias !11962
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i67.i), !dbg !11969, !noalias !11971
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, ptr noundef nonnull align 16 dereferenceable(16) %930, i64 16, i1 false), !dbg !11974, !noalias !11899
  store float %value.us.i, ptr %922, align 4, !dbg !11975, !noalias !11971
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %930, ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, i64 16, i1 false), !dbg !11976, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i67.i), !dbg !11977, !noalias !11971
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i68.i), !dbg !11978, !noalias !11980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, ptr noundef nonnull align 16 dereferenceable(16) %931, i64 16, i1 false), !dbg !11983, !noalias !11899
  store float %ramp4.sroa.0.0.us.i, ptr %923, align 4, !dbg !11984, !noalias !11980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %931, ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, i64 16, i1 false), !dbg !11985, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i68.i), !dbg !11986, !noalias !11980
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i69.i), !dbg !11987, !noalias !11989
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, ptr noundef nonnull align 16 dereferenceable(16) %932, i64 16, i1 false), !dbg !11992, !noalias !11899
  store float %ramp5.sroa.0.0.us.i, ptr %924, align 4, !dbg !11993, !noalias !11989
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %932, ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, i64 16, i1 false), !dbg !11994, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i69.i), !dbg !11995, !noalias !11989
  store i32 64, ptr %14, align 4, !dbg !11996, !alias.scope !11869, !noalias !11899
  br label %bb40.backedge.us.i, !dbg !11997

bb40.backedge.us.i:                               ; preds = %bb46.us.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %iter2.sroa.0.0.ptr106.us.1.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 8, !dbg !11998
  %936 = load i32, ptr %iter2.sroa.0.0.ptr106.us.1.i, align 4, !dbg !11925, !range !5852, !noalias !11880, !noundef !12
  %937 = trunc nuw i32 %936 to i1, !dbg !11929
  br i1 %937, label %bb46.us.1.i, label %bb40.backedge.us.1.i, !dbg !11929

bb46.us.1.i:                                      ; preds = %bb40.backedge.us.i
  %938 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 12, !dbg !11925
  %value.us.1.i = load float, ptr %938, align 4, !dbg !11930, !noalias !11880, !noundef !12
  %slot.us.1.i = getelementptr inbounds nuw i8, ptr %926, i64 64, !dbg !12007
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !11931, !noalias !11935
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %words.i.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.1.i, i64 16, i1 false), !dbg !11938, !noalias !11899
  %_0.i.us.1.i = load float, ptr %920, align 4, !dbg !11943, !noalias !11935, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !11944, !noalias !11935
  %939 = getelementptr inbounds nuw i8, ptr %926, i64 80, !dbg !11945
  %940 = getelementptr inbounds nuw i8, ptr %926, i64 96, !dbg !11946
  %941 = getelementptr inbounds nuw i8, ptr %926, i64 112, !dbg !11947
  %_148.us.1.i = bitcast float %_0.i.us.1.i to i32, !dbg !11948
  %_150.us.1.i = bitcast float %value.us.1.i to i32, !dbg !11956
  %_149.us.1.i = icmp ne i32 %_148.us.1.i, %_150.us.1.i, !dbg !11959
  %942 = icmp eq i32 %_148.us.1.i, -2147483648
  %or.cond.us.1.i = or i1 %_149.us.1.i, %942, !dbg !11959
  %943 = tail call float @llvm.fabs.f32(float %_0.i.us.1.i)
  %_144.us.1.i = fcmp ueq float %943, 0x7FF0000000000000
  %or.cond40.us.1.i = or i1 %_144.us.1.i, %or.cond.us.1.i, !dbg !11959
  %_146.us.1.i = fsub float %value.us.1.i, %_0.i.us.1.i, !dbg !11959
  %944 = fmul float %_146.us.1.i, 1.562500e-02, !dbg !11959
  %ramp.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %_0.i.us.1.i, float %value.us.1.i, !dbg !11959
  %ramp4.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %944, float 0.000000e+00, !dbg !11959
  %ramp5.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !11959
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i66.i), !dbg !11960, !noalias !11962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.1.i, i64 16, i1 false), !dbg !11965, !noalias !11899
  store float %ramp.sroa.0.0.us.1.i, ptr %921, align 4, !dbg !11966, !noalias !11962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %slot.us.1.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, i64 16, i1 false), !dbg !11967, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i66.i), !dbg !11968, !noalias !11962
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i67.i), !dbg !11969, !noalias !11971
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, ptr noundef nonnull align 16 dereferenceable(16) %939, i64 16, i1 false), !dbg !11974, !noalias !11899
  store float %value.us.1.i, ptr %922, align 4, !dbg !11975, !noalias !11971
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %939, ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, i64 16, i1 false), !dbg !11976, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i67.i), !dbg !11977, !noalias !11971
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i68.i), !dbg !11978, !noalias !11980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, ptr noundef nonnull align 16 dereferenceable(16) %940, i64 16, i1 false), !dbg !11983, !noalias !11899
  store float %ramp4.sroa.0.0.us.1.i, ptr %923, align 4, !dbg !11984, !noalias !11980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %940, ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, i64 16, i1 false), !dbg !11985, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i68.i), !dbg !11986, !noalias !11980
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i69.i), !dbg !11987, !noalias !11989
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, ptr noundef nonnull align 16 dereferenceable(16) %941, i64 16, i1 false), !dbg !11992, !noalias !11899
  store float %ramp5.sroa.0.0.us.1.i, ptr %924, align 4, !dbg !11993, !noalias !11989
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %941, ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, i64 16, i1 false), !dbg !11994, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i69.i), !dbg !11995, !noalias !11989
  store i32 64, ptr %14, align 4, !dbg !11996, !alias.scope !11869, !noalias !11899
  br label %bb40.backedge.us.1.i, !dbg !11997

bb40.backedge.us.1.i:                             ; preds = %bb46.us.1.i, %bb40.backedge.us.i
  %iter2.sroa.0.0.ptr106.us.2.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 16, !dbg !11998
  %945 = load i32, ptr %iter2.sroa.0.0.ptr106.us.2.i, align 4, !dbg !11925, !range !5852, !noalias !11880, !noundef !12
  %946 = trunc nuw i32 %945 to i1, !dbg !11929
  br i1 %946, label %bb46.us.2.i, label %bb40.backedge.us.2.i, !dbg !11929

bb46.us.2.i:                                      ; preds = %bb40.backedge.us.1.i
  %947 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 20, !dbg !11925
  %value.us.2.i = load float, ptr %947, align 4, !dbg !11930, !noalias !11880, !noundef !12
  %slot.us.2.i = getelementptr inbounds nuw i8, ptr %926, i64 128, !dbg !12007
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !11931, !noalias !11935
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %words.i.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.2.i, i64 16, i1 false), !dbg !11938, !noalias !11899
  %_0.i.us.2.i = load float, ptr %920, align 4, !dbg !11943, !noalias !11935, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !11944, !noalias !11935
  %948 = getelementptr inbounds nuw i8, ptr %926, i64 144, !dbg !11945
  %949 = getelementptr inbounds nuw i8, ptr %926, i64 160, !dbg !11946
  %950 = getelementptr inbounds nuw i8, ptr %926, i64 176, !dbg !11947
  %_148.us.2.i = bitcast float %_0.i.us.2.i to i32, !dbg !11948
  %_150.us.2.i = bitcast float %value.us.2.i to i32, !dbg !11956
  %_149.us.2.i = icmp ne i32 %_148.us.2.i, %_150.us.2.i, !dbg !11959
  %951 = icmp eq i32 %_148.us.2.i, -2147483648
  %or.cond.us.2.i = or i1 %_149.us.2.i, %951, !dbg !11959
  %952 = tail call float @llvm.fabs.f32(float %_0.i.us.2.i)
  %_144.us.2.i = fcmp ueq float %952, 0x7FF0000000000000
  %or.cond40.us.2.i = or i1 %_144.us.2.i, %or.cond.us.2.i, !dbg !11959
  %_146.us.2.i = fsub float %value.us.2.i, %_0.i.us.2.i, !dbg !11959
  %953 = fmul float %_146.us.2.i, 1.562500e-02, !dbg !11959
  %ramp.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %_0.i.us.2.i, float %value.us.2.i, !dbg !11959
  %ramp4.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %953, float 0.000000e+00, !dbg !11959
  %ramp5.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float 6.400000e+01, float 0.000000e+00, !dbg !11959
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i66.i), !dbg !11960, !noalias !11962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.2.i, i64 16, i1 false), !dbg !11965, !noalias !11899
  store float %ramp.sroa.0.0.us.2.i, ptr %921, align 4, !dbg !11966, !noalias !11962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %slot.us.2.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, i64 16, i1 false), !dbg !11967, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i66.i), !dbg !11968, !noalias !11962
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i67.i), !dbg !11969, !noalias !11971
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, ptr noundef nonnull align 16 dereferenceable(16) %948, i64 16, i1 false), !dbg !11974, !noalias !11899
  store float %value.us.2.i, ptr %922, align 4, !dbg !11975, !noalias !11971
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %948, ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, i64 16, i1 false), !dbg !11976, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i67.i), !dbg !11977, !noalias !11971
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i68.i), !dbg !11978, !noalias !11980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, ptr noundef nonnull align 16 dereferenceable(16) %949, i64 16, i1 false), !dbg !11983, !noalias !11899
  store float %ramp4.sroa.0.0.us.2.i, ptr %923, align 4, !dbg !11984, !noalias !11980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %949, ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, i64 16, i1 false), !dbg !11985, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i68.i), !dbg !11986, !noalias !11980
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i69.i), !dbg !11987, !noalias !11989
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, ptr noundef nonnull align 16 dereferenceable(16) %950, i64 16, i1 false), !dbg !11992, !noalias !11899
  store float %ramp5.sroa.0.0.us.2.i, ptr %924, align 4, !dbg !11993, !noalias !11989
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %950, ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, i64 16, i1 false), !dbg !11994, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i69.i), !dbg !11995, !noalias !11989
  store i32 64, ptr %14, align 4, !dbg !11996, !alias.scope !11869, !noalias !11899
  br label %bb40.backedge.us.2.i, !dbg !11997

bb40.backedge.us.2.i:                             ; preds = %bb46.us.2.i, %bb40.backedge.us.1.i
  %iter2.sroa.0.0.ptr106.us.3.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 24, !dbg !11998
  %954 = load i32, ptr %iter2.sroa.0.0.ptr106.us.3.i, align 4, !dbg !11925, !range !5852, !noalias !11880, !noundef !12
  %955 = trunc nuw i32 %954 to i1, !dbg !11929
  br i1 %955, label %bb46.us.3.i, label %bb36.loopexit.i, !dbg !11929

bb46.us.3.i:                                      ; preds = %bb40.backedge.us.2.i
  %956 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 28, !dbg !11925
  %value.us.3.i = load float, ptr %956, align 4, !dbg !11930, !noalias !11880, !noundef !12
  %slot.us.3.i = getelementptr inbounds nuw i8, ptr %926, i64 192, !dbg !12007
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !11931, !noalias !11935
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %words.i.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.3.i, i64 16, i1 false), !dbg !11938, !noalias !11899
  %_0.i.us.3.i = load float, ptr %920, align 4, !dbg !11943, !noalias !11935, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !11944, !noalias !11935
  %957 = getelementptr inbounds nuw i8, ptr %926, i64 208, !dbg !11945
  %958 = getelementptr inbounds nuw i8, ptr %926, i64 224, !dbg !11946
  %959 = getelementptr inbounds nuw i8, ptr %926, i64 240, !dbg !11947
  %_148.us.3.i = bitcast float %_0.i.us.3.i to i32, !dbg !11948
  %_150.us.3.i = bitcast float %value.us.3.i to i32, !dbg !11956
  %_149.us.3.i = icmp ne i32 %_148.us.3.i, %_150.us.3.i, !dbg !11959
  %960 = icmp eq i32 %_148.us.3.i, -2147483648
  %or.cond.us.3.i = or i1 %_149.us.3.i, %960, !dbg !11959
  %961 = tail call float @llvm.fabs.f32(float %_0.i.us.3.i)
  %_144.us.3.i = fcmp ueq float %961, 0x7FF0000000000000
  %or.cond40.us.3.i = or i1 %_144.us.3.i, %or.cond.us.3.i, !dbg !11959
  %_146.us.3.i = fsub float %value.us.3.i, %_0.i.us.3.i, !dbg !11959
  %962 = fmul float %_146.us.3.i, 1.562500e-02, !dbg !11959
  %ramp.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %_0.i.us.3.i, float %value.us.3.i, !dbg !11959
  %ramp4.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %962, float 0.000000e+00, !dbg !11959
  %ramp5.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float 6.400000e+01, float 0.000000e+00, !dbg !11959
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i66.i), !dbg !11960, !noalias !11962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.3.i, i64 16, i1 false), !dbg !11965, !noalias !11899
  store float %ramp.sroa.0.0.us.3.i, ptr %921, align 4, !dbg !11966, !noalias !11962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %slot.us.3.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, i64 16, i1 false), !dbg !11967, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i66.i), !dbg !11968, !noalias !11962
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i67.i), !dbg !11969, !noalias !11971
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, ptr noundef nonnull align 16 dereferenceable(16) %957, i64 16, i1 false), !dbg !11974, !noalias !11899
  store float %value.us.3.i, ptr %922, align 4, !dbg !11975, !noalias !11971
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %957, ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, i64 16, i1 false), !dbg !11976, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i67.i), !dbg !11977, !noalias !11971
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i68.i), !dbg !11978, !noalias !11980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, ptr noundef nonnull align 16 dereferenceable(16) %958, i64 16, i1 false), !dbg !11983, !noalias !11899
  store float %ramp4.sroa.0.0.us.3.i, ptr %923, align 4, !dbg !11984, !noalias !11980
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %958, ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, i64 16, i1 false), !dbg !11985, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i68.i), !dbg !11986, !noalias !11980
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i69.i), !dbg !11987, !noalias !11989
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, ptr noundef nonnull align 16 dereferenceable(16) %959, i64 16, i1 false), !dbg !11992, !noalias !11899
  store float %ramp5.sroa.0.0.us.3.i, ptr %924, align 4, !dbg !11993, !noalias !11989
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %959, ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, i64 16, i1 false), !dbg !11994, !noalias !11899
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i69.i), !dbg !11995, !noalias !11989
  store i32 64, ptr %14, align 4, !dbg !11996, !alias.scope !11869, !noalias !11899
  br label %bb36.loopexit.i, !dbg !11997

bb7.i11:                                          ; preds = %bb4.i10
  br label %bb9.i12, !dbg !12008

bb9.i12:                                          ; preds = %bb7.i11, %bb4.i10
  %channel.sroa.0.0.sroa.phi28.i = phi ptr [ %11, %bb7.i11 ], [ %pending.i, %bb4.i10 ], !dbg !12009
  %channel.sroa.0.0.i = phi i32 [ 1, %bb7.i11 ], [ 0, %bb4.i10 ], !dbg !12009
  %963 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 16, !dbg !12010
  %_21.i = load i32, ptr %963, align 8, !dbg !12010, !alias.scope !11873, !noalias !11914, !noundef !12
  %parameter_index.i = zext i32 %_21.i to i64, !dbg !12010
  %_121.1.i = icmp slt i32 %_21.i, 0, !dbg !12012
  br i1 %_121.1.i, label %bb35.i, label %bb59.i, !dbg !12018, !prof !180

bb59.i:                                           ; preds = %bb9.i12
  %_121.0.i = shl nuw i32 %_21.i, 1, !dbg !12012
  %_127.0.i = or disjoint i32 %_121.0.i, %channel.sroa.0.0.i, !dbg !12022
  %_29.i = icmp ult i64 %iter.sroa.7.092.i, %_30.i, !dbg !12030
  %_32.i = icmp samesign ult i32 %_21.i, 4
  %or.cond16.i = and i1 %_29.i, %_32.i, !dbg !12030
  br i1 %or.cond16.i, label %bb11.i, label %bb35.i, !dbg !12030

bb11.i:                                           ; preds = %bb59.i
  %964 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 28, !dbg !12032
  %_130.i = load i32, ptr %964, align 4, !dbg !12032, !range !6307, !alias.scope !11873, !noalias !11914, !noundef !12
  %965 = icmp eq i32 %_130.i, 1, !dbg !12035
  br i1 %965, label %bb12.i13, label %bb35.i, !dbg !12035

bb12.i13:                                         ; preds = %bb11.i
  %_34.i = load i64, ptr %iter.sroa.0.093.i, align 8, !dbg !12036, !alias.scope !11873, !noalias !11914, !noundef !12
  %_33.i = icmp eq i64 %_34.i, %_23, !dbg !12036
  br i1 %_33.i, label %bb13.i, label %bb35.i, !dbg !12036

bb13.i:                                           ; preds = %bb12.i13
  %966 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 8, !dbg !12037
  %_36.i = load i64, ptr %966, align 8, !dbg !12037, !alias.scope !11873, !noalias !11914, !noundef !12
  %_35.i = icmp eq i64 %_36.i, %_23, !dbg !12037
  br i1 %_35.i, label %bb14.i14, label %bb35.i, !dbg !12037

bb14.i14:                                         ; preds = %bb13.i
  %967 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 20, !dbg !12038
  %_39.i = load float, ptr %967, align 4, !dbg !12038, !alias.scope !11873, !noalias !11914, !noundef !12
  %_38.i = bitcast float %_39.i to i32, !dbg !12039
  %968 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 24, !dbg !12041
  %_4139.i = load i32, ptr %968, align 8, !dbg !12041, !alias.scope !11873, !noalias !11914, !noundef !12
  %_37.i = icmp eq i32 %_4139.i, %_38.i, !dbg !12038
  br i1 %_37.i, label %bb16.i15, label %bb35.i, !dbg !12038

bb16.i15:                                         ; preds = %bb14.i14
  %_43.i = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_d0058eaee52f1ba8b2e5577cef0c3c7f, i64 %parameter_index.i, !dbg !12042
; call effect_runtime::params::parameter_value_valid
  %_42.i = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_43.i, float noundef %_39.i) #23, !dbg !12043, !noalias !11880
  %_45.i = icmp ugt i32 %_127.0.i, %last_order.sroa.3.0.ph98.i
  %or.cond17.not.not112.i = select i1 %last_order.sroa.0.0.ph97.not.i, i1 true, i1 %_45.i, !dbg !12043
  %or.cond41.not.i = select i1 %_42.i, i1 %or.cond17.not.not112.i, i1 false, !dbg !12043
  br i1 %or.cond41.not.i, label %bb29.i, label %bb35.i, !dbg !12043

bb29.i:                                           ; preds = %bb16.i15
  %_47.i = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i, !dbg !12044
  %969 = load i32, ptr %_47.i, align 4, !dbg !12045, !range !5852, !noalias !11880, !noundef !12
  %_133.not.i = icmp eq i32 %969, 0, !dbg !12052
  br i1 %_133.not.i, label %bb31.i, label %bb35.i, !dbg !12053

bb31.i:                                           ; preds = %bb29.i
  %_47.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i
  %_135.i = fcmp oeq float %_39.i, 0.000000e+00, !dbg !12055
  %_55.sroa.0.0.i = select i1 %_135.i, float 0.000000e+00, float %_39.i, !dbg !12055
  store i32 1, ptr %_47.i.le, align 4, !dbg !12058, !noalias !11880
  %970 = getelementptr inbounds nuw i8, ptr %_47.i.le, i64 4, !dbg !12058
  store float %_55.sroa.0.0.i, ptr %970, align 4, !dbg !12058, !noalias !11880
  %_6.i.i91.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !11892
  br i1 %_6.i.i91.i, label %bb36.preheader.i, label %bb4.lr.ph.i, !dbg !11897

bb35.i:                                           ; preds = %bb29.i, %bb16.i15, %bb14.i14, %bb13.i, %bb12.i13, %bb11.i, %bb59.i, %bb9.i12, %bb4.i10
  %971 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted100.i, i64 1), !dbg !12059
  store i64 %971, ptr %919, align 8, !dbg !12009, !alias.scope !11875, !noalias !11900
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !11892
  br i1 %_6.i.i.i, label %bb36.preheader.i, label %bb4.i10, !dbg !11897

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit: ; preds = %bb36.loopexit.i
  call void @llvm.lifetime.end.p0(ptr nonnull %pending.i), !dbg !12062, !noalias !11880
  %exitcond338.not = icmp eq i64 %916, 4, !dbg !12063
  br i1 %exitcond338.not, label %bb16, label %bb15, !dbg !8427
}
