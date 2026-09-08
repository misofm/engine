define void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address) %spans.0, i64 noundef range(i64 0, 230584300921369396) %spans.1, ptr noalias noundef readonly align 8 captures(none) dereferenceable(104) %metadata, i64 noundef %first_sample, ptr noalias noundef readonly align 8 captures(none) dereferenceable(200) %left, ptr noalias noundef readonly align 8 captures(none) dereferenceable(200) %right, i64 noundef %lane, ptr noalias noundef align 8 captures(none) dereferenceable(40) %report) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !12092 {
start:
  %pending = alloca [32 x i8], align 4
  call void @llvm.lifetime.start.p0(ptr nonnull %pending), !dbg !12093
  store i32 0, ptr %pending, align 4
  %_9.sroa.5.0.pending.sroa_idx = getelementptr inbounds nuw i8, ptr %pending, i64 4
  %_9.sroa.5101.0.pending.sroa_idx = getelementptr inbounds nuw i8, ptr %pending, i64 8
  store i32 0, ptr %_9.sroa.5101.0.pending.sroa_idx, align 4
  %_9.sroa.6.0.pending.sroa_idx = getelementptr inbounds nuw i8, ptr %pending, i64 12
  %0 = getelementptr inbounds nuw i8, ptr %pending, i64 16
  store i32 0, ptr %0, align 4
  %_9.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %pending, i64 20
  %_9.sroa.5101.0..sroa_idx = getelementptr inbounds nuw i8, ptr %pending, i64 24
  store i32 0, ptr %_9.sroa.5101.0..sroa_idx, align 4
  %_9.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %pending, i64 28
  %_93.idx = mul nuw nsw i64 %spans.1, 40, !dbg !12094
  %_93 = getelementptr inbounds nuw i8, ptr %spans.0, i64 %_93.idx, !dbg !12094
  %_6.i.i6164 = icmp eq i64 %spans.1, 0, !dbg !12105
  br i1 %_6.i.i6164, label %bb5, label %bb4.lr.ph.lr.ph, !dbg !12115

bb4.lr.ph.lr.ph:                                  ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %report, i64 16
  %2 = getelementptr inbounds nuw i8, ptr %metadata, i64 92
  %_33 = load i32, ptr %2, align 4
  %_32 = zext i32 %_33 to i64
  %.promoted69 = load i64, ptr %1, align 8
  br label %bb4.lr.ph, !dbg !12115

bb4.lr.ph:                                        ; preds = %bb4.lr.ph.lr.ph, %bb32
  %.promoted71 = phi i64 [ %.promoted69, %bb4.lr.ph.lr.ph ], [ %.promoted70, %bb32 ]
  %last_order.sroa.3.0.ph68 = phi i32 [ undef, %bb4.lr.ph.lr.ph ], [ %_110.0, %bb32 ]
  %last_order.sroa.0.0.ph67.not = phi i1 [ true, %bb4.lr.ph.lr.ph ], [ false, %bb32 ]
  %iter.sroa.0.0.ph66 = phi ptr [ %spans.0, %bb4.lr.ph.lr.ph ], [ %_16.i.i, %bb32 ]
  %iter.sroa.7.0.ph65 = phi i64 [ 0, %bb4.lr.ph.lr.ph ], [ %_9.0.i, %bb32 ]
  br label %bb4, !dbg !12115

bb4:                                              ; preds = %bb4.lr.ph, %bb36
  %.promoted70 = phi i64 [ %.promoted71, %bb4.lr.ph ], [ %54, %bb36 ]
  %iter.sroa.0.063 = phi ptr [ %iter.sroa.0.0.ph66, %bb4.lr.ph ], [ %_16.i.i, %bb36 ]
  %iter.sroa.7.062 = phi i64 [ %iter.sroa.7.0.ph65, %bb4.lr.ph ], [ %_9.0.i, %bb36 ]
  %_16.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.063, i64 40, !dbg !12116
  %_9.0.i = add i64 %iter.sroa.7.062, 1, !dbg !12119
  %3 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.063, i64 32, !dbg !12122
  %_19 = load i32, ptr %3, align 8, !dbg !12122, !range !12124, !noundef !12
  switch i32 %_19, label %default.unreachable104 [
    i32 1, label %bb9
    i32 2, label %bb7
    i32 3, label %bb36
  ], !dbg !12125

bb5:                                              ; preds = %bb32, %bb36, %start
  %4 = getelementptr inbounds nuw i8, ptr %metadata, i64 72, !dbg !12126
  %rate = load i32, ptr %4, align 8, !dbg !12126, !noundef !12
  %_9.i = uitofp i32 %rate to double
  %5 = load i32, ptr %pending, align 4, !dbg !12127, !range !12132, !noundef !12
  %6 = trunc nuw i32 %5 to i1, !dbg !12133
  br i1 %6, label %bb42, label %bb46, !dbg !12133

bb42:                                             ; preds = %bb5
  %7 = getelementptr inbounds nuw i8, ptr %left, i64 136, !dbg !12134
  %_135.1 = load i64, ptr %7, align 8, !dbg !12134, !noundef !12
  %_76 = icmp ult i64 %lane, %_135.1, !dbg !12134
  br i1 %_76, label %bb43, label %panic6, !dbg !12134

bb43:                                             ; preds = %bb42
  %8 = getelementptr inbounds nuw i8, ptr %left, i64 128, !dbg !12134
  %_135.0 = load ptr, ptr %8, align 8, !dbg !12134, !nonnull !12, !noundef !12
  %value = load float, ptr %_9.sroa.5.0.pending.sroa_idx, align 4, !dbg !12135, !noundef !12
  %_74 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_135.0, i64 %lane, !dbg !12134
  %_4.i = fpext float %value to double, !dbg !12136
  %_3.i = fadd double %_4.i, -1.000000e+00, !dbg !12145
  %_5.i = fmul double %_3.i, 0x3FC542A5A12E1C5A, !dbg !12146
; call math::vendored::exp2::exp2
  %_2.i = tail call noundef double @_RNvNtNtCshmZ46FhrXRY_4math8vendored4exp24exp2(double noundef %_5.i) #31, !dbg !12151
  %_0.i = fptrunc double %_2.i to float, !dbg !12154
  %9 = getelementptr inbounds nuw i8, ptr %_74, i64 4, !dbg !12155
  store float %_0.i, ptr %9, align 4, !dbg !12155
  %_120 = load float, ptr %_74, align 4, !dbg !12161, !noundef !12
  %_123 = bitcast float %_120 to i32, !dbg !12162
  %_125 = bitcast float %_0.i to i32, !dbg !12170
  %_124 = icmp ne i32 %_123, %_125, !dbg !12173
  %10 = icmp eq i32 %_123, -2147483648
  %or.cond = or i1 %_124, %10, !dbg !12173
  %11 = tail call float @llvm.fabs.f32(float %_120)
  %_119 = fcmp ueq float %11, 0x7FF0000000000000
  %or.cond35 = or i1 %_119, %or.cond, !dbg !12173
  %12 = getelementptr inbounds nuw i8, ptr %_74, i64 8, !dbg !12174
  br i1 %or.cond35, label %bb68, label %bb67, !dbg !12173

panic6:                                           ; preds = %bb42.1, %bb42
  %_135.1.lcssa = phi i64 [ %_135.1, %bb42 ], [ %_135.1.1, %bb42.1 ], !dbg !12134
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %lane, i64 noundef %_135.1.lcssa, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5cf57561855c71df127711389f77b379) #30, !dbg !12134
  unreachable, !dbg !12134

bb68:                                             ; preds = %bb43
  %_121 = fsub float %_0.i, %_120, !dbg !12175
  %13 = fmul float %_121, 1.562500e-02, !dbg !12176
  br label %bb46.sink.split, !dbg !12177

bb67:                                             ; preds = %bb43
  store float %_0.i, ptr %_74, align 4, !dbg !12178
  br label %bb46.sink.split, !dbg !12177

bb46.sink.split:                                  ; preds = %bb68, %bb67
  %.sink113 = phi float [ 0.000000e+00, %bb67 ], [ %13, %bb68 ]
  %.sink = phi i32 [ 0, %bb67 ], [ 64, %bb68 ]
  store float %.sink113, ptr %12, align 4, !dbg !12174
  %14 = getelementptr inbounds nuw i8, ptr %_74, i64 12, !dbg !12174
  store i32 %.sink, ptr %14, align 4, !dbg !12174
  br label %bb46, !dbg !12179

bb46:                                             ; preds = %bb46.sink.split, %bb5
  %15 = load i32, ptr %_9.sroa.5101.0.pending.sroa_idx, align 4, !dbg !12179, !range !12132, !noundef !12
  %16 = trunc nuw i32 %15 to i1, !dbg !12181
  br i1 %16, label %bb47, label %bb41.1, !dbg !12181

bb47:                                             ; preds = %bb46
  %17 = getelementptr inbounds nuw i8, ptr %left, i64 152, !dbg !12182
  %_137.1 = load i64, ptr %17, align 8, !dbg !12182, !noundef !12
  %_82 = icmp ult i64 %lane, %_137.1, !dbg !12182
  br i1 %_82, label %bb48, label %panic9, !dbg !12182

bb41.1.sink.split:                                ; preds = %bb76, %bb75
  %.sink123 = phi float [ %44, %bb76 ], [ 0.000000e+00, %bb75 ]
  %.sink114 = phi i32 [ 64, %bb76 ], [ 0, %bb75 ]
  store float %.sink123, ptr %43, align 4, !dbg !12183
  %18 = getelementptr inbounds nuw i8, ptr %_80, i64 12, !dbg !12183
  store i32 %.sink114, ptr %18, align 4, !dbg !12183
  br label %bb41.1, !dbg !12127

bb41.1:                                           ; preds = %bb41.1.sink.split, %bb46
  %19 = load i32, ptr %0, align 4, !dbg !12127, !range !12132, !noundef !12
  %20 = trunc nuw i32 %19 to i1, !dbg !12133
  br i1 %20, label %bb42.1, label %bb46.1, !dbg !12133

bb42.1:                                           ; preds = %bb41.1
  %21 = getelementptr inbounds nuw i8, ptr %right, i64 136, !dbg !12134
  %_135.1.1 = load i64, ptr %21, align 8, !dbg !12134, !noundef !12
  %_76.1 = icmp ult i64 %lane, %_135.1.1, !dbg !12134
  br i1 %_76.1, label %bb43.1, label %panic6, !dbg !12134

bb43.1:                                           ; preds = %bb42.1
  %22 = getelementptr inbounds nuw i8, ptr %right, i64 128, !dbg !12134
  %_135.0.1 = load ptr, ptr %22, align 8, !dbg !12134, !nonnull !12, !noundef !12
  %value.1 = load float, ptr %_9.sroa.5.0..sroa_idx, align 4, !dbg !12135, !noundef !12
  %_74.1 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_135.0.1, i64 %lane, !dbg !12134
  %_4.i.1 = fpext float %value.1 to double, !dbg !12136
  %_3.i.1 = fadd double %_4.i.1, -1.000000e+00, !dbg !12145
  %_5.i.1 = fmul double %_3.i.1, 0x3FC542A5A12E1C5A, !dbg !12146
; call math::vendored::exp2::exp2
  %_2.i.1 = tail call noundef double @_RNvNtNtCshmZ46FhrXRY_4math8vendored4exp24exp2(double noundef %_5.i.1) #31, !dbg !12151
  %_0.i.1 = fptrunc double %_2.i.1 to float, !dbg !12154
  %23 = getelementptr inbounds nuw i8, ptr %_74.1, i64 4, !dbg !12155
  store float %_0.i.1, ptr %23, align 4, !dbg !12155
  %_120.1 = load float, ptr %_74.1, align 4, !dbg !12161, !noundef !12
  %_123.1 = bitcast float %_120.1 to i32, !dbg !12162
  %_125.1 = bitcast float %_0.i.1 to i32, !dbg !12170
  %_124.1 = icmp ne i32 %_123.1, %_125.1, !dbg !12173
  %24 = icmp eq i32 %_123.1, -2147483648
  %or.cond.1 = or i1 %_124.1, %24, !dbg !12173
  %25 = tail call float @llvm.fabs.f32(float %_120.1)
  %_119.1 = fcmp ueq float %25, 0x7FF0000000000000
  %or.cond35.1 = or i1 %_119.1, %or.cond.1, !dbg !12173
  %26 = getelementptr inbounds nuw i8, ptr %_74.1, i64 8, !dbg !12174
  br i1 %or.cond35.1, label %bb68.1, label %bb67.1, !dbg !12173

bb67.1:                                           ; preds = %bb43.1
  store float %_0.i.1, ptr %_74.1, align 4, !dbg !12178
  br label %bb46.1.sink.split, !dbg !12177

bb68.1:                                           ; preds = %bb43.1
  %_121.1 = fsub float %_0.i.1, %_120.1, !dbg !12175
  %27 = fmul float %_121.1, 1.562500e-02, !dbg !12176
  br label %bb46.1.sink.split, !dbg !12177

bb46.1.sink.split:                                ; preds = %bb67.1, %bb68.1
  %.sink118 = phi float [ %27, %bb68.1 ], [ 0.000000e+00, %bb67.1 ]
  %.sink116 = phi i32 [ 64, %bb68.1 ], [ 0, %bb67.1 ]
  store float %.sink118, ptr %26, align 4, !dbg !12174
  %28 = getelementptr inbounds nuw i8, ptr %_74.1, i64 12, !dbg !12174
  store i32 %.sink116, ptr %28, align 4, !dbg !12174
  br label %bb46.1, !dbg !12179

bb46.1:                                           ; preds = %bb46.1.sink.split, %bb41.1
  %29 = load i32, ptr %_9.sroa.5101.0..sroa_idx, align 4, !dbg !12179, !range !12132, !noundef !12
  %30 = trunc nuw i32 %29 to i1, !dbg !12181
  br i1 %30, label %bb47.1, label %bb50.1, !dbg !12181

bb47.1:                                           ; preds = %bb46.1
  %31 = getelementptr inbounds nuw i8, ptr %right, i64 152, !dbg !12182
  %_137.1.1 = load i64, ptr %31, align 8, !dbg !12182, !noundef !12
  %_82.1 = icmp ult i64 %lane, %_137.1.1, !dbg !12182
  br i1 %_82.1, label %bb48.1, label %panic9, !dbg !12182

bb48.1:                                           ; preds = %bb47.1
  %32 = getelementptr inbounds nuw i8, ptr %right, i64 144, !dbg !12182
  %_137.0.1 = load ptr, ptr %32, align 8, !dbg !12182, !nonnull !12, !noundef !12
  %value8.1 = load float, ptr %_9.sroa.6.0..sroa_idx, align 4, !dbg !12185, !noundef !12
  %_80.1 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_137.0.1, i64 %lane, !dbg !12182
  %_8.i.1 = fpext float %value8.1 to double, !dbg !12186
  %_7.i.1 = fmul double %_8.i.1, 1.000000e-03, !dbg !12191
  %_6.i.1 = fmul double %_7.i.1, %_9.i, !dbg !12192
  %_5.i43.1 = fdiv double -1.000000e+00, %_6.i.1, !dbg !12193
; call math::vendored::exp::exp
  %_4.i44.1 = tail call noundef double @_RNvNtNtCshmZ46FhrXRY_4math8vendored3exp3exp(double noundef %_5.i43.1) #31, !dbg !12194
  %_3.i45.1 = fsub double 1.000000e+00, %_4.i44.1, !dbg !12197
  %_0.i46.1 = fptrunc double %_3.i45.1 to float, !dbg !12197
  %33 = getelementptr inbounds nuw i8, ptr %_80.1, i64 4, !dbg !12198
  store float %_0.i46.1, ptr %33, align 4, !dbg !12198
  %_128.1 = load float, ptr %_80.1, align 4, !dbg !12199, !noundef !12
  %_131.1 = bitcast float %_128.1 to i32, !dbg !12200
  %_133.1 = bitcast float %_0.i46.1 to i32, !dbg !12205
  %_132.1 = icmp ne i32 %_131.1, %_133.1, !dbg !12209
  %34 = icmp eq i32 %_131.1, -2147483648
  %or.cond15.1 = or i1 %_132.1, %34, !dbg !12209
  %35 = tail call float @llvm.fabs.f32(float %_128.1)
  %_127.1 = fcmp ueq float %35, 0x7FF0000000000000
  %or.cond36.1 = or i1 %_127.1, %or.cond15.1, !dbg !12209
  %36 = getelementptr inbounds nuw i8, ptr %_80.1, i64 8, !dbg !12183
  br i1 %or.cond36.1, label %bb76.1, label %bb75.1, !dbg !12209

bb75.1:                                           ; preds = %bb48.1
  store float %_0.i46.1, ptr %_80.1, align 4, !dbg !12210
  br label %bb50.1.sink.split, !dbg !12211

bb76.1:                                           ; preds = %bb48.1
  %_129.1 = fsub float %_0.i46.1, %_128.1, !dbg !12212
  %37 = fmul float %_129.1, 1.562500e-02, !dbg !12213
  br label %bb50.1.sink.split, !dbg !12211

bb50.1.sink.split:                                ; preds = %bb75.1, %bb76.1
  %.sink121 = phi float [ %37, %bb76.1 ], [ 0.000000e+00, %bb75.1 ]
  %.sink119 = phi i32 [ 64, %bb76.1 ], [ 0, %bb75.1 ]
  store float %.sink121, ptr %36, align 4, !dbg !12183
  %38 = getelementptr inbounds nuw i8, ptr %_80.1, i64 12, !dbg !12183
  store i32 %.sink119, ptr %38, align 4, !dbg !12183
  br label %bb50.1, !dbg !12214

bb50.1:                                           ; preds = %bb50.1.sink.split, %bb46.1
  call void @llvm.lifetime.end.p0(ptr nonnull %pending), !dbg !12214
  ret void, !dbg !12215

bb48:                                             ; preds = %bb47
  %39 = getelementptr inbounds nuw i8, ptr %left, i64 144, !dbg !12182
  %_137.0 = load ptr, ptr %39, align 8, !dbg !12182, !nonnull !12, !noundef !12
  %value8 = load float, ptr %_9.sroa.6.0.pending.sroa_idx, align 4, !dbg !12185, !noundef !12
  %_80 = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_137.0, i64 %lane, !dbg !12182
  %_8.i = fpext float %value8 to double, !dbg !12186
  %_7.i = fmul double %_8.i, 1.000000e-03, !dbg !12191
  %_6.i = fmul double %_7.i, %_9.i, !dbg !12192
  %_5.i43 = fdiv double -1.000000e+00, %_6.i, !dbg !12193
; call math::vendored::exp::exp
  %_4.i44 = tail call noundef double @_RNvNtNtCshmZ46FhrXRY_4math8vendored3exp3exp(double noundef %_5.i43) #31, !dbg !12194
  %_3.i45 = fsub double 1.000000e+00, %_4.i44, !dbg !12197
  %_0.i46 = fptrunc double %_3.i45 to float, !dbg !12197
  %40 = getelementptr inbounds nuw i8, ptr %_80, i64 4, !dbg !12198
  store float %_0.i46, ptr %40, align 4, !dbg !12198
  %_128 = load float, ptr %_80, align 4, !dbg !12199, !noundef !12
  %_131 = bitcast float %_128 to i32, !dbg !12200
  %_133 = bitcast float %_0.i46 to i32, !dbg !12205
  %_132 = icmp ne i32 %_131, %_133, !dbg !12209
  %41 = icmp eq i32 %_131, -2147483648
  %or.cond15 = or i1 %_132, %41, !dbg !12209
  %42 = tail call float @llvm.fabs.f32(float %_128)
  %_127 = fcmp ueq float %42, 0x7FF0000000000000
  %or.cond36 = or i1 %_127, %or.cond15, !dbg !12209
  %43 = getelementptr inbounds nuw i8, ptr %_80, i64 8, !dbg !12183
  br i1 %or.cond36, label %bb76, label %bb75, !dbg !12209

panic9:                                           ; preds = %bb47.1, %bb47
  %_137.1.lcssa = phi i64 [ %_137.1, %bb47 ], [ %_137.1.1, %bb47.1 ], !dbg !12182
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %lane, i64 noundef %_137.1.lcssa, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_93e6eee1cc4097d01ff84e10e6b70f81) #30, !dbg !12182
  unreachable, !dbg !12182

bb76:                                             ; preds = %bb48
  %_129 = fsub float %_0.i46, %_128, !dbg !12212
  %44 = fmul float %_129, 1.562500e-02, !dbg !12213
  br label %bb41.1.sink.split, !dbg !12211

bb75:                                             ; preds = %bb48
  store float %_0.i46, ptr %_80, align 4, !dbg !12210
  br label %bb41.1.sink.split, !dbg !12211

default.unreachable104:                           ; preds = %bb4
  unreachable

bb7:                                              ; preds = %bb4
  br label %bb9, !dbg !12216

bb9:                                              ; preds = %bb4, %bb7
  %channel.sroa.0.0.sroa.phi = phi ptr [ %0, %bb7 ], [ %pending, %bb4 ], !dbg !12217
  %channel.sroa.0.0 = phi i32 [ 1, %bb7 ], [ 0, %bb4 ], !dbg !12217
  %45 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.063, i64 16, !dbg !12218
  %_23 = load i32, ptr %45, align 8, !dbg !12218, !noundef !12
  %parameter = zext i32 %_23 to i64, !dbg !12218
  %_104.1 = icmp slt i32 %_23, 0, !dbg !12220
  br i1 %_104.1, label %bb36, label %bb55, !dbg !12226, !prof !639

bb55:                                             ; preds = %bb9
  %_104.0 = shl nuw i32 %_23, 1, !dbg !12220
  %_110.0 = or disjoint i32 %_104.0, %channel.sroa.0.0, !dbg !12230
  %_31 = icmp ult i64 %iter.sroa.7.062, %_32, !dbg !12242
  %_34 = icmp samesign ult i32 %_23, 2
  %or.cond16 = and i1 %_34, %_31, !dbg !12242
  br i1 %or.cond16, label %bb11, label %bb36, !dbg !12242

bb11:                                             ; preds = %bb55
  %46 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.063, i64 28, !dbg !12244
  %_113 = load i32, ptr %46, align 4, !dbg !12244, !range !12248, !noundef !12
  %47 = icmp eq i32 %_113, 1, !dbg !12247
  br i1 %47, label %bb12, label %bb36, !dbg !12247

bb12:                                             ; preds = %bb11
  %_36 = load i64, ptr %iter.sroa.0.063, align 8, !dbg !12249, !noundef !12
  %_35 = icmp eq i64 %_36, %first_sample, !dbg !12249
  br i1 %_35, label %bb13, label %bb36, !dbg !12249

bb13:                                             ; preds = %bb12
  %48 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.063, i64 8, !dbg !12250
  %_38 = load i64, ptr %48, align 8, !dbg !12250, !noundef !12
  %_37 = icmp eq i64 %_38, %first_sample, !dbg !12250
  br i1 %_37, label %bb14, label %bb36, !dbg !12250

bb14:                                             ; preds = %bb13
  %49 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.063, i64 20, !dbg !12251
  %_41 = load float, ptr %49, align 4, !dbg !12251, !noundef !12
  %_40 = bitcast float %_41 to i32, !dbg !12252
  %50 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.063, i64 24, !dbg !12254
  %_4333 = load i32, ptr %50, align 8, !dbg !12254, !noundef !12
  %_39 = icmp eq i32 %_4333, %_40, !dbg !12251
  br i1 %_39, label %bb16, label %bb36, !dbg !12251

bb16:                                             ; preds = %bb14
  %_45 = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_ce93677d93a92959931541f74fa374eb, i64 %parameter, !dbg !12255
; call effect_runtime::params::parameter_value_valid
  %_44 = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_45, float noundef %_41) #31, !dbg !12256
  %51 = icmp ne i32 %_40, -2147483648
  %or.cond17.not = and i1 %51, %_44, !dbg !12256
  %_47 = icmp ugt i32 %_110.0, %last_order.sroa.3.0.ph68
  %or.cond18.not.not80 = select i1 %last_order.sroa.0.0.ph67.not, i1 true, i1 %_47, !dbg !12256
  %or.cond37.not = select i1 %or.cond17.not, i1 %or.cond18.not.not80, i1 false, !dbg !12256
  br i1 %or.cond37.not, label %bb30, label %bb36, !dbg !12256

bb30:                                             ; preds = %bb16
  %_49 = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi, i64 %parameter, !dbg !12257
  %52 = load i32, ptr %_49, align 4, !dbg !12258, !range !12132, !noundef !12
  %_116.not = icmp eq i32 %52, 0, !dbg !12266
  br i1 %_116.not, label %bb32, label %bb36, !dbg !12267

bb32:                                             ; preds = %bb30
  %_49.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi, i64 %parameter
  %53 = getelementptr inbounds nuw i8, ptr %_49.le, i64 4, !dbg !12258
  %_118 = fcmp oeq float %_41, 0.000000e+00, !dbg !12269
  %_57.sroa.0.0 = select i1 %_118, float 0.000000e+00, float %_41, !dbg !12269
  store i32 1, ptr %_49.le, align 4, !dbg !12274
  store float %_57.sroa.0.0, ptr %53, align 4, !dbg !12274
  %_6.i.i61 = icmp eq ptr %_16.i.i, %_93, !dbg !12105
  br i1 %_6.i.i61, label %bb5, label %bb4.lr.ph, !dbg !12115

bb36:                                             ; preds = %bb9, %bb30, %bb16, %bb14, %bb13, %bb12, %bb11, %bb55, %bb4
  %54 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted70, i64 1), !dbg !12275
  store i64 %54, ptr %1, align 8, !dbg !12217
  %_6.i.i = icmp eq ptr %_16.i.i, %_93, !dbg !12105
  br i1 %_6.i.i, label %bb5, label %bb4, !dbg !12115
}
