define internal fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 32 captures(none) dereferenceable(736) %_0, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(200) %state) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !14985 {
start:
  %_16 = alloca [32 x i8], align 32
  %_14 = alloca [32 x i8], align 32
  %history = alloca [384 x i8], align 32
  call void @llvm.lifetime.start.p0(ptr nonnull %history), !dbg !14986
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14987), !dbg !14990
  %0 = getelementptr inbounds nuw i8, ptr %state, i64 192, !dbg !14991
  %1 = load i64, ptr %0, align 8, !dbg !14991, !alias.scope !14987, !noalias !14994, !noundef !12
  %_31.0.i = load ptr, ptr %state, align 8, !dbg !14996, !alias.scope !14987, !noalias !14994, !nonnull !12, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %state, i64 8, !dbg !14996
  %_31.1.i = load i64, ptr %2, align 8, !dbg !14996, !alias.scope !14987, !noalias !14994, !noundef !12
  %_8.i.i.i = icmp samesign ugt i64 %_31.1.i, 7, !dbg !14998
  br i1 %_8.i.i.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit.i, label %bb2.i.i.i, !dbg !14998, !prof !651

bb2.i.i.i:                                        ; preds = %start
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15006, !noalias !15007
  unreachable, !dbg !15006

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit.i: ; preds = %start
  %_8.i.i = icmp ugt i64 %1, %_31.1.i, !dbg !15014
  br i1 %_8.i.i, label %bb2.i.i, label %bb3.i.i, !dbg !15014, !prof !639

bb3.i.i:                                          ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit.i
  %_11.i.i = sub nuw i64 %_31.1.i, %1, !dbg !15020
  %_8.i.i4.i = icmp samesign ugt i64 %_11.i.i, 7, !dbg !15021
  br i1 %_8.i.i4.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit7.i, label %bb2.i.i5.i, !dbg !15021, !prof !651

bb2.i.i5.i:                                       ; preds = %bb3.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15026, !noalias !15027
  unreachable, !dbg !15026

bb2.i.i:                                          ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %1, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15034, !noalias !15035
  unreachable, !dbg !15034

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit7.i: ; preds = %bb3.i.i
  %_4.i.i = shl i64 %1, 1, !dbg !15036
  %_8.i11.i = icmp ugt i64 %_4.i.i, %_31.1.i, !dbg !15038
  br i1 %_8.i11.i, label %bb2.i18.i, label %bb3.i12.i, !dbg !15038, !prof !639

bb3.i12.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit7.i
  %_11.i13.i = sub nuw i64 %_31.1.i, %_4.i.i, !dbg !15041
  %_8.i.i14.i = icmp samesign ugt i64 %_11.i13.i, 7, !dbg !15042
  br i1 %_8.i.i14.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit19.i, label %bb2.i.i15.i, !dbg !15042, !prof !651

bb2.i.i15.i:                                      ; preds = %bb3.i12.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i13.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15047, !noalias !15048
  unreachable, !dbg !15047

bb2.i18.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit7.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15055, !noalias !15056
  unreachable, !dbg !15055

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit19.i: ; preds = %bb3.i12.i
  %_4.i23.i = mul i64 %1, 3, !dbg !15057
  %_8.i24.i = icmp ugt i64 %_4.i23.i, %_31.1.i, !dbg !15059
  br i1 %_8.i24.i, label %bb2.i31.i, label %bb3.i25.i, !dbg !15059, !prof !639

bb3.i25.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit19.i
  %_11.i26.i = sub nuw i64 %_31.1.i, %_4.i23.i, !dbg !15062
  %_8.i.i27.i = icmp samesign ugt i64 %_11.i26.i, 7, !dbg !15063
  br i1 %_8.i.i27.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit32.i, label %bb2.i.i28.i, !dbg !15063, !prof !651

bb2.i.i28.i:                                      ; preds = %bb3.i25.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i26.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15068, !noalias !15069
  unreachable, !dbg !15068

bb2.i31.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit19.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i23.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15076, !noalias !15077
  unreachable, !dbg !15076

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit32.i: ; preds = %bb3.i25.i
  %_4.i36.i = shl i64 %1, 2, !dbg !15078
  %_8.i37.i = icmp ugt i64 %_4.i36.i, %_31.1.i, !dbg !15080
  br i1 %_8.i37.i, label %bb2.i44.i, label %bb3.i38.i, !dbg !15080, !prof !639

bb3.i38.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit32.i
  %_11.i39.i = sub nuw i64 %_31.1.i, %_4.i36.i, !dbg !15083
  %_8.i.i40.i = icmp samesign ugt i64 %_11.i39.i, 7, !dbg !15084
  br i1 %_8.i.i40.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit45.i, label %bb2.i.i41.i, !dbg !15084, !prof !651

bb2.i.i41.i:                                      ; preds = %bb3.i38.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i39.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15089, !noalias !15090
  unreachable, !dbg !15089

bb2.i44.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit32.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i36.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15097, !noalias !15098
  unreachable, !dbg !15097

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit45.i: ; preds = %bb3.i38.i
  %_4.i49.i = mul i64 %1, 5, !dbg !15099
  %_8.i50.i = icmp ugt i64 %_4.i49.i, %_31.1.i, !dbg !15101
  br i1 %_8.i50.i, label %bb2.i57.i, label %bb3.i51.i, !dbg !15101, !prof !639

bb3.i51.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit45.i
  %_11.i52.i = sub nuw i64 %_31.1.i, %_4.i49.i, !dbg !15104
  %_8.i.i53.i = icmp samesign ugt i64 %_11.i52.i, 7, !dbg !15105
  br i1 %_8.i.i53.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit58.i, label %bb2.i.i54.i, !dbg !15105, !prof !651

bb2.i.i54.i:                                      ; preds = %bb3.i51.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i52.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15110, !noalias !15111
  unreachable, !dbg !15110

bb2.i57.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit45.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i49.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15118, !noalias !15119
  unreachable, !dbg !15118

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit58.i: ; preds = %bb3.i51.i
  %_4.i62.i = mul i64 %1, 6, !dbg !15120
  %_8.i63.i = icmp ugt i64 %_4.i62.i, %_31.1.i, !dbg !15122
  br i1 %_8.i63.i, label %bb2.i70.i, label %bb3.i64.i, !dbg !15122, !prof !639

bb3.i64.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit58.i
  %_11.i65.i = sub nuw i64 %_31.1.i, %_4.i62.i, !dbg !15125
  %_8.i.i66.i = icmp samesign ugt i64 %_11.i65.i, 7, !dbg !15126
  br i1 %_8.i.i66.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit71.i, label %bb2.i.i67.i, !dbg !15126, !prof !651

bb2.i.i67.i:                                      ; preds = %bb3.i64.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i65.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15131, !noalias !15132
  unreachable, !dbg !15131

bb2.i70.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit58.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i62.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15139, !noalias !15140
  unreachable, !dbg !15139

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit71.i: ; preds = %bb3.i64.i
  %_4.i75.i = mul i64 %1, 7, !dbg !15141
  %_8.i76.i = icmp ugt i64 %_4.i75.i, %_31.1.i, !dbg !15143
  br i1 %_8.i76.i, label %bb2.i83.i, label %bb3.i77.i, !dbg !15143, !prof !639

bb3.i77.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit71.i
  %_11.i78.i = sub nuw i64 %_31.1.i, %_4.i75.i, !dbg !15146
  %_8.i.i79.i = icmp samesign ugt i64 %_11.i78.i, 7, !dbg !15147
  br i1 %_8.i.i79.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit84.i, label %bb2.i.i80.i, !dbg !15147, !prof !651

bb2.i.i80.i:                                      ; preds = %bb3.i77.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i78.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15152, !noalias !15153
  unreachable, !dbg !15152

bb2.i83.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit71.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i75.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15160, !noalias !15161
  unreachable, !dbg !15160

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit84.i: ; preds = %bb3.i77.i
  %_4.i88.i = shl i64 %1, 3, !dbg !15162
  %_8.i89.i = icmp ugt i64 %_4.i88.i, %_31.1.i, !dbg !15164
  br i1 %_8.i89.i, label %bb2.i96.i, label %bb3.i90.i, !dbg !15164, !prof !639

bb3.i90.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit84.i
  %_11.i91.i = sub nuw i64 %_31.1.i, %_4.i88.i, !dbg !15167
  %_8.i.i92.i = icmp samesign ugt i64 %_11.i91.i, 7, !dbg !15168
  br i1 %_8.i.i92.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit97.i, label %bb2.i.i93.i, !dbg !15168, !prof !651

bb2.i.i93.i:                                      ; preds = %bb3.i90.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i91.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15173, !noalias !15174
  unreachable, !dbg !15173

bb2.i96.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit84.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i88.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15181, !noalias !15182
  unreachable, !dbg !15181

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit97.i: ; preds = %bb3.i90.i
  %_4.i101.i = mul i64 %1, 9, !dbg !15183
  %_8.i102.i = icmp ugt i64 %_4.i101.i, %_31.1.i, !dbg !15185
  br i1 %_8.i102.i, label %bb2.i109.i, label %bb3.i103.i, !dbg !15185, !prof !639

bb3.i103.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit97.i
  %_11.i104.i = sub nuw i64 %_31.1.i, %_4.i101.i, !dbg !15188
  %_8.i.i105.i = icmp samesign ugt i64 %_11.i104.i, 7, !dbg !15189
  br i1 %_8.i.i105.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit110.i, label %bb2.i.i106.i, !dbg !15189, !prof !651

bb2.i.i106.i:                                     ; preds = %bb3.i103.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i104.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15194, !noalias !15195
  unreachable, !dbg !15194

bb2.i109.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit97.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i101.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15202, !noalias !15203
  unreachable, !dbg !15202

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit110.i: ; preds = %bb3.i103.i
  %_4.i114.i = mul i64 %1, 10, !dbg !15204
  %_8.i115.i = icmp ugt i64 %_4.i114.i, %_31.1.i, !dbg !15206
  br i1 %_8.i115.i, label %bb2.i122.i, label %bb3.i116.i, !dbg !15206, !prof !639

bb3.i116.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit110.i
  %_11.i117.i = sub nuw i64 %_31.1.i, %_4.i114.i, !dbg !15209
  %_8.i.i118.i = icmp samesign ugt i64 %_11.i117.i, 7, !dbg !15210
  br i1 %_8.i.i118.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit123.i, label %bb2.i.i119.i, !dbg !15210, !prof !651

bb2.i.i119.i:                                     ; preds = %bb3.i116.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i117.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15215, !noalias !15216
  unreachable, !dbg !15215

bb2.i122.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit110.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i114.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15223, !noalias !15224
  unreachable, !dbg !15223

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit123.i: ; preds = %bb3.i116.i
  %_4.i127.i = mul i64 %1, 11, !dbg !15225
  %_8.i128.i = icmp ugt i64 %_4.i127.i, %_31.1.i, !dbg !15227
  br i1 %_8.i128.i, label %bb2.i135.i, label %bb3.i129.i, !dbg !15227, !prof !639

bb3.i129.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit123.i
  %_11.i130.i = sub nuw i64 %_31.1.i, %_4.i127.i, !dbg !15230
  %_8.i.i131.i = icmp samesign ugt i64 %_11.i130.i, 7, !dbg !15231
  br i1 %_8.i.i131.i, label %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit, label %bb2.i.i132.i, !dbg !15231, !prof !651

bb2.i.i132.i:                                     ; preds = %bb3.i129.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_11.i130.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15236, !noalias !15237
  unreachable, !dbg !15236

bb2.i135.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4load0B7_.exit123.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i127.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15244, !noalias !15245
  unreachable, !dbg !15244

_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit: ; preds = %bb3.i129.i
  %_15.i121.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i114.i, !dbg !15246
  %_15.i108.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i101.i, !dbg !15251
  %_15.i95.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i88.i, !dbg !15253
  %_15.i82.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i75.i, !dbg !15255
  %_15.i69.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i62.i, !dbg !15257
  %_15.i56.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i49.i, !dbg !15259
  %_15.i43.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i36.i, !dbg !15261
  %_15.i30.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i23.i, !dbg !15263
  %_15.i17.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i.i, !dbg !15265
  %_15.i.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %1, !dbg !15267
  %_15.i134.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i127.i, !dbg !15269
  %3 = getelementptr inbounds nuw i8, ptr %history, i64 352, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %3, ptr noundef nonnull align 4 dereferenceable(32) %_15.i134.i, i64 32, i1 false), !dbg !15272, !noalias !14987
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(384) %history, ptr noundef nonnull align 4 dereferenceable(32) %_31.0.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %4 = getelementptr inbounds nuw i8, ptr %history, i64 32, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %4, ptr noundef nonnull align 4 dereferenceable(32) %_15.i.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %5 = getelementptr inbounds nuw i8, ptr %history, i64 64, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %5, ptr noundef nonnull align 4 dereferenceable(32) %_15.i17.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %6 = getelementptr inbounds nuw i8, ptr %history, i64 96, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %6, ptr noundef nonnull align 4 dereferenceable(32) %_15.i30.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %7 = getelementptr inbounds nuw i8, ptr %history, i64 128, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %7, ptr noundef nonnull align 4 dereferenceable(32) %_15.i43.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %8 = getelementptr inbounds nuw i8, ptr %history, i64 160, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %8, ptr noundef nonnull align 4 dereferenceable(32) %_15.i56.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %9 = getelementptr inbounds nuw i8, ptr %history, i64 192, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %9, ptr noundef nonnull align 4 dereferenceable(32) %_15.i69.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %10 = getelementptr inbounds nuw i8, ptr %history, i64 224, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %10, ptr noundef nonnull align 4 dereferenceable(32) %_15.i82.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %11 = getelementptr inbounds nuw i8, ptr %history, i64 256, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %11, ptr noundef nonnull align 4 dereferenceable(32) %_15.i95.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %12 = getelementptr inbounds nuw i8, ptr %history, i64 288, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %12, ptr noundef nonnull align 4 dereferenceable(32) %_15.i108.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %13 = getelementptr inbounds nuw i8, ptr %history, i64 320, !dbg !15271
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %13, ptr noundef nonnull align 4 dereferenceable(32) %_15.i121.i, i64 32, i1 false), !dbg !15271, !noalias !14987
  %14 = getelementptr inbounds nuw i8, ptr %state, i64 176, !dbg !15273
  %_39.0 = load ptr, ptr %14, align 8, !dbg !15273, !nonnull !12, !noundef !12
  %15 = getelementptr inbounds nuw i8, ptr %state, i64 184, !dbg !15273
  %_39.1 = load i64, ptr %15, align 8, !dbg !15273, !noundef !12
  %_6.i.i156 = icmp eq i64 %_39.1, 0, !dbg !15276
  br i1 %_6.i.i156, label %bb6, label %bb5.preheader, !dbg !15281

bb5.preheader:                                    ; preds = %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit
  %_12 = load i32, ptr %_39.0, align 4, !dbg !15282, !noundef !12
  %16 = uitofp i32 %_12 to float, !dbg !15284
  %_6.i.i = icmp eq i64 %_39.1, 1, !dbg !15276
  br i1 %_6.i.i, label %bb6, label %bb5.1, !dbg !15281

bb6:                                              ; preds = %bb5.preheader, %bb5.1, %bb5.2, %bb5.3, %bb5.4, %bb5.5, %bb5.6, %bb5.7, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit
  %window.sroa.0.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit ], [ %16, %bb5.7 ], [ %16, %bb5.6 ], [ %16, %bb5.5 ], [ %16, %bb5.4 ], [ %16, %bb5.3 ], [ %16, %bb5.2 ], [ %16, %bb5.1 ], [ %16, %bb5.preheader ]
  %window.sroa.5.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit ], [ %78, %bb5.7 ], [ %78, %bb5.6 ], [ %78, %bb5.5 ], [ %78, %bb5.4 ], [ %78, %bb5.3 ], [ %78, %bb5.2 ], [ %78, %bb5.1 ], [ 0.000000e+00, %bb5.preheader ]
  %window.sroa.6.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit ], [ %79, %bb5.7 ], [ %79, %bb5.6 ], [ %79, %bb5.5 ], [ %79, %bb5.4 ], [ %79, %bb5.3 ], [ %79, %bb5.2 ], [ 0.000000e+00, %bb5.1 ], [ 0.000000e+00, %bb5.preheader ]
  %window.sroa.7.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit ], [ %80, %bb5.7 ], [ %80, %bb5.6 ], [ %80, %bb5.5 ], [ %80, %bb5.4 ], [ %80, %bb5.3 ], [ 0.000000e+00, %bb5.2 ], [ 0.000000e+00, %bb5.1 ], [ 0.000000e+00, %bb5.preheader ]
  %window.sroa.8.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit ], [ %81, %bb5.7 ], [ %81, %bb5.6 ], [ %81, %bb5.5 ], [ %81, %bb5.4 ], [ 0.000000e+00, %bb5.3 ], [ 0.000000e+00, %bb5.2 ], [ 0.000000e+00, %bb5.1 ], [ 0.000000e+00, %bb5.preheader ]
  %window.sroa.9.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit ], [ %82, %bb5.7 ], [ %82, %bb5.6 ], [ %82, %bb5.5 ], [ 0.000000e+00, %bb5.4 ], [ 0.000000e+00, %bb5.3 ], [ 0.000000e+00, %bb5.2 ], [ 0.000000e+00, %bb5.1 ], [ 0.000000e+00, %bb5.preheader ]
  %window.sroa.10.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit ], [ %83, %bb5.7 ], [ %83, %bb5.6 ], [ 0.000000e+00, %bb5.5 ], [ 0.000000e+00, %bb5.4 ], [ 0.000000e+00, %bb5.3 ], [ 0.000000e+00, %bb5.2 ], [ 0.000000e+00, %bb5.1 ], [ 0.000000e+00, %bb5.preheader ]
  %window.sroa.11.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_.exit ], [ %84, %bb5.7 ], [ 0.000000e+00, %bb5.6 ], [ 0.000000e+00, %bb5.5 ], [ 0.000000e+00, %bb5.4 ], [ 0.000000e+00, %bb5.3 ], [ 0.000000e+00, %bb5.2 ], [ 0.000000e+00, %bb5.1 ], [ 0.000000e+00, %bb5.preheader ]
  call void @llvm.lifetime.start.p0(ptr nonnull %_14), !dbg !15285
  %17 = getelementptr inbounds nuw i8, ptr %state, i64 72, !dbg !15286
  %_40.1 = load i64, ptr %17, align 8, !dbg !15286, !noundef !12
  %_8.i12 = icmp samesign ugt i64 %_40.1, 7, !dbg !15287
  br i1 %_8.i12, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit15, label %bb2.i13, !dbg !15287, !prof !651

bb2.i13:                                          ; preds = %bb6
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_40.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15292, !noalias !15293
  unreachable, !dbg !15292

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit15: ; preds = %bb6
  %18 = getelementptr inbounds nuw i8, ptr %state, i64 64, !dbg !15286
  %_40.0 = load ptr, ptr %18, align 8, !dbg !15286, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_14, ptr noundef nonnull align 4 dereferenceable(32) %_40.0, i64 32, i1 false), !dbg !15297
  call void @llvm.lifetime.start.p0(ptr nonnull %_16), !dbg !15298
  %19 = getelementptr inbounds nuw i8, ptr %state, i64 104, !dbg !15299
  %_41.1 = load i64, ptr %19, align 8, !dbg !15299, !noundef !12
  %_8.i = icmp samesign ugt i64 %_41.1, 7, !dbg !15300
  br i1 %_8.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7, label %bb2.i, !dbg !15300, !prof !651

bb2.i:                                            ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit15
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_41.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !15305, !noalias !15306
  unreachable, !dbg !15305

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit15
  %20 = getelementptr inbounds nuw i8, ptr %state, i64 96, !dbg !15299
  %_41.0 = load ptr, ptr %20, align 8, !dbg !15299, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %_16, ptr noundef nonnull align 4 dereferenceable(32) %_41.0, i64 32, i1 false), !dbg !15310
  %21 = getelementptr inbounds nuw i8, ptr %state, i64 128, !dbg !15311
  %_42.0 = load ptr, ptr %21, align 8, !dbg !15311, !nonnull !12, !noundef !12
  %22 = getelementptr inbounds nuw i8, ptr %state, i64 136, !dbg !15311
  %_42.1 = load i64, ptr %22, align 8, !dbg !15311, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15312), !dbg !15315
  %_6.i.i39.i = icmp eq i64 %_42.1, 0, !dbg !15316
  br i1 %_6.i.i39.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %bb4.preheader.i, !dbg !15327

bb4.preheader.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7
  %_14.i = load float, ptr %_42.0, align 4, !dbg !15328, !alias.scope !15312, !noalias !15330, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %_42.0, i64 4, !dbg !15332
  %_16.i = load float, ptr %23, align 4, !dbg !15332, !alias.scope !15312, !noalias !15330, !noundef !12
  %24 = getelementptr inbounds nuw i8, ptr %_42.0, i64 8, !dbg !15333
  %_17.i = load float, ptr %24, align 4, !dbg !15333, !alias.scope !15312, !noalias !15330, !noundef !12
  %25 = getelementptr inbounds nuw i8, ptr %_42.0, i64 12, !dbg !15334
  %_20.i = load i32, ptr %25, align 4, !dbg !15334, !alias.scope !15312, !noalias !15330, !noundef !12
  %_19.i = trunc i32 %_20.i to i16, !dbg !15334
  %_18.i = uitofp i16 %_19.i to float, !dbg !15335
  %_6.i.i.i = icmp eq i64 %_42.1, 1, !dbg !15316
  br i1 %_6.i.i.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %bb4.1.i, !dbg !15327

bb4.1.i:                                          ; preds = %bb4.preheader.i
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %_42.0, i64 16, !dbg !15338
  %_14.1.i = load float, ptr %_16.i.i.i, align 4, !dbg !15328, !alias.scope !15312, !noalias !15330, !noundef !12
  %26 = getelementptr inbounds nuw i8, ptr %_42.0, i64 20, !dbg !15332
  %_16.1.i = load float, ptr %26, align 4, !dbg !15332, !alias.scope !15312, !noalias !15330, !noundef !12
  %27 = getelementptr inbounds nuw i8, ptr %_42.0, i64 24, !dbg !15333
  %_17.1.i = load float, ptr %27, align 4, !dbg !15333, !alias.scope !15312, !noalias !15330, !noundef !12
  %28 = getelementptr inbounds nuw i8, ptr %_42.0, i64 28, !dbg !15334
  %_20.1.i = load i32, ptr %28, align 4, !dbg !15334, !alias.scope !15312, !noalias !15330, !noundef !12
  %_19.1.i = trunc i32 %_20.1.i to i16, !dbg !15334
  %_18.1.i = uitofp i16 %_19.1.i to float, !dbg !15335
  %_6.i.i.1.i = icmp eq i64 %_42.1, 2, !dbg !15316
  br i1 %_6.i.i.1.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %bb4.2.i, !dbg !15327

bb4.2.i:                                          ; preds = %bb4.1.i
  %_16.i.i.1.i = getelementptr inbounds nuw i8, ptr %_42.0, i64 32, !dbg !15338
  %_14.2.i = load float, ptr %_16.i.i.1.i, align 4, !dbg !15328, !alias.scope !15312, !noalias !15330, !noundef !12
  %29 = getelementptr inbounds nuw i8, ptr %_42.0, i64 36, !dbg !15332
  %_16.2.i = load float, ptr %29, align 4, !dbg !15332, !alias.scope !15312, !noalias !15330, !noundef !12
  %30 = getelementptr inbounds nuw i8, ptr %_42.0, i64 40, !dbg !15333
  %_17.2.i = load float, ptr %30, align 4, !dbg !15333, !alias.scope !15312, !noalias !15330, !noundef !12
  %31 = getelementptr inbounds nuw i8, ptr %_42.0, i64 44, !dbg !15334
  %_20.2.i = load i32, ptr %31, align 4, !dbg !15334, !alias.scope !15312, !noalias !15330, !noundef !12
  %_19.2.i = trunc i32 %_20.2.i to i16, !dbg !15334
  %_18.2.i = uitofp i16 %_19.2.i to float, !dbg !15335
  %_6.i.i.2.i = icmp eq i64 %_42.1, 3, !dbg !15316
  br i1 %_6.i.i.2.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %bb4.3.i, !dbg !15327

bb4.3.i:                                          ; preds = %bb4.2.i
  %_16.i.i.2.i = getelementptr inbounds nuw i8, ptr %_42.0, i64 48, !dbg !15338
  %_14.3.i = load float, ptr %_16.i.i.2.i, align 4, !dbg !15328, !alias.scope !15312, !noalias !15330, !noundef !12
  %32 = getelementptr inbounds nuw i8, ptr %_42.0, i64 52, !dbg !15332
  %_16.3.i = load float, ptr %32, align 4, !dbg !15332, !alias.scope !15312, !noalias !15330, !noundef !12
  %33 = getelementptr inbounds nuw i8, ptr %_42.0, i64 56, !dbg !15333
  %_17.3.i = load float, ptr %33, align 4, !dbg !15333, !alias.scope !15312, !noalias !15330, !noundef !12
  %34 = getelementptr inbounds nuw i8, ptr %_42.0, i64 60, !dbg !15334
  %_20.3.i = load i32, ptr %34, align 4, !dbg !15334, !alias.scope !15312, !noalias !15330, !noundef !12
  %_19.3.i = trunc i32 %_20.3.i to i16, !dbg !15334
  %_18.3.i = uitofp i16 %_19.3.i to float, !dbg !15335
  %_6.i.i.3.i = icmp eq i64 %_42.1, 4, !dbg !15316
  br i1 %_6.i.i.3.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %bb4.4.i, !dbg !15327

bb4.4.i:                                          ; preds = %bb4.3.i
  %_16.i.i.3.i = getelementptr inbounds nuw i8, ptr %_42.0, i64 64, !dbg !15338
  %_14.4.i = load float, ptr %_16.i.i.3.i, align 4, !dbg !15328, !alias.scope !15312, !noalias !15330, !noundef !12
  %35 = getelementptr inbounds nuw i8, ptr %_42.0, i64 68, !dbg !15332
  %_16.4.i = load float, ptr %35, align 4, !dbg !15332, !alias.scope !15312, !noalias !15330, !noundef !12
  %36 = getelementptr inbounds nuw i8, ptr %_42.0, i64 72, !dbg !15333
  %_17.4.i = load float, ptr %36, align 4, !dbg !15333, !alias.scope !15312, !noalias !15330, !noundef !12
  %37 = getelementptr inbounds nuw i8, ptr %_42.0, i64 76, !dbg !15334
  %_20.4.i = load i32, ptr %37, align 4, !dbg !15334, !alias.scope !15312, !noalias !15330, !noundef !12
  %_19.4.i = trunc i32 %_20.4.i to i16, !dbg !15334
  %_18.4.i = uitofp i16 %_19.4.i to float, !dbg !15335
  %_6.i.i.4.i = icmp eq i64 %_42.1, 5, !dbg !15316
  br i1 %_6.i.i.4.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %bb4.5.i, !dbg !15327

bb4.5.i:                                          ; preds = %bb4.4.i
  %_16.i.i.4.i = getelementptr inbounds nuw i8, ptr %_42.0, i64 80, !dbg !15338
  %_14.5.i = load float, ptr %_16.i.i.4.i, align 4, !dbg !15328, !alias.scope !15312, !noalias !15330, !noundef !12
  %38 = getelementptr inbounds nuw i8, ptr %_42.0, i64 84, !dbg !15332
  %_16.5.i = load float, ptr %38, align 4, !dbg !15332, !alias.scope !15312, !noalias !15330, !noundef !12
  %39 = getelementptr inbounds nuw i8, ptr %_42.0, i64 88, !dbg !15333
  %_17.5.i = load float, ptr %39, align 4, !dbg !15333, !alias.scope !15312, !noalias !15330, !noundef !12
  %40 = getelementptr inbounds nuw i8, ptr %_42.0, i64 92, !dbg !15334
  %_20.5.i = load i32, ptr %40, align 4, !dbg !15334, !alias.scope !15312, !noalias !15330, !noundef !12
  %_19.5.i = trunc i32 %_20.5.i to i16, !dbg !15334
  %_18.5.i = uitofp i16 %_19.5.i to float, !dbg !15335
  %_6.i.i.5.i = icmp eq i64 %_42.1, 6, !dbg !15316
  br i1 %_6.i.i.5.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %bb4.6.i, !dbg !15327

bb4.6.i:                                          ; preds = %bb4.5.i
  %_16.i.i.5.i = getelementptr inbounds nuw i8, ptr %_42.0, i64 96, !dbg !15338
  %_14.6.i = load float, ptr %_16.i.i.5.i, align 4, !dbg !15328, !alias.scope !15312, !noalias !15330, !noundef !12
  %41 = getelementptr inbounds nuw i8, ptr %_42.0, i64 100, !dbg !15332
  %_16.6.i = load float, ptr %41, align 4, !dbg !15332, !alias.scope !15312, !noalias !15330, !noundef !12
  %42 = getelementptr inbounds nuw i8, ptr %_42.0, i64 104, !dbg !15333
  %_17.6.i = load float, ptr %42, align 4, !dbg !15333, !alias.scope !15312, !noalias !15330, !noundef !12
  %43 = getelementptr inbounds nuw i8, ptr %_42.0, i64 108, !dbg !15334
  %_20.6.i = load i32, ptr %43, align 4, !dbg !15334, !alias.scope !15312, !noalias !15330, !noundef !12
  %_19.6.i = trunc i32 %_20.6.i to i16, !dbg !15334
  %_18.6.i = uitofp i16 %_19.6.i to float, !dbg !15335
  %_6.i.i.6.i = icmp eq i64 %_42.1, 7, !dbg !15316
  br i1 %_6.i.i.6.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %bb4.7.i, !dbg !15327

bb4.7.i:                                          ; preds = %bb4.6.i
  %_16.i.i.6.i = getelementptr inbounds nuw i8, ptr %_42.0, i64 112, !dbg !15338
  %_14.7.i = load float, ptr %_16.i.i.6.i, align 4, !dbg !15328, !alias.scope !15312, !noalias !15330, !noundef !12
  %44 = getelementptr inbounds nuw i8, ptr %_42.0, i64 116, !dbg !15332
  %_16.7.i = load float, ptr %44, align 4, !dbg !15332, !alias.scope !15312, !noalias !15330, !noundef !12
  %45 = getelementptr inbounds nuw i8, ptr %_42.0, i64 120, !dbg !15333
  %_17.7.i = load float, ptr %45, align 4, !dbg !15333, !alias.scope !15312, !noalias !15330, !noundef !12
  %46 = getelementptr inbounds nuw i8, ptr %_42.0, i64 124, !dbg !15334
  %_20.7.i = load i32, ptr %46, align 4, !dbg !15334, !alias.scope !15312, !noalias !15330, !noundef !12
  %_19.7.i = trunc i32 %_20.7.i to i16, !dbg !15334
  %_18.7.i = uitofp i16 %_19.7.i to float, !dbg !15335
  %_6.i.i.7.i = icmp eq i64 %_42.1, 8, !dbg !15316
  br i1 %_6.i.i.7.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, label %panic.i, !dbg !15327

panic.i:                                          ; preds = %bb4.7.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_34ec9deef3d1165c325897b5845d9fe1) #30, !dbg !15340, !noalias !15341
  unreachable, !dbg !15340

_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7, %bb4.preheader.i, %bb4.1.i, %bb4.2.i, %bb4.3.i, %bb4.4.i, %bb4.5.i, %bb4.6.i, %bb4.7.i
  %remaining.sroa.0.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_18.i, %bb4.7.i ], [ %_18.i, %bb4.6.i ], [ %_18.i, %bb4.5.i ], [ %_18.i, %bb4.4.i ], [ %_18.i, %bb4.3.i ], [ %_18.i, %bb4.2.i ], [ %_18.i, %bb4.1.i ], [ %_18.i, %bb4.preheader.i ]
  %remaining.sroa.5.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_18.1.i, %bb4.7.i ], [ %_18.1.i, %bb4.6.i ], [ %_18.1.i, %bb4.5.i ], [ %_18.1.i, %bb4.4.i ], [ %_18.1.i, %bb4.3.i ], [ %_18.1.i, %bb4.2.i ], [ %_18.1.i, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %remaining.sroa.6.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_18.2.i, %bb4.7.i ], [ %_18.2.i, %bb4.6.i ], [ %_18.2.i, %bb4.5.i ], [ %_18.2.i, %bb4.4.i ], [ %_18.2.i, %bb4.3.i ], [ %_18.2.i, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %remaining.sroa.7.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_18.3.i, %bb4.7.i ], [ %_18.3.i, %bb4.6.i ], [ %_18.3.i, %bb4.5.i ], [ %_18.3.i, %bb4.4.i ], [ %_18.3.i, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %remaining.sroa.8.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_18.4.i, %bb4.7.i ], [ %_18.4.i, %bb4.6.i ], [ %_18.4.i, %bb4.5.i ], [ %_18.4.i, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %remaining.sroa.9.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_18.5.i, %bb4.7.i ], [ %_18.5.i, %bb4.6.i ], [ %_18.5.i, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %remaining.sroa.10.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_18.6.i, %bb4.7.i ], [ %_18.6.i, %bb4.6.i ], [ 0.000000e+00, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %remaining.sroa.11.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_18.7.i, %bb4.7.i ], [ 0.000000e+00, %bb4.6.i ], [ 0.000000e+00, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %step.sroa.0.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_17.i, %bb4.7.i ], [ %_17.i, %bb4.6.i ], [ %_17.i, %bb4.5.i ], [ %_17.i, %bb4.4.i ], [ %_17.i, %bb4.3.i ], [ %_17.i, %bb4.2.i ], [ %_17.i, %bb4.1.i ], [ %_17.i, %bb4.preheader.i ]
  %step.sroa.5.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_17.1.i, %bb4.7.i ], [ %_17.1.i, %bb4.6.i ], [ %_17.1.i, %bb4.5.i ], [ %_17.1.i, %bb4.4.i ], [ %_17.1.i, %bb4.3.i ], [ %_17.1.i, %bb4.2.i ], [ %_17.1.i, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %step.sroa.6.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_17.2.i, %bb4.7.i ], [ %_17.2.i, %bb4.6.i ], [ %_17.2.i, %bb4.5.i ], [ %_17.2.i, %bb4.4.i ], [ %_17.2.i, %bb4.3.i ], [ %_17.2.i, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %step.sroa.7.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_17.3.i, %bb4.7.i ], [ %_17.3.i, %bb4.6.i ], [ %_17.3.i, %bb4.5.i ], [ %_17.3.i, %bb4.4.i ], [ %_17.3.i, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %step.sroa.8.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_17.4.i, %bb4.7.i ], [ %_17.4.i, %bb4.6.i ], [ %_17.4.i, %bb4.5.i ], [ %_17.4.i, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %step.sroa.9.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_17.5.i, %bb4.7.i ], [ %_17.5.i, %bb4.6.i ], [ %_17.5.i, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %step.sroa.10.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_17.6.i, %bb4.7.i ], [ %_17.6.i, %bb4.6.i ], [ 0.000000e+00, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %step.sroa.11.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_17.7.i, %bb4.7.i ], [ 0.000000e+00, %bb4.6.i ], [ 0.000000e+00, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %target.sroa.0.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_16.i, %bb4.7.i ], [ %_16.i, %bb4.6.i ], [ %_16.i, %bb4.5.i ], [ %_16.i, %bb4.4.i ], [ %_16.i, %bb4.3.i ], [ %_16.i, %bb4.2.i ], [ %_16.i, %bb4.1.i ], [ %_16.i, %bb4.preheader.i ]
  %target.sroa.5.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_16.1.i, %bb4.7.i ], [ %_16.1.i, %bb4.6.i ], [ %_16.1.i, %bb4.5.i ], [ %_16.1.i, %bb4.4.i ], [ %_16.1.i, %bb4.3.i ], [ %_16.1.i, %bb4.2.i ], [ %_16.1.i, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %target.sroa.6.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_16.2.i, %bb4.7.i ], [ %_16.2.i, %bb4.6.i ], [ %_16.2.i, %bb4.5.i ], [ %_16.2.i, %bb4.4.i ], [ %_16.2.i, %bb4.3.i ], [ %_16.2.i, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %target.sroa.7.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_16.3.i, %bb4.7.i ], [ %_16.3.i, %bb4.6.i ], [ %_16.3.i, %bb4.5.i ], [ %_16.3.i, %bb4.4.i ], [ %_16.3.i, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %target.sroa.8.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_16.4.i, %bb4.7.i ], [ %_16.4.i, %bb4.6.i ], [ %_16.4.i, %bb4.5.i ], [ %_16.4.i, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %target.sroa.9.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_16.5.i, %bb4.7.i ], [ %_16.5.i, %bb4.6.i ], [ %_16.5.i, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %target.sroa.10.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_16.6.i, %bb4.7.i ], [ %_16.6.i, %bb4.6.i ], [ 0.000000e+00, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %target.sroa.11.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_16.7.i, %bb4.7.i ], [ 0.000000e+00, %bb4.6.i ], [ 0.000000e+00, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %current.sroa.0.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_14.i, %bb4.7.i ], [ %_14.i, %bb4.6.i ], [ %_14.i, %bb4.5.i ], [ %_14.i, %bb4.4.i ], [ %_14.i, %bb4.3.i ], [ %_14.i, %bb4.2.i ], [ %_14.i, %bb4.1.i ], [ %_14.i, %bb4.preheader.i ]
  %current.sroa.5.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_14.1.i, %bb4.7.i ], [ %_14.1.i, %bb4.6.i ], [ %_14.1.i, %bb4.5.i ], [ %_14.1.i, %bb4.4.i ], [ %_14.1.i, %bb4.3.i ], [ %_14.1.i, %bb4.2.i ], [ %_14.1.i, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %current.sroa.6.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_14.2.i, %bb4.7.i ], [ %_14.2.i, %bb4.6.i ], [ %_14.2.i, %bb4.5.i ], [ %_14.2.i, %bb4.4.i ], [ %_14.2.i, %bb4.3.i ], [ %_14.2.i, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %current.sroa.7.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_14.3.i, %bb4.7.i ], [ %_14.3.i, %bb4.6.i ], [ %_14.3.i, %bb4.5.i ], [ %_14.3.i, %bb4.4.i ], [ %_14.3.i, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %current.sroa.8.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_14.4.i, %bb4.7.i ], [ %_14.4.i, %bb4.6.i ], [ %_14.4.i, %bb4.5.i ], [ %_14.4.i, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %current.sroa.9.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_14.5.i, %bb4.7.i ], [ %_14.5.i, %bb4.6.i ], [ %_14.5.i, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %current.sroa.10.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_14.6.i, %bb4.7.i ], [ %_14.6.i, %bb4.6.i ], [ 0.000000e+00, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %current.sroa.11.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit7 ], [ %_14.7.i, %bb4.7.i ], [ 0.000000e+00, %bb4.6.i ], [ 0.000000e+00, %bb4.5.i ], [ 0.000000e+00, %bb4.4.i ], [ 0.000000e+00, %bb4.3.i ], [ 0.000000e+00, %bb4.2.i ], [ 0.000000e+00, %bb4.1.i ], [ 0.000000e+00, %bb4.preheader.i ]
  %47 = getelementptr inbounds nuw i8, ptr %state, i64 144, !dbg !15342
  %_43.0 = load ptr, ptr %47, align 8, !dbg !15342, !nonnull !12, !noundef !12
  %48 = getelementptr inbounds nuw i8, ptr %state, i64 152, !dbg !15342
  %_43.1 = load i64, ptr %48, align 8, !dbg !15342, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15343), !dbg !15346
  %_6.i.i39.i16 = icmp eq i64 %_43.1, 0, !dbg !15347
  br i1 %_6.i.i39.i16, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %bb4.preheader.i17, !dbg !15352

bb4.preheader.i17:                                ; preds = %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit
  %_14.i18 = load float, ptr %_43.0, align 4, !dbg !15353, !alias.scope !15343, !noalias !15354, !noundef !12
  %49 = getelementptr inbounds nuw i8, ptr %_43.0, i64 4, !dbg !15356
  %_16.i19 = load float, ptr %49, align 4, !dbg !15356, !alias.scope !15343, !noalias !15354, !noundef !12
  %50 = getelementptr inbounds nuw i8, ptr %_43.0, i64 8, !dbg !15357
  %_17.i20 = load float, ptr %50, align 4, !dbg !15357, !alias.scope !15343, !noalias !15354, !noundef !12
  %51 = getelementptr inbounds nuw i8, ptr %_43.0, i64 12, !dbg !15358
  %_20.i21 = load i32, ptr %51, align 4, !dbg !15358, !alias.scope !15343, !noalias !15354, !noundef !12
  %_19.i22 = trunc i32 %_20.i21 to i16, !dbg !15358
  %_18.i23 = uitofp i16 %_19.i22 to float, !dbg !15359
  %_6.i.i.i24 = icmp eq i64 %_43.1, 1, !dbg !15347
  br i1 %_6.i.i.i24, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %bb4.1.i25, !dbg !15352

bb4.1.i25:                                        ; preds = %bb4.preheader.i17
  %_16.i.i.i26 = getelementptr inbounds nuw i8, ptr %_43.0, i64 16, !dbg !15361
  %_14.1.i27 = load float, ptr %_16.i.i.i26, align 4, !dbg !15353, !alias.scope !15343, !noalias !15354, !noundef !12
  %52 = getelementptr inbounds nuw i8, ptr %_43.0, i64 20, !dbg !15356
  %_16.1.i28 = load float, ptr %52, align 4, !dbg !15356, !alias.scope !15343, !noalias !15354, !noundef !12
  %53 = getelementptr inbounds nuw i8, ptr %_43.0, i64 24, !dbg !15357
  %_17.1.i29 = load float, ptr %53, align 4, !dbg !15357, !alias.scope !15343, !noalias !15354, !noundef !12
  %54 = getelementptr inbounds nuw i8, ptr %_43.0, i64 28, !dbg !15358
  %_20.1.i30 = load i32, ptr %54, align 4, !dbg !15358, !alias.scope !15343, !noalias !15354, !noundef !12
  %_19.1.i31 = trunc i32 %_20.1.i30 to i16, !dbg !15358
  %_18.1.i32 = uitofp i16 %_19.1.i31 to float, !dbg !15359
  %_6.i.i.1.i33 = icmp eq i64 %_43.1, 2, !dbg !15347
  br i1 %_6.i.i.1.i33, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %bb4.2.i34, !dbg !15352

bb4.2.i34:                                        ; preds = %bb4.1.i25
  %_16.i.i.1.i35 = getelementptr inbounds nuw i8, ptr %_43.0, i64 32, !dbg !15361
  %_14.2.i36 = load float, ptr %_16.i.i.1.i35, align 4, !dbg !15353, !alias.scope !15343, !noalias !15354, !noundef !12
  %55 = getelementptr inbounds nuw i8, ptr %_43.0, i64 36, !dbg !15356
  %_16.2.i37 = load float, ptr %55, align 4, !dbg !15356, !alias.scope !15343, !noalias !15354, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %_43.0, i64 40, !dbg !15357
  %_17.2.i38 = load float, ptr %56, align 4, !dbg !15357, !alias.scope !15343, !noalias !15354, !noundef !12
  %57 = getelementptr inbounds nuw i8, ptr %_43.0, i64 44, !dbg !15358
  %_20.2.i39 = load i32, ptr %57, align 4, !dbg !15358, !alias.scope !15343, !noalias !15354, !noundef !12
  %_19.2.i40 = trunc i32 %_20.2.i39 to i16, !dbg !15358
  %_18.2.i41 = uitofp i16 %_19.2.i40 to float, !dbg !15359
  %_6.i.i.2.i42 = icmp eq i64 %_43.1, 3, !dbg !15347
  br i1 %_6.i.i.2.i42, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %bb4.3.i43, !dbg !15352

bb4.3.i43:                                        ; preds = %bb4.2.i34
  %_16.i.i.2.i44 = getelementptr inbounds nuw i8, ptr %_43.0, i64 48, !dbg !15361
  %_14.3.i45 = load float, ptr %_16.i.i.2.i44, align 4, !dbg !15353, !alias.scope !15343, !noalias !15354, !noundef !12
  %58 = getelementptr inbounds nuw i8, ptr %_43.0, i64 52, !dbg !15356
  %_16.3.i46 = load float, ptr %58, align 4, !dbg !15356, !alias.scope !15343, !noalias !15354, !noundef !12
  %59 = getelementptr inbounds nuw i8, ptr %_43.0, i64 56, !dbg !15357
  %_17.3.i47 = load float, ptr %59, align 4, !dbg !15357, !alias.scope !15343, !noalias !15354, !noundef !12
  %60 = getelementptr inbounds nuw i8, ptr %_43.0, i64 60, !dbg !15358
  %_20.3.i48 = load i32, ptr %60, align 4, !dbg !15358, !alias.scope !15343, !noalias !15354, !noundef !12
  %_19.3.i49 = trunc i32 %_20.3.i48 to i16, !dbg !15358
  %_18.3.i50 = uitofp i16 %_19.3.i49 to float, !dbg !15359
  %_6.i.i.3.i51 = icmp eq i64 %_43.1, 4, !dbg !15347
  br i1 %_6.i.i.3.i51, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %bb4.4.i52, !dbg !15352

bb4.4.i52:                                        ; preds = %bb4.3.i43
  %_16.i.i.3.i53 = getelementptr inbounds nuw i8, ptr %_43.0, i64 64, !dbg !15361
  %_14.4.i54 = load float, ptr %_16.i.i.3.i53, align 4, !dbg !15353, !alias.scope !15343, !noalias !15354, !noundef !12
  %61 = getelementptr inbounds nuw i8, ptr %_43.0, i64 68, !dbg !15356
  %_16.4.i55 = load float, ptr %61, align 4, !dbg !15356, !alias.scope !15343, !noalias !15354, !noundef !12
  %62 = getelementptr inbounds nuw i8, ptr %_43.0, i64 72, !dbg !15357
  %_17.4.i56 = load float, ptr %62, align 4, !dbg !15357, !alias.scope !15343, !noalias !15354, !noundef !12
  %63 = getelementptr inbounds nuw i8, ptr %_43.0, i64 76, !dbg !15358
  %_20.4.i57 = load i32, ptr %63, align 4, !dbg !15358, !alias.scope !15343, !noalias !15354, !noundef !12
  %_19.4.i58 = trunc i32 %_20.4.i57 to i16, !dbg !15358
  %_18.4.i59 = uitofp i16 %_19.4.i58 to float, !dbg !15359
  %_6.i.i.4.i60 = icmp eq i64 %_43.1, 5, !dbg !15347
  br i1 %_6.i.i.4.i60, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %bb4.5.i61, !dbg !15352

bb4.5.i61:                                        ; preds = %bb4.4.i52
  %_16.i.i.4.i62 = getelementptr inbounds nuw i8, ptr %_43.0, i64 80, !dbg !15361
  %_14.5.i63 = load float, ptr %_16.i.i.4.i62, align 4, !dbg !15353, !alias.scope !15343, !noalias !15354, !noundef !12
  %64 = getelementptr inbounds nuw i8, ptr %_43.0, i64 84, !dbg !15356
  %_16.5.i64 = load float, ptr %64, align 4, !dbg !15356, !alias.scope !15343, !noalias !15354, !noundef !12
  %65 = getelementptr inbounds nuw i8, ptr %_43.0, i64 88, !dbg !15357
  %_17.5.i65 = load float, ptr %65, align 4, !dbg !15357, !alias.scope !15343, !noalias !15354, !noundef !12
  %66 = getelementptr inbounds nuw i8, ptr %_43.0, i64 92, !dbg !15358
  %_20.5.i66 = load i32, ptr %66, align 4, !dbg !15358, !alias.scope !15343, !noalias !15354, !noundef !12
  %_19.5.i67 = trunc i32 %_20.5.i66 to i16, !dbg !15358
  %_18.5.i68 = uitofp i16 %_19.5.i67 to float, !dbg !15359
  %_6.i.i.5.i69 = icmp eq i64 %_43.1, 6, !dbg !15347
  br i1 %_6.i.i.5.i69, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %bb4.6.i70, !dbg !15352

bb4.6.i70:                                        ; preds = %bb4.5.i61
  %_16.i.i.5.i71 = getelementptr inbounds nuw i8, ptr %_43.0, i64 96, !dbg !15361
  %_14.6.i72 = load float, ptr %_16.i.i.5.i71, align 4, !dbg !15353, !alias.scope !15343, !noalias !15354, !noundef !12
  %67 = getelementptr inbounds nuw i8, ptr %_43.0, i64 100, !dbg !15356
  %_16.6.i73 = load float, ptr %67, align 4, !dbg !15356, !alias.scope !15343, !noalias !15354, !noundef !12
  %68 = getelementptr inbounds nuw i8, ptr %_43.0, i64 104, !dbg !15357
  %_17.6.i74 = load float, ptr %68, align 4, !dbg !15357, !alias.scope !15343, !noalias !15354, !noundef !12
  %69 = getelementptr inbounds nuw i8, ptr %_43.0, i64 108, !dbg !15358
  %_20.6.i75 = load i32, ptr %69, align 4, !dbg !15358, !alias.scope !15343, !noalias !15354, !noundef !12
  %_19.6.i76 = trunc i32 %_20.6.i75 to i16, !dbg !15358
  %_18.6.i77 = uitofp i16 %_19.6.i76 to float, !dbg !15359
  %_6.i.i.6.i78 = icmp eq i64 %_43.1, 7, !dbg !15347
  br i1 %_6.i.i.6.i78, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %bb4.7.i79, !dbg !15352

bb4.7.i79:                                        ; preds = %bb4.6.i70
  %_16.i.i.6.i80 = getelementptr inbounds nuw i8, ptr %_43.0, i64 112, !dbg !15361
  %_14.7.i81 = load float, ptr %_16.i.i.6.i80, align 4, !dbg !15353, !alias.scope !15343, !noalias !15354, !noundef !12
  %70 = getelementptr inbounds nuw i8, ptr %_43.0, i64 116, !dbg !15356
  %_16.7.i82 = load float, ptr %70, align 4, !dbg !15356, !alias.scope !15343, !noalias !15354, !noundef !12
  %71 = getelementptr inbounds nuw i8, ptr %_43.0, i64 120, !dbg !15357
  %_17.7.i83 = load float, ptr %71, align 4, !dbg !15357, !alias.scope !15343, !noalias !15354, !noundef !12
  %72 = getelementptr inbounds nuw i8, ptr %_43.0, i64 124, !dbg !15358
  %_20.7.i84 = load i32, ptr %72, align 4, !dbg !15358, !alias.scope !15343, !noalias !15354, !noundef !12
  %_19.7.i85 = trunc i32 %_20.7.i84 to i16, !dbg !15358
  %_18.7.i86 = uitofp i16 %_19.7.i85 to float, !dbg !15359
  %_6.i.i.7.i87 = icmp eq i64 %_43.1, 8, !dbg !15347
  br i1 %_6.i.i.7.i87, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149, label %panic.i88, !dbg !15352

panic.i88:                                        ; preds = %bb4.7.i79
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_34ec9deef3d1165c325897b5845d9fe1) #30, !dbg !15363, !noalias !15364
  unreachable, !dbg !15363

_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit149: ; preds = %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit, %bb4.preheader.i17, %bb4.1.i25, %bb4.2.i34, %bb4.3.i43, %bb4.4.i52, %bb4.5.i61, %bb4.6.i70, %bb4.7.i79
  %remaining.sroa.0.0.i89 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_18.i23, %bb4.7.i79 ], [ %_18.i23, %bb4.6.i70 ], [ %_18.i23, %bb4.5.i61 ], [ %_18.i23, %bb4.4.i52 ], [ %_18.i23, %bb4.3.i43 ], [ %_18.i23, %bb4.2.i34 ], [ %_18.i23, %bb4.1.i25 ], [ %_18.i23, %bb4.preheader.i17 ]
  %remaining.sroa.5.0.i90 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_18.1.i32, %bb4.7.i79 ], [ %_18.1.i32, %bb4.6.i70 ], [ %_18.1.i32, %bb4.5.i61 ], [ %_18.1.i32, %bb4.4.i52 ], [ %_18.1.i32, %bb4.3.i43 ], [ %_18.1.i32, %bb4.2.i34 ], [ %_18.1.i32, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %remaining.sroa.6.0.i91 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_18.2.i41, %bb4.7.i79 ], [ %_18.2.i41, %bb4.6.i70 ], [ %_18.2.i41, %bb4.5.i61 ], [ %_18.2.i41, %bb4.4.i52 ], [ %_18.2.i41, %bb4.3.i43 ], [ %_18.2.i41, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %remaining.sroa.7.0.i92 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_18.3.i50, %bb4.7.i79 ], [ %_18.3.i50, %bb4.6.i70 ], [ %_18.3.i50, %bb4.5.i61 ], [ %_18.3.i50, %bb4.4.i52 ], [ %_18.3.i50, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %remaining.sroa.8.0.i93 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_18.4.i59, %bb4.7.i79 ], [ %_18.4.i59, %bb4.6.i70 ], [ %_18.4.i59, %bb4.5.i61 ], [ %_18.4.i59, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %remaining.sroa.9.0.i94 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_18.5.i68, %bb4.7.i79 ], [ %_18.5.i68, %bb4.6.i70 ], [ %_18.5.i68, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %remaining.sroa.10.0.i95 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_18.6.i77, %bb4.7.i79 ], [ %_18.6.i77, %bb4.6.i70 ], [ 0.000000e+00, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %remaining.sroa.11.0.i96 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_18.7.i86, %bb4.7.i79 ], [ 0.000000e+00, %bb4.6.i70 ], [ 0.000000e+00, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %step.sroa.0.0.i97 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_17.i20, %bb4.7.i79 ], [ %_17.i20, %bb4.6.i70 ], [ %_17.i20, %bb4.5.i61 ], [ %_17.i20, %bb4.4.i52 ], [ %_17.i20, %bb4.3.i43 ], [ %_17.i20, %bb4.2.i34 ], [ %_17.i20, %bb4.1.i25 ], [ %_17.i20, %bb4.preheader.i17 ]
  %step.sroa.5.0.i98 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_17.1.i29, %bb4.7.i79 ], [ %_17.1.i29, %bb4.6.i70 ], [ %_17.1.i29, %bb4.5.i61 ], [ %_17.1.i29, %bb4.4.i52 ], [ %_17.1.i29, %bb4.3.i43 ], [ %_17.1.i29, %bb4.2.i34 ], [ %_17.1.i29, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %step.sroa.6.0.i99 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_17.2.i38, %bb4.7.i79 ], [ %_17.2.i38, %bb4.6.i70 ], [ %_17.2.i38, %bb4.5.i61 ], [ %_17.2.i38, %bb4.4.i52 ], [ %_17.2.i38, %bb4.3.i43 ], [ %_17.2.i38, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %step.sroa.7.0.i100 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_17.3.i47, %bb4.7.i79 ], [ %_17.3.i47, %bb4.6.i70 ], [ %_17.3.i47, %bb4.5.i61 ], [ %_17.3.i47, %bb4.4.i52 ], [ %_17.3.i47, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %step.sroa.8.0.i101 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_17.4.i56, %bb4.7.i79 ], [ %_17.4.i56, %bb4.6.i70 ], [ %_17.4.i56, %bb4.5.i61 ], [ %_17.4.i56, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %step.sroa.9.0.i102 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_17.5.i65, %bb4.7.i79 ], [ %_17.5.i65, %bb4.6.i70 ], [ %_17.5.i65, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %step.sroa.10.0.i103 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_17.6.i74, %bb4.7.i79 ], [ %_17.6.i74, %bb4.6.i70 ], [ 0.000000e+00, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %step.sroa.11.0.i104 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_17.7.i83, %bb4.7.i79 ], [ 0.000000e+00, %bb4.6.i70 ], [ 0.000000e+00, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %target.sroa.0.0.i105 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_16.i19, %bb4.7.i79 ], [ %_16.i19, %bb4.6.i70 ], [ %_16.i19, %bb4.5.i61 ], [ %_16.i19, %bb4.4.i52 ], [ %_16.i19, %bb4.3.i43 ], [ %_16.i19, %bb4.2.i34 ], [ %_16.i19, %bb4.1.i25 ], [ %_16.i19, %bb4.preheader.i17 ]
  %target.sroa.5.0.i106 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_16.1.i28, %bb4.7.i79 ], [ %_16.1.i28, %bb4.6.i70 ], [ %_16.1.i28, %bb4.5.i61 ], [ %_16.1.i28, %bb4.4.i52 ], [ %_16.1.i28, %bb4.3.i43 ], [ %_16.1.i28, %bb4.2.i34 ], [ %_16.1.i28, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %target.sroa.6.0.i107 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_16.2.i37, %bb4.7.i79 ], [ %_16.2.i37, %bb4.6.i70 ], [ %_16.2.i37, %bb4.5.i61 ], [ %_16.2.i37, %bb4.4.i52 ], [ %_16.2.i37, %bb4.3.i43 ], [ %_16.2.i37, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %target.sroa.7.0.i108 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_16.3.i46, %bb4.7.i79 ], [ %_16.3.i46, %bb4.6.i70 ], [ %_16.3.i46, %bb4.5.i61 ], [ %_16.3.i46, %bb4.4.i52 ], [ %_16.3.i46, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %target.sroa.8.0.i109 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_16.4.i55, %bb4.7.i79 ], [ %_16.4.i55, %bb4.6.i70 ], [ %_16.4.i55, %bb4.5.i61 ], [ %_16.4.i55, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %target.sroa.9.0.i110 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_16.5.i64, %bb4.7.i79 ], [ %_16.5.i64, %bb4.6.i70 ], [ %_16.5.i64, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %target.sroa.10.0.i111 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_16.6.i73, %bb4.7.i79 ], [ %_16.6.i73, %bb4.6.i70 ], [ 0.000000e+00, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %target.sroa.11.0.i112 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_16.7.i82, %bb4.7.i79 ], [ 0.000000e+00, %bb4.6.i70 ], [ 0.000000e+00, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %current.sroa.0.0.i113 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_14.i18, %bb4.7.i79 ], [ %_14.i18, %bb4.6.i70 ], [ %_14.i18, %bb4.5.i61 ], [ %_14.i18, %bb4.4.i52 ], [ %_14.i18, %bb4.3.i43 ], [ %_14.i18, %bb4.2.i34 ], [ %_14.i18, %bb4.1.i25 ], [ %_14.i18, %bb4.preheader.i17 ]
  %current.sroa.5.0.i114 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_14.1.i27, %bb4.7.i79 ], [ %_14.1.i27, %bb4.6.i70 ], [ %_14.1.i27, %bb4.5.i61 ], [ %_14.1.i27, %bb4.4.i52 ], [ %_14.1.i27, %bb4.3.i43 ], [ %_14.1.i27, %bb4.2.i34 ], [ %_14.1.i27, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %current.sroa.6.0.i115 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_14.2.i36, %bb4.7.i79 ], [ %_14.2.i36, %bb4.6.i70 ], [ %_14.2.i36, %bb4.5.i61 ], [ %_14.2.i36, %bb4.4.i52 ], [ %_14.2.i36, %bb4.3.i43 ], [ %_14.2.i36, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %current.sroa.7.0.i116 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_14.3.i45, %bb4.7.i79 ], [ %_14.3.i45, %bb4.6.i70 ], [ %_14.3.i45, %bb4.5.i61 ], [ %_14.3.i45, %bb4.4.i52 ], [ %_14.3.i45, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %current.sroa.8.0.i117 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_14.4.i54, %bb4.7.i79 ], [ %_14.4.i54, %bb4.6.i70 ], [ %_14.4.i54, %bb4.5.i61 ], [ %_14.4.i54, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %current.sroa.9.0.i118 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_14.5.i63, %bb4.7.i79 ], [ %_14.5.i63, %bb4.6.i70 ], [ %_14.5.i63, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %current.sroa.10.0.i119 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_14.6.i72, %bb4.7.i79 ], [ %_14.6.i72, %bb4.6.i70 ], [ 0.000000e+00, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  %current.sroa.11.0.i120 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E6gatherB5_.exit ], [ %_14.7.i81, %bb4.7.i79 ], [ 0.000000e+00, %bb4.6.i70 ], [ 0.000000e+00, %bb4.5.i61 ], [ 0.000000e+00, %bb4.4.i52 ], [ 0.000000e+00, %bb4.3.i43 ], [ 0.000000e+00, %bb4.2.i34 ], [ 0.000000e+00, %bb4.1.i25 ], [ 0.000000e+00, %bb4.preheader.i17 ]
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(384) %_0, ptr noundef nonnull align 32 dereferenceable(384) %history, i64 384, i1 false), !dbg !15365
  %73 = getelementptr inbounds nuw i8, ptr %_0, i64 640, !dbg !15365
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %73, ptr noundef nonnull align 32 dereferenceable(32) %_14, i64 32, i1 false), !dbg !15365
  %74 = getelementptr inbounds nuw i8, ptr %_0, i64 672, !dbg !15365
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %74, ptr noundef nonnull align 32 dereferenceable(32) %_16, i64 32, i1 false), !dbg !15365
  %75 = getelementptr inbounds nuw i8, ptr %_0, i64 704, !dbg !15365
  store float %window.sroa.0.0, ptr %75, align 32, !dbg !15365
  %_18.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 708, !dbg !15365
  store float %window.sroa.5.0, ptr %_18.sroa.4.0..sroa_idx, align 4, !dbg !15365
  %_18.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 712, !dbg !15365
  store float %window.sroa.6.0, ptr %_18.sroa.5.0..sroa_idx, align 8, !dbg !15365
  %_18.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 716, !dbg !15365
  store float %window.sroa.7.0, ptr %_18.sroa.6.0..sroa_idx, align 4, !dbg !15365
  %_18.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 720, !dbg !15365
  store float %window.sroa.8.0, ptr %_18.sroa.7.0..sroa_idx, align 16, !dbg !15365
  %_18.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 724, !dbg !15365
  store float %window.sroa.9.0, ptr %_18.sroa.8.0..sroa_idx, align 4, !dbg !15365
  %_18.sroa.9.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 728, !dbg !15365
  store float %window.sroa.10.0, ptr %_18.sroa.9.0..sroa_idx, align 8, !dbg !15365
  %_18.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 732, !dbg !15365
  store float %window.sroa.11.0, ptr %_18.sroa.10.0..sroa_idx, align 4, !dbg !15365
  %76 = getelementptr inbounds nuw i8, ptr %_0, i64 384, !dbg !15365
  store float %current.sroa.0.0.i, ptr %76, align 32, !dbg !15365
  %_21.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 388, !dbg !15365
  store float %current.sroa.5.0.i, ptr %_21.sroa.4.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 392, !dbg !15365
  store float %current.sroa.6.0.i, ptr %_21.sroa.5.0..sroa_idx, align 8, !dbg !15365
  %_21.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 396, !dbg !15365
  store float %current.sroa.7.0.i, ptr %_21.sroa.6.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 400, !dbg !15365
  store float %current.sroa.8.0.i, ptr %_21.sroa.7.0..sroa_idx, align 16, !dbg !15365
  %_21.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 404, !dbg !15365
  store float %current.sroa.9.0.i, ptr %_21.sroa.8.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.9.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 408, !dbg !15365
  store float %current.sroa.10.0.i, ptr %_21.sroa.9.0..sroa_idx, align 8, !dbg !15365
  %_21.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 412, !dbg !15365
  store float %current.sroa.11.0.i, ptr %_21.sroa.10.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 416, !dbg !15365
  store float %target.sroa.0.0.i, ptr %_21.sroa.11.0..sroa_idx, align 32, !dbg !15365
  %_21.sroa.12.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 420, !dbg !15365
  store float %target.sroa.5.0.i, ptr %_21.sroa.12.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.13.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 424, !dbg !15365
  store float %target.sroa.6.0.i, ptr %_21.sroa.13.0..sroa_idx, align 8, !dbg !15365
  %_21.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 428, !dbg !15365
  store float %target.sroa.7.0.i, ptr %_21.sroa.14.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.15.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 432, !dbg !15365
  store float %target.sroa.8.0.i, ptr %_21.sroa.15.0..sroa_idx, align 16, !dbg !15365
  %_21.sroa.16.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 436, !dbg !15365
  store float %target.sroa.9.0.i, ptr %_21.sroa.16.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.17.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 440, !dbg !15365
  store float %target.sroa.10.0.i, ptr %_21.sroa.17.0..sroa_idx, align 8, !dbg !15365
  %_21.sroa.18.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 444, !dbg !15365
  store float %target.sroa.11.0.i, ptr %_21.sroa.18.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.19.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 448, !dbg !15365
  store float %step.sroa.0.0.i, ptr %_21.sroa.19.0..sroa_idx, align 32, !dbg !15365
  %_21.sroa.20.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 452, !dbg !15365
  store float %step.sroa.5.0.i, ptr %_21.sroa.20.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.21.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 456, !dbg !15365
  store float %step.sroa.6.0.i, ptr %_21.sroa.21.0..sroa_idx, align 8, !dbg !15365
  %_21.sroa.22.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 460, !dbg !15365
  store float %step.sroa.7.0.i, ptr %_21.sroa.22.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.23.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 464, !dbg !15365
  store float %step.sroa.8.0.i, ptr %_21.sroa.23.0..sroa_idx, align 16, !dbg !15365
  %_21.sroa.24.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 468, !dbg !15365
  store float %step.sroa.9.0.i, ptr %_21.sroa.24.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.25.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 472, !dbg !15365
  store float %step.sroa.10.0.i, ptr %_21.sroa.25.0..sroa_idx, align 8, !dbg !15365
  %_21.sroa.26.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 476, !dbg !15365
  store float %step.sroa.11.0.i, ptr %_21.sroa.26.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.27.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 480, !dbg !15365
  store float %remaining.sroa.0.0.i, ptr %_21.sroa.27.0..sroa_idx, align 32, !dbg !15365
  %_21.sroa.28.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 484, !dbg !15365
  store float %remaining.sroa.5.0.i, ptr %_21.sroa.28.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.29.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 488, !dbg !15365
  store float %remaining.sroa.6.0.i, ptr %_21.sroa.29.0..sroa_idx, align 8, !dbg !15365
  %_21.sroa.30.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 492, !dbg !15365
  store float %remaining.sroa.7.0.i, ptr %_21.sroa.30.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.31.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 496, !dbg !15365
  store float %remaining.sroa.8.0.i, ptr %_21.sroa.31.0..sroa_idx, align 16, !dbg !15365
  %_21.sroa.32.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 500, !dbg !15365
  store float %remaining.sroa.9.0.i, ptr %_21.sroa.32.0..sroa_idx, align 4, !dbg !15365
  %_21.sroa.33.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 504, !dbg !15365
  store float %remaining.sroa.10.0.i, ptr %_21.sroa.33.0..sroa_idx, align 8, !dbg !15365
  %_21.sroa.34.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 508, !dbg !15365
  store float %remaining.sroa.11.0.i, ptr %_21.sroa.34.0..sroa_idx, align 4, !dbg !15365
  %77 = getelementptr inbounds nuw i8, ptr %_0, i64 512, !dbg !15365
  store float %current.sroa.0.0.i113, ptr %77, align 32, !dbg !15365
  %_23.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 516, !dbg !15365
  store float %current.sroa.5.0.i114, ptr %_23.sroa.4.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 520, !dbg !15365
  store float %current.sroa.6.0.i115, ptr %_23.sroa.5.0..sroa_idx, align 8, !dbg !15365
  %_23.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 524, !dbg !15365
  store float %current.sroa.7.0.i116, ptr %_23.sroa.6.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 528, !dbg !15365
  store float %current.sroa.8.0.i117, ptr %_23.sroa.7.0..sroa_idx, align 16, !dbg !15365
  %_23.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 532, !dbg !15365
  store float %current.sroa.9.0.i118, ptr %_23.sroa.8.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.9.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 536, !dbg !15365
  store float %current.sroa.10.0.i119, ptr %_23.sroa.9.0..sroa_idx, align 8, !dbg !15365
  %_23.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 540, !dbg !15365
  store float %current.sroa.11.0.i120, ptr %_23.sroa.10.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 544, !dbg !15365
  store float %target.sroa.0.0.i105, ptr %_23.sroa.11.0..sroa_idx, align 32, !dbg !15365
  %_23.sroa.12.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 548, !dbg !15365
  store float %target.sroa.5.0.i106, ptr %_23.sroa.12.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.13.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 552, !dbg !15365
  store float %target.sroa.6.0.i107, ptr %_23.sroa.13.0..sroa_idx, align 8, !dbg !15365
  %_23.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 556, !dbg !15365
  store float %target.sroa.7.0.i108, ptr %_23.sroa.14.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.15.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 560, !dbg !15365
  store float %target.sroa.8.0.i109, ptr %_23.sroa.15.0..sroa_idx, align 16, !dbg !15365
  %_23.sroa.16.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 564, !dbg !15365
  store float %target.sroa.9.0.i110, ptr %_23.sroa.16.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.17.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 568, !dbg !15365
  store float %target.sroa.10.0.i111, ptr %_23.sroa.17.0..sroa_idx, align 8, !dbg !15365
  %_23.sroa.18.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 572, !dbg !15365
  store float %target.sroa.11.0.i112, ptr %_23.sroa.18.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.19.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 576, !dbg !15365
  store float %step.sroa.0.0.i97, ptr %_23.sroa.19.0..sroa_idx, align 32, !dbg !15365
  %_23.sroa.20.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 580, !dbg !15365
  store float %step.sroa.5.0.i98, ptr %_23.sroa.20.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.21.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 584, !dbg !15365
  store float %step.sroa.6.0.i99, ptr %_23.sroa.21.0..sroa_idx, align 8, !dbg !15365
  %_23.sroa.22.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 588, !dbg !15365
  store float %step.sroa.7.0.i100, ptr %_23.sroa.22.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.23.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 592, !dbg !15365
  store float %step.sroa.8.0.i101, ptr %_23.sroa.23.0..sroa_idx, align 16, !dbg !15365
  %_23.sroa.24.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 596, !dbg !15365
  store float %step.sroa.9.0.i102, ptr %_23.sroa.24.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.25.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 600, !dbg !15365
  store float %step.sroa.10.0.i103, ptr %_23.sroa.25.0..sroa_idx, align 8, !dbg !15365
  %_23.sroa.26.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 604, !dbg !15365
  store float %step.sroa.11.0.i104, ptr %_23.sroa.26.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.27.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 608, !dbg !15365
  store float %remaining.sroa.0.0.i89, ptr %_23.sroa.27.0..sroa_idx, align 32, !dbg !15365
  %_23.sroa.28.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 612, !dbg !15365
  store float %remaining.sroa.5.0.i90, ptr %_23.sroa.28.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.29.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 616, !dbg !15365
  store float %remaining.sroa.6.0.i91, ptr %_23.sroa.29.0..sroa_idx, align 8, !dbg !15365
  %_23.sroa.30.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 620, !dbg !15365
  store float %remaining.sroa.7.0.i92, ptr %_23.sroa.30.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.31.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 624, !dbg !15365
  store float %remaining.sroa.8.0.i93, ptr %_23.sroa.31.0..sroa_idx, align 16, !dbg !15365
  %_23.sroa.32.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 628, !dbg !15365
  store float %remaining.sroa.9.0.i94, ptr %_23.sroa.32.0..sroa_idx, align 4, !dbg !15365
  %_23.sroa.33.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 632, !dbg !15365
  store float %remaining.sroa.10.0.i95, ptr %_23.sroa.33.0..sroa_idx, align 8, !dbg !15365
  %_23.sroa.34.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 636, !dbg !15365
  store float %remaining.sroa.11.0.i96, ptr %_23.sroa.34.0..sroa_idx, align 4, !dbg !15365
  call void @llvm.lifetime.end.p0(ptr nonnull %_16), !dbg !15366
  call void @llvm.lifetime.end.p0(ptr nonnull %_14), !dbg !15366
  call void @llvm.lifetime.end.p0(ptr nonnull %history), !dbg !15367
  ret void, !dbg !15368

bb5.1:                                            ; preds = %bb5.preheader
  %_16.i.i = getelementptr inbounds nuw i8, ptr %_39.0, i64 12, !dbg !15369
  %_12.1 = load i32, ptr %_16.i.i, align 4, !dbg !15282, !noundef !12
  %78 = uitofp i32 %_12.1 to float, !dbg !15284
  %_6.i.i.1 = icmp eq i64 %_39.1, 2, !dbg !15276
  br i1 %_6.i.i.1, label %bb6, label %bb5.2, !dbg !15281

bb5.2:                                            ; preds = %bb5.1
  %_16.i.i.1 = getelementptr inbounds nuw i8, ptr %_39.0, i64 24, !dbg !15369
  %_12.2 = load i32, ptr %_16.i.i.1, align 4, !dbg !15282, !noundef !12
  %79 = uitofp i32 %_12.2 to float, !dbg !15284
  %_6.i.i.2 = icmp eq i64 %_39.1, 3, !dbg !15276
  br i1 %_6.i.i.2, label %bb6, label %bb5.3, !dbg !15281

bb5.3:                                            ; preds = %bb5.2
  %_16.i.i.2 = getelementptr inbounds nuw i8, ptr %_39.0, i64 36, !dbg !15369
  %_12.3 = load i32, ptr %_16.i.i.2, align 4, !dbg !15282, !noundef !12
  %80 = uitofp i32 %_12.3 to float, !dbg !15284
  %_6.i.i.3 = icmp eq i64 %_39.1, 4, !dbg !15276
  br i1 %_6.i.i.3, label %bb6, label %bb5.4, !dbg !15281

bb5.4:                                            ; preds = %bb5.3
  %_16.i.i.3 = getelementptr inbounds nuw i8, ptr %_39.0, i64 48, !dbg !15369
  %_12.4 = load i32, ptr %_16.i.i.3, align 4, !dbg !15282, !noundef !12
  %81 = uitofp i32 %_12.4 to float, !dbg !15284
  %_6.i.i.4 = icmp eq i64 %_39.1, 5, !dbg !15276
  br i1 %_6.i.i.4, label %bb6, label %bb5.5, !dbg !15281

bb5.5:                                            ; preds = %bb5.4
  %_16.i.i.4 = getelementptr inbounds nuw i8, ptr %_39.0, i64 60, !dbg !15369
  %_12.5 = load i32, ptr %_16.i.i.4, align 4, !dbg !15282, !noundef !12
  %82 = uitofp i32 %_12.5 to float, !dbg !15284
  %_6.i.i.5 = icmp eq i64 %_39.1, 6, !dbg !15276
  br i1 %_6.i.i.5, label %bb6, label %bb5.6, !dbg !15281

bb5.6:                                            ; preds = %bb5.5
  %_16.i.i.5 = getelementptr inbounds nuw i8, ptr %_39.0, i64 72, !dbg !15369
  %_12.6 = load i32, ptr %_16.i.i.5, align 4, !dbg !15282, !noundef !12
  %83 = uitofp i32 %_12.6 to float, !dbg !15284
  %_6.i.i.6 = icmp eq i64 %_39.1, 7, !dbg !15276
  br i1 %_6.i.i.6, label %bb6, label %bb5.7, !dbg !15281

bb5.7:                                            ; preds = %bb5.6
  %_16.i.i.6 = getelementptr inbounds nuw i8, ptr %_39.0, i64 84, !dbg !15369
  %_12.7 = load i32, ptr %_16.i.i.6, align 4, !dbg !15282, !noundef !12
  %84 = uitofp i32 %_12.7 to float, !dbg !15284
  %_6.i.i.7 = icmp eq i64 %_39.1, 8, !dbg !15276
  br i1 %_6.i.i.7, label %bb6, label %panic, !dbg !15281

panic:                                            ; preds = %bb5.7
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3862c97910d0689d8b59a4255cb98beb) #30, !dbg !15284
  unreachable, !dbg !15284
}
