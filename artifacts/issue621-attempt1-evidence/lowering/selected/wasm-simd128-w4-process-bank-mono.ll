define internal void @_RNvXse_CsjLJhryqjeDL_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ENtCs2mr8MC2dYJN_15effect_contract24PreparedNativeEffectBank17process_bank_monoB5_(ptr dead_on_unwind noalias noundef writable writeonly sret([328 x i8]) align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef align 16 dereferenceable(1248) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(64) %block) unnamed_addr #0 !dbg !26232 {
start:
  %report.i = alloca [328 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26233), !dbg !26236
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26237), !dbg !26236
  %0 = getelementptr inbounds nuw i8, ptr %block, i32 16, !dbg !26239
  %_37.0.i = load ptr, ptr %0, align 8, !dbg !26239, !alias.scope !26237, !noalias !26242, !nonnull !10, !align !25010, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %block, i32 20, !dbg !26239
  %_37.1.i = load i32, ptr %1, align 4, !dbg !26239, !alias.scope !26237, !noalias !26242, !noundef !10
  %2 = icmp eq i32 %_37.1.i, 0, !dbg !26239
  br i1 %2, label %bb2.i, label %bb1.i, !dbg !26239

bb2.i:                                            ; preds = %bb1.i, %start
  call void @llvm.lifetime.start.p0(ptr nonnull %report.i), !dbg !26244, !noalias !26245
  %3 = getelementptr inbounds nuw i8, ptr %self, i32 1232, !dbg !26246
  %4 = load i8, ptr %3, align 16, !dbg !26246, !range !4667, !alias.scope !26233, !noalias !26247, !noundef !10
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 8 dereferenceable(320) %report.i, i8 0, i32 320, i1 false), !noalias !26245
  %5 = getelementptr inbounds nuw i8, ptr %report.i, i32 320, !dbg !26248
  store i8 %4, ptr %5, align 8, !dbg !26248, !noalias !26245
  %6 = getelementptr inbounds nuw i8, ptr %block, i32 24
  %_38.0.i = load ptr, ptr %6, align 8, !alias.scope !26237, !noalias !26242, !nonnull !10, !align !10173, !noundef !10
  %7 = getelementptr inbounds nuw i8, ptr %block, i32 28
  %_38.1.i = load i32, ptr %7, align 4, !alias.scope !26237, !noalias !26242, !noundef !10
  %_26.i = getelementptr inbounds nuw i8, ptr %self, i32 1024
  %_25.i = getelementptr inbounds nuw i8, ptr %self, i32 924
  %8 = getelementptr inbounds nuw i8, ptr %block, i32 48
  %_24.i = load i64, ptr %8, align 8, !alias.scope !26237, !noalias !26242
  %_23.i = getelementptr inbounds nuw i8, ptr %self, i32 824
  %9 = tail call i32 @llvm.usub.sat.i32(i32 %_38.1.i, i32 1), !dbg !26251
  %exitcond.not.i = icmp eq i32 %_38.1.i, 0, !dbg !26259
  br i1 %exitcond.not.i, label %panic.i, label %bb4.i, !dbg !26259

bb1.i:                                            ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 1124, !dbg !26261
  store i8 0, ptr %10, align 4, !dbg !26261, !alias.scope !26233, !noalias !26247
  br label %bb2.i, !dbg !26262

bb4.i:                                            ; preds = %bb2.i
  %_14.i = load i32, ptr %_38.0.i, align 4, !dbg !26259, !noalias !26245, !noundef !10
  %exitcond23.not.i = icmp eq i32 %_38.1.i, 1, !dbg !26263
  br i1 %exitcond23.not.i, label %panic1.i, label %bb5.i, !dbg !26263

panic.i:                                          ; preds = %bb6.2.i, %bb6.1.i, %bb2.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_38.1.i, i32 noundef %_38.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_297cd72dee4462ad27dbe24f13722ef7) #32, !dbg !26259, !noalias !26245
  unreachable, !dbg !26259

bb5.i:                                            ; preds = %bb4.i
  %11 = getelementptr inbounds nuw i8, ptr %_38.0.i, i32 4, !dbg !26263
  %_18.i = load i32, ptr %11, align 4, !dbg !26263, !noalias !26245, !noundef !10
  %_53.i = icmp ult i32 %_18.i, %_14.i, !dbg !26265
  %_49.not.i = icmp ugt i32 %_18.i, %_37.1.i
  %or.cond.i = or i1 %_53.i, %_49.not.i, !dbg !26265
  br i1 %or.cond.i, label %bb16.i, label %bb4.1.i, !dbg !26265, !prof !4596

panic1.i:                                         ; preds = %bb4.3.i, %bb4.2.i, %bb4.1.i, %bb4.i
  %.lcssa14.i = phi i32 [ 1, %bb4.i ], [ 2, %bb4.1.i ], [ 3, %bb4.2.i ], [ 4, %bb4.3.i ], !dbg !26273
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %.lcssa14.i, i32 noundef %_38.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_f03666103170d1da5fd4470c4d946344) #32, !dbg !26263, !noalias !26245
  unreachable, !dbg !26263

bb16.i:                                           ; preds = %bb5.3.i, %bb5.2.i, %bb5.1.i, %bb5.i
  %_18.lcssa.i = phi i32 [ %_18.i, %bb5.i ], [ %_18.1.i, %bb5.1.i ], [ %_18.2.i, %bb5.2.i ], [ %_18.3.i, %bb5.3.i ], !dbg !26263
  %_14.lcssa20.i = phi i32 [ %_14.i, %bb5.i ], [ %_14.1.i, %bb5.1.i ], [ %_14.2.i, %bb5.2.i ], [ %_14.3.i, %bb5.3.i ], !dbg !26259
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_14.lcssa20.i, i32 noundef %_18.lcssa.i, i32 noundef %_37.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_20d21e0032ad9c6108b3daa26f0ec9a2) #32, !dbg !26279, !noalias !26245
  unreachable, !dbg !26279

bb4.1.i:                                          ; preds = %bb5.i
  %_54.i = sub nuw i32 %_18.i, %_14.i, !dbg !26280
  %_56.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i32 %_14.i, !dbg !26281
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.i, i32 noundef %_54.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_25.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_26.i, i32 noundef 0, ptr noalias noundef nonnull align 8 dereferenceable(40) %report.i) #31, !dbg !26285, !noalias !26247
  %_14.1.i = load i32, ptr %11, align 4, !dbg !26259, !noalias !26245, !noundef !10
  %exitcond23.1.not.i = icmp eq i32 %9, 1, !dbg !26263
  br i1 %exitcond23.1.not.i, label %panic1.i, label %bb5.1.i, !dbg !26263

bb5.1.i:                                          ; preds = %bb4.1.i
  %12 = getelementptr inbounds nuw i8, ptr %_38.0.i, i32 8, !dbg !26263
  %_18.1.i = load i32, ptr %12, align 4, !dbg !26263, !noalias !26245, !noundef !10
  %_53.1.i = icmp ult i32 %_18.1.i, %_14.1.i, !dbg !26265
  %_49.not.1.i = icmp ugt i32 %_18.1.i, %_37.1.i
  %or.cond.1.i = or i1 %_53.1.i, %_49.not.1.i, !dbg !26265
  br i1 %or.cond.1.i, label %bb16.i, label %bb6.1.i, !dbg !26265, !prof !4596

bb6.1.i:                                          ; preds = %bb5.1.i
  %_54.1.i = sub nuw i32 %_18.1.i, %_14.1.i, !dbg !26280
  %_56.1.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i32 %_14.1.i, !dbg !26281
  %_27.1.i = getelementptr inbounds nuw i8, ptr %report.i, i32 40, !dbg !26286
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.1.i, i32 noundef %_54.1.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_25.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_26.i, i32 noundef 1, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.1.i) #31, !dbg !26285, !noalias !26247
  %exitcond.2.not.i = icmp eq i32 %_38.1.i, 2, !dbg !26259
  br i1 %exitcond.2.not.i, label %panic.i, label %bb4.2.i, !dbg !26259

bb4.2.i:                                          ; preds = %bb6.1.i
  %_14.2.i = load i32, ptr %12, align 4, !dbg !26259, !noalias !26245, !noundef !10
  %exitcond23.2.not.i = icmp eq i32 %9, 2, !dbg !26263
  br i1 %exitcond23.2.not.i, label %panic1.i, label %bb5.2.i, !dbg !26263

bb5.2.i:                                          ; preds = %bb4.2.i
  %13 = getelementptr inbounds nuw i8, ptr %_38.0.i, i32 12, !dbg !26263
  %_18.2.i = load i32, ptr %13, align 4, !dbg !26263, !noalias !26245, !noundef !10
  %_53.2.i = icmp ult i32 %_18.2.i, %_14.2.i, !dbg !26265
  %_49.not.2.i = icmp ugt i32 %_18.2.i, %_37.1.i
  %or.cond.2.i = or i1 %_53.2.i, %_49.not.2.i, !dbg !26265
  br i1 %or.cond.2.i, label %bb16.i, label %bb6.2.i, !dbg !26265, !prof !4596

bb6.2.i:                                          ; preds = %bb5.2.i
  %_54.2.i = sub nuw i32 %_18.2.i, %_14.2.i, !dbg !26280
  %_56.2.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i32 %_14.2.i, !dbg !26281
  %_27.2.i = getelementptr inbounds nuw i8, ptr %report.i, i32 80, !dbg !26286
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.2.i, i32 noundef %_54.2.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_25.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_26.i, i32 noundef 2, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.2.i) #31, !dbg !26285, !noalias !26247
  %exitcond.3.not.i = icmp eq i32 %_38.1.i, 3, !dbg !26259
  br i1 %exitcond.3.not.i, label %panic.i, label %bb4.3.i, !dbg !26259

bb4.3.i:                                          ; preds = %bb6.2.i
  %_14.3.i = load i32, ptr %13, align 4, !dbg !26259, !noalias !26245, !noundef !10
  %exitcond23.3.not.i = icmp eq i32 %9, 3, !dbg !26263
  br i1 %exitcond23.3.not.i, label %panic1.i, label %bb5.3.i, !dbg !26263

bb5.3.i:                                          ; preds = %bb4.3.i
  %14 = getelementptr inbounds nuw i8, ptr %_38.0.i, i32 16, !dbg !26263
  %_18.3.i = load i32, ptr %14, align 4, !dbg !26263, !noalias !26245, !noundef !10
  %_53.3.i = icmp ult i32 %_18.3.i, %_14.3.i, !dbg !26265
  %_49.not.3.i = icmp ugt i32 %_18.3.i, %_37.1.i
  %or.cond.3.i = or i1 %_53.3.i, %_49.not.3.i, !dbg !26265
  br i1 %or.cond.3.i, label %bb16.i, label %_RINvMsf_CsjLJhryqjeDL_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_bank_innerKb1_EB6_.exit, !dbg !26265, !prof !4596

_RINvMsf_CsjLJhryqjeDL_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_bank_innerKb1_EB6_.exit: ; preds = %bb5.3.i
  %_54.3.i = sub nuw i32 %_18.3.i, %_14.3.i, !dbg !26280
  %_56.3.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i32 %_14.3.i, !dbg !26281
  %_27.3.i = getelementptr inbounds nuw i8, ptr %report.i, i32 120, !dbg !26286
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.3.i, i32 noundef %_54.3.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_25.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_26.i, i32 noundef 3, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.3.i) #31, !dbg !26285, !noalias !26247
  %_39.0.i = load ptr, ptr %block, align 8, !dbg !26287, !alias.scope !26237, !noalias !26242, !nonnull !10, !align !10173, !noundef !10
  %15 = getelementptr inbounds nuw i8, ptr %block, i32 4, !dbg !26287
  %_39.1.i = load i32, ptr %15, align 4, !dbg !26287, !alias.scope !26237, !noalias !26242, !noundef !10
  %16 = getelementptr inbounds nuw i8, ptr %block, i32 56, !dbg !26288
  %_32.i = load i32, ptr %16, align 8, !dbg !26288, !alias.scope !26237, !noalias !26242, !noundef !10
; call <true_peak_limiter::LimiterCore<wide::f32x4_::f32x4>>::process_block_mono
  tail call fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_block_monoB5_(ptr noalias noundef nonnull align 16 dereferenceable(1248) %self, ptr noalias noundef nonnull align 4 %_39.0.i, i32 noundef %_39.1.i, i32 noundef %_32.i) #31, !dbg !26289, !noalias !26247
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 8 dereferenceable(328) %_0, ptr noundef nonnull align 8 dereferenceable(328) %report.i, i32 328, i1 false), !dbg !26290, !noalias !26291
  call void @llvm.lifetime.end.p0(ptr nonnull %report.i), !dbg !26292, !noalias !26245
  ret void, !dbg !26293
}
