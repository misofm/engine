define internal void @_RNvXse_CsdvPQf9CMsz3_17true_peak_limiterINtB5_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ENtCsfGpaX3jkkqY_15effect_contract24PreparedNativeEffectBank12process_bankB5_(ptr dead_on_unwind noalias noundef writable writeonly sret([328 x i8]) align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef align 32 dereferenceable(2304) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(112) %block) unnamed_addr #0 !dbg !44096 {
start:
  %report.i = alloca [328 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !44097), !dbg !44100
  tail call void @llvm.experimental.noalias.scope.decl(metadata !44101), !dbg !44100
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !44103
  %_37.0.i = load ptr, ptr %0, align 8, !dbg !44103, !alias.scope !44101, !noalias !44106, !nonnull !12, !align !14, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !44103
  %_37.1.i = load i64, ptr %1, align 8, !dbg !44103, !alias.scope !44101, !noalias !44106, !noundef !12
  %2 = icmp eq i64 %_37.1.i, 0, !dbg !44103
  br i1 %2, label %bb2.i, label %bb1.i, !dbg !44103

bb2.i:                                            ; preds = %bb1.i, %start
  call void @llvm.lifetime.start.p0(ptr nonnull %report.i), !dbg !44108, !noalias !44109
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 2288, !dbg !44110
  %4 = load i8, ptr %3, align 16, !dbg !44110, !range !17, !alias.scope !44097, !noalias !44111, !noundef !12
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %report.i, i8 0, i64 320, i1 false), !noalias !44109
  %5 = getelementptr inbounds nuw i8, ptr %report.i, i64 320, !dbg !44112
  store i8 %4, ptr %5, align 8, !dbg !44112, !noalias !44109
  %6 = getelementptr inbounds nuw i8, ptr %block, i64 48
  %_38.0.i = load ptr, ptr %6, align 8, !alias.scope !44101, !noalias !44106, !nonnull !12, !align !24, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %block, i64 56
  %_38.1.i = load i64, ptr %7, align 8, !alias.scope !44101, !noalias !44106, !noundef !12
  %_26.i = getelementptr inbounds nuw i8, ptr %self, i64 1848
  %_25.i = getelementptr inbounds nuw i8, ptr %self, i64 1648
  %8 = getelementptr inbounds nuw i8, ptr %block, i64 96
  %_24.i = load i64, ptr %8, align 8, !alias.scope !44101, !noalias !44106
  %_23.i = getelementptr inbounds nuw i8, ptr %self, i64 2048
  %9 = tail call i64 @llvm.usub.sat.i64(i64 %_38.1.i, i64 1), !dbg !44115
  %exitcond.not.i = icmp eq i64 %_38.1.i, 0, !dbg !44123
  br i1 %exitcond.not.i, label %panic.i, label %bb4.i, !dbg !44123

bb1.i:                                            ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 2152, !dbg !44125
  store i8 0, ptr %10, align 8, !dbg !44125, !alias.scope !44097, !noalias !44111
  br label %bb2.i, !dbg !44126

bb4.i:                                            ; preds = %bb2.i
  %_14.i = load i32, ptr %_38.0.i, align 4, !dbg !44123, !noalias !44109, !noundef !12
  %start1.i = zext i32 %_14.i to i64, !dbg !44123
  %exitcond24.not.i = icmp eq i64 %_38.1.i, 1, !dbg !44127
  br i1 %exitcond24.not.i, label %panic2.i, label %bb5.i, !dbg !44127

panic.i:                                          ; preds = %bb6.6.i, %bb6.5.i, %bb6.4.i, %bb6.3.i, %bb6.2.i, %bb6.1.i, %bb2.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_38.1.i, i64 noundef %_38.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91f48754cb619b8499b70f373b18ecef) #30, !dbg !44123, !noalias !44109
  unreachable, !dbg !44123

bb5.i:                                            ; preds = %bb4.i
  %11 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 4, !dbg !44127
  %_18.i = load i32, ptr %11, align 4, !dbg !44127, !noalias !44109, !noundef !12
  %end.i = zext i32 %_18.i to i64, !dbg !44127
  %_53.i = icmp ult i32 %_18.i, %_14.i, !dbg !44129
  %_49.not.i = icmp ult i64 %_37.1.i, %end.i
  %or.cond.i = or i1 %_53.i, %_49.not.i, !dbg !44129
  br i1 %or.cond.i, label %bb16.i, label %bb4.1.i, !dbg !44129, !prof !165

panic2.i:                                         ; preds = %bb4.7.i, %bb4.6.i, %bb4.5.i, %bb4.4.i, %bb4.3.i, %bb4.2.i, %bb4.1.i, %bb4.i
  %.lcssa15.i = phi i64 [ 1, %bb4.i ], [ 2, %bb4.1.i ], [ 3, %bb4.2.i ], [ 4, %bb4.3.i ], [ 5, %bb4.4.i ], [ 6, %bb4.5.i ], [ 7, %bb4.6.i ], [ 8, %bb4.7.i ], !dbg !44137
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.lcssa15.i, i64 noundef %_38.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5a0c2622bcd26c1787d22f3bd7318ae1) #30, !dbg !44127, !noalias !44109
  unreachable, !dbg !44127

bb16.i:                                           ; preds = %bb5.7.i, %bb5.6.i, %bb5.5.i, %bb5.4.i, %bb5.3.i, %bb5.2.i, %bb5.1.i, %bb5.i
  %end.lcssa.i = phi i64 [ %end.i, %bb5.i ], [ %end.1.i, %bb5.1.i ], [ %end.2.i, %bb5.2.i ], [ %end.3.i, %bb5.3.i ], [ %end.4.i, %bb5.4.i ], [ %end.5.i, %bb5.5.i ], [ %end.6.i, %bb5.6.i ], [ %end.7.i, %bb5.7.i ], !dbg !44127
  %start1.lcssa21.i = phi i64 [ %start1.i, %bb5.i ], [ %start1.1.i, %bb5.1.i ], [ %start1.2.i, %bb5.2.i ], [ %start1.3.i, %bb5.3.i ], [ %start1.4.i, %bb5.4.i ], [ %start1.5.i, %bb5.5.i ], [ %start1.6.i, %bb5.6.i ], [ %start1.7.i, %bb5.7.i ], !dbg !44123
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1.lcssa21.i, i64 noundef %end.lcssa.i, i64 noundef %_37.1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_736dcbdd888794a2a6b72420f98deec7) #30, !dbg !44143, !noalias !44109
  unreachable, !dbg !44143

bb4.1.i:                                          ; preds = %bb5.i
  %_54.i = sub nuw nsw i64 %end.i, %start1.i, !dbg !44144
  %_56.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.i, !dbg !44145
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.i, i64 noundef %_54.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 0, ptr noalias noundef nonnull align 8 dereferenceable(40) %report.i) #31, !dbg !44149, !noalias !44111
  %_14.1.i = load i32, ptr %11, align 4, !dbg !44123, !noalias !44109, !noundef !12
  %start1.1.i = zext i32 %_14.1.i to i64, !dbg !44123
  %exitcond24.1.not.i = icmp eq i64 %9, 1, !dbg !44127
  br i1 %exitcond24.1.not.i, label %panic2.i, label %bb5.1.i, !dbg !44127

bb5.1.i:                                          ; preds = %bb4.1.i
  %12 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 8, !dbg !44127
  %_18.1.i = load i32, ptr %12, align 4, !dbg !44127, !noalias !44109, !noundef !12
  %end.1.i = zext i32 %_18.1.i to i64, !dbg !44127
  %_53.1.i = icmp ult i32 %_18.1.i, %_14.1.i, !dbg !44129
  %_49.not.1.i = icmp ult i64 %_37.1.i, %end.1.i
  %or.cond.1.i = or i1 %_53.1.i, %_49.not.1.i, !dbg !44129
  br i1 %or.cond.1.i, label %bb16.i, label %bb6.1.i, !dbg !44129, !prof !165

bb6.1.i:                                          ; preds = %bb5.1.i
  %_54.1.i = sub nuw nsw i64 %end.1.i, %start1.1.i, !dbg !44144
  %_56.1.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.1.i, !dbg !44145
  %_27.1.i = getelementptr inbounds nuw i8, ptr %report.i, i64 40, !dbg !44150
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.1.i, i64 noundef %_54.1.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 1, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.1.i) #31, !dbg !44149, !noalias !44111
  %exitcond.2.not.i = icmp eq i64 %_38.1.i, 2, !dbg !44123
  br i1 %exitcond.2.not.i, label %panic.i, label %bb4.2.i, !dbg !44123

bb4.2.i:                                          ; preds = %bb6.1.i
  %_14.2.i = load i32, ptr %12, align 4, !dbg !44123, !noalias !44109, !noundef !12
  %start1.2.i = zext i32 %_14.2.i to i64, !dbg !44123
  %exitcond24.2.not.i = icmp eq i64 %9, 2, !dbg !44127
  br i1 %exitcond24.2.not.i, label %panic2.i, label %bb5.2.i, !dbg !44127

bb5.2.i:                                          ; preds = %bb4.2.i
  %13 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 12, !dbg !44127
  %_18.2.i = load i32, ptr %13, align 4, !dbg !44127, !noalias !44109, !noundef !12
  %end.2.i = zext i32 %_18.2.i to i64, !dbg !44127
  %_53.2.i = icmp ult i32 %_18.2.i, %_14.2.i, !dbg !44129
  %_49.not.2.i = icmp ult i64 %_37.1.i, %end.2.i
  %or.cond.2.i = or i1 %_53.2.i, %_49.not.2.i, !dbg !44129
  br i1 %or.cond.2.i, label %bb16.i, label %bb6.2.i, !dbg !44129, !prof !165

bb6.2.i:                                          ; preds = %bb5.2.i
  %_54.2.i = sub nuw nsw i64 %end.2.i, %start1.2.i, !dbg !44144
  %_56.2.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.2.i, !dbg !44145
  %_27.2.i = getelementptr inbounds nuw i8, ptr %report.i, i64 80, !dbg !44150
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.2.i, i64 noundef %_54.2.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 2, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.2.i) #31, !dbg !44149, !noalias !44111
  %exitcond.3.not.i = icmp eq i64 %_38.1.i, 3, !dbg !44123
  br i1 %exitcond.3.not.i, label %panic.i, label %bb4.3.i, !dbg !44123

bb4.3.i:                                          ; preds = %bb6.2.i
  %_14.3.i = load i32, ptr %13, align 4, !dbg !44123, !noalias !44109, !noundef !12
  %start1.3.i = zext i32 %_14.3.i to i64, !dbg !44123
  %exitcond24.3.not.i = icmp eq i64 %9, 3, !dbg !44127
  br i1 %exitcond24.3.not.i, label %panic2.i, label %bb5.3.i, !dbg !44127

bb5.3.i:                                          ; preds = %bb4.3.i
  %14 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 16, !dbg !44127
  %_18.3.i = load i32, ptr %14, align 4, !dbg !44127, !noalias !44109, !noundef !12
  %end.3.i = zext i32 %_18.3.i to i64, !dbg !44127
  %_53.3.i = icmp ult i32 %_18.3.i, %_14.3.i, !dbg !44129
  %_49.not.3.i = icmp ult i64 %_37.1.i, %end.3.i
  %or.cond.3.i = or i1 %_53.3.i, %_49.not.3.i, !dbg !44129
  br i1 %or.cond.3.i, label %bb16.i, label %bb6.3.i, !dbg !44129, !prof !165

bb6.3.i:                                          ; preds = %bb5.3.i
  %_54.3.i = sub nuw nsw i64 %end.3.i, %start1.3.i, !dbg !44144
  %_56.3.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.3.i, !dbg !44145
  %_27.3.i = getelementptr inbounds nuw i8, ptr %report.i, i64 120, !dbg !44150
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.3.i, i64 noundef %_54.3.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 3, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.3.i) #31, !dbg !44149, !noalias !44111
  %exitcond.4.not.i = icmp eq i64 %_38.1.i, 4, !dbg !44123
  br i1 %exitcond.4.not.i, label %panic.i, label %bb4.4.i, !dbg !44123

bb4.4.i:                                          ; preds = %bb6.3.i
  %_14.4.i = load i32, ptr %14, align 4, !dbg !44123, !noalias !44109, !noundef !12
  %start1.4.i = zext i32 %_14.4.i to i64, !dbg !44123
  %exitcond24.4.not.i = icmp eq i64 %9, 4, !dbg !44127
  br i1 %exitcond24.4.not.i, label %panic2.i, label %bb5.4.i, !dbg !44127

bb5.4.i:                                          ; preds = %bb4.4.i
  %15 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 20, !dbg !44127
  %_18.4.i = load i32, ptr %15, align 4, !dbg !44127, !noalias !44109, !noundef !12
  %end.4.i = zext i32 %_18.4.i to i64, !dbg !44127
  %_53.4.i = icmp ult i32 %_18.4.i, %_14.4.i, !dbg !44129
  %_49.not.4.i = icmp ult i64 %_37.1.i, %end.4.i
  %or.cond.4.i = or i1 %_53.4.i, %_49.not.4.i, !dbg !44129
  br i1 %or.cond.4.i, label %bb16.i, label %bb6.4.i, !dbg !44129, !prof !165

bb6.4.i:                                          ; preds = %bb5.4.i
  %_54.4.i = sub nuw nsw i64 %end.4.i, %start1.4.i, !dbg !44144
  %_56.4.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.4.i, !dbg !44145
  %_27.4.i = getelementptr inbounds nuw i8, ptr %report.i, i64 160, !dbg !44150
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.4.i, i64 noundef %_54.4.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 4, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.4.i) #31, !dbg !44149, !noalias !44111
  %exitcond.5.not.i = icmp eq i64 %_38.1.i, 5, !dbg !44123
  br i1 %exitcond.5.not.i, label %panic.i, label %bb4.5.i, !dbg !44123

bb4.5.i:                                          ; preds = %bb6.4.i
  %_14.5.i = load i32, ptr %15, align 4, !dbg !44123, !noalias !44109, !noundef !12
  %start1.5.i = zext i32 %_14.5.i to i64, !dbg !44123
  %exitcond24.5.not.i = icmp eq i64 %9, 5, !dbg !44127
  br i1 %exitcond24.5.not.i, label %panic2.i, label %bb5.5.i, !dbg !44127

bb5.5.i:                                          ; preds = %bb4.5.i
  %16 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 24, !dbg !44127
  %_18.5.i = load i32, ptr %16, align 4, !dbg !44127, !noalias !44109, !noundef !12
  %end.5.i = zext i32 %_18.5.i to i64, !dbg !44127
  %_53.5.i = icmp ult i32 %_18.5.i, %_14.5.i, !dbg !44129
  %_49.not.5.i = icmp ult i64 %_37.1.i, %end.5.i
  %or.cond.5.i = or i1 %_53.5.i, %_49.not.5.i, !dbg !44129
  br i1 %or.cond.5.i, label %bb16.i, label %bb6.5.i, !dbg !44129, !prof !165

bb6.5.i:                                          ; preds = %bb5.5.i
  %_54.5.i = sub nuw nsw i64 %end.5.i, %start1.5.i, !dbg !44144
  %_56.5.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.5.i, !dbg !44145
  %_27.5.i = getelementptr inbounds nuw i8, ptr %report.i, i64 200, !dbg !44150
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.5.i, i64 noundef %_54.5.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 5, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.5.i) #31, !dbg !44149, !noalias !44111
  %exitcond.6.not.i = icmp eq i64 %_38.1.i, 6, !dbg !44123
  br i1 %exitcond.6.not.i, label %panic.i, label %bb4.6.i, !dbg !44123

bb4.6.i:                                          ; preds = %bb6.5.i
  %_14.6.i = load i32, ptr %16, align 4, !dbg !44123, !noalias !44109, !noundef !12
  %start1.6.i = zext i32 %_14.6.i to i64, !dbg !44123
  %exitcond24.6.not.i = icmp eq i64 %9, 6, !dbg !44127
  br i1 %exitcond24.6.not.i, label %panic2.i, label %bb5.6.i, !dbg !44127

bb5.6.i:                                          ; preds = %bb4.6.i
  %17 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 28, !dbg !44127
  %_18.6.i = load i32, ptr %17, align 4, !dbg !44127, !noalias !44109, !noundef !12
  %end.6.i = zext i32 %_18.6.i to i64, !dbg !44127
  %_53.6.i = icmp ult i32 %_18.6.i, %_14.6.i, !dbg !44129
  %_49.not.6.i = icmp ult i64 %_37.1.i, %end.6.i
  %or.cond.6.i = or i1 %_53.6.i, %_49.not.6.i, !dbg !44129
  br i1 %or.cond.6.i, label %bb16.i, label %bb6.6.i, !dbg !44129, !prof !165

bb6.6.i:                                          ; preds = %bb5.6.i
  %_54.6.i = sub nuw nsw i64 %end.6.i, %start1.6.i, !dbg !44144
  %_56.6.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.6.i, !dbg !44145
  %_27.6.i = getelementptr inbounds nuw i8, ptr %report.i, i64 240, !dbg !44150
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.6.i, i64 noundef %_54.6.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 6, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.6.i) #31, !dbg !44149, !noalias !44111
  %exitcond.7.not.i = icmp eq i64 %_38.1.i, 7, !dbg !44123
  br i1 %exitcond.7.not.i, label %panic.i, label %bb4.7.i, !dbg !44123

bb4.7.i:                                          ; preds = %bb6.6.i
  %_14.7.i = load i32, ptr %17, align 4, !dbg !44123, !noalias !44109, !noundef !12
  %start1.7.i = zext i32 %_14.7.i to i64, !dbg !44123
  %exitcond24.7.not.i = icmp eq i64 %9, 7, !dbg !44127
  br i1 %exitcond24.7.not.i, label %panic2.i, label %bb5.7.i, !dbg !44127

bb5.7.i:                                          ; preds = %bb4.7.i
  %18 = getelementptr inbounds nuw i8, ptr %_38.0.i, i64 32, !dbg !44127
  %_18.7.i = load i32, ptr %18, align 4, !dbg !44127, !noalias !44109, !noundef !12
  %end.7.i = zext i32 %_18.7.i to i64, !dbg !44127
  %_53.7.i = icmp ult i32 %_18.7.i, %_14.7.i, !dbg !44129
  %_49.not.7.i = icmp ult i64 %_37.1.i, %end.7.i
  %or.cond.7.i = or i1 %_53.7.i, %_49.not.7.i, !dbg !44129
  br i1 %or.cond.7.i, label %bb16.i, label %_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_EB6_.exit, !dbg !44129, !prof !165

_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb0_EB6_.exit: ; preds = %bb5.7.i
  %_54.7.i = sub nuw nsw i64 %end.7.i, %start1.7.i, !dbg !44144
  %_56.7.i = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0.i, i64 %start1.7.i, !dbg !44145
  %_27.7.i = getelementptr inbounds nuw i8, ptr %report.i, i64 280, !dbg !44150
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.7.i, i64 noundef %_54.7.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23.i, i64 noundef %_24.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26.i, i64 noundef 7, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.7.i) #31, !dbg !44149, !noalias !44111
  %_40.0.i = load ptr, ptr %block, align 8, !dbg !44151, !alias.scope !44101, !noalias !44106, !nonnull !12, !align !24, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !44151
  %_40.1.i = load i64, ptr %19, align 8, !dbg !44151, !alias.scope !44101, !noalias !44106, !noundef !12
  %20 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !44152
  %_41.0.i = load ptr, ptr %20, align 8, !dbg !44152, !alias.scope !44101, !noalias !44106, !nonnull !12, !align !24, !noundef !12
  %21 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !44152
  %_41.1.i = load i64, ptr %21, align 8, !dbg !44152, !alias.scope !44101, !noalias !44106, !noundef !12
  %22 = getelementptr inbounds nuw i8, ptr %block, i64 104, !dbg !44153
  %_36.i = load i32, ptr %22, align 8, !dbg !44153, !alias.scope !44101, !noalias !44106, !noundef !12
  %_35.i = zext i32 %_36.i to i64, !dbg !44153
; call <true_peak_limiter::LimiterCore<wide::f32x8_::f32x8>>::process_block
  tail call fastcc void @_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E13process_blockB5_(ptr noalias noundef nonnull align 32 dereferenceable(2304) %self, ptr noalias noundef nonnull align 4 %_40.0.i, i64 noundef %_40.1.i, ptr noalias noundef nonnull align 4 %_41.0.i, i64 noundef %_41.1.i, i64 noundef %_35.i) #31, !dbg !44154, !noalias !44111
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(328) %_0, ptr noundef nonnull align 8 dereferenceable(328) %report.i, i64 328, i1 false), !dbg !44155, !noalias !44156
  call void @llvm.lifetime.end.p0(ptr nonnull %report.i), !dbg !44157, !noalias !44109
  ret void, !dbg !44158
}
