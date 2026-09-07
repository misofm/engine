define internal void @_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb0_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_(ptr dead_on_unwind noalias noundef writable writeonly sret([40 x i8]) align 8 captures(none) dereferenceable(40) %_0, ptr noalias noundef align 8 dereferenceable(1232) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(88) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !6025 {
start:
  %iter.i.i = alloca [56 x i8], align 8
  %pending.i = alloca [64 x i8], align 4
  %reports.sroa.0 = alloca [16 x i8], align 8
  %reports.sroa.7 = alloca i64, align 8
  %reports.sroa.8 = alloca i64, align 8
  call void @llvm.lifetime.start.p0(ptr nonnull %reports.sroa.0), !dbg !6026
  call void @llvm.lifetime.start.p0(ptr nonnull %reports.sroa.7), !dbg !6026
  call void @llvm.lifetime.start.p0(ptr nonnull %reports.sroa.8), !dbg !6026
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %reports.sroa.0, i8 0, i64 16, i1 false)
  store i64 0, ptr %reports.sroa.7, align 8
  store i64 0, ptr %reports.sroa.8, align 8
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !6027
  %_12.0 = load ptr, ptr %0, align 8, !dbg !6027, !nonnull !12, !align !4661, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !6027
  %_12.1 = load i64, ptr %1, align 8, !dbg !6027, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %block, i64 80, !dbg !6029
  %_6 = load i64, ptr %2, align 8, !dbg !6029, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6030), !dbg !6033
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6034), !dbg !6033
  call void @llvm.lifetime.start.p0(ptr nonnull %pending.i), !dbg !6036, !noalias !6039
  store i32 0, ptr %pending.i, align 4, !noalias !6039
  %_7.sroa.5.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 4
  %_7.sroa.5134.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 8
  store i32 0, ptr %_7.sroa.5134.0.pending.sroa_idx.i, align 4, !noalias !6039
  %_7.sroa.6137.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 16
  store i32 0, ptr %_7.sroa.6137.0.pending.sroa_idx.i, align 4, !noalias !6039
  %_7.sroa.7140.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 24
  store i32 0, ptr %_7.sroa.7140.0.pending.sroa_idx.i, align 4, !noalias !6039
  %3 = getelementptr inbounds nuw i8, ptr %pending.i, i64 32
  store i32 0, ptr %3, align 4, !noalias !6039
  %_7.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 36
  %_7.sroa.5134.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 40
  store i32 0, ptr %_7.sroa.5134.0..sroa_idx.i, align 4, !noalias !6039
  %_7.sroa.6137.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 48
  store i32 0, ptr %_7.sroa.6137.0..sroa_idx.i, align 4, !noalias !6039
  %_7.sroa.7140.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 56
  store i32 0, ptr %_7.sroa.7140.0..sroa_idx.i, align 4, !noalias !6039
  %_106.idx.i = mul nuw nsw i64 %_12.1, 40, !dbg !6041
  %_106.i = getelementptr inbounds nuw i8, ptr %_12.0, i64 %_106.idx.i, !dbg !6041
  %_6.i.i8184.i = icmp eq i64 %_12.1, 0, !dbg !6052
  br i1 %_6.i.i8184.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, label %bb4.lr.ph.lr.ph.i, !dbg !6062

bb4.lr.ph.lr.ph.i:                                ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 92
  %_31.i = load i32, ptr %4, align 4, !alias.scope !6030, !noalias !6063
  %_30.i = zext i32 %_31.i to i64
  br label %bb4.lr.ph.i, !dbg !6062

bb4.lr.ph.i:                                      ; preds = %bb31.i, %bb4.lr.ph.lr.ph.i
  %.lcssa175178 = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %5, %bb31.i ]
  %.promoted91.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %.promoted90.i, %bb31.i ]
  %last_order.sroa.3.0.ph88.i = phi i32 [ undef, %bb4.lr.ph.lr.ph.i ], [ %_127.0.i, %bb31.i ]
  %last_order.sroa.0.0.ph87.not.i = phi i1 [ true, %bb4.lr.ph.lr.ph.i ], [ false, %bb31.i ]
  %iter.sroa.0.0.ph86.i = phi ptr [ %_12.0, %bb4.lr.ph.lr.ph.i ], [ %_16.i.i.i, %bb31.i ]
  %iter.sroa.7.0.ph85.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %bb4.i, !dbg !6062

bb4.i:                                            ; preds = %bb35.i, %bb4.lr.ph.i
  %5 = phi i64 [ %.lcssa175178, %bb4.lr.ph.i ], [ %89, %bb35.i ]
  %.promoted90.i = phi i64 [ %.promoted91.i, %bb4.lr.ph.i ], [ %89, %bb35.i ]
  %iter.sroa.0.083.i = phi ptr [ %iter.sroa.0.0.ph86.i, %bb4.lr.ph.i ], [ %_16.i.i.i, %bb35.i ]
  %iter.sroa.7.082.i = phi i64 [ %iter.sroa.7.0.ph85.i, %bb4.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 40, !dbg !6064
  %_9.0.i.i = add i64 %iter.sroa.7.082.i, 1, !dbg !6067
  %6 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 32, !dbg !6070
  %_17.i = load i32, ptr %6, align 8, !dbg !6070, !range !1335, !alias.scope !6034, !noalias !6072, !noundef !12
  switch i32 %_17.i, label %default.unreachable [
    i32 1, label %bb9.i
    i32 2, label %bb7.i
    i32 3, label %bb35.i
  ], !dbg !6073

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i: ; preds = %bb46.us.3.i, %bb40.backedge.us.2.i
  %7 = load i32, ptr %3, align 4, !dbg !6074, !range !5716, !noalias !6039, !noundef !12
  %8 = trunc nuw i32 %7 to i1, !dbg !6079
  br i1 %8, label %bb46.us.1130.i, label %bb40.backedge.us.1131.i, !dbg !6079

bb46.us.1130.i:                                   ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 868
  %value.us.1118.i = load float, ptr %_7.sroa.5.0..sroa_idx.i, align 4, !dbg !6080, !noalias !6039, !noundef !12
  %_84.us.1119.i = load float, ptr %9, align 4, !dbg !6081, !alias.scope !6030, !noalias !6063, !noundef !12
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 872, !dbg !6084
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 876, !dbg !6085
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 880, !dbg !6086
  %_148.us.1120.i = bitcast float %_84.us.1119.i to i32, !dbg !6087
  %_150.us.1121.i = bitcast float %value.us.1118.i to i32, !dbg !6098
  %_149.us.1122.i = icmp ne i32 %_148.us.1120.i, %_150.us.1121.i, !dbg !6101
  %13 = icmp eq i32 %_148.us.1120.i, -2147483648
  %or.cond.us.1123.i = or i1 %_149.us.1122.i, %13, !dbg !6101
  %14 = tail call float @llvm.fabs.f32(float %_84.us.1119.i)
  %_144.us.1124.i = fcmp ueq float %14, 0x7FF0000000000000
  %or.cond40.us.1125.i = or i1 %_144.us.1124.i, %or.cond.us.1123.i, !dbg !6101
  %_146.us.1126.i = fsub float %value.us.1118.i, %_84.us.1119.i, !dbg !6101
  %15 = fmul float %_146.us.1126.i, 1.562500e-02, !dbg !6101
  %ramp.sroa.0.0.us.1127.i = select i1 %or.cond40.us.1125.i, float %_84.us.1119.i, float %value.us.1118.i, !dbg !6101
  %ramp4.sroa.0.0.us.1128.i = select i1 %or.cond40.us.1125.i, float %15, float 0.000000e+00, !dbg !6101
  %ramp5.sroa.0.0.us.1129.i = select i1 %or.cond40.us.1125.i, float 6.400000e+01, float 0.000000e+00, !dbg !6101
  store float %ramp.sroa.0.0.us.1127.i, ptr %9, align 4, !dbg !6102, !alias.scope !6104, !noalias !6063
  store float %value.us.1118.i, ptr %10, align 4, !dbg !6107, !alias.scope !6109, !noalias !6063
  store float %ramp4.sroa.0.0.us.1128.i, ptr %11, align 4, !dbg !6112, !alias.scope !6114, !noalias !6063
  store float %ramp5.sroa.0.0.us.1129.i, ptr %12, align 4, !dbg !6117, !alias.scope !6119, !noalias !6063
  store i32 64, ptr %44, align 4, !dbg !6122, !alias.scope !6030, !noalias !6063
  br label %bb40.backedge.us.1131.i, !dbg !6123

bb40.backedge.us.1131.i:                          ; preds = %bb46.us.1130.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i
  %16 = load i32, ptr %_7.sroa.5134.0..sroa_idx.i, align 4, !dbg !6074, !range !5716, !noalias !6039, !noundef !12
  %17 = trunc nuw i32 %16 to i1, !dbg !6079
  br i1 %17, label %bb46.us.1.1.i, label %bb40.backedge.us.1.1.i, !dbg !6079

bb46.us.1.1.i:                                    ; preds = %bb40.backedge.us.1131.i
  %18 = getelementptr inbounds nuw i8, ptr %pending.i, i64 44, !dbg !6074
  %value.us.1.1.i = load float, ptr %18, align 4, !dbg !6080, !noalias !6039, !noundef !12
  %slot.us.1.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884, !dbg !6124
  %_84.us.1.1.i = load float, ptr %slot.us.1.1.i, align 4, !dbg !6081, !alias.scope !6030, !noalias !6063, !noundef !12
  %19 = getelementptr inbounds nuw i8, ptr %self, i64 888, !dbg !6084
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 892, !dbg !6085
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 896, !dbg !6086
  %_148.us.1.1.i = bitcast float %_84.us.1.1.i to i32, !dbg !6087
  %_150.us.1.1.i = bitcast float %value.us.1.1.i to i32, !dbg !6098
  %_149.us.1.1.i = icmp ne i32 %_148.us.1.1.i, %_150.us.1.1.i, !dbg !6101
  %22 = icmp eq i32 %_148.us.1.1.i, -2147483648
  %or.cond.us.1.1.i = or i1 %_149.us.1.1.i, %22, !dbg !6101
  %23 = tail call float @llvm.fabs.f32(float %_84.us.1.1.i)
  %_144.us.1.1.i = fcmp ueq float %23, 0x7FF0000000000000
  %or.cond40.us.1.1.i = or i1 %_144.us.1.1.i, %or.cond.us.1.1.i, !dbg !6101
  %_146.us.1.1.i = fsub float %value.us.1.1.i, %_84.us.1.1.i, !dbg !6101
  %24 = fmul float %_146.us.1.1.i, 1.562500e-02, !dbg !6101
  %ramp.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float %_84.us.1.1.i, float %value.us.1.1.i, !dbg !6101
  %ramp4.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float %24, float 0.000000e+00, !dbg !6101
  %ramp5.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !6101
  store float %ramp.sroa.0.0.us.1.1.i, ptr %slot.us.1.1.i, align 4, !dbg !6102, !alias.scope !6104, !noalias !6063
  store float %value.us.1.1.i, ptr %19, align 4, !dbg !6107, !alias.scope !6109, !noalias !6063
  store float %ramp4.sroa.0.0.us.1.1.i, ptr %20, align 4, !dbg !6112, !alias.scope !6114, !noalias !6063
  store float %ramp5.sroa.0.0.us.1.1.i, ptr %21, align 4, !dbg !6117, !alias.scope !6119, !noalias !6063
  store i32 64, ptr %44, align 4, !dbg !6122, !alias.scope !6030, !noalias !6063
  br label %bb40.backedge.us.1.1.i, !dbg !6123

bb40.backedge.us.1.1.i:                           ; preds = %bb46.us.1.1.i, %bb40.backedge.us.1131.i
  %25 = load i32, ptr %_7.sroa.6137.0..sroa_idx.i, align 4, !dbg !6074, !range !5716, !noalias !6039, !noundef !12
  %26 = trunc nuw i32 %25 to i1, !dbg !6079
  br i1 %26, label %bb46.us.2.1.i, label %bb40.backedge.us.2.1.i, !dbg !6079

bb46.us.2.1.i:                                    ; preds = %bb40.backedge.us.1.1.i
  %27 = getelementptr inbounds nuw i8, ptr %pending.i, i64 52, !dbg !6074
  %value.us.2.1.i = load float, ptr %27, align 4, !dbg !6080, !noalias !6039, !noundef !12
  %slot.us.2.1.i = getelementptr inbounds nuw i8, ptr %self, i64 900, !dbg !6124
  %_84.us.2.1.i = load float, ptr %slot.us.2.1.i, align 4, !dbg !6081, !alias.scope !6030, !noalias !6063, !noundef !12
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 904, !dbg !6084
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 908, !dbg !6085
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 912, !dbg !6086
  %_148.us.2.1.i = bitcast float %_84.us.2.1.i to i32, !dbg !6087
  %_150.us.2.1.i = bitcast float %value.us.2.1.i to i32, !dbg !6098
  %_149.us.2.1.i = icmp ne i32 %_148.us.2.1.i, %_150.us.2.1.i, !dbg !6101
  %31 = icmp eq i32 %_148.us.2.1.i, -2147483648
  %or.cond.us.2.1.i = or i1 %_149.us.2.1.i, %31, !dbg !6101
  %32 = tail call float @llvm.fabs.f32(float %_84.us.2.1.i)
  %_144.us.2.1.i = fcmp ueq float %32, 0x7FF0000000000000
  %or.cond40.us.2.1.i = or i1 %_144.us.2.1.i, %or.cond.us.2.1.i, !dbg !6101
  %_146.us.2.1.i = fsub float %value.us.2.1.i, %_84.us.2.1.i, !dbg !6101
  %33 = fmul float %_146.us.2.1.i, 1.562500e-02, !dbg !6101
  %ramp.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float %_84.us.2.1.i, float %value.us.2.1.i, !dbg !6101
  %ramp4.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float %33, float 0.000000e+00, !dbg !6101
  %ramp5.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !6101
  store float %ramp.sroa.0.0.us.2.1.i, ptr %slot.us.2.1.i, align 4, !dbg !6102, !alias.scope !6104, !noalias !6063
  store float %value.us.2.1.i, ptr %28, align 4, !dbg !6107, !alias.scope !6109, !noalias !6063
  store float %ramp4.sroa.0.0.us.2.1.i, ptr %29, align 4, !dbg !6112, !alias.scope !6114, !noalias !6063
  store float %ramp5.sroa.0.0.us.2.1.i, ptr %30, align 4, !dbg !6117, !alias.scope !6119, !noalias !6063
  store i32 64, ptr %44, align 4, !dbg !6122, !alias.scope !6030, !noalias !6063
  br label %bb40.backedge.us.2.1.i, !dbg !6123

bb40.backedge.us.2.1.i:                           ; preds = %bb46.us.2.1.i, %bb40.backedge.us.1.1.i
  %34 = load i32, ptr %_7.sroa.7140.0..sroa_idx.i, align 4, !dbg !6074, !range !5716, !noalias !6039, !noundef !12
  %35 = trunc nuw i32 %34 to i1, !dbg !6079
  br i1 %35, label %bb46.us.3.1.i, label %bb40.backedge.us.2.1.i._RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit_crit_edge, !dbg !6079

bb40.backedge.us.2.1.i._RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit_crit_edge: ; preds = %bb40.backedge.us.2.1.i
  %_9.i.pre = load i32, ptr %44, align 4, !dbg !6125, !alias.scope !6131, !noalias !6134
  %36 = zext i32 %_9.i.pre to i64, !dbg !6139
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit, !dbg !6079

bb46.us.3.1.i:                                    ; preds = %bb40.backedge.us.2.1.i
  %37 = getelementptr inbounds nuw i8, ptr %pending.i, i64 60, !dbg !6074
  %value.us.3.1.i = load float, ptr %37, align 4, !dbg !6080, !noalias !6039, !noundef !12
  %slot.us.3.1.i = getelementptr inbounds nuw i8, ptr %self, i64 916, !dbg !6124
  %_84.us.3.1.i = load float, ptr %slot.us.3.1.i, align 4, !dbg !6081, !alias.scope !6030, !noalias !6063, !noundef !12
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 920, !dbg !6084
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 924, !dbg !6085
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 928, !dbg !6086
  %_148.us.3.1.i = bitcast float %_84.us.3.1.i to i32, !dbg !6087
  %_150.us.3.1.i = bitcast float %value.us.3.1.i to i32, !dbg !6098
  %_149.us.3.1.i = icmp ne i32 %_148.us.3.1.i, %_150.us.3.1.i, !dbg !6101
  %41 = icmp eq i32 %_148.us.3.1.i, -2147483648
  %or.cond.us.3.1.i = or i1 %_149.us.3.1.i, %41, !dbg !6101
  %42 = tail call float @llvm.fabs.f32(float %_84.us.3.1.i)
  %_144.us.3.1.i = fcmp ueq float %42, 0x7FF0000000000000
  %or.cond40.us.3.1.i = or i1 %_144.us.3.1.i, %or.cond.us.3.1.i, !dbg !6101
  %_146.us.3.1.i = fsub float %value.us.3.1.i, %_84.us.3.1.i, !dbg !6101
  %43 = fmul float %_146.us.3.1.i, 1.562500e-02, !dbg !6101
  %ramp.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float %_84.us.3.1.i, float %value.us.3.1.i, !dbg !6101
  %ramp4.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float %43, float 0.000000e+00, !dbg !6101
  %ramp5.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !6101
  store float %ramp.sroa.0.0.us.3.1.i, ptr %slot.us.3.1.i, align 4, !dbg !6102, !alias.scope !6104, !noalias !6063
  store float %value.us.3.1.i, ptr %38, align 4, !dbg !6107, !alias.scope !6109, !noalias !6063
  store float %ramp4.sroa.0.0.us.3.1.i, ptr %39, align 4, !dbg !6112, !alias.scope !6114, !noalias !6063
  store float %ramp5.sroa.0.0.us.3.1.i, ptr %40, align 4, !dbg !6117, !alias.scope !6119, !noalias !6063
  store i32 64, ptr %44, align 4, !dbg !6122, !alias.scope !6030, !noalias !6063
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit, !dbg !6123

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i: ; preds = %bb31.i, %bb35.i, %start
  %reports.sroa.4.0 = phi i64 [ 0, %start ], [ %89, %bb35.i ], [ %5, %bb31.i ]
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 1220
  %45 = load i32, ptr %pending.i, align 4, !dbg !6074, !range !5716, !noalias !6039, !noundef !12
  %46 = trunc nuw i32 %45 to i1, !dbg !6079
  br i1 %46, label %bb46.us.i, label %bb40.backedge.us.i, !dbg !6079

bb46.us.i:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 792
  %value.us.i = load float, ptr %_7.sroa.5.0.pending.sroa_idx.i, align 4, !dbg !6080, !noalias !6039, !noundef !12
  %_84.us.i = load float, ptr %47, align 4, !dbg !6081, !alias.scope !6030, !noalias !6063, !noundef !12
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 796, !dbg !6084
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 800, !dbg !6085
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 804, !dbg !6086
  %_148.us.i = bitcast float %_84.us.i to i32, !dbg !6087
  %_150.us.i = bitcast float %value.us.i to i32, !dbg !6098
  %_149.us.i = icmp ne i32 %_148.us.i, %_150.us.i, !dbg !6101
  %51 = icmp eq i32 %_148.us.i, -2147483648
  %or.cond.us.i = or i1 %_149.us.i, %51, !dbg !6101
  %52 = tail call float @llvm.fabs.f32(float %_84.us.i)
  %_144.us.i = fcmp ueq float %52, 0x7FF0000000000000
  %or.cond40.us.i = or i1 %_144.us.i, %or.cond.us.i, !dbg !6101
  %_146.us.i = fsub float %value.us.i, %_84.us.i, !dbg !6101
  %53 = fmul float %_146.us.i, 1.562500e-02, !dbg !6101
  %ramp.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %_84.us.i, float %value.us.i, !dbg !6101
  %ramp4.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %53, float 0.000000e+00, !dbg !6101
  %ramp5.sroa.0.0.us.i = select i1 %or.cond40.us.i, float 6.400000e+01, float 0.000000e+00, !dbg !6101
  store float %ramp.sroa.0.0.us.i, ptr %47, align 4, !dbg !6102, !alias.scope !6104, !noalias !6063
  store float %value.us.i, ptr %48, align 4, !dbg !6107, !alias.scope !6109, !noalias !6063
  store float %ramp4.sroa.0.0.us.i, ptr %49, align 4, !dbg !6112, !alias.scope !6114, !noalias !6063
  store float %ramp5.sroa.0.0.us.i, ptr %50, align 4, !dbg !6117, !alias.scope !6119, !noalias !6063
  store i32 64, ptr %44, align 4, !dbg !6122, !alias.scope !6030, !noalias !6063
  br label %bb40.backedge.us.i, !dbg !6123

bb40.backedge.us.i:                               ; preds = %bb46.us.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %54 = load i32, ptr %_7.sroa.5134.0.pending.sroa_idx.i, align 4, !dbg !6074, !range !5716, !noalias !6039, !noundef !12
  %55 = trunc nuw i32 %54 to i1, !dbg !6079
  br i1 %55, label %bb46.us.1.i, label %bb40.backedge.us.1.i, !dbg !6079

bb46.us.1.i:                                      ; preds = %bb40.backedge.us.i
  %56 = getelementptr inbounds nuw i8, ptr %pending.i, i64 12, !dbg !6074
  %value.us.1.i = load float, ptr %56, align 4, !dbg !6080, !noalias !6039, !noundef !12
  %slot.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808, !dbg !6124
  %_84.us.1.i = load float, ptr %slot.us.1.i, align 4, !dbg !6081, !alias.scope !6030, !noalias !6063, !noundef !12
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 812, !dbg !6084
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 816, !dbg !6085
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 820, !dbg !6086
  %_148.us.1.i = bitcast float %_84.us.1.i to i32, !dbg !6087
  %_150.us.1.i = bitcast float %value.us.1.i to i32, !dbg !6098
  %_149.us.1.i = icmp ne i32 %_148.us.1.i, %_150.us.1.i, !dbg !6101
  %60 = icmp eq i32 %_148.us.1.i, -2147483648
  %or.cond.us.1.i = or i1 %_149.us.1.i, %60, !dbg !6101
  %61 = tail call float @llvm.fabs.f32(float %_84.us.1.i)
  %_144.us.1.i = fcmp ueq float %61, 0x7FF0000000000000
  %or.cond40.us.1.i = or i1 %_144.us.1.i, %or.cond.us.1.i, !dbg !6101
  %_146.us.1.i = fsub float %value.us.1.i, %_84.us.1.i, !dbg !6101
  %62 = fmul float %_146.us.1.i, 1.562500e-02, !dbg !6101
  %ramp.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %_84.us.1.i, float %value.us.1.i, !dbg !6101
  %ramp4.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %62, float 0.000000e+00, !dbg !6101
  %ramp5.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !6101
  store float %ramp.sroa.0.0.us.1.i, ptr %slot.us.1.i, align 4, !dbg !6102, !alias.scope !6104, !noalias !6063
  store float %value.us.1.i, ptr %57, align 4, !dbg !6107, !alias.scope !6109, !noalias !6063
  store float %ramp4.sroa.0.0.us.1.i, ptr %58, align 4, !dbg !6112, !alias.scope !6114, !noalias !6063
  store float %ramp5.sroa.0.0.us.1.i, ptr %59, align 4, !dbg !6117, !alias.scope !6119, !noalias !6063
  store i32 64, ptr %44, align 4, !dbg !6122, !alias.scope !6030, !noalias !6063
  br label %bb40.backedge.us.1.i, !dbg !6123

bb40.backedge.us.1.i:                             ; preds = %bb46.us.1.i, %bb40.backedge.us.i
  %63 = load i32, ptr %_7.sroa.6137.0.pending.sroa_idx.i, align 4, !dbg !6074, !range !5716, !noalias !6039, !noundef !12
  %64 = trunc nuw i32 %63 to i1, !dbg !6079
  br i1 %64, label %bb46.us.2.i, label %bb40.backedge.us.2.i, !dbg !6079

bb46.us.2.i:                                      ; preds = %bb40.backedge.us.1.i
  %65 = getelementptr inbounds nuw i8, ptr %pending.i, i64 20, !dbg !6074
  %value.us.2.i = load float, ptr %65, align 4, !dbg !6080, !noalias !6039, !noundef !12
  %slot.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824, !dbg !6124
  %_84.us.2.i = load float, ptr %slot.us.2.i, align 4, !dbg !6081, !alias.scope !6030, !noalias !6063, !noundef !12
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 828, !dbg !6084
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 832, !dbg !6085
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 836, !dbg !6086
  %_148.us.2.i = bitcast float %_84.us.2.i to i32, !dbg !6087
  %_150.us.2.i = bitcast float %value.us.2.i to i32, !dbg !6098
  %_149.us.2.i = icmp ne i32 %_148.us.2.i, %_150.us.2.i, !dbg !6101
  %69 = icmp eq i32 %_148.us.2.i, -2147483648
  %or.cond.us.2.i = or i1 %_149.us.2.i, %69, !dbg !6101
  %70 = tail call float @llvm.fabs.f32(float %_84.us.2.i)
  %_144.us.2.i = fcmp ueq float %70, 0x7FF0000000000000
  %or.cond40.us.2.i = or i1 %_144.us.2.i, %or.cond.us.2.i, !dbg !6101
  %_146.us.2.i = fsub float %value.us.2.i, %_84.us.2.i, !dbg !6101
  %71 = fmul float %_146.us.2.i, 1.562500e-02, !dbg !6101
  %ramp.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %_84.us.2.i, float %value.us.2.i, !dbg !6101
  %ramp4.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %71, float 0.000000e+00, !dbg !6101
  %ramp5.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float 6.400000e+01, float 0.000000e+00, !dbg !6101
  store float %ramp.sroa.0.0.us.2.i, ptr %slot.us.2.i, align 4, !dbg !6102, !alias.scope !6104, !noalias !6063
  store float %value.us.2.i, ptr %66, align 4, !dbg !6107, !alias.scope !6109, !noalias !6063
  store float %ramp4.sroa.0.0.us.2.i, ptr %67, align 4, !dbg !6112, !alias.scope !6114, !noalias !6063
  store float %ramp5.sroa.0.0.us.2.i, ptr %68, align 4, !dbg !6117, !alias.scope !6119, !noalias !6063
  store i32 64, ptr %44, align 4, !dbg !6122, !alias.scope !6030, !noalias !6063
  br label %bb40.backedge.us.2.i, !dbg !6123

bb40.backedge.us.2.i:                             ; preds = %bb46.us.2.i, %bb40.backedge.us.1.i
  %72 = load i32, ptr %_7.sroa.7140.0.pending.sroa_idx.i, align 4, !dbg !6074, !range !5716, !noalias !6039, !noundef !12
  %73 = trunc nuw i32 %72 to i1, !dbg !6079
  br i1 %73, label %bb46.us.3.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i, !dbg !6079

bb46.us.3.i:                                      ; preds = %bb40.backedge.us.2.i
  %74 = getelementptr inbounds nuw i8, ptr %pending.i, i64 28, !dbg !6074
  %value.us.3.i = load float, ptr %74, align 4, !dbg !6080, !noalias !6039, !noundef !12
  %slot.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !6124
  %_84.us.3.i = load float, ptr %slot.us.3.i, align 4, !dbg !6081, !alias.scope !6030, !noalias !6063, !noundef !12
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 844, !dbg !6084
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 848, !dbg !6085
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 852, !dbg !6086
  %_148.us.3.i = bitcast float %_84.us.3.i to i32, !dbg !6087
  %_150.us.3.i = bitcast float %value.us.3.i to i32, !dbg !6098
  %_149.us.3.i = icmp ne i32 %_148.us.3.i, %_150.us.3.i, !dbg !6101
  %78 = icmp eq i32 %_148.us.3.i, -2147483648
  %or.cond.us.3.i = or i1 %_149.us.3.i, %78, !dbg !6101
  %79 = tail call float @llvm.fabs.f32(float %_84.us.3.i)
  %_144.us.3.i = fcmp ueq float %79, 0x7FF0000000000000
  %or.cond40.us.3.i = or i1 %_144.us.3.i, %or.cond.us.3.i, !dbg !6101
  %_146.us.3.i = fsub float %value.us.3.i, %_84.us.3.i, !dbg !6101
  %80 = fmul float %_146.us.3.i, 1.562500e-02, !dbg !6101
  %ramp.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %_84.us.3.i, float %value.us.3.i, !dbg !6101
  %ramp4.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %80, float 0.000000e+00, !dbg !6101
  %ramp5.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float 6.400000e+01, float 0.000000e+00, !dbg !6101
  store float %ramp.sroa.0.0.us.3.i, ptr %slot.us.3.i, align 4, !dbg !6102, !alias.scope !6104, !noalias !6063
  store float %value.us.3.i, ptr %75, align 4, !dbg !6107, !alias.scope !6109, !noalias !6063
  store float %ramp4.sroa.0.0.us.3.i, ptr %76, align 4, !dbg !6112, !alias.scope !6114, !noalias !6063
  store float %ramp5.sroa.0.0.us.3.i, ptr %77, align 4, !dbg !6117, !alias.scope !6119, !noalias !6063
  store i32 64, ptr %44, align 4, !dbg !6122, !alias.scope !6030, !noalias !6063
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i, !dbg !6123

default.unreachable:                              ; preds = %bb4.i
  unreachable

bb7.i:                                            ; preds = %bb4.i
  br label %bb9.i, !dbg !6140

bb9.i:                                            ; preds = %bb4.i, %bb7.i
  %channel.sroa.0.0.sroa.phi28.i = phi ptr [ %3, %bb7.i ], [ %pending.i, %bb4.i ], !dbg !6141
  %channel.sroa.0.0.i = phi i32 [ 1, %bb7.i ], [ 0, %bb4.i ], !dbg !6141
  %81 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 16, !dbg !6142
  %_21.i = load i32, ptr %81, align 8, !dbg !6142, !alias.scope !6034, !noalias !6072, !noundef !12
  %parameter_index.i = zext i32 %_21.i to i64, !dbg !6142
  %_121.1.i = icmp slt i32 %_21.i, 0, !dbg !6144
  br i1 %_121.1.i, label %bb35.i, label %bb59.i, !dbg !6150, !prof !180

bb59.i:                                           ; preds = %bb9.i
  %_121.0.i = shl nuw i32 %_21.i, 1, !dbg !6144
  %_127.0.i = or disjoint i32 %_121.0.i, %channel.sroa.0.0.i, !dbg !6156
  %_29.i = icmp ult i64 %iter.sroa.7.082.i, %_30.i, !dbg !6165
  %_32.i = icmp samesign ult i32 %_21.i, 4
  %or.cond16.i = and i1 %_29.i, %_32.i, !dbg !6165
  br i1 %or.cond16.i, label %bb11.i, label %bb35.i, !dbg !6165

bb11.i:                                           ; preds = %bb59.i
  %82 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 28, !dbg !6167
  %_130.i = load i32, ptr %82, align 4, !dbg !6167, !range !6171, !alias.scope !6034, !noalias !6072, !noundef !12
  %83 = icmp eq i32 %_130.i, 1, !dbg !6172
  br i1 %83, label %bb12.i, label %bb35.i, !dbg !6172

bb12.i:                                           ; preds = %bb11.i
  %_34.i = load i64, ptr %iter.sroa.0.083.i, align 8, !dbg !6173, !alias.scope !6034, !noalias !6072, !noundef !12
  %_33.i = icmp eq i64 %_34.i, %_6, !dbg !6173
  br i1 %_33.i, label %bb13.i, label %bb35.i, !dbg !6173

bb13.i:                                           ; preds = %bb12.i
  %84 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 8, !dbg !6174
  %_36.i = load i64, ptr %84, align 8, !dbg !6174, !alias.scope !6034, !noalias !6072, !noundef !12
  %_35.i = icmp eq i64 %_36.i, %_6, !dbg !6174
  br i1 %_35.i, label %bb14.i, label %bb35.i, !dbg !6174

bb14.i:                                           ; preds = %bb13.i
  %85 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 20, !dbg !6175
  %_39.i = load float, ptr %85, align 4, !dbg !6175, !alias.scope !6034, !noalias !6072, !noundef !12
  %_38.i = bitcast float %_39.i to i32, !dbg !6176
  %86 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 24, !dbg !6178
  %_4139.i = load i32, ptr %86, align 8, !dbg !6178, !alias.scope !6034, !noalias !6072, !noundef !12
  %_37.i = icmp eq i32 %_4139.i, %_38.i, !dbg !6175
  br i1 %_37.i, label %bb16.i, label %bb35.i, !dbg !6175

bb16.i:                                           ; preds = %bb14.i
  %_43.i = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_d0058eaee52f1ba8b2e5577cef0c3c7f, i64 %parameter_index.i, !dbg !6179
; call effect_runtime::params::parameter_value_valid
  %_42.i = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_43.i, float noundef %_39.i) #23, !dbg !6180, !noalias !6039
  %_45.i = icmp ugt i32 %_127.0.i, %last_order.sroa.3.0.ph88.i
  %or.cond17.not.not102.i = select i1 %last_order.sroa.0.0.ph87.not.i, i1 true, i1 %_45.i, !dbg !6180
  %or.cond41.not.i = select i1 %_42.i, i1 %or.cond17.not.not102.i, i1 false, !dbg !6180
  br i1 %or.cond41.not.i, label %bb29.i, label %bb35.i, !dbg !6180

bb29.i:                                           ; preds = %bb16.i
  %_47.i = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i, !dbg !6181
  %87 = load i32, ptr %_47.i, align 4, !dbg !6182, !range !5716, !noalias !6039, !noundef !12
  %_133.not.i = icmp eq i32 %87, 0, !dbg !6189
  br i1 %_133.not.i, label %bb31.i, label %bb35.i, !dbg !6190

bb31.i:                                           ; preds = %bb29.i
  %_47.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i
  %_135.i = fcmp oeq float %_39.i, 0.000000e+00, !dbg !6192
  %_55.sroa.0.0.i = select i1 %_135.i, float 0.000000e+00, float %_39.i, !dbg !6192
  store i32 1, ptr %_47.i.le, align 4, !dbg !6195, !noalias !6039
  %88 = getelementptr inbounds nuw i8, ptr %_47.i.le, i64 4, !dbg !6195
  store float %_55.sroa.0.0.i, ptr %88, align 4, !dbg !6195, !noalias !6039
  %_6.i.i81.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !6052
  br i1 %_6.i.i81.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, label %bb4.lr.ph.i, !dbg !6062

bb35.i:                                           ; preds = %bb4.i, %bb29.i, %bb16.i, %bb14.i, %bb13.i, %bb12.i, %bb11.i, %bb59.i, %bb9.i
  %89 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted90.i, i64 1), !dbg !6196
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !6052
  br i1 %_6.i.i.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, label %bb4.i, !dbg !6062

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit: ; preds = %bb40.backedge.us.2.1.i._RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit_crit_edge, %bb46.us.3.1.i
  %_9.i = phi i64 [ %36, %bb40.backedge.us.2.1.i._RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit_crit_edge ], [ 64, %bb46.us.3.1.i ], !dbg !6125
  call void @llvm.lifetime.end.p0(ptr nonnull %pending.i), !dbg !6199, !noalias !6039
  %_14.0 = load ptr, ptr %block, align 8, !dbg !6200, !nonnull !12, !align !3484, !noundef !12
  %90 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !6200
  %_14.1 = load i64, ptr %90, align 8, !dbg !6200, !noundef !12
  %91 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !6204
  %_13.0 = load ptr, ptr %91, align 8, !dbg !6204, !nonnull !12, !align !3484, !noundef !12
  %92 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !6204
  %_13.1 = load i64, ptr %92, align 8, !dbg !6204, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6131), !dbg !6205
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6206), !dbg !6205
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6207), !dbg !6205
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6208), !dbg !6205
  %..i.i = tail call noundef range(i64 0, 4294967296) i64 @llvm.umin.i64(i64 %_14.1, i64 range(i64 0, 4294967296) %_9.i), !dbg !6209
  %_10.not.i = icmp eq i64 %..i.i, 0, !dbg !6211
  br i1 %_10.not.i, label %bb4.i4, label %bb9.i2, !dbg !6211

bb4.i4:                                           ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKb1_EB3_.exit.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit
  %_18.i = icmp ugt i64 %_14.1, %_9.i, !dbg !6213
  br i1 %_18.i, label %bb20.i, label %bb7.i5, !dbg !6213

bb9.i2:                                           ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16apply_automationB2_.exit
  %_41.not.i = icmp samesign ugt i64 %..i.i, %_13.1, !dbg !6214
  br i1 %_41.not.i, label %bb19.i, label %bb18.i, !dbg !6214, !prof !180

bb19.i:                                           ; preds = %bb9.i2
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %..i.i, i64 noundef range(i64 0, 2305843009213693952) %_13.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0e50f2d670fa7dbeabdb68abcb7ac10) #24, !dbg !6225, !noalias !6134
  unreachable, !dbg !6225

bb18.i:                                           ; preds = %bb9.i2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6226), !dbg !6229
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6230), !dbg !6229
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6232), !dbg !6229
  %_10.i.i = getelementptr inbounds nuw i8, ptr %self, i64 792, !dbg !6234
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !6237
  %_15.i.i = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !6242
  %data.i.i645.i.i = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !6244
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 744, !dbg !6249
  %_32.i.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !6253
  %_58.0.i.i = load ptr, ptr %_15.i.i, align 8, !dbg !6254, !alias.scope !6255, !noalias !6256, !nonnull !12, !noundef !12
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !6254
  %_58.1.i.i = load i64, ptr %94, align 8, !dbg !6254, !alias.scope !6255, !noalias !6256, !noundef !12
  %_45.i.i = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !6258
  %_60.0.i.i = load ptr, ptr %data.i.i645.i.i, align 8, !dbg !6259, !alias.scope !6255, !noalias !6256, !nonnull !12, !noundef !12
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !6259
  %_60.1.i.i = load i64, ptr %95, align 8, !dbg !6259, !alias.scope !6255, !noalias !6256, !noundef !12
  %_50.i.i = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !6260
  %_51.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !6261
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !6262
  %_52.i.i = load i32, ptr %96, align 4, !dbg !6262, !alias.scope !6255, !noalias !6256, !noundef !12
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !6263
  %_53.i.i = load i32, ptr %97, align 8, !dbg !6263, !alias.scope !6255, !noalias !6256, !noundef !12
  %base.i.i.i = load i32, ptr %_51.i.i, align 4, !dbg !6264, !alias.scope !6255, !noalias !6274, !noundef !12
  %_67.i.i.i = load i32, ptr %_45.i.i, align 4, !alias.scope !6255, !noalias !6256
  %_75.i.i.i = load i32, ptr %_50.i.i, align 4, !alias.scope !6255, !noalias !6256
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 856
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %_37.i58.i.i = load float, ptr %99, align 4, !alias.scope !6255, !noalias !6256
  %_3.i220.i.i = fcmp ule float %_37.i58.i.i, 0.000000e+00
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %_41.i62.i.i = load float, ptr %100, align 4, !alias.scope !6255, !noalias !6256
  %_3.i218.i.i = fcmp ule float %_41.i62.i.i, 0.000000e+00
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 860
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %_71.i95663664.i.i = load float, ptr %102, align 4, !alias.scope !6255, !noalias !6256
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %_87.i110666.i.i = load i32, ptr %93, align 4, !alias.scope !6255, !noalias !6256
  %104 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %_88.i111667.i.i = load i32, ptr %104, align 4, !alias.scope !6255, !noalias !6256
  %105 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %_98.i124.i.i = load float, ptr %105, align 4, !alias.scope !6255, !noalias !6256
  %_3.i208.i.i = fcmp ule float %_98.i124.i.i, 0.000000e+00
  %106 = getelementptr inbounds nuw i8, ptr %self, i64 932
  %107 = getelementptr inbounds nuw i8, ptr %self, i64 784
  %_37.i.i.i = load float, ptr %107, align 4, !alias.scope !6255, !noalias !6256
  %_3.i236.i.i = fcmp ule float %_37.i.i.i, 0.000000e+00
  %108 = getelementptr inbounds nuw i8, ptr %self, i64 788
  %_41.i.i.i = load float, ptr %108, align 4, !alias.scope !6255, !noalias !6256
  %_3.i234.i.i = fcmp ule float %_41.i.i.i, 0.000000e+00
  %109 = getelementptr inbounds nuw i8, ptr %self, i64 936
  %110 = getelementptr inbounds nuw i8, ptr %self, i64 776
  %_71.i685686.i.i = load float, ptr %110, align 4, !alias.scope !6255, !noalias !6256
  %111 = getelementptr inbounds nuw i8, ptr %self, i64 940
  %_87.i22688.i.i = load i32, ptr %_32.i.i, align 4, !alias.scope !6255, !noalias !6256
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %_88.i23689.i.i = load i32, ptr %112, align 4, !alias.scope !6255, !noalias !6256
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %_98.i.i.i = load float, ptr %113, align 4, !alias.scope !6255, !noalias !6256
  %_3.i224.i.i = fcmp ule float %_98.i.i.i, 0.000000e+00
  %injected.cond1160.not.i.i = icmp ugt i64 %_58.1.i.i, %_60.1.i.i
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 804
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 796
  %iter1.sroa.0.0.ptr.i35.us.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 808
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 820
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 816
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 812
  %iter1.sroa.0.0.ptr.i35.us.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 824
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 836
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 828
  %iter1.sroa.0.0.ptr.i35.us.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 840
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 852
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 848
  %125 = getelementptr inbounds nuw i8, ptr %self, i64 844
  %126 = getelementptr inbounds nuw i8, ptr %self, i64 880
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 876
  %128 = getelementptr inbounds nuw i8, ptr %self, i64 872
  %iter1.sroa.0.0.ptr.i.us.1.i.i = getelementptr inbounds nuw i8, ptr %self, i64 884
  %129 = getelementptr inbounds nuw i8, ptr %self, i64 896
  %130 = getelementptr inbounds nuw i8, ptr %self, i64 892
  %131 = getelementptr inbounds nuw i8, ptr %self, i64 888
  %iter1.sroa.0.0.ptr.i.us.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 900
  %132 = getelementptr inbounds nuw i8, ptr %self, i64 912
  %133 = getelementptr inbounds nuw i8, ptr %self, i64 908
  %134 = getelementptr inbounds nuw i8, ptr %self, i64 904
  %iter1.sroa.0.0.ptr.i.us.3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 916
  %135 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %136 = getelementptr inbounds nuw i8, ptr %self, i64 924
  %137 = getelementptr inbounds nuw i8, ptr %self, i64 920
  br i1 %injected.cond1160.not.i.i, label %bb42.i.us.i.i, label %bb42.i.us.us.us.i.i

bb42.i.us.us.us.i.i:                              ; preds = %bb18.i, %bb28.i.us.us.lr.ph.us.us.us.i.i
  %iter.sroa.0.0.i1043.us.us.us.i.i = phi i64 [ %138, %bb28.i.us.us.lr.ph.us.us.us.i.i ], [ 0, %bb18.i ]
  %138 = add nuw nsw i64 %iter.sroa.0.0.i1043.us.us.us.i.i, 1, !dbg !6277
  %_27.i.us.us.us.i.i = trunc i64 %iter.sroa.0.0.i1043.us.us.us.i.i to i32, !dbg !6291
  %now.i.us.us.us.i.i = add i32 %base.i.i.i, %_27.i.us.us.us.i.i, !dbg !6294
  %_30.i.us.us.us.i.i = and i32 %now.i.us.us.us.i.i, %_52.i.i, !dbg !6297
  %_29.i.us.us.us.i.i = zext i32 %_30.i.us.us.us.i.i to i64, !dbg !6299
  %_123.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_14.0, i64 %iter.sroa.0.0.i1043.us.us.us.i.i, !dbg !6300
  %_124.not.not.i.us.us.us.i.i = icmp ugt i64 %_58.1.i.i, %_29.i.us.us.us.i.i, !dbg !6309
  br i1 %_124.not.not.i.us.us.us.i.i, label %bb45.i.us.us.us.i.i, label %bb46.i.i.i, !dbg !6309, !prof !2704

bb45.i.us.us.us.i.i:                              ; preds = %bb42.i.us.us.us.i.i
  %_0.i313.us.us.us.i.i = load float, ptr %_123.i.us.us.us.i.i, align 4, !dbg !6314, !alias.scope !6316, !noalias !6319, !noundef !12
  %_133.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_29.i.us.us.us.i.i, !dbg !6320
  store float %_0.i313.us.us.us.i.i, ptr %_133.i.us.us.us.i.i, align 4, !dbg !6324, !alias.scope !6326, !noalias !6274
  %_141.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_13.0, i64 %iter.sroa.0.0.i1043.us.us.us.i.i, !dbg !6329
  %_0.i311.us.us.us.i.i = load float, ptr %_141.i.us.us.us.i.i, align 4, !dbg !6336, !alias.scope !6338, !noalias !6341, !noundef !12
  %_149.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_29.i.us.us.us.i.i, !dbg !6342
  store float %_0.i311.us.us.us.i.i, ptr %_149.i.us.us.us.i.i, align 4, !dbg !6349, !alias.scope !6351, !noalias !6274
  %_52.i.us.us.us.i.i = sub i32 %now.i.us.us.us.i.i, %_53.i.i, !dbg !6354
  %_51.i.us.us.us.i.i = and i32 %_52.i.us.us.us.i.i, %_52.i.i, !dbg !6357
  %_50.i.us.us.us.i.i = zext i32 %_51.i.us.us.us.i.i to i64, !dbg !6358
  %_182.not.not.i.us.us.us.i.i = icmp ugt i64 %_58.1.i.i, %_50.i.us.us.us.i.i, !dbg !6359
  br i1 %_182.not.not.i.us.us.us.i.i, label %bb60.i.us.us.us.i.i, label %bb61.i.i.i, !dbg !6359, !prof !2704

bb60.i.us.us.us.i.i:                              ; preds = %bb45.i.us.us.us.i.i
  %_189.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_50.i.us.us.us.i.i, !dbg !6364
  %_0.i309.us.us.us.i.i = load float, ptr %_189.i.us.us.us.i.i, align 4, !dbg !6368, !alias.scope !6370, !noalias !6274, !noundef !12
  %_195.i.us.us.us.i.i = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_50.i.us.us.us.i.i, !dbg !6373
  %_0.i307.us.us.us.i.i = load float, ptr %_195.i.us.us.us.i.i, align 4, !dbg !6381, !alias.scope !6383, !noalias !6274, !noundef !12
  %_66.i.us.us.us.i.i = sub i32 %now.i.us.us.us.i.i, %_67.i.i.i
  %_65.i.us.us.us.i.i = and i32 %_66.i.us.us.us.i.i, %_52.i.i
  %_64.i.us.us.us.i.i = zext i32 %_65.i.us.us.us.i.i to i64
  %_74.i.us.us.us.i.i = sub i32 %now.i.us.us.us.i.i, %_75.i.i.i
  %_73.i.us.us.us.i.i = and i32 %_74.i.us.us.us.i.i, %_52.i.i
  %_72.i.us.us.us.i.i = zext i32 %_73.i.us.us.us.i.i to i64
  %_80.i.us.us.us.i.i = icmp ugt i64 %_58.1.i.i, %_64.i.us.us.us.i.i
  %139 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_64.i.us.us.us.i.i
  %140 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_72.i.us.us.us.i.i
  %_88.i.us.us.us.i.i = icmp ugt i64 %_58.1.i.i, %_72.i.us.us.us.i.i
  %141 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_72.i.us.us.us.i.i
  br i1 %_80.i.us.us.us.i.i, label %bb63.i.split.us.us.us.us.i.i, label %bb63.i.split.i.i

bb63.i.split.us.us.us.us.i.i:                     ; preds = %bb60.i.us.us.us.i.i
  %_86.i.us.us.us.i.i = icmp ugt i64 %_60.1.i.i, %_72.i.us.us.us.i.i
  %142 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_64.i.us.us.us.i.i
  %_82.i.us.us.us.us.i.i = load float, ptr %142, align 4, !noalias !6274, !noundef !12
  br i1 %_86.i.us.us.us.i.i, label %bb24.i.us.lr.ph.split.us.us.us.us.i.i, label %bb24.i.us.lr.ph.split.i.i

bb24.i.us.lr.ph.split.us.us.us.us.i.i:            ; preds = %bb63.i.split.us.us.us.us.i.i
  br i1 %_88.i.us.us.us.i.i, label %bb28.i.us.us.lr.ph.us.us.us.i.i, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i.i, !dbg !6386

bb28.i.us.us.lr.ph.us.us.us.i.i:                  ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.i.i
  %_87.i.us.us.us.us.us.i.i = load float, ptr %141, align 4, !noalias !6274, !noundef !12
  %_85.i.us.us.le911.us.us.us.i.i = load float, ptr %140, align 4, !noalias !6274, !noundef !12
  %_78.i.us.le.us.us.us.i.i = load float, ptr %139, align 4, !noalias !6274, !noundef !12
  %_12.i37.us.us.us.i.i = load float, ptr %114, align 4, !dbg !6394, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i222.us.us.us.i.i = fcmp ule float %_12.i37.us.us.us.i.i, 0.000000e+00, !dbg !6401
  %_3.i194.us.us.us.i.i = fcmp une float %_12.i37.us.us.us.i.i, 1.000000e+00, !dbg !6403
  %_16.i40.us.us.us.i.i = load float, ptr %_10.i.i, align 4, !dbg !6405, !alias.scope !6396, !noalias !6399, !noundef !12
  %_17.i41.us.us.us.i.i = load float, ptr %115, align 4, !dbg !6406, !alias.scope !6396, !noalias !6399, !noundef !12
  %_0.i253.us.us.us.i.i = fadd float %_16.i40.us.us.us.i.i, %_17.i41.us.us.us.i.i, !dbg !6407
  %_20.i43651654.us.us.us.i.i = load float, ptr %116, align 4, !dbg !6409, !alias.scope !6396, !noalias !6399, !noundef !12
  %143 = select i1 %_3.i194.us.us.us.i.i, float %_0.i253.us.us.us.i.i, float %_20.i43651654.us.us.us.i.i, !dbg !6410
  %_0.i401.us.us.us.i.i = select i1 %_3.i222.us.us.us.i.i, float %_16.i40.us.us.us.i.i, float %143, !dbg !6412
  store float %_0.i401.us.us.us.i.i, ptr %_10.i.i, align 4, !dbg !6414, !alias.scope !6396, !noalias !6399
  %_0.i394.us.us.us.i.i = select i1 %_3.i194.us.us.us.i.i, float %_17.i41.us.us.us.i.i, float 0.000000e+00, !dbg !6415
  store float %_0.i394.us.us.us.i.i, ptr %115, align 4, !dbg !6417, !alias.scope !6396, !noalias !6399
  %_0.i291.us.us.us.i.i = fadd float %_12.i37.us.us.us.i.i, -1.000000e+00, !dbg !6418
  %_4.i387.v.us.us.us.i.i = select i1 %_3.i222.us.us.us.i.i, float %_12.i37.us.us.us.i.i, float %_0.i291.us.us.us.i.i, !dbg !6420
  store float %_4.i387.v.us.us.us.i.i, ptr %114, align 4, !dbg !6422, !alias.scope !6396, !noalias !6399
  %_12.i37.us.us.us.1.i.i = load float, ptr %117, align 4, !dbg !6394, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i222.us.us.us.1.i.i = fcmp ule float %_12.i37.us.us.us.1.i.i, 0.000000e+00, !dbg !6401
  %_3.i194.us.us.us.1.i.i = fcmp une float %_12.i37.us.us.us.1.i.i, 1.000000e+00, !dbg !6403
  %_16.i40.us.us.us.1.i.i = load float, ptr %iter1.sroa.0.0.ptr.i35.us.1.i.i, align 4, !dbg !6405, !alias.scope !6396, !noalias !6399, !noundef !12
  %_17.i41.us.us.us.1.i.i = load float, ptr %118, align 4, !dbg !6406, !alias.scope !6396, !noalias !6399, !noundef !12
  %_0.i253.us.us.us.1.i.i = fadd float %_16.i40.us.us.us.1.i.i, %_17.i41.us.us.us.1.i.i, !dbg !6407
  %_20.i43651654.us.us.us.1.i.i = load float, ptr %119, align 4, !dbg !6409, !alias.scope !6396, !noalias !6399, !noundef !12
  %144 = select i1 %_3.i194.us.us.us.1.i.i, float %_0.i253.us.us.us.1.i.i, float %_20.i43651654.us.us.us.1.i.i, !dbg !6410
  %_0.i401.us.us.us.1.i.i = select i1 %_3.i222.us.us.us.1.i.i, float %_16.i40.us.us.us.1.i.i, float %144, !dbg !6412
  store float %_0.i401.us.us.us.1.i.i, ptr %iter1.sroa.0.0.ptr.i35.us.1.i.i, align 4, !dbg !6414, !alias.scope !6396, !noalias !6399
  %_0.i394.us.us.us.1.i.i = select i1 %_3.i194.us.us.us.1.i.i, float %_17.i41.us.us.us.1.i.i, float 0.000000e+00, !dbg !6415
  store float %_0.i394.us.us.us.1.i.i, ptr %118, align 4, !dbg !6417, !alias.scope !6396, !noalias !6399
  %_0.i291.us.us.us.1.i.i = fadd float %_12.i37.us.us.us.1.i.i, -1.000000e+00, !dbg !6418
  %_4.i387.v.us.us.us.1.i.i = select i1 %_3.i222.us.us.us.1.i.i, float %_12.i37.us.us.us.1.i.i, float %_0.i291.us.us.us.1.i.i, !dbg !6420
  store float %_4.i387.v.us.us.us.1.i.i, ptr %117, align 4, !dbg !6422, !alias.scope !6396, !noalias !6399
  %_12.i37.us.us.us.2.i.i = load float, ptr %120, align 4, !dbg !6394, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i222.us.us.us.2.i.i = fcmp ule float %_12.i37.us.us.us.2.i.i, 0.000000e+00, !dbg !6401
  %_3.i194.us.us.us.2.i.i = fcmp une float %_12.i37.us.us.us.2.i.i, 1.000000e+00, !dbg !6403
  %_16.i40.us.us.us.2.i.i = load float, ptr %iter1.sroa.0.0.ptr.i35.us.2.i.i, align 4, !dbg !6405, !alias.scope !6396, !noalias !6399, !noundef !12
  %_17.i41.us.us.us.2.i.i = load float, ptr %121, align 4, !dbg !6406, !alias.scope !6396, !noalias !6399, !noundef !12
  %_0.i253.us.us.us.2.i.i = fadd float %_16.i40.us.us.us.2.i.i, %_17.i41.us.us.us.2.i.i, !dbg !6407
  %_20.i43651654.us.us.us.2.i.i = load float, ptr %122, align 4, !dbg !6409, !alias.scope !6396, !noalias !6399, !noundef !12
  %145 = select i1 %_3.i194.us.us.us.2.i.i, float %_0.i253.us.us.us.2.i.i, float %_20.i43651654.us.us.us.2.i.i, !dbg !6410
  %_0.i401.us.us.us.2.i.i = select i1 %_3.i222.us.us.us.2.i.i, float %_16.i40.us.us.us.2.i.i, float %145, !dbg !6412
  store float %_0.i401.us.us.us.2.i.i, ptr %iter1.sroa.0.0.ptr.i35.us.2.i.i, align 4, !dbg !6414, !alias.scope !6396, !noalias !6399
  %_0.i394.us.us.us.2.i.i = select i1 %_3.i194.us.us.us.2.i.i, float %_17.i41.us.us.us.2.i.i, float 0.000000e+00, !dbg !6415
  store float %_0.i394.us.us.us.2.i.i, ptr %121, align 4, !dbg !6417, !alias.scope !6396, !noalias !6399
  %_0.i291.us.us.us.2.i.i = fadd float %_12.i37.us.us.us.2.i.i, -1.000000e+00, !dbg !6418
  %_4.i387.v.us.us.us.2.i.i = select i1 %_3.i222.us.us.us.2.i.i, float %_12.i37.us.us.us.2.i.i, float %_0.i291.us.us.us.2.i.i, !dbg !6420
  store float %_4.i387.v.us.us.us.2.i.i, ptr %120, align 4, !dbg !6422, !alias.scope !6396, !noalias !6399
  %_12.i37.us.us.us.3.i.i = load float, ptr %123, align 4, !dbg !6394, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i222.us.us.us.3.i.i = fcmp ule float %_12.i37.us.us.us.3.i.i, 0.000000e+00, !dbg !6401
  %_3.i194.us.us.us.3.i.i = fcmp une float %_12.i37.us.us.us.3.i.i, 1.000000e+00, !dbg !6403
  %_16.i40.us.us.us.3.i.i = load float, ptr %iter1.sroa.0.0.ptr.i35.us.3.i.i, align 4, !dbg !6405, !alias.scope !6396, !noalias !6399, !noundef !12
  %_17.i41.us.us.us.3.i.i = load float, ptr %124, align 4, !dbg !6406, !alias.scope !6396, !noalias !6399, !noundef !12
  %_0.i253.us.us.us.3.i.i = fadd float %_16.i40.us.us.us.3.i.i, %_17.i41.us.us.us.3.i.i, !dbg !6407
  %_20.i43651654.us.us.us.3.i.i = load float, ptr %125, align 4, !dbg !6409, !alias.scope !6396, !noalias !6399, !noundef !12
  %146 = select i1 %_3.i194.us.us.us.3.i.i, float %_0.i253.us.us.us.3.i.i, float %_20.i43651654.us.us.us.3.i.i, !dbg !6410
  %_0.i401.us.us.us.3.i.i = select i1 %_3.i222.us.us.us.3.i.i, float %_16.i40.us.us.us.3.i.i, float %146, !dbg !6412
  store float %_0.i401.us.us.us.3.i.i, ptr %iter1.sroa.0.0.ptr.i35.us.3.i.i, align 4, !dbg !6414, !alias.scope !6396, !noalias !6399
  %_0.i394.us.us.us.3.i.i = select i1 %_3.i194.us.us.us.3.i.i, float %_17.i41.us.us.us.3.i.i, float 0.000000e+00, !dbg !6415
  store float %_0.i394.us.us.us.3.i.i, ptr %124, align 4, !dbg !6417, !alias.scope !6396, !noalias !6399
  %_0.i291.us.us.us.3.i.i = fadd float %_12.i37.us.us.us.3.i.i, -1.000000e+00, !dbg !6418
  %_4.i387.v.us.us.us.3.i.i = select i1 %_3.i222.us.us.us.3.i.i, float %_12.i37.us.us.us.3.i.i, float %_0.i291.us.us.us.3.i.i, !dbg !6420
  store float %_4.i387.v.us.us.us.3.i.i, ptr %123, align 4, !dbg !6422, !alias.scope !6396, !noalias !6399
  %147 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.i.i), !dbg !6423
  %148 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.i.i), !dbg !6425
  %_3.i.i554.us.us.us.i.i = fcmp ule float %147, %148, !dbg !6427
  %_6.i.i556.us.us.us.i.i = bitcast float %147 to i32, !dbg !6430
  %_8.i.i558.us.us.us.i.i = bitcast float %148 to i32, !dbg !6433
  %_4.i.i561.us.us.us.i.i = select i1 %_3.i.i554.us.us.us.i.i, i32 %_8.i.i558.us.us.us.i.i, i32 %_6.i.i556.us.us.us.i.i, !dbg !6435
  %_4.i380.us.us.us.i.i = select i1 %_3.i220.i.i, i32 %_6.i.i556.us.us.us.i.i, i32 %_4.i.i561.us.us.us.i.i, !dbg !6436
  %_0.i275.us.us.us.i.i = fmul float %147, 5.000000e-01, !dbg !6438
  %_0.i274.us.us.us.i.i = fmul float %148, 5.000000e-01, !dbg !6440
  %_0.i252.us.us.us.i.i = fadd float %_0.i274.us.us.us.i.i, %_0.i275.us.us.us.i.i, !dbg !6442
  %_6.i368.us.us.us.i.i = bitcast float %_0.i252.us.us.us.i.i to i32, !dbg !6444
  %_4.i373.us.us.us.i.i = select i1 %_3.i218.i.i, i32 %_4.i380.us.us.us.i.i, i32 %_6.i368.us.us.us.i.i, !dbg !6447
  %_0.i374.us.us.us.i.i = bitcast i32 %_4.i373.us.us.us.i.i to float, !dbg !6448
  %_3.i.i546.us.us.us.i.i = fcmp ule float %_0.i374.us.us.us.i.i, 0x3E45798EE0000000, !dbg !6450
  %_4.i.i552.us.us.us.i.i = select i1 %_3.i.i546.us.us.us.i.i, i32 841731191, i32 %_4.i373.us.us.us.i.i, !dbg !6453
  %_0.i.i553.us.us.us.i.i = bitcast i32 %_4.i.i552.us.us.us.i.i to float, !dbg !6455
  %_3.i.i506.us.us.us.i.i = fcmp ule float %_0.i.i553.us.us.us.i.i, 0x3810000000000000, !dbg !6457
  %_4.i.i512.us.us.us.i.i = select i1 %_3.i.i506.us.us.us.i.i, i32 8388608, i32 %_4.i.i552.us.us.us.i.i, !dbg !6462
  %_5.i318.us.us.us.i.i = and i32 %_4.i.i512.us.us.us.i.i, 8388607, !dbg !6464
  %_4.i319.us.us.us.i.i = or disjoint i32 %_5.i318.us.us.us.i.i, 1065353216, !dbg !6464
  %significand.i320.us.us.us.i.i = bitcast i32 %_4.i319.us.us.us.i.i to float, !dbg !6466
  %_0.i283.us.us.us.i.i = fadd float %significand.i320.us.us.us.i.i, -1.000000e+00, !dbg !6468
  %_0.i263.us.us.us.i.i = fmul float %_0.i283.us.us.us.i.i, 0x3F9B17A960000000, !dbg !6470
  %149 = fsub float 0x3FBF9A8440000000, %_0.i263.us.us.us.i.i, !dbg !6472
  %_0.i263.us.us.us.1.i.i = fmul float %_0.i283.us.us.us.i.i, %149, !dbg !6470
  %_0.i247.us.us.us.1.i.i = fadd float %_0.i263.us.us.us.1.i.i, 0xBFD1E3F400000000, !dbg !6472
  %_0.i263.us.us.us.2.i.i = fmul float %_0.i283.us.us.us.i.i, %_0.i247.us.us.us.1.i.i, !dbg !6470
  %_0.i247.us.us.us.2.i.i = fadd float %_0.i263.us.us.us.2.i.i, 0x3FDD544F20000000, !dbg !6472
  %_0.i263.us.us.us.3.i.i = fmul float %_0.i283.us.us.us.i.i, %_0.i247.us.us.us.2.i.i, !dbg !6470
  %_0.i247.us.us.us.3.i.i = fadd float %_0.i263.us.us.us.3.i.i, 0xBFE6FC2A60000000, !dbg !6472
  %_0.i263.us.us.us.4.i.i = fmul float %_0.i283.us.us.us.i.i, %_0.i247.us.us.us.3.i.i, !dbg !6470
  %_0.i247.us.us.us.4.i.i = fadd float %_0.i263.us.us.us.4.i.i, 0x3FF714B2A0000000, !dbg !6472
  %_9.i321.us.us.us.i.i = lshr i32 %_4.i.i512.us.us.us.i.i, 23, !dbg !6474
  %_8.i322.us.us.us.i.i = or disjoint i32 %_9.i321.us.us.us.i.i, 1258291200, !dbg !6474
  %_7.i323.us.us.us.i.i = bitcast i32 %_8.i322.us.us.us.i.i to float, !dbg !6475
  %exponent.i324.us.us.us.i.i = fadd float %_7.i323.us.us.us.i.i, 0xC160000FE0000000, !dbg !6477
  %_0.i262.us.us.us.i.i = fmul float %_0.i283.us.us.us.i.i, %_0.i247.us.us.us.4.i.i, !dbg !6478
  %_0.i246.us.us.us.i.i = fadd float %exponent.i324.us.us.us.i.i, %_0.i262.us.us.us.i.i, !dbg !6480
  %_0.i273.us.us.us.i.i = fmul float %_0.i246.us.us.us.i.i, 0x4018151820000000, !dbg !6482
  %_3.i.i621.us.us.us.inv.i.i = fcmp olt float %_0.i273.us.us.us.i.i, 2.400000e+01, !dbg !6484
  %_0.i.i628.us.us.us.i.i = select i1 %_3.i.i621.us.us.us.inv.i.i, float %_0.i273.us.us.us.i.i, float 2.400000e+01, !dbg !6484
  %_3.i.i538.us.us.us.inv.i.i = fcmp ogt float %_0.i.i628.us.us.us.i.i, -1.600000e+02, !dbg !6487
  %_0.i.i545.us.us.us.i.i = select i1 %_3.i.i538.us.us.us.inv.i.i, float %_0.i.i628.us.us.us.i.i, float -1.600000e+02, !dbg !6487
  %_55.i79.us.us.us.i.i = load float, ptr %98, align 4, !dbg !6490, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i216.us.us.us.i.i = fcmp ule float %_55.i79.us.us.us.i.i, 0.000000e+00, !dbg !6491
  %_3.i202.us.us.us.i.i = fcmp oge float %_0.i.i545.us.us.us.i.i, %_0.i401.us.us.us.i.i, !dbg !6493
  %_0.i290.us.us.us.i.i = fsub float %_0.i401.us.us.us.i.i, %_0.i401.us.us.us.3.i.i, !dbg !6495
  %_3.i200.us.us.us.i.i = fcmp oge float %_0.i.i545.us.us.us.i.i, %_0.i290.us.us.us.i.i, !dbg !6497
  %..i201.us.us.us.i.i = sext i1 %_3.i200.us.us.us.i.i to i32, !dbg !6499
  %_0.i496.us.us.us.i.i = sext i1 %_3.i202.us.us.us.i.i to i32, !dbg !6501
  %_0.i490.us.us.us.i.i = select i1 %_3.i216.us.us.us.i.i, i32 %_0.i496.us.us.us.i.i, i32 %..i201.us.us.us.i.i, !dbg !6501
  %_0.i502.us.us.us.i.i = xor i32 %..i201.us.us.us.i.i, -1, !dbg !6503
  %_67.i89.us.us.us.i.i = load float, ptr %101, align 4, !dbg !6505, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i214.us.us.us.i.i = fcmp ogt float %_67.i89.us.us.us.i.i, 0.000000e+00, !dbg !6506
  %_0.i495.us.us.us.i.i = select i1 %_3.i214.us.us.us.i.i, i32 %_0.i502.us.us.us.i.i, i32 0, !dbg !6508
  %_0.i494.us.us.us.i.i = select i1 %_3.i216.us.us.us.i.i, i32 0, i32 %_0.i495.us.us.us.i.i, !dbg !6510
  %_0.i489.us.us.us.i.i = or i32 %_0.i494.us.us.us.i.i, %_0.i490.us.us.us.i.i, !dbg !6512
  %_5.i363.us.us.us.i.i = and i32 %_0.i489.us.us.us.i.i, 1065353216, !dbg !6514
  %_0.i367.us.us.us.i.i = bitcast i32 %_5.i363.us.us.us.i.i to float, !dbg !6516
  %_0.i289.us.us.us.i.i = fadd float %_67.i89.us.us.us.i.i, -1.000000e+00, !dbg !6518
  %150 = trunc nsw i32 %_0.i494.us.us.us.i.i to i1, !dbg !6520
  %_4.i361.v.us.us.us.i.i = select i1 %150, float %_0.i289.us.us.us.i.i, float %_67.i89.us.us.us.i.i, !dbg !6520
  %151 = trunc nsw i32 %_0.i490.us.us.us.i.i to i1, !dbg !6522
  %_0.i355.us.us.us.i.i = select i1 %151, float %_71.i95663664.i.i, float %_4.i361.v.us.us.us.i.i, !dbg !6522
  store float %_0.i355.us.us.us.i.i, ptr %101, align 4, !dbg !6524, !alias.scope !6396, !noalias !6399
  store i32 %_5.i363.us.us.us.i.i, ptr %98, align 4, !dbg !6525, !alias.scope !6396, !noalias !6399
  %_0.i288.us.us.us.i.i = fadd float %_0.i401.us.us.us.1.i.i, -1.000000e+00, !dbg !6526
  %_0.i287.us.us.us.i.i = fsub float %_0.i.i545.us.us.us.i.i, %_0.i401.us.us.us.i.i, !dbg !6528
  %_0.i272.us.us.us.i.i = fmul float %_0.i288.us.us.us.i.i, %_0.i287.us.us.us.i.i, !dbg !6530
  %152 = fneg float %_0.i401.us.us.us.2.i.i, !dbg !6532
  %_3.i.i530.inv.us.us.us.i.i = fcmp ogt float %_0.i272.us.us.us.i.i, %152, !dbg !6534
  %_4.i.i536.v.us.us.us.i.i = select i1 %_3.i.i530.inv.us.us.us.i.i, float %_0.i272.us.us.us.i.i, float %152, !dbg !6534
  %_3.i.i613.us.us.us.i.i = fcmp olt float %_4.i.i536.v.us.us.us.i.i, 0.000000e+00, !dbg !6537
  %153 = fcmp ule float %_0.i367.us.us.us.i.i, 0.000000e+00, !dbg !6540
  %154 = select i1 %153, i1 %_3.i.i613.us.us.us.i.i, i1 false, !dbg !6542
  %_0.i348.us.us.us.i.i = select i1 %154, float %_4.i.i536.v.us.us.us.i.i, float 0.000000e+00, !dbg !6542
  %_86.i108.us.us.us.i.i = load float, ptr %103, align 4, !dbg !6543, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i210.us.us.us.i.i = fcmp ule float %_0.i348.us.us.us.i.i, %_86.i108.us.us.us.i.i, !dbg !6544
  %_4.i341.us.us.us.i.i = select i1 %_3.i210.us.us.us.i.i, i32 %_88.i111667.i.i, i32 %_87.i110666.i.i, !dbg !6546
  %_0.i342.us.us.us.i.i = bitcast i32 %_4.i341.us.us.us.i.i to float, !dbg !6548
  %_0.i286.us.us.us.i.i = fsub float %_0.i348.us.us.us.i.i, %_86.i108.us.us.us.i.i, !dbg !6550
  %_4.i256.us.us.us.i.i = fmul float %_0.i286.us.us.us.i.i, %_0.i342.us.us.us.i.i, !dbg !6552
  %_0.i257.us.us.us.i.i = fadd float %_86.i108.us.us.us.i.i, %_4.i256.us.us.us.i.i, !dbg !6552
  %155 = tail call noundef float @llvm.fabs.f32(float %_0.i257.us.us.us.i.i), !dbg !6554
  %156 = fcmp uge float %155, 0x3BC79CA100000000, !dbg !6557
  %_0.i326.us.us.us.i.i = select i1 %156, float %_0.i257.us.us.us.i.i, float 0.000000e+00, !dbg !6559
  store float %_0.i326.us.us.us.i.i, ptr %103, align 4, !dbg !6560, !alias.scope !6396, !noalias !6399
  %_0.i271.us.us.us.i.i = fmul float %_0.i326.us.us.us.i.i, 0x3FC542A5A0000000, !dbg !6561
  %_3.i.i522.us.us.us.inv.i.i = fcmp ogt float %_0.i271.us.us.us.i.i, -1.260000e+02, !dbg !6564
  %_0.i.i529.us.us.us.i.i = select i1 %_3.i.i522.us.us.us.inv.i.i, float %_0.i271.us.us.us.i.i, float -1.260000e+02, !dbg !6564
  %_3.i.i605.us.us.us.inv.i.i = fcmp olt float %_0.i.i529.us.us.us.i.i, 1.270000e+02, !dbg !6568
  %_0.i.i612.us.us.us.i.i = select i1 %_3.i.i605.us.us.us.inv.i.i, float %_0.i.i529.us.us.us.i.i, float 1.270000e+02, !dbg !6568
  %157 = tail call noundef float @llvm.floor.f32(float %_0.i.i612.us.us.us.i.i), !dbg !6571
  %_0.i285.us.us.us.i.i = fsub float %_0.i.i612.us.us.us.i.i, %157, !dbg !6575
  %_12.i.us.us.us.i.i = load float, ptr %126, align 4, !dbg !6577, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i238.us.us.us.i.i = fcmp ule float %_12.i.us.us.us.i.i, 0.000000e+00, !dbg !6585
  %_3.i198.us.us.us.i.i = fcmp une float %_12.i.us.us.us.i.i, 1.000000e+00, !dbg !6587
  %_16.i.us.us.us.i.i = load float, ptr %data.i.i.i.i, align 4, !dbg !6589, !alias.scope !6580, !noalias !6583, !noundef !12
  %_17.i.us.us.us.i.i = load float, ptr %127, align 4, !dbg !6590, !alias.scope !6580, !noalias !6583, !noundef !12
  %_0.i255.us.us.us.i.i = fadd float %_16.i.us.us.us.i.i, %_17.i.us.us.us.i.i, !dbg !6591
  %_20.i673676.us.us.us.i.i = load float, ptr %128, align 4, !dbg !6593, !alias.scope !6580, !noalias !6583, !noundef !12
  %158 = select i1 %_3.i198.us.us.us.i.i, float %_0.i255.us.us.us.i.i, float %_20.i673676.us.us.us.i.i, !dbg !6594
  %_0.i480.us.us.us.i.i = select i1 %_3.i238.us.us.us.i.i, float %_16.i.us.us.us.i.i, float %158, !dbg !6596
  store float %_0.i480.us.us.us.i.i, ptr %data.i.i.i.i, align 4, !dbg !6598, !alias.scope !6580, !noalias !6583
  %_0.i473.us.us.us.i.i = select i1 %_3.i198.us.us.us.i.i, float %_17.i.us.us.us.i.i, float 0.000000e+00, !dbg !6599
  store float %_0.i473.us.us.us.i.i, ptr %127, align 4, !dbg !6601, !alias.scope !6580, !noalias !6583
  %_0.i297.us.us.us.i.i = fadd float %_12.i.us.us.us.i.i, -1.000000e+00, !dbg !6602
  %_4.i466.v.us.us.us.i.i = select i1 %_3.i238.us.us.us.i.i, float %_12.i.us.us.us.i.i, float %_0.i297.us.us.us.i.i, !dbg !6604
  store float %_4.i466.v.us.us.us.i.i, ptr %126, align 4, !dbg !6606, !alias.scope !6580, !noalias !6583
  %_12.i.us.us.us.1.i.i = load float, ptr %129, align 4, !dbg !6577, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i238.us.us.us.1.i.i = fcmp ule float %_12.i.us.us.us.1.i.i, 0.000000e+00, !dbg !6585
  %_3.i198.us.us.us.1.i.i = fcmp une float %_12.i.us.us.us.1.i.i, 1.000000e+00, !dbg !6587
  %_16.i.us.us.us.1.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.1.i.i, align 4, !dbg !6589, !alias.scope !6580, !noalias !6583, !noundef !12
  %_17.i.us.us.us.1.i.i = load float, ptr %130, align 4, !dbg !6590, !alias.scope !6580, !noalias !6583, !noundef !12
  %_0.i255.us.us.us.1.i.i = fadd float %_16.i.us.us.us.1.i.i, %_17.i.us.us.us.1.i.i, !dbg !6591
  %_20.i673676.us.us.us.1.i.i = load float, ptr %131, align 4, !dbg !6593, !alias.scope !6580, !noalias !6583, !noundef !12
  %159 = select i1 %_3.i198.us.us.us.1.i.i, float %_0.i255.us.us.us.1.i.i, float %_20.i673676.us.us.us.1.i.i, !dbg !6594
  %_0.i480.us.us.us.1.i.i = select i1 %_3.i238.us.us.us.1.i.i, float %_16.i.us.us.us.1.i.i, float %159, !dbg !6596
  store float %_0.i480.us.us.us.1.i.i, ptr %iter1.sroa.0.0.ptr.i.us.1.i.i, align 4, !dbg !6598, !alias.scope !6580, !noalias !6583
  %_0.i473.us.us.us.1.i.i = select i1 %_3.i198.us.us.us.1.i.i, float %_17.i.us.us.us.1.i.i, float 0.000000e+00, !dbg !6599
  store float %_0.i473.us.us.us.1.i.i, ptr %130, align 4, !dbg !6601, !alias.scope !6580, !noalias !6583
  %_0.i297.us.us.us.1.i.i = fadd float %_12.i.us.us.us.1.i.i, -1.000000e+00, !dbg !6602
  %_4.i466.v.us.us.us.1.i.i = select i1 %_3.i238.us.us.us.1.i.i, float %_12.i.us.us.us.1.i.i, float %_0.i297.us.us.us.1.i.i, !dbg !6604
  store float %_4.i466.v.us.us.us.1.i.i, ptr %129, align 4, !dbg !6606, !alias.scope !6580, !noalias !6583
  %_12.i.us.us.us.2.i.i = load float, ptr %132, align 4, !dbg !6577, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i238.us.us.us.2.i.i = fcmp ule float %_12.i.us.us.us.2.i.i, 0.000000e+00, !dbg !6585
  %_3.i198.us.us.us.2.i.i = fcmp une float %_12.i.us.us.us.2.i.i, 1.000000e+00, !dbg !6587
  %_16.i.us.us.us.2.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.2.i.i, align 4, !dbg !6589, !alias.scope !6580, !noalias !6583, !noundef !12
  %_17.i.us.us.us.2.i.i = load float, ptr %133, align 4, !dbg !6590, !alias.scope !6580, !noalias !6583, !noundef !12
  %_0.i255.us.us.us.2.i.i = fadd float %_16.i.us.us.us.2.i.i, %_17.i.us.us.us.2.i.i, !dbg !6591
  %_20.i673676.us.us.us.2.i.i = load float, ptr %134, align 4, !dbg !6593, !alias.scope !6580, !noalias !6583, !noundef !12
  %160 = select i1 %_3.i198.us.us.us.2.i.i, float %_0.i255.us.us.us.2.i.i, float %_20.i673676.us.us.us.2.i.i, !dbg !6594
  %_0.i480.us.us.us.2.i.i = select i1 %_3.i238.us.us.us.2.i.i, float %_16.i.us.us.us.2.i.i, float %160, !dbg !6596
  store float %_0.i480.us.us.us.2.i.i, ptr %iter1.sroa.0.0.ptr.i.us.2.i.i, align 4, !dbg !6598, !alias.scope !6580, !noalias !6583
  %_0.i473.us.us.us.2.i.i = select i1 %_3.i198.us.us.us.2.i.i, float %_17.i.us.us.us.2.i.i, float 0.000000e+00, !dbg !6599
  store float %_0.i473.us.us.us.2.i.i, ptr %133, align 4, !dbg !6601, !alias.scope !6580, !noalias !6583
  %_0.i297.us.us.us.2.i.i = fadd float %_12.i.us.us.us.2.i.i, -1.000000e+00, !dbg !6602
  %_4.i466.v.us.us.us.2.i.i = select i1 %_3.i238.us.us.us.2.i.i, float %_12.i.us.us.us.2.i.i, float %_0.i297.us.us.us.2.i.i, !dbg !6604
  store float %_4.i466.v.us.us.us.2.i.i, ptr %132, align 4, !dbg !6606, !alias.scope !6580, !noalias !6583
  %_12.i.us.us.us.3.i.i = load float, ptr %135, align 4, !dbg !6577, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i238.us.us.us.3.i.i = fcmp ule float %_12.i.us.us.us.3.i.i, 0.000000e+00, !dbg !6585
  %_3.i198.us.us.us.3.i.i = fcmp une float %_12.i.us.us.us.3.i.i, 1.000000e+00, !dbg !6587
  %_16.i.us.us.us.3.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.3.i.i, align 4, !dbg !6589, !alias.scope !6580, !noalias !6583, !noundef !12
  %_17.i.us.us.us.3.i.i = load float, ptr %136, align 4, !dbg !6590, !alias.scope !6580, !noalias !6583, !noundef !12
  %_0.i255.us.us.us.3.i.i = fadd float %_16.i.us.us.us.3.i.i, %_17.i.us.us.us.3.i.i, !dbg !6591
  %_20.i673676.us.us.us.3.i.i = load float, ptr %137, align 4, !dbg !6593, !alias.scope !6580, !noalias !6583, !noundef !12
  %161 = select i1 %_3.i198.us.us.us.3.i.i, float %_0.i255.us.us.us.3.i.i, float %_20.i673676.us.us.us.3.i.i, !dbg !6594
  %_0.i480.us.us.us.3.i.i = select i1 %_3.i238.us.us.us.3.i.i, float %_16.i.us.us.us.3.i.i, float %161, !dbg !6596
  store float %_0.i480.us.us.us.3.i.i, ptr %iter1.sroa.0.0.ptr.i.us.3.i.i, align 4, !dbg !6598, !alias.scope !6580, !noalias !6583
  %_0.i473.us.us.us.3.i.i = select i1 %_3.i198.us.us.us.3.i.i, float %_17.i.us.us.us.3.i.i, float 0.000000e+00, !dbg !6599
  store float %_0.i473.us.us.us.3.i.i, ptr %136, align 4, !dbg !6601, !alias.scope !6580, !noalias !6583
  %_0.i297.us.us.us.3.i.i = fadd float %_12.i.us.us.us.3.i.i, -1.000000e+00, !dbg !6602
  %_4.i466.v.us.us.us.3.i.i = select i1 %_3.i238.us.us.us.3.i.i, float %_12.i.us.us.us.3.i.i, float %_0.i297.us.us.us.3.i.i, !dbg !6604
  store float %_4.i466.v.us.us.us.3.i.i, ptr %135, align 4, !dbg !6606, !alias.scope !6580, !noalias !6583
  %162 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le911.us.us.us.i.i), !dbg !6607
  %163 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.i.i), !dbg !6609
  %_3.i.i588.us.us.us.i.i = fcmp ule float %162, %163, !dbg !6611
  %_6.i.i590.us.us.us.i.i = bitcast float %162 to i32, !dbg !6614
  %_8.i.i592.us.us.us.i.i = bitcast float %163 to i32, !dbg !6617
  %_4.i.i595.us.us.us.i.i = select i1 %_3.i.i588.us.us.us.i.i, i32 %_8.i.i592.us.us.us.i.i, i32 %_6.i.i590.us.us.us.i.i, !dbg !6619
  %_4.i459.us.us.us.i.i = select i1 %_3.i236.i.i, i32 %_6.i.i590.us.us.us.i.i, i32 %_4.i.i595.us.us.us.i.i, !dbg !6620
  %_0.i281.us.us.us.i.i = fmul float %162, 5.000000e-01, !dbg !6622
  %_0.i280.us.us.us.i.i = fmul float %163, 5.000000e-01, !dbg !6624
  %_0.i254.us.us.us.i.i = fadd float %_0.i280.us.us.us.i.i, %_0.i281.us.us.us.i.i, !dbg !6626
  %_6.i447.us.us.us.i.i = bitcast float %_0.i254.us.us.us.i.i to i32, !dbg !6628
  %_4.i452.us.us.us.i.i = select i1 %_3.i234.i.i, i32 %_4.i459.us.us.us.i.i, i32 %_6.i447.us.us.us.i.i, !dbg !6631
  %_0.i453.us.us.us.i.i = bitcast i32 %_4.i452.us.us.us.i.i to float, !dbg !6632
  %_3.i.i580.us.us.us.i.i = fcmp ule float %_0.i453.us.us.us.i.i, 0x3E45798EE0000000, !dbg !6634
  %_4.i.i586.us.us.us.i.i = select i1 %_3.i.i580.us.us.us.i.i, i32 841731191, i32 %_4.i452.us.us.us.i.i, !dbg !6637
  %_0.i.i587.us.us.us.i.i = bitcast i32 %_4.i.i586.us.us.us.i.i to float, !dbg !6639
  %_3.i.i.us.us.us.i.i = fcmp ule float %_0.i.i587.us.us.us.i.i, 0x3810000000000000, !dbg !6641
  %_4.i.i.us.us.us.i.i = select i1 %_3.i.i.us.us.us.i.i, i32 8388608, i32 %_4.i.i586.us.us.us.i.i, !dbg !6646
  %_5.i314.us.us.us.i.i = and i32 %_4.i.i.us.us.us.i.i, 8388607, !dbg !6648
  %_4.i315.us.us.us.i.i = or disjoint i32 %_5.i314.us.us.us.i.i, 1065353216, !dbg !6648
  %significand.i.us.us.us.i.i = bitcast i32 %_4.i315.us.us.us.i.i to float, !dbg !6650
  %_0.i282.us.us.us.i.i = fadd float %significand.i.us.us.us.i.i, -1.000000e+00, !dbg !6652
  %_0.i261.us.us.us.i.i = fmul float %_0.i282.us.us.us.i.i, 0x3F9B17A960000000, !dbg !6654
  %164 = fsub float 0x3FBF9A8440000000, %_0.i261.us.us.us.i.i, !dbg !6656
  %_0.i261.us.us.us.1.i.i = fmul float %_0.i282.us.us.us.i.i, %164, !dbg !6654
  %_0.i245.us.us.us.1.i.i = fadd float %_0.i261.us.us.us.1.i.i, 0xBFD1E3F400000000, !dbg !6656
  %_0.i261.us.us.us.2.i.i = fmul float %_0.i282.us.us.us.i.i, %_0.i245.us.us.us.1.i.i, !dbg !6654
  %_0.i245.us.us.us.2.i.i = fadd float %_0.i261.us.us.us.2.i.i, 0x3FDD544F20000000, !dbg !6656
  %_0.i261.us.us.us.3.i.i = fmul float %_0.i282.us.us.us.i.i, %_0.i245.us.us.us.2.i.i, !dbg !6654
  %_0.i245.us.us.us.3.i.i = fadd float %_0.i261.us.us.us.3.i.i, 0xBFE6FC2A60000000, !dbg !6656
  %_0.i261.us.us.us.4.i.i = fmul float %_0.i282.us.us.us.i.i, %_0.i245.us.us.us.3.i.i, !dbg !6654
  %_0.i245.us.us.us.4.i.i = fadd float %_0.i261.us.us.us.4.i.i, 0x3FF714B2A0000000, !dbg !6656
  %_9.i.us.us.us.i.i = lshr i32 %_4.i.i.us.us.us.i.i, 23, !dbg !6658
  %_8.i316.us.us.us.i.i = or disjoint i32 %_9.i.us.us.us.i.i, 1258291200, !dbg !6658
  %_7.i.us.us.us.i.i = bitcast i32 %_8.i316.us.us.us.i.i to float, !dbg !6659
  %exponent.i.us.us.us.i.i = fadd float %_7.i.us.us.us.i.i, 0xC160000FE0000000, !dbg !6661
  %_0.i260.us.us.us.i.i = fmul float %_0.i282.us.us.us.i.i, %_0.i245.us.us.us.4.i.i, !dbg !6662
  %_0.i244.us.us.us.i.i = fadd float %exponent.i.us.us.us.i.i, %_0.i260.us.us.us.i.i, !dbg !6664
  %_0.i279.us.us.us.i.i = fmul float %_0.i244.us.us.us.i.i, 0x4018151820000000, !dbg !6666
  %_3.i.i637.us.us.us.inv.i.i = fcmp olt float %_0.i279.us.us.us.i.i, 2.400000e+01, !dbg !6668
  %_0.i.i644.us.us.us.i.i = select i1 %_3.i.i637.us.us.us.inv.i.i, float %_0.i279.us.us.us.i.i, float 2.400000e+01, !dbg !6668
  %_3.i.i572.us.us.us.inv.i.i = fcmp ogt float %_0.i.i644.us.us.us.i.i, -1.600000e+02, !dbg !6671
  %_0.i.i579.us.us.us.i.i = select i1 %_3.i.i572.us.us.us.inv.i.i, float %_0.i.i644.us.us.us.i.i, float -1.600000e+02, !dbg !6671
  %_55.i10.us.us.us.i.i = load float, ptr %106, align 4, !dbg !6674, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i232.us.us.us.i.i = fcmp ule float %_55.i10.us.us.us.i.i, 0.000000e+00, !dbg !6675
  %_3.i206.us.us.us.i.i = fcmp oge float %_0.i.i579.us.us.us.i.i, %_0.i480.us.us.us.i.i, !dbg !6677
  %_0.i296.us.us.us.i.i = fsub float %_0.i480.us.us.us.i.i, %_0.i480.us.us.us.3.i.i, !dbg !6679
  %_3.i204.us.us.us.i.i = fcmp oge float %_0.i.i579.us.us.us.i.i, %_0.i296.us.us.us.i.i, !dbg !6681
  %..i205.us.us.us.i.i = sext i1 %_3.i204.us.us.us.i.i to i32, !dbg !6683
  %_0.i500.us.us.us.i.i = sext i1 %_3.i206.us.us.us.i.i to i32, !dbg !6685
  %_0.i493.us.us.us.i.i = select i1 %_3.i232.us.us.us.i.i, i32 %_0.i500.us.us.us.i.i, i32 %..i205.us.us.us.i.i, !dbg !6685
  %_0.i504.us.us.us.i.i = xor i32 %..i205.us.us.us.i.i, -1, !dbg !6687
  %_67.i12.us.us.us.i.i = load float, ptr %109, align 4, !dbg !6689, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i230.us.us.us.i.i = fcmp ogt float %_67.i12.us.us.us.i.i, 0.000000e+00, !dbg !6690
  %_0.i499.us.us.us.i.i = select i1 %_3.i230.us.us.us.i.i, i32 %_0.i504.us.us.us.i.i, i32 0, !dbg !6692
  %_0.i498.us.us.us.i.i = select i1 %_3.i232.us.us.us.i.i, i32 0, i32 %_0.i499.us.us.us.i.i, !dbg !6694
  %_0.i492.us.us.us.i.i = or i32 %_0.i498.us.us.us.i.i, %_0.i493.us.us.us.i.i, !dbg !6696
  %_5.i442.us.us.us.i.i = and i32 %_0.i492.us.us.us.i.i, 1065353216, !dbg !6698
  %_0.i446.us.us.us.i.i = bitcast i32 %_5.i442.us.us.us.i.i to float, !dbg !6700
  %_0.i295.us.us.us.i.i = fadd float %_67.i12.us.us.us.i.i, -1.000000e+00, !dbg !6702
  %165 = trunc nsw i32 %_0.i498.us.us.us.i.i to i1, !dbg !6704
  %_4.i440.v.us.us.us.i.i = select i1 %165, float %_0.i295.us.us.us.i.i, float %_67.i12.us.us.us.i.i, !dbg !6704
  %166 = trunc nsw i32 %_0.i493.us.us.us.i.i to i1, !dbg !6706
  %_0.i434.us.us.us.i.i = select i1 %166, float %_71.i685686.i.i, float %_4.i440.v.us.us.us.i.i, !dbg !6706
  store float %_0.i434.us.us.us.i.i, ptr %109, align 4, !dbg !6708, !alias.scope !6580, !noalias !6583
  store i32 %_5.i442.us.us.us.i.i, ptr %106, align 4, !dbg !6709, !alias.scope !6580, !noalias !6583
  %_0.i294.us.us.us.i.i = fadd float %_0.i480.us.us.us.1.i.i, -1.000000e+00, !dbg !6710
  %_0.i293.us.us.us.i.i = fsub float %_0.i.i579.us.us.us.i.i, %_0.i480.us.us.us.i.i, !dbg !6712
  %_0.i278.us.us.us.i.i = fmul float %_0.i293.us.us.us.i.i, %_0.i294.us.us.us.i.i, !dbg !6714
  %167 = fneg float %_0.i480.us.us.us.2.i.i, !dbg !6716
  %_3.i.i563.inv.us.us.us.i.i = fcmp ogt float %_0.i278.us.us.us.i.i, %167, !dbg !6718
  %_4.i.i570.v.us.us.us.i.i = select i1 %_3.i.i563.inv.us.us.us.i.i, float %_0.i278.us.us.us.i.i, float %167, !dbg !6718
  %_3.i.i629.us.us.us.i.i = fcmp olt float %_4.i.i570.v.us.us.us.i.i, 0.000000e+00, !dbg !6721
  %168 = fcmp ule float %_0.i446.us.us.us.i.i, 0.000000e+00, !dbg !6724
  %169 = select i1 %168, i1 %_3.i.i629.us.us.us.i.i, i1 false, !dbg !6726
  %_0.i427.us.us.us.i.i = select i1 %169, float %_4.i.i570.v.us.us.us.i.i, float 0.000000e+00, !dbg !6726
  %_86.i20.us.us.us.i.i = load float, ptr %111, align 4, !dbg !6727, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i226.us.us.us.i.i = fcmp ule float %_0.i427.us.us.us.i.i, %_86.i20.us.us.us.i.i, !dbg !6728
  %_4.i421.us.us.us.i.i = select i1 %_3.i226.us.us.us.i.i, i32 %_88.i23689.i.i, i32 %_87.i22688.i.i, !dbg !6730
  %_0.i.us.us.us.i.i = bitcast i32 %_4.i421.us.us.us.i.i to float, !dbg !6732
  %_0.i292.us.us.us.i.i = fsub float %_0.i427.us.us.us.i.i, %_86.i20.us.us.us.i.i, !dbg !6734
  %_4.i258.us.us.us.i.i = fmul float %_0.i292.us.us.us.i.i, %_0.i.us.us.us.i.i, !dbg !6736
  %_0.i259.us.us.us.i.i = fadd float %_86.i20.us.us.us.i.i, %_4.i258.us.us.us.i.i, !dbg !6736
  %170 = tail call noundef float @llvm.fabs.f32(float %_0.i259.us.us.us.i.i), !dbg !6738
  %171 = fcmp uge float %170, 0x3BC79CA100000000, !dbg !6741
  %_0.i330.us.us.us.i.i = select i1 %171, float %_0.i259.us.us.us.i.i, float 0.000000e+00, !dbg !6743
  store float %_0.i330.us.us.us.i.i, ptr %111, align 4, !dbg !6744, !alias.scope !6580, !noalias !6583
  %_0.i277.us.us.us.i.i = fmul float %_0.i330.us.us.us.i.i, 0x3FC542A5A0000000, !dbg !6745
  %_3.i.i514.us.us.us.inv.i.i = fcmp ogt float %_0.i277.us.us.us.i.i, -1.260000e+02, !dbg !6748
  %_0.i.i521.us.us.us.i.i = select i1 %_3.i.i514.us.us.us.inv.i.i, float %_0.i277.us.us.us.i.i, float -1.260000e+02, !dbg !6748
  %_3.i.i597.us.us.us.inv.i.i = fcmp olt float %_0.i.i521.us.us.us.i.i, 1.270000e+02, !dbg !6752
  %_0.i.i604.us.us.us.i.i = select i1 %_3.i.i597.us.us.us.inv.i.i, float %_0.i.i521.us.us.us.i.i, float 1.270000e+02, !dbg !6752
  %172 = tail call noundef float @llvm.floor.f32(float %_0.i.i604.us.us.us.i.i), !dbg !6755
  %_0.i284.us.us.us.i.i = fsub float %_0.i.i604.us.us.us.i.i, %172, !dbg !6759
  %_0.i266.us.us.us.i.i = fmul float %_0.i284.us.us.us.i.i, 0x3F5E974FA0000000, !dbg !6761
  %_0.i249.us.us.us.i.i = fadd float %_0.i266.us.us.us.i.i, 0x3F82778560000000, !dbg !6763
  %_0.i266.us.us.us.1.i.i = fmul float %_0.i284.us.us.us.i.i, %_0.i249.us.us.us.i.i, !dbg !6761
  %_0.i249.us.us.us.1.i.i = fadd float %_0.i266.us.us.us.1.i.i, 0x3FAC91CE60000000, !dbg !6763
  %_0.i266.us.us.us.2.i.i = fmul float %_0.i284.us.us.us.i.i, %_0.i249.us.us.us.1.i.i, !dbg !6761
  %_0.i249.us.us.us.2.i.i = fadd float %_0.i266.us.us.us.2.i.i, 0x3FCEBDB560000000, !dbg !6763
  %_0.i266.us.us.us.3.i.i = fmul float %_0.i284.us.us.us.i.i, %_0.i249.us.us.us.2.i.i, !dbg !6761
  %_0.i249.us.us.us.3.i.i = fadd float %_0.i266.us.us.us.3.i.i, 0x3FE62E4BA0000000, !dbg !6763
  %_0.i269.us.us.us.i.i = fmul float %_0.i285.us.us.us.i.i, 0x3F5E974FA0000000, !dbg !6765
  %_0.i251.us.us.us.i.i = fadd float %_0.i269.us.us.us.i.i, 0x3F82778560000000, !dbg !6767
  %_0.i269.us.us.us.1.i.i = fmul float %_0.i285.us.us.us.i.i, %_0.i251.us.us.us.i.i, !dbg !6765
  %_0.i251.us.us.us.1.i.i = fadd float %_0.i269.us.us.us.1.i.i, 0x3FAC91CE60000000, !dbg !6767
  %_0.i269.us.us.us.2.i.i = fmul float %_0.i285.us.us.us.i.i, %_0.i251.us.us.us.1.i.i, !dbg !6765
  %_0.i251.us.us.us.2.i.i = fadd float %_0.i269.us.us.us.2.i.i, 0x3FCEBDB560000000, !dbg !6767
  %_0.i269.us.us.us.3.i.i = fmul float %_0.i285.us.us.us.i.i, %_0.i251.us.us.us.2.i.i, !dbg !6765
  %_0.i251.us.us.us.3.i.i = fadd float %_0.i269.us.us.us.3.i.i, 0x3FE62E4BA0000000, !dbg !6767
  %_0.i268.us.us.us.i.i = fmul float %_0.i285.us.us.us.i.i, %_0.i251.us.us.us.3.i.i, !dbg !6769
  %_0.i250.us.us.us.i.i = fadd float %_0.i268.us.us.us.i.i, 1.000000e+00, !dbg !6771
  %biased.i189.us.us.us.i.i = fadd float %157, 0x4160000FE0000000, !dbg !6773
  %_4.i190.us.us.us.i.i = bitcast float %biased.i189.us.us.us.i.i to i32, !dbg !6775
  %_3.i191.us.us.us.i.i = shl i32 %_4.i190.us.us.us.i.i, 23, !dbg !6777
  %_0.i192.us.us.us.i.i = bitcast i32 %_3.i191.us.us.us.i.i to float, !dbg !6778
  %_0.i267.us.us.us.i.i = fmul float %_0.i250.us.us.us.i.i, %_0.i192.us.us.us.i.i, !dbg !6780
  %_3.i193.us.us.us.i.i = fcmp une float %_0.i326.us.us.us.i.i, 0.000000e+00, !dbg !6782
  %_0.i488671.not.us.us.us.i.i = and i1 %_3.i208.i.i, %_3.i193.us.us.us.i.i, !dbg !6784
  %_0.i270.us.us.us.i.i = fmul float %_0.i309.us.us.us.i.i, %_0.i267.us.us.us.i.i, !dbg !6784
  %_4.i334.v.us.us.us.i.i = select i1 %_0.i488671.not.us.us.us.i.i, float %_0.i270.us.us.us.i.i, float %_0.i309.us.us.us.i.i, !dbg !6786
  %_0.i265.us.us.us.i.i = fmul float %_0.i284.us.us.us.i.i, %_0.i249.us.us.us.3.i.i, !dbg !6788
  %_0.i248.us.us.us.i.i = fadd float %_0.i265.us.us.us.i.i, 1.000000e+00, !dbg !6790
  %biased.i.us.us.us.i.i = fadd float %172, 0x4160000FE0000000, !dbg !6792
  %_4.i186.us.us.us.i.i = bitcast float %biased.i.us.us.us.i.i to i32, !dbg !6794
  %_3.i187.us.us.us.i.i = shl i32 %_4.i186.us.us.us.i.i, 23, !dbg !6796
  %_0.i188.us.us.us.i.i = bitcast i32 %_3.i187.us.us.us.i.i to float, !dbg !6797
  %_0.i264.us.us.us.i.i = fmul float %_0.i248.us.us.us.i.i, %_0.i188.us.us.us.i.i, !dbg !6799
  %_3.i196.us.us.us.i.i = fcmp une float %_0.i330.us.us.us.i.i, 0.000000e+00, !dbg !6801
  %_0.i491693.not.us.us.us.i.i = and i1 %_3.i224.i.i, %_3.i196.us.us.us.i.i, !dbg !6803
  %_0.i276.us.us.us.i.i = fmul float %_0.i307.us.us.us.i.i, %_0.i264.us.us.us.i.i, !dbg !6803
  %_4.i414.v.us.us.us.i.i = select i1 %_0.i491693.not.us.us.us.i.i, float %_0.i276.us.us.us.i.i, float %_0.i307.us.us.us.i.i, !dbg !6805
  store float %_4.i334.v.us.us.us.i.i, ptr %_123.i.us.us.us.i.i, align 4, !dbg !6807, !alias.scope !6810, !noalias !6319
  store float %_4.i414.v.us.us.us.i.i, ptr %_141.i.us.us.us.i.i, align 4, !dbg !6813, !alias.scope !6815, !noalias !6341
  %exitcond1587.not.i.i = icmp eq i64 %138, %..i.i, !dbg !6818
  br i1 %exitcond1587.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKb1_EB3_.exit.i, label %bb42.i.us.us.us.i.i, !dbg !6821

bb42.i.us.i.i:                                    ; preds = %bb18.i, %bb24.i.us.lr.ph.split.us.us.i.i
  %iter.sroa.0.0.i1043.us.i.i = phi i64 [ %173, %bb24.i.us.lr.ph.split.us.us.i.i ], [ 0, %bb18.i ]
  %173 = add nuw nsw i64 %iter.sroa.0.0.i1043.us.i.i, 1, !dbg !6277
  %_27.i.us.i.i = trunc i64 %iter.sroa.0.0.i1043.us.i.i to i32, !dbg !6291
  %now.i.us.i.i = add i32 %base.i.i.i, %_27.i.us.i.i, !dbg !6294
  %_30.i.us.i.i = and i32 %now.i.us.i.i, %_52.i.i, !dbg !6297
  %_29.i.us.i.i = zext i32 %_30.i.us.i.i to i64, !dbg !6299
  %_123.i.us.i.i = getelementptr inbounds nuw float, ptr %_14.0, i64 %iter.sroa.0.0.i1043.us.i.i, !dbg !6300
  %_124.not.not.i.us.i.i = icmp ugt i64 %_58.1.i.i, %_29.i.us.i.i, !dbg !6309
  br i1 %_124.not.not.i.us.i.i, label %bb45.i.us.i.i, label %bb46.i.i.i, !dbg !6309, !prof !2704

bb45.i.us.i.i:                                    ; preds = %bb42.i.us.i.i
  %_0.i313.us.i.i = load float, ptr %_123.i.us.i.i, align 4, !dbg !6314, !alias.scope !6316, !noalias !6319, !noundef !12
  %_133.i.us.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_29.i.us.i.i, !dbg !6320
  store float %_0.i313.us.i.i, ptr %_133.i.us.i.i, align 4, !dbg !6324, !alias.scope !6326, !noalias !6274
  %_141.i.us.i.i = getelementptr inbounds nuw float, ptr %_13.0, i64 %iter.sroa.0.0.i1043.us.i.i, !dbg !6329
  %_142.not.not.i.us.i.i = icmp ugt i64 %_60.1.i.i, %_29.i.us.i.i, !dbg !6822
  br i1 %_142.not.not.i.us.i.i, label %bb50.i.us.i.i, label %bb51.i.i.i, !dbg !6822, !prof !2704

bb50.i.us.i.i:                                    ; preds = %bb45.i.us.i.i
  %_0.i311.us.i.i = load float, ptr %_141.i.us.i.i, align 4, !dbg !6336, !alias.scope !6338, !noalias !6341, !noundef !12
  %_149.i.us.i.i = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_29.i.us.i.i, !dbg !6342
  store float %_0.i311.us.i.i, ptr %_149.i.us.i.i, align 4, !dbg !6349, !alias.scope !6351, !noalias !6274
  %_52.i.us.i.i = sub i32 %now.i.us.i.i, %_53.i.i, !dbg !6354
  %_51.i.us.i.i = and i32 %_52.i.us.i.i, %_52.i.i, !dbg !6357
  %_50.i.us.i.i = zext i32 %_51.i.us.i.i to i64, !dbg !6358
  %_182.not.not.i.us.i.i = icmp ugt i64 %_58.1.i.i, %_50.i.us.i.i, !dbg !6359
  br i1 %_182.not.not.i.us.i.i, label %bb60.i.us.i.i, label %bb61.i.i.i, !dbg !6359, !prof !2704

bb60.i.us.i.i:                                    ; preds = %bb50.i.us.i.i
  %_189.i.us.i.i = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_50.i.us.i.i, !dbg !6364
  %_0.i309.us.i.i = load float, ptr %_189.i.us.i.i, align 4, !dbg !6368, !alias.scope !6370, !noalias !6274, !noundef !12
  %_190.not.not.i.us.i.i = icmp ugt i64 %_60.1.i.i, %_50.i.us.i.i, !dbg !6823
  br i1 %_190.not.not.i.us.i.i, label %bb63.i.us.i.i, label %bb64.i.i.i, !dbg !6823, !prof !2704

bb63.i.us.i.i:                                    ; preds = %bb60.i.us.i.i
  %_195.i.us.i.i = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_50.i.us.i.i, !dbg !6373
  %_0.i307.us.i.i = load float, ptr %_195.i.us.i.i, align 4, !dbg !6381, !alias.scope !6383, !noalias !6274, !noundef !12
  %_66.i.us.i.i = sub i32 %now.i.us.i.i, %_67.i.i.i
  %_65.i.us.i.i = and i32 %_66.i.us.i.i, %_52.i.i
  %_64.i.us.i.i = zext i32 %_65.i.us.i.i to i64
  %_74.i.us.i.i = sub i32 %now.i.us.i.i, %_75.i.i.i
  %_73.i.us.i.i = and i32 %_74.i.us.i.i, %_52.i.i
  %_72.i.us.i.i = zext i32 %_73.i.us.i.i to i64
  %_80.i.us.i.i = icmp ugt i64 %_58.1.i.i, %_64.i.us.i.i
  %174 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_64.i.us.i.i
  %175 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_64.i.us.i.i
  %_86.i.us.i.i = icmp ugt i64 %_60.1.i.i, %_72.i.us.i.i
  %176 = getelementptr inbounds nuw float, ptr %_60.0.i.i, i64 %_72.i.us.i.i
  %177 = getelementptr inbounds nuw float, ptr %_58.0.i.i, i64 %_72.i.us.i.i
  br i1 %_80.i.us.i.i, label %bb63.i.split.us.us.i.i, label %bb63.i.split.i.i

bb63.i.split.us.us.i.i:                           ; preds = %bb63.i.us.i.i
  %_84.i.us.i.i = icmp ugt i64 %_60.1.i.i, %_64.i.us.i.i
  br i1 %_84.i.us.i.i, label %bb24.i.us.lr.ph.us.i.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i.i, !dbg !6824

bb24.i.us.lr.ph.us.i.i:                           ; preds = %bb63.i.split.us.us.i.i
  br i1 %_86.i.us.i.i, label %bb24.i.us.lr.ph.split.us.us.i.i, label %bb24.i.us.lr.ph.split.i.i

bb24.i.us.lr.ph.split.us.us.i.i:                  ; preds = %bb24.i.us.lr.ph.us.i.i
  %_82.i.us.us.i.i = load float, ptr %175, align 4, !noalias !6274, !noundef !12
  %_87.i.us.us.us.i.i = load float, ptr %177, align 4, !noalias !6274, !noundef !12
  %_85.i.us.us.le911.us.i.i = load float, ptr %176, align 4, !noalias !6274, !noundef !12
  %_78.i.us.le.us.i.i = load float, ptr %174, align 4, !noalias !6274, !noundef !12
  %_12.i37.us.i.i = load float, ptr %114, align 4, !dbg !6394, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i222.us.i.i = fcmp ule float %_12.i37.us.i.i, 0.000000e+00, !dbg !6401
  %_3.i194.us.i.i = fcmp une float %_12.i37.us.i.i, 1.000000e+00, !dbg !6403
  %_16.i40.us.i.i = load float, ptr %_10.i.i, align 4, !dbg !6405, !alias.scope !6396, !noalias !6399, !noundef !12
  %_17.i41.us.i.i = load float, ptr %115, align 4, !dbg !6406, !alias.scope !6396, !noalias !6399, !noundef !12
  %_0.i253.us.i.i = fadd float %_16.i40.us.i.i, %_17.i41.us.i.i, !dbg !6407
  %_20.i43651654.us.i.i = load float, ptr %116, align 4, !dbg !6409, !alias.scope !6396, !noalias !6399, !noundef !12
  %178 = select i1 %_3.i194.us.i.i, float %_0.i253.us.i.i, float %_20.i43651654.us.i.i, !dbg !6410
  %_0.i401.us.i.i = select i1 %_3.i222.us.i.i, float %_16.i40.us.i.i, float %178, !dbg !6412
  store float %_0.i401.us.i.i, ptr %_10.i.i, align 4, !dbg !6414, !alias.scope !6396, !noalias !6399
  %_0.i394.us.i.i = select i1 %_3.i194.us.i.i, float %_17.i41.us.i.i, float 0.000000e+00, !dbg !6415
  store float %_0.i394.us.i.i, ptr %115, align 4, !dbg !6417, !alias.scope !6396, !noalias !6399
  %_0.i291.us.i.i = fadd float %_12.i37.us.i.i, -1.000000e+00, !dbg !6418
  %_4.i387.v.us.i.i = select i1 %_3.i222.us.i.i, float %_12.i37.us.i.i, float %_0.i291.us.i.i, !dbg !6420
  store float %_4.i387.v.us.i.i, ptr %114, align 4, !dbg !6422, !alias.scope !6396, !noalias !6399
  %_12.i37.us.1.i.i = load float, ptr %117, align 4, !dbg !6394, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i222.us.1.i.i = fcmp ule float %_12.i37.us.1.i.i, 0.000000e+00, !dbg !6401
  %_3.i194.us.1.i.i = fcmp une float %_12.i37.us.1.i.i, 1.000000e+00, !dbg !6403
  %_16.i40.us.1.i.i = load float, ptr %iter1.sroa.0.0.ptr.i35.us.1.i.i, align 4, !dbg !6405, !alias.scope !6396, !noalias !6399, !noundef !12
  %_17.i41.us.1.i.i = load float, ptr %118, align 4, !dbg !6406, !alias.scope !6396, !noalias !6399, !noundef !12
  %_0.i253.us.1.i.i = fadd float %_16.i40.us.1.i.i, %_17.i41.us.1.i.i, !dbg !6407
  %_20.i43651654.us.1.i.i = load float, ptr %119, align 4, !dbg !6409, !alias.scope !6396, !noalias !6399, !noundef !12
  %179 = select i1 %_3.i194.us.1.i.i, float %_0.i253.us.1.i.i, float %_20.i43651654.us.1.i.i, !dbg !6410
  %_0.i401.us.1.i.i = select i1 %_3.i222.us.1.i.i, float %_16.i40.us.1.i.i, float %179, !dbg !6412
  store float %_0.i401.us.1.i.i, ptr %iter1.sroa.0.0.ptr.i35.us.1.i.i, align 4, !dbg !6414, !alias.scope !6396, !noalias !6399
  %_0.i394.us.1.i.i = select i1 %_3.i194.us.1.i.i, float %_17.i41.us.1.i.i, float 0.000000e+00, !dbg !6415
  store float %_0.i394.us.1.i.i, ptr %118, align 4, !dbg !6417, !alias.scope !6396, !noalias !6399
  %_0.i291.us.1.i.i = fadd float %_12.i37.us.1.i.i, -1.000000e+00, !dbg !6418
  %_4.i387.v.us.1.i.i = select i1 %_3.i222.us.1.i.i, float %_12.i37.us.1.i.i, float %_0.i291.us.1.i.i, !dbg !6420
  store float %_4.i387.v.us.1.i.i, ptr %117, align 4, !dbg !6422, !alias.scope !6396, !noalias !6399
  %_12.i37.us.2.i.i = load float, ptr %120, align 4, !dbg !6394, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i222.us.2.i.i = fcmp ule float %_12.i37.us.2.i.i, 0.000000e+00, !dbg !6401
  %_3.i194.us.2.i.i = fcmp une float %_12.i37.us.2.i.i, 1.000000e+00, !dbg !6403
  %_16.i40.us.2.i.i = load float, ptr %iter1.sroa.0.0.ptr.i35.us.2.i.i, align 4, !dbg !6405, !alias.scope !6396, !noalias !6399, !noundef !12
  %_17.i41.us.2.i.i = load float, ptr %121, align 4, !dbg !6406, !alias.scope !6396, !noalias !6399, !noundef !12
  %_0.i253.us.2.i.i = fadd float %_16.i40.us.2.i.i, %_17.i41.us.2.i.i, !dbg !6407
  %_20.i43651654.us.2.i.i = load float, ptr %122, align 4, !dbg !6409, !alias.scope !6396, !noalias !6399, !noundef !12
  %180 = select i1 %_3.i194.us.2.i.i, float %_0.i253.us.2.i.i, float %_20.i43651654.us.2.i.i, !dbg !6410
  %_0.i401.us.2.i.i = select i1 %_3.i222.us.2.i.i, float %_16.i40.us.2.i.i, float %180, !dbg !6412
  store float %_0.i401.us.2.i.i, ptr %iter1.sroa.0.0.ptr.i35.us.2.i.i, align 4, !dbg !6414, !alias.scope !6396, !noalias !6399
  %_0.i394.us.2.i.i = select i1 %_3.i194.us.2.i.i, float %_17.i41.us.2.i.i, float 0.000000e+00, !dbg !6415
  store float %_0.i394.us.2.i.i, ptr %121, align 4, !dbg !6417, !alias.scope !6396, !noalias !6399
  %_0.i291.us.2.i.i = fadd float %_12.i37.us.2.i.i, -1.000000e+00, !dbg !6418
  %_4.i387.v.us.2.i.i = select i1 %_3.i222.us.2.i.i, float %_12.i37.us.2.i.i, float %_0.i291.us.2.i.i, !dbg !6420
  store float %_4.i387.v.us.2.i.i, ptr %120, align 4, !dbg !6422, !alias.scope !6396, !noalias !6399
  %_12.i37.us.3.i.i = load float, ptr %123, align 4, !dbg !6394, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i222.us.3.i.i = fcmp ule float %_12.i37.us.3.i.i, 0.000000e+00, !dbg !6401
  %_3.i194.us.3.i.i = fcmp une float %_12.i37.us.3.i.i, 1.000000e+00, !dbg !6403
  %_16.i40.us.3.i.i = load float, ptr %iter1.sroa.0.0.ptr.i35.us.3.i.i, align 4, !dbg !6405, !alias.scope !6396, !noalias !6399, !noundef !12
  %_17.i41.us.3.i.i = load float, ptr %124, align 4, !dbg !6406, !alias.scope !6396, !noalias !6399, !noundef !12
  %_0.i253.us.3.i.i = fadd float %_16.i40.us.3.i.i, %_17.i41.us.3.i.i, !dbg !6407
  %_20.i43651654.us.3.i.i = load float, ptr %125, align 4, !dbg !6409, !alias.scope !6396, !noalias !6399, !noundef !12
  %181 = select i1 %_3.i194.us.3.i.i, float %_0.i253.us.3.i.i, float %_20.i43651654.us.3.i.i, !dbg !6410
  %_0.i401.us.3.i.i = select i1 %_3.i222.us.3.i.i, float %_16.i40.us.3.i.i, float %181, !dbg !6412
  store float %_0.i401.us.3.i.i, ptr %iter1.sroa.0.0.ptr.i35.us.3.i.i, align 4, !dbg !6414, !alias.scope !6396, !noalias !6399
  %_0.i394.us.3.i.i = select i1 %_3.i194.us.3.i.i, float %_17.i41.us.3.i.i, float 0.000000e+00, !dbg !6415
  store float %_0.i394.us.3.i.i, ptr %124, align 4, !dbg !6417, !alias.scope !6396, !noalias !6399
  %_0.i291.us.3.i.i = fadd float %_12.i37.us.3.i.i, -1.000000e+00, !dbg !6418
  %_4.i387.v.us.3.i.i = select i1 %_3.i222.us.3.i.i, float %_12.i37.us.3.i.i, float %_0.i291.us.3.i.i, !dbg !6420
  store float %_4.i387.v.us.3.i.i, ptr %123, align 4, !dbg !6422, !alias.scope !6396, !noalias !6399
  %182 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.i.i), !dbg !6423
  %183 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.i.i), !dbg !6425
  %_3.i.i554.us.i.i = fcmp ule float %182, %183, !dbg !6427
  %_6.i.i556.us.i.i = bitcast float %182 to i32, !dbg !6430
  %_8.i.i558.us.i.i = bitcast float %183 to i32, !dbg !6433
  %_4.i.i561.us.i.i = select i1 %_3.i.i554.us.i.i, i32 %_8.i.i558.us.i.i, i32 %_6.i.i556.us.i.i, !dbg !6435
  %_4.i380.us.i.i = select i1 %_3.i220.i.i, i32 %_6.i.i556.us.i.i, i32 %_4.i.i561.us.i.i, !dbg !6436
  %_0.i275.us.i.i = fmul float %182, 5.000000e-01, !dbg !6438
  %_0.i274.us.i.i = fmul float %183, 5.000000e-01, !dbg !6440
  %_0.i252.us.i.i = fadd float %_0.i274.us.i.i, %_0.i275.us.i.i, !dbg !6442
  %_6.i368.us.i.i = bitcast float %_0.i252.us.i.i to i32, !dbg !6444
  %_4.i373.us.i.i = select i1 %_3.i218.i.i, i32 %_4.i380.us.i.i, i32 %_6.i368.us.i.i, !dbg !6447
  %_0.i374.us.i.i = bitcast i32 %_4.i373.us.i.i to float, !dbg !6448
  %_3.i.i546.us.i.i = fcmp ule float %_0.i374.us.i.i, 0x3E45798EE0000000, !dbg !6450
  %_4.i.i552.us.i.i = select i1 %_3.i.i546.us.i.i, i32 841731191, i32 %_4.i373.us.i.i, !dbg !6453
  %_0.i.i553.us.i.i = bitcast i32 %_4.i.i552.us.i.i to float, !dbg !6455
  %_3.i.i506.us.i.i = fcmp ule float %_0.i.i553.us.i.i, 0x3810000000000000, !dbg !6457
  %_4.i.i512.us.i.i = select i1 %_3.i.i506.us.i.i, i32 8388608, i32 %_4.i.i552.us.i.i, !dbg !6462
  %_5.i318.us.i.i = and i32 %_4.i.i512.us.i.i, 8388607, !dbg !6464
  %_4.i319.us.i.i = or disjoint i32 %_5.i318.us.i.i, 1065353216, !dbg !6464
  %significand.i320.us.i.i = bitcast i32 %_4.i319.us.i.i to float, !dbg !6466
  %_0.i283.us.i.i = fadd float %significand.i320.us.i.i, -1.000000e+00, !dbg !6468
  %_0.i263.us.i.i = fmul float %_0.i283.us.i.i, 0x3F9B17A960000000, !dbg !6470
  %184 = fsub float 0x3FBF9A8440000000, %_0.i263.us.i.i, !dbg !6472
  %_0.i263.us.1.i.i = fmul float %_0.i283.us.i.i, %184, !dbg !6470
  %_0.i247.us.1.i.i = fadd float %_0.i263.us.1.i.i, 0xBFD1E3F400000000, !dbg !6472
  %_0.i263.us.2.i.i = fmul float %_0.i283.us.i.i, %_0.i247.us.1.i.i, !dbg !6470
  %_0.i247.us.2.i.i = fadd float %_0.i263.us.2.i.i, 0x3FDD544F20000000, !dbg !6472
  %_0.i263.us.3.i.i = fmul float %_0.i283.us.i.i, %_0.i247.us.2.i.i, !dbg !6470
  %_0.i247.us.3.i.i = fadd float %_0.i263.us.3.i.i, 0xBFE6FC2A60000000, !dbg !6472
  %_0.i263.us.4.i.i = fmul float %_0.i283.us.i.i, %_0.i247.us.3.i.i, !dbg !6470
  %_0.i247.us.4.i.i = fadd float %_0.i263.us.4.i.i, 0x3FF714B2A0000000, !dbg !6472
  %_9.i321.us.i.i = lshr i32 %_4.i.i512.us.i.i, 23, !dbg !6474
  %_8.i322.us.i.i = or disjoint i32 %_9.i321.us.i.i, 1258291200, !dbg !6474
  %_7.i323.us.i.i = bitcast i32 %_8.i322.us.i.i to float, !dbg !6475
  %exponent.i324.us.i.i = fadd float %_7.i323.us.i.i, 0xC160000FE0000000, !dbg !6477
  %_0.i262.us.i.i = fmul float %_0.i283.us.i.i, %_0.i247.us.4.i.i, !dbg !6478
  %_0.i246.us.i.i = fadd float %exponent.i324.us.i.i, %_0.i262.us.i.i, !dbg !6480
  %_0.i273.us.i.i = fmul float %_0.i246.us.i.i, 0x4018151820000000, !dbg !6482
  %_3.i.i621.us.inv.i.i = fcmp olt float %_0.i273.us.i.i, 2.400000e+01, !dbg !6484
  %_0.i.i628.us.i.i = select i1 %_3.i.i621.us.inv.i.i, float %_0.i273.us.i.i, float 2.400000e+01, !dbg !6484
  %_3.i.i538.us.inv.i.i = fcmp ogt float %_0.i.i628.us.i.i, -1.600000e+02, !dbg !6487
  %_0.i.i545.us.i.i = select i1 %_3.i.i538.us.inv.i.i, float %_0.i.i628.us.i.i, float -1.600000e+02, !dbg !6487
  %_55.i79.us.i.i = load float, ptr %98, align 4, !dbg !6490, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i216.us.i.i = fcmp ule float %_55.i79.us.i.i, 0.000000e+00, !dbg !6491
  %_3.i202.us.i.i = fcmp oge float %_0.i.i545.us.i.i, %_0.i401.us.i.i, !dbg !6493
  %_0.i290.us.i.i = fsub float %_0.i401.us.i.i, %_0.i401.us.3.i.i, !dbg !6495
  %_3.i200.us.i.i = fcmp oge float %_0.i.i545.us.i.i, %_0.i290.us.i.i, !dbg !6497
  %..i201.us.i.i = sext i1 %_3.i200.us.i.i to i32, !dbg !6499
  %_0.i496.us.i.i = sext i1 %_3.i202.us.i.i to i32, !dbg !6501
  %_0.i490.us.i.i = select i1 %_3.i216.us.i.i, i32 %_0.i496.us.i.i, i32 %..i201.us.i.i, !dbg !6501
  %_0.i502.us.i.i = xor i32 %..i201.us.i.i, -1, !dbg !6503
  %_67.i89.us.i.i = load float, ptr %101, align 4, !dbg !6505, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i214.us.i.i = fcmp ogt float %_67.i89.us.i.i, 0.000000e+00, !dbg !6506
  %_0.i495.us.i.i = select i1 %_3.i214.us.i.i, i32 %_0.i502.us.i.i, i32 0, !dbg !6508
  %_0.i494.us.i.i = select i1 %_3.i216.us.i.i, i32 0, i32 %_0.i495.us.i.i, !dbg !6510
  %_0.i489.us.i.i = or i32 %_0.i494.us.i.i, %_0.i490.us.i.i, !dbg !6512
  %_5.i363.us.i.i = and i32 %_0.i489.us.i.i, 1065353216, !dbg !6514
  %_0.i367.us.i.i = bitcast i32 %_5.i363.us.i.i to float, !dbg !6516
  %_0.i289.us.i.i = fadd float %_67.i89.us.i.i, -1.000000e+00, !dbg !6518
  %185 = trunc nsw i32 %_0.i494.us.i.i to i1, !dbg !6520
  %_4.i361.v.us.i.i = select i1 %185, float %_0.i289.us.i.i, float %_67.i89.us.i.i, !dbg !6520
  %186 = trunc nsw i32 %_0.i490.us.i.i to i1, !dbg !6522
  %_0.i355.us.i.i = select i1 %186, float %_71.i95663664.i.i, float %_4.i361.v.us.i.i, !dbg !6522
  store float %_0.i355.us.i.i, ptr %101, align 4, !dbg !6524, !alias.scope !6396, !noalias !6399
  store i32 %_5.i363.us.i.i, ptr %98, align 4, !dbg !6525, !alias.scope !6396, !noalias !6399
  %_0.i288.us.i.i = fadd float %_0.i401.us.1.i.i, -1.000000e+00, !dbg !6526
  %_0.i287.us.i.i = fsub float %_0.i.i545.us.i.i, %_0.i401.us.i.i, !dbg !6528
  %_0.i272.us.i.i = fmul float %_0.i288.us.i.i, %_0.i287.us.i.i, !dbg !6530
  %187 = fneg float %_0.i401.us.2.i.i, !dbg !6532
  %_3.i.i530.inv.us.i.i = fcmp ogt float %_0.i272.us.i.i, %187, !dbg !6534
  %_4.i.i536.v.us.i.i = select i1 %_3.i.i530.inv.us.i.i, float %_0.i272.us.i.i, float %187, !dbg !6534
  %_3.i.i613.us.i.i = fcmp olt float %_4.i.i536.v.us.i.i, 0.000000e+00, !dbg !6537
  %188 = fcmp ule float %_0.i367.us.i.i, 0.000000e+00, !dbg !6540
  %189 = select i1 %188, i1 %_3.i.i613.us.i.i, i1 false, !dbg !6542
  %_0.i348.us.i.i = select i1 %189, float %_4.i.i536.v.us.i.i, float 0.000000e+00, !dbg !6542
  %_86.i108.us.i.i = load float, ptr %103, align 4, !dbg !6543, !alias.scope !6396, !noalias !6399, !noundef !12
  %_3.i210.us.i.i = fcmp ule float %_0.i348.us.i.i, %_86.i108.us.i.i, !dbg !6544
  %_4.i341.us.i.i = select i1 %_3.i210.us.i.i, i32 %_88.i111667.i.i, i32 %_87.i110666.i.i, !dbg !6546
  %_0.i342.us.i.i = bitcast i32 %_4.i341.us.i.i to float, !dbg !6548
  %_0.i286.us.i.i = fsub float %_0.i348.us.i.i, %_86.i108.us.i.i, !dbg !6550
  %_4.i256.us.i.i = fmul float %_0.i286.us.i.i, %_0.i342.us.i.i, !dbg !6552
  %_0.i257.us.i.i = fadd float %_86.i108.us.i.i, %_4.i256.us.i.i, !dbg !6552
  %190 = tail call noundef float @llvm.fabs.f32(float %_0.i257.us.i.i), !dbg !6554
  %191 = fcmp uge float %190, 0x3BC79CA100000000, !dbg !6557
  %_0.i326.us.i.i = select i1 %191, float %_0.i257.us.i.i, float 0.000000e+00, !dbg !6559
  store float %_0.i326.us.i.i, ptr %103, align 4, !dbg !6560, !alias.scope !6396, !noalias !6399
  %_0.i271.us.i.i = fmul float %_0.i326.us.i.i, 0x3FC542A5A0000000, !dbg !6561
  %_3.i.i522.us.inv.i.i = fcmp ogt float %_0.i271.us.i.i, -1.260000e+02, !dbg !6564
  %_0.i.i529.us.i.i = select i1 %_3.i.i522.us.inv.i.i, float %_0.i271.us.i.i, float -1.260000e+02, !dbg !6564
  %_3.i.i605.us.inv.i.i = fcmp olt float %_0.i.i529.us.i.i, 1.270000e+02, !dbg !6568
  %_0.i.i612.us.i.i = select i1 %_3.i.i605.us.inv.i.i, float %_0.i.i529.us.i.i, float 1.270000e+02, !dbg !6568
  %192 = tail call noundef float @llvm.floor.f32(float %_0.i.i612.us.i.i), !dbg !6571
  %_0.i285.us.i.i = fsub float %_0.i.i612.us.i.i, %192, !dbg !6575
  %_12.i.us.i.i = load float, ptr %126, align 4, !dbg !6577, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i238.us.i.i = fcmp ule float %_12.i.us.i.i, 0.000000e+00, !dbg !6585
  %_3.i198.us.i.i = fcmp une float %_12.i.us.i.i, 1.000000e+00, !dbg !6587
  %_16.i.us.i.i = load float, ptr %data.i.i.i.i, align 4, !dbg !6589, !alias.scope !6580, !noalias !6583, !noundef !12
  %_17.i.us.i.i = load float, ptr %127, align 4, !dbg !6590, !alias.scope !6580, !noalias !6583, !noundef !12
  %_0.i255.us.i.i = fadd float %_16.i.us.i.i, %_17.i.us.i.i, !dbg !6591
  %_20.i673676.us.i.i = load float, ptr %128, align 4, !dbg !6593, !alias.scope !6580, !noalias !6583, !noundef !12
  %193 = select i1 %_3.i198.us.i.i, float %_0.i255.us.i.i, float %_20.i673676.us.i.i, !dbg !6594
  %_0.i480.us.i.i = select i1 %_3.i238.us.i.i, float %_16.i.us.i.i, float %193, !dbg !6596
  store float %_0.i480.us.i.i, ptr %data.i.i.i.i, align 4, !dbg !6598, !alias.scope !6580, !noalias !6583
  %_0.i473.us.i.i = select i1 %_3.i198.us.i.i, float %_17.i.us.i.i, float 0.000000e+00, !dbg !6599
  store float %_0.i473.us.i.i, ptr %127, align 4, !dbg !6601, !alias.scope !6580, !noalias !6583
  %_0.i297.us.i.i = fadd float %_12.i.us.i.i, -1.000000e+00, !dbg !6602
  %_4.i466.v.us.i.i = select i1 %_3.i238.us.i.i, float %_12.i.us.i.i, float %_0.i297.us.i.i, !dbg !6604
  store float %_4.i466.v.us.i.i, ptr %126, align 4, !dbg !6606, !alias.scope !6580, !noalias !6583
  %_12.i.us.1.i.i = load float, ptr %129, align 4, !dbg !6577, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i238.us.1.i.i = fcmp ule float %_12.i.us.1.i.i, 0.000000e+00, !dbg !6585
  %_3.i198.us.1.i.i = fcmp une float %_12.i.us.1.i.i, 1.000000e+00, !dbg !6587
  %_16.i.us.1.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.1.i.i, align 4, !dbg !6589, !alias.scope !6580, !noalias !6583, !noundef !12
  %_17.i.us.1.i.i = load float, ptr %130, align 4, !dbg !6590, !alias.scope !6580, !noalias !6583, !noundef !12
  %_0.i255.us.1.i.i = fadd float %_16.i.us.1.i.i, %_17.i.us.1.i.i, !dbg !6591
  %_20.i673676.us.1.i.i = load float, ptr %131, align 4, !dbg !6593, !alias.scope !6580, !noalias !6583, !noundef !12
  %194 = select i1 %_3.i198.us.1.i.i, float %_0.i255.us.1.i.i, float %_20.i673676.us.1.i.i, !dbg !6594
  %_0.i480.us.1.i.i = select i1 %_3.i238.us.1.i.i, float %_16.i.us.1.i.i, float %194, !dbg !6596
  store float %_0.i480.us.1.i.i, ptr %iter1.sroa.0.0.ptr.i.us.1.i.i, align 4, !dbg !6598, !alias.scope !6580, !noalias !6583
  %_0.i473.us.1.i.i = select i1 %_3.i198.us.1.i.i, float %_17.i.us.1.i.i, float 0.000000e+00, !dbg !6599
  store float %_0.i473.us.1.i.i, ptr %130, align 4, !dbg !6601, !alias.scope !6580, !noalias !6583
  %_0.i297.us.1.i.i = fadd float %_12.i.us.1.i.i, -1.000000e+00, !dbg !6602
  %_4.i466.v.us.1.i.i = select i1 %_3.i238.us.1.i.i, float %_12.i.us.1.i.i, float %_0.i297.us.1.i.i, !dbg !6604
  store float %_4.i466.v.us.1.i.i, ptr %129, align 4, !dbg !6606, !alias.scope !6580, !noalias !6583
  %_12.i.us.2.i.i = load float, ptr %132, align 4, !dbg !6577, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i238.us.2.i.i = fcmp ule float %_12.i.us.2.i.i, 0.000000e+00, !dbg !6585
  %_3.i198.us.2.i.i = fcmp une float %_12.i.us.2.i.i, 1.000000e+00, !dbg !6587
  %_16.i.us.2.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.2.i.i, align 4, !dbg !6589, !alias.scope !6580, !noalias !6583, !noundef !12
  %_17.i.us.2.i.i = load float, ptr %133, align 4, !dbg !6590, !alias.scope !6580, !noalias !6583, !noundef !12
  %_0.i255.us.2.i.i = fadd float %_16.i.us.2.i.i, %_17.i.us.2.i.i, !dbg !6591
  %_20.i673676.us.2.i.i = load float, ptr %134, align 4, !dbg !6593, !alias.scope !6580, !noalias !6583, !noundef !12
  %195 = select i1 %_3.i198.us.2.i.i, float %_0.i255.us.2.i.i, float %_20.i673676.us.2.i.i, !dbg !6594
  %_0.i480.us.2.i.i = select i1 %_3.i238.us.2.i.i, float %_16.i.us.2.i.i, float %195, !dbg !6596
  store float %_0.i480.us.2.i.i, ptr %iter1.sroa.0.0.ptr.i.us.2.i.i, align 4, !dbg !6598, !alias.scope !6580, !noalias !6583
  %_0.i473.us.2.i.i = select i1 %_3.i198.us.2.i.i, float %_17.i.us.2.i.i, float 0.000000e+00, !dbg !6599
  store float %_0.i473.us.2.i.i, ptr %133, align 4, !dbg !6601, !alias.scope !6580, !noalias !6583
  %_0.i297.us.2.i.i = fadd float %_12.i.us.2.i.i, -1.000000e+00, !dbg !6602
  %_4.i466.v.us.2.i.i = select i1 %_3.i238.us.2.i.i, float %_12.i.us.2.i.i, float %_0.i297.us.2.i.i, !dbg !6604
  store float %_4.i466.v.us.2.i.i, ptr %132, align 4, !dbg !6606, !alias.scope !6580, !noalias !6583
  %_12.i.us.3.i.i = load float, ptr %135, align 4, !dbg !6577, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i238.us.3.i.i = fcmp ule float %_12.i.us.3.i.i, 0.000000e+00, !dbg !6585
  %_3.i198.us.3.i.i = fcmp une float %_12.i.us.3.i.i, 1.000000e+00, !dbg !6587
  %_16.i.us.3.i.i = load float, ptr %iter1.sroa.0.0.ptr.i.us.3.i.i, align 4, !dbg !6589, !alias.scope !6580, !noalias !6583, !noundef !12
  %_17.i.us.3.i.i = load float, ptr %136, align 4, !dbg !6590, !alias.scope !6580, !noalias !6583, !noundef !12
  %_0.i255.us.3.i.i = fadd float %_16.i.us.3.i.i, %_17.i.us.3.i.i, !dbg !6591
  %_20.i673676.us.3.i.i = load float, ptr %137, align 4, !dbg !6593, !alias.scope !6580, !noalias !6583, !noundef !12
  %196 = select i1 %_3.i198.us.3.i.i, float %_0.i255.us.3.i.i, float %_20.i673676.us.3.i.i, !dbg !6594
  %_0.i480.us.3.i.i = select i1 %_3.i238.us.3.i.i, float %_16.i.us.3.i.i, float %196, !dbg !6596
  store float %_0.i480.us.3.i.i, ptr %iter1.sroa.0.0.ptr.i.us.3.i.i, align 4, !dbg !6598, !alias.scope !6580, !noalias !6583
  %_0.i473.us.3.i.i = select i1 %_3.i198.us.3.i.i, float %_17.i.us.3.i.i, float 0.000000e+00, !dbg !6599
  store float %_0.i473.us.3.i.i, ptr %136, align 4, !dbg !6601, !alias.scope !6580, !noalias !6583
  %_0.i297.us.3.i.i = fadd float %_12.i.us.3.i.i, -1.000000e+00, !dbg !6602
  %_4.i466.v.us.3.i.i = select i1 %_3.i238.us.3.i.i, float %_12.i.us.3.i.i, float %_0.i297.us.3.i.i, !dbg !6604
  store float %_4.i466.v.us.3.i.i, ptr %135, align 4, !dbg !6606, !alias.scope !6580, !noalias !6583
  %197 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le911.us.i.i), !dbg !6607
  %198 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.i.i), !dbg !6609
  %_3.i.i588.us.i.i = fcmp ule float %197, %198, !dbg !6611
  %_6.i.i590.us.i.i = bitcast float %197 to i32, !dbg !6614
  %_8.i.i592.us.i.i = bitcast float %198 to i32, !dbg !6617
  %_4.i.i595.us.i.i = select i1 %_3.i.i588.us.i.i, i32 %_8.i.i592.us.i.i, i32 %_6.i.i590.us.i.i, !dbg !6619
  %_4.i459.us.i.i = select i1 %_3.i236.i.i, i32 %_6.i.i590.us.i.i, i32 %_4.i.i595.us.i.i, !dbg !6620
  %_0.i281.us.i.i = fmul float %197, 5.000000e-01, !dbg !6622
  %_0.i280.us.i.i = fmul float %198, 5.000000e-01, !dbg !6624
  %_0.i254.us.i.i = fadd float %_0.i280.us.i.i, %_0.i281.us.i.i, !dbg !6626
  %_6.i447.us.i.i = bitcast float %_0.i254.us.i.i to i32, !dbg !6628
  %_4.i452.us.i.i = select i1 %_3.i234.i.i, i32 %_4.i459.us.i.i, i32 %_6.i447.us.i.i, !dbg !6631
  %_0.i453.us.i.i = bitcast i32 %_4.i452.us.i.i to float, !dbg !6632
  %_3.i.i580.us.i.i = fcmp ule float %_0.i453.us.i.i, 0x3E45798EE0000000, !dbg !6634
  %_4.i.i586.us.i.i = select i1 %_3.i.i580.us.i.i, i32 841731191, i32 %_4.i452.us.i.i, !dbg !6637
  %_0.i.i587.us.i.i = bitcast i32 %_4.i.i586.us.i.i to float, !dbg !6639
  %_3.i.i.us.i.i = fcmp ule float %_0.i.i587.us.i.i, 0x3810000000000000, !dbg !6641
  %_4.i.i.us.i.i = select i1 %_3.i.i.us.i.i, i32 8388608, i32 %_4.i.i586.us.i.i, !dbg !6646
  %_5.i314.us.i.i = and i32 %_4.i.i.us.i.i, 8388607, !dbg !6648
  %_4.i315.us.i.i = or disjoint i32 %_5.i314.us.i.i, 1065353216, !dbg !6648
  %significand.i.us.i.i = bitcast i32 %_4.i315.us.i.i to float, !dbg !6650
  %_0.i282.us.i.i = fadd float %significand.i.us.i.i, -1.000000e+00, !dbg !6652
  %_0.i261.us.i.i = fmul float %_0.i282.us.i.i, 0x3F9B17A960000000, !dbg !6654
  %199 = fsub float 0x3FBF9A8440000000, %_0.i261.us.i.i, !dbg !6656
  %_0.i261.us.1.i.i = fmul float %_0.i282.us.i.i, %199, !dbg !6654
  %_0.i245.us.1.i.i = fadd float %_0.i261.us.1.i.i, 0xBFD1E3F400000000, !dbg !6656
  %_0.i261.us.2.i.i = fmul float %_0.i282.us.i.i, %_0.i245.us.1.i.i, !dbg !6654
  %_0.i245.us.2.i.i = fadd float %_0.i261.us.2.i.i, 0x3FDD544F20000000, !dbg !6656
  %_0.i261.us.3.i.i = fmul float %_0.i282.us.i.i, %_0.i245.us.2.i.i, !dbg !6654
  %_0.i245.us.3.i.i = fadd float %_0.i261.us.3.i.i, 0xBFE6FC2A60000000, !dbg !6656
  %_0.i261.us.4.i.i = fmul float %_0.i282.us.i.i, %_0.i245.us.3.i.i, !dbg !6654
  %_0.i245.us.4.i.i = fadd float %_0.i261.us.4.i.i, 0x3FF714B2A0000000, !dbg !6656
  %_9.i.us.i.i = lshr i32 %_4.i.i.us.i.i, 23, !dbg !6658
  %_8.i316.us.i.i = or disjoint i32 %_9.i.us.i.i, 1258291200, !dbg !6658
  %_7.i.us.i.i = bitcast i32 %_8.i316.us.i.i to float, !dbg !6659
  %exponent.i.us.i.i = fadd float %_7.i.us.i.i, 0xC160000FE0000000, !dbg !6661
  %_0.i260.us.i.i = fmul float %_0.i282.us.i.i, %_0.i245.us.4.i.i, !dbg !6662
  %_0.i244.us.i.i = fadd float %exponent.i.us.i.i, %_0.i260.us.i.i, !dbg !6664
  %_0.i279.us.i.i = fmul float %_0.i244.us.i.i, 0x4018151820000000, !dbg !6666
  %_3.i.i637.us.inv.i.i = fcmp olt float %_0.i279.us.i.i, 2.400000e+01, !dbg !6668
  %_0.i.i644.us.i.i = select i1 %_3.i.i637.us.inv.i.i, float %_0.i279.us.i.i, float 2.400000e+01, !dbg !6668
  %_3.i.i572.us.inv.i.i = fcmp ogt float %_0.i.i644.us.i.i, -1.600000e+02, !dbg !6671
  %_0.i.i579.us.i.i = select i1 %_3.i.i572.us.inv.i.i, float %_0.i.i644.us.i.i, float -1.600000e+02, !dbg !6671
  %_55.i10.us.i.i = load float, ptr %106, align 4, !dbg !6674, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i232.us.i.i = fcmp ule float %_55.i10.us.i.i, 0.000000e+00, !dbg !6675
  %_3.i206.us.i.i = fcmp oge float %_0.i.i579.us.i.i, %_0.i480.us.i.i, !dbg !6677
  %_0.i296.us.i.i = fsub float %_0.i480.us.i.i, %_0.i480.us.3.i.i, !dbg !6679
  %_3.i204.us.i.i = fcmp oge float %_0.i.i579.us.i.i, %_0.i296.us.i.i, !dbg !6681
  %..i205.us.i.i = sext i1 %_3.i204.us.i.i to i32, !dbg !6683
  %_0.i500.us.i.i = sext i1 %_3.i206.us.i.i to i32, !dbg !6685
  %_0.i493.us.i.i = select i1 %_3.i232.us.i.i, i32 %_0.i500.us.i.i, i32 %..i205.us.i.i, !dbg !6685
  %_0.i504.us.i.i = xor i32 %..i205.us.i.i, -1, !dbg !6687
  %_67.i12.us.i.i = load float, ptr %109, align 4, !dbg !6689, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i230.us.i.i = fcmp ogt float %_67.i12.us.i.i, 0.000000e+00, !dbg !6690
  %_0.i499.us.i.i = select i1 %_3.i230.us.i.i, i32 %_0.i504.us.i.i, i32 0, !dbg !6692
  %_0.i498.us.i.i = select i1 %_3.i232.us.i.i, i32 0, i32 %_0.i499.us.i.i, !dbg !6694
  %_0.i492.us.i.i = or i32 %_0.i498.us.i.i, %_0.i493.us.i.i, !dbg !6696
  %_5.i442.us.i.i = and i32 %_0.i492.us.i.i, 1065353216, !dbg !6698
  %_0.i446.us.i.i = bitcast i32 %_5.i442.us.i.i to float, !dbg !6700
  %_0.i295.us.i.i = fadd float %_67.i12.us.i.i, -1.000000e+00, !dbg !6702
  %200 = trunc nsw i32 %_0.i498.us.i.i to i1, !dbg !6704
  %_4.i440.v.us.i.i = select i1 %200, float %_0.i295.us.i.i, float %_67.i12.us.i.i, !dbg !6704
  %201 = trunc nsw i32 %_0.i493.us.i.i to i1, !dbg !6706
  %_0.i434.us.i.i = select i1 %201, float %_71.i685686.i.i, float %_4.i440.v.us.i.i, !dbg !6706
  store float %_0.i434.us.i.i, ptr %109, align 4, !dbg !6708, !alias.scope !6580, !noalias !6583
  store i32 %_5.i442.us.i.i, ptr %106, align 4, !dbg !6709, !alias.scope !6580, !noalias !6583
  %_0.i294.us.i.i = fadd float %_0.i480.us.1.i.i, -1.000000e+00, !dbg !6710
  %_0.i293.us.i.i = fsub float %_0.i.i579.us.i.i, %_0.i480.us.i.i, !dbg !6712
  %_0.i278.us.i.i = fmul float %_0.i293.us.i.i, %_0.i294.us.i.i, !dbg !6714
  %202 = fneg float %_0.i480.us.2.i.i, !dbg !6716
  %_3.i.i563.inv.us.i.i = fcmp ogt float %_0.i278.us.i.i, %202, !dbg !6718
  %_4.i.i570.v.us.i.i = select i1 %_3.i.i563.inv.us.i.i, float %_0.i278.us.i.i, float %202, !dbg !6718
  %_3.i.i629.us.i.i = fcmp olt float %_4.i.i570.v.us.i.i, 0.000000e+00, !dbg !6721
  %203 = fcmp ule float %_0.i446.us.i.i, 0.000000e+00, !dbg !6724
  %204 = select i1 %203, i1 %_3.i.i629.us.i.i, i1 false, !dbg !6726
  %_0.i427.us.i.i = select i1 %204, float %_4.i.i570.v.us.i.i, float 0.000000e+00, !dbg !6726
  %_86.i20.us.i.i = load float, ptr %111, align 4, !dbg !6727, !alias.scope !6580, !noalias !6583, !noundef !12
  %_3.i226.us.i.i = fcmp ule float %_0.i427.us.i.i, %_86.i20.us.i.i, !dbg !6728
  %_4.i421.us.i.i = select i1 %_3.i226.us.i.i, i32 %_88.i23689.i.i, i32 %_87.i22688.i.i, !dbg !6730
  %_0.i.us.i.i = bitcast i32 %_4.i421.us.i.i to float, !dbg !6732
  %_0.i292.us.i.i = fsub float %_0.i427.us.i.i, %_86.i20.us.i.i, !dbg !6734
  %_4.i258.us.i.i = fmul float %_0.i292.us.i.i, %_0.i.us.i.i, !dbg !6736
  %_0.i259.us.i.i = fadd float %_86.i20.us.i.i, %_4.i258.us.i.i, !dbg !6736
  %205 = tail call noundef float @llvm.fabs.f32(float %_0.i259.us.i.i), !dbg !6738
  %206 = fcmp uge float %205, 0x3BC79CA100000000, !dbg !6741
  %_0.i330.us.i.i = select i1 %206, float %_0.i259.us.i.i, float 0.000000e+00, !dbg !6743
  store float %_0.i330.us.i.i, ptr %111, align 4, !dbg !6744, !alias.scope !6580, !noalias !6583
  %_0.i277.us.i.i = fmul float %_0.i330.us.i.i, 0x3FC542A5A0000000, !dbg !6745
  %_3.i.i514.us.inv.i.i = fcmp ogt float %_0.i277.us.i.i, -1.260000e+02, !dbg !6748
  %_0.i.i521.us.i.i = select i1 %_3.i.i514.us.inv.i.i, float %_0.i277.us.i.i, float -1.260000e+02, !dbg !6748
  %_3.i.i597.us.inv.i.i = fcmp olt float %_0.i.i521.us.i.i, 1.270000e+02, !dbg !6752
  %_0.i.i604.us.i.i = select i1 %_3.i.i597.us.inv.i.i, float %_0.i.i521.us.i.i, float 1.270000e+02, !dbg !6752
  %207 = tail call noundef float @llvm.floor.f32(float %_0.i.i604.us.i.i), !dbg !6755
  %_0.i284.us.i.i = fsub float %_0.i.i604.us.i.i, %207, !dbg !6759
  %_0.i266.us.i.i = fmul float %_0.i284.us.i.i, 0x3F5E974FA0000000, !dbg !6761
  %_0.i249.us.i.i = fadd float %_0.i266.us.i.i, 0x3F82778560000000, !dbg !6763
  %_0.i266.us.1.i.i = fmul float %_0.i284.us.i.i, %_0.i249.us.i.i, !dbg !6761
  %_0.i249.us.1.i.i = fadd float %_0.i266.us.1.i.i, 0x3FAC91CE60000000, !dbg !6763
  %_0.i266.us.2.i.i = fmul float %_0.i284.us.i.i, %_0.i249.us.1.i.i, !dbg !6761
  %_0.i249.us.2.i.i = fadd float %_0.i266.us.2.i.i, 0x3FCEBDB560000000, !dbg !6763
  %_0.i266.us.3.i.i = fmul float %_0.i284.us.i.i, %_0.i249.us.2.i.i, !dbg !6761
  %_0.i249.us.3.i.i = fadd float %_0.i266.us.3.i.i, 0x3FE62E4BA0000000, !dbg !6763
  %_0.i269.us.i.i = fmul float %_0.i285.us.i.i, 0x3F5E974FA0000000, !dbg !6765
  %_0.i251.us.i.i = fadd float %_0.i269.us.i.i, 0x3F82778560000000, !dbg !6767
  %_0.i269.us.1.i.i = fmul float %_0.i285.us.i.i, %_0.i251.us.i.i, !dbg !6765
  %_0.i251.us.1.i.i = fadd float %_0.i269.us.1.i.i, 0x3FAC91CE60000000, !dbg !6767
  %_0.i269.us.2.i.i = fmul float %_0.i285.us.i.i, %_0.i251.us.1.i.i, !dbg !6765
  %_0.i251.us.2.i.i = fadd float %_0.i269.us.2.i.i, 0x3FCEBDB560000000, !dbg !6767
  %_0.i269.us.3.i.i = fmul float %_0.i285.us.i.i, %_0.i251.us.2.i.i, !dbg !6765
  %_0.i251.us.3.i.i = fadd float %_0.i269.us.3.i.i, 0x3FE62E4BA0000000, !dbg !6767
  %_0.i268.us.i.i = fmul float %_0.i285.us.i.i, %_0.i251.us.3.i.i, !dbg !6769
  %_0.i250.us.i.i = fadd float %_0.i268.us.i.i, 1.000000e+00, !dbg !6771
  %biased.i189.us.i.i = fadd float %192, 0x4160000FE0000000, !dbg !6773
  %_4.i190.us.i.i = bitcast float %biased.i189.us.i.i to i32, !dbg !6775
  %_3.i191.us.i.i = shl i32 %_4.i190.us.i.i, 23, !dbg !6777
  %_0.i192.us.i.i = bitcast i32 %_3.i191.us.i.i to float, !dbg !6778
  %_0.i267.us.i.i = fmul float %_0.i250.us.i.i, %_0.i192.us.i.i, !dbg !6780
  %_3.i193.us.i.i = fcmp une float %_0.i326.us.i.i, 0.000000e+00, !dbg !6782
  %_0.i488671.not.us.i.i = and i1 %_3.i208.i.i, %_3.i193.us.i.i, !dbg !6784
  %_0.i270.us.i.i = fmul float %_0.i309.us.i.i, %_0.i267.us.i.i, !dbg !6784
  %_4.i334.v.us.i.i = select i1 %_0.i488671.not.us.i.i, float %_0.i270.us.i.i, float %_0.i309.us.i.i, !dbg !6786
  %_0.i265.us.i.i = fmul float %_0.i284.us.i.i, %_0.i249.us.3.i.i, !dbg !6788
  %_0.i248.us.i.i = fadd float %_0.i265.us.i.i, 1.000000e+00, !dbg !6790
  %biased.i.us.i.i = fadd float %207, 0x4160000FE0000000, !dbg !6792
  %_4.i186.us.i.i = bitcast float %biased.i.us.i.i to i32, !dbg !6794
  %_3.i187.us.i.i = shl i32 %_4.i186.us.i.i, 23, !dbg !6796
  %_0.i188.us.i.i = bitcast i32 %_3.i187.us.i.i to float, !dbg !6797
  %_0.i264.us.i.i = fmul float %_0.i248.us.i.i, %_0.i188.us.i.i, !dbg !6799
  %_3.i196.us.i.i = fcmp une float %_0.i330.us.i.i, 0.000000e+00, !dbg !6801
  %_0.i491693.not.us.i.i = and i1 %_3.i224.i.i, %_3.i196.us.i.i, !dbg !6803
  %_0.i276.us.i.i = fmul float %_0.i307.us.i.i, %_0.i264.us.i.i, !dbg !6803
  %_4.i414.v.us.i.i = select i1 %_0.i491693.not.us.i.i, float %_0.i276.us.i.i, float %_0.i307.us.i.i, !dbg !6805
  store float %_4.i334.v.us.i.i, ptr %_123.i.us.i.i, align 4, !dbg !6807, !alias.scope !6810, !noalias !6319
  store float %_4.i414.v.us.i.i, ptr %_141.i.us.i.i, align 4, !dbg !6813, !alias.scope !6815, !noalias !6341
  %exitcond1589.not.i.i = icmp eq i64 %173, %..i.i, !dbg !6818
  br i1 %exitcond1589.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKb1_EB3_.exit.i, label %bb42.i.us.i.i, !dbg !6821, !llvm.loop !6825

bb46.i.i.i:                                       ; preds = %bb42.i.us.us.us.i.i, %bb42.i.us.i.i
  %.us-phi1088.i.i = phi i64 [ %_29.i.us.i.i, %bb42.i.us.i.i ], [ %_29.i.us.us.us.i.i, %bb42.i.us.us.us.i.i ]
  %_36.i.le1028.i.i = add nuw nsw i64 %.us-phi1088.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1088.i.i, i64 noundef %_36.i.le1028.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cad5a7b24766ffcdda9cbfe066eb4078) #24, !dbg !6826, !noalias !6274
  unreachable, !dbg !6826

bb51.i.i.i:                                       ; preds = %bb45.i.us.i.i
  %_36.i.le.i.i = add nuw nsw i64 %_29.i.us.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_29.i.us.i.i, i64 noundef %_36.i.le.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_72e24c5578221343e8a784545a3910d2) #24, !dbg !6827, !noalias !6274
  unreachable, !dbg !6827

bb61.i.i.i:                                       ; preds = %bb45.i.us.us.us.i.i, %bb50.i.us.i.i
  %.us-phi1100.i.i = phi i64 [ %_50.i.us.i.i, %bb50.i.us.i.i ], [ %_50.i.us.us.us.i.i, %bb45.i.us.us.us.i.i ]
  %_55.i.le1026.i.i = add nuw nsw i64 %.us-phi1100.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1100.i.i, i64 noundef %_55.i.le1026.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba99eeeb3482270ebe1c674b4013dd6a) #24, !dbg !6828, !noalias !6274
  unreachable, !dbg !6828

bb64.i.i.i:                                       ; preds = %bb60.i.us.i.i
  %_55.i.le.i.i = add nuw nsw i64 %_50.i.us.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_50.i.us.i.i, i64 noundef %_55.i.le.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f4c27429e4885e1b619f1e4a3587acc) #24, !dbg !6829, !noalias !6274
  unreachable, !dbg !6829

bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i.i: ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_72.i.us.us.us.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5ae5decb60097d7e1183cb43cae6b3f) #24, !dbg !6386, !noalias !6274
  unreachable, !dbg !6386

bb24.i.us.lr.ph.split.i.i:                        ; preds = %bb63.i.split.us.us.us.us.i.i, %bb24.i.us.lr.ph.us.i.i
  %.us-phi1141.i.i = phi i64 [ %_72.i.us.i.i, %bb24.i.us.lr.ph.us.i.i ], [ %_72.i.us.us.us.i.i, %bb63.i.split.us.us.us.us.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1141.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5833dd3f10f38ea96ec24c6054ff083c) #24, !dbg !6830, !noalias !6274
  unreachable, !dbg !6830

bb63.i.split.us.panic20.i.split.us_crit_edge.i.i: ; preds = %bb63.i.split.us.us.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_64.i.us.i.i, i64 noundef %_60.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30bf8a301493a607abcc78dbf93977e0) #24, !dbg !6824, !noalias !6274
  unreachable, !dbg !6824

bb63.i.split.i.i:                                 ; preds = %bb60.i.us.us.us.i.i, %bb63.i.us.i.i
  %.us-phi1113.i.i = phi i64 [ %_64.i.us.i.i, %bb63.i.us.i.i ], [ %_64.i.us.us.us.i.i, %bb60.i.us.us.us.i.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1113.i.i, i64 noundef %_58.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b53d553f9945f7702333f7af20204159) #24, !dbg !6831, !noalias !6274
  unreachable, !dbg !6831

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKb1_EB3_.exit.i: ; preds = %bb28.i.us.us.lr.ph.us.us.us.i.i, %bb24.i.us.lr.ph.split.us.us.i.i
  %_108.i.i.i = trunc nuw i64 %..i.i to i32, !dbg !6832
  %_107.i.i.i = add i32 %base.i.i.i, %_108.i.i.i, !dbg !6833
  store i32 %_107.i.i.i, ptr %_51.i.i, align 4, !dbg !6835, !alias.scope !6255, !noalias !6274
  br label %bb4.i4, !dbg !6836

bb7.i5:                                           ; preds = %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i, %bb4.i4
  %_27.i = trunc nuw i64 %..i.i to i32, !dbg !6837
  %208 = load i32, ptr %44, align 4, !dbg !6838, !alias.scope !6131, !noalias !6134, !noundef !12
  %209 = sub i32 %208, %_27.i, !dbg !6838
  store i32 %209, ptr %44, align 4, !dbg !6838, !alias.scope !6131, !noalias !6134
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6839), !dbg !6842
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6843), !dbg !6842
  call void @llvm.lifetime.start.p0(ptr nonnull %iter.i.i), !dbg !6845, !noalias !6850
  %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 16, !dbg !6845
  store ptr %_14.0, ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, align 8, !dbg !6845, !noalias !6850
  %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 24, !dbg !6845
  store i64 %_14.1, ptr %_7.sroa.0.sroa.4.0.iter.sroa_idx.i.i, align 8, !dbg !6845, !noalias !6850
  %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 32, !dbg !6845
  store ptr %_13.0, ptr %_7.sroa.0.sroa.5.0.iter.sroa_idx.i.i, align 8, !dbg !6845, !noalias !6850
  %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 40, !dbg !6845
  store i64 %_13.1, ptr %_7.sroa.0.sroa.6.0.iter.sroa_idx.i.i, align 8, !dbg !6845, !noalias !6850
  %_7178.not.i.i = icmp eq i64 %_14.1, 0
  %210 = getelementptr inbounds nuw i8, ptr %self, i64 1200
  %211 = getelementptr inbounds nuw i8, ptr %self, i64 104
  %212 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %213 = getelementptr inbounds nuw i8, ptr %self, i64 944
  %214 = getelementptr inbounds nuw i8, ptr %self, i64 72
  %sample_rate.i.i.i.i = load i32, ptr %214, align 8, !alias.scope !6853, !noalias !6854
  %_32.i.i.i.i = uitofp i32 %sample_rate.i.i.i.i to double
  %215 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %216 = getelementptr inbounds nuw i8, ptr %self, i64 744
  %217 = getelementptr inbounds nuw i8, ptr %self, i64 792
  %slots.i.i.i = load i64, ptr %210, align 8, !alias.scope !6853, !noalias !6854
  %_194.not.i.i.i = icmp eq i64 %slots.i.i.i, 0
  %_12.i.i.i.i = load i32, ptr %215, align 8, !alias.scope !6853, !noalias !6854
  br label %bb6.i5.i, !dbg !6855

bb6.i5.i:                                         ; preds = %bb1.backedge.i.i, %bb7.i5
  %counter.sroa.0.0.v.i.i.sroa.phi = phi ptr [ %reports.sroa.7, %bb7.i5 ], [ %reports.sroa.8, %bb1.backedge.i.i ]
  %_5.not.i.i.i.i.i = phi i1 [ false, %bb7.i5 ], [ true, %bb1.backedge.i.i ]
  %218 = phi i64 [ 0, %bb7.i5 ], [ 1, %bb1.backedge.i.i ]
  %self3.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<&mut [f32]>", ptr %_7.sroa.0.sroa.3.0.iter.sroa_idx.i.i, i64 %218, !dbg !6861
  %_14.0.i.i.i.i.i = load ptr, ptr %self3.i.i.i.i.i, align 8, !dbg !6866, !alias.scope !6870, !noalias !6877, !nonnull !12, !align !3484, !noundef !12
  %219 = getelementptr inbounds nuw i8, ptr %self3.i.i.i.i.i, i64 8, !dbg !6866
  %_14.1.i.i.i.i.i = load i64, ptr %219, align 8, !dbg !6866, !alias.scope !6870, !noalias !6877, !noundef !12
  %220 = getelementptr inbounds nuw %"kernel::GateState<f32>", ptr %self, i64 %218, !dbg !6879
  %221 = getelementptr inbounds nuw i8, ptr %220, i64 864, !dbg !6879
  %_17.i.i = load float, ptr %221, align 4, !dbg !6879, !alias.scope !6853, !noalias !6854, !noundef !12
  %_22.not.i2171.i.i = icmp eq i64 %_14.1.i.i.i.i.i, 0, !dbg !6881
  br i1 %_22.not.i2171.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !6881

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %bb6.i5.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %ok.sroa.0.0.i2074.i.i = phi i32 [ %_0.i40.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ -1, %bb6.i5.i ]
  %iter.sroa.0.0.i1973.i.i = phi ptr [ %_27.i23.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %_14.0.i.i.i.i.i, %bb6.i5.i ]
  %iter.sroa.5.0.i1872.i.i = phi i64 [ %_28.i24.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ], [ %_14.1.i.i.i.i.i, %bb6.i5.i ]
  %_27.i23.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1973.i.i, i64 4, !dbg !6885
  %_28.i24.i.i = add i64 %iter.sroa.5.0.i1872.i.i, -1, !dbg !6888
  %_0.i35.i.i = load float, ptr %iter.sroa.0.0.i1973.i.i, align 4, !dbg !6889, !alias.scope !6891, !noalias !6894, !noundef !12
  %222 = tail call noundef float @llvm.fabs.f32(float %_0.i35.i.i), !dbg !6895
  %_3.i.i.i = fcmp olt float %222, 0x46293E5940000000, !dbg !6897
  %_0.i40.i.i = select i1 %_3.i.i.i, i32 %ok.sroa.0.0.i2074.i.i, i32 0, !dbg !6899
  %_22.not.i21.i.i = icmp eq i64 %_28.i24.i.i, 0, !dbg !6881
  br i1 %_22.not.i21.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !6881

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %223 = icmp eq i32 %_0.i40.i.i, -1, !dbg !6901
  %224 = tail call float @llvm.fabs.f32(float %_17.i.i)
  %_3.i33104.i.i = fcmp olt float %224, 0x46293E5940000000
  %or.cond.i.i = and i1 %_3.i33104.i.i, %223, !dbg !6903
  br i1 %or.cond.i.i, label %bb1.backedge.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, !dbg !6903

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i: ; preds = %bb6.i5.i
  %225 = tail call noundef float @llvm.fabs.f32(float %_17.i.i), !dbg !6904
  %_3.i33.i.i = fcmp olt float %225, 0x46293E5940000000, !dbg !6907
  br i1 %_3.i33.i.i, label %bb1.backedge.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !6909

bb1.backedge.i.i:                                 ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i
  br i1 %_5.not.i.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9run_blockB2_.exit, label %bb6.i5.i, !dbg !6855

bb24.loopexit.i.i.i:                              ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i
  %226 = and i32 %_0.i7.i.i.i, 1065353216, !dbg !6910
  %227 = icmp ne i32 %226, 1065353216, !dbg !6913
  %228 = zext i1 %227 to i32, !dbg !6913
  br label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i, !dbg !6914

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i
  %ok.sroa.0.017.i.i.i = phi i32 [ %_0.i7.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ], [ -1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i ]
  %iter.sroa.0.016.i.i.i = phi ptr [ %_45.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ], [ %_14.0.i.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i ]
  %iter.sroa.5.015.i.i.i = phi i64 [ %_46.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ], [ %_14.1.i.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECsdOTRa1MFkeb_13gate_expander.exit32.i.i ]
  %_45.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.016.i.i.i, i64 4, !dbg !6915
  %_46.i.i.i = add nsw i64 %iter.sroa.5.015.i.i.i, -1, !dbg !6920
  %_0.i.i.i.i = load float, ptr %iter.sroa.0.016.i.i.i, align 4, !dbg !6921, !alias.scope !6923, !noalias !6894, !noundef !12
  %229 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i.i), !dbg !6928
  %_3.i.i.i.i = fcmp olt float %229, 0x46293E5940000000, !dbg !6930
  %_0.i7.i.i.i = select i1 %_3.i.i.i.i, i32 %ok.sroa.0.017.i.i.i, i32 0, !dbg !6932
  %_40.not.i.i.i = icmp eq i64 %_46.i.i.i, 0, !dbg !6934
  br i1 %_40.not.i.i.i, label %bb24.loopexit.i.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, !dbg !6934

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i: ; preds = %bb24.loopexit.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i
  %.pre-phi.i = phi float [ %224, %bb24.loopexit.i.i.i ], [ %225, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i ], !dbg !6935
  %ok.sroa.0.0.lcssa.i.i.i = phi i32 [ %228, %bb24.loopexit.i.i.i ], [ 0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit39.preheader.i.i ], !dbg !6938
  %_3.i.i54.i.i = fcmp uge float %.pre-phi.i, 0x46293E5940000000, !dbg !6939
  %230 = zext i1 %_3.i.i54.i.i to i32, !dbg !6941
  %failed.i.i = or i32 %ok.sroa.0.0.lcssa.i.i.i, %230, !dbg !6942
  %231 = icmp eq i32 %failed.i.i, 0
  %ring.i.i.i = getelementptr inbounds nuw %Ring, ptr %211, i64 %218
  %232 = getelementptr inbounds nuw i8, ptr %ring.i.i.i, i64 8
  %233 = getelementptr inbounds nuw [8 x float], ptr %212, i64 %218
  %values.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %233, i64 4
  %values.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %233, i64 8
  %values.sroa.7.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %233, i64 12
  %values.sroa.8.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %233, i64 16
  %values.sroa.9.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %233, i64 20
  %values.sroa.10.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %233, i64 24
  %values.sroa.11.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %233, i64 28
  %234 = getelementptr inbounds nuw %LaneTiming, ptr %213, i64 %218
  %_7.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %234, i64 4
  %_7.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %234, i64 8
  %_7.sroa.6.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %234, i64 12
  %235 = getelementptr inbounds nuw %Ring, ptr %self, i64 %218
  %236 = getelementptr inbounds nuw i8, ptr %235, i64 136
  %237 = getelementptr inbounds nuw %"kernel::GateCoef<f32>", ptr %216, i64 %218
  %_19.i.i.i.i = getelementptr inbounds nuw i8, ptr %237, i64 8
  %_25.i.i.i.i = getelementptr inbounds nuw i8, ptr %237, i64 4
  %238 = getelementptr inbounds nuw %"kernel::GateState<f32>", ptr %217, i64 %218
  %_17.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 72
  %_19.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 64
  %_21.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 68
  %_37.i.i6.i = getelementptr inbounds nuw i8, ptr %238, i64 4
  %_39.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 8
  %_41.i.i7.i = getelementptr inbounds nuw i8, ptr %238, i64 12
  %iter.sroa.0.0.ptr24.1.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 16
  %_37.1.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 20
  %_39.1.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 24
  %_41.1.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 28
  %iter.sroa.0.0.ptr24.2.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 32
  %_37.2.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 36
  %_39.2.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 40
  %_41.2.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 44
  %iter.sroa.0.0.ptr24.3.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 48
  %_37.3.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 52
  %_39.3.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 56
  %_41.3.i.i.i = getelementptr inbounds nuw i8, ptr %238, i64 60
  br i1 %231, label %bb1.backedge.i.i, label %bb30.i.i

bb30.i.i:                                         ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECsdOTRa1MFkeb_13gate_expander.exit.i.i
  br i1 %_7178.not.i.i, label %bb33.i.i, label %bb32.i.i, !dbg !6943

bb33.i.i:                                         ; preds = %bb21.i.i, %bb30.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6954), !dbg !6957
  br i1 %_194.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.lr.ph.i.i.i, !dbg !6960

bb7.lr.ph.i.i.i:                                  ; preds = %bb33.i.i
  %_23.1.i.i.i = load i64, ptr %232, align 8, !alias.scope !6964, !noalias !6854, !noundef !12
  br label %bb7.i.i.i, !dbg !6960

bb7.i.i.i:                                        ; preds = %bb3.i.i.i, %bb7.lr.ph.i.i.i
  %iter.sroa.0.05.i.i.i = phi i64 [ 0, %bb7.lr.ph.i.i.i ], [ %239, %bb3.i.i.i ]
  %exitcond.not.i.i.i = icmp eq i64 %iter.sroa.0.05.i.i.i, %_23.1.i.i.i, !dbg !6965
  br i1 %exitcond.not.i.i.i, label %panic1.i.i.i, label %bb3.i.i.i, !dbg !6965

bb3.i.i.i:                                        ; preds = %bb7.i.i.i
  %_23.0.i.i.i = load ptr, ptr %ring.i.i.i, align 8, !dbg !6965, !alias.scope !6964, !noalias !6854, !nonnull !12, !noundef !12
  %239 = add nuw i64 %iter.sroa.0.05.i.i.i, 1, !dbg !6966
  %240 = getelementptr inbounds nuw float, ptr %_23.0.i.i.i, i64 %iter.sroa.0.05.i.i.i, !dbg !6965
  store float 0.000000e+00, ptr %240, align 4, !dbg !6965, !noalias !6969
  %exitcond7.not.i.i.i = icmp eq i64 %239, %slots.i.i.i, !dbg !6970
  br i1 %exitcond7.not.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i, label %bb7.i.i.i, !dbg !6960

panic1.i.i.i:                                     ; preds = %bb7.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_23.1.i.i.i, i64 noundef %_23.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f8f0512af3f0ba047152c3c522a44b59) #24, !dbg !6965, !noalias !6969
  unreachable, !dbg !6965

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i: ; preds = %bb3.i.i.i, %bb33.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6972), !dbg !6975
  %values.sroa.0.0.copyload.i.i.i = load float, ptr %233, align 8, !dbg !6976, !alias.scope !6978, !noalias !6854
  %values.sroa.5.0.copyload.i.i.i = load float, ptr %values.sroa.5.0..sroa_idx.i.i.i, align 4, !dbg !6976, !alias.scope !6978, !noalias !6854
  %values.sroa.6.0.copyload.i.i.i = load float, ptr %values.sroa.6.0..sroa_idx.i.i.i, align 8, !dbg !6976, !alias.scope !6978, !noalias !6854
  %values.sroa.7.0.copyload.i.i.i = load float, ptr %values.sroa.7.0..sroa_idx.i.i.i, align 4, !dbg !6976, !alias.scope !6978, !noalias !6854
  %values.sroa.8.0.copyload.i.i.i = load float, ptr %values.sroa.8.0..sroa_idx.i.i.i, align 8, !dbg !6976, !alias.scope !6978, !noalias !6854
  %values.sroa.9.0.copyload.i.i.i = load float, ptr %values.sroa.9.0..sroa_idx.i.i.i, align 4, !dbg !6976, !alias.scope !6978, !noalias !6854
  %values.sroa.10.0.copyload.i.i.i = load float, ptr %values.sroa.10.0..sroa_idx.i.i.i, align 8, !dbg !6976, !alias.scope !6978, !noalias !6854
  %values.sroa.11.0.copyload.i.i.i = load float, ptr %values.sroa.11.0..sroa_idx.i.i.i, align 4, !dbg !6976, !alias.scope !6978, !noalias !6854
  store float %values.sroa.11.0.copyload.i.i.i, ptr %234, align 8, !dbg !6979, !alias.scope !6978, !noalias !6854
  store float %values.sroa.8.0.copyload.i.i.i, ptr %_7.sroa.4.0..sroa_idx.i.i.i, align 4, !dbg !6979, !alias.scope !6978, !noalias !6854
  store float %values.sroa.9.0.copyload.i.i.i, ptr %_7.sroa.5.0..sroa_idx.i.i.i, align 8, !dbg !6979, !alias.scope !6978, !noalias !6854
  store float %values.sroa.10.0.copyload.i.i.i, ptr %_7.sroa.6.0..sroa_idx.i.i.i, align 4, !dbg !6979, !alias.scope !6978, !noalias !6854
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6980), !dbg !6983
  %_31.i.i.i.i = fpext float %values.sroa.11.0.copyload.i.i.i to double, !dbg !6984
  %_30.i.i.i.i = fmul double %_32.i.i.i.i, %_31.i.i.i.i, !dbg !6988
  %_29.i.i.i.i = fdiv double %_30.i.i.i.i, 1.000000e+03, !dbg !6988
  %_28.i.i.i.i = fadd double %_29.i.i.i.i, 5.000000e-01, !dbg !6989
  %241 = tail call double @llvm.floor.f64(double %_28.i.i.i.i), !dbg !6990
  %or.cond.i.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %241, i32 527), !dbg !6993
  %_35.i.i.i.i = fcmp ogt double %241, 0x41EFFFFFFFE00000
  %or.cond10.i.i.i.i = or i1 %or.cond.i.i.i.i, %_35.i.i.i.i, !dbg !6993
  br i1 %or.cond10.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i, label %bb19.i.i.i.i, !dbg !6993

bb19.i.i.i.i:                                     ; preds = %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i
  %_45.i.i.i.i = fpext float %values.sroa.9.0.copyload.i.i.i to double, !dbg !6994
  %_44.i.i.i.i = fmul double %_32.i.i.i.i, %_45.i.i.i.i, !dbg !6997
  %_43.i.i.i.i = fdiv double %_44.i.i.i.i, 1.000000e+03, !dbg !6997
  %_42.i.i.i.i = fadd double %_43.i.i.i.i, 5.000000e-01, !dbg !6998
  %242 = tail call double @llvm.floor.f64(double %_42.i.i.i.i), !dbg !6999
  %or.cond11.i.i.i.i = tail call i1 @llvm.is.fpclass.f64(double %242, i32 527), !dbg !7002
  %_48.i.i.i.i = fcmp ogt double %242, 0x41EFFFFFFFE00000
  %or.cond12.i.i.i.i = or i1 %or.cond11.i.i.i.i, %_48.i.i.i.i, !dbg !7002
  br i1 %or.cond12.i.i.i.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i, label %bb20.3.i.i.i, !dbg !7002

bb20.3.i.i.i:                                     ; preds = %bb19.i.i.i.i
  %_36.i.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %241), !dbg !7003
  %_49.i.i.i.i = tail call i32 @llvm.fptoui.sat.i32.f64(double %242), !dbg !7004
  %243 = tail call i32 @llvm.usub.sat.i32(i32 %_12.i.i.i.i, i32 %_36.i.i.i.i), !dbg !7005
  store i32 %243, ptr %236, align 4, !dbg !7005, !alias.scope !7006, !noalias !6854
  %_20.i.i.i.i = uitofp i32 %_49.i.i.i.i to float, !dbg !7007
  store float %_20.i.i.i.i, ptr %_19.i.i.i.i, align 4, !dbg !7008, !alias.scope !7010, !noalias !6854
; call effect_runtime::envelope::attack_release_coefficient
  %_23.i.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.8.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i.i) #23, !dbg !7013, !noalias !7014
  store float %_23.i.i.i.i, ptr %237, align 4, !dbg !7015, !alias.scope !7017, !noalias !6854
; call effect_runtime::envelope::attack_release_coefficient
  %_26.i.i.i.i = tail call noundef float @_RNvNtCseSJggkR25Fr_14effect_runtime8envelope26attack_release_coefficient(float noundef %values.sroa.10.0.copyload.i.i.i, i32 noundef %sample_rate.i.i.i.i) #23, !dbg !7020, !noalias !7014
  store float %_26.i.i.i.i, ptr %_25.i.i.i.i, align 4, !dbg !7021, !alias.scope !7023, !noalias !6854
  store float 0.000000e+00, ptr %_17.i.i.i, align 4, !dbg !7026, !alias.scope !7028, !noalias !6854
  store float 1.000000e+00, ptr %_19.i.i.i, align 4, !dbg !7031, !alias.scope !7033, !noalias !6854
  store float %_20.i.i.i.i, ptr %_21.i.i.i, align 4, !dbg !7036, !alias.scope !7038, !noalias !6854
  store float %values.sroa.0.0.copyload.i.i.i, ptr %238, align 4, !dbg !7041, !alias.scope !7043, !noalias !6854
  store float %values.sroa.0.0.copyload.i.i.i, ptr %_37.i.i6.i, align 4, !dbg !7046, !alias.scope !7048, !noalias !6854
  store float 0.000000e+00, ptr %_39.i.i.i, align 4, !dbg !7051, !alias.scope !7053, !noalias !6854
  store float 0.000000e+00, ptr %_41.i.i7.i, align 4, !dbg !7056, !alias.scope !7058, !noalias !6854
  store float %values.sroa.5.0.copyload.i.i.i, ptr %iter.sroa.0.0.ptr24.1.i.i.i, align 4, !dbg !7041, !alias.scope !7043, !noalias !6854
  store float %values.sroa.5.0.copyload.i.i.i, ptr %_37.1.i.i.i, align 4, !dbg !7046, !alias.scope !7048, !noalias !6854
  store float 0.000000e+00, ptr %_39.1.i.i.i, align 4, !dbg !7051, !alias.scope !7053, !noalias !6854
  store float 0.000000e+00, ptr %_41.1.i.i.i, align 4, !dbg !7056, !alias.scope !7058, !noalias !6854
  store float %values.sroa.6.0.copyload.i.i.i, ptr %iter.sroa.0.0.ptr24.2.i.i.i, align 4, !dbg !7041, !alias.scope !7043, !noalias !6854
  store float %values.sroa.6.0.copyload.i.i.i, ptr %_37.2.i.i.i, align 4, !dbg !7046, !alias.scope !7048, !noalias !6854
  store float 0.000000e+00, ptr %_39.2.i.i.i, align 4, !dbg !7051, !alias.scope !7053, !noalias !6854
  store float 0.000000e+00, ptr %_41.2.i.i.i, align 4, !dbg !7056, !alias.scope !7058, !noalias !6854
  store float %values.sroa.7.0.copyload.i.i.i, ptr %iter.sroa.0.0.ptr24.3.i.i.i, align 4, !dbg !7041, !alias.scope !7043, !noalias !6854
  store float %values.sroa.7.0.copyload.i.i.i, ptr %_37.3.i.i.i, align 4, !dbg !7046, !alias.scope !7048, !noalias !6854
  store float 0.000000e+00, ptr %_39.3.i.i.i, align 4, !dbg !7051, !alias.scope !7053, !noalias !6854
  store float 0.000000e+00, ptr %_41.3.i.i.i, align 4, !dbg !7056, !alias.scope !7058, !noalias !6854
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i, !dbg !7061

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9seed_laneB2_.exit.i.i: ; preds = %bb20.3.i.i.i, %bb19.i.i.i.i, %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E16clear_lane_ringsB2_.exit.i.i
  %_46.i.i = load i64, ptr %counter.sroa.0.0.v.i.i.sroa.phi, align 8, !dbg !7062, !alias.scope !7065, !noalias !7066, !noundef !12
  %244 = tail call i64 @llvm.uadd.sat.i64(i64 %_46.i.i, i64 %_14.1), !dbg !7067
  store i64 %244, ptr %counter.sroa.0.0.v.i.i.sroa.phi, align 8, !dbg !7070, !alias.scope !7065, !noalias !7066
  br label %bb1.backedge.i.i, !dbg !6855

bb32.i.i:                                         ; preds = %bb30.i.i, %bb21.i.i
  %iter2.sroa.0.079.i.i = phi i64 [ %245, %bb21.i.i ], [ 0, %bb30.i.i ]
  %exitcond.not.i8.i = icmp eq i64 %iter2.sroa.0.079.i.i, %_14.1.i.i.i.i.i, !dbg !7071
  br i1 %exitcond.not.i8.i, label %panic4.i.i, label %bb21.i.i, !dbg !7071

bb21.i.i:                                         ; preds = %bb32.i.i
  %245 = add nuw i64 %iter2.sroa.0.079.i.i, 1, !dbg !7073
  %246 = getelementptr inbounds nuw float, ptr %_14.0.i.i.i.i.i, i64 %iter2.sroa.0.079.i.i, !dbg !7071
  store float 0.000000e+00, ptr %246, align 4, !dbg !7071, !noalias !6894
  %exitcond99.not.i.i = icmp eq i64 %245, %_14.1, !dbg !7081
  br i1 %exitcond99.not.i.i, label %bb33.i.i, label %bb32.i.i, !dbg !6943

panic4.i.i:                                       ; preds = %bb32.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_14.1.i.i.i.i.i, i64 noundef %_14.1.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30d570589940b68d7f77af9df77eb2ee) #24, !dbg !7071, !noalias !6894
  unreachable, !dbg !7071

bb20.i:                                           ; preds = %bb4.i4
  %_60.i = icmp samesign ugt i64 %..i.i, %_13.1, !dbg !7085
  br i1 %_60.i, label %bb26.i, label %bb27.i, !dbg !7085, !prof !180

bb27.i:                                           ; preds = %bb20.i
  %_59.i = getelementptr inbounds nuw float, ptr %_14.0, i64 %..i.i, !dbg !7093
  %_55.i = sub i64 %_14.1, %..i.i, !dbg !7101
  %_63.i = sub nuw nsw i64 %_13.1, %..i.i, !dbg !7102
  %_67.i = getelementptr inbounds nuw float, ptr %_13.0, i64 %..i.i, !dbg !7103
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7108), !dbg !7111
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7112), !dbg !7111
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7114), !dbg !7111
  %_10.i17.i = getelementptr inbounds nuw i8, ptr %self, i64 792, !dbg !7116
  %data.i.i.i18.i = getelementptr inbounds nuw i8, ptr %self, i64 868, !dbg !7119
  %_15.i19.i = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !7124
  %data.i.i551.i.i = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !7126
  %247 = getelementptr inbounds nuw i8, ptr %self, i64 744, !dbg !7131
  %_32.i20.i = getelementptr inbounds nuw i8, ptr %self, i64 768, !dbg !7135
  %_58.0.i21.i = load ptr, ptr %_15.i19.i, align 8, !dbg !7136, !alias.scope !7137, !noalias !7138, !nonnull !12, !noundef !12
  %248 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !7136
  %_58.1.i22.i = load i64, ptr %248, align 8, !dbg !7136, !alias.scope !7137, !noalias !7138, !noundef !12
  %_45.i23.i = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !7140
  %_60.0.i24.i = load ptr, ptr %data.i.i551.i.i, align 8, !dbg !7141, !alias.scope !7137, !noalias !7138, !nonnull !12, !noundef !12
  %249 = getelementptr inbounds nuw i8, ptr %self, i64 176, !dbg !7141
  %_60.1.i25.i = load i64, ptr %249, align 8, !dbg !7141, !alias.scope !7137, !noalias !7138, !noundef !12
  %_50.i26.i = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !7142
  %_51.i27.i = getelementptr inbounds nuw i8, ptr %self, i64 1208, !dbg !7143
  %250 = getelementptr inbounds nuw i8, ptr %self, i64 1212, !dbg !7144
  %_52.i28.i = load i32, ptr %250, align 4, !dbg !7144, !alias.scope !7137, !noalias !7138, !noundef !12
  %251 = getelementptr inbounds nuw i8, ptr %self, i64 1216, !dbg !7145
  %_53.i29.i = load i32, ptr %251, align 8, !dbg !7145, !alias.scope !7137, !noalias !7138, !noundef !12
  %base.i.i34.i = load i32, ptr %_51.i27.i, align 4, !dbg !7146, !alias.scope !7137, !noalias !7156, !noundef !12
  %_67.i.i35.i = load i32, ptr %_45.i23.i, align 4, !alias.scope !7137, !noalias !7138
  %_75.i.i36.i = load i32, ptr %_50.i26.i, align 4, !alias.scope !7137, !noalias !7138
  %threshold.i30.i.i = load float, ptr %_10.i17.i, align 4, !alias.scope !7137, !noalias !7138
  %252 = getelementptr inbounds nuw i8, ptr %self, i64 808
  %ratio.i31.i.i = load float, ptr %252, align 4, !alias.scope !7137, !noalias !7138
  %253 = getelementptr inbounds nuw i8, ptr %self, i64 824
  %range.i32.i.i = load float, ptr %253, align 4, !alias.scope !7137, !noalias !7138
  %254 = getelementptr inbounds nuw i8, ptr %self, i64 840
  %hysteresis.i33.i.i = load float, ptr %254, align 4, !alias.scope !7137, !noalias !7138
  %255 = getelementptr inbounds nuw i8, ptr %self, i64 760
  %_37.i36.i.i = load float, ptr %255, align 4, !alias.scope !7137, !noalias !7138
  %_3.i188.i.i = fcmp ule float %_37.i36.i.i, 0.000000e+00
  %256 = getelementptr inbounds nuw i8, ptr %self, i64 764
  %_41.i40.i.i = load float, ptr %256, align 4, !alias.scope !7137, !noalias !7138
  %_3.i186.i.i = fcmp ule float %_41.i40.i.i, 0.000000e+00
  %257 = getelementptr inbounds nuw i8, ptr %self, i64 856
  %_0.i252.i.i = fsub float %threshold.i30.i.i, %hysteresis.i33.i.i
  %258 = getelementptr inbounds nuw i8, ptr %self, i64 860
  %259 = getelementptr inbounds nuw i8, ptr %self, i64 752
  %_71.i73564565.i.i = load float, ptr %259, align 4, !alias.scope !7137, !noalias !7138
  %_0.i250.i.i = fadd float %ratio.i31.i.i, -1.000000e+00
  %260 = fneg float %range.i32.i.i
  %261 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %_87.i88567.i.i = load i32, ptr %247, align 4, !alias.scope !7137, !noalias !7138
  %262 = getelementptr inbounds nuw i8, ptr %self, i64 748
  %_88.i89568.i.i = load i32, ptr %262, align 4, !alias.scope !7137, !noalias !7138
  %263 = getelementptr inbounds nuw i8, ptr %self, i64 756
  %_98.i102.i.i = load float, ptr %263, align 4, !alias.scope !7137, !noalias !7138
  %_3.i176.i.i = fcmp ule float %_98.i102.i.i, 0.000000e+00
  %threshold.i.i.i = load float, ptr %data.i.i.i18.i, align 4, !alias.scope !7137, !noalias !7138
  %264 = getelementptr inbounds nuw i8, ptr %self, i64 884
  %ratio.i.i.i = load float, ptr %264, align 4, !alias.scope !7137, !noalias !7138
  %265 = getelementptr inbounds nuw i8, ptr %self, i64 900
  %range.i.i.i = load float, ptr %265, align 4, !alias.scope !7137, !noalias !7138
  %266 = getelementptr inbounds nuw i8, ptr %self, i64 916
  %hysteresis.i.i.i = load float, ptr %266, align 4, !alias.scope !7137, !noalias !7138
  %267 = getelementptr inbounds nuw i8, ptr %self, i64 784
  %_37.i.i37.i = load float, ptr %267, align 4, !alias.scope !7137, !noalias !7138
  %_3.i202.i.i = fcmp ule float %_37.i.i37.i, 0.000000e+00
  %268 = getelementptr inbounds nuw i8, ptr %self, i64 788
  %_41.i.i38.i = load float, ptr %268, align 4, !alias.scope !7137, !noalias !7138
  %_3.i200.i.i = fcmp ule float %_41.i.i38.i, 0.000000e+00
  %269 = getelementptr inbounds nuw i8, ptr %self, i64 932
  %_0.i257.i.i = fsub float %threshold.i.i.i, %hysteresis.i.i.i
  %270 = getelementptr inbounds nuw i8, ptr %self, i64 936
  %271 = getelementptr inbounds nuw i8, ptr %self, i64 776
  %_71.i581582.i.i = load float, ptr %271, align 4, !alias.scope !7137, !noalias !7138
  %_0.i255.i.i = fadd float %ratio.i.i.i, -1.000000e+00
  %272 = fneg float %range.i.i.i
  %273 = getelementptr inbounds nuw i8, ptr %self, i64 940
  %_87.i21584.i.i = load i32, ptr %_32.i20.i, align 4, !alias.scope !7137, !noalias !7138
  %274 = getelementptr inbounds nuw i8, ptr %self, i64 772
  %_88.i22585.i.i = load i32, ptr %274, align 4, !alias.scope !7137, !noalias !7138
  %275 = getelementptr inbounds nuw i8, ptr %self, i64 780
  %_98.i.i39.i = load float, ptr %275, align 4, !alias.scope !7137, !noalias !7138
  %_3.i190.i.i = fcmp ule float %_98.i.i39.i, 0.000000e+00
  %.promoted973.i.i = load float, ptr %258, align 4, !alias.scope !7137, !noalias !7138
  %.promoted975.i.i = load float, ptr %261, align 4, !alias.scope !7137, !noalias !7138
  %.promoted977.i.i = load float, ptr %270, align 4, !alias.scope !7137, !noalias !7138
  %.promoted979.i.i = load float, ptr %273, align 4, !alias.scope !7137, !noalias !7138
  %injected.cond.not.i.i = icmp samesign ugt i64 %_14.1, %_13.1
  br i1 %injected.cond.not.i.i, label %bb42.i.i.i, label %bb40.i.lr.ph.split.us.i.i

bb40.i.lr.ph.split.us.i.i:                        ; preds = %bb27.i
  %injected.cond1064.not.i.i = icmp ugt i64 %_58.1.i22.i, %_60.1.i25.i
  br i1 %injected.cond1064.not.i.i, label %bb42.i.us.i118.i, label %bb42.i.us.us.us.i46.i

bb42.i.us.us.us.i46.i:                            ; preds = %bb40.i.lr.ph.split.us.i.i, %bb28.i.us.us.lr.ph.us.us.us.i84.i
  %_86.i19980.us.us.us.i.i = phi float [ %_0.i290.us.us.us.i107.i, %bb28.i.us.us.lr.ph.us.us.us.i84.i ], [ %.promoted979.i.i, %bb40.i.lr.ph.split.us.i.i ]
  %_67.i11978.us.us.us.i.i = phi float [ %_0.i367.us.us.us.i103.i, %bb28.i.us.us.lr.ph.us.us.us.i84.i ], [ %.promoted977.i.i, %bb40.i.lr.ph.split.us.i.i ]
  %_86.i86976.us.us.us.i.i = phi float [ %_0.i286.us.us.us.i92.i, %bb28.i.us.us.lr.ph.us.us.us.i84.i ], [ %.promoted975.i.i, %bb40.i.lr.ph.split.us.i.i ]
  %_67.i67974.us.us.us.i.i = phi float [ %_0.i315.us.us.us.i.i, %bb28.i.us.us.lr.ph.us.us.us.i84.i ], [ %.promoted973.i.i, %bb40.i.lr.ph.split.us.i.i ]
  %iter.sroa.0.0.i938.us.us.us.i.i = phi i64 [ %276, %bb28.i.us.us.lr.ph.us.us.us.i84.i ], [ 0, %bb40.i.lr.ph.split.us.i.i ]
  %276 = add nuw nsw i64 %iter.sroa.0.0.i938.us.us.us.i.i, 1, !dbg !7159
  %_27.i.us.us.us.i41.i = trunc i64 %iter.sroa.0.0.i938.us.us.us.i.i to i32, !dbg !7173
  %now.i.us.us.us.i42.i = add i32 %base.i.i34.i, %_27.i.us.us.us.i41.i, !dbg !7176
  %_30.i.us.us.us.i43.i = and i32 %now.i.us.us.us.i42.i, %_52.i28.i, !dbg !7179
  %_29.i.us.us.us.i44.i = zext i32 %_30.i.us.us.us.i43.i to i64, !dbg !7181
  %_123.i.us.us.us.i47.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %iter.sroa.0.0.i938.us.us.us.i.i, !dbg !7182
  %_124.not.not.i.us.us.us.i48.i = icmp ugt i64 %_58.1.i22.i, %_29.i.us.us.us.i44.i, !dbg !7191
  br i1 %_124.not.not.i.us.us.us.i48.i, label %bb45.i.us.us.us.i50.i, label %bb46.i.i49.i, !dbg !7191, !prof !2704

bb45.i.us.us.us.i50.i:                            ; preds = %bb42.i.us.us.us.i46.i
  %_0.i273.us.us.us.i51.i = load float, ptr %_123.i.us.us.us.i47.i, align 4, !dbg !7196, !alias.scope !7198, !noalias !7201, !noundef !12
  %_133.i.us.us.us.i52.i = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_29.i.us.us.us.i44.i, !dbg !7202
  store float %_0.i273.us.us.us.i51.i, ptr %_133.i.us.us.us.i52.i, align 4, !dbg !7206, !alias.scope !7208, !noalias !7211
  %_141.i.us.us.us.i53.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %iter.sroa.0.0.i938.us.us.us.i.i, !dbg !7212
  %_0.i271.us.us.us.i54.i = load float, ptr %_141.i.us.us.us.i53.i, align 4, !dbg !7219, !alias.scope !7221, !noalias !7224, !noundef !12
  %_149.i.us.us.us.i55.i = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_29.i.us.us.us.i44.i, !dbg !7225
  store float %_0.i271.us.us.us.i54.i, ptr %_149.i.us.us.us.i55.i, align 4, !dbg !7232, !alias.scope !7234, !noalias !7211
  %_52.i.us.us.us.i56.i = sub i32 %now.i.us.us.us.i42.i, %_53.i29.i, !dbg !7237
  %_51.i.us.us.us.i57.i = and i32 %_52.i.us.us.us.i56.i, %_52.i28.i, !dbg !7240
  %_50.i.us.us.us.i58.i = zext i32 %_51.i.us.us.us.i57.i to i64, !dbg !7241
  %_182.not.not.i.us.us.us.i59.i = icmp ugt i64 %_58.1.i22.i, %_50.i.us.us.us.i58.i, !dbg !7242
  br i1 %_182.not.not.i.us.us.us.i59.i, label %bb60.i.us.us.us.i61.i, label %bb61.i.i60.i, !dbg !7242, !prof !2704

bb60.i.us.us.us.i61.i:                            ; preds = %bb45.i.us.us.us.i50.i
  %_189.i.us.us.us.i62.i = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_50.i.us.us.us.i58.i, !dbg !7247
  %_0.i269.us.us.us.i63.i = load float, ptr %_189.i.us.us.us.i62.i, align 4, !dbg !7251, !alias.scope !7253, !noalias !7211, !noundef !12
  %_195.i.us.us.us.i64.i = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_50.i.us.us.us.i58.i, !dbg !7256
  %_0.i267.us.us.us.i65.i = load float, ptr %_195.i.us.us.us.i64.i, align 4, !dbg !7264, !alias.scope !7266, !noalias !7211, !noundef !12
  %_66.i.us.us.us.i66.i = sub i32 %now.i.us.us.us.i42.i, %_67.i.i35.i
  %_65.i.us.us.us.i67.i = and i32 %_66.i.us.us.us.i66.i, %_52.i28.i
  %_64.i.us.us.us.i68.i = zext i32 %_65.i.us.us.us.i67.i to i64
  %_74.i.us.us.us.i69.i = sub i32 %now.i.us.us.us.i42.i, %_75.i.i36.i
  %_73.i.us.us.us.i70.i = and i32 %_74.i.us.us.us.i69.i, %_52.i28.i
  %_72.i.us.us.us.i71.i = zext i32 %_73.i.us.us.us.i70.i to i64
  %_80.i.us.us.us.i72.i = icmp ugt i64 %_58.1.i22.i, %_64.i.us.us.us.i68.i
  %277 = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_64.i.us.us.us.i68.i
  %278 = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_72.i.us.us.us.i71.i
  %_88.i.us.us.us.i74.i = icmp ugt i64 %_58.1.i22.i, %_72.i.us.us.us.i71.i
  %279 = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_72.i.us.us.us.i71.i
  br i1 %_80.i.us.us.us.i72.i, label %bb63.i.split.us.us.us.us.i76.i, label %bb63.i.split.i75.i

bb63.i.split.us.us.us.us.i76.i:                   ; preds = %bb60.i.us.us.us.i61.i
  %_86.i.us.us.us.i73.i = icmp ugt i64 %_60.1.i25.i, %_72.i.us.us.us.i71.i
  %280 = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_64.i.us.us.us.i68.i
  %_82.i.us.us.us.us.i80.i = load float, ptr %280, align 4, !noalias !7211, !noundef !12
  br i1 %_86.i.us.us.us.i73.i, label %bb24.i.us.lr.ph.split.us.us.us.us.i82.i, label %bb24.i.us.lr.ph.split.i81.i

bb24.i.us.lr.ph.split.us.us.us.us.i82.i:          ; preds = %bb63.i.split.us.us.us.us.i76.i
  br i1 %_88.i.us.us.us.i74.i, label %bb28.i.us.us.lr.ph.us.us.us.i84.i, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i83.i, !dbg !7269

bb28.i.us.us.lr.ph.us.us.us.i84.i:                ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.i82.i
  %_87.i.us.us.us.us.us.i85.i = load float, ptr %279, align 4, !noalias !7211, !noundef !12
  %_85.i.us.us.le807.us.us.us.i.i = load float, ptr %278, align 4, !noalias !7211, !noundef !12
  %_78.i.us.le.us.us.us.i86.i = load float, ptr %277, align 4, !noalias !7211, !noundef !12
  %281 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.us.us.i86.i), !dbg !7277
  %282 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.us.us.i80.i), !dbg !7280
  %_3.i.i460.us.us.us.i.i = fcmp ule float %281, %282, !dbg !7282
  %_6.i.i462.us.us.us.i.i = bitcast float %281 to i32, !dbg !7285
  %_8.i.i464.us.us.us.i.i = bitcast float %282 to i32, !dbg !7288
  %_4.i.i467.us.us.us.i.i = select i1 %_3.i.i460.us.us.us.i.i, i32 %_8.i.i464.us.us.us.i.i, i32 %_6.i.i462.us.us.us.i.i, !dbg !7290
  %_4.i340.us.us.us.i.i = select i1 %_3.i188.i.i, i32 %_6.i.i462.us.us.us.i.i, i32 %_4.i.i467.us.us.us.i.i, !dbg !7291
  %_0.i237.us.us.us.i.i = fmul float %281, 5.000000e-01, !dbg !7293
  %_0.i236.us.us.us.i.i = fmul float %282, 5.000000e-01, !dbg !7295
  %_0.i216.us.us.us.i.i = fadd float %_0.i236.us.us.us.i.i, %_0.i237.us.us.us.i.i, !dbg !7297
  %_6.i328.us.us.us.i.i = bitcast float %_0.i216.us.us.us.i.i to i32, !dbg !7299
  %_4.i333.us.us.us.i.i = select i1 %_3.i186.i.i, i32 %_4.i340.us.us.us.i.i, i32 %_6.i328.us.us.us.i.i, !dbg !7302
  %_0.i334.us.us.us.i.i = bitcast i32 %_4.i333.us.us.us.i.i to float, !dbg !7303
  %_3.i.i452.us.us.us.i.i = fcmp ule float %_0.i334.us.us.us.i.i, 0x3E45798EE0000000, !dbg !7305
  %_4.i.i458.us.us.us.i.i = select i1 %_3.i.i452.us.us.us.i.i, i32 841731191, i32 %_4.i333.us.us.us.i.i, !dbg !7308
  %_0.i.i459.us.us.us.i.i = bitcast i32 %_4.i.i458.us.us.us.i.i to float, !dbg !7310
  %_3.i.i412.us.us.us.i.i = fcmp ule float %_0.i.i459.us.us.us.i.i, 0x3810000000000000, !dbg !7312
  %_4.i.i418.us.us.us.i.i = select i1 %_3.i.i412.us.us.us.i.i, i32 8388608, i32 %_4.i.i458.us.us.us.i.i, !dbg !7317
  %_5.i278.us.us.us.i.i = and i32 %_4.i.i418.us.us.us.i.i, 8388607, !dbg !7319
  %_4.i279.us.us.us.i.i = or disjoint i32 %_5.i278.us.us.us.i.i, 1065353216, !dbg !7319
  %significand.i280.us.us.us.i.i = bitcast i32 %_4.i279.us.us.us.i.i to float, !dbg !7321
  %_0.i245.us.us.us.i87.i = fadd float %significand.i280.us.us.us.i.i, -1.000000e+00, !dbg !7323
  %_0.i225.us.us.us.i.i = fmul float %_0.i245.us.us.us.i87.i, 0x3F9B17A960000000, !dbg !7325
  %283 = fsub float 0x3FBF9A8440000000, %_0.i225.us.us.us.i.i, !dbg !7327
  %_0.i225.us.us.us.1.i.i = fmul float %_0.i245.us.us.us.i87.i, %283, !dbg !7325
  %_0.i211.us.us.us.1.i.i = fadd float %_0.i225.us.us.us.1.i.i, 0xBFD1E3F400000000, !dbg !7327
  %_0.i225.us.us.us.2.i.i = fmul float %_0.i245.us.us.us.i87.i, %_0.i211.us.us.us.1.i.i, !dbg !7325
  %_0.i211.us.us.us.2.i.i = fadd float %_0.i225.us.us.us.2.i.i, 0x3FDD544F20000000, !dbg !7327
  %_0.i225.us.us.us.3.i.i = fmul float %_0.i245.us.us.us.i87.i, %_0.i211.us.us.us.2.i.i, !dbg !7325
  %_0.i211.us.us.us.3.i.i = fadd float %_0.i225.us.us.us.3.i.i, 0xBFE6FC2A60000000, !dbg !7327
  %_0.i225.us.us.us.4.i.i = fmul float %_0.i245.us.us.us.i87.i, %_0.i211.us.us.us.3.i.i, !dbg !7325
  %_0.i211.us.us.us.4.i.i = fadd float %_0.i225.us.us.us.4.i.i, 0x3FF714B2A0000000, !dbg !7327
  %_9.i281.us.us.us.i.i = lshr i32 %_4.i.i418.us.us.us.i.i, 23, !dbg !7329
  %_8.i282.us.us.us.i.i = or disjoint i32 %_9.i281.us.us.us.i.i, 1258291200, !dbg !7329
  %_7.i283.us.us.us.i.i = bitcast i32 %_8.i282.us.us.us.i.i to float, !dbg !7330
  %exponent.i284.us.us.us.i.i = fadd float %_7.i283.us.us.us.i.i, 0xC160000FE0000000, !dbg !7332
  %_0.i224.us.us.us.i.i = fmul float %_0.i245.us.us.us.i87.i, %_0.i211.us.us.us.4.i.i, !dbg !7333
  %_0.i210.us.us.us.i.i = fadd float %exponent.i284.us.us.us.i.i, %_0.i224.us.us.us.i.i, !dbg !7335
  %_0.i235.us.us.us.i.i = fmul float %_0.i210.us.us.us.i.i, 0x4018151820000000, !dbg !7337
  %_3.i.i527.us.us.us.inv.i.i = fcmp olt float %_0.i235.us.us.us.i.i, 2.400000e+01, !dbg !7339
  %_0.i.i534.us.us.us.i.i = select i1 %_3.i.i527.us.us.us.inv.i.i, float %_0.i235.us.us.us.i.i, float 2.400000e+01, !dbg !7339
  %_3.i.i444.us.us.us.inv.i.i = fcmp ogt float %_0.i.i534.us.us.us.i.i, -1.600000e+02, !dbg !7342
  %_0.i.i451.us.us.us.i.i = select i1 %_3.i.i444.us.us.us.inv.i.i, float %_0.i.i534.us.us.us.i.i, float -1.600000e+02, !dbg !7342
  %_55.i57.us.us.us.i.i = load float, ptr %257, align 4, !dbg !7345, !alias.scope !7346, !noalias !7349, !noundef !12
  %_3.i184.us.us.us.i.i = fcmp ule float %_55.i57.us.us.us.i.i, 0.000000e+00, !dbg !7351
  %_3.i170.us.us.us.i.i = fcmp oge float %_0.i.i451.us.us.us.i.i, %threshold.i30.i.i, !dbg !7353
  %_3.i168.us.us.us.i.i = fcmp oge float %_0.i.i451.us.us.us.i.i, %_0.i252.i.i, !dbg !7355
  %..i169.us.us.us.i.i = sext i1 %_3.i168.us.us.us.i.i to i32, !dbg !7357
  %_0.i402.us.us.us.i.i = sext i1 %_3.i170.us.us.us.i.i to i32, !dbg !7359
  %_0.i396.us.us.us.i.i = select i1 %_3.i184.us.us.us.i.i, i32 %_0.i402.us.us.us.i.i, i32 %..i169.us.us.us.i.i, !dbg !7359
  %_0.i408.us.us.us.i.i = xor i32 %..i169.us.us.us.i.i, -1, !dbg !7361
  %_3.i182.us.us.us.i.i = fcmp ogt float %_67.i67974.us.us.us.i.i, 0.000000e+00, !dbg !7363
  %_0.i401.us.us.us.i88.i = select i1 %_3.i182.us.us.us.i.i, i32 %_0.i408.us.us.us.i.i, i32 0, !dbg !7365
  %_0.i400.us.us.us.i.i = select i1 %_3.i184.us.us.us.i.i, i32 0, i32 %_0.i401.us.us.us.i88.i, !dbg !7367
  %_0.i395.us.us.us.i.i = or i32 %_0.i400.us.us.us.i.i, %_0.i396.us.us.us.i.i, !dbg !7369
  %_5.i323.us.us.us.i.i = and i32 %_0.i395.us.us.us.i.i, 1065353216, !dbg !7371
  %_0.i327.us.us.us.i.i = bitcast i32 %_5.i323.us.us.us.i.i to float, !dbg !7373
  %_0.i251.us.us.us.i89.i = fadd float %_67.i67974.us.us.us.i.i, -1.000000e+00, !dbg !7375
  %284 = trunc nsw i32 %_0.i400.us.us.us.i.i to i1, !dbg !7377
  %_4.i321.v.us.us.us.i.i = select i1 %284, float %_0.i251.us.us.us.i89.i, float %_67.i67974.us.us.us.i.i, !dbg !7377
  %285 = trunc nsw i32 %_0.i396.us.us.us.i.i to i1, !dbg !7379
  %_0.i315.us.us.us.i.i = select i1 %285, float %_71.i73564565.i.i, float %_4.i321.v.us.us.us.i.i, !dbg !7379
  store float %_0.i315.us.us.us.i.i, ptr %258, align 4, !dbg !7381, !alias.scope !7346, !noalias !7349
  store i32 %_5.i323.us.us.us.i.i, ptr %257, align 4, !dbg !7382, !alias.scope !7346, !noalias !7349
  %_0.i249.us.us.us.i90.i = fsub float %_0.i.i451.us.us.us.i.i, %threshold.i30.i.i, !dbg !7383
  %_0.i234.us.us.us.i.i = fmul float %_0.i250.i.i, %_0.i249.us.us.us.i90.i, !dbg !7385
  %_3.i.i436.inv.us.us.us.i.i = fcmp ogt float %_0.i234.us.us.us.i.i, %260, !dbg !7387
  %_4.i.i442.v.us.us.us.i.i = select i1 %_3.i.i436.inv.us.us.us.i.i, float %_0.i234.us.us.us.i.i, float %260, !dbg !7387
  %_3.i.i519.us.us.us.i.i = fcmp olt float %_4.i.i442.v.us.us.us.i.i, 0.000000e+00, !dbg !7390
  %286 = fcmp ule float %_0.i327.us.us.us.i.i, 0.000000e+00, !dbg !7393
  %287 = select i1 %286, i1 %_3.i.i519.us.us.us.i.i, i1 false, !dbg !7395
  %_0.i308.us.us.us.i.i = select i1 %287, float %_4.i.i442.v.us.us.us.i.i, float 0.000000e+00, !dbg !7395
  %_3.i178.us.us.us.i.i = fcmp ule float %_0.i308.us.us.us.i.i, %_86.i86976.us.us.us.i.i, !dbg !7396
  %_4.i301.us.us.us.i.i = select i1 %_3.i178.us.us.us.i.i, i32 %_88.i89568.i.i, i32 %_87.i88567.i.i, !dbg !7398
  %_0.i302.us.us.us.i.i = bitcast i32 %_4.i301.us.us.us.i.i to float, !dbg !7400
  %_0.i248.us.us.us.i91.i = fsub float %_0.i308.us.us.us.i.i, %_86.i86976.us.us.us.i.i, !dbg !7402
  %_4.i218.us.us.us.i.i = fmul float %_0.i248.us.us.us.i91.i, %_0.i302.us.us.us.i.i, !dbg !7404
  %_0.i219.us.us.us.i.i = fadd float %_86.i86976.us.us.us.i.i, %_4.i218.us.us.us.i.i, !dbg !7404
  %288 = tail call noundef float @llvm.fabs.f32(float %_0.i219.us.us.us.i.i), !dbg !7406
  %289 = fcmp uge float %288, 0x3BC79CA100000000, !dbg !7409
  %_0.i286.us.us.us.i92.i = select i1 %289, float %_0.i219.us.us.us.i.i, float 0.000000e+00, !dbg !7411
  store float %_0.i286.us.us.us.i92.i, ptr %261, align 4, !dbg !7412, !alias.scope !7346, !noalias !7349
  %_0.i233.us.us.us.i.i = fmul float %_0.i286.us.us.us.i92.i, 0x3FC542A5A0000000, !dbg !7413
  %_3.i.i428.us.us.us.inv.i.i = fcmp ogt float %_0.i233.us.us.us.i.i, -1.260000e+02, !dbg !7416
  %_0.i.i435.us.us.us.i.i = select i1 %_3.i.i428.us.us.us.inv.i.i, float %_0.i233.us.us.us.i.i, float -1.260000e+02, !dbg !7416
  %_3.i.i511.us.us.us.inv.i.i = fcmp olt float %_0.i.i435.us.us.us.i.i, 1.270000e+02, !dbg !7420
  %_0.i.i518.us.us.us.i.i = select i1 %_3.i.i511.us.us.us.inv.i.i, float %_0.i.i435.us.us.us.i.i, float 1.270000e+02, !dbg !7420
  %290 = tail call noundef float @llvm.floor.f32(float %_0.i.i518.us.us.us.i.i), !dbg !7423
  %_0.i247.us.us.us.i93.i = fsub float %_0.i.i518.us.us.us.i.i, %290, !dbg !7427
  %291 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le807.us.us.us.i.i), !dbg !7429
  %292 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.us.us.i85.i), !dbg !7433
  %_3.i.i494.us.us.us.i.i = fcmp ule float %291, %292, !dbg !7435
  %_6.i.i496.us.us.us.i.i = bitcast float %291 to i32, !dbg !7438
  %_8.i.i498.us.us.us.i.i = bitcast float %292 to i32, !dbg !7441
  %_4.i.i501.us.us.us.i.i = select i1 %_3.i.i494.us.us.us.i.i, i32 %_8.i.i498.us.us.us.i.i, i32 %_6.i.i496.us.us.us.i.i, !dbg !7443
  %_4.i392.us.us.us.i.i = select i1 %_3.i202.i.i, i32 %_6.i.i496.us.us.us.i.i, i32 %_4.i.i501.us.us.us.i.i, !dbg !7444
  %_0.i243.us.us.us.i.i = fmul float %291, 5.000000e-01, !dbg !7446
  %_0.i242.us.us.us.i.i = fmul float %292, 5.000000e-01, !dbg !7448
  %_0.i217.us.us.us.i.i = fadd float %_0.i242.us.us.us.i.i, %_0.i243.us.us.us.i.i, !dbg !7450
  %_6.i380.us.us.us.i.i = bitcast float %_0.i217.us.us.us.i.i to i32, !dbg !7452
  %_4.i385.us.us.us.i.i = select i1 %_3.i200.i.i, i32 %_4.i392.us.us.us.i.i, i32 %_6.i380.us.us.us.i.i, !dbg !7455
  %_0.i386.us.us.us.i.i = bitcast i32 %_4.i385.us.us.us.i.i to float, !dbg !7456
  %_3.i.i486.us.us.us.i.i = fcmp ule float %_0.i386.us.us.us.i.i, 0x3E45798EE0000000, !dbg !7458
  %_4.i.i492.us.us.us.i.i = select i1 %_3.i.i486.us.us.us.i.i, i32 841731191, i32 %_4.i385.us.us.us.i.i, !dbg !7461
  %_0.i.i493.us.us.us.i.i = bitcast i32 %_4.i.i492.us.us.us.i.i to float, !dbg !7463
  %_3.i.i.us.us.us.i94.i = fcmp ule float %_0.i.i493.us.us.us.i.i, 0x3810000000000000, !dbg !7465
  %_4.i.i.us.us.us.i95.i = select i1 %_3.i.i.us.us.us.i94.i, i32 8388608, i32 %_4.i.i492.us.us.us.i.i, !dbg !7470
  %_5.i274.us.us.us.i.i = and i32 %_4.i.i.us.us.us.i95.i, 8388607, !dbg !7472
  %_4.i275.us.us.us.i.i = or disjoint i32 %_5.i274.us.us.us.i.i, 1065353216, !dbg !7472
  %significand.i.us.us.us.i96.i = bitcast i32 %_4.i275.us.us.us.i.i to float, !dbg !7474
  %_0.i244.us.us.us.i97.i = fadd float %significand.i.us.us.us.i96.i, -1.000000e+00, !dbg !7476
  %_0.i223.us.us.us.i.i = fmul float %_0.i244.us.us.us.i97.i, 0x3F9B17A960000000, !dbg !7478
  %293 = fsub float 0x3FBF9A8440000000, %_0.i223.us.us.us.i.i, !dbg !7480
  %_0.i223.us.us.us.1.i.i = fmul float %_0.i244.us.us.us.i97.i, %293, !dbg !7478
  %_0.i209.us.us.us.1.i.i = fadd float %_0.i223.us.us.us.1.i.i, 0xBFD1E3F400000000, !dbg !7480
  %_0.i223.us.us.us.2.i.i = fmul float %_0.i244.us.us.us.i97.i, %_0.i209.us.us.us.1.i.i, !dbg !7478
  %_0.i209.us.us.us.2.i.i = fadd float %_0.i223.us.us.us.2.i.i, 0x3FDD544F20000000, !dbg !7480
  %_0.i223.us.us.us.3.i.i = fmul float %_0.i244.us.us.us.i97.i, %_0.i209.us.us.us.2.i.i, !dbg !7478
  %_0.i209.us.us.us.3.i.i = fadd float %_0.i223.us.us.us.3.i.i, 0xBFE6FC2A60000000, !dbg !7480
  %_0.i223.us.us.us.4.i.i = fmul float %_0.i244.us.us.us.i97.i, %_0.i209.us.us.us.3.i.i, !dbg !7478
  %_0.i209.us.us.us.4.i.i = fadd float %_0.i223.us.us.us.4.i.i, 0x3FF714B2A0000000, !dbg !7480
  %_9.i.us.us.us.i98.i = lshr i32 %_4.i.i.us.us.us.i95.i, 23, !dbg !7482
  %_8.i276.us.us.us.i.i = or disjoint i32 %_9.i.us.us.us.i98.i, 1258291200, !dbg !7482
  %_7.i.us.us.us.i99.i = bitcast i32 %_8.i276.us.us.us.i.i to float, !dbg !7483
  %exponent.i.us.us.us.i100.i = fadd float %_7.i.us.us.us.i99.i, 0xC160000FE0000000, !dbg !7485
  %_0.i222.us.us.us.i.i = fmul float %_0.i244.us.us.us.i97.i, %_0.i209.us.us.us.4.i.i, !dbg !7486
  %_0.i208.us.us.us.i.i = fadd float %exponent.i.us.us.us.i100.i, %_0.i222.us.us.us.i.i, !dbg !7488
  %_0.i241.us.us.us.i.i = fmul float %_0.i208.us.us.us.i.i, 0x4018151820000000, !dbg !7490
  %_3.i.i543.us.us.us.inv.i.i = fcmp olt float %_0.i241.us.us.us.i.i, 2.400000e+01, !dbg !7492
  %_0.i.i550.us.us.us.i.i = select i1 %_3.i.i543.us.us.us.inv.i.i, float %_0.i241.us.us.us.i.i, float 2.400000e+01, !dbg !7492
  %_3.i.i478.us.us.us.inv.i.i = fcmp ogt float %_0.i.i550.us.us.us.i.i, -1.600000e+02, !dbg !7495
  %_0.i.i485.us.us.us.i.i = select i1 %_3.i.i478.us.us.us.inv.i.i, float %_0.i.i550.us.us.us.i.i, float -1.600000e+02, !dbg !7495
  %_55.i9.us.us.us.i.i = load float, ptr %269, align 4, !dbg !7498, !alias.scope !7499, !noalias !7502, !noundef !12
  %_3.i198.us.us.us.i101.i = fcmp ule float %_55.i9.us.us.us.i.i, 0.000000e+00, !dbg !7504
  %_3.i174.us.us.us.i.i = fcmp oge float %_0.i.i485.us.us.us.i.i, %threshold.i.i.i, !dbg !7506
  %_3.i172.us.us.us.i.i = fcmp oge float %_0.i.i485.us.us.us.i.i, %_0.i257.i.i, !dbg !7508
  %..i173.us.us.us.i.i = sext i1 %_3.i172.us.us.us.i.i to i32, !dbg !7510
  %_0.i406.us.us.us.i.i = sext i1 %_3.i174.us.us.us.i.i to i32, !dbg !7512
  %_0.i399.us.us.us.i.i = select i1 %_3.i198.us.us.us.i101.i, i32 %_0.i406.us.us.us.i.i, i32 %..i173.us.us.us.i.i, !dbg !7512
  %_0.i410.us.us.us.i.i = xor i32 %..i173.us.us.us.i.i, -1, !dbg !7514
  %_3.i196.us.us.us.i102.i = fcmp ogt float %_67.i11978.us.us.us.i.i, 0.000000e+00, !dbg !7516
  %_0.i405.us.us.us.i.i = select i1 %_3.i196.us.us.us.i102.i, i32 %_0.i410.us.us.us.i.i, i32 0, !dbg !7518
  %_0.i404.us.us.us.i.i = select i1 %_3.i198.us.us.us.i101.i, i32 0, i32 %_0.i405.us.us.us.i.i, !dbg !7520
  %_0.i398.us.us.us.i.i = or i32 %_0.i404.us.us.us.i.i, %_0.i399.us.us.us.i.i, !dbg !7522
  %_5.i375.us.us.us.i.i = and i32 %_0.i398.us.us.us.i.i, 1065353216, !dbg !7524
  %_0.i379.us.us.us.i.i = bitcast i32 %_5.i375.us.us.us.i.i to float, !dbg !7526
  %_0.i256.us.us.us.i.i = fadd float %_67.i11978.us.us.us.i.i, -1.000000e+00, !dbg !7528
  %294 = trunc nsw i32 %_0.i404.us.us.us.i.i to i1, !dbg !7530
  %_4.i373.v.us.us.us.i.i = select i1 %294, float %_0.i256.us.us.us.i.i, float %_67.i11978.us.us.us.i.i, !dbg !7530
  %295 = trunc nsw i32 %_0.i399.us.us.us.i.i to i1, !dbg !7532
  %_0.i367.us.us.us.i103.i = select i1 %295, float %_71.i581582.i.i, float %_4.i373.v.us.us.us.i.i, !dbg !7532
  store float %_0.i367.us.us.us.i103.i, ptr %270, align 4, !dbg !7534, !alias.scope !7499, !noalias !7502
  store i32 %_5.i375.us.us.us.i.i, ptr %269, align 4, !dbg !7535, !alias.scope !7499, !noalias !7502
  %_0.i254.us.us.us.i104.i = fsub float %_0.i.i485.us.us.us.i.i, %threshold.i.i.i, !dbg !7536
  %_0.i240.us.us.us.i.i = fmul float %_0.i255.i.i, %_0.i254.us.us.us.i104.i, !dbg !7538
  %_3.i.i469.inv.us.us.us.i.i = fcmp ogt float %_0.i240.us.us.us.i.i, %272, !dbg !7540
  %_4.i.i476.v.us.us.us.i.i = select i1 %_3.i.i469.inv.us.us.us.i.i, float %_0.i240.us.us.us.i.i, float %272, !dbg !7540
  %_3.i.i535.us.us.us.i.i = fcmp olt float %_4.i.i476.v.us.us.us.i.i, 0.000000e+00, !dbg !7543
  %296 = fcmp ule float %_0.i379.us.us.us.i.i, 0.000000e+00, !dbg !7546
  %297 = select i1 %296, i1 %_3.i.i535.us.us.us.i.i, i1 false, !dbg !7548
  %_0.i360.us.us.us.i.i = select i1 %297, float %_4.i.i476.v.us.us.us.i.i, float 0.000000e+00, !dbg !7548
  %_3.i192.us.us.us.i.i = fcmp ule float %_0.i360.us.us.us.i.i, %_86.i19980.us.us.us.i.i, !dbg !7549
  %_4.i354.us.us.us.i.i = select i1 %_3.i192.us.us.us.i.i, i32 %_88.i22585.i.i, i32 %_87.i21584.i.i, !dbg !7551
  %_0.i.us.us.us.i105.i = bitcast i32 %_4.i354.us.us.us.i.i to float, !dbg !7553
  %_0.i253.us.us.us.i106.i = fsub float %_0.i360.us.us.us.i.i, %_86.i19980.us.us.us.i.i, !dbg !7555
  %_4.i220.us.us.us.i.i = fmul float %_0.i253.us.us.us.i106.i, %_0.i.us.us.us.i105.i, !dbg !7557
  %_0.i221.us.us.us.i.i = fadd float %_86.i19980.us.us.us.i.i, %_4.i220.us.us.us.i.i, !dbg !7557
  %298 = tail call noundef float @llvm.fabs.f32(float %_0.i221.us.us.us.i.i), !dbg !7559
  %299 = fcmp uge float %298, 0x3BC79CA100000000, !dbg !7562
  %_0.i290.us.us.us.i107.i = select i1 %299, float %_0.i221.us.us.us.i.i, float 0.000000e+00, !dbg !7564
  store float %_0.i290.us.us.us.i107.i, ptr %273, align 4, !dbg !7565, !alias.scope !7499, !noalias !7502
  %_0.i239.us.us.us.i.i = fmul float %_0.i290.us.us.us.i107.i, 0x3FC542A5A0000000, !dbg !7566
  %_3.i.i420.us.us.us.inv.i.i = fcmp ogt float %_0.i239.us.us.us.i.i, -1.260000e+02, !dbg !7569
  %_0.i.i427.us.us.us.i.i = select i1 %_3.i.i420.us.us.us.inv.i.i, float %_0.i239.us.us.us.i.i, float -1.260000e+02, !dbg !7569
  %_3.i.i503.us.us.us.inv.i.i = fcmp olt float %_0.i.i427.us.us.us.i.i, 1.270000e+02, !dbg !7573
  %_0.i.i510.us.us.us.i.i = select i1 %_3.i.i503.us.us.us.inv.i.i, float %_0.i.i427.us.us.us.i.i, float 1.270000e+02, !dbg !7573
  %300 = tail call noundef float @llvm.floor.f32(float %_0.i.i510.us.us.us.i.i), !dbg !7576
  %_0.i246.us.us.us.i108.i = fsub float %_0.i.i510.us.us.us.i.i, %300, !dbg !7580
  %_0.i228.us.us.us.i.i = fmul float %_0.i246.us.us.us.i108.i, 0x3F5E974FA0000000, !dbg !7582
  %_0.i213.us.us.us.i.i = fadd float %_0.i228.us.us.us.i.i, 0x3F82778560000000, !dbg !7584
  %_0.i228.us.us.us.1.i.i = fmul float %_0.i246.us.us.us.i108.i, %_0.i213.us.us.us.i.i, !dbg !7582
  %_0.i213.us.us.us.1.i.i = fadd float %_0.i228.us.us.us.1.i.i, 0x3FAC91CE60000000, !dbg !7584
  %_0.i228.us.us.us.2.i.i = fmul float %_0.i246.us.us.us.i108.i, %_0.i213.us.us.us.1.i.i, !dbg !7582
  %_0.i213.us.us.us.2.i.i = fadd float %_0.i228.us.us.us.2.i.i, 0x3FCEBDB560000000, !dbg !7584
  %_0.i228.us.us.us.3.i.i = fmul float %_0.i246.us.us.us.i108.i, %_0.i213.us.us.us.2.i.i, !dbg !7582
  %_0.i213.us.us.us.3.i.i = fadd float %_0.i228.us.us.us.3.i.i, 0x3FE62E4BA0000000, !dbg !7584
  %_0.i231.us.us.us.i.i = fmul float %_0.i247.us.us.us.i93.i, 0x3F5E974FA0000000, !dbg !7586
  %_0.i215.us.us.us.i.i = fadd float %_0.i231.us.us.us.i.i, 0x3F82778560000000, !dbg !7588
  %_0.i231.us.us.us.1.i.i = fmul float %_0.i247.us.us.us.i93.i, %_0.i215.us.us.us.i.i, !dbg !7586
  %_0.i215.us.us.us.1.i.i = fadd float %_0.i231.us.us.us.1.i.i, 0x3FAC91CE60000000, !dbg !7588
  %_0.i231.us.us.us.2.i.i = fmul float %_0.i247.us.us.us.i93.i, %_0.i215.us.us.us.1.i.i, !dbg !7586
  %_0.i215.us.us.us.2.i.i = fadd float %_0.i231.us.us.us.2.i.i, 0x3FCEBDB560000000, !dbg !7588
  %_0.i231.us.us.us.3.i.i = fmul float %_0.i247.us.us.us.i93.i, %_0.i215.us.us.us.2.i.i, !dbg !7586
  %_0.i215.us.us.us.3.i.i = fadd float %_0.i231.us.us.us.3.i.i, 0x3FE62E4BA0000000, !dbg !7588
  %_0.i230.us.us.us.i.i = fmul float %_0.i247.us.us.us.i93.i, %_0.i215.us.us.us.3.i.i, !dbg !7590
  %_0.i214.us.us.us.i.i = fadd float %_0.i230.us.us.us.i.i, 1.000000e+00, !dbg !7592
  %biased.i161.us.us.us.i.i = fadd float %290, 0x4160000FE0000000, !dbg !7594
  %_4.i162.us.us.us.i.i = bitcast float %biased.i161.us.us.us.i.i to i32, !dbg !7596
  %_3.i163.us.us.us.i.i = shl i32 %_4.i162.us.us.us.i.i, 23, !dbg !7598
  %_0.i164.us.us.us.i.i = bitcast i32 %_3.i163.us.us.us.i.i to float, !dbg !7599
  %_0.i229.us.us.us.i.i = fmul float %_0.i214.us.us.us.i.i, %_0.i164.us.us.us.i.i, !dbg !7601
  %_3.i165.us.us.us.i.i = fcmp une float %_0.i286.us.us.us.i92.i, 0.000000e+00, !dbg !7603
  %_0.i394572.not.us.us.us.i.i = and i1 %_3.i176.i.i, %_3.i165.us.us.us.i.i, !dbg !7605
  %_0.i232.us.us.us.i.i = fmul float %_0.i269.us.us.us.i63.i, %_0.i229.us.us.us.i.i, !dbg !7605
  %_4.i294.v.us.us.us.i.i = select i1 %_0.i394572.not.us.us.us.i.i, float %_0.i232.us.us.us.i.i, float %_0.i269.us.us.us.i63.i, !dbg !7607
  %_0.i227.us.us.us.i.i = fmul float %_0.i246.us.us.us.i108.i, %_0.i213.us.us.us.3.i.i, !dbg !7609
  %_0.i212.us.us.us.i.i = fadd float %_0.i227.us.us.us.i.i, 1.000000e+00, !dbg !7611
  %biased.i.us.us.us.i109.i = fadd float %300, 0x4160000FE0000000, !dbg !7613
  %_4.i158.us.us.us.i.i = bitcast float %biased.i.us.us.us.i109.i to i32, !dbg !7615
  %_3.i159.us.us.us.i.i = shl i32 %_4.i158.us.us.us.i.i, 23, !dbg !7617
  %_0.i160.us.us.us.i.i = bitcast i32 %_3.i159.us.us.us.i.i to float, !dbg !7618
  %_0.i226.us.us.us.i.i = fmul float %_0.i212.us.us.us.i.i, %_0.i160.us.us.us.i.i, !dbg !7620
  %_3.i166.us.us.us.i.i = fcmp une float %_0.i290.us.us.us.i107.i, 0.000000e+00, !dbg !7622
  %_0.i397589.not.us.us.us.i.i = and i1 %_3.i190.i.i, %_3.i166.us.us.us.i.i, !dbg !7624
  %_0.i238.us.us.us.i.i = fmul float %_0.i267.us.us.us.i65.i, %_0.i226.us.us.us.i.i, !dbg !7624
  %_4.i347.v.us.us.us.i.i = select i1 %_0.i397589.not.us.us.us.i.i, float %_0.i238.us.us.us.i.i, float %_0.i267.us.us.us.i65.i, !dbg !7626
  store float %_4.i294.v.us.us.us.i.i, ptr %_123.i.us.us.us.i47.i, align 4, !dbg !7628, !alias.scope !7631, !noalias !7201
  store float %_4.i347.v.us.us.us.i.i, ptr %_141.i.us.us.us.i53.i, align 4, !dbg !7634, !alias.scope !7636, !noalias !7224
  %exitcond1491.not.i.i = icmp eq i64 %276, %_55.i, !dbg !7639
  br i1 %exitcond1491.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i, label %bb42.i.us.us.us.i46.i, !dbg !7642

bb42.i.us.i118.i:                                 ; preds = %bb40.i.lr.ph.split.us.i.i, %bb24.i.us.lr.ph.split.us.us.i157.i
  %_86.i19980.us.i.i = phi float [ %_0.i290.us.i181.i, %bb24.i.us.lr.ph.split.us.us.i157.i ], [ %.promoted979.i.i, %bb40.i.lr.ph.split.us.i.i ]
  %_67.i11978.us.i.i = phi float [ %_0.i367.us.i177.i, %bb24.i.us.lr.ph.split.us.us.i157.i ], [ %.promoted977.i.i, %bb40.i.lr.ph.split.us.i.i ]
  %_86.i86976.us.i.i = phi float [ %_0.i286.us.i166.i, %bb24.i.us.lr.ph.split.us.us.i157.i ], [ %.promoted975.i.i, %bb40.i.lr.ph.split.us.i.i ]
  %_67.i67974.us.i.i = phi float [ %_0.i315.us.i.i, %bb24.i.us.lr.ph.split.us.us.i157.i ], [ %.promoted973.i.i, %bb40.i.lr.ph.split.us.i.i ]
  %iter.sroa.0.0.i938.us.i.i = phi i64 [ %301, %bb24.i.us.lr.ph.split.us.us.i157.i ], [ 0, %bb40.i.lr.ph.split.us.i.i ]
  %301 = add nuw nsw i64 %iter.sroa.0.0.i938.us.i.i, 1, !dbg !7159
  %_27.i.us.i114.i = trunc i64 %iter.sroa.0.0.i938.us.i.i to i32, !dbg !7173
  %now.i.us.i115.i = add i32 %base.i.i34.i, %_27.i.us.i114.i, !dbg !7176
  %_30.i.us.i116.i = and i32 %now.i.us.i115.i, %_52.i28.i, !dbg !7179
  %_29.i.us.i117.i = zext i32 %_30.i.us.i116.i to i64, !dbg !7181
  %_123.i.us.i119.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %iter.sroa.0.0.i938.us.i.i, !dbg !7182
  %_124.not.not.i.us.i120.i = icmp ugt i64 %_58.1.i22.i, %_29.i.us.i117.i, !dbg !7191
  br i1 %_124.not.not.i.us.i120.i, label %bb45.i.us.i121.i, label %bb46.i.i49.i, !dbg !7191, !prof !2704

bb45.i.us.i121.i:                                 ; preds = %bb42.i.us.i118.i
  %_0.i273.us.i122.i = load float, ptr %_123.i.us.i119.i, align 4, !dbg !7196, !alias.scope !7198, !noalias !7201, !noundef !12
  %_133.i.us.i123.i = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_29.i.us.i117.i, !dbg !7202
  store float %_0.i273.us.i122.i, ptr %_133.i.us.i123.i, align 4, !dbg !7206, !alias.scope !7208, !noalias !7211
  %_141.i.us.i124.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %iter.sroa.0.0.i938.us.i.i, !dbg !7212
  %_142.not.not.i.us.i125.i = icmp ugt i64 %_60.1.i25.i, %_29.i.us.i117.i, !dbg !7643
  br i1 %_142.not.not.i.us.i125.i, label %bb50.i.us.i128.i, label %bb51.i.i126.i, !dbg !7643, !prof !2704

bb50.i.us.i128.i:                                 ; preds = %bb45.i.us.i121.i
  %_0.i271.us.i129.i = load float, ptr %_141.i.us.i124.i, align 4, !dbg !7219, !alias.scope !7221, !noalias !7224, !noundef !12
  %_149.i.us.i130.i = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_29.i.us.i117.i, !dbg !7225
  store float %_0.i271.us.i129.i, ptr %_149.i.us.i130.i, align 4, !dbg !7232, !alias.scope !7234, !noalias !7211
  %_52.i.us.i131.i = sub i32 %now.i.us.i115.i, %_53.i29.i, !dbg !7237
  %_51.i.us.i132.i = and i32 %_52.i.us.i131.i, %_52.i28.i, !dbg !7240
  %_50.i.us.i133.i = zext i32 %_51.i.us.i132.i to i64, !dbg !7241
  %_182.not.not.i.us.i134.i = icmp ugt i64 %_58.1.i22.i, %_50.i.us.i133.i, !dbg !7242
  br i1 %_182.not.not.i.us.i134.i, label %bb60.i.us.i135.i, label %bb61.i.i60.i, !dbg !7242, !prof !2704

bb60.i.us.i135.i:                                 ; preds = %bb50.i.us.i128.i
  %_189.i.us.i136.i = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_50.i.us.i133.i, !dbg !7247
  %_0.i269.us.i137.i = load float, ptr %_189.i.us.i136.i, align 4, !dbg !7251, !alias.scope !7253, !noalias !7211, !noundef !12
  %_190.not.not.i.us.i138.i = icmp ugt i64 %_60.1.i25.i, %_50.i.us.i133.i, !dbg !7644
  br i1 %_190.not.not.i.us.i138.i, label %bb63.i.us.i141.i, label %bb64.i.i139.i, !dbg !7644, !prof !2704

bb63.i.us.i141.i:                                 ; preds = %bb60.i.us.i135.i
  %_195.i.us.i142.i = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_50.i.us.i133.i, !dbg !7256
  %_0.i267.us.i143.i = load float, ptr %_195.i.us.i142.i, align 4, !dbg !7264, !alias.scope !7266, !noalias !7211, !noundef !12
  %_66.i.us.i144.i = sub i32 %now.i.us.i115.i, %_67.i.i35.i
  %_65.i.us.i145.i = and i32 %_66.i.us.i144.i, %_52.i28.i
  %_64.i.us.i146.i = zext i32 %_65.i.us.i145.i to i64
  %_74.i.us.i147.i = sub i32 %now.i.us.i115.i, %_75.i.i36.i
  %_73.i.us.i148.i = and i32 %_74.i.us.i147.i, %_52.i28.i
  %_72.i.us.i149.i = zext i32 %_73.i.us.i148.i to i64
  %_80.i.us.i150.i = icmp ugt i64 %_58.1.i22.i, %_64.i.us.i146.i
  %302 = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_64.i.us.i146.i
  %303 = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_64.i.us.i146.i
  %_86.i.us.i151.i = icmp ugt i64 %_60.1.i25.i, %_72.i.us.i149.i
  %304 = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_72.i.us.i149.i
  %305 = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_72.i.us.i149.i
  br i1 %_80.i.us.i150.i, label %bb63.i.split.us.us.i153.i, label %bb63.i.split.i75.i

bb63.i.split.us.us.i153.i:                        ; preds = %bb63.i.us.i141.i
  %_84.i.us.i154.i = icmp ugt i64 %_60.1.i25.i, %_64.i.us.i146.i
  br i1 %_84.i.us.i154.i, label %bb24.i.us.lr.ph.us.i155.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i78.i, !dbg !7645

bb24.i.us.lr.ph.us.i155.i:                        ; preds = %bb63.i.split.us.us.i153.i
  br i1 %_86.i.us.i151.i, label %bb24.i.us.lr.ph.split.us.us.i157.i, label %bb24.i.us.lr.ph.split.i81.i

bb24.i.us.lr.ph.split.us.us.i157.i:               ; preds = %bb24.i.us.lr.ph.us.i155.i
  %_82.i.us.us.i156.i = load float, ptr %303, align 4, !noalias !7211, !noundef !12
  %_87.i.us.us.us.i159.i = load float, ptr %305, align 4, !noalias !7211, !noundef !12
  %_85.i.us.us.le807.us.i.i = load float, ptr %304, align 4, !noalias !7211, !noundef !12
  %_78.i.us.le.us.i160.i = load float, ptr %302, align 4, !noalias !7211, !noundef !12
  %306 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.us.i160.i), !dbg !7277
  %307 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.us.i156.i), !dbg !7280
  %_3.i.i460.us.i.i = fcmp ule float %306, %307, !dbg !7282
  %_6.i.i462.us.i.i = bitcast float %306 to i32, !dbg !7285
  %_8.i.i464.us.i.i = bitcast float %307 to i32, !dbg !7288
  %_4.i.i467.us.i.i = select i1 %_3.i.i460.us.i.i, i32 %_8.i.i464.us.i.i, i32 %_6.i.i462.us.i.i, !dbg !7290
  %_4.i340.us.i.i = select i1 %_3.i188.i.i, i32 %_6.i.i462.us.i.i, i32 %_4.i.i467.us.i.i, !dbg !7291
  %_0.i237.us.i.i = fmul float %306, 5.000000e-01, !dbg !7293
  %_0.i236.us.i.i = fmul float %307, 5.000000e-01, !dbg !7295
  %_0.i216.us.i.i = fadd float %_0.i236.us.i.i, %_0.i237.us.i.i, !dbg !7297
  %_6.i328.us.i.i = bitcast float %_0.i216.us.i.i to i32, !dbg !7299
  %_4.i333.us.i.i = select i1 %_3.i186.i.i, i32 %_4.i340.us.i.i, i32 %_6.i328.us.i.i, !dbg !7302
  %_0.i334.us.i.i = bitcast i32 %_4.i333.us.i.i to float, !dbg !7303
  %_3.i.i452.us.i.i = fcmp ule float %_0.i334.us.i.i, 0x3E45798EE0000000, !dbg !7305
  %_4.i.i458.us.i.i = select i1 %_3.i.i452.us.i.i, i32 841731191, i32 %_4.i333.us.i.i, !dbg !7308
  %_0.i.i459.us.i.i = bitcast i32 %_4.i.i458.us.i.i to float, !dbg !7310
  %_3.i.i412.us.i.i = fcmp ule float %_0.i.i459.us.i.i, 0x3810000000000000, !dbg !7312
  %_4.i.i418.us.i.i = select i1 %_3.i.i412.us.i.i, i32 8388608, i32 %_4.i.i458.us.i.i, !dbg !7317
  %_5.i278.us.i.i = and i32 %_4.i.i418.us.i.i, 8388607, !dbg !7319
  %_4.i279.us.i.i = or disjoint i32 %_5.i278.us.i.i, 1065353216, !dbg !7319
  %significand.i280.us.i.i = bitcast i32 %_4.i279.us.i.i to float, !dbg !7321
  %_0.i245.us.i161.i = fadd float %significand.i280.us.i.i, -1.000000e+00, !dbg !7323
  %_0.i225.us.i.i = fmul float %_0.i245.us.i161.i, 0x3F9B17A960000000, !dbg !7325
  %308 = fsub float 0x3FBF9A8440000000, %_0.i225.us.i.i, !dbg !7327
  %_0.i225.us.1.i.i = fmul float %_0.i245.us.i161.i, %308, !dbg !7325
  %_0.i211.us.1.i.i = fadd float %_0.i225.us.1.i.i, 0xBFD1E3F400000000, !dbg !7327
  %_0.i225.us.2.i.i = fmul float %_0.i245.us.i161.i, %_0.i211.us.1.i.i, !dbg !7325
  %_0.i211.us.2.i.i = fadd float %_0.i225.us.2.i.i, 0x3FDD544F20000000, !dbg !7327
  %_0.i225.us.3.i.i = fmul float %_0.i245.us.i161.i, %_0.i211.us.2.i.i, !dbg !7325
  %_0.i211.us.3.i.i = fadd float %_0.i225.us.3.i.i, 0xBFE6FC2A60000000, !dbg !7327
  %_0.i225.us.4.i.i = fmul float %_0.i245.us.i161.i, %_0.i211.us.3.i.i, !dbg !7325
  %_0.i211.us.4.i.i = fadd float %_0.i225.us.4.i.i, 0x3FF714B2A0000000, !dbg !7327
  %_9.i281.us.i.i = lshr i32 %_4.i.i418.us.i.i, 23, !dbg !7329
  %_8.i282.us.i.i = or disjoint i32 %_9.i281.us.i.i, 1258291200, !dbg !7329
  %_7.i283.us.i.i = bitcast i32 %_8.i282.us.i.i to float, !dbg !7330
  %exponent.i284.us.i.i = fadd float %_7.i283.us.i.i, 0xC160000FE0000000, !dbg !7332
  %_0.i224.us.i.i = fmul float %_0.i245.us.i161.i, %_0.i211.us.4.i.i, !dbg !7333
  %_0.i210.us.i.i = fadd float %exponent.i284.us.i.i, %_0.i224.us.i.i, !dbg !7335
  %_0.i235.us.i.i = fmul float %_0.i210.us.i.i, 0x4018151820000000, !dbg !7337
  %_3.i.i527.us.inv.i.i = fcmp olt float %_0.i235.us.i.i, 2.400000e+01, !dbg !7339
  %_0.i.i534.us.i.i = select i1 %_3.i.i527.us.inv.i.i, float %_0.i235.us.i.i, float 2.400000e+01, !dbg !7339
  %_3.i.i444.us.inv.i.i = fcmp ogt float %_0.i.i534.us.i.i, -1.600000e+02, !dbg !7342
  %_0.i.i451.us.i.i = select i1 %_3.i.i444.us.inv.i.i, float %_0.i.i534.us.i.i, float -1.600000e+02, !dbg !7342
  %_55.i57.us.i.i = load float, ptr %257, align 4, !dbg !7345, !alias.scope !7346, !noalias !7349, !noundef !12
  %_3.i184.us.i.i = fcmp ule float %_55.i57.us.i.i, 0.000000e+00, !dbg !7351
  %_3.i170.us.i.i = fcmp oge float %_0.i.i451.us.i.i, %threshold.i30.i.i, !dbg !7353
  %_3.i168.us.i.i = fcmp oge float %_0.i.i451.us.i.i, %_0.i252.i.i, !dbg !7355
  %..i169.us.i.i = sext i1 %_3.i168.us.i.i to i32, !dbg !7357
  %_0.i402.us.i.i = sext i1 %_3.i170.us.i.i to i32, !dbg !7359
  %_0.i396.us.i.i = select i1 %_3.i184.us.i.i, i32 %_0.i402.us.i.i, i32 %..i169.us.i.i, !dbg !7359
  %_0.i408.us.i.i = xor i32 %..i169.us.i.i, -1, !dbg !7361
  %_3.i182.us.i.i = fcmp ogt float %_67.i67974.us.i.i, 0.000000e+00, !dbg !7363
  %_0.i401.us.i162.i = select i1 %_3.i182.us.i.i, i32 %_0.i408.us.i.i, i32 0, !dbg !7365
  %_0.i400.us.i.i = select i1 %_3.i184.us.i.i, i32 0, i32 %_0.i401.us.i162.i, !dbg !7367
  %_0.i395.us.i.i = or i32 %_0.i400.us.i.i, %_0.i396.us.i.i, !dbg !7369
  %_5.i323.us.i.i = and i32 %_0.i395.us.i.i, 1065353216, !dbg !7371
  %_0.i327.us.i.i = bitcast i32 %_5.i323.us.i.i to float, !dbg !7373
  %_0.i251.us.i163.i = fadd float %_67.i67974.us.i.i, -1.000000e+00, !dbg !7375
  %309 = trunc nsw i32 %_0.i400.us.i.i to i1, !dbg !7377
  %_4.i321.v.us.i.i = select i1 %309, float %_0.i251.us.i163.i, float %_67.i67974.us.i.i, !dbg !7377
  %310 = trunc nsw i32 %_0.i396.us.i.i to i1, !dbg !7379
  %_0.i315.us.i.i = select i1 %310, float %_71.i73564565.i.i, float %_4.i321.v.us.i.i, !dbg !7379
  store float %_0.i315.us.i.i, ptr %258, align 4, !dbg !7381, !alias.scope !7346, !noalias !7349
  store i32 %_5.i323.us.i.i, ptr %257, align 4, !dbg !7382, !alias.scope !7346, !noalias !7349
  %_0.i249.us.i164.i = fsub float %_0.i.i451.us.i.i, %threshold.i30.i.i, !dbg !7383
  %_0.i234.us.i.i = fmul float %_0.i250.i.i, %_0.i249.us.i164.i, !dbg !7385
  %_3.i.i436.inv.us.i.i = fcmp ogt float %_0.i234.us.i.i, %260, !dbg !7387
  %_4.i.i442.v.us.i.i = select i1 %_3.i.i436.inv.us.i.i, float %_0.i234.us.i.i, float %260, !dbg !7387
  %_3.i.i519.us.i.i = fcmp olt float %_4.i.i442.v.us.i.i, 0.000000e+00, !dbg !7390
  %311 = fcmp ule float %_0.i327.us.i.i, 0.000000e+00, !dbg !7393
  %312 = select i1 %311, i1 %_3.i.i519.us.i.i, i1 false, !dbg !7395
  %_0.i308.us.i.i = select i1 %312, float %_4.i.i442.v.us.i.i, float 0.000000e+00, !dbg !7395
  %_3.i178.us.i.i = fcmp ule float %_0.i308.us.i.i, %_86.i86976.us.i.i, !dbg !7396
  %_4.i301.us.i.i = select i1 %_3.i178.us.i.i, i32 %_88.i89568.i.i, i32 %_87.i88567.i.i, !dbg !7398
  %_0.i302.us.i.i = bitcast i32 %_4.i301.us.i.i to float, !dbg !7400
  %_0.i248.us.i165.i = fsub float %_0.i308.us.i.i, %_86.i86976.us.i.i, !dbg !7402
  %_4.i218.us.i.i = fmul float %_0.i248.us.i165.i, %_0.i302.us.i.i, !dbg !7404
  %_0.i219.us.i.i = fadd float %_86.i86976.us.i.i, %_4.i218.us.i.i, !dbg !7404
  %313 = tail call noundef float @llvm.fabs.f32(float %_0.i219.us.i.i), !dbg !7406
  %314 = fcmp uge float %313, 0x3BC79CA100000000, !dbg !7409
  %_0.i286.us.i166.i = select i1 %314, float %_0.i219.us.i.i, float 0.000000e+00, !dbg !7411
  store float %_0.i286.us.i166.i, ptr %261, align 4, !dbg !7412, !alias.scope !7346, !noalias !7349
  %_0.i233.us.i.i = fmul float %_0.i286.us.i166.i, 0x3FC542A5A0000000, !dbg !7413
  %_3.i.i428.us.inv.i.i = fcmp ogt float %_0.i233.us.i.i, -1.260000e+02, !dbg !7416
  %_0.i.i435.us.i.i = select i1 %_3.i.i428.us.inv.i.i, float %_0.i233.us.i.i, float -1.260000e+02, !dbg !7416
  %_3.i.i511.us.inv.i.i = fcmp olt float %_0.i.i435.us.i.i, 1.270000e+02, !dbg !7420
  %_0.i.i518.us.i.i = select i1 %_3.i.i511.us.inv.i.i, float %_0.i.i435.us.i.i, float 1.270000e+02, !dbg !7420
  %315 = tail call noundef float @llvm.floor.f32(float %_0.i.i518.us.i.i), !dbg !7423
  %_0.i247.us.i167.i = fsub float %_0.i.i518.us.i.i, %315, !dbg !7427
  %316 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le807.us.i.i), !dbg !7429
  %317 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.us.i159.i), !dbg !7433
  %_3.i.i494.us.i.i = fcmp ule float %316, %317, !dbg !7435
  %_6.i.i496.us.i.i = bitcast float %316 to i32, !dbg !7438
  %_8.i.i498.us.i.i = bitcast float %317 to i32, !dbg !7441
  %_4.i.i501.us.i.i = select i1 %_3.i.i494.us.i.i, i32 %_8.i.i498.us.i.i, i32 %_6.i.i496.us.i.i, !dbg !7443
  %_4.i392.us.i.i = select i1 %_3.i202.i.i, i32 %_6.i.i496.us.i.i, i32 %_4.i.i501.us.i.i, !dbg !7444
  %_0.i243.us.i.i = fmul float %316, 5.000000e-01, !dbg !7446
  %_0.i242.us.i.i = fmul float %317, 5.000000e-01, !dbg !7448
  %_0.i217.us.i.i = fadd float %_0.i242.us.i.i, %_0.i243.us.i.i, !dbg !7450
  %_6.i380.us.i.i = bitcast float %_0.i217.us.i.i to i32, !dbg !7452
  %_4.i385.us.i.i = select i1 %_3.i200.i.i, i32 %_4.i392.us.i.i, i32 %_6.i380.us.i.i, !dbg !7455
  %_0.i386.us.i.i = bitcast i32 %_4.i385.us.i.i to float, !dbg !7456
  %_3.i.i486.us.i.i = fcmp ule float %_0.i386.us.i.i, 0x3E45798EE0000000, !dbg !7458
  %_4.i.i492.us.i.i = select i1 %_3.i.i486.us.i.i, i32 841731191, i32 %_4.i385.us.i.i, !dbg !7461
  %_0.i.i493.us.i.i = bitcast i32 %_4.i.i492.us.i.i to float, !dbg !7463
  %_3.i.i.us.i168.i = fcmp ule float %_0.i.i493.us.i.i, 0x3810000000000000, !dbg !7465
  %_4.i.i.us.i169.i = select i1 %_3.i.i.us.i168.i, i32 8388608, i32 %_4.i.i492.us.i.i, !dbg !7470
  %_5.i274.us.i.i = and i32 %_4.i.i.us.i169.i, 8388607, !dbg !7472
  %_4.i275.us.i.i = or disjoint i32 %_5.i274.us.i.i, 1065353216, !dbg !7472
  %significand.i.us.i170.i = bitcast i32 %_4.i275.us.i.i to float, !dbg !7474
  %_0.i244.us.i171.i = fadd float %significand.i.us.i170.i, -1.000000e+00, !dbg !7476
  %_0.i223.us.i.i = fmul float %_0.i244.us.i171.i, 0x3F9B17A960000000, !dbg !7478
  %318 = fsub float 0x3FBF9A8440000000, %_0.i223.us.i.i, !dbg !7480
  %_0.i223.us.1.i.i = fmul float %_0.i244.us.i171.i, %318, !dbg !7478
  %_0.i209.us.1.i.i = fadd float %_0.i223.us.1.i.i, 0xBFD1E3F400000000, !dbg !7480
  %_0.i223.us.2.i.i = fmul float %_0.i244.us.i171.i, %_0.i209.us.1.i.i, !dbg !7478
  %_0.i209.us.2.i.i = fadd float %_0.i223.us.2.i.i, 0x3FDD544F20000000, !dbg !7480
  %_0.i223.us.3.i.i = fmul float %_0.i244.us.i171.i, %_0.i209.us.2.i.i, !dbg !7478
  %_0.i209.us.3.i.i = fadd float %_0.i223.us.3.i.i, 0xBFE6FC2A60000000, !dbg !7480
  %_0.i223.us.4.i.i = fmul float %_0.i244.us.i171.i, %_0.i209.us.3.i.i, !dbg !7478
  %_0.i209.us.4.i.i = fadd float %_0.i223.us.4.i.i, 0x3FF714B2A0000000, !dbg !7480
  %_9.i.us.i172.i = lshr i32 %_4.i.i.us.i169.i, 23, !dbg !7482
  %_8.i276.us.i.i = or disjoint i32 %_9.i.us.i172.i, 1258291200, !dbg !7482
  %_7.i.us.i173.i = bitcast i32 %_8.i276.us.i.i to float, !dbg !7483
  %exponent.i.us.i174.i = fadd float %_7.i.us.i173.i, 0xC160000FE0000000, !dbg !7485
  %_0.i222.us.i.i = fmul float %_0.i244.us.i171.i, %_0.i209.us.4.i.i, !dbg !7486
  %_0.i208.us.i.i = fadd float %exponent.i.us.i174.i, %_0.i222.us.i.i, !dbg !7488
  %_0.i241.us.i.i = fmul float %_0.i208.us.i.i, 0x4018151820000000, !dbg !7490
  %_3.i.i543.us.inv.i.i = fcmp olt float %_0.i241.us.i.i, 2.400000e+01, !dbg !7492
  %_0.i.i550.us.i.i = select i1 %_3.i.i543.us.inv.i.i, float %_0.i241.us.i.i, float 2.400000e+01, !dbg !7492
  %_3.i.i478.us.inv.i.i = fcmp ogt float %_0.i.i550.us.i.i, -1.600000e+02, !dbg !7495
  %_0.i.i485.us.i.i = select i1 %_3.i.i478.us.inv.i.i, float %_0.i.i550.us.i.i, float -1.600000e+02, !dbg !7495
  %_55.i9.us.i.i = load float, ptr %269, align 4, !dbg !7498, !alias.scope !7499, !noalias !7502, !noundef !12
  %_3.i198.us.i175.i = fcmp ule float %_55.i9.us.i.i, 0.000000e+00, !dbg !7504
  %_3.i174.us.i.i = fcmp oge float %_0.i.i485.us.i.i, %threshold.i.i.i, !dbg !7506
  %_3.i172.us.i.i = fcmp oge float %_0.i.i485.us.i.i, %_0.i257.i.i, !dbg !7508
  %..i173.us.i.i = sext i1 %_3.i172.us.i.i to i32, !dbg !7510
  %_0.i406.us.i.i = sext i1 %_3.i174.us.i.i to i32, !dbg !7512
  %_0.i399.us.i.i = select i1 %_3.i198.us.i175.i, i32 %_0.i406.us.i.i, i32 %..i173.us.i.i, !dbg !7512
  %_0.i410.us.i.i = xor i32 %..i173.us.i.i, -1, !dbg !7514
  %_3.i196.us.i176.i = fcmp ogt float %_67.i11978.us.i.i, 0.000000e+00, !dbg !7516
  %_0.i405.us.i.i = select i1 %_3.i196.us.i176.i, i32 %_0.i410.us.i.i, i32 0, !dbg !7518
  %_0.i404.us.i.i = select i1 %_3.i198.us.i175.i, i32 0, i32 %_0.i405.us.i.i, !dbg !7520
  %_0.i398.us.i.i = or i32 %_0.i404.us.i.i, %_0.i399.us.i.i, !dbg !7522
  %_5.i375.us.i.i = and i32 %_0.i398.us.i.i, 1065353216, !dbg !7524
  %_0.i379.us.i.i = bitcast i32 %_5.i375.us.i.i to float, !dbg !7526
  %_0.i256.us.i.i = fadd float %_67.i11978.us.i.i, -1.000000e+00, !dbg !7528
  %319 = trunc nsw i32 %_0.i404.us.i.i to i1, !dbg !7530
  %_4.i373.v.us.i.i = select i1 %319, float %_0.i256.us.i.i, float %_67.i11978.us.i.i, !dbg !7530
  %320 = trunc nsw i32 %_0.i399.us.i.i to i1, !dbg !7532
  %_0.i367.us.i177.i = select i1 %320, float %_71.i581582.i.i, float %_4.i373.v.us.i.i, !dbg !7532
  store float %_0.i367.us.i177.i, ptr %270, align 4, !dbg !7534, !alias.scope !7499, !noalias !7502
  store i32 %_5.i375.us.i.i, ptr %269, align 4, !dbg !7535, !alias.scope !7499, !noalias !7502
  %_0.i254.us.i178.i = fsub float %_0.i.i485.us.i.i, %threshold.i.i.i, !dbg !7536
  %_0.i240.us.i.i = fmul float %_0.i255.i.i, %_0.i254.us.i178.i, !dbg !7538
  %_3.i.i469.inv.us.i.i = fcmp ogt float %_0.i240.us.i.i, %272, !dbg !7540
  %_4.i.i476.v.us.i.i = select i1 %_3.i.i469.inv.us.i.i, float %_0.i240.us.i.i, float %272, !dbg !7540
  %_3.i.i535.us.i.i = fcmp olt float %_4.i.i476.v.us.i.i, 0.000000e+00, !dbg !7543
  %321 = fcmp ule float %_0.i379.us.i.i, 0.000000e+00, !dbg !7546
  %322 = select i1 %321, i1 %_3.i.i535.us.i.i, i1 false, !dbg !7548
  %_0.i360.us.i.i = select i1 %322, float %_4.i.i476.v.us.i.i, float 0.000000e+00, !dbg !7548
  %_3.i192.us.i.i = fcmp ule float %_0.i360.us.i.i, %_86.i19980.us.i.i, !dbg !7549
  %_4.i354.us.i.i = select i1 %_3.i192.us.i.i, i32 %_88.i22585.i.i, i32 %_87.i21584.i.i, !dbg !7551
  %_0.i.us.i179.i = bitcast i32 %_4.i354.us.i.i to float, !dbg !7553
  %_0.i253.us.i180.i = fsub float %_0.i360.us.i.i, %_86.i19980.us.i.i, !dbg !7555
  %_4.i220.us.i.i = fmul float %_0.i253.us.i180.i, %_0.i.us.i179.i, !dbg !7557
  %_0.i221.us.i.i = fadd float %_86.i19980.us.i.i, %_4.i220.us.i.i, !dbg !7557
  %323 = tail call noundef float @llvm.fabs.f32(float %_0.i221.us.i.i), !dbg !7559
  %324 = fcmp uge float %323, 0x3BC79CA100000000, !dbg !7562
  %_0.i290.us.i181.i = select i1 %324, float %_0.i221.us.i.i, float 0.000000e+00, !dbg !7564
  store float %_0.i290.us.i181.i, ptr %273, align 4, !dbg !7565, !alias.scope !7499, !noalias !7502
  %_0.i239.us.i.i = fmul float %_0.i290.us.i181.i, 0x3FC542A5A0000000, !dbg !7566
  %_3.i.i420.us.inv.i.i = fcmp ogt float %_0.i239.us.i.i, -1.260000e+02, !dbg !7569
  %_0.i.i427.us.i.i = select i1 %_3.i.i420.us.inv.i.i, float %_0.i239.us.i.i, float -1.260000e+02, !dbg !7569
  %_3.i.i503.us.inv.i.i = fcmp olt float %_0.i.i427.us.i.i, 1.270000e+02, !dbg !7573
  %_0.i.i510.us.i.i = select i1 %_3.i.i503.us.inv.i.i, float %_0.i.i427.us.i.i, float 1.270000e+02, !dbg !7573
  %325 = tail call noundef float @llvm.floor.f32(float %_0.i.i510.us.i.i), !dbg !7576
  %_0.i246.us.i182.i = fsub float %_0.i.i510.us.i.i, %325, !dbg !7580
  %_0.i228.us.i.i = fmul float %_0.i246.us.i182.i, 0x3F5E974FA0000000, !dbg !7582
  %_0.i213.us.i.i = fadd float %_0.i228.us.i.i, 0x3F82778560000000, !dbg !7584
  %_0.i228.us.1.i.i = fmul float %_0.i246.us.i182.i, %_0.i213.us.i.i, !dbg !7582
  %_0.i213.us.1.i.i = fadd float %_0.i228.us.1.i.i, 0x3FAC91CE60000000, !dbg !7584
  %_0.i228.us.2.i.i = fmul float %_0.i246.us.i182.i, %_0.i213.us.1.i.i, !dbg !7582
  %_0.i213.us.2.i.i = fadd float %_0.i228.us.2.i.i, 0x3FCEBDB560000000, !dbg !7584
  %_0.i228.us.3.i.i = fmul float %_0.i246.us.i182.i, %_0.i213.us.2.i.i, !dbg !7582
  %_0.i213.us.3.i.i = fadd float %_0.i228.us.3.i.i, 0x3FE62E4BA0000000, !dbg !7584
  %_0.i231.us.i.i = fmul float %_0.i247.us.i167.i, 0x3F5E974FA0000000, !dbg !7586
  %_0.i215.us.i.i = fadd float %_0.i231.us.i.i, 0x3F82778560000000, !dbg !7588
  %_0.i231.us.1.i.i = fmul float %_0.i247.us.i167.i, %_0.i215.us.i.i, !dbg !7586
  %_0.i215.us.1.i.i = fadd float %_0.i231.us.1.i.i, 0x3FAC91CE60000000, !dbg !7588
  %_0.i231.us.2.i.i = fmul float %_0.i247.us.i167.i, %_0.i215.us.1.i.i, !dbg !7586
  %_0.i215.us.2.i.i = fadd float %_0.i231.us.2.i.i, 0x3FCEBDB560000000, !dbg !7588
  %_0.i231.us.3.i.i = fmul float %_0.i247.us.i167.i, %_0.i215.us.2.i.i, !dbg !7586
  %_0.i215.us.3.i.i = fadd float %_0.i231.us.3.i.i, 0x3FE62E4BA0000000, !dbg !7588
  %_0.i230.us.i.i = fmul float %_0.i247.us.i167.i, %_0.i215.us.3.i.i, !dbg !7590
  %_0.i214.us.i.i = fadd float %_0.i230.us.i.i, 1.000000e+00, !dbg !7592
  %biased.i161.us.i.i = fadd float %315, 0x4160000FE0000000, !dbg !7594
  %_4.i162.us.i.i = bitcast float %biased.i161.us.i.i to i32, !dbg !7596
  %_3.i163.us.i.i = shl i32 %_4.i162.us.i.i, 23, !dbg !7598
  %_0.i164.us.i.i = bitcast i32 %_3.i163.us.i.i to float, !dbg !7599
  %_0.i229.us.i.i = fmul float %_0.i214.us.i.i, %_0.i164.us.i.i, !dbg !7601
  %_3.i165.us.i.i = fcmp une float %_0.i286.us.i166.i, 0.000000e+00, !dbg !7603
  %_0.i394572.not.us.i.i = and i1 %_3.i176.i.i, %_3.i165.us.i.i, !dbg !7605
  %_0.i232.us.i.i = fmul float %_0.i269.us.i137.i, %_0.i229.us.i.i, !dbg !7605
  %_4.i294.v.us.i.i = select i1 %_0.i394572.not.us.i.i, float %_0.i232.us.i.i, float %_0.i269.us.i137.i, !dbg !7607
  %_0.i227.us.i.i = fmul float %_0.i246.us.i182.i, %_0.i213.us.3.i.i, !dbg !7609
  %_0.i212.us.i.i = fadd float %_0.i227.us.i.i, 1.000000e+00, !dbg !7611
  %biased.i.us.i183.i = fadd float %325, 0x4160000FE0000000, !dbg !7613
  %_4.i158.us.i.i = bitcast float %biased.i.us.i183.i to i32, !dbg !7615
  %_3.i159.us.i.i = shl i32 %_4.i158.us.i.i, 23, !dbg !7617
  %_0.i160.us.i.i = bitcast i32 %_3.i159.us.i.i to float, !dbg !7618
  %_0.i226.us.i.i = fmul float %_0.i212.us.i.i, %_0.i160.us.i.i, !dbg !7620
  %_3.i166.us.i.i = fcmp une float %_0.i290.us.i181.i, 0.000000e+00, !dbg !7622
  %_0.i397589.not.us.i.i = and i1 %_3.i190.i.i, %_3.i166.us.i.i, !dbg !7624
  %_0.i238.us.i.i = fmul float %_0.i267.us.i143.i, %_0.i226.us.i.i, !dbg !7624
  %_4.i347.v.us.i.i = select i1 %_0.i397589.not.us.i.i, float %_0.i238.us.i.i, float %_0.i267.us.i143.i, !dbg !7626
  store float %_4.i294.v.us.i.i, ptr %_123.i.us.i119.i, align 4, !dbg !7628, !alias.scope !7631, !noalias !7201
  store float %_4.i347.v.us.i.i, ptr %_141.i.us.i124.i, align 4, !dbg !7634, !alias.scope !7636, !noalias !7224
  %exitcond1493.not.i.i = icmp eq i64 %301, %_55.i, !dbg !7639
  br i1 %exitcond1493.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i, label %bb42.i.us.i118.i, !dbg !7642, !llvm.loop !7646

bb42.i.i.i:                                       ; preds = %bb27.i, %bb28.i.us.us.lr.ph.i.i
  %_86.i19980.i.i = phi float [ %_0.i290.i.i, %bb28.i.us.us.lr.ph.i.i ], [ %.promoted979.i.i, %bb27.i ]
  %_67.i11978.i.i = phi float [ %_0.i367.i.i, %bb28.i.us.us.lr.ph.i.i ], [ %.promoted977.i.i, %bb27.i ]
  %_86.i86976.i.i = phi float [ %_0.i286.i.i, %bb28.i.us.us.lr.ph.i.i ], [ %.promoted975.i.i, %bb27.i ]
  %_67.i67974.i.i = phi float [ %_0.i315.i.i, %bb28.i.us.us.lr.ph.i.i ], [ %.promoted973.i.i, %bb27.i ]
  %iter.sroa.0.0.i938.i.i = phi i64 [ %326, %bb28.i.us.us.lr.ph.i.i ], [ 0, %bb27.i ]
  %326 = add nuw nsw i64 %iter.sroa.0.0.i938.i.i, 1, !dbg !7159
  %_27.i.i.i = trunc i64 %iter.sroa.0.0.i938.i.i to i32, !dbg !7173
  %now.i.i.i = add i32 %base.i.i34.i, %_27.i.i.i, !dbg !7176
  %_30.i.i.i = and i32 %now.i.i.i, %_52.i28.i, !dbg !7179
  %_29.i.i.i = zext i32 %_30.i.i.i to i64, !dbg !7181
  %_123.i.i.i = getelementptr inbounds nuw float, ptr %_59.i, i64 %iter.sroa.0.0.i938.i.i, !dbg !7182
  %_124.not.not.i.i.i = icmp ugt i64 %_58.1.i22.i, %_29.i.i.i, !dbg !7191
  br i1 %_124.not.not.i.i.i, label %bb45.i.i.i, label %bb46.i.i49.i, !dbg !7191, !prof !2704

bb46.i.i49.i:                                     ; preds = %bb42.i.us.us.us.i46.i, %bb42.i.us.i118.i, %bb42.i.i.i
  %.us-phi992.i.i = phi i64 [ %_29.i.us.i117.i, %bb42.i.us.i118.i ], [ %_29.i.i.i, %bb42.i.i.i ], [ %_29.i.us.us.us.i44.i, %bb42.i.us.us.us.i46.i ]
  %_36.i.le922.i.i = add nuw nsw i64 %.us-phi992.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi992.i.i, i64 noundef %_36.i.le922.i.i, i64 noundef %_58.1.i22.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cad5a7b24766ffcdda9cbfe066eb4078) #24, !dbg !7647, !noalias !7211
  unreachable, !dbg !7647

bb45.i.i.i:                                       ; preds = %bb42.i.i.i
  %_0.i273.i.i = load float, ptr %_123.i.i.i, align 4, !dbg !7196, !alias.scope !7198, !noalias !7201, !noundef !12
  %_133.i.i.i = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_29.i.i.i, !dbg !7202
  store float %_0.i273.i.i, ptr %_133.i.i.i, align 4, !dbg !7206, !alias.scope !7208, !noalias !7211
  %exitcond1494.not.i.i = icmp eq i64 %iter.sroa.0.0.i938.i.i, %_63.i, !dbg !7648
  br i1 %exitcond1494.not.i.i, label %bb49.i.i.i, label %bb48.i.i.i, !dbg !7648, !prof !180

bb49.i.i.i:                                       ; preds = %bb45.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef range(i64 0, 2305843009213693952) %_63.i, i64 noundef %326, i64 noundef range(i64 0, 2305843009213693952) %_63.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_aebeae245abdbd708a3d39b73262e641) #24, !dbg !7649, !noalias !7211
  unreachable, !dbg !7649

bb48.i.i.i:                                       ; preds = %bb45.i.i.i
  %_141.i.i.i = getelementptr inbounds nuw float, ptr %_67.i, i64 %iter.sroa.0.0.i938.i.i, !dbg !7212
  %_142.not.not.i.i.i = icmp ugt i64 %_60.1.i25.i, %_29.i.i.i, !dbg !7643
  br i1 %_142.not.not.i.i.i, label %bb50.i.i.i, label %bb51.i.i126.i, !dbg !7643, !prof !2704

bb51.i.i126.i:                                    ; preds = %bb45.i.us.i121.i, %bb48.i.i.i
  %.us-phi998.i.i = phi i64 [ %_29.i.i.i, %bb48.i.i.i ], [ %_29.i.us.i117.i, %bb45.i.us.i121.i ]
  %_36.i.le.i127.i = add nuw nsw i64 %.us-phi998.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi998.i.i, i64 noundef %_36.i.le.i127.i, i64 noundef %_60.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_72e24c5578221343e8a784545a3910d2) #24, !dbg !7650, !noalias !7211
  unreachable, !dbg !7650

bb50.i.i.i:                                       ; preds = %bb48.i.i.i
  %_0.i271.i.i = load float, ptr %_141.i.i.i, align 4, !dbg !7219, !alias.scope !7221, !noalias !7224, !noundef !12
  %_149.i.i.i = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_29.i.i.i, !dbg !7225
  store float %_0.i271.i.i, ptr %_149.i.i.i, align 4, !dbg !7232, !alias.scope !7234, !noalias !7211
  %_52.i.i.i = sub i32 %now.i.i.i, %_53.i29.i, !dbg !7237
  %_51.i.i.i = and i32 %_52.i.i.i, %_52.i28.i, !dbg !7240
  %_50.i.i.i = zext i32 %_51.i.i.i to i64, !dbg !7241
  %_182.not.not.i.i.i = icmp ugt i64 %_58.1.i22.i, %_50.i.i.i, !dbg !7242
  br i1 %_182.not.not.i.i.i, label %bb60.i.i.i, label %bb61.i.i60.i, !dbg !7242, !prof !2704

bb61.i.i60.i:                                     ; preds = %bb45.i.us.us.us.i50.i, %bb50.i.us.i128.i, %bb50.i.i.i
  %.us-phi1004.i.i = phi i64 [ %_50.i.us.i133.i, %bb50.i.us.i128.i ], [ %_50.i.i.i, %bb50.i.i.i ], [ %_50.i.us.us.us.i58.i, %bb45.i.us.us.us.i50.i ]
  %_55.i.le920.i.i = add nuw nsw i64 %.us-phi1004.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1004.i.i, i64 noundef %_55.i.le920.i.i, i64 noundef %_58.1.i22.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ba99eeeb3482270ebe1c674b4013dd6a) #24, !dbg !7651, !noalias !7211
  unreachable, !dbg !7651

bb60.i.i.i:                                       ; preds = %bb50.i.i.i
  %_189.i.i.i = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_50.i.i.i, !dbg !7247
  %_0.i269.i.i = load float, ptr %_189.i.i.i, align 4, !dbg !7251, !alias.scope !7253, !noalias !7211, !noundef !12
  %_190.not.not.i.i.i = icmp ugt i64 %_60.1.i25.i, %_50.i.i.i, !dbg !7644
  br i1 %_190.not.not.i.i.i, label %bb63.i.i.i, label %bb64.i.i139.i, !dbg !7644, !prof !2704

bb64.i.i139.i:                                    ; preds = %bb60.i.us.i135.i, %bb60.i.i.i
  %.us-phi1010.i.i = phi i64 [ %_50.i.i.i, %bb60.i.i.i ], [ %_50.i.us.i133.i, %bb60.i.us.i135.i ]
  %_55.i.le.i140.i = add nuw nsw i64 %.us-phi1010.i.i, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %.us-phi1010.i.i, i64 noundef %_55.i.le.i140.i, i64 noundef %_60.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f4c27429e4885e1b619f1e4a3587acc) #24, !dbg !7652, !noalias !7211
  unreachable, !dbg !7652

bb63.i.i.i:                                       ; preds = %bb60.i.i.i
  %_195.i.i.i = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_50.i.i.i, !dbg !7256
  %_0.i267.i.i = load float, ptr %_195.i.i.i, align 4, !dbg !7264, !alias.scope !7266, !noalias !7211, !noundef !12
  %_66.i.i.i = sub i32 %now.i.i.i, %_67.i.i35.i
  %_65.i.i.i = and i32 %_66.i.i.i, %_52.i28.i
  %_64.i.i.i = zext i32 %_65.i.i.i to i64
  %_74.i.i.i = sub i32 %now.i.i.i, %_75.i.i36.i
  %_73.i.i.i = and i32 %_74.i.i.i, %_52.i28.i
  %_72.i.i.i = zext i32 %_73.i.i.i to i64
  %_80.i.i.i = icmp ugt i64 %_58.1.i22.i, %_64.i.i.i
  %327 = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_64.i.i.i
  %328 = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_64.i.i.i
  %_86.i.i.i = icmp ugt i64 %_60.1.i25.i, %_72.i.i.i
  %329 = getelementptr inbounds nuw float, ptr %_60.0.i24.i, i64 %_72.i.i.i
  %_88.i.i.i = icmp ugt i64 %_58.1.i22.i, %_72.i.i.i
  %330 = getelementptr inbounds nuw float, ptr %_58.0.i21.i, i64 %_72.i.i.i
  br i1 %_80.i.i.i, label %bb63.i.split.us.i.i, label %bb63.i.split.i75.i

bb63.i.split.us.i.i:                              ; preds = %bb63.i.i.i
  %_84.i.i.i = icmp ugt i64 %_60.1.i25.i, %_64.i.i.i
  br i1 %_84.i.i.i, label %bb24.i.us.lr.ph.i.i, label %bb63.i.split.us.panic20.i.split.us_crit_edge.i78.i, !dbg !7645

bb24.i.us.lr.ph.i.i:                              ; preds = %bb63.i.split.us.i.i
  %_82.i.us.i.i = load float, ptr %328, align 4, !noalias !7211, !noundef !12
  br i1 %_86.i.i.i, label %bb24.i.us.lr.ph.split.us.i.i, label %bb24.i.us.lr.ph.split.i81.i

bb24.i.us.lr.ph.split.us.i.i:                     ; preds = %bb24.i.us.lr.ph.i.i
  br i1 %_88.i.i.i, label %bb28.i.us.us.lr.ph.i.i, label %bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i83.i, !dbg !7269

bb28.i.us.us.lr.ph.i.i:                           ; preds = %bb24.i.us.lr.ph.split.us.i.i
  %_87.i.us.us.i.i = load float, ptr %330, align 4, !noalias !7211, !noundef !12
  %_85.i.us.us.le807.i.i = load float, ptr %329, align 4, !noalias !7211, !noundef !12
  %_78.i.us.le.i.i = load float, ptr %327, align 4, !noalias !7211, !noundef !12
  %331 = tail call noundef float @llvm.fabs.f32(float %_78.i.us.le.i.i), !dbg !7277
  %332 = tail call noundef float @llvm.fabs.f32(float %_82.i.us.i.i), !dbg !7280
  %_3.i.i460.i.i = fcmp ule float %331, %332, !dbg !7282
  %_6.i.i462.i.i = bitcast float %331 to i32, !dbg !7285
  %_8.i.i464.i.i = bitcast float %332 to i32, !dbg !7288
  %_4.i.i467.i.i = select i1 %_3.i.i460.i.i, i32 %_8.i.i464.i.i, i32 %_6.i.i462.i.i, !dbg !7290
  %_4.i340.i.i = select i1 %_3.i188.i.i, i32 %_6.i.i462.i.i, i32 %_4.i.i467.i.i, !dbg !7291
  %_0.i237.i.i = fmul float %331, 5.000000e-01, !dbg !7293
  %_0.i236.i.i = fmul float %332, 5.000000e-01, !dbg !7295
  %_0.i216.i.i = fadd float %_0.i236.i.i, %_0.i237.i.i, !dbg !7297
  %_6.i328.i.i = bitcast float %_0.i216.i.i to i32, !dbg !7299
  %_4.i333.i.i = select i1 %_3.i186.i.i, i32 %_4.i340.i.i, i32 %_6.i328.i.i, !dbg !7302
  %_0.i334.i.i = bitcast i32 %_4.i333.i.i to float, !dbg !7303
  %_3.i.i452.i.i = fcmp ule float %_0.i334.i.i, 0x3E45798EE0000000, !dbg !7305
  %_4.i.i458.i.i = select i1 %_3.i.i452.i.i, i32 841731191, i32 %_4.i333.i.i, !dbg !7308
  %_0.i.i459.i.i = bitcast i32 %_4.i.i458.i.i to float, !dbg !7310
  %_3.i.i412.i.i = fcmp ule float %_0.i.i459.i.i, 0x3810000000000000, !dbg !7312
  %_4.i.i418.i.i = select i1 %_3.i.i412.i.i, i32 8388608, i32 %_4.i.i458.i.i, !dbg !7317
  %_5.i278.i.i = and i32 %_4.i.i418.i.i, 8388607, !dbg !7319
  %_4.i279.i.i = or disjoint i32 %_5.i278.i.i, 1065353216, !dbg !7319
  %significand.i280.i.i = bitcast i32 %_4.i279.i.i to float, !dbg !7321
  %_0.i245.i.i = fadd float %significand.i280.i.i, -1.000000e+00, !dbg !7323
  %_0.i225.i.i = fmul float %_0.i245.i.i, 0x3F9B17A960000000, !dbg !7325
  %333 = fsub float 0x3FBF9A8440000000, %_0.i225.i.i, !dbg !7327
  %_0.i225.1.i.i = fmul float %_0.i245.i.i, %333, !dbg !7325
  %_0.i211.1.i.i = fadd float %_0.i225.1.i.i, 0xBFD1E3F400000000, !dbg !7327
  %_0.i225.2.i.i = fmul float %_0.i245.i.i, %_0.i211.1.i.i, !dbg !7325
  %_0.i211.2.i.i = fadd float %_0.i225.2.i.i, 0x3FDD544F20000000, !dbg !7327
  %_0.i225.3.i.i = fmul float %_0.i245.i.i, %_0.i211.2.i.i, !dbg !7325
  %_0.i211.3.i.i = fadd float %_0.i225.3.i.i, 0xBFE6FC2A60000000, !dbg !7327
  %_0.i225.4.i.i = fmul float %_0.i245.i.i, %_0.i211.3.i.i, !dbg !7325
  %_0.i211.4.i.i = fadd float %_0.i225.4.i.i, 0x3FF714B2A0000000, !dbg !7327
  %_9.i281.i.i = lshr i32 %_4.i.i418.i.i, 23, !dbg !7329
  %_8.i282.i.i = or disjoint i32 %_9.i281.i.i, 1258291200, !dbg !7329
  %_7.i283.i.i = bitcast i32 %_8.i282.i.i to float, !dbg !7330
  %exponent.i284.i.i = fadd float %_7.i283.i.i, 0xC160000FE0000000, !dbg !7332
  %_0.i224.i.i = fmul float %_0.i245.i.i, %_0.i211.4.i.i, !dbg !7333
  %_0.i210.i.i = fadd float %exponent.i284.i.i, %_0.i224.i.i, !dbg !7335
  %_0.i235.i.i = fmul float %_0.i210.i.i, 0x4018151820000000, !dbg !7337
  %_3.i.i527.inv.i.i = fcmp olt float %_0.i235.i.i, 2.400000e+01, !dbg !7339
  %_0.i.i534.i.i = select i1 %_3.i.i527.inv.i.i, float %_0.i235.i.i, float 2.400000e+01, !dbg !7339
  %_3.i.i444.inv.i.i = fcmp ogt float %_0.i.i534.i.i, -1.600000e+02, !dbg !7342
  %_0.i.i451.i.i = select i1 %_3.i.i444.inv.i.i, float %_0.i.i534.i.i, float -1.600000e+02, !dbg !7342
  %_55.i57.i.i = load float, ptr %257, align 4, !dbg !7345, !alias.scope !7346, !noalias !7349, !noundef !12
  %_3.i184.i.i = fcmp ule float %_55.i57.i.i, 0.000000e+00, !dbg !7351
  %_3.i170.i.i = fcmp oge float %_0.i.i451.i.i, %threshold.i30.i.i, !dbg !7353
  %_3.i168.i.i = fcmp oge float %_0.i.i451.i.i, %_0.i252.i.i, !dbg !7355
  %..i169.i.i = sext i1 %_3.i168.i.i to i32, !dbg !7357
  %_0.i402.i.i = sext i1 %_3.i170.i.i to i32, !dbg !7359
  %_0.i396.i.i = select i1 %_3.i184.i.i, i32 %_0.i402.i.i, i32 %..i169.i.i, !dbg !7359
  %_0.i408.i.i = xor i32 %..i169.i.i, -1, !dbg !7361
  %_3.i182.i.i = fcmp ogt float %_67.i67974.i.i, 0.000000e+00, !dbg !7363
  %_0.i401.i.i = select i1 %_3.i182.i.i, i32 %_0.i408.i.i, i32 0, !dbg !7365
  %_0.i400.i.i = select i1 %_3.i184.i.i, i32 0, i32 %_0.i401.i.i, !dbg !7367
  %_0.i395.i.i = or i32 %_0.i400.i.i, %_0.i396.i.i, !dbg !7369
  %_5.i323.i.i = and i32 %_0.i395.i.i, 1065353216, !dbg !7371
  %_0.i327.i.i = bitcast i32 %_5.i323.i.i to float, !dbg !7373
  %_0.i251.i.i = fadd float %_67.i67974.i.i, -1.000000e+00, !dbg !7375
  %334 = trunc nsw i32 %_0.i400.i.i to i1, !dbg !7377
  %_4.i321.v.i.i = select i1 %334, float %_0.i251.i.i, float %_67.i67974.i.i, !dbg !7377
  %335 = trunc nsw i32 %_0.i396.i.i to i1, !dbg !7379
  %_0.i315.i.i = select i1 %335, float %_71.i73564565.i.i, float %_4.i321.v.i.i, !dbg !7379
  store float %_0.i315.i.i, ptr %258, align 4, !dbg !7381, !alias.scope !7346, !noalias !7349
  store i32 %_5.i323.i.i, ptr %257, align 4, !dbg !7382, !alias.scope !7346, !noalias !7349
  %_0.i249.i.i = fsub float %_0.i.i451.i.i, %threshold.i30.i.i, !dbg !7383
  %_0.i234.i.i = fmul float %_0.i250.i.i, %_0.i249.i.i, !dbg !7385
  %_3.i.i436.inv.i.i = fcmp ogt float %_0.i234.i.i, %260, !dbg !7387
  %_4.i.i442.v.i.i = select i1 %_3.i.i436.inv.i.i, float %_0.i234.i.i, float %260, !dbg !7387
  %_3.i.i519.i.i = fcmp olt float %_4.i.i442.v.i.i, 0.000000e+00, !dbg !7390
  %336 = fcmp ule float %_0.i327.i.i, 0.000000e+00, !dbg !7393
  %337 = select i1 %336, i1 %_3.i.i519.i.i, i1 false, !dbg !7395
  %_0.i308.i.i = select i1 %337, float %_4.i.i442.v.i.i, float 0.000000e+00, !dbg !7395
  %_3.i178.i.i = fcmp ule float %_0.i308.i.i, %_86.i86976.i.i, !dbg !7396
  %_4.i301.i.i = select i1 %_3.i178.i.i, i32 %_88.i89568.i.i, i32 %_87.i88567.i.i, !dbg !7398
  %_0.i302.i.i = bitcast i32 %_4.i301.i.i to float, !dbg !7400
  %_0.i248.i.i = fsub float %_0.i308.i.i, %_86.i86976.i.i, !dbg !7402
  %_4.i218.i.i = fmul float %_0.i248.i.i, %_0.i302.i.i, !dbg !7404
  %_0.i219.i.i = fadd float %_86.i86976.i.i, %_4.i218.i.i, !dbg !7404
  %338 = tail call noundef float @llvm.fabs.f32(float %_0.i219.i.i), !dbg !7406
  %339 = fcmp uge float %338, 0x3BC79CA100000000, !dbg !7409
  %_0.i286.i.i = select i1 %339, float %_0.i219.i.i, float 0.000000e+00, !dbg !7411
  store float %_0.i286.i.i, ptr %261, align 4, !dbg !7412, !alias.scope !7346, !noalias !7349
  %_0.i233.i.i = fmul float %_0.i286.i.i, 0x3FC542A5A0000000, !dbg !7413
  %_3.i.i428.inv.i.i = fcmp ogt float %_0.i233.i.i, -1.260000e+02, !dbg !7416
  %_0.i.i435.i.i = select i1 %_3.i.i428.inv.i.i, float %_0.i233.i.i, float -1.260000e+02, !dbg !7416
  %_3.i.i511.inv.i.i = fcmp olt float %_0.i.i435.i.i, 1.270000e+02, !dbg !7420
  %_0.i.i518.i.i = select i1 %_3.i.i511.inv.i.i, float %_0.i.i435.i.i, float 1.270000e+02, !dbg !7420
  %340 = tail call noundef float @llvm.floor.f32(float %_0.i.i518.i.i), !dbg !7423
  %_0.i247.i.i = fsub float %_0.i.i518.i.i, %340, !dbg !7427
  %341 = tail call noundef float @llvm.fabs.f32(float %_85.i.us.us.le807.i.i), !dbg !7429
  %342 = tail call noundef float @llvm.fabs.f32(float %_87.i.us.us.i.i), !dbg !7433
  %_3.i.i494.i.i = fcmp ule float %341, %342, !dbg !7435
  %_6.i.i496.i.i = bitcast float %341 to i32, !dbg !7438
  %_8.i.i498.i.i = bitcast float %342 to i32, !dbg !7441
  %_4.i.i501.i.i = select i1 %_3.i.i494.i.i, i32 %_8.i.i498.i.i, i32 %_6.i.i496.i.i, !dbg !7443
  %_4.i392.i.i = select i1 %_3.i202.i.i, i32 %_6.i.i496.i.i, i32 %_4.i.i501.i.i, !dbg !7444
  %_0.i243.i.i = fmul float %341, 5.000000e-01, !dbg !7446
  %_0.i242.i.i = fmul float %342, 5.000000e-01, !dbg !7448
  %_0.i217.i.i = fadd float %_0.i242.i.i, %_0.i243.i.i, !dbg !7450
  %_6.i380.i.i = bitcast float %_0.i217.i.i to i32, !dbg !7452
  %_4.i385.i.i = select i1 %_3.i200.i.i, i32 %_4.i392.i.i, i32 %_6.i380.i.i, !dbg !7455
  %_0.i386.i.i = bitcast i32 %_4.i385.i.i to float, !dbg !7456
  %_3.i.i486.i.i = fcmp ule float %_0.i386.i.i, 0x3E45798EE0000000, !dbg !7458
  %_4.i.i492.i.i = select i1 %_3.i.i486.i.i, i32 841731191, i32 %_4.i385.i.i, !dbg !7461
  %_0.i.i493.i.i = bitcast i32 %_4.i.i492.i.i to float, !dbg !7463
  %_3.i.i.i184.i = fcmp ule float %_0.i.i493.i.i, 0x3810000000000000, !dbg !7465
  %_4.i.i.i.i = select i1 %_3.i.i.i184.i, i32 8388608, i32 %_4.i.i492.i.i, !dbg !7470
  %_5.i274.i.i = and i32 %_4.i.i.i.i, 8388607, !dbg !7472
  %_4.i275.i.i = or disjoint i32 %_5.i274.i.i, 1065353216, !dbg !7472
  %significand.i.i.i = bitcast i32 %_4.i275.i.i to float, !dbg !7474
  %_0.i244.i.i = fadd float %significand.i.i.i, -1.000000e+00, !dbg !7476
  %_0.i223.i.i = fmul float %_0.i244.i.i, 0x3F9B17A960000000, !dbg !7478
  %343 = fsub float 0x3FBF9A8440000000, %_0.i223.i.i, !dbg !7480
  %_0.i223.1.i.i = fmul float %_0.i244.i.i, %343, !dbg !7478
  %_0.i209.1.i.i = fadd float %_0.i223.1.i.i, 0xBFD1E3F400000000, !dbg !7480
  %_0.i223.2.i.i = fmul float %_0.i244.i.i, %_0.i209.1.i.i, !dbg !7478
  %_0.i209.2.i.i = fadd float %_0.i223.2.i.i, 0x3FDD544F20000000, !dbg !7480
  %_0.i223.3.i.i = fmul float %_0.i244.i.i, %_0.i209.2.i.i, !dbg !7478
  %_0.i209.3.i.i = fadd float %_0.i223.3.i.i, 0xBFE6FC2A60000000, !dbg !7480
  %_0.i223.4.i.i = fmul float %_0.i244.i.i, %_0.i209.3.i.i, !dbg !7478
  %_0.i209.4.i.i = fadd float %_0.i223.4.i.i, 0x3FF714B2A0000000, !dbg !7480
  %_9.i.i.i = lshr i32 %_4.i.i.i.i, 23, !dbg !7482
  %_8.i276.i.i = or disjoint i32 %_9.i.i.i, 1258291200, !dbg !7482
  %_7.i.i.i = bitcast i32 %_8.i276.i.i to float, !dbg !7483
  %exponent.i.i.i = fadd float %_7.i.i.i, 0xC160000FE0000000, !dbg !7485
  %_0.i222.i.i = fmul float %_0.i244.i.i, %_0.i209.4.i.i, !dbg !7486
  %_0.i208.i.i = fadd float %exponent.i.i.i, %_0.i222.i.i, !dbg !7488
  %_0.i241.i.i = fmul float %_0.i208.i.i, 0x4018151820000000, !dbg !7490
  %_3.i.i543.inv.i.i = fcmp olt float %_0.i241.i.i, 2.400000e+01, !dbg !7492
  %_0.i.i550.i.i = select i1 %_3.i.i543.inv.i.i, float %_0.i241.i.i, float 2.400000e+01, !dbg !7492
  %_3.i.i478.inv.i.i = fcmp ogt float %_0.i.i550.i.i, -1.600000e+02, !dbg !7495
  %_0.i.i485.i.i = select i1 %_3.i.i478.inv.i.i, float %_0.i.i550.i.i, float -1.600000e+02, !dbg !7495
  %_55.i9.i.i = load float, ptr %269, align 4, !dbg !7498, !alias.scope !7499, !noalias !7502, !noundef !12
  %_3.i198.i.i = fcmp ule float %_55.i9.i.i, 0.000000e+00, !dbg !7504
  %_3.i174.i.i = fcmp oge float %_0.i.i485.i.i, %threshold.i.i.i, !dbg !7506
  %_3.i172.i.i = fcmp oge float %_0.i.i485.i.i, %_0.i257.i.i, !dbg !7508
  %..i173.i.i = sext i1 %_3.i172.i.i to i32, !dbg !7510
  %_0.i406.i.i = sext i1 %_3.i174.i.i to i32, !dbg !7512
  %_0.i399.i.i = select i1 %_3.i198.i.i, i32 %_0.i406.i.i, i32 %..i173.i.i, !dbg !7512
  %_0.i410.i.i = xor i32 %..i173.i.i, -1, !dbg !7514
  %_3.i196.i.i = fcmp ogt float %_67.i11978.i.i, 0.000000e+00, !dbg !7516
  %_0.i405.i.i = select i1 %_3.i196.i.i, i32 %_0.i410.i.i, i32 0, !dbg !7518
  %_0.i404.i.i = select i1 %_3.i198.i.i, i32 0, i32 %_0.i405.i.i, !dbg !7520
  %_0.i398.i.i = or i32 %_0.i404.i.i, %_0.i399.i.i, !dbg !7522
  %_5.i375.i.i = and i32 %_0.i398.i.i, 1065353216, !dbg !7524
  %_0.i379.i.i = bitcast i32 %_5.i375.i.i to float, !dbg !7526
  %_0.i256.i.i = fadd float %_67.i11978.i.i, -1.000000e+00, !dbg !7528
  %344 = trunc nsw i32 %_0.i404.i.i to i1, !dbg !7530
  %_4.i373.v.i.i = select i1 %344, float %_0.i256.i.i, float %_67.i11978.i.i, !dbg !7530
  %345 = trunc nsw i32 %_0.i399.i.i to i1, !dbg !7532
  %_0.i367.i.i = select i1 %345, float %_71.i581582.i.i, float %_4.i373.v.i.i, !dbg !7532
  store float %_0.i367.i.i, ptr %270, align 4, !dbg !7534, !alias.scope !7499, !noalias !7502
  store i32 %_5.i375.i.i, ptr %269, align 4, !dbg !7535, !alias.scope !7499, !noalias !7502
  %_0.i254.i.i = fsub float %_0.i.i485.i.i, %threshold.i.i.i, !dbg !7536
  %_0.i240.i.i = fmul float %_0.i255.i.i, %_0.i254.i.i, !dbg !7538
  %_3.i.i469.inv.i.i = fcmp ogt float %_0.i240.i.i, %272, !dbg !7540
  %_4.i.i476.v.i.i = select i1 %_3.i.i469.inv.i.i, float %_0.i240.i.i, float %272, !dbg !7540
  %_3.i.i535.i.i = fcmp olt float %_4.i.i476.v.i.i, 0.000000e+00, !dbg !7543
  %346 = fcmp ule float %_0.i379.i.i, 0.000000e+00, !dbg !7546
  %347 = select i1 %346, i1 %_3.i.i535.i.i, i1 false, !dbg !7548
  %_0.i360.i.i = select i1 %347, float %_4.i.i476.v.i.i, float 0.000000e+00, !dbg !7548
  %_3.i192.i.i = fcmp ule float %_0.i360.i.i, %_86.i19980.i.i, !dbg !7549
  %_4.i354.i.i = select i1 %_3.i192.i.i, i32 %_88.i22585.i.i, i32 %_87.i21584.i.i, !dbg !7551
  %_0.i.i.i = bitcast i32 %_4.i354.i.i to float, !dbg !7553
  %_0.i253.i.i = fsub float %_0.i360.i.i, %_86.i19980.i.i, !dbg !7555
  %_4.i220.i.i = fmul float %_0.i253.i.i, %_0.i.i.i, !dbg !7557
  %_0.i221.i.i = fadd float %_86.i19980.i.i, %_4.i220.i.i, !dbg !7557
  %348 = tail call noundef float @llvm.fabs.f32(float %_0.i221.i.i), !dbg !7559
  %349 = fcmp uge float %348, 0x3BC79CA100000000, !dbg !7562
  %_0.i290.i.i = select i1 %349, float %_0.i221.i.i, float 0.000000e+00, !dbg !7564
  store float %_0.i290.i.i, ptr %273, align 4, !dbg !7565, !alias.scope !7499, !noalias !7502
  %_0.i239.i.i = fmul float %_0.i290.i.i, 0x3FC542A5A0000000, !dbg !7566
  %_3.i.i420.inv.i.i = fcmp ogt float %_0.i239.i.i, -1.260000e+02, !dbg !7569
  %_0.i.i427.i.i = select i1 %_3.i.i420.inv.i.i, float %_0.i239.i.i, float -1.260000e+02, !dbg !7569
  %_3.i.i503.inv.i.i = fcmp olt float %_0.i.i427.i.i, 1.270000e+02, !dbg !7573
  %_0.i.i510.i.i = select i1 %_3.i.i503.inv.i.i, float %_0.i.i427.i.i, float 1.270000e+02, !dbg !7573
  %350 = tail call noundef float @llvm.floor.f32(float %_0.i.i510.i.i), !dbg !7576
  %_0.i246.i.i = fsub float %_0.i.i510.i.i, %350, !dbg !7580
  %_0.i228.i.i = fmul float %_0.i246.i.i, 0x3F5E974FA0000000, !dbg !7582
  %_0.i213.i.i = fadd float %_0.i228.i.i, 0x3F82778560000000, !dbg !7584
  %_0.i228.1.i.i = fmul float %_0.i246.i.i, %_0.i213.i.i, !dbg !7582
  %_0.i213.1.i.i = fadd float %_0.i228.1.i.i, 0x3FAC91CE60000000, !dbg !7584
  %_0.i228.2.i.i = fmul float %_0.i246.i.i, %_0.i213.1.i.i, !dbg !7582
  %_0.i213.2.i.i = fadd float %_0.i228.2.i.i, 0x3FCEBDB560000000, !dbg !7584
  %_0.i228.3.i.i = fmul float %_0.i246.i.i, %_0.i213.2.i.i, !dbg !7582
  %_0.i213.3.i.i = fadd float %_0.i228.3.i.i, 0x3FE62E4BA0000000, !dbg !7584
  %_0.i231.i.i = fmul float %_0.i247.i.i, 0x3F5E974FA0000000, !dbg !7586
  %_0.i215.i.i = fadd float %_0.i231.i.i, 0x3F82778560000000, !dbg !7588
  %_0.i231.1.i.i = fmul float %_0.i247.i.i, %_0.i215.i.i, !dbg !7586
  %_0.i215.1.i.i = fadd float %_0.i231.1.i.i, 0x3FAC91CE60000000, !dbg !7588
  %_0.i231.2.i.i = fmul float %_0.i247.i.i, %_0.i215.1.i.i, !dbg !7586
  %_0.i215.2.i.i = fadd float %_0.i231.2.i.i, 0x3FCEBDB560000000, !dbg !7588
  %_0.i231.3.i.i = fmul float %_0.i247.i.i, %_0.i215.2.i.i, !dbg !7586
  %_0.i215.3.i.i = fadd float %_0.i231.3.i.i, 0x3FE62E4BA0000000, !dbg !7588
  %_0.i230.i.i = fmul float %_0.i247.i.i, %_0.i215.3.i.i, !dbg !7590
  %_0.i214.i.i = fadd float %_0.i230.i.i, 1.000000e+00, !dbg !7592
  %biased.i161.i.i = fadd float %340, 0x4160000FE0000000, !dbg !7594
  %_4.i162.i.i = bitcast float %biased.i161.i.i to i32, !dbg !7596
  %_3.i163.i.i = shl i32 %_4.i162.i.i, 23, !dbg !7598
  %_0.i164.i.i = bitcast i32 %_3.i163.i.i to float, !dbg !7599
  %_0.i229.i.i = fmul float %_0.i214.i.i, %_0.i164.i.i, !dbg !7601
  %_3.i165.i.i = fcmp une float %_0.i286.i.i, 0.000000e+00, !dbg !7603
  %_0.i394572.not.i.i = and i1 %_3.i176.i.i, %_3.i165.i.i, !dbg !7605
  %_0.i232.i.i = fmul float %_0.i269.i.i, %_0.i229.i.i, !dbg !7605
  %_4.i294.v.i.i = select i1 %_0.i394572.not.i.i, float %_0.i232.i.i, float %_0.i269.i.i, !dbg !7607
  %_0.i227.i.i = fmul float %_0.i246.i.i, %_0.i213.3.i.i, !dbg !7609
  %_0.i212.i.i = fadd float %_0.i227.i.i, 1.000000e+00, !dbg !7611
  %biased.i.i.i = fadd float %350, 0x4160000FE0000000, !dbg !7613
  %_4.i158.i.i = bitcast float %biased.i.i.i to i32, !dbg !7615
  %_3.i159.i.i = shl i32 %_4.i158.i.i, 23, !dbg !7617
  %_0.i160.i.i = bitcast i32 %_3.i159.i.i to float, !dbg !7618
  %_0.i226.i.i = fmul float %_0.i212.i.i, %_0.i160.i.i, !dbg !7620
  %_3.i166.i.i = fcmp une float %_0.i290.i.i, 0.000000e+00, !dbg !7622
  %_0.i397589.not.i.i = and i1 %_3.i190.i.i, %_3.i166.i.i, !dbg !7624
  %_0.i238.i.i = fmul float %_0.i267.i.i, %_0.i226.i.i, !dbg !7624
  %_4.i347.v.i.i = select i1 %_0.i397589.not.i.i, float %_0.i238.i.i, float %_0.i267.i.i, !dbg !7626
  store float %_4.i294.v.i.i, ptr %_123.i.i.i, align 4, !dbg !7628, !alias.scope !7631, !noalias !7201
  store float %_4.i347.v.i.i, ptr %_141.i.i.i, align 4, !dbg !7634, !alias.scope !7636, !noalias !7224
  %exitcond1495.not.i.i = icmp eq i64 %326, %_55.i, !dbg !7639
  br i1 %exitcond1495.not.i.i, label %_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i, label %bb42.i.i.i, !dbg !7642, !llvm.loop !7653

bb24.i.us.lr.ph.split.us.panic24.i.split.us.split.us_crit_edge.i83.i: ; preds = %bb24.i.us.lr.ph.split.us.us.us.us.i82.i, %bb24.i.us.lr.ph.split.us.i.i
  %.us-phi1063.i.i = phi i64 [ %_72.i.i.i, %bb24.i.us.lr.ph.split.us.i.i ], [ %_72.i.us.us.us.i71.i, %bb24.i.us.lr.ph.split.us.us.us.us.i82.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1063.i.i, i64 noundef %_58.1.i22.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c5ae5decb60097d7e1183cb43cae6b3f) #24, !dbg !7269, !noalias !7211
  unreachable, !dbg !7269

bb24.i.us.lr.ph.split.i81.i:                      ; preds = %bb63.i.split.us.us.us.us.i76.i, %bb24.i.us.lr.ph.us.i155.i, %bb24.i.us.lr.ph.i.i
  %.us-phi1045.i.i = phi i64 [ %_72.i.us.i149.i, %bb24.i.us.lr.ph.us.i155.i ], [ %_72.i.i.i, %bb24.i.us.lr.ph.i.i ], [ %_72.i.us.us.us.i71.i, %bb63.i.split.us.us.us.us.i76.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1045.i.i, i64 noundef %_60.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5833dd3f10f38ea96ec24c6054ff083c) #24, !dbg !7654, !noalias !7211
  unreachable, !dbg !7654

bb63.i.split.us.panic20.i.split.us_crit_edge.i78.i: ; preds = %bb63.i.split.us.us.i153.i, %bb63.i.split.us.i.i
  %.us-phi1039.i.i = phi i64 [ %_64.i.i.i, %bb63.i.split.us.i.i ], [ %_64.i.us.i146.i, %bb63.i.split.us.us.i153.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1039.i.i, i64 noundef %_60.1.i25.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_30bf8a301493a607abcc78dbf93977e0) #24, !dbg !7645, !noalias !7211
  unreachable, !dbg !7645

bb63.i.split.i75.i:                               ; preds = %bb60.i.us.us.us.i61.i, %bb63.i.us.i141.i, %bb63.i.i.i
  %.us-phi1017.i.i = phi i64 [ %_64.i.us.i146.i, %bb63.i.us.i141.i ], [ %_64.i.i.i, %bb63.i.i.i ], [ %_64.i.us.us.us.i68.i, %bb60.i.us.us.us.i61.i ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.us-phi1017.i.i, i64 noundef %_58.1.i22.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b53d553f9945f7702333f7af20204159) #24, !dbg !7655, !noalias !7211
  unreachable, !dbg !7655

_RINvMCsdOTRa1MFkeb_13gate_expanderINtB3_12PreparedGatefKb0_E11run_segmentKBS_EB3_.exit.i: ; preds = %bb28.i.us.us.lr.ph.us.us.us.i84.i, %bb24.i.us.lr.ph.split.us.us.i157.i, %bb28.i.us.us.lr.ph.i.i
  %_108.i.i110.i = trunc i64 %_55.i to i32, !dbg !7656
  %_107.i.i111.i = add i32 %base.i.i34.i, %_108.i.i110.i, !dbg !7657
  store i32 %_107.i.i111.i, ptr %_51.i27.i, align 4, !dbg !7659, !alias.scope !7137, !noalias !7156
  br label %bb7.i5, !dbg !7660

bb26.i:                                           ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %..i.i, i64 noundef range(i64 0, 2305843009213693952) %_13.1, i64 noundef range(i64 0, 2305843009213693952) %_13.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9c634077742a23e4340a846355ecc484) #24, !dbg !7661, !noalias !6134
  unreachable, !dbg !7661

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb0_E9run_blockB2_.exit: ; preds = %bb1.backedge.i.i
  call void @llvm.lifetime.end.p0(ptr nonnull %iter.i.i), !dbg !7662, !noalias !6850
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) %reports.sroa.0, i64 16, i1 false), !dbg !7663
  %reports.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16, !dbg !7663
  store i64 %reports.sroa.4.0, ptr %reports.sroa.4.0._0.sroa_idx, align 8, !dbg !7663
  %reports.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24, !dbg !7663
  %reports.sroa.7.0.reports.sroa.7.0.copyload = load i64, ptr %reports.sroa.7, align 8, !dbg !7663
  store i64 %reports.sroa.7.0.reports.sroa.7.0.copyload, ptr %reports.sroa.7.0._0.sroa_idx, align 8, !dbg !7663
  %reports.sroa.8.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32, !dbg !7663
  %reports.sroa.8.0.reports.sroa.8.0.copyload = load i64, ptr %reports.sroa.8, align 8, !dbg !7663
  store i64 %reports.sroa.8.0.reports.sroa.8.0.copyload, ptr %reports.sroa.8.0._0.sroa_idx, align 8, !dbg !7663
  call void @llvm.lifetime.end.p0(ptr nonnull %reports.sroa.0), !dbg !7664
  call void @llvm.lifetime.end.p0(ptr nonnull %reports.sroa.7), !dbg !7664
  call void @llvm.lifetime.end.p0(ptr nonnull %reports.sroa.8), !dbg !7664
  ret void, !dbg !7665
}
