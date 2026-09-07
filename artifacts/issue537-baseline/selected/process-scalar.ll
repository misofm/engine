define void @_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process(ptr dead_on_unwind noalias noundef writable writeonly sret([40 x i8]) align 8 captures(none) dereferenceable(40) %_0, ptr noalias noundef align 8 dereferenceable(872) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(88) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !3624 {
start:
  %pending.i = alloca [160 x i8], align 4
  %_22.i579.i = alloca [24 x i8], align 4
  %_20.i580.i = alloca [24 x i8], align 4
  %_22.i121.i = alloca [24 x i8], align 4
  %_20.i122.i = alloca [24 x i8], align 4
  %filter_far.i14.i.i = alloca [16 x i8], align 4
  %filter_near.i15.i.i = alloca [16 x i8], align 4
  %filter_far.i.i5.i = alloca [16 x i8], align 4
  %filter_near.i.i6.i = alloca [16 x i8], align 4
  %_22.i8.i = alloca [24 x i8], align 4
  %_20.i9.i = alloca [24 x i8], align 4
  %_22.i.i = alloca [24 x i8], align 4
  %_20.i.i = alloca [24 x i8], align 4
  %_5 = getelementptr inbounds nuw i8, ptr %self, i64 104, !dbg !3625
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !3627
  %_18.0 = load ptr, ptr %0, align 8, !dbg !3627, !nonnull !11, !align !2513, !noundef !11
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !3627
  %_18.1 = load i64, ptr %1, align 8, !dbg !3627, !noundef !11
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 92, !dbg !3628
  %_6 = load i32, ptr %2, align 4, !dbg !3628, !noundef !11
  %3 = getelementptr inbounds nuw i8, ptr %block, i64 80, !dbg !3629
  %_7 = load i64, ptr %3, align 8, !dbg !3629, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3630), !dbg !3633
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3634), !dbg !3633
  call void @llvm.lifetime.start.p0(ptr nonnull %pending.i), !dbg !3636, !noalias !3639
  store i32 0, ptr %pending.i, align 4, !noalias !3639
  %_8.sroa.5177.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 8
  store i32 0, ptr %_8.sroa.5177.0.pending.sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.6180.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 16
  store i32 0, ptr %_8.sroa.6180.0.pending.sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.7183.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 24
  store i32 0, ptr %_8.sroa.7183.0.pending.sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.8186.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 32
  store i32 0, ptr %_8.sroa.8186.0.pending.sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.9189.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 40
  store i32 0, ptr %_8.sroa.9189.0.pending.sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.10192.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 48
  store i32 0, ptr %_8.sroa.10192.0.pending.sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.11195.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 56
  store i32 0, ptr %_8.sroa.11195.0.pending.sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.12198.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 64
  store i32 0, ptr %_8.sroa.12198.0.pending.sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.13201.0.pending.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 72
  store i32 0, ptr %_8.sroa.13201.0.pending.sroa_idx.i, align 4, !noalias !3639
  %4 = getelementptr inbounds nuw i8, ptr %pending.i, i64 80
  store i32 0, ptr %4, align 4, !noalias !3639
  %_8.sroa.5177.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 88
  store i32 0, ptr %_8.sroa.5177.0..sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.6180.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 96
  store i32 0, ptr %_8.sroa.6180.0..sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.7183.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 104
  store i32 0, ptr %_8.sroa.7183.0..sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.8186.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 112
  store i32 0, ptr %_8.sroa.8186.0..sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.9189.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 120
  store i32 0, ptr %_8.sroa.9189.0..sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.10192.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 128
  store i32 0, ptr %_8.sroa.10192.0..sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.11195.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 136
  store i32 0, ptr %_8.sroa.11195.0..sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.12198.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 144
  store i32 0, ptr %_8.sroa.12198.0..sroa_idx.i, align 4, !noalias !3639
  %_8.sroa.13201.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 152
  store i32 0, ptr %_8.sroa.13201.0..sroa_idx.i, align 4, !noalias !3639
  %_91.idx.i = mul nuw nsw i64 %_18.1, 40, !dbg !3641
  %_91.i = getelementptr inbounds nuw i8, ptr %_18.0, i64 %_91.idx.i, !dbg !3641
  %_6.i.i121124.i = icmp eq i64 %_18.1, 0, !dbg !3653
  br i1 %_6.i.i121124.i, label %bb36.preheader.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i, !dbg !3663

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i: ; preds = %start
  %_33.i = zext i32 %_6 to i64
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i, !dbg !3663

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i: ; preds = %bb31.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i
  %report.sroa.7.0 = phi i64 [ 0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i ], [ %report.sroa.7.1, %bb31.i ], !dbg !3664
  %.promoted131.i = phi i64 [ 0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i ], [ %.promoted130.i, %bb31.i ]
  %previous.sroa.0.0.ph128.not.i = phi i1 [ true, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i ], [ false, %bb31.i ]
  %previous.sroa.5.0.ph127.i = phi i32 [ undef, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i ], [ %_112.0.i, %bb31.i ]
  %iter.sroa.0.0.ph126.i = phi ptr [ %_18.0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i ], [ %_16.i.i.i6428, %bb31.i ]
  %iter.sroa.7.0.ph125.i = phi i64 [ 0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i, !dbg !3663

bb36.preheader.i:                                 ; preds = %bb31.i, %bb35.i, %start
  %report.sroa.7.2 = phi i64 [ 0, %start ], [ %113, %bb35.i ], [ %report.sroa.7.1, %bb31.i ], !dbg !3668
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.us.preheader.i, !dbg !3669

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i: ; preds = %bb35.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i
  %report.sroa.7.1 = phi i64 [ %report.sroa.7.0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i ], [ %113, %bb35.i ], !dbg !3668
  %.promoted130.i = phi i64 [ %.promoted131.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i ], [ %113, %bb35.i ]
  %iter.sroa.0.0123.i = phi ptr [ %iter.sroa.0.0.ph126.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i ], [ %_16.i.i.i6428, %bb35.i ]
  %iter.sroa.7.0122.i = phi i64 [ %iter.sroa.7.0.ph125.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i6428 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 40, !dbg !3677
  %_9.0.i.i = add i64 %iter.sroa.7.0122.i, 1, !dbg !3680
  %5 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 32, !dbg !3683
  %_19.i = load i32, ptr %5, align 8, !dbg !3683, !range !2567, !alias.scope !3634, !noalias !3685, !noundef !11
  switch i32 %_19.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i.unreachabledefault [
    i32 1, label %bb9.i6430
    i32 2, label %bb7.i6429
    i32 3, label %bb35.i
  ], !dbg !3686

bb36.loopexit.sink.split.i:                       ; preds = %bb76.us.9.i, %bb75.us.9.i
  %.sink257.i = phi float [ %97, %bb76.us.9.i ], [ 0.000000e+00, %bb75.us.9.i ]
  %.sink.i = phi i32 [ 64, %bb76.us.9.i ], [ 0, %bb75.us.9.i ]
  store float %.sink257.i, ptr %96, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %6 = getelementptr inbounds nuw i8, ptr %7, i64 244, !dbg !3687
  store i32 %.sink.i, ptr %6, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb36.loopexit.i, !dbg !3695

bb36.loopexit.i:                                  ; preds = %bb48.us.8.i, %bb36.loopexit.sink.split.i
  %_6.i.i58.i = icmp eq i64 %iter2.sroa.0.0.add.i, 160, !dbg !3695
  br i1 %_6.i.i58.i, label %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E16apply_automationB5_.exit, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.us.preheader.i, !dbg !3669

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.us.preheader.i: ; preds = %bb36.loopexit.i, %bb36.preheader.i
  %iter2.sroa.0.0.idx143.i = phi i64 [ 0, %bb36.preheader.i ], [ %iter2.sroa.0.0.add.i, %bb36.loopexit.i ]
  %iter2.sroa.7.0142.i = phi i64 [ 0, %bb36.preheader.i ], [ %_9.0.i62.i, %bb36.loopexit.i ]
  %iter2.sroa.0.0.ptr.i = getelementptr inbounds nuw i8, ptr %pending.i, i64 %iter2.sroa.0.0.idx143.i, !dbg !3695
  %iter2.sroa.0.0.add.i = add nuw nsw i64 %iter2.sroa.0.0.idx143.i, 80, !dbg !3698
  %_9.0.i62.i = add nuw nsw i64 %iter2.sroa.7.0142.i, 1, !dbg !3701
  %7 = getelementptr inbounds nuw %"Side<f32, 1>", ptr %_5, i64 %iter2.sroa.7.0142.i
  %8 = getelementptr inbounds nuw i8, ptr %7, i64 88
  %9 = load i32, ptr %iter2.sroa.0.0.ptr.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %10 = trunc nuw i32 %9 to i1, !dbg !3705
  br i1 %10, label %bb47.us.i, label %bb48.us.i, !dbg !3705

bb47.us.i:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.us.preheader.i
  %11 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 4, !dbg !3704
  %value.us.i = load float, ptr %11, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %12 = getelementptr inbounds nuw i8, ptr %7, i64 92, !dbg !3707
  store float %value.us.i, ptr %12, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.i = load float, ptr %8, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.i = bitcast float %_141.us.i to i32, !dbg !3709
  %_146.us.i = bitcast float %value.us.i to i32, !dbg !3714
  %_145.us.i = icmp ne i32 %_144.us.i, %_146.us.i, !dbg !3717
  %13 = icmp eq i32 %_144.us.i, -2147483648
  %or.cond.us.i = or i1 %_145.us.i, %13, !dbg !3717
  %14 = tail call float @llvm.fabs.f32(float %_141.us.i)
  %_140.us.i = fcmp ueq float %14, 0x7FF0000000000000
  %or.cond51.us.i = or i1 %_140.us.i, %or.cond.us.i, !dbg !3717
  %15 = getelementptr inbounds nuw i8, ptr %7, i64 96, !dbg !3687
  br i1 %or.cond51.us.i, label %bb76.us.i, label %bb75.us.i, !dbg !3717

bb75.us.i:                                        ; preds = %bb47.us.i
  store float %value.us.i, ptr %8, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.sink.split.i, !dbg !3719

bb76.us.i:                                        ; preds = %bb47.us.i
  %_142.us.i = fsub float %value.us.i, %_141.us.i, !dbg !3720
  %16 = fmul float %_142.us.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.sink.split.i, !dbg !3719

bb48.us.sink.split.i:                             ; preds = %bb76.us.i, %bb75.us.i
  %.sink231.i = phi float [ %16, %bb76.us.i ], [ 0.000000e+00, %bb75.us.i ]
  %.sink229.i = phi i32 [ 64, %bb76.us.i ], [ 0, %bb75.us.i ]
  store float %.sink231.i, ptr %15, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %17 = getelementptr inbounds nuw i8, ptr %7, i64 100, !dbg !3687
  store i32 %.sink229.i, ptr %17, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.i, !dbg !3722

bb48.us.i:                                        ; preds = %bb48.us.sink.split.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.us.preheader.i
  %iter3.sroa.0.0.ptr138.us.1.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 8, !dbg !3722
  %18 = load i32, ptr %iter3.sroa.0.0.ptr138.us.1.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %19 = trunc nuw i32 %18 to i1, !dbg !3705
  br i1 %19, label %bb47.us.1.i, label %bb48.us.1.i, !dbg !3705

bb47.us.1.i:                                      ; preds = %bb48.us.i
  %20 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 12, !dbg !3704
  %value.us.1.i = load float, ptr %20, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.1.i = getelementptr inbounds nuw i8, ptr %7, i64 104, !dbg !3731
  %21 = getelementptr inbounds nuw i8, ptr %7, i64 108, !dbg !3707
  store float %value.us.1.i, ptr %21, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.1.i = load float, ptr %_82.us.1.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.1.i = bitcast float %_141.us.1.i to i32, !dbg !3709
  %_146.us.1.i = bitcast float %value.us.1.i to i32, !dbg !3714
  %_145.us.1.i = icmp ne i32 %_144.us.1.i, %_146.us.1.i, !dbg !3717
  %22 = icmp eq i32 %_144.us.1.i, -2147483648
  %or.cond.us.1.i = or i1 %_145.us.1.i, %22, !dbg !3717
  %23 = tail call float @llvm.fabs.f32(float %_141.us.1.i)
  %_140.us.1.i = fcmp ueq float %23, 0x7FF0000000000000
  %or.cond51.us.1.i = or i1 %_140.us.1.i, %or.cond.us.1.i, !dbg !3717
  %24 = getelementptr inbounds nuw i8, ptr %7, i64 112, !dbg !3687
  br i1 %or.cond51.us.1.i, label %bb76.us.1.i, label %bb75.us.1.i, !dbg !3717

bb75.us.1.i:                                      ; preds = %bb47.us.1.i
  store float %value.us.1.i, ptr %_82.us.1.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.1.sink.split.i, !dbg !3719

bb76.us.1.i:                                      ; preds = %bb47.us.1.i
  %_142.us.1.i = fsub float %value.us.1.i, %_141.us.1.i, !dbg !3720
  %25 = fmul float %_142.us.1.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.1.sink.split.i, !dbg !3719

bb48.us.1.sink.split.i:                           ; preds = %bb76.us.1.i, %bb75.us.1.i
  %.sink234.i = phi float [ %25, %bb76.us.1.i ], [ 0.000000e+00, %bb75.us.1.i ]
  %.sink232.i = phi i32 [ 64, %bb76.us.1.i ], [ 0, %bb75.us.1.i ]
  store float %.sink234.i, ptr %24, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %26 = getelementptr inbounds nuw i8, ptr %7, i64 116, !dbg !3687
  store i32 %.sink232.i, ptr %26, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.1.i, !dbg !3722

bb48.us.1.i:                                      ; preds = %bb48.us.1.sink.split.i, %bb48.us.i
  %iter3.sroa.0.0.ptr138.us.2.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 16, !dbg !3722
  %27 = load i32, ptr %iter3.sroa.0.0.ptr138.us.2.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %28 = trunc nuw i32 %27 to i1, !dbg !3705
  br i1 %28, label %bb47.us.2.i, label %bb48.us.2.i, !dbg !3705

bb47.us.2.i:                                      ; preds = %bb48.us.1.i
  %29 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 20, !dbg !3704
  %value.us.2.i = load float, ptr %29, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.2.i = getelementptr inbounds nuw i8, ptr %7, i64 120, !dbg !3731
  %30 = getelementptr inbounds nuw i8, ptr %7, i64 124, !dbg !3707
  store float %value.us.2.i, ptr %30, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.2.i = load float, ptr %_82.us.2.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.2.i = bitcast float %_141.us.2.i to i32, !dbg !3709
  %_146.us.2.i = bitcast float %value.us.2.i to i32, !dbg !3714
  %_145.us.2.i = icmp ne i32 %_144.us.2.i, %_146.us.2.i, !dbg !3717
  %31 = icmp eq i32 %_144.us.2.i, -2147483648
  %or.cond.us.2.i = or i1 %_145.us.2.i, %31, !dbg !3717
  %32 = tail call float @llvm.fabs.f32(float %_141.us.2.i)
  %_140.us.2.i = fcmp ueq float %32, 0x7FF0000000000000
  %or.cond51.us.2.i = or i1 %_140.us.2.i, %or.cond.us.2.i, !dbg !3717
  %33 = getelementptr inbounds nuw i8, ptr %7, i64 128, !dbg !3687
  br i1 %or.cond51.us.2.i, label %bb76.us.2.i, label %bb75.us.2.i, !dbg !3717

bb75.us.2.i:                                      ; preds = %bb47.us.2.i
  store float %value.us.2.i, ptr %_82.us.2.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.2.sink.split.i, !dbg !3719

bb76.us.2.i:                                      ; preds = %bb47.us.2.i
  %_142.us.2.i = fsub float %value.us.2.i, %_141.us.2.i, !dbg !3720
  %34 = fmul float %_142.us.2.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.2.sink.split.i, !dbg !3719

bb48.us.2.sink.split.i:                           ; preds = %bb76.us.2.i, %bb75.us.2.i
  %.sink237.i = phi float [ %34, %bb76.us.2.i ], [ 0.000000e+00, %bb75.us.2.i ]
  %.sink235.i = phi i32 [ 64, %bb76.us.2.i ], [ 0, %bb75.us.2.i ]
  store float %.sink237.i, ptr %33, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %35 = getelementptr inbounds nuw i8, ptr %7, i64 132, !dbg !3687
  store i32 %.sink235.i, ptr %35, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.2.i, !dbg !3722

bb48.us.2.i:                                      ; preds = %bb48.us.2.sink.split.i, %bb48.us.1.i
  %iter3.sroa.0.0.ptr138.us.3.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 24, !dbg !3722
  %36 = load i32, ptr %iter3.sroa.0.0.ptr138.us.3.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %37 = trunc nuw i32 %36 to i1, !dbg !3705
  br i1 %37, label %bb47.us.3.i, label %bb48.us.3.i, !dbg !3705

bb47.us.3.i:                                      ; preds = %bb48.us.2.i
  %38 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 28, !dbg !3704
  %value.us.3.i = load float, ptr %38, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.3.i = getelementptr inbounds nuw i8, ptr %7, i64 136, !dbg !3731
  %39 = getelementptr inbounds nuw i8, ptr %7, i64 140, !dbg !3707
  store float %value.us.3.i, ptr %39, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.3.i = load float, ptr %_82.us.3.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.3.i = bitcast float %_141.us.3.i to i32, !dbg !3709
  %_146.us.3.i = bitcast float %value.us.3.i to i32, !dbg !3714
  %_145.us.3.i = icmp ne i32 %_144.us.3.i, %_146.us.3.i, !dbg !3717
  %40 = icmp eq i32 %_144.us.3.i, -2147483648
  %or.cond.us.3.i = or i1 %_145.us.3.i, %40, !dbg !3717
  %41 = tail call float @llvm.fabs.f32(float %_141.us.3.i)
  %_140.us.3.i = fcmp ueq float %41, 0x7FF0000000000000
  %or.cond51.us.3.i = or i1 %_140.us.3.i, %or.cond.us.3.i, !dbg !3717
  %42 = getelementptr inbounds nuw i8, ptr %7, i64 144, !dbg !3687
  br i1 %or.cond51.us.3.i, label %bb76.us.3.i, label %bb75.us.3.i, !dbg !3717

bb75.us.3.i:                                      ; preds = %bb47.us.3.i
  store float %value.us.3.i, ptr %_82.us.3.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.3.sink.split.i, !dbg !3719

bb76.us.3.i:                                      ; preds = %bb47.us.3.i
  %_142.us.3.i = fsub float %value.us.3.i, %_141.us.3.i, !dbg !3720
  %43 = fmul float %_142.us.3.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.3.sink.split.i, !dbg !3719

bb48.us.3.sink.split.i:                           ; preds = %bb76.us.3.i, %bb75.us.3.i
  %.sink240.i = phi float [ %43, %bb76.us.3.i ], [ 0.000000e+00, %bb75.us.3.i ]
  %.sink238.i = phi i32 [ 64, %bb76.us.3.i ], [ 0, %bb75.us.3.i ]
  store float %.sink240.i, ptr %42, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %44 = getelementptr inbounds nuw i8, ptr %7, i64 148, !dbg !3687
  store i32 %.sink238.i, ptr %44, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.3.i, !dbg !3722

bb48.us.3.i:                                      ; preds = %bb48.us.3.sink.split.i, %bb48.us.2.i
  %iter3.sroa.0.0.ptr138.us.4.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 32, !dbg !3722
  %45 = load i32, ptr %iter3.sroa.0.0.ptr138.us.4.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %46 = trunc nuw i32 %45 to i1, !dbg !3705
  br i1 %46, label %bb47.us.4.i, label %bb48.us.4.i, !dbg !3705

bb47.us.4.i:                                      ; preds = %bb48.us.3.i
  %47 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 36, !dbg !3704
  %value.us.4.i = load float, ptr %47, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.4.i = getelementptr inbounds nuw i8, ptr %7, i64 152, !dbg !3731
  %48 = getelementptr inbounds nuw i8, ptr %7, i64 156, !dbg !3707
  store float %value.us.4.i, ptr %48, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.4.i = load float, ptr %_82.us.4.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.4.i = bitcast float %_141.us.4.i to i32, !dbg !3709
  %_146.us.4.i = bitcast float %value.us.4.i to i32, !dbg !3714
  %_145.us.4.i = icmp ne i32 %_144.us.4.i, %_146.us.4.i, !dbg !3717
  %49 = icmp eq i32 %_144.us.4.i, -2147483648
  %or.cond.us.4.i = or i1 %_145.us.4.i, %49, !dbg !3717
  %50 = tail call float @llvm.fabs.f32(float %_141.us.4.i)
  %_140.us.4.i = fcmp ueq float %50, 0x7FF0000000000000
  %or.cond51.us.4.i = or i1 %_140.us.4.i, %or.cond.us.4.i, !dbg !3717
  %51 = getelementptr inbounds nuw i8, ptr %7, i64 160, !dbg !3687
  br i1 %or.cond51.us.4.i, label %bb76.us.4.i, label %bb75.us.4.i, !dbg !3717

bb75.us.4.i:                                      ; preds = %bb47.us.4.i
  store float %value.us.4.i, ptr %_82.us.4.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.4.sink.split.i, !dbg !3719

bb76.us.4.i:                                      ; preds = %bb47.us.4.i
  %_142.us.4.i = fsub float %value.us.4.i, %_141.us.4.i, !dbg !3720
  %52 = fmul float %_142.us.4.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.4.sink.split.i, !dbg !3719

bb48.us.4.sink.split.i:                           ; preds = %bb76.us.4.i, %bb75.us.4.i
  %.sink243.i = phi float [ %52, %bb76.us.4.i ], [ 0.000000e+00, %bb75.us.4.i ]
  %.sink241.i = phi i32 [ 64, %bb76.us.4.i ], [ 0, %bb75.us.4.i ]
  store float %.sink243.i, ptr %51, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %53 = getelementptr inbounds nuw i8, ptr %7, i64 164, !dbg !3687
  store i32 %.sink241.i, ptr %53, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.4.i, !dbg !3722

bb48.us.4.i:                                      ; preds = %bb48.us.4.sink.split.i, %bb48.us.3.i
  %iter3.sroa.0.0.ptr138.us.5.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 40, !dbg !3722
  %54 = load i32, ptr %iter3.sroa.0.0.ptr138.us.5.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %55 = trunc nuw i32 %54 to i1, !dbg !3705
  br i1 %55, label %bb47.us.5.i, label %bb48.us.5.i, !dbg !3705

bb47.us.5.i:                                      ; preds = %bb48.us.4.i
  %56 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 44, !dbg !3704
  %value.us.5.i = load float, ptr %56, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.5.i = getelementptr inbounds nuw i8, ptr %7, i64 168, !dbg !3731
  %57 = getelementptr inbounds nuw i8, ptr %7, i64 172, !dbg !3707
  store float %value.us.5.i, ptr %57, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.5.i = load float, ptr %_82.us.5.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.5.i = bitcast float %_141.us.5.i to i32, !dbg !3709
  %_146.us.5.i = bitcast float %value.us.5.i to i32, !dbg !3714
  %_145.us.5.i = icmp ne i32 %_144.us.5.i, %_146.us.5.i, !dbg !3717
  %58 = icmp eq i32 %_144.us.5.i, -2147483648
  %or.cond.us.5.i = or i1 %_145.us.5.i, %58, !dbg !3717
  %59 = tail call float @llvm.fabs.f32(float %_141.us.5.i)
  %_140.us.5.i = fcmp ueq float %59, 0x7FF0000000000000
  %or.cond51.us.5.i = or i1 %_140.us.5.i, %or.cond.us.5.i, !dbg !3717
  %60 = getelementptr inbounds nuw i8, ptr %7, i64 176, !dbg !3687
  br i1 %or.cond51.us.5.i, label %bb76.us.5.i, label %bb75.us.5.i, !dbg !3717

bb75.us.5.i:                                      ; preds = %bb47.us.5.i
  store float %value.us.5.i, ptr %_82.us.5.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.5.sink.split.i, !dbg !3719

bb76.us.5.i:                                      ; preds = %bb47.us.5.i
  %_142.us.5.i = fsub float %value.us.5.i, %_141.us.5.i, !dbg !3720
  %61 = fmul float %_142.us.5.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.5.sink.split.i, !dbg !3719

bb48.us.5.sink.split.i:                           ; preds = %bb76.us.5.i, %bb75.us.5.i
  %.sink246.i = phi float [ %61, %bb76.us.5.i ], [ 0.000000e+00, %bb75.us.5.i ]
  %.sink244.i = phi i32 [ 64, %bb76.us.5.i ], [ 0, %bb75.us.5.i ]
  store float %.sink246.i, ptr %60, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %62 = getelementptr inbounds nuw i8, ptr %7, i64 180, !dbg !3687
  store i32 %.sink244.i, ptr %62, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.5.i, !dbg !3722

bb48.us.5.i:                                      ; preds = %bb48.us.5.sink.split.i, %bb48.us.4.i
  %iter3.sroa.0.0.ptr138.us.6.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 48, !dbg !3722
  %63 = load i32, ptr %iter3.sroa.0.0.ptr138.us.6.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %64 = trunc nuw i32 %63 to i1, !dbg !3705
  br i1 %64, label %bb47.us.6.i, label %bb48.us.6.i, !dbg !3705

bb47.us.6.i:                                      ; preds = %bb48.us.5.i
  %65 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 52, !dbg !3704
  %value.us.6.i = load float, ptr %65, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.6.i = getelementptr inbounds nuw i8, ptr %7, i64 184, !dbg !3731
  %66 = getelementptr inbounds nuw i8, ptr %7, i64 188, !dbg !3707
  store float %value.us.6.i, ptr %66, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.6.i = load float, ptr %_82.us.6.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.6.i = bitcast float %_141.us.6.i to i32, !dbg !3709
  %_146.us.6.i = bitcast float %value.us.6.i to i32, !dbg !3714
  %_145.us.6.i = icmp ne i32 %_144.us.6.i, %_146.us.6.i, !dbg !3717
  %67 = icmp eq i32 %_144.us.6.i, -2147483648
  %or.cond.us.6.i = or i1 %_145.us.6.i, %67, !dbg !3717
  %68 = tail call float @llvm.fabs.f32(float %_141.us.6.i)
  %_140.us.6.i = fcmp ueq float %68, 0x7FF0000000000000
  %or.cond51.us.6.i = or i1 %_140.us.6.i, %or.cond.us.6.i, !dbg !3717
  %69 = getelementptr inbounds nuw i8, ptr %7, i64 192, !dbg !3687
  br i1 %or.cond51.us.6.i, label %bb76.us.6.i, label %bb75.us.6.i, !dbg !3717

bb75.us.6.i:                                      ; preds = %bb47.us.6.i
  store float %value.us.6.i, ptr %_82.us.6.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.6.sink.split.i, !dbg !3719

bb76.us.6.i:                                      ; preds = %bb47.us.6.i
  %_142.us.6.i = fsub float %value.us.6.i, %_141.us.6.i, !dbg !3720
  %70 = fmul float %_142.us.6.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.6.sink.split.i, !dbg !3719

bb48.us.6.sink.split.i:                           ; preds = %bb76.us.6.i, %bb75.us.6.i
  %.sink249.i = phi float [ %70, %bb76.us.6.i ], [ 0.000000e+00, %bb75.us.6.i ]
  %.sink247.i = phi i32 [ 64, %bb76.us.6.i ], [ 0, %bb75.us.6.i ]
  store float %.sink249.i, ptr %69, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %71 = getelementptr inbounds nuw i8, ptr %7, i64 196, !dbg !3687
  store i32 %.sink247.i, ptr %71, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.6.i, !dbg !3722

bb48.us.6.i:                                      ; preds = %bb48.us.6.sink.split.i, %bb48.us.5.i
  %iter3.sroa.0.0.ptr138.us.7.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 56, !dbg !3722
  %72 = load i32, ptr %iter3.sroa.0.0.ptr138.us.7.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %73 = trunc nuw i32 %72 to i1, !dbg !3705
  br i1 %73, label %bb47.us.7.i, label %bb48.us.7.i, !dbg !3705

bb47.us.7.i:                                      ; preds = %bb48.us.6.i
  %74 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 60, !dbg !3704
  %value.us.7.i = load float, ptr %74, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.7.i = getelementptr inbounds nuw i8, ptr %7, i64 200, !dbg !3731
  %75 = getelementptr inbounds nuw i8, ptr %7, i64 204, !dbg !3707
  store float %value.us.7.i, ptr %75, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.7.i = load float, ptr %_82.us.7.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.7.i = bitcast float %_141.us.7.i to i32, !dbg !3709
  %_146.us.7.i = bitcast float %value.us.7.i to i32, !dbg !3714
  %_145.us.7.i = icmp ne i32 %_144.us.7.i, %_146.us.7.i, !dbg !3717
  %76 = icmp eq i32 %_144.us.7.i, -2147483648
  %or.cond.us.7.i = or i1 %_145.us.7.i, %76, !dbg !3717
  %77 = tail call float @llvm.fabs.f32(float %_141.us.7.i)
  %_140.us.7.i = fcmp ueq float %77, 0x7FF0000000000000
  %or.cond51.us.7.i = or i1 %_140.us.7.i, %or.cond.us.7.i, !dbg !3717
  %78 = getelementptr inbounds nuw i8, ptr %7, i64 208, !dbg !3687
  br i1 %or.cond51.us.7.i, label %bb76.us.7.i, label %bb75.us.7.i, !dbg !3717

bb75.us.7.i:                                      ; preds = %bb47.us.7.i
  store float %value.us.7.i, ptr %_82.us.7.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.7.sink.split.i, !dbg !3719

bb76.us.7.i:                                      ; preds = %bb47.us.7.i
  %_142.us.7.i = fsub float %value.us.7.i, %_141.us.7.i, !dbg !3720
  %79 = fmul float %_142.us.7.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.7.sink.split.i, !dbg !3719

bb48.us.7.sink.split.i:                           ; preds = %bb76.us.7.i, %bb75.us.7.i
  %.sink252.i = phi float [ %79, %bb76.us.7.i ], [ 0.000000e+00, %bb75.us.7.i ]
  %.sink250.i = phi i32 [ 64, %bb76.us.7.i ], [ 0, %bb75.us.7.i ]
  store float %.sink252.i, ptr %78, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %80 = getelementptr inbounds nuw i8, ptr %7, i64 212, !dbg !3687
  store i32 %.sink250.i, ptr %80, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.7.i, !dbg !3722

bb48.us.7.i:                                      ; preds = %bb48.us.7.sink.split.i, %bb48.us.6.i
  %iter3.sroa.0.0.ptr138.us.8.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 64, !dbg !3722
  %81 = load i32, ptr %iter3.sroa.0.0.ptr138.us.8.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %82 = trunc nuw i32 %81 to i1, !dbg !3705
  br i1 %82, label %bb47.us.8.i, label %bb48.us.8.i, !dbg !3705

bb47.us.8.i:                                      ; preds = %bb48.us.7.i
  %83 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 68, !dbg !3704
  %value.us.8.i = load float, ptr %83, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.8.i = getelementptr inbounds nuw i8, ptr %7, i64 216, !dbg !3731
  %84 = getelementptr inbounds nuw i8, ptr %7, i64 220, !dbg !3707
  store float %value.us.8.i, ptr %84, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.8.i = load float, ptr %_82.us.8.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.8.i = bitcast float %_141.us.8.i to i32, !dbg !3709
  %_146.us.8.i = bitcast float %value.us.8.i to i32, !dbg !3714
  %_145.us.8.i = icmp ne i32 %_144.us.8.i, %_146.us.8.i, !dbg !3717
  %85 = icmp eq i32 %_144.us.8.i, -2147483648
  %or.cond.us.8.i = or i1 %_145.us.8.i, %85, !dbg !3717
  %86 = tail call float @llvm.fabs.f32(float %_141.us.8.i)
  %_140.us.8.i = fcmp ueq float %86, 0x7FF0000000000000
  %or.cond51.us.8.i = or i1 %_140.us.8.i, %or.cond.us.8.i, !dbg !3717
  %87 = getelementptr inbounds nuw i8, ptr %7, i64 224, !dbg !3687
  br i1 %or.cond51.us.8.i, label %bb76.us.8.i, label %bb75.us.8.i, !dbg !3717

bb75.us.8.i:                                      ; preds = %bb47.us.8.i
  store float %value.us.8.i, ptr %_82.us.8.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb48.us.8.sink.split.i, !dbg !3719

bb76.us.8.i:                                      ; preds = %bb47.us.8.i
  %_142.us.8.i = fsub float %value.us.8.i, %_141.us.8.i, !dbg !3720
  %88 = fmul float %_142.us.8.i, 1.562500e-02, !dbg !3721
  br label %bb48.us.8.sink.split.i, !dbg !3719

bb48.us.8.sink.split.i:                           ; preds = %bb76.us.8.i, %bb75.us.8.i
  %.sink255.i = phi float [ %88, %bb76.us.8.i ], [ 0.000000e+00, %bb75.us.8.i ]
  %.sink253.i = phi i32 [ 64, %bb76.us.8.i ], [ 0, %bb75.us.8.i ]
  store float %.sink255.i, ptr %87, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  %89 = getelementptr inbounds nuw i8, ptr %7, i64 228, !dbg !3687
  store i32 %.sink253.i, ptr %89, align 4, !dbg !3687, !alias.scope !3630, !noalias !3694
  br label %bb48.us.8.i, !dbg !3722

bb48.us.8.i:                                      ; preds = %bb48.us.8.sink.split.i, %bb48.us.7.i
  %iter3.sroa.0.0.ptr138.us.9.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 72, !dbg !3722
  %90 = load i32, ptr %iter3.sroa.0.0.ptr138.us.9.i, align 4, !dbg !3704, !range !1163, !noalias !3639, !noundef !11
  %91 = trunc nuw i32 %90 to i1, !dbg !3705
  br i1 %91, label %bb47.us.9.i, label %bb36.loopexit.i, !dbg !3705

bb47.us.9.i:                                      ; preds = %bb48.us.8.i
  %92 = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.0.ptr.i, i64 76, !dbg !3704
  %value.us.9.i = load float, ptr %92, align 4, !dbg !3706, !noalias !3639, !noundef !11
  %_82.us.9.i = getelementptr inbounds nuw i8, ptr %7, i64 232, !dbg !3731
  %93 = getelementptr inbounds nuw i8, ptr %7, i64 236, !dbg !3707
  store float %value.us.9.i, ptr %93, align 4, !dbg !3707, !alias.scope !3630, !noalias !3694
  %_141.us.9.i = load float, ptr %_82.us.9.i, align 4, !dbg !3708, !alias.scope !3630, !noalias !3694, !noundef !11
  %_144.us.9.i = bitcast float %_141.us.9.i to i32, !dbg !3709
  %_146.us.9.i = bitcast float %value.us.9.i to i32, !dbg !3714
  %_145.us.9.i = icmp ne i32 %_144.us.9.i, %_146.us.9.i, !dbg !3717
  %94 = icmp eq i32 %_144.us.9.i, -2147483648
  %or.cond.us.9.i = or i1 %_145.us.9.i, %94, !dbg !3717
  %95 = tail call float @llvm.fabs.f32(float %_141.us.9.i)
  %_140.us.9.i = fcmp ueq float %95, 0x7FF0000000000000
  %or.cond51.us.9.i = or i1 %_140.us.9.i, %or.cond.us.9.i, !dbg !3717
  %96 = getelementptr inbounds nuw i8, ptr %7, i64 240, !dbg !3687
  br i1 %or.cond51.us.9.i, label %bb76.us.9.i, label %bb75.us.9.i, !dbg !3717

bb75.us.9.i:                                      ; preds = %bb47.us.9.i
  store float %value.us.9.i, ptr %_82.us.9.i, align 4, !dbg !3718, !alias.scope !3630, !noalias !3694
  br label %bb36.loopexit.sink.split.i, !dbg !3719

bb76.us.9.i:                                      ; preds = %bb47.us.9.i
  %_142.us.9.i = fsub float %value.us.9.i, %_141.us.9.i, !dbg !3720
  %97 = fmul float %_142.us.9.i, 1.562500e-02, !dbg !3721
  br label %bb36.loopexit.sink.split.i, !dbg !3719

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i.unreachabledefault: ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i
  unreachable

default.unreachable:                              ; preds = %bb5
  unreachable

bb7.i6429:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i
  br label %bb9.i6430, !dbg !3732

bb9.i6430:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i, %bb7.i6429
  %side.sroa.0.0.sroa.phi35.i = phi ptr [ %4, %bb7.i6429 ], [ %pending.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i ], !dbg !3733
  %side.sroa.0.0.i = phi i32 [ 1, %bb7.i6429 ], [ 0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i ], !dbg !3733
  %98 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 16, !dbg !3734
  %_23.i = load i32, ptr %98, align 8, !dbg !3734, !alias.scope !3634, !noalias !3685, !noundef !11
  %99 = zext i32 %_23.i to i64, !dbg !3734
  %_106.0.i = shl nuw i32 %_23.i, 1, !dbg !3736
  %_112.0.i = or disjoint i32 %_106.0.i, %side.sroa.0.0.i, !dbg !3736
  %_115.i = icmp ult i32 %_23.i, 2, !dbg !3746
  br i1 %_115.i, label %bb60.i, label %bb59.split.i, !dbg !3746

bb59.split.i:                                     ; preds = %bb9.i6430
  %_116.i = add nsw i64 %99, -2, !dbg !3750
  %100 = icmp ult i32 %_23.i, 12, !dbg !3751
  %_0.sroa.0.0.i74.i = zext i1 %100 to i64, !dbg !3751
  %101 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0.i74.i, 0, !dbg !3755
  %102 = insertvalue { i64, i64 } %101, i64 %_116.i, 1, !dbg !3755
  br label %bb60.i, !dbg !3756

bb60.i:                                           ; preds = %bb59.split.i, %bb9.i6430
  %phi.call.i = phi { i64, i64 } [ %102, %bb59.split.i ], [ { i64 0, i64 undef }, %bb9.i6430 ], !dbg !3757
  %103 = extractvalue { i64, i64 } %phi.call.i, 0, !dbg !3757
  %104 = extractvalue { i64, i64 } %phi.call.i, 1, !dbg !3757
  %105 = icmp eq i64 %103, 1, !dbg !3758
  %_32.i = icmp ult i64 %iter.sroa.7.0122.i, %_33.i
  %or.cond16.i = and i1 %_32.i, %105, !dbg !3758
  br i1 %or.cond16.i, label %bb12.i, label %bb35.i, !dbg !3758

bb12.i:                                           ; preds = %bb60.i
  %106 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 28, !dbg !3760
  %_118.i = load i32, ptr %106, align 4, !dbg !3760, !range !3764, !alias.scope !3634, !noalias !3685, !noundef !11
  %107 = icmp eq i32 %_118.i, 1, !dbg !3765
  br i1 %107, label %bb13.i6431, label %bb35.i, !dbg !3765

bb13.i6431:                                       ; preds = %bb12.i
  %_35.i = load i64, ptr %iter.sroa.0.0123.i, align 8, !dbg !3766, !alias.scope !3634, !noalias !3685, !noundef !11
  %_34.i = icmp eq i64 %_35.i, %_7, !dbg !3766
  br i1 %_34.i, label %bb14.i6432, label %bb35.i, !dbg !3766

bb14.i6432:                                       ; preds = %bb13.i6431
  %108 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 8, !dbg !3767
  %_37.i = load i64, ptr %108, align 8, !dbg !3767, !alias.scope !3634, !noalias !3685, !noundef !11
  %_36.i = icmp eq i64 %_37.i, %_7, !dbg !3767
  br i1 %_36.i, label %bb15.i, label %bb35.i, !dbg !3767

bb15.i:                                           ; preds = %bb14.i6432
  %109 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 20, !dbg !3768
  %_40.i = load float, ptr %109, align 4, !dbg !3768, !alias.scope !3634, !noalias !3685, !noundef !11
  %_39.i = bitcast float %_40.i to i32, !dbg !3769
  %110 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 24, !dbg !3771
  %_4250.i = load i32, ptr %110, align 8, !dbg !3771, !alias.scope !3634, !noalias !3685, !noundef !11
  %_38.i = icmp eq i32 %_4250.i, %_39.i, !dbg !3768
  br i1 %_38.i, label %bb16.i, label %bb35.i, !dbg !3768

bb16.i:                                           ; preds = %bb15.i
  %_45.i = icmp ult i32 %_23.i, 12, !dbg !3772
  br i1 %_45.i, label %bb17.i, label %panic11.i, !dbg !3772

bb17.i:                                           ; preds = %bb16.i
  %_44.i = getelementptr inbounds nuw %"effect_runtime::params::ParameterSpec", ptr @alloc_cc33a3b9cd8c16d253f2168b5461d31d, i64 %99, !dbg !3773
; call effect_runtime::params::parameter_value_valid
  %_43.i = tail call noundef zeroext i1 @_RNvNtCseSJggkR25Fr_14effect_runtime6params21parameter_value_valid(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(40) %_44.i, float noundef %_40.i) #25, !dbg !3774, !noalias !3639
  %_46.i = icmp ugt i32 %_112.0.i, %previous.sroa.5.0.ph127.i
  %or.cond54.not.not144.i = select i1 %previous.sroa.0.0.ph128.not.i, i1 true, i1 %_46.i, !dbg !3774
  %or.cond99.not.i = select i1 %_43.i, i1 %or.cond54.not.not144.i, i1 false, !dbg !3774
  br i1 %or.cond99.not.i, label %bb67.i, label %bb35.i, !dbg !3774

panic11.i:                                        ; preds = %bb16.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %99, i64 noundef 12, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_26decc7ea6b284c3e2e2a8674b90acb8) #26, !dbg !3772, !noalias !3639
  unreachable, !dbg !3772

bb67.i:                                           ; preds = %bb17.i
  %_128.i = icmp ult i64 %104, 10, !dbg !3775
  br i1 %_128.i, label %bb68.i, label %panic13.i, !dbg !3775

bb68.i:                                           ; preds = %bb67.i
  %_125.i = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %side.sroa.0.0.sroa.phi35.i, i64 %104, !dbg !3775
  %111 = load i32, ptr %_125.i, align 4, !dbg !3784, !range !1163, !noalias !3639, !noundef !11
  %_129.not.i = icmp eq i32 %111, 0, !dbg !3791
  br i1 %_129.not.i, label %bb31.i, label %bb35.i, !dbg !3792

panic13.i:                                        ; preds = %bb67.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %104, i64 noundef 10, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_229cdd12ec7f4a0b614787777e6def50) #26, !dbg !3775, !noalias !3639
  unreachable, !dbg !3775

bb31.i:                                           ; preds = %bb68.i
  %_125.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %side.sroa.0.0.sroa.phi35.i, i64 %104
  %_131.i = fcmp oeq float %_40.i, 0.000000e+00, !dbg !3795
  %_57.sroa.0.0.i = select i1 %_131.i, float 0.000000e+00, float %_40.i, !dbg !3795
  store i32 1, ptr %_125.i.le, align 4, !dbg !3798, !noalias !3639
  %112 = getelementptr inbounds nuw i8, ptr %_125.i.le, i64 4, !dbg !3798
  store float %_57.sroa.0.0.i, ptr %112, align 4, !dbg !3798, !noalias !3639
  %_6.i.i121.i = icmp eq ptr %_16.i.i.i6428, %_91.i, !dbg !3653
  br i1 %_6.i.i121.i, label %bb36.preheader.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i, !dbg !3663

bb35.i:                                           ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i, %bb68.i, %bb17.i, %bb15.i, %bb14.i6432, %bb13.i6431, %bb12.i, %bb60.i
  %113 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted130.i, i64 1), !dbg !3799
  %_6.i.i.i = icmp eq ptr %_16.i.i.i6428, %_91.i, !dbg !3653
  br i1 %_6.i.i.i, label %bb36.preheader.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i, !dbg !3663

_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E16apply_automationB5_.exit: ; preds = %bb36.loopexit.i
  call void @llvm.lifetime.end.p0(ptr nonnull %pending.i), !dbg !3803, !noalias !3639
  %_19.0 = load ptr, ptr %block, align 8, !dbg !3804, !nonnull !11, !align !3103, !noundef !11
  %114 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !3804
  %_19.1 = load i64, ptr %114, align 8, !dbg !3804, !noundef !11
  %115 = icmp ne i64 %_19.1, 0, !dbg !3805
  %116 = getelementptr inbounds nuw i8, ptr %block, i64 24
  %_20.1 = load i64, ptr %116, align 8
  %_10.not = icmp eq i64 %_20.1, %_19.1
  %or.cond = select i1 %115, i1 %_10.not, i1 false, !dbg !3805
  br i1 %or.cond, label %bb5, label %bb4, !dbg !3805

bb4:                                              ; preds = %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E16apply_automationB5_.exit
  %report.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16, !dbg !3807
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, i8 0, i64 16, i1 false), !dbg !3807
  store i64 %report.sroa.7.2, ptr %report.sroa.7.0._0.sroa_idx, align 8, !dbg !3807
  %report.sroa.10.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24, !dbg !3807
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %report.sroa.10.0._0.sroa_idx, i8 0, i64 16, i1 false), !dbg !3807
  br label %bb7, !dbg !3808

bb5:                                              ; preds = %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E16apply_automationB5_.exit
  %117 = getelementptr inbounds nuw i8, ptr %block, i64 16, !dbg !3809
  %_20.0 = load ptr, ptr %117, align 8, !dbg !3809, !nonnull !11, !align !3103, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3810), !dbg !3813
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 860, !dbg !3815
  %_6.i = load i32, ptr %118, align 4, !dbg !3815, !range !2567, !alias.scope !3810, !noalias !3818, !noundef !11
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 864, !dbg !3822
  %120 = load i8, ptr %119, align 8, !dbg !3822, !range !2487, !alias.scope !3810, !noalias !3818, !noundef !11
  %_7.i = trunc nuw i8 %120 to i1, !dbg !3822
  switch i32 %_6.i, label %default.unreachable [
    i32 1, label %bb2.i
    i32 2, label %bb3.i
    i32 3, label %bb4.i
  ], !dbg !3823

bb2.i:                                            ; preds = %bb5
  br i1 %_7.i, label %bb2.i19.i.lr.ph, label %bb2.i.i.lr.ph, !dbg !3823

bb3.i:                                            ; preds = %bb5
  br i1 %_7.i, label %bb2.i19.i.lr.ph, label %bb2.i132.i.lr.ph, !dbg !3823

bb4.i:                                            ; preds = %bb5
  br i1 %_7.i, label %bb2.i19.i.lr.ph, label %bb2.i590.i.lr.ph, !dbg !3823

bb2.i.i.lr.ph:                                    ; preds = %bb2.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3824), !dbg !3827
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 856, !dbg !3828
  %sample_rate.i.i = load i32, ptr %121, align 8, !dbg !3828, !alias.scope !3831, !noalias !3832, !noundef !11
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !3835
  %ring_len.i.i = load i64, ptr %122, align 8, !dbg !3835, !alias.scope !3831, !noalias !3832, !noundef !11
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 120
  %124 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %125 = getelementptr inbounds nuw i8, ptr %self, i64 200
  %126 = getelementptr inbounds nuw i8, ptr %self, i64 208
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %128 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %129 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %130 = getelementptr inbounds nuw i8, ptr %self, i64 240
  %131 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %132 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %133 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %134 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %135 = getelementptr inbounds nuw i8, ptr %self, i64 280
  %136 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %137 = getelementptr inbounds nuw i8, ptr %self, i64 296
  %138 = getelementptr inbounds nuw i8, ptr %self, i64 304
  %139 = getelementptr inbounds nuw i8, ptr %self, i64 312
  %140 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %141 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %142 = getelementptr inbounds nuw i8, ptr %self, i64 336
  %143 = getelementptr inbounds nuw i8, ptr %self, i64 344
  %_18.i.i = getelementptr inbounds nuw i8, ptr %self, i64 480
  %144 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %145 = getelementptr inbounds nuw i8, ptr %self, i64 560
  %146 = getelementptr inbounds nuw i8, ptr %self, i64 568
  %147 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %148 = getelementptr inbounds nuw i8, ptr %self, i64 584
  %149 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %150 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %151 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %152 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %153 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %154 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %155 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %156 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %157 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %158 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %159 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %160 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %161 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %162 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %163 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %coefficients.i.i.sroa.5.0._20.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i.i, i64 4
  %coefficients.i.i.sroa.7.0._20.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i.i, i64 8
  %coefficients.i.i.sroa.9.0._20.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i.i, i64 12
  %coefficients.i.i.sroa.11.0._20.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i.i, i64 16
  %coefficients.i.i.sroa.13.0._20.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i.i, i64 20
  %coefficients.i.i.sroa.18.24._22.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i.i, i64 4
  %coefficients.i.i.sroa.20.24._22.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i.i, i64 8
  %coefficients.i.i.sroa.22.24._22.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i.i, i64 12
  %coefficients.i.i.sroa.24.24._22.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i.i, i64 16
  %coefficients.i.i.sroa.26.24._22.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i.i, i64 20
  %_51.i.i = getelementptr inbounds nuw i8, ptr %self, i64 848
  %164 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %filter_near.i.i.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 172
  %filter_near.i.i.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 176
  %filter_near.i.i.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 180
  %165 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %filter_far.i.i.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 532
  %filter_far.i.i.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 536
  %filter_far.i.i.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 540
  %166 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %.sroa_idx6954 = getelementptr inbounds nuw i8, ptr %self, i64 188
  %167 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %.sroa_idx6959 = getelementptr inbounds nuw i8, ptr %self, i64 548
  %_63.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 152
  %168 = getelementptr inbounds nuw i8, ptr %self, i64 156
  %169 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %170 = getelementptr inbounds nuw i8, ptr %self, i64 164
  %_68.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %171 = getelementptr inbounds nuw i8, ptr %self, i64 516
  %172 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %173 = getelementptr inbounds nuw i8, ptr %self, i64 524
  %174 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %175 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %176 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %177 = getelementptr inbounds nuw i8, ptr %self, i64 488
  %178 = getelementptr inbounds nuw i8, ptr %self, i64 496
  %179 = getelementptr inbounds nuw i8, ptr %self, i64 504
  %_85.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 400
  %_93.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %180 = getelementptr inbounds nuw i8, ptr %self, i64 204
  %181 = getelementptr inbounds nuw i8, ptr %self, i64 220
  %182 = getelementptr inbounds nuw i8, ptr %self, i64 236
  %183 = getelementptr inbounds nuw i8, ptr %self, i64 252
  %184 = getelementptr inbounds nuw i8, ptr %self, i64 268
  %185 = getelementptr inbounds nuw i8, ptr %self, i64 284
  %186 = getelementptr inbounds nuw i8, ptr %self, i64 300
  %187 = getelementptr inbounds nuw i8, ptr %self, i64 316
  %188 = getelementptr inbounds nuw i8, ptr %self, i64 332
  %189 = getelementptr inbounds nuw i8, ptr %self, i64 348
  %190 = getelementptr inbounds nuw i8, ptr %self, i64 564
  %191 = getelementptr inbounds nuw i8, ptr %self, i64 580
  %192 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %193 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %194 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %195 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %196 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %197 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %198 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %199 = getelementptr inbounds nuw i8, ptr %self, i64 708
  br label %bb2.i.i, !dbg !3837

bb2.i.i:                                          ; preds = %bb2.i.i.lr.ph, %bb15.i.i
  %position.sroa.0.0.i.i11228 = phi i64 [ 0, %bb2.i.i.lr.ph ], [ %_32.i.i, %bb15.i.i ]
  %_11.i.i = sub nuw i64 %_19.1, %position.sroa.0.0.i.i11228, !dbg !3840
; call <multiband_compressor::Instance<f32, 1>>::plan_segment
  %200 = tail call fastcc { i64, i1 } @_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_(ptr noalias noundef nonnull align 8 dereferenceable(768) %_5, i64 noundef %_11.i.i) #25, !dbg !3841, !noalias !3842
  %plan.0.i.i = extractvalue { i64, i1 } %200, 0, !dbg !3841
  %plan.1.i.i = extractvalue { i64, i1 } %200, 1, !dbg !3841
  %_12.le.i = load float, ptr %124, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.i = load float, ptr %125, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.1.i = load float, ptr %126, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.1.i = load float, ptr %127, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.2.i = load float, ptr %128, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.2.i = load float, ptr %129, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.3.i = load float, ptr %130, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.3.i = load float, ptr %131, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.4.i = load float, ptr %132, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.4.i = load float, ptr %133, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.5.i = load float, ptr %134, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.5.i = load float, ptr %135, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.6.i = load float, ptr %136, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.6.i = load float, ptr %137, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.7.i = load float, ptr %138, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.7.i = load float, ptr %139, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.8.i = load float, ptr %140, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.8.i = load float, ptr %141, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.9.i = load float, ptr %142, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_16.le.9.i = load float, ptr %143, align 8, !alias.scope !3843, !noalias !3846, !noundef !11
  %_12.le.i6433 = load float, ptr %144, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.i6434 = load float, ptr %145, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.1.i6435 = load float, ptr %146, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.1.i6436 = load float, ptr %147, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.2.i6437 = load float, ptr %148, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.2.i6438 = load float, ptr %149, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.3.i6439 = load float, ptr %150, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.3.i6440 = load float, ptr %151, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.4.i6441 = load float, ptr %152, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.4.i6442 = load float, ptr %153, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.5.i6443 = load float, ptr %154, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.5.i6444 = load float, ptr %155, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.6.i6445 = load float, ptr %156, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.6.i6446 = load float, ptr %157, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.7.i6447 = load float, ptr %158, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.7.i6448 = load float, ptr %159, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.8.i6449 = load float, ptr %160, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.8.i6450 = load float, ptr %161, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.9.i6451 = load float, ptr %162, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.9.i6452 = load float, ptr %163, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  call void @llvm.lifetime.start.p0(ptr nonnull %_20.i.i), !dbg !3853, !noalias !3857
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_20.i.i, ptr noalias noundef align 8 dereferenceable(360) %123, i32 noundef %sample_rate.i.i) #25, !dbg !3858, !noalias !3842
  call void @llvm.lifetime.start.p0(ptr nonnull %_22.i.i), !dbg !3859, !noalias !3857
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_22.i.i, ptr noalias noundef align 8 dereferenceable(360) %_18.i.i, i32 noundef %sample_rate.i.i) #25, !dbg !3860, !noalias !3842
  %coefficients.i.i.sroa.0.0.copyload = load float, ptr %_20.i.i, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.5.0.copyload = load float, ptr %coefficients.i.i.sroa.5.0._20.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.7.0.copyload = load float, ptr %coefficients.i.i.sroa.7.0._20.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.9.0.copyload = load float, ptr %coefficients.i.i.sroa.9.0._20.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.11.0.copyload = load float, ptr %coefficients.i.i.sroa.11.0._20.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.13.0.copyload = load float, ptr %coefficients.i.i.sroa.13.0._20.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.15.24.copyload = load float, ptr %_22.i.i, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.18.24.copyload = load float, ptr %coefficients.i.i.sroa.18.24._22.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.20.24.copyload = load float, ptr %coefficients.i.i.sroa.20.24._22.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.22.24.copyload = load float, ptr %coefficients.i.i.sroa.22.24._22.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.24.24.copyload = load float, ptr %coefficients.i.i.sroa.24.24._22.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  %coefficients.i.i.sroa.26.24.copyload = load float, ptr %coefficients.i.i.sroa.26.24._22.i.i.sroa_idx, align 4, !dbg !3861, !noalias !3857
  call void @llvm.lifetime.end.p0(ptr nonnull %_22.i.i), !dbg !3862, !noalias !3857
  call void @llvm.lifetime.end.p0(ptr nonnull %_20.i.i), !dbg !3862, !noalias !3857
  %_32.i.i = add i64 %plan.0.i.i, %position.sroa.0.0.i.i11228, !dbg !3863
  %_72.i.i = icmp ult i64 %_32.i.i, %position.sroa.0.0.i.i11228, !dbg !3865
  %_66.not.i.i = icmp ugt i64 %_32.i.i, %_19.1
  %or.cond11.i.i = or i1 %_72.i.i, %_66.not.i.i, !dbg !3865
  br i1 %plan.1.i.i, label %bb9.i.i, label %bb13.i.i, !dbg !3873

bb9.i.i:                                          ; preds = %bb2.i.i
  br i1 %or.cond11.i.i, label %bb19.i.i, label %bb17.i.i, !dbg !3874, !prof !3878

bb13.i.i:                                         ; preds = %bb2.i.i
  br i1 %or.cond11.i.i, label %bb29.i.i, label %bb27.i.i, !dbg !3879, !prof !3878

bb27.i.i:                                         ; preds = %bb13.i.i
  %_95.i.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i.i11228, !dbg !3885
  %_105.i.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i.i11228, !dbg !3889
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3896), !dbg !3899
  %filter_near.i.i.i.sroa.0.0.copyload = load float, ptr %164, align 8, !dbg !3900, !noalias !3906
  %filter_near.i.i.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !3900, !noalias !3906
  %filter_near.i.i.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !3900, !noalias !3906
  %filter_near.i.i.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !3900, !noalias !3906
  %filter_far.i.i.i.sroa.0.0.copyload = load float, ptr %165, align 8, !dbg !3911, !noalias !3906
  %filter_far.i.i.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !3911, !noalias !3906
  %filter_far.i.i.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !3911, !noalias !3906
  %filter_far.i.i.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !3911, !noalias !3906
  %201 = load i32, ptr %166, align 8, !dbg !3913
  %202 = load i32, ptr %.sroa_idx6954, align 4, !dbg !3913
  %203 = load i32, ptr %167, align 8, !dbg !3915
  %204 = load i32, ptr %.sroa_idx6959, align 4, !dbg !3915
  %205 = load i64, ptr %_51.i.i, align 8, !dbg !3917, !alias.scope !3919, !noalias !3920, !noundef !11
  %_164.i.i.i11039.not = icmp eq i64 %plan.0.i.i, 0, !dbg !3922
  br i1 %_164.i.i.i11039.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_KBZ_EB2_.exit.i.i, label %bb54.i.i.i, !dbg !3932

bb29.i.i:                                         ; preds = %bb13.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i11228, i64 noundef %_32.i.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a05c61cb172b1016332ce1d3ce81e461) #26, !dbg !3933, !noalias !3842
  unreachable, !dbg !3933

bb54.i.i.i:                                       ; preds = %bb27.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949
  %iter.sroa.0.0.i.i.i11053 = phi i64 [ %206, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ 0, %bb27.i.i ]
  %position.sroa.0.0.i.i.i11052 = phi i64 [ %_37.sroa.0.0.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %205, %bb27.i.i ]
  %gain_far.i.i.i.sroa.6.011051 = phi i32 [ %_3.i4307, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %204, %bb27.i.i ]
  %gain_far.i.i.i.sroa.0.011050 = phi i32 [ %_3.i4311, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %203, %bb27.i.i ]
  %gain_near.i.i.i.sroa.6.011049 = phi i32 [ %_3.i4315, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %202, %bb27.i.i ]
  %gain_near.i.i.i.sroa.0.011048 = phi i32 [ %_3.i4319, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %201, %bb27.i.i ]
  %filter_far.i.i.i.sroa.14.011047 = phi float [ %_0.i4140, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %filter_far.i.i.i.sroa.14.0.copyload, %bb27.i.i ]
  %filter_far.i.i.i.sroa.11.011046 = phi float [ %_0.i4144, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %filter_far.i.i.i.sroa.11.0.copyload, %bb27.i.i ]
  %filter_far.i.i.i.sroa.7.011045 = phi float [ %_0.i4132, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %filter_far.i.i.i.sroa.7.0.copyload, %bb27.i.i ]
  %filter_far.i.i.i.sroa.0.011044 = phi float [ %_0.i4136, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %filter_far.i.i.i.sroa.0.0.copyload, %bb27.i.i ]
  %filter_near.i.i.i.sroa.0.011043 = phi float [ %_0.i4152, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %filter_near.i.i.i.sroa.0.0.copyload, %bb27.i.i ]
  %filter_near.i.i.i.sroa.7.011042 = phi float [ %_0.i4148, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %filter_near.i.i.i.sroa.7.0.copyload, %bb27.i.i ]
  %filter_near.i.i.i.sroa.11.011041 = phi float [ %_0.i4160, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %filter_near.i.i.i.sroa.11.0.copyload, %bb27.i.i ]
  %filter_near.i.i.i.sroa.14.011040 = phi float [ %_0.i4156, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], [ %filter_near.i.i.i.sroa.14.0.copyload, %bb27.i.i ]
  %206 = add nuw i64 %iter.sroa.0.0.i.i.i11053, 1, !dbg !3934
  %_38.i.i.i = add i64 %position.sroa.0.0.i.i.i11052, 1, !dbg !3940
  %_172.not.i.i.i = icmp ult i64 %_38.i.i.i, %ring_len.i.i, !dbg !3943
  %207 = select i1 %_172.not.i.i.i, i64 0, i64 %ring_len.i.i, !dbg !3943
  %_37.sroa.0.0.i.i.i = sub nuw i64 %_38.i.i.i, %207, !dbg !3943
  %_180.i.i.i = getelementptr inbounds nuw float, ptr %_95.i.i, i64 %iter.sroa.0.0.i.i.i11053, !dbg !3946
  %_0.i3540 = load float, ptr %_180.i.i.i, align 4, !dbg !3958, !alias.scope !3961, !noalias !3964, !noundef !11
  %_188.i.i.i = getelementptr inbounds nuw float, ptr %_105.i.i, i64 %iter.sroa.0.0.i.i.i11053, !dbg !3965
  %_0.i3535 = load float, ptr %_188.i.i.i, align 4, !dbg !3974, !alias.scope !3976, !noalias !3964, !noundef !11
  %_7.i28 = load float, ptr %_63.i.i.i, align 4, !dbg !3979, !alias.scope !3983, !noalias !3986, !noundef !11
  %_8.i29 = load float, ptr %168, align 4, !dbg !3988, !alias.scope !3983, !noalias !3986, !noundef !11
  %_9.i30 = load float, ptr %169, align 4, !dbg !3989, !alias.scope !3983, !noalias !3986, !noundef !11
  %_0.i3415 = fsub float %_0.i3540, %filter_near.i.i.i.sroa.7.011042, !dbg !3990
  %_0.i3230 = fmul float %_0.i3415, %_8.i29, !dbg !3997
  %_4.i2851 = fmul float %filter_near.i.i.i.sroa.0.011043, %_7.i28, !dbg !4001
  %_0.i2852 = fadd float %_4.i2851, %_0.i3230, !dbg !4001
  %_0.i2678 = fadd float %filter_near.i.i.i.sroa.0.011043, %_0.i2852, !dbg !4004
  %_0.i3229 = fmul float %filter_near.i.i.i.sroa.0.011043, %_8.i29, !dbg !4008
  %_4.i2849 = fmul float %_0.i3415, %_9.i30, !dbg !4011
  %_0.i2850 = fadd float %_0.i3229, %_4.i2849, !dbg !4011
  %_0.i2677 = fadd float %filter_near.i.i.i.sroa.7.011042, %_0.i2850, !dbg !4013
  %_0.i2676 = fadd float %_0.i2852, %_0.i2852, !dbg !4016
  %_0.i2675 = fadd float %filter_near.i.i.i.sroa.0.011043, %_0.i2676, !dbg !4019
  %208 = tail call noundef float @llvm.fabs.f32(float %_0.i2675), !dbg !4021
  %209 = fcmp uge float %208, 0x3BC79CA100000000, !dbg !4027
  %_0.i4152 = select i1 %209, float %_0.i2675, float 0.000000e+00, !dbg !4030
  %_0.i2674 = fadd float %_0.i2850, %_0.i2850, !dbg !4031
  %_0.i2673 = fadd float %filter_near.i.i.i.sroa.7.011042, %_0.i2674, !dbg !4033
  %210 = tail call noundef float @llvm.fabs.f32(float %_0.i2673), !dbg !4035
  %211 = fcmp uge float %210, 0x3BC79CA100000000, !dbg !4038
  %_0.i4148 = select i1 %211, float %_0.i2673, float 0.000000e+00, !dbg !4040
  %_12.i33 = load float, ptr %170, align 4, !dbg !4041, !alias.scope !3983, !noalias !3986, !noundef !11
  %_4.i2937 = fmul float %_12.i33, %_0.i2678, !dbg !4043
  %_0.i2938 = fadd float %_0.i3540, %_4.i2937, !dbg !4043
  %_0.i3416 = fsub float %_0.i2677, %filter_near.i.i.i.sroa.14.011040, !dbg !4045
  %_0.i3232 = fmul float %_8.i29, %_0.i3416, !dbg !4049
  %_4.i2855 = fmul float %filter_near.i.i.i.sroa.11.011041, %_7.i28, !dbg !4051
  %_0.i2856 = fadd float %_4.i2855, %_0.i3232, !dbg !4051
  %_0.i3231 = fmul float %filter_near.i.i.i.sroa.11.011041, %_8.i29, !dbg !4053
  %_4.i2853 = fmul float %_9.i30, %_0.i3416, !dbg !4055
  %_0.i2854 = fadd float %_0.i3231, %_4.i2853, !dbg !4055
  %_0.i2683 = fadd float %filter_near.i.i.i.sroa.14.011040, %_0.i2854, !dbg !4057
  %_0.i2682 = fadd float %_0.i2856, %_0.i2856, !dbg !4059
  %_0.i2681 = fadd float %filter_near.i.i.i.sroa.11.011041, %_0.i2682, !dbg !4061
  %212 = tail call noundef float @llvm.fabs.f32(float %_0.i2681), !dbg !4063
  %213 = fcmp uge float %212, 0x3BC79CA100000000, !dbg !4066
  %_0.i4160 = select i1 %213, float %_0.i2681, float 0.000000e+00, !dbg !4068
  %_0.i2680 = fadd float %_0.i2854, %_0.i2854, !dbg !4069
  %_0.i2679 = fadd float %filter_near.i.i.i.sroa.14.011040, %_0.i2680, !dbg !4071
  %214 = tail call noundef float @llvm.fabs.f32(float %_0.i2679), !dbg !4073
  %215 = fcmp uge float %214, 0x3BC79CA100000000, !dbg !4076
  %_0.i4156 = select i1 %215, float %_0.i2679, float 0.000000e+00, !dbg !4078
  %_0.i3441 = fsub float %_0.i2938, %_0.i2683, !dbg !4079
  %_7.i15 = load float, ptr %_68.i.i.i, align 4, !dbg !4082, !alias.scope !4085, !noalias !4088, !noundef !11
  %_8.i16 = load float, ptr %171, align 4, !dbg !4090, !alias.scope !4085, !noalias !4088, !noundef !11
  %_9.i17 = load float, ptr %172, align 4, !dbg !4091, !alias.scope !4085, !noalias !4088, !noundef !11
  %_0.i3413 = fsub float %_0.i3535, %filter_far.i.i.i.sroa.7.011045, !dbg !4092
  %_0.i3226 = fmul float %_0.i3413, %_8.i16, !dbg !4095
  %_4.i2843 = fmul float %filter_far.i.i.i.sroa.0.011044, %_7.i15, !dbg !4097
  %_0.i2844 = fadd float %_4.i2843, %_0.i3226, !dbg !4097
  %_0.i2666 = fadd float %filter_far.i.i.i.sroa.0.011044, %_0.i2844, !dbg !4099
  %_0.i3225 = fmul float %filter_far.i.i.i.sroa.0.011044, %_8.i16, !dbg !4101
  %_4.i2841 = fmul float %_0.i3413, %_9.i17, !dbg !4103
  %_0.i2842 = fadd float %_0.i3225, %_4.i2841, !dbg !4103
  %_0.i2665 = fadd float %filter_far.i.i.i.sroa.7.011045, %_0.i2842, !dbg !4105
  %_0.i2664 = fadd float %_0.i2844, %_0.i2844, !dbg !4107
  %_0.i2663 = fadd float %filter_far.i.i.i.sroa.0.011044, %_0.i2664, !dbg !4109
  %216 = tail call noundef float @llvm.fabs.f32(float %_0.i2663), !dbg !4111
  %217 = fcmp uge float %216, 0x3BC79CA100000000, !dbg !4114
  %_0.i4136 = select i1 %217, float %_0.i2663, float 0.000000e+00, !dbg !4116
  %_0.i2662 = fadd float %_0.i2842, %_0.i2842, !dbg !4117
  %_0.i2661 = fadd float %filter_far.i.i.i.sroa.7.011045, %_0.i2662, !dbg !4119
  %218 = tail call noundef float @llvm.fabs.f32(float %_0.i2661), !dbg !4121
  %219 = fcmp uge float %218, 0x3BC79CA100000000, !dbg !4124
  %_0.i4132 = select i1 %219, float %_0.i2661, float 0.000000e+00, !dbg !4126
  %_12.i20 = load float, ptr %173, align 4, !dbg !4127, !alias.scope !4085, !noalias !4088, !noundef !11
  %_4.i2939 = fmul float %_12.i20, %_0.i2666, !dbg !4128
  %_0.i2940 = fadd float %_0.i3535, %_4.i2939, !dbg !4128
  %_0.i3414 = fsub float %_0.i2665, %filter_far.i.i.i.sroa.14.011047, !dbg !4130
  %_0.i3228 = fmul float %_8.i16, %_0.i3414, !dbg !4133
  %_4.i2847 = fmul float %filter_far.i.i.i.sroa.11.011046, %_7.i15, !dbg !4135
  %_0.i2848 = fadd float %_4.i2847, %_0.i3228, !dbg !4135
  %_0.i3227 = fmul float %filter_far.i.i.i.sroa.11.011046, %_8.i16, !dbg !4137
  %_4.i2845 = fmul float %_9.i17, %_0.i3414, !dbg !4139
  %_0.i2846 = fadd float %_0.i3227, %_4.i2845, !dbg !4139
  %_0.i2671 = fadd float %filter_far.i.i.i.sroa.14.011047, %_0.i2846, !dbg !4141
  %_0.i2670 = fadd float %_0.i2848, %_0.i2848, !dbg !4143
  %_0.i2669 = fadd float %filter_far.i.i.i.sroa.11.011046, %_0.i2670, !dbg !4145
  %220 = tail call noundef float @llvm.fabs.f32(float %_0.i2669), !dbg !4147
  %221 = fcmp uge float %220, 0x3BC79CA100000000, !dbg !4150
  %_0.i4144 = select i1 %221, float %_0.i2669, float 0.000000e+00, !dbg !4152
  %_0.i2668 = fadd float %_0.i2846, %_0.i2846, !dbg !4153
  %_0.i2667 = fadd float %filter_far.i.i.i.sroa.14.011047, %_0.i2668, !dbg !4155
  %222 = tail call noundef float @llvm.fabs.f32(float %_0.i2667), !dbg !4157
  %223 = fcmp uge float %222, 0x3BC79CA100000000, !dbg !4160
  %_0.i4140 = select i1 %223, float %_0.i2667, float 0.000000e+00, !dbg !4162
  %_0.i3442 = fsub float %_0.i2940, %_0.i2671, !dbg !4163
  %_307.1.i.i.i = load i64, ptr %174, align 8, !dbg !4165, !noalias !3964, !noundef !11
  %_230.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i.i11052, %_307.1.i.i.i, !dbg !4167
  br i1 %_230.i.i.i, label %bb76.i.i.i, label %bb77.i.i.i, !dbg !4167, !prof !1664

bb77.i.i.i:                                       ; preds = %bb54.i.i.i
  %_307.0.i.i.i = load ptr, ptr %123, align 8, !dbg !4165, !noalias !3964, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4173), !dbg !4176
  %_4.not.i3966 = icmp eq i64 %_307.1.i.i.i, %position.sroa.0.0.i.i.i11052, !dbg !4177
  br i1 %_4.not.i3966, label %panic.i3968, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3969, !dbg !4177

panic.i3968:                                      ; preds = %bb77.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !4177, !noalias !4180
  unreachable, !dbg !4177

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3969: ; preds = %bb77.i.i.i
  %_237.i.i.i = getelementptr inbounds nuw float, ptr %_307.0.i.i.i, i64 %position.sroa.0.0.i.i.i11052, !dbg !4181
  store float %_0.i2683, ptr %_237.i.i.i, align 4, !dbg !4177, !alias.scope !4173, !noalias !3964
  %_308.1.i.i.i = load i64, ptr %176, align 8, !dbg !4187, !noalias !3964, !noundef !11
  %_238.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i.i11052, %_308.1.i.i.i, !dbg !4188
  br i1 %_238.i.i.i, label %bb78.i.i.i, label %bb79.i.i.i, !dbg !4188, !prof !1664

bb76.i.i.i:                                       ; preds = %bb54.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i.i11052, i64 noundef %_307.1.i.i.i, i64 noundef %_307.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7aeb6b0a3ba8e73c50a5abef6c30558) #26, !dbg !4192, !noalias !3964
  unreachable, !dbg !4192

bb79.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3969
  %_308.0.i.i.i = load ptr, ptr %175, align 8, !dbg !4187, !noalias !3964, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4193), !dbg !4196
  %_4.not.i3962 = icmp eq i64 %_308.1.i.i.i, %position.sroa.0.0.i.i.i11052, !dbg !4197
  br i1 %_4.not.i3962, label %panic.i3964, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3965, !dbg !4197

panic.i3964:                                      ; preds = %bb79.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !4197, !noalias !4199
  unreachable, !dbg !4197

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3965: ; preds = %bb79.i.i.i
  %_245.i.i.i = getelementptr inbounds nuw float, ptr %_308.0.i.i.i, i64 %position.sroa.0.0.i.i.i11052, !dbg !4200
  store float %_0.i3441, ptr %_245.i.i.i, align 4, !dbg !4197, !alias.scope !4193, !noalias !3964
  %_309.1.i.i.i = load i64, ptr %177, align 8, !dbg !4205, !noalias !3964, !noundef !11
  %_246.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i.i11052, %_309.1.i.i.i, !dbg !4206
  br i1 %_246.i.i.i, label %bb80.i.i.i, label %bb81.i.i.i, !dbg !4206, !prof !1664

bb78.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3969
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i.i11052, i64 noundef %_308.1.i.i.i, i64 noundef %_308.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8e669d1bed0fb747f8a7bff920a6571) #26, !dbg !4210, !noalias !3964
  unreachable, !dbg !4210

bb81.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3965
  %_309.0.i.i.i = load ptr, ptr %_18.i.i, align 8, !dbg !4205, !noalias !3964, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4211), !dbg !4214
  %_4.not.i3958 = icmp eq i64 %_309.1.i.i.i, %position.sroa.0.0.i.i.i11052, !dbg !4215
  br i1 %_4.not.i3958, label %panic.i3960, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3961, !dbg !4215

panic.i3960:                                      ; preds = %bb81.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !4215, !noalias !4217
  unreachable, !dbg !4215

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3961: ; preds = %bb81.i.i.i
  %_253.i.i.i = getelementptr inbounds nuw float, ptr %_309.0.i.i.i, i64 %position.sroa.0.0.i.i.i11052, !dbg !4218
  store float %_0.i2671, ptr %_253.i.i.i, align 4, !dbg !4215, !alias.scope !4211, !noalias !3964
  %_310.1.i.i.i = load i64, ptr %179, align 8, !dbg !4223, !noalias !3964, !noundef !11
  %_254.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i.i11052, %_310.1.i.i.i, !dbg !4224
  br i1 %_254.i.i.i, label %bb82.i.i.i, label %bb83.i.i.i, !dbg !4224, !prof !1664

bb80.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3965
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i.i11052, i64 noundef %_309.1.i.i.i, i64 noundef %_309.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e81c2bc19b75441ce2fb90ce8f9eb70) #26, !dbg !4228, !noalias !3964
  unreachable, !dbg !4228

bb83.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3961
  %_310.0.i.i.i = load ptr, ptr %178, align 8, !dbg !4223, !noalias !3964, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4229), !dbg !4232
  %_4.not.i3954 = icmp eq i64 %_310.1.i.i.i, %position.sroa.0.0.i.i.i11052, !dbg !4233
  br i1 %_4.not.i3954, label %panic.i3956, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3957, !dbg !4233

panic.i3956:                                      ; preds = %bb83.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !4233, !noalias !4235
  unreachable, !dbg !4233

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3957: ; preds = %bb83.i.i.i
  %_261.i.i.i = getelementptr inbounds nuw float, ptr %_310.0.i.i.i, i64 %position.sroa.0.0.i.i.i11052, !dbg !4236
  store float %_0.i3442, ptr %_261.i.i.i, align 4, !dbg !4233, !alias.scope !4229, !noalias !3964
  %_311.0.i.i.i = load ptr, ptr %123, align 8, !dbg !4241, !noalias !3964, !nonnull !11, !noundef !11
  %_311.1.i.i.i = load i64, ptr %174, align 8, !dbg !4241, !noalias !3964, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4242), !dbg !4245
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4246), !dbg !4245
  %_13.i267.i.i = load i64, ptr %_85.i.i.i, align 8, !alias.scope !4246, !noalias !4248, !noundef !11
  %_12.i268.i.i = add i64 %_13.i267.i.i, %position.sroa.0.0.i.i.i11052
  %_31.not.i269.i.i = icmp ult i64 %_12.i268.i.i, %ring_len.i.i
  %224 = select i1 %_31.not.i269.i.i, i64 0, i64 %ring_len.i.i
  %_11.sroa.0.0.i270.i.i = sub nuw i64 %_12.i268.i.i, %224
  %_16.i271.i.i = icmp ult i64 %_11.sroa.0.0.i270.i.i, %_311.1.i.i.i
  br i1 %_16.i271.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i.i.split.us, label %panic1.i272.i.i

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i.i.split.us: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3957
  %225 = getelementptr inbounds nuw float, ptr %_311.0.i.i.i, i64 %_11.sroa.0.0.i270.i.i
  %_8.i274.i.i.us.le = load float, ptr %225, align 4, !alias.scope !4242, !noalias !4249, !noundef !11
  %_312.0.i.i.i = load ptr, ptr %175, align 8, !dbg !4250, !noalias !3964, !nonnull !11, !noundef !11
  %_312.1.i.i.i = load i64, ptr %176, align 8, !dbg !4250, !noalias !3964, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4252), !dbg !4255
  %_16.i254.i.i = icmp ult i64 %_11.sroa.0.0.i270.i.i, %_312.1.i.i.i
  br i1 %_16.i254.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i.i.split.us, label %panic1.i255.i.i

panic1.i272.i.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3957
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i270.i.i, i64 noundef range(i64 0, 2305843009213693952) %_311.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !4256, !noalias !4262
  unreachable, !dbg !4256

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i.i.split.us
  %226 = getelementptr inbounds nuw float, ptr %_312.0.i.i.i, i64 %_11.sroa.0.0.i270.i.i
  %_8.i257.i.i.us.le = load float, ptr %226, align 4, !alias.scope !4252, !noalias !4263, !noundef !11
  %_313.0.i.i.i = load ptr, ptr %_18.i.i, align 8, !dbg !4265, !noalias !3964, !nonnull !11, !noundef !11
  %_313.1.i.i.i = load i64, ptr %177, align 8, !dbg !4265, !noalias !3964, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4267), !dbg !4270
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4271), !dbg !4270
  %_13.i233.i.i = load i64, ptr %_93.i.i.i, align 8, !alias.scope !4271, !noalias !4273, !noundef !11
  %_12.i234.i.i = add i64 %_13.i233.i.i, %position.sroa.0.0.i.i.i11052
  %_31.not.i235.i.i = icmp ult i64 %_12.i234.i.i, %ring_len.i.i
  %227 = select i1 %_31.not.i235.i.i, i64 0, i64 %ring_len.i.i
  %_11.sroa.0.0.i236.i.i = sub nuw i64 %_12.i234.i.i, %227
  %_16.i237.i.i = icmp ult i64 %_11.sroa.0.0.i236.i.i, %_313.1.i.i.i
  br i1 %_16.i237.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i.i.split.us, label %panic1.i238.i.i

panic1.i255.i.i:                                  ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i270.i.i, i64 noundef range(i64 0, 2305843009213693952) %_312.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !4274, !noalias !4276
  unreachable, !dbg !4274

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i.i.split.us
  %228 = getelementptr inbounds nuw float, ptr %_313.0.i.i.i, i64 %_11.sroa.0.0.i236.i.i
  %_8.i240.i.i.us.le = load float, ptr %228, align 4, !alias.scope !4267, !noalias !4277, !noundef !11
  %_314.0.i.i.i = load ptr, ptr %178, align 8, !dbg !4278, !noalias !3964, !nonnull !11, !noundef !11
  %_314.1.i.i.i = load i64, ptr %179, align 8, !dbg !4278, !noalias !3964, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4280), !dbg !4283
  %_16.i220.i.i = icmp ult i64 %_11.sroa.0.0.i236.i.i, %_314.1.i.i.i
  br i1 %_16.i220.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i.i.split.us, label %panic1.i221.i.i

panic1.i238.i.i:                                  ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i236.i.i, i64 noundef range(i64 0, 2305843009213693952) %_313.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !4284, !noalias !4286
  unreachable, !dbg !4284

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i.i.split.us
  %229 = getelementptr inbounds nuw float, ptr %_314.0.i.i.i, i64 %_11.sroa.0.0.i236.i.i
  %_8.i223.i.i.us.le = load float, ptr %229, align 4, !alias.scope !4280, !noalias !4287, !noundef !11
  %230 = tail call noundef float @llvm.fabs.f32(float %_8.i274.i.i.us.le), !dbg !4289
  %_3.i.i5462 = fcmp ule float %230, 0x3E45798EE0000000, !dbg !4296
  %_6.i.i5464 = bitcast float %230 to i32, !dbg !4306
  %_4.i.i5468 = select i1 %_3.i.i5462, i32 841731191, i32 %_6.i.i5464, !dbg !4311
  %_0.i.i5469 = bitcast i32 %_4.i.i5468 to float, !dbg !4312
  %_3.i.i4986 = fcmp ule float %_0.i.i5469, 0x3810000000000000, !dbg !4315
  %_4.i.i4992 = select i1 %_3.i.i4986, i32 8388608, i32 %_4.i.i5468, !dbg !4324
  %_5.i3861 = and i32 %_4.i.i4992, 8388607, !dbg !4326
  %_4.i3862 = or disjoint i32 %_5.i3861, 1065353216, !dbg !4326
  %significand.i3863 = bitcast i32 %_4.i3862 to float, !dbg !4331
  %_0.i3353 = fadd float %significand.i3863, -1.000000e+00, !dbg !4334
  %_0.i3026 = fmul float %_0.i3353, 0xBF9B17A960000000, !dbg !4337
  %_0.i2546 = fadd float %_0.i3026, 0x3FBF9A8440000000, !dbg !4342
  %_0.i3026.1 = fmul float %_0.i3353, %_0.i2546, !dbg !4337
  %_0.i2546.1 = fadd float %_0.i3026.1, 0xBFD1E3F400000000, !dbg !4342
  %_0.i3026.2 = fmul float %_0.i3353, %_0.i2546.1, !dbg !4337
  %_0.i2546.2 = fadd float %_0.i3026.2, 0x3FDD544F20000000, !dbg !4342
  %_0.i3026.3 = fmul float %_0.i3353, %_0.i2546.2, !dbg !4337
  %_0.i2546.3 = fadd float %_0.i3026.3, 0xBFE6FC2A60000000, !dbg !4342
  %_0.i3026.4 = fmul float %_0.i3353, %_0.i2546.3, !dbg !4337
  %_0.i2546.4 = fadd float %_0.i3026.4, 0x3FF714B2A0000000, !dbg !4342
  %_9.i3864 = lshr i32 %_4.i.i4992, 23, !dbg !4344
  %_8.i3865 = or disjoint i32 %_9.i3864, 1258291200, !dbg !4344
  %_7.i3866 = bitcast i32 %_8.i3865 to float, !dbg !4346
  %exponent.i3867 = fadd float %_7.i3866, 0xC160000FE0000000, !dbg !4348
  %_0.i3025 = fmul float %_0.i3353, %_0.i2546.4, !dbg !4349
  %_0.i2545 = fadd float %exponent.i3867, %_0.i3025, !dbg !4351
  %_0.i3288 = fmul float %_0.i2545, 0x4018151820000000, !dbg !4353
  %_3.i.i5454.inv = fcmp ogt float %_0.i3288, -1.600000e+02, !dbg !4355
  %_0.i.i5461 = select i1 %_3.i.i5454.inv, float %_0.i3288, float -1.600000e+02, !dbg !4355
  %_3.i.i6164.inv = fcmp olt float %_0.i.i5461, 2.400000e+01, !dbg !4358
  %_0.i.i6171 = select i1 %_3.i.i6164.inv, float %_0.i.i5461, float 2.400000e+01, !dbg !4358
  %_0.i3401 = fsub float %_0.i.i6171, %_12.le.i, !dbg !4362
  %_3.i2257 = fcmp ule float %_0.i3401, 3.000000e+00, !dbg !4370
  %_0.i2625 = fadd float %_0.i3401, 3.000000e+00, !dbg !4373
  %_0.i3179 = fmul float %_0.i2625, %_0.i2625, !dbg !4378
  %_0.i3178 = fmul float %_0.i3179, 0x3FB5555560000000, !dbg !4381
  %_4.i4604.v.v = select i1 %_3.i2257, float %_0.i3178, float %_0.i3401, !dbg !4383
  %_4.i4604.v = fmul float %coefficients.i.i.sroa.0.0.copyload, %_4.i4604.v.v, !dbg !4383
  %_4.i4604 = bitcast float %_4.i4604.v to i32, !dbg !4383
  %231 = fcmp ugt float %_0.i3401, -3.000000e+00, !dbg !4386
  %_7.i4596 = select i1 %231, i32 %_4.i4604, i32 0, !dbg !4388
  %_0.i4598 = bitcast i32 %_7.i4596 to float, !dbg !4389
  %_3.i.i5446 = fcmp ule float %_0.i4598, -1.000000e+02, !dbg !4391
  %232 = bitcast i32 %_7.i4596 to float, !dbg !4394
  %_0.i.i5453 = select i1 %_3.i.i5446, float -1.000000e+02, float %232, !dbg !4397
  %_3.i.i6156 = fcmp olt float %_0.i.i5453, 0.000000e+00, !dbg !4398
  %_0.i.i6163 = select i1 %_3.i.i6156, float %_0.i.i5453, float 0.000000e+00, !dbg !4402
  %233 = bitcast i32 %gain_near.i.i.i.sroa.0.011048 to float, !dbg !4404
  %_3.i2447 = fcmp uge float %_0.i.i6163, %233, !dbg !4406
  %_4.i4750.v = select i1 %_3.i2447, float %coefficients.i.i.sroa.7.0.copyload, float %coefficients.i.i.sroa.5.0.copyload, !dbg !4410
  %_0.i3452 = fsub float %233, %_0.i.i6163, !dbg !4412
  %_4.i2959 = fmul float %_0.i3452, %_4.i4750.v, !dbg !4415
  %_0.i2960 = fadd float %_0.i.i6163, %_4.i2959, !dbg !4415
  %234 = tail call noundef float @llvm.fabs.f32(float %_0.i2960), !dbg !4418
  %_4.i4317 = bitcast float %_0.i2960 to i32, !dbg !4422
  %235 = fcmp uge float %234, 0x3BC79CA100000000, !dbg !4426
  %_3.i4319 = select i1 %235, i32 %_4.i4317, i32 0, !dbg !4427
  %_0.i4320 = bitcast i32 %_3.i4319 to float, !dbg !4428
  %_0.i2794 = fadd float %_12.le.4.i, %_0.i4320, !dbg !4431
  %_0.i3287 = fmul float %_0.i2794, 0x3FC542A5A0000000, !dbg !4434
  %_3.i.i5178.inv = fcmp ogt float %_0.i3287, -1.260000e+02, !dbg !4438
  %_0.i.i5185 = select i1 %_3.i.i5178.inv, float %_0.i3287, float -1.260000e+02, !dbg !4438
  %_3.i.i5980.inv = fcmp olt float %_0.i.i5185, 1.270000e+02, !dbg !4443
  %_0.i.i5987 = select i1 %_3.i.i5980.inv, float %_0.i.i5185, float 1.270000e+02, !dbg !4443
  %236 = tail call noundef float @llvm.floor.f32(float %_0.i.i5987), !dbg !4446
  %_0.i3377 = fsub float %_0.i.i5987, %236, !dbg !4459
  %237 = tail call noundef float @llvm.fabs.f32(float %_8.i257.i.i.us.le), !dbg !4462
  %_3.i.i5438 = fcmp ule float %237, 0x3E45798EE0000000, !dbg !4465
  %_6.i.i5440 = bitcast float %237 to i32, !dbg !4470
  %_4.i.i5444 = select i1 %_3.i.i5438, i32 841731191, i32 %_6.i.i5440, !dbg !4473
  %_0.i.i5445 = bitcast i32 %_4.i.i5444 to float, !dbg !4474
  %_3.i.i4994 = fcmp ule float %_0.i.i5445, 0x3810000000000000, !dbg !4476
  %_4.i.i5000 = select i1 %_3.i.i4994, i32 8388608, i32 %_4.i.i5444, !dbg !4481
  %_5.i3869 = and i32 %_4.i.i5000, 8388607, !dbg !4483
  %_4.i3870 = or disjoint i32 %_5.i3869, 1065353216, !dbg !4483
  %significand.i3871 = bitcast i32 %_4.i3870 to float, !dbg !4485
  %_0.i3354 = fadd float %significand.i3871, -1.000000e+00, !dbg !4487
  %_0.i3028 = fmul float %_0.i3354, 0xBF9B17A960000000, !dbg !4489
  %_0.i2548 = fadd float %_0.i3028, 0x3FBF9A8440000000, !dbg !4491
  %_0.i3028.1 = fmul float %_0.i3354, %_0.i2548, !dbg !4489
  %_0.i2548.1 = fadd float %_0.i3028.1, 0xBFD1E3F400000000, !dbg !4491
  %_0.i3028.2 = fmul float %_0.i3354, %_0.i2548.1, !dbg !4489
  %_0.i2548.2 = fadd float %_0.i3028.2, 0x3FDD544F20000000, !dbg !4491
  %_0.i3028.3 = fmul float %_0.i3354, %_0.i2548.2, !dbg !4489
  %_0.i2548.3 = fadd float %_0.i3028.3, 0xBFE6FC2A60000000, !dbg !4491
  %_0.i3028.4 = fmul float %_0.i3354, %_0.i2548.3, !dbg !4489
  %_0.i2548.4 = fadd float %_0.i3028.4, 0x3FF714B2A0000000, !dbg !4491
  %_9.i3872 = lshr i32 %_4.i.i5000, 23, !dbg !4493
  %_8.i3873 = or disjoint i32 %_9.i3872, 1258291200, !dbg !4493
  %_7.i3874 = bitcast i32 %_8.i3873 to float, !dbg !4494
  %exponent.i3875 = fadd float %_7.i3874, 0xC160000FE0000000, !dbg !4496
  %_0.i3027 = fmul float %_0.i3354, %_0.i2548.4, !dbg !4497
  %_0.i2547 = fadd float %exponent.i3875, %_0.i3027, !dbg !4499
  %_0.i3286 = fmul float %_0.i2547, 0x4018151820000000, !dbg !4501
  %_3.i.i5430.inv = fcmp ogt float %_0.i3286, -1.600000e+02, !dbg !4503
  %_0.i.i5437 = select i1 %_3.i.i5430.inv, float %_0.i3286, float -1.600000e+02, !dbg !4503
  %_3.i.i6148.inv = fcmp olt float %_0.i.i5437, 2.400000e+01, !dbg !4506
  %_0.i.i6155 = select i1 %_3.i.i6148.inv, float %_0.i.i5437, float 2.400000e+01, !dbg !4506
  %_0.i3402 = fsub float %_0.i.i6155, %_12.le.5.i, !dbg !4509
  %_3.i2259 = fcmp ule float %_0.i3402, 3.000000e+00, !dbg !4512
  %_0.i2626 = fadd float %_0.i3402, 3.000000e+00, !dbg !4514
  %_0.i3183 = fmul float %_0.i2626, %_0.i2626, !dbg !4516
  %_0.i3182 = fmul float %_0.i3183, 0x3FB5555560000000, !dbg !4518
  %_4.i4617.v.v = select i1 %_3.i2259, float %_0.i3182, float %_0.i3402, !dbg !4520
  %_4.i4617.v = fmul float %coefficients.i.i.sroa.9.0.copyload, %_4.i4617.v.v, !dbg !4520
  %_4.i4617 = bitcast float %_4.i4617.v to i32, !dbg !4520
  %238 = fcmp ugt float %_0.i3402, -3.000000e+00, !dbg !4522
  %_7.i4609 = select i1 %238, i32 %_4.i4617, i32 0, !dbg !4524
  %_0.i4611 = bitcast i32 %_7.i4609 to float, !dbg !4525
  %_3.i.i5422 = fcmp ule float %_0.i4611, -1.000000e+02, !dbg !4527
  %239 = bitcast i32 %_7.i4609 to float, !dbg !4530
  %_0.i.i5429 = select i1 %_3.i.i5422, float -1.000000e+02, float %239, !dbg !4533
  %_3.i.i6140 = fcmp olt float %_0.i.i5429, 0.000000e+00, !dbg !4534
  %_0.i.i6147 = select i1 %_3.i.i6140, float %_0.i.i5429, float 0.000000e+00, !dbg !4537
  %240 = bitcast i32 %gain_near.i.i.i.sroa.6.011049 to float, !dbg !4539
  %_3.i2443 = fcmp uge float %_0.i.i6147, %240, !dbg !4540
  %_4.i4743.v = select i1 %_3.i2443, float %coefficients.i.i.sroa.13.0.copyload, float %coefficients.i.i.sroa.11.0.copyload, !dbg !4543
  %_0.i3451 = fsub float %240, %_0.i.i6147, !dbg !4545
  %_4.i2957 = fmul float %_0.i3451, %_4.i4743.v, !dbg !4547
  %_0.i2958 = fadd float %_0.i.i6147, %_4.i2957, !dbg !4547
  %241 = tail call noundef float @llvm.fabs.f32(float %_0.i2958), !dbg !4549
  %_4.i4313 = bitcast float %_0.i2958 to i32, !dbg !4552
  %242 = fcmp uge float %241, 0x3BC79CA100000000, !dbg !4555
  %_3.i4315 = select i1 %242, i32 %_4.i4313, i32 0, !dbg !4556
  %_0.i4316 = bitcast i32 %_3.i4315 to float, !dbg !4557
  %_0.i2793 = fadd float %_12.le.9.i, %_0.i4316, !dbg !4559
  %_0.i3285 = fmul float %_0.i2793, 0x3FC542A5A0000000, !dbg !4561
  %_3.i.i5186.inv = fcmp ogt float %_0.i3285, -1.260000e+02, !dbg !4564
  %_0.i.i5193 = select i1 %_3.i.i5186.inv, float %_0.i3285, float -1.260000e+02, !dbg !4564
  %_3.i.i5988.inv = fcmp olt float %_0.i.i5193, 1.270000e+02, !dbg !4568
  %_0.i.i5995 = select i1 %_3.i.i5988.inv, float %_0.i.i5193, float 1.270000e+02, !dbg !4568
  %243 = tail call noundef float @llvm.floor.f32(float %_0.i.i5995), !dbg !4571
  %_0.i3378 = fsub float %_0.i.i5995, %243, !dbg !4575
  %244 = tail call noundef float @llvm.fabs.f32(float %_8.i240.i.i.us.le), !dbg !4577
  %_3.i.i5414 = fcmp ule float %244, 0x3E45798EE0000000, !dbg !4579
  %_6.i.i5416 = bitcast float %244 to i32, !dbg !4584
  %_4.i.i5420 = select i1 %_3.i.i5414, i32 841731191, i32 %_6.i.i5416, !dbg !4587
  %_0.i.i5421 = bitcast i32 %_4.i.i5420 to float, !dbg !4588
  %_3.i.i5002 = fcmp ule float %_0.i.i5421, 0x3810000000000000, !dbg !4590
  %_4.i.i5008 = select i1 %_3.i.i5002, i32 8388608, i32 %_4.i.i5420, !dbg !4595
  %_5.i3877 = and i32 %_4.i.i5008, 8388607, !dbg !4597
  %_4.i3878 = or disjoint i32 %_5.i3877, 1065353216, !dbg !4597
  %significand.i3879 = bitcast i32 %_4.i3878 to float, !dbg !4599
  %_0.i3355 = fadd float %significand.i3879, -1.000000e+00, !dbg !4601
  %_0.i3030 = fmul float %_0.i3355, 0xBF9B17A960000000, !dbg !4603
  %_0.i2550 = fadd float %_0.i3030, 0x3FBF9A8440000000, !dbg !4605
  %_0.i3030.1 = fmul float %_0.i3355, %_0.i2550, !dbg !4603
  %_0.i2550.1 = fadd float %_0.i3030.1, 0xBFD1E3F400000000, !dbg !4605
  %_0.i3030.2 = fmul float %_0.i3355, %_0.i2550.1, !dbg !4603
  %_0.i2550.2 = fadd float %_0.i3030.2, 0x3FDD544F20000000, !dbg !4605
  %_0.i3030.3 = fmul float %_0.i3355, %_0.i2550.2, !dbg !4603
  %_0.i2550.3 = fadd float %_0.i3030.3, 0xBFE6FC2A60000000, !dbg !4605
  %_0.i3030.4 = fmul float %_0.i3355, %_0.i2550.3, !dbg !4603
  %_0.i2550.4 = fadd float %_0.i3030.4, 0x3FF714B2A0000000, !dbg !4605
  %_9.i3880 = lshr i32 %_4.i.i5008, 23, !dbg !4607
  %_8.i3881 = or disjoint i32 %_9.i3880, 1258291200, !dbg !4607
  %_7.i3882 = bitcast i32 %_8.i3881 to float, !dbg !4608
  %exponent.i3883 = fadd float %_7.i3882, 0xC160000FE0000000, !dbg !4610
  %_0.i3029 = fmul float %_0.i3355, %_0.i2550.4, !dbg !4611
  %_0.i2549 = fadd float %exponent.i3883, %_0.i3029, !dbg !4613
  %_0.i3284 = fmul float %_0.i2549, 0x4018151820000000, !dbg !4615
  %_3.i.i5406.inv = fcmp ogt float %_0.i3284, -1.600000e+02, !dbg !4617
  %_0.i.i5413 = select i1 %_3.i.i5406.inv, float %_0.i3284, float -1.600000e+02, !dbg !4617
  %_3.i.i6132.inv = fcmp olt float %_0.i.i5413, 2.400000e+01, !dbg !4620
  %_0.i.i6139 = select i1 %_3.i.i6132.inv, float %_0.i.i5413, float 2.400000e+01, !dbg !4620
  %_0.i3403 = fsub float %_0.i.i6139, %_12.le.i6433, !dbg !4623
  %_3.i2261 = fcmp ule float %_0.i3403, 3.000000e+00, !dbg !4626
  %_0.i2627 = fadd float %_0.i3403, 3.000000e+00, !dbg !4628
  %_0.i3187 = fmul float %_0.i2627, %_0.i2627, !dbg !4630
  %_0.i3186 = fmul float %_0.i3187, 0x3FB5555560000000, !dbg !4632
  %_4.i4630.v.v = select i1 %_3.i2261, float %_0.i3186, float %_0.i3403, !dbg !4634
  %_4.i4630.v = fmul float %coefficients.i.i.sroa.15.24.copyload, %_4.i4630.v.v, !dbg !4634
  %_4.i4630 = bitcast float %_4.i4630.v to i32, !dbg !4634
  %245 = fcmp ugt float %_0.i3403, -3.000000e+00, !dbg !4636
  %_7.i4622 = select i1 %245, i32 %_4.i4630, i32 0, !dbg !4638
  %_0.i4624 = bitcast i32 %_7.i4622 to float, !dbg !4639
  %_3.i.i5398 = fcmp ule float %_0.i4624, -1.000000e+02, !dbg !4641
  %246 = bitcast i32 %_7.i4622 to float, !dbg !4644
  %_0.i.i5405 = select i1 %_3.i.i5398, float -1.000000e+02, float %246, !dbg !4647
  %_3.i.i6124 = fcmp olt float %_0.i.i5405, 0.000000e+00, !dbg !4648
  %_0.i.i6131 = select i1 %_3.i.i6124, float %_0.i.i5405, float 0.000000e+00, !dbg !4651
  %247 = bitcast i32 %gain_far.i.i.i.sroa.0.011050 to float, !dbg !4653
  %_3.i2439 = fcmp uge float %_0.i.i6131, %247, !dbg !4654
  %_4.i4736.v = select i1 %_3.i2439, float %coefficients.i.i.sroa.20.24.copyload, float %coefficients.i.i.sroa.18.24.copyload, !dbg !4657
  %_0.i3450 = fsub float %247, %_0.i.i6131, !dbg !4659
  %_4.i2955 = fmul float %_0.i3450, %_4.i4736.v, !dbg !4661
  %_0.i2956 = fadd float %_0.i.i6131, %_4.i2955, !dbg !4661
  %248 = tail call noundef float @llvm.fabs.f32(float %_0.i2956), !dbg !4663
  %_4.i4309 = bitcast float %_0.i2956 to i32, !dbg !4666
  %249 = fcmp uge float %248, 0x3BC79CA100000000, !dbg !4669
  %_3.i4311 = select i1 %249, i32 %_4.i4309, i32 0, !dbg !4670
  %_0.i4312 = bitcast i32 %_3.i4311 to float, !dbg !4671
  %_0.i2792 = fadd float %_12.le.4.i6441, %_0.i4312, !dbg !4673
  %_0.i3283 = fmul float %_0.i2792, 0x3FC542A5A0000000, !dbg !4675
  %_3.i.i5194.inv = fcmp ogt float %_0.i3283, -1.260000e+02, !dbg !4678
  %_0.i.i5201 = select i1 %_3.i.i5194.inv, float %_0.i3283, float -1.260000e+02, !dbg !4678
  %_3.i.i5996.inv = fcmp olt float %_0.i.i5201, 1.270000e+02, !dbg !4682
  %_0.i.i6003 = select i1 %_3.i.i5996.inv, float %_0.i.i5201, float 1.270000e+02, !dbg !4682
  %250 = tail call noundef float @llvm.floor.f32(float %_0.i.i6003), !dbg !4685
  %_0.i3379 = fsub float %_0.i.i6003, %250, !dbg !4689
  %251 = tail call noundef float @llvm.fabs.f32(float %_8.i223.i.i.us.le), !dbg !4691
  %_3.i.i5390 = fcmp ule float %251, 0x3E45798EE0000000, !dbg !4693
  %_6.i.i5392 = bitcast float %251 to i32, !dbg !4698
  %_4.i.i5396 = select i1 %_3.i.i5390, i32 841731191, i32 %_6.i.i5392, !dbg !4701
  %_0.i.i5397 = bitcast i32 %_4.i.i5396 to float, !dbg !4702
  %_3.i.i5010 = fcmp ule float %_0.i.i5397, 0x3810000000000000, !dbg !4704
  %_4.i.i5016 = select i1 %_3.i.i5010, i32 8388608, i32 %_4.i.i5396, !dbg !4709
  %_5.i3885 = and i32 %_4.i.i5016, 8388607, !dbg !4711
  %_4.i3886 = or disjoint i32 %_5.i3885, 1065353216, !dbg !4711
  %significand.i3887 = bitcast i32 %_4.i3886 to float, !dbg !4713
  %_0.i3356 = fadd float %significand.i3887, -1.000000e+00, !dbg !4715
  %_0.i3032 = fmul float %_0.i3356, 0xBF9B17A960000000, !dbg !4717
  %_0.i2552 = fadd float %_0.i3032, 0x3FBF9A8440000000, !dbg !4719
  %_0.i3032.1 = fmul float %_0.i3356, %_0.i2552, !dbg !4717
  %_0.i2552.1 = fadd float %_0.i3032.1, 0xBFD1E3F400000000, !dbg !4719
  %_0.i3032.2 = fmul float %_0.i3356, %_0.i2552.1, !dbg !4717
  %_0.i2552.2 = fadd float %_0.i3032.2, 0x3FDD544F20000000, !dbg !4719
  %_0.i3032.3 = fmul float %_0.i3356, %_0.i2552.2, !dbg !4717
  %_0.i2552.3 = fadd float %_0.i3032.3, 0xBFE6FC2A60000000, !dbg !4719
  %_0.i3032.4 = fmul float %_0.i3356, %_0.i2552.3, !dbg !4717
  %_0.i2552.4 = fadd float %_0.i3032.4, 0x3FF714B2A0000000, !dbg !4719
  %_9.i3888 = lshr i32 %_4.i.i5016, 23, !dbg !4721
  %_8.i3889 = or disjoint i32 %_9.i3888, 1258291200, !dbg !4721
  %_7.i3890 = bitcast i32 %_8.i3889 to float, !dbg !4722
  %exponent.i3891 = fadd float %_7.i3890, 0xC160000FE0000000, !dbg !4724
  %_0.i3031 = fmul float %_0.i3356, %_0.i2552.4, !dbg !4725
  %_0.i2551 = fadd float %exponent.i3891, %_0.i3031, !dbg !4727
  %_0.i3282 = fmul float %_0.i2551, 0x4018151820000000, !dbg !4729
  %_3.i.i5382.inv = fcmp ogt float %_0.i3282, -1.600000e+02, !dbg !4731
  %_0.i.i5389 = select i1 %_3.i.i5382.inv, float %_0.i3282, float -1.600000e+02, !dbg !4731
  %_3.i.i6116.inv = fcmp olt float %_0.i.i5389, 2.400000e+01, !dbg !4734
  %_0.i.i6123 = select i1 %_3.i.i6116.inv, float %_0.i.i5389, float 2.400000e+01, !dbg !4734
  %_0.i3404 = fsub float %_0.i.i6123, %_12.le.5.i6443, !dbg !4737
  %_3.i2263 = fcmp ule float %_0.i3404, 3.000000e+00, !dbg !4740
  %_0.i2628 = fadd float %_0.i3404, 3.000000e+00, !dbg !4742
  %_0.i3191 = fmul float %_0.i2628, %_0.i2628, !dbg !4744
  %_0.i3190 = fmul float %_0.i3191, 0x3FB5555560000000, !dbg !4746
  %_4.i4643.v.v = select i1 %_3.i2263, float %_0.i3190, float %_0.i3404, !dbg !4748
  %_4.i4643.v = fmul float %coefficients.i.i.sroa.22.24.copyload, %_4.i4643.v.v, !dbg !4748
  %_4.i4643 = bitcast float %_4.i4643.v to i32, !dbg !4748
  %252 = fcmp ugt float %_0.i3404, -3.000000e+00, !dbg !4750
  %_7.i4635 = select i1 %252, i32 %_4.i4643, i32 0, !dbg !4752
  %_0.i4637 = bitcast i32 %_7.i4635 to float, !dbg !4753
  %_3.i.i5374 = fcmp ule float %_0.i4637, -1.000000e+02, !dbg !4755
  %253 = bitcast i32 %_7.i4635 to float, !dbg !4758
  %_0.i.i5381 = select i1 %_3.i.i5374, float -1.000000e+02, float %253, !dbg !4761
  %_3.i.i6108 = fcmp olt float %_0.i.i5381, 0.000000e+00, !dbg !4762
  %_0.i.i6115 = select i1 %_3.i.i6108, float %_0.i.i5381, float 0.000000e+00, !dbg !4765
  %254 = bitcast i32 %gain_far.i.i.i.sroa.6.011051 to float, !dbg !4767
  %_3.i2435 = fcmp uge float %_0.i.i6115, %254, !dbg !4768
  %_4.i4729.v = select i1 %_3.i2435, float %coefficients.i.i.sroa.26.24.copyload, float %coefficients.i.i.sroa.24.24.copyload, !dbg !4771
  %_0.i3449 = fsub float %254, %_0.i.i6115, !dbg !4773
  %_4.i2953 = fmul float %_0.i3449, %_4.i4729.v, !dbg !4775
  %_0.i2954 = fadd float %_0.i.i6115, %_4.i2953, !dbg !4775
  %255 = tail call noundef float @llvm.fabs.f32(float %_0.i2954), !dbg !4777
  %_4.i4305 = bitcast float %_0.i2954 to i32, !dbg !4780
  %256 = fcmp uge float %255, 0x3BC79CA100000000, !dbg !4783
  %_3.i4307 = select i1 %256, i32 %_4.i4305, i32 0, !dbg !4784
  %_0.i4308 = bitcast i32 %_3.i4307 to float, !dbg !4785
  %_0.i2791 = fadd float %_12.le.9.i6451, %_0.i4308, !dbg !4787
  %_0.i3281 = fmul float %_0.i2791, 0x3FC542A5A0000000, !dbg !4789
  %_3.i.i5202.inv = fcmp ogt float %_0.i3281, -1.260000e+02, !dbg !4792
  %_0.i.i5209 = select i1 %_3.i.i5202.inv, float %_0.i3281, float -1.260000e+02, !dbg !4792
  %_3.i.i6004.inv = fcmp olt float %_0.i.i5209, 1.270000e+02, !dbg !4796
  %_0.i.i6011 = select i1 %_3.i.i6004.inv, float %_0.i.i5209, float 1.270000e+02, !dbg !4796
  %257 = tail call noundef float @llvm.floor.f32(float %_0.i.i6011), !dbg !4799
  %_0.i3380 = fsub float %_0.i.i6011, %257, !dbg !4803
  %_0.i3100 = fmul float %_0.i3380, 0x3F5E974FA0000000, !dbg !4805
  %_0.i2600 = fadd float %_0.i3100, 0x3F82778560000000, !dbg !4810
  %_0.i3100.1 = fmul float %_0.i3380, %_0.i2600, !dbg !4805
  %_0.i2600.1 = fadd float %_0.i3100.1, 0x3FAC91CE60000000, !dbg !4810
  %_0.i3100.2 = fmul float %_0.i3380, %_0.i2600.1, !dbg !4805
  %_0.i2600.2 = fadd float %_0.i3100.2, 0x3FCEBDB560000000, !dbg !4810
  %_0.i3100.3 = fmul float %_0.i3380, %_0.i2600.2, !dbg !4805
  %_0.i2600.3 = fadd float %_0.i3100.3, 0x3FE62E4BA0000000, !dbg !4810
  %_0.i3097 = fmul float %_0.i3379, 0x3F5E974FA0000000, !dbg !4812
  %_0.i2598 = fadd float %_0.i3097, 0x3F82778560000000, !dbg !4814
  %_0.i3097.1 = fmul float %_0.i3379, %_0.i2598, !dbg !4812
  %_0.i2598.1 = fadd float %_0.i3097.1, 0x3FAC91CE60000000, !dbg !4814
  %_0.i3097.2 = fmul float %_0.i3379, %_0.i2598.1, !dbg !4812
  %_0.i2598.2 = fadd float %_0.i3097.2, 0x3FCEBDB560000000, !dbg !4814
  %_0.i3097.3 = fmul float %_0.i3379, %_0.i2598.2, !dbg !4812
  %_0.i2598.3 = fadd float %_0.i3097.3, 0x3FE62E4BA0000000, !dbg !4814
  %_0.i3094 = fmul float %_0.i3378, 0x3F5E974FA0000000, !dbg !4816
  %_0.i2596 = fadd float %_0.i3094, 0x3F82778560000000, !dbg !4818
  %_0.i3094.1 = fmul float %_0.i3378, %_0.i2596, !dbg !4816
  %_0.i2596.1 = fadd float %_0.i3094.1, 0x3FAC91CE60000000, !dbg !4818
  %_0.i3094.2 = fmul float %_0.i3378, %_0.i2596.1, !dbg !4816
  %_0.i2596.2 = fadd float %_0.i3094.2, 0x3FCEBDB560000000, !dbg !4818
  %_0.i3094.3 = fmul float %_0.i3378, %_0.i2596.2, !dbg !4816
  %_0.i2596.3 = fadd float %_0.i3094.3, 0x3FE62E4BA0000000, !dbg !4818
  %_0.i3091 = fmul float %_0.i3377, 0x3F5E974FA0000000, !dbg !4820
  %_0.i2594 = fadd float %_0.i3091, 0x3F82778560000000, !dbg !4822
  %_0.i3091.1 = fmul float %_0.i3377, %_0.i2594, !dbg !4820
  %_0.i2594.1 = fadd float %_0.i3091.1, 0x3FAC91CE60000000, !dbg !4822
  %_0.i3091.2 = fmul float %_0.i3377, %_0.i2594.1, !dbg !4820
  %_0.i2594.2 = fadd float %_0.i3091.2, 0x3FCEBDB560000000, !dbg !4822
  %_0.i3091.3 = fmul float %_0.i3377, %_0.i2594.2, !dbg !4820
  %_0.i2594.3 = fadd float %_0.i3091.3, 0x3FE62E4BA0000000, !dbg !4822
  %_0.i3090 = fmul float %_0.i3377, %_0.i2594.3, !dbg !4824
  %_0.i2593 = fadd float %_0.i3090, 1.000000e+00, !dbg !4826
  %biased.i2194 = fadd float %236, 0x4160000FE0000000, !dbg !4828
  %_4.i2195 = bitcast float %biased.i2194 to i32, !dbg !4832
  %_3.i2196 = shl i32 %_4.i2195, 23, !dbg !4836
  %_0.i2197 = bitcast i32 %_3.i2196 to float, !dbg !4837
  %_0.i3089 = fmul float %_0.i2593, %_0.i2197, !dbg !4840
  %_0.i3093 = fmul float %_0.i3378, %_0.i2596.3, !dbg !4842
  %_0.i2595 = fadd float %_0.i3093, 1.000000e+00, !dbg !4844
  %biased.i2198 = fadd float %243, 0x4160000FE0000000, !dbg !4846
  %_4.i2199 = bitcast float %biased.i2198 to i32, !dbg !4848
  %_3.i2200 = shl i32 %_4.i2199, 23, !dbg !4850
  %_0.i2201 = bitcast i32 %_3.i2200 to float, !dbg !4851
  %_0.i3092 = fmul float %_0.i2595, %_0.i2201, !dbg !4853
  %_0.i3096 = fmul float %_0.i3379, %_0.i2598.3, !dbg !4855
  %_0.i2597 = fadd float %_0.i3096, 1.000000e+00, !dbg !4857
  %biased.i2202 = fadd float %250, 0x4160000FE0000000, !dbg !4859
  %_4.i2203 = bitcast float %biased.i2202 to i32, !dbg !4861
  %_3.i2204 = shl i32 %_4.i2203, 23, !dbg !4863
  %_0.i2205 = bitcast i32 %_3.i2204 to float, !dbg !4864
  %_0.i3095 = fmul float %_0.i2597, %_0.i2205, !dbg !4866
  %_0.i3099 = fmul float %_0.i3380, %_0.i2600.3, !dbg !4868
  %_0.i2599 = fadd float %_0.i3099, 1.000000e+00, !dbg !4870
  %biased.i2206 = fadd float %257, 0x4160000FE0000000, !dbg !4872
  %_4.i2207 = bitcast float %biased.i2206 to i32, !dbg !4874
  %_3.i2208 = shl i32 %_4.i2207, 23, !dbg !4876
  %_0.i2209 = bitcast i32 %_3.i2208 to float, !dbg !4877
  %_0.i3098 = fmul float %_0.i2599, %_0.i2209, !dbg !4879
  %_262.i.i.i = icmp ugt i64 %_37.sroa.0.0.i.i.i, %_311.1.i.i.i, !dbg !4881
  br i1 %_262.i.i.i, label %bb84.i.i.i, label %bb85.i.i.i, !dbg !4881, !prof !1664

panic1.i221.i.i:                                  ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i236.i.i, i64 noundef range(i64 0, 2305843009213693952) %_314.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !4886, !noalias !4888
  unreachable, !dbg !4886

bb82.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3961
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i.i11052, i64 noundef %_310.1.i.i.i, i64 noundef %_310.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_007bf1cfdcf9f845db5ef166fc9a1790) #26, !dbg !4889, !noalias !3964
  unreachable, !dbg !4889

bb85.i.i.i:                                       ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i.i.split.us
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4890), !dbg !4893
  %_3.not.i3520 = icmp eq i64 %_311.1.i.i.i, %_37.sroa.0.0.i.i.i, !dbg !4894
  br i1 %_3.not.i3520, label %panic.i3523, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3524, !dbg !4894

panic.i3523:                                      ; preds = %bb85.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4894, !noalias !4896
  unreachable, !dbg !4894

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3524: ; preds = %bb85.i.i.i
  %_269.i.i.i = getelementptr inbounds nuw float, ptr %_311.0.i.i.i, i64 %_37.sroa.0.0.i.i.i, !dbg !4897
  %_0.i3522 = load float, ptr %_269.i.i.i, align 4, !dbg !4894, !alias.scope !4890, !noalias !3964, !noundef !11
  %_0.i3280 = fmul float %_0.i3089, %_0.i3522, !dbg !4902
  %_270.i.i.i = icmp ugt i64 %_37.sroa.0.0.i.i.i, %_312.1.i.i.i, !dbg !4904
  br i1 %_270.i.i.i, label %bb86.i.i.i, label %bb87.i.i.i, !dbg !4904, !prof !1664

bb84.i.i.i:                                       ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i.i.split.us
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i.i, i64 noundef %_311.1.i.i.i, i64 noundef %_311.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a45b75cd2d07085fe69fe186ba115725) #26, !dbg !4908, !noalias !3964
  unreachable, !dbg !4908

bb87.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3524
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4909), !dbg !4912
  %_3.not.i3515 = icmp eq i64 %_312.1.i.i.i, %_37.sroa.0.0.i.i.i, !dbg !4913
  br i1 %_3.not.i3515, label %panic.i3518, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3519, !dbg !4913

panic.i3518:                                      ; preds = %bb87.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4913, !noalias !4915
  unreachable, !dbg !4913

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3519: ; preds = %bb87.i.i.i
  %_277.i.i.i = getelementptr inbounds nuw float, ptr %_312.0.i.i.i, i64 %_37.sroa.0.0.i.i.i, !dbg !4916
  %_0.i3517 = load float, ptr %_277.i.i.i, align 4, !dbg !4913, !alias.scope !4909, !noalias !3964, !noundef !11
  %_0.i3279 = fmul float %_0.i3092, %_0.i3517, !dbg !4921
  %_0.i2790 = fadd float %_0.i3280, %_0.i3279, !dbg !4923
  %_278.i.i.i = icmp ugt i64 %_37.sroa.0.0.i.i.i, %_313.1.i.i.i, !dbg !4925
  br i1 %_278.i.i.i, label %bb88.i.i.i, label %bb89.i.i.i, !dbg !4925, !prof !1664

bb86.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3524
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i.i, i64 noundef %_312.1.i.i.i, i64 noundef %_312.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_36be93341d7083c8a492c530428430ea) #26, !dbg !4930, !noalias !3964
  unreachable, !dbg !4930

bb89.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3519
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4931), !dbg !4934
  %_3.not.i3510 = icmp eq i64 %_313.1.i.i.i, %_37.sroa.0.0.i.i.i, !dbg !4935
  br i1 %_3.not.i3510, label %panic.i3513, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3514, !dbg !4935

panic.i3513:                                      ; preds = %bb89.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4935, !noalias !4937
  unreachable, !dbg !4935

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3514: ; preds = %bb89.i.i.i
  %_285.i.i.i = getelementptr inbounds nuw float, ptr %_313.0.i.i.i, i64 %_37.sroa.0.0.i.i.i, !dbg !4938
  %_0.i3512 = load float, ptr %_285.i.i.i, align 4, !dbg !4935, !alias.scope !4931, !noalias !3964, !noundef !11
  %_0.i3278 = fmul float %_0.i3095, %_0.i3512, !dbg !4943
  %_286.i.i.i = icmp ugt i64 %_37.sroa.0.0.i.i.i, %_314.1.i.i.i, !dbg !4945
  br i1 %_286.i.i.i, label %bb90.i.i.i, label %bb91.i.i.i, !dbg !4945, !prof !1664

bb88.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3519
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i.i, i64 noundef %_313.1.i.i.i, i64 noundef %_313.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a6f1b2a44d3e33eba5c708677237c81) #26, !dbg !4949, !noalias !3964
  unreachable, !dbg !4949

bb91.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3514
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4950), !dbg !4953
  %_3.not.i3505 = icmp eq i64 %_314.1.i.i.i, %_37.sroa.0.0.i.i.i, !dbg !4954
  br i1 %_3.not.i3505, label %panic.i3508, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949, !dbg !4954

panic.i3508:                                      ; preds = %bb91.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4954, !noalias !4956
  unreachable, !dbg !4954

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949: ; preds = %bb91.i.i.i
  %_293.i.i.i = getelementptr inbounds nuw float, ptr %_314.0.i.i.i, i64 %_37.sroa.0.0.i.i.i, !dbg !4957
  %_0.i3507 = load float, ptr %_293.i.i.i, align 4, !dbg !4954, !alias.scope !4950, !noalias !3964, !noundef !11
  %_0.i3277 = fmul float %_0.i3098, %_0.i3507, !dbg !4962
  %_0.i2789 = fadd float %_0.i3278, %_0.i3277, !dbg !4964
  store float %_0.i2790, ptr %_180.i.i.i, align 4, !dbg !4966, !alias.scope !4969, !noalias !3964
  store float %_0.i2789, ptr %_188.i.i.i, align 4, !dbg !4972, !alias.scope !4974, !noalias !3964
  %exitcond13933.not = icmp eq i64 %206, %plan.0.i.i, !dbg !3922
  br i1 %exitcond13933.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_KBZ_EB2_.exit.i.i, label %bb54.i.i.i, !dbg !3932

bb90.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3514
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i.i, i64 noundef %_314.1.i.i.i, i64 noundef %_314.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4aa2eaec3d1833a4a887fe1d76c05ca7) #26, !dbg !4977, !noalias !3964
  unreachable, !dbg !4977

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_KBZ_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949, %bb27.i.i
  %filter_near.i.i.i.sroa.14.0.lcssa = phi float [ %filter_near.i.i.i.sroa.14.0.copyload, %bb27.i.i ], [ %_0.i4156, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4978
  %filter_near.i.i.i.sroa.11.0.lcssa = phi float [ %filter_near.i.i.i.sroa.11.0.copyload, %bb27.i.i ], [ %_0.i4160, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4978
  %filter_near.i.i.i.sroa.7.0.lcssa = phi float [ %filter_near.i.i.i.sroa.7.0.copyload, %bb27.i.i ], [ %_0.i4148, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4978
  %filter_near.i.i.i.sroa.0.0.lcssa = phi float [ %filter_near.i.i.i.sroa.0.0.copyload, %bb27.i.i ], [ %_0.i4152, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4978
  %filter_far.i.i.i.sroa.0.0.lcssa = phi float [ %filter_far.i.i.i.sroa.0.0.copyload, %bb27.i.i ], [ %_0.i4136, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4979
  %filter_far.i.i.i.sroa.7.0.lcssa = phi float [ %filter_far.i.i.i.sroa.7.0.copyload, %bb27.i.i ], [ %_0.i4132, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4979
  %filter_far.i.i.i.sroa.11.0.lcssa = phi float [ %filter_far.i.i.i.sroa.11.0.copyload, %bb27.i.i ], [ %_0.i4144, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4979
  %filter_far.i.i.i.sroa.14.0.lcssa = phi float [ %filter_far.i.i.i.sroa.14.0.copyload, %bb27.i.i ], [ %_0.i4140, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4979
  %gain_near.i.i.i.sroa.0.0.lcssa = phi i32 [ %201, %bb27.i.i ], [ %_3.i4319, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4980
  %gain_near.i.i.i.sroa.6.0.lcssa = phi i32 [ %202, %bb27.i.i ], [ %_3.i4315, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4980
  %gain_far.i.i.i.sroa.0.0.lcssa = phi i32 [ %203, %bb27.i.i ], [ %_3.i4311, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4981
  %gain_far.i.i.i.sroa.6.0.lcssa = phi i32 [ %204, %bb27.i.i ], [ %_3.i4307, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4981
  %position.sroa.0.0.i.i.i.lcssa = phi i64 [ %205, %bb27.i.i ], [ %_37.sroa.0.0.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3949 ], !dbg !4982
  store float %filter_near.i.i.i.sroa.0.0.lcssa, ptr %164, align 8, !dbg !4983, !noalias !3964
  store float %filter_near.i.i.i.sroa.7.0.lcssa, ptr %filter_near.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !4983, !noalias !3964
  store float %filter_near.i.i.i.sroa.11.0.lcssa, ptr %filter_near.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !4983, !noalias !3964
  store float %filter_near.i.i.i.sroa.14.0.lcssa, ptr %filter_near.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !4983, !noalias !3964
  store float %filter_far.i.i.i.sroa.0.0.lcssa, ptr %165, align 8, !dbg !4984, !noalias !3964
  store float %filter_far.i.i.i.sroa.7.0.lcssa, ptr %filter_far.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !4984, !noalias !3964
  store float %filter_far.i.i.i.sroa.11.0.lcssa, ptr %filter_far.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !4984, !noalias !3964
  store float %filter_far.i.i.i.sroa.14.0.lcssa, ptr %filter_far.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !4984, !noalias !3964
  store i32 %gain_near.i.i.i.sroa.0.0.lcssa, ptr %166, align 8, !dbg !4985, !noalias !3964
  store i32 %gain_near.i.i.i.sroa.6.0.lcssa, ptr %.sroa_idx6954, align 4, !dbg !4985, !noalias !3964
  store i32 %gain_far.i.i.i.sroa.0.0.lcssa, ptr %167, align 8, !dbg !4986, !noalias !3964
  store i32 %gain_far.i.i.i.sroa.6.0.lcssa, ptr %.sroa_idx6959, align 4, !dbg !4986, !noalias !3964
  store i64 %position.sroa.0.0.i.i.i.lcssa, ptr %_51.i.i, align 8, !dbg !4987, !alias.scope !3919, !noalias !3920
  br label %bb15.i.i, !dbg !4988

bb15.i.i:                                         ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_Kb1_EB2_.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_KBZ_EB2_.exit.i.i
  %_8.i.i = icmp ult i64 %_32.i.i, %_19.1, !dbg !3837
  br i1 %_8.i.i, label %bb2.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit, !dbg !3837

bb17.i.i:                                         ; preds = %bb9.i.i
  %_75.i.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i.i11228, !dbg !4989
  %_85.i.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i.i11228, !dbg !4992
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4999), !dbg !5002
  %filter_near.i17.i.i.sroa.0.0.copyload = load float, ptr %164, align 8, !dbg !5003, !noalias !5009
  %filter_near.i17.i.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !5003, !noalias !5009
  %filter_near.i17.i.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !5003, !noalias !5009
  %filter_near.i17.i.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !5003, !noalias !5009
  %filter_far.i16.i.i.sroa.0.0.copyload = load float, ptr %165, align 8, !dbg !5014, !noalias !5009
  %filter_far.i16.i.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !5014, !noalias !5009
  %filter_far.i16.i.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !5014, !noalias !5009
  %filter_far.i16.i.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !5014, !noalias !5009
  %258 = load i32, ptr %166, align 8, !dbg !5016
  %259 = load i32, ptr %.sroa_idx6954, align 4, !dbg !5016
  %260 = load i32, ptr %167, align 8, !dbg !5018
  %261 = load i32, ptr %.sroa_idx6959, align 4, !dbg !5018
  %262 = load i64, ptr %_51.i.i, align 8, !dbg !5020, !alias.scope !5022, !noalias !5023, !noundef !11
  %_164.i30.i.i11199.not = icmp eq i64 %plan.0.i.i, 0, !dbg !5025
  br i1 %_164.i30.i.i11199.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_Kb1_EB2_.exit.i.i, label %bb54.i33.i.i, !dbg !5035

bb19.i.i:                                         ; preds = %bb9.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i11228, i64 noundef %_32.i.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_61a2f59006034c74bb8ab3eed52140c6) #26, !dbg !5036, !noalias !3842
  unreachable, !dbg !5036

bb54.i33.i.i:                                     ; preds = %bb17.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit
  %segments.i.i.sroa.112.1 = phi float [ %_0.i2781.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.9.i6451, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.108.1 = phi float [ %_0.i2781.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.8.i6449, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.104.1 = phi float [ %_0.i2781.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.7.i6447, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.100.1 = phi float [ %_0.i2781.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.6.i6445, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.94.1 = phi float [ %_0.i2781.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.5.i6443, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.88.1 = phi float [ %_0.i2781.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.4.i6441, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.84.1 = phi float [ %_0.i2781.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.3.i6439, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.80.1 = phi float [ %_0.i2781.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.2.i6437, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.76.1 = phi float [ %_0.i2781.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.1.i6435, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.70.1 = phi float [ %_0.i2781, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.i6433, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.44.1 = phi float [ %_0.i2782.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.9.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.40.1 = phi float [ %_0.i2782.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.8.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.36.1 = phi float [ %_0.i2782.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.7.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.32.1 = phi float [ %_0.i2782.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.6.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.26.1 = phi float [ %_0.i2782.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.5.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.20.1 = phi float [ %_0.i2782.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.4.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.16.1 = phi float [ %_0.i2782.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.3.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.12.1 = phi float [ %_0.i2782.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.2.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.8.1 = phi float [ %_0.i2782.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.1.i, %bb17.i.i ], !dbg !5037
  %segments.i.i.sroa.0.1 = phi float [ %_0.i2782, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.i, %bb17.i.i ], !dbg !5037
  %iter.sroa.0.0.i29.i.i11213 = phi i64 [ %263, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ 0, %bb17.i.i ]
  %position.sroa.0.0.i28.i.i11212 = phi i64 [ %_37.sroa.0.0.i36.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %262, %bb17.i.i ]
  %gain_far.i14.i.i.sroa.6.011211 = phi i32 [ %_3.i4291, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %261, %bb17.i.i ]
  %gain_far.i14.i.i.sroa.0.011210 = phi i32 [ %_3.i4295, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %260, %bb17.i.i ]
  %gain_near.i15.i.i.sroa.6.011209 = phi i32 [ %_3.i4299, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %259, %bb17.i.i ]
  %gain_near.i15.i.i.sroa.0.011208 = phi i32 [ %_3.i4303, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %258, %bb17.i.i ]
  %filter_far.i16.i.i.sroa.14.011207 = phi float [ %_0.i4108, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_far.i16.i.i.sroa.14.0.copyload, %bb17.i.i ]
  %filter_far.i16.i.i.sroa.11.011206 = phi float [ %_0.i4112, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_far.i16.i.i.sroa.11.0.copyload, %bb17.i.i ]
  %filter_far.i16.i.i.sroa.7.011205 = phi float [ %_0.i4100, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_far.i16.i.i.sroa.7.0.copyload, %bb17.i.i ]
  %filter_far.i16.i.i.sroa.0.011204 = phi float [ %_0.i4104, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_far.i16.i.i.sroa.0.0.copyload, %bb17.i.i ]
  %filter_near.i17.i.i.sroa.14.011203 = phi float [ %_0.i4124, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_near.i17.i.i.sroa.14.0.copyload, %bb17.i.i ]
  %filter_near.i17.i.i.sroa.11.011202 = phi float [ %_0.i4128, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_near.i17.i.i.sroa.11.0.copyload, %bb17.i.i ]
  %filter_near.i17.i.i.sroa.7.011201 = phi float [ %_0.i4116, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_near.i17.i.i.sroa.7.0.copyload, %bb17.i.i ]
  %filter_near.i17.i.i.sroa.0.011200 = phi float [ %_0.i4120, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_near.i17.i.i.sroa.0.0.copyload, %bb17.i.i ]
  %_0.i2782 = fadd float %segments.i.i.sroa.0.1, %_16.le.i, !dbg !5038
  %_0.i2781 = fadd float %segments.i.i.sroa.70.1, %_16.le.i6434, !dbg !5043
  %_0.i2782.1 = fadd float %segments.i.i.sroa.8.1, %_16.le.1.i, !dbg !5038
  %_0.i2781.1 = fadd float %segments.i.i.sroa.76.1, %_16.le.1.i6436, !dbg !5043
  %_0.i2782.2 = fadd float %segments.i.i.sroa.12.1, %_16.le.2.i, !dbg !5038
  %_0.i2781.2 = fadd float %segments.i.i.sroa.80.1, %_16.le.2.i6438, !dbg !5043
  %_0.i2782.3 = fadd float %segments.i.i.sroa.16.1, %_16.le.3.i, !dbg !5038
  %_0.i2781.3 = fadd float %segments.i.i.sroa.84.1, %_16.le.3.i6440, !dbg !5043
  %_0.i2782.4 = fadd float %segments.i.i.sroa.20.1, %_16.le.4.i, !dbg !5038
  %_0.i2781.4 = fadd float %segments.i.i.sroa.88.1, %_16.le.4.i6442, !dbg !5043
  %_0.i2782.5 = fadd float %segments.i.i.sroa.26.1, %_16.le.5.i, !dbg !5038
  %_0.i2781.5 = fadd float %segments.i.i.sroa.94.1, %_16.le.5.i6444, !dbg !5043
  %_0.i2782.6 = fadd float %segments.i.i.sroa.32.1, %_16.le.6.i, !dbg !5038
  %_0.i2781.6 = fadd float %segments.i.i.sroa.100.1, %_16.le.6.i6446, !dbg !5043
  %_0.i2782.7 = fadd float %segments.i.i.sroa.36.1, %_16.le.7.i, !dbg !5038
  %_0.i2781.7 = fadd float %segments.i.i.sroa.104.1, %_16.le.7.i6448, !dbg !5043
  %_0.i2782.8 = fadd float %segments.i.i.sroa.40.1, %_16.le.8.i, !dbg !5038
  %_0.i2781.8 = fadd float %segments.i.i.sroa.108.1, %_16.le.8.i6450, !dbg !5043
  %_0.i2782.9 = fadd float %segments.i.i.sroa.44.1, %_16.le.9.i, !dbg !5038
  %_0.i2781.9 = fadd float %segments.i.i.sroa.112.1, %_16.le.9.i6452, !dbg !5043
  %263 = add nuw i64 %iter.sroa.0.0.i29.i.i11213, 1, !dbg !5045
  %_38.i34.i.i = add i64 %position.sroa.0.0.i28.i.i11212, 1, !dbg !5051
  %_172.not.i35.i.i = icmp ult i64 %_38.i34.i.i, %ring_len.i.i, !dbg !5053
  %264 = select i1 %_172.not.i35.i.i, i64 0, i64 %ring_len.i.i, !dbg !5053
  %_37.sroa.0.0.i36.i.i = sub nuw i64 %_38.i34.i.i, %264, !dbg !5053
  %_180.i38.i.i = getelementptr inbounds nuw float, ptr %_75.i.i, i64 %iter.sroa.0.0.i29.i.i11213, !dbg !5056
  %_0.i3502 = load float, ptr %_180.i38.i.i, align 4, !dbg !5066, !alias.scope !5068, !noalias !5071, !noundef !11
  %_188.i43.i.i = getelementptr inbounds nuw float, ptr %_85.i.i, i64 %iter.sroa.0.0.i29.i.i11213, !dbg !5072
  %_0.i3497 = load float, ptr %_188.i43.i.i, align 4, !dbg !5081, !alias.scope !5083, !noalias !5071, !noundef !11
  %_7.i2 = load float, ptr %_63.i.i.i, align 4, !dbg !5086, !alias.scope !5089, !noalias !5092, !noundef !11
  %_8.i3 = load float, ptr %168, align 4, !dbg !5094, !alias.scope !5089, !noalias !5092, !noundef !11
  %_9.i4 = load float, ptr %169, align 4, !dbg !5095, !alias.scope !5089, !noalias !5092, !noundef !11
  %_0.i3411 = fsub float %_0.i3502, %filter_near.i17.i.i.sroa.7.011201, !dbg !5096
  %_0.i3222 = fmul float %_0.i3411, %_8.i3, !dbg !5099
  %_4.i2835 = fmul float %filter_near.i17.i.i.sroa.0.011200, %_7.i2, !dbg !5101
  %_0.i2836 = fadd float %_4.i2835, %_0.i3222, !dbg !5101
  %_0.i2654 = fadd float %filter_near.i17.i.i.sroa.0.011200, %_0.i2836, !dbg !5103
  %_0.i3221 = fmul float %filter_near.i17.i.i.sroa.0.011200, %_8.i3, !dbg !5105
  %_4.i2833 = fmul float %_0.i3411, %_9.i4, !dbg !5107
  %_0.i2834 = fadd float %_0.i3221, %_4.i2833, !dbg !5107
  %_0.i2653 = fadd float %filter_near.i17.i.i.sroa.7.011201, %_0.i2834, !dbg !5109
  %_0.i2652 = fadd float %_0.i2836, %_0.i2836, !dbg !5111
  %_0.i2651 = fadd float %filter_near.i17.i.i.sroa.0.011200, %_0.i2652, !dbg !5113
  %265 = tail call noundef float @llvm.fabs.f32(float %_0.i2651), !dbg !5115
  %266 = fcmp uge float %265, 0x3BC79CA100000000, !dbg !5118
  %_0.i4120 = select i1 %266, float %_0.i2651, float 0.000000e+00, !dbg !5120
  %_0.i2650 = fadd float %_0.i2834, %_0.i2834, !dbg !5121
  %_0.i2649 = fadd float %filter_near.i17.i.i.sroa.7.011201, %_0.i2650, !dbg !5123
  %267 = tail call noundef float @llvm.fabs.f32(float %_0.i2649), !dbg !5125
  %268 = fcmp uge float %267, 0x3BC79CA100000000, !dbg !5128
  %_0.i4116 = select i1 %268, float %_0.i2649, float 0.000000e+00, !dbg !5130
  %_12.i7 = load float, ptr %170, align 4, !dbg !5131, !alias.scope !5089, !noalias !5092, !noundef !11
  %_4.i2941 = fmul float %_12.i7, %_0.i2654, !dbg !5132
  %_0.i2942 = fadd float %_0.i3502, %_4.i2941, !dbg !5132
  %_0.i3412 = fsub float %_0.i2653, %filter_near.i17.i.i.sroa.14.011203, !dbg !5134
  %_0.i3224 = fmul float %_8.i3, %_0.i3412, !dbg !5137
  %_4.i2839 = fmul float %filter_near.i17.i.i.sroa.11.011202, %_7.i2, !dbg !5139
  %_0.i2840 = fadd float %_4.i2839, %_0.i3224, !dbg !5139
  %_0.i3223 = fmul float %filter_near.i17.i.i.sroa.11.011202, %_8.i3, !dbg !5141
  %_4.i2837 = fmul float %_9.i4, %_0.i3412, !dbg !5143
  %_0.i2838 = fadd float %_0.i3223, %_4.i2837, !dbg !5143
  %_0.i2659 = fadd float %filter_near.i17.i.i.sroa.14.011203, %_0.i2838, !dbg !5145
  %_0.i2658 = fadd float %_0.i2840, %_0.i2840, !dbg !5147
  %_0.i2657 = fadd float %filter_near.i17.i.i.sroa.11.011202, %_0.i2658, !dbg !5149
  %269 = tail call noundef float @llvm.fabs.f32(float %_0.i2657), !dbg !5151
  %270 = fcmp uge float %269, 0x3BC79CA100000000, !dbg !5154
  %_0.i4128 = select i1 %270, float %_0.i2657, float 0.000000e+00, !dbg !5156
  %_0.i2656 = fadd float %_0.i2838, %_0.i2838, !dbg !5157
  %_0.i2655 = fadd float %filter_near.i17.i.i.sroa.14.011203, %_0.i2656, !dbg !5159
  %271 = tail call noundef float @llvm.fabs.f32(float %_0.i2655), !dbg !5161
  %272 = fcmp uge float %271, 0x3BC79CA100000000, !dbg !5164
  %_0.i4124 = select i1 %272, float %_0.i2655, float 0.000000e+00, !dbg !5166
  %_0.i3443 = fsub float %_0.i2942, %_0.i2659, !dbg !5167
  %_7.i1 = load float, ptr %_68.i.i.i, align 4, !dbg !5169, !alias.scope !5172, !noalias !5175, !noundef !11
  %_8.i = load float, ptr %171, align 4, !dbg !5177, !alias.scope !5172, !noalias !5175, !noundef !11
  %_9.i = load float, ptr %172, align 4, !dbg !5178, !alias.scope !5172, !noalias !5175, !noundef !11
  %_0.i3409 = fsub float %_0.i3497, %filter_far.i16.i.i.sroa.7.011205, !dbg !5179
  %_0.i3218 = fmul float %_0.i3409, %_8.i, !dbg !5182
  %_4.i2827 = fmul float %filter_far.i16.i.i.sroa.0.011204, %_7.i1, !dbg !5184
  %_0.i2828 = fadd float %_4.i2827, %_0.i3218, !dbg !5184
  %_0.i2642 = fadd float %filter_far.i16.i.i.sroa.0.011204, %_0.i2828, !dbg !5186
  %_0.i3217 = fmul float %filter_far.i16.i.i.sroa.0.011204, %_8.i, !dbg !5188
  %_4.i2825 = fmul float %_0.i3409, %_9.i, !dbg !5190
  %_0.i2826 = fadd float %_0.i3217, %_4.i2825, !dbg !5190
  %_0.i2641 = fadd float %filter_far.i16.i.i.sroa.7.011205, %_0.i2826, !dbg !5192
  %_0.i2640 = fadd float %_0.i2828, %_0.i2828, !dbg !5194
  %_0.i2639 = fadd float %filter_far.i16.i.i.sroa.0.011204, %_0.i2640, !dbg !5196
  %273 = tail call noundef float @llvm.fabs.f32(float %_0.i2639), !dbg !5198
  %274 = fcmp uge float %273, 0x3BC79CA100000000, !dbg !5201
  %_0.i4104 = select i1 %274, float %_0.i2639, float 0.000000e+00, !dbg !5203
  %_0.i2638 = fadd float %_0.i2826, %_0.i2826, !dbg !5204
  %_0.i2637 = fadd float %filter_far.i16.i.i.sroa.7.011205, %_0.i2638, !dbg !5206
  %275 = tail call noundef float @llvm.fabs.f32(float %_0.i2637), !dbg !5208
  %276 = fcmp uge float %275, 0x3BC79CA100000000, !dbg !5211
  %_0.i4100 = select i1 %276, float %_0.i2637, float 0.000000e+00, !dbg !5213
  %_12.i = load float, ptr %173, align 4, !dbg !5214, !alias.scope !5172, !noalias !5175, !noundef !11
  %_4.i2943 = fmul float %_12.i, %_0.i2642, !dbg !5215
  %_0.i2944 = fadd float %_0.i3497, %_4.i2943, !dbg !5215
  %_0.i3410 = fsub float %_0.i2641, %filter_far.i16.i.i.sroa.14.011207, !dbg !5217
  %_0.i3220 = fmul float %_8.i, %_0.i3410, !dbg !5220
  %_4.i2831 = fmul float %filter_far.i16.i.i.sroa.11.011206, %_7.i1, !dbg !5222
  %_0.i2832 = fadd float %_4.i2831, %_0.i3220, !dbg !5222
  %_0.i3219 = fmul float %filter_far.i16.i.i.sroa.11.011206, %_8.i, !dbg !5224
  %_4.i2829 = fmul float %_9.i, %_0.i3410, !dbg !5226
  %_0.i2830 = fadd float %_0.i3219, %_4.i2829, !dbg !5226
  %_0.i2647 = fadd float %filter_far.i16.i.i.sroa.14.011207, %_0.i2830, !dbg !5228
  %_0.i2646 = fadd float %_0.i2832, %_0.i2832, !dbg !5230
  %_0.i2645 = fadd float %filter_far.i16.i.i.sroa.11.011206, %_0.i2646, !dbg !5232
  %277 = tail call noundef float @llvm.fabs.f32(float %_0.i2645), !dbg !5234
  %278 = fcmp uge float %277, 0x3BC79CA100000000, !dbg !5237
  %_0.i4112 = select i1 %278, float %_0.i2645, float 0.000000e+00, !dbg !5239
  %_0.i2644 = fadd float %_0.i2830, %_0.i2830, !dbg !5240
  %_0.i2643 = fadd float %filter_far.i16.i.i.sroa.14.011207, %_0.i2644, !dbg !5242
  %279 = tail call noundef float @llvm.fabs.f32(float %_0.i2643), !dbg !5244
  %280 = fcmp uge float %279, 0x3BC79CA100000000, !dbg !5247
  %_0.i4108 = select i1 %280, float %_0.i2643, float 0.000000e+00, !dbg !5249
  %_0.i3444 = fsub float %_0.i2944, %_0.i2647, !dbg !5250
  %_307.1.i50.i.i = load i64, ptr %174, align 8, !dbg !5252, !noalias !5071, !noundef !11
  %_230.i51.i.i = icmp ugt i64 %position.sroa.0.0.i28.i.i11212, %_307.1.i50.i.i, !dbg !5254
  br i1 %_230.i51.i.i, label %bb76.i151.i.i, label %bb77.i52.i.i, !dbg !5254, !prof !1664

bb77.i52.i.i:                                     ; preds = %bb54.i33.i.i
  %_307.0.i53.i.i = load ptr, ptr %123, align 8, !dbg !5252, !noalias !5071, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5260), !dbg !5263
  %_4.not.i3942 = icmp eq i64 %_307.1.i50.i.i, %position.sroa.0.0.i28.i.i11212, !dbg !5264
  br i1 %_4.not.i3942, label %panic.i3944, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3945, !dbg !5264

panic.i3944:                                      ; preds = %bb77.i52.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !5264, !noalias !5266
  unreachable, !dbg !5264

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3945: ; preds = %bb77.i52.i.i
  %_237.i56.i.i = getelementptr inbounds nuw float, ptr %_307.0.i53.i.i, i64 %position.sroa.0.0.i28.i.i11212, !dbg !5267
  store float %_0.i2659, ptr %_237.i56.i.i, align 4, !dbg !5264, !alias.scope !5260, !noalias !5071
  %_308.1.i57.i.i = load i64, ptr %176, align 8, !dbg !5273, !noalias !5071, !noundef !11
  %_238.i58.i.i = icmp ugt i64 %position.sroa.0.0.i28.i.i11212, %_308.1.i57.i.i, !dbg !5274
  br i1 %_238.i58.i.i, label %bb78.i150.i.i, label %bb79.i59.i.i, !dbg !5274, !prof !1664

bb76.i151.i.i:                                    ; preds = %bb54.i33.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i.i11212, i64 noundef %_307.1.i50.i.i, i64 noundef %_307.1.i50.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7aeb6b0a3ba8e73c50a5abef6c30558) #26, !dbg !5278, !noalias !5071
  unreachable, !dbg !5278

bb79.i59.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3945
  %_308.0.i60.i.i = load ptr, ptr %175, align 8, !dbg !5273, !noalias !5071, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5279), !dbg !5282
  %_4.not.i3938 = icmp eq i64 %_308.1.i57.i.i, %position.sroa.0.0.i28.i.i11212, !dbg !5283
  br i1 %_4.not.i3938, label %panic.i3940, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3941, !dbg !5283

panic.i3940:                                      ; preds = %bb79.i59.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !5283, !noalias !5285
  unreachable, !dbg !5283

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3941: ; preds = %bb79.i59.i.i
  %_245.i62.i.i = getelementptr inbounds nuw float, ptr %_308.0.i60.i.i, i64 %position.sroa.0.0.i28.i.i11212, !dbg !5286
  store float %_0.i3443, ptr %_245.i62.i.i, align 4, !dbg !5283, !alias.scope !5279, !noalias !5071
  %_309.1.i63.i.i = load i64, ptr %177, align 8, !dbg !5291, !noalias !5071, !noundef !11
  %_246.i64.i.i = icmp ugt i64 %position.sroa.0.0.i28.i.i11212, %_309.1.i63.i.i, !dbg !5292
  br i1 %_246.i64.i.i, label %bb80.i149.i.i, label %bb81.i65.i.i, !dbg !5292, !prof !1664

bb78.i150.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3945
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i.i11212, i64 noundef %_308.1.i57.i.i, i64 noundef %_308.1.i57.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8e669d1bed0fb747f8a7bff920a6571) #26, !dbg !5296, !noalias !5071
  unreachable, !dbg !5296

bb81.i65.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3941
  %_309.0.i66.i.i = load ptr, ptr %_18.i.i, align 8, !dbg !5291, !noalias !5071, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5297), !dbg !5300
  %_4.not.i3934 = icmp eq i64 %_309.1.i63.i.i, %position.sroa.0.0.i28.i.i11212, !dbg !5301
  br i1 %_4.not.i3934, label %panic.i3936, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3937, !dbg !5301

panic.i3936:                                      ; preds = %bb81.i65.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !5301, !noalias !5303
  unreachable, !dbg !5301

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3937: ; preds = %bb81.i65.i.i
  %_253.i68.i.i = getelementptr inbounds nuw float, ptr %_309.0.i66.i.i, i64 %position.sroa.0.0.i28.i.i11212, !dbg !5304
  store float %_0.i2647, ptr %_253.i68.i.i, align 4, !dbg !5301, !alias.scope !5297, !noalias !5071
  %_310.1.i69.i.i = load i64, ptr %179, align 8, !dbg !5309, !noalias !5071, !noundef !11
  %_254.i70.i.i = icmp ugt i64 %position.sroa.0.0.i28.i.i11212, %_310.1.i69.i.i, !dbg !5310
  br i1 %_254.i70.i.i, label %bb82.i148.i.i, label %bb83.i71.i.i, !dbg !5310, !prof !1664

bb80.i149.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3941
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i.i11212, i64 noundef %_309.1.i63.i.i, i64 noundef %_309.1.i63.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e81c2bc19b75441ce2fb90ce8f9eb70) #26, !dbg !5314, !noalias !5071
  unreachable, !dbg !5314

bb83.i71.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3937
  %_310.0.i72.i.i = load ptr, ptr %178, align 8, !dbg !5309, !noalias !5071, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5315), !dbg !5318
  %_4.not.i3930 = icmp eq i64 %_310.1.i69.i.i, %position.sroa.0.0.i28.i.i11212, !dbg !5319
  br i1 %_4.not.i3930, label %panic.i3932, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3933, !dbg !5319

panic.i3932:                                      ; preds = %bb83.i71.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !5319, !noalias !5321
  unreachable, !dbg !5319

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3933: ; preds = %bb83.i71.i.i
  %_261.i74.i.i = getelementptr inbounds nuw float, ptr %_310.0.i72.i.i, i64 %position.sroa.0.0.i28.i.i11212, !dbg !5322
  store float %_0.i3444, ptr %_261.i74.i.i, align 4, !dbg !5319, !alias.scope !5315, !noalias !5071
  %_311.0.i75.i.i = load ptr, ptr %123, align 8, !dbg !5327, !noalias !5071, !nonnull !11, !noundef !11
  %_311.1.i76.i.i = load i64, ptr %174, align 8, !dbg !5327, !noalias !5071, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5328), !dbg !5331
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5332), !dbg !5331
  %_13.i199.i.i = load i64, ptr %_85.i.i.i, align 8, !alias.scope !5332, !noalias !5334, !noundef !11
  %_12.i200.i.i = add i64 %_13.i199.i.i, %position.sroa.0.0.i28.i.i11212
  %_31.not.i201.i.i = icmp ult i64 %_12.i200.i.i, %ring_len.i.i
  %281 = select i1 %_31.not.i201.i.i, i64 0, i64 %ring_len.i.i
  %_11.sroa.0.0.i202.i.i = sub nuw i64 %_12.i200.i.i, %281
  %_16.i203.i.i = icmp ult i64 %_11.sroa.0.0.i202.i.i, %_311.1.i76.i.i
  br i1 %_16.i203.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i.i.split.us, label %panic1.i204.i.i

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i.i.split.us: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3933
  %282 = getelementptr inbounds nuw float, ptr %_311.0.i75.i.i, i64 %_11.sroa.0.0.i202.i.i
  %_8.i206.i.i.us.le = load float, ptr %282, align 4, !alias.scope !5328, !noalias !5335, !noundef !11
  %_312.0.i79.i.i = load ptr, ptr %175, align 8, !dbg !5336, !noalias !5071, !nonnull !11, !noundef !11
  %_312.1.i80.i.i = load i64, ptr %176, align 8, !dbg !5336, !noalias !5071, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5338), !dbg !5341
  %_16.i186.i.i = icmp ult i64 %_11.sroa.0.0.i202.i.i, %_312.1.i80.i.i
  br i1 %_16.i186.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i.i.split.us, label %panic1.i187.i.i

panic1.i204.i.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3933
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i202.i.i, i64 noundef range(i64 0, 2305843009213693952) %_311.1.i76.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !5342, !noalias !5344
  unreachable, !dbg !5342

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i.i.split.us
  %283 = getelementptr inbounds nuw float, ptr %_312.0.i79.i.i, i64 %_11.sroa.0.0.i202.i.i
  %_8.i189.i.i.us.le = load float, ptr %283, align 4, !alias.scope !5338, !noalias !5345, !noundef !11
  %_313.0.i82.i.i = load ptr, ptr %_18.i.i, align 8, !dbg !5347, !noalias !5071, !nonnull !11, !noundef !11
  %_313.1.i83.i.i = load i64, ptr %177, align 8, !dbg !5347, !noalias !5071, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5349), !dbg !5352
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5353), !dbg !5352
  %_13.i165.i.i = load i64, ptr %_93.i.i.i, align 8, !alias.scope !5353, !noalias !5355, !noundef !11
  %_12.i166.i.i = add i64 %_13.i165.i.i, %position.sroa.0.0.i28.i.i11212
  %_31.not.i167.i.i = icmp ult i64 %_12.i166.i.i, %ring_len.i.i
  %284 = select i1 %_31.not.i167.i.i, i64 0, i64 %ring_len.i.i
  %_11.sroa.0.0.i168.i.i = sub nuw i64 %_12.i166.i.i, %284
  %_16.i169.i.i = icmp ult i64 %_11.sroa.0.0.i168.i.i, %_313.1.i83.i.i
  br i1 %_16.i169.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i.i.split.us, label %panic1.i170.i.i

panic1.i187.i.i:                                  ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i202.i.i, i64 noundef range(i64 0, 2305843009213693952) %_312.1.i80.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !5356, !noalias !5358
  unreachable, !dbg !5356

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i.i.split.us
  %285 = getelementptr inbounds nuw float, ptr %_313.0.i82.i.i, i64 %_11.sroa.0.0.i168.i.i
  %_8.i172.i.i.us.le = load float, ptr %285, align 4, !alias.scope !5349, !noalias !5359, !noundef !11
  %_314.0.i86.i.i = load ptr, ptr %178, align 8, !dbg !5360, !noalias !5071, !nonnull !11, !noundef !11
  %_314.1.i87.i.i = load i64, ptr %179, align 8, !dbg !5360, !noalias !5071, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5362), !dbg !5365
  %_16.i.i.i = icmp ult i64 %_11.sroa.0.0.i168.i.i, %_314.1.i87.i.i
  br i1 %_16.i.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i.i.split.us, label %panic1.i156.i.i

panic1.i170.i.i:                                  ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i168.i.i, i64 noundef range(i64 0, 2305843009213693952) %_313.1.i83.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !5366, !noalias !5368
  unreachable, !dbg !5366

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i.i.split.us
  %286 = getelementptr inbounds nuw float, ptr %_314.0.i86.i.i, i64 %_11.sroa.0.0.i168.i.i
  %_8.i.i.i.us.le = load float, ptr %286, align 4, !alias.scope !5362, !noalias !5369, !noundef !11
  %287 = tail call noundef float @llvm.fabs.f32(float %_8.i206.i.i.us.le), !dbg !5371
  %_3.i.i5366 = fcmp ule float %287, 0x3E45798EE0000000, !dbg !5375
  %_6.i.i5368 = bitcast float %287 to i32, !dbg !5381
  %_4.i.i5372 = select i1 %_3.i.i5366, i32 841731191, i32 %_6.i.i5368, !dbg !5384
  %_0.i.i5373 = bitcast i32 %_4.i.i5372 to float, !dbg !5385
  %_3.i.i5018 = fcmp ule float %_0.i.i5373, 0x3810000000000000, !dbg !5387
  %_4.i.i5024 = select i1 %_3.i.i5018, i32 8388608, i32 %_4.i.i5372, !dbg !5392
  %_5.i3893 = and i32 %_4.i.i5024, 8388607, !dbg !5394
  %_4.i3894 = or disjoint i32 %_5.i3893, 1065353216, !dbg !5394
  %significand.i3895 = bitcast i32 %_4.i3894 to float, !dbg !5396
  %_0.i3357 = fadd float %significand.i3895, -1.000000e+00, !dbg !5398
  %_0.i3034 = fmul float %_0.i3357, 0xBF9B17A960000000, !dbg !5400
  %_0.i2554 = fadd float %_0.i3034, 0x3FBF9A8440000000, !dbg !5402
  %_0.i3034.1 = fmul float %_0.i3357, %_0.i2554, !dbg !5400
  %_0.i2554.1 = fadd float %_0.i3034.1, 0xBFD1E3F400000000, !dbg !5402
  %_0.i3034.2 = fmul float %_0.i3357, %_0.i2554.1, !dbg !5400
  %_0.i2554.2 = fadd float %_0.i3034.2, 0x3FDD544F20000000, !dbg !5402
  %_0.i3034.3 = fmul float %_0.i3357, %_0.i2554.2, !dbg !5400
  %_0.i2554.3 = fadd float %_0.i3034.3, 0xBFE6FC2A60000000, !dbg !5402
  %_0.i3034.4 = fmul float %_0.i3357, %_0.i2554.3, !dbg !5400
  %_0.i2554.4 = fadd float %_0.i3034.4, 0x3FF714B2A0000000, !dbg !5402
  %_9.i3896 = lshr i32 %_4.i.i5024, 23, !dbg !5404
  %_8.i3897 = or disjoint i32 %_9.i3896, 1258291200, !dbg !5404
  %_7.i3898 = bitcast i32 %_8.i3897 to float, !dbg !5405
  %exponent.i3899 = fadd float %_7.i3898, 0xC160000FE0000000, !dbg !5407
  %_0.i3033 = fmul float %_0.i3357, %_0.i2554.4, !dbg !5408
  %_0.i2553 = fadd float %exponent.i3899, %_0.i3033, !dbg !5410
  %_0.i3276 = fmul float %_0.i2553, 0x4018151820000000, !dbg !5412
  %_3.i.i5358.inv = fcmp ogt float %_0.i3276, -1.600000e+02, !dbg !5414
  %_0.i.i5365 = select i1 %_3.i.i5358.inv, float %_0.i3276, float -1.600000e+02, !dbg !5414
  %_3.i.i6100.inv = fcmp olt float %_0.i.i5365, 2.400000e+01, !dbg !5417
  %_0.i.i6107 = select i1 %_3.i.i6100.inv, float %_0.i.i5365, float 2.400000e+01, !dbg !5417
  %_0.i3405 = fsub float %_0.i.i6107, %_0.i2782, !dbg !5420
  %_3.i2265 = fcmp ule float %_0.i3405, 3.000000e+00, !dbg !5423
  %_0.i2629 = fadd float %_0.i3405, 3.000000e+00, !dbg !5425
  %_0.i3195 = fmul float %_0.i2629, %_0.i2629, !dbg !5427
  %_0.i3194 = fmul float %_0.i3195, 0x3FB5555560000000, !dbg !5429
  %_4.i4656.v.v = select i1 %_3.i2265, float %_0.i3194, float %_0.i3405, !dbg !5431
  %_4.i4656.v = fmul float %coefficients.i.i.sroa.0.0.copyload, %_4.i4656.v.v, !dbg !5431
  %_4.i4656 = bitcast float %_4.i4656.v to i32, !dbg !5431
  %288 = fcmp ugt float %_0.i3405, -3.000000e+00, !dbg !5433
  %_7.i4648 = select i1 %288, i32 %_4.i4656, i32 0, !dbg !5435
  %_0.i4650 = bitcast i32 %_7.i4648 to float, !dbg !5436
  %_3.i.i5350 = fcmp ule float %_0.i4650, -1.000000e+02, !dbg !5438
  %289 = bitcast i32 %_7.i4648 to float, !dbg !5441
  %_0.i.i5357 = select i1 %_3.i.i5350, float -1.000000e+02, float %289, !dbg !5444
  %_3.i.i6092 = fcmp olt float %_0.i.i5357, 0.000000e+00, !dbg !5445
  %_0.i.i6099 = select i1 %_3.i.i6092, float %_0.i.i5357, float 0.000000e+00, !dbg !5448
  %290 = bitcast i32 %gain_near.i15.i.i.sroa.0.011208 to float, !dbg !5450
  %_3.i2431 = fcmp uge float %_0.i.i6099, %290, !dbg !5451
  %_4.i4722.v = select i1 %_3.i2431, float %coefficients.i.i.sroa.7.0.copyload, float %coefficients.i.i.sroa.5.0.copyload, !dbg !5454
  %_0.i3448 = fsub float %290, %_0.i.i6099, !dbg !5456
  %_4.i2951 = fmul float %_0.i3448, %_4.i4722.v, !dbg !5458
  %_0.i2952 = fadd float %_0.i.i6099, %_4.i2951, !dbg !5458
  %291 = tail call noundef float @llvm.fabs.f32(float %_0.i2952), !dbg !5460
  %_4.i4301 = bitcast float %_0.i2952 to i32, !dbg !5463
  %292 = fcmp uge float %291, 0x3BC79CA100000000, !dbg !5466
  %_3.i4303 = select i1 %292, i32 %_4.i4301, i32 0, !dbg !5467
  %_0.i4304 = bitcast i32 %_3.i4303 to float, !dbg !5468
  %_0.i2788 = fadd float %_0.i2782.4, %_0.i4304, !dbg !5470
  %_0.i3275 = fmul float %_0.i2788, 0x3FC542A5A0000000, !dbg !5472
  %_3.i.i5210.inv = fcmp ogt float %_0.i3275, -1.260000e+02, !dbg !5475
  %_0.i.i5217 = select i1 %_3.i.i5210.inv, float %_0.i3275, float -1.260000e+02, !dbg !5475
  %_3.i.i6012.inv = fcmp olt float %_0.i.i5217, 1.270000e+02, !dbg !5479
  %_0.i.i6019 = select i1 %_3.i.i6012.inv, float %_0.i.i5217, float 1.270000e+02, !dbg !5479
  %293 = tail call noundef float @llvm.floor.f32(float %_0.i.i6019), !dbg !5482
  %_0.i3381 = fsub float %_0.i.i6019, %293, !dbg !5486
  %294 = tail call noundef float @llvm.fabs.f32(float %_8.i189.i.i.us.le), !dbg !5488
  %_3.i.i5342 = fcmp ule float %294, 0x3E45798EE0000000, !dbg !5491
  %_6.i.i5344 = bitcast float %294 to i32, !dbg !5496
  %_4.i.i5348 = select i1 %_3.i.i5342, i32 841731191, i32 %_6.i.i5344, !dbg !5499
  %_0.i.i5349 = bitcast i32 %_4.i.i5348 to float, !dbg !5500
  %_3.i.i5026 = fcmp ule float %_0.i.i5349, 0x3810000000000000, !dbg !5502
  %_4.i.i5032 = select i1 %_3.i.i5026, i32 8388608, i32 %_4.i.i5348, !dbg !5507
  %_5.i3901 = and i32 %_4.i.i5032, 8388607, !dbg !5509
  %_4.i3902 = or disjoint i32 %_5.i3901, 1065353216, !dbg !5509
  %significand.i3903 = bitcast i32 %_4.i3902 to float, !dbg !5511
  %_0.i3358 = fadd float %significand.i3903, -1.000000e+00, !dbg !5513
  %_0.i3036 = fmul float %_0.i3358, 0xBF9B17A960000000, !dbg !5515
  %_0.i2556 = fadd float %_0.i3036, 0x3FBF9A8440000000, !dbg !5517
  %_0.i3036.1 = fmul float %_0.i3358, %_0.i2556, !dbg !5515
  %_0.i2556.1 = fadd float %_0.i3036.1, 0xBFD1E3F400000000, !dbg !5517
  %_0.i3036.2 = fmul float %_0.i3358, %_0.i2556.1, !dbg !5515
  %_0.i2556.2 = fadd float %_0.i3036.2, 0x3FDD544F20000000, !dbg !5517
  %_0.i3036.3 = fmul float %_0.i3358, %_0.i2556.2, !dbg !5515
  %_0.i2556.3 = fadd float %_0.i3036.3, 0xBFE6FC2A60000000, !dbg !5517
  %_0.i3036.4 = fmul float %_0.i3358, %_0.i2556.3, !dbg !5515
  %_0.i2556.4 = fadd float %_0.i3036.4, 0x3FF714B2A0000000, !dbg !5517
  %_9.i3904 = lshr i32 %_4.i.i5032, 23, !dbg !5519
  %_8.i3905 = or disjoint i32 %_9.i3904, 1258291200, !dbg !5519
  %_7.i3906 = bitcast i32 %_8.i3905 to float, !dbg !5520
  %exponent.i3907 = fadd float %_7.i3906, 0xC160000FE0000000, !dbg !5522
  %_0.i3035 = fmul float %_0.i3358, %_0.i2556.4, !dbg !5523
  %_0.i2555 = fadd float %exponent.i3907, %_0.i3035, !dbg !5525
  %_0.i3274 = fmul float %_0.i2555, 0x4018151820000000, !dbg !5527
  %_3.i.i5334.inv = fcmp ogt float %_0.i3274, -1.600000e+02, !dbg !5529
  %_0.i.i5341 = select i1 %_3.i.i5334.inv, float %_0.i3274, float -1.600000e+02, !dbg !5529
  %_3.i.i6084.inv = fcmp olt float %_0.i.i5341, 2.400000e+01, !dbg !5532
  %_0.i.i6091 = select i1 %_3.i.i6084.inv, float %_0.i.i5341, float 2.400000e+01, !dbg !5532
  %_0.i3406 = fsub float %_0.i.i6091, %_0.i2782.5, !dbg !5535
  %_3.i2267 = fcmp ule float %_0.i3406, 3.000000e+00, !dbg !5538
  %_0.i2630 = fadd float %_0.i3406, 3.000000e+00, !dbg !5540
  %_0.i3199 = fmul float %_0.i2630, %_0.i2630, !dbg !5542
  %_0.i3198 = fmul float %_0.i3199, 0x3FB5555560000000, !dbg !5544
  %_4.i4669.v.v = select i1 %_3.i2267, float %_0.i3198, float %_0.i3406, !dbg !5546
  %_4.i4669.v = fmul float %coefficients.i.i.sroa.9.0.copyload, %_4.i4669.v.v, !dbg !5546
  %_4.i4669 = bitcast float %_4.i4669.v to i32, !dbg !5546
  %295 = fcmp ugt float %_0.i3406, -3.000000e+00, !dbg !5548
  %_7.i4661 = select i1 %295, i32 %_4.i4669, i32 0, !dbg !5550
  %_0.i4663 = bitcast i32 %_7.i4661 to float, !dbg !5551
  %_3.i.i5326 = fcmp ule float %_0.i4663, -1.000000e+02, !dbg !5553
  %296 = bitcast i32 %_7.i4661 to float, !dbg !5556
  %_0.i.i5333 = select i1 %_3.i.i5326, float -1.000000e+02, float %296, !dbg !5559
  %_3.i.i6076 = fcmp olt float %_0.i.i5333, 0.000000e+00, !dbg !5560
  %_0.i.i6083 = select i1 %_3.i.i6076, float %_0.i.i5333, float 0.000000e+00, !dbg !5563
  %297 = bitcast i32 %gain_near.i15.i.i.sroa.6.011209 to float, !dbg !5565
  %_3.i2427 = fcmp uge float %_0.i.i6083, %297, !dbg !5566
  %_4.i4715.v = select i1 %_3.i2427, float %coefficients.i.i.sroa.13.0.copyload, float %coefficients.i.i.sroa.11.0.copyload, !dbg !5569
  %_0.i3447 = fsub float %297, %_0.i.i6083, !dbg !5571
  %_4.i2949 = fmul float %_0.i3447, %_4.i4715.v, !dbg !5573
  %_0.i2950 = fadd float %_0.i.i6083, %_4.i2949, !dbg !5573
  %298 = tail call noundef float @llvm.fabs.f32(float %_0.i2950), !dbg !5575
  %_4.i4297 = bitcast float %_0.i2950 to i32, !dbg !5578
  %299 = fcmp uge float %298, 0x3BC79CA100000000, !dbg !5581
  %_3.i4299 = select i1 %299, i32 %_4.i4297, i32 0, !dbg !5582
  %_0.i4300 = bitcast i32 %_3.i4299 to float, !dbg !5583
  %_0.i2787 = fadd float %_0.i2782.9, %_0.i4300, !dbg !5585
  %_0.i3273 = fmul float %_0.i2787, 0x3FC542A5A0000000, !dbg !5587
  %_3.i.i5218.inv = fcmp ogt float %_0.i3273, -1.260000e+02, !dbg !5590
  %_0.i.i5225 = select i1 %_3.i.i5218.inv, float %_0.i3273, float -1.260000e+02, !dbg !5590
  %_3.i.i6020.inv = fcmp olt float %_0.i.i5225, 1.270000e+02, !dbg !5594
  %_0.i.i6027 = select i1 %_3.i.i6020.inv, float %_0.i.i5225, float 1.270000e+02, !dbg !5594
  %300 = tail call noundef float @llvm.floor.f32(float %_0.i.i6027), !dbg !5597
  %_0.i3382 = fsub float %_0.i.i6027, %300, !dbg !5601
  %301 = tail call noundef float @llvm.fabs.f32(float %_8.i172.i.i.us.le), !dbg !5603
  %_3.i.i5318 = fcmp ule float %301, 0x3E45798EE0000000, !dbg !5605
  %_6.i.i5320 = bitcast float %301 to i32, !dbg !5610
  %_4.i.i5324 = select i1 %_3.i.i5318, i32 841731191, i32 %_6.i.i5320, !dbg !5613
  %_0.i.i5325 = bitcast i32 %_4.i.i5324 to float, !dbg !5614
  %_3.i.i5034 = fcmp ule float %_0.i.i5325, 0x3810000000000000, !dbg !5616
  %_4.i.i5040 = select i1 %_3.i.i5034, i32 8388608, i32 %_4.i.i5324, !dbg !5621
  %_5.i3909 = and i32 %_4.i.i5040, 8388607, !dbg !5623
  %_4.i3910 = or disjoint i32 %_5.i3909, 1065353216, !dbg !5623
  %significand.i3911 = bitcast i32 %_4.i3910 to float, !dbg !5625
  %_0.i3359 = fadd float %significand.i3911, -1.000000e+00, !dbg !5627
  %_0.i3038 = fmul float %_0.i3359, 0xBF9B17A960000000, !dbg !5629
  %_0.i2558 = fadd float %_0.i3038, 0x3FBF9A8440000000, !dbg !5631
  %_0.i3038.1 = fmul float %_0.i3359, %_0.i2558, !dbg !5629
  %_0.i2558.1 = fadd float %_0.i3038.1, 0xBFD1E3F400000000, !dbg !5631
  %_0.i3038.2 = fmul float %_0.i3359, %_0.i2558.1, !dbg !5629
  %_0.i2558.2 = fadd float %_0.i3038.2, 0x3FDD544F20000000, !dbg !5631
  %_0.i3038.3 = fmul float %_0.i3359, %_0.i2558.2, !dbg !5629
  %_0.i2558.3 = fadd float %_0.i3038.3, 0xBFE6FC2A60000000, !dbg !5631
  %_0.i3038.4 = fmul float %_0.i3359, %_0.i2558.3, !dbg !5629
  %_0.i2558.4 = fadd float %_0.i3038.4, 0x3FF714B2A0000000, !dbg !5631
  %_9.i3912 = lshr i32 %_4.i.i5040, 23, !dbg !5633
  %_8.i3913 = or disjoint i32 %_9.i3912, 1258291200, !dbg !5633
  %_7.i3914 = bitcast i32 %_8.i3913 to float, !dbg !5634
  %exponent.i3915 = fadd float %_7.i3914, 0xC160000FE0000000, !dbg !5636
  %_0.i3037 = fmul float %_0.i3359, %_0.i2558.4, !dbg !5637
  %_0.i2557 = fadd float %exponent.i3915, %_0.i3037, !dbg !5639
  %_0.i3272 = fmul float %_0.i2557, 0x4018151820000000, !dbg !5641
  %_3.i.i5310.inv = fcmp ogt float %_0.i3272, -1.600000e+02, !dbg !5643
  %_0.i.i5317 = select i1 %_3.i.i5310.inv, float %_0.i3272, float -1.600000e+02, !dbg !5643
  %_3.i.i6068.inv = fcmp olt float %_0.i.i5317, 2.400000e+01, !dbg !5646
  %_0.i.i6075 = select i1 %_3.i.i6068.inv, float %_0.i.i5317, float 2.400000e+01, !dbg !5646
  %_0.i3407 = fsub float %_0.i.i6075, %_0.i2781, !dbg !5649
  %_3.i2269 = fcmp ule float %_0.i3407, 3.000000e+00, !dbg !5652
  %_0.i2631 = fadd float %_0.i3407, 3.000000e+00, !dbg !5654
  %_0.i3203 = fmul float %_0.i2631, %_0.i2631, !dbg !5656
  %_0.i3202 = fmul float %_0.i3203, 0x3FB5555560000000, !dbg !5658
  %_4.i4682.v.v = select i1 %_3.i2269, float %_0.i3202, float %_0.i3407, !dbg !5660
  %_4.i4682.v = fmul float %coefficients.i.i.sroa.15.24.copyload, %_4.i4682.v.v, !dbg !5660
  %_4.i4682 = bitcast float %_4.i4682.v to i32, !dbg !5660
  %302 = fcmp ugt float %_0.i3407, -3.000000e+00, !dbg !5662
  %_7.i4674 = select i1 %302, i32 %_4.i4682, i32 0, !dbg !5664
  %_0.i4676 = bitcast i32 %_7.i4674 to float, !dbg !5665
  %_3.i.i5302 = fcmp ule float %_0.i4676, -1.000000e+02, !dbg !5667
  %303 = bitcast i32 %_7.i4674 to float, !dbg !5670
  %_0.i.i5309 = select i1 %_3.i.i5302, float -1.000000e+02, float %303, !dbg !5673
  %_3.i.i6060 = fcmp olt float %_0.i.i5309, 0.000000e+00, !dbg !5674
  %_0.i.i6067 = select i1 %_3.i.i6060, float %_0.i.i5309, float 0.000000e+00, !dbg !5677
  %304 = bitcast i32 %gain_far.i14.i.i.sroa.0.011210 to float, !dbg !5679
  %_3.i2423 = fcmp uge float %_0.i.i6067, %304, !dbg !5680
  %_4.i4708.v = select i1 %_3.i2423, float %coefficients.i.i.sroa.20.24.copyload, float %coefficients.i.i.sroa.18.24.copyload, !dbg !5683
  %_0.i3446 = fsub float %304, %_0.i.i6067, !dbg !5685
  %_4.i2947 = fmul float %_0.i3446, %_4.i4708.v, !dbg !5687
  %_0.i2948 = fadd float %_0.i.i6067, %_4.i2947, !dbg !5687
  %305 = tail call noundef float @llvm.fabs.f32(float %_0.i2948), !dbg !5689
  %_4.i4293 = bitcast float %_0.i2948 to i32, !dbg !5692
  %306 = fcmp uge float %305, 0x3BC79CA100000000, !dbg !5695
  %_3.i4295 = select i1 %306, i32 %_4.i4293, i32 0, !dbg !5696
  %_0.i4296 = bitcast i32 %_3.i4295 to float, !dbg !5697
  %_0.i2786 = fadd float %_0.i2781.4, %_0.i4296, !dbg !5699
  %_0.i3271 = fmul float %_0.i2786, 0x3FC542A5A0000000, !dbg !5701
  %_3.i.i5226.inv = fcmp ogt float %_0.i3271, -1.260000e+02, !dbg !5704
  %_0.i.i5233 = select i1 %_3.i.i5226.inv, float %_0.i3271, float -1.260000e+02, !dbg !5704
  %_3.i.i6028.inv = fcmp olt float %_0.i.i5233, 1.270000e+02, !dbg !5708
  %_0.i.i6035 = select i1 %_3.i.i6028.inv, float %_0.i.i5233, float 1.270000e+02, !dbg !5708
  %307 = tail call noundef float @llvm.floor.f32(float %_0.i.i6035), !dbg !5711
  %_0.i3383 = fsub float %_0.i.i6035, %307, !dbg !5715
  %308 = tail call noundef float @llvm.fabs.f32(float %_8.i.i.i.us.le), !dbg !5717
  %_3.i.i5294 = fcmp ule float %308, 0x3E45798EE0000000, !dbg !5719
  %_6.i.i5296 = bitcast float %308 to i32, !dbg !5724
  %_4.i.i5300 = select i1 %_3.i.i5294, i32 841731191, i32 %_6.i.i5296, !dbg !5727
  %_0.i.i5301 = bitcast i32 %_4.i.i5300 to float, !dbg !5728
  %_3.i.i5042 = fcmp ule float %_0.i.i5301, 0x3810000000000000, !dbg !5730
  %_4.i.i5048 = select i1 %_3.i.i5042, i32 8388608, i32 %_4.i.i5300, !dbg !5735
  %_5.i3917 = and i32 %_4.i.i5048, 8388607, !dbg !5737
  %_4.i3918 = or disjoint i32 %_5.i3917, 1065353216, !dbg !5737
  %significand.i3919 = bitcast i32 %_4.i3918 to float, !dbg !5739
  %_0.i3360 = fadd float %significand.i3919, -1.000000e+00, !dbg !5741
  %_0.i3040 = fmul float %_0.i3360, 0xBF9B17A960000000, !dbg !5743
  %_0.i2560 = fadd float %_0.i3040, 0x3FBF9A8440000000, !dbg !5745
  %_0.i3040.1 = fmul float %_0.i3360, %_0.i2560, !dbg !5743
  %_0.i2560.1 = fadd float %_0.i3040.1, 0xBFD1E3F400000000, !dbg !5745
  %_0.i3040.2 = fmul float %_0.i3360, %_0.i2560.1, !dbg !5743
  %_0.i2560.2 = fadd float %_0.i3040.2, 0x3FDD544F20000000, !dbg !5745
  %_0.i3040.3 = fmul float %_0.i3360, %_0.i2560.2, !dbg !5743
  %_0.i2560.3 = fadd float %_0.i3040.3, 0xBFE6FC2A60000000, !dbg !5745
  %_0.i3040.4 = fmul float %_0.i3360, %_0.i2560.3, !dbg !5743
  %_0.i2560.4 = fadd float %_0.i3040.4, 0x3FF714B2A0000000, !dbg !5745
  %_9.i3920 = lshr i32 %_4.i.i5048, 23, !dbg !5747
  %_8.i3921 = or disjoint i32 %_9.i3920, 1258291200, !dbg !5747
  %_7.i3922 = bitcast i32 %_8.i3921 to float, !dbg !5748
  %exponent.i3923 = fadd float %_7.i3922, 0xC160000FE0000000, !dbg !5750
  %_0.i3039 = fmul float %_0.i3360, %_0.i2560.4, !dbg !5751
  %_0.i2559 = fadd float %exponent.i3923, %_0.i3039, !dbg !5753
  %_0.i3270 = fmul float %_0.i2559, 0x4018151820000000, !dbg !5755
  %_3.i.i5286.inv = fcmp ogt float %_0.i3270, -1.600000e+02, !dbg !5757
  %_0.i.i5293 = select i1 %_3.i.i5286.inv, float %_0.i3270, float -1.600000e+02, !dbg !5757
  %_3.i.i6052.inv = fcmp olt float %_0.i.i5293, 2.400000e+01, !dbg !5760
  %_0.i.i6059 = select i1 %_3.i.i6052.inv, float %_0.i.i5293, float 2.400000e+01, !dbg !5760
  %_0.i3408 = fsub float %_0.i.i6059, %_0.i2781.5, !dbg !5763
  %_3.i2271 = fcmp ule float %_0.i3408, 3.000000e+00, !dbg !5766
  %_0.i2632 = fadd float %_0.i3408, 3.000000e+00, !dbg !5768
  %_0.i3207 = fmul float %_0.i2632, %_0.i2632, !dbg !5770
  %_0.i3206 = fmul float %_0.i3207, 0x3FB5555560000000, !dbg !5772
  %_4.i4695.v.v = select i1 %_3.i2271, float %_0.i3206, float %_0.i3408, !dbg !5774
  %_4.i4695.v = fmul float %coefficients.i.i.sroa.22.24.copyload, %_4.i4695.v.v, !dbg !5774
  %_4.i4695 = bitcast float %_4.i4695.v to i32, !dbg !5774
  %309 = fcmp ugt float %_0.i3408, -3.000000e+00, !dbg !5776
  %_7.i4687 = select i1 %309, i32 %_4.i4695, i32 0, !dbg !5778
  %_0.i4689 = bitcast i32 %_7.i4687 to float, !dbg !5779
  %_3.i.i5278 = fcmp ule float %_0.i4689, -1.000000e+02, !dbg !5781
  %310 = bitcast i32 %_7.i4687 to float, !dbg !5784
  %_0.i.i5285 = select i1 %_3.i.i5278, float -1.000000e+02, float %310, !dbg !5787
  %_3.i.i6044 = fcmp olt float %_0.i.i5285, 0.000000e+00, !dbg !5788
  %_0.i.i6051 = select i1 %_3.i.i6044, float %_0.i.i5285, float 0.000000e+00, !dbg !5791
  %311 = bitcast i32 %gain_far.i14.i.i.sroa.6.011211 to float, !dbg !5793
  %_3.i2419 = fcmp uge float %_0.i.i6051, %311, !dbg !5794
  %_4.i4701.v = select i1 %_3.i2419, float %coefficients.i.i.sroa.26.24.copyload, float %coefficients.i.i.sroa.24.24.copyload, !dbg !5797
  %_0.i3445 = fsub float %311, %_0.i.i6051, !dbg !5799
  %_4.i2945 = fmul float %_0.i3445, %_4.i4701.v, !dbg !5801
  %_0.i2946 = fadd float %_0.i.i6051, %_4.i2945, !dbg !5801
  %312 = tail call noundef float @llvm.fabs.f32(float %_0.i2946), !dbg !5803
  %_4.i4289 = bitcast float %_0.i2946 to i32, !dbg !5806
  %313 = fcmp uge float %312, 0x3BC79CA100000000, !dbg !5809
  %_3.i4291 = select i1 %313, i32 %_4.i4289, i32 0, !dbg !5810
  %_0.i4292 = bitcast i32 %_3.i4291 to float, !dbg !5811
  %_0.i2785 = fadd float %_0.i2781.9, %_0.i4292, !dbg !5813
  %_0.i3269 = fmul float %_0.i2785, 0x3FC542A5A0000000, !dbg !5815
  %_3.i.i5234.inv = fcmp ogt float %_0.i3269, -1.260000e+02, !dbg !5818
  %_0.i.i5241 = select i1 %_3.i.i5234.inv, float %_0.i3269, float -1.260000e+02, !dbg !5818
  %_3.i.i6036.inv = fcmp olt float %_0.i.i5241, 1.270000e+02, !dbg !5822
  %_0.i.i6043 = select i1 %_3.i.i6036.inv, float %_0.i.i5241, float 1.270000e+02, !dbg !5822
  %314 = tail call noundef float @llvm.floor.f32(float %_0.i.i6043), !dbg !5825
  %_0.i3384 = fsub float %_0.i.i6043, %314, !dbg !5829
  %_0.i3112 = fmul float %_0.i3384, 0x3F5E974FA0000000, !dbg !5831
  %_0.i2608 = fadd float %_0.i3112, 0x3F82778560000000, !dbg !5833
  %_0.i3112.1 = fmul float %_0.i3384, %_0.i2608, !dbg !5831
  %_0.i2608.1 = fadd float %_0.i3112.1, 0x3FAC91CE60000000, !dbg !5833
  %_0.i3112.2 = fmul float %_0.i3384, %_0.i2608.1, !dbg !5831
  %_0.i2608.2 = fadd float %_0.i3112.2, 0x3FCEBDB560000000, !dbg !5833
  %_0.i3112.3 = fmul float %_0.i3384, %_0.i2608.2, !dbg !5831
  %_0.i2608.3 = fadd float %_0.i3112.3, 0x3FE62E4BA0000000, !dbg !5833
  %_0.i3109 = fmul float %_0.i3383, 0x3F5E974FA0000000, !dbg !5835
  %_0.i2606 = fadd float %_0.i3109, 0x3F82778560000000, !dbg !5837
  %_0.i3109.1 = fmul float %_0.i3383, %_0.i2606, !dbg !5835
  %_0.i2606.1 = fadd float %_0.i3109.1, 0x3FAC91CE60000000, !dbg !5837
  %_0.i3109.2 = fmul float %_0.i3383, %_0.i2606.1, !dbg !5835
  %_0.i2606.2 = fadd float %_0.i3109.2, 0x3FCEBDB560000000, !dbg !5837
  %_0.i3109.3 = fmul float %_0.i3383, %_0.i2606.2, !dbg !5835
  %_0.i2606.3 = fadd float %_0.i3109.3, 0x3FE62E4BA0000000, !dbg !5837
  %_0.i3106 = fmul float %_0.i3382, 0x3F5E974FA0000000, !dbg !5839
  %_0.i2604 = fadd float %_0.i3106, 0x3F82778560000000, !dbg !5841
  %_0.i3106.1 = fmul float %_0.i3382, %_0.i2604, !dbg !5839
  %_0.i2604.1 = fadd float %_0.i3106.1, 0x3FAC91CE60000000, !dbg !5841
  %_0.i3106.2 = fmul float %_0.i3382, %_0.i2604.1, !dbg !5839
  %_0.i2604.2 = fadd float %_0.i3106.2, 0x3FCEBDB560000000, !dbg !5841
  %_0.i3106.3 = fmul float %_0.i3382, %_0.i2604.2, !dbg !5839
  %_0.i2604.3 = fadd float %_0.i3106.3, 0x3FE62E4BA0000000, !dbg !5841
  %_0.i3103 = fmul float %_0.i3381, 0x3F5E974FA0000000, !dbg !5843
  %_0.i2602 = fadd float %_0.i3103, 0x3F82778560000000, !dbg !5845
  %_0.i3103.1 = fmul float %_0.i3381, %_0.i2602, !dbg !5843
  %_0.i2602.1 = fadd float %_0.i3103.1, 0x3FAC91CE60000000, !dbg !5845
  %_0.i3103.2 = fmul float %_0.i3381, %_0.i2602.1, !dbg !5843
  %_0.i2602.2 = fadd float %_0.i3103.2, 0x3FCEBDB560000000, !dbg !5845
  %_0.i3103.3 = fmul float %_0.i3381, %_0.i2602.2, !dbg !5843
  %_0.i2602.3 = fadd float %_0.i3103.3, 0x3FE62E4BA0000000, !dbg !5845
  %_0.i3102 = fmul float %_0.i3381, %_0.i2602.3, !dbg !5847
  %_0.i2601 = fadd float %_0.i3102, 1.000000e+00, !dbg !5849
  %biased.i2210 = fadd float %293, 0x4160000FE0000000, !dbg !5851
  %_4.i2211 = bitcast float %biased.i2210 to i32, !dbg !5853
  %_3.i2212 = shl i32 %_4.i2211, 23, !dbg !5855
  %_0.i2213 = bitcast i32 %_3.i2212 to float, !dbg !5856
  %_0.i3101 = fmul float %_0.i2601, %_0.i2213, !dbg !5858
  %_0.i3105 = fmul float %_0.i3382, %_0.i2604.3, !dbg !5860
  %_0.i2603 = fadd float %_0.i3105, 1.000000e+00, !dbg !5862
  %biased.i2214 = fadd float %300, 0x4160000FE0000000, !dbg !5864
  %_4.i2215 = bitcast float %biased.i2214 to i32, !dbg !5866
  %_3.i2216 = shl i32 %_4.i2215, 23, !dbg !5868
  %_0.i2217 = bitcast i32 %_3.i2216 to float, !dbg !5869
  %_0.i3104 = fmul float %_0.i2603, %_0.i2217, !dbg !5871
  %_0.i3108 = fmul float %_0.i3383, %_0.i2606.3, !dbg !5873
  %_0.i2605 = fadd float %_0.i3108, 1.000000e+00, !dbg !5875
  %biased.i2218 = fadd float %307, 0x4160000FE0000000, !dbg !5877
  %_4.i2219 = bitcast float %biased.i2218 to i32, !dbg !5879
  %_3.i2220 = shl i32 %_4.i2219, 23, !dbg !5881
  %_0.i2221 = bitcast i32 %_3.i2220 to float, !dbg !5882
  %_0.i3107 = fmul float %_0.i2605, %_0.i2221, !dbg !5884
  %_0.i3111 = fmul float %_0.i3384, %_0.i2608.3, !dbg !5886
  %_0.i2607 = fadd float %_0.i3111, 1.000000e+00, !dbg !5888
  %biased.i2222 = fadd float %314, 0x4160000FE0000000, !dbg !5890
  %_4.i2223 = bitcast float %biased.i2222 to i32, !dbg !5892
  %_3.i2224 = shl i32 %_4.i2223, 23, !dbg !5894
  %_0.i2225 = bitcast i32 %_3.i2224 to float, !dbg !5895
  %_0.i3110 = fmul float %_0.i2607, %_0.i2225, !dbg !5897
  %_262.i110.i.i = icmp ugt i64 %_37.sroa.0.0.i36.i.i, %_311.1.i76.i.i, !dbg !5899
  br i1 %_262.i110.i.i, label %bb84.i147.i.i, label %bb85.i111.i.i, !dbg !5899, !prof !1664

panic1.i156.i.i:                                  ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i168.i.i, i64 noundef range(i64 0, 2305843009213693952) %_314.1.i87.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !5904, !noalias !5906
  unreachable, !dbg !5904

bb82.i148.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3937
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i.i11212, i64 noundef %_310.1.i69.i.i, i64 noundef %_310.1.i69.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_007bf1cfdcf9f845db5ef166fc9a1790) #26, !dbg !5907, !noalias !5071
  unreachable, !dbg !5907

bb85.i111.i.i:                                    ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i.i.split.us
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5908), !dbg !5911
  %_3.not.i3482 = icmp eq i64 %_311.1.i76.i.i, %_37.sroa.0.0.i36.i.i, !dbg !5912
  br i1 %_3.not.i3482, label %panic.i3485, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3486, !dbg !5912

panic.i3485:                                      ; preds = %bb85.i111.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5912, !noalias !5914
  unreachable, !dbg !5912

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3486: ; preds = %bb85.i111.i.i
  %_269.i114.i.i = getelementptr inbounds nuw float, ptr %_311.0.i75.i.i, i64 %_37.sroa.0.0.i36.i.i, !dbg !5915
  %_0.i3484 = load float, ptr %_269.i114.i.i, align 4, !dbg !5912, !alias.scope !5908, !noalias !5071, !noundef !11
  %_0.i3268 = fmul float %_0.i3101, %_0.i3484, !dbg !5920
  %_270.i118.i.i = icmp ugt i64 %_37.sroa.0.0.i36.i.i, %_312.1.i80.i.i, !dbg !5922
  br i1 %_270.i118.i.i, label %bb86.i146.i.i, label %bb87.i119.i.i, !dbg !5922, !prof !1664

bb84.i147.i.i:                                    ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i.i.split.us
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i.i, i64 noundef %_311.1.i76.i.i, i64 noundef %_311.1.i76.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a45b75cd2d07085fe69fe186ba115725) #26, !dbg !5926, !noalias !5071
  unreachable, !dbg !5926

bb87.i119.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3486
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5927), !dbg !5930
  %_3.not.i3477 = icmp eq i64 %_312.1.i80.i.i, %_37.sroa.0.0.i36.i.i, !dbg !5931
  br i1 %_3.not.i3477, label %panic.i3480, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3481, !dbg !5931

panic.i3480:                                      ; preds = %bb87.i119.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5931, !noalias !5933
  unreachable, !dbg !5931

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3481: ; preds = %bb87.i119.i.i
  %_277.i122.i.i = getelementptr inbounds nuw float, ptr %_312.0.i79.i.i, i64 %_37.sroa.0.0.i36.i.i, !dbg !5934
  %_0.i3479 = load float, ptr %_277.i122.i.i, align 4, !dbg !5931, !alias.scope !5927, !noalias !5071, !noundef !11
  %_0.i3267 = fmul float %_0.i3104, %_0.i3479, !dbg !5939
  %_0.i2784 = fadd float %_0.i3268, %_0.i3267, !dbg !5941
  %_278.i127.i.i = icmp ugt i64 %_37.sroa.0.0.i36.i.i, %_313.1.i83.i.i, !dbg !5943
  br i1 %_278.i127.i.i, label %bb88.i145.i.i, label %bb89.i128.i.i, !dbg !5943, !prof !1664

bb86.i146.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3486
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i.i, i64 noundef %_312.1.i80.i.i, i64 noundef %_312.1.i80.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_36be93341d7083c8a492c530428430ea) #26, !dbg !5948, !noalias !5071
  unreachable, !dbg !5948

bb89.i128.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3481
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5949), !dbg !5952
  %_3.not.i3472 = icmp eq i64 %_313.1.i83.i.i, %_37.sroa.0.0.i36.i.i, !dbg !5953
  br i1 %_3.not.i3472, label %panic.i3475, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3476, !dbg !5953

panic.i3475:                                      ; preds = %bb89.i128.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5953, !noalias !5955
  unreachable, !dbg !5953

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3476: ; preds = %bb89.i128.i.i
  %_285.i131.i.i = getelementptr inbounds nuw float, ptr %_313.0.i82.i.i, i64 %_37.sroa.0.0.i36.i.i, !dbg !5956
  %_0.i3474 = load float, ptr %_285.i131.i.i, align 4, !dbg !5953, !alias.scope !5949, !noalias !5071, !noundef !11
  %_0.i3266 = fmul float %_0.i3107, %_0.i3474, !dbg !5961
  %_286.i135.i.i = icmp ugt i64 %_37.sroa.0.0.i36.i.i, %_314.1.i87.i.i, !dbg !5963
  br i1 %_286.i135.i.i, label %bb90.i144.i.i, label %bb91.i136.i.i, !dbg !5963, !prof !1664

bb88.i145.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3481
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i.i, i64 noundef %_313.1.i83.i.i, i64 noundef %_313.1.i83.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a6f1b2a44d3e33eba5c708677237c81) #26, !dbg !5967, !noalias !5071
  unreachable, !dbg !5967

bb91.i136.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3476
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5968), !dbg !5971
  %_3.not.i = icmp eq i64 %_314.1.i87.i.i, %_37.sroa.0.0.i36.i.i, !dbg !5972
  br i1 %_3.not.i, label %panic.i3471, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit, !dbg !5972

panic.i3471:                                      ; preds = %bb91.i136.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5972, !noalias !5974
  unreachable, !dbg !5972

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit: ; preds = %bb91.i136.i.i
  %_293.i139.i.i = getelementptr inbounds nuw float, ptr %_314.0.i86.i.i, i64 %_37.sroa.0.0.i36.i.i, !dbg !5975
  %_0.i3470 = load float, ptr %_293.i139.i.i, align 4, !dbg !5972, !alias.scope !5968, !noalias !5071, !noundef !11
  %_0.i3265 = fmul float %_0.i3110, %_0.i3470, !dbg !5980
  %_0.i2783 = fadd float %_0.i3266, %_0.i3265, !dbg !5982
  store float %_0.i2784, ptr %_180.i38.i.i, align 4, !dbg !5984, !alias.scope !5987, !noalias !5071
  store float %_0.i2783, ptr %_188.i43.i.i, align 4, !dbg !5990, !alias.scope !5992, !noalias !5071
  %exitcond13935.not = icmp eq i64 %263, %plan.0.i.i, !dbg !5025
  br i1 %exitcond13935.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_Kb1_EB2_.exit.i.i, label %bb54.i33.i.i, !dbg !5035

bb90.i144.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3476
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i.i, i64 noundef %_314.1.i87.i.i, i64 noundef %_314.1.i87.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4aa2eaec3d1833a4a887fe1d76c05ca7) #26, !dbg !5995, !noalias !5071
  unreachable, !dbg !5995

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_Kb1_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit, %bb17.i.i
  %segments.i.i.sroa.112.0 = phi float [ %_12.le.9.i6451, %bb17.i.i ], [ %_0.i2781.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.108.0 = phi float [ %_12.le.8.i6449, %bb17.i.i ], [ %_0.i2781.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.104.0 = phi float [ %_12.le.7.i6447, %bb17.i.i ], [ %_0.i2781.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.100.0 = phi float [ %_12.le.6.i6445, %bb17.i.i ], [ %_0.i2781.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.94.0 = phi float [ %_12.le.5.i6443, %bb17.i.i ], [ %_0.i2781.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.88.0 = phi float [ %_12.le.4.i6441, %bb17.i.i ], [ %_0.i2781.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.84.0 = phi float [ %_12.le.3.i6439, %bb17.i.i ], [ %_0.i2781.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.80.0 = phi float [ %_12.le.2.i6437, %bb17.i.i ], [ %_0.i2781.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.76.0 = phi float [ %_12.le.1.i6435, %bb17.i.i ], [ %_0.i2781.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.70.0 = phi float [ %_12.le.i6433, %bb17.i.i ], [ %_0.i2781, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.44.0 = phi float [ %_12.le.9.i, %bb17.i.i ], [ %_0.i2782.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.40.0 = phi float [ %_12.le.8.i, %bb17.i.i ], [ %_0.i2782.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.36.0 = phi float [ %_12.le.7.i, %bb17.i.i ], [ %_0.i2782.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.32.0 = phi float [ %_12.le.6.i, %bb17.i.i ], [ %_0.i2782.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.26.0 = phi float [ %_12.le.5.i, %bb17.i.i ], [ %_0.i2782.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.20.0 = phi float [ %_12.le.4.i, %bb17.i.i ], [ %_0.i2782.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.16.0 = phi float [ %_12.le.3.i, %bb17.i.i ], [ %_0.i2782.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.12.0 = phi float [ %_12.le.2.i, %bb17.i.i ], [ %_0.i2782.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.8.0 = phi float [ %_12.le.1.i, %bb17.i.i ], [ %_0.i2782.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %segments.i.i.sroa.0.0 = phi float [ %_12.le.i, %bb17.i.i ], [ %_0.i2782, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5037
  %filter_near.i17.i.i.sroa.0.0.lcssa = phi float [ %filter_near.i17.i.i.sroa.0.0.copyload, %bb17.i.i ], [ %_0.i4120, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5996
  %filter_near.i17.i.i.sroa.7.0.lcssa = phi float [ %filter_near.i17.i.i.sroa.7.0.copyload, %bb17.i.i ], [ %_0.i4116, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5996
  %filter_near.i17.i.i.sroa.11.0.lcssa = phi float [ %filter_near.i17.i.i.sroa.11.0.copyload, %bb17.i.i ], [ %_0.i4128, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5996
  %filter_near.i17.i.i.sroa.14.0.lcssa = phi float [ %filter_near.i17.i.i.sroa.14.0.copyload, %bb17.i.i ], [ %_0.i4124, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5996
  %filter_far.i16.i.i.sroa.0.0.lcssa = phi float [ %filter_far.i16.i.i.sroa.0.0.copyload, %bb17.i.i ], [ %_0.i4104, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5997
  %filter_far.i16.i.i.sroa.7.0.lcssa = phi float [ %filter_far.i16.i.i.sroa.7.0.copyload, %bb17.i.i ], [ %_0.i4100, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5997
  %filter_far.i16.i.i.sroa.11.0.lcssa = phi float [ %filter_far.i16.i.i.sroa.11.0.copyload, %bb17.i.i ], [ %_0.i4112, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5997
  %filter_far.i16.i.i.sroa.14.0.lcssa = phi float [ %filter_far.i16.i.i.sroa.14.0.copyload, %bb17.i.i ], [ %_0.i4108, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5997
  %gain_near.i15.i.i.sroa.0.0.lcssa = phi i32 [ %258, %bb17.i.i ], [ %_3.i4303, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5998
  %gain_near.i15.i.i.sroa.6.0.lcssa = phi i32 [ %259, %bb17.i.i ], [ %_3.i4299, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5998
  %gain_far.i14.i.i.sroa.0.0.lcssa = phi i32 [ %260, %bb17.i.i ], [ %_3.i4295, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5999
  %gain_far.i14.i.i.sroa.6.0.lcssa = phi i32 [ %261, %bb17.i.i ], [ %_3.i4291, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5999
  %position.sroa.0.0.i28.i.i.lcssa = phi i64 [ %262, %bb17.i.i ], [ %_37.sroa.0.0.i36.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6000
  store float %filter_near.i17.i.i.sroa.0.0.lcssa, ptr %164, align 8, !dbg !6001, !noalias !5071
  store float %filter_near.i17.i.i.sroa.7.0.lcssa, ptr %filter_near.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !6001, !noalias !5071
  store float %filter_near.i17.i.i.sroa.11.0.lcssa, ptr %filter_near.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !6001, !noalias !5071
  store float %filter_near.i17.i.i.sroa.14.0.lcssa, ptr %filter_near.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !6001, !noalias !5071
  store float %filter_far.i16.i.i.sroa.0.0.lcssa, ptr %165, align 8, !dbg !6002, !noalias !5071
  store float %filter_far.i16.i.i.sroa.7.0.lcssa, ptr %filter_far.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !6002, !noalias !5071
  store float %filter_far.i16.i.i.sroa.11.0.lcssa, ptr %filter_far.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !6002, !noalias !5071
  store float %filter_far.i16.i.i.sroa.14.0.lcssa, ptr %filter_far.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !6002, !noalias !5071
  store i32 %gain_near.i15.i.i.sroa.0.0.lcssa, ptr %166, align 8, !dbg !6003, !noalias !5071
  store i32 %gain_near.i15.i.i.sroa.6.0.lcssa, ptr %.sroa_idx6954, align 4, !dbg !6003, !noalias !5071
  store i32 %gain_far.i14.i.i.sroa.0.0.lcssa, ptr %167, align 8, !dbg !6004, !noalias !5071
  store i32 %gain_far.i14.i.i.sroa.6.0.lcssa, ptr %.sroa_idx6959, align 4, !dbg !6004, !noalias !5071
  store i64 %position.sroa.0.0.i28.i.i.lcssa, ptr %_51.i.i, align 8, !dbg !6005, !alias.scope !5022, !noalias !5023
  %advanced.i.i = trunc i64 %plan.0.i.i to i32, !dbg !6006
  store float %segments.i.i.sroa.0.0, ptr %124, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.i = load i32, ptr %180, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %315 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i, i32 %advanced.i.i), !dbg !6023
  store i32 %315, ptr %180, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.8.0, ptr %126, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.1.i = load i32, ptr %181, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %316 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i, i32 %advanced.i.i), !dbg !6023
  store i32 %316, ptr %181, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.12.0, ptr %128, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.2.i = load i32, ptr %182, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %317 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i, i32 %advanced.i.i), !dbg !6023
  store i32 %317, ptr %182, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.16.0, ptr %130, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.3.i = load i32, ptr %183, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %318 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i, i32 %advanced.i.i), !dbg !6023
  store i32 %318, ptr %183, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.20.0, ptr %132, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.4.i = load i32, ptr %184, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %319 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i, i32 %advanced.i.i), !dbg !6023
  store i32 %319, ptr %184, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.26.0, ptr %134, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.5.i = load i32, ptr %185, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %320 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i, i32 %advanced.i.i), !dbg !6023
  store i32 %320, ptr %185, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.32.0, ptr %136, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.6.i = load i32, ptr %186, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %321 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i, i32 %advanced.i.i), !dbg !6023
  store i32 %321, ptr %186, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.36.0, ptr %138, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.7.i = load i32, ptr %187, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %322 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i, i32 %advanced.i.i), !dbg !6023
  store i32 %322, ptr %187, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.40.0, ptr %140, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.8.i = load i32, ptr %188, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %323 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i, i32 %advanced.i.i), !dbg !6023
  store i32 %323, ptr %188, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.44.0, ptr %142, align 4, !dbg !6007, !alias.scope !6017, !noalias !6020
  %_26.9.i = load i32, ptr %189, align 4, !dbg !6022, !alias.scope !6017, !noalias !6020, !noundef !11
  %324 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i, i32 %advanced.i.i), !dbg !6023
  store i32 %324, ptr %189, align 4, !dbg !6026, !alias.scope !6017, !noalias !6020
  store float %segments.i.i.sroa.70.0, ptr %144, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.i6477 = load i32, ptr %190, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %325 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6477, i32 %advanced.i.i), !dbg !6035
  store i32 %325, ptr %190, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.76.0, ptr %146, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.1.i6480 = load i32, ptr %191, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %326 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6480, i32 %advanced.i.i), !dbg !6035
  store i32 %326, ptr %191, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.80.0, ptr %148, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.2.i6483 = load i32, ptr %192, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %327 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6483, i32 %advanced.i.i), !dbg !6035
  store i32 %327, ptr %192, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.84.0, ptr %150, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.3.i6486 = load i32, ptr %193, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %328 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6486, i32 %advanced.i.i), !dbg !6035
  store i32 %328, ptr %193, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.88.0, ptr %152, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.4.i6489 = load i32, ptr %194, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %329 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6489, i32 %advanced.i.i), !dbg !6035
  store i32 %329, ptr %194, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.94.0, ptr %154, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.5.i6492 = load i32, ptr %195, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %330 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6492, i32 %advanced.i.i), !dbg !6035
  store i32 %330, ptr %195, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.100.0, ptr %156, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.6.i6495 = load i32, ptr %196, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %331 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6495, i32 %advanced.i.i), !dbg !6035
  store i32 %331, ptr %196, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.104.0, ptr %158, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.7.i6498 = load i32, ptr %197, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %332 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6498, i32 %advanced.i.i), !dbg !6035
  store i32 %332, ptr %197, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.108.0, ptr %160, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.8.i6501 = load i32, ptr %198, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %333 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6501, i32 %advanced.i.i), !dbg !6035
  store i32 %333, ptr %198, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  store float %segments.i.i.sroa.112.0, ptr %162, align 4, !dbg !6027, !alias.scope !6029, !noalias !6032
  %_26.9.i6504 = load i32, ptr %199, align 4, !dbg !6034, !alias.scope !6029, !noalias !6032, !noundef !11
  %334 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6504, i32 %advanced.i.i), !dbg !6035
  store i32 %334, ptr %199, align 4, !dbg !6037, !alias.scope !6029, !noalias !6032
  br label %bb15.i.i, !dbg !4988

bb2.i19.i.lr.ph:                                  ; preds = %bb2.i, %bb3.i, %bb4.i
  tail call void @llvm.assume(i1 %_7.i), !dbg !3823
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6038), !dbg !6041
  %335 = getelementptr inbounds nuw i8, ptr %self, i64 856, !dbg !6042
  %sample_rate.i14.i = load i32, ptr %335, align 8, !dbg !6042, !alias.scope !6045, !noalias !6046, !noundef !11
  %336 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !6049
  %ring_len.i15.i = load i64, ptr %336, align 8, !dbg !6049, !alias.scope !6045, !noalias !6046, !noundef !11
  %337 = getelementptr inbounds nuw i8, ptr %self, i64 120
  %338 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %339 = getelementptr inbounds nuw i8, ptr %self, i64 200
  %340 = getelementptr inbounds nuw i8, ptr %self, i64 208
  %341 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %342 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %343 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %344 = getelementptr inbounds nuw i8, ptr %self, i64 240
  %345 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %346 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %347 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %348 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %349 = getelementptr inbounds nuw i8, ptr %self, i64 280
  %350 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %351 = getelementptr inbounds nuw i8, ptr %self, i64 296
  %352 = getelementptr inbounds nuw i8, ptr %self, i64 304
  %353 = getelementptr inbounds nuw i8, ptr %self, i64 312
  %354 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %355 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %356 = getelementptr inbounds nuw i8, ptr %self, i64 336
  %357 = getelementptr inbounds nuw i8, ptr %self, i64 344
  %_18.i23.i = getelementptr inbounds nuw i8, ptr %self, i64 480
  %358 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %359 = getelementptr inbounds nuw i8, ptr %self, i64 560
  %360 = getelementptr inbounds nuw i8, ptr %self, i64 568
  %361 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %362 = getelementptr inbounds nuw i8, ptr %self, i64 584
  %363 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %364 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %365 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %366 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %367 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %368 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %369 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %370 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %371 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %374 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %375 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %376 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %377 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %_51.i25.i = getelementptr inbounds nuw i8, ptr %self, i64 848
  %378 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %379 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %380 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %381 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %382 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %383 = getelementptr inbounds nuw i8, ptr %self, i64 488
  %384 = getelementptr inbounds nuw i8, ptr %self, i64 204
  %385 = getelementptr inbounds nuw i8, ptr %self, i64 220
  %386 = getelementptr inbounds nuw i8, ptr %self, i64 236
  %387 = getelementptr inbounds nuw i8, ptr %self, i64 252
  %388 = getelementptr inbounds nuw i8, ptr %self, i64 268
  %389 = getelementptr inbounds nuw i8, ptr %self, i64 284
  %390 = getelementptr inbounds nuw i8, ptr %self, i64 300
  %391 = getelementptr inbounds nuw i8, ptr %self, i64 316
  %392 = getelementptr inbounds nuw i8, ptr %self, i64 332
  %393 = getelementptr inbounds nuw i8, ptr %self, i64 348
  %394 = getelementptr inbounds nuw i8, ptr %self, i64 564
  %395 = getelementptr inbounds nuw i8, ptr %self, i64 580
  %396 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %397 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %398 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %399 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %400 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %401 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %402 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %403 = getelementptr inbounds nuw i8, ptr %self, i64 708
  br label %bb2.i19.i, !dbg !6051

bb2.i19.i:                                        ; preds = %bb2.i19.i.lr.ph, %bb15.i39.i
  %position.sroa.0.0.i17.i11239 = phi i64 [ 0, %bb2.i19.i.lr.ph ], [ %_32.i48.i, %bb15.i39.i ]
  %_11.i20.i = sub nuw i64 %_19.1, %position.sroa.0.0.i17.i11239, !dbg !6054
; call <multiband_compressor::Instance<f32, 1>>::plan_segment
  %404 = tail call fastcc { i64, i1 } @_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_(ptr noalias noundef nonnull align 8 dereferenceable(768) %_5, i64 noundef %_11.i20.i) #25, !dbg !6055, !noalias !3842
  %plan.0.i21.i = extractvalue { i64, i1 } %404, 0, !dbg !6055
  %plan.1.i22.i = extractvalue { i64, i1 } %404, 1, !dbg !6055
  %_12.le.i6505 = load float, ptr %338, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.i6506 = load float, ptr %339, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.1.i6507 = load float, ptr %340, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.1.i6508 = load float, ptr %341, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.2.i6509 = load float, ptr %342, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.2.i6510 = load float, ptr %343, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.3.i6511 = load float, ptr %344, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.3.i6512 = load float, ptr %345, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.4.i6513 = load float, ptr %346, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.4.i6514 = load float, ptr %347, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.5.i6515 = load float, ptr %348, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.5.i6516 = load float, ptr %349, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.6.i6517 = load float, ptr %350, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.6.i6518 = load float, ptr %351, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.7.i6519 = load float, ptr %352, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.7.i6520 = load float, ptr %353, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.8.i6521 = load float, ptr %354, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.8.i6522 = load float, ptr %355, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.9.i6523 = load float, ptr %356, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_16.le.9.i6524 = load float, ptr %357, align 8, !alias.scope !6056, !noalias !6059, !noundef !11
  %_12.le.i6543 = load float, ptr %358, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.i6544 = load float, ptr %359, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.1.i6545 = load float, ptr %360, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.1.i6546 = load float, ptr %361, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.2.i6547 = load float, ptr %362, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.2.i6548 = load float, ptr %363, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.3.i6549 = load float, ptr %364, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.3.i6550 = load float, ptr %365, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.4.i6551 = load float, ptr %366, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.4.i6552 = load float, ptr %367, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.5.i6553 = load float, ptr %368, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.5.i6554 = load float, ptr %369, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.6.i6555 = load float, ptr %370, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.6.i6556 = load float, ptr %371, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.7.i6557 = load float, ptr %372, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.7.i6558 = load float, ptr %373, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.8.i6559 = load float, ptr %374, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.8.i6560 = load float, ptr %375, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_12.le.9.i6561 = load float, ptr %376, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  %_16.le.9.i6562 = load float, ptr %377, align 8, !alias.scope !6061, !noalias !6064, !noundef !11
  call void @llvm.lifetime.start.p0(ptr nonnull %_20.i9.i), !dbg !6066, !noalias !6070
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_20.i9.i, ptr noalias noundef align 8 dereferenceable(360) %337, i32 noundef %sample_rate.i14.i) #25, !dbg !6071, !noalias !3842
  call void @llvm.lifetime.start.p0(ptr nonnull %_22.i8.i), !dbg !6072, !noalias !6070
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_22.i8.i, ptr noalias noundef align 8 dereferenceable(360) %_18.i23.i, i32 noundef %sample_rate.i14.i) #25, !dbg !6073, !noalias !3842
  call void @llvm.lifetime.end.p0(ptr nonnull %_22.i8.i), !dbg !6074, !noalias !6070
  call void @llvm.lifetime.end.p0(ptr nonnull %_20.i9.i), !dbg !6074, !noalias !6070
  %_32.i48.i = add i64 %plan.0.i21.i, %position.sroa.0.0.i17.i11239, !dbg !6075
  %_72.i49.i = icmp ult i64 %_32.i48.i, %position.sroa.0.0.i17.i11239, !dbg !6077
  %_66.not.i50.i = icmp ugt i64 %_32.i48.i, %_19.1
  %or.cond11.i51.i = or i1 %_72.i49.i, %_66.not.i50.i, !dbg !6077
  br i1 %plan.1.i22.i, label %bb9.i46.i, label %bb13.i24.i, !dbg !6083

bb9.i46.i:                                        ; preds = %bb2.i19.i
  br i1 %or.cond11.i51.i, label %bb19.i98.i, label %bb17.i52.i, !dbg !6084, !prof !3878

bb13.i24.i:                                       ; preds = %bb2.i19.i
  br i1 %or.cond11.i51.i, label %bb29.i45.i, label %bb27.i30.i, !dbg !6088, !prof !3878

bb27.i30.i:                                       ; preds = %bb13.i24.i
  %_95.i31.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i17.i11239, !dbg !6094
  %_105.i34.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i17.i11239, !dbg !6098
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6105), !dbg !6108
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %filter_near.i15.i.i, ptr noundef nonnull align 8 dereferenceable(16) %378, i64 16, i1 false), !dbg !6109
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %filter_far.i14.i.i, ptr noundef nonnull align 8 dereferenceable(16) %379, i64 16, i1 false), !dbg !6115
  %gain_near.sroa.0.0.copyload.i25.i.i = load i64, ptr %380, align 8, !dbg !6117, !noalias !6119
  %gain_far.sroa.0.0.copyload.i26.i.i = load i64, ptr %381, align 8, !dbg !6122, !noalias !6119
  %405 = load i64, ptr %_51.i25.i, align 8, !dbg !6124, !alias.scope !6126, !noalias !6127, !noundef !11
  %_164.i30.i38.i11229.not = icmp eq i64 %plan.0.i21.i, 0, !dbg !6129
  br i1 %_164.i30.i38.i11229.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_Kb0_EB2_.exit.i.i, label %bb54.i31.i.i, !dbg !6139

bb29.i45.i:                                       ; preds = %bb13.i24.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i17.i11239, i64 noundef %_32.i48.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a05c61cb172b1016332ce1d3ce81e461) #26, !dbg !6140, !noalias !3842
  unreachable, !dbg !6140

bb54.i31.i.i:                                     ; preds = %bb27.i30.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037
  %iter.sroa.0.0.i29.i37.i11231 = phi i64 [ %406, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037 ], [ 0, %bb27.i30.i ]
  %position.sroa.0.0.i28.i36.i11230 = phi i64 [ %_37.sroa.0.0.i34.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037 ], [ %405, %bb27.i30.i ]
  %406 = add nuw i64 %iter.sroa.0.0.i29.i37.i11231, 1, !dbg !6141
  %_38.i32.i.i = add i64 %position.sroa.0.0.i28.i36.i11230, 1, !dbg !6147
  %_172.not.i33.i.i = icmp ult i64 %_38.i32.i.i, %ring_len.i15.i, !dbg !6150
  %407 = select i1 %_172.not.i33.i.i, i64 0, i64 %ring_len.i15.i, !dbg !6150
  %_37.sroa.0.0.i34.i.i = sub nuw i64 %_38.i32.i.i, %407, !dbg !6150
  %_180.i38.i41.i = getelementptr inbounds nuw float, ptr %_95.i31.i, i64 %iter.sroa.0.0.i29.i37.i11231, !dbg !6153
  %_0.i3656 = load float, ptr %_180.i38.i41.i, align 4, !dbg !6163, !alias.scope !6165, !noalias !6168, !noundef !11
  %_188.i41.i.i = getelementptr inbounds nuw float, ptr %_105.i34.i, i64 %iter.sroa.0.0.i29.i37.i11231, !dbg !6169
  %_0.i3651 = load float, ptr %_188.i41.i.i, align 4, !dbg !6178, !alias.scope !6180, !noalias !6168, !noundef !11
  %_303.1.i43.i.i = load i64, ptr %382, align 8, !dbg !6183, !noalias !6168, !noundef !11
  %_189.i44.i.i = icmp ugt i64 %position.sroa.0.0.i28.i36.i11230, %_303.1.i43.i.i, !dbg !6185
  br i1 %_189.i44.i.i, label %bb65.i73.i.i, label %bb66.i45.i.i, !dbg !6185, !prof !1664

bb66.i45.i.i:                                     ; preds = %bb54.i31.i.i
  %_303.0.i46.i.i = load ptr, ptr %337, align 8, !dbg !6183, !noalias !6168, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6190), !dbg !6193
  %_4.not.i4046 = icmp eq i64 %_303.1.i43.i.i, %position.sroa.0.0.i28.i36.i11230, !dbg !6194
  br i1 %_4.not.i4046, label %panic.i4048, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4049, !dbg !6194

panic.i4048:                                      ; preds = %bb66.i45.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6194, !noalias !6196
  unreachable, !dbg !6194

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4049: ; preds = %bb66.i45.i.i
  %_196.i48.i.i = getelementptr inbounds nuw float, ptr %_303.0.i46.i.i, i64 %position.sroa.0.0.i28.i36.i11230, !dbg !6197
  store float %_0.i3656, ptr %_196.i48.i.i, align 4, !dbg !6194, !alias.scope !6190, !noalias !6168
  %_304.1.i49.i.i = load i64, ptr %383, align 8, !dbg !6202, !noalias !6168, !noundef !11
  %_197.i50.i.i = icmp ugt i64 %position.sroa.0.0.i28.i36.i11230, %_304.1.i49.i.i, !dbg !6203
  br i1 %_197.i50.i.i, label %bb67.i72.i.i, label %bb68.i51.i.i, !dbg !6203, !prof !1664

bb65.i73.i.i:                                     ; preds = %bb54.i31.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i36.i11230, i64 noundef %_303.1.i43.i.i, i64 noundef %_303.1.i43.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a5fe42438bf1cd51848d461e43168e01) #26, !dbg !6207, !noalias !6168
  unreachable, !dbg !6207

bb68.i51.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4049
  %_304.0.i52.i.i = load ptr, ptr %_18.i23.i, align 8, !dbg !6202, !noalias !6168, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6208), !dbg !6211
  %_4.not.i4042 = icmp eq i64 %_304.1.i49.i.i, %position.sroa.0.0.i28.i36.i11230, !dbg !6212
  br i1 %_4.not.i4042, label %panic.i4044, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045, !dbg !6212

panic.i4044:                                      ; preds = %bb68.i51.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6212, !noalias !6214
  unreachable, !dbg !6212

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045: ; preds = %bb68.i51.i.i
  %_204.i54.i.i = getelementptr inbounds nuw float, ptr %_304.0.i52.i.i, i64 %position.sroa.0.0.i28.i36.i11230, !dbg !6215
  store float %_0.i3651, ptr %_204.i54.i.i, align 4, !dbg !6212, !alias.scope !6208, !noalias !6168
  %_305.1.i55.i.i = load i64, ptr %382, align 8, !dbg !6220, !noalias !6168, !noundef !11
  %_205.i56.i.i = icmp ugt i64 %_37.sroa.0.0.i34.i.i, %_305.1.i55.i.i, !dbg !6221
  br i1 %_205.i56.i.i, label %bb69.i71.i.i, label %bb70.i57.i.i, !dbg !6221, !prof !1664

bb67.i72.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4049
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i36.i11230, i64 noundef %_304.1.i49.i.i, i64 noundef %_304.1.i49.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_947a12da53e0da5ef683a81874315ce0) #26, !dbg !6225, !noalias !6168
  unreachable, !dbg !6225

bb70.i57.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045
  %_305.0.i58.i.i = load ptr, ptr %337, align 8, !dbg !6220, !noalias !6168, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6226), !dbg !6229
  %_3.not.i3644 = icmp eq i64 %_305.1.i55.i.i, %_37.sroa.0.0.i34.i.i, !dbg !6230
  br i1 %_3.not.i3644, label %panic.i3647, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4041, !dbg !6230

panic.i3647:                                      ; preds = %bb70.i57.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6230, !noalias !6232
  unreachable, !dbg !6230

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4041: ; preds = %bb70.i57.i.i
  %_212.i60.i.i = getelementptr inbounds nuw float, ptr %_305.0.i58.i.i, i64 %_37.sroa.0.0.i34.i.i, !dbg !6233
  %_0.i3646 = load float, ptr %_212.i60.i.i, align 4, !dbg !6230, !alias.scope !6226, !noalias !6168, !noundef !11
  store float %_0.i3646, ptr %_180.i38.i41.i, align 4, !dbg !6238, !alias.scope !6240, !noalias !6168
  %_306.1.i62.i.i = load i64, ptr %383, align 8, !dbg !6243, !noalias !6168, !noundef !11
  %_217.i63.i.i = icmp ugt i64 %_37.sroa.0.0.i34.i.i, %_306.1.i62.i.i, !dbg !6244
  br i1 %_217.i63.i.i, label %bb71.i70.i.i, label %bb72.i64.i.i, !dbg !6244, !prof !1664

bb69.i71.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i34.i.i, i64 noundef %_305.1.i55.i.i, i64 noundef %_305.1.i55.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3c433846b75a8609e2aa4275946b39b4) #26, !dbg !6248, !noalias !6168
  unreachable, !dbg !6248

bb72.i64.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4041
  %_306.0.i65.i.i = load ptr, ptr %_18.i23.i, align 8, !dbg !6243, !noalias !6168, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6249), !dbg !6252
  %_3.not.i3639 = icmp eq i64 %_306.1.i62.i.i, %_37.sroa.0.0.i34.i.i, !dbg !6253
  br i1 %_3.not.i3639, label %panic.i3642, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037, !dbg !6253

panic.i3642:                                      ; preds = %bb72.i64.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6253, !noalias !6255
  unreachable, !dbg !6253

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037: ; preds = %bb72.i64.i.i
  %_224.i67.i.i = getelementptr inbounds nuw float, ptr %_306.0.i65.i.i, i64 %_37.sroa.0.0.i34.i.i, !dbg !6256
  %_0.i3641 = load float, ptr %_224.i67.i.i, align 4, !dbg !6253, !alias.scope !6249, !noalias !6168, !noundef !11
  store float %_0.i3641, ptr %_188.i41.i.i, align 4, !dbg !6261, !alias.scope !6263, !noalias !6168
  %exitcond13936.not = icmp eq i64 %406, %plan.0.i21.i, !dbg !6129
  br i1 %exitcond13936.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_Kb0_EB2_.exit.i.i, label %bb54.i31.i.i, !dbg !6139

bb71.i70.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4041
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i34.i.i, i64 noundef %_306.1.i62.i.i, i64 noundef %_306.1.i62.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7cdc51debee5c62c8dce89684c9c22f) #26, !dbg !6266, !noalias !6168
  unreachable, !dbg !6266

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_Kb0_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037, %bb27.i30.i
  %position.sroa.0.0.i28.i36.i.lcssa = phi i64 [ %405, %bb27.i30.i ], [ %_37.sroa.0.0.i34.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037 ], !dbg !6267
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %378, ptr noundef nonnull align 4 dereferenceable(16) %filter_near.i15.i.i, i64 16, i1 false), !dbg !6268
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %379, ptr noundef nonnull align 4 dereferenceable(16) %filter_far.i14.i.i, i64 16, i1 false), !dbg !6269
  store i64 %gain_near.sroa.0.0.copyload.i25.i.i, ptr %380, align 8, !dbg !6270, !noalias !6168
  store i64 %gain_far.sroa.0.0.copyload.i26.i.i, ptr %381, align 8, !dbg !6271, !noalias !6168
  store i64 %position.sroa.0.0.i28.i36.i.lcssa, ptr %_51.i25.i, align 8, !dbg !6272, !alias.scope !6126, !noalias !6127
  br label %bb15.i39.i, !dbg !6273

bb15.i39.i:                                       ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_KBZ_EB2_.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_Kb0_EB2_.exit.i.i
  %_8.i18.i = icmp ult i64 %_32.i48.i, %_19.1, !dbg !6051
  br i1 %_8.i18.i, label %bb2.i19.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit, !dbg !6051

bb17.i52.i:                                       ; preds = %bb9.i46.i
  %_75.i53.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i17.i11239, !dbg !6274
  %_85.i56.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i17.i11239, !dbg !6277
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6284), !dbg !6287
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %filter_near.i.i6.i, ptr noundef nonnull align 8 dereferenceable(16) %378, i64 16, i1 false), !dbg !6288
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %filter_far.i.i5.i, ptr noundef nonnull align 8 dereferenceable(16) %379, i64 16, i1 false), !dbg !6294
  %gain_near.sroa.0.0.copyload.i.i.i = load i64, ptr %380, align 8, !dbg !6296, !noalias !6298
  %gain_far.sroa.0.0.copyload.i.i.i = load i64, ptr %381, align 8, !dbg !6302, !noalias !6298
  %408 = load i64, ptr %_51.i25.i, align 8, !dbg !6304, !alias.scope !6306, !noalias !6307, !noundef !11
  %_164.i.i68.i11234.not = icmp eq i64 %plan.0.i21.i, 0, !dbg !6309
  br i1 %_164.i.i68.i11234.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_KBZ_EB2_.exit.i.i, label %bb54.i.i70.i, !dbg !6319

bb19.i98.i:                                       ; preds = %bb9.i46.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i17.i11239, i64 noundef %_32.i48.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_61a2f59006034c74bb8ab3eed52140c6) #26, !dbg !6320, !noalias !3842
  unreachable, !dbg !6320

bb54.i.i70.i:                                     ; preds = %bb17.i52.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021
  %segments.i13.i.sroa.98.1 = phi float [ %_0.i2809.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.9.i6561, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.94.1 = phi float [ %_0.i2809.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.8.i6559, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.90.1 = phi float [ %_0.i2809.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.7.i6557, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.86.1 = phi float [ %_0.i2809.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.6.i6555, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.82.1 = phi float [ %_0.i2809.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.5.i6553, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.78.1 = phi float [ %_0.i2809.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.4.i6551, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.74.1 = phi float [ %_0.i2809.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.3.i6549, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.70.1 = phi float [ %_0.i2809.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.2.i6547, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.66.1 = phi float [ %_0.i2809.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.1.i6545, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.62.1 = phi float [ %_0.i2809, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.i6543, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.38.1 = phi float [ %_0.i2810.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.9.i6523, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.34.1 = phi float [ %_0.i2810.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.8.i6521, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.30.1 = phi float [ %_0.i2810.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.7.i6519, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.26.1 = phi float [ %_0.i2810.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.6.i6517, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.22.1 = phi float [ %_0.i2810.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.5.i6515, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.18.1 = phi float [ %_0.i2810.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.4.i6513, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.14.1 = phi float [ %_0.i2810.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.3.i6511, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.10.1 = phi float [ %_0.i2810.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.2.i6509, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.6.1 = phi float [ %_0.i2810.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.1.i6507, %bb17.i52.i ], !dbg !6321
  %segments.i13.i.sroa.0.1 = phi float [ %_0.i2810, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %_12.le.i6505, %bb17.i52.i ], !dbg !6321
  %iter.sroa.0.0.i.i67.i11236 = phi i64 [ %409, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ 0, %bb17.i52.i ]
  %position.sroa.0.0.i.i66.i11235 = phi i64 [ %_37.sroa.0.0.i.i77.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %408, %bb17.i52.i ]
  %_0.i2810 = fadd float %segments.i13.i.sroa.0.1, %_16.le.i6506, !dbg !6322
  %_0.i2809 = fadd float %segments.i13.i.sroa.62.1, %_16.le.i6544, !dbg !6327
  %_0.i2810.1 = fadd float %segments.i13.i.sroa.6.1, %_16.le.1.i6508, !dbg !6322
  %_0.i2809.1 = fadd float %segments.i13.i.sroa.66.1, %_16.le.1.i6546, !dbg !6327
  %_0.i2810.2 = fadd float %segments.i13.i.sroa.10.1, %_16.le.2.i6510, !dbg !6322
  %_0.i2809.2 = fadd float %segments.i13.i.sroa.70.1, %_16.le.2.i6548, !dbg !6327
  %_0.i2810.3 = fadd float %segments.i13.i.sroa.14.1, %_16.le.3.i6512, !dbg !6322
  %_0.i2809.3 = fadd float %segments.i13.i.sroa.74.1, %_16.le.3.i6550, !dbg !6327
  %_0.i2810.4 = fadd float %segments.i13.i.sroa.18.1, %_16.le.4.i6514, !dbg !6322
  %_0.i2809.4 = fadd float %segments.i13.i.sroa.78.1, %_16.le.4.i6552, !dbg !6327
  %_0.i2810.5 = fadd float %segments.i13.i.sroa.22.1, %_16.le.5.i6516, !dbg !6322
  %_0.i2809.5 = fadd float %segments.i13.i.sroa.82.1, %_16.le.5.i6554, !dbg !6327
  %_0.i2810.6 = fadd float %segments.i13.i.sroa.26.1, %_16.le.6.i6518, !dbg !6322
  %_0.i2809.6 = fadd float %segments.i13.i.sroa.86.1, %_16.le.6.i6556, !dbg !6327
  %_0.i2810.7 = fadd float %segments.i13.i.sroa.30.1, %_16.le.7.i6520, !dbg !6322
  %_0.i2809.7 = fadd float %segments.i13.i.sroa.90.1, %_16.le.7.i6558, !dbg !6327
  %_0.i2810.8 = fadd float %segments.i13.i.sroa.34.1, %_16.le.8.i6522, !dbg !6322
  %_0.i2809.8 = fadd float %segments.i13.i.sroa.94.1, %_16.le.8.i6560, !dbg !6327
  %_0.i2810.9 = fadd float %segments.i13.i.sroa.38.1, %_16.le.9.i6524, !dbg !6322
  %_0.i2809.9 = fadd float %segments.i13.i.sroa.98.1, %_16.le.9.i6562, !dbg !6327
  %409 = add nuw i64 %iter.sroa.0.0.i.i67.i11236, 1, !dbg !6329
  %_38.i.i75.i = add i64 %position.sroa.0.0.i.i66.i11235, 1, !dbg !6335
  %_172.not.i.i76.i = icmp ult i64 %_38.i.i75.i, %ring_len.i15.i, !dbg !6337
  %410 = select i1 %_172.not.i.i76.i, i64 0, i64 %ring_len.i15.i, !dbg !6337
  %_37.sroa.0.0.i.i77.i = sub nuw i64 %_38.i.i75.i, %410, !dbg !6337
  %_180.i.i81.i = getelementptr inbounds nuw float, ptr %_75.i53.i, i64 %iter.sroa.0.0.i.i67.i11236, !dbg !6340
  %_0.i3636 = load float, ptr %_180.i.i81.i, align 4, !dbg !6350, !alias.scope !6352, !noalias !6355, !noundef !11
  %_188.i.i84.i = getelementptr inbounds nuw float, ptr %_85.i56.i, i64 %iter.sroa.0.0.i.i67.i11236, !dbg !6356
  %_0.i3631 = load float, ptr %_188.i.i84.i, align 4, !dbg !6365, !alias.scope !6367, !noalias !6355, !noundef !11
  %_303.1.i.i.i = load i64, ptr %382, align 8, !dbg !6370, !noalias !6355, !noundef !11
  %_189.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i66.i11235, %_303.1.i.i.i, !dbg !6372
  br i1 %_189.i.i.i, label %bb65.i.i.i, label %bb66.i.i.i, !dbg !6372, !prof !1664

bb66.i.i.i:                                       ; preds = %bb54.i.i70.i
  %_303.0.i.i.i = load ptr, ptr %337, align 8, !dbg !6370, !noalias !6355, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6377), !dbg !6380
  %_4.not.i4030 = icmp eq i64 %_303.1.i.i.i, %position.sroa.0.0.i.i66.i11235, !dbg !6381
  br i1 %_4.not.i4030, label %panic.i4032, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4033, !dbg !6381

panic.i4032:                                      ; preds = %bb66.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6381, !noalias !6383
  unreachable, !dbg !6381

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4033: ; preds = %bb66.i.i.i
  %_196.i.i.i = getelementptr inbounds nuw float, ptr %_303.0.i.i.i, i64 %position.sroa.0.0.i.i66.i11235, !dbg !6384
  store float %_0.i3636, ptr %_196.i.i.i, align 4, !dbg !6381, !alias.scope !6377, !noalias !6355
  %_304.1.i.i.i = load i64, ptr %383, align 8, !dbg !6389, !noalias !6355, !noundef !11
  %_197.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i66.i11235, %_304.1.i.i.i, !dbg !6390
  br i1 %_197.i.i.i, label %bb67.i.i.i, label %bb68.i.i.i, !dbg !6390, !prof !1664

bb65.i.i.i:                                       ; preds = %bb54.i.i70.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i66.i11235, i64 noundef %_303.1.i.i.i, i64 noundef %_303.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a5fe42438bf1cd51848d461e43168e01) #26, !dbg !6394, !noalias !6355
  unreachable, !dbg !6394

bb68.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4033
  %_304.0.i.i.i = load ptr, ptr %_18.i23.i, align 8, !dbg !6389, !noalias !6355, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6395), !dbg !6398
  %_4.not.i4026 = icmp eq i64 %_304.1.i.i.i, %position.sroa.0.0.i.i66.i11235, !dbg !6399
  br i1 %_4.not.i4026, label %panic.i4028, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4029, !dbg !6399

panic.i4028:                                      ; preds = %bb68.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6399, !noalias !6401
  unreachable, !dbg !6399

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4029: ; preds = %bb68.i.i.i
  %_204.i.i.i = getelementptr inbounds nuw float, ptr %_304.0.i.i.i, i64 %position.sroa.0.0.i.i66.i11235, !dbg !6402
  store float %_0.i3631, ptr %_204.i.i.i, align 4, !dbg !6399, !alias.scope !6395, !noalias !6355
  %_305.1.i.i.i = load i64, ptr %382, align 8, !dbg !6407, !noalias !6355, !noundef !11
  %_205.i.i.i = icmp ugt i64 %_37.sroa.0.0.i.i77.i, %_305.1.i.i.i, !dbg !6408
  br i1 %_205.i.i.i, label %bb69.i.i.i, label %bb70.i.i.i, !dbg !6408, !prof !1664

bb67.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4033
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i66.i11235, i64 noundef %_304.1.i.i.i, i64 noundef %_304.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_947a12da53e0da5ef683a81874315ce0) #26, !dbg !6412, !noalias !6355
  unreachable, !dbg !6412

bb70.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4029
  %_305.0.i.i.i = load ptr, ptr %337, align 8, !dbg !6407, !noalias !6355, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6413), !dbg !6416
  %_3.not.i3624 = icmp eq i64 %_305.1.i.i.i, %_37.sroa.0.0.i.i77.i, !dbg !6417
  br i1 %_3.not.i3624, label %panic.i3627, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4025, !dbg !6417

panic.i3627:                                      ; preds = %bb70.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6417, !noalias !6419
  unreachable, !dbg !6417

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4025: ; preds = %bb70.i.i.i
  %_212.i.i.i = getelementptr inbounds nuw float, ptr %_305.0.i.i.i, i64 %_37.sroa.0.0.i.i77.i, !dbg !6420
  %_0.i3626 = load float, ptr %_212.i.i.i, align 4, !dbg !6417, !alias.scope !6413, !noalias !6355, !noundef !11
  store float %_0.i3626, ptr %_180.i.i81.i, align 4, !dbg !6425, !alias.scope !6427, !noalias !6355
  %_306.1.i.i.i = load i64, ptr %383, align 8, !dbg !6430, !noalias !6355, !noundef !11
  %_217.i.i.i = icmp ugt i64 %_37.sroa.0.0.i.i77.i, %_306.1.i.i.i, !dbg !6431
  br i1 %_217.i.i.i, label %bb71.i.i.i, label %bb72.i.i.i, !dbg !6431, !prof !1664

bb69.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4029
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i77.i, i64 noundef %_305.1.i.i.i, i64 noundef %_305.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3c433846b75a8609e2aa4275946b39b4) #26, !dbg !6435, !noalias !6355
  unreachable, !dbg !6435

bb72.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4025
  %_306.0.i.i.i = load ptr, ptr %_18.i23.i, align 8, !dbg !6430, !noalias !6355, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6436), !dbg !6439
  %_3.not.i3619 = icmp eq i64 %_306.1.i.i.i, %_37.sroa.0.0.i.i77.i, !dbg !6440
  br i1 %_3.not.i3619, label %panic.i3622, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021, !dbg !6440

panic.i3622:                                      ; preds = %bb72.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6440, !noalias !6442
  unreachable, !dbg !6440

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021: ; preds = %bb72.i.i.i
  %_224.i.i.i = getelementptr inbounds nuw float, ptr %_306.0.i.i.i, i64 %_37.sroa.0.0.i.i77.i, !dbg !6443
  %_0.i3621 = load float, ptr %_224.i.i.i, align 4, !dbg !6440, !alias.scope !6436, !noalias !6355, !noundef !11
  store float %_0.i3621, ptr %_188.i.i84.i, align 4, !dbg !6448, !alias.scope !6450, !noalias !6355
  %exitcond13938.not = icmp eq i64 %409, %plan.0.i21.i, !dbg !6309
  br i1 %exitcond13938.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_KBZ_EB2_.exit.i.i, label %bb54.i.i70.i, !dbg !6319

bb71.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4025
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i77.i, i64 noundef %_306.1.i.i.i, i64 noundef %_306.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7cdc51debee5c62c8dce89684c9c22f) #26, !dbg !6453, !noalias !6355
  unreachable, !dbg !6453

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_KBZ_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021, %bb17.i52.i
  %segments.i13.i.sroa.98.0 = phi float [ %_12.le.9.i6561, %bb17.i52.i ], [ %_0.i2809.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.94.0 = phi float [ %_12.le.8.i6559, %bb17.i52.i ], [ %_0.i2809.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.90.0 = phi float [ %_12.le.7.i6557, %bb17.i52.i ], [ %_0.i2809.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.86.0 = phi float [ %_12.le.6.i6555, %bb17.i52.i ], [ %_0.i2809.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.82.0 = phi float [ %_12.le.5.i6553, %bb17.i52.i ], [ %_0.i2809.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.78.0 = phi float [ %_12.le.4.i6551, %bb17.i52.i ], [ %_0.i2809.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.74.0 = phi float [ %_12.le.3.i6549, %bb17.i52.i ], [ %_0.i2809.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.70.0 = phi float [ %_12.le.2.i6547, %bb17.i52.i ], [ %_0.i2809.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.66.0 = phi float [ %_12.le.1.i6545, %bb17.i52.i ], [ %_0.i2809.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.62.0 = phi float [ %_12.le.i6543, %bb17.i52.i ], [ %_0.i2809, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.38.0 = phi float [ %_12.le.9.i6523, %bb17.i52.i ], [ %_0.i2810.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.34.0 = phi float [ %_12.le.8.i6521, %bb17.i52.i ], [ %_0.i2810.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.30.0 = phi float [ %_12.le.7.i6519, %bb17.i52.i ], [ %_0.i2810.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.26.0 = phi float [ %_12.le.6.i6517, %bb17.i52.i ], [ %_0.i2810.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.22.0 = phi float [ %_12.le.5.i6515, %bb17.i52.i ], [ %_0.i2810.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.18.0 = phi float [ %_12.le.4.i6513, %bb17.i52.i ], [ %_0.i2810.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.14.0 = phi float [ %_12.le.3.i6511, %bb17.i52.i ], [ %_0.i2810.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.10.0 = phi float [ %_12.le.2.i6509, %bb17.i52.i ], [ %_0.i2810.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.6.0 = phi float [ %_12.le.1.i6507, %bb17.i52.i ], [ %_0.i2810.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %segments.i13.i.sroa.0.0 = phi float [ %_12.le.i6505, %bb17.i52.i ], [ %_0.i2810, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6321
  %position.sroa.0.0.i.i66.i.lcssa = phi i64 [ %408, %bb17.i52.i ], [ %_37.sroa.0.0.i.i77.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !6454
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %378, ptr noundef nonnull align 4 dereferenceable(16) %filter_near.i.i6.i, i64 16, i1 false), !dbg !6455
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %379, ptr noundef nonnull align 4 dereferenceable(16) %filter_far.i.i5.i, i64 16, i1 false), !dbg !6456
  store i64 %gain_near.sroa.0.0.copyload.i.i.i, ptr %380, align 8, !dbg !6457, !noalias !6355
  store i64 %gain_far.sroa.0.0.copyload.i.i.i, ptr %381, align 8, !dbg !6458, !noalias !6355
  store i64 %position.sroa.0.0.i.i66.i.lcssa, ptr %_51.i25.i, align 8, !dbg !6459, !alias.scope !6306, !noalias !6307
  %advanced.i69.i = trunc i64 %plan.0.i21.i to i32, !dbg !6460
  store float %segments.i13.i.sroa.0.0, ptr %338, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.i6590 = load i32, ptr %384, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %411 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6590, i32 %advanced.i69.i), !dbg !6470
  store i32 %411, ptr %384, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.6.0, ptr %340, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.1.i6593 = load i32, ptr %385, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %412 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6593, i32 %advanced.i69.i), !dbg !6470
  store i32 %412, ptr %385, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.10.0, ptr %342, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.2.i6596 = load i32, ptr %386, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %413 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6596, i32 %advanced.i69.i), !dbg !6470
  store i32 %413, ptr %386, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.14.0, ptr %344, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.3.i6599 = load i32, ptr %387, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %414 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6599, i32 %advanced.i69.i), !dbg !6470
  store i32 %414, ptr %387, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.18.0, ptr %346, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.4.i6602 = load i32, ptr %388, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %415 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6602, i32 %advanced.i69.i), !dbg !6470
  store i32 %415, ptr %388, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.22.0, ptr %348, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.5.i6605 = load i32, ptr %389, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %416 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6605, i32 %advanced.i69.i), !dbg !6470
  store i32 %416, ptr %389, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.26.0, ptr %350, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.6.i6608 = load i32, ptr %390, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %417 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6608, i32 %advanced.i69.i), !dbg !6470
  store i32 %417, ptr %390, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.30.0, ptr %352, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.7.i6611 = load i32, ptr %391, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %418 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6611, i32 %advanced.i69.i), !dbg !6470
  store i32 %418, ptr %391, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.34.0, ptr %354, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.8.i6614 = load i32, ptr %392, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %419 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6614, i32 %advanced.i69.i), !dbg !6470
  store i32 %419, ptr %392, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.38.0, ptr %356, align 4, !dbg !6461, !alias.scope !6464, !noalias !6467
  %_26.9.i6617 = load i32, ptr %393, align 4, !dbg !6469, !alias.scope !6464, !noalias !6467, !noundef !11
  %420 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6617, i32 %advanced.i69.i), !dbg !6470
  store i32 %420, ptr %393, align 4, !dbg !6472, !alias.scope !6464, !noalias !6467
  store float %segments.i13.i.sroa.62.0, ptr %358, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.i6619 = load i32, ptr %394, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %421 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6619, i32 %advanced.i69.i), !dbg !6481
  store i32 %421, ptr %394, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.66.0, ptr %360, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.1.i6622 = load i32, ptr %395, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %422 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6622, i32 %advanced.i69.i), !dbg !6481
  store i32 %422, ptr %395, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.70.0, ptr %362, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.2.i6625 = load i32, ptr %396, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %423 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6625, i32 %advanced.i69.i), !dbg !6481
  store i32 %423, ptr %396, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.74.0, ptr %364, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.3.i6628 = load i32, ptr %397, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %424 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6628, i32 %advanced.i69.i), !dbg !6481
  store i32 %424, ptr %397, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.78.0, ptr %366, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.4.i6631 = load i32, ptr %398, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %425 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6631, i32 %advanced.i69.i), !dbg !6481
  store i32 %425, ptr %398, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.82.0, ptr %368, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.5.i6634 = load i32, ptr %399, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %426 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6634, i32 %advanced.i69.i), !dbg !6481
  store i32 %426, ptr %399, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.86.0, ptr %370, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.6.i6637 = load i32, ptr %400, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %427 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6637, i32 %advanced.i69.i), !dbg !6481
  store i32 %427, ptr %400, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.90.0, ptr %372, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.7.i6640 = load i32, ptr %401, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %428 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6640, i32 %advanced.i69.i), !dbg !6481
  store i32 %428, ptr %401, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.94.0, ptr %374, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.8.i6643 = load i32, ptr %402, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %429 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6643, i32 %advanced.i69.i), !dbg !6481
  store i32 %429, ptr %402, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  store float %segments.i13.i.sroa.98.0, ptr %376, align 4, !dbg !6473, !alias.scope !6475, !noalias !6478
  %_26.9.i6646 = load i32, ptr %403, align 4, !dbg !6480, !alias.scope !6475, !noalias !6478, !noundef !11
  %430 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6646, i32 %advanced.i69.i), !dbg !6481
  store i32 %430, ptr %403, align 4, !dbg !6483, !alias.scope !6475, !noalias !6478
  br label %bb15.i39.i, !dbg !6273

bb2.i132.i.lr.ph:                                 ; preds = %bb3.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6484), !dbg !6487
  %431 = getelementptr inbounds nuw i8, ptr %self, i64 856, !dbg !6488
  %sample_rate.i127.i = load i32, ptr %431, align 8, !dbg !6488, !alias.scope !6491, !noalias !6492, !noundef !11
  %432 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !6495
  %ring_len.i128.i = load i64, ptr %432, align 8, !dbg !6495, !alias.scope !6491, !noalias !6492, !noundef !11
  %433 = getelementptr inbounds nuw i8, ptr %self, i64 120
  %434 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %435 = getelementptr inbounds nuw i8, ptr %self, i64 200
  %436 = getelementptr inbounds nuw i8, ptr %self, i64 208
  %437 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %438 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %439 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %440 = getelementptr inbounds nuw i8, ptr %self, i64 240
  %441 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %442 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %443 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %444 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %445 = getelementptr inbounds nuw i8, ptr %self, i64 280
  %446 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %447 = getelementptr inbounds nuw i8, ptr %self, i64 296
  %448 = getelementptr inbounds nuw i8, ptr %self, i64 304
  %449 = getelementptr inbounds nuw i8, ptr %self, i64 312
  %450 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %451 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %452 = getelementptr inbounds nuw i8, ptr %self, i64 336
  %453 = getelementptr inbounds nuw i8, ptr %self, i64 344
  %_18.i136.i = getelementptr inbounds nuw i8, ptr %self, i64 480
  %454 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %455 = getelementptr inbounds nuw i8, ptr %self, i64 560
  %456 = getelementptr inbounds nuw i8, ptr %self, i64 568
  %457 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %458 = getelementptr inbounds nuw i8, ptr %self, i64 584
  %459 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %460 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %461 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %462 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %463 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %464 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %465 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %466 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %467 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %468 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %469 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %470 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %471 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %472 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %473 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %coefficients.i123.i.sroa.5.0._20.i122.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i122.i, i64 4
  %coefficients.i123.i.sroa.7.0._20.i122.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i122.i, i64 8
  %coefficients.i123.i.sroa.9.0._20.i122.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i122.i, i64 12
  %coefficients.i123.i.sroa.11.0._20.i122.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i122.i, i64 16
  %coefficients.i123.i.sroa.13.0._20.i122.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i122.i, i64 20
  %coefficients.i123.i.sroa.18.24._22.i121.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i121.i, i64 4
  %coefficients.i123.i.sroa.20.24._22.i121.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i121.i, i64 8
  %coefficients.i123.i.sroa.22.24._22.i121.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i121.i, i64 12
  %coefficients.i123.i.sroa.24.24._22.i121.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i121.i, i64 16
  %coefficients.i123.i.sroa.26.24._22.i121.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i121.i, i64 20
  %_51.i138.i = getelementptr inbounds nuw i8, ptr %self, i64 848
  %474 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %filter_near.i.i119.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 172
  %filter_near.i.i119.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 176
  %filter_near.i.i119.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 180
  %475 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %filter_far.i.i118.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 532
  %filter_far.i.i118.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 536
  %filter_far.i.i118.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 540
  %476 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %.sroa_idx6990 = getelementptr inbounds nuw i8, ptr %self, i64 188
  %477 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %.sroa_idx6995 = getelementptr inbounds nuw i8, ptr %self, i64 548
  %_63.i.i173.i = getelementptr inbounds nuw i8, ptr %self, i64 152
  %478 = getelementptr inbounds nuw i8, ptr %self, i64 156
  %479 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %480 = getelementptr inbounds nuw i8, ptr %self, i64 164
  %_68.i.i175.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %481 = getelementptr inbounds nuw i8, ptr %self, i64 516
  %482 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %483 = getelementptr inbounds nuw i8, ptr %self, i64 524
  %484 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %485 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %486 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %487 = getelementptr inbounds nuw i8, ptr %self, i64 488
  %488 = getelementptr inbounds nuw i8, ptr %self, i64 496
  %489 = getelementptr inbounds nuw i8, ptr %self, i64 504
  %_85.i.i205.i = getelementptr inbounds nuw i8, ptr %self, i64 400
  %_93.i.i220.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %490 = getelementptr inbounds nuw i8, ptr %self, i64 204
  %491 = getelementptr inbounds nuw i8, ptr %self, i64 220
  %492 = getelementptr inbounds nuw i8, ptr %self, i64 236
  %493 = getelementptr inbounds nuw i8, ptr %self, i64 252
  %494 = getelementptr inbounds nuw i8, ptr %self, i64 268
  %495 = getelementptr inbounds nuw i8, ptr %self, i64 284
  %496 = getelementptr inbounds nuw i8, ptr %self, i64 300
  %497 = getelementptr inbounds nuw i8, ptr %self, i64 316
  %498 = getelementptr inbounds nuw i8, ptr %self, i64 332
  %499 = getelementptr inbounds nuw i8, ptr %self, i64 348
  %500 = getelementptr inbounds nuw i8, ptr %self, i64 564
  %501 = getelementptr inbounds nuw i8, ptr %self, i64 580
  %502 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %503 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %504 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %505 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %506 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %507 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %508 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %509 = getelementptr inbounds nuw i8, ptr %self, i64 708
  br label %bb2.i132.i, !dbg !6497

bb2.i132.i:                                       ; preds = %bb2.i132.i.lr.ph, %bb15.i162.i
  %position.sroa.0.0.i130.i10907 = phi i64 [ 0, %bb2.i132.i.lr.ph ], [ %_32.i342.i, %bb15.i162.i ]
  %_11.i133.i = sub nuw i64 %_19.1, %position.sroa.0.0.i130.i10907, !dbg !6500
; call <multiband_compressor::Instance<f32, 1>>::plan_segment
  %510 = tail call fastcc { i64, i1 } @_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_(ptr noalias noundef nonnull align 8 dereferenceable(768) %_5, i64 noundef %_11.i133.i) #25, !dbg !6501, !noalias !3842
  %plan.0.i134.i = extractvalue { i64, i1 } %510, 0, !dbg !6501
  %plan.1.i135.i = extractvalue { i64, i1 } %510, 1, !dbg !6501
  %_12.le.i6647 = load float, ptr %434, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.i6648 = load float, ptr %435, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.1.i6649 = load float, ptr %436, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.1.i6650 = load float, ptr %437, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.2.i6651 = load float, ptr %438, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.2.i6652 = load float, ptr %439, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.3.i6653 = load float, ptr %440, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.3.i6654 = load float, ptr %441, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.4.i6655 = load float, ptr %442, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.4.i6656 = load float, ptr %443, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.5.i6657 = load float, ptr %444, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.5.i6658 = load float, ptr %445, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.6.i6659 = load float, ptr %446, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.6.i6660 = load float, ptr %447, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.7.i6661 = load float, ptr %448, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.7.i6662 = load float, ptr %449, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.8.i6663 = load float, ptr %450, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.8.i6664 = load float, ptr %451, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.9.i6665 = load float, ptr %452, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_16.le.9.i6666 = load float, ptr %453, align 8, !alias.scope !6502, !noalias !6505, !noundef !11
  %_12.le.i6685 = load float, ptr %454, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.i6686 = load float, ptr %455, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.1.i6687 = load float, ptr %456, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.1.i6688 = load float, ptr %457, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.2.i6689 = load float, ptr %458, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.2.i6690 = load float, ptr %459, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.3.i6691 = load float, ptr %460, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.3.i6692 = load float, ptr %461, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.4.i6693 = load float, ptr %462, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.4.i6694 = load float, ptr %463, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.5.i6695 = load float, ptr %464, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.5.i6696 = load float, ptr %465, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.6.i6697 = load float, ptr %466, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.6.i6698 = load float, ptr %467, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.7.i6699 = load float, ptr %468, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.7.i6700 = load float, ptr %469, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.8.i6701 = load float, ptr %470, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.8.i6702 = load float, ptr %471, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_12.le.9.i6703 = load float, ptr %472, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  %_16.le.9.i6704 = load float, ptr %473, align 8, !alias.scope !6507, !noalias !6510, !noundef !11
  call void @llvm.lifetime.start.p0(ptr nonnull %_20.i122.i), !dbg !6512, !noalias !6516
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_20.i122.i, ptr noalias noundef align 8 dereferenceable(360) %433, i32 noundef %sample_rate.i127.i) #25, !dbg !6517, !noalias !3842
  call void @llvm.lifetime.start.p0(ptr nonnull %_22.i121.i), !dbg !6518, !noalias !6516
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_22.i121.i, ptr noalias noundef align 8 dereferenceable(360) %_18.i136.i, i32 noundef %sample_rate.i127.i) #25, !dbg !6519, !noalias !3842
  %coefficients.i123.i.sroa.0.0.copyload = load float, ptr %_20.i122.i, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.5.0.copyload = load float, ptr %coefficients.i123.i.sroa.5.0._20.i122.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.7.0.copyload = load float, ptr %coefficients.i123.i.sroa.7.0._20.i122.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.9.0.copyload = load float, ptr %coefficients.i123.i.sroa.9.0._20.i122.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.11.0.copyload = load float, ptr %coefficients.i123.i.sroa.11.0._20.i122.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.13.0.copyload = load float, ptr %coefficients.i123.i.sroa.13.0._20.i122.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.15.24.copyload = load float, ptr %_22.i121.i, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.18.24.copyload = load float, ptr %coefficients.i123.i.sroa.18.24._22.i121.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.20.24.copyload = load float, ptr %coefficients.i123.i.sroa.20.24._22.i121.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.22.24.copyload = load float, ptr %coefficients.i123.i.sroa.22.24._22.i121.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.24.24.copyload = load float, ptr %coefficients.i123.i.sroa.24.24._22.i121.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  %coefficients.i123.i.sroa.26.24.copyload = load float, ptr %coefficients.i123.i.sroa.26.24._22.i121.i.sroa_idx, align 4, !dbg !6520, !noalias !6516
  call void @llvm.lifetime.end.p0(ptr nonnull %_22.i121.i), !dbg !6521, !noalias !6516
  call void @llvm.lifetime.end.p0(ptr nonnull %_20.i122.i), !dbg !6521, !noalias !6516
  %_32.i342.i = add i64 %plan.0.i134.i, %position.sroa.0.0.i130.i10907, !dbg !6522
  %_72.i343.i = icmp ult i64 %_32.i342.i, %position.sroa.0.0.i130.i10907, !dbg !6524
  %_66.not.i344.i = icmp ugt i64 %_32.i342.i, %_19.1
  %or.cond11.i345.i = or i1 %_72.i343.i, %_66.not.i344.i, !dbg !6524
  br i1 %plan.1.i135.i, label %bb9.i340.i, label %bb13.i137.i, !dbg !6530

bb9.i340.i:                                       ; preds = %bb2.i132.i
  br i1 %or.cond11.i345.i, label %bb19.i556.i, label %bb17.i346.i, !dbg !6531, !prof !3878

bb13.i137.i:                                      ; preds = %bb2.i132.i
  br i1 %or.cond11.i345.i, label %bb29.i339.i, label %bb27.i143.i, !dbg !6535, !prof !3878

bb27.i143.i:                                      ; preds = %bb13.i137.i
  %_95.i144.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i130.i10907, !dbg !6541
  %_105.i147.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i130.i10907, !dbg !6545
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6552), !dbg !6555
  %filter_near.i.i119.i.sroa.0.0.copyload = load float, ptr %474, align 8, !dbg !6556, !noalias !6562
  %filter_near.i.i119.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.7.0..sroa_idx, align 4, !dbg !6556, !noalias !6562
  %filter_near.i.i119.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.11.0..sroa_idx, align 8, !dbg !6556, !noalias !6562
  %filter_near.i.i119.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.14.0..sroa_idx, align 4, !dbg !6556, !noalias !6562
  %filter_far.i.i118.i.sroa.0.0.copyload = load float, ptr %475, align 8, !dbg !6567, !noalias !6562
  %filter_far.i.i118.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.7.0..sroa_idx, align 4, !dbg !6567, !noalias !6562
  %filter_far.i.i118.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.11.0..sroa_idx, align 8, !dbg !6567, !noalias !6562
  %filter_far.i.i118.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.14.0..sroa_idx, align 4, !dbg !6567, !noalias !6562
  %511 = load i32, ptr %476, align 8, !dbg !6569
  %512 = load i32, ptr %.sroa_idx6990, align 4, !dbg !6569
  %513 = load i32, ptr %477, align 8, !dbg !6571
  %514 = load i32, ptr %.sroa_idx6995, align 4, !dbg !6571
  %515 = load i64, ptr %_51.i138.i, align 8, !dbg !6573, !alias.scope !6575, !noalias !6576, !noundef !11
  %_164.i.i159.i10718.not = icmp eq i64 %plan.0.i134.i, 0, !dbg !6578
  br i1 %_164.i.i159.i10718.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_KBZ_EB2_.exit.i.i, label %bb54.i.i163.i, !dbg !6588

bb29.i339.i:                                      ; preds = %bb13.i137.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i130.i10907, i64 noundef %_32.i342.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a05c61cb172b1016332ce1d3ce81e461) #26, !dbg !6589, !noalias !3842
  unreachable, !dbg !6589

bb54.i.i163.i:                                    ; preds = %bb27.i143.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997
  %iter.sroa.0.0.i.i158.i10732 = phi i64 [ %516, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ 0, %bb27.i143.i ]
  %position.sroa.0.0.i.i157.i10731 = phi i64 [ %_37.sroa.0.0.i.i166.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %515, %bb27.i143.i ]
  %gain_far.i.i116.i.sroa.6.010730 = phi i32 [ %_3.i4339, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %514, %bb27.i143.i ]
  %gain_far.i.i116.i.sroa.0.010729 = phi i32 [ %_3.i4343, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %513, %bb27.i143.i ]
  %gain_near.i.i117.i.sroa.6.010728 = phi i32 [ %_3.i4347, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %512, %bb27.i143.i ]
  %gain_near.i.i117.i.sroa.0.010727 = phi i32 [ %_3.i4351, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %511, %bb27.i143.i ]
  %filter_far.i.i118.i.sroa.14.010726 = phi float [ %_0.i4204, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %filter_far.i.i118.i.sroa.14.0.copyload, %bb27.i143.i ]
  %filter_far.i.i118.i.sroa.11.010725 = phi float [ %_0.i4208, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %filter_far.i.i118.i.sroa.11.0.copyload, %bb27.i143.i ]
  %filter_far.i.i118.i.sroa.7.010724 = phi float [ %_0.i4196, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %filter_far.i.i118.i.sroa.7.0.copyload, %bb27.i143.i ]
  %filter_far.i.i118.i.sroa.0.010723 = phi float [ %_0.i4200, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %filter_far.i.i118.i.sroa.0.0.copyload, %bb27.i143.i ]
  %filter_near.i.i119.i.sroa.14.010722 = phi float [ %_0.i4220, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %filter_near.i.i119.i.sroa.14.0.copyload, %bb27.i143.i ]
  %filter_near.i.i119.i.sroa.11.010721 = phi float [ %_0.i4224, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %filter_near.i.i119.i.sroa.11.0.copyload, %bb27.i143.i ]
  %filter_near.i.i119.i.sroa.7.010720 = phi float [ %_0.i4212, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %filter_near.i.i119.i.sroa.7.0.copyload, %bb27.i143.i ]
  %filter_near.i.i119.i.sroa.0.010719 = phi float [ %_0.i4216, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], [ %filter_near.i.i119.i.sroa.0.0.copyload, %bb27.i143.i ]
  %516 = add nuw i64 %iter.sroa.0.0.i.i158.i10732, 1, !dbg !6590
  %_38.i.i164.i = add i64 %position.sroa.0.0.i.i157.i10731, 1, !dbg !6596
  %_172.not.i.i165.i = icmp ult i64 %_38.i.i164.i, %ring_len.i128.i, !dbg !6599
  %517 = select i1 %_172.not.i.i165.i, i64 0, i64 %ring_len.i128.i, !dbg !6599
  %_37.sroa.0.0.i.i166.i = sub nuw i64 %_38.i.i164.i, %517, !dbg !6599
  %_180.i.i168.i = getelementptr inbounds nuw float, ptr %_95.i144.i, i64 %iter.sroa.0.0.i.i158.i10732, !dbg !6602
  %_0.i3616 = load float, ptr %_180.i.i168.i, align 4, !dbg !6612, !alias.scope !6614, !noalias !6617, !noundef !11
  %_188.i.i171.i = getelementptr inbounds nuw float, ptr %_105.i147.i, i64 %iter.sroa.0.0.i.i158.i10732, !dbg !6618
  %_0.i3611 = load float, ptr %_188.i.i171.i, align 4, !dbg !6627, !alias.scope !6629, !noalias !6617, !noundef !11
  %_7.i80 = load float, ptr %_63.i.i173.i, align 4, !dbg !6632, !alias.scope !6635, !noalias !6638, !noundef !11
  %_8.i81 = load float, ptr %478, align 4, !dbg !6640, !alias.scope !6635, !noalias !6638, !noundef !11
  %_9.i82 = load float, ptr %479, align 4, !dbg !6641, !alias.scope !6635, !noalias !6638, !noundef !11
  %_0.i3423 = fsub float %_0.i3616, %filter_near.i.i119.i.sroa.7.010720, !dbg !6642
  %_0.i3246 = fmul float %_0.i3423, %_8.i81, !dbg !6645
  %_4.i2883 = fmul float %filter_near.i.i119.i.sroa.0.010719, %_7.i80, !dbg !6647
  %_0.i2884 = fadd float %_4.i2883, %_0.i3246, !dbg !6647
  %_0.i2726 = fadd float %filter_near.i.i119.i.sroa.0.010719, %_0.i2884, !dbg !6649
  %_0.i3245 = fmul float %filter_near.i.i119.i.sroa.0.010719, %_8.i81, !dbg !6651
  %_4.i2881 = fmul float %_0.i3423, %_9.i82, !dbg !6653
  %_0.i2882 = fadd float %_0.i3245, %_4.i2881, !dbg !6653
  %_0.i2725 = fadd float %filter_near.i.i119.i.sroa.7.010720, %_0.i2882, !dbg !6655
  %_0.i2724 = fadd float %_0.i2884, %_0.i2884, !dbg !6657
  %_0.i2723 = fadd float %filter_near.i.i119.i.sroa.0.010719, %_0.i2724, !dbg !6659
  %518 = tail call noundef float @llvm.fabs.f32(float %_0.i2723), !dbg !6661
  %519 = fcmp uge float %518, 0x3BC79CA100000000, !dbg !6664
  %_0.i4216 = select i1 %519, float %_0.i2723, float 0.000000e+00, !dbg !6666
  %_0.i2722 = fadd float %_0.i2882, %_0.i2882, !dbg !6667
  %_0.i2721 = fadd float %filter_near.i.i119.i.sroa.7.010720, %_0.i2722, !dbg !6669
  %520 = tail call noundef float @llvm.fabs.f32(float %_0.i2721), !dbg !6671
  %521 = fcmp uge float %520, 0x3BC79CA100000000, !dbg !6674
  %_0.i4212 = select i1 %521, float %_0.i2721, float 0.000000e+00, !dbg !6676
  %_12.i85 = load float, ptr %480, align 4, !dbg !6677, !alias.scope !6635, !noalias !6638, !noundef !11
  %_4.i2929 = fmul float %_12.i85, %_0.i2726, !dbg !6678
  %_0.i2930 = fadd float %_0.i3616, %_4.i2929, !dbg !6678
  %_0.i3424 = fsub float %_0.i2725, %filter_near.i.i119.i.sroa.14.010722, !dbg !6680
  %_0.i3248 = fmul float %_8.i81, %_0.i3424, !dbg !6683
  %_4.i2887 = fmul float %filter_near.i.i119.i.sroa.11.010721, %_7.i80, !dbg !6685
  %_0.i2888 = fadd float %_4.i2887, %_0.i3248, !dbg !6685
  %_0.i3247 = fmul float %filter_near.i.i119.i.sroa.11.010721, %_8.i81, !dbg !6687
  %_4.i2885 = fmul float %_9.i82, %_0.i3424, !dbg !6689
  %_0.i2886 = fadd float %_0.i3247, %_4.i2885, !dbg !6689
  %_0.i2731 = fadd float %filter_near.i.i119.i.sroa.14.010722, %_0.i2886, !dbg !6691
  %_0.i2730 = fadd float %_0.i2888, %_0.i2888, !dbg !6693
  %_0.i2729 = fadd float %filter_near.i.i119.i.sroa.11.010721, %_0.i2730, !dbg !6695
  %522 = tail call noundef float @llvm.fabs.f32(float %_0.i2729), !dbg !6697
  %523 = fcmp uge float %522, 0x3BC79CA100000000, !dbg !6700
  %_0.i4224 = select i1 %523, float %_0.i2729, float 0.000000e+00, !dbg !6702
  %_0.i2728 = fadd float %_0.i2886, %_0.i2886, !dbg !6703
  %_0.i2727 = fadd float %filter_near.i.i119.i.sroa.14.010722, %_0.i2728, !dbg !6705
  %524 = tail call noundef float @llvm.fabs.f32(float %_0.i2727), !dbg !6707
  %525 = fcmp uge float %524, 0x3BC79CA100000000, !dbg !6710
  %_0.i4220 = select i1 %525, float %_0.i2727, float 0.000000e+00, !dbg !6712
  %_0.i3437 = fsub float %_0.i2930, %_0.i2731, !dbg !6713
  %_7.i67 = load float, ptr %_68.i.i175.i, align 4, !dbg !6715, !alias.scope !6718, !noalias !6721, !noundef !11
  %_8.i68 = load float, ptr %481, align 4, !dbg !6723, !alias.scope !6718, !noalias !6721, !noundef !11
  %_9.i69 = load float, ptr %482, align 4, !dbg !6724, !alias.scope !6718, !noalias !6721, !noundef !11
  %_0.i3421 = fsub float %_0.i3611, %filter_far.i.i118.i.sroa.7.010724, !dbg !6725
  %_0.i3242 = fmul float %_0.i3421, %_8.i68, !dbg !6728
  %_4.i2875 = fmul float %filter_far.i.i118.i.sroa.0.010723, %_7.i67, !dbg !6730
  %_0.i2876 = fadd float %_4.i2875, %_0.i3242, !dbg !6730
  %_0.i2714 = fadd float %filter_far.i.i118.i.sroa.0.010723, %_0.i2876, !dbg !6732
  %_0.i3241 = fmul float %filter_far.i.i118.i.sroa.0.010723, %_8.i68, !dbg !6734
  %_4.i2873 = fmul float %_0.i3421, %_9.i69, !dbg !6736
  %_0.i2874 = fadd float %_0.i3241, %_4.i2873, !dbg !6736
  %_0.i2713 = fadd float %filter_far.i.i118.i.sroa.7.010724, %_0.i2874, !dbg !6738
  %_0.i2712 = fadd float %_0.i2876, %_0.i2876, !dbg !6740
  %_0.i2711 = fadd float %filter_far.i.i118.i.sroa.0.010723, %_0.i2712, !dbg !6742
  %526 = tail call noundef float @llvm.fabs.f32(float %_0.i2711), !dbg !6744
  %527 = fcmp uge float %526, 0x3BC79CA100000000, !dbg !6747
  %_0.i4200 = select i1 %527, float %_0.i2711, float 0.000000e+00, !dbg !6749
  %_0.i2710 = fadd float %_0.i2874, %_0.i2874, !dbg !6750
  %_0.i2709 = fadd float %filter_far.i.i118.i.sroa.7.010724, %_0.i2710, !dbg !6752
  %528 = tail call noundef float @llvm.fabs.f32(float %_0.i2709), !dbg !6754
  %529 = fcmp uge float %528, 0x3BC79CA100000000, !dbg !6757
  %_0.i4196 = select i1 %529, float %_0.i2709, float 0.000000e+00, !dbg !6759
  %_12.i72 = load float, ptr %483, align 4, !dbg !6760, !alias.scope !6718, !noalias !6721, !noundef !11
  %_4.i2931 = fmul float %_12.i72, %_0.i2714, !dbg !6761
  %_0.i2932 = fadd float %_0.i3611, %_4.i2931, !dbg !6761
  %_0.i3422 = fsub float %_0.i2713, %filter_far.i.i118.i.sroa.14.010726, !dbg !6763
  %_0.i3244 = fmul float %_8.i68, %_0.i3422, !dbg !6766
  %_4.i2879 = fmul float %filter_far.i.i118.i.sroa.11.010725, %_7.i67, !dbg !6768
  %_0.i2880 = fadd float %_4.i2879, %_0.i3244, !dbg !6768
  %_0.i3243 = fmul float %filter_far.i.i118.i.sroa.11.010725, %_8.i68, !dbg !6770
  %_4.i2877 = fmul float %_9.i69, %_0.i3422, !dbg !6772
  %_0.i2878 = fadd float %_0.i3243, %_4.i2877, !dbg !6772
  %_0.i2719 = fadd float %filter_far.i.i118.i.sroa.14.010726, %_0.i2878, !dbg !6774
  %_0.i2718 = fadd float %_0.i2880, %_0.i2880, !dbg !6776
  %_0.i2717 = fadd float %filter_far.i.i118.i.sroa.11.010725, %_0.i2718, !dbg !6778
  %530 = tail call noundef float @llvm.fabs.f32(float %_0.i2717), !dbg !6780
  %531 = fcmp uge float %530, 0x3BC79CA100000000, !dbg !6783
  %_0.i4208 = select i1 %531, float %_0.i2717, float 0.000000e+00, !dbg !6785
  %_0.i2716 = fadd float %_0.i2878, %_0.i2878, !dbg !6786
  %_0.i2715 = fadd float %filter_far.i.i118.i.sroa.14.010726, %_0.i2716, !dbg !6788
  %532 = tail call noundef float @llvm.fabs.f32(float %_0.i2715), !dbg !6790
  %533 = fcmp uge float %532, 0x3BC79CA100000000, !dbg !6793
  %_0.i4204 = select i1 %533, float %_0.i2715, float 0.000000e+00, !dbg !6795
  %_0.i3438 = fsub float %_0.i2932, %_0.i2719, !dbg !6796
  %_307.1.i.i178.i = load i64, ptr %484, align 8, !dbg !6798, !noalias !6617, !noundef !11
  %_230.i.i179.i = icmp ugt i64 %position.sroa.0.0.i.i157.i10731, %_307.1.i.i178.i, !dbg !6800
  br i1 %_230.i.i179.i, label %bb76.i.i335.i, label %bb77.i.i180.i, !dbg !6800, !prof !1664

bb77.i.i180.i:                                    ; preds = %bb54.i.i163.i
  %_307.0.i.i181.i = load ptr, ptr %433, align 8, !dbg !6798, !noalias !6617, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6806), !dbg !6809
  %_4.not.i4014 = icmp eq i64 %_307.1.i.i178.i, %position.sroa.0.0.i.i157.i10731, !dbg !6810
  br i1 %_4.not.i4014, label %panic.i4016, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4017, !dbg !6810

panic.i4016:                                      ; preds = %bb77.i.i180.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6810, !noalias !6812
  unreachable, !dbg !6810

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4017: ; preds = %bb77.i.i180.i
  %_237.i.i184.i = getelementptr inbounds nuw float, ptr %_307.0.i.i181.i, i64 %position.sroa.0.0.i.i157.i10731, !dbg !6813
  store float %_0.i2731, ptr %_237.i.i184.i, align 4, !dbg !6810, !alias.scope !6806, !noalias !6617
  %_308.1.i.i185.i = load i64, ptr %486, align 8, !dbg !6819, !noalias !6617, !noundef !11
  %_238.i.i186.i = icmp ugt i64 %position.sroa.0.0.i.i157.i10731, %_308.1.i.i185.i, !dbg !6820
  br i1 %_238.i.i186.i, label %bb78.i.i334.i, label %bb79.i.i187.i, !dbg !6820, !prof !1664

bb76.i.i335.i:                                    ; preds = %bb54.i.i163.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i157.i10731, i64 noundef %_307.1.i.i178.i, i64 noundef %_307.1.i.i178.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7aeb6b0a3ba8e73c50a5abef6c30558) #26, !dbg !6824, !noalias !6617
  unreachable, !dbg !6824

bb79.i.i187.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4017
  %_308.0.i.i188.i = load ptr, ptr %485, align 8, !dbg !6819, !noalias !6617, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6825), !dbg !6828
  %_4.not.i4010 = icmp eq i64 %_308.1.i.i185.i, %position.sroa.0.0.i.i157.i10731, !dbg !6829
  br i1 %_4.not.i4010, label %panic.i4012, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4013, !dbg !6829

panic.i4012:                                      ; preds = %bb79.i.i187.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6829, !noalias !6831
  unreachable, !dbg !6829

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4013: ; preds = %bb79.i.i187.i
  %_245.i.i190.i = getelementptr inbounds nuw float, ptr %_308.0.i.i188.i, i64 %position.sroa.0.0.i.i157.i10731, !dbg !6832
  store float %_0.i3437, ptr %_245.i.i190.i, align 4, !dbg !6829, !alias.scope !6825, !noalias !6617
  %_309.1.i.i191.i = load i64, ptr %487, align 8, !dbg !6837, !noalias !6617, !noundef !11
  %_246.i.i192.i = icmp ugt i64 %position.sroa.0.0.i.i157.i10731, %_309.1.i.i191.i, !dbg !6838
  br i1 %_246.i.i192.i, label %bb80.i.i333.i, label %bb81.i.i193.i, !dbg !6838, !prof !1664

bb78.i.i334.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4017
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i157.i10731, i64 noundef %_308.1.i.i185.i, i64 noundef %_308.1.i.i185.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8e669d1bed0fb747f8a7bff920a6571) #26, !dbg !6842, !noalias !6617
  unreachable, !dbg !6842

bb81.i.i193.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4013
  %_309.0.i.i194.i = load ptr, ptr %_18.i136.i, align 8, !dbg !6837, !noalias !6617, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6843), !dbg !6846
  %_4.not.i4006 = icmp eq i64 %_309.1.i.i191.i, %position.sroa.0.0.i.i157.i10731, !dbg !6847
  br i1 %_4.not.i4006, label %panic.i4008, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4009, !dbg !6847

panic.i4008:                                      ; preds = %bb81.i.i193.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6847, !noalias !6849
  unreachable, !dbg !6847

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4009: ; preds = %bb81.i.i193.i
  %_253.i.i196.i = getelementptr inbounds nuw float, ptr %_309.0.i.i194.i, i64 %position.sroa.0.0.i.i157.i10731, !dbg !6850
  store float %_0.i2719, ptr %_253.i.i196.i, align 4, !dbg !6847, !alias.scope !6843, !noalias !6617
  %_310.1.i.i197.i = load i64, ptr %489, align 8, !dbg !6855, !noalias !6617, !noundef !11
  %_254.i.i198.i = icmp ugt i64 %position.sroa.0.0.i.i157.i10731, %_310.1.i.i197.i, !dbg !6856
  br i1 %_254.i.i198.i, label %bb82.i.i332.i, label %bb83.i.i199.i, !dbg !6856, !prof !1664

bb80.i.i333.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4013
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i157.i10731, i64 noundef %_309.1.i.i191.i, i64 noundef %_309.1.i.i191.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e81c2bc19b75441ce2fb90ce8f9eb70) #26, !dbg !6860, !noalias !6617
  unreachable, !dbg !6860

bb83.i.i199.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4009
  %_310.0.i.i200.i = load ptr, ptr %488, align 8, !dbg !6855, !noalias !6617, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6861), !dbg !6864
  %_4.not.i4002 = icmp eq i64 %_310.1.i.i197.i, %position.sroa.0.0.i.i157.i10731, !dbg !6865
  br i1 %_4.not.i4002, label %panic.i4004, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4005, !dbg !6865

panic.i4004:                                      ; preds = %bb83.i.i199.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6865, !noalias !6867
  unreachable, !dbg !6865

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4005: ; preds = %bb83.i.i199.i
  %_261.i.i202.i = getelementptr inbounds nuw float, ptr %_310.0.i.i200.i, i64 %position.sroa.0.0.i.i157.i10731, !dbg !6868
  store float %_0.i3438, ptr %_261.i.i202.i, align 4, !dbg !6865, !alias.scope !6861, !noalias !6617
  %_311.0.i.i203.i = load ptr, ptr %433, align 8, !dbg !6873, !noalias !6617, !nonnull !11, !noundef !11
  %_311.1.i.i204.i = load i64, ptr %484, align 8, !dbg !6873, !noalias !6617, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6874), !dbg !6877
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6878), !dbg !6877
  %_13.i267.i323.i = load i64, ptr %_85.i.i205.i, align 8, !alias.scope !6878, !noalias !6880, !noundef !11
  %_12.i268.i324.i = add i64 %_13.i267.i323.i, %position.sroa.0.0.i.i157.i10731
  %_31.not.i269.i325.i = icmp ult i64 %_12.i268.i324.i, %ring_len.i128.i
  %534 = select i1 %_31.not.i269.i325.i, i64 0, i64 %ring_len.i128.i
  %_11.sroa.0.0.i270.i326.i = sub nuw i64 %_12.i268.i324.i, %534
  %_16.i271.i327.i = icmp ult i64 %_11.sroa.0.0.i270.i326.i, %_311.1.i.i204.i
  br i1 %_16.i271.i327.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i209.i.split.us, label %panic1.i272.i328.i

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i209.i.split.us: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4005
  %535 = getelementptr inbounds nuw float, ptr %_311.0.i.i203.i, i64 %_11.sroa.0.0.i270.i326.i
  %_8.i274.i330.i.us.le = load float, ptr %535, align 4, !alias.scope !6874, !noalias !6881, !noundef !11
  %_312.0.i.i211.i = load ptr, ptr %485, align 8, !dbg !6882, !noalias !6617, !nonnull !11, !noundef !11
  %_312.1.i.i212.i = load i64, ptr %486, align 8, !dbg !6882, !noalias !6617, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6884), !dbg !6887
  %_16.i254.i317.i = icmp ult i64 %_11.sroa.0.0.i270.i326.i, %_312.1.i.i212.i
  br i1 %_16.i254.i317.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i216.i.split.us, label %panic1.i255.i318.i

panic1.i272.i328.i:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4005
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i270.i326.i, i64 noundef range(i64 0, 2305843009213693952) %_311.1.i.i204.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !6888, !noalias !6890
  unreachable, !dbg !6888

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i216.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i209.i.split.us
  %536 = getelementptr inbounds nuw float, ptr %_312.0.i.i211.i, i64 %_11.sroa.0.0.i270.i326.i
  %_8.i257.i320.i.us.le = load float, ptr %536, align 4, !alias.scope !6884, !noalias !6891, !noundef !11
  %_313.0.i.i218.i = load ptr, ptr %_18.i136.i, align 8, !dbg !6893, !noalias !6617, !nonnull !11, !noundef !11
  %_313.1.i.i219.i = load i64, ptr %487, align 8, !dbg !6893, !noalias !6617, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6895), !dbg !6898
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6899), !dbg !6898
  %_13.i233.i303.i = load i64, ptr %_93.i.i220.i, align 8, !alias.scope !6899, !noalias !6901, !noundef !11
  %_12.i234.i304.i = add i64 %_13.i233.i303.i, %position.sroa.0.0.i.i157.i10731
  %_31.not.i235.i305.i = icmp ult i64 %_12.i234.i304.i, %ring_len.i128.i
  %537 = select i1 %_31.not.i235.i305.i, i64 0, i64 %ring_len.i128.i
  %_11.sroa.0.0.i236.i306.i = sub nuw i64 %_12.i234.i304.i, %537
  %_16.i237.i307.i = icmp ult i64 %_11.sroa.0.0.i236.i306.i, %_313.1.i.i219.i
  br i1 %_16.i237.i307.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i224.i.split.us, label %panic1.i238.i308.i

panic1.i255.i318.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i209.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i270.i326.i, i64 noundef range(i64 0, 2305843009213693952) %_312.1.i.i212.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !6902, !noalias !6904
  unreachable, !dbg !6902

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i224.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i216.i.split.us
  %538 = getelementptr inbounds nuw float, ptr %_313.0.i.i218.i, i64 %_11.sroa.0.0.i236.i306.i
  %_8.i240.i310.i.us.le = load float, ptr %538, align 4, !alias.scope !6895, !noalias !6905, !noundef !11
  %_314.0.i.i226.i = load ptr, ptr %488, align 8, !dbg !6906, !noalias !6617, !nonnull !11, !noundef !11
  %_314.1.i.i227.i = load i64, ptr %489, align 8, !dbg !6906, !noalias !6617, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6908), !dbg !6911
  %_16.i220.i297.i = icmp ult i64 %_11.sroa.0.0.i236.i306.i, %_314.1.i.i227.i
  br i1 %_16.i220.i297.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i231.i.split.us, label %panic1.i221.i298.i

panic1.i238.i308.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i216.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i236.i306.i, i64 noundef range(i64 0, 2305843009213693952) %_313.1.i.i219.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !6912, !noalias !6914
  unreachable, !dbg !6912

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i231.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i224.i.split.us
  %539 = getelementptr inbounds nuw float, ptr %_314.0.i.i226.i, i64 %_11.sroa.0.0.i236.i306.i
  %_8.i223.i300.i.us.le = load float, ptr %539, align 4, !alias.scope !6908, !noalias !6915, !noundef !11
  %540 = tail call noundef float @llvm.fabs.f32(float %_8.i274.i330.i.us.le), !dbg !6917
  %541 = tail call noundef float @llvm.fabs.f32(float %_8.i240.i310.i.us.le), !dbg !6922
  %_3.i.i5242.inv = fcmp ogt float %540, %541, !dbg !6924
  %_4.i.i5249.v = select i1 %_3.i.i5242.inv, float %540, float %541, !dbg !6924
  %_4.i.i5249 = bitcast float %_4.i.i5249.v to i32, !dbg !6924
  %_3.i.i5654 = fcmp ule float %_4.i.i5249.v, 0x3E45798EE0000000, !dbg !6928
  %_4.i.i5660 = select i1 %_3.i.i5654, i32 841731191, i32 %_4.i.i5249, !dbg !6934
  %_0.i.i5661 = bitcast i32 %_4.i.i5660 to float, !dbg !6936
  %_3.i.i4922 = fcmp ule float %_0.i.i5661, 0x3810000000000000, !dbg !6938
  %_4.i.i4928 = select i1 %_3.i.i4922, i32 8388608, i32 %_4.i.i5660, !dbg !6943
  %_5.i3797 = and i32 %_4.i.i4928, 8388607, !dbg !6945
  %_4.i3798 = or disjoint i32 %_5.i3797, 1065353216, !dbg !6945
  %significand.i3799 = bitcast i32 %_4.i3798 to float, !dbg !6947
  %_0.i3345 = fadd float %significand.i3799, -1.000000e+00, !dbg !6949
  %_0.i3010 = fmul float %_0.i3345, 0xBF9B17A960000000, !dbg !6951
  %_0.i2530 = fadd float %_0.i3010, 0x3FBF9A8440000000, !dbg !6953
  %_0.i3010.1 = fmul float %_0.i3345, %_0.i2530, !dbg !6951
  %_0.i2530.1 = fadd float %_0.i3010.1, 0xBFD1E3F400000000, !dbg !6953
  %_0.i3010.2 = fmul float %_0.i3345, %_0.i2530.1, !dbg !6951
  %_0.i2530.2 = fadd float %_0.i3010.2, 0x3FDD544F20000000, !dbg !6953
  %_0.i3010.3 = fmul float %_0.i3345, %_0.i2530.2, !dbg !6951
  %_0.i2530.3 = fadd float %_0.i3010.3, 0xBFE6FC2A60000000, !dbg !6953
  %_0.i3010.4 = fmul float %_0.i3345, %_0.i2530.3, !dbg !6951
  %_0.i2530.4 = fadd float %_0.i3010.4, 0x3FF714B2A0000000, !dbg !6953
  %_9.i3800 = lshr i32 %_4.i.i4928, 23, !dbg !6955
  %_8.i3801 = or disjoint i32 %_9.i3800, 1258291200, !dbg !6955
  %_7.i3802 = bitcast i32 %_8.i3801 to float, !dbg !6956
  %exponent.i3803 = fadd float %_7.i3802, 0xC160000FE0000000, !dbg !6958
  %_0.i3009 = fmul float %_0.i3345, %_0.i2530.4, !dbg !6959
  %_0.i2529 = fadd float %exponent.i3803, %_0.i3009, !dbg !6961
  %_0.i3312 = fmul float %_0.i2529, 0x4018151820000000, !dbg !6963
  %_3.i.i5646.inv = fcmp ogt float %_0.i3312, -1.600000e+02, !dbg !6965
  %_0.i.i5653 = select i1 %_3.i.i5646.inv, float %_0.i3312, float -1.600000e+02, !dbg !6965
  %_3.i.i6292.inv = fcmp olt float %_0.i.i5653, 2.400000e+01, !dbg !6968
  %_0.i.i6299 = select i1 %_3.i.i6292.inv, float %_0.i.i5653, float 2.400000e+01, !dbg !6968
  %_0.i3393 = fsub float %_0.i.i6299, %_12.le.i6647, !dbg !6971
  %_3.i2241 = fcmp ule float %_0.i3393, 3.000000e+00, !dbg !6974
  %_0.i2617 = fadd float %_0.i3393, 3.000000e+00, !dbg !6976
  %_0.i3147 = fmul float %_0.i2617, %_0.i2617, !dbg !6978
  %_0.i3146 = fmul float %_0.i3147, 0x3FB5555560000000, !dbg !6980
  %_4.i4500.v.v = select i1 %_3.i2241, float %_0.i3146, float %_0.i3393, !dbg !6982
  %_4.i4500.v = fmul float %coefficients.i123.i.sroa.0.0.copyload, %_4.i4500.v.v, !dbg !6982
  %_4.i4500 = bitcast float %_4.i4500.v to i32, !dbg !6982
  %542 = fcmp ugt float %_0.i3393, -3.000000e+00, !dbg !6984
  %_7.i4492 = select i1 %542, i32 %_4.i4500, i32 0, !dbg !6986
  %_0.i4494 = bitcast i32 %_7.i4492 to float, !dbg !6987
  %_3.i.i5638 = fcmp ule float %_0.i4494, -1.000000e+02, !dbg !6989
  %543 = bitcast i32 %_7.i4492 to float, !dbg !6992
  %_0.i.i5645 = select i1 %_3.i.i5638, float -1.000000e+02, float %543, !dbg !6995
  %_3.i.i6284 = fcmp olt float %_0.i.i5645, 0.000000e+00, !dbg !6996
  %_0.i.i6291 = select i1 %_3.i.i6284, float %_0.i.i5645, float 0.000000e+00, !dbg !6999
  %544 = bitcast i32 %gain_near.i.i117.i.sroa.0.010727 to float, !dbg !7001
  %_3.i2479 = fcmp uge float %_0.i.i6291, %544, !dbg !7002
  %_4.i4806.v = select i1 %_3.i2479, float %coefficients.i123.i.sroa.7.0.copyload, float %coefficients.i123.i.sroa.5.0.copyload, !dbg !7005
  %_0.i3460 = fsub float %544, %_0.i.i6291, !dbg !7007
  %_4.i2975 = fmul float %_0.i3460, %_4.i4806.v, !dbg !7009
  %_0.i2976 = fadd float %_0.i.i6291, %_4.i2975, !dbg !7009
  %545 = tail call noundef float @llvm.fabs.f32(float %_0.i2976), !dbg !7011
  %_4.i4349 = bitcast float %_0.i2976 to i32, !dbg !7014
  %546 = fcmp uge float %545, 0x3BC79CA100000000, !dbg !7017
  %_3.i4351 = select i1 %546, i32 %_4.i4349, i32 0, !dbg !7018
  %_0.i4352 = bitcast i32 %_3.i4351 to float, !dbg !7019
  %_0.i2808 = fadd float %_12.le.4.i6655, %_0.i4352, !dbg !7021
  %_0.i3311 = fmul float %_0.i2808, 0x3FC542A5A0000000, !dbg !7023
  %_3.i.i5114.inv = fcmp ogt float %_0.i3311, -1.260000e+02, !dbg !7026
  %_0.i.i5121 = select i1 %_3.i.i5114.inv, float %_0.i3311, float -1.260000e+02, !dbg !7026
  %_3.i.i5916.inv = fcmp olt float %_0.i.i5121, 1.270000e+02, !dbg !7030
  %_0.i.i5923 = select i1 %_3.i.i5916.inv, float %_0.i.i5121, float 1.270000e+02, !dbg !7030
  %547 = tail call noundef float @llvm.floor.f32(float %_0.i.i5923), !dbg !7033
  %_0.i3369 = fsub float %_0.i.i5923, %547, !dbg !7037
  %548 = tail call noundef float @llvm.fabs.f32(float %_8.i257.i320.i.us.le), !dbg !7039
  %549 = tail call noundef float @llvm.fabs.f32(float %_8.i223.i300.i.us.le), !dbg !7042
  %_3.i.i5251.inv = fcmp ogt float %548, %549, !dbg !7044
  %_4.i.i5258.v = select i1 %_3.i.i5251.inv, float %548, float %549, !dbg !7044
  %_4.i.i5258 = bitcast float %_4.i.i5258.v to i32, !dbg !7044
  %_3.i.i5630 = fcmp ule float %_4.i.i5258.v, 0x3E45798EE0000000, !dbg !7047
  %_4.i.i5636 = select i1 %_3.i.i5630, i32 841731191, i32 %_4.i.i5258, !dbg !7052
  %_0.i.i5637 = bitcast i32 %_4.i.i5636 to float, !dbg !7054
  %_3.i.i4930 = fcmp ule float %_0.i.i5637, 0x3810000000000000, !dbg !7056
  %_4.i.i4936 = select i1 %_3.i.i4930, i32 8388608, i32 %_4.i.i5636, !dbg !7061
  %_5.i3805 = and i32 %_4.i.i4936, 8388607, !dbg !7063
  %_4.i3806 = or disjoint i32 %_5.i3805, 1065353216, !dbg !7063
  %significand.i3807 = bitcast i32 %_4.i3806 to float, !dbg !7065
  %_0.i3346 = fadd float %significand.i3807, -1.000000e+00, !dbg !7067
  %_0.i3012 = fmul float %_0.i3346, 0xBF9B17A960000000, !dbg !7069
  %_0.i2532 = fadd float %_0.i3012, 0x3FBF9A8440000000, !dbg !7071
  %_0.i3012.1 = fmul float %_0.i3346, %_0.i2532, !dbg !7069
  %_0.i2532.1 = fadd float %_0.i3012.1, 0xBFD1E3F400000000, !dbg !7071
  %_0.i3012.2 = fmul float %_0.i3346, %_0.i2532.1, !dbg !7069
  %_0.i2532.2 = fadd float %_0.i3012.2, 0x3FDD544F20000000, !dbg !7071
  %_0.i3012.3 = fmul float %_0.i3346, %_0.i2532.2, !dbg !7069
  %_0.i2532.3 = fadd float %_0.i3012.3, 0xBFE6FC2A60000000, !dbg !7071
  %_0.i3012.4 = fmul float %_0.i3346, %_0.i2532.3, !dbg !7069
  %_0.i2532.4 = fadd float %_0.i3012.4, 0x3FF714B2A0000000, !dbg !7071
  %_9.i3808 = lshr i32 %_4.i.i4936, 23, !dbg !7073
  %_8.i3809 = or disjoint i32 %_9.i3808, 1258291200, !dbg !7073
  %_7.i3810 = bitcast i32 %_8.i3809 to float, !dbg !7074
  %exponent.i3811 = fadd float %_7.i3810, 0xC160000FE0000000, !dbg !7076
  %_0.i3011 = fmul float %_0.i3346, %_0.i2532.4, !dbg !7077
  %_0.i2531 = fadd float %exponent.i3811, %_0.i3011, !dbg !7079
  %_0.i3310 = fmul float %_0.i2531, 0x4018151820000000, !dbg !7081
  %_3.i.i5622.inv = fcmp ogt float %_0.i3310, -1.600000e+02, !dbg !7083
  %_0.i.i5629 = select i1 %_3.i.i5622.inv, float %_0.i3310, float -1.600000e+02, !dbg !7083
  %_3.i.i6276.inv = fcmp olt float %_0.i.i5629, 2.400000e+01, !dbg !7086
  %_0.i.i6283 = select i1 %_3.i.i6276.inv, float %_0.i.i5629, float 2.400000e+01, !dbg !7086
  %_0.i3394 = fsub float %_0.i.i6283, %_12.le.5.i6657, !dbg !7089
  %_3.i2243 = fcmp ule float %_0.i3394, 3.000000e+00, !dbg !7092
  %_0.i2618 = fadd float %_0.i3394, 3.000000e+00, !dbg !7094
  %_0.i3151 = fmul float %_0.i2618, %_0.i2618, !dbg !7096
  %_0.i3150 = fmul float %_0.i3151, 0x3FB5555560000000, !dbg !7098
  %_4.i4513.v.v = select i1 %_3.i2243, float %_0.i3150, float %_0.i3394, !dbg !7100
  %_4.i4513.v = fmul float %coefficients.i123.i.sroa.9.0.copyload, %_4.i4513.v.v, !dbg !7100
  %_4.i4513 = bitcast float %_4.i4513.v to i32, !dbg !7100
  %550 = fcmp ugt float %_0.i3394, -3.000000e+00, !dbg !7102
  %_7.i4505 = select i1 %550, i32 %_4.i4513, i32 0, !dbg !7104
  %_0.i4507 = bitcast i32 %_7.i4505 to float, !dbg !7105
  %_3.i.i5614 = fcmp ule float %_0.i4507, -1.000000e+02, !dbg !7107
  %551 = bitcast i32 %_7.i4505 to float, !dbg !7110
  %_0.i.i5621 = select i1 %_3.i.i5614, float -1.000000e+02, float %551, !dbg !7113
  %_3.i.i6268 = fcmp olt float %_0.i.i5621, 0.000000e+00, !dbg !7114
  %_0.i.i6275 = select i1 %_3.i.i6268, float %_0.i.i5621, float 0.000000e+00, !dbg !7117
  %552 = bitcast i32 %gain_near.i.i117.i.sroa.6.010728 to float, !dbg !7119
  %_3.i2475 = fcmp uge float %_0.i.i6275, %552, !dbg !7120
  %_4.i4799.v = select i1 %_3.i2475, float %coefficients.i123.i.sroa.13.0.copyload, float %coefficients.i123.i.sroa.11.0.copyload, !dbg !7123
  %_0.i3459 = fsub float %552, %_0.i.i6275, !dbg !7125
  %_4.i2973 = fmul float %_0.i3459, %_4.i4799.v, !dbg !7127
  %_0.i2974 = fadd float %_0.i.i6275, %_4.i2973, !dbg !7127
  %553 = tail call noundef float @llvm.fabs.f32(float %_0.i2974), !dbg !7129
  %_4.i4345 = bitcast float %_0.i2974 to i32, !dbg !7132
  %554 = fcmp uge float %553, 0x3BC79CA100000000, !dbg !7135
  %_3.i4347 = select i1 %554, i32 %_4.i4345, i32 0, !dbg !7136
  %_0.i4348 = bitcast i32 %_3.i4347 to float, !dbg !7137
  %_0.i2807 = fadd float %_12.le.9.i6665, %_0.i4348, !dbg !7139
  %_0.i3309 = fmul float %_0.i2807, 0x3FC542A5A0000000, !dbg !7141
  %_3.i.i5122.inv = fcmp ogt float %_0.i3309, -1.260000e+02, !dbg !7144
  %_0.i.i5129 = select i1 %_3.i.i5122.inv, float %_0.i3309, float -1.260000e+02, !dbg !7144
  %_3.i.i5924.inv = fcmp olt float %_0.i.i5129, 1.270000e+02, !dbg !7148
  %_0.i.i5931 = select i1 %_3.i.i5924.inv, float %_0.i.i5129, float 1.270000e+02, !dbg !7148
  %555 = tail call noundef float @llvm.floor.f32(float %_0.i.i5931), !dbg !7151
  %_0.i3370 = fsub float %_0.i.i5931, %555, !dbg !7155
  %_0.i3395 = fsub float %_0.i.i6299, %_12.le.i6685, !dbg !7157
  %_3.i2245 = fcmp ule float %_0.i3395, 3.000000e+00, !dbg !7162
  %_0.i2619 = fadd float %_0.i3395, 3.000000e+00, !dbg !7164
  %_0.i3155 = fmul float %_0.i2619, %_0.i2619, !dbg !7166
  %_0.i3154 = fmul float %_0.i3155, 0x3FB5555560000000, !dbg !7168
  %_4.i4526.v.v = select i1 %_3.i2245, float %_0.i3154, float %_0.i3395, !dbg !7170
  %_4.i4526.v = fmul float %coefficients.i123.i.sroa.15.24.copyload, %_4.i4526.v.v, !dbg !7170
  %_4.i4526 = bitcast float %_4.i4526.v to i32, !dbg !7170
  %556 = fcmp ugt float %_0.i3395, -3.000000e+00, !dbg !7172
  %_7.i4518 = select i1 %556, i32 %_4.i4526, i32 0, !dbg !7174
  %_0.i4520 = bitcast i32 %_7.i4518 to float, !dbg !7175
  %_3.i.i5590 = fcmp ule float %_0.i4520, -1.000000e+02, !dbg !7177
  %557 = bitcast i32 %_7.i4518 to float, !dbg !7180
  %_0.i.i5597 = select i1 %_3.i.i5590, float -1.000000e+02, float %557, !dbg !7183
  %_3.i.i6252 = fcmp olt float %_0.i.i5597, 0.000000e+00, !dbg !7184
  %_0.i.i6259 = select i1 %_3.i.i6252, float %_0.i.i5597, float 0.000000e+00, !dbg !7187
  %558 = bitcast i32 %gain_far.i.i116.i.sroa.0.010729 to float, !dbg !7189
  %_3.i2471 = fcmp uge float %_0.i.i6259, %558, !dbg !7190
  %_4.i4792.v = select i1 %_3.i2471, float %coefficients.i123.i.sroa.20.24.copyload, float %coefficients.i123.i.sroa.18.24.copyload, !dbg !7193
  %_0.i3458 = fsub float %558, %_0.i.i6259, !dbg !7195
  %_4.i2971 = fmul float %_0.i3458, %_4.i4792.v, !dbg !7197
  %_0.i2972 = fadd float %_0.i.i6259, %_4.i2971, !dbg !7197
  %559 = tail call noundef float @llvm.fabs.f32(float %_0.i2972), !dbg !7199
  %_4.i4341 = bitcast float %_0.i2972 to i32, !dbg !7202
  %560 = fcmp uge float %559, 0x3BC79CA100000000, !dbg !7205
  %_3.i4343 = select i1 %560, i32 %_4.i4341, i32 0, !dbg !7206
  %_0.i4344 = bitcast i32 %_3.i4343 to float, !dbg !7207
  %_0.i2806 = fadd float %_12.le.4.i6693, %_0.i4344, !dbg !7209
  %_0.i3307 = fmul float %_0.i2806, 0x3FC542A5A0000000, !dbg !7211
  %_3.i.i5130.inv = fcmp ogt float %_0.i3307, -1.260000e+02, !dbg !7214
  %_0.i.i5137 = select i1 %_3.i.i5130.inv, float %_0.i3307, float -1.260000e+02, !dbg !7214
  %_3.i.i5932.inv = fcmp olt float %_0.i.i5137, 1.270000e+02, !dbg !7218
  %_0.i.i5939 = select i1 %_3.i.i5932.inv, float %_0.i.i5137, float 1.270000e+02, !dbg !7218
  %561 = tail call noundef float @llvm.floor.f32(float %_0.i.i5939), !dbg !7221
  %_0.i3371 = fsub float %_0.i.i5939, %561, !dbg !7225
  %_0.i3396 = fsub float %_0.i.i6283, %_12.le.5.i6695, !dbg !7227
  %_3.i2247 = fcmp ule float %_0.i3396, 3.000000e+00, !dbg !7232
  %_0.i2620 = fadd float %_0.i3396, 3.000000e+00, !dbg !7234
  %_0.i3159 = fmul float %_0.i2620, %_0.i2620, !dbg !7236
  %_0.i3158 = fmul float %_0.i3159, 0x3FB5555560000000, !dbg !7238
  %_4.i4539.v.v = select i1 %_3.i2247, float %_0.i3158, float %_0.i3396, !dbg !7240
  %_4.i4539.v = fmul float %coefficients.i123.i.sroa.22.24.copyload, %_4.i4539.v.v, !dbg !7240
  %_4.i4539 = bitcast float %_4.i4539.v to i32, !dbg !7240
  %562 = fcmp ugt float %_0.i3396, -3.000000e+00, !dbg !7242
  %_7.i4531 = select i1 %562, i32 %_4.i4539, i32 0, !dbg !7244
  %_0.i4533 = bitcast i32 %_7.i4531 to float, !dbg !7245
  %_3.i.i5566 = fcmp ule float %_0.i4533, -1.000000e+02, !dbg !7247
  %563 = bitcast i32 %_7.i4531 to float, !dbg !7250
  %_0.i.i5573 = select i1 %_3.i.i5566, float -1.000000e+02, float %563, !dbg !7253
  %_3.i.i6236 = fcmp olt float %_0.i.i5573, 0.000000e+00, !dbg !7254
  %_0.i.i6243 = select i1 %_3.i.i6236, float %_0.i.i5573, float 0.000000e+00, !dbg !7257
  %564 = bitcast i32 %gain_far.i.i116.i.sroa.6.010730 to float, !dbg !7259
  %_3.i2467 = fcmp uge float %_0.i.i6243, %564, !dbg !7260
  %_4.i4785.v = select i1 %_3.i2467, float %coefficients.i123.i.sroa.26.24.copyload, float %coefficients.i123.i.sroa.24.24.copyload, !dbg !7263
  %_0.i3457 = fsub float %564, %_0.i.i6243, !dbg !7265
  %_4.i2969 = fmul float %_0.i3457, %_4.i4785.v, !dbg !7267
  %_0.i2970 = fadd float %_0.i.i6243, %_4.i2969, !dbg !7267
  %565 = tail call noundef float @llvm.fabs.f32(float %_0.i2970), !dbg !7269
  %_4.i4337 = bitcast float %_0.i2970 to i32, !dbg !7272
  %566 = fcmp uge float %565, 0x3BC79CA100000000, !dbg !7275
  %_3.i4339 = select i1 %566, i32 %_4.i4337, i32 0, !dbg !7276
  %_0.i4340 = bitcast i32 %_3.i4339 to float, !dbg !7277
  %_0.i2805 = fadd float %_12.le.9.i6703, %_0.i4340, !dbg !7279
  %_0.i3305 = fmul float %_0.i2805, 0x3FC542A5A0000000, !dbg !7281
  %_3.i.i5138.inv = fcmp ogt float %_0.i3305, -1.260000e+02, !dbg !7284
  %_0.i.i5145 = select i1 %_3.i.i5138.inv, float %_0.i3305, float -1.260000e+02, !dbg !7284
  %_3.i.i5940.inv = fcmp olt float %_0.i.i5145, 1.270000e+02, !dbg !7288
  %_0.i.i5947 = select i1 %_3.i.i5940.inv, float %_0.i.i5145, float 1.270000e+02, !dbg !7288
  %567 = tail call noundef float @llvm.floor.f32(float %_0.i.i5947), !dbg !7291
  %_0.i3372 = fsub float %_0.i.i5947, %567, !dbg !7295
  %_0.i3076 = fmul float %_0.i3372, 0x3F5E974FA0000000, !dbg !7297
  %_0.i2584 = fadd float %_0.i3076, 0x3F82778560000000, !dbg !7299
  %_0.i3076.1 = fmul float %_0.i3372, %_0.i2584, !dbg !7297
  %_0.i2584.1 = fadd float %_0.i3076.1, 0x3FAC91CE60000000, !dbg !7299
  %_0.i3076.2 = fmul float %_0.i3372, %_0.i2584.1, !dbg !7297
  %_0.i2584.2 = fadd float %_0.i3076.2, 0x3FCEBDB560000000, !dbg !7299
  %_0.i3076.3 = fmul float %_0.i3372, %_0.i2584.2, !dbg !7297
  %_0.i2584.3 = fadd float %_0.i3076.3, 0x3FE62E4BA0000000, !dbg !7299
  %_0.i3073 = fmul float %_0.i3371, 0x3F5E974FA0000000, !dbg !7301
  %_0.i2582 = fadd float %_0.i3073, 0x3F82778560000000, !dbg !7303
  %_0.i3073.1 = fmul float %_0.i3371, %_0.i2582, !dbg !7301
  %_0.i2582.1 = fadd float %_0.i3073.1, 0x3FAC91CE60000000, !dbg !7303
  %_0.i3073.2 = fmul float %_0.i3371, %_0.i2582.1, !dbg !7301
  %_0.i2582.2 = fadd float %_0.i3073.2, 0x3FCEBDB560000000, !dbg !7303
  %_0.i3073.3 = fmul float %_0.i3371, %_0.i2582.2, !dbg !7301
  %_0.i2582.3 = fadd float %_0.i3073.3, 0x3FE62E4BA0000000, !dbg !7303
  %_0.i3070 = fmul float %_0.i3370, 0x3F5E974FA0000000, !dbg !7305
  %_0.i2580 = fadd float %_0.i3070, 0x3F82778560000000, !dbg !7307
  %_0.i3070.1 = fmul float %_0.i3370, %_0.i2580, !dbg !7305
  %_0.i2580.1 = fadd float %_0.i3070.1, 0x3FAC91CE60000000, !dbg !7307
  %_0.i3070.2 = fmul float %_0.i3370, %_0.i2580.1, !dbg !7305
  %_0.i2580.2 = fadd float %_0.i3070.2, 0x3FCEBDB560000000, !dbg !7307
  %_0.i3070.3 = fmul float %_0.i3370, %_0.i2580.2, !dbg !7305
  %_0.i2580.3 = fadd float %_0.i3070.3, 0x3FE62E4BA0000000, !dbg !7307
  %_0.i3067 = fmul float %_0.i3369, 0x3F5E974FA0000000, !dbg !7309
  %_0.i2578 = fadd float %_0.i3067, 0x3F82778560000000, !dbg !7311
  %_0.i3067.1 = fmul float %_0.i3369, %_0.i2578, !dbg !7309
  %_0.i2578.1 = fadd float %_0.i3067.1, 0x3FAC91CE60000000, !dbg !7311
  %_0.i3067.2 = fmul float %_0.i3369, %_0.i2578.1, !dbg !7309
  %_0.i2578.2 = fadd float %_0.i3067.2, 0x3FCEBDB560000000, !dbg !7311
  %_0.i3067.3 = fmul float %_0.i3369, %_0.i2578.2, !dbg !7309
  %_0.i2578.3 = fadd float %_0.i3067.3, 0x3FE62E4BA0000000, !dbg !7311
  %_0.i3066 = fmul float %_0.i3369, %_0.i2578.3, !dbg !7313
  %_0.i2577 = fadd float %_0.i3066, 1.000000e+00, !dbg !7315
  %biased.i2162 = fadd float %547, 0x4160000FE0000000, !dbg !7317
  %_4.i2163 = bitcast float %biased.i2162 to i32, !dbg !7319
  %_3.i2164 = shl i32 %_4.i2163, 23, !dbg !7321
  %_0.i2165 = bitcast i32 %_3.i2164 to float, !dbg !7322
  %_0.i3065 = fmul float %_0.i2577, %_0.i2165, !dbg !7324
  %_0.i3069 = fmul float %_0.i3370, %_0.i2580.3, !dbg !7326
  %_0.i2579 = fadd float %_0.i3069, 1.000000e+00, !dbg !7328
  %biased.i2166 = fadd float %555, 0x4160000FE0000000, !dbg !7330
  %_4.i2167 = bitcast float %biased.i2166 to i32, !dbg !7332
  %_3.i2168 = shl i32 %_4.i2167, 23, !dbg !7334
  %_0.i2169 = bitcast i32 %_3.i2168 to float, !dbg !7335
  %_0.i3068 = fmul float %_0.i2579, %_0.i2169, !dbg !7337
  %_0.i3072 = fmul float %_0.i3371, %_0.i2582.3, !dbg !7339
  %_0.i2581 = fadd float %_0.i3072, 1.000000e+00, !dbg !7341
  %biased.i2170 = fadd float %561, 0x4160000FE0000000, !dbg !7343
  %_4.i2171 = bitcast float %biased.i2170 to i32, !dbg !7345
  %_3.i2172 = shl i32 %_4.i2171, 23, !dbg !7347
  %_0.i2173 = bitcast i32 %_3.i2172 to float, !dbg !7348
  %_0.i3071 = fmul float %_0.i2581, %_0.i2173, !dbg !7350
  %_0.i3075 = fmul float %_0.i3372, %_0.i2584.3, !dbg !7352
  %_0.i2583 = fadd float %_0.i3075, 1.000000e+00, !dbg !7354
  %biased.i2174 = fadd float %567, 0x4160000FE0000000, !dbg !7356
  %_4.i2175 = bitcast float %biased.i2174 to i32, !dbg !7358
  %_3.i2176 = shl i32 %_4.i2175, 23, !dbg !7360
  %_0.i2177 = bitcast i32 %_3.i2176 to float, !dbg !7361
  %_0.i3074 = fmul float %_0.i2583, %_0.i2177, !dbg !7363
  %_262.i.i254.i = icmp ugt i64 %_37.sroa.0.0.i.i166.i, %_311.1.i.i204.i, !dbg !7365
  br i1 %_262.i.i254.i, label %bb84.i.i291.i, label %bb85.i.i255.i, !dbg !7365, !prof !1664

panic1.i221.i298.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i224.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i236.i306.i, i64 noundef range(i64 0, 2305843009213693952) %_314.1.i.i227.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !7370, !noalias !7372
  unreachable, !dbg !7370

bb82.i.i332.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4009
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i157.i10731, i64 noundef %_310.1.i.i197.i, i64 noundef %_310.1.i.i197.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_007bf1cfdcf9f845db5ef166fc9a1790) #26, !dbg !7373, !noalias !6617
  unreachable, !dbg !7373

bb85.i.i255.i:                                    ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i231.i.split.us
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7374), !dbg !7377
  %_3.not.i3596 = icmp eq i64 %_311.1.i.i204.i, %_37.sroa.0.0.i.i166.i, !dbg !7378
  br i1 %_3.not.i3596, label %panic.i3599, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3600, !dbg !7378

panic.i3599:                                      ; preds = %bb85.i.i255.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7378, !noalias !7380
  unreachable, !dbg !7378

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3600: ; preds = %bb85.i.i255.i
  %_269.i.i258.i = getelementptr inbounds nuw float, ptr %_311.0.i.i203.i, i64 %_37.sroa.0.0.i.i166.i, !dbg !7381
  %_0.i3598 = load float, ptr %_269.i.i258.i, align 4, !dbg !7378, !alias.scope !7374, !noalias !6617, !noundef !11
  %_0.i3304 = fmul float %_0.i3065, %_0.i3598, !dbg !7386
  %_270.i.i262.i = icmp ugt i64 %_37.sroa.0.0.i.i166.i, %_312.1.i.i212.i, !dbg !7388
  br i1 %_270.i.i262.i, label %bb86.i.i290.i, label %bb87.i.i263.i, !dbg !7388, !prof !1664

bb84.i.i291.i:                                    ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i231.i.split.us
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i166.i, i64 noundef %_311.1.i.i204.i, i64 noundef %_311.1.i.i204.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a45b75cd2d07085fe69fe186ba115725) #26, !dbg !7392, !noalias !6617
  unreachable, !dbg !7392

bb87.i.i263.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3600
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7393), !dbg !7396
  %_3.not.i3591 = icmp eq i64 %_312.1.i.i212.i, %_37.sroa.0.0.i.i166.i, !dbg !7397
  br i1 %_3.not.i3591, label %panic.i3594, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3595, !dbg !7397

panic.i3594:                                      ; preds = %bb87.i.i263.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7397, !noalias !7399
  unreachable, !dbg !7397

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3595: ; preds = %bb87.i.i263.i
  %_277.i.i266.i = getelementptr inbounds nuw float, ptr %_312.0.i.i211.i, i64 %_37.sroa.0.0.i.i166.i, !dbg !7400
  %_0.i3593 = load float, ptr %_277.i.i266.i, align 4, !dbg !7397, !alias.scope !7393, !noalias !6617, !noundef !11
  %_0.i3303 = fmul float %_0.i3068, %_0.i3593, !dbg !7405
  %_0.i2804 = fadd float %_0.i3304, %_0.i3303, !dbg !7407
  %_278.i.i271.i = icmp ugt i64 %_37.sroa.0.0.i.i166.i, %_313.1.i.i219.i, !dbg !7409
  br i1 %_278.i.i271.i, label %bb88.i.i289.i, label %bb89.i.i272.i, !dbg !7409, !prof !1664

bb86.i.i290.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3600
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i166.i, i64 noundef %_312.1.i.i212.i, i64 noundef %_312.1.i.i212.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_36be93341d7083c8a492c530428430ea) #26, !dbg !7414, !noalias !6617
  unreachable, !dbg !7414

bb89.i.i272.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3595
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7415), !dbg !7418
  %_3.not.i3586 = icmp eq i64 %_313.1.i.i219.i, %_37.sroa.0.0.i.i166.i, !dbg !7419
  br i1 %_3.not.i3586, label %panic.i3589, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590, !dbg !7419

panic.i3589:                                      ; preds = %bb89.i.i272.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7419, !noalias !7421
  unreachable, !dbg !7419

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590: ; preds = %bb89.i.i272.i
  %_285.i.i275.i = getelementptr inbounds nuw float, ptr %_313.0.i.i218.i, i64 %_37.sroa.0.0.i.i166.i, !dbg !7422
  %_0.i3588 = load float, ptr %_285.i.i275.i, align 4, !dbg !7419, !alias.scope !7415, !noalias !6617, !noundef !11
  %_0.i3302 = fmul float %_0.i3071, %_0.i3588, !dbg !7427
  %_286.i.i279.i = icmp ugt i64 %_37.sroa.0.0.i.i166.i, %_314.1.i.i227.i, !dbg !7429
  br i1 %_286.i.i279.i, label %bb90.i.i288.i, label %bb91.i.i280.i, !dbg !7429, !prof !1664

bb88.i.i289.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3595
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i166.i, i64 noundef %_313.1.i.i219.i, i64 noundef %_313.1.i.i219.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a6f1b2a44d3e33eba5c708677237c81) #26, !dbg !7433, !noalias !6617
  unreachable, !dbg !7433

bb91.i.i280.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7434), !dbg !7437
  %_3.not.i3581 = icmp eq i64 %_314.1.i.i227.i, %_37.sroa.0.0.i.i166.i, !dbg !7438
  br i1 %_3.not.i3581, label %panic.i3584, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997, !dbg !7438

panic.i3584:                                      ; preds = %bb91.i.i280.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7438, !noalias !7440
  unreachable, !dbg !7438

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997: ; preds = %bb91.i.i280.i
  %_293.i.i283.i = getelementptr inbounds nuw float, ptr %_314.0.i.i226.i, i64 %_37.sroa.0.0.i.i166.i, !dbg !7441
  %_0.i3583 = load float, ptr %_293.i.i283.i, align 4, !dbg !7438, !alias.scope !7434, !noalias !6617, !noundef !11
  %_0.i3301 = fmul float %_0.i3074, %_0.i3583, !dbg !7446
  %_0.i2803 = fadd float %_0.i3302, %_0.i3301, !dbg !7448
  store float %_0.i2804, ptr %_180.i.i168.i, align 4, !dbg !7450, !alias.scope !7453, !noalias !6617
  store float %_0.i2803, ptr %_188.i.i171.i, align 4, !dbg !7456, !alias.scope !7458, !noalias !6617
  %exitcond13930.not = icmp eq i64 %516, %plan.0.i134.i, !dbg !6578
  br i1 %exitcond13930.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_KBZ_EB2_.exit.i.i, label %bb54.i.i163.i, !dbg !6588

bb90.i.i288.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3590
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i166.i, i64 noundef %_314.1.i.i227.i, i64 noundef %_314.1.i.i227.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4aa2eaec3d1833a4a887fe1d76c05ca7) #26, !dbg !7461, !noalias !6617
  unreachable, !dbg !7461

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_KBZ_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997, %bb27.i143.i
  %filter_near.i.i119.i.sroa.0.0.lcssa = phi float [ %filter_near.i.i119.i.sroa.0.0.copyload, %bb27.i143.i ], [ %_0.i4216, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7462
  %filter_near.i.i119.i.sroa.7.0.lcssa = phi float [ %filter_near.i.i119.i.sroa.7.0.copyload, %bb27.i143.i ], [ %_0.i4212, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7462
  %filter_near.i.i119.i.sroa.11.0.lcssa = phi float [ %filter_near.i.i119.i.sroa.11.0.copyload, %bb27.i143.i ], [ %_0.i4224, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7462
  %filter_near.i.i119.i.sroa.14.0.lcssa = phi float [ %filter_near.i.i119.i.sroa.14.0.copyload, %bb27.i143.i ], [ %_0.i4220, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7462
  %filter_far.i.i118.i.sroa.0.0.lcssa = phi float [ %filter_far.i.i118.i.sroa.0.0.copyload, %bb27.i143.i ], [ %_0.i4200, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7463
  %filter_far.i.i118.i.sroa.7.0.lcssa = phi float [ %filter_far.i.i118.i.sroa.7.0.copyload, %bb27.i143.i ], [ %_0.i4196, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7463
  %filter_far.i.i118.i.sroa.11.0.lcssa = phi float [ %filter_far.i.i118.i.sroa.11.0.copyload, %bb27.i143.i ], [ %_0.i4208, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7463
  %filter_far.i.i118.i.sroa.14.0.lcssa = phi float [ %filter_far.i.i118.i.sroa.14.0.copyload, %bb27.i143.i ], [ %_0.i4204, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7463
  %gain_near.i.i117.i.sroa.0.0.lcssa = phi i32 [ %511, %bb27.i143.i ], [ %_3.i4351, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7464
  %gain_near.i.i117.i.sroa.6.0.lcssa = phi i32 [ %512, %bb27.i143.i ], [ %_3.i4347, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7464
  %gain_far.i.i116.i.sroa.0.0.lcssa = phi i32 [ %513, %bb27.i143.i ], [ %_3.i4343, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7465
  %gain_far.i.i116.i.sroa.6.0.lcssa = phi i32 [ %514, %bb27.i143.i ], [ %_3.i4339, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7465
  %position.sroa.0.0.i.i157.i.lcssa = phi i64 [ %515, %bb27.i143.i ], [ %_37.sroa.0.0.i.i166.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3997 ], !dbg !7466
  store float %filter_near.i.i119.i.sroa.0.0.lcssa, ptr %474, align 8, !dbg !7467, !noalias !6617
  store float %filter_near.i.i119.i.sroa.7.0.lcssa, ptr %filter_near.i.i119.i.sroa.7.0..sroa_idx, align 4, !dbg !7467, !noalias !6617
  store float %filter_near.i.i119.i.sroa.11.0.lcssa, ptr %filter_near.i.i119.i.sroa.11.0..sroa_idx, align 8, !dbg !7467, !noalias !6617
  store float %filter_near.i.i119.i.sroa.14.0.lcssa, ptr %filter_near.i.i119.i.sroa.14.0..sroa_idx, align 4, !dbg !7467, !noalias !6617
  store float %filter_far.i.i118.i.sroa.0.0.lcssa, ptr %475, align 8, !dbg !7468, !noalias !6617
  store float %filter_far.i.i118.i.sroa.7.0.lcssa, ptr %filter_far.i.i118.i.sroa.7.0..sroa_idx, align 4, !dbg !7468, !noalias !6617
  store float %filter_far.i.i118.i.sroa.11.0.lcssa, ptr %filter_far.i.i118.i.sroa.11.0..sroa_idx, align 8, !dbg !7468, !noalias !6617
  store float %filter_far.i.i118.i.sroa.14.0.lcssa, ptr %filter_far.i.i118.i.sroa.14.0..sroa_idx, align 4, !dbg !7468, !noalias !6617
  store i32 %gain_near.i.i117.i.sroa.0.0.lcssa, ptr %476, align 8, !dbg !7469, !noalias !6617
  store i32 %gain_near.i.i117.i.sroa.6.0.lcssa, ptr %.sroa_idx6990, align 4, !dbg !7469, !noalias !6617
  store i32 %gain_far.i.i116.i.sroa.0.0.lcssa, ptr %477, align 8, !dbg !7470, !noalias !6617
  store i32 %gain_far.i.i116.i.sroa.6.0.lcssa, ptr %.sroa_idx6995, align 4, !dbg !7470, !noalias !6617
  store i64 %position.sroa.0.0.i.i157.i.lcssa, ptr %_51.i138.i, align 8, !dbg !7471, !alias.scope !6575, !noalias !6576
  br label %bb15.i162.i, !dbg !7472

bb15.i162.i:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_Kb1_EB2_.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_KBZ_EB2_.exit.i.i
  %_8.i131.i = icmp ult i64 %_32.i342.i, %_19.1, !dbg !6497
  br i1 %_8.i131.i, label %bb2.i132.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit, !dbg !6497

bb17.i346.i:                                      ; preds = %bb9.i340.i
  %_75.i347.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i130.i10907, !dbg !7473
  %_85.i350.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i130.i10907, !dbg !7476
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7483), !dbg !7486
  %filter_near.i17.i112.i.sroa.0.0.copyload = load float, ptr %474, align 8, !dbg !7487, !noalias !7493
  %filter_near.i17.i112.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.7.0..sroa_idx, align 4, !dbg !7487, !noalias !7493
  %filter_near.i17.i112.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.11.0..sroa_idx, align 8, !dbg !7487, !noalias !7493
  %filter_near.i17.i112.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.14.0..sroa_idx, align 4, !dbg !7487, !noalias !7493
  %filter_far.i16.i111.i.sroa.0.0.copyload = load float, ptr %475, align 8, !dbg !7498, !noalias !7493
  %filter_far.i16.i111.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.7.0..sroa_idx, align 4, !dbg !7498, !noalias !7493
  %filter_far.i16.i111.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.11.0..sroa_idx, align 8, !dbg !7498, !noalias !7493
  %filter_far.i16.i111.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.14.0..sroa_idx, align 4, !dbg !7498, !noalias !7493
  %568 = load i32, ptr %476, align 8, !dbg !7500
  %569 = load i32, ptr %.sroa_idx6990, align 4, !dbg !7500
  %570 = load i32, ptr %477, align 8, !dbg !7502
  %571 = load i32, ptr %.sroa_idx6995, align 4, !dbg !7502
  %572 = load i64, ptr %_51.i138.i, align 8, !dbg !7504, !alias.scope !7506, !noalias !7507, !noundef !11
  %_164.i30.i362.i10878.not = icmp eq i64 %plan.0.i134.i, 0, !dbg !7509
  br i1 %_164.i30.i362.i10878.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_Kb1_EB2_.exit.i.i, label %bb54.i33.i366.i, !dbg !7519

bb19.i556.i:                                      ; preds = %bb9.i340.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i130.i10907, i64 noundef %_32.i342.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_61a2f59006034c74bb8ab3eed52140c6) #26, !dbg !7520, !noalias !3842
  unreachable, !dbg !7520

bb54.i33.i366.i:                                  ; preds = %bb17.i346.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973
  %segments.i126.i.sroa.0.1 = phi float [ %_0.i2796, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.i6647, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.8.1 = phi float [ %_0.i2796.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.1.i6649, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.12.1 = phi float [ %_0.i2796.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.2.i6651, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.16.1 = phi float [ %_0.i2796.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.3.i6653, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.20.1 = phi float [ %_0.i2796.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.4.i6655, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.26.1 = phi float [ %_0.i2796.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.5.i6657, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.32.1 = phi float [ %_0.i2796.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.6.i6659, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.36.1 = phi float [ %_0.i2796.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.7.i6661, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.40.1 = phi float [ %_0.i2796.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.8.i6663, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.44.1 = phi float [ %_0.i2796.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.9.i6665, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.70.1 = phi float [ %_0.i2795, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.i6685, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.76.1 = phi float [ %_0.i2795.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.1.i6687, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.80.1 = phi float [ %_0.i2795.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.2.i6689, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.84.1 = phi float [ %_0.i2795.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.3.i6691, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.88.1 = phi float [ %_0.i2795.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.4.i6693, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.94.1 = phi float [ %_0.i2795.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.5.i6695, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.100.1 = phi float [ %_0.i2795.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.6.i6697, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.104.1 = phi float [ %_0.i2795.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.7.i6699, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.108.1 = phi float [ %_0.i2795.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.8.i6701, %bb17.i346.i ], !dbg !7521
  %segments.i126.i.sroa.112.1 = phi float [ %_0.i2795.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %_12.le.9.i6703, %bb17.i346.i ], !dbg !7521
  %iter.sroa.0.0.i29.i361.i10892 = phi i64 [ %573, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ 0, %bb17.i346.i ]
  %position.sroa.0.0.i28.i360.i10891 = phi i64 [ %_37.sroa.0.0.i36.i373.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %572, %bb17.i346.i ]
  %gain_far.i14.i109.i.sroa.6.010890 = phi i32 [ %_3.i4323, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %571, %bb17.i346.i ]
  %gain_far.i14.i109.i.sroa.0.010889 = phi i32 [ %_3.i4327, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %570, %bb17.i346.i ]
  %gain_near.i15.i110.i.sroa.6.010888 = phi i32 [ %_3.i4331, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %569, %bb17.i346.i ]
  %gain_near.i15.i110.i.sroa.0.010887 = phi i32 [ %_3.i4335, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %568, %bb17.i346.i ]
  %filter_far.i16.i111.i.sroa.14.010886 = phi float [ %_0.i4172, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %filter_far.i16.i111.i.sroa.14.0.copyload, %bb17.i346.i ]
  %filter_far.i16.i111.i.sroa.11.010885 = phi float [ %_0.i4176, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %filter_far.i16.i111.i.sroa.11.0.copyload, %bb17.i346.i ]
  %filter_far.i16.i111.i.sroa.7.010884 = phi float [ %_0.i4164, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %filter_far.i16.i111.i.sroa.7.0.copyload, %bb17.i346.i ]
  %filter_far.i16.i111.i.sroa.0.010883 = phi float [ %_0.i4168, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %filter_far.i16.i111.i.sroa.0.0.copyload, %bb17.i346.i ]
  %filter_near.i17.i112.i.sroa.14.010882 = phi float [ %_0.i4188, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %filter_near.i17.i112.i.sroa.14.0.copyload, %bb17.i346.i ]
  %filter_near.i17.i112.i.sroa.11.010881 = phi float [ %_0.i4192, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %filter_near.i17.i112.i.sroa.11.0.copyload, %bb17.i346.i ]
  %filter_near.i17.i112.i.sroa.7.010880 = phi float [ %_0.i4180, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %filter_near.i17.i112.i.sroa.7.0.copyload, %bb17.i346.i ]
  %filter_near.i17.i112.i.sroa.0.010879 = phi float [ %_0.i4184, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], [ %filter_near.i17.i112.i.sroa.0.0.copyload, %bb17.i346.i ]
  %_0.i2796 = fadd float %segments.i126.i.sroa.0.1, %_16.le.i6648, !dbg !7522
  %_0.i2795 = fadd float %segments.i126.i.sroa.70.1, %_16.le.i6686, !dbg !7527
  %_0.i2796.1 = fadd float %segments.i126.i.sroa.8.1, %_16.le.1.i6650, !dbg !7522
  %_0.i2795.1 = fadd float %segments.i126.i.sroa.76.1, %_16.le.1.i6688, !dbg !7527
  %_0.i2796.2 = fadd float %segments.i126.i.sroa.12.1, %_16.le.2.i6652, !dbg !7522
  %_0.i2795.2 = fadd float %segments.i126.i.sroa.80.1, %_16.le.2.i6690, !dbg !7527
  %_0.i2796.3 = fadd float %segments.i126.i.sroa.16.1, %_16.le.3.i6654, !dbg !7522
  %_0.i2795.3 = fadd float %segments.i126.i.sroa.84.1, %_16.le.3.i6692, !dbg !7527
  %_0.i2796.4 = fadd float %segments.i126.i.sroa.20.1, %_16.le.4.i6656, !dbg !7522
  %_0.i2795.4 = fadd float %segments.i126.i.sroa.88.1, %_16.le.4.i6694, !dbg !7527
  %_0.i2796.5 = fadd float %segments.i126.i.sroa.26.1, %_16.le.5.i6658, !dbg !7522
  %_0.i2795.5 = fadd float %segments.i126.i.sroa.94.1, %_16.le.5.i6696, !dbg !7527
  %_0.i2796.6 = fadd float %segments.i126.i.sroa.32.1, %_16.le.6.i6660, !dbg !7522
  %_0.i2795.6 = fadd float %segments.i126.i.sroa.100.1, %_16.le.6.i6698, !dbg !7527
  %_0.i2796.7 = fadd float %segments.i126.i.sroa.36.1, %_16.le.7.i6662, !dbg !7522
  %_0.i2795.7 = fadd float %segments.i126.i.sroa.104.1, %_16.le.7.i6700, !dbg !7527
  %_0.i2796.8 = fadd float %segments.i126.i.sroa.40.1, %_16.le.8.i6664, !dbg !7522
  %_0.i2795.8 = fadd float %segments.i126.i.sroa.108.1, %_16.le.8.i6702, !dbg !7527
  %_0.i2796.9 = fadd float %segments.i126.i.sroa.44.1, %_16.le.9.i6666, !dbg !7522
  %_0.i2795.9 = fadd float %segments.i126.i.sroa.112.1, %_16.le.9.i6704, !dbg !7527
  %573 = add nuw i64 %iter.sroa.0.0.i29.i361.i10892, 1, !dbg !7529
  %_38.i34.i371.i = add i64 %position.sroa.0.0.i28.i360.i10891, 1, !dbg !7535
  %_172.not.i35.i372.i = icmp ult i64 %_38.i34.i371.i, %ring_len.i128.i, !dbg !7537
  %574 = select i1 %_172.not.i35.i372.i, i64 0, i64 %ring_len.i128.i, !dbg !7537
  %_37.sroa.0.0.i36.i373.i = sub nuw i64 %_38.i34.i371.i, %574, !dbg !7537
  %_180.i38.i377.i = getelementptr inbounds nuw float, ptr %_75.i347.i, i64 %iter.sroa.0.0.i29.i361.i10892, !dbg !7540
  %_0.i3578 = load float, ptr %_180.i38.i377.i, align 4, !dbg !7550, !alias.scope !7552, !noalias !7555, !noundef !11
  %_188.i43.i380.i = getelementptr inbounds nuw float, ptr %_85.i350.i, i64 %iter.sroa.0.0.i29.i361.i10892, !dbg !7556
  %_0.i3573 = load float, ptr %_188.i43.i380.i, align 4, !dbg !7565, !alias.scope !7567, !noalias !7555, !noundef !11
  %_7.i54 = load float, ptr %_63.i.i173.i, align 4, !dbg !7570, !alias.scope !7573, !noalias !7576, !noundef !11
  %_8.i55 = load float, ptr %478, align 4, !dbg !7578, !alias.scope !7573, !noalias !7576, !noundef !11
  %_9.i56 = load float, ptr %479, align 4, !dbg !7579, !alias.scope !7573, !noalias !7576, !noundef !11
  %_0.i3419 = fsub float %_0.i3578, %filter_near.i17.i112.i.sroa.7.010880, !dbg !7580
  %_0.i3238 = fmul float %_0.i3419, %_8.i55, !dbg !7583
  %_4.i2867 = fmul float %filter_near.i17.i112.i.sroa.0.010879, %_7.i54, !dbg !7585
  %_0.i2868 = fadd float %_4.i2867, %_0.i3238, !dbg !7585
  %_0.i2702 = fadd float %filter_near.i17.i112.i.sroa.0.010879, %_0.i2868, !dbg !7587
  %_0.i3237 = fmul float %filter_near.i17.i112.i.sroa.0.010879, %_8.i55, !dbg !7589
  %_4.i2865 = fmul float %_0.i3419, %_9.i56, !dbg !7591
  %_0.i2866 = fadd float %_0.i3237, %_4.i2865, !dbg !7591
  %_0.i2701 = fadd float %filter_near.i17.i112.i.sroa.7.010880, %_0.i2866, !dbg !7593
  %_0.i2700 = fadd float %_0.i2868, %_0.i2868, !dbg !7595
  %_0.i2699 = fadd float %filter_near.i17.i112.i.sroa.0.010879, %_0.i2700, !dbg !7597
  %575 = tail call noundef float @llvm.fabs.f32(float %_0.i2699), !dbg !7599
  %576 = fcmp uge float %575, 0x3BC79CA100000000, !dbg !7602
  %_0.i4184 = select i1 %576, float %_0.i2699, float 0.000000e+00, !dbg !7604
  %_0.i2698 = fadd float %_0.i2866, %_0.i2866, !dbg !7605
  %_0.i2697 = fadd float %filter_near.i17.i112.i.sroa.7.010880, %_0.i2698, !dbg !7607
  %577 = tail call noundef float @llvm.fabs.f32(float %_0.i2697), !dbg !7609
  %578 = fcmp uge float %577, 0x3BC79CA100000000, !dbg !7612
  %_0.i4180 = select i1 %578, float %_0.i2697, float 0.000000e+00, !dbg !7614
  %_12.i59 = load float, ptr %480, align 4, !dbg !7615, !alias.scope !7573, !noalias !7576, !noundef !11
  %_4.i2933 = fmul float %_12.i59, %_0.i2702, !dbg !7616
  %_0.i2934 = fadd float %_0.i3578, %_4.i2933, !dbg !7616
  %_0.i3420 = fsub float %_0.i2701, %filter_near.i17.i112.i.sroa.14.010882, !dbg !7618
  %_0.i3240 = fmul float %_8.i55, %_0.i3420, !dbg !7621
  %_4.i2871 = fmul float %filter_near.i17.i112.i.sroa.11.010881, %_7.i54, !dbg !7623
  %_0.i2872 = fadd float %_4.i2871, %_0.i3240, !dbg !7623
  %_0.i3239 = fmul float %filter_near.i17.i112.i.sroa.11.010881, %_8.i55, !dbg !7625
  %_4.i2869 = fmul float %_9.i56, %_0.i3420, !dbg !7627
  %_0.i2870 = fadd float %_0.i3239, %_4.i2869, !dbg !7627
  %_0.i2707 = fadd float %filter_near.i17.i112.i.sroa.14.010882, %_0.i2870, !dbg !7629
  %_0.i2706 = fadd float %_0.i2872, %_0.i2872, !dbg !7631
  %_0.i2705 = fadd float %filter_near.i17.i112.i.sroa.11.010881, %_0.i2706, !dbg !7633
  %579 = tail call noundef float @llvm.fabs.f32(float %_0.i2705), !dbg !7635
  %580 = fcmp uge float %579, 0x3BC79CA100000000, !dbg !7638
  %_0.i4192 = select i1 %580, float %_0.i2705, float 0.000000e+00, !dbg !7640
  %_0.i2704 = fadd float %_0.i2870, %_0.i2870, !dbg !7641
  %_0.i2703 = fadd float %filter_near.i17.i112.i.sroa.14.010882, %_0.i2704, !dbg !7643
  %581 = tail call noundef float @llvm.fabs.f32(float %_0.i2703), !dbg !7645
  %582 = fcmp uge float %581, 0x3BC79CA100000000, !dbg !7648
  %_0.i4188 = select i1 %582, float %_0.i2703, float 0.000000e+00, !dbg !7650
  %_0.i3439 = fsub float %_0.i2934, %_0.i2707, !dbg !7651
  %_7.i41 = load float, ptr %_68.i.i175.i, align 4, !dbg !7653, !alias.scope !7656, !noalias !7659, !noundef !11
  %_8.i42 = load float, ptr %481, align 4, !dbg !7661, !alias.scope !7656, !noalias !7659, !noundef !11
  %_9.i43 = load float, ptr %482, align 4, !dbg !7662, !alias.scope !7656, !noalias !7659, !noundef !11
  %_0.i3417 = fsub float %_0.i3573, %filter_far.i16.i111.i.sroa.7.010884, !dbg !7663
  %_0.i3234 = fmul float %_0.i3417, %_8.i42, !dbg !7666
  %_4.i2859 = fmul float %filter_far.i16.i111.i.sroa.0.010883, %_7.i41, !dbg !7668
  %_0.i2860 = fadd float %_4.i2859, %_0.i3234, !dbg !7668
  %_0.i2690 = fadd float %filter_far.i16.i111.i.sroa.0.010883, %_0.i2860, !dbg !7670
  %_0.i3233 = fmul float %filter_far.i16.i111.i.sroa.0.010883, %_8.i42, !dbg !7672
  %_4.i2857 = fmul float %_0.i3417, %_9.i43, !dbg !7674
  %_0.i2858 = fadd float %_0.i3233, %_4.i2857, !dbg !7674
  %_0.i2689 = fadd float %filter_far.i16.i111.i.sroa.7.010884, %_0.i2858, !dbg !7676
  %_0.i2688 = fadd float %_0.i2860, %_0.i2860, !dbg !7678
  %_0.i2687 = fadd float %filter_far.i16.i111.i.sroa.0.010883, %_0.i2688, !dbg !7680
  %583 = tail call noundef float @llvm.fabs.f32(float %_0.i2687), !dbg !7682
  %584 = fcmp uge float %583, 0x3BC79CA100000000, !dbg !7685
  %_0.i4168 = select i1 %584, float %_0.i2687, float 0.000000e+00, !dbg !7687
  %_0.i2686 = fadd float %_0.i2858, %_0.i2858, !dbg !7688
  %_0.i2685 = fadd float %filter_far.i16.i111.i.sroa.7.010884, %_0.i2686, !dbg !7690
  %585 = tail call noundef float @llvm.fabs.f32(float %_0.i2685), !dbg !7692
  %586 = fcmp uge float %585, 0x3BC79CA100000000, !dbg !7695
  %_0.i4164 = select i1 %586, float %_0.i2685, float 0.000000e+00, !dbg !7697
  %_12.i46 = load float, ptr %483, align 4, !dbg !7698, !alias.scope !7656, !noalias !7659, !noundef !11
  %_4.i2935 = fmul float %_12.i46, %_0.i2690, !dbg !7699
  %_0.i2936 = fadd float %_0.i3573, %_4.i2935, !dbg !7699
  %_0.i3418 = fsub float %_0.i2689, %filter_far.i16.i111.i.sroa.14.010886, !dbg !7701
  %_0.i3236 = fmul float %_8.i42, %_0.i3418, !dbg !7704
  %_4.i2863 = fmul float %filter_far.i16.i111.i.sroa.11.010885, %_7.i41, !dbg !7706
  %_0.i2864 = fadd float %_4.i2863, %_0.i3236, !dbg !7706
  %_0.i3235 = fmul float %filter_far.i16.i111.i.sroa.11.010885, %_8.i42, !dbg !7708
  %_4.i2861 = fmul float %_9.i43, %_0.i3418, !dbg !7710
  %_0.i2862 = fadd float %_0.i3235, %_4.i2861, !dbg !7710
  %_0.i2695 = fadd float %filter_far.i16.i111.i.sroa.14.010886, %_0.i2862, !dbg !7712
  %_0.i2694 = fadd float %_0.i2864, %_0.i2864, !dbg !7714
  %_0.i2693 = fadd float %filter_far.i16.i111.i.sroa.11.010885, %_0.i2694, !dbg !7716
  %587 = tail call noundef float @llvm.fabs.f32(float %_0.i2693), !dbg !7718
  %588 = fcmp uge float %587, 0x3BC79CA100000000, !dbg !7721
  %_0.i4176 = select i1 %588, float %_0.i2693, float 0.000000e+00, !dbg !7723
  %_0.i2692 = fadd float %_0.i2862, %_0.i2862, !dbg !7724
  %_0.i2691 = fadd float %filter_far.i16.i111.i.sroa.14.010886, %_0.i2692, !dbg !7726
  %589 = tail call noundef float @llvm.fabs.f32(float %_0.i2691), !dbg !7728
  %590 = fcmp uge float %589, 0x3BC79CA100000000, !dbg !7731
  %_0.i4172 = select i1 %590, float %_0.i2691, float 0.000000e+00, !dbg !7733
  %_0.i3440 = fsub float %_0.i2936, %_0.i2695, !dbg !7734
  %_307.1.i50.i387.i = load i64, ptr %484, align 8, !dbg !7736, !noalias !7555, !noundef !11
  %_230.i51.i388.i = icmp ugt i64 %position.sroa.0.0.i28.i360.i10891, %_307.1.i50.i387.i, !dbg !7738
  br i1 %_230.i51.i388.i, label %bb76.i151.i544.i, label %bb77.i52.i389.i, !dbg !7738, !prof !1664

bb77.i52.i389.i:                                  ; preds = %bb54.i33.i366.i
  %_307.0.i53.i390.i = load ptr, ptr %433, align 8, !dbg !7736, !noalias !7555, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7744), !dbg !7747
  %_4.not.i3990 = icmp eq i64 %_307.1.i50.i387.i, %position.sroa.0.0.i28.i360.i10891, !dbg !7748
  br i1 %_4.not.i3990, label %panic.i3992, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3993, !dbg !7748

panic.i3992:                                      ; preds = %bb77.i52.i389.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !7748, !noalias !7750
  unreachable, !dbg !7748

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3993: ; preds = %bb77.i52.i389.i
  %_237.i56.i393.i = getelementptr inbounds nuw float, ptr %_307.0.i53.i390.i, i64 %position.sroa.0.0.i28.i360.i10891, !dbg !7751
  store float %_0.i2707, ptr %_237.i56.i393.i, align 4, !dbg !7748, !alias.scope !7744, !noalias !7555
  %_308.1.i57.i394.i = load i64, ptr %486, align 8, !dbg !7757, !noalias !7555, !noundef !11
  %_238.i58.i395.i = icmp ugt i64 %position.sroa.0.0.i28.i360.i10891, %_308.1.i57.i394.i, !dbg !7758
  br i1 %_238.i58.i395.i, label %bb78.i150.i543.i, label %bb79.i59.i396.i, !dbg !7758, !prof !1664

bb76.i151.i544.i:                                 ; preds = %bb54.i33.i366.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i360.i10891, i64 noundef %_307.1.i50.i387.i, i64 noundef %_307.1.i50.i387.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7aeb6b0a3ba8e73c50a5abef6c30558) #26, !dbg !7762, !noalias !7555
  unreachable, !dbg !7762

bb79.i59.i396.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3993
  %_308.0.i60.i397.i = load ptr, ptr %485, align 8, !dbg !7757, !noalias !7555, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7763), !dbg !7766
  %_4.not.i3986 = icmp eq i64 %_308.1.i57.i394.i, %position.sroa.0.0.i28.i360.i10891, !dbg !7767
  br i1 %_4.not.i3986, label %panic.i3988, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3989, !dbg !7767

panic.i3988:                                      ; preds = %bb79.i59.i396.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !7767, !noalias !7769
  unreachable, !dbg !7767

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3989: ; preds = %bb79.i59.i396.i
  %_245.i62.i399.i = getelementptr inbounds nuw float, ptr %_308.0.i60.i397.i, i64 %position.sroa.0.0.i28.i360.i10891, !dbg !7770
  store float %_0.i3439, ptr %_245.i62.i399.i, align 4, !dbg !7767, !alias.scope !7763, !noalias !7555
  %_309.1.i63.i400.i = load i64, ptr %487, align 8, !dbg !7775, !noalias !7555, !noundef !11
  %_246.i64.i401.i = icmp ugt i64 %position.sroa.0.0.i28.i360.i10891, %_309.1.i63.i400.i, !dbg !7776
  br i1 %_246.i64.i401.i, label %bb80.i149.i542.i, label %bb81.i65.i402.i, !dbg !7776, !prof !1664

bb78.i150.i543.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3993
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i360.i10891, i64 noundef %_308.1.i57.i394.i, i64 noundef %_308.1.i57.i394.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8e669d1bed0fb747f8a7bff920a6571) #26, !dbg !7780, !noalias !7555
  unreachable, !dbg !7780

bb81.i65.i402.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3989
  %_309.0.i66.i403.i = load ptr, ptr %_18.i136.i, align 8, !dbg !7775, !noalias !7555, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7781), !dbg !7784
  %_4.not.i3982 = icmp eq i64 %_309.1.i63.i400.i, %position.sroa.0.0.i28.i360.i10891, !dbg !7785
  br i1 %_4.not.i3982, label %panic.i3984, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3985, !dbg !7785

panic.i3984:                                      ; preds = %bb81.i65.i402.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !7785, !noalias !7787
  unreachable, !dbg !7785

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3985: ; preds = %bb81.i65.i402.i
  %_253.i68.i405.i = getelementptr inbounds nuw float, ptr %_309.0.i66.i403.i, i64 %position.sroa.0.0.i28.i360.i10891, !dbg !7788
  store float %_0.i2695, ptr %_253.i68.i405.i, align 4, !dbg !7785, !alias.scope !7781, !noalias !7555
  %_310.1.i69.i406.i = load i64, ptr %489, align 8, !dbg !7793, !noalias !7555, !noundef !11
  %_254.i70.i407.i = icmp ugt i64 %position.sroa.0.0.i28.i360.i10891, %_310.1.i69.i406.i, !dbg !7794
  br i1 %_254.i70.i407.i, label %bb82.i148.i541.i, label %bb83.i71.i408.i, !dbg !7794, !prof !1664

bb80.i149.i542.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3989
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i360.i10891, i64 noundef %_309.1.i63.i400.i, i64 noundef %_309.1.i63.i400.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e81c2bc19b75441ce2fb90ce8f9eb70) #26, !dbg !7798, !noalias !7555
  unreachable, !dbg !7798

bb83.i71.i408.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3985
  %_310.0.i72.i409.i = load ptr, ptr %488, align 8, !dbg !7793, !noalias !7555, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7799), !dbg !7802
  %_4.not.i3978 = icmp eq i64 %_310.1.i69.i406.i, %position.sroa.0.0.i28.i360.i10891, !dbg !7803
  br i1 %_4.not.i3978, label %panic.i3980, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3981, !dbg !7803

panic.i3980:                                      ; preds = %bb83.i71.i408.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !7803, !noalias !7805
  unreachable, !dbg !7803

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3981: ; preds = %bb83.i71.i408.i
  %_261.i74.i411.i = getelementptr inbounds nuw float, ptr %_310.0.i72.i409.i, i64 %position.sroa.0.0.i28.i360.i10891, !dbg !7806
  store float %_0.i3440, ptr %_261.i74.i411.i, align 4, !dbg !7803, !alias.scope !7799, !noalias !7555
  %_311.0.i75.i412.i = load ptr, ptr %433, align 8, !dbg !7811, !noalias !7555, !nonnull !11, !noundef !11
  %_311.1.i76.i413.i = load i64, ptr %484, align 8, !dbg !7811, !noalias !7555, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7812), !dbg !7815
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7816), !dbg !7815
  %_13.i199.i532.i = load i64, ptr %_85.i.i205.i, align 8, !alias.scope !7816, !noalias !7818, !noundef !11
  %_12.i200.i533.i = add i64 %_13.i199.i532.i, %position.sroa.0.0.i28.i360.i10891
  %_31.not.i201.i534.i = icmp ult i64 %_12.i200.i533.i, %ring_len.i128.i
  %591 = select i1 %_31.not.i201.i534.i, i64 0, i64 %ring_len.i128.i
  %_11.sroa.0.0.i202.i535.i = sub nuw i64 %_12.i200.i533.i, %591
  %_16.i203.i536.i = icmp ult i64 %_11.sroa.0.0.i202.i535.i, %_311.1.i76.i413.i
  br i1 %_16.i203.i536.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i418.i.split.us, label %panic1.i204.i537.i

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i418.i.split.us: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3981
  %592 = getelementptr inbounds nuw float, ptr %_311.0.i75.i412.i, i64 %_11.sroa.0.0.i202.i535.i
  %_8.i206.i539.i.us.le = load float, ptr %592, align 4, !alias.scope !7812, !noalias !7819, !noundef !11
  %_312.0.i79.i420.i = load ptr, ptr %485, align 8, !dbg !7820, !noalias !7555, !nonnull !11, !noundef !11
  %_312.1.i80.i421.i = load i64, ptr %486, align 8, !dbg !7820, !noalias !7555, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7822), !dbg !7825
  %_16.i186.i526.i = icmp ult i64 %_11.sroa.0.0.i202.i535.i, %_312.1.i80.i421.i
  br i1 %_16.i186.i526.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i425.i.split.us, label %panic1.i187.i527.i

panic1.i204.i537.i:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3981
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i202.i535.i, i64 noundef range(i64 0, 2305843009213693952) %_311.1.i76.i413.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !7826, !noalias !7828
  unreachable, !dbg !7826

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i425.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i418.i.split.us
  %593 = getelementptr inbounds nuw float, ptr %_312.0.i79.i420.i, i64 %_11.sroa.0.0.i202.i535.i
  %_8.i189.i529.i.us.le = load float, ptr %593, align 4, !alias.scope !7822, !noalias !7829, !noundef !11
  %_313.0.i82.i427.i = load ptr, ptr %_18.i136.i, align 8, !dbg !7831, !noalias !7555, !nonnull !11, !noundef !11
  %_313.1.i83.i428.i = load i64, ptr %487, align 8, !dbg !7831, !noalias !7555, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7833), !dbg !7836
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7837), !dbg !7836
  %_13.i165.i512.i = load i64, ptr %_93.i.i220.i, align 8, !alias.scope !7837, !noalias !7839, !noundef !11
  %_12.i166.i513.i = add i64 %_13.i165.i512.i, %position.sroa.0.0.i28.i360.i10891
  %_31.not.i167.i514.i = icmp ult i64 %_12.i166.i513.i, %ring_len.i128.i
  %594 = select i1 %_31.not.i167.i514.i, i64 0, i64 %ring_len.i128.i
  %_11.sroa.0.0.i168.i515.i = sub nuw i64 %_12.i166.i513.i, %594
  %_16.i169.i516.i = icmp ult i64 %_11.sroa.0.0.i168.i515.i, %_313.1.i83.i428.i
  br i1 %_16.i169.i516.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i433.i.split.us, label %panic1.i170.i517.i

panic1.i187.i527.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i418.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i202.i535.i, i64 noundef range(i64 0, 2305843009213693952) %_312.1.i80.i421.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !7840, !noalias !7842
  unreachable, !dbg !7840

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i433.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i425.i.split.us
  %595 = getelementptr inbounds nuw float, ptr %_313.0.i82.i427.i, i64 %_11.sroa.0.0.i168.i515.i
  %_8.i172.i519.i.us.le = load float, ptr %595, align 4, !alias.scope !7833, !noalias !7843, !noundef !11
  %_314.0.i86.i435.i = load ptr, ptr %488, align 8, !dbg !7844, !noalias !7555, !nonnull !11, !noundef !11
  %_314.1.i87.i436.i = load i64, ptr %489, align 8, !dbg !7844, !noalias !7555, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7846), !dbg !7849
  %_16.i.i506.i = icmp ult i64 %_11.sroa.0.0.i168.i515.i, %_314.1.i87.i436.i
  br i1 %_16.i.i506.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i440.i.split.us, label %panic1.i156.i507.i

panic1.i170.i517.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i425.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i168.i515.i, i64 noundef range(i64 0, 2305843009213693952) %_313.1.i83.i428.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !7850, !noalias !7852
  unreachable, !dbg !7850

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i440.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i433.i.split.us
  %596 = getelementptr inbounds nuw float, ptr %_314.0.i86.i435.i, i64 %_11.sroa.0.0.i168.i515.i
  %_8.i.i509.i.us.le = load float, ptr %596, align 4, !alias.scope !7846, !noalias !7853, !noundef !11
  %597 = tail call noundef float @llvm.fabs.f32(float %_8.i206.i539.i.us.le), !dbg !7855
  %598 = tail call noundef float @llvm.fabs.f32(float %_8.i172.i519.i.us.le), !dbg !7859
  %_3.i.i5260.inv = fcmp ogt float %597, %598, !dbg !7861
  %_4.i.i5267.v = select i1 %_3.i.i5260.inv, float %597, float %598, !dbg !7861
  %_4.i.i5267 = bitcast float %_4.i.i5267.v to i32, !dbg !7861
  %_3.i.i5558 = fcmp ule float %_4.i.i5267.v, 0x3E45798EE0000000, !dbg !7864
  %_4.i.i5564 = select i1 %_3.i.i5558, i32 841731191, i32 %_4.i.i5267, !dbg !7870
  %_0.i.i5565 = bitcast i32 %_4.i.i5564 to float, !dbg !7872
  %_3.i.i4954 = fcmp ule float %_0.i.i5565, 0x3810000000000000, !dbg !7874
  %_4.i.i4960 = select i1 %_3.i.i4954, i32 8388608, i32 %_4.i.i5564, !dbg !7879
  %_5.i3829 = and i32 %_4.i.i4960, 8388607, !dbg !7881
  %_4.i3830 = or disjoint i32 %_5.i3829, 1065353216, !dbg !7881
  %significand.i3831 = bitcast i32 %_4.i3830 to float, !dbg !7883
  %_0.i3349 = fadd float %significand.i3831, -1.000000e+00, !dbg !7885
  %_0.i3018 = fmul float %_0.i3349, 0xBF9B17A960000000, !dbg !7887
  %_0.i2538 = fadd float %_0.i3018, 0x3FBF9A8440000000, !dbg !7889
  %_0.i3018.1 = fmul float %_0.i3349, %_0.i2538, !dbg !7887
  %_0.i2538.1 = fadd float %_0.i3018.1, 0xBFD1E3F400000000, !dbg !7889
  %_0.i3018.2 = fmul float %_0.i3349, %_0.i2538.1, !dbg !7887
  %_0.i2538.2 = fadd float %_0.i3018.2, 0x3FDD544F20000000, !dbg !7889
  %_0.i3018.3 = fmul float %_0.i3349, %_0.i2538.2, !dbg !7887
  %_0.i2538.3 = fadd float %_0.i3018.3, 0xBFE6FC2A60000000, !dbg !7889
  %_0.i3018.4 = fmul float %_0.i3349, %_0.i2538.3, !dbg !7887
  %_0.i2538.4 = fadd float %_0.i3018.4, 0x3FF714B2A0000000, !dbg !7889
  %_9.i3832 = lshr i32 %_4.i.i4960, 23, !dbg !7891
  %_8.i3833 = or disjoint i32 %_9.i3832, 1258291200, !dbg !7891
  %_7.i3834 = bitcast i32 %_8.i3833 to float, !dbg !7892
  %exponent.i3835 = fadd float %_7.i3834, 0xC160000FE0000000, !dbg !7894
  %_0.i3017 = fmul float %_0.i3349, %_0.i2538.4, !dbg !7895
  %_0.i2537 = fadd float %exponent.i3835, %_0.i3017, !dbg !7897
  %_0.i3300 = fmul float %_0.i2537, 0x4018151820000000, !dbg !7899
  %_3.i.i5550.inv = fcmp ogt float %_0.i3300, -1.600000e+02, !dbg !7901
  %_0.i.i5557 = select i1 %_3.i.i5550.inv, float %_0.i3300, float -1.600000e+02, !dbg !7901
  %_3.i.i6228.inv = fcmp olt float %_0.i.i5557, 2.400000e+01, !dbg !7904
  %_0.i.i6235 = select i1 %_3.i.i6228.inv, float %_0.i.i5557, float 2.400000e+01, !dbg !7904
  %_0.i3397 = fsub float %_0.i.i6235, %_0.i2796, !dbg !7907
  %_3.i2249 = fcmp ule float %_0.i3397, 3.000000e+00, !dbg !7910
  %_0.i2621 = fadd float %_0.i3397, 3.000000e+00, !dbg !7912
  %_0.i3163 = fmul float %_0.i2621, %_0.i2621, !dbg !7914
  %_0.i3162 = fmul float %_0.i3163, 0x3FB5555560000000, !dbg !7916
  %_4.i4552.v.v = select i1 %_3.i2249, float %_0.i3162, float %_0.i3397, !dbg !7918
  %_4.i4552.v = fmul float %coefficients.i123.i.sroa.0.0.copyload, %_4.i4552.v.v, !dbg !7918
  %_4.i4552 = bitcast float %_4.i4552.v to i32, !dbg !7918
  %599 = fcmp ugt float %_0.i3397, -3.000000e+00, !dbg !7920
  %_7.i4544 = select i1 %599, i32 %_4.i4552, i32 0, !dbg !7922
  %_0.i4546 = bitcast i32 %_7.i4544 to float, !dbg !7923
  %_3.i.i5542 = fcmp ule float %_0.i4546, -1.000000e+02, !dbg !7925
  %600 = bitcast i32 %_7.i4544 to float, !dbg !7928
  %_0.i.i5549 = select i1 %_3.i.i5542, float -1.000000e+02, float %600, !dbg !7931
  %_3.i.i6220 = fcmp olt float %_0.i.i5549, 0.000000e+00, !dbg !7932
  %_0.i.i6227 = select i1 %_3.i.i6220, float %_0.i.i5549, float 0.000000e+00, !dbg !7935
  %601 = bitcast i32 %gain_near.i15.i110.i.sroa.0.010887 to float, !dbg !7937
  %_3.i2463 = fcmp uge float %_0.i.i6227, %601, !dbg !7938
  %_4.i4778.v = select i1 %_3.i2463, float %coefficients.i123.i.sroa.7.0.copyload, float %coefficients.i123.i.sroa.5.0.copyload, !dbg !7941
  %_0.i3456 = fsub float %601, %_0.i.i6227, !dbg !7943
  %_4.i2967 = fmul float %_0.i3456, %_4.i4778.v, !dbg !7945
  %_0.i2968 = fadd float %_0.i.i6227, %_4.i2967, !dbg !7945
  %602 = tail call noundef float @llvm.fabs.f32(float %_0.i2968), !dbg !7947
  %_4.i4333 = bitcast float %_0.i2968 to i32, !dbg !7950
  %603 = fcmp uge float %602, 0x3BC79CA100000000, !dbg !7953
  %_3.i4335 = select i1 %603, i32 %_4.i4333, i32 0, !dbg !7954
  %_0.i4336 = bitcast i32 %_3.i4335 to float, !dbg !7955
  %_0.i2802 = fadd float %_0.i2796.4, %_0.i4336, !dbg !7957
  %_0.i3299 = fmul float %_0.i2802, 0x3FC542A5A0000000, !dbg !7959
  %_3.i.i5146.inv = fcmp ogt float %_0.i3299, -1.260000e+02, !dbg !7962
  %_0.i.i5153 = select i1 %_3.i.i5146.inv, float %_0.i3299, float -1.260000e+02, !dbg !7962
  %_3.i.i5948.inv = fcmp olt float %_0.i.i5153, 1.270000e+02, !dbg !7966
  %_0.i.i5955 = select i1 %_3.i.i5948.inv, float %_0.i.i5153, float 1.270000e+02, !dbg !7966
  %604 = tail call noundef float @llvm.floor.f32(float %_0.i.i5955), !dbg !7969
  %_0.i3373 = fsub float %_0.i.i5955, %604, !dbg !7973
  %605 = tail call noundef float @llvm.fabs.f32(float %_8.i189.i529.i.us.le), !dbg !7975
  %606 = tail call noundef float @llvm.fabs.f32(float %_8.i.i509.i.us.le), !dbg !7978
  %_3.i.i5269.inv = fcmp ogt float %605, %606, !dbg !7980
  %_4.i.i5276.v = select i1 %_3.i.i5269.inv, float %605, float %606, !dbg !7980
  %_4.i.i5276 = bitcast float %_4.i.i5276.v to i32, !dbg !7980
  %_3.i.i5534 = fcmp ule float %_4.i.i5276.v, 0x3E45798EE0000000, !dbg !7983
  %_4.i.i5540 = select i1 %_3.i.i5534, i32 841731191, i32 %_4.i.i5276, !dbg !7988
  %_0.i.i5541 = bitcast i32 %_4.i.i5540 to float, !dbg !7990
  %_3.i.i4962 = fcmp ule float %_0.i.i5541, 0x3810000000000000, !dbg !7992
  %_4.i.i4968 = select i1 %_3.i.i4962, i32 8388608, i32 %_4.i.i5540, !dbg !7997
  %_5.i3837 = and i32 %_4.i.i4968, 8388607, !dbg !7999
  %_4.i3838 = or disjoint i32 %_5.i3837, 1065353216, !dbg !7999
  %significand.i3839 = bitcast i32 %_4.i3838 to float, !dbg !8001
  %_0.i3350 = fadd float %significand.i3839, -1.000000e+00, !dbg !8003
  %_0.i3020 = fmul float %_0.i3350, 0xBF9B17A960000000, !dbg !8005
  %_0.i2540 = fadd float %_0.i3020, 0x3FBF9A8440000000, !dbg !8007
  %_0.i3020.1 = fmul float %_0.i3350, %_0.i2540, !dbg !8005
  %_0.i2540.1 = fadd float %_0.i3020.1, 0xBFD1E3F400000000, !dbg !8007
  %_0.i3020.2 = fmul float %_0.i3350, %_0.i2540.1, !dbg !8005
  %_0.i2540.2 = fadd float %_0.i3020.2, 0x3FDD544F20000000, !dbg !8007
  %_0.i3020.3 = fmul float %_0.i3350, %_0.i2540.2, !dbg !8005
  %_0.i2540.3 = fadd float %_0.i3020.3, 0xBFE6FC2A60000000, !dbg !8007
  %_0.i3020.4 = fmul float %_0.i3350, %_0.i2540.3, !dbg !8005
  %_0.i2540.4 = fadd float %_0.i3020.4, 0x3FF714B2A0000000, !dbg !8007
  %_9.i3840 = lshr i32 %_4.i.i4968, 23, !dbg !8009
  %_8.i3841 = or disjoint i32 %_9.i3840, 1258291200, !dbg !8009
  %_7.i3842 = bitcast i32 %_8.i3841 to float, !dbg !8010
  %exponent.i3843 = fadd float %_7.i3842, 0xC160000FE0000000, !dbg !8012
  %_0.i3019 = fmul float %_0.i3350, %_0.i2540.4, !dbg !8013
  %_0.i2539 = fadd float %exponent.i3843, %_0.i3019, !dbg !8015
  %_0.i3298 = fmul float %_0.i2539, 0x4018151820000000, !dbg !8017
  %_3.i.i5526.inv = fcmp ogt float %_0.i3298, -1.600000e+02, !dbg !8019
  %_0.i.i5533 = select i1 %_3.i.i5526.inv, float %_0.i3298, float -1.600000e+02, !dbg !8019
  %_3.i.i6212.inv = fcmp olt float %_0.i.i5533, 2.400000e+01, !dbg !8022
  %_0.i.i6219 = select i1 %_3.i.i6212.inv, float %_0.i.i5533, float 2.400000e+01, !dbg !8022
  %_0.i3398 = fsub float %_0.i.i6219, %_0.i2796.5, !dbg !8025
  %_3.i2251 = fcmp ule float %_0.i3398, 3.000000e+00, !dbg !8028
  %_0.i2622 = fadd float %_0.i3398, 3.000000e+00, !dbg !8030
  %_0.i3167 = fmul float %_0.i2622, %_0.i2622, !dbg !8032
  %_0.i3166 = fmul float %_0.i3167, 0x3FB5555560000000, !dbg !8034
  %_4.i4565.v.v = select i1 %_3.i2251, float %_0.i3166, float %_0.i3398, !dbg !8036
  %_4.i4565.v = fmul float %coefficients.i123.i.sroa.9.0.copyload, %_4.i4565.v.v, !dbg !8036
  %_4.i4565 = bitcast float %_4.i4565.v to i32, !dbg !8036
  %607 = fcmp ugt float %_0.i3398, -3.000000e+00, !dbg !8038
  %_7.i4557 = select i1 %607, i32 %_4.i4565, i32 0, !dbg !8040
  %_0.i4559 = bitcast i32 %_7.i4557 to float, !dbg !8041
  %_3.i.i5518 = fcmp ule float %_0.i4559, -1.000000e+02, !dbg !8043
  %608 = bitcast i32 %_7.i4557 to float, !dbg !8046
  %_0.i.i5525 = select i1 %_3.i.i5518, float -1.000000e+02, float %608, !dbg !8049
  %_3.i.i6204 = fcmp olt float %_0.i.i5525, 0.000000e+00, !dbg !8050
  %_0.i.i6211 = select i1 %_3.i.i6204, float %_0.i.i5525, float 0.000000e+00, !dbg !8053
  %609 = bitcast i32 %gain_near.i15.i110.i.sroa.6.010888 to float, !dbg !8055
  %_3.i2459 = fcmp uge float %_0.i.i6211, %609, !dbg !8056
  %_4.i4771.v = select i1 %_3.i2459, float %coefficients.i123.i.sroa.13.0.copyload, float %coefficients.i123.i.sroa.11.0.copyload, !dbg !8059
  %_0.i3455 = fsub float %609, %_0.i.i6211, !dbg !8061
  %_4.i2965 = fmul float %_0.i3455, %_4.i4771.v, !dbg !8063
  %_0.i2966 = fadd float %_0.i.i6211, %_4.i2965, !dbg !8063
  %610 = tail call noundef float @llvm.fabs.f32(float %_0.i2966), !dbg !8065
  %_4.i4329 = bitcast float %_0.i2966 to i32, !dbg !8068
  %611 = fcmp uge float %610, 0x3BC79CA100000000, !dbg !8071
  %_3.i4331 = select i1 %611, i32 %_4.i4329, i32 0, !dbg !8072
  %_0.i4332 = bitcast i32 %_3.i4331 to float, !dbg !8073
  %_0.i2801 = fadd float %_0.i2796.9, %_0.i4332, !dbg !8075
  %_0.i3297 = fmul float %_0.i2801, 0x3FC542A5A0000000, !dbg !8077
  %_3.i.i5154.inv = fcmp ogt float %_0.i3297, -1.260000e+02, !dbg !8080
  %_0.i.i5161 = select i1 %_3.i.i5154.inv, float %_0.i3297, float -1.260000e+02, !dbg !8080
  %_3.i.i5956.inv = fcmp olt float %_0.i.i5161, 1.270000e+02, !dbg !8084
  %_0.i.i5963 = select i1 %_3.i.i5956.inv, float %_0.i.i5161, float 1.270000e+02, !dbg !8084
  %612 = tail call noundef float @llvm.floor.f32(float %_0.i.i5963), !dbg !8087
  %_0.i3374 = fsub float %_0.i.i5963, %612, !dbg !8091
  %_0.i3399 = fsub float %_0.i.i6235, %_0.i2795, !dbg !8093
  %_3.i2253 = fcmp ule float %_0.i3399, 3.000000e+00, !dbg !8098
  %_0.i2623 = fadd float %_0.i3399, 3.000000e+00, !dbg !8100
  %_0.i3171 = fmul float %_0.i2623, %_0.i2623, !dbg !8102
  %_0.i3170 = fmul float %_0.i3171, 0x3FB5555560000000, !dbg !8104
  %_4.i4578.v.v = select i1 %_3.i2253, float %_0.i3170, float %_0.i3399, !dbg !8106
  %_4.i4578.v = fmul float %coefficients.i123.i.sroa.15.24.copyload, %_4.i4578.v.v, !dbg !8106
  %_4.i4578 = bitcast float %_4.i4578.v to i32, !dbg !8106
  %613 = fcmp ugt float %_0.i3399, -3.000000e+00, !dbg !8108
  %_7.i4570 = select i1 %613, i32 %_4.i4578, i32 0, !dbg !8110
  %_0.i4572 = bitcast i32 %_7.i4570 to float, !dbg !8111
  %_3.i.i5494 = fcmp ule float %_0.i4572, -1.000000e+02, !dbg !8113
  %614 = bitcast i32 %_7.i4570 to float, !dbg !8116
  %_0.i.i5501 = select i1 %_3.i.i5494, float -1.000000e+02, float %614, !dbg !8119
  %_3.i.i6188 = fcmp olt float %_0.i.i5501, 0.000000e+00, !dbg !8120
  %_0.i.i6195 = select i1 %_3.i.i6188, float %_0.i.i5501, float 0.000000e+00, !dbg !8123
  %615 = bitcast i32 %gain_far.i14.i109.i.sroa.0.010889 to float, !dbg !8125
  %_3.i2455 = fcmp uge float %_0.i.i6195, %615, !dbg !8126
  %_4.i4764.v = select i1 %_3.i2455, float %coefficients.i123.i.sroa.20.24.copyload, float %coefficients.i123.i.sroa.18.24.copyload, !dbg !8129
  %_0.i3454 = fsub float %615, %_0.i.i6195, !dbg !8131
  %_4.i2963 = fmul float %_0.i3454, %_4.i4764.v, !dbg !8133
  %_0.i2964 = fadd float %_0.i.i6195, %_4.i2963, !dbg !8133
  %616 = tail call noundef float @llvm.fabs.f32(float %_0.i2964), !dbg !8135
  %_4.i4325 = bitcast float %_0.i2964 to i32, !dbg !8138
  %617 = fcmp uge float %616, 0x3BC79CA100000000, !dbg !8141
  %_3.i4327 = select i1 %617, i32 %_4.i4325, i32 0, !dbg !8142
  %_0.i4328 = bitcast i32 %_3.i4327 to float, !dbg !8143
  %_0.i2800 = fadd float %_0.i2795.4, %_0.i4328, !dbg !8145
  %_0.i3295 = fmul float %_0.i2800, 0x3FC542A5A0000000, !dbg !8147
  %_3.i.i5162.inv = fcmp ogt float %_0.i3295, -1.260000e+02, !dbg !8150
  %_0.i.i5169 = select i1 %_3.i.i5162.inv, float %_0.i3295, float -1.260000e+02, !dbg !8150
  %_3.i.i5964.inv = fcmp olt float %_0.i.i5169, 1.270000e+02, !dbg !8154
  %_0.i.i5971 = select i1 %_3.i.i5964.inv, float %_0.i.i5169, float 1.270000e+02, !dbg !8154
  %618 = tail call noundef float @llvm.floor.f32(float %_0.i.i5971), !dbg !8157
  %_0.i3375 = fsub float %_0.i.i5971, %618, !dbg !8161
  %_0.i3400 = fsub float %_0.i.i6219, %_0.i2795.5, !dbg !8163
  %_3.i2255 = fcmp ule float %_0.i3400, 3.000000e+00, !dbg !8168
  %_0.i2624 = fadd float %_0.i3400, 3.000000e+00, !dbg !8170
  %_0.i3175 = fmul float %_0.i2624, %_0.i2624, !dbg !8172
  %_0.i3174 = fmul float %_0.i3175, 0x3FB5555560000000, !dbg !8174
  %_4.i4591.v.v = select i1 %_3.i2255, float %_0.i3174, float %_0.i3400, !dbg !8176
  %_4.i4591.v = fmul float %coefficients.i123.i.sroa.22.24.copyload, %_4.i4591.v.v, !dbg !8176
  %_4.i4591 = bitcast float %_4.i4591.v to i32, !dbg !8176
  %619 = fcmp ugt float %_0.i3400, -3.000000e+00, !dbg !8178
  %_7.i4583 = select i1 %619, i32 %_4.i4591, i32 0, !dbg !8180
  %_0.i4585 = bitcast i32 %_7.i4583 to float, !dbg !8181
  %_3.i.i5470 = fcmp ule float %_0.i4585, -1.000000e+02, !dbg !8183
  %620 = bitcast i32 %_7.i4583 to float, !dbg !8186
  %_0.i.i5477 = select i1 %_3.i.i5470, float -1.000000e+02, float %620, !dbg !8189
  %_3.i.i6172 = fcmp olt float %_0.i.i5477, 0.000000e+00, !dbg !8190
  %_0.i.i6179 = select i1 %_3.i.i6172, float %_0.i.i5477, float 0.000000e+00, !dbg !8193
  %621 = bitcast i32 %gain_far.i14.i109.i.sroa.6.010890 to float, !dbg !8195
  %_3.i2451 = fcmp uge float %_0.i.i6179, %621, !dbg !8196
  %_4.i4757.v = select i1 %_3.i2451, float %coefficients.i123.i.sroa.26.24.copyload, float %coefficients.i123.i.sroa.24.24.copyload, !dbg !8199
  %_0.i3453 = fsub float %621, %_0.i.i6179, !dbg !8201
  %_4.i2961 = fmul float %_0.i3453, %_4.i4757.v, !dbg !8203
  %_0.i2962 = fadd float %_0.i.i6179, %_4.i2961, !dbg !8203
  %622 = tail call noundef float @llvm.fabs.f32(float %_0.i2962), !dbg !8205
  %_4.i4321 = bitcast float %_0.i2962 to i32, !dbg !8208
  %623 = fcmp uge float %622, 0x3BC79CA100000000, !dbg !8211
  %_3.i4323 = select i1 %623, i32 %_4.i4321, i32 0, !dbg !8212
  %_0.i4324 = bitcast i32 %_3.i4323 to float, !dbg !8213
  %_0.i2799 = fadd float %_0.i2795.9, %_0.i4324, !dbg !8215
  %_0.i3293 = fmul float %_0.i2799, 0x3FC542A5A0000000, !dbg !8217
  %_3.i.i5170.inv = fcmp ogt float %_0.i3293, -1.260000e+02, !dbg !8220
  %_0.i.i5177 = select i1 %_3.i.i5170.inv, float %_0.i3293, float -1.260000e+02, !dbg !8220
  %_3.i.i5972.inv = fcmp olt float %_0.i.i5177, 1.270000e+02, !dbg !8224
  %_0.i.i5979 = select i1 %_3.i.i5972.inv, float %_0.i.i5177, float 1.270000e+02, !dbg !8224
  %624 = tail call noundef float @llvm.floor.f32(float %_0.i.i5979), !dbg !8227
  %_0.i3376 = fsub float %_0.i.i5979, %624, !dbg !8231
  %_0.i3088 = fmul float %_0.i3376, 0x3F5E974FA0000000, !dbg !8233
  %_0.i2592 = fadd float %_0.i3088, 0x3F82778560000000, !dbg !8235
  %_0.i3088.1 = fmul float %_0.i3376, %_0.i2592, !dbg !8233
  %_0.i2592.1 = fadd float %_0.i3088.1, 0x3FAC91CE60000000, !dbg !8235
  %_0.i3088.2 = fmul float %_0.i3376, %_0.i2592.1, !dbg !8233
  %_0.i2592.2 = fadd float %_0.i3088.2, 0x3FCEBDB560000000, !dbg !8235
  %_0.i3088.3 = fmul float %_0.i3376, %_0.i2592.2, !dbg !8233
  %_0.i2592.3 = fadd float %_0.i3088.3, 0x3FE62E4BA0000000, !dbg !8235
  %_0.i3085 = fmul float %_0.i3375, 0x3F5E974FA0000000, !dbg !8237
  %_0.i2590 = fadd float %_0.i3085, 0x3F82778560000000, !dbg !8239
  %_0.i3085.1 = fmul float %_0.i3375, %_0.i2590, !dbg !8237
  %_0.i2590.1 = fadd float %_0.i3085.1, 0x3FAC91CE60000000, !dbg !8239
  %_0.i3085.2 = fmul float %_0.i3375, %_0.i2590.1, !dbg !8237
  %_0.i2590.2 = fadd float %_0.i3085.2, 0x3FCEBDB560000000, !dbg !8239
  %_0.i3085.3 = fmul float %_0.i3375, %_0.i2590.2, !dbg !8237
  %_0.i2590.3 = fadd float %_0.i3085.3, 0x3FE62E4BA0000000, !dbg !8239
  %_0.i3082 = fmul float %_0.i3374, 0x3F5E974FA0000000, !dbg !8241
  %_0.i2588 = fadd float %_0.i3082, 0x3F82778560000000, !dbg !8243
  %_0.i3082.1 = fmul float %_0.i3374, %_0.i2588, !dbg !8241
  %_0.i2588.1 = fadd float %_0.i3082.1, 0x3FAC91CE60000000, !dbg !8243
  %_0.i3082.2 = fmul float %_0.i3374, %_0.i2588.1, !dbg !8241
  %_0.i2588.2 = fadd float %_0.i3082.2, 0x3FCEBDB560000000, !dbg !8243
  %_0.i3082.3 = fmul float %_0.i3374, %_0.i2588.2, !dbg !8241
  %_0.i2588.3 = fadd float %_0.i3082.3, 0x3FE62E4BA0000000, !dbg !8243
  %_0.i3079 = fmul float %_0.i3373, 0x3F5E974FA0000000, !dbg !8245
  %_0.i2586 = fadd float %_0.i3079, 0x3F82778560000000, !dbg !8247
  %_0.i3079.1 = fmul float %_0.i3373, %_0.i2586, !dbg !8245
  %_0.i2586.1 = fadd float %_0.i3079.1, 0x3FAC91CE60000000, !dbg !8247
  %_0.i3079.2 = fmul float %_0.i3373, %_0.i2586.1, !dbg !8245
  %_0.i2586.2 = fadd float %_0.i3079.2, 0x3FCEBDB560000000, !dbg !8247
  %_0.i3079.3 = fmul float %_0.i3373, %_0.i2586.2, !dbg !8245
  %_0.i2586.3 = fadd float %_0.i3079.3, 0x3FE62E4BA0000000, !dbg !8247
  %_0.i3078 = fmul float %_0.i3373, %_0.i2586.3, !dbg !8249
  %_0.i2585 = fadd float %_0.i3078, 1.000000e+00, !dbg !8251
  %biased.i2178 = fadd float %604, 0x4160000FE0000000, !dbg !8253
  %_4.i2179 = bitcast float %biased.i2178 to i32, !dbg !8255
  %_3.i2180 = shl i32 %_4.i2179, 23, !dbg !8257
  %_0.i2181 = bitcast i32 %_3.i2180 to float, !dbg !8258
  %_0.i3077 = fmul float %_0.i2585, %_0.i2181, !dbg !8260
  %_0.i3081 = fmul float %_0.i3374, %_0.i2588.3, !dbg !8262
  %_0.i2587 = fadd float %_0.i3081, 1.000000e+00, !dbg !8264
  %biased.i2182 = fadd float %612, 0x4160000FE0000000, !dbg !8266
  %_4.i2183 = bitcast float %biased.i2182 to i32, !dbg !8268
  %_3.i2184 = shl i32 %_4.i2183, 23, !dbg !8270
  %_0.i2185 = bitcast i32 %_3.i2184 to float, !dbg !8271
  %_0.i3080 = fmul float %_0.i2587, %_0.i2185, !dbg !8273
  %_0.i3084 = fmul float %_0.i3375, %_0.i2590.3, !dbg !8275
  %_0.i2589 = fadd float %_0.i3084, 1.000000e+00, !dbg !8277
  %biased.i2186 = fadd float %618, 0x4160000FE0000000, !dbg !8279
  %_4.i2187 = bitcast float %biased.i2186 to i32, !dbg !8281
  %_3.i2188 = shl i32 %_4.i2187, 23, !dbg !8283
  %_0.i2189 = bitcast i32 %_3.i2188 to float, !dbg !8284
  %_0.i3083 = fmul float %_0.i2589, %_0.i2189, !dbg !8286
  %_0.i3087 = fmul float %_0.i3376, %_0.i2592.3, !dbg !8288
  %_0.i2591 = fadd float %_0.i3087, 1.000000e+00, !dbg !8290
  %biased.i2190 = fadd float %624, 0x4160000FE0000000, !dbg !8292
  %_4.i2191 = bitcast float %biased.i2190 to i32, !dbg !8294
  %_3.i2192 = shl i32 %_4.i2191, 23, !dbg !8296
  %_0.i2193 = bitcast i32 %_3.i2192 to float, !dbg !8297
  %_0.i3086 = fmul float %_0.i2591, %_0.i2193, !dbg !8299
  %_262.i110.i463.i = icmp ugt i64 %_37.sroa.0.0.i36.i373.i, %_311.1.i76.i413.i, !dbg !8301
  br i1 %_262.i110.i463.i, label %bb84.i147.i500.i, label %bb85.i111.i464.i, !dbg !8301, !prof !1664

panic1.i156.i507.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i433.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i168.i515.i, i64 noundef range(i64 0, 2305843009213693952) %_314.1.i87.i436.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !8306, !noalias !8308
  unreachable, !dbg !8306

bb82.i148.i541.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3985
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i360.i10891, i64 noundef %_310.1.i69.i406.i, i64 noundef %_310.1.i69.i406.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_007bf1cfdcf9f845db5ef166fc9a1790) #26, !dbg !8309, !noalias !7555
  unreachable, !dbg !8309

bb85.i111.i464.i:                                 ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i440.i.split.us
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8310), !dbg !8313
  %_3.not.i3558 = icmp eq i64 %_311.1.i76.i413.i, %_37.sroa.0.0.i36.i373.i, !dbg !8314
  br i1 %_3.not.i3558, label %panic.i3561, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3562, !dbg !8314

panic.i3561:                                      ; preds = %bb85.i111.i464.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8314, !noalias !8316
  unreachable, !dbg !8314

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3562: ; preds = %bb85.i111.i464.i
  %_269.i114.i467.i = getelementptr inbounds nuw float, ptr %_311.0.i75.i412.i, i64 %_37.sroa.0.0.i36.i373.i, !dbg !8317
  %_0.i3560 = load float, ptr %_269.i114.i467.i, align 4, !dbg !8314, !alias.scope !8310, !noalias !7555, !noundef !11
  %_0.i3292 = fmul float %_0.i3077, %_0.i3560, !dbg !8322
  %_270.i118.i471.i = icmp ugt i64 %_37.sroa.0.0.i36.i373.i, %_312.1.i80.i421.i, !dbg !8324
  br i1 %_270.i118.i471.i, label %bb86.i146.i499.i, label %bb87.i119.i472.i, !dbg !8324, !prof !1664

bb84.i147.i500.i:                                 ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i440.i.split.us
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i373.i, i64 noundef %_311.1.i76.i413.i, i64 noundef %_311.1.i76.i413.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a45b75cd2d07085fe69fe186ba115725) #26, !dbg !8328, !noalias !7555
  unreachable, !dbg !8328

bb87.i119.i472.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3562
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8329), !dbg !8332
  %_3.not.i3553 = icmp eq i64 %_312.1.i80.i421.i, %_37.sroa.0.0.i36.i373.i, !dbg !8333
  br i1 %_3.not.i3553, label %panic.i3556, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3557, !dbg !8333

panic.i3556:                                      ; preds = %bb87.i119.i472.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8333, !noalias !8335
  unreachable, !dbg !8333

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3557: ; preds = %bb87.i119.i472.i
  %_277.i122.i475.i = getelementptr inbounds nuw float, ptr %_312.0.i79.i420.i, i64 %_37.sroa.0.0.i36.i373.i, !dbg !8336
  %_0.i3555 = load float, ptr %_277.i122.i475.i, align 4, !dbg !8333, !alias.scope !8329, !noalias !7555, !noundef !11
  %_0.i3291 = fmul float %_0.i3080, %_0.i3555, !dbg !8341
  %_0.i2798 = fadd float %_0.i3292, %_0.i3291, !dbg !8343
  %_278.i127.i480.i = icmp ugt i64 %_37.sroa.0.0.i36.i373.i, %_313.1.i83.i428.i, !dbg !8345
  br i1 %_278.i127.i480.i, label %bb88.i145.i498.i, label %bb89.i128.i481.i, !dbg !8345, !prof !1664

bb86.i146.i499.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3562
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i373.i, i64 noundef %_312.1.i80.i421.i, i64 noundef %_312.1.i80.i421.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_36be93341d7083c8a492c530428430ea) #26, !dbg !8350, !noalias !7555
  unreachable, !dbg !8350

bb89.i128.i481.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3557
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8351), !dbg !8354
  %_3.not.i3548 = icmp eq i64 %_313.1.i83.i428.i, %_37.sroa.0.0.i36.i373.i, !dbg !8355
  br i1 %_3.not.i3548, label %panic.i3551, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3552, !dbg !8355

panic.i3551:                                      ; preds = %bb89.i128.i481.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8355, !noalias !8357
  unreachable, !dbg !8355

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3552: ; preds = %bb89.i128.i481.i
  %_285.i131.i484.i = getelementptr inbounds nuw float, ptr %_313.0.i82.i427.i, i64 %_37.sroa.0.0.i36.i373.i, !dbg !8358
  %_0.i3550 = load float, ptr %_285.i131.i484.i, align 4, !dbg !8355, !alias.scope !8351, !noalias !7555, !noundef !11
  %_0.i3290 = fmul float %_0.i3083, %_0.i3550, !dbg !8363
  %_286.i135.i488.i = icmp ugt i64 %_37.sroa.0.0.i36.i373.i, %_314.1.i87.i436.i, !dbg !8365
  br i1 %_286.i135.i488.i, label %bb90.i144.i497.i, label %bb91.i136.i489.i, !dbg !8365, !prof !1664

bb88.i145.i498.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3557
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i373.i, i64 noundef %_313.1.i83.i428.i, i64 noundef %_313.1.i83.i428.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a6f1b2a44d3e33eba5c708677237c81) #26, !dbg !8369, !noalias !7555
  unreachable, !dbg !8369

bb91.i136.i489.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3552
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8370), !dbg !8373
  %_3.not.i3543 = icmp eq i64 %_314.1.i87.i436.i, %_37.sroa.0.0.i36.i373.i, !dbg !8374
  br i1 %_3.not.i3543, label %panic.i3546, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973, !dbg !8374

panic.i3546:                                      ; preds = %bb91.i136.i489.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8374, !noalias !8376
  unreachable, !dbg !8374

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973: ; preds = %bb91.i136.i489.i
  %_293.i139.i492.i = getelementptr inbounds nuw float, ptr %_314.0.i86.i435.i, i64 %_37.sroa.0.0.i36.i373.i, !dbg !8377
  %_0.i3545 = load float, ptr %_293.i139.i492.i, align 4, !dbg !8374, !alias.scope !8370, !noalias !7555, !noundef !11
  %_0.i3289 = fmul float %_0.i3086, %_0.i3545, !dbg !8382
  %_0.i2797 = fadd float %_0.i3290, %_0.i3289, !dbg !8384
  store float %_0.i2798, ptr %_180.i38.i377.i, align 4, !dbg !8386, !alias.scope !8389, !noalias !7555
  store float %_0.i2797, ptr %_188.i43.i380.i, align 4, !dbg !8392, !alias.scope !8394, !noalias !7555
  %exitcond13932.not = icmp eq i64 %573, %plan.0.i134.i, !dbg !7509
  br i1 %exitcond13932.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_Kb1_EB2_.exit.i.i, label %bb54.i33.i366.i, !dbg !7519

bb90.i144.i497.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3552
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i373.i, i64 noundef %_314.1.i87.i436.i, i64 noundef %_314.1.i87.i436.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4aa2eaec3d1833a4a887fe1d76c05ca7) #26, !dbg !8397, !noalias !7555
  unreachable, !dbg !8397

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_Kb1_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973, %bb17.i346.i
  %segments.i126.i.sroa.0.0 = phi float [ %_12.le.i6647, %bb17.i346.i ], [ %_0.i2796, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.8.0 = phi float [ %_12.le.1.i6649, %bb17.i346.i ], [ %_0.i2796.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.12.0 = phi float [ %_12.le.2.i6651, %bb17.i346.i ], [ %_0.i2796.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.16.0 = phi float [ %_12.le.3.i6653, %bb17.i346.i ], [ %_0.i2796.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.20.0 = phi float [ %_12.le.4.i6655, %bb17.i346.i ], [ %_0.i2796.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.26.0 = phi float [ %_12.le.5.i6657, %bb17.i346.i ], [ %_0.i2796.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.32.0 = phi float [ %_12.le.6.i6659, %bb17.i346.i ], [ %_0.i2796.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.36.0 = phi float [ %_12.le.7.i6661, %bb17.i346.i ], [ %_0.i2796.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.40.0 = phi float [ %_12.le.8.i6663, %bb17.i346.i ], [ %_0.i2796.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.44.0 = phi float [ %_12.le.9.i6665, %bb17.i346.i ], [ %_0.i2796.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.70.0 = phi float [ %_12.le.i6685, %bb17.i346.i ], [ %_0.i2795, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.76.0 = phi float [ %_12.le.1.i6687, %bb17.i346.i ], [ %_0.i2795.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.80.0 = phi float [ %_12.le.2.i6689, %bb17.i346.i ], [ %_0.i2795.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.84.0 = phi float [ %_12.le.3.i6691, %bb17.i346.i ], [ %_0.i2795.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.88.0 = phi float [ %_12.le.4.i6693, %bb17.i346.i ], [ %_0.i2795.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.94.0 = phi float [ %_12.le.5.i6695, %bb17.i346.i ], [ %_0.i2795.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.100.0 = phi float [ %_12.le.6.i6697, %bb17.i346.i ], [ %_0.i2795.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.104.0 = phi float [ %_12.le.7.i6699, %bb17.i346.i ], [ %_0.i2795.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.108.0 = phi float [ %_12.le.8.i6701, %bb17.i346.i ], [ %_0.i2795.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %segments.i126.i.sroa.112.0 = phi float [ %_12.le.9.i6703, %bb17.i346.i ], [ %_0.i2795.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !7521
  %filter_near.i17.i112.i.sroa.0.0.lcssa = phi float [ %filter_near.i17.i112.i.sroa.0.0.copyload, %bb17.i346.i ], [ %_0.i4184, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8398
  %filter_near.i17.i112.i.sroa.7.0.lcssa = phi float [ %filter_near.i17.i112.i.sroa.7.0.copyload, %bb17.i346.i ], [ %_0.i4180, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8398
  %filter_near.i17.i112.i.sroa.11.0.lcssa = phi float [ %filter_near.i17.i112.i.sroa.11.0.copyload, %bb17.i346.i ], [ %_0.i4192, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8398
  %filter_near.i17.i112.i.sroa.14.0.lcssa = phi float [ %filter_near.i17.i112.i.sroa.14.0.copyload, %bb17.i346.i ], [ %_0.i4188, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8398
  %filter_far.i16.i111.i.sroa.0.0.lcssa = phi float [ %filter_far.i16.i111.i.sroa.0.0.copyload, %bb17.i346.i ], [ %_0.i4168, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8399
  %filter_far.i16.i111.i.sroa.7.0.lcssa = phi float [ %filter_far.i16.i111.i.sroa.7.0.copyload, %bb17.i346.i ], [ %_0.i4164, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8399
  %filter_far.i16.i111.i.sroa.11.0.lcssa = phi float [ %filter_far.i16.i111.i.sroa.11.0.copyload, %bb17.i346.i ], [ %_0.i4176, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8399
  %filter_far.i16.i111.i.sroa.14.0.lcssa = phi float [ %filter_far.i16.i111.i.sroa.14.0.copyload, %bb17.i346.i ], [ %_0.i4172, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8399
  %gain_near.i15.i110.i.sroa.0.0.lcssa = phi i32 [ %568, %bb17.i346.i ], [ %_3.i4335, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8400
  %gain_near.i15.i110.i.sroa.6.0.lcssa = phi i32 [ %569, %bb17.i346.i ], [ %_3.i4331, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8400
  %gain_far.i14.i109.i.sroa.0.0.lcssa = phi i32 [ %570, %bb17.i346.i ], [ %_3.i4327, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8401
  %gain_far.i14.i109.i.sroa.6.0.lcssa = phi i32 [ %571, %bb17.i346.i ], [ %_3.i4323, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8401
  %position.sroa.0.0.i28.i360.i.lcssa = phi i64 [ %572, %bb17.i346.i ], [ %_37.sroa.0.0.i36.i373.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit3973 ], !dbg !8402
  store float %filter_near.i17.i112.i.sroa.0.0.lcssa, ptr %474, align 8, !dbg !8403, !noalias !7555
  store float %filter_near.i17.i112.i.sroa.7.0.lcssa, ptr %filter_near.i.i119.i.sroa.7.0..sroa_idx, align 4, !dbg !8403, !noalias !7555
  store float %filter_near.i17.i112.i.sroa.11.0.lcssa, ptr %filter_near.i.i119.i.sroa.11.0..sroa_idx, align 8, !dbg !8403, !noalias !7555
  store float %filter_near.i17.i112.i.sroa.14.0.lcssa, ptr %filter_near.i.i119.i.sroa.14.0..sroa_idx, align 4, !dbg !8403, !noalias !7555
  store float %filter_far.i16.i111.i.sroa.0.0.lcssa, ptr %475, align 8, !dbg !8404, !noalias !7555
  store float %filter_far.i16.i111.i.sroa.7.0.lcssa, ptr %filter_far.i.i118.i.sroa.7.0..sroa_idx, align 4, !dbg !8404, !noalias !7555
  store float %filter_far.i16.i111.i.sroa.11.0.lcssa, ptr %filter_far.i.i118.i.sroa.11.0..sroa_idx, align 8, !dbg !8404, !noalias !7555
  store float %filter_far.i16.i111.i.sroa.14.0.lcssa, ptr %filter_far.i.i118.i.sroa.14.0..sroa_idx, align 4, !dbg !8404, !noalias !7555
  store i32 %gain_near.i15.i110.i.sroa.0.0.lcssa, ptr %476, align 8, !dbg !8405, !noalias !7555
  store i32 %gain_near.i15.i110.i.sroa.6.0.lcssa, ptr %.sroa_idx6990, align 4, !dbg !8405, !noalias !7555
  store i32 %gain_far.i14.i109.i.sroa.0.0.lcssa, ptr %477, align 8, !dbg !8406, !noalias !7555
  store i32 %gain_far.i14.i109.i.sroa.6.0.lcssa, ptr %.sroa_idx6995, align 4, !dbg !8406, !noalias !7555
  store i64 %position.sroa.0.0.i28.i360.i.lcssa, ptr %_51.i138.i, align 8, !dbg !8407, !alias.scope !7506, !noalias !7507
  %advanced.i365.i = trunc i64 %plan.0.i134.i to i32, !dbg !8408
  store float %segments.i126.i.sroa.0.0, ptr %434, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.i6732 = load i32, ptr %490, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %625 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6732, i32 %advanced.i365.i), !dbg !8418
  store i32 %625, ptr %490, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.8.0, ptr %436, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.1.i6735 = load i32, ptr %491, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %626 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6735, i32 %advanced.i365.i), !dbg !8418
  store i32 %626, ptr %491, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.12.0, ptr %438, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.2.i6738 = load i32, ptr %492, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %627 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6738, i32 %advanced.i365.i), !dbg !8418
  store i32 %627, ptr %492, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.16.0, ptr %440, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.3.i6741 = load i32, ptr %493, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %628 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6741, i32 %advanced.i365.i), !dbg !8418
  store i32 %628, ptr %493, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.20.0, ptr %442, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.4.i6744 = load i32, ptr %494, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %629 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6744, i32 %advanced.i365.i), !dbg !8418
  store i32 %629, ptr %494, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.26.0, ptr %444, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.5.i6747 = load i32, ptr %495, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %630 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6747, i32 %advanced.i365.i), !dbg !8418
  store i32 %630, ptr %495, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.32.0, ptr %446, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.6.i6750 = load i32, ptr %496, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %631 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6750, i32 %advanced.i365.i), !dbg !8418
  store i32 %631, ptr %496, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.36.0, ptr %448, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.7.i6753 = load i32, ptr %497, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %632 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6753, i32 %advanced.i365.i), !dbg !8418
  store i32 %632, ptr %497, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.40.0, ptr %450, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.8.i6756 = load i32, ptr %498, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %633 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6756, i32 %advanced.i365.i), !dbg !8418
  store i32 %633, ptr %498, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.44.0, ptr %452, align 4, !dbg !8409, !alias.scope !8412, !noalias !8415
  %_26.9.i6759 = load i32, ptr %499, align 4, !dbg !8417, !alias.scope !8412, !noalias !8415, !noundef !11
  %634 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6759, i32 %advanced.i365.i), !dbg !8418
  store i32 %634, ptr %499, align 4, !dbg !8420, !alias.scope !8412, !noalias !8415
  store float %segments.i126.i.sroa.70.0, ptr %454, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.i6761 = load i32, ptr %500, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %635 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6761, i32 %advanced.i365.i), !dbg !8429
  store i32 %635, ptr %500, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.76.0, ptr %456, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.1.i6764 = load i32, ptr %501, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %636 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6764, i32 %advanced.i365.i), !dbg !8429
  store i32 %636, ptr %501, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.80.0, ptr %458, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.2.i6767 = load i32, ptr %502, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %637 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6767, i32 %advanced.i365.i), !dbg !8429
  store i32 %637, ptr %502, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.84.0, ptr %460, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.3.i6770 = load i32, ptr %503, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %638 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6770, i32 %advanced.i365.i), !dbg !8429
  store i32 %638, ptr %503, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.88.0, ptr %462, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.4.i6773 = load i32, ptr %504, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %639 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6773, i32 %advanced.i365.i), !dbg !8429
  store i32 %639, ptr %504, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.94.0, ptr %464, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.5.i6776 = load i32, ptr %505, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %640 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6776, i32 %advanced.i365.i), !dbg !8429
  store i32 %640, ptr %505, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.100.0, ptr %466, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.6.i6779 = load i32, ptr %506, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %641 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6779, i32 %advanced.i365.i), !dbg !8429
  store i32 %641, ptr %506, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.104.0, ptr %468, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.7.i6782 = load i32, ptr %507, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %642 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6782, i32 %advanced.i365.i), !dbg !8429
  store i32 %642, ptr %507, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.108.0, ptr %470, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.8.i6785 = load i32, ptr %508, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %643 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6785, i32 %advanced.i365.i), !dbg !8429
  store i32 %643, ptr %508, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  store float %segments.i126.i.sroa.112.0, ptr %472, align 4, !dbg !8421, !alias.scope !8423, !noalias !8426
  %_26.9.i6788 = load i32, ptr %509, align 4, !dbg !8428, !alias.scope !8423, !noalias !8426, !noundef !11
  %644 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6788, i32 %advanced.i365.i), !dbg !8429
  store i32 %644, ptr %509, align 4, !dbg !8431, !alias.scope !8423, !noalias !8426
  br label %bb15.i162.i, !dbg !7472

bb2.i590.i.lr.ph:                                 ; preds = %bb4.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8432), !dbg !8435
  %645 = getelementptr inbounds nuw i8, ptr %self, i64 856, !dbg !8436
  %sample_rate.i585.i = load i32, ptr %645, align 8, !dbg !8436, !alias.scope !8439, !noalias !8440, !noundef !11
  %646 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !8443
  %ring_len.i586.i = load i64, ptr %646, align 8, !dbg !8443, !alias.scope !8439, !noalias !8440, !noundef !11
  %647 = getelementptr inbounds nuw i8, ptr %self, i64 120
  %648 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %649 = getelementptr inbounds nuw i8, ptr %self, i64 200
  %650 = getelementptr inbounds nuw i8, ptr %self, i64 208
  %651 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %652 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %653 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %654 = getelementptr inbounds nuw i8, ptr %self, i64 240
  %655 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %656 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %657 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %658 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %659 = getelementptr inbounds nuw i8, ptr %self, i64 280
  %660 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %661 = getelementptr inbounds nuw i8, ptr %self, i64 296
  %662 = getelementptr inbounds nuw i8, ptr %self, i64 304
  %663 = getelementptr inbounds nuw i8, ptr %self, i64 312
  %664 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %665 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %666 = getelementptr inbounds nuw i8, ptr %self, i64 336
  %667 = getelementptr inbounds nuw i8, ptr %self, i64 344
  %_18.i594.i = getelementptr inbounds nuw i8, ptr %self, i64 480
  %668 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %669 = getelementptr inbounds nuw i8, ptr %self, i64 560
  %670 = getelementptr inbounds nuw i8, ptr %self, i64 568
  %671 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %672 = getelementptr inbounds nuw i8, ptr %self, i64 584
  %673 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %674 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %675 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %676 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %677 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %678 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %679 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %680 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %681 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %682 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %683 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %684 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %685 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %686 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %687 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %coefficients.i581.i.sroa.5.0._20.i580.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i580.i, i64 4
  %coefficients.i581.i.sroa.7.0._20.i580.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i580.i, i64 8
  %coefficients.i581.i.sroa.9.0._20.i580.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i580.i, i64 12
  %coefficients.i581.i.sroa.11.0._20.i580.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i580.i, i64 16
  %coefficients.i581.i.sroa.13.0._20.i580.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i580.i, i64 20
  %coefficients.i581.i.sroa.18.24._22.i579.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i579.i, i64 4
  %coefficients.i581.i.sroa.20.24._22.i579.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i579.i, i64 8
  %coefficients.i581.i.sroa.22.24._22.i579.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i579.i, i64 12
  %coefficients.i581.i.sroa.24.24._22.i579.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i579.i, i64 16
  %coefficients.i581.i.sroa.26.24._22.i579.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i579.i, i64 20
  %_51.i596.i = getelementptr inbounds nuw i8, ptr %self, i64 848
  %688 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %filter_near.i.i577.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 172
  %filter_near.i.i577.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 176
  %filter_near.i.i577.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 180
  %689 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %filter_far.i.i576.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 532
  %filter_far.i.i576.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 536
  %filter_far.i.i576.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 540
  %690 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %.sroa_idx7026 = getelementptr inbounds nuw i8, ptr %self, i64 188
  %691 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %.sroa_idx7031 = getelementptr inbounds nuw i8, ptr %self, i64 548
  %_63.i.i631.i = getelementptr inbounds nuw i8, ptr %self, i64 152
  %692 = getelementptr inbounds nuw i8, ptr %self, i64 156
  %693 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %694 = getelementptr inbounds nuw i8, ptr %self, i64 164
  %_68.i.i633.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %695 = getelementptr inbounds nuw i8, ptr %self, i64 516
  %696 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %697 = getelementptr inbounds nuw i8, ptr %self, i64 524
  %698 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %699 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %700 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %701 = getelementptr inbounds nuw i8, ptr %self, i64 488
  %702 = getelementptr inbounds nuw i8, ptr %self, i64 496
  %703 = getelementptr inbounds nuw i8, ptr %self, i64 504
  %_85.i.i663.i = getelementptr inbounds nuw i8, ptr %self, i64 400
  %_93.i.i678.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %704 = getelementptr inbounds nuw i8, ptr %self, i64 204
  %705 = getelementptr inbounds nuw i8, ptr %self, i64 220
  %706 = getelementptr inbounds nuw i8, ptr %self, i64 236
  %707 = getelementptr inbounds nuw i8, ptr %self, i64 252
  %708 = getelementptr inbounds nuw i8, ptr %self, i64 268
  %709 = getelementptr inbounds nuw i8, ptr %self, i64 284
  %710 = getelementptr inbounds nuw i8, ptr %self, i64 300
  %711 = getelementptr inbounds nuw i8, ptr %self, i64 316
  %712 = getelementptr inbounds nuw i8, ptr %self, i64 332
  %713 = getelementptr inbounds nuw i8, ptr %self, i64 348
  %714 = getelementptr inbounds nuw i8, ptr %self, i64 564
  %715 = getelementptr inbounds nuw i8, ptr %self, i64 580
  %716 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %717 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %718 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %719 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %720 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %721 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %722 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %723 = getelementptr inbounds nuw i8, ptr %self, i64 708
  br label %bb2.i590.i, !dbg !8445

bb2.i590.i:                                       ; preds = %bb2.i590.i.lr.ph, %bb15.i620.i
  %position.sroa.0.0.i588.i10586 = phi i64 [ 0, %bb2.i590.i.lr.ph ], [ %_32.i800.i, %bb15.i620.i ]
  %_11.i591.i = sub nuw i64 %_19.1, %position.sroa.0.0.i588.i10586, !dbg !8448
; call <multiband_compressor::Instance<f32, 1>>::plan_segment
  %724 = tail call fastcc { i64, i1 } @_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_(ptr noalias noundef nonnull align 8 dereferenceable(768) %_5, i64 noundef %_11.i591.i) #25, !dbg !8449, !noalias !3842
  %plan.0.i592.i = extractvalue { i64, i1 } %724, 0, !dbg !8449
  %plan.1.i593.i = extractvalue { i64, i1 } %724, 1, !dbg !8449
  %_12.le.i6789 = load float, ptr %648, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.i6790 = load float, ptr %649, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.1.i6791 = load float, ptr %650, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.1.i6792 = load float, ptr %651, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.2.i6793 = load float, ptr %652, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.2.i6794 = load float, ptr %653, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.3.i6795 = load float, ptr %654, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.3.i6796 = load float, ptr %655, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.4.i6797 = load float, ptr %656, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.4.i6798 = load float, ptr %657, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.5.i6799 = load float, ptr %658, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.5.i6800 = load float, ptr %659, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.6.i6801 = load float, ptr %660, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.6.i6802 = load float, ptr %661, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.7.i6803 = load float, ptr %662, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.7.i6804 = load float, ptr %663, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.8.i6805 = load float, ptr %664, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.8.i6806 = load float, ptr %665, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.9.i6807 = load float, ptr %666, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_16.le.9.i6808 = load float, ptr %667, align 8, !alias.scope !8450, !noalias !8453, !noundef !11
  %_12.le.i6827 = load float, ptr %668, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.i6828 = load float, ptr %669, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.1.i6829 = load float, ptr %670, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.1.i6830 = load float, ptr %671, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.2.i6831 = load float, ptr %672, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.2.i6832 = load float, ptr %673, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.3.i6833 = load float, ptr %674, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.3.i6834 = load float, ptr %675, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.4.i6835 = load float, ptr %676, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.4.i6836 = load float, ptr %677, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.5.i6837 = load float, ptr %678, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.5.i6838 = load float, ptr %679, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.6.i6839 = load float, ptr %680, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.6.i6840 = load float, ptr %681, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.7.i6841 = load float, ptr %682, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.7.i6842 = load float, ptr %683, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.8.i6843 = load float, ptr %684, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.8.i6844 = load float, ptr %685, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_12.le.9.i6845 = load float, ptr %686, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  %_16.le.9.i6846 = load float, ptr %687, align 8, !alias.scope !8455, !noalias !8458, !noundef !11
  call void @llvm.lifetime.start.p0(ptr nonnull %_20.i580.i), !dbg !8460, !noalias !8464
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_20.i580.i, ptr noalias noundef align 8 dereferenceable(360) %647, i32 noundef %sample_rate.i585.i) #25, !dbg !8465, !noalias !3842
  call void @llvm.lifetime.start.p0(ptr nonnull %_22.i579.i), !dbg !8466, !noalias !8464
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_22.i579.i, ptr noalias noundef align 8 dereferenceable(360) %_18.i594.i, i32 noundef %sample_rate.i585.i) #25, !dbg !8467, !noalias !3842
  %coefficients.i581.i.sroa.0.0.copyload = load float, ptr %_20.i580.i, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.5.0.copyload = load float, ptr %coefficients.i581.i.sroa.5.0._20.i580.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.7.0.copyload = load float, ptr %coefficients.i581.i.sroa.7.0._20.i580.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.9.0.copyload = load float, ptr %coefficients.i581.i.sroa.9.0._20.i580.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.11.0.copyload = load float, ptr %coefficients.i581.i.sroa.11.0._20.i580.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.13.0.copyload = load float, ptr %coefficients.i581.i.sroa.13.0._20.i580.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.15.24.copyload = load float, ptr %_22.i579.i, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.18.24.copyload = load float, ptr %coefficients.i581.i.sroa.18.24._22.i579.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.20.24.copyload = load float, ptr %coefficients.i581.i.sroa.20.24._22.i579.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.22.24.copyload = load float, ptr %coefficients.i581.i.sroa.22.24._22.i579.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.24.24.copyload = load float, ptr %coefficients.i581.i.sroa.24.24._22.i579.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  %coefficients.i581.i.sroa.26.24.copyload = load float, ptr %coefficients.i581.i.sroa.26.24._22.i579.i.sroa_idx, align 4, !dbg !8468, !noalias !8464
  call void @llvm.lifetime.end.p0(ptr nonnull %_22.i579.i), !dbg !8469, !noalias !8464
  call void @llvm.lifetime.end.p0(ptr nonnull %_20.i580.i), !dbg !8469, !noalias !8464
  %_32.i800.i = add i64 %plan.0.i592.i, %position.sroa.0.0.i588.i10586, !dbg !8470
  %_72.i801.i = icmp ult i64 %_32.i800.i, %position.sroa.0.0.i588.i10586, !dbg !8472
  %_66.not.i802.i = icmp ugt i64 %_32.i800.i, %_19.1
  %or.cond11.i803.i = or i1 %_72.i801.i, %_66.not.i802.i, !dbg !8472
  br i1 %plan.1.i593.i, label %bb9.i798.i, label %bb13.i595.i, !dbg !8478

bb9.i798.i:                                       ; preds = %bb2.i590.i
  br i1 %or.cond11.i803.i, label %bb19.i1014.i, label %bb17.i804.i, !dbg !8479, !prof !3878

bb13.i595.i:                                      ; preds = %bb2.i590.i
  br i1 %or.cond11.i803.i, label %bb29.i797.i, label %bb27.i601.i, !dbg !8483, !prof !3878

bb27.i601.i:                                      ; preds = %bb13.i595.i
  %_95.i602.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i588.i10586, !dbg !8489
  %_105.i605.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i588.i10586, !dbg !8493
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8500), !dbg !8503
  %filter_near.i.i577.i.sroa.0.0.copyload = load float, ptr %688, align 8, !dbg !8504, !noalias !8510
  %filter_near.i.i577.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i577.i.sroa.7.0..sroa_idx, align 4, !dbg !8504, !noalias !8510
  %filter_near.i.i577.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i577.i.sroa.11.0..sroa_idx, align 8, !dbg !8504, !noalias !8510
  %filter_near.i.i577.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i577.i.sroa.14.0..sroa_idx, align 4, !dbg !8504, !noalias !8510
  %filter_far.i.i576.i.sroa.0.0.copyload = load float, ptr %689, align 8, !dbg !8515, !noalias !8510
  %filter_far.i.i576.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i576.i.sroa.7.0..sroa_idx, align 4, !dbg !8515, !noalias !8510
  %filter_far.i.i576.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i576.i.sroa.11.0..sroa_idx, align 8, !dbg !8515, !noalias !8510
  %filter_far.i.i576.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i576.i.sroa.14.0..sroa_idx, align 4, !dbg !8515, !noalias !8510
  %725 = load i32, ptr %690, align 8, !dbg !8517
  %726 = load i32, ptr %.sroa_idx7026, align 4, !dbg !8517
  %727 = load i32, ptr %691, align 8, !dbg !8519
  %728 = load i32, ptr %.sroa_idx7031, align 4, !dbg !8519
  %729 = load i64, ptr %_51.i596.i, align 8, !dbg !8521, !alias.scope !8523, !noalias !8524, !noundef !11
  %_164.i.i617.i10398.not = icmp eq i64 %plan.0.i592.i, 0, !dbg !8526
  br i1 %_164.i.i617.i10398.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_KBZ_EB2_.exit.i.i, label %bb54.i.i621.i, !dbg !8536

bb29.i797.i:                                      ; preds = %bb13.i595.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i588.i10586, i64 noundef %_32.i800.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a05c61cb172b1016332ce1d3ce81e461) #26, !dbg !8537, !noalias !3842
  unreachable, !dbg !8537

bb54.i.i621.i:                                    ; preds = %bb27.i601.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077
  %iter.sroa.0.0.i.i616.i10412 = phi i64 [ %730, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ 0, %bb27.i601.i ]
  %position.sroa.0.0.i.i615.i10411 = phi i64 [ %_37.sroa.0.0.i.i624.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %729, %bb27.i601.i ]
  %gain_far.i.i574.i.sroa.6.010410 = phi i32 [ %_3.i4371, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %728, %bb27.i601.i ]
  %gain_far.i.i574.i.sroa.0.010409 = phi i32 [ %_3.i4375, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %727, %bb27.i601.i ]
  %gain_near.i.i575.i.sroa.6.010408 = phi i32 [ %_3.i4379, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %726, %bb27.i601.i ]
  %gain_near.i.i575.i.sroa.0.010407 = phi i32 [ %_3.i4383, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %725, %bb27.i601.i ]
  %filter_far.i.i576.i.sroa.14.010406 = phi float [ %_0.i4268, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %filter_far.i.i576.i.sroa.14.0.copyload, %bb27.i601.i ]
  %filter_far.i.i576.i.sroa.11.010405 = phi float [ %_0.i4272, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %filter_far.i.i576.i.sroa.11.0.copyload, %bb27.i601.i ]
  %filter_far.i.i576.i.sroa.7.010404 = phi float [ %_0.i4260, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %filter_far.i.i576.i.sroa.7.0.copyload, %bb27.i601.i ]
  %filter_far.i.i576.i.sroa.0.010403 = phi float [ %_0.i4264, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %filter_far.i.i576.i.sroa.0.0.copyload, %bb27.i601.i ]
  %filter_near.i.i577.i.sroa.14.010402 = phi float [ %_0.i4284, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %filter_near.i.i577.i.sroa.14.0.copyload, %bb27.i601.i ]
  %filter_near.i.i577.i.sroa.11.010401 = phi float [ %_0.i4288, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %filter_near.i.i577.i.sroa.11.0.copyload, %bb27.i601.i ]
  %filter_near.i.i577.i.sroa.7.010400 = phi float [ %_0.i4276, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %filter_near.i.i577.i.sroa.7.0.copyload, %bb27.i601.i ]
  %filter_near.i.i577.i.sroa.0.010399 = phi float [ %_0.i4280, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], [ %filter_near.i.i577.i.sroa.0.0.copyload, %bb27.i601.i ]
  %730 = add nuw i64 %iter.sroa.0.0.i.i616.i10412, 1, !dbg !8538
  %_38.i.i622.i = add i64 %position.sroa.0.0.i.i615.i10411, 1, !dbg !8544
  %_172.not.i.i623.i = icmp ult i64 %_38.i.i622.i, %ring_len.i586.i, !dbg !8547
  %731 = select i1 %_172.not.i.i623.i, i64 0, i64 %ring_len.i586.i, !dbg !8547
  %_37.sroa.0.0.i.i624.i = sub nuw i64 %_38.i.i622.i, %731, !dbg !8547
  %_180.i.i626.i = getelementptr inbounds nuw float, ptr %_95.i602.i, i64 %iter.sroa.0.0.i.i616.i10412, !dbg !8550
  %_0.i3732 = load float, ptr %_180.i.i626.i, align 4, !dbg !8560, !alias.scope !8562, !noalias !8565, !noundef !11
  %_188.i.i629.i = getelementptr inbounds nuw float, ptr %_105.i605.i, i64 %iter.sroa.0.0.i.i616.i10412, !dbg !8566
  %_0.i3727 = load float, ptr %_188.i.i629.i, align 4, !dbg !8575, !alias.scope !8577, !noalias !8565, !noundef !11
  %_7.i132 = load float, ptr %_63.i.i631.i, align 4, !dbg !8580, !alias.scope !8583, !noalias !8586, !noundef !11
  %_8.i133 = load float, ptr %692, align 4, !dbg !8588, !alias.scope !8583, !noalias !8586, !noundef !11
  %_9.i134 = load float, ptr %693, align 4, !dbg !8589, !alias.scope !8583, !noalias !8586, !noundef !11
  %_0.i3431 = fsub float %_0.i3732, %filter_near.i.i577.i.sroa.7.010400, !dbg !8590
  %_0.i3262 = fmul float %_0.i3431, %_8.i133, !dbg !8593
  %_4.i2915 = fmul float %filter_near.i.i577.i.sroa.0.010399, %_7.i132, !dbg !8595
  %_0.i2916 = fadd float %_4.i2915, %_0.i3262, !dbg !8595
  %_0.i2774 = fadd float %filter_near.i.i577.i.sroa.0.010399, %_0.i2916, !dbg !8597
  %_0.i3261 = fmul float %filter_near.i.i577.i.sroa.0.010399, %_8.i133, !dbg !8599
  %_4.i2913 = fmul float %_0.i3431, %_9.i134, !dbg !8601
  %_0.i2914 = fadd float %_0.i3261, %_4.i2913, !dbg !8601
  %_0.i2773 = fadd float %filter_near.i.i577.i.sroa.7.010400, %_0.i2914, !dbg !8603
  %_0.i2772 = fadd float %_0.i2916, %_0.i2916, !dbg !8605
  %_0.i2771 = fadd float %filter_near.i.i577.i.sroa.0.010399, %_0.i2772, !dbg !8607
  %732 = tail call noundef float @llvm.fabs.f32(float %_0.i2771), !dbg !8609
  %733 = fcmp uge float %732, 0x3BC79CA100000000, !dbg !8612
  %_0.i4280 = select i1 %733, float %_0.i2771, float 0.000000e+00, !dbg !8614
  %_0.i2770 = fadd float %_0.i2914, %_0.i2914, !dbg !8615
  %_0.i2769 = fadd float %filter_near.i.i577.i.sroa.7.010400, %_0.i2770, !dbg !8617
  %734 = tail call noundef float @llvm.fabs.f32(float %_0.i2769), !dbg !8619
  %735 = fcmp uge float %734, 0x3BC79CA100000000, !dbg !8622
  %_0.i4276 = select i1 %735, float %_0.i2769, float 0.000000e+00, !dbg !8624
  %_12.i137 = load float, ptr %694, align 4, !dbg !8625, !alias.scope !8583, !noalias !8586, !noundef !11
  %_4.i2921 = fmul float %_12.i137, %_0.i2774, !dbg !8626
  %_0.i2922 = fadd float %_0.i3732, %_4.i2921, !dbg !8626
  %_0.i3432 = fsub float %_0.i2773, %filter_near.i.i577.i.sroa.14.010402, !dbg !8628
  %_0.i3264 = fmul float %_8.i133, %_0.i3432, !dbg !8631
  %_4.i2919 = fmul float %filter_near.i.i577.i.sroa.11.010401, %_7.i132, !dbg !8633
  %_0.i2920 = fadd float %_4.i2919, %_0.i3264, !dbg !8633
  %_0.i3263 = fmul float %filter_near.i.i577.i.sroa.11.010401, %_8.i133, !dbg !8635
  %_4.i2917 = fmul float %_9.i134, %_0.i3432, !dbg !8637
  %_0.i2918 = fadd float %_0.i3263, %_4.i2917, !dbg !8637
  %_0.i2779 = fadd float %filter_near.i.i577.i.sroa.14.010402, %_0.i2918, !dbg !8639
  %_0.i2778 = fadd float %_0.i2920, %_0.i2920, !dbg !8641
  %_0.i2777 = fadd float %filter_near.i.i577.i.sroa.11.010401, %_0.i2778, !dbg !8643
  %736 = tail call noundef float @llvm.fabs.f32(float %_0.i2777), !dbg !8645
  %737 = fcmp uge float %736, 0x3BC79CA100000000, !dbg !8648
  %_0.i4288 = select i1 %737, float %_0.i2777, float 0.000000e+00, !dbg !8650
  %_0.i2776 = fadd float %_0.i2918, %_0.i2918, !dbg !8651
  %_0.i2775 = fadd float %filter_near.i.i577.i.sroa.14.010402, %_0.i2776, !dbg !8653
  %738 = tail call noundef float @llvm.fabs.f32(float %_0.i2775), !dbg !8655
  %739 = fcmp uge float %738, 0x3BC79CA100000000, !dbg !8658
  %_0.i4284 = select i1 %739, float %_0.i2775, float 0.000000e+00, !dbg !8660
  %_0.i3433 = fsub float %_0.i2922, %_0.i2779, !dbg !8661
  %_7.i119 = load float, ptr %_68.i.i633.i, align 4, !dbg !8663, !alias.scope !8666, !noalias !8669, !noundef !11
  %_8.i120 = load float, ptr %695, align 4, !dbg !8671, !alias.scope !8666, !noalias !8669, !noundef !11
  %_9.i121 = load float, ptr %696, align 4, !dbg !8672, !alias.scope !8666, !noalias !8669, !noundef !11
  %_0.i3429 = fsub float %_0.i3727, %filter_far.i.i576.i.sroa.7.010404, !dbg !8673
  %_0.i3258 = fmul float %_0.i3429, %_8.i120, !dbg !8676
  %_4.i2907 = fmul float %filter_far.i.i576.i.sroa.0.010403, %_7.i119, !dbg !8678
  %_0.i2908 = fadd float %_4.i2907, %_0.i3258, !dbg !8678
  %_0.i2762 = fadd float %filter_far.i.i576.i.sroa.0.010403, %_0.i2908, !dbg !8680
  %_0.i3257 = fmul float %filter_far.i.i576.i.sroa.0.010403, %_8.i120, !dbg !8682
  %_4.i2905 = fmul float %_0.i3429, %_9.i121, !dbg !8684
  %_0.i2906 = fadd float %_0.i3257, %_4.i2905, !dbg !8684
  %_0.i2761 = fadd float %filter_far.i.i576.i.sroa.7.010404, %_0.i2906, !dbg !8686
  %_0.i2760 = fadd float %_0.i2908, %_0.i2908, !dbg !8688
  %_0.i2759 = fadd float %filter_far.i.i576.i.sroa.0.010403, %_0.i2760, !dbg !8690
  %740 = tail call noundef float @llvm.fabs.f32(float %_0.i2759), !dbg !8692
  %741 = fcmp uge float %740, 0x3BC79CA100000000, !dbg !8695
  %_0.i4264 = select i1 %741, float %_0.i2759, float 0.000000e+00, !dbg !8697
  %_0.i2758 = fadd float %_0.i2906, %_0.i2906, !dbg !8698
  %_0.i2757 = fadd float %filter_far.i.i576.i.sroa.7.010404, %_0.i2758, !dbg !8700
  %742 = tail call noundef float @llvm.fabs.f32(float %_0.i2757), !dbg !8702
  %743 = fcmp uge float %742, 0x3BC79CA100000000, !dbg !8705
  %_0.i4260 = select i1 %743, float %_0.i2757, float 0.000000e+00, !dbg !8707
  %_12.i124 = load float, ptr %697, align 4, !dbg !8708, !alias.scope !8666, !noalias !8669, !noundef !11
  %_4.i2923 = fmul float %_12.i124, %_0.i2762, !dbg !8709
  %_0.i2924 = fadd float %_0.i3727, %_4.i2923, !dbg !8709
  %_0.i3430 = fsub float %_0.i2761, %filter_far.i.i576.i.sroa.14.010406, !dbg !8711
  %_0.i3260 = fmul float %_8.i120, %_0.i3430, !dbg !8714
  %_4.i2911 = fmul float %filter_far.i.i576.i.sroa.11.010405, %_7.i119, !dbg !8716
  %_0.i2912 = fadd float %_4.i2911, %_0.i3260, !dbg !8716
  %_0.i3259 = fmul float %filter_far.i.i576.i.sroa.11.010405, %_8.i120, !dbg !8718
  %_4.i2909 = fmul float %_9.i121, %_0.i3430, !dbg !8720
  %_0.i2910 = fadd float %_0.i3259, %_4.i2909, !dbg !8720
  %_0.i2767 = fadd float %filter_far.i.i576.i.sroa.14.010406, %_0.i2910, !dbg !8722
  %_0.i2766 = fadd float %_0.i2912, %_0.i2912, !dbg !8724
  %_0.i2765 = fadd float %filter_far.i.i576.i.sroa.11.010405, %_0.i2766, !dbg !8726
  %744 = tail call noundef float @llvm.fabs.f32(float %_0.i2765), !dbg !8728
  %745 = fcmp uge float %744, 0x3BC79CA100000000, !dbg !8731
  %_0.i4272 = select i1 %745, float %_0.i2765, float 0.000000e+00, !dbg !8733
  %_0.i2764 = fadd float %_0.i2910, %_0.i2910, !dbg !8734
  %_0.i2763 = fadd float %filter_far.i.i576.i.sroa.14.010406, %_0.i2764, !dbg !8736
  %746 = tail call noundef float @llvm.fabs.f32(float %_0.i2763), !dbg !8738
  %747 = fcmp uge float %746, 0x3BC79CA100000000, !dbg !8741
  %_0.i4268 = select i1 %747, float %_0.i2763, float 0.000000e+00, !dbg !8743
  %_0.i3434 = fsub float %_0.i2924, %_0.i2767, !dbg !8744
  %_307.1.i.i636.i = load i64, ptr %698, align 8, !dbg !8746, !noalias !8565, !noundef !11
  %_230.i.i637.i = icmp ugt i64 %position.sroa.0.0.i.i615.i10411, %_307.1.i.i636.i, !dbg !8748
  br i1 %_230.i.i637.i, label %bb76.i.i793.i, label %bb77.i.i638.i, !dbg !8748, !prof !1664

bb77.i.i638.i:                                    ; preds = %bb54.i.i621.i
  %_307.0.i.i639.i = load ptr, ptr %647, align 8, !dbg !8746, !noalias !8565, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8754), !dbg !8757
  %_4.not.i4094 = icmp eq i64 %_307.1.i.i636.i, %position.sroa.0.0.i.i615.i10411, !dbg !8758
  br i1 %_4.not.i4094, label %panic.i4096, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4097, !dbg !8758

panic.i4096:                                      ; preds = %bb77.i.i638.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !8758, !noalias !8760
  unreachable, !dbg !8758

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4097: ; preds = %bb77.i.i638.i
  %_237.i.i642.i = getelementptr inbounds nuw float, ptr %_307.0.i.i639.i, i64 %position.sroa.0.0.i.i615.i10411, !dbg !8761
  store float %_0.i2779, ptr %_237.i.i642.i, align 4, !dbg !8758, !alias.scope !8754, !noalias !8565
  %_308.1.i.i643.i = load i64, ptr %700, align 8, !dbg !8767, !noalias !8565, !noundef !11
  %_238.i.i644.i = icmp ugt i64 %position.sroa.0.0.i.i615.i10411, %_308.1.i.i643.i, !dbg !8768
  br i1 %_238.i.i644.i, label %bb78.i.i792.i, label %bb79.i.i645.i, !dbg !8768, !prof !1664

bb76.i.i793.i:                                    ; preds = %bb54.i.i621.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i615.i10411, i64 noundef %_307.1.i.i636.i, i64 noundef %_307.1.i.i636.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7aeb6b0a3ba8e73c50a5abef6c30558) #26, !dbg !8772, !noalias !8565
  unreachable, !dbg !8772

bb79.i.i645.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4097
  %_308.0.i.i646.i = load ptr, ptr %699, align 8, !dbg !8767, !noalias !8565, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8773), !dbg !8776
  %_4.not.i4090 = icmp eq i64 %_308.1.i.i643.i, %position.sroa.0.0.i.i615.i10411, !dbg !8777
  br i1 %_4.not.i4090, label %panic.i4092, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093, !dbg !8777

panic.i4092:                                      ; preds = %bb79.i.i645.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !8777, !noalias !8779
  unreachable, !dbg !8777

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093: ; preds = %bb79.i.i645.i
  %_245.i.i648.i = getelementptr inbounds nuw float, ptr %_308.0.i.i646.i, i64 %position.sroa.0.0.i.i615.i10411, !dbg !8780
  store float %_0.i3433, ptr %_245.i.i648.i, align 4, !dbg !8777, !alias.scope !8773, !noalias !8565
  %_309.1.i.i649.i = load i64, ptr %701, align 8, !dbg !8785, !noalias !8565, !noundef !11
  %_246.i.i650.i = icmp ugt i64 %position.sroa.0.0.i.i615.i10411, %_309.1.i.i649.i, !dbg !8786
  br i1 %_246.i.i650.i, label %bb80.i.i791.i, label %bb81.i.i651.i, !dbg !8786, !prof !1664

bb78.i.i792.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4097
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i615.i10411, i64 noundef %_308.1.i.i643.i, i64 noundef %_308.1.i.i643.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8e669d1bed0fb747f8a7bff920a6571) #26, !dbg !8790, !noalias !8565
  unreachable, !dbg !8790

bb81.i.i651.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093
  %_309.0.i.i652.i = load ptr, ptr %_18.i594.i, align 8, !dbg !8785, !noalias !8565, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8791), !dbg !8794
  %_4.not.i4086 = icmp eq i64 %_309.1.i.i649.i, %position.sroa.0.0.i.i615.i10411, !dbg !8795
  br i1 %_4.not.i4086, label %panic.i4088, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4089, !dbg !8795

panic.i4088:                                      ; preds = %bb81.i.i651.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !8795, !noalias !8797
  unreachable, !dbg !8795

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4089: ; preds = %bb81.i.i651.i
  %_253.i.i654.i = getelementptr inbounds nuw float, ptr %_309.0.i.i652.i, i64 %position.sroa.0.0.i.i615.i10411, !dbg !8798
  store float %_0.i2767, ptr %_253.i.i654.i, align 4, !dbg !8795, !alias.scope !8791, !noalias !8565
  %_310.1.i.i655.i = load i64, ptr %703, align 8, !dbg !8803, !noalias !8565, !noundef !11
  %_254.i.i656.i = icmp ugt i64 %position.sroa.0.0.i.i615.i10411, %_310.1.i.i655.i, !dbg !8804
  br i1 %_254.i.i656.i, label %bb82.i.i790.i, label %bb83.i.i657.i, !dbg !8804, !prof !1664

bb80.i.i791.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i615.i10411, i64 noundef %_309.1.i.i649.i, i64 noundef %_309.1.i.i649.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e81c2bc19b75441ce2fb90ce8f9eb70) #26, !dbg !8808, !noalias !8565
  unreachable, !dbg !8808

bb83.i.i657.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4089
  %_310.0.i.i658.i = load ptr, ptr %702, align 8, !dbg !8803, !noalias !8565, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8809), !dbg !8812
  %_4.not.i4082 = icmp eq i64 %_310.1.i.i655.i, %position.sroa.0.0.i.i615.i10411, !dbg !8813
  br i1 %_4.not.i4082, label %panic.i4084, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4085, !dbg !8813

panic.i4084:                                      ; preds = %bb83.i.i657.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !8813, !noalias !8815
  unreachable, !dbg !8813

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4085: ; preds = %bb83.i.i657.i
  %_261.i.i660.i = getelementptr inbounds nuw float, ptr %_310.0.i.i658.i, i64 %position.sroa.0.0.i.i615.i10411, !dbg !8816
  store float %_0.i3434, ptr %_261.i.i660.i, align 4, !dbg !8813, !alias.scope !8809, !noalias !8565
  %_311.0.i.i661.i = load ptr, ptr %647, align 8, !dbg !8821, !noalias !8565, !nonnull !11, !noundef !11
  %_311.1.i.i662.i = load i64, ptr %698, align 8, !dbg !8821, !noalias !8565, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8822), !dbg !8825
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8826), !dbg !8825
  %_13.i267.i781.i = load i64, ptr %_85.i.i663.i, align 8, !alias.scope !8826, !noalias !8828, !noundef !11
  %_12.i268.i782.i = add i64 %_13.i267.i781.i, %position.sroa.0.0.i.i615.i10411
  %_31.not.i269.i783.i = icmp ult i64 %_12.i268.i782.i, %ring_len.i586.i
  %748 = select i1 %_31.not.i269.i783.i, i64 0, i64 %ring_len.i586.i
  %_11.sroa.0.0.i270.i784.i = sub nuw i64 %_12.i268.i782.i, %748
  %_16.i271.i785.i = icmp ult i64 %_11.sroa.0.0.i270.i784.i, %_311.1.i.i662.i
  br i1 %_16.i271.i785.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i667.i.split.us, label %panic1.i272.i786.i

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i667.i.split.us: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4085
  %749 = getelementptr inbounds nuw float, ptr %_311.0.i.i661.i, i64 %_11.sroa.0.0.i270.i784.i
  %_8.i274.i788.i.us.le = load float, ptr %749, align 4, !alias.scope !8822, !noalias !8829, !noundef !11
  %_312.0.i.i669.i = load ptr, ptr %699, align 8, !dbg !8830, !noalias !8565, !nonnull !11, !noundef !11
  %_312.1.i.i670.i = load i64, ptr %700, align 8, !dbg !8830, !noalias !8565, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8832), !dbg !8835
  %_16.i254.i775.i = icmp ult i64 %_11.sroa.0.0.i270.i784.i, %_312.1.i.i670.i
  br i1 %_16.i254.i775.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i674.i.split.us, label %panic1.i255.i776.i

panic1.i272.i786.i:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4085
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i270.i784.i, i64 noundef range(i64 0, 2305843009213693952) %_311.1.i.i662.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !8836, !noalias !8838
  unreachable, !dbg !8836

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i674.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i667.i.split.us
  %750 = getelementptr inbounds nuw float, ptr %_312.0.i.i669.i, i64 %_11.sroa.0.0.i270.i784.i
  %_8.i257.i778.i.us.le = load float, ptr %750, align 4, !alias.scope !8832, !noalias !8839, !noundef !11
  %_313.0.i.i676.i = load ptr, ptr %_18.i594.i, align 8, !dbg !8841, !noalias !8565, !nonnull !11, !noundef !11
  %_313.1.i.i677.i = load i64, ptr %701, align 8, !dbg !8841, !noalias !8565, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8843), !dbg !8846
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8847), !dbg !8846
  %_13.i233.i761.i = load i64, ptr %_93.i.i678.i, align 8, !alias.scope !8847, !noalias !8849, !noundef !11
  %_12.i234.i762.i = add i64 %_13.i233.i761.i, %position.sroa.0.0.i.i615.i10411
  %_31.not.i235.i763.i = icmp ult i64 %_12.i234.i762.i, %ring_len.i586.i
  %751 = select i1 %_31.not.i235.i763.i, i64 0, i64 %ring_len.i586.i
  %_11.sroa.0.0.i236.i764.i = sub nuw i64 %_12.i234.i762.i, %751
  %_16.i237.i765.i = icmp ult i64 %_11.sroa.0.0.i236.i764.i, %_313.1.i.i677.i
  br i1 %_16.i237.i765.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i682.i.split.us, label %panic1.i238.i766.i

panic1.i255.i776.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit276.i667.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i270.i784.i, i64 noundef range(i64 0, 2305843009213693952) %_312.1.i.i670.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !8850, !noalias !8852
  unreachable, !dbg !8850

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i682.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i674.i.split.us
  %752 = getelementptr inbounds nuw float, ptr %_313.0.i.i676.i, i64 %_11.sroa.0.0.i236.i764.i
  %_8.i240.i768.i.us.le = load float, ptr %752, align 4, !alias.scope !8843, !noalias !8853, !noundef !11
  %_314.0.i.i684.i = load ptr, ptr %702, align 8, !dbg !8854, !noalias !8565, !nonnull !11, !noundef !11
  %_314.1.i.i685.i = load i64, ptr %703, align 8, !dbg !8854, !noalias !8565, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8856), !dbg !8859
  %_16.i220.i755.i = icmp ult i64 %_11.sroa.0.0.i236.i764.i, %_314.1.i.i685.i
  br i1 %_16.i220.i755.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i689.i.split.us, label %panic1.i221.i756.i

panic1.i238.i766.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit259.i674.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i236.i764.i, i64 noundef range(i64 0, 2305843009213693952) %_313.1.i.i677.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !8860, !noalias !8862
  unreachable, !dbg !8860

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i689.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i682.i.split.us
  %753 = getelementptr inbounds nuw float, ptr %_314.0.i.i684.i, i64 %_11.sroa.0.0.i236.i764.i
  %_8.i223.i758.i.us.le = load float, ptr %753, align 4, !alias.scope !8856, !noalias !8863, !noundef !11
  %754 = tail call noundef float @llvm.fabs.f32(float %_8.i274.i788.i.us.le), !dbg !8865
  %755 = tail call noundef float @llvm.fabs.f32(float %_8.i240.i768.i.us.le), !dbg !8870
  %_0.i3210 = fmul float %754, 5.000000e-01, !dbg !8872
  %_0.i3209 = fmul float %755, 5.000000e-01, !dbg !8876
  %_0.i2633 = fadd float %_0.i3210, %_0.i3209, !dbg !8878
  %_3.i.i5844 = fcmp ule float %_0.i2633, 0x3E45798EE0000000, !dbg !8880
  %_6.i.i5846 = bitcast float %_0.i2633 to i32, !dbg !8886
  %_4.i.i5850 = select i1 %_3.i.i5844, i32 841731191, i32 %_6.i.i5846, !dbg !8889
  %_0.i.i5851 = bitcast i32 %_4.i.i5850 to float, !dbg !8890
  %_3.i.i = fcmp ule float %_0.i.i5851, 0x3810000000000000, !dbg !8892
  %_4.i.i = select i1 %_3.i.i, i32 8388608, i32 %_4.i.i5850, !dbg !8897
  %_5.i3735 = and i32 %_4.i.i, 8388607, !dbg !8899
  %_4.i3736 = or disjoint i32 %_5.i3735, 1065353216, !dbg !8899
  %significand.i = bitcast i32 %_4.i3736 to float, !dbg !8901
  %_0.i3337 = fadd float %significand.i, -1.000000e+00, !dbg !8903
  %_0.i2994 = fmul float %_0.i3337, 0xBF9B17A960000000, !dbg !8905
  %_0.i2514 = fadd float %_0.i2994, 0x3FBF9A8440000000, !dbg !8907
  %_0.i2994.1 = fmul float %_0.i3337, %_0.i2514, !dbg !8905
  %_0.i2514.1 = fadd float %_0.i2994.1, 0xBFD1E3F400000000, !dbg !8907
  %_0.i2994.2 = fmul float %_0.i3337, %_0.i2514.1, !dbg !8905
  %_0.i2514.2 = fadd float %_0.i2994.2, 0x3FDD544F20000000, !dbg !8907
  %_0.i2994.3 = fmul float %_0.i3337, %_0.i2514.2, !dbg !8905
  %_0.i2514.3 = fadd float %_0.i2994.3, 0xBFE6FC2A60000000, !dbg !8907
  %_0.i2994.4 = fmul float %_0.i3337, %_0.i2514.3, !dbg !8905
  %_0.i2514.4 = fadd float %_0.i2994.4, 0x3FF714B2A0000000, !dbg !8907
  %_9.i3737 = lshr i32 %_4.i.i, 23, !dbg !8909
  %_8.i3738 = or disjoint i32 %_9.i3737, 1258291200, !dbg !8909
  %_7.i3739 = bitcast i32 %_8.i3738 to float, !dbg !8910
  %exponent.i = fadd float %_7.i3739, 0xC160000FE0000000, !dbg !8912
  %_0.i2993 = fmul float %_0.i3337, %_0.i2514.4, !dbg !8913
  %_0.i2513 = fadd float %exponent.i, %_0.i2993, !dbg !8915
  %_0.i3336 = fmul float %_0.i2513, 0x4018151820000000, !dbg !8917
  %_3.i.i5836.inv = fcmp ogt float %_0.i3336, -1.600000e+02, !dbg !8919
  %_0.i.i5843 = select i1 %_3.i.i5836.inv, float %_0.i3336, float -1.600000e+02, !dbg !8919
  %_3.i.i6420.inv = fcmp olt float %_0.i.i5843, 2.400000e+01, !dbg !8922
  %_0.i.i6427 = select i1 %_3.i.i6420.inv, float %_0.i.i5843, float 2.400000e+01, !dbg !8922
  %_0.i3385 = fsub float %_0.i.i6427, %_12.le.i6789, !dbg !8925
  %_3.i2226 = fcmp ule float %_0.i3385, 3.000000e+00, !dbg !8928
  %_0.i2609 = fadd float %_0.i3385, 3.000000e+00, !dbg !8930
  %_0.i3115 = fmul float %_0.i2609, %_0.i2609, !dbg !8932
  %_0.i3114 = fmul float %_0.i3115, 0x3FB5555560000000, !dbg !8934
  %_4.i4396.v.v = select i1 %_3.i2226, float %_0.i3114, float %_0.i3385, !dbg !8936
  %_4.i4396.v = fmul float %coefficients.i581.i.sroa.0.0.copyload, %_4.i4396.v.v, !dbg !8936
  %_4.i4396 = bitcast float %_4.i4396.v to i32, !dbg !8936
  %756 = fcmp ugt float %_0.i3385, -3.000000e+00, !dbg !8938
  %_7.i4388 = select i1 %756, i32 %_4.i4396, i32 0, !dbg !8940
  %_0.i4390 = bitcast i32 %_7.i4388 to float, !dbg !8941
  %_3.i.i5828 = fcmp ule float %_0.i4390, -1.000000e+02, !dbg !8943
  %757 = bitcast i32 %_7.i4388 to float, !dbg !8946
  %_0.i.i5835 = select i1 %_3.i.i5828, float -1.000000e+02, float %757, !dbg !8949
  %_3.i.i6412 = fcmp olt float %_0.i.i5835, 0.000000e+00, !dbg !8950
  %_0.i.i6419 = select i1 %_3.i.i6412, float %_0.i.i5835, float 0.000000e+00, !dbg !8953
  %758 = bitcast i32 %gain_near.i.i575.i.sroa.0.010407 to float, !dbg !8955
  %_3.i2511 = fcmp uge float %_0.i.i6419, %758, !dbg !8956
  %_4.i4862.v = select i1 %_3.i2511, float %coefficients.i581.i.sroa.7.0.copyload, float %coefficients.i581.i.sroa.5.0.copyload, !dbg !8959
  %_0.i3468 = fsub float %758, %_0.i.i6419, !dbg !8961
  %_4.i2991 = fmul float %_0.i3468, %_4.i4862.v, !dbg !8963
  %_0.i2992 = fadd float %_0.i.i6419, %_4.i2991, !dbg !8963
  %759 = tail call noundef float @llvm.fabs.f32(float %_0.i2992), !dbg !8965
  %_4.i4381 = bitcast float %_0.i2992 to i32, !dbg !8968
  %760 = fcmp uge float %759, 0x3BC79CA100000000, !dbg !8971
  %_3.i4383 = select i1 %760, i32 %_4.i4381, i32 0, !dbg !8972
  %_0.i4384 = bitcast i32 %_3.i4383 to float, !dbg !8973
  %_0.i2824 = fadd float %_12.le.4.i6797, %_0.i4384, !dbg !8975
  %_0.i3335 = fmul float %_0.i2824, 0x3FC542A5A0000000, !dbg !8977
  %_3.i.i5050.inv = fcmp ogt float %_0.i3335, -1.260000e+02, !dbg !8980
  %_0.i.i5057 = select i1 %_3.i.i5050.inv, float %_0.i3335, float -1.260000e+02, !dbg !8980
  %_3.i.i5852.inv = fcmp olt float %_0.i.i5057, 1.270000e+02, !dbg !8984
  %_0.i.i5859 = select i1 %_3.i.i5852.inv, float %_0.i.i5057, float 1.270000e+02, !dbg !8984
  %761 = tail call noundef float @llvm.floor.f32(float %_0.i.i5859), !dbg !8987
  %_0.i3361 = fsub float %_0.i.i5859, %761, !dbg !8991
  %762 = tail call noundef float @llvm.fabs.f32(float %_8.i257.i778.i.us.le), !dbg !8993
  %763 = tail call noundef float @llvm.fabs.f32(float %_8.i223.i758.i.us.le), !dbg !8996
  %_0.i3212 = fmul float %762, 5.000000e-01, !dbg !8998
  %_0.i3211 = fmul float %763, 5.000000e-01, !dbg !9000
  %_0.i2634 = fadd float %_0.i3212, %_0.i3211, !dbg !9002
  %_3.i.i5820 = fcmp ule float %_0.i2634, 0x3E45798EE0000000, !dbg !9004
  %_6.i.i5822 = bitcast float %_0.i2634 to i32, !dbg !9009
  %_4.i.i5826 = select i1 %_3.i.i5820, i32 841731191, i32 %_6.i.i5822, !dbg !9012
  %_0.i.i5827 = bitcast i32 %_4.i.i5826 to float, !dbg !9013
  %_3.i.i4866 = fcmp ule float %_0.i.i5827, 0x3810000000000000, !dbg !9015
  %_4.i.i4872 = select i1 %_3.i.i4866, i32 8388608, i32 %_4.i.i5826, !dbg !9020
  %_5.i3741 = and i32 %_4.i.i4872, 8388607, !dbg !9022
  %_4.i3742 = or disjoint i32 %_5.i3741, 1065353216, !dbg !9022
  %significand.i3743 = bitcast i32 %_4.i3742 to float, !dbg !9024
  %_0.i3338 = fadd float %significand.i3743, -1.000000e+00, !dbg !9026
  %_0.i2996 = fmul float %_0.i3338, 0xBF9B17A960000000, !dbg !9028
  %_0.i2516 = fadd float %_0.i2996, 0x3FBF9A8440000000, !dbg !9030
  %_0.i2996.1 = fmul float %_0.i3338, %_0.i2516, !dbg !9028
  %_0.i2516.1 = fadd float %_0.i2996.1, 0xBFD1E3F400000000, !dbg !9030
  %_0.i2996.2 = fmul float %_0.i3338, %_0.i2516.1, !dbg !9028
  %_0.i2516.2 = fadd float %_0.i2996.2, 0x3FDD544F20000000, !dbg !9030
  %_0.i2996.3 = fmul float %_0.i3338, %_0.i2516.2, !dbg !9028
  %_0.i2516.3 = fadd float %_0.i2996.3, 0xBFE6FC2A60000000, !dbg !9030
  %_0.i2996.4 = fmul float %_0.i3338, %_0.i2516.3, !dbg !9028
  %_0.i2516.4 = fadd float %_0.i2996.4, 0x3FF714B2A0000000, !dbg !9030
  %_9.i3744 = lshr i32 %_4.i.i4872, 23, !dbg !9032
  %_8.i3745 = or disjoint i32 %_9.i3744, 1258291200, !dbg !9032
  %_7.i3746 = bitcast i32 %_8.i3745 to float, !dbg !9033
  %exponent.i3747 = fadd float %_7.i3746, 0xC160000FE0000000, !dbg !9035
  %_0.i2995 = fmul float %_0.i3338, %_0.i2516.4, !dbg !9036
  %_0.i2515 = fadd float %exponent.i3747, %_0.i2995, !dbg !9038
  %_0.i3334 = fmul float %_0.i2515, 0x4018151820000000, !dbg !9040
  %_3.i.i5812.inv = fcmp ogt float %_0.i3334, -1.600000e+02, !dbg !9042
  %_0.i.i5819 = select i1 %_3.i.i5812.inv, float %_0.i3334, float -1.600000e+02, !dbg !9042
  %_3.i.i6404.inv = fcmp olt float %_0.i.i5819, 2.400000e+01, !dbg !9045
  %_0.i.i6411 = select i1 %_3.i.i6404.inv, float %_0.i.i5819, float 2.400000e+01, !dbg !9045
  %_0.i3386 = fsub float %_0.i.i6411, %_12.le.5.i6799, !dbg !9048
  %_3.i2227 = fcmp ule float %_0.i3386, 3.000000e+00, !dbg !9051
  %_0.i2610 = fadd float %_0.i3386, 3.000000e+00, !dbg !9053
  %_0.i3119 = fmul float %_0.i2610, %_0.i2610, !dbg !9055
  %_0.i3118 = fmul float %_0.i3119, 0x3FB5555560000000, !dbg !9057
  %_4.i4409.v.v = select i1 %_3.i2227, float %_0.i3118, float %_0.i3386, !dbg !9059
  %_4.i4409.v = fmul float %coefficients.i581.i.sroa.9.0.copyload, %_4.i4409.v.v, !dbg !9059
  %_4.i4409 = bitcast float %_4.i4409.v to i32, !dbg !9059
  %764 = fcmp ugt float %_0.i3386, -3.000000e+00, !dbg !9061
  %_7.i4401 = select i1 %764, i32 %_4.i4409, i32 0, !dbg !9063
  %_0.i4403 = bitcast i32 %_7.i4401 to float, !dbg !9064
  %_3.i.i5804 = fcmp ule float %_0.i4403, -1.000000e+02, !dbg !9066
  %765 = bitcast i32 %_7.i4401 to float, !dbg !9069
  %_0.i.i5811 = select i1 %_3.i.i5804, float -1.000000e+02, float %765, !dbg !9072
  %_3.i.i6396 = fcmp olt float %_0.i.i5811, 0.000000e+00, !dbg !9073
  %_0.i.i6403 = select i1 %_3.i.i6396, float %_0.i.i5811, float 0.000000e+00, !dbg !9076
  %766 = bitcast i32 %gain_near.i.i575.i.sroa.6.010408 to float, !dbg !9078
  %_3.i2507 = fcmp uge float %_0.i.i6403, %766, !dbg !9079
  %_4.i4855.v = select i1 %_3.i2507, float %coefficients.i581.i.sroa.13.0.copyload, float %coefficients.i581.i.sroa.11.0.copyload, !dbg !9082
  %_0.i3467 = fsub float %766, %_0.i.i6403, !dbg !9084
  %_4.i2989 = fmul float %_0.i3467, %_4.i4855.v, !dbg !9086
  %_0.i2990 = fadd float %_0.i.i6403, %_4.i2989, !dbg !9086
  %767 = tail call noundef float @llvm.fabs.f32(float %_0.i2990), !dbg !9088
  %_4.i4377 = bitcast float %_0.i2990 to i32, !dbg !9091
  %768 = fcmp uge float %767, 0x3BC79CA100000000, !dbg !9094
  %_3.i4379 = select i1 %768, i32 %_4.i4377, i32 0, !dbg !9095
  %_0.i4380 = bitcast i32 %_3.i4379 to float, !dbg !9096
  %_0.i2823 = fadd float %_12.le.9.i6807, %_0.i4380, !dbg !9098
  %_0.i3333 = fmul float %_0.i2823, 0x3FC542A5A0000000, !dbg !9100
  %_3.i.i5058.inv = fcmp ogt float %_0.i3333, -1.260000e+02, !dbg !9103
  %_0.i.i5065 = select i1 %_3.i.i5058.inv, float %_0.i3333, float -1.260000e+02, !dbg !9103
  %_3.i.i5860.inv = fcmp olt float %_0.i.i5065, 1.270000e+02, !dbg !9107
  %_0.i.i5867 = select i1 %_3.i.i5860.inv, float %_0.i.i5065, float 1.270000e+02, !dbg !9107
  %769 = tail call noundef float @llvm.floor.f32(float %_0.i.i5867), !dbg !9110
  %_0.i3362 = fsub float %_0.i.i5867, %769, !dbg !9114
  %_0.i3387 = fsub float %_0.i.i6427, %_12.le.i6827, !dbg !9116
  %_3.i2229 = fcmp ule float %_0.i3387, 3.000000e+00, !dbg !9121
  %_0.i2611 = fadd float %_0.i3387, 3.000000e+00, !dbg !9123
  %_0.i3123 = fmul float %_0.i2611, %_0.i2611, !dbg !9125
  %_0.i3122 = fmul float %_0.i3123, 0x3FB5555560000000, !dbg !9127
  %_4.i4422.v.v = select i1 %_3.i2229, float %_0.i3122, float %_0.i3387, !dbg !9129
  %_4.i4422.v = fmul float %coefficients.i581.i.sroa.15.24.copyload, %_4.i4422.v.v, !dbg !9129
  %_4.i4422 = bitcast float %_4.i4422.v to i32, !dbg !9129
  %770 = fcmp ugt float %_0.i3387, -3.000000e+00, !dbg !9131
  %_7.i4414 = select i1 %770, i32 %_4.i4422, i32 0, !dbg !9133
  %_0.i4416 = bitcast i32 %_7.i4414 to float, !dbg !9134
  %_3.i.i5780 = fcmp ule float %_0.i4416, -1.000000e+02, !dbg !9136
  %771 = bitcast i32 %_7.i4414 to float, !dbg !9139
  %_0.i.i5787 = select i1 %_3.i.i5780, float -1.000000e+02, float %771, !dbg !9142
  %_3.i.i6380 = fcmp olt float %_0.i.i5787, 0.000000e+00, !dbg !9143
  %_0.i.i6387 = select i1 %_3.i.i6380, float %_0.i.i5787, float 0.000000e+00, !dbg !9146
  %772 = bitcast i32 %gain_far.i.i574.i.sroa.0.010409 to float, !dbg !9148
  %_3.i2503 = fcmp uge float %_0.i.i6387, %772, !dbg !9149
  %_4.i4848.v = select i1 %_3.i2503, float %coefficients.i581.i.sroa.20.24.copyload, float %coefficients.i581.i.sroa.18.24.copyload, !dbg !9152
  %_0.i3466 = fsub float %772, %_0.i.i6387, !dbg !9154
  %_4.i2987 = fmul float %_0.i3466, %_4.i4848.v, !dbg !9156
  %_0.i2988 = fadd float %_0.i.i6387, %_4.i2987, !dbg !9156
  %773 = tail call noundef float @llvm.fabs.f32(float %_0.i2988), !dbg !9158
  %_4.i4373 = bitcast float %_0.i2988 to i32, !dbg !9161
  %774 = fcmp uge float %773, 0x3BC79CA100000000, !dbg !9164
  %_3.i4375 = select i1 %774, i32 %_4.i4373, i32 0, !dbg !9165
  %_0.i4376 = bitcast i32 %_3.i4375 to float, !dbg !9166
  %_0.i2822 = fadd float %_12.le.4.i6835, %_0.i4376, !dbg !9168
  %_0.i3331 = fmul float %_0.i2822, 0x3FC542A5A0000000, !dbg !9170
  %_3.i.i5066.inv = fcmp ogt float %_0.i3331, -1.260000e+02, !dbg !9173
  %_0.i.i5073 = select i1 %_3.i.i5066.inv, float %_0.i3331, float -1.260000e+02, !dbg !9173
  %_3.i.i5868.inv = fcmp olt float %_0.i.i5073, 1.270000e+02, !dbg !9177
  %_0.i.i5875 = select i1 %_3.i.i5868.inv, float %_0.i.i5073, float 1.270000e+02, !dbg !9177
  %775 = tail call noundef float @llvm.floor.f32(float %_0.i.i5875), !dbg !9180
  %_0.i3363 = fsub float %_0.i.i5875, %775, !dbg !9184
  %_0.i3388 = fsub float %_0.i.i6411, %_12.le.5.i6837, !dbg !9186
  %_3.i2231 = fcmp ule float %_0.i3388, 3.000000e+00, !dbg !9191
  %_0.i2612 = fadd float %_0.i3388, 3.000000e+00, !dbg !9193
  %_0.i3127 = fmul float %_0.i2612, %_0.i2612, !dbg !9195
  %_0.i3126 = fmul float %_0.i3127, 0x3FB5555560000000, !dbg !9197
  %_4.i4435.v.v = select i1 %_3.i2231, float %_0.i3126, float %_0.i3388, !dbg !9199
  %_4.i4435.v = fmul float %coefficients.i581.i.sroa.22.24.copyload, %_4.i4435.v.v, !dbg !9199
  %_4.i4435 = bitcast float %_4.i4435.v to i32, !dbg !9199
  %776 = fcmp ugt float %_0.i3388, -3.000000e+00, !dbg !9201
  %_7.i4427 = select i1 %776, i32 %_4.i4435, i32 0, !dbg !9203
  %_0.i4429 = bitcast i32 %_7.i4427 to float, !dbg !9204
  %_3.i.i5756 = fcmp ule float %_0.i4429, -1.000000e+02, !dbg !9206
  %777 = bitcast i32 %_7.i4427 to float, !dbg !9209
  %_0.i.i5763 = select i1 %_3.i.i5756, float -1.000000e+02, float %777, !dbg !9212
  %_3.i.i6364 = fcmp olt float %_0.i.i5763, 0.000000e+00, !dbg !9213
  %_0.i.i6371 = select i1 %_3.i.i6364, float %_0.i.i5763, float 0.000000e+00, !dbg !9216
  %778 = bitcast i32 %gain_far.i.i574.i.sroa.6.010410 to float, !dbg !9218
  %_3.i2499 = fcmp uge float %_0.i.i6371, %778, !dbg !9219
  %_4.i4841.v = select i1 %_3.i2499, float %coefficients.i581.i.sroa.26.24.copyload, float %coefficients.i581.i.sroa.24.24.copyload, !dbg !9222
  %_0.i3465 = fsub float %778, %_0.i.i6371, !dbg !9224
  %_4.i2985 = fmul float %_0.i3465, %_4.i4841.v, !dbg !9226
  %_0.i2986 = fadd float %_0.i.i6371, %_4.i2985, !dbg !9226
  %779 = tail call noundef float @llvm.fabs.f32(float %_0.i2986), !dbg !9228
  %_4.i4369 = bitcast float %_0.i2986 to i32, !dbg !9231
  %780 = fcmp uge float %779, 0x3BC79CA100000000, !dbg !9234
  %_3.i4371 = select i1 %780, i32 %_4.i4369, i32 0, !dbg !9235
  %_0.i4372 = bitcast i32 %_3.i4371 to float, !dbg !9236
  %_0.i2821 = fadd float %_12.le.9.i6845, %_0.i4372, !dbg !9238
  %_0.i3329 = fmul float %_0.i2821, 0x3FC542A5A0000000, !dbg !9240
  %_3.i.i5074.inv = fcmp ogt float %_0.i3329, -1.260000e+02, !dbg !9243
  %_0.i.i5081 = select i1 %_3.i.i5074.inv, float %_0.i3329, float -1.260000e+02, !dbg !9243
  %_3.i.i5876.inv = fcmp olt float %_0.i.i5081, 1.270000e+02, !dbg !9247
  %_0.i.i5883 = select i1 %_3.i.i5876.inv, float %_0.i.i5081, float 1.270000e+02, !dbg !9247
  %781 = tail call noundef float @llvm.floor.f32(float %_0.i.i5883), !dbg !9250
  %_0.i3364 = fsub float %_0.i.i5883, %781, !dbg !9254
  %_0.i3052 = fmul float %_0.i3364, 0x3F5E974FA0000000, !dbg !9256
  %_0.i2568 = fadd float %_0.i3052, 0x3F82778560000000, !dbg !9258
  %_0.i3052.1 = fmul float %_0.i3364, %_0.i2568, !dbg !9256
  %_0.i2568.1 = fadd float %_0.i3052.1, 0x3FAC91CE60000000, !dbg !9258
  %_0.i3052.2 = fmul float %_0.i3364, %_0.i2568.1, !dbg !9256
  %_0.i2568.2 = fadd float %_0.i3052.2, 0x3FCEBDB560000000, !dbg !9258
  %_0.i3052.3 = fmul float %_0.i3364, %_0.i2568.2, !dbg !9256
  %_0.i2568.3 = fadd float %_0.i3052.3, 0x3FE62E4BA0000000, !dbg !9258
  %_0.i3049 = fmul float %_0.i3363, 0x3F5E974FA0000000, !dbg !9260
  %_0.i2566 = fadd float %_0.i3049, 0x3F82778560000000, !dbg !9262
  %_0.i3049.1 = fmul float %_0.i3363, %_0.i2566, !dbg !9260
  %_0.i2566.1 = fadd float %_0.i3049.1, 0x3FAC91CE60000000, !dbg !9262
  %_0.i3049.2 = fmul float %_0.i3363, %_0.i2566.1, !dbg !9260
  %_0.i2566.2 = fadd float %_0.i3049.2, 0x3FCEBDB560000000, !dbg !9262
  %_0.i3049.3 = fmul float %_0.i3363, %_0.i2566.2, !dbg !9260
  %_0.i2566.3 = fadd float %_0.i3049.3, 0x3FE62E4BA0000000, !dbg !9262
  %_0.i3046 = fmul float %_0.i3362, 0x3F5E974FA0000000, !dbg !9264
  %_0.i2564 = fadd float %_0.i3046, 0x3F82778560000000, !dbg !9266
  %_0.i3046.1 = fmul float %_0.i3362, %_0.i2564, !dbg !9264
  %_0.i2564.1 = fadd float %_0.i3046.1, 0x3FAC91CE60000000, !dbg !9266
  %_0.i3046.2 = fmul float %_0.i3362, %_0.i2564.1, !dbg !9264
  %_0.i2564.2 = fadd float %_0.i3046.2, 0x3FCEBDB560000000, !dbg !9266
  %_0.i3046.3 = fmul float %_0.i3362, %_0.i2564.2, !dbg !9264
  %_0.i2564.3 = fadd float %_0.i3046.3, 0x3FE62E4BA0000000, !dbg !9266
  %_0.i3043 = fmul float %_0.i3361, 0x3F5E974FA0000000, !dbg !9268
  %_0.i2562 = fadd float %_0.i3043, 0x3F82778560000000, !dbg !9270
  %_0.i3043.1 = fmul float %_0.i3361, %_0.i2562, !dbg !9268
  %_0.i2562.1 = fadd float %_0.i3043.1, 0x3FAC91CE60000000, !dbg !9270
  %_0.i3043.2 = fmul float %_0.i3361, %_0.i2562.1, !dbg !9268
  %_0.i2562.2 = fadd float %_0.i3043.2, 0x3FCEBDB560000000, !dbg !9270
  %_0.i3043.3 = fmul float %_0.i3361, %_0.i2562.2, !dbg !9268
  %_0.i2562.3 = fadd float %_0.i3043.3, 0x3FE62E4BA0000000, !dbg !9270
  %_0.i3042 = fmul float %_0.i3361, %_0.i2562.3, !dbg !9272
  %_0.i2561 = fadd float %_0.i3042, 1.000000e+00, !dbg !9274
  %biased.i = fadd float %761, 0x4160000FE0000000, !dbg !9276
  %_4.i2131 = bitcast float %biased.i to i32, !dbg !9278
  %_3.i2132 = shl i32 %_4.i2131, 23, !dbg !9280
  %_0.i2133 = bitcast i32 %_3.i2132 to float, !dbg !9281
  %_0.i3041 = fmul float %_0.i2561, %_0.i2133, !dbg !9283
  %_0.i3045 = fmul float %_0.i3362, %_0.i2564.3, !dbg !9285
  %_0.i2563 = fadd float %_0.i3045, 1.000000e+00, !dbg !9287
  %biased.i2134 = fadd float %769, 0x4160000FE0000000, !dbg !9289
  %_4.i2135 = bitcast float %biased.i2134 to i32, !dbg !9291
  %_3.i2136 = shl i32 %_4.i2135, 23, !dbg !9293
  %_0.i2137 = bitcast i32 %_3.i2136 to float, !dbg !9294
  %_0.i3044 = fmul float %_0.i2563, %_0.i2137, !dbg !9296
  %_0.i3048 = fmul float %_0.i3363, %_0.i2566.3, !dbg !9298
  %_0.i2565 = fadd float %_0.i3048, 1.000000e+00, !dbg !9300
  %biased.i2138 = fadd float %775, 0x4160000FE0000000, !dbg !9302
  %_4.i2139 = bitcast float %biased.i2138 to i32, !dbg !9304
  %_3.i2140 = shl i32 %_4.i2139, 23, !dbg !9306
  %_0.i2141 = bitcast i32 %_3.i2140 to float, !dbg !9307
  %_0.i3047 = fmul float %_0.i2565, %_0.i2141, !dbg !9309
  %_0.i3051 = fmul float %_0.i3364, %_0.i2568.3, !dbg !9311
  %_0.i2567 = fadd float %_0.i3051, 1.000000e+00, !dbg !9313
  %biased.i2142 = fadd float %781, 0x4160000FE0000000, !dbg !9315
  %_4.i2143 = bitcast float %biased.i2142 to i32, !dbg !9317
  %_3.i2144 = shl i32 %_4.i2143, 23, !dbg !9319
  %_0.i2145 = bitcast i32 %_3.i2144 to float, !dbg !9320
  %_0.i3050 = fmul float %_0.i2567, %_0.i2145, !dbg !9322
  %_262.i.i712.i = icmp ugt i64 %_37.sroa.0.0.i.i624.i, %_311.1.i.i662.i, !dbg !9324
  br i1 %_262.i.i712.i, label %bb84.i.i749.i, label %bb85.i.i713.i, !dbg !9324, !prof !1664

panic1.i221.i756.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit242.i682.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i236.i764.i, i64 noundef range(i64 0, 2305843009213693952) %_314.1.i.i685.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !9329, !noalias !9331
  unreachable, !dbg !9329

bb82.i.i790.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4089
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i615.i10411, i64 noundef %_310.1.i.i655.i, i64 noundef %_310.1.i.i655.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_007bf1cfdcf9f845db5ef166fc9a1790) #26, !dbg !9332, !noalias !8565
  unreachable, !dbg !9332

bb85.i.i713.i:                                    ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i689.i.split.us
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9333), !dbg !9336
  %_3.not.i3712 = icmp eq i64 %_311.1.i.i662.i, %_37.sroa.0.0.i.i624.i, !dbg !9337
  br i1 %_3.not.i3712, label %panic.i3715, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3716, !dbg !9337

panic.i3715:                                      ; preds = %bb85.i.i713.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9337, !noalias !9339
  unreachable, !dbg !9337

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3716: ; preds = %bb85.i.i713.i
  %_269.i.i716.i = getelementptr inbounds nuw float, ptr %_311.0.i.i661.i, i64 %_37.sroa.0.0.i.i624.i, !dbg !9340
  %_0.i3714 = load float, ptr %_269.i.i716.i, align 4, !dbg !9337, !alias.scope !9333, !noalias !8565, !noundef !11
  %_0.i3328 = fmul float %_0.i3041, %_0.i3714, !dbg !9345
  %_270.i.i720.i = icmp ugt i64 %_37.sroa.0.0.i.i624.i, %_312.1.i.i670.i, !dbg !9347
  br i1 %_270.i.i720.i, label %bb86.i.i748.i, label %bb87.i.i721.i, !dbg !9347, !prof !1664

bb84.i.i749.i:                                    ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit225.i689.i.split.us
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i624.i, i64 noundef %_311.1.i.i662.i, i64 noundef %_311.1.i.i662.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a45b75cd2d07085fe69fe186ba115725) #26, !dbg !9351, !noalias !8565
  unreachable, !dbg !9351

bb87.i.i721.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3716
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9352), !dbg !9355
  %_3.not.i3707 = icmp eq i64 %_312.1.i.i670.i, %_37.sroa.0.0.i.i624.i, !dbg !9356
  br i1 %_3.not.i3707, label %panic.i3710, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3711, !dbg !9356

panic.i3710:                                      ; preds = %bb87.i.i721.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9356, !noalias !9358
  unreachable, !dbg !9356

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3711: ; preds = %bb87.i.i721.i
  %_277.i.i724.i = getelementptr inbounds nuw float, ptr %_312.0.i.i669.i, i64 %_37.sroa.0.0.i.i624.i, !dbg !9359
  %_0.i3709 = load float, ptr %_277.i.i724.i, align 4, !dbg !9356, !alias.scope !9352, !noalias !8565, !noundef !11
  %_0.i3327 = fmul float %_0.i3044, %_0.i3709, !dbg !9364
  %_0.i2820 = fadd float %_0.i3328, %_0.i3327, !dbg !9366
  %_278.i.i729.i = icmp ugt i64 %_37.sroa.0.0.i.i624.i, %_313.1.i.i677.i, !dbg !9368
  br i1 %_278.i.i729.i, label %bb88.i.i747.i, label %bb89.i.i730.i, !dbg !9368, !prof !1664

bb86.i.i748.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3716
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i624.i, i64 noundef %_312.1.i.i670.i, i64 noundef %_312.1.i.i670.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_36be93341d7083c8a492c530428430ea) #26, !dbg !9373, !noalias !8565
  unreachable, !dbg !9373

bb89.i.i730.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3711
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9374), !dbg !9377
  %_3.not.i3702 = icmp eq i64 %_313.1.i.i677.i, %_37.sroa.0.0.i.i624.i, !dbg !9378
  br i1 %_3.not.i3702, label %panic.i3705, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3706, !dbg !9378

panic.i3705:                                      ; preds = %bb89.i.i730.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9378, !noalias !9380
  unreachable, !dbg !9378

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3706: ; preds = %bb89.i.i730.i
  %_285.i.i733.i = getelementptr inbounds nuw float, ptr %_313.0.i.i676.i, i64 %_37.sroa.0.0.i.i624.i, !dbg !9381
  %_0.i3704 = load float, ptr %_285.i.i733.i, align 4, !dbg !9378, !alias.scope !9374, !noalias !8565, !noundef !11
  %_0.i3326 = fmul float %_0.i3047, %_0.i3704, !dbg !9386
  %_286.i.i737.i = icmp ugt i64 %_37.sroa.0.0.i.i624.i, %_314.1.i.i685.i, !dbg !9388
  br i1 %_286.i.i737.i, label %bb90.i.i746.i, label %bb91.i.i738.i, !dbg !9388, !prof !1664

bb88.i.i747.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3711
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i624.i, i64 noundef %_313.1.i.i677.i, i64 noundef %_313.1.i.i677.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a6f1b2a44d3e33eba5c708677237c81) #26, !dbg !9392, !noalias !8565
  unreachable, !dbg !9392

bb91.i.i738.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3706
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9393), !dbg !9396
  %_3.not.i3697 = icmp eq i64 %_314.1.i.i685.i, %_37.sroa.0.0.i.i624.i, !dbg !9397
  br i1 %_3.not.i3697, label %panic.i3700, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077, !dbg !9397

panic.i3700:                                      ; preds = %bb91.i.i738.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9397, !noalias !9399
  unreachable, !dbg !9397

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077: ; preds = %bb91.i.i738.i
  %_293.i.i741.i = getelementptr inbounds nuw float, ptr %_314.0.i.i684.i, i64 %_37.sroa.0.0.i.i624.i, !dbg !9400
  %_0.i3699 = load float, ptr %_293.i.i741.i, align 4, !dbg !9397, !alias.scope !9393, !noalias !8565, !noundef !11
  %_0.i3325 = fmul float %_0.i3050, %_0.i3699, !dbg !9405
  %_0.i2819 = fadd float %_0.i3326, %_0.i3325, !dbg !9407
  store float %_0.i2820, ptr %_180.i.i626.i, align 4, !dbg !9409, !alias.scope !9412, !noalias !8565
  store float %_0.i2819, ptr %_188.i.i629.i, align 4, !dbg !9415, !alias.scope !9417, !noalias !8565
  %exitcond.not = icmp eq i64 %730, %plan.0.i592.i, !dbg !8526
  br i1 %exitcond.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_KBZ_EB2_.exit.i.i, label %bb54.i.i621.i, !dbg !8536

bb90.i.i746.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3706
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i.i624.i, i64 noundef %_314.1.i.i685.i, i64 noundef %_314.1.i.i685.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4aa2eaec3d1833a4a887fe1d76c05ca7) #26, !dbg !9420, !noalias !8565
  unreachable, !dbg !9420

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_KBZ_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077, %bb27.i601.i
  %filter_near.i.i577.i.sroa.0.0.lcssa = phi float [ %filter_near.i.i577.i.sroa.0.0.copyload, %bb27.i601.i ], [ %_0.i4280, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9421
  %filter_near.i.i577.i.sroa.7.0.lcssa = phi float [ %filter_near.i.i577.i.sroa.7.0.copyload, %bb27.i601.i ], [ %_0.i4276, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9421
  %filter_near.i.i577.i.sroa.11.0.lcssa = phi float [ %filter_near.i.i577.i.sroa.11.0.copyload, %bb27.i601.i ], [ %_0.i4288, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9421
  %filter_near.i.i577.i.sroa.14.0.lcssa = phi float [ %filter_near.i.i577.i.sroa.14.0.copyload, %bb27.i601.i ], [ %_0.i4284, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9421
  %filter_far.i.i576.i.sroa.0.0.lcssa = phi float [ %filter_far.i.i576.i.sroa.0.0.copyload, %bb27.i601.i ], [ %_0.i4264, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9422
  %filter_far.i.i576.i.sroa.7.0.lcssa = phi float [ %filter_far.i.i576.i.sroa.7.0.copyload, %bb27.i601.i ], [ %_0.i4260, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9422
  %filter_far.i.i576.i.sroa.11.0.lcssa = phi float [ %filter_far.i.i576.i.sroa.11.0.copyload, %bb27.i601.i ], [ %_0.i4272, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9422
  %filter_far.i.i576.i.sroa.14.0.lcssa = phi float [ %filter_far.i.i576.i.sroa.14.0.copyload, %bb27.i601.i ], [ %_0.i4268, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9422
  %gain_near.i.i575.i.sroa.0.0.lcssa = phi i32 [ %725, %bb27.i601.i ], [ %_3.i4383, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9423
  %gain_near.i.i575.i.sroa.6.0.lcssa = phi i32 [ %726, %bb27.i601.i ], [ %_3.i4379, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9423
  %gain_far.i.i574.i.sroa.0.0.lcssa = phi i32 [ %727, %bb27.i601.i ], [ %_3.i4375, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9424
  %gain_far.i.i574.i.sroa.6.0.lcssa = phi i32 [ %728, %bb27.i601.i ], [ %_3.i4371, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9424
  %position.sroa.0.0.i.i615.i.lcssa = phi i64 [ %729, %bb27.i601.i ], [ %_37.sroa.0.0.i.i624.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077 ], !dbg !9425
  store float %filter_near.i.i577.i.sroa.0.0.lcssa, ptr %688, align 8, !dbg !9426, !noalias !8565
  store float %filter_near.i.i577.i.sroa.7.0.lcssa, ptr %filter_near.i.i577.i.sroa.7.0..sroa_idx, align 4, !dbg !9426, !noalias !8565
  store float %filter_near.i.i577.i.sroa.11.0.lcssa, ptr %filter_near.i.i577.i.sroa.11.0..sroa_idx, align 8, !dbg !9426, !noalias !8565
  store float %filter_near.i.i577.i.sroa.14.0.lcssa, ptr %filter_near.i.i577.i.sroa.14.0..sroa_idx, align 4, !dbg !9426, !noalias !8565
  store float %filter_far.i.i576.i.sroa.0.0.lcssa, ptr %689, align 8, !dbg !9427, !noalias !8565
  store float %filter_far.i.i576.i.sroa.7.0.lcssa, ptr %filter_far.i.i576.i.sroa.7.0..sroa_idx, align 4, !dbg !9427, !noalias !8565
  store float %filter_far.i.i576.i.sroa.11.0.lcssa, ptr %filter_far.i.i576.i.sroa.11.0..sroa_idx, align 8, !dbg !9427, !noalias !8565
  store float %filter_far.i.i576.i.sroa.14.0.lcssa, ptr %filter_far.i.i576.i.sroa.14.0..sroa_idx, align 4, !dbg !9427, !noalias !8565
  store i32 %gain_near.i.i575.i.sroa.0.0.lcssa, ptr %690, align 8, !dbg !9428, !noalias !8565
  store i32 %gain_near.i.i575.i.sroa.6.0.lcssa, ptr %.sroa_idx7026, align 4, !dbg !9428, !noalias !8565
  store i32 %gain_far.i.i574.i.sroa.0.0.lcssa, ptr %691, align 8, !dbg !9429, !noalias !8565
  store i32 %gain_far.i.i574.i.sroa.6.0.lcssa, ptr %.sroa_idx7031, align 4, !dbg !9429, !noalias !8565
  store i64 %position.sroa.0.0.i.i615.i.lcssa, ptr %_51.i596.i, align 8, !dbg !9430, !alias.scope !8523, !noalias !8524
  br label %bb15.i620.i, !dbg !9431

bb15.i620.i:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_Kb1_EB2_.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_KBZ_EB2_.exit.i.i
  %_8.i589.i = icmp ult i64 %_32.i800.i, %_19.1, !dbg !8445
  br i1 %_8.i589.i, label %bb2.i590.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit, !dbg !8445

bb17.i804.i:                                      ; preds = %bb9.i798.i
  %_75.i805.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i588.i10586, !dbg !9432
  %_85.i808.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i588.i10586, !dbg !9435
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9442), !dbg !9445
  %filter_near.i17.i570.i.sroa.0.0.copyload = load float, ptr %688, align 8, !dbg !9446, !noalias !9452
  %filter_near.i17.i570.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i577.i.sroa.7.0..sroa_idx, align 4, !dbg !9446, !noalias !9452
  %filter_near.i17.i570.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i577.i.sroa.11.0..sroa_idx, align 8, !dbg !9446, !noalias !9452
  %filter_near.i17.i570.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i577.i.sroa.14.0..sroa_idx, align 4, !dbg !9446, !noalias !9452
  %filter_far.i16.i569.i.sroa.0.0.copyload = load float, ptr %689, align 8, !dbg !9457, !noalias !9452
  %filter_far.i16.i569.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i576.i.sroa.7.0..sroa_idx, align 4, !dbg !9457, !noalias !9452
  %filter_far.i16.i569.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i576.i.sroa.11.0..sroa_idx, align 8, !dbg !9457, !noalias !9452
  %filter_far.i16.i569.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i576.i.sroa.14.0..sroa_idx, align 4, !dbg !9457, !noalias !9452
  %782 = load i32, ptr %690, align 8, !dbg !9459
  %783 = load i32, ptr %.sroa_idx7026, align 4, !dbg !9459
  %784 = load i32, ptr %691, align 8, !dbg !9461
  %785 = load i32, ptr %.sroa_idx7031, align 4, !dbg !9461
  %786 = load i64, ptr %_51.i596.i, align 8, !dbg !9463, !alias.scope !9465, !noalias !9466, !noundef !11
  %_164.i30.i820.i10557.not = icmp eq i64 %plan.0.i592.i, 0, !dbg !9468
  br i1 %_164.i30.i820.i10557.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_Kb1_EB2_.exit.i.i, label %bb54.i33.i824.i, !dbg !9478

bb19.i1014.i:                                     ; preds = %bb9.i798.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i588.i10586, i64 noundef %_32.i800.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_61a2f59006034c74bb8ab3eed52140c6) #26, !dbg !9479, !noalias !3842
  unreachable, !dbg !9479

bb54.i33.i824.i:                                  ; preds = %bb17.i804.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053
  %segments.i584.i.sroa.0.1 = phi float [ %_0.i2812, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.i6789, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.8.1 = phi float [ %_0.i2812.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.1.i6791, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.12.1 = phi float [ %_0.i2812.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.2.i6793, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.16.1 = phi float [ %_0.i2812.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.3.i6795, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.20.1 = phi float [ %_0.i2812.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.4.i6797, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.26.1 = phi float [ %_0.i2812.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.5.i6799, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.32.1 = phi float [ %_0.i2812.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.6.i6801, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.36.1 = phi float [ %_0.i2812.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.7.i6803, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.40.1 = phi float [ %_0.i2812.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.8.i6805, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.44.1 = phi float [ %_0.i2812.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.9.i6807, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.70.1 = phi float [ %_0.i2811, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.i6827, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.76.1 = phi float [ %_0.i2811.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.1.i6829, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.80.1 = phi float [ %_0.i2811.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.2.i6831, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.84.1 = phi float [ %_0.i2811.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.3.i6833, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.88.1 = phi float [ %_0.i2811.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.4.i6835, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.94.1 = phi float [ %_0.i2811.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.5.i6837, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.100.1 = phi float [ %_0.i2811.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.6.i6839, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.104.1 = phi float [ %_0.i2811.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.7.i6841, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.108.1 = phi float [ %_0.i2811.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.8.i6843, %bb17.i804.i ], !dbg !9480
  %segments.i584.i.sroa.112.1 = phi float [ %_0.i2811.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %_12.le.9.i6845, %bb17.i804.i ], !dbg !9480
  %iter.sroa.0.0.i29.i819.i10571 = phi i64 [ %787, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ 0, %bb17.i804.i ]
  %position.sroa.0.0.i28.i818.i10570 = phi i64 [ %_37.sroa.0.0.i36.i831.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %786, %bb17.i804.i ]
  %gain_far.i14.i567.i.sroa.6.010569 = phi i32 [ %_3.i4355, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %785, %bb17.i804.i ]
  %gain_far.i14.i567.i.sroa.0.010568 = phi i32 [ %_3.i4359, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %784, %bb17.i804.i ]
  %gain_near.i15.i568.i.sroa.6.010567 = phi i32 [ %_3.i4363, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %783, %bb17.i804.i ]
  %gain_near.i15.i568.i.sroa.0.010566 = phi i32 [ %_3.i4367, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %782, %bb17.i804.i ]
  %filter_far.i16.i569.i.sroa.14.010565 = phi float [ %_0.i4236, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %filter_far.i16.i569.i.sroa.14.0.copyload, %bb17.i804.i ]
  %filter_far.i16.i569.i.sroa.11.010564 = phi float [ %_0.i4240, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %filter_far.i16.i569.i.sroa.11.0.copyload, %bb17.i804.i ]
  %filter_far.i16.i569.i.sroa.7.010563 = phi float [ %_0.i4228, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %filter_far.i16.i569.i.sroa.7.0.copyload, %bb17.i804.i ]
  %filter_far.i16.i569.i.sroa.0.010562 = phi float [ %_0.i4232, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %filter_far.i16.i569.i.sroa.0.0.copyload, %bb17.i804.i ]
  %filter_near.i17.i570.i.sroa.14.010561 = phi float [ %_0.i4252, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %filter_near.i17.i570.i.sroa.14.0.copyload, %bb17.i804.i ]
  %filter_near.i17.i570.i.sroa.11.010560 = phi float [ %_0.i4256, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %filter_near.i17.i570.i.sroa.11.0.copyload, %bb17.i804.i ]
  %filter_near.i17.i570.i.sroa.7.010559 = phi float [ %_0.i4244, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %filter_near.i17.i570.i.sroa.7.0.copyload, %bb17.i804.i ]
  %filter_near.i17.i570.i.sroa.0.010558 = phi float [ %_0.i4248, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], [ %filter_near.i17.i570.i.sroa.0.0.copyload, %bb17.i804.i ]
  %_0.i2812 = fadd float %segments.i584.i.sroa.0.1, %_16.le.i6790, !dbg !9481
  %_0.i2811 = fadd float %segments.i584.i.sroa.70.1, %_16.le.i6828, !dbg !9486
  %_0.i2812.1 = fadd float %segments.i584.i.sroa.8.1, %_16.le.1.i6792, !dbg !9481
  %_0.i2811.1 = fadd float %segments.i584.i.sroa.76.1, %_16.le.1.i6830, !dbg !9486
  %_0.i2812.2 = fadd float %segments.i584.i.sroa.12.1, %_16.le.2.i6794, !dbg !9481
  %_0.i2811.2 = fadd float %segments.i584.i.sroa.80.1, %_16.le.2.i6832, !dbg !9486
  %_0.i2812.3 = fadd float %segments.i584.i.sroa.16.1, %_16.le.3.i6796, !dbg !9481
  %_0.i2811.3 = fadd float %segments.i584.i.sroa.84.1, %_16.le.3.i6834, !dbg !9486
  %_0.i2812.4 = fadd float %segments.i584.i.sroa.20.1, %_16.le.4.i6798, !dbg !9481
  %_0.i2811.4 = fadd float %segments.i584.i.sroa.88.1, %_16.le.4.i6836, !dbg !9486
  %_0.i2812.5 = fadd float %segments.i584.i.sroa.26.1, %_16.le.5.i6800, !dbg !9481
  %_0.i2811.5 = fadd float %segments.i584.i.sroa.94.1, %_16.le.5.i6838, !dbg !9486
  %_0.i2812.6 = fadd float %segments.i584.i.sroa.32.1, %_16.le.6.i6802, !dbg !9481
  %_0.i2811.6 = fadd float %segments.i584.i.sroa.100.1, %_16.le.6.i6840, !dbg !9486
  %_0.i2812.7 = fadd float %segments.i584.i.sroa.36.1, %_16.le.7.i6804, !dbg !9481
  %_0.i2811.7 = fadd float %segments.i584.i.sroa.104.1, %_16.le.7.i6842, !dbg !9486
  %_0.i2812.8 = fadd float %segments.i584.i.sroa.40.1, %_16.le.8.i6806, !dbg !9481
  %_0.i2811.8 = fadd float %segments.i584.i.sroa.108.1, %_16.le.8.i6844, !dbg !9486
  %_0.i2812.9 = fadd float %segments.i584.i.sroa.44.1, %_16.le.9.i6808, !dbg !9481
  %_0.i2811.9 = fadd float %segments.i584.i.sroa.112.1, %_16.le.9.i6846, !dbg !9486
  %787 = add nuw i64 %iter.sroa.0.0.i29.i819.i10571, 1, !dbg !9488
  %_38.i34.i829.i = add i64 %position.sroa.0.0.i28.i818.i10570, 1, !dbg !9494
  %_172.not.i35.i830.i = icmp ult i64 %_38.i34.i829.i, %ring_len.i586.i, !dbg !9496
  %788 = select i1 %_172.not.i35.i830.i, i64 0, i64 %ring_len.i586.i, !dbg !9496
  %_37.sroa.0.0.i36.i831.i = sub nuw i64 %_38.i34.i829.i, %788, !dbg !9496
  %_180.i38.i835.i = getelementptr inbounds nuw float, ptr %_75.i805.i, i64 %iter.sroa.0.0.i29.i819.i10571, !dbg !9499
  %_0.i3694 = load float, ptr %_180.i38.i835.i, align 4, !dbg !9509, !alias.scope !9511, !noalias !9514, !noundef !11
  %_188.i43.i838.i = getelementptr inbounds nuw float, ptr %_85.i808.i, i64 %iter.sroa.0.0.i29.i819.i10571, !dbg !9515
  %_0.i3689 = load float, ptr %_188.i43.i838.i, align 4, !dbg !9524, !alias.scope !9526, !noalias !9514, !noundef !11
  %_7.i106 = load float, ptr %_63.i.i631.i, align 4, !dbg !9529, !alias.scope !9532, !noalias !9535, !noundef !11
  %_8.i107 = load float, ptr %692, align 4, !dbg !9537, !alias.scope !9532, !noalias !9535, !noundef !11
  %_9.i108 = load float, ptr %693, align 4, !dbg !9538, !alias.scope !9532, !noalias !9535, !noundef !11
  %_0.i3427 = fsub float %_0.i3694, %filter_near.i17.i570.i.sroa.7.010559, !dbg !9539
  %_0.i3254 = fmul float %_0.i3427, %_8.i107, !dbg !9542
  %_4.i2899 = fmul float %filter_near.i17.i570.i.sroa.0.010558, %_7.i106, !dbg !9544
  %_0.i2900 = fadd float %_4.i2899, %_0.i3254, !dbg !9544
  %_0.i2750 = fadd float %filter_near.i17.i570.i.sroa.0.010558, %_0.i2900, !dbg !9546
  %_0.i3253 = fmul float %filter_near.i17.i570.i.sroa.0.010558, %_8.i107, !dbg !9548
  %_4.i2897 = fmul float %_0.i3427, %_9.i108, !dbg !9550
  %_0.i2898 = fadd float %_0.i3253, %_4.i2897, !dbg !9550
  %_0.i2749 = fadd float %filter_near.i17.i570.i.sroa.7.010559, %_0.i2898, !dbg !9552
  %_0.i2748 = fadd float %_0.i2900, %_0.i2900, !dbg !9554
  %_0.i2747 = fadd float %filter_near.i17.i570.i.sroa.0.010558, %_0.i2748, !dbg !9556
  %789 = tail call noundef float @llvm.fabs.f32(float %_0.i2747), !dbg !9558
  %790 = fcmp uge float %789, 0x3BC79CA100000000, !dbg !9561
  %_0.i4248 = select i1 %790, float %_0.i2747, float 0.000000e+00, !dbg !9563
  %_0.i2746 = fadd float %_0.i2898, %_0.i2898, !dbg !9564
  %_0.i2745 = fadd float %filter_near.i17.i570.i.sroa.7.010559, %_0.i2746, !dbg !9566
  %791 = tail call noundef float @llvm.fabs.f32(float %_0.i2745), !dbg !9568
  %792 = fcmp uge float %791, 0x3BC79CA100000000, !dbg !9571
  %_0.i4244 = select i1 %792, float %_0.i2745, float 0.000000e+00, !dbg !9573
  %_12.i111 = load float, ptr %694, align 4, !dbg !9574, !alias.scope !9532, !noalias !9535, !noundef !11
  %_4.i2925 = fmul float %_12.i111, %_0.i2750, !dbg !9575
  %_0.i2926 = fadd float %_0.i3694, %_4.i2925, !dbg !9575
  %_0.i3428 = fsub float %_0.i2749, %filter_near.i17.i570.i.sroa.14.010561, !dbg !9577
  %_0.i3256 = fmul float %_8.i107, %_0.i3428, !dbg !9580
  %_4.i2903 = fmul float %filter_near.i17.i570.i.sroa.11.010560, %_7.i106, !dbg !9582
  %_0.i2904 = fadd float %_4.i2903, %_0.i3256, !dbg !9582
  %_0.i3255 = fmul float %filter_near.i17.i570.i.sroa.11.010560, %_8.i107, !dbg !9584
  %_4.i2901 = fmul float %_9.i108, %_0.i3428, !dbg !9586
  %_0.i2902 = fadd float %_0.i3255, %_4.i2901, !dbg !9586
  %_0.i2755 = fadd float %filter_near.i17.i570.i.sroa.14.010561, %_0.i2902, !dbg !9588
  %_0.i2754 = fadd float %_0.i2904, %_0.i2904, !dbg !9590
  %_0.i2753 = fadd float %filter_near.i17.i570.i.sroa.11.010560, %_0.i2754, !dbg !9592
  %793 = tail call noundef float @llvm.fabs.f32(float %_0.i2753), !dbg !9594
  %794 = fcmp uge float %793, 0x3BC79CA100000000, !dbg !9597
  %_0.i4256 = select i1 %794, float %_0.i2753, float 0.000000e+00, !dbg !9599
  %_0.i2752 = fadd float %_0.i2902, %_0.i2902, !dbg !9600
  %_0.i2751 = fadd float %filter_near.i17.i570.i.sroa.14.010561, %_0.i2752, !dbg !9602
  %795 = tail call noundef float @llvm.fabs.f32(float %_0.i2751), !dbg !9604
  %796 = fcmp uge float %795, 0x3BC79CA100000000, !dbg !9607
  %_0.i4252 = select i1 %796, float %_0.i2751, float 0.000000e+00, !dbg !9609
  %_0.i3435 = fsub float %_0.i2926, %_0.i2755, !dbg !9610
  %_7.i93 = load float, ptr %_68.i.i633.i, align 4, !dbg !9612, !alias.scope !9615, !noalias !9618, !noundef !11
  %_8.i94 = load float, ptr %695, align 4, !dbg !9620, !alias.scope !9615, !noalias !9618, !noundef !11
  %_9.i95 = load float, ptr %696, align 4, !dbg !9621, !alias.scope !9615, !noalias !9618, !noundef !11
  %_0.i3425 = fsub float %_0.i3689, %filter_far.i16.i569.i.sroa.7.010563, !dbg !9622
  %_0.i3250 = fmul float %_0.i3425, %_8.i94, !dbg !9625
  %_4.i2891 = fmul float %filter_far.i16.i569.i.sroa.0.010562, %_7.i93, !dbg !9627
  %_0.i2892 = fadd float %_4.i2891, %_0.i3250, !dbg !9627
  %_0.i2738 = fadd float %filter_far.i16.i569.i.sroa.0.010562, %_0.i2892, !dbg !9629
  %_0.i3249 = fmul float %filter_far.i16.i569.i.sroa.0.010562, %_8.i94, !dbg !9631
  %_4.i2889 = fmul float %_0.i3425, %_9.i95, !dbg !9633
  %_0.i2890 = fadd float %_0.i3249, %_4.i2889, !dbg !9633
  %_0.i2737 = fadd float %filter_far.i16.i569.i.sroa.7.010563, %_0.i2890, !dbg !9635
  %_0.i2736 = fadd float %_0.i2892, %_0.i2892, !dbg !9637
  %_0.i2735 = fadd float %filter_far.i16.i569.i.sroa.0.010562, %_0.i2736, !dbg !9639
  %797 = tail call noundef float @llvm.fabs.f32(float %_0.i2735), !dbg !9641
  %798 = fcmp uge float %797, 0x3BC79CA100000000, !dbg !9644
  %_0.i4232 = select i1 %798, float %_0.i2735, float 0.000000e+00, !dbg !9646
  %_0.i2734 = fadd float %_0.i2890, %_0.i2890, !dbg !9647
  %_0.i2733 = fadd float %filter_far.i16.i569.i.sroa.7.010563, %_0.i2734, !dbg !9649
  %799 = tail call noundef float @llvm.fabs.f32(float %_0.i2733), !dbg !9651
  %800 = fcmp uge float %799, 0x3BC79CA100000000, !dbg !9654
  %_0.i4228 = select i1 %800, float %_0.i2733, float 0.000000e+00, !dbg !9656
  %_12.i98 = load float, ptr %697, align 4, !dbg !9657, !alias.scope !9615, !noalias !9618, !noundef !11
  %_4.i2927 = fmul float %_12.i98, %_0.i2738, !dbg !9658
  %_0.i2928 = fadd float %_0.i3689, %_4.i2927, !dbg !9658
  %_0.i3426 = fsub float %_0.i2737, %filter_far.i16.i569.i.sroa.14.010565, !dbg !9660
  %_0.i3252 = fmul float %_8.i94, %_0.i3426, !dbg !9663
  %_4.i2895 = fmul float %filter_far.i16.i569.i.sroa.11.010564, %_7.i93, !dbg !9665
  %_0.i2896 = fadd float %_4.i2895, %_0.i3252, !dbg !9665
  %_0.i3251 = fmul float %filter_far.i16.i569.i.sroa.11.010564, %_8.i94, !dbg !9667
  %_4.i2893 = fmul float %_9.i95, %_0.i3426, !dbg !9669
  %_0.i2894 = fadd float %_0.i3251, %_4.i2893, !dbg !9669
  %_0.i2743 = fadd float %filter_far.i16.i569.i.sroa.14.010565, %_0.i2894, !dbg !9671
  %_0.i2742 = fadd float %_0.i2896, %_0.i2896, !dbg !9673
  %_0.i2741 = fadd float %filter_far.i16.i569.i.sroa.11.010564, %_0.i2742, !dbg !9675
  %801 = tail call noundef float @llvm.fabs.f32(float %_0.i2741), !dbg !9677
  %802 = fcmp uge float %801, 0x3BC79CA100000000, !dbg !9680
  %_0.i4240 = select i1 %802, float %_0.i2741, float 0.000000e+00, !dbg !9682
  %_0.i2740 = fadd float %_0.i2894, %_0.i2894, !dbg !9683
  %_0.i2739 = fadd float %filter_far.i16.i569.i.sroa.14.010565, %_0.i2740, !dbg !9685
  %803 = tail call noundef float @llvm.fabs.f32(float %_0.i2739), !dbg !9687
  %804 = fcmp uge float %803, 0x3BC79CA100000000, !dbg !9690
  %_0.i4236 = select i1 %804, float %_0.i2739, float 0.000000e+00, !dbg !9692
  %_0.i3436 = fsub float %_0.i2928, %_0.i2743, !dbg !9693
  %_307.1.i50.i845.i = load i64, ptr %698, align 8, !dbg !9695, !noalias !9514, !noundef !11
  %_230.i51.i846.i = icmp ugt i64 %position.sroa.0.0.i28.i818.i10570, %_307.1.i50.i845.i, !dbg !9697
  br i1 %_230.i51.i846.i, label %bb76.i151.i1002.i, label %bb77.i52.i847.i, !dbg !9697, !prof !1664

bb77.i52.i847.i:                                  ; preds = %bb54.i33.i824.i
  %_307.0.i53.i848.i = load ptr, ptr %647, align 8, !dbg !9695, !noalias !9514, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9703), !dbg !9706
  %_4.not.i4070 = icmp eq i64 %_307.1.i50.i845.i, %position.sroa.0.0.i28.i818.i10570, !dbg !9707
  br i1 %_4.not.i4070, label %panic.i4072, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4073, !dbg !9707

panic.i4072:                                      ; preds = %bb77.i52.i847.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !9707, !noalias !9709
  unreachable, !dbg !9707

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4073: ; preds = %bb77.i52.i847.i
  %_237.i56.i851.i = getelementptr inbounds nuw float, ptr %_307.0.i53.i848.i, i64 %position.sroa.0.0.i28.i818.i10570, !dbg !9710
  store float %_0.i2755, ptr %_237.i56.i851.i, align 4, !dbg !9707, !alias.scope !9703, !noalias !9514
  %_308.1.i57.i852.i = load i64, ptr %700, align 8, !dbg !9716, !noalias !9514, !noundef !11
  %_238.i58.i853.i = icmp ugt i64 %position.sroa.0.0.i28.i818.i10570, %_308.1.i57.i852.i, !dbg !9717
  br i1 %_238.i58.i853.i, label %bb78.i150.i1001.i, label %bb79.i59.i854.i, !dbg !9717, !prof !1664

bb76.i151.i1002.i:                                ; preds = %bb54.i33.i824.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i818.i10570, i64 noundef %_307.1.i50.i845.i, i64 noundef %_307.1.i50.i845.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7aeb6b0a3ba8e73c50a5abef6c30558) #26, !dbg !9721, !noalias !9514
  unreachable, !dbg !9721

bb79.i59.i854.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4073
  %_308.0.i60.i855.i = load ptr, ptr %699, align 8, !dbg !9716, !noalias !9514, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9722), !dbg !9725
  %_4.not.i4066 = icmp eq i64 %_308.1.i57.i852.i, %position.sroa.0.0.i28.i818.i10570, !dbg !9726
  br i1 %_4.not.i4066, label %panic.i4068, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069, !dbg !9726

panic.i4068:                                      ; preds = %bb79.i59.i854.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !9726, !noalias !9728
  unreachable, !dbg !9726

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069: ; preds = %bb79.i59.i854.i
  %_245.i62.i857.i = getelementptr inbounds nuw float, ptr %_308.0.i60.i855.i, i64 %position.sroa.0.0.i28.i818.i10570, !dbg !9729
  store float %_0.i3435, ptr %_245.i62.i857.i, align 4, !dbg !9726, !alias.scope !9722, !noalias !9514
  %_309.1.i63.i858.i = load i64, ptr %701, align 8, !dbg !9734, !noalias !9514, !noundef !11
  %_246.i64.i859.i = icmp ugt i64 %position.sroa.0.0.i28.i818.i10570, %_309.1.i63.i858.i, !dbg !9735
  br i1 %_246.i64.i859.i, label %bb80.i149.i1000.i, label %bb81.i65.i860.i, !dbg !9735, !prof !1664

bb78.i150.i1001.i:                                ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4073
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i818.i10570, i64 noundef %_308.1.i57.i852.i, i64 noundef %_308.1.i57.i852.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8e669d1bed0fb747f8a7bff920a6571) #26, !dbg !9739, !noalias !9514
  unreachable, !dbg !9739

bb81.i65.i860.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069
  %_309.0.i66.i861.i = load ptr, ptr %_18.i594.i, align 8, !dbg !9734, !noalias !9514, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9740), !dbg !9743
  %_4.not.i4062 = icmp eq i64 %_309.1.i63.i858.i, %position.sroa.0.0.i28.i818.i10570, !dbg !9744
  br i1 %_4.not.i4062, label %panic.i4064, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4065, !dbg !9744

panic.i4064:                                      ; preds = %bb81.i65.i860.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !9744, !noalias !9746
  unreachable, !dbg !9744

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4065: ; preds = %bb81.i65.i860.i
  %_253.i68.i863.i = getelementptr inbounds nuw float, ptr %_309.0.i66.i861.i, i64 %position.sroa.0.0.i28.i818.i10570, !dbg !9747
  store float %_0.i2743, ptr %_253.i68.i863.i, align 4, !dbg !9744, !alias.scope !9740, !noalias !9514
  %_310.1.i69.i864.i = load i64, ptr %703, align 8, !dbg !9752, !noalias !9514, !noundef !11
  %_254.i70.i865.i = icmp ugt i64 %position.sroa.0.0.i28.i818.i10570, %_310.1.i69.i864.i, !dbg !9753
  br i1 %_254.i70.i865.i, label %bb82.i148.i999.i, label %bb83.i71.i866.i, !dbg !9753, !prof !1664

bb80.i149.i1000.i:                                ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i818.i10570, i64 noundef %_309.1.i63.i858.i, i64 noundef %_309.1.i63.i858.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1e81c2bc19b75441ce2fb90ce8f9eb70) #26, !dbg !9757, !noalias !9514
  unreachable, !dbg !9757

bb83.i71.i866.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4065
  %_310.0.i72.i867.i = load ptr, ptr %702, align 8, !dbg !9752, !noalias !9514, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9758), !dbg !9761
  %_4.not.i4058 = icmp eq i64 %_310.1.i69.i864.i, %position.sroa.0.0.i28.i818.i10570, !dbg !9762
  br i1 %_4.not.i4058, label %panic.i4060, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4061, !dbg !9762

panic.i4060:                                      ; preds = %bb83.i71.i866.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !9762, !noalias !9764
  unreachable, !dbg !9762

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4061: ; preds = %bb83.i71.i866.i
  %_261.i74.i869.i = getelementptr inbounds nuw float, ptr %_310.0.i72.i867.i, i64 %position.sroa.0.0.i28.i818.i10570, !dbg !9765
  store float %_0.i3436, ptr %_261.i74.i869.i, align 4, !dbg !9762, !alias.scope !9758, !noalias !9514
  %_311.0.i75.i870.i = load ptr, ptr %647, align 8, !dbg !9770, !noalias !9514, !nonnull !11, !noundef !11
  %_311.1.i76.i871.i = load i64, ptr %698, align 8, !dbg !9770, !noalias !9514, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9771), !dbg !9774
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9775), !dbg !9774
  %_13.i199.i990.i = load i64, ptr %_85.i.i663.i, align 8, !alias.scope !9775, !noalias !9777, !noundef !11
  %_12.i200.i991.i = add i64 %_13.i199.i990.i, %position.sroa.0.0.i28.i818.i10570
  %_31.not.i201.i992.i = icmp ult i64 %_12.i200.i991.i, %ring_len.i586.i
  %805 = select i1 %_31.not.i201.i992.i, i64 0, i64 %ring_len.i586.i
  %_11.sroa.0.0.i202.i993.i = sub nuw i64 %_12.i200.i991.i, %805
  %_16.i203.i994.i = icmp ult i64 %_11.sroa.0.0.i202.i993.i, %_311.1.i76.i871.i
  br i1 %_16.i203.i994.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i876.i.split.us, label %panic1.i204.i995.i

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i876.i.split.us: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4061
  %806 = getelementptr inbounds nuw float, ptr %_311.0.i75.i870.i, i64 %_11.sroa.0.0.i202.i993.i
  %_8.i206.i997.i.us.le = load float, ptr %806, align 4, !alias.scope !9771, !noalias !9778, !noundef !11
  %_312.0.i79.i878.i = load ptr, ptr %699, align 8, !dbg !9779, !noalias !9514, !nonnull !11, !noundef !11
  %_312.1.i80.i879.i = load i64, ptr %700, align 8, !dbg !9779, !noalias !9514, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9781), !dbg !9784
  %_16.i186.i984.i = icmp ult i64 %_11.sroa.0.0.i202.i993.i, %_312.1.i80.i879.i
  br i1 %_16.i186.i984.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i883.i.split.us, label %panic1.i187.i985.i

panic1.i204.i995.i:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4061
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i202.i993.i, i64 noundef range(i64 0, 2305843009213693952) %_311.1.i76.i871.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !9785, !noalias !9787
  unreachable, !dbg !9785

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i883.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i876.i.split.us
  %807 = getelementptr inbounds nuw float, ptr %_312.0.i79.i878.i, i64 %_11.sroa.0.0.i202.i993.i
  %_8.i189.i987.i.us.le = load float, ptr %807, align 4, !alias.scope !9781, !noalias !9788, !noundef !11
  %_313.0.i82.i885.i = load ptr, ptr %_18.i594.i, align 8, !dbg !9790, !noalias !9514, !nonnull !11, !noundef !11
  %_313.1.i83.i886.i = load i64, ptr %701, align 8, !dbg !9790, !noalias !9514, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9792), !dbg !9795
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9796), !dbg !9795
  %_13.i165.i970.i = load i64, ptr %_93.i.i678.i, align 8, !alias.scope !9796, !noalias !9798, !noundef !11
  %_12.i166.i971.i = add i64 %_13.i165.i970.i, %position.sroa.0.0.i28.i818.i10570
  %_31.not.i167.i972.i = icmp ult i64 %_12.i166.i971.i, %ring_len.i586.i
  %808 = select i1 %_31.not.i167.i972.i, i64 0, i64 %ring_len.i586.i
  %_11.sroa.0.0.i168.i973.i = sub nuw i64 %_12.i166.i971.i, %808
  %_16.i169.i974.i = icmp ult i64 %_11.sroa.0.0.i168.i973.i, %_313.1.i83.i886.i
  br i1 %_16.i169.i974.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i891.i.split.us, label %panic1.i170.i975.i

panic1.i187.i985.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit208.i876.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i202.i993.i, i64 noundef range(i64 0, 2305843009213693952) %_312.1.i80.i879.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !9799, !noalias !9801
  unreachable, !dbg !9799

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i891.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i883.i.split.us
  %809 = getelementptr inbounds nuw float, ptr %_313.0.i82.i885.i, i64 %_11.sroa.0.0.i168.i973.i
  %_8.i172.i977.i.us.le = load float, ptr %809, align 4, !alias.scope !9792, !noalias !9802, !noundef !11
  %_314.0.i86.i893.i = load ptr, ptr %702, align 8, !dbg !9803, !noalias !9514, !nonnull !11, !noundef !11
  %_314.1.i87.i894.i = load i64, ptr %703, align 8, !dbg !9803, !noalias !9514, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9805), !dbg !9808
  %_16.i.i964.i = icmp ult i64 %_11.sroa.0.0.i168.i973.i, %_314.1.i87.i894.i
  br i1 %_16.i.i964.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i898.i.split.us, label %panic1.i156.i965.i

panic1.i170.i975.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit191.i883.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i168.i973.i, i64 noundef range(i64 0, 2305843009213693952) %_313.1.i83.i886.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !9809, !noalias !9811
  unreachable, !dbg !9809

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i898.i.split.us: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i891.i.split.us
  %810 = getelementptr inbounds nuw float, ptr %_314.0.i86.i893.i, i64 %_11.sroa.0.0.i168.i973.i
  %_8.i.i967.i.us.le = load float, ptr %810, align 4, !alias.scope !9805, !noalias !9812, !noundef !11
  %811 = tail call noundef float @llvm.fabs.f32(float %_8.i206.i997.i.us.le), !dbg !9814
  %812 = tail call noundef float @llvm.fabs.f32(float %_8.i172.i977.i.us.le), !dbg !9818
  %_0.i3214 = fmul float %811, 5.000000e-01, !dbg !9820
  %_0.i3213 = fmul float %812, 5.000000e-01, !dbg !9822
  %_0.i2635 = fadd float %_0.i3214, %_0.i3213, !dbg !9824
  %_3.i.i5748 = fcmp ule float %_0.i2635, 0x3E45798EE0000000, !dbg !9826
  %_6.i.i5750 = bitcast float %_0.i2635 to i32, !dbg !9832
  %_4.i.i5754 = select i1 %_3.i.i5748, i32 841731191, i32 %_6.i.i5750, !dbg !9835
  %_0.i.i5755 = bitcast i32 %_4.i.i5754 to float, !dbg !9836
  %_3.i.i4890 = fcmp ule float %_0.i.i5755, 0x3810000000000000, !dbg !9838
  %_4.i.i4896 = select i1 %_3.i.i4890, i32 8388608, i32 %_4.i.i5754, !dbg !9843
  %_5.i3765 = and i32 %_4.i.i4896, 8388607, !dbg !9845
  %_4.i3766 = or disjoint i32 %_5.i3765, 1065353216, !dbg !9845
  %significand.i3767 = bitcast i32 %_4.i3766 to float, !dbg !9847
  %_0.i3341 = fadd float %significand.i3767, -1.000000e+00, !dbg !9849
  %_0.i3002 = fmul float %_0.i3341, 0xBF9B17A960000000, !dbg !9851
  %_0.i2522 = fadd float %_0.i3002, 0x3FBF9A8440000000, !dbg !9853
  %_0.i3002.1 = fmul float %_0.i3341, %_0.i2522, !dbg !9851
  %_0.i2522.1 = fadd float %_0.i3002.1, 0xBFD1E3F400000000, !dbg !9853
  %_0.i3002.2 = fmul float %_0.i3341, %_0.i2522.1, !dbg !9851
  %_0.i2522.2 = fadd float %_0.i3002.2, 0x3FDD544F20000000, !dbg !9853
  %_0.i3002.3 = fmul float %_0.i3341, %_0.i2522.2, !dbg !9851
  %_0.i2522.3 = fadd float %_0.i3002.3, 0xBFE6FC2A60000000, !dbg !9853
  %_0.i3002.4 = fmul float %_0.i3341, %_0.i2522.3, !dbg !9851
  %_0.i2522.4 = fadd float %_0.i3002.4, 0x3FF714B2A0000000, !dbg !9853
  %_9.i3768 = lshr i32 %_4.i.i4896, 23, !dbg !9855
  %_8.i3769 = or disjoint i32 %_9.i3768, 1258291200, !dbg !9855
  %_7.i3770 = bitcast i32 %_8.i3769 to float, !dbg !9856
  %exponent.i3771 = fadd float %_7.i3770, 0xC160000FE0000000, !dbg !9858
  %_0.i3001 = fmul float %_0.i3341, %_0.i2522.4, !dbg !9859
  %_0.i2521 = fadd float %exponent.i3771, %_0.i3001, !dbg !9861
  %_0.i3324 = fmul float %_0.i2521, 0x4018151820000000, !dbg !9863
  %_3.i.i5740.inv = fcmp ogt float %_0.i3324, -1.600000e+02, !dbg !9865
  %_0.i.i5747 = select i1 %_3.i.i5740.inv, float %_0.i3324, float -1.600000e+02, !dbg !9865
  %_3.i.i6356.inv = fcmp olt float %_0.i.i5747, 2.400000e+01, !dbg !9868
  %_0.i.i6363 = select i1 %_3.i.i6356.inv, float %_0.i.i5747, float 2.400000e+01, !dbg !9868
  %_0.i3389 = fsub float %_0.i.i6363, %_0.i2812, !dbg !9871
  %_3.i2233 = fcmp ule float %_0.i3389, 3.000000e+00, !dbg !9874
  %_0.i2613 = fadd float %_0.i3389, 3.000000e+00, !dbg !9876
  %_0.i3131 = fmul float %_0.i2613, %_0.i2613, !dbg !9878
  %_0.i3130 = fmul float %_0.i3131, 0x3FB5555560000000, !dbg !9880
  %_4.i4448.v.v = select i1 %_3.i2233, float %_0.i3130, float %_0.i3389, !dbg !9882
  %_4.i4448.v = fmul float %coefficients.i581.i.sroa.0.0.copyload, %_4.i4448.v.v, !dbg !9882
  %_4.i4448 = bitcast float %_4.i4448.v to i32, !dbg !9882
  %813 = fcmp ugt float %_0.i3389, -3.000000e+00, !dbg !9884
  %_7.i4440 = select i1 %813, i32 %_4.i4448, i32 0, !dbg !9886
  %_0.i4442 = bitcast i32 %_7.i4440 to float, !dbg !9887
  %_3.i.i5732 = fcmp ule float %_0.i4442, -1.000000e+02, !dbg !9889
  %814 = bitcast i32 %_7.i4440 to float, !dbg !9892
  %_0.i.i5739 = select i1 %_3.i.i5732, float -1.000000e+02, float %814, !dbg !9895
  %_3.i.i6348 = fcmp olt float %_0.i.i5739, 0.000000e+00, !dbg !9896
  %_0.i.i6355 = select i1 %_3.i.i6348, float %_0.i.i5739, float 0.000000e+00, !dbg !9899
  %815 = bitcast i32 %gain_near.i15.i568.i.sroa.0.010566 to float, !dbg !9901
  %_3.i2495 = fcmp uge float %_0.i.i6355, %815, !dbg !9902
  %_4.i4834.v = select i1 %_3.i2495, float %coefficients.i581.i.sroa.7.0.copyload, float %coefficients.i581.i.sroa.5.0.copyload, !dbg !9905
  %_0.i3464 = fsub float %815, %_0.i.i6355, !dbg !9907
  %_4.i2983 = fmul float %_0.i3464, %_4.i4834.v, !dbg !9909
  %_0.i2984 = fadd float %_0.i.i6355, %_4.i2983, !dbg !9909
  %816 = tail call noundef float @llvm.fabs.f32(float %_0.i2984), !dbg !9911
  %_4.i4365 = bitcast float %_0.i2984 to i32, !dbg !9914
  %817 = fcmp uge float %816, 0x3BC79CA100000000, !dbg !9917
  %_3.i4367 = select i1 %817, i32 %_4.i4365, i32 0, !dbg !9918
  %_0.i4368 = bitcast i32 %_3.i4367 to float, !dbg !9919
  %_0.i2818 = fadd float %_0.i2812.4, %_0.i4368, !dbg !9921
  %_0.i3323 = fmul float %_0.i2818, 0x3FC542A5A0000000, !dbg !9923
  %_3.i.i5082.inv = fcmp ogt float %_0.i3323, -1.260000e+02, !dbg !9926
  %_0.i.i5089 = select i1 %_3.i.i5082.inv, float %_0.i3323, float -1.260000e+02, !dbg !9926
  %_3.i.i5884.inv = fcmp olt float %_0.i.i5089, 1.270000e+02, !dbg !9930
  %_0.i.i5891 = select i1 %_3.i.i5884.inv, float %_0.i.i5089, float 1.270000e+02, !dbg !9930
  %818 = tail call noundef float @llvm.floor.f32(float %_0.i.i5891), !dbg !9933
  %_0.i3365 = fsub float %_0.i.i5891, %818, !dbg !9937
  %819 = tail call noundef float @llvm.fabs.f32(float %_8.i189.i987.i.us.le), !dbg !9939
  %820 = tail call noundef float @llvm.fabs.f32(float %_8.i.i967.i.us.le), !dbg !9942
  %_0.i3216 = fmul float %819, 5.000000e-01, !dbg !9944
  %_0.i3215 = fmul float %820, 5.000000e-01, !dbg !9946
  %_0.i2636 = fadd float %_0.i3216, %_0.i3215, !dbg !9948
  %_3.i.i5724 = fcmp ule float %_0.i2636, 0x3E45798EE0000000, !dbg !9950
  %_6.i.i5726 = bitcast float %_0.i2636 to i32, !dbg !9955
  %_4.i.i5730 = select i1 %_3.i.i5724, i32 841731191, i32 %_6.i.i5726, !dbg !9958
  %_0.i.i5731 = bitcast i32 %_4.i.i5730 to float, !dbg !9959
  %_3.i.i4898 = fcmp ule float %_0.i.i5731, 0x3810000000000000, !dbg !9961
  %_4.i.i4904 = select i1 %_3.i.i4898, i32 8388608, i32 %_4.i.i5730, !dbg !9966
  %_5.i3773 = and i32 %_4.i.i4904, 8388607, !dbg !9968
  %_4.i3774 = or disjoint i32 %_5.i3773, 1065353216, !dbg !9968
  %significand.i3775 = bitcast i32 %_4.i3774 to float, !dbg !9970
  %_0.i3342 = fadd float %significand.i3775, -1.000000e+00, !dbg !9972
  %_0.i3004 = fmul float %_0.i3342, 0xBF9B17A960000000, !dbg !9974
  %_0.i2524 = fadd float %_0.i3004, 0x3FBF9A8440000000, !dbg !9976
  %_0.i3004.1 = fmul float %_0.i3342, %_0.i2524, !dbg !9974
  %_0.i2524.1 = fadd float %_0.i3004.1, 0xBFD1E3F400000000, !dbg !9976
  %_0.i3004.2 = fmul float %_0.i3342, %_0.i2524.1, !dbg !9974
  %_0.i2524.2 = fadd float %_0.i3004.2, 0x3FDD544F20000000, !dbg !9976
  %_0.i3004.3 = fmul float %_0.i3342, %_0.i2524.2, !dbg !9974
  %_0.i2524.3 = fadd float %_0.i3004.3, 0xBFE6FC2A60000000, !dbg !9976
  %_0.i3004.4 = fmul float %_0.i3342, %_0.i2524.3, !dbg !9974
  %_0.i2524.4 = fadd float %_0.i3004.4, 0x3FF714B2A0000000, !dbg !9976
  %_9.i3776 = lshr i32 %_4.i.i4904, 23, !dbg !9978
  %_8.i3777 = or disjoint i32 %_9.i3776, 1258291200, !dbg !9978
  %_7.i3778 = bitcast i32 %_8.i3777 to float, !dbg !9979
  %exponent.i3779 = fadd float %_7.i3778, 0xC160000FE0000000, !dbg !9981
  %_0.i3003 = fmul float %_0.i3342, %_0.i2524.4, !dbg !9982
  %_0.i2523 = fadd float %exponent.i3779, %_0.i3003, !dbg !9984
  %_0.i3322 = fmul float %_0.i2523, 0x4018151820000000, !dbg !9986
  %_3.i.i5716.inv = fcmp ogt float %_0.i3322, -1.600000e+02, !dbg !9988
  %_0.i.i5723 = select i1 %_3.i.i5716.inv, float %_0.i3322, float -1.600000e+02, !dbg !9988
  %_3.i.i6340.inv = fcmp olt float %_0.i.i5723, 2.400000e+01, !dbg !9991
  %_0.i.i6347 = select i1 %_3.i.i6340.inv, float %_0.i.i5723, float 2.400000e+01, !dbg !9991
  %_0.i3390 = fsub float %_0.i.i6347, %_0.i2812.5, !dbg !9994
  %_3.i2235 = fcmp ule float %_0.i3390, 3.000000e+00, !dbg !9997
  %_0.i2614 = fadd float %_0.i3390, 3.000000e+00, !dbg !9999
  %_0.i3135 = fmul float %_0.i2614, %_0.i2614, !dbg !10001
  %_0.i3134 = fmul float %_0.i3135, 0x3FB5555560000000, !dbg !10003
  %_4.i4461.v.v = select i1 %_3.i2235, float %_0.i3134, float %_0.i3390, !dbg !10005
  %_4.i4461.v = fmul float %coefficients.i581.i.sroa.9.0.copyload, %_4.i4461.v.v, !dbg !10005
  %_4.i4461 = bitcast float %_4.i4461.v to i32, !dbg !10005
  %821 = fcmp ugt float %_0.i3390, -3.000000e+00, !dbg !10007
  %_7.i4453 = select i1 %821, i32 %_4.i4461, i32 0, !dbg !10009
  %_0.i4455 = bitcast i32 %_7.i4453 to float, !dbg !10010
  %_3.i.i5708 = fcmp ule float %_0.i4455, -1.000000e+02, !dbg !10012
  %822 = bitcast i32 %_7.i4453 to float, !dbg !10015
  %_0.i.i5715 = select i1 %_3.i.i5708, float -1.000000e+02, float %822, !dbg !10018
  %_3.i.i6332 = fcmp olt float %_0.i.i5715, 0.000000e+00, !dbg !10019
  %_0.i.i6339 = select i1 %_3.i.i6332, float %_0.i.i5715, float 0.000000e+00, !dbg !10022
  %823 = bitcast i32 %gain_near.i15.i568.i.sroa.6.010567 to float, !dbg !10024
  %_3.i2491 = fcmp uge float %_0.i.i6339, %823, !dbg !10025
  %_4.i4827.v = select i1 %_3.i2491, float %coefficients.i581.i.sroa.13.0.copyload, float %coefficients.i581.i.sroa.11.0.copyload, !dbg !10028
  %_0.i3463 = fsub float %823, %_0.i.i6339, !dbg !10030
  %_4.i2981 = fmul float %_0.i3463, %_4.i4827.v, !dbg !10032
  %_0.i2982 = fadd float %_0.i.i6339, %_4.i2981, !dbg !10032
  %824 = tail call noundef float @llvm.fabs.f32(float %_0.i2982), !dbg !10034
  %_4.i4361 = bitcast float %_0.i2982 to i32, !dbg !10037
  %825 = fcmp uge float %824, 0x3BC79CA100000000, !dbg !10040
  %_3.i4363 = select i1 %825, i32 %_4.i4361, i32 0, !dbg !10041
  %_0.i4364 = bitcast i32 %_3.i4363 to float, !dbg !10042
  %_0.i2817 = fadd float %_0.i2812.9, %_0.i4364, !dbg !10044
  %_0.i3321 = fmul float %_0.i2817, 0x3FC542A5A0000000, !dbg !10046
  %_3.i.i5090.inv = fcmp ogt float %_0.i3321, -1.260000e+02, !dbg !10049
  %_0.i.i5097 = select i1 %_3.i.i5090.inv, float %_0.i3321, float -1.260000e+02, !dbg !10049
  %_3.i.i5892.inv = fcmp olt float %_0.i.i5097, 1.270000e+02, !dbg !10053
  %_0.i.i5899 = select i1 %_3.i.i5892.inv, float %_0.i.i5097, float 1.270000e+02, !dbg !10053
  %826 = tail call noundef float @llvm.floor.f32(float %_0.i.i5899), !dbg !10056
  %_0.i3366 = fsub float %_0.i.i5899, %826, !dbg !10060
  %_0.i3391 = fsub float %_0.i.i6363, %_0.i2811, !dbg !10062
  %_3.i2237 = fcmp ule float %_0.i3391, 3.000000e+00, !dbg !10067
  %_0.i2615 = fadd float %_0.i3391, 3.000000e+00, !dbg !10069
  %_0.i3139 = fmul float %_0.i2615, %_0.i2615, !dbg !10071
  %_0.i3138 = fmul float %_0.i3139, 0x3FB5555560000000, !dbg !10073
  %_4.i4474.v.v = select i1 %_3.i2237, float %_0.i3138, float %_0.i3391, !dbg !10075
  %_4.i4474.v = fmul float %coefficients.i581.i.sroa.15.24.copyload, %_4.i4474.v.v, !dbg !10075
  %_4.i4474 = bitcast float %_4.i4474.v to i32, !dbg !10075
  %827 = fcmp ugt float %_0.i3391, -3.000000e+00, !dbg !10077
  %_7.i4466 = select i1 %827, i32 %_4.i4474, i32 0, !dbg !10079
  %_0.i4468 = bitcast i32 %_7.i4466 to float, !dbg !10080
  %_3.i.i5685 = fcmp ule float %_0.i4468, -1.000000e+02, !dbg !10082
  %828 = bitcast i32 %_7.i4466 to float, !dbg !10085
  %_0.i.i5691 = select i1 %_3.i.i5685, float -1.000000e+02, float %828, !dbg !10088
  %_3.i.i6316 = fcmp olt float %_0.i.i5691, 0.000000e+00, !dbg !10089
  %_0.i.i6323 = select i1 %_3.i.i6316, float %_0.i.i5691, float 0.000000e+00, !dbg !10092
  %829 = bitcast i32 %gain_far.i14.i567.i.sroa.0.010568 to float, !dbg !10094
  %_3.i2487 = fcmp uge float %_0.i.i6323, %829, !dbg !10095
  %_4.i4820.v = select i1 %_3.i2487, float %coefficients.i581.i.sroa.20.24.copyload, float %coefficients.i581.i.sroa.18.24.copyload, !dbg !10098
  %_0.i3462 = fsub float %829, %_0.i.i6323, !dbg !10100
  %_4.i2979 = fmul float %_0.i3462, %_4.i4820.v, !dbg !10102
  %_0.i2980 = fadd float %_0.i.i6323, %_4.i2979, !dbg !10102
  %830 = tail call noundef float @llvm.fabs.f32(float %_0.i2980), !dbg !10104
  %_4.i4357 = bitcast float %_0.i2980 to i32, !dbg !10107
  %831 = fcmp uge float %830, 0x3BC79CA100000000, !dbg !10110
  %_3.i4359 = select i1 %831, i32 %_4.i4357, i32 0, !dbg !10111
  %_0.i4360 = bitcast i32 %_3.i4359 to float, !dbg !10112
  %_0.i2816 = fadd float %_0.i2811.4, %_0.i4360, !dbg !10114
  %_0.i3319 = fmul float %_0.i2816, 0x3FC542A5A0000000, !dbg !10116
  %_3.i.i5098.inv = fcmp ogt float %_0.i3319, -1.260000e+02, !dbg !10119
  %_0.i.i5105 = select i1 %_3.i.i5098.inv, float %_0.i3319, float -1.260000e+02, !dbg !10119
  %_3.i.i5900.inv = fcmp olt float %_0.i.i5105, 1.270000e+02, !dbg !10123
  %_0.i.i5907 = select i1 %_3.i.i5900.inv, float %_0.i.i5105, float 1.270000e+02, !dbg !10123
  %832 = tail call noundef float @llvm.floor.f32(float %_0.i.i5907), !dbg !10126
  %_0.i3367 = fsub float %_0.i.i5907, %832, !dbg !10130
  %_0.i3392 = fsub float %_0.i.i6347, %_0.i2811.5, !dbg !10132
  %_3.i2239 = fcmp ule float %_0.i3392, 3.000000e+00, !dbg !10137
  %_0.i2616 = fadd float %_0.i3392, 3.000000e+00, !dbg !10139
  %_0.i3143 = fmul float %_0.i2616, %_0.i2616, !dbg !10141
  %_0.i3142 = fmul float %_0.i3143, 0x3FB5555560000000, !dbg !10143
  %_4.i4487.v.v = select i1 %_3.i2239, float %_0.i3142, float %_0.i3392, !dbg !10145
  %_4.i4487.v = fmul float %coefficients.i581.i.sroa.22.24.copyload, %_4.i4487.v.v, !dbg !10145
  %_4.i4487 = bitcast float %_4.i4487.v to i32, !dbg !10145
  %833 = fcmp ugt float %_0.i3392, -3.000000e+00, !dbg !10147
  %_7.i4479 = select i1 %833, i32 %_4.i4487, i32 0, !dbg !10149
  %_0.i4481 = bitcast i32 %_7.i4479 to float, !dbg !10150
  %_3.i.i5662 = fcmp ule float %_0.i4481, -1.000000e+02, !dbg !10152
  %834 = bitcast i32 %_7.i4479 to float, !dbg !10155
  %_0.i.i5669 = select i1 %_3.i.i5662, float -1.000000e+02, float %834, !dbg !10158
  %_3.i.i6300 = fcmp olt float %_0.i.i5669, 0.000000e+00, !dbg !10159
  %_0.i.i6307 = select i1 %_3.i.i6300, float %_0.i.i5669, float 0.000000e+00, !dbg !10162
  %835 = bitcast i32 %gain_far.i14.i567.i.sroa.6.010569 to float, !dbg !10164
  %_3.i2483 = fcmp uge float %_0.i.i6307, %835, !dbg !10165
  %_4.i4813.v = select i1 %_3.i2483, float %coefficients.i581.i.sroa.26.24.copyload, float %coefficients.i581.i.sroa.24.24.copyload, !dbg !10168
  %_0.i3461 = fsub float %835, %_0.i.i6307, !dbg !10170
  %_4.i2977 = fmul float %_0.i3461, %_4.i4813.v, !dbg !10172
  %_0.i2978 = fadd float %_0.i.i6307, %_4.i2977, !dbg !10172
  %836 = tail call noundef float @llvm.fabs.f32(float %_0.i2978), !dbg !10174
  %_4.i4353 = bitcast float %_0.i2978 to i32, !dbg !10177
  %837 = fcmp uge float %836, 0x3BC79CA100000000, !dbg !10180
  %_3.i4355 = select i1 %837, i32 %_4.i4353, i32 0, !dbg !10181
  %_0.i4356 = bitcast i32 %_3.i4355 to float, !dbg !10182
  %_0.i2815 = fadd float %_0.i2811.9, %_0.i4356, !dbg !10184
  %_0.i3317 = fmul float %_0.i2815, 0x3FC542A5A0000000, !dbg !10186
  %_3.i.i5106.inv = fcmp ogt float %_0.i3317, -1.260000e+02, !dbg !10189
  %_0.i.i5113 = select i1 %_3.i.i5106.inv, float %_0.i3317, float -1.260000e+02, !dbg !10189
  %_3.i.i5908.inv = fcmp olt float %_0.i.i5113, 1.270000e+02, !dbg !10193
  %_0.i.i5915 = select i1 %_3.i.i5908.inv, float %_0.i.i5113, float 1.270000e+02, !dbg !10193
  %838 = tail call noundef float @llvm.floor.f32(float %_0.i.i5915), !dbg !10196
  %_0.i3368 = fsub float %_0.i.i5915, %838, !dbg !10200
  %_0.i3064 = fmul float %_0.i3368, 0x3F5E974FA0000000, !dbg !10202
  %_0.i2576 = fadd float %_0.i3064, 0x3F82778560000000, !dbg !10204
  %_0.i3064.1 = fmul float %_0.i3368, %_0.i2576, !dbg !10202
  %_0.i2576.1 = fadd float %_0.i3064.1, 0x3FAC91CE60000000, !dbg !10204
  %_0.i3064.2 = fmul float %_0.i3368, %_0.i2576.1, !dbg !10202
  %_0.i2576.2 = fadd float %_0.i3064.2, 0x3FCEBDB560000000, !dbg !10204
  %_0.i3064.3 = fmul float %_0.i3368, %_0.i2576.2, !dbg !10202
  %_0.i2576.3 = fadd float %_0.i3064.3, 0x3FE62E4BA0000000, !dbg !10204
  %_0.i3061 = fmul float %_0.i3367, 0x3F5E974FA0000000, !dbg !10206
  %_0.i2574 = fadd float %_0.i3061, 0x3F82778560000000, !dbg !10208
  %_0.i3061.1 = fmul float %_0.i3367, %_0.i2574, !dbg !10206
  %_0.i2574.1 = fadd float %_0.i3061.1, 0x3FAC91CE60000000, !dbg !10208
  %_0.i3061.2 = fmul float %_0.i3367, %_0.i2574.1, !dbg !10206
  %_0.i2574.2 = fadd float %_0.i3061.2, 0x3FCEBDB560000000, !dbg !10208
  %_0.i3061.3 = fmul float %_0.i3367, %_0.i2574.2, !dbg !10206
  %_0.i2574.3 = fadd float %_0.i3061.3, 0x3FE62E4BA0000000, !dbg !10208
  %_0.i3058 = fmul float %_0.i3366, 0x3F5E974FA0000000, !dbg !10210
  %_0.i2572 = fadd float %_0.i3058, 0x3F82778560000000, !dbg !10212
  %_0.i3058.1 = fmul float %_0.i3366, %_0.i2572, !dbg !10210
  %_0.i2572.1 = fadd float %_0.i3058.1, 0x3FAC91CE60000000, !dbg !10212
  %_0.i3058.2 = fmul float %_0.i3366, %_0.i2572.1, !dbg !10210
  %_0.i2572.2 = fadd float %_0.i3058.2, 0x3FCEBDB560000000, !dbg !10212
  %_0.i3058.3 = fmul float %_0.i3366, %_0.i2572.2, !dbg !10210
  %_0.i2572.3 = fadd float %_0.i3058.3, 0x3FE62E4BA0000000, !dbg !10212
  %_0.i3055 = fmul float %_0.i3365, 0x3F5E974FA0000000, !dbg !10214
  %_0.i2570 = fadd float %_0.i3055, 0x3F82778560000000, !dbg !10216
  %_0.i3055.1 = fmul float %_0.i3365, %_0.i2570, !dbg !10214
  %_0.i2570.1 = fadd float %_0.i3055.1, 0x3FAC91CE60000000, !dbg !10216
  %_0.i3055.2 = fmul float %_0.i3365, %_0.i2570.1, !dbg !10214
  %_0.i2570.2 = fadd float %_0.i3055.2, 0x3FCEBDB560000000, !dbg !10216
  %_0.i3055.3 = fmul float %_0.i3365, %_0.i2570.2, !dbg !10214
  %_0.i2570.3 = fadd float %_0.i3055.3, 0x3FE62E4BA0000000, !dbg !10216
  %_0.i3054 = fmul float %_0.i3365, %_0.i2570.3, !dbg !10218
  %_0.i2569 = fadd float %_0.i3054, 1.000000e+00, !dbg !10220
  %biased.i2146 = fadd float %818, 0x4160000FE0000000, !dbg !10222
  %_4.i2147 = bitcast float %biased.i2146 to i32, !dbg !10224
  %_3.i2148 = shl i32 %_4.i2147, 23, !dbg !10226
  %_0.i2149 = bitcast i32 %_3.i2148 to float, !dbg !10227
  %_0.i3053 = fmul float %_0.i2569, %_0.i2149, !dbg !10229
  %_0.i3057 = fmul float %_0.i3366, %_0.i2572.3, !dbg !10231
  %_0.i2571 = fadd float %_0.i3057, 1.000000e+00, !dbg !10233
  %biased.i2150 = fadd float %826, 0x4160000FE0000000, !dbg !10235
  %_4.i2151 = bitcast float %biased.i2150 to i32, !dbg !10237
  %_3.i2152 = shl i32 %_4.i2151, 23, !dbg !10239
  %_0.i2153 = bitcast i32 %_3.i2152 to float, !dbg !10240
  %_0.i3056 = fmul float %_0.i2571, %_0.i2153, !dbg !10242
  %_0.i3060 = fmul float %_0.i3367, %_0.i2574.3, !dbg !10244
  %_0.i2573 = fadd float %_0.i3060, 1.000000e+00, !dbg !10246
  %biased.i2154 = fadd float %832, 0x4160000FE0000000, !dbg !10248
  %_4.i2155 = bitcast float %biased.i2154 to i32, !dbg !10250
  %_3.i2156 = shl i32 %_4.i2155, 23, !dbg !10252
  %_0.i2157 = bitcast i32 %_3.i2156 to float, !dbg !10253
  %_0.i3059 = fmul float %_0.i2573, %_0.i2157, !dbg !10255
  %_0.i3063 = fmul float %_0.i3368, %_0.i2576.3, !dbg !10257
  %_0.i2575 = fadd float %_0.i3063, 1.000000e+00, !dbg !10259
  %biased.i2158 = fadd float %838, 0x4160000FE0000000, !dbg !10261
  %_4.i2159 = bitcast float %biased.i2158 to i32, !dbg !10263
  %_3.i2160 = shl i32 %_4.i2159, 23, !dbg !10265
  %_0.i2161 = bitcast i32 %_3.i2160 to float, !dbg !10266
  %_0.i3062 = fmul float %_0.i2575, %_0.i2161, !dbg !10268
  %_262.i110.i921.i = icmp ugt i64 %_37.sroa.0.0.i36.i831.i, %_311.1.i76.i871.i, !dbg !10270
  br i1 %_262.i110.i921.i, label %bb84.i147.i958.i, label %bb85.i111.i922.i, !dbg !10270, !prof !1664

panic1.i156.i965.i:                               ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit174.i891.i.split.us
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_11.sroa.0.0.i168.i973.i, i64 noundef range(i64 0, 2305843009213693952) %_314.1.i87.i894.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_996d872dbf8039f736508df84ee27faf) #26, !dbg !10275, !noalias !10277
  unreachable, !dbg !10275

bb82.i148.i999.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4065
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i28.i818.i10570, i64 noundef %_310.1.i69.i864.i, i64 noundef %_310.1.i69.i864.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_007bf1cfdcf9f845db5ef166fc9a1790) #26, !dbg !10278, !noalias !9514
  unreachable, !dbg !10278

bb85.i111.i922.i:                                 ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i898.i.split.us
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10279), !dbg !10282
  %_3.not.i3674 = icmp eq i64 %_311.1.i76.i871.i, %_37.sroa.0.0.i36.i831.i, !dbg !10283
  br i1 %_3.not.i3674, label %panic.i3677, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3678, !dbg !10283

panic.i3677:                                      ; preds = %bb85.i111.i922.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10283, !noalias !10285
  unreachable, !dbg !10283

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3678: ; preds = %bb85.i111.i922.i
  %_269.i114.i925.i = getelementptr inbounds nuw float, ptr %_311.0.i75.i870.i, i64 %_37.sroa.0.0.i36.i831.i, !dbg !10286
  %_0.i3676 = load float, ptr %_269.i114.i925.i, align 4, !dbg !10283, !alias.scope !10279, !noalias !9514, !noundef !11
  %_0.i3316 = fmul float %_0.i3053, %_0.i3676, !dbg !10291
  %_270.i118.i929.i = icmp ugt i64 %_37.sroa.0.0.i36.i831.i, %_312.1.i80.i879.i, !dbg !10293
  br i1 %_270.i118.i929.i, label %bb86.i146.i957.i, label %bb87.i119.i930.i, !dbg !10293, !prof !1664

bb84.i147.i958.i:                                 ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i898.i.split.us
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i831.i, i64 noundef %_311.1.i76.i871.i, i64 noundef %_311.1.i76.i871.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a45b75cd2d07085fe69fe186ba115725) #26, !dbg !10297, !noalias !9514
  unreachable, !dbg !10297

bb87.i119.i930.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3678
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10298), !dbg !10301
  %_3.not.i3669 = icmp eq i64 %_312.1.i80.i879.i, %_37.sroa.0.0.i36.i831.i, !dbg !10302
  br i1 %_3.not.i3669, label %panic.i3672, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3673, !dbg !10302

panic.i3672:                                      ; preds = %bb87.i119.i930.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10302, !noalias !10304
  unreachable, !dbg !10302

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3673: ; preds = %bb87.i119.i930.i
  %_277.i122.i933.i = getelementptr inbounds nuw float, ptr %_312.0.i79.i878.i, i64 %_37.sroa.0.0.i36.i831.i, !dbg !10305
  %_0.i3671 = load float, ptr %_277.i122.i933.i, align 4, !dbg !10302, !alias.scope !10298, !noalias !9514, !noundef !11
  %_0.i3315 = fmul float %_0.i3056, %_0.i3671, !dbg !10310
  %_0.i2814 = fadd float %_0.i3316, %_0.i3315, !dbg !10312
  %_278.i127.i938.i = icmp ugt i64 %_37.sroa.0.0.i36.i831.i, %_313.1.i83.i886.i, !dbg !10314
  br i1 %_278.i127.i938.i, label %bb88.i145.i956.i, label %bb89.i128.i939.i, !dbg !10314, !prof !1664

bb86.i146.i957.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3678
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i831.i, i64 noundef %_312.1.i80.i879.i, i64 noundef %_312.1.i80.i879.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_36be93341d7083c8a492c530428430ea) #26, !dbg !10319, !noalias !9514
  unreachable, !dbg !10319

bb89.i128.i939.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3673
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10320), !dbg !10323
  %_3.not.i3664 = icmp eq i64 %_313.1.i83.i886.i, %_37.sroa.0.0.i36.i831.i, !dbg !10324
  br i1 %_3.not.i3664, label %panic.i3667, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3668, !dbg !10324

panic.i3667:                                      ; preds = %bb89.i128.i939.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10324, !noalias !10326
  unreachable, !dbg !10324

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3668: ; preds = %bb89.i128.i939.i
  %_285.i131.i942.i = getelementptr inbounds nuw float, ptr %_313.0.i82.i885.i, i64 %_37.sroa.0.0.i36.i831.i, !dbg !10327
  %_0.i3666 = load float, ptr %_285.i131.i942.i, align 4, !dbg !10324, !alias.scope !10320, !noalias !9514, !noundef !11
  %_0.i3314 = fmul float %_0.i3059, %_0.i3666, !dbg !10332
  %_286.i135.i946.i = icmp ugt i64 %_37.sroa.0.0.i36.i831.i, %_314.1.i87.i894.i, !dbg !10334
  br i1 %_286.i135.i946.i, label %bb90.i144.i955.i, label %bb91.i136.i947.i, !dbg !10334, !prof !1664

bb88.i145.i956.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3673
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i831.i, i64 noundef %_313.1.i83.i886.i, i64 noundef %_313.1.i83.i886.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a6f1b2a44d3e33eba5c708677237c81) #26, !dbg !10338, !noalias !9514
  unreachable, !dbg !10338

bb91.i136.i947.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3668
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10339), !dbg !10342
  %_3.not.i3659 = icmp eq i64 %_314.1.i87.i894.i, %_37.sroa.0.0.i36.i831.i, !dbg !10343
  br i1 %_3.not.i3659, label %panic.i3662, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053, !dbg !10343

panic.i3662:                                      ; preds = %bb91.i136.i947.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10343, !noalias !10345
  unreachable, !dbg !10343

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053: ; preds = %bb91.i136.i947.i
  %_293.i139.i950.i = getelementptr inbounds nuw float, ptr %_314.0.i86.i893.i, i64 %_37.sroa.0.0.i36.i831.i, !dbg !10346
  %_0.i3661 = load float, ptr %_293.i139.i950.i, align 4, !dbg !10343, !alias.scope !10339, !noalias !9514, !noundef !11
  %_0.i3313 = fmul float %_0.i3062, %_0.i3661, !dbg !10351
  %_0.i2813 = fadd float %_0.i3314, %_0.i3313, !dbg !10353
  store float %_0.i2814, ptr %_180.i38.i835.i, align 4, !dbg !10355, !alias.scope !10358, !noalias !9514
  store float %_0.i2813, ptr %_188.i43.i838.i, align 4, !dbg !10361, !alias.scope !10363, !noalias !9514
  %exitcond13929.not = icmp eq i64 %787, %plan.0.i592.i, !dbg !9468
  br i1 %exitcond13929.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_Kb1_EB2_.exit.i.i, label %bb54.i33.i824.i, !dbg !9478

bb90.i144.i955.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3668
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_37.sroa.0.0.i36.i831.i, i64 noundef %_314.1.i87.i894.i, i64 noundef %_314.1.i87.i894.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4aa2eaec3d1833a4a887fe1d76c05ca7) #26, !dbg !10366, !noalias !9514
  unreachable, !dbg !10366

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_Kb1_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053, %bb17.i804.i
  %segments.i584.i.sroa.0.0 = phi float [ %_12.le.i6789, %bb17.i804.i ], [ %_0.i2812, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.8.0 = phi float [ %_12.le.1.i6791, %bb17.i804.i ], [ %_0.i2812.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.12.0 = phi float [ %_12.le.2.i6793, %bb17.i804.i ], [ %_0.i2812.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.16.0 = phi float [ %_12.le.3.i6795, %bb17.i804.i ], [ %_0.i2812.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.20.0 = phi float [ %_12.le.4.i6797, %bb17.i804.i ], [ %_0.i2812.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.26.0 = phi float [ %_12.le.5.i6799, %bb17.i804.i ], [ %_0.i2812.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.32.0 = phi float [ %_12.le.6.i6801, %bb17.i804.i ], [ %_0.i2812.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.36.0 = phi float [ %_12.le.7.i6803, %bb17.i804.i ], [ %_0.i2812.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.40.0 = phi float [ %_12.le.8.i6805, %bb17.i804.i ], [ %_0.i2812.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.44.0 = phi float [ %_12.le.9.i6807, %bb17.i804.i ], [ %_0.i2812.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.70.0 = phi float [ %_12.le.i6827, %bb17.i804.i ], [ %_0.i2811, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.76.0 = phi float [ %_12.le.1.i6829, %bb17.i804.i ], [ %_0.i2811.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.80.0 = phi float [ %_12.le.2.i6831, %bb17.i804.i ], [ %_0.i2811.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.84.0 = phi float [ %_12.le.3.i6833, %bb17.i804.i ], [ %_0.i2811.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.88.0 = phi float [ %_12.le.4.i6835, %bb17.i804.i ], [ %_0.i2811.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.94.0 = phi float [ %_12.le.5.i6837, %bb17.i804.i ], [ %_0.i2811.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.100.0 = phi float [ %_12.le.6.i6839, %bb17.i804.i ], [ %_0.i2811.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.104.0 = phi float [ %_12.le.7.i6841, %bb17.i804.i ], [ %_0.i2811.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.108.0 = phi float [ %_12.le.8.i6843, %bb17.i804.i ], [ %_0.i2811.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %segments.i584.i.sroa.112.0 = phi float [ %_12.le.9.i6845, %bb17.i804.i ], [ %_0.i2811.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !9480
  %filter_near.i17.i570.i.sroa.0.0.lcssa = phi float [ %filter_near.i17.i570.i.sroa.0.0.copyload, %bb17.i804.i ], [ %_0.i4248, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10367
  %filter_near.i17.i570.i.sroa.7.0.lcssa = phi float [ %filter_near.i17.i570.i.sroa.7.0.copyload, %bb17.i804.i ], [ %_0.i4244, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10367
  %filter_near.i17.i570.i.sroa.11.0.lcssa = phi float [ %filter_near.i17.i570.i.sroa.11.0.copyload, %bb17.i804.i ], [ %_0.i4256, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10367
  %filter_near.i17.i570.i.sroa.14.0.lcssa = phi float [ %filter_near.i17.i570.i.sroa.14.0.copyload, %bb17.i804.i ], [ %_0.i4252, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10367
  %filter_far.i16.i569.i.sroa.0.0.lcssa = phi float [ %filter_far.i16.i569.i.sroa.0.0.copyload, %bb17.i804.i ], [ %_0.i4232, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10368
  %filter_far.i16.i569.i.sroa.7.0.lcssa = phi float [ %filter_far.i16.i569.i.sroa.7.0.copyload, %bb17.i804.i ], [ %_0.i4228, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10368
  %filter_far.i16.i569.i.sroa.11.0.lcssa = phi float [ %filter_far.i16.i569.i.sroa.11.0.copyload, %bb17.i804.i ], [ %_0.i4240, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10368
  %filter_far.i16.i569.i.sroa.14.0.lcssa = phi float [ %filter_far.i16.i569.i.sroa.14.0.copyload, %bb17.i804.i ], [ %_0.i4236, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10368
  %gain_near.i15.i568.i.sroa.0.0.lcssa = phi i32 [ %782, %bb17.i804.i ], [ %_3.i4367, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10369
  %gain_near.i15.i568.i.sroa.6.0.lcssa = phi i32 [ %783, %bb17.i804.i ], [ %_3.i4363, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10369
  %gain_far.i14.i567.i.sroa.0.0.lcssa = phi i32 [ %784, %bb17.i804.i ], [ %_3.i4359, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10370
  %gain_far.i14.i567.i.sroa.6.0.lcssa = phi i32 [ %785, %bb17.i804.i ], [ %_3.i4355, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10370
  %position.sroa.0.0.i28.i818.i.lcssa = phi i64 [ %786, %bb17.i804.i ], [ %_37.sroa.0.0.i36.i831.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053 ], !dbg !10371
  store float %filter_near.i17.i570.i.sroa.0.0.lcssa, ptr %688, align 8, !dbg !10372, !noalias !9514
  store float %filter_near.i17.i570.i.sroa.7.0.lcssa, ptr %filter_near.i.i577.i.sroa.7.0..sroa_idx, align 4, !dbg !10372, !noalias !9514
  store float %filter_near.i17.i570.i.sroa.11.0.lcssa, ptr %filter_near.i.i577.i.sroa.11.0..sroa_idx, align 8, !dbg !10372, !noalias !9514
  store float %filter_near.i17.i570.i.sroa.14.0.lcssa, ptr %filter_near.i.i577.i.sroa.14.0..sroa_idx, align 4, !dbg !10372, !noalias !9514
  store float %filter_far.i16.i569.i.sroa.0.0.lcssa, ptr %689, align 8, !dbg !10373, !noalias !9514
  store float %filter_far.i16.i569.i.sroa.7.0.lcssa, ptr %filter_far.i.i576.i.sroa.7.0..sroa_idx, align 4, !dbg !10373, !noalias !9514
  store float %filter_far.i16.i569.i.sroa.11.0.lcssa, ptr %filter_far.i.i576.i.sroa.11.0..sroa_idx, align 8, !dbg !10373, !noalias !9514
  store float %filter_far.i16.i569.i.sroa.14.0.lcssa, ptr %filter_far.i.i576.i.sroa.14.0..sroa_idx, align 4, !dbg !10373, !noalias !9514
  store i32 %gain_near.i15.i568.i.sroa.0.0.lcssa, ptr %690, align 8, !dbg !10374, !noalias !9514
  store i32 %gain_near.i15.i568.i.sroa.6.0.lcssa, ptr %.sroa_idx7026, align 4, !dbg !10374, !noalias !9514
  store i32 %gain_far.i14.i567.i.sroa.0.0.lcssa, ptr %691, align 8, !dbg !10375, !noalias !9514
  store i32 %gain_far.i14.i567.i.sroa.6.0.lcssa, ptr %.sroa_idx7031, align 4, !dbg !10375, !noalias !9514
  store i64 %position.sroa.0.0.i28.i818.i.lcssa, ptr %_51.i596.i, align 8, !dbg !10376, !alias.scope !9465, !noalias !9466
  %advanced.i823.i = trunc i64 %plan.0.i592.i to i32, !dbg !10377
  store float %segments.i584.i.sroa.0.0, ptr %648, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.i6874 = load i32, ptr %704, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %839 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6874, i32 %advanced.i823.i), !dbg !10387
  store i32 %839, ptr %704, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.8.0, ptr %650, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.1.i6877 = load i32, ptr %705, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %840 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6877, i32 %advanced.i823.i), !dbg !10387
  store i32 %840, ptr %705, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.12.0, ptr %652, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.2.i6880 = load i32, ptr %706, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %841 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6880, i32 %advanced.i823.i), !dbg !10387
  store i32 %841, ptr %706, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.16.0, ptr %654, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.3.i6883 = load i32, ptr %707, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %842 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6883, i32 %advanced.i823.i), !dbg !10387
  store i32 %842, ptr %707, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.20.0, ptr %656, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.4.i6886 = load i32, ptr %708, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %843 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6886, i32 %advanced.i823.i), !dbg !10387
  store i32 %843, ptr %708, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.26.0, ptr %658, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.5.i6889 = load i32, ptr %709, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %844 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6889, i32 %advanced.i823.i), !dbg !10387
  store i32 %844, ptr %709, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.32.0, ptr %660, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.6.i6892 = load i32, ptr %710, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %845 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6892, i32 %advanced.i823.i), !dbg !10387
  store i32 %845, ptr %710, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.36.0, ptr %662, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.7.i6895 = load i32, ptr %711, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %846 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6895, i32 %advanced.i823.i), !dbg !10387
  store i32 %846, ptr %711, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.40.0, ptr %664, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.8.i6898 = load i32, ptr %712, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %847 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6898, i32 %advanced.i823.i), !dbg !10387
  store i32 %847, ptr %712, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.44.0, ptr %666, align 4, !dbg !10378, !alias.scope !10381, !noalias !10384
  %_26.9.i6901 = load i32, ptr %713, align 4, !dbg !10386, !alias.scope !10381, !noalias !10384, !noundef !11
  %848 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6901, i32 %advanced.i823.i), !dbg !10387
  store i32 %848, ptr %713, align 4, !dbg !10389, !alias.scope !10381, !noalias !10384
  store float %segments.i584.i.sroa.70.0, ptr %668, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.i6903 = load i32, ptr %714, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %849 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6903, i32 %advanced.i823.i), !dbg !10398
  store i32 %849, ptr %714, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.76.0, ptr %670, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.1.i6906 = load i32, ptr %715, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %850 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6906, i32 %advanced.i823.i), !dbg !10398
  store i32 %850, ptr %715, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.80.0, ptr %672, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.2.i6909 = load i32, ptr %716, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %851 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6909, i32 %advanced.i823.i), !dbg !10398
  store i32 %851, ptr %716, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.84.0, ptr %674, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.3.i6912 = load i32, ptr %717, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %852 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6912, i32 %advanced.i823.i), !dbg !10398
  store i32 %852, ptr %717, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.88.0, ptr %676, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.4.i6915 = load i32, ptr %718, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %853 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6915, i32 %advanced.i823.i), !dbg !10398
  store i32 %853, ptr %718, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.94.0, ptr %678, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.5.i6918 = load i32, ptr %719, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %854 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6918, i32 %advanced.i823.i), !dbg !10398
  store i32 %854, ptr %719, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.100.0, ptr %680, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.6.i6921 = load i32, ptr %720, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %855 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6921, i32 %advanced.i823.i), !dbg !10398
  store i32 %855, ptr %720, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.104.0, ptr %682, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.7.i6924 = load i32, ptr %721, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %856 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6924, i32 %advanced.i823.i), !dbg !10398
  store i32 %856, ptr %721, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.108.0, ptr %684, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.8.i6927 = load i32, ptr %722, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %857 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6927, i32 %advanced.i823.i), !dbg !10398
  store i32 %857, ptr %722, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  store float %segments.i584.i.sroa.112.0, ptr %686, align 4, !dbg !10390, !alias.scope !10392, !noalias !10395
  %_26.9.i6930 = load i32, ptr %723, align 4, !dbg !10397, !alias.scope !10392, !noalias !10395, !noundef !11
  %858 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6930, i32 %advanced.i823.i), !dbg !10398
  store i32 %858, ptr %723, align 4, !dbg !10400, !alias.scope !10392, !noalias !10395
  br label %bb15.i620.i, !dbg !9431

_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit: ; preds = %bb15.i620.i, %bb15.i162.i, %bb15.i.i, %bb15.i39.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10401), !dbg !10404
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10405), !dbg !10404
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10407), !dbg !10404
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10409), !dbg !10412
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10417), !dbg !10412
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10419), !dbg !10412
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10421), !dbg !10412
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10423), !dbg !10412
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !10425

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit
  %ok.sroa.0.0.i1355.i.i = phi i32 [ -1, %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit ], [ %_0.i33.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ]
  %iter.sroa.0.0.i1254.i.i = phi ptr [ %_19.0, %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit ], [ %_27.i16.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ]
  %iter.sroa.5.0.i1153.i.i = phi i64 [ %_19.1, %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit ], [ %_28.i17.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ]
  %_27.i16.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1254.i.i, i64 4, !dbg !10441
  %_28.i17.i.i = add nsw i64 %iter.sroa.5.0.i1153.i.i, -1, !dbg !10451
  %_0.i28.i.i = load float, ptr %iter.sroa.0.0.i1254.i.i, align 4, !dbg !10452, !alias.scope !10455, !noalias !10458, !noundef !11
  %859 = tail call noundef float @llvm.fabs.f32(float %_0.i28.i.i), !dbg !10460
  %_3.i.i.i = fcmp olt float %859, 0x46293E5940000000, !dbg !10463
  %_0.i33.i.i = select i1 %_3.i.i.i, i32 %ok.sroa.0.0.i1355.i.i, i32 0, !dbg !10465
  %_22.not.i14.i.i = icmp eq i64 %_28.i17.i.i, 0, !dbg !10425
  br i1 %_22.not.i14.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !10425

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %sides.i = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !10468
  %cursor.i = getelementptr inbounds nuw i8, ptr %self, i64 848, !dbg !10469
  %_0.i35.not.i.i = icmp eq i32 %_0.i33.i.i, -1, !dbg !10470
  br i1 %_0.i35.not.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i, label %bb7.i.i, !dbg !10473

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i
  %ok.sroa.0.0.i58.i.i = phi i32 [ %_0.i34.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i ], [ -1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i ]
  %iter.sroa.0.0.i57.i.i = phi ptr [ %_27.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i ], [ %_20.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i ]
  %iter.sroa.5.0.i56.i.i = phi i64 [ %_28.i.i.i6936, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i ], [ %_19.1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i ]
  %_27.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i57.i.i, i64 4, !dbg !10474
  %_28.i.i.i6936 = add nsw i64 %iter.sroa.5.0.i56.i.i, -1, !dbg !10480
  %_0.i30.i.i = load float, ptr %iter.sroa.0.0.i57.i.i, align 4, !dbg !10481, !alias.scope !10483, !noalias !10486, !noundef !11
  %860 = tail call noundef float @llvm.fabs.f32(float %_0.i30.i.i), !dbg !10487
  %_3.i26.i.i = fcmp olt float %860, 0x46293E5940000000, !dbg !10489
  %_0.i34.i.i = select i1 %_3.i26.i.i, i32 %ok.sroa.0.0.i58.i.i, i32 0, !dbg !10491
  %_22.not.i.i.i = icmp eq i64 %_28.i.i.i6936, 0, !dbg !10493
  br i1 %_22.not.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i, !dbg !10493

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i
  %_0.i36.not.i.i = icmp eq i32 %_0.i34.i.i, -1, !dbg !10494
  br i1 %_0.i36.not.i.i, label %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E6finishB5_.exit, label %bb7.i.i, !dbg !10496

bb7.i.i:                                          ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, !dbg !10497

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, %bb7.i.i
  %ok.sroa.0.016.i.i.i = phi i32 [ -1, %bb7.i.i ], [ %_0.i7.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ]
  %iter.sroa.0.015.i.i.i = phi ptr [ %_19.0, %bb7.i.i ], [ %_45.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ]
  %iter.sroa.5.014.i.i.i = phi i64 [ %_19.1, %bb7.i.i ], [ %_46.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ]
  %_45.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.015.i.i.i, i64 4, !dbg !10508
  %_46.i.i.i = add nsw i64 %iter.sroa.5.014.i.i.i, -1, !dbg !10515
  %_0.i.i.i.i = load float, ptr %iter.sroa.0.015.i.i.i, align 4, !dbg !10516, !alias.scope !10519, !noalias !10458, !noundef !11
  %861 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i.i), !dbg !10524
  %_3.i.i.i.i = fcmp olt float %861, 0x46293E5940000000, !dbg !10527
  %_0.i7.i.i.i = select i1 %_3.i.i.i.i, i32 %ok.sroa.0.016.i.i.i, i32 0, !dbg !10529
  %_40.not.i.i.i = icmp eq i64 %_46.i.i.i, 0, !dbg !10497
  br i1 %_40.not.i.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, !dbg !10497

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i, !dbg !10531

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i
  %ok.sroa.0.016.i41.i.i = phi i32 [ %_0.i7.i48.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i ], [ -1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader ]
  %iter.sroa.0.015.i42.i.i = phi ptr [ %_45.i44.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i ], [ %_20.0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader ]
  %iter.sroa.5.014.i43.i.i = phi i64 [ %_46.i45.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i ], [ %_19.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader ]
  %_45.i44.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.015.i42.i.i, i64 4, !dbg !10535
  %_46.i45.i.i = add nsw i64 %iter.sroa.5.014.i43.i.i, -1, !dbg !10538
  %_0.i.i46.i.i = load float, ptr %iter.sroa.0.015.i42.i.i, align 4, !dbg !10539, !alias.scope !10541, !noalias !10486, !noundef !11
  %862 = tail call noundef float @llvm.fabs.f32(float %_0.i.i46.i.i), !dbg !10546
  %_3.i.i47.i.i = fcmp olt float %862, 0x46293E5940000000, !dbg !10548
  %_0.i7.i48.i.i = select i1 %_3.i.i47.i.i, i32 %ok.sroa.0.016.i41.i.i, i32 0, !dbg !10550
  %_40.not.i49.i.i = icmp eq i64 %_46.i45.i.i, 0, !dbg !10531
  br i1 %_40.not.i49.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECs5xZ4lC6BZIV_20multiband_compressor.exit51.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i, !dbg !10531

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECs5xZ4lC6BZIV_20multiband_compressor.exit51.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i
  %863 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !10552
  %864 = and i32 %_0.i7.i.i.i, 1065353216, !dbg !10552
  %865 = and i32 %864, %_0.i7.i48.i.i, !dbg !10552
  %866 = icmp ne i32 %865, 1065353216, !dbg !10552
  %867 = zext i1 %866 to i32, !dbg !10552
  store i32 %867, ptr %863, align 8, !dbg !10552, !alias.scope !10553, !noalias !10554
  %_14.i.i6931 = load i64, ptr %_5, align 8, !dbg !10555, !alias.scope !10553, !noalias !10554, !noundef !11
  %868 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i.i6931, i64 1), !dbg !10556
  store i64 %868, ptr %_5, align 8, !dbg !10559, !alias.scope !10553, !noalias !10554
  %.idx.i.i.i = shl nuw nsw i64 %_19.1, 2, !dbg !10560
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %_19.0, i8 0, i64 %.idx.i.i.i, i1 false), !dbg !10567, !alias.scope !10568, !noalias !10458
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %_20.0, i8 0, i64 %.idx.i.i.i, i1 false), !dbg !10571, !alias.scope !10574, !noalias !10486
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10577), !dbg !10580
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10581), !dbg !10580
  store i64 0, ptr %cursor.i, align 8, !dbg !10583, !alias.scope !10587, !noalias !10588
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10589), !dbg !10592
  %869 = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !10595
  %870 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !10597
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %869, i8 0, i64 24, i1 false), !alias.scope !10598, !noalias !10599
  %_39.1.i.i.i.i = load i64, ptr %870, align 8, !dbg !10597, !alias.scope !10600, !noalias !10599, !noundef !11
  %_222.i.i.i.i.i = icmp eq i64 %_39.1.i.i.i.i, 0, !dbg !10601
  br i1 %_222.i.i.i.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i, label %bb14.preheader.i.i.i.i.i, !dbg !10606

bb14.preheader.i.i.i.i.i:                         ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECs5xZ4lC6BZIV_20multiband_compressor.exit51.i.i
  %_39.0.i.i.i.i = load ptr, ptr %sides.i, align 8, !dbg !10597, !alias.scope !10600, !noalias !10599, !nonnull !11, !noundef !11
  %.idx.i.i.i.i.i = shl nuw nsw i64 %_39.1.i.i.i.i, 2, !dbg !10607
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_39.0.i.i.i.i, i8 0, i64 %.idx.i.i.i.i.i, i1 false), !dbg !10611, !alias.scope !10612, !noalias !10615
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i, !dbg !10616

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i: ; preds = %bb14.preheader.i.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECs5xZ4lC6BZIV_20multiband_compressor.exit51.i.i
  %871 = getelementptr inbounds nuw i8, ptr %self, i64 144, !dbg !10617
  %_40.1.i.i.i.i = load i64, ptr %871, align 8, !dbg !10617, !alias.scope !10600, !noalias !10599, !noundef !11
  %_222.i3.i.i.i.i = icmp eq i64 %_40.1.i.i.i.i, 0, !dbg !10618
  br i1 %_222.i3.i.i.i.i, label %_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i, label %bb14.preheader.i4.i.i.i.i, !dbg !10623

bb14.preheader.i4.i.i.i.i:                        ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i
  %872 = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !10617
  %_40.0.i.i.i.i = load ptr, ptr %872, align 8, !dbg !10617, !alias.scope !10600, !noalias !10599, !nonnull !11, !noundef !11
  %.idx.i5.i.i.i.i = shl nuw nsw i64 %_40.1.i.i.i.i, 2, !dbg !10624
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_40.0.i.i.i.i, i8 0, i64 %.idx.i5.i.i.i.i, i1 false), !dbg !10628, !alias.scope !10629, !noalias !10615
  br label %_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i, !dbg !10632

_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i: ; preds = %bb14.preheader.i4.i.i.i.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i
  %iter.sroa.0.0.ptr.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 192, !dbg !10633
  %873 = getelementptr inbounds nuw i8, ptr %self, i64 196, !dbg !10636
  %_38.i.i.i.i = load float, ptr %873, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.i.i.i.i, ptr %iter.sroa.0.0.ptr.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %874 = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !10639
  store float 0.000000e+00, ptr %874, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %875 = getelementptr inbounds nuw i8, ptr %self, i64 204, !dbg !10640
  store i32 0, ptr %875, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.1.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 208, !dbg !10633
  %876 = getelementptr inbounds nuw i8, ptr %self, i64 212, !dbg !10636
  %_38.1.i.i.i.i = load float, ptr %876, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.1.i.i.i.i, ptr %iter.sroa.0.0.ptr.1.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %877 = getelementptr inbounds nuw i8, ptr %self, i64 216, !dbg !10639
  store float 0.000000e+00, ptr %877, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %878 = getelementptr inbounds nuw i8, ptr %self, i64 220, !dbg !10640
  store i32 0, ptr %878, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.2.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 224, !dbg !10633
  %879 = getelementptr inbounds nuw i8, ptr %self, i64 228, !dbg !10636
  %_38.2.i.i.i.i = load float, ptr %879, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.2.i.i.i.i, ptr %iter.sroa.0.0.ptr.2.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %880 = getelementptr inbounds nuw i8, ptr %self, i64 232, !dbg !10639
  store float 0.000000e+00, ptr %880, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %881 = getelementptr inbounds nuw i8, ptr %self, i64 236, !dbg !10640
  store i32 0, ptr %881, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.3.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 240, !dbg !10633
  %882 = getelementptr inbounds nuw i8, ptr %self, i64 244, !dbg !10636
  %_38.3.i.i.i.i = load float, ptr %882, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.3.i.i.i.i, ptr %iter.sroa.0.0.ptr.3.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %883 = getelementptr inbounds nuw i8, ptr %self, i64 248, !dbg !10639
  store float 0.000000e+00, ptr %883, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %884 = getelementptr inbounds nuw i8, ptr %self, i64 252, !dbg !10640
  store i32 0, ptr %884, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.4.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !10633
  %885 = getelementptr inbounds nuw i8, ptr %self, i64 260, !dbg !10636
  %_38.4.i.i.i.i = load float, ptr %885, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.4.i.i.i.i, ptr %iter.sroa.0.0.ptr.4.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %886 = getelementptr inbounds nuw i8, ptr %self, i64 264, !dbg !10639
  store float 0.000000e+00, ptr %886, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %887 = getelementptr inbounds nuw i8, ptr %self, i64 268, !dbg !10640
  store i32 0, ptr %887, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.5.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 272, !dbg !10633
  %888 = getelementptr inbounds nuw i8, ptr %self, i64 276, !dbg !10636
  %_38.5.i.i.i.i = load float, ptr %888, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.5.i.i.i.i, ptr %iter.sroa.0.0.ptr.5.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %889 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !10639
  store float 0.000000e+00, ptr %889, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %890 = getelementptr inbounds nuw i8, ptr %self, i64 284, !dbg !10640
  store i32 0, ptr %890, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.6.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !10633
  %891 = getelementptr inbounds nuw i8, ptr %self, i64 292, !dbg !10636
  %_38.6.i.i.i.i = load float, ptr %891, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.6.i.i.i.i, ptr %iter.sroa.0.0.ptr.6.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %892 = getelementptr inbounds nuw i8, ptr %self, i64 296, !dbg !10639
  store float 0.000000e+00, ptr %892, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %893 = getelementptr inbounds nuw i8, ptr %self, i64 300, !dbg !10640
  store i32 0, ptr %893, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.7.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 304, !dbg !10633
  %894 = getelementptr inbounds nuw i8, ptr %self, i64 308, !dbg !10636
  %_38.7.i.i.i.i = load float, ptr %894, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.7.i.i.i.i, ptr %iter.sroa.0.0.ptr.7.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %895 = getelementptr inbounds nuw i8, ptr %self, i64 312, !dbg !10639
  store float 0.000000e+00, ptr %895, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %896 = getelementptr inbounds nuw i8, ptr %self, i64 316, !dbg !10640
  store i32 0, ptr %896, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.8.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !10633
  %897 = getelementptr inbounds nuw i8, ptr %self, i64 324, !dbg !10636
  %_38.8.i.i.i.i = load float, ptr %897, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.8.i.i.i.i, ptr %iter.sroa.0.0.ptr.8.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %898 = getelementptr inbounds nuw i8, ptr %self, i64 328, !dbg !10639
  store float 0.000000e+00, ptr %898, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %899 = getelementptr inbounds nuw i8, ptr %self, i64 332, !dbg !10640
  store i32 0, ptr %899, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  %iter.sroa.0.0.ptr.9.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 336, !dbg !10633
  %900 = getelementptr inbounds nuw i8, ptr %self, i64 340, !dbg !10636
  %_38.9.i.i.i.i = load float, ptr %900, align 4, !dbg !10636, !alias.scope !10600, !noalias !10599, !noundef !11
  store float %_38.9.i.i.i.i, ptr %iter.sroa.0.0.ptr.9.i.i.i.i, align 4, !dbg !10638, !alias.scope !10600, !noalias !10599
  %901 = getelementptr inbounds nuw i8, ptr %self, i64 344, !dbg !10639
  store float 0.000000e+00, ptr %901, align 4, !dbg !10639, !alias.scope !10600, !noalias !10599
  %902 = getelementptr inbounds nuw i8, ptr %self, i64 348, !dbg !10640
  store i32 0, ptr %902, align 4, !dbg !10640, !alias.scope !10600, !noalias !10599
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10641), !dbg !10592
  %903 = getelementptr inbounds nuw i8, ptr %self, i64 528, !dbg !10595
  %904 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !10597
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %903, i8 0, i64 24, i1 false), !dbg !10595, !alias.scope !10598, !noalias !10599
  %_39.1.i.1.i.i.i = load i64, ptr %904, align 8, !dbg !10597, !alias.scope !10643, !noalias !10599, !noundef !11
  %_222.i.i.1.i.i.i = icmp eq i64 %_39.1.i.1.i.i.i, 0, !dbg !10601
  br i1 %_222.i.i.1.i.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i, label %bb14.preheader.i.i.1.i.i.i, !dbg !10606

bb14.preheader.i.i.1.i.i.i:                       ; preds = %_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i
  %iter1.sroa.0.0.ptr.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !10644
  %_39.0.i.1.i.i.i = load ptr, ptr %iter1.sroa.0.0.ptr.1.i.i.i, align 8, !dbg !10597, !alias.scope !10643, !noalias !10599, !nonnull !11, !noundef !11
  %.idx.i.i.1.i.i.i = shl nuw nsw i64 %_39.1.i.1.i.i.i, 2, !dbg !10607
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_39.0.i.1.i.i.i, i8 0, i64 %.idx.i.i.1.i.i.i, i1 false), !dbg !10611, !alias.scope !10612, !noalias !10652
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i, !dbg !10616

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i: ; preds = %bb14.preheader.i.i.1.i.i.i, %_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i
  %905 = getelementptr inbounds nuw i8, ptr %self, i64 504, !dbg !10617
  %_40.1.i.1.i.i.i = load i64, ptr %905, align 8, !dbg !10617, !alias.scope !10643, !noalias !10599, !noundef !11
  %_222.i3.i.1.i.i.i = icmp eq i64 %_40.1.i.1.i.i.i, 0, !dbg !10618
  br i1 %_222.i3.i.1.i.i.i, label %bb6.i6932, label %bb14.preheader.i4.i.1.i.i.i, !dbg !10623

bb14.preheader.i4.i.1.i.i.i:                      ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i
  %906 = getelementptr inbounds nuw i8, ptr %self, i64 496, !dbg !10617
  %_40.0.i.1.i.i.i = load ptr, ptr %906, align 8, !dbg !10617, !alias.scope !10643, !noalias !10599, !nonnull !11, !noundef !11
  %.idx.i5.i.1.i.i.i = shl nuw nsw i64 %_40.1.i.1.i.i.i, 2, !dbg !10624
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_40.0.i.1.i.i.i, i8 0, i64 %.idx.i5.i.1.i.i.i, i1 false), !dbg !10628, !alias.scope !10629, !noalias !10652
  br label %bb6.i6932, !dbg !10632

bb6.i6932:                                        ; preds = %bb14.preheader.i4.i.1.i.i.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i
  %iter.sroa.0.0.ptr.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !10633
  %907 = getelementptr inbounds nuw i8, ptr %self, i64 556, !dbg !10636
  %_38.i.1.i.i.i = load float, ptr %907, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %908 = getelementptr inbounds nuw i8, ptr %self, i64 560, !dbg !10639
  store float 0.000000e+00, ptr %908, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %909 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !10640
  store i32 0, ptr %909, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.1.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 568, !dbg !10633
  %910 = getelementptr inbounds nuw i8, ptr %self, i64 572, !dbg !10636
  %_38.1.i.1.i.i.i = load float, ptr %910, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.1.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.1.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %911 = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !10639
  store float 0.000000e+00, ptr %911, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %912 = getelementptr inbounds nuw i8, ptr %self, i64 580, !dbg !10640
  store i32 0, ptr %912, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.2.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 584, !dbg !10633
  %913 = getelementptr inbounds nuw i8, ptr %self, i64 588, !dbg !10636
  %_38.2.i.1.i.i.i = load float, ptr %913, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.2.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.2.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %914 = getelementptr inbounds nuw i8, ptr %self, i64 592, !dbg !10639
  store float 0.000000e+00, ptr %914, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %915 = getelementptr inbounds nuw i8, ptr %self, i64 596, !dbg !10640
  store i32 0, ptr %915, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.3.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 600, !dbg !10633
  %916 = getelementptr inbounds nuw i8, ptr %self, i64 604, !dbg !10636
  %_38.3.i.1.i.i.i = load float, ptr %916, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.3.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.3.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %917 = getelementptr inbounds nuw i8, ptr %self, i64 608, !dbg !10639
  store float 0.000000e+00, ptr %917, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %918 = getelementptr inbounds nuw i8, ptr %self, i64 612, !dbg !10640
  store i32 0, ptr %918, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.4.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 616, !dbg !10633
  %919 = getelementptr inbounds nuw i8, ptr %self, i64 620, !dbg !10636
  %_38.4.i.1.i.i.i = load float, ptr %919, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.4.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.4.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %920 = getelementptr inbounds nuw i8, ptr %self, i64 624, !dbg !10639
  store float 0.000000e+00, ptr %920, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %921 = getelementptr inbounds nuw i8, ptr %self, i64 628, !dbg !10640
  store i32 0, ptr %921, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.5.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 632, !dbg !10633
  %922 = getelementptr inbounds nuw i8, ptr %self, i64 636, !dbg !10636
  %_38.5.i.1.i.i.i = load float, ptr %922, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.5.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.5.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %923 = getelementptr inbounds nuw i8, ptr %self, i64 640, !dbg !10639
  store float 0.000000e+00, ptr %923, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %924 = getelementptr inbounds nuw i8, ptr %self, i64 644, !dbg !10640
  store i32 0, ptr %924, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.6.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 648, !dbg !10633
  %925 = getelementptr inbounds nuw i8, ptr %self, i64 652, !dbg !10636
  %_38.6.i.1.i.i.i = load float, ptr %925, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.6.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.6.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %926 = getelementptr inbounds nuw i8, ptr %self, i64 656, !dbg !10639
  store float 0.000000e+00, ptr %926, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %927 = getelementptr inbounds nuw i8, ptr %self, i64 660, !dbg !10640
  store i32 0, ptr %927, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.7.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 664, !dbg !10633
  %928 = getelementptr inbounds nuw i8, ptr %self, i64 668, !dbg !10636
  %_38.7.i.1.i.i.i = load float, ptr %928, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.7.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.7.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %929 = getelementptr inbounds nuw i8, ptr %self, i64 672, !dbg !10639
  store float 0.000000e+00, ptr %929, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %930 = getelementptr inbounds nuw i8, ptr %self, i64 676, !dbg !10640
  store i32 0, ptr %930, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.8.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 680, !dbg !10633
  %931 = getelementptr inbounds nuw i8, ptr %self, i64 684, !dbg !10636
  %_38.8.i.1.i.i.i = load float, ptr %931, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.8.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.8.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %932 = getelementptr inbounds nuw i8, ptr %self, i64 688, !dbg !10639
  store float 0.000000e+00, ptr %932, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %933 = getelementptr inbounds nuw i8, ptr %self, i64 692, !dbg !10640
  store i32 0, ptr %933, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %iter.sroa.0.0.ptr.9.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 696, !dbg !10633
  %934 = getelementptr inbounds nuw i8, ptr %self, i64 700, !dbg !10636
  %_38.9.i.1.i.i.i = load float, ptr %934, align 4, !dbg !10636, !alias.scope !10643, !noalias !10599, !noundef !11
  store float %_38.9.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.9.i.1.i.i.i, align 4, !dbg !10638, !alias.scope !10643, !noalias !10599
  %935 = getelementptr inbounds nuw i8, ptr %self, i64 704, !dbg !10639
  store float 0.000000e+00, ptr %935, align 4, !dbg !10639, !alias.scope !10643, !noalias !10599
  %936 = getelementptr inbounds nuw i8, ptr %self, i64 708, !dbg !10640
  store i32 0, ptr %936, align 4, !dbg !10640, !alias.scope !10643, !noalias !10599
  %mask.i = load i32, ptr %863, align 8, !dbg !10653, !alias.scope !10401, !noalias !10655, !noundef !11
  %_21.i = and i32 %mask.i, 1, !dbg !10656
  %937 = icmp eq i32 %_21.i, 0, !dbg !10656
  %spec.select = select i1 %937, i64 0, i64 %_19.1, !dbg !10656
  br label %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E6finishB5_.exit, !dbg !10656

_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E6finishB5_.exit: ; preds = %bb6.i6932, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i
  %reports.sroa.6.0 = phi i64 [ 0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i ], [ %spec.select, %bb6.i6932 ], !dbg !10661
  %reports.sroa.0.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16, !dbg !10662
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, i8 0, i64 16, i1 false), !dbg !10662
  store i64 %report.sroa.7.2, ptr %reports.sroa.0.sroa.5.0._0.sroa_idx, align 8, !dbg !10662
  %reports.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24, !dbg !10662
  store i64 %reports.sroa.6.0, ptr %reports.sroa.4.0._0.sroa_idx, align 8, !dbg !10662
  %reports.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32, !dbg !10662
  store i64 %reports.sroa.6.0, ptr %reports.sroa.6.0._0.sroa_idx, align 8, !dbg !10662
  br label %bb7, !dbg !3808

bb7:                                              ; preds = %bb4, %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E6finishB5_.exit
  ret void, !dbg !3808
}
