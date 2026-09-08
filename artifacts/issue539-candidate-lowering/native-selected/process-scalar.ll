define void @_RNvXsd_CsdvPQf9CMsz3_17true_peak_limiterNtB5_23PreparedTruePeakLimiterNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process(ptr dead_on_unwind noalias noundef writable writeonly sret([40 x i8]) align 8 captures(none) dereferenceable(40) initializes((0, 40)) %_0, ptr noalias noundef align 8 dereferenceable(784) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(88) %block) unnamed_addr #0 !dbg !43503 {
start:
  %report = alloca [40 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !43504
  %_13.0 = load ptr, ptr %0, align 8, !dbg !43504, !nonnull !12, !align !14, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !43504
  %_13.1 = load i64, ptr %1, align 8, !dbg !43504, !noundef !12
  %2 = icmp eq i64 %_13.1, 0, !dbg !43504
  br i1 %2, label %bb3, label %bb2, !dbg !43504

bb2:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 780, !dbg !43505
  store i8 0, ptr %3, align 4, !dbg !43505
  br label %bb3, !dbg !43506

bb3:                                              ; preds = %start, %bb2
  call void @llvm.lifetime.start.p0(ptr nonnull %report), !dbg !43507
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(40) %report, i8 0, i64 40, i1 false), !dbg !43508
  %_16.0 = load ptr, ptr %block, align 8, !dbg !43512, !nonnull !12, !align !24, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !43512
  %_16.1 = load i64, ptr %4, align 8, !dbg !43512, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %block, i64 80, !dbg !43517
  %_7 = load i64, ptr %5, align 8, !dbg !43517, !noundef !12
  %_8 = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !43519
  %_9 = getelementptr inbounds nuw i8, ptr %self, i64 336, !dbg !43520
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_13.0, i64 noundef %_13.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %self, i64 noundef %_7, ptr noalias noundef nonnull align 8 dereferenceable(200) %_8, ptr noalias noundef nonnull align 8 dereferenceable(200) %_9, i64 noundef 0, ptr noalias noundef nonnull align 8 dereferenceable(40) %report) #31, !dbg !43521
  %6 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !43522
  %_14.0 = load ptr, ptr %6, align 8, !dbg !43522, !nonnull !12, !align !24, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !43522
  %_14.1 = load i64, ptr %7, align 8, !dbg !43522, !noundef !12
; call <true_peak_limiter::LimiterCore<f32>>::process_block
  tail call fastcc void @_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_(ptr noalias noundef align 8 dereferenceable(784) %self, ptr noalias noundef nonnull align 4 %_16.0, i64 noundef %_16.1, ptr noalias noundef nonnull align 4 %_14.0, i64 noundef %_14.1, i64 noundef %_16.1) #31, !dbg !43523
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(40) %_0, ptr noundef nonnull align 8 dereferenceable(40) %report, i64 40, i1 false), !dbg !43524
  call void @llvm.lifetime.end.p0(ptr nonnull %report), !dbg !43525
  ret void, !dbg !43526
}
