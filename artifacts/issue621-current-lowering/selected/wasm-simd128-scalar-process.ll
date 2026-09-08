define dso_local void @_RNvXsd_CsjLJhryqjeDL_17true_peak_limiterNtB5_23PreparedTruePeakLimiterNtCs2mr8MC2dYJN_15effect_contract20PreparedNativeEffect7process(ptr dead_on_unwind noalias noundef writable writeonly sret([40 x i8]) align 8 captures(none) dereferenceable(40) initializes((0, 40)) %_0, ptr noalias noundef align 8 dereferenceable(544) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(48) %block) unnamed_addr #1 !dbg !25986 {
start:
  %report = alloca [40 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %block, i32 16, !dbg !25987
  %_13.0 = load ptr, ptr %0, align 8, !dbg !25987, !nonnull !10, !align !24857, !noundef !10
  %1 = getelementptr inbounds nuw i8, ptr %block, i32 20, !dbg !25987
  %_13.1 = load i32, ptr %1, align 4, !dbg !25987, !noundef !10
  %2 = icmp eq i32 %_13.1, 0, !dbg !25987
  br i1 %2, label %bb3, label %bb2, !dbg !25987

bb2:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %self, i32 536, !dbg !25988
  store i8 0, ptr %3, align 8, !dbg !25988
  br label %bb3, !dbg !25989

bb3:                                              ; preds = %start, %bb2
  call void @llvm.lifetime.start.p0(ptr nonnull %report), !dbg !25990
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(40) %report, i8 0, i64 40, i1 false), !dbg !25991
  %_16.0 = load ptr, ptr %block, align 8, !dbg !25995, !nonnull !10, !align !10189, !noundef !10
  %4 = getelementptr inbounds nuw i8, ptr %block, i32 4, !dbg !25995
  %_16.1 = load i32, ptr %4, align 4, !dbg !25995, !noundef !10
  %5 = getelementptr inbounds nuw i8, ptr %block, i32 40, !dbg !26000
  %_7 = load i64, ptr %5, align 8, !dbg !26000, !noundef !10
  %_8 = getelementptr inbounds nuw i8, ptr %self, i32 324, !dbg !26002
  %_9 = getelementptr inbounds nuw i8, ptr %self, i32 424, !dbg !26003
; call true_peak_limiter::apply_automation
  call void @_RNvCsjLJhryqjeDL_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_13.0, i32 noundef %_13.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(88) %self, i64 noundef %_7, ptr noalias noundef nonnull align 4 dereferenceable(100) %_8, ptr noalias noundef nonnull align 4 dereferenceable(100) %_9, i32 noundef 0, ptr noalias noundef nonnull align 8 dereferenceable(40) %report) #32, !dbg !26004
  %6 = getelementptr inbounds nuw i8, ptr %block, i32 8, !dbg !26005
  %_14.0 = load ptr, ptr %6, align 8, !dbg !26005, !nonnull !10, !align !10189, !noundef !10
  %7 = getelementptr inbounds nuw i8, ptr %block, i32 12, !dbg !26005
  %_14.1 = load i32, ptr %7, align 4, !dbg !26005, !noundef !10
; call <true_peak_limiter::LimiterCore<f32>>::process_block
  tail call fastcc void @_RNvMs9_CsjLJhryqjeDL_17true_peak_limiterINtB5_11LimiterCorefE13process_blockB5_(ptr noalias noundef align 8 dereferenceable(544) %self, ptr noalias noundef nonnull align 4 %_16.0, i32 noundef %_16.1, ptr noalias noundef nonnull align 4 %_14.0, i32 noundef %_14.1, i32 noundef %_16.1) #32, !dbg !26006
  call void @llvm.memcpy.p0.p0.i32(ptr noundef nonnull align 8 dereferenceable(40) %_0, ptr noundef nonnull align 8 dereferenceable(40) %report, i32 40, i1 false), !dbg !26007
  call void @llvm.lifetime.end.p0(ptr nonnull %report), !dbg !26008
  ret void, !dbg !26009
}
