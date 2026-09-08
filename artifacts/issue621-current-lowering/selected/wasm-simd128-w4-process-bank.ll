define internal void @_RNvXse_CsjLJhryqjeDL_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4ENtCs2mr8MC2dYJN_15effect_contract24PreparedNativeEffectBank12process_bankB5_(ptr dead_on_unwind noalias noundef writable writeonly sret([328 x i8]) align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef align 16 dereferenceable(1248) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(64) %block) unnamed_addr #1 !dbg !26013 {
start:
  %report.i = alloca [328 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26015), !dbg !26018
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26019), !dbg !26018
  %0 = getelementptr inbounds nuw i8, ptr %block, i32 16, !dbg !26021
  %_37.0.i = load ptr, ptr %0, align 8, !dbg !26021, !alias.scope !26019, !noalias !26025, !nonnull !10, !align !24857, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %block, i32 20, !dbg !26021
  %_37.1.i = load i32, ptr %1, align 4, !dbg !26021, !alias.scope !26019, !noalias !26025, !noundef !10
  %2 = icmp eq i32 %_37.1.i, 0, !dbg !26021
  br i1 %2, label %bb2.i, label %bb1.i, !dbg !26021

bb2.i:                                            ; preds = %bb1.i, %start
  call void @llvm.lifetime.start.p0(ptr nonnull %report.i), !dbg !26027, !noalias !26028
  %3 = getelementptr inbounds nuw i8, ptr %self, i32 1232, !dbg !26029
  %4 = load i8, ptr %3, align 16, !dbg !26029, !range !4765, !alias.scope !26015, !noalias !26030, !noundef !10
  call void @llvm.memset.p0.i32(ptr noundef nonnull align 8 dereferenceable(320) %report.i, i8 0, i32 320, i1 false), !noalias !26028
  %5 = getelementptr inbounds nuw i8, ptr %report.i, i32 320, !dbg !26031
  store i8 %4, ptr %5, align 8, !dbg !26031, !noalias !26028
  %6 = getelementptr inbounds nuw i8, ptr %block, i32 24
  %_38.0.i = load ptr, ptr %6, align 8, !alias.scope !26019, !noalias !26025, !nonnull !10, !align !10189, !noundef !10
  %7 = getelementptr inbounds nuw i8, ptr %block, i32 28
  %_38.1.i = load i32, ptr %7, align 4, !alias.scope !26019, !noalias !26025, !noundef !10
  %_26.i = getelementptr inbounds nuw i8, ptr %self, i32 1024
  %_25.i = getelementptr inbounds nuw i8, ptr %self, i32 924
  %8 = getelementptr inbounds nuw i8, ptr %block, i32 48
  %_24.i = load i64, ptr %8, align 8, !alias.scope !26019, !noalias !26025
  %_23.i = getelementptr inbounds nuw i8, ptr %self, i32 824
  %9 = tail call i32 @llvm.usub.sat.i32(i32 %_38.1.i, i32 1), !dbg !26035
  %exitcond.not.i = icmp eq i32 %_38.1.i, 0, !dbg !26043
  br i1 %exitcond.not.i, label %panic.i, label %bb4.i, !dbg !26043

bb1.i:                                            ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i32 1124, !dbg !26045
  store i8 0, ptr %10, align 4, !dbg !26045, !alias.scope !26015, !noalias !26030
  br label %bb2.i, !dbg !26046

bb4.i:                                            ; preds = %bb2.i
  %_14.i = load i32, ptr %_38.0.i, align 4, !dbg !26043, !noalias !26028, !noundef !10
  %exitcond23.not.i = icmp eq i32 %_38.1.i, 1, !dbg !26047
  br i1 %exitcond23.not.i, label %panic1.i, label %bb5.i, !dbg !26047

panic.i:                                          ; preds = %bb6.2.i, %bb6.1.i, %bb2.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %_38.1.i, i32 noundef %_38.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_4de42a380bf11124b0fb54945ff6764c) #33, !dbg !26043, !noalias !26028
  unreachable, !dbg !26043

bb5.i:                                            ; preds = %bb4.i
  %11 = getelementptr inbounds nuw i8, ptr %_38.0.i, i32 4, !dbg !26047
  %_18.i = load i32, ptr %11, align 4, !dbg !26047, !noalias !26028, !noundef !10
  %_53.i = icmp ult i32 %_18.i, %_14.i, !dbg !26049
  %_49.not.i = icmp ugt i32 %_18.i, %_37.1.i
  %or.cond.i = or i1 %_53.i, %_49.not.i, !dbg !26049
  br i1 %or.cond.i, label %bb16.i, label %bb4.1.i, !dbg !26049, !prof !4694

panic1.i:                                         ; preds = %bb4.3.i, %bb4.2.i, %bb4.1.i, %bb4.i
  %.lcssa14.i = phi i32 [ 1, %bb4.i ], [ 2, %bb4.1.i ], [ 3, %bb4.2.i ], [ 4, %bb4.3.i ], !dbg !26057
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsdkdt1aaAg1T_4core9panicking18panic_bounds_check(i32 noundef %.lcssa14.i, i32 noundef %_38.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_73103fbd0321956551877cb949272a2e) #33, !dbg !26047, !noalias !26028
  unreachable, !dbg !26047

bb16.i:                                           ; preds = %bb5.3.i, %bb5.2.i, %bb5.1.i, %bb5.i
  %_18.lcssa.i = phi i32 [ %_18.i, %bb5.i ], [ %_18.1.i, %bb5.1.i ], [ %_18.2.i, %bb5.2.i ], [ %_18.3.i, %bb5.3.i ], !dbg !26047
  %_14.lcssa20.i = phi i32 [ %_14.i, %bb5.i ], [ %_14.1.i, %bb5.1.i ], [ %_14.2.i, %bb5.2.i ], [ %_14.3.i, %bb5.3.i ], !dbg !26043
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsdkdt1aaAg1T_4core5slice5index16slice_index_fail(i32 noundef %_14.lcssa20.i, i32 noundef %_18.lcssa.i, i32 noundef %_37.1.i, ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(16) @alloc_1667d564aac521e9269794d81fe09235) #33, !dbg !26063, !noalias !26028
  unreachable, !dbg !26063

bb4.1.i:                                          ; preds = %bb5.i
  %_54.i = sub nuw i32 %_18.i, %_14.i, !dbg !26064
  %_56.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i32 %_14.i, !dbg !26065
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.i, i32 noundef %_54.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_25.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_26.i, i32 noundef 0, ptr noalias noundef nonnull align 8 dereferenceable(40) %report.i) #32, !dbg !26069, !noalias !26030
  %_14.1.i = load i32, ptr %11, align 4, !dbg !26043, !noalias !26028, !noundef !10
  %exitcond23.1.not.i = icmp eq i32 %9, 1, !dbg !26047
  br i1 %exitcond23.1.not.i, label %panic1.i, label %bb5.1.i, !dbg !26047

bb5.1.i:                                          ; preds = %bb4.1.i
  %12 = getelementptr inbounds nuw i8, ptr %_38.0.i, i32 8, !dbg !26047
  %_18.1.i = load i32, ptr %12, align 4, !dbg !26047, !noalias !26028, !noundef !10
  %_53.1.i = icmp ult i32 %_18.1.i, %_14.1.i, !dbg !26049
  %_49.not.1.i = icmp ugt i32 %_18.1.i, %_37.1.i
  %or.cond.1.i = or i1 %_53.1.i, %_49.not.1.i, !dbg !26049
  br i1 %or.cond.1.i, label %bb16.i, label %bb6.1.i, !dbg !26049, !prof !4694

bb6.1.i:                                          ; preds = %bb5.1.i
  %_54.1.i = sub nuw i32 %_18.1.i, %_14.1.i, !dbg !26064
  %_56.1.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i32 %_14.1.i, !dbg !26065
  %_27.1.i = getelementptr inbounds nuw i8, ptr %report.i, i32 40, !dbg !26070
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.1.i, i32 noundef %_54.1.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_25.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_26.i, i32 noundef 1, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.1.i) #32, !dbg !26069, !noalias !26030
  %exitcond.2.not.i = icmp eq i32 %_38.1.i, 2, !dbg !26043
  br i1 %exitcond.2.not.i, label %panic.i, label %bb4.2.i, !dbg !26043

bb4.2.i:                                          ; preds = %bb6.1.i
  %_14.2.i = load i32, ptr %12, align 4, !dbg !26043, !noalias !26028, !noundef !10
  %exitcond23.2.not.i = icmp eq i32 %9, 2, !dbg !26047
  br i1 %exitcond23.2.not.i, label %panic1.i, label %bb5.2.i, !dbg !26047

bb5.2.i:                                          ; preds = %bb4.2.i
  %13 = getelementptr inbounds nuw i8, ptr %_38.0.i, i32 12, !dbg !26047
  %_18.2.i = load i32, ptr %13, align 4, !dbg !26047, !noalias !26028, !noundef !10
  %_53.2.i = icmp ult i32 %_18.2.i, %_14.2.i, !dbg !26049
  %_49.not.2.i = icmp ugt i32 %_18.2.i, %_37.1.i
  %or.cond.2.i = or i1 %_53.2.i, %_49.not.2.i, !dbg !26049
  br i1 %or.cond.2.i, label %bb16.i, label %bb6.2.i, !dbg !26049, !prof !4694

bb6.2.i:                                          ; preds = %bb5.2.i
  %_54.2.i = sub nuw i32 %_18.2.i, %_14.2.i, !dbg !26064
  %_56.2.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i32 %_14.2.i, !dbg !26065
  %_27.2.i = getelementptr inbounds nuw i8, ptr %report.i, i32 80, !dbg !26070
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.2.i, i32 noundef %_54.2.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_25.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_26.i, i32 noundef 2, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.2.i) #32, !dbg !26069, !noalias !26030
  %exitcond.3.not.i = icmp eq i32 %_38.1.i, 3, !dbg !26043
  br i1 %exitcond.3.not.i, label %panic.i, label %bb4.3.i, !dbg !26043

bb4.3.i:                                          ; preds = %bb6.2.i
  %_14.3.i = load i32, ptr %13, align 4, !dbg !26043, !noalias !26028, !noundef !10
  %exitcond23.3.not.i = icmp eq i32 %9, 3, !dbg !26047
  br i1 %exitcond23.3.not.i, label %panic1.i, label %bb5.3.i, !dbg !26047

bb5.3.i:                                          ; preds = %bb4.3.i
  %14 = getelementptr inbounds nuw i8, ptr %_38.0.i, i32 16, !dbg !26047
  %_18.3.i = load i32, ptr %14, align 4, !dbg !26047, !noalias !26028, !noundef !10
  %_53.3.i = icmp ult i32 %_18.3.i, %_14.3.i, !dbg !26049
  %_49.not.3.i = icmp ugt i32 %_18.3.i, %_37.1.i
  %or.cond.3.i = or i1 %_53.3.i, %_49.not.3.i, !dbg !26049
  br i1 %or.cond.3.i, label %bb16.i, label %_RINvMsf_CsjLJhryqjeDL_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_bank_innerKb0_EB6_.exit, !dbg !26049, !prof !4694

_RINvMsf_CsjLJhryqjeDL_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E18process_bank_innerKb0_EB6_.exit: ; preds = %bb5.3.i
  %_54.3.i = sub nuw i32 %_18.3.i, %_14.3.i, !dbg !26064
  %_56.3.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i32 %_14.3.i, !dbg !26065
  %_27.3.i = getelementptr inbounds nuw i8, ptr %report.i, i32 120, !dbg !26070
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.3.i, i32 noundef %_54.3.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_25.i, ptr noalias noundef nonnull align 4 dereferenceable(100) %_26.i, i32 noundef 3, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.3.i) #32, !dbg !26069, !noalias !26030
  %_40.0.i = load ptr, ptr %block, align 8, !dbg !26071, !alias.scope !26019, !noalias !26025, !nonnull !10, !align !10189, !noundef !10
  %15 = getelementptr inbounds nuw i8, ptr %block, i32 4, !dbg !26071
  %_40.1.i = load i32, ptr %15, align 4, !dbg !26071, !alias.scope !26019, !noalias !26025, !noundef !10
  %16 = getelementptr inbounds nuw i8, ptr %block, i32 8, !dbg !26072
  %_41.0.i = load ptr, ptr %16, align 8, !dbg !26072, !alias.scope !26019, !noalias !26025, !nonnull !10, !align !10189, !noundef !10
  %17 = getelementptr inbounds nuw i8, ptr %block, i32 12, !dbg !26072
  %_41.1.i = load i32, ptr %17, align 4, !dbg !26072, !alias.scope !26019, !noalias !26025, !noundef !10
  %18 = getelementptr inbounds nuw i8, ptr %block, i32 56, !dbg !26073
  %_36.i = load i32, ptr %18, align 8, !dbg !26073, !alias.scope !26019, !noalias !26025, !noundef !10
; call <true_peak_limiter::LimiterCore<wide::f32x4_::f32x4>>::process_block
  tail call fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCoreNtNtCsk6DwtE1d11l_4wide6f32x4_5f32x4E13process_blockB5_(ptr noalias noundef nonnull align 16 dereferenceable(1248) %self, ptr noalias noundef nonnull align 4 %_40.0.i, i32 noundef %_40.1.i, ptr noalias noundef nonnull align 4 %_41.0.i, i32 noundef %_41.1.i, i32 noundef %_36.i) #32, !dbg !26074, !noalias !26030
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 8 dereferenceable(328) %_0, ptr noundef nonnull align 8 dereferenceable(328) %report.i, i32 328, i1 false), !dbg !26075, !noalias !26076
  call void @llvm.lifetime.end.p0(ptr nonnull %report.i), !dbg !26077, !noalias !26028
  ret void, !dbg !26078
}
