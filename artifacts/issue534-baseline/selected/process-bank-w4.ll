define void @_RNvXsd_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank(ptr dead_on_unwind noalias noundef writable writeonly sret([328 x i8]) align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef align 16 dereferenceable(1840) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(112) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !8172 {
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
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !8174
  %1 = load i8, ptr %0, align 16, !dbg !8174, !range !1313, !noundef !12
  %.not = icmp eq i8 %1, 2, !dbg !8175
  br i1 %.not, label %bb13, label %bb14, !dbg !8178, !prof !180

bb14:                                             ; preds = %start
  call void @llvm.lifetime.start.p0(ptr nonnull %report.sroa.0), !dbg !8179
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %report.sroa.0, i8 0, i64 320, i1 false), !dbg !8181
  %2 = getelementptr inbounds nuw i8, ptr %block, i64 108, !dbg !8185
  %3 = load i8, ptr %2, align 4, !dbg !8185, !range !1328, !noundef !12
  %_44.not = icmp eq i8 %3, %1, !dbg !8192
  %4 = getelementptr inbounds nuw i8, ptr %block, i64 64
  %5 = load ptr, ptr %4, align 8
  %.not7 = icmp eq ptr %5, null
  %or.cond = select i1 %_44.not, i1 %.not7, i1 false, !dbg !8190
  br i1 %or.cond, label %bb4, label %bb3, !dbg !8190

bb13:                                             ; preds = %start
; call core::option::expect_failed
  tail call void @_RNvNtCs4NRVxsYgnAr_4core6option13expect_failed(ptr noalias noundef nonnull readonly captures(address, read_provenance) @alloc_376120b9c5efdf3d59386c16952a74b7, i64 noundef 31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d8ffa1b111d58613892f54938d1219d1) #24, !dbg !8195
  unreachable, !dbg !8195

bb3:                                              ; preds = %bb14
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %_0, i8 0, i64 320, i1 false), !dbg !8196
  %report.sroa.7.0._0.sroa_idx16 = getelementptr inbounds nuw i8, ptr %_0, i64 320, !dbg !8196
  store i8 %1, ptr %report.sroa.7.0._0.sroa_idx16, align 8, !dbg !8196
  br label %bb12, !dbg !8197

bb4:                                              ; preds = %bb14
  call void @llvm.lifetime.start.p0(ptr nonnull %reports), !dbg !8198
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %reports, i8 0, i64 320, i1 false), !dbg !8199
  %6 = getelementptr inbounds nuw i8, ptr %block, i64 48
  %_36.0 = load ptr, ptr %6, align 8, !nonnull !12, !align !3484, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %block, i64 56
  %_36.1 = load i64, ptr %7, align 8, !noundef !12
  %8 = getelementptr inbounds nuw i8, ptr %block, i64 32
  %_39.0 = load ptr, ptr %8, align 8, !nonnull !12, !align !4661
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
  %15 = call i64 @llvm.usub.sat.i64(i64 %_36.1, i64 1), !dbg !8200
  br label %bb15, !dbg !8200

bb16:                                             ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit
  %16 = getelementptr inbounds nuw i8, ptr %block, i64 104, !dbg !8208
  %_27 = load i32, ptr %16, align 8, !dbg !8208, !noundef !12
  %frames = zext i32 %_27 to i64, !dbg !8208
  %_37.0 = load ptr, ptr %block, align 8, !dbg !8209, !nonnull !12, !align !3484, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !8209
  %_37.1 = load i64, ptr %17, align 8, !dbg !8209, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !8211
  %_38.0 = load ptr, ptr %18, align 8, !dbg !8211, !nonnull !12, !align !3484, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !8211
  %_38.1 = load i64, ptr %19, align 8, !dbg !8211, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8212), !dbg !8215
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8216), !dbg !8215
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8218), !dbg !8215
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8220), !dbg !8215
  %_9.i = load i32, ptr %14, align 4, !dbg !8222, !alias.scope !8212, !noalias !8226, !noundef !12
  %20 = tail call i32 @llvm.umin.i32(i32 %_27, i32 %_9.i), !dbg !8228
  %..i.i = zext i32 %20 to i64, !dbg !8228
  %_10.not.i = icmp eq i32 %20, 0, !dbg !8230
  br i1 %_10.not.i, label %bb4.i, label %bb9.i, !dbg !8230

bb4.i:                                            ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKb1_EB3_.exit.i, %bb16
  %_18.i = icmp ugt i32 %_27, %_9.i, !dbg !8232
  br i1 %_18.i, label %bb20.i, label %bb7.i, !dbg !8232

bb9.i:                                            ; preds = %bb16
  %21 = shl nuw nsw i64 %..i.i, 2, !dbg !8233
  %_33.not.i = icmp samesign ugt i64 %21, %_37.1
  br i1 %_33.not.i, label %bb16.i, label %bb14.i, !dbg !8234, !prof !2561

bb16.i:                                           ; preds = %bb9.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %21, i64 noundef range(i64 0, 2305843009213693952) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bc75afababbf20974edb8b966373fd0c) #24, !dbg !8245, !noalias !8226
  unreachable, !dbg !8245

bb14.i:                                           ; preds = %bb9.i
  %_41.not.i = icmp samesign ugt i64 %21, %_38.1, !dbg !8246
  br i1 %_41.not.i, label %bb19.i, label %bb18.i, !dbg !8246, !prof !180

bb19.i:                                           ; preds = %bb14.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %21, i64 noundef range(i64 0, 2305843009213693952) %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0e50f2d670fa7dbeabdb68abcb7ac10) #24, !dbg !8252, !noalias !8226
  unreachable, !dbg !8252

bb18.i:                                           ; preds = %bb14.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8253), !dbg !8256
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8257), !dbg !8256
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8259), !dbg !8256
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1392, !dbg !8261
  %_15.i.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !8274
  %data.i.i984.i.i = getelementptr inbounds nuw i8, ptr %self, i64 832, !dbg !8276
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 896, !dbg !8281
  %_32.i.i = getelementptr inbounds nuw i8, ptr %self, i64 992, !dbg !8285
  %_58.0.i.i = load ptr, ptr %_15.i.i, align 8, !dbg !8286, !alias.scope !8287, !noalias !8288, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !8286
  %_58.1.i.i = load i64, ptr %23, align 8, !dbg !8286, !alias.scope !8287, !noalias !8288, !noundef !12
  %_45.i.i = getelementptr inbounds nuw i8, ptr %self, i64 800, !dbg !8290
  %_60.0.i.i = load ptr, ptr %data.i.i984.i.i, align 8, !dbg !8291, !alias.scope !8287, !noalias !8288, !nonnull !12, !noundef !12
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !8291
  %_60.1.i.i = load i64, ptr %24, align 8, !dbg !8291, !alias.scope !8287, !noalias !8288, !noundef !12
  %_50.i.i = getelementptr inbounds nuw i8, ptr %self, i64 864, !dbg !8292
  %_51.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1808, !dbg !8293
  %25 = getelementptr inbounds nuw i8, ptr %self, i64 1812, !dbg !8294
  %_52.i.i = load i32, ptr %25, align 4, !dbg !8294, !alias.scope !8287, !noalias !8288, !noundef !12
  %26 = getelementptr inbounds nuw i8, ptr %self, i64 1816, !dbg !8295
  %_53.i.i = load i32, ptr %26, align 8, !dbg !8295, !alias.scope !8287, !noalias !8288, !noundef !12
  %base.i.i.i = load i32, ptr %_51.i.i, align 4, !dbg !8296, !alias.scope !8287, !noalias !8306, !noundef !12
  %27 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 976
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 1360
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %33 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 944
  %35 = getelementptr inbounds nuw i8, ptr %self, i64 1648
  %36 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 1072
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 1008
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 1040
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 868
  %45 = getelementptr inbounds nuw i8, ptr %self, i64 808
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 1136
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %51 = getelementptr inbounds nuw i8, ptr %self, i64 1104
  %iter1.sroa.0.0.ptr.i135.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %52 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %53 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %54 = getelementptr inbounds nuw i8, ptr %self, i64 1168
  %iter1.sroa.0.0.ptr.i135.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1264
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 1232
  %iter1.sroa.0.0.ptr.i135.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 1328
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %60 = getelementptr inbounds nuw i8, ptr %self, i64 1296
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 1424
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %iter1.sroa.0.0.ptr.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1456
  %64 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %65 = getelementptr inbounds nuw i8, ptr %self, i64 1488
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %iter1.sroa.0.0.ptr.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1520
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 1568
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 1552
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 1536
  %iter1.sroa.0.0.ptr.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1584
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 1616
  %72 = getelementptr inbounds nuw i8, ptr %self, i64 1600
  br label %bb42.i.i.i, !dbg !8309

bb42.i.i.i:                                       ; preds = %bb18.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit163.i.i
  %iter.sroa.0.0.i1830.i.i = phi i64 [ 0, %bb18.i ], [ %73, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit163.i.i ]
  %73 = add nuw nsw i64 %iter.sroa.0.0.i1830.i.i, 1, !dbg !8318
  %span.i.i.i = shl i64 %iter.sroa.0.0.i1830.i.i, 2, !dbg !8324
  %_27.i.i.i = trunc i64 %iter.sroa.0.0.i1830.i.i to i32, !dbg !8326
  %now.i.i.i = add i32 %base.i.i.i, %_27.i.i.i, !dbg !8328
  %_30.i.i.i = and i32 %now.i.i.i, %_52.i.i, !dbg !8331
  %_29.i.i.i = zext i32 %_30.i.i.i to i64, !dbg !8333
  %write.i.i.i = shl nuw nsw i64 %_29.i.i.i, 2, !dbg !8333
  %_123.i.i.i = getelementptr inbounds nuw float, ptr %_37.0, i64 %span.i.i.i, !dbg !8334
  %_36.i.i.i = add nuw nsw i64 %write.i.i.i, 4, !dbg !8343
  %_124.not.i.i.i = icmp ugt i64 %_36.i.i.i, %_58.1.i.i, !dbg !8344
  br i1 %_124.not.i.i.i, label %bb46.i.i.i, label %bb48.i.i.i, !dbg !8344, !prof !180

bb46.i.i.i:                                       ; preds = %bb42.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i.i, i64 noundef %_36.i.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cad5a7b24766ffcdda9cbfe066eb4078) #24, !dbg !8349, !noalias !8306
  unreachable, !dbg !8349

bb48.i.i.i:                                       ; preds = %bb42.i.i.i
  %_133.i.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %write.i.i.i, !dbg !8350
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %_133.i.i.i, ptr noundef nonnull align 4 dereferenceable(16) %_123.i.i.i, i64 16, i1 false), !dbg !8354, !noalias !8359
  %_141.i.i.i = getelementptr inbounds nuw float, ptr %_38.0, i64 %span.i.i.i, !dbg !8360
  %_142.not.i.i.i = icmp ugt i64 %_36.i.i.i, %_60.1.i.i, !dbg !8367
  br i1 %_142.not.i.i.i, label %bb51.i.i.i, label %bb50.i.i.i, !dbg !8367, !prof !180

bb51.i.i.i:                                       ; preds = %bb48.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i.i, i64 noundef %_36.i.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_72e24c5578221343e8a784545a3910d2) #24, !dbg !8371, !noalias !8306
  unreachable, !dbg !8371

bb50.i.i.i:                                       ; preds = %bb48.i.i.i
  %_149.i.i.i = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %write.i.i.i, !dbg !8372
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %_149.i.i.i, ptr noundef nonnull align 4 dereferenceable(16) %_141.i.i.i, i64 16, i1 false), !dbg !8376, !noalias !8381
  %_52.i.i.i = sub i32 %now.i.i.i, %_53.i.i, !dbg !8382
  %_51.i.i.i = and i32 %_52.i.i.i, %_52.i.i, !dbg !8385
  %_50.i.i.i = zext i32 %_51.i.i.i to i64, !dbg !8386
  %read.i.i.i = shl nuw nsw i64 %_50.i.i.i, 2, !dbg !8386
  %_55.i.i.i = add nuw nsw i64 %read.i.i.i, 4, !dbg !8387
  %_182.not.i.i.i = icmp ugt i64 %_55.i.i.i, %_58.1.i.i, !dbg !8389
  br i1 %_182.not.i.i.i, label %bb61.i.i.i, label %bb60.i.i.i, !dbg !8389, !prof !180

bb61.i.i.i:                                       ; preds = %bb50.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i.i, i64 noundef %_55.i.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba99eeeb3482270ebe1c674b4013dd6a) #24, !dbg !8393, !noalias !8306
  unreachable, !dbg !8393

bb60.i.i.i:                                       ; preds = %bb50.i.i.i
  %_189.i.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %read.i.i.i, !dbg !8394
  %lanes.i582.sroa.0.0.copyload.i.i = load <4 x float>, ptr %_189.i.i.i, align 4, !dbg !8398, !alias.scope !8406, !noalias !8410
  %_190.not.i.i.i = icmp ugt i64 %_55.i.i.i, %_60.1.i.i, !dbg !8414
  br i1 %_190.not.i.i.i, label %bb64.i.i.i, label %bb63.i.i.i, !dbg !8414, !prof !180

bb64.i.i.i:                                       ; preds = %bb60.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i.i, i64 noundef %_55.i.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f4c27429e4885e1b619f1e4a3587acc) #24, !dbg !8419, !noalias !8306
  unreachable, !dbg !8419

bb63.i.i.i:                                       ; preds = %bb60.i.i.i
  %_195.i.i.i = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %read.i.i.i, !dbg !8420
  %lanes.i576.sroa.0.0.copyload.i.i = load <4 x float>, ptr %_195.i.i.i, align 4, !dbg !8424, !alias.scope !8429, !noalias !8433
  %_67.i.i.i = load i32, ptr %_45.i.i, align 4, !dbg !8437, !alias.scope !8287, !noalias !8306, !noundef !12
  %_66.i.i.i = sub i32 %now.i.i.i, %_67.i.i.i, !dbg !8443
  %_65.i.i.i = and i32 %_66.i.i.i, %_52.i.i, !dbg !8445
  %_64.i.i.i = zext i32 %_65.i.i.i to i64, !dbg !8446
  %_63.i.i.i = shl nuw nsw i64 %_64.i.i.i, 2, !dbg !8446
  %_75.i.i.i = load i32, ptr %_50.i.i, align 4, !dbg !8447, !alias.scope !8287, !noalias !8306, !noundef !12
  %_74.i.i.i = sub i32 %now.i.i.i, %_75.i.i.i, !dbg !8449
  %_73.i.i.i = and i32 %_74.i.i.i, %_52.i.i, !dbg !8451
  %_72.i.i.i = zext i32 %_73.i.i.i to i64, !dbg !8452
  %_71.i.i.i = shl nuw nsw i64 %_72.i.i.i, 2, !dbg !8452
  %_80.i.i.i = icmp ult i64 %_63.i.i.i, %_58.1.i.i, !dbg !8453
  br i1 %_80.i.i.i, label %bb22.i.i.i, label %panic18.i.i.i, !dbg !8453

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit163.i.i: ; preds = %bb26.i.3.i.i
  %74 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.3.i.i, !dbg !8455
  %_85.i.32146.i.i = load i32, ptr %74, align 4, !dbg !8455, !noalias !8306, !noundef !12
  %75 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.3.i.i, !dbg !8456
  %_87.i.32147.i.i = load i32, ptr %75, align 4, !dbg !8456, !noalias !8306, !noundef !12
  %_12.i127.sroa.0.0.copyload.i.i = load <4 x float>, ptr %49, align 16, !dbg !8457, !alias.scope !8287, !noalias !8464
  %76 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i127.sroa.0.0.copyload.i.i, i8 1), !dbg !8471
  %77 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i127.sroa.0.0.copyload.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !8496
  %_16.i123.sroa.0.0.copyload.i.i = load <4 x float>, ptr %13, align 16, !dbg !8509, !alias.scope !8287, !noalias !8464
  %_17.i122.sroa.0.0.copyload.i.i = load <4 x float>, ptr %50, align 16, !dbg !8511, !alias.scope !8287, !noalias !8464
  %78 = fadd <4 x float> %_16.i123.sroa.0.0.copyload.i.i, %_17.i122.sroa.0.0.copyload.i.i, !dbg !8512
  %_20.i119.sroa.0.0.copyload.i.i = load <4 x float>, ptr %51, align 16, !dbg !8522, !alias.scope !8287, !noalias !8464
  %79 = bitcast <4 x float> %77 to <4 x i32>, !dbg !8524
  %80 = icmp slt <4 x i32> %79, zeroinitializer, !dbg !8533
  %81 = select <4 x i1> %80, <4 x float> %_20.i119.sroa.0.0.copyload.i.i, <4 x float> %78, !dbg !8533
  %82 = bitcast <4 x float> %76 to <4 x i32>, !dbg !8539
  %83 = icmp slt <4 x i32> %82, zeroinitializer, !dbg !8543
  %84 = select <4 x i1> %83, <4 x float> %81, <4 x float> %_16.i123.sroa.0.0.copyload.i.i, !dbg !8543
  store <4 x float> %84, ptr %13, align 16, !dbg !8545, !alias.scope !8287, !noalias !8464
  %85 = select <4 x i1> %80, <4 x float> zeroinitializer, <4 x float> %_17.i122.sroa.0.0.copyload.i.i, !dbg !8546
  store <4 x float> %85, ptr %50, align 16, !dbg !8551, !alias.scope !8287, !noalias !8464
  %86 = fadd <4 x float> %_12.i127.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !8552
  %87 = select <4 x i1> %83, <4 x float> %86, <4 x float> %_12.i127.sroa.0.0.copyload.i.i, !dbg !8562
  store <4 x float> %87, ptr %49, align 16, !dbg !8567, !alias.scope !8287, !noalias !8464
  %_12.i127.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %52, align 16, !dbg !8457, !alias.scope !8287, !noalias !8464
  %88 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i127.sroa.0.0.copyload.1.i.i, i8 1), !dbg !8471
  %89 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i127.sroa.0.0.copyload.1.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !8496
  %_16.i123.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i135.1.i.i, align 16, !dbg !8509, !alias.scope !8287, !noalias !8464
  %_17.i122.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %53, align 16, !dbg !8511, !alias.scope !8287, !noalias !8464
  %90 = fadd <4 x float> %_16.i123.sroa.0.0.copyload.1.i.i, %_17.i122.sroa.0.0.copyload.1.i.i, !dbg !8512
  %_20.i119.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %54, align 16, !dbg !8522, !alias.scope !8287, !noalias !8464
  %91 = bitcast <4 x float> %89 to <4 x i32>, !dbg !8524
  %92 = icmp slt <4 x i32> %91, zeroinitializer, !dbg !8533
  %93 = select <4 x i1> %92, <4 x float> %_20.i119.sroa.0.0.copyload.1.i.i, <4 x float> %90, !dbg !8533
  %94 = bitcast <4 x float> %88 to <4 x i32>, !dbg !8539
  %95 = icmp slt <4 x i32> %94, zeroinitializer, !dbg !8543
  %96 = select <4 x i1> %95, <4 x float> %93, <4 x float> %_16.i123.sroa.0.0.copyload.1.i.i, !dbg !8543
  store <4 x float> %96, ptr %iter1.sroa.0.0.ptr.i135.1.i.i, align 16, !dbg !8545, !alias.scope !8287, !noalias !8464
  %97 = select <4 x i1> %92, <4 x float> zeroinitializer, <4 x float> %_17.i122.sroa.0.0.copyload.1.i.i, !dbg !8546
  store <4 x float> %97, ptr %53, align 16, !dbg !8551, !alias.scope !8287, !noalias !8464
  %98 = fadd <4 x float> %_12.i127.sroa.0.0.copyload.1.i.i, splat (float -1.000000e+00), !dbg !8552
  %99 = select <4 x i1> %95, <4 x float> %98, <4 x float> %_12.i127.sroa.0.0.copyload.1.i.i, !dbg !8562
  store <4 x float> %99, ptr %52, align 16, !dbg !8567, !alias.scope !8287, !noalias !8464
  %_12.i127.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %55, align 16, !dbg !8457, !alias.scope !8287, !noalias !8464
  %100 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i127.sroa.0.0.copyload.2.i.i, i8 1), !dbg !8471
  %101 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i127.sroa.0.0.copyload.2.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !8496
  %_16.i123.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i135.2.i.i, align 16, !dbg !8509, !alias.scope !8287, !noalias !8464
  %_17.i122.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %56, align 16, !dbg !8511, !alias.scope !8287, !noalias !8464
  %102 = fadd <4 x float> %_16.i123.sroa.0.0.copyload.2.i.i, %_17.i122.sroa.0.0.copyload.2.i.i, !dbg !8512
  %_20.i119.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %57, align 16, !dbg !8522, !alias.scope !8287, !noalias !8464
  %103 = bitcast <4 x float> %101 to <4 x i32>, !dbg !8524
  %104 = icmp slt <4 x i32> %103, zeroinitializer, !dbg !8533
  %105 = select <4 x i1> %104, <4 x float> %_20.i119.sroa.0.0.copyload.2.i.i, <4 x float> %102, !dbg !8533
  %106 = bitcast <4 x float> %100 to <4 x i32>, !dbg !8539
  %107 = icmp slt <4 x i32> %106, zeroinitializer, !dbg !8543
  %108 = select <4 x i1> %107, <4 x float> %105, <4 x float> %_16.i123.sroa.0.0.copyload.2.i.i, !dbg !8543
  store <4 x float> %108, ptr %iter1.sroa.0.0.ptr.i135.2.i.i, align 16, !dbg !8545, !alias.scope !8287, !noalias !8464
  %109 = select <4 x i1> %104, <4 x float> zeroinitializer, <4 x float> %_17.i122.sroa.0.0.copyload.2.i.i, !dbg !8546
  store <4 x float> %109, ptr %56, align 16, !dbg !8551, !alias.scope !8287, !noalias !8464
  %110 = fadd <4 x float> %_12.i127.sroa.0.0.copyload.2.i.i, splat (float -1.000000e+00), !dbg !8552
  %111 = select <4 x i1> %107, <4 x float> %110, <4 x float> %_12.i127.sroa.0.0.copyload.2.i.i, !dbg !8562
  store <4 x float> %111, ptr %55, align 16, !dbg !8567, !alias.scope !8287, !noalias !8464
  %_12.i127.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %58, align 16, !dbg !8457, !alias.scope !8287, !noalias !8464
  %112 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i127.sroa.0.0.copyload.3.i.i, i8 1), !dbg !8471
  %113 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i127.sroa.0.0.copyload.3.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !8496
  %_16.i123.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i135.3.i.i, align 16, !dbg !8509, !alias.scope !8287, !noalias !8464
  %_17.i122.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %59, align 16, !dbg !8511, !alias.scope !8287, !noalias !8464
  %114 = fadd <4 x float> %_16.i123.sroa.0.0.copyload.3.i.i, %_17.i122.sroa.0.0.copyload.3.i.i, !dbg !8512
  %_20.i119.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %60, align 16, !dbg !8522, !alias.scope !8287, !noalias !8464
  %115 = bitcast <4 x float> %113 to <4 x i32>, !dbg !8524
  %116 = icmp slt <4 x i32> %115, zeroinitializer, !dbg !8533
  %117 = select <4 x i1> %116, <4 x float> %_20.i119.sroa.0.0.copyload.3.i.i, <4 x float> %114, !dbg !8533
  %118 = bitcast <4 x float> %112 to <4 x i32>, !dbg !8539
  %119 = icmp slt <4 x i32> %118, zeroinitializer, !dbg !8543
  %120 = select <4 x i1> %119, <4 x float> %117, <4 x float> %_16.i123.sroa.0.0.copyload.3.i.i, !dbg !8543
  store <4 x float> %120, ptr %iter1.sroa.0.0.ptr.i135.3.i.i, align 16, !dbg !8545, !alias.scope !8287, !noalias !8464
  %121 = select <4 x i1> %116, <4 x float> zeroinitializer, <4 x float> %_17.i122.sroa.0.0.copyload.3.i.i, !dbg !8546
  store <4 x float> %121, ptr %59, align 16, !dbg !8551, !alias.scope !8287, !noalias !8464
  %122 = fadd <4 x float> %_12.i127.sroa.0.0.copyload.3.i.i, splat (float -1.000000e+00), !dbg !8552
  %123 = select <4 x i1> %119, <4 x float> %122, <4 x float> %_12.i127.sroa.0.0.copyload.3.i.i, !dbg !8562
  store <4 x float> %123, ptr %58, align 16, !dbg !8567, !alias.scope !8287, !noalias !8464
  %_41.i98.sroa.0.0.copyload.i.i = load <4 x float>, ptr %29, align 16, !dbg !8568, !alias.scope !8287, !noalias !8576
  %124 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_41.i98.sroa.0.0.copyload.i.i, i8 1), !dbg !8578
  %125 = bitcast <4 x float> %124 to <4 x i32>, !dbg !8584
  %126 = icmp slt <4 x i32> %125, zeroinitializer, !dbg !8588
  %lanes.i570.sroa.0.0.vec.insert.i.i = insertelement <4 x i32> poison, i32 %_78.i2132.i.i, i64 0, !dbg !8590
  %lanes.i570.sroa.0.4.vec.insert.i.i = insertelement <4 x i32> %lanes.i570.sroa.0.0.vec.insert.i.i, i32 %_78.i.12136.i.i, i64 1, !dbg !8590
  %lanes.i570.sroa.0.8.vec.insert.i.i = insertelement <4 x i32> %lanes.i570.sroa.0.4.vec.insert.i.i, i32 %_78.i.22140.i.i, i64 2, !dbg !8590
  %lanes.i570.sroa.0.12.vec.insert.i.i = insertelement <4 x i32> %lanes.i570.sroa.0.8.vec.insert.i.i, i32 %_78.i.32144.i.i, i64 3, !dbg !8590
  %127 = bitcast <4 x i32> %lanes.i570.sroa.0.12.vec.insert.i.i to <2 x i64>, !dbg !8595
  %128 = and <2 x i64> %127, splat (i64 9223372034707292159), !dbg !8596
  %129 = bitcast <2 x i64> %128 to <4 x float>, !dbg !8611
  %130 = fmul <4 x float> %129, splat (float 5.000000e-01), !dbg !8612
  %lanes.i564.sroa.0.0.vec.insert.i.i = insertelement <4 x i32> poison, i32 %_82.i2133.i.i, i64 0, !dbg !8622
  %lanes.i564.sroa.0.4.vec.insert.i.i = insertelement <4 x i32> %lanes.i564.sroa.0.0.vec.insert.i.i, i32 %_82.i.12137.i.i, i64 1, !dbg !8622
  %lanes.i564.sroa.0.8.vec.insert.i.i = insertelement <4 x i32> %lanes.i564.sroa.0.4.vec.insert.i.i, i32 %_82.i.22141.i.i, i64 2, !dbg !8622
  %lanes.i564.sroa.0.12.vec.insert.i.i = insertelement <4 x i32> %lanes.i564.sroa.0.8.vec.insert.i.i, i32 %_82.i.32145.i.i, i64 3, !dbg !8622
  %131 = bitcast <4 x i32> %lanes.i564.sroa.0.12.vec.insert.i.i to <2 x i64>, !dbg !8627
  %132 = and <2 x i64> %131, splat (i64 9223372034707292159), !dbg !8628
  %133 = bitcast <2 x i64> %132 to <4 x float>, !dbg !8634
  %134 = fmul <4 x float> %133, splat (float 5.000000e-01), !dbg !8635
  %135 = fadd <4 x float> %130, %134, !dbg !8640
  %_37.i102.sroa.0.0.copyload.i.i = load <4 x float>, ptr %28, align 16, !dbg !8645, !alias.scope !8287, !noalias !8576
  %136 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_37.i102.sroa.0.0.copyload.i.i, i8 1), !dbg !8646
  %137 = bitcast <4 x float> %136 to <4 x i32>, !dbg !8652
  %138 = icmp slt <4 x i32> %137, zeroinitializer, !dbg !8656
  %139 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %129, <4 x float> %133), !dbg !8658
  %140 = select <4 x i1> %138, <4 x float> %139, <4 x float> %129, !dbg !8656
  %141 = select <4 x i1> %126, <4 x float> %135, <4 x float> %140, !dbg !8588
  %142 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %141, <4 x float> splat (float 0x3E45798EE0000000)), !dbg !8667
  %143 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %142, <4 x float> splat (float 0x3810000000000000)), !dbg !8672
  %144 = bitcast <4 x float> %143 to <2 x i64>, !dbg !8681
  %145 = and <2 x i64> %144, splat (i64 36028792732385279), !dbg !8682
  %146 = or disjoint <2 x i64> %145, splat (i64 4575657222473777152), !dbg !8700
  %147 = bitcast <2 x i64> %146 to <4 x float>, !dbg !8709
  %148 = fadd <4 x float> %147, splat (float -1.000000e+00), !dbg !8710
  %149 = fmul <4 x float> %148, splat (float 0x3F9B17A960000000), !dbg !8716
  %150 = fsub <4 x float> splat (float 0x3FBF9A8440000000), %149, !dbg !8724
  %151 = fmul <4 x float> %148, %150, !dbg !8716
  %152 = fadd <4 x float> %151, splat (float 0xBFD1E3F400000000), !dbg !8724
  %153 = fmul <4 x float> %148, %152, !dbg !8716
  %154 = fadd <4 x float> %153, splat (float 0x3FDD544F20000000), !dbg !8724
  %155 = fmul <4 x float> %148, %154, !dbg !8716
  %156 = fadd <4 x float> %155, splat (float 0xBFE6FC2A60000000), !dbg !8724
  %157 = fmul <4 x float> %148, %156, !dbg !8716
  %158 = fadd <4 x float> %157, splat (float 0x3FF714B2A0000000), !dbg !8724
  %159 = bitcast <4 x float> %143 to <4 x i32>, !dbg !8729
  %_3.i988.i.i = lshr <4 x i32> %159, splat (i32 23), !dbg !8739
  %160 = bitcast <4 x i32> %_3.i988.i.i to <2 x i64>, !dbg !8740
  %161 = or disjoint <2 x i64> %160, splat (i64 5404319554102886400), !dbg !8741
  %162 = bitcast <2 x i64> %161 to <4 x float>, !dbg !8747
  %163 = fadd <4 x float> %162, splat (float 0xC160000FE0000000), !dbg !8748
  %164 = bitcast <4 x float> %108 to <2 x i64>, !dbg !8754
  %165 = fmul <4 x float> %148, %158, !dbg !8755
  %166 = fadd <4 x float> %163, %165, !dbg !8760
  %167 = fmul <4 x float> %166, splat (float 0x4018151820000000), !dbg !8765
  %168 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %167, <4 x float> splat (float 2.400000e+01)), !dbg !8770
  %169 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %168, <4 x float> splat (float -1.600000e+02)), !dbg !8779
  %_55.i84.sroa.0.0.copyload.i.i = load <4 x float>, ptr %27, align 16, !dbg !8784, !alias.scope !8287, !noalias !8464
  %170 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_55.i84.sroa.0.0.copyload.i.i, i8 1), !dbg !8786
  %171 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %84, <4 x float> %169, i8 2), !dbg !8792
  %172 = fsub <4 x float> %84, %120, !dbg !8805
  %173 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %172, <4 x float> %169, i8 2), !dbg !8811
  %174 = bitcast <4 x float> %170 to <2 x i64>, !dbg !8817
  %175 = xor <2 x i64> %174, splat (i64 -1), !dbg !8829
  %176 = bitcast <4 x float> %171 to <2 x i64>, !dbg !8834
  %177 = and <2 x i64> %176, %175, !dbg !8841
  %178 = bitcast <4 x float> %173 to <2 x i64>, !dbg !8843
  %179 = and <2 x i64> %178, %174, !dbg !8848
  %180 = or <2 x i64> %179, %177, !dbg !8850
  %181 = xor <2 x i64> %178, splat (i64 -1), !dbg !8863
  %_67.i72.sroa.0.0.copyload.i.i = load <4 x float>, ptr %30, align 16, !dbg !8870, !alias.scope !8287, !noalias !8464
  %182 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_67.i72.sroa.0.0.copyload.i.i, i8 1), !dbg !8871
  %183 = bitcast <4 x float> %182 to <2 x i64>, !dbg !8877
  %184 = and <2 x i64> %181, %183, !dbg !8881
  %185 = and <2 x i64> %184, %174, !dbg !8881
  %186 = or <2 x i64> %185, %180, !dbg !8886
  %187 = bitcast <2 x i64> %186 to <4 x i32>, !dbg !8892
  %188 = icmp slt <4 x i32> %187, zeroinitializer, !dbg !8896
  %189 = select <4 x i1> %188, <4 x float> splat (float 1.000000e+00), <4 x float> zeroinitializer, !dbg !8896
  %_71.i68.sroa.0.0.copyload.i.i = load <4 x float>, ptr %31, align 16, !dbg !8898, !alias.scope !8287, !noalias !8576
  %190 = fadd <4 x float> %_67.i72.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !8900
  %191 = bitcast <2 x i64> %185 to <4 x i32>, !dbg !8905
  %192 = icmp slt <4 x i32> %191, zeroinitializer, !dbg !8909
  %193 = select <4 x i1> %192, <4 x float> %190, <4 x float> %_67.i72.sroa.0.0.copyload.i.i, !dbg !8909
  %194 = bitcast <2 x i64> %180 to <4 x i32>, !dbg !8911
  %195 = icmp slt <4 x i32> %194, zeroinitializer, !dbg !8915
  %196 = select <4 x i1> %195, <4 x float> %_71.i68.sroa.0.0.copyload.i.i, <4 x float> %193, !dbg !8915
  store <4 x float> %196, ptr %30, align 16, !dbg !8917, !alias.scope !8287, !noalias !8464
  store <4 x float> %189, ptr %27, align 16, !dbg !8918, !alias.scope !8287, !noalias !8464
  %_86.i53.sroa.0.0.copyload.i.i = load <4 x float>, ptr %32, align 16, !dbg !8919, !alias.scope !8287, !noalias !8464
  %_88.i52.sroa.0.0.copyload.i.i = load <4 x float>, ptr %33, align 16, !dbg !8922, !alias.scope !8287, !noalias !8576
  %_7.i850.i.i = load <4 x float>, ptr %22, align 16, !dbg !8923, !alias.scope !8925, !noalias !8928
  %197 = fadd <4 x float> %96, splat (float -1.000000e+00), !dbg !8932
  %198 = fsub <4 x float> %169, %84, !dbg !8937
  %199 = fmul <4 x float> %197, %198, !dbg !8942
  %200 = xor <2 x i64> %164, splat (i64 -9223372034707292160), !dbg !8947
  %201 = bitcast <2 x i64> %200 to <4 x float>, !dbg !8956
  %202 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %199, <4 x float> %201), !dbg !8957
  %203 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %202, <4 x float> zeroinitializer), !dbg !8962
  %204 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %189, i8 1), !dbg !8967
  %205 = bitcast <4 x float> %204 to <4 x i32>, !dbg !8973
  %206 = icmp slt <4 x i32> %205, zeroinitializer, !dbg !8977
  %207 = select <4 x i1> %206, <4 x float> zeroinitializer, <4 x float> %203, !dbg !8977
  %208 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_86.i53.sroa.0.0.copyload.i.i, <4 x float> %207, i8 1), !dbg !8979
  %209 = bitcast <4 x float> %208 to <4 x i32>, !dbg !8985
  %210 = icmp slt <4 x i32> %209, zeroinitializer, !dbg !8988
  %211 = select <4 x i1> %210, <4 x float> %_7.i850.i.i, <4 x float> %_88.i52.sroa.0.0.copyload.i.i, !dbg !8988
  %212 = fsub <4 x float> %207, %_86.i53.sroa.0.0.copyload.i.i, !dbg !8990
  %213 = fmul <4 x float> %212, %211, !dbg !8996
  %214 = fadd <4 x float> %_86.i53.sroa.0.0.copyload.i.i, %213, !dbg !9004
  %215 = bitcast <4 x float> %214 to <2 x i64>, !dbg !9011
  %216 = and <2 x i64> %215, splat (i64 9223372034707292159), !dbg !9018
  %217 = bitcast <2 x i64> %216 to <4 x float>, !dbg !9011
  %218 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %217, <4 x float> splat (float 0x3BC79CA100000000), i8 1), !dbg !9020
  %219 = bitcast <4 x float> %218 to <2 x i64>, !dbg !9032
  %220 = xor <2 x i64> %219, splat (i64 -1), !dbg !9041
  %221 = and <2 x i64> %215, %220, !dbg !9043
  store <2 x i64> %221, ptr %32, align 16, !dbg !9049, !alias.scope !8287, !noalias !8464
  %222 = bitcast <2 x i64> %221 to <4 x float>, !dbg !9051
  %223 = fmul <4 x float> %222, splat (float 0x3FC542A5A0000000), !dbg !9052
  %224 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %223, <4 x float> splat (float -1.260000e+02)), !dbg !9059
  %225 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %224, <4 x float> splat (float 1.270000e+02)), !dbg !9066
  %226 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %225), !dbg !9071
  %227 = fsub <4 x float> %225, %226, !dbg !9081
  %228 = fmul <4 x float> %227, splat (float 0x3F5E974FA0000000), !dbg !9087
  %229 = fadd <4 x float> %228, splat (float 0x3F82778560000000), !dbg !9095
  %230 = fmul <4 x float> %227, %229, !dbg !9087
  %231 = fadd <4 x float> %230, splat (float 0x3FAC91CE60000000), !dbg !9095
  %232 = fmul <4 x float> %227, %231, !dbg !9087
  %233 = fadd <4 x float> %232, splat (float 0x3FCEBDB560000000), !dbg !9095
  %234 = fmul <4 x float> %227, %233, !dbg !9087
  %235 = fadd <4 x float> %234, splat (float 0x3FE62E4BA0000000), !dbg !9095
  %_98.i42.sroa.0.0.copyload.i.i = load <4 x float>, ptr %34, align 16, !dbg !9100, !alias.scope !8287, !noalias !8576
  %_12.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %61, align 16, !dbg !9102, !alias.scope !8287, !noalias !9105
  %236 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9112
  %237 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i.sroa.0.0.copyload.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !9118
  %_16.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %data.i.i.i.i, align 16, !dbg !9124, !alias.scope !8287, !noalias !9105
  %_17.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %62, align 16, !dbg !9125, !alias.scope !8287, !noalias !9105
  %238 = fadd <4 x float> %_16.i.sroa.0.0.copyload.i.i, %_17.i.sroa.0.0.copyload.i.i, !dbg !9126
  %_20.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %63, align 16, !dbg !9131, !alias.scope !8287, !noalias !9105
  %239 = bitcast <4 x float> %237 to <4 x i32>, !dbg !9132
  %240 = icmp slt <4 x i32> %239, zeroinitializer, !dbg !9136
  %241 = select <4 x i1> %240, <4 x float> %_20.i.sroa.0.0.copyload.i.i, <4 x float> %238, !dbg !9136
  %242 = bitcast <4 x float> %236 to <4 x i32>, !dbg !9138
  %243 = icmp slt <4 x i32> %242, zeroinitializer, !dbg !9142
  %244 = select <4 x i1> %243, <4 x float> %241, <4 x float> %_16.i.sroa.0.0.copyload.i.i, !dbg !9142
  store <4 x float> %244, ptr %data.i.i.i.i, align 16, !dbg !9144, !alias.scope !8287, !noalias !9105
  %245 = select <4 x i1> %240, <4 x float> zeroinitializer, <4 x float> %_17.i.sroa.0.0.copyload.i.i, !dbg !9145
  store <4 x float> %245, ptr %62, align 16, !dbg !9150, !alias.scope !8287, !noalias !9105
  %246 = fadd <4 x float> %_12.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !9151
  %247 = select <4 x i1> %243, <4 x float> %246, <4 x float> %_12.i.sroa.0.0.copyload.i.i, !dbg !9156
  store <4 x float> %247, ptr %61, align 16, !dbg !9161, !alias.scope !8287, !noalias !9105
  %_12.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %64, align 16, !dbg !9102, !alias.scope !8287, !noalias !9105
  %248 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i.sroa.0.0.copyload.1.i.i, i8 1), !dbg !9112
  %249 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i.sroa.0.0.copyload.1.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !9118
  %_16.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i.1.i.i, align 16, !dbg !9124, !alias.scope !8287, !noalias !9105
  %_17.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %65, align 16, !dbg !9125, !alias.scope !8287, !noalias !9105
  %250 = fadd <4 x float> %_16.i.sroa.0.0.copyload.1.i.i, %_17.i.sroa.0.0.copyload.1.i.i, !dbg !9126
  %_20.i.sroa.0.0.copyload.1.i.i = load <4 x float>, ptr %66, align 16, !dbg !9131, !alias.scope !8287, !noalias !9105
  %251 = bitcast <4 x float> %249 to <4 x i32>, !dbg !9132
  %252 = icmp slt <4 x i32> %251, zeroinitializer, !dbg !9136
  %253 = select <4 x i1> %252, <4 x float> %_20.i.sroa.0.0.copyload.1.i.i, <4 x float> %250, !dbg !9136
  %254 = bitcast <4 x float> %248 to <4 x i32>, !dbg !9138
  %255 = icmp slt <4 x i32> %254, zeroinitializer, !dbg !9142
  %256 = select <4 x i1> %255, <4 x float> %253, <4 x float> %_16.i.sroa.0.0.copyload.1.i.i, !dbg !9142
  store <4 x float> %256, ptr %iter1.sroa.0.0.ptr.i.1.i.i, align 16, !dbg !9144, !alias.scope !8287, !noalias !9105
  %257 = select <4 x i1> %252, <4 x float> zeroinitializer, <4 x float> %_17.i.sroa.0.0.copyload.1.i.i, !dbg !9145
  store <4 x float> %257, ptr %65, align 16, !dbg !9150, !alias.scope !8287, !noalias !9105
  %258 = fadd <4 x float> %_12.i.sroa.0.0.copyload.1.i.i, splat (float -1.000000e+00), !dbg !9151
  %259 = select <4 x i1> %255, <4 x float> %258, <4 x float> %_12.i.sroa.0.0.copyload.1.i.i, !dbg !9156
  store <4 x float> %259, ptr %64, align 16, !dbg !9161, !alias.scope !8287, !noalias !9105
  %_12.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %67, align 16, !dbg !9102, !alias.scope !8287, !noalias !9105
  %260 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i.sroa.0.0.copyload.2.i.i, i8 1), !dbg !9112
  %261 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i.sroa.0.0.copyload.2.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !9118
  %_16.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i.2.i.i, align 16, !dbg !9124, !alias.scope !8287, !noalias !9105
  %_17.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %68, align 16, !dbg !9125, !alias.scope !8287, !noalias !9105
  %262 = fadd <4 x float> %_16.i.sroa.0.0.copyload.2.i.i, %_17.i.sroa.0.0.copyload.2.i.i, !dbg !9126
  %_20.i.sroa.0.0.copyload.2.i.i = load <4 x float>, ptr %69, align 16, !dbg !9131, !alias.scope !8287, !noalias !9105
  %263 = bitcast <4 x float> %261 to <4 x i32>, !dbg !9132
  %264 = icmp slt <4 x i32> %263, zeroinitializer, !dbg !9136
  %265 = select <4 x i1> %264, <4 x float> %_20.i.sroa.0.0.copyload.2.i.i, <4 x float> %262, !dbg !9136
  %266 = bitcast <4 x float> %260 to <4 x i32>, !dbg !9138
  %267 = icmp slt <4 x i32> %266, zeroinitializer, !dbg !9142
  %268 = select <4 x i1> %267, <4 x float> %265, <4 x float> %_16.i.sroa.0.0.copyload.2.i.i, !dbg !9142
  store <4 x float> %268, ptr %iter1.sroa.0.0.ptr.i.2.i.i, align 16, !dbg !9144, !alias.scope !8287, !noalias !9105
  %269 = select <4 x i1> %264, <4 x float> zeroinitializer, <4 x float> %_17.i.sroa.0.0.copyload.2.i.i, !dbg !9145
  store <4 x float> %269, ptr %68, align 16, !dbg !9150, !alias.scope !8287, !noalias !9105
  %270 = fadd <4 x float> %_12.i.sroa.0.0.copyload.2.i.i, splat (float -1.000000e+00), !dbg !9151
  %271 = select <4 x i1> %267, <4 x float> %270, <4 x float> %_12.i.sroa.0.0.copyload.2.i.i, !dbg !9156
  store <4 x float> %271, ptr %67, align 16, !dbg !9161, !alias.scope !8287, !noalias !9105
  %_12.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %70, align 16, !dbg !9102, !alias.scope !8287, !noalias !9105
  %272 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_12.i.sroa.0.0.copyload.3.i.i, i8 1), !dbg !9112
  %273 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_12.i.sroa.0.0.copyload.3.i.i, <4 x float> splat (float 1.000000e+00), i8 0), !dbg !9118
  %_16.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %iter1.sroa.0.0.ptr.i.3.i.i, align 16, !dbg !9124, !alias.scope !8287, !noalias !9105
  %_17.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %71, align 16, !dbg !9125, !alias.scope !8287, !noalias !9105
  %274 = fadd <4 x float> %_16.i.sroa.0.0.copyload.3.i.i, %_17.i.sroa.0.0.copyload.3.i.i, !dbg !9126
  %_20.i.sroa.0.0.copyload.3.i.i = load <4 x float>, ptr %72, align 16, !dbg !9131, !alias.scope !8287, !noalias !9105
  %275 = bitcast <4 x float> %273 to <4 x i32>, !dbg !9132
  %276 = icmp slt <4 x i32> %275, zeroinitializer, !dbg !9136
  %277 = select <4 x i1> %276, <4 x float> %_20.i.sroa.0.0.copyload.3.i.i, <4 x float> %274, !dbg !9136
  %278 = bitcast <4 x float> %272 to <4 x i32>, !dbg !9138
  %279 = icmp slt <4 x i32> %278, zeroinitializer, !dbg !9142
  %280 = select <4 x i1> %279, <4 x float> %277, <4 x float> %_16.i.sroa.0.0.copyload.3.i.i, !dbg !9142
  store <4 x float> %280, ptr %iter1.sroa.0.0.ptr.i.3.i.i, align 16, !dbg !9144, !alias.scope !8287, !noalias !9105
  %281 = select <4 x i1> %276, <4 x float> zeroinitializer, <4 x float> %_17.i.sroa.0.0.copyload.3.i.i, !dbg !9145
  store <4 x float> %281, ptr %71, align 16, !dbg !9150, !alias.scope !8287, !noalias !9105
  %282 = fadd <4 x float> %_12.i.sroa.0.0.copyload.3.i.i, splat (float -1.000000e+00), !dbg !9151
  %283 = select <4 x i1> %279, <4 x float> %282, <4 x float> %_12.i.sroa.0.0.copyload.3.i.i, !dbg !9156
  store <4 x float> %283, ptr %70, align 16, !dbg !9161, !alias.scope !8287, !noalias !9105
  %284 = fmul <4 x float> %227, %235, !dbg !9162
  %285 = fadd <4 x float> %284, splat (float 1.000000e+00), !dbg !9167
  %286 = fadd <4 x float> %226, splat (float 0x4160000FE0000000), !dbg !9172
  %287 = bitcast <4 x float> %286 to <4 x i32>, !dbg !9181
  %_3.i989.i.i = shl <4 x i32> %287, splat (i32 23), !dbg !9191
  %288 = bitcast <4 x i32> %_3.i989.i.i to <4 x float>, !dbg !9192
  %289 = fmul <4 x float> %285, %288, !dbg !9194
  %290 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %222, <4 x float> zeroinitializer, i8 0), !dbg !9198
  %291 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_98.i42.sroa.0.0.copyload.i.i, i8 1), !dbg !9204
  %292 = bitcast <4 x float> %290 to <2 x i64>, !dbg !9210
  %293 = bitcast <4 x float> %291 to <2 x i64>, !dbg !9210
  %294 = or <2 x i64> %293, %292, !dbg !9214
  %295 = fmul <4 x float> %lanes.i582.sroa.0.0.copyload.i.i, %289, !dbg !9216
  %296 = bitcast <2 x i64> %294 to <4 x i32>, !dbg !9222
  %297 = icmp slt <4 x i32> %296, zeroinitializer, !dbg !9226
  %298 = select <4 x i1> %297, <4 x float> %lanes.i582.sroa.0.0.copyload.i.i, <4 x float> %295, !dbg !9226
  %_41.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %37, align 16, !dbg !9228, !alias.scope !8287, !noalias !9229
  %299 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_41.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9231
  %300 = bitcast <4 x float> %299 to <4 x i32>, !dbg !9237
  %301 = icmp slt <4 x i32> %300, zeroinitializer, !dbg !9241
  %lanes.i558.sroa.0.0.vec.insert.i.i = insertelement <4 x i32> poison, i32 %_85.i2134.i.i, i64 0, !dbg !9243
  %lanes.i558.sroa.0.4.vec.insert.i.i = insertelement <4 x i32> %lanes.i558.sroa.0.0.vec.insert.i.i, i32 %_85.i.12138.i.i, i64 1, !dbg !9243
  %lanes.i558.sroa.0.8.vec.insert.i.i = insertelement <4 x i32> %lanes.i558.sroa.0.4.vec.insert.i.i, i32 %_85.i.22142.i.i, i64 2, !dbg !9243
  %lanes.i558.sroa.0.12.vec.insert.i.i = insertelement <4 x i32> %lanes.i558.sroa.0.8.vec.insert.i.i, i32 %_85.i.32146.i.i, i64 3, !dbg !9243
  %302 = bitcast <4 x i32> %lanes.i558.sroa.0.12.vec.insert.i.i to <2 x i64>, !dbg !9248
  %303 = and <2 x i64> %302, splat (i64 9223372034707292159), !dbg !9249
  %304 = bitcast <2 x i64> %303 to <4 x float>, !dbg !9255
  %305 = fmul <4 x float> %304, splat (float 5.000000e-01), !dbg !9256
  %lanes.i.sroa.0.0.vec.insert.i.i = insertelement <4 x i32> poison, i32 %_87.i2135.i.i, i64 0, !dbg !9261
  %lanes.i.sroa.0.4.vec.insert.i.i = insertelement <4 x i32> %lanes.i.sroa.0.0.vec.insert.i.i, i32 %_87.i.12139.i.i, i64 1, !dbg !9261
  %lanes.i.sroa.0.8.vec.insert.i.i = insertelement <4 x i32> %lanes.i.sroa.0.4.vec.insert.i.i, i32 %_87.i.22143.i.i, i64 2, !dbg !9261
  %lanes.i.sroa.0.12.vec.insert.i.i = insertelement <4 x i32> %lanes.i.sroa.0.8.vec.insert.i.i, i32 %_87.i.32147.i.i, i64 3, !dbg !9261
  %306 = bitcast <4 x i32> %lanes.i.sroa.0.12.vec.insert.i.i to <2 x i64>, !dbg !9266
  %307 = and <2 x i64> %306, splat (i64 9223372034707292159), !dbg !9267
  %308 = bitcast <2 x i64> %307 to <4 x float>, !dbg !9273
  %309 = fmul <4 x float> %308, splat (float 5.000000e-01), !dbg !9274
  %310 = fadd <4 x float> %305, %309, !dbg !9279
  %_37.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %36, align 16, !dbg !9284, !alias.scope !8287, !noalias !9229
  %311 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_37.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9285
  %312 = bitcast <4 x float> %311 to <4 x i32>, !dbg !9291
  %313 = icmp slt <4 x i32> %312, zeroinitializer, !dbg !9295
  %314 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %304, <4 x float> %308), !dbg !9297
  %315 = select <4 x i1> %313, <4 x float> %314, <4 x float> %304, !dbg !9295
  %316 = select <4 x i1> %301, <4 x float> %310, <4 x float> %315, !dbg !9241
  %317 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %316, <4 x float> splat (float 0x3E45798EE0000000)), !dbg !9302
  %318 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %317, <4 x float> splat (float 0x3810000000000000)), !dbg !9307
  %319 = bitcast <4 x float> %318 to <2 x i64>, !dbg !9314
  %320 = and <2 x i64> %319, splat (i64 36028792732385279), !dbg !9315
  %321 = or disjoint <2 x i64> %320, splat (i64 4575657222473777152), !dbg !9320
  %322 = bitcast <2 x i64> %321 to <4 x float>, !dbg !9324
  %323 = fadd <4 x float> %322, splat (float -1.000000e+00), !dbg !9325
  %324 = fmul <4 x float> %323, splat (float 0x3F9B17A960000000), !dbg !9330
  %325 = fsub <4 x float> splat (float 0x3FBF9A8440000000), %324, !dbg !9335
  %326 = fmul <4 x float> %323, %325, !dbg !9330
  %327 = fadd <4 x float> %326, splat (float 0xBFD1E3F400000000), !dbg !9335
  %328 = fmul <4 x float> %323, %327, !dbg !9330
  %329 = fadd <4 x float> %328, splat (float 0x3FDD544F20000000), !dbg !9335
  %330 = fmul <4 x float> %323, %329, !dbg !9330
  %331 = fadd <4 x float> %330, splat (float 0xBFE6FC2A60000000), !dbg !9335
  %332 = fmul <4 x float> %323, %331, !dbg !9330
  %333 = fadd <4 x float> %332, splat (float 0x3FF714B2A0000000), !dbg !9335
  %334 = bitcast <4 x float> %318 to <4 x i32>, !dbg !9340
  %_3.i990.i.i = lshr <4 x i32> %334, splat (i32 23), !dbg !9344
  %335 = bitcast <4 x i32> %_3.i990.i.i to <2 x i64>, !dbg !9345
  %336 = or disjoint <2 x i64> %335, splat (i64 5404319554102886400), !dbg !9346
  %337 = bitcast <2 x i64> %336 to <4 x float>, !dbg !9350
  %338 = fadd <4 x float> %337, splat (float 0xC160000FE0000000), !dbg !9351
  %339 = bitcast <4 x float> %268 to <2 x i64>, !dbg !9355
  %340 = fmul <4 x float> %323, %333, !dbg !9356
  %341 = fadd <4 x float> %338, %340, !dbg !9361
  %342 = fmul <4 x float> %341, splat (float 0x4018151820000000), !dbg !9366
  %343 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %342, <4 x float> splat (float 2.400000e+01)), !dbg !9371
  %344 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %343, <4 x float> splat (float -1.600000e+02)), !dbg !9376
  %_55.i26.sroa.0.0.copyload.i.i = load <4 x float>, ptr %35, align 16, !dbg !9381, !alias.scope !8287, !noalias !9105
  %345 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_55.i26.sroa.0.0.copyload.i.i, i8 1), !dbg !9382
  %346 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %244, <4 x float> %344, i8 2), !dbg !9388
  %347 = fsub <4 x float> %244, %280, !dbg !9394
  %348 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %347, <4 x float> %344, i8 2), !dbg !9399
  %349 = bitcast <4 x float> %345 to <2 x i64>, !dbg !9405
  %350 = xor <2 x i64> %349, splat (i64 -1), !dbg !9410
  %351 = bitcast <4 x float> %346 to <2 x i64>, !dbg !9412
  %352 = and <2 x i64> %351, %350, !dbg !9416
  %353 = bitcast <4 x float> %348 to <2 x i64>, !dbg !9418
  %354 = and <2 x i64> %353, %349, !dbg !9422
  %355 = or <2 x i64> %354, %352, !dbg !9424
  %356 = xor <2 x i64> %353, splat (i64 -1), !dbg !9429
  %_67.i22.sroa.0.0.copyload.i.i = load <4 x float>, ptr %38, align 16, !dbg !9435, !alias.scope !8287, !noalias !9105
  %357 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_67.i22.sroa.0.0.copyload.i.i, i8 1), !dbg !9436
  %358 = bitcast <4 x float> %357 to <2 x i64>, !dbg !9442
  %359 = and <2 x i64> %356, %358, !dbg !9446
  %360 = and <2 x i64> %359, %349, !dbg !9446
  %361 = or <2 x i64> %360, %355, !dbg !9451
  %362 = bitcast <2 x i64> %361 to <4 x i32>, !dbg !9456
  %363 = icmp slt <4 x i32> %362, zeroinitializer, !dbg !9460
  %364 = select <4 x i1> %363, <4 x float> splat (float 1.000000e+00), <4 x float> zeroinitializer, !dbg !9460
  %_71.i20.sroa.0.0.copyload.i.i = load <4 x float>, ptr %39, align 16, !dbg !9462, !alias.scope !8287, !noalias !9229
  %365 = fadd <4 x float> %_67.i22.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !9463
  %366 = bitcast <2 x i64> %360 to <4 x i32>, !dbg !9468
  %367 = icmp slt <4 x i32> %366, zeroinitializer, !dbg !9472
  %368 = select <4 x i1> %367, <4 x float> %365, <4 x float> %_67.i22.sroa.0.0.copyload.i.i, !dbg !9472
  %369 = bitcast <2 x i64> %355 to <4 x i32>, !dbg !9474
  %370 = icmp slt <4 x i32> %369, zeroinitializer, !dbg !9478
  %371 = select <4 x i1> %370, <4 x float> %_71.i20.sroa.0.0.copyload.i.i, <4 x float> %368, !dbg !9478
  store <4 x float> %371, ptr %38, align 16, !dbg !9480, !alias.scope !8287, !noalias !9105
  store <4 x float> %364, ptr %35, align 16, !dbg !9481, !alias.scope !8287, !noalias !9105
  %_86.i11.sroa.0.0.copyload.i.i = load <4 x float>, ptr %40, align 16, !dbg !9482, !alias.scope !8287, !noalias !9105
  %_88.i10.sroa.0.0.copyload.i.i = load <4 x float>, ptr %41, align 16, !dbg !9483, !alias.scope !8287, !noalias !9229
  %_7.i898.i.i = load <4 x float>, ptr %_32.i.i, align 16, !dbg !9484, !alias.scope !9486, !noalias !9489
  %372 = fadd <4 x float> %256, splat (float -1.000000e+00), !dbg !9493
  %373 = fsub <4 x float> %344, %244, !dbg !9498
  %374 = fmul <4 x float> %372, %373, !dbg !9503
  %375 = xor <2 x i64> %339, splat (i64 -9223372034707292160), !dbg !9508
  %376 = bitcast <2 x i64> %375 to <4 x float>, !dbg !9513
  %377 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %374, <4 x float> %376), !dbg !9514
  %378 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %377, <4 x float> zeroinitializer), !dbg !9519
  %379 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %364, i8 1), !dbg !9524
  %380 = bitcast <4 x float> %379 to <4 x i32>, !dbg !9530
  %381 = icmp slt <4 x i32> %380, zeroinitializer, !dbg !9534
  %382 = select <4 x i1> %381, <4 x float> zeroinitializer, <4 x float> %378, !dbg !9534
  %383 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_86.i11.sroa.0.0.copyload.i.i, <4 x float> %382, i8 1), !dbg !9536
  %384 = bitcast <4 x float> %383 to <4 x i32>, !dbg !9542
  %385 = icmp slt <4 x i32> %384, zeroinitializer, !dbg !9545
  %386 = select <4 x i1> %385, <4 x float> %_7.i898.i.i, <4 x float> %_88.i10.sroa.0.0.copyload.i.i, !dbg !9545
  %387 = fsub <4 x float> %382, %_86.i11.sroa.0.0.copyload.i.i, !dbg !9547
  %388 = fmul <4 x float> %387, %386, !dbg !9552
  %389 = fadd <4 x float> %_86.i11.sroa.0.0.copyload.i.i, %388, !dbg !9557
  %390 = bitcast <4 x float> %389 to <2 x i64>, !dbg !9561
  %391 = and <2 x i64> %390, splat (i64 9223372034707292159), !dbg !9567
  %392 = bitcast <2 x i64> %391 to <4 x float>, !dbg !9561
  %393 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %392, <4 x float> splat (float 0x3BC79CA100000000), i8 1), !dbg !9569
  %394 = bitcast <4 x float> %393 to <2 x i64>, !dbg !9575
  %395 = xor <2 x i64> %394, splat (i64 -1), !dbg !9580
  %396 = and <2 x i64> %390, %395, !dbg !9582
  store <2 x i64> %396, ptr %40, align 16, !dbg !9586, !alias.scope !8287, !noalias !9105
  %397 = bitcast <2 x i64> %396 to <4 x float>, !dbg !9587
  %398 = fmul <4 x float> %397, splat (float 0x3FC542A5A0000000), !dbg !9588
  %399 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %398, <4 x float> splat (float -1.260000e+02)), !dbg !9594
  %400 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %399, <4 x float> splat (float 1.270000e+02)), !dbg !9600
  %401 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %400), !dbg !9605
  %402 = fsub <4 x float> %400, %401, !dbg !9610
  %403 = fmul <4 x float> %402, splat (float 0x3F5E974FA0000000), !dbg !9615
  %404 = fadd <4 x float> %403, splat (float 0x3F82778560000000), !dbg !9620
  %405 = fmul <4 x float> %402, %404, !dbg !9615
  %406 = fadd <4 x float> %405, splat (float 0x3FAC91CE60000000), !dbg !9620
  %407 = fmul <4 x float> %402, %406, !dbg !9615
  %408 = fadd <4 x float> %407, splat (float 0x3FCEBDB560000000), !dbg !9620
  %409 = fmul <4 x float> %402, %408, !dbg !9615
  %410 = fadd <4 x float> %409, splat (float 0x3FE62E4BA0000000), !dbg !9620
  %411 = fmul <4 x float> %402, %410, !dbg !9625
  %412 = fadd <4 x float> %411, splat (float 1.000000e+00), !dbg !9630
  %413 = fadd <4 x float> %401, splat (float 0x4160000FE0000000), !dbg !9635
  %414 = bitcast <4 x float> %413 to <4 x i32>, !dbg !9640
  %_3.i991.i.i = shl <4 x i32> %414, splat (i32 23), !dbg !9644
  %415 = bitcast <4 x i32> %_3.i991.i.i to <4 x float>, !dbg !9645
  %416 = fmul <4 x float> %412, %415, !dbg !9647
  %417 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %397, <4 x float> zeroinitializer, i8 0), !dbg !9651
  %_98.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %42, align 16, !dbg !9657, !alias.scope !8287, !noalias !9229
  %418 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_98.i.sroa.0.0.copyload.i.i, i8 1), !dbg !9658
  %419 = bitcast <4 x float> %417 to <2 x i64>, !dbg !9664
  %420 = bitcast <4 x float> %418 to <2 x i64>, !dbg !9664
  %421 = or <2 x i64> %420, %419, !dbg !9668
  %422 = fmul <4 x float> %lanes.i576.sroa.0.0.copyload.i.i, %416, !dbg !9670
  %423 = bitcast <2 x i64> %421 to <4 x i32>, !dbg !9675
  %424 = icmp slt <4 x i32> %423, zeroinitializer, !dbg !9679
  %425 = select <4 x i1> %424, <4 x float> %lanes.i576.sroa.0.0.copyload.i.i, <4 x float> %422, !dbg !9679
  store <4 x float> %298, ptr %_123.i.i.i, align 4, !dbg !9681, !alias.scope !9687, !noalias !9691
  store <4 x float> %425, ptr %_141.i.i.i, align 4, !dbg !9695, !alias.scope !9700, !noalias !9704
  %exitcond1926.not.i.i = icmp eq i64 %73, %..i.i, !dbg !9708
  br i1 %exitcond1926.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKb1_EB3_.exit.i, label %bb42.i.i.i, !dbg !8309

panic18.i.i.i:                                    ; preds = %bb28.i.2.i.i, %bb28.i.1.i.i, %bb28.i.i.i, %bb63.i.i.i
  %own.i.lcssa.i.i = phi i64 [ %_63.i.i.i, %bb63.i.i.i ], [ %own.i.1.i.i, %bb28.i.i.i ], [ %own.i.2.i.i, %bb28.i.1.i.i ], [ %own.i.3.i.i, %bb28.i.2.i.i ], !dbg !8446
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %own.i.lcssa.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b53d553f9945f7702333f7af20204159) #24, !dbg !8453, !noalias !8306
  unreachable, !dbg !8453

bb22.i.i.i:                                       ; preds = %bb63.i.i.i
  %426 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_63.i.i.i, !dbg !8453
  %_78.i2132.i.i = load i32, ptr %426, align 4, !dbg !8453, !noalias !8306, !noundef !12
  %_84.i.i.i = icmp ult i64 %_63.i.i.i, %_60.1.i.i, !dbg !9711
  br i1 %_84.i.i.i, label %bb24.i.i.i, label %panic20.i.i.i, !dbg !9711

panic20.i.i.i:                                    ; preds = %bb22.i.3.i.i, %bb22.i.2.i.i, %bb22.i.1.i.i, %bb22.i.i.i
  %own.i.lcssa1835.i.i = phi i64 [ %_63.i.i.i, %bb22.i.i.i ], [ %own.i.1.i.i, %bb22.i.1.i.i ], [ %own.i.2.i.i, %bb22.i.2.i.i ], [ %own.i.3.i.i, %bb22.i.3.i.i ], !dbg !8446
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %own.i.lcssa1835.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30bf8a301493a607abcc78dbf93977e0) #24, !dbg !9711, !noalias !8306
  unreachable, !dbg !9711

bb24.i.i.i:                                       ; preds = %bb22.i.i.i
  %427 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_63.i.i.i, !dbg !9711
  %_82.i2133.i.i = load i32, ptr %427, align 4, !dbg !9711, !noalias !8306, !noundef !12
  %_86.i.i.i = icmp ult i64 %_71.i.i.i, %_60.1.i.i, !dbg !8455
  br i1 %_86.i.i.i, label %bb26.i.i.i, label %panic22.i.i.i, !dbg !8455

panic22.i.i.i:                                    ; preds = %bb24.i.3.i.i, %bb24.i.2.i.i, %bb24.i.1.i.i, %bb24.i.i.i
  %partner.i.lcssa1832.i.i = phi i64 [ %_71.i.i.i, %bb24.i.i.i ], [ %partner.i.1.i.i, %bb24.i.1.i.i ], [ %partner.i.2.i.i, %bb24.i.2.i.i ], [ %partner.i.3.i.i, %bb24.i.3.i.i ], !dbg !8452
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %partner.i.lcssa1832.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5833dd3f10f38ea96ec24c6054ff083c) #24, !dbg !8455, !noalias !8306
  unreachable, !dbg !8455

bb26.i.i.i:                                       ; preds = %bb24.i.i.i
  %428 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_71.i.i.i, !dbg !8455
  %_85.i2134.i.i = load i32, ptr %428, align 4, !dbg !8455, !noalias !8306, !noundef !12
  %_88.i.i.i = icmp ult i64 %_71.i.i.i, %_58.1.i.i, !dbg !8456
  br i1 %_88.i.i.i, label %bb28.i.i.i, label %panic24.i.i.i, !dbg !8456

panic24.i.i.i:                                    ; preds = %bb26.i.3.i.i, %bb26.i.2.i.i, %bb26.i.1.i.i, %bb26.i.i.i
  %partner.i.lcssa1833.i.i = phi i64 [ %_71.i.i.i, %bb26.i.i.i ], [ %partner.i.1.i.i, %bb26.i.1.i.i ], [ %partner.i.2.i.i, %bb26.i.2.i.i ], [ %partner.i.3.i.i, %bb26.i.3.i.i ], !dbg !8452
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %partner.i.lcssa1833.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5ae5decb60097d7e1183cb43cae6b3f) #24, !dbg !8456, !noalias !8306
  unreachable, !dbg !8456

bb28.i.i.i:                                       ; preds = %bb26.i.i.i
  %429 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_71.i.i.i, !dbg !8456
  %_87.i2135.i.i = load i32, ptr %429, align 4, !dbg !8456, !noalias !8306, !noundef !12
  %_67.i.1.i.i = load i32, ptr %43, align 4, !dbg !8437, !alias.scope !8287, !noalias !8306, !noundef !12
  %_66.i.1.i.i = sub i32 %now.i.i.i, %_67.i.1.i.i, !dbg !8443
  %_65.i.1.i.i = and i32 %_66.i.1.i.i, %_52.i.i, !dbg !8445
  %_64.i.1.i.i = zext i32 %_65.i.1.i.i to i64, !dbg !8446
  %_63.i.1.i.i = shl nuw nsw i64 %_64.i.1.i.i, 2, !dbg !8446
  %own.i.1.i.i = or disjoint i64 %_63.i.1.i.i, 1, !dbg !8446
  %_75.i.1.i.i = load i32, ptr %44, align 4, !dbg !8447, !alias.scope !8287, !noalias !8306, !noundef !12
  %_74.i.1.i.i = sub i32 %now.i.i.i, %_75.i.1.i.i, !dbg !8449
  %_73.i.1.i.i = and i32 %_74.i.1.i.i, %_52.i.i, !dbg !8451
  %_72.i.1.i.i = zext i32 %_73.i.1.i.i to i64, !dbg !8452
  %_71.i.1.i.i = shl nuw nsw i64 %_72.i.1.i.i, 2, !dbg !8452
  %partner.i.1.i.i = or disjoint i64 %_71.i.1.i.i, 1, !dbg !8452
  %_80.i.1.i.i = icmp ult i64 %own.i.1.i.i, %_58.1.i.i, !dbg !8453
  br i1 %_80.i.1.i.i, label %bb22.i.1.i.i, label %panic18.i.i.i, !dbg !8453

bb22.i.1.i.i:                                     ; preds = %bb28.i.i.i
  %430 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.1.i.i, !dbg !8453
  %_78.i.12136.i.i = load i32, ptr %430, align 4, !dbg !8453, !noalias !8306, !noundef !12
  %_84.i.1.i.i = icmp ult i64 %own.i.1.i.i, %_60.1.i.i, !dbg !9711
  br i1 %_84.i.1.i.i, label %bb24.i.1.i.i, label %panic20.i.i.i, !dbg !9711

bb24.i.1.i.i:                                     ; preds = %bb22.i.1.i.i
  %431 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.1.i.i, !dbg !9711
  %_82.i.12137.i.i = load i32, ptr %431, align 4, !dbg !9711, !noalias !8306, !noundef !12
  %_86.i.1.i.i = icmp ult i64 %partner.i.1.i.i, %_60.1.i.i, !dbg !8455
  br i1 %_86.i.1.i.i, label %bb26.i.1.i.i, label %panic22.i.i.i, !dbg !8455

bb26.i.1.i.i:                                     ; preds = %bb24.i.1.i.i
  %432 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.1.i.i, !dbg !8455
  %_85.i.12138.i.i = load i32, ptr %432, align 4, !dbg !8455, !noalias !8306, !noundef !12
  %_88.i.1.i.i = icmp ult i64 %partner.i.1.i.i, %_58.1.i.i, !dbg !8456
  br i1 %_88.i.1.i.i, label %bb28.i.1.i.i, label %panic24.i.i.i, !dbg !8456

bb28.i.1.i.i:                                     ; preds = %bb26.i.1.i.i
  %433 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.1.i.i, !dbg !8456
  %_87.i.12139.i.i = load i32, ptr %433, align 4, !dbg !8456, !noalias !8306, !noundef !12
  %_67.i.2.i.i = load i32, ptr %45, align 4, !dbg !8437, !alias.scope !8287, !noalias !8306, !noundef !12
  %_66.i.2.i.i = sub i32 %now.i.i.i, %_67.i.2.i.i, !dbg !8443
  %_65.i.2.i.i = and i32 %_66.i.2.i.i, %_52.i.i, !dbg !8445
  %_64.i.2.i.i = zext i32 %_65.i.2.i.i to i64, !dbg !8446
  %_63.i.2.i.i = shl nuw nsw i64 %_64.i.2.i.i, 2, !dbg !8446
  %own.i.2.i.i = or disjoint i64 %_63.i.2.i.i, 2, !dbg !8446
  %_75.i.2.i.i = load i32, ptr %46, align 4, !dbg !8447, !alias.scope !8287, !noalias !8306, !noundef !12
  %_74.i.2.i.i = sub i32 %now.i.i.i, %_75.i.2.i.i, !dbg !8449
  %_73.i.2.i.i = and i32 %_74.i.2.i.i, %_52.i.i, !dbg !8451
  %_72.i.2.i.i = zext i32 %_73.i.2.i.i to i64, !dbg !8452
  %_71.i.2.i.i = shl nuw nsw i64 %_72.i.2.i.i, 2, !dbg !8452
  %partner.i.2.i.i = or disjoint i64 %_71.i.2.i.i, 2, !dbg !8452
  %_80.i.2.i.i = icmp ult i64 %own.i.2.i.i, %_58.1.i.i, !dbg !8453
  br i1 %_80.i.2.i.i, label %bb22.i.2.i.i, label %panic18.i.i.i, !dbg !8453

bb22.i.2.i.i:                                     ; preds = %bb28.i.1.i.i
  %434 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.2.i.i, !dbg !8453
  %_78.i.22140.i.i = load i32, ptr %434, align 4, !dbg !8453, !noalias !8306, !noundef !12
  %_84.i.2.i.i = icmp ult i64 %own.i.2.i.i, %_60.1.i.i, !dbg !9711
  br i1 %_84.i.2.i.i, label %bb24.i.2.i.i, label %panic20.i.i.i, !dbg !9711

bb24.i.2.i.i:                                     ; preds = %bb22.i.2.i.i
  %435 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.2.i.i, !dbg !9711
  %_82.i.22141.i.i = load i32, ptr %435, align 4, !dbg !9711, !noalias !8306, !noundef !12
  %_86.i.2.i.i = icmp ult i64 %partner.i.2.i.i, %_60.1.i.i, !dbg !8455
  br i1 %_86.i.2.i.i, label %bb26.i.2.i.i, label %panic22.i.i.i, !dbg !8455

bb26.i.2.i.i:                                     ; preds = %bb24.i.2.i.i
  %436 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.2.i.i, !dbg !8455
  %_85.i.22142.i.i = load i32, ptr %436, align 4, !dbg !8455, !noalias !8306, !noundef !12
  %_88.i.2.i.i = icmp ult i64 %partner.i.2.i.i, %_58.1.i.i, !dbg !8456
  br i1 %_88.i.2.i.i, label %bb28.i.2.i.i, label %panic24.i.i.i, !dbg !8456

bb28.i.2.i.i:                                     ; preds = %bb26.i.2.i.i
  %437 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.2.i.i, !dbg !8456
  %_87.i.22143.i.i = load i32, ptr %437, align 4, !dbg !8456, !noalias !8306, !noundef !12
  %_67.i.3.i.i = load i32, ptr %47, align 4, !dbg !8437, !alias.scope !8287, !noalias !8306, !noundef !12
  %_66.i.3.i.i = sub i32 %now.i.i.i, %_67.i.3.i.i, !dbg !8443
  %_65.i.3.i.i = and i32 %_66.i.3.i.i, %_52.i.i, !dbg !8445
  %_64.i.3.i.i = zext i32 %_65.i.3.i.i to i64, !dbg !8446
  %_63.i.3.i.i = shl nuw nsw i64 %_64.i.3.i.i, 2, !dbg !8446
  %own.i.3.i.i = or disjoint i64 %_63.i.3.i.i, 3, !dbg !8446
  %_75.i.3.i.i = load i32, ptr %48, align 4, !dbg !8447, !alias.scope !8287, !noalias !8306, !noundef !12
  %_74.i.3.i.i = sub i32 %now.i.i.i, %_75.i.3.i.i, !dbg !8449
  %_73.i.3.i.i = and i32 %_74.i.3.i.i, %_52.i.i, !dbg !8451
  %_72.i.3.i.i = zext i32 %_73.i.3.i.i to i64, !dbg !8452
  %_71.i.3.i.i = shl nuw nsw i64 %_72.i.3.i.i, 2, !dbg !8452
  %partner.i.3.i.i = or disjoint i64 %_71.i.3.i.i, 3, !dbg !8452
  %_80.i.3.i.i = icmp ult i64 %own.i.3.i.i, %_58.1.i.i, !dbg !8453
  br i1 %_80.i.3.i.i, label %bb22.i.3.i.i, label %panic18.i.i.i, !dbg !8453

bb22.i.3.i.i:                                     ; preds = %bb28.i.2.i.i
  %438 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.3.i.i, !dbg !8453
  %_78.i.32144.i.i = load i32, ptr %438, align 4, !dbg !8453, !noalias !8306, !noundef !12
  %_84.i.3.i.i = icmp ult i64 %own.i.3.i.i, %_60.1.i.i, !dbg !9711
  br i1 %_84.i.3.i.i, label %bb24.i.3.i.i, label %panic20.i.i.i, !dbg !9711

bb24.i.3.i.i:                                     ; preds = %bb22.i.3.i.i
  %439 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.3.i.i, !dbg !9711
  %_82.i.32145.i.i = load i32, ptr %439, align 4, !dbg !9711, !noalias !8306, !noundef !12
  %_86.i.3.i.i = icmp ult i64 %partner.i.3.i.i, %_60.1.i.i, !dbg !8455
  br i1 %_86.i.3.i.i, label %bb26.i.3.i.i, label %panic22.i.i.i, !dbg !8455

bb26.i.3.i.i:                                     ; preds = %bb24.i.3.i.i
  %_88.i.3.i.i = icmp ult i64 %partner.i.3.i.i, %_58.1.i.i, !dbg !8456
  br i1 %_88.i.3.i.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit163.i.i, label %panic24.i.i.i, !dbg !8456

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKb1_EB3_.exit.i: ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit163.i.i
  %_107.i.i.i = add i32 %base.i.i.i, %20, !dbg !9712
  store i32 %_107.i.i.i, ptr %_51.i.i, align 4, !dbg !9714, !alias.scope !8287, !noalias !8306
  br label %bb4.i, !dbg !9715

bb7.i:                                            ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKB1r_EB3_.exit.i, %bb4.i
  %440 = load i32, ptr %14, align 4, !dbg !9716, !alias.scope !8212, !noalias !8226, !noundef !12
  %441 = sub i32 %440, %20, !dbg !9716
  store i32 %441, ptr %14, align 4, !dbg !9716, !alias.scope !8212, !noalias !8226
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9717), !dbg !9720
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9721), !dbg !9720
  call void @llvm.lifetime.start.p0(ptr nonnull %iter.i.i), !dbg !9723, !noalias !9728
  store i64 0, ptr %iter.i.i, align 8, !dbg !9723, !noalias !9728
  %_7.sroa.0.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 8, !dbg !9723
  store i64 2, ptr %_7.sroa.0.sroa.2.0.iter.sroa_idx.i.i, align 8, !dbg !9723, !noalias !9728
  %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 16, !dbg !9723
  store ptr %_37.0, ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, align 8, !dbg !9723, !noalias !9728
  %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 24, !dbg !9723
  store i64 %_37.1, ptr %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i, align 8, !dbg !9723, !noalias !9728
  %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 32, !dbg !9723
  store ptr %_38.0, ptr %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i, align 8, !dbg !9723, !noalias !9728
  %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 40, !dbg !9723
  store i64 %_38.1, ptr %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i, align 8, !dbg !9723, !noalias !9728
  %_7.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 48, !dbg !9723
  store i64 0, ptr %_7.sroa.2.0.iter.sroa_idx.i.i, align 8, !dbg !9723, !noalias !9728
  %_71197.not.i.i = icmp eq i32 %_27, 0
  %442 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %443 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %444 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %445 = getelementptr inbounds nuw i8, ptr %self, i64 1776
  %446 = getelementptr inbounds nuw i8, ptr %self, i64 1816
  %447 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %448 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> zeroinitializer, i8 0)
  %449 = bitcast <4 x float> %448 to <2 x i64>
  br label %bb6.i6.i, !dbg !9731

bb6.i6.i:                                         ; preds = %bb1.backedge.i.i, %bb7.i
  %450 = phi i64 [ 24, %bb7.i ], [ 32, %bb1.backedge.i.i ]
  %_5.not.i.i.i.i.i = phi i1 [ false, %bb7.i ], [ true, %bb1.backedge.i.i ]
  %451 = phi i64 [ 0, %bb7.i ], [ 1, %bb1.backedge.i.i ]
  %self3.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<&mut [f32]>", ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, i64 %451, !dbg !9737
  %_14.0.i.i.i.i.i = load ptr, ptr %self3.i.i.i.i.i, align 8, !dbg !9742, !alias.scope !9746, !noalias !9753, !nonnull !12, !align !3484, !noundef !12
  %452 = getelementptr inbounds nuw i8, ptr %self3.i.i.i.i.i, i64 8, !dbg !9742
  %_14.1.i.i.i.i.i = load i64, ptr %452, align 8, !dbg !9742, !alias.scope !9746, !noalias !9753, !noundef !12
  %453 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x4_::f32x4>", ptr %self, i64 %451, !dbg !9755
  %454 = getelementptr inbounds nuw i8, ptr %453, i64 1376, !dbg !9755
  %_17.sroa.0.0.copyload.i.i = load <2 x i64>, ptr %454, align 16, !dbg !9755, !alias.scope !9757, !noalias !9758
  %fst_len.i.i.i = and i64 %_14.1.i.i.i.i.i, -4, !dbg !9759
  %_22.not.i26190.i.i = icmp eq i64 %fst_len.i.i.i, 0, !dbg !9770
  br i1 %_22.not.i26190.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %bb13.i27.i.i, !dbg !9770

bb13.i27.i.i:                                     ; preds = %bb6.i6.i, %bb13.i27.i.i
  %iter.sroa.0.0.i25193.i.i = phi ptr [ %_27.i28.i.i, %bb13.i27.i.i ], [ %_14.0.i.i.i.i.i, %bb6.i6.i ]
  %iter.sroa.5.0.i24192.i.i = phi i64 [ %_28.i29.i.i, %bb13.i27.i.i ], [ %fst_len.i.i.i, %bb6.i6.i ]
  %ok.i16.sroa.0.0191.i.i = phi <2 x i64> [ %459, %bb13.i27.i.i ], [ %449, %bb6.i6.i ]
  %lanes.i.sroa.0.0.copyload.i.i = load <2 x i64>, ptr %iter.sroa.0.0.i25193.i.i, align 4, !dbg !9777, !alias.scope !9783, !noalias !9787
  %_27.i28.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i25193.i.i, i64 16, !dbg !9791
  %_28.i29.i.i = add i64 %iter.sroa.5.0.i24192.i.i, -4, !dbg !9798
  %455 = and <2 x i64> %lanes.i.sroa.0.0.copyload.i.i, splat (i64 9223372034707292159), !dbg !9799
  %456 = bitcast <2 x i64> %455 to <4 x float>, !dbg !9806
  %457 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %456, <4 x float> splat (float 0x46293E5940000000), i8 1), !dbg !9807
  %458 = bitcast <4 x float> %457 to <2 x i64>, !dbg !9813
  %459 = and <2 x i64> %ok.i16.sroa.0.0191.i.i, %458, !dbg !9817
  %_22.not.i26.i.i = icmp eq i64 %_28.i29.i.i, 0, !dbg !9770
  br i1 %_22.not.i26.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %bb13.i27.i.i, !dbg !9770

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i: ; preds = %bb13.i27.i.i, %bb6.i6.i
  %ok.i16.sroa.0.0.lcssa.i.i = phi <2 x i64> [ %449, %bb6.i6.i ], [ %459, %bb13.i27.i.i ], !dbg !9819
  %460 = bitcast <2 x i64> %ok.i16.sroa.0.0.lcssa.i.i to <4 x i32>, !dbg !9820
  %461 = icmp sgt <4 x i32> %460, splat (i32 -1), !dbg !9829
  %462 = bitcast <4 x i1> %461 to i4, !dbg !9829
  %_0.i98.not.i.i = icmp eq i4 %462, 0, !dbg !9833
  br i1 %_0.i98.not.i.i, label %bb9.i.i, label %bb14.i.i, !dbg !9834

bb9.i.i:                                          ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  %463 = and <2 x i64> %_17.sroa.0.0.copyload.i.i, splat (i64 9223372034707292159), !dbg !9835
  %464 = bitcast <2 x i64> %463 to <4 x float>, !dbg !9842
  %465 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %464, <4 x float> splat (float 0x46293E5940000000), i8 1), !dbg !9843
  %466 = bitcast <4 x float> %465 to <2 x i64>, !dbg !9849
  %467 = and <2 x i64> %466, %449, !dbg !9853
  %468 = bitcast <2 x i64> %467 to <4 x i32>, !dbg !9855
  %469 = icmp sgt <4 x i32> %468, splat (i32 -1), !dbg !9860
  %470 = bitcast <4 x i1> %469 to i4, !dbg !9860
  %_0.i101.not.i.i = icmp eq i4 %470, 0, !dbg !9862
  br i1 %_0.i101.not.i.i, label %bb1.backedge.i.i, label %bb14.i.i, !dbg !9863

bb1.backedge.i.i:                                 ; preds = %bb17.backedge.i.i, %bb9.i.i
  br i1 %_5.not.i.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9run_blockB2_.exit, label %bb6.i6.i, !dbg !9731

bb14.i.i:                                         ; preds = %bb9.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  %fst_len.i.i.i.i = and i64 %_14.1.i.i.i.i.i, 2305843009213693948, !dbg !9864
  %_40.not71.i.i.i = icmp eq i64 %fst_len.i.i.i.i, 0, !dbg !9871
  br i1 %_40.not71.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i, label %bb23.i.i.i, !dbg !9871

bb24.loopexit.i.i.i:                              ; preds = %bb23.i.i.i
  %471 = bitcast <2 x i64> %476 to <4 x float>, !dbg !9878
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !9884

bb23.i.i.i:                                       ; preds = %bb14.i.i, %bb23.i.i.i
  %iter.sroa.0.074.i.i.i = phi ptr [ %_45.i.i.i, %bb23.i.i.i ], [ %_14.0.i.i.i.i.i, %bb14.i.i ]
  %iter.sroa.5.073.i.i.i = phi i64 [ %_46.i.i.i, %bb23.i.i.i ], [ %fst_len.i.i.i.i, %bb14.i.i ]
  %ok.sroa.0.072.i.i.i = phi <2 x i64> [ %476, %bb23.i.i.i ], [ %449, %bb14.i.i ]
  %lanes.i.sroa.0.0.copyload.i.i.i = load <2 x i64>, ptr %iter.sroa.0.074.i.i.i, align 4, !dbg !9885, !alias.scope !9890, !noalias !9896
  %_45.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i.i, i64 16, !dbg !9900
  %_46.i.i.i = add i64 %iter.sroa.5.073.i.i.i, -4, !dbg !9907
  %472 = and <2 x i64> %lanes.i.sroa.0.0.copyload.i.i.i, splat (i64 9223372034707292159), !dbg !9908
  %473 = bitcast <2 x i64> %472 to <4 x float>, !dbg !9914
  %474 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %473, <4 x float> splat (float 0x46293E5940000000), i8 1), !dbg !9915
  %475 = bitcast <4 x float> %474 to <2 x i64>, !dbg !9878
  %476 = and <2 x i64> %ok.sroa.0.072.i.i.i, %475, !dbg !9921
  %_40.not.i.i.i = icmp eq i64 %_46.i.i.i, 0, !dbg !9871
  br i1 %_40.not.i.i.i, label %bb24.loopexit.i.i.i, label %bb23.i.i.i, !dbg !9871

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i: ; preds = %bb24.loopexit.i.i.i, %bb14.i.i
  %ok.sroa.0.0.lcssa.i.i.i = phi <4 x float> [ %448, %bb14.i.i ], [ %471, %bb24.loopexit.i.i.i ], !dbg !9923
  %477 = and <2 x i64> %_17.sroa.0.0.copyload.i.i, splat (i64 9223372034707292159), !dbg !9924
  %478 = bitcast <2 x i64> %477 to <4 x float>, !dbg !9931
  %479 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %478, <4 x float> splat (float 0x46293E5940000000), i8 1), !dbg !9932
  %480 = bitcast <4 x float> %479 to <2 x i64>, !dbg !9938
  %481 = and <2 x i64> %480, %449, !dbg !9942
  %482 = bitcast <4 x float> %ok.sroa.0.0.lcssa.i.i.i to <4 x i32>, !dbg !9944
  %483 = icmp slt <4 x i32> %482, zeroinitializer, !dbg !9949
  %bc.i.i.i = select <4 x i1> %483, <4 x i32> zeroinitializer, <4 x i32> splat (i32 1065353216), !dbg !9949
  %484 = extractelement <4 x i32> %bc.i.i.i, i64 0, !dbg !9951
  %485 = icmp ne i32 %484, 0, !dbg !9951
  %486 = zext i1 %485 to i32, !dbg !9951
  %487 = extractelement <4 x i32> %bc.i.i.i, i64 1, !dbg !9951
  %488 = icmp eq i32 %487, 0, !dbg !9951
  %489 = select i1 %488, i32 0, i32 2, !dbg !9951
  %490 = extractelement <4 x i32> %bc.i.i.i, i64 2, !dbg !9951
  %491 = icmp eq i32 %490, 0, !dbg !9951
  %492 = select i1 %491, i32 0, i32 4, !dbg !9951
  %493 = extractelement <4 x i32> %bc.i.i.i, i64 3, !dbg !9951
  %494 = icmp eq i32 %493, 0, !dbg !9951
  %495 = select i1 %494, i32 0, i32 8, !dbg !9951
  %496 = bitcast <2 x i64> %481 to <4 x i32>, !dbg !9955
  %497 = icmp slt <4 x i32> %496, zeroinitializer, !dbg !9959
  %bc.i116.i.i = select <4 x i1> %497, <4 x i32> zeroinitializer, <4 x i32> splat (i32 1065353216), !dbg !9959
  %498 = extractelement <4 x i32> %bc.i116.i.i, i64 0, !dbg !9961
  %499 = icmp ne i32 %498, 0, !dbg !9961
  %500 = zext i1 %499 to i32, !dbg !9961
  %501 = extractelement <4 x i32> %bc.i116.i.i, i64 1, !dbg !9961
  %502 = icmp eq i32 %501, 0, !dbg !9961
  %503 = select i1 %502, i32 0, i32 2, !dbg !9961
  %504 = extractelement <4 x i32> %bc.i116.i.i, i64 2, !dbg !9961
  %505 = icmp eq i32 %504, 0, !dbg !9961
  %506 = select i1 %505, i32 0, i32 4, !dbg !9961
  %507 = extractelement <4 x i32> %bc.i116.i.i, i64 3, !dbg !9961
  %508 = icmp eq i32 %507, 0, !dbg !9961
  %509 = select i1 %508, i32 0, i32 8, !dbg !9961
  %mask.sroa.0.1.1.i118.i.i = or disjoint i32 %489, %486, !dbg !9961
  %mask.sroa.0.1.2.i120.i.i = or disjoint i32 %mask.sroa.0.1.1.i118.i.i, %492, !dbg !9961
  %mask.sroa.0.1.3.i122.i.i = or disjoint i32 %mask.sroa.0.1.2.i120.i.i, %495, !dbg !9961
  %mask.sroa.0.1.1.i.i.i = or i32 %mask.sroa.0.1.3.i122.i.i, %500, !dbg !9951
  %mask.sroa.0.1.2.i.i.i = or i32 %mask.sroa.0.1.1.i.i.i, %503, !dbg !9951
  %mask.sroa.0.1.3.i.i.i = or i32 %mask.sroa.0.1.2.i.i.i, %506, !dbg !9951
  %failed.i.i = or i32 %mask.sroa.0.1.3.i.i.i, %509, !dbg !9962
  %invariant.gep.i.i = getelementptr [8 x float], ptr %self, i64 %451, !dbg !9963
  %ring.i.i.i = getelementptr inbounds nuw %Ring, ptr %443, i64 %451
  %510 = getelementptr inbounds nuw i8, ptr %ring.i.i.i, i64 8
  %invariant.gep200.i.i = getelementptr %LaneTiming, ptr %444, i64 %451, !dbg !9966
  %511 = getelementptr inbounds nuw %Ring, ptr %self, i64 %451
  %512 = getelementptr inbounds nuw i8, ptr %511, i64 800
  %513 = getelementptr inbounds nuw %"kernel::GateCoef<wide::f32x4_::f32x4>", ptr %447, i64 %451
  %_19.i133.i.i = getelementptr inbounds nuw i8, ptr %513, i64 32
  %_25.i.i.i = getelementptr inbounds nuw i8, ptr %513, i64 16
  %514 = getelementptr inbounds nuw %"kernel::GateCoef<wide::f32x4_::f32x4>", ptr %self, i64 %451
  %515 = getelementptr inbounds nuw i8, ptr %514, i64 928
  %516 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x4_::f32x4>", ptr %13, i64 %451
  %_17.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 288
  %_19.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 256
  %_21.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 272
  %_37.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 16
  %_39.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 32
  %_41.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 48
  %iter.sroa.0.0.ptr27.1.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 64
  %_37.1.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 80
  %_39.1.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 96
  %_41.1.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 112
  %iter.sroa.0.0.ptr27.2.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 128
  %_37.2.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 144
  %_39.2.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 160
  %_41.2.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 176
  %iter.sroa.0.0.ptr27.3.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 192
  %_37.3.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 208
  %_39.3.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 224
  %_41.3.i.i.i = getelementptr inbounds nuw i8, ptr %516, i64 240
  %517 = getelementptr inbounds nuw i8, ptr %reports, i64 %450
  br label %bb30.i.i, !dbg !9966

bb30.i.i:                                         ; preds = %bb17.backedge.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i
  %iter1.sroa.0.0199.i.i = phi i64 [ 0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit.i.i ], [ %518, %bb17.backedge.i.i ]
  %518 = add nuw nsw i64 %iter1.sroa.0.0199.i.i, 1, !dbg !9972
  %519 = trunc nuw nsw i64 %iter1.sroa.0.0199.i.i to i32, !dbg !9978
  %_35.i.i = shl nuw nsw i32 1, %519, !dbg !9978
  %_34.i.i = and i32 %_35.i.i, %failed.i.i, !dbg !9980
  %520 = icmp eq i32 %_34.i.i, 0, !dbg !9980
  br i1 %520, label %bb17.backedge.i.i, label %bb20.preheader.i.i, !dbg !9980

bb20.preheader.i.i:                               ; preds = %bb30.i.i
  br i1 %_71197.not.i.i, label %bb33.i.i, label %bb32.i.i, !dbg !9981

bb33.i.i:                                         ; preds = %bb21.i.i, %bb20.preheader.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9987), !dbg !9990
  %slots.i.i.i = load i64, ptr %442, align 16, !dbg !9993, !alias.scope !9997, !noalias !9758, !noundef !12
  %_193.not.i.i.i = icmp eq i64 %slots.i.i.i, 0, !dbg !9998
  br i1 %_193.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.lr.ph.i.i.i, !dbg !10009

bb7.lr.ph.i.i.i:                                  ; preds = %bb33.i.i
  %_23.1.i.i.i = load i64, ptr %510, align 8, !alias.scope !9997, !noalias !9758, !noundef !12
  br label %bb7.i.i.i, !dbg !10009

bb7.i.i.i:                                        ; preds = %bb3.i.i.i, %bb7.lr.ph.i.i.i
  %iter.sroa.0.04.i.i.i = phi i64 [ 0, %bb7.lr.ph.i.i.i ], [ %521, %bb3.i.i.i ]
  %_10.i.i.i = shl i64 %iter.sroa.0.04.i.i.i, 2, !dbg !10010
  %_9.i124.i.i = add nuw nsw i64 %_10.i.i.i, %iter1.sroa.0.0199.i.i, !dbg !10010
  %_12.i.i.i = icmp ult i64 %_9.i124.i.i, %_23.1.i.i.i, !dbg !10012
  br i1 %_12.i.i.i, label %bb3.i.i.i, label %panic1.i.i.i, !dbg !10012

bb3.i.i.i:                                        ; preds = %bb7.i.i.i
  %_23.0.i.i.i = load ptr, ptr %ring.i.i.i, align 8, !dbg !10012, !alias.scope !9997, !noalias !9758, !nonnull !12, !noundef !12
  %521 = add nuw i64 %iter.sroa.0.04.i.i.i, 1, !dbg !10013
  %522 = getelementptr inbounds nuw float, ptr %_23.0.i.i.i, i64 %_9.i124.i.i, !dbg !10012
  store float 0.000000e+00, ptr %522, align 4, !dbg !10012, !noalias !10019
  %exitcond.not.i.i.i = icmp eq i64 %521, %slots.i.i.i, !dbg !9998
  br i1 %exitcond.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.i.i.i, !dbg !10009

panic1.i.i.i:                                     ; preds = %bb7.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_9.i124.i.i, i64 noundef %_23.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f8f0512af3f0ba047152c3c522a44b59) #24, !dbg !10012, !noalias !10019
  unreachable, !dbg !10012

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i: ; preds = %bb3.i.i.i, %bb33.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10020), !dbg !10023
  %gep.i.i = getelementptr [2 x [8 x float]], ptr %invariant.gep.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10024
  %values.sroa.0.0.copyload.i.i.i = load float, ptr %gep.i.i, align 16, !dbg !10024, !alias.scope !10027, !noalias !9758
  %values.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 4, !dbg !10024
  %values.sroa.5.0.copyload.i.i.i = load float, ptr %values.sroa.5.0..sroa_idx.i.i.i, align 4, !dbg !10024, !alias.scope !10027, !noalias !9758
  %values.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 8, !dbg !10024
  %values.sroa.6.0.copyload.i.i.i = load float, ptr %values.sroa.6.0..sroa_idx.i.i.i, align 8, !dbg !10024, !alias.scope !10027, !noalias !9758
  %values.sroa.7.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 12, !dbg !10024
  %values.sroa.7.0.copyload.i.i.i = load float, ptr %values.sroa.7.0..sroa_idx.i.i.i, align 4, !dbg !10024, !alias.scope !10027, !noalias !9758
  %values.sroa.8.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 16, !dbg !10024
  %values.sroa.8.0.copyload.i.i.i = load float, ptr %values.sroa.8.0..sroa_idx.i.i.i, align 16, !dbg !10024, !alias.scope !10027, !noalias !9758
  %values.sroa.9.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 20, !dbg !10024
  %values.sroa.9.0.copyload.i.i.i = load float, ptr %values.sroa.9.0..sroa_idx.i.i.i, align 4, !dbg !10024, !alias.scope !10027, !noalias !9758
  %values.sroa.10.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 24, !dbg !10024
  %values.sroa.10.0.copyload.i.i.i = load float, ptr %values.sroa.10.0..sroa_idx.i.i.i, align 8, !dbg !10024, !alias.scope !10027, !noalias !9758
  %values.sroa.11.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 28, !dbg !10024
  %values.sroa.11.0.copyload.i.i.i = load float, ptr %values.sroa.11.0..sroa_idx.i.i.i, align 4, !dbg !10024, !alias.scope !10027, !noalias !9758
  %gep201.i.i = getelementptr [2 x %LaneTiming], ptr %invariant.gep200.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10028
  store float %values.sroa.11.0.copyload.i.i.i, ptr %gep201.i.i, align 16, !dbg !10028, !alias.scope !10027, !noalias !9758
  %_7.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep201.i.i, i64 4, !dbg !10028
  store float %values.sroa.8.0.copyload.i.i.i, ptr %_7.sroa.4.0..sroa_idx.i.i.i, align 4, !dbg !10028, !alias.scope !10027, !noalias !9758
  %_7.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep201.i.i, i64 8, !dbg !10028
  store float %values.sroa.9.0.copyload.i.i.i, ptr %_7.sroa.5.0..sroa_idx.i.i.i, align 8, !dbg !10028, !alias.scope !10027, !noalias !9758
  %_7.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep201.i.i, i64 12, !dbg !10028
  store float %values.sroa.10.0.copyload.i.i.i, ptr %_7.sroa.6.0..sroa_idx.i.i.i, align 4, !dbg !10028, !alias.scope !10027, !noalias !9758
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10030), !dbg !10033
  %sample_rate.i.i.i = load i32, ptr %445, align 8, !dbg !10034, !alias.scope !10036, !noalias !9758, !noundef !12
  %_31.i.i.i = fpext float %values.sroa.11.0.copyload.i.i.i to double, !dbg !10037
  %_32.i.i.i = uitofp i32 %sample_rate.i.i.i to double, !dbg !10040
  %_30.i.i8.i = fmul double %_31.i.i.i, %_32.i.i.i, !dbg !10042
  %_29.i.i9.i = fdiv double %_30.i.i8.i, 1.000000e+03, !dbg !10042
  %_28.i130.i.i = fadd double %_29.i.i9.i, 5.000000e-01, !dbg !10043
  %523 = tail call double @llvm.floor.f64(double %_28.i130.i.i), !dbg !10044
  %or.cond.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %523, i32 527), !dbg !10047
  %_35.i.i.i = fcmp ogt double %523, 0x41EFFFFFFFE00000
  %or.cond10.i.i.i = or i1 %or.cond.i.i.i, %_35.i.i.i, !dbg !10047
  br i1 %or.cond10.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i, label %bb19.i.i.i, !dbg !10047

bb19.i.i.i:                                       ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i
  %_45.i131.i.i = fpext float %values.sroa.9.0.copyload.i.i.i to double, !dbg !10048
  %_44.i.i.i = fmul double %_45.i131.i.i, %_32.i.i.i, !dbg !10051
  %_43.i.i.i = fdiv double %_44.i.i.i, 1.000000e+03, !dbg !10051
  %_42.i.i.i = fadd double %_43.i.i.i, 5.000000e-01, !dbg !10052
  %524 = tail call double @llvm.floor.f64(double %_42.i.i.i), !dbg !10053
  %or.cond11.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %524, i32 527), !dbg !10056
  %_48.i.i.i = fcmp ogt double %524, 0x41EFFFFFFFE00000
  %or.cond12.i.i.i = or i1 %or.cond11.i.i.i, %_48.i.i.i, !dbg !10056
  br i1 %or.cond12.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i, label %bb20.3.i.i.i, !dbg !10056

bb20.3.i.i.i:                                     ; preds = %bb19.i.i.i
  %_12.i132.i.i = load i32, ptr %446, align 8, !dbg !10057, !alias.scope !10036, !noalias !9758, !noundef !12
  %_36.i.i10.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %523), !dbg !10058
  %_49.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %524), !dbg !10059
  %525 = getelementptr inbounds nuw i32, ptr %512, i64 %iter1.sroa.0.0199.i.i, !dbg !10060
  %526 = tail call i32 @llvm.usub.sat.i32(i32 %_12.i132.i.i, i32 %_36.i.i10.i), !dbg !10060
  store i32 %526, ptr %525, align 4, !dbg !10060, !alias.scope !10036, !noalias !9758
  %_20.i.i.i = uitofp i32 %_49.i.i.i to float, !dbg !10061
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i129.i.i), !dbg !10062, !noalias !10064
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i.i129.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_19.i133.i.i, i64 16, i1 false), !dbg !10067, !noalias !9758
  %527 = getelementptr inbounds nuw float, ptr %words.i.i129.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10068
  store float %_20.i.i.i, ptr %527, align 4, !dbg !10068, !noalias !10064
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_19.i133.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i.i129.i.i, i64 16, i1 false), !dbg !10069, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i129.i.i), !dbg !10070, !noalias !10064
; call effect_runtime::envelope::attack_release_coefficient
  %_23.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.8.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !10071, !noalias !10072
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i128.i.i), !dbg !10073, !noalias !10075
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i128.i.i, ptr noundef nonnull align 16 dereferenceable(16) %513, i64 16, i1 false), !dbg !10078, !noalias !9758
  %528 = getelementptr inbounds nuw float, ptr %words.i17.i128.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10079
  store float %_23.i.i.i, ptr %528, align 4, !dbg !10079, !noalias !10075
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %513, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i128.i.i, i64 16, i1 false), !dbg !10080, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i128.i.i), !dbg !10081, !noalias !10075
; call effect_runtime::envelope::attack_release_coefficient
  %_26.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.10.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !10082, !noalias !10072
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i127.i.i), !dbg !10083, !noalias !10085
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i127.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_25.i.i.i, i64 16, i1 false), !dbg !10088, !noalias !9758
  %529 = getelementptr inbounds nuw float, ptr %words.i18.i127.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10089
  store float %_26.i.i.i, ptr %529, align 4, !dbg !10089, !noalias !10085
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_25.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i127.i.i, i64 16, i1 false), !dbg !10090, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i127.i.i), !dbg !10091, !noalias !10085
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i.i.i), !dbg !10092, !noalias !10094
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %515, i64 16, i1 false), !dbg !10097, !noalias !9758
  %530 = getelementptr inbounds nuw float, ptr %words.i.i.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10098
  %_0.i.i.i.i = load float, ptr %530, align 4, !dbg !10098, !noalias !10094, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i.i.i), !dbg !10099, !noalias !10094
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i12.i.i.i), !dbg !10100, !noalias !10103
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i12.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_17.i.i.i, i64 16, i1 false), !dbg !10106, !noalias !9758
  %531 = getelementptr inbounds nuw float, ptr %words.i12.i.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10107
  store float 0.000000e+00, ptr %531, align 4, !dbg !10107, !noalias !10103
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i12.i.i.i, i64 16, i1 false), !dbg !10108, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i12.i.i.i), !dbg !10109, !noalias !10103
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i13.i.i.i), !dbg !10110, !noalias !10112
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i13.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_19.i.i.i, i64 16, i1 false), !dbg !10115, !noalias !9758
  %532 = getelementptr inbounds nuw float, ptr %words.i13.i.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10116
  store float 1.000000e+00, ptr %532, align 4, !dbg !10116, !noalias !10112
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_19.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i13.i.i.i, i64 16, i1 false), !dbg !10117, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i13.i.i.i), !dbg !10118, !noalias !10112
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !10119, !noalias !10121
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i14.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_21.i.i.i, i64 16, i1 false), !dbg !10124, !noalias !9758
  %533 = getelementptr inbounds nuw float, ptr %words.i14.i.i.i, i64 %iter1.sroa.0.0199.i.i, !dbg !10125
  store float %_0.i.i.i.i, ptr %533, align 4, !dbg !10125, !noalias !10121
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_21.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i14.i.i.i, i64 16, i1 false), !dbg !10126, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !10127, !noalias !10121
  %534 = getelementptr inbounds nuw float, ptr %words.i15.i.i.i, i64 %iter1.sroa.0.0199.i.i
  %535 = getelementptr inbounds nuw float, ptr %words.i16.i.i.i, i64 %iter1.sroa.0.0199.i.i
  %536 = getelementptr inbounds nuw float, ptr %words.i17.i.i.i, i64 %iter1.sroa.0.0199.i.i
  %537 = getelementptr inbounds nuw float, ptr %words.i18.i.i.i, i64 %iter1.sroa.0.0199.i.i
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !10128, !noalias !10132
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %516, i64 16, i1 false), !dbg !10135, !noalias !9758
  store float %values.sroa.0.0.copyload.i.i.i, ptr %534, align 4, !dbg !10136, !noalias !10132
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %516, ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, i64 16, i1 false), !dbg !10137, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !10138, !noalias !10132
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !10139, !noalias !10141
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_37.i.i.i, i64 16, i1 false), !dbg !10144, !noalias !9758
  store float %values.sroa.0.0.copyload.i.i.i, ptr %535, align 4, !dbg !10145, !noalias !10141
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_37.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, i64 16, i1 false), !dbg !10146, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !10147, !noalias !10141
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !10148, !noalias !10150
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_39.i.i.i, i64 16, i1 false), !dbg !10153, !noalias !9758
  store float 0.000000e+00, ptr %536, align 4, !dbg !10154, !noalias !10150
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_39.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, i64 16, i1 false), !dbg !10155, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !10156, !noalias !10150
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !10157, !noalias !10159
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_41.i.i.i, i64 16, i1 false), !dbg !10162, !noalias !9758
  store float 0.000000e+00, ptr %537, align 4, !dbg !10163, !noalias !10159
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_41.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, i64 16, i1 false), !dbg !10164, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !10165, !noalias !10159
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !10128, !noalias !10132
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.1.i.i.i, i64 16, i1 false), !dbg !10135, !noalias !9758
  store float %values.sroa.5.0.copyload.i.i.i, ptr %534, align 4, !dbg !10136, !noalias !10132
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.1.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, i64 16, i1 false), !dbg !10137, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !10138, !noalias !10132
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !10139, !noalias !10141
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_37.1.i.i.i, i64 16, i1 false), !dbg !10144, !noalias !9758
  store float %values.sroa.5.0.copyload.i.i.i, ptr %535, align 4, !dbg !10145, !noalias !10141
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_37.1.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, i64 16, i1 false), !dbg !10146, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !10147, !noalias !10141
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !10148, !noalias !10150
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_39.1.i.i.i, i64 16, i1 false), !dbg !10153, !noalias !9758
  store float 0.000000e+00, ptr %536, align 4, !dbg !10154, !noalias !10150
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_39.1.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, i64 16, i1 false), !dbg !10155, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !10156, !noalias !10150
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !10157, !noalias !10159
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_41.1.i.i.i, i64 16, i1 false), !dbg !10162, !noalias !9758
  store float 0.000000e+00, ptr %537, align 4, !dbg !10163, !noalias !10159
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_41.1.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, i64 16, i1 false), !dbg !10164, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !10165, !noalias !10159
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !10128, !noalias !10132
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.2.i.i.i, i64 16, i1 false), !dbg !10135, !noalias !9758
  store float %values.sroa.6.0.copyload.i.i.i, ptr %534, align 4, !dbg !10136, !noalias !10132
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.2.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, i64 16, i1 false), !dbg !10137, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !10138, !noalias !10132
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !10139, !noalias !10141
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_37.2.i.i.i, i64 16, i1 false), !dbg !10144, !noalias !9758
  store float %values.sroa.6.0.copyload.i.i.i, ptr %535, align 4, !dbg !10145, !noalias !10141
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_37.2.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, i64 16, i1 false), !dbg !10146, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !10147, !noalias !10141
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !10148, !noalias !10150
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_39.2.i.i.i, i64 16, i1 false), !dbg !10153, !noalias !9758
  store float 0.000000e+00, ptr %536, align 4, !dbg !10154, !noalias !10150
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_39.2.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, i64 16, i1 false), !dbg !10155, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !10156, !noalias !10150
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !10157, !noalias !10159
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_41.2.i.i.i, i64 16, i1 false), !dbg !10162, !noalias !9758
  store float 0.000000e+00, ptr %537, align 4, !dbg !10163, !noalias !10159
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_41.2.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, i64 16, i1 false), !dbg !10164, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !10165, !noalias !10159
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !10128, !noalias !10132
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.3.i.i.i, i64 16, i1 false), !dbg !10135, !noalias !9758
  store float %values.sroa.7.0.copyload.i.i.i, ptr %534, align 4, !dbg !10136, !noalias !10132
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %iter.sroa.0.0.ptr27.3.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i15.i.i.i, i64 16, i1 false), !dbg !10137, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !10138, !noalias !10132
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !10139, !noalias !10141
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_37.3.i.i.i, i64 16, i1 false), !dbg !10144, !noalias !9758
  store float %values.sroa.7.0.copyload.i.i.i, ptr %535, align 4, !dbg !10145, !noalias !10141
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_37.3.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i16.i.i.i, i64 16, i1 false), !dbg !10146, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !10147, !noalias !10141
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !10148, !noalias !10150
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_39.3.i.i.i, i64 16, i1 false), !dbg !10153, !noalias !9758
  store float 0.000000e+00, ptr %536, align 4, !dbg !10154, !noalias !10150
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_39.3.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i17.i.i.i, i64 16, i1 false), !dbg !10155, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !10156, !noalias !10150
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !10157, !noalias !10159
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %_41.3.i.i.i, i64 16, i1 false), !dbg !10162, !noalias !9758
  store float 0.000000e+00, ptr %537, align 4, !dbg !10163, !noalias !10159
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_41.3.i.i.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i18.i.i.i, i64 16, i1 false), !dbg !10164, !noalias !9758
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !10165, !noalias !10159
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i, !dbg !10166

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i: ; preds = %bb20.3.i.i.i, %bb19.i.i.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16clear_lane_ringsB2_.exit.i.i
  %gep203.i.i = getelementptr inbounds nuw %"effect_contract::ProcessReport", ptr %517, i64 %iter1.sroa.0.0199.i.i, !dbg !10167
  %_46.i.i = load i64, ptr %gep203.i.i, align 8, !dbg !10169, !alias.scope !10171, !noalias !10172, !noundef !12
  %538 = tail call i64 @llvm.uadd.sat.i64(i64 %_46.i.i, i64 range(i64 0, 4294967296) %frames), !dbg !10173
  store i64 %538, ptr %gep203.i.i, align 8, !dbg !10176, !alias.scope !10171, !noalias !10172
  br label %bb17.backedge.i.i, !dbg !9963

bb17.backedge.i.i:                                ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9seed_laneB2_.exit.i.i, %bb30.i.i
  %exitcond228.not.i.i = icmp eq i64 %518, 4, !dbg !10177
  br i1 %exitcond228.not.i.i, label %bb1.backedge.i.i, label %bb30.i.i, !dbg !9966

bb32.i.i:                                         ; preds = %bb20.preheader.i.i, %bb21.i.i
  %iter2.sroa.0.0198.i.i = phi i64 [ %539, %bb21.i.i ], [ 0, %bb20.preheader.i.i ]
  %_39.i.i = shl nuw nsw i64 %iter2.sroa.0.0198.i.i, 2, !dbg !10180
  %_38.i.i = add nuw nsw i64 %_39.i.i, %iter1.sroa.0.0199.i.i, !dbg !10180
  %_41.i.i = icmp ult i64 %_38.i.i, %_14.1.i.i.i.i.i, !dbg !10182
  br i1 %_41.i.i, label %bb21.i.i, label %panic4.i.i, !dbg !10182

bb21.i.i:                                         ; preds = %bb32.i.i
  %539 = add nuw nsw i64 %iter2.sroa.0.0198.i.i, 1, !dbg !10183
  %540 = getelementptr inbounds nuw float, ptr %_14.0.i.i.i.i.i, i64 %_38.i.i, !dbg !10182
  store float 0.000000e+00, ptr %540, align 4, !dbg !10182, !noalias !10189
  %exitcond.not.i7.i = icmp eq i64 %539, %frames, !dbg !10190
  br i1 %exitcond.not.i7.i, label %bb33.i.i, label %bb32.i.i, !dbg !9981

panic4.i.i:                                       ; preds = %bb32.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_38.i.i, i64 noundef %_14.1.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30d570589940b68d7f77af9df77eb2ee) #24, !dbg !10182, !noalias !10189
  unreachable, !dbg !10182

bb20.i:                                           ; preds = %bb4.i
  %541 = shl nuw nsw i64 %..i.i, 2, !dbg !10193
  %_52.i = icmp samesign ugt i64 %541, %_37.1, !dbg !10194
  br i1 %_52.i, label %bb24.i, label %bb25.i, !dbg !10194, !prof !180

bb25.i:                                           ; preds = %bb20.i
  %_60.i = icmp samesign ugt i64 %541, %_38.1, !dbg !10202
  br i1 %_60.i, label %bb26.i, label %bb27.i, !dbg !10202, !prof !180

bb24.i:                                           ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %541, i64 noundef range(i64 0, 2305843009213693952) %_37.1, i64 noundef range(i64 0, 2305843009213693952) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e6adad6d678d00eb6b02b8162f35ebb4) #24, !dbg !10206, !noalias !8226
  unreachable, !dbg !10206

bb27.i:                                           ; preds = %bb25.i
  %_59.i = getelementptr inbounds nuw float, ptr %_37.0, i64 %541, !dbg !10207
  %_55.i = sub nuw nsw i64 %_37.1, %541, !dbg !10212
  %_63.i = sub nuw nsw i64 %_38.1, %541, !dbg !10213
  %_67.i = getelementptr inbounds nuw float, ptr %_38.0, i64 %541, !dbg !10214
  %_26.i = sub nsw i64 %frames, %..i.i, !dbg !10219
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10220), !dbg !10223
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10224), !dbg !10223
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10226), !dbg !10223
  %data.i.i.i19.i = getelementptr inbounds nuw i8, ptr %self, i64 1392, !dbg !10228
  %_15.i20.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !10235
  %data.i.i892.i.i = getelementptr inbounds nuw i8, ptr %self, i64 832, !dbg !10237
  %542 = getelementptr inbounds nuw i8, ptr %self, i64 896, !dbg !10242
  %_32.i21.i = getelementptr inbounds nuw i8, ptr %self, i64 992, !dbg !10246
  %_58.0.i22.i = load ptr, ptr %_15.i20.i, align 8, !dbg !10247, !alias.scope !10248, !noalias !10249, !nonnull !12, !noundef !12
  %543 = getelementptr inbounds nuw i8, ptr %self, i64 776, !dbg !10247
  %_58.1.i23.i = load i64, ptr %543, align 8, !dbg !10247, !alias.scope !10248, !noalias !10249, !noundef !12
  %_45.i24.i = getelementptr inbounds nuw i8, ptr %self, i64 800, !dbg !10251
  %_60.0.i25.i = load ptr, ptr %data.i.i892.i.i, align 8, !dbg !10252, !alias.scope !10248, !noalias !10249, !nonnull !12, !noundef !12
  %544 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !10252
  %_60.1.i26.i = load i64, ptr %544, align 8, !dbg !10252, !alias.scope !10248, !noalias !10249, !noundef !12
  %_50.i27.i = getelementptr inbounds nuw i8, ptr %self, i64 864, !dbg !10253
  %_51.i28.i = getelementptr inbounds nuw i8, ptr %self, i64 1808, !dbg !10254
  %545 = getelementptr inbounds nuw i8, ptr %self, i64 1812, !dbg !10255
  %_52.i29.i = load i32, ptr %545, align 4, !dbg !10255, !alias.scope !10248, !noalias !10249, !noundef !12
  %546 = getelementptr inbounds nuw i8, ptr %self, i64 1816, !dbg !10256
  %_53.i30.i = load i32, ptr %546, align 8, !dbg !10256, !alias.scope !10248, !noalias !10249, !noundef !12
  %base.i.i35.i = load i32, ptr %_51.i28.i, align 4, !dbg !10257, !alias.scope !10248, !noalias !10267, !noundef !12
  %547 = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %548 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %549 = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %550 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %551 = getelementptr inbounds nuw i8, ptr %self, i64 976
  %552 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %553 = getelementptr inbounds nuw i8, ptr %self, i64 1360
  %554 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %555 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %556 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %557 = getelementptr inbounds nuw i8, ptr %self, i64 944
  %558 = getelementptr inbounds nuw i8, ptr %self, i64 1456
  %559 = getelementptr inbounds nuw i8, ptr %self, i64 1520
  %560 = getelementptr inbounds nuw i8, ptr %self, i64 1584
  %561 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %562 = getelementptr inbounds nuw i8, ptr %self, i64 1072
  %563 = getelementptr inbounds nuw i8, ptr %self, i64 1648
  %564 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %565 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %566 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %567 = getelementptr inbounds nuw i8, ptr %self, i64 1008
  %568 = getelementptr inbounds nuw i8, ptr %self, i64 1040
  %569 = lshr i64 %_63.i, 2, !dbg !10270
  %570 = lshr i64 %_55.i, 2, !dbg !10270
  %_67.i.i36.i = load i32, ptr %_45.i24.i, align 4, !alias.scope !10248, !noalias !10249
  %_75.i.i37.i = load i32, ptr %_50.i27.i, align 4, !alias.scope !10248, !noalias !10249
  %571 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %_67.i.1.i38.i = load i32, ptr %571, align 4, !alias.scope !10248, !noalias !10249
  %572 = getelementptr inbounds nuw i8, ptr %self, i64 868
  %_75.i.1.i39.i = load i32, ptr %572, align 4, !alias.scope !10248, !noalias !10249
  %573 = getelementptr inbounds nuw i8, ptr %self, i64 808
  %_67.i.2.i40.i = load i32, ptr %573, align 4, !alias.scope !10248, !noalias !10249
  %574 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %_75.i.2.i41.i = load i32, ptr %574, align 4, !alias.scope !10248, !noalias !10249
  %575 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %_67.i.3.i42.i = load i32, ptr %575, align 4, !alias.scope !10248, !noalias !10249
  %576 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %_75.i.3.i43.i = load i32, ptr %576, align 4, !alias.scope !10248, !noalias !10249
  %_41.i96.sroa.0.0.copyload.i.i = load <4 x float>, ptr %551, align 16, !alias.scope !10248, !noalias !10249
  %_37.i100.sroa.0.0.copyload.i.i = load <4 x float>, ptr %550, align 16, !alias.scope !10248, !noalias !10249
  %hysteresis.i106.sroa.0.0.copyload.i.i = load <4 x float>, ptr %549, align 16, !alias.scope !10248, !noalias !10249
  %range.i107.sroa.0.0.copyload1553.i.i = load <2 x i64>, ptr %548, align 16, !alias.scope !10248, !noalias !10249
  %ratio.i108.sroa.0.0.copyload.i.i = load <4 x float>, ptr %547, align 16, !alias.scope !10248, !noalias !10249
  %threshold.i109.sroa.0.0.copyload.i.i = load <4 x float>, ptr %13, align 16, !alias.scope !10248, !noalias !10249
  %577 = fsub <4 x float> %threshold.i109.sroa.0.0.copyload.i.i, %hysteresis.i106.sroa.0.0.copyload.i.i
  %_71.i66.sroa.0.0.copyload.i.i = load <4 x float>, ptr %554, align 16, !alias.scope !10248, !noalias !10249
  %_88.i50.sroa.0.0.copyload.i.i = load <4 x float>, ptr %556, align 16, !alias.scope !10248, !noalias !10249
  %_7.i790.i.i = load <4 x float>, ptr %542, align 16, !alias.scope !10248, !noalias !10249
  %578 = fadd <4 x float> %ratio.i108.sroa.0.0.copyload.i.i, splat (float -1.000000e+00)
  %579 = xor <2 x i64> %range.i107.sroa.0.0.copyload1553.i.i, splat (i64 -9223372034707292160)
  %580 = bitcast <2 x i64> %579 to <4 x float>
  %_98.i40.sroa.0.0.copyload.i.i = load <4 x float>, ptr %557, align 16, !alias.scope !10248, !noalias !10249
  %_41.i.sroa.0.0.copyload.i44.i = load <4 x float>, ptr %562, align 16, !alias.scope !10248, !noalias !10249
  %_37.i.sroa.0.0.copyload.i45.i = load <4 x float>, ptr %561, align 16, !alias.scope !10248, !noalias !10249
  %hysteresis.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %560, align 16, !alias.scope !10248, !noalias !10249
  %range.i.sroa.0.0.copyload1560.i.i = load <2 x i64>, ptr %559, align 16, !alias.scope !10248, !noalias !10249
  %ratio.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %558, align 16, !alias.scope !10248, !noalias !10249
  %threshold.i.sroa.0.0.copyload.i.i = load <4 x float>, ptr %data.i.i.i19.i, align 16, !alias.scope !10248, !noalias !10249
  %581 = fsub <4 x float> %threshold.i.sroa.0.0.copyload.i.i, %hysteresis.i.sroa.0.0.copyload.i.i
  %_71.i20.sroa.0.0.copyload.i46.i = load <4 x float>, ptr %565, align 16, !alias.scope !10248, !noalias !10249
  %_88.i10.sroa.0.0.copyload.i47.i = load <4 x float>, ptr %567, align 16, !alias.scope !10248, !noalias !10249
  %_7.i822.i.i = load <4 x float>, ptr %_32.i21.i, align 16, !alias.scope !10248, !noalias !10249
  %582 = fadd <4 x float> %ratio.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00)
  %583 = xor <2 x i64> %range.i.sroa.0.0.copyload1560.i.i, splat (i64 -9223372034707292160)
  %584 = bitcast <2 x i64> %583 to <4 x float>
  %_98.i.sroa.0.0.copyload.i48.i = load <4 x float>, ptr %568, align 16, !alias.scope !10248, !noalias !10249
  %.promoted.i.i = load <4 x float>, ptr %552, align 16, !alias.scope !10248, !noalias !10249
  %.promoted2013.i.i = load <4 x float>, ptr %553, align 16, !alias.scope !10248, !noalias !10249
  %.promoted2015.i.i = load <4 x float>, ptr %563, align 16, !alias.scope !10248, !noalias !10249
  %.promoted2017.i.i = load <4 x float>, ptr %564, align 16, !alias.scope !10248, !noalias !10249
  br label %bb40.i.i49.i, !dbg !10270

bb40.i.i49.i:                                     ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i, %bb27.i
  %_67.i22.sroa.0.0.copyload2018.i.i = phi <4 x float> [ %.promoted2017.i.i, %bb27.i ], [ %782, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i ]
  %_55.i26.sroa.0.0.copyload2016.i.i = phi <4 x float> [ %.promoted2015.i.i, %bb27.i ], [ %775, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i ]
  %_67.i70.sroa.0.0.copyload2014.i.i = phi <4 x float> [ %.promoted2013.i.i, %bb27.i ], [ %660, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i ]
  %_55.i82.sroa.0.0.copyload2012.i.i = phi <4 x float> [ %.promoted.i.i, %bb27.i ], [ %653, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i ]
  %iter.sroa.0.0.i1653.i.i = phi i64 [ 0, %bb27.i ], [ %585, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i ]
  %585 = add nuw nsw i64 %iter.sroa.0.0.i1653.i.i, 1, !dbg !10279
  %span.i.i50.i = shl i64 %iter.sroa.0.0.i1653.i.i, 2, !dbg !10285
  %_27.i.i51.i = trunc i64 %iter.sroa.0.0.i1653.i.i to i32, !dbg !10287
  %now.i.i52.i = add i32 %base.i.i35.i, %_27.i.i51.i, !dbg !10289
  %_30.i.i53.i = and i32 %now.i.i52.i, %_52.i29.i, !dbg !10292
  %_29.i.i54.i = zext i32 %_30.i.i53.i to i64, !dbg !10294
  %write.i.i55.i = shl nuw nsw i64 %_29.i.i54.i, 2, !dbg !10294
  %exitcond.not.i56.i = icmp eq i64 %iter.sroa.0.0.i1653.i.i, %570, !dbg !10295
  br i1 %exitcond.not.i56.i, label %bb43.i.i165.i, label %bb42.i.i57.i, !dbg !10295, !prof !2561

bb43.i.i165.i:                                    ; preds = %bb40.i.i49.i
  %_34.i.i166.i = add nuw nsw i64 %span.i.i50.i, 4, !dbg !10303
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i50.i, i64 noundef %_34.i.i166.i, i64 noundef range(i64 0, 2305843009213693952) %_55.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_97ed0782c9e7425b1b215f66cb64a347) #24, !dbg !10304, !noalias !10305
  unreachable, !dbg !10304

bb42.i.i57.i:                                     ; preds = %bb40.i.i49.i
  %_123.i.i58.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %span.i.i50.i, !dbg !10306
  %_36.i.i59.i = add nuw nsw i64 %write.i.i55.i, 4, !dbg !10310
  %_124.not.i.i60.i = icmp ugt i64 %_36.i.i59.i, %_58.1.i23.i, !dbg !10311
  br i1 %_124.not.i.i60.i, label %bb46.i.i164.i, label %bb45.i.i61.i, !dbg !10311, !prof !180

bb46.i.i164.i:                                    ; preds = %bb42.i.i57.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i55.i, i64 noundef %_36.i.i59.i, i64 noundef %_58.1.i23.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cad5a7b24766ffcdda9cbfe066eb4078) #24, !dbg !10316, !noalias !10305
  unreachable, !dbg !10316

bb45.i.i61.i:                                     ; preds = %bb42.i.i57.i
  %_133.i.i62.i = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %write.i.i55.i, !dbg !10317
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %_133.i.i62.i, ptr noundef nonnull align 4 dereferenceable(16) %_123.i.i58.i, i64 16, i1 false), !dbg !10321, !noalias !10326
  %exitcond1748.i.i = icmp eq i64 %iter.sroa.0.0.i1653.i.i, %569, !dbg !10327
  br i1 %exitcond1748.i.i, label %bb49.i.i163.i, label %bb48.i.i63.i, !dbg !10327, !prof !180

bb49.i.i163.i:                                    ; preds = %bb45.i.i61.i
  %586 = and i64 %_63.i, 2305843009213693948, !dbg !10270
  %587 = add nuw nsw i64 %586, 4, !dbg !10270
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i50.i, i64 noundef %587, i64 noundef range(i64 0, 2305843009213693952) %_63.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aebeae245abdbd708a3d39b73262e641) #24, !dbg !10331, !noalias !10305
  unreachable, !dbg !10331

bb48.i.i63.i:                                     ; preds = %bb45.i.i61.i
  %_141.i.i64.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %span.i.i50.i, !dbg !10332
  %_142.not.i.i65.i = icmp ugt i64 %_36.i.i59.i, %_60.1.i26.i, !dbg !10336
  br i1 %_142.not.i.i65.i, label %bb51.i.i162.i, label %bb50.i.i66.i, !dbg !10336, !prof !180

bb51.i.i162.i:                                    ; preds = %bb48.i.i63.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i55.i, i64 noundef %_36.i.i59.i, i64 noundef %_60.1.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_72e24c5578221343e8a784545a3910d2) #24, !dbg !10340, !noalias !10305
  unreachable, !dbg !10340

bb50.i.i66.i:                                     ; preds = %bb48.i.i63.i
  %_149.i.i67.i = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %write.i.i55.i, !dbg !10341
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %_149.i.i67.i, ptr noundef nonnull align 4 dereferenceable(16) %_141.i.i64.i, i64 16, i1 false), !dbg !10345, !noalias !10350
  %_52.i.i68.i = sub i32 %now.i.i52.i, %_53.i30.i, !dbg !10351
  %_51.i.i69.i = and i32 %_52.i.i68.i, %_52.i29.i, !dbg !10354
  %_50.i.i70.i = zext i32 %_51.i.i69.i to i64, !dbg !10355
  %read.i.i71.i = shl nuw nsw i64 %_50.i.i70.i, 2, !dbg !10355
  %_55.i.i72.i = add nuw nsw i64 %read.i.i71.i, 4, !dbg !10356
  %_182.not.i.i73.i = icmp ugt i64 %_55.i.i72.i, %_58.1.i23.i, !dbg !10358
  br i1 %_182.not.i.i73.i, label %bb61.i.i161.i, label %bb60.i.i74.i, !dbg !10358, !prof !180

bb61.i.i161.i:                                    ; preds = %bb50.i.i66.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i71.i, i64 noundef %_55.i.i72.i, i64 noundef %_58.1.i23.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba99eeeb3482270ebe1c674b4013dd6a) #24, !dbg !10362, !noalias !10305
  unreachable, !dbg !10362

bb60.i.i74.i:                                     ; preds = %bb50.i.i66.i
  %_189.i.i75.i = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %read.i.i71.i, !dbg !10363
  %lanes.i525.sroa.0.0.copyload.i.i = load <4 x float>, ptr %_189.i.i75.i, align 4, !dbg !10367, !alias.scope !10372, !noalias !10376
  %_190.not.i.i76.i = icmp ugt i64 %_55.i.i72.i, %_60.1.i26.i, !dbg !10380
  br i1 %_190.not.i.i76.i, label %bb64.i.i160.i, label %bb63.i.i77.i, !dbg !10380, !prof !180

bb64.i.i160.i:                                    ; preds = %bb60.i.i74.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i71.i, i64 noundef %_55.i.i72.i, i64 noundef %_60.1.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f4c27429e4885e1b619f1e4a3587acc) #24, !dbg !10385, !noalias !10305
  unreachable, !dbg !10385

bb63.i.i77.i:                                     ; preds = %bb60.i.i74.i
  %_195.i.i78.i = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %read.i.i71.i, !dbg !10386
  %lanes.i519.sroa.0.0.copyload.i.i = load <4 x float>, ptr %_195.i.i78.i, align 4, !dbg !10390, !alias.scope !10395, !noalias !10399
  %_66.i.i79.i = sub i32 %now.i.i52.i, %_67.i.i36.i, !dbg !10403
  %_65.i.i80.i = and i32 %_66.i.i79.i, %_52.i29.i, !dbg !10410
  %_64.i.i81.i = zext i32 %_65.i.i80.i to i64, !dbg !10411
  %_63.i.i82.i = shl nuw nsw i64 %_64.i.i81.i, 2, !dbg !10411
  %_74.i.i83.i = sub i32 %now.i.i52.i, %_75.i.i37.i, !dbg !10412
  %_73.i.i84.i = and i32 %_74.i.i83.i, %_52.i29.i, !dbg !10415
  %_72.i.i85.i = zext i32 %_73.i.i84.i to i64, !dbg !10416
  %_71.i.i86.i = shl nuw nsw i64 %_72.i.i85.i, 2, !dbg !10416
  %_80.i.i87.i = icmp ult i64 %_63.i.i82.i, %_58.1.i23.i, !dbg !10417
  br i1 %_80.i.i87.i, label %bb22.i.i90.i, label %panic18.i.i88.i, !dbg !10417

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i: ; preds = %bb26.i.3.i151.i
  %588 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.3.i145.i, !dbg !10419
  %_85.i.31969.i.i = load i32, ptr %588, align 4, !dbg !10419, !noalias !10305, !noundef !12
  %589 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.3.i145.i, !dbg !10420
  %_87.i.31970.i.i = load i32, ptr %589, align 4, !dbg !10420, !noalias !10305, !noundef !12
  %590 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_41.i96.sroa.0.0.copyload.i.i, i8 1), !dbg !10421
  %591 = bitcast <4 x float> %590 to <4 x i32>, !dbg !10438
  %592 = icmp slt <4 x i32> %591, zeroinitializer, !dbg !10442
  %lanes.i513.sroa.0.0.vec.insert.i.i = insertelement <4 x i32> poison, i32 %_78.i1955.i.i, i64 0, !dbg !10444
  %lanes.i513.sroa.0.4.vec.insert.i.i = insertelement <4 x i32> %lanes.i513.sroa.0.0.vec.insert.i.i, i32 %_78.i.11959.i.i, i64 1, !dbg !10444
  %lanes.i513.sroa.0.8.vec.insert.i.i = insertelement <4 x i32> %lanes.i513.sroa.0.4.vec.insert.i.i, i32 %_78.i.21963.i.i, i64 2, !dbg !10444
  %lanes.i513.sroa.0.12.vec.insert.i.i = insertelement <4 x i32> %lanes.i513.sroa.0.8.vec.insert.i.i, i32 %_78.i.31967.i.i, i64 3, !dbg !10444
  %593 = bitcast <4 x i32> %lanes.i513.sroa.0.12.vec.insert.i.i to <2 x i64>, !dbg !10449
  %594 = and <2 x i64> %593, splat (i64 9223372034707292159), !dbg !10450
  %595 = bitcast <2 x i64> %594 to <4 x float>, !dbg !10456
  %596 = fmul <4 x float> %595, splat (float 5.000000e-01), !dbg !10457
  %lanes.i507.sroa.0.0.vec.insert.i.i = insertelement <4 x i32> poison, i32 %_82.i1956.i.i, i64 0, !dbg !10462
  %lanes.i507.sroa.0.4.vec.insert.i.i = insertelement <4 x i32> %lanes.i507.sroa.0.0.vec.insert.i.i, i32 %_82.i.11960.i.i, i64 1, !dbg !10462
  %lanes.i507.sroa.0.8.vec.insert.i.i = insertelement <4 x i32> %lanes.i507.sroa.0.4.vec.insert.i.i, i32 %_82.i.21964.i.i, i64 2, !dbg !10462
  %lanes.i507.sroa.0.12.vec.insert.i.i = insertelement <4 x i32> %lanes.i507.sroa.0.8.vec.insert.i.i, i32 %_82.i.31968.i.i, i64 3, !dbg !10462
  %597 = bitcast <4 x i32> %lanes.i507.sroa.0.12.vec.insert.i.i to <2 x i64>, !dbg !10467
  %598 = and <2 x i64> %597, splat (i64 9223372034707292159), !dbg !10468
  %599 = bitcast <2 x i64> %598 to <4 x float>, !dbg !10474
  %600 = fmul <4 x float> %599, splat (float 5.000000e-01), !dbg !10475
  %601 = fadd <4 x float> %596, %600, !dbg !10480
  %602 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_37.i100.sroa.0.0.copyload.i.i, i8 1), !dbg !10485
  %603 = bitcast <4 x float> %602 to <4 x i32>, !dbg !10491
  %604 = icmp slt <4 x i32> %603, zeroinitializer, !dbg !10495
  %605 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %595, <4 x float> %599), !dbg !10497
  %606 = select <4 x i1> %604, <4 x float> %605, <4 x float> %595, !dbg !10495
  %607 = select <4 x i1> %592, <4 x float> %601, <4 x float> %606, !dbg !10442
  %608 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %607, <4 x float> splat (float 0x3E45798EE0000000)), !dbg !10502
  %609 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %608, <4 x float> splat (float 0x3810000000000000)), !dbg !10507
  %610 = bitcast <4 x float> %609 to <2 x i64>, !dbg !10515
  %611 = and <2 x i64> %610, splat (i64 36028792732385279), !dbg !10516
  %612 = or disjoint <2 x i64> %611, splat (i64 4575657222473777152), !dbg !10521
  %613 = bitcast <2 x i64> %612 to <4 x float>, !dbg !10525
  %614 = fadd <4 x float> %613, splat (float -1.000000e+00), !dbg !10526
  %615 = fmul <4 x float> %614, splat (float 0x3F9B17A960000000), !dbg !10531
  %616 = fsub <4 x float> splat (float 0x3FBF9A8440000000), %615, !dbg !10536
  %617 = fmul <4 x float> %614, %616, !dbg !10531
  %618 = fadd <4 x float> %617, splat (float 0xBFD1E3F400000000), !dbg !10536
  %619 = fmul <4 x float> %614, %618, !dbg !10531
  %620 = fadd <4 x float> %619, splat (float 0x3FDD544F20000000), !dbg !10536
  %621 = fmul <4 x float> %614, %620, !dbg !10531
  %622 = fadd <4 x float> %621, splat (float 0xBFE6FC2A60000000), !dbg !10536
  %623 = fmul <4 x float> %614, %622, !dbg !10531
  %624 = fadd <4 x float> %623, splat (float 0x3FF714B2A0000000), !dbg !10536
  %625 = bitcast <4 x float> %609 to <4 x i32>, !dbg !10541
  %_3.i896.i.i = lshr <4 x i32> %625, splat (i32 23), !dbg !10545
  %626 = bitcast <4 x i32> %_3.i896.i.i to <2 x i64>, !dbg !10546
  %627 = or disjoint <2 x i64> %626, splat (i64 5404319554102886400), !dbg !10547
  %628 = bitcast <2 x i64> %627 to <4 x float>, !dbg !10551
  %629 = fadd <4 x float> %628, splat (float 0xC160000FE0000000), !dbg !10552
  %630 = fmul <4 x float> %614, %624, !dbg !10556
  %631 = fadd <4 x float> %629, %630, !dbg !10561
  %632 = fmul <4 x float> %631, splat (float 0x4018151820000000), !dbg !10566
  %633 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %632, <4 x float> splat (float 2.400000e+01)), !dbg !10571
  %634 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %633, <4 x float> splat (float -1.600000e+02)), !dbg !10576
  %635 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_55.i82.sroa.0.0.copyload2012.i.i, i8 1), !dbg !10581
  %636 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %threshold.i109.sroa.0.0.copyload.i.i, <4 x float> %634, i8 2), !dbg !10588
  %637 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %577, <4 x float> %634, i8 2), !dbg !10595
  %638 = bitcast <4 x float> %635 to <2 x i64>, !dbg !10602
  %639 = xor <2 x i64> %638, splat (i64 -1), !dbg !10608
  %640 = bitcast <4 x float> %636 to <2 x i64>, !dbg !10610
  %641 = and <2 x i64> %640, %639, !dbg !10614
  %642 = bitcast <4 x float> %637 to <2 x i64>, !dbg !10616
  %643 = and <2 x i64> %642, %638, !dbg !10621
  %644 = or <2 x i64> %643, %641, !dbg !10623
  %645 = xor <2 x i64> %642, splat (i64 -1), !dbg !10629
  %646 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_67.i70.sroa.0.0.copyload2014.i.i, i8 1), !dbg !10636
  %647 = bitcast <4 x float> %646 to <2 x i64>, !dbg !10642
  %648 = and <2 x i64> %645, %647, !dbg !10646
  %649 = and <2 x i64> %648, %638, !dbg !10646
  %650 = or <2 x i64> %649, %644, !dbg !10651
  %651 = bitcast <2 x i64> %650 to <4 x i32>, !dbg !10657
  %652 = icmp slt <4 x i32> %651, zeroinitializer, !dbg !10661
  %653 = select <4 x i1> %652, <4 x float> splat (float 1.000000e+00), <4 x float> zeroinitializer, !dbg !10661
  %654 = fadd <4 x float> %_67.i70.sroa.0.0.copyload2014.i.i, splat (float -1.000000e+00), !dbg !10663
  %655 = bitcast <2 x i64> %649 to <4 x i32>, !dbg !10669
  %656 = icmp slt <4 x i32> %655, zeroinitializer, !dbg !10673
  %657 = select <4 x i1> %656, <4 x float> %654, <4 x float> %_67.i70.sroa.0.0.copyload2014.i.i, !dbg !10673
  %658 = bitcast <2 x i64> %644 to <4 x i32>, !dbg !10675
  %659 = icmp slt <4 x i32> %658, zeroinitializer, !dbg !10679
  %660 = select <4 x i1> %659, <4 x float> %_71.i66.sroa.0.0.copyload.i.i, <4 x float> %657, !dbg !10679
  store <4 x float> %660, ptr %553, align 16, !dbg !10681, !alias.scope !10248, !noalias !10682
  store <4 x float> %653, ptr %552, align 16, !dbg !10689, !alias.scope !10248, !noalias !10682
  %_86.i51.sroa.0.0.copyload.i.i = load <4 x float>, ptr %555, align 16, !dbg !10690, !alias.scope !10248, !noalias !10682
  %661 = fsub <4 x float> %634, %threshold.i109.sroa.0.0.copyload.i.i, !dbg !10693
  %662 = fmul <4 x float> %578, %661, !dbg !10698
  %663 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %662, <4 x float> %580), !dbg !10703
  %664 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %663, <4 x float> zeroinitializer), !dbg !10708
  %665 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %653, i8 1), !dbg !10713
  %666 = bitcast <4 x float> %665 to <4 x i32>, !dbg !10719
  %667 = icmp slt <4 x i32> %666, zeroinitializer, !dbg !10723
  %668 = select <4 x i1> %667, <4 x float> zeroinitializer, <4 x float> %664, !dbg !10723
  %669 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_86.i51.sroa.0.0.copyload.i.i, <4 x float> %668, i8 1), !dbg !10725
  %670 = bitcast <4 x float> %669 to <4 x i32>, !dbg !10731
  %671 = icmp slt <4 x i32> %670, zeroinitializer, !dbg !10735
  %672 = select <4 x i1> %671, <4 x float> %_7.i790.i.i, <4 x float> %_88.i50.sroa.0.0.copyload.i.i, !dbg !10735
  %673 = fsub <4 x float> %668, %_86.i51.sroa.0.0.copyload.i.i, !dbg !10737
  %674 = fmul <4 x float> %673, %672, !dbg !10743
  %675 = fadd <4 x float> %_86.i51.sroa.0.0.copyload.i.i, %674, !dbg !10748
  %676 = bitcast <4 x float> %675 to <2 x i64>, !dbg !10752
  %677 = and <2 x i64> %676, splat (i64 9223372034707292159), !dbg !10759
  %678 = bitcast <2 x i64> %677 to <4 x float>, !dbg !10752
  %679 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %678, <4 x float> splat (float 0x3BC79CA100000000), i8 1), !dbg !10761
  %680 = bitcast <4 x float> %679 to <2 x i64>, !dbg !10767
  %681 = xor <2 x i64> %680, splat (i64 -1), !dbg !10772
  %682 = and <2 x i64> %676, %681, !dbg !10774
  store <2 x i64> %682, ptr %555, align 16, !dbg !10778, !alias.scope !10248, !noalias !10682
  %683 = bitcast <2 x i64> %682 to <4 x float>, !dbg !10780
  %684 = fmul <4 x float> %683, splat (float 0x3FC542A5A0000000), !dbg !10781
  %685 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %684, <4 x float> splat (float -1.260000e+02)), !dbg !10788
  %686 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %685, <4 x float> splat (float 1.270000e+02)), !dbg !10794
  %687 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %686), !dbg !10799
  %688 = fsub <4 x float> %686, %687, !dbg !10804
  %689 = fmul <4 x float> %688, splat (float 0x3F5E974FA0000000), !dbg !10809
  %690 = fadd <4 x float> %689, splat (float 0x3F82778560000000), !dbg !10814
  %691 = fmul <4 x float> %688, %690, !dbg !10809
  %692 = fadd <4 x float> %691, splat (float 0x3FAC91CE60000000), !dbg !10814
  %693 = fmul <4 x float> %688, %692, !dbg !10809
  %694 = fadd <4 x float> %693, splat (float 0x3FCEBDB560000000), !dbg !10814
  %695 = fmul <4 x float> %688, %694, !dbg !10809
  %696 = fadd <4 x float> %695, splat (float 0x3FE62E4BA0000000), !dbg !10814
  %697 = fmul <4 x float> %688, %696, !dbg !10819
  %698 = fadd <4 x float> %697, splat (float 1.000000e+00), !dbg !10824
  %699 = fadd <4 x float> %687, splat (float 0x4160000FE0000000), !dbg !10829
  %700 = bitcast <4 x float> %699 to <4 x i32>, !dbg !10834
  %_3.i897.i.i = shl <4 x i32> %700, splat (i32 23), !dbg !10838
  %701 = bitcast <4 x i32> %_3.i897.i.i to <4 x float>, !dbg !10839
  %702 = fmul <4 x float> %698, %701, !dbg !10841
  %703 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %683, <4 x float> zeroinitializer, i8 0), !dbg !10845
  %704 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_98.i40.sroa.0.0.copyload.i.i, i8 1), !dbg !10852
  %705 = bitcast <4 x float> %703 to <2 x i64>, !dbg !10858
  %706 = bitcast <4 x float> %704 to <2 x i64>, !dbg !10858
  %707 = or <2 x i64> %706, %705, !dbg !10862
  %708 = fmul <4 x float> %lanes.i525.sroa.0.0.copyload.i.i, %702, !dbg !10864
  %709 = bitcast <2 x i64> %707 to <4 x i32>, !dbg !10870
  %710 = icmp slt <4 x i32> %709, zeroinitializer, !dbg !10874
  %711 = select <4 x i1> %710, <4 x float> %lanes.i525.sroa.0.0.copyload.i.i, <4 x float> %708, !dbg !10874
  %712 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_41.i.sroa.0.0.copyload.i44.i, i8 1), !dbg !10876
  %713 = bitcast <4 x float> %712 to <4 x i32>, !dbg !10884
  %714 = icmp slt <4 x i32> %713, zeroinitializer, !dbg !10888
  %lanes.i501.sroa.0.0.vec.insert.i.i = insertelement <4 x i32> poison, i32 %_85.i1957.i.i, i64 0, !dbg !10890
  %lanes.i501.sroa.0.4.vec.insert.i.i = insertelement <4 x i32> %lanes.i501.sroa.0.0.vec.insert.i.i, i32 %_85.i.11961.i.i, i64 1, !dbg !10890
  %lanes.i501.sroa.0.8.vec.insert.i.i = insertelement <4 x i32> %lanes.i501.sroa.0.4.vec.insert.i.i, i32 %_85.i.21965.i.i, i64 2, !dbg !10890
  %lanes.i501.sroa.0.12.vec.insert.i.i = insertelement <4 x i32> %lanes.i501.sroa.0.8.vec.insert.i.i, i32 %_85.i.31969.i.i, i64 3, !dbg !10890
  %715 = bitcast <4 x i32> %lanes.i501.sroa.0.12.vec.insert.i.i to <2 x i64>, !dbg !10895
  %716 = and <2 x i64> %715, splat (i64 9223372034707292159), !dbg !10896
  %717 = bitcast <2 x i64> %716 to <4 x float>, !dbg !10902
  %718 = fmul <4 x float> %717, splat (float 5.000000e-01), !dbg !10903
  %lanes.i.sroa.0.0.vec.insert.i153.i = insertelement <4 x i32> poison, i32 %_87.i1958.i.i, i64 0, !dbg !10908
  %lanes.i.sroa.0.4.vec.insert.i154.i = insertelement <4 x i32> %lanes.i.sroa.0.0.vec.insert.i153.i, i32 %_87.i.11962.i.i, i64 1, !dbg !10908
  %lanes.i.sroa.0.8.vec.insert.i155.i = insertelement <4 x i32> %lanes.i.sroa.0.4.vec.insert.i154.i, i32 %_87.i.21966.i.i, i64 2, !dbg !10908
  %lanes.i.sroa.0.12.vec.insert.i156.i = insertelement <4 x i32> %lanes.i.sroa.0.8.vec.insert.i155.i, i32 %_87.i.31970.i.i, i64 3, !dbg !10908
  %719 = bitcast <4 x i32> %lanes.i.sroa.0.12.vec.insert.i156.i to <2 x i64>, !dbg !10913
  %720 = and <2 x i64> %719, splat (i64 9223372034707292159), !dbg !10914
  %721 = bitcast <2 x i64> %720 to <4 x float>, !dbg !10920
  %722 = fmul <4 x float> %721, splat (float 5.000000e-01), !dbg !10921
  %723 = fadd <4 x float> %718, %722, !dbg !10926
  %724 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_37.i.sroa.0.0.copyload.i45.i, i8 1), !dbg !10931
  %725 = bitcast <4 x float> %724 to <4 x i32>, !dbg !10937
  %726 = icmp slt <4 x i32> %725, zeroinitializer, !dbg !10941
  %727 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %717, <4 x float> %721), !dbg !10943
  %728 = select <4 x i1> %726, <4 x float> %727, <4 x float> %717, !dbg !10941
  %729 = select <4 x i1> %714, <4 x float> %723, <4 x float> %728, !dbg !10888
  %730 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %729, <4 x float> splat (float 0x3E45798EE0000000)), !dbg !10948
  %731 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %730, <4 x float> splat (float 0x3810000000000000)), !dbg !10953
  %732 = bitcast <4 x float> %731 to <2 x i64>, !dbg !10960
  %733 = and <2 x i64> %732, splat (i64 36028792732385279), !dbg !10961
  %734 = or disjoint <2 x i64> %733, splat (i64 4575657222473777152), !dbg !10966
  %735 = bitcast <2 x i64> %734 to <4 x float>, !dbg !10970
  %736 = fadd <4 x float> %735, splat (float -1.000000e+00), !dbg !10971
  %737 = fmul <4 x float> %736, splat (float 0x3F9B17A960000000), !dbg !10976
  %738 = fsub <4 x float> splat (float 0x3FBF9A8440000000), %737, !dbg !10981
  %739 = fmul <4 x float> %736, %738, !dbg !10976
  %740 = fadd <4 x float> %739, splat (float 0xBFD1E3F400000000), !dbg !10981
  %741 = fmul <4 x float> %736, %740, !dbg !10976
  %742 = fadd <4 x float> %741, splat (float 0x3FDD544F20000000), !dbg !10981
  %743 = fmul <4 x float> %736, %742, !dbg !10976
  %744 = fadd <4 x float> %743, splat (float 0xBFE6FC2A60000000), !dbg !10981
  %745 = fmul <4 x float> %736, %744, !dbg !10976
  %746 = fadd <4 x float> %745, splat (float 0x3FF714B2A0000000), !dbg !10981
  %747 = bitcast <4 x float> %731 to <4 x i32>, !dbg !10986
  %_3.i898.i.i = lshr <4 x i32> %747, splat (i32 23), !dbg !10990
  %748 = bitcast <4 x i32> %_3.i898.i.i to <2 x i64>, !dbg !10991
  %749 = or disjoint <2 x i64> %748, splat (i64 5404319554102886400), !dbg !10992
  %750 = bitcast <2 x i64> %749 to <4 x float>, !dbg !10996
  %751 = fadd <4 x float> %750, splat (float 0xC160000FE0000000), !dbg !10997
  %752 = fmul <4 x float> %736, %746, !dbg !11001
  %753 = fadd <4 x float> %751, %752, !dbg !11006
  %754 = fmul <4 x float> %753, splat (float 0x4018151820000000), !dbg !11011
  %755 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %754, <4 x float> splat (float 2.400000e+01)), !dbg !11016
  %756 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %755, <4 x float> splat (float -1.600000e+02)), !dbg !11021
  %757 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_55.i26.sroa.0.0.copyload2016.i.i, i8 1), !dbg !11026
  %758 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %threshold.i.sroa.0.0.copyload.i.i, <4 x float> %756, i8 2), !dbg !11032
  %759 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %581, <4 x float> %756, i8 2), !dbg !11038
  %760 = bitcast <4 x float> %757 to <2 x i64>, !dbg !11044
  %761 = xor <2 x i64> %760, splat (i64 -1), !dbg !11049
  %762 = bitcast <4 x float> %758 to <2 x i64>, !dbg !11051
  %763 = and <2 x i64> %762, %761, !dbg !11055
  %764 = bitcast <4 x float> %759 to <2 x i64>, !dbg !11057
  %765 = and <2 x i64> %764, %760, !dbg !11061
  %766 = or <2 x i64> %765, %763, !dbg !11063
  %767 = xor <2 x i64> %764, splat (i64 -1), !dbg !11068
  %768 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_67.i22.sroa.0.0.copyload2018.i.i, i8 1), !dbg !11074
  %769 = bitcast <4 x float> %768 to <2 x i64>, !dbg !11080
  %770 = and <2 x i64> %767, %769, !dbg !11084
  %771 = and <2 x i64> %770, %760, !dbg !11084
  %772 = or <2 x i64> %771, %766, !dbg !11089
  %773 = bitcast <2 x i64> %772 to <4 x i32>, !dbg !11094
  %774 = icmp slt <4 x i32> %773, zeroinitializer, !dbg !11098
  %775 = select <4 x i1> %774, <4 x float> splat (float 1.000000e+00), <4 x float> zeroinitializer, !dbg !11098
  %776 = fadd <4 x float> %_67.i22.sroa.0.0.copyload2018.i.i, splat (float -1.000000e+00), !dbg !11100
  %777 = bitcast <2 x i64> %771 to <4 x i32>, !dbg !11105
  %778 = icmp slt <4 x i32> %777, zeroinitializer, !dbg !11109
  %779 = select <4 x i1> %778, <4 x float> %776, <4 x float> %_67.i22.sroa.0.0.copyload2018.i.i, !dbg !11109
  %780 = bitcast <2 x i64> %766 to <4 x i32>, !dbg !11111
  %781 = icmp slt <4 x i32> %780, zeroinitializer, !dbg !11115
  %782 = select <4 x i1> %781, <4 x float> %_71.i20.sroa.0.0.copyload.i46.i, <4 x float> %779, !dbg !11115
  store <4 x float> %782, ptr %564, align 16, !dbg !11117, !alias.scope !10248, !noalias !11118
  store <4 x float> %775, ptr %563, align 16, !dbg !11125, !alias.scope !10248, !noalias !11118
  %_86.i11.sroa.0.0.copyload.i157.i = load <4 x float>, ptr %566, align 16, !dbg !11126, !alias.scope !10248, !noalias !11118
  %783 = fsub <4 x float> %756, %threshold.i.sroa.0.0.copyload.i.i, !dbg !11127
  %784 = fmul <4 x float> %582, %783, !dbg !11132
  %785 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %784, <4 x float> %584), !dbg !11137
  %786 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %785, <4 x float> zeroinitializer), !dbg !11142
  %787 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %775, i8 1), !dbg !11147
  %788 = bitcast <4 x float> %787 to <4 x i32>, !dbg !11153
  %789 = icmp slt <4 x i32> %788, zeroinitializer, !dbg !11157
  %790 = select <4 x i1> %789, <4 x float> zeroinitializer, <4 x float> %786, !dbg !11157
  %791 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %_86.i11.sroa.0.0.copyload.i157.i, <4 x float> %790, i8 1), !dbg !11159
  %792 = bitcast <4 x float> %791 to <4 x i32>, !dbg !11165
  %793 = icmp slt <4 x i32> %792, zeroinitializer, !dbg !11169
  %794 = select <4 x i1> %793, <4 x float> %_7.i822.i.i, <4 x float> %_88.i10.sroa.0.0.copyload.i47.i, !dbg !11169
  %795 = fsub <4 x float> %790, %_86.i11.sroa.0.0.copyload.i157.i, !dbg !11171
  %796 = fmul <4 x float> %795, %794, !dbg !11176
  %797 = fadd <4 x float> %_86.i11.sroa.0.0.copyload.i157.i, %796, !dbg !11181
  %798 = bitcast <4 x float> %797 to <2 x i64>, !dbg !11185
  %799 = and <2 x i64> %798, splat (i64 9223372034707292159), !dbg !11191
  %800 = bitcast <2 x i64> %799 to <4 x float>, !dbg !11185
  %801 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %800, <4 x float> splat (float 0x3BC79CA100000000), i8 1), !dbg !11193
  %802 = bitcast <4 x float> %801 to <2 x i64>, !dbg !11199
  %803 = xor <2 x i64> %802, splat (i64 -1), !dbg !11204
  %804 = and <2 x i64> %798, %803, !dbg !11206
  store <2 x i64> %804, ptr %566, align 16, !dbg !11210, !alias.scope !10248, !noalias !11118
  %805 = bitcast <2 x i64> %804 to <4 x float>, !dbg !11211
  %806 = fmul <4 x float> %805, splat (float 0x3FC542A5A0000000), !dbg !11212
  %807 = tail call <4 x float> @llvm.x86.sse.max.ps(<4 x float> %806, <4 x float> splat (float -1.260000e+02)), !dbg !11218
  %808 = tail call <4 x float> @llvm.x86.sse.min.ps(<4 x float> %807, <4 x float> splat (float 1.270000e+02)), !dbg !11224
  %809 = tail call <4 x float> @llvm.floor.v4f32(<4 x float> %808), !dbg !11229
  %810 = fsub <4 x float> %808, %809, !dbg !11234
  %811 = fmul <4 x float> %810, splat (float 0x3F5E974FA0000000), !dbg !11239
  %812 = fadd <4 x float> %811, splat (float 0x3F82778560000000), !dbg !11244
  %813 = fmul <4 x float> %810, %812, !dbg !11239
  %814 = fadd <4 x float> %813, splat (float 0x3FAC91CE60000000), !dbg !11244
  %815 = fmul <4 x float> %810, %814, !dbg !11239
  %816 = fadd <4 x float> %815, splat (float 0x3FCEBDB560000000), !dbg !11244
  %817 = fmul <4 x float> %810, %816, !dbg !11239
  %818 = fadd <4 x float> %817, splat (float 0x3FE62E4BA0000000), !dbg !11244
  %819 = fmul <4 x float> %810, %818, !dbg !11249
  %820 = fadd <4 x float> %819, splat (float 1.000000e+00), !dbg !11254
  %821 = fadd <4 x float> %809, splat (float 0x4160000FE0000000), !dbg !11259
  %822 = bitcast <4 x float> %821 to <4 x i32>, !dbg !11264
  %_3.i899.i.i = shl <4 x i32> %822, splat (i32 23), !dbg !11268
  %823 = bitcast <4 x i32> %_3.i899.i.i to <4 x float>, !dbg !11269
  %824 = fmul <4 x float> %820, %823, !dbg !11271
  %825 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> %805, <4 x float> zeroinitializer, i8 0), !dbg !11275
  %826 = tail call <4 x float> @llvm.x86.sse.cmp.ps(<4 x float> zeroinitializer, <4 x float> %_98.i.sroa.0.0.copyload.i48.i, i8 1), !dbg !11281
  %827 = bitcast <4 x float> %825 to <2 x i64>, !dbg !11287
  %828 = bitcast <4 x float> %826 to <2 x i64>, !dbg !11287
  %829 = or <2 x i64> %828, %827, !dbg !11291
  %830 = fmul <4 x float> %lanes.i519.sroa.0.0.copyload.i.i, %824, !dbg !11293
  %831 = bitcast <2 x i64> %829 to <4 x i32>, !dbg !11298
  %832 = icmp slt <4 x i32> %831, zeroinitializer, !dbg !11302
  %833 = select <4 x i1> %832, <4 x float> %lanes.i519.sroa.0.0.copyload.i.i, <4 x float> %830, !dbg !11302
  store <4 x float> %711, ptr %_123.i.i58.i, align 4, !dbg !11304, !alias.scope !11310, !noalias !11314
  store <4 x float> %833, ptr %_141.i.i64.i, align 4, !dbg !11318, !alias.scope !11323, !noalias !11327
  %exitcond1749.not.i.i = icmp eq i64 %585, %_26.i, !dbg !11331
  br i1 %exitcond1749.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKB1r_EB3_.exit.i, label %bb40.i.i49.i, !dbg !10270

panic18.i.i88.i:                                  ; preds = %bb28.i.2.i135.i, %bb28.i.1.i117.i, %bb28.i.i99.i, %bb63.i.i77.i
  %own.i.lcssa.i89.i = phi i64 [ %_63.i.i82.i, %bb63.i.i77.i ], [ %own.i.1.i104.i, %bb28.i.i99.i ], [ %own.i.2.i122.i, %bb28.i.1.i117.i ], [ %own.i.3.i140.i, %bb28.i.2.i135.i ], !dbg !10411
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %own.i.lcssa.i89.i, i64 noundef %_58.1.i23.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b53d553f9945f7702333f7af20204159) #24, !dbg !10417, !noalias !10305
  unreachable, !dbg !10417

bb22.i.i90.i:                                     ; preds = %bb63.i.i77.i
  %834 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %_63.i.i82.i, !dbg !10417
  %_78.i1955.i.i = load i32, ptr %834, align 4, !dbg !10417, !noalias !10305, !noundef !12
  %_84.i.i91.i = icmp ult i64 %_63.i.i82.i, %_60.1.i26.i, !dbg !11334
  br i1 %_84.i.i91.i, label %bb24.i.i93.i, label %panic20.i.i92.i, !dbg !11334

panic20.i.i92.i:                                  ; preds = %bb22.i.3.i147.i, %bb22.i.2.i129.i, %bb22.i.1.i111.i, %bb22.i.i90.i
  %own.i.lcssa1658.i.i = phi i64 [ %_63.i.i82.i, %bb22.i.i90.i ], [ %own.i.1.i104.i, %bb22.i.1.i111.i ], [ %own.i.2.i122.i, %bb22.i.2.i129.i ], [ %own.i.3.i140.i, %bb22.i.3.i147.i ], !dbg !10411
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %own.i.lcssa1658.i.i, i64 noundef %_60.1.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30bf8a301493a607abcc78dbf93977e0) #24, !dbg !11334, !noalias !10305
  unreachable, !dbg !11334

bb24.i.i93.i:                                     ; preds = %bb22.i.i90.i
  %835 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %_63.i.i82.i, !dbg !11334
  %_82.i1956.i.i = load i32, ptr %835, align 4, !dbg !11334, !noalias !10305, !noundef !12
  %_86.i.i94.i = icmp ult i64 %_71.i.i86.i, %_60.1.i26.i, !dbg !10419
  br i1 %_86.i.i94.i, label %bb26.i.i96.i, label %panic22.i.i95.i, !dbg !10419

panic22.i.i95.i:                                  ; preds = %bb24.i.3.i149.i, %bb24.i.2.i131.i, %bb24.i.1.i113.i, %bb24.i.i93.i
  %partner.i.lcssa1655.i.i = phi i64 [ %_71.i.i86.i, %bb24.i.i93.i ], [ %partner.i.1.i109.i, %bb24.i.1.i113.i ], [ %partner.i.2.i127.i, %bb24.i.2.i131.i ], [ %partner.i.3.i145.i, %bb24.i.3.i149.i ], !dbg !10416
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %partner.i.lcssa1655.i.i, i64 noundef %_60.1.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5833dd3f10f38ea96ec24c6054ff083c) #24, !dbg !10419, !noalias !10305
  unreachable, !dbg !10419

bb26.i.i96.i:                                     ; preds = %bb24.i.i93.i
  %836 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %_71.i.i86.i, !dbg !10419
  %_85.i1957.i.i = load i32, ptr %836, align 4, !dbg !10419, !noalias !10305, !noundef !12
  %_88.i.i97.i = icmp ult i64 %_71.i.i86.i, %_58.1.i23.i, !dbg !10420
  br i1 %_88.i.i97.i, label %bb28.i.i99.i, label %panic24.i.i98.i, !dbg !10420

panic24.i.i98.i:                                  ; preds = %bb26.i.3.i151.i, %bb26.i.2.i133.i, %bb26.i.1.i115.i, %bb26.i.i96.i
  %partner.i.lcssa1656.i.i = phi i64 [ %_71.i.i86.i, %bb26.i.i96.i ], [ %partner.i.1.i109.i, %bb26.i.1.i115.i ], [ %partner.i.2.i127.i, %bb26.i.2.i133.i ], [ %partner.i.3.i145.i, %bb26.i.3.i151.i ], !dbg !10416
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %partner.i.lcssa1656.i.i, i64 noundef %_58.1.i23.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5ae5decb60097d7e1183cb43cae6b3f) #24, !dbg !10420, !noalias !10305
  unreachable, !dbg !10420

bb28.i.i99.i:                                     ; preds = %bb26.i.i96.i
  %837 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %_71.i.i86.i, !dbg !10420
  %_87.i1958.i.i = load i32, ptr %837, align 4, !dbg !10420, !noalias !10305, !noundef !12
  %_66.i.1.i100.i = sub i32 %now.i.i52.i, %_67.i.1.i38.i, !dbg !10403
  %_65.i.1.i101.i = and i32 %_66.i.1.i100.i, %_52.i29.i, !dbg !10410
  %_64.i.1.i102.i = zext i32 %_65.i.1.i101.i to i64, !dbg !10411
  %_63.i.1.i103.i = shl nuw nsw i64 %_64.i.1.i102.i, 2, !dbg !10411
  %own.i.1.i104.i = or disjoint i64 %_63.i.1.i103.i, 1, !dbg !10411
  %_74.i.1.i105.i = sub i32 %now.i.i52.i, %_75.i.1.i39.i, !dbg !10412
  %_73.i.1.i106.i = and i32 %_74.i.1.i105.i, %_52.i29.i, !dbg !10415
  %_72.i.1.i107.i = zext i32 %_73.i.1.i106.i to i64, !dbg !10416
  %_71.i.1.i108.i = shl nuw nsw i64 %_72.i.1.i107.i, 2, !dbg !10416
  %partner.i.1.i109.i = or disjoint i64 %_71.i.1.i108.i, 1, !dbg !10416
  %_80.i.1.i110.i = icmp ult i64 %own.i.1.i104.i, %_58.1.i23.i, !dbg !10417
  br i1 %_80.i.1.i110.i, label %bb22.i.1.i111.i, label %panic18.i.i88.i, !dbg !10417

bb22.i.1.i111.i:                                  ; preds = %bb28.i.i99.i
  %838 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.1.i104.i, !dbg !10417
  %_78.i.11959.i.i = load i32, ptr %838, align 4, !dbg !10417, !noalias !10305, !noundef !12
  %_84.i.1.i112.i = icmp ult i64 %own.i.1.i104.i, %_60.1.i26.i, !dbg !11334
  br i1 %_84.i.1.i112.i, label %bb24.i.1.i113.i, label %panic20.i.i92.i, !dbg !11334

bb24.i.1.i113.i:                                  ; preds = %bb22.i.1.i111.i
  %839 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.1.i104.i, !dbg !11334
  %_82.i.11960.i.i = load i32, ptr %839, align 4, !dbg !11334, !noalias !10305, !noundef !12
  %_86.i.1.i114.i = icmp ult i64 %partner.i.1.i109.i, %_60.1.i26.i, !dbg !10419
  br i1 %_86.i.1.i114.i, label %bb26.i.1.i115.i, label %panic22.i.i95.i, !dbg !10419

bb26.i.1.i115.i:                                  ; preds = %bb24.i.1.i113.i
  %840 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.1.i109.i, !dbg !10419
  %_85.i.11961.i.i = load i32, ptr %840, align 4, !dbg !10419, !noalias !10305, !noundef !12
  %_88.i.1.i116.i = icmp ult i64 %partner.i.1.i109.i, %_58.1.i23.i, !dbg !10420
  br i1 %_88.i.1.i116.i, label %bb28.i.1.i117.i, label %panic24.i.i98.i, !dbg !10420

bb28.i.1.i117.i:                                  ; preds = %bb26.i.1.i115.i
  %841 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.1.i109.i, !dbg !10420
  %_87.i.11962.i.i = load i32, ptr %841, align 4, !dbg !10420, !noalias !10305, !noundef !12
  %_66.i.2.i118.i = sub i32 %now.i.i52.i, %_67.i.2.i40.i, !dbg !10403
  %_65.i.2.i119.i = and i32 %_66.i.2.i118.i, %_52.i29.i, !dbg !10410
  %_64.i.2.i120.i = zext i32 %_65.i.2.i119.i to i64, !dbg !10411
  %_63.i.2.i121.i = shl nuw nsw i64 %_64.i.2.i120.i, 2, !dbg !10411
  %own.i.2.i122.i = or disjoint i64 %_63.i.2.i121.i, 2, !dbg !10411
  %_74.i.2.i123.i = sub i32 %now.i.i52.i, %_75.i.2.i41.i, !dbg !10412
  %_73.i.2.i124.i = and i32 %_74.i.2.i123.i, %_52.i29.i, !dbg !10415
  %_72.i.2.i125.i = zext i32 %_73.i.2.i124.i to i64, !dbg !10416
  %_71.i.2.i126.i = shl nuw nsw i64 %_72.i.2.i125.i, 2, !dbg !10416
  %partner.i.2.i127.i = or disjoint i64 %_71.i.2.i126.i, 2, !dbg !10416
  %_80.i.2.i128.i = icmp ult i64 %own.i.2.i122.i, %_58.1.i23.i, !dbg !10417
  br i1 %_80.i.2.i128.i, label %bb22.i.2.i129.i, label %panic18.i.i88.i, !dbg !10417

bb22.i.2.i129.i:                                  ; preds = %bb28.i.1.i117.i
  %842 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.2.i122.i, !dbg !10417
  %_78.i.21963.i.i = load i32, ptr %842, align 4, !dbg !10417, !noalias !10305, !noundef !12
  %_84.i.2.i130.i = icmp ult i64 %own.i.2.i122.i, %_60.1.i26.i, !dbg !11334
  br i1 %_84.i.2.i130.i, label %bb24.i.2.i131.i, label %panic20.i.i92.i, !dbg !11334

bb24.i.2.i131.i:                                  ; preds = %bb22.i.2.i129.i
  %843 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.2.i122.i, !dbg !11334
  %_82.i.21964.i.i = load i32, ptr %843, align 4, !dbg !11334, !noalias !10305, !noundef !12
  %_86.i.2.i132.i = icmp ult i64 %partner.i.2.i127.i, %_60.1.i26.i, !dbg !10419
  br i1 %_86.i.2.i132.i, label %bb26.i.2.i133.i, label %panic22.i.i95.i, !dbg !10419

bb26.i.2.i133.i:                                  ; preds = %bb24.i.2.i131.i
  %844 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.2.i127.i, !dbg !10419
  %_85.i.21965.i.i = load i32, ptr %844, align 4, !dbg !10419, !noalias !10305, !noundef !12
  %_88.i.2.i134.i = icmp ult i64 %partner.i.2.i127.i, %_58.1.i23.i, !dbg !10420
  br i1 %_88.i.2.i134.i, label %bb28.i.2.i135.i, label %panic24.i.i98.i, !dbg !10420

bb28.i.2.i135.i:                                  ; preds = %bb26.i.2.i133.i
  %845 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.2.i127.i, !dbg !10420
  %_87.i.21966.i.i = load i32, ptr %845, align 4, !dbg !10420, !noalias !10305, !noundef !12
  %_66.i.3.i136.i = sub i32 %now.i.i52.i, %_67.i.3.i42.i, !dbg !10403
  %_65.i.3.i137.i = and i32 %_66.i.3.i136.i, %_52.i29.i, !dbg !10410
  %_64.i.3.i138.i = zext i32 %_65.i.3.i137.i to i64, !dbg !10411
  %_63.i.3.i139.i = shl nuw nsw i64 %_64.i.3.i138.i, 2, !dbg !10411
  %own.i.3.i140.i = or disjoint i64 %_63.i.3.i139.i, 3, !dbg !10411
  %_74.i.3.i141.i = sub i32 %now.i.i52.i, %_75.i.3.i43.i, !dbg !10412
  %_73.i.3.i142.i = and i32 %_74.i.3.i141.i, %_52.i29.i, !dbg !10415
  %_72.i.3.i143.i = zext i32 %_73.i.3.i142.i to i64, !dbg !10416
  %_71.i.3.i144.i = shl nuw nsw i64 %_72.i.3.i143.i, 2, !dbg !10416
  %partner.i.3.i145.i = or disjoint i64 %_71.i.3.i144.i, 3, !dbg !10416
  %_80.i.3.i146.i = icmp ult i64 %own.i.3.i140.i, %_58.1.i23.i, !dbg !10417
  br i1 %_80.i.3.i146.i, label %bb22.i.3.i147.i, label %panic18.i.i88.i, !dbg !10417

bb22.i.3.i147.i:                                  ; preds = %bb28.i.2.i135.i
  %846 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.3.i140.i, !dbg !10417
  %_78.i.31967.i.i = load i32, ptr %846, align 4, !dbg !10417, !noalias !10305, !noundef !12
  %_84.i.3.i148.i = icmp ult i64 %own.i.3.i140.i, %_60.1.i26.i, !dbg !11334
  br i1 %_84.i.3.i148.i, label %bb24.i.3.i149.i, label %panic20.i.i92.i, !dbg !11334

bb24.i.3.i149.i:                                  ; preds = %bb22.i.3.i147.i
  %847 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.3.i140.i, !dbg !11334
  %_82.i.31968.i.i = load i32, ptr %847, align 4, !dbg !11334, !noalias !10305, !noundef !12
  %_86.i.3.i150.i = icmp ult i64 %partner.i.3.i145.i, %_60.1.i26.i, !dbg !10419
  br i1 %_86.i.3.i150.i, label %bb26.i.3.i151.i, label %panic22.i.i95.i, !dbg !10419

bb26.i.3.i151.i:                                  ; preds = %bb24.i.3.i149.i
  %_88.i.3.i152.i = icmp ult i64 %partner.i.3.i145.i, %_58.1.i23.i, !dbg !10420
  br i1 %_88.i.3.i152.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i, label %panic24.i.i98.i, !dbg !10420

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E11run_segmentKB1r_EB3_.exit.i: ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4ECsdOTRa1MFkeb_13gate_expander.exit133.i.i
  %_108.i.i158.i = trunc i64 %_26.i to i32, !dbg !11335
  %_107.i.i159.i = add i32 %base.i.i35.i, %_108.i.i158.i, !dbg !11336
  store i32 %_107.i.i159.i, ptr %_51.i28.i, align 4, !dbg !11338, !alias.scope !10248, !noalias !10267
  br label %bb7.i, !dbg !11339

bb26.i:                                           ; preds = %bb25.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %541, i64 noundef range(i64 0, 2305843009213693952) %_38.1, i64 noundef range(i64 0, 2305843009213693952) %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9c634077742a23e4340a846355ecc484) #24, !dbg !11340, !noalias !8226
  unreachable, !dbg !11340

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9run_blockB2_.exit: ; preds = %bb1.backedge.i.i
  call void @llvm.lifetime.end.p0(ptr nonnull %iter.i.i), !dbg !11341, !noalias !9728
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(160) %report.sroa.0, ptr noundef nonnull align 8 dereferenceable(160) %reports, i64 160, i1 false), !dbg !11342, !alias.scope !11349, !noalias !11353
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %_0, ptr noundef nonnull align 8 dereferenceable(320) %report.sroa.0, i64 320, i1 false), !dbg !11355
  %report.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 320, !dbg !11355
  store i8 %1, ptr %report.sroa.7.0._0.sroa_idx, align 8, !dbg !11355
  call void @llvm.lifetime.end.p0(ptr nonnull %reports), !dbg !11356
  br label %bb12, !dbg !8197

bb15:                                             ; preds = %bb4, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit
  %iter.sroa.0.0131 = phi i64 [ 0, %bb4 ], [ %848, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit ]
  %848 = add nuw nsw i64 %iter.sroa.0.0131, 1, !dbg !11357
  %exitcond.not = icmp eq i64 %iter.sroa.0.0131, %_36.1, !dbg !11363
  br i1 %exitcond.not, label %panic, label %bb7, !dbg !11363

bb12:                                             ; preds = %bb3, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E9run_blockB2_.exit
  call void @llvm.lifetime.end.p0(ptr nonnull %report.sroa.0), !dbg !11365
  ret void, !dbg !8197

bb7:                                              ; preds = %bb15
  %849 = getelementptr inbounds nuw i32, ptr %_36.0, i64 %iter.sroa.0.0131, !dbg !11363
  %_14 = load i32, ptr %849, align 4, !dbg !11363, !noundef !12
  %start1 = zext i32 %_14 to i64, !dbg !11363
  %exitcond237.not = icmp eq i64 %iter.sroa.0.0131, %15, !dbg !11366
  br i1 %exitcond237.not, label %panic2, label %bb8, !dbg !11366

panic:                                            ; preds = %bb15
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_36.1, i64 noundef %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d8ffa1b111d58613892f54938d1219d1) #24, !dbg !11363
  unreachable, !dbg !11363

bb8:                                              ; preds = %bb7
  %850 = getelementptr inbounds nuw i32, ptr %_36.0, i64 %848, !dbg !11366
  %_18 = load i32, ptr %850, align 4, !dbg !11366, !noundef !12
  %end = zext i32 %_18 to i64, !dbg !11366
  %_58 = icmp ult i32 %_18, %_14, !dbg !11368
  %_52.not = icmp ult i64 %_39.1, %end
  %or.cond24 = or i1 %_58, %_52.not, !dbg !11368
  br i1 %or.cond24, label %bb19, label %bb9, !dbg !11368, !prof !2561

panic2:                                           ; preds = %bb7
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %848, i64 noundef %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d8ffa1b111d58613892f54938d1219d1) #24, !dbg !11366
  unreachable, !dbg !11366

bb19:                                             ; preds = %bb8
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1, i64 noundef %end, i64 noundef %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d8ffa1b111d58613892f54938d1219d1) #24, !dbg !11376
  unreachable, !dbg !11376

bb9:                                              ; preds = %bb8
  %_59 = sub nuw nsw i64 %end, %start1, !dbg !11377
  %_61 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_39.0, i64 %start1, !dbg !11378
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11382), !dbg !11385
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11386), !dbg !11385
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11388), !dbg !11385
  call void @llvm.lifetime.start.p0(ptr nonnull %pending.i), !dbg !11390, !noalias !11393
  store i32 0, ptr %pending.i, align 4, !noalias !11393
  store i32 0, ptr %_7.sroa.5128.0.pending.sroa_idx.i, align 4, !noalias !11393
  store i32 0, ptr %_7.sroa.6131.0.pending.sroa_idx.i, align 4, !noalias !11393
  store i32 0, ptr %_7.sroa.7134.0.pending.sroa_idx.i, align 4, !noalias !11393
  store i32 0, ptr %11, align 4, !noalias !11393
  store i32 0, ptr %_7.sroa.5128.0..sroa_idx.i, align 4, !noalias !11393
  store i32 0, ptr %_7.sroa.6131.0..sroa_idx.i, align 4, !noalias !11393
  store i32 0, ptr %_7.sroa.7134.0..sroa_idx.i, align 4, !noalias !11393
  %_106.idx.i = mul nuw nsw i64 %_59, 40, !dbg !11394
  %_106.i = getelementptr inbounds nuw i8, ptr %_61, i64 %_106.idx.i, !dbg !11394
  %_6.i.i9194.i = icmp eq i32 %_18, %_14, !dbg !11405
  br i1 %_6.i.i9194.i, label %bb36.preheader.i, label %bb4.lr.ph.lr.ph.i, !dbg !11410

bb4.lr.ph.lr.ph.i:                                ; preds = %bb9
  %_24 = getelementptr inbounds nuw %"effect_contract::ProcessReport", ptr %reports, i64 %iter.sroa.0.0131, !dbg !11411
  %851 = getelementptr inbounds nuw i8, ptr %_24, i64 16
  %_31.i = load i32, ptr %12, align 4, !alias.scope !11382, !noalias !11412
  %_30.i = zext i32 %_31.i to i64
  %.promoted99.i = load i64, ptr %851, align 8, !alias.scope !11388, !noalias !11413
  br label %bb4.lr.ph.i, !dbg !11410

bb4.lr.ph.i:                                      ; preds = %bb31.i, %bb4.lr.ph.lr.ph.i
  %.promoted101.i = phi i64 [ %.promoted99.i, %bb4.lr.ph.lr.ph.i ], [ %.promoted100.i, %bb31.i ]
  %last_order.sroa.3.0.ph98.i = phi i32 [ undef, %bb4.lr.ph.lr.ph.i ], [ %_127.0.i, %bb31.i ]
  %last_order.sroa.0.0.ph97.not.i = phi i1 [ true, %bb4.lr.ph.lr.ph.i ], [ false, %bb31.i ]
  %iter.sroa.0.0.ph96.i = phi ptr [ %_61, %bb4.lr.ph.lr.ph.i ], [ %_16.i.i.i, %bb31.i ]
  %iter.sroa.7.0.ph95.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %bb4.i10, !dbg !11410

bb36.preheader.i:                                 ; preds = %bb31.i, %bb35.i, %bb9
  %852 = getelementptr inbounds nuw float, ptr %words.i.i, i64 %iter.sroa.0.0131
  %853 = getelementptr inbounds nuw float, ptr %words.i66.i, i64 %iter.sroa.0.0131
  %854 = getelementptr inbounds nuw float, ptr %words.i67.i, i64 %iter.sroa.0.0131
  %855 = getelementptr inbounds nuw float, ptr %words.i68.i, i64 %iter.sroa.0.0131
  %856 = getelementptr inbounds nuw float, ptr %words.i69.i, i64 %iter.sroa.0.0131
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, !dbg !11414

bb4.i10:                                          ; preds = %bb35.i, %bb4.lr.ph.i
  %.promoted100.i = phi i64 [ %.promoted101.i, %bb4.lr.ph.i ], [ %903, %bb35.i ]
  %iter.sroa.0.093.i = phi ptr [ %iter.sroa.0.0.ph96.i, %bb4.lr.ph.i ], [ %_16.i.i.i, %bb35.i ]
  %iter.sroa.7.092.i = phi i64 [ %iter.sroa.7.0.ph95.i, %bb4.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 40, !dbg !11422
  %_9.0.i.i = add i64 %iter.sroa.7.092.i, 1, !dbg !11424
  %857 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 32, !dbg !11425
  %_17.i = load i32, ptr %857, align 8, !dbg !11425, !range !1335, !alias.scope !11386, !noalias !11427, !noundef !12
  switch i32 %_17.i, label %default.unreachable [
    i32 1, label %bb9.i12
    i32 2, label %bb7.i11
    i32 3, label %bb35.i
  ], !dbg !11428

bb36.loopexit.i:                                  ; preds = %bb46.us.3.i, %bb40.backedge.us.2.i
  %_6.i.i44.i = icmp eq i64 %iter1.sroa.0.0.add.i, 64, !dbg !11429
  br i1 %_6.i.i44.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, !dbg !11414

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i: ; preds = %bb36.loopexit.i, %bb36.preheader.i
  %iter1.sroa.0.0.idx111.i = phi i64 [ 0, %bb36.preheader.i ], [ %iter1.sroa.0.0.add.i, %bb36.loopexit.i ]
  %iter1.sroa.7.0110.i = phi i64 [ 0, %bb36.preheader.i ], [ %_9.0.i48.i, %bb36.loopexit.i ]
  %iter1.sroa.0.0.ptr.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 %iter1.sroa.0.0.idx111.i, !dbg !11429
  %iter1.sroa.0.0.add.i = add nuw nsw i64 %iter1.sroa.0.0.idx111.i, 32, !dbg !11432
  %_9.0.i48.i = add nuw nsw i64 %iter1.sroa.7.0110.i, 1, !dbg !11435
  %858 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x4_::f32x4>", ptr %13, i64 %iter1.sroa.7.0110.i
  %859 = load i32, ptr %iter1.sroa.0.0.ptr.i, align 4, !dbg !11438, !range !5716, !noalias !11393, !noundef !12
  %860 = trunc nuw i32 %859 to i1, !dbg !11442
  br i1 %860, label %bb46.us.i, label %bb40.backedge.us.i, !dbg !11442

bb46.us.i:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %861 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 4, !dbg !11438
  %value.us.i = load float, ptr %861, align 4, !dbg !11443, !noalias !11393, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !11444, !noalias !11448
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %words.i.i, ptr noundef nonnull align 16 dereferenceable(16) %858, i64 16, i1 false), !dbg !11451, !noalias !11412
  %_0.i.us.i = load float, ptr %852, align 4, !dbg !11456, !noalias !11448, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !11457, !noalias !11448
  %862 = getelementptr inbounds nuw i8, ptr %858, i64 16, !dbg !11458
  %863 = getelementptr inbounds nuw i8, ptr %858, i64 32, !dbg !11459
  %864 = getelementptr inbounds nuw i8, ptr %858, i64 48, !dbg !11460
  %_148.us.i = bitcast float %_0.i.us.i to i32, !dbg !11461
  %_150.us.i = bitcast float %value.us.i to i32, !dbg !11469
  %_149.us.i = icmp ne i32 %_148.us.i, %_150.us.i, !dbg !11472
  %865 = icmp eq i32 %_148.us.i, -2147483648
  %or.cond.us.i = or i1 %_149.us.i, %865, !dbg !11472
  %866 = tail call float @llvm.fabs.f32(float %_0.i.us.i)
  %_144.us.i = fcmp ueq float %866, 0x7FF0000000000000
  %or.cond40.us.i = or i1 %_144.us.i, %or.cond.us.i, !dbg !11472
  %_146.us.i = fsub float %value.us.i, %_0.i.us.i, !dbg !11472
  %867 = fmul float %_146.us.i, 1.562500e-02, !dbg !11472
  %ramp.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %_0.i.us.i, float %value.us.i, !dbg !11472
  %ramp4.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %867, float 0.000000e+00, !dbg !11472
  %ramp5.sroa.0.0.us.i = select i1 %or.cond40.us.i, float 6.400000e+01, float 0.000000e+00, !dbg !11472
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i66.i), !dbg !11473, !noalias !11475
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, ptr noundef nonnull align 16 dereferenceable(16) %858, i64 16, i1 false), !dbg !11478, !noalias !11412
  store float %ramp.sroa.0.0.us.i, ptr %853, align 4, !dbg !11479, !noalias !11475
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %858, ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, i64 16, i1 false), !dbg !11480, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i66.i), !dbg !11481, !noalias !11475
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i67.i), !dbg !11482, !noalias !11484
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, ptr noundef nonnull align 16 dereferenceable(16) %862, i64 16, i1 false), !dbg !11487, !noalias !11412
  store float %value.us.i, ptr %854, align 4, !dbg !11488, !noalias !11484
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %862, ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, i64 16, i1 false), !dbg !11489, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i67.i), !dbg !11490, !noalias !11484
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i68.i), !dbg !11491, !noalias !11493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, ptr noundef nonnull align 16 dereferenceable(16) %863, i64 16, i1 false), !dbg !11496, !noalias !11412
  store float %ramp4.sroa.0.0.us.i, ptr %855, align 4, !dbg !11497, !noalias !11493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %863, ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, i64 16, i1 false), !dbg !11498, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i68.i), !dbg !11499, !noalias !11493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i69.i), !dbg !11500, !noalias !11502
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, ptr noundef nonnull align 16 dereferenceable(16) %864, i64 16, i1 false), !dbg !11505, !noalias !11412
  store float %ramp5.sroa.0.0.us.i, ptr %856, align 4, !dbg !11506, !noalias !11502
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %864, ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, i64 16, i1 false), !dbg !11507, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i69.i), !dbg !11508, !noalias !11502
  store i32 64, ptr %14, align 4, !dbg !11509, !alias.scope !11382, !noalias !11412
  br label %bb40.backedge.us.i, !dbg !11510

bb40.backedge.us.i:                               ; preds = %bb46.us.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %iter2.sroa.0.0.ptr106.us.1.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 8, !dbg !11511
  %868 = load i32, ptr %iter2.sroa.0.0.ptr106.us.1.i, align 4, !dbg !11438, !range !5716, !noalias !11393, !noundef !12
  %869 = trunc nuw i32 %868 to i1, !dbg !11442
  br i1 %869, label %bb46.us.1.i, label %bb40.backedge.us.1.i, !dbg !11442

bb46.us.1.i:                                      ; preds = %bb40.backedge.us.i
  %870 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 12, !dbg !11438
  %value.us.1.i = load float, ptr %870, align 4, !dbg !11443, !noalias !11393, !noundef !12
  %slot.us.1.i = getelementptr inbounds nuw i8, ptr %858, i64 64, !dbg !11520
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !11444, !noalias !11448
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %words.i.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.1.i, i64 16, i1 false), !dbg !11451, !noalias !11412
  %_0.i.us.1.i = load float, ptr %852, align 4, !dbg !11456, !noalias !11448, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !11457, !noalias !11448
  %871 = getelementptr inbounds nuw i8, ptr %858, i64 80, !dbg !11458
  %872 = getelementptr inbounds nuw i8, ptr %858, i64 96, !dbg !11459
  %873 = getelementptr inbounds nuw i8, ptr %858, i64 112, !dbg !11460
  %_148.us.1.i = bitcast float %_0.i.us.1.i to i32, !dbg !11461
  %_150.us.1.i = bitcast float %value.us.1.i to i32, !dbg !11469
  %_149.us.1.i = icmp ne i32 %_148.us.1.i, %_150.us.1.i, !dbg !11472
  %874 = icmp eq i32 %_148.us.1.i, -2147483648
  %or.cond.us.1.i = or i1 %_149.us.1.i, %874, !dbg !11472
  %875 = tail call float @llvm.fabs.f32(float %_0.i.us.1.i)
  %_144.us.1.i = fcmp ueq float %875, 0x7FF0000000000000
  %or.cond40.us.1.i = or i1 %_144.us.1.i, %or.cond.us.1.i, !dbg !11472
  %_146.us.1.i = fsub float %value.us.1.i, %_0.i.us.1.i, !dbg !11472
  %876 = fmul float %_146.us.1.i, 1.562500e-02, !dbg !11472
  %ramp.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %_0.i.us.1.i, float %value.us.1.i, !dbg !11472
  %ramp4.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %876, float 0.000000e+00, !dbg !11472
  %ramp5.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !11472
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i66.i), !dbg !11473, !noalias !11475
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.1.i, i64 16, i1 false), !dbg !11478, !noalias !11412
  store float %ramp.sroa.0.0.us.1.i, ptr %853, align 4, !dbg !11479, !noalias !11475
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %slot.us.1.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, i64 16, i1 false), !dbg !11480, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i66.i), !dbg !11481, !noalias !11475
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i67.i), !dbg !11482, !noalias !11484
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, ptr noundef nonnull align 16 dereferenceable(16) %871, i64 16, i1 false), !dbg !11487, !noalias !11412
  store float %value.us.1.i, ptr %854, align 4, !dbg !11488, !noalias !11484
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %871, ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, i64 16, i1 false), !dbg !11489, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i67.i), !dbg !11490, !noalias !11484
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i68.i), !dbg !11491, !noalias !11493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, ptr noundef nonnull align 16 dereferenceable(16) %872, i64 16, i1 false), !dbg !11496, !noalias !11412
  store float %ramp4.sroa.0.0.us.1.i, ptr %855, align 4, !dbg !11497, !noalias !11493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %872, ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, i64 16, i1 false), !dbg !11498, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i68.i), !dbg !11499, !noalias !11493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i69.i), !dbg !11500, !noalias !11502
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, ptr noundef nonnull align 16 dereferenceable(16) %873, i64 16, i1 false), !dbg !11505, !noalias !11412
  store float %ramp5.sroa.0.0.us.1.i, ptr %856, align 4, !dbg !11506, !noalias !11502
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %873, ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, i64 16, i1 false), !dbg !11507, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i69.i), !dbg !11508, !noalias !11502
  store i32 64, ptr %14, align 4, !dbg !11509, !alias.scope !11382, !noalias !11412
  br label %bb40.backedge.us.1.i, !dbg !11510

bb40.backedge.us.1.i:                             ; preds = %bb46.us.1.i, %bb40.backedge.us.i
  %iter2.sroa.0.0.ptr106.us.2.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 16, !dbg !11511
  %877 = load i32, ptr %iter2.sroa.0.0.ptr106.us.2.i, align 4, !dbg !11438, !range !5716, !noalias !11393, !noundef !12
  %878 = trunc nuw i32 %877 to i1, !dbg !11442
  br i1 %878, label %bb46.us.2.i, label %bb40.backedge.us.2.i, !dbg !11442

bb46.us.2.i:                                      ; preds = %bb40.backedge.us.1.i
  %879 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 20, !dbg !11438
  %value.us.2.i = load float, ptr %879, align 4, !dbg !11443, !noalias !11393, !noundef !12
  %slot.us.2.i = getelementptr inbounds nuw i8, ptr %858, i64 128, !dbg !11520
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !11444, !noalias !11448
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %words.i.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.2.i, i64 16, i1 false), !dbg !11451, !noalias !11412
  %_0.i.us.2.i = load float, ptr %852, align 4, !dbg !11456, !noalias !11448, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !11457, !noalias !11448
  %880 = getelementptr inbounds nuw i8, ptr %858, i64 144, !dbg !11458
  %881 = getelementptr inbounds nuw i8, ptr %858, i64 160, !dbg !11459
  %882 = getelementptr inbounds nuw i8, ptr %858, i64 176, !dbg !11460
  %_148.us.2.i = bitcast float %_0.i.us.2.i to i32, !dbg !11461
  %_150.us.2.i = bitcast float %value.us.2.i to i32, !dbg !11469
  %_149.us.2.i = icmp ne i32 %_148.us.2.i, %_150.us.2.i, !dbg !11472
  %883 = icmp eq i32 %_148.us.2.i, -2147483648
  %or.cond.us.2.i = or i1 %_149.us.2.i, %883, !dbg !11472
  %884 = tail call float @llvm.fabs.f32(float %_0.i.us.2.i)
  %_144.us.2.i = fcmp ueq float %884, 0x7FF0000000000000
  %or.cond40.us.2.i = or i1 %_144.us.2.i, %or.cond.us.2.i, !dbg !11472
  %_146.us.2.i = fsub float %value.us.2.i, %_0.i.us.2.i, !dbg !11472
  %885 = fmul float %_146.us.2.i, 1.562500e-02, !dbg !11472
  %ramp.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %_0.i.us.2.i, float %value.us.2.i, !dbg !11472
  %ramp4.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %885, float 0.000000e+00, !dbg !11472
  %ramp5.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float 6.400000e+01, float 0.000000e+00, !dbg !11472
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i66.i), !dbg !11473, !noalias !11475
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.2.i, i64 16, i1 false), !dbg !11478, !noalias !11412
  store float %ramp.sroa.0.0.us.2.i, ptr %853, align 4, !dbg !11479, !noalias !11475
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %slot.us.2.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, i64 16, i1 false), !dbg !11480, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i66.i), !dbg !11481, !noalias !11475
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i67.i), !dbg !11482, !noalias !11484
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, ptr noundef nonnull align 16 dereferenceable(16) %880, i64 16, i1 false), !dbg !11487, !noalias !11412
  store float %value.us.2.i, ptr %854, align 4, !dbg !11488, !noalias !11484
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %880, ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, i64 16, i1 false), !dbg !11489, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i67.i), !dbg !11490, !noalias !11484
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i68.i), !dbg !11491, !noalias !11493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, ptr noundef nonnull align 16 dereferenceable(16) %881, i64 16, i1 false), !dbg !11496, !noalias !11412
  store float %ramp4.sroa.0.0.us.2.i, ptr %855, align 4, !dbg !11497, !noalias !11493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %881, ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, i64 16, i1 false), !dbg !11498, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i68.i), !dbg !11499, !noalias !11493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i69.i), !dbg !11500, !noalias !11502
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, ptr noundef nonnull align 16 dereferenceable(16) %882, i64 16, i1 false), !dbg !11505, !noalias !11412
  store float %ramp5.sroa.0.0.us.2.i, ptr %856, align 4, !dbg !11506, !noalias !11502
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %882, ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, i64 16, i1 false), !dbg !11507, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i69.i), !dbg !11508, !noalias !11502
  store i32 64, ptr %14, align 4, !dbg !11509, !alias.scope !11382, !noalias !11412
  br label %bb40.backedge.us.2.i, !dbg !11510

bb40.backedge.us.2.i:                             ; preds = %bb46.us.2.i, %bb40.backedge.us.1.i
  %iter2.sroa.0.0.ptr106.us.3.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 24, !dbg !11511
  %886 = load i32, ptr %iter2.sroa.0.0.ptr106.us.3.i, align 4, !dbg !11438, !range !5716, !noalias !11393, !noundef !12
  %887 = trunc nuw i32 %886 to i1, !dbg !11442
  br i1 %887, label %bb46.us.3.i, label %bb36.loopexit.i, !dbg !11442

bb46.us.3.i:                                      ; preds = %bb40.backedge.us.2.i
  %888 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 28, !dbg !11438
  %value.us.3.i = load float, ptr %888, align 4, !dbg !11443, !noalias !11393, !noundef !12
  %slot.us.3.i = getelementptr inbounds nuw i8, ptr %858, i64 192, !dbg !11520
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !11444, !noalias !11448
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %words.i.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.3.i, i64 16, i1 false), !dbg !11451, !noalias !11412
  %_0.i.us.3.i = load float, ptr %852, align 4, !dbg !11456, !noalias !11448, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !11457, !noalias !11448
  %889 = getelementptr inbounds nuw i8, ptr %858, i64 208, !dbg !11458
  %890 = getelementptr inbounds nuw i8, ptr %858, i64 224, !dbg !11459
  %891 = getelementptr inbounds nuw i8, ptr %858, i64 240, !dbg !11460
  %_148.us.3.i = bitcast float %_0.i.us.3.i to i32, !dbg !11461
  %_150.us.3.i = bitcast float %value.us.3.i to i32, !dbg !11469
  %_149.us.3.i = icmp ne i32 %_148.us.3.i, %_150.us.3.i, !dbg !11472
  %892 = icmp eq i32 %_148.us.3.i, -2147483648
  %or.cond.us.3.i = or i1 %_149.us.3.i, %892, !dbg !11472
  %893 = tail call float @llvm.fabs.f32(float %_0.i.us.3.i)
  %_144.us.3.i = fcmp ueq float %893, 0x7FF0000000000000
  %or.cond40.us.3.i = or i1 %_144.us.3.i, %or.cond.us.3.i, !dbg !11472
  %_146.us.3.i = fsub float %value.us.3.i, %_0.i.us.3.i, !dbg !11472
  %894 = fmul float %_146.us.3.i, 1.562500e-02, !dbg !11472
  %ramp.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %_0.i.us.3.i, float %value.us.3.i, !dbg !11472
  %ramp4.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %894, float 0.000000e+00, !dbg !11472
  %ramp5.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float 6.400000e+01, float 0.000000e+00, !dbg !11472
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i66.i), !dbg !11473, !noalias !11475
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, ptr noundef nonnull align 16 dereferenceable(16) %slot.us.3.i, i64 16, i1 false), !dbg !11478, !noalias !11412
  store float %ramp.sroa.0.0.us.3.i, ptr %853, align 4, !dbg !11479, !noalias !11475
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %slot.us.3.i, ptr noundef nonnull align 16 dereferenceable(16) %words.i66.i, i64 16, i1 false), !dbg !11480, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i66.i), !dbg !11481, !noalias !11475
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i67.i), !dbg !11482, !noalias !11484
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, ptr noundef nonnull align 16 dereferenceable(16) %889, i64 16, i1 false), !dbg !11487, !noalias !11412
  store float %value.us.3.i, ptr %854, align 4, !dbg !11488, !noalias !11484
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %889, ptr noundef nonnull align 16 dereferenceable(16) %words.i67.i, i64 16, i1 false), !dbg !11489, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i67.i), !dbg !11490, !noalias !11484
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i68.i), !dbg !11491, !noalias !11493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, ptr noundef nonnull align 16 dereferenceable(16) %890, i64 16, i1 false), !dbg !11496, !noalias !11412
  store float %ramp4.sroa.0.0.us.3.i, ptr %855, align 4, !dbg !11497, !noalias !11493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %890, ptr noundef nonnull align 16 dereferenceable(16) %words.i68.i, i64 16, i1 false), !dbg !11498, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i68.i), !dbg !11499, !noalias !11493
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i69.i), !dbg !11500, !noalias !11502
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, ptr noundef nonnull align 16 dereferenceable(16) %891, i64 16, i1 false), !dbg !11505, !noalias !11412
  store float %ramp5.sroa.0.0.us.3.i, ptr %856, align 4, !dbg !11506, !noalias !11502
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %891, ptr noundef nonnull align 16 dereferenceable(16) %words.i69.i, i64 16, i1 false), !dbg !11507, !noalias !11412
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i69.i), !dbg !11508, !noalias !11502
  store i32 64, ptr %14, align 4, !dbg !11509, !alias.scope !11382, !noalias !11412
  br label %bb36.loopexit.i, !dbg !11510

default.unreachable:                              ; preds = %bb4.i10
  unreachable

bb7.i11:                                          ; preds = %bb4.i10
  br label %bb9.i12, !dbg !11521

bb9.i12:                                          ; preds = %bb7.i11, %bb4.i10
  %channel.sroa.0.0.sroa.phi28.i = phi ptr [ %11, %bb7.i11 ], [ %pending.i, %bb4.i10 ], !dbg !11522
  %channel.sroa.0.0.i = phi i32 [ 1, %bb7.i11 ], [ 0, %bb4.i10 ], !dbg !11522
  %895 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 16, !dbg !11523
  %_21.i = load i32, ptr %895, align 8, !dbg !11523, !alias.scope !11386, !noalias !11427, !noundef !12
  %parameter_index.i = zext i32 %_21.i to i64, !dbg !11523
  %_121.1.i = icmp slt i32 %_21.i, 0, !dbg !11525
  br i1 %_121.1.i, label %bb35.i, label %bb59.i, !dbg !11531, !prof !180

bb59.i:                                           ; preds = %bb9.i12
  %_121.0.i = shl nuw i32 %_21.i, 1, !dbg !11525
  %_127.0.i = or disjoint i32 %_121.0.i, %channel.sroa.0.0.i, !dbg !11535
  %_29.i = icmp ult i64 %iter.sroa.7.092.i, %_30.i, !dbg !11543
  %_32.i = icmp samesign ult i32 %_21.i, 4
  %or.cond16.i = and i1 %_29.i, %_32.i, !dbg !11543
  br i1 %or.cond16.i, label %bb11.i, label %bb35.i, !dbg !11543

bb11.i:                                           ; preds = %bb59.i
  %896 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 28, !dbg !11545
  %_130.i = load i32, ptr %896, align 4, !dbg !11545, !range !6171, !alias.scope !11386, !noalias !11427, !noundef !12
  %897 = icmp eq i32 %_130.i, 1, !dbg !11548
  br i1 %897, label %bb12.i13, label %bb35.i, !dbg !11548

bb12.i13:                                         ; preds = %bb11.i
  %_34.i = load i64, ptr %iter.sroa.0.093.i, align 8, !dbg !11549, !alias.scope !11386, !noalias !11427, !noundef !12
  %_33.i = icmp eq i64 %_34.i, %_23, !dbg !11549
  br i1 %_33.i, label %bb13.i, label %bb35.i, !dbg !11549

bb13.i:                                           ; preds = %bb12.i13
  %898 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 8, !dbg !11550
  %_36.i = load i64, ptr %898, align 8, !dbg !11550, !alias.scope !11386, !noalias !11427, !noundef !12
  %_35.i = icmp eq i64 %_36.i, %_23, !dbg !11550
  br i1 %_35.i, label %bb14.i14, label %bb35.i, !dbg !11550

bb14.i14:                                         ; preds = %bb13.i
  %899 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 20, !dbg !11551
  %_39.i = load float, ptr %899, align 4, !dbg !11551, !alias.scope !11386, !noalias !11427, !noundef !12
  %_38.i = bitcast float %_39.i to i32, !dbg !11552
  %900 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.093.i, i64 24, !dbg !11554
  %_4139.i = load i32, ptr %900, align 8, !dbg !11554, !alias.scope !11386, !noalias !11427, !noundef !12
  %_37.i = icmp eq i32 %_4139.i, %_38.i, !dbg !11551
  br i1 %_37.i, label %bb16.i15, label %bb35.i, !dbg !11551

bb16.i15:                                         ; preds = %bb14.i14
  %_43.i = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_d0058eaee52f1ba8b2e5577cef0c3c7f, i64 %parameter_index.i, !dbg !11555
; call effect_runtime::params::parameter_value_valid
  %_42.i = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_43.i, float noundef %_39.i) #23, !dbg !11556, !noalias !11393
  %_45.i = icmp ugt i32 %_127.0.i, %last_order.sroa.3.0.ph98.i
  %or.cond17.not.not112.i = select i1 %last_order.sroa.0.0.ph97.not.i, i1 true, i1 %_45.i, !dbg !11556
  %or.cond41.not.i = select i1 %_42.i, i1 %or.cond17.not.not112.i, i1 false, !dbg !11556
  br i1 %or.cond41.not.i, label %bb29.i, label %bb35.i, !dbg !11556

bb29.i:                                           ; preds = %bb16.i15
  %_47.i = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i, !dbg !11557
  %901 = load i32, ptr %_47.i, align 4, !dbg !11558, !range !5716, !noalias !11393, !noundef !12
  %_133.not.i = icmp eq i32 %901, 0, !dbg !11565
  br i1 %_133.not.i, label %bb31.i, label %bb35.i, !dbg !11566

bb31.i:                                           ; preds = %bb29.i
  %_47.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i
  %_135.i = fcmp oeq float %_39.i, 0.000000e+00, !dbg !11568
  %_55.sroa.0.0.i = select i1 %_135.i, float 0.000000e+00, float %_39.i, !dbg !11568
  store i32 1, ptr %_47.i.le, align 4, !dbg !11571, !noalias !11393
  %902 = getelementptr inbounds nuw i8, ptr %_47.i.le, i64 4, !dbg !11571
  store float %_55.sroa.0.0.i, ptr %902, align 4, !dbg !11571, !noalias !11393
  %_6.i.i91.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !11405
  br i1 %_6.i.i91.i, label %bb36.preheader.i, label %bb4.lr.ph.i, !dbg !11410

bb35.i:                                           ; preds = %bb29.i, %bb16.i15, %bb14.i14, %bb13.i, %bb12.i13, %bb11.i, %bb59.i, %bb9.i12, %bb4.i10
  %903 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted100.i, i64 1), !dbg !11572
  store i64 %903, ptr %851, align 8, !dbg !11522, !alias.scope !11388, !noalias !11413
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !11405
  br i1 %_6.i.i.i, label %bb36.preheader.i, label %bb4.i10, !dbg !11410

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x4_5f32x4Kb0_E16apply_automationB2_.exit: ; preds = %bb36.loopexit.i
  call void @llvm.lifetime.end.p0(ptr nonnull %pending.i), !dbg !11575, !noalias !11393
  %exitcond238.not = icmp eq i64 %848, 4, !dbg !11576
  br i1 %exitcond238.not, label %bb16, label %bb15, !dbg !8200
}
