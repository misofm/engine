define internal fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr dead_on_return noalias noundef nonnull readonly align 32 captures(none) dereferenceable(736) %self, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(200) %state) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !15371 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15372), !dbg !15375
  %0 = getelementptr inbounds nuw i8, ptr %state, i64 192, !dbg !15376
  %1 = load i64, ptr %0, align 8, !dbg !15376, !alias.scope !15372, !noalias !15379, !noundef !12
  %_56.0.i = load ptr, ptr %state, align 8, !dbg !15381, !alias.scope !15372, !noalias !15379, !nonnull !12, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %state, i64 8, !dbg !15381
  %_56.1.i = load i64, ptr %2, align 8, !dbg !15381, !alias.scope !15372, !noalias !15379, !noundef !12
  %_8.i.i.i = icmp samesign ugt i64 %_56.1.i, 7, !dbg !15383
  br i1 %_8.i.i.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit.i, label %bb2.i.i.i, !dbg !15383, !prof !651

bb2.i.i.i:                                        ; preds = %start
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15391, !noalias !15392
  unreachable, !dbg !15391

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit.i: ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_56.0.i, ptr noundef nonnull readonly align 32 dereferenceable(384) %self, i64 32, i1 false), !dbg !15399, !noalias !15372
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32, !dbg !15403
  %_9.i.i = icmp ugt i64 %1, %_56.1.i, !dbg !15404
  br i1 %_9.i.i, label %bb2.i.i, label %bb3.i.i, !dbg !15404, !prof !639

bb3.i.i:                                          ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit.i
  %_12.i.i = sub nuw i64 %_56.1.i, %1, !dbg !15410
  %_8.i.i4.i = icmp samesign ugt i64 %_12.i.i, 7, !dbg !15411
  br i1 %_8.i.i4.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit7.i, label %bb2.i.i5.i, !dbg !15411, !prof !651

bb2.i.i5.i:                                       ; preds = %bb3.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15416, !noalias !15417
  unreachable, !dbg !15416

bb2.i.i:                                          ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %1, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15424, !noalias !15425
  unreachable, !dbg !15424

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit7.i: ; preds = %bb3.i.i
  %_16.i.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %1, !dbg !15426
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %3, i64 32, i1 false), !dbg !15431, !noalias !15372
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 64, !dbg !15435
  %_5.i.i = shl i64 %1, 1, !dbg !15436
  %_9.i11.i = icmp ugt i64 %_5.i.i, %_56.1.i, !dbg !15438
  br i1 %_9.i11.i, label %bb2.i18.i, label %bb3.i12.i, !dbg !15438, !prof !639

bb3.i12.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit7.i
  %_12.i13.i = sub nuw i64 %_56.1.i, %_5.i.i, !dbg !15441
  %_8.i.i14.i = icmp samesign ugt i64 %_12.i13.i, 7, !dbg !15442
  br i1 %_8.i.i14.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit19.i, label %bb2.i.i15.i, !dbg !15442, !prof !651

bb2.i.i15.i:                                      ; preds = %bb3.i12.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i13.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15447, !noalias !15448
  unreachable, !dbg !15447

bb2.i18.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit7.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15455, !noalias !15456
  unreachable, !dbg !15455

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit19.i: ; preds = %bb3.i12.i
  %_16.i17.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i.i, !dbg !15457
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i17.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %4, i64 32, i1 false), !dbg !15459, !noalias !15372
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 96, !dbg !15463
  %_5.i23.i = mul i64 %1, 3, !dbg !15464
  %_9.i24.i = icmp ugt i64 %_5.i23.i, %_56.1.i, !dbg !15466
  br i1 %_9.i24.i, label %bb2.i31.i, label %bb3.i25.i, !dbg !15466, !prof !639

bb3.i25.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit19.i
  %_12.i26.i = sub nuw i64 %_56.1.i, %_5.i23.i, !dbg !15469
  %_8.i.i27.i = icmp samesign ugt i64 %_12.i26.i, 7, !dbg !15470
  br i1 %_8.i.i27.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit32.i, label %bb2.i.i28.i, !dbg !15470, !prof !651

bb2.i.i28.i:                                      ; preds = %bb3.i25.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15475, !noalias !15476
  unreachable, !dbg !15475

bb2.i31.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit19.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i23.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15483, !noalias !15484
  unreachable, !dbg !15483

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit32.i: ; preds = %bb3.i25.i
  %_16.i30.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i23.i, !dbg !15485
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i30.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %5, i64 32, i1 false), !dbg !15487, !noalias !15372
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !15491
  %_5.i36.i = shl i64 %1, 2, !dbg !15492
  %_9.i37.i = icmp ugt i64 %_5.i36.i, %_56.1.i, !dbg !15494
  br i1 %_9.i37.i, label %bb2.i44.i, label %bb3.i38.i, !dbg !15494, !prof !639

bb3.i38.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit32.i
  %_12.i39.i = sub nuw i64 %_56.1.i, %_5.i36.i, !dbg !15497
  %_8.i.i40.i = icmp samesign ugt i64 %_12.i39.i, 7, !dbg !15498
  br i1 %_8.i.i40.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit45.i, label %bb2.i.i41.i, !dbg !15498, !prof !651

bb2.i.i41.i:                                      ; preds = %bb3.i38.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i39.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15503, !noalias !15504
  unreachable, !dbg !15503

bb2.i44.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit32.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i36.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15511, !noalias !15512
  unreachable, !dbg !15511

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit45.i: ; preds = %bb3.i38.i
  %_16.i43.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i36.i, !dbg !15513
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i43.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %6, i64 32, i1 false), !dbg !15515, !noalias !15372
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 160, !dbg !15519
  %_5.i49.i = mul i64 %1, 5, !dbg !15520
  %_9.i50.i = icmp ugt i64 %_5.i49.i, %_56.1.i, !dbg !15522
  br i1 %_9.i50.i, label %bb2.i57.i, label %bb3.i51.i, !dbg !15522, !prof !639

bb3.i51.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit45.i
  %_12.i52.i = sub nuw i64 %_56.1.i, %_5.i49.i, !dbg !15525
  %_8.i.i53.i = icmp samesign ugt i64 %_12.i52.i, 7, !dbg !15526
  br i1 %_8.i.i53.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit58.i, label %bb2.i.i54.i, !dbg !15526, !prof !651

bb2.i.i54.i:                                      ; preds = %bb3.i51.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i52.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15531, !noalias !15532
  unreachable, !dbg !15531

bb2.i57.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit45.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i49.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15539, !noalias !15540
  unreachable, !dbg !15539

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit58.i: ; preds = %bb3.i51.i
  %_16.i56.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i49.i, !dbg !15541
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i56.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %7, i64 32, i1 false), !dbg !15543, !noalias !15372
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 192, !dbg !15547
  %_5.i62.i = mul i64 %1, 6, !dbg !15548
  %_9.i63.i = icmp ugt i64 %_5.i62.i, %_56.1.i, !dbg !15550
  br i1 %_9.i63.i, label %bb2.i70.i, label %bb3.i64.i, !dbg !15550, !prof !639

bb3.i64.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit58.i
  %_12.i65.i = sub nuw i64 %_56.1.i, %_5.i62.i, !dbg !15553
  %_8.i.i66.i = icmp samesign ugt i64 %_12.i65.i, 7, !dbg !15554
  br i1 %_8.i.i66.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit71.i, label %bb2.i.i67.i, !dbg !15554, !prof !651

bb2.i.i67.i:                                      ; preds = %bb3.i64.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i65.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15559, !noalias !15560
  unreachable, !dbg !15559

bb2.i70.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit58.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i62.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15567, !noalias !15568
  unreachable, !dbg !15567

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit71.i: ; preds = %bb3.i64.i
  %_16.i69.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i62.i, !dbg !15569
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i69.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %8, i64 32, i1 false), !dbg !15571, !noalias !15372
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 224, !dbg !15575
  %_5.i75.i = mul i64 %1, 7, !dbg !15576
  %_9.i76.i = icmp ugt i64 %_5.i75.i, %_56.1.i, !dbg !15578
  br i1 %_9.i76.i, label %bb2.i83.i, label %bb3.i77.i, !dbg !15578, !prof !639

bb3.i77.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit71.i
  %_12.i78.i = sub nuw i64 %_56.1.i, %_5.i75.i, !dbg !15581
  %_8.i.i79.i = icmp samesign ugt i64 %_12.i78.i, 7, !dbg !15582
  br i1 %_8.i.i79.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit84.i, label %bb2.i.i80.i, !dbg !15582, !prof !651

bb2.i.i80.i:                                      ; preds = %bb3.i77.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i78.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15587, !noalias !15588
  unreachable, !dbg !15587

bb2.i83.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit71.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i75.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15595, !noalias !15596
  unreachable, !dbg !15595

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit84.i: ; preds = %bb3.i77.i
  %_16.i82.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i75.i, !dbg !15597
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i82.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %9, i64 32, i1 false), !dbg !15599, !noalias !15372
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !15603
  %_5.i88.i = shl i64 %1, 3, !dbg !15604
  %_9.i89.i = icmp ugt i64 %_5.i88.i, %_56.1.i, !dbg !15606
  br i1 %_9.i89.i, label %bb2.i96.i, label %bb3.i90.i, !dbg !15606, !prof !639

bb3.i90.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit84.i
  %_12.i91.i = sub nuw i64 %_56.1.i, %_5.i88.i, !dbg !15609
  %_8.i.i92.i = icmp samesign ugt i64 %_12.i91.i, 7, !dbg !15610
  br i1 %_8.i.i92.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit97.i, label %bb2.i.i93.i, !dbg !15610, !prof !651

bb2.i.i93.i:                                      ; preds = %bb3.i90.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i91.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15615, !noalias !15616
  unreachable, !dbg !15615

bb2.i96.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit84.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i88.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15623, !noalias !15624
  unreachable, !dbg !15623

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit97.i: ; preds = %bb3.i90.i
  %_16.i95.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i88.i, !dbg !15625
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i95.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %10, i64 32, i1 false), !dbg !15627, !noalias !15372
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !15631
  %_5.i101.i = mul i64 %1, 9, !dbg !15632
  %_9.i102.i = icmp ugt i64 %_5.i101.i, %_56.1.i, !dbg !15634
  br i1 %_9.i102.i, label %bb2.i109.i, label %bb3.i103.i, !dbg !15634, !prof !639

bb3.i103.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit97.i
  %_12.i104.i = sub nuw i64 %_56.1.i, %_5.i101.i, !dbg !15637
  %_8.i.i105.i = icmp samesign ugt i64 %_12.i104.i, 7, !dbg !15638
  br i1 %_8.i.i105.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit110.i, label %bb2.i.i106.i, !dbg !15638, !prof !651

bb2.i.i106.i:                                     ; preds = %bb3.i103.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i104.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15643, !noalias !15644
  unreachable, !dbg !15643

bb2.i109.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit97.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i101.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15651, !noalias !15652
  unreachable, !dbg !15651

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit110.i: ; preds = %bb3.i103.i
  %_16.i108.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i101.i, !dbg !15653
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i108.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %11, i64 32, i1 false), !dbg !15655, !noalias !15372
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !15659
  %_5.i114.i = mul i64 %1, 10, !dbg !15660
  %_9.i115.i = icmp ugt i64 %_5.i114.i, %_56.1.i, !dbg !15662
  br i1 %_9.i115.i, label %bb2.i122.i, label %bb3.i116.i, !dbg !15662, !prof !639

bb3.i116.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit110.i
  %_12.i117.i = sub nuw i64 %_56.1.i, %_5.i114.i, !dbg !15665
  %_8.i.i118.i = icmp samesign ugt i64 %_12.i117.i, 7, !dbg !15666
  br i1 %_8.i.i118.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit123.i, label %bb2.i.i119.i, !dbg !15666, !prof !651

bb2.i.i119.i:                                     ; preds = %bb3.i116.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i117.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15671, !noalias !15672
  unreachable, !dbg !15671

bb2.i122.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit110.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i114.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15679, !noalias !15680
  unreachable, !dbg !15679

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit123.i: ; preds = %bb3.i116.i
  %_16.i121.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i114.i, !dbg !15681
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i121.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %12, i64 32, i1 false), !dbg !15683, !noalias !15372
  %_5.i127.i = mul i64 %1, 11, !dbg !15687
  %_9.i128.i = icmp ugt i64 %_5.i127.i, %_56.1.i, !dbg !15689
  br i1 %_9.i128.i, label %bb2.i135.i, label %bb3.i129.i, !dbg !15689, !prof !639

bb3.i129.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit123.i
  %_12.i130.i = sub nuw i64 %_56.1.i, %_5.i127.i, !dbg !15692
  %_8.i.i131.i = icmp samesign ugt i64 %_12.i130.i, 7, !dbg !15693
  br i1 %_8.i.i131.i, label %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_.exit, label %bb2.i.i132.i, !dbg !15693, !prof !651

bb2.i.i132.i:                                     ; preds = %bb3.i129.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_12.i130.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15698, !noalias !15699
  unreachable, !dbg !15698

bb2.i135.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5store0B7_.exit123.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i127.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !15706, !noalias !15707
  unreachable, !dbg !15706

_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_.exit: ; preds = %bb3.i129.i
  %13 = getelementptr inbounds nuw i8, ptr %self, i64 352, !dbg !15708
  %_16.i134.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i127.i, !dbg !15709
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_16.i134.i, ptr noundef nonnull readonly align 32 dereferenceable(32) %13, i64 32, i1 false), !dbg !15711, !noalias !15372
  %14 = getelementptr inbounds nuw i8, ptr %state, i64 72, !dbg !15715
  %_21.1 = load i64, ptr %14, align 8, !dbg !15715, !noundef !12
  %_8.i2 = icmp samesign ugt i64 %_21.1, 7, !dbg !15716
  br i1 %_8.i2, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit4, label %bb2.i3, !dbg !15716, !prof !651

bb2.i3:                                           ; preds = %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_.exit
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_21.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15721, !noalias !15722
  unreachable, !dbg !15721

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit4: ; preds = %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_.exit
  %15 = getelementptr inbounds nuw i8, ptr %self, i64 640, !dbg !15726
  %16 = getelementptr inbounds nuw i8, ptr %state, i64 64, !dbg !15715
  %_21.0 = load ptr, ptr %16, align 8, !dbg !15715, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_21.0, ptr noundef nonnull align 32 dereferenceable(32) %15, i64 32, i1 false), !dbg !15727
  %17 = getelementptr inbounds nuw i8, ptr %state, i64 104, !dbg !15731
  %_22.1 = load i64, ptr %17, align 8, !dbg !15731, !noundef !12
  %_8.i = icmp samesign ugt i64 %_22.1, 7, !dbg !15732
  br i1 %_8.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit, label %bb2.i, !dbg !15732, !prof !651

bb2.i:                                            ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_22.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15737, !noalias !15738
  unreachable, !dbg !15737

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit4
  %18 = getelementptr inbounds nuw i8, ptr %self, i64 672, !dbg !15742
  %19 = getelementptr inbounds nuw i8, ptr %state, i64 96, !dbg !15731
  %_22.0 = load ptr, ptr %19, align 8, !dbg !15731, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_22.0, ptr noundef nonnull align 32 dereferenceable(32) %18, i64 32, i1 false), !dbg !15743
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 384, !dbg !15747
  %_12.sroa.0.0.copyload = load float, ptr %20, align 32, !dbg !15747
  %_12.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 388, !dbg !15747
  %_12.sroa.4.0.copyload = load float, ptr %_12.sroa.4.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 392, !dbg !15747
  %_12.sroa.5.0.copyload = load float, ptr %_12.sroa.5.0..sroa_idx, align 8, !dbg !15747
  %_12.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 396, !dbg !15747
  %_12.sroa.6.0.copyload = load float, ptr %_12.sroa.6.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 400, !dbg !15747
  %_12.sroa.7.0.copyload = load float, ptr %_12.sroa.7.0..sroa_idx, align 16, !dbg !15747
  %_12.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 404, !dbg !15747
  %_12.sroa.8.0.copyload = load float, ptr %_12.sroa.8.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.9.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 408, !dbg !15747
  %_12.sroa.9.0.copyload = load float, ptr %_12.sroa.9.0..sroa_idx, align 8, !dbg !15747
  %_12.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 412, !dbg !15747
  %_12.sroa.10.0.copyload = load float, ptr %_12.sroa.10.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.1176.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 448, !dbg !15747
  %_12.sroa.1176.0.copyload = load float, ptr %_12.sroa.1176.0..sroa_idx, align 32, !dbg !15747
  %_12.sroa.12.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 452, !dbg !15747
  %_12.sroa.12.0.copyload = load float, ptr %_12.sroa.12.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.13.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 456, !dbg !15747
  %_12.sroa.13.0.copyload = load float, ptr %_12.sroa.13.0..sroa_idx, align 8, !dbg !15747
  %_12.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 460, !dbg !15747
  %_12.sroa.14.0.copyload = load float, ptr %_12.sroa.14.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.15.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 464, !dbg !15747
  %_12.sroa.15.0.copyload = load float, ptr %_12.sroa.15.0..sroa_idx, align 16, !dbg !15747
  %_12.sroa.16.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 468, !dbg !15747
  %_12.sroa.16.0.copyload = load float, ptr %_12.sroa.16.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.17.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 472, !dbg !15747
  %_12.sroa.17.0.copyload = load float, ptr %_12.sroa.17.0..sroa_idx, align 8, !dbg !15747
  %_12.sroa.18.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 476, !dbg !15747
  %_12.sroa.18.0.copyload = load float, ptr %_12.sroa.18.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.19.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !15747
  %_12.sroa.19.0.copyload = load float, ptr %_12.sroa.19.0..sroa_idx, align 32, !dbg !15747
  %_12.sroa.20.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 484, !dbg !15747
  %_12.sroa.20.0.copyload = load float, ptr %_12.sroa.20.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.21.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !15747
  %_12.sroa.21.0.copyload = load float, ptr %_12.sroa.21.0..sroa_idx, align 8, !dbg !15747
  %_12.sroa.22.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 492, !dbg !15747
  %_12.sroa.22.0.copyload = load float, ptr %_12.sroa.22.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.23.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 496, !dbg !15747
  %_12.sroa.23.0.copyload = load float, ptr %_12.sroa.23.0..sroa_idx, align 16, !dbg !15747
  %_12.sroa.24.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 500, !dbg !15747
  %_12.sroa.24.0.copyload = load float, ptr %_12.sroa.24.0..sroa_idx, align 4, !dbg !15747
  %_12.sroa.25.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 504, !dbg !15747
  %_12.sroa.25.0.copyload = load float, ptr %_12.sroa.25.0..sroa_idx, align 8, !dbg !15747
  %_12.sroa.26.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 508, !dbg !15747
  %_12.sroa.26.0.copyload = load float, ptr %_12.sroa.26.0..sroa_idx, align 4, !dbg !15747
  %21 = getelementptr inbounds nuw i8, ptr %state, i64 128, !dbg !15748
  %_23.0 = load ptr, ptr %21, align 8, !dbg !15748, !nonnull !12, !noundef !12
  %22 = getelementptr inbounds nuw i8, ptr %state, i64 136, !dbg !15748
  %_23.1 = load i64, ptr %22, align 8, !dbg !15748, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15749), !dbg !15752
  %_7.i.i21.i = icmp eq i64 %_23.1, 0, !dbg !15753
  br i1 %_7.i.i21.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %bb7.preheader.i, !dbg !15763

bb7.preheader.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit
  store float %_12.sroa.0.0.copyload, ptr %_23.0, align 4, !dbg !15764, !alias.scope !15749, !noalias !15766
  %23 = getelementptr inbounds nuw i8, ptr %_23.0, i64 8, !dbg !15768
  store float %_12.sroa.1176.0.copyload, ptr %23, align 4, !dbg !15768, !alias.scope !15749, !noalias !15766
  %24 = getelementptr inbounds nuw i8, ptr %_23.0, i64 12, !dbg !15769
  %25 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.19.0.copyload), !dbg !15769
  store i32 %25, ptr %24, align 4, !dbg !15769, !alias.scope !15749, !noalias !15766
  %_7.i.i.i = icmp eq i64 %_23.1, 1, !dbg !15753
  br i1 %_7.i.i.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %bb7.1.i, !dbg !15763

bb7.1.i:                                          ; preds = %bb7.preheader.i
  %_17.i.i.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 16, !dbg !15770
  store float %_12.sroa.4.0.copyload, ptr %_17.i.i.i, align 4, !dbg !15764, !alias.scope !15749, !noalias !15766
  %26 = getelementptr inbounds nuw i8, ptr %_23.0, i64 24, !dbg !15768
  store float %_12.sroa.12.0.copyload, ptr %26, align 4, !dbg !15768, !alias.scope !15749, !noalias !15766
  %27 = getelementptr inbounds nuw i8, ptr %_23.0, i64 28, !dbg !15769
  %28 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.20.0.copyload), !dbg !15769
  store i32 %28, ptr %27, align 4, !dbg !15769, !alias.scope !15749, !noalias !15766
  %_7.i.i.1.i = icmp eq i64 %_23.1, 2, !dbg !15753
  br i1 %_7.i.i.1.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %bb7.2.i, !dbg !15763

bb7.2.i:                                          ; preds = %bb7.1.i
  %_17.i.i.1.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 32, !dbg !15770
  store float %_12.sroa.5.0.copyload, ptr %_17.i.i.1.i, align 4, !dbg !15764, !alias.scope !15749, !noalias !15766
  %29 = getelementptr inbounds nuw i8, ptr %_23.0, i64 40, !dbg !15768
  store float %_12.sroa.13.0.copyload, ptr %29, align 4, !dbg !15768, !alias.scope !15749, !noalias !15766
  %30 = getelementptr inbounds nuw i8, ptr %_23.0, i64 44, !dbg !15769
  %31 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.21.0.copyload), !dbg !15769
  store i32 %31, ptr %30, align 4, !dbg !15769, !alias.scope !15749, !noalias !15766
  %_7.i.i.2.i = icmp eq i64 %_23.1, 3, !dbg !15753
  br i1 %_7.i.i.2.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %bb7.3.i, !dbg !15763

bb7.3.i:                                          ; preds = %bb7.2.i
  %_17.i.i.2.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 48, !dbg !15770
  store float %_12.sroa.6.0.copyload, ptr %_17.i.i.2.i, align 4, !dbg !15764, !alias.scope !15749, !noalias !15766
  %32 = getelementptr inbounds nuw i8, ptr %_23.0, i64 56, !dbg !15768
  store float %_12.sroa.14.0.copyload, ptr %32, align 4, !dbg !15768, !alias.scope !15749, !noalias !15766
  %33 = getelementptr inbounds nuw i8, ptr %_23.0, i64 60, !dbg !15769
  %34 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.22.0.copyload), !dbg !15769
  store i32 %34, ptr %33, align 4, !dbg !15769, !alias.scope !15749, !noalias !15766
  %_7.i.i.3.i = icmp eq i64 %_23.1, 4, !dbg !15753
  br i1 %_7.i.i.3.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %bb7.4.i, !dbg !15763

bb7.4.i:                                          ; preds = %bb7.3.i
  %_17.i.i.3.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 64, !dbg !15770
  store float %_12.sroa.7.0.copyload, ptr %_17.i.i.3.i, align 4, !dbg !15764, !alias.scope !15749, !noalias !15766
  %35 = getelementptr inbounds nuw i8, ptr %_23.0, i64 72, !dbg !15768
  store float %_12.sroa.15.0.copyload, ptr %35, align 4, !dbg !15768, !alias.scope !15749, !noalias !15766
  %36 = getelementptr inbounds nuw i8, ptr %_23.0, i64 76, !dbg !15769
  %37 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.23.0.copyload), !dbg !15769
  store i32 %37, ptr %36, align 4, !dbg !15769, !alias.scope !15749, !noalias !15766
  %_7.i.i.4.i = icmp eq i64 %_23.1, 5, !dbg !15753
  br i1 %_7.i.i.4.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %bb7.5.i, !dbg !15763

bb7.5.i:                                          ; preds = %bb7.4.i
  %_17.i.i.4.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 80, !dbg !15770
  store float %_12.sroa.8.0.copyload, ptr %_17.i.i.4.i, align 4, !dbg !15764, !alias.scope !15749, !noalias !15766
  %38 = getelementptr inbounds nuw i8, ptr %_23.0, i64 88, !dbg !15768
  store float %_12.sroa.16.0.copyload, ptr %38, align 4, !dbg !15768, !alias.scope !15749, !noalias !15766
  %39 = getelementptr inbounds nuw i8, ptr %_23.0, i64 92, !dbg !15769
  %40 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.24.0.copyload), !dbg !15769
  store i32 %40, ptr %39, align 4, !dbg !15769, !alias.scope !15749, !noalias !15766
  %_7.i.i.5.i = icmp eq i64 %_23.1, 6, !dbg !15753
  br i1 %_7.i.i.5.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %bb7.6.i, !dbg !15763

bb7.6.i:                                          ; preds = %bb7.5.i
  %_17.i.i.5.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 96, !dbg !15770
  store float %_12.sroa.9.0.copyload, ptr %_17.i.i.5.i, align 4, !dbg !15764, !alias.scope !15749, !noalias !15766
  %41 = getelementptr inbounds nuw i8, ptr %_23.0, i64 104, !dbg !15768
  store float %_12.sroa.17.0.copyload, ptr %41, align 4, !dbg !15768, !alias.scope !15749, !noalias !15766
  %42 = getelementptr inbounds nuw i8, ptr %_23.0, i64 108, !dbg !15769
  %43 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.25.0.copyload), !dbg !15769
  store i32 %43, ptr %42, align 4, !dbg !15769, !alias.scope !15749, !noalias !15766
  %_7.i.i.6.i = icmp eq i64 %_23.1, 7, !dbg !15753
  br i1 %_7.i.i.6.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %bb7.7.i, !dbg !15763

bb7.7.i:                                          ; preds = %bb7.6.i
  %_17.i.i.6.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 112, !dbg !15770
  store float %_12.sroa.10.0.copyload, ptr %_17.i.i.6.i, align 4, !dbg !15764, !alias.scope !15749, !noalias !15766
  %44 = getelementptr inbounds nuw i8, ptr %_23.0, i64 120, !dbg !15768
  store float %_12.sroa.18.0.copyload, ptr %44, align 4, !dbg !15768, !alias.scope !15749, !noalias !15766
  %45 = getelementptr inbounds nuw i8, ptr %_23.0, i64 124, !dbg !15769
  %46 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.26.0.copyload), !dbg !15769
  store i32 %46, ptr %45, align 4, !dbg !15769, !alias.scope !15749, !noalias !15766
  %_7.i.i.7.i = icmp eq i64 %_23.1, 8, !dbg !15753
  br i1 %_7.i.i.7.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, label %panic.i, !dbg !15763

panic.i:                                          ; preds = %bb7.7.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e7cf9526cbf9c5bb2e965885a8a18fa) #30, !dbg !15772, !noalias !15773
  unreachable, !dbg !15772

_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit, %bb7.preheader.i, %bb7.1.i, %bb7.2.i, %bb7.3.i, %bb7.4.i, %bb7.5.i, %bb7.6.i, %bb7.7.i
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 512, !dbg !15774
  %_15.sroa.0.0.copyload = load float, ptr %47, align 32, !dbg !15774
  %_15.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 516, !dbg !15774
  %_15.sroa.4.0.copyload = load float, ptr %_15.sroa.4.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 520, !dbg !15774
  %_15.sroa.5.0.copyload = load float, ptr %_15.sroa.5.0..sroa_idx, align 8, !dbg !15774
  %_15.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 524, !dbg !15774
  %_15.sroa.6.0.copyload = load float, ptr %_15.sroa.6.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 528, !dbg !15774
  %_15.sroa.7.0.copyload = load float, ptr %_15.sroa.7.0..sroa_idx, align 16, !dbg !15774
  %_15.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 532, !dbg !15774
  %_15.sroa.8.0.copyload = load float, ptr %_15.sroa.8.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.9.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 536, !dbg !15774
  %_15.sroa.9.0.copyload = load float, ptr %_15.sroa.9.0..sroa_idx, align 8, !dbg !15774
  %_15.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 540, !dbg !15774
  %_15.sroa.10.0.copyload = load float, ptr %_15.sroa.10.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.1177.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !15774
  %_15.sroa.1177.0.copyload = load float, ptr %_15.sroa.1177.0..sroa_idx, align 32, !dbg !15774
  %_15.sroa.12.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 580, !dbg !15774
  %_15.sroa.12.0.copyload = load float, ptr %_15.sroa.12.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.13.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 584, !dbg !15774
  %_15.sroa.13.0.copyload = load float, ptr %_15.sroa.13.0..sroa_idx, align 8, !dbg !15774
  %_15.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 588, !dbg !15774
  %_15.sroa.14.0.copyload = load float, ptr %_15.sroa.14.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.15.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 592, !dbg !15774
  %_15.sroa.15.0.copyload = load float, ptr %_15.sroa.15.0..sroa_idx, align 16, !dbg !15774
  %_15.sroa.16.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 596, !dbg !15774
  %_15.sroa.16.0.copyload = load float, ptr %_15.sroa.16.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.17.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 600, !dbg !15774
  %_15.sroa.17.0.copyload = load float, ptr %_15.sroa.17.0..sroa_idx, align 8, !dbg !15774
  %_15.sroa.18.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 604, !dbg !15774
  %_15.sroa.18.0.copyload = load float, ptr %_15.sroa.18.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.19.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 608, !dbg !15774
  %_15.sroa.19.0.copyload = load float, ptr %_15.sroa.19.0..sroa_idx, align 32, !dbg !15774
  %_15.sroa.20.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 612, !dbg !15774
  %_15.sroa.20.0.copyload = load float, ptr %_15.sroa.20.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.21.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 616, !dbg !15774
  %_15.sroa.21.0.copyload = load float, ptr %_15.sroa.21.0..sroa_idx, align 8, !dbg !15774
  %_15.sroa.22.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 620, !dbg !15774
  %_15.sroa.22.0.copyload = load float, ptr %_15.sroa.22.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.23.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 624, !dbg !15774
  %_15.sroa.23.0.copyload = load float, ptr %_15.sroa.23.0..sroa_idx, align 16, !dbg !15774
  %_15.sroa.24.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 628, !dbg !15774
  %_15.sroa.24.0.copyload = load float, ptr %_15.sroa.24.0..sroa_idx, align 4, !dbg !15774
  %_15.sroa.25.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 632, !dbg !15774
  %_15.sroa.25.0.copyload = load float, ptr %_15.sroa.25.0..sroa_idx, align 8, !dbg !15774
  %_15.sroa.26.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 636, !dbg !15774
  %_15.sroa.26.0.copyload = load float, ptr %_15.sroa.26.0..sroa_idx, align 4, !dbg !15774
  %48 = getelementptr inbounds nuw i8, ptr %state, i64 144, !dbg !15775
  %_24.0 = load ptr, ptr %48, align 8, !dbg !15775, !nonnull !12, !noundef !12
  %49 = getelementptr inbounds nuw i8, ptr %state, i64 152, !dbg !15775
  %_24.1 = load i64, ptr %49, align 8, !dbg !15775, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15776), !dbg !15779
  %_7.i.i21.i47 = icmp eq i64 %_24.1, 0, !dbg !15780
  br i1 %_7.i.i21.i47, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %bb7.preheader.i48, !dbg !15785

bb7.preheader.i48:                                ; preds = %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit
  store float %_15.sroa.0.0.copyload, ptr %_24.0, align 4, !dbg !15786, !alias.scope !15776, !noalias !15787
  %50 = getelementptr inbounds nuw i8, ptr %_24.0, i64 8, !dbg !15789
  store float %_15.sroa.1177.0.copyload, ptr %50, align 4, !dbg !15789, !alias.scope !15776, !noalias !15787
  %51 = getelementptr inbounds nuw i8, ptr %_24.0, i64 12, !dbg !15790
  %52 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.19.0.copyload), !dbg !15790
  store i32 %52, ptr %51, align 4, !dbg !15790, !alias.scope !15776, !noalias !15787
  %_7.i.i.i52 = icmp eq i64 %_24.1, 1, !dbg !15780
  br i1 %_7.i.i.i52, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %bb7.1.i53, !dbg !15785

bb7.1.i53:                                        ; preds = %bb7.preheader.i48
  %_17.i.i.i54 = getelementptr inbounds nuw i8, ptr %_24.0, i64 16, !dbg !15791
  store float %_15.sroa.4.0.copyload, ptr %_17.i.i.i54, align 4, !dbg !15786, !alias.scope !15776, !noalias !15787
  %53 = getelementptr inbounds nuw i8, ptr %_24.0, i64 24, !dbg !15789
  store float %_15.sroa.12.0.copyload, ptr %53, align 4, !dbg !15789, !alias.scope !15776, !noalias !15787
  %54 = getelementptr inbounds nuw i8, ptr %_24.0, i64 28, !dbg !15790
  %55 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.20.0.copyload), !dbg !15790
  store i32 %55, ptr %54, align 4, !dbg !15790, !alias.scope !15776, !noalias !15787
  %_7.i.i.1.i55 = icmp eq i64 %_24.1, 2, !dbg !15780
  br i1 %_7.i.i.1.i55, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %bb7.2.i56, !dbg !15785

bb7.2.i56:                                        ; preds = %bb7.1.i53
  %_17.i.i.1.i57 = getelementptr inbounds nuw i8, ptr %_24.0, i64 32, !dbg !15791
  store float %_15.sroa.5.0.copyload, ptr %_17.i.i.1.i57, align 4, !dbg !15786, !alias.scope !15776, !noalias !15787
  %56 = getelementptr inbounds nuw i8, ptr %_24.0, i64 40, !dbg !15789
  store float %_15.sroa.13.0.copyload, ptr %56, align 4, !dbg !15789, !alias.scope !15776, !noalias !15787
  %57 = getelementptr inbounds nuw i8, ptr %_24.0, i64 44, !dbg !15790
  %58 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.21.0.copyload), !dbg !15790
  store i32 %58, ptr %57, align 4, !dbg !15790, !alias.scope !15776, !noalias !15787
  %_7.i.i.2.i58 = icmp eq i64 %_24.1, 3, !dbg !15780
  br i1 %_7.i.i.2.i58, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %bb7.3.i59, !dbg !15785

bb7.3.i59:                                        ; preds = %bb7.2.i56
  %_17.i.i.2.i60 = getelementptr inbounds nuw i8, ptr %_24.0, i64 48, !dbg !15791
  store float %_15.sroa.6.0.copyload, ptr %_17.i.i.2.i60, align 4, !dbg !15786, !alias.scope !15776, !noalias !15787
  %59 = getelementptr inbounds nuw i8, ptr %_24.0, i64 56, !dbg !15789
  store float %_15.sroa.14.0.copyload, ptr %59, align 4, !dbg !15789, !alias.scope !15776, !noalias !15787
  %60 = getelementptr inbounds nuw i8, ptr %_24.0, i64 60, !dbg !15790
  %61 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.22.0.copyload), !dbg !15790
  store i32 %61, ptr %60, align 4, !dbg !15790, !alias.scope !15776, !noalias !15787
  %_7.i.i.3.i61 = icmp eq i64 %_24.1, 4, !dbg !15780
  br i1 %_7.i.i.3.i61, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %bb7.4.i62, !dbg !15785

bb7.4.i62:                                        ; preds = %bb7.3.i59
  %_17.i.i.3.i63 = getelementptr inbounds nuw i8, ptr %_24.0, i64 64, !dbg !15791
  store float %_15.sroa.7.0.copyload, ptr %_17.i.i.3.i63, align 4, !dbg !15786, !alias.scope !15776, !noalias !15787
  %62 = getelementptr inbounds nuw i8, ptr %_24.0, i64 72, !dbg !15789
  store float %_15.sroa.15.0.copyload, ptr %62, align 4, !dbg !15789, !alias.scope !15776, !noalias !15787
  %63 = getelementptr inbounds nuw i8, ptr %_24.0, i64 76, !dbg !15790
  %64 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.23.0.copyload), !dbg !15790
  store i32 %64, ptr %63, align 4, !dbg !15790, !alias.scope !15776, !noalias !15787
  %_7.i.i.4.i64 = icmp eq i64 %_24.1, 5, !dbg !15780
  br i1 %_7.i.i.4.i64, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %bb7.5.i65, !dbg !15785

bb7.5.i65:                                        ; preds = %bb7.4.i62
  %_17.i.i.4.i66 = getelementptr inbounds nuw i8, ptr %_24.0, i64 80, !dbg !15791
  store float %_15.sroa.8.0.copyload, ptr %_17.i.i.4.i66, align 4, !dbg !15786, !alias.scope !15776, !noalias !15787
  %65 = getelementptr inbounds nuw i8, ptr %_24.0, i64 88, !dbg !15789
  store float %_15.sroa.16.0.copyload, ptr %65, align 4, !dbg !15789, !alias.scope !15776, !noalias !15787
  %66 = getelementptr inbounds nuw i8, ptr %_24.0, i64 92, !dbg !15790
  %67 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.24.0.copyload), !dbg !15790
  store i32 %67, ptr %66, align 4, !dbg !15790, !alias.scope !15776, !noalias !15787
  %_7.i.i.5.i67 = icmp eq i64 %_24.1, 6, !dbg !15780
  br i1 %_7.i.i.5.i67, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %bb7.6.i68, !dbg !15785

bb7.6.i68:                                        ; preds = %bb7.5.i65
  %_17.i.i.5.i69 = getelementptr inbounds nuw i8, ptr %_24.0, i64 96, !dbg !15791
  store float %_15.sroa.9.0.copyload, ptr %_17.i.i.5.i69, align 4, !dbg !15786, !alias.scope !15776, !noalias !15787
  %68 = getelementptr inbounds nuw i8, ptr %_24.0, i64 104, !dbg !15789
  store float %_15.sroa.17.0.copyload, ptr %68, align 4, !dbg !15789, !alias.scope !15776, !noalias !15787
  %69 = getelementptr inbounds nuw i8, ptr %_24.0, i64 108, !dbg !15790
  %70 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.25.0.copyload), !dbg !15790
  store i32 %70, ptr %69, align 4, !dbg !15790, !alias.scope !15776, !noalias !15787
  %_7.i.i.6.i70 = icmp eq i64 %_24.1, 7, !dbg !15780
  br i1 %_7.i.i.6.i70, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %bb7.7.i71, !dbg !15785

bb7.7.i71:                                        ; preds = %bb7.6.i68
  %_17.i.i.6.i72 = getelementptr inbounds nuw i8, ptr %_24.0, i64 112, !dbg !15791
  store float %_15.sroa.10.0.copyload, ptr %_17.i.i.6.i72, align 4, !dbg !15786, !alias.scope !15776, !noalias !15787
  %71 = getelementptr inbounds nuw i8, ptr %_24.0, i64 120, !dbg !15789
  store float %_15.sroa.18.0.copyload, ptr %71, align 4, !dbg !15789, !alias.scope !15776, !noalias !15787
  %72 = getelementptr inbounds nuw i8, ptr %_24.0, i64 124, !dbg !15790
  %73 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.26.0.copyload), !dbg !15790
  store i32 %73, ptr %72, align 4, !dbg !15790, !alias.scope !15776, !noalias !15787
  %_7.i.i.7.i73 = icmp eq i64 %_24.1, 8, !dbg !15780
  br i1 %_7.i.i.7.i73, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75, label %panic.i74, !dbg !15785

panic.i74:                                        ; preds = %bb7.7.i71
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e7cf9526cbf9c5bb2e965885a8a18fa) #30, !dbg !15793, !noalias !15794
  unreachable, !dbg !15793

_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit75: ; preds = %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E7scatterB5_.exit, %bb7.preheader.i48, %bb7.1.i53, %bb7.2.i56, %bb7.3.i59, %bb7.4.i62, %bb7.5.i65, %bb7.6.i68, %bb7.7.i71
  ret void, !dbg !15795
}
