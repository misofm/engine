define internal fastcc void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState13clear_runtime(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(200) %self) unnamed_addr #4 personality ptr @rust_eh_personality !dbg !12771 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8, !dbg !12772
  %_57.1 = load i64, ptr %0, align 8, !dbg !12772, !noundef !12
  %_222.i = icmp eq i64 %_57.1, 0, !dbg !12773
  br i1 %_222.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit, label %bb14.i.preheader, !dbg !12779

bb14.i.preheader:                                 ; preds = %start
  %.idx.i = shl i64 %_57.1, 2, !dbg !12780
  %_57.0 = load ptr, ptr %self, align 8, !dbg !12772, !nonnull !12, !noundef !12
  call void @llvm.memset.p0.i64(ptr nonnull align 4 %_57.0, i8 0, i64 %.idx.i, i1 false), !dbg !12784, !alias.scope !12785
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit, !dbg !12788

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb14.i.preheader, %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 24, !dbg !12788
  %_58.1 = load i64, ptr %1, align 8, !dbg !12788, !noundef !12
  %_222.i3 = icmp eq i64 %_58.1, 0, !dbg !12789
  br i1 %_222.i3, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit8, label %bb14.i4.preheader, !dbg !12794

bb14.i4.preheader:                                ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit
  %.idx.i2 = shl i64 %_58.1, 2, !dbg !12795
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 16, !dbg !12788
  %_58.0 = load ptr, ptr %2, align 8, !dbg !12788, !nonnull !12, !noundef !12
  call void @llvm.memset.p0.i64(ptr nonnull align 4 %_58.0, i8 0, i64 %.idx.i2, i1 false), !dbg !12799, !alias.scope !12800
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit8, !dbg !12803

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit8: ; preds = %bb14.i4.preheader, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32, !dbg !12803
  %_59.0 = load ptr, ptr %3, align 8, !dbg !12803, !nonnull !12, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 40, !dbg !12803
  %_59.1 = load i64, ptr %4, align 8, !dbg !12803, !noundef !12
  %.idx.i9 = shl nuw nsw i64 %_59.1, 2, !dbg !12804
  %5 = getelementptr inbounds nuw i8, ptr %_59.0, i64 %.idx.i9, !dbg !12804
  %_222.i10 = icmp eq i64 %_59.1, 0, !dbg !12810
  br i1 %_222.i10, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit15, label %bb14.i11, !dbg !12813

bb14.i11:                                         ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit8, %bb14.i11
  %iter.sroa.0.03.i12 = phi ptr [ %_32.i13, %bb14.i11 ], [ %_59.0, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit8 ]
  %_32.i13 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.03.i12, i64 4, !dbg !12814
  store float 1.000000e+00, ptr %iter.sroa.0.03.i12, align 4, !dbg !12817, !alias.scope !12818
  %_22.i14 = icmp eq ptr %_32.i13, %5, !dbg !12810
  br i1 %_22.i14, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit15, label %bb14.i11, !dbg !12813

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit15: ; preds = %bb14.i11, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit8
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 48, !dbg !12821
  %_60.0 = load ptr, ptr %6, align 8, !dbg !12821, !nonnull !12, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 56, !dbg !12821
  %_60.1 = load i64, ptr %7, align 8, !dbg !12821, !noundef !12
  %.idx.i16 = shl nuw nsw i64 %_60.1, 2, !dbg !12822
  %8 = getelementptr inbounds nuw i8, ptr %_60.0, i64 %.idx.i16, !dbg !12822
  %_222.i17 = icmp eq i64 %_60.1, 0, !dbg !12828
  br i1 %_222.i17, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit22, label %bb14.i18, !dbg !12831

bb14.i18:                                         ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit15, %bb14.i18
  %iter.sroa.0.03.i19 = phi ptr [ %_32.i20, %bb14.i18 ], [ %_60.0, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit15 ]
  %_32.i20 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.03.i19, i64 4, !dbg !12832
  store float 1.000000e+00, ptr %iter.sroa.0.03.i19, align 4, !dbg !12834, !alias.scope !12835
  %_22.i21 = icmp eq ptr %_32.i20, %8, !dbg !12828
  br i1 %_22.i21, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit22, label %bb14.i18, !dbg !12831

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit22: ; preds = %bb14.i18, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit15
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 72, !dbg !12838
  %_61.1 = load i64, ptr %9, align 8, !dbg !12838, !noundef !12
  %_222.i24 = icmp eq i64 %_61.1, 0, !dbg !12839
  br i1 %_222.i24, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit29, label %bb14.i25.preheader, !dbg !12844

bb14.i25.preheader:                               ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit22
  %.idx.i23 = shl i64 %_61.1, 2, !dbg !12845
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 64, !dbg !12838
  %_61.0 = load ptr, ptr %10, align 8, !dbg !12838, !nonnull !12, !noundef !12
  call void @llvm.memset.p0.i64(ptr nonnull align 4 %_61.0, i8 0, i64 %.idx.i23, i1 false), !dbg !12849, !alias.scope !12850
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit29, !dbg !12853

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit29: ; preds = %bb14.i25.preheader, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit22
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 80, !dbg !12853
  %_62.0 = load ptr, ptr %11, align 8, !dbg !12853, !nonnull !12, !noundef !12
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 88, !dbg !12853
  %_62.1 = load i64, ptr %12, align 8, !dbg !12853, !noundef !12
  %.idx.i30 = shl nuw nsw i64 %_62.1, 2, !dbg !12854
  %13 = getelementptr inbounds nuw i8, ptr %_62.0, i64 %.idx.i30, !dbg !12854
  %_222.i31 = icmp eq i64 %_62.1, 0, !dbg !12860
  br i1 %_222.i31, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit36, label %bb14.i32, !dbg !12863

bb14.i32:                                         ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit29, %bb14.i32
  %iter.sroa.0.03.i33 = phi ptr [ %_32.i34, %bb14.i32 ], [ %_62.0, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit29 ]
  %_32.i34 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.03.i33, i64 4, !dbg !12864
  store float 1.000000e+00, ptr %iter.sroa.0.03.i33, align 4, !dbg !12866, !alias.scope !12867
  %_22.i35 = icmp eq ptr %_32.i34, %13, !dbg !12860
  br i1 %_22.i35, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit36, label %bb14.i32, !dbg !12863

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit36: ; preds = %bb14.i32, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit29
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !12870
  %_63.0 = load ptr, ptr %14, align 8, !dbg !12870, !nonnull !12, !noundef !12
  %15 = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !12870
  %_63.1 = load i64, ptr %15, align 8, !dbg !12870, !noundef !12
  %16 = shl nuw nsw i64 %_63.1, 2, !dbg !12871
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_63.0, i8 0, i64 %16, i1 false), !dbg !12871, !alias.scope !12875
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 96, !dbg !12878
  %_64.0 = load ptr, ptr %17, align 8, !dbg !12878, !nonnull !12, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !12878
  %_64.1 = load i64, ptr %18, align 8, !dbg !12878, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !12879
  %_65.0 = load ptr, ptr %19, align 8, !dbg !12879, !nonnull !12, !noundef !12
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 184, !dbg !12879
  %_65.1 = load i64, ptr %20, align 8, !dbg !12879, !noundef !12
  %..i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_65.1, i64 %_64.1), !dbg !12880
  %_2.i5.not = icmp eq i64 %..i.i.i, 0, !dbg !12890
  br i1 %_2.i5.not, label %bb5, label %bb4, !dbg !12890

bb4:                                              ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit36, %bb4
  %iter.sroa.8.06 = phi i64 [ %21, %bb4 ], [ 0, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit36 ]
  %21 = add nuw i64 %iter.sroa.8.06, 1, !dbg !12897
  %_3.i.i = getelementptr inbounds nuw float, ptr %_64.0, i64 %iter.sroa.8.06, !dbg !12899
  %_3.i1.i = getelementptr inbounds nuw %LaneShape, ptr %_65.0, i64 %iter.sroa.8.06, !dbg !12904
  %_23 = load i32, ptr %_3.i1.i, align 4, !dbg !12907, !noundef !12
  %22 = uitofp i32 %_23 to float, !dbg !12909
  store float %22, ptr %_3.i.i, align 4, !dbg !12909
  %exitcond.not = icmp eq i64 %21, %..i.i.i, !dbg !12890
  br i1 %exitcond.not, label %bb5, label %bb4, !dbg !12890

bb5:                                              ; preds = %bb4, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit36
  ret void, !dbg !12910
}
