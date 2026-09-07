define internal void @_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb0_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_(ptr dead_on_unwind noalias noundef writable writeonly sret([40 x i8]) align 8 captures(none) dereferenceable(40) %_0, ptr noalias noundef align 8 dereferenceable(1232) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(88) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !6161 {
start:
  %iter.i.i = alloca [56 x i8], align 8
  %pending.i = alloca [64 x i8], align 4
  %reports.sroa.0 = alloca [16 x i8], align 8
  %reports.sroa.7 = alloca i64, align 8
  %reports.sroa.8 = alloca i64, align 8
  call void @llvm.lifetime.start.p0(ptr nonnull %reports.sroa.0), !dbg !6162
  call void @llvm.lifetime.start.p0(ptr nonnull %reports.sroa.7), !dbg !6162
  call void @llvm.lifetime.start.p0(ptr nonnull %reports.sroa.8), !dbg !6162
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %reports.sroa.0, i8 0, i64 16, i1 false)
  store i64 0, ptr %reports.sroa.7, align 8
  store i64 0, ptr %reports.sroa.8, align 8
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !6163
  %_12.0 = load ptr, ptr %0, align 8, !dbg !6163, !nonnull !12, !align !4797, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !6163
  %_12.1 = load i64, ptr %1, align 8, !dbg !6163, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %block, i64 80, !dbg !6165
  %_6 = load i64, ptr %2, align 8, !dbg !6165, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6166), !dbg !6169
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6170), !dbg !6169
  call void @llvm.lifetime.start.p0(ptr nonnull %pending.i), !dbg !6172, !noalias !6175
  store i32 0, ptr %pending.i, align 4, !noalias !6175
  %_7.sroa.5.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 4
  %_7.sroa.5134.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 8
  store i32 0, ptr %_7.sroa.5134.0.pending.sroa_idx.i, align 4, !noalias !6175
  %_7.sroa.6137.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 16
  store i32 0, ptr %_7.sroa.6137.0.pending.sroa_idx.i, align 4, !noalias !6175
  %_7.sroa.7140.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 24
  store i32 0, ptr %_7.sroa.7140.0.pending.sroa_idx.i, align 4, !noalias !6175
  %3 = getelementptr inbounds nuw i8, ptr %pending.i, i64 32
  store i32 0, ptr %3, align 4, !noalias !6175
  %_7.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 36
  %_7.sroa.5134.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 40
  store i32 0, ptr %_7.sroa.5134.0..sroa_idx.i, align 4, !noalias !6175
  %_7.sroa.6137.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 48
  store i32 0, ptr %_7.sroa.6137.0..sroa_idx.i, align 4, !noalias !6175
  %_7.sroa.7140.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 56
  store i32 0, ptr %_7.sroa.7140.0..sroa_idx.i, align 4, !noalias !6175
  %_106.idx.i = mul nuw nsw i64 %_12.1, 40, !dbg !6177
  %_106.i = getelementptr inbounds nuw i8, ptr %_12.0, i64 %_106.idx.i, !dbg !6177
  %_6.i.i8184.i = icmp eq i64 %_12.1, 0, !dbg !6188
  br i1 %_6.i.i8184.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, label %bb4.lr.ph.lr.ph.i, !dbg !6198

bb4.lr.ph.lr.ph.i:                                ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 92
  %_31.i = load i32, ptr %4, align 4, !alias.scope !6166, !noalias !6199
  %_30.i = zext i32 %_31.i to i64
  br label %bb4.lr.ph.i, !dbg !6198

bb4.lr.ph.i:                                      ; preds = %bb31.i, %bb4.lr.ph.lr.ph.i
  %.lcssa315318 = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %5, %bb31.i ]
  %.promoted91.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %.promoted90.i, %bb31.i ]
  %last_order.sroa.3.0.ph88.i = phi i32 [ undef, %bb4.lr.ph.lr.ph.i ], [ %_127.0.i, %bb31.i ]
  %last_order.sroa.0.0.ph87.not.i = phi i1 [ true, %bb4.lr.ph.lr.ph.i ], [ false, %bb31.i ]
  %iter.sroa.0.0.ph86.i = phi ptr [ %_12.0, %bb4.lr.ph.lr.ph.i ], [ %_16.i.i.i, %bb31.i ]
  %iter.sroa.7.0.ph85.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %bb4.i, !dbg !6198

bb4.i:                                            ; preds = %bb35.i, %bb4.lr.ph.i
  %5 = phi i64 [ %.lcssa315318, %bb4.lr.ph.i ], [ %89, %bb35.i ]
  %.promoted90.i = phi i64 [ %.promoted91.i, %bb4.lr.ph.i ], [ %89, %bb35.i ]
  %iter.sroa.0.083.i = phi ptr [ %iter.sroa.0.0.ph86.i, %bb4.lr.ph.i ], [ %_16.i.i.i, %bb35.i ]
  %iter.sroa.7.082.i = phi i64 [ %iter.sroa.7.0.ph85.i, %bb4.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 40, !dbg !6200
  %_9.0.i.i = add i64 %iter.sroa.7.082.i, 1, !dbg !6203
  %6 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 32, !dbg !6206
  %_17.i = load i32, ptr %6, align 8, !dbg !6206, !range !1335, !alias.scope !6170, !noalias !6208, !noundef !12
  switch i32 %_17.i, label %bb4.i.unreachabledefault [
    i32 1, label %bb9.i
    i32 2, label %bb7.i
    i32 3, label %bb35.i
  ], !dbg !6209

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i: ; preds = %bb46.us.3.i, %bb40.backedge.us.2.i
  %7 = load i32, ptr %3, align 4, !dbg !6210, !range !5852, !noalias !6175, !noundef !12
  %8 = trunc nuw i32 %7 to i1, !dbg !6215
  br i1 %8, label %bb46.us.1130.i, label %bb40.backedge.us.1131.i, !dbg !6215

bb46.us.1130.i:                                   ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 868
  %value.us.1118.i = load float, ptr %_7.sroa.5.0..sroa_idx.i, align 4, !dbg !6216, !noalias !6175, !noundef !12
  %_84.us.1119.i = load float, ptr %9, align 4, !dbg !6217, !alias.scope !6166, !noalias !6199, !noundef !12
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 872, !dbg !6220
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 876, !dbg !6221
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 880, !dbg !6222
  %_148.us.1120.i = bitcast float %_84.us.1119.i to i32, !dbg !6223
  %_150.us.1121.i = bitcast float %value.us.1118.i to i32, !dbg !6234
  %_149.us.1122.i = icmp ne i32 %_148.us.1120.i, %_150.us.1121.i, !dbg !6237
  %13 = icmp eq i32 %_148.us.1120.i, -2147483648
  %or.cond.us.1123.i = or i1 %_149.us.1122.i, %13, !dbg !6237
  %14 = tail call float @llvm.fabs.f32(float %_84.us.1119.i)
  %_144.us.1124.i = fcmp ueq float %14, 0x7FF0000000000000
  %or.cond40.us.1125.i = or i1 %_144.us.1124.i, %or.cond.us.1123.i, !dbg !6237
  %_146.us.1126.i = fsub float %value.us.1118.i, %_84.us.1119.i, !dbg !6237
  %15 = fmul float %_146.us.1126.i, 1.562500e-02, !dbg !6237
  %ramp.sroa.0.0.us.1127.i = select i1 %or.cond40.us.1125.i, float %_84.us.1119.i, float %value.us.1118.i, !dbg !6237
  %ramp4.sroa.0.0.us.1128.i = select i1 %or.cond40.us.1125.i, float %15, float 0.000000e+00, !dbg !6237
  %ramp5.sroa.0.0.us.1129.i = select i1 %or.cond40.us.1125.i, float 6.400000e+01, float 0.000000e+00, !dbg !6237
  store float %ramp.sroa.0.0.us.1127.i, ptr %9, align 4, !dbg !6238, !alias.scope !6240, !noalias !6199
  store float %value.us.1118.i, ptr %10, align 4, !dbg !6243, !alias.scope !6245, !noalias !6199
  store float %ramp4.sroa.0.0.us.1128.i, ptr %11, align 4, !dbg !6248, !alias.scope !6250, !noalias !6199
  store float %ramp5.sroa.0.0.us.1129.i, ptr %12, align 4, !dbg !6253, !alias.scope !6255, !noalias !6199
  store i32 64, ptr %44, align 4, !dbg !6258, !alias.scope !6166, !noalias !6199
  br label %bb40.backedge.us.1131.i, !dbg !6259

bb40.backedge.us.1131.i:                          ; preds = %bb46.us.1130.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i
  %16 = load i32, ptr %_7.sroa.5134.0..sroa_idx.i, align 4, !dbg !6210, !range !5852, !noalias !6175, !noundef !12
  %17 = trunc nuw i32 %16 to i1, !dbg !6215
  br i1 %17, label %bb46.us.1.1.i, label %bb40.backedge.us.1.1.i, !dbg !6215

bb46.us.1.1.i:                                    ; preds = %bb40.backedge.us.1131.i
  %18 = getelementptr inbounds nuw i8, ptr %pending.i, i64 44, !dbg !6210
  %value.us.1.1.i = load float, ptr %18, align 4, !dbg !6216, !noalias !6175, !noundef !12
  %slot.us.1.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884, !dbg !6260
  %_84.us.1.1.i = load float, ptr %slot.us.1.1.i, align 4, !dbg !6217, !alias.scope !6166, !noalias !6199, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %self, i64 888, !dbg !6220
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 892, !dbg !6221
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 896, !dbg !6222
  %_148.us.1.1.i = bitcast float %_84.us.1.1.i to i32, !dbg !6223
  %_150.us.1.1.i = bitcast float %value.us.1.1.i to i32, !dbg !6234
  %_149.us.1.1.i = icmp ne i32 %_148.us.1.1.i, %_150.us.1.1.i, !dbg !6237
  %22 = icmp eq i32 %_148.us.1.1.i, -2147483648
  %or.cond.us.1.1.i = or i1 %_149.us.1.1.i, %22, !dbg !6237
  %23 = tail call float @llvm.fabs.f32(float %_84.us.1.1.i)
  %_144.us.1.1.i = fcmp ueq float %23, 0x7FF0000000000000
  %or.cond40.us.1.1.i = or i1 %_144.us.1.1.i, %or.cond.us.1.1.i, !dbg !6237
  %_146.us.1.1.i = fsub float %value.us.1.1.i, %_84.us.1.1.i, !dbg !6237
  %24 = fmul float %_146.us.1.1.i, 1.562500e-02, !dbg !6237
  %ramp.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float %_84.us.1.1.i, float %value.us.1.1.i, !dbg !6237
  %ramp4.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float %24, float 0.000000e+00, !dbg !6237
  %ramp5.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !6237
  store float %ramp.sroa.0.0.us.1.1.i, ptr %slot.us.1.1.i, align 4, !dbg !6238, !alias.scope !6240, !noalias !6199
  store float %value.us.1.1.i, ptr %19, align 4, !dbg !6243, !alias.scope !6245, !noalias !6199
  store float %ramp4.sroa.0.0.us.1.1.i, ptr %20, align 4, !dbg !6248, !alias.scope !6250, !noalias !6199
  store float %ramp5.sroa.0.0.us.1.1.i, ptr %21, align 4, !dbg !6253, !alias.scope !6255, !noalias !6199
  store i32 64, ptr %44, align 4, !dbg !6258, !alias.scope !6166, !noalias !6199
  br label %bb40.backedge.us.1.1.i, !dbg !6259

bb40.backedge.us.1.1.i:                           ; preds = %bb46.us.1.1.i, %bb40.backedge.us.1131.i
  %25 = load i32, ptr %_7.sroa.6137.0..sroa_idx.i, align 4, !dbg !6210, !range !5852, !noalias !6175, !noundef !12
  %26 = trunc nuw i32 %25 to i1, !dbg !6215
  br i1 %26, label %bb46.us.2.1.i, label %bb40.backedge.us.2.1.i, !dbg !6215

bb46.us.2.1.i:                                    ; preds = %bb40.backedge.us.1.1.i
  %27 = getelementptr inbounds nuw i8, ptr %pending.i, i64 52, !dbg !6210
  %value.us.2.1.i = load float, ptr %27, align 4, !dbg !6216, !noalias !6175, !noundef !12
  %slot.us.2.1.i = getelementptr inbounds nuw i8, ptr %self, i64 900, !dbg !6260
  %_84.us.2.1.i = load float, ptr %slot.us.2.1.i, align 4, !dbg !6217, !alias.scope !6166, !noalias !6199, !noundef !12
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 904, !dbg !6220
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 908, !dbg !6221
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 912, !dbg !6222
  %_148.us.2.1.i = bitcast float %_84.us.2.1.i to i32, !dbg !6223
  %_150.us.2.1.i = bitcast float %value.us.2.1.i to i32, !dbg !6234
  %_149.us.2.1.i = icmp ne i32 %_148.us.2.1.i, %_150.us.2.1.i, !dbg !6237
  %31 = icmp eq i32 %_148.us.2.1.i, -2147483648
  %or.cond.us.2.1.i = or i1 %_149.us.2.1.i, %31, !dbg !6237
  %32 = tail call float @llvm.fabs.f32(float %_84.us.2.1.i)
  %_144.us.2.1.i = fcmp ueq float %32, 0x7FF0000000000000
  %or.cond40.us.2.1.i = or i1 %_144.us.2.1.i, %or.cond.us.2.1.i, !dbg !6237
  %_146.us.2.1.i = fsub float %value.us.2.1.i, %_84.us.2.1.i, !dbg !6237
  %33 = fmul float %_146.us.2.1.i, 1.562500e-02, !dbg !6237
  %ramp.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float %_84.us.2.1.i, float %value.us.2.1.i, !dbg !6237
  %ramp4.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float %33, float 0.000000e+00, !dbg !6237
  %ramp5.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !6237
  store float %ramp.sroa.0.0.us.2.1.i, ptr %slot.us.2.1.i, align 4, !dbg !6238, !alias.scope !6240, !noalias !6199
  store float %value.us.2.1.i, ptr %28, align 4, !dbg !6243, !alias.scope !6245, !noalias !6199
  store float %ramp4.sroa.0.0.us.2.1.i, ptr %29, align 4, !dbg !6248, !alias.scope !6250, !noalias !6199
  store float %ramp5.sroa.0.0.us.2.1.i, ptr %30, align 4, !dbg !6253, !alias.scope !6255, !noalias !6199
  store i32 64, ptr %44, align 4, !dbg !6258, !alias.scope !6166, !noalias !6199
  br label %bb40.backedge.us.2.1.i, !dbg !6259

bb40.backedge.us.2.1.i:                           ; preds = %bb46.us.2.1.i, %bb40.backedge.us.1.1.i
  %34 = load i32, ptr %_7.sroa.7140.0..sroa_idx.i, align 4, !dbg !6210, !range !5852, !noalias !6175, !noundef !12
  %35 = trunc nuw i32 %34 to i1, !dbg !6215
  br i1 %35, label %bb46.us.3.1.i, label %bb40.backedge.us.2.1.i._RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit_crit_edge, !dbg !6215

bb40.backedge.us.2.1.i._RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit_crit_edge: ; preds = %bb40.backedge.us.2.1.i
  %_9.i.pre = load i32, ptr %44, align 4, !dbg !6261, !alias.scope !6267, !noalias !6270
  %36 = zext i32 %_9.i.pre to i64, !dbg !6275
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit, !dbg !6215

bb46.us.3.1.i:                                    ; preds = %bb40.backedge.us.2.1.i
  %37 = getelementptr inbounds nuw i8, ptr %pending.i, i64 60, !dbg !6210
  %value.us.3.1.i = load float, ptr %37, align 4, !dbg !6216, !noalias !6175, !noundef !12
  %slot.us.3.1.i = getelementptr inbounds nuw i8, ptr %self, i64 916, !dbg !6260
  %_84.us.3.1.i = load float, ptr %slot.us.3.1.i, align 4, !dbg !6217, !alias.scope !6166, !noalias !6199, !noundef !12
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 920, !dbg !6220
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 924, !dbg !6221
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 928, !dbg !6222
  %_148.us.3.1.i = bitcast float %_84.us.3.1.i to i32, !dbg !6223
  %_150.us.3.1.i = bitcast float %value.us.3.1.i to i32, !dbg !6234
  %_149.us.3.1.i = icmp ne i32 %_148.us.3.1.i, %_150.us.3.1.i, !dbg !6237
  %41 = icmp eq i32 %_148.us.3.1.i, -2147483648
  %or.cond.us.3.1.i = or i1 %_149.us.3.1.i, %41, !dbg !6237
  %42 = tail call float @llvm.fabs.f32(float %_84.us.3.1.i)
  %_144.us.3.1.i = fcmp ueq float %42, 0x7FF0000000000000
  %or.cond40.us.3.1.i = or i1 %_144.us.3.1.i, %or.cond.us.3.1.i, !dbg !6237
  %_146.us.3.1.i = fsub float %value.us.3.1.i, %_84.us.3.1.i, !dbg !6237
  %43 = fmul float %_146.us.3.1.i, 1.562500e-02, !dbg !6237
  %ramp.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float %_84.us.3.1.i, float %value.us.3.1.i, !dbg !6237
  %ramp4.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float %43, float 0.000000e+00, !dbg !6237
  %ramp5.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !6237
  store float %ramp.sroa.0.0.us.3.1.i, ptr %slot.us.3.1.i, align 4, !dbg !6238, !alias.scope !6240, !noalias !6199
  store float %value.us.3.1.i, ptr %38, align 4, !dbg !6243, !alias.scope !6245, !noalias !6199
  store float %ramp4.sroa.0.0.us.3.1.i, ptr %39, align 4, !dbg !6248, !alias.scope !6250, !noalias !6199
  store float %ramp5.sroa.0.0.us.3.1.i, ptr %40, align 4, !dbg !6253, !alias.scope !6255, !noalias !6199
  store i32 64, ptr %44, align 4, !dbg !6258, !alias.scope !6166, !noalias !6199
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit, !dbg !6259

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i: ; preds = %bb31.i, %bb35.i, %start
  %reports.sroa.4.0 = phi i64 [ 0, %start ], [ %89, %bb35.i ], [ %5, %bb31.i ]
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 1220
  %45 = load i32, ptr %pending.i, align 4, !dbg !6210, !range !5852, !noalias !6175, !noundef !12
  %46 = trunc nuw i32 %45 to i1, !dbg !6215
  br i1 %46, label %bb46.us.i, label %bb40.backedge.us.i, !dbg !6215

bb46.us.i:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 792
  %value.us.i = load float, ptr %_7.sroa.5.0.pending.sroa_idx.i, align 4, !dbg !6216, !noalias !6175, !noundef !12
  %_84.us.i = load float, ptr %47, align 4, !dbg !6217, !alias.scope !6166, !noalias !6199, !noundef !12
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 796, !dbg !6220
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 800, !dbg !6221
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 804, !dbg !6222
  %_148.us.i = bitcast float %_84.us.i to i32, !dbg !6223
  %_150.us.i = bitcast float %value.us.i to i32, !dbg !6234
  %_149.us.i = icmp ne i32 %_148.us.i, %_150.us.i, !dbg !6237
  %51 = icmp eq i32 %_148.us.i, -2147483648
  %or.cond.us.i = or i1 %_149.us.i, %51, !dbg !6237
  %52 = tail call float @llvm.fabs.f32(float %_84.us.i)
  %_144.us.i = fcmp ueq float %52, 0x7FF0000000000000
  %or.cond40.us.i = or i1 %_144.us.i, %or.cond.us.i, !dbg !6237
  %_146.us.i = fsub float %value.us.i, %_84.us.i, !dbg !6237
  %53 = fmul float %_146.us.i, 1.562500e-02, !dbg !6237
  %ramp.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %_84.us.i, float %value.us.i, !dbg !6237
  %ramp4.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %53, float 0.000000e+00, !dbg !6237
  %ramp5.sroa.0.0.us.i = select i1 %or.cond40.us.i, float 6.400000e+01, float 0.000000e+00, !dbg !6237
  store float %ramp.sroa.0.0.us.i, ptr %47, align 4, !dbg !6238, !alias.scope !6240, !noalias !6199
  store float %value.us.i, ptr %48, align 4, !dbg !6243, !alias.scope !6245, !noalias !6199
  store float %ramp4.sroa.0.0.us.i, ptr %49, align 4, !dbg !6248, !alias.scope !6250, !noalias !6199
  store float %ramp5.sroa.0.0.us.i, ptr %50, align 4, !dbg !6253, !alias.scope !6255, !noalias !6199
  store i32 64, ptr %44, align 4, !dbg !6258, !alias.scope !6166, !noalias !6199
  br label %bb40.backedge.us.i, !dbg !6259

bb40.backedge.us.i:                               ; preds = %bb46.us.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %54 = load i32, ptr %_7.sroa.5134.0.pending.sroa_idx.i, align 4, !dbg !6210, !range !5852, !noalias !6175, !noundef !12
  %55 = trunc nuw i32 %54 to i1, !dbg !6215
  br i1 %55, label %bb46.us.1.i, label %bb40.backedge.us.1.i, !dbg !6215

bb46.us.1.i:                                      ; preds = %bb40.backedge.us.i
  %56 = getelementptr inbounds nuw i8, ptr %pending.i, i64 12, !dbg !6210
  %value.us.1.i = load float, ptr %56, align 4, !dbg !6216, !noalias !6175, !noundef !12
  %slot.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808, !dbg !6260
  %_84.us.1.i = load float, ptr %slot.us.1.i, align 4, !dbg !6217, !alias.scope !6166, !noalias !6199, !noundef !12
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 812, !dbg !6220
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 816, !dbg !6221
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 820, !dbg !6222
  %_148.us.1.i = bitcast float %_84.us.1.i to i32, !dbg !6223
  %_150.us.1.i = bitcast float %value.us.1.i to i32, !dbg !6234
  %_149.us.1.i = icmp ne i32 %_148.us.1.i, %_150.us.1.i, !dbg !6237
  %60 = icmp eq i32 %_148.us.1.i, -2147483648
  %or.cond.us.1.i = or i1 %_149.us.1.i, %60, !dbg !6237
  %61 = tail call float @llvm.fabs.f32(float %_84.us.1.i)
  %_144.us.1.i = fcmp ueq float %61, 0x7FF0000000000000
  %or.cond40.us.1.i = or i1 %_144.us.1.i, %or.cond.us.1.i, !dbg !6237
  %_146.us.1.i = fsub float %value.us.1.i, %_84.us.1.i, !dbg !6237
  %62 = fmul float %_146.us.1.i, 1.562500e-02, !dbg !6237
  %ramp.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %_84.us.1.i, float %value.us.1.i, !dbg !6237
  %ramp4.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %62, float 0.000000e+00, !dbg !6237
  %ramp5.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !6237
  store float %ramp.sroa.0.0.us.1.i, ptr %slot.us.1.i, align 4, !dbg !6238, !alias.scope !6240, !noalias !6199
  store float %value.us.1.i, ptr %57, align 4, !dbg !6243, !alias.scope !6245, !noalias !6199
  store float %ramp4.sroa.0.0.us.1.i, ptr %58, align 4, !dbg !6248, !alias.scope !6250, !noalias !6199
  store float %ramp5.sroa.0.0.us.1.i, ptr %59, align 4, !dbg !6253, !alias.scope !6255, !noalias !6199
  store i32 64, ptr %44, align 4, !dbg !6258, !alias.scope !6166, !noalias !6199
  br label %bb40.backedge.us.1.i, !dbg !6259

bb40.backedge.us.1.i:                             ; preds = %bb46.us.1.i, %bb40.backedge.us.i
  %63 = load i32, ptr %_7.sroa.6137.0.pending.sroa_idx.i, align 4, !dbg !6210, !range !5852, !noalias !6175, !noundef !12
  %64 = trunc nuw i32 %63 to i1, !dbg !6215
  br i1 %64, label %bb46.us.2.i, label %bb40.backedge.us.2.i, !dbg !6215

bb46.us.2.i:                                      ; preds = %bb40.backedge.us.1.i
  %65 = getelementptr inbounds nuw i8, ptr %pending.i, i64 20, !dbg !6210
  %value.us.2.i = load float, ptr %65, align 4, !dbg !6216, !noalias !6175, !noundef !12
  %slot.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824, !dbg !6260
  %_84.us.2.i = load float, ptr %slot.us.2.i, align 4, !dbg !6217, !alias.scope !6166, !noalias !6199, !noundef !12
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 828, !dbg !6220
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 832, !dbg !6221
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 836, !dbg !6222
  %_148.us.2.i = bitcast float %_84.us.2.i to i32, !dbg !6223
  %_150.us.2.i = bitcast float %value.us.2.i to i32, !dbg !6234
  %_149.us.2.i = icmp ne i32 %_148.us.2.i, %_150.us.2.i, !dbg !6237
  %69 = icmp eq i32 %_148.us.2.i, -2147483648
  %or.cond.us.2.i = or i1 %_149.us.2.i, %69, !dbg !6237
  %70 = tail call float @llvm.fabs.f32(float %_84.us.2.i)
  %_144.us.2.i = fcmp ueq float %70, 0x7FF0000000000000
  %or.cond40.us.2.i = or i1 %_144.us.2.i, %or.cond.us.2.i, !dbg !6237
  %_146.us.2.i = fsub float %value.us.2.i, %_84.us.2.i, !dbg !6237
  %71 = fmul float %_146.us.2.i, 1.562500e-02, !dbg !6237
  %ramp.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %_84.us.2.i, float %value.us.2.i, !dbg !6237
  %ramp4.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %71, float 0.000000e+00, !dbg !6237
  %ramp5.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float 6.400000e+01, float 0.000000e+00, !dbg !6237
  store float %ramp.sroa.0.0.us.2.i, ptr %slot.us.2.i, align 4, !dbg !6238, !alias.scope !6240, !noalias !6199
  store float %value.us.2.i, ptr %66, align 4, !dbg !6243, !alias.scope !6245, !noalias !6199
  store float %ramp4.sroa.0.0.us.2.i, ptr %67, align 4, !dbg !6248, !alias.scope !6250, !noalias !6199
  store float %ramp5.sroa.0.0.us.2.i, ptr %68, align 4, !dbg !6253, !alias.scope !6255, !noalias !6199
  store i32 64, ptr %44, align 4, !dbg !6258, !alias.scope !6166, !noalias !6199
  br label %bb40.backedge.us.2.i, !dbg !6259

bb40.backedge.us.2.i:                             ; preds = %bb46.us.2.i, %bb40.backedge.us.1.i
  %72 = load i32, ptr %_7.sroa.7140.0.pending.sroa_idx.i, align 4, !dbg !6210, !range !5852, !noalias !6175, !noundef !12
  %73 = trunc nuw i32 %72 to i1, !dbg !6215
  br i1 %73, label %bb46.us.3.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i, !dbg !6215

bb46.us.3.i:                                      ; preds = %bb40.backedge.us.2.i
  %74 = getelementptr inbounds nuw i8, ptr %pending.i, i64 28, !dbg !6210
  %value.us.3.i = load float, ptr %74, align 4, !dbg !6216, !noalias !6175, !noundef !12
  %slot.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !6260
  %_84.us.3.i = load float, ptr %slot.us.3.i, align 4, !dbg !6217, !alias.scope !6166, !noalias !6199, !noundef !12
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 844, !dbg !6220
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 848, !dbg !6221
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 852, !dbg !6222
  %_148.us.3.i = bitcast float %_84.us.3.i to i32, !dbg !6223
  %_150.us.3.i = bitcast float %value.us.3.i to i32, !dbg !6234
  %_149.us.3.i = icmp ne i32 %_148.us.3.i, %_150.us.3.i, !dbg !6237
  %78 = icmp eq i32 %_148.us.3.i, -2147483648
  %or.cond.us.3.i = or i1 %_149.us.3.i, %78, !dbg !6237
  %79 = tail call float @llvm.fabs.f32(float %_84.us.3.i)
  %_144.us.3.i = fcmp ueq float %79, 0x7FF0000000000000
  %or.cond40.us.3.i = or i1 %_144.us.3.i, %or.cond.us.3.i, !dbg !6237
  %_146.us.3.i = fsub float %value.us.3.i, %_84.us.3.i, !dbg !6237
  %80 = fmul float %_146.us.3.i, 1.562500e-02, !dbg !6237
  %ramp.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %_84.us.3.i, float %value.us.3.i, !dbg !6237
  %ramp4.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %80, float 0.000000e+00, !dbg !6237
  %ramp5.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float 6.400000e+01, float 0.000000e+00, !dbg !6237
  store float %ramp.sroa.0.0.us.3.i, ptr %slot.us.3.i, align 4, !dbg !6238, !alias.scope !6240, !noalias !6199
  store float %value.us.3.i, ptr %75, align 4, !dbg !6243, !alias.scope !6245, !noalias !6199
  store float %ramp4.sroa.0.0.us.3.i, ptr %76, align 4, !dbg !6248, !alias.scope !6250, !noalias !6199
  store float %ramp5.sroa.0.0.us.3.i, ptr %77, align 4, !dbg !6253, !alias.scope !6255, !noalias !6199
  store i32 64, ptr %44, align 4, !dbg !6258, !alias.scope !6166, !noalias !6199
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i, !dbg !6259

bb4.i.unreachabledefault:                         ; preds = %bb4.i
  unreachable

default.unreachable:                              ; preds = %bb50.i.us.us.us.i.i, %bb53.i.us.i.i, %bb50.i.us.us.us.i55.i, %bb53.i.us.i207.i, %bb53.i.i.i
  unreachable

bb7.i:                                            ; preds = %bb4.i
  br label %bb9.i, !dbg !6276

bb9.i:                                            ; preds = %bb4.i, %bb7.i
  %channel.sroa.0.0.sroa.phi28.i = phi ptr [ %3, %bb7.i ], [ %pending.i, %bb4.i ], !dbg !6277
  %channel.sroa.0.0.i = phi i32 [ 1, %bb7.i ], [ 0, %bb4.i ], !dbg !6277
  %81 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 16, !dbg !6278
  %_21.i = load i32, ptr %81, align 8, !dbg !6278, !alias.scope !6170, !noalias !6208, !noundef !12
  %parameter_index.i = zext i32 %_21.i to i64, !dbg !6278
  %_121.1.i = icmp slt i32 %_21.i, 0, !dbg !6280
  br i1 %_121.1.i, label %bb35.i, label %bb59.i, !dbg !6286, !prof !180

bb59.i:                                           ; preds = %bb9.i
  %_121.0.i = shl nuw i32 %_21.i, 1, !dbg !6280
  %_127.0.i = or disjoint i32 %_121.0.i, %channel.sroa.0.0.i, !dbg !6292
  %_29.i = icmp ult i64 %iter.sroa.7.082.i, %_30.i, !dbg !6301
  %_32.i = icmp samesign ult i32 %_21.i, 4
  %or.cond16.i = and i1 %_29.i, %_32.i, !dbg !6301
  br i1 %or.cond16.i, label %bb11.i, label %bb35.i, !dbg !6301

bb11.i:                                           ; preds = %bb59.i
  %82 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 28, !dbg !6303
  %_130.i = load i32, ptr %82, align 4, !dbg !6303, !range !6307, !alias.scope !6170, !noalias !6208, !noundef !12
  %83 = icmp eq i32 %_130.i, 1, !dbg !6308
  br i1 %83, label %bb12.i, label %bb35.i, !dbg !6308

bb12.i:                                           ; preds = %bb11.i
  %_34.i = load i64, ptr %iter.sroa.0.083.i, align 8, !dbg !6309, !alias.scope !6170, !noalias !6208, !noundef !12
  %_33.i = icmp eq i64 %_34.i, %_6, !dbg !6309
  br i1 %_33.i, label %bb13.i, label %bb35.i, !dbg !6309

bb13.i:                                           ; preds = %bb12.i
  %84 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 8, !dbg !6310
  %_36.i = load i64, ptr %84, align 8, !dbg !6310, !alias.scope !6170, !noalias !6208, !noundef !12
  %_35.i = icmp eq i64 %_36.i, %_6, !dbg !6310
  br i1 %_35.i, label %bb14.i, label %bb35.i, !dbg !6310

bb14.i:                                           ; preds = %bb13.i
  %85 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 20, !dbg !6311
  %_39.i = load float, ptr %85, align 4, !dbg !6311, !alias.scope !6170, !noalias !6208, !noundef !12
  %_38.i = bitcast float %_39.i to i32, !dbg !6312
  %86 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 24, !dbg !6314
  %_4139.i = load i32, ptr %86, align 8, !dbg !6314, !alias.scope !6170, !noalias !6208, !noundef !12
  %_37.i = icmp eq i32 %_4139.i, %_38.i, !dbg !6311
  br i1 %_37.i, label %bb16.i, label %bb35.i, !dbg !6311

bb16.i:                                           ; preds = %bb14.i
  %_43.i = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_d0058eaee52f1ba8b2e5577cef0c3c7f, i64 %parameter_index.i, !dbg !6315
; call effect_runtime::params::parameter_value_valid
  %_42.i = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_43.i, float noundef %_39.i) #23, !dbg !6316, !noalias !6175
  %_45.i = icmp ugt i32 %_127.0.i, %last_order.sroa.3.0.ph88.i
  %or.cond17.not.not102.i = select i1 %last_order.sroa.0.0.ph87.not.i, i1 true, i1 %_45.i, !dbg !6316
  %or.cond41.not.i = select i1 %_42.i, i1 %or.cond17.not.not102.i, i1 false, !dbg !6316
  br i1 %or.cond41.not.i, label %bb29.i, label %bb35.i, !dbg !6316

bb29.i:                                           ; preds = %bb16.i
  %_47.i = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i, !dbg !6317
  %87 = load i32, ptr %_47.i, align 4, !dbg !6318, !range !5852, !noalias !6175, !noundef !12
  %_133.not.i = icmp eq i32 %87, 0, !dbg !6325
  br i1 %_133.not.i, label %bb31.i, label %bb35.i, !dbg !6326

bb31.i:                                           ; preds = %bb29.i
  %_47.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i
  %_135.i = fcmp oeq float %_39.i, 0.000000e+00, !dbg !6328
  %_55.sroa.0.0.i = select i1 %_135.i, float 0.000000e+00, float %_39.i, !dbg !6328
  store i32 1, ptr %_47.i.le, align 4, !dbg !6331, !noalias !6175
  %88 = getelementptr inbounds nuw i8, ptr %_47.i.le, i64 4, !dbg !6331
  store float %_55.sroa.0.0.i, ptr %88, align 4, !dbg !6331, !noalias !6175
  %_6.i.i81.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !6188
  br i1 %_6.i.i81.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, label %bb4.lr.ph.i, !dbg !6198

bb35.i:                                           ; preds = %bb4.i, %bb29.i, %bb16.i, %bb14.i, %bb13.i, %bb12.i, %bb11.i, %bb59.i, %bb9.i
  %89 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted90.i, i64 1), !dbg !6332
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !6188
  br i1 %_6.i.i.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, label %bb4.i, !dbg !6198

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit: ; preds = %bb40.backedge.us.2.1.i._RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit_crit_edge, %bb46.us.3.1.i
  %_9.i = phi i64 [ %36, %bb40.backedge.us.2.1.i._RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit_crit_edge ], [ 64, %bb46.us.3.1.i ], !dbg !6261
  call void @llvm.lifetime.end.p0(ptr nonnull %pending.i), !dbg !6335, !noalias !6175
  %_14.0 = load ptr, ptr %block, align 8, !dbg !6336, !nonnull !12, !align !3533, !noundef !12
  %90 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !6336
  %_14.1 = load i64, ptr %90, align 8, !dbg !6336, !noundef !12
  %91 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !6340
  %_13.0 = load ptr, ptr %91, align 8, !dbg !6340, !nonnull !12, !align !3533, !noundef !12
  %92 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !6340
  %_13.1 = load i64, ptr %92, align 8, !dbg !6340, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6267), !dbg !6341
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6342), !dbg !6341
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6343), !dbg !6341
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6344), !dbg !6341
  %..i.i = tail call noundef i64 @llvm.umin.i64(i64 %_14.1, i64 %_9.i), !dbg !6345
  %_10.not.i = icmp eq i64 %..i.i, 0, !dbg !6347
  br i1 %_10.not.i, label %bb4.i4, label %bb9.i2, !dbg !6347

bb4.i4:                                           ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKb1_EB3_.exit.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit
  %_18.i = icmp ugt i64 %_14.1, %_9.i, !dbg !6349
  br i1 %_18.i, label %bb20.i, label %bb7.i5, !dbg !6349

bb9.i2:                                           ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit
  %_41.not.i = icmp samesign ugt i64 %..i.i, %_13.1, !dbg !6350
  br i1 %_41.not.i, label %bb19.i, label %bb18.i, !dbg !6350, !prof !180

bb19.i:                                           ; preds = %bb9.i2
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i.i, i64 noundef range(i64 0, 2305843009213693952) %_13.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1d7cc6e40c752396aa7def7556a6c433) #24, !dbg !6361, !noalias !6270
  unreachable, !dbg !6361

bb18.i:                                           ; preds = %bb9.i2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6362), !dbg !6365
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6366), !dbg !6365
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6368), !dbg !6365
  %_10.i.i = getelementptr inbounds nuw i8, ptr %self, i64 792, !dbg !6370
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !6373
  %_15.i.i = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !6378
  %data.i.i540.i.i = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !6380
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 68, !dbg !6385
  %_29.i.i = load i32, ptr %93, align 4, !dbg !6385, !range !1335, !alias.scope !6389, !noalias !6390, !noundef !12
  %_31.i.i = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !6392
  %_33.i.i = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !6393
  %94 = icmp eq i32 %_29.i.i, 1, !dbg !6394
  %.val.i.i.i.i = load i32, ptr %_31.i.i, align 4, !dbg !6394, !alias.scope !6389, !noalias !6390
  %.val1.i.i.i.i = load i32, ptr %_33.i.i, align 4, !dbg !6394, !alias.scope !6389, !noalias !6390
  %_0.i.i.not.i.i.i.i = icmp eq i32 %.val.i.i.i.i, %.val1.i.i.i.i, !dbg !6394
  %spec.select.i.i = select i1 %_0.i.i.not.i.i.i.i, i8 1, i8 2, !dbg !6394
  %_0.sroa.0.0.i545.i.i = select i1 %94, i8 0, i8 %spec.select.i.i, !dbg !6394
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 744, !dbg !6396
  %_38.i.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !6398
  %_64.0.i.i = load ptr, ptr %_15.i.i, align 8, !dbg !6399, !alias.scope !6389, !noalias !6390, !nonnull !12, !noundef !12
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !6399
  %_64.1.i.i = load i64, ptr %96, align 8, !dbg !6399, !alias.scope !6389, !noalias !6390, !noundef !12
  %_66.0.i.i = load ptr, ptr %data.i.i540.i.i, align 8, !dbg !6400, !alias.scope !6389, !noalias !6390, !nonnull !12, !noundef !12
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !6400
  %_66.1.i.i = load i64, ptr %97, align 8, !dbg !6400, !alias.scope !6389, !noalias !6390, !noundef !12
  %_57.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !6401
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !6402
  %_58.i.i = load i32, ptr %98, align 4, !dbg !6402, !alias.scope !6389, !noalias !6390, !noundef !12
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !6403
  %_59.i.i = load i32, ptr %99, align 8, !dbg !6403, !alias.scope !6389, !noalias !6390, !noundef !12
  %base.i.i.i = load i32, ptr %_57.i.i, align 4, !dbg !6404, !alias.scope !6389, !noalias !6414, !noundef !12
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 856
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 860
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 932
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 784
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 788
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 936
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 776
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 940
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %injected.cond708.not.i.i = icmp ugt i64 %_64.1.i.i, %_66.1.i.i
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i26.i.us.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i26.i.us.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i26.i.us.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %125 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %126 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %128 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %129 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %130 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.i.us.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %131 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %132 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %133 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.i.us.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %134 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %135 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %136 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.i.us.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %137 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %138 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %139 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br i1 %injected.cond708.not.i.i, label %bb32.i.us.i.i, label %bb32.i.us.us.us.i.i

bb32.i.us.us.us.i.i:                              ; preds = %bb18.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i.i
  %iter.sroa.0.0.i690.us.us.us.i.i = phi i64 [ %140, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i.i ], [ 0, %bb18.i ]
  %140 = add nuw nsw i64 %iter.sroa.0.0.i690.us.us.us.i.i, 1, !dbg !6417
  %_28.i.us.us.us.i.i = trunc i64 %iter.sroa.0.0.i690.us.us.us.i.i to i32, !dbg !6431
  %now.i.us.us.us.i.i = add i32 %base.i.i.i, %_28.i.us.us.us.i.i, !dbg !6434
  %_31.i.us.us.us.i.i = and i32 %now.i.us.us.us.i.i, %_58.i.i, !dbg !6437
  %_30.i.us.us.us.i.i = zext i32 %_31.i.us.us.us.i.i to i64, !dbg !6439
  %_97.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_14.0, i64 %iter.sroa.0.0.i690.us.us.us.i.i, !dbg !6440
  %_98.not.not.i.us.us.us.i.i = icmp ugt i64 %_64.1.i.i, %_30.i.us.us.us.i.i, !dbg !6449
  br i1 %_98.not.not.i.us.us.us.i.i, label %bb35.i.us.us.us.i.i, label %bb36.i.i.i, !dbg !6449, !prof !2709

bb35.i.us.us.us.i.i:                              ; preds = %bb32.i.us.us.us.i.i
  %_0.i207.us.us.us.i.i = load float, ptr %_97.i.us.us.us.i.i, align 4, !dbg !6454, !alias.scope !6456, !noalias !6459, !noundef !12
  %_107.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_30.i.us.us.us.i.i, !dbg !6460
  store float %_0.i207.us.us.us.i.i, ptr %_107.i.us.us.us.i.i, align 4, !dbg !6464, !alias.scope !6466, !noalias !6414
  %_115.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_13.0, i64 %iter.sroa.0.0.i690.us.us.us.i.i, !dbg !6469
  %_0.i205.us.us.us.i.i = load float, ptr %_115.i.us.us.us.i.i, align 4, !dbg !6476, !alias.scope !6478, !noalias !6481, !noundef !12
  %_123.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_30.i.us.us.us.i.i, !dbg !6482
  store float %_0.i205.us.us.us.i.i, ptr %_123.i.us.us.us.i.i, align 4, !dbg !6489, !alias.scope !6491, !noalias !6414
  %_53.i.us.us.us.i.i = sub i32 %now.i.us.us.us.i.i, %_59.i.i, !dbg !6494
  %_52.i.us.us.us.i.i = and i32 %_53.i.us.us.us.i.i, %_58.i.i, !dbg !6497
  %_51.i.us.us.us.i.i = zext i32 %_52.i.us.us.us.i.i to i64, !dbg !6498
  %_156.not.not.i.us.us.us.i.i = icmp ugt i64 %_64.1.i.i, %_51.i.us.us.us.i.i, !dbg !6499
  br i1 %_156.not.not.i.us.us.us.i.i, label %bb50.i.us.us.us.i.i, label %bb51.i.i.i, !dbg !6499, !prof !2709

bb50.i.us.us.us.i.i:                              ; preds = %bb35.i.us.us.us.i.i
  %_163.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_51.i.us.us.us.i.i, !dbg !6504
  %_0.i203.us.us.us.i.i = load float, ptr %_163.i.us.us.us.i.i, align 4, !dbg !6508, !alias.scope !6510, !noalias !6414, !noundef !12
  %_169.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_51.i.us.us.us.i.i, !dbg !6513
  %_0.i201.us.us.us.i.i = load float, ptr %_169.i.us.us.us.i.i, align 4, !dbg !6521, !alias.scope !6523, !noalias !6414, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6526), !dbg !6529
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6532), !dbg !6529
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6534), !dbg !6529
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6536), !dbg !6529
  %_18.i72.us.us.us.i.i = load i32, ptr %_31.i.i, align 4, !dbg !6538, !alias.scope !6540, !noalias !6541, !noundef !12
  %_17.i.us.us.us.i.i = sub i32 %now.i.us.us.us.i.i, %_18.i72.us.us.us.i.i, !dbg !6543
  %_16.i73.us.us.us.i.i = and i32 %_17.i.us.us.us.i.i, %_58.i.i, !dbg !6538
  %_15.i74.us.us.us.i.i = zext i32 %_16.i73.us.us.us.i.i to i64, !dbg !6538
  %_26.i.us.us.us.i.i = load i32, ptr %_33.i.i, align 4, !dbg !6538, !alias.scope !6545, !noalias !6546, !noundef !12
  %_25.i.us.us.us.i.i = sub i32 %now.i.us.us.us.i.i, %_26.i.us.us.us.i.i, !dbg !6543
  %_24.i.us.us.us.i.i = and i32 %_25.i.us.us.us.i.i, %_58.i.i, !dbg !6538
  %_23.i77.us.us.us.i.i = zext i32 %_24.i.us.us.us.i.i to i64, !dbg !6538
  %_31.i78.us.us.us.i.i = icmp samesign ugt i64 %_64.1.i.i, %_15.i74.us.us.us.i.i, !dbg !6538
  switch i8 %_0.sroa.0.0.i545.i.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.us.i.i
    i8 1, label %bb14.i63.preheader.us.us.us.i.i
    i8 2, label %bb23.i.preheader.us.us.us.i.i
  ], !dbg !6547

bb27.i60.us.us.us.i.i:                            ; preds = %bb23.i.preheader.us.us.us.i.i
  %141 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_15.i74.us.us.us.i.i, !dbg !6548
  %_79.i.us.us.us.i.i = load float, ptr %141, align 4, !dbg !6548, !alias.scope !6534, !noalias !6549, !noundef !12
  %142 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_15.i74.us.us.us.i.i, !dbg !6550
  %_83.i.us.us.us.i.i = load float, ptr %142, align 4, !dbg !6550, !alias.scope !6536, !noalias !6551, !noundef !12
  %_87.i.us.us.us.i.i = icmp samesign ugt i64 %_66.1.i.i, %_23.i77.us.us.us.i.i, !dbg !6552
  br i1 %_87.i.us.us.us.i.i, label %bb31.i.us.us.us.i.i, label %panic32.i.i.i, !dbg !6552

bb31.i.us.us.us.i.i:                              ; preds = %bb27.i60.us.us.us.i.i
  %_89.i.us.us.us.i.i = icmp samesign ugt i64 %_64.1.i.i, %_23.i77.us.us.us.i.i, !dbg !6553
  br i1 %_89.i.us.us.us.i.i, label %bb33.i62.us.us.us.i.i, label %panic34.i.i.i, !dbg !6553

bb33.i62.us.us.us.i.i:                            ; preds = %bb31.i.us.us.us.i.i
  %143 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_23.i77.us.us.us.i.i, !dbg !6552
  %_86.i.us.us.us.i.i = load float, ptr %143, align 4, !dbg !6552, !alias.scope !6536, !noalias !6551, !noundef !12
  %144 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_23.i77.us.us.us.i.i, !dbg !6553
  %_88.i.us.us.us.i.i = load float, ptr %144, align 4, !dbg !6553, !alias.scope !6534, !noalias !6549, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i.i, !dbg !6554

bb17.i.us.us.us.i.i:                              ; preds = %bb14.i63.preheader.us.us.us.i.i
  %_59.i.us.us.us.i.i = icmp samesign ugt i64 %_66.1.i.i, %_23.i77.us.us.us.i.i, !dbg !6557
  br i1 %_59.i.us.us.us.i.i, label %bb19.i.us.us.us.i.i, label %panic17.i.i.i, !dbg !6557

bb19.i.us.us.us.i.i:                              ; preds = %bb17.i.us.us.us.i.i
  %145 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_15.i74.us.us.us.i.i, !dbg !6558
  %left_own16.i.us.us.us.i.i = load float, ptr %145, align 4, !dbg !6558, !alias.scope !6534, !noalias !6549, !noundef !12
  %146 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_23.i77.us.us.us.i.i, !dbg !6557
  %right_own18.i.us.us.us.i.i = load float, ptr %146, align 4, !dbg !6557, !alias.scope !6536, !noalias !6551, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i.i, !dbg !6554

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i.i: ; preds = %bb10.i80.us.us.us.i.i, %bb19.i.us.us.us.i.i, %bb33.i62.us.us.us.i.i
  %taps.i.sroa.41960.2.i.i = phi float [ %right_own.i.us.us.us.i.i, %bb10.i80.us.us.us.i.i ], [ %left_own16.i.us.us.us.i.i, %bb19.i.us.us.us.i.i ], [ %_88.i.us.us.us.i.i, %bb33.i62.us.us.us.i.i ], !dbg !6538
  %taps.i.sroa.28959.2.i.i = phi float [ %right_own.i.us.us.us.i.i, %bb10.i80.us.us.us.i.i ], [ %right_own18.i.us.us.us.i.i, %bb19.i.us.us.us.i.i ], [ %_86.i.us.us.us.i.i, %bb33.i62.us.us.us.i.i ], !dbg !6538
  %taps.i.sroa.15958.2.i.i = phi float [ %left_own.i.us.us.us.i.i, %bb10.i80.us.us.us.i.i ], [ %right_own18.i.us.us.us.i.i, %bb19.i.us.us.us.i.i ], [ %_83.i.us.us.us.i.i, %bb33.i62.us.us.us.i.i ], !dbg !6538
  %taps.i.sroa.0.2.i.i = phi float [ %left_own.i.us.us.us.i.i, %bb10.i80.us.us.us.i.i ], [ %left_own16.i.us.us.us.i.i, %bb19.i.us.us.us.i.i ], [ %_79.i.us.us.us.i.i, %bb33.i62.us.us.us.i.i ], !dbg !6538
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6559), !dbg !6562
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6563), !dbg !6562
  %_12.i28.i.us.us.us.i.i = load float, ptr %116, align 4, !dbg !6565, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i133.us.us.us.i.i = fcmp ule float %_12.i28.i.us.us.us.i.i, 0.000000e+00, !dbg !6569
  %_3.i93.us.us.us.i.i = fcmp une float %_12.i28.i.us.us.us.i.i, 1.000000e+00, !dbg !6571
  %_16.i31.i.us.us.us.i.i = load float, ptr %_10.i.i, align 4, !dbg !6573, !alias.scope !6567, !noalias !6568, !noundef !12
  %_17.i32.i.us.us.us.i.i = load float, ptr %117, align 4, !dbg !6574, !alias.scope !6567, !noalias !6568, !noundef !12
  %_0.i150.us.us.us.i.i = fadd float %_16.i31.i.us.us.us.i.i, %_17.i32.i.us.us.us.i.i, !dbg !6575
  %_20.i34.i548551.us.us.us.i.i = load float, ptr %118, align 4, !dbg !6577, !alias.scope !6567, !noalias !6568, !noundef !12
  %147 = select i1 %_3.i93.us.us.us.i.i, float %_0.i150.us.us.us.i.i, float %_20.i34.i548551.us.us.us.i.i, !dbg !6578
  %_0.i375.us.us.us.i.i = select i1 %_3.i133.us.us.us.i.i, float %_16.i31.i.us.us.us.i.i, float %147, !dbg !6580
  store float %_0.i375.us.us.us.i.i, ptr %_10.i.i, align 4, !dbg !6582, !alias.scope !6567, !noalias !6568
  %_0.i368.us.us.us.i.i = select i1 %_3.i93.us.us.us.i.i, float %_17.i32.i.us.us.us.i.i, float 0.000000e+00, !dbg !6583
  store float %_0.i368.us.us.us.i.i, ptr %117, align 4, !dbg !6585, !alias.scope !6567, !noalias !6568
  %_0.i191.us.us.us.i.i = fadd float %_12.i28.i.us.us.us.i.i, -1.000000e+00, !dbg !6586
  %_4.i361.v.us.us.us.i.i = select i1 %_3.i133.us.us.us.i.i, float %_12.i28.i.us.us.us.i.i, float %_0.i191.us.us.us.i.i, !dbg !6588
  store float %_4.i361.v.us.us.us.i.i, ptr %116, align 4, !dbg !6590, !alias.scope !6567, !noalias !6568
  %_12.i28.i.us.us.us.1.i.i = load float, ptr %119, align 4, !dbg !6565, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i133.us.us.us.1.i.i = fcmp ule float %_12.i28.i.us.us.us.1.i.i, 0.000000e+00, !dbg !6569
  %_3.i93.us.us.us.1.i.i = fcmp une float %_12.i28.i.us.us.us.1.i.i, 1.000000e+00, !dbg !6571
  %_16.i31.i.us.us.us.1.i.i = load float, ptr %iter1.sroa.0.0.ptr.i26.i.us.1.i.i, align 4, !dbg !6573, !alias.scope !6567, !noalias !6568, !noundef !12
  %_17.i32.i.us.us.us.1.i.i = load float, ptr %120, align 4, !dbg !6574, !alias.scope !6567, !noalias !6568, !noundef !12
  %_0.i150.us.us.us.1.i.i = fadd float %_16.i31.i.us.us.us.1.i.i, %_17.i32.i.us.us.us.1.i.i, !dbg !6575
  %_20.i34.i548551.us.us.us.1.i.i = load float, ptr %121, align 4, !dbg !6577, !alias.scope !6567, !noalias !6568, !noundef !12
  %148 = select i1 %_3.i93.us.us.us.1.i.i, float %_0.i150.us.us.us.1.i.i, float %_20.i34.i548551.us.us.us.1.i.i, !dbg !6578
  %_0.i375.us.us.us.1.i.i = select i1 %_3.i133.us.us.us.1.i.i, float %_16.i31.i.us.us.us.1.i.i, float %148, !dbg !6580
  store float %_0.i375.us.us.us.1.i.i, ptr %iter1.sroa.0.0.ptr.i26.i.us.1.i.i, align 4, !dbg !6582, !alias.scope !6567, !noalias !6568
  %_0.i368.us.us.us.1.i.i = select i1 %_3.i93.us.us.us.1.i.i, float %_17.i32.i.us.us.us.1.i.i, float 0.000000e+00, !dbg !6583
  store float %_0.i368.us.us.us.1.i.i, ptr %120, align 4, !dbg !6585, !alias.scope !6567, !noalias !6568
  %_0.i191.us.us.us.1.i.i = fadd float %_12.i28.i.us.us.us.1.i.i, -1.000000e+00, !dbg !6586
  %_4.i361.v.us.us.us.1.i.i = select i1 %_3.i133.us.us.us.1.i.i, float %_12.i28.i.us.us.us.1.i.i, float %_0.i191.us.us.us.1.i.i, !dbg !6588
  store float %_4.i361.v.us.us.us.1.i.i, ptr %119, align 4, !dbg !6590, !alias.scope !6567, !noalias !6568
  %_12.i28.i.us.us.us.2.i.i = load float, ptr %122, align 4, !dbg !6565, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i133.us.us.us.2.i.i = fcmp ule float %_12.i28.i.us.us.us.2.i.i, 0.000000e+00, !dbg !6569
  %_3.i93.us.us.us.2.i.i = fcmp une float %_12.i28.i.us.us.us.2.i.i, 1.000000e+00, !dbg !6571
  %_16.i31.i.us.us.us.2.i.i = load float, ptr %iter1.sroa.0.0.ptr.i26.i.us.2.i.i, align 4, !dbg !6573, !alias.scope !6567, !noalias !6568, !noundef !12
  %_17.i32.i.us.us.us.2.i.i = load float, ptr %123, align 4, !dbg !6574, !alias.scope !6567, !noalias !6568, !noundef !12
  %_0.i150.us.us.us.2.i.i = fadd float %_16.i31.i.us.us.us.2.i.i, %_17.i32.i.us.us.us.2.i.i, !dbg !6575
  %_20.i34.i548551.us.us.us.2.i.i = load float, ptr %124, align 4, !dbg !6577, !alias.scope !6567, !noalias !6568, !noundef !12
  %149 = select i1 %_3.i93.us.us.us.2.i.i, float %_0.i150.us.us.us.2.i.i, float %_20.i34.i548551.us.us.us.2.i.i, !dbg !6578
  %_0.i375.us.us.us.2.i.i = select i1 %_3.i133.us.us.us.2.i.i, float %_16.i31.i.us.us.us.2.i.i, float %149, !dbg !6580
  store float %_0.i375.us.us.us.2.i.i, ptr %iter1.sroa.0.0.ptr.i26.i.us.2.i.i, align 4, !dbg !6582, !alias.scope !6567, !noalias !6568
  %_0.i368.us.us.us.2.i.i = select i1 %_3.i93.us.us.us.2.i.i, float %_17.i32.i.us.us.us.2.i.i, float 0.000000e+00, !dbg !6583
  store float %_0.i368.us.us.us.2.i.i, ptr %123, align 4, !dbg !6585, !alias.scope !6567, !noalias !6568
  %_0.i191.us.us.us.2.i.i = fadd float %_12.i28.i.us.us.us.2.i.i, -1.000000e+00, !dbg !6586
  %_4.i361.v.us.us.us.2.i.i = select i1 %_3.i133.us.us.us.2.i.i, float %_12.i28.i.us.us.us.2.i.i, float %_0.i191.us.us.us.2.i.i, !dbg !6588
  store float %_4.i361.v.us.us.us.2.i.i, ptr %122, align 4, !dbg !6590, !alias.scope !6567, !noalias !6568
  %_12.i28.i.us.us.us.3.i.i = load float, ptr %125, align 4, !dbg !6565, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i133.us.us.us.3.i.i = fcmp ule float %_12.i28.i.us.us.us.3.i.i, 0.000000e+00, !dbg !6569
  %_3.i93.us.us.us.3.i.i = fcmp une float %_12.i28.i.us.us.us.3.i.i, 1.000000e+00, !dbg !6571
  %_16.i31.i.us.us.us.3.i.i = load float, ptr %iter1.sroa.0.0.ptr.i26.i.us.3.i.i, align 4, !dbg !6573, !alias.scope !6567, !noalias !6568, !noundef !12
  %_17.i32.i.us.us.us.3.i.i = load float, ptr %126, align 4, !dbg !6574, !alias.scope !6567, !noalias !6568, !noundef !12
  %_0.i150.us.us.us.3.i.i = fadd float %_16.i31.i.us.us.us.3.i.i, %_17.i32.i.us.us.us.3.i.i, !dbg !6575
  %_20.i34.i548551.us.us.us.3.i.i = load float, ptr %127, align 4, !dbg !6577, !alias.scope !6567, !noalias !6568, !noundef !12
  %150 = select i1 %_3.i93.us.us.us.3.i.i, float %_0.i150.us.us.us.3.i.i, float %_20.i34.i548551.us.us.us.3.i.i, !dbg !6578
  %_0.i375.us.us.us.3.i.i = select i1 %_3.i133.us.us.us.3.i.i, float %_16.i31.i.us.us.us.3.i.i, float %150, !dbg !6580
  store float %_0.i375.us.us.us.3.i.i, ptr %iter1.sroa.0.0.ptr.i26.i.us.3.i.i, align 4, !dbg !6582, !alias.scope !6567, !noalias !6568
  %_0.i368.us.us.us.3.i.i = select i1 %_3.i93.us.us.us.3.i.i, float %_17.i32.i.us.us.us.3.i.i, float 0.000000e+00, !dbg !6583
  store float %_0.i368.us.us.us.3.i.i, ptr %126, align 4, !dbg !6585, !alias.scope !6567, !noalias !6568
  %_0.i191.us.us.us.3.i.i = fadd float %_12.i28.i.us.us.us.3.i.i, -1.000000e+00, !dbg !6586
  %_4.i361.v.us.us.us.3.i.i = select i1 %_3.i133.us.us.us.3.i.i, float %_12.i28.i.us.us.us.3.i.i, float %_0.i191.us.us.us.3.i.i, !dbg !6588
  store float %_4.i361.v.us.us.us.3.i.i, ptr %125, align 4, !dbg !6590, !alias.scope !6567, !noalias !6568
  %151 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.2.i.i), !dbg !6591
  %152 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.15958.2.i.i), !dbg !6593
  %_37.i49.i.us.us.us.i.i = load float, ptr %101, align 4, !dbg !6595, !alias.scope !6596, !noalias !6597, !noundef !12
  %_3.i131.us.us.us.i.i = fcmp ule float %_37.i49.i.us.us.us.i.i, 0.000000e+00, !dbg !6598
  %_3.i.i483.us.us.us.i.i = fcmp ule float %151, %152, !dbg !6600
  %_6.i.i485.us.us.us.i.i = bitcast float %151 to i32, !dbg !6603
  %_8.i.i487.us.us.us.i.i = bitcast float %152 to i32, !dbg !6606
  %_4.i.i490.us.us.us.i.i = select i1 %_3.i.i483.us.us.us.i.i, i32 %_8.i.i487.us.us.us.i.i, i32 %_6.i.i485.us.us.us.i.i, !dbg !6608
  %_4.i354.us.us.us.i.i = select i1 %_3.i131.us.us.us.i.i, i32 %_6.i.i485.us.us.us.i.i, i32 %_4.i.i490.us.us.us.i.i, !dbg !6609
  %_41.i53.i.us.us.us.i.i = load float, ptr %102, align 4, !dbg !6611, !alias.scope !6596, !noalias !6597, !noundef !12
  %_3.i129.us.us.us.i.i = fcmp ule float %_41.i53.i.us.us.us.i.i, 0.000000e+00, !dbg !6612
  %_0.i175.us.us.us.i.i = fmul float %151, 5.000000e-01, !dbg !6614
  %_0.i174.us.us.us.i.i = fmul float %152, 5.000000e-01, !dbg !6616
  %_0.i149.us.us.us.i.i = fadd float %_0.i174.us.us.us.i.i, %_0.i175.us.us.us.i.i, !dbg !6618
  %_6.i342.us.us.us.i.i = bitcast float %_0.i149.us.us.us.i.i to i32, !dbg !6620
  %_4.i347.us.us.us.i.i = select i1 %_3.i129.us.us.us.i.i, i32 %_4.i354.us.us.us.i.i, i32 %_6.i342.us.us.us.i.i, !dbg !6623
  %_0.i348.us.us.us.i.i = bitcast i32 %_4.i347.us.us.us.i.i to float, !dbg !6624
  %_3.i.i475.us.us.us.i.i = fcmp ule float %_0.i348.us.us.us.i.i, 0x3E45798EE0000000, !dbg !6626
  %_4.i.i481.us.us.us.i.i = select i1 %_3.i.i475.us.us.us.i.i, i32 841731191, i32 %_4.i347.us.us.us.i.i, !dbg !6629
  %_0.i.i482.us.us.us.i.i = bitcast i32 %_4.i.i481.us.us.us.i.i to float, !dbg !6631
  %_3.i.i.us.us.us.i.i = fcmp ule float %_0.i.i482.us.us.us.i.i, 0x3810000000000000, !dbg !6633
  %_4.i.i.us.us.us.i.i = select i1 %_3.i.i.us.us.us.i.i, i32 8388608, i32 %_4.i.i481.us.us.us.i.i, !dbg !6638
  %_5.i208.us.us.us.i.i = and i32 %_4.i.i.us.us.us.i.i, 8388607, !dbg !6640
  %_4.i209.us.us.us.i.i = or disjoint i32 %_5.i208.us.us.us.i.i, 1065353216, !dbg !6640
  %significand.i.us.us.us.i.i = bitcast i32 %_4.i209.us.us.us.i.i to float, !dbg !6642
  %_0.i176.us.us.us.i.i = fadd float %significand.i.us.us.us.i.i, -1.000000e+00, !dbg !6644
  %_0.i156.us.us.us.i.i = fmul float %_0.i176.us.us.us.i.i, 0x3F9B17A960000000, !dbg !6646
  %153 = fsub float 0x3FBF9A8440000000, %_0.i156.us.us.us.i.i, !dbg !6648
  %_0.i156.us.us.us.1.i.i = fmul float %_0.i176.us.us.us.i.i, %153, !dbg !6646
  %_0.i140.us.us.us.1.i.i = fadd float %_0.i156.us.us.us.1.i.i, 0xBFD1E3F400000000, !dbg !6648
  %_0.i156.us.us.us.2.i.i = fmul float %_0.i176.us.us.us.i.i, %_0.i140.us.us.us.1.i.i, !dbg !6646
  %_0.i140.us.us.us.2.i.i = fadd float %_0.i156.us.us.us.2.i.i, 0x3FDD544F20000000, !dbg !6648
  %_0.i156.us.us.us.3.i.i = fmul float %_0.i176.us.us.us.i.i, %_0.i140.us.us.us.2.i.i, !dbg !6646
  %_0.i140.us.us.us.3.i.i = fadd float %_0.i156.us.us.us.3.i.i, 0xBFE6FC2A60000000, !dbg !6648
  %_0.i156.us.us.us.4.i.i = fmul float %_0.i176.us.us.us.i.i, %_0.i140.us.us.us.3.i.i, !dbg !6646
  %_0.i140.us.us.us.4.i.i = fadd float %_0.i156.us.us.us.4.i.i, 0x3FF714B2A0000000, !dbg !6648
  %_9.i.us.us.us.i.i = lshr i32 %_4.i.i.us.us.us.i.i, 23, !dbg !6650
  %_8.i210.us.us.us.i.i = or disjoint i32 %_9.i.us.us.us.i.i, 1258291200, !dbg !6650
  %_7.i.us.us.us.i.i = bitcast i32 %_8.i210.us.us.us.i.i to float, !dbg !6651
  %exponent.i.us.us.us.i.i = fadd float %_7.i.us.us.us.i.i, 0xC160000FE0000000, !dbg !6653
  %_0.i155.us.us.us.i.i = fmul float %_0.i176.us.us.us.i.i, %_0.i140.us.us.us.4.i.i, !dbg !6654
  %_0.i139.us.us.us.i.i = fadd float %exponent.i.us.us.us.i.i, %_0.i155.us.us.us.i.i, !dbg !6656
  %_0.i173.us.us.us.i.i = fmul float %_0.i139.us.us.us.i.i, 0x4018151820000000, !dbg !6658
  %_3.i.i532.us.us.us.inv.i.i = fcmp olt float %_0.i173.us.us.us.i.i, 2.400000e+01, !dbg !6660
  %_0.i.i539.us.us.us.i.i = select i1 %_3.i.i532.us.us.us.inv.i.i, float %_0.i173.us.us.us.i.i, float 2.400000e+01, !dbg !6660
  %_3.i.i467.us.us.us.inv.i.i = fcmp ogt float %_0.i.i539.us.us.us.i.i, -1.600000e+02, !dbg !6663
  %_0.i.i474.us.us.us.i.i = select i1 %_3.i.i467.us.us.us.inv.i.i, float %_0.i.i539.us.us.us.i.i, float -1.600000e+02, !dbg !6663
  %_55.i70.i.us.us.us.i.i = load float, ptr %100, align 4, !dbg !6666, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i127.us.us.us.i.i = fcmp ule float %_55.i70.i.us.us.us.i.i, 0.000000e+00, !dbg !6667
  %_3.i101.us.us.us.i.i = fcmp oge float %_0.i.i474.us.us.us.i.i, %_0.i375.us.us.us.i.i, !dbg !6669
  %_0.i190.us.us.us.i.i = fsub float %_0.i375.us.us.us.i.i, %_0.i375.us.us.us.3.i.i, !dbg !6671
  %_3.i99.us.us.us.i.i = fcmp oge float %_0.i.i474.us.us.us.i.i, %_0.i190.us.us.us.i.i, !dbg !6673
  %..i100.us.us.us.i.i = sext i1 %_3.i99.us.us.us.i.i to i32, !dbg !6675
  %_0.i395.us.us.us.i.i = sext i1 %_3.i101.us.us.us.i.i to i32, !dbg !6677
  %_0.i388.us.us.us.i.i = select i1 %_3.i127.us.us.us.i.i, i32 %_0.i395.us.us.us.i.i, i32 %..i100.us.us.us.i.i, !dbg !6677
  %_0.i399.us.us.us.i.i = xor i32 %..i100.us.us.us.i.i, -1, !dbg !6679
  %_67.i80.i.us.us.us.i.i = load float, ptr %103, align 4, !dbg !6681, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i125.us.us.us.i.i = fcmp ogt float %_67.i80.i.us.us.us.i.i, 0.000000e+00, !dbg !6682
  %_0.i394.us.us.us.i.i = select i1 %_3.i125.us.us.us.i.i, i32 %_0.i399.us.us.us.i.i, i32 0, !dbg !6684
  %_0.i393.us.us.us.i.i = select i1 %_3.i127.us.us.us.i.i, i32 0, i32 %_0.i394.us.us.us.i.i, !dbg !6686
  %_0.i387.us.us.us.i.i = or i32 %_0.i393.us.us.us.i.i, %_0.i388.us.us.us.i.i, !dbg !6688
  %_5.i337.us.us.us.i.i = and i32 %_0.i387.us.us.us.i.i, 1065353216, !dbg !6690
  %_0.i341.us.us.us.i.i = bitcast i32 %_5.i337.us.us.us.i.i to float, !dbg !6692
  %_71.i86.i560561.us.us.us.i.i = load float, ptr %104, align 4, !dbg !6694, !alias.scope !6596, !noalias !6597, !noundef !12
  %_0.i189.us.us.us.i.i = fadd float %_67.i80.i.us.us.us.i.i, -1.000000e+00, !dbg !6695
  %154 = trunc nsw i32 %_0.i393.us.us.us.i.i to i1, !dbg !6697
  %_4.i335.v.us.us.us.i.i = select i1 %154, float %_0.i189.us.us.us.i.i, float %_67.i80.i.us.us.us.i.i, !dbg !6697
  %155 = trunc nsw i32 %_0.i388.us.us.us.i.i to i1, !dbg !6699
  %_0.i329.us.us.us.i.i = select i1 %155, float %_71.i86.i560561.us.us.us.i.i, float %_4.i335.v.us.us.us.i.i, !dbg !6699
  store float %_0.i329.us.us.us.i.i, ptr %103, align 4, !dbg !6701, !alias.scope !6567, !noalias !6568
  store i32 %_5.i337.us.us.us.i.i, ptr %100, align 4, !dbg !6702, !alias.scope !6567, !noalias !6568
  %_0.i188.us.us.us.i.i = fadd float %_0.i375.us.us.us.1.i.i, -1.000000e+00, !dbg !6703
  %_0.i187.us.us.us.i.i = fsub float %_0.i.i474.us.us.us.i.i, %_0.i375.us.us.us.i.i, !dbg !6705
  %_0.i172.us.us.us.i.i = fmul float %_0.i188.us.us.us.i.i, %_0.i187.us.us.us.i.i, !dbg !6707
  %156 = fneg float %_0.i375.us.us.us.2.i.i, !dbg !6709
  %_3.i.i458.inv.us.us.us.i.i = fcmp ogt float %_0.i172.us.us.us.i.i, %156, !dbg !6711
  %_4.i.i465.v.us.us.us.i.i = select i1 %_3.i.i458.inv.us.us.us.i.i, float %_0.i172.us.us.us.i.i, float %156, !dbg !6711
  %_3.i.i524.us.us.us.i.i = fcmp olt float %_4.i.i465.v.us.us.us.i.i, 0.000000e+00, !dbg !6714
  %157 = fcmp ule float %_0.i341.us.us.us.i.i, 0.000000e+00, !dbg !6717
  %158 = select i1 %157, i1 %_3.i.i524.us.us.us.i.i, i1 false, !dbg !6719
  %_0.i322.us.us.us.i.i = select i1 %158, float %_4.i.i465.v.us.us.us.i.i, float 0.000000e+00, !dbg !6719
  %_86.i99.i.us.us.us.i.i = load float, ptr %105, align 4, !dbg !6720, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i121.us.us.us.i.i = fcmp ule float %_0.i322.us.us.us.i.i, %_86.i99.i.us.us.us.i.i, !dbg !6721
  %_87.i101.i563.us.us.us.i.i = load i32, ptr %95, align 4, !dbg !6723, !alias.scope !6596, !noalias !6597, !noundef !12
  %_88.i102.i564.us.us.us.i.i = load i32, ptr %106, align 4, !dbg !6724, !alias.scope !6596, !noalias !6597, !noundef !12
  %_4.i315.us.us.us.i.i = select i1 %_3.i121.us.us.us.i.i, i32 %_88.i102.i564.us.us.us.i.i, i32 %_87.i101.i563.us.us.us.i.i, !dbg !6725
  %_0.i316.us.us.us.i.i = bitcast i32 %_4.i315.us.us.us.i.i to float, !dbg !6727
  %_0.i186.us.us.us.i.i = fsub float %_0.i322.us.us.us.i.i, %_86.i99.i.us.us.us.i.i, !dbg !6729
  %_4.i153.us.us.us.i.i = fmul float %_0.i186.us.us.us.i.i, %_0.i316.us.us.us.i.i, !dbg !6731
  %_0.i154.us.us.us.i.i = fadd float %_86.i99.i.us.us.us.i.i, %_4.i153.us.us.us.i.i, !dbg !6731
  %159 = tail call noundef float @llvm.fabs.f32(float %_0.i154.us.us.us.i.i), !dbg !6733
  %160 = fcmp uge float %159, 0x3BC79CA100000000, !dbg !6736
  %_0.i224.us.us.us.i.i = select i1 %160, float %_0.i154.us.us.us.i.i, float 0.000000e+00, !dbg !6738
  store float %_0.i224.us.us.us.i.i, ptr %105, align 4, !dbg !6739, !alias.scope !6567, !noalias !6568
  %_0.i171.us.us.us.i.i = fmul float %_0.i224.us.us.us.i.i, 0x3FC542A5A0000000, !dbg !6740
  %_3.i.i409.us.us.us.inv.i.i = fcmp ogt float %_0.i171.us.us.us.i.i, -1.260000e+02, !dbg !6743
  %_0.i.i416.us.us.us.i.i = select i1 %_3.i.i409.us.us.us.inv.i.i, float %_0.i171.us.us.us.i.i, float -1.260000e+02, !dbg !6743
  %_3.i.i492.us.us.us.inv.i.i = fcmp olt float %_0.i.i416.us.us.us.i.i, 1.270000e+02, !dbg !6747
  %_0.i.i499.us.us.us.i.i = select i1 %_3.i.i492.us.us.us.inv.i.i, float %_0.i.i416.us.us.us.i.i, float 1.270000e+02, !dbg !6747
  %161 = tail call noundef float @llvm.floor.f32(float %_0.i.i499.us.us.us.i.i), !dbg !6750
  %_0.i178.us.us.us.i.i = fsub float %_0.i.i499.us.us.us.i.i, %161, !dbg !6754
  %_98.i115.i.us.us.us.i.i = load float, ptr %107, align 4, !dbg !6756, !alias.scope !6596, !noalias !6597, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6757), !dbg !6760
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6762), !dbg !6760
  %_12.i.i.us.us.us.i.i = load float, ptr %128, align 4, !dbg !6764, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i117.us.us.us.i.i = fcmp ule float %_12.i.i.us.us.us.i.i, 0.000000e+00, !dbg !6768
  %_3.i89.us.us.us.i.i = fcmp une float %_12.i.i.us.us.us.i.i, 1.000000e+00, !dbg !6770
  %_16.i.i.us.us.us.i.i = load float, ptr %data.i.i.i.i, align 4, !dbg !6772, !alias.scope !6766, !noalias !6767, !noundef !12
  %_17.i.i.us.us.us.i.i = load float, ptr %129, align 4, !dbg !6773, !alias.scope !6766, !noalias !6767, !noundef !12
  %_0.i148.us.us.us.i.i = fadd float %_16.i.i.us.us.us.i.i, %_17.i.i.us.us.us.i.i, !dbg !6774
  %_20.i.i570573.us.us.us.i.i = load float, ptr %130, align 4, !dbg !6776, !alias.scope !6766, !noalias !6767, !noundef !12
  %162 = select i1 %_3.i89.us.us.us.i.i, float %_0.i148.us.us.us.i.i, float %_20.i.i570573.us.us.us.i.i, !dbg !6777
  %_0.i295.us.us.us.i.i = select i1 %_3.i117.us.us.us.i.i, float %_16.i.i.us.us.us.i.i, float %162, !dbg !6779
  store float %_0.i295.us.us.us.i.i, ptr %data.i.i.i.i, align 4, !dbg !6781, !alias.scope !6766, !noalias !6767
  %_0.i288.us.us.us.i.i = select i1 %_3.i89.us.us.us.i.i, float %_17.i.i.us.us.us.i.i, float 0.000000e+00, !dbg !6782
  store float %_0.i288.us.us.us.i.i, ptr %129, align 4, !dbg !6784, !alias.scope !6766, !noalias !6767
  %_0.i185.us.us.us.i.i = fadd float %_12.i.i.us.us.us.i.i, -1.000000e+00, !dbg !6785
  %_4.i281.v.us.us.us.i.i = select i1 %_3.i117.us.us.us.i.i, float %_12.i.i.us.us.us.i.i, float %_0.i185.us.us.us.i.i, !dbg !6787
  store float %_4.i281.v.us.us.us.i.i, ptr %128, align 4, !dbg !6789, !alias.scope !6766, !noalias !6767
  %_12.i.i.us.us.us.1.i.i = load float, ptr %131, align 4, !dbg !6764, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i117.us.us.us.1.i.i = fcmp ule float %_12.i.i.us.us.us.1.i.i, 0.000000e+00, !dbg !6768
  %_3.i89.us.us.us.1.i.i = fcmp une float %_12.i.i.us.us.us.1.i.i, 1.000000e+00, !dbg !6770
  %_16.i.i.us.us.us.1.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.1.i.i, align 4, !dbg !6772, !alias.scope !6766, !noalias !6767, !noundef !12
  %_17.i.i.us.us.us.1.i.i = load float, ptr %132, align 4, !dbg !6773, !alias.scope !6766, !noalias !6767, !noundef !12
  %_0.i148.us.us.us.1.i.i = fadd float %_16.i.i.us.us.us.1.i.i, %_17.i.i.us.us.us.1.i.i, !dbg !6774
  %_20.i.i570573.us.us.us.1.i.i = load float, ptr %133, align 4, !dbg !6776, !alias.scope !6766, !noalias !6767, !noundef !12
  %163 = select i1 %_3.i89.us.us.us.1.i.i, float %_0.i148.us.us.us.1.i.i, float %_20.i.i570573.us.us.us.1.i.i, !dbg !6777
  %_0.i295.us.us.us.1.i.i = select i1 %_3.i117.us.us.us.1.i.i, float %_16.i.i.us.us.us.1.i.i, float %163, !dbg !6779
  store float %_0.i295.us.us.us.1.i.i, ptr %iter1.sroa.0.0.ptr.i.i.us.1.i.i, align 4, !dbg !6781, !alias.scope !6766, !noalias !6767
  %_0.i288.us.us.us.1.i.i = select i1 %_3.i89.us.us.us.1.i.i, float %_17.i.i.us.us.us.1.i.i, float 0.000000e+00, !dbg !6782
  store float %_0.i288.us.us.us.1.i.i, ptr %132, align 4, !dbg !6784, !alias.scope !6766, !noalias !6767
  %_0.i185.us.us.us.1.i.i = fadd float %_12.i.i.us.us.us.1.i.i, -1.000000e+00, !dbg !6785
  %_4.i281.v.us.us.us.1.i.i = select i1 %_3.i117.us.us.us.1.i.i, float %_12.i.i.us.us.us.1.i.i, float %_0.i185.us.us.us.1.i.i, !dbg !6787
  store float %_4.i281.v.us.us.us.1.i.i, ptr %131, align 4, !dbg !6789, !alias.scope !6766, !noalias !6767
  %_12.i.i.us.us.us.2.i.i = load float, ptr %134, align 4, !dbg !6764, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i117.us.us.us.2.i.i = fcmp ule float %_12.i.i.us.us.us.2.i.i, 0.000000e+00, !dbg !6768
  %_3.i89.us.us.us.2.i.i = fcmp une float %_12.i.i.us.us.us.2.i.i, 1.000000e+00, !dbg !6770
  %_16.i.i.us.us.us.2.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.2.i.i, align 4, !dbg !6772, !alias.scope !6766, !noalias !6767, !noundef !12
  %_17.i.i.us.us.us.2.i.i = load float, ptr %135, align 4, !dbg !6773, !alias.scope !6766, !noalias !6767, !noundef !12
  %_0.i148.us.us.us.2.i.i = fadd float %_16.i.i.us.us.us.2.i.i, %_17.i.i.us.us.us.2.i.i, !dbg !6774
  %_20.i.i570573.us.us.us.2.i.i = load float, ptr %136, align 4, !dbg !6776, !alias.scope !6766, !noalias !6767, !noundef !12
  %164 = select i1 %_3.i89.us.us.us.2.i.i, float %_0.i148.us.us.us.2.i.i, float %_20.i.i570573.us.us.us.2.i.i, !dbg !6777
  %_0.i295.us.us.us.2.i.i = select i1 %_3.i117.us.us.us.2.i.i, float %_16.i.i.us.us.us.2.i.i, float %164, !dbg !6779
  store float %_0.i295.us.us.us.2.i.i, ptr %iter1.sroa.0.0.ptr.i.i.us.2.i.i, align 4, !dbg !6781, !alias.scope !6766, !noalias !6767
  %_0.i288.us.us.us.2.i.i = select i1 %_3.i89.us.us.us.2.i.i, float %_17.i.i.us.us.us.2.i.i, float 0.000000e+00, !dbg !6782
  store float %_0.i288.us.us.us.2.i.i, ptr %135, align 4, !dbg !6784, !alias.scope !6766, !noalias !6767
  %_0.i185.us.us.us.2.i.i = fadd float %_12.i.i.us.us.us.2.i.i, -1.000000e+00, !dbg !6785
  %_4.i281.v.us.us.us.2.i.i = select i1 %_3.i117.us.us.us.2.i.i, float %_12.i.i.us.us.us.2.i.i, float %_0.i185.us.us.us.2.i.i, !dbg !6787
  store float %_4.i281.v.us.us.us.2.i.i, ptr %134, align 4, !dbg !6789, !alias.scope !6766, !noalias !6767
  %_12.i.i.us.us.us.3.i.i = load float, ptr %137, align 4, !dbg !6764, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i117.us.us.us.3.i.i = fcmp ule float %_12.i.i.us.us.us.3.i.i, 0.000000e+00, !dbg !6768
  %_3.i89.us.us.us.3.i.i = fcmp une float %_12.i.i.us.us.us.3.i.i, 1.000000e+00, !dbg !6770
  %_16.i.i.us.us.us.3.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.3.i.i, align 4, !dbg !6772, !alias.scope !6766, !noalias !6767, !noundef !12
  %_17.i.i.us.us.us.3.i.i = load float, ptr %138, align 4, !dbg !6773, !alias.scope !6766, !noalias !6767, !noundef !12
  %_0.i148.us.us.us.3.i.i = fadd float %_16.i.i.us.us.us.3.i.i, %_17.i.i.us.us.us.3.i.i, !dbg !6774
  %_20.i.i570573.us.us.us.3.i.i = load float, ptr %139, align 4, !dbg !6776, !alias.scope !6766, !noalias !6767, !noundef !12
  %165 = select i1 %_3.i89.us.us.us.3.i.i, float %_0.i148.us.us.us.3.i.i, float %_20.i.i570573.us.us.us.3.i.i, !dbg !6777
  %_0.i295.us.us.us.3.i.i = select i1 %_3.i117.us.us.us.3.i.i, float %_16.i.i.us.us.us.3.i.i, float %165, !dbg !6779
  store float %_0.i295.us.us.us.3.i.i, ptr %iter1.sroa.0.0.ptr.i.i.us.3.i.i, align 4, !dbg !6781, !alias.scope !6766, !noalias !6767
  %_0.i288.us.us.us.3.i.i = select i1 %_3.i89.us.us.us.3.i.i, float %_17.i.i.us.us.us.3.i.i, float 0.000000e+00, !dbg !6782
  store float %_0.i288.us.us.us.3.i.i, ptr %138, align 4, !dbg !6784, !alias.scope !6766, !noalias !6767
  %_0.i185.us.us.us.3.i.i = fadd float %_12.i.i.us.us.us.3.i.i, -1.000000e+00, !dbg !6785
  %_4.i281.v.us.us.us.3.i.i = select i1 %_3.i117.us.us.us.3.i.i, float %_12.i.i.us.us.us.3.i.i, float %_0.i185.us.us.us.3.i.i, !dbg !6787
  store float %_4.i281.v.us.us.us.3.i.i, ptr %137, align 4, !dbg !6789, !alias.scope !6766, !noalias !6767
  %166 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.28959.2.i.i), !dbg !6790
  %167 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.41960.2.i.i), !dbg !6792
  %_37.i.i.us.us.us.i.i = load float, ptr %109, align 4, !dbg !6794, !alias.scope !6795, !noalias !6796, !noundef !12
  %_3.i115.us.us.us.i.i = fcmp ule float %_37.i.i.us.us.us.i.i, 0.000000e+00, !dbg !6797
  %_3.i.i449.us.us.us.i.i = fcmp ule float %166, %167, !dbg !6799
  %_6.i.i451.us.us.us.i.i = bitcast float %166 to i32, !dbg !6802
  %_8.i.i453.us.us.us.i.i = bitcast float %167 to i32, !dbg !6805
  %_4.i.i456.us.us.us.i.i = select i1 %_3.i.i449.us.us.us.i.i, i32 %_8.i.i453.us.us.us.i.i, i32 %_6.i.i451.us.us.us.i.i, !dbg !6807
  %_4.i274.us.us.us.i.i = select i1 %_3.i115.us.us.us.i.i, i32 %_6.i.i451.us.us.us.i.i, i32 %_4.i.i456.us.us.us.i.i, !dbg !6808
  %_41.i.i.us.us.us.i.i = load float, ptr %110, align 4, !dbg !6810, !alias.scope !6795, !noalias !6796, !noundef !12
  %_3.i113.us.us.us.i.i = fcmp ule float %_41.i.i.us.us.us.i.i, 0.000000e+00, !dbg !6811
  %_0.i169.us.us.us.i.i = fmul float %166, 5.000000e-01, !dbg !6813
  %_0.i168.us.us.us.i.i = fmul float %167, 5.000000e-01, !dbg !6815
  %_0.i147.us.us.us.i.i = fadd float %_0.i168.us.us.us.i.i, %_0.i169.us.us.us.i.i, !dbg !6817
  %_6.i262.us.us.us.i.i = bitcast float %_0.i147.us.us.us.i.i to i32, !dbg !6819
  %_4.i267.us.us.us.i.i = select i1 %_3.i113.us.us.us.i.i, i32 %_4.i274.us.us.us.i.i, i32 %_6.i262.us.us.us.i.i, !dbg !6822
  %_0.i268.us.us.us.i.i = bitcast i32 %_4.i267.us.us.us.i.i to float, !dbg !6823
  %_3.i.i441.us.us.us.i.i = fcmp ule float %_0.i268.us.us.us.i.i, 0x3E45798EE0000000, !dbg !6825
  %_4.i.i447.us.us.us.i.i = select i1 %_3.i.i441.us.us.us.i.i, i32 841731191, i32 %_4.i267.us.us.us.i.i, !dbg !6828
  %_0.i.i448.us.us.us.i.i = bitcast i32 %_4.i.i447.us.us.us.i.i to float, !dbg !6830
  %_3.i.i401.us.us.us.i.i = fcmp ule float %_0.i.i448.us.us.us.i.i, 0x3810000000000000, !dbg !6832
  %_4.i.i407.us.us.us.i.i = select i1 %_3.i.i401.us.us.us.i.i, i32 8388608, i32 %_4.i.i447.us.us.us.i.i, !dbg !6837
  %_5.i212.us.us.us.i.i = and i32 %_4.i.i407.us.us.us.i.i, 8388607, !dbg !6839
  %_4.i213.us.us.us.i.i = or disjoint i32 %_5.i212.us.us.us.i.i, 1065353216, !dbg !6839
  %significand.i214.us.us.us.i.i = bitcast i32 %_4.i213.us.us.us.i.i to float, !dbg !6841
  %_0.i177.us.us.us.i.i = fadd float %significand.i214.us.us.us.i.i, -1.000000e+00, !dbg !6843
  %_0.i158.us.us.us.i.i = fmul float %_0.i177.us.us.us.i.i, 0x3F9B17A960000000, !dbg !6845
  %168 = fsub float 0x3FBF9A8440000000, %_0.i158.us.us.us.i.i, !dbg !6847
  %_0.i158.us.us.us.1.i.i = fmul float %_0.i177.us.us.us.i.i, %168, !dbg !6845
  %_0.i142.us.us.us.1.i.i = fadd float %_0.i158.us.us.us.1.i.i, 0xBFD1E3F400000000, !dbg !6847
  %_0.i158.us.us.us.2.i.i = fmul float %_0.i177.us.us.us.i.i, %_0.i142.us.us.us.1.i.i, !dbg !6845
  %_0.i142.us.us.us.2.i.i = fadd float %_0.i158.us.us.us.2.i.i, 0x3FDD544F20000000, !dbg !6847
  %_0.i158.us.us.us.3.i.i = fmul float %_0.i177.us.us.us.i.i, %_0.i142.us.us.us.2.i.i, !dbg !6845
  %_0.i142.us.us.us.3.i.i = fadd float %_0.i158.us.us.us.3.i.i, 0xBFE6FC2A60000000, !dbg !6847
  %_0.i158.us.us.us.4.i.i = fmul float %_0.i177.us.us.us.i.i, %_0.i142.us.us.us.3.i.i, !dbg !6845
  %_0.i142.us.us.us.4.i.i = fadd float %_0.i158.us.us.us.4.i.i, 0x3FF714B2A0000000, !dbg !6847
  %_9.i215.us.us.us.i.i = lshr i32 %_4.i.i407.us.us.us.i.i, 23, !dbg !6849
  %_8.i216.us.us.us.i.i = or disjoint i32 %_9.i215.us.us.us.i.i, 1258291200, !dbg !6849
  %_7.i217.us.us.us.i.i = bitcast i32 %_8.i216.us.us.us.i.i to float, !dbg !6850
  %exponent.i218.us.us.us.i.i = fadd float %_7.i217.us.us.us.i.i, 0xC160000FE0000000, !dbg !6852
  %_0.i157.us.us.us.i.i = fmul float %_0.i177.us.us.us.i.i, %_0.i142.us.us.us.4.i.i, !dbg !6853
  %_0.i141.us.us.us.i.i = fadd float %exponent.i218.us.us.us.i.i, %_0.i157.us.us.us.i.i, !dbg !6855
  %_0.i167.us.us.us.i.i = fmul float %_0.i141.us.us.us.i.i, 0x4018151820000000, !dbg !6857
  %_3.i.i516.us.us.us.inv.i.i = fcmp olt float %_0.i167.us.us.us.i.i, 2.400000e+01, !dbg !6859
  %_0.i.i523.us.us.us.i.i = select i1 %_3.i.i516.us.us.us.inv.i.i, float %_0.i167.us.us.us.i.i, float 2.400000e+01, !dbg !6859
  %_3.i.i433.us.us.us.inv.i.i = fcmp ogt float %_0.i.i523.us.us.us.i.i, -1.600000e+02, !dbg !6862
  %_0.i.i440.us.us.us.i.i = select i1 %_3.i.i433.us.us.us.inv.i.i, float %_0.i.i523.us.us.us.i.i, float -1.600000e+02, !dbg !6862
  %_55.i.i.us.us.us.i.i = load float, ptr %108, align 4, !dbg !6865, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i111.us.us.us.i.i = fcmp ule float %_55.i.i.us.us.us.i.i, 0.000000e+00, !dbg !6866
  %_3.i97.us.us.us.i.i = fcmp oge float %_0.i.i440.us.us.us.i.i, %_0.i295.us.us.us.i.i, !dbg !6868
  %_0.i184.us.us.us.i.i = fsub float %_0.i295.us.us.us.i.i, %_0.i295.us.us.us.3.i.i, !dbg !6870
  %_3.i95.us.us.us.i.i = fcmp oge float %_0.i.i440.us.us.us.i.i, %_0.i184.us.us.us.i.i, !dbg !6872
  %..i96.us.us.us.i.i = sext i1 %_3.i95.us.us.us.i.i to i32, !dbg !6874
  %_0.i391.us.us.us.i.i = sext i1 %_3.i97.us.us.us.i.i to i32, !dbg !6876
  %_0.i385.us.us.us.i.i = select i1 %_3.i111.us.us.us.i.i, i32 %_0.i391.us.us.us.i.i, i32 %..i96.us.us.us.i.i, !dbg !6876
  %_0.i397.us.us.us.i.i = xor i32 %..i96.us.us.us.i.i, -1, !dbg !6878
  %_67.i.i.us.us.us.i.i = load float, ptr %111, align 4, !dbg !6880, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i109.us.us.us.i.i = fcmp ogt float %_67.i.i.us.us.us.i.i, 0.000000e+00, !dbg !6881
  %_0.i390.us.us.us.i.i = select i1 %_3.i109.us.us.us.i.i, i32 %_0.i397.us.us.us.i.i, i32 0, !dbg !6883
  %_0.i389.us.us.us.i.i = select i1 %_3.i111.us.us.us.i.i, i32 0, i32 %_0.i390.us.us.us.i.i, !dbg !6885
  %_0.i384.us.us.us.i.i = or i32 %_0.i389.us.us.us.i.i, %_0.i385.us.us.us.i.i, !dbg !6887
  %_5.i257.us.us.us.i.i = and i32 %_0.i384.us.us.us.i.i, 1065353216, !dbg !6889
  %_0.i261.us.us.us.i.i = bitcast i32 %_5.i257.us.us.us.i.i to float, !dbg !6891
  %_71.i.i582583.us.us.us.i.i = load float, ptr %112, align 4, !dbg !6893, !alias.scope !6795, !noalias !6796, !noundef !12
  %_0.i183.us.us.us.i.i = fadd float %_67.i.i.us.us.us.i.i, -1.000000e+00, !dbg !6894
  %169 = trunc nsw i32 %_0.i389.us.us.us.i.i to i1, !dbg !6896
  %_4.i255.v.us.us.us.i.i = select i1 %169, float %_0.i183.us.us.us.i.i, float %_67.i.i.us.us.us.i.i, !dbg !6896
  %170 = trunc nsw i32 %_0.i385.us.us.us.i.i to i1, !dbg !6898
  %_0.i249.us.us.us.i.i = select i1 %170, float %_71.i.i582583.us.us.us.i.i, float %_4.i255.v.us.us.us.i.i, !dbg !6898
  store float %_0.i249.us.us.us.i.i, ptr %111, align 4, !dbg !6900, !alias.scope !6766, !noalias !6767
  store i32 %_5.i257.us.us.us.i.i, ptr %108, align 4, !dbg !6901, !alias.scope !6766, !noalias !6767
  %_0.i182.us.us.us.i.i = fadd float %_0.i295.us.us.us.1.i.i, -1.000000e+00, !dbg !6902
  %_0.i181.us.us.us.i.i = fsub float %_0.i.i440.us.us.us.i.i, %_0.i295.us.us.us.i.i, !dbg !6904
  %_0.i166.us.us.us.i.i = fmul float %_0.i182.us.us.us.i.i, %_0.i181.us.us.us.i.i, !dbg !6906
  %171 = fneg float %_0.i295.us.us.us.2.i.i, !dbg !6908
  %_3.i.i425.inv.us.us.us.i.i = fcmp ogt float %_0.i166.us.us.us.i.i, %171, !dbg !6910
  %_4.i.i431.v.us.us.us.i.i = select i1 %_3.i.i425.inv.us.us.us.i.i, float %_0.i166.us.us.us.i.i, float %171, !dbg !6910
  %_3.i.i508.us.us.us.i.i = fcmp olt float %_4.i.i431.v.us.us.us.i.i, 0.000000e+00, !dbg !6913
  %172 = fcmp ule float %_0.i261.us.us.us.i.i, 0.000000e+00, !dbg !6916
  %173 = select i1 %172, i1 %_3.i.i508.us.us.us.i.i, i1 false, !dbg !6918
  %_0.i242.us.us.us.i.i = select i1 %173, float %_4.i.i431.v.us.us.us.i.i, float 0.000000e+00, !dbg !6918
  %_86.i.i.us.us.us.i.i = load float, ptr %113, align 4, !dbg !6919, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i105.us.us.us.i.i = fcmp ule float %_0.i242.us.us.us.i.i, %_86.i.i.us.us.us.i.i, !dbg !6920
  %_87.i.i585.us.us.us.i.i = load i32, ptr %_38.i.i, align 4, !dbg !6922, !alias.scope !6795, !noalias !6796, !noundef !12
  %_88.i.i586.us.us.us.i.i = load i32, ptr %114, align 4, !dbg !6923, !alias.scope !6795, !noalias !6796, !noundef !12
  %_4.i235.us.us.us.i.i = select i1 %_3.i105.us.us.us.i.i, i32 %_88.i.i586.us.us.us.i.i, i32 %_87.i.i585.us.us.us.i.i, !dbg !6924
  %_0.i236.us.us.us.i.i = bitcast i32 %_4.i235.us.us.us.i.i to float, !dbg !6926
  %_0.i180.us.us.us.i.i = fsub float %_0.i242.us.us.us.i.i, %_86.i.i.us.us.us.i.i, !dbg !6928
  %_4.i151.us.us.us.i.i = fmul float %_0.i180.us.us.us.i.i, %_0.i236.us.us.us.i.i, !dbg !6930
  %_0.i152.us.us.us.i.i = fadd float %_86.i.i.us.us.us.i.i, %_4.i151.us.us.us.i.i, !dbg !6930
  %174 = tail call noundef float @llvm.fabs.f32(float %_0.i152.us.us.us.i.i), !dbg !6932
  %175 = fcmp uge float %174, 0x3BC79CA100000000, !dbg !6935
  %_0.i220.us.us.us.i.i = select i1 %175, float %_0.i152.us.us.us.i.i, float 0.000000e+00, !dbg !6937
  store float %_0.i220.us.us.us.i.i, ptr %113, align 4, !dbg !6938, !alias.scope !6766, !noalias !6767
  %_0.i165.us.us.us.i.i = fmul float %_0.i220.us.us.us.i.i, 0x3FC542A5A0000000, !dbg !6939
  %_3.i.i417.us.us.us.inv.i.i = fcmp ogt float %_0.i165.us.us.us.i.i, -1.260000e+02, !dbg !6942
  %_0.i.i424.us.us.us.i.i = select i1 %_3.i.i417.us.us.us.inv.i.i, float %_0.i165.us.us.us.i.i, float -1.260000e+02, !dbg !6942
  %_3.i.i500.us.us.us.inv.i.i = fcmp olt float %_0.i.i424.us.us.us.i.i, 1.270000e+02, !dbg !6946
  %_0.i.i507.us.us.us.i.i = select i1 %_3.i.i500.us.us.us.inv.i.i, float %_0.i.i424.us.us.us.i.i, float 1.270000e+02, !dbg !6946
  %176 = tail call noundef float @llvm.floor.f32(float %_0.i.i507.us.us.us.i.i), !dbg !6949
  %_0.i179.us.us.us.i.i = fsub float %_0.i.i507.us.us.us.i.i, %176, !dbg !6953
  %_0.i163.us.us.us.i.i = fmul float %_0.i179.us.us.us.i.i, 0x3F5E974FA0000000, !dbg !6955
  %_0.i146.us.us.us.i.i = fadd float %_0.i163.us.us.us.i.i, 0x3F82778560000000, !dbg !6957
  %_0.i163.us.us.us.1.i.i = fmul float %_0.i179.us.us.us.i.i, %_0.i146.us.us.us.i.i, !dbg !6955
  %_0.i146.us.us.us.1.i.i = fadd float %_0.i163.us.us.us.1.i.i, 0x3FAC91CE60000000, !dbg !6957
  %_0.i163.us.us.us.2.i.i = fmul float %_0.i179.us.us.us.i.i, %_0.i146.us.us.us.1.i.i, !dbg !6955
  %_0.i146.us.us.us.2.i.i = fadd float %_0.i163.us.us.us.2.i.i, 0x3FCEBDB560000000, !dbg !6957
  %_0.i163.us.us.us.3.i.i = fmul float %_0.i179.us.us.us.i.i, %_0.i146.us.us.us.2.i.i, !dbg !6955
  %_0.i146.us.us.us.3.i.i = fadd float %_0.i163.us.us.us.3.i.i, 0x3FE62E4BA0000000, !dbg !6957
  %_0.i161.us.us.us.i.i = fmul float %_0.i178.us.us.us.i.i, 0x3F5E974FA0000000, !dbg !6959
  %_0.i144.us.us.us.i.i = fadd float %_0.i161.us.us.us.i.i, 0x3F82778560000000, !dbg !6961
  %_0.i161.us.us.us.1.i.i = fmul float %_0.i178.us.us.us.i.i, %_0.i144.us.us.us.i.i, !dbg !6959
  %_0.i144.us.us.us.1.i.i = fadd float %_0.i161.us.us.us.1.i.i, 0x3FAC91CE60000000, !dbg !6961
  %_0.i161.us.us.us.2.i.i = fmul float %_0.i178.us.us.us.i.i, %_0.i144.us.us.us.1.i.i, !dbg !6959
  %_0.i144.us.us.us.2.i.i = fadd float %_0.i161.us.us.us.2.i.i, 0x3FCEBDB560000000, !dbg !6961
  %_0.i161.us.us.us.3.i.i = fmul float %_0.i178.us.us.us.i.i, %_0.i144.us.us.us.2.i.i, !dbg !6959
  %_0.i144.us.us.us.3.i.i = fadd float %_0.i161.us.us.us.3.i.i, 0x3FE62E4BA0000000, !dbg !6961
  %_0.i160.us.us.us.i.i = fmul float %_0.i178.us.us.us.i.i, %_0.i144.us.us.us.3.i.i, !dbg !6963
  %_0.i143.us.us.us.i.i = fadd float %_0.i160.us.us.us.i.i, 1.000000e+00, !dbg !6965
  %biased.i.us.us.us.i.i = fadd float %161, 0x4160000FE0000000, !dbg !6967
  %_4.i81.us.us.us.i.i = bitcast float %biased.i.us.us.us.i.i to i32, !dbg !6969
  %_3.i82.us.us.us.i.i = shl i32 %_4.i81.us.us.us.i.i, 23, !dbg !6971
  %_0.i83.us.us.us.i.i = bitcast i32 %_3.i82.us.us.us.i.i to float, !dbg !6972
  %_0.i159.us.us.us.i.i = fmul float %_0.i143.us.us.us.i.i, %_0.i83.us.us.us.i.i, !dbg !6974
  %_3.i91.us.us.us.i.i = fcmp une float %_0.i224.us.us.us.i.i, 0.000000e+00, !dbg !6976
  %_3.i119.us.us.us.i.i = fcmp ule float %_98.i115.i.us.us.us.i.i, 0.000000e+00, !dbg !6978
  %_0.i386568.not.us.us.us.i.i = and i1 %_3.i119.us.us.us.i.i, %_3.i91.us.us.us.i.i, !dbg !6980
  %_0.i170.us.us.us.i.i = fmul float %_0.i203.us.us.us.i.i, %_0.i159.us.us.us.i.i, !dbg !6980
  %_4.i308.v.us.us.us.i.i = select i1 %_0.i386568.not.us.us.us.i.i, float %_0.i170.us.us.us.i.i, float %_0.i203.us.us.us.i.i, !dbg !6982
  %_0.i.us.us.us.i.i = fmul float %_0.i179.us.us.us.i.i, %_0.i146.us.us.us.3.i.i, !dbg !6984
  %_0.i145.us.us.us.i.i = fadd float %_0.i.us.us.us.i.i, 1.000000e+00, !dbg !6986
  %biased.i84.us.us.us.i.i = fadd float %176, 0x4160000FE0000000, !dbg !6988
  %_4.i85.us.us.us.i.i = bitcast float %biased.i84.us.us.us.i.i to i32, !dbg !6990
  %_3.i86.us.us.us.i.i = shl i32 %_4.i85.us.us.us.i.i, 23, !dbg !6992
  %_0.i87.us.us.us.i.i = bitcast i32 %_3.i86.us.us.us.i.i to float, !dbg !6993
  %_0.i162.us.us.us.i.i = fmul float %_0.i145.us.us.us.i.i, %_0.i87.us.us.us.i.i, !dbg !6995
  %_3.i88.us.us.us.i.i = fcmp une float %_0.i220.us.us.us.i.i, 0.000000e+00, !dbg !6997
  %_98.i.i.us.us.us.i.i = load float, ptr %115, align 4, !dbg !6999, !alias.scope !6795, !noalias !6796, !noundef !12
  %_3.i103.us.us.us.i.i = fcmp ule float %_98.i.i.us.us.us.i.i, 0.000000e+00, !dbg !7000
  %_0.i383590.not.us.us.us.i.i = and i1 %_3.i103.us.us.us.i.i, %_3.i88.us.us.us.i.i, !dbg !7002
  %_0.i164.us.us.us.i.i = fmul float %_0.i201.us.us.us.i.i, %_0.i162.us.us.us.i.i, !dbg !7002
  %_4.i228.v.us.us.us.i.i = select i1 %_0.i383590.not.us.us.us.i.i, float %_0.i164.us.us.us.i.i, float %_0.i201.us.us.us.i.i, !dbg !7004
  store float %_4.i308.v.us.us.us.i.i, ptr %_97.i.us.us.us.i.i, align 4, !dbg !7006, !alias.scope !7009, !noalias !6459
  store float %_4.i228.v.us.us.us.i.i, ptr %_115.i.us.us.us.i.i, align 4, !dbg !7012, !alias.scope !7014, !noalias !6481
  %exitcond953.not.i.i = icmp eq i64 %140, %..i.i, !dbg !7017
  br i1 %exitcond953.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKb1_EB3_.exit.i, label %bb32.i.us.us.us.i.i, !dbg !7020

bb8.i79.us.us.us.i.i:                             ; preds = %bb5.i.preheader.us.us.us.i.i
  %_34.i.us.us.us.i.i = icmp samesign ugt i64 %_66.1.i.i, %_23.i77.us.us.us.i.i, !dbg !7021
  br i1 %_34.i.us.us.us.i.i, label %bb10.i80.us.us.us.i.i, label %panic5.i.i.i, !dbg !7021

bb10.i80.us.us.us.i.i:                            ; preds = %bb8.i79.us.us.us.i.i
  %177 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_15.i74.us.us.us.i.i, !dbg !7022
  %left_own.i.us.us.us.i.i = load float, ptr %177, align 4, !dbg !7022, !alias.scope !6534, !noalias !6549, !noundef !12
  %178 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_23.i77.us.us.us.i.i, !dbg !7021
  %right_own.i.us.us.us.i.i = load float, ptr %178, align 4, !dbg !7021, !alias.scope !6536, !noalias !6551, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i.i, !dbg !6554

bb5.i.preheader.us.us.us.i.i:                     ; preds = %bb50.i.us.us.us.i.i
  br i1 %_31.i78.us.us.us.i.i, label %bb8.i79.us.us.us.i.i, label %panic4.i.i.i, !dbg !7022

bb14.i63.preheader.us.us.us.i.i:                  ; preds = %bb50.i.us.us.us.i.i
  br i1 %_31.i78.us.us.us.i.i, label %bb17.i.us.us.us.i.i, label %panic15.i.i.i, !dbg !6558

bb23.i.preheader.us.us.us.i.i:                    ; preds = %bb50.i.us.us.us.i.i
  br i1 %_31.i78.us.us.us.i.i, label %bb27.i60.us.us.us.i.i, label %panic28.i.i.i, !dbg !6548

bb32.i.us.i.i:                                    ; preds = %bb18.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i.i
  %iter.sroa.0.0.i690.us.i.i = phi i64 [ %179, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i.i ], [ 0, %bb18.i ]
  %179 = add nuw nsw i64 %iter.sroa.0.0.i690.us.i.i, 1, !dbg !6417
  %_28.i.us.i.i = trunc i64 %iter.sroa.0.0.i690.us.i.i to i32, !dbg !6431
  %now.i.us.i.i = add i32 %base.i.i.i, %_28.i.us.i.i, !dbg !6434
  %_31.i.us.i.i = and i32 %now.i.us.i.i, %_58.i.i, !dbg !6437
  %_30.i.us.i.i = zext i32 %_31.i.us.i.i to i64, !dbg !6439
  %_97.i.us.i.i = getelementptr inbounds nuw float, ptr %_14.0, i64 %iter.sroa.0.0.i690.us.i.i, !dbg !6440
  %_98.not.not.i.us.i.i = icmp ugt i64 %_64.1.i.i, %_30.i.us.i.i, !dbg !6449
  br i1 %_98.not.not.i.us.i.i, label %bb35.i.us.i.i, label %bb36.i.i.i, !dbg !6449, !prof !2709

bb35.i.us.i.i:                                    ; preds = %bb32.i.us.i.i
  %_0.i207.us.i.i = load float, ptr %_97.i.us.i.i, align 4, !dbg !6454, !alias.scope !6456, !noalias !6459, !noundef !12
  %_107.i.us.i.i = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_30.i.us.i.i, !dbg !6460
  store float %_0.i207.us.i.i, ptr %_107.i.us.i.i, align 4, !dbg !6464, !alias.scope !6466, !noalias !6414
  %_115.i.us.i.i = getelementptr inbounds nuw float, ptr %_13.0, i64 %iter.sroa.0.0.i690.us.i.i, !dbg !6469
  %_116.not.not.i.us.i.i = icmp ugt i64 %_66.1.i.i, %_30.i.us.i.i, !dbg !7023
  br i1 %_116.not.not.i.us.i.i, label %bb40.i.us.i.i, label %bb41.i.i.i, !dbg !7023, !prof !2709

bb40.i.us.i.i:                                    ; preds = %bb35.i.us.i.i
  %_0.i205.us.i.i = load float, ptr %_115.i.us.i.i, align 4, !dbg !6476, !alias.scope !6478, !noalias !6481, !noundef !12
  %_123.i.us.i.i = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_30.i.us.i.i, !dbg !6482
  store float %_0.i205.us.i.i, ptr %_123.i.us.i.i, align 4, !dbg !6489, !alias.scope !6491, !noalias !6414
  %_53.i.us.i.i = sub i32 %now.i.us.i.i, %_59.i.i, !dbg !6494
  %_52.i.us.i.i = and i32 %_53.i.us.i.i, %_58.i.i, !dbg !6497
  %_51.i.us.i.i = zext i32 %_52.i.us.i.i to i64, !dbg !6498
  %_156.not.not.i.us.i.i = icmp ugt i64 %_64.1.i.i, %_51.i.us.i.i, !dbg !6499
  br i1 %_156.not.not.i.us.i.i, label %bb50.i.us.i.i, label %bb51.i.i.i, !dbg !6499, !prof !2709

bb50.i.us.i.i:                                    ; preds = %bb40.i.us.i.i
  %_163.i.us.i.i = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_51.i.us.i.i, !dbg !6504
  %_0.i203.us.i.i = load float, ptr %_163.i.us.i.i, align 4, !dbg !6508, !alias.scope !6510, !noalias !6414, !noundef !12
  %_164.not.not.i.us.i.i = icmp ugt i64 %_66.1.i.i, %_51.i.us.i.i, !dbg !7024
  br i1 %_164.not.not.i.us.i.i, label %bb53.i.us.i.i, label %bb54.i.i.i, !dbg !7024, !prof !2709

bb53.i.us.i.i:                                    ; preds = %bb50.i.us.i.i
  %_169.i.us.i.i = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_51.i.us.i.i, !dbg !6513
  %_0.i201.us.i.i = load float, ptr %_169.i.us.i.i, align 4, !dbg !6521, !alias.scope !6523, !noalias !6414, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6526), !dbg !6529
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6532), !dbg !6529
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6534), !dbg !6529
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6536), !dbg !6529
  %_18.i72.us.i.i = load i32, ptr %_31.i.i, align 4, !dbg !6538, !alias.scope !6540, !noalias !6541, !noundef !12
  %_17.i.us.i.i = sub i32 %now.i.us.i.i, %_18.i72.us.i.i, !dbg !6543
  %_16.i73.us.i.i = and i32 %_17.i.us.i.i, %_58.i.i, !dbg !6538
  %_15.i74.us.i.i = zext i32 %_16.i73.us.i.i to i64, !dbg !6538
  %_26.i.us.i.i = load i32, ptr %_33.i.i, align 4, !dbg !6538, !alias.scope !6545, !noalias !6546, !noundef !12
  %_25.i.us.i.i = sub i32 %now.i.us.i.i, %_26.i.us.i.i, !dbg !6543
  %_24.i.us.i.i = and i32 %_25.i.us.i.i, %_58.i.i, !dbg !6538
  %_23.i77.us.i.i = zext i32 %_24.i.us.i.i to i64, !dbg !6538
  %_31.i78.us.i.i = icmp samesign ugt i64 %_64.1.i.i, %_15.i74.us.i.i, !dbg !6538
  switch i8 %_0.sroa.0.0.i545.i.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.i.i
    i8 1, label %bb14.i63.preheader.us.i.i
    i8 2, label %bb23.i.preheader.us.i.i
  ], !dbg !6547

bb27.i60.us.i.i:                                  ; preds = %bb23.i.preheader.us.i.i
  %180 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_15.i74.us.i.i, !dbg !6548
  %_79.i.us.i.i = load float, ptr %180, align 4, !dbg !6548, !alias.scope !6534, !noalias !6549, !noundef !12
  %_85.i.us.i.i = icmp samesign ugt i64 %_66.1.i.i, %_15.i74.us.i.i, !dbg !6550
  br i1 %_85.i.us.i.i, label %bb29.i61.us.i.i, label %panic30.i.i.i, !dbg !6550

bb29.i61.us.i.i:                                  ; preds = %bb27.i60.us.i.i
  %_87.i.us.i.i = icmp samesign ugt i64 %_66.1.i.i, %_23.i77.us.i.i, !dbg !6552
  br i1 %_87.i.us.i.i, label %bb31.i.us.i.i, label %panic32.i.i.i, !dbg !6552

bb31.i.us.i.i:                                    ; preds = %bb29.i61.us.i.i
  %181 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_15.i74.us.i.i, !dbg !6550
  %_83.i.us.i.i = load float, ptr %181, align 4, !dbg !6550, !alias.scope !6536, !noalias !6551, !noundef !12
  %182 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_23.i77.us.i.i, !dbg !6552
  %_86.i.us.i.i = load float, ptr %182, align 4, !dbg !6552, !alias.scope !6536, !noalias !6551, !noundef !12
  %183 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_23.i77.us.i.i, !dbg !6553
  %_88.i.us.i.i = load float, ptr %183, align 4, !dbg !6553, !alias.scope !6534, !noalias !6549, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i.i, !dbg !6554

bb17.i.us.i.i:                                    ; preds = %bb14.i63.preheader.us.i.i
  %_59.i.us.i.i = icmp samesign ugt i64 %_66.1.i.i, %_23.i77.us.i.i, !dbg !6557
  br i1 %_59.i.us.i.i, label %bb19.i.us.i.i, label %panic17.i.i.i, !dbg !6557

bb19.i.us.i.i:                                    ; preds = %bb17.i.us.i.i
  %184 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_15.i74.us.i.i, !dbg !6558
  %left_own16.i.us.i.i = load float, ptr %184, align 4, !dbg !6558, !alias.scope !6534, !noalias !6549, !noundef !12
  %185 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_23.i77.us.i.i, !dbg !6557
  %right_own18.i.us.i.i = load float, ptr %185, align 4, !dbg !6557, !alias.scope !6536, !noalias !6551, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i.i, !dbg !6554

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i.i: ; preds = %bb10.i80.us.i.i, %bb19.i.us.i.i, %bb31.i.us.i.i
  %taps.i.sroa.41960.1.i.i = phi float [ %right_own.i.us.i.i, %bb10.i80.us.i.i ], [ %left_own16.i.us.i.i, %bb19.i.us.i.i ], [ %_88.i.us.i.i, %bb31.i.us.i.i ], !dbg !6538
  %taps.i.sroa.28959.1.i.i = phi float [ %right_own.i.us.i.i, %bb10.i80.us.i.i ], [ %right_own18.i.us.i.i, %bb19.i.us.i.i ], [ %_86.i.us.i.i, %bb31.i.us.i.i ], !dbg !6538
  %taps.i.sroa.15958.1.i.i = phi float [ %left_own.i.us.i.i, %bb10.i80.us.i.i ], [ %right_own18.i.us.i.i, %bb19.i.us.i.i ], [ %_83.i.us.i.i, %bb31.i.us.i.i ], !dbg !6538
  %taps.i.sroa.0.1.i.i = phi float [ %left_own.i.us.i.i, %bb10.i80.us.i.i ], [ %left_own16.i.us.i.i, %bb19.i.us.i.i ], [ %_79.i.us.i.i, %bb31.i.us.i.i ], !dbg !6538
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6559), !dbg !6562
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6563), !dbg !6562
  %_12.i28.i.us.i.i = load float, ptr %116, align 4, !dbg !6565, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i133.us.i.i = fcmp ule float %_12.i28.i.us.i.i, 0.000000e+00, !dbg !6569
  %_3.i93.us.i.i = fcmp une float %_12.i28.i.us.i.i, 1.000000e+00, !dbg !6571
  %_16.i31.i.us.i.i = load float, ptr %_10.i.i, align 4, !dbg !6573, !alias.scope !6567, !noalias !6568, !noundef !12
  %_17.i32.i.us.i.i = load float, ptr %117, align 4, !dbg !6574, !alias.scope !6567, !noalias !6568, !noundef !12
  %_0.i150.us.i.i = fadd float %_16.i31.i.us.i.i, %_17.i32.i.us.i.i, !dbg !6575
  %_20.i34.i548551.us.i.i = load float, ptr %118, align 4, !dbg !6577, !alias.scope !6567, !noalias !6568, !noundef !12
  %186 = select i1 %_3.i93.us.i.i, float %_0.i150.us.i.i, float %_20.i34.i548551.us.i.i, !dbg !6578
  %_0.i375.us.i.i = select i1 %_3.i133.us.i.i, float %_16.i31.i.us.i.i, float %186, !dbg !6580
  store float %_0.i375.us.i.i, ptr %_10.i.i, align 4, !dbg !6582, !alias.scope !6567, !noalias !6568
  %_0.i368.us.i.i = select i1 %_3.i93.us.i.i, float %_17.i32.i.us.i.i, float 0.000000e+00, !dbg !6583
  store float %_0.i368.us.i.i, ptr %117, align 4, !dbg !6585, !alias.scope !6567, !noalias !6568
  %_0.i191.us.i.i = fadd float %_12.i28.i.us.i.i, -1.000000e+00, !dbg !6586
  %_4.i361.v.us.i.i = select i1 %_3.i133.us.i.i, float %_12.i28.i.us.i.i, float %_0.i191.us.i.i, !dbg !6588
  store float %_4.i361.v.us.i.i, ptr %116, align 4, !dbg !6590, !alias.scope !6567, !noalias !6568
  %_12.i28.i.us.1.i.i = load float, ptr %119, align 4, !dbg !6565, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i133.us.1.i.i = fcmp ule float %_12.i28.i.us.1.i.i, 0.000000e+00, !dbg !6569
  %_3.i93.us.1.i.i = fcmp une float %_12.i28.i.us.1.i.i, 1.000000e+00, !dbg !6571
  %_16.i31.i.us.1.i.i = load float, ptr %iter1.sroa.0.0.ptr.i26.i.us.1.i.i, align 4, !dbg !6573, !alias.scope !6567, !noalias !6568, !noundef !12
  %_17.i32.i.us.1.i.i = load float, ptr %120, align 4, !dbg !6574, !alias.scope !6567, !noalias !6568, !noundef !12
  %_0.i150.us.1.i.i = fadd float %_16.i31.i.us.1.i.i, %_17.i32.i.us.1.i.i, !dbg !6575
  %_20.i34.i548551.us.1.i.i = load float, ptr %121, align 4, !dbg !6577, !alias.scope !6567, !noalias !6568, !noundef !12
  %187 = select i1 %_3.i93.us.1.i.i, float %_0.i150.us.1.i.i, float %_20.i34.i548551.us.1.i.i, !dbg !6578
  %_0.i375.us.1.i.i = select i1 %_3.i133.us.1.i.i, float %_16.i31.i.us.1.i.i, float %187, !dbg !6580
  store float %_0.i375.us.1.i.i, ptr %iter1.sroa.0.0.ptr.i26.i.us.1.i.i, align 4, !dbg !6582, !alias.scope !6567, !noalias !6568
  %_0.i368.us.1.i.i = select i1 %_3.i93.us.1.i.i, float %_17.i32.i.us.1.i.i, float 0.000000e+00, !dbg !6583
  store float %_0.i368.us.1.i.i, ptr %120, align 4, !dbg !6585, !alias.scope !6567, !noalias !6568
  %_0.i191.us.1.i.i = fadd float %_12.i28.i.us.1.i.i, -1.000000e+00, !dbg !6586
  %_4.i361.v.us.1.i.i = select i1 %_3.i133.us.1.i.i, float %_12.i28.i.us.1.i.i, float %_0.i191.us.1.i.i, !dbg !6588
  store float %_4.i361.v.us.1.i.i, ptr %119, align 4, !dbg !6590, !alias.scope !6567, !noalias !6568
  %_12.i28.i.us.2.i.i = load float, ptr %122, align 4, !dbg !6565, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i133.us.2.i.i = fcmp ule float %_12.i28.i.us.2.i.i, 0.000000e+00, !dbg !6569
  %_3.i93.us.2.i.i = fcmp une float %_12.i28.i.us.2.i.i, 1.000000e+00, !dbg !6571
  %_16.i31.i.us.2.i.i = load float, ptr %iter1.sroa.0.0.ptr.i26.i.us.2.i.i, align 4, !dbg !6573, !alias.scope !6567, !noalias !6568, !noundef !12
  %_17.i32.i.us.2.i.i = load float, ptr %123, align 4, !dbg !6574, !alias.scope !6567, !noalias !6568, !noundef !12
  %_0.i150.us.2.i.i = fadd float %_16.i31.i.us.2.i.i, %_17.i32.i.us.2.i.i, !dbg !6575
  %_20.i34.i548551.us.2.i.i = load float, ptr %124, align 4, !dbg !6577, !alias.scope !6567, !noalias !6568, !noundef !12
  %188 = select i1 %_3.i93.us.2.i.i, float %_0.i150.us.2.i.i, float %_20.i34.i548551.us.2.i.i, !dbg !6578
  %_0.i375.us.2.i.i = select i1 %_3.i133.us.2.i.i, float %_16.i31.i.us.2.i.i, float %188, !dbg !6580
  store float %_0.i375.us.2.i.i, ptr %iter1.sroa.0.0.ptr.i26.i.us.2.i.i, align 4, !dbg !6582, !alias.scope !6567, !noalias !6568
  %_0.i368.us.2.i.i = select i1 %_3.i93.us.2.i.i, float %_17.i32.i.us.2.i.i, float 0.000000e+00, !dbg !6583
  store float %_0.i368.us.2.i.i, ptr %123, align 4, !dbg !6585, !alias.scope !6567, !noalias !6568
  %_0.i191.us.2.i.i = fadd float %_12.i28.i.us.2.i.i, -1.000000e+00, !dbg !6586
  %_4.i361.v.us.2.i.i = select i1 %_3.i133.us.2.i.i, float %_12.i28.i.us.2.i.i, float %_0.i191.us.2.i.i, !dbg !6588
  store float %_4.i361.v.us.2.i.i, ptr %122, align 4, !dbg !6590, !alias.scope !6567, !noalias !6568
  %_12.i28.i.us.3.i.i = load float, ptr %125, align 4, !dbg !6565, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i133.us.3.i.i = fcmp ule float %_12.i28.i.us.3.i.i, 0.000000e+00, !dbg !6569
  %_3.i93.us.3.i.i = fcmp une float %_12.i28.i.us.3.i.i, 1.000000e+00, !dbg !6571
  %_16.i31.i.us.3.i.i = load float, ptr %iter1.sroa.0.0.ptr.i26.i.us.3.i.i, align 4, !dbg !6573, !alias.scope !6567, !noalias !6568, !noundef !12
  %_17.i32.i.us.3.i.i = load float, ptr %126, align 4, !dbg !6574, !alias.scope !6567, !noalias !6568, !noundef !12
  %_0.i150.us.3.i.i = fadd float %_16.i31.i.us.3.i.i, %_17.i32.i.us.3.i.i, !dbg !6575
  %_20.i34.i548551.us.3.i.i = load float, ptr %127, align 4, !dbg !6577, !alias.scope !6567, !noalias !6568, !noundef !12
  %189 = select i1 %_3.i93.us.3.i.i, float %_0.i150.us.3.i.i, float %_20.i34.i548551.us.3.i.i, !dbg !6578
  %_0.i375.us.3.i.i = select i1 %_3.i133.us.3.i.i, float %_16.i31.i.us.3.i.i, float %189, !dbg !6580
  store float %_0.i375.us.3.i.i, ptr %iter1.sroa.0.0.ptr.i26.i.us.3.i.i, align 4, !dbg !6582, !alias.scope !6567, !noalias !6568
  %_0.i368.us.3.i.i = select i1 %_3.i93.us.3.i.i, float %_17.i32.i.us.3.i.i, float 0.000000e+00, !dbg !6583
  store float %_0.i368.us.3.i.i, ptr %126, align 4, !dbg !6585, !alias.scope !6567, !noalias !6568
  %_0.i191.us.3.i.i = fadd float %_12.i28.i.us.3.i.i, -1.000000e+00, !dbg !6586
  %_4.i361.v.us.3.i.i = select i1 %_3.i133.us.3.i.i, float %_12.i28.i.us.3.i.i, float %_0.i191.us.3.i.i, !dbg !6588
  store float %_4.i361.v.us.3.i.i, ptr %125, align 4, !dbg !6590, !alias.scope !6567, !noalias !6568
  %190 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.1.i.i), !dbg !6591
  %191 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.15958.1.i.i), !dbg !6593
  %_37.i49.i.us.i.i = load float, ptr %101, align 4, !dbg !6595, !alias.scope !6596, !noalias !6597, !noundef !12
  %_3.i131.us.i.i = fcmp ule float %_37.i49.i.us.i.i, 0.000000e+00, !dbg !6598
  %_3.i.i483.us.i.i = fcmp ule float %190, %191, !dbg !6600
  %_6.i.i485.us.i.i = bitcast float %190 to i32, !dbg !6603
  %_8.i.i487.us.i.i = bitcast float %191 to i32, !dbg !6606
  %_4.i.i490.us.i.i = select i1 %_3.i.i483.us.i.i, i32 %_8.i.i487.us.i.i, i32 %_6.i.i485.us.i.i, !dbg !6608
  %_4.i354.us.i.i = select i1 %_3.i131.us.i.i, i32 %_6.i.i485.us.i.i, i32 %_4.i.i490.us.i.i, !dbg !6609
  %_41.i53.i.us.i.i = load float, ptr %102, align 4, !dbg !6611, !alias.scope !6596, !noalias !6597, !noundef !12
  %_3.i129.us.i.i = fcmp ule float %_41.i53.i.us.i.i, 0.000000e+00, !dbg !6612
  %_0.i175.us.i.i = fmul float %190, 5.000000e-01, !dbg !6614
  %_0.i174.us.i.i = fmul float %191, 5.000000e-01, !dbg !6616
  %_0.i149.us.i.i = fadd float %_0.i174.us.i.i, %_0.i175.us.i.i, !dbg !6618
  %_6.i342.us.i.i = bitcast float %_0.i149.us.i.i to i32, !dbg !6620
  %_4.i347.us.i.i = select i1 %_3.i129.us.i.i, i32 %_4.i354.us.i.i, i32 %_6.i342.us.i.i, !dbg !6623
  %_0.i348.us.i.i = bitcast i32 %_4.i347.us.i.i to float, !dbg !6624
  %_3.i.i475.us.i.i = fcmp ule float %_0.i348.us.i.i, 0x3E45798EE0000000, !dbg !6626
  %_4.i.i481.us.i.i = select i1 %_3.i.i475.us.i.i, i32 841731191, i32 %_4.i347.us.i.i, !dbg !6629
  %_0.i.i482.us.i.i = bitcast i32 %_4.i.i481.us.i.i to float, !dbg !6631
  %_3.i.i.us.i.i = fcmp ule float %_0.i.i482.us.i.i, 0x3810000000000000, !dbg !6633
  %_4.i.i.us.i.i = select i1 %_3.i.i.us.i.i, i32 8388608, i32 %_4.i.i481.us.i.i, !dbg !6638
  %_5.i208.us.i.i = and i32 %_4.i.i.us.i.i, 8388607, !dbg !6640
  %_4.i209.us.i.i = or disjoint i32 %_5.i208.us.i.i, 1065353216, !dbg !6640
  %significand.i.us.i.i = bitcast i32 %_4.i209.us.i.i to float, !dbg !6642
  %_0.i176.us.i.i = fadd float %significand.i.us.i.i, -1.000000e+00, !dbg !6644
  %_0.i156.us.i.i = fmul float %_0.i176.us.i.i, 0x3F9B17A960000000, !dbg !6646
  %192 = fsub float 0x3FBF9A8440000000, %_0.i156.us.i.i, !dbg !6648
  %_0.i156.us.1.i.i = fmul float %_0.i176.us.i.i, %192, !dbg !6646
  %_0.i140.us.1.i.i = fadd float %_0.i156.us.1.i.i, 0xBFD1E3F400000000, !dbg !6648
  %_0.i156.us.2.i.i = fmul float %_0.i176.us.i.i, %_0.i140.us.1.i.i, !dbg !6646
  %_0.i140.us.2.i.i = fadd float %_0.i156.us.2.i.i, 0x3FDD544F20000000, !dbg !6648
  %_0.i156.us.3.i.i = fmul float %_0.i176.us.i.i, %_0.i140.us.2.i.i, !dbg !6646
  %_0.i140.us.3.i.i = fadd float %_0.i156.us.3.i.i, 0xBFE6FC2A60000000, !dbg !6648
  %_0.i156.us.4.i.i = fmul float %_0.i176.us.i.i, %_0.i140.us.3.i.i, !dbg !6646
  %_0.i140.us.4.i.i = fadd float %_0.i156.us.4.i.i, 0x3FF714B2A0000000, !dbg !6648
  %_9.i.us.i.i = lshr i32 %_4.i.i.us.i.i, 23, !dbg !6650
  %_8.i210.us.i.i = or disjoint i32 %_9.i.us.i.i, 1258291200, !dbg !6650
  %_7.i.us.i.i = bitcast i32 %_8.i210.us.i.i to float, !dbg !6651
  %exponent.i.us.i.i = fadd float %_7.i.us.i.i, 0xC160000FE0000000, !dbg !6653
  %_0.i155.us.i.i = fmul float %_0.i176.us.i.i, %_0.i140.us.4.i.i, !dbg !6654
  %_0.i139.us.i.i = fadd float %exponent.i.us.i.i, %_0.i155.us.i.i, !dbg !6656
  %_0.i173.us.i.i = fmul float %_0.i139.us.i.i, 0x4018151820000000, !dbg !6658
  %_3.i.i532.us.inv.i.i = fcmp olt float %_0.i173.us.i.i, 2.400000e+01, !dbg !6660
  %_0.i.i539.us.i.i = select i1 %_3.i.i532.us.inv.i.i, float %_0.i173.us.i.i, float 2.400000e+01, !dbg !6660
  %_3.i.i467.us.inv.i.i = fcmp ogt float %_0.i.i539.us.i.i, -1.600000e+02, !dbg !6663
  %_0.i.i474.us.i.i = select i1 %_3.i.i467.us.inv.i.i, float %_0.i.i539.us.i.i, float -1.600000e+02, !dbg !6663
  %_55.i70.i.us.i.i = load float, ptr %100, align 4, !dbg !6666, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i127.us.i.i = fcmp ule float %_55.i70.i.us.i.i, 0.000000e+00, !dbg !6667
  %_3.i101.us.i.i = fcmp oge float %_0.i.i474.us.i.i, %_0.i375.us.i.i, !dbg !6669
  %_0.i190.us.i.i = fsub float %_0.i375.us.i.i, %_0.i375.us.3.i.i, !dbg !6671
  %_3.i99.us.i.i = fcmp oge float %_0.i.i474.us.i.i, %_0.i190.us.i.i, !dbg !6673
  %..i100.us.i.i = sext i1 %_3.i99.us.i.i to i32, !dbg !6675
  %_0.i395.us.i.i = sext i1 %_3.i101.us.i.i to i32, !dbg !6677
  %_0.i388.us.i.i = select i1 %_3.i127.us.i.i, i32 %_0.i395.us.i.i, i32 %..i100.us.i.i, !dbg !6677
  %_0.i399.us.i.i = xor i32 %..i100.us.i.i, -1, !dbg !6679
  %_67.i80.i.us.i.i = load float, ptr %103, align 4, !dbg !6681, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i125.us.i.i = fcmp ogt float %_67.i80.i.us.i.i, 0.000000e+00, !dbg !6682
  %_0.i394.us.i.i = select i1 %_3.i125.us.i.i, i32 %_0.i399.us.i.i, i32 0, !dbg !6684
  %_0.i393.us.i.i = select i1 %_3.i127.us.i.i, i32 0, i32 %_0.i394.us.i.i, !dbg !6686
  %_0.i387.us.i.i = or i32 %_0.i393.us.i.i, %_0.i388.us.i.i, !dbg !6688
  %_5.i337.us.i.i = and i32 %_0.i387.us.i.i, 1065353216, !dbg !6690
  %_0.i341.us.i.i = bitcast i32 %_5.i337.us.i.i to float, !dbg !6692
  %_71.i86.i560561.us.i.i = load float, ptr %104, align 4, !dbg !6694, !alias.scope !6596, !noalias !6597, !noundef !12
  %_0.i189.us.i.i = fadd float %_67.i80.i.us.i.i, -1.000000e+00, !dbg !6695
  %193 = trunc nsw i32 %_0.i393.us.i.i to i1, !dbg !6697
  %_4.i335.v.us.i.i = select i1 %193, float %_0.i189.us.i.i, float %_67.i80.i.us.i.i, !dbg !6697
  %194 = trunc nsw i32 %_0.i388.us.i.i to i1, !dbg !6699
  %_0.i329.us.i.i = select i1 %194, float %_71.i86.i560561.us.i.i, float %_4.i335.v.us.i.i, !dbg !6699
  store float %_0.i329.us.i.i, ptr %103, align 4, !dbg !6701, !alias.scope !6567, !noalias !6568
  store i32 %_5.i337.us.i.i, ptr %100, align 4, !dbg !6702, !alias.scope !6567, !noalias !6568
  %_0.i188.us.i.i = fadd float %_0.i375.us.1.i.i, -1.000000e+00, !dbg !6703
  %_0.i187.us.i.i = fsub float %_0.i.i474.us.i.i, %_0.i375.us.i.i, !dbg !6705
  %_0.i172.us.i.i = fmul float %_0.i188.us.i.i, %_0.i187.us.i.i, !dbg !6707
  %195 = fneg float %_0.i375.us.2.i.i, !dbg !6709
  %_3.i.i458.inv.us.i.i = fcmp ogt float %_0.i172.us.i.i, %195, !dbg !6711
  %_4.i.i465.v.us.i.i = select i1 %_3.i.i458.inv.us.i.i, float %_0.i172.us.i.i, float %195, !dbg !6711
  %_3.i.i524.us.i.i = fcmp olt float %_4.i.i465.v.us.i.i, 0.000000e+00, !dbg !6714
  %196 = fcmp ule float %_0.i341.us.i.i, 0.000000e+00, !dbg !6717
  %197 = select i1 %196, i1 %_3.i.i524.us.i.i, i1 false, !dbg !6719
  %_0.i322.us.i.i = select i1 %197, float %_4.i.i465.v.us.i.i, float 0.000000e+00, !dbg !6719
  %_86.i99.i.us.i.i = load float, ptr %105, align 4, !dbg !6720, !alias.scope !6567, !noalias !6568, !noundef !12
  %_3.i121.us.i.i = fcmp ule float %_0.i322.us.i.i, %_86.i99.i.us.i.i, !dbg !6721
  %_87.i101.i563.us.i.i = load i32, ptr %95, align 4, !dbg !6723, !alias.scope !6596, !noalias !6597, !noundef !12
  %_88.i102.i564.us.i.i = load i32, ptr %106, align 4, !dbg !6724, !alias.scope !6596, !noalias !6597, !noundef !12
  %_4.i315.us.i.i = select i1 %_3.i121.us.i.i, i32 %_88.i102.i564.us.i.i, i32 %_87.i101.i563.us.i.i, !dbg !6725
  %_0.i316.us.i.i = bitcast i32 %_4.i315.us.i.i to float, !dbg !6727
  %_0.i186.us.i.i = fsub float %_0.i322.us.i.i, %_86.i99.i.us.i.i, !dbg !6729
  %_4.i153.us.i.i = fmul float %_0.i186.us.i.i, %_0.i316.us.i.i, !dbg !6731
  %_0.i154.us.i.i = fadd float %_86.i99.i.us.i.i, %_4.i153.us.i.i, !dbg !6731
  %198 = tail call noundef float @llvm.fabs.f32(float %_0.i154.us.i.i), !dbg !6733
  %199 = fcmp uge float %198, 0x3BC79CA100000000, !dbg !6736
  %_0.i224.us.i.i = select i1 %199, float %_0.i154.us.i.i, float 0.000000e+00, !dbg !6738
  store float %_0.i224.us.i.i, ptr %105, align 4, !dbg !6739, !alias.scope !6567, !noalias !6568
  %_0.i171.us.i.i = fmul float %_0.i224.us.i.i, 0x3FC542A5A0000000, !dbg !6740
  %_3.i.i409.us.inv.i.i = fcmp ogt float %_0.i171.us.i.i, -1.260000e+02, !dbg !6743
  %_0.i.i416.us.i.i = select i1 %_3.i.i409.us.inv.i.i, float %_0.i171.us.i.i, float -1.260000e+02, !dbg !6743
  %_3.i.i492.us.inv.i.i = fcmp olt float %_0.i.i416.us.i.i, 1.270000e+02, !dbg !6747
  %_0.i.i499.us.i.i = select i1 %_3.i.i492.us.inv.i.i, float %_0.i.i416.us.i.i, float 1.270000e+02, !dbg !6747
  %200 = tail call noundef float @llvm.floor.f32(float %_0.i.i499.us.i.i), !dbg !6750
  %_0.i178.us.i.i = fsub float %_0.i.i499.us.i.i, %200, !dbg !6754
  %_98.i115.i.us.i.i = load float, ptr %107, align 4, !dbg !6756, !alias.scope !6596, !noalias !6597, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6757), !dbg !6760
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6762), !dbg !6760
  %_12.i.i.us.i.i = load float, ptr %128, align 4, !dbg !6764, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i117.us.i.i = fcmp ule float %_12.i.i.us.i.i, 0.000000e+00, !dbg !6768
  %_3.i89.us.i.i = fcmp une float %_12.i.i.us.i.i, 1.000000e+00, !dbg !6770
  %_16.i.i.us.i.i = load float, ptr %data.i.i.i.i, align 4, !dbg !6772, !alias.scope !6766, !noalias !6767, !noundef !12
  %_17.i.i.us.i.i = load float, ptr %129, align 4, !dbg !6773, !alias.scope !6766, !noalias !6767, !noundef !12
  %_0.i148.us.i.i = fadd float %_16.i.i.us.i.i, %_17.i.i.us.i.i, !dbg !6774
  %_20.i.i570573.us.i.i = load float, ptr %130, align 4, !dbg !6776, !alias.scope !6766, !noalias !6767, !noundef !12
  %201 = select i1 %_3.i89.us.i.i, float %_0.i148.us.i.i, float %_20.i.i570573.us.i.i, !dbg !6777
  %_0.i295.us.i.i = select i1 %_3.i117.us.i.i, float %_16.i.i.us.i.i, float %201, !dbg !6779
  store float %_0.i295.us.i.i, ptr %data.i.i.i.i, align 4, !dbg !6781, !alias.scope !6766, !noalias !6767
  %_0.i288.us.i.i = select i1 %_3.i89.us.i.i, float %_17.i.i.us.i.i, float 0.000000e+00, !dbg !6782
  store float %_0.i288.us.i.i, ptr %129, align 4, !dbg !6784, !alias.scope !6766, !noalias !6767
  %_0.i185.us.i.i = fadd float %_12.i.i.us.i.i, -1.000000e+00, !dbg !6785
  %_4.i281.v.us.i.i = select i1 %_3.i117.us.i.i, float %_12.i.i.us.i.i, float %_0.i185.us.i.i, !dbg !6787
  store float %_4.i281.v.us.i.i, ptr %128, align 4, !dbg !6789, !alias.scope !6766, !noalias !6767
  %_12.i.i.us.1.i.i = load float, ptr %131, align 4, !dbg !6764, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i117.us.1.i.i = fcmp ule float %_12.i.i.us.1.i.i, 0.000000e+00, !dbg !6768
  %_3.i89.us.1.i.i = fcmp une float %_12.i.i.us.1.i.i, 1.000000e+00, !dbg !6770
  %_16.i.i.us.1.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.1.i.i, align 4, !dbg !6772, !alias.scope !6766, !noalias !6767, !noundef !12
  %_17.i.i.us.1.i.i = load float, ptr %132, align 4, !dbg !6773, !alias.scope !6766, !noalias !6767, !noundef !12
  %_0.i148.us.1.i.i = fadd float %_16.i.i.us.1.i.i, %_17.i.i.us.1.i.i, !dbg !6774
  %_20.i.i570573.us.1.i.i = load float, ptr %133, align 4, !dbg !6776, !alias.scope !6766, !noalias !6767, !noundef !12
  %202 = select i1 %_3.i89.us.1.i.i, float %_0.i148.us.1.i.i, float %_20.i.i570573.us.1.i.i, !dbg !6777
  %_0.i295.us.1.i.i = select i1 %_3.i117.us.1.i.i, float %_16.i.i.us.1.i.i, float %202, !dbg !6779
  store float %_0.i295.us.1.i.i, ptr %iter1.sroa.0.0.ptr.i.i.us.1.i.i, align 4, !dbg !6781, !alias.scope !6766, !noalias !6767
  %_0.i288.us.1.i.i = select i1 %_3.i89.us.1.i.i, float %_17.i.i.us.1.i.i, float 0.000000e+00, !dbg !6782
  store float %_0.i288.us.1.i.i, ptr %132, align 4, !dbg !6784, !alias.scope !6766, !noalias !6767
  %_0.i185.us.1.i.i = fadd float %_12.i.i.us.1.i.i, -1.000000e+00, !dbg !6785
  %_4.i281.v.us.1.i.i = select i1 %_3.i117.us.1.i.i, float %_12.i.i.us.1.i.i, float %_0.i185.us.1.i.i, !dbg !6787
  store float %_4.i281.v.us.1.i.i, ptr %131, align 4, !dbg !6789, !alias.scope !6766, !noalias !6767
  %_12.i.i.us.2.i.i = load float, ptr %134, align 4, !dbg !6764, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i117.us.2.i.i = fcmp ule float %_12.i.i.us.2.i.i, 0.000000e+00, !dbg !6768
  %_3.i89.us.2.i.i = fcmp une float %_12.i.i.us.2.i.i, 1.000000e+00, !dbg !6770
  %_16.i.i.us.2.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.2.i.i, align 4, !dbg !6772, !alias.scope !6766, !noalias !6767, !noundef !12
  %_17.i.i.us.2.i.i = load float, ptr %135, align 4, !dbg !6773, !alias.scope !6766, !noalias !6767, !noundef !12
  %_0.i148.us.2.i.i = fadd float %_16.i.i.us.2.i.i, %_17.i.i.us.2.i.i, !dbg !6774
  %_20.i.i570573.us.2.i.i = load float, ptr %136, align 4, !dbg !6776, !alias.scope !6766, !noalias !6767, !noundef !12
  %203 = select i1 %_3.i89.us.2.i.i, float %_0.i148.us.2.i.i, float %_20.i.i570573.us.2.i.i, !dbg !6777
  %_0.i295.us.2.i.i = select i1 %_3.i117.us.2.i.i, float %_16.i.i.us.2.i.i, float %203, !dbg !6779
  store float %_0.i295.us.2.i.i, ptr %iter1.sroa.0.0.ptr.i.i.us.2.i.i, align 4, !dbg !6781, !alias.scope !6766, !noalias !6767
  %_0.i288.us.2.i.i = select i1 %_3.i89.us.2.i.i, float %_17.i.i.us.2.i.i, float 0.000000e+00, !dbg !6782
  store float %_0.i288.us.2.i.i, ptr %135, align 4, !dbg !6784, !alias.scope !6766, !noalias !6767
  %_0.i185.us.2.i.i = fadd float %_12.i.i.us.2.i.i, -1.000000e+00, !dbg !6785
  %_4.i281.v.us.2.i.i = select i1 %_3.i117.us.2.i.i, float %_12.i.i.us.2.i.i, float %_0.i185.us.2.i.i, !dbg !6787
  store float %_4.i281.v.us.2.i.i, ptr %134, align 4, !dbg !6789, !alias.scope !6766, !noalias !6767
  %_12.i.i.us.3.i.i = load float, ptr %137, align 4, !dbg !6764, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i117.us.3.i.i = fcmp ule float %_12.i.i.us.3.i.i, 0.000000e+00, !dbg !6768
  %_3.i89.us.3.i.i = fcmp une float %_12.i.i.us.3.i.i, 1.000000e+00, !dbg !6770
  %_16.i.i.us.3.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.i.us.3.i.i, align 4, !dbg !6772, !alias.scope !6766, !noalias !6767, !noundef !12
  %_17.i.i.us.3.i.i = load float, ptr %138, align 4, !dbg !6773, !alias.scope !6766, !noalias !6767, !noundef !12
  %_0.i148.us.3.i.i = fadd float %_16.i.i.us.3.i.i, %_17.i.i.us.3.i.i, !dbg !6774
  %_20.i.i570573.us.3.i.i = load float, ptr %139, align 4, !dbg !6776, !alias.scope !6766, !noalias !6767, !noundef !12
  %204 = select i1 %_3.i89.us.3.i.i, float %_0.i148.us.3.i.i, float %_20.i.i570573.us.3.i.i, !dbg !6777
  %_0.i295.us.3.i.i = select i1 %_3.i117.us.3.i.i, float %_16.i.i.us.3.i.i, float %204, !dbg !6779
  store float %_0.i295.us.3.i.i, ptr %iter1.sroa.0.0.ptr.i.i.us.3.i.i, align 4, !dbg !6781, !alias.scope !6766, !noalias !6767
  %_0.i288.us.3.i.i = select i1 %_3.i89.us.3.i.i, float %_17.i.i.us.3.i.i, float 0.000000e+00, !dbg !6782
  store float %_0.i288.us.3.i.i, ptr %138, align 4, !dbg !6784, !alias.scope !6766, !noalias !6767
  %_0.i185.us.3.i.i = fadd float %_12.i.i.us.3.i.i, -1.000000e+00, !dbg !6785
  %_4.i281.v.us.3.i.i = select i1 %_3.i117.us.3.i.i, float %_12.i.i.us.3.i.i, float %_0.i185.us.3.i.i, !dbg !6787
  store float %_4.i281.v.us.3.i.i, ptr %137, align 4, !dbg !6789, !alias.scope !6766, !noalias !6767
  %205 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.28959.1.i.i), !dbg !6790
  %206 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.41960.1.i.i), !dbg !6792
  %_37.i.i.us.i.i = load float, ptr %109, align 4, !dbg !6794, !alias.scope !6795, !noalias !6796, !noundef !12
  %_3.i115.us.i.i = fcmp ule float %_37.i.i.us.i.i, 0.000000e+00, !dbg !6797
  %_3.i.i449.us.i.i = fcmp ule float %205, %206, !dbg !6799
  %_6.i.i451.us.i.i = bitcast float %205 to i32, !dbg !6802
  %_8.i.i453.us.i.i = bitcast float %206 to i32, !dbg !6805
  %_4.i.i456.us.i.i = select i1 %_3.i.i449.us.i.i, i32 %_8.i.i453.us.i.i, i32 %_6.i.i451.us.i.i, !dbg !6807
  %_4.i274.us.i.i = select i1 %_3.i115.us.i.i, i32 %_6.i.i451.us.i.i, i32 %_4.i.i456.us.i.i, !dbg !6808
  %_41.i.i.us.i.i = load float, ptr %110, align 4, !dbg !6810, !alias.scope !6795, !noalias !6796, !noundef !12
  %_3.i113.us.i.i = fcmp ule float %_41.i.i.us.i.i, 0.000000e+00, !dbg !6811
  %_0.i169.us.i.i = fmul float %205, 5.000000e-01, !dbg !6813
  %_0.i168.us.i.i = fmul float %206, 5.000000e-01, !dbg !6815
  %_0.i147.us.i.i = fadd float %_0.i168.us.i.i, %_0.i169.us.i.i, !dbg !6817
  %_6.i262.us.i.i = bitcast float %_0.i147.us.i.i to i32, !dbg !6819
  %_4.i267.us.i.i = select i1 %_3.i113.us.i.i, i32 %_4.i274.us.i.i, i32 %_6.i262.us.i.i, !dbg !6822
  %_0.i268.us.i.i = bitcast i32 %_4.i267.us.i.i to float, !dbg !6823
  %_3.i.i441.us.i.i = fcmp ule float %_0.i268.us.i.i, 0x3E45798EE0000000, !dbg !6825
  %_4.i.i447.us.i.i = select i1 %_3.i.i441.us.i.i, i32 841731191, i32 %_4.i267.us.i.i, !dbg !6828
  %_0.i.i448.us.i.i = bitcast i32 %_4.i.i447.us.i.i to float, !dbg !6830
  %_3.i.i401.us.i.i = fcmp ule float %_0.i.i448.us.i.i, 0x3810000000000000, !dbg !6832
  %_4.i.i407.us.i.i = select i1 %_3.i.i401.us.i.i, i32 8388608, i32 %_4.i.i447.us.i.i, !dbg !6837
  %_5.i212.us.i.i = and i32 %_4.i.i407.us.i.i, 8388607, !dbg !6839
  %_4.i213.us.i.i = or disjoint i32 %_5.i212.us.i.i, 1065353216, !dbg !6839
  %significand.i214.us.i.i = bitcast i32 %_4.i213.us.i.i to float, !dbg !6841
  %_0.i177.us.i.i = fadd float %significand.i214.us.i.i, -1.000000e+00, !dbg !6843
  %_0.i158.us.i.i = fmul float %_0.i177.us.i.i, 0x3F9B17A960000000, !dbg !6845
  %207 = fsub float 0x3FBF9A8440000000, %_0.i158.us.i.i, !dbg !6847
  %_0.i158.us.1.i.i = fmul float %_0.i177.us.i.i, %207, !dbg !6845
  %_0.i142.us.1.i.i = fadd float %_0.i158.us.1.i.i, 0xBFD1E3F400000000, !dbg !6847
  %_0.i158.us.2.i.i = fmul float %_0.i177.us.i.i, %_0.i142.us.1.i.i, !dbg !6845
  %_0.i142.us.2.i.i = fadd float %_0.i158.us.2.i.i, 0x3FDD544F20000000, !dbg !6847
  %_0.i158.us.3.i.i = fmul float %_0.i177.us.i.i, %_0.i142.us.2.i.i, !dbg !6845
  %_0.i142.us.3.i.i = fadd float %_0.i158.us.3.i.i, 0xBFE6FC2A60000000, !dbg !6847
  %_0.i158.us.4.i.i = fmul float %_0.i177.us.i.i, %_0.i142.us.3.i.i, !dbg !6845
  %_0.i142.us.4.i.i = fadd float %_0.i158.us.4.i.i, 0x3FF714B2A0000000, !dbg !6847
  %_9.i215.us.i.i = lshr i32 %_4.i.i407.us.i.i, 23, !dbg !6849
  %_8.i216.us.i.i = or disjoint i32 %_9.i215.us.i.i, 1258291200, !dbg !6849
  %_7.i217.us.i.i = bitcast i32 %_8.i216.us.i.i to float, !dbg !6850
  %exponent.i218.us.i.i = fadd float %_7.i217.us.i.i, 0xC160000FE0000000, !dbg !6852
  %_0.i157.us.i.i = fmul float %_0.i177.us.i.i, %_0.i142.us.4.i.i, !dbg !6853
  %_0.i141.us.i.i = fadd float %exponent.i218.us.i.i, %_0.i157.us.i.i, !dbg !6855
  %_0.i167.us.i.i = fmul float %_0.i141.us.i.i, 0x4018151820000000, !dbg !6857
  %_3.i.i516.us.inv.i.i = fcmp olt float %_0.i167.us.i.i, 2.400000e+01, !dbg !6859
  %_0.i.i523.us.i.i = select i1 %_3.i.i516.us.inv.i.i, float %_0.i167.us.i.i, float 2.400000e+01, !dbg !6859
  %_3.i.i433.us.inv.i.i = fcmp ogt float %_0.i.i523.us.i.i, -1.600000e+02, !dbg !6862
  %_0.i.i440.us.i.i = select i1 %_3.i.i433.us.inv.i.i, float %_0.i.i523.us.i.i, float -1.600000e+02, !dbg !6862
  %_55.i.i.us.i.i = load float, ptr %108, align 4, !dbg !6865, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i111.us.i.i = fcmp ule float %_55.i.i.us.i.i, 0.000000e+00, !dbg !6866
  %_3.i97.us.i.i = fcmp oge float %_0.i.i440.us.i.i, %_0.i295.us.i.i, !dbg !6868
  %_0.i184.us.i.i = fsub float %_0.i295.us.i.i, %_0.i295.us.3.i.i, !dbg !6870
  %_3.i95.us.i.i = fcmp oge float %_0.i.i440.us.i.i, %_0.i184.us.i.i, !dbg !6872
  %..i96.us.i.i = sext i1 %_3.i95.us.i.i to i32, !dbg !6874
  %_0.i391.us.i.i = sext i1 %_3.i97.us.i.i to i32, !dbg !6876
  %_0.i385.us.i.i = select i1 %_3.i111.us.i.i, i32 %_0.i391.us.i.i, i32 %..i96.us.i.i, !dbg !6876
  %_0.i397.us.i.i = xor i32 %..i96.us.i.i, -1, !dbg !6878
  %_67.i.i.us.i.i = load float, ptr %111, align 4, !dbg !6880, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i109.us.i.i = fcmp ogt float %_67.i.i.us.i.i, 0.000000e+00, !dbg !6881
  %_0.i390.us.i.i = select i1 %_3.i109.us.i.i, i32 %_0.i397.us.i.i, i32 0, !dbg !6883
  %_0.i389.us.i.i = select i1 %_3.i111.us.i.i, i32 0, i32 %_0.i390.us.i.i, !dbg !6885
  %_0.i384.us.i.i = or i32 %_0.i389.us.i.i, %_0.i385.us.i.i, !dbg !6887
  %_5.i257.us.i.i = and i32 %_0.i384.us.i.i, 1065353216, !dbg !6889
  %_0.i261.us.i.i = bitcast i32 %_5.i257.us.i.i to float, !dbg !6891
  %_71.i.i582583.us.i.i = load float, ptr %112, align 4, !dbg !6893, !alias.scope !6795, !noalias !6796, !noundef !12
  %_0.i183.us.i.i = fadd float %_67.i.i.us.i.i, -1.000000e+00, !dbg !6894
  %208 = trunc nsw i32 %_0.i389.us.i.i to i1, !dbg !6896
  %_4.i255.v.us.i.i = select i1 %208, float %_0.i183.us.i.i, float %_67.i.i.us.i.i, !dbg !6896
  %209 = trunc nsw i32 %_0.i385.us.i.i to i1, !dbg !6898
  %_0.i249.us.i.i = select i1 %209, float %_71.i.i582583.us.i.i, float %_4.i255.v.us.i.i, !dbg !6898
  store float %_0.i249.us.i.i, ptr %111, align 4, !dbg !6900, !alias.scope !6766, !noalias !6767
  store i32 %_5.i257.us.i.i, ptr %108, align 4, !dbg !6901, !alias.scope !6766, !noalias !6767
  %_0.i182.us.i.i = fadd float %_0.i295.us.1.i.i, -1.000000e+00, !dbg !6902
  %_0.i181.us.i.i = fsub float %_0.i.i440.us.i.i, %_0.i295.us.i.i, !dbg !6904
  %_0.i166.us.i.i = fmul float %_0.i182.us.i.i, %_0.i181.us.i.i, !dbg !6906
  %210 = fneg float %_0.i295.us.2.i.i, !dbg !6908
  %_3.i.i425.inv.us.i.i = fcmp ogt float %_0.i166.us.i.i, %210, !dbg !6910
  %_4.i.i431.v.us.i.i = select i1 %_3.i.i425.inv.us.i.i, float %_0.i166.us.i.i, float %210, !dbg !6910
  %_3.i.i508.us.i.i = fcmp olt float %_4.i.i431.v.us.i.i, 0.000000e+00, !dbg !6913
  %211 = fcmp ule float %_0.i261.us.i.i, 0.000000e+00, !dbg !6916
  %212 = select i1 %211, i1 %_3.i.i508.us.i.i, i1 false, !dbg !6918
  %_0.i242.us.i.i = select i1 %212, float %_4.i.i431.v.us.i.i, float 0.000000e+00, !dbg !6918
  %_86.i.i.us.i.i = load float, ptr %113, align 4, !dbg !6919, !alias.scope !6766, !noalias !6767, !noundef !12
  %_3.i105.us.i.i = fcmp ule float %_0.i242.us.i.i, %_86.i.i.us.i.i, !dbg !6920
  %_87.i.i585.us.i.i = load i32, ptr %_38.i.i, align 4, !dbg !6922, !alias.scope !6795, !noalias !6796, !noundef !12
  %_88.i.i586.us.i.i = load i32, ptr %114, align 4, !dbg !6923, !alias.scope !6795, !noalias !6796, !noundef !12
  %_4.i235.us.i.i = select i1 %_3.i105.us.i.i, i32 %_88.i.i586.us.i.i, i32 %_87.i.i585.us.i.i, !dbg !6924
  %_0.i236.us.i.i = bitcast i32 %_4.i235.us.i.i to float, !dbg !6926
  %_0.i180.us.i.i = fsub float %_0.i242.us.i.i, %_86.i.i.us.i.i, !dbg !6928
  %_4.i151.us.i.i = fmul float %_0.i180.us.i.i, %_0.i236.us.i.i, !dbg !6930
  %_0.i152.us.i.i = fadd float %_86.i.i.us.i.i, %_4.i151.us.i.i, !dbg !6930
  %213 = tail call noundef float @llvm.fabs.f32(float %_0.i152.us.i.i), !dbg !6932
  %214 = fcmp uge float %213, 0x3BC79CA100000000, !dbg !6935
  %_0.i220.us.i.i = select i1 %214, float %_0.i152.us.i.i, float 0.000000e+00, !dbg !6937
  store float %_0.i220.us.i.i, ptr %113, align 4, !dbg !6938, !alias.scope !6766, !noalias !6767
  %_0.i165.us.i.i = fmul float %_0.i220.us.i.i, 0x3FC542A5A0000000, !dbg !6939
  %_3.i.i417.us.inv.i.i = fcmp ogt float %_0.i165.us.i.i, -1.260000e+02, !dbg !6942
  %_0.i.i424.us.i.i = select i1 %_3.i.i417.us.inv.i.i, float %_0.i165.us.i.i, float -1.260000e+02, !dbg !6942
  %_3.i.i500.us.inv.i.i = fcmp olt float %_0.i.i424.us.i.i, 1.270000e+02, !dbg !6946
  %_0.i.i507.us.i.i = select i1 %_3.i.i500.us.inv.i.i, float %_0.i.i424.us.i.i, float 1.270000e+02, !dbg !6946
  %215 = tail call noundef float @llvm.floor.f32(float %_0.i.i507.us.i.i), !dbg !6949
  %_0.i179.us.i.i = fsub float %_0.i.i507.us.i.i, %215, !dbg !6953
  %_0.i163.us.i.i = fmul float %_0.i179.us.i.i, 0x3F5E974FA0000000, !dbg !6955
  %_0.i146.us.i.i = fadd float %_0.i163.us.i.i, 0x3F82778560000000, !dbg !6957
  %_0.i163.us.1.i.i = fmul float %_0.i179.us.i.i, %_0.i146.us.i.i, !dbg !6955
  %_0.i146.us.1.i.i = fadd float %_0.i163.us.1.i.i, 0x3FAC91CE60000000, !dbg !6957
  %_0.i163.us.2.i.i = fmul float %_0.i179.us.i.i, %_0.i146.us.1.i.i, !dbg !6955
  %_0.i146.us.2.i.i = fadd float %_0.i163.us.2.i.i, 0x3FCEBDB560000000, !dbg !6957
  %_0.i163.us.3.i.i = fmul float %_0.i179.us.i.i, %_0.i146.us.2.i.i, !dbg !6955
  %_0.i146.us.3.i.i = fadd float %_0.i163.us.3.i.i, 0x3FE62E4BA0000000, !dbg !6957
  %_0.i161.us.i.i = fmul float %_0.i178.us.i.i, 0x3F5E974FA0000000, !dbg !6959
  %_0.i144.us.i.i = fadd float %_0.i161.us.i.i, 0x3F82778560000000, !dbg !6961
  %_0.i161.us.1.i.i = fmul float %_0.i178.us.i.i, %_0.i144.us.i.i, !dbg !6959
  %_0.i144.us.1.i.i = fadd float %_0.i161.us.1.i.i, 0x3FAC91CE60000000, !dbg !6961
  %_0.i161.us.2.i.i = fmul float %_0.i178.us.i.i, %_0.i144.us.1.i.i, !dbg !6959
  %_0.i144.us.2.i.i = fadd float %_0.i161.us.2.i.i, 0x3FCEBDB560000000, !dbg !6961
  %_0.i161.us.3.i.i = fmul float %_0.i178.us.i.i, %_0.i144.us.2.i.i, !dbg !6959
  %_0.i144.us.3.i.i = fadd float %_0.i161.us.3.i.i, 0x3FE62E4BA0000000, !dbg !6961
  %_0.i160.us.i.i = fmul float %_0.i178.us.i.i, %_0.i144.us.3.i.i, !dbg !6963
  %_0.i143.us.i.i = fadd float %_0.i160.us.i.i, 1.000000e+00, !dbg !6965
  %biased.i.us.i.i = fadd float %200, 0x4160000FE0000000, !dbg !6967
  %_4.i81.us.i.i = bitcast float %biased.i.us.i.i to i32, !dbg !6969
  %_3.i82.us.i.i = shl i32 %_4.i81.us.i.i, 23, !dbg !6971
  %_0.i83.us.i.i = bitcast i32 %_3.i82.us.i.i to float, !dbg !6972
  %_0.i159.us.i.i = fmul float %_0.i143.us.i.i, %_0.i83.us.i.i, !dbg !6974
  %_3.i91.us.i.i = fcmp une float %_0.i224.us.i.i, 0.000000e+00, !dbg !6976
  %_3.i119.us.i.i = fcmp ule float %_98.i115.i.us.i.i, 0.000000e+00, !dbg !6978
  %_0.i386568.not.us.i.i = and i1 %_3.i119.us.i.i, %_3.i91.us.i.i, !dbg !6980
  %_0.i170.us.i.i = fmul float %_0.i203.us.i.i, %_0.i159.us.i.i, !dbg !6980
  %_4.i308.v.us.i.i = select i1 %_0.i386568.not.us.i.i, float %_0.i170.us.i.i, float %_0.i203.us.i.i, !dbg !6982
  %_0.i.us.i.i = fmul float %_0.i179.us.i.i, %_0.i146.us.3.i.i, !dbg !6984
  %_0.i145.us.i.i = fadd float %_0.i.us.i.i, 1.000000e+00, !dbg !6986
  %biased.i84.us.i.i = fadd float %215, 0x4160000FE0000000, !dbg !6988
  %_4.i85.us.i.i = bitcast float %biased.i84.us.i.i to i32, !dbg !6990
  %_3.i86.us.i.i = shl i32 %_4.i85.us.i.i, 23, !dbg !6992
  %_0.i87.us.i.i = bitcast i32 %_3.i86.us.i.i to float, !dbg !6993
  %_0.i162.us.i.i = fmul float %_0.i145.us.i.i, %_0.i87.us.i.i, !dbg !6995
  %_3.i88.us.i.i = fcmp une float %_0.i220.us.i.i, 0.000000e+00, !dbg !6997
  %_98.i.i.us.i.i = load float, ptr %115, align 4, !dbg !6999, !alias.scope !6795, !noalias !6796, !noundef !12
  %_3.i103.us.i.i = fcmp ule float %_98.i.i.us.i.i, 0.000000e+00, !dbg !7000
  %_0.i383590.not.us.i.i = and i1 %_3.i103.us.i.i, %_3.i88.us.i.i, !dbg !7002
  %_0.i164.us.i.i = fmul float %_0.i201.us.i.i, %_0.i162.us.i.i, !dbg !7002
  %_4.i228.v.us.i.i = select i1 %_0.i383590.not.us.i.i, float %_0.i164.us.i.i, float %_0.i201.us.i.i, !dbg !7004
  store float %_4.i308.v.us.i.i, ptr %_97.i.us.i.i, align 4, !dbg !7006, !alias.scope !7009, !noalias !6459
  store float %_4.i228.v.us.i.i, ptr %_115.i.us.i.i, align 4, !dbg !7012, !alias.scope !7014, !noalias !6481
  %exitcond955.not.i.i = icmp eq i64 %179, %..i.i, !dbg !7017
  br i1 %exitcond955.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKb1_EB3_.exit.i, label %bb32.i.us.i.i, !dbg !7020, !llvm.loop !7025

bb8.i79.us.i.i:                                   ; preds = %bb5.i.preheader.us.i.i
  %_34.i.us.i.i = icmp samesign ugt i64 %_66.1.i.i, %_23.i77.us.i.i, !dbg !7021
  br i1 %_34.i.us.i.i, label %bb10.i80.us.i.i, label %panic5.i.i.i, !dbg !7021

bb10.i80.us.i.i:                                  ; preds = %bb8.i79.us.i.i
  %216 = getelementptr inbounds nuw float, ptr %_64.0.i.i, i64 %_15.i74.us.i.i, !dbg !7022
  %left_own.i.us.i.i = load float, ptr %216, align 4, !dbg !7022, !alias.scope !6534, !noalias !6549, !noundef !12
  %217 = getelementptr inbounds nuw float, ptr %_66.0.i.i, i64 %_23.i77.us.i.i, !dbg !7021
  %right_own.i.us.i.i = load float, ptr %217, align 4, !dbg !7021, !alias.scope !6536, !noalias !6551, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i.i, !dbg !6554

bb5.i.preheader.us.i.i:                           ; preds = %bb53.i.us.i.i
  br i1 %_31.i78.us.i.i, label %bb8.i79.us.i.i, label %panic4.i.i.i, !dbg !7022

bb14.i63.preheader.us.i.i:                        ; preds = %bb53.i.us.i.i
  br i1 %_31.i78.us.i.i, label %bb17.i.us.i.i, label %panic15.i.i.i, !dbg !6558

bb23.i.preheader.us.i.i:                          ; preds = %bb53.i.us.i.i
  br i1 %_31.i78.us.i.i, label %bb27.i60.us.i.i, label %panic28.i.i.i, !dbg !6548

bb36.i.i.i:                                       ; preds = %bb32.i.us.us.us.i.i, %bb32.i.us.i.i
  %.us-phi692.i.i = phi i64 [ %_30.i.us.i.i, %bb32.i.us.i.i ], [ %_30.i.us.us.us.i.i, %bb32.i.us.us.us.i.i ]
  %_37.i.le688.i.i = add nuw nsw i64 %.us-phi692.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi692.i.i, i64 noundef %_37.i.le688.i.i, i64 noundef %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1eeef3195352adc58f4bfdb015316fef) #24, !dbg !7026, !noalias !6414
  unreachable, !dbg !7026

bb41.i.i.i:                                       ; preds = %bb35.i.us.i.i
  %_37.i.le.i.i = add nuw nsw i64 %_30.i.us.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_30.i.us.i.i, i64 noundef %_37.i.le.i.i, i64 noundef %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9d56679ca30f2caa500ddf2e89d00cb) #24, !dbg !7027, !noalias !6414
  unreachable, !dbg !7027

bb51.i.i.i:                                       ; preds = %bb35.i.us.us.us.i.i, %bb40.i.us.i.i
  %.us-phi696.i.i = phi i64 [ %_51.i.us.i.i, %bb40.i.us.i.i ], [ %_51.i.us.us.us.i.i, %bb35.i.us.us.us.i.i ]
  %_56.i.le686.i.i = add nuw nsw i64 %.us-phi696.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi696.i.i, i64 noundef %_56.i.le686.i.i, i64 noundef %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7) #24, !dbg !7028, !noalias !6414
  unreachable, !dbg !7028

bb54.i.i.i:                                       ; preds = %bb50.i.us.i.i
  %_56.i.le.i.i = add nuw nsw i64 %_51.i.us.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_51.i.us.i.i, i64 noundef %_56.i.le.i.i, i64 noundef %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f198228fd40f4c04a48a745576c5c5ee) #24, !dbg !7029, !noalias !6414
  unreachable, !dbg !7029

panic4.i.i.i:                                     ; preds = %bb5.i.preheader.us.us.us.i.i, %bb5.i.preheader.us.i.i
  %.us-phi706.i.i = phi i64 [ %_15.i74.us.i.i, %bb5.i.preheader.us.i.i ], [ %_15.i74.us.us.us.i.i, %bb5.i.preheader.us.us.us.i.i ], !dbg !7022
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi706.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cba26bb3a04aaba20f9c249edc5e133d) #24, !dbg !7022, !noalias !7030
  unreachable, !dbg !7022

panic5.i.i.i:                                     ; preds = %bb8.i79.us.us.us.i.i, %bb8.i79.us.i.i
  %.us-phi707.i.i = phi i64 [ %_23.i77.us.i.i, %bb8.i79.us.i.i ], [ %_23.i77.us.us.us.i.i, %bb8.i79.us.us.us.i.i ], !dbg !7021
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi707.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2ad2d313de59c36d62f4a7d75017a71f) #24, !dbg !7021, !noalias !7030
  unreachable, !dbg !7021

panic15.i.i.i:                                    ; preds = %bb14.i63.preheader.us.us.us.i.i, %bb14.i63.preheader.us.i.i
  %.us-phi704.i.i = phi i64 [ %_15.i74.us.i.i, %bb14.i63.preheader.us.i.i ], [ %_15.i74.us.us.us.i.i, %bb14.i63.preheader.us.us.us.i.i ], !dbg !6558
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi704.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2e01e5f964095ee8e5aa091c7d92b88) #24, !dbg !6558, !noalias !7030
  unreachable, !dbg !6558

panic17.i.i.i:                                    ; preds = %bb17.i.us.us.us.i.i, %bb17.i.us.i.i
  %.us-phi705.i.i = phi i64 [ %_23.i77.us.i.i, %bb17.i.us.i.i ], [ %_23.i77.us.us.us.i.i, %bb17.i.us.us.us.i.i ], !dbg !6557
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi705.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_48dca199019b49040caac979cf1ddc5a) #24, !dbg !6557, !noalias !7030
  unreachable, !dbg !6557

panic28.i.i.i:                                    ; preds = %bb23.i.preheader.us.us.us.i.i, %bb23.i.preheader.us.i.i
  %.us-phi700.i.i = phi i64 [ %_15.i74.us.i.i, %bb23.i.preheader.us.i.i ], [ %_15.i74.us.us.us.i.i, %bb23.i.preheader.us.us.us.i.i ], !dbg !6548
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi700.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4b7b2eb7fa1b19ad0451b09b71313122) #24, !dbg !6548, !noalias !7030
  unreachable, !dbg !6548

panic30.i.i.i:                                    ; preds = %bb27.i60.us.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_15.i74.us.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60256fc2b51ee5bb69caa4304418caff) #24, !dbg !6550, !noalias !7030
  unreachable, !dbg !6550

panic32.i.i.i:                                    ; preds = %bb27.i60.us.us.us.i.i, %bb29.i61.us.i.i
  %.us-phi702.i.i = phi i64 [ %_23.i77.us.i.i, %bb29.i61.us.i.i ], [ %_23.i77.us.us.us.i.i, %bb27.i60.us.us.us.i.i ], !dbg !6552
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi702.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_03bcac171bcf0d517141d8cd3ac9ded0) #24, !dbg !6552, !noalias !7030
  unreachable, !dbg !6552

panic34.i.i.i:                                    ; preds = %bb31.i.us.us.us.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.i77.us.us.us.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ed84bd2438b7b7e3dfb2147bac09e923) #24, !dbg !6553, !noalias !7030
  unreachable, !dbg !6553

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKb1_EB3_.exit.i: ; preds = %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i.i
  %_82.i.i.i = trunc nuw i64 %..i.i to i32, !dbg !7031
  %_81.i.i.i = add i32 %base.i.i.i, %_82.i.i.i, !dbg !7032
  store i32 %_81.i.i.i, ptr %_57.i.i, align 4, !dbg !7034, !alias.scope !6389, !noalias !6414
  br label %bb4.i4, !dbg !7035

bb7.i5:                                           ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i, %bb4.i4
  %_27.i = trunc nuw i64 %..i.i to i32, !dbg !7036
  %218 = load i32, ptr %44, align 4, !dbg !7037, !alias.scope !6267, !noalias !6270, !noundef !12
  %219 = sub i32 %218, %_27.i, !dbg !7037
  store i32 %219, ptr %44, align 4, !dbg !7037, !alias.scope !6267, !noalias !6270
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7038), !dbg !7041
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7042), !dbg !7041
  call void @llvm.lifetime.start.p0(ptr nonnull %iter.i.i), !dbg !7044, !noalias !7049
  %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 16, !dbg !7044
  store ptr %_14.0, ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, align 8, !dbg !7044, !noalias !7049
  %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 24, !dbg !7044
  store i64 %_14.1, ptr %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i, align 8, !dbg !7044, !noalias !7049
  %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 32, !dbg !7044
  store ptr %_13.0, ptr %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i, align 8, !dbg !7044, !noalias !7049
  %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 40, !dbg !7044
  store i64 %_13.1, ptr %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i, align 8, !dbg !7044, !noalias !7049
  %_7178.not.i.i = icmp eq i64 %_14.1, 0
  %220 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %221 = getelementptr inbounds nuw i8, ptr %self, i64 104
  %222 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %223 = getelementptr inbounds nuw i8, ptr %self, i64 944
  %224 = getelementptr inbounds nuw i8, ptr %self, i64 72
  %sample_rate.i.i.i.i = load i32, ptr %224, align 8, !alias.scope !7052, !noalias !7053
  %_32.i.i.i.i = uitofp i32 %sample_rate.i.i.i.i to double
  %225 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %226 = getelementptr inbounds nuw i8, ptr %self, i64 744
  %227 = getelementptr inbounds nuw i8, ptr %self, i64 792
  %slots.i.i.i = load i64, ptr %220, align 8, !alias.scope !7052, !noalias !7053
  %_194.not.i.i.i = icmp eq i64 %slots.i.i.i, 0
  %_12.i.i.i.i = load i32, ptr %225, align 8, !alias.scope !7052, !noalias !7053
  br label %bb6.i4.i, !dbg !7054

bb6.i4.i:                                         ; preds = %bb1.backedge.i.i, %bb7.i5
  %counter.sroa.0.0.v.i.i.sroa.phi = phi ptr [ %reports.sroa.7, %bb7.i5 ], [ %reports.sroa.8, %bb1.backedge.i.i ]
  %_5.not.i.i.i.i.i = phi i1 [ false, %bb7.i5 ], [ true, %bb1.backedge.i.i ]
  %228 = phi i64 [ 0, %bb7.i5 ], [ 1, %bb1.backedge.i.i ]
  %self3.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<&mut [f32]>", ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, i64 %228, !dbg !7060
  %_14.0.i.i.i.i.i = load ptr, ptr %self3.i.i.i.i.i, align 8, !dbg !7065, !alias.scope !7069, !noalias !7076, !nonnull !12, !align !3533, !noundef !12
  %229 = getelementptr inbounds nuw i8, ptr %self3.i.i.i.i.i, i64 8, !dbg !7065
  %_14.1.i.i.i.i.i = load i64, ptr %229, align 8, !dbg !7065, !alias.scope !7069, !noalias !7076, !noundef !12
  %230 = getelementptr inbounds nuw %"kernel::GateState<f32>", ptr %self, i64 %228, !dbg !7078
  %231 = getelementptr inbounds nuw i8, ptr %230, i64 864, !dbg !7078
  %_17.i.i = load float, ptr %231, align 4, !dbg !7078, !alias.scope !7052, !noalias !7053, !noundef !12
  %_22.not.i2171.i.i = icmp eq i64 %_14.1.i.i.i.i.i, 0, !dbg !7080
  br i1 %_22.not.i2171.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !7080

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %bb6.i4.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %ok.sroa.0.0.i2074.i.i = phi i32 [ %_0.i40.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %bb6.i4.i ]
  %iter.sroa.0.0.i1973.i.i = phi ptr [ %_27.i23.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %_14.0.i.i.i.i.i, %bb6.i4.i ]
  %iter.sroa.5.0.i1872.i.i = phi i64 [ %_28.i24.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %_14.1.i.i.i.i.i, %bb6.i4.i ]
  %_27.i23.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1973.i.i, i64 4, !dbg !7084
  %_28.i24.i.i = add i64 %iter.sroa.5.0.i1872.i.i, -1, !dbg !7087
  %_0.i35.i.i = load float, ptr %iter.sroa.0.0.i1973.i.i, align 4, !dbg !7088, !alias.scope !7090, !noalias !7093, !noundef !12
  %232 = tail call noundef float @llvm.fabs.f32(float %_0.i35.i.i), !dbg !7094
  %_3.i.i.i = fcmp olt float %232, 0x46293E5940000000, !dbg !7096
  %_0.i40.i.i = select i1 %_3.i.i.i, i32 %ok.sroa.0.0.i2074.i.i, i32 0, !dbg !7098
  %_22.not.i21.i.i = icmp eq i64 %_28.i24.i.i, 0, !dbg !7080
  br i1 %_22.not.i21.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !7080

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %233 = icmp eq i32 %_0.i40.i.i, -1, !dbg !7100
  %234 = tail call float @llvm.fabs.f32(float %_17.i.i)
  %_3.i33104.i.i = fcmp olt float %234, 0x46293E5940000000
  %or.cond.i.i = and i1 %_3.i33104.i.i, %233, !dbg !7102
  br i1 %or.cond.i.i, label %bb1.backedge.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, !dbg !7102

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i: ; preds = %bb6.i4.i
  %235 = tail call noundef float @llvm.fabs.f32(float %_17.i.i), !dbg !7103
  %_3.i33.i.i = fcmp olt float %235, 0x46293E5940000000, !dbg !7106
  br i1 %_3.i33.i.i, label %bb1.backedge.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !7108

bb1.backedge.i.i:                                 ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  br i1 %_5.not.i.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9run_blockB2_.exit, label %bb6.i4.i, !dbg !7054

bb24.loopexit.i.i.i:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i
  %236 = and i32 %_0.i7.i.i.i, 1065353216, !dbg !7109
  %237 = icmp ne i32 %236, 1065353216, !dbg !7112
  %238 = zext i1 %237 to i32, !dbg !7112
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !7113

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i
  %ok.sroa.0.017.i.i.i = phi i32 [ %_0.i7.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ], [ -1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i ]
  %iter.sroa.0.016.i.i.i = phi ptr [ %_45.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ], [ %_14.0.i.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i ]
  %iter.sroa.5.015.i.i.i = phi i64 [ %_46.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ], [ %_14.1.i.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i ]
  %_45.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i.i.i, i64 4, !dbg !7114
  %_46.i.i.i = add nsw i64 %iter.sroa.5.015.i.i.i, -1, !dbg !7119
  %_0.i.i.i.i = load float, ptr %iter.sroa.0.016.i.i.i, align 4, !dbg !7120, !alias.scope !7122, !noalias !7093, !noundef !12
  %239 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i.i), !dbg !7127
  %_3.i.i.i.i = fcmp olt float %239, 0x46293E5940000000, !dbg !7129
  %_0.i7.i.i.i = select i1 %_3.i.i.i.i, i32 %ok.sroa.0.017.i.i.i, i32 0, !dbg !7131
  %_40.not.i.i.i = icmp eq i64 %_46.i.i.i, 0, !dbg !7133
  br i1 %_40.not.i.i.i, label %bb24.loopexit.i.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, !dbg !7133

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i: ; preds = %bb24.loopexit.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i
  %.pre-phi.i = phi float [ %234, %bb24.loopexit.i.i.i ], [ %235, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i ], !dbg !7134
  %ok.sroa.0.0.lcssa.i.i.i = phi i32 [ %238, %bb24.loopexit.i.i.i ], [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i ], !dbg !7137
  %_3.i.i54.i.i = fcmp uge float %.pre-phi.i, 0x46293E5940000000, !dbg !7138
  %240 = zext i1 %_3.i.i54.i.i to i32, !dbg !7140
  %failed.i.i = or i32 %ok.sroa.0.0.lcssa.i.i.i, %240, !dbg !7141
  %241 = icmp eq i32 %failed.i.i, 0
  %ring.i.i.i = getelementptr inbounds nuw %Ring, ptr %221, i64 %228
  %242 = getelementptr inbounds nuw i8, ptr %ring.i.i.i, i64 8
  %243 = getelementptr inbounds nuw [8 x float], ptr %222, i64 %228
  %values.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %243, i64 4
  %values.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %243, i64 8
  %values.sroa.7.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %243, i64 12
  %values.sroa.8.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %243, i64 16
  %values.sroa.9.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %243, i64 20
  %values.sroa.10.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %243, i64 24
  %values.sroa.11.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %243, i64 28
  %244 = getelementptr inbounds nuw %LaneTiming, ptr %223, i64 %228
  %_7.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %244, i64 4
  %_7.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %244, i64 8
  %_7.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %244, i64 12
  %245 = getelementptr inbounds nuw %Ring, ptr %self, i64 %228
  %246 = getelementptr inbounds nuw i8, ptr %245, i64 136
  %247 = getelementptr inbounds nuw %"kernel::GateCoef<f32>", ptr %226, i64 %228
  %_19.i.i.i.i = getelementptr inbounds nuw i8, ptr %247, i64 8
  %_25.i.i.i.i = getelementptr inbounds nuw i8, ptr %247, i64 4
  %248 = getelementptr inbounds nuw %"kernel::GateState<f32>", ptr %227, i64 %228
  %_17.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 72
  %_19.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 64
  %_21.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 68
  %_37.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 4
  %_39.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 8
  %_41.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 12
  %iter.sroa.0.0.ptr24.1.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 16
  %_37.1.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 20
  %_39.1.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 24
  %_41.1.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 28
  %iter.sroa.0.0.ptr24.2.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 32
  %_37.2.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 36
  %_39.2.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 40
  %_41.2.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 44
  %iter.sroa.0.0.ptr24.3.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 48
  %_37.3.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 52
  %_39.3.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 56
  %_41.3.i.i.i = getelementptr inbounds nuw i8, ptr %248, i64 60
  br i1 %241, label %bb1.backedge.i.i, label %bb30.i.i

bb30.i.i:                                         ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i
  br i1 %_7178.not.i.i, label %bb33.i.i, label %bb32.i.i, !dbg !7142

bb33.i.i:                                         ; preds = %bb21.i.i, %bb30.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7153), !dbg !7156
  br i1 %_194.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.lr.ph.i.i.i, !dbg !7159

bb7.lr.ph.i.i.i:                                  ; preds = %bb33.i.i
  %_23.1.i.i.i = load i64, ptr %242, align 8, !alias.scope !7163, !noalias !7053, !noundef !12
  br label %bb7.i.i.i, !dbg !7159

bb7.i.i.i:                                        ; preds = %bb3.i.i.i, %bb7.lr.ph.i.i.i
  %iter.sroa.0.05.i.i.i = phi i64 [ 0, %bb7.lr.ph.i.i.i ], [ %249, %bb3.i.i.i ]
  %exitcond.not.i.i.i = icmp eq i64 %iter.sroa.0.05.i.i.i, %_23.1.i.i.i, !dbg !7164
  br i1 %exitcond.not.i.i.i, label %panic1.i.i.i, label %bb3.i.i.i, !dbg !7164

bb3.i.i.i:                                        ; preds = %bb7.i.i.i
  %_23.0.i.i.i = load ptr, ptr %ring.i.i.i, align 8, !dbg !7164, !alias.scope !7163, !noalias !7053, !nonnull !12, !noundef !12
  %249 = add nuw i64 %iter.sroa.0.05.i.i.i, 1, !dbg !7165
  %250 = getelementptr inbounds nuw float, ptr %_23.0.i.i.i, i64 %iter.sroa.0.05.i.i.i, !dbg !7164
  store float 0.000000e+00, ptr %250, align 4, !dbg !7164, !noalias !7168
  %exitcond7.not.i.i.i = icmp eq i64 %249, %slots.i.i.i, !dbg !7169
  br i1 %exitcond7.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.i.i.i, !dbg !7159

panic1.i.i.i:                                     ; preds = %bb7.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.1.i.i.i, i64 noundef %_23.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_835aafef72e8508601474e7b1f4172a9) #24, !dbg !7164, !noalias !7168
  unreachable, !dbg !7164

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i: ; preds = %bb3.i.i.i, %bb33.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7171), !dbg !7174
  %values.sroa.0.0.copyload.i.i.i = load float, ptr %243, align 8, !dbg !7175, !alias.scope !7177, !noalias !7053
  %values.sroa.5.0.copyload.i.i.i = load float, ptr %values.sroa.5.0..sroa_idx.i.i.i, align 4, !dbg !7175, !alias.scope !7177, !noalias !7053
  %values.sroa.6.0.copyload.i.i.i = load float, ptr %values.sroa.6.0..sroa_idx.i.i.i, align 8, !dbg !7175, !alias.scope !7177, !noalias !7053
  %values.sroa.7.0.copyload.i.i.i = load float, ptr %values.sroa.7.0..sroa_idx.i.i.i, align 4, !dbg !7175, !alias.scope !7177, !noalias !7053
  %values.sroa.8.0.copyload.i.i.i = load float, ptr %values.sroa.8.0..sroa_idx.i.i.i, align 8, !dbg !7175, !alias.scope !7177, !noalias !7053
  %values.sroa.9.0.copyload.i.i.i = load float, ptr %values.sroa.9.0..sroa_idx.i.i.i, align 4, !dbg !7175, !alias.scope !7177, !noalias !7053
  %values.sroa.10.0.copyload.i.i.i = load float, ptr %values.sroa.10.0..sroa_idx.i.i.i, align 8, !dbg !7175, !alias.scope !7177, !noalias !7053
  %values.sroa.11.0.copyload.i.i.i = load float, ptr %values.sroa.11.0..sroa_idx.i.i.i, align 4, !dbg !7175, !alias.scope !7177, !noalias !7053
  store float %values.sroa.11.0.copyload.i.i.i, ptr %244, align 8, !dbg !7178, !alias.scope !7177, !noalias !7053
  store float %values.sroa.8.0.copyload.i.i.i, ptr %_7.sroa.4.0..sroa_idx.i.i.i, align 4, !dbg !7178, !alias.scope !7177, !noalias !7053
  store float %values.sroa.9.0.copyload.i.i.i, ptr %_7.sroa.5.0..sroa_idx.i.i.i, align 8, !dbg !7178, !alias.scope !7177, !noalias !7053
  store float %values.sroa.10.0.copyload.i.i.i, ptr %_7.sroa.6.0..sroa_idx.i.i.i, align 4, !dbg !7178, !alias.scope !7177, !noalias !7053
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7179), !dbg !7182
  %_31.i.i.i.i = fpext float %values.sroa.11.0.copyload.i.i.i to double, !dbg !7183
  %_30.i.i.i.i = fmul double %_32.i.i.i.i, %_31.i.i.i.i, !dbg !7187
  %_29.i.i.i.i = fdiv double %_30.i.i.i.i, 1.000000e+03, !dbg !7187
  %_28.i.i.i.i = fadd double %_29.i.i.i.i, 5.000000e-01, !dbg !7188
  %251 = tail call double @llvm.floor.f64(double %_28.i.i.i.i), !dbg !7189
  %or.cond.i.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %251, i32 527), !dbg !7192
  %_35.i.i.i.i = fcmp ogt double %251, 0x41EFFFFFFFE00000
  %or.cond10.i.i.i.i = or i1 %or.cond.i.i.i.i, %_35.i.i.i.i, !dbg !7192
  br i1 %or.cond10.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i, label %bb19.i.i.i.i, !dbg !7192

bb19.i.i.i.i:                                     ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i
  %_45.i.i.i.i = fpext float %values.sroa.9.0.copyload.i.i.i to double, !dbg !7193
  %_44.i.i.i.i = fmul double %_32.i.i.i.i, %_45.i.i.i.i, !dbg !7196
  %_43.i.i.i.i = fdiv double %_44.i.i.i.i, 1.000000e+03, !dbg !7196
  %_42.i.i.i.i = fadd double %_43.i.i.i.i, 5.000000e-01, !dbg !7197
  %252 = tail call double @llvm.floor.f64(double %_42.i.i.i.i), !dbg !7198
  %or.cond11.i.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %252, i32 527), !dbg !7201
  %_48.i.i.i.i = fcmp ogt double %252, 0x41EFFFFFFFE00000
  %or.cond12.i.i.i.i = or i1 %or.cond11.i.i.i.i, %_48.i.i.i.i, !dbg !7201
  br i1 %or.cond12.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i, label %bb20.3.i.i.i, !dbg !7201

bb20.3.i.i.i:                                     ; preds = %bb19.i.i.i.i
  %_36.i.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %251), !dbg !7202
  %_49.i.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %252), !dbg !7203
  %253 = tail call i32 @llvm.usub.sat.i32(i32 %_12.i.i.i.i, i32 %_36.i.i.i.i), !dbg !7204
  store i32 %253, ptr %246, align 4, !dbg !7204, !alias.scope !7205, !noalias !7053
  %_20.i.i.i.i = uitofp i32 %_49.i.i.i.i to float, !dbg !7206
  store float %_20.i.i.i.i, ptr %_19.i.i.i.i, align 4, !dbg !7207, !alias.scope !7209, !noalias !7053
; call effect_runtime::envelope::attack_release_coefficient
  %_23.i.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.8.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i.i) #23, !dbg !7212, !noalias !7213
  store float %_23.i.i.i.i, ptr %247, align 4, !dbg !7214, !alias.scope !7216, !noalias !7053
; call effect_runtime::envelope::attack_release_coefficient
  %_26.i.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.10.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i.i) #23, !dbg !7219, !noalias !7213
  store float %_26.i.i.i.i, ptr %_25.i.i.i.i, align 4, !dbg !7220, !alias.scope !7222, !noalias !7053
  store float 0.000000e+00, ptr %_17.i.i.i, align 4, !dbg !7225, !alias.scope !7227, !noalias !7053
  store float 1.000000e+00, ptr %_19.i.i.i, align 4, !dbg !7230, !alias.scope !7232, !noalias !7053
  store float %_20.i.i.i.i, ptr %_21.i.i.i, align 4, !dbg !7235, !alias.scope !7237, !noalias !7053
  store float %values.sroa.0.0.copyload.i.i.i, ptr %248, align 4, !dbg !7240, !alias.scope !7242, !noalias !7053
  store float %values.sroa.0.0.copyload.i.i.i, ptr %_37.i.i.i, align 4, !dbg !7245, !alias.scope !7247, !noalias !7053
  store float 0.000000e+00, ptr %_39.i.i.i, align 4, !dbg !7250, !alias.scope !7252, !noalias !7053
  store float 0.000000e+00, ptr %_41.i.i.i, align 4, !dbg !7255, !alias.scope !7257, !noalias !7053
  store float %values.sroa.5.0.copyload.i.i.i, ptr %iter.sroa.0.0.ptr24.1.i.i.i, align 4, !dbg !7240, !alias.scope !7242, !noalias !7053
  store float %values.sroa.5.0.copyload.i.i.i, ptr %_37.1.i.i.i, align 4, !dbg !7245, !alias.scope !7247, !noalias !7053
  store float 0.000000e+00, ptr %_39.1.i.i.i, align 4, !dbg !7250, !alias.scope !7252, !noalias !7053
  store float 0.000000e+00, ptr %_41.1.i.i.i, align 4, !dbg !7255, !alias.scope !7257, !noalias !7053
  store float %values.sroa.6.0.copyload.i.i.i, ptr %iter.sroa.0.0.ptr24.2.i.i.i, align 4, !dbg !7240, !alias.scope !7242, !noalias !7053
  store float %values.sroa.6.0.copyload.i.i.i, ptr %_37.2.i.i.i, align 4, !dbg !7245, !alias.scope !7247, !noalias !7053
  store float 0.000000e+00, ptr %_39.2.i.i.i, align 4, !dbg !7250, !alias.scope !7252, !noalias !7053
  store float 0.000000e+00, ptr %_41.2.i.i.i, align 4, !dbg !7255, !alias.scope !7257, !noalias !7053
  store float %values.sroa.7.0.copyload.i.i.i, ptr %iter.sroa.0.0.ptr24.3.i.i.i, align 4, !dbg !7240, !alias.scope !7242, !noalias !7053
  store float %values.sroa.7.0.copyload.i.i.i, ptr %_37.3.i.i.i, align 4, !dbg !7245, !alias.scope !7247, !noalias !7053
  store float 0.000000e+00, ptr %_39.3.i.i.i, align 4, !dbg !7250, !alias.scope !7252, !noalias !7053
  store float 0.000000e+00, ptr %_41.3.i.i.i, align 4, !dbg !7255, !alias.scope !7257, !noalias !7053
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i, !dbg !7260

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i: ; preds = %bb20.3.i.i.i, %bb19.i.i.i.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i
  %_46.i.i = load i64, ptr %counter.sroa.0.0.v.i.i.sroa.phi, align 8, !dbg !7261, !alias.scope !7264, !noalias !7265, !noundef !12
  %254 = tail call i64 @llvm.uadd.sat.i64(i64 %_46.i.i, i64 %_14.1), !dbg !7266
  store i64 %254, ptr %counter.sroa.0.0.v.i.i.sroa.phi, align 8, !dbg !7269, !alias.scope !7264, !noalias !7265
  br label %bb1.backedge.i.i, !dbg !7054

bb32.i.i:                                         ; preds = %bb30.i.i, %bb21.i.i
  %iter2.sroa.0.079.i.i = phi i64 [ %255, %bb21.i.i ], [ 0, %bb30.i.i ]
  %exitcond.not.i5.i = icmp eq i64 %iter2.sroa.0.079.i.i, %_14.1.i.i.i.i.i, !dbg !7270
  br i1 %exitcond.not.i5.i, label %panic4.i.i, label %bb21.i.i, !dbg !7270

bb21.i.i:                                         ; preds = %bb32.i.i
  %255 = add nuw i64 %iter2.sroa.0.079.i.i, 1, !dbg !7272
  %256 = getelementptr inbounds nuw float, ptr %_14.0.i.i.i.i.i, i64 %iter2.sroa.0.079.i.i, !dbg !7270
  store float 0.000000e+00, ptr %256, align 4, !dbg !7270, !noalias !7093
  %exitcond99.not.i.i = icmp eq i64 %255, %_14.1, !dbg !7280
  br i1 %exitcond99.not.i.i, label %bb33.i.i, label %bb32.i.i, !dbg !7142

panic4.i.i:                                       ; preds = %bb32.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_14.1.i.i.i.i.i, i64 noundef %_14.1.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6797264598a169e4722ae66c7bc497b8) #24, !dbg !7270, !noalias !7093
  unreachable, !dbg !7270

bb20.i:                                           ; preds = %bb4.i4
  %_60.i = icmp samesign ugt i64 %..i.i, %_13.1, !dbg !7284
  br i1 %_60.i, label %bb26.i, label %bb27.i, !dbg !7284, !prof !180

bb27.i:                                           ; preds = %bb20.i
  %_59.i = getelementptr inbounds nuw float, ptr %_14.0, i64 %..i.i, !dbg !7292
  %_55.i = sub i64 %_14.1, %..i.i, !dbg !7300
  %_63.i = sub nuw nsw i64 %_13.1, %..i.i, !dbg !7301
  %_67.i = getelementptr inbounds nuw float, ptr %_13.0, i64 %..i.i, !dbg !7302
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7307), !dbg !7310
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7311), !dbg !7310
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7313), !dbg !7310
  %_10.i13.i = getelementptr inbounds nuw i8, ptr %self, i64 792, !dbg !7315
  %data.i.i.i14.i = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !7318
  %_15.i15.i = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !7323
  %data.i.i474.i.i = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !7325
  %257 = getelementptr inbounds nuw i8, ptr %self, i64 68, !dbg !7330
  %_29.i16.i = load i32, ptr %257, align 4, !dbg !7330, !range !1335, !alias.scope !7334, !noalias !7335, !noundef !12
  %_31.i17.i = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !7337
  %_33.i18.i = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !7338
  %258 = icmp eq i32 %_29.i16.i, 1, !dbg !7339
  %.val.i.i.i19.i = load i32, ptr %_31.i17.i, align 4, !dbg !7339, !alias.scope !7334, !noalias !7335
  %.val1.i.i.i20.i = load i32, ptr %_33.i18.i, align 4, !dbg !7339, !alias.scope !7334, !noalias !7335
  %_0.i.i.not.i.i.i21.i = icmp eq i32 %.val.i.i.i19.i, %.val1.i.i.i20.i, !dbg !7339
  %spec.select.i22.i = select i1 %_0.i.i.not.i.i.i21.i, i8 1, i8 2, !dbg !7339
  %_0.sroa.0.0.i479.i.i = select i1 %258, i8 0, i8 %spec.select.i22.i, !dbg !7339
  %259 = getelementptr inbounds nuw i8, ptr %self, i64 744, !dbg !7341
  %_38.i23.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !7343
  %_64.0.i24.i = load ptr, ptr %_15.i15.i, align 8, !dbg !7344, !alias.scope !7334, !noalias !7335, !nonnull !12, !noundef !12
  %260 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !7344
  %_64.1.i25.i = load i64, ptr %260, align 8, !dbg !7344, !alias.scope !7334, !noalias !7335, !noundef !12
  %_66.0.i26.i = load ptr, ptr %data.i.i474.i.i, align 8, !dbg !7345, !alias.scope !7334, !noalias !7335, !nonnull !12, !noundef !12
  %261 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !7345
  %_66.1.i27.i = load i64, ptr %261, align 8, !dbg !7345, !alias.scope !7334, !noalias !7335, !noundef !12
  %_57.i28.i = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !7346
  %262 = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !7347
  %_58.i29.i = load i32, ptr %262, align 4, !dbg !7347, !alias.scope !7334, !noalias !7335, !noundef !12
  %263 = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !7348
  %_59.i30.i = load i32, ptr %263, align 8, !dbg !7348, !alias.scope !7334, !noalias !7335, !noundef !12
  %base.i.i35.i = load i32, ptr %_57.i28.i, align 4, !dbg !7349, !alias.scope !7334, !noalias !7359, !noundef !12
  %threshold.i22.i.i.i = load float, ptr %_10.i13.i, align 4, !alias.scope !7334, !noalias !7335
  %264 = getelementptr inbounds nuw i8, ptr %self, i64 808
  %ratio.i23.i.i.i = load float, ptr %264, align 4, !alias.scope !7334, !noalias !7335
  %265 = getelementptr inbounds nuw i8, ptr %self, i64 824
  %range.i24.i.i.i = load float, ptr %265, align 4, !alias.scope !7334, !noalias !7335
  %266 = getelementptr inbounds nuw i8, ptr %self, i64 840
  %hysteresis.i25.i.i.i = load float, ptr %266, align 4, !alias.scope !7334, !noalias !7335
  %267 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %_37.i28.i.i.i = load float, ptr %267, align 4, !alias.scope !7334, !noalias !7335
  %_3.i125.i.i = fcmp ule float %_37.i28.i.i.i, 0.000000e+00
  %268 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %_41.i32.i.i.i = load float, ptr %268, align 4, !alias.scope !7334, !noalias !7335
  %_3.i123.i.i = fcmp ule float %_41.i32.i.i.i, 0.000000e+00
  %269 = getelementptr inbounds nuw i8, ptr %self, i64 856
  %_0.i179.i.i = fsub float %threshold.i22.i.i.i, %hysteresis.i25.i.i.i
  %270 = getelementptr inbounds nuw i8, ptr %self, i64 860
  %271 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %_71.i65.i489490.i.i = load float, ptr %271, align 4, !alias.scope !7334, !noalias !7335
  %_0.i177.i.i = fadd float %ratio.i23.i.i.i, -1.000000e+00
  %272 = fneg float %range.i24.i.i.i
  %273 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %_87.i80.i492.i.i = load i32, ptr %259, align 4, !alias.scope !7334, !noalias !7335
  %274 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %_88.i81.i493.i.i = load i32, ptr %274, align 4, !alias.scope !7334, !noalias !7335
  %275 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %_98.i94.i.i.i = load float, ptr %275, align 4, !alias.scope !7334, !noalias !7335
  %_3.i113.i.i = fcmp ule float %_98.i94.i.i.i, 0.000000e+00
  %threshold.i.i.i.i = load float, ptr %data.i.i.i14.i, align 4, !alias.scope !7334, !noalias !7335
  %276 = getelementptr inbounds nuw i8, ptr %self, i64 884
  %ratio.i.i.i.i = load float, ptr %276, align 4, !alias.scope !7334, !noalias !7335
  %277 = getelementptr inbounds nuw i8, ptr %self, i64 900
  %range.i.i.i.i = load float, ptr %277, align 4, !alias.scope !7334, !noalias !7335
  %278 = getelementptr inbounds nuw i8, ptr %self, i64 916
  %hysteresis.i.i.i.i = load float, ptr %278, align 4, !alias.scope !7334, !noalias !7335
  %279 = getelementptr inbounds nuw i8, ptr %self, i64 784
  %_37.i.i.i.i = load float, ptr %279, align 4, !alias.scope !7334, !noalias !7335
  %_3.i111.i.i = fcmp ule float %_37.i.i.i.i, 0.000000e+00
  %280 = getelementptr inbounds nuw i8, ptr %self, i64 788
  %_41.i.i.i.i = load float, ptr %280, align 4, !alias.scope !7334, !noalias !7335
  %_3.i109.i.i = fcmp ule float %_41.i.i.i.i, 0.000000e+00
  %281 = getelementptr inbounds nuw i8, ptr %self, i64 932
  %_0.i174.i.i = fsub float %threshold.i.i.i.i, %hysteresis.i.i.i.i
  %282 = getelementptr inbounds nuw i8, ptr %self, i64 936
  %283 = getelementptr inbounds nuw i8, ptr %self, i64 776
  %_71.i.i506507.i.i = load float, ptr %283, align 4, !alias.scope !7334, !noalias !7335
  %_0.i172.i.i = fadd float %ratio.i.i.i.i, -1.000000e+00
  %284 = fneg float %range.i.i.i.i
  %285 = getelementptr inbounds nuw i8, ptr %self, i64 940
  %_87.i.i509.i.i = load i32, ptr %_38.i23.i, align 4, !alias.scope !7334, !noalias !7335
  %286 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %_88.i.i510.i.i = load i32, ptr %286, align 4, !alias.scope !7334, !noalias !7335
  %287 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %_98.i.i.i.i = load float, ptr %287, align 4, !alias.scope !7334, !noalias !7335
  %_3.i99.i.i = fcmp ule float %_98.i.i.i.i, 0.000000e+00
  %.promoted.i.i = load float, ptr %270, align 4, !alias.scope !7334, !noalias !7335
  %.promoted615.i.i = load float, ptr %273, align 4, !alias.scope !7334, !noalias !7335
  %.promoted617.i.i = load float, ptr %282, align 4, !alias.scope !7334, !noalias !7335
  %.promoted619.i.i = load float, ptr %285, align 4, !alias.scope !7334, !noalias !7335
  %injected.cond.not.i.i = icmp samesign ugt i64 %_14.1, %_13.1
  br i1 %injected.cond.not.i.i, label %bb32.i.i.i, label %bb30.i.lr.ph.split.us.i.i

bb30.i.lr.ph.split.us.i.i:                        ; preds = %bb27.i
  %injected.cond638.not.i.i = icmp ugt i64 %_64.1.i25.i, %_66.1.i27.i
  br i1 %injected.cond638.not.i.i, label %bb32.i.us.i186.i, label %bb32.i.us.us.us.i42.i

bb32.i.us.us.us.i42.i:                            ; preds = %bb30.i.lr.ph.split.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i
  %_86.i.i620.us.us.us.i.i = phi float [ %_0.i208.us.us.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i ], [ %.promoted619.i.i, %bb30.i.lr.ph.split.us.i.i ]
  %_67.i.i618.us.us.us.i.i = phi float [ %_0.i237.us.us.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i ], [ %.promoted617.i.i, %bb30.i.lr.ph.split.us.i.i ]
  %_86.i78.i616.us.us.us.i.i = phi float [ %_0.i212.us.us.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i ], [ %.promoted615.i.i, %bb30.i.lr.ph.split.us.i.i ]
  %_67.i59.i614.us.us.us.i.i = phi float [ %_0.i290.us.us.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i ], [ %.promoted.i.i, %bb30.i.lr.ph.split.us.i.i ]
  %iter.sroa.0.0.i613.us.us.us.i.i = phi i64 [ %288, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i ], [ 0, %bb30.i.lr.ph.split.us.i.i ]
  %288 = add nuw nsw i64 %iter.sroa.0.0.i613.us.us.us.i.i, 1, !dbg !7362
  %_28.i.us.us.us.i37.i = trunc i64 %iter.sroa.0.0.i613.us.us.us.i.i to i32, !dbg !7376
  %now.i.us.us.us.i38.i = add i32 %base.i.i35.i, %_28.i.us.us.us.i37.i, !dbg !7379
  %_31.i.us.us.us.i39.i = and i32 %now.i.us.us.us.i38.i, %_58.i29.i, !dbg !7382
  %_30.i.us.us.us.i40.i = zext i32 %_31.i.us.us.us.i39.i to i64, !dbg !7384
  %_97.i.us.us.us.i43.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %iter.sroa.0.0.i613.us.us.us.i.i, !dbg !7385
  %_98.not.not.i.us.us.us.i44.i = icmp ugt i64 %_64.1.i25.i, %_30.i.us.us.us.i40.i, !dbg !7394
  br i1 %_98.not.not.i.us.us.us.i44.i, label %bb35.i.us.us.us.i46.i, label %bb36.i.i45.i, !dbg !7394, !prof !2709

bb35.i.us.us.us.i46.i:                            ; preds = %bb32.i.us.us.us.i42.i
  %_0.i195.us.us.us.i.i = load float, ptr %_97.i.us.us.us.i43.i, align 4, !dbg !7399, !alias.scope !7401, !noalias !7404, !noundef !12
  %_107.i.us.us.us.i47.i = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_30.i.us.us.us.i40.i, !dbg !7405
  store float %_0.i195.us.us.us.i.i, ptr %_107.i.us.us.us.i47.i, align 4, !dbg !7409, !alias.scope !7411, !noalias !7359
  %_115.i.us.us.us.i48.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %iter.sroa.0.0.i613.us.us.us.i.i, !dbg !7414
  %_0.i193.us.us.us.i.i = load float, ptr %_115.i.us.us.us.i48.i, align 4, !dbg !7421, !alias.scope !7423, !noalias !7426, !noundef !12
  %_123.i.us.us.us.i49.i = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_30.i.us.us.us.i40.i, !dbg !7427
  store float %_0.i193.us.us.us.i.i, ptr %_123.i.us.us.us.i49.i, align 4, !dbg !7434, !alias.scope !7436, !noalias !7359
  %_53.i.us.us.us.i50.i = sub i32 %now.i.us.us.us.i38.i, %_59.i30.i, !dbg !7439
  %_52.i.us.us.us.i51.i = and i32 %_53.i.us.us.us.i50.i, %_58.i29.i, !dbg !7442
  %_51.i.us.us.us.i52.i = zext i32 %_52.i.us.us.us.i51.i to i64, !dbg !7443
  %_156.not.not.i.us.us.us.i53.i = icmp ugt i64 %_64.1.i25.i, %_51.i.us.us.us.i52.i, !dbg !7444
  br i1 %_156.not.not.i.us.us.us.i53.i, label %bb50.i.us.us.us.i55.i, label %bb51.i.i54.i, !dbg !7444, !prof !2709

bb50.i.us.us.us.i55.i:                            ; preds = %bb35.i.us.us.us.i46.i
  %_163.i.us.us.us.i56.i = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_51.i.us.us.us.i52.i, !dbg !7449
  %_0.i191.us.us.us.i57.i = load float, ptr %_163.i.us.us.us.i56.i, align 4, !dbg !7453, !alias.scope !7455, !noalias !7359, !noundef !12
  %_169.i.us.us.us.i58.i = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_51.i.us.us.us.i52.i, !dbg !7458
  %_0.i189.us.us.us.i59.i = load float, ptr %_169.i.us.us.us.i58.i, align 4, !dbg !7466, !alias.scope !7468, !noalias !7359, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7471), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7477), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7479), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7481), !dbg !7474
  %_18.i72.us.us.us.i60.i = load i32, ptr %_31.i17.i, align 4, !dbg !7483, !alias.scope !7485, !noalias !7486, !noundef !12
  %_17.i.us.us.us.i61.i = sub i32 %now.i.us.us.us.i38.i, %_18.i72.us.us.us.i60.i, !dbg !7488
  %_16.i73.us.us.us.i62.i = and i32 %_17.i.us.us.us.i61.i, %_58.i29.i, !dbg !7483
  %_15.i74.us.us.us.i63.i = zext i32 %_16.i73.us.us.us.i62.i to i64, !dbg !7483
  %_26.i.us.us.us.i64.i = load i32, ptr %_33.i18.i, align 4, !dbg !7483, !alias.scope !7490, !noalias !7491, !noundef !12
  %_25.i.us.us.us.i65.i = sub i32 %now.i.us.us.us.i38.i, %_26.i.us.us.us.i64.i, !dbg !7488
  %_24.i.us.us.us.i66.i = and i32 %_25.i.us.us.us.i65.i, %_58.i29.i, !dbg !7483
  %_23.i77.us.us.us.i67.i = zext i32 %_24.i.us.us.us.i66.i to i64, !dbg !7483
  %_31.i78.us.us.us.i68.i = icmp samesign ugt i64 %_64.1.i25.i, %_15.i74.us.us.us.i63.i, !dbg !7483
  switch i8 %_0.sroa.0.0.i479.i.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.us.us.i170.i
    i8 1, label %bb14.i63.preheader.us.us.us.i162.i
    i8 2, label %bb23.i.preheader.us.us.us.i69.i
  ], !dbg !7492

bb27.i60.us.us.us.i71.i:                          ; preds = %bb23.i.preheader.us.us.us.i69.i
  %289 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.us.us.us.i63.i, !dbg !7493
  %_79.i.us.us.us.i72.i = load float, ptr %289, align 4, !dbg !7493, !alias.scope !7479, !noalias !7494, !noundef !12
  %290 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_15.i74.us.us.us.i63.i, !dbg !7495
  %_83.i.us.us.us.i76.i = load float, ptr %290, align 4, !dbg !7495, !alias.scope !7481, !noalias !7496, !noundef !12
  %_87.i.us.us.us.i77.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.us.us.us.i67.i, !dbg !7497
  br i1 %_87.i.us.us.us.i77.i, label %bb31.i.us.us.us.i79.i, label %panic32.i.i78.i, !dbg !7497

bb31.i.us.us.us.i79.i:                            ; preds = %bb27.i60.us.us.us.i71.i
  %_89.i.us.us.us.i80.i = icmp samesign ugt i64 %_64.1.i25.i, %_23.i77.us.us.us.i67.i, !dbg !7498
  br i1 %_89.i.us.us.us.i80.i, label %bb33.i62.us.us.us.i82.i, label %panic34.i.i81.i, !dbg !7498

bb33.i62.us.us.us.i82.i:                          ; preds = %bb31.i.us.us.us.i79.i
  %291 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.us.us.us.i67.i, !dbg !7497
  %_86.i.us.us.us.i83.i = load float, ptr %291, align 4, !dbg !7497, !alias.scope !7481, !noalias !7496, !noundef !12
  %292 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_23.i77.us.us.us.i67.i, !dbg !7498
  %_88.i.us.us.us.i84.i = load float, ptr %292, align 4, !dbg !7498, !alias.scope !7479, !noalias !7494, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i, !dbg !7499

bb17.i.us.us.us.i164.i:                           ; preds = %bb14.i63.preheader.us.us.us.i162.i
  %_59.i.us.us.us.i165.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.us.us.us.i67.i, !dbg !7502
  br i1 %_59.i.us.us.us.i165.i, label %bb19.i.us.us.us.i167.i, label %panic17.i.i166.i, !dbg !7502

bb19.i.us.us.us.i167.i:                           ; preds = %bb17.i.us.us.us.i164.i
  %293 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.us.us.us.i63.i, !dbg !7503
  %left_own16.i.us.us.us.i168.i = load float, ptr %293, align 4, !dbg !7503, !alias.scope !7479, !noalias !7494, !noundef !12
  %294 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.us.us.us.i67.i, !dbg !7502
  %right_own18.i.us.us.us.i169.i = load float, ptr %294, align 4, !dbg !7502, !alias.scope !7481, !noalias !7496, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i, !dbg !7499

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i: ; preds = %bb10.i80.us.us.us.i175.i, %bb19.i.us.us.us.i167.i, %bb33.i62.us.us.us.i82.i
  %taps.i.sroa.41890.2.i.i = phi float [ %right_own.i.us.us.us.i177.i, %bb10.i80.us.us.us.i175.i ], [ %left_own16.i.us.us.us.i168.i, %bb19.i.us.us.us.i167.i ], [ %_88.i.us.us.us.i84.i, %bb33.i62.us.us.us.i82.i ], !dbg !7483
  %taps.i.sroa.28889.2.i.i = phi float [ %right_own.i.us.us.us.i177.i, %bb10.i80.us.us.us.i175.i ], [ %right_own18.i.us.us.us.i169.i, %bb19.i.us.us.us.i167.i ], [ %_86.i.us.us.us.i83.i, %bb33.i62.us.us.us.i82.i ], !dbg !7483
  %taps.i.sroa.15888.2.i.i = phi float [ %left_own.i.us.us.us.i176.i, %bb10.i80.us.us.us.i175.i ], [ %right_own18.i.us.us.us.i169.i, %bb19.i.us.us.us.i167.i ], [ %_83.i.us.us.us.i76.i, %bb33.i62.us.us.us.i82.i ], !dbg !7483
  %taps.i.sroa.0.2.i86.i = phi float [ %left_own.i.us.us.us.i176.i, %bb10.i80.us.us.us.i175.i ], [ %left_own16.i.us.us.us.i168.i, %bb19.i.us.us.us.i167.i ], [ %_79.i.us.us.us.i72.i, %bb33.i62.us.us.us.i82.i ], !dbg !7483
  %295 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.2.i86.i), !dbg !7504
  %296 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.15888.2.i.i), !dbg !7507
  %_3.i.i417.us.us.us.i.i = fcmp ule float %295, %296, !dbg !7509
  %_6.i.i419.us.us.us.i.i = bitcast float %295 to i32, !dbg !7512
  %_8.i.i421.us.us.us.i.i = bitcast float %296 to i32, !dbg !7515
  %_4.i.i424.us.us.us.i.i = select i1 %_3.i.i417.us.us.us.i.i, i32 %_8.i.i421.us.us.us.i.i, i32 %_6.i.i419.us.us.us.i.i, !dbg !7517
  %_4.i315.us.us.us.i87.i = select i1 %_3.i125.i.i, i32 %_6.i.i419.us.us.us.i.i, i32 %_4.i.i424.us.us.us.i.i, !dbg !7518
  %_0.i165.us.us.us.i88.i = fmul float %295, 5.000000e-01, !dbg !7520
  %_0.i164.us.us.us.i89.i = fmul float %296, 5.000000e-01, !dbg !7522
  %_0.i140.us.us.us.i90.i = fadd float %_0.i164.us.us.us.i89.i, %_0.i165.us.us.us.i88.i, !dbg !7524
  %_6.i303.us.us.us.i.i = bitcast float %_0.i140.us.us.us.i90.i to i32, !dbg !7526
  %_4.i308.us.us.us.i.i = select i1 %_3.i123.i.i, i32 %_4.i315.us.us.us.i87.i, i32 %_6.i303.us.us.us.i.i, !dbg !7529
  %_0.i309.us.us.us.i.i = bitcast i32 %_4.i308.us.us.us.i.i to float, !dbg !7530
  %_3.i.i409.us.us.us.i.i = fcmp ule float %_0.i309.us.us.us.i.i, 0x3E45798EE0000000, !dbg !7532
  %_4.i.i415.us.us.us.i.i = select i1 %_3.i.i409.us.us.us.i.i, i32 841731191, i32 %_4.i308.us.us.us.i.i, !dbg !7535
  %_0.i.i416.us.us.us.i91.i = bitcast i32 %_4.i.i415.us.us.us.i.i to float, !dbg !7537
  %_3.i.i.us.us.us.i92.i = fcmp ule float %_0.i.i416.us.us.us.i91.i, 0x3810000000000000, !dbg !7539
  %_4.i.i.us.us.us.i93.i = select i1 %_3.i.i.us.us.us.i92.i, i32 8388608, i32 %_4.i.i415.us.us.us.i.i, !dbg !7544
  %_5.i196.us.us.us.i.i = and i32 %_4.i.i.us.us.us.i93.i, 8388607, !dbg !7546
  %_4.i197.us.us.us.i.i = or disjoint i32 %_5.i196.us.us.us.i.i, 1065353216, !dbg !7546
  %significand.i.us.us.us.i94.i = bitcast i32 %_4.i197.us.us.us.i.i to float, !dbg !7548
  %_0.i166.us.us.us.i95.i = fadd float %significand.i.us.us.us.i94.i, -1.000000e+00, !dbg !7550
  %_0.i146.us.us.us.i96.i = fmul float %_0.i166.us.us.us.i95.i, 0x3F9B17A960000000, !dbg !7552
  %297 = fsub float 0x3FBF9A8440000000, %_0.i146.us.us.us.i96.i, !dbg !7554
  %_0.i146.us.us.us.1.i97.i = fmul float %_0.i166.us.us.us.i95.i, %297, !dbg !7552
  %_0.i132.us.us.us.1.i.i = fadd float %_0.i146.us.us.us.1.i97.i, 0xBFD1E3F400000000, !dbg !7554
  %_0.i146.us.us.us.2.i98.i = fmul float %_0.i166.us.us.us.i95.i, %_0.i132.us.us.us.1.i.i, !dbg !7552
  %_0.i132.us.us.us.2.i.i = fadd float %_0.i146.us.us.us.2.i98.i, 0x3FDD544F20000000, !dbg !7554
  %_0.i146.us.us.us.3.i99.i = fmul float %_0.i166.us.us.us.i95.i, %_0.i132.us.us.us.2.i.i, !dbg !7552
  %_0.i132.us.us.us.3.i.i = fadd float %_0.i146.us.us.us.3.i99.i, 0xBFE6FC2A60000000, !dbg !7554
  %_0.i146.us.us.us.4.i.i = fmul float %_0.i166.us.us.us.i95.i, %_0.i132.us.us.us.3.i.i, !dbg !7552
  %_0.i132.us.us.us.4.i.i = fadd float %_0.i146.us.us.us.4.i.i, 0x3FF714B2A0000000, !dbg !7554
  %_9.i.us.us.us.i100.i = lshr i32 %_4.i.i.us.us.us.i93.i, 23, !dbg !7556
  %_8.i198.us.us.us.i.i = or disjoint i32 %_9.i.us.us.us.i100.i, 1258291200, !dbg !7556
  %_7.i.us.us.us.i101.i = bitcast i32 %_8.i198.us.us.us.i.i to float, !dbg !7557
  %exponent.i.us.us.us.i102.i = fadd float %_7.i.us.us.us.i101.i, 0xC160000FE0000000, !dbg !7559
  %_0.i145.us.us.us.i103.i = fmul float %_0.i166.us.us.us.i95.i, %_0.i132.us.us.us.4.i.i, !dbg !7560
  %_0.i131.us.us.us.i.i = fadd float %exponent.i.us.us.us.i102.i, %_0.i145.us.us.us.i103.i, !dbg !7562
  %_0.i163.us.us.us.i104.i = fmul float %_0.i131.us.us.us.i.i, 0x4018151820000000, !dbg !7564
  %_3.i.i466.us.us.us.inv.i.i = fcmp olt float %_0.i163.us.us.us.i104.i, 2.400000e+01, !dbg !7566
  %_0.i.i473.us.us.us.i.i = select i1 %_3.i.i466.us.us.us.inv.i.i, float %_0.i163.us.us.us.i104.i, float 2.400000e+01, !dbg !7566
  %_3.i.i401.us.us.us.inv.i.i = fcmp ogt float %_0.i.i473.us.us.us.i.i, -1.600000e+02, !dbg !7569
  %_0.i.i408.us.us.us.i.i = select i1 %_3.i.i401.us.us.us.inv.i.i, float %_0.i.i473.us.us.us.i.i, float -1.600000e+02, !dbg !7569
  %_55.i49.i.us.us.us.i.i = load float, ptr %269, align 4, !dbg !7572, !alias.scope !7573, !noalias !7576, !noundef !12
  %_3.i121.us.us.us.i105.i = fcmp ule float %_55.i49.i.us.us.us.i.i, 0.000000e+00, !dbg !7578
  %_3.i97.us.us.us.i106.i = fcmp oge float %_0.i.i408.us.us.us.i.i, %threshold.i22.i.i.i, !dbg !7580
  %_3.i95.us.us.us.i107.i = fcmp oge float %_0.i.i408.us.us.us.i.i, %_0.i179.i.i, !dbg !7582
  %..i96.us.us.us.i108.i = sext i1 %_3.i95.us.us.us.i107.i to i32, !dbg !7584
  %_0.i329.us.us.us.i109.i = sext i1 %_3.i97.us.us.us.i106.i to i32, !dbg !7586
  %_0.i322.us.us.us.i110.i = select i1 %_3.i121.us.us.us.i105.i, i32 %_0.i329.us.us.us.i109.i, i32 %..i96.us.us.us.i108.i, !dbg !7586
  %_0.i333.us.us.us.i.i = xor i32 %..i96.us.us.us.i108.i, -1, !dbg !7588
  %_3.i119.us.us.us.i111.i = fcmp ogt float %_67.i59.i614.us.us.us.i.i, 0.000000e+00, !dbg !7590
  %_0.i328.us.us.us.i.i = select i1 %_3.i119.us.us.us.i111.i, i32 %_0.i333.us.us.us.i.i, i32 0, !dbg !7592
  %_0.i327.us.us.us.i.i = select i1 %_3.i121.us.us.us.i105.i, i32 0, i32 %_0.i328.us.us.us.i.i, !dbg !7594
  %_0.i321.us.us.us.i.i = or i32 %_0.i327.us.us.us.i.i, %_0.i322.us.us.us.i110.i, !dbg !7596
  %_5.i298.us.us.us.i.i = and i32 %_0.i321.us.us.us.i.i, 1065353216, !dbg !7598
  %_0.i302.us.us.us.i.i = bitcast i32 %_5.i298.us.us.us.i.i to float, !dbg !7600
  %_0.i178.us.us.us.i112.i = fadd float %_67.i59.i614.us.us.us.i.i, -1.000000e+00, !dbg !7602
  %298 = trunc nsw i32 %_0.i327.us.us.us.i.i to i1, !dbg !7604
  %_4.i296.v.us.us.us.i.i = select i1 %298, float %_0.i178.us.us.us.i112.i, float %_67.i59.i614.us.us.us.i.i, !dbg !7604
  %299 = trunc nsw i32 %_0.i322.us.us.us.i110.i to i1, !dbg !7606
  %_0.i290.us.us.us.i.i = select i1 %299, float %_71.i65.i489490.i.i, float %_4.i296.v.us.us.us.i.i, !dbg !7606
  store float %_0.i290.us.us.us.i.i, ptr %270, align 4, !dbg !7608, !alias.scope !7573, !noalias !7576
  store i32 %_5.i298.us.us.us.i.i, ptr %269, align 4, !dbg !7609, !alias.scope !7573, !noalias !7576
  %_0.i176.us.us.us.i113.i = fsub float %_0.i.i408.us.us.us.i.i, %threshold.i22.i.i.i, !dbg !7610
  %_0.i162.us.us.us.i114.i = fmul float %_0.i177.i.i, %_0.i176.us.us.us.i113.i, !dbg !7612
  %_3.i.i392.inv.us.us.us.i.i = fcmp ogt float %_0.i162.us.us.us.i114.i, %272, !dbg !7614
  %_4.i.i399.v.us.us.us.i.i = select i1 %_3.i.i392.inv.us.us.us.i.i, float %_0.i162.us.us.us.i114.i, float %272, !dbg !7614
  %_3.i.i458.us.us.us.i.i = fcmp olt float %_4.i.i399.v.us.us.us.i.i, 0.000000e+00, !dbg !7617
  %300 = fcmp ule float %_0.i302.us.us.us.i.i, 0.000000e+00, !dbg !7620
  %301 = select i1 %300, i1 %_3.i.i458.us.us.us.i.i, i1 false, !dbg !7622
  %_0.i283.us.us.us.i.i = select i1 %301, float %_4.i.i399.v.us.us.us.i.i, float 0.000000e+00, !dbg !7622
  %_3.i115.us.us.us.i115.i = fcmp ule float %_0.i283.us.us.us.i.i, %_86.i78.i616.us.us.us.i.i, !dbg !7623
  %_4.i276.us.us.us.i.i = select i1 %_3.i115.us.us.us.i115.i, i32 %_88.i81.i493.i.i, i32 %_87.i80.i492.i.i, !dbg !7625
  %_0.i277.us.us.us.i.i = bitcast i32 %_4.i276.us.us.us.i.i to float, !dbg !7627
  %_0.i175.us.us.us.i116.i = fsub float %_0.i283.us.us.us.i.i, %_86.i78.i616.us.us.us.i.i, !dbg !7629
  %_4.i143.us.us.us.i.i = fmul float %_0.i175.us.us.us.i116.i, %_0.i277.us.us.us.i.i, !dbg !7631
  %_0.i144.us.us.us.i117.i = fadd float %_86.i78.i616.us.us.us.i.i, %_4.i143.us.us.us.i.i, !dbg !7631
  %302 = tail call noundef float @llvm.fabs.f32(float %_0.i144.us.us.us.i117.i), !dbg !7633
  %303 = fcmp uge float %302, 0x3BC79CA100000000, !dbg !7636
  %_0.i212.us.us.us.i.i = select i1 %303, float %_0.i144.us.us.us.i117.i, float 0.000000e+00, !dbg !7638
  store float %_0.i212.us.us.us.i.i, ptr %273, align 4, !dbg !7639, !alias.scope !7573, !noalias !7576
  %_0.i161.us.us.us.i118.i = fmul float %_0.i212.us.us.us.i.i, 0x3FC542A5A0000000, !dbg !7640
  %_3.i.i343.us.us.us.inv.i.i = fcmp ogt float %_0.i161.us.us.us.i118.i, -1.260000e+02, !dbg !7643
  %_0.i.i350.us.us.us.i.i = select i1 %_3.i.i343.us.us.us.inv.i.i, float %_0.i161.us.us.us.i118.i, float -1.260000e+02, !dbg !7643
  %_3.i.i426.us.us.us.inv.i.i = fcmp olt float %_0.i.i350.us.us.us.i.i, 1.270000e+02, !dbg !7647
  %_0.i.i433.us.us.us.i.i = select i1 %_3.i.i426.us.us.us.inv.i.i, float %_0.i.i350.us.us.us.i.i, float 1.270000e+02, !dbg !7647
  %304 = tail call noundef float @llvm.floor.f32(float %_0.i.i433.us.us.us.i.i), !dbg !7650
  %_0.i168.us.us.us.i119.i = fsub float %_0.i.i433.us.us.us.i.i, %304, !dbg !7654
  %305 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.28889.2.i.i), !dbg !7656
  %306 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.41890.2.i.i), !dbg !7660
  %_3.i.i383.us.us.us.i.i = fcmp ule float %305, %306, !dbg !7662
  %_6.i.i385.us.us.us.i.i = bitcast float %305 to i32, !dbg !7665
  %_8.i.i387.us.us.us.i.i = bitcast float %306 to i32, !dbg !7668
  %_4.i.i390.us.us.us.i.i = select i1 %_3.i.i383.us.us.us.i.i, i32 %_8.i.i387.us.us.us.i.i, i32 %_6.i.i385.us.us.us.i.i, !dbg !7670
  %_4.i262.us.us.us.i.i = select i1 %_3.i111.i.i, i32 %_6.i.i385.us.us.us.i.i, i32 %_4.i.i390.us.us.us.i.i, !dbg !7671
  %_0.i159.us.us.us.i120.i = fmul float %305, 5.000000e-01, !dbg !7673
  %_0.i158.us.us.us.i121.i = fmul float %306, 5.000000e-01, !dbg !7675
  %_0.i139.us.us.us.i122.i = fadd float %_0.i158.us.us.us.i121.i, %_0.i159.us.us.us.i120.i, !dbg !7677
  %_6.i250.us.us.us.i.i = bitcast float %_0.i139.us.us.us.i122.i to i32, !dbg !7679
  %_4.i255.us.us.us.i.i = select i1 %_3.i109.i.i, i32 %_4.i262.us.us.us.i.i, i32 %_6.i250.us.us.us.i.i, !dbg !7682
  %_0.i256.us.us.us.i.i = bitcast i32 %_4.i255.us.us.us.i.i to float, !dbg !7683
  %_3.i.i375.us.us.us.i.i = fcmp ule float %_0.i256.us.us.us.i.i, 0x3E45798EE0000000, !dbg !7685
  %_4.i.i381.us.us.us.i.i = select i1 %_3.i.i375.us.us.us.i.i, i32 841731191, i32 %_4.i255.us.us.us.i.i, !dbg !7688
  %_0.i.i382.us.us.us.i.i = bitcast i32 %_4.i.i381.us.us.us.i.i to float, !dbg !7690
  %_3.i.i335.us.us.us.i.i = fcmp ule float %_0.i.i382.us.us.us.i.i, 0x3810000000000000, !dbg !7692
  %_4.i.i341.us.us.us.i.i = select i1 %_3.i.i335.us.us.us.i.i, i32 8388608, i32 %_4.i.i381.us.us.us.i.i, !dbg !7697
  %_5.i200.us.us.us.i.i = and i32 %_4.i.i341.us.us.us.i.i, 8388607, !dbg !7699
  %_4.i201.us.us.us.i.i = or disjoint i32 %_5.i200.us.us.us.i.i, 1065353216, !dbg !7699
  %significand.i202.us.us.us.i.i = bitcast i32 %_4.i201.us.us.us.i.i to float, !dbg !7701
  %_0.i167.us.us.us.i123.i = fadd float %significand.i202.us.us.us.i.i, -1.000000e+00, !dbg !7703
  %_0.i148.us.us.us.i124.i = fmul float %_0.i167.us.us.us.i123.i, 0x3F9B17A960000000, !dbg !7705
  %307 = fsub float 0x3FBF9A8440000000, %_0.i148.us.us.us.i124.i, !dbg !7707
  %_0.i148.us.us.us.1.i125.i = fmul float %_0.i167.us.us.us.i123.i, %307, !dbg !7705
  %_0.i134.us.us.us.1.i.i = fadd float %_0.i148.us.us.us.1.i125.i, 0xBFD1E3F400000000, !dbg !7707
  %_0.i148.us.us.us.2.i126.i = fmul float %_0.i167.us.us.us.i123.i, %_0.i134.us.us.us.1.i.i, !dbg !7705
  %_0.i134.us.us.us.2.i.i = fadd float %_0.i148.us.us.us.2.i126.i, 0x3FDD544F20000000, !dbg !7707
  %_0.i148.us.us.us.3.i127.i = fmul float %_0.i167.us.us.us.i123.i, %_0.i134.us.us.us.2.i.i, !dbg !7705
  %_0.i134.us.us.us.3.i.i = fadd float %_0.i148.us.us.us.3.i127.i, 0xBFE6FC2A60000000, !dbg !7707
  %_0.i148.us.us.us.4.i.i = fmul float %_0.i167.us.us.us.i123.i, %_0.i134.us.us.us.3.i.i, !dbg !7705
  %_0.i134.us.us.us.4.i.i = fadd float %_0.i148.us.us.us.4.i.i, 0x3FF714B2A0000000, !dbg !7707
  %_9.i203.us.us.us.i.i = lshr i32 %_4.i.i341.us.us.us.i.i, 23, !dbg !7709
  %_8.i204.us.us.us.i.i = or disjoint i32 %_9.i203.us.us.us.i.i, 1258291200, !dbg !7709
  %_7.i205.us.us.us.i.i = bitcast i32 %_8.i204.us.us.us.i.i to float, !dbg !7710
  %exponent.i206.us.us.us.i.i = fadd float %_7.i205.us.us.us.i.i, 0xC160000FE0000000, !dbg !7712
  %_0.i147.us.us.us.i128.i = fmul float %_0.i167.us.us.us.i123.i, %_0.i134.us.us.us.4.i.i, !dbg !7713
  %_0.i133.us.us.us.i.i = fadd float %exponent.i206.us.us.us.i.i, %_0.i147.us.us.us.i128.i, !dbg !7715
  %_0.i157.us.us.us.i129.i = fmul float %_0.i133.us.us.us.i.i, 0x4018151820000000, !dbg !7717
  %_3.i.i450.us.us.us.inv.i.i = fcmp olt float %_0.i157.us.us.us.i129.i, 2.400000e+01, !dbg !7719
  %_0.i.i457.us.us.us.i.i = select i1 %_3.i.i450.us.us.us.inv.i.i, float %_0.i157.us.us.us.i129.i, float 2.400000e+01, !dbg !7719
  %_3.i.i367.us.us.us.inv.i.i = fcmp ogt float %_0.i.i457.us.us.us.i.i, -1.600000e+02, !dbg !7722
  %_0.i.i374.us.us.us.i.i = select i1 %_3.i.i367.us.us.us.inv.i.i, float %_0.i.i457.us.us.us.i.i, float -1.600000e+02, !dbg !7722
  %_55.i.i.us.us.us.i130.i = load float, ptr %281, align 4, !dbg !7725, !alias.scope !7726, !noalias !7729, !noundef !12
  %_3.i107.us.us.us.i.i = fcmp ule float %_55.i.i.us.us.us.i130.i, 0.000000e+00, !dbg !7731
  %_3.i93.us.us.us.i131.i = fcmp oge float %_0.i.i374.us.us.us.i.i, %threshold.i.i.i.i, !dbg !7733
  %_3.i91.us.us.us.i132.i = fcmp oge float %_0.i.i374.us.us.us.i.i, %_0.i174.i.i, !dbg !7735
  %..i92.us.us.us.i.i = sext i1 %_3.i91.us.us.us.i132.i to i32, !dbg !7737
  %_0.i325.us.us.us.i.i = sext i1 %_3.i93.us.us.us.i131.i to i32, !dbg !7739
  %_0.i319.us.us.us.i.i = select i1 %_3.i107.us.us.us.i.i, i32 %_0.i325.us.us.us.i.i, i32 %..i92.us.us.us.i.i, !dbg !7739
  %_0.i331.us.us.us.i.i = xor i32 %..i92.us.us.us.i.i, -1, !dbg !7741
  %_3.i105.us.us.us.i133.i = fcmp ogt float %_67.i.i618.us.us.us.i.i, 0.000000e+00, !dbg !7743
  %_0.i324.us.us.us.i.i = select i1 %_3.i105.us.us.us.i133.i, i32 %_0.i331.us.us.us.i.i, i32 0, !dbg !7745
  %_0.i323.us.us.us.i.i = select i1 %_3.i107.us.us.us.i.i, i32 0, i32 %_0.i324.us.us.us.i.i, !dbg !7747
  %_0.i318.us.us.us.i.i = or i32 %_0.i323.us.us.us.i.i, %_0.i319.us.us.us.i.i, !dbg !7749
  %_5.i245.us.us.us.i.i = and i32 %_0.i318.us.us.us.i.i, 1065353216, !dbg !7751
  %_0.i249.us.us.us.i134.i = bitcast i32 %_5.i245.us.us.us.i.i to float, !dbg !7753
  %_0.i173.us.us.us.i135.i = fadd float %_67.i.i618.us.us.us.i.i, -1.000000e+00, !dbg !7755
  %308 = trunc nsw i32 %_0.i323.us.us.us.i.i to i1, !dbg !7757
  %_4.i243.v.us.us.us.i.i = select i1 %308, float %_0.i173.us.us.us.i135.i, float %_67.i.i618.us.us.us.i.i, !dbg !7757
  %309 = trunc nsw i32 %_0.i319.us.us.us.i.i to i1, !dbg !7759
  %_0.i237.us.us.us.i.i = select i1 %309, float %_71.i.i506507.i.i, float %_4.i243.v.us.us.us.i.i, !dbg !7759
  store float %_0.i237.us.us.us.i.i, ptr %282, align 4, !dbg !7761, !alias.scope !7726, !noalias !7729
  store i32 %_5.i245.us.us.us.i.i, ptr %281, align 4, !dbg !7762, !alias.scope !7726, !noalias !7729
  %_0.i171.us.us.us.i136.i = fsub float %_0.i.i374.us.us.us.i.i, %threshold.i.i.i.i, !dbg !7763
  %_0.i156.us.us.us.i137.i = fmul float %_0.i172.i.i, %_0.i171.us.us.us.i136.i, !dbg !7765
  %_3.i.i359.inv.us.us.us.i.i = fcmp ogt float %_0.i156.us.us.us.i137.i, %284, !dbg !7767
  %_4.i.i365.v.us.us.us.i.i = select i1 %_3.i.i359.inv.us.us.us.i.i, float %_0.i156.us.us.us.i137.i, float %284, !dbg !7767
  %_3.i.i442.us.us.us.i.i = fcmp olt float %_4.i.i365.v.us.us.us.i.i, 0.000000e+00, !dbg !7770
  %310 = fcmp ule float %_0.i249.us.us.us.i134.i, 0.000000e+00, !dbg !7773
  %311 = select i1 %310, i1 %_3.i.i442.us.us.us.i.i, i1 false, !dbg !7775
  %_0.i230.us.us.us.i.i = select i1 %311, float %_4.i.i365.v.us.us.us.i.i, float 0.000000e+00, !dbg !7775
  %_3.i101.us.us.us.i138.i = fcmp ule float %_0.i230.us.us.us.i.i, %_86.i.i620.us.us.us.i.i, !dbg !7776
  %_4.i223.us.us.us.i.i = select i1 %_3.i101.us.us.us.i138.i, i32 %_88.i.i510.i.i, i32 %_87.i.i509.i.i, !dbg !7778
  %_0.i224.us.us.us.i139.i = bitcast i32 %_4.i223.us.us.us.i.i to float, !dbg !7780
  %_0.i170.us.us.us.i140.i = fsub float %_0.i230.us.us.us.i.i, %_86.i.i620.us.us.us.i.i, !dbg !7782
  %_4.i141.us.us.us.i.i = fmul float %_0.i170.us.us.us.i140.i, %_0.i224.us.us.us.i139.i, !dbg !7784
  %_0.i142.us.us.us.i141.i = fadd float %_86.i.i620.us.us.us.i.i, %_4.i141.us.us.us.i.i, !dbg !7784
  %312 = tail call noundef float @llvm.fabs.f32(float %_0.i142.us.us.us.i141.i), !dbg !7786
  %313 = fcmp uge float %312, 0x3BC79CA100000000, !dbg !7789
  %_0.i208.us.us.us.i.i = select i1 %313, float %_0.i142.us.us.us.i141.i, float 0.000000e+00, !dbg !7791
  store float %_0.i208.us.us.us.i.i, ptr %285, align 4, !dbg !7792, !alias.scope !7726, !noalias !7729
  %_0.i155.us.us.us.i142.i = fmul float %_0.i208.us.us.us.i.i, 0x3FC542A5A0000000, !dbg !7793
  %_3.i.i351.us.us.us.inv.i.i = fcmp ogt float %_0.i155.us.us.us.i142.i, -1.260000e+02, !dbg !7796
  %_0.i.i358.us.us.us.i.i = select i1 %_3.i.i351.us.us.us.inv.i.i, float %_0.i155.us.us.us.i142.i, float -1.260000e+02, !dbg !7796
  %_3.i.i434.us.us.us.inv.i.i = fcmp olt float %_0.i.i358.us.us.us.i.i, 1.270000e+02, !dbg !7800
  %_0.i.i441.us.us.us.i.i = select i1 %_3.i.i434.us.us.us.inv.i.i, float %_0.i.i358.us.us.us.i.i, float 1.270000e+02, !dbg !7800
  %314 = tail call noundef float @llvm.floor.f32(float %_0.i.i441.us.us.us.i.i), !dbg !7803
  %_0.i169.us.us.us.i143.i = fsub float %_0.i.i441.us.us.us.i.i, %314, !dbg !7807
  %_0.i153.us.us.us.i.i = fmul float %_0.i169.us.us.us.i143.i, 0x3F5E974FA0000000, !dbg !7809
  %_0.i138.us.us.us.i.i = fadd float %_0.i153.us.us.us.i.i, 0x3F82778560000000, !dbg !7811
  %_0.i153.us.us.us.1.i.i = fmul float %_0.i169.us.us.us.i143.i, %_0.i138.us.us.us.i.i, !dbg !7809
  %_0.i138.us.us.us.1.i.i = fadd float %_0.i153.us.us.us.1.i.i, 0x3FAC91CE60000000, !dbg !7811
  %_0.i153.us.us.us.2.i.i = fmul float %_0.i169.us.us.us.i143.i, %_0.i138.us.us.us.1.i.i, !dbg !7809
  %_0.i138.us.us.us.2.i.i = fadd float %_0.i153.us.us.us.2.i.i, 0x3FCEBDB560000000, !dbg !7811
  %_0.i153.us.us.us.3.i.i = fmul float %_0.i169.us.us.us.i143.i, %_0.i138.us.us.us.2.i.i, !dbg !7809
  %_0.i138.us.us.us.3.i.i = fadd float %_0.i153.us.us.us.3.i.i, 0x3FE62E4BA0000000, !dbg !7811
  %_0.i151.us.us.us.i.i = fmul float %_0.i168.us.us.us.i119.i, 0x3F5E974FA0000000, !dbg !7813
  %_0.i136.us.us.us.i.i = fadd float %_0.i151.us.us.us.i.i, 0x3F82778560000000, !dbg !7815
  %_0.i151.us.us.us.1.i.i = fmul float %_0.i168.us.us.us.i119.i, %_0.i136.us.us.us.i.i, !dbg !7813
  %_0.i136.us.us.us.1.i.i = fadd float %_0.i151.us.us.us.1.i.i, 0x3FAC91CE60000000, !dbg !7815
  %_0.i151.us.us.us.2.i.i = fmul float %_0.i168.us.us.us.i119.i, %_0.i136.us.us.us.1.i.i, !dbg !7813
  %_0.i136.us.us.us.2.i.i = fadd float %_0.i151.us.us.us.2.i.i, 0x3FCEBDB560000000, !dbg !7815
  %_0.i151.us.us.us.3.i.i = fmul float %_0.i168.us.us.us.i119.i, %_0.i136.us.us.us.2.i.i, !dbg !7813
  %_0.i136.us.us.us.3.i.i = fadd float %_0.i151.us.us.us.3.i.i, 0x3FE62E4BA0000000, !dbg !7815
  %_0.i150.us.us.us.i144.i = fmul float %_0.i168.us.us.us.i119.i, %_0.i136.us.us.us.3.i.i, !dbg !7817
  %_0.i135.us.us.us.i.i = fadd float %_0.i150.us.us.us.i144.i, 1.000000e+00, !dbg !7819
  %biased.i.us.us.us.i145.i = fadd float %304, 0x4160000FE0000000, !dbg !7821
  %_4.i81.us.us.us.i146.i = bitcast float %biased.i.us.us.us.i145.i to i32, !dbg !7823
  %_3.i82.us.us.us.i147.i = shl i32 %_4.i81.us.us.us.i146.i, 23, !dbg !7825
  %_0.i83.us.us.us.i148.i = bitcast i32 %_3.i82.us.us.us.i147.i to float, !dbg !7826
  %_0.i149.us.us.us.i149.i = fmul float %_0.i135.us.us.us.i.i, %_0.i83.us.us.us.i148.i, !dbg !7828
  %_3.i89.us.us.us.i150.i = fcmp une float %_0.i212.us.us.us.i.i, 0.000000e+00, !dbg !7830
  %_0.i320497.not.us.us.us.i.i = and i1 %_3.i113.i.i, %_3.i89.us.us.us.i150.i, !dbg !7832
  %_0.i160.us.us.us.i151.i = fmul float %_0.i191.us.us.us.i57.i, %_0.i149.us.us.us.i149.i, !dbg !7832
  %_4.i269.v.us.us.us.i.i = select i1 %_0.i320497.not.us.us.us.i.i, float %_0.i160.us.us.us.i151.i, float %_0.i191.us.us.us.i57.i, !dbg !7834
  %_0.i.us.us.us.i152.i = fmul float %_0.i169.us.us.us.i143.i, %_0.i138.us.us.us.3.i.i, !dbg !7836
  %_0.i137.us.us.us.i.i = fadd float %_0.i.us.us.us.i152.i, 1.000000e+00, !dbg !7838
  %biased.i84.us.us.us.i153.i = fadd float %314, 0x4160000FE0000000, !dbg !7840
  %_4.i85.us.us.us.i154.i = bitcast float %biased.i84.us.us.us.i153.i to i32, !dbg !7842
  %_3.i86.us.us.us.i155.i = shl i32 %_4.i85.us.us.us.i154.i, 23, !dbg !7844
  %_0.i87.us.us.us.i156.i = bitcast i32 %_3.i86.us.us.us.i155.i to float, !dbg !7845
  %_0.i152.us.us.us.i157.i = fmul float %_0.i137.us.us.us.i.i, %_0.i87.us.us.us.i156.i, !dbg !7847
  %_3.i88.us.us.us.i158.i = fcmp une float %_0.i208.us.us.us.i.i, 0.000000e+00, !dbg !7849
  %_0.i317514.not.us.us.us.i.i = and i1 %_3.i99.i.i, %_3.i88.us.us.us.i158.i, !dbg !7851
  %_0.i154.us.us.us.i159.i = fmul float %_0.i189.us.us.us.i59.i, %_0.i152.us.us.us.i157.i, !dbg !7851
  %_4.i216.v.us.us.us.i.i = select i1 %_0.i317514.not.us.us.us.i.i, float %_0.i154.us.us.us.i159.i, float %_0.i189.us.us.us.i59.i, !dbg !7853
  store float %_4.i269.v.us.us.us.i.i, ptr %_97.i.us.us.us.i43.i, align 4, !dbg !7855, !alias.scope !7858, !noalias !7404
  store float %_4.i216.v.us.us.us.i.i, ptr %_115.i.us.us.us.i48.i, align 4, !dbg !7861, !alias.scope !7863, !noalias !7426
  %exitcond883.not.i.i = icmp eq i64 %288, %_55.i, !dbg !7866
  br i1 %exitcond883.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i, label %bb32.i.us.us.us.i42.i, !dbg !7869

bb8.i79.us.us.us.i172.i:                          ; preds = %bb5.i.preheader.us.us.us.i170.i
  %_34.i.us.us.us.i173.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.us.us.us.i67.i, !dbg !7870
  br i1 %_34.i.us.us.us.i173.i, label %bb10.i80.us.us.us.i175.i, label %panic5.i.i174.i, !dbg !7870

bb10.i80.us.us.us.i175.i:                         ; preds = %bb8.i79.us.us.us.i172.i
  %315 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.us.us.us.i63.i, !dbg !7871
  %left_own.i.us.us.us.i176.i = load float, ptr %315, align 4, !dbg !7871, !alias.scope !7479, !noalias !7494, !noundef !12
  %316 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.us.us.us.i67.i, !dbg !7870
  %right_own.i.us.us.us.i177.i = load float, ptr %316, align 4, !dbg !7870, !alias.scope !7481, !noalias !7496, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i, !dbg !7499

bb5.i.preheader.us.us.us.i170.i:                  ; preds = %bb50.i.us.us.us.i55.i
  br i1 %_31.i78.us.us.us.i68.i, label %bb8.i79.us.us.us.i172.i, label %panic4.i.i171.i, !dbg !7871

bb14.i63.preheader.us.us.us.i162.i:               ; preds = %bb50.i.us.us.us.i55.i
  br i1 %_31.i78.us.us.us.i68.i, label %bb17.i.us.us.us.i164.i, label %panic15.i.i163.i, !dbg !7503

bb23.i.preheader.us.us.us.i69.i:                  ; preds = %bb50.i.us.us.us.i55.i
  br i1 %_31.i78.us.us.us.i68.i, label %bb27.i60.us.us.us.i71.i, label %panic28.i.i70.i, !dbg !7493

bb32.i.us.i186.i:                                 ; preds = %bb30.i.lr.ph.split.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i
  %_86.i.i620.us.i.i = phi float [ %_0.i208.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i ], [ %.promoted619.i.i, %bb30.i.lr.ph.split.us.i.i ]
  %_67.i.i618.us.i.i = phi float [ %_0.i237.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i ], [ %.promoted617.i.i, %bb30.i.lr.ph.split.us.i.i ]
  %_86.i78.i616.us.i.i = phi float [ %_0.i212.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i ], [ %.promoted615.i.i, %bb30.i.lr.ph.split.us.i.i ]
  %_67.i59.i614.us.i.i = phi float [ %_0.i290.us.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i ], [ %.promoted.i.i, %bb30.i.lr.ph.split.us.i.i ]
  %iter.sroa.0.0.i613.us.i.i = phi i64 [ %317, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i ], [ 0, %bb30.i.lr.ph.split.us.i.i ]
  %317 = add nuw nsw i64 %iter.sroa.0.0.i613.us.i.i, 1, !dbg !7362
  %_28.i.us.i182.i = trunc i64 %iter.sroa.0.0.i613.us.i.i to i32, !dbg !7376
  %now.i.us.i183.i = add i32 %base.i.i35.i, %_28.i.us.i182.i, !dbg !7379
  %_31.i.us.i184.i = and i32 %now.i.us.i183.i, %_58.i29.i, !dbg !7382
  %_30.i.us.i185.i = zext i32 %_31.i.us.i184.i to i64, !dbg !7384
  %_97.i.us.i187.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %iter.sroa.0.0.i613.us.i.i, !dbg !7385
  %_98.not.not.i.us.i188.i = icmp ugt i64 %_64.1.i25.i, %_30.i.us.i185.i, !dbg !7394
  br i1 %_98.not.not.i.us.i188.i, label %bb35.i.us.i189.i, label %bb36.i.i45.i, !dbg !7394, !prof !2709

bb35.i.us.i189.i:                                 ; preds = %bb32.i.us.i186.i
  %_0.i195.us.i.i = load float, ptr %_97.i.us.i187.i, align 4, !dbg !7399, !alias.scope !7401, !noalias !7404, !noundef !12
  %_107.i.us.i190.i = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_30.i.us.i185.i, !dbg !7405
  store float %_0.i195.us.i.i, ptr %_107.i.us.i190.i, align 4, !dbg !7409, !alias.scope !7411, !noalias !7359
  %_115.i.us.i191.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %iter.sroa.0.0.i613.us.i.i, !dbg !7414
  %_116.not.not.i.us.i192.i = icmp ugt i64 %_66.1.i27.i, %_30.i.us.i185.i, !dbg !7872
  br i1 %_116.not.not.i.us.i192.i, label %bb40.i.us.i195.i, label %bb41.i.i193.i, !dbg !7872, !prof !2709

bb40.i.us.i195.i:                                 ; preds = %bb35.i.us.i189.i
  %_0.i193.us.i.i = load float, ptr %_115.i.us.i191.i, align 4, !dbg !7421, !alias.scope !7423, !noalias !7426, !noundef !12
  %_123.i.us.i196.i = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_30.i.us.i185.i, !dbg !7427
  store float %_0.i193.us.i.i, ptr %_123.i.us.i196.i, align 4, !dbg !7434, !alias.scope !7436, !noalias !7359
  %_53.i.us.i197.i = sub i32 %now.i.us.i183.i, %_59.i30.i, !dbg !7439
  %_52.i.us.i198.i = and i32 %_53.i.us.i197.i, %_58.i29.i, !dbg !7442
  %_51.i.us.i199.i = zext i32 %_52.i.us.i198.i to i64, !dbg !7443
  %_156.not.not.i.us.i200.i = icmp ugt i64 %_64.1.i25.i, %_51.i.us.i199.i, !dbg !7444
  br i1 %_156.not.not.i.us.i200.i, label %bb50.i.us.i201.i, label %bb51.i.i54.i, !dbg !7444, !prof !2709

bb50.i.us.i201.i:                                 ; preds = %bb40.i.us.i195.i
  %_163.i.us.i202.i = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_51.i.us.i199.i, !dbg !7449
  %_0.i191.us.i203.i = load float, ptr %_163.i.us.i202.i, align 4, !dbg !7453, !alias.scope !7455, !noalias !7359, !noundef !12
  %_164.not.not.i.us.i204.i = icmp ugt i64 %_66.1.i27.i, %_51.i.us.i199.i, !dbg !7873
  br i1 %_164.not.not.i.us.i204.i, label %bb53.i.us.i207.i, label %bb54.i.i205.i, !dbg !7873, !prof !2709

bb53.i.us.i207.i:                                 ; preds = %bb50.i.us.i201.i
  %_169.i.us.i208.i = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_51.i.us.i199.i, !dbg !7458
  %_0.i189.us.i209.i = load float, ptr %_169.i.us.i208.i, align 4, !dbg !7466, !alias.scope !7468, !noalias !7359, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7471), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7477), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7479), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7481), !dbg !7474
  %_18.i72.us.i210.i = load i32, ptr %_31.i17.i, align 4, !dbg !7483, !alias.scope !7485, !noalias !7486, !noundef !12
  %_17.i.us.i211.i = sub i32 %now.i.us.i183.i, %_18.i72.us.i210.i, !dbg !7488
  %_16.i73.us.i212.i = and i32 %_17.i.us.i211.i, %_58.i29.i, !dbg !7483
  %_15.i74.us.i213.i = zext i32 %_16.i73.us.i212.i to i64, !dbg !7483
  %_26.i.us.i214.i = load i32, ptr %_33.i18.i, align 4, !dbg !7483, !alias.scope !7490, !noalias !7491, !noundef !12
  %_25.i.us.i215.i = sub i32 %now.i.us.i183.i, %_26.i.us.i214.i, !dbg !7488
  %_24.i.us.i216.i = and i32 %_25.i.us.i215.i, %_58.i29.i, !dbg !7483
  %_23.i77.us.i217.i = zext i32 %_24.i.us.i216.i to i64, !dbg !7483
  %_31.i78.us.i218.i = icmp samesign ugt i64 %_64.1.i25.i, %_15.i74.us.i213.i, !dbg !7483
  switch i8 %_0.sroa.0.0.i479.i.i, label %default.unreachable [
    i8 0, label %bb5.i.preheader.us.i312.i
    i8 1, label %bb14.i63.preheader.us.i306.i
    i8 2, label %bb23.i.preheader.us.i219.i
  ], !dbg !7492

bb27.i60.us.i220.i:                               ; preds = %bb23.i.preheader.us.i219.i
  %318 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.us.i213.i, !dbg !7493
  %_79.i.us.i221.i = load float, ptr %318, align 4, !dbg !7493, !alias.scope !7479, !noalias !7494, !noundef !12
  %_85.i.us.i222.i = icmp samesign ugt i64 %_66.1.i27.i, %_15.i74.us.i213.i, !dbg !7495
  br i1 %_85.i.us.i222.i, label %bb29.i61.us.i223.i, label %panic30.i.i74.i, !dbg !7495

bb29.i61.us.i223.i:                               ; preds = %bb27.i60.us.i220.i
  %_87.i.us.i225.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.us.i217.i, !dbg !7497
  br i1 %_87.i.us.i225.i, label %bb31.i.us.i226.i, label %panic32.i.i78.i, !dbg !7497

bb31.i.us.i226.i:                                 ; preds = %bb29.i61.us.i223.i
  %319 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_15.i74.us.i213.i, !dbg !7495
  %_83.i.us.i224.i = load float, ptr %319, align 4, !dbg !7495, !alias.scope !7481, !noalias !7496, !noundef !12
  %320 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.us.i217.i, !dbg !7497
  %_86.i.us.i229.i = load float, ptr %320, align 4, !dbg !7497, !alias.scope !7481, !noalias !7496, !noundef !12
  %321 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_23.i77.us.i217.i, !dbg !7498
  %_88.i.us.i230.i = load float, ptr %321, align 4, !dbg !7498, !alias.scope !7479, !noalias !7494, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i, !dbg !7499

bb17.i.us.i307.i:                                 ; preds = %bb14.i63.preheader.us.i306.i
  %_59.i.us.i308.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.us.i217.i, !dbg !7502
  br i1 %_59.i.us.i308.i, label %bb19.i.us.i309.i, label %panic17.i.i166.i, !dbg !7502

bb19.i.us.i309.i:                                 ; preds = %bb17.i.us.i307.i
  %322 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.us.i213.i, !dbg !7503
  %left_own16.i.us.i310.i = load float, ptr %322, align 4, !dbg !7503, !alias.scope !7479, !noalias !7494, !noundef !12
  %323 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.us.i217.i, !dbg !7502
  %right_own18.i.us.i311.i = load float, ptr %323, align 4, !dbg !7502, !alias.scope !7481, !noalias !7496, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i, !dbg !7499

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i: ; preds = %bb10.i80.us.i315.i, %bb19.i.us.i309.i, %bb31.i.us.i226.i
  %taps.i.sroa.41890.1.i.i = phi float [ %right_own.i.us.i317.i, %bb10.i80.us.i315.i ], [ %left_own16.i.us.i310.i, %bb19.i.us.i309.i ], [ %_88.i.us.i230.i, %bb31.i.us.i226.i ], !dbg !7483
  %taps.i.sroa.28889.1.i.i = phi float [ %right_own.i.us.i317.i, %bb10.i80.us.i315.i ], [ %right_own18.i.us.i311.i, %bb19.i.us.i309.i ], [ %_86.i.us.i229.i, %bb31.i.us.i226.i ], !dbg !7483
  %taps.i.sroa.15888.1.i.i = phi float [ %left_own.i.us.i316.i, %bb10.i80.us.i315.i ], [ %right_own18.i.us.i311.i, %bb19.i.us.i309.i ], [ %_83.i.us.i224.i, %bb31.i.us.i226.i ], !dbg !7483
  %taps.i.sroa.0.1.i232.i = phi float [ %left_own.i.us.i316.i, %bb10.i80.us.i315.i ], [ %left_own16.i.us.i310.i, %bb19.i.us.i309.i ], [ %_79.i.us.i221.i, %bb31.i.us.i226.i ], !dbg !7483
  %324 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.1.i232.i), !dbg !7504
  %325 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.15888.1.i.i), !dbg !7507
  %_3.i.i417.us.i.i = fcmp ule float %324, %325, !dbg !7509
  %_6.i.i419.us.i.i = bitcast float %324 to i32, !dbg !7512
  %_8.i.i421.us.i.i = bitcast float %325 to i32, !dbg !7515
  %_4.i.i424.us.i.i = select i1 %_3.i.i417.us.i.i, i32 %_8.i.i421.us.i.i, i32 %_6.i.i419.us.i.i, !dbg !7517
  %_4.i315.us.i233.i = select i1 %_3.i125.i.i, i32 %_6.i.i419.us.i.i, i32 %_4.i.i424.us.i.i, !dbg !7518
  %_0.i165.us.i234.i = fmul float %324, 5.000000e-01, !dbg !7520
  %_0.i164.us.i235.i = fmul float %325, 5.000000e-01, !dbg !7522
  %_0.i140.us.i236.i = fadd float %_0.i164.us.i235.i, %_0.i165.us.i234.i, !dbg !7524
  %_6.i303.us.i.i = bitcast float %_0.i140.us.i236.i to i32, !dbg !7526
  %_4.i308.us.i.i = select i1 %_3.i123.i.i, i32 %_4.i315.us.i233.i, i32 %_6.i303.us.i.i, !dbg !7529
  %_0.i309.us.i.i = bitcast i32 %_4.i308.us.i.i to float, !dbg !7530
  %_3.i.i409.us.i.i = fcmp ule float %_0.i309.us.i.i, 0x3E45798EE0000000, !dbg !7532
  %_4.i.i415.us.i.i = select i1 %_3.i.i409.us.i.i, i32 841731191, i32 %_4.i308.us.i.i, !dbg !7535
  %_0.i.i416.us.i237.i = bitcast i32 %_4.i.i415.us.i.i to float, !dbg !7537
  %_3.i.i.us.i238.i = fcmp ule float %_0.i.i416.us.i237.i, 0x3810000000000000, !dbg !7539
  %_4.i.i.us.i239.i = select i1 %_3.i.i.us.i238.i, i32 8388608, i32 %_4.i.i415.us.i.i, !dbg !7544
  %_5.i196.us.i.i = and i32 %_4.i.i.us.i239.i, 8388607, !dbg !7546
  %_4.i197.us.i.i = or disjoint i32 %_5.i196.us.i.i, 1065353216, !dbg !7546
  %significand.i.us.i240.i = bitcast i32 %_4.i197.us.i.i to float, !dbg !7548
  %_0.i166.us.i241.i = fadd float %significand.i.us.i240.i, -1.000000e+00, !dbg !7550
  %_0.i146.us.i242.i = fmul float %_0.i166.us.i241.i, 0x3F9B17A960000000, !dbg !7552
  %326 = fsub float 0x3FBF9A8440000000, %_0.i146.us.i242.i, !dbg !7554
  %_0.i146.us.1.i243.i = fmul float %_0.i166.us.i241.i, %326, !dbg !7552
  %_0.i132.us.1.i.i = fadd float %_0.i146.us.1.i243.i, 0xBFD1E3F400000000, !dbg !7554
  %_0.i146.us.2.i244.i = fmul float %_0.i166.us.i241.i, %_0.i132.us.1.i.i, !dbg !7552
  %_0.i132.us.2.i.i = fadd float %_0.i146.us.2.i244.i, 0x3FDD544F20000000, !dbg !7554
  %_0.i146.us.3.i245.i = fmul float %_0.i166.us.i241.i, %_0.i132.us.2.i.i, !dbg !7552
  %_0.i132.us.3.i.i = fadd float %_0.i146.us.3.i245.i, 0xBFE6FC2A60000000, !dbg !7554
  %_0.i146.us.4.i.i = fmul float %_0.i166.us.i241.i, %_0.i132.us.3.i.i, !dbg !7552
  %_0.i132.us.4.i.i = fadd float %_0.i146.us.4.i.i, 0x3FF714B2A0000000, !dbg !7554
  %_9.i.us.i246.i = lshr i32 %_4.i.i.us.i239.i, 23, !dbg !7556
  %_8.i198.us.i.i = or disjoint i32 %_9.i.us.i246.i, 1258291200, !dbg !7556
  %_7.i.us.i247.i = bitcast i32 %_8.i198.us.i.i to float, !dbg !7557
  %exponent.i.us.i248.i = fadd float %_7.i.us.i247.i, 0xC160000FE0000000, !dbg !7559
  %_0.i145.us.i249.i = fmul float %_0.i166.us.i241.i, %_0.i132.us.4.i.i, !dbg !7560
  %_0.i131.us.i.i = fadd float %exponent.i.us.i248.i, %_0.i145.us.i249.i, !dbg !7562
  %_0.i163.us.i250.i = fmul float %_0.i131.us.i.i, 0x4018151820000000, !dbg !7564
  %_3.i.i466.us.inv.i.i = fcmp olt float %_0.i163.us.i250.i, 2.400000e+01, !dbg !7566
  %_0.i.i473.us.i.i = select i1 %_3.i.i466.us.inv.i.i, float %_0.i163.us.i250.i, float 2.400000e+01, !dbg !7566
  %_3.i.i401.us.inv.i.i = fcmp ogt float %_0.i.i473.us.i.i, -1.600000e+02, !dbg !7569
  %_0.i.i408.us.i.i = select i1 %_3.i.i401.us.inv.i.i, float %_0.i.i473.us.i.i, float -1.600000e+02, !dbg !7569
  %_55.i49.i.us.i.i = load float, ptr %269, align 4, !dbg !7572, !alias.scope !7573, !noalias !7576, !noundef !12
  %_3.i121.us.i251.i = fcmp ule float %_55.i49.i.us.i.i, 0.000000e+00, !dbg !7578
  %_3.i97.us.i252.i = fcmp oge float %_0.i.i408.us.i.i, %threshold.i22.i.i.i, !dbg !7580
  %_3.i95.us.i253.i = fcmp oge float %_0.i.i408.us.i.i, %_0.i179.i.i, !dbg !7582
  %..i96.us.i254.i = sext i1 %_3.i95.us.i253.i to i32, !dbg !7584
  %_0.i329.us.i255.i = sext i1 %_3.i97.us.i252.i to i32, !dbg !7586
  %_0.i322.us.i256.i = select i1 %_3.i121.us.i251.i, i32 %_0.i329.us.i255.i, i32 %..i96.us.i254.i, !dbg !7586
  %_0.i333.us.i.i = xor i32 %..i96.us.i254.i, -1, !dbg !7588
  %_3.i119.us.i257.i = fcmp ogt float %_67.i59.i614.us.i.i, 0.000000e+00, !dbg !7590
  %_0.i328.us.i.i = select i1 %_3.i119.us.i257.i, i32 %_0.i333.us.i.i, i32 0, !dbg !7592
  %_0.i327.us.i.i = select i1 %_3.i121.us.i251.i, i32 0, i32 %_0.i328.us.i.i, !dbg !7594
  %_0.i321.us.i.i = or i32 %_0.i327.us.i.i, %_0.i322.us.i256.i, !dbg !7596
  %_5.i298.us.i.i = and i32 %_0.i321.us.i.i, 1065353216, !dbg !7598
  %_0.i302.us.i.i = bitcast i32 %_5.i298.us.i.i to float, !dbg !7600
  %_0.i178.us.i258.i = fadd float %_67.i59.i614.us.i.i, -1.000000e+00, !dbg !7602
  %327 = trunc nsw i32 %_0.i327.us.i.i to i1, !dbg !7604
  %_4.i296.v.us.i.i = select i1 %327, float %_0.i178.us.i258.i, float %_67.i59.i614.us.i.i, !dbg !7604
  %328 = trunc nsw i32 %_0.i322.us.i256.i to i1, !dbg !7606
  %_0.i290.us.i.i = select i1 %328, float %_71.i65.i489490.i.i, float %_4.i296.v.us.i.i, !dbg !7606
  store float %_0.i290.us.i.i, ptr %270, align 4, !dbg !7608, !alias.scope !7573, !noalias !7576
  store i32 %_5.i298.us.i.i, ptr %269, align 4, !dbg !7609, !alias.scope !7573, !noalias !7576
  %_0.i176.us.i259.i = fsub float %_0.i.i408.us.i.i, %threshold.i22.i.i.i, !dbg !7610
  %_0.i162.us.i260.i = fmul float %_0.i177.i.i, %_0.i176.us.i259.i, !dbg !7612
  %_3.i.i392.inv.us.i.i = fcmp ogt float %_0.i162.us.i260.i, %272, !dbg !7614
  %_4.i.i399.v.us.i.i = select i1 %_3.i.i392.inv.us.i.i, float %_0.i162.us.i260.i, float %272, !dbg !7614
  %_3.i.i458.us.i.i = fcmp olt float %_4.i.i399.v.us.i.i, 0.000000e+00, !dbg !7617
  %329 = fcmp ule float %_0.i302.us.i.i, 0.000000e+00, !dbg !7620
  %330 = select i1 %329, i1 %_3.i.i458.us.i.i, i1 false, !dbg !7622
  %_0.i283.us.i.i = select i1 %330, float %_4.i.i399.v.us.i.i, float 0.000000e+00, !dbg !7622
  %_3.i115.us.i261.i = fcmp ule float %_0.i283.us.i.i, %_86.i78.i616.us.i.i, !dbg !7623
  %_4.i276.us.i.i = select i1 %_3.i115.us.i261.i, i32 %_88.i81.i493.i.i, i32 %_87.i80.i492.i.i, !dbg !7625
  %_0.i277.us.i.i = bitcast i32 %_4.i276.us.i.i to float, !dbg !7627
  %_0.i175.us.i262.i = fsub float %_0.i283.us.i.i, %_86.i78.i616.us.i.i, !dbg !7629
  %_4.i143.us.i.i = fmul float %_0.i175.us.i262.i, %_0.i277.us.i.i, !dbg !7631
  %_0.i144.us.i263.i = fadd float %_86.i78.i616.us.i.i, %_4.i143.us.i.i, !dbg !7631
  %331 = tail call noundef float @llvm.fabs.f32(float %_0.i144.us.i263.i), !dbg !7633
  %332 = fcmp uge float %331, 0x3BC79CA100000000, !dbg !7636
  %_0.i212.us.i.i = select i1 %332, float %_0.i144.us.i263.i, float 0.000000e+00, !dbg !7638
  store float %_0.i212.us.i.i, ptr %273, align 4, !dbg !7639, !alias.scope !7573, !noalias !7576
  %_0.i161.us.i264.i = fmul float %_0.i212.us.i.i, 0x3FC542A5A0000000, !dbg !7640
  %_3.i.i343.us.inv.i.i = fcmp ogt float %_0.i161.us.i264.i, -1.260000e+02, !dbg !7643
  %_0.i.i350.us.i.i = select i1 %_3.i.i343.us.inv.i.i, float %_0.i161.us.i264.i, float -1.260000e+02, !dbg !7643
  %_3.i.i426.us.inv.i.i = fcmp olt float %_0.i.i350.us.i.i, 1.270000e+02, !dbg !7647
  %_0.i.i433.us.i.i = select i1 %_3.i.i426.us.inv.i.i, float %_0.i.i350.us.i.i, float 1.270000e+02, !dbg !7647
  %333 = tail call noundef float @llvm.floor.f32(float %_0.i.i433.us.i.i), !dbg !7650
  %_0.i168.us.i265.i = fsub float %_0.i.i433.us.i.i, %333, !dbg !7654
  %334 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.28889.1.i.i), !dbg !7656
  %335 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.41890.1.i.i), !dbg !7660
  %_3.i.i383.us.i.i = fcmp ule float %334, %335, !dbg !7662
  %_6.i.i385.us.i.i = bitcast float %334 to i32, !dbg !7665
  %_8.i.i387.us.i.i = bitcast float %335 to i32, !dbg !7668
  %_4.i.i390.us.i.i = select i1 %_3.i.i383.us.i.i, i32 %_8.i.i387.us.i.i, i32 %_6.i.i385.us.i.i, !dbg !7670
  %_4.i262.us.i.i = select i1 %_3.i111.i.i, i32 %_6.i.i385.us.i.i, i32 %_4.i.i390.us.i.i, !dbg !7671
  %_0.i159.us.i266.i = fmul float %334, 5.000000e-01, !dbg !7673
  %_0.i158.us.i267.i = fmul float %335, 5.000000e-01, !dbg !7675
  %_0.i139.us.i268.i = fadd float %_0.i158.us.i267.i, %_0.i159.us.i266.i, !dbg !7677
  %_6.i250.us.i.i = bitcast float %_0.i139.us.i268.i to i32, !dbg !7679
  %_4.i255.us.i.i = select i1 %_3.i109.i.i, i32 %_4.i262.us.i.i, i32 %_6.i250.us.i.i, !dbg !7682
  %_0.i256.us.i.i = bitcast i32 %_4.i255.us.i.i to float, !dbg !7683
  %_3.i.i375.us.i.i = fcmp ule float %_0.i256.us.i.i, 0x3E45798EE0000000, !dbg !7685
  %_4.i.i381.us.i.i = select i1 %_3.i.i375.us.i.i, i32 841731191, i32 %_4.i255.us.i.i, !dbg !7688
  %_0.i.i382.us.i.i = bitcast i32 %_4.i.i381.us.i.i to float, !dbg !7690
  %_3.i.i335.us.i.i = fcmp ule float %_0.i.i382.us.i.i, 0x3810000000000000, !dbg !7692
  %_4.i.i341.us.i.i = select i1 %_3.i.i335.us.i.i, i32 8388608, i32 %_4.i.i381.us.i.i, !dbg !7697
  %_5.i200.us.i.i = and i32 %_4.i.i341.us.i.i, 8388607, !dbg !7699
  %_4.i201.us.i.i = or disjoint i32 %_5.i200.us.i.i, 1065353216, !dbg !7699
  %significand.i202.us.i.i = bitcast i32 %_4.i201.us.i.i to float, !dbg !7701
  %_0.i167.us.i269.i = fadd float %significand.i202.us.i.i, -1.000000e+00, !dbg !7703
  %_0.i148.us.i270.i = fmul float %_0.i167.us.i269.i, 0x3F9B17A960000000, !dbg !7705
  %336 = fsub float 0x3FBF9A8440000000, %_0.i148.us.i270.i, !dbg !7707
  %_0.i148.us.1.i271.i = fmul float %_0.i167.us.i269.i, %336, !dbg !7705
  %_0.i134.us.1.i.i = fadd float %_0.i148.us.1.i271.i, 0xBFD1E3F400000000, !dbg !7707
  %_0.i148.us.2.i272.i = fmul float %_0.i167.us.i269.i, %_0.i134.us.1.i.i, !dbg !7705
  %_0.i134.us.2.i.i = fadd float %_0.i148.us.2.i272.i, 0x3FDD544F20000000, !dbg !7707
  %_0.i148.us.3.i273.i = fmul float %_0.i167.us.i269.i, %_0.i134.us.2.i.i, !dbg !7705
  %_0.i134.us.3.i.i = fadd float %_0.i148.us.3.i273.i, 0xBFE6FC2A60000000, !dbg !7707
  %_0.i148.us.4.i.i = fmul float %_0.i167.us.i269.i, %_0.i134.us.3.i.i, !dbg !7705
  %_0.i134.us.4.i.i = fadd float %_0.i148.us.4.i.i, 0x3FF714B2A0000000, !dbg !7707
  %_9.i203.us.i.i = lshr i32 %_4.i.i341.us.i.i, 23, !dbg !7709
  %_8.i204.us.i.i = or disjoint i32 %_9.i203.us.i.i, 1258291200, !dbg !7709
  %_7.i205.us.i.i = bitcast i32 %_8.i204.us.i.i to float, !dbg !7710
  %exponent.i206.us.i.i = fadd float %_7.i205.us.i.i, 0xC160000FE0000000, !dbg !7712
  %_0.i147.us.i274.i = fmul float %_0.i167.us.i269.i, %_0.i134.us.4.i.i, !dbg !7713
  %_0.i133.us.i.i = fadd float %exponent.i206.us.i.i, %_0.i147.us.i274.i, !dbg !7715
  %_0.i157.us.i275.i = fmul float %_0.i133.us.i.i, 0x4018151820000000, !dbg !7717
  %_3.i.i450.us.inv.i.i = fcmp olt float %_0.i157.us.i275.i, 2.400000e+01, !dbg !7719
  %_0.i.i457.us.i.i = select i1 %_3.i.i450.us.inv.i.i, float %_0.i157.us.i275.i, float 2.400000e+01, !dbg !7719
  %_3.i.i367.us.inv.i.i = fcmp ogt float %_0.i.i457.us.i.i, -1.600000e+02, !dbg !7722
  %_0.i.i374.us.i.i = select i1 %_3.i.i367.us.inv.i.i, float %_0.i.i457.us.i.i, float -1.600000e+02, !dbg !7722
  %_55.i.i.us.i276.i = load float, ptr %281, align 4, !dbg !7725, !alias.scope !7726, !noalias !7729, !noundef !12
  %_3.i107.us.i.i = fcmp ule float %_55.i.i.us.i276.i, 0.000000e+00, !dbg !7731
  %_3.i93.us.i277.i = fcmp oge float %_0.i.i374.us.i.i, %threshold.i.i.i.i, !dbg !7733
  %_3.i91.us.i278.i = fcmp oge float %_0.i.i374.us.i.i, %_0.i174.i.i, !dbg !7735
  %..i92.us.i.i = sext i1 %_3.i91.us.i278.i to i32, !dbg !7737
  %_0.i325.us.i.i = sext i1 %_3.i93.us.i277.i to i32, !dbg !7739
  %_0.i319.us.i.i = select i1 %_3.i107.us.i.i, i32 %_0.i325.us.i.i, i32 %..i92.us.i.i, !dbg !7739
  %_0.i331.us.i.i = xor i32 %..i92.us.i.i, -1, !dbg !7741
  %_3.i105.us.i279.i = fcmp ogt float %_67.i.i618.us.i.i, 0.000000e+00, !dbg !7743
  %_0.i324.us.i.i = select i1 %_3.i105.us.i279.i, i32 %_0.i331.us.i.i, i32 0, !dbg !7745
  %_0.i323.us.i.i = select i1 %_3.i107.us.i.i, i32 0, i32 %_0.i324.us.i.i, !dbg !7747
  %_0.i318.us.i.i = or i32 %_0.i323.us.i.i, %_0.i319.us.i.i, !dbg !7749
  %_5.i245.us.i.i = and i32 %_0.i318.us.i.i, 1065353216, !dbg !7751
  %_0.i249.us.i280.i = bitcast i32 %_5.i245.us.i.i to float, !dbg !7753
  %_0.i173.us.i281.i = fadd float %_67.i.i618.us.i.i, -1.000000e+00, !dbg !7755
  %337 = trunc nsw i32 %_0.i323.us.i.i to i1, !dbg !7757
  %_4.i243.v.us.i.i = select i1 %337, float %_0.i173.us.i281.i, float %_67.i.i618.us.i.i, !dbg !7757
  %338 = trunc nsw i32 %_0.i319.us.i.i to i1, !dbg !7759
  %_0.i237.us.i.i = select i1 %338, float %_71.i.i506507.i.i, float %_4.i243.v.us.i.i, !dbg !7759
  store float %_0.i237.us.i.i, ptr %282, align 4, !dbg !7761, !alias.scope !7726, !noalias !7729
  store i32 %_5.i245.us.i.i, ptr %281, align 4, !dbg !7762, !alias.scope !7726, !noalias !7729
  %_0.i171.us.i282.i = fsub float %_0.i.i374.us.i.i, %threshold.i.i.i.i, !dbg !7763
  %_0.i156.us.i283.i = fmul float %_0.i172.i.i, %_0.i171.us.i282.i, !dbg !7765
  %_3.i.i359.inv.us.i.i = fcmp ogt float %_0.i156.us.i283.i, %284, !dbg !7767
  %_4.i.i365.v.us.i.i = select i1 %_3.i.i359.inv.us.i.i, float %_0.i156.us.i283.i, float %284, !dbg !7767
  %_3.i.i442.us.i.i = fcmp olt float %_4.i.i365.v.us.i.i, 0.000000e+00, !dbg !7770
  %339 = fcmp ule float %_0.i249.us.i280.i, 0.000000e+00, !dbg !7773
  %340 = select i1 %339, i1 %_3.i.i442.us.i.i, i1 false, !dbg !7775
  %_0.i230.us.i.i = select i1 %340, float %_4.i.i365.v.us.i.i, float 0.000000e+00, !dbg !7775
  %_3.i101.us.i284.i = fcmp ule float %_0.i230.us.i.i, %_86.i.i620.us.i.i, !dbg !7776
  %_4.i223.us.i.i = select i1 %_3.i101.us.i284.i, i32 %_88.i.i510.i.i, i32 %_87.i.i509.i.i, !dbg !7778
  %_0.i224.us.i285.i = bitcast i32 %_4.i223.us.i.i to float, !dbg !7780
  %_0.i170.us.i286.i = fsub float %_0.i230.us.i.i, %_86.i.i620.us.i.i, !dbg !7782
  %_4.i141.us.i.i = fmul float %_0.i170.us.i286.i, %_0.i224.us.i285.i, !dbg !7784
  %_0.i142.us.i287.i = fadd float %_86.i.i620.us.i.i, %_4.i141.us.i.i, !dbg !7784
  %341 = tail call noundef float @llvm.fabs.f32(float %_0.i142.us.i287.i), !dbg !7786
  %342 = fcmp uge float %341, 0x3BC79CA100000000, !dbg !7789
  %_0.i208.us.i.i = select i1 %342, float %_0.i142.us.i287.i, float 0.000000e+00, !dbg !7791
  store float %_0.i208.us.i.i, ptr %285, align 4, !dbg !7792, !alias.scope !7726, !noalias !7729
  %_0.i155.us.i288.i = fmul float %_0.i208.us.i.i, 0x3FC542A5A0000000, !dbg !7793
  %_3.i.i351.us.inv.i.i = fcmp ogt float %_0.i155.us.i288.i, -1.260000e+02, !dbg !7796
  %_0.i.i358.us.i.i = select i1 %_3.i.i351.us.inv.i.i, float %_0.i155.us.i288.i, float -1.260000e+02, !dbg !7796
  %_3.i.i434.us.inv.i.i = fcmp olt float %_0.i.i358.us.i.i, 1.270000e+02, !dbg !7800
  %_0.i.i441.us.i.i = select i1 %_3.i.i434.us.inv.i.i, float %_0.i.i358.us.i.i, float 1.270000e+02, !dbg !7800
  %343 = tail call noundef float @llvm.floor.f32(float %_0.i.i441.us.i.i), !dbg !7803
  %_0.i169.us.i289.i = fsub float %_0.i.i441.us.i.i, %343, !dbg !7807
  %_0.i153.us.i.i = fmul float %_0.i169.us.i289.i, 0x3F5E974FA0000000, !dbg !7809
  %_0.i138.us.i.i = fadd float %_0.i153.us.i.i, 0x3F82778560000000, !dbg !7811
  %_0.i153.us.1.i.i = fmul float %_0.i169.us.i289.i, %_0.i138.us.i.i, !dbg !7809
  %_0.i138.us.1.i.i = fadd float %_0.i153.us.1.i.i, 0x3FAC91CE60000000, !dbg !7811
  %_0.i153.us.2.i.i = fmul float %_0.i169.us.i289.i, %_0.i138.us.1.i.i, !dbg !7809
  %_0.i138.us.2.i.i = fadd float %_0.i153.us.2.i.i, 0x3FCEBDB560000000, !dbg !7811
  %_0.i153.us.3.i.i = fmul float %_0.i169.us.i289.i, %_0.i138.us.2.i.i, !dbg !7809
  %_0.i138.us.3.i.i = fadd float %_0.i153.us.3.i.i, 0x3FE62E4BA0000000, !dbg !7811
  %_0.i151.us.i.i = fmul float %_0.i168.us.i265.i, 0x3F5E974FA0000000, !dbg !7813
  %_0.i136.us.i.i = fadd float %_0.i151.us.i.i, 0x3F82778560000000, !dbg !7815
  %_0.i151.us.1.i.i = fmul float %_0.i168.us.i265.i, %_0.i136.us.i.i, !dbg !7813
  %_0.i136.us.1.i.i = fadd float %_0.i151.us.1.i.i, 0x3FAC91CE60000000, !dbg !7815
  %_0.i151.us.2.i.i = fmul float %_0.i168.us.i265.i, %_0.i136.us.1.i.i, !dbg !7813
  %_0.i136.us.2.i.i = fadd float %_0.i151.us.2.i.i, 0x3FCEBDB560000000, !dbg !7815
  %_0.i151.us.3.i.i = fmul float %_0.i168.us.i265.i, %_0.i136.us.2.i.i, !dbg !7813
  %_0.i136.us.3.i.i = fadd float %_0.i151.us.3.i.i, 0x3FE62E4BA0000000, !dbg !7815
  %_0.i150.us.i290.i = fmul float %_0.i168.us.i265.i, %_0.i136.us.3.i.i, !dbg !7817
  %_0.i135.us.i.i = fadd float %_0.i150.us.i290.i, 1.000000e+00, !dbg !7819
  %biased.i.us.i291.i = fadd float %333, 0x4160000FE0000000, !dbg !7821
  %_4.i81.us.i292.i = bitcast float %biased.i.us.i291.i to i32, !dbg !7823
  %_3.i82.us.i293.i = shl i32 %_4.i81.us.i292.i, 23, !dbg !7825
  %_0.i83.us.i294.i = bitcast i32 %_3.i82.us.i293.i to float, !dbg !7826
  %_0.i149.us.i295.i = fmul float %_0.i135.us.i.i, %_0.i83.us.i294.i, !dbg !7828
  %_3.i89.us.i296.i = fcmp une float %_0.i212.us.i.i, 0.000000e+00, !dbg !7830
  %_0.i320497.not.us.i.i = and i1 %_3.i113.i.i, %_3.i89.us.i296.i, !dbg !7832
  %_0.i160.us.i297.i = fmul float %_0.i191.us.i203.i, %_0.i149.us.i295.i, !dbg !7832
  %_4.i269.v.us.i.i = select i1 %_0.i320497.not.us.i.i, float %_0.i160.us.i297.i, float %_0.i191.us.i203.i, !dbg !7834
  %_0.i.us.i298.i = fmul float %_0.i169.us.i289.i, %_0.i138.us.3.i.i, !dbg !7836
  %_0.i137.us.i.i = fadd float %_0.i.us.i298.i, 1.000000e+00, !dbg !7838
  %biased.i84.us.i299.i = fadd float %343, 0x4160000FE0000000, !dbg !7840
  %_4.i85.us.i300.i = bitcast float %biased.i84.us.i299.i to i32, !dbg !7842
  %_3.i86.us.i301.i = shl i32 %_4.i85.us.i300.i, 23, !dbg !7844
  %_0.i87.us.i302.i = bitcast i32 %_3.i86.us.i301.i to float, !dbg !7845
  %_0.i152.us.i303.i = fmul float %_0.i137.us.i.i, %_0.i87.us.i302.i, !dbg !7847
  %_3.i88.us.i304.i = fcmp une float %_0.i208.us.i.i, 0.000000e+00, !dbg !7849
  %_0.i317514.not.us.i.i = and i1 %_3.i99.i.i, %_3.i88.us.i304.i, !dbg !7851
  %_0.i154.us.i305.i = fmul float %_0.i189.us.i209.i, %_0.i152.us.i303.i, !dbg !7851
  %_4.i216.v.us.i.i = select i1 %_0.i317514.not.us.i.i, float %_0.i154.us.i305.i, float %_0.i189.us.i209.i, !dbg !7853
  store float %_4.i269.v.us.i.i, ptr %_97.i.us.i187.i, align 4, !dbg !7855, !alias.scope !7858, !noalias !7404
  store float %_4.i216.v.us.i.i, ptr %_115.i.us.i191.i, align 4, !dbg !7861, !alias.scope !7863, !noalias !7426
  %exitcond885.not.i.i = icmp eq i64 %317, %_55.i, !dbg !7866
  br i1 %exitcond885.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i, label %bb32.i.us.i186.i, !dbg !7869, !llvm.loop !7874

bb8.i79.us.i313.i:                                ; preds = %bb5.i.preheader.us.i312.i
  %_34.i.us.i314.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.us.i217.i, !dbg !7870
  br i1 %_34.i.us.i314.i, label %bb10.i80.us.i315.i, label %panic5.i.i174.i, !dbg !7870

bb10.i80.us.i315.i:                               ; preds = %bb8.i79.us.i313.i
  %344 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.us.i213.i, !dbg !7871
  %left_own.i.us.i316.i = load float, ptr %344, align 4, !dbg !7871, !alias.scope !7479, !noalias !7494, !noundef !12
  %345 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.us.i217.i, !dbg !7870
  %right_own.i.us.i317.i = load float, ptr %345, align 4, !dbg !7870, !alias.scope !7481, !noalias !7496, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i, !dbg !7499

bb5.i.preheader.us.i312.i:                        ; preds = %bb53.i.us.i207.i
  br i1 %_31.i78.us.i218.i, label %bb8.i79.us.i313.i, label %panic4.i.i171.i, !dbg !7871

bb14.i63.preheader.us.i306.i:                     ; preds = %bb53.i.us.i207.i
  br i1 %_31.i78.us.i218.i, label %bb17.i.us.i307.i, label %panic15.i.i163.i, !dbg !7503

bb23.i.preheader.us.i219.i:                       ; preds = %bb53.i.us.i207.i
  br i1 %_31.i78.us.i218.i, label %bb27.i60.us.i220.i, label %panic28.i.i70.i, !dbg !7493

bb32.i.i.i:                                       ; preds = %bb27.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i
  %_86.i.i620.i.i = phi float [ %_0.i208.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i ], [ %.promoted619.i.i, %bb27.i ]
  %_67.i.i618.i.i = phi float [ %_0.i237.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i ], [ %.promoted617.i.i, %bb27.i ]
  %_86.i78.i616.i.i = phi float [ %_0.i212.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i ], [ %.promoted615.i.i, %bb27.i ]
  %_67.i59.i614.i.i = phi float [ %_0.i290.i.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i ], [ %.promoted.i.i, %bb27.i ]
  %iter.sroa.0.0.i613.i.i = phi i64 [ %346, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i ], [ 0, %bb27.i ]
  %346 = add nuw nsw i64 %iter.sroa.0.0.i613.i.i, 1, !dbg !7362
  %_28.i.i.i = trunc i64 %iter.sroa.0.0.i613.i.i to i32, !dbg !7376
  %now.i.i.i = add i32 %base.i.i35.i, %_28.i.i.i, !dbg !7379
  %_31.i.i.i = and i32 %now.i.i.i, %_58.i29.i, !dbg !7382
  %_30.i.i.i = zext i32 %_31.i.i.i to i64, !dbg !7384
  %_97.i.i.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %iter.sroa.0.0.i613.i.i, !dbg !7385
  %_98.not.not.i.i.i = icmp ugt i64 %_64.1.i25.i, %_30.i.i.i, !dbg !7394
  br i1 %_98.not.not.i.i.i, label %bb35.i.i.i, label %bb36.i.i45.i, !dbg !7394, !prof !2709

bb36.i.i45.i:                                     ; preds = %bb32.i.us.us.us.i42.i, %bb32.i.us.i186.i, %bb32.i.i.i
  %.us-phi622.i.i = phi i64 [ %_30.i.us.i185.i, %bb32.i.us.i186.i ], [ %_30.i.i.i, %bb32.i.i.i ], [ %_30.i.us.us.us.i40.i, %bb32.i.us.us.us.i42.i ]
  %_37.i.le610.i.i = add nuw nsw i64 %.us-phi622.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi622.i.i, i64 noundef %_37.i.le610.i.i, i64 noundef %_64.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1eeef3195352adc58f4bfdb015316fef) #24, !dbg !7875, !noalias !7359
  unreachable, !dbg !7875

bb35.i.i.i:                                       ; preds = %bb32.i.i.i
  %_0.i195.i.i = load float, ptr %_97.i.i.i, align 4, !dbg !7399, !alias.scope !7401, !noalias !7404, !noundef !12
  %_107.i.i.i = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_30.i.i.i, !dbg !7405
  store float %_0.i195.i.i, ptr %_107.i.i.i, align 4, !dbg !7409, !alias.scope !7411, !noalias !7359
  %exitcond886.not.i.i = icmp eq i64 %iter.sroa.0.0.i613.i.i, %_63.i, !dbg !7876
  br i1 %exitcond886.not.i.i, label %bb39.i.i.i, label %bb38.i.i.i, !dbg !7876, !prof !180

bb39.i.i.i:                                       ; preds = %bb35.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_63.i, i64 noundef %346, i64 noundef range(i64 0, 2305843009213693952) %_63.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d9529ff5ddc99dd60299cff5ff3cd676) #24, !dbg !7877, !noalias !7359
  unreachable, !dbg !7877

bb38.i.i.i:                                       ; preds = %bb35.i.i.i
  %_115.i.i.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %iter.sroa.0.0.i613.i.i, !dbg !7414
  %_116.not.not.i.i.i = icmp ugt i64 %_66.1.i27.i, %_30.i.i.i, !dbg !7872
  br i1 %_116.not.not.i.i.i, label %bb40.i.i.i, label %bb41.i.i193.i, !dbg !7872, !prof !2709

bb41.i.i193.i:                                    ; preds = %bb35.i.us.i189.i, %bb38.i.i.i
  %.us-phi624.i.i = phi i64 [ %_30.i.i.i, %bb38.i.i.i ], [ %_30.i.us.i185.i, %bb35.i.us.i189.i ]
  %_37.i.le.i194.i = add nuw nsw i64 %.us-phi624.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi624.i.i, i64 noundef %_37.i.le.i194.i, i64 noundef %_66.1.i27.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9d56679ca30f2caa500ddf2e89d00cb) #24, !dbg !7878, !noalias !7359
  unreachable, !dbg !7878

bb40.i.i.i:                                       ; preds = %bb38.i.i.i
  %_0.i193.i.i = load float, ptr %_115.i.i.i, align 4, !dbg !7421, !alias.scope !7423, !noalias !7426, !noundef !12
  %_123.i.i.i = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_30.i.i.i, !dbg !7427
  store float %_0.i193.i.i, ptr %_123.i.i.i, align 4, !dbg !7434, !alias.scope !7436, !noalias !7359
  %_53.i.i.i = sub i32 %now.i.i.i, %_59.i30.i, !dbg !7439
  %_52.i.i.i = and i32 %_53.i.i.i, %_58.i29.i, !dbg !7442
  %_51.i.i.i = zext i32 %_52.i.i.i to i64, !dbg !7443
  %_156.not.not.i.i.i = icmp ugt i64 %_64.1.i25.i, %_51.i.i.i, !dbg !7444
  br i1 %_156.not.not.i.i.i, label %bb50.i.i.i, label %bb51.i.i54.i, !dbg !7444, !prof !2709

bb51.i.i54.i:                                     ; preds = %bb35.i.us.us.us.i46.i, %bb40.i.us.i195.i, %bb40.i.i.i
  %.us-phi626.i.i = phi i64 [ %_51.i.us.i199.i, %bb40.i.us.i195.i ], [ %_51.i.i.i, %bb40.i.i.i ], [ %_51.i.us.us.us.i52.i, %bb35.i.us.us.us.i46.i ]
  %_56.i.le608.i.i = add nuw nsw i64 %.us-phi626.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi626.i.i, i64 noundef %_56.i.le608.i.i, i64 noundef %_64.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd47fa4d4a56fb8b8bbd8b0c2f635fa7) #24, !dbg !7879, !noalias !7359
  unreachable, !dbg !7879

bb50.i.i.i:                                       ; preds = %bb40.i.i.i
  %_163.i.i.i = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_51.i.i.i, !dbg !7449
  %_0.i191.i.i = load float, ptr %_163.i.i.i, align 4, !dbg !7453, !alias.scope !7455, !noalias !7359, !noundef !12
  %_164.not.not.i.i.i = icmp ugt i64 %_66.1.i27.i, %_51.i.i.i, !dbg !7873
  br i1 %_164.not.not.i.i.i, label %bb53.i.i.i, label %bb54.i.i205.i, !dbg !7873, !prof !2709

bb54.i.i205.i:                                    ; preds = %bb50.i.us.i201.i, %bb50.i.i.i
  %.us-phi628.i.i = phi i64 [ %_51.i.i.i, %bb50.i.i.i ], [ %_51.i.us.i199.i, %bb50.i.us.i201.i ]
  %_56.i.le.i206.i = add nuw nsw i64 %.us-phi628.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi628.i.i, i64 noundef %_56.i.le.i206.i, i64 noundef %_66.1.i27.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f198228fd40f4c04a48a745576c5c5ee) #24, !dbg !7880, !noalias !7359
  unreachable, !dbg !7880

bb53.i.i.i:                                       ; preds = %bb50.i.i.i
  %_169.i.i.i = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_51.i.i.i, !dbg !7458
  %_0.i189.i.i = load float, ptr %_169.i.i.i, align 4, !dbg !7466, !alias.scope !7468, !noalias !7359, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7471), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7477), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7479), !dbg !7474
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7481), !dbg !7474
  %_18.i72.i.i = load i32, ptr %_31.i17.i, align 4, !dbg !7483, !alias.scope !7485, !noalias !7486, !noundef !12
  %_17.i.i319.i = sub i32 %now.i.i.i, %_18.i72.i.i, !dbg !7488
  %_16.i73.i.i = and i32 %_17.i.i319.i, %_58.i29.i, !dbg !7483
  %_15.i74.i.i = zext i32 %_16.i73.i.i to i64, !dbg !7483
  %_26.i.i.i = load i32, ptr %_33.i18.i, align 4, !dbg !7483, !alias.scope !7490, !noalias !7491, !noundef !12
  %_25.i.i.i = sub i32 %now.i.i.i, %_26.i.i.i, !dbg !7488
  %_24.i.i.i = and i32 %_25.i.i.i, %_58.i29.i, !dbg !7483
  %_23.i77.i.i = zext i32 %_24.i.i.i to i64, !dbg !7483
  %_31.i78.i.i = icmp samesign ugt i64 %_64.1.i25.i, %_15.i74.i.i, !dbg !7483
  switch i8 %_0.sroa.0.0.i479.i.i, label %default.unreachable [
    i8 0, label %bb7.i76.i.i
    i8 1, label %bb16.i.i.i
    i8 2, label %bb25.i.i.i
  ], !dbg !7492

bb7.i76.i.i:                                      ; preds = %bb53.i.i.i
  br i1 %_31.i78.i.i, label %bb8.i79.i.i, label %panic4.i.i171.i, !dbg !7871

bb8.i79.i.i:                                      ; preds = %bb7.i76.i.i
  %_34.i.i.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.i.i, !dbg !7870
  br i1 %_34.i.i.i, label %bb10.i80.i.i, label %panic5.i.i174.i, !dbg !7870

panic4.i.i171.i:                                  ; preds = %bb5.i.preheader.us.us.us.i170.i, %bb5.i.preheader.us.i312.i, %bb7.i76.i.i
  %.us-phi636.i.i = phi i64 [ %_15.i74.us.i213.i, %bb5.i.preheader.us.i312.i ], [ %_15.i74.i.i, %bb7.i76.i.i ], [ %_15.i74.us.us.us.i63.i, %bb5.i.preheader.us.us.us.i170.i ], !dbg !7871
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi636.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cba26bb3a04aaba20f9c249edc5e133d) #24, !dbg !7871, !noalias !7881
  unreachable, !dbg !7871

panic5.i.i174.i:                                  ; preds = %bb8.i79.us.us.us.i172.i, %bb8.i79.us.i313.i, %bb8.i79.i.i
  %.us-phi637.i.i = phi i64 [ %_23.i77.us.i217.i, %bb8.i79.us.i313.i ], [ %_23.i77.i.i, %bb8.i79.i.i ], [ %_23.i77.us.us.us.i67.i, %bb8.i79.us.us.us.i172.i ], !dbg !7870
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi637.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i27.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2ad2d313de59c36d62f4a7d75017a71f) #24, !dbg !7870, !noalias !7881
  unreachable, !dbg !7870

bb10.i80.i.i:                                     ; preds = %bb8.i79.i.i
  %347 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.i.i, !dbg !7871
  %left_own.i.i.i = load float, ptr %347, align 4, !dbg !7871, !alias.scope !7479, !noalias !7494, !noundef !12
  %348 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.i.i, !dbg !7870
  %right_own.i.i.i = load float, ptr %348, align 4, !dbg !7870, !alias.scope !7481, !noalias !7496, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i, !dbg !7499

bb16.i.i.i:                                       ; preds = %bb53.i.i.i
  br i1 %_31.i78.i.i, label %bb17.i.i.i, label %panic15.i.i163.i, !dbg !7503

bb17.i.i.i:                                       ; preds = %bb16.i.i.i
  %_59.i.i.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.i.i, !dbg !7502
  br i1 %_59.i.i.i, label %bb19.i.i.i, label %panic17.i.i166.i, !dbg !7502

panic15.i.i163.i:                                 ; preds = %bb14.i63.preheader.us.us.us.i162.i, %bb14.i63.preheader.us.i306.i, %bb16.i.i.i
  %.us-phi634.i.i = phi i64 [ %_15.i74.us.i213.i, %bb14.i63.preheader.us.i306.i ], [ %_15.i74.i.i, %bb16.i.i.i ], [ %_15.i74.us.us.us.i63.i, %bb14.i63.preheader.us.us.us.i162.i ], !dbg !7503
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi634.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e2e01e5f964095ee8e5aa091c7d92b88) #24, !dbg !7503, !noalias !7881
  unreachable, !dbg !7503

panic17.i.i166.i:                                 ; preds = %bb17.i.us.us.us.i164.i, %bb17.i.us.i307.i, %bb17.i.i.i
  %.us-phi635.i.i = phi i64 [ %_23.i77.us.i217.i, %bb17.i.us.i307.i ], [ %_23.i77.i.i, %bb17.i.i.i ], [ %_23.i77.us.us.us.i67.i, %bb17.i.us.us.us.i164.i ], !dbg !7502
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi635.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i27.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_48dca199019b49040caac979cf1ddc5a) #24, !dbg !7502, !noalias !7881
  unreachable, !dbg !7502

bb19.i.i.i:                                       ; preds = %bb17.i.i.i
  %349 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.i.i, !dbg !7503
  %left_own16.i.i.i = load float, ptr %349, align 4, !dbg !7503, !alias.scope !7479, !noalias !7494, !noundef !12
  %350 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.i.i, !dbg !7502
  %right_own18.i.i.i = load float, ptr %350, align 4, !dbg !7502, !alias.scope !7481, !noalias !7496, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i, !dbg !7499

bb25.i.i.i:                                       ; preds = %bb53.i.i.i
  br i1 %_31.i78.i.i, label %bb27.i60.i.i, label %panic28.i.i70.i, !dbg !7493

panic28.i.i70.i:                                  ; preds = %bb23.i.preheader.us.us.us.i69.i, %bb23.i.preheader.us.i219.i, %bb25.i.i.i
  %.us-phi630.i.i = phi i64 [ %_15.i74.us.i213.i, %bb23.i.preheader.us.i219.i ], [ %_15.i74.i.i, %bb25.i.i.i ], [ %_15.i74.us.us.us.i63.i, %bb23.i.preheader.us.us.us.i69.i ], !dbg !7493
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi630.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4b7b2eb7fa1b19ad0451b09b71313122) #24, !dbg !7493, !noalias !7881
  unreachable, !dbg !7493

bb27.i60.i.i:                                     ; preds = %bb25.i.i.i
  %351 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_15.i74.i.i, !dbg !7493
  %_79.i.i.i = load float, ptr %351, align 4, !dbg !7493, !alias.scope !7479, !noalias !7494, !noundef !12
  %_85.i.i.i = icmp samesign ugt i64 %_66.1.i27.i, %_15.i74.i.i, !dbg !7495
  br i1 %_85.i.i.i, label %bb29.i61.i.i, label %panic30.i.i74.i, !dbg !7495

panic30.i.i74.i:                                  ; preds = %bb27.i60.us.i220.i, %bb27.i60.i.i
  %.us-phi631.i.i = phi i64 [ %_15.i74.i.i, %bb27.i60.i.i ], [ %_15.i74.us.i213.i, %bb27.i60.us.i220.i ], !dbg !7495
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi631.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i27.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60256fc2b51ee5bb69caa4304418caff) #24, !dbg !7495, !noalias !7881
  unreachable, !dbg !7495

bb29.i61.i.i:                                     ; preds = %bb27.i60.i.i
  %352 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_15.i74.i.i, !dbg !7495
  %_83.i.i.i = load float, ptr %352, align 4, !dbg !7495, !alias.scope !7481, !noalias !7496, !noundef !12
  %_87.i.i.i = icmp samesign ugt i64 %_66.1.i27.i, %_23.i77.i.i, !dbg !7497
  br i1 %_87.i.i.i, label %bb31.i.i.i, label %panic32.i.i78.i, !dbg !7497

panic32.i.i78.i:                                  ; preds = %bb27.i60.us.us.us.i71.i, %bb29.i61.us.i223.i, %bb29.i61.i.i
  %.us-phi632.i.i = phi i64 [ %_23.i77.us.i217.i, %bb29.i61.us.i223.i ], [ %_23.i77.i.i, %bb29.i61.i.i ], [ %_23.i77.us.us.us.i67.i, %bb27.i60.us.us.us.i71.i ], !dbg !7497
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi632.i.i, i64 noundef range(i64 1, 2305843009213693952) %_66.1.i27.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_03bcac171bcf0d517141d8cd3ac9ded0) #24, !dbg !7497, !noalias !7881
  unreachable, !dbg !7497

bb31.i.i.i:                                       ; preds = %bb29.i61.i.i
  %_89.i.i.i = icmp samesign ugt i64 %_64.1.i25.i, %_23.i77.i.i, !dbg !7498
  br i1 %_89.i.i.i, label %bb33.i62.i.i, label %panic34.i.i81.i, !dbg !7498

panic34.i.i81.i:                                  ; preds = %bb31.i.us.us.us.i79.i, %bb31.i.i.i
  %.us-phi633.i.i = phi i64 [ %_23.i77.i.i, %bb31.i.i.i ], [ %_23.i77.us.us.us.i67.i, %bb31.i.us.us.us.i79.i ], !dbg !7498
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi633.i.i, i64 noundef range(i64 1, 2305843009213693952) %_64.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ed84bd2438b7b7e3dfb2147bac09e923) #24, !dbg !7498, !noalias !7881
  unreachable, !dbg !7498

bb33.i62.i.i:                                     ; preds = %bb31.i.i.i
  %353 = getelementptr inbounds nuw float, ptr %_66.0.i26.i, i64 %_23.i77.i.i, !dbg !7497
  %_86.i.i.i = load float, ptr %353, align 4, !dbg !7497, !alias.scope !7481, !noalias !7496, !noundef !12
  %354 = getelementptr inbounds nuw float, ptr %_64.0.i24.i, i64 %_23.i77.i.i, !dbg !7498
  %_88.i.i.i = load float, ptr %354, align 4, !dbg !7498, !alias.scope !7479, !noalias !7494, !noundef !12
  br label %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i, !dbg !7499

_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i: ; preds = %bb33.i62.i.i, %bb19.i.i.i, %bb10.i80.i.i
  %taps.i.sroa.41890.0.i.i = phi float [ %right_own.i.i.i, %bb10.i80.i.i ], [ %left_own16.i.i.i, %bb19.i.i.i ], [ %_88.i.i.i, %bb33.i62.i.i ], !dbg !7483
  %taps.i.sroa.28889.0.i.i = phi float [ %right_own.i.i.i, %bb10.i80.i.i ], [ %right_own18.i.i.i, %bb19.i.i.i ], [ %_86.i.i.i, %bb33.i62.i.i ], !dbg !7483
  %taps.i.sroa.15888.0.i.i = phi float [ %left_own.i.i.i, %bb10.i80.i.i ], [ %right_own18.i.i.i, %bb19.i.i.i ], [ %_83.i.i.i, %bb33.i62.i.i ], !dbg !7483
  %taps.i.sroa.0.0.i.i = phi float [ %left_own.i.i.i, %bb10.i80.i.i ], [ %left_own16.i.i.i, %bb19.i.i.i ], [ %_79.i.i.i, %bb33.i62.i.i ], !dbg !7483
  %355 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.0.0.i.i), !dbg !7504
  %356 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.15888.0.i.i), !dbg !7507
  %_3.i.i417.i.i = fcmp ule float %355, %356, !dbg !7509
  %_6.i.i419.i.i = bitcast float %355 to i32, !dbg !7512
  %_8.i.i421.i.i = bitcast float %356 to i32, !dbg !7515
  %_4.i.i424.i.i = select i1 %_3.i.i417.i.i, i32 %_8.i.i421.i.i, i32 %_6.i.i419.i.i, !dbg !7517
  %_4.i315.i.i = select i1 %_3.i125.i.i, i32 %_6.i.i419.i.i, i32 %_4.i.i424.i.i, !dbg !7518
  %_0.i165.i.i = fmul float %355, 5.000000e-01, !dbg !7520
  %_0.i164.i.i = fmul float %356, 5.000000e-01, !dbg !7522
  %_0.i140.i.i = fadd float %_0.i164.i.i, %_0.i165.i.i, !dbg !7524
  %_6.i303.i.i = bitcast float %_0.i140.i.i to i32, !dbg !7526
  %_4.i308.i.i = select i1 %_3.i123.i.i, i32 %_4.i315.i.i, i32 %_6.i303.i.i, !dbg !7529
  %_0.i309.i.i = bitcast i32 %_4.i308.i.i to float, !dbg !7530
  %_3.i.i409.i.i = fcmp ule float %_0.i309.i.i, 0x3E45798EE0000000, !dbg !7532
  %_4.i.i415.i.i = select i1 %_3.i.i409.i.i, i32 841731191, i32 %_4.i308.i.i, !dbg !7535
  %_0.i.i416.i.i = bitcast i32 %_4.i.i415.i.i to float, !dbg !7537
  %_3.i.i.i320.i = fcmp ule float %_0.i.i416.i.i, 0x3810000000000000, !dbg !7539
  %_4.i.i.i.i = select i1 %_3.i.i.i320.i, i32 8388608, i32 %_4.i.i415.i.i, !dbg !7544
  %_5.i196.i.i = and i32 %_4.i.i.i.i, 8388607, !dbg !7546
  %_4.i197.i.i = or disjoint i32 %_5.i196.i.i, 1065353216, !dbg !7546
  %significand.i.i.i = bitcast i32 %_4.i197.i.i to float, !dbg !7548
  %_0.i166.i.i = fadd float %significand.i.i.i, -1.000000e+00, !dbg !7550
  %_0.i146.i.i = fmul float %_0.i166.i.i, 0x3F9B17A960000000, !dbg !7552
  %357 = fsub float 0x3FBF9A8440000000, %_0.i146.i.i, !dbg !7554
  %_0.i146.1.i.i = fmul float %_0.i166.i.i, %357, !dbg !7552
  %_0.i132.1.i.i = fadd float %_0.i146.1.i.i, 0xBFD1E3F400000000, !dbg !7554
  %_0.i146.2.i.i = fmul float %_0.i166.i.i, %_0.i132.1.i.i, !dbg !7552
  %_0.i132.2.i.i = fadd float %_0.i146.2.i.i, 0x3FDD544F20000000, !dbg !7554
  %_0.i146.3.i.i = fmul float %_0.i166.i.i, %_0.i132.2.i.i, !dbg !7552
  %_0.i132.3.i.i = fadd float %_0.i146.3.i.i, 0xBFE6FC2A60000000, !dbg !7554
  %_0.i146.4.i.i = fmul float %_0.i166.i.i, %_0.i132.3.i.i, !dbg !7552
  %_0.i132.4.i.i = fadd float %_0.i146.4.i.i, 0x3FF714B2A0000000, !dbg !7554
  %_9.i.i.i = lshr i32 %_4.i.i.i.i, 23, !dbg !7556
  %_8.i198.i.i = or disjoint i32 %_9.i.i.i, 1258291200, !dbg !7556
  %_7.i.i.i = bitcast i32 %_8.i198.i.i to float, !dbg !7557
  %exponent.i.i.i = fadd float %_7.i.i.i, 0xC160000FE0000000, !dbg !7559
  %_0.i145.i.i = fmul float %_0.i166.i.i, %_0.i132.4.i.i, !dbg !7560
  %_0.i131.i.i = fadd float %exponent.i.i.i, %_0.i145.i.i, !dbg !7562
  %_0.i163.i.i = fmul float %_0.i131.i.i, 0x4018151820000000, !dbg !7564
  %_3.i.i466.inv.i.i = fcmp olt float %_0.i163.i.i, 2.400000e+01, !dbg !7566
  %_0.i.i473.i.i = select i1 %_3.i.i466.inv.i.i, float %_0.i163.i.i, float 2.400000e+01, !dbg !7566
  %_3.i.i401.inv.i.i = fcmp ogt float %_0.i.i473.i.i, -1.600000e+02, !dbg !7569
  %_0.i.i408.i.i = select i1 %_3.i.i401.inv.i.i, float %_0.i.i473.i.i, float -1.600000e+02, !dbg !7569
  %_55.i49.i.i.i = load float, ptr %269, align 4, !dbg !7572, !alias.scope !7573, !noalias !7576, !noundef !12
  %_3.i121.i.i = fcmp ule float %_55.i49.i.i.i, 0.000000e+00, !dbg !7578
  %_3.i97.i.i = fcmp oge float %_0.i.i408.i.i, %threshold.i22.i.i.i, !dbg !7580
  %_3.i95.i.i = fcmp oge float %_0.i.i408.i.i, %_0.i179.i.i, !dbg !7582
  %..i96.i.i = sext i1 %_3.i95.i.i to i32, !dbg !7584
  %_0.i329.i.i = sext i1 %_3.i97.i.i to i32, !dbg !7586
  %_0.i322.i.i = select i1 %_3.i121.i.i, i32 %_0.i329.i.i, i32 %..i96.i.i, !dbg !7586
  %_0.i333.i.i = xor i32 %..i96.i.i, -1, !dbg !7588
  %_3.i119.i.i = fcmp ogt float %_67.i59.i614.i.i, 0.000000e+00, !dbg !7590
  %_0.i328.i.i = select i1 %_3.i119.i.i, i32 %_0.i333.i.i, i32 0, !dbg !7592
  %_0.i327.i.i = select i1 %_3.i121.i.i, i32 0, i32 %_0.i328.i.i, !dbg !7594
  %_0.i321.i.i = or i32 %_0.i327.i.i, %_0.i322.i.i, !dbg !7596
  %_5.i298.i.i = and i32 %_0.i321.i.i, 1065353216, !dbg !7598
  %_0.i302.i.i = bitcast i32 %_5.i298.i.i to float, !dbg !7600
  %_0.i178.i.i = fadd float %_67.i59.i614.i.i, -1.000000e+00, !dbg !7602
  %358 = trunc nsw i32 %_0.i327.i.i to i1, !dbg !7604
  %_4.i296.v.i.i = select i1 %358, float %_0.i178.i.i, float %_67.i59.i614.i.i, !dbg !7604
  %359 = trunc nsw i32 %_0.i322.i.i to i1, !dbg !7606
  %_0.i290.i.i = select i1 %359, float %_71.i65.i489490.i.i, float %_4.i296.v.i.i, !dbg !7606
  store float %_0.i290.i.i, ptr %270, align 4, !dbg !7608, !alias.scope !7573, !noalias !7576
  store i32 %_5.i298.i.i, ptr %269, align 4, !dbg !7609, !alias.scope !7573, !noalias !7576
  %_0.i176.i.i = fsub float %_0.i.i408.i.i, %threshold.i22.i.i.i, !dbg !7610
  %_0.i162.i.i = fmul float %_0.i177.i.i, %_0.i176.i.i, !dbg !7612
  %_3.i.i392.inv.i.i = fcmp ogt float %_0.i162.i.i, %272, !dbg !7614
  %_4.i.i399.v.i.i = select i1 %_3.i.i392.inv.i.i, float %_0.i162.i.i, float %272, !dbg !7614
  %_3.i.i458.i.i = fcmp olt float %_4.i.i399.v.i.i, 0.000000e+00, !dbg !7617
  %360 = fcmp ule float %_0.i302.i.i, 0.000000e+00, !dbg !7620
  %361 = select i1 %360, i1 %_3.i.i458.i.i, i1 false, !dbg !7622
  %_0.i283.i.i = select i1 %361, float %_4.i.i399.v.i.i, float 0.000000e+00, !dbg !7622
  %_3.i115.i.i = fcmp ule float %_0.i283.i.i, %_86.i78.i616.i.i, !dbg !7623
  %_4.i276.i.i = select i1 %_3.i115.i.i, i32 %_88.i81.i493.i.i, i32 %_87.i80.i492.i.i, !dbg !7625
  %_0.i277.i.i = bitcast i32 %_4.i276.i.i to float, !dbg !7627
  %_0.i175.i.i = fsub float %_0.i283.i.i, %_86.i78.i616.i.i, !dbg !7629
  %_4.i143.i.i = fmul float %_0.i175.i.i, %_0.i277.i.i, !dbg !7631
  %_0.i144.i.i = fadd float %_86.i78.i616.i.i, %_4.i143.i.i, !dbg !7631
  %362 = tail call noundef float @llvm.fabs.f32(float %_0.i144.i.i), !dbg !7633
  %363 = fcmp uge float %362, 0x3BC79CA100000000, !dbg !7636
  %_0.i212.i.i = select i1 %363, float %_0.i144.i.i, float 0.000000e+00, !dbg !7638
  store float %_0.i212.i.i, ptr %273, align 4, !dbg !7639, !alias.scope !7573, !noalias !7576
  %_0.i161.i.i = fmul float %_0.i212.i.i, 0x3FC542A5A0000000, !dbg !7640
  %_3.i.i343.inv.i.i = fcmp ogt float %_0.i161.i.i, -1.260000e+02, !dbg !7643
  %_0.i.i350.i.i = select i1 %_3.i.i343.inv.i.i, float %_0.i161.i.i, float -1.260000e+02, !dbg !7643
  %_3.i.i426.inv.i.i = fcmp olt float %_0.i.i350.i.i, 1.270000e+02, !dbg !7647
  %_0.i.i433.i.i = select i1 %_3.i.i426.inv.i.i, float %_0.i.i350.i.i, float 1.270000e+02, !dbg !7647
  %364 = tail call noundef float @llvm.floor.f32(float %_0.i.i433.i.i), !dbg !7650
  %_0.i168.i.i = fsub float %_0.i.i433.i.i, %364, !dbg !7654
  %365 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.28889.0.i.i), !dbg !7656
  %366 = tail call noundef float @llvm.fabs.f32(float %taps.i.sroa.41890.0.i.i), !dbg !7660
  %_3.i.i383.i.i = fcmp ule float %365, %366, !dbg !7662
  %_6.i.i385.i.i = bitcast float %365 to i32, !dbg !7665
  %_8.i.i387.i.i = bitcast float %366 to i32, !dbg !7668
  %_4.i.i390.i.i = select i1 %_3.i.i383.i.i, i32 %_8.i.i387.i.i, i32 %_6.i.i385.i.i, !dbg !7670
  %_4.i262.i.i = select i1 %_3.i111.i.i, i32 %_6.i.i385.i.i, i32 %_4.i.i390.i.i, !dbg !7671
  %_0.i159.i.i = fmul float %365, 5.000000e-01, !dbg !7673
  %_0.i158.i.i = fmul float %366, 5.000000e-01, !dbg !7675
  %_0.i139.i.i = fadd float %_0.i158.i.i, %_0.i159.i.i, !dbg !7677
  %_6.i250.i.i = bitcast float %_0.i139.i.i to i32, !dbg !7679
  %_4.i255.i.i = select i1 %_3.i109.i.i, i32 %_4.i262.i.i, i32 %_6.i250.i.i, !dbg !7682
  %_0.i256.i.i = bitcast i32 %_4.i255.i.i to float, !dbg !7683
  %_3.i.i375.i.i = fcmp ule float %_0.i256.i.i, 0x3E45798EE0000000, !dbg !7685
  %_4.i.i381.i.i = select i1 %_3.i.i375.i.i, i32 841731191, i32 %_4.i255.i.i, !dbg !7688
  %_0.i.i382.i.i = bitcast i32 %_4.i.i381.i.i to float, !dbg !7690
  %_3.i.i335.i.i = fcmp ule float %_0.i.i382.i.i, 0x3810000000000000, !dbg !7692
  %_4.i.i341.i.i = select i1 %_3.i.i335.i.i, i32 8388608, i32 %_4.i.i381.i.i, !dbg !7697
  %_5.i200.i.i = and i32 %_4.i.i341.i.i, 8388607, !dbg !7699
  %_4.i201.i.i = or disjoint i32 %_5.i200.i.i, 1065353216, !dbg !7699
  %significand.i202.i.i = bitcast i32 %_4.i201.i.i to float, !dbg !7701
  %_0.i167.i.i = fadd float %significand.i202.i.i, -1.000000e+00, !dbg !7703
  %_0.i148.i.i = fmul float %_0.i167.i.i, 0x3F9B17A960000000, !dbg !7705
  %367 = fsub float 0x3FBF9A8440000000, %_0.i148.i.i, !dbg !7707
  %_0.i148.1.i.i = fmul float %_0.i167.i.i, %367, !dbg !7705
  %_0.i134.1.i.i = fadd float %_0.i148.1.i.i, 0xBFD1E3F400000000, !dbg !7707
  %_0.i148.2.i.i = fmul float %_0.i167.i.i, %_0.i134.1.i.i, !dbg !7705
  %_0.i134.2.i.i = fadd float %_0.i148.2.i.i, 0x3FDD544F20000000, !dbg !7707
  %_0.i148.3.i.i = fmul float %_0.i167.i.i, %_0.i134.2.i.i, !dbg !7705
  %_0.i134.3.i.i = fadd float %_0.i148.3.i.i, 0xBFE6FC2A60000000, !dbg !7707
  %_0.i148.4.i.i = fmul float %_0.i167.i.i, %_0.i134.3.i.i, !dbg !7705
  %_0.i134.4.i.i = fadd float %_0.i148.4.i.i, 0x3FF714B2A0000000, !dbg !7707
  %_9.i203.i.i = lshr i32 %_4.i.i341.i.i, 23, !dbg !7709
  %_8.i204.i.i = or disjoint i32 %_9.i203.i.i, 1258291200, !dbg !7709
  %_7.i205.i.i = bitcast i32 %_8.i204.i.i to float, !dbg !7710
  %exponent.i206.i.i = fadd float %_7.i205.i.i, 0xC160000FE0000000, !dbg !7712
  %_0.i147.i.i = fmul float %_0.i167.i.i, %_0.i134.4.i.i, !dbg !7713
  %_0.i133.i.i = fadd float %exponent.i206.i.i, %_0.i147.i.i, !dbg !7715
  %_0.i157.i.i = fmul float %_0.i133.i.i, 0x4018151820000000, !dbg !7717
  %_3.i.i450.inv.i.i = fcmp olt float %_0.i157.i.i, 2.400000e+01, !dbg !7719
  %_0.i.i457.i.i = select i1 %_3.i.i450.inv.i.i, float %_0.i157.i.i, float 2.400000e+01, !dbg !7719
  %_3.i.i367.inv.i.i = fcmp ogt float %_0.i.i457.i.i, -1.600000e+02, !dbg !7722
  %_0.i.i374.i.i = select i1 %_3.i.i367.inv.i.i, float %_0.i.i457.i.i, float -1.600000e+02, !dbg !7722
  %_55.i.i.i.i = load float, ptr %281, align 4, !dbg !7725, !alias.scope !7726, !noalias !7729, !noundef !12
  %_3.i107.i.i = fcmp ule float %_55.i.i.i.i, 0.000000e+00, !dbg !7731
  %_3.i93.i.i = fcmp oge float %_0.i.i374.i.i, %threshold.i.i.i.i, !dbg !7733
  %_3.i91.i.i = fcmp oge float %_0.i.i374.i.i, %_0.i174.i.i, !dbg !7735
  %..i92.i.i = sext i1 %_3.i91.i.i to i32, !dbg !7737
  %_0.i325.i.i = sext i1 %_3.i93.i.i to i32, !dbg !7739
  %_0.i319.i.i = select i1 %_3.i107.i.i, i32 %_0.i325.i.i, i32 %..i92.i.i, !dbg !7739
  %_0.i331.i.i = xor i32 %..i92.i.i, -1, !dbg !7741
  %_3.i105.i.i = fcmp ogt float %_67.i.i618.i.i, 0.000000e+00, !dbg !7743
  %_0.i324.i.i = select i1 %_3.i105.i.i, i32 %_0.i331.i.i, i32 0, !dbg !7745
  %_0.i323.i.i = select i1 %_3.i107.i.i, i32 0, i32 %_0.i324.i.i, !dbg !7747
  %_0.i318.i.i = or i32 %_0.i323.i.i, %_0.i319.i.i, !dbg !7749
  %_5.i245.i.i = and i32 %_0.i318.i.i, 1065353216, !dbg !7751
  %_0.i249.i.i = bitcast i32 %_5.i245.i.i to float, !dbg !7753
  %_0.i173.i.i = fadd float %_67.i.i618.i.i, -1.000000e+00, !dbg !7755
  %368 = trunc nsw i32 %_0.i323.i.i to i1, !dbg !7757
  %_4.i243.v.i.i = select i1 %368, float %_0.i173.i.i, float %_67.i.i618.i.i, !dbg !7757
  %369 = trunc nsw i32 %_0.i319.i.i to i1, !dbg !7759
  %_0.i237.i.i = select i1 %369, float %_71.i.i506507.i.i, float %_4.i243.v.i.i, !dbg !7759
  store float %_0.i237.i.i, ptr %282, align 4, !dbg !7761, !alias.scope !7726, !noalias !7729
  store i32 %_5.i245.i.i, ptr %281, align 4, !dbg !7762, !alias.scope !7726, !noalias !7729
  %_0.i171.i.i = fsub float %_0.i.i374.i.i, %threshold.i.i.i.i, !dbg !7763
  %_0.i156.i.i = fmul float %_0.i172.i.i, %_0.i171.i.i, !dbg !7765
  %_3.i.i359.inv.i.i = fcmp ogt float %_0.i156.i.i, %284, !dbg !7767
  %_4.i.i365.v.i.i = select i1 %_3.i.i359.inv.i.i, float %_0.i156.i.i, float %284, !dbg !7767
  %_3.i.i442.i.i = fcmp olt float %_4.i.i365.v.i.i, 0.000000e+00, !dbg !7770
  %370 = fcmp ule float %_0.i249.i.i, 0.000000e+00, !dbg !7773
  %371 = select i1 %370, i1 %_3.i.i442.i.i, i1 false, !dbg !7775
  %_0.i230.i.i = select i1 %371, float %_4.i.i365.v.i.i, float 0.000000e+00, !dbg !7775
  %_3.i101.i.i = fcmp ule float %_0.i230.i.i, %_86.i.i620.i.i, !dbg !7776
  %_4.i223.i.i = select i1 %_3.i101.i.i, i32 %_88.i.i510.i.i, i32 %_87.i.i509.i.i, !dbg !7778
  %_0.i224.i.i = bitcast i32 %_4.i223.i.i to float, !dbg !7780
  %_0.i170.i.i = fsub float %_0.i230.i.i, %_86.i.i620.i.i, !dbg !7782
  %_4.i141.i.i = fmul float %_0.i170.i.i, %_0.i224.i.i, !dbg !7784
  %_0.i142.i.i = fadd float %_86.i.i620.i.i, %_4.i141.i.i, !dbg !7784
  %372 = tail call noundef float @llvm.fabs.f32(float %_0.i142.i.i), !dbg !7786
  %373 = fcmp uge float %372, 0x3BC79CA100000000, !dbg !7789
  %_0.i208.i.i = select i1 %373, float %_0.i142.i.i, float 0.000000e+00, !dbg !7791
  store float %_0.i208.i.i, ptr %285, align 4, !dbg !7792, !alias.scope !7726, !noalias !7729
  %_0.i155.i.i = fmul float %_0.i208.i.i, 0x3FC542A5A0000000, !dbg !7793
  %_3.i.i351.inv.i.i = fcmp ogt float %_0.i155.i.i, -1.260000e+02, !dbg !7796
  %_0.i.i358.i.i = select i1 %_3.i.i351.inv.i.i, float %_0.i155.i.i, float -1.260000e+02, !dbg !7796
  %_3.i.i434.inv.i.i = fcmp olt float %_0.i.i358.i.i, 1.270000e+02, !dbg !7800
  %_0.i.i441.i.i = select i1 %_3.i.i434.inv.i.i, float %_0.i.i358.i.i, float 1.270000e+02, !dbg !7800
  %374 = tail call noundef float @llvm.floor.f32(float %_0.i.i441.i.i), !dbg !7803
  %_0.i169.i.i = fsub float %_0.i.i441.i.i, %374, !dbg !7807
  %_0.i153.i.i = fmul float %_0.i169.i.i, 0x3F5E974FA0000000, !dbg !7809
  %_0.i138.i.i = fadd float %_0.i153.i.i, 0x3F82778560000000, !dbg !7811
  %_0.i153.1.i.i = fmul float %_0.i169.i.i, %_0.i138.i.i, !dbg !7809
  %_0.i138.1.i.i = fadd float %_0.i153.1.i.i, 0x3FAC91CE60000000, !dbg !7811
  %_0.i153.2.i.i = fmul float %_0.i169.i.i, %_0.i138.1.i.i, !dbg !7809
  %_0.i138.2.i.i = fadd float %_0.i153.2.i.i, 0x3FCEBDB560000000, !dbg !7811
  %_0.i153.3.i.i = fmul float %_0.i169.i.i, %_0.i138.2.i.i, !dbg !7809
  %_0.i138.3.i.i = fadd float %_0.i153.3.i.i, 0x3FE62E4BA0000000, !dbg !7811
  %_0.i151.i.i = fmul float %_0.i168.i.i, 0x3F5E974FA0000000, !dbg !7813
  %_0.i136.i.i = fadd float %_0.i151.i.i, 0x3F82778560000000, !dbg !7815
  %_0.i151.1.i.i = fmul float %_0.i168.i.i, %_0.i136.i.i, !dbg !7813
  %_0.i136.1.i.i = fadd float %_0.i151.1.i.i, 0x3FAC91CE60000000, !dbg !7815
  %_0.i151.2.i.i = fmul float %_0.i168.i.i, %_0.i136.1.i.i, !dbg !7813
  %_0.i136.2.i.i = fadd float %_0.i151.2.i.i, 0x3FCEBDB560000000, !dbg !7815
  %_0.i151.3.i.i = fmul float %_0.i168.i.i, %_0.i136.2.i.i, !dbg !7813
  %_0.i136.3.i.i = fadd float %_0.i151.3.i.i, 0x3FE62E4BA0000000, !dbg !7815
  %_0.i150.i.i = fmul float %_0.i168.i.i, %_0.i136.3.i.i, !dbg !7817
  %_0.i135.i.i = fadd float %_0.i150.i.i, 1.000000e+00, !dbg !7819
  %biased.i.i.i = fadd float %364, 0x4160000FE0000000, !dbg !7821
  %_4.i81.i.i = bitcast float %biased.i.i.i to i32, !dbg !7823
  %_3.i82.i.i = shl i32 %_4.i81.i.i, 23, !dbg !7825
  %_0.i83.i.i = bitcast i32 %_3.i82.i.i to float, !dbg !7826
  %_0.i149.i.i = fmul float %_0.i135.i.i, %_0.i83.i.i, !dbg !7828
  %_3.i89.i.i = fcmp une float %_0.i212.i.i, 0.000000e+00, !dbg !7830
  %_0.i320497.not.i.i = and i1 %_3.i113.i.i, %_3.i89.i.i, !dbg !7832
  %_0.i160.i.i = fmul float %_0.i191.i.i, %_0.i149.i.i, !dbg !7832
  %_4.i269.v.i.i = select i1 %_0.i320497.not.i.i, float %_0.i160.i.i, float %_0.i191.i.i, !dbg !7834
  %_0.i.i.i = fmul float %_0.i169.i.i, %_0.i138.3.i.i, !dbg !7836
  %_0.i137.i.i = fadd float %_0.i.i.i, 1.000000e+00, !dbg !7838
  %biased.i84.i.i = fadd float %374, 0x4160000FE0000000, !dbg !7840
  %_4.i85.i.i = bitcast float %biased.i84.i.i to i32, !dbg !7842
  %_3.i86.i.i = shl i32 %_4.i85.i.i, 23, !dbg !7844
  %_0.i87.i.i = bitcast i32 %_3.i86.i.i to float, !dbg !7845
  %_0.i152.i.i = fmul float %_0.i137.i.i, %_0.i87.i.i, !dbg !7847
  %_3.i88.i.i = fcmp une float %_0.i208.i.i, 0.000000e+00, !dbg !7849
  %_0.i317514.not.i.i = and i1 %_3.i99.i.i, %_3.i88.i.i, !dbg !7851
  %_0.i154.i.i = fmul float %_0.i189.i.i, %_0.i152.i.i, !dbg !7851
  %_4.i216.v.i.i = select i1 %_0.i317514.not.i.i, float %_0.i154.i.i, float %_0.i189.i.i, !dbg !7853
  store float %_4.i269.v.i.i, ptr %_97.i.i.i, align 4, !dbg !7855, !alias.scope !7858, !noalias !7404
  store float %_4.i216.v.i.i, ptr %_115.i.i.i, align 4, !dbg !7861, !alias.scope !7863, !noalias !7426
  %exitcond887.not.i.i = icmp eq i64 %346, %_55.i, !dbg !7866
  br i1 %exitcond887.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i, label %bb32.i.i.i, !dbg !7869, !llvm.loop !7882

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i: ; preds = %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.us.us.i85.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.us.i231.i, %_RNvNtCsdOTRa1MFkeb_13gate_expander6kernel15gather_detector.exit.i.i
  %_82.i.i160.i = trunc i64 %_55.i to i32, !dbg !7883
  %_81.i.i161.i = add i32 %base.i.i35.i, %_82.i.i160.i, !dbg !7884
  store i32 %_81.i.i161.i, ptr %_57.i28.i, align 4, !dbg !7886, !alias.scope !7334, !noalias !7359
  br label %bb7.i5, !dbg !7887

bb26.i:                                           ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i.i, i64 noundef range(i64 0, 2305843009213693952) %_13.1, i64 noundef range(i64 0, 2305843009213693952) %_13.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a6d4388bd1c2ee005f6a969a0e3ca0f4) #24, !dbg !7888, !noalias !6270
  unreachable, !dbg !7888

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9run_blockB2_.exit: ; preds = %bb1.backedge.i.i
  call void @llvm.lifetime.end.p0(ptr nonnull %iter.i.i), !dbg !7889, !noalias !7049
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) %reports.sroa.0, i64 16, i1 false), !dbg !7890
  %reports.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16, !dbg !7890
  store i64 %reports.sroa.4.0, ptr %reports.sroa.4.0._0.sroa_idx, align 8, !dbg !7890
  %reports.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24, !dbg !7890
  %reports.sroa.7.0.reports.sroa.7.0.copyload = load i64, ptr %reports.sroa.7, align 8, !dbg !7890
  store i64 %reports.sroa.7.0.reports.sroa.7.0.copyload, ptr %reports.sroa.7.0._0.sroa_idx, align 8, !dbg !7890
  %reports.sroa.8.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32, !dbg !7890
  %reports.sroa.8.0.reports.sroa.8.0.copyload = load i64, ptr %reports.sroa.8, align 8, !dbg !7890
  store i64 %reports.sroa.8.0.reports.sroa.8.0.copyload, ptr %reports.sroa.8.0._0.sroa_idx, align 8, !dbg !7890
  call void @llvm.lifetime.end.p0(ptr nonnull %reports.sroa.0), !dbg !7891
  call void @llvm.lifetime.end.p0(ptr nonnull %reports.sroa.7), !dbg !7891
  call void @llvm.lifetime.end.p0(ptr nonnull %reports.sroa.8), !dbg !7891
  ret void, !dbg !7892
}
