define internal void @_RNvXs_CsdOTRa1MFkeb_13gate_expanderINtB4_12PreparedGatefKb1_ENtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7processB4_(ptr dead_on_unwind noalias noundef writable writeonly sret([40 x i8]) align 8 captures(none) dereferenceable(40) %_0, ptr noalias noundef align 8 dereferenceable(1232) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(88) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !8014 {
start:
  %pending.i = alloca [64 x i8], align 4
  %reports = alloca [320 x i8], align 8
  call void @llvm.lifetime.start.p0(ptr nonnull %reports), !dbg !8015
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %reports, i8 0, i64 320, i1 false)
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !8016
  %_12.0 = load ptr, ptr %0, align 8, !dbg !8016, !nonnull !12, !align !4661, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !8016
  %_12.1 = load i64, ptr %1, align 8, !dbg !8016, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %block, i64 80, !dbg !8018
  %_6 = load i64, ptr %2, align 8, !dbg !8018, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8019), !dbg !8022
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8023), !dbg !8022
  call void @llvm.lifetime.start.p0(ptr nonnull %pending.i), !dbg !8025, !noalias !8028
  store i32 0, ptr %pending.i, align 4, !noalias !8028
  %_7.sroa.5.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 4
  %_7.sroa.5134.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 8
  store i32 0, ptr %_7.sroa.5134.0.pending.sroa_idx.i, align 4, !noalias !8028
  %_7.sroa.6137.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 16
  store i32 0, ptr %_7.sroa.6137.0.pending.sroa_idx.i, align 4, !noalias !8028
  %_7.sroa.7140.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 24
  store i32 0, ptr %_7.sroa.7140.0.pending.sroa_idx.i, align 4, !noalias !8028
  %3 = getelementptr inbounds nuw i8, ptr %pending.i, i64 32
  store i32 0, ptr %3, align 4, !noalias !8028
  %_7.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 36
  %_7.sroa.5134.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 40
  store i32 0, ptr %_7.sroa.5134.0..sroa_idx.i, align 4, !noalias !8028
  %_7.sroa.6137.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 48
  store i32 0, ptr %_7.sroa.6137.0..sroa_idx.i, align 4, !noalias !8028
  %_7.sroa.7140.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 56
  store i32 0, ptr %_7.sroa.7140.0..sroa_idx.i, align 4, !noalias !8028
  %_106.idx.i = mul nuw nsw i64 %_12.1, 40, !dbg !8030
  %_106.i = getelementptr inbounds nuw i8, ptr %_12.0, i64 %_106.idx.i, !dbg !8030
  %_6.i.i8184.i = icmp eq i64 %_12.1, 0, !dbg !8041
  br i1 %_6.i.i8184.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, label %bb4.lr.ph.lr.ph.i, !dbg !8046

bb4.lr.ph.lr.ph.i:                                ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %reports, i64 16
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 92
  %_31.i = load i32, ptr %5, align 4, !alias.scope !8019, !noalias !8047
  %_30.i = zext i32 %_31.i to i64
  br label %bb4.lr.ph.i, !dbg !8046

bb4.lr.ph.i:                                      ; preds = %bb31.i, %bb4.lr.ph.lr.ph.i
  %.lcssa811 = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %6, %bb31.i ]
  %.promoted91.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %.promoted90.i, %bb31.i ]
  %last_order.sroa.3.0.ph88.i = phi i32 [ undef, %bb4.lr.ph.lr.ph.i ], [ %_127.0.i, %bb31.i ]
  %last_order.sroa.0.0.ph87.not.i = phi i1 [ true, %bb4.lr.ph.lr.ph.i ], [ false, %bb31.i ]
  %iter.sroa.0.0.ph86.i = phi ptr [ %_12.0, %bb4.lr.ph.lr.ph.i ], [ %_16.i.i.i, %bb31.i ]
  %iter.sroa.7.0.ph85.i = phi i64 [ 0, %bb4.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %bb4.i, !dbg !8046

bb4.i:                                            ; preds = %bb35.i, %bb4.lr.ph.i
  %6 = phi i64 [ %.lcssa811, %bb4.lr.ph.i ], [ %89, %bb35.i ]
  %.promoted90.i = phi i64 [ %.promoted91.i, %bb4.lr.ph.i ], [ %89, %bb35.i ]
  %iter.sroa.0.083.i = phi ptr [ %iter.sroa.0.0.ph86.i, %bb4.lr.ph.i ], [ %_16.i.i.i, %bb35.i ]
  %iter.sroa.7.082.i = phi i64 [ %iter.sroa.7.0.ph85.i, %bb4.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 40, !dbg !8048
  %_9.0.i.i = add i64 %iter.sroa.7.082.i, 1, !dbg !8050
  %7 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 32, !dbg !8051
  %_17.i = load i32, ptr %7, align 8, !dbg !8051, !range !1335, !alias.scope !8023, !noalias !8053, !noundef !12
  switch i32 %_17.i, label %default.unreachable [
    i32 1, label %bb9.i
    i32 2, label %bb7.i
    i32 3, label %bb35.i
  ], !dbg !8054

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i: ; preds = %bb46.us.3.i, %bb40.backedge.us.2.i
  %8 = load i32, ptr %3, align 4, !dbg !8055, !range !5716, !noalias !8028, !noundef !12
  %9 = trunc nuw i32 %8 to i1, !dbg !8060
  br i1 %9, label %bb46.us.1130.i, label %bb40.backedge.us.1131.i, !dbg !8060

bb46.us.1130.i:                                   ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 868
  %value.us.1118.i = load float, ptr %_7.sroa.5.0..sroa_idx.i, align 4, !dbg !8061, !noalias !8028, !noundef !12
  %_84.us.1119.i = load float, ptr %10, align 4, !dbg !8062, !alias.scope !8019, !noalias !8047, !noundef !12
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 872, !dbg !8065
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 876, !dbg !8066
  %13 = getelementptr inbounds nuw i8, ptr %self, i64 880, !dbg !8067
  %_148.us.1120.i = bitcast float %_84.us.1119.i to i32, !dbg !8068
  %_150.us.1121.i = bitcast float %value.us.1118.i to i32, !dbg !8076
  %_149.us.1122.i = icmp ne i32 %_148.us.1120.i, %_150.us.1121.i, !dbg !8079
  %14 = icmp eq i32 %_148.us.1120.i, -2147483648
  %or.cond.us.1123.i = or i1 %_149.us.1122.i, %14, !dbg !8079
  %15 = tail call float @llvm.fabs.f32(float %_84.us.1119.i)
  %_144.us.1124.i = fcmp ueq float %15, 0x7FF0000000000000
  %or.cond40.us.1125.i = or i1 %_144.us.1124.i, %or.cond.us.1123.i, !dbg !8079
  %_146.us.1126.i = fsub float %value.us.1118.i, %_84.us.1119.i, !dbg !8079
  %16 = fmul float %_146.us.1126.i, 1.562500e-02, !dbg !8079
  %ramp.sroa.0.0.us.1127.i = select i1 %or.cond40.us.1125.i, float %_84.us.1119.i, float %value.us.1118.i, !dbg !8079
  %ramp4.sroa.0.0.us.1128.i = select i1 %or.cond40.us.1125.i, float %16, float 0.000000e+00, !dbg !8079
  %ramp5.sroa.0.0.us.1129.i = select i1 %or.cond40.us.1125.i, float 6.400000e+01, float 0.000000e+00, !dbg !8079
  store float %ramp.sroa.0.0.us.1127.i, ptr %10, align 4, !dbg !8080, !alias.scope !8082, !noalias !8047
  store float %value.us.1118.i, ptr %11, align 4, !dbg !8085, !alias.scope !8087, !noalias !8047
  store float %ramp4.sroa.0.0.us.1128.i, ptr %12, align 4, !dbg !8090, !alias.scope !8092, !noalias !8047
  store float %ramp5.sroa.0.0.us.1129.i, ptr %13, align 4, !dbg !8095, !alias.scope !8097, !noalias !8047
  store i32 64, ptr %44, align 4, !dbg !8100, !alias.scope !8019, !noalias !8047
  br label %bb40.backedge.us.1131.i, !dbg !8101

bb40.backedge.us.1131.i:                          ; preds = %bb46.us.1130.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i
  %17 = load i32, ptr %_7.sroa.5134.0..sroa_idx.i, align 4, !dbg !8055, !range !5716, !noalias !8028, !noundef !12
  %18 = trunc nuw i32 %17 to i1, !dbg !8060
  br i1 %18, label %bb46.us.1.1.i, label %bb40.backedge.us.1.1.i, !dbg !8060

bb46.us.1.1.i:                                    ; preds = %bb40.backedge.us.1131.i
  %19 = getelementptr inbounds nuw i8, ptr %pending.i, i64 44, !dbg !8055
  %value.us.1.1.i = load float, ptr %19, align 4, !dbg !8061, !noalias !8028, !noundef !12
  %slot.us.1.1.i = getelementptr inbounds nuw i8, ptr %self, i64 884, !dbg !8102
  %_84.us.1.1.i = load float, ptr %slot.us.1.1.i, align 4, !dbg !8062, !alias.scope !8019, !noalias !8047, !noundef !12
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 888, !dbg !8065
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 892, !dbg !8066
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 896, !dbg !8067
  %_148.us.1.1.i = bitcast float %_84.us.1.1.i to i32, !dbg !8068
  %_150.us.1.1.i = bitcast float %value.us.1.1.i to i32, !dbg !8076
  %_149.us.1.1.i = icmp ne i32 %_148.us.1.1.i, %_150.us.1.1.i, !dbg !8079
  %23 = icmp eq i32 %_148.us.1.1.i, -2147483648
  %or.cond.us.1.1.i = or i1 %_149.us.1.1.i, %23, !dbg !8079
  %24 = tail call float @llvm.fabs.f32(float %_84.us.1.1.i)
  %_144.us.1.1.i = fcmp ueq float %24, 0x7FF0000000000000
  %or.cond40.us.1.1.i = or i1 %_144.us.1.1.i, %or.cond.us.1.1.i, !dbg !8079
  %_146.us.1.1.i = fsub float %value.us.1.1.i, %_84.us.1.1.i, !dbg !8079
  %25 = fmul float %_146.us.1.1.i, 1.562500e-02, !dbg !8079
  %ramp.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float %_84.us.1.1.i, float %value.us.1.1.i, !dbg !8079
  %ramp4.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float %25, float 0.000000e+00, !dbg !8079
  %ramp5.sroa.0.0.us.1.1.i = select i1 %or.cond40.us.1.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !8079
  store float %ramp.sroa.0.0.us.1.1.i, ptr %slot.us.1.1.i, align 4, !dbg !8080, !alias.scope !8082, !noalias !8047
  store float %value.us.1.1.i, ptr %20, align 4, !dbg !8085, !alias.scope !8087, !noalias !8047
  store float %ramp4.sroa.0.0.us.1.1.i, ptr %21, align 4, !dbg !8090, !alias.scope !8092, !noalias !8047
  store float %ramp5.sroa.0.0.us.1.1.i, ptr %22, align 4, !dbg !8095, !alias.scope !8097, !noalias !8047
  store i32 64, ptr %44, align 4, !dbg !8100, !alias.scope !8019, !noalias !8047
  br label %bb40.backedge.us.1.1.i, !dbg !8101

bb40.backedge.us.1.1.i:                           ; preds = %bb46.us.1.1.i, %bb40.backedge.us.1131.i
  %26 = load i32, ptr %_7.sroa.6137.0..sroa_idx.i, align 4, !dbg !8055, !range !5716, !noalias !8028, !noundef !12
  %27 = trunc nuw i32 %26 to i1, !dbg !8060
  br i1 %27, label %bb46.us.2.1.i, label %bb40.backedge.us.2.1.i, !dbg !8060

bb46.us.2.1.i:                                    ; preds = %bb40.backedge.us.1.1.i
  %28 = getelementptr inbounds nuw i8, ptr %pending.i, i64 52, !dbg !8055
  %value.us.2.1.i = load float, ptr %28, align 4, !dbg !8061, !noalias !8028, !noundef !12
  %slot.us.2.1.i = getelementptr inbounds nuw i8, ptr %self, i64 900, !dbg !8102
  %_84.us.2.1.i = load float, ptr %slot.us.2.1.i, align 4, !dbg !8062, !alias.scope !8019, !noalias !8047, !noundef !12
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 904, !dbg !8065
  %30 = getelementptr inbounds nuw i8, ptr %self, i64 908, !dbg !8066
  %31 = getelementptr inbounds nuw i8, ptr %self, i64 912, !dbg !8067
  %_148.us.2.1.i = bitcast float %_84.us.2.1.i to i32, !dbg !8068
  %_150.us.2.1.i = bitcast float %value.us.2.1.i to i32, !dbg !8076
  %_149.us.2.1.i = icmp ne i32 %_148.us.2.1.i, %_150.us.2.1.i, !dbg !8079
  %32 = icmp eq i32 %_148.us.2.1.i, -2147483648
  %or.cond.us.2.1.i = or i1 %_149.us.2.1.i, %32, !dbg !8079
  %33 = tail call float @llvm.fabs.f32(float %_84.us.2.1.i)
  %_144.us.2.1.i = fcmp ueq float %33, 0x7FF0000000000000
  %or.cond40.us.2.1.i = or i1 %_144.us.2.1.i, %or.cond.us.2.1.i, !dbg !8079
  %_146.us.2.1.i = fsub float %value.us.2.1.i, %_84.us.2.1.i, !dbg !8079
  %34 = fmul float %_146.us.2.1.i, 1.562500e-02, !dbg !8079
  %ramp.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float %_84.us.2.1.i, float %value.us.2.1.i, !dbg !8079
  %ramp4.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float %34, float 0.000000e+00, !dbg !8079
  %ramp5.sroa.0.0.us.2.1.i = select i1 %or.cond40.us.2.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !8079
  store float %ramp.sroa.0.0.us.2.1.i, ptr %slot.us.2.1.i, align 4, !dbg !8080, !alias.scope !8082, !noalias !8047
  store float %value.us.2.1.i, ptr %29, align 4, !dbg !8085, !alias.scope !8087, !noalias !8047
  store float %ramp4.sroa.0.0.us.2.1.i, ptr %30, align 4, !dbg !8090, !alias.scope !8092, !noalias !8047
  store float %ramp5.sroa.0.0.us.2.1.i, ptr %31, align 4, !dbg !8095, !alias.scope !8097, !noalias !8047
  store i32 64, ptr %44, align 4, !dbg !8100, !alias.scope !8019, !noalias !8047
  br label %bb40.backedge.us.2.1.i, !dbg !8101

bb40.backedge.us.2.1.i:                           ; preds = %bb46.us.2.1.i, %bb40.backedge.us.1.1.i
  %35 = load i32, ptr %_7.sroa.7140.0..sroa_idx.i, align 4, !dbg !8055, !range !5716, !noalias !8028, !noundef !12
  %36 = trunc nuw i32 %35 to i1, !dbg !8060
  br i1 %36, label %bb46.us.3.1.i, label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16apply_automationB2_.exit, !dbg !8060

bb46.us.3.1.i:                                    ; preds = %bb40.backedge.us.2.1.i
  %37 = getelementptr inbounds nuw i8, ptr %pending.i, i64 60, !dbg !8055
  %value.us.3.1.i = load float, ptr %37, align 4, !dbg !8061, !noalias !8028, !noundef !12
  %slot.us.3.1.i = getelementptr inbounds nuw i8, ptr %self, i64 916, !dbg !8102
  %_84.us.3.1.i = load float, ptr %slot.us.3.1.i, align 4, !dbg !8062, !alias.scope !8019, !noalias !8047, !noundef !12
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 920, !dbg !8065
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 924, !dbg !8066
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 928, !dbg !8067
  %_148.us.3.1.i = bitcast float %_84.us.3.1.i to i32, !dbg !8068
  %_150.us.3.1.i = bitcast float %value.us.3.1.i to i32, !dbg !8076
  %_149.us.3.1.i = icmp ne i32 %_148.us.3.1.i, %_150.us.3.1.i, !dbg !8079
  %41 = icmp eq i32 %_148.us.3.1.i, -2147483648
  %or.cond.us.3.1.i = or i1 %_149.us.3.1.i, %41, !dbg !8079
  %42 = tail call float @llvm.fabs.f32(float %_84.us.3.1.i)
  %_144.us.3.1.i = fcmp ueq float %42, 0x7FF0000000000000
  %or.cond40.us.3.1.i = or i1 %_144.us.3.1.i, %or.cond.us.3.1.i, !dbg !8079
  %_146.us.3.1.i = fsub float %value.us.3.1.i, %_84.us.3.1.i, !dbg !8079
  %43 = fmul float %_146.us.3.1.i, 1.562500e-02, !dbg !8079
  %ramp.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float %_84.us.3.1.i, float %value.us.3.1.i, !dbg !8079
  %ramp4.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float %43, float 0.000000e+00, !dbg !8079
  %ramp5.sroa.0.0.us.3.1.i = select i1 %or.cond40.us.3.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !8079
  store float %ramp.sroa.0.0.us.3.1.i, ptr %slot.us.3.1.i, align 4, !dbg !8080, !alias.scope !8082, !noalias !8047
  store float %value.us.3.1.i, ptr %38, align 4, !dbg !8085, !alias.scope !8087, !noalias !8047
  store float %ramp4.sroa.0.0.us.3.1.i, ptr %39, align 4, !dbg !8090, !alias.scope !8092, !noalias !8047
  store float %ramp5.sroa.0.0.us.3.1.i, ptr %40, align 4, !dbg !8095, !alias.scope !8097, !noalias !8047
  store i32 64, ptr %44, align 4, !dbg !8100, !alias.scope !8019, !noalias !8047
  br label %_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16apply_automationB2_.exit, !dbg !8101

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i.loopexit: ; preds = %bb35.i
  store i64 %89, ptr %4, align 1, !dbg !8103
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i: ; preds = %bb31.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i.loopexit, %start
  %44 = getelementptr inbounds nuw i8, ptr %self, i64 1220
  %45 = load i32, ptr %pending.i, align 4, !dbg !8055, !range !5716, !noalias !8028, !noundef !12
  %46 = trunc nuw i32 %45 to i1, !dbg !8060
  br i1 %46, label %bb46.us.i, label %bb40.backedge.us.i, !dbg !8060

bb46.us.i:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %47 = getelementptr inbounds nuw i8, ptr %self, i64 792
  %value.us.i = load float, ptr %_7.sroa.5.0.pending.sroa_idx.i, align 4, !dbg !8061, !noalias !8028, !noundef !12
  %_84.us.i = load float, ptr %47, align 4, !dbg !8062, !alias.scope !8019, !noalias !8047, !noundef !12
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 796, !dbg !8065
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 800, !dbg !8066
  %50 = getelementptr inbounds nuw i8, ptr %self, i64 804, !dbg !8067
  %_148.us.i = bitcast float %_84.us.i to i32, !dbg !8068
  %_150.us.i = bitcast float %value.us.i to i32, !dbg !8076
  %_149.us.i = icmp ne i32 %_148.us.i, %_150.us.i, !dbg !8079
  %51 = icmp eq i32 %_148.us.i, -2147483648
  %or.cond.us.i = or i1 %_149.us.i, %51, !dbg !8079
  %52 = tail call float @llvm.fabs.f32(float %_84.us.i)
  %_144.us.i = fcmp ueq float %52, 0x7FF0000000000000
  %or.cond40.us.i = or i1 %_144.us.i, %or.cond.us.i, !dbg !8079
  %_146.us.i = fsub float %value.us.i, %_84.us.i, !dbg !8079
  %53 = fmul float %_146.us.i, 1.562500e-02, !dbg !8079
  %ramp.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %_84.us.i, float %value.us.i, !dbg !8079
  %ramp4.sroa.0.0.us.i = select i1 %or.cond40.us.i, float %53, float 0.000000e+00, !dbg !8079
  %ramp5.sroa.0.0.us.i = select i1 %or.cond40.us.i, float 6.400000e+01, float 0.000000e+00, !dbg !8079
  store float %ramp.sroa.0.0.us.i, ptr %47, align 4, !dbg !8080, !alias.scope !8082, !noalias !8047
  store float %value.us.i, ptr %48, align 4, !dbg !8085, !alias.scope !8087, !noalias !8047
  store float %ramp4.sroa.0.0.us.i, ptr %49, align 4, !dbg !8090, !alias.scope !8092, !noalias !8047
  store float %ramp5.sroa.0.0.us.i, ptr %50, align 4, !dbg !8095, !alias.scope !8097, !noalias !8047
  store i32 64, ptr %44, align 4, !dbg !8100, !alias.scope !8019, !noalias !8047
  br label %bb40.backedge.us.i, !dbg !8101

bb40.backedge.us.i:                               ; preds = %bb46.us.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i
  %54 = load i32, ptr %_7.sroa.5134.0.pending.sroa_idx.i, align 4, !dbg !8055, !range !5716, !noalias !8028, !noundef !12
  %55 = trunc nuw i32 %54 to i1, !dbg !8060
  br i1 %55, label %bb46.us.1.i, label %bb40.backedge.us.1.i, !dbg !8060

bb46.us.1.i:                                      ; preds = %bb40.backedge.us.i
  %56 = getelementptr inbounds nuw i8, ptr %pending.i, i64 12, !dbg !8055
  %value.us.1.i = load float, ptr %56, align 4, !dbg !8061, !noalias !8028, !noundef !12
  %slot.us.1.i = getelementptr inbounds nuw i8, ptr %self, i64 808, !dbg !8102
  %_84.us.1.i = load float, ptr %slot.us.1.i, align 4, !dbg !8062, !alias.scope !8019, !noalias !8047, !noundef !12
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 812, !dbg !8065
  %58 = getelementptr inbounds nuw i8, ptr %self, i64 816, !dbg !8066
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 820, !dbg !8067
  %_148.us.1.i = bitcast float %_84.us.1.i to i32, !dbg !8068
  %_150.us.1.i = bitcast float %value.us.1.i to i32, !dbg !8076
  %_149.us.1.i = icmp ne i32 %_148.us.1.i, %_150.us.1.i, !dbg !8079
  %60 = icmp eq i32 %_148.us.1.i, -2147483648
  %or.cond.us.1.i = or i1 %_149.us.1.i, %60, !dbg !8079
  %61 = tail call float @llvm.fabs.f32(float %_84.us.1.i)
  %_144.us.1.i = fcmp ueq float %61, 0x7FF0000000000000
  %or.cond40.us.1.i = or i1 %_144.us.1.i, %or.cond.us.1.i, !dbg !8079
  %_146.us.1.i = fsub float %value.us.1.i, %_84.us.1.i, !dbg !8079
  %62 = fmul float %_146.us.1.i, 1.562500e-02, !dbg !8079
  %ramp.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %_84.us.1.i, float %value.us.1.i, !dbg !8079
  %ramp4.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float %62, float 0.000000e+00, !dbg !8079
  %ramp5.sroa.0.0.us.1.i = select i1 %or.cond40.us.1.i, float 6.400000e+01, float 0.000000e+00, !dbg !8079
  store float %ramp.sroa.0.0.us.1.i, ptr %slot.us.1.i, align 4, !dbg !8080, !alias.scope !8082, !noalias !8047
  store float %value.us.1.i, ptr %57, align 4, !dbg !8085, !alias.scope !8087, !noalias !8047
  store float %ramp4.sroa.0.0.us.1.i, ptr %58, align 4, !dbg !8090, !alias.scope !8092, !noalias !8047
  store float %ramp5.sroa.0.0.us.1.i, ptr %59, align 4, !dbg !8095, !alias.scope !8097, !noalias !8047
  store i32 64, ptr %44, align 4, !dbg !8100, !alias.scope !8019, !noalias !8047
  br label %bb40.backedge.us.1.i, !dbg !8101

bb40.backedge.us.1.i:                             ; preds = %bb46.us.1.i, %bb40.backedge.us.i
  %63 = load i32, ptr %_7.sroa.6137.0.pending.sroa_idx.i, align 4, !dbg !8055, !range !5716, !noalias !8028, !noundef !12
  %64 = trunc nuw i32 %63 to i1, !dbg !8060
  br i1 %64, label %bb46.us.2.i, label %bb40.backedge.us.2.i, !dbg !8060

bb46.us.2.i:                                      ; preds = %bb40.backedge.us.1.i
  %65 = getelementptr inbounds nuw i8, ptr %pending.i, i64 20, !dbg !8055
  %value.us.2.i = load float, ptr %65, align 4, !dbg !8061, !noalias !8028, !noundef !12
  %slot.us.2.i = getelementptr inbounds nuw i8, ptr %self, i64 824, !dbg !8102
  %_84.us.2.i = load float, ptr %slot.us.2.i, align 4, !dbg !8062, !alias.scope !8019, !noalias !8047, !noundef !12
  %66 = getelementptr inbounds nuw i8, ptr %self, i64 828, !dbg !8065
  %67 = getelementptr inbounds nuw i8, ptr %self, i64 832, !dbg !8066
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 836, !dbg !8067
  %_148.us.2.i = bitcast float %_84.us.2.i to i32, !dbg !8068
  %_150.us.2.i = bitcast float %value.us.2.i to i32, !dbg !8076
  %_149.us.2.i = icmp ne i32 %_148.us.2.i, %_150.us.2.i, !dbg !8079
  %69 = icmp eq i32 %_148.us.2.i, -2147483648
  %or.cond.us.2.i = or i1 %_149.us.2.i, %69, !dbg !8079
  %70 = tail call float @llvm.fabs.f32(float %_84.us.2.i)
  %_144.us.2.i = fcmp ueq float %70, 0x7FF0000000000000
  %or.cond40.us.2.i = or i1 %_144.us.2.i, %or.cond.us.2.i, !dbg !8079
  %_146.us.2.i = fsub float %value.us.2.i, %_84.us.2.i, !dbg !8079
  %71 = fmul float %_146.us.2.i, 1.562500e-02, !dbg !8079
  %ramp.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %_84.us.2.i, float %value.us.2.i, !dbg !8079
  %ramp4.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float %71, float 0.000000e+00, !dbg !8079
  %ramp5.sroa.0.0.us.2.i = select i1 %or.cond40.us.2.i, float 6.400000e+01, float 0.000000e+00, !dbg !8079
  store float %ramp.sroa.0.0.us.2.i, ptr %slot.us.2.i, align 4, !dbg !8080, !alias.scope !8082, !noalias !8047
  store float %value.us.2.i, ptr %66, align 4, !dbg !8085, !alias.scope !8087, !noalias !8047
  store float %ramp4.sroa.0.0.us.2.i, ptr %67, align 4, !dbg !8090, !alias.scope !8092, !noalias !8047
  store float %ramp5.sroa.0.0.us.2.i, ptr %68, align 4, !dbg !8095, !alias.scope !8097, !noalias !8047
  store i32 64, ptr %44, align 4, !dbg !8100, !alias.scope !8019, !noalias !8047
  br label %bb40.backedge.us.2.i, !dbg !8101

bb40.backedge.us.2.i:                             ; preds = %bb46.us.2.i, %bb40.backedge.us.1.i
  %72 = load i32, ptr %_7.sroa.7140.0.pending.sroa_idx.i, align 4, !dbg !8055, !range !5716, !noalias !8028, !noundef !12
  %73 = trunc nuw i32 %72 to i1, !dbg !8060
  br i1 %73, label %bb46.us.3.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i, !dbg !8060

bb46.us.3.i:                                      ; preds = %bb40.backedge.us.2.i
  %74 = getelementptr inbounds nuw i8, ptr %pending.i, i64 28, !dbg !8055
  %value.us.3.i = load float, ptr %74, align 4, !dbg !8061, !noalias !8028, !noundef !12
  %slot.us.3.i = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !8102
  %_84.us.3.i = load float, ptr %slot.us.3.i, align 4, !dbg !8062, !alias.scope !8019, !noalias !8047, !noundef !12
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 844, !dbg !8065
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 848, !dbg !8066
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 852, !dbg !8067
  %_148.us.3.i = bitcast float %_84.us.3.i to i32, !dbg !8068
  %_150.us.3.i = bitcast float %value.us.3.i to i32, !dbg !8076
  %_149.us.3.i = icmp ne i32 %_148.us.3.i, %_150.us.3.i, !dbg !8079
  %78 = icmp eq i32 %_148.us.3.i, -2147483648
  %or.cond.us.3.i = or i1 %_149.us.3.i, %78, !dbg !8079
  %79 = tail call float @llvm.fabs.f32(float %_84.us.3.i)
  %_144.us.3.i = fcmp ueq float %79, 0x7FF0000000000000
  %or.cond40.us.3.i = or i1 %_144.us.3.i, %or.cond.us.3.i, !dbg !8079
  %_146.us.3.i = fsub float %value.us.3.i, %_84.us.3.i, !dbg !8079
  %80 = fmul float %_146.us.3.i, 1.562500e-02, !dbg !8079
  %ramp.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %_84.us.3.i, float %value.us.3.i, !dbg !8079
  %ramp4.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float %80, float 0.000000e+00, !dbg !8079
  %ramp5.sroa.0.0.us.3.i = select i1 %or.cond40.us.3.i, float 6.400000e+01, float 0.000000e+00, !dbg !8079
  store float %ramp.sroa.0.0.us.3.i, ptr %slot.us.3.i, align 4, !dbg !8080, !alias.scope !8082, !noalias !8047
  store float %value.us.3.i, ptr %75, align 4, !dbg !8085, !alias.scope !8087, !noalias !8047
  store float %ramp4.sroa.0.0.us.3.i, ptr %76, align 4, !dbg !8090, !alias.scope !8092, !noalias !8047
  store float %ramp5.sroa.0.0.us.3.i, ptr %77, align 4, !dbg !8095, !alias.scope !8097, !noalias !8047
  store i32 64, ptr %44, align 4, !dbg !8100, !alias.scope !8019, !noalias !8047
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.1.i, !dbg !8101

default.unreachable:                              ; preds = %bb4.i
  unreachable

bb7.i:                                            ; preds = %bb4.i
  br label %bb9.i, !dbg !8104

bb9.i:                                            ; preds = %bb4.i, %bb7.i
  %channel.sroa.0.0.sroa.phi28.i = phi ptr [ %3, %bb7.i ], [ %pending.i, %bb4.i ], !dbg !8103
  %channel.sroa.0.0.i = phi i32 [ 1, %bb7.i ], [ 0, %bb4.i ], !dbg !8103
  %81 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 16, !dbg !8105
  %_21.i = load i32, ptr %81, align 8, !dbg !8105, !alias.scope !8023, !noalias !8053, !noundef !12
  %parameter_index.i = zext i32 %_21.i to i64, !dbg !8105
  %_121.1.i = icmp slt i32 %_21.i, 0, !dbg !8107
  br i1 %_121.1.i, label %bb35.i, label %bb59.i, !dbg !8113, !prof !180

bb59.i:                                           ; preds = %bb9.i
  %_121.0.i = shl nuw i32 %_21.i, 1, !dbg !8107
  %_127.0.i = or disjoint i32 %_121.0.i, %channel.sroa.0.0.i, !dbg !8117
  %_29.i = icmp ult i64 %iter.sroa.7.082.i, %_30.i, !dbg !8125
  %_32.i = icmp samesign ult i32 %_21.i, 4
  %or.cond16.i = and i1 %_29.i, %_32.i, !dbg !8125
  br i1 %or.cond16.i, label %bb11.i, label %bb35.i, !dbg !8125

bb11.i:                                           ; preds = %bb59.i
  %82 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 28, !dbg !8127
  %_130.i = load i32, ptr %82, align 4, !dbg !8127, !range !6171, !alias.scope !8023, !noalias !8053, !noundef !12
  %83 = icmp eq i32 %_130.i, 1, !dbg !8130
  br i1 %83, label %bb12.i, label %bb35.i, !dbg !8130

bb12.i:                                           ; preds = %bb11.i
  %_34.i = load i64, ptr %iter.sroa.0.083.i, align 8, !dbg !8131, !alias.scope !8023, !noalias !8053, !noundef !12
  %_33.i = icmp eq i64 %_34.i, %_6, !dbg !8131
  br i1 %_33.i, label %bb13.i, label %bb35.i, !dbg !8131

bb13.i:                                           ; preds = %bb12.i
  %84 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 8, !dbg !8132
  %_36.i = load i64, ptr %84, align 8, !dbg !8132, !alias.scope !8023, !noalias !8053, !noundef !12
  %_35.i = icmp eq i64 %_36.i, %_6, !dbg !8132
  br i1 %_35.i, label %bb14.i, label %bb35.i, !dbg !8132

bb14.i:                                           ; preds = %bb13.i
  %85 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 20, !dbg !8133
  %_39.i = load float, ptr %85, align 4, !dbg !8133, !alias.scope !8023, !noalias !8053, !noundef !12
  %_38.i = bitcast float %_39.i to i32, !dbg !8134
  %86 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.083.i, i64 24, !dbg !8136
  %_4139.i = load i32, ptr %86, align 8, !dbg !8136, !alias.scope !8023, !noalias !8053, !noundef !12
  %_37.i = icmp eq i32 %_4139.i, %_38.i, !dbg !8133
  br i1 %_37.i, label %bb16.i, label %bb35.i, !dbg !8133

bb16.i:                                           ; preds = %bb14.i
  %_43.i = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_d0058eaee52f1ba8b2e5577cef0c3c7f, i64 %parameter_index.i, !dbg !8137
; call effect_runtime::params::parameter_value_valid
  %_42.i = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_43.i, float noundef %_39.i) #23, !dbg !8138, !noalias !8028
  %_45.i = icmp ugt i32 %_127.0.i, %last_order.sroa.3.0.ph88.i
  %or.cond17.not.not102.i = select i1 %last_order.sroa.0.0.ph87.not.i, i1 true, i1 %_45.i, !dbg !8138
  %or.cond41.not.i = select i1 %_42.i, i1 %or.cond17.not.not102.i, i1 false, !dbg !8138
  br i1 %or.cond41.not.i, label %bb29.i, label %bb35.i, !dbg !8138

bb29.i:                                           ; preds = %bb16.i
  %_47.i = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i, !dbg !8139
  %87 = load i32, ptr %_47.i, align 4, !dbg !8140, !range !5716, !noalias !8028, !noundef !12
  %_133.not.i = icmp eq i32 %87, 0, !dbg !8147
  br i1 %_133.not.i, label %bb31.i, label %bb35.i, !dbg !8148

bb31.i:                                           ; preds = %bb29.i
  %_47.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %channel.sroa.0.0.sroa.phi28.i, i64 %parameter_index.i
  store i64 %6, ptr %4, align 1, !dbg !8103
  %_135.i = fcmp oeq float %_39.i, 0.000000e+00, !dbg !8150
  %_55.sroa.0.0.i = select i1 %_135.i, float 0.000000e+00, float %_39.i, !dbg !8150
  store i32 1, ptr %_47.i.le, align 4, !dbg !8153, !noalias !8028
  %88 = getelementptr inbounds nuw i8, ptr %_47.i.le, i64 4, !dbg !8153
  store float %_55.sroa.0.0.i, ptr %88, align 4, !dbg !8153, !noalias !8028
  %_6.i.i81.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !8041
  br i1 %_6.i.i81.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i, label %bb4.lr.ph.i, !dbg !8046

bb35.i:                                           ; preds = %bb4.i, %bb29.i, %bb16.i, %bb14.i, %bb13.i, %bb12.i, %bb11.i, %bb59.i, %bb9.i
  %89 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted90.i, i64 1), !dbg !8154
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_106.i, !dbg !8041
  br i1 %_6.i.i.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCsdOTRa1MFkeb_13gate_expander.exit.us.preheader.i.loopexit, label %bb4.i, !dbg !8046

_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E16apply_automationB2_.exit: ; preds = %bb40.backedge.us.2.1.i, %bb46.us.3.1.i
  call void @llvm.lifetime.end.p0(ptr nonnull %pending.i), !dbg !8157, !noalias !8028
  %_14.0 = load ptr, ptr %block, align 8, !dbg !8158, !nonnull !12, !align !3484, !noundef !12
  %90 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !8158
  %_14.1 = load i64, ptr %90, align 8, !dbg !8158, !noundef !12
  %91 = getelementptr inbounds nuw i8, ptr %block, i64 48, !dbg !8161
  %92 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !8163
  %_13.0 = load ptr, ptr %92, align 8, !dbg !8163, !nonnull !12, !align !3484, !noundef !12
  %93 = getelementptr inbounds nuw i8, ptr %block, i64 24, !dbg !8163
  %_13.1 = load i64, ptr %93, align 8, !dbg !8163, !noundef !12
; call <gate_expander::PreparedGate<f32, true>>::run_block
  call fastcc void @_RNvMCsdOTRa1MFkeb_13gate_expanderINtB2_12PreparedGatefKb1_E9run_blockB2_(ptr noalias noundef align 8 dereferenceable(1232) %self, ptr noalias noundef nonnull align 4 %_14.0, i64 noundef %_14.1, ptr noalias noundef nonnull align 4 %_13.0, i64 noundef %_13.1, ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %91, i64 noundef %_14.1, ptr noalias noundef align 8 dereferenceable(320) %reports) #23, !dbg !8165
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(40) %_0, ptr noundef nonnull align 8 dereferenceable(40) %reports, i64 40, i1 false), !dbg !8166
  call void @llvm.lifetime.end.p0(ptr nonnull %reports), !dbg !8167
  ret void, !dbg !8168
}
