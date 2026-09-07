define void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef readonly align 8 captures(none) dereferenceable(200) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %shape, ptr noalias noundef nonnull readonly align 4 captures(address) %defaults.0, i64 noundef range(i64 0, 768614336404564651) %defaults.1, i32 noundef %rate) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !13360 {
start:
  %args.i.i = alloca [32 x i8], align 8
  %max2.i.i = alloca [8 x i8], align 8
  %min1.i.i = alloca [8 x i8], align 8
  %_44.idx = mul nuw nsw i64 %defaults.1, 12, !dbg !13361
  %_44 = getelementptr inbounds nuw i8, ptr %defaults.0, i64 %_44.idx, !dbg !13361
  %_6.i.i39 = icmp eq i64 %defaults.1, 0, !dbg !13370
  br i1 %_6.i.i39, label %bb5, label %bb4.lr.ph, !dbg !13380

bb4.lr.ph:                                        ; preds = %start
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %_49.1 = load i64, ptr %0, align 8, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %_49.0 = load ptr, ptr %1, align 8, !nonnull !12
  %_9.i = uitofp i32 %rate to double
  %_18 = load i64, ptr %shape, align 8
  %2 = getelementptr inbounds nuw i8, ptr %shape, i64 8
  %shape.val = load i64, ptr %2, align 8
  %shape.val.fr = freeze i64 %shape.val
  %_4.i.i = icmp ugt i64 %shape.val.fr, 31
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %_51.1 = load i64, ptr %3, align 8
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 176
  %_51.0 = load ptr, ptr %4, align 8, !nonnull !12
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %_53.1 = load i64, ptr %5, align 8
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %_53.0 = load ptr, ptr %6, align 8, !nonnull !12
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 152
  %_55.1 = load i64, ptr %7, align 8
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %_55.0 = load ptr, ptr %8, align 8, !nonnull !12
  br i1 %_4.i.i, label %bb4.us, label %bb4, !prof !651

bb4.us:                                           ; preds = %bb4.lr.ph, %bb13.us
  %iter.sroa.0.041.us = phi ptr [ %_16.i.i.us, %bb13.us ], [ %defaults.0, %bb4.lr.ph ]
  %iter.sroa.7.040.us = phi i64 [ %_9.0.i.us, %bb13.us ], [ 0, %bb4.lr.ph ]
  %_16.i.i.us = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.us, i64 12, !dbg !13381
  %_9.0.i.us = add nuw nsw i64 %iter.sroa.7.040.us, 1, !dbg !13384
  %exitcond.not = icmp eq i64 %iter.sroa.7.040.us, %_49.1, !dbg !13387
  br i1 %exitcond.not, label %panic, label %bb6.us, !dbg !13387

bb6.us:                                           ; preds = %bb4.us
  %9 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.us, i64 8, !dbg !13389
  %_13.us = load float, ptr %9, align 4, !dbg !13389, !noundef !12
  %10 = getelementptr inbounds nuw float, ptr %_49.0, i64 %iter.sroa.7.040.us, !dbg !13387
  store float %_13.us, ptr %10, align 4, !dbg !13387
  %_8.i.us = fpext float %_13.us to double, !dbg !13390
  %_7.i.us = fmul double %_9.i, %_8.i.us, !dbg !13393
  %_6.i.us = fdiv double %_7.i.us, 1.000000e+03, !dbg !13393
  %_5.i.us = fadd double %_6.i.us, 5.000000e-01, !dbg !13394
  %11 = tail call double @llvm.floor.f64(double %_5.i.us), !dbg !13395
  %or.cond.i.us = tail call i1 @llvm.is.fpclass.f64(double %11, i32 527), !dbg !13398
  br i1 %or.cond.i.us, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter17lookahead_samples.exit.us, label %bb2.i.us, !dbg !13398

bb2.i.us:                                         ; preds = %bb6.us
  %_12.i.us = tail call i64 @llvm.fptoui.sat.i64.f64(double %11), !dbg !13399
  %..i.i.us = tail call noundef i64 @llvm.umin.i64(i64 %_18, i64 %_12.i.us), !dbg !13400
  %12 = add i64 %..i.i.us, 1, !dbg !13402
  br label %_RNvCsdvPQf9CMsz3_17true_peak_limiter17lookahead_samples.exit.us, !dbg !13404

_RNvCsdvPQf9CMsz3_17true_peak_limiter17lookahead_samples.exit.us: ; preds = %bb2.i.us, %bb6.us
  %_0.sroa.0.0.i5.us = phi i64 [ %12, %bb2.i.us ], [ 1, %bb6.us ], !dbg !13405
  %exitcond53.not = icmp eq i64 %iter.sroa.7.040.us, %_51.1, !dbg !13406
  br i1 %exitcond53.not, label %panic1, label %bb9.us, !dbg !13406

bb9.us:                                           ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter17lookahead_samples.exit.us
  %_7.i.i.us = icmp ult i64 %_0.sroa.0.0.i5.us, 32, !dbg !13407
  %max.self.i.i.us = tail call i64 @llvm.umin.i64(i64 %_0.sroa.0.0.i5.us, i64 %shape.val.fr), !dbg !13407
  %_0.sroa.0.0.i.i.us = select i1 %_7.i.i.us, i64 32, i64 %max.self.i.i.us, !dbg !13407
  %_8.i7.us = sub i64 %shape.val.fr, %_0.sroa.0.0.i.i.us, !dbg !13409
  %_7.i8.us = trunc i64 %_8.i7.us to i32, !dbg !13409
  %_6.i6.us = trunc i64 %_0.sroa.0.0.i.i.us to i32, !dbg !13410
  %13 = getelementptr inbounds nuw %LaneShape, ptr %_51.0, i64 %iter.sroa.7.040.us, !dbg !13406
  store i32 %_6.i6.us, ptr %13, align 4, !dbg !13406
  %_16.sroa.4.0..sroa_idx.us = getelementptr inbounds nuw i8, ptr %13, i64 4, !dbg !13406
  store i32 %_6.i6.us, ptr %_16.sroa.4.0..sroa_idx.us, align 4, !dbg !13406
  %_16.sroa.5.0..sroa_idx.us = getelementptr inbounds nuw i8, ptr %13, i64 8, !dbg !13406
  store i32 %_7.i8.us, ptr %_16.sroa.5.0..sroa_idx.us, align 4, !dbg !13406
  %_23.us = load float, ptr %iter.sroa.0.041.us, align 4, !dbg !13411, !noundef !12
  %_4.i9.us = fpext float %_23.us to double, !dbg !13412
  %_3.i.us = fadd double %_4.i9.us, -1.000000e+00, !dbg !13415
  %_5.i10.us = fmul double %_3.i.us, 0x3FC542A5A12E1C5A, !dbg !13416
; call math::vendored::exp2::exp2
  %_2.i.us = tail call noundef double @_RNvNtNtCshmZ46FhrXRY_4math8vendored4exp24exp2(double noundef %_5.i10.us) #31, !dbg !13418
  %exitcond54.not = icmp eq i64 %iter.sroa.7.040.us, %_53.1, !dbg !13420
  br i1 %exitcond54.not, label %panic2, label %bb11.us, !dbg !13420

bb11.us:                                          ; preds = %bb9.us
  %_0.i.us = fptrunc double %_2.i.us to float, !dbg !13421
  %14 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_53.0, i64 %iter.sroa.7.040.us, !dbg !13420
  store float %_0.i.us, ptr %14, align 4, !dbg !13420
  %_21.sroa.4.0..sroa_idx.us = getelementptr inbounds nuw i8, ptr %14, i64 4, !dbg !13420
  store float %_0.i.us, ptr %_21.sroa.4.0..sroa_idx.us, align 4, !dbg !13420
  %_21.sroa.5.0..sroa_idx.us = getelementptr inbounds nuw i8, ptr %14, i64 8, !dbg !13420
  store float 0.000000e+00, ptr %_21.sroa.5.0..sroa_idx.us, align 4, !dbg !13420
  %_21.sroa.6.0..sroa_idx.us = getelementptr inbounds nuw i8, ptr %14, i64 12, !dbg !13420
  store i32 0, ptr %_21.sroa.6.0..sroa_idx.us, align 4, !dbg !13420
  %15 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.us, i64 4, !dbg !13422
  %_28.us = load float, ptr %15, align 4, !dbg !13422, !noundef !12
  %_8.i11.us = fpext float %_28.us to double, !dbg !13423
  %_7.i12.us = fmul double %_8.i11.us, 1.000000e-03, !dbg !13426
  %_6.i14.us = fmul double %_7.i12.us, %_9.i, !dbg !13427
  %_5.i15.us = fdiv double -1.000000e+00, %_6.i14.us, !dbg !13428
; call math::vendored::exp::exp
  %_4.i16.us = tail call noundef double @_RNvNtNtCshmZ46FhrXRY_4math8vendored3exp3exp(double noundef %_5.i15.us) #31, !dbg !13429
  %exitcond55.not = icmp eq i64 %iter.sroa.7.040.us, %_55.1, !dbg !13431
  br i1 %exitcond55.not, label %panic3, label %bb13.us, !dbg !13431

bb13.us:                                          ; preds = %bb11.us
  %_3.i17.us = fsub double 1.000000e+00, %_4.i16.us, !dbg !13432
  %_0.i18.us = fptrunc double %_3.i17.us to float, !dbg !13432
  %16 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_55.0, i64 %iter.sroa.7.040.us, !dbg !13431
  store float %_0.i18.us, ptr %16, align 4, !dbg !13431
  %_26.sroa.4.0..sroa_idx.us = getelementptr inbounds nuw i8, ptr %16, i64 4, !dbg !13431
  store float %_0.i18.us, ptr %_26.sroa.4.0..sroa_idx.us, align 4, !dbg !13431
  %_26.sroa.5.0..sroa_idx.us = getelementptr inbounds nuw i8, ptr %16, i64 8, !dbg !13431
  store float 0.000000e+00, ptr %_26.sroa.5.0..sroa_idx.us, align 4, !dbg !13431
  %_26.sroa.6.0..sroa_idx.us = getelementptr inbounds nuw i8, ptr %16, i64 12, !dbg !13431
  store i32 0, ptr %_26.sroa.6.0..sroa_idx.us, align 4, !dbg !13431
  %_6.i.i.us = icmp eq ptr %_16.i.i.us, %_44, !dbg !13370
  br i1 %_6.i.i.us, label %bb5, label %bb4.us, !dbg !13380

bb4:                                              ; preds = %bb4.lr.ph
  %_15.not = icmp eq i64 %_49.1, 0, !dbg !13387
  br i1 %_15.not, label %panic, label %bb6, !dbg !13387

bb5:                                              ; preds = %bb13.us, %start
; call <true_peak_limiter::ChannelState>::clear_runtime
  tail call fastcc void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState13clear_runtime(ptr noalias noundef align 8 dereferenceable(200) %self) #31, !dbg !13433
  ret void, !dbg !13434

bb6:                                              ; preds = %bb4
  %17 = getelementptr inbounds nuw i8, ptr %defaults.0, i64 8, !dbg !13389
  %_13 = load float, ptr %17, align 4, !dbg !13389, !noundef !12
  store float %_13, ptr %_49.0, align 4, !dbg !13387
  call void @llvm.lifetime.start.p0(ptr nonnull %min1.i.i), !dbg !13435, !noalias !13436
  store i64 32, ptr %min1.i.i, align 8, !dbg !13435, !noalias !13436
  call void @llvm.lifetime.start.p0(ptr nonnull %max2.i.i), !dbg !13435, !noalias !13436
  store i64 %shape.val.fr, ptr %max2.i.i, align 8, !dbg !13435, !noalias !13436
  call void @llvm.lifetime.start.p0(ptr nonnull %args.i.i), !dbg !13439, !noalias !13436
  store ptr %min1.i.i, ptr %args.i.i, align 8, !dbg !13439, !noalias !13436
  %_14.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 8, !dbg !13439
  store ptr @_RNvXsZ_NtNtCs4NRVxsYgnAr_4core3fmt3numjNtB7_5Debug3fmt, ptr %_14.sroa.4.0..sroa_idx.i.i, align 8, !dbg !13439, !noalias !13436
  %18 = getelementptr inbounds nuw i8, ptr %args.i.i, i64 16, !dbg !13439
  store ptr %max2.i.i, ptr %18, align 8, !dbg !13439, !noalias !13436
  %_15.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 24, !dbg !13439
  store ptr @_RNvXsZ_NtNtCs4NRVxsYgnAr_4core3fmt3numjNtB7_5Debug3fmt, ptr %_15.sroa.4.0..sroa_idx.i.i, align 8, !dbg !13439, !noalias !13436
; call core::panicking::panic_fmt
  call void @_RNvNtCs4NRVxsYgnAr_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_c04ec3b757b5ea96b9c02edd3d57a08e, ptr noundef nonnull %args.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7a177f1354713a4ff75927eff53b346c) #30, !dbg !13442, !noalias !13436
  unreachable, !dbg !13442

panic:                                            ; preds = %bb4.us, %bb4
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_49.1, i64 noundef %_49.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a20102380cf44fc1efe2c5344cffc5c6) #30, !dbg !13387
  unreachable, !dbg !13387

panic1:                                           ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter17lookahead_samples.exit.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_51.1, i64 noundef %_51.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1134635281c242588a9fd414ca7c27d6) #30, !dbg !13406
  unreachable, !dbg !13406

panic2:                                           ; preds = %bb9.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_53.1, i64 noundef %_53.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_954640f53e67cee7e83cb536615a8349) #30, !dbg !13420
  unreachable, !dbg !13420

panic3:                                           ; preds = %bb11.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.1, i64 noundef %_55.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1f687d87a0376ba32176b9f54c6cf767) #30, !dbg !13431
  unreachable, !dbg !13431
}
