define internal fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE5storeB5_(ptr dead_on_return noalias noundef nonnull readonly align 4 captures(none) dereferenceable(92) %self, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(200) %state) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !16105 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16106), !dbg !16109
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16110), !dbg !16109
  %0 = getelementptr inbounds nuw i8, ptr %state, i64 192, !dbg !16112
  %1 = load i64, ptr %0, align 8, !dbg !16112, !alias.scope !16110, !noalias !16106, !noundef !12
  %_56.0.i = load ptr, ptr %state, align 8, !dbg !16115, !alias.scope !16110, !noalias !16106, !nonnull !12, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %state, i64 8, !dbg !16115
  %_56.1.i = load i64, ptr %2, align 8, !dbg !16115, !alias.scope !16110, !noalias !16106, !noundef !12
  %_10.i = load float, ptr %self, align 4, !dbg !16117, !alias.scope !16106, !noalias !16110, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16119), !dbg !16122
  %_4.not.i.i.i = icmp eq i64 %_56.1.i, 0, !dbg !16125
  br i1 %_4.not.i.i.i, label %panic.i.i.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit.i, !dbg !16125

panic.i.i.i:                                      ; preds = %start
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16125, !noalias !16128
  unreachable, !dbg !16125

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit.i: ; preds = %start
  store float %_10.i, ptr %_56.0.i, align 4, !dbg !16125, !alias.scope !16119, !noalias !16131
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 4, !dbg !16132
  %_14.i = load float, ptr %3, align 4, !dbg !16132, !alias.scope !16106, !noalias !16110, !noundef !12
  %_9.i.i = icmp ugt i64 %1, %_56.1.i, !dbg !16133
  br i1 %_9.i.i, label %bb2.i.i, label %bb3.i.i, !dbg !16133, !prof !639

bb3.i.i:                                          ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16139), !dbg !16142
  %_4.not.i.i5.i = icmp eq i64 %_56.1.i, %1, !dbg !16143
  br i1 %_4.not.i.i5.i, label %panic.i.i6.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit7.i, !dbg !16143

panic.i.i6.i:                                     ; preds = %bb3.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16143, !noalias !16145
  unreachable, !dbg !16143

bb2.i.i:                                          ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %1, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16148, !noalias !16149
  unreachable, !dbg !16148

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit7.i: ; preds = %bb3.i.i
  %_16.i.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %1, !dbg !16150
  store float %_14.i, ptr %_16.i.i, align 4, !dbg !16143, !alias.scope !16139, !noalias !16149
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 8, !dbg !16155
  %_18.i = load float, ptr %4, align 4, !dbg !16155, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i.i = shl i64 %1, 1, !dbg !16156
  %_9.i11.i = icmp ugt i64 %_5.i.i, %_56.1.i, !dbg !16158
  br i1 %_9.i11.i, label %bb2.i17.i, label %bb3.i12.i, !dbg !16158, !prof !639

bb3.i12.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit7.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16161), !dbg !16164
  %_4.not.i.i14.i = icmp eq i64 %_56.1.i, %_5.i.i, !dbg !16165
  br i1 %_4.not.i.i14.i, label %panic.i.i16.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit18.i, !dbg !16165

panic.i.i16.i:                                    ; preds = %bb3.i12.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16165, !noalias !16167
  unreachable, !dbg !16165

bb2.i17.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit7.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16170, !noalias !16171
  unreachable, !dbg !16170

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit18.i: ; preds = %bb3.i12.i
  %_16.i15.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i.i, !dbg !16172
  store float %_18.i, ptr %_16.i15.i, align 4, !dbg !16165, !alias.scope !16161, !noalias !16171
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 12, !dbg !16174
  %_22.i = load float, ptr %5, align 4, !dbg !16174, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i22.i = mul i64 %1, 3, !dbg !16175
  %_9.i23.i = icmp ugt i64 %_5.i22.i, %_56.1.i, !dbg !16177
  br i1 %_9.i23.i, label %bb2.i29.i, label %bb3.i24.i, !dbg !16177, !prof !639

bb3.i24.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit18.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16180), !dbg !16183
  %_4.not.i.i26.i = icmp eq i64 %_56.1.i, %_5.i22.i, !dbg !16184
  br i1 %_4.not.i.i26.i, label %panic.i.i28.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit30.i, !dbg !16184

panic.i.i28.i:                                    ; preds = %bb3.i24.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16184, !noalias !16186
  unreachable, !dbg !16184

bb2.i29.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit18.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i22.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16189, !noalias !16190
  unreachable, !dbg !16189

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit30.i: ; preds = %bb3.i24.i
  %_16.i27.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i22.i, !dbg !16191
  store float %_22.i, ptr %_16.i27.i, align 4, !dbg !16184, !alias.scope !16180, !noalias !16190
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 16, !dbg !16193
  %_26.i = load float, ptr %6, align 4, !dbg !16193, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i34.i = shl i64 %1, 2, !dbg !16194
  %_9.i35.i = icmp ugt i64 %_5.i34.i, %_56.1.i, !dbg !16196
  br i1 %_9.i35.i, label %bb2.i41.i, label %bb3.i36.i, !dbg !16196, !prof !639

bb3.i36.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit30.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16199), !dbg !16202
  %_4.not.i.i38.i = icmp eq i64 %_56.1.i, %_5.i34.i, !dbg !16203
  br i1 %_4.not.i.i38.i, label %panic.i.i40.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit42.i, !dbg !16203

panic.i.i40.i:                                    ; preds = %bb3.i36.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16203, !noalias !16205
  unreachable, !dbg !16203

bb2.i41.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit30.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i34.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16208, !noalias !16209
  unreachable, !dbg !16208

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit42.i: ; preds = %bb3.i36.i
  %_16.i39.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i34.i, !dbg !16210
  store float %_26.i, ptr %_16.i39.i, align 4, !dbg !16203, !alias.scope !16199, !noalias !16209
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 20, !dbg !16212
  %_30.i = load float, ptr %7, align 4, !dbg !16212, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i46.i = mul i64 %1, 5, !dbg !16213
  %_9.i47.i = icmp ugt i64 %_5.i46.i, %_56.1.i, !dbg !16215
  br i1 %_9.i47.i, label %bb2.i53.i, label %bb3.i48.i, !dbg !16215, !prof !639

bb3.i48.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit42.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16218), !dbg !16221
  %_4.not.i.i50.i = icmp eq i64 %_56.1.i, %_5.i46.i, !dbg !16222
  br i1 %_4.not.i.i50.i, label %panic.i.i52.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit54.i, !dbg !16222

panic.i.i52.i:                                    ; preds = %bb3.i48.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16222, !noalias !16224
  unreachable, !dbg !16222

bb2.i53.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit42.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i46.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16227, !noalias !16228
  unreachable, !dbg !16227

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit54.i: ; preds = %bb3.i48.i
  %_16.i51.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i46.i, !dbg !16229
  store float %_30.i, ptr %_16.i51.i, align 4, !dbg !16222, !alias.scope !16218, !noalias !16228
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 24, !dbg !16231
  %_34.i = load float, ptr %8, align 4, !dbg !16231, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i58.i = mul i64 %1, 6, !dbg !16232
  %_9.i59.i = icmp ugt i64 %_5.i58.i, %_56.1.i, !dbg !16234
  br i1 %_9.i59.i, label %bb2.i65.i, label %bb3.i60.i, !dbg !16234, !prof !639

bb3.i60.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit54.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16237), !dbg !16240
  %_4.not.i.i62.i = icmp eq i64 %_56.1.i, %_5.i58.i, !dbg !16241
  br i1 %_4.not.i.i62.i, label %panic.i.i64.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit66.i, !dbg !16241

panic.i.i64.i:                                    ; preds = %bb3.i60.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16241, !noalias !16243
  unreachable, !dbg !16241

bb2.i65.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit54.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i58.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16246, !noalias !16247
  unreachable, !dbg !16246

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit66.i: ; preds = %bb3.i60.i
  %_16.i63.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i58.i, !dbg !16248
  store float %_34.i, ptr %_16.i63.i, align 4, !dbg !16241, !alias.scope !16237, !noalias !16247
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 28, !dbg !16250
  %_38.i = load float, ptr %9, align 4, !dbg !16250, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i70.i = mul i64 %1, 7, !dbg !16251
  %_9.i71.i = icmp ugt i64 %_5.i70.i, %_56.1.i, !dbg !16253
  br i1 %_9.i71.i, label %bb2.i77.i, label %bb3.i72.i, !dbg !16253, !prof !639

bb3.i72.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit66.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16256), !dbg !16259
  %_4.not.i.i74.i = icmp eq i64 %_56.1.i, %_5.i70.i, !dbg !16260
  br i1 %_4.not.i.i74.i, label %panic.i.i76.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit78.i, !dbg !16260

panic.i.i76.i:                                    ; preds = %bb3.i72.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16260, !noalias !16262
  unreachable, !dbg !16260

bb2.i77.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit66.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i70.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16265, !noalias !16266
  unreachable, !dbg !16265

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit78.i: ; preds = %bb3.i72.i
  %_16.i75.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i70.i, !dbg !16267
  store float %_38.i, ptr %_16.i75.i, align 4, !dbg !16260, !alias.scope !16256, !noalias !16266
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 32, !dbg !16269
  %_42.i = load float, ptr %10, align 4, !dbg !16269, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i82.i = shl i64 %1, 3, !dbg !16270
  %_9.i83.i = icmp ugt i64 %_5.i82.i, %_56.1.i, !dbg !16272
  br i1 %_9.i83.i, label %bb2.i89.i, label %bb3.i84.i, !dbg !16272, !prof !639

bb3.i84.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit78.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16275), !dbg !16278
  %_4.not.i.i86.i = icmp eq i64 %_56.1.i, %_5.i82.i, !dbg !16279
  br i1 %_4.not.i.i86.i, label %panic.i.i88.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit90.i, !dbg !16279

panic.i.i88.i:                                    ; preds = %bb3.i84.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16279, !noalias !16281
  unreachable, !dbg !16279

bb2.i89.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit78.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i82.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16284, !noalias !16285
  unreachable, !dbg !16284

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit90.i: ; preds = %bb3.i84.i
  %_16.i87.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i82.i, !dbg !16286
  store float %_42.i, ptr %_16.i87.i, align 4, !dbg !16279, !alias.scope !16275, !noalias !16285
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 36, !dbg !16288
  %_46.i = load float, ptr %11, align 4, !dbg !16288, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i94.i = mul i64 %1, 9, !dbg !16289
  %_9.i95.i = icmp ugt i64 %_5.i94.i, %_56.1.i, !dbg !16291
  br i1 %_9.i95.i, label %bb2.i101.i, label %bb3.i96.i, !dbg !16291, !prof !639

bb3.i96.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit90.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16294), !dbg !16297
  %_4.not.i.i98.i = icmp eq i64 %_56.1.i, %_5.i94.i, !dbg !16298
  br i1 %_4.not.i.i98.i, label %panic.i.i100.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit102.i, !dbg !16298

panic.i.i100.i:                                   ; preds = %bb3.i96.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16298, !noalias !16300
  unreachable, !dbg !16298

bb2.i101.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit90.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i94.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16303, !noalias !16304
  unreachable, !dbg !16303

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit102.i: ; preds = %bb3.i96.i
  %_16.i99.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i94.i, !dbg !16305
  store float %_46.i, ptr %_16.i99.i, align 4, !dbg !16298, !alias.scope !16294, !noalias !16304
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 40, !dbg !16307
  %_50.i = load float, ptr %12, align 4, !dbg !16307, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i106.i = mul i64 %1, 10, !dbg !16308
  %_9.i107.i = icmp ugt i64 %_5.i106.i, %_56.1.i, !dbg !16310
  br i1 %_9.i107.i, label %bb2.i113.i, label %bb3.i108.i, !dbg !16310, !prof !639

bb3.i108.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit102.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16313), !dbg !16316
  %_4.not.i.i110.i = icmp eq i64 %_56.1.i, %_5.i106.i, !dbg !16317
  br i1 %_4.not.i.i110.i, label %panic.i.i112.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit114.i, !dbg !16317

panic.i.i112.i:                                   ; preds = %bb3.i108.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16317, !noalias !16319
  unreachable, !dbg !16317

bb2.i113.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit102.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i106.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16322, !noalias !16323
  unreachable, !dbg !16322

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit114.i: ; preds = %bb3.i108.i
  %_16.i111.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i106.i, !dbg !16324
  store float %_50.i, ptr %_16.i111.i, align 4, !dbg !16317, !alias.scope !16313, !noalias !16323
  %13 = getelementptr inbounds nuw i8, ptr %self, i64 44, !dbg !16326
  %_54.i = load float, ptr %13, align 4, !dbg !16326, !alias.scope !16106, !noalias !16110, !noundef !12
  %_5.i118.i = mul i64 %1, 11, !dbg !16327
  %_9.i119.i = icmp ugt i64 %_5.i118.i, %_56.1.i, !dbg !16329
  br i1 %_9.i119.i, label %bb2.i125.i, label %bb3.i120.i, !dbg !16329, !prof !639

bb3.i120.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit114.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16332), !dbg !16335
  %_4.not.i.i122.i = icmp eq i64 %_56.1.i, %_5.i118.i, !dbg !16336
  br i1 %_4.not.i.i122.i, label %panic.i.i124.i, label %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE5storeB5_.exit, !dbg !16336

panic.i.i124.i:                                   ; preds = %bb3.i120.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16336, !noalias !16338
  unreachable, !dbg !16336

bb2.i125.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE5store0B7_.exit114.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_5.i118.i, i64 noundef %_56.1.i, i64 noundef %_56.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2607b4735793736b813e4d78bd281000) #30, !dbg !16341, !noalias !16342
  unreachable, !dbg !16341

_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE5storeB5_.exit: ; preds = %bb3.i120.i
  %_16.i123.i = getelementptr inbounds nuw float, ptr %_56.0.i, i64 %_5.i118.i, !dbg !16343
  store float %_54.i, ptr %_16.i123.i, align 4, !dbg !16336, !alias.scope !16332, !noalias !16342
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 80, !dbg !16345
  %_6 = load float, ptr %14, align 4, !dbg !16345, !noundef !12
  %15 = getelementptr inbounds nuw i8, ptr %state, i64 64, !dbg !16346
  %_21.0 = load ptr, ptr %15, align 8, !dbg !16346, !nonnull !12, !noundef !12
  %16 = getelementptr inbounds nuw i8, ptr %state, i64 72, !dbg !16346
  %_21.1 = load i64, ptr %16, align 8, !dbg !16346, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16347), !dbg !16350
  %_4.not.i1 = icmp eq i64 %_21.1, 0, !dbg !16351
  br i1 %_4.not.i1, label %panic.i2, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3, !dbg !16351

panic.i2:                                         ; preds = %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE5storeB5_.exit
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16351, !noalias !16347
  unreachable, !dbg !16351

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3: ; preds = %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE5storeB5_.exit
  store float %_6, ptr %_21.0, align 4, !dbg !16351, !alias.scope !16347
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 84, !dbg !16353
  %_9 = load float, ptr %17, align 4, !dbg !16353, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %state, i64 96, !dbg !16354
  %_22.0 = load ptr, ptr %18, align 8, !dbg !16354, !nonnull !12, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %state, i64 104, !dbg !16354
  %_22.1 = load i64, ptr %19, align 8, !dbg !16354, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16355), !dbg !16358
  %_4.not.i = icmp eq i64 %_22.1, 0, !dbg !16359
  br i1 %_4.not.i, label %panic.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit, !dbg !16359

panic.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #30, !dbg !16359, !noalias !16355
  unreachable, !dbg !16359

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3
  store float %_9, ptr %_22.0, align 4, !dbg !16359, !alias.scope !16355
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 48, !dbg !16361
  %_12.sroa.0.0.copyload = load float, ptr %20, align 4, !dbg !16361
  %_12.sroa.434.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 56, !dbg !16361
  %_12.sroa.434.0.copyload = load float, ptr %_12.sroa.434.0..sroa_idx, align 4, !dbg !16361
  %_12.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 60, !dbg !16361
  %_12.sroa.5.0.copyload = load float, ptr %_12.sroa.5.0..sroa_idx, align 4, !dbg !16361
  %21 = getelementptr inbounds nuw i8, ptr %state, i64 128, !dbg !16362
  %_23.0 = load ptr, ptr %21, align 8, !dbg !16362, !nonnull !12, !noundef !12
  %22 = getelementptr inbounds nuw i8, ptr %state, i64 136, !dbg !16362
  %_23.1 = load i64, ptr %22, align 8, !dbg !16362, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16363), !dbg !16366
  %_7.i.i19.i = icmp eq i64 %_23.1, 0, !dbg !16367
  br i1 %_7.i.i19.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %bb7.preheader.i, !dbg !16377

bb7.preheader.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit
  store float %_12.sroa.0.0.copyload, ptr %_23.0, align 4, !dbg !16378, !alias.scope !16363, !noalias !16380
  %23 = getelementptr inbounds nuw i8, ptr %_23.0, i64 8, !dbg !16382
  store float %_12.sroa.434.0.copyload, ptr %23, align 4, !dbg !16382, !alias.scope !16363, !noalias !16380
  %24 = getelementptr inbounds nuw i8, ptr %_23.0, i64 12, !dbg !16383
  %25 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_12.sroa.5.0.copyload), !dbg !16383
  store i32 %25, ptr %24, align 4, !dbg !16383, !alias.scope !16363, !noalias !16380
  %_7.i.i.i = icmp eq i64 %_23.1, 1, !dbg !16367
  br i1 %_7.i.i.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %bb7.1.i, !dbg !16377

bb7.1.i:                                          ; preds = %bb7.preheader.i
  %_17.i.i.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 16, !dbg !16384
  store float 0.000000e+00, ptr %_17.i.i.i, align 4, !dbg !16378, !alias.scope !16363, !noalias !16380
  %26 = getelementptr inbounds nuw i8, ptr %_23.0, i64 24, !dbg !16382
  store float 0.000000e+00, ptr %26, align 4, !dbg !16382, !alias.scope !16363, !noalias !16380
  %27 = getelementptr inbounds nuw i8, ptr %_23.0, i64 28, !dbg !16383
  store i32 0, ptr %27, align 4, !dbg !16383, !alias.scope !16363, !noalias !16380
  %_7.i.i.1.i = icmp eq i64 %_23.1, 2, !dbg !16367
  br i1 %_7.i.i.1.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %bb7.2.i, !dbg !16377

bb7.2.i:                                          ; preds = %bb7.1.i
  %_17.i.i.1.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 32, !dbg !16384
  store float 0.000000e+00, ptr %_17.i.i.1.i, align 4, !dbg !16378, !alias.scope !16363, !noalias !16380
  %28 = getelementptr inbounds nuw i8, ptr %_23.0, i64 40, !dbg !16382
  store float 0.000000e+00, ptr %28, align 4, !dbg !16382, !alias.scope !16363, !noalias !16380
  %29 = getelementptr inbounds nuw i8, ptr %_23.0, i64 44, !dbg !16383
  store i32 0, ptr %29, align 4, !dbg !16383, !alias.scope !16363, !noalias !16380
  %_7.i.i.2.i = icmp eq i64 %_23.1, 3, !dbg !16367
  br i1 %_7.i.i.2.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %bb7.3.i, !dbg !16377

bb7.3.i:                                          ; preds = %bb7.2.i
  %_17.i.i.2.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 48, !dbg !16384
  store float 0.000000e+00, ptr %_17.i.i.2.i, align 4, !dbg !16378, !alias.scope !16363, !noalias !16380
  %30 = getelementptr inbounds nuw i8, ptr %_23.0, i64 56, !dbg !16382
  store float 0.000000e+00, ptr %30, align 4, !dbg !16382, !alias.scope !16363, !noalias !16380
  %31 = getelementptr inbounds nuw i8, ptr %_23.0, i64 60, !dbg !16383
  store i32 0, ptr %31, align 4, !dbg !16383, !alias.scope !16363, !noalias !16380
  %_7.i.i.3.i = icmp eq i64 %_23.1, 4, !dbg !16367
  br i1 %_7.i.i.3.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %bb7.4.i, !dbg !16377

bb7.4.i:                                          ; preds = %bb7.3.i
  %_17.i.i.3.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 64, !dbg !16384
  store float 0.000000e+00, ptr %_17.i.i.3.i, align 4, !dbg !16378, !alias.scope !16363, !noalias !16380
  %32 = getelementptr inbounds nuw i8, ptr %_23.0, i64 72, !dbg !16382
  store float 0.000000e+00, ptr %32, align 4, !dbg !16382, !alias.scope !16363, !noalias !16380
  %33 = getelementptr inbounds nuw i8, ptr %_23.0, i64 76, !dbg !16383
  store i32 0, ptr %33, align 4, !dbg !16383, !alias.scope !16363, !noalias !16380
  %_7.i.i.4.i = icmp eq i64 %_23.1, 5, !dbg !16367
  br i1 %_7.i.i.4.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %bb7.5.i, !dbg !16377

bb7.5.i:                                          ; preds = %bb7.4.i
  %_17.i.i.4.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 80, !dbg !16384
  store float 0.000000e+00, ptr %_17.i.i.4.i, align 4, !dbg !16378, !alias.scope !16363, !noalias !16380
  %34 = getelementptr inbounds nuw i8, ptr %_23.0, i64 88, !dbg !16382
  store float 0.000000e+00, ptr %34, align 4, !dbg !16382, !alias.scope !16363, !noalias !16380
  %35 = getelementptr inbounds nuw i8, ptr %_23.0, i64 92, !dbg !16383
  store i32 0, ptr %35, align 4, !dbg !16383, !alias.scope !16363, !noalias !16380
  %_7.i.i.5.i = icmp eq i64 %_23.1, 6, !dbg !16367
  br i1 %_7.i.i.5.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %bb7.6.i, !dbg !16377

bb7.6.i:                                          ; preds = %bb7.5.i
  %_17.i.i.5.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 96, !dbg !16384
  store float 0.000000e+00, ptr %_17.i.i.5.i, align 4, !dbg !16378, !alias.scope !16363, !noalias !16380
  %36 = getelementptr inbounds nuw i8, ptr %_23.0, i64 104, !dbg !16382
  store float 0.000000e+00, ptr %36, align 4, !dbg !16382, !alias.scope !16363, !noalias !16380
  %37 = getelementptr inbounds nuw i8, ptr %_23.0, i64 108, !dbg !16383
  store i32 0, ptr %37, align 4, !dbg !16383, !alias.scope !16363, !noalias !16380
  %_7.i.i.6.i = icmp eq i64 %_23.1, 7, !dbg !16367
  br i1 %_7.i.i.6.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %bb7.7.i, !dbg !16377

bb7.7.i:                                          ; preds = %bb7.6.i
  %_17.i.i.6.i = getelementptr inbounds nuw i8, ptr %_23.0, i64 112, !dbg !16384
  store float 0.000000e+00, ptr %_17.i.i.6.i, align 4, !dbg !16378, !alias.scope !16363, !noalias !16380
  %38 = getelementptr inbounds nuw i8, ptr %_23.0, i64 120, !dbg !16382
  store float 0.000000e+00, ptr %38, align 4, !dbg !16382, !alias.scope !16363, !noalias !16380
  %39 = getelementptr inbounds nuw i8, ptr %_23.0, i64 124, !dbg !16383
  store i32 0, ptr %39, align 4, !dbg !16383, !alias.scope !16363, !noalias !16380
  %_7.i.i.7.i = icmp eq i64 %_23.1, 8, !dbg !16367
  br i1 %_7.i.i.7.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, label %panic.i4, !dbg !16377

panic.i4:                                         ; preds = %bb7.7.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e7cf9526cbf9c5bb2e965885a8a18fa) #30, !dbg !16386, !noalias !16387
  unreachable, !dbg !16386

_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit, %bb7.preheader.i, %bb7.1.i, %bb7.2.i, %bb7.3.i, %bb7.4.i, %bb7.5.i, %bb7.6.i, %bb7.7.i
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 64, !dbg !16388
  %_15.sroa.0.0.copyload = load float, ptr %40, align 4, !dbg !16388
  %_15.sroa.435.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 72, !dbg !16388
  %_15.sroa.435.0.copyload = load float, ptr %_15.sroa.435.0..sroa_idx, align 4, !dbg !16388
  %_15.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 76, !dbg !16388
  %_15.sroa.5.0.copyload = load float, ptr %_15.sroa.5.0..sroa_idx, align 4, !dbg !16388
  %41 = getelementptr inbounds nuw i8, ptr %state, i64 144, !dbg !16389
  %_24.0 = load ptr, ptr %41, align 8, !dbg !16389, !nonnull !12, !noundef !12
  %42 = getelementptr inbounds nuw i8, ptr %state, i64 152, !dbg !16389
  %_24.1 = load i64, ptr %42, align 8, !dbg !16389, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16390), !dbg !16393
  %_7.i.i19.i5 = icmp eq i64 %_24.1, 0, !dbg !16394
  br i1 %_7.i.i19.i5, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %bb7.preheader.i6, !dbg !16399

bb7.preheader.i6:                                 ; preds = %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit
  store float %_15.sroa.0.0.copyload, ptr %_24.0, align 4, !dbg !16400, !alias.scope !16390, !noalias !16401
  %43 = getelementptr inbounds nuw i8, ptr %_24.0, i64 8, !dbg !16403
  store float %_15.sroa.435.0.copyload, ptr %43, align 4, !dbg !16403, !alias.scope !16390, !noalias !16401
  %44 = getelementptr inbounds nuw i8, ptr %_24.0, i64 12, !dbg !16404
  %45 = tail call i32 @llvm.fptoui.sat.i32.f32(float %_15.sroa.5.0.copyload), !dbg !16404
  store i32 %45, ptr %44, align 4, !dbg !16404, !alias.scope !16390, !noalias !16401
  %_7.i.i.i10 = icmp eq i64 %_24.1, 1, !dbg !16394
  br i1 %_7.i.i.i10, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %bb7.1.i11, !dbg !16399

bb7.1.i11:                                        ; preds = %bb7.preheader.i6
  %_17.i.i.i12 = getelementptr inbounds nuw i8, ptr %_24.0, i64 16, !dbg !16405
  store float 0.000000e+00, ptr %_17.i.i.i12, align 4, !dbg !16400, !alias.scope !16390, !noalias !16401
  %46 = getelementptr inbounds nuw i8, ptr %_24.0, i64 24, !dbg !16403
  store float 0.000000e+00, ptr %46, align 4, !dbg !16403, !alias.scope !16390, !noalias !16401
  %47 = getelementptr inbounds nuw i8, ptr %_24.0, i64 28, !dbg !16404
  store i32 0, ptr %47, align 4, !dbg !16404, !alias.scope !16390, !noalias !16401
  %_7.i.i.1.i13 = icmp eq i64 %_24.1, 2, !dbg !16394
  br i1 %_7.i.i.1.i13, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %bb7.2.i14, !dbg !16399

bb7.2.i14:                                        ; preds = %bb7.1.i11
  %_17.i.i.1.i15 = getelementptr inbounds nuw i8, ptr %_24.0, i64 32, !dbg !16405
  store float 0.000000e+00, ptr %_17.i.i.1.i15, align 4, !dbg !16400, !alias.scope !16390, !noalias !16401
  %48 = getelementptr inbounds nuw i8, ptr %_24.0, i64 40, !dbg !16403
  store float 0.000000e+00, ptr %48, align 4, !dbg !16403, !alias.scope !16390, !noalias !16401
  %49 = getelementptr inbounds nuw i8, ptr %_24.0, i64 44, !dbg !16404
  store i32 0, ptr %49, align 4, !dbg !16404, !alias.scope !16390, !noalias !16401
  %_7.i.i.2.i16 = icmp eq i64 %_24.1, 3, !dbg !16394
  br i1 %_7.i.i.2.i16, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %bb7.3.i17, !dbg !16399

bb7.3.i17:                                        ; preds = %bb7.2.i14
  %_17.i.i.2.i18 = getelementptr inbounds nuw i8, ptr %_24.0, i64 48, !dbg !16405
  store float 0.000000e+00, ptr %_17.i.i.2.i18, align 4, !dbg !16400, !alias.scope !16390, !noalias !16401
  %50 = getelementptr inbounds nuw i8, ptr %_24.0, i64 56, !dbg !16403
  store float 0.000000e+00, ptr %50, align 4, !dbg !16403, !alias.scope !16390, !noalias !16401
  %51 = getelementptr inbounds nuw i8, ptr %_24.0, i64 60, !dbg !16404
  store i32 0, ptr %51, align 4, !dbg !16404, !alias.scope !16390, !noalias !16401
  %_7.i.i.3.i19 = icmp eq i64 %_24.1, 4, !dbg !16394
  br i1 %_7.i.i.3.i19, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %bb7.4.i20, !dbg !16399

bb7.4.i20:                                        ; preds = %bb7.3.i17
  %_17.i.i.3.i21 = getelementptr inbounds nuw i8, ptr %_24.0, i64 64, !dbg !16405
  store float 0.000000e+00, ptr %_17.i.i.3.i21, align 4, !dbg !16400, !alias.scope !16390, !noalias !16401
  %52 = getelementptr inbounds nuw i8, ptr %_24.0, i64 72, !dbg !16403
  store float 0.000000e+00, ptr %52, align 4, !dbg !16403, !alias.scope !16390, !noalias !16401
  %53 = getelementptr inbounds nuw i8, ptr %_24.0, i64 76, !dbg !16404
  store i32 0, ptr %53, align 4, !dbg !16404, !alias.scope !16390, !noalias !16401
  %_7.i.i.4.i22 = icmp eq i64 %_24.1, 5, !dbg !16394
  br i1 %_7.i.i.4.i22, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %bb7.5.i23, !dbg !16399

bb7.5.i23:                                        ; preds = %bb7.4.i20
  %_17.i.i.4.i24 = getelementptr inbounds nuw i8, ptr %_24.0, i64 80, !dbg !16405
  store float 0.000000e+00, ptr %_17.i.i.4.i24, align 4, !dbg !16400, !alias.scope !16390, !noalias !16401
  %54 = getelementptr inbounds nuw i8, ptr %_24.0, i64 88, !dbg !16403
  store float 0.000000e+00, ptr %54, align 4, !dbg !16403, !alias.scope !16390, !noalias !16401
  %55 = getelementptr inbounds nuw i8, ptr %_24.0, i64 92, !dbg !16404
  store i32 0, ptr %55, align 4, !dbg !16404, !alias.scope !16390, !noalias !16401
  %_7.i.i.5.i25 = icmp eq i64 %_24.1, 6, !dbg !16394
  br i1 %_7.i.i.5.i25, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %bb7.6.i26, !dbg !16399

bb7.6.i26:                                        ; preds = %bb7.5.i23
  %_17.i.i.5.i27 = getelementptr inbounds nuw i8, ptr %_24.0, i64 96, !dbg !16405
  store float 0.000000e+00, ptr %_17.i.i.5.i27, align 4, !dbg !16400, !alias.scope !16390, !noalias !16401
  %56 = getelementptr inbounds nuw i8, ptr %_24.0, i64 104, !dbg !16403
  store float 0.000000e+00, ptr %56, align 4, !dbg !16403, !alias.scope !16390, !noalias !16401
  %57 = getelementptr inbounds nuw i8, ptr %_24.0, i64 108, !dbg !16404
  store i32 0, ptr %57, align 4, !dbg !16404, !alias.scope !16390, !noalias !16401
  %_7.i.i.6.i28 = icmp eq i64 %_24.1, 7, !dbg !16394
  br i1 %_7.i.i.6.i28, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %bb7.7.i29, !dbg !16399

bb7.7.i29:                                        ; preds = %bb7.6.i26
  %_17.i.i.6.i30 = getelementptr inbounds nuw i8, ptr %_24.0, i64 112, !dbg !16405
  store float 0.000000e+00, ptr %_17.i.i.6.i30, align 4, !dbg !16400, !alias.scope !16390, !noalias !16401
  %58 = getelementptr inbounds nuw i8, ptr %_24.0, i64 120, !dbg !16403
  store float 0.000000e+00, ptr %58, align 4, !dbg !16403, !alias.scope !16390, !noalias !16401
  %59 = getelementptr inbounds nuw i8, ptr %_24.0, i64 124, !dbg !16404
  store i32 0, ptr %59, align 4, !dbg !16404, !alias.scope !16390, !noalias !16401
  %_7.i.i.7.i31 = icmp eq i64 %_24.1, 8, !dbg !16394
  br i1 %_7.i.i.7.i31, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33, label %panic.i32, !dbg !16399

panic.i32:                                        ; preds = %bb7.7.i29
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e7cf9526cbf9c5bb2e965885a8a18fa) #30, !dbg !16407, !noalias !16408
  unreachable, !dbg !16407

_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit33: ; preds = %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE7scatterB5_.exit, %bb7.preheader.i6, %bb7.1.i11, %bb7.2.i14, %bb7.3.i17, %bb7.4.i20, %bb7.5.i23, %bb7.6.i26, %bb7.7.i29
  ret void, !dbg !16409
}
