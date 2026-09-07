define void @_RNvXse_CsdOTRa1MFkeb_13gate_expanderINtB5_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bank(ptr dead_on_unwind noalias noundef writable writeonly sret([328 x i8]) align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef align 32 dereferenceable(2656) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(112) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !12042 {
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
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 2624, !dbg !12044
  %1 = load i8, ptr %0, align 32, !dbg !12044, !range !1313, !noundef !12
  %.not = icmp eq i8 %1, 2, !dbg !12045
  br i1 %.not, label %bb13, label %bb14, !dbg !12048, !prof !180

bb14:                                             ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %block, i64 108, !dbg !12049
  %3 = load i8, ptr %2, align 4, !dbg !12049, !range !1328, !noundef !12
  %_44.not = icmp eq i8 %3, %1, !dbg !12056
  %4 = getelementptr inbounds nuw i8, ptr %block, i64 64
  %5 = load ptr, ptr %4, align 8
  %.not7 = icmp eq ptr %5, null
  %or.cond = select i1 %_44.not, i1 %.not7, i1 false, !dbg !12053
  br i1 %or.cond, label %bb4, label %bb3, !dbg !12053

bb13:                                             ; preds = %start
; call core::option::expect_failed
  tail call void @_RNvNtCs4NRVxsYgnAr_4core6option13expect_failed(ptr noalias noundef nonnull readonly captures(address, read_provenance) @alloc_376120b9c5efdf3d59386c16952a74b7, i64 noundef 31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_71049c4faf961883bcf0219fde9a65c7) #24, !dbg !12059
  unreachable, !dbg !12059

bb3:                                              ; preds = %bb14
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %_0, i8 0, i64 320, i1 false), !dbg !12060
  %report.sroa.7.0._0.sroa_idx16 = getelementptr inbounds nuw i8, ptr %_0, i64 320, !dbg !12060
  store i8 %1, ptr %report.sroa.7.0._0.sroa_idx16, align 8, !dbg !12060
  br label %bb12, !dbg !12061

bb4:                                              ; preds = %bb14
  call void @llvm.lifetime.start.p0(ptr nonnull %reports), !dbg !12062
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %reports, i8 0, i64 320, i1 false), !dbg !12063
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
  %15 = call i64 @llvm.usub.sat.i64(i64 %_36.1, i64 1), !dbg !12064
  br label %bb15, !dbg !12064

bb16:                                             ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit
  %16 = getelementptr inbounds nuw i8, ptr %block, i64 104, !dbg !12072
  %_27 = load i32, ptr %16, align 8, !dbg !12072, !noundef !12
  %frames = zext i32 %_27 to i64, !dbg !12072
  %_37.0 = load ptr, ptr %block, align 8, !dbg !12073, !nonnull !12, !align !3484, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !12073
  %_37.1 = load i64, ptr %17, align 8, !dbg !12073, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !12075
  %_38.0 = load ptr, ptr %18, align 8, !dbg !12075, !nonnull !12, !align !3484, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !12075
  %_38.1 = load i64, ptr %19, align 8, !dbg !12075, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12076), !dbg !12079
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12080), !dbg !12079
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12082), !dbg !12079
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12084), !dbg !12079
  %_9.i = load i32, ptr %14, align 4, !dbg !12086, !alias.scope !12076, !noalias !12090, !noundef !12
  %20 = tail call i32 @llvm.umin.i32(i32 %_27, i32 %_9.i), !dbg !12092
  %..i.i = zext i32 %20 to i64, !dbg !12092
  %_10.not.i = icmp eq i32 %20, 0, !dbg !12094
  br i1 %_10.not.i, label %bb4.i, label %bb9.i, !dbg !12094

bb4.i:                                            ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKb1_EB3_.exit.i, %bb16
  %_18.i = icmp ugt i32 %_27, %_9.i, !dbg !12096
  br i1 %_18.i, label %bb20.i, label %bb7.i, !dbg !12096

bb9.i:                                            ; preds = %bb16
  %21 = shl nuw nsw i64 %..i.i, 3, !dbg !12097
  %_33.not.i = icmp samesign ugt i64 %21, %_37.1
  br i1 %_33.not.i, label %bb16.i, label %bb14.i, !dbg !12098, !prof !2561

bb16.i:                                           ; preds = %bb9.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %21, i64 noundef range(i64 0, 2305843009213693952) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bc75afababbf20974edb8b966373fd0c) #24, !dbg !12109, !noalias !12090
  unreachable, !dbg !12109

bb14.i:                                           ; preds = %bb9.i
  %_41.not.i = icmp samesign ugt i64 %21, %_38.1, !dbg !12110
  br i1 %_41.not.i, label %bb19.i, label %bb18.i, !dbg !12110, !prof !180

bb19.i:                                           ; preds = %bb14.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %21, i64 noundef range(i64 0, 2305843009213693952) %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0e50f2d670fa7dbeabdb68abcb7ac10) #24, !dbg !12116, !noalias !12090
  unreachable, !dbg !12116

bb18.i:                                           ; preds = %bb14.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12117), !dbg !12120
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12121), !dbg !12120
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12123), !dbg !12120
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1888, !dbg !12125
  %_15.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1152, !dbg !12138
  %data.i.i998.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !12140
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !12145
  %_32.i.i = getelementptr inbounds nuw i8, ptr %self, i64 960, !dbg !12149
  %_58.0.i.i = load ptr, ptr %_15.i.i, align 8, !dbg !12150, !alias.scope !12151, !noalias !12152, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 1160, !dbg !12150
  %_58.1.i.i = load i64, ptr %23, align 8, !dbg !12150, !alias.scope !12151, !noalias !12152, !noundef !12
  %_45.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1184, !dbg !12154
  %_60.0.i.i = load ptr, ptr %data.i.i998.i.i, align 8, !dbg !12155, !alias.scope !12151, !noalias !12152, !nonnull !12, !noundef !12
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 1224, !dbg !12155
  %_60.1.i.i = load i64, ptr %24, align 8, !dbg !12155, !alias.scope !12151, !noalias !12152, !noundef !12
  %_50.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1248, !dbg !12156
  %_51.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2608, !dbg !12157
  %25 = getelementptr inbounds nuw i8, ptr %self, i64 2612, !dbg !12158
  %_52.i.i = load i32, ptr %25, align 4, !dbg !12158, !alias.scope !12151, !noalias !12152, !noundef !12
  %26 = getelementptr inbounds nuw i8, ptr %self, i64 2616, !dbg !12159
  %_53.i.i = load i32, ptr %26, align 8, !dbg !12159, !alias.scope !12151, !noalias !12152, !noundef !12
  %base.i.i.i = load i32, ptr %_51.i.i, align 4, !dbg !12160, !alias.scope !12151, !noalias !12170, !noundef !12
  %27 = getelementptr inbounds nuw i8, ptr %self, i64 1792
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 1824
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %32 = getelementptr inbounds nuw i8, ptr %self, i64 1856
  %33 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %35 = getelementptr inbounds nuw i8, ptr %self, i64 2400
  %36 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %37 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 2432
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 2464
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 1188
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 1252
  %45 = getelementptr inbounds nuw i8, ptr %self, i64 1192
  %46 = getelementptr inbounds nuw i8, ptr %self, i64 1256
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 1196
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 1260
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 1264
  %51 = getelementptr inbounds nuw i8, ptr %self, i64 1204
  %52 = getelementptr inbounds nuw i8, ptr %self, i64 1268
  %53 = getelementptr inbounds nuw i8, ptr %self, i64 1208
  %54 = getelementptr inbounds nuw i8, ptr %self, i64 1272
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1212
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 1276
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %iter1.sroa.0.0.ptr.i135.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %60 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %62 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %iter1.sroa.0.0.ptr.i135.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1536
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %64 = getelementptr inbounds nuw i8, ptr %self, i64 1600
  %65 = getelementptr inbounds nuw i8, ptr %self, i64 1568
  %iter1.sroa.0.0.ptr.i135.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 1760
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 1728
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 1984
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 1952
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 1920
  %iter1.sroa.0.0.ptr.i.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2016
  %72 = getelementptr inbounds nuw i8, ptr %self, i64 2112
  %73 = getelementptr inbounds nuw i8, ptr %self, i64 2080
  %74 = getelementptr inbounds nuw i8, ptr %self, i64 2048
  %iter1.sroa.0.0.ptr.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2144
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 2240
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 2208
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 2176
  %iter1.sroa.0.0.ptr.i.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2272
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 2368
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 2336
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 2304
  br label %bb42.i.i.i, !dbg !12173

bb42.i.i.i:                                       ; preds = %bb18.i, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit163.i.i
  %iter.sroa.0.0.i1884.i.i = phi i64 [ 0, %bb18.i ], [ %81, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit163.i.i ]
  %81 = add nuw nsw i64 %iter.sroa.0.0.i1884.i.i, 1, !dbg !12182
  %span.i.i.i = shl i64 %iter.sroa.0.0.i1884.i.i, 3, !dbg !12188
  %_27.i.i.i = trunc i64 %iter.sroa.0.0.i1884.i.i to i32, !dbg !12190
  %now.i.i.i = add i32 %base.i.i.i, %_27.i.i.i, !dbg !12192
  %_30.i.i.i = and i32 %now.i.i.i, %_52.i.i, !dbg !12195
  %_29.i.i.i = zext i32 %_30.i.i.i to i64, !dbg !12197
  %write.i.i.i = shl nuw nsw i64 %_29.i.i.i, 3, !dbg !12197
  %_123.i.i.i = getelementptr inbounds nuw float, ptr %_37.0, i64 %span.i.i.i, !dbg !12198
  %_36.i.i.i = add nuw nsw i64 %write.i.i.i, 8, !dbg !12207
  %_124.not.i.i.i = icmp ugt i64 %_36.i.i.i, %_58.1.i.i, !dbg !12208
  br i1 %_124.not.i.i.i, label %bb46.i.i.i, label %bb48.i.i.i, !dbg !12208, !prof !180

bb46.i.i.i:                                       ; preds = %bb42.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i.i, i64 noundef %_36.i.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cad5a7b24766ffcdda9cbfe066eb4078) #24, !dbg !12213, !noalias !12170
  unreachable, !dbg !12213

bb48.i.i.i:                                       ; preds = %bb42.i.i.i
  %_133.i.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %write.i.i.i, !dbg !12214
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_133.i.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_123.i.i.i, i64 32, i1 false), !dbg !12218, !noalias !12227
  %_141.i.i.i = getelementptr inbounds nuw float, ptr %_38.0, i64 %span.i.i.i, !dbg !12228
  %_142.not.i.i.i = icmp ugt i64 %_36.i.i.i, %_60.1.i.i, !dbg !12235
  br i1 %_142.not.i.i.i, label %bb51.i.i.i, label %bb50.i.i.i, !dbg !12235, !prof !180

bb51.i.i.i:                                       ; preds = %bb48.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i.i, i64 noundef %_36.i.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_72e24c5578221343e8a784545a3910d2) #24, !dbg !12239, !noalias !12170
  unreachable, !dbg !12239

bb50.i.i.i:                                       ; preds = %bb48.i.i.i
  %_149.i.i.i = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %write.i.i.i, !dbg !12240
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_149.i.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_141.i.i.i, i64 32, i1 false), !dbg !12244, !noalias !12249
  %_52.i.i.i = sub i32 %now.i.i.i, %_53.i.i, !dbg !12250
  %_51.i.i.i = and i32 %_52.i.i.i, %_52.i.i, !dbg !12253
  %_50.i.i.i = zext i32 %_51.i.i.i to i64, !dbg !12254
  %read.i.i.i = shl nuw nsw i64 %_50.i.i.i, 3, !dbg !12254
  %_55.i.i.i = add nuw nsw i64 %read.i.i.i, 8, !dbg !12255
  %_182.not.i.i.i = icmp ugt i64 %_55.i.i.i, %_58.1.i.i, !dbg !12257
  br i1 %_182.not.i.i.i, label %bb61.i.i.i, label %bb60.i.i.i, !dbg !12257, !prof !180

bb61.i.i.i:                                       ; preds = %bb50.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i.i, i64 noundef %_55.i.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba99eeeb3482270ebe1c674b4013dd6a) #24, !dbg !12261, !noalias !12170
  unreachable, !dbg !12261

bb60.i.i.i:                                       ; preds = %bb50.i.i.i
  %_189.i.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %read.i.i.i, !dbg !12262
  %lanes.i582.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_189.i.i.i, align 4, !dbg !12266, !alias.scope !12274, !noalias !12278
  %_190.not.i.i.i = icmp ugt i64 %_55.i.i.i, %_60.1.i.i, !dbg !12282
  br i1 %_190.not.i.i.i, label %bb64.i.i.i, label %bb63.i.i.i, !dbg !12282, !prof !180

bb64.i.i.i:                                       ; preds = %bb60.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i.i, i64 noundef %_55.i.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f4c27429e4885e1b619f1e4a3587acc) #24, !dbg !12287, !noalias !12170
  unreachable, !dbg !12287

bb63.i.i.i:                                       ; preds = %bb60.i.i.i
  %_195.i.i.i = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %read.i.i.i, !dbg !12288
  %lanes.i576.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_195.i.i.i, align 4, !dbg !12292, !alias.scope !12297, !noalias !12301
  %_67.i.i.i = load i32, ptr %_45.i.i, align 4, !dbg !12305, !alias.scope !12151, !noalias !12170, !noundef !12
  %_66.i.i.i = sub i32 %now.i.i.i, %_67.i.i.i, !dbg !12311
  %_65.i.i.i = and i32 %_66.i.i.i, %_52.i.i, !dbg !12313
  %_64.i.i.i = zext i32 %_65.i.i.i to i64, !dbg !12314
  %_63.i.i.i = shl nuw nsw i64 %_64.i.i.i, 3, !dbg !12314
  %_75.i.i.i = load i32, ptr %_50.i.i, align 4, !dbg !12315, !alias.scope !12151, !noalias !12170, !noundef !12
  %_74.i.i.i = sub i32 %now.i.i.i, %_75.i.i.i, !dbg !12317
  %_73.i.i.i = and i32 %_74.i.i.i, %_52.i.i, !dbg !12319
  %_72.i.i.i = zext i32 %_73.i.i.i to i64, !dbg !12320
  %_71.i.i.i = shl nuw nsw i64 %_72.i.i.i, 3, !dbg !12320
  %_80.i.i.i = icmp ult i64 %_63.i.i.i, %_58.1.i.i, !dbg !12321
  br i1 %_80.i.i.i, label %bb22.i.i.i, label %panic18.i.i.i, !dbg !12321

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit163.i.i: ; preds = %bb26.i.7.i.i
  %82 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.7.i.i, !dbg !12323
  %_85.i.72341.i.i = load i32, ptr %82, align 4, !dbg !12323, !noalias !12170, !noundef !12
  %83 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.7.i.i, !dbg !12324
  %_87.i.72342.i.i = load i32, ptr %83, align 4, !dbg !12324, !noalias !12170, !noundef !12
  %_12.i127.sroa.0.0.copyload.i.i = load <8 x float>, ptr %57, align 32, !dbg !12325, !alias.scope !12151, !noalias !12332
  %84 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i127.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12339
  %85 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i127.sroa.0.0.copyload.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12358
  %_16.i123.sroa.0.0.copyload.i.i = load <8 x float>, ptr %13, align 32, !dbg !12371, !alias.scope !12151, !noalias !12332
  %_17.i122.sroa.0.0.copyload.i.i = load <8 x float>, ptr %58, align 32, !dbg !12373, !alias.scope !12151, !noalias !12332
  %86 = fadd <8 x float> %_16.i123.sroa.0.0.copyload.i.i, %_17.i122.sroa.0.0.copyload.i.i, !dbg !12374
  %_20.i119.sroa.0.0.copyload.i.i = load <8 x float>, ptr %59, align 32, !dbg !12384, !alias.scope !12151, !noalias !12332
  %87 = bitcast <8 x float> %85 to <8 x i32>, !dbg !12386
  %88 = icmp slt <8 x i32> %87, zeroinitializer, !dbg !12393
  %89 = select <8 x i1> %88, <8 x float> %_20.i119.sroa.0.0.copyload.i.i, <8 x float> %86, !dbg !12393
  %90 = bitcast <8 x float> %84 to <8 x i32>, !dbg !12397
  %91 = icmp slt <8 x i32> %90, zeroinitializer, !dbg !12401
  %92 = select <8 x i1> %91, <8 x float> %89, <8 x float> %_16.i123.sroa.0.0.copyload.i.i, !dbg !12401
  store <8 x float> %92, ptr %13, align 32, !dbg !12403, !alias.scope !12151, !noalias !12332
  %93 = select <8 x i1> %88, <8 x float> zeroinitializer, <8 x float> %_17.i122.sroa.0.0.copyload.i.i, !dbg !12404
  store <8 x float> %93, ptr %58, align 32, !dbg !12409, !alias.scope !12151, !noalias !12332
  %94 = fadd <8 x float> %_12.i127.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !12410
  %95 = select <8 x i1> %91, <8 x float> %94, <8 x float> %_12.i127.sroa.0.0.copyload.i.i, !dbg !12420
  store <8 x float> %95, ptr %57, align 32, !dbg !12425, !alias.scope !12151, !noalias !12332
  %_12.i127.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %60, align 32, !dbg !12325, !alias.scope !12151, !noalias !12332
  %96 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i127.sroa.0.0.copyload.1.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12339
  %97 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i127.sroa.0.0.copyload.1.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12358
  %_16.i123.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i135.1.i.i, align 32, !dbg !12371, !alias.scope !12151, !noalias !12332
  %_17.i122.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %61, align 32, !dbg !12373, !alias.scope !12151, !noalias !12332
  %98 = fadd <8 x float> %_16.i123.sroa.0.0.copyload.1.i.i, %_17.i122.sroa.0.0.copyload.1.i.i, !dbg !12374
  %_20.i119.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %62, align 32, !dbg !12384, !alias.scope !12151, !noalias !12332
  %99 = bitcast <8 x float> %97 to <8 x i32>, !dbg !12386
  %100 = icmp slt <8 x i32> %99, zeroinitializer, !dbg !12393
  %101 = select <8 x i1> %100, <8 x float> %_20.i119.sroa.0.0.copyload.1.i.i, <8 x float> %98, !dbg !12393
  %102 = bitcast <8 x float> %96 to <8 x i32>, !dbg !12397
  %103 = icmp slt <8 x i32> %102, zeroinitializer, !dbg !12401
  %104 = select <8 x i1> %103, <8 x float> %101, <8 x float> %_16.i123.sroa.0.0.copyload.1.i.i, !dbg !12401
  store <8 x float> %104, ptr %iter1.sroa.0.0.ptr.i135.1.i.i, align 32, !dbg !12403, !alias.scope !12151, !noalias !12332
  %105 = select <8 x i1> %100, <8 x float> zeroinitializer, <8 x float> %_17.i122.sroa.0.0.copyload.1.i.i, !dbg !12404
  store <8 x float> %105, ptr %61, align 32, !dbg !12409, !alias.scope !12151, !noalias !12332
  %106 = fadd <8 x float> %_12.i127.sroa.0.0.copyload.1.i.i, splat (float -1.000000e+00), !dbg !12410
  %107 = select <8 x i1> %103, <8 x float> %106, <8 x float> %_12.i127.sroa.0.0.copyload.1.i.i, !dbg !12420
  store <8 x float> %107, ptr %60, align 32, !dbg !12425, !alias.scope !12151, !noalias !12332
  %_12.i127.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %63, align 32, !dbg !12325, !alias.scope !12151, !noalias !12332
  %108 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i127.sroa.0.0.copyload.2.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12339
  %109 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i127.sroa.0.0.copyload.2.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12358
  %_16.i123.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i135.2.i.i, align 32, !dbg !12371, !alias.scope !12151, !noalias !12332
  %_17.i122.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %64, align 32, !dbg !12373, !alias.scope !12151, !noalias !12332
  %110 = fadd <8 x float> %_16.i123.sroa.0.0.copyload.2.i.i, %_17.i122.sroa.0.0.copyload.2.i.i, !dbg !12374
  %_20.i119.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %65, align 32, !dbg !12384, !alias.scope !12151, !noalias !12332
  %111 = bitcast <8 x float> %109 to <8 x i32>, !dbg !12386
  %112 = icmp slt <8 x i32> %111, zeroinitializer, !dbg !12393
  %113 = select <8 x i1> %112, <8 x float> %_20.i119.sroa.0.0.copyload.2.i.i, <8 x float> %110, !dbg !12393
  %114 = bitcast <8 x float> %108 to <8 x i32>, !dbg !12397
  %115 = icmp slt <8 x i32> %114, zeroinitializer, !dbg !12401
  %116 = select <8 x i1> %115, <8 x float> %113, <8 x float> %_16.i123.sroa.0.0.copyload.2.i.i, !dbg !12401
  store <8 x float> %116, ptr %iter1.sroa.0.0.ptr.i135.2.i.i, align 32, !dbg !12403, !alias.scope !12151, !noalias !12332
  %117 = select <8 x i1> %112, <8 x float> zeroinitializer, <8 x float> %_17.i122.sroa.0.0.copyload.2.i.i, !dbg !12404
  store <8 x float> %117, ptr %64, align 32, !dbg !12409, !alias.scope !12151, !noalias !12332
  %118 = fadd <8 x float> %_12.i127.sroa.0.0.copyload.2.i.i, splat (float -1.000000e+00), !dbg !12410
  %119 = select <8 x i1> %115, <8 x float> %118, <8 x float> %_12.i127.sroa.0.0.copyload.2.i.i, !dbg !12420
  store <8 x float> %119, ptr %63, align 32, !dbg !12425, !alias.scope !12151, !noalias !12332
  %_12.i127.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %66, align 32, !dbg !12325, !alias.scope !12151, !noalias !12332
  %120 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i127.sroa.0.0.copyload.3.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12339
  %121 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i127.sroa.0.0.copyload.3.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12358
  %_16.i123.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i135.3.i.i, align 32, !dbg !12371, !alias.scope !12151, !noalias !12332
  %_17.i122.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %67, align 32, !dbg !12373, !alias.scope !12151, !noalias !12332
  %122 = fadd <8 x float> %_16.i123.sroa.0.0.copyload.3.i.i, %_17.i122.sroa.0.0.copyload.3.i.i, !dbg !12374
  %_20.i119.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %68, align 32, !dbg !12384, !alias.scope !12151, !noalias !12332
  %123 = bitcast <8 x float> %121 to <8 x i32>, !dbg !12386
  %124 = icmp slt <8 x i32> %123, zeroinitializer, !dbg !12393
  %125 = select <8 x i1> %124, <8 x float> %_20.i119.sroa.0.0.copyload.3.i.i, <8 x float> %122, !dbg !12393
  %126 = bitcast <8 x float> %120 to <8 x i32>, !dbg !12397
  %127 = icmp slt <8 x i32> %126, zeroinitializer, !dbg !12401
  %128 = select <8 x i1> %127, <8 x float> %125, <8 x float> %_16.i123.sroa.0.0.copyload.3.i.i, !dbg !12401
  store <8 x float> %128, ptr %iter1.sroa.0.0.ptr.i135.3.i.i, align 32, !dbg !12403, !alias.scope !12151, !noalias !12332
  %129 = select <8 x i1> %124, <8 x float> zeroinitializer, <8 x float> %_17.i122.sroa.0.0.copyload.3.i.i, !dbg !12404
  store <8 x float> %129, ptr %67, align 32, !dbg !12409, !alias.scope !12151, !noalias !12332
  %130 = fadd <8 x float> %_12.i127.sroa.0.0.copyload.3.i.i, splat (float -1.000000e+00), !dbg !12410
  %131 = select <8 x i1> %127, <8 x float> %130, <8 x float> %_12.i127.sroa.0.0.copyload.3.i.i, !dbg !12420
  store <8 x float> %131, ptr %66, align 32, !dbg !12425, !alias.scope !12151, !noalias !12332
  %_41.i98.sroa.0.0.copyload.i.i = load <8 x float>, ptr %29, align 32, !dbg !12426, !alias.scope !12151, !noalias !12434
  %132 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_41.i98.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12436
  %133 = bitcast <8 x float> %132 to <8 x i32>, !dbg !12442
  %134 = icmp slt <8 x i32> %133, zeroinitializer, !dbg !12446
  %lanes.i570.sroa.0.0.vec.insert.i.i = insertelement <8 x i32> poison, i32 %_78.i2311.i.i, i64 0, !dbg !12448
  %lanes.i570.sroa.0.4.vec.insert.i.i = insertelement <8 x i32> %lanes.i570.sroa.0.0.vec.insert.i.i, i32 %_78.i.12315.i.i, i64 1, !dbg !12448
  %lanes.i570.sroa.0.8.vec.insert.i.i = insertelement <8 x i32> %lanes.i570.sroa.0.4.vec.insert.i.i, i32 %_78.i.22319.i.i, i64 2, !dbg !12448
  %lanes.i570.sroa.0.12.vec.insert.i.i = insertelement <8 x i32> %lanes.i570.sroa.0.8.vec.insert.i.i, i32 %_78.i.32323.i.i, i64 3, !dbg !12448
  %lanes.i570.sroa.0.16.vec.insert.i.i = insertelement <8 x i32> %lanes.i570.sroa.0.12.vec.insert.i.i, i32 %_78.i.42327.i.i, i64 4, !dbg !12448
  %lanes.i570.sroa.0.20.vec.insert.i.i = insertelement <8 x i32> %lanes.i570.sroa.0.16.vec.insert.i.i, i32 %_78.i.52331.i.i, i64 5, !dbg !12448
  %lanes.i570.sroa.0.24.vec.insert.i.i = insertelement <8 x i32> %lanes.i570.sroa.0.20.vec.insert.i.i, i32 %_78.i.62335.i.i, i64 6, !dbg !12448
  %lanes.i570.sroa.0.28.vec.insert.i.i = insertelement <8 x i32> %lanes.i570.sroa.0.24.vec.insert.i.i, i32 %_78.i.72339.i.i, i64 7, !dbg !12448
  %135 = and <8 x i32> %lanes.i570.sroa.0.28.vec.insert.i.i, splat (i32 2147483647), !dbg !12453
  %136 = bitcast <8 x i32> %135 to <8 x float>, !dbg !12468
  %137 = fmul <8 x float> %136, splat (float 5.000000e-01), !dbg !12469
  %lanes.i564.sroa.0.0.vec.insert.i.i = insertelement <8 x i32> poison, i32 %_82.i2312.i.i, i64 0, !dbg !12479
  %lanes.i564.sroa.0.4.vec.insert.i.i = insertelement <8 x i32> %lanes.i564.sroa.0.0.vec.insert.i.i, i32 %_82.i.12316.i.i, i64 1, !dbg !12479
  %lanes.i564.sroa.0.8.vec.insert.i.i = insertelement <8 x i32> %lanes.i564.sroa.0.4.vec.insert.i.i, i32 %_82.i.22320.i.i, i64 2, !dbg !12479
  %lanes.i564.sroa.0.12.vec.insert.i.i = insertelement <8 x i32> %lanes.i564.sroa.0.8.vec.insert.i.i, i32 %_82.i.32324.i.i, i64 3, !dbg !12479
  %lanes.i564.sroa.0.16.vec.insert.i.i = insertelement <8 x i32> %lanes.i564.sroa.0.12.vec.insert.i.i, i32 %_82.i.42328.i.i, i64 4, !dbg !12479
  %lanes.i564.sroa.0.20.vec.insert.i.i = insertelement <8 x i32> %lanes.i564.sroa.0.16.vec.insert.i.i, i32 %_82.i.52332.i.i, i64 5, !dbg !12479
  %lanes.i564.sroa.0.24.vec.insert.i.i = insertelement <8 x i32> %lanes.i564.sroa.0.20.vec.insert.i.i, i32 %_82.i.62336.i.i, i64 6, !dbg !12479
  %lanes.i564.sroa.0.28.vec.insert.i.i = insertelement <8 x i32> %lanes.i564.sroa.0.24.vec.insert.i.i, i32 %_82.i.72340.i.i, i64 7, !dbg !12479
  %138 = and <8 x i32> %lanes.i564.sroa.0.28.vec.insert.i.i, splat (i32 2147483647), !dbg !12484
  %139 = bitcast <8 x i32> %138 to <8 x float>, !dbg !12490
  %140 = fmul <8 x float> %139, splat (float 5.000000e-01), !dbg !12491
  %141 = fadd <8 x float> %137, %140, !dbg !12496
  %_37.i102.sroa.0.0.copyload.i.i = load <8 x float>, ptr %28, align 32, !dbg !12501, !alias.scope !12151, !noalias !12434
  %142 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_37.i102.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12502
  %143 = bitcast <8 x float> %142 to <8 x i32>, !dbg !12508
  %144 = icmp slt <8 x i32> %143, zeroinitializer, !dbg !12512
  %145 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %136, <8 x float> %139), !dbg !12514
  %146 = select <8 x i1> %144, <8 x float> %145, <8 x float> %136, !dbg !12512
  %147 = select <8 x i1> %134, <8 x float> %141, <8 x float> %146, !dbg !12446
  %148 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %147, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !12523
  %149 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %148, <8 x float> splat (float 0x3810000000000000)), !dbg !12528
  %150 = bitcast <8 x float> %149 to <4 x i64>, !dbg !12537
  %151 = and <4 x i64> %150, splat (i64 36028792732385279), !dbg !12538
  %152 = or disjoint <4 x i64> %151, splat (i64 4575657222473777152), !dbg !12556
  %153 = bitcast <4 x i64> %152 to <8 x float>, !dbg !12565
  %154 = fadd <8 x float> %153, splat (float -1.000000e+00), !dbg !12566
  %155 = fmul <8 x float> %154, splat (float 0x3F9B17A960000000), !dbg !12572
  %156 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %155, !dbg !12580
  %157 = fmul <8 x float> %154, %156, !dbg !12572
  %158 = fadd <8 x float> %157, splat (float 0xBFD1E3F400000000), !dbg !12580
  %159 = fmul <8 x float> %154, %158, !dbg !12572
  %160 = fadd <8 x float> %159, splat (float 0x3FDD544F20000000), !dbg !12580
  %161 = fmul <8 x float> %154, %160, !dbg !12572
  %162 = fadd <8 x float> %161, splat (float 0xBFE6FC2A60000000), !dbg !12580
  %163 = fmul <8 x float> %154, %162, !dbg !12572
  %164 = fadd <8 x float> %163, splat (float 0x3FF714B2A0000000), !dbg !12580
  %165 = bitcast <8 x float> %149 to <8 x i32>, !dbg !12585
  %_3.i1002.i.i = lshr <8 x i32> %165, splat (i32 23), !dbg !12595
  %166 = or disjoint <8 x i32> %_3.i1002.i.i, splat (i32 1258291200), !dbg !12596
  %167 = bitcast <8 x i32> %166 to <8 x float>, !dbg !12602
  %168 = fadd <8 x float> %167, splat (float 0xC160000FE0000000), !dbg !12603
  %169 = fmul <8 x float> %154, %164, !dbg !12609
  %170 = fadd <8 x float> %168, %169, !dbg !12614
  %171 = fmul <8 x float> %170, splat (float 0x4018151820000000), !dbg !12619
  %172 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %171, <8 x float> splat (float 2.400000e+01)), !dbg !12624
  %173 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %172, <8 x float> splat (float -1.600000e+02)), !dbg !12633
  %_55.i84.sroa.0.0.copyload.i.i = load <8 x float>, ptr %27, align 32, !dbg !12638, !alias.scope !12151, !noalias !12332
  %174 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_55.i84.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12640
  %175 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %173, <8 x float> %92, i8 29), !dbg !12646
  %176 = fsub <8 x float> %92, %128, !dbg !12659
  %177 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %173, <8 x float> %176, i8 29), !dbg !12665
  %178 = bitcast <8 x float> %174 to <8 x i32>, !dbg !12671
  %179 = xor <8 x i32> %178, splat (i32 -1), !dbg !12687
  %180 = bitcast <8 x float> %175 to <8 x i32>, !dbg !12692
  %181 = and <8 x i32> %180, %179, !dbg !12699
  %182 = bitcast <8 x float> %177 to <8 x i32>, !dbg !12701
  %183 = and <8 x i32> %182, %178, !dbg !12706
  %184 = or <8 x i32> %183, %181, !dbg !12708
  %185 = xor <8 x i32> %182, splat (i32 -1), !dbg !12721
  %_67.i72.sroa.0.0.copyload.i.i = load <8 x float>, ptr %30, align 32, !dbg !12729, !alias.scope !12151, !noalias !12332
  %186 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_67.i72.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12730
  %187 = bitcast <8 x float> %186 to <8 x i32>, !dbg !12736
  %188 = and <8 x i32> %185, %187, !dbg !12740
  %189 = and <8 x i32> %188, %178, !dbg !12740
  %190 = or <8 x i32> %189, %184, !dbg !12745
  %191 = icmp slt <8 x i32> %190, zeroinitializer, !dbg !12751
  %192 = select <8 x i1> %191, <8 x float> splat (float 1.000000e+00), <8 x float> zeroinitializer, !dbg !12751
  %_71.i68.sroa.0.0.copyload.i.i = load <8 x float>, ptr %31, align 32, !dbg !12756, !alias.scope !12151, !noalias !12434
  %193 = fadd <8 x float> %_67.i72.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !12758
  %194 = icmp slt <8 x i32> %189, zeroinitializer, !dbg !12763
  %195 = select <8 x i1> %194, <8 x float> %193, <8 x float> %_67.i72.sroa.0.0.copyload.i.i, !dbg !12763
  %196 = icmp slt <8 x i32> %184, zeroinitializer, !dbg !12768
  %197 = select <8 x i1> %196, <8 x float> %_71.i68.sroa.0.0.copyload.i.i, <8 x float> %195, !dbg !12768
  store <8 x float> %197, ptr %30, align 32, !dbg !12773, !alias.scope !12151, !noalias !12332
  store <8 x float> %192, ptr %27, align 32, !dbg !12774, !alias.scope !12151, !noalias !12332
  %_86.i53.sroa.0.0.copyload.i.i = load <8 x float>, ptr %32, align 32, !dbg !12775, !alias.scope !12151, !noalias !12332
  %_88.i52.sroa.0.0.copyload.i.i = load <8 x float>, ptr %33, align 32, !dbg !12778, !alias.scope !12151, !noalias !12434
  %_7.i846.i.i = load <8 x float>, ptr %22, align 32, !dbg !12779, !alias.scope !12781, !noalias !12784
  %198 = fadd <8 x float> %104, splat (float -1.000000e+00), !dbg !12788
  %199 = fsub <8 x float> %173, %92, !dbg !12793
  %200 = fmul <8 x float> %198, %199, !dbg !12798
  %201 = fneg <8 x float> %116, !dbg !12803
  %202 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %200, <8 x float> %201), !dbg !12812
  %203 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %202, <8 x float> zeroinitializer), !dbg !12817
  %204 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %192, <8 x float> zeroinitializer, i8 30), !dbg !12822
  %205 = bitcast <8 x float> %204 to <8 x i32>, !dbg !12828
  %206 = icmp slt <8 x i32> %205, zeroinitializer, !dbg !12832
  %207 = select <8 x i1> %206, <8 x float> zeroinitializer, <8 x float> %203, !dbg !12832
  %208 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %207, <8 x float> %_86.i53.sroa.0.0.copyload.i.i, i8 30), !dbg !12834
  %209 = bitcast <8 x float> %208 to <8 x i32>, !dbg !12840
  %210 = icmp slt <8 x i32> %209, zeroinitializer, !dbg !12843
  %211 = select <8 x i1> %210, <8 x float> %_7.i846.i.i, <8 x float> %_88.i52.sroa.0.0.copyload.i.i, !dbg !12843
  %212 = fsub <8 x float> %207, %_86.i53.sroa.0.0.copyload.i.i, !dbg !12845
  %213 = fmul <8 x float> %212, %211, !dbg !12851
  %214 = fadd <8 x float> %_86.i53.sroa.0.0.copyload.i.i, %213, !dbg !12859
  %215 = bitcast <8 x float> %214 to <8 x i32>, !dbg !12866
  %216 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %214), !dbg !12873
  %217 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %216, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !12875
  %218 = bitcast <8 x float> %217 to <8 x i32>, !dbg !12887
  %219 = xor <8 x i32> %218, splat (i32 -1), !dbg !12899
  %220 = and <8 x i32> %215, %219, !dbg !12901
  store <8 x i32> %220, ptr %32, align 32, !dbg !12907, !alias.scope !12151, !noalias !12332
  %221 = bitcast <8 x i32> %220 to <8 x float>, !dbg !12909
  %222 = fmul <8 x float> %221, splat (float 0x3FC542A5A0000000), !dbg !12910
  %223 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %222, <8 x float> splat (float -1.260000e+02)), !dbg !12917
  %224 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %223, <8 x float> splat (float 1.270000e+02)), !dbg !12924
  %225 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %224), !dbg !12929
  %226 = fsub <8 x float> %224, %225, !dbg !12939
  %227 = fmul <8 x float> %226, splat (float 0x3F5E974FA0000000), !dbg !12945
  %228 = fadd <8 x float> %227, splat (float 0x3F82778560000000), !dbg !12953
  %229 = fmul <8 x float> %226, %228, !dbg !12945
  %230 = fadd <8 x float> %229, splat (float 0x3FAC91CE60000000), !dbg !12953
  %231 = fmul <8 x float> %226, %230, !dbg !12945
  %232 = fadd <8 x float> %231, splat (float 0x3FCEBDB560000000), !dbg !12953
  %233 = fmul <8 x float> %226, %232, !dbg !12945
  %234 = fadd <8 x float> %233, splat (float 0x3FE62E4BA0000000), !dbg !12953
  %_98.i42.sroa.0.0.copyload.i.i = load <8 x float>, ptr %34, align 32, !dbg !12958, !alias.scope !12151, !noalias !12434
  %_12.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %69, align 32, !dbg !12960, !alias.scope !12151, !noalias !12963
  %235 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12970
  %236 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.sroa.0.0.copyload.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12976
  %_16.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %data.i.i.i.i, align 32, !dbg !12982, !alias.scope !12151, !noalias !12963
  %_17.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %70, align 32, !dbg !12983, !alias.scope !12151, !noalias !12963
  %237 = fadd <8 x float> %_16.i.sroa.0.0.copyload.i.i, %_17.i.sroa.0.0.copyload.i.i, !dbg !12984
  %_20.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %71, align 32, !dbg !12989, !alias.scope !12151, !noalias !12963
  %238 = bitcast <8 x float> %236 to <8 x i32>, !dbg !12990
  %239 = icmp slt <8 x i32> %238, zeroinitializer, !dbg !12994
  %240 = select <8 x i1> %239, <8 x float> %_20.i.sroa.0.0.copyload.i.i, <8 x float> %237, !dbg !12994
  %241 = bitcast <8 x float> %235 to <8 x i32>, !dbg !12996
  %242 = icmp slt <8 x i32> %241, zeroinitializer, !dbg !13000
  %243 = select <8 x i1> %242, <8 x float> %240, <8 x float> %_16.i.sroa.0.0.copyload.i.i, !dbg !13000
  store <8 x float> %243, ptr %data.i.i.i.i, align 32, !dbg !13002, !alias.scope !12151, !noalias !12963
  %244 = select <8 x i1> %239, <8 x float> zeroinitializer, <8 x float> %_17.i.sroa.0.0.copyload.i.i, !dbg !13003
  store <8 x float> %244, ptr %70, align 32, !dbg !13008, !alias.scope !12151, !noalias !12963
  %245 = fadd <8 x float> %_12.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !13009
  %246 = select <8 x i1> %242, <8 x float> %245, <8 x float> %_12.i.sroa.0.0.copyload.i.i, !dbg !13014
  store <8 x float> %246, ptr %69, align 32, !dbg !13019, !alias.scope !12151, !noalias !12963
  %_12.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %72, align 32, !dbg !12960, !alias.scope !12151, !noalias !12963
  %247 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.sroa.0.0.copyload.1.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12970
  %248 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.sroa.0.0.copyload.1.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12976
  %_16.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i.1.i.i, align 32, !dbg !12982, !alias.scope !12151, !noalias !12963
  %_17.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %73, align 32, !dbg !12983, !alias.scope !12151, !noalias !12963
  %249 = fadd <8 x float> %_16.i.sroa.0.0.copyload.1.i.i, %_17.i.sroa.0.0.copyload.1.i.i, !dbg !12984
  %_20.i.sroa.0.0.copyload.1.i.i = load <8 x float>, ptr %74, align 32, !dbg !12989, !alias.scope !12151, !noalias !12963
  %250 = bitcast <8 x float> %248 to <8 x i32>, !dbg !12990
  %251 = icmp slt <8 x i32> %250, zeroinitializer, !dbg !12994
  %252 = select <8 x i1> %251, <8 x float> %_20.i.sroa.0.0.copyload.1.i.i, <8 x float> %249, !dbg !12994
  %253 = bitcast <8 x float> %247 to <8 x i32>, !dbg !12996
  %254 = icmp slt <8 x i32> %253, zeroinitializer, !dbg !13000
  %255 = select <8 x i1> %254, <8 x float> %252, <8 x float> %_16.i.sroa.0.0.copyload.1.i.i, !dbg !13000
  store <8 x float> %255, ptr %iter1.sroa.0.0.ptr.i.1.i.i, align 32, !dbg !13002, !alias.scope !12151, !noalias !12963
  %256 = select <8 x i1> %251, <8 x float> zeroinitializer, <8 x float> %_17.i.sroa.0.0.copyload.1.i.i, !dbg !13003
  store <8 x float> %256, ptr %73, align 32, !dbg !13008, !alias.scope !12151, !noalias !12963
  %257 = fadd <8 x float> %_12.i.sroa.0.0.copyload.1.i.i, splat (float -1.000000e+00), !dbg !13009
  %258 = select <8 x i1> %254, <8 x float> %257, <8 x float> %_12.i.sroa.0.0.copyload.1.i.i, !dbg !13014
  store <8 x float> %258, ptr %72, align 32, !dbg !13019, !alias.scope !12151, !noalias !12963
  %_12.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %75, align 32, !dbg !12960, !alias.scope !12151, !noalias !12963
  %259 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.sroa.0.0.copyload.2.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12970
  %260 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.sroa.0.0.copyload.2.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12976
  %_16.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i.2.i.i, align 32, !dbg !12982, !alias.scope !12151, !noalias !12963
  %_17.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %76, align 32, !dbg !12983, !alias.scope !12151, !noalias !12963
  %261 = fadd <8 x float> %_16.i.sroa.0.0.copyload.2.i.i, %_17.i.sroa.0.0.copyload.2.i.i, !dbg !12984
  %_20.i.sroa.0.0.copyload.2.i.i = load <8 x float>, ptr %77, align 32, !dbg !12989, !alias.scope !12151, !noalias !12963
  %262 = bitcast <8 x float> %260 to <8 x i32>, !dbg !12990
  %263 = icmp slt <8 x i32> %262, zeroinitializer, !dbg !12994
  %264 = select <8 x i1> %263, <8 x float> %_20.i.sroa.0.0.copyload.2.i.i, <8 x float> %261, !dbg !12994
  %265 = bitcast <8 x float> %259 to <8 x i32>, !dbg !12996
  %266 = icmp slt <8 x i32> %265, zeroinitializer, !dbg !13000
  %267 = select <8 x i1> %266, <8 x float> %264, <8 x float> %_16.i.sroa.0.0.copyload.2.i.i, !dbg !13000
  store <8 x float> %267, ptr %iter1.sroa.0.0.ptr.i.2.i.i, align 32, !dbg !13002, !alias.scope !12151, !noalias !12963
  %268 = select <8 x i1> %263, <8 x float> zeroinitializer, <8 x float> %_17.i.sroa.0.0.copyload.2.i.i, !dbg !13003
  store <8 x float> %268, ptr %76, align 32, !dbg !13008, !alias.scope !12151, !noalias !12963
  %269 = fadd <8 x float> %_12.i.sroa.0.0.copyload.2.i.i, splat (float -1.000000e+00), !dbg !13009
  %270 = select <8 x i1> %266, <8 x float> %269, <8 x float> %_12.i.sroa.0.0.copyload.2.i.i, !dbg !13014
  store <8 x float> %270, ptr %75, align 32, !dbg !13019, !alias.scope !12151, !noalias !12963
  %_12.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %78, align 32, !dbg !12960, !alias.scope !12151, !noalias !12963
  %271 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.sroa.0.0.copyload.3.i.i, <8 x float> zeroinitializer, i8 30), !dbg !12970
  %272 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_12.i.sroa.0.0.copyload.3.i.i, <8 x float> splat (float 1.000000e+00), i8 0), !dbg !12976
  %_16.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %iter1.sroa.0.0.ptr.i.3.i.i, align 32, !dbg !12982, !alias.scope !12151, !noalias !12963
  %_17.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %79, align 32, !dbg !12983, !alias.scope !12151, !noalias !12963
  %273 = fadd <8 x float> %_16.i.sroa.0.0.copyload.3.i.i, %_17.i.sroa.0.0.copyload.3.i.i, !dbg !12984
  %_20.i.sroa.0.0.copyload.3.i.i = load <8 x float>, ptr %80, align 32, !dbg !12989, !alias.scope !12151, !noalias !12963
  %274 = bitcast <8 x float> %272 to <8 x i32>, !dbg !12990
  %275 = icmp slt <8 x i32> %274, zeroinitializer, !dbg !12994
  %276 = select <8 x i1> %275, <8 x float> %_20.i.sroa.0.0.copyload.3.i.i, <8 x float> %273, !dbg !12994
  %277 = bitcast <8 x float> %271 to <8 x i32>, !dbg !12996
  %278 = icmp slt <8 x i32> %277, zeroinitializer, !dbg !13000
  %279 = select <8 x i1> %278, <8 x float> %276, <8 x float> %_16.i.sroa.0.0.copyload.3.i.i, !dbg !13000
  store <8 x float> %279, ptr %iter1.sroa.0.0.ptr.i.3.i.i, align 32, !dbg !13002, !alias.scope !12151, !noalias !12963
  %280 = select <8 x i1> %275, <8 x float> zeroinitializer, <8 x float> %_17.i.sroa.0.0.copyload.3.i.i, !dbg !13003
  store <8 x float> %280, ptr %79, align 32, !dbg !13008, !alias.scope !12151, !noalias !12963
  %281 = fadd <8 x float> %_12.i.sroa.0.0.copyload.3.i.i, splat (float -1.000000e+00), !dbg !13009
  %282 = select <8 x i1> %278, <8 x float> %281, <8 x float> %_12.i.sroa.0.0.copyload.3.i.i, !dbg !13014
  store <8 x float> %282, ptr %78, align 32, !dbg !13019, !alias.scope !12151, !noalias !12963
  %283 = fmul <8 x float> %226, %234, !dbg !13020
  %284 = fadd <8 x float> %283, splat (float 1.000000e+00), !dbg !13025
  %285 = fadd <8 x float> %225, splat (float 0x4160000FE0000000), !dbg !13030
  %286 = bitcast <8 x float> %285 to <8 x i32>, !dbg !13039
  %_3.i1003.i.i = shl <8 x i32> %286, splat (i32 23), !dbg !13049
  %287 = bitcast <8 x i32> %_3.i1003.i.i to <8 x float>, !dbg !13050
  %288 = fmul <8 x float> %284, %287, !dbg !13052
  %289 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %221, <8 x float> zeroinitializer, i8 0), !dbg !13056
  %290 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_98.i42.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13062
  %291 = bitcast <8 x float> %289 to <8 x i32>, !dbg !13068
  %292 = bitcast <8 x float> %290 to <8 x i32>, !dbg !13068
  %293 = or <8 x i32> %292, %291, !dbg !13072
  %294 = fmul <8 x float> %lanes.i582.sroa.0.0.copyload.i.i, %288, !dbg !13074
  %295 = icmp slt <8 x i32> %293, zeroinitializer, !dbg !13080
  %296 = select <8 x i1> %295, <8 x float> %lanes.i582.sroa.0.0.copyload.i.i, <8 x float> %294, !dbg !13080
  %_41.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %37, align 32, !dbg !13085, !alias.scope !12151, !noalias !13086
  %297 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_41.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13088
  %298 = bitcast <8 x float> %297 to <8 x i32>, !dbg !13094
  %299 = icmp slt <8 x i32> %298, zeroinitializer, !dbg !13098
  %lanes.i558.sroa.0.0.vec.insert.i.i = insertelement <8 x i32> poison, i32 %_85.i2313.i.i, i64 0, !dbg !13100
  %lanes.i558.sroa.0.4.vec.insert.i.i = insertelement <8 x i32> %lanes.i558.sroa.0.0.vec.insert.i.i, i32 %_85.i.12317.i.i, i64 1, !dbg !13100
  %lanes.i558.sroa.0.8.vec.insert.i.i = insertelement <8 x i32> %lanes.i558.sroa.0.4.vec.insert.i.i, i32 %_85.i.22321.i.i, i64 2, !dbg !13100
  %lanes.i558.sroa.0.12.vec.insert.i.i = insertelement <8 x i32> %lanes.i558.sroa.0.8.vec.insert.i.i, i32 %_85.i.32325.i.i, i64 3, !dbg !13100
  %lanes.i558.sroa.0.16.vec.insert.i.i = insertelement <8 x i32> %lanes.i558.sroa.0.12.vec.insert.i.i, i32 %_85.i.42329.i.i, i64 4, !dbg !13100
  %lanes.i558.sroa.0.20.vec.insert.i.i = insertelement <8 x i32> %lanes.i558.sroa.0.16.vec.insert.i.i, i32 %_85.i.52333.i.i, i64 5, !dbg !13100
  %lanes.i558.sroa.0.24.vec.insert.i.i = insertelement <8 x i32> %lanes.i558.sroa.0.20.vec.insert.i.i, i32 %_85.i.62337.i.i, i64 6, !dbg !13100
  %lanes.i558.sroa.0.28.vec.insert.i.i = insertelement <8 x i32> %lanes.i558.sroa.0.24.vec.insert.i.i, i32 %_85.i.72341.i.i, i64 7, !dbg !13100
  %300 = and <8 x i32> %lanes.i558.sroa.0.28.vec.insert.i.i, splat (i32 2147483647), !dbg !13105
  %301 = bitcast <8 x i32> %300 to <8 x float>, !dbg !13111
  %302 = fmul <8 x float> %301, splat (float 5.000000e-01), !dbg !13112
  %lanes.i.sroa.0.0.vec.insert.i.i = insertelement <8 x i32> poison, i32 %_87.i2314.i.i, i64 0, !dbg !13117
  %lanes.i.sroa.0.4.vec.insert.i.i = insertelement <8 x i32> %lanes.i.sroa.0.0.vec.insert.i.i, i32 %_87.i.12318.i.i, i64 1, !dbg !13117
  %lanes.i.sroa.0.8.vec.insert.i.i = insertelement <8 x i32> %lanes.i.sroa.0.4.vec.insert.i.i, i32 %_87.i.22322.i.i, i64 2, !dbg !13117
  %lanes.i.sroa.0.12.vec.insert.i.i = insertelement <8 x i32> %lanes.i.sroa.0.8.vec.insert.i.i, i32 %_87.i.32326.i.i, i64 3, !dbg !13117
  %lanes.i.sroa.0.16.vec.insert.i.i = insertelement <8 x i32> %lanes.i.sroa.0.12.vec.insert.i.i, i32 %_87.i.42330.i.i, i64 4, !dbg !13117
  %lanes.i.sroa.0.20.vec.insert.i.i = insertelement <8 x i32> %lanes.i.sroa.0.16.vec.insert.i.i, i32 %_87.i.52334.i.i, i64 5, !dbg !13117
  %lanes.i.sroa.0.24.vec.insert.i.i = insertelement <8 x i32> %lanes.i.sroa.0.20.vec.insert.i.i, i32 %_87.i.62338.i.i, i64 6, !dbg !13117
  %lanes.i.sroa.0.28.vec.insert.i.i = insertelement <8 x i32> %lanes.i.sroa.0.24.vec.insert.i.i, i32 %_87.i.72342.i.i, i64 7, !dbg !13117
  %303 = and <8 x i32> %lanes.i.sroa.0.28.vec.insert.i.i, splat (i32 2147483647), !dbg !13122
  %304 = bitcast <8 x i32> %303 to <8 x float>, !dbg !13128
  %305 = fmul <8 x float> %304, splat (float 5.000000e-01), !dbg !13129
  %306 = fadd <8 x float> %302, %305, !dbg !13134
  %_37.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %36, align 32, !dbg !13139, !alias.scope !12151, !noalias !13086
  %307 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_37.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13140
  %308 = bitcast <8 x float> %307 to <8 x i32>, !dbg !13146
  %309 = icmp slt <8 x i32> %308, zeroinitializer, !dbg !13150
  %310 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %301, <8 x float> %304), !dbg !13152
  %311 = select <8 x i1> %309, <8 x float> %310, <8 x float> %301, !dbg !13150
  %312 = select <8 x i1> %299, <8 x float> %306, <8 x float> %311, !dbg !13098
  %313 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %312, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !13157
  %314 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %313, <8 x float> splat (float 0x3810000000000000)), !dbg !13162
  %315 = bitcast <8 x float> %314 to <4 x i64>, !dbg !13169
  %316 = and <4 x i64> %315, splat (i64 36028792732385279), !dbg !13170
  %317 = or disjoint <4 x i64> %316, splat (i64 4575657222473777152), !dbg !13175
  %318 = bitcast <4 x i64> %317 to <8 x float>, !dbg !13179
  %319 = fadd <8 x float> %318, splat (float -1.000000e+00), !dbg !13180
  %320 = fmul <8 x float> %319, splat (float 0x3F9B17A960000000), !dbg !13185
  %321 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %320, !dbg !13190
  %322 = fmul <8 x float> %319, %321, !dbg !13185
  %323 = fadd <8 x float> %322, splat (float 0xBFD1E3F400000000), !dbg !13190
  %324 = fmul <8 x float> %319, %323, !dbg !13185
  %325 = fadd <8 x float> %324, splat (float 0x3FDD544F20000000), !dbg !13190
  %326 = fmul <8 x float> %319, %325, !dbg !13185
  %327 = fadd <8 x float> %326, splat (float 0xBFE6FC2A60000000), !dbg !13190
  %328 = fmul <8 x float> %319, %327, !dbg !13185
  %329 = fadd <8 x float> %328, splat (float 0x3FF714B2A0000000), !dbg !13190
  %330 = bitcast <8 x float> %314 to <8 x i32>, !dbg !13195
  %_3.i1004.i.i = lshr <8 x i32> %330, splat (i32 23), !dbg !13199
  %331 = or disjoint <8 x i32> %_3.i1004.i.i, splat (i32 1258291200), !dbg !13200
  %332 = bitcast <8 x i32> %331 to <8 x float>, !dbg !13204
  %333 = fadd <8 x float> %332, splat (float 0xC160000FE0000000), !dbg !13205
  %334 = fmul <8 x float> %319, %329, !dbg !13209
  %335 = fadd <8 x float> %333, %334, !dbg !13214
  %336 = fmul <8 x float> %335, splat (float 0x4018151820000000), !dbg !13219
  %337 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %336, <8 x float> splat (float 2.400000e+01)), !dbg !13224
  %338 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %337, <8 x float> splat (float -1.600000e+02)), !dbg !13229
  %_55.i26.sroa.0.0.copyload.i.i = load <8 x float>, ptr %35, align 32, !dbg !13234, !alias.scope !12151, !noalias !12963
  %339 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_55.i26.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13235
  %340 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %338, <8 x float> %243, i8 29), !dbg !13241
  %341 = fsub <8 x float> %243, %279, !dbg !13247
  %342 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %338, <8 x float> %341, i8 29), !dbg !13252
  %343 = bitcast <8 x float> %339 to <8 x i32>, !dbg !13258
  %344 = xor <8 x i32> %343, splat (i32 -1), !dbg !13264
  %345 = bitcast <8 x float> %340 to <8 x i32>, !dbg !13266
  %346 = and <8 x i32> %345, %344, !dbg !13270
  %347 = bitcast <8 x float> %342 to <8 x i32>, !dbg !13272
  %348 = and <8 x i32> %347, %343, !dbg !13276
  %349 = or <8 x i32> %348, %346, !dbg !13278
  %350 = xor <8 x i32> %347, splat (i32 -1), !dbg !13283
  %_67.i22.sroa.0.0.copyload.i.i = load <8 x float>, ptr %38, align 32, !dbg !13290, !alias.scope !12151, !noalias !12963
  %351 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_67.i22.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13291
  %352 = bitcast <8 x float> %351 to <8 x i32>, !dbg !13297
  %353 = and <8 x i32> %350, %352, !dbg !13301
  %354 = and <8 x i32> %353, %343, !dbg !13301
  %355 = or <8 x i32> %354, %349, !dbg !13306
  %356 = icmp slt <8 x i32> %355, zeroinitializer, !dbg !13311
  %357 = select <8 x i1> %356, <8 x float> splat (float 1.000000e+00), <8 x float> zeroinitializer, !dbg !13311
  %_71.i20.sroa.0.0.copyload.i.i = load <8 x float>, ptr %39, align 32, !dbg !13316, !alias.scope !12151, !noalias !13086
  %358 = fadd <8 x float> %_67.i22.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !13317
  %359 = icmp slt <8 x i32> %354, zeroinitializer, !dbg !13322
  %360 = select <8 x i1> %359, <8 x float> %358, <8 x float> %_67.i22.sroa.0.0.copyload.i.i, !dbg !13322
  %361 = icmp slt <8 x i32> %349, zeroinitializer, !dbg !13327
  %362 = select <8 x i1> %361, <8 x float> %_71.i20.sroa.0.0.copyload.i.i, <8 x float> %360, !dbg !13327
  store <8 x float> %362, ptr %38, align 32, !dbg !13332, !alias.scope !12151, !noalias !12963
  store <8 x float> %357, ptr %35, align 32, !dbg !13333, !alias.scope !12151, !noalias !12963
  %_86.i11.sroa.0.0.copyload.i.i = load <8 x float>, ptr %40, align 32, !dbg !13334, !alias.scope !12151, !noalias !12963
  %_88.i10.sroa.0.0.copyload.i.i = load <8 x float>, ptr %41, align 32, !dbg !13335, !alias.scope !12151, !noalias !13086
  %_7.i894.i.i = load <8 x float>, ptr %_32.i.i, align 32, !dbg !13336, !alias.scope !13338, !noalias !13341
  %363 = fadd <8 x float> %255, splat (float -1.000000e+00), !dbg !13345
  %364 = fsub <8 x float> %338, %243, !dbg !13350
  %365 = fmul <8 x float> %363, %364, !dbg !13355
  %366 = fneg <8 x float> %267, !dbg !13360
  %367 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %365, <8 x float> %366), !dbg !13365
  %368 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %367, <8 x float> zeroinitializer), !dbg !13370
  %369 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %357, <8 x float> zeroinitializer, i8 30), !dbg !13375
  %370 = bitcast <8 x float> %369 to <8 x i32>, !dbg !13381
  %371 = icmp slt <8 x i32> %370, zeroinitializer, !dbg !13385
  %372 = select <8 x i1> %371, <8 x float> zeroinitializer, <8 x float> %368, !dbg !13385
  %373 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %372, <8 x float> %_86.i11.sroa.0.0.copyload.i.i, i8 30), !dbg !13387
  %374 = bitcast <8 x float> %373 to <8 x i32>, !dbg !13393
  %375 = icmp slt <8 x i32> %374, zeroinitializer, !dbg !13396
  %376 = select <8 x i1> %375, <8 x float> %_7.i894.i.i, <8 x float> %_88.i10.sroa.0.0.copyload.i.i, !dbg !13396
  %377 = fsub <8 x float> %372, %_86.i11.sroa.0.0.copyload.i.i, !dbg !13398
  %378 = fmul <8 x float> %377, %376, !dbg !13403
  %379 = fadd <8 x float> %_86.i11.sroa.0.0.copyload.i.i, %378, !dbg !13408
  %380 = bitcast <8 x float> %379 to <8 x i32>, !dbg !13412
  %381 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %379), !dbg !13418
  %382 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %381, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !13420
  %383 = bitcast <8 x float> %382 to <8 x i32>, !dbg !13426
  %384 = xor <8 x i32> %383, splat (i32 -1), !dbg !13432
  %385 = and <8 x i32> %380, %384, !dbg !13434
  store <8 x i32> %385, ptr %40, align 32, !dbg !13438, !alias.scope !12151, !noalias !12963
  %386 = bitcast <8 x i32> %385 to <8 x float>, !dbg !13439
  %387 = fmul <8 x float> %386, splat (float 0x3FC542A5A0000000), !dbg !13440
  %388 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %387, <8 x float> splat (float -1.260000e+02)), !dbg !13446
  %389 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %388, <8 x float> splat (float 1.270000e+02)), !dbg !13452
  %390 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %389), !dbg !13457
  %391 = fsub <8 x float> %389, %390, !dbg !13462
  %392 = fmul <8 x float> %391, splat (float 0x3F5E974FA0000000), !dbg !13467
  %393 = fadd <8 x float> %392, splat (float 0x3F82778560000000), !dbg !13472
  %394 = fmul <8 x float> %391, %393, !dbg !13467
  %395 = fadd <8 x float> %394, splat (float 0x3FAC91CE60000000), !dbg !13472
  %396 = fmul <8 x float> %391, %395, !dbg !13467
  %397 = fadd <8 x float> %396, splat (float 0x3FCEBDB560000000), !dbg !13472
  %398 = fmul <8 x float> %391, %397, !dbg !13467
  %399 = fadd <8 x float> %398, splat (float 0x3FE62E4BA0000000), !dbg !13472
  %400 = fmul <8 x float> %391, %399, !dbg !13477
  %401 = fadd <8 x float> %400, splat (float 1.000000e+00), !dbg !13482
  %402 = fadd <8 x float> %390, splat (float 0x4160000FE0000000), !dbg !13487
  %403 = bitcast <8 x float> %402 to <8 x i32>, !dbg !13492
  %_3.i1005.i.i = shl <8 x i32> %403, splat (i32 23), !dbg !13496
  %404 = bitcast <8 x i32> %_3.i1005.i.i to <8 x float>, !dbg !13497
  %405 = fmul <8 x float> %401, %404, !dbg !13499
  %406 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %386, <8 x float> zeroinitializer, i8 0), !dbg !13503
  %_98.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %42, align 32, !dbg !13509, !alias.scope !12151, !noalias !13086
  %407 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_98.i.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !13510
  %408 = bitcast <8 x float> %406 to <8 x i32>, !dbg !13516
  %409 = bitcast <8 x float> %407 to <8 x i32>, !dbg !13516
  %410 = or <8 x i32> %409, %408, !dbg !13520
  %411 = fmul <8 x float> %lanes.i576.sroa.0.0.copyload.i.i, %405, !dbg !13522
  %412 = icmp slt <8 x i32> %410, zeroinitializer, !dbg !13527
  %413 = select <8 x i1> %412, <8 x float> %lanes.i576.sroa.0.0.copyload.i.i, <8 x float> %411, !dbg !13527
  store <8 x float> %296, ptr %_123.i.i.i, align 4, !dbg !13532, !alias.scope !13538, !noalias !13542
  store <8 x float> %413, ptr %_141.i.i.i, align 4, !dbg !13546, !alias.scope !13551, !noalias !13555
  %exitcond1980.not.i.i = icmp eq i64 %81, %..i.i, !dbg !13559
  br i1 %exitcond1980.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKb1_EB3_.exit.i, label %bb42.i.i.i, !dbg !12173

panic18.i.i.i:                                    ; preds = %bb28.i.6.i.i, %bb28.i.5.i.i, %bb28.i.4.i.i, %bb28.i.3.i.i, %bb28.i.2.i.i, %bb28.i.1.i.i, %bb28.i.i.i, %bb63.i.i.i
  %own.i.lcssa.i.i = phi i64 [ %_63.i.i.i, %bb63.i.i.i ], [ %own.i.1.i.i, %bb28.i.i.i ], [ %own.i.2.i.i, %bb28.i.1.i.i ], [ %own.i.3.i.i, %bb28.i.2.i.i ], [ %own.i.4.i.i, %bb28.i.3.i.i ], [ %own.i.5.i.i, %bb28.i.4.i.i ], [ %own.i.6.i.i, %bb28.i.5.i.i ], [ %own.i.7.i.i, %bb28.i.6.i.i ], !dbg !12314
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %own.i.lcssa.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b53d553f9945f7702333f7af20204159) #24, !dbg !12321, !noalias !12170
  unreachable, !dbg !12321

bb22.i.i.i:                                       ; preds = %bb63.i.i.i
  %414 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_63.i.i.i, !dbg !12321
  %_78.i2311.i.i = load i32, ptr %414, align 4, !dbg !12321, !noalias !12170, !noundef !12
  %_84.i.i.i = icmp ult i64 %_63.i.i.i, %_60.1.i.i, !dbg !13562
  br i1 %_84.i.i.i, label %bb24.i.i.i, label %panic20.i.i.i, !dbg !13562

panic20.i.i.i:                                    ; preds = %bb22.i.7.i.i, %bb22.i.6.i.i, %bb22.i.5.i.i, %bb22.i.4.i.i, %bb22.i.3.i.i, %bb22.i.2.i.i, %bb22.i.1.i.i, %bb22.i.i.i
  %own.i.lcssa1889.i.i = phi i64 [ %_63.i.i.i, %bb22.i.i.i ], [ %own.i.1.i.i, %bb22.i.1.i.i ], [ %own.i.2.i.i, %bb22.i.2.i.i ], [ %own.i.3.i.i, %bb22.i.3.i.i ], [ %own.i.4.i.i, %bb22.i.4.i.i ], [ %own.i.5.i.i, %bb22.i.5.i.i ], [ %own.i.6.i.i, %bb22.i.6.i.i ], [ %own.i.7.i.i, %bb22.i.7.i.i ], !dbg !12314
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %own.i.lcssa1889.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30bf8a301493a607abcc78dbf93977e0) #24, !dbg !13562, !noalias !12170
  unreachable, !dbg !13562

bb24.i.i.i:                                       ; preds = %bb22.i.i.i
  %415 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_63.i.i.i, !dbg !13562
  %_82.i2312.i.i = load i32, ptr %415, align 4, !dbg !13562, !noalias !12170, !noundef !12
  %_86.i.i.i = icmp ult i64 %_71.i.i.i, %_60.1.i.i, !dbg !12323
  br i1 %_86.i.i.i, label %bb26.i.i.i, label %panic22.i.i.i, !dbg !12323

panic22.i.i.i:                                    ; preds = %bb24.i.7.i.i, %bb24.i.6.i.i, %bb24.i.5.i.i, %bb24.i.4.i.i, %bb24.i.3.i.i, %bb24.i.2.i.i, %bb24.i.1.i.i, %bb24.i.i.i
  %partner.i.lcssa1886.i.i = phi i64 [ %_71.i.i.i, %bb24.i.i.i ], [ %partner.i.1.i.i, %bb24.i.1.i.i ], [ %partner.i.2.i.i, %bb24.i.2.i.i ], [ %partner.i.3.i.i, %bb24.i.3.i.i ], [ %partner.i.4.i.i, %bb24.i.4.i.i ], [ %partner.i.5.i.i, %bb24.i.5.i.i ], [ %partner.i.6.i.i, %bb24.i.6.i.i ], [ %partner.i.7.i.i, %bb24.i.7.i.i ], !dbg !12320
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %partner.i.lcssa1886.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5833dd3f10f38ea96ec24c6054ff083c) #24, !dbg !12323, !noalias !12170
  unreachable, !dbg !12323

bb26.i.i.i:                                       ; preds = %bb24.i.i.i
  %416 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_71.i.i.i, !dbg !12323
  %_85.i2313.i.i = load i32, ptr %416, align 4, !dbg !12323, !noalias !12170, !noundef !12
  %_88.i.i.i = icmp ult i64 %_71.i.i.i, %_58.1.i.i, !dbg !12324
  br i1 %_88.i.i.i, label %bb28.i.i.i, label %panic24.i.i.i, !dbg !12324

panic24.i.i.i:                                    ; preds = %bb26.i.7.i.i, %bb26.i.6.i.i, %bb26.i.5.i.i, %bb26.i.4.i.i, %bb26.i.3.i.i, %bb26.i.2.i.i, %bb26.i.1.i.i, %bb26.i.i.i
  %partner.i.lcssa1887.i.i = phi i64 [ %_71.i.i.i, %bb26.i.i.i ], [ %partner.i.1.i.i, %bb26.i.1.i.i ], [ %partner.i.2.i.i, %bb26.i.2.i.i ], [ %partner.i.3.i.i, %bb26.i.3.i.i ], [ %partner.i.4.i.i, %bb26.i.4.i.i ], [ %partner.i.5.i.i, %bb26.i.5.i.i ], [ %partner.i.6.i.i, %bb26.i.6.i.i ], [ %partner.i.7.i.i, %bb26.i.7.i.i ], !dbg !12320
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %partner.i.lcssa1887.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5ae5decb60097d7e1183cb43cae6b3f) #24, !dbg !12324, !noalias !12170
  unreachable, !dbg !12324

bb28.i.i.i:                                       ; preds = %bb26.i.i.i
  %417 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_71.i.i.i, !dbg !12324
  %_87.i2314.i.i = load i32, ptr %417, align 4, !dbg !12324, !noalias !12170, !noundef !12
  %_67.i.1.i.i = load i32, ptr %43, align 4, !dbg !12305, !alias.scope !12151, !noalias !12170, !noundef !12
  %_66.i.1.i.i = sub i32 %now.i.i.i, %_67.i.1.i.i, !dbg !12311
  %_65.i.1.i.i = and i32 %_66.i.1.i.i, %_52.i.i, !dbg !12313
  %_64.i.1.i.i = zext i32 %_65.i.1.i.i to i64, !dbg !12314
  %_63.i.1.i.i = shl nuw nsw i64 %_64.i.1.i.i, 3, !dbg !12314
  %own.i.1.i.i = or disjoint i64 %_63.i.1.i.i, 1, !dbg !12314
  %_75.i.1.i.i = load i32, ptr %44, align 4, !dbg !12315, !alias.scope !12151, !noalias !12170, !noundef !12
  %_74.i.1.i.i = sub i32 %now.i.i.i, %_75.i.1.i.i, !dbg !12317
  %_73.i.1.i.i = and i32 %_74.i.1.i.i, %_52.i.i, !dbg !12319
  %_72.i.1.i.i = zext i32 %_73.i.1.i.i to i64, !dbg !12320
  %_71.i.1.i.i = shl nuw nsw i64 %_72.i.1.i.i, 3, !dbg !12320
  %partner.i.1.i.i = or disjoint i64 %_71.i.1.i.i, 1, !dbg !12320
  %_80.i.1.i.i = icmp ult i64 %own.i.1.i.i, %_58.1.i.i, !dbg !12321
  br i1 %_80.i.1.i.i, label %bb22.i.1.i.i, label %panic18.i.i.i, !dbg !12321

bb22.i.1.i.i:                                     ; preds = %bb28.i.i.i
  %418 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.1.i.i, !dbg !12321
  %_78.i.12315.i.i = load i32, ptr %418, align 4, !dbg !12321, !noalias !12170, !noundef !12
  %_84.i.1.i.i = icmp ult i64 %own.i.1.i.i, %_60.1.i.i, !dbg !13562
  br i1 %_84.i.1.i.i, label %bb24.i.1.i.i, label %panic20.i.i.i, !dbg !13562

bb24.i.1.i.i:                                     ; preds = %bb22.i.1.i.i
  %419 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.1.i.i, !dbg !13562
  %_82.i.12316.i.i = load i32, ptr %419, align 4, !dbg !13562, !noalias !12170, !noundef !12
  %_86.i.1.i.i = icmp ult i64 %partner.i.1.i.i, %_60.1.i.i, !dbg !12323
  br i1 %_86.i.1.i.i, label %bb26.i.1.i.i, label %panic22.i.i.i, !dbg !12323

bb26.i.1.i.i:                                     ; preds = %bb24.i.1.i.i
  %420 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.1.i.i, !dbg !12323
  %_85.i.12317.i.i = load i32, ptr %420, align 4, !dbg !12323, !noalias !12170, !noundef !12
  %_88.i.1.i.i = icmp ult i64 %partner.i.1.i.i, %_58.1.i.i, !dbg !12324
  br i1 %_88.i.1.i.i, label %bb28.i.1.i.i, label %panic24.i.i.i, !dbg !12324

bb28.i.1.i.i:                                     ; preds = %bb26.i.1.i.i
  %421 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.1.i.i, !dbg !12324
  %_87.i.12318.i.i = load i32, ptr %421, align 4, !dbg !12324, !noalias !12170, !noundef !12
  %_67.i.2.i.i = load i32, ptr %45, align 4, !dbg !12305, !alias.scope !12151, !noalias !12170, !noundef !12
  %_66.i.2.i.i = sub i32 %now.i.i.i, %_67.i.2.i.i, !dbg !12311
  %_65.i.2.i.i = and i32 %_66.i.2.i.i, %_52.i.i, !dbg !12313
  %_64.i.2.i.i = zext i32 %_65.i.2.i.i to i64, !dbg !12314
  %_63.i.2.i.i = shl nuw nsw i64 %_64.i.2.i.i, 3, !dbg !12314
  %own.i.2.i.i = or disjoint i64 %_63.i.2.i.i, 2, !dbg !12314
  %_75.i.2.i.i = load i32, ptr %46, align 4, !dbg !12315, !alias.scope !12151, !noalias !12170, !noundef !12
  %_74.i.2.i.i = sub i32 %now.i.i.i, %_75.i.2.i.i, !dbg !12317
  %_73.i.2.i.i = and i32 %_74.i.2.i.i, %_52.i.i, !dbg !12319
  %_72.i.2.i.i = zext i32 %_73.i.2.i.i to i64, !dbg !12320
  %_71.i.2.i.i = shl nuw nsw i64 %_72.i.2.i.i, 3, !dbg !12320
  %partner.i.2.i.i = or disjoint i64 %_71.i.2.i.i, 2, !dbg !12320
  %_80.i.2.i.i = icmp ult i64 %own.i.2.i.i, %_58.1.i.i, !dbg !12321
  br i1 %_80.i.2.i.i, label %bb22.i.2.i.i, label %panic18.i.i.i, !dbg !12321

bb22.i.2.i.i:                                     ; preds = %bb28.i.1.i.i
  %422 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.2.i.i, !dbg !12321
  %_78.i.22319.i.i = load i32, ptr %422, align 4, !dbg !12321, !noalias !12170, !noundef !12
  %_84.i.2.i.i = icmp ult i64 %own.i.2.i.i, %_60.1.i.i, !dbg !13562
  br i1 %_84.i.2.i.i, label %bb24.i.2.i.i, label %panic20.i.i.i, !dbg !13562

bb24.i.2.i.i:                                     ; preds = %bb22.i.2.i.i
  %423 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.2.i.i, !dbg !13562
  %_82.i.22320.i.i = load i32, ptr %423, align 4, !dbg !13562, !noalias !12170, !noundef !12
  %_86.i.2.i.i = icmp ult i64 %partner.i.2.i.i, %_60.1.i.i, !dbg !12323
  br i1 %_86.i.2.i.i, label %bb26.i.2.i.i, label %panic22.i.i.i, !dbg !12323

bb26.i.2.i.i:                                     ; preds = %bb24.i.2.i.i
  %424 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.2.i.i, !dbg !12323
  %_85.i.22321.i.i = load i32, ptr %424, align 4, !dbg !12323, !noalias !12170, !noundef !12
  %_88.i.2.i.i = icmp ult i64 %partner.i.2.i.i, %_58.1.i.i, !dbg !12324
  br i1 %_88.i.2.i.i, label %bb28.i.2.i.i, label %panic24.i.i.i, !dbg !12324

bb28.i.2.i.i:                                     ; preds = %bb26.i.2.i.i
  %425 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.2.i.i, !dbg !12324
  %_87.i.22322.i.i = load i32, ptr %425, align 4, !dbg !12324, !noalias !12170, !noundef !12
  %_67.i.3.i.i = load i32, ptr %47, align 4, !dbg !12305, !alias.scope !12151, !noalias !12170, !noundef !12
  %_66.i.3.i.i = sub i32 %now.i.i.i, %_67.i.3.i.i, !dbg !12311
  %_65.i.3.i.i = and i32 %_66.i.3.i.i, %_52.i.i, !dbg !12313
  %_64.i.3.i.i = zext i32 %_65.i.3.i.i to i64, !dbg !12314
  %_63.i.3.i.i = shl nuw nsw i64 %_64.i.3.i.i, 3, !dbg !12314
  %own.i.3.i.i = or disjoint i64 %_63.i.3.i.i, 3, !dbg !12314
  %_75.i.3.i.i = load i32, ptr %48, align 4, !dbg !12315, !alias.scope !12151, !noalias !12170, !noundef !12
  %_74.i.3.i.i = sub i32 %now.i.i.i, %_75.i.3.i.i, !dbg !12317
  %_73.i.3.i.i = and i32 %_74.i.3.i.i, %_52.i.i, !dbg !12319
  %_72.i.3.i.i = zext i32 %_73.i.3.i.i to i64, !dbg !12320
  %_71.i.3.i.i = shl nuw nsw i64 %_72.i.3.i.i, 3, !dbg !12320
  %partner.i.3.i.i = or disjoint i64 %_71.i.3.i.i, 3, !dbg !12320
  %_80.i.3.i.i = icmp ult i64 %own.i.3.i.i, %_58.1.i.i, !dbg !12321
  br i1 %_80.i.3.i.i, label %bb22.i.3.i.i, label %panic18.i.i.i, !dbg !12321

bb22.i.3.i.i:                                     ; preds = %bb28.i.2.i.i
  %426 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.3.i.i, !dbg !12321
  %_78.i.32323.i.i = load i32, ptr %426, align 4, !dbg !12321, !noalias !12170, !noundef !12
  %_84.i.3.i.i = icmp ult i64 %own.i.3.i.i, %_60.1.i.i, !dbg !13562
  br i1 %_84.i.3.i.i, label %bb24.i.3.i.i, label %panic20.i.i.i, !dbg !13562

bb24.i.3.i.i:                                     ; preds = %bb22.i.3.i.i
  %427 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.3.i.i, !dbg !13562
  %_82.i.32324.i.i = load i32, ptr %427, align 4, !dbg !13562, !noalias !12170, !noundef !12
  %_86.i.3.i.i = icmp ult i64 %partner.i.3.i.i, %_60.1.i.i, !dbg !12323
  br i1 %_86.i.3.i.i, label %bb26.i.3.i.i, label %panic22.i.i.i, !dbg !12323

bb26.i.3.i.i:                                     ; preds = %bb24.i.3.i.i
  %428 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.3.i.i, !dbg !12323
  %_85.i.32325.i.i = load i32, ptr %428, align 4, !dbg !12323, !noalias !12170, !noundef !12
  %_88.i.3.i.i = icmp ult i64 %partner.i.3.i.i, %_58.1.i.i, !dbg !12324
  br i1 %_88.i.3.i.i, label %bb28.i.3.i.i, label %panic24.i.i.i, !dbg !12324

bb28.i.3.i.i:                                     ; preds = %bb26.i.3.i.i
  %429 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.3.i.i, !dbg !12324
  %_87.i.32326.i.i = load i32, ptr %429, align 4, !dbg !12324, !noalias !12170, !noundef !12
  %_67.i.4.i.i = load i32, ptr %49, align 4, !dbg !12305, !alias.scope !12151, !noalias !12170, !noundef !12
  %_66.i.4.i.i = sub i32 %now.i.i.i, %_67.i.4.i.i, !dbg !12311
  %_65.i.4.i.i = and i32 %_66.i.4.i.i, %_52.i.i, !dbg !12313
  %_64.i.4.i.i = zext i32 %_65.i.4.i.i to i64, !dbg !12314
  %_63.i.4.i.i = shl nuw nsw i64 %_64.i.4.i.i, 3, !dbg !12314
  %own.i.4.i.i = or disjoint i64 %_63.i.4.i.i, 4, !dbg !12314
  %_75.i.4.i.i = load i32, ptr %50, align 4, !dbg !12315, !alias.scope !12151, !noalias !12170, !noundef !12
  %_74.i.4.i.i = sub i32 %now.i.i.i, %_75.i.4.i.i, !dbg !12317
  %_73.i.4.i.i = and i32 %_74.i.4.i.i, %_52.i.i, !dbg !12319
  %_72.i.4.i.i = zext i32 %_73.i.4.i.i to i64, !dbg !12320
  %_71.i.4.i.i = shl nuw nsw i64 %_72.i.4.i.i, 3, !dbg !12320
  %partner.i.4.i.i = or disjoint i64 %_71.i.4.i.i, 4, !dbg !12320
  %_80.i.4.i.i = icmp ult i64 %own.i.4.i.i, %_58.1.i.i, !dbg !12321
  br i1 %_80.i.4.i.i, label %bb22.i.4.i.i, label %panic18.i.i.i, !dbg !12321

bb22.i.4.i.i:                                     ; preds = %bb28.i.3.i.i
  %430 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.4.i.i, !dbg !12321
  %_78.i.42327.i.i = load i32, ptr %430, align 4, !dbg !12321, !noalias !12170, !noundef !12
  %_84.i.4.i.i = icmp ult i64 %own.i.4.i.i, %_60.1.i.i, !dbg !13562
  br i1 %_84.i.4.i.i, label %bb24.i.4.i.i, label %panic20.i.i.i, !dbg !13562

bb24.i.4.i.i:                                     ; preds = %bb22.i.4.i.i
  %431 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.4.i.i, !dbg !13562
  %_82.i.42328.i.i = load i32, ptr %431, align 4, !dbg !13562, !noalias !12170, !noundef !12
  %_86.i.4.i.i = icmp ult i64 %partner.i.4.i.i, %_60.1.i.i, !dbg !12323
  br i1 %_86.i.4.i.i, label %bb26.i.4.i.i, label %panic22.i.i.i, !dbg !12323

bb26.i.4.i.i:                                     ; preds = %bb24.i.4.i.i
  %432 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.4.i.i, !dbg !12323
  %_85.i.42329.i.i = load i32, ptr %432, align 4, !dbg !12323, !noalias !12170, !noundef !12
  %_88.i.4.i.i = icmp ult i64 %partner.i.4.i.i, %_58.1.i.i, !dbg !12324
  br i1 %_88.i.4.i.i, label %bb28.i.4.i.i, label %panic24.i.i.i, !dbg !12324

bb28.i.4.i.i:                                     ; preds = %bb26.i.4.i.i
  %433 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.4.i.i, !dbg !12324
  %_87.i.42330.i.i = load i32, ptr %433, align 4, !dbg !12324, !noalias !12170, !noundef !12
  %_67.i.5.i.i = load i32, ptr %51, align 4, !dbg !12305, !alias.scope !12151, !noalias !12170, !noundef !12
  %_66.i.5.i.i = sub i32 %now.i.i.i, %_67.i.5.i.i, !dbg !12311
  %_65.i.5.i.i = and i32 %_66.i.5.i.i, %_52.i.i, !dbg !12313
  %_64.i.5.i.i = zext i32 %_65.i.5.i.i to i64, !dbg !12314
  %_63.i.5.i.i = shl nuw nsw i64 %_64.i.5.i.i, 3, !dbg !12314
  %own.i.5.i.i = or disjoint i64 %_63.i.5.i.i, 5, !dbg !12314
  %_75.i.5.i.i = load i32, ptr %52, align 4, !dbg !12315, !alias.scope !12151, !noalias !12170, !noundef !12
  %_74.i.5.i.i = sub i32 %now.i.i.i, %_75.i.5.i.i, !dbg !12317
  %_73.i.5.i.i = and i32 %_74.i.5.i.i, %_52.i.i, !dbg !12319
  %_72.i.5.i.i = zext i32 %_73.i.5.i.i to i64, !dbg !12320
  %_71.i.5.i.i = shl nuw nsw i64 %_72.i.5.i.i, 3, !dbg !12320
  %partner.i.5.i.i = or disjoint i64 %_71.i.5.i.i, 5, !dbg !12320
  %_80.i.5.i.i = icmp ult i64 %own.i.5.i.i, %_58.1.i.i, !dbg !12321
  br i1 %_80.i.5.i.i, label %bb22.i.5.i.i, label %panic18.i.i.i, !dbg !12321

bb22.i.5.i.i:                                     ; preds = %bb28.i.4.i.i
  %434 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.5.i.i, !dbg !12321
  %_78.i.52331.i.i = load i32, ptr %434, align 4, !dbg !12321, !noalias !12170, !noundef !12
  %_84.i.5.i.i = icmp ult i64 %own.i.5.i.i, %_60.1.i.i, !dbg !13562
  br i1 %_84.i.5.i.i, label %bb24.i.5.i.i, label %panic20.i.i.i, !dbg !13562

bb24.i.5.i.i:                                     ; preds = %bb22.i.5.i.i
  %435 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.5.i.i, !dbg !13562
  %_82.i.52332.i.i = load i32, ptr %435, align 4, !dbg !13562, !noalias !12170, !noundef !12
  %_86.i.5.i.i = icmp ult i64 %partner.i.5.i.i, %_60.1.i.i, !dbg !12323
  br i1 %_86.i.5.i.i, label %bb26.i.5.i.i, label %panic22.i.i.i, !dbg !12323

bb26.i.5.i.i:                                     ; preds = %bb24.i.5.i.i
  %436 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.5.i.i, !dbg !12323
  %_85.i.52333.i.i = load i32, ptr %436, align 4, !dbg !12323, !noalias !12170, !noundef !12
  %_88.i.5.i.i = icmp ult i64 %partner.i.5.i.i, %_58.1.i.i, !dbg !12324
  br i1 %_88.i.5.i.i, label %bb28.i.5.i.i, label %panic24.i.i.i, !dbg !12324

bb28.i.5.i.i:                                     ; preds = %bb26.i.5.i.i
  %437 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.5.i.i, !dbg !12324
  %_87.i.52334.i.i = load i32, ptr %437, align 4, !dbg !12324, !noalias !12170, !noundef !12
  %_67.i.6.i.i = load i32, ptr %53, align 4, !dbg !12305, !alias.scope !12151, !noalias !12170, !noundef !12
  %_66.i.6.i.i = sub i32 %now.i.i.i, %_67.i.6.i.i, !dbg !12311
  %_65.i.6.i.i = and i32 %_66.i.6.i.i, %_52.i.i, !dbg !12313
  %_64.i.6.i.i = zext i32 %_65.i.6.i.i to i64, !dbg !12314
  %_63.i.6.i.i = shl nuw nsw i64 %_64.i.6.i.i, 3, !dbg !12314
  %own.i.6.i.i = or disjoint i64 %_63.i.6.i.i, 6, !dbg !12314
  %_75.i.6.i.i = load i32, ptr %54, align 4, !dbg !12315, !alias.scope !12151, !noalias !12170, !noundef !12
  %_74.i.6.i.i = sub i32 %now.i.i.i, %_75.i.6.i.i, !dbg !12317
  %_73.i.6.i.i = and i32 %_74.i.6.i.i, %_52.i.i, !dbg !12319
  %_72.i.6.i.i = zext i32 %_73.i.6.i.i to i64, !dbg !12320
  %_71.i.6.i.i = shl nuw nsw i64 %_72.i.6.i.i, 3, !dbg !12320
  %partner.i.6.i.i = or disjoint i64 %_71.i.6.i.i, 6, !dbg !12320
  %_80.i.6.i.i = icmp ult i64 %own.i.6.i.i, %_58.1.i.i, !dbg !12321
  br i1 %_80.i.6.i.i, label %bb22.i.6.i.i, label %panic18.i.i.i, !dbg !12321

bb22.i.6.i.i:                                     ; preds = %bb28.i.5.i.i
  %438 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.6.i.i, !dbg !12321
  %_78.i.62335.i.i = load i32, ptr %438, align 4, !dbg !12321, !noalias !12170, !noundef !12
  %_84.i.6.i.i = icmp ult i64 %own.i.6.i.i, %_60.1.i.i, !dbg !13562
  br i1 %_84.i.6.i.i, label %bb24.i.6.i.i, label %panic20.i.i.i, !dbg !13562

bb24.i.6.i.i:                                     ; preds = %bb22.i.6.i.i
  %439 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.6.i.i, !dbg !13562
  %_82.i.62336.i.i = load i32, ptr %439, align 4, !dbg !13562, !noalias !12170, !noundef !12
  %_86.i.6.i.i = icmp ult i64 %partner.i.6.i.i, %_60.1.i.i, !dbg !12323
  br i1 %_86.i.6.i.i, label %bb26.i.6.i.i, label %panic22.i.i.i, !dbg !12323

bb26.i.6.i.i:                                     ; preds = %bb24.i.6.i.i
  %440 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %partner.i.6.i.i, !dbg !12323
  %_85.i.62337.i.i = load i32, ptr %440, align 4, !dbg !12323, !noalias !12170, !noundef !12
  %_88.i.6.i.i = icmp ult i64 %partner.i.6.i.i, %_58.1.i.i, !dbg !12324
  br i1 %_88.i.6.i.i, label %bb28.i.6.i.i, label %panic24.i.i.i, !dbg !12324

bb28.i.6.i.i:                                     ; preds = %bb26.i.6.i.i
  %441 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %partner.i.6.i.i, !dbg !12324
  %_87.i.62338.i.i = load i32, ptr %441, align 4, !dbg !12324, !noalias !12170, !noundef !12
  %_67.i.7.i.i = load i32, ptr %55, align 4, !dbg !12305, !alias.scope !12151, !noalias !12170, !noundef !12
  %_66.i.7.i.i = sub i32 %now.i.i.i, %_67.i.7.i.i, !dbg !12311
  %_65.i.7.i.i = and i32 %_66.i.7.i.i, %_52.i.i, !dbg !12313
  %_64.i.7.i.i = zext i32 %_65.i.7.i.i to i64, !dbg !12314
  %_63.i.7.i.i = shl nuw nsw i64 %_64.i.7.i.i, 3, !dbg !12314
  %own.i.7.i.i = or disjoint i64 %_63.i.7.i.i, 7, !dbg !12314
  %_75.i.7.i.i = load i32, ptr %56, align 4, !dbg !12315, !alias.scope !12151, !noalias !12170, !noundef !12
  %_74.i.7.i.i = sub i32 %now.i.i.i, %_75.i.7.i.i, !dbg !12317
  %_73.i.7.i.i = and i32 %_74.i.7.i.i, %_52.i.i, !dbg !12319
  %_72.i.7.i.i = zext i32 %_73.i.7.i.i to i64, !dbg !12320
  %_71.i.7.i.i = shl nuw nsw i64 %_72.i.7.i.i, 3, !dbg !12320
  %partner.i.7.i.i = or disjoint i64 %_71.i.7.i.i, 7, !dbg !12320
  %_80.i.7.i.i = icmp ult i64 %own.i.7.i.i, %_58.1.i.i, !dbg !12321
  br i1 %_80.i.7.i.i, label %bb22.i.7.i.i, label %panic18.i.i.i, !dbg !12321

bb22.i.7.i.i:                                     ; preds = %bb28.i.6.i.i
  %442 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %own.i.7.i.i, !dbg !12321
  %_78.i.72339.i.i = load i32, ptr %442, align 4, !dbg !12321, !noalias !12170, !noundef !12
  %_84.i.7.i.i = icmp ult i64 %own.i.7.i.i, %_60.1.i.i, !dbg !13562
  br i1 %_84.i.7.i.i, label %bb24.i.7.i.i, label %panic20.i.i.i, !dbg !13562

bb24.i.7.i.i:                                     ; preds = %bb22.i.7.i.i
  %443 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %own.i.7.i.i, !dbg !13562
  %_82.i.72340.i.i = load i32, ptr %443, align 4, !dbg !13562, !noalias !12170, !noundef !12
  %_86.i.7.i.i = icmp ult i64 %partner.i.7.i.i, %_60.1.i.i, !dbg !12323
  br i1 %_86.i.7.i.i, label %bb26.i.7.i.i, label %panic22.i.i.i, !dbg !12323

bb26.i.7.i.i:                                     ; preds = %bb24.i.7.i.i
  %_88.i.7.i.i = icmp ult i64 %partner.i.7.i.i, %_58.1.i.i, !dbg !12324
  br i1 %_88.i.7.i.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit163.i.i, label %panic24.i.i.i, !dbg !12324

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKb1_EB3_.exit.i: ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit163.i.i
  %_107.i.i.i = add i32 %base.i.i.i, %20, !dbg !13563
  store i32 %_107.i.i.i, ptr %_51.i.i, align 4, !dbg !13565, !alias.scope !12151, !noalias !12170
  br label %bb4.i, !dbg !13566

bb7.i:                                            ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKB1r_EB3_.exit.i, %bb4.i
  %444 = load i32, ptr %14, align 4, !dbg !13567, !alias.scope !12076, !noalias !12090, !noundef !12
  %445 = sub i32 %444, %20, !dbg !13567
  store i32 %445, ptr %14, align 4, !dbg !13567, !alias.scope !12076, !noalias !12090
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13568), !dbg !13571
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13572), !dbg !13571
  call void @llvm.lifetime.start.p0(ptr nonnull %iter.i.i), !dbg !13574, !noalias !13579
  store i64 0, ptr %iter.i.i, align 8, !dbg !13574, !noalias !13579
  %_7.sroa.0.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 8, !dbg !13574
  store i64 2, ptr %_7.sroa.0.sroa.2.0.iter.sroa_idx.i.i, align 8, !dbg !13574, !noalias !13579
  %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 16, !dbg !13574
  store ptr %_37.0, ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, align 8, !dbg !13574, !noalias !13579
  %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 24, !dbg !13574
  store i64 %_37.1, ptr %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i, align 8, !dbg !13574, !noalias !13579
  %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 32, !dbg !13574
  store ptr %_38.0, ptr %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i, align 8, !dbg !13574, !noalias !13579
  %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 40, !dbg !13574
  store i64 %_38.1, ptr %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i, align 8, !dbg !13574, !noalias !13579
  %_7.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 48, !dbg !13574
  store i64 0, ptr %_7.sroa.2.0.iter.sroa_idx.i.i, align 8, !dbg !13574, !noalias !13579
  %_71228.not.i.i = icmp eq i32 %_27, 0
  %446 = getelementptr inbounds nuw i8, ptr %self, i64 2496
  %447 = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %448 = getelementptr inbounds nuw i8, ptr %self, i64 512
  %449 = getelementptr inbounds nuw i8, ptr %self, i64 2576
  %450 = getelementptr inbounds nuw i8, ptr %self, i64 2616
  %451 = getelementptr inbounds nuw i8, ptr %self, i64 768
  %452 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0)
  %453 = bitcast <8 x float> %452 to <8 x i32>
  br label %bb6.i6.i, !dbg !13582

bb6.i6.i:                                         ; preds = %bb1.backedge.i.i, %bb7.i
  %454 = phi i64 [ 24, %bb7.i ], [ 32, %bb1.backedge.i.i ]
  %_5.not.i.i.i.i.i = phi i1 [ false, %bb7.i ], [ true, %bb1.backedge.i.i ]
  %455 = phi i64 [ 0, %bb7.i ], [ 1, %bb1.backedge.i.i ]
  %self3.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<&mut [f32]>", ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, i64 %455, !dbg !13588
  %_14.0.i.i.i.i.i = load ptr, ptr %self3.i.i.i.i.i, align 8, !dbg !13593, !alias.scope !13597, !noalias !13604, !nonnull !12, !align !3484, !noundef !12
  %456 = getelementptr inbounds nuw i8, ptr %self3.i.i.i.i.i, i64 8, !dbg !13593
  %_14.1.i.i.i.i.i = load i64, ptr %456, align 8, !dbg !13593, !alias.scope !13597, !noalias !13604, !noundef !12
  %457 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x8_::f32x8>", ptr %self, i64 %455, !dbg !13606
  %458 = getelementptr inbounds nuw i8, ptr %457, i64 1856, !dbg !13606
  %_17.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %458, align 32, !dbg !13606, !alias.scope !13608, !noalias !13609
  %fst_len.i.i.i = and i64 %_14.1.i.i.i.i.i, -8, !dbg !13610
  %_22.not.i26221.i.i = icmp eq i64 %fst_len.i.i.i, 0, !dbg !13617
  br i1 %_22.not.i26221.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %bb13.i27.i.i, !dbg !13617

bb13.i27.i.i:                                     ; preds = %bb6.i6.i, %bb13.i27.i.i
  %iter.sroa.0.0.i25224.i.i = phi ptr [ %_27.i28.i.i, %bb13.i27.i.i ], [ %_14.0.i.i.i.i.i, %bb6.i6.i ]
  %iter.sroa.5.0.i24223.i.i = phi i64 [ %_28.i29.i.i, %bb13.i27.i.i ], [ %fst_len.i.i.i, %bb6.i6.i ]
  %ok.i16.sroa.0.0222.i.i = phi <8 x i32> [ %463, %bb13.i27.i.i ], [ %453, %bb6.i6.i ]
  %lanes.i.sroa.0.0.copyload.i.i = load <8 x i32>, ptr %iter.sroa.0.0.i25224.i.i, align 4, !dbg !13624, !alias.scope !13630, !noalias !13634
  %_27.i28.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i25224.i.i, i64 32, !dbg !13638
  %_28.i29.i.i = add i64 %iter.sroa.5.0.i24223.i.i, -8, !dbg !13645
  %459 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !13646
  %460 = bitcast <8 x i32> %459 to <8 x float>, !dbg !13653
  %461 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %460, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !13654
  %462 = bitcast <8 x float> %461 to <8 x i32>, !dbg !13660
  %463 = and <8 x i32> %ok.i16.sroa.0.0222.i.i, %462, !dbg !13664
  %_22.not.i26.i.i = icmp eq i64 %_28.i29.i.i, 0, !dbg !13617
  br i1 %_22.not.i26.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %bb13.i27.i.i, !dbg !13617

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i: ; preds = %bb13.i27.i.i, %bb6.i6.i
  %ok.i16.sroa.0.0.lcssa.i.i = phi <8 x i32> [ %453, %bb6.i6.i ], [ %463, %bb13.i27.i.i ], !dbg !13666
  %464 = icmp sgt <8 x i32> %ok.i16.sroa.0.0.lcssa.i.i, splat (i32 -1), !dbg !13667
  %465 = bitcast <8 x i1> %464 to i8, !dbg !13667
  %_0.i98.not.i.i = icmp eq i8 %465, 0, !dbg !13677
  br i1 %_0.i98.not.i.i, label %bb9.i.i, label %bb14.i.i, !dbg !13678

bb9.i.i:                                          ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  %466 = and <8 x i32> %_17.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !13679
  %467 = bitcast <8 x i32> %466 to <8 x float>, !dbg !13686
  %468 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %467, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !13687
  %469 = bitcast <8 x float> %468 to <8 x i32>, !dbg !13693
  %470 = and <8 x i32> %469, %453, !dbg !13697
  %471 = icmp sgt <8 x i32> %470, splat (i32 -1), !dbg !13699
  %472 = bitcast <8 x i1> %471 to i8, !dbg !13699
  %_0.i101.not.i.i = icmp eq i8 %472, 0, !dbg !13704
  br i1 %_0.i101.not.i.i, label %bb1.backedge.i.i, label %bb14.i.i, !dbg !13705

bb1.backedge.i.i:                                 ; preds = %bb17.backedge.i.i, %bb9.i.i
  br i1 %_5.not.i.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9run_blockB2_.exit, label %bb6.i6.i, !dbg !13582

bb14.i.i:                                         ; preds = %bb9.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  %fst_len.i.i.i.i = and i64 %_14.1.i.i.i.i.i, 2305843009213693944, !dbg !13706
  %_40.not71.i.i.i = icmp eq i64 %fst_len.i.i.i.i, 0, !dbg !13713
  br i1 %_40.not71.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i, label %bb23.i.i.i, !dbg !13713

bb23.i.i.i:                                       ; preds = %bb14.i.i, %bb23.i.i.i
  %iter.sroa.0.074.i.i.i = phi ptr [ %_45.i.i.i, %bb23.i.i.i ], [ %_14.0.i.i.i.i.i, %bb14.i.i ]
  %iter.sroa.5.073.i.i.i = phi i64 [ %_46.i.i.i, %bb23.i.i.i ], [ %fst_len.i.i.i.i, %bb14.i.i ]
  %ok.sroa.0.072.i.i.i = phi <8 x i32> [ %477, %bb23.i.i.i ], [ %453, %bb14.i.i ]
  %lanes.i.sroa.0.0.copyload.i.i.i = load <8 x i32>, ptr %iter.sroa.0.074.i.i.i, align 4, !dbg !13720, !alias.scope !13726, !noalias !13732
  %_45.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i.i.i, i64 32, !dbg !13736
  %_46.i.i.i = add i64 %iter.sroa.5.073.i.i.i, -8, !dbg !13743
  %473 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i.i.i, splat (i32 2147483647), !dbg !13744
  %474 = bitcast <8 x i32> %473 to <8 x float>, !dbg !13751
  %475 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %474, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !13752
  %476 = bitcast <8 x float> %475 to <8 x i32>, !dbg !13758
  %477 = and <8 x i32> %ok.sroa.0.072.i.i.i, %476, !dbg !13762
  %_40.not.i.i.i = icmp eq i64 %_46.i.i.i, 0, !dbg !13713
  br i1 %_40.not.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i, label %bb23.i.i.i, !dbg !13713

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i: ; preds = %bb23.i.i.i, %bb14.i.i
  %ok.sroa.0.0.lcssa.i.i.i = phi <8 x i32> [ %453, %bb14.i.i ], [ %477, %bb23.i.i.i ], !dbg !13764
  %478 = and <8 x i32> %_17.sroa.0.0.copyload.i.i, splat (i32 2147483647), !dbg !13765
  %479 = bitcast <8 x i32> %478 to <8 x float>, !dbg !13772
  %480 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %479, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !13773
  %481 = bitcast <8 x float> %480 to <8 x i32>, !dbg !13779
  %482 = and <8 x i32> %481, %453, !dbg !13783
  %483 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i.i.i, zeroinitializer, !dbg !13785
  %bc.i.i.i = select <8 x i1> %483, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !13785
  %484 = extractelement <8 x i32> %bc.i.i.i, i64 0, !dbg !13791
  %485 = icmp ne i32 %484, 0, !dbg !13791
  %486 = zext i1 %485 to i32, !dbg !13791
  %487 = extractelement <8 x i32> %bc.i.i.i, i64 1, !dbg !13791
  %488 = icmp eq i32 %487, 0, !dbg !13791
  %489 = select i1 %488, i32 0, i32 2, !dbg !13791
  %490 = extractelement <8 x i32> %bc.i.i.i, i64 2, !dbg !13791
  %491 = icmp eq i32 %490, 0, !dbg !13791
  %492 = select i1 %491, i32 0, i32 4, !dbg !13791
  %493 = extractelement <8 x i32> %bc.i.i.i, i64 3, !dbg !13791
  %494 = icmp eq i32 %493, 0, !dbg !13791
  %495 = select i1 %494, i32 0, i32 8, !dbg !13791
  %496 = extractelement <8 x i32> %bc.i.i.i, i64 4, !dbg !13791
  %497 = icmp eq i32 %496, 0, !dbg !13791
  %498 = select i1 %497, i32 0, i32 16, !dbg !13791
  %499 = extractelement <8 x i32> %bc.i.i.i, i64 5, !dbg !13791
  %500 = icmp eq i32 %499, 0, !dbg !13791
  %501 = select i1 %500, i32 0, i32 32, !dbg !13791
  %502 = extractelement <8 x i32> %bc.i.i.i, i64 6, !dbg !13791
  %503 = icmp eq i32 %502, 0, !dbg !13791
  %504 = select i1 %503, i32 0, i32 64, !dbg !13791
  %505 = extractelement <8 x i32> %bc.i.i.i, i64 7, !dbg !13791
  %506 = icmp eq i32 %505, 0, !dbg !13791
  %507 = select i1 %506, i32 0, i32 128, !dbg !13791
  %508 = icmp slt <8 x i32> %482, zeroinitializer, !dbg !13795
  %bc.i123.i.i = select <8 x i1> %508, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !13795
  %509 = extractelement <8 x i32> %bc.i123.i.i, i64 0, !dbg !13800
  %510 = icmp ne i32 %509, 0, !dbg !13800
  %511 = zext i1 %510 to i32, !dbg !13800
  %512 = extractelement <8 x i32> %bc.i123.i.i, i64 1, !dbg !13800
  %513 = icmp eq i32 %512, 0, !dbg !13800
  %514 = select i1 %513, i32 0, i32 2, !dbg !13800
  %515 = extractelement <8 x i32> %bc.i123.i.i, i64 2, !dbg !13800
  %516 = icmp eq i32 %515, 0, !dbg !13800
  %517 = select i1 %516, i32 0, i32 4, !dbg !13800
  %518 = extractelement <8 x i32> %bc.i123.i.i, i64 3, !dbg !13800
  %519 = icmp eq i32 %518, 0, !dbg !13800
  %520 = select i1 %519, i32 0, i32 8, !dbg !13800
  %521 = extractelement <8 x i32> %bc.i123.i.i, i64 4, !dbg !13800
  %522 = icmp eq i32 %521, 0, !dbg !13800
  %523 = select i1 %522, i32 0, i32 16, !dbg !13800
  %524 = extractelement <8 x i32> %bc.i123.i.i, i64 5, !dbg !13800
  %525 = icmp eq i32 %524, 0, !dbg !13800
  %526 = select i1 %525, i32 0, i32 32, !dbg !13800
  %527 = extractelement <8 x i32> %bc.i123.i.i, i64 6, !dbg !13800
  %528 = icmp eq i32 %527, 0, !dbg !13800
  %529 = select i1 %528, i32 0, i32 64, !dbg !13800
  %530 = extractelement <8 x i32> %bc.i123.i.i, i64 7, !dbg !13800
  %531 = icmp eq i32 %530, 0, !dbg !13800
  %532 = select i1 %531, i32 0, i32 128, !dbg !13800
  %mask.sroa.0.1.1.i125.i.i = or disjoint i32 %489, %486, !dbg !13800
  %mask.sroa.0.1.2.i127.i.i = or disjoint i32 %mask.sroa.0.1.1.i125.i.i, %492, !dbg !13800
  %mask.sroa.0.1.3.i129.i.i = or disjoint i32 %mask.sroa.0.1.2.i127.i.i, %495, !dbg !13800
  %mask.sroa.0.1.4.i131.i.i = or disjoint i32 %mask.sroa.0.1.3.i129.i.i, %498, !dbg !13800
  %mask.sroa.0.1.5.i133.i.i = or disjoint i32 %mask.sroa.0.1.4.i131.i.i, %501, !dbg !13800
  %mask.sroa.0.1.6.i135.i.i = or i32 %mask.sroa.0.1.5.i133.i.i, %504, !dbg !13800
  %mask.sroa.0.1.7.i137.i.i = or i32 %mask.sroa.0.1.6.i135.i.i, %507, !dbg !13800
  %mask.sroa.0.1.1.i.i.i = or i32 %mask.sroa.0.1.7.i137.i.i, %511, !dbg !13791
  %mask.sroa.0.1.2.i.i.i = or i32 %mask.sroa.0.1.1.i.i.i, %514, !dbg !13791
  %mask.sroa.0.1.3.i.i.i = or i32 %mask.sroa.0.1.2.i.i.i, %517, !dbg !13791
  %mask.sroa.0.1.4.i.i.i = or i32 %mask.sroa.0.1.3.i.i.i, %520, !dbg !13791
  %mask.sroa.0.1.5.i.i.i = or i32 %mask.sroa.0.1.4.i.i.i, %523, !dbg !13791
  %mask.sroa.0.1.6.i.i.i = or i32 %mask.sroa.0.1.5.i.i.i, %526, !dbg !13791
  %mask.sroa.0.1.7.i.i.i = or i32 %mask.sroa.0.1.6.i.i.i, %529, !dbg !13791
  %failed.i.i = or i32 %mask.sroa.0.1.7.i.i.i, %532, !dbg !13801
  %invariant.gep.i.i = getelementptr [8 x float], ptr %self, i64 %455, !dbg !13802
  %ring.i.i.i = getelementptr inbounds nuw %Ring, ptr %447, i64 %455
  %533 = getelementptr inbounds nuw i8, ptr %ring.i.i.i, i64 8
  %invariant.gep231.i.i = getelementptr %LaneTiming, ptr %448, i64 %455, !dbg !13805
  %534 = getelementptr inbounds nuw %Ring, ptr %self, i64 %455
  %535 = getelementptr inbounds nuw i8, ptr %534, i64 1184
  %536 = getelementptr inbounds nuw %"kernel::GateCoef<wide::f32x8_::f32x8>", ptr %451, i64 %455
  %_19.i148.i.i = getelementptr inbounds nuw i8, ptr %536, i64 64
  %_25.i.i.i = getelementptr inbounds nuw i8, ptr %536, i64 32
  %537 = getelementptr inbounds nuw %"kernel::GateCoef<wide::f32x8_::f32x8>", ptr %self, i64 %455
  %538 = getelementptr inbounds nuw i8, ptr %537, i64 832
  %539 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x8_::f32x8>", ptr %13, i64 %455
  %_17.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 576
  %_19.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 512
  %_21.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 544
  %_37.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 32
  %_39.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 64
  %_41.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 96
  %iter.sroa.0.0.ptr26.1.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 128
  %_37.1.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 160
  %_39.1.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 192
  %_41.1.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 224
  %iter.sroa.0.0.ptr26.2.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 256
  %_37.2.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 288
  %_39.2.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 320
  %_41.2.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 352
  %iter.sroa.0.0.ptr26.3.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 384
  %_37.3.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 416
  %_39.3.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 448
  %_41.3.i.i.i = getelementptr inbounds nuw i8, ptr %539, i64 480
  %540 = getelementptr inbounds nuw i8, ptr %reports, i64 %454
  br label %bb30.i.i, !dbg !13805

bb30.i.i:                                         ; preds = %bb17.backedge.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i
  %iter1.sroa.0.0230.i.i = phi i64 [ 0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit.i.i ], [ %541, %bb17.backedge.i.i ]
  %541 = add nuw nsw i64 %iter1.sroa.0.0230.i.i, 1, !dbg !13811
  %542 = trunc nuw nsw i64 %iter1.sroa.0.0230.i.i to i32, !dbg !13817
  %_35.i.i = shl nuw nsw i32 1, %542, !dbg !13817
  %_34.i.i = and i32 %_35.i.i, %failed.i.i, !dbg !13819
  %543 = icmp eq i32 %_34.i.i, 0, !dbg !13819
  br i1 %543, label %bb17.backedge.i.i, label %bb20.preheader.i.i, !dbg !13819

bb20.preheader.i.i:                               ; preds = %bb30.i.i
  br i1 %_71228.not.i.i, label %bb33.i.i, label %bb32.i.i, !dbg !13820

bb33.i.i:                                         ; preds = %bb21.i.i, %bb20.preheader.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13826), !dbg !13829
  %slots.i.i.i = load i64, ptr %446, align 32, !dbg !13832, !alias.scope !13836, !noalias !13609, !noundef !12
  %_193.not.i.i.i = icmp eq i64 %slots.i.i.i, 0, !dbg !13837
  br i1 %_193.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.lr.ph.i.i.i, !dbg !13848

bb7.lr.ph.i.i.i:                                  ; preds = %bb33.i.i
  %_23.1.i.i.i = load i64, ptr %533, align 8, !alias.scope !13836, !noalias !13609, !noundef !12
  br label %bb7.i.i.i, !dbg !13848

bb7.i.i.i:                                        ; preds = %bb3.i.i.i, %bb7.lr.ph.i.i.i
  %iter.sroa.0.04.i.i.i = phi i64 [ 0, %bb7.lr.ph.i.i.i ], [ %544, %bb3.i.i.i ]
  %_10.i.i.i = shl i64 %iter.sroa.0.04.i.i.i, 3, !dbg !13849
  %_9.i139.i.i = add nuw nsw i64 %_10.i.i.i, %iter1.sroa.0.0230.i.i, !dbg !13849
  %_12.i.i.i = icmp ult i64 %_9.i139.i.i, %_23.1.i.i.i, !dbg !13851
  br i1 %_12.i.i.i, label %bb3.i.i.i, label %panic1.i.i.i, !dbg !13851

bb3.i.i.i:                                        ; preds = %bb7.i.i.i
  %_23.0.i.i.i = load ptr, ptr %ring.i.i.i, align 8, !dbg !13851, !alias.scope !13836, !noalias !13609, !nonnull !12, !noundef !12
  %544 = add nuw i64 %iter.sroa.0.04.i.i.i, 1, !dbg !13852
  %545 = getelementptr inbounds nuw float, ptr %_23.0.i.i.i, i64 %_9.i139.i.i, !dbg !13851
  store float 0.000000e+00, ptr %545, align 4, !dbg !13851, !noalias !13858
  %exitcond.not.i.i.i = icmp eq i64 %544, %slots.i.i.i, !dbg !13837
  br i1 %exitcond.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.i.i.i, !dbg !13848

panic1.i.i.i:                                     ; preds = %bb7.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_9.i139.i.i, i64 noundef %_23.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f8f0512af3f0ba047152c3c522a44b59) #24, !dbg !13851, !noalias !13858
  unreachable, !dbg !13851

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i: ; preds = %bb3.i.i.i, %bb33.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13859), !dbg !13862
  %gep.i.i = getelementptr [2 x [8 x float]], ptr %invariant.gep.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13863
  %values.sroa.0.0.copyload.i.i.i = load float, ptr %gep.i.i, align 32, !dbg !13863, !alias.scope !13865, !noalias !13609
  %values.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 4, !dbg !13863
  %values.sroa.5.0.copyload.i.i.i = load float, ptr %values.sroa.5.0..sroa_idx.i.i.i, align 4, !dbg !13863, !alias.scope !13865, !noalias !13609
  %values.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 8, !dbg !13863
  %values.sroa.6.0.copyload.i.i.i = load float, ptr %values.sroa.6.0..sroa_idx.i.i.i, align 8, !dbg !13863, !alias.scope !13865, !noalias !13609
  %values.sroa.7.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 12, !dbg !13863
  %values.sroa.7.0.copyload.i.i.i = load float, ptr %values.sroa.7.0..sroa_idx.i.i.i, align 4, !dbg !13863, !alias.scope !13865, !noalias !13609
  %values.sroa.8.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 16, !dbg !13863
  %values.sroa.8.0.copyload.i.i.i = load float, ptr %values.sroa.8.0..sroa_idx.i.i.i, align 16, !dbg !13863, !alias.scope !13865, !noalias !13609
  %values.sroa.9.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 20, !dbg !13863
  %values.sroa.9.0.copyload.i.i.i = load float, ptr %values.sroa.9.0..sroa_idx.i.i.i, align 4, !dbg !13863, !alias.scope !13865, !noalias !13609
  %values.sroa.10.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 24, !dbg !13863
  %values.sroa.10.0.copyload.i.i.i = load float, ptr %values.sroa.10.0..sroa_idx.i.i.i, align 8, !dbg !13863, !alias.scope !13865, !noalias !13609
  %values.sroa.11.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep.i.i, i64 28, !dbg !13863
  %values.sroa.11.0.copyload.i.i.i = load float, ptr %values.sroa.11.0..sroa_idx.i.i.i, align 4, !dbg !13863, !alias.scope !13865, !noalias !13609
  %gep232.i.i = getelementptr [2 x %LaneTiming], ptr %invariant.gep231.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13866
  store float %values.sroa.11.0.copyload.i.i.i, ptr %gep232.i.i, align 16, !dbg !13866, !alias.scope !13865, !noalias !13609
  %_7.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep232.i.i, i64 4, !dbg !13866
  store float %values.sroa.8.0.copyload.i.i.i, ptr %_7.sroa.4.0..sroa_idx.i.i.i, align 4, !dbg !13866, !alias.scope !13865, !noalias !13609
  %_7.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep232.i.i, i64 8, !dbg !13866
  store float %values.sroa.9.0.copyload.i.i.i, ptr %_7.sroa.5.0..sroa_idx.i.i.i, align 8, !dbg !13866, !alias.scope !13865, !noalias !13609
  %_7.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %gep232.i.i, i64 12, !dbg !13866
  store float %values.sroa.10.0.copyload.i.i.i, ptr %_7.sroa.6.0..sroa_idx.i.i.i, align 4, !dbg !13866, !alias.scope !13865, !noalias !13609
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13867), !dbg !13870
  %sample_rate.i.i.i = load i32, ptr %449, align 8, !dbg !13871, !alias.scope !13873, !noalias !13609, !noundef !12
  %_31.i.i.i = fpext float %values.sroa.11.0.copyload.i.i.i to double, !dbg !13874
  %_32.i.i.i = uitofp i32 %sample_rate.i.i.i to double, !dbg !13877
  %_30.i.i8.i = fmul double %_31.i.i.i, %_32.i.i.i, !dbg !13879
  %_29.i.i9.i = fdiv double %_30.i.i8.i, 1.000000e+03, !dbg !13879
  %_28.i145.i.i = fadd double %_29.i.i9.i, 5.000000e-01, !dbg !13880
  %546 = tail call double @llvm.floor.f64(double %_28.i145.i.i), !dbg !13881
  %or.cond.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %546, i32 527), !dbg !13884
  %_35.i.i.i = fcmp ogt double %546, 0x41EFFFFFFFE00000
  %or.cond10.i.i.i = or i1 %or.cond.i.i.i, %_35.i.i.i, !dbg !13884
  br i1 %or.cond10.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i, label %bb19.i.i.i, !dbg !13884

bb19.i.i.i:                                       ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i
  %_45.i146.i.i = fpext float %values.sroa.9.0.copyload.i.i.i to double, !dbg !13885
  %_44.i.i.i = fmul double %_45.i146.i.i, %_32.i.i.i, !dbg !13888
  %_43.i.i.i = fdiv double %_44.i.i.i, 1.000000e+03, !dbg !13888
  %_42.i.i.i = fadd double %_43.i.i.i, 5.000000e-01, !dbg !13889
  %547 = tail call double @llvm.floor.f64(double %_42.i.i.i), !dbg !13890
  %or.cond11.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %547, i32 527), !dbg !13893
  %_48.i.i.i = fcmp ogt double %547, 0x41EFFFFFFFE00000
  %or.cond12.i.i.i = or i1 %or.cond11.i.i.i, %_48.i.i.i, !dbg !13893
  br i1 %or.cond12.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i, label %bb20.3.i.i.i, !dbg !13893

bb20.3.i.i.i:                                     ; preds = %bb19.i.i.i
  %_12.i147.i.i = load i32, ptr %450, align 8, !dbg !13894, !alias.scope !13873, !noalias !13609, !noundef !12
  %_36.i.i10.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %546), !dbg !13895
  %_49.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %547), !dbg !13896
  %548 = getelementptr inbounds nuw i32, ptr %535, i64 %iter1.sroa.0.0230.i.i, !dbg !13897
  %549 = tail call i32 @llvm.usub.sat.i32(i32 %_12.i147.i.i, i32 %_36.i.i10.i), !dbg !13897
  store i32 %549, ptr %548, align 4, !dbg !13897, !alias.scope !13873, !noalias !13609
  %_20.i.i.i = uitofp i32 %_49.i.i.i to float, !dbg !13898
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i144.i.i), !dbg !13899, !noalias !13901
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i144.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_19.i148.i.i, i64 32, i1 false), !dbg !13899, !noalias !13609
  %550 = getelementptr inbounds nuw float, ptr %words.i.i144.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13902
  store float %_20.i.i.i, ptr %550, align 4, !dbg !13902, !noalias !13903
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_19.i148.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i144.i.i, i64 32, i1 false), !dbg !13906, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i144.i.i), !dbg !13907, !noalias !13901
; call effect_runtime::envelope::attack_release_coefficient
  %_23.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.8.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !13908, !noalias !13909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i143.i.i), !dbg !13910, !noalias !13901
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i143.i.i, ptr noundef nonnull align 32 dereferenceable(32) %536, i64 32, i1 false), !dbg !13910, !noalias !13609
  %551 = getelementptr inbounds nuw float, ptr %words.i17.i143.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13912
  store float %_23.i.i.i, ptr %551, align 4, !dbg !13912, !noalias !13913
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %536, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i143.i.i, i64 32, i1 false), !dbg !13916, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i143.i.i), !dbg !13917, !noalias !13901
; call effect_runtime::envelope::attack_release_coefficient
  %_26.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.10.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i) #23, !dbg !13918, !noalias !13909
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i18.i.i.i), !dbg !13919, !noalias !13901
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i18.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_25.i.i.i, i64 32, i1 false), !dbg !13919, !noalias !13609
  %552 = getelementptr inbounds nuw float, ptr %words.i18.i.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13921
  store float %_26.i.i.i, ptr %552, align 4, !dbg !13921, !noalias !13922
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_25.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i18.i.i.i, i64 32, i1 false), !dbg !13925, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i18.i.i.i), !dbg !13926, !noalias !13901
  call void @llvm.lifetime.start.p0(ptr nonnull %_15.i141.i.i), !dbg !13927, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_15.i141.i.i, ptr noundef nonnull align 32 dereferenceable(32) %538, i64 32, i1 false), !dbg !13927, !noalias !13609
  %553 = getelementptr inbounds nuw float, ptr %_15.i141.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13929
  %_0.i.i.i.i = load float, ptr %553, align 4, !dbg !13929, !alias.scope !13931, !noalias !13928, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_15.i141.i.i), !dbg !13934, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i.i.i), !dbg !13935, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_17.i.i.i, i64 32, i1 false), !dbg !13935, !noalias !13609
  %554 = getelementptr inbounds nuw float, ptr %words.i.i.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13937
  store float 0.000000e+00, ptr %554, align 4, !dbg !13937, !noalias !13938
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i.i.i, i64 32, i1 false), !dbg !13941, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i.i.i), !dbg !13942, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i12.i.i.i), !dbg !13943, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i12.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_19.i.i.i, i64 32, i1 false), !dbg !13943, !noalias !13609
  %555 = getelementptr inbounds nuw float, ptr %words.i12.i.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13945
  store float 1.000000e+00, ptr %555, align 4, !dbg !13945, !noalias !13946
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_19.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i12.i.i.i, i64 32, i1 false), !dbg !13949, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i12.i.i.i), !dbg !13950, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i13.i.i.i), !dbg !13951, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i13.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_21.i.i.i, i64 32, i1 false), !dbg !13951, !noalias !13609
  %556 = getelementptr inbounds nuw float, ptr %words.i13.i.i.i, i64 %iter1.sroa.0.0230.i.i, !dbg !13953
  store float %_0.i.i.i.i, ptr %556, align 4, !dbg !13953, !noalias !13954
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_21.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i13.i.i.i, i64 32, i1 false), !dbg !13957, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i13.i.i.i), !dbg !13958, !noalias !13928
  %557 = getelementptr inbounds nuw float, ptr %words.i14.i.i.i, i64 %iter1.sroa.0.0230.i.i
  %558 = getelementptr inbounds nuw float, ptr %words.i15.i.i.i, i64 %iter1.sroa.0.0230.i.i
  %559 = getelementptr inbounds nuw float, ptr %words.i16.i.i.i, i64 %iter1.sroa.0.0230.i.i
  %560 = getelementptr inbounds nuw float, ptr %words.i17.i.i.i, i64 %iter1.sroa.0.0230.i.i
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !13959, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %539, i64 32, i1 false), !dbg !13959, !noalias !13609
  store float %values.sroa.0.0.copyload.i.i.i, ptr %557, align 4, !dbg !13961, !noalias !13962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %539, ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, i64 32, i1 false), !dbg !13965, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !13966, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !13967, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_37.i.i.i, i64 32, i1 false), !dbg !13967, !noalias !13609
  store float %values.sroa.0.0.copyload.i.i.i, ptr %558, align 4, !dbg !13969, !noalias !13970
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_37.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, i64 32, i1 false), !dbg !13973, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !13974, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !13975, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_39.i.i.i, i64 32, i1 false), !dbg !13975, !noalias !13609
  store float 0.000000e+00, ptr %559, align 4, !dbg !13977, !noalias !13978
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_39.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, i64 32, i1 false), !dbg !13981, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !13982, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !13983, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_41.i.i.i, i64 32, i1 false), !dbg !13983, !noalias !13609
  store float 0.000000e+00, ptr %560, align 4, !dbg !13985, !noalias !13986
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_41.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, i64 32, i1 false), !dbg !13989, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !13990, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !13959, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.1.i.i.i, i64 32, i1 false), !dbg !13959, !noalias !13609
  store float %values.sroa.5.0.copyload.i.i.i, ptr %557, align 4, !dbg !13961, !noalias !13962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.1.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, i64 32, i1 false), !dbg !13965, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !13966, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !13967, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_37.1.i.i.i, i64 32, i1 false), !dbg !13967, !noalias !13609
  store float %values.sroa.5.0.copyload.i.i.i, ptr %558, align 4, !dbg !13969, !noalias !13970
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_37.1.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, i64 32, i1 false), !dbg !13973, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !13974, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !13975, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_39.1.i.i.i, i64 32, i1 false), !dbg !13975, !noalias !13609
  store float 0.000000e+00, ptr %559, align 4, !dbg !13977, !noalias !13978
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_39.1.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, i64 32, i1 false), !dbg !13981, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !13982, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !13983, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_41.1.i.i.i, i64 32, i1 false), !dbg !13983, !noalias !13609
  store float 0.000000e+00, ptr %560, align 4, !dbg !13985, !noalias !13986
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_41.1.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, i64 32, i1 false), !dbg !13989, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !13990, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !13959, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.2.i.i.i, i64 32, i1 false), !dbg !13959, !noalias !13609
  store float %values.sroa.6.0.copyload.i.i.i, ptr %557, align 4, !dbg !13961, !noalias !13962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.2.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, i64 32, i1 false), !dbg !13965, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !13966, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !13967, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_37.2.i.i.i, i64 32, i1 false), !dbg !13967, !noalias !13609
  store float %values.sroa.6.0.copyload.i.i.i, ptr %558, align 4, !dbg !13969, !noalias !13970
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_37.2.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, i64 32, i1 false), !dbg !13973, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !13974, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !13975, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_39.2.i.i.i, i64 32, i1 false), !dbg !13975, !noalias !13609
  store float 0.000000e+00, ptr %559, align 4, !dbg !13977, !noalias !13978
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_39.2.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, i64 32, i1 false), !dbg !13981, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !13982, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !13983, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_41.2.i.i.i, i64 32, i1 false), !dbg !13983, !noalias !13609
  store float 0.000000e+00, ptr %560, align 4, !dbg !13985, !noalias !13986
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_41.2.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, i64 32, i1 false), !dbg !13989, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !13990, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i14.i.i.i), !dbg !13959, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.3.i.i.i, i64 32, i1 false), !dbg !13959, !noalias !13609
  store float %values.sroa.7.0.copyload.i.i.i, ptr %557, align 4, !dbg !13961, !noalias !13962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %iter.sroa.0.0.ptr26.3.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i14.i.i.i, i64 32, i1 false), !dbg !13965, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i14.i.i.i), !dbg !13966, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i15.i.i.i), !dbg !13967, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_37.3.i.i.i, i64 32, i1 false), !dbg !13967, !noalias !13609
  store float %values.sroa.7.0.copyload.i.i.i, ptr %558, align 4, !dbg !13969, !noalias !13970
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_37.3.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i15.i.i.i, i64 32, i1 false), !dbg !13973, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i15.i.i.i), !dbg !13974, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i16.i.i.i), !dbg !13975, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_39.3.i.i.i, i64 32, i1 false), !dbg !13975, !noalias !13609
  store float 0.000000e+00, ptr %559, align 4, !dbg !13977, !noalias !13978
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_39.3.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i16.i.i.i, i64 32, i1 false), !dbg !13981, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i16.i.i.i), !dbg !13982, !noalias !13928
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i17.i.i.i), !dbg !13983, !noalias !13928
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %_41.3.i.i.i, i64 32, i1 false), !dbg !13983, !noalias !13609
  store float 0.000000e+00, ptr %560, align 4, !dbg !13985, !noalias !13986
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_41.3.i.i.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i17.i.i.i, i64 32, i1 false), !dbg !13989, !noalias !13609
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i17.i.i.i), !dbg !13990, !noalias !13928
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i, !dbg !13991

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i: ; preds = %bb20.3.i.i.i, %bb19.i.i.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16clear_lane_ringsB2_.exit.i.i
  %gep234.i.i = getelementptr inbounds nuw %"effect_contract::ProcessReport", ptr %540, i64 %iter1.sroa.0.0230.i.i, !dbg !13992
  %_46.i.i = load i64, ptr %gep234.i.i, align 8, !dbg !13994, !alias.scope !13996, !noalias !13997, !noundef !12
  %561 = tail call i64 @llvm.uadd.sat.i64(i64 %_46.i.i, i64 range(i64 0, 4294967296) %frames), !dbg !13998
  store i64 %561, ptr %gep234.i.i, align 8, !dbg !14001, !alias.scope !13996, !noalias !13997
  br label %bb17.backedge.i.i, !dbg !13802

bb17.backedge.i.i:                                ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9seed_laneB2_.exit.i.i, %bb30.i.i
  %exitcond259.not.i.i = icmp eq i64 %541, 8, !dbg !14002
  br i1 %exitcond259.not.i.i, label %bb1.backedge.i.i, label %bb30.i.i, !dbg !13805

bb32.i.i:                                         ; preds = %bb20.preheader.i.i, %bb21.i.i
  %iter2.sroa.0.0229.i.i = phi i64 [ %562, %bb21.i.i ], [ 0, %bb20.preheader.i.i ]
  %_39.i.i = shl nuw nsw i64 %iter2.sroa.0.0229.i.i, 3, !dbg !14005
  %_38.i.i = add nuw nsw i64 %_39.i.i, %iter1.sroa.0.0230.i.i, !dbg !14005
  %_41.i.i = icmp ult i64 %_38.i.i, %_14.1.i.i.i.i.i, !dbg !14007
  br i1 %_41.i.i, label %bb21.i.i, label %panic4.i.i, !dbg !14007

bb21.i.i:                                         ; preds = %bb32.i.i
  %562 = add nuw nsw i64 %iter2.sroa.0.0229.i.i, 1, !dbg !14008
  %563 = getelementptr inbounds nuw float, ptr %_14.0.i.i.i.i.i, i64 %_38.i.i, !dbg !14007
  store float 0.000000e+00, ptr %563, align 4, !dbg !14007, !noalias !14014
  %exitcond.not.i7.i = icmp eq i64 %562, %frames, !dbg !14015
  br i1 %exitcond.not.i7.i, label %bb33.i.i, label %bb32.i.i, !dbg !13820

panic4.i.i:                                       ; preds = %bb32.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_38.i.i, i64 noundef %_14.1.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30d570589940b68d7f77af9df77eb2ee) #24, !dbg !14007, !noalias !14014
  unreachable, !dbg !14007

bb20.i:                                           ; preds = %bb4.i
  %564 = shl nuw nsw i64 %..i.i, 3, !dbg !14018
  %_52.i = icmp samesign ugt i64 %564, %_37.1, !dbg !14019
  br i1 %_52.i, label %bb24.i, label %bb25.i, !dbg !14019, !prof !180

bb25.i:                                           ; preds = %bb20.i
  %_60.i = icmp samesign ugt i64 %564, %_38.1, !dbg !14027
  br i1 %_60.i, label %bb26.i, label %bb27.i, !dbg !14027, !prof !180

bb24.i:                                           ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %564, i64 noundef range(i64 0, 2305843009213693952) %_37.1, i64 noundef range(i64 0, 2305843009213693952) %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e6adad6d678d00eb6b02b8162f35ebb4) #24, !dbg !14031, !noalias !12090
  unreachable, !dbg !14031

bb27.i:                                           ; preds = %bb25.i
  %_59.i = getelementptr inbounds nuw float, ptr %_37.0, i64 %564, !dbg !14032
  %_55.i = sub nuw nsw i64 %_37.1, %564, !dbg !14037
  %_63.i = sub nuw nsw i64 %_38.1, %564, !dbg !14038
  %_67.i = getelementptr inbounds nuw float, ptr %_38.0, i64 %564, !dbg !14039
  %_26.i = sub nsw i64 %frames, %..i.i, !dbg !14044
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14045), !dbg !14048
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14049), !dbg !14048
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14051), !dbg !14048
  %data.i.i.i19.i = getelementptr inbounds nuw i8, ptr %self, i64 1888, !dbg !14053
  %_15.i20.i = getelementptr inbounds nuw i8, ptr %self, i64 1152, !dbg !14060
  %data.i.i906.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !14062
  %565 = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !14067
  %_32.i21.i = getelementptr inbounds nuw i8, ptr %self, i64 960, !dbg !14071
  %_58.0.i22.i = load ptr, ptr %_15.i20.i, align 8, !dbg !14072, !alias.scope !14073, !noalias !14074, !nonnull !12, !noundef !12
  %566 = getelementptr inbounds nuw i8, ptr %self, i64 1160, !dbg !14072
  %_58.1.i23.i = load i64, ptr %566, align 8, !dbg !14072, !alias.scope !14073, !noalias !14074, !noundef !12
  %_45.i24.i = getelementptr inbounds nuw i8, ptr %self, i64 1184, !dbg !14076
  %_60.0.i25.i = load ptr, ptr %data.i.i906.i.i, align 8, !dbg !14077, !alias.scope !14073, !noalias !14074, !nonnull !12, !noundef !12
  %567 = getelementptr inbounds nuw i8, ptr %self, i64 1224, !dbg !14077
  %_60.1.i26.i = load i64, ptr %567, align 8, !dbg !14077, !alias.scope !14073, !noalias !14074, !noundef !12
  %_50.i27.i = getelementptr inbounds nuw i8, ptr %self, i64 1248, !dbg !14078
  %_51.i28.i = getelementptr inbounds nuw i8, ptr %self, i64 2608, !dbg !14079
  %568 = getelementptr inbounds nuw i8, ptr %self, i64 2612, !dbg !14080
  %_52.i29.i = load i32, ptr %568, align 4, !dbg !14080, !alias.scope !14073, !noalias !14074, !noundef !12
  %569 = getelementptr inbounds nuw i8, ptr %self, i64 2616, !dbg !14081
  %_53.i30.i = load i32, ptr %569, align 8, !dbg !14081, !alias.scope !14073, !noalias !14074, !noundef !12
  %base.i.i35.i = load i32, ptr %_51.i28.i, align 4, !dbg !14082, !alias.scope !14073, !noalias !14092, !noundef !12
  %570 = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %571 = getelementptr inbounds nuw i8, ptr %self, i64 1536
  %572 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %573 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %574 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %575 = getelementptr inbounds nuw i8, ptr %self, i64 1792
  %576 = getelementptr inbounds nuw i8, ptr %self, i64 1824
  %577 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %578 = getelementptr inbounds nuw i8, ptr %self, i64 1856
  %579 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %580 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %581 = getelementptr inbounds nuw i8, ptr %self, i64 2016
  %582 = getelementptr inbounds nuw i8, ptr %self, i64 2144
  %583 = getelementptr inbounds nuw i8, ptr %self, i64 2272
  %584 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %585 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %586 = getelementptr inbounds nuw i8, ptr %self, i64 2400
  %587 = getelementptr inbounds nuw i8, ptr %self, i64 2432
  %588 = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %589 = getelementptr inbounds nuw i8, ptr %self, i64 2464
  %590 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %591 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %592 = lshr i64 %_63.i, 3, !dbg !14095
  %593 = lshr i64 %_55.i, 3, !dbg !14095
  %594 = getelementptr inbounds nuw i8, ptr %self, i64 1188
  %595 = getelementptr inbounds nuw i8, ptr %self, i64 1252
  %596 = getelementptr inbounds nuw i8, ptr %self, i64 1192
  %597 = getelementptr inbounds nuw i8, ptr %self, i64 1256
  %598 = getelementptr inbounds nuw i8, ptr %self, i64 1196
  %599 = getelementptr inbounds nuw i8, ptr %self, i64 1260
  %600 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %601 = getelementptr inbounds nuw i8, ptr %self, i64 1264
  %602 = getelementptr inbounds nuw i8, ptr %self, i64 1204
  %603 = getelementptr inbounds nuw i8, ptr %self, i64 1268
  %604 = getelementptr inbounds nuw i8, ptr %self, i64 1208
  %605 = getelementptr inbounds nuw i8, ptr %self, i64 1272
  %606 = getelementptr inbounds nuw i8, ptr %self, i64 1212
  %607 = getelementptr inbounds nuw i8, ptr %self, i64 1276
  br label %bb40.i.i36.i, !dbg !14095

bb40.i.i36.i:                                     ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit133.i.i, %bb27.i
  %iter.sroa.0.0.i1707.i.i = phi i64 [ 0, %bb27.i ], [ %608, %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit133.i.i ]
  %608 = add nuw nsw i64 %iter.sroa.0.0.i1707.i.i, 1, !dbg !14104
  %span.i.i37.i = shl i64 %iter.sroa.0.0.i1707.i.i, 3, !dbg !14110
  %_27.i.i38.i = trunc i64 %iter.sroa.0.0.i1707.i.i to i32, !dbg !14112
  %now.i.i39.i = add i32 %base.i.i35.i, %_27.i.i38.i, !dbg !14114
  %_30.i.i40.i = and i32 %now.i.i39.i, %_52.i29.i, !dbg !14117
  %_29.i.i41.i = zext i32 %_30.i.i40.i to i64, !dbg !14119
  %write.i.i42.i = shl nuw nsw i64 %_29.i.i41.i, 3, !dbg !14119
  %exitcond.not.i43.i = icmp eq i64 %iter.sroa.0.0.i1707.i.i, %593, !dbg !14120
  br i1 %exitcond.not.i43.i, label %bb43.i.i251.i, label %bb42.i.i44.i, !dbg !14120, !prof !2561

bb43.i.i251.i:                                    ; preds = %bb40.i.i36.i
  %_34.i.i252.i = add nuw nsw i64 %span.i.i37.i, 8, !dbg !14128
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i37.i, i64 noundef %_34.i.i252.i, i64 noundef range(i64 0, 2305843009213693952) %_55.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_97ed0782c9e7425b1b215f66cb64a347) #24, !dbg !14129, !noalias !14092
  unreachable, !dbg !14129

bb42.i.i44.i:                                     ; preds = %bb40.i.i36.i
  %_123.i.i45.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %span.i.i37.i, !dbg !14130
  %_36.i.i46.i = add nuw nsw i64 %write.i.i42.i, 8, !dbg !14134
  %_124.not.i.i47.i = icmp ugt i64 %_36.i.i46.i, %_58.1.i23.i, !dbg !14135
  br i1 %_124.not.i.i47.i, label %bb46.i.i250.i, label %bb45.i.i48.i, !dbg !14135, !prof !180

bb46.i.i250.i:                                    ; preds = %bb42.i.i44.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i42.i, i64 noundef %_36.i.i46.i, i64 noundef %_58.1.i23.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cad5a7b24766ffcdda9cbfe066eb4078) #24, !dbg !14140, !noalias !14092
  unreachable, !dbg !14140

bb45.i.i48.i:                                     ; preds = %bb42.i.i44.i
  %_133.i.i49.i = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %write.i.i42.i, !dbg !14141
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_133.i.i49.i, ptr noundef nonnull align 4 dereferenceable(32) %_123.i.i45.i, i64 32, i1 false), !dbg !14145, !noalias !14150
  %exitcond1802.i.i = icmp eq i64 %iter.sroa.0.0.i1707.i.i, %592, !dbg !14151
  br i1 %exitcond1802.i.i, label %bb49.i.i249.i, label %bb48.i.i50.i, !dbg !14151, !prof !180

bb49.i.i249.i:                                    ; preds = %bb45.i.i48.i
  %609 = and i64 %_63.i, 2305843009213693944, !dbg !14095
  %610 = add nuw nsw i64 %609, 8, !dbg !14095
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %span.i.i37.i, i64 noundef %610, i64 noundef range(i64 0, 2305843009213693952) %_63.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aebeae245abdbd708a3d39b73262e641) #24, !dbg !14155, !noalias !14092
  unreachable, !dbg !14155

bb48.i.i50.i:                                     ; preds = %bb45.i.i48.i
  %_141.i.i51.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %span.i.i37.i, !dbg !14156
  %_142.not.i.i52.i = icmp ugt i64 %_36.i.i46.i, %_60.1.i26.i, !dbg !14160
  br i1 %_142.not.i.i52.i, label %bb51.i.i248.i, label %bb50.i.i53.i, !dbg !14160, !prof !180

bb51.i.i248.i:                                    ; preds = %bb48.i.i50.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %write.i.i42.i, i64 noundef %_36.i.i46.i, i64 noundef %_60.1.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_72e24c5578221343e8a784545a3910d2) #24, !dbg !14164, !noalias !14092
  unreachable, !dbg !14164

bb50.i.i53.i:                                     ; preds = %bb48.i.i50.i
  %_149.i.i54.i = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %write.i.i42.i, !dbg !14165
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_149.i.i54.i, ptr noundef nonnull align 4 dereferenceable(32) %_141.i.i51.i, i64 32, i1 false), !dbg !14169, !noalias !14174
  %_52.i.i55.i = sub i32 %now.i.i39.i, %_53.i30.i, !dbg !14175
  %_51.i.i56.i = and i32 %_52.i.i55.i, %_52.i29.i, !dbg !14178
  %_50.i.i57.i = zext i32 %_51.i.i56.i to i64, !dbg !14179
  %read.i.i58.i = shl nuw nsw i64 %_50.i.i57.i, 3, !dbg !14179
  %_55.i.i59.i = add nuw nsw i64 %read.i.i58.i, 8, !dbg !14180
  %_182.not.i.i60.i = icmp ugt i64 %_55.i.i59.i, %_58.1.i23.i, !dbg !14182
  br i1 %_182.not.i.i60.i, label %bb61.i.i247.i, label %bb60.i.i61.i, !dbg !14182, !prof !180

bb61.i.i247.i:                                    ; preds = %bb50.i.i53.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i58.i, i64 noundef %_55.i.i59.i, i64 noundef %_58.1.i23.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba99eeeb3482270ebe1c674b4013dd6a) #24, !dbg !14186, !noalias !14092
  unreachable, !dbg !14186

bb60.i.i61.i:                                     ; preds = %bb50.i.i53.i
  %_189.i.i62.i = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %read.i.i58.i, !dbg !14187
  %lanes.i525.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_189.i.i62.i, align 4, !dbg !14191, !alias.scope !14196, !noalias !14200
  %_190.not.i.i63.i = icmp ugt i64 %_55.i.i59.i, %_60.1.i26.i, !dbg !14204
  br i1 %_190.not.i.i63.i, label %bb64.i.i246.i, label %bb63.i.i64.i, !dbg !14204, !prof !180

bb64.i.i246.i:                                    ; preds = %bb60.i.i61.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %read.i.i58.i, i64 noundef %_55.i.i59.i, i64 noundef %_60.1.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f4c27429e4885e1b619f1e4a3587acc) #24, !dbg !14209, !noalias !14092
  unreachable, !dbg !14209

bb63.i.i64.i:                                     ; preds = %bb60.i.i61.i
  %_195.i.i65.i = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %read.i.i58.i, !dbg !14210
  %lanes.i519.sroa.0.0.copyload.i.i = load <8 x float>, ptr %_195.i.i65.i, align 4, !dbg !14214, !alias.scope !14219, !noalias !14223
  %_67.i.i66.i = load i32, ptr %_45.i24.i, align 4, !dbg !14227, !alias.scope !14073, !noalias !14092, !noundef !12
  %_66.i.i67.i = sub i32 %now.i.i39.i, %_67.i.i66.i, !dbg !14233
  %_65.i.i68.i = and i32 %_66.i.i67.i, %_52.i29.i, !dbg !14235
  %_64.i.i69.i = zext i32 %_65.i.i68.i to i64, !dbg !14236
  %_63.i.i70.i = shl nuw nsw i64 %_64.i.i69.i, 3, !dbg !14236
  %_75.i.i71.i = load i32, ptr %_50.i27.i, align 4, !dbg !14237, !alias.scope !14073, !noalias !14092, !noundef !12
  %_74.i.i72.i = sub i32 %now.i.i39.i, %_75.i.i71.i, !dbg !14239
  %_73.i.i73.i = and i32 %_74.i.i72.i, %_52.i29.i, !dbg !14241
  %_72.i.i74.i = zext i32 %_73.i.i73.i to i64, !dbg !14242
  %_71.i.i75.i = shl nuw nsw i64 %_72.i.i74.i, 3, !dbg !14242
  %_80.i.i76.i = icmp ult i64 %_63.i.i70.i, %_58.1.i23.i, !dbg !14243
  br i1 %_80.i.i76.i, label %bb22.i.i79.i, label %panic18.i.i77.i, !dbg !14243

_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit133.i.i: ; preds = %bb26.i.7.i226.i
  %611 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.7.i220.i, !dbg !14245
  %_85.i.72164.i.i = load i32, ptr %611, align 4, !dbg !14245, !noalias !14092, !noundef !12
  %612 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.7.i220.i, !dbg !14246
  %_87.i.72165.i.i = load i32, ptr %612, align 4, !dbg !14246, !noalias !14092, !noundef !12
  %_41.i96.sroa.0.0.copyload.i.i = load <8 x float>, ptr %574, align 32, !dbg !14247, !alias.scope !14073, !noalias !14259
  %613 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_41.i96.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !14266
  %614 = bitcast <8 x float> %613 to <8 x i32>, !dbg !14272
  %615 = icmp slt <8 x i32> %614, zeroinitializer, !dbg !14276
  %lanes.i513.sroa.0.0.vec.insert.i.i = insertelement <8 x i32> poison, i32 %_78.i2134.i.i, i64 0, !dbg !14278
  %lanes.i513.sroa.0.4.vec.insert.i.i = insertelement <8 x i32> %lanes.i513.sroa.0.0.vec.insert.i.i, i32 %_78.i.12138.i.i, i64 1, !dbg !14278
  %lanes.i513.sroa.0.8.vec.insert.i.i = insertelement <8 x i32> %lanes.i513.sroa.0.4.vec.insert.i.i, i32 %_78.i.22142.i.i, i64 2, !dbg !14278
  %lanes.i513.sroa.0.12.vec.insert.i.i = insertelement <8 x i32> %lanes.i513.sroa.0.8.vec.insert.i.i, i32 %_78.i.32146.i.i, i64 3, !dbg !14278
  %lanes.i513.sroa.0.16.vec.insert.i.i = insertelement <8 x i32> %lanes.i513.sroa.0.12.vec.insert.i.i, i32 %_78.i.42150.i.i, i64 4, !dbg !14278
  %lanes.i513.sroa.0.20.vec.insert.i.i = insertelement <8 x i32> %lanes.i513.sroa.0.16.vec.insert.i.i, i32 %_78.i.52154.i.i, i64 5, !dbg !14278
  %lanes.i513.sroa.0.24.vec.insert.i.i = insertelement <8 x i32> %lanes.i513.sroa.0.20.vec.insert.i.i, i32 %_78.i.62158.i.i, i64 6, !dbg !14278
  %lanes.i513.sroa.0.28.vec.insert.i.i = insertelement <8 x i32> %lanes.i513.sroa.0.24.vec.insert.i.i, i32 %_78.i.72162.i.i, i64 7, !dbg !14278
  %616 = and <8 x i32> %lanes.i513.sroa.0.28.vec.insert.i.i, splat (i32 2147483647), !dbg !14283
  %617 = bitcast <8 x i32> %616 to <8 x float>, !dbg !14289
  %618 = fmul <8 x float> %617, splat (float 5.000000e-01), !dbg !14290
  %lanes.i507.sroa.0.0.vec.insert.i.i = insertelement <8 x i32> poison, i32 %_82.i2135.i.i, i64 0, !dbg !14295
  %lanes.i507.sroa.0.4.vec.insert.i.i = insertelement <8 x i32> %lanes.i507.sroa.0.0.vec.insert.i.i, i32 %_82.i.12139.i.i, i64 1, !dbg !14295
  %lanes.i507.sroa.0.8.vec.insert.i.i = insertelement <8 x i32> %lanes.i507.sroa.0.4.vec.insert.i.i, i32 %_82.i.22143.i.i, i64 2, !dbg !14295
  %lanes.i507.sroa.0.12.vec.insert.i.i = insertelement <8 x i32> %lanes.i507.sroa.0.8.vec.insert.i.i, i32 %_82.i.32147.i.i, i64 3, !dbg !14295
  %lanes.i507.sroa.0.16.vec.insert.i.i = insertelement <8 x i32> %lanes.i507.sroa.0.12.vec.insert.i.i, i32 %_82.i.42151.i.i, i64 4, !dbg !14295
  %lanes.i507.sroa.0.20.vec.insert.i.i = insertelement <8 x i32> %lanes.i507.sroa.0.16.vec.insert.i.i, i32 %_82.i.52155.i.i, i64 5, !dbg !14295
  %lanes.i507.sroa.0.24.vec.insert.i.i = insertelement <8 x i32> %lanes.i507.sroa.0.20.vec.insert.i.i, i32 %_82.i.62159.i.i, i64 6, !dbg !14295
  %lanes.i507.sroa.0.28.vec.insert.i.i = insertelement <8 x i32> %lanes.i507.sroa.0.24.vec.insert.i.i, i32 %_82.i.72163.i.i, i64 7, !dbg !14295
  %619 = and <8 x i32> %lanes.i507.sroa.0.28.vec.insert.i.i, splat (i32 2147483647), !dbg !14300
  %620 = bitcast <8 x i32> %619 to <8 x float>, !dbg !14306
  %621 = fmul <8 x float> %620, splat (float 5.000000e-01), !dbg !14307
  %622 = fadd <8 x float> %618, %621, !dbg !14312
  %_37.i100.sroa.0.0.copyload.i.i = load <8 x float>, ptr %573, align 32, !dbg !14317, !alias.scope !14073, !noalias !14259
  %623 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_37.i100.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !14318
  %624 = bitcast <8 x float> %623 to <8 x i32>, !dbg !14324
  %625 = icmp slt <8 x i32> %624, zeroinitializer, !dbg !14328
  %626 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %617, <8 x float> %620), !dbg !14330
  %627 = select <8 x i1> %625, <8 x float> %626, <8 x float> %617, !dbg !14328
  %628 = select <8 x i1> %615, <8 x float> %622, <8 x float> %627, !dbg !14276
  %629 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %628, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !14335
  %630 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %629, <8 x float> splat (float 0x3810000000000000)), !dbg !14340
  %631 = bitcast <8 x float> %630 to <4 x i64>, !dbg !14348
  %632 = and <4 x i64> %631, splat (i64 36028792732385279), !dbg !14349
  %633 = or disjoint <4 x i64> %632, splat (i64 4575657222473777152), !dbg !14354
  %634 = bitcast <4 x i64> %633 to <8 x float>, !dbg !14358
  %635 = fadd <8 x float> %634, splat (float -1.000000e+00), !dbg !14359
  %636 = fmul <8 x float> %635, splat (float 0x3F9B17A960000000), !dbg !14364
  %637 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %636, !dbg !14369
  %638 = fmul <8 x float> %635, %637, !dbg !14364
  %639 = fadd <8 x float> %638, splat (float 0xBFD1E3F400000000), !dbg !14369
  %640 = fmul <8 x float> %635, %639, !dbg !14364
  %641 = fadd <8 x float> %640, splat (float 0x3FDD544F20000000), !dbg !14369
  %642 = fmul <8 x float> %635, %641, !dbg !14364
  %643 = fadd <8 x float> %642, splat (float 0xBFE6FC2A60000000), !dbg !14369
  %644 = fmul <8 x float> %635, %643, !dbg !14364
  %645 = fadd <8 x float> %644, splat (float 0x3FF714B2A0000000), !dbg !14369
  %646 = bitcast <8 x float> %630 to <8 x i32>, !dbg !14374
  %_3.i910.i.i = lshr <8 x i32> %646, splat (i32 23), !dbg !14378
  %647 = or disjoint <8 x i32> %_3.i910.i.i, splat (i32 1258291200), !dbg !14379
  %648 = bitcast <8 x i32> %647 to <8 x float>, !dbg !14383
  %649 = fadd <8 x float> %648, splat (float 0xC160000FE0000000), !dbg !14384
  %hysteresis.i106.sroa.0.0.copyload.i.i = load <8 x float>, ptr %572, align 32, !dbg !14388, !alias.scope !14073, !noalias !14389
  %range.i107.sroa.0.0.copyload1601.i.i = load <8 x i32>, ptr %571, align 32, !dbg !14391, !alias.scope !14073, !noalias !14389
  %ratio.i108.sroa.0.0.copyload.i.i = load <8 x float>, ptr %570, align 32, !dbg !14392, !alias.scope !14073, !noalias !14389
  %threshold.i109.sroa.0.0.copyload.i.i = load <8 x float>, ptr %13, align 32, !dbg !14393, !alias.scope !14073, !noalias !14389
  %650 = fmul <8 x float> %635, %645, !dbg !14394
  %651 = fadd <8 x float> %649, %650, !dbg !14399
  %652 = fmul <8 x float> %651, splat (float 0x4018151820000000), !dbg !14404
  %653 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %652, <8 x float> splat (float 2.400000e+01)), !dbg !14409
  %654 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %653, <8 x float> splat (float -1.600000e+02)), !dbg !14414
  %_55.i82.sroa.0.0.copyload.i.i = load <8 x float>, ptr %575, align 32, !dbg !14419, !alias.scope !14073, !noalias !14389
  %655 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_55.i82.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !14421
  %656 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %654, <8 x float> %threshold.i109.sroa.0.0.copyload.i.i, i8 29), !dbg !14427
  %657 = fsub <8 x float> %threshold.i109.sroa.0.0.copyload.i.i, %hysteresis.i106.sroa.0.0.copyload.i.i, !dbg !14434
  %658 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %654, <8 x float> %657, i8 29), !dbg !14440
  %659 = bitcast <8 x float> %655 to <8 x i32>, !dbg !14446
  %660 = xor <8 x i32> %659, splat (i32 -1), !dbg !14453
  %661 = bitcast <8 x float> %656 to <8 x i32>, !dbg !14455
  %662 = and <8 x i32> %661, %660, !dbg !14459
  %663 = bitcast <8 x float> %658 to <8 x i32>, !dbg !14461
  %664 = and <8 x i32> %663, %659, !dbg !14466
  %665 = or <8 x i32> %664, %662, !dbg !14468
  %666 = xor <8 x i32> %663, splat (i32 -1), !dbg !14474
  %_67.i70.sroa.0.0.copyload.i.i = load <8 x float>, ptr %576, align 32, !dbg !14482, !alias.scope !14073, !noalias !14389
  %667 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_67.i70.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !14483
  %668 = bitcast <8 x float> %667 to <8 x i32>, !dbg !14489
  %669 = and <8 x i32> %666, %668, !dbg !14493
  %670 = and <8 x i32> %669, %659, !dbg !14493
  %671 = or <8 x i32> %670, %665, !dbg !14498
  %672 = icmp slt <8 x i32> %671, zeroinitializer, !dbg !14504
  %673 = select <8 x i1> %672, <8 x float> splat (float 1.000000e+00), <8 x float> zeroinitializer, !dbg !14504
  %_71.i66.sroa.0.0.copyload.i.i = load <8 x float>, ptr %577, align 32, !dbg !14509, !alias.scope !14073, !noalias !14259
  %674 = fadd <8 x float> %_67.i70.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !14511
  %675 = icmp slt <8 x i32> %670, zeroinitializer, !dbg !14516
  %676 = select <8 x i1> %675, <8 x float> %674, <8 x float> %_67.i70.sroa.0.0.copyload.i.i, !dbg !14516
  %677 = icmp slt <8 x i32> %665, zeroinitializer, !dbg !14521
  %678 = select <8 x i1> %677, <8 x float> %_71.i66.sroa.0.0.copyload.i.i, <8 x float> %676, !dbg !14521
  store <8 x float> %678, ptr %576, align 32, !dbg !14526, !alias.scope !14073, !noalias !14389
  store <8 x float> %673, ptr %575, align 32, !dbg !14527, !alias.scope !14073, !noalias !14389
  %_86.i51.sroa.0.0.copyload.i.i = load <8 x float>, ptr %578, align 32, !dbg !14528, !alias.scope !14073, !noalias !14389
  %_88.i50.sroa.0.0.copyload.i.i = load <8 x float>, ptr %579, align 32, !dbg !14531, !alias.scope !14073, !noalias !14259
  %_7.i786.i.i = load <8 x float>, ptr %565, align 32, !dbg !14532, !alias.scope !14534, !noalias !14537
  %679 = fadd <8 x float> %ratio.i108.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !14541
  %680 = fsub <8 x float> %654, %threshold.i109.sroa.0.0.copyload.i.i, !dbg !14546
  %681 = fmul <8 x float> %679, %680, !dbg !14551
  %682 = xor <8 x i32> %range.i107.sroa.0.0.copyload1601.i.i, splat (i32 -2147483648), !dbg !14556
  %683 = bitcast <8 x i32> %682 to <8 x float>, !dbg !14561
  %684 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %681, <8 x float> %683), !dbg !14562
  %685 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %684, <8 x float> zeroinitializer), !dbg !14567
  %686 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %673, <8 x float> zeroinitializer, i8 30), !dbg !14572
  %687 = bitcast <8 x float> %686 to <8 x i32>, !dbg !14578
  %688 = icmp slt <8 x i32> %687, zeroinitializer, !dbg !14582
  %689 = select <8 x i1> %688, <8 x float> zeroinitializer, <8 x float> %685, !dbg !14582
  %690 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %689, <8 x float> %_86.i51.sroa.0.0.copyload.i.i, i8 30), !dbg !14584
  %691 = bitcast <8 x float> %690 to <8 x i32>, !dbg !14590
  %692 = icmp slt <8 x i32> %691, zeroinitializer, !dbg !14593
  %693 = select <8 x i1> %692, <8 x float> %_7.i786.i.i, <8 x float> %_88.i50.sroa.0.0.copyload.i.i, !dbg !14593
  %694 = fsub <8 x float> %689, %_86.i51.sroa.0.0.copyload.i.i, !dbg !14595
  %695 = fmul <8 x float> %694, %693, !dbg !14601
  %696 = fadd <8 x float> %_86.i51.sroa.0.0.copyload.i.i, %695, !dbg !14606
  %697 = bitcast <8 x float> %696 to <8 x i32>, !dbg !14610
  %698 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %696), !dbg !14617
  %699 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %698, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !14619
  %700 = bitcast <8 x float> %699 to <8 x i32>, !dbg !14625
  %701 = xor <8 x i32> %700, splat (i32 -1), !dbg !14631
  %702 = and <8 x i32> %697, %701, !dbg !14633
  store <8 x i32> %702, ptr %578, align 32, !dbg !14637, !alias.scope !14073, !noalias !14389
  %703 = bitcast <8 x i32> %702 to <8 x float>, !dbg !14639
  %704 = fmul <8 x float> %703, splat (float 0x3FC542A5A0000000), !dbg !14640
  %705 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %704, <8 x float> splat (float -1.260000e+02)), !dbg !14647
  %706 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %705, <8 x float> splat (float 1.270000e+02)), !dbg !14653
  %707 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %706), !dbg !14658
  %708 = fsub <8 x float> %706, %707, !dbg !14663
  %709 = fmul <8 x float> %708, splat (float 0x3F5E974FA0000000), !dbg !14668
  %710 = fadd <8 x float> %709, splat (float 0x3F82778560000000), !dbg !14673
  %711 = fmul <8 x float> %708, %710, !dbg !14668
  %712 = fadd <8 x float> %711, splat (float 0x3FAC91CE60000000), !dbg !14673
  %713 = fmul <8 x float> %708, %712, !dbg !14668
  %714 = fadd <8 x float> %713, splat (float 0x3FCEBDB560000000), !dbg !14673
  %715 = fmul <8 x float> %708, %714, !dbg !14668
  %716 = fadd <8 x float> %715, splat (float 0x3FE62E4BA0000000), !dbg !14673
  %_98.i40.sroa.0.0.copyload.i.i = load <8 x float>, ptr %580, align 32, !dbg !14678, !alias.scope !14073, !noalias !14259
  %717 = fmul <8 x float> %708, %716, !dbg !14680
  %718 = fadd <8 x float> %717, splat (float 1.000000e+00), !dbg !14685
  %719 = fadd <8 x float> %707, splat (float 0x4160000FE0000000), !dbg !14690
  %720 = bitcast <8 x float> %719 to <8 x i32>, !dbg !14695
  %_3.i911.i.i = shl <8 x i32> %720, splat (i32 23), !dbg !14699
  %721 = bitcast <8 x i32> %_3.i911.i.i to <8 x float>, !dbg !14700
  %722 = fmul <8 x float> %718, %721, !dbg !14702
  %723 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %703, <8 x float> zeroinitializer, i8 0), !dbg !14706
  %724 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_98.i40.sroa.0.0.copyload.i.i, <8 x float> zeroinitializer, i8 30), !dbg !14712
  %725 = bitcast <8 x float> %723 to <8 x i32>, !dbg !14718
  %726 = bitcast <8 x float> %724 to <8 x i32>, !dbg !14718
  %727 = or <8 x i32> %726, %725, !dbg !14722
  %728 = fmul <8 x float> %lanes.i525.sroa.0.0.copyload.i.i, %722, !dbg !14724
  %729 = icmp slt <8 x i32> %727, zeroinitializer, !dbg !14730
  %730 = select <8 x i1> %729, <8 x float> %lanes.i525.sroa.0.0.copyload.i.i, <8 x float> %728, !dbg !14730
  %_41.i.sroa.0.0.copyload.i228.i = load <8 x float>, ptr %585, align 32, !dbg !14735, !alias.scope !14073, !noalias !14738
  %731 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_41.i.sroa.0.0.copyload.i228.i, <8 x float> zeroinitializer, i8 30), !dbg !14745
  %732 = bitcast <8 x float> %731 to <8 x i32>, !dbg !14751
  %733 = icmp slt <8 x i32> %732, zeroinitializer, !dbg !14755
  %lanes.i501.sroa.0.0.vec.insert.i.i = insertelement <8 x i32> poison, i32 %_85.i2136.i.i, i64 0, !dbg !14757
  %lanes.i501.sroa.0.4.vec.insert.i.i = insertelement <8 x i32> %lanes.i501.sroa.0.0.vec.insert.i.i, i32 %_85.i.12140.i.i, i64 1, !dbg !14757
  %lanes.i501.sroa.0.8.vec.insert.i.i = insertelement <8 x i32> %lanes.i501.sroa.0.4.vec.insert.i.i, i32 %_85.i.22144.i.i, i64 2, !dbg !14757
  %lanes.i501.sroa.0.12.vec.insert.i.i = insertelement <8 x i32> %lanes.i501.sroa.0.8.vec.insert.i.i, i32 %_85.i.32148.i.i, i64 3, !dbg !14757
  %lanes.i501.sroa.0.16.vec.insert.i.i = insertelement <8 x i32> %lanes.i501.sroa.0.12.vec.insert.i.i, i32 %_85.i.42152.i.i, i64 4, !dbg !14757
  %lanes.i501.sroa.0.20.vec.insert.i.i = insertelement <8 x i32> %lanes.i501.sroa.0.16.vec.insert.i.i, i32 %_85.i.52156.i.i, i64 5, !dbg !14757
  %lanes.i501.sroa.0.24.vec.insert.i.i = insertelement <8 x i32> %lanes.i501.sroa.0.20.vec.insert.i.i, i32 %_85.i.62160.i.i, i64 6, !dbg !14757
  %lanes.i501.sroa.0.28.vec.insert.i.i = insertelement <8 x i32> %lanes.i501.sroa.0.24.vec.insert.i.i, i32 %_85.i.72164.i.i, i64 7, !dbg !14757
  %734 = and <8 x i32> %lanes.i501.sroa.0.28.vec.insert.i.i, splat (i32 2147483647), !dbg !14762
  %735 = bitcast <8 x i32> %734 to <8 x float>, !dbg !14768
  %736 = fmul <8 x float> %735, splat (float 5.000000e-01), !dbg !14769
  %lanes.i.sroa.0.0.vec.insert.i229.i = insertelement <8 x i32> poison, i32 %_87.i2137.i.i, i64 0, !dbg !14774
  %lanes.i.sroa.0.4.vec.insert.i230.i = insertelement <8 x i32> %lanes.i.sroa.0.0.vec.insert.i229.i, i32 %_87.i.12141.i.i, i64 1, !dbg !14774
  %lanes.i.sroa.0.8.vec.insert.i231.i = insertelement <8 x i32> %lanes.i.sroa.0.4.vec.insert.i230.i, i32 %_87.i.22145.i.i, i64 2, !dbg !14774
  %lanes.i.sroa.0.12.vec.insert.i232.i = insertelement <8 x i32> %lanes.i.sroa.0.8.vec.insert.i231.i, i32 %_87.i.32149.i.i, i64 3, !dbg !14774
  %lanes.i.sroa.0.16.vec.insert.i233.i = insertelement <8 x i32> %lanes.i.sroa.0.12.vec.insert.i232.i, i32 %_87.i.42153.i.i, i64 4, !dbg !14774
  %lanes.i.sroa.0.20.vec.insert.i234.i = insertelement <8 x i32> %lanes.i.sroa.0.16.vec.insert.i233.i, i32 %_87.i.52157.i.i, i64 5, !dbg !14774
  %lanes.i.sroa.0.24.vec.insert.i235.i = insertelement <8 x i32> %lanes.i.sroa.0.20.vec.insert.i234.i, i32 %_87.i.62161.i.i, i64 6, !dbg !14774
  %lanes.i.sroa.0.28.vec.insert.i236.i = insertelement <8 x i32> %lanes.i.sroa.0.24.vec.insert.i235.i, i32 %_87.i.72165.i.i, i64 7, !dbg !14774
  %737 = and <8 x i32> %lanes.i.sroa.0.28.vec.insert.i236.i, splat (i32 2147483647), !dbg !14779
  %738 = bitcast <8 x i32> %737 to <8 x float>, !dbg !14785
  %739 = fmul <8 x float> %738, splat (float 5.000000e-01), !dbg !14786
  %740 = fadd <8 x float> %736, %739, !dbg !14791
  %_37.i.sroa.0.0.copyload.i237.i = load <8 x float>, ptr %584, align 32, !dbg !14796, !alias.scope !14073, !noalias !14738
  %741 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_37.i.sroa.0.0.copyload.i237.i, <8 x float> zeroinitializer, i8 30), !dbg !14797
  %742 = bitcast <8 x float> %741 to <8 x i32>, !dbg !14803
  %743 = icmp slt <8 x i32> %742, zeroinitializer, !dbg !14807
  %744 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %735, <8 x float> %738), !dbg !14809
  %745 = select <8 x i1> %743, <8 x float> %744, <8 x float> %735, !dbg !14807
  %746 = select <8 x i1> %733, <8 x float> %740, <8 x float> %745, !dbg !14755
  %747 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %746, <8 x float> splat (float 0x3E45798EE0000000)), !dbg !14814
  %748 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %747, <8 x float> splat (float 0x3810000000000000)), !dbg !14819
  %749 = bitcast <8 x float> %748 to <4 x i64>, !dbg !14826
  %750 = and <4 x i64> %749, splat (i64 36028792732385279), !dbg !14827
  %751 = or disjoint <4 x i64> %750, splat (i64 4575657222473777152), !dbg !14832
  %752 = bitcast <4 x i64> %751 to <8 x float>, !dbg !14836
  %753 = fadd <8 x float> %752, splat (float -1.000000e+00), !dbg !14837
  %754 = fmul <8 x float> %753, splat (float 0x3F9B17A960000000), !dbg !14842
  %755 = fsub <8 x float> splat (float 0x3FBF9A8440000000), %754, !dbg !14847
  %756 = fmul <8 x float> %753, %755, !dbg !14842
  %757 = fadd <8 x float> %756, splat (float 0xBFD1E3F400000000), !dbg !14847
  %758 = fmul <8 x float> %753, %757, !dbg !14842
  %759 = fadd <8 x float> %758, splat (float 0x3FDD544F20000000), !dbg !14847
  %760 = fmul <8 x float> %753, %759, !dbg !14842
  %761 = fadd <8 x float> %760, splat (float 0xBFE6FC2A60000000), !dbg !14847
  %762 = fmul <8 x float> %753, %761, !dbg !14842
  %763 = fadd <8 x float> %762, splat (float 0x3FF714B2A0000000), !dbg !14847
  %764 = bitcast <8 x float> %748 to <8 x i32>, !dbg !14852
  %_3.i912.i.i = lshr <8 x i32> %764, splat (i32 23), !dbg !14856
  %765 = or disjoint <8 x i32> %_3.i912.i.i, splat (i32 1258291200), !dbg !14857
  %766 = bitcast <8 x i32> %765 to <8 x float>, !dbg !14861
  %767 = fadd <8 x float> %766, splat (float 0xC160000FE0000000), !dbg !14862
  %hysteresis.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %583, align 32, !dbg !14866, !alias.scope !14073, !noalias !14867
  %range.i.sroa.0.0.copyload1611.i.i = load <8 x i32>, ptr %582, align 32, !dbg !14869, !alias.scope !14073, !noalias !14867
  %ratio.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %581, align 32, !dbg !14870, !alias.scope !14073, !noalias !14867
  %threshold.i.sroa.0.0.copyload.i.i = load <8 x float>, ptr %data.i.i.i19.i, align 32, !dbg !14871, !alias.scope !14073, !noalias !14867
  %768 = fmul <8 x float> %753, %763, !dbg !14872
  %769 = fadd <8 x float> %767, %768, !dbg !14877
  %770 = fmul <8 x float> %769, splat (float 0x4018151820000000), !dbg !14882
  %771 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %770, <8 x float> splat (float 2.400000e+01)), !dbg !14887
  %772 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %771, <8 x float> splat (float -1.600000e+02)), !dbg !14892
  %_55.i26.sroa.0.0.copyload.i238.i = load <8 x float>, ptr %586, align 32, !dbg !14897, !alias.scope !14073, !noalias !14867
  %773 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_55.i26.sroa.0.0.copyload.i238.i, <8 x float> zeroinitializer, i8 30), !dbg !14898
  %774 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %772, <8 x float> %threshold.i.sroa.0.0.copyload.i.i, i8 29), !dbg !14904
  %775 = fsub <8 x float> %threshold.i.sroa.0.0.copyload.i.i, %hysteresis.i.sroa.0.0.copyload.i.i, !dbg !14910
  %776 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %772, <8 x float> %775, i8 29), !dbg !14915
  %777 = bitcast <8 x float> %773 to <8 x i32>, !dbg !14921
  %778 = xor <8 x i32> %777, splat (i32 -1), !dbg !14927
  %779 = bitcast <8 x float> %774 to <8 x i32>, !dbg !14929
  %780 = and <8 x i32> %779, %778, !dbg !14933
  %781 = bitcast <8 x float> %776 to <8 x i32>, !dbg !14935
  %782 = and <8 x i32> %781, %777, !dbg !14939
  %783 = or <8 x i32> %782, %780, !dbg !14941
  %784 = xor <8 x i32> %781, splat (i32 -1), !dbg !14946
  %_67.i22.sroa.0.0.copyload.i239.i = load <8 x float>, ptr %587, align 32, !dbg !14953, !alias.scope !14073, !noalias !14867
  %785 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_67.i22.sroa.0.0.copyload.i239.i, <8 x float> zeroinitializer, i8 30), !dbg !14954
  %786 = bitcast <8 x float> %785 to <8 x i32>, !dbg !14960
  %787 = and <8 x i32> %784, %786, !dbg !14964
  %788 = and <8 x i32> %787, %777, !dbg !14964
  %789 = or <8 x i32> %788, %783, !dbg !14969
  %790 = icmp slt <8 x i32> %789, zeroinitializer, !dbg !14974
  %791 = select <8 x i1> %790, <8 x float> splat (float 1.000000e+00), <8 x float> zeroinitializer, !dbg !14974
  %_71.i20.sroa.0.0.copyload.i240.i = load <8 x float>, ptr %588, align 32, !dbg !14979, !alias.scope !14073, !noalias !14738
  %792 = fadd <8 x float> %_67.i22.sroa.0.0.copyload.i239.i, splat (float -1.000000e+00), !dbg !14980
  %793 = icmp slt <8 x i32> %788, zeroinitializer, !dbg !14985
  %794 = select <8 x i1> %793, <8 x float> %792, <8 x float> %_67.i22.sroa.0.0.copyload.i239.i, !dbg !14985
  %795 = icmp slt <8 x i32> %783, zeroinitializer, !dbg !14990
  %796 = select <8 x i1> %795, <8 x float> %_71.i20.sroa.0.0.copyload.i240.i, <8 x float> %794, !dbg !14990
  store <8 x float> %796, ptr %587, align 32, !dbg !14995, !alias.scope !14073, !noalias !14867
  store <8 x float> %791, ptr %586, align 32, !dbg !14996, !alias.scope !14073, !noalias !14867
  %_86.i11.sroa.0.0.copyload.i241.i = load <8 x float>, ptr %589, align 32, !dbg !14997, !alias.scope !14073, !noalias !14867
  %_88.i10.sroa.0.0.copyload.i242.i = load <8 x float>, ptr %590, align 32, !dbg !14998, !alias.scope !14073, !noalias !14738
  %_7.i818.i.i = load <8 x float>, ptr %_32.i21.i, align 32, !dbg !14999, !alias.scope !15001, !noalias !15004
  %797 = fadd <8 x float> %ratio.i.sroa.0.0.copyload.i.i, splat (float -1.000000e+00), !dbg !15008
  %798 = fsub <8 x float> %772, %threshold.i.sroa.0.0.copyload.i.i, !dbg !15013
  %799 = fmul <8 x float> %797, %798, !dbg !15018
  %800 = xor <8 x i32> %range.i.sroa.0.0.copyload1611.i.i, splat (i32 -2147483648), !dbg !15023
  %801 = bitcast <8 x i32> %800 to <8 x float>, !dbg !15028
  %802 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %799, <8 x float> %801), !dbg !15029
  %803 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %802, <8 x float> zeroinitializer), !dbg !15034
  %804 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %791, <8 x float> zeroinitializer, i8 30), !dbg !15039
  %805 = bitcast <8 x float> %804 to <8 x i32>, !dbg !15045
  %806 = icmp slt <8 x i32> %805, zeroinitializer, !dbg !15049
  %807 = select <8 x i1> %806, <8 x float> zeroinitializer, <8 x float> %803, !dbg !15049
  %808 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %807, <8 x float> %_86.i11.sroa.0.0.copyload.i241.i, i8 30), !dbg !15051
  %809 = bitcast <8 x float> %808 to <8 x i32>, !dbg !15057
  %810 = icmp slt <8 x i32> %809, zeroinitializer, !dbg !15060
  %811 = select <8 x i1> %810, <8 x float> %_7.i818.i.i, <8 x float> %_88.i10.sroa.0.0.copyload.i242.i, !dbg !15060
  %812 = fsub <8 x float> %807, %_86.i11.sroa.0.0.copyload.i241.i, !dbg !15062
  %813 = fmul <8 x float> %812, %811, !dbg !15067
  %814 = fadd <8 x float> %_86.i11.sroa.0.0.copyload.i241.i, %813, !dbg !15072
  %815 = bitcast <8 x float> %814 to <8 x i32>, !dbg !15076
  %816 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %814), !dbg !15082
  %817 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %816, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !15084
  %818 = bitcast <8 x float> %817 to <8 x i32>, !dbg !15090
  %819 = xor <8 x i32> %818, splat (i32 -1), !dbg !15096
  %820 = and <8 x i32> %815, %819, !dbg !15098
  store <8 x i32> %820, ptr %589, align 32, !dbg !15102, !alias.scope !14073, !noalias !14867
  %821 = bitcast <8 x i32> %820 to <8 x float>, !dbg !15103
  %822 = fmul <8 x float> %821, splat (float 0x3FC542A5A0000000), !dbg !15104
  %823 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %822, <8 x float> splat (float -1.260000e+02)), !dbg !15110
  %824 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %823, <8 x float> splat (float 1.270000e+02)), !dbg !15116
  %825 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %824), !dbg !15121
  %826 = fsub <8 x float> %824, %825, !dbg !15126
  %827 = fmul <8 x float> %826, splat (float 0x3F5E974FA0000000), !dbg !15131
  %828 = fadd <8 x float> %827, splat (float 0x3F82778560000000), !dbg !15136
  %829 = fmul <8 x float> %826, %828, !dbg !15131
  %830 = fadd <8 x float> %829, splat (float 0x3FAC91CE60000000), !dbg !15136
  %831 = fmul <8 x float> %826, %830, !dbg !15131
  %832 = fadd <8 x float> %831, splat (float 0x3FCEBDB560000000), !dbg !15136
  %833 = fmul <8 x float> %826, %832, !dbg !15131
  %834 = fadd <8 x float> %833, splat (float 0x3FE62E4BA0000000), !dbg !15136
  %835 = fmul <8 x float> %826, %834, !dbg !15141
  %836 = fadd <8 x float> %835, splat (float 1.000000e+00), !dbg !15146
  %837 = fadd <8 x float> %825, splat (float 0x4160000FE0000000), !dbg !15151
  %838 = bitcast <8 x float> %837 to <8 x i32>, !dbg !15156
  %_3.i913.i.i = shl <8 x i32> %838, splat (i32 23), !dbg !15160
  %839 = bitcast <8 x i32> %_3.i913.i.i to <8 x float>, !dbg !15161
  %840 = fmul <8 x float> %836, %839, !dbg !15163
  %841 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %821, <8 x float> zeroinitializer, i8 0), !dbg !15167
  %_98.i.sroa.0.0.copyload.i243.i = load <8 x float>, ptr %591, align 32, !dbg !15173, !alias.scope !14073, !noalias !14738
  %842 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %_98.i.sroa.0.0.copyload.i243.i, <8 x float> zeroinitializer, i8 30), !dbg !15174
  %843 = bitcast <8 x float> %841 to <8 x i32>, !dbg !15180
  %844 = bitcast <8 x float> %842 to <8 x i32>, !dbg !15180
  %845 = or <8 x i32> %844, %843, !dbg !15184
  %846 = fmul <8 x float> %lanes.i519.sroa.0.0.copyload.i.i, %840, !dbg !15186
  %847 = icmp slt <8 x i32> %845, zeroinitializer, !dbg !15191
  %848 = select <8 x i1> %847, <8 x float> %lanes.i519.sroa.0.0.copyload.i.i, <8 x float> %846, !dbg !15191
  store <8 x float> %730, ptr %_123.i.i45.i, align 4, !dbg !15196, !alias.scope !15202, !noalias !15206
  store <8 x float> %848, ptr %_141.i.i51.i, align 4, !dbg !15210, !alias.scope !15215, !noalias !15219
  %exitcond1803.not.i.i = icmp eq i64 %608, %_26.i, !dbg !15223
  br i1 %exitcond1803.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKB1r_EB3_.exit.i, label %bb40.i.i36.i, !dbg !14095

panic18.i.i77.i:                                  ; preds = %bb28.i.6.i208.i, %bb28.i.5.i188.i, %bb28.i.4.i168.i, %bb28.i.3.i148.i, %bb28.i.2.i128.i, %bb28.i.1.i108.i, %bb28.i.i88.i, %bb63.i.i64.i
  %own.i.lcssa.i78.i = phi i64 [ %_63.i.i70.i, %bb63.i.i64.i ], [ %own.i.1.i94.i, %bb28.i.i88.i ], [ %own.i.2.i114.i, %bb28.i.1.i108.i ], [ %own.i.3.i134.i, %bb28.i.2.i128.i ], [ %own.i.4.i154.i, %bb28.i.3.i148.i ], [ %own.i.5.i174.i, %bb28.i.4.i168.i ], [ %own.i.6.i194.i, %bb28.i.5.i188.i ], [ %own.i.7.i214.i, %bb28.i.6.i208.i ], !dbg !14236
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %own.i.lcssa.i78.i, i64 noundef %_58.1.i23.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b53d553f9945f7702333f7af20204159) #24, !dbg !14243, !noalias !14092
  unreachable, !dbg !14243

bb22.i.i79.i:                                     ; preds = %bb63.i.i64.i
  %849 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %_63.i.i70.i, !dbg !14243
  %_78.i2134.i.i = load i32, ptr %849, align 4, !dbg !14243, !noalias !14092, !noundef !12
  %_84.i.i80.i = icmp ult i64 %_63.i.i70.i, %_60.1.i26.i, !dbg !15226
  br i1 %_84.i.i80.i, label %bb24.i.i82.i, label %panic20.i.i81.i, !dbg !15226

panic20.i.i81.i:                                  ; preds = %bb22.i.7.i222.i, %bb22.i.6.i202.i, %bb22.i.5.i182.i, %bb22.i.4.i162.i, %bb22.i.3.i142.i, %bb22.i.2.i122.i, %bb22.i.1.i102.i, %bb22.i.i79.i
  %own.i.lcssa1712.i.i = phi i64 [ %_63.i.i70.i, %bb22.i.i79.i ], [ %own.i.1.i94.i, %bb22.i.1.i102.i ], [ %own.i.2.i114.i, %bb22.i.2.i122.i ], [ %own.i.3.i134.i, %bb22.i.3.i142.i ], [ %own.i.4.i154.i, %bb22.i.4.i162.i ], [ %own.i.5.i174.i, %bb22.i.5.i182.i ], [ %own.i.6.i194.i, %bb22.i.6.i202.i ], [ %own.i.7.i214.i, %bb22.i.7.i222.i ], !dbg !14236
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %own.i.lcssa1712.i.i, i64 noundef %_60.1.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30bf8a301493a607abcc78dbf93977e0) #24, !dbg !15226, !noalias !14092
  unreachable, !dbg !15226

bb24.i.i82.i:                                     ; preds = %bb22.i.i79.i
  %850 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %_63.i.i70.i, !dbg !15226
  %_82.i2135.i.i = load i32, ptr %850, align 4, !dbg !15226, !noalias !14092, !noundef !12
  %_86.i.i83.i = icmp ult i64 %_71.i.i75.i, %_60.1.i26.i, !dbg !14245
  br i1 %_86.i.i83.i, label %bb26.i.i85.i, label %panic22.i.i84.i, !dbg !14245

panic22.i.i84.i:                                  ; preds = %bb24.i.7.i224.i, %bb24.i.6.i204.i, %bb24.i.5.i184.i, %bb24.i.4.i164.i, %bb24.i.3.i144.i, %bb24.i.2.i124.i, %bb24.i.1.i104.i, %bb24.i.i82.i
  %partner.i.lcssa1709.i.i = phi i64 [ %_71.i.i75.i, %bb24.i.i82.i ], [ %partner.i.1.i100.i, %bb24.i.1.i104.i ], [ %partner.i.2.i120.i, %bb24.i.2.i124.i ], [ %partner.i.3.i140.i, %bb24.i.3.i144.i ], [ %partner.i.4.i160.i, %bb24.i.4.i164.i ], [ %partner.i.5.i180.i, %bb24.i.5.i184.i ], [ %partner.i.6.i200.i, %bb24.i.6.i204.i ], [ %partner.i.7.i220.i, %bb24.i.7.i224.i ], !dbg !14242
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %partner.i.lcssa1709.i.i, i64 noundef %_60.1.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5833dd3f10f38ea96ec24c6054ff083c) #24, !dbg !14245, !noalias !14092
  unreachable, !dbg !14245

bb26.i.i85.i:                                     ; preds = %bb24.i.i82.i
  %851 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %_71.i.i75.i, !dbg !14245
  %_85.i2136.i.i = load i32, ptr %851, align 4, !dbg !14245, !noalias !14092, !noundef !12
  %_88.i.i86.i = icmp ult i64 %_71.i.i75.i, %_58.1.i23.i, !dbg !14246
  br i1 %_88.i.i86.i, label %bb28.i.i88.i, label %panic24.i.i87.i, !dbg !14246

panic24.i.i87.i:                                  ; preds = %bb26.i.7.i226.i, %bb26.i.6.i206.i, %bb26.i.5.i186.i, %bb26.i.4.i166.i, %bb26.i.3.i146.i, %bb26.i.2.i126.i, %bb26.i.1.i106.i, %bb26.i.i85.i
  %partner.i.lcssa1710.i.i = phi i64 [ %_71.i.i75.i, %bb26.i.i85.i ], [ %partner.i.1.i100.i, %bb26.i.1.i106.i ], [ %partner.i.2.i120.i, %bb26.i.2.i126.i ], [ %partner.i.3.i140.i, %bb26.i.3.i146.i ], [ %partner.i.4.i160.i, %bb26.i.4.i166.i ], [ %partner.i.5.i180.i, %bb26.i.5.i186.i ], [ %partner.i.6.i200.i, %bb26.i.6.i206.i ], [ %partner.i.7.i220.i, %bb26.i.7.i226.i ], !dbg !14242
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %partner.i.lcssa1710.i.i, i64 noundef %_58.1.i23.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5ae5decb60097d7e1183cb43cae6b3f) #24, !dbg !14246, !noalias !14092
  unreachable, !dbg !14246

bb28.i.i88.i:                                     ; preds = %bb26.i.i85.i
  %852 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %_71.i.i75.i, !dbg !14246
  %_87.i2137.i.i = load i32, ptr %852, align 4, !dbg !14246, !noalias !14092, !noundef !12
  %_67.i.1.i89.i = load i32, ptr %594, align 4, !dbg !14227, !alias.scope !14073, !noalias !14092, !noundef !12
  %_66.i.1.i90.i = sub i32 %now.i.i39.i, %_67.i.1.i89.i, !dbg !14233
  %_65.i.1.i91.i = and i32 %_66.i.1.i90.i, %_52.i29.i, !dbg !14235
  %_64.i.1.i92.i = zext i32 %_65.i.1.i91.i to i64, !dbg !14236
  %_63.i.1.i93.i = shl nuw nsw i64 %_64.i.1.i92.i, 3, !dbg !14236
  %own.i.1.i94.i = or disjoint i64 %_63.i.1.i93.i, 1, !dbg !14236
  %_75.i.1.i95.i = load i32, ptr %595, align 4, !dbg !14237, !alias.scope !14073, !noalias !14092, !noundef !12
  %_74.i.1.i96.i = sub i32 %now.i.i39.i, %_75.i.1.i95.i, !dbg !14239
  %_73.i.1.i97.i = and i32 %_74.i.1.i96.i, %_52.i29.i, !dbg !14241
  %_72.i.1.i98.i = zext i32 %_73.i.1.i97.i to i64, !dbg !14242
  %_71.i.1.i99.i = shl nuw nsw i64 %_72.i.1.i98.i, 3, !dbg !14242
  %partner.i.1.i100.i = or disjoint i64 %_71.i.1.i99.i, 1, !dbg !14242
  %_80.i.1.i101.i = icmp ult i64 %own.i.1.i94.i, %_58.1.i23.i, !dbg !14243
  br i1 %_80.i.1.i101.i, label %bb22.i.1.i102.i, label %panic18.i.i77.i, !dbg !14243

bb22.i.1.i102.i:                                  ; preds = %bb28.i.i88.i
  %853 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.1.i94.i, !dbg !14243
  %_78.i.12138.i.i = load i32, ptr %853, align 4, !dbg !14243, !noalias !14092, !noundef !12
  %_84.i.1.i103.i = icmp ult i64 %own.i.1.i94.i, %_60.1.i26.i, !dbg !15226
  br i1 %_84.i.1.i103.i, label %bb24.i.1.i104.i, label %panic20.i.i81.i, !dbg !15226

bb24.i.1.i104.i:                                  ; preds = %bb22.i.1.i102.i
  %854 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.1.i94.i, !dbg !15226
  %_82.i.12139.i.i = load i32, ptr %854, align 4, !dbg !15226, !noalias !14092, !noundef !12
  %_86.i.1.i105.i = icmp ult i64 %partner.i.1.i100.i, %_60.1.i26.i, !dbg !14245
  br i1 %_86.i.1.i105.i, label %bb26.i.1.i106.i, label %panic22.i.i84.i, !dbg !14245

bb26.i.1.i106.i:                                  ; preds = %bb24.i.1.i104.i
  %855 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.1.i100.i, !dbg !14245
  %_85.i.12140.i.i = load i32, ptr %855, align 4, !dbg !14245, !noalias !14092, !noundef !12
  %_88.i.1.i107.i = icmp ult i64 %partner.i.1.i100.i, %_58.1.i23.i, !dbg !14246
  br i1 %_88.i.1.i107.i, label %bb28.i.1.i108.i, label %panic24.i.i87.i, !dbg !14246

bb28.i.1.i108.i:                                  ; preds = %bb26.i.1.i106.i
  %856 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.1.i100.i, !dbg !14246
  %_87.i.12141.i.i = load i32, ptr %856, align 4, !dbg !14246, !noalias !14092, !noundef !12
  %_67.i.2.i109.i = load i32, ptr %596, align 4, !dbg !14227, !alias.scope !14073, !noalias !14092, !noundef !12
  %_66.i.2.i110.i = sub i32 %now.i.i39.i, %_67.i.2.i109.i, !dbg !14233
  %_65.i.2.i111.i = and i32 %_66.i.2.i110.i, %_52.i29.i, !dbg !14235
  %_64.i.2.i112.i = zext i32 %_65.i.2.i111.i to i64, !dbg !14236
  %_63.i.2.i113.i = shl nuw nsw i64 %_64.i.2.i112.i, 3, !dbg !14236
  %own.i.2.i114.i = or disjoint i64 %_63.i.2.i113.i, 2, !dbg !14236
  %_75.i.2.i115.i = load i32, ptr %597, align 4, !dbg !14237, !alias.scope !14073, !noalias !14092, !noundef !12
  %_74.i.2.i116.i = sub i32 %now.i.i39.i, %_75.i.2.i115.i, !dbg !14239
  %_73.i.2.i117.i = and i32 %_74.i.2.i116.i, %_52.i29.i, !dbg !14241
  %_72.i.2.i118.i = zext i32 %_73.i.2.i117.i to i64, !dbg !14242
  %_71.i.2.i119.i = shl nuw nsw i64 %_72.i.2.i118.i, 3, !dbg !14242
  %partner.i.2.i120.i = or disjoint i64 %_71.i.2.i119.i, 2, !dbg !14242
  %_80.i.2.i121.i = icmp ult i64 %own.i.2.i114.i, %_58.1.i23.i, !dbg !14243
  br i1 %_80.i.2.i121.i, label %bb22.i.2.i122.i, label %panic18.i.i77.i, !dbg !14243

bb22.i.2.i122.i:                                  ; preds = %bb28.i.1.i108.i
  %857 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.2.i114.i, !dbg !14243
  %_78.i.22142.i.i = load i32, ptr %857, align 4, !dbg !14243, !noalias !14092, !noundef !12
  %_84.i.2.i123.i = icmp ult i64 %own.i.2.i114.i, %_60.1.i26.i, !dbg !15226
  br i1 %_84.i.2.i123.i, label %bb24.i.2.i124.i, label %panic20.i.i81.i, !dbg !15226

bb24.i.2.i124.i:                                  ; preds = %bb22.i.2.i122.i
  %858 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.2.i114.i, !dbg !15226
  %_82.i.22143.i.i = load i32, ptr %858, align 4, !dbg !15226, !noalias !14092, !noundef !12
  %_86.i.2.i125.i = icmp ult i64 %partner.i.2.i120.i, %_60.1.i26.i, !dbg !14245
  br i1 %_86.i.2.i125.i, label %bb26.i.2.i126.i, label %panic22.i.i84.i, !dbg !14245

bb26.i.2.i126.i:                                  ; preds = %bb24.i.2.i124.i
  %859 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.2.i120.i, !dbg !14245
  %_85.i.22144.i.i = load i32, ptr %859, align 4, !dbg !14245, !noalias !14092, !noundef !12
  %_88.i.2.i127.i = icmp ult i64 %partner.i.2.i120.i, %_58.1.i23.i, !dbg !14246
  br i1 %_88.i.2.i127.i, label %bb28.i.2.i128.i, label %panic24.i.i87.i, !dbg !14246

bb28.i.2.i128.i:                                  ; preds = %bb26.i.2.i126.i
  %860 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.2.i120.i, !dbg !14246
  %_87.i.22145.i.i = load i32, ptr %860, align 4, !dbg !14246, !noalias !14092, !noundef !12
  %_67.i.3.i129.i = load i32, ptr %598, align 4, !dbg !14227, !alias.scope !14073, !noalias !14092, !noundef !12
  %_66.i.3.i130.i = sub i32 %now.i.i39.i, %_67.i.3.i129.i, !dbg !14233
  %_65.i.3.i131.i = and i32 %_66.i.3.i130.i, %_52.i29.i, !dbg !14235
  %_64.i.3.i132.i = zext i32 %_65.i.3.i131.i to i64, !dbg !14236
  %_63.i.3.i133.i = shl nuw nsw i64 %_64.i.3.i132.i, 3, !dbg !14236
  %own.i.3.i134.i = or disjoint i64 %_63.i.3.i133.i, 3, !dbg !14236
  %_75.i.3.i135.i = load i32, ptr %599, align 4, !dbg !14237, !alias.scope !14073, !noalias !14092, !noundef !12
  %_74.i.3.i136.i = sub i32 %now.i.i39.i, %_75.i.3.i135.i, !dbg !14239
  %_73.i.3.i137.i = and i32 %_74.i.3.i136.i, %_52.i29.i, !dbg !14241
  %_72.i.3.i138.i = zext i32 %_73.i.3.i137.i to i64, !dbg !14242
  %_71.i.3.i139.i = shl nuw nsw i64 %_72.i.3.i138.i, 3, !dbg !14242
  %partner.i.3.i140.i = or disjoint i64 %_71.i.3.i139.i, 3, !dbg !14242
  %_80.i.3.i141.i = icmp ult i64 %own.i.3.i134.i, %_58.1.i23.i, !dbg !14243
  br i1 %_80.i.3.i141.i, label %bb22.i.3.i142.i, label %panic18.i.i77.i, !dbg !14243

bb22.i.3.i142.i:                                  ; preds = %bb28.i.2.i128.i
  %861 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.3.i134.i, !dbg !14243
  %_78.i.32146.i.i = load i32, ptr %861, align 4, !dbg !14243, !noalias !14092, !noundef !12
  %_84.i.3.i143.i = icmp ult i64 %own.i.3.i134.i, %_60.1.i26.i, !dbg !15226
  br i1 %_84.i.3.i143.i, label %bb24.i.3.i144.i, label %panic20.i.i81.i, !dbg !15226

bb24.i.3.i144.i:                                  ; preds = %bb22.i.3.i142.i
  %862 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.3.i134.i, !dbg !15226
  %_82.i.32147.i.i = load i32, ptr %862, align 4, !dbg !15226, !noalias !14092, !noundef !12
  %_86.i.3.i145.i = icmp ult i64 %partner.i.3.i140.i, %_60.1.i26.i, !dbg !14245
  br i1 %_86.i.3.i145.i, label %bb26.i.3.i146.i, label %panic22.i.i84.i, !dbg !14245

bb26.i.3.i146.i:                                  ; preds = %bb24.i.3.i144.i
  %863 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.3.i140.i, !dbg !14245
  %_85.i.32148.i.i = load i32, ptr %863, align 4, !dbg !14245, !noalias !14092, !noundef !12
  %_88.i.3.i147.i = icmp ult i64 %partner.i.3.i140.i, %_58.1.i23.i, !dbg !14246
  br i1 %_88.i.3.i147.i, label %bb28.i.3.i148.i, label %panic24.i.i87.i, !dbg !14246

bb28.i.3.i148.i:                                  ; preds = %bb26.i.3.i146.i
  %864 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.3.i140.i, !dbg !14246
  %_87.i.32149.i.i = load i32, ptr %864, align 4, !dbg !14246, !noalias !14092, !noundef !12
  %_67.i.4.i149.i = load i32, ptr %600, align 4, !dbg !14227, !alias.scope !14073, !noalias !14092, !noundef !12
  %_66.i.4.i150.i = sub i32 %now.i.i39.i, %_67.i.4.i149.i, !dbg !14233
  %_65.i.4.i151.i = and i32 %_66.i.4.i150.i, %_52.i29.i, !dbg !14235
  %_64.i.4.i152.i = zext i32 %_65.i.4.i151.i to i64, !dbg !14236
  %_63.i.4.i153.i = shl nuw nsw i64 %_64.i.4.i152.i, 3, !dbg !14236
  %own.i.4.i154.i = or disjoint i64 %_63.i.4.i153.i, 4, !dbg !14236
  %_75.i.4.i155.i = load i32, ptr %601, align 4, !dbg !14237, !alias.scope !14073, !noalias !14092, !noundef !12
  %_74.i.4.i156.i = sub i32 %now.i.i39.i, %_75.i.4.i155.i, !dbg !14239
  %_73.i.4.i157.i = and i32 %_74.i.4.i156.i, %_52.i29.i, !dbg !14241
  %_72.i.4.i158.i = zext i32 %_73.i.4.i157.i to i64, !dbg !14242
  %_71.i.4.i159.i = shl nuw nsw i64 %_72.i.4.i158.i, 3, !dbg !14242
  %partner.i.4.i160.i = or disjoint i64 %_71.i.4.i159.i, 4, !dbg !14242
  %_80.i.4.i161.i = icmp ult i64 %own.i.4.i154.i, %_58.1.i23.i, !dbg !14243
  br i1 %_80.i.4.i161.i, label %bb22.i.4.i162.i, label %panic18.i.i77.i, !dbg !14243

bb22.i.4.i162.i:                                  ; preds = %bb28.i.3.i148.i
  %865 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.4.i154.i, !dbg !14243
  %_78.i.42150.i.i = load i32, ptr %865, align 4, !dbg !14243, !noalias !14092, !noundef !12
  %_84.i.4.i163.i = icmp ult i64 %own.i.4.i154.i, %_60.1.i26.i, !dbg !15226
  br i1 %_84.i.4.i163.i, label %bb24.i.4.i164.i, label %panic20.i.i81.i, !dbg !15226

bb24.i.4.i164.i:                                  ; preds = %bb22.i.4.i162.i
  %866 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.4.i154.i, !dbg !15226
  %_82.i.42151.i.i = load i32, ptr %866, align 4, !dbg !15226, !noalias !14092, !noundef !12
  %_86.i.4.i165.i = icmp ult i64 %partner.i.4.i160.i, %_60.1.i26.i, !dbg !14245
  br i1 %_86.i.4.i165.i, label %bb26.i.4.i166.i, label %panic22.i.i84.i, !dbg !14245

bb26.i.4.i166.i:                                  ; preds = %bb24.i.4.i164.i
  %867 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.4.i160.i, !dbg !14245
  %_85.i.42152.i.i = load i32, ptr %867, align 4, !dbg !14245, !noalias !14092, !noundef !12
  %_88.i.4.i167.i = icmp ult i64 %partner.i.4.i160.i, %_58.1.i23.i, !dbg !14246
  br i1 %_88.i.4.i167.i, label %bb28.i.4.i168.i, label %panic24.i.i87.i, !dbg !14246

bb28.i.4.i168.i:                                  ; preds = %bb26.i.4.i166.i
  %868 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.4.i160.i, !dbg !14246
  %_87.i.42153.i.i = load i32, ptr %868, align 4, !dbg !14246, !noalias !14092, !noundef !12
  %_67.i.5.i169.i = load i32, ptr %602, align 4, !dbg !14227, !alias.scope !14073, !noalias !14092, !noundef !12
  %_66.i.5.i170.i = sub i32 %now.i.i39.i, %_67.i.5.i169.i, !dbg !14233
  %_65.i.5.i171.i = and i32 %_66.i.5.i170.i, %_52.i29.i, !dbg !14235
  %_64.i.5.i172.i = zext i32 %_65.i.5.i171.i to i64, !dbg !14236
  %_63.i.5.i173.i = shl nuw nsw i64 %_64.i.5.i172.i, 3, !dbg !14236
  %own.i.5.i174.i = or disjoint i64 %_63.i.5.i173.i, 5, !dbg !14236
  %_75.i.5.i175.i = load i32, ptr %603, align 4, !dbg !14237, !alias.scope !14073, !noalias !14092, !noundef !12
  %_74.i.5.i176.i = sub i32 %now.i.i39.i, %_75.i.5.i175.i, !dbg !14239
  %_73.i.5.i177.i = and i32 %_74.i.5.i176.i, %_52.i29.i, !dbg !14241
  %_72.i.5.i178.i = zext i32 %_73.i.5.i177.i to i64, !dbg !14242
  %_71.i.5.i179.i = shl nuw nsw i64 %_72.i.5.i178.i, 3, !dbg !14242
  %partner.i.5.i180.i = or disjoint i64 %_71.i.5.i179.i, 5, !dbg !14242
  %_80.i.5.i181.i = icmp ult i64 %own.i.5.i174.i, %_58.1.i23.i, !dbg !14243
  br i1 %_80.i.5.i181.i, label %bb22.i.5.i182.i, label %panic18.i.i77.i, !dbg !14243

bb22.i.5.i182.i:                                  ; preds = %bb28.i.4.i168.i
  %869 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.5.i174.i, !dbg !14243
  %_78.i.52154.i.i = load i32, ptr %869, align 4, !dbg !14243, !noalias !14092, !noundef !12
  %_84.i.5.i183.i = icmp ult i64 %own.i.5.i174.i, %_60.1.i26.i, !dbg !15226
  br i1 %_84.i.5.i183.i, label %bb24.i.5.i184.i, label %panic20.i.i81.i, !dbg !15226

bb24.i.5.i184.i:                                  ; preds = %bb22.i.5.i182.i
  %870 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.5.i174.i, !dbg !15226
  %_82.i.52155.i.i = load i32, ptr %870, align 4, !dbg !15226, !noalias !14092, !noundef !12
  %_86.i.5.i185.i = icmp ult i64 %partner.i.5.i180.i, %_60.1.i26.i, !dbg !14245
  br i1 %_86.i.5.i185.i, label %bb26.i.5.i186.i, label %panic22.i.i84.i, !dbg !14245

bb26.i.5.i186.i:                                  ; preds = %bb24.i.5.i184.i
  %871 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.5.i180.i, !dbg !14245
  %_85.i.52156.i.i = load i32, ptr %871, align 4, !dbg !14245, !noalias !14092, !noundef !12
  %_88.i.5.i187.i = icmp ult i64 %partner.i.5.i180.i, %_58.1.i23.i, !dbg !14246
  br i1 %_88.i.5.i187.i, label %bb28.i.5.i188.i, label %panic24.i.i87.i, !dbg !14246

bb28.i.5.i188.i:                                  ; preds = %bb26.i.5.i186.i
  %872 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.5.i180.i, !dbg !14246
  %_87.i.52157.i.i = load i32, ptr %872, align 4, !dbg !14246, !noalias !14092, !noundef !12
  %_67.i.6.i189.i = load i32, ptr %604, align 4, !dbg !14227, !alias.scope !14073, !noalias !14092, !noundef !12
  %_66.i.6.i190.i = sub i32 %now.i.i39.i, %_67.i.6.i189.i, !dbg !14233
  %_65.i.6.i191.i = and i32 %_66.i.6.i190.i, %_52.i29.i, !dbg !14235
  %_64.i.6.i192.i = zext i32 %_65.i.6.i191.i to i64, !dbg !14236
  %_63.i.6.i193.i = shl nuw nsw i64 %_64.i.6.i192.i, 3, !dbg !14236
  %own.i.6.i194.i = or disjoint i64 %_63.i.6.i193.i, 6, !dbg !14236
  %_75.i.6.i195.i = load i32, ptr %605, align 4, !dbg !14237, !alias.scope !14073, !noalias !14092, !noundef !12
  %_74.i.6.i196.i = sub i32 %now.i.i39.i, %_75.i.6.i195.i, !dbg !14239
  %_73.i.6.i197.i = and i32 %_74.i.6.i196.i, %_52.i29.i, !dbg !14241
  %_72.i.6.i198.i = zext i32 %_73.i.6.i197.i to i64, !dbg !14242
  %_71.i.6.i199.i = shl nuw nsw i64 %_72.i.6.i198.i, 3, !dbg !14242
  %partner.i.6.i200.i = or disjoint i64 %_71.i.6.i199.i, 6, !dbg !14242
  %_80.i.6.i201.i = icmp ult i64 %own.i.6.i194.i, %_58.1.i23.i, !dbg !14243
  br i1 %_80.i.6.i201.i, label %bb22.i.6.i202.i, label %panic18.i.i77.i, !dbg !14243

bb22.i.6.i202.i:                                  ; preds = %bb28.i.5.i188.i
  %873 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.6.i194.i, !dbg !14243
  %_78.i.62158.i.i = load i32, ptr %873, align 4, !dbg !14243, !noalias !14092, !noundef !12
  %_84.i.6.i203.i = icmp ult i64 %own.i.6.i194.i, %_60.1.i26.i, !dbg !15226
  br i1 %_84.i.6.i203.i, label %bb24.i.6.i204.i, label %panic20.i.i81.i, !dbg !15226

bb24.i.6.i204.i:                                  ; preds = %bb22.i.6.i202.i
  %874 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.6.i194.i, !dbg !15226
  %_82.i.62159.i.i = load i32, ptr %874, align 4, !dbg !15226, !noalias !14092, !noundef !12
  %_86.i.6.i205.i = icmp ult i64 %partner.i.6.i200.i, %_60.1.i26.i, !dbg !14245
  br i1 %_86.i.6.i205.i, label %bb26.i.6.i206.i, label %panic22.i.i84.i, !dbg !14245

bb26.i.6.i206.i:                                  ; preds = %bb24.i.6.i204.i
  %875 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %partner.i.6.i200.i, !dbg !14245
  %_85.i.62160.i.i = load i32, ptr %875, align 4, !dbg !14245, !noalias !14092, !noundef !12
  %_88.i.6.i207.i = icmp ult i64 %partner.i.6.i200.i, %_58.1.i23.i, !dbg !14246
  br i1 %_88.i.6.i207.i, label %bb28.i.6.i208.i, label %panic24.i.i87.i, !dbg !14246

bb28.i.6.i208.i:                                  ; preds = %bb26.i.6.i206.i
  %876 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %partner.i.6.i200.i, !dbg !14246
  %_87.i.62161.i.i = load i32, ptr %876, align 4, !dbg !14246, !noalias !14092, !noundef !12
  %_67.i.7.i209.i = load i32, ptr %606, align 4, !dbg !14227, !alias.scope !14073, !noalias !14092, !noundef !12
  %_66.i.7.i210.i = sub i32 %now.i.i39.i, %_67.i.7.i209.i, !dbg !14233
  %_65.i.7.i211.i = and i32 %_66.i.7.i210.i, %_52.i29.i, !dbg !14235
  %_64.i.7.i212.i = zext i32 %_65.i.7.i211.i to i64, !dbg !14236
  %_63.i.7.i213.i = shl nuw nsw i64 %_64.i.7.i212.i, 3, !dbg !14236
  %own.i.7.i214.i = or disjoint i64 %_63.i.7.i213.i, 7, !dbg !14236
  %_75.i.7.i215.i = load i32, ptr %607, align 4, !dbg !14237, !alias.scope !14073, !noalias !14092, !noundef !12
  %_74.i.7.i216.i = sub i32 %now.i.i39.i, %_75.i.7.i215.i, !dbg !14239
  %_73.i.7.i217.i = and i32 %_74.i.7.i216.i, %_52.i29.i, !dbg !14241
  %_72.i.7.i218.i = zext i32 %_73.i.7.i217.i to i64, !dbg !14242
  %_71.i.7.i219.i = shl nuw nsw i64 %_72.i.7.i218.i, 3, !dbg !14242
  %partner.i.7.i220.i = or disjoint i64 %_71.i.7.i219.i, 7, !dbg !14242
  %_80.i.7.i221.i = icmp ult i64 %own.i.7.i214.i, %_58.1.i23.i, !dbg !14243
  br i1 %_80.i.7.i221.i, label %bb22.i.7.i222.i, label %panic18.i.i77.i, !dbg !14243

bb22.i.7.i222.i:                                  ; preds = %bb28.i.6.i208.i
  %877 = getelementptr inbounds nuw float, ptr %_58.0.i22.i, i64 %own.i.7.i214.i, !dbg !14243
  %_78.i.72162.i.i = load i32, ptr %877, align 4, !dbg !14243, !noalias !14092, !noundef !12
  %_84.i.7.i223.i = icmp ult i64 %own.i.7.i214.i, %_60.1.i26.i, !dbg !15226
  br i1 %_84.i.7.i223.i, label %bb24.i.7.i224.i, label %panic20.i.i81.i, !dbg !15226

bb24.i.7.i224.i:                                  ; preds = %bb22.i.7.i222.i
  %878 = getelementptr inbounds nuw float, ptr %_60.0.i25.i, i64 %own.i.7.i214.i, !dbg !15226
  %_82.i.72163.i.i = load i32, ptr %878, align 4, !dbg !15226, !noalias !14092, !noundef !12
  %_86.i.7.i225.i = icmp ult i64 %partner.i.7.i220.i, %_60.1.i26.i, !dbg !14245
  br i1 %_86.i.7.i225.i, label %bb26.i.7.i226.i, label %panic22.i.i84.i, !dbg !14245

bb26.i.7.i226.i:                                  ; preds = %bb24.i.7.i224.i
  %_88.i.7.i227.i = icmp ult i64 %partner.i.7.i220.i, %_58.1.i23.i, !dbg !14246
  br i1 %_88.i.7.i227.i, label %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit133.i.i, label %panic24.i.i87.i, !dbg !14246

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E11run_segmentKB1r_EB3_.exit.i: ; preds = %_RINvNtCshmZ46FhrXRY_4math7fast_db9fast_exp2NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdOTRa1MFkeb_13gate_expander.exit133.i.i
  %_108.i.i244.i = trunc i64 %_26.i to i32, !dbg !15227
  %_107.i.i245.i = add i32 %base.i.i35.i, %_108.i.i244.i, !dbg !15228
  store i32 %_107.i.i245.i, ptr %_51.i28.i, align 4, !dbg !15230, !alias.scope !14073, !noalias !14092
  br label %bb7.i, !dbg !15231

bb26.i:                                           ; preds = %bb25.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %564, i64 noundef range(i64 0, 2305843009213693952) %_38.1, i64 noundef range(i64 0, 2305843009213693952) %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9c634077742a23e4340a846355ecc484) #24, !dbg !15232, !noalias !12090
  unreachable, !dbg !15232

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9run_blockB2_.exit: ; preds = %bb1.backedge.i.i
  call void @llvm.lifetime.end.p0(ptr nonnull %iter.i.i), !dbg !15233, !noalias !13579
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %_0, ptr noundef nonnull align 8 dereferenceable(320) %reports, i64 320, i1 false), !dbg !15234
  %report.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 320, !dbg !15234
  store i8 %1, ptr %report.sroa.7.0._0.sroa_idx, align 8, !dbg !15234
  call void @llvm.lifetime.end.p0(ptr nonnull %reports), !dbg !15235
  br label %bb12, !dbg !12061

bb15:                                             ; preds = %bb4, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit
  %iter.sroa.0.0131 = phi i64 [ 0, %bb4 ], [ %879, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit ]
  %879 = add nuw nsw i64 %iter.sroa.0.0131, 1, !dbg !15236
  %exitcond.not = icmp eq i64 %iter.sroa.0.0131, %_36.1, !dbg !15242
  br i1 %exitcond.not, label %panic, label %bb7, !dbg !15242

bb12:                                             ; preds = %bb3, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E9run_blockB2_.exit
  ret void, !dbg !12061

bb7:                                              ; preds = %bb15
  %880 = getelementptr inbounds nuw i32, ptr %_36.0, i64 %iter.sroa.0.0131, !dbg !15242
  %_14 = load i32, ptr %880, align 4, !dbg !15242, !noundef !12
  %start1 = zext i32 %_14 to i64, !dbg !15242
  %exitcond237.not = icmp eq i64 %iter.sroa.0.0131, %15, !dbg !15244
  br i1 %exitcond237.not, label %panic2, label %bb8, !dbg !15244

panic:                                            ; preds = %bb15
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_36.1, i64 noundef %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_71049c4faf961883bcf0219fde9a65c7) #24, !dbg !15242
  unreachable, !dbg !15242

bb8:                                              ; preds = %bb7
  %881 = getelementptr inbounds nuw i32, ptr %_36.0, i64 %879, !dbg !15244
  %_18 = load i32, ptr %881, align 4, !dbg !15244, !noundef !12
  %end = zext i32 %_18 to i64, !dbg !15244
  %_58 = icmp ult i32 %_18, %_14, !dbg !15246
  %_52.not = icmp ult i64 %_39.1, %end
  %or.cond24 = or i1 %_58, %_52.not, !dbg !15246
  br i1 %or.cond24, label %bb19, label %bb9, !dbg !15246, !prof !2561

panic2:                                           ; preds = %bb7
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %879, i64 noundef %_36.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_71049c4faf961883bcf0219fde9a65c7) #24, !dbg !15244
  unreachable, !dbg !15244

bb19:                                             ; preds = %bb8
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1, i64 noundef %end, i64 noundef %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_71049c4faf961883bcf0219fde9a65c7) #24, !dbg !15254
  unreachable, !dbg !15254

bb9:                                              ; preds = %bb8
  %_59 = sub nuw nsw i64 %end, %start1, !dbg !15255
  %_61 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_39.0, i64 %start1, !dbg !15256
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15260), !dbg !15263
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15264), !dbg !15263
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15266), !dbg !15263
  call void @llvm.lifetime.start.p0(ptr nonnull %pending.i), !dbg !15268, !noalias !15271
  store i32 0, ptr %pending.i, align 4, !noalias !15271
  store i32 0, ptr %_7.sroa.5124.0.pending.sroa_idx.i, align 4, !noalias !15271
  store i32 0, ptr %_7.sroa.6127.0.pending.sroa_idx.i, align 4, !noalias !15271
  store i32 0, ptr %_7.sroa.7130.0.pending.sroa_idx.i, align 4, !noalias !15271
  store i32 0, ptr %11, align 4, !noalias !15271
  store i32 0, ptr %_7.sroa.5124.0..sroa_idx.i, align 4, !noalias !15271
  store i32 0, ptr %_7.sroa.6127.0..sroa_idx.i, align 4, !noalias !15271
  store i32 0, ptr %_7.sroa.7130.0..sroa_idx.i, align 4, !noalias !15271
  %_106.idx.i = mul nuw nsw i64 %_59, 40, !dbg !15272
  %_106.i = getelementptr inbounds nuw i8, ptr %_61, i64 %_106.idx.i, !dbg !15272
  %_6.i.i8790.i = icmp eq i32 %_18, %_14, !dbg !15283
  br i1 %_6.i.i8790.i, label %bb36.preheader.i, label %bb4.lr.ph.lr.ph.i, !dbg !15288

bb4.lr.ph.lr.ph.i:                                ; preds = %bb9
  %_24 = getelementptr inbounds nuw %"effect_contract::ProcessReport", ptr %reports, i64 %iter.sroa.0.0131, !dbg !15289
  %882 = getelementptr inbounds nuw i8, ptr %_24, i64 16
  %_31.i = load i32, ptr %12, align 4, !alias.scope !15260, !noalias !15290
  %_30.i = zext i32 %_31.i to i64
  %.promoted95.i = load i64, ptr %882, align 8, !alias.scope !15266, !noalias !15291
  br label %bb4.lr.ph.i, !dbg !15288

bb4.lr.ph.i:                                      ; preds = %bb31.i, %bb4.lr.ph.lr.ph.i
  %.promoted97.i = phi i64 [ %.promoted95.i, %bb4.lr.ph.lr.ph.i ], [ %.promoted96.i, %bb31.i ]
  %last_order.sroa.3.0.ph94.i = phi i32 [ undef, %bb4.lr.ph.lr.ph.i ], [ %_127.0.i, %bb31.i ]
  %last_order.sroa.0.0.ph93.not.i = phi i1 [ true, %bb4.lr.ph.lr.ph.i ], [ false, %bb31.i ]
  %iter.sroa.0.0.ph92.i = phi ptr [ %_61, %bb4.lr.ph.lr.ph.i ], [ %_16.i.i.i, %bb31.i ]
  %iter.sroa.7.0.ph91.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %bb4.i10, !dbg !15288

bb36.preheader.i:                                 ; preds = %bb31.i, %bb35.i, %bb9
  %883 = getelementptr inbounds nuw float, ptr %_84.i, i64 %iter.sroa.0.0131
  %884 = getelementptr inbounds nuw float, ptr %words.i.i, i64 %iter.sroa.0.0131
  %885 = getelementptr inbounds nuw float, ptr %words.i63.i, i64 %iter.sroa.0.0131
  %886 = getelementptr inbounds nuw float, ptr %words.i64.i, i64 %iter.sroa.0.0131
  %887 = getelementptr inbounds nuw float, ptr %words.i65.i, i64 %iter.sroa.0.0131
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, !dbg !15292

bb4.i10:                                          ; preds = %bb35.i, %bb4.lr.ph.i
  %.promoted96.i = phi i64 [ %.promoted97.i, %bb4.lr.ph.i ], [ %934, %bb35.i ]
  %iter.sroa.0.089.i = phi ptr [ %iter.sroa.0.0.ph92.i, %bb4.lr.ph.i ], [ %_16.i.i.i, %bb35.i ]
  %iter.sroa.7.088.i = phi i64 [ %iter.sroa.7.0.ph91.i, %bb4.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 40, !dbg !15296
  %_9.0.i.i = add i64 %iter.sroa.7.088.i, 1, !dbg !15298
  %888 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 32, !dbg !15299
  %_17.i = load i32, ptr %888, align 8, !dbg !15299, !range !1335, !alias.scope !15264, !noalias !15301, !noundef !12
  switch i32 %_17.i, label %default.unreachable [
    i32 1, label %bb9.i12
    i32 2, label %bb7.i11
    i32 3, label %bb35.i
  ], !dbg !15302

bb36.loopexit.i:                                  ; preds = %bb46.us.3.i, %bb40.backedge.us.2.i
  %_6.i.i44.i = icmp eq i64 %iter1.sroa.0.0.add.i, 64, !dbg !15303
  br i1 %_6.i.i44.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, !dbg !15292

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i: ; preds = %bb36.loopexit.i, %bb36.preheader.i
  %iter1.sroa.0.0.idx107.i = phi i64 [ 0, %bb36.preheader.i ], [ %iter1.sroa.0.0.add.i, %bb36.loopexit.i ]
  %iter1.sroa.7.0106.i = phi i64 [ 0, %bb36.preheader.i ], [ %_9.0.i48.i, %bb36.loopexit.i ]
  %iter1.sroa.0.0.ptr.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 %iter1.sroa.0.0.idx107.i, !dbg !15303
  %iter1.sroa.0.0.add.i = add nuw nsw i64 %iter1.sroa.0.0.idx107.i, 32, !dbg !15305
  %_9.0.i48.i = add nuw nsw i64 %iter1.sroa.7.0106.i, 1, !dbg !15307
  %889 = getelementptr inbounds nuw %"kernel::GateState<wide::f32x8_::f32x8>", ptr %13, i64 %iter1.sroa.7.0106.i
  %890 = load i32, ptr %iter1.sroa.0.0.ptr.i, align 4, !dbg !15308, !range !5716, !noalias !15271, !noundef !12
  %891 = trunc nuw i32 %890 to i1, !dbg !15312
  br i1 %891, label %bb46.us.i, label %bb40.backedge.us.i, !dbg !15312

bb46.us.i:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %892 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 4, !dbg !15308
  %value.us.i = load float, ptr %892, align 4, !dbg !15313, !noalias !15271, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %_84.i), !dbg !15314, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_84.i, ptr noundef nonnull align 32 dereferenceable(32) %889, i64 32, i1 false), !dbg !15314, !noalias !15290
  %_0.i.us.i = load float, ptr %883, align 4, !dbg !15317, !alias.scope !15319, !noalias !15271, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_84.i), !dbg !15322, !noalias !15271
  %893 = getelementptr inbounds nuw i8, ptr %889, i64 32, !dbg !15323
  %894 = getelementptr inbounds nuw i8, ptr %889, i64 64, !dbg !15324
  %895 = getelementptr inbounds nuw i8, ptr %889, i64 96, !dbg !15325
  %_148.us.i = bitcast float %_0.i.us.i to i32, !dbg !15326
  %_150.us.i = bitcast float %value.us.i to i32, !dbg !15334
  %_149.us.i = icmp ne i32 %_148.us.i, %_150.us.i, !dbg !15337
  %896 = icmp eq i32 %_148.us.i, -2147483648
  %or.cond.us.i = or i1 %_149.us.i, %896, !dbg !15337
  %897 = tail call float @llvm.fabs.f32(float %_0.i.us.i)
  %_144.us.i = fcmp ueq float %897, 0x7FF0000000000000
  %or.cond40.us.i = or i1 %_144.us.i, %or.cond.us.i, !dbg !15337
  %_146.us.i = fsub float %value.us.i, %_0.i.us.i, !dbg !15337
  %898 = fmul float %_146.us.i, 1.562500e-02, !dbg !15337
  %ramp.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %_0.i.us.i, float %value.us.i, !dbg !15337
  %ramp4.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %898, float 0.000000e+00, !dbg !15337
  %ramp5.sroa.0.0.us.i = select i1 %or.cond40.us.i, float 6.400000e+01, float 0.000000e+00, !dbg !15337
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !15338, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, ptr noundef nonnull align 32 dereferenceable(32) %889, i64 32, i1 false), !dbg !15338, !noalias !15290
  store float %ramp.sroa.0.0.us.i, ptr %884, align 4, !dbg !15340, !noalias !15341
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %889, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, i64 32, i1 false), !dbg !15344, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !15345, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i63.i), !dbg !15346, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, ptr noundef nonnull align 32 dereferenceable(32) %893, i64 32, i1 false), !dbg !15346, !noalias !15290
  store float %value.us.i, ptr %885, align 4, !dbg !15348, !noalias !15349
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %893, ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, i64 32, i1 false), !dbg !15352, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i63.i), !dbg !15353, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i64.i), !dbg !15354, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, ptr noundef nonnull align 32 dereferenceable(32) %894, i64 32, i1 false), !dbg !15354, !noalias !15290
  store float %ramp4.sroa.0.0.us.i, ptr %886, align 4, !dbg !15356, !noalias !15357
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %894, ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, i64 32, i1 false), !dbg !15360, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i64.i), !dbg !15361, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i65.i), !dbg !15362, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, ptr noundef nonnull align 32 dereferenceable(32) %895, i64 32, i1 false), !dbg !15362, !noalias !15290
  store float %ramp5.sroa.0.0.us.i, ptr %887, align 4, !dbg !15364, !noalias !15365
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %895, ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, i64 32, i1 false), !dbg !15368, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i65.i), !dbg !15369, !noalias !15271
  store i32 64, ptr %14, align 4, !dbg !15370, !alias.scope !15260, !noalias !15290
  br label %bb40.backedge.us.i, !dbg !15371

bb40.backedge.us.i:                               ; preds = %bb46.us.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %iter2.sroa.0.0.ptr102.us.1.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 8, !dbg !15372
  %899 = load i32, ptr %iter2.sroa.0.0.ptr102.us.1.i, align 4, !dbg !15308, !range !5716, !noalias !15271, !noundef !12
  %900 = trunc nuw i32 %899 to i1, !dbg !15312
  br i1 %900, label %bb46.us.1.i, label %bb40.backedge.us.1.i, !dbg !15312

bb46.us.1.i:                                      ; preds = %bb40.backedge.us.i
  %901 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 12, !dbg !15308
  %value.us.1.i = load float, ptr %901, align 4, !dbg !15313, !noalias !15271, !noundef !12
  %slot.us.1.i = getelementptr inbounds nuw i8, ptr %889, i64 128, !dbg !15376
  call void @llvm.lifetime.start.p0(ptr nonnull %_84.i), !dbg !15314, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_84.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.1.i, i64 32, i1 false), !dbg !15314, !noalias !15290
  %_0.i.us.1.i = load float, ptr %883, align 4, !dbg !15317, !alias.scope !15319, !noalias !15271, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_84.i), !dbg !15322, !noalias !15271
  %902 = getelementptr inbounds nuw i8, ptr %889, i64 160, !dbg !15323
  %903 = getelementptr inbounds nuw i8, ptr %889, i64 192, !dbg !15324
  %904 = getelementptr inbounds nuw i8, ptr %889, i64 224, !dbg !15325
  %_148.us.1.i = bitcast float %_0.i.us.1.i to i32, !dbg !15326
  %_150.us.1.i = bitcast float %value.us.1.i to i32, !dbg !15334
  %_149.us.1.i = icmp ne i32 %_148.us.1.i, %_150.us.1.i, !dbg !15337
  %905 = icmp eq i32 %_148.us.1.i, -2147483648
  %or.cond.us.1.i = or i1 %_149.us.1.i, %905, !dbg !15337
  %906 = tail call float @llvm.fabs.f32(float %_0.i.us.1.i)
  %_144.us.1.i = fcmp ueq float %906, 0x7FF0000000000000
  %or.cond40.us.1.i = or i1 %_144.us.1.i, %or.cond.us.1.i, !dbg !15337
  %_146.us.1.i = fsub float %value.us.1.i, %_0.i.us.1.i, !dbg !15337
  %907 = fmul float %_146.us.1.i, 1.562500e-02, !dbg !15337
  %ramp.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %_0.i.us.1.i, float %value.us.1.i, !dbg !15337
  %ramp4.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %907, float 0.000000e+00, !dbg !15337
  %ramp5.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !15337
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !15338, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.1.i, i64 32, i1 false), !dbg !15338, !noalias !15290
  store float %ramp.sroa.0.0.us.1.i, ptr %884, align 4, !dbg !15340, !noalias !15341
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %slot.us.1.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, i64 32, i1 false), !dbg !15344, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !15345, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i63.i), !dbg !15346, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, ptr noundef nonnull align 32 dereferenceable(32) %902, i64 32, i1 false), !dbg !15346, !noalias !15290
  store float %value.us.1.i, ptr %885, align 4, !dbg !15348, !noalias !15349
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %902, ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, i64 32, i1 false), !dbg !15352, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i63.i), !dbg !15353, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i64.i), !dbg !15354, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, ptr noundef nonnull align 32 dereferenceable(32) %903, i64 32, i1 false), !dbg !15354, !noalias !15290
  store float %ramp4.sroa.0.0.us.1.i, ptr %886, align 4, !dbg !15356, !noalias !15357
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %903, ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, i64 32, i1 false), !dbg !15360, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i64.i), !dbg !15361, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i65.i), !dbg !15362, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, ptr noundef nonnull align 32 dereferenceable(32) %904, i64 32, i1 false), !dbg !15362, !noalias !15290
  store float %ramp5.sroa.0.0.us.1.i, ptr %887, align 4, !dbg !15364, !noalias !15365
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %904, ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, i64 32, i1 false), !dbg !15368, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i65.i), !dbg !15369, !noalias !15271
  store i32 64, ptr %14, align 4, !dbg !15370, !alias.scope !15260, !noalias !15290
  br label %bb40.backedge.us.1.i, !dbg !15371

bb40.backedge.us.1.i:                             ; preds = %bb46.us.1.i, %bb40.backedge.us.i
  %iter2.sroa.0.0.ptr102.us.2.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 16, !dbg !15372
  %908 = load i32, ptr %iter2.sroa.0.0.ptr102.us.2.i, align 4, !dbg !15308, !range !5716, !noalias !15271, !noundef !12
  %909 = trunc nuw i32 %908 to i1, !dbg !15312
  br i1 %909, label %bb46.us.2.i, label %bb40.backedge.us.2.i, !dbg !15312

bb46.us.2.i:                                      ; preds = %bb40.backedge.us.1.i
  %910 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 20, !dbg !15308
  %value.us.2.i = load float, ptr %910, align 4, !dbg !15313, !noalias !15271, !noundef !12
  %slot.us.2.i = getelementptr inbounds nuw i8, ptr %889, i64 256, !dbg !15376
  call void @llvm.lifetime.start.p0(ptr nonnull %_84.i), !dbg !15314, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_84.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.2.i, i64 32, i1 false), !dbg !15314, !noalias !15290
  %_0.i.us.2.i = load float, ptr %883, align 4, !dbg !15317, !alias.scope !15319, !noalias !15271, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_84.i), !dbg !15322, !noalias !15271
  %911 = getelementptr inbounds nuw i8, ptr %889, i64 288, !dbg !15323
  %912 = getelementptr inbounds nuw i8, ptr %889, i64 320, !dbg !15324
  %913 = getelementptr inbounds nuw i8, ptr %889, i64 352, !dbg !15325
  %_148.us.2.i = bitcast float %_0.i.us.2.i to i32, !dbg !15326
  %_150.us.2.i = bitcast float %value.us.2.i to i32, !dbg !15334
  %_149.us.2.i = icmp ne i32 %_148.us.2.i, %_150.us.2.i, !dbg !15337
  %914 = icmp eq i32 %_148.us.2.i, -2147483648
  %or.cond.us.2.i = or i1 %_149.us.2.i, %914, !dbg !15337
  %915 = tail call float @llvm.fabs.f32(float %_0.i.us.2.i)
  %_144.us.2.i = fcmp ueq float %915, 0x7FF0000000000000
  %or.cond40.us.2.i = or i1 %_144.us.2.i, %or.cond.us.2.i, !dbg !15337
  %_146.us.2.i = fsub float %value.us.2.i, %_0.i.us.2.i, !dbg !15337
  %916 = fmul float %_146.us.2.i, 1.562500e-02, !dbg !15337
  %ramp.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %_0.i.us.2.i, float %value.us.2.i, !dbg !15337
  %ramp4.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %916, float 0.000000e+00, !dbg !15337
  %ramp5.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float 6.400000e+01, float 0.000000e+00, !dbg !15337
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !15338, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.2.i, i64 32, i1 false), !dbg !15338, !noalias !15290
  store float %ramp.sroa.0.0.us.2.i, ptr %884, align 4, !dbg !15340, !noalias !15341
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %slot.us.2.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, i64 32, i1 false), !dbg !15344, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !15345, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i63.i), !dbg !15346, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, ptr noundef nonnull align 32 dereferenceable(32) %911, i64 32, i1 false), !dbg !15346, !noalias !15290
  store float %value.us.2.i, ptr %885, align 4, !dbg !15348, !noalias !15349
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %911, ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, i64 32, i1 false), !dbg !15352, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i63.i), !dbg !15353, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i64.i), !dbg !15354, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, ptr noundef nonnull align 32 dereferenceable(32) %912, i64 32, i1 false), !dbg !15354, !noalias !15290
  store float %ramp4.sroa.0.0.us.2.i, ptr %886, align 4, !dbg !15356, !noalias !15357
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %912, ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, i64 32, i1 false), !dbg !15360, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i64.i), !dbg !15361, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i65.i), !dbg !15362, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, ptr noundef nonnull align 32 dereferenceable(32) %913, i64 32, i1 false), !dbg !15362, !noalias !15290
  store float %ramp5.sroa.0.0.us.2.i, ptr %887, align 4, !dbg !15364, !noalias !15365
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %913, ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, i64 32, i1 false), !dbg !15368, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i65.i), !dbg !15369, !noalias !15271
  store i32 64, ptr %14, align 4, !dbg !15370, !alias.scope !15260, !noalias !15290
  br label %bb40.backedge.us.2.i, !dbg !15371

bb40.backedge.us.2.i:                             ; preds = %bb46.us.2.i, %bb40.backedge.us.1.i
  %iter2.sroa.0.0.ptr102.us.3.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 24, !dbg !15372
  %917 = load i32, ptr %iter2.sroa.0.0.ptr102.us.3.i, align 4, !dbg !15308, !range !5716, !noalias !15271, !noundef !12
  %918 = trunc nuw i32 %917 to i1, !dbg !15312
  br i1 %918, label %bb46.us.3.i, label %bb36.loopexit.i, !dbg !15312

bb46.us.3.i:                                      ; preds = %bb40.backedge.us.2.i
  %919 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0.ptr.i, i64 28, !dbg !15308
  %value.us.3.i = load float, ptr %919, align 4, !dbg !15313, !noalias !15271, !noundef !12
  %slot.us.3.i = getelementptr inbounds nuw i8, ptr %889, i64 384, !dbg !15376
  call void @llvm.lifetime.start.p0(ptr nonnull %_84.i), !dbg !15314, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_84.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.3.i, i64 32, i1 false), !dbg !15314, !noalias !15290
  %_0.i.us.3.i = load float, ptr %883, align 4, !dbg !15317, !alias.scope !15319, !noalias !15271, !noundef !12
  call void @llvm.lifetime.end.p0(ptr nonnull %_84.i), !dbg !15322, !noalias !15271
  %920 = getelementptr inbounds nuw i8, ptr %889, i64 416, !dbg !15323
  %921 = getelementptr inbounds nuw i8, ptr %889, i64 448, !dbg !15324
  %922 = getelementptr inbounds nuw i8, ptr %889, i64 480, !dbg !15325
  %_148.us.3.i = bitcast float %_0.i.us.3.i to i32, !dbg !15326
  %_150.us.3.i = bitcast float %value.us.3.i to i32, !dbg !15334
  %_149.us.3.i = icmp ne i32 %_148.us.3.i, %_150.us.3.i, !dbg !15337
  %923 = icmp eq i32 %_148.us.3.i, -2147483648
  %or.cond.us.3.i = or i1 %_149.us.3.i, %923, !dbg !15337
  %924 = tail call float @llvm.fabs.f32(float %_0.i.us.3.i)
  %_144.us.3.i = fcmp ueq float %924, 0x7FF0000000000000
  %or.cond40.us.3.i = or i1 %_144.us.3.i, %or.cond.us.3.i, !dbg !15337
  %_146.us.3.i = fsub float %value.us.3.i, %_0.i.us.3.i, !dbg !15337
  %925 = fmul float %_146.us.3.i, 1.562500e-02, !dbg !15337
  %ramp.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %_0.i.us.3.i, float %value.us.3.i, !dbg !15337
  %ramp4.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %925, float 0.000000e+00, !dbg !15337
  %ramp5.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float 6.400000e+01, float 0.000000e+00, !dbg !15337
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i.i), !dbg !15338, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, ptr noundef nonnull align 32 dereferenceable(32) %slot.us.3.i, i64 32, i1 false), !dbg !15338, !noalias !15290
  store float %ramp.sroa.0.0.us.3.i, ptr %884, align 4, !dbg !15340, !noalias !15341
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %slot.us.3.i, ptr noundef nonnull align 32 dereferenceable(32) %words.i.i, i64 32, i1 false), !dbg !15344, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i.i), !dbg !15345, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i63.i), !dbg !15346, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, ptr noundef nonnull align 32 dereferenceable(32) %920, i64 32, i1 false), !dbg !15346, !noalias !15290
  store float %value.us.3.i, ptr %885, align 4, !dbg !15348, !noalias !15349
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %920, ptr noundef nonnull align 32 dereferenceable(32) %words.i63.i, i64 32, i1 false), !dbg !15352, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i63.i), !dbg !15353, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i64.i), !dbg !15354, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, ptr noundef nonnull align 32 dereferenceable(32) %921, i64 32, i1 false), !dbg !15354, !noalias !15290
  store float %ramp4.sroa.0.0.us.3.i, ptr %886, align 4, !dbg !15356, !noalias !15357
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %921, ptr noundef nonnull align 32 dereferenceable(32) %words.i64.i, i64 32, i1 false), !dbg !15360, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i64.i), !dbg !15361, !noalias !15271
  call void @llvm.lifetime.start.p0(ptr nonnull %words.i65.i), !dbg !15362, !noalias !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, ptr noundef nonnull align 32 dereferenceable(32) %922, i64 32, i1 false), !dbg !15362, !noalias !15290
  store float %ramp5.sroa.0.0.us.3.i, ptr %887, align 4, !dbg !15364, !noalias !15365
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %922, ptr noundef nonnull align 32 dereferenceable(32) %words.i65.i, i64 32, i1 false), !dbg !15368, !noalias !15290
  call void @llvm.lifetime.end.p0(ptr nonnull %words.i65.i), !dbg !15369, !noalias !15271
  store i32 64, ptr %14, align 4, !dbg !15370, !alias.scope !15260, !noalias !15290
  br label %bb36.loopexit.i, !dbg !15371

default.unreachable:                              ; preds = %bb4.i10
  unreachable

bb7.i11:                                          ; preds = %bb4.i10
  br label %bb9.i12, !dbg !15377

bb9.i12:                                          ; preds = %bb7.i11, %bb4.i10
  %channel.sroa.0.0.sroa.phi28.i = phi ptr [ %11, %bb7.i11 ], [ %pending.i, %bb4.i10 ], !dbg !15378
  %channel.sroa.0.0.i = phi i32 [ 1, %bb7.i11 ], [ 0, %bb4.i10 ], !dbg !15378
  %926 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 16, !dbg !15379
  %_21.i = load i32, ptr %926, align 8, !dbg !15379, !alias.scope !15264, !noalias !15301, !noundef !12
  %parameter_index.i = zext i32 %_21.i to i64, !dbg !15379
  %_121.1.i = icmp slt i32 %_21.i, 0, !dbg !15381
  br i1 %_121.1.i, label %bb35.i, label %bb59.i, !dbg !15387, !prof !180

bb59.i:                                           ; preds = %bb9.i12
  %_121.0.i = shl nuw i32 %_21.i, 1, !dbg !15381
  %_127.0.i = or disjoint i32 %_121.0.i, %channel.sroa.0.0.i, !dbg !15391
  %_29.i = icmp ult i64 %iter.sroa.7.088.i, %_30.i, !dbg !15399
  %_32.i = icmp samesign ult i32 %_21.i, 4
  %or.cond16.i = and i1 %_29.i, %_32.i, !dbg !15399
  br i1 %or.cond16.i, label %bb11.i, label %bb35.i, !dbg !15399

bb11.i:                                           ; preds = %bb59.i
  %927 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 28, !dbg !15401
  %_130.i = load i32, ptr %927, align 4, !dbg !15401, !range !6171, !alias.scope !15264, !noalias !15301, !noundef !12
  %928 = icmp eq i32 %_130.i, 1, !dbg !15404
  br i1 %928, label %bb12.i13, label %bb35.i, !dbg !15404

bb12.i13:                                         ; preds = %bb11.i
  %_34.i = load i64, ptr %iter.sroa.0.089.i, align 8, !dbg !15405, !alias.scope !15264, !noalias !15301, !noundef !12
  %_33.i = icmp eq i64 %_34.i, %_23, !dbg !15405
  br i1 %_33.i, label %bb13.i, label %bb35.i, !dbg !15405

bb13.i:                                           ; preds = %bb12.i13
  %929 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 8, !dbg !15406
  %_36.i = load i64, ptr %929, align 8, !dbg !15406, !alias.scope !15264, !noalias !15301, !noundef !12
  %_35.i = icmp eq i64 %_36.i, %_23, !dbg !15406
  br i1 %_35.i, label %bb14.i14, label %bb35.i, !dbg !15406

bb14.i14:                                         ; preds = %bb13.i
  %930 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 20, !dbg !15407
  %_39.i = load float, ptr %930, align 4, !dbg !15407, !alias.scope !15264, !noalias !15301, !noundef !12
  %_38.i = bitcast float %_39.i to i32, !dbg !15408
  %931 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.089.i, i64 24, !dbg !15410
  %_4139.i = load i32, ptr %931, align 8, !dbg !15410, !alias.scope !15264, !noalias !15301, !noundef !12
  %_37.i = icmp eq i32 %_4139.i, %_38.i, !dbg !15407
  br i1 %_37.i, label %bb16.i15, label %bb35.i, !dbg !15407

bb16.i15:                                         ; preds = %bb14.i14
  %_43.i = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_d0058eaee52f1ba8b2e5577cef0c3c7f, i64 %parameter_index.i, !dbg !15411
; call effect_runtime::params::parameter_value_valid
  %_42.i = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_43.i, float noundef %_39.i) #23, !dbg !15412, !noalias !15271
  %_45.i = icmp ugt i32 %_127.0.i, %last_order.sroa.3.0.ph94.i
  %or.cond17.not.not108.i = select i1 %last_order.sroa.0.0.ph93.not.i, i1 true, i1 %_45.i, !dbg !15412
  %or.cond41.not.i = select i1 %_42.i, i1 %or.cond17.not.not108.i, i1 false, !dbg !15412
  br i1 %or.cond41.not.i, label %bb29.i, label %bb35.i, !dbg !15412

bb29.i:                                           ; preds = %bb16.i15
  %_47.i = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i, !dbg !15413
  %932 = load i32, ptr %_47.i, align 4, !dbg !15414, !range !5716, !noalias !15271, !noundef !12
  %_133.not.i = icmp eq i32 %932, 0, !dbg !15421
  br i1 %_133.not.i, label %bb31.i, label %bb35.i, !dbg !15422

bb31.i:                                           ; preds = %bb29.i
  %_47.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i
  %_135.i = fcmp oeq float %_39.i, 0.000000e+00, !dbg !15424
  %_55.sroa.0.0.i = select i1 %_135.i, float 0.000000e+00, float %_39.i, !dbg !15424
  store i32 1, ptr %_47.i.le, align 4, !dbg !15427, !noalias !15271
  %933 = getelementptr inbounds nuw i8, ptr %_47.i.le, i64 4, !dbg !15427
  store float %_55.sroa.0.0.i, ptr %933, align 4, !dbg !15427, !noalias !15271
  %_6.i.i87.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !15283
  br i1 %_6.i.i87.i, label %bb36.preheader.i, label %bb4.lr.ph.i, !dbg !15288

bb35.i:                                           ; preds = %bb29.i, %bb16.i15, %bb14.i14, %bb13.i, %bb12.i13, %bb11.i, %bb59.i, %bb9.i12, %bb4.i10
  %934 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted96.i, i64 1), !dbg !15428
  store i64 %934, ptr %882, align 8, !dbg !15378, !alias.scope !15266, !noalias !15291
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !15283
  br i1 %_6.i.i.i, label %bb36.preheader.i, label %bb4.i10, !dbg !15288

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGateNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8Kb0_E16apply_automationB2_.exit: ; preds = %bb36.loopexit.i
  call void @llvm.lifetime.end.p0(ptr nonnull %pending.i), !dbg !15431, !noalias !15271
  %exitcond238.not = icmp eq i64 %879, 8, !dbg !15432
  br i1 %exitcond238.not, label %bb16, label %bb15, !dbg !12064
}
