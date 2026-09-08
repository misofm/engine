define internal fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelfE4loadB5_(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 4 captures(none) dereferenceable(92) %_0, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(200) %state) unnamed_addr #1 personality ptr @rust_eh_personality !dbg !15796 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15797), !dbg !15800
  %0 = getelementptr inbounds nuw i8, ptr %state, i64 192, !dbg !15801
  %1 = load i64, ptr %0, align 8, !dbg !15801, !alias.scope !15797, !noalias !15804, !noundef !12
  %_31.0.i = load ptr, ptr %state, align 8, !dbg !15806, !alias.scope !15797, !noalias !15804, !nonnull !12, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %state, i64 8, !dbg !15806
  %_31.1.i = load i64, ptr %2, align 8, !dbg !15806, !alias.scope !15797, !noalias !15804, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15808), !dbg !15811
  %_3.not.i.i.i = icmp eq i64 %_31.1.i, 0, !dbg !15815
  br i1 %_3.not.i.i.i, label %panic.i.i.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit.i, !dbg !15815

panic.i.i.i:                                      ; preds = %start
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15815, !noalias !15821
  unreachable, !dbg !15815

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit.i: ; preds = %start
  %_0.i.i.i = load float, ptr %_31.0.i, align 4, !dbg !15815, !alias.scope !15808, !noalias !15824, !noundef !12
  %_8.i.i = icmp ugt i64 %1, %_31.1.i, !dbg !15825
  br i1 %_8.i.i, label %bb2.i.i, label %bb3.i.i, !dbg !15825, !prof !639

bb3.i.i:                                          ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15831), !dbg !15834
  %_3.not.i.i5.i = icmp eq i64 %_31.1.i, %1, !dbg !15835
  br i1 %_3.not.i.i5.i, label %panic.i.i7.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit8.i, !dbg !15835

panic.i.i7.i:                                     ; preds = %bb3.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15835, !noalias !15837
  unreachable, !dbg !15835

bb2.i.i:                                          ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %1, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15840, !noalias !15841
  unreachable, !dbg !15840

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit8.i: ; preds = %bb3.i.i
  %_15.i.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %1, !dbg !15842
  %_0.i.i6.i = load float, ptr %_15.i.i, align 4, !dbg !15835, !alias.scope !15831, !noalias !15841, !noundef !12
  %_4.i.i = shl i64 %1, 1, !dbg !15847
  %_8.i12.i = icmp ugt i64 %_4.i.i, %_31.1.i, !dbg !15849
  br i1 %_8.i12.i, label %bb2.i19.i, label %bb3.i13.i, !dbg !15849, !prof !639

bb3.i13.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit8.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15852), !dbg !15855
  %_3.not.i.i15.i = icmp eq i64 %_31.1.i, %_4.i.i, !dbg !15856
  br i1 %_3.not.i.i15.i, label %panic.i.i18.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit20.i, !dbg !15856

panic.i.i18.i:                                    ; preds = %bb3.i13.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15856, !noalias !15858
  unreachable, !dbg !15856

bb2.i19.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit8.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15861, !noalias !15862
  unreachable, !dbg !15861

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit20.i: ; preds = %bb3.i13.i
  %_15.i16.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i.i, !dbg !15863
  %_0.i.i17.i = load float, ptr %_15.i16.i, align 4, !dbg !15856, !alias.scope !15852, !noalias !15862, !noundef !12
  %_4.i24.i = mul i64 %1, 3, !dbg !15865
  %_8.i25.i = icmp ugt i64 %_4.i24.i, %_31.1.i, !dbg !15867
  br i1 %_8.i25.i, label %bb2.i32.i, label %bb3.i26.i, !dbg !15867, !prof !639

bb3.i26.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit20.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15870), !dbg !15873
  %_3.not.i.i28.i = icmp eq i64 %_31.1.i, %_4.i24.i, !dbg !15874
  br i1 %_3.not.i.i28.i, label %panic.i.i31.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit33.i, !dbg !15874

panic.i.i31.i:                                    ; preds = %bb3.i26.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15874, !noalias !15876
  unreachable, !dbg !15874

bb2.i32.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i24.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15879, !noalias !15880
  unreachable, !dbg !15879

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit33.i: ; preds = %bb3.i26.i
  %_15.i29.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i24.i, !dbg !15881
  %_0.i.i30.i = load float, ptr %_15.i29.i, align 4, !dbg !15874, !alias.scope !15870, !noalias !15880, !noundef !12
  %_4.i37.i = shl i64 %1, 2, !dbg !15883
  %_8.i38.i = icmp ugt i64 %_4.i37.i, %_31.1.i, !dbg !15885
  br i1 %_8.i38.i, label %bb2.i45.i, label %bb3.i39.i, !dbg !15885, !prof !639

bb3.i39.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit33.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15888), !dbg !15891
  %_3.not.i.i41.i = icmp eq i64 %_31.1.i, %_4.i37.i, !dbg !15892
  br i1 %_3.not.i.i41.i, label %panic.i.i44.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit46.i, !dbg !15892

panic.i.i44.i:                                    ; preds = %bb3.i39.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15892, !noalias !15894
  unreachable, !dbg !15892

bb2.i45.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit33.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i37.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15897, !noalias !15898
  unreachable, !dbg !15897

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit46.i: ; preds = %bb3.i39.i
  %_15.i42.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i37.i, !dbg !15899
  %_0.i.i43.i = load float, ptr %_15.i42.i, align 4, !dbg !15892, !alias.scope !15888, !noalias !15898, !noundef !12
  %_4.i50.i = mul i64 %1, 5, !dbg !15901
  %_8.i51.i = icmp ugt i64 %_4.i50.i, %_31.1.i, !dbg !15903
  br i1 %_8.i51.i, label %bb2.i58.i, label %bb3.i52.i, !dbg !15903, !prof !639

bb3.i52.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit46.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15906), !dbg !15909
  %_3.not.i.i54.i = icmp eq i64 %_31.1.i, %_4.i50.i, !dbg !15910
  br i1 %_3.not.i.i54.i, label %panic.i.i57.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit59.i, !dbg !15910

panic.i.i57.i:                                    ; preds = %bb3.i52.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15910, !noalias !15912
  unreachable, !dbg !15910

bb2.i58.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit46.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i50.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15915, !noalias !15916
  unreachable, !dbg !15915

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit59.i: ; preds = %bb3.i52.i
  %_15.i55.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i50.i, !dbg !15917
  %_0.i.i56.i = load float, ptr %_15.i55.i, align 4, !dbg !15910, !alias.scope !15906, !noalias !15916, !noundef !12
  %_4.i63.i = mul i64 %1, 6, !dbg !15919
  %_8.i64.i = icmp ugt i64 %_4.i63.i, %_31.1.i, !dbg !15921
  br i1 %_8.i64.i, label %bb2.i71.i, label %bb3.i65.i, !dbg !15921, !prof !639

bb3.i65.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit59.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15924), !dbg !15927
  %_3.not.i.i67.i = icmp eq i64 %_31.1.i, %_4.i63.i, !dbg !15928
  br i1 %_3.not.i.i67.i, label %panic.i.i70.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit72.i, !dbg !15928

panic.i.i70.i:                                    ; preds = %bb3.i65.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15928, !noalias !15930
  unreachable, !dbg !15928

bb2.i71.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit59.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i63.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15933, !noalias !15934
  unreachable, !dbg !15933

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit72.i: ; preds = %bb3.i65.i
  %_15.i68.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i63.i, !dbg !15935
  %_0.i.i69.i = load float, ptr %_15.i68.i, align 4, !dbg !15928, !alias.scope !15924, !noalias !15934, !noundef !12
  %_4.i76.i = mul i64 %1, 7, !dbg !15937
  %_8.i77.i = icmp ugt i64 %_4.i76.i, %_31.1.i, !dbg !15939
  br i1 %_8.i77.i, label %bb2.i84.i, label %bb3.i78.i, !dbg !15939, !prof !639

bb3.i78.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit72.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15942), !dbg !15945
  %_3.not.i.i80.i = icmp eq i64 %_31.1.i, %_4.i76.i, !dbg !15946
  br i1 %_3.not.i.i80.i, label %panic.i.i83.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit85.i, !dbg !15946

panic.i.i83.i:                                    ; preds = %bb3.i78.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15946, !noalias !15948
  unreachable, !dbg !15946

bb2.i84.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit72.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i76.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15951, !noalias !15952
  unreachable, !dbg !15951

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit85.i: ; preds = %bb3.i78.i
  %_15.i81.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i76.i, !dbg !15953
  %_0.i.i82.i = load float, ptr %_15.i81.i, align 4, !dbg !15946, !alias.scope !15942, !noalias !15952, !noundef !12
  %_4.i89.i = shl i64 %1, 3, !dbg !15955
  %_8.i90.i = icmp ugt i64 %_4.i89.i, %_31.1.i, !dbg !15957
  br i1 %_8.i90.i, label %bb2.i97.i, label %bb3.i91.i, !dbg !15957, !prof !639

bb3.i91.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit85.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15960), !dbg !15963
  %_3.not.i.i93.i = icmp eq i64 %_31.1.i, %_4.i89.i, !dbg !15964
  br i1 %_3.not.i.i93.i, label %panic.i.i96.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit98.i, !dbg !15964

panic.i.i96.i:                                    ; preds = %bb3.i91.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15964, !noalias !15966
  unreachable, !dbg !15964

bb2.i97.i:                                        ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit85.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i89.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15969, !noalias !15970
  unreachable, !dbg !15969

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit98.i: ; preds = %bb3.i91.i
  %_15.i94.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i89.i, !dbg !15971
  %_0.i.i95.i = load float, ptr %_15.i94.i, align 4, !dbg !15964, !alias.scope !15960, !noalias !15970, !noundef !12
  %_4.i102.i = mul i64 %1, 9, !dbg !15973
  %_8.i103.i = icmp ugt i64 %_4.i102.i, %_31.1.i, !dbg !15975
  br i1 %_8.i103.i, label %bb2.i110.i, label %bb3.i104.i, !dbg !15975, !prof !639

bb3.i104.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit98.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15978), !dbg !15981
  %_3.not.i.i106.i = icmp eq i64 %_31.1.i, %_4.i102.i, !dbg !15982
  br i1 %_3.not.i.i106.i, label %panic.i.i109.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit111.i, !dbg !15982

panic.i.i109.i:                                   ; preds = %bb3.i104.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !15982, !noalias !15984
  unreachable, !dbg !15982

bb2.i110.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit98.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i102.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !15987, !noalias !15988
  unreachable, !dbg !15987

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit111.i: ; preds = %bb3.i104.i
  %_15.i107.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i102.i, !dbg !15989
  %_0.i.i108.i = load float, ptr %_15.i107.i, align 4, !dbg !15982, !alias.scope !15978, !noalias !15988, !noundef !12
  %_4.i115.i = mul i64 %1, 10, !dbg !15991
  %_8.i116.i = icmp ugt i64 %_4.i115.i, %_31.1.i, !dbg !15993
  br i1 %_8.i116.i, label %bb2.i123.i, label %bb3.i117.i, !dbg !15993, !prof !639

bb3.i117.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit111.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !15996), !dbg !15999
  %_3.not.i.i119.i = icmp eq i64 %_31.1.i, %_4.i115.i, !dbg !16000
  br i1 %_3.not.i.i119.i, label %panic.i.i122.i, label %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit124.i, !dbg !16000

panic.i.i122.i:                                   ; preds = %bb3.i117.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !16000, !noalias !16002
  unreachable, !dbg !16000

bb2.i123.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit111.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i115.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !16005, !noalias !16006
  unreachable, !dbg !16005

_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit124.i: ; preds = %bb3.i117.i
  %_15.i120.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i115.i, !dbg !16007
  %_0.i.i121.i = load float, ptr %_15.i120.i, align 4, !dbg !16000, !alias.scope !15996, !noalias !16006, !noundef !12
  %_4.i128.i = mul i64 %1, 11, !dbg !16009
  %_8.i129.i = icmp ugt i64 %_4.i128.i, %_31.1.i, !dbg !16011
  br i1 %_8.i129.i, label %bb2.i136.i, label %bb3.i130.i, !dbg !16011, !prof !639

bb3.i130.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit124.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16014), !dbg !16017
  %_3.not.i.i132.i = icmp eq i64 %_31.1.i, %_4.i128.i, !dbg !16018
  br i1 %_3.not.i.i132.i, label %panic.i.i135.i, label %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE4loadB5_.exit, !dbg !16018

panic.i.i135.i:                                   ; preds = %bb3.i130.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !16018, !noalias !16020
  unreachable, !dbg !16018

bb2.i136.i:                                       ; preds = %_RNCNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB7_7HistoryfE4load0B7_.exit124.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_4.i128.i, i64 noundef %_31.1.i, i64 noundef %_31.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7f43518a494eccc88ee423e0d076b927) #30, !dbg !16023, !noalias !16024
  unreachable, !dbg !16023

_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE4loadB5_.exit: ; preds = %bb3.i130.i
  %_15.i133.i = getelementptr inbounds nuw float, ptr %_31.0.i, i64 %_4.i128.i, !dbg !16025
  %_0.i.i134.i = load float, ptr %_15.i133.i, align 4, !dbg !16018, !alias.scope !16014, !noalias !16024, !noundef !12
  %3 = getelementptr inbounds nuw i8, ptr %state, i64 184, !dbg !16027
  %_39.1 = load i64, ptr %3, align 8, !dbg !16027, !noundef !12
  %_6.i.i30 = icmp eq i64 %_39.1, 0, !dbg !16030
  br i1 %_6.i.i30, label %bb6, label %bb5.preheader, !dbg !16035

bb5.preheader:                                    ; preds = %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE4loadB5_.exit
  %4 = getelementptr inbounds nuw i8, ptr %state, i64 176, !dbg !16027
  %_39.0 = load ptr, ptr %4, align 8, !dbg !16027, !nonnull !12, !noundef !12
  %_12 = load i32, ptr %_39.0, align 4, !dbg !16036, !noundef !12
  %5 = uitofp i32 %_12 to float, !dbg !16038
  %switch = icmp ult i64 %_39.1, 9, !dbg !16035
  br i1 %switch, label %bb6, label %panic, !dbg !16035

bb6:                                              ; preds = %bb5.preheader, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE4loadB5_.exit
  %window.sroa.0.0 = phi float [ 0.000000e+00, %_RNvMs4_CsdvPQf9CMsz3_17true_peak_limiterINtB5_7HistoryfE4loadB5_.exit ], [ %5, %bb5.preheader ]
  %6 = getelementptr inbounds nuw i8, ptr %state, i64 64, !dbg !16039
  %_40.0 = load ptr, ptr %6, align 8, !dbg !16039, !nonnull !12, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %state, i64 72, !dbg !16039
  %_40.1 = load i64, ptr %7, align 8, !dbg !16039, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16040), !dbg !16043
  %_3.not.i4 = icmp eq i64 %_40.1, 0, !dbg !16044
  br i1 %_3.not.i4, label %panic.i6, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit7, !dbg !16044

panic.i6:                                         ; preds = %bb6
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !16044, !noalias !16040
  unreachable, !dbg !16044

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit7: ; preds = %bb6
  %_0.i5 = load float, ptr %_40.0, align 4, !dbg !16044, !alias.scope !16040, !noundef !12
  %8 = getelementptr inbounds nuw i8, ptr %state, i64 96, !dbg !16046
  %_41.0 = load ptr, ptr %8, align 8, !dbg !16046, !nonnull !12, !noundef !12
  %9 = getelementptr inbounds nuw i8, ptr %state, i64 104, !dbg !16046
  %_41.1 = load i64, ptr %9, align 8, !dbg !16046, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16047), !dbg !16050
  %_3.not.i = icmp eq i64 %_41.1, 0, !dbg !16051
  br i1 %_3.not.i, label %panic.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit, !dbg !16051

panic.i:                                          ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit7
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #30, !dbg !16051, !noalias !16047
  unreachable, !dbg !16051

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit7
  %_0.i3 = load float, ptr %_41.0, align 4, !dbg !16051, !alias.scope !16047, !noundef !12
  %10 = getelementptr inbounds nuw i8, ptr %state, i64 128, !dbg !16053
  %_42.0 = load ptr, ptr %10, align 8, !dbg !16053, !nonnull !12, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %state, i64 136, !dbg !16053
  %_42.1 = load i64, ptr %11, align 8, !dbg !16053, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16054), !dbg !16057
  %_6.i.i27.i = icmp eq i64 %_42.1, 0, !dbg !16058
  br i1 %_6.i.i27.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit, label %bb4.preheader.i, !dbg !16069

bb4.preheader.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit
  %_14.i = load float, ptr %_42.0, align 4, !dbg !16070, !alias.scope !16054, !noalias !16072, !noundef !12
  %12 = getelementptr inbounds nuw i8, ptr %_42.0, i64 4, !dbg !16074
  %_16.i = load float, ptr %12, align 4, !dbg !16074, !alias.scope !16054, !noalias !16072, !noundef !12
  %13 = getelementptr inbounds nuw i8, ptr %_42.0, i64 8, !dbg !16075
  %_17.i = load float, ptr %13, align 4, !dbg !16075, !alias.scope !16054, !noalias !16072, !noundef !12
  %14 = getelementptr inbounds nuw i8, ptr %_42.0, i64 12, !dbg !16076
  %_20.i = load i32, ptr %14, align 4, !dbg !16076, !alias.scope !16054, !noalias !16072, !noundef !12
  %_19.i = trunc i32 %_20.i to i16, !dbg !16076
  %_18.i = uitofp i16 %_19.i to float, !dbg !16077
  %switch.i = icmp samesign ult i64 %_42.1, 9, !dbg !16069
  br i1 %switch.i, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit, label %panic.i8, !dbg !16069

panic.i8:                                         ; preds = %bb4.preheader.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_34ec9deef3d1165c325897b5845d9fe1) #30, !dbg !16080, !noalias !16081
  unreachable, !dbg !16080

_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit, %bb4.preheader.i
  %remaining.sroa.0.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], [ %_18.i, %bb4.preheader.i ]
  %step.sroa.0.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], [ %_17.i, %bb4.preheader.i ]
  %target.sroa.0.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], [ %_16.i, %bb4.preheader.i ]
  %current.sroa.0.0.i = phi float [ 0.000000e+00, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit ], [ %_14.i, %bb4.preheader.i ]
  %15 = getelementptr inbounds nuw i8, ptr %state, i64 144, !dbg !16082
  %_43.0 = load ptr, ptr %15, align 8, !dbg !16082, !nonnull !12, !noundef !12
  %16 = getelementptr inbounds nuw i8, ptr %state, i64 152, !dbg !16082
  %_43.1 = load i64, ptr %16, align 8, !dbg !16082, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16083), !dbg !16086
  %_6.i.i27.i9 = icmp eq i64 %_43.1, 0, !dbg !16087
  br i1 %_6.i.i27.i9, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit23, label %bb4.preheader.i10, !dbg !16092

bb4.preheader.i10:                                ; preds = %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit
  %_14.i11 = load float, ptr %_43.0, align 4, !dbg !16093, !alias.scope !16083, !noalias !16094, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %_43.0, i64 4, !dbg !16096
  %_16.i12 = load float, ptr %17, align 4, !dbg !16096, !alias.scope !16083, !noalias !16094, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %_43.0, i64 8, !dbg !16097
  %_17.i13 = load float, ptr %18, align 4, !dbg !16097, !alias.scope !16083, !noalias !16094, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %_43.0, i64 12, !dbg !16098
  %_20.i14 = load i32, ptr %19, align 4, !dbg !16098, !alias.scope !16083, !noalias !16094, !noundef !12
  %_19.i15 = trunc i32 %_20.i14 to i16, !dbg !16098
  %_18.i16 = uitofp i16 %_19.i15 to float, !dbg !16099
  %switch.i17 = icmp samesign ult i64 %_43.1, 9, !dbg !16092
  br i1 %switch.i17, label %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit23, label %panic.i18, !dbg !16092

panic.i18:                                        ; preds = %bb4.preheader.i10
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_34ec9deef3d1165c325897b5845d9fe1) #30, !dbg !16101, !noalias !16102
  unreachable, !dbg !16101

_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit23: ; preds = %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit, %bb4.preheader.i10
  %remaining.sroa.0.0.i19 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit ], [ %_18.i16, %bb4.preheader.i10 ]
  %step.sroa.0.0.i20 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit ], [ %_17.i13, %bb4.preheader.i10 ]
  %target.sroa.0.0.i21 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit ], [ %_16.i12, %bb4.preheader.i10 ]
  %current.sroa.0.0.i22 = phi float [ 0.000000e+00, %_RNvMs3_CsdvPQf9CMsz3_17true_peak_limiterINtB5_9RampLanesfE6gatherB5_.exit ], [ %_14.i11, %bb4.preheader.i10 ]
  store float %_0.i.i.i, ptr %_0, align 4, !dbg !16103
  %history.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 4, !dbg !16103
  store float %_0.i.i6.i, ptr %history.sroa.4.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8, !dbg !16103
  store float %_0.i.i17.i, ptr %history.sroa.5.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 12, !dbg !16103
  store float %_0.i.i30.i, ptr %history.sroa.6.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16, !dbg !16103
  store float %_0.i.i43.i, ptr %history.sroa.7.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.8.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 20, !dbg !16103
  store float %_0.i.i56.i, ptr %history.sroa.8.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.9.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24, !dbg !16103
  store float %_0.i.i69.i, ptr %history.sroa.9.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.10.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 28, !dbg !16103
  store float %_0.i.i82.i, ptr %history.sroa.10.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.11.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32, !dbg !16103
  store float %_0.i.i95.i, ptr %history.sroa.11.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.12.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 36, !dbg !16103
  store float %_0.i.i108.i, ptr %history.sroa.12.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.13.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40, !dbg !16103
  store float %_0.i.i121.i, ptr %history.sroa.13.0._0.sroa_idx, align 4, !dbg !16103
  %history.sroa.14.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 44, !dbg !16103
  store float %_0.i.i134.i, ptr %history.sroa.14.0._0.sroa_idx, align 4, !dbg !16103
  %20 = getelementptr inbounds nuw i8, ptr %_0, i64 80, !dbg !16103
  store float %_0.i5, ptr %20, align 4, !dbg !16103
  %21 = getelementptr inbounds nuw i8, ptr %_0, i64 84, !dbg !16103
  store float %_0.i3, ptr %21, align 4, !dbg !16103
  %22 = getelementptr inbounds nuw i8, ptr %_0, i64 88, !dbg !16103
  store float %window.sroa.0.0, ptr %22, align 4, !dbg !16103
  %23 = getelementptr inbounds nuw i8, ptr %_0, i64 48, !dbg !16103
  store float %current.sroa.0.0.i, ptr %23, align 4, !dbg !16103
  %_21.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 52, !dbg !16103
  store float %target.sroa.0.0.i, ptr %_21.sroa.4.0..sroa_idx, align 4, !dbg !16103
  %_21.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 56, !dbg !16103
  store float %step.sroa.0.0.i, ptr %_21.sroa.5.0..sroa_idx, align 4, !dbg !16103
  %_21.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 60, !dbg !16103
  store float %remaining.sroa.0.0.i, ptr %_21.sroa.6.0..sroa_idx, align 4, !dbg !16103
  %24 = getelementptr inbounds nuw i8, ptr %_0, i64 64, !dbg !16103
  store float %current.sroa.0.0.i22, ptr %24, align 4, !dbg !16103
  %_23.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 68, !dbg !16103
  store float %target.sroa.0.0.i21, ptr %_23.sroa.4.0..sroa_idx, align 4, !dbg !16103
  %_23.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 72, !dbg !16103
  store float %step.sroa.0.0.i20, ptr %_23.sroa.5.0..sroa_idx, align 4, !dbg !16103
  %_23.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 76, !dbg !16103
  store float %remaining.sroa.0.0.i19, ptr %_23.sroa.6.0..sroa_idx, align 4, !dbg !16103
  ret void, !dbg !16104

panic:                                            ; preds = %bb5.preheader
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 8, i64 noundef 8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3862c97910d0689d8b59a4255cb98beb) #30, !dbg !16038
  unreachable, !dbg !16038
}
