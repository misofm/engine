define void @_RNvXs6_Cs5xZ4lC6BZIV_20multiband_compressorNtB5_27PreparedMultibandCompressorNtCsfGpaX3jkkqY_15effect_contract20PreparedNativeEffect7process(ptr dead_on_unwind noalias noundef writable writeonly sret([40 x i8]) align 8 captures(none) dereferenceable(40) %_0, ptr noalias noundef align 8 dereferenceable(872) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(88) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !3624 {
start:
  %pending.i = alloca [160 x i8], align 4
  %_22.i527.i = alloca [24 x i8], align 4
  %_20.i528.i = alloca [24 x i8], align 4
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
  %iter.sroa.0.0.ph126.i = phi ptr [ %_18.0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i ], [ %_16.i.i.i, %bb31.i ]
  %iter.sroa.7.0.ph125.i = phi i64 [ 0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.lr.ph.i ], [ %_9.0.i.i, %bb31.i ]
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i, !dbg !3663

bb36.preheader.i:                                 ; preds = %bb31.i, %bb35.i, %start
  %report.sroa.7.2 = phi i64 [ 0, %start ], [ %113, %bb35.i ], [ %report.sroa.7.1, %bb31.i ], !dbg !3668
  br label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterINtNtBa_6option6OptionfEEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.us.preheader.i, !dbg !3669

_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i: ; preds = %bb35.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i
  %report.sroa.7.1 = phi i64 [ %report.sroa.7.0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i ], [ %113, %bb35.i ], !dbg !3668
  %.promoted130.i = phi i64 [ %.promoted131.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i ], [ %113, %bb35.i ]
  %iter.sroa.0.0123.i = phi ptr [ %iter.sroa.0.0.ph126.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i ], [ %_16.i.i.i, %bb35.i ]
  %iter.sroa.7.0122.i = phi i64 [ %iter.sroa.7.0.ph125.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i ], [ %_9.0.i.i, %bb35.i ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 40, !dbg !3677
  %_9.0.i.i = add i64 %iter.sroa.7.0122.i, 1, !dbg !3680
  %5 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 32, !dbg !3683
  %_19.i = load i32, ptr %5, align 8, !dbg !3683, !range !2567, !alias.scope !3634, !noalias !3685, !noundef !11
  switch i32 %_19.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i.unreachabledefault [
    i32 1, label %bb9.i6501
    i32 2, label %bb7.i6500
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

bb7.i6500:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i
  br label %bb9.i6501, !dbg !3732

bb9.i6501:                                        ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i, %bb7.i6500
  %side.sroa.0.0.sroa.phi35.i = phi ptr [ %4, %bb7.i6500 ], [ %pending.i, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i ], !dbg !3733
  %side.sroa.0.0.i = phi i32 [ 1, %bb7.i6500 ], [ 0, %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i ], !dbg !3733
  %98 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 16, !dbg !3734
  %_23.i = load i32, ptr %98, align 8, !dbg !3734, !alias.scope !3634, !noalias !3685, !noundef !11
  %99 = zext i32 %_23.i to i64, !dbg !3734
  %_106.0.i = shl nuw i32 %_23.i, 1, !dbg !3736
  %_112.0.i = or disjoint i32 %_106.0.i, %side.sroa.0.0.i, !dbg !3736
  %_115.i = icmp ult i32 %_23.i, 2, !dbg !3746
  br i1 %_115.i, label %bb60.i, label %bb59.split.i, !dbg !3746

bb59.split.i:                                     ; preds = %bb9.i6501
  %_116.i = add nsw i64 %99, -2, !dbg !3750
  %100 = icmp ult i32 %_23.i, 12, !dbg !3751
  %_0.sroa.0.0.i74.i = zext i1 %100 to i64, !dbg !3751
  %101 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0.i74.i, 0, !dbg !3755
  %102 = insertvalue { i64, i64 } %101, i64 %_116.i, 1, !dbg !3755
  br label %bb60.i, !dbg !3756

bb60.i:                                           ; preds = %bb59.split.i, %bb9.i6501
  %phi.call.i = phi { i64, i64 } [ %102, %bb59.split.i ], [ { i64 0, i64 undef }, %bb9.i6501 ], !dbg !3757
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
  br i1 %107, label %bb13.i6502, label %bb35.i, !dbg !3765

bb13.i6502:                                       ; preds = %bb12.i
  %_35.i = load i64, ptr %iter.sroa.0.0123.i, align 8, !dbg !3766, !alias.scope !3634, !noalias !3685, !noundef !11
  %_34.i = icmp eq i64 %_35.i, %_7, !dbg !3766
  br i1 %_34.i, label %bb14.i6503, label %bb35.i, !dbg !3766

bb14.i6503:                                       ; preds = %bb13.i6502
  %108 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0123.i, i64 8, !dbg !3767
  %_37.i = load i64, ptr %108, align 8, !dbg !3767, !alias.scope !3634, !noalias !3685, !noundef !11
  %_36.i = icmp eq i64 %_37.i, %_7, !dbg !3767
  br i1 %_36.i, label %bb15.i, label %bb35.i, !dbg !3767

bb15.i:                                           ; preds = %bb14.i6503
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
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %99, i64 noundef 12, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_618452d81e57e47d125c0afad7fc5006) #26, !dbg !3772, !noalias !3639
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
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %104, i64 noundef 10, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_97e1583d528f48ca4578bc0e35112ce6) #26, !dbg !3775, !noalias !3639
  unreachable, !dbg !3775

bb31.i:                                           ; preds = %bb68.i
  %_125.i.le = getelementptr inbounds nuw %"core::option::Option<f32>", ptr %side.sroa.0.0.sroa.phi35.i, i64 %104
  %_131.i = fcmp oeq float %_40.i, 0.000000e+00, !dbg !3795
  %_57.sroa.0.0.i = select i1 %_131.i, float 0.000000e+00, float %_40.i, !dbg !3795
  store i32 1, ptr %_125.i.le, align 4, !dbg !3798, !noalias !3639
  %112 = getelementptr inbounds nuw i8, ptr %_125.i.le, i64 4, !dbg !3798
  store float %_57.sroa.0.0.i, ptr %112, align 4, !dbg !3798, !noalias !3639
  %_6.i.i121.i = icmp eq ptr %_16.i.i.i, %_91.i, !dbg !3653
  br i1 %_6.i.i121.i, label %bb36.preheader.i, label %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.lr.ph.i, !dbg !3663

bb35.i:                                           ; preds = %_RNvXs_NtNtNtCs4NRVxsYgnAr_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_5slice4iter4IterNtCsfGpaX3jkkqY_15effect_contract22PreparedAutomationSpanEENtNtNtB8_6traits8iterator8Iterator4nextCs5xZ4lC6BZIV_20multiband_compressor.exit.i, %bb68.i, %bb17.i, %bb15.i, %bb14.i6503, %bb13.i6502, %bb12.i, %bb60.i
  %113 = tail call i64 @llvm.uadd.sat.i64(i64 %.promoted130.i, i64 1), !dbg !3799
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_91.i, !dbg !3653
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
  br i1 %_7.i, label %bb2.i19.i.lr.ph, label %bb2.i538.i.lr.ph, !dbg !3823

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
  %.sroa_idx7025 = getelementptr inbounds nuw i8, ptr %self, i64 188
  %167 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %.sroa_idx7030 = getelementptr inbounds nuw i8, ptr %self, i64 548
  %_24.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 400
  %_26.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %_67.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 152
  %168 = getelementptr inbounds nuw i8, ptr %self, i64 156
  %169 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %170 = getelementptr inbounds nuw i8, ptr %self, i64 164
  %_72.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %171 = getelementptr inbounds nuw i8, ptr %self, i64 516
  %172 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %173 = getelementptr inbounds nuw i8, ptr %self, i64 524
  %174 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %175 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %176 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %177 = getelementptr inbounds nuw i8, ptr %self, i64 488
  %178 = getelementptr inbounds nuw i8, ptr %self, i64 496
  %179 = getelementptr inbounds nuw i8, ptr %self, i64 504
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
  %position.sroa.0.0.i.i11245 = phi i64 [ 0, %bb2.i.i.lr.ph ], [ %_32.i.i, %bb15.i.i ]
  %_11.i.i = sub nuw i64 %_19.1, %position.sroa.0.0.i.i11245, !dbg !3840
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
  %_12.le.i6504 = load float, ptr %144, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.i6505 = load float, ptr %145, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.1.i6506 = load float, ptr %146, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.1.i6507 = load float, ptr %147, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.2.i6508 = load float, ptr %148, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.2.i6509 = load float, ptr %149, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.3.i6510 = load float, ptr %150, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.3.i6511 = load float, ptr %151, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.4.i6512 = load float, ptr %152, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.4.i6513 = load float, ptr %153, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.5.i6514 = load float, ptr %154, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.5.i6515 = load float, ptr %155, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.6.i6516 = load float, ptr %156, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.6.i6517 = load float, ptr %157, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.7.i6518 = load float, ptr %158, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.7.i6519 = load float, ptr %159, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.8.i6520 = load float, ptr %160, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.8.i6521 = load float, ptr %161, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_12.le.9.i6522 = load float, ptr %162, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
  %_16.le.9.i6523 = load float, ptr %163, align 8, !alias.scope !3848, !noalias !3851, !noundef !11
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
  %_32.i.i = add i64 %plan.0.i.i, %position.sroa.0.0.i.i11245, !dbg !3863
  %_72.i.i = icmp ult i64 %_32.i.i, %position.sroa.0.0.i.i11245, !dbg !3865
  %_66.not.i.i = icmp ugt i64 %_32.i.i, %_19.1
  %or.cond11.i.i = or i1 %_72.i.i, %_66.not.i.i, !dbg !3865
  br i1 %plan.1.i.i, label %bb9.i.i, label %bb13.i.i, !dbg !3873

bb9.i.i:                                          ; preds = %bb2.i.i
  br i1 %or.cond11.i.i, label %bb19.i.i, label %bb17.i.i, !dbg !3874, !prof !3878

bb13.i.i:                                         ; preds = %bb2.i.i
  br i1 %or.cond11.i.i, label %bb29.i.i, label %bb27.i.i, !dbg !3879, !prof !3878

bb27.i.i:                                         ; preds = %bb13.i.i
  %_95.i.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i.i11245, !dbg !3885
  %_105.i.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i.i11245, !dbg !3889
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
  %202 = load i32, ptr %.sroa_idx7025, align 4, !dbg !3913
  %203 = load i32, ptr %167, align 8, !dbg !3915
  %204 = load i32, ptr %.sroa_idx7030, align 4, !dbg !3915
  %205 = load i64, ptr %_51.i.i, align 8, !dbg !3917, !alias.scope !3919, !noalias !3920, !noundef !11
  %_168.i.i.i11187.not = icmp eq i64 %plan.0.i.i, 0, !dbg !3922
  br i1 %_168.i.i.i11187.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_KBZ_EB2_.exit.i.i, label %bb56.i.i.i, !dbg !3934

bb29.i.i:                                         ; preds = %bb13.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i11245, i64 noundef %_32.i.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ac5156198d2516c0e2a17140923b083) #26, !dbg !3935, !noalias !3842
  unreachable, !dbg !3935

bb56.i.i.i:                                       ; preds = %bb27.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021
  %iter.sroa.0.0.i.i.i11201 = phi i64 [ %206, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ 0, %bb27.i.i ]
  %position.sroa.0.0.i.i.i11200 = phi i64 [ %_41.sroa.0.0.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %205, %bb27.i.i ]
  %gain_far.i.i.i.sroa.6.011199 = phi i32 [ %_3.i4379, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %204, %bb27.i.i ]
  %gain_far.i.i.i.sroa.0.011198 = phi i32 [ %_3.i4383, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %203, %bb27.i.i ]
  %gain_near.i.i.i.sroa.6.011197 = phi i32 [ %_3.i4387, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %202, %bb27.i.i ]
  %gain_near.i.i.i.sroa.0.011196 = phi i32 [ %_3.i4391, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %201, %bb27.i.i ]
  %filter_far.i.i.i.sroa.14.011195 = phi float [ %_0.i4212, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %filter_far.i.i.i.sroa.14.0.copyload, %bb27.i.i ]
  %filter_far.i.i.i.sroa.11.011194 = phi float [ %_0.i4216, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %filter_far.i.i.i.sroa.11.0.copyload, %bb27.i.i ]
  %filter_far.i.i.i.sroa.7.011193 = phi float [ %_0.i4204, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %filter_far.i.i.i.sroa.7.0.copyload, %bb27.i.i ]
  %filter_far.i.i.i.sroa.0.011192 = phi float [ %_0.i4208, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %filter_far.i.i.i.sroa.0.0.copyload, %bb27.i.i ]
  %filter_near.i.i.i.sroa.0.011191 = phi float [ %_0.i4224, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %filter_near.i.i.i.sroa.0.0.copyload, %bb27.i.i ]
  %filter_near.i.i.i.sroa.7.011190 = phi float [ %_0.i4220, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %filter_near.i.i.i.sroa.7.0.copyload, %bb27.i.i ]
  %filter_near.i.i.i.sroa.11.011189 = phi float [ %_0.i4232, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %filter_near.i.i.i.sroa.11.0.copyload, %bb27.i.i ]
  %filter_near.i.i.i.sroa.14.011188 = phi float [ %_0.i4228, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], [ %filter_near.i.i.i.sroa.14.0.copyload, %bb27.i.i ]
  %206 = add nuw i64 %iter.sroa.0.0.i.i.i11201, 1, !dbg !3936
  %_42.i.i.i = add i64 %position.sroa.0.0.i.i.i11200, 1, !dbg !3942
  %_176.not.i.i.i = icmp ult i64 %_42.i.i.i, %ring_len.i.i, !dbg !3945
  %207 = select i1 %_176.not.i.i.i, i64 0, i64 %ring_len.i.i, !dbg !3945
  %_41.sroa.0.0.i.i.i = sub nuw i64 %_42.i.i.i, %207, !dbg !3945
  %_184.i.i.i = getelementptr inbounds nuw float, ptr %_95.i.i, i64 %iter.sroa.0.0.i.i.i11201, !dbg !3948
  %_0.i3564 = load float, ptr %_184.i.i.i, align 4, !dbg !3960, !alias.scope !3963, !noalias !3966, !noundef !11
  %_192.i.i.i = getelementptr inbounds nuw float, ptr %_105.i.i, i64 %iter.sroa.0.0.i.i.i11201, !dbg !3967
  %_0.i3559 = load float, ptr %_192.i.i.i, align 4, !dbg !3976, !alias.scope !3978, !noalias !3966, !noundef !11
  %_7.i28 = load float, ptr %_67.i.i.i, align 4, !dbg !3981, !alias.scope !3985, !noalias !3988, !noundef !11
  %_8.i29 = load float, ptr %168, align 4, !dbg !3990, !alias.scope !3985, !noalias !3988, !noundef !11
  %_9.i30 = load float, ptr %169, align 4, !dbg !3991, !alias.scope !3985, !noalias !3988, !noundef !11
  %_0.i3415 = fsub float %_0.i3564, %filter_near.i.i.i.sroa.7.011190, !dbg !3992
  %_0.i3230 = fmul float %_0.i3415, %_8.i29, !dbg !3999
  %_4.i2851 = fmul float %filter_near.i.i.i.sroa.0.011191, %_7.i28, !dbg !4003
  %_0.i2852 = fadd float %_4.i2851, %_0.i3230, !dbg !4003
  %_0.i2678 = fadd float %filter_near.i.i.i.sroa.0.011191, %_0.i2852, !dbg !4006
  %_0.i3229 = fmul float %filter_near.i.i.i.sroa.0.011191, %_8.i29, !dbg !4010
  %_4.i2849 = fmul float %_0.i3415, %_9.i30, !dbg !4013
  %_0.i2850 = fadd float %_0.i3229, %_4.i2849, !dbg !4013
  %_0.i2677 = fadd float %filter_near.i.i.i.sroa.7.011190, %_0.i2850, !dbg !4015
  %_0.i2676 = fadd float %_0.i2852, %_0.i2852, !dbg !4018
  %_0.i2675 = fadd float %filter_near.i.i.i.sroa.0.011191, %_0.i2676, !dbg !4021
  %208 = tail call noundef float @llvm.fabs.f32(float %_0.i2675), !dbg !4023
  %209 = fcmp uge float %208, 0x3BC79CA100000000, !dbg !4029
  %_0.i4224 = select i1 %209, float %_0.i2675, float 0.000000e+00, !dbg !4032
  %_0.i2674 = fadd float %_0.i2850, %_0.i2850, !dbg !4033
  %_0.i2673 = fadd float %filter_near.i.i.i.sroa.7.011190, %_0.i2674, !dbg !4035
  %210 = tail call noundef float @llvm.fabs.f32(float %_0.i2673), !dbg !4037
  %211 = fcmp uge float %210, 0x3BC79CA100000000, !dbg !4040
  %_0.i4220 = select i1 %211, float %_0.i2673, float 0.000000e+00, !dbg !4042
  %_12.i33 = load float, ptr %170, align 4, !dbg !4043, !alias.scope !3985, !noalias !3988, !noundef !11
  %_4.i2937 = fmul float %_12.i33, %_0.i2678, !dbg !4045
  %_0.i2938 = fadd float %_0.i3564, %_4.i2937, !dbg !4045
  %_0.i3416 = fsub float %_0.i2677, %filter_near.i.i.i.sroa.14.011188, !dbg !4047
  %_0.i3232 = fmul float %_8.i29, %_0.i3416, !dbg !4051
  %_4.i2855 = fmul float %filter_near.i.i.i.sroa.11.011189, %_7.i28, !dbg !4053
  %_0.i2856 = fadd float %_4.i2855, %_0.i3232, !dbg !4053
  %_0.i3231 = fmul float %filter_near.i.i.i.sroa.11.011189, %_8.i29, !dbg !4055
  %_4.i2853 = fmul float %_9.i30, %_0.i3416, !dbg !4057
  %_0.i2854 = fadd float %_0.i3231, %_4.i2853, !dbg !4057
  %_0.i2683 = fadd float %filter_near.i.i.i.sroa.14.011188, %_0.i2854, !dbg !4059
  %_0.i2682 = fadd float %_0.i2856, %_0.i2856, !dbg !4061
  %_0.i2681 = fadd float %filter_near.i.i.i.sroa.11.011189, %_0.i2682, !dbg !4063
  %212 = tail call noundef float @llvm.fabs.f32(float %_0.i2681), !dbg !4065
  %213 = fcmp uge float %212, 0x3BC79CA100000000, !dbg !4068
  %_0.i4232 = select i1 %213, float %_0.i2681, float 0.000000e+00, !dbg !4070
  %_0.i2680 = fadd float %_0.i2854, %_0.i2854, !dbg !4071
  %_0.i2679 = fadd float %filter_near.i.i.i.sroa.14.011188, %_0.i2680, !dbg !4073
  %214 = tail call noundef float @llvm.fabs.f32(float %_0.i2679), !dbg !4075
  %215 = fcmp uge float %214, 0x3BC79CA100000000, !dbg !4078
  %_0.i4228 = select i1 %215, float %_0.i2679, float 0.000000e+00, !dbg !4080
  %_0.i3441 = fsub float %_0.i2938, %_0.i2683, !dbg !4081
  %_7.i15 = load float, ptr %_72.i.i.i, align 4, !dbg !4084, !alias.scope !4087, !noalias !4090, !noundef !11
  %_8.i16 = load float, ptr %171, align 4, !dbg !4092, !alias.scope !4087, !noalias !4090, !noundef !11
  %_9.i17 = load float, ptr %172, align 4, !dbg !4093, !alias.scope !4087, !noalias !4090, !noundef !11
  %_0.i3413 = fsub float %_0.i3559, %filter_far.i.i.i.sroa.7.011193, !dbg !4094
  %_0.i3226 = fmul float %_0.i3413, %_8.i16, !dbg !4097
  %_4.i2843 = fmul float %filter_far.i.i.i.sroa.0.011192, %_7.i15, !dbg !4099
  %_0.i2844 = fadd float %_4.i2843, %_0.i3226, !dbg !4099
  %_0.i2666 = fadd float %filter_far.i.i.i.sroa.0.011192, %_0.i2844, !dbg !4101
  %_0.i3225 = fmul float %filter_far.i.i.i.sroa.0.011192, %_8.i16, !dbg !4103
  %_4.i2841 = fmul float %_0.i3413, %_9.i17, !dbg !4105
  %_0.i2842 = fadd float %_0.i3225, %_4.i2841, !dbg !4105
  %_0.i2665 = fadd float %filter_far.i.i.i.sroa.7.011193, %_0.i2842, !dbg !4107
  %_0.i2664 = fadd float %_0.i2844, %_0.i2844, !dbg !4109
  %_0.i2663 = fadd float %filter_far.i.i.i.sroa.0.011192, %_0.i2664, !dbg !4111
  %216 = tail call noundef float @llvm.fabs.f32(float %_0.i2663), !dbg !4113
  %217 = fcmp uge float %216, 0x3BC79CA100000000, !dbg !4116
  %_0.i4208 = select i1 %217, float %_0.i2663, float 0.000000e+00, !dbg !4118
  %_0.i2662 = fadd float %_0.i2842, %_0.i2842, !dbg !4119
  %_0.i2661 = fadd float %filter_far.i.i.i.sroa.7.011193, %_0.i2662, !dbg !4121
  %218 = tail call noundef float @llvm.fabs.f32(float %_0.i2661), !dbg !4123
  %219 = fcmp uge float %218, 0x3BC79CA100000000, !dbg !4126
  %_0.i4204 = select i1 %219, float %_0.i2661, float 0.000000e+00, !dbg !4128
  %_12.i20 = load float, ptr %173, align 4, !dbg !4129, !alias.scope !4087, !noalias !4090, !noundef !11
  %_4.i2939 = fmul float %_12.i20, %_0.i2666, !dbg !4130
  %_0.i2940 = fadd float %_0.i3559, %_4.i2939, !dbg !4130
  %_0.i3414 = fsub float %_0.i2665, %filter_far.i.i.i.sroa.14.011195, !dbg !4132
  %_0.i3228 = fmul float %_8.i16, %_0.i3414, !dbg !4135
  %_4.i2847 = fmul float %filter_far.i.i.i.sroa.11.011194, %_7.i15, !dbg !4137
  %_0.i2848 = fadd float %_4.i2847, %_0.i3228, !dbg !4137
  %_0.i3227 = fmul float %filter_far.i.i.i.sroa.11.011194, %_8.i16, !dbg !4139
  %_4.i2845 = fmul float %_9.i17, %_0.i3414, !dbg !4141
  %_0.i2846 = fadd float %_0.i3227, %_4.i2845, !dbg !4141
  %_0.i2671 = fadd float %filter_far.i.i.i.sroa.14.011195, %_0.i2846, !dbg !4143
  %_0.i2670 = fadd float %_0.i2848, %_0.i2848, !dbg !4145
  %_0.i2669 = fadd float %filter_far.i.i.i.sroa.11.011194, %_0.i2670, !dbg !4147
  %220 = tail call noundef float @llvm.fabs.f32(float %_0.i2669), !dbg !4149
  %221 = fcmp uge float %220, 0x3BC79CA100000000, !dbg !4152
  %_0.i4216 = select i1 %221, float %_0.i2669, float 0.000000e+00, !dbg !4154
  %_0.i2668 = fadd float %_0.i2846, %_0.i2846, !dbg !4155
  %_0.i2667 = fadd float %filter_far.i.i.i.sroa.14.011195, %_0.i2668, !dbg !4157
  %222 = tail call noundef float @llvm.fabs.f32(float %_0.i2667), !dbg !4159
  %223 = fcmp uge float %222, 0x3BC79CA100000000, !dbg !4162
  %_0.i4212 = select i1 %223, float %_0.i2667, float 0.000000e+00, !dbg !4164
  %_0.i3442 = fsub float %_0.i2940, %_0.i2671, !dbg !4165
  %_311.1.i.i.i = load i64, ptr %174, align 8, !dbg !4167, !noalias !3966, !noundef !11
  %_234.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i.i11200, %_311.1.i.i.i, !dbg !4169
  br i1 %_234.i.i.i, label %bb78.i.i.i, label %bb79.i.i.i, !dbg !4169, !prof !1664

bb79.i.i.i:                                       ; preds = %bb56.i.i.i
  %_311.0.i.i.i = load ptr, ptr %123, align 8, !dbg !4167, !noalias !3966, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4175), !dbg !4178
  %_4.not.i4038 = icmp eq i64 %_311.1.i.i.i, %position.sroa.0.0.i.i.i11200, !dbg !4179
  br i1 %_4.not.i4038, label %panic.i4040, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4041, !dbg !4179

panic.i4040:                                      ; preds = %bb79.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !4179, !noalias !4182
  unreachable, !dbg !4179

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4041: ; preds = %bb79.i.i.i
  %_241.i.i.i = getelementptr inbounds nuw float, ptr %_311.0.i.i.i, i64 %position.sroa.0.0.i.i.i11200, !dbg !4183
  store float %_0.i2683, ptr %_241.i.i.i, align 4, !dbg !4179, !alias.scope !4175, !noalias !3966
  %_312.1.i.i.i = load i64, ptr %176, align 8, !dbg !4189, !noalias !3966, !noundef !11
  %_242.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i.i11200, %_312.1.i.i.i, !dbg !4190
  br i1 %_242.i.i.i, label %bb80.i.i.i, label %bb81.i.i.i, !dbg !4190, !prof !1664

bb78.i.i.i:                                       ; preds = %bb56.i.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i.i11200, i64 noundef %_311.1.i.i.i, i64 noundef %_311.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7515f6adc8a5521b72f04a3abb372e72) #26, !dbg !4194, !noalias !3966
  unreachable, !dbg !4194

bb81.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4041
  %_312.0.i.i.i = load ptr, ptr %175, align 8, !dbg !4189, !noalias !3966, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4195), !dbg !4198
  %_4.not.i4034 = icmp eq i64 %_312.1.i.i.i, %position.sroa.0.0.i.i.i11200, !dbg !4199
  br i1 %_4.not.i4034, label %panic.i4036, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037, !dbg !4199

panic.i4036:                                      ; preds = %bb81.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !4199, !noalias !4201
  unreachable, !dbg !4199

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037: ; preds = %bb81.i.i.i
  %_249.i.i.i = getelementptr inbounds nuw float, ptr %_312.0.i.i.i, i64 %position.sroa.0.0.i.i.i11200, !dbg !4202
  store float %_0.i3441, ptr %_249.i.i.i, align 4, !dbg !4199, !alias.scope !4195, !noalias !3966
  %_313.1.i.i.i = load i64, ptr %177, align 8, !dbg !4207, !noalias !3966, !noundef !11
  %_250.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i.i11200, %_313.1.i.i.i, !dbg !4208
  br i1 %_250.i.i.i, label %bb82.i.i.i, label %bb83.i.i.i, !dbg !4208, !prof !1664

bb80.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4041
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i.i11200, i64 noundef %_312.1.i.i.i, i64 noundef %_312.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8c2aade3368450b16e7e4997ad3fca81) #26, !dbg !4212, !noalias !3966
  unreachable, !dbg !4212

bb83.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037
  %_313.0.i.i.i = load ptr, ptr %_18.i.i, align 8, !dbg !4207, !noalias !3966, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4213), !dbg !4216
  %_4.not.i4030 = icmp eq i64 %_313.1.i.i.i, %position.sroa.0.0.i.i.i11200, !dbg !4217
  br i1 %_4.not.i4030, label %panic.i4032, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4033, !dbg !4217

panic.i4032:                                      ; preds = %bb83.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !4217, !noalias !4219
  unreachable, !dbg !4217

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4033: ; preds = %bb83.i.i.i
  %_257.i.i.i = getelementptr inbounds nuw float, ptr %_313.0.i.i.i, i64 %position.sroa.0.0.i.i.i11200, !dbg !4220
  store float %_0.i2671, ptr %_257.i.i.i, align 4, !dbg !4217, !alias.scope !4213, !noalias !3966
  %_314.1.i.i.i = load i64, ptr %179, align 8, !dbg !4225, !noalias !3966, !noundef !11
  %_258.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i.i11200, %_314.1.i.i.i, !dbg !4226
  br i1 %_258.i.i.i, label %bb84.i.i.i, label %bb85.i.i.i, !dbg !4226, !prof !1664

bb82.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4037
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i.i11200, i64 noundef %_313.1.i.i.i, i64 noundef %_313.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a4c3da5a99763e9452b49d42852d67e3) #26, !dbg !4230, !noalias !3966
  unreachable, !dbg !4230

bb85.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4033
  %_314.0.i.i.i = load ptr, ptr %178, align 8, !dbg !4225, !noalias !3966, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4231), !dbg !4234
  %_4.not.i4026 = icmp eq i64 %_314.1.i.i.i, %position.sroa.0.0.i.i.i11200, !dbg !4235
  br i1 %_4.not.i4026, label %panic.i4028, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4029, !dbg !4235

panic.i4028:                                      ; preds = %bb85.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !4235, !noalias !4237
  unreachable, !dbg !4235

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4029: ; preds = %bb85.i.i.i
  %_265.i.i.i = getelementptr inbounds nuw float, ptr %_314.0.i.i.i, i64 %position.sroa.0.0.i.i.i11200, !dbg !4238
  store float %_0.i3442, ptr %_265.i.i.i, align 4, !dbg !4235, !alias.scope !4231, !noalias !3966
  %_315.0.i.i.i = load ptr, ptr %123, align 8, !dbg !4243, !noalias !3966, !nonnull !11, !noundef !11
  %_315.1.i.i.i = load i64, ptr %174, align 8, !dbg !4243, !noalias !3966, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4244), !dbg !4247
  %_8.i212.i.i = load i64, ptr %_24.i.i.i, align 8, !dbg !4248, !alias.scope !4244, !noalias !4251, !noundef !11
  %_7.i213.i.i = add i64 %_8.i212.i.i, %position.sroa.0.0.i.i.i11200, !dbg !4253
  %_27.not.i214.i.i = icmp ult i64 %_7.i213.i.i, %ring_len.i.i, !dbg !4254
  %224 = select i1 %_27.not.i214.i.i, i64 0, i64 %ring_len.i.i, !dbg !4254
  %row.sroa.0.0.i215.i.i = sub nuw i64 %_7.i213.i.i, %224, !dbg !4254
  %_28.i216.i.i = icmp ugt i64 %row.sroa.0.0.i215.i.i, %_315.1.i.i.i, !dbg !4257
  br i1 %_28.i216.i.i, label %bb14.i219.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i.i, !dbg !4257, !prof !1664

bb14.i219.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4029
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i215.i.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !4263, !noalias !4264
  unreachable, !dbg !4263

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4029
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4265), !dbg !4268
  %_3.not.i3552 = icmp eq i64 %_315.1.i.i.i, %row.sroa.0.0.i215.i.i, !dbg !4269
  br i1 %_3.not.i3552, label %panic.i3555, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3556, !dbg !4269

panic.i3555:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4269, !noalias !4271
  unreachable, !dbg !4269

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3556: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i.i
  %_35.i218.i.i = getelementptr inbounds nuw float, ptr %_315.0.i.i.i, i64 %row.sroa.0.0.i215.i.i, !dbg !4272
  %_0.i3554 = load float, ptr %_35.i218.i.i, align 4, !dbg !4269, !alias.scope !4265, !noalias !4277, !noundef !11
  %_316.0.i.i.i = load ptr, ptr %175, align 8, !dbg !4278, !noalias !3966, !nonnull !11, !noundef !11
  %_316.1.i.i.i = load i64, ptr %176, align 8, !dbg !4278, !noalias !3966, !noundef !11
  %_28.i207.i.i = icmp ugt i64 %row.sroa.0.0.i215.i.i, %_316.1.i.i.i, !dbg !4280
  br i1 %_28.i207.i.i, label %bb14.i210.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i.i, !dbg !4280, !prof !1664

bb14.i210.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3556
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i215.i.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !4284, !noalias !4285
  unreachable, !dbg !4284

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3556
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4289), !dbg !4292
  %_3.not.i3547 = icmp eq i64 %_316.1.i.i.i, %row.sroa.0.0.i215.i.i, !dbg !4293
  br i1 %_3.not.i3547, label %panic.i3550, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3551, !dbg !4293

panic.i3550:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4293, !noalias !4295
  unreachable, !dbg !4293

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3551: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i.i
  %_35.i209.i.i = getelementptr inbounds nuw float, ptr %_316.0.i.i.i, i64 %row.sroa.0.0.i215.i.i, !dbg !4296
  %_0.i3549 = load float, ptr %_35.i209.i.i, align 4, !dbg !4293, !alias.scope !4289, !noalias !4298, !noundef !11
  %_317.0.i.i.i = load ptr, ptr %_18.i.i, align 8, !dbg !4299, !noalias !3966, !nonnull !11, !noundef !11
  %_317.1.i.i.i = load i64, ptr %177, align 8, !dbg !4299, !noalias !3966, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4301), !dbg !4304
  %_8.i194.i.i = load i64, ptr %_26.i.i.i, align 8, !dbg !4305, !alias.scope !4301, !noalias !4307, !noundef !11
  %_7.i195.i.i = add i64 %_8.i194.i.i, %position.sroa.0.0.i.i.i11200, !dbg !4309
  %_27.not.i196.i.i = icmp ult i64 %_7.i195.i.i, %ring_len.i.i, !dbg !4310
  %225 = select i1 %_27.not.i196.i.i, i64 0, i64 %ring_len.i.i, !dbg !4310
  %row.sroa.0.0.i197.i.i = sub nuw i64 %_7.i195.i.i, %225, !dbg !4310
  %_28.i198.i.i = icmp ugt i64 %row.sroa.0.0.i197.i.i, %_317.1.i.i.i, !dbg !4312
  br i1 %_28.i198.i.i, label %bb14.i201.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i.i, !dbg !4312, !prof !1664

bb14.i201.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3551
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i197.i.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !4315, !noalias !4316
  unreachable, !dbg !4315

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3551
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4317), !dbg !4320
  %_3.not.i3542 = icmp eq i64 %_317.1.i.i.i, %row.sroa.0.0.i197.i.i, !dbg !4321
  br i1 %_3.not.i3542, label %panic.i3545, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3546, !dbg !4321

panic.i3545:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4321, !noalias !4323
  unreachable, !dbg !4321

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3546: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i.i
  %_35.i200.i.i = getelementptr inbounds nuw float, ptr %_317.0.i.i.i, i64 %row.sroa.0.0.i197.i.i, !dbg !4324
  %_0.i3544 = load float, ptr %_35.i200.i.i, align 4, !dbg !4321, !alias.scope !4317, !noalias !4326, !noundef !11
  %_318.0.i.i.i = load ptr, ptr %178, align 8, !dbg !4327, !noalias !3966, !nonnull !11, !noundef !11
  %_318.1.i.i.i = load i64, ptr %179, align 8, !dbg !4327, !noalias !3966, !noundef !11
  %_28.i189.i.i = icmp ugt i64 %row.sroa.0.0.i197.i.i, %_318.1.i.i.i, !dbg !4329
  br i1 %_28.i189.i.i, label %bb14.i192.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i.i, !dbg !4329, !prof !1664

bb14.i192.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3546
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i197.i.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !4333, !noalias !4334
  unreachable, !dbg !4333

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3546
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4338), !dbg !4341
  %_3.not.i3537 = icmp eq i64 %_318.1.i.i.i, %row.sroa.0.0.i197.i.i, !dbg !4342
  br i1 %_3.not.i3537, label %panic.i3540, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3541, !dbg !4342

panic.i3540:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4342, !noalias !4344
  unreachable, !dbg !4342

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3541: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i.i
  %_35.i191.i.i = getelementptr inbounds nuw float, ptr %_318.0.i.i.i, i64 %row.sroa.0.0.i197.i.i, !dbg !4345
  %_0.i3539 = load float, ptr %_35.i191.i.i, align 4, !dbg !4342, !alias.scope !4338, !noalias !4347, !noundef !11
  %226 = tail call noundef float @llvm.fabs.f32(float %_0.i3554), !dbg !4348
  %_3.i.i5534 = fcmp ule float %226, 0x3E45798EE0000000, !dbg !4355
  %_6.i.i5536 = bitcast float %226 to i32, !dbg !4365
  %_4.i.i5540 = select i1 %_3.i.i5534, i32 841731191, i32 %_6.i.i5536, !dbg !4370
  %_0.i.i5541 = bitcast i32 %_4.i.i5540 to float, !dbg !4371
  %_3.i.i5058 = fcmp ule float %_0.i.i5541, 0x3810000000000000, !dbg !4374
  %_4.i.i5064 = select i1 %_3.i.i5058, i32 8388608, i32 %_4.i.i5540, !dbg !4383
  %_5.i3933 = and i32 %_4.i.i5064, 8388607, !dbg !4385
  %_4.i3934 = or disjoint i32 %_5.i3933, 1065353216, !dbg !4385
  %significand.i3935 = bitcast i32 %_4.i3934 to float, !dbg !4390
  %_0.i3353 = fadd float %significand.i3935, -1.000000e+00, !dbg !4393
  %_0.i3026 = fmul float %_0.i3353, 0xBF9B17A960000000, !dbg !4396
  %_0.i2546 = fadd float %_0.i3026, 0x3FBF9A8440000000, !dbg !4401
  %_0.i3026.1 = fmul float %_0.i3353, %_0.i2546, !dbg !4396
  %_0.i2546.1 = fadd float %_0.i3026.1, 0xBFD1E3F400000000, !dbg !4401
  %_0.i3026.2 = fmul float %_0.i3353, %_0.i2546.1, !dbg !4396
  %_0.i2546.2 = fadd float %_0.i3026.2, 0x3FDD544F20000000, !dbg !4401
  %_0.i3026.3 = fmul float %_0.i3353, %_0.i2546.2, !dbg !4396
  %_0.i2546.3 = fadd float %_0.i3026.3, 0xBFE6FC2A60000000, !dbg !4401
  %_0.i3026.4 = fmul float %_0.i3353, %_0.i2546.3, !dbg !4396
  %_0.i2546.4 = fadd float %_0.i3026.4, 0x3FF714B2A0000000, !dbg !4401
  %_9.i3936 = lshr i32 %_4.i.i5064, 23, !dbg !4403
  %_8.i3937 = or disjoint i32 %_9.i3936, 1258291200, !dbg !4403
  %_7.i3938 = bitcast i32 %_8.i3937 to float, !dbg !4405
  %exponent.i3939 = fadd float %_7.i3938, 0xC160000FE0000000, !dbg !4407
  %_0.i3025 = fmul float %_0.i3353, %_0.i2546.4, !dbg !4408
  %_0.i2545 = fadd float %exponent.i3939, %_0.i3025, !dbg !4410
  %_0.i3288 = fmul float %_0.i2545, 0x4018151820000000, !dbg !4412
  %_3.i.i5526.inv = fcmp ogt float %_0.i3288, -1.600000e+02, !dbg !4414
  %_0.i.i5533 = select i1 %_3.i.i5526.inv, float %_0.i3288, float -1.600000e+02, !dbg !4414
  %_3.i.i6236.inv = fcmp olt float %_0.i.i5533, 2.400000e+01, !dbg !4417
  %_0.i.i6243 = select i1 %_3.i.i6236.inv, float %_0.i.i5533, float 2.400000e+01, !dbg !4417
  %_0.i3401 = fsub float %_0.i.i6243, %_12.le.i, !dbg !4421
  %_3.i2257 = fcmp ule float %_0.i3401, 3.000000e+00, !dbg !4429
  %_0.i2625 = fadd float %_0.i3401, 3.000000e+00, !dbg !4432
  %_0.i3179 = fmul float %_0.i2625, %_0.i2625, !dbg !4437
  %_0.i3178 = fmul float %_0.i3179, 0x3FB5555560000000, !dbg !4440
  %_4.i4676.v.v = select i1 %_3.i2257, float %_0.i3178, float %_0.i3401, !dbg !4442
  %_4.i4676.v = fmul float %coefficients.i.i.sroa.0.0.copyload, %_4.i4676.v.v, !dbg !4442
  %_4.i4676 = bitcast float %_4.i4676.v to i32, !dbg !4442
  %227 = fcmp ugt float %_0.i3401, -3.000000e+00, !dbg !4445
  %_7.i4668 = select i1 %227, i32 %_4.i4676, i32 0, !dbg !4447
  %_0.i4670 = bitcast i32 %_7.i4668 to float, !dbg !4448
  %_3.i.i5518 = fcmp ule float %_0.i4670, -1.000000e+02, !dbg !4450
  %228 = bitcast i32 %_7.i4668 to float, !dbg !4453
  %_0.i.i5525 = select i1 %_3.i.i5518, float -1.000000e+02, float %228, !dbg !4456
  %_3.i.i6228 = fcmp olt float %_0.i.i5525, 0.000000e+00, !dbg !4457
  %_0.i.i6235 = select i1 %_3.i.i6228, float %_0.i.i5525, float 0.000000e+00, !dbg !4461
  %229 = bitcast i32 %gain_near.i.i.i.sroa.0.011196 to float, !dbg !4463
  %_3.i2447 = fcmp uge float %_0.i.i6235, %229, !dbg !4465
  %_4.i4822.v = select i1 %_3.i2447, float %coefficients.i.i.sroa.7.0.copyload, float %coefficients.i.i.sroa.5.0.copyload, !dbg !4469
  %_0.i3452 = fsub float %229, %_0.i.i6235, !dbg !4471
  %_4.i2959 = fmul float %_0.i3452, %_4.i4822.v, !dbg !4474
  %_0.i2960 = fadd float %_0.i.i6235, %_4.i2959, !dbg !4474
  %230 = tail call noundef float @llvm.fabs.f32(float %_0.i2960), !dbg !4477
  %_4.i4389 = bitcast float %_0.i2960 to i32, !dbg !4481
  %231 = fcmp uge float %230, 0x3BC79CA100000000, !dbg !4485
  %_3.i4391 = select i1 %231, i32 %_4.i4389, i32 0, !dbg !4486
  %_0.i4392 = bitcast i32 %_3.i4391 to float, !dbg !4487
  %_0.i2794 = fadd float %_12.le.4.i, %_0.i4392, !dbg !4490
  %_0.i3287 = fmul float %_0.i2794, 0x3FC542A5A0000000, !dbg !4493
  %_3.i.i5250.inv = fcmp ogt float %_0.i3287, -1.260000e+02, !dbg !4497
  %_0.i.i5257 = select i1 %_3.i.i5250.inv, float %_0.i3287, float -1.260000e+02, !dbg !4497
  %_3.i.i6052.inv = fcmp olt float %_0.i.i5257, 1.270000e+02, !dbg !4502
  %_0.i.i6059 = select i1 %_3.i.i6052.inv, float %_0.i.i5257, float 1.270000e+02, !dbg !4502
  %232 = tail call noundef float @llvm.floor.f32(float %_0.i.i6059), !dbg !4505
  %_0.i3377 = fsub float %_0.i.i6059, %232, !dbg !4518
  %233 = tail call noundef float @llvm.fabs.f32(float %_0.i3549), !dbg !4521
  %_3.i.i5510 = fcmp ule float %233, 0x3E45798EE0000000, !dbg !4524
  %_6.i.i5512 = bitcast float %233 to i32, !dbg !4529
  %_4.i.i5516 = select i1 %_3.i.i5510, i32 841731191, i32 %_6.i.i5512, !dbg !4532
  %_0.i.i5517 = bitcast i32 %_4.i.i5516 to float, !dbg !4533
  %_3.i.i5066 = fcmp ule float %_0.i.i5517, 0x3810000000000000, !dbg !4535
  %_4.i.i5072 = select i1 %_3.i.i5066, i32 8388608, i32 %_4.i.i5516, !dbg !4540
  %_5.i3941 = and i32 %_4.i.i5072, 8388607, !dbg !4542
  %_4.i3942 = or disjoint i32 %_5.i3941, 1065353216, !dbg !4542
  %significand.i3943 = bitcast i32 %_4.i3942 to float, !dbg !4544
  %_0.i3354 = fadd float %significand.i3943, -1.000000e+00, !dbg !4546
  %_0.i3028 = fmul float %_0.i3354, 0xBF9B17A960000000, !dbg !4548
  %_0.i2548 = fadd float %_0.i3028, 0x3FBF9A8440000000, !dbg !4550
  %_0.i3028.1 = fmul float %_0.i3354, %_0.i2548, !dbg !4548
  %_0.i2548.1 = fadd float %_0.i3028.1, 0xBFD1E3F400000000, !dbg !4550
  %_0.i3028.2 = fmul float %_0.i3354, %_0.i2548.1, !dbg !4548
  %_0.i2548.2 = fadd float %_0.i3028.2, 0x3FDD544F20000000, !dbg !4550
  %_0.i3028.3 = fmul float %_0.i3354, %_0.i2548.2, !dbg !4548
  %_0.i2548.3 = fadd float %_0.i3028.3, 0xBFE6FC2A60000000, !dbg !4550
  %_0.i3028.4 = fmul float %_0.i3354, %_0.i2548.3, !dbg !4548
  %_0.i2548.4 = fadd float %_0.i3028.4, 0x3FF714B2A0000000, !dbg !4550
  %_9.i3944 = lshr i32 %_4.i.i5072, 23, !dbg !4552
  %_8.i3945 = or disjoint i32 %_9.i3944, 1258291200, !dbg !4552
  %_7.i3946 = bitcast i32 %_8.i3945 to float, !dbg !4553
  %exponent.i3947 = fadd float %_7.i3946, 0xC160000FE0000000, !dbg !4555
  %_0.i3027 = fmul float %_0.i3354, %_0.i2548.4, !dbg !4556
  %_0.i2547 = fadd float %exponent.i3947, %_0.i3027, !dbg !4558
  %_0.i3286 = fmul float %_0.i2547, 0x4018151820000000, !dbg !4560
  %_3.i.i5502.inv = fcmp ogt float %_0.i3286, -1.600000e+02, !dbg !4562
  %_0.i.i5509 = select i1 %_3.i.i5502.inv, float %_0.i3286, float -1.600000e+02, !dbg !4562
  %_3.i.i6220.inv = fcmp olt float %_0.i.i5509, 2.400000e+01, !dbg !4565
  %_0.i.i6227 = select i1 %_3.i.i6220.inv, float %_0.i.i5509, float 2.400000e+01, !dbg !4565
  %_0.i3402 = fsub float %_0.i.i6227, %_12.le.5.i, !dbg !4568
  %_3.i2259 = fcmp ule float %_0.i3402, 3.000000e+00, !dbg !4571
  %_0.i2626 = fadd float %_0.i3402, 3.000000e+00, !dbg !4573
  %_0.i3183 = fmul float %_0.i2626, %_0.i2626, !dbg !4575
  %_0.i3182 = fmul float %_0.i3183, 0x3FB5555560000000, !dbg !4577
  %_4.i4689.v.v = select i1 %_3.i2259, float %_0.i3182, float %_0.i3402, !dbg !4579
  %_4.i4689.v = fmul float %coefficients.i.i.sroa.9.0.copyload, %_4.i4689.v.v, !dbg !4579
  %_4.i4689 = bitcast float %_4.i4689.v to i32, !dbg !4579
  %234 = fcmp ugt float %_0.i3402, -3.000000e+00, !dbg !4581
  %_7.i4681 = select i1 %234, i32 %_4.i4689, i32 0, !dbg !4583
  %_0.i4683 = bitcast i32 %_7.i4681 to float, !dbg !4584
  %_3.i.i5494 = fcmp ule float %_0.i4683, -1.000000e+02, !dbg !4586
  %235 = bitcast i32 %_7.i4681 to float, !dbg !4589
  %_0.i.i5501 = select i1 %_3.i.i5494, float -1.000000e+02, float %235, !dbg !4592
  %_3.i.i6212 = fcmp olt float %_0.i.i5501, 0.000000e+00, !dbg !4593
  %_0.i.i6219 = select i1 %_3.i.i6212, float %_0.i.i5501, float 0.000000e+00, !dbg !4596
  %236 = bitcast i32 %gain_near.i.i.i.sroa.6.011197 to float, !dbg !4598
  %_3.i2443 = fcmp uge float %_0.i.i6219, %236, !dbg !4599
  %_4.i4815.v = select i1 %_3.i2443, float %coefficients.i.i.sroa.13.0.copyload, float %coefficients.i.i.sroa.11.0.copyload, !dbg !4602
  %_0.i3451 = fsub float %236, %_0.i.i6219, !dbg !4604
  %_4.i2957 = fmul float %_0.i3451, %_4.i4815.v, !dbg !4606
  %_0.i2958 = fadd float %_0.i.i6219, %_4.i2957, !dbg !4606
  %237 = tail call noundef float @llvm.fabs.f32(float %_0.i2958), !dbg !4608
  %_4.i4385 = bitcast float %_0.i2958 to i32, !dbg !4611
  %238 = fcmp uge float %237, 0x3BC79CA100000000, !dbg !4614
  %_3.i4387 = select i1 %238, i32 %_4.i4385, i32 0, !dbg !4615
  %_0.i4388 = bitcast i32 %_3.i4387 to float, !dbg !4616
  %_0.i2793 = fadd float %_12.le.9.i, %_0.i4388, !dbg !4618
  %_0.i3285 = fmul float %_0.i2793, 0x3FC542A5A0000000, !dbg !4620
  %_3.i.i5258.inv = fcmp ogt float %_0.i3285, -1.260000e+02, !dbg !4623
  %_0.i.i5265 = select i1 %_3.i.i5258.inv, float %_0.i3285, float -1.260000e+02, !dbg !4623
  %_3.i.i6060.inv = fcmp olt float %_0.i.i5265, 1.270000e+02, !dbg !4627
  %_0.i.i6067 = select i1 %_3.i.i6060.inv, float %_0.i.i5265, float 1.270000e+02, !dbg !4627
  %239 = tail call noundef float @llvm.floor.f32(float %_0.i.i6067), !dbg !4630
  %_0.i3378 = fsub float %_0.i.i6067, %239, !dbg !4634
  %240 = tail call noundef float @llvm.fabs.f32(float %_0.i3544), !dbg !4636
  %_3.i.i5486 = fcmp ule float %240, 0x3E45798EE0000000, !dbg !4638
  %_6.i.i5488 = bitcast float %240 to i32, !dbg !4643
  %_4.i.i5492 = select i1 %_3.i.i5486, i32 841731191, i32 %_6.i.i5488, !dbg !4646
  %_0.i.i5493 = bitcast i32 %_4.i.i5492 to float, !dbg !4647
  %_3.i.i5074 = fcmp ule float %_0.i.i5493, 0x3810000000000000, !dbg !4649
  %_4.i.i5080 = select i1 %_3.i.i5074, i32 8388608, i32 %_4.i.i5492, !dbg !4654
  %_5.i3949 = and i32 %_4.i.i5080, 8388607, !dbg !4656
  %_4.i3950 = or disjoint i32 %_5.i3949, 1065353216, !dbg !4656
  %significand.i3951 = bitcast i32 %_4.i3950 to float, !dbg !4658
  %_0.i3355 = fadd float %significand.i3951, -1.000000e+00, !dbg !4660
  %_0.i3030 = fmul float %_0.i3355, 0xBF9B17A960000000, !dbg !4662
  %_0.i2550 = fadd float %_0.i3030, 0x3FBF9A8440000000, !dbg !4664
  %_0.i3030.1 = fmul float %_0.i3355, %_0.i2550, !dbg !4662
  %_0.i2550.1 = fadd float %_0.i3030.1, 0xBFD1E3F400000000, !dbg !4664
  %_0.i3030.2 = fmul float %_0.i3355, %_0.i2550.1, !dbg !4662
  %_0.i2550.2 = fadd float %_0.i3030.2, 0x3FDD544F20000000, !dbg !4664
  %_0.i3030.3 = fmul float %_0.i3355, %_0.i2550.2, !dbg !4662
  %_0.i2550.3 = fadd float %_0.i3030.3, 0xBFE6FC2A60000000, !dbg !4664
  %_0.i3030.4 = fmul float %_0.i3355, %_0.i2550.3, !dbg !4662
  %_0.i2550.4 = fadd float %_0.i3030.4, 0x3FF714B2A0000000, !dbg !4664
  %_9.i3952 = lshr i32 %_4.i.i5080, 23, !dbg !4666
  %_8.i3953 = or disjoint i32 %_9.i3952, 1258291200, !dbg !4666
  %_7.i3954 = bitcast i32 %_8.i3953 to float, !dbg !4667
  %exponent.i3955 = fadd float %_7.i3954, 0xC160000FE0000000, !dbg !4669
  %_0.i3029 = fmul float %_0.i3355, %_0.i2550.4, !dbg !4670
  %_0.i2549 = fadd float %exponent.i3955, %_0.i3029, !dbg !4672
  %_0.i3284 = fmul float %_0.i2549, 0x4018151820000000, !dbg !4674
  %_3.i.i5478.inv = fcmp ogt float %_0.i3284, -1.600000e+02, !dbg !4676
  %_0.i.i5485 = select i1 %_3.i.i5478.inv, float %_0.i3284, float -1.600000e+02, !dbg !4676
  %_3.i.i6204.inv = fcmp olt float %_0.i.i5485, 2.400000e+01, !dbg !4679
  %_0.i.i6211 = select i1 %_3.i.i6204.inv, float %_0.i.i5485, float 2.400000e+01, !dbg !4679
  %_0.i3403 = fsub float %_0.i.i6211, %_12.le.i6504, !dbg !4682
  %_3.i2261 = fcmp ule float %_0.i3403, 3.000000e+00, !dbg !4685
  %_0.i2627 = fadd float %_0.i3403, 3.000000e+00, !dbg !4687
  %_0.i3187 = fmul float %_0.i2627, %_0.i2627, !dbg !4689
  %_0.i3186 = fmul float %_0.i3187, 0x3FB5555560000000, !dbg !4691
  %_4.i4702.v.v = select i1 %_3.i2261, float %_0.i3186, float %_0.i3403, !dbg !4693
  %_4.i4702.v = fmul float %coefficients.i.i.sroa.15.24.copyload, %_4.i4702.v.v, !dbg !4693
  %_4.i4702 = bitcast float %_4.i4702.v to i32, !dbg !4693
  %241 = fcmp ugt float %_0.i3403, -3.000000e+00, !dbg !4695
  %_7.i4694 = select i1 %241, i32 %_4.i4702, i32 0, !dbg !4697
  %_0.i4696 = bitcast i32 %_7.i4694 to float, !dbg !4698
  %_3.i.i5470 = fcmp ule float %_0.i4696, -1.000000e+02, !dbg !4700
  %242 = bitcast i32 %_7.i4694 to float, !dbg !4703
  %_0.i.i5477 = select i1 %_3.i.i5470, float -1.000000e+02, float %242, !dbg !4706
  %_3.i.i6196 = fcmp olt float %_0.i.i5477, 0.000000e+00, !dbg !4707
  %_0.i.i6203 = select i1 %_3.i.i6196, float %_0.i.i5477, float 0.000000e+00, !dbg !4710
  %243 = bitcast i32 %gain_far.i.i.i.sroa.0.011198 to float, !dbg !4712
  %_3.i2439 = fcmp uge float %_0.i.i6203, %243, !dbg !4713
  %_4.i4808.v = select i1 %_3.i2439, float %coefficients.i.i.sroa.20.24.copyload, float %coefficients.i.i.sroa.18.24.copyload, !dbg !4716
  %_0.i3450 = fsub float %243, %_0.i.i6203, !dbg !4718
  %_4.i2955 = fmul float %_0.i3450, %_4.i4808.v, !dbg !4720
  %_0.i2956 = fadd float %_0.i.i6203, %_4.i2955, !dbg !4720
  %244 = tail call noundef float @llvm.fabs.f32(float %_0.i2956), !dbg !4722
  %_4.i4381 = bitcast float %_0.i2956 to i32, !dbg !4725
  %245 = fcmp uge float %244, 0x3BC79CA100000000, !dbg !4728
  %_3.i4383 = select i1 %245, i32 %_4.i4381, i32 0, !dbg !4729
  %_0.i4384 = bitcast i32 %_3.i4383 to float, !dbg !4730
  %_0.i2792 = fadd float %_12.le.4.i6512, %_0.i4384, !dbg !4732
  %_0.i3283 = fmul float %_0.i2792, 0x3FC542A5A0000000, !dbg !4734
  %_3.i.i5266.inv = fcmp ogt float %_0.i3283, -1.260000e+02, !dbg !4737
  %_0.i.i5273 = select i1 %_3.i.i5266.inv, float %_0.i3283, float -1.260000e+02, !dbg !4737
  %_3.i.i6068.inv = fcmp olt float %_0.i.i5273, 1.270000e+02, !dbg !4741
  %_0.i.i6075 = select i1 %_3.i.i6068.inv, float %_0.i.i5273, float 1.270000e+02, !dbg !4741
  %246 = tail call noundef float @llvm.floor.f32(float %_0.i.i6075), !dbg !4744
  %_0.i3379 = fsub float %_0.i.i6075, %246, !dbg !4748
  %247 = tail call noundef float @llvm.fabs.f32(float %_0.i3539), !dbg !4750
  %_3.i.i5462 = fcmp ule float %247, 0x3E45798EE0000000, !dbg !4752
  %_6.i.i5464 = bitcast float %247 to i32, !dbg !4757
  %_4.i.i5468 = select i1 %_3.i.i5462, i32 841731191, i32 %_6.i.i5464, !dbg !4760
  %_0.i.i5469 = bitcast i32 %_4.i.i5468 to float, !dbg !4761
  %_3.i.i5082 = fcmp ule float %_0.i.i5469, 0x3810000000000000, !dbg !4763
  %_4.i.i5088 = select i1 %_3.i.i5082, i32 8388608, i32 %_4.i.i5468, !dbg !4768
  %_5.i3957 = and i32 %_4.i.i5088, 8388607, !dbg !4770
  %_4.i3958 = or disjoint i32 %_5.i3957, 1065353216, !dbg !4770
  %significand.i3959 = bitcast i32 %_4.i3958 to float, !dbg !4772
  %_0.i3356 = fadd float %significand.i3959, -1.000000e+00, !dbg !4774
  %_0.i3032 = fmul float %_0.i3356, 0xBF9B17A960000000, !dbg !4776
  %_0.i2552 = fadd float %_0.i3032, 0x3FBF9A8440000000, !dbg !4778
  %_0.i3032.1 = fmul float %_0.i3356, %_0.i2552, !dbg !4776
  %_0.i2552.1 = fadd float %_0.i3032.1, 0xBFD1E3F400000000, !dbg !4778
  %_0.i3032.2 = fmul float %_0.i3356, %_0.i2552.1, !dbg !4776
  %_0.i2552.2 = fadd float %_0.i3032.2, 0x3FDD544F20000000, !dbg !4778
  %_0.i3032.3 = fmul float %_0.i3356, %_0.i2552.2, !dbg !4776
  %_0.i2552.3 = fadd float %_0.i3032.3, 0xBFE6FC2A60000000, !dbg !4778
  %_0.i3032.4 = fmul float %_0.i3356, %_0.i2552.3, !dbg !4776
  %_0.i2552.4 = fadd float %_0.i3032.4, 0x3FF714B2A0000000, !dbg !4778
  %_9.i3960 = lshr i32 %_4.i.i5088, 23, !dbg !4780
  %_8.i3961 = or disjoint i32 %_9.i3960, 1258291200, !dbg !4780
  %_7.i3962 = bitcast i32 %_8.i3961 to float, !dbg !4781
  %exponent.i3963 = fadd float %_7.i3962, 0xC160000FE0000000, !dbg !4783
  %_0.i3031 = fmul float %_0.i3356, %_0.i2552.4, !dbg !4784
  %_0.i2551 = fadd float %exponent.i3963, %_0.i3031, !dbg !4786
  %_0.i3282 = fmul float %_0.i2551, 0x4018151820000000, !dbg !4788
  %_3.i.i5454.inv = fcmp ogt float %_0.i3282, -1.600000e+02, !dbg !4790
  %_0.i.i5461 = select i1 %_3.i.i5454.inv, float %_0.i3282, float -1.600000e+02, !dbg !4790
  %_3.i.i6188.inv = fcmp olt float %_0.i.i5461, 2.400000e+01, !dbg !4793
  %_0.i.i6195 = select i1 %_3.i.i6188.inv, float %_0.i.i5461, float 2.400000e+01, !dbg !4793
  %_0.i3404 = fsub float %_0.i.i6195, %_12.le.5.i6514, !dbg !4796
  %_3.i2263 = fcmp ule float %_0.i3404, 3.000000e+00, !dbg !4799
  %_0.i2628 = fadd float %_0.i3404, 3.000000e+00, !dbg !4801
  %_0.i3191 = fmul float %_0.i2628, %_0.i2628, !dbg !4803
  %_0.i3190 = fmul float %_0.i3191, 0x3FB5555560000000, !dbg !4805
  %_4.i4715.v.v = select i1 %_3.i2263, float %_0.i3190, float %_0.i3404, !dbg !4807
  %_4.i4715.v = fmul float %coefficients.i.i.sroa.22.24.copyload, %_4.i4715.v.v, !dbg !4807
  %_4.i4715 = bitcast float %_4.i4715.v to i32, !dbg !4807
  %248 = fcmp ugt float %_0.i3404, -3.000000e+00, !dbg !4809
  %_7.i4707 = select i1 %248, i32 %_4.i4715, i32 0, !dbg !4811
  %_0.i4709 = bitcast i32 %_7.i4707 to float, !dbg !4812
  %_3.i.i5446 = fcmp ule float %_0.i4709, -1.000000e+02, !dbg !4814
  %249 = bitcast i32 %_7.i4707 to float, !dbg !4817
  %_0.i.i5453 = select i1 %_3.i.i5446, float -1.000000e+02, float %249, !dbg !4820
  %_3.i.i6180 = fcmp olt float %_0.i.i5453, 0.000000e+00, !dbg !4821
  %_0.i.i6187 = select i1 %_3.i.i6180, float %_0.i.i5453, float 0.000000e+00, !dbg !4824
  %250 = bitcast i32 %gain_far.i.i.i.sroa.6.011199 to float, !dbg !4826
  %_3.i2435 = fcmp uge float %_0.i.i6187, %250, !dbg !4827
  %_4.i4801.v = select i1 %_3.i2435, float %coefficients.i.i.sroa.26.24.copyload, float %coefficients.i.i.sroa.24.24.copyload, !dbg !4830
  %_0.i3449 = fsub float %250, %_0.i.i6187, !dbg !4832
  %_4.i2953 = fmul float %_0.i3449, %_4.i4801.v, !dbg !4834
  %_0.i2954 = fadd float %_0.i.i6187, %_4.i2953, !dbg !4834
  %251 = tail call noundef float @llvm.fabs.f32(float %_0.i2954), !dbg !4836
  %_4.i4377 = bitcast float %_0.i2954 to i32, !dbg !4839
  %252 = fcmp uge float %251, 0x3BC79CA100000000, !dbg !4842
  %_3.i4379 = select i1 %252, i32 %_4.i4377, i32 0, !dbg !4843
  %_0.i4380 = bitcast i32 %_3.i4379 to float, !dbg !4844
  %_0.i2791 = fadd float %_12.le.9.i6522, %_0.i4380, !dbg !4846
  %_0.i3281 = fmul float %_0.i2791, 0x3FC542A5A0000000, !dbg !4848
  %_3.i.i5274.inv = fcmp ogt float %_0.i3281, -1.260000e+02, !dbg !4851
  %_0.i.i5281 = select i1 %_3.i.i5274.inv, float %_0.i3281, float -1.260000e+02, !dbg !4851
  %_3.i.i6076.inv = fcmp olt float %_0.i.i5281, 1.270000e+02, !dbg !4855
  %_0.i.i6083 = select i1 %_3.i.i6076.inv, float %_0.i.i5281, float 1.270000e+02, !dbg !4855
  %253 = tail call noundef float @llvm.floor.f32(float %_0.i.i6083), !dbg !4858
  %_0.i3380 = fsub float %_0.i.i6083, %253, !dbg !4862
  %_0.i3100 = fmul float %_0.i3380, 0x3F5E974FA0000000, !dbg !4864
  %_0.i2600 = fadd float %_0.i3100, 0x3F82778560000000, !dbg !4869
  %_0.i3100.1 = fmul float %_0.i3380, %_0.i2600, !dbg !4864
  %_0.i2600.1 = fadd float %_0.i3100.1, 0x3FAC91CE60000000, !dbg !4869
  %_0.i3100.2 = fmul float %_0.i3380, %_0.i2600.1, !dbg !4864
  %_0.i2600.2 = fadd float %_0.i3100.2, 0x3FCEBDB560000000, !dbg !4869
  %_0.i3100.3 = fmul float %_0.i3380, %_0.i2600.2, !dbg !4864
  %_0.i2600.3 = fadd float %_0.i3100.3, 0x3FE62E4BA0000000, !dbg !4869
  %_0.i3097 = fmul float %_0.i3379, 0x3F5E974FA0000000, !dbg !4871
  %_0.i2598 = fadd float %_0.i3097, 0x3F82778560000000, !dbg !4873
  %_0.i3097.1 = fmul float %_0.i3379, %_0.i2598, !dbg !4871
  %_0.i2598.1 = fadd float %_0.i3097.1, 0x3FAC91CE60000000, !dbg !4873
  %_0.i3097.2 = fmul float %_0.i3379, %_0.i2598.1, !dbg !4871
  %_0.i2598.2 = fadd float %_0.i3097.2, 0x3FCEBDB560000000, !dbg !4873
  %_0.i3097.3 = fmul float %_0.i3379, %_0.i2598.2, !dbg !4871
  %_0.i2598.3 = fadd float %_0.i3097.3, 0x3FE62E4BA0000000, !dbg !4873
  %_0.i3094 = fmul float %_0.i3378, 0x3F5E974FA0000000, !dbg !4875
  %_0.i2596 = fadd float %_0.i3094, 0x3F82778560000000, !dbg !4877
  %_0.i3094.1 = fmul float %_0.i3378, %_0.i2596, !dbg !4875
  %_0.i2596.1 = fadd float %_0.i3094.1, 0x3FAC91CE60000000, !dbg !4877
  %_0.i3094.2 = fmul float %_0.i3378, %_0.i2596.1, !dbg !4875
  %_0.i2596.2 = fadd float %_0.i3094.2, 0x3FCEBDB560000000, !dbg !4877
  %_0.i3094.3 = fmul float %_0.i3378, %_0.i2596.2, !dbg !4875
  %_0.i2596.3 = fadd float %_0.i3094.3, 0x3FE62E4BA0000000, !dbg !4877
  %_0.i3091 = fmul float %_0.i3377, 0x3F5E974FA0000000, !dbg !4879
  %_0.i2594 = fadd float %_0.i3091, 0x3F82778560000000, !dbg !4881
  %_0.i3091.1 = fmul float %_0.i3377, %_0.i2594, !dbg !4879
  %_0.i2594.1 = fadd float %_0.i3091.1, 0x3FAC91CE60000000, !dbg !4881
  %_0.i3091.2 = fmul float %_0.i3377, %_0.i2594.1, !dbg !4879
  %_0.i2594.2 = fadd float %_0.i3091.2, 0x3FCEBDB560000000, !dbg !4881
  %_0.i3091.3 = fmul float %_0.i3377, %_0.i2594.2, !dbg !4879
  %_0.i2594.3 = fadd float %_0.i3091.3, 0x3FE62E4BA0000000, !dbg !4881
  %_0.i3090 = fmul float %_0.i3377, %_0.i2594.3, !dbg !4883
  %_0.i2593 = fadd float %_0.i3090, 1.000000e+00, !dbg !4885
  %biased.i2194 = fadd float %232, 0x4160000FE0000000, !dbg !4887
  %_4.i2195 = bitcast float %biased.i2194 to i32, !dbg !4891
  %_3.i2196 = shl i32 %_4.i2195, 23, !dbg !4895
  %_0.i2197 = bitcast i32 %_3.i2196 to float, !dbg !4896
  %_0.i3089 = fmul float %_0.i2593, %_0.i2197, !dbg !4899
  %_0.i3093 = fmul float %_0.i3378, %_0.i2596.3, !dbg !4901
  %_0.i2595 = fadd float %_0.i3093, 1.000000e+00, !dbg !4903
  %biased.i2198 = fadd float %239, 0x4160000FE0000000, !dbg !4905
  %_4.i2199 = bitcast float %biased.i2198 to i32, !dbg !4907
  %_3.i2200 = shl i32 %_4.i2199, 23, !dbg !4909
  %_0.i2201 = bitcast i32 %_3.i2200 to float, !dbg !4910
  %_0.i3092 = fmul float %_0.i2595, %_0.i2201, !dbg !4912
  %_0.i3096 = fmul float %_0.i3379, %_0.i2598.3, !dbg !4914
  %_0.i2597 = fadd float %_0.i3096, 1.000000e+00, !dbg !4916
  %biased.i2202 = fadd float %246, 0x4160000FE0000000, !dbg !4918
  %_4.i2203 = bitcast float %biased.i2202 to i32, !dbg !4920
  %_3.i2204 = shl i32 %_4.i2203, 23, !dbg !4922
  %_0.i2205 = bitcast i32 %_3.i2204 to float, !dbg !4923
  %_0.i3095 = fmul float %_0.i2597, %_0.i2205, !dbg !4925
  %_0.i3099 = fmul float %_0.i3380, %_0.i2600.3, !dbg !4927
  %_0.i2599 = fadd float %_0.i3099, 1.000000e+00, !dbg !4929
  %biased.i2206 = fadd float %253, 0x4160000FE0000000, !dbg !4931
  %_4.i2207 = bitcast float %biased.i2206 to i32, !dbg !4933
  %_3.i2208 = shl i32 %_4.i2207, 23, !dbg !4935
  %_0.i2209 = bitcast i32 %_3.i2208 to float, !dbg !4936
  %_0.i3098 = fmul float %_0.i2599, %_0.i2209, !dbg !4938
  %_266.i.i.i = icmp ugt i64 %_41.sroa.0.0.i.i.i, %_315.1.i.i.i, !dbg !4940
  br i1 %_266.i.i.i, label %bb86.i.i.i, label %bb87.i.i.i, !dbg !4940, !prof !1664

bb84.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4033
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i.i11200, i64 noundef %_314.1.i.i.i, i64 noundef %_314.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e11ba0c4c1124bbcae4603a2ae121338) #26, !dbg !4945, !noalias !3966
  unreachable, !dbg !4945

bb87.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3541
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4946), !dbg !4949
  %_3.not.i3532 = icmp eq i64 %_315.1.i.i.i, %_41.sroa.0.0.i.i.i, !dbg !4950
  br i1 %_3.not.i3532, label %panic.i3535, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3536, !dbg !4950

panic.i3535:                                      ; preds = %bb87.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4950, !noalias !4952
  unreachable, !dbg !4950

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3536: ; preds = %bb87.i.i.i
  %_273.i.i.i = getelementptr inbounds nuw float, ptr %_315.0.i.i.i, i64 %_41.sroa.0.0.i.i.i, !dbg !4953
  %_0.i3534 = load float, ptr %_273.i.i.i, align 4, !dbg !4950, !alias.scope !4946, !noalias !3966, !noundef !11
  %_0.i3280 = fmul float %_0.i3089, %_0.i3534, !dbg !4958
  %_274.i.i.i = icmp ugt i64 %_41.sroa.0.0.i.i.i, %_316.1.i.i.i, !dbg !4960
  br i1 %_274.i.i.i, label %bb88.i.i.i, label %bb89.i.i.i, !dbg !4960, !prof !1664

bb86.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3541
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i.i, i64 noundef %_315.1.i.i.i, i64 noundef %_315.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_379b93204ee4659b79f4b07b6520076a) #26, !dbg !4964, !noalias !3966
  unreachable, !dbg !4964

bb89.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3536
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4965), !dbg !4968
  %_3.not.i3527 = icmp eq i64 %_316.1.i.i.i, %_41.sroa.0.0.i.i.i, !dbg !4969
  br i1 %_3.not.i3527, label %panic.i3530, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3531, !dbg !4969

panic.i3530:                                      ; preds = %bb89.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4969, !noalias !4971
  unreachable, !dbg !4969

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3531: ; preds = %bb89.i.i.i
  %_281.i.i.i = getelementptr inbounds nuw float, ptr %_316.0.i.i.i, i64 %_41.sroa.0.0.i.i.i, !dbg !4972
  %_0.i3529 = load float, ptr %_281.i.i.i, align 4, !dbg !4969, !alias.scope !4965, !noalias !3966, !noundef !11
  %_0.i3279 = fmul float %_0.i3092, %_0.i3529, !dbg !4977
  %_0.i2790 = fadd float %_0.i3280, %_0.i3279, !dbg !4979
  %_282.i.i.i = icmp ugt i64 %_41.sroa.0.0.i.i.i, %_317.1.i.i.i, !dbg !4981
  br i1 %_282.i.i.i, label %bb90.i.i.i, label %bb91.i.i.i, !dbg !4981, !prof !1664

bb88.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3536
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i.i, i64 noundef %_316.1.i.i.i, i64 noundef %_316.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2bb8eb0542a889f29ef069491eb97a17) #26, !dbg !4986, !noalias !3966
  unreachable, !dbg !4986

bb91.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3531
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4987), !dbg !4990
  %_3.not.i3522 = icmp eq i64 %_317.1.i.i.i, %_41.sroa.0.0.i.i.i, !dbg !4991
  br i1 %_3.not.i3522, label %panic.i3525, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3526, !dbg !4991

panic.i3525:                                      ; preds = %bb91.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !4991, !noalias !4993
  unreachable, !dbg !4991

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3526: ; preds = %bb91.i.i.i
  %_289.i.i.i = getelementptr inbounds nuw float, ptr %_317.0.i.i.i, i64 %_41.sroa.0.0.i.i.i, !dbg !4994
  %_0.i3524 = load float, ptr %_289.i.i.i, align 4, !dbg !4991, !alias.scope !4987, !noalias !3966, !noundef !11
  %_0.i3278 = fmul float %_0.i3095, %_0.i3524, !dbg !4999
  %_290.i.i.i = icmp ugt i64 %_41.sroa.0.0.i.i.i, %_318.1.i.i.i, !dbg !5001
  br i1 %_290.i.i.i, label %bb92.i.i.i, label %bb93.i.i.i, !dbg !5001, !prof !1664

bb90.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3531
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i.i, i64 noundef %_317.1.i.i.i, i64 noundef %_317.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd96798e807157d7db308788cb4dd125) #26, !dbg !5005, !noalias !3966
  unreachable, !dbg !5005

bb93.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3526
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5006), !dbg !5009
  %_3.not.i3517 = icmp eq i64 %_318.1.i.i.i, %_41.sroa.0.0.i.i.i, !dbg !5010
  br i1 %_3.not.i3517, label %panic.i3520, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021, !dbg !5010

panic.i3520:                                      ; preds = %bb93.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5010, !noalias !5012
  unreachable, !dbg !5010

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021: ; preds = %bb93.i.i.i
  %_297.i.i.i = getelementptr inbounds nuw float, ptr %_318.0.i.i.i, i64 %_41.sroa.0.0.i.i.i, !dbg !5013
  %_0.i3519 = load float, ptr %_297.i.i.i, align 4, !dbg !5010, !alias.scope !5006, !noalias !3966, !noundef !11
  %_0.i3277 = fmul float %_0.i3098, %_0.i3519, !dbg !5018
  %_0.i2789 = fadd float %_0.i3278, %_0.i3277, !dbg !5020
  store float %_0.i2790, ptr %_184.i.i.i, align 4, !dbg !5022, !alias.scope !5025, !noalias !3966
  store float %_0.i2789, ptr %_192.i.i.i, align 4, !dbg !5028, !alias.scope !5030, !noalias !3966
  %exitcond14310.not = icmp eq i64 %206, %plan.0.i.i, !dbg !3922
  br i1 %exitcond14310.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_KBZ_EB2_.exit.i.i, label %bb56.i.i.i, !dbg !3934

bb92.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3526
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i.i, i64 noundef %_318.1.i.i.i, i64 noundef %_318.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_164adf6876ccf79975d46e129e248ca5) #26, !dbg !5033, !noalias !3966
  unreachable, !dbg !5033

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_KBZ_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021, %bb27.i.i
  %filter_near.i.i.i.sroa.14.0.lcssa = phi float [ %filter_near.i.i.i.sroa.14.0.copyload, %bb27.i.i ], [ %_0.i4228, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5034
  %filter_near.i.i.i.sroa.11.0.lcssa = phi float [ %filter_near.i.i.i.sroa.11.0.copyload, %bb27.i.i ], [ %_0.i4232, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5034
  %filter_near.i.i.i.sroa.7.0.lcssa = phi float [ %filter_near.i.i.i.sroa.7.0.copyload, %bb27.i.i ], [ %_0.i4220, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5034
  %filter_near.i.i.i.sroa.0.0.lcssa = phi float [ %filter_near.i.i.i.sroa.0.0.copyload, %bb27.i.i ], [ %_0.i4224, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5034
  %filter_far.i.i.i.sroa.0.0.lcssa = phi float [ %filter_far.i.i.i.sroa.0.0.copyload, %bb27.i.i ], [ %_0.i4208, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5035
  %filter_far.i.i.i.sroa.7.0.lcssa = phi float [ %filter_far.i.i.i.sroa.7.0.copyload, %bb27.i.i ], [ %_0.i4204, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5035
  %filter_far.i.i.i.sroa.11.0.lcssa = phi float [ %filter_far.i.i.i.sroa.11.0.copyload, %bb27.i.i ], [ %_0.i4216, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5035
  %filter_far.i.i.i.sroa.14.0.lcssa = phi float [ %filter_far.i.i.i.sroa.14.0.copyload, %bb27.i.i ], [ %_0.i4212, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5035
  %gain_near.i.i.i.sroa.0.0.lcssa = phi i32 [ %201, %bb27.i.i ], [ %_3.i4391, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5036
  %gain_near.i.i.i.sroa.6.0.lcssa = phi i32 [ %202, %bb27.i.i ], [ %_3.i4387, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5036
  %gain_far.i.i.i.sroa.0.0.lcssa = phi i32 [ %203, %bb27.i.i ], [ %_3.i4383, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5037
  %gain_far.i.i.i.sroa.6.0.lcssa = phi i32 [ %204, %bb27.i.i ], [ %_3.i4379, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5037
  %position.sroa.0.0.i.i.i.lcssa = phi i64 [ %205, %bb27.i.i ], [ %_41.sroa.0.0.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4021 ], !dbg !5038
  store float %filter_near.i.i.i.sroa.0.0.lcssa, ptr %164, align 8, !dbg !5039, !noalias !3966
  store float %filter_near.i.i.i.sroa.7.0.lcssa, ptr %filter_near.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !5039, !noalias !3966
  store float %filter_near.i.i.i.sroa.11.0.lcssa, ptr %filter_near.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !5039, !noalias !3966
  store float %filter_near.i.i.i.sroa.14.0.lcssa, ptr %filter_near.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !5039, !noalias !3966
  store float %filter_far.i.i.i.sroa.0.0.lcssa, ptr %165, align 8, !dbg !5040, !noalias !3966
  store float %filter_far.i.i.i.sroa.7.0.lcssa, ptr %filter_far.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !5040, !noalias !3966
  store float %filter_far.i.i.i.sroa.11.0.lcssa, ptr %filter_far.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !5040, !noalias !3966
  store float %filter_far.i.i.i.sroa.14.0.lcssa, ptr %filter_far.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !5040, !noalias !3966
  store i32 %gain_near.i.i.i.sroa.0.0.lcssa, ptr %166, align 8, !dbg !5041, !noalias !3966
  store i32 %gain_near.i.i.i.sroa.6.0.lcssa, ptr %.sroa_idx7025, align 4, !dbg !5041, !noalias !3966
  store i32 %gain_far.i.i.i.sroa.0.0.lcssa, ptr %167, align 8, !dbg !5042, !noalias !3966
  store i32 %gain_far.i.i.i.sroa.6.0.lcssa, ptr %.sroa_idx7030, align 4, !dbg !5042, !noalias !3966
  store i64 %position.sroa.0.0.i.i.i.lcssa, ptr %_51.i.i, align 8, !dbg !5043, !alias.scope !3919, !noalias !3920
  br label %bb15.i.i, !dbg !5044

bb15.i.i:                                         ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_Kb1_EB2_.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_KBZ_EB2_.exit.i.i
  %_8.i.i = icmp ult i64 %_32.i.i, %_19.1, !dbg !3837
  br i1 %_8.i.i, label %bb2.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit, !dbg !3837

bb17.i.i:                                         ; preds = %bb9.i.i
  %_75.i.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i.i11245, !dbg !5045
  %_85.i.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i.i11245, !dbg !5048
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5055), !dbg !5058
  %filter_near.i17.i.i.sroa.0.0.copyload = load float, ptr %164, align 8, !dbg !5059, !noalias !5065
  %filter_near.i17.i.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !5059, !noalias !5065
  %filter_near.i17.i.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !5059, !noalias !5065
  %filter_near.i17.i.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !5059, !noalias !5065
  %filter_far.i16.i.i.sroa.0.0.copyload = load float, ptr %165, align 8, !dbg !5070, !noalias !5065
  %filter_far.i16.i.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !5070, !noalias !5065
  %filter_far.i16.i.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !5070, !noalias !5065
  %filter_far.i16.i.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !5070, !noalias !5065
  %254 = load i32, ptr %166, align 8, !dbg !5072
  %255 = load i32, ptr %.sroa_idx7025, align 4, !dbg !5072
  %256 = load i32, ptr %167, align 8, !dbg !5074
  %257 = load i32, ptr %.sroa_idx7030, align 4, !dbg !5074
  %258 = load i64, ptr %_51.i.i, align 8, !dbg !5076, !alias.scope !5078, !noalias !5079, !noundef !11
  %_168.i34.i.i11216.not = icmp eq i64 %plan.0.i.i, 0, !dbg !5081
  br i1 %_168.i34.i.i11216.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_Kb1_EB2_.exit.i.i, label %bb56.i37.i.i, !dbg !5093

bb19.i.i:                                         ; preds = %bb9.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i11245, i64 noundef %_32.i.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_683f9160cdc5ee4596b2ecf709188a9a) #26, !dbg !5094, !noalias !3842
  unreachable, !dbg !5094

bb56.i37.i.i:                                     ; preds = %bb17.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit
  %segments.i.i.sroa.112.1 = phi float [ %_0.i2781.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.9.i6522, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.108.1 = phi float [ %_0.i2781.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.8.i6520, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.104.1 = phi float [ %_0.i2781.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.7.i6518, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.100.1 = phi float [ %_0.i2781.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.6.i6516, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.94.1 = phi float [ %_0.i2781.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.5.i6514, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.88.1 = phi float [ %_0.i2781.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.4.i6512, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.84.1 = phi float [ %_0.i2781.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.3.i6510, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.80.1 = phi float [ %_0.i2781.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.2.i6508, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.76.1 = phi float [ %_0.i2781.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.1.i6506, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.70.1 = phi float [ %_0.i2781, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.i6504, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.44.1 = phi float [ %_0.i2782.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.9.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.40.1 = phi float [ %_0.i2782.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.8.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.36.1 = phi float [ %_0.i2782.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.7.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.32.1 = phi float [ %_0.i2782.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.6.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.26.1 = phi float [ %_0.i2782.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.5.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.20.1 = phi float [ %_0.i2782.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.4.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.16.1 = phi float [ %_0.i2782.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.3.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.12.1 = phi float [ %_0.i2782.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.2.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.8.1 = phi float [ %_0.i2782.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.1.i, %bb17.i.i ], !dbg !5095
  %segments.i.i.sroa.0.1 = phi float [ %_0.i2782, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %_12.le.i, %bb17.i.i ], !dbg !5095
  %iter.sroa.0.0.i33.i.i11230 = phi i64 [ %259, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ 0, %bb17.i.i ]
  %position.sroa.0.0.i32.i.i11229 = phi i64 [ %_41.sroa.0.0.i40.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %258, %bb17.i.i ]
  %gain_far.i14.i.i.sroa.6.011228 = phi i32 [ %_3.i4363, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %257, %bb17.i.i ]
  %gain_far.i14.i.i.sroa.0.011227 = phi i32 [ %_3.i4367, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %256, %bb17.i.i ]
  %gain_near.i15.i.i.sroa.6.011226 = phi i32 [ %_3.i4371, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %255, %bb17.i.i ]
  %gain_near.i15.i.i.sroa.0.011225 = phi i32 [ %_3.i4375, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %254, %bb17.i.i ]
  %filter_far.i16.i.i.sroa.14.011224 = phi float [ %_0.i4180, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_far.i16.i.i.sroa.14.0.copyload, %bb17.i.i ]
  %filter_far.i16.i.i.sroa.11.011223 = phi float [ %_0.i4184, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_far.i16.i.i.sroa.11.0.copyload, %bb17.i.i ]
  %filter_far.i16.i.i.sroa.7.011222 = phi float [ %_0.i4172, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_far.i16.i.i.sroa.7.0.copyload, %bb17.i.i ]
  %filter_far.i16.i.i.sroa.0.011221 = phi float [ %_0.i4176, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_far.i16.i.i.sroa.0.0.copyload, %bb17.i.i ]
  %filter_near.i17.i.i.sroa.14.011220 = phi float [ %_0.i4196, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_near.i17.i.i.sroa.14.0.copyload, %bb17.i.i ]
  %filter_near.i17.i.i.sroa.11.011219 = phi float [ %_0.i4200, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_near.i17.i.i.sroa.11.0.copyload, %bb17.i.i ]
  %filter_near.i17.i.i.sroa.7.011218 = phi float [ %_0.i4188, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_near.i17.i.i.sroa.7.0.copyload, %bb17.i.i ]
  %filter_near.i17.i.i.sroa.0.011217 = phi float [ %_0.i4192, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], [ %filter_near.i17.i.i.sroa.0.0.copyload, %bb17.i.i ]
  %_0.i2782 = fadd float %segments.i.i.sroa.0.1, %_16.le.i, !dbg !5096
  %_0.i2781 = fadd float %segments.i.i.sroa.70.1, %_16.le.i6505, !dbg !5101
  %_0.i2782.1 = fadd float %segments.i.i.sroa.8.1, %_16.le.1.i, !dbg !5096
  %_0.i2781.1 = fadd float %segments.i.i.sroa.76.1, %_16.le.1.i6507, !dbg !5101
  %_0.i2782.2 = fadd float %segments.i.i.sroa.12.1, %_16.le.2.i, !dbg !5096
  %_0.i2781.2 = fadd float %segments.i.i.sroa.80.1, %_16.le.2.i6509, !dbg !5101
  %_0.i2782.3 = fadd float %segments.i.i.sroa.16.1, %_16.le.3.i, !dbg !5096
  %_0.i2781.3 = fadd float %segments.i.i.sroa.84.1, %_16.le.3.i6511, !dbg !5101
  %_0.i2782.4 = fadd float %segments.i.i.sroa.20.1, %_16.le.4.i, !dbg !5096
  %_0.i2781.4 = fadd float %segments.i.i.sroa.88.1, %_16.le.4.i6513, !dbg !5101
  %_0.i2782.5 = fadd float %segments.i.i.sroa.26.1, %_16.le.5.i, !dbg !5096
  %_0.i2781.5 = fadd float %segments.i.i.sroa.94.1, %_16.le.5.i6515, !dbg !5101
  %_0.i2782.6 = fadd float %segments.i.i.sroa.32.1, %_16.le.6.i, !dbg !5096
  %_0.i2781.6 = fadd float %segments.i.i.sroa.100.1, %_16.le.6.i6517, !dbg !5101
  %_0.i2782.7 = fadd float %segments.i.i.sroa.36.1, %_16.le.7.i, !dbg !5096
  %_0.i2781.7 = fadd float %segments.i.i.sroa.104.1, %_16.le.7.i6519, !dbg !5101
  %_0.i2782.8 = fadd float %segments.i.i.sroa.40.1, %_16.le.8.i, !dbg !5096
  %_0.i2781.8 = fadd float %segments.i.i.sroa.108.1, %_16.le.8.i6521, !dbg !5101
  %_0.i2782.9 = fadd float %segments.i.i.sroa.44.1, %_16.le.9.i, !dbg !5096
  %_0.i2781.9 = fadd float %segments.i.i.sroa.112.1, %_16.le.9.i6523, !dbg !5101
  %259 = add nuw i64 %iter.sroa.0.0.i33.i.i11230, 1, !dbg !5103
  %_42.i38.i.i = add i64 %position.sroa.0.0.i32.i.i11229, 1, !dbg !5109
  %_176.not.i39.i.i = icmp ult i64 %_42.i38.i.i, %ring_len.i.i, !dbg !5111
  %260 = select i1 %_176.not.i39.i.i, i64 0, i64 %ring_len.i.i, !dbg !5111
  %_41.sroa.0.0.i40.i.i = sub nuw i64 %_42.i38.i.i, %260, !dbg !5111
  %_184.i42.i.i = getelementptr inbounds nuw float, ptr %_75.i.i, i64 %iter.sroa.0.0.i33.i.i11230, !dbg !5114
  %_0.i3514 = load float, ptr %_184.i42.i.i, align 4, !dbg !5124, !alias.scope !5126, !noalias !5129, !noundef !11
  %_192.i47.i.i = getelementptr inbounds nuw float, ptr %_85.i.i, i64 %iter.sroa.0.0.i33.i.i11230, !dbg !5130
  %_0.i3509 = load float, ptr %_192.i47.i.i, align 4, !dbg !5139, !alias.scope !5141, !noalias !5129, !noundef !11
  %_7.i2 = load float, ptr %_67.i.i.i, align 4, !dbg !5144, !alias.scope !5147, !noalias !5150, !noundef !11
  %_8.i3 = load float, ptr %168, align 4, !dbg !5152, !alias.scope !5147, !noalias !5150, !noundef !11
  %_9.i4 = load float, ptr %169, align 4, !dbg !5153, !alias.scope !5147, !noalias !5150, !noundef !11
  %_0.i3411 = fsub float %_0.i3514, %filter_near.i17.i.i.sroa.7.011218, !dbg !5154
  %_0.i3222 = fmul float %_0.i3411, %_8.i3, !dbg !5157
  %_4.i2835 = fmul float %filter_near.i17.i.i.sroa.0.011217, %_7.i2, !dbg !5159
  %_0.i2836 = fadd float %_4.i2835, %_0.i3222, !dbg !5159
  %_0.i2654 = fadd float %filter_near.i17.i.i.sroa.0.011217, %_0.i2836, !dbg !5161
  %_0.i3221 = fmul float %filter_near.i17.i.i.sroa.0.011217, %_8.i3, !dbg !5163
  %_4.i2833 = fmul float %_0.i3411, %_9.i4, !dbg !5165
  %_0.i2834 = fadd float %_0.i3221, %_4.i2833, !dbg !5165
  %_0.i2653 = fadd float %filter_near.i17.i.i.sroa.7.011218, %_0.i2834, !dbg !5167
  %_0.i2652 = fadd float %_0.i2836, %_0.i2836, !dbg !5169
  %_0.i2651 = fadd float %filter_near.i17.i.i.sroa.0.011217, %_0.i2652, !dbg !5171
  %261 = tail call noundef float @llvm.fabs.f32(float %_0.i2651), !dbg !5173
  %262 = fcmp uge float %261, 0x3BC79CA100000000, !dbg !5176
  %_0.i4192 = select i1 %262, float %_0.i2651, float 0.000000e+00, !dbg !5178
  %_0.i2650 = fadd float %_0.i2834, %_0.i2834, !dbg !5179
  %_0.i2649 = fadd float %filter_near.i17.i.i.sroa.7.011218, %_0.i2650, !dbg !5181
  %263 = tail call noundef float @llvm.fabs.f32(float %_0.i2649), !dbg !5183
  %264 = fcmp uge float %263, 0x3BC79CA100000000, !dbg !5186
  %_0.i4188 = select i1 %264, float %_0.i2649, float 0.000000e+00, !dbg !5188
  %_12.i7 = load float, ptr %170, align 4, !dbg !5189, !alias.scope !5147, !noalias !5150, !noundef !11
  %_4.i2941 = fmul float %_12.i7, %_0.i2654, !dbg !5190
  %_0.i2942 = fadd float %_0.i3514, %_4.i2941, !dbg !5190
  %_0.i3412 = fsub float %_0.i2653, %filter_near.i17.i.i.sroa.14.011220, !dbg !5192
  %_0.i3224 = fmul float %_8.i3, %_0.i3412, !dbg !5195
  %_4.i2839 = fmul float %filter_near.i17.i.i.sroa.11.011219, %_7.i2, !dbg !5197
  %_0.i2840 = fadd float %_4.i2839, %_0.i3224, !dbg !5197
  %_0.i3223 = fmul float %filter_near.i17.i.i.sroa.11.011219, %_8.i3, !dbg !5199
  %_4.i2837 = fmul float %_9.i4, %_0.i3412, !dbg !5201
  %_0.i2838 = fadd float %_0.i3223, %_4.i2837, !dbg !5201
  %_0.i2659 = fadd float %filter_near.i17.i.i.sroa.14.011220, %_0.i2838, !dbg !5203
  %_0.i2658 = fadd float %_0.i2840, %_0.i2840, !dbg !5205
  %_0.i2657 = fadd float %filter_near.i17.i.i.sroa.11.011219, %_0.i2658, !dbg !5207
  %265 = tail call noundef float @llvm.fabs.f32(float %_0.i2657), !dbg !5209
  %266 = fcmp uge float %265, 0x3BC79CA100000000, !dbg !5212
  %_0.i4200 = select i1 %266, float %_0.i2657, float 0.000000e+00, !dbg !5214
  %_0.i2656 = fadd float %_0.i2838, %_0.i2838, !dbg !5215
  %_0.i2655 = fadd float %filter_near.i17.i.i.sroa.14.011220, %_0.i2656, !dbg !5217
  %267 = tail call noundef float @llvm.fabs.f32(float %_0.i2655), !dbg !5219
  %268 = fcmp uge float %267, 0x3BC79CA100000000, !dbg !5222
  %_0.i4196 = select i1 %268, float %_0.i2655, float 0.000000e+00, !dbg !5224
  %_0.i3443 = fsub float %_0.i2942, %_0.i2659, !dbg !5225
  %_7.i1 = load float, ptr %_72.i.i.i, align 4, !dbg !5227, !alias.scope !5230, !noalias !5233, !noundef !11
  %_8.i = load float, ptr %171, align 4, !dbg !5235, !alias.scope !5230, !noalias !5233, !noundef !11
  %_9.i = load float, ptr %172, align 4, !dbg !5236, !alias.scope !5230, !noalias !5233, !noundef !11
  %_0.i3409 = fsub float %_0.i3509, %filter_far.i16.i.i.sroa.7.011222, !dbg !5237
  %_0.i3218 = fmul float %_0.i3409, %_8.i, !dbg !5240
  %_4.i2827 = fmul float %filter_far.i16.i.i.sroa.0.011221, %_7.i1, !dbg !5242
  %_0.i2828 = fadd float %_4.i2827, %_0.i3218, !dbg !5242
  %_0.i2642 = fadd float %filter_far.i16.i.i.sroa.0.011221, %_0.i2828, !dbg !5244
  %_0.i3217 = fmul float %filter_far.i16.i.i.sroa.0.011221, %_8.i, !dbg !5246
  %_4.i2825 = fmul float %_0.i3409, %_9.i, !dbg !5248
  %_0.i2826 = fadd float %_0.i3217, %_4.i2825, !dbg !5248
  %_0.i2641 = fadd float %filter_far.i16.i.i.sroa.7.011222, %_0.i2826, !dbg !5250
  %_0.i2640 = fadd float %_0.i2828, %_0.i2828, !dbg !5252
  %_0.i2639 = fadd float %filter_far.i16.i.i.sroa.0.011221, %_0.i2640, !dbg !5254
  %269 = tail call noundef float @llvm.fabs.f32(float %_0.i2639), !dbg !5256
  %270 = fcmp uge float %269, 0x3BC79CA100000000, !dbg !5259
  %_0.i4176 = select i1 %270, float %_0.i2639, float 0.000000e+00, !dbg !5261
  %_0.i2638 = fadd float %_0.i2826, %_0.i2826, !dbg !5262
  %_0.i2637 = fadd float %filter_far.i16.i.i.sroa.7.011222, %_0.i2638, !dbg !5264
  %271 = tail call noundef float @llvm.fabs.f32(float %_0.i2637), !dbg !5266
  %272 = fcmp uge float %271, 0x3BC79CA100000000, !dbg !5269
  %_0.i4172 = select i1 %272, float %_0.i2637, float 0.000000e+00, !dbg !5271
  %_12.i = load float, ptr %173, align 4, !dbg !5272, !alias.scope !5230, !noalias !5233, !noundef !11
  %_4.i2943 = fmul float %_12.i, %_0.i2642, !dbg !5273
  %_0.i2944 = fadd float %_0.i3509, %_4.i2943, !dbg !5273
  %_0.i3410 = fsub float %_0.i2641, %filter_far.i16.i.i.sroa.14.011224, !dbg !5275
  %_0.i3220 = fmul float %_8.i, %_0.i3410, !dbg !5278
  %_4.i2831 = fmul float %filter_far.i16.i.i.sroa.11.011223, %_7.i1, !dbg !5280
  %_0.i2832 = fadd float %_4.i2831, %_0.i3220, !dbg !5280
  %_0.i3219 = fmul float %filter_far.i16.i.i.sroa.11.011223, %_8.i, !dbg !5282
  %_4.i2829 = fmul float %_9.i, %_0.i3410, !dbg !5284
  %_0.i2830 = fadd float %_0.i3219, %_4.i2829, !dbg !5284
  %_0.i2647 = fadd float %filter_far.i16.i.i.sroa.14.011224, %_0.i2830, !dbg !5286
  %_0.i2646 = fadd float %_0.i2832, %_0.i2832, !dbg !5288
  %_0.i2645 = fadd float %filter_far.i16.i.i.sroa.11.011223, %_0.i2646, !dbg !5290
  %273 = tail call noundef float @llvm.fabs.f32(float %_0.i2645), !dbg !5292
  %274 = fcmp uge float %273, 0x3BC79CA100000000, !dbg !5295
  %_0.i4184 = select i1 %274, float %_0.i2645, float 0.000000e+00, !dbg !5297
  %_0.i2644 = fadd float %_0.i2830, %_0.i2830, !dbg !5298
  %_0.i2643 = fadd float %filter_far.i16.i.i.sroa.14.011224, %_0.i2644, !dbg !5300
  %275 = tail call noundef float @llvm.fabs.f32(float %_0.i2643), !dbg !5302
  %276 = fcmp uge float %275, 0x3BC79CA100000000, !dbg !5305
  %_0.i4180 = select i1 %276, float %_0.i2643, float 0.000000e+00, !dbg !5307
  %_0.i3444 = fsub float %_0.i2944, %_0.i2647, !dbg !5308
  %_311.1.i54.i.i = load i64, ptr %174, align 8, !dbg !5310, !noalias !5129, !noundef !11
  %_234.i55.i.i = icmp ugt i64 %position.sroa.0.0.i32.i.i11229, %_311.1.i54.i.i, !dbg !5312
  br i1 %_234.i55.i.i, label %bb78.i153.i.i, label %bb79.i56.i.i, !dbg !5312, !prof !1664

bb79.i56.i.i:                                     ; preds = %bb56.i37.i.i
  %_311.0.i57.i.i = load ptr, ptr %123, align 8, !dbg !5310, !noalias !5129, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5318), !dbg !5321
  %_4.not.i4014 = icmp eq i64 %_311.1.i54.i.i, %position.sroa.0.0.i32.i.i11229, !dbg !5322
  br i1 %_4.not.i4014, label %panic.i4016, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4017, !dbg !5322

panic.i4016:                                      ; preds = %bb79.i56.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !5322, !noalias !5324
  unreachable, !dbg !5322

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4017: ; preds = %bb79.i56.i.i
  %_241.i60.i.i = getelementptr inbounds nuw float, ptr %_311.0.i57.i.i, i64 %position.sroa.0.0.i32.i.i11229, !dbg !5325
  store float %_0.i2659, ptr %_241.i60.i.i, align 4, !dbg !5322, !alias.scope !5318, !noalias !5129
  %_312.1.i61.i.i = load i64, ptr %176, align 8, !dbg !5331, !noalias !5129, !noundef !11
  %_242.i62.i.i = icmp ugt i64 %position.sroa.0.0.i32.i.i11229, %_312.1.i61.i.i, !dbg !5332
  br i1 %_242.i62.i.i, label %bb80.i152.i.i, label %bb81.i63.i.i, !dbg !5332, !prof !1664

bb78.i153.i.i:                                    ; preds = %bb56.i37.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i.i11229, i64 noundef %_311.1.i54.i.i, i64 noundef %_311.1.i54.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7515f6adc8a5521b72f04a3abb372e72) #26, !dbg !5336, !noalias !5129
  unreachable, !dbg !5336

bb81.i63.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4017
  %_312.0.i64.i.i = load ptr, ptr %175, align 8, !dbg !5331, !noalias !5129, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5337), !dbg !5340
  %_4.not.i4010 = icmp eq i64 %_312.1.i61.i.i, %position.sroa.0.0.i32.i.i11229, !dbg !5341
  br i1 %_4.not.i4010, label %panic.i4012, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4013, !dbg !5341

panic.i4012:                                      ; preds = %bb81.i63.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !5341, !noalias !5343
  unreachable, !dbg !5341

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4013: ; preds = %bb81.i63.i.i
  %_249.i66.i.i = getelementptr inbounds nuw float, ptr %_312.0.i64.i.i, i64 %position.sroa.0.0.i32.i.i11229, !dbg !5344
  store float %_0.i3443, ptr %_249.i66.i.i, align 4, !dbg !5341, !alias.scope !5337, !noalias !5129
  %_313.1.i67.i.i = load i64, ptr %177, align 8, !dbg !5349, !noalias !5129, !noundef !11
  %_250.i68.i.i = icmp ugt i64 %position.sroa.0.0.i32.i.i11229, %_313.1.i67.i.i, !dbg !5350
  br i1 %_250.i68.i.i, label %bb82.i151.i.i, label %bb83.i69.i.i, !dbg !5350, !prof !1664

bb80.i152.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4017
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i.i11229, i64 noundef %_312.1.i61.i.i, i64 noundef %_312.1.i61.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8c2aade3368450b16e7e4997ad3fca81) #26, !dbg !5354, !noalias !5129
  unreachable, !dbg !5354

bb83.i69.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4013
  %_313.0.i70.i.i = load ptr, ptr %_18.i.i, align 8, !dbg !5349, !noalias !5129, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5355), !dbg !5358
  %_4.not.i4006 = icmp eq i64 %_313.1.i67.i.i, %position.sroa.0.0.i32.i.i11229, !dbg !5359
  br i1 %_4.not.i4006, label %panic.i4008, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4009, !dbg !5359

panic.i4008:                                      ; preds = %bb83.i69.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !5359, !noalias !5361
  unreachable, !dbg !5359

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4009: ; preds = %bb83.i69.i.i
  %_257.i72.i.i = getelementptr inbounds nuw float, ptr %_313.0.i70.i.i, i64 %position.sroa.0.0.i32.i.i11229, !dbg !5362
  store float %_0.i2647, ptr %_257.i72.i.i, align 4, !dbg !5359, !alias.scope !5355, !noalias !5129
  %_314.1.i73.i.i = load i64, ptr %179, align 8, !dbg !5367, !noalias !5129, !noundef !11
  %_258.i74.i.i = icmp ugt i64 %position.sroa.0.0.i32.i.i11229, %_314.1.i73.i.i, !dbg !5368
  br i1 %_258.i74.i.i, label %bb84.i150.i.i, label %bb85.i75.i.i, !dbg !5368, !prof !1664

bb82.i151.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4013
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i.i11229, i64 noundef %_313.1.i67.i.i, i64 noundef %_313.1.i67.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a4c3da5a99763e9452b49d42852d67e3) #26, !dbg !5372, !noalias !5129
  unreachable, !dbg !5372

bb85.i75.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4009
  %_314.0.i76.i.i = load ptr, ptr %178, align 8, !dbg !5367, !noalias !5129, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5373), !dbg !5376
  %_4.not.i4002 = icmp eq i64 %_314.1.i73.i.i, %position.sroa.0.0.i32.i.i11229, !dbg !5377
  br i1 %_4.not.i4002, label %panic.i4004, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4005, !dbg !5377

panic.i4004:                                      ; preds = %bb85.i75.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !5377, !noalias !5379
  unreachable, !dbg !5377

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4005: ; preds = %bb85.i75.i.i
  %_265.i78.i.i = getelementptr inbounds nuw float, ptr %_314.0.i76.i.i, i64 %position.sroa.0.0.i32.i.i11229, !dbg !5380
  store float %_0.i3444, ptr %_265.i78.i.i, align 4, !dbg !5377, !alias.scope !5373, !noalias !5129
  %_315.0.i79.i.i = load ptr, ptr %123, align 8, !dbg !5385, !noalias !5129, !nonnull !11, !noundef !11
  %_315.1.i80.i.i = load i64, ptr %174, align 8, !dbg !5385, !noalias !5129, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5386), !dbg !5389
  %_8.i176.i.i = load i64, ptr %_24.i.i.i, align 8, !dbg !5390, !alias.scope !5386, !noalias !5392, !noundef !11
  %_7.i177.i.i = add i64 %_8.i176.i.i, %position.sroa.0.0.i32.i.i11229, !dbg !5394
  %_27.not.i178.i.i = icmp ult i64 %_7.i177.i.i, %ring_len.i.i, !dbg !5395
  %277 = select i1 %_27.not.i178.i.i, i64 0, i64 %ring_len.i.i, !dbg !5395
  %row.sroa.0.0.i179.i.i = sub nuw i64 %_7.i177.i.i, %277, !dbg !5395
  %_28.i180.i.i = icmp ugt i64 %row.sroa.0.0.i179.i.i, %_315.1.i80.i.i, !dbg !5397
  br i1 %_28.i180.i.i, label %bb14.i183.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i.i, !dbg !5397, !prof !1664

bb14.i183.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4005
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i179.i.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i80.i.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i80.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !5400, !noalias !5401
  unreachable, !dbg !5400

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4005
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5402), !dbg !5405
  %_3.not.i3502 = icmp eq i64 %_315.1.i80.i.i, %row.sroa.0.0.i179.i.i, !dbg !5406
  br i1 %_3.not.i3502, label %panic.i3505, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3506, !dbg !5406

panic.i3505:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5406, !noalias !5408
  unreachable, !dbg !5406

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3506: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i.i
  %_35.i182.i.i = getelementptr inbounds nuw float, ptr %_315.0.i79.i.i, i64 %row.sroa.0.0.i179.i.i, !dbg !5409
  %_0.i3504 = load float, ptr %_35.i182.i.i, align 4, !dbg !5406, !alias.scope !5402, !noalias !5411, !noundef !11
  %_316.0.i82.i.i = load ptr, ptr %175, align 8, !dbg !5412, !noalias !5129, !nonnull !11, !noundef !11
  %_316.1.i83.i.i = load i64, ptr %176, align 8, !dbg !5412, !noalias !5129, !noundef !11
  %_28.i171.i.i = icmp ugt i64 %row.sroa.0.0.i179.i.i, %_316.1.i83.i.i, !dbg !5414
  br i1 %_28.i171.i.i, label %bb14.i174.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i.i, !dbg !5414, !prof !1664

bb14.i174.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3506
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i179.i.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i83.i.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i83.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !5418, !noalias !5419
  unreachable, !dbg !5418

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3506
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5423), !dbg !5426
  %_3.not.i3497 = icmp eq i64 %_316.1.i83.i.i, %row.sroa.0.0.i179.i.i, !dbg !5427
  br i1 %_3.not.i3497, label %panic.i3500, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3501, !dbg !5427

panic.i3500:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5427, !noalias !5429
  unreachable, !dbg !5427

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3501: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i.i
  %_35.i173.i.i = getelementptr inbounds nuw float, ptr %_316.0.i82.i.i, i64 %row.sroa.0.0.i179.i.i, !dbg !5430
  %_0.i3499 = load float, ptr %_35.i173.i.i, align 4, !dbg !5427, !alias.scope !5423, !noalias !5432, !noundef !11
  %_317.0.i85.i.i = load ptr, ptr %_18.i.i, align 8, !dbg !5433, !noalias !5129, !nonnull !11, !noundef !11
  %_317.1.i86.i.i = load i64, ptr %177, align 8, !dbg !5433, !noalias !5129, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5435), !dbg !5438
  %_8.i158.i.i = load i64, ptr %_26.i.i.i, align 8, !dbg !5439, !alias.scope !5435, !noalias !5441, !noundef !11
  %_7.i159.i.i = add i64 %_8.i158.i.i, %position.sroa.0.0.i32.i.i11229, !dbg !5443
  %_27.not.i160.i.i = icmp ult i64 %_7.i159.i.i, %ring_len.i.i, !dbg !5444
  %278 = select i1 %_27.not.i160.i.i, i64 0, i64 %ring_len.i.i, !dbg !5444
  %row.sroa.0.0.i161.i.i = sub nuw i64 %_7.i159.i.i, %278, !dbg !5444
  %_28.i162.i.i = icmp ugt i64 %row.sroa.0.0.i161.i.i, %_317.1.i86.i.i, !dbg !5446
  br i1 %_28.i162.i.i, label %bb14.i165.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i.i, !dbg !5446, !prof !1664

bb14.i165.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3501
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i161.i.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i86.i.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i86.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !5449, !noalias !5450
  unreachable, !dbg !5449

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3501
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5451), !dbg !5454
  %_3.not.i3492 = icmp eq i64 %_317.1.i86.i.i, %row.sroa.0.0.i161.i.i, !dbg !5455
  br i1 %_3.not.i3492, label %panic.i3495, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3496, !dbg !5455

panic.i3495:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5455, !noalias !5457
  unreachable, !dbg !5455

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3496: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i.i
  %_35.i164.i.i = getelementptr inbounds nuw float, ptr %_317.0.i85.i.i, i64 %row.sroa.0.0.i161.i.i, !dbg !5458
  %_0.i3494 = load float, ptr %_35.i164.i.i, align 4, !dbg !5455, !alias.scope !5451, !noalias !5460, !noundef !11
  %_318.0.i88.i.i = load ptr, ptr %178, align 8, !dbg !5461, !noalias !5129, !nonnull !11, !noundef !11
  %_318.1.i89.i.i = load i64, ptr %179, align 8, !dbg !5461, !noalias !5129, !noundef !11
  %_28.i.i.i = icmp ugt i64 %row.sroa.0.0.i161.i.i, %_318.1.i89.i.i, !dbg !5463
  br i1 %_28.i.i.i, label %bb14.i.i.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i.i, !dbg !5463, !prof !1664

bb14.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3496
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i161.i.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i89.i.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i89.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !5467, !noalias !5468
  unreachable, !dbg !5467

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3496
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5472), !dbg !5475
  %_3.not.i3487 = icmp eq i64 %_318.1.i89.i.i, %row.sroa.0.0.i161.i.i, !dbg !5476
  br i1 %_3.not.i3487, label %panic.i3490, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3491, !dbg !5476

panic.i3490:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !5476, !noalias !5478
  unreachable, !dbg !5476

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3491: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i.i
  %_35.i157.i.i = getelementptr inbounds nuw float, ptr %_318.0.i88.i.i, i64 %row.sroa.0.0.i161.i.i, !dbg !5479
  %_0.i3489 = load float, ptr %_35.i157.i.i, align 4, !dbg !5476, !alias.scope !5472, !noalias !5481, !noundef !11
  %279 = tail call noundef float @llvm.fabs.f32(float %_0.i3504), !dbg !5482
  %_3.i.i5438 = fcmp ule float %279, 0x3E45798EE0000000, !dbg !5486
  %_6.i.i5440 = bitcast float %279 to i32, !dbg !5492
  %_4.i.i5444 = select i1 %_3.i.i5438, i32 841731191, i32 %_6.i.i5440, !dbg !5495
  %_0.i.i5445 = bitcast i32 %_4.i.i5444 to float, !dbg !5496
  %_3.i.i5090 = fcmp ule float %_0.i.i5445, 0x3810000000000000, !dbg !5498
  %_4.i.i5096 = select i1 %_3.i.i5090, i32 8388608, i32 %_4.i.i5444, !dbg !5503
  %_5.i3965 = and i32 %_4.i.i5096, 8388607, !dbg !5505
  %_4.i3966 = or disjoint i32 %_5.i3965, 1065353216, !dbg !5505
  %significand.i3967 = bitcast i32 %_4.i3966 to float, !dbg !5507
  %_0.i3357 = fadd float %significand.i3967, -1.000000e+00, !dbg !5509
  %_0.i3034 = fmul float %_0.i3357, 0xBF9B17A960000000, !dbg !5511
  %_0.i2554 = fadd float %_0.i3034, 0x3FBF9A8440000000, !dbg !5513
  %_0.i3034.1 = fmul float %_0.i3357, %_0.i2554, !dbg !5511
  %_0.i2554.1 = fadd float %_0.i3034.1, 0xBFD1E3F400000000, !dbg !5513
  %_0.i3034.2 = fmul float %_0.i3357, %_0.i2554.1, !dbg !5511
  %_0.i2554.2 = fadd float %_0.i3034.2, 0x3FDD544F20000000, !dbg !5513
  %_0.i3034.3 = fmul float %_0.i3357, %_0.i2554.2, !dbg !5511
  %_0.i2554.3 = fadd float %_0.i3034.3, 0xBFE6FC2A60000000, !dbg !5513
  %_0.i3034.4 = fmul float %_0.i3357, %_0.i2554.3, !dbg !5511
  %_0.i2554.4 = fadd float %_0.i3034.4, 0x3FF714B2A0000000, !dbg !5513
  %_9.i3968 = lshr i32 %_4.i.i5096, 23, !dbg !5515
  %_8.i3969 = or disjoint i32 %_9.i3968, 1258291200, !dbg !5515
  %_7.i3970 = bitcast i32 %_8.i3969 to float, !dbg !5516
  %exponent.i3971 = fadd float %_7.i3970, 0xC160000FE0000000, !dbg !5518
  %_0.i3033 = fmul float %_0.i3357, %_0.i2554.4, !dbg !5519
  %_0.i2553 = fadd float %exponent.i3971, %_0.i3033, !dbg !5521
  %_0.i3276 = fmul float %_0.i2553, 0x4018151820000000, !dbg !5523
  %_3.i.i5430.inv = fcmp ogt float %_0.i3276, -1.600000e+02, !dbg !5525
  %_0.i.i5437 = select i1 %_3.i.i5430.inv, float %_0.i3276, float -1.600000e+02, !dbg !5525
  %_3.i.i6172.inv = fcmp olt float %_0.i.i5437, 2.400000e+01, !dbg !5528
  %_0.i.i6179 = select i1 %_3.i.i6172.inv, float %_0.i.i5437, float 2.400000e+01, !dbg !5528
  %_0.i3405 = fsub float %_0.i.i6179, %_0.i2782, !dbg !5531
  %_3.i2265 = fcmp ule float %_0.i3405, 3.000000e+00, !dbg !5534
  %_0.i2629 = fadd float %_0.i3405, 3.000000e+00, !dbg !5536
  %_0.i3195 = fmul float %_0.i2629, %_0.i2629, !dbg !5538
  %_0.i3194 = fmul float %_0.i3195, 0x3FB5555560000000, !dbg !5540
  %_4.i4728.v.v = select i1 %_3.i2265, float %_0.i3194, float %_0.i3405, !dbg !5542
  %_4.i4728.v = fmul float %coefficients.i.i.sroa.0.0.copyload, %_4.i4728.v.v, !dbg !5542
  %_4.i4728 = bitcast float %_4.i4728.v to i32, !dbg !5542
  %280 = fcmp ugt float %_0.i3405, -3.000000e+00, !dbg !5544
  %_7.i4720 = select i1 %280, i32 %_4.i4728, i32 0, !dbg !5546
  %_0.i4722 = bitcast i32 %_7.i4720 to float, !dbg !5547
  %_3.i.i5422 = fcmp ule float %_0.i4722, -1.000000e+02, !dbg !5549
  %281 = bitcast i32 %_7.i4720 to float, !dbg !5552
  %_0.i.i5429 = select i1 %_3.i.i5422, float -1.000000e+02, float %281, !dbg !5555
  %_3.i.i6164 = fcmp olt float %_0.i.i5429, 0.000000e+00, !dbg !5556
  %_0.i.i6171 = select i1 %_3.i.i6164, float %_0.i.i5429, float 0.000000e+00, !dbg !5559
  %282 = bitcast i32 %gain_near.i15.i.i.sroa.0.011225 to float, !dbg !5561
  %_3.i2431 = fcmp uge float %_0.i.i6171, %282, !dbg !5562
  %_4.i4794.v = select i1 %_3.i2431, float %coefficients.i.i.sroa.7.0.copyload, float %coefficients.i.i.sroa.5.0.copyload, !dbg !5565
  %_0.i3448 = fsub float %282, %_0.i.i6171, !dbg !5567
  %_4.i2951 = fmul float %_0.i3448, %_4.i4794.v, !dbg !5569
  %_0.i2952 = fadd float %_0.i.i6171, %_4.i2951, !dbg !5569
  %283 = tail call noundef float @llvm.fabs.f32(float %_0.i2952), !dbg !5571
  %_4.i4373 = bitcast float %_0.i2952 to i32, !dbg !5574
  %284 = fcmp uge float %283, 0x3BC79CA100000000, !dbg !5577
  %_3.i4375 = select i1 %284, i32 %_4.i4373, i32 0, !dbg !5578
  %_0.i4376 = bitcast i32 %_3.i4375 to float, !dbg !5579
  %_0.i2788 = fadd float %_0.i2782.4, %_0.i4376, !dbg !5581
  %_0.i3275 = fmul float %_0.i2788, 0x3FC542A5A0000000, !dbg !5583
  %_3.i.i5282.inv = fcmp ogt float %_0.i3275, -1.260000e+02, !dbg !5586
  %_0.i.i5289 = select i1 %_3.i.i5282.inv, float %_0.i3275, float -1.260000e+02, !dbg !5586
  %_3.i.i6084.inv = fcmp olt float %_0.i.i5289, 1.270000e+02, !dbg !5590
  %_0.i.i6091 = select i1 %_3.i.i6084.inv, float %_0.i.i5289, float 1.270000e+02, !dbg !5590
  %285 = tail call noundef float @llvm.floor.f32(float %_0.i.i6091), !dbg !5593
  %_0.i3381 = fsub float %_0.i.i6091, %285, !dbg !5597
  %286 = tail call noundef float @llvm.fabs.f32(float %_0.i3499), !dbg !5599
  %_3.i.i5414 = fcmp ule float %286, 0x3E45798EE0000000, !dbg !5602
  %_6.i.i5416 = bitcast float %286 to i32, !dbg !5607
  %_4.i.i5420 = select i1 %_3.i.i5414, i32 841731191, i32 %_6.i.i5416, !dbg !5610
  %_0.i.i5421 = bitcast i32 %_4.i.i5420 to float, !dbg !5611
  %_3.i.i5098 = fcmp ule float %_0.i.i5421, 0x3810000000000000, !dbg !5613
  %_4.i.i5104 = select i1 %_3.i.i5098, i32 8388608, i32 %_4.i.i5420, !dbg !5618
  %_5.i3973 = and i32 %_4.i.i5104, 8388607, !dbg !5620
  %_4.i3974 = or disjoint i32 %_5.i3973, 1065353216, !dbg !5620
  %significand.i3975 = bitcast i32 %_4.i3974 to float, !dbg !5622
  %_0.i3358 = fadd float %significand.i3975, -1.000000e+00, !dbg !5624
  %_0.i3036 = fmul float %_0.i3358, 0xBF9B17A960000000, !dbg !5626
  %_0.i2556 = fadd float %_0.i3036, 0x3FBF9A8440000000, !dbg !5628
  %_0.i3036.1 = fmul float %_0.i3358, %_0.i2556, !dbg !5626
  %_0.i2556.1 = fadd float %_0.i3036.1, 0xBFD1E3F400000000, !dbg !5628
  %_0.i3036.2 = fmul float %_0.i3358, %_0.i2556.1, !dbg !5626
  %_0.i2556.2 = fadd float %_0.i3036.2, 0x3FDD544F20000000, !dbg !5628
  %_0.i3036.3 = fmul float %_0.i3358, %_0.i2556.2, !dbg !5626
  %_0.i2556.3 = fadd float %_0.i3036.3, 0xBFE6FC2A60000000, !dbg !5628
  %_0.i3036.4 = fmul float %_0.i3358, %_0.i2556.3, !dbg !5626
  %_0.i2556.4 = fadd float %_0.i3036.4, 0x3FF714B2A0000000, !dbg !5628
  %_9.i3976 = lshr i32 %_4.i.i5104, 23, !dbg !5630
  %_8.i3977 = or disjoint i32 %_9.i3976, 1258291200, !dbg !5630
  %_7.i3978 = bitcast i32 %_8.i3977 to float, !dbg !5631
  %exponent.i3979 = fadd float %_7.i3978, 0xC160000FE0000000, !dbg !5633
  %_0.i3035 = fmul float %_0.i3358, %_0.i2556.4, !dbg !5634
  %_0.i2555 = fadd float %exponent.i3979, %_0.i3035, !dbg !5636
  %_0.i3274 = fmul float %_0.i2555, 0x4018151820000000, !dbg !5638
  %_3.i.i5406.inv = fcmp ogt float %_0.i3274, -1.600000e+02, !dbg !5640
  %_0.i.i5413 = select i1 %_3.i.i5406.inv, float %_0.i3274, float -1.600000e+02, !dbg !5640
  %_3.i.i6156.inv = fcmp olt float %_0.i.i5413, 2.400000e+01, !dbg !5643
  %_0.i.i6163 = select i1 %_3.i.i6156.inv, float %_0.i.i5413, float 2.400000e+01, !dbg !5643
  %_0.i3406 = fsub float %_0.i.i6163, %_0.i2782.5, !dbg !5646
  %_3.i2267 = fcmp ule float %_0.i3406, 3.000000e+00, !dbg !5649
  %_0.i2630 = fadd float %_0.i3406, 3.000000e+00, !dbg !5651
  %_0.i3199 = fmul float %_0.i2630, %_0.i2630, !dbg !5653
  %_0.i3198 = fmul float %_0.i3199, 0x3FB5555560000000, !dbg !5655
  %_4.i4741.v.v = select i1 %_3.i2267, float %_0.i3198, float %_0.i3406, !dbg !5657
  %_4.i4741.v = fmul float %coefficients.i.i.sroa.9.0.copyload, %_4.i4741.v.v, !dbg !5657
  %_4.i4741 = bitcast float %_4.i4741.v to i32, !dbg !5657
  %287 = fcmp ugt float %_0.i3406, -3.000000e+00, !dbg !5659
  %_7.i4733 = select i1 %287, i32 %_4.i4741, i32 0, !dbg !5661
  %_0.i4735 = bitcast i32 %_7.i4733 to float, !dbg !5662
  %_3.i.i5398 = fcmp ule float %_0.i4735, -1.000000e+02, !dbg !5664
  %288 = bitcast i32 %_7.i4733 to float, !dbg !5667
  %_0.i.i5405 = select i1 %_3.i.i5398, float -1.000000e+02, float %288, !dbg !5670
  %_3.i.i6148 = fcmp olt float %_0.i.i5405, 0.000000e+00, !dbg !5671
  %_0.i.i6155 = select i1 %_3.i.i6148, float %_0.i.i5405, float 0.000000e+00, !dbg !5674
  %289 = bitcast i32 %gain_near.i15.i.i.sroa.6.011226 to float, !dbg !5676
  %_3.i2427 = fcmp uge float %_0.i.i6155, %289, !dbg !5677
  %_4.i4787.v = select i1 %_3.i2427, float %coefficients.i.i.sroa.13.0.copyload, float %coefficients.i.i.sroa.11.0.copyload, !dbg !5680
  %_0.i3447 = fsub float %289, %_0.i.i6155, !dbg !5682
  %_4.i2949 = fmul float %_0.i3447, %_4.i4787.v, !dbg !5684
  %_0.i2950 = fadd float %_0.i.i6155, %_4.i2949, !dbg !5684
  %290 = tail call noundef float @llvm.fabs.f32(float %_0.i2950), !dbg !5686
  %_4.i4369 = bitcast float %_0.i2950 to i32, !dbg !5689
  %291 = fcmp uge float %290, 0x3BC79CA100000000, !dbg !5692
  %_3.i4371 = select i1 %291, i32 %_4.i4369, i32 0, !dbg !5693
  %_0.i4372 = bitcast i32 %_3.i4371 to float, !dbg !5694
  %_0.i2787 = fadd float %_0.i2782.9, %_0.i4372, !dbg !5696
  %_0.i3273 = fmul float %_0.i2787, 0x3FC542A5A0000000, !dbg !5698
  %_3.i.i5290.inv = fcmp ogt float %_0.i3273, -1.260000e+02, !dbg !5701
  %_0.i.i5297 = select i1 %_3.i.i5290.inv, float %_0.i3273, float -1.260000e+02, !dbg !5701
  %_3.i.i6092.inv = fcmp olt float %_0.i.i5297, 1.270000e+02, !dbg !5705
  %_0.i.i6099 = select i1 %_3.i.i6092.inv, float %_0.i.i5297, float 1.270000e+02, !dbg !5705
  %292 = tail call noundef float @llvm.floor.f32(float %_0.i.i6099), !dbg !5708
  %_0.i3382 = fsub float %_0.i.i6099, %292, !dbg !5712
  %293 = tail call noundef float @llvm.fabs.f32(float %_0.i3494), !dbg !5714
  %_3.i.i5390 = fcmp ule float %293, 0x3E45798EE0000000, !dbg !5716
  %_6.i.i5392 = bitcast float %293 to i32, !dbg !5721
  %_4.i.i5396 = select i1 %_3.i.i5390, i32 841731191, i32 %_6.i.i5392, !dbg !5724
  %_0.i.i5397 = bitcast i32 %_4.i.i5396 to float, !dbg !5725
  %_3.i.i5106 = fcmp ule float %_0.i.i5397, 0x3810000000000000, !dbg !5727
  %_4.i.i5112 = select i1 %_3.i.i5106, i32 8388608, i32 %_4.i.i5396, !dbg !5732
  %_5.i3981 = and i32 %_4.i.i5112, 8388607, !dbg !5734
  %_4.i3982 = or disjoint i32 %_5.i3981, 1065353216, !dbg !5734
  %significand.i3983 = bitcast i32 %_4.i3982 to float, !dbg !5736
  %_0.i3359 = fadd float %significand.i3983, -1.000000e+00, !dbg !5738
  %_0.i3038 = fmul float %_0.i3359, 0xBF9B17A960000000, !dbg !5740
  %_0.i2558 = fadd float %_0.i3038, 0x3FBF9A8440000000, !dbg !5742
  %_0.i3038.1 = fmul float %_0.i3359, %_0.i2558, !dbg !5740
  %_0.i2558.1 = fadd float %_0.i3038.1, 0xBFD1E3F400000000, !dbg !5742
  %_0.i3038.2 = fmul float %_0.i3359, %_0.i2558.1, !dbg !5740
  %_0.i2558.2 = fadd float %_0.i3038.2, 0x3FDD544F20000000, !dbg !5742
  %_0.i3038.3 = fmul float %_0.i3359, %_0.i2558.2, !dbg !5740
  %_0.i2558.3 = fadd float %_0.i3038.3, 0xBFE6FC2A60000000, !dbg !5742
  %_0.i3038.4 = fmul float %_0.i3359, %_0.i2558.3, !dbg !5740
  %_0.i2558.4 = fadd float %_0.i3038.4, 0x3FF714B2A0000000, !dbg !5742
  %_9.i3984 = lshr i32 %_4.i.i5112, 23, !dbg !5744
  %_8.i3985 = or disjoint i32 %_9.i3984, 1258291200, !dbg !5744
  %_7.i3986 = bitcast i32 %_8.i3985 to float, !dbg !5745
  %exponent.i3987 = fadd float %_7.i3986, 0xC160000FE0000000, !dbg !5747
  %_0.i3037 = fmul float %_0.i3359, %_0.i2558.4, !dbg !5748
  %_0.i2557 = fadd float %exponent.i3987, %_0.i3037, !dbg !5750
  %_0.i3272 = fmul float %_0.i2557, 0x4018151820000000, !dbg !5752
  %_3.i.i5382.inv = fcmp ogt float %_0.i3272, -1.600000e+02, !dbg !5754
  %_0.i.i5389 = select i1 %_3.i.i5382.inv, float %_0.i3272, float -1.600000e+02, !dbg !5754
  %_3.i.i6140.inv = fcmp olt float %_0.i.i5389, 2.400000e+01, !dbg !5757
  %_0.i.i6147 = select i1 %_3.i.i6140.inv, float %_0.i.i5389, float 2.400000e+01, !dbg !5757
  %_0.i3407 = fsub float %_0.i.i6147, %_0.i2781, !dbg !5760
  %_3.i2269 = fcmp ule float %_0.i3407, 3.000000e+00, !dbg !5763
  %_0.i2631 = fadd float %_0.i3407, 3.000000e+00, !dbg !5765
  %_0.i3203 = fmul float %_0.i2631, %_0.i2631, !dbg !5767
  %_0.i3202 = fmul float %_0.i3203, 0x3FB5555560000000, !dbg !5769
  %_4.i4754.v.v = select i1 %_3.i2269, float %_0.i3202, float %_0.i3407, !dbg !5771
  %_4.i4754.v = fmul float %coefficients.i.i.sroa.15.24.copyload, %_4.i4754.v.v, !dbg !5771
  %_4.i4754 = bitcast float %_4.i4754.v to i32, !dbg !5771
  %294 = fcmp ugt float %_0.i3407, -3.000000e+00, !dbg !5773
  %_7.i4746 = select i1 %294, i32 %_4.i4754, i32 0, !dbg !5775
  %_0.i4748 = bitcast i32 %_7.i4746 to float, !dbg !5776
  %_3.i.i5374 = fcmp ule float %_0.i4748, -1.000000e+02, !dbg !5778
  %295 = bitcast i32 %_7.i4746 to float, !dbg !5781
  %_0.i.i5381 = select i1 %_3.i.i5374, float -1.000000e+02, float %295, !dbg !5784
  %_3.i.i6132 = fcmp olt float %_0.i.i5381, 0.000000e+00, !dbg !5785
  %_0.i.i6139 = select i1 %_3.i.i6132, float %_0.i.i5381, float 0.000000e+00, !dbg !5788
  %296 = bitcast i32 %gain_far.i14.i.i.sroa.0.011227 to float, !dbg !5790
  %_3.i2423 = fcmp uge float %_0.i.i6139, %296, !dbg !5791
  %_4.i4780.v = select i1 %_3.i2423, float %coefficients.i.i.sroa.20.24.copyload, float %coefficients.i.i.sroa.18.24.copyload, !dbg !5794
  %_0.i3446 = fsub float %296, %_0.i.i6139, !dbg !5796
  %_4.i2947 = fmul float %_0.i3446, %_4.i4780.v, !dbg !5798
  %_0.i2948 = fadd float %_0.i.i6139, %_4.i2947, !dbg !5798
  %297 = tail call noundef float @llvm.fabs.f32(float %_0.i2948), !dbg !5800
  %_4.i4365 = bitcast float %_0.i2948 to i32, !dbg !5803
  %298 = fcmp uge float %297, 0x3BC79CA100000000, !dbg !5806
  %_3.i4367 = select i1 %298, i32 %_4.i4365, i32 0, !dbg !5807
  %_0.i4368 = bitcast i32 %_3.i4367 to float, !dbg !5808
  %_0.i2786 = fadd float %_0.i2781.4, %_0.i4368, !dbg !5810
  %_0.i3271 = fmul float %_0.i2786, 0x3FC542A5A0000000, !dbg !5812
  %_3.i.i5298.inv = fcmp ogt float %_0.i3271, -1.260000e+02, !dbg !5815
  %_0.i.i5305 = select i1 %_3.i.i5298.inv, float %_0.i3271, float -1.260000e+02, !dbg !5815
  %_3.i.i6100.inv = fcmp olt float %_0.i.i5305, 1.270000e+02, !dbg !5819
  %_0.i.i6107 = select i1 %_3.i.i6100.inv, float %_0.i.i5305, float 1.270000e+02, !dbg !5819
  %299 = tail call noundef float @llvm.floor.f32(float %_0.i.i6107), !dbg !5822
  %_0.i3383 = fsub float %_0.i.i6107, %299, !dbg !5826
  %300 = tail call noundef float @llvm.fabs.f32(float %_0.i3489), !dbg !5828
  %_3.i.i5366 = fcmp ule float %300, 0x3E45798EE0000000, !dbg !5830
  %_6.i.i5368 = bitcast float %300 to i32, !dbg !5835
  %_4.i.i5372 = select i1 %_3.i.i5366, i32 841731191, i32 %_6.i.i5368, !dbg !5838
  %_0.i.i5373 = bitcast i32 %_4.i.i5372 to float, !dbg !5839
  %_3.i.i5114 = fcmp ule float %_0.i.i5373, 0x3810000000000000, !dbg !5841
  %_4.i.i5120 = select i1 %_3.i.i5114, i32 8388608, i32 %_4.i.i5372, !dbg !5846
  %_5.i3989 = and i32 %_4.i.i5120, 8388607, !dbg !5848
  %_4.i3990 = or disjoint i32 %_5.i3989, 1065353216, !dbg !5848
  %significand.i3991 = bitcast i32 %_4.i3990 to float, !dbg !5850
  %_0.i3360 = fadd float %significand.i3991, -1.000000e+00, !dbg !5852
  %_0.i3040 = fmul float %_0.i3360, 0xBF9B17A960000000, !dbg !5854
  %_0.i2560 = fadd float %_0.i3040, 0x3FBF9A8440000000, !dbg !5856
  %_0.i3040.1 = fmul float %_0.i3360, %_0.i2560, !dbg !5854
  %_0.i2560.1 = fadd float %_0.i3040.1, 0xBFD1E3F400000000, !dbg !5856
  %_0.i3040.2 = fmul float %_0.i3360, %_0.i2560.1, !dbg !5854
  %_0.i2560.2 = fadd float %_0.i3040.2, 0x3FDD544F20000000, !dbg !5856
  %_0.i3040.3 = fmul float %_0.i3360, %_0.i2560.2, !dbg !5854
  %_0.i2560.3 = fadd float %_0.i3040.3, 0xBFE6FC2A60000000, !dbg !5856
  %_0.i3040.4 = fmul float %_0.i3360, %_0.i2560.3, !dbg !5854
  %_0.i2560.4 = fadd float %_0.i3040.4, 0x3FF714B2A0000000, !dbg !5856
  %_9.i3992 = lshr i32 %_4.i.i5120, 23, !dbg !5858
  %_8.i3993 = or disjoint i32 %_9.i3992, 1258291200, !dbg !5858
  %_7.i3994 = bitcast i32 %_8.i3993 to float, !dbg !5859
  %exponent.i3995 = fadd float %_7.i3994, 0xC160000FE0000000, !dbg !5861
  %_0.i3039 = fmul float %_0.i3360, %_0.i2560.4, !dbg !5862
  %_0.i2559 = fadd float %exponent.i3995, %_0.i3039, !dbg !5864
  %_0.i3270 = fmul float %_0.i2559, 0x4018151820000000, !dbg !5866
  %_3.i.i5358.inv = fcmp ogt float %_0.i3270, -1.600000e+02, !dbg !5868
  %_0.i.i5365 = select i1 %_3.i.i5358.inv, float %_0.i3270, float -1.600000e+02, !dbg !5868
  %_3.i.i6124.inv = fcmp olt float %_0.i.i5365, 2.400000e+01, !dbg !5871
  %_0.i.i6131 = select i1 %_3.i.i6124.inv, float %_0.i.i5365, float 2.400000e+01, !dbg !5871
  %_0.i3408 = fsub float %_0.i.i6131, %_0.i2781.5, !dbg !5874
  %_3.i2271 = fcmp ule float %_0.i3408, 3.000000e+00, !dbg !5877
  %_0.i2632 = fadd float %_0.i3408, 3.000000e+00, !dbg !5879
  %_0.i3207 = fmul float %_0.i2632, %_0.i2632, !dbg !5881
  %_0.i3206 = fmul float %_0.i3207, 0x3FB5555560000000, !dbg !5883
  %_4.i4767.v.v = select i1 %_3.i2271, float %_0.i3206, float %_0.i3408, !dbg !5885
  %_4.i4767.v = fmul float %coefficients.i.i.sroa.22.24.copyload, %_4.i4767.v.v, !dbg !5885
  %_4.i4767 = bitcast float %_4.i4767.v to i32, !dbg !5885
  %301 = fcmp ugt float %_0.i3408, -3.000000e+00, !dbg !5887
  %_7.i4759 = select i1 %301, i32 %_4.i4767, i32 0, !dbg !5889
  %_0.i4761 = bitcast i32 %_7.i4759 to float, !dbg !5890
  %_3.i.i5350 = fcmp ule float %_0.i4761, -1.000000e+02, !dbg !5892
  %302 = bitcast i32 %_7.i4759 to float, !dbg !5895
  %_0.i.i5357 = select i1 %_3.i.i5350, float -1.000000e+02, float %302, !dbg !5898
  %_3.i.i6116 = fcmp olt float %_0.i.i5357, 0.000000e+00, !dbg !5899
  %_0.i.i6123 = select i1 %_3.i.i6116, float %_0.i.i5357, float 0.000000e+00, !dbg !5902
  %303 = bitcast i32 %gain_far.i14.i.i.sroa.6.011228 to float, !dbg !5904
  %_3.i2419 = fcmp uge float %_0.i.i6123, %303, !dbg !5905
  %_4.i4773.v = select i1 %_3.i2419, float %coefficients.i.i.sroa.26.24.copyload, float %coefficients.i.i.sroa.24.24.copyload, !dbg !5908
  %_0.i3445 = fsub float %303, %_0.i.i6123, !dbg !5910
  %_4.i2945 = fmul float %_0.i3445, %_4.i4773.v, !dbg !5912
  %_0.i2946 = fadd float %_0.i.i6123, %_4.i2945, !dbg !5912
  %304 = tail call noundef float @llvm.fabs.f32(float %_0.i2946), !dbg !5914
  %_4.i4361 = bitcast float %_0.i2946 to i32, !dbg !5917
  %305 = fcmp uge float %304, 0x3BC79CA100000000, !dbg !5920
  %_3.i4363 = select i1 %305, i32 %_4.i4361, i32 0, !dbg !5921
  %_0.i4364 = bitcast i32 %_3.i4363 to float, !dbg !5922
  %_0.i2785 = fadd float %_0.i2781.9, %_0.i4364, !dbg !5924
  %_0.i3269 = fmul float %_0.i2785, 0x3FC542A5A0000000, !dbg !5926
  %_3.i.i5306.inv = fcmp ogt float %_0.i3269, -1.260000e+02, !dbg !5929
  %_0.i.i5313 = select i1 %_3.i.i5306.inv, float %_0.i3269, float -1.260000e+02, !dbg !5929
  %_3.i.i6108.inv = fcmp olt float %_0.i.i5313, 1.270000e+02, !dbg !5933
  %_0.i.i6115 = select i1 %_3.i.i6108.inv, float %_0.i.i5313, float 1.270000e+02, !dbg !5933
  %306 = tail call noundef float @llvm.floor.f32(float %_0.i.i6115), !dbg !5936
  %_0.i3384 = fsub float %_0.i.i6115, %306, !dbg !5940
  %_0.i3112 = fmul float %_0.i3384, 0x3F5E974FA0000000, !dbg !5942
  %_0.i2608 = fadd float %_0.i3112, 0x3F82778560000000, !dbg !5944
  %_0.i3112.1 = fmul float %_0.i3384, %_0.i2608, !dbg !5942
  %_0.i2608.1 = fadd float %_0.i3112.1, 0x3FAC91CE60000000, !dbg !5944
  %_0.i3112.2 = fmul float %_0.i3384, %_0.i2608.1, !dbg !5942
  %_0.i2608.2 = fadd float %_0.i3112.2, 0x3FCEBDB560000000, !dbg !5944
  %_0.i3112.3 = fmul float %_0.i3384, %_0.i2608.2, !dbg !5942
  %_0.i2608.3 = fadd float %_0.i3112.3, 0x3FE62E4BA0000000, !dbg !5944
  %_0.i3109 = fmul float %_0.i3383, 0x3F5E974FA0000000, !dbg !5946
  %_0.i2606 = fadd float %_0.i3109, 0x3F82778560000000, !dbg !5948
  %_0.i3109.1 = fmul float %_0.i3383, %_0.i2606, !dbg !5946
  %_0.i2606.1 = fadd float %_0.i3109.1, 0x3FAC91CE60000000, !dbg !5948
  %_0.i3109.2 = fmul float %_0.i3383, %_0.i2606.1, !dbg !5946
  %_0.i2606.2 = fadd float %_0.i3109.2, 0x3FCEBDB560000000, !dbg !5948
  %_0.i3109.3 = fmul float %_0.i3383, %_0.i2606.2, !dbg !5946
  %_0.i2606.3 = fadd float %_0.i3109.3, 0x3FE62E4BA0000000, !dbg !5948
  %_0.i3106 = fmul float %_0.i3382, 0x3F5E974FA0000000, !dbg !5950
  %_0.i2604 = fadd float %_0.i3106, 0x3F82778560000000, !dbg !5952
  %_0.i3106.1 = fmul float %_0.i3382, %_0.i2604, !dbg !5950
  %_0.i2604.1 = fadd float %_0.i3106.1, 0x3FAC91CE60000000, !dbg !5952
  %_0.i3106.2 = fmul float %_0.i3382, %_0.i2604.1, !dbg !5950
  %_0.i2604.2 = fadd float %_0.i3106.2, 0x3FCEBDB560000000, !dbg !5952
  %_0.i3106.3 = fmul float %_0.i3382, %_0.i2604.2, !dbg !5950
  %_0.i2604.3 = fadd float %_0.i3106.3, 0x3FE62E4BA0000000, !dbg !5952
  %_0.i3103 = fmul float %_0.i3381, 0x3F5E974FA0000000, !dbg !5954
  %_0.i2602 = fadd float %_0.i3103, 0x3F82778560000000, !dbg !5956
  %_0.i3103.1 = fmul float %_0.i3381, %_0.i2602, !dbg !5954
  %_0.i2602.1 = fadd float %_0.i3103.1, 0x3FAC91CE60000000, !dbg !5956
  %_0.i3103.2 = fmul float %_0.i3381, %_0.i2602.1, !dbg !5954
  %_0.i2602.2 = fadd float %_0.i3103.2, 0x3FCEBDB560000000, !dbg !5956
  %_0.i3103.3 = fmul float %_0.i3381, %_0.i2602.2, !dbg !5954
  %_0.i2602.3 = fadd float %_0.i3103.3, 0x3FE62E4BA0000000, !dbg !5956
  %_0.i3102 = fmul float %_0.i3381, %_0.i2602.3, !dbg !5958
  %_0.i2601 = fadd float %_0.i3102, 1.000000e+00, !dbg !5960
  %biased.i2210 = fadd float %285, 0x4160000FE0000000, !dbg !5962
  %_4.i2211 = bitcast float %biased.i2210 to i32, !dbg !5964
  %_3.i2212 = shl i32 %_4.i2211, 23, !dbg !5966
  %_0.i2213 = bitcast i32 %_3.i2212 to float, !dbg !5967
  %_0.i3101 = fmul float %_0.i2601, %_0.i2213, !dbg !5969
  %_0.i3105 = fmul float %_0.i3382, %_0.i2604.3, !dbg !5971
  %_0.i2603 = fadd float %_0.i3105, 1.000000e+00, !dbg !5973
  %biased.i2214 = fadd float %292, 0x4160000FE0000000, !dbg !5975
  %_4.i2215 = bitcast float %biased.i2214 to i32, !dbg !5977
  %_3.i2216 = shl i32 %_4.i2215, 23, !dbg !5979
  %_0.i2217 = bitcast i32 %_3.i2216 to float, !dbg !5980
  %_0.i3104 = fmul float %_0.i2603, %_0.i2217, !dbg !5982
  %_0.i3108 = fmul float %_0.i3383, %_0.i2606.3, !dbg !5984
  %_0.i2605 = fadd float %_0.i3108, 1.000000e+00, !dbg !5986
  %biased.i2218 = fadd float %299, 0x4160000FE0000000, !dbg !5988
  %_4.i2219 = bitcast float %biased.i2218 to i32, !dbg !5990
  %_3.i2220 = shl i32 %_4.i2219, 23, !dbg !5992
  %_0.i2221 = bitcast i32 %_3.i2220 to float, !dbg !5993
  %_0.i3107 = fmul float %_0.i2605, %_0.i2221, !dbg !5995
  %_0.i3111 = fmul float %_0.i3384, %_0.i2608.3, !dbg !5997
  %_0.i2607 = fadd float %_0.i3111, 1.000000e+00, !dbg !5999
  %biased.i2222 = fadd float %306, 0x4160000FE0000000, !dbg !6001
  %_4.i2223 = bitcast float %biased.i2222 to i32, !dbg !6003
  %_3.i2224 = shl i32 %_4.i2223, 23, !dbg !6005
  %_0.i2225 = bitcast i32 %_3.i2224 to float, !dbg !6006
  %_0.i3110 = fmul float %_0.i2607, %_0.i2225, !dbg !6008
  %_266.i112.i.i = icmp ugt i64 %_41.sroa.0.0.i40.i.i, %_315.1.i80.i.i, !dbg !6010
  br i1 %_266.i112.i.i, label %bb86.i149.i.i, label %bb87.i113.i.i, !dbg !6010, !prof !1664

bb84.i150.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4009
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i.i11229, i64 noundef %_314.1.i73.i.i, i64 noundef %_314.1.i73.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e11ba0c4c1124bbcae4603a2ae121338) #26, !dbg !6015, !noalias !5129
  unreachable, !dbg !6015

bb87.i113.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3491
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6016), !dbg !6019
  %_3.not.i3482 = icmp eq i64 %_315.1.i80.i.i, %_41.sroa.0.0.i40.i.i, !dbg !6020
  br i1 %_3.not.i3482, label %panic.i3485, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3486, !dbg !6020

panic.i3485:                                      ; preds = %bb87.i113.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6020, !noalias !6022
  unreachable, !dbg !6020

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3486: ; preds = %bb87.i113.i.i
  %_273.i116.i.i = getelementptr inbounds nuw float, ptr %_315.0.i79.i.i, i64 %_41.sroa.0.0.i40.i.i, !dbg !6023
  %_0.i3484 = load float, ptr %_273.i116.i.i, align 4, !dbg !6020, !alias.scope !6016, !noalias !5129, !noundef !11
  %_0.i3268 = fmul float %_0.i3101, %_0.i3484, !dbg !6028
  %_274.i120.i.i = icmp ugt i64 %_41.sroa.0.0.i40.i.i, %_316.1.i83.i.i, !dbg !6030
  br i1 %_274.i120.i.i, label %bb88.i148.i.i, label %bb89.i121.i.i, !dbg !6030, !prof !1664

bb86.i149.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3491
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i.i, i64 noundef %_315.1.i80.i.i, i64 noundef %_315.1.i80.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_379b93204ee4659b79f4b07b6520076a) #26, !dbg !6034, !noalias !5129
  unreachable, !dbg !6034

bb89.i121.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3486
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6035), !dbg !6038
  %_3.not.i3477 = icmp eq i64 %_316.1.i83.i.i, %_41.sroa.0.0.i40.i.i, !dbg !6039
  br i1 %_3.not.i3477, label %panic.i3480, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3481, !dbg !6039

panic.i3480:                                      ; preds = %bb89.i121.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6039, !noalias !6041
  unreachable, !dbg !6039

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3481: ; preds = %bb89.i121.i.i
  %_281.i124.i.i = getelementptr inbounds nuw float, ptr %_316.0.i82.i.i, i64 %_41.sroa.0.0.i40.i.i, !dbg !6042
  %_0.i3479 = load float, ptr %_281.i124.i.i, align 4, !dbg !6039, !alias.scope !6035, !noalias !5129, !noundef !11
  %_0.i3267 = fmul float %_0.i3104, %_0.i3479, !dbg !6047
  %_0.i2784 = fadd float %_0.i3268, %_0.i3267, !dbg !6049
  %_282.i129.i.i = icmp ugt i64 %_41.sroa.0.0.i40.i.i, %_317.1.i86.i.i, !dbg !6051
  br i1 %_282.i129.i.i, label %bb90.i147.i.i, label %bb91.i130.i.i, !dbg !6051, !prof !1664

bb88.i148.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3486
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i.i, i64 noundef %_316.1.i83.i.i, i64 noundef %_316.1.i83.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2bb8eb0542a889f29ef069491eb97a17) #26, !dbg !6056, !noalias !5129
  unreachable, !dbg !6056

bb91.i130.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3481
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6057), !dbg !6060
  %_3.not.i3472 = icmp eq i64 %_317.1.i86.i.i, %_41.sroa.0.0.i40.i.i, !dbg !6061
  br i1 %_3.not.i3472, label %panic.i3475, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3476, !dbg !6061

panic.i3475:                                      ; preds = %bb91.i130.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6061, !noalias !6063
  unreachable, !dbg !6061

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3476: ; preds = %bb91.i130.i.i
  %_289.i133.i.i = getelementptr inbounds nuw float, ptr %_317.0.i85.i.i, i64 %_41.sroa.0.0.i40.i.i, !dbg !6064
  %_0.i3474 = load float, ptr %_289.i133.i.i, align 4, !dbg !6061, !alias.scope !6057, !noalias !5129, !noundef !11
  %_0.i3266 = fmul float %_0.i3107, %_0.i3474, !dbg !6069
  %_290.i137.i.i = icmp ugt i64 %_41.sroa.0.0.i40.i.i, %_318.1.i89.i.i, !dbg !6071
  br i1 %_290.i137.i.i, label %bb92.i146.i.i, label %bb93.i138.i.i, !dbg !6071, !prof !1664

bb90.i147.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3481
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i.i, i64 noundef %_317.1.i86.i.i, i64 noundef %_317.1.i86.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd96798e807157d7db308788cb4dd125) #26, !dbg !6075, !noalias !5129
  unreachable, !dbg !6075

bb93.i138.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3476
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6076), !dbg !6079
  %_3.not.i = icmp eq i64 %_318.1.i89.i.i, %_41.sroa.0.0.i40.i.i, !dbg !6080
  br i1 %_3.not.i, label %panic.i3471, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit, !dbg !6080

panic.i3471:                                      ; preds = %bb93.i138.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6080, !noalias !6082
  unreachable, !dbg !6080

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit: ; preds = %bb93.i138.i.i
  %_297.i141.i.i = getelementptr inbounds nuw float, ptr %_318.0.i88.i.i, i64 %_41.sroa.0.0.i40.i.i, !dbg !6083
  %_0.i3470 = load float, ptr %_297.i141.i.i, align 4, !dbg !6080, !alias.scope !6076, !noalias !5129, !noundef !11
  %_0.i3265 = fmul float %_0.i3110, %_0.i3470, !dbg !6088
  %_0.i2783 = fadd float %_0.i3266, %_0.i3265, !dbg !6090
  store float %_0.i2784, ptr %_184.i42.i.i, align 4, !dbg !6092, !alias.scope !6095, !noalias !5129
  store float %_0.i2783, ptr %_192.i47.i.i, align 4, !dbg !6098, !alias.scope !6100, !noalias !5129
  %exitcond14312.not = icmp eq i64 %259, %plan.0.i.i, !dbg !5081
  br i1 %exitcond14312.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_Kb1_EB2_.exit.i.i, label %bb56.i37.i.i, !dbg !5093

bb92.i146.i.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3476
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i.i, i64 noundef %_318.1.i89.i.i, i64 noundef %_318.1.i89.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_164adf6876ccf79975d46e129e248ca5) #26, !dbg !6103, !noalias !5129
  unreachable, !dbg !6103

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb0_Kb1_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit, %bb17.i.i
  %segments.i.i.sroa.112.0 = phi float [ %_12.le.9.i6522, %bb17.i.i ], [ %_0.i2781.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.108.0 = phi float [ %_12.le.8.i6520, %bb17.i.i ], [ %_0.i2781.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.104.0 = phi float [ %_12.le.7.i6518, %bb17.i.i ], [ %_0.i2781.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.100.0 = phi float [ %_12.le.6.i6516, %bb17.i.i ], [ %_0.i2781.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.94.0 = phi float [ %_12.le.5.i6514, %bb17.i.i ], [ %_0.i2781.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.88.0 = phi float [ %_12.le.4.i6512, %bb17.i.i ], [ %_0.i2781.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.84.0 = phi float [ %_12.le.3.i6510, %bb17.i.i ], [ %_0.i2781.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.80.0 = phi float [ %_12.le.2.i6508, %bb17.i.i ], [ %_0.i2781.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.76.0 = phi float [ %_12.le.1.i6506, %bb17.i.i ], [ %_0.i2781.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.70.0 = phi float [ %_12.le.i6504, %bb17.i.i ], [ %_0.i2781, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.44.0 = phi float [ %_12.le.9.i, %bb17.i.i ], [ %_0.i2782.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.40.0 = phi float [ %_12.le.8.i, %bb17.i.i ], [ %_0.i2782.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.36.0 = phi float [ %_12.le.7.i, %bb17.i.i ], [ %_0.i2782.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.32.0 = phi float [ %_12.le.6.i, %bb17.i.i ], [ %_0.i2782.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.26.0 = phi float [ %_12.le.5.i, %bb17.i.i ], [ %_0.i2782.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.20.0 = phi float [ %_12.le.4.i, %bb17.i.i ], [ %_0.i2782.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.16.0 = phi float [ %_12.le.3.i, %bb17.i.i ], [ %_0.i2782.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.12.0 = phi float [ %_12.le.2.i, %bb17.i.i ], [ %_0.i2782.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.8.0 = phi float [ %_12.le.1.i, %bb17.i.i ], [ %_0.i2782.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %segments.i.i.sroa.0.0 = phi float [ %_12.le.i, %bb17.i.i ], [ %_0.i2782, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !5095
  %filter_near.i17.i.i.sroa.0.0.lcssa = phi float [ %filter_near.i17.i.i.sroa.0.0.copyload, %bb17.i.i ], [ %_0.i4192, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6104
  %filter_near.i17.i.i.sroa.7.0.lcssa = phi float [ %filter_near.i17.i.i.sroa.7.0.copyload, %bb17.i.i ], [ %_0.i4188, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6104
  %filter_near.i17.i.i.sroa.11.0.lcssa = phi float [ %filter_near.i17.i.i.sroa.11.0.copyload, %bb17.i.i ], [ %_0.i4200, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6104
  %filter_near.i17.i.i.sroa.14.0.lcssa = phi float [ %filter_near.i17.i.i.sroa.14.0.copyload, %bb17.i.i ], [ %_0.i4196, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6104
  %filter_far.i16.i.i.sroa.0.0.lcssa = phi float [ %filter_far.i16.i.i.sroa.0.0.copyload, %bb17.i.i ], [ %_0.i4176, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6105
  %filter_far.i16.i.i.sroa.7.0.lcssa = phi float [ %filter_far.i16.i.i.sroa.7.0.copyload, %bb17.i.i ], [ %_0.i4172, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6105
  %filter_far.i16.i.i.sroa.11.0.lcssa = phi float [ %filter_far.i16.i.i.sroa.11.0.copyload, %bb17.i.i ], [ %_0.i4184, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6105
  %filter_far.i16.i.i.sroa.14.0.lcssa = phi float [ %filter_far.i16.i.i.sroa.14.0.copyload, %bb17.i.i ], [ %_0.i4180, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6105
  %gain_near.i15.i.i.sroa.0.0.lcssa = phi i32 [ %254, %bb17.i.i ], [ %_3.i4375, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6106
  %gain_near.i15.i.i.sroa.6.0.lcssa = phi i32 [ %255, %bb17.i.i ], [ %_3.i4371, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6106
  %gain_far.i14.i.i.sroa.0.0.lcssa = phi i32 [ %256, %bb17.i.i ], [ %_3.i4367, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6107
  %gain_far.i14.i.i.sroa.6.0.lcssa = phi i32 [ %257, %bb17.i.i ], [ %_3.i4363, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6107
  %position.sroa.0.0.i32.i.i.lcssa = phi i64 [ %258, %bb17.i.i ], [ %_41.sroa.0.0.i40.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit ], !dbg !6108
  store float %filter_near.i17.i.i.sroa.0.0.lcssa, ptr %164, align 8, !dbg !6109, !noalias !5129
  store float %filter_near.i17.i.i.sroa.7.0.lcssa, ptr %filter_near.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !6109, !noalias !5129
  store float %filter_near.i17.i.i.sroa.11.0.lcssa, ptr %filter_near.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !6109, !noalias !5129
  store float %filter_near.i17.i.i.sroa.14.0.lcssa, ptr %filter_near.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !6109, !noalias !5129
  store float %filter_far.i16.i.i.sroa.0.0.lcssa, ptr %165, align 8, !dbg !6110, !noalias !5129
  store float %filter_far.i16.i.i.sroa.7.0.lcssa, ptr %filter_far.i.i.i.sroa.7.0..sroa_idx, align 4, !dbg !6110, !noalias !5129
  store float %filter_far.i16.i.i.sroa.11.0.lcssa, ptr %filter_far.i.i.i.sroa.11.0..sroa_idx, align 8, !dbg !6110, !noalias !5129
  store float %filter_far.i16.i.i.sroa.14.0.lcssa, ptr %filter_far.i.i.i.sroa.14.0..sroa_idx, align 4, !dbg !6110, !noalias !5129
  store i32 %gain_near.i15.i.i.sroa.0.0.lcssa, ptr %166, align 8, !dbg !6111, !noalias !5129
  store i32 %gain_near.i15.i.i.sroa.6.0.lcssa, ptr %.sroa_idx7025, align 4, !dbg !6111, !noalias !5129
  store i32 %gain_far.i14.i.i.sroa.0.0.lcssa, ptr %167, align 8, !dbg !6112, !noalias !5129
  store i32 %gain_far.i14.i.i.sroa.6.0.lcssa, ptr %.sroa_idx7030, align 4, !dbg !6112, !noalias !5129
  store i64 %position.sroa.0.0.i32.i.i.lcssa, ptr %_51.i.i, align 8, !dbg !6113, !alias.scope !5078, !noalias !5079
  %advanced.i.i = trunc i64 %plan.0.i.i to i32, !dbg !6114
  store float %segments.i.i.sroa.0.0, ptr %124, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.i = load i32, ptr %180, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %307 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i, i32 %advanced.i.i), !dbg !6131
  store i32 %307, ptr %180, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.8.0, ptr %126, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.1.i = load i32, ptr %181, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %308 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i, i32 %advanced.i.i), !dbg !6131
  store i32 %308, ptr %181, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.12.0, ptr %128, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.2.i = load i32, ptr %182, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %309 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i, i32 %advanced.i.i), !dbg !6131
  store i32 %309, ptr %182, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.16.0, ptr %130, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.3.i = load i32, ptr %183, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %310 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i, i32 %advanced.i.i), !dbg !6131
  store i32 %310, ptr %183, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.20.0, ptr %132, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.4.i = load i32, ptr %184, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %311 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i, i32 %advanced.i.i), !dbg !6131
  store i32 %311, ptr %184, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.26.0, ptr %134, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.5.i = load i32, ptr %185, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %312 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i, i32 %advanced.i.i), !dbg !6131
  store i32 %312, ptr %185, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.32.0, ptr %136, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.6.i = load i32, ptr %186, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %313 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i, i32 %advanced.i.i), !dbg !6131
  store i32 %313, ptr %186, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.36.0, ptr %138, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.7.i = load i32, ptr %187, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %314 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i, i32 %advanced.i.i), !dbg !6131
  store i32 %314, ptr %187, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.40.0, ptr %140, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.8.i = load i32, ptr %188, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %315 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i, i32 %advanced.i.i), !dbg !6131
  store i32 %315, ptr %188, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.44.0, ptr %142, align 4, !dbg !6115, !alias.scope !6125, !noalias !6128
  %_26.9.i = load i32, ptr %189, align 4, !dbg !6130, !alias.scope !6125, !noalias !6128, !noundef !11
  %316 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i, i32 %advanced.i.i), !dbg !6131
  store i32 %316, ptr %189, align 4, !dbg !6134, !alias.scope !6125, !noalias !6128
  store float %segments.i.i.sroa.70.0, ptr %144, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.i6548 = load i32, ptr %190, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %317 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6548, i32 %advanced.i.i), !dbg !6143
  store i32 %317, ptr %190, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.76.0, ptr %146, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.1.i6551 = load i32, ptr %191, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %318 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6551, i32 %advanced.i.i), !dbg !6143
  store i32 %318, ptr %191, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.80.0, ptr %148, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.2.i6554 = load i32, ptr %192, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %319 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6554, i32 %advanced.i.i), !dbg !6143
  store i32 %319, ptr %192, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.84.0, ptr %150, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.3.i6557 = load i32, ptr %193, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %320 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6557, i32 %advanced.i.i), !dbg !6143
  store i32 %320, ptr %193, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.88.0, ptr %152, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.4.i6560 = load i32, ptr %194, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %321 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6560, i32 %advanced.i.i), !dbg !6143
  store i32 %321, ptr %194, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.94.0, ptr %154, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.5.i6563 = load i32, ptr %195, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %322 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6563, i32 %advanced.i.i), !dbg !6143
  store i32 %322, ptr %195, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.100.0, ptr %156, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.6.i6566 = load i32, ptr %196, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %323 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6566, i32 %advanced.i.i), !dbg !6143
  store i32 %323, ptr %196, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.104.0, ptr %158, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.7.i6569 = load i32, ptr %197, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %324 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6569, i32 %advanced.i.i), !dbg !6143
  store i32 %324, ptr %197, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.108.0, ptr %160, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.8.i6572 = load i32, ptr %198, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %325 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6572, i32 %advanced.i.i), !dbg !6143
  store i32 %325, ptr %198, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  store float %segments.i.i.sroa.112.0, ptr %162, align 4, !dbg !6135, !alias.scope !6137, !noalias !6140
  %_26.9.i6575 = load i32, ptr %199, align 4, !dbg !6142, !alias.scope !6137, !noalias !6140, !noundef !11
  %326 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6575, i32 %advanced.i.i), !dbg !6143
  store i32 %326, ptr %199, align 4, !dbg !6145, !alias.scope !6137, !noalias !6140
  br label %bb15.i.i, !dbg !5044

bb2.i19.i.lr.ph:                                  ; preds = %bb2.i, %bb3.i, %bb4.i
  tail call void @llvm.assume(i1 %_7.i), !dbg !3823
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6146), !dbg !6149
  %327 = getelementptr inbounds nuw i8, ptr %self, i64 856, !dbg !6150
  %sample_rate.i14.i = load i32, ptr %327, align 8, !dbg !6150, !alias.scope !6153, !noalias !6154, !noundef !11
  %328 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !6157
  %ring_len.i15.i = load i64, ptr %328, align 8, !dbg !6157, !alias.scope !6153, !noalias !6154, !noundef !11
  %329 = getelementptr inbounds nuw i8, ptr %self, i64 120
  %330 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %331 = getelementptr inbounds nuw i8, ptr %self, i64 200
  %332 = getelementptr inbounds nuw i8, ptr %self, i64 208
  %333 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %334 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %335 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %336 = getelementptr inbounds nuw i8, ptr %self, i64 240
  %337 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %338 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %339 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %340 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %341 = getelementptr inbounds nuw i8, ptr %self, i64 280
  %342 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %343 = getelementptr inbounds nuw i8, ptr %self, i64 296
  %344 = getelementptr inbounds nuw i8, ptr %self, i64 304
  %345 = getelementptr inbounds nuw i8, ptr %self, i64 312
  %346 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %347 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %348 = getelementptr inbounds nuw i8, ptr %self, i64 336
  %349 = getelementptr inbounds nuw i8, ptr %self, i64 344
  %_18.i23.i = getelementptr inbounds nuw i8, ptr %self, i64 480
  %350 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %351 = getelementptr inbounds nuw i8, ptr %self, i64 560
  %352 = getelementptr inbounds nuw i8, ptr %self, i64 568
  %353 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %354 = getelementptr inbounds nuw i8, ptr %self, i64 584
  %355 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %356 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %357 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %358 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %359 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %360 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %361 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %362 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %363 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %364 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %365 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %366 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %367 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %368 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %369 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %_51.i25.i = getelementptr inbounds nuw i8, ptr %self, i64 848
  %370 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %371 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %374 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %375 = getelementptr inbounds nuw i8, ptr %self, i64 488
  %376 = getelementptr inbounds nuw i8, ptr %self, i64 204
  %377 = getelementptr inbounds nuw i8, ptr %self, i64 220
  %378 = getelementptr inbounds nuw i8, ptr %self, i64 236
  %379 = getelementptr inbounds nuw i8, ptr %self, i64 252
  %380 = getelementptr inbounds nuw i8, ptr %self, i64 268
  %381 = getelementptr inbounds nuw i8, ptr %self, i64 284
  %382 = getelementptr inbounds nuw i8, ptr %self, i64 300
  %383 = getelementptr inbounds nuw i8, ptr %self, i64 316
  %384 = getelementptr inbounds nuw i8, ptr %self, i64 332
  %385 = getelementptr inbounds nuw i8, ptr %self, i64 348
  %386 = getelementptr inbounds nuw i8, ptr %self, i64 564
  %387 = getelementptr inbounds nuw i8, ptr %self, i64 580
  %388 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %389 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %390 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %391 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %392 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %393 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %394 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %395 = getelementptr inbounds nuw i8, ptr %self, i64 708
  br label %bb2.i19.i, !dbg !6159

bb2.i19.i:                                        ; preds = %bb2.i19.i.lr.ph, %bb15.i43.i
  %position.sroa.0.0.i17.i11256 = phi i64 [ 0, %bb2.i19.i.lr.ph ], [ %_32.i52.i, %bb15.i43.i ]
  %_11.i20.i = sub nuw i64 %_19.1, %position.sroa.0.0.i17.i11256, !dbg !6162
; call <multiband_compressor::Instance<f32, 1>>::plan_segment
  %396 = tail call fastcc { i64, i1 } @_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_(ptr noalias noundef nonnull align 8 dereferenceable(768) %_5, i64 noundef %_11.i20.i) #25, !dbg !6163, !noalias !3842
  %plan.0.i21.i = extractvalue { i64, i1 } %396, 0, !dbg !6163
  %plan.1.i22.i = extractvalue { i64, i1 } %396, 1, !dbg !6163
  %_12.le.i6576 = load float, ptr %330, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.i6577 = load float, ptr %331, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.1.i6578 = load float, ptr %332, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.1.i6579 = load float, ptr %333, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.2.i6580 = load float, ptr %334, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.2.i6581 = load float, ptr %335, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.3.i6582 = load float, ptr %336, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.3.i6583 = load float, ptr %337, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.4.i6584 = load float, ptr %338, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.4.i6585 = load float, ptr %339, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.5.i6586 = load float, ptr %340, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.5.i6587 = load float, ptr %341, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.6.i6588 = load float, ptr %342, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.6.i6589 = load float, ptr %343, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.7.i6590 = load float, ptr %344, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.7.i6591 = load float, ptr %345, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.8.i6592 = load float, ptr %346, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.8.i6593 = load float, ptr %347, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.9.i6594 = load float, ptr %348, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_16.le.9.i6595 = load float, ptr %349, align 8, !alias.scope !6164, !noalias !6167, !noundef !11
  %_12.le.i6614 = load float, ptr %350, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.i6615 = load float, ptr %351, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.1.i6616 = load float, ptr %352, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.1.i6617 = load float, ptr %353, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.2.i6618 = load float, ptr %354, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.2.i6619 = load float, ptr %355, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.3.i6620 = load float, ptr %356, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.3.i6621 = load float, ptr %357, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.4.i6622 = load float, ptr %358, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.4.i6623 = load float, ptr %359, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.5.i6624 = load float, ptr %360, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.5.i6625 = load float, ptr %361, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.6.i6626 = load float, ptr %362, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.6.i6627 = load float, ptr %363, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.7.i6628 = load float, ptr %364, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.7.i6629 = load float, ptr %365, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.8.i6630 = load float, ptr %366, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.8.i6631 = load float, ptr %367, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_12.le.9.i6632 = load float, ptr %368, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  %_16.le.9.i6633 = load float, ptr %369, align 8, !alias.scope !6169, !noalias !6172, !noundef !11
  call void @llvm.lifetime.start.p0(ptr nonnull %_20.i9.i), !dbg !6174, !noalias !6178
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_20.i9.i, ptr noalias noundef align 8 dereferenceable(360) %329, i32 noundef %sample_rate.i14.i) #25, !dbg !6179, !noalias !3842
  call void @llvm.lifetime.start.p0(ptr nonnull %_22.i8.i), !dbg !6180, !noalias !6178
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_22.i8.i, ptr noalias noundef align 8 dereferenceable(360) %_18.i23.i, i32 noundef %sample_rate.i14.i) #25, !dbg !6181, !noalias !3842
  call void @llvm.lifetime.end.p0(ptr nonnull %_22.i8.i), !dbg !6182, !noalias !6178
  call void @llvm.lifetime.end.p0(ptr nonnull %_20.i9.i), !dbg !6182, !noalias !6178
  %_32.i52.i = add i64 %plan.0.i21.i, %position.sroa.0.0.i17.i11256, !dbg !6183
  %_72.i53.i = icmp ult i64 %_32.i52.i, %position.sroa.0.0.i17.i11256, !dbg !6185
  %_66.not.i54.i = icmp ugt i64 %_32.i52.i, %_19.1
  %or.cond11.i55.i = or i1 %_72.i53.i, %_66.not.i54.i, !dbg !6185
  br i1 %plan.1.i22.i, label %bb9.i50.i, label %bb13.i24.i, !dbg !6191

bb9.i50.i:                                        ; preds = %bb2.i19.i
  br i1 %or.cond11.i55.i, label %bb19.i106.i, label %bb17.i56.i, !dbg !6192, !prof !3878

bb13.i24.i:                                       ; preds = %bb2.i19.i
  br i1 %or.cond11.i55.i, label %bb29.i49.i, label %bb27.i30.i, !dbg !6196, !prof !3878

bb27.i30.i:                                       ; preds = %bb13.i24.i
  %_95.i31.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i17.i11256, !dbg !6202
  %_105.i34.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i17.i11256, !dbg !6206
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6213), !dbg !6216
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %filter_near.i15.i.i, ptr noundef nonnull align 8 dereferenceable(16) %370, i64 16, i1 false), !dbg !6217
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %filter_far.i14.i.i, ptr noundef nonnull align 8 dereferenceable(16) %371, i64 16, i1 false), !dbg !6223
  %gain_near.sroa.0.0.copyload.i25.i.i = load i64, ptr %372, align 8, !dbg !6225, !noalias !6227
  %gain_far.sroa.0.0.copyload.i26.i.i = load i64, ptr %373, align 8, !dbg !6230, !noalias !6227
  %397 = load i64, ptr %_51.i25.i, align 8, !dbg !6232, !alias.scope !6234, !noalias !6235, !noundef !11
  %_168.i34.i42.i11246.not = icmp eq i64 %plan.0.i21.i, 0, !dbg !6237
  br i1 %_168.i34.i42.i11246.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_Kb0_EB2_.exit.i.i, label %bb56.i35.i.i, !dbg !6249

bb29.i49.i:                                       ; preds = %bb13.i24.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i17.i11256, i64 noundef %_32.i52.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ac5156198d2516c0e2a17140923b083) #26, !dbg !6250, !noalias !3842
  unreachable, !dbg !6250

bb56.i35.i.i:                                     ; preds = %bb27.i30.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4109
  %iter.sroa.0.0.i33.i41.i11248 = phi i64 [ %398, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4109 ], [ 0, %bb27.i30.i ]
  %position.sroa.0.0.i32.i40.i11247 = phi i64 [ %_41.sroa.0.0.i38.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4109 ], [ %397, %bb27.i30.i ]
  %398 = add nuw i64 %iter.sroa.0.0.i33.i41.i11248, 1, !dbg !6251
  %_42.i36.i.i = add i64 %position.sroa.0.0.i32.i40.i11247, 1, !dbg !6257
  %_176.not.i37.i.i = icmp ult i64 %_42.i36.i.i, %ring_len.i15.i, !dbg !6260
  %399 = select i1 %_176.not.i37.i.i, i64 0, i64 %ring_len.i15.i, !dbg !6260
  %_41.sroa.0.0.i38.i.i = sub nuw i64 %_42.i36.i.i, %399, !dbg !6260
  %_184.i42.i45.i = getelementptr inbounds nuw float, ptr %_95.i31.i, i64 %iter.sroa.0.0.i33.i41.i11248, !dbg !6263
  %_0.i3704 = load float, ptr %_184.i42.i45.i, align 4, !dbg !6273, !alias.scope !6275, !noalias !6278, !noundef !11
  %_192.i45.i.i = getelementptr inbounds nuw float, ptr %_105.i34.i, i64 %iter.sroa.0.0.i33.i41.i11248, !dbg !6279
  %_0.i3699 = load float, ptr %_192.i45.i.i, align 4, !dbg !6288, !alias.scope !6290, !noalias !6278, !noundef !11
  %_307.1.i47.i.i = load i64, ptr %374, align 8, !dbg !6293, !noalias !6278, !noundef !11
  %_193.i48.i.i = icmp ugt i64 %position.sroa.0.0.i32.i40.i11247, %_307.1.i47.i.i, !dbg !6295
  br i1 %_193.i48.i.i, label %bb67.i77.i.i, label %bb68.i49.i.i, !dbg !6295, !prof !1664

bb68.i49.i.i:                                     ; preds = %bb56.i35.i.i
  %_307.0.i50.i.i = load ptr, ptr %329, align 8, !dbg !6293, !noalias !6278, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6300), !dbg !6303
  %_4.not.i4118 = icmp eq i64 %_307.1.i47.i.i, %position.sroa.0.0.i32.i40.i11247, !dbg !6304
  br i1 %_4.not.i4118, label %panic.i4120, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4121, !dbg !6304

panic.i4120:                                      ; preds = %bb68.i49.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6304, !noalias !6306
  unreachable, !dbg !6304

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4121: ; preds = %bb68.i49.i.i
  %_200.i52.i.i = getelementptr inbounds nuw float, ptr %_307.0.i50.i.i, i64 %position.sroa.0.0.i32.i40.i11247, !dbg !6307
  store float %_0.i3704, ptr %_200.i52.i.i, align 4, !dbg !6304, !alias.scope !6300, !noalias !6278
  %_308.1.i53.i.i = load i64, ptr %375, align 8, !dbg !6312, !noalias !6278, !noundef !11
  %_201.i54.i.i = icmp ugt i64 %position.sroa.0.0.i32.i40.i11247, %_308.1.i53.i.i, !dbg !6313
  br i1 %_201.i54.i.i, label %bb69.i76.i.i, label %bb70.i55.i.i, !dbg !6313, !prof !1664

bb67.i77.i.i:                                     ; preds = %bb56.i35.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i40.i11247, i64 noundef %_307.1.i47.i.i, i64 noundef %_307.1.i47.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1f724420e117514d5eeca145f523ec40) #26, !dbg !6317, !noalias !6278
  unreachable, !dbg !6317

bb70.i55.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4121
  %_308.0.i56.i.i = load ptr, ptr %_18.i23.i, align 8, !dbg !6312, !noalias !6278, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6318), !dbg !6321
  %_4.not.i4114 = icmp eq i64 %_308.1.i53.i.i, %position.sroa.0.0.i32.i40.i11247, !dbg !6322
  br i1 %_4.not.i4114, label %panic.i4116, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4117, !dbg !6322

panic.i4116:                                      ; preds = %bb70.i55.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6322, !noalias !6324
  unreachable, !dbg !6322

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4117: ; preds = %bb70.i55.i.i
  %_208.i58.i.i = getelementptr inbounds nuw float, ptr %_308.0.i56.i.i, i64 %position.sroa.0.0.i32.i40.i11247, !dbg !6325
  store float %_0.i3699, ptr %_208.i58.i.i, align 4, !dbg !6322, !alias.scope !6318, !noalias !6278
  %_309.1.i59.i.i = load i64, ptr %374, align 8, !dbg !6330, !noalias !6278, !noundef !11
  %_209.i60.i.i = icmp ugt i64 %_41.sroa.0.0.i38.i.i, %_309.1.i59.i.i, !dbg !6331
  br i1 %_209.i60.i.i, label %bb71.i75.i.i, label %bb72.i61.i.i, !dbg !6331, !prof !1664

bb69.i76.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4121
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i40.i11247, i64 noundef %_308.1.i53.i.i, i64 noundef %_308.1.i53.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ebd89d0b93275e5686fc2b41f2e9561d) #26, !dbg !6335, !noalias !6278
  unreachable, !dbg !6335

bb72.i61.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4117
  %_309.0.i62.i.i = load ptr, ptr %329, align 8, !dbg !6330, !noalias !6278, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6336), !dbg !6339
  %_3.not.i3692 = icmp eq i64 %_309.1.i59.i.i, %_41.sroa.0.0.i38.i.i, !dbg !6340
  br i1 %_3.not.i3692, label %panic.i3695, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4113, !dbg !6340

panic.i3695:                                      ; preds = %bb72.i61.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6340, !noalias !6342
  unreachable, !dbg !6340

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4113: ; preds = %bb72.i61.i.i
  %_216.i64.i.i = getelementptr inbounds nuw float, ptr %_309.0.i62.i.i, i64 %_41.sroa.0.0.i38.i.i, !dbg !6343
  %_0.i3694 = load float, ptr %_216.i64.i.i, align 4, !dbg !6340, !alias.scope !6336, !noalias !6278, !noundef !11
  store float %_0.i3694, ptr %_184.i42.i45.i, align 4, !dbg !6348, !alias.scope !6350, !noalias !6278
  %_310.1.i66.i.i = load i64, ptr %375, align 8, !dbg !6353, !noalias !6278, !noundef !11
  %_221.i67.i.i = icmp ugt i64 %_41.sroa.0.0.i38.i.i, %_310.1.i66.i.i, !dbg !6354
  br i1 %_221.i67.i.i, label %bb73.i74.i.i, label %bb74.i68.i.i, !dbg !6354, !prof !1664

bb71.i75.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4117
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i38.i.i, i64 noundef %_309.1.i59.i.i, i64 noundef %_309.1.i59.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_89de14a4db9a7a5e199f0b3432909247) #26, !dbg !6358, !noalias !6278
  unreachable, !dbg !6358

bb74.i68.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4113
  %_310.0.i69.i.i = load ptr, ptr %_18.i23.i, align 8, !dbg !6353, !noalias !6278, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6359), !dbg !6362
  %_3.not.i3687 = icmp eq i64 %_310.1.i66.i.i, %_41.sroa.0.0.i38.i.i, !dbg !6363
  br i1 %_3.not.i3687, label %panic.i3690, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4109, !dbg !6363

panic.i3690:                                      ; preds = %bb74.i68.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6363, !noalias !6365
  unreachable, !dbg !6363

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4109: ; preds = %bb74.i68.i.i
  %_228.i71.i.i = getelementptr inbounds nuw float, ptr %_310.0.i69.i.i, i64 %_41.sroa.0.0.i38.i.i, !dbg !6366
  %_0.i3689 = load float, ptr %_228.i71.i.i, align 4, !dbg !6363, !alias.scope !6359, !noalias !6278, !noundef !11
  store float %_0.i3689, ptr %_192.i45.i.i, align 4, !dbg !6371, !alias.scope !6373, !noalias !6278
  %exitcond14313.not = icmp eq i64 %398, %plan.0.i21.i, !dbg !6237
  br i1 %exitcond14313.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_Kb0_EB2_.exit.i.i, label %bb56.i35.i.i, !dbg !6249

bb73.i74.i.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4113
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i38.i.i, i64 noundef %_310.1.i66.i.i, i64 noundef %_310.1.i66.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e4c416c2b2b3df4fd71341a77882ada1) #26, !dbg !6376, !noalias !6278
  unreachable, !dbg !6376

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_Kb0_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4109, %bb27.i30.i
  %position.sroa.0.0.i32.i40.i.lcssa = phi i64 [ %397, %bb27.i30.i ], [ %_41.sroa.0.0.i38.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4109 ], !dbg !6377
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %370, ptr noundef nonnull align 4 dereferenceable(16) %filter_near.i15.i.i, i64 16, i1 false), !dbg !6378
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %371, ptr noundef nonnull align 4 dereferenceable(16) %filter_far.i14.i.i, i64 16, i1 false), !dbg !6379
  store i64 %gain_near.sroa.0.0.copyload.i25.i.i, ptr %372, align 8, !dbg !6380, !noalias !6278
  store i64 %gain_far.sroa.0.0.copyload.i26.i.i, ptr %373, align 8, !dbg !6381, !noalias !6278
  store i64 %position.sroa.0.0.i32.i40.i.lcssa, ptr %_51.i25.i, align 8, !dbg !6382, !alias.scope !6234, !noalias !6235
  br label %bb15.i43.i, !dbg !6383

bb15.i43.i:                                       ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_KBZ_EB2_.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_Kb0_EB2_.exit.i.i
  %_8.i18.i = icmp ult i64 %_32.i52.i, %_19.1, !dbg !6159
  br i1 %_8.i18.i, label %bb2.i19.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit, !dbg !6159

bb17.i56.i:                                       ; preds = %bb9.i50.i
  %_75.i57.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i17.i11256, !dbg !6384
  %_85.i60.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i17.i11256, !dbg !6387
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6394), !dbg !6397
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %filter_near.i.i6.i, ptr noundef nonnull align 8 dereferenceable(16) %370, i64 16, i1 false), !dbg !6398
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(16) %filter_far.i.i5.i, ptr noundef nonnull align 8 dereferenceable(16) %371, i64 16, i1 false), !dbg !6404
  %gain_near.sroa.0.0.copyload.i.i.i = load i64, ptr %372, align 8, !dbg !6406, !noalias !6408
  %gain_far.sroa.0.0.copyload.i.i.i = load i64, ptr %373, align 8, !dbg !6412, !noalias !6408
  %400 = load i64, ptr %_51.i25.i, align 8, !dbg !6414, !alias.scope !6416, !noalias !6417, !noundef !11
  %_168.i.i76.i11251.not = icmp eq i64 %plan.0.i21.i, 0, !dbg !6419
  br i1 %_168.i.i76.i11251.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_KBZ_EB2_.exit.i.i, label %bb56.i.i78.i, !dbg !6431

bb19.i106.i:                                      ; preds = %bb9.i50.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i17.i11256, i64 noundef %_32.i52.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_683f9160cdc5ee4596b2ecf709188a9a) #26, !dbg !6432, !noalias !3842
  unreachable, !dbg !6432

bb56.i.i78.i:                                     ; preds = %bb17.i56.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093
  %segments.i13.i.sroa.98.1 = phi float [ %_0.i2809.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.9.i6632, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.94.1 = phi float [ %_0.i2809.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.8.i6630, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.90.1 = phi float [ %_0.i2809.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.7.i6628, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.86.1 = phi float [ %_0.i2809.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.6.i6626, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.82.1 = phi float [ %_0.i2809.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.5.i6624, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.78.1 = phi float [ %_0.i2809.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.4.i6622, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.74.1 = phi float [ %_0.i2809.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.3.i6620, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.70.1 = phi float [ %_0.i2809.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.2.i6618, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.66.1 = phi float [ %_0.i2809.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.1.i6616, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.62.1 = phi float [ %_0.i2809, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.i6614, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.38.1 = phi float [ %_0.i2810.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.9.i6594, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.34.1 = phi float [ %_0.i2810.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.8.i6592, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.30.1 = phi float [ %_0.i2810.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.7.i6590, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.26.1 = phi float [ %_0.i2810.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.6.i6588, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.22.1 = phi float [ %_0.i2810.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.5.i6586, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.18.1 = phi float [ %_0.i2810.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.4.i6584, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.14.1 = phi float [ %_0.i2810.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.3.i6582, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.10.1 = phi float [ %_0.i2810.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.2.i6580, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.6.1 = phi float [ %_0.i2810.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.1.i6578, %bb17.i56.i ], !dbg !6433
  %segments.i13.i.sroa.0.1 = phi float [ %_0.i2810, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %_12.le.i6576, %bb17.i56.i ], !dbg !6433
  %iter.sroa.0.0.i.i75.i11253 = phi i64 [ %401, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ 0, %bb17.i56.i ]
  %position.sroa.0.0.i.i74.i11252 = phi i64 [ %_41.sroa.0.0.i.i85.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], [ %400, %bb17.i56.i ]
  %_0.i2810 = fadd float %segments.i13.i.sroa.0.1, %_16.le.i6577, !dbg !6434
  %_0.i2809 = fadd float %segments.i13.i.sroa.62.1, %_16.le.i6615, !dbg !6439
  %_0.i2810.1 = fadd float %segments.i13.i.sroa.6.1, %_16.le.1.i6579, !dbg !6434
  %_0.i2809.1 = fadd float %segments.i13.i.sroa.66.1, %_16.le.1.i6617, !dbg !6439
  %_0.i2810.2 = fadd float %segments.i13.i.sroa.10.1, %_16.le.2.i6581, !dbg !6434
  %_0.i2809.2 = fadd float %segments.i13.i.sroa.70.1, %_16.le.2.i6619, !dbg !6439
  %_0.i2810.3 = fadd float %segments.i13.i.sroa.14.1, %_16.le.3.i6583, !dbg !6434
  %_0.i2809.3 = fadd float %segments.i13.i.sroa.74.1, %_16.le.3.i6621, !dbg !6439
  %_0.i2810.4 = fadd float %segments.i13.i.sroa.18.1, %_16.le.4.i6585, !dbg !6434
  %_0.i2809.4 = fadd float %segments.i13.i.sroa.78.1, %_16.le.4.i6623, !dbg !6439
  %_0.i2810.5 = fadd float %segments.i13.i.sroa.22.1, %_16.le.5.i6587, !dbg !6434
  %_0.i2809.5 = fadd float %segments.i13.i.sroa.82.1, %_16.le.5.i6625, !dbg !6439
  %_0.i2810.6 = fadd float %segments.i13.i.sroa.26.1, %_16.le.6.i6589, !dbg !6434
  %_0.i2809.6 = fadd float %segments.i13.i.sroa.86.1, %_16.le.6.i6627, !dbg !6439
  %_0.i2810.7 = fadd float %segments.i13.i.sroa.30.1, %_16.le.7.i6591, !dbg !6434
  %_0.i2809.7 = fadd float %segments.i13.i.sroa.90.1, %_16.le.7.i6629, !dbg !6439
  %_0.i2810.8 = fadd float %segments.i13.i.sroa.34.1, %_16.le.8.i6593, !dbg !6434
  %_0.i2809.8 = fadd float %segments.i13.i.sroa.94.1, %_16.le.8.i6631, !dbg !6439
  %_0.i2810.9 = fadd float %segments.i13.i.sroa.38.1, %_16.le.9.i6595, !dbg !6434
  %_0.i2809.9 = fadd float %segments.i13.i.sroa.98.1, %_16.le.9.i6633, !dbg !6439
  %401 = add nuw i64 %iter.sroa.0.0.i.i75.i11253, 1, !dbg !6441
  %_42.i.i83.i = add i64 %position.sroa.0.0.i.i74.i11252, 1, !dbg !6447
  %_176.not.i.i84.i = icmp ult i64 %_42.i.i83.i, %ring_len.i15.i, !dbg !6449
  %402 = select i1 %_176.not.i.i84.i, i64 0, i64 %ring_len.i15.i, !dbg !6449
  %_41.sroa.0.0.i.i85.i = sub nuw i64 %_42.i.i83.i, %402, !dbg !6449
  %_184.i.i89.i = getelementptr inbounds nuw float, ptr %_75.i57.i, i64 %iter.sroa.0.0.i.i75.i11253, !dbg !6452
  %_0.i3684 = load float, ptr %_184.i.i89.i, align 4, !dbg !6462, !alias.scope !6464, !noalias !6467, !noundef !11
  %_192.i.i92.i = getelementptr inbounds nuw float, ptr %_85.i60.i, i64 %iter.sroa.0.0.i.i75.i11253, !dbg !6468
  %_0.i3679 = load float, ptr %_192.i.i92.i, align 4, !dbg !6477, !alias.scope !6479, !noalias !6467, !noundef !11
  %_307.1.i.i.i = load i64, ptr %374, align 8, !dbg !6482, !noalias !6467, !noundef !11
  %_193.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i74.i11252, %_307.1.i.i.i, !dbg !6484
  br i1 %_193.i.i.i, label %bb67.i.i.i, label %bb68.i.i.i, !dbg !6484, !prof !1664

bb68.i.i.i:                                       ; preds = %bb56.i.i78.i
  %_307.0.i.i.i = load ptr, ptr %329, align 8, !dbg !6482, !noalias !6467, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6489), !dbg !6492
  %_4.not.i4102 = icmp eq i64 %_307.1.i.i.i, %position.sroa.0.0.i.i74.i11252, !dbg !6493
  br i1 %_4.not.i4102, label %panic.i4104, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4105, !dbg !6493

panic.i4104:                                      ; preds = %bb68.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6493, !noalias !6495
  unreachable, !dbg !6493

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4105: ; preds = %bb68.i.i.i
  %_200.i.i.i = getelementptr inbounds nuw float, ptr %_307.0.i.i.i, i64 %position.sroa.0.0.i.i74.i11252, !dbg !6496
  store float %_0.i3684, ptr %_200.i.i.i, align 4, !dbg !6493, !alias.scope !6489, !noalias !6467
  %_308.1.i.i.i = load i64, ptr %375, align 8, !dbg !6501, !noalias !6467, !noundef !11
  %_201.i.i.i = icmp ugt i64 %position.sroa.0.0.i.i74.i11252, %_308.1.i.i.i, !dbg !6502
  br i1 %_201.i.i.i, label %bb69.i.i.i, label %bb70.i.i.i, !dbg !6502, !prof !1664

bb67.i.i.i:                                       ; preds = %bb56.i.i78.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i74.i11252, i64 noundef %_307.1.i.i.i, i64 noundef %_307.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1f724420e117514d5eeca145f523ec40) #26, !dbg !6506, !noalias !6467
  unreachable, !dbg !6506

bb70.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4105
  %_308.0.i.i.i = load ptr, ptr %_18.i23.i, align 8, !dbg !6501, !noalias !6467, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6507), !dbg !6510
  %_4.not.i4098 = icmp eq i64 %_308.1.i.i.i, %position.sroa.0.0.i.i74.i11252, !dbg !6511
  br i1 %_4.not.i4098, label %panic.i4100, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4101, !dbg !6511

panic.i4100:                                      ; preds = %bb70.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6511, !noalias !6513
  unreachable, !dbg !6511

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4101: ; preds = %bb70.i.i.i
  %_208.i.i.i = getelementptr inbounds nuw float, ptr %_308.0.i.i.i, i64 %position.sroa.0.0.i.i74.i11252, !dbg !6514
  store float %_0.i3679, ptr %_208.i.i.i, align 4, !dbg !6511, !alias.scope !6507, !noalias !6467
  %_309.1.i.i.i = load i64, ptr %374, align 8, !dbg !6519, !noalias !6467, !noundef !11
  %_209.i.i.i = icmp ugt i64 %_41.sroa.0.0.i.i85.i, %_309.1.i.i.i, !dbg !6520
  br i1 %_209.i.i.i, label %bb71.i.i.i, label %bb72.i.i.i, !dbg !6520, !prof !1664

bb69.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4105
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i74.i11252, i64 noundef %_308.1.i.i.i, i64 noundef %_308.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ebd89d0b93275e5686fc2b41f2e9561d) #26, !dbg !6524, !noalias !6467
  unreachable, !dbg !6524

bb72.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4101
  %_309.0.i.i.i = load ptr, ptr %329, align 8, !dbg !6519, !noalias !6467, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6525), !dbg !6528
  %_3.not.i3672 = icmp eq i64 %_309.1.i.i.i, %_41.sroa.0.0.i.i85.i, !dbg !6529
  br i1 %_3.not.i3672, label %panic.i3675, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4097, !dbg !6529

panic.i3675:                                      ; preds = %bb72.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6529, !noalias !6531
  unreachable, !dbg !6529

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4097: ; preds = %bb72.i.i.i
  %_216.i.i.i = getelementptr inbounds nuw float, ptr %_309.0.i.i.i, i64 %_41.sroa.0.0.i.i85.i, !dbg !6532
  %_0.i3674 = load float, ptr %_216.i.i.i, align 4, !dbg !6529, !alias.scope !6525, !noalias !6467, !noundef !11
  store float %_0.i3674, ptr %_184.i.i89.i, align 4, !dbg !6537, !alias.scope !6539, !noalias !6467
  %_310.1.i.i.i = load i64, ptr %375, align 8, !dbg !6542, !noalias !6467, !noundef !11
  %_221.i.i.i = icmp ugt i64 %_41.sroa.0.0.i.i85.i, %_310.1.i.i.i, !dbg !6543
  br i1 %_221.i.i.i, label %bb73.i.i.i, label %bb74.i.i.i, !dbg !6543, !prof !1664

bb71.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4101
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i85.i, i64 noundef %_309.1.i.i.i, i64 noundef %_309.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_89de14a4db9a7a5e199f0b3432909247) #26, !dbg !6547, !noalias !6467
  unreachable, !dbg !6547

bb74.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4097
  %_310.0.i.i.i = load ptr, ptr %_18.i23.i, align 8, !dbg !6542, !noalias !6467, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6548), !dbg !6551
  %_3.not.i3667 = icmp eq i64 %_310.1.i.i.i, %_41.sroa.0.0.i.i85.i, !dbg !6552
  br i1 %_3.not.i3667, label %panic.i3670, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093, !dbg !6552

panic.i3670:                                      ; preds = %bb74.i.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !6552, !noalias !6554
  unreachable, !dbg !6552

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093: ; preds = %bb74.i.i.i
  %_228.i.i.i = getelementptr inbounds nuw float, ptr %_310.0.i.i.i, i64 %_41.sroa.0.0.i.i85.i, !dbg !6555
  %_0.i3669 = load float, ptr %_228.i.i.i, align 4, !dbg !6552, !alias.scope !6548, !noalias !6467, !noundef !11
  store float %_0.i3669, ptr %_192.i.i92.i, align 4, !dbg !6560, !alias.scope !6562, !noalias !6467
  %exitcond14315.not = icmp eq i64 %401, %plan.0.i21.i, !dbg !6419
  br i1 %exitcond14315.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_KBZ_EB2_.exit.i.i, label %bb56.i.i78.i, !dbg !6431

bb73.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4097
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i85.i, i64 noundef %_310.1.i.i.i, i64 noundef %_310.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e4c416c2b2b3df4fd71341a77882ada1) #26, !dbg !6565, !noalias !6467
  unreachable, !dbg !6565

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh0_Kb1_KBZ_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093, %bb17.i56.i
  %segments.i13.i.sroa.98.0 = phi float [ %_12.le.9.i6632, %bb17.i56.i ], [ %_0.i2809.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.94.0 = phi float [ %_12.le.8.i6630, %bb17.i56.i ], [ %_0.i2809.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.90.0 = phi float [ %_12.le.7.i6628, %bb17.i56.i ], [ %_0.i2809.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.86.0 = phi float [ %_12.le.6.i6626, %bb17.i56.i ], [ %_0.i2809.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.82.0 = phi float [ %_12.le.5.i6624, %bb17.i56.i ], [ %_0.i2809.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.78.0 = phi float [ %_12.le.4.i6622, %bb17.i56.i ], [ %_0.i2809.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.74.0 = phi float [ %_12.le.3.i6620, %bb17.i56.i ], [ %_0.i2809.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.70.0 = phi float [ %_12.le.2.i6618, %bb17.i56.i ], [ %_0.i2809.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.66.0 = phi float [ %_12.le.1.i6616, %bb17.i56.i ], [ %_0.i2809.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.62.0 = phi float [ %_12.le.i6614, %bb17.i56.i ], [ %_0.i2809, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.38.0 = phi float [ %_12.le.9.i6594, %bb17.i56.i ], [ %_0.i2810.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.34.0 = phi float [ %_12.le.8.i6592, %bb17.i56.i ], [ %_0.i2810.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.30.0 = phi float [ %_12.le.7.i6590, %bb17.i56.i ], [ %_0.i2810.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.26.0 = phi float [ %_12.le.6.i6588, %bb17.i56.i ], [ %_0.i2810.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.22.0 = phi float [ %_12.le.5.i6586, %bb17.i56.i ], [ %_0.i2810.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.18.0 = phi float [ %_12.le.4.i6584, %bb17.i56.i ], [ %_0.i2810.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.14.0 = phi float [ %_12.le.3.i6582, %bb17.i56.i ], [ %_0.i2810.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.10.0 = phi float [ %_12.le.2.i6580, %bb17.i56.i ], [ %_0.i2810.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.6.0 = phi float [ %_12.le.1.i6578, %bb17.i56.i ], [ %_0.i2810.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %segments.i13.i.sroa.0.0 = phi float [ %_12.le.i6576, %bb17.i56.i ], [ %_0.i2810, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6433
  %position.sroa.0.0.i.i74.i.lcssa = phi i64 [ %400, %bb17.i56.i ], [ %_41.sroa.0.0.i.i85.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4093 ], !dbg !6566
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %370, ptr noundef nonnull align 4 dereferenceable(16) %filter_near.i.i6.i, i64 16, i1 false), !dbg !6567
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %371, ptr noundef nonnull align 4 dereferenceable(16) %filter_far.i.i5.i, i64 16, i1 false), !dbg !6568
  store i64 %gain_near.sroa.0.0.copyload.i.i.i, ptr %372, align 8, !dbg !6569, !noalias !6467
  store i64 %gain_far.sroa.0.0.copyload.i.i.i, ptr %373, align 8, !dbg !6570, !noalias !6467
  store i64 %position.sroa.0.0.i.i74.i.lcssa, ptr %_51.i25.i, align 8, !dbg !6571, !alias.scope !6416, !noalias !6417
  %advanced.i77.i = trunc i64 %plan.0.i21.i to i32, !dbg !6572
  store float %segments.i13.i.sroa.0.0, ptr %330, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.i6661 = load i32, ptr %376, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %403 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6661, i32 %advanced.i77.i), !dbg !6582
  store i32 %403, ptr %376, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.6.0, ptr %332, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.1.i6664 = load i32, ptr %377, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %404 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6664, i32 %advanced.i77.i), !dbg !6582
  store i32 %404, ptr %377, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.10.0, ptr %334, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.2.i6667 = load i32, ptr %378, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %405 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6667, i32 %advanced.i77.i), !dbg !6582
  store i32 %405, ptr %378, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.14.0, ptr %336, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.3.i6670 = load i32, ptr %379, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %406 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6670, i32 %advanced.i77.i), !dbg !6582
  store i32 %406, ptr %379, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.18.0, ptr %338, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.4.i6673 = load i32, ptr %380, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %407 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6673, i32 %advanced.i77.i), !dbg !6582
  store i32 %407, ptr %380, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.22.0, ptr %340, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.5.i6676 = load i32, ptr %381, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %408 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6676, i32 %advanced.i77.i), !dbg !6582
  store i32 %408, ptr %381, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.26.0, ptr %342, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.6.i6679 = load i32, ptr %382, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %409 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6679, i32 %advanced.i77.i), !dbg !6582
  store i32 %409, ptr %382, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.30.0, ptr %344, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.7.i6682 = load i32, ptr %383, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %410 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6682, i32 %advanced.i77.i), !dbg !6582
  store i32 %410, ptr %383, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.34.0, ptr %346, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.8.i6685 = load i32, ptr %384, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %411 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6685, i32 %advanced.i77.i), !dbg !6582
  store i32 %411, ptr %384, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.38.0, ptr %348, align 4, !dbg !6573, !alias.scope !6576, !noalias !6579
  %_26.9.i6688 = load i32, ptr %385, align 4, !dbg !6581, !alias.scope !6576, !noalias !6579, !noundef !11
  %412 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6688, i32 %advanced.i77.i), !dbg !6582
  store i32 %412, ptr %385, align 4, !dbg !6584, !alias.scope !6576, !noalias !6579
  store float %segments.i13.i.sroa.62.0, ptr %350, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.i6690 = load i32, ptr %386, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %413 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6690, i32 %advanced.i77.i), !dbg !6593
  store i32 %413, ptr %386, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.66.0, ptr %352, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.1.i6693 = load i32, ptr %387, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %414 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6693, i32 %advanced.i77.i), !dbg !6593
  store i32 %414, ptr %387, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.70.0, ptr %354, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.2.i6696 = load i32, ptr %388, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %415 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6696, i32 %advanced.i77.i), !dbg !6593
  store i32 %415, ptr %388, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.74.0, ptr %356, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.3.i6699 = load i32, ptr %389, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %416 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6699, i32 %advanced.i77.i), !dbg !6593
  store i32 %416, ptr %389, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.78.0, ptr %358, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.4.i6702 = load i32, ptr %390, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %417 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6702, i32 %advanced.i77.i), !dbg !6593
  store i32 %417, ptr %390, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.82.0, ptr %360, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.5.i6705 = load i32, ptr %391, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %418 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6705, i32 %advanced.i77.i), !dbg !6593
  store i32 %418, ptr %391, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.86.0, ptr %362, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.6.i6708 = load i32, ptr %392, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %419 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6708, i32 %advanced.i77.i), !dbg !6593
  store i32 %419, ptr %392, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.90.0, ptr %364, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.7.i6711 = load i32, ptr %393, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %420 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6711, i32 %advanced.i77.i), !dbg !6593
  store i32 %420, ptr %393, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.94.0, ptr %366, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.8.i6714 = load i32, ptr %394, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %421 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6714, i32 %advanced.i77.i), !dbg !6593
  store i32 %421, ptr %394, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  store float %segments.i13.i.sroa.98.0, ptr %368, align 4, !dbg !6585, !alias.scope !6587, !noalias !6590
  %_26.9.i6717 = load i32, ptr %395, align 4, !dbg !6592, !alias.scope !6587, !noalias !6590, !noundef !11
  %422 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6717, i32 %advanced.i77.i), !dbg !6593
  store i32 %422, ptr %395, align 4, !dbg !6595, !alias.scope !6587, !noalias !6590
  br label %bb15.i43.i, !dbg !6383

bb2.i132.i.lr.ph:                                 ; preds = %bb3.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6596), !dbg !6599
  %423 = getelementptr inbounds nuw i8, ptr %self, i64 856, !dbg !6600
  %sample_rate.i127.i = load i32, ptr %423, align 8, !dbg !6600, !alias.scope !6603, !noalias !6604, !noundef !11
  %424 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !6607
  %ring_len.i128.i = load i64, ptr %424, align 8, !dbg !6607, !alias.scope !6603, !noalias !6604, !noundef !11
  %425 = getelementptr inbounds nuw i8, ptr %self, i64 120
  %426 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %427 = getelementptr inbounds nuw i8, ptr %self, i64 200
  %428 = getelementptr inbounds nuw i8, ptr %self, i64 208
  %429 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %430 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %431 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %432 = getelementptr inbounds nuw i8, ptr %self, i64 240
  %433 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %434 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %435 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %436 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %437 = getelementptr inbounds nuw i8, ptr %self, i64 280
  %438 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %439 = getelementptr inbounds nuw i8, ptr %self, i64 296
  %440 = getelementptr inbounds nuw i8, ptr %self, i64 304
  %441 = getelementptr inbounds nuw i8, ptr %self, i64 312
  %442 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %443 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %444 = getelementptr inbounds nuw i8, ptr %self, i64 336
  %445 = getelementptr inbounds nuw i8, ptr %self, i64 344
  %_18.i136.i = getelementptr inbounds nuw i8, ptr %self, i64 480
  %446 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %447 = getelementptr inbounds nuw i8, ptr %self, i64 560
  %448 = getelementptr inbounds nuw i8, ptr %self, i64 568
  %449 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %450 = getelementptr inbounds nuw i8, ptr %self, i64 584
  %451 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %452 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %453 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %454 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %455 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %456 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %457 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %458 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %459 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %460 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %461 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %462 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %463 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %464 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %465 = getelementptr inbounds nuw i8, ptr %self, i64 704
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
  %466 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %filter_near.i.i119.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 172
  %filter_near.i.i119.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 176
  %filter_near.i.i119.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 180
  %467 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %filter_far.i.i118.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 532
  %filter_far.i.i118.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 536
  %filter_far.i.i118.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 540
  %468 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %.sroa_idx7061 = getelementptr inbounds nuw i8, ptr %self, i64 188
  %469 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %.sroa_idx7066 = getelementptr inbounds nuw i8, ptr %self, i64 548
  %_24.i.i156.i = getelementptr inbounds nuw i8, ptr %self, i64 400
  %_26.i.i158.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %_67.i.i177.i = getelementptr inbounds nuw i8, ptr %self, i64 152
  %470 = getelementptr inbounds nuw i8, ptr %self, i64 156
  %471 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %472 = getelementptr inbounds nuw i8, ptr %self, i64 164
  %_72.i.i179.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %473 = getelementptr inbounds nuw i8, ptr %self, i64 516
  %474 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %475 = getelementptr inbounds nuw i8, ptr %self, i64 524
  %476 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %477 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %478 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %479 = getelementptr inbounds nuw i8, ptr %self, i64 488
  %480 = getelementptr inbounds nuw i8, ptr %self, i64 496
  %481 = getelementptr inbounds nuw i8, ptr %self, i64 504
  %482 = getelementptr inbounds nuw i8, ptr %self, i64 204
  %483 = getelementptr inbounds nuw i8, ptr %self, i64 220
  %484 = getelementptr inbounds nuw i8, ptr %self, i64 236
  %485 = getelementptr inbounds nuw i8, ptr %self, i64 252
  %486 = getelementptr inbounds nuw i8, ptr %self, i64 268
  %487 = getelementptr inbounds nuw i8, ptr %self, i64 284
  %488 = getelementptr inbounds nuw i8, ptr %self, i64 300
  %489 = getelementptr inbounds nuw i8, ptr %self, i64 316
  %490 = getelementptr inbounds nuw i8, ptr %self, i64 332
  %491 = getelementptr inbounds nuw i8, ptr %self, i64 348
  %492 = getelementptr inbounds nuw i8, ptr %self, i64 564
  %493 = getelementptr inbounds nuw i8, ptr %self, i64 580
  %494 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %495 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %496 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %497 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %498 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %499 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %500 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %501 = getelementptr inbounds nuw i8, ptr %self, i64 708
  br label %bb2.i132.i, !dbg !6609

bb2.i132.i:                                       ; preds = %bb2.i132.i.lr.ph, %bb15.i166.i
  %position.sroa.0.0.i130.i11186 = phi i64 [ 0, %bb2.i132.i.lr.ph ], [ %_32.i320.i, %bb15.i166.i ]
  %_11.i133.i = sub nuw i64 %_19.1, %position.sroa.0.0.i130.i11186, !dbg !6612
; call <multiband_compressor::Instance<f32, 1>>::plan_segment
  %502 = tail call fastcc { i64, i1 } @_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_(ptr noalias noundef nonnull align 8 dereferenceable(768) %_5, i64 noundef %_11.i133.i) #25, !dbg !6613, !noalias !3842
  %plan.0.i134.i = extractvalue { i64, i1 } %502, 0, !dbg !6613
  %plan.1.i135.i = extractvalue { i64, i1 } %502, 1, !dbg !6613
  %_12.le.i6718 = load float, ptr %426, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.i6719 = load float, ptr %427, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.1.i6720 = load float, ptr %428, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.1.i6721 = load float, ptr %429, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.2.i6722 = load float, ptr %430, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.2.i6723 = load float, ptr %431, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.3.i6724 = load float, ptr %432, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.3.i6725 = load float, ptr %433, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.4.i6726 = load float, ptr %434, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.4.i6727 = load float, ptr %435, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.5.i6728 = load float, ptr %436, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.5.i6729 = load float, ptr %437, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.6.i6730 = load float, ptr %438, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.6.i6731 = load float, ptr %439, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.7.i6732 = load float, ptr %440, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.7.i6733 = load float, ptr %441, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.8.i6734 = load float, ptr %442, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.8.i6735 = load float, ptr %443, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.9.i6736 = load float, ptr %444, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_16.le.9.i6737 = load float, ptr %445, align 8, !alias.scope !6614, !noalias !6617, !noundef !11
  %_12.le.i6756 = load float, ptr %446, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.i6757 = load float, ptr %447, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.1.i6758 = load float, ptr %448, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.1.i6759 = load float, ptr %449, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.2.i6760 = load float, ptr %450, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.2.i6761 = load float, ptr %451, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.3.i6762 = load float, ptr %452, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.3.i6763 = load float, ptr %453, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.4.i6764 = load float, ptr %454, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.4.i6765 = load float, ptr %455, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.5.i6766 = load float, ptr %456, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.5.i6767 = load float, ptr %457, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.6.i6768 = load float, ptr %458, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.6.i6769 = load float, ptr %459, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.7.i6770 = load float, ptr %460, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.7.i6771 = load float, ptr %461, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.8.i6772 = load float, ptr %462, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.8.i6773 = load float, ptr %463, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_12.le.9.i6774 = load float, ptr %464, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  %_16.le.9.i6775 = load float, ptr %465, align 8, !alias.scope !6619, !noalias !6622, !noundef !11
  call void @llvm.lifetime.start.p0(ptr nonnull %_20.i122.i), !dbg !6624, !noalias !6628
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_20.i122.i, ptr noalias noundef align 8 dereferenceable(360) %425, i32 noundef %sample_rate.i127.i) #25, !dbg !6629, !noalias !3842
  call void @llvm.lifetime.start.p0(ptr nonnull %_22.i121.i), !dbg !6630, !noalias !6628
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_22.i121.i, ptr noalias noundef align 8 dereferenceable(360) %_18.i136.i, i32 noundef %sample_rate.i127.i) #25, !dbg !6631, !noalias !3842
  %coefficients.i123.i.sroa.0.0.copyload = load float, ptr %_20.i122.i, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.5.0.copyload = load float, ptr %coefficients.i123.i.sroa.5.0._20.i122.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.7.0.copyload = load float, ptr %coefficients.i123.i.sroa.7.0._20.i122.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.9.0.copyload = load float, ptr %coefficients.i123.i.sroa.9.0._20.i122.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.11.0.copyload = load float, ptr %coefficients.i123.i.sroa.11.0._20.i122.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.13.0.copyload = load float, ptr %coefficients.i123.i.sroa.13.0._20.i122.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.15.24.copyload = load float, ptr %_22.i121.i, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.18.24.copyload = load float, ptr %coefficients.i123.i.sroa.18.24._22.i121.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.20.24.copyload = load float, ptr %coefficients.i123.i.sroa.20.24._22.i121.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.22.24.copyload = load float, ptr %coefficients.i123.i.sroa.22.24._22.i121.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.24.24.copyload = load float, ptr %coefficients.i123.i.sroa.24.24._22.i121.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  %coefficients.i123.i.sroa.26.24.copyload = load float, ptr %coefficients.i123.i.sroa.26.24._22.i121.i.sroa_idx, align 4, !dbg !6632, !noalias !6628
  call void @llvm.lifetime.end.p0(ptr nonnull %_22.i121.i), !dbg !6633, !noalias !6628
  call void @llvm.lifetime.end.p0(ptr nonnull %_20.i122.i), !dbg !6633, !noalias !6628
  %_32.i320.i = add i64 %plan.0.i134.i, %position.sroa.0.0.i130.i11186, !dbg !6634
  %_72.i321.i = icmp ult i64 %_32.i320.i, %position.sroa.0.0.i130.i11186, !dbg !6636
  %_66.not.i322.i = icmp ugt i64 %_32.i320.i, %_19.1
  %or.cond11.i323.i = or i1 %_72.i321.i, %_66.not.i322.i, !dbg !6636
  br i1 %plan.1.i135.i, label %bb9.i318.i, label %bb13.i137.i, !dbg !6642

bb9.i318.i:                                       ; preds = %bb2.i132.i
  br i1 %or.cond11.i323.i, label %bb19.i512.i, label %bb17.i324.i, !dbg !6643, !prof !3878

bb13.i137.i:                                      ; preds = %bb2.i132.i
  br i1 %or.cond11.i323.i, label %bb29.i317.i, label %bb27.i143.i, !dbg !6647, !prof !3878

bb27.i143.i:                                      ; preds = %bb13.i137.i
  %_95.i144.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i130.i11186, !dbg !6653
  %_105.i147.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i130.i11186, !dbg !6657
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6664), !dbg !6667
  %filter_near.i.i119.i.sroa.0.0.copyload = load float, ptr %466, align 8, !dbg !6668, !noalias !6674
  %filter_near.i.i119.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.7.0..sroa_idx, align 4, !dbg !6668, !noalias !6674
  %filter_near.i.i119.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.11.0..sroa_idx, align 8, !dbg !6668, !noalias !6674
  %filter_near.i.i119.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.14.0..sroa_idx, align 4, !dbg !6668, !noalias !6674
  %filter_far.i.i118.i.sroa.0.0.copyload = load float, ptr %467, align 8, !dbg !6679, !noalias !6674
  %filter_far.i.i118.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.7.0..sroa_idx, align 4, !dbg !6679, !noalias !6674
  %filter_far.i.i118.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.11.0..sroa_idx, align 8, !dbg !6679, !noalias !6674
  %filter_far.i.i118.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.14.0..sroa_idx, align 4, !dbg !6679, !noalias !6674
  %503 = load i32, ptr %468, align 8, !dbg !6681
  %504 = load i32, ptr %.sroa_idx7061, align 4, !dbg !6681
  %505 = load i32, ptr %469, align 8, !dbg !6683
  %506 = load i32, ptr %.sroa_idx7066, align 4, !dbg !6683
  %507 = load i64, ptr %_51.i138.i, align 8, !dbg !6685, !alias.scope !6687, !noalias !6688, !noundef !11
  %_168.i.i163.i11128.not = icmp eq i64 %plan.0.i134.i, 0, !dbg !6690
  br i1 %_168.i.i163.i11128.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_KBZ_EB2_.exit.i.i, label %bb56.i.i167.i, !dbg !6702

bb29.i317.i:                                      ; preds = %bb13.i137.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i130.i11186, i64 noundef %_32.i320.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ac5156198d2516c0e2a17140923b083) #26, !dbg !6703, !noalias !3842
  unreachable, !dbg !6703

bb56.i.i167.i:                                    ; preds = %bb27.i143.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069
  %iter.sroa.0.0.i.i162.i11142 = phi i64 [ %508, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ 0, %bb27.i143.i ]
  %position.sroa.0.0.i.i161.i11141 = phi i64 [ %_41.sroa.0.0.i.i170.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %507, %bb27.i143.i ]
  %gain_far.i.i116.i.sroa.6.011140 = phi i32 [ %_3.i4411, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %506, %bb27.i143.i ]
  %gain_far.i.i116.i.sroa.0.011139 = phi i32 [ %_3.i4415, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %505, %bb27.i143.i ]
  %gain_near.i.i117.i.sroa.6.011138 = phi i32 [ %_3.i4419, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %504, %bb27.i143.i ]
  %gain_near.i.i117.i.sroa.0.011137 = phi i32 [ %_3.i4423, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %503, %bb27.i143.i ]
  %filter_far.i.i118.i.sroa.14.011136 = phi float [ %_0.i4276, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %filter_far.i.i118.i.sroa.14.0.copyload, %bb27.i143.i ]
  %filter_far.i.i118.i.sroa.11.011135 = phi float [ %_0.i4280, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %filter_far.i.i118.i.sroa.11.0.copyload, %bb27.i143.i ]
  %filter_far.i.i118.i.sroa.7.011134 = phi float [ %_0.i4268, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %filter_far.i.i118.i.sroa.7.0.copyload, %bb27.i143.i ]
  %filter_far.i.i118.i.sroa.0.011133 = phi float [ %_0.i4272, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %filter_far.i.i118.i.sroa.0.0.copyload, %bb27.i143.i ]
  %filter_near.i.i119.i.sroa.14.011132 = phi float [ %_0.i4292, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %filter_near.i.i119.i.sroa.14.0.copyload, %bb27.i143.i ]
  %filter_near.i.i119.i.sroa.11.011131 = phi float [ %_0.i4296, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %filter_near.i.i119.i.sroa.11.0.copyload, %bb27.i143.i ]
  %filter_near.i.i119.i.sroa.7.011130 = phi float [ %_0.i4284, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %filter_near.i.i119.i.sroa.7.0.copyload, %bb27.i143.i ]
  %filter_near.i.i119.i.sroa.0.011129 = phi float [ %_0.i4288, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], [ %filter_near.i.i119.i.sroa.0.0.copyload, %bb27.i143.i ]
  %508 = add nuw i64 %iter.sroa.0.0.i.i162.i11142, 1, !dbg !6704
  %_42.i.i168.i = add i64 %position.sroa.0.0.i.i161.i11141, 1, !dbg !6710
  %_176.not.i.i169.i = icmp ult i64 %_42.i.i168.i, %ring_len.i128.i, !dbg !6713
  %509 = select i1 %_176.not.i.i169.i, i64 0, i64 %ring_len.i128.i, !dbg !6713
  %_41.sroa.0.0.i.i170.i = sub nuw i64 %_42.i.i168.i, %509, !dbg !6713
  %_184.i.i172.i = getelementptr inbounds nuw float, ptr %_95.i144.i, i64 %iter.sroa.0.0.i.i162.i11142, !dbg !6716
  %_0.i3664 = load float, ptr %_184.i.i172.i, align 4, !dbg !6726, !alias.scope !6728, !noalias !6731, !noundef !11
  %_192.i.i175.i = getelementptr inbounds nuw float, ptr %_105.i147.i, i64 %iter.sroa.0.0.i.i162.i11142, !dbg !6732
  %_0.i3659 = load float, ptr %_192.i.i175.i, align 4, !dbg !6741, !alias.scope !6743, !noalias !6731, !noundef !11
  %_7.i80 = load float, ptr %_67.i.i177.i, align 4, !dbg !6746, !alias.scope !6749, !noalias !6752, !noundef !11
  %_8.i81 = load float, ptr %470, align 4, !dbg !6754, !alias.scope !6749, !noalias !6752, !noundef !11
  %_9.i82 = load float, ptr %471, align 4, !dbg !6755, !alias.scope !6749, !noalias !6752, !noundef !11
  %_0.i3423 = fsub float %_0.i3664, %filter_near.i.i119.i.sroa.7.011130, !dbg !6756
  %_0.i3246 = fmul float %_0.i3423, %_8.i81, !dbg !6759
  %_4.i2883 = fmul float %filter_near.i.i119.i.sroa.0.011129, %_7.i80, !dbg !6761
  %_0.i2884 = fadd float %_4.i2883, %_0.i3246, !dbg !6761
  %_0.i2726 = fadd float %filter_near.i.i119.i.sroa.0.011129, %_0.i2884, !dbg !6763
  %_0.i3245 = fmul float %filter_near.i.i119.i.sroa.0.011129, %_8.i81, !dbg !6765
  %_4.i2881 = fmul float %_0.i3423, %_9.i82, !dbg !6767
  %_0.i2882 = fadd float %_0.i3245, %_4.i2881, !dbg !6767
  %_0.i2725 = fadd float %filter_near.i.i119.i.sroa.7.011130, %_0.i2882, !dbg !6769
  %_0.i2724 = fadd float %_0.i2884, %_0.i2884, !dbg !6771
  %_0.i2723 = fadd float %filter_near.i.i119.i.sroa.0.011129, %_0.i2724, !dbg !6773
  %510 = tail call noundef float @llvm.fabs.f32(float %_0.i2723), !dbg !6775
  %511 = fcmp uge float %510, 0x3BC79CA100000000, !dbg !6778
  %_0.i4288 = select i1 %511, float %_0.i2723, float 0.000000e+00, !dbg !6780
  %_0.i2722 = fadd float %_0.i2882, %_0.i2882, !dbg !6781
  %_0.i2721 = fadd float %filter_near.i.i119.i.sroa.7.011130, %_0.i2722, !dbg !6783
  %512 = tail call noundef float @llvm.fabs.f32(float %_0.i2721), !dbg !6785
  %513 = fcmp uge float %512, 0x3BC79CA100000000, !dbg !6788
  %_0.i4284 = select i1 %513, float %_0.i2721, float 0.000000e+00, !dbg !6790
  %_12.i85 = load float, ptr %472, align 4, !dbg !6791, !alias.scope !6749, !noalias !6752, !noundef !11
  %_4.i2929 = fmul float %_12.i85, %_0.i2726, !dbg !6792
  %_0.i2930 = fadd float %_0.i3664, %_4.i2929, !dbg !6792
  %_0.i3424 = fsub float %_0.i2725, %filter_near.i.i119.i.sroa.14.011132, !dbg !6794
  %_0.i3248 = fmul float %_8.i81, %_0.i3424, !dbg !6797
  %_4.i2887 = fmul float %filter_near.i.i119.i.sroa.11.011131, %_7.i80, !dbg !6799
  %_0.i2888 = fadd float %_4.i2887, %_0.i3248, !dbg !6799
  %_0.i3247 = fmul float %filter_near.i.i119.i.sroa.11.011131, %_8.i81, !dbg !6801
  %_4.i2885 = fmul float %_9.i82, %_0.i3424, !dbg !6803
  %_0.i2886 = fadd float %_0.i3247, %_4.i2885, !dbg !6803
  %_0.i2731 = fadd float %filter_near.i.i119.i.sroa.14.011132, %_0.i2886, !dbg !6805
  %_0.i2730 = fadd float %_0.i2888, %_0.i2888, !dbg !6807
  %_0.i2729 = fadd float %filter_near.i.i119.i.sroa.11.011131, %_0.i2730, !dbg !6809
  %514 = tail call noundef float @llvm.fabs.f32(float %_0.i2729), !dbg !6811
  %515 = fcmp uge float %514, 0x3BC79CA100000000, !dbg !6814
  %_0.i4296 = select i1 %515, float %_0.i2729, float 0.000000e+00, !dbg !6816
  %_0.i2728 = fadd float %_0.i2886, %_0.i2886, !dbg !6817
  %_0.i2727 = fadd float %filter_near.i.i119.i.sroa.14.011132, %_0.i2728, !dbg !6819
  %516 = tail call noundef float @llvm.fabs.f32(float %_0.i2727), !dbg !6821
  %517 = fcmp uge float %516, 0x3BC79CA100000000, !dbg !6824
  %_0.i4292 = select i1 %517, float %_0.i2727, float 0.000000e+00, !dbg !6826
  %_0.i3437 = fsub float %_0.i2930, %_0.i2731, !dbg !6827
  %_7.i67 = load float, ptr %_72.i.i179.i, align 4, !dbg !6829, !alias.scope !6832, !noalias !6835, !noundef !11
  %_8.i68 = load float, ptr %473, align 4, !dbg !6837, !alias.scope !6832, !noalias !6835, !noundef !11
  %_9.i69 = load float, ptr %474, align 4, !dbg !6838, !alias.scope !6832, !noalias !6835, !noundef !11
  %_0.i3421 = fsub float %_0.i3659, %filter_far.i.i118.i.sroa.7.011134, !dbg !6839
  %_0.i3242 = fmul float %_0.i3421, %_8.i68, !dbg !6842
  %_4.i2875 = fmul float %filter_far.i.i118.i.sroa.0.011133, %_7.i67, !dbg !6844
  %_0.i2876 = fadd float %_4.i2875, %_0.i3242, !dbg !6844
  %_0.i2714 = fadd float %filter_far.i.i118.i.sroa.0.011133, %_0.i2876, !dbg !6846
  %_0.i3241 = fmul float %filter_far.i.i118.i.sroa.0.011133, %_8.i68, !dbg !6848
  %_4.i2873 = fmul float %_0.i3421, %_9.i69, !dbg !6850
  %_0.i2874 = fadd float %_0.i3241, %_4.i2873, !dbg !6850
  %_0.i2713 = fadd float %filter_far.i.i118.i.sroa.7.011134, %_0.i2874, !dbg !6852
  %_0.i2712 = fadd float %_0.i2876, %_0.i2876, !dbg !6854
  %_0.i2711 = fadd float %filter_far.i.i118.i.sroa.0.011133, %_0.i2712, !dbg !6856
  %518 = tail call noundef float @llvm.fabs.f32(float %_0.i2711), !dbg !6858
  %519 = fcmp uge float %518, 0x3BC79CA100000000, !dbg !6861
  %_0.i4272 = select i1 %519, float %_0.i2711, float 0.000000e+00, !dbg !6863
  %_0.i2710 = fadd float %_0.i2874, %_0.i2874, !dbg !6864
  %_0.i2709 = fadd float %filter_far.i.i118.i.sroa.7.011134, %_0.i2710, !dbg !6866
  %520 = tail call noundef float @llvm.fabs.f32(float %_0.i2709), !dbg !6868
  %521 = fcmp uge float %520, 0x3BC79CA100000000, !dbg !6871
  %_0.i4268 = select i1 %521, float %_0.i2709, float 0.000000e+00, !dbg !6873
  %_12.i72 = load float, ptr %475, align 4, !dbg !6874, !alias.scope !6832, !noalias !6835, !noundef !11
  %_4.i2931 = fmul float %_12.i72, %_0.i2714, !dbg !6875
  %_0.i2932 = fadd float %_0.i3659, %_4.i2931, !dbg !6875
  %_0.i3422 = fsub float %_0.i2713, %filter_far.i.i118.i.sroa.14.011136, !dbg !6877
  %_0.i3244 = fmul float %_8.i68, %_0.i3422, !dbg !6880
  %_4.i2879 = fmul float %filter_far.i.i118.i.sroa.11.011135, %_7.i67, !dbg !6882
  %_0.i2880 = fadd float %_4.i2879, %_0.i3244, !dbg !6882
  %_0.i3243 = fmul float %filter_far.i.i118.i.sroa.11.011135, %_8.i68, !dbg !6884
  %_4.i2877 = fmul float %_9.i69, %_0.i3422, !dbg !6886
  %_0.i2878 = fadd float %_0.i3243, %_4.i2877, !dbg !6886
  %_0.i2719 = fadd float %filter_far.i.i118.i.sroa.14.011136, %_0.i2878, !dbg !6888
  %_0.i2718 = fadd float %_0.i2880, %_0.i2880, !dbg !6890
  %_0.i2717 = fadd float %filter_far.i.i118.i.sroa.11.011135, %_0.i2718, !dbg !6892
  %522 = tail call noundef float @llvm.fabs.f32(float %_0.i2717), !dbg !6894
  %523 = fcmp uge float %522, 0x3BC79CA100000000, !dbg !6897
  %_0.i4280 = select i1 %523, float %_0.i2717, float 0.000000e+00, !dbg !6899
  %_0.i2716 = fadd float %_0.i2878, %_0.i2878, !dbg !6900
  %_0.i2715 = fadd float %filter_far.i.i118.i.sroa.14.011136, %_0.i2716, !dbg !6902
  %524 = tail call noundef float @llvm.fabs.f32(float %_0.i2715), !dbg !6904
  %525 = fcmp uge float %524, 0x3BC79CA100000000, !dbg !6907
  %_0.i4276 = select i1 %525, float %_0.i2715, float 0.000000e+00, !dbg !6909
  %_0.i3438 = fsub float %_0.i2932, %_0.i2719, !dbg !6910
  %_311.1.i.i182.i = load i64, ptr %476, align 8, !dbg !6912, !noalias !6731, !noundef !11
  %_234.i.i183.i = icmp ugt i64 %position.sroa.0.0.i.i161.i11141, %_311.1.i.i182.i, !dbg !6914
  br i1 %_234.i.i183.i, label %bb78.i.i313.i, label %bb79.i.i184.i, !dbg !6914, !prof !1664

bb79.i.i184.i:                                    ; preds = %bb56.i.i167.i
  %_311.0.i.i185.i = load ptr, ptr %425, align 8, !dbg !6912, !noalias !6731, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6920), !dbg !6923
  %_4.not.i4086 = icmp eq i64 %_311.1.i.i182.i, %position.sroa.0.0.i.i161.i11141, !dbg !6924
  br i1 %_4.not.i4086, label %panic.i4088, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4089, !dbg !6924

panic.i4088:                                      ; preds = %bb79.i.i184.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6924, !noalias !6926
  unreachable, !dbg !6924

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4089: ; preds = %bb79.i.i184.i
  %_241.i.i188.i = getelementptr inbounds nuw float, ptr %_311.0.i.i185.i, i64 %position.sroa.0.0.i.i161.i11141, !dbg !6927
  store float %_0.i2731, ptr %_241.i.i188.i, align 4, !dbg !6924, !alias.scope !6920, !noalias !6731
  %_312.1.i.i189.i = load i64, ptr %478, align 8, !dbg !6933, !noalias !6731, !noundef !11
  %_242.i.i190.i = icmp ugt i64 %position.sroa.0.0.i.i161.i11141, %_312.1.i.i189.i, !dbg !6934
  br i1 %_242.i.i190.i, label %bb80.i.i312.i, label %bb81.i.i191.i, !dbg !6934, !prof !1664

bb78.i.i313.i:                                    ; preds = %bb56.i.i167.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i161.i11141, i64 noundef %_311.1.i.i182.i, i64 noundef %_311.1.i.i182.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7515f6adc8a5521b72f04a3abb372e72) #26, !dbg !6938, !noalias !6731
  unreachable, !dbg !6938

bb81.i.i191.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4089
  %_312.0.i.i192.i = load ptr, ptr %477, align 8, !dbg !6933, !noalias !6731, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6939), !dbg !6942
  %_4.not.i4082 = icmp eq i64 %_312.1.i.i189.i, %position.sroa.0.0.i.i161.i11141, !dbg !6943
  br i1 %_4.not.i4082, label %panic.i4084, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4085, !dbg !6943

panic.i4084:                                      ; preds = %bb81.i.i191.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6943, !noalias !6945
  unreachable, !dbg !6943

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4085: ; preds = %bb81.i.i191.i
  %_249.i.i194.i = getelementptr inbounds nuw float, ptr %_312.0.i.i192.i, i64 %position.sroa.0.0.i.i161.i11141, !dbg !6946
  store float %_0.i3437, ptr %_249.i.i194.i, align 4, !dbg !6943, !alias.scope !6939, !noalias !6731
  %_313.1.i.i195.i = load i64, ptr %479, align 8, !dbg !6951, !noalias !6731, !noundef !11
  %_250.i.i196.i = icmp ugt i64 %position.sroa.0.0.i.i161.i11141, %_313.1.i.i195.i, !dbg !6952
  br i1 %_250.i.i196.i, label %bb82.i.i311.i, label %bb83.i.i197.i, !dbg !6952, !prof !1664

bb80.i.i312.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4089
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i161.i11141, i64 noundef %_312.1.i.i189.i, i64 noundef %_312.1.i.i189.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8c2aade3368450b16e7e4997ad3fca81) #26, !dbg !6956, !noalias !6731
  unreachable, !dbg !6956

bb83.i.i197.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4085
  %_313.0.i.i198.i = load ptr, ptr %_18.i136.i, align 8, !dbg !6951, !noalias !6731, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6957), !dbg !6960
  %_4.not.i4078 = icmp eq i64 %_313.1.i.i195.i, %position.sroa.0.0.i.i161.i11141, !dbg !6961
  br i1 %_4.not.i4078, label %panic.i4080, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4081, !dbg !6961

panic.i4080:                                      ; preds = %bb83.i.i197.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6961, !noalias !6963
  unreachable, !dbg !6961

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4081: ; preds = %bb83.i.i197.i
  %_257.i.i200.i = getelementptr inbounds nuw float, ptr %_313.0.i.i198.i, i64 %position.sroa.0.0.i.i161.i11141, !dbg !6964
  store float %_0.i2719, ptr %_257.i.i200.i, align 4, !dbg !6961, !alias.scope !6957, !noalias !6731
  %_314.1.i.i201.i = load i64, ptr %481, align 8, !dbg !6969, !noalias !6731, !noundef !11
  %_258.i.i202.i = icmp ugt i64 %position.sroa.0.0.i.i161.i11141, %_314.1.i.i201.i, !dbg !6970
  br i1 %_258.i.i202.i, label %bb84.i.i310.i, label %bb85.i.i203.i, !dbg !6970, !prof !1664

bb82.i.i311.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4085
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i161.i11141, i64 noundef %_313.1.i.i195.i, i64 noundef %_313.1.i.i195.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a4c3da5a99763e9452b49d42852d67e3) #26, !dbg !6974, !noalias !6731
  unreachable, !dbg !6974

bb85.i.i203.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4081
  %_314.0.i.i204.i = load ptr, ptr %480, align 8, !dbg !6969, !noalias !6731, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6975), !dbg !6978
  %_4.not.i4074 = icmp eq i64 %_314.1.i.i201.i, %position.sroa.0.0.i.i161.i11141, !dbg !6979
  br i1 %_4.not.i4074, label %panic.i4076, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077, !dbg !6979

panic.i4076:                                      ; preds = %bb85.i.i203.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !6979, !noalias !6981
  unreachable, !dbg !6979

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077: ; preds = %bb85.i.i203.i
  %_265.i.i206.i = getelementptr inbounds nuw float, ptr %_314.0.i.i204.i, i64 %position.sroa.0.0.i.i161.i11141, !dbg !6982
  store float %_0.i3438, ptr %_265.i.i206.i, align 4, !dbg !6979, !alias.scope !6975, !noalias !6731
  %_315.0.i.i207.i = load ptr, ptr %425, align 8, !dbg !6987, !noalias !6731, !nonnull !11, !noundef !11
  %_315.1.i.i208.i = load i64, ptr %476, align 8, !dbg !6987, !noalias !6731, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6988), !dbg !6991
  %_8.i212.i209.i = load i64, ptr %_24.i.i156.i, align 8, !dbg !6992, !alias.scope !6988, !noalias !6994, !noundef !11
  %_7.i213.i210.i = add i64 %_8.i212.i209.i, %position.sroa.0.0.i.i161.i11141, !dbg !6996
  %_27.not.i214.i211.i = icmp ult i64 %_7.i213.i210.i, %ring_len.i128.i, !dbg !6997
  %526 = select i1 %_27.not.i214.i211.i, i64 0, i64 %ring_len.i128.i, !dbg !6997
  %row.sroa.0.0.i215.i212.i = sub nuw i64 %_7.i213.i210.i, %526, !dbg !6997
  %_28.i216.i213.i = icmp ugt i64 %row.sroa.0.0.i215.i212.i, %_315.1.i.i208.i, !dbg !6999
  br i1 %_28.i216.i213.i, label %bb14.i219.i309.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i214.i, !dbg !6999, !prof !1664

bb14.i219.i309.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i215.i212.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i.i208.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i.i208.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !7002, !noalias !7003
  unreachable, !dbg !7002

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i214.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4077
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7004), !dbg !7007
  %_3.not.i3652 = icmp eq i64 %_315.1.i.i208.i, %row.sroa.0.0.i215.i212.i, !dbg !7008
  br i1 %_3.not.i3652, label %panic.i3655, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3656, !dbg !7008

panic.i3655:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i214.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7008, !noalias !7010
  unreachable, !dbg !7008

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3656: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i214.i
  %_35.i218.i216.i = getelementptr inbounds nuw float, ptr %_315.0.i.i207.i, i64 %row.sroa.0.0.i215.i212.i, !dbg !7011
  %_0.i3654 = load float, ptr %_35.i218.i216.i, align 4, !dbg !7008, !alias.scope !7004, !noalias !7013, !noundef !11
  %_316.0.i.i217.i = load ptr, ptr %477, align 8, !dbg !7014, !noalias !6731, !nonnull !11, !noundef !11
  %_316.1.i.i218.i = load i64, ptr %478, align 8, !dbg !7014, !noalias !6731, !noundef !11
  %_28.i207.i223.i = icmp ugt i64 %row.sroa.0.0.i215.i212.i, %_316.1.i.i218.i, !dbg !7016
  br i1 %_28.i207.i223.i, label %bb14.i210.i308.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i224.i, !dbg !7016, !prof !1664

bb14.i210.i308.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3656
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i215.i212.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i.i218.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i.i218.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !7020, !noalias !7021
  unreachable, !dbg !7020

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i224.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3656
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7025), !dbg !7028
  %_3.not.i3647 = icmp eq i64 %_316.1.i.i218.i, %row.sroa.0.0.i215.i212.i, !dbg !7029
  br i1 %_3.not.i3647, label %panic.i3650, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3651, !dbg !7029

panic.i3650:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i224.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7029, !noalias !7031
  unreachable, !dbg !7029

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3651: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i224.i
  %_35.i209.i226.i = getelementptr inbounds nuw float, ptr %_316.0.i.i217.i, i64 %row.sroa.0.0.i215.i212.i, !dbg !7032
  %_0.i3649 = load float, ptr %_35.i209.i226.i, align 4, !dbg !7029, !alias.scope !7025, !noalias !7034, !noundef !11
  %_317.0.i.i227.i = load ptr, ptr %_18.i136.i, align 8, !dbg !7035, !noalias !6731, !nonnull !11, !noundef !11
  %_317.1.i.i228.i = load i64, ptr %479, align 8, !dbg !7035, !noalias !6731, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7037), !dbg !7040
  %_8.i194.i229.i = load i64, ptr %_26.i.i158.i, align 8, !dbg !7041, !alias.scope !7037, !noalias !7043, !noundef !11
  %_7.i195.i230.i = add i64 %_8.i194.i229.i, %position.sroa.0.0.i.i161.i11141, !dbg !7045
  %_27.not.i196.i231.i = icmp ult i64 %_7.i195.i230.i, %ring_len.i128.i, !dbg !7046
  %527 = select i1 %_27.not.i196.i231.i, i64 0, i64 %ring_len.i128.i, !dbg !7046
  %row.sroa.0.0.i197.i232.i = sub nuw i64 %_7.i195.i230.i, %527, !dbg !7046
  %_28.i198.i233.i = icmp ugt i64 %row.sroa.0.0.i197.i232.i, %_317.1.i.i228.i, !dbg !7048
  br i1 %_28.i198.i233.i, label %bb14.i201.i307.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i234.i, !dbg !7048, !prof !1664

bb14.i201.i307.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3651
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i197.i232.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i.i228.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i.i228.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !7051, !noalias !7052
  unreachable, !dbg !7051

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i234.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3651
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7053), !dbg !7056
  %_3.not.i3642 = icmp eq i64 %_317.1.i.i228.i, %row.sroa.0.0.i197.i232.i, !dbg !7057
  br i1 %_3.not.i3642, label %panic.i3645, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3646, !dbg !7057

panic.i3645:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i234.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7057, !noalias !7059
  unreachable, !dbg !7057

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3646: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i234.i
  %_35.i200.i236.i = getelementptr inbounds nuw float, ptr %_317.0.i.i227.i, i64 %row.sroa.0.0.i197.i232.i, !dbg !7060
  %_0.i3644 = load float, ptr %_35.i200.i236.i, align 4, !dbg !7057, !alias.scope !7053, !noalias !7062, !noundef !11
  %_318.0.i.i237.i = load ptr, ptr %480, align 8, !dbg !7063, !noalias !6731, !nonnull !11, !noundef !11
  %_318.1.i.i238.i = load i64, ptr %481, align 8, !dbg !7063, !noalias !6731, !noundef !11
  %_28.i189.i243.i = icmp ugt i64 %row.sroa.0.0.i197.i232.i, %_318.1.i.i238.i, !dbg !7065
  br i1 %_28.i189.i243.i, label %bb14.i192.i306.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i244.i, !dbg !7065, !prof !1664

bb14.i192.i306.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3646
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i197.i232.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i.i238.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i.i238.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !7069, !noalias !7070
  unreachable, !dbg !7069

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i244.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3646
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7074), !dbg !7077
  %_3.not.i3637 = icmp eq i64 %_318.1.i.i238.i, %row.sroa.0.0.i197.i232.i, !dbg !7078
  br i1 %_3.not.i3637, label %panic.i3640, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3641, !dbg !7078

panic.i3640:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i244.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7078, !noalias !7080
  unreachable, !dbg !7078

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3641: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i244.i
  %_35.i191.i246.i = getelementptr inbounds nuw float, ptr %_318.0.i.i237.i, i64 %row.sroa.0.0.i197.i232.i, !dbg !7081
  %_0.i3639 = load float, ptr %_35.i191.i246.i, align 4, !dbg !7078, !alias.scope !7074, !noalias !7083, !noundef !11
  %528 = tail call noundef float @llvm.fabs.f32(float %_0.i3654), !dbg !7084
  %529 = tail call noundef float @llvm.fabs.f32(float %_0.i3644), !dbg !7089
  %_3.i.i5314.inv = fcmp ogt float %528, %529, !dbg !7091
  %_4.i.i5321.v = select i1 %_3.i.i5314.inv, float %528, float %529, !dbg !7091
  %_4.i.i5321 = bitcast float %_4.i.i5321.v to i32, !dbg !7091
  %_3.i.i5726 = fcmp ule float %_4.i.i5321.v, 0x3E45798EE0000000, !dbg !7095
  %_4.i.i5732 = select i1 %_3.i.i5726, i32 841731191, i32 %_4.i.i5321, !dbg !7101
  %_0.i.i5733 = bitcast i32 %_4.i.i5732 to float, !dbg !7103
  %_3.i.i4994 = fcmp ule float %_0.i.i5733, 0x3810000000000000, !dbg !7105
  %_4.i.i5000 = select i1 %_3.i.i4994, i32 8388608, i32 %_4.i.i5732, !dbg !7110
  %_5.i3869 = and i32 %_4.i.i5000, 8388607, !dbg !7112
  %_4.i3870 = or disjoint i32 %_5.i3869, 1065353216, !dbg !7112
  %significand.i3871 = bitcast i32 %_4.i3870 to float, !dbg !7114
  %_0.i3345 = fadd float %significand.i3871, -1.000000e+00, !dbg !7116
  %_0.i3010 = fmul float %_0.i3345, 0xBF9B17A960000000, !dbg !7118
  %_0.i2530 = fadd float %_0.i3010, 0x3FBF9A8440000000, !dbg !7120
  %_0.i3010.1 = fmul float %_0.i3345, %_0.i2530, !dbg !7118
  %_0.i2530.1 = fadd float %_0.i3010.1, 0xBFD1E3F400000000, !dbg !7120
  %_0.i3010.2 = fmul float %_0.i3345, %_0.i2530.1, !dbg !7118
  %_0.i2530.2 = fadd float %_0.i3010.2, 0x3FDD544F20000000, !dbg !7120
  %_0.i3010.3 = fmul float %_0.i3345, %_0.i2530.2, !dbg !7118
  %_0.i2530.3 = fadd float %_0.i3010.3, 0xBFE6FC2A60000000, !dbg !7120
  %_0.i3010.4 = fmul float %_0.i3345, %_0.i2530.3, !dbg !7118
  %_0.i2530.4 = fadd float %_0.i3010.4, 0x3FF714B2A0000000, !dbg !7120
  %_9.i3872 = lshr i32 %_4.i.i5000, 23, !dbg !7122
  %_8.i3873 = or disjoint i32 %_9.i3872, 1258291200, !dbg !7122
  %_7.i3874 = bitcast i32 %_8.i3873 to float, !dbg !7123
  %exponent.i3875 = fadd float %_7.i3874, 0xC160000FE0000000, !dbg !7125
  %_0.i3009 = fmul float %_0.i3345, %_0.i2530.4, !dbg !7126
  %_0.i2529 = fadd float %exponent.i3875, %_0.i3009, !dbg !7128
  %_0.i3312 = fmul float %_0.i2529, 0x4018151820000000, !dbg !7130
  %_3.i.i5718.inv = fcmp ogt float %_0.i3312, -1.600000e+02, !dbg !7132
  %_0.i.i5725 = select i1 %_3.i.i5718.inv, float %_0.i3312, float -1.600000e+02, !dbg !7132
  %_3.i.i6364.inv = fcmp olt float %_0.i.i5725, 2.400000e+01, !dbg !7135
  %_0.i.i6371 = select i1 %_3.i.i6364.inv, float %_0.i.i5725, float 2.400000e+01, !dbg !7135
  %_0.i3393 = fsub float %_0.i.i6371, %_12.le.i6718, !dbg !7138
  %_3.i2241 = fcmp ule float %_0.i3393, 3.000000e+00, !dbg !7141
  %_0.i2617 = fadd float %_0.i3393, 3.000000e+00, !dbg !7143
  %_0.i3147 = fmul float %_0.i2617, %_0.i2617, !dbg !7145
  %_0.i3146 = fmul float %_0.i3147, 0x3FB5555560000000, !dbg !7147
  %_4.i4572.v.v = select i1 %_3.i2241, float %_0.i3146, float %_0.i3393, !dbg !7149
  %_4.i4572.v = fmul float %coefficients.i123.i.sroa.0.0.copyload, %_4.i4572.v.v, !dbg !7149
  %_4.i4572 = bitcast float %_4.i4572.v to i32, !dbg !7149
  %530 = fcmp ugt float %_0.i3393, -3.000000e+00, !dbg !7151
  %_7.i4564 = select i1 %530, i32 %_4.i4572, i32 0, !dbg !7153
  %_0.i4566 = bitcast i32 %_7.i4564 to float, !dbg !7154
  %_3.i.i5710 = fcmp ule float %_0.i4566, -1.000000e+02, !dbg !7156
  %531 = bitcast i32 %_7.i4564 to float, !dbg !7159
  %_0.i.i5717 = select i1 %_3.i.i5710, float -1.000000e+02, float %531, !dbg !7162
  %_3.i.i6356 = fcmp olt float %_0.i.i5717, 0.000000e+00, !dbg !7163
  %_0.i.i6363 = select i1 %_3.i.i6356, float %_0.i.i5717, float 0.000000e+00, !dbg !7166
  %532 = bitcast i32 %gain_near.i.i117.i.sroa.0.011137 to float, !dbg !7168
  %_3.i2479 = fcmp uge float %_0.i.i6363, %532, !dbg !7169
  %_4.i4878.v = select i1 %_3.i2479, float %coefficients.i123.i.sroa.7.0.copyload, float %coefficients.i123.i.sroa.5.0.copyload, !dbg !7172
  %_0.i3460 = fsub float %532, %_0.i.i6363, !dbg !7174
  %_4.i2975 = fmul float %_0.i3460, %_4.i4878.v, !dbg !7176
  %_0.i2976 = fadd float %_0.i.i6363, %_4.i2975, !dbg !7176
  %533 = tail call noundef float @llvm.fabs.f32(float %_0.i2976), !dbg !7178
  %_4.i4421 = bitcast float %_0.i2976 to i32, !dbg !7181
  %534 = fcmp uge float %533, 0x3BC79CA100000000, !dbg !7184
  %_3.i4423 = select i1 %534, i32 %_4.i4421, i32 0, !dbg !7185
  %_0.i4424 = bitcast i32 %_3.i4423 to float, !dbg !7186
  %_0.i2808 = fadd float %_12.le.4.i6726, %_0.i4424, !dbg !7188
  %_0.i3311 = fmul float %_0.i2808, 0x3FC542A5A0000000, !dbg !7190
  %_3.i.i5186.inv = fcmp ogt float %_0.i3311, -1.260000e+02, !dbg !7193
  %_0.i.i5193 = select i1 %_3.i.i5186.inv, float %_0.i3311, float -1.260000e+02, !dbg !7193
  %_3.i.i5988.inv = fcmp olt float %_0.i.i5193, 1.270000e+02, !dbg !7197
  %_0.i.i5995 = select i1 %_3.i.i5988.inv, float %_0.i.i5193, float 1.270000e+02, !dbg !7197
  %535 = tail call noundef float @llvm.floor.f32(float %_0.i.i5995), !dbg !7200
  %_0.i3369 = fsub float %_0.i.i5995, %535, !dbg !7204
  %536 = tail call noundef float @llvm.fabs.f32(float %_0.i3649), !dbg !7206
  %537 = tail call noundef float @llvm.fabs.f32(float %_0.i3639), !dbg !7209
  %_3.i.i5323.inv = fcmp ogt float %536, %537, !dbg !7211
  %_4.i.i5330.v = select i1 %_3.i.i5323.inv, float %536, float %537, !dbg !7211
  %_4.i.i5330 = bitcast float %_4.i.i5330.v to i32, !dbg !7211
  %_3.i.i5702 = fcmp ule float %_4.i.i5330.v, 0x3E45798EE0000000, !dbg !7214
  %_4.i.i5708 = select i1 %_3.i.i5702, i32 841731191, i32 %_4.i.i5330, !dbg !7219
  %_0.i.i5709 = bitcast i32 %_4.i.i5708 to float, !dbg !7221
  %_3.i.i5002 = fcmp ule float %_0.i.i5709, 0x3810000000000000, !dbg !7223
  %_4.i.i5008 = select i1 %_3.i.i5002, i32 8388608, i32 %_4.i.i5708, !dbg !7228
  %_5.i3877 = and i32 %_4.i.i5008, 8388607, !dbg !7230
  %_4.i3878 = or disjoint i32 %_5.i3877, 1065353216, !dbg !7230
  %significand.i3879 = bitcast i32 %_4.i3878 to float, !dbg !7232
  %_0.i3346 = fadd float %significand.i3879, -1.000000e+00, !dbg !7234
  %_0.i3012 = fmul float %_0.i3346, 0xBF9B17A960000000, !dbg !7236
  %_0.i2532 = fadd float %_0.i3012, 0x3FBF9A8440000000, !dbg !7238
  %_0.i3012.1 = fmul float %_0.i3346, %_0.i2532, !dbg !7236
  %_0.i2532.1 = fadd float %_0.i3012.1, 0xBFD1E3F400000000, !dbg !7238
  %_0.i3012.2 = fmul float %_0.i3346, %_0.i2532.1, !dbg !7236
  %_0.i2532.2 = fadd float %_0.i3012.2, 0x3FDD544F20000000, !dbg !7238
  %_0.i3012.3 = fmul float %_0.i3346, %_0.i2532.2, !dbg !7236
  %_0.i2532.3 = fadd float %_0.i3012.3, 0xBFE6FC2A60000000, !dbg !7238
  %_0.i3012.4 = fmul float %_0.i3346, %_0.i2532.3, !dbg !7236
  %_0.i2532.4 = fadd float %_0.i3012.4, 0x3FF714B2A0000000, !dbg !7238
  %_9.i3880 = lshr i32 %_4.i.i5008, 23, !dbg !7240
  %_8.i3881 = or disjoint i32 %_9.i3880, 1258291200, !dbg !7240
  %_7.i3882 = bitcast i32 %_8.i3881 to float, !dbg !7241
  %exponent.i3883 = fadd float %_7.i3882, 0xC160000FE0000000, !dbg !7243
  %_0.i3011 = fmul float %_0.i3346, %_0.i2532.4, !dbg !7244
  %_0.i2531 = fadd float %exponent.i3883, %_0.i3011, !dbg !7246
  %_0.i3310 = fmul float %_0.i2531, 0x4018151820000000, !dbg !7248
  %_3.i.i5694.inv = fcmp ogt float %_0.i3310, -1.600000e+02, !dbg !7250
  %_0.i.i5701 = select i1 %_3.i.i5694.inv, float %_0.i3310, float -1.600000e+02, !dbg !7250
  %_3.i.i6348.inv = fcmp olt float %_0.i.i5701, 2.400000e+01, !dbg !7253
  %_0.i.i6355 = select i1 %_3.i.i6348.inv, float %_0.i.i5701, float 2.400000e+01, !dbg !7253
  %_0.i3394 = fsub float %_0.i.i6355, %_12.le.5.i6728, !dbg !7256
  %_3.i2243 = fcmp ule float %_0.i3394, 3.000000e+00, !dbg !7259
  %_0.i2618 = fadd float %_0.i3394, 3.000000e+00, !dbg !7261
  %_0.i3151 = fmul float %_0.i2618, %_0.i2618, !dbg !7263
  %_0.i3150 = fmul float %_0.i3151, 0x3FB5555560000000, !dbg !7265
  %_4.i4585.v.v = select i1 %_3.i2243, float %_0.i3150, float %_0.i3394, !dbg !7267
  %_4.i4585.v = fmul float %coefficients.i123.i.sroa.9.0.copyload, %_4.i4585.v.v, !dbg !7267
  %_4.i4585 = bitcast float %_4.i4585.v to i32, !dbg !7267
  %538 = fcmp ugt float %_0.i3394, -3.000000e+00, !dbg !7269
  %_7.i4577 = select i1 %538, i32 %_4.i4585, i32 0, !dbg !7271
  %_0.i4579 = bitcast i32 %_7.i4577 to float, !dbg !7272
  %_3.i.i5686 = fcmp ule float %_0.i4579, -1.000000e+02, !dbg !7274
  %539 = bitcast i32 %_7.i4577 to float, !dbg !7277
  %_0.i.i5693 = select i1 %_3.i.i5686, float -1.000000e+02, float %539, !dbg !7280
  %_3.i.i6340 = fcmp olt float %_0.i.i5693, 0.000000e+00, !dbg !7281
  %_0.i.i6347 = select i1 %_3.i.i6340, float %_0.i.i5693, float 0.000000e+00, !dbg !7284
  %540 = bitcast i32 %gain_near.i.i117.i.sroa.6.011138 to float, !dbg !7286
  %_3.i2475 = fcmp uge float %_0.i.i6347, %540, !dbg !7287
  %_4.i4871.v = select i1 %_3.i2475, float %coefficients.i123.i.sroa.13.0.copyload, float %coefficients.i123.i.sroa.11.0.copyload, !dbg !7290
  %_0.i3459 = fsub float %540, %_0.i.i6347, !dbg !7292
  %_4.i2973 = fmul float %_0.i3459, %_4.i4871.v, !dbg !7294
  %_0.i2974 = fadd float %_0.i.i6347, %_4.i2973, !dbg !7294
  %541 = tail call noundef float @llvm.fabs.f32(float %_0.i2974), !dbg !7296
  %_4.i4417 = bitcast float %_0.i2974 to i32, !dbg !7299
  %542 = fcmp uge float %541, 0x3BC79CA100000000, !dbg !7302
  %_3.i4419 = select i1 %542, i32 %_4.i4417, i32 0, !dbg !7303
  %_0.i4420 = bitcast i32 %_3.i4419 to float, !dbg !7304
  %_0.i2807 = fadd float %_12.le.9.i6736, %_0.i4420, !dbg !7306
  %_0.i3309 = fmul float %_0.i2807, 0x3FC542A5A0000000, !dbg !7308
  %_3.i.i5194.inv = fcmp ogt float %_0.i3309, -1.260000e+02, !dbg !7311
  %_0.i.i5201 = select i1 %_3.i.i5194.inv, float %_0.i3309, float -1.260000e+02, !dbg !7311
  %_3.i.i5996.inv = fcmp olt float %_0.i.i5201, 1.270000e+02, !dbg !7315
  %_0.i.i6003 = select i1 %_3.i.i5996.inv, float %_0.i.i5201, float 1.270000e+02, !dbg !7315
  %543 = tail call noundef float @llvm.floor.f32(float %_0.i.i6003), !dbg !7318
  %_0.i3370 = fsub float %_0.i.i6003, %543, !dbg !7322
  %_0.i3395 = fsub float %_0.i.i6371, %_12.le.i6756, !dbg !7324
  %_3.i2245 = fcmp ule float %_0.i3395, 3.000000e+00, !dbg !7329
  %_0.i2619 = fadd float %_0.i3395, 3.000000e+00, !dbg !7331
  %_0.i3155 = fmul float %_0.i2619, %_0.i2619, !dbg !7333
  %_0.i3154 = fmul float %_0.i3155, 0x3FB5555560000000, !dbg !7335
  %_4.i4598.v.v = select i1 %_3.i2245, float %_0.i3154, float %_0.i3395, !dbg !7337
  %_4.i4598.v = fmul float %coefficients.i123.i.sroa.15.24.copyload, %_4.i4598.v.v, !dbg !7337
  %_4.i4598 = bitcast float %_4.i4598.v to i32, !dbg !7337
  %544 = fcmp ugt float %_0.i3395, -3.000000e+00, !dbg !7339
  %_7.i4590 = select i1 %544, i32 %_4.i4598, i32 0, !dbg !7341
  %_0.i4592 = bitcast i32 %_7.i4590 to float, !dbg !7342
  %_3.i.i5662 = fcmp ule float %_0.i4592, -1.000000e+02, !dbg !7344
  %545 = bitcast i32 %_7.i4590 to float, !dbg !7347
  %_0.i.i5669 = select i1 %_3.i.i5662, float -1.000000e+02, float %545, !dbg !7350
  %_3.i.i6324 = fcmp olt float %_0.i.i5669, 0.000000e+00, !dbg !7351
  %_0.i.i6331 = select i1 %_3.i.i6324, float %_0.i.i5669, float 0.000000e+00, !dbg !7354
  %546 = bitcast i32 %gain_far.i.i116.i.sroa.0.011139 to float, !dbg !7356
  %_3.i2471 = fcmp uge float %_0.i.i6331, %546, !dbg !7357
  %_4.i4864.v = select i1 %_3.i2471, float %coefficients.i123.i.sroa.20.24.copyload, float %coefficients.i123.i.sroa.18.24.copyload, !dbg !7360
  %_0.i3458 = fsub float %546, %_0.i.i6331, !dbg !7362
  %_4.i2971 = fmul float %_0.i3458, %_4.i4864.v, !dbg !7364
  %_0.i2972 = fadd float %_0.i.i6331, %_4.i2971, !dbg !7364
  %547 = tail call noundef float @llvm.fabs.f32(float %_0.i2972), !dbg !7366
  %_4.i4413 = bitcast float %_0.i2972 to i32, !dbg !7369
  %548 = fcmp uge float %547, 0x3BC79CA100000000, !dbg !7372
  %_3.i4415 = select i1 %548, i32 %_4.i4413, i32 0, !dbg !7373
  %_0.i4416 = bitcast i32 %_3.i4415 to float, !dbg !7374
  %_0.i2806 = fadd float %_12.le.4.i6764, %_0.i4416, !dbg !7376
  %_0.i3307 = fmul float %_0.i2806, 0x3FC542A5A0000000, !dbg !7378
  %_3.i.i5202.inv = fcmp ogt float %_0.i3307, -1.260000e+02, !dbg !7381
  %_0.i.i5209 = select i1 %_3.i.i5202.inv, float %_0.i3307, float -1.260000e+02, !dbg !7381
  %_3.i.i6004.inv = fcmp olt float %_0.i.i5209, 1.270000e+02, !dbg !7385
  %_0.i.i6011 = select i1 %_3.i.i6004.inv, float %_0.i.i5209, float 1.270000e+02, !dbg !7385
  %549 = tail call noundef float @llvm.floor.f32(float %_0.i.i6011), !dbg !7388
  %_0.i3371 = fsub float %_0.i.i6011, %549, !dbg !7392
  %_0.i3396 = fsub float %_0.i.i6355, %_12.le.5.i6766, !dbg !7394
  %_3.i2247 = fcmp ule float %_0.i3396, 3.000000e+00, !dbg !7399
  %_0.i2620 = fadd float %_0.i3396, 3.000000e+00, !dbg !7401
  %_0.i3159 = fmul float %_0.i2620, %_0.i2620, !dbg !7403
  %_0.i3158 = fmul float %_0.i3159, 0x3FB5555560000000, !dbg !7405
  %_4.i4611.v.v = select i1 %_3.i2247, float %_0.i3158, float %_0.i3396, !dbg !7407
  %_4.i4611.v = fmul float %coefficients.i123.i.sroa.22.24.copyload, %_4.i4611.v.v, !dbg !7407
  %_4.i4611 = bitcast float %_4.i4611.v to i32, !dbg !7407
  %550 = fcmp ugt float %_0.i3396, -3.000000e+00, !dbg !7409
  %_7.i4603 = select i1 %550, i32 %_4.i4611, i32 0, !dbg !7411
  %_0.i4605 = bitcast i32 %_7.i4603 to float, !dbg !7412
  %_3.i.i5638 = fcmp ule float %_0.i4605, -1.000000e+02, !dbg !7414
  %551 = bitcast i32 %_7.i4603 to float, !dbg !7417
  %_0.i.i5645 = select i1 %_3.i.i5638, float -1.000000e+02, float %551, !dbg !7420
  %_3.i.i6308 = fcmp olt float %_0.i.i5645, 0.000000e+00, !dbg !7421
  %_0.i.i6315 = select i1 %_3.i.i6308, float %_0.i.i5645, float 0.000000e+00, !dbg !7424
  %552 = bitcast i32 %gain_far.i.i116.i.sroa.6.011140 to float, !dbg !7426
  %_3.i2467 = fcmp uge float %_0.i.i6315, %552, !dbg !7427
  %_4.i4857.v = select i1 %_3.i2467, float %coefficients.i123.i.sroa.26.24.copyload, float %coefficients.i123.i.sroa.24.24.copyload, !dbg !7430
  %_0.i3457 = fsub float %552, %_0.i.i6315, !dbg !7432
  %_4.i2969 = fmul float %_0.i3457, %_4.i4857.v, !dbg !7434
  %_0.i2970 = fadd float %_0.i.i6315, %_4.i2969, !dbg !7434
  %553 = tail call noundef float @llvm.fabs.f32(float %_0.i2970), !dbg !7436
  %_4.i4409 = bitcast float %_0.i2970 to i32, !dbg !7439
  %554 = fcmp uge float %553, 0x3BC79CA100000000, !dbg !7442
  %_3.i4411 = select i1 %554, i32 %_4.i4409, i32 0, !dbg !7443
  %_0.i4412 = bitcast i32 %_3.i4411 to float, !dbg !7444
  %_0.i2805 = fadd float %_12.le.9.i6774, %_0.i4412, !dbg !7446
  %_0.i3305 = fmul float %_0.i2805, 0x3FC542A5A0000000, !dbg !7448
  %_3.i.i5210.inv = fcmp ogt float %_0.i3305, -1.260000e+02, !dbg !7451
  %_0.i.i5217 = select i1 %_3.i.i5210.inv, float %_0.i3305, float -1.260000e+02, !dbg !7451
  %_3.i.i6012.inv = fcmp olt float %_0.i.i5217, 1.270000e+02, !dbg !7455
  %_0.i.i6019 = select i1 %_3.i.i6012.inv, float %_0.i.i5217, float 1.270000e+02, !dbg !7455
  %555 = tail call noundef float @llvm.floor.f32(float %_0.i.i6019), !dbg !7458
  %_0.i3372 = fsub float %_0.i.i6019, %555, !dbg !7462
  %_0.i3076 = fmul float %_0.i3372, 0x3F5E974FA0000000, !dbg !7464
  %_0.i2584 = fadd float %_0.i3076, 0x3F82778560000000, !dbg !7466
  %_0.i3076.1 = fmul float %_0.i3372, %_0.i2584, !dbg !7464
  %_0.i2584.1 = fadd float %_0.i3076.1, 0x3FAC91CE60000000, !dbg !7466
  %_0.i3076.2 = fmul float %_0.i3372, %_0.i2584.1, !dbg !7464
  %_0.i2584.2 = fadd float %_0.i3076.2, 0x3FCEBDB560000000, !dbg !7466
  %_0.i3076.3 = fmul float %_0.i3372, %_0.i2584.2, !dbg !7464
  %_0.i2584.3 = fadd float %_0.i3076.3, 0x3FE62E4BA0000000, !dbg !7466
  %_0.i3073 = fmul float %_0.i3371, 0x3F5E974FA0000000, !dbg !7468
  %_0.i2582 = fadd float %_0.i3073, 0x3F82778560000000, !dbg !7470
  %_0.i3073.1 = fmul float %_0.i3371, %_0.i2582, !dbg !7468
  %_0.i2582.1 = fadd float %_0.i3073.1, 0x3FAC91CE60000000, !dbg !7470
  %_0.i3073.2 = fmul float %_0.i3371, %_0.i2582.1, !dbg !7468
  %_0.i2582.2 = fadd float %_0.i3073.2, 0x3FCEBDB560000000, !dbg !7470
  %_0.i3073.3 = fmul float %_0.i3371, %_0.i2582.2, !dbg !7468
  %_0.i2582.3 = fadd float %_0.i3073.3, 0x3FE62E4BA0000000, !dbg !7470
  %_0.i3070 = fmul float %_0.i3370, 0x3F5E974FA0000000, !dbg !7472
  %_0.i2580 = fadd float %_0.i3070, 0x3F82778560000000, !dbg !7474
  %_0.i3070.1 = fmul float %_0.i3370, %_0.i2580, !dbg !7472
  %_0.i2580.1 = fadd float %_0.i3070.1, 0x3FAC91CE60000000, !dbg !7474
  %_0.i3070.2 = fmul float %_0.i3370, %_0.i2580.1, !dbg !7472
  %_0.i2580.2 = fadd float %_0.i3070.2, 0x3FCEBDB560000000, !dbg !7474
  %_0.i3070.3 = fmul float %_0.i3370, %_0.i2580.2, !dbg !7472
  %_0.i2580.3 = fadd float %_0.i3070.3, 0x3FE62E4BA0000000, !dbg !7474
  %_0.i3067 = fmul float %_0.i3369, 0x3F5E974FA0000000, !dbg !7476
  %_0.i2578 = fadd float %_0.i3067, 0x3F82778560000000, !dbg !7478
  %_0.i3067.1 = fmul float %_0.i3369, %_0.i2578, !dbg !7476
  %_0.i2578.1 = fadd float %_0.i3067.1, 0x3FAC91CE60000000, !dbg !7478
  %_0.i3067.2 = fmul float %_0.i3369, %_0.i2578.1, !dbg !7476
  %_0.i2578.2 = fadd float %_0.i3067.2, 0x3FCEBDB560000000, !dbg !7478
  %_0.i3067.3 = fmul float %_0.i3369, %_0.i2578.2, !dbg !7476
  %_0.i2578.3 = fadd float %_0.i3067.3, 0x3FE62E4BA0000000, !dbg !7478
  %_0.i3066 = fmul float %_0.i3369, %_0.i2578.3, !dbg !7480
  %_0.i2577 = fadd float %_0.i3066, 1.000000e+00, !dbg !7482
  %biased.i2162 = fadd float %535, 0x4160000FE0000000, !dbg !7484
  %_4.i2163 = bitcast float %biased.i2162 to i32, !dbg !7486
  %_3.i2164 = shl i32 %_4.i2163, 23, !dbg !7488
  %_0.i2165 = bitcast i32 %_3.i2164 to float, !dbg !7489
  %_0.i3065 = fmul float %_0.i2577, %_0.i2165, !dbg !7491
  %_0.i3069 = fmul float %_0.i3370, %_0.i2580.3, !dbg !7493
  %_0.i2579 = fadd float %_0.i3069, 1.000000e+00, !dbg !7495
  %biased.i2166 = fadd float %543, 0x4160000FE0000000, !dbg !7497
  %_4.i2167 = bitcast float %biased.i2166 to i32, !dbg !7499
  %_3.i2168 = shl i32 %_4.i2167, 23, !dbg !7501
  %_0.i2169 = bitcast i32 %_3.i2168 to float, !dbg !7502
  %_0.i3068 = fmul float %_0.i2579, %_0.i2169, !dbg !7504
  %_0.i3072 = fmul float %_0.i3371, %_0.i2582.3, !dbg !7506
  %_0.i2581 = fadd float %_0.i3072, 1.000000e+00, !dbg !7508
  %biased.i2170 = fadd float %549, 0x4160000FE0000000, !dbg !7510
  %_4.i2171 = bitcast float %biased.i2170 to i32, !dbg !7512
  %_3.i2172 = shl i32 %_4.i2171, 23, !dbg !7514
  %_0.i2173 = bitcast i32 %_3.i2172 to float, !dbg !7515
  %_0.i3071 = fmul float %_0.i2581, %_0.i2173, !dbg !7517
  %_0.i3075 = fmul float %_0.i3372, %_0.i2584.3, !dbg !7519
  %_0.i2583 = fadd float %_0.i3075, 1.000000e+00, !dbg !7521
  %biased.i2174 = fadd float %555, 0x4160000FE0000000, !dbg !7523
  %_4.i2175 = bitcast float %biased.i2174 to i32, !dbg !7525
  %_3.i2176 = shl i32 %_4.i2175, 23, !dbg !7527
  %_0.i2177 = bitcast i32 %_3.i2176 to float, !dbg !7528
  %_0.i3074 = fmul float %_0.i2583, %_0.i2177, !dbg !7530
  %_266.i.i268.i = icmp ugt i64 %_41.sroa.0.0.i.i170.i, %_315.1.i.i208.i, !dbg !7532
  br i1 %_266.i.i268.i, label %bb86.i.i305.i, label %bb87.i.i269.i, !dbg !7532, !prof !1664

bb84.i.i310.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4081
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i161.i11141, i64 noundef %_314.1.i.i201.i, i64 noundef %_314.1.i.i201.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e11ba0c4c1124bbcae4603a2ae121338) #26, !dbg !7537, !noalias !6731
  unreachable, !dbg !7537

bb87.i.i269.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3641
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7538), !dbg !7541
  %_3.not.i3632 = icmp eq i64 %_315.1.i.i208.i, %_41.sroa.0.0.i.i170.i, !dbg !7542
  br i1 %_3.not.i3632, label %panic.i3635, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3636, !dbg !7542

panic.i3635:                                      ; preds = %bb87.i.i269.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7542, !noalias !7544
  unreachable, !dbg !7542

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3636: ; preds = %bb87.i.i269.i
  %_273.i.i272.i = getelementptr inbounds nuw float, ptr %_315.0.i.i207.i, i64 %_41.sroa.0.0.i.i170.i, !dbg !7545
  %_0.i3634 = load float, ptr %_273.i.i272.i, align 4, !dbg !7542, !alias.scope !7538, !noalias !6731, !noundef !11
  %_0.i3304 = fmul float %_0.i3065, %_0.i3634, !dbg !7550
  %_274.i.i276.i = icmp ugt i64 %_41.sroa.0.0.i.i170.i, %_316.1.i.i218.i, !dbg !7552
  br i1 %_274.i.i276.i, label %bb88.i.i304.i, label %bb89.i.i277.i, !dbg !7552, !prof !1664

bb86.i.i305.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3641
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i170.i, i64 noundef %_315.1.i.i208.i, i64 noundef %_315.1.i.i208.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_379b93204ee4659b79f4b07b6520076a) #26, !dbg !7556, !noalias !6731
  unreachable, !dbg !7556

bb89.i.i277.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3636
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7557), !dbg !7560
  %_3.not.i3627 = icmp eq i64 %_316.1.i.i218.i, %_41.sroa.0.0.i.i170.i, !dbg !7561
  br i1 %_3.not.i3627, label %panic.i3630, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3631, !dbg !7561

panic.i3630:                                      ; preds = %bb89.i.i277.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7561, !noalias !7563
  unreachable, !dbg !7561

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3631: ; preds = %bb89.i.i277.i
  %_281.i.i280.i = getelementptr inbounds nuw float, ptr %_316.0.i.i217.i, i64 %_41.sroa.0.0.i.i170.i, !dbg !7564
  %_0.i3629 = load float, ptr %_281.i.i280.i, align 4, !dbg !7561, !alias.scope !7557, !noalias !6731, !noundef !11
  %_0.i3303 = fmul float %_0.i3068, %_0.i3629, !dbg !7569
  %_0.i2804 = fadd float %_0.i3304, %_0.i3303, !dbg !7571
  %_282.i.i285.i = icmp ugt i64 %_41.sroa.0.0.i.i170.i, %_317.1.i.i228.i, !dbg !7573
  br i1 %_282.i.i285.i, label %bb90.i.i303.i, label %bb91.i.i286.i, !dbg !7573, !prof !1664

bb88.i.i304.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3636
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i170.i, i64 noundef %_316.1.i.i218.i, i64 noundef %_316.1.i.i218.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2bb8eb0542a889f29ef069491eb97a17) #26, !dbg !7578, !noalias !6731
  unreachable, !dbg !7578

bb91.i.i286.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3631
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7579), !dbg !7582
  %_3.not.i3622 = icmp eq i64 %_317.1.i.i228.i, %_41.sroa.0.0.i.i170.i, !dbg !7583
  br i1 %_3.not.i3622, label %panic.i3625, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3626, !dbg !7583

panic.i3625:                                      ; preds = %bb91.i.i286.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7583, !noalias !7585
  unreachable, !dbg !7583

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3626: ; preds = %bb91.i.i286.i
  %_289.i.i289.i = getelementptr inbounds nuw float, ptr %_317.0.i.i227.i, i64 %_41.sroa.0.0.i.i170.i, !dbg !7586
  %_0.i3624 = load float, ptr %_289.i.i289.i, align 4, !dbg !7583, !alias.scope !7579, !noalias !6731, !noundef !11
  %_0.i3302 = fmul float %_0.i3071, %_0.i3624, !dbg !7591
  %_290.i.i293.i = icmp ugt i64 %_41.sroa.0.0.i.i170.i, %_318.1.i.i238.i, !dbg !7593
  br i1 %_290.i.i293.i, label %bb92.i.i302.i, label %bb93.i.i294.i, !dbg !7593, !prof !1664

bb90.i.i303.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3631
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i170.i, i64 noundef %_317.1.i.i228.i, i64 noundef %_317.1.i.i228.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd96798e807157d7db308788cb4dd125) #26, !dbg !7597, !noalias !6731
  unreachable, !dbg !7597

bb93.i.i294.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3626
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7598), !dbg !7601
  %_3.not.i3617 = icmp eq i64 %_318.1.i.i238.i, %_41.sroa.0.0.i.i170.i, !dbg !7602
  br i1 %_3.not.i3617, label %panic.i3620, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069, !dbg !7602

panic.i3620:                                      ; preds = %bb93.i.i294.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7602, !noalias !7604
  unreachable, !dbg !7602

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069: ; preds = %bb93.i.i294.i
  %_297.i.i297.i = getelementptr inbounds nuw float, ptr %_318.0.i.i237.i, i64 %_41.sroa.0.0.i.i170.i, !dbg !7605
  %_0.i3619 = load float, ptr %_297.i.i297.i, align 4, !dbg !7602, !alias.scope !7598, !noalias !6731, !noundef !11
  %_0.i3301 = fmul float %_0.i3074, %_0.i3619, !dbg !7610
  %_0.i2803 = fadd float %_0.i3302, %_0.i3301, !dbg !7612
  store float %_0.i2804, ptr %_184.i.i172.i, align 4, !dbg !7614, !alias.scope !7617, !noalias !6731
  store float %_0.i2803, ptr %_192.i.i175.i, align 4, !dbg !7620, !alias.scope !7622, !noalias !6731
  %exitcond14307.not = icmp eq i64 %508, %plan.0.i134.i, !dbg !6690
  br i1 %exitcond14307.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_KBZ_EB2_.exit.i.i, label %bb56.i.i167.i, !dbg !6702

bb92.i.i302.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3626
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i170.i, i64 noundef %_318.1.i.i238.i, i64 noundef %_318.1.i.i238.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_164adf6876ccf79975d46e129e248ca5) #26, !dbg !7625, !noalias !6731
  unreachable, !dbg !7625

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_KBZ_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069, %bb27.i143.i
  %filter_near.i.i119.i.sroa.0.0.lcssa = phi float [ %filter_near.i.i119.i.sroa.0.0.copyload, %bb27.i143.i ], [ %_0.i4288, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7626
  %filter_near.i.i119.i.sroa.7.0.lcssa = phi float [ %filter_near.i.i119.i.sroa.7.0.copyload, %bb27.i143.i ], [ %_0.i4284, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7626
  %filter_near.i.i119.i.sroa.11.0.lcssa = phi float [ %filter_near.i.i119.i.sroa.11.0.copyload, %bb27.i143.i ], [ %_0.i4296, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7626
  %filter_near.i.i119.i.sroa.14.0.lcssa = phi float [ %filter_near.i.i119.i.sroa.14.0.copyload, %bb27.i143.i ], [ %_0.i4292, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7626
  %filter_far.i.i118.i.sroa.0.0.lcssa = phi float [ %filter_far.i.i118.i.sroa.0.0.copyload, %bb27.i143.i ], [ %_0.i4272, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7627
  %filter_far.i.i118.i.sroa.7.0.lcssa = phi float [ %filter_far.i.i118.i.sroa.7.0.copyload, %bb27.i143.i ], [ %_0.i4268, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7627
  %filter_far.i.i118.i.sroa.11.0.lcssa = phi float [ %filter_far.i.i118.i.sroa.11.0.copyload, %bb27.i143.i ], [ %_0.i4280, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7627
  %filter_far.i.i118.i.sroa.14.0.lcssa = phi float [ %filter_far.i.i118.i.sroa.14.0.copyload, %bb27.i143.i ], [ %_0.i4276, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7627
  %gain_near.i.i117.i.sroa.0.0.lcssa = phi i32 [ %503, %bb27.i143.i ], [ %_3.i4423, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7628
  %gain_near.i.i117.i.sroa.6.0.lcssa = phi i32 [ %504, %bb27.i143.i ], [ %_3.i4419, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7628
  %gain_far.i.i116.i.sroa.0.0.lcssa = phi i32 [ %505, %bb27.i143.i ], [ %_3.i4415, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7629
  %gain_far.i.i116.i.sroa.6.0.lcssa = phi i32 [ %506, %bb27.i143.i ], [ %_3.i4411, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7629
  %position.sroa.0.0.i.i161.i.lcssa = phi i64 [ %507, %bb27.i143.i ], [ %_41.sroa.0.0.i.i170.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4069 ], !dbg !7630
  store float %filter_near.i.i119.i.sroa.0.0.lcssa, ptr %466, align 8, !dbg !7631, !noalias !6731
  store float %filter_near.i.i119.i.sroa.7.0.lcssa, ptr %filter_near.i.i119.i.sroa.7.0..sroa_idx, align 4, !dbg !7631, !noalias !6731
  store float %filter_near.i.i119.i.sroa.11.0.lcssa, ptr %filter_near.i.i119.i.sroa.11.0..sroa_idx, align 8, !dbg !7631, !noalias !6731
  store float %filter_near.i.i119.i.sroa.14.0.lcssa, ptr %filter_near.i.i119.i.sroa.14.0..sroa_idx, align 4, !dbg !7631, !noalias !6731
  store float %filter_far.i.i118.i.sroa.0.0.lcssa, ptr %467, align 8, !dbg !7632, !noalias !6731
  store float %filter_far.i.i118.i.sroa.7.0.lcssa, ptr %filter_far.i.i118.i.sroa.7.0..sroa_idx, align 4, !dbg !7632, !noalias !6731
  store float %filter_far.i.i118.i.sroa.11.0.lcssa, ptr %filter_far.i.i118.i.sroa.11.0..sroa_idx, align 8, !dbg !7632, !noalias !6731
  store float %filter_far.i.i118.i.sroa.14.0.lcssa, ptr %filter_far.i.i118.i.sroa.14.0..sroa_idx, align 4, !dbg !7632, !noalias !6731
  store i32 %gain_near.i.i117.i.sroa.0.0.lcssa, ptr %468, align 8, !dbg !7633, !noalias !6731
  store i32 %gain_near.i.i117.i.sroa.6.0.lcssa, ptr %.sroa_idx7061, align 4, !dbg !7633, !noalias !6731
  store i32 %gain_far.i.i116.i.sroa.0.0.lcssa, ptr %469, align 8, !dbg !7634, !noalias !6731
  store i32 %gain_far.i.i116.i.sroa.6.0.lcssa, ptr %.sroa_idx7066, align 4, !dbg !7634, !noalias !6731
  store i64 %position.sroa.0.0.i.i161.i.lcssa, ptr %_51.i138.i, align 8, !dbg !7635, !alias.scope !6687, !noalias !6688
  br label %bb15.i166.i, !dbg !7636

bb15.i166.i:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_Kb1_EB2_.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_KBZ_EB2_.exit.i.i
  %_8.i131.i = icmp ult i64 %_32.i320.i, %_19.1, !dbg !6609
  br i1 %_8.i131.i, label %bb2.i132.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit, !dbg !6609

bb17.i324.i:                                      ; preds = %bb9.i318.i
  %_75.i325.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i130.i11186, !dbg !7637
  %_85.i328.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i130.i11186, !dbg !7640
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7647), !dbg !7650
  %filter_near.i17.i112.i.sroa.0.0.copyload = load float, ptr %466, align 8, !dbg !7651, !noalias !7657
  %filter_near.i17.i112.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.7.0..sroa_idx, align 4, !dbg !7651, !noalias !7657
  %filter_near.i17.i112.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.11.0..sroa_idx, align 8, !dbg !7651, !noalias !7657
  %filter_near.i17.i112.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i119.i.sroa.14.0..sroa_idx, align 4, !dbg !7651, !noalias !7657
  %filter_far.i16.i111.i.sroa.0.0.copyload = load float, ptr %467, align 8, !dbg !7662, !noalias !7657
  %filter_far.i16.i111.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.7.0..sroa_idx, align 4, !dbg !7662, !noalias !7657
  %filter_far.i16.i111.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.11.0..sroa_idx, align 8, !dbg !7662, !noalias !7657
  %filter_far.i16.i111.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i118.i.sroa.14.0..sroa_idx, align 4, !dbg !7662, !noalias !7657
  %556 = load i32, ptr %468, align 8, !dbg !7664
  %557 = load i32, ptr %.sroa_idx7061, align 4, !dbg !7664
  %558 = load i32, ptr %469, align 8, !dbg !7666
  %559 = load i32, ptr %.sroa_idx7066, align 4, !dbg !7666
  %560 = load i64, ptr %_51.i138.i, align 8, !dbg !7668, !alias.scope !7670, !noalias !7671, !noundef !11
  %_168.i34.i344.i11157.not = icmp eq i64 %plan.0.i134.i, 0, !dbg !7673
  br i1 %_168.i34.i344.i11157.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_Kb1_EB2_.exit.i.i, label %bb56.i37.i348.i, !dbg !7685

bb19.i512.i:                                      ; preds = %bb9.i318.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i130.i11186, i64 noundef %_32.i320.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_683f9160cdc5ee4596b2ecf709188a9a) #26, !dbg !7686, !noalias !3842
  unreachable, !dbg !7686

bb56.i37.i348.i:                                  ; preds = %bb17.i324.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045
  %segments.i126.i.sroa.0.1 = phi float [ %_0.i2796, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.i6718, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.8.1 = phi float [ %_0.i2796.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.1.i6720, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.12.1 = phi float [ %_0.i2796.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.2.i6722, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.16.1 = phi float [ %_0.i2796.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.3.i6724, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.20.1 = phi float [ %_0.i2796.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.4.i6726, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.26.1 = phi float [ %_0.i2796.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.5.i6728, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.32.1 = phi float [ %_0.i2796.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.6.i6730, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.36.1 = phi float [ %_0.i2796.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.7.i6732, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.40.1 = phi float [ %_0.i2796.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.8.i6734, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.44.1 = phi float [ %_0.i2796.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.9.i6736, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.70.1 = phi float [ %_0.i2795, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.i6756, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.76.1 = phi float [ %_0.i2795.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.1.i6758, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.80.1 = phi float [ %_0.i2795.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.2.i6760, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.84.1 = phi float [ %_0.i2795.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.3.i6762, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.88.1 = phi float [ %_0.i2795.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.4.i6764, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.94.1 = phi float [ %_0.i2795.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.5.i6766, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.100.1 = phi float [ %_0.i2795.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.6.i6768, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.104.1 = phi float [ %_0.i2795.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.7.i6770, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.108.1 = phi float [ %_0.i2795.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.8.i6772, %bb17.i324.i ], !dbg !7687
  %segments.i126.i.sroa.112.1 = phi float [ %_0.i2795.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %_12.le.9.i6774, %bb17.i324.i ], !dbg !7687
  %iter.sroa.0.0.i33.i343.i11171 = phi i64 [ %561, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ 0, %bb17.i324.i ]
  %position.sroa.0.0.i32.i342.i11170 = phi i64 [ %_41.sroa.0.0.i40.i355.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %560, %bb17.i324.i ]
  %gain_far.i14.i109.i.sroa.6.011169 = phi i32 [ %_3.i4395, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %559, %bb17.i324.i ]
  %gain_far.i14.i109.i.sroa.0.011168 = phi i32 [ %_3.i4399, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %558, %bb17.i324.i ]
  %gain_near.i15.i110.i.sroa.6.011167 = phi i32 [ %_3.i4403, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %557, %bb17.i324.i ]
  %gain_near.i15.i110.i.sroa.0.011166 = phi i32 [ %_3.i4407, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %556, %bb17.i324.i ]
  %filter_far.i16.i111.i.sroa.14.011165 = phi float [ %_0.i4244, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %filter_far.i16.i111.i.sroa.14.0.copyload, %bb17.i324.i ]
  %filter_far.i16.i111.i.sroa.11.011164 = phi float [ %_0.i4248, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %filter_far.i16.i111.i.sroa.11.0.copyload, %bb17.i324.i ]
  %filter_far.i16.i111.i.sroa.7.011163 = phi float [ %_0.i4236, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %filter_far.i16.i111.i.sroa.7.0.copyload, %bb17.i324.i ]
  %filter_far.i16.i111.i.sroa.0.011162 = phi float [ %_0.i4240, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %filter_far.i16.i111.i.sroa.0.0.copyload, %bb17.i324.i ]
  %filter_near.i17.i112.i.sroa.14.011161 = phi float [ %_0.i4260, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %filter_near.i17.i112.i.sroa.14.0.copyload, %bb17.i324.i ]
  %filter_near.i17.i112.i.sroa.11.011160 = phi float [ %_0.i4264, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %filter_near.i17.i112.i.sroa.11.0.copyload, %bb17.i324.i ]
  %filter_near.i17.i112.i.sroa.7.011159 = phi float [ %_0.i4252, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %filter_near.i17.i112.i.sroa.7.0.copyload, %bb17.i324.i ]
  %filter_near.i17.i112.i.sroa.0.011158 = phi float [ %_0.i4256, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], [ %filter_near.i17.i112.i.sroa.0.0.copyload, %bb17.i324.i ]
  %_0.i2796 = fadd float %segments.i126.i.sroa.0.1, %_16.le.i6719, !dbg !7688
  %_0.i2795 = fadd float %segments.i126.i.sroa.70.1, %_16.le.i6757, !dbg !7693
  %_0.i2796.1 = fadd float %segments.i126.i.sroa.8.1, %_16.le.1.i6721, !dbg !7688
  %_0.i2795.1 = fadd float %segments.i126.i.sroa.76.1, %_16.le.1.i6759, !dbg !7693
  %_0.i2796.2 = fadd float %segments.i126.i.sroa.12.1, %_16.le.2.i6723, !dbg !7688
  %_0.i2795.2 = fadd float %segments.i126.i.sroa.80.1, %_16.le.2.i6761, !dbg !7693
  %_0.i2796.3 = fadd float %segments.i126.i.sroa.16.1, %_16.le.3.i6725, !dbg !7688
  %_0.i2795.3 = fadd float %segments.i126.i.sroa.84.1, %_16.le.3.i6763, !dbg !7693
  %_0.i2796.4 = fadd float %segments.i126.i.sroa.20.1, %_16.le.4.i6727, !dbg !7688
  %_0.i2795.4 = fadd float %segments.i126.i.sroa.88.1, %_16.le.4.i6765, !dbg !7693
  %_0.i2796.5 = fadd float %segments.i126.i.sroa.26.1, %_16.le.5.i6729, !dbg !7688
  %_0.i2795.5 = fadd float %segments.i126.i.sroa.94.1, %_16.le.5.i6767, !dbg !7693
  %_0.i2796.6 = fadd float %segments.i126.i.sroa.32.1, %_16.le.6.i6731, !dbg !7688
  %_0.i2795.6 = fadd float %segments.i126.i.sroa.100.1, %_16.le.6.i6769, !dbg !7693
  %_0.i2796.7 = fadd float %segments.i126.i.sroa.36.1, %_16.le.7.i6733, !dbg !7688
  %_0.i2795.7 = fadd float %segments.i126.i.sroa.104.1, %_16.le.7.i6771, !dbg !7693
  %_0.i2796.8 = fadd float %segments.i126.i.sroa.40.1, %_16.le.8.i6735, !dbg !7688
  %_0.i2795.8 = fadd float %segments.i126.i.sroa.108.1, %_16.le.8.i6773, !dbg !7693
  %_0.i2796.9 = fadd float %segments.i126.i.sroa.44.1, %_16.le.9.i6737, !dbg !7688
  %_0.i2795.9 = fadd float %segments.i126.i.sroa.112.1, %_16.le.9.i6775, !dbg !7693
  %561 = add nuw i64 %iter.sroa.0.0.i33.i343.i11171, 1, !dbg !7695
  %_42.i38.i353.i = add i64 %position.sroa.0.0.i32.i342.i11170, 1, !dbg !7701
  %_176.not.i39.i354.i = icmp ult i64 %_42.i38.i353.i, %ring_len.i128.i, !dbg !7703
  %562 = select i1 %_176.not.i39.i354.i, i64 0, i64 %ring_len.i128.i, !dbg !7703
  %_41.sroa.0.0.i40.i355.i = sub nuw i64 %_42.i38.i353.i, %562, !dbg !7703
  %_184.i42.i359.i = getelementptr inbounds nuw float, ptr %_75.i325.i, i64 %iter.sroa.0.0.i33.i343.i11171, !dbg !7706
  %_0.i3614 = load float, ptr %_184.i42.i359.i, align 4, !dbg !7716, !alias.scope !7718, !noalias !7721, !noundef !11
  %_192.i47.i362.i = getelementptr inbounds nuw float, ptr %_85.i328.i, i64 %iter.sroa.0.0.i33.i343.i11171, !dbg !7722
  %_0.i3609 = load float, ptr %_192.i47.i362.i, align 4, !dbg !7731, !alias.scope !7733, !noalias !7721, !noundef !11
  %_7.i54 = load float, ptr %_67.i.i177.i, align 4, !dbg !7736, !alias.scope !7739, !noalias !7742, !noundef !11
  %_8.i55 = load float, ptr %470, align 4, !dbg !7744, !alias.scope !7739, !noalias !7742, !noundef !11
  %_9.i56 = load float, ptr %471, align 4, !dbg !7745, !alias.scope !7739, !noalias !7742, !noundef !11
  %_0.i3419 = fsub float %_0.i3614, %filter_near.i17.i112.i.sroa.7.011159, !dbg !7746
  %_0.i3238 = fmul float %_0.i3419, %_8.i55, !dbg !7749
  %_4.i2867 = fmul float %filter_near.i17.i112.i.sroa.0.011158, %_7.i54, !dbg !7751
  %_0.i2868 = fadd float %_4.i2867, %_0.i3238, !dbg !7751
  %_0.i2702 = fadd float %filter_near.i17.i112.i.sroa.0.011158, %_0.i2868, !dbg !7753
  %_0.i3237 = fmul float %filter_near.i17.i112.i.sroa.0.011158, %_8.i55, !dbg !7755
  %_4.i2865 = fmul float %_0.i3419, %_9.i56, !dbg !7757
  %_0.i2866 = fadd float %_0.i3237, %_4.i2865, !dbg !7757
  %_0.i2701 = fadd float %filter_near.i17.i112.i.sroa.7.011159, %_0.i2866, !dbg !7759
  %_0.i2700 = fadd float %_0.i2868, %_0.i2868, !dbg !7761
  %_0.i2699 = fadd float %filter_near.i17.i112.i.sroa.0.011158, %_0.i2700, !dbg !7763
  %563 = tail call noundef float @llvm.fabs.f32(float %_0.i2699), !dbg !7765
  %564 = fcmp uge float %563, 0x3BC79CA100000000, !dbg !7768
  %_0.i4256 = select i1 %564, float %_0.i2699, float 0.000000e+00, !dbg !7770
  %_0.i2698 = fadd float %_0.i2866, %_0.i2866, !dbg !7771
  %_0.i2697 = fadd float %filter_near.i17.i112.i.sroa.7.011159, %_0.i2698, !dbg !7773
  %565 = tail call noundef float @llvm.fabs.f32(float %_0.i2697), !dbg !7775
  %566 = fcmp uge float %565, 0x3BC79CA100000000, !dbg !7778
  %_0.i4252 = select i1 %566, float %_0.i2697, float 0.000000e+00, !dbg !7780
  %_12.i59 = load float, ptr %472, align 4, !dbg !7781, !alias.scope !7739, !noalias !7742, !noundef !11
  %_4.i2933 = fmul float %_12.i59, %_0.i2702, !dbg !7782
  %_0.i2934 = fadd float %_0.i3614, %_4.i2933, !dbg !7782
  %_0.i3420 = fsub float %_0.i2701, %filter_near.i17.i112.i.sroa.14.011161, !dbg !7784
  %_0.i3240 = fmul float %_8.i55, %_0.i3420, !dbg !7787
  %_4.i2871 = fmul float %filter_near.i17.i112.i.sroa.11.011160, %_7.i54, !dbg !7789
  %_0.i2872 = fadd float %_4.i2871, %_0.i3240, !dbg !7789
  %_0.i3239 = fmul float %filter_near.i17.i112.i.sroa.11.011160, %_8.i55, !dbg !7791
  %_4.i2869 = fmul float %_9.i56, %_0.i3420, !dbg !7793
  %_0.i2870 = fadd float %_0.i3239, %_4.i2869, !dbg !7793
  %_0.i2707 = fadd float %filter_near.i17.i112.i.sroa.14.011161, %_0.i2870, !dbg !7795
  %_0.i2706 = fadd float %_0.i2872, %_0.i2872, !dbg !7797
  %_0.i2705 = fadd float %filter_near.i17.i112.i.sroa.11.011160, %_0.i2706, !dbg !7799
  %567 = tail call noundef float @llvm.fabs.f32(float %_0.i2705), !dbg !7801
  %568 = fcmp uge float %567, 0x3BC79CA100000000, !dbg !7804
  %_0.i4264 = select i1 %568, float %_0.i2705, float 0.000000e+00, !dbg !7806
  %_0.i2704 = fadd float %_0.i2870, %_0.i2870, !dbg !7807
  %_0.i2703 = fadd float %filter_near.i17.i112.i.sroa.14.011161, %_0.i2704, !dbg !7809
  %569 = tail call noundef float @llvm.fabs.f32(float %_0.i2703), !dbg !7811
  %570 = fcmp uge float %569, 0x3BC79CA100000000, !dbg !7814
  %_0.i4260 = select i1 %570, float %_0.i2703, float 0.000000e+00, !dbg !7816
  %_0.i3439 = fsub float %_0.i2934, %_0.i2707, !dbg !7817
  %_7.i41 = load float, ptr %_72.i.i179.i, align 4, !dbg !7819, !alias.scope !7822, !noalias !7825, !noundef !11
  %_8.i42 = load float, ptr %473, align 4, !dbg !7827, !alias.scope !7822, !noalias !7825, !noundef !11
  %_9.i43 = load float, ptr %474, align 4, !dbg !7828, !alias.scope !7822, !noalias !7825, !noundef !11
  %_0.i3417 = fsub float %_0.i3609, %filter_far.i16.i111.i.sroa.7.011163, !dbg !7829
  %_0.i3234 = fmul float %_0.i3417, %_8.i42, !dbg !7832
  %_4.i2859 = fmul float %filter_far.i16.i111.i.sroa.0.011162, %_7.i41, !dbg !7834
  %_0.i2860 = fadd float %_4.i2859, %_0.i3234, !dbg !7834
  %_0.i2690 = fadd float %filter_far.i16.i111.i.sroa.0.011162, %_0.i2860, !dbg !7836
  %_0.i3233 = fmul float %filter_far.i16.i111.i.sroa.0.011162, %_8.i42, !dbg !7838
  %_4.i2857 = fmul float %_0.i3417, %_9.i43, !dbg !7840
  %_0.i2858 = fadd float %_0.i3233, %_4.i2857, !dbg !7840
  %_0.i2689 = fadd float %filter_far.i16.i111.i.sroa.7.011163, %_0.i2858, !dbg !7842
  %_0.i2688 = fadd float %_0.i2860, %_0.i2860, !dbg !7844
  %_0.i2687 = fadd float %filter_far.i16.i111.i.sroa.0.011162, %_0.i2688, !dbg !7846
  %571 = tail call noundef float @llvm.fabs.f32(float %_0.i2687), !dbg !7848
  %572 = fcmp uge float %571, 0x3BC79CA100000000, !dbg !7851
  %_0.i4240 = select i1 %572, float %_0.i2687, float 0.000000e+00, !dbg !7853
  %_0.i2686 = fadd float %_0.i2858, %_0.i2858, !dbg !7854
  %_0.i2685 = fadd float %filter_far.i16.i111.i.sroa.7.011163, %_0.i2686, !dbg !7856
  %573 = tail call noundef float @llvm.fabs.f32(float %_0.i2685), !dbg !7858
  %574 = fcmp uge float %573, 0x3BC79CA100000000, !dbg !7861
  %_0.i4236 = select i1 %574, float %_0.i2685, float 0.000000e+00, !dbg !7863
  %_12.i46 = load float, ptr %475, align 4, !dbg !7864, !alias.scope !7822, !noalias !7825, !noundef !11
  %_4.i2935 = fmul float %_12.i46, %_0.i2690, !dbg !7865
  %_0.i2936 = fadd float %_0.i3609, %_4.i2935, !dbg !7865
  %_0.i3418 = fsub float %_0.i2689, %filter_far.i16.i111.i.sroa.14.011165, !dbg !7867
  %_0.i3236 = fmul float %_8.i42, %_0.i3418, !dbg !7870
  %_4.i2863 = fmul float %filter_far.i16.i111.i.sroa.11.011164, %_7.i41, !dbg !7872
  %_0.i2864 = fadd float %_4.i2863, %_0.i3236, !dbg !7872
  %_0.i3235 = fmul float %filter_far.i16.i111.i.sroa.11.011164, %_8.i42, !dbg !7874
  %_4.i2861 = fmul float %_9.i43, %_0.i3418, !dbg !7876
  %_0.i2862 = fadd float %_0.i3235, %_4.i2861, !dbg !7876
  %_0.i2695 = fadd float %filter_far.i16.i111.i.sroa.14.011165, %_0.i2862, !dbg !7878
  %_0.i2694 = fadd float %_0.i2864, %_0.i2864, !dbg !7880
  %_0.i2693 = fadd float %filter_far.i16.i111.i.sroa.11.011164, %_0.i2694, !dbg !7882
  %575 = tail call noundef float @llvm.fabs.f32(float %_0.i2693), !dbg !7884
  %576 = fcmp uge float %575, 0x3BC79CA100000000, !dbg !7887
  %_0.i4248 = select i1 %576, float %_0.i2693, float 0.000000e+00, !dbg !7889
  %_0.i2692 = fadd float %_0.i2862, %_0.i2862, !dbg !7890
  %_0.i2691 = fadd float %filter_far.i16.i111.i.sroa.14.011165, %_0.i2692, !dbg !7892
  %577 = tail call noundef float @llvm.fabs.f32(float %_0.i2691), !dbg !7894
  %578 = fcmp uge float %577, 0x3BC79CA100000000, !dbg !7897
  %_0.i4244 = select i1 %578, float %_0.i2691, float 0.000000e+00, !dbg !7899
  %_0.i3440 = fsub float %_0.i2936, %_0.i2695, !dbg !7900
  %_311.1.i54.i369.i = load i64, ptr %476, align 8, !dbg !7902, !noalias !7721, !noundef !11
  %_234.i55.i370.i = icmp ugt i64 %position.sroa.0.0.i32.i342.i11170, %_311.1.i54.i369.i, !dbg !7904
  br i1 %_234.i55.i370.i, label %bb78.i153.i500.i, label %bb79.i56.i371.i, !dbg !7904, !prof !1664

bb79.i56.i371.i:                                  ; preds = %bb56.i37.i348.i
  %_311.0.i57.i372.i = load ptr, ptr %425, align 8, !dbg !7902, !noalias !7721, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7910), !dbg !7913
  %_4.not.i4062 = icmp eq i64 %_311.1.i54.i369.i, %position.sroa.0.0.i32.i342.i11170, !dbg !7914
  br i1 %_4.not.i4062, label %panic.i4064, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4065, !dbg !7914

panic.i4064:                                      ; preds = %bb79.i56.i371.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !7914, !noalias !7916
  unreachable, !dbg !7914

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4065: ; preds = %bb79.i56.i371.i
  %_241.i60.i375.i = getelementptr inbounds nuw float, ptr %_311.0.i57.i372.i, i64 %position.sroa.0.0.i32.i342.i11170, !dbg !7917
  store float %_0.i2707, ptr %_241.i60.i375.i, align 4, !dbg !7914, !alias.scope !7910, !noalias !7721
  %_312.1.i61.i376.i = load i64, ptr %478, align 8, !dbg !7923, !noalias !7721, !noundef !11
  %_242.i62.i377.i = icmp ugt i64 %position.sroa.0.0.i32.i342.i11170, %_312.1.i61.i376.i, !dbg !7924
  br i1 %_242.i62.i377.i, label %bb80.i152.i499.i, label %bb81.i63.i378.i, !dbg !7924, !prof !1664

bb78.i153.i500.i:                                 ; preds = %bb56.i37.i348.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i342.i11170, i64 noundef %_311.1.i54.i369.i, i64 noundef %_311.1.i54.i369.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7515f6adc8a5521b72f04a3abb372e72) #26, !dbg !7928, !noalias !7721
  unreachable, !dbg !7928

bb81.i63.i378.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4065
  %_312.0.i64.i379.i = load ptr, ptr %477, align 8, !dbg !7923, !noalias !7721, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7929), !dbg !7932
  %_4.not.i4058 = icmp eq i64 %_312.1.i61.i376.i, %position.sroa.0.0.i32.i342.i11170, !dbg !7933
  br i1 %_4.not.i4058, label %panic.i4060, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4061, !dbg !7933

panic.i4060:                                      ; preds = %bb81.i63.i378.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !7933, !noalias !7935
  unreachable, !dbg !7933

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4061: ; preds = %bb81.i63.i378.i
  %_249.i66.i381.i = getelementptr inbounds nuw float, ptr %_312.0.i64.i379.i, i64 %position.sroa.0.0.i32.i342.i11170, !dbg !7936
  store float %_0.i3439, ptr %_249.i66.i381.i, align 4, !dbg !7933, !alias.scope !7929, !noalias !7721
  %_313.1.i67.i382.i = load i64, ptr %479, align 8, !dbg !7941, !noalias !7721, !noundef !11
  %_250.i68.i383.i = icmp ugt i64 %position.sroa.0.0.i32.i342.i11170, %_313.1.i67.i382.i, !dbg !7942
  br i1 %_250.i68.i383.i, label %bb82.i151.i498.i, label %bb83.i69.i384.i, !dbg !7942, !prof !1664

bb80.i152.i499.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4065
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i342.i11170, i64 noundef %_312.1.i61.i376.i, i64 noundef %_312.1.i61.i376.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8c2aade3368450b16e7e4997ad3fca81) #26, !dbg !7946, !noalias !7721
  unreachable, !dbg !7946

bb83.i69.i384.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4061
  %_313.0.i70.i385.i = load ptr, ptr %_18.i136.i, align 8, !dbg !7941, !noalias !7721, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7947), !dbg !7950
  %_4.not.i4054 = icmp eq i64 %_313.1.i67.i382.i, %position.sroa.0.0.i32.i342.i11170, !dbg !7951
  br i1 %_4.not.i4054, label %panic.i4056, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4057, !dbg !7951

panic.i4056:                                      ; preds = %bb83.i69.i384.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !7951, !noalias !7953
  unreachable, !dbg !7951

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4057: ; preds = %bb83.i69.i384.i
  %_257.i72.i387.i = getelementptr inbounds nuw float, ptr %_313.0.i70.i385.i, i64 %position.sroa.0.0.i32.i342.i11170, !dbg !7954
  store float %_0.i2695, ptr %_257.i72.i387.i, align 4, !dbg !7951, !alias.scope !7947, !noalias !7721
  %_314.1.i73.i388.i = load i64, ptr %481, align 8, !dbg !7959, !noalias !7721, !noundef !11
  %_258.i74.i389.i = icmp ugt i64 %position.sroa.0.0.i32.i342.i11170, %_314.1.i73.i388.i, !dbg !7960
  br i1 %_258.i74.i389.i, label %bb84.i150.i497.i, label %bb85.i75.i390.i, !dbg !7960, !prof !1664

bb82.i151.i498.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4061
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i342.i11170, i64 noundef %_313.1.i67.i382.i, i64 noundef %_313.1.i67.i382.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a4c3da5a99763e9452b49d42852d67e3) #26, !dbg !7964, !noalias !7721
  unreachable, !dbg !7964

bb85.i75.i390.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4057
  %_314.0.i76.i391.i = load ptr, ptr %480, align 8, !dbg !7959, !noalias !7721, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7965), !dbg !7968
  %_4.not.i4050 = icmp eq i64 %_314.1.i73.i388.i, %position.sroa.0.0.i32.i342.i11170, !dbg !7969
  br i1 %_4.not.i4050, label %panic.i4052, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053, !dbg !7969

panic.i4052:                                      ; preds = %bb85.i75.i390.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !7969, !noalias !7971
  unreachable, !dbg !7969

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053: ; preds = %bb85.i75.i390.i
  %_265.i78.i393.i = getelementptr inbounds nuw float, ptr %_314.0.i76.i391.i, i64 %position.sroa.0.0.i32.i342.i11170, !dbg !7972
  store float %_0.i3440, ptr %_265.i78.i393.i, align 4, !dbg !7969, !alias.scope !7965, !noalias !7721
  %_315.0.i79.i394.i = load ptr, ptr %425, align 8, !dbg !7977, !noalias !7721, !nonnull !11, !noundef !11
  %_315.1.i80.i395.i = load i64, ptr %476, align 8, !dbg !7977, !noalias !7721, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7978), !dbg !7981
  %_8.i176.i396.i = load i64, ptr %_24.i.i156.i, align 8, !dbg !7982, !alias.scope !7978, !noalias !7984, !noundef !11
  %_7.i177.i397.i = add i64 %_8.i176.i396.i, %position.sroa.0.0.i32.i342.i11170, !dbg !7986
  %_27.not.i178.i398.i = icmp ult i64 %_7.i177.i397.i, %ring_len.i128.i, !dbg !7987
  %579 = select i1 %_27.not.i178.i398.i, i64 0, i64 %ring_len.i128.i, !dbg !7987
  %row.sroa.0.0.i179.i399.i = sub nuw i64 %_7.i177.i397.i, %579, !dbg !7987
  %_28.i180.i400.i = icmp ugt i64 %row.sroa.0.0.i179.i399.i, %_315.1.i80.i395.i, !dbg !7989
  br i1 %_28.i180.i400.i, label %bb14.i183.i496.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i401.i, !dbg !7989, !prof !1664

bb14.i183.i496.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i179.i399.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i80.i395.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i80.i395.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !7992, !noalias !7993
  unreachable, !dbg !7992

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i401.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4053
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7994), !dbg !7997
  %_3.not.i3602 = icmp eq i64 %_315.1.i80.i395.i, %row.sroa.0.0.i179.i399.i, !dbg !7998
  br i1 %_3.not.i3602, label %panic.i3605, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3606, !dbg !7998

panic.i3605:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i401.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !7998, !noalias !8000
  unreachable, !dbg !7998

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3606: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i401.i
  %_35.i182.i403.i = getelementptr inbounds nuw float, ptr %_315.0.i79.i394.i, i64 %row.sroa.0.0.i179.i399.i, !dbg !8001
  %_0.i3604 = load float, ptr %_35.i182.i403.i, align 4, !dbg !7998, !alias.scope !7994, !noalias !8003, !noundef !11
  %_316.0.i82.i404.i = load ptr, ptr %477, align 8, !dbg !8004, !noalias !7721, !nonnull !11, !noundef !11
  %_316.1.i83.i405.i = load i64, ptr %478, align 8, !dbg !8004, !noalias !7721, !noundef !11
  %_28.i171.i410.i = icmp ugt i64 %row.sroa.0.0.i179.i399.i, %_316.1.i83.i405.i, !dbg !8006
  br i1 %_28.i171.i410.i, label %bb14.i174.i495.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i411.i, !dbg !8006, !prof !1664

bb14.i174.i495.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3606
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i179.i399.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i83.i405.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i83.i405.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !8010, !noalias !8011
  unreachable, !dbg !8010

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i411.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3606
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8015), !dbg !8018
  %_3.not.i3597 = icmp eq i64 %_316.1.i83.i405.i, %row.sroa.0.0.i179.i399.i, !dbg !8019
  br i1 %_3.not.i3597, label %panic.i3600, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3601, !dbg !8019

panic.i3600:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i411.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8019, !noalias !8021
  unreachable, !dbg !8019

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3601: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i411.i
  %_35.i173.i413.i = getelementptr inbounds nuw float, ptr %_316.0.i82.i404.i, i64 %row.sroa.0.0.i179.i399.i, !dbg !8022
  %_0.i3599 = load float, ptr %_35.i173.i413.i, align 4, !dbg !8019, !alias.scope !8015, !noalias !8024, !noundef !11
  %_317.0.i85.i414.i = load ptr, ptr %_18.i136.i, align 8, !dbg !8025, !noalias !7721, !nonnull !11, !noundef !11
  %_317.1.i86.i415.i = load i64, ptr %479, align 8, !dbg !8025, !noalias !7721, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8027), !dbg !8030
  %_8.i158.i416.i = load i64, ptr %_26.i.i158.i, align 8, !dbg !8031, !alias.scope !8027, !noalias !8033, !noundef !11
  %_7.i159.i417.i = add i64 %_8.i158.i416.i, %position.sroa.0.0.i32.i342.i11170, !dbg !8035
  %_27.not.i160.i418.i = icmp ult i64 %_7.i159.i417.i, %ring_len.i128.i, !dbg !8036
  %580 = select i1 %_27.not.i160.i418.i, i64 0, i64 %ring_len.i128.i, !dbg !8036
  %row.sroa.0.0.i161.i419.i = sub nuw i64 %_7.i159.i417.i, %580, !dbg !8036
  %_28.i162.i420.i = icmp ugt i64 %row.sroa.0.0.i161.i419.i, %_317.1.i86.i415.i, !dbg !8038
  br i1 %_28.i162.i420.i, label %bb14.i165.i494.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i421.i, !dbg !8038, !prof !1664

bb14.i165.i494.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3601
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i161.i419.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i86.i415.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i86.i415.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !8041, !noalias !8042
  unreachable, !dbg !8041

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i421.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3601
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8043), !dbg !8046
  %_3.not.i3592 = icmp eq i64 %_317.1.i86.i415.i, %row.sroa.0.0.i161.i419.i, !dbg !8047
  br i1 %_3.not.i3592, label %panic.i3595, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3596, !dbg !8047

panic.i3595:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i421.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8047, !noalias !8049
  unreachable, !dbg !8047

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3596: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i421.i
  %_35.i164.i423.i = getelementptr inbounds nuw float, ptr %_317.0.i85.i414.i, i64 %row.sroa.0.0.i161.i419.i, !dbg !8050
  %_0.i3594 = load float, ptr %_35.i164.i423.i, align 4, !dbg !8047, !alias.scope !8043, !noalias !8052, !noundef !11
  %_318.0.i88.i424.i = load ptr, ptr %480, align 8, !dbg !8053, !noalias !7721, !nonnull !11, !noundef !11
  %_318.1.i89.i425.i = load i64, ptr %481, align 8, !dbg !8053, !noalias !7721, !noundef !11
  %_28.i.i430.i = icmp ugt i64 %row.sroa.0.0.i161.i419.i, %_318.1.i89.i425.i, !dbg !8055
  br i1 %_28.i.i430.i, label %bb14.i.i493.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i431.i, !dbg !8055, !prof !1664

bb14.i.i493.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3596
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i161.i419.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i89.i425.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i89.i425.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !8059, !noalias !8060
  unreachable, !dbg !8059

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i431.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3596
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8064), !dbg !8067
  %_3.not.i3587 = icmp eq i64 %_318.1.i89.i425.i, %row.sroa.0.0.i161.i419.i, !dbg !8068
  br i1 %_3.not.i3587, label %panic.i3590, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3591, !dbg !8068

panic.i3590:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i431.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8068, !noalias !8070
  unreachable, !dbg !8068

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3591: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i431.i
  %_35.i157.i433.i = getelementptr inbounds nuw float, ptr %_318.0.i88.i424.i, i64 %row.sroa.0.0.i161.i419.i, !dbg !8071
  %_0.i3589 = load float, ptr %_35.i157.i433.i, align 4, !dbg !8068, !alias.scope !8064, !noalias !8073, !noundef !11
  %581 = tail call noundef float @llvm.fabs.f32(float %_0.i3604), !dbg !8074
  %582 = tail call noundef float @llvm.fabs.f32(float %_0.i3594), !dbg !8078
  %_3.i.i5332.inv = fcmp ogt float %581, %582, !dbg !8080
  %_4.i.i5339.v = select i1 %_3.i.i5332.inv, float %581, float %582, !dbg !8080
  %_4.i.i5339 = bitcast float %_4.i.i5339.v to i32, !dbg !8080
  %_3.i.i5630 = fcmp ule float %_4.i.i5339.v, 0x3E45798EE0000000, !dbg !8083
  %_4.i.i5636 = select i1 %_3.i.i5630, i32 841731191, i32 %_4.i.i5339, !dbg !8089
  %_0.i.i5637 = bitcast i32 %_4.i.i5636 to float, !dbg !8091
  %_3.i.i5026 = fcmp ule float %_0.i.i5637, 0x3810000000000000, !dbg !8093
  %_4.i.i5032 = select i1 %_3.i.i5026, i32 8388608, i32 %_4.i.i5636, !dbg !8098
  %_5.i3901 = and i32 %_4.i.i5032, 8388607, !dbg !8100
  %_4.i3902 = or disjoint i32 %_5.i3901, 1065353216, !dbg !8100
  %significand.i3903 = bitcast i32 %_4.i3902 to float, !dbg !8102
  %_0.i3349 = fadd float %significand.i3903, -1.000000e+00, !dbg !8104
  %_0.i3018 = fmul float %_0.i3349, 0xBF9B17A960000000, !dbg !8106
  %_0.i2538 = fadd float %_0.i3018, 0x3FBF9A8440000000, !dbg !8108
  %_0.i3018.1 = fmul float %_0.i3349, %_0.i2538, !dbg !8106
  %_0.i2538.1 = fadd float %_0.i3018.1, 0xBFD1E3F400000000, !dbg !8108
  %_0.i3018.2 = fmul float %_0.i3349, %_0.i2538.1, !dbg !8106
  %_0.i2538.2 = fadd float %_0.i3018.2, 0x3FDD544F20000000, !dbg !8108
  %_0.i3018.3 = fmul float %_0.i3349, %_0.i2538.2, !dbg !8106
  %_0.i2538.3 = fadd float %_0.i3018.3, 0xBFE6FC2A60000000, !dbg !8108
  %_0.i3018.4 = fmul float %_0.i3349, %_0.i2538.3, !dbg !8106
  %_0.i2538.4 = fadd float %_0.i3018.4, 0x3FF714B2A0000000, !dbg !8108
  %_9.i3904 = lshr i32 %_4.i.i5032, 23, !dbg !8110
  %_8.i3905 = or disjoint i32 %_9.i3904, 1258291200, !dbg !8110
  %_7.i3906 = bitcast i32 %_8.i3905 to float, !dbg !8111
  %exponent.i3907 = fadd float %_7.i3906, 0xC160000FE0000000, !dbg !8113
  %_0.i3017 = fmul float %_0.i3349, %_0.i2538.4, !dbg !8114
  %_0.i2537 = fadd float %exponent.i3907, %_0.i3017, !dbg !8116
  %_0.i3300 = fmul float %_0.i2537, 0x4018151820000000, !dbg !8118
  %_3.i.i5622.inv = fcmp ogt float %_0.i3300, -1.600000e+02, !dbg !8120
  %_0.i.i5629 = select i1 %_3.i.i5622.inv, float %_0.i3300, float -1.600000e+02, !dbg !8120
  %_3.i.i6300.inv = fcmp olt float %_0.i.i5629, 2.400000e+01, !dbg !8123
  %_0.i.i6307 = select i1 %_3.i.i6300.inv, float %_0.i.i5629, float 2.400000e+01, !dbg !8123
  %_0.i3397 = fsub float %_0.i.i6307, %_0.i2796, !dbg !8126
  %_3.i2249 = fcmp ule float %_0.i3397, 3.000000e+00, !dbg !8129
  %_0.i2621 = fadd float %_0.i3397, 3.000000e+00, !dbg !8131
  %_0.i3163 = fmul float %_0.i2621, %_0.i2621, !dbg !8133
  %_0.i3162 = fmul float %_0.i3163, 0x3FB5555560000000, !dbg !8135
  %_4.i4624.v.v = select i1 %_3.i2249, float %_0.i3162, float %_0.i3397, !dbg !8137
  %_4.i4624.v = fmul float %coefficients.i123.i.sroa.0.0.copyload, %_4.i4624.v.v, !dbg !8137
  %_4.i4624 = bitcast float %_4.i4624.v to i32, !dbg !8137
  %583 = fcmp ugt float %_0.i3397, -3.000000e+00, !dbg !8139
  %_7.i4616 = select i1 %583, i32 %_4.i4624, i32 0, !dbg !8141
  %_0.i4618 = bitcast i32 %_7.i4616 to float, !dbg !8142
  %_3.i.i5614 = fcmp ule float %_0.i4618, -1.000000e+02, !dbg !8144
  %584 = bitcast i32 %_7.i4616 to float, !dbg !8147
  %_0.i.i5621 = select i1 %_3.i.i5614, float -1.000000e+02, float %584, !dbg !8150
  %_3.i.i6292 = fcmp olt float %_0.i.i5621, 0.000000e+00, !dbg !8151
  %_0.i.i6299 = select i1 %_3.i.i6292, float %_0.i.i5621, float 0.000000e+00, !dbg !8154
  %585 = bitcast i32 %gain_near.i15.i110.i.sroa.0.011166 to float, !dbg !8156
  %_3.i2463 = fcmp uge float %_0.i.i6299, %585, !dbg !8157
  %_4.i4850.v = select i1 %_3.i2463, float %coefficients.i123.i.sroa.7.0.copyload, float %coefficients.i123.i.sroa.5.0.copyload, !dbg !8160
  %_0.i3456 = fsub float %585, %_0.i.i6299, !dbg !8162
  %_4.i2967 = fmul float %_0.i3456, %_4.i4850.v, !dbg !8164
  %_0.i2968 = fadd float %_0.i.i6299, %_4.i2967, !dbg !8164
  %586 = tail call noundef float @llvm.fabs.f32(float %_0.i2968), !dbg !8166
  %_4.i4405 = bitcast float %_0.i2968 to i32, !dbg !8169
  %587 = fcmp uge float %586, 0x3BC79CA100000000, !dbg !8172
  %_3.i4407 = select i1 %587, i32 %_4.i4405, i32 0, !dbg !8173
  %_0.i4408 = bitcast i32 %_3.i4407 to float, !dbg !8174
  %_0.i2802 = fadd float %_0.i2796.4, %_0.i4408, !dbg !8176
  %_0.i3299 = fmul float %_0.i2802, 0x3FC542A5A0000000, !dbg !8178
  %_3.i.i5218.inv = fcmp ogt float %_0.i3299, -1.260000e+02, !dbg !8181
  %_0.i.i5225 = select i1 %_3.i.i5218.inv, float %_0.i3299, float -1.260000e+02, !dbg !8181
  %_3.i.i6020.inv = fcmp olt float %_0.i.i5225, 1.270000e+02, !dbg !8185
  %_0.i.i6027 = select i1 %_3.i.i6020.inv, float %_0.i.i5225, float 1.270000e+02, !dbg !8185
  %588 = tail call noundef float @llvm.floor.f32(float %_0.i.i6027), !dbg !8188
  %_0.i3373 = fsub float %_0.i.i6027, %588, !dbg !8192
  %589 = tail call noundef float @llvm.fabs.f32(float %_0.i3599), !dbg !8194
  %590 = tail call noundef float @llvm.fabs.f32(float %_0.i3589), !dbg !8197
  %_3.i.i5341.inv = fcmp ogt float %589, %590, !dbg !8199
  %_4.i.i5348.v = select i1 %_3.i.i5341.inv, float %589, float %590, !dbg !8199
  %_4.i.i5348 = bitcast float %_4.i.i5348.v to i32, !dbg !8199
  %_3.i.i5606 = fcmp ule float %_4.i.i5348.v, 0x3E45798EE0000000, !dbg !8202
  %_4.i.i5612 = select i1 %_3.i.i5606, i32 841731191, i32 %_4.i.i5348, !dbg !8207
  %_0.i.i5613 = bitcast i32 %_4.i.i5612 to float, !dbg !8209
  %_3.i.i5034 = fcmp ule float %_0.i.i5613, 0x3810000000000000, !dbg !8211
  %_4.i.i5040 = select i1 %_3.i.i5034, i32 8388608, i32 %_4.i.i5612, !dbg !8216
  %_5.i3909 = and i32 %_4.i.i5040, 8388607, !dbg !8218
  %_4.i3910 = or disjoint i32 %_5.i3909, 1065353216, !dbg !8218
  %significand.i3911 = bitcast i32 %_4.i3910 to float, !dbg !8220
  %_0.i3350 = fadd float %significand.i3911, -1.000000e+00, !dbg !8222
  %_0.i3020 = fmul float %_0.i3350, 0xBF9B17A960000000, !dbg !8224
  %_0.i2540 = fadd float %_0.i3020, 0x3FBF9A8440000000, !dbg !8226
  %_0.i3020.1 = fmul float %_0.i3350, %_0.i2540, !dbg !8224
  %_0.i2540.1 = fadd float %_0.i3020.1, 0xBFD1E3F400000000, !dbg !8226
  %_0.i3020.2 = fmul float %_0.i3350, %_0.i2540.1, !dbg !8224
  %_0.i2540.2 = fadd float %_0.i3020.2, 0x3FDD544F20000000, !dbg !8226
  %_0.i3020.3 = fmul float %_0.i3350, %_0.i2540.2, !dbg !8224
  %_0.i2540.3 = fadd float %_0.i3020.3, 0xBFE6FC2A60000000, !dbg !8226
  %_0.i3020.4 = fmul float %_0.i3350, %_0.i2540.3, !dbg !8224
  %_0.i2540.4 = fadd float %_0.i3020.4, 0x3FF714B2A0000000, !dbg !8226
  %_9.i3912 = lshr i32 %_4.i.i5040, 23, !dbg !8228
  %_8.i3913 = or disjoint i32 %_9.i3912, 1258291200, !dbg !8228
  %_7.i3914 = bitcast i32 %_8.i3913 to float, !dbg !8229
  %exponent.i3915 = fadd float %_7.i3914, 0xC160000FE0000000, !dbg !8231
  %_0.i3019 = fmul float %_0.i3350, %_0.i2540.4, !dbg !8232
  %_0.i2539 = fadd float %exponent.i3915, %_0.i3019, !dbg !8234
  %_0.i3298 = fmul float %_0.i2539, 0x4018151820000000, !dbg !8236
  %_3.i.i5598.inv = fcmp ogt float %_0.i3298, -1.600000e+02, !dbg !8238
  %_0.i.i5605 = select i1 %_3.i.i5598.inv, float %_0.i3298, float -1.600000e+02, !dbg !8238
  %_3.i.i6284.inv = fcmp olt float %_0.i.i5605, 2.400000e+01, !dbg !8241
  %_0.i.i6291 = select i1 %_3.i.i6284.inv, float %_0.i.i5605, float 2.400000e+01, !dbg !8241
  %_0.i3398 = fsub float %_0.i.i6291, %_0.i2796.5, !dbg !8244
  %_3.i2251 = fcmp ule float %_0.i3398, 3.000000e+00, !dbg !8247
  %_0.i2622 = fadd float %_0.i3398, 3.000000e+00, !dbg !8249
  %_0.i3167 = fmul float %_0.i2622, %_0.i2622, !dbg !8251
  %_0.i3166 = fmul float %_0.i3167, 0x3FB5555560000000, !dbg !8253
  %_4.i4637.v.v = select i1 %_3.i2251, float %_0.i3166, float %_0.i3398, !dbg !8255
  %_4.i4637.v = fmul float %coefficients.i123.i.sroa.9.0.copyload, %_4.i4637.v.v, !dbg !8255
  %_4.i4637 = bitcast float %_4.i4637.v to i32, !dbg !8255
  %591 = fcmp ugt float %_0.i3398, -3.000000e+00, !dbg !8257
  %_7.i4629 = select i1 %591, i32 %_4.i4637, i32 0, !dbg !8259
  %_0.i4631 = bitcast i32 %_7.i4629 to float, !dbg !8260
  %_3.i.i5590 = fcmp ule float %_0.i4631, -1.000000e+02, !dbg !8262
  %592 = bitcast i32 %_7.i4629 to float, !dbg !8265
  %_0.i.i5597 = select i1 %_3.i.i5590, float -1.000000e+02, float %592, !dbg !8268
  %_3.i.i6276 = fcmp olt float %_0.i.i5597, 0.000000e+00, !dbg !8269
  %_0.i.i6283 = select i1 %_3.i.i6276, float %_0.i.i5597, float 0.000000e+00, !dbg !8272
  %593 = bitcast i32 %gain_near.i15.i110.i.sroa.6.011167 to float, !dbg !8274
  %_3.i2459 = fcmp uge float %_0.i.i6283, %593, !dbg !8275
  %_4.i4843.v = select i1 %_3.i2459, float %coefficients.i123.i.sroa.13.0.copyload, float %coefficients.i123.i.sroa.11.0.copyload, !dbg !8278
  %_0.i3455 = fsub float %593, %_0.i.i6283, !dbg !8280
  %_4.i2965 = fmul float %_0.i3455, %_4.i4843.v, !dbg !8282
  %_0.i2966 = fadd float %_0.i.i6283, %_4.i2965, !dbg !8282
  %594 = tail call noundef float @llvm.fabs.f32(float %_0.i2966), !dbg !8284
  %_4.i4401 = bitcast float %_0.i2966 to i32, !dbg !8287
  %595 = fcmp uge float %594, 0x3BC79CA100000000, !dbg !8290
  %_3.i4403 = select i1 %595, i32 %_4.i4401, i32 0, !dbg !8291
  %_0.i4404 = bitcast i32 %_3.i4403 to float, !dbg !8292
  %_0.i2801 = fadd float %_0.i2796.9, %_0.i4404, !dbg !8294
  %_0.i3297 = fmul float %_0.i2801, 0x3FC542A5A0000000, !dbg !8296
  %_3.i.i5226.inv = fcmp ogt float %_0.i3297, -1.260000e+02, !dbg !8299
  %_0.i.i5233 = select i1 %_3.i.i5226.inv, float %_0.i3297, float -1.260000e+02, !dbg !8299
  %_3.i.i6028.inv = fcmp olt float %_0.i.i5233, 1.270000e+02, !dbg !8303
  %_0.i.i6035 = select i1 %_3.i.i6028.inv, float %_0.i.i5233, float 1.270000e+02, !dbg !8303
  %596 = tail call noundef float @llvm.floor.f32(float %_0.i.i6035), !dbg !8306
  %_0.i3374 = fsub float %_0.i.i6035, %596, !dbg !8310
  %_0.i3399 = fsub float %_0.i.i6307, %_0.i2795, !dbg !8312
  %_3.i2253 = fcmp ule float %_0.i3399, 3.000000e+00, !dbg !8317
  %_0.i2623 = fadd float %_0.i3399, 3.000000e+00, !dbg !8319
  %_0.i3171 = fmul float %_0.i2623, %_0.i2623, !dbg !8321
  %_0.i3170 = fmul float %_0.i3171, 0x3FB5555560000000, !dbg !8323
  %_4.i4650.v.v = select i1 %_3.i2253, float %_0.i3170, float %_0.i3399, !dbg !8325
  %_4.i4650.v = fmul float %coefficients.i123.i.sroa.15.24.copyload, %_4.i4650.v.v, !dbg !8325
  %_4.i4650 = bitcast float %_4.i4650.v to i32, !dbg !8325
  %597 = fcmp ugt float %_0.i3399, -3.000000e+00, !dbg !8327
  %_7.i4642 = select i1 %597, i32 %_4.i4650, i32 0, !dbg !8329
  %_0.i4644 = bitcast i32 %_7.i4642 to float, !dbg !8330
  %_3.i.i5566 = fcmp ule float %_0.i4644, -1.000000e+02, !dbg !8332
  %598 = bitcast i32 %_7.i4642 to float, !dbg !8335
  %_0.i.i5573 = select i1 %_3.i.i5566, float -1.000000e+02, float %598, !dbg !8338
  %_3.i.i6260 = fcmp olt float %_0.i.i5573, 0.000000e+00, !dbg !8339
  %_0.i.i6267 = select i1 %_3.i.i6260, float %_0.i.i5573, float 0.000000e+00, !dbg !8342
  %599 = bitcast i32 %gain_far.i14.i109.i.sroa.0.011168 to float, !dbg !8344
  %_3.i2455 = fcmp uge float %_0.i.i6267, %599, !dbg !8345
  %_4.i4836.v = select i1 %_3.i2455, float %coefficients.i123.i.sroa.20.24.copyload, float %coefficients.i123.i.sroa.18.24.copyload, !dbg !8348
  %_0.i3454 = fsub float %599, %_0.i.i6267, !dbg !8350
  %_4.i2963 = fmul float %_0.i3454, %_4.i4836.v, !dbg !8352
  %_0.i2964 = fadd float %_0.i.i6267, %_4.i2963, !dbg !8352
  %600 = tail call noundef float @llvm.fabs.f32(float %_0.i2964), !dbg !8354
  %_4.i4397 = bitcast float %_0.i2964 to i32, !dbg !8357
  %601 = fcmp uge float %600, 0x3BC79CA100000000, !dbg !8360
  %_3.i4399 = select i1 %601, i32 %_4.i4397, i32 0, !dbg !8361
  %_0.i4400 = bitcast i32 %_3.i4399 to float, !dbg !8362
  %_0.i2800 = fadd float %_0.i2795.4, %_0.i4400, !dbg !8364
  %_0.i3295 = fmul float %_0.i2800, 0x3FC542A5A0000000, !dbg !8366
  %_3.i.i5234.inv = fcmp ogt float %_0.i3295, -1.260000e+02, !dbg !8369
  %_0.i.i5241 = select i1 %_3.i.i5234.inv, float %_0.i3295, float -1.260000e+02, !dbg !8369
  %_3.i.i6036.inv = fcmp olt float %_0.i.i5241, 1.270000e+02, !dbg !8373
  %_0.i.i6043 = select i1 %_3.i.i6036.inv, float %_0.i.i5241, float 1.270000e+02, !dbg !8373
  %602 = tail call noundef float @llvm.floor.f32(float %_0.i.i6043), !dbg !8376
  %_0.i3375 = fsub float %_0.i.i6043, %602, !dbg !8380
  %_0.i3400 = fsub float %_0.i.i6291, %_0.i2795.5, !dbg !8382
  %_3.i2255 = fcmp ule float %_0.i3400, 3.000000e+00, !dbg !8387
  %_0.i2624 = fadd float %_0.i3400, 3.000000e+00, !dbg !8389
  %_0.i3175 = fmul float %_0.i2624, %_0.i2624, !dbg !8391
  %_0.i3174 = fmul float %_0.i3175, 0x3FB5555560000000, !dbg !8393
  %_4.i4663.v.v = select i1 %_3.i2255, float %_0.i3174, float %_0.i3400, !dbg !8395
  %_4.i4663.v = fmul float %coefficients.i123.i.sroa.22.24.copyload, %_4.i4663.v.v, !dbg !8395
  %_4.i4663 = bitcast float %_4.i4663.v to i32, !dbg !8395
  %603 = fcmp ugt float %_0.i3400, -3.000000e+00, !dbg !8397
  %_7.i4655 = select i1 %603, i32 %_4.i4663, i32 0, !dbg !8399
  %_0.i4657 = bitcast i32 %_7.i4655 to float, !dbg !8400
  %_3.i.i5542 = fcmp ule float %_0.i4657, -1.000000e+02, !dbg !8402
  %604 = bitcast i32 %_7.i4655 to float, !dbg !8405
  %_0.i.i5549 = select i1 %_3.i.i5542, float -1.000000e+02, float %604, !dbg !8408
  %_3.i.i6244 = fcmp olt float %_0.i.i5549, 0.000000e+00, !dbg !8409
  %_0.i.i6251 = select i1 %_3.i.i6244, float %_0.i.i5549, float 0.000000e+00, !dbg !8412
  %605 = bitcast i32 %gain_far.i14.i109.i.sroa.6.011169 to float, !dbg !8414
  %_3.i2451 = fcmp uge float %_0.i.i6251, %605, !dbg !8415
  %_4.i4829.v = select i1 %_3.i2451, float %coefficients.i123.i.sroa.26.24.copyload, float %coefficients.i123.i.sroa.24.24.copyload, !dbg !8418
  %_0.i3453 = fsub float %605, %_0.i.i6251, !dbg !8420
  %_4.i2961 = fmul float %_0.i3453, %_4.i4829.v, !dbg !8422
  %_0.i2962 = fadd float %_0.i.i6251, %_4.i2961, !dbg !8422
  %606 = tail call noundef float @llvm.fabs.f32(float %_0.i2962), !dbg !8424
  %_4.i4393 = bitcast float %_0.i2962 to i32, !dbg !8427
  %607 = fcmp uge float %606, 0x3BC79CA100000000, !dbg !8430
  %_3.i4395 = select i1 %607, i32 %_4.i4393, i32 0, !dbg !8431
  %_0.i4396 = bitcast i32 %_3.i4395 to float, !dbg !8432
  %_0.i2799 = fadd float %_0.i2795.9, %_0.i4396, !dbg !8434
  %_0.i3293 = fmul float %_0.i2799, 0x3FC542A5A0000000, !dbg !8436
  %_3.i.i5242.inv = fcmp ogt float %_0.i3293, -1.260000e+02, !dbg !8439
  %_0.i.i5249 = select i1 %_3.i.i5242.inv, float %_0.i3293, float -1.260000e+02, !dbg !8439
  %_3.i.i6044.inv = fcmp olt float %_0.i.i5249, 1.270000e+02, !dbg !8443
  %_0.i.i6051 = select i1 %_3.i.i6044.inv, float %_0.i.i5249, float 1.270000e+02, !dbg !8443
  %608 = tail call noundef float @llvm.floor.f32(float %_0.i.i6051), !dbg !8446
  %_0.i3376 = fsub float %_0.i.i6051, %608, !dbg !8450
  %_0.i3088 = fmul float %_0.i3376, 0x3F5E974FA0000000, !dbg !8452
  %_0.i2592 = fadd float %_0.i3088, 0x3F82778560000000, !dbg !8454
  %_0.i3088.1 = fmul float %_0.i3376, %_0.i2592, !dbg !8452
  %_0.i2592.1 = fadd float %_0.i3088.1, 0x3FAC91CE60000000, !dbg !8454
  %_0.i3088.2 = fmul float %_0.i3376, %_0.i2592.1, !dbg !8452
  %_0.i2592.2 = fadd float %_0.i3088.2, 0x3FCEBDB560000000, !dbg !8454
  %_0.i3088.3 = fmul float %_0.i3376, %_0.i2592.2, !dbg !8452
  %_0.i2592.3 = fadd float %_0.i3088.3, 0x3FE62E4BA0000000, !dbg !8454
  %_0.i3085 = fmul float %_0.i3375, 0x3F5E974FA0000000, !dbg !8456
  %_0.i2590 = fadd float %_0.i3085, 0x3F82778560000000, !dbg !8458
  %_0.i3085.1 = fmul float %_0.i3375, %_0.i2590, !dbg !8456
  %_0.i2590.1 = fadd float %_0.i3085.1, 0x3FAC91CE60000000, !dbg !8458
  %_0.i3085.2 = fmul float %_0.i3375, %_0.i2590.1, !dbg !8456
  %_0.i2590.2 = fadd float %_0.i3085.2, 0x3FCEBDB560000000, !dbg !8458
  %_0.i3085.3 = fmul float %_0.i3375, %_0.i2590.2, !dbg !8456
  %_0.i2590.3 = fadd float %_0.i3085.3, 0x3FE62E4BA0000000, !dbg !8458
  %_0.i3082 = fmul float %_0.i3374, 0x3F5E974FA0000000, !dbg !8460
  %_0.i2588 = fadd float %_0.i3082, 0x3F82778560000000, !dbg !8462
  %_0.i3082.1 = fmul float %_0.i3374, %_0.i2588, !dbg !8460
  %_0.i2588.1 = fadd float %_0.i3082.1, 0x3FAC91CE60000000, !dbg !8462
  %_0.i3082.2 = fmul float %_0.i3374, %_0.i2588.1, !dbg !8460
  %_0.i2588.2 = fadd float %_0.i3082.2, 0x3FCEBDB560000000, !dbg !8462
  %_0.i3082.3 = fmul float %_0.i3374, %_0.i2588.2, !dbg !8460
  %_0.i2588.3 = fadd float %_0.i3082.3, 0x3FE62E4BA0000000, !dbg !8462
  %_0.i3079 = fmul float %_0.i3373, 0x3F5E974FA0000000, !dbg !8464
  %_0.i2586 = fadd float %_0.i3079, 0x3F82778560000000, !dbg !8466
  %_0.i3079.1 = fmul float %_0.i3373, %_0.i2586, !dbg !8464
  %_0.i2586.1 = fadd float %_0.i3079.1, 0x3FAC91CE60000000, !dbg !8466
  %_0.i3079.2 = fmul float %_0.i3373, %_0.i2586.1, !dbg !8464
  %_0.i2586.2 = fadd float %_0.i3079.2, 0x3FCEBDB560000000, !dbg !8466
  %_0.i3079.3 = fmul float %_0.i3373, %_0.i2586.2, !dbg !8464
  %_0.i2586.3 = fadd float %_0.i3079.3, 0x3FE62E4BA0000000, !dbg !8466
  %_0.i3078 = fmul float %_0.i3373, %_0.i2586.3, !dbg !8468
  %_0.i2585 = fadd float %_0.i3078, 1.000000e+00, !dbg !8470
  %biased.i2178 = fadd float %588, 0x4160000FE0000000, !dbg !8472
  %_4.i2179 = bitcast float %biased.i2178 to i32, !dbg !8474
  %_3.i2180 = shl i32 %_4.i2179, 23, !dbg !8476
  %_0.i2181 = bitcast i32 %_3.i2180 to float, !dbg !8477
  %_0.i3077 = fmul float %_0.i2585, %_0.i2181, !dbg !8479
  %_0.i3081 = fmul float %_0.i3374, %_0.i2588.3, !dbg !8481
  %_0.i2587 = fadd float %_0.i3081, 1.000000e+00, !dbg !8483
  %biased.i2182 = fadd float %596, 0x4160000FE0000000, !dbg !8485
  %_4.i2183 = bitcast float %biased.i2182 to i32, !dbg !8487
  %_3.i2184 = shl i32 %_4.i2183, 23, !dbg !8489
  %_0.i2185 = bitcast i32 %_3.i2184 to float, !dbg !8490
  %_0.i3080 = fmul float %_0.i2587, %_0.i2185, !dbg !8492
  %_0.i3084 = fmul float %_0.i3375, %_0.i2590.3, !dbg !8494
  %_0.i2589 = fadd float %_0.i3084, 1.000000e+00, !dbg !8496
  %biased.i2186 = fadd float %602, 0x4160000FE0000000, !dbg !8498
  %_4.i2187 = bitcast float %biased.i2186 to i32, !dbg !8500
  %_3.i2188 = shl i32 %_4.i2187, 23, !dbg !8502
  %_0.i2189 = bitcast i32 %_3.i2188 to float, !dbg !8503
  %_0.i3083 = fmul float %_0.i2589, %_0.i2189, !dbg !8505
  %_0.i3087 = fmul float %_0.i3376, %_0.i2592.3, !dbg !8507
  %_0.i2591 = fadd float %_0.i3087, 1.000000e+00, !dbg !8509
  %biased.i2190 = fadd float %608, 0x4160000FE0000000, !dbg !8511
  %_4.i2191 = bitcast float %biased.i2190 to i32, !dbg !8513
  %_3.i2192 = shl i32 %_4.i2191, 23, !dbg !8515
  %_0.i2193 = bitcast i32 %_3.i2192 to float, !dbg !8516
  %_0.i3086 = fmul float %_0.i2591, %_0.i2193, !dbg !8518
  %_266.i112.i455.i = icmp ugt i64 %_41.sroa.0.0.i40.i355.i, %_315.1.i80.i395.i, !dbg !8520
  br i1 %_266.i112.i455.i, label %bb86.i149.i492.i, label %bb87.i113.i456.i, !dbg !8520, !prof !1664

bb84.i150.i497.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4057
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i342.i11170, i64 noundef %_314.1.i73.i388.i, i64 noundef %_314.1.i73.i388.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e11ba0c4c1124bbcae4603a2ae121338) #26, !dbg !8525, !noalias !7721
  unreachable, !dbg !8525

bb87.i113.i456.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3591
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8526), !dbg !8529
  %_3.not.i3582 = icmp eq i64 %_315.1.i80.i395.i, %_41.sroa.0.0.i40.i355.i, !dbg !8530
  br i1 %_3.not.i3582, label %panic.i3585, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3586, !dbg !8530

panic.i3585:                                      ; preds = %bb87.i113.i456.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8530, !noalias !8532
  unreachable, !dbg !8530

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3586: ; preds = %bb87.i113.i456.i
  %_273.i116.i459.i = getelementptr inbounds nuw float, ptr %_315.0.i79.i394.i, i64 %_41.sroa.0.0.i40.i355.i, !dbg !8533
  %_0.i3584 = load float, ptr %_273.i116.i459.i, align 4, !dbg !8530, !alias.scope !8526, !noalias !7721, !noundef !11
  %_0.i3292 = fmul float %_0.i3077, %_0.i3584, !dbg !8538
  %_274.i120.i463.i = icmp ugt i64 %_41.sroa.0.0.i40.i355.i, %_316.1.i83.i405.i, !dbg !8540
  br i1 %_274.i120.i463.i, label %bb88.i148.i491.i, label %bb89.i121.i464.i, !dbg !8540, !prof !1664

bb86.i149.i492.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3591
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i355.i, i64 noundef %_315.1.i80.i395.i, i64 noundef %_315.1.i80.i395.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_379b93204ee4659b79f4b07b6520076a) #26, !dbg !8544, !noalias !7721
  unreachable, !dbg !8544

bb89.i121.i464.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3586
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8545), !dbg !8548
  %_3.not.i3577 = icmp eq i64 %_316.1.i83.i405.i, %_41.sroa.0.0.i40.i355.i, !dbg !8549
  br i1 %_3.not.i3577, label %panic.i3580, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3581, !dbg !8549

panic.i3580:                                      ; preds = %bb89.i121.i464.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8549, !noalias !8551
  unreachable, !dbg !8549

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3581: ; preds = %bb89.i121.i464.i
  %_281.i124.i467.i = getelementptr inbounds nuw float, ptr %_316.0.i82.i404.i, i64 %_41.sroa.0.0.i40.i355.i, !dbg !8552
  %_0.i3579 = load float, ptr %_281.i124.i467.i, align 4, !dbg !8549, !alias.scope !8545, !noalias !7721, !noundef !11
  %_0.i3291 = fmul float %_0.i3080, %_0.i3579, !dbg !8557
  %_0.i2798 = fadd float %_0.i3292, %_0.i3291, !dbg !8559
  %_282.i129.i472.i = icmp ugt i64 %_41.sroa.0.0.i40.i355.i, %_317.1.i86.i415.i, !dbg !8561
  br i1 %_282.i129.i472.i, label %bb90.i147.i490.i, label %bb91.i130.i473.i, !dbg !8561, !prof !1664

bb88.i148.i491.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3586
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i355.i, i64 noundef %_316.1.i83.i405.i, i64 noundef %_316.1.i83.i405.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2bb8eb0542a889f29ef069491eb97a17) #26, !dbg !8566, !noalias !7721
  unreachable, !dbg !8566

bb91.i130.i473.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3581
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8567), !dbg !8570
  %_3.not.i3572 = icmp eq i64 %_317.1.i86.i415.i, %_41.sroa.0.0.i40.i355.i, !dbg !8571
  br i1 %_3.not.i3572, label %panic.i3575, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3576, !dbg !8571

panic.i3575:                                      ; preds = %bb91.i130.i473.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8571, !noalias !8573
  unreachable, !dbg !8571

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3576: ; preds = %bb91.i130.i473.i
  %_289.i133.i476.i = getelementptr inbounds nuw float, ptr %_317.0.i85.i414.i, i64 %_41.sroa.0.0.i40.i355.i, !dbg !8574
  %_0.i3574 = load float, ptr %_289.i133.i476.i, align 4, !dbg !8571, !alias.scope !8567, !noalias !7721, !noundef !11
  %_0.i3290 = fmul float %_0.i3083, %_0.i3574, !dbg !8579
  %_290.i137.i480.i = icmp ugt i64 %_41.sroa.0.0.i40.i355.i, %_318.1.i89.i425.i, !dbg !8581
  br i1 %_290.i137.i480.i, label %bb92.i146.i489.i, label %bb93.i138.i481.i, !dbg !8581, !prof !1664

bb90.i147.i490.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3581
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i355.i, i64 noundef %_317.1.i86.i415.i, i64 noundef %_317.1.i86.i415.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd96798e807157d7db308788cb4dd125) #26, !dbg !8585, !noalias !7721
  unreachable, !dbg !8585

bb93.i138.i481.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3576
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8586), !dbg !8589
  %_3.not.i3567 = icmp eq i64 %_318.1.i89.i425.i, %_41.sroa.0.0.i40.i355.i, !dbg !8590
  br i1 %_3.not.i3567, label %panic.i3570, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045, !dbg !8590

panic.i3570:                                      ; preds = %bb93.i138.i481.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !8590, !noalias !8592
  unreachable, !dbg !8590

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045: ; preds = %bb93.i138.i481.i
  %_297.i141.i484.i = getelementptr inbounds nuw float, ptr %_318.0.i88.i424.i, i64 %_41.sroa.0.0.i40.i355.i, !dbg !8593
  %_0.i3569 = load float, ptr %_297.i141.i484.i, align 4, !dbg !8590, !alias.scope !8586, !noalias !7721, !noundef !11
  %_0.i3289 = fmul float %_0.i3086, %_0.i3569, !dbg !8598
  %_0.i2797 = fadd float %_0.i3290, %_0.i3289, !dbg !8600
  store float %_0.i2798, ptr %_184.i42.i359.i, align 4, !dbg !8602, !alias.scope !8605, !noalias !7721
  store float %_0.i2797, ptr %_192.i47.i362.i, align 4, !dbg !8608, !alias.scope !8610, !noalias !7721
  %exitcond14309.not = icmp eq i64 %561, %plan.0.i134.i, !dbg !7673
  br i1 %exitcond14309.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_Kb1_EB2_.exit.i.i, label %bb56.i37.i348.i, !dbg !7685

bb92.i146.i489.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3576
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i355.i, i64 noundef %_318.1.i89.i425.i, i64 noundef %_318.1.i89.i425.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_164adf6876ccf79975d46e129e248ca5) #26, !dbg !8613, !noalias !7721
  unreachable, !dbg !8613

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh1_Kb0_Kb1_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045, %bb17.i324.i
  %segments.i126.i.sroa.0.0 = phi float [ %_12.le.i6718, %bb17.i324.i ], [ %_0.i2796, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.8.0 = phi float [ %_12.le.1.i6720, %bb17.i324.i ], [ %_0.i2796.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.12.0 = phi float [ %_12.le.2.i6722, %bb17.i324.i ], [ %_0.i2796.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.16.0 = phi float [ %_12.le.3.i6724, %bb17.i324.i ], [ %_0.i2796.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.20.0 = phi float [ %_12.le.4.i6726, %bb17.i324.i ], [ %_0.i2796.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.26.0 = phi float [ %_12.le.5.i6728, %bb17.i324.i ], [ %_0.i2796.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.32.0 = phi float [ %_12.le.6.i6730, %bb17.i324.i ], [ %_0.i2796.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.36.0 = phi float [ %_12.le.7.i6732, %bb17.i324.i ], [ %_0.i2796.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.40.0 = phi float [ %_12.le.8.i6734, %bb17.i324.i ], [ %_0.i2796.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.44.0 = phi float [ %_12.le.9.i6736, %bb17.i324.i ], [ %_0.i2796.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.70.0 = phi float [ %_12.le.i6756, %bb17.i324.i ], [ %_0.i2795, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.76.0 = phi float [ %_12.le.1.i6758, %bb17.i324.i ], [ %_0.i2795.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.80.0 = phi float [ %_12.le.2.i6760, %bb17.i324.i ], [ %_0.i2795.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.84.0 = phi float [ %_12.le.3.i6762, %bb17.i324.i ], [ %_0.i2795.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.88.0 = phi float [ %_12.le.4.i6764, %bb17.i324.i ], [ %_0.i2795.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.94.0 = phi float [ %_12.le.5.i6766, %bb17.i324.i ], [ %_0.i2795.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.100.0 = phi float [ %_12.le.6.i6768, %bb17.i324.i ], [ %_0.i2795.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.104.0 = phi float [ %_12.le.7.i6770, %bb17.i324.i ], [ %_0.i2795.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.108.0 = phi float [ %_12.le.8.i6772, %bb17.i324.i ], [ %_0.i2795.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %segments.i126.i.sroa.112.0 = phi float [ %_12.le.9.i6774, %bb17.i324.i ], [ %_0.i2795.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !7687
  %filter_near.i17.i112.i.sroa.0.0.lcssa = phi float [ %filter_near.i17.i112.i.sroa.0.0.copyload, %bb17.i324.i ], [ %_0.i4256, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8614
  %filter_near.i17.i112.i.sroa.7.0.lcssa = phi float [ %filter_near.i17.i112.i.sroa.7.0.copyload, %bb17.i324.i ], [ %_0.i4252, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8614
  %filter_near.i17.i112.i.sroa.11.0.lcssa = phi float [ %filter_near.i17.i112.i.sroa.11.0.copyload, %bb17.i324.i ], [ %_0.i4264, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8614
  %filter_near.i17.i112.i.sroa.14.0.lcssa = phi float [ %filter_near.i17.i112.i.sroa.14.0.copyload, %bb17.i324.i ], [ %_0.i4260, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8614
  %filter_far.i16.i111.i.sroa.0.0.lcssa = phi float [ %filter_far.i16.i111.i.sroa.0.0.copyload, %bb17.i324.i ], [ %_0.i4240, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8615
  %filter_far.i16.i111.i.sroa.7.0.lcssa = phi float [ %filter_far.i16.i111.i.sroa.7.0.copyload, %bb17.i324.i ], [ %_0.i4236, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8615
  %filter_far.i16.i111.i.sroa.11.0.lcssa = phi float [ %filter_far.i16.i111.i.sroa.11.0.copyload, %bb17.i324.i ], [ %_0.i4248, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8615
  %filter_far.i16.i111.i.sroa.14.0.lcssa = phi float [ %filter_far.i16.i111.i.sroa.14.0.copyload, %bb17.i324.i ], [ %_0.i4244, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8615
  %gain_near.i15.i110.i.sroa.0.0.lcssa = phi i32 [ %556, %bb17.i324.i ], [ %_3.i4407, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8616
  %gain_near.i15.i110.i.sroa.6.0.lcssa = phi i32 [ %557, %bb17.i324.i ], [ %_3.i4403, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8616
  %gain_far.i14.i109.i.sroa.0.0.lcssa = phi i32 [ %558, %bb17.i324.i ], [ %_3.i4399, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8617
  %gain_far.i14.i109.i.sroa.6.0.lcssa = phi i32 [ %559, %bb17.i324.i ], [ %_3.i4395, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8617
  %position.sroa.0.0.i32.i342.i.lcssa = phi i64 [ %560, %bb17.i324.i ], [ %_41.sroa.0.0.i40.i355.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4045 ], !dbg !8618
  store float %filter_near.i17.i112.i.sroa.0.0.lcssa, ptr %466, align 8, !dbg !8619, !noalias !7721
  store float %filter_near.i17.i112.i.sroa.7.0.lcssa, ptr %filter_near.i.i119.i.sroa.7.0..sroa_idx, align 4, !dbg !8619, !noalias !7721
  store float %filter_near.i17.i112.i.sroa.11.0.lcssa, ptr %filter_near.i.i119.i.sroa.11.0..sroa_idx, align 8, !dbg !8619, !noalias !7721
  store float %filter_near.i17.i112.i.sroa.14.0.lcssa, ptr %filter_near.i.i119.i.sroa.14.0..sroa_idx, align 4, !dbg !8619, !noalias !7721
  store float %filter_far.i16.i111.i.sroa.0.0.lcssa, ptr %467, align 8, !dbg !8620, !noalias !7721
  store float %filter_far.i16.i111.i.sroa.7.0.lcssa, ptr %filter_far.i.i118.i.sroa.7.0..sroa_idx, align 4, !dbg !8620, !noalias !7721
  store float %filter_far.i16.i111.i.sroa.11.0.lcssa, ptr %filter_far.i.i118.i.sroa.11.0..sroa_idx, align 8, !dbg !8620, !noalias !7721
  store float %filter_far.i16.i111.i.sroa.14.0.lcssa, ptr %filter_far.i.i118.i.sroa.14.0..sroa_idx, align 4, !dbg !8620, !noalias !7721
  store i32 %gain_near.i15.i110.i.sroa.0.0.lcssa, ptr %468, align 8, !dbg !8621, !noalias !7721
  store i32 %gain_near.i15.i110.i.sroa.6.0.lcssa, ptr %.sroa_idx7061, align 4, !dbg !8621, !noalias !7721
  store i32 %gain_far.i14.i109.i.sroa.0.0.lcssa, ptr %469, align 8, !dbg !8622, !noalias !7721
  store i32 %gain_far.i14.i109.i.sroa.6.0.lcssa, ptr %.sroa_idx7066, align 4, !dbg !8622, !noalias !7721
  store i64 %position.sroa.0.0.i32.i342.i.lcssa, ptr %_51.i138.i, align 8, !dbg !8623, !alias.scope !7670, !noalias !7671
  %advanced.i347.i = trunc i64 %plan.0.i134.i to i32, !dbg !8624
  store float %segments.i126.i.sroa.0.0, ptr %426, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.i6803 = load i32, ptr %482, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %609 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6803, i32 %advanced.i347.i), !dbg !8634
  store i32 %609, ptr %482, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.8.0, ptr %428, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.1.i6806 = load i32, ptr %483, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %610 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6806, i32 %advanced.i347.i), !dbg !8634
  store i32 %610, ptr %483, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.12.0, ptr %430, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.2.i6809 = load i32, ptr %484, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %611 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6809, i32 %advanced.i347.i), !dbg !8634
  store i32 %611, ptr %484, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.16.0, ptr %432, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.3.i6812 = load i32, ptr %485, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %612 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6812, i32 %advanced.i347.i), !dbg !8634
  store i32 %612, ptr %485, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.20.0, ptr %434, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.4.i6815 = load i32, ptr %486, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %613 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6815, i32 %advanced.i347.i), !dbg !8634
  store i32 %613, ptr %486, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.26.0, ptr %436, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.5.i6818 = load i32, ptr %487, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %614 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6818, i32 %advanced.i347.i), !dbg !8634
  store i32 %614, ptr %487, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.32.0, ptr %438, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.6.i6821 = load i32, ptr %488, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %615 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6821, i32 %advanced.i347.i), !dbg !8634
  store i32 %615, ptr %488, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.36.0, ptr %440, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.7.i6824 = load i32, ptr %489, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %616 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6824, i32 %advanced.i347.i), !dbg !8634
  store i32 %616, ptr %489, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.40.0, ptr %442, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.8.i6827 = load i32, ptr %490, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %617 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6827, i32 %advanced.i347.i), !dbg !8634
  store i32 %617, ptr %490, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.44.0, ptr %444, align 4, !dbg !8625, !alias.scope !8628, !noalias !8631
  %_26.9.i6830 = load i32, ptr %491, align 4, !dbg !8633, !alias.scope !8628, !noalias !8631, !noundef !11
  %618 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6830, i32 %advanced.i347.i), !dbg !8634
  store i32 %618, ptr %491, align 4, !dbg !8636, !alias.scope !8628, !noalias !8631
  store float %segments.i126.i.sroa.70.0, ptr %446, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.i6832 = load i32, ptr %492, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %619 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6832, i32 %advanced.i347.i), !dbg !8645
  store i32 %619, ptr %492, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.76.0, ptr %448, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.1.i6835 = load i32, ptr %493, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %620 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6835, i32 %advanced.i347.i), !dbg !8645
  store i32 %620, ptr %493, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.80.0, ptr %450, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.2.i6838 = load i32, ptr %494, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %621 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6838, i32 %advanced.i347.i), !dbg !8645
  store i32 %621, ptr %494, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.84.0, ptr %452, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.3.i6841 = load i32, ptr %495, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %622 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6841, i32 %advanced.i347.i), !dbg !8645
  store i32 %622, ptr %495, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.88.0, ptr %454, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.4.i6844 = load i32, ptr %496, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %623 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6844, i32 %advanced.i347.i), !dbg !8645
  store i32 %623, ptr %496, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.94.0, ptr %456, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.5.i6847 = load i32, ptr %497, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %624 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6847, i32 %advanced.i347.i), !dbg !8645
  store i32 %624, ptr %497, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.100.0, ptr %458, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.6.i6850 = load i32, ptr %498, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %625 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6850, i32 %advanced.i347.i), !dbg !8645
  store i32 %625, ptr %498, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.104.0, ptr %460, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.7.i6853 = load i32, ptr %499, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %626 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6853, i32 %advanced.i347.i), !dbg !8645
  store i32 %626, ptr %499, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.108.0, ptr %462, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.8.i6856 = load i32, ptr %500, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %627 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6856, i32 %advanced.i347.i), !dbg !8645
  store i32 %627, ptr %500, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  store float %segments.i126.i.sroa.112.0, ptr %464, align 4, !dbg !8637, !alias.scope !8639, !noalias !8642
  %_26.9.i6859 = load i32, ptr %501, align 4, !dbg !8644, !alias.scope !8639, !noalias !8642, !noundef !11
  %628 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6859, i32 %advanced.i347.i), !dbg !8645
  store i32 %628, ptr %501, align 4, !dbg !8647, !alias.scope !8639, !noalias !8642
  br label %bb15.i166.i, !dbg !7636

bb2.i538.i.lr.ph:                                 ; preds = %bb4.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8648), !dbg !8651
  %629 = getelementptr inbounds nuw i8, ptr %self, i64 856, !dbg !8652
  %sample_rate.i533.i = load i32, ptr %629, align 8, !dbg !8652, !alias.scope !8655, !noalias !8656, !noundef !11
  %630 = getelementptr inbounds nuw i8, ptr %self, i64 840, !dbg !8659
  %ring_len.i534.i = load i64, ptr %630, align 8, !dbg !8659, !alias.scope !8655, !noalias !8656, !noundef !11
  %631 = getelementptr inbounds nuw i8, ptr %self, i64 120
  %632 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %633 = getelementptr inbounds nuw i8, ptr %self, i64 200
  %634 = getelementptr inbounds nuw i8, ptr %self, i64 208
  %635 = getelementptr inbounds nuw i8, ptr %self, i64 216
  %636 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %637 = getelementptr inbounds nuw i8, ptr %self, i64 232
  %638 = getelementptr inbounds nuw i8, ptr %self, i64 240
  %639 = getelementptr inbounds nuw i8, ptr %self, i64 248
  %640 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %641 = getelementptr inbounds nuw i8, ptr %self, i64 264
  %642 = getelementptr inbounds nuw i8, ptr %self, i64 272
  %643 = getelementptr inbounds nuw i8, ptr %self, i64 280
  %644 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %645 = getelementptr inbounds nuw i8, ptr %self, i64 296
  %646 = getelementptr inbounds nuw i8, ptr %self, i64 304
  %647 = getelementptr inbounds nuw i8, ptr %self, i64 312
  %648 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %649 = getelementptr inbounds nuw i8, ptr %self, i64 328
  %650 = getelementptr inbounds nuw i8, ptr %self, i64 336
  %651 = getelementptr inbounds nuw i8, ptr %self, i64 344
  %_18.i542.i = getelementptr inbounds nuw i8, ptr %self, i64 480
  %652 = getelementptr inbounds nuw i8, ptr %self, i64 552
  %653 = getelementptr inbounds nuw i8, ptr %self, i64 560
  %654 = getelementptr inbounds nuw i8, ptr %self, i64 568
  %655 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %656 = getelementptr inbounds nuw i8, ptr %self, i64 584
  %657 = getelementptr inbounds nuw i8, ptr %self, i64 592
  %658 = getelementptr inbounds nuw i8, ptr %self, i64 600
  %659 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %660 = getelementptr inbounds nuw i8, ptr %self, i64 616
  %661 = getelementptr inbounds nuw i8, ptr %self, i64 624
  %662 = getelementptr inbounds nuw i8, ptr %self, i64 632
  %663 = getelementptr inbounds nuw i8, ptr %self, i64 640
  %664 = getelementptr inbounds nuw i8, ptr %self, i64 648
  %665 = getelementptr inbounds nuw i8, ptr %self, i64 656
  %666 = getelementptr inbounds nuw i8, ptr %self, i64 664
  %667 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %668 = getelementptr inbounds nuw i8, ptr %self, i64 680
  %669 = getelementptr inbounds nuw i8, ptr %self, i64 688
  %670 = getelementptr inbounds nuw i8, ptr %self, i64 696
  %671 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %coefficients.i529.i.sroa.5.0._20.i528.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i528.i, i64 4
  %coefficients.i529.i.sroa.7.0._20.i528.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i528.i, i64 8
  %coefficients.i529.i.sroa.9.0._20.i528.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i528.i, i64 12
  %coefficients.i529.i.sroa.11.0._20.i528.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i528.i, i64 16
  %coefficients.i529.i.sroa.13.0._20.i528.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_20.i528.i, i64 20
  %coefficients.i529.i.sroa.18.24._22.i527.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i527.i, i64 4
  %coefficients.i529.i.sroa.20.24._22.i527.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i527.i, i64 8
  %coefficients.i529.i.sroa.22.24._22.i527.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i527.i, i64 12
  %coefficients.i529.i.sroa.24.24._22.i527.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i527.i, i64 16
  %coefficients.i529.i.sroa.26.24._22.i527.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_22.i527.i, i64 20
  %_51.i544.i = getelementptr inbounds nuw i8, ptr %self, i64 848
  %672 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %filter_near.i.i525.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 172
  %filter_near.i.i525.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 176
  %filter_near.i.i525.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 180
  %673 = getelementptr inbounds nuw i8, ptr %self, i64 528
  %filter_far.i.i524.i.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 532
  %filter_far.i.i524.i.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 536
  %filter_far.i.i524.i.sroa.14.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 540
  %674 = getelementptr inbounds nuw i8, ptr %self, i64 184
  %.sroa_idx7097 = getelementptr inbounds nuw i8, ptr %self, i64 188
  %675 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %.sroa_idx7102 = getelementptr inbounds nuw i8, ptr %self, i64 548
  %_24.i.i562.i = getelementptr inbounds nuw i8, ptr %self, i64 400
  %_26.i.i564.i = getelementptr inbounds nuw i8, ptr %self, i64 760
  %_67.i.i583.i = getelementptr inbounds nuw i8, ptr %self, i64 152
  %676 = getelementptr inbounds nuw i8, ptr %self, i64 156
  %677 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %678 = getelementptr inbounds nuw i8, ptr %self, i64 164
  %_72.i.i585.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %679 = getelementptr inbounds nuw i8, ptr %self, i64 516
  %680 = getelementptr inbounds nuw i8, ptr %self, i64 520
  %681 = getelementptr inbounds nuw i8, ptr %self, i64 524
  %682 = getelementptr inbounds nuw i8, ptr %self, i64 128
  %683 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %684 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %685 = getelementptr inbounds nuw i8, ptr %self, i64 488
  %686 = getelementptr inbounds nuw i8, ptr %self, i64 496
  %687 = getelementptr inbounds nuw i8, ptr %self, i64 504
  %688 = getelementptr inbounds nuw i8, ptr %self, i64 204
  %689 = getelementptr inbounds nuw i8, ptr %self, i64 220
  %690 = getelementptr inbounds nuw i8, ptr %self, i64 236
  %691 = getelementptr inbounds nuw i8, ptr %self, i64 252
  %692 = getelementptr inbounds nuw i8, ptr %self, i64 268
  %693 = getelementptr inbounds nuw i8, ptr %self, i64 284
  %694 = getelementptr inbounds nuw i8, ptr %self, i64 300
  %695 = getelementptr inbounds nuw i8, ptr %self, i64 316
  %696 = getelementptr inbounds nuw i8, ptr %self, i64 332
  %697 = getelementptr inbounds nuw i8, ptr %self, i64 348
  %698 = getelementptr inbounds nuw i8, ptr %self, i64 564
  %699 = getelementptr inbounds nuw i8, ptr %self, i64 580
  %700 = getelementptr inbounds nuw i8, ptr %self, i64 596
  %701 = getelementptr inbounds nuw i8, ptr %self, i64 612
  %702 = getelementptr inbounds nuw i8, ptr %self, i64 628
  %703 = getelementptr inbounds nuw i8, ptr %self, i64 644
  %704 = getelementptr inbounds nuw i8, ptr %self, i64 660
  %705 = getelementptr inbounds nuw i8, ptr %self, i64 676
  %706 = getelementptr inbounds nuw i8, ptr %self, i64 692
  %707 = getelementptr inbounds nuw i8, ptr %self, i64 708
  br label %bb2.i538.i, !dbg !8661

bb2.i538.i:                                       ; preds = %bb2.i538.i.lr.ph, %bb15.i572.i
  %position.sroa.0.0.i536.i11127 = phi i64 [ 0, %bb2.i538.i.lr.ph ], [ %_32.i726.i, %bb15.i572.i ]
  %_11.i539.i = sub nuw i64 %_19.1, %position.sroa.0.0.i536.i11127, !dbg !8664
; call <multiband_compressor::Instance<f32, 1>>::plan_segment
  %708 = tail call fastcc { i64, i1 } @_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E12plan_segmentB5_(ptr noalias noundef nonnull align 8 dereferenceable(768) %_5, i64 noundef %_11.i539.i) #25, !dbg !8665, !noalias !3842
  %plan.0.i540.i = extractvalue { i64, i1 } %708, 0, !dbg !8665
  %plan.1.i541.i = extractvalue { i64, i1 } %708, 1, !dbg !8665
  %_12.le.i6860 = load float, ptr %632, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.i6861 = load float, ptr %633, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.1.i6862 = load float, ptr %634, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.1.i6863 = load float, ptr %635, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.2.i6864 = load float, ptr %636, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.2.i6865 = load float, ptr %637, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.3.i6866 = load float, ptr %638, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.3.i6867 = load float, ptr %639, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.4.i6868 = load float, ptr %640, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.4.i6869 = load float, ptr %641, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.5.i6870 = load float, ptr %642, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.5.i6871 = load float, ptr %643, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.6.i6872 = load float, ptr %644, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.6.i6873 = load float, ptr %645, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.7.i6874 = load float, ptr %646, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.7.i6875 = load float, ptr %647, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.8.i6876 = load float, ptr %648, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.8.i6877 = load float, ptr %649, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.9.i6878 = load float, ptr %650, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_16.le.9.i6879 = load float, ptr %651, align 8, !alias.scope !8666, !noalias !8669, !noundef !11
  %_12.le.i6898 = load float, ptr %652, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.i6899 = load float, ptr %653, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.1.i6900 = load float, ptr %654, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.1.i6901 = load float, ptr %655, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.2.i6902 = load float, ptr %656, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.2.i6903 = load float, ptr %657, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.3.i6904 = load float, ptr %658, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.3.i6905 = load float, ptr %659, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.4.i6906 = load float, ptr %660, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.4.i6907 = load float, ptr %661, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.5.i6908 = load float, ptr %662, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.5.i6909 = load float, ptr %663, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.6.i6910 = load float, ptr %664, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.6.i6911 = load float, ptr %665, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.7.i6912 = load float, ptr %666, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.7.i6913 = load float, ptr %667, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.8.i6914 = load float, ptr %668, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.8.i6915 = load float, ptr %669, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_12.le.9.i6916 = load float, ptr %670, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  %_16.le.9.i6917 = load float, ptr %671, align 8, !alias.scope !8671, !noalias !8674, !noundef !11
  call void @llvm.lifetime.start.p0(ptr nonnull %_20.i528.i), !dbg !8676, !noalias !8680
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_20.i528.i, ptr noalias noundef align 8 dereferenceable(360) %631, i32 noundef %sample_rate.i533.i) #25, !dbg !8681, !noalias !3842
  call void @llvm.lifetime.start.p0(ptr nonnull %_22.i527.i), !dbg !8682, !noalias !8680
; call <multiband_compressor::Side<f32, 1>>::band_coefficients
  call fastcc void @_RNvMs2_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E17band_coefficientsB5_(ptr noalias noundef align 4 captures(none) dereferenceable(24) %_22.i527.i, ptr noalias noundef align 8 dereferenceable(360) %_18.i542.i, i32 noundef %sample_rate.i533.i) #25, !dbg !8683, !noalias !3842
  %coefficients.i529.i.sroa.0.0.copyload = load float, ptr %_20.i528.i, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.5.0.copyload = load float, ptr %coefficients.i529.i.sroa.5.0._20.i528.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.7.0.copyload = load float, ptr %coefficients.i529.i.sroa.7.0._20.i528.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.9.0.copyload = load float, ptr %coefficients.i529.i.sroa.9.0._20.i528.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.11.0.copyload = load float, ptr %coefficients.i529.i.sroa.11.0._20.i528.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.13.0.copyload = load float, ptr %coefficients.i529.i.sroa.13.0._20.i528.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.15.24.copyload = load float, ptr %_22.i527.i, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.18.24.copyload = load float, ptr %coefficients.i529.i.sroa.18.24._22.i527.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.20.24.copyload = load float, ptr %coefficients.i529.i.sroa.20.24._22.i527.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.22.24.copyload = load float, ptr %coefficients.i529.i.sroa.22.24._22.i527.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.24.24.copyload = load float, ptr %coefficients.i529.i.sroa.24.24._22.i527.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  %coefficients.i529.i.sroa.26.24.copyload = load float, ptr %coefficients.i529.i.sroa.26.24._22.i527.i.sroa_idx, align 4, !dbg !8684, !noalias !8680
  call void @llvm.lifetime.end.p0(ptr nonnull %_22.i527.i), !dbg !8685, !noalias !8680
  call void @llvm.lifetime.end.p0(ptr nonnull %_20.i528.i), !dbg !8685, !noalias !8680
  %_32.i726.i = add i64 %plan.0.i540.i, %position.sroa.0.0.i536.i11127, !dbg !8686
  %_72.i727.i = icmp ult i64 %_32.i726.i, %position.sroa.0.0.i536.i11127, !dbg !8688
  %_66.not.i728.i = icmp ugt i64 %_32.i726.i, %_19.1
  %or.cond11.i729.i = or i1 %_72.i727.i, %_66.not.i728.i, !dbg !8688
  br i1 %plan.1.i541.i, label %bb9.i724.i, label %bb13.i543.i, !dbg !8694

bb9.i724.i:                                       ; preds = %bb2.i538.i
  br i1 %or.cond11.i729.i, label %bb19.i918.i, label %bb17.i730.i, !dbg !8695, !prof !3878

bb13.i543.i:                                      ; preds = %bb2.i538.i
  br i1 %or.cond11.i729.i, label %bb29.i723.i, label %bb27.i549.i, !dbg !8699, !prof !3878

bb27.i549.i:                                      ; preds = %bb13.i543.i
  %_95.i550.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i536.i11127, !dbg !8705
  %_105.i553.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i536.i11127, !dbg !8709
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8716), !dbg !8719
  %filter_near.i.i525.i.sroa.0.0.copyload = load float, ptr %672, align 8, !dbg !8720, !noalias !8726
  %filter_near.i.i525.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i525.i.sroa.7.0..sroa_idx, align 4, !dbg !8720, !noalias !8726
  %filter_near.i.i525.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i525.i.sroa.11.0..sroa_idx, align 8, !dbg !8720, !noalias !8726
  %filter_near.i.i525.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i525.i.sroa.14.0..sroa_idx, align 4, !dbg !8720, !noalias !8726
  %filter_far.i.i524.i.sroa.0.0.copyload = load float, ptr %673, align 8, !dbg !8731, !noalias !8726
  %filter_far.i.i524.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i524.i.sroa.7.0..sroa_idx, align 4, !dbg !8731, !noalias !8726
  %filter_far.i.i524.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i524.i.sroa.11.0..sroa_idx, align 8, !dbg !8731, !noalias !8726
  %filter_far.i.i524.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i524.i.sroa.14.0..sroa_idx, align 4, !dbg !8731, !noalias !8726
  %709 = load i32, ptr %674, align 8, !dbg !8733
  %710 = load i32, ptr %.sroa_idx7097, align 4, !dbg !8733
  %711 = load i32, ptr %675, align 8, !dbg !8735
  %712 = load i32, ptr %.sroa_idx7102, align 4, !dbg !8735
  %713 = load i64, ptr %_51.i544.i, align 8, !dbg !8737, !alias.scope !8739, !noalias !8740, !noundef !11
  %_168.i.i569.i11070.not = icmp eq i64 %plan.0.i540.i, 0, !dbg !8742
  br i1 %_168.i.i569.i11070.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_KBZ_EB2_.exit.i.i, label %bb56.i.i573.i, !dbg !8754

bb29.i723.i:                                      ; preds = %bb13.i543.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i536.i11127, i64 noundef %_32.i726.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ac5156198d2516c0e2a17140923b083) #26, !dbg !8755, !noalias !3842
  unreachable, !dbg !8755

bb56.i.i573.i:                                    ; preds = %bb27.i549.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149
  %iter.sroa.0.0.i.i568.i11084 = phi i64 [ %714, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ 0, %bb27.i549.i ]
  %position.sroa.0.0.i.i567.i11083 = phi i64 [ %_41.sroa.0.0.i.i576.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %713, %bb27.i549.i ]
  %gain_far.i.i522.i.sroa.6.011082 = phi i32 [ %_3.i4443, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %712, %bb27.i549.i ]
  %gain_far.i.i522.i.sroa.0.011081 = phi i32 [ %_3.i4447, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %711, %bb27.i549.i ]
  %gain_near.i.i523.i.sroa.6.011080 = phi i32 [ %_3.i4451, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %710, %bb27.i549.i ]
  %gain_near.i.i523.i.sroa.0.011079 = phi i32 [ %_3.i4455, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %709, %bb27.i549.i ]
  %filter_far.i.i524.i.sroa.14.011078 = phi float [ %_0.i4340, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %filter_far.i.i524.i.sroa.14.0.copyload, %bb27.i549.i ]
  %filter_far.i.i524.i.sroa.11.011077 = phi float [ %_0.i4344, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %filter_far.i.i524.i.sroa.11.0.copyload, %bb27.i549.i ]
  %filter_far.i.i524.i.sroa.7.011076 = phi float [ %_0.i4332, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %filter_far.i.i524.i.sroa.7.0.copyload, %bb27.i549.i ]
  %filter_far.i.i524.i.sroa.0.011075 = phi float [ %_0.i4336, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %filter_far.i.i524.i.sroa.0.0.copyload, %bb27.i549.i ]
  %filter_near.i.i525.i.sroa.14.011074 = phi float [ %_0.i4356, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %filter_near.i.i525.i.sroa.14.0.copyload, %bb27.i549.i ]
  %filter_near.i.i525.i.sroa.11.011073 = phi float [ %_0.i4360, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %filter_near.i.i525.i.sroa.11.0.copyload, %bb27.i549.i ]
  %filter_near.i.i525.i.sroa.7.011072 = phi float [ %_0.i4348, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %filter_near.i.i525.i.sroa.7.0.copyload, %bb27.i549.i ]
  %filter_near.i.i525.i.sroa.0.011071 = phi float [ %_0.i4352, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], [ %filter_near.i.i525.i.sroa.0.0.copyload, %bb27.i549.i ]
  %714 = add nuw i64 %iter.sroa.0.0.i.i568.i11084, 1, !dbg !8756
  %_42.i.i574.i = add i64 %position.sroa.0.0.i.i567.i11083, 1, !dbg !8762
  %_176.not.i.i575.i = icmp ult i64 %_42.i.i574.i, %ring_len.i534.i, !dbg !8765
  %715 = select i1 %_176.not.i.i575.i, i64 0, i64 %ring_len.i534.i, !dbg !8765
  %_41.sroa.0.0.i.i576.i = sub nuw i64 %_42.i.i574.i, %715, !dbg !8765
  %_184.i.i578.i = getelementptr inbounds nuw float, ptr %_95.i550.i, i64 %iter.sroa.0.0.i.i568.i11084, !dbg !8768
  %_0.i3804 = load float, ptr %_184.i.i578.i, align 4, !dbg !8778, !alias.scope !8780, !noalias !8783, !noundef !11
  %_192.i.i581.i = getelementptr inbounds nuw float, ptr %_105.i553.i, i64 %iter.sroa.0.0.i.i568.i11084, !dbg !8784
  %_0.i3799 = load float, ptr %_192.i.i581.i, align 4, !dbg !8793, !alias.scope !8795, !noalias !8783, !noundef !11
  %_7.i132 = load float, ptr %_67.i.i583.i, align 4, !dbg !8798, !alias.scope !8801, !noalias !8804, !noundef !11
  %_8.i133 = load float, ptr %676, align 4, !dbg !8806, !alias.scope !8801, !noalias !8804, !noundef !11
  %_9.i134 = load float, ptr %677, align 4, !dbg !8807, !alias.scope !8801, !noalias !8804, !noundef !11
  %_0.i3431 = fsub float %_0.i3804, %filter_near.i.i525.i.sroa.7.011072, !dbg !8808
  %_0.i3262 = fmul float %_0.i3431, %_8.i133, !dbg !8811
  %_4.i2915 = fmul float %filter_near.i.i525.i.sroa.0.011071, %_7.i132, !dbg !8813
  %_0.i2916 = fadd float %_4.i2915, %_0.i3262, !dbg !8813
  %_0.i2774 = fadd float %filter_near.i.i525.i.sroa.0.011071, %_0.i2916, !dbg !8815
  %_0.i3261 = fmul float %filter_near.i.i525.i.sroa.0.011071, %_8.i133, !dbg !8817
  %_4.i2913 = fmul float %_0.i3431, %_9.i134, !dbg !8819
  %_0.i2914 = fadd float %_0.i3261, %_4.i2913, !dbg !8819
  %_0.i2773 = fadd float %filter_near.i.i525.i.sroa.7.011072, %_0.i2914, !dbg !8821
  %_0.i2772 = fadd float %_0.i2916, %_0.i2916, !dbg !8823
  %_0.i2771 = fadd float %filter_near.i.i525.i.sroa.0.011071, %_0.i2772, !dbg !8825
  %716 = tail call noundef float @llvm.fabs.f32(float %_0.i2771), !dbg !8827
  %717 = fcmp uge float %716, 0x3BC79CA100000000, !dbg !8830
  %_0.i4352 = select i1 %717, float %_0.i2771, float 0.000000e+00, !dbg !8832
  %_0.i2770 = fadd float %_0.i2914, %_0.i2914, !dbg !8833
  %_0.i2769 = fadd float %filter_near.i.i525.i.sroa.7.011072, %_0.i2770, !dbg !8835
  %718 = tail call noundef float @llvm.fabs.f32(float %_0.i2769), !dbg !8837
  %719 = fcmp uge float %718, 0x3BC79CA100000000, !dbg !8840
  %_0.i4348 = select i1 %719, float %_0.i2769, float 0.000000e+00, !dbg !8842
  %_12.i137 = load float, ptr %678, align 4, !dbg !8843, !alias.scope !8801, !noalias !8804, !noundef !11
  %_4.i2921 = fmul float %_12.i137, %_0.i2774, !dbg !8844
  %_0.i2922 = fadd float %_0.i3804, %_4.i2921, !dbg !8844
  %_0.i3432 = fsub float %_0.i2773, %filter_near.i.i525.i.sroa.14.011074, !dbg !8846
  %_0.i3264 = fmul float %_8.i133, %_0.i3432, !dbg !8849
  %_4.i2919 = fmul float %filter_near.i.i525.i.sroa.11.011073, %_7.i132, !dbg !8851
  %_0.i2920 = fadd float %_4.i2919, %_0.i3264, !dbg !8851
  %_0.i3263 = fmul float %filter_near.i.i525.i.sroa.11.011073, %_8.i133, !dbg !8853
  %_4.i2917 = fmul float %_9.i134, %_0.i3432, !dbg !8855
  %_0.i2918 = fadd float %_0.i3263, %_4.i2917, !dbg !8855
  %_0.i2779 = fadd float %filter_near.i.i525.i.sroa.14.011074, %_0.i2918, !dbg !8857
  %_0.i2778 = fadd float %_0.i2920, %_0.i2920, !dbg !8859
  %_0.i2777 = fadd float %filter_near.i.i525.i.sroa.11.011073, %_0.i2778, !dbg !8861
  %720 = tail call noundef float @llvm.fabs.f32(float %_0.i2777), !dbg !8863
  %721 = fcmp uge float %720, 0x3BC79CA100000000, !dbg !8866
  %_0.i4360 = select i1 %721, float %_0.i2777, float 0.000000e+00, !dbg !8868
  %_0.i2776 = fadd float %_0.i2918, %_0.i2918, !dbg !8869
  %_0.i2775 = fadd float %filter_near.i.i525.i.sroa.14.011074, %_0.i2776, !dbg !8871
  %722 = tail call noundef float @llvm.fabs.f32(float %_0.i2775), !dbg !8873
  %723 = fcmp uge float %722, 0x3BC79CA100000000, !dbg !8876
  %_0.i4356 = select i1 %723, float %_0.i2775, float 0.000000e+00, !dbg !8878
  %_0.i3433 = fsub float %_0.i2922, %_0.i2779, !dbg !8879
  %_7.i119 = load float, ptr %_72.i.i585.i, align 4, !dbg !8881, !alias.scope !8884, !noalias !8887, !noundef !11
  %_8.i120 = load float, ptr %679, align 4, !dbg !8889, !alias.scope !8884, !noalias !8887, !noundef !11
  %_9.i121 = load float, ptr %680, align 4, !dbg !8890, !alias.scope !8884, !noalias !8887, !noundef !11
  %_0.i3429 = fsub float %_0.i3799, %filter_far.i.i524.i.sroa.7.011076, !dbg !8891
  %_0.i3258 = fmul float %_0.i3429, %_8.i120, !dbg !8894
  %_4.i2907 = fmul float %filter_far.i.i524.i.sroa.0.011075, %_7.i119, !dbg !8896
  %_0.i2908 = fadd float %_4.i2907, %_0.i3258, !dbg !8896
  %_0.i2762 = fadd float %filter_far.i.i524.i.sroa.0.011075, %_0.i2908, !dbg !8898
  %_0.i3257 = fmul float %filter_far.i.i524.i.sroa.0.011075, %_8.i120, !dbg !8900
  %_4.i2905 = fmul float %_0.i3429, %_9.i121, !dbg !8902
  %_0.i2906 = fadd float %_0.i3257, %_4.i2905, !dbg !8902
  %_0.i2761 = fadd float %filter_far.i.i524.i.sroa.7.011076, %_0.i2906, !dbg !8904
  %_0.i2760 = fadd float %_0.i2908, %_0.i2908, !dbg !8906
  %_0.i2759 = fadd float %filter_far.i.i524.i.sroa.0.011075, %_0.i2760, !dbg !8908
  %724 = tail call noundef float @llvm.fabs.f32(float %_0.i2759), !dbg !8910
  %725 = fcmp uge float %724, 0x3BC79CA100000000, !dbg !8913
  %_0.i4336 = select i1 %725, float %_0.i2759, float 0.000000e+00, !dbg !8915
  %_0.i2758 = fadd float %_0.i2906, %_0.i2906, !dbg !8916
  %_0.i2757 = fadd float %filter_far.i.i524.i.sroa.7.011076, %_0.i2758, !dbg !8918
  %726 = tail call noundef float @llvm.fabs.f32(float %_0.i2757), !dbg !8920
  %727 = fcmp uge float %726, 0x3BC79CA100000000, !dbg !8923
  %_0.i4332 = select i1 %727, float %_0.i2757, float 0.000000e+00, !dbg !8925
  %_12.i124 = load float, ptr %681, align 4, !dbg !8926, !alias.scope !8884, !noalias !8887, !noundef !11
  %_4.i2923 = fmul float %_12.i124, %_0.i2762, !dbg !8927
  %_0.i2924 = fadd float %_0.i3799, %_4.i2923, !dbg !8927
  %_0.i3430 = fsub float %_0.i2761, %filter_far.i.i524.i.sroa.14.011078, !dbg !8929
  %_0.i3260 = fmul float %_8.i120, %_0.i3430, !dbg !8932
  %_4.i2911 = fmul float %filter_far.i.i524.i.sroa.11.011077, %_7.i119, !dbg !8934
  %_0.i2912 = fadd float %_4.i2911, %_0.i3260, !dbg !8934
  %_0.i3259 = fmul float %filter_far.i.i524.i.sroa.11.011077, %_8.i120, !dbg !8936
  %_4.i2909 = fmul float %_9.i121, %_0.i3430, !dbg !8938
  %_0.i2910 = fadd float %_0.i3259, %_4.i2909, !dbg !8938
  %_0.i2767 = fadd float %filter_far.i.i524.i.sroa.14.011078, %_0.i2910, !dbg !8940
  %_0.i2766 = fadd float %_0.i2912, %_0.i2912, !dbg !8942
  %_0.i2765 = fadd float %filter_far.i.i524.i.sroa.11.011077, %_0.i2766, !dbg !8944
  %728 = tail call noundef float @llvm.fabs.f32(float %_0.i2765), !dbg !8946
  %729 = fcmp uge float %728, 0x3BC79CA100000000, !dbg !8949
  %_0.i4344 = select i1 %729, float %_0.i2765, float 0.000000e+00, !dbg !8951
  %_0.i2764 = fadd float %_0.i2910, %_0.i2910, !dbg !8952
  %_0.i2763 = fadd float %filter_far.i.i524.i.sroa.14.011078, %_0.i2764, !dbg !8954
  %730 = tail call noundef float @llvm.fabs.f32(float %_0.i2763), !dbg !8956
  %731 = fcmp uge float %730, 0x3BC79CA100000000, !dbg !8959
  %_0.i4340 = select i1 %731, float %_0.i2763, float 0.000000e+00, !dbg !8961
  %_0.i3434 = fsub float %_0.i2924, %_0.i2767, !dbg !8962
  %_311.1.i.i588.i = load i64, ptr %682, align 8, !dbg !8964, !noalias !8783, !noundef !11
  %_234.i.i589.i = icmp ugt i64 %position.sroa.0.0.i.i567.i11083, %_311.1.i.i588.i, !dbg !8966
  br i1 %_234.i.i589.i, label %bb78.i.i719.i, label %bb79.i.i590.i, !dbg !8966, !prof !1664

bb79.i.i590.i:                                    ; preds = %bb56.i.i573.i
  %_311.0.i.i591.i = load ptr, ptr %631, align 8, !dbg !8964, !noalias !8783, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8972), !dbg !8975
  %_4.not.i4166 = icmp eq i64 %_311.1.i.i588.i, %position.sroa.0.0.i.i567.i11083, !dbg !8976
  br i1 %_4.not.i4166, label %panic.i4168, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4169, !dbg !8976

panic.i4168:                                      ; preds = %bb79.i.i590.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !8976, !noalias !8978
  unreachable, !dbg !8976

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4169: ; preds = %bb79.i.i590.i
  %_241.i.i594.i = getelementptr inbounds nuw float, ptr %_311.0.i.i591.i, i64 %position.sroa.0.0.i.i567.i11083, !dbg !8979
  store float %_0.i2779, ptr %_241.i.i594.i, align 4, !dbg !8976, !alias.scope !8972, !noalias !8783
  %_312.1.i.i595.i = load i64, ptr %684, align 8, !dbg !8985, !noalias !8783, !noundef !11
  %_242.i.i596.i = icmp ugt i64 %position.sroa.0.0.i.i567.i11083, %_312.1.i.i595.i, !dbg !8986
  br i1 %_242.i.i596.i, label %bb80.i.i718.i, label %bb81.i.i597.i, !dbg !8986, !prof !1664

bb78.i.i719.i:                                    ; preds = %bb56.i.i573.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i567.i11083, i64 noundef %_311.1.i.i588.i, i64 noundef %_311.1.i.i588.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7515f6adc8a5521b72f04a3abb372e72) #26, !dbg !8990, !noalias !8783
  unreachable, !dbg !8990

bb81.i.i597.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4169
  %_312.0.i.i598.i = load ptr, ptr %683, align 8, !dbg !8985, !noalias !8783, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8991), !dbg !8994
  %_4.not.i4162 = icmp eq i64 %_312.1.i.i595.i, %position.sroa.0.0.i.i567.i11083, !dbg !8995
  br i1 %_4.not.i4162, label %panic.i4164, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4165, !dbg !8995

panic.i4164:                                      ; preds = %bb81.i.i597.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !8995, !noalias !8997
  unreachable, !dbg !8995

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4165: ; preds = %bb81.i.i597.i
  %_249.i.i600.i = getelementptr inbounds nuw float, ptr %_312.0.i.i598.i, i64 %position.sroa.0.0.i.i567.i11083, !dbg !8998
  store float %_0.i3433, ptr %_249.i.i600.i, align 4, !dbg !8995, !alias.scope !8991, !noalias !8783
  %_313.1.i.i601.i = load i64, ptr %685, align 8, !dbg !9003, !noalias !8783, !noundef !11
  %_250.i.i602.i = icmp ugt i64 %position.sroa.0.0.i.i567.i11083, %_313.1.i.i601.i, !dbg !9004
  br i1 %_250.i.i602.i, label %bb82.i.i717.i, label %bb83.i.i603.i, !dbg !9004, !prof !1664

bb80.i.i718.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4169
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i567.i11083, i64 noundef %_312.1.i.i595.i, i64 noundef %_312.1.i.i595.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8c2aade3368450b16e7e4997ad3fca81) #26, !dbg !9008, !noalias !8783
  unreachable, !dbg !9008

bb83.i.i603.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4165
  %_313.0.i.i604.i = load ptr, ptr %_18.i542.i, align 8, !dbg !9003, !noalias !8783, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9009), !dbg !9012
  %_4.not.i4158 = icmp eq i64 %_313.1.i.i601.i, %position.sroa.0.0.i.i567.i11083, !dbg !9013
  br i1 %_4.not.i4158, label %panic.i4160, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4161, !dbg !9013

panic.i4160:                                      ; preds = %bb83.i.i603.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !9013, !noalias !9015
  unreachable, !dbg !9013

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4161: ; preds = %bb83.i.i603.i
  %_257.i.i606.i = getelementptr inbounds nuw float, ptr %_313.0.i.i604.i, i64 %position.sroa.0.0.i.i567.i11083, !dbg !9016
  store float %_0.i2767, ptr %_257.i.i606.i, align 4, !dbg !9013, !alias.scope !9009, !noalias !8783
  %_314.1.i.i607.i = load i64, ptr %687, align 8, !dbg !9021, !noalias !8783, !noundef !11
  %_258.i.i608.i = icmp ugt i64 %position.sroa.0.0.i.i567.i11083, %_314.1.i.i607.i, !dbg !9022
  br i1 %_258.i.i608.i, label %bb84.i.i716.i, label %bb85.i.i609.i, !dbg !9022, !prof !1664

bb82.i.i717.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4165
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i567.i11083, i64 noundef %_313.1.i.i601.i, i64 noundef %_313.1.i.i601.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a4c3da5a99763e9452b49d42852d67e3) #26, !dbg !9026, !noalias !8783
  unreachable, !dbg !9026

bb85.i.i609.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4161
  %_314.0.i.i610.i = load ptr, ptr %686, align 8, !dbg !9021, !noalias !8783, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9027), !dbg !9030
  %_4.not.i4154 = icmp eq i64 %_314.1.i.i607.i, %position.sroa.0.0.i.i567.i11083, !dbg !9031
  br i1 %_4.not.i4154, label %panic.i4156, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4157, !dbg !9031

panic.i4156:                                      ; preds = %bb85.i.i609.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !9031, !noalias !9033
  unreachable, !dbg !9031

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4157: ; preds = %bb85.i.i609.i
  %_265.i.i612.i = getelementptr inbounds nuw float, ptr %_314.0.i.i610.i, i64 %position.sroa.0.0.i.i567.i11083, !dbg !9034
  store float %_0.i3434, ptr %_265.i.i612.i, align 4, !dbg !9031, !alias.scope !9027, !noalias !8783
  %_315.0.i.i613.i = load ptr, ptr %631, align 8, !dbg !9039, !noalias !8783, !nonnull !11, !noundef !11
  %_315.1.i.i614.i = load i64, ptr %682, align 8, !dbg !9039, !noalias !8783, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9040), !dbg !9043
  %_8.i212.i615.i = load i64, ptr %_24.i.i562.i, align 8, !dbg !9044, !alias.scope !9040, !noalias !9046, !noundef !11
  %_7.i213.i616.i = add i64 %_8.i212.i615.i, %position.sroa.0.0.i.i567.i11083, !dbg !9048
  %_27.not.i214.i617.i = icmp ult i64 %_7.i213.i616.i, %ring_len.i534.i, !dbg !9049
  %732 = select i1 %_27.not.i214.i617.i, i64 0, i64 %ring_len.i534.i, !dbg !9049
  %row.sroa.0.0.i215.i618.i = sub nuw i64 %_7.i213.i616.i, %732, !dbg !9049
  %_28.i216.i619.i = icmp ugt i64 %row.sroa.0.0.i215.i618.i, %_315.1.i.i614.i, !dbg !9051
  br i1 %_28.i216.i619.i, label %bb14.i219.i715.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i620.i, !dbg !9051, !prof !1664

bb14.i219.i715.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4157
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i215.i618.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i.i614.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i.i614.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !9054, !noalias !9055
  unreachable, !dbg !9054

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i620.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4157
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9056), !dbg !9059
  %_3.not.i3792 = icmp eq i64 %_315.1.i.i614.i, %row.sroa.0.0.i215.i618.i, !dbg !9060
  br i1 %_3.not.i3792, label %panic.i3795, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3796, !dbg !9060

panic.i3795:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i620.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9060, !noalias !9062
  unreachable, !dbg !9060

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3796: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit220.i620.i
  %_35.i218.i622.i = getelementptr inbounds nuw float, ptr %_315.0.i.i613.i, i64 %row.sroa.0.0.i215.i618.i, !dbg !9063
  %_0.i3794 = load float, ptr %_35.i218.i622.i, align 4, !dbg !9060, !alias.scope !9056, !noalias !9065, !noundef !11
  %_316.0.i.i623.i = load ptr, ptr %683, align 8, !dbg !9066, !noalias !8783, !nonnull !11, !noundef !11
  %_316.1.i.i624.i = load i64, ptr %684, align 8, !dbg !9066, !noalias !8783, !noundef !11
  %_28.i207.i629.i = icmp ugt i64 %row.sroa.0.0.i215.i618.i, %_316.1.i.i624.i, !dbg !9068
  br i1 %_28.i207.i629.i, label %bb14.i210.i714.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i630.i, !dbg !9068, !prof !1664

bb14.i210.i714.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3796
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i215.i618.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i.i624.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i.i624.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !9072, !noalias !9073
  unreachable, !dbg !9072

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i630.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3796
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9077), !dbg !9080
  %_3.not.i3787 = icmp eq i64 %_316.1.i.i624.i, %row.sroa.0.0.i215.i618.i, !dbg !9081
  br i1 %_3.not.i3787, label %panic.i3790, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3791, !dbg !9081

panic.i3790:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i630.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9081, !noalias !9083
  unreachable, !dbg !9081

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3791: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit211.i630.i
  %_35.i209.i632.i = getelementptr inbounds nuw float, ptr %_316.0.i.i623.i, i64 %row.sroa.0.0.i215.i618.i, !dbg !9084
  %_0.i3789 = load float, ptr %_35.i209.i632.i, align 4, !dbg !9081, !alias.scope !9077, !noalias !9086, !noundef !11
  %_317.0.i.i633.i = load ptr, ptr %_18.i542.i, align 8, !dbg !9087, !noalias !8783, !nonnull !11, !noundef !11
  %_317.1.i.i634.i = load i64, ptr %685, align 8, !dbg !9087, !noalias !8783, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9089), !dbg !9092
  %_8.i194.i635.i = load i64, ptr %_26.i.i564.i, align 8, !dbg !9093, !alias.scope !9089, !noalias !9095, !noundef !11
  %_7.i195.i636.i = add i64 %_8.i194.i635.i, %position.sroa.0.0.i.i567.i11083, !dbg !9097
  %_27.not.i196.i637.i = icmp ult i64 %_7.i195.i636.i, %ring_len.i534.i, !dbg !9098
  %733 = select i1 %_27.not.i196.i637.i, i64 0, i64 %ring_len.i534.i, !dbg !9098
  %row.sroa.0.0.i197.i638.i = sub nuw i64 %_7.i195.i636.i, %733, !dbg !9098
  %_28.i198.i639.i = icmp ugt i64 %row.sroa.0.0.i197.i638.i, %_317.1.i.i634.i, !dbg !9100
  br i1 %_28.i198.i639.i, label %bb14.i201.i713.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i640.i, !dbg !9100, !prof !1664

bb14.i201.i713.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3791
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i197.i638.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i.i634.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i.i634.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !9103, !noalias !9104
  unreachable, !dbg !9103

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i640.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3791
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9105), !dbg !9108
  %_3.not.i3782 = icmp eq i64 %_317.1.i.i634.i, %row.sroa.0.0.i197.i638.i, !dbg !9109
  br i1 %_3.not.i3782, label %panic.i3785, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3786, !dbg !9109

panic.i3785:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i640.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9109, !noalias !9111
  unreachable, !dbg !9109

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3786: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit202.i640.i
  %_35.i200.i642.i = getelementptr inbounds nuw float, ptr %_317.0.i.i633.i, i64 %row.sroa.0.0.i197.i638.i, !dbg !9112
  %_0.i3784 = load float, ptr %_35.i200.i642.i, align 4, !dbg !9109, !alias.scope !9105, !noalias !9114, !noundef !11
  %_318.0.i.i643.i = load ptr, ptr %686, align 8, !dbg !9115, !noalias !8783, !nonnull !11, !noundef !11
  %_318.1.i.i644.i = load i64, ptr %687, align 8, !dbg !9115, !noalias !8783, !noundef !11
  %_28.i189.i649.i = icmp ugt i64 %row.sroa.0.0.i197.i638.i, %_318.1.i.i644.i, !dbg !9117
  br i1 %_28.i189.i649.i, label %bb14.i192.i712.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i650.i, !dbg !9117, !prof !1664

bb14.i192.i712.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3786
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i197.i638.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i.i644.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i.i644.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !9121, !noalias !9122
  unreachable, !dbg !9121

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i650.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3786
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9126), !dbg !9129
  %_3.not.i3777 = icmp eq i64 %_318.1.i.i644.i, %row.sroa.0.0.i197.i638.i, !dbg !9130
  br i1 %_3.not.i3777, label %panic.i3780, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3781, !dbg !9130

panic.i3780:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i650.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9130, !noalias !9132
  unreachable, !dbg !9130

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3781: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit193.i650.i
  %_35.i191.i652.i = getelementptr inbounds nuw float, ptr %_318.0.i.i643.i, i64 %row.sroa.0.0.i197.i638.i, !dbg !9133
  %_0.i3779 = load float, ptr %_35.i191.i652.i, align 4, !dbg !9130, !alias.scope !9126, !noalias !9135, !noundef !11
  %734 = tail call noundef float @llvm.fabs.f32(float %_0.i3794), !dbg !9136
  %735 = tail call noundef float @llvm.fabs.f32(float %_0.i3784), !dbg !9141
  %_0.i3210 = fmul float %734, 5.000000e-01, !dbg !9143
  %_0.i3209 = fmul float %735, 5.000000e-01, !dbg !9147
  %_0.i2633 = fadd float %_0.i3210, %_0.i3209, !dbg !9149
  %_3.i.i5916 = fcmp ule float %_0.i2633, 0x3E45798EE0000000, !dbg !9151
  %_6.i.i5918 = bitcast float %_0.i2633 to i32, !dbg !9157
  %_4.i.i5922 = select i1 %_3.i.i5916, i32 841731191, i32 %_6.i.i5918, !dbg !9160
  %_0.i.i5923 = bitcast i32 %_4.i.i5922 to float, !dbg !9161
  %_3.i.i = fcmp ule float %_0.i.i5923, 0x3810000000000000, !dbg !9163
  %_4.i.i = select i1 %_3.i.i, i32 8388608, i32 %_4.i.i5922, !dbg !9168
  %_5.i3807 = and i32 %_4.i.i, 8388607, !dbg !9170
  %_4.i3808 = or disjoint i32 %_5.i3807, 1065353216, !dbg !9170
  %significand.i = bitcast i32 %_4.i3808 to float, !dbg !9172
  %_0.i3337 = fadd float %significand.i, -1.000000e+00, !dbg !9174
  %_0.i2994 = fmul float %_0.i3337, 0xBF9B17A960000000, !dbg !9176
  %_0.i2514 = fadd float %_0.i2994, 0x3FBF9A8440000000, !dbg !9178
  %_0.i2994.1 = fmul float %_0.i3337, %_0.i2514, !dbg !9176
  %_0.i2514.1 = fadd float %_0.i2994.1, 0xBFD1E3F400000000, !dbg !9178
  %_0.i2994.2 = fmul float %_0.i3337, %_0.i2514.1, !dbg !9176
  %_0.i2514.2 = fadd float %_0.i2994.2, 0x3FDD544F20000000, !dbg !9178
  %_0.i2994.3 = fmul float %_0.i3337, %_0.i2514.2, !dbg !9176
  %_0.i2514.3 = fadd float %_0.i2994.3, 0xBFE6FC2A60000000, !dbg !9178
  %_0.i2994.4 = fmul float %_0.i3337, %_0.i2514.3, !dbg !9176
  %_0.i2514.4 = fadd float %_0.i2994.4, 0x3FF714B2A0000000, !dbg !9178
  %_9.i3809 = lshr i32 %_4.i.i, 23, !dbg !9180
  %_8.i3810 = or disjoint i32 %_9.i3809, 1258291200, !dbg !9180
  %_7.i3811 = bitcast i32 %_8.i3810 to float, !dbg !9181
  %exponent.i = fadd float %_7.i3811, 0xC160000FE0000000, !dbg !9183
  %_0.i2993 = fmul float %_0.i3337, %_0.i2514.4, !dbg !9184
  %_0.i2513 = fadd float %exponent.i, %_0.i2993, !dbg !9186
  %_0.i3336 = fmul float %_0.i2513, 0x4018151820000000, !dbg !9188
  %_3.i.i5908.inv = fcmp ogt float %_0.i3336, -1.600000e+02, !dbg !9190
  %_0.i.i5915 = select i1 %_3.i.i5908.inv, float %_0.i3336, float -1.600000e+02, !dbg !9190
  %_3.i.i6492.inv = fcmp olt float %_0.i.i5915, 2.400000e+01, !dbg !9193
  %_0.i.i6499 = select i1 %_3.i.i6492.inv, float %_0.i.i5915, float 2.400000e+01, !dbg !9193
  %_0.i3385 = fsub float %_0.i.i6499, %_12.le.i6860, !dbg !9196
  %_3.i2226 = fcmp ule float %_0.i3385, 3.000000e+00, !dbg !9199
  %_0.i2609 = fadd float %_0.i3385, 3.000000e+00, !dbg !9201
  %_0.i3115 = fmul float %_0.i2609, %_0.i2609, !dbg !9203
  %_0.i3114 = fmul float %_0.i3115, 0x3FB5555560000000, !dbg !9205
  %_4.i4468.v.v = select i1 %_3.i2226, float %_0.i3114, float %_0.i3385, !dbg !9207
  %_4.i4468.v = fmul float %coefficients.i529.i.sroa.0.0.copyload, %_4.i4468.v.v, !dbg !9207
  %_4.i4468 = bitcast float %_4.i4468.v to i32, !dbg !9207
  %736 = fcmp ugt float %_0.i3385, -3.000000e+00, !dbg !9209
  %_7.i4460 = select i1 %736, i32 %_4.i4468, i32 0, !dbg !9211
  %_0.i4462 = bitcast i32 %_7.i4460 to float, !dbg !9212
  %_3.i.i5900 = fcmp ule float %_0.i4462, -1.000000e+02, !dbg !9214
  %737 = bitcast i32 %_7.i4460 to float, !dbg !9217
  %_0.i.i5907 = select i1 %_3.i.i5900, float -1.000000e+02, float %737, !dbg !9220
  %_3.i.i6484 = fcmp olt float %_0.i.i5907, 0.000000e+00, !dbg !9221
  %_0.i.i6491 = select i1 %_3.i.i6484, float %_0.i.i5907, float 0.000000e+00, !dbg !9224
  %738 = bitcast i32 %gain_near.i.i523.i.sroa.0.011079 to float, !dbg !9226
  %_3.i2511 = fcmp uge float %_0.i.i6491, %738, !dbg !9227
  %_4.i4934.v = select i1 %_3.i2511, float %coefficients.i529.i.sroa.7.0.copyload, float %coefficients.i529.i.sroa.5.0.copyload, !dbg !9230
  %_0.i3468 = fsub float %738, %_0.i.i6491, !dbg !9232
  %_4.i2991 = fmul float %_0.i3468, %_4.i4934.v, !dbg !9234
  %_0.i2992 = fadd float %_0.i.i6491, %_4.i2991, !dbg !9234
  %739 = tail call noundef float @llvm.fabs.f32(float %_0.i2992), !dbg !9236
  %_4.i4453 = bitcast float %_0.i2992 to i32, !dbg !9239
  %740 = fcmp uge float %739, 0x3BC79CA100000000, !dbg !9242
  %_3.i4455 = select i1 %740, i32 %_4.i4453, i32 0, !dbg !9243
  %_0.i4456 = bitcast i32 %_3.i4455 to float, !dbg !9244
  %_0.i2824 = fadd float %_12.le.4.i6868, %_0.i4456, !dbg !9246
  %_0.i3335 = fmul float %_0.i2824, 0x3FC542A5A0000000, !dbg !9248
  %_3.i.i5122.inv = fcmp ogt float %_0.i3335, -1.260000e+02, !dbg !9251
  %_0.i.i5129 = select i1 %_3.i.i5122.inv, float %_0.i3335, float -1.260000e+02, !dbg !9251
  %_3.i.i5924.inv = fcmp olt float %_0.i.i5129, 1.270000e+02, !dbg !9255
  %_0.i.i5931 = select i1 %_3.i.i5924.inv, float %_0.i.i5129, float 1.270000e+02, !dbg !9255
  %741 = tail call noundef float @llvm.floor.f32(float %_0.i.i5931), !dbg !9258
  %_0.i3361 = fsub float %_0.i.i5931, %741, !dbg !9262
  %742 = tail call noundef float @llvm.fabs.f32(float %_0.i3789), !dbg !9264
  %743 = tail call noundef float @llvm.fabs.f32(float %_0.i3779), !dbg !9267
  %_0.i3212 = fmul float %742, 5.000000e-01, !dbg !9269
  %_0.i3211 = fmul float %743, 5.000000e-01, !dbg !9271
  %_0.i2634 = fadd float %_0.i3212, %_0.i3211, !dbg !9273
  %_3.i.i5892 = fcmp ule float %_0.i2634, 0x3E45798EE0000000, !dbg !9275
  %_6.i.i5894 = bitcast float %_0.i2634 to i32, !dbg !9280
  %_4.i.i5898 = select i1 %_3.i.i5892, i32 841731191, i32 %_6.i.i5894, !dbg !9283
  %_0.i.i5899 = bitcast i32 %_4.i.i5898 to float, !dbg !9284
  %_3.i.i4938 = fcmp ule float %_0.i.i5899, 0x3810000000000000, !dbg !9286
  %_4.i.i4944 = select i1 %_3.i.i4938, i32 8388608, i32 %_4.i.i5898, !dbg !9291
  %_5.i3813 = and i32 %_4.i.i4944, 8388607, !dbg !9293
  %_4.i3814 = or disjoint i32 %_5.i3813, 1065353216, !dbg !9293
  %significand.i3815 = bitcast i32 %_4.i3814 to float, !dbg !9295
  %_0.i3338 = fadd float %significand.i3815, -1.000000e+00, !dbg !9297
  %_0.i2996 = fmul float %_0.i3338, 0xBF9B17A960000000, !dbg !9299
  %_0.i2516 = fadd float %_0.i2996, 0x3FBF9A8440000000, !dbg !9301
  %_0.i2996.1 = fmul float %_0.i3338, %_0.i2516, !dbg !9299
  %_0.i2516.1 = fadd float %_0.i2996.1, 0xBFD1E3F400000000, !dbg !9301
  %_0.i2996.2 = fmul float %_0.i3338, %_0.i2516.1, !dbg !9299
  %_0.i2516.2 = fadd float %_0.i2996.2, 0x3FDD544F20000000, !dbg !9301
  %_0.i2996.3 = fmul float %_0.i3338, %_0.i2516.2, !dbg !9299
  %_0.i2516.3 = fadd float %_0.i2996.3, 0xBFE6FC2A60000000, !dbg !9301
  %_0.i2996.4 = fmul float %_0.i3338, %_0.i2516.3, !dbg !9299
  %_0.i2516.4 = fadd float %_0.i2996.4, 0x3FF714B2A0000000, !dbg !9301
  %_9.i3816 = lshr i32 %_4.i.i4944, 23, !dbg !9303
  %_8.i3817 = or disjoint i32 %_9.i3816, 1258291200, !dbg !9303
  %_7.i3818 = bitcast i32 %_8.i3817 to float, !dbg !9304
  %exponent.i3819 = fadd float %_7.i3818, 0xC160000FE0000000, !dbg !9306
  %_0.i2995 = fmul float %_0.i3338, %_0.i2516.4, !dbg !9307
  %_0.i2515 = fadd float %exponent.i3819, %_0.i2995, !dbg !9309
  %_0.i3334 = fmul float %_0.i2515, 0x4018151820000000, !dbg !9311
  %_3.i.i5884.inv = fcmp ogt float %_0.i3334, -1.600000e+02, !dbg !9313
  %_0.i.i5891 = select i1 %_3.i.i5884.inv, float %_0.i3334, float -1.600000e+02, !dbg !9313
  %_3.i.i6476.inv = fcmp olt float %_0.i.i5891, 2.400000e+01, !dbg !9316
  %_0.i.i6483 = select i1 %_3.i.i6476.inv, float %_0.i.i5891, float 2.400000e+01, !dbg !9316
  %_0.i3386 = fsub float %_0.i.i6483, %_12.le.5.i6870, !dbg !9319
  %_3.i2227 = fcmp ule float %_0.i3386, 3.000000e+00, !dbg !9322
  %_0.i2610 = fadd float %_0.i3386, 3.000000e+00, !dbg !9324
  %_0.i3119 = fmul float %_0.i2610, %_0.i2610, !dbg !9326
  %_0.i3118 = fmul float %_0.i3119, 0x3FB5555560000000, !dbg !9328
  %_4.i4481.v.v = select i1 %_3.i2227, float %_0.i3118, float %_0.i3386, !dbg !9330
  %_4.i4481.v = fmul float %coefficients.i529.i.sroa.9.0.copyload, %_4.i4481.v.v, !dbg !9330
  %_4.i4481 = bitcast float %_4.i4481.v to i32, !dbg !9330
  %744 = fcmp ugt float %_0.i3386, -3.000000e+00, !dbg !9332
  %_7.i4473 = select i1 %744, i32 %_4.i4481, i32 0, !dbg !9334
  %_0.i4475 = bitcast i32 %_7.i4473 to float, !dbg !9335
  %_3.i.i5876 = fcmp ule float %_0.i4475, -1.000000e+02, !dbg !9337
  %745 = bitcast i32 %_7.i4473 to float, !dbg !9340
  %_0.i.i5883 = select i1 %_3.i.i5876, float -1.000000e+02, float %745, !dbg !9343
  %_3.i.i6468 = fcmp olt float %_0.i.i5883, 0.000000e+00, !dbg !9344
  %_0.i.i6475 = select i1 %_3.i.i6468, float %_0.i.i5883, float 0.000000e+00, !dbg !9347
  %746 = bitcast i32 %gain_near.i.i523.i.sroa.6.011080 to float, !dbg !9349
  %_3.i2507 = fcmp uge float %_0.i.i6475, %746, !dbg !9350
  %_4.i4927.v = select i1 %_3.i2507, float %coefficients.i529.i.sroa.13.0.copyload, float %coefficients.i529.i.sroa.11.0.copyload, !dbg !9353
  %_0.i3467 = fsub float %746, %_0.i.i6475, !dbg !9355
  %_4.i2989 = fmul float %_0.i3467, %_4.i4927.v, !dbg !9357
  %_0.i2990 = fadd float %_0.i.i6475, %_4.i2989, !dbg !9357
  %747 = tail call noundef float @llvm.fabs.f32(float %_0.i2990), !dbg !9359
  %_4.i4449 = bitcast float %_0.i2990 to i32, !dbg !9362
  %748 = fcmp uge float %747, 0x3BC79CA100000000, !dbg !9365
  %_3.i4451 = select i1 %748, i32 %_4.i4449, i32 0, !dbg !9366
  %_0.i4452 = bitcast i32 %_3.i4451 to float, !dbg !9367
  %_0.i2823 = fadd float %_12.le.9.i6878, %_0.i4452, !dbg !9369
  %_0.i3333 = fmul float %_0.i2823, 0x3FC542A5A0000000, !dbg !9371
  %_3.i.i5130.inv = fcmp ogt float %_0.i3333, -1.260000e+02, !dbg !9374
  %_0.i.i5137 = select i1 %_3.i.i5130.inv, float %_0.i3333, float -1.260000e+02, !dbg !9374
  %_3.i.i5932.inv = fcmp olt float %_0.i.i5137, 1.270000e+02, !dbg !9378
  %_0.i.i5939 = select i1 %_3.i.i5932.inv, float %_0.i.i5137, float 1.270000e+02, !dbg !9378
  %749 = tail call noundef float @llvm.floor.f32(float %_0.i.i5939), !dbg !9381
  %_0.i3362 = fsub float %_0.i.i5939, %749, !dbg !9385
  %_0.i3387 = fsub float %_0.i.i6499, %_12.le.i6898, !dbg !9387
  %_3.i2229 = fcmp ule float %_0.i3387, 3.000000e+00, !dbg !9392
  %_0.i2611 = fadd float %_0.i3387, 3.000000e+00, !dbg !9394
  %_0.i3123 = fmul float %_0.i2611, %_0.i2611, !dbg !9396
  %_0.i3122 = fmul float %_0.i3123, 0x3FB5555560000000, !dbg !9398
  %_4.i4494.v.v = select i1 %_3.i2229, float %_0.i3122, float %_0.i3387, !dbg !9400
  %_4.i4494.v = fmul float %coefficients.i529.i.sroa.15.24.copyload, %_4.i4494.v.v, !dbg !9400
  %_4.i4494 = bitcast float %_4.i4494.v to i32, !dbg !9400
  %750 = fcmp ugt float %_0.i3387, -3.000000e+00, !dbg !9402
  %_7.i4486 = select i1 %750, i32 %_4.i4494, i32 0, !dbg !9404
  %_0.i4488 = bitcast i32 %_7.i4486 to float, !dbg !9405
  %_3.i.i5852 = fcmp ule float %_0.i4488, -1.000000e+02, !dbg !9407
  %751 = bitcast i32 %_7.i4486 to float, !dbg !9410
  %_0.i.i5859 = select i1 %_3.i.i5852, float -1.000000e+02, float %751, !dbg !9413
  %_3.i.i6452 = fcmp olt float %_0.i.i5859, 0.000000e+00, !dbg !9414
  %_0.i.i6459 = select i1 %_3.i.i6452, float %_0.i.i5859, float 0.000000e+00, !dbg !9417
  %752 = bitcast i32 %gain_far.i.i522.i.sroa.0.011081 to float, !dbg !9419
  %_3.i2503 = fcmp uge float %_0.i.i6459, %752, !dbg !9420
  %_4.i4920.v = select i1 %_3.i2503, float %coefficients.i529.i.sroa.20.24.copyload, float %coefficients.i529.i.sroa.18.24.copyload, !dbg !9423
  %_0.i3466 = fsub float %752, %_0.i.i6459, !dbg !9425
  %_4.i2987 = fmul float %_0.i3466, %_4.i4920.v, !dbg !9427
  %_0.i2988 = fadd float %_0.i.i6459, %_4.i2987, !dbg !9427
  %753 = tail call noundef float @llvm.fabs.f32(float %_0.i2988), !dbg !9429
  %_4.i4445 = bitcast float %_0.i2988 to i32, !dbg !9432
  %754 = fcmp uge float %753, 0x3BC79CA100000000, !dbg !9435
  %_3.i4447 = select i1 %754, i32 %_4.i4445, i32 0, !dbg !9436
  %_0.i4448 = bitcast i32 %_3.i4447 to float, !dbg !9437
  %_0.i2822 = fadd float %_12.le.4.i6906, %_0.i4448, !dbg !9439
  %_0.i3331 = fmul float %_0.i2822, 0x3FC542A5A0000000, !dbg !9441
  %_3.i.i5138.inv = fcmp ogt float %_0.i3331, -1.260000e+02, !dbg !9444
  %_0.i.i5145 = select i1 %_3.i.i5138.inv, float %_0.i3331, float -1.260000e+02, !dbg !9444
  %_3.i.i5940.inv = fcmp olt float %_0.i.i5145, 1.270000e+02, !dbg !9448
  %_0.i.i5947 = select i1 %_3.i.i5940.inv, float %_0.i.i5145, float 1.270000e+02, !dbg !9448
  %755 = tail call noundef float @llvm.floor.f32(float %_0.i.i5947), !dbg !9451
  %_0.i3363 = fsub float %_0.i.i5947, %755, !dbg !9455
  %_0.i3388 = fsub float %_0.i.i6483, %_12.le.5.i6908, !dbg !9457
  %_3.i2231 = fcmp ule float %_0.i3388, 3.000000e+00, !dbg !9462
  %_0.i2612 = fadd float %_0.i3388, 3.000000e+00, !dbg !9464
  %_0.i3127 = fmul float %_0.i2612, %_0.i2612, !dbg !9466
  %_0.i3126 = fmul float %_0.i3127, 0x3FB5555560000000, !dbg !9468
  %_4.i4507.v.v = select i1 %_3.i2231, float %_0.i3126, float %_0.i3388, !dbg !9470
  %_4.i4507.v = fmul float %coefficients.i529.i.sroa.22.24.copyload, %_4.i4507.v.v, !dbg !9470
  %_4.i4507 = bitcast float %_4.i4507.v to i32, !dbg !9470
  %756 = fcmp ugt float %_0.i3388, -3.000000e+00, !dbg !9472
  %_7.i4499 = select i1 %756, i32 %_4.i4507, i32 0, !dbg !9474
  %_0.i4501 = bitcast i32 %_7.i4499 to float, !dbg !9475
  %_3.i.i5828 = fcmp ule float %_0.i4501, -1.000000e+02, !dbg !9477
  %757 = bitcast i32 %_7.i4499 to float, !dbg !9480
  %_0.i.i5835 = select i1 %_3.i.i5828, float -1.000000e+02, float %757, !dbg !9483
  %_3.i.i6436 = fcmp olt float %_0.i.i5835, 0.000000e+00, !dbg !9484
  %_0.i.i6443 = select i1 %_3.i.i6436, float %_0.i.i5835, float 0.000000e+00, !dbg !9487
  %758 = bitcast i32 %gain_far.i.i522.i.sroa.6.011082 to float, !dbg !9489
  %_3.i2499 = fcmp uge float %_0.i.i6443, %758, !dbg !9490
  %_4.i4913.v = select i1 %_3.i2499, float %coefficients.i529.i.sroa.26.24.copyload, float %coefficients.i529.i.sroa.24.24.copyload, !dbg !9493
  %_0.i3465 = fsub float %758, %_0.i.i6443, !dbg !9495
  %_4.i2985 = fmul float %_0.i3465, %_4.i4913.v, !dbg !9497
  %_0.i2986 = fadd float %_0.i.i6443, %_4.i2985, !dbg !9497
  %759 = tail call noundef float @llvm.fabs.f32(float %_0.i2986), !dbg !9499
  %_4.i4441 = bitcast float %_0.i2986 to i32, !dbg !9502
  %760 = fcmp uge float %759, 0x3BC79CA100000000, !dbg !9505
  %_3.i4443 = select i1 %760, i32 %_4.i4441, i32 0, !dbg !9506
  %_0.i4444 = bitcast i32 %_3.i4443 to float, !dbg !9507
  %_0.i2821 = fadd float %_12.le.9.i6916, %_0.i4444, !dbg !9509
  %_0.i3329 = fmul float %_0.i2821, 0x3FC542A5A0000000, !dbg !9511
  %_3.i.i5146.inv = fcmp ogt float %_0.i3329, -1.260000e+02, !dbg !9514
  %_0.i.i5153 = select i1 %_3.i.i5146.inv, float %_0.i3329, float -1.260000e+02, !dbg !9514
  %_3.i.i5948.inv = fcmp olt float %_0.i.i5153, 1.270000e+02, !dbg !9518
  %_0.i.i5955 = select i1 %_3.i.i5948.inv, float %_0.i.i5153, float 1.270000e+02, !dbg !9518
  %761 = tail call noundef float @llvm.floor.f32(float %_0.i.i5955), !dbg !9521
  %_0.i3364 = fsub float %_0.i.i5955, %761, !dbg !9525
  %_0.i3052 = fmul float %_0.i3364, 0x3F5E974FA0000000, !dbg !9527
  %_0.i2568 = fadd float %_0.i3052, 0x3F82778560000000, !dbg !9529
  %_0.i3052.1 = fmul float %_0.i3364, %_0.i2568, !dbg !9527
  %_0.i2568.1 = fadd float %_0.i3052.1, 0x3FAC91CE60000000, !dbg !9529
  %_0.i3052.2 = fmul float %_0.i3364, %_0.i2568.1, !dbg !9527
  %_0.i2568.2 = fadd float %_0.i3052.2, 0x3FCEBDB560000000, !dbg !9529
  %_0.i3052.3 = fmul float %_0.i3364, %_0.i2568.2, !dbg !9527
  %_0.i2568.3 = fadd float %_0.i3052.3, 0x3FE62E4BA0000000, !dbg !9529
  %_0.i3049 = fmul float %_0.i3363, 0x3F5E974FA0000000, !dbg !9531
  %_0.i2566 = fadd float %_0.i3049, 0x3F82778560000000, !dbg !9533
  %_0.i3049.1 = fmul float %_0.i3363, %_0.i2566, !dbg !9531
  %_0.i2566.1 = fadd float %_0.i3049.1, 0x3FAC91CE60000000, !dbg !9533
  %_0.i3049.2 = fmul float %_0.i3363, %_0.i2566.1, !dbg !9531
  %_0.i2566.2 = fadd float %_0.i3049.2, 0x3FCEBDB560000000, !dbg !9533
  %_0.i3049.3 = fmul float %_0.i3363, %_0.i2566.2, !dbg !9531
  %_0.i2566.3 = fadd float %_0.i3049.3, 0x3FE62E4BA0000000, !dbg !9533
  %_0.i3046 = fmul float %_0.i3362, 0x3F5E974FA0000000, !dbg !9535
  %_0.i2564 = fadd float %_0.i3046, 0x3F82778560000000, !dbg !9537
  %_0.i3046.1 = fmul float %_0.i3362, %_0.i2564, !dbg !9535
  %_0.i2564.1 = fadd float %_0.i3046.1, 0x3FAC91CE60000000, !dbg !9537
  %_0.i3046.2 = fmul float %_0.i3362, %_0.i2564.1, !dbg !9535
  %_0.i2564.2 = fadd float %_0.i3046.2, 0x3FCEBDB560000000, !dbg !9537
  %_0.i3046.3 = fmul float %_0.i3362, %_0.i2564.2, !dbg !9535
  %_0.i2564.3 = fadd float %_0.i3046.3, 0x3FE62E4BA0000000, !dbg !9537
  %_0.i3043 = fmul float %_0.i3361, 0x3F5E974FA0000000, !dbg !9539
  %_0.i2562 = fadd float %_0.i3043, 0x3F82778560000000, !dbg !9541
  %_0.i3043.1 = fmul float %_0.i3361, %_0.i2562, !dbg !9539
  %_0.i2562.1 = fadd float %_0.i3043.1, 0x3FAC91CE60000000, !dbg !9541
  %_0.i3043.2 = fmul float %_0.i3361, %_0.i2562.1, !dbg !9539
  %_0.i2562.2 = fadd float %_0.i3043.2, 0x3FCEBDB560000000, !dbg !9541
  %_0.i3043.3 = fmul float %_0.i3361, %_0.i2562.2, !dbg !9539
  %_0.i2562.3 = fadd float %_0.i3043.3, 0x3FE62E4BA0000000, !dbg !9541
  %_0.i3042 = fmul float %_0.i3361, %_0.i2562.3, !dbg !9543
  %_0.i2561 = fadd float %_0.i3042, 1.000000e+00, !dbg !9545
  %biased.i = fadd float %741, 0x4160000FE0000000, !dbg !9547
  %_4.i2131 = bitcast float %biased.i to i32, !dbg !9549
  %_3.i2132 = shl i32 %_4.i2131, 23, !dbg !9551
  %_0.i2133 = bitcast i32 %_3.i2132 to float, !dbg !9552
  %_0.i3041 = fmul float %_0.i2561, %_0.i2133, !dbg !9554
  %_0.i3045 = fmul float %_0.i3362, %_0.i2564.3, !dbg !9556
  %_0.i2563 = fadd float %_0.i3045, 1.000000e+00, !dbg !9558
  %biased.i2134 = fadd float %749, 0x4160000FE0000000, !dbg !9560
  %_4.i2135 = bitcast float %biased.i2134 to i32, !dbg !9562
  %_3.i2136 = shl i32 %_4.i2135, 23, !dbg !9564
  %_0.i2137 = bitcast i32 %_3.i2136 to float, !dbg !9565
  %_0.i3044 = fmul float %_0.i2563, %_0.i2137, !dbg !9567
  %_0.i3048 = fmul float %_0.i3363, %_0.i2566.3, !dbg !9569
  %_0.i2565 = fadd float %_0.i3048, 1.000000e+00, !dbg !9571
  %biased.i2138 = fadd float %755, 0x4160000FE0000000, !dbg !9573
  %_4.i2139 = bitcast float %biased.i2138 to i32, !dbg !9575
  %_3.i2140 = shl i32 %_4.i2139, 23, !dbg !9577
  %_0.i2141 = bitcast i32 %_3.i2140 to float, !dbg !9578
  %_0.i3047 = fmul float %_0.i2565, %_0.i2141, !dbg !9580
  %_0.i3051 = fmul float %_0.i3364, %_0.i2568.3, !dbg !9582
  %_0.i2567 = fadd float %_0.i3051, 1.000000e+00, !dbg !9584
  %biased.i2142 = fadd float %761, 0x4160000FE0000000, !dbg !9586
  %_4.i2143 = bitcast float %biased.i2142 to i32, !dbg !9588
  %_3.i2144 = shl i32 %_4.i2143, 23, !dbg !9590
  %_0.i2145 = bitcast i32 %_3.i2144 to float, !dbg !9591
  %_0.i3050 = fmul float %_0.i2567, %_0.i2145, !dbg !9593
  %_266.i.i674.i = icmp ugt i64 %_41.sroa.0.0.i.i576.i, %_315.1.i.i614.i, !dbg !9595
  br i1 %_266.i.i674.i, label %bb86.i.i711.i, label %bb87.i.i675.i, !dbg !9595, !prof !1664

bb84.i.i716.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4161
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i.i567.i11083, i64 noundef %_314.1.i.i607.i, i64 noundef %_314.1.i.i607.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e11ba0c4c1124bbcae4603a2ae121338) #26, !dbg !9600, !noalias !8783
  unreachable, !dbg !9600

bb87.i.i675.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3781
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9601), !dbg !9604
  %_3.not.i3772 = icmp eq i64 %_315.1.i.i614.i, %_41.sroa.0.0.i.i576.i, !dbg !9605
  br i1 %_3.not.i3772, label %panic.i3775, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3776, !dbg !9605

panic.i3775:                                      ; preds = %bb87.i.i675.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9605, !noalias !9607
  unreachable, !dbg !9605

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3776: ; preds = %bb87.i.i675.i
  %_273.i.i678.i = getelementptr inbounds nuw float, ptr %_315.0.i.i613.i, i64 %_41.sroa.0.0.i.i576.i, !dbg !9608
  %_0.i3774 = load float, ptr %_273.i.i678.i, align 4, !dbg !9605, !alias.scope !9601, !noalias !8783, !noundef !11
  %_0.i3328 = fmul float %_0.i3041, %_0.i3774, !dbg !9613
  %_274.i.i682.i = icmp ugt i64 %_41.sroa.0.0.i.i576.i, %_316.1.i.i624.i, !dbg !9615
  br i1 %_274.i.i682.i, label %bb88.i.i710.i, label %bb89.i.i683.i, !dbg !9615, !prof !1664

bb86.i.i711.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3781
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i576.i, i64 noundef %_315.1.i.i614.i, i64 noundef %_315.1.i.i614.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_379b93204ee4659b79f4b07b6520076a) #26, !dbg !9619, !noalias !8783
  unreachable, !dbg !9619

bb89.i.i683.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3776
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9620), !dbg !9623
  %_3.not.i3767 = icmp eq i64 %_316.1.i.i624.i, %_41.sroa.0.0.i.i576.i, !dbg !9624
  br i1 %_3.not.i3767, label %panic.i3770, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3771, !dbg !9624

panic.i3770:                                      ; preds = %bb89.i.i683.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9624, !noalias !9626
  unreachable, !dbg !9624

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3771: ; preds = %bb89.i.i683.i
  %_281.i.i686.i = getelementptr inbounds nuw float, ptr %_316.0.i.i623.i, i64 %_41.sroa.0.0.i.i576.i, !dbg !9627
  %_0.i3769 = load float, ptr %_281.i.i686.i, align 4, !dbg !9624, !alias.scope !9620, !noalias !8783, !noundef !11
  %_0.i3327 = fmul float %_0.i3044, %_0.i3769, !dbg !9632
  %_0.i2820 = fadd float %_0.i3328, %_0.i3327, !dbg !9634
  %_282.i.i691.i = icmp ugt i64 %_41.sroa.0.0.i.i576.i, %_317.1.i.i634.i, !dbg !9636
  br i1 %_282.i.i691.i, label %bb90.i.i709.i, label %bb91.i.i692.i, !dbg !9636, !prof !1664

bb88.i.i710.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3776
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i576.i, i64 noundef %_316.1.i.i624.i, i64 noundef %_316.1.i.i624.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2bb8eb0542a889f29ef069491eb97a17) #26, !dbg !9641, !noalias !8783
  unreachable, !dbg !9641

bb91.i.i692.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3771
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9642), !dbg !9645
  %_3.not.i3762 = icmp eq i64 %_317.1.i.i634.i, %_41.sroa.0.0.i.i576.i, !dbg !9646
  br i1 %_3.not.i3762, label %panic.i3765, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3766, !dbg !9646

panic.i3765:                                      ; preds = %bb91.i.i692.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9646, !noalias !9648
  unreachable, !dbg !9646

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3766: ; preds = %bb91.i.i692.i
  %_289.i.i695.i = getelementptr inbounds nuw float, ptr %_317.0.i.i633.i, i64 %_41.sroa.0.0.i.i576.i, !dbg !9649
  %_0.i3764 = load float, ptr %_289.i.i695.i, align 4, !dbg !9646, !alias.scope !9642, !noalias !8783, !noundef !11
  %_0.i3326 = fmul float %_0.i3047, %_0.i3764, !dbg !9654
  %_290.i.i699.i = icmp ugt i64 %_41.sroa.0.0.i.i576.i, %_318.1.i.i644.i, !dbg !9656
  br i1 %_290.i.i699.i, label %bb92.i.i708.i, label %bb93.i.i700.i, !dbg !9656, !prof !1664

bb90.i.i709.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3771
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i576.i, i64 noundef %_317.1.i.i634.i, i64 noundef %_317.1.i.i634.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd96798e807157d7db308788cb4dd125) #26, !dbg !9660, !noalias !8783
  unreachable, !dbg !9660

bb93.i.i700.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3766
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9661), !dbg !9664
  %_3.not.i3757 = icmp eq i64 %_318.1.i.i644.i, %_41.sroa.0.0.i.i576.i, !dbg !9665
  br i1 %_3.not.i3757, label %panic.i3760, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149, !dbg !9665

panic.i3760:                                      ; preds = %bb93.i.i700.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !9665, !noalias !9667
  unreachable, !dbg !9665

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149: ; preds = %bb93.i.i700.i
  %_297.i.i703.i = getelementptr inbounds nuw float, ptr %_318.0.i.i643.i, i64 %_41.sroa.0.0.i.i576.i, !dbg !9668
  %_0.i3759 = load float, ptr %_297.i.i703.i, align 4, !dbg !9665, !alias.scope !9661, !noalias !8783, !noundef !11
  %_0.i3325 = fmul float %_0.i3050, %_0.i3759, !dbg !9673
  %_0.i2819 = fadd float %_0.i3326, %_0.i3325, !dbg !9675
  store float %_0.i2820, ptr %_184.i.i578.i, align 4, !dbg !9677, !alias.scope !9680, !noalias !8783
  store float %_0.i2819, ptr %_192.i.i581.i, align 4, !dbg !9683, !alias.scope !9685, !noalias !8783
  %exitcond.not = icmp eq i64 %714, %plan.0.i540.i, !dbg !8742
  br i1 %exitcond.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_KBZ_EB2_.exit.i.i, label %bb56.i.i573.i, !dbg !8754

bb92.i.i708.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3766
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i.i576.i, i64 noundef %_318.1.i.i644.i, i64 noundef %_318.1.i.i644.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_164adf6876ccf79975d46e129e248ca5) #26, !dbg !9688, !noalias !8783
  unreachable, !dbg !9688

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_KBZ_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149, %bb27.i549.i
  %filter_near.i.i525.i.sroa.0.0.lcssa = phi float [ %filter_near.i.i525.i.sroa.0.0.copyload, %bb27.i549.i ], [ %_0.i4352, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9689
  %filter_near.i.i525.i.sroa.7.0.lcssa = phi float [ %filter_near.i.i525.i.sroa.7.0.copyload, %bb27.i549.i ], [ %_0.i4348, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9689
  %filter_near.i.i525.i.sroa.11.0.lcssa = phi float [ %filter_near.i.i525.i.sroa.11.0.copyload, %bb27.i549.i ], [ %_0.i4360, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9689
  %filter_near.i.i525.i.sroa.14.0.lcssa = phi float [ %filter_near.i.i525.i.sroa.14.0.copyload, %bb27.i549.i ], [ %_0.i4356, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9689
  %filter_far.i.i524.i.sroa.0.0.lcssa = phi float [ %filter_far.i.i524.i.sroa.0.0.copyload, %bb27.i549.i ], [ %_0.i4336, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9690
  %filter_far.i.i524.i.sroa.7.0.lcssa = phi float [ %filter_far.i.i524.i.sroa.7.0.copyload, %bb27.i549.i ], [ %_0.i4332, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9690
  %filter_far.i.i524.i.sroa.11.0.lcssa = phi float [ %filter_far.i.i524.i.sroa.11.0.copyload, %bb27.i549.i ], [ %_0.i4344, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9690
  %filter_far.i.i524.i.sroa.14.0.lcssa = phi float [ %filter_far.i.i524.i.sroa.14.0.copyload, %bb27.i549.i ], [ %_0.i4340, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9690
  %gain_near.i.i523.i.sroa.0.0.lcssa = phi i32 [ %709, %bb27.i549.i ], [ %_3.i4455, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9691
  %gain_near.i.i523.i.sroa.6.0.lcssa = phi i32 [ %710, %bb27.i549.i ], [ %_3.i4451, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9691
  %gain_far.i.i522.i.sroa.0.0.lcssa = phi i32 [ %711, %bb27.i549.i ], [ %_3.i4447, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9692
  %gain_far.i.i522.i.sroa.6.0.lcssa = phi i32 [ %712, %bb27.i549.i ], [ %_3.i4443, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9692
  %position.sroa.0.0.i.i567.i.lcssa = phi i64 [ %713, %bb27.i549.i ], [ %_41.sroa.0.0.i.i576.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4149 ], !dbg !9693
  store float %filter_near.i.i525.i.sroa.0.0.lcssa, ptr %672, align 8, !dbg !9694, !noalias !8783
  store float %filter_near.i.i525.i.sroa.7.0.lcssa, ptr %filter_near.i.i525.i.sroa.7.0..sroa_idx, align 4, !dbg !9694, !noalias !8783
  store float %filter_near.i.i525.i.sroa.11.0.lcssa, ptr %filter_near.i.i525.i.sroa.11.0..sroa_idx, align 8, !dbg !9694, !noalias !8783
  store float %filter_near.i.i525.i.sroa.14.0.lcssa, ptr %filter_near.i.i525.i.sroa.14.0..sroa_idx, align 4, !dbg !9694, !noalias !8783
  store float %filter_far.i.i524.i.sroa.0.0.lcssa, ptr %673, align 8, !dbg !9695, !noalias !8783
  store float %filter_far.i.i524.i.sroa.7.0.lcssa, ptr %filter_far.i.i524.i.sroa.7.0..sroa_idx, align 4, !dbg !9695, !noalias !8783
  store float %filter_far.i.i524.i.sroa.11.0.lcssa, ptr %filter_far.i.i524.i.sroa.11.0..sroa_idx, align 8, !dbg !9695, !noalias !8783
  store float %filter_far.i.i524.i.sroa.14.0.lcssa, ptr %filter_far.i.i524.i.sroa.14.0..sroa_idx, align 4, !dbg !9695, !noalias !8783
  store i32 %gain_near.i.i523.i.sroa.0.0.lcssa, ptr %674, align 8, !dbg !9696, !noalias !8783
  store i32 %gain_near.i.i523.i.sroa.6.0.lcssa, ptr %.sroa_idx7097, align 4, !dbg !9696, !noalias !8783
  store i32 %gain_far.i.i522.i.sroa.0.0.lcssa, ptr %675, align 8, !dbg !9697, !noalias !8783
  store i32 %gain_far.i.i522.i.sroa.6.0.lcssa, ptr %.sroa_idx7102, align 4, !dbg !9697, !noalias !8783
  store i64 %position.sroa.0.0.i.i567.i.lcssa, ptr %_51.i544.i, align 8, !dbg !9698, !alias.scope !8739, !noalias !8740
  br label %bb15.i572.i, !dbg !9699

bb15.i572.i:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_Kb1_EB2_.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_KBZ_EB2_.exit.i.i
  %_8.i537.i = icmp ult i64 %_32.i726.i, %_19.1, !dbg !8661
  br i1 %_8.i537.i, label %bb2.i538.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit, !dbg !8661

bb17.i730.i:                                      ; preds = %bb9.i724.i
  %_75.i731.i = getelementptr inbounds nuw float, ptr %_19.0, i64 %position.sroa.0.0.i536.i11127, !dbg !9700
  %_85.i734.i = getelementptr inbounds nuw float, ptr %_20.0, i64 %position.sroa.0.0.i536.i11127, !dbg !9703
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9710), !dbg !9713
  %filter_near.i17.i518.i.sroa.0.0.copyload = load float, ptr %672, align 8, !dbg !9714, !noalias !9720
  %filter_near.i17.i518.i.sroa.7.0.copyload = load float, ptr %filter_near.i.i525.i.sroa.7.0..sroa_idx, align 4, !dbg !9714, !noalias !9720
  %filter_near.i17.i518.i.sroa.11.0.copyload = load float, ptr %filter_near.i.i525.i.sroa.11.0..sroa_idx, align 8, !dbg !9714, !noalias !9720
  %filter_near.i17.i518.i.sroa.14.0.copyload = load float, ptr %filter_near.i.i525.i.sroa.14.0..sroa_idx, align 4, !dbg !9714, !noalias !9720
  %filter_far.i16.i517.i.sroa.0.0.copyload = load float, ptr %673, align 8, !dbg !9725, !noalias !9720
  %filter_far.i16.i517.i.sroa.7.0.copyload = load float, ptr %filter_far.i.i524.i.sroa.7.0..sroa_idx, align 4, !dbg !9725, !noalias !9720
  %filter_far.i16.i517.i.sroa.11.0.copyload = load float, ptr %filter_far.i.i524.i.sroa.11.0..sroa_idx, align 8, !dbg !9725, !noalias !9720
  %filter_far.i16.i517.i.sroa.14.0.copyload = load float, ptr %filter_far.i.i524.i.sroa.14.0..sroa_idx, align 4, !dbg !9725, !noalias !9720
  %762 = load i32, ptr %674, align 8, !dbg !9727
  %763 = load i32, ptr %.sroa_idx7097, align 4, !dbg !9727
  %764 = load i32, ptr %675, align 8, !dbg !9729
  %765 = load i32, ptr %.sroa_idx7102, align 4, !dbg !9729
  %766 = load i64, ptr %_51.i544.i, align 8, !dbg !9731, !alias.scope !9733, !noalias !9734, !noundef !11
  %_168.i34.i750.i11098.not = icmp eq i64 %plan.0.i540.i, 0, !dbg !9736
  br i1 %_168.i34.i750.i11098.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_Kb1_EB2_.exit.i.i, label %bb56.i37.i754.i, !dbg !9748

bb19.i918.i:                                      ; preds = %bb9.i724.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i536.i11127, i64 noundef %_32.i726.i, i64 noundef range(i64 1, 2305843009213693952) %_19.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_683f9160cdc5ee4596b2ecf709188a9a) #26, !dbg !9749, !noalias !3842
  unreachable, !dbg !9749

bb56.i37.i754.i:                                  ; preds = %bb17.i730.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125
  %segments.i532.i.sroa.0.1 = phi float [ %_0.i2812, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.i6860, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.8.1 = phi float [ %_0.i2812.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.1.i6862, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.12.1 = phi float [ %_0.i2812.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.2.i6864, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.16.1 = phi float [ %_0.i2812.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.3.i6866, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.20.1 = phi float [ %_0.i2812.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.4.i6868, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.26.1 = phi float [ %_0.i2812.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.5.i6870, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.32.1 = phi float [ %_0.i2812.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.6.i6872, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.36.1 = phi float [ %_0.i2812.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.7.i6874, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.40.1 = phi float [ %_0.i2812.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.8.i6876, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.44.1 = phi float [ %_0.i2812.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.9.i6878, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.70.1 = phi float [ %_0.i2811, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.i6898, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.76.1 = phi float [ %_0.i2811.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.1.i6900, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.80.1 = phi float [ %_0.i2811.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.2.i6902, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.84.1 = phi float [ %_0.i2811.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.3.i6904, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.88.1 = phi float [ %_0.i2811.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.4.i6906, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.94.1 = phi float [ %_0.i2811.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.5.i6908, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.100.1 = phi float [ %_0.i2811.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.6.i6910, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.104.1 = phi float [ %_0.i2811.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.7.i6912, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.108.1 = phi float [ %_0.i2811.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.8.i6914, %bb17.i730.i ], !dbg !9750
  %segments.i532.i.sroa.112.1 = phi float [ %_0.i2811.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %_12.le.9.i6916, %bb17.i730.i ], !dbg !9750
  %iter.sroa.0.0.i33.i749.i11112 = phi i64 [ %767, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ 0, %bb17.i730.i ]
  %position.sroa.0.0.i32.i748.i11111 = phi i64 [ %_41.sroa.0.0.i40.i761.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %766, %bb17.i730.i ]
  %gain_far.i14.i515.i.sroa.6.011110 = phi i32 [ %_3.i4427, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %765, %bb17.i730.i ]
  %gain_far.i14.i515.i.sroa.0.011109 = phi i32 [ %_3.i4431, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %764, %bb17.i730.i ]
  %gain_near.i15.i516.i.sroa.6.011108 = phi i32 [ %_3.i4435, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %763, %bb17.i730.i ]
  %gain_near.i15.i516.i.sroa.0.011107 = phi i32 [ %_3.i4439, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %762, %bb17.i730.i ]
  %filter_far.i16.i517.i.sroa.14.011106 = phi float [ %_0.i4308, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %filter_far.i16.i517.i.sroa.14.0.copyload, %bb17.i730.i ]
  %filter_far.i16.i517.i.sroa.11.011105 = phi float [ %_0.i4312, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %filter_far.i16.i517.i.sroa.11.0.copyload, %bb17.i730.i ]
  %filter_far.i16.i517.i.sroa.7.011104 = phi float [ %_0.i4300, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %filter_far.i16.i517.i.sroa.7.0.copyload, %bb17.i730.i ]
  %filter_far.i16.i517.i.sroa.0.011103 = phi float [ %_0.i4304, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %filter_far.i16.i517.i.sroa.0.0.copyload, %bb17.i730.i ]
  %filter_near.i17.i518.i.sroa.14.011102 = phi float [ %_0.i4324, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %filter_near.i17.i518.i.sroa.14.0.copyload, %bb17.i730.i ]
  %filter_near.i17.i518.i.sroa.11.011101 = phi float [ %_0.i4328, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %filter_near.i17.i518.i.sroa.11.0.copyload, %bb17.i730.i ]
  %filter_near.i17.i518.i.sroa.7.011100 = phi float [ %_0.i4316, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %filter_near.i17.i518.i.sroa.7.0.copyload, %bb17.i730.i ]
  %filter_near.i17.i518.i.sroa.0.011099 = phi float [ %_0.i4320, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], [ %filter_near.i17.i518.i.sroa.0.0.copyload, %bb17.i730.i ]
  %_0.i2812 = fadd float %segments.i532.i.sroa.0.1, %_16.le.i6861, !dbg !9751
  %_0.i2811 = fadd float %segments.i532.i.sroa.70.1, %_16.le.i6899, !dbg !9756
  %_0.i2812.1 = fadd float %segments.i532.i.sroa.8.1, %_16.le.1.i6863, !dbg !9751
  %_0.i2811.1 = fadd float %segments.i532.i.sroa.76.1, %_16.le.1.i6901, !dbg !9756
  %_0.i2812.2 = fadd float %segments.i532.i.sroa.12.1, %_16.le.2.i6865, !dbg !9751
  %_0.i2811.2 = fadd float %segments.i532.i.sroa.80.1, %_16.le.2.i6903, !dbg !9756
  %_0.i2812.3 = fadd float %segments.i532.i.sroa.16.1, %_16.le.3.i6867, !dbg !9751
  %_0.i2811.3 = fadd float %segments.i532.i.sroa.84.1, %_16.le.3.i6905, !dbg !9756
  %_0.i2812.4 = fadd float %segments.i532.i.sroa.20.1, %_16.le.4.i6869, !dbg !9751
  %_0.i2811.4 = fadd float %segments.i532.i.sroa.88.1, %_16.le.4.i6907, !dbg !9756
  %_0.i2812.5 = fadd float %segments.i532.i.sroa.26.1, %_16.le.5.i6871, !dbg !9751
  %_0.i2811.5 = fadd float %segments.i532.i.sroa.94.1, %_16.le.5.i6909, !dbg !9756
  %_0.i2812.6 = fadd float %segments.i532.i.sroa.32.1, %_16.le.6.i6873, !dbg !9751
  %_0.i2811.6 = fadd float %segments.i532.i.sroa.100.1, %_16.le.6.i6911, !dbg !9756
  %_0.i2812.7 = fadd float %segments.i532.i.sroa.36.1, %_16.le.7.i6875, !dbg !9751
  %_0.i2811.7 = fadd float %segments.i532.i.sroa.104.1, %_16.le.7.i6913, !dbg !9756
  %_0.i2812.8 = fadd float %segments.i532.i.sroa.40.1, %_16.le.8.i6877, !dbg !9751
  %_0.i2811.8 = fadd float %segments.i532.i.sroa.108.1, %_16.le.8.i6915, !dbg !9756
  %_0.i2812.9 = fadd float %segments.i532.i.sroa.44.1, %_16.le.9.i6879, !dbg !9751
  %_0.i2811.9 = fadd float %segments.i532.i.sroa.112.1, %_16.le.9.i6917, !dbg !9756
  %767 = add nuw i64 %iter.sroa.0.0.i33.i749.i11112, 1, !dbg !9758
  %_42.i38.i759.i = add i64 %position.sroa.0.0.i32.i748.i11111, 1, !dbg !9764
  %_176.not.i39.i760.i = icmp ult i64 %_42.i38.i759.i, %ring_len.i534.i, !dbg !9766
  %768 = select i1 %_176.not.i39.i760.i, i64 0, i64 %ring_len.i534.i, !dbg !9766
  %_41.sroa.0.0.i40.i761.i = sub nuw i64 %_42.i38.i759.i, %768, !dbg !9766
  %_184.i42.i765.i = getelementptr inbounds nuw float, ptr %_75.i731.i, i64 %iter.sroa.0.0.i33.i749.i11112, !dbg !9769
  %_0.i3754 = load float, ptr %_184.i42.i765.i, align 4, !dbg !9779, !alias.scope !9781, !noalias !9784, !noundef !11
  %_192.i47.i768.i = getelementptr inbounds nuw float, ptr %_85.i734.i, i64 %iter.sroa.0.0.i33.i749.i11112, !dbg !9785
  %_0.i3749 = load float, ptr %_192.i47.i768.i, align 4, !dbg !9794, !alias.scope !9796, !noalias !9784, !noundef !11
  %_7.i106 = load float, ptr %_67.i.i583.i, align 4, !dbg !9799, !alias.scope !9802, !noalias !9805, !noundef !11
  %_8.i107 = load float, ptr %676, align 4, !dbg !9807, !alias.scope !9802, !noalias !9805, !noundef !11
  %_9.i108 = load float, ptr %677, align 4, !dbg !9808, !alias.scope !9802, !noalias !9805, !noundef !11
  %_0.i3427 = fsub float %_0.i3754, %filter_near.i17.i518.i.sroa.7.011100, !dbg !9809
  %_0.i3254 = fmul float %_0.i3427, %_8.i107, !dbg !9812
  %_4.i2899 = fmul float %filter_near.i17.i518.i.sroa.0.011099, %_7.i106, !dbg !9814
  %_0.i2900 = fadd float %_4.i2899, %_0.i3254, !dbg !9814
  %_0.i2750 = fadd float %filter_near.i17.i518.i.sroa.0.011099, %_0.i2900, !dbg !9816
  %_0.i3253 = fmul float %filter_near.i17.i518.i.sroa.0.011099, %_8.i107, !dbg !9818
  %_4.i2897 = fmul float %_0.i3427, %_9.i108, !dbg !9820
  %_0.i2898 = fadd float %_0.i3253, %_4.i2897, !dbg !9820
  %_0.i2749 = fadd float %filter_near.i17.i518.i.sroa.7.011100, %_0.i2898, !dbg !9822
  %_0.i2748 = fadd float %_0.i2900, %_0.i2900, !dbg !9824
  %_0.i2747 = fadd float %filter_near.i17.i518.i.sroa.0.011099, %_0.i2748, !dbg !9826
  %769 = tail call noundef float @llvm.fabs.f32(float %_0.i2747), !dbg !9828
  %770 = fcmp uge float %769, 0x3BC79CA100000000, !dbg !9831
  %_0.i4320 = select i1 %770, float %_0.i2747, float 0.000000e+00, !dbg !9833
  %_0.i2746 = fadd float %_0.i2898, %_0.i2898, !dbg !9834
  %_0.i2745 = fadd float %filter_near.i17.i518.i.sroa.7.011100, %_0.i2746, !dbg !9836
  %771 = tail call noundef float @llvm.fabs.f32(float %_0.i2745), !dbg !9838
  %772 = fcmp uge float %771, 0x3BC79CA100000000, !dbg !9841
  %_0.i4316 = select i1 %772, float %_0.i2745, float 0.000000e+00, !dbg !9843
  %_12.i111 = load float, ptr %678, align 4, !dbg !9844, !alias.scope !9802, !noalias !9805, !noundef !11
  %_4.i2925 = fmul float %_12.i111, %_0.i2750, !dbg !9845
  %_0.i2926 = fadd float %_0.i3754, %_4.i2925, !dbg !9845
  %_0.i3428 = fsub float %_0.i2749, %filter_near.i17.i518.i.sroa.14.011102, !dbg !9847
  %_0.i3256 = fmul float %_8.i107, %_0.i3428, !dbg !9850
  %_4.i2903 = fmul float %filter_near.i17.i518.i.sroa.11.011101, %_7.i106, !dbg !9852
  %_0.i2904 = fadd float %_4.i2903, %_0.i3256, !dbg !9852
  %_0.i3255 = fmul float %filter_near.i17.i518.i.sroa.11.011101, %_8.i107, !dbg !9854
  %_4.i2901 = fmul float %_9.i108, %_0.i3428, !dbg !9856
  %_0.i2902 = fadd float %_0.i3255, %_4.i2901, !dbg !9856
  %_0.i2755 = fadd float %filter_near.i17.i518.i.sroa.14.011102, %_0.i2902, !dbg !9858
  %_0.i2754 = fadd float %_0.i2904, %_0.i2904, !dbg !9860
  %_0.i2753 = fadd float %filter_near.i17.i518.i.sroa.11.011101, %_0.i2754, !dbg !9862
  %773 = tail call noundef float @llvm.fabs.f32(float %_0.i2753), !dbg !9864
  %774 = fcmp uge float %773, 0x3BC79CA100000000, !dbg !9867
  %_0.i4328 = select i1 %774, float %_0.i2753, float 0.000000e+00, !dbg !9869
  %_0.i2752 = fadd float %_0.i2902, %_0.i2902, !dbg !9870
  %_0.i2751 = fadd float %filter_near.i17.i518.i.sroa.14.011102, %_0.i2752, !dbg !9872
  %775 = tail call noundef float @llvm.fabs.f32(float %_0.i2751), !dbg !9874
  %776 = fcmp uge float %775, 0x3BC79CA100000000, !dbg !9877
  %_0.i4324 = select i1 %776, float %_0.i2751, float 0.000000e+00, !dbg !9879
  %_0.i3435 = fsub float %_0.i2926, %_0.i2755, !dbg !9880
  %_7.i93 = load float, ptr %_72.i.i585.i, align 4, !dbg !9882, !alias.scope !9885, !noalias !9888, !noundef !11
  %_8.i94 = load float, ptr %679, align 4, !dbg !9890, !alias.scope !9885, !noalias !9888, !noundef !11
  %_9.i95 = load float, ptr %680, align 4, !dbg !9891, !alias.scope !9885, !noalias !9888, !noundef !11
  %_0.i3425 = fsub float %_0.i3749, %filter_far.i16.i517.i.sroa.7.011104, !dbg !9892
  %_0.i3250 = fmul float %_0.i3425, %_8.i94, !dbg !9895
  %_4.i2891 = fmul float %filter_far.i16.i517.i.sroa.0.011103, %_7.i93, !dbg !9897
  %_0.i2892 = fadd float %_4.i2891, %_0.i3250, !dbg !9897
  %_0.i2738 = fadd float %filter_far.i16.i517.i.sroa.0.011103, %_0.i2892, !dbg !9899
  %_0.i3249 = fmul float %filter_far.i16.i517.i.sroa.0.011103, %_8.i94, !dbg !9901
  %_4.i2889 = fmul float %_0.i3425, %_9.i95, !dbg !9903
  %_0.i2890 = fadd float %_0.i3249, %_4.i2889, !dbg !9903
  %_0.i2737 = fadd float %filter_far.i16.i517.i.sroa.7.011104, %_0.i2890, !dbg !9905
  %_0.i2736 = fadd float %_0.i2892, %_0.i2892, !dbg !9907
  %_0.i2735 = fadd float %filter_far.i16.i517.i.sroa.0.011103, %_0.i2736, !dbg !9909
  %777 = tail call noundef float @llvm.fabs.f32(float %_0.i2735), !dbg !9911
  %778 = fcmp uge float %777, 0x3BC79CA100000000, !dbg !9914
  %_0.i4304 = select i1 %778, float %_0.i2735, float 0.000000e+00, !dbg !9916
  %_0.i2734 = fadd float %_0.i2890, %_0.i2890, !dbg !9917
  %_0.i2733 = fadd float %filter_far.i16.i517.i.sroa.7.011104, %_0.i2734, !dbg !9919
  %779 = tail call noundef float @llvm.fabs.f32(float %_0.i2733), !dbg !9921
  %780 = fcmp uge float %779, 0x3BC79CA100000000, !dbg !9924
  %_0.i4300 = select i1 %780, float %_0.i2733, float 0.000000e+00, !dbg !9926
  %_12.i98 = load float, ptr %681, align 4, !dbg !9927, !alias.scope !9885, !noalias !9888, !noundef !11
  %_4.i2927 = fmul float %_12.i98, %_0.i2738, !dbg !9928
  %_0.i2928 = fadd float %_0.i3749, %_4.i2927, !dbg !9928
  %_0.i3426 = fsub float %_0.i2737, %filter_far.i16.i517.i.sroa.14.011106, !dbg !9930
  %_0.i3252 = fmul float %_8.i94, %_0.i3426, !dbg !9933
  %_4.i2895 = fmul float %filter_far.i16.i517.i.sroa.11.011105, %_7.i93, !dbg !9935
  %_0.i2896 = fadd float %_4.i2895, %_0.i3252, !dbg !9935
  %_0.i3251 = fmul float %filter_far.i16.i517.i.sroa.11.011105, %_8.i94, !dbg !9937
  %_4.i2893 = fmul float %_9.i95, %_0.i3426, !dbg !9939
  %_0.i2894 = fadd float %_0.i3251, %_4.i2893, !dbg !9939
  %_0.i2743 = fadd float %filter_far.i16.i517.i.sroa.14.011106, %_0.i2894, !dbg !9941
  %_0.i2742 = fadd float %_0.i2896, %_0.i2896, !dbg !9943
  %_0.i2741 = fadd float %filter_far.i16.i517.i.sroa.11.011105, %_0.i2742, !dbg !9945
  %781 = tail call noundef float @llvm.fabs.f32(float %_0.i2741), !dbg !9947
  %782 = fcmp uge float %781, 0x3BC79CA100000000, !dbg !9950
  %_0.i4312 = select i1 %782, float %_0.i2741, float 0.000000e+00, !dbg !9952
  %_0.i2740 = fadd float %_0.i2894, %_0.i2894, !dbg !9953
  %_0.i2739 = fadd float %filter_far.i16.i517.i.sroa.14.011106, %_0.i2740, !dbg !9955
  %783 = tail call noundef float @llvm.fabs.f32(float %_0.i2739), !dbg !9957
  %784 = fcmp uge float %783, 0x3BC79CA100000000, !dbg !9960
  %_0.i4308 = select i1 %784, float %_0.i2739, float 0.000000e+00, !dbg !9962
  %_0.i3436 = fsub float %_0.i2928, %_0.i2743, !dbg !9963
  %_311.1.i54.i775.i = load i64, ptr %682, align 8, !dbg !9965, !noalias !9784, !noundef !11
  %_234.i55.i776.i = icmp ugt i64 %position.sroa.0.0.i32.i748.i11111, %_311.1.i54.i775.i, !dbg !9967
  br i1 %_234.i55.i776.i, label %bb78.i153.i906.i, label %bb79.i56.i777.i, !dbg !9967, !prof !1664

bb79.i56.i777.i:                                  ; preds = %bb56.i37.i754.i
  %_311.0.i57.i778.i = load ptr, ptr %631, align 8, !dbg !9965, !noalias !9784, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9973), !dbg !9976
  %_4.not.i4142 = icmp eq i64 %_311.1.i54.i775.i, %position.sroa.0.0.i32.i748.i11111, !dbg !9977
  br i1 %_4.not.i4142, label %panic.i4144, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4145, !dbg !9977

panic.i4144:                                      ; preds = %bb79.i56.i777.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !9977, !noalias !9979
  unreachable, !dbg !9977

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4145: ; preds = %bb79.i56.i777.i
  %_241.i60.i781.i = getelementptr inbounds nuw float, ptr %_311.0.i57.i778.i, i64 %position.sroa.0.0.i32.i748.i11111, !dbg !9980
  store float %_0.i2755, ptr %_241.i60.i781.i, align 4, !dbg !9977, !alias.scope !9973, !noalias !9784
  %_312.1.i61.i782.i = load i64, ptr %684, align 8, !dbg !9986, !noalias !9784, !noundef !11
  %_242.i62.i783.i = icmp ugt i64 %position.sroa.0.0.i32.i748.i11111, %_312.1.i61.i782.i, !dbg !9987
  br i1 %_242.i62.i783.i, label %bb80.i152.i905.i, label %bb81.i63.i784.i, !dbg !9987, !prof !1664

bb78.i153.i906.i:                                 ; preds = %bb56.i37.i754.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i748.i11111, i64 noundef %_311.1.i54.i775.i, i64 noundef %_311.1.i54.i775.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7515f6adc8a5521b72f04a3abb372e72) #26, !dbg !9991, !noalias !9784
  unreachable, !dbg !9991

bb81.i63.i784.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4145
  %_312.0.i64.i785.i = load ptr, ptr %683, align 8, !dbg !9986, !noalias !9784, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9992), !dbg !9995
  %_4.not.i4138 = icmp eq i64 %_312.1.i61.i782.i, %position.sroa.0.0.i32.i748.i11111, !dbg !9996
  br i1 %_4.not.i4138, label %panic.i4140, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4141, !dbg !9996

panic.i4140:                                      ; preds = %bb81.i63.i784.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !9996, !noalias !9998
  unreachable, !dbg !9996

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4141: ; preds = %bb81.i63.i784.i
  %_249.i66.i787.i = getelementptr inbounds nuw float, ptr %_312.0.i64.i785.i, i64 %position.sroa.0.0.i32.i748.i11111, !dbg !9999
  store float %_0.i3435, ptr %_249.i66.i787.i, align 4, !dbg !9996, !alias.scope !9992, !noalias !9784
  %_313.1.i67.i788.i = load i64, ptr %685, align 8, !dbg !10004, !noalias !9784, !noundef !11
  %_250.i68.i789.i = icmp ugt i64 %position.sroa.0.0.i32.i748.i11111, %_313.1.i67.i788.i, !dbg !10005
  br i1 %_250.i68.i789.i, label %bb82.i151.i904.i, label %bb83.i69.i790.i, !dbg !10005, !prof !1664

bb80.i152.i905.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4145
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i748.i11111, i64 noundef %_312.1.i61.i782.i, i64 noundef %_312.1.i61.i782.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8c2aade3368450b16e7e4997ad3fca81) #26, !dbg !10009, !noalias !9784
  unreachable, !dbg !10009

bb83.i69.i790.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4141
  %_313.0.i70.i791.i = load ptr, ptr %_18.i542.i, align 8, !dbg !10004, !noalias !9784, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10010), !dbg !10013
  %_4.not.i4134 = icmp eq i64 %_313.1.i67.i788.i, %position.sroa.0.0.i32.i748.i11111, !dbg !10014
  br i1 %_4.not.i4134, label %panic.i4136, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4137, !dbg !10014

panic.i4136:                                      ; preds = %bb83.i69.i790.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !10014, !noalias !10016
  unreachable, !dbg !10014

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4137: ; preds = %bb83.i69.i790.i
  %_257.i72.i793.i = getelementptr inbounds nuw float, ptr %_313.0.i70.i791.i, i64 %position.sroa.0.0.i32.i748.i11111, !dbg !10017
  store float %_0.i2743, ptr %_257.i72.i793.i, align 4, !dbg !10014, !alias.scope !10010, !noalias !9784
  %_314.1.i73.i794.i = load i64, ptr %687, align 8, !dbg !10022, !noalias !9784, !noundef !11
  %_258.i74.i795.i = icmp ugt i64 %position.sroa.0.0.i32.i748.i11111, %_314.1.i73.i794.i, !dbg !10023
  br i1 %_258.i74.i795.i, label %bb84.i150.i903.i, label %bb85.i75.i796.i, !dbg !10023, !prof !1664

bb82.i151.i904.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4141
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i748.i11111, i64 noundef %_313.1.i67.i788.i, i64 noundef %_313.1.i67.i788.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a4c3da5a99763e9452b49d42852d67e3) #26, !dbg !10027, !noalias !9784
  unreachable, !dbg !10027

bb85.i75.i796.i:                                  ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4137
  %_314.0.i76.i797.i = load ptr, ptr %686, align 8, !dbg !10022, !noalias !9784, !nonnull !11, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10028), !dbg !10031
  %_4.not.i4130 = icmp eq i64 %_314.1.i73.i794.i, %position.sroa.0.0.i32.i748.i11111, !dbg !10032
  br i1 %_4.not.i4130, label %panic.i4132, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4133, !dbg !10032

panic.i4132:                                      ; preds = %bb85.i75.i796.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b60532d7446335d93b6150c47af294d0) #26, !dbg !10032, !noalias !10034
  unreachable, !dbg !10032

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4133: ; preds = %bb85.i75.i796.i
  %_265.i78.i799.i = getelementptr inbounds nuw float, ptr %_314.0.i76.i797.i, i64 %position.sroa.0.0.i32.i748.i11111, !dbg !10035
  store float %_0.i3436, ptr %_265.i78.i799.i, align 4, !dbg !10032, !alias.scope !10028, !noalias !9784
  %_315.0.i79.i800.i = load ptr, ptr %631, align 8, !dbg !10040, !noalias !9784, !nonnull !11, !noundef !11
  %_315.1.i80.i801.i = load i64, ptr %682, align 8, !dbg !10040, !noalias !9784, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10041), !dbg !10044
  %_8.i176.i802.i = load i64, ptr %_24.i.i562.i, align 8, !dbg !10045, !alias.scope !10041, !noalias !10047, !noundef !11
  %_7.i177.i803.i = add i64 %_8.i176.i802.i, %position.sroa.0.0.i32.i748.i11111, !dbg !10049
  %_27.not.i178.i804.i = icmp ult i64 %_7.i177.i803.i, %ring_len.i534.i, !dbg !10050
  %785 = select i1 %_27.not.i178.i804.i, i64 0, i64 %ring_len.i534.i, !dbg !10050
  %row.sroa.0.0.i179.i805.i = sub nuw i64 %_7.i177.i803.i, %785, !dbg !10050
  %_28.i180.i806.i = icmp ugt i64 %row.sroa.0.0.i179.i805.i, %_315.1.i80.i801.i, !dbg !10052
  br i1 %_28.i180.i806.i, label %bb14.i183.i902.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i807.i, !dbg !10052, !prof !1664

bb14.i183.i902.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4133
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i179.i805.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i80.i801.i, i64 noundef range(i64 0, 2305843009213693952) %_315.1.i80.i801.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !10055, !noalias !10056
  unreachable, !dbg !10055

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i807.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4133
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10057), !dbg !10060
  %_3.not.i3742 = icmp eq i64 %_315.1.i80.i801.i, %row.sroa.0.0.i179.i805.i, !dbg !10061
  br i1 %_3.not.i3742, label %panic.i3745, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3746, !dbg !10061

panic.i3745:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i807.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10061, !noalias !10063
  unreachable, !dbg !10061

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3746: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit184.i807.i
  %_35.i182.i809.i = getelementptr inbounds nuw float, ptr %_315.0.i79.i800.i, i64 %row.sroa.0.0.i179.i805.i, !dbg !10064
  %_0.i3744 = load float, ptr %_35.i182.i809.i, align 4, !dbg !10061, !alias.scope !10057, !noalias !10066, !noundef !11
  %_316.0.i82.i810.i = load ptr, ptr %683, align 8, !dbg !10067, !noalias !9784, !nonnull !11, !noundef !11
  %_316.1.i83.i811.i = load i64, ptr %684, align 8, !dbg !10067, !noalias !9784, !noundef !11
  %_28.i171.i816.i = icmp ugt i64 %row.sroa.0.0.i179.i805.i, %_316.1.i83.i811.i, !dbg !10069
  br i1 %_28.i171.i816.i, label %bb14.i174.i901.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i817.i, !dbg !10069, !prof !1664

bb14.i174.i901.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3746
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i179.i805.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i83.i811.i, i64 noundef range(i64 0, 2305843009213693952) %_316.1.i83.i811.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !10073, !noalias !10074
  unreachable, !dbg !10073

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i817.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3746
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10078), !dbg !10081
  %_3.not.i3737 = icmp eq i64 %_316.1.i83.i811.i, %row.sroa.0.0.i179.i805.i, !dbg !10082
  br i1 %_3.not.i3737, label %panic.i3740, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3741, !dbg !10082

panic.i3740:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i817.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10082, !noalias !10084
  unreachable, !dbg !10082

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3741: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit175.i817.i
  %_35.i173.i819.i = getelementptr inbounds nuw float, ptr %_316.0.i82.i810.i, i64 %row.sroa.0.0.i179.i805.i, !dbg !10085
  %_0.i3739 = load float, ptr %_35.i173.i819.i, align 4, !dbg !10082, !alias.scope !10078, !noalias !10087, !noundef !11
  %_317.0.i85.i820.i = load ptr, ptr %_18.i542.i, align 8, !dbg !10088, !noalias !9784, !nonnull !11, !noundef !11
  %_317.1.i86.i821.i = load i64, ptr %685, align 8, !dbg !10088, !noalias !9784, !noundef !11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10090), !dbg !10093
  %_8.i158.i822.i = load i64, ptr %_26.i.i564.i, align 8, !dbg !10094, !alias.scope !10090, !noalias !10096, !noundef !11
  %_7.i159.i823.i = add i64 %_8.i158.i822.i, %position.sroa.0.0.i32.i748.i11111, !dbg !10098
  %_27.not.i160.i824.i = icmp ult i64 %_7.i159.i823.i, %ring_len.i534.i, !dbg !10099
  %786 = select i1 %_27.not.i160.i824.i, i64 0, i64 %ring_len.i534.i, !dbg !10099
  %row.sroa.0.0.i161.i825.i = sub nuw i64 %_7.i159.i823.i, %786, !dbg !10099
  %_28.i162.i826.i = icmp ugt i64 %row.sroa.0.0.i161.i825.i, %_317.1.i86.i821.i, !dbg !10101
  br i1 %_28.i162.i826.i, label %bb14.i165.i900.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i827.i, !dbg !10101, !prof !1664

bb14.i165.i900.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3741
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i161.i825.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i86.i821.i, i64 noundef range(i64 0, 2305843009213693952) %_317.1.i86.i821.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !10104, !noalias !10105
  unreachable, !dbg !10104

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i827.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3741
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10106), !dbg !10109
  %_3.not.i3732 = icmp eq i64 %_317.1.i86.i821.i, %row.sroa.0.0.i161.i825.i, !dbg !10110
  br i1 %_3.not.i3732, label %panic.i3735, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3736, !dbg !10110

panic.i3735:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i827.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10110, !noalias !10112
  unreachable, !dbg !10110

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3736: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit166.i827.i
  %_35.i164.i829.i = getelementptr inbounds nuw float, ptr %_317.0.i85.i820.i, i64 %row.sroa.0.0.i161.i825.i, !dbg !10113
  %_0.i3734 = load float, ptr %_35.i164.i829.i, align 4, !dbg !10110, !alias.scope !10106, !noalias !10115, !noundef !11
  %_318.0.i88.i830.i = load ptr, ptr %686, align 8, !dbg !10116, !noalias !9784, !nonnull !11, !noundef !11
  %_318.1.i89.i831.i = load i64, ptr %687, align 8, !dbg !10116, !noalias !9784, !noundef !11
  %_28.i.i836.i = icmp ugt i64 %row.sroa.0.0.i161.i825.i, %_318.1.i89.i831.i, !dbg !10118
  br i1 %_28.i.i836.i, label %bb14.i.i899.i, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i837.i, !dbg !10118, !prof !1664

bb14.i.i899.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3736
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %row.sroa.0.0.i161.i825.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i89.i831.i, i64 noundef range(i64 0, 2305843009213693952) %_318.1.i89.i831.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a843826d471182f1ef404bfcc8b3fde6) #26, !dbg !10122, !noalias !10123
  unreachable, !dbg !10122

_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i837.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3736
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10127), !dbg !10130
  %_3.not.i3727 = icmp eq i64 %_318.1.i89.i831.i, %row.sroa.0.0.i161.i825.i, !dbg !10131
  br i1 %_3.not.i3727, label %panic.i3730, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3731, !dbg !10131

panic.i3730:                                      ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i837.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10131, !noalias !10133
  unreachable, !dbg !10131

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3731: ; preds = %_RINvCs5xZ4lC6BZIV_20multiband_compressor12detector_tapfKj1_EB2_.exit.i837.i
  %_35.i157.i839.i = getelementptr inbounds nuw float, ptr %_318.0.i88.i830.i, i64 %row.sroa.0.0.i161.i825.i, !dbg !10134
  %_0.i3729 = load float, ptr %_35.i157.i839.i, align 4, !dbg !10131, !alias.scope !10127, !noalias !10136, !noundef !11
  %787 = tail call noundef float @llvm.fabs.f32(float %_0.i3744), !dbg !10137
  %788 = tail call noundef float @llvm.fabs.f32(float %_0.i3734), !dbg !10141
  %_0.i3214 = fmul float %787, 5.000000e-01, !dbg !10143
  %_0.i3213 = fmul float %788, 5.000000e-01, !dbg !10145
  %_0.i2635 = fadd float %_0.i3214, %_0.i3213, !dbg !10147
  %_3.i.i5820 = fcmp ule float %_0.i2635, 0x3E45798EE0000000, !dbg !10149
  %_6.i.i5822 = bitcast float %_0.i2635 to i32, !dbg !10155
  %_4.i.i5826 = select i1 %_3.i.i5820, i32 841731191, i32 %_6.i.i5822, !dbg !10158
  %_0.i.i5827 = bitcast i32 %_4.i.i5826 to float, !dbg !10159
  %_3.i.i4962 = fcmp ule float %_0.i.i5827, 0x3810000000000000, !dbg !10161
  %_4.i.i4968 = select i1 %_3.i.i4962, i32 8388608, i32 %_4.i.i5826, !dbg !10166
  %_5.i3837 = and i32 %_4.i.i4968, 8388607, !dbg !10168
  %_4.i3838 = or disjoint i32 %_5.i3837, 1065353216, !dbg !10168
  %significand.i3839 = bitcast i32 %_4.i3838 to float, !dbg !10170
  %_0.i3341 = fadd float %significand.i3839, -1.000000e+00, !dbg !10172
  %_0.i3002 = fmul float %_0.i3341, 0xBF9B17A960000000, !dbg !10174
  %_0.i2522 = fadd float %_0.i3002, 0x3FBF9A8440000000, !dbg !10176
  %_0.i3002.1 = fmul float %_0.i3341, %_0.i2522, !dbg !10174
  %_0.i2522.1 = fadd float %_0.i3002.1, 0xBFD1E3F400000000, !dbg !10176
  %_0.i3002.2 = fmul float %_0.i3341, %_0.i2522.1, !dbg !10174
  %_0.i2522.2 = fadd float %_0.i3002.2, 0x3FDD544F20000000, !dbg !10176
  %_0.i3002.3 = fmul float %_0.i3341, %_0.i2522.2, !dbg !10174
  %_0.i2522.3 = fadd float %_0.i3002.3, 0xBFE6FC2A60000000, !dbg !10176
  %_0.i3002.4 = fmul float %_0.i3341, %_0.i2522.3, !dbg !10174
  %_0.i2522.4 = fadd float %_0.i3002.4, 0x3FF714B2A0000000, !dbg !10176
  %_9.i3840 = lshr i32 %_4.i.i4968, 23, !dbg !10178
  %_8.i3841 = or disjoint i32 %_9.i3840, 1258291200, !dbg !10178
  %_7.i3842 = bitcast i32 %_8.i3841 to float, !dbg !10179
  %exponent.i3843 = fadd float %_7.i3842, 0xC160000FE0000000, !dbg !10181
  %_0.i3001 = fmul float %_0.i3341, %_0.i2522.4, !dbg !10182
  %_0.i2521 = fadd float %exponent.i3843, %_0.i3001, !dbg !10184
  %_0.i3324 = fmul float %_0.i2521, 0x4018151820000000, !dbg !10186
  %_3.i.i5812.inv = fcmp ogt float %_0.i3324, -1.600000e+02, !dbg !10188
  %_0.i.i5819 = select i1 %_3.i.i5812.inv, float %_0.i3324, float -1.600000e+02, !dbg !10188
  %_3.i.i6428.inv = fcmp olt float %_0.i.i5819, 2.400000e+01, !dbg !10191
  %_0.i.i6435 = select i1 %_3.i.i6428.inv, float %_0.i.i5819, float 2.400000e+01, !dbg !10191
  %_0.i3389 = fsub float %_0.i.i6435, %_0.i2812, !dbg !10194
  %_3.i2233 = fcmp ule float %_0.i3389, 3.000000e+00, !dbg !10197
  %_0.i2613 = fadd float %_0.i3389, 3.000000e+00, !dbg !10199
  %_0.i3131 = fmul float %_0.i2613, %_0.i2613, !dbg !10201
  %_0.i3130 = fmul float %_0.i3131, 0x3FB5555560000000, !dbg !10203
  %_4.i4520.v.v = select i1 %_3.i2233, float %_0.i3130, float %_0.i3389, !dbg !10205
  %_4.i4520.v = fmul float %coefficients.i529.i.sroa.0.0.copyload, %_4.i4520.v.v, !dbg !10205
  %_4.i4520 = bitcast float %_4.i4520.v to i32, !dbg !10205
  %789 = fcmp ugt float %_0.i3389, -3.000000e+00, !dbg !10207
  %_7.i4512 = select i1 %789, i32 %_4.i4520, i32 0, !dbg !10209
  %_0.i4514 = bitcast i32 %_7.i4512 to float, !dbg !10210
  %_3.i.i5804 = fcmp ule float %_0.i4514, -1.000000e+02, !dbg !10212
  %790 = bitcast i32 %_7.i4512 to float, !dbg !10215
  %_0.i.i5811 = select i1 %_3.i.i5804, float -1.000000e+02, float %790, !dbg !10218
  %_3.i.i6420 = fcmp olt float %_0.i.i5811, 0.000000e+00, !dbg !10219
  %_0.i.i6427 = select i1 %_3.i.i6420, float %_0.i.i5811, float 0.000000e+00, !dbg !10222
  %791 = bitcast i32 %gain_near.i15.i516.i.sroa.0.011107 to float, !dbg !10224
  %_3.i2495 = fcmp uge float %_0.i.i6427, %791, !dbg !10225
  %_4.i4906.v = select i1 %_3.i2495, float %coefficients.i529.i.sroa.7.0.copyload, float %coefficients.i529.i.sroa.5.0.copyload, !dbg !10228
  %_0.i3464 = fsub float %791, %_0.i.i6427, !dbg !10230
  %_4.i2983 = fmul float %_0.i3464, %_4.i4906.v, !dbg !10232
  %_0.i2984 = fadd float %_0.i.i6427, %_4.i2983, !dbg !10232
  %792 = tail call noundef float @llvm.fabs.f32(float %_0.i2984), !dbg !10234
  %_4.i4437 = bitcast float %_0.i2984 to i32, !dbg !10237
  %793 = fcmp uge float %792, 0x3BC79CA100000000, !dbg !10240
  %_3.i4439 = select i1 %793, i32 %_4.i4437, i32 0, !dbg !10241
  %_0.i4440 = bitcast i32 %_3.i4439 to float, !dbg !10242
  %_0.i2818 = fadd float %_0.i2812.4, %_0.i4440, !dbg !10244
  %_0.i3323 = fmul float %_0.i2818, 0x3FC542A5A0000000, !dbg !10246
  %_3.i.i5154.inv = fcmp ogt float %_0.i3323, -1.260000e+02, !dbg !10249
  %_0.i.i5161 = select i1 %_3.i.i5154.inv, float %_0.i3323, float -1.260000e+02, !dbg !10249
  %_3.i.i5956.inv = fcmp olt float %_0.i.i5161, 1.270000e+02, !dbg !10253
  %_0.i.i5963 = select i1 %_3.i.i5956.inv, float %_0.i.i5161, float 1.270000e+02, !dbg !10253
  %794 = tail call noundef float @llvm.floor.f32(float %_0.i.i5963), !dbg !10256
  %_0.i3365 = fsub float %_0.i.i5963, %794, !dbg !10260
  %795 = tail call noundef float @llvm.fabs.f32(float %_0.i3739), !dbg !10262
  %796 = tail call noundef float @llvm.fabs.f32(float %_0.i3729), !dbg !10265
  %_0.i3216 = fmul float %795, 5.000000e-01, !dbg !10267
  %_0.i3215 = fmul float %796, 5.000000e-01, !dbg !10269
  %_0.i2636 = fadd float %_0.i3216, %_0.i3215, !dbg !10271
  %_3.i.i5796 = fcmp ule float %_0.i2636, 0x3E45798EE0000000, !dbg !10273
  %_6.i.i5798 = bitcast float %_0.i2636 to i32, !dbg !10278
  %_4.i.i5802 = select i1 %_3.i.i5796, i32 841731191, i32 %_6.i.i5798, !dbg !10281
  %_0.i.i5803 = bitcast i32 %_4.i.i5802 to float, !dbg !10282
  %_3.i.i4970 = fcmp ule float %_0.i.i5803, 0x3810000000000000, !dbg !10284
  %_4.i.i4976 = select i1 %_3.i.i4970, i32 8388608, i32 %_4.i.i5802, !dbg !10289
  %_5.i3845 = and i32 %_4.i.i4976, 8388607, !dbg !10291
  %_4.i3846 = or disjoint i32 %_5.i3845, 1065353216, !dbg !10291
  %significand.i3847 = bitcast i32 %_4.i3846 to float, !dbg !10293
  %_0.i3342 = fadd float %significand.i3847, -1.000000e+00, !dbg !10295
  %_0.i3004 = fmul float %_0.i3342, 0xBF9B17A960000000, !dbg !10297
  %_0.i2524 = fadd float %_0.i3004, 0x3FBF9A8440000000, !dbg !10299
  %_0.i3004.1 = fmul float %_0.i3342, %_0.i2524, !dbg !10297
  %_0.i2524.1 = fadd float %_0.i3004.1, 0xBFD1E3F400000000, !dbg !10299
  %_0.i3004.2 = fmul float %_0.i3342, %_0.i2524.1, !dbg !10297
  %_0.i2524.2 = fadd float %_0.i3004.2, 0x3FDD544F20000000, !dbg !10299
  %_0.i3004.3 = fmul float %_0.i3342, %_0.i2524.2, !dbg !10297
  %_0.i2524.3 = fadd float %_0.i3004.3, 0xBFE6FC2A60000000, !dbg !10299
  %_0.i3004.4 = fmul float %_0.i3342, %_0.i2524.3, !dbg !10297
  %_0.i2524.4 = fadd float %_0.i3004.4, 0x3FF714B2A0000000, !dbg !10299
  %_9.i3848 = lshr i32 %_4.i.i4976, 23, !dbg !10301
  %_8.i3849 = or disjoint i32 %_9.i3848, 1258291200, !dbg !10301
  %_7.i3850 = bitcast i32 %_8.i3849 to float, !dbg !10302
  %exponent.i3851 = fadd float %_7.i3850, 0xC160000FE0000000, !dbg !10304
  %_0.i3003 = fmul float %_0.i3342, %_0.i2524.4, !dbg !10305
  %_0.i2523 = fadd float %exponent.i3851, %_0.i3003, !dbg !10307
  %_0.i3322 = fmul float %_0.i2523, 0x4018151820000000, !dbg !10309
  %_3.i.i5788.inv = fcmp ogt float %_0.i3322, -1.600000e+02, !dbg !10311
  %_0.i.i5795 = select i1 %_3.i.i5788.inv, float %_0.i3322, float -1.600000e+02, !dbg !10311
  %_3.i.i6412.inv = fcmp olt float %_0.i.i5795, 2.400000e+01, !dbg !10314
  %_0.i.i6419 = select i1 %_3.i.i6412.inv, float %_0.i.i5795, float 2.400000e+01, !dbg !10314
  %_0.i3390 = fsub float %_0.i.i6419, %_0.i2812.5, !dbg !10317
  %_3.i2235 = fcmp ule float %_0.i3390, 3.000000e+00, !dbg !10320
  %_0.i2614 = fadd float %_0.i3390, 3.000000e+00, !dbg !10322
  %_0.i3135 = fmul float %_0.i2614, %_0.i2614, !dbg !10324
  %_0.i3134 = fmul float %_0.i3135, 0x3FB5555560000000, !dbg !10326
  %_4.i4533.v.v = select i1 %_3.i2235, float %_0.i3134, float %_0.i3390, !dbg !10328
  %_4.i4533.v = fmul float %coefficients.i529.i.sroa.9.0.copyload, %_4.i4533.v.v, !dbg !10328
  %_4.i4533 = bitcast float %_4.i4533.v to i32, !dbg !10328
  %797 = fcmp ugt float %_0.i3390, -3.000000e+00, !dbg !10330
  %_7.i4525 = select i1 %797, i32 %_4.i4533, i32 0, !dbg !10332
  %_0.i4527 = bitcast i32 %_7.i4525 to float, !dbg !10333
  %_3.i.i5780 = fcmp ule float %_0.i4527, -1.000000e+02, !dbg !10335
  %798 = bitcast i32 %_7.i4525 to float, !dbg !10338
  %_0.i.i5787 = select i1 %_3.i.i5780, float -1.000000e+02, float %798, !dbg !10341
  %_3.i.i6404 = fcmp olt float %_0.i.i5787, 0.000000e+00, !dbg !10342
  %_0.i.i6411 = select i1 %_3.i.i6404, float %_0.i.i5787, float 0.000000e+00, !dbg !10345
  %799 = bitcast i32 %gain_near.i15.i516.i.sroa.6.011108 to float, !dbg !10347
  %_3.i2491 = fcmp uge float %_0.i.i6411, %799, !dbg !10348
  %_4.i4899.v = select i1 %_3.i2491, float %coefficients.i529.i.sroa.13.0.copyload, float %coefficients.i529.i.sroa.11.0.copyload, !dbg !10351
  %_0.i3463 = fsub float %799, %_0.i.i6411, !dbg !10353
  %_4.i2981 = fmul float %_0.i3463, %_4.i4899.v, !dbg !10355
  %_0.i2982 = fadd float %_0.i.i6411, %_4.i2981, !dbg !10355
  %800 = tail call noundef float @llvm.fabs.f32(float %_0.i2982), !dbg !10357
  %_4.i4433 = bitcast float %_0.i2982 to i32, !dbg !10360
  %801 = fcmp uge float %800, 0x3BC79CA100000000, !dbg !10363
  %_3.i4435 = select i1 %801, i32 %_4.i4433, i32 0, !dbg !10364
  %_0.i4436 = bitcast i32 %_3.i4435 to float, !dbg !10365
  %_0.i2817 = fadd float %_0.i2812.9, %_0.i4436, !dbg !10367
  %_0.i3321 = fmul float %_0.i2817, 0x3FC542A5A0000000, !dbg !10369
  %_3.i.i5162.inv = fcmp ogt float %_0.i3321, -1.260000e+02, !dbg !10372
  %_0.i.i5169 = select i1 %_3.i.i5162.inv, float %_0.i3321, float -1.260000e+02, !dbg !10372
  %_3.i.i5964.inv = fcmp olt float %_0.i.i5169, 1.270000e+02, !dbg !10376
  %_0.i.i5971 = select i1 %_3.i.i5964.inv, float %_0.i.i5169, float 1.270000e+02, !dbg !10376
  %802 = tail call noundef float @llvm.floor.f32(float %_0.i.i5971), !dbg !10379
  %_0.i3366 = fsub float %_0.i.i5971, %802, !dbg !10383
  %_0.i3391 = fsub float %_0.i.i6435, %_0.i2811, !dbg !10385
  %_3.i2237 = fcmp ule float %_0.i3391, 3.000000e+00, !dbg !10390
  %_0.i2615 = fadd float %_0.i3391, 3.000000e+00, !dbg !10392
  %_0.i3139 = fmul float %_0.i2615, %_0.i2615, !dbg !10394
  %_0.i3138 = fmul float %_0.i3139, 0x3FB5555560000000, !dbg !10396
  %_4.i4546.v.v = select i1 %_3.i2237, float %_0.i3138, float %_0.i3391, !dbg !10398
  %_4.i4546.v = fmul float %coefficients.i529.i.sroa.15.24.copyload, %_4.i4546.v.v, !dbg !10398
  %_4.i4546 = bitcast float %_4.i4546.v to i32, !dbg !10398
  %803 = fcmp ugt float %_0.i3391, -3.000000e+00, !dbg !10400
  %_7.i4538 = select i1 %803, i32 %_4.i4546, i32 0, !dbg !10402
  %_0.i4540 = bitcast i32 %_7.i4538 to float, !dbg !10403
  %_3.i.i5757 = fcmp ule float %_0.i4540, -1.000000e+02, !dbg !10405
  %804 = bitcast i32 %_7.i4538 to float, !dbg !10408
  %_0.i.i5763 = select i1 %_3.i.i5757, float -1.000000e+02, float %804, !dbg !10411
  %_3.i.i6388 = fcmp olt float %_0.i.i5763, 0.000000e+00, !dbg !10412
  %_0.i.i6395 = select i1 %_3.i.i6388, float %_0.i.i5763, float 0.000000e+00, !dbg !10415
  %805 = bitcast i32 %gain_far.i14.i515.i.sroa.0.011109 to float, !dbg !10417
  %_3.i2487 = fcmp uge float %_0.i.i6395, %805, !dbg !10418
  %_4.i4892.v = select i1 %_3.i2487, float %coefficients.i529.i.sroa.20.24.copyload, float %coefficients.i529.i.sroa.18.24.copyload, !dbg !10421
  %_0.i3462 = fsub float %805, %_0.i.i6395, !dbg !10423
  %_4.i2979 = fmul float %_0.i3462, %_4.i4892.v, !dbg !10425
  %_0.i2980 = fadd float %_0.i.i6395, %_4.i2979, !dbg !10425
  %806 = tail call noundef float @llvm.fabs.f32(float %_0.i2980), !dbg !10427
  %_4.i4429 = bitcast float %_0.i2980 to i32, !dbg !10430
  %807 = fcmp uge float %806, 0x3BC79CA100000000, !dbg !10433
  %_3.i4431 = select i1 %807, i32 %_4.i4429, i32 0, !dbg !10434
  %_0.i4432 = bitcast i32 %_3.i4431 to float, !dbg !10435
  %_0.i2816 = fadd float %_0.i2811.4, %_0.i4432, !dbg !10437
  %_0.i3319 = fmul float %_0.i2816, 0x3FC542A5A0000000, !dbg !10439
  %_3.i.i5170.inv = fcmp ogt float %_0.i3319, -1.260000e+02, !dbg !10442
  %_0.i.i5177 = select i1 %_3.i.i5170.inv, float %_0.i3319, float -1.260000e+02, !dbg !10442
  %_3.i.i5972.inv = fcmp olt float %_0.i.i5177, 1.270000e+02, !dbg !10446
  %_0.i.i5979 = select i1 %_3.i.i5972.inv, float %_0.i.i5177, float 1.270000e+02, !dbg !10446
  %808 = tail call noundef float @llvm.floor.f32(float %_0.i.i5979), !dbg !10449
  %_0.i3367 = fsub float %_0.i.i5979, %808, !dbg !10453
  %_0.i3392 = fsub float %_0.i.i6419, %_0.i2811.5, !dbg !10455
  %_3.i2239 = fcmp ule float %_0.i3392, 3.000000e+00, !dbg !10460
  %_0.i2616 = fadd float %_0.i3392, 3.000000e+00, !dbg !10462
  %_0.i3143 = fmul float %_0.i2616, %_0.i2616, !dbg !10464
  %_0.i3142 = fmul float %_0.i3143, 0x3FB5555560000000, !dbg !10466
  %_4.i4559.v.v = select i1 %_3.i2239, float %_0.i3142, float %_0.i3392, !dbg !10468
  %_4.i4559.v = fmul float %coefficients.i529.i.sroa.22.24.copyload, %_4.i4559.v.v, !dbg !10468
  %_4.i4559 = bitcast float %_4.i4559.v to i32, !dbg !10468
  %809 = fcmp ugt float %_0.i3392, -3.000000e+00, !dbg !10470
  %_7.i4551 = select i1 %809, i32 %_4.i4559, i32 0, !dbg !10472
  %_0.i4553 = bitcast i32 %_7.i4551 to float, !dbg !10473
  %_3.i.i5734 = fcmp ule float %_0.i4553, -1.000000e+02, !dbg !10475
  %810 = bitcast i32 %_7.i4551 to float, !dbg !10478
  %_0.i.i5741 = select i1 %_3.i.i5734, float -1.000000e+02, float %810, !dbg !10481
  %_3.i.i6372 = fcmp olt float %_0.i.i5741, 0.000000e+00, !dbg !10482
  %_0.i.i6379 = select i1 %_3.i.i6372, float %_0.i.i5741, float 0.000000e+00, !dbg !10485
  %811 = bitcast i32 %gain_far.i14.i515.i.sroa.6.011110 to float, !dbg !10487
  %_3.i2483 = fcmp uge float %_0.i.i6379, %811, !dbg !10488
  %_4.i4885.v = select i1 %_3.i2483, float %coefficients.i529.i.sroa.26.24.copyload, float %coefficients.i529.i.sroa.24.24.copyload, !dbg !10491
  %_0.i3461 = fsub float %811, %_0.i.i6379, !dbg !10493
  %_4.i2977 = fmul float %_0.i3461, %_4.i4885.v, !dbg !10495
  %_0.i2978 = fadd float %_0.i.i6379, %_4.i2977, !dbg !10495
  %812 = tail call noundef float @llvm.fabs.f32(float %_0.i2978), !dbg !10497
  %_4.i4425 = bitcast float %_0.i2978 to i32, !dbg !10500
  %813 = fcmp uge float %812, 0x3BC79CA100000000, !dbg !10503
  %_3.i4427 = select i1 %813, i32 %_4.i4425, i32 0, !dbg !10504
  %_0.i4428 = bitcast i32 %_3.i4427 to float, !dbg !10505
  %_0.i2815 = fadd float %_0.i2811.9, %_0.i4428, !dbg !10507
  %_0.i3317 = fmul float %_0.i2815, 0x3FC542A5A0000000, !dbg !10509
  %_3.i.i5178.inv = fcmp ogt float %_0.i3317, -1.260000e+02, !dbg !10512
  %_0.i.i5185 = select i1 %_3.i.i5178.inv, float %_0.i3317, float -1.260000e+02, !dbg !10512
  %_3.i.i5980.inv = fcmp olt float %_0.i.i5185, 1.270000e+02, !dbg !10516
  %_0.i.i5987 = select i1 %_3.i.i5980.inv, float %_0.i.i5185, float 1.270000e+02, !dbg !10516
  %814 = tail call noundef float @llvm.floor.f32(float %_0.i.i5987), !dbg !10519
  %_0.i3368 = fsub float %_0.i.i5987, %814, !dbg !10523
  %_0.i3064 = fmul float %_0.i3368, 0x3F5E974FA0000000, !dbg !10525
  %_0.i2576 = fadd float %_0.i3064, 0x3F82778560000000, !dbg !10527
  %_0.i3064.1 = fmul float %_0.i3368, %_0.i2576, !dbg !10525
  %_0.i2576.1 = fadd float %_0.i3064.1, 0x3FAC91CE60000000, !dbg !10527
  %_0.i3064.2 = fmul float %_0.i3368, %_0.i2576.1, !dbg !10525
  %_0.i2576.2 = fadd float %_0.i3064.2, 0x3FCEBDB560000000, !dbg !10527
  %_0.i3064.3 = fmul float %_0.i3368, %_0.i2576.2, !dbg !10525
  %_0.i2576.3 = fadd float %_0.i3064.3, 0x3FE62E4BA0000000, !dbg !10527
  %_0.i3061 = fmul float %_0.i3367, 0x3F5E974FA0000000, !dbg !10529
  %_0.i2574 = fadd float %_0.i3061, 0x3F82778560000000, !dbg !10531
  %_0.i3061.1 = fmul float %_0.i3367, %_0.i2574, !dbg !10529
  %_0.i2574.1 = fadd float %_0.i3061.1, 0x3FAC91CE60000000, !dbg !10531
  %_0.i3061.2 = fmul float %_0.i3367, %_0.i2574.1, !dbg !10529
  %_0.i2574.2 = fadd float %_0.i3061.2, 0x3FCEBDB560000000, !dbg !10531
  %_0.i3061.3 = fmul float %_0.i3367, %_0.i2574.2, !dbg !10529
  %_0.i2574.3 = fadd float %_0.i3061.3, 0x3FE62E4BA0000000, !dbg !10531
  %_0.i3058 = fmul float %_0.i3366, 0x3F5E974FA0000000, !dbg !10533
  %_0.i2572 = fadd float %_0.i3058, 0x3F82778560000000, !dbg !10535
  %_0.i3058.1 = fmul float %_0.i3366, %_0.i2572, !dbg !10533
  %_0.i2572.1 = fadd float %_0.i3058.1, 0x3FAC91CE60000000, !dbg !10535
  %_0.i3058.2 = fmul float %_0.i3366, %_0.i2572.1, !dbg !10533
  %_0.i2572.2 = fadd float %_0.i3058.2, 0x3FCEBDB560000000, !dbg !10535
  %_0.i3058.3 = fmul float %_0.i3366, %_0.i2572.2, !dbg !10533
  %_0.i2572.3 = fadd float %_0.i3058.3, 0x3FE62E4BA0000000, !dbg !10535
  %_0.i3055 = fmul float %_0.i3365, 0x3F5E974FA0000000, !dbg !10537
  %_0.i2570 = fadd float %_0.i3055, 0x3F82778560000000, !dbg !10539
  %_0.i3055.1 = fmul float %_0.i3365, %_0.i2570, !dbg !10537
  %_0.i2570.1 = fadd float %_0.i3055.1, 0x3FAC91CE60000000, !dbg !10539
  %_0.i3055.2 = fmul float %_0.i3365, %_0.i2570.1, !dbg !10537
  %_0.i2570.2 = fadd float %_0.i3055.2, 0x3FCEBDB560000000, !dbg !10539
  %_0.i3055.3 = fmul float %_0.i3365, %_0.i2570.2, !dbg !10537
  %_0.i2570.3 = fadd float %_0.i3055.3, 0x3FE62E4BA0000000, !dbg !10539
  %_0.i3054 = fmul float %_0.i3365, %_0.i2570.3, !dbg !10541
  %_0.i2569 = fadd float %_0.i3054, 1.000000e+00, !dbg !10543
  %biased.i2146 = fadd float %794, 0x4160000FE0000000, !dbg !10545
  %_4.i2147 = bitcast float %biased.i2146 to i32, !dbg !10547
  %_3.i2148 = shl i32 %_4.i2147, 23, !dbg !10549
  %_0.i2149 = bitcast i32 %_3.i2148 to float, !dbg !10550
  %_0.i3053 = fmul float %_0.i2569, %_0.i2149, !dbg !10552
  %_0.i3057 = fmul float %_0.i3366, %_0.i2572.3, !dbg !10554
  %_0.i2571 = fadd float %_0.i3057, 1.000000e+00, !dbg !10556
  %biased.i2150 = fadd float %802, 0x4160000FE0000000, !dbg !10558
  %_4.i2151 = bitcast float %biased.i2150 to i32, !dbg !10560
  %_3.i2152 = shl i32 %_4.i2151, 23, !dbg !10562
  %_0.i2153 = bitcast i32 %_3.i2152 to float, !dbg !10563
  %_0.i3056 = fmul float %_0.i2571, %_0.i2153, !dbg !10565
  %_0.i3060 = fmul float %_0.i3367, %_0.i2574.3, !dbg !10567
  %_0.i2573 = fadd float %_0.i3060, 1.000000e+00, !dbg !10569
  %biased.i2154 = fadd float %808, 0x4160000FE0000000, !dbg !10571
  %_4.i2155 = bitcast float %biased.i2154 to i32, !dbg !10573
  %_3.i2156 = shl i32 %_4.i2155, 23, !dbg !10575
  %_0.i2157 = bitcast i32 %_3.i2156 to float, !dbg !10576
  %_0.i3059 = fmul float %_0.i2573, %_0.i2157, !dbg !10578
  %_0.i3063 = fmul float %_0.i3368, %_0.i2576.3, !dbg !10580
  %_0.i2575 = fadd float %_0.i3063, 1.000000e+00, !dbg !10582
  %biased.i2158 = fadd float %814, 0x4160000FE0000000, !dbg !10584
  %_4.i2159 = bitcast float %biased.i2158 to i32, !dbg !10586
  %_3.i2160 = shl i32 %_4.i2159, 23, !dbg !10588
  %_0.i2161 = bitcast i32 %_3.i2160 to float, !dbg !10589
  %_0.i3062 = fmul float %_0.i2575, %_0.i2161, !dbg !10591
  %_266.i112.i861.i = icmp ugt i64 %_41.sroa.0.0.i40.i761.i, %_315.1.i80.i801.i, !dbg !10593
  br i1 %_266.i112.i861.i, label %bb86.i149.i898.i, label %bb87.i113.i862.i, !dbg !10593, !prof !1664

bb84.i150.i903.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4137
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %position.sroa.0.0.i32.i748.i11111, i64 noundef %_314.1.i73.i794.i, i64 noundef %_314.1.i73.i794.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e11ba0c4c1124bbcae4603a2ae121338) #26, !dbg !10598, !noalias !9784
  unreachable, !dbg !10598

bb87.i113.i862.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3731
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10599), !dbg !10602
  %_3.not.i3722 = icmp eq i64 %_315.1.i80.i801.i, %_41.sroa.0.0.i40.i761.i, !dbg !10603
  br i1 %_3.not.i3722, label %panic.i3725, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3726, !dbg !10603

panic.i3725:                                      ; preds = %bb87.i113.i862.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10603, !noalias !10605
  unreachable, !dbg !10603

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3726: ; preds = %bb87.i113.i862.i
  %_273.i116.i865.i = getelementptr inbounds nuw float, ptr %_315.0.i79.i800.i, i64 %_41.sroa.0.0.i40.i761.i, !dbg !10606
  %_0.i3724 = load float, ptr %_273.i116.i865.i, align 4, !dbg !10603, !alias.scope !10599, !noalias !9784, !noundef !11
  %_0.i3316 = fmul float %_0.i3053, %_0.i3724, !dbg !10611
  %_274.i120.i869.i = icmp ugt i64 %_41.sroa.0.0.i40.i761.i, %_316.1.i83.i811.i, !dbg !10613
  br i1 %_274.i120.i869.i, label %bb88.i148.i897.i, label %bb89.i121.i870.i, !dbg !10613, !prof !1664

bb86.i149.i898.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3731
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i761.i, i64 noundef %_315.1.i80.i801.i, i64 noundef %_315.1.i80.i801.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_379b93204ee4659b79f4b07b6520076a) #26, !dbg !10617, !noalias !9784
  unreachable, !dbg !10617

bb89.i121.i870.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3726
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10618), !dbg !10621
  %_3.not.i3717 = icmp eq i64 %_316.1.i83.i811.i, %_41.sroa.0.0.i40.i761.i, !dbg !10622
  br i1 %_3.not.i3717, label %panic.i3720, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3721, !dbg !10622

panic.i3720:                                      ; preds = %bb89.i121.i870.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10622, !noalias !10624
  unreachable, !dbg !10622

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3721: ; preds = %bb89.i121.i870.i
  %_281.i124.i873.i = getelementptr inbounds nuw float, ptr %_316.0.i82.i810.i, i64 %_41.sroa.0.0.i40.i761.i, !dbg !10625
  %_0.i3719 = load float, ptr %_281.i124.i873.i, align 4, !dbg !10622, !alias.scope !10618, !noalias !9784, !noundef !11
  %_0.i3315 = fmul float %_0.i3056, %_0.i3719, !dbg !10630
  %_0.i2814 = fadd float %_0.i3316, %_0.i3315, !dbg !10632
  %_282.i129.i878.i = icmp ugt i64 %_41.sroa.0.0.i40.i761.i, %_317.1.i86.i821.i, !dbg !10634
  br i1 %_282.i129.i878.i, label %bb90.i147.i896.i, label %bb91.i130.i879.i, !dbg !10634, !prof !1664

bb88.i148.i897.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3726
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i761.i, i64 noundef %_316.1.i83.i811.i, i64 noundef %_316.1.i83.i811.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2bb8eb0542a889f29ef069491eb97a17) #26, !dbg !10639, !noalias !9784
  unreachable, !dbg !10639

bb91.i130.i879.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3721
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10640), !dbg !10643
  %_3.not.i3712 = icmp eq i64 %_317.1.i86.i821.i, %_41.sroa.0.0.i40.i761.i, !dbg !10644
  br i1 %_3.not.i3712, label %panic.i3715, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3716, !dbg !10644

panic.i3715:                                      ; preds = %bb91.i130.i879.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10644, !noalias !10646
  unreachable, !dbg !10644

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3716: ; preds = %bb91.i130.i879.i
  %_289.i133.i882.i = getelementptr inbounds nuw float, ptr %_317.0.i85.i820.i, i64 %_41.sroa.0.0.i40.i761.i, !dbg !10647
  %_0.i3714 = load float, ptr %_289.i133.i882.i, align 4, !dbg !10644, !alias.scope !10640, !noalias !9784, !noundef !11
  %_0.i3314 = fmul float %_0.i3059, %_0.i3714, !dbg !10652
  %_290.i137.i886.i = icmp ugt i64 %_41.sroa.0.0.i40.i761.i, %_318.1.i89.i831.i, !dbg !10654
  br i1 %_290.i137.i886.i, label %bb92.i146.i895.i, label %bb93.i138.i887.i, !dbg !10654, !prof !1664

bb90.i147.i896.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3721
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i761.i, i64 noundef %_317.1.i86.i821.i, i64 noundef %_317.1.i86.i821.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cd96798e807157d7db308788cb4dd125) #26, !dbg !10658, !noalias !9784
  unreachable, !dbg !10658

bb93.i138.i887.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3716
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10659), !dbg !10662
  %_3.not.i3707 = icmp eq i64 %_318.1.i89.i831.i, %_41.sroa.0.0.i40.i761.i, !dbg !10663
  br i1 %_3.not.i3707, label %panic.i3710, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125, !dbg !10663

panic.i3710:                                      ; preds = %bb93.i138.i887.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_448b96be3252b33bba65fff6359cc55f) #26, !dbg !10663, !noalias !10665
  unreachable, !dbg !10663

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125: ; preds = %bb93.i138.i887.i
  %_297.i141.i890.i = getelementptr inbounds nuw float, ptr %_318.0.i88.i830.i, i64 %_41.sroa.0.0.i40.i761.i, !dbg !10666
  %_0.i3709 = load float, ptr %_297.i141.i890.i, align 4, !dbg !10663, !alias.scope !10659, !noalias !9784, !noundef !11
  %_0.i3313 = fmul float %_0.i3062, %_0.i3709, !dbg !10671
  %_0.i2813 = fadd float %_0.i3314, %_0.i3313, !dbg !10673
  store float %_0.i2814, ptr %_184.i42.i765.i, align 4, !dbg !10675, !alias.scope !10678, !noalias !9784
  store float %_0.i2813, ptr %_192.i47.i768.i, align 4, !dbg !10681, !alias.scope !10683, !noalias !9784
  %exitcond14306.not = icmp eq i64 %767, %plan.0.i540.i, !dbg !9736
  br i1 %exitcond14306.not, label %_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_Kb1_EB2_.exit.i.i, label %bb56.i37.i754.i, !dbg !9748

bb92.i146.i895.i:                                 ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit3716
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_41.sroa.0.0.i40.i761.i, i64 noundef %_318.1.i89.i831.i, i64 noundef %_318.1.i89.i831.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_164adf6876ccf79975d46e129e248ca5) #26, !dbg !10686, !noalias !9784
  unreachable, !dbg !10686

_RINvCs5xZ4lC6BZIV_20multiband_compressor11run_segmentfKj1_Kh2_Kb0_Kb1_EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125, %bb17.i730.i
  %segments.i532.i.sroa.0.0 = phi float [ %_12.le.i6860, %bb17.i730.i ], [ %_0.i2812, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.8.0 = phi float [ %_12.le.1.i6862, %bb17.i730.i ], [ %_0.i2812.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.12.0 = phi float [ %_12.le.2.i6864, %bb17.i730.i ], [ %_0.i2812.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.16.0 = phi float [ %_12.le.3.i6866, %bb17.i730.i ], [ %_0.i2812.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.20.0 = phi float [ %_12.le.4.i6868, %bb17.i730.i ], [ %_0.i2812.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.26.0 = phi float [ %_12.le.5.i6870, %bb17.i730.i ], [ %_0.i2812.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.32.0 = phi float [ %_12.le.6.i6872, %bb17.i730.i ], [ %_0.i2812.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.36.0 = phi float [ %_12.le.7.i6874, %bb17.i730.i ], [ %_0.i2812.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.40.0 = phi float [ %_12.le.8.i6876, %bb17.i730.i ], [ %_0.i2812.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.44.0 = phi float [ %_12.le.9.i6878, %bb17.i730.i ], [ %_0.i2812.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.70.0 = phi float [ %_12.le.i6898, %bb17.i730.i ], [ %_0.i2811, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.76.0 = phi float [ %_12.le.1.i6900, %bb17.i730.i ], [ %_0.i2811.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.80.0 = phi float [ %_12.le.2.i6902, %bb17.i730.i ], [ %_0.i2811.2, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.84.0 = phi float [ %_12.le.3.i6904, %bb17.i730.i ], [ %_0.i2811.3, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.88.0 = phi float [ %_12.le.4.i6906, %bb17.i730.i ], [ %_0.i2811.4, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.94.0 = phi float [ %_12.le.5.i6908, %bb17.i730.i ], [ %_0.i2811.5, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.100.0 = phi float [ %_12.le.6.i6910, %bb17.i730.i ], [ %_0.i2811.6, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.104.0 = phi float [ %_12.le.7.i6912, %bb17.i730.i ], [ %_0.i2811.7, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.108.0 = phi float [ %_12.le.8.i6914, %bb17.i730.i ], [ %_0.i2811.8, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %segments.i532.i.sroa.112.0 = phi float [ %_12.le.9.i6916, %bb17.i730.i ], [ %_0.i2811.9, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !9750
  %filter_near.i17.i518.i.sroa.0.0.lcssa = phi float [ %filter_near.i17.i518.i.sroa.0.0.copyload, %bb17.i730.i ], [ %_0.i4320, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10687
  %filter_near.i17.i518.i.sroa.7.0.lcssa = phi float [ %filter_near.i17.i518.i.sroa.7.0.copyload, %bb17.i730.i ], [ %_0.i4316, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10687
  %filter_near.i17.i518.i.sroa.11.0.lcssa = phi float [ %filter_near.i17.i518.i.sroa.11.0.copyload, %bb17.i730.i ], [ %_0.i4328, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10687
  %filter_near.i17.i518.i.sroa.14.0.lcssa = phi float [ %filter_near.i17.i518.i.sroa.14.0.copyload, %bb17.i730.i ], [ %_0.i4324, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10687
  %filter_far.i16.i517.i.sroa.0.0.lcssa = phi float [ %filter_far.i16.i517.i.sroa.0.0.copyload, %bb17.i730.i ], [ %_0.i4304, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10688
  %filter_far.i16.i517.i.sroa.7.0.lcssa = phi float [ %filter_far.i16.i517.i.sroa.7.0.copyload, %bb17.i730.i ], [ %_0.i4300, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10688
  %filter_far.i16.i517.i.sroa.11.0.lcssa = phi float [ %filter_far.i16.i517.i.sroa.11.0.copyload, %bb17.i730.i ], [ %_0.i4312, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10688
  %filter_far.i16.i517.i.sroa.14.0.lcssa = phi float [ %filter_far.i16.i517.i.sroa.14.0.copyload, %bb17.i730.i ], [ %_0.i4308, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10688
  %gain_near.i15.i516.i.sroa.0.0.lcssa = phi i32 [ %762, %bb17.i730.i ], [ %_3.i4439, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10689
  %gain_near.i15.i516.i.sroa.6.0.lcssa = phi i32 [ %763, %bb17.i730.i ], [ %_3.i4435, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10689
  %gain_far.i14.i515.i.sroa.0.0.lcssa = phi i32 [ %764, %bb17.i730.i ], [ %_3.i4431, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10690
  %gain_far.i14.i515.i.sroa.6.0.lcssa = phi i32 [ %765, %bb17.i730.i ], [ %_3.i4427, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10690
  %position.sroa.0.0.i32.i748.i.lcssa = phi i64 [ %766, %bb17.i730.i ], [ %_41.sroa.0.0.i40.i761.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane5store.exit4125 ], !dbg !10691
  store float %filter_near.i17.i518.i.sroa.0.0.lcssa, ptr %672, align 8, !dbg !10692, !noalias !9784
  store float %filter_near.i17.i518.i.sroa.7.0.lcssa, ptr %filter_near.i.i525.i.sroa.7.0..sroa_idx, align 4, !dbg !10692, !noalias !9784
  store float %filter_near.i17.i518.i.sroa.11.0.lcssa, ptr %filter_near.i.i525.i.sroa.11.0..sroa_idx, align 8, !dbg !10692, !noalias !9784
  store float %filter_near.i17.i518.i.sroa.14.0.lcssa, ptr %filter_near.i.i525.i.sroa.14.0..sroa_idx, align 4, !dbg !10692, !noalias !9784
  store float %filter_far.i16.i517.i.sroa.0.0.lcssa, ptr %673, align 8, !dbg !10693, !noalias !9784
  store float %filter_far.i16.i517.i.sroa.7.0.lcssa, ptr %filter_far.i.i524.i.sroa.7.0..sroa_idx, align 4, !dbg !10693, !noalias !9784
  store float %filter_far.i16.i517.i.sroa.11.0.lcssa, ptr %filter_far.i.i524.i.sroa.11.0..sroa_idx, align 8, !dbg !10693, !noalias !9784
  store float %filter_far.i16.i517.i.sroa.14.0.lcssa, ptr %filter_far.i.i524.i.sroa.14.0..sroa_idx, align 4, !dbg !10693, !noalias !9784
  store i32 %gain_near.i15.i516.i.sroa.0.0.lcssa, ptr %674, align 8, !dbg !10694, !noalias !9784
  store i32 %gain_near.i15.i516.i.sroa.6.0.lcssa, ptr %.sroa_idx7097, align 4, !dbg !10694, !noalias !9784
  store i32 %gain_far.i14.i515.i.sroa.0.0.lcssa, ptr %675, align 8, !dbg !10695, !noalias !9784
  store i32 %gain_far.i14.i515.i.sroa.6.0.lcssa, ptr %.sroa_idx7102, align 4, !dbg !10695, !noalias !9784
  store i64 %position.sroa.0.0.i32.i748.i.lcssa, ptr %_51.i544.i, align 8, !dbg !10696, !alias.scope !9733, !noalias !9734
  %advanced.i753.i = trunc i64 %plan.0.i540.i to i32, !dbg !10697
  store float %segments.i532.i.sroa.0.0, ptr %632, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.i6945 = load i32, ptr %688, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %815 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6945, i32 %advanced.i753.i), !dbg !10707
  store i32 %815, ptr %688, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.8.0, ptr %634, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.1.i6948 = load i32, ptr %689, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %816 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6948, i32 %advanced.i753.i), !dbg !10707
  store i32 %816, ptr %689, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.12.0, ptr %636, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.2.i6951 = load i32, ptr %690, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %817 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6951, i32 %advanced.i753.i), !dbg !10707
  store i32 %817, ptr %690, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.16.0, ptr %638, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.3.i6954 = load i32, ptr %691, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %818 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6954, i32 %advanced.i753.i), !dbg !10707
  store i32 %818, ptr %691, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.20.0, ptr %640, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.4.i6957 = load i32, ptr %692, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %819 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6957, i32 %advanced.i753.i), !dbg !10707
  store i32 %819, ptr %692, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.26.0, ptr %642, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.5.i6960 = load i32, ptr %693, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %820 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6960, i32 %advanced.i753.i), !dbg !10707
  store i32 %820, ptr %693, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.32.0, ptr %644, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.6.i6963 = load i32, ptr %694, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %821 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6963, i32 %advanced.i753.i), !dbg !10707
  store i32 %821, ptr %694, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.36.0, ptr %646, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.7.i6966 = load i32, ptr %695, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %822 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6966, i32 %advanced.i753.i), !dbg !10707
  store i32 %822, ptr %695, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.40.0, ptr %648, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.8.i6969 = load i32, ptr %696, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %823 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6969, i32 %advanced.i753.i), !dbg !10707
  store i32 %823, ptr %696, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.44.0, ptr %650, align 4, !dbg !10698, !alias.scope !10701, !noalias !10704
  %_26.9.i6972 = load i32, ptr %697, align 4, !dbg !10706, !alias.scope !10701, !noalias !10704, !noundef !11
  %824 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i6972, i32 %advanced.i753.i), !dbg !10707
  store i32 %824, ptr %697, align 4, !dbg !10709, !alias.scope !10701, !noalias !10704
  store float %segments.i532.i.sroa.70.0, ptr %652, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.i6974 = load i32, ptr %698, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %825 = tail call i32 @llvm.usub.sat.i32(i32 %_26.i6974, i32 %advanced.i753.i), !dbg !10718
  store i32 %825, ptr %698, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.76.0, ptr %654, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.1.i6977 = load i32, ptr %699, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %826 = tail call i32 @llvm.usub.sat.i32(i32 %_26.1.i6977, i32 %advanced.i753.i), !dbg !10718
  store i32 %826, ptr %699, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.80.0, ptr %656, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.2.i6980 = load i32, ptr %700, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %827 = tail call i32 @llvm.usub.sat.i32(i32 %_26.2.i6980, i32 %advanced.i753.i), !dbg !10718
  store i32 %827, ptr %700, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.84.0, ptr %658, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.3.i6983 = load i32, ptr %701, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %828 = tail call i32 @llvm.usub.sat.i32(i32 %_26.3.i6983, i32 %advanced.i753.i), !dbg !10718
  store i32 %828, ptr %701, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.88.0, ptr %660, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.4.i6986 = load i32, ptr %702, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %829 = tail call i32 @llvm.usub.sat.i32(i32 %_26.4.i6986, i32 %advanced.i753.i), !dbg !10718
  store i32 %829, ptr %702, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.94.0, ptr %662, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.5.i6989 = load i32, ptr %703, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %830 = tail call i32 @llvm.usub.sat.i32(i32 %_26.5.i6989, i32 %advanced.i753.i), !dbg !10718
  store i32 %830, ptr %703, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.100.0, ptr %664, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.6.i6992 = load i32, ptr %704, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %831 = tail call i32 @llvm.usub.sat.i32(i32 %_26.6.i6992, i32 %advanced.i753.i), !dbg !10718
  store i32 %831, ptr %704, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.104.0, ptr %666, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.7.i6995 = load i32, ptr %705, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %832 = tail call i32 @llvm.usub.sat.i32(i32 %_26.7.i6995, i32 %advanced.i753.i), !dbg !10718
  store i32 %832, ptr %705, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.108.0, ptr %668, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.8.i6998 = load i32, ptr %706, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %833 = tail call i32 @llvm.usub.sat.i32(i32 %_26.8.i6998, i32 %advanced.i753.i), !dbg !10718
  store i32 %833, ptr %706, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  store float %segments.i532.i.sroa.112.0, ptr %670, align 4, !dbg !10710, !alias.scope !10712, !noalias !10715
  %_26.9.i7001 = load i32, ptr %707, align 4, !dbg !10717, !alias.scope !10712, !noalias !10715, !noundef !11
  %834 = tail call i32 @llvm.usub.sat.i32(i32 %_26.9.i7001, i32 %advanced.i753.i), !dbg !10718
  store i32 %834, ptr %707, align 4, !dbg !10720, !alias.scope !10712, !noalias !10715
  br label %bb15.i572.i, !dbg !9699

_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit: ; preds = %bb15.i572.i, %bb15.i166.i, %bb15.i.i, %bb15.i43.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10721), !dbg !10724
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10725), !dbg !10724
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10727), !dbg !10724
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10729), !dbg !10732
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10737), !dbg !10732
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10739), !dbg !10732
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10741), !dbg !10732
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10743), !dbg !10732
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !10745

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit
  %ok.sroa.0.0.i1355.i.i = phi i32 [ -1, %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit ], [ %_0.i33.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ]
  %iter.sroa.0.0.i1254.i.i = phi ptr [ %_19.0, %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit ], [ %_27.i16.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ]
  %iter.sroa.5.0.i1153.i.i = phi i64 [ %_19.1, %_RINvCs5xZ4lC6BZIV_20multiband_compressor6renderfKj1_Kb0_EB2_.exit ], [ %_28.i17.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i ]
  %_27.i16.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i1254.i.i, i64 4, !dbg !10761
  %_28.i17.i.i = add nsw i64 %iter.sroa.5.0.i1153.i.i, -1, !dbg !10771
  %_0.i28.i.i = load float, ptr %iter.sroa.0.0.i1254.i.i, align 4, !dbg !10772, !alias.scope !10775, !noalias !10778, !noundef !11
  %835 = tail call noundef float @llvm.fabs.f32(float %_0.i28.i.i), !dbg !10780
  %_3.i.i.i = fcmp olt float %835, 0x46293E5940000000, !dbg !10783
  %_0.i33.i.i = select i1 %_3.i.i.i, i32 %ok.sroa.0.0.i1355.i.i, i32 0, !dbg !10785
  %_22.not.i14.i.i = icmp eq i64 %_28.i17.i.i, 0, !dbg !10745
  br i1 %_22.not.i14.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i, !dbg !10745

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i
  %sides.i = getelementptr inbounds nuw i8, ptr %self, i64 120, !dbg !10788
  %cursor.i = getelementptr inbounds nuw i8, ptr %self, i64 848, !dbg !10789
  %_0.i35.not.i.i = icmp eq i32 %_0.i33.i.i, -1, !dbg !10790
  br i1 %_0.i35.not.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i, label %bb7.i.i, !dbg !10793

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i
  %ok.sroa.0.0.i58.i.i = phi i32 [ %_0.i34.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i ], [ -1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i ]
  %iter.sroa.0.0.i57.i.i = phi ptr [ %_27.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i ], [ %_20.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i ]
  %iter.sroa.5.0.i56.i.i = phi i64 [ %_28.i.i.i7007, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i ], [ %_19.1, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i ]
  %_27.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i57.i.i, i64 4, !dbg !10794
  %_28.i.i.i7007 = add nsw i64 %iter.sroa.5.0.i56.i.i, -1, !dbg !10800
  %_0.i30.i.i = load float, ptr %iter.sroa.0.0.i57.i.i, align 4, !dbg !10801, !alias.scope !10803, !noalias !10806, !noundef !11
  %836 = tail call noundef float @llvm.fabs.f32(float %_0.i30.i.i), !dbg !10807
  %_3.i26.i.i = fcmp olt float %836, 0x46293E5940000000, !dbg !10809
  %_0.i34.i.i = select i1 %_3.i26.i.i, i32 %ok.sroa.0.0.i58.i.i, i32 0, !dbg !10811
  %_22.not.i.i.i = icmp eq i64 %_28.i.i.i7007, 0, !dbg !10813
  br i1 %_22.not.i.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i, !dbg !10813

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit32.i.i
  %_0.i36.not.i.i = icmp eq i32 %_0.i34.i.i, -1, !dbg !10814
  br i1 %_0.i36.not.i.i, label %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E6finishB5_.exit, label %bb7.i.i, !dbg !10816

bb7.i.i:                                          ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit25.i.i
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, !dbg !10817

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, %bb7.i.i
  %ok.sroa.0.016.i.i.i = phi i32 [ -1, %bb7.i.i ], [ %_0.i7.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ]
  %iter.sroa.0.015.i.i.i = phi ptr [ %_19.0, %bb7.i.i ], [ %_45.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ]
  %iter.sroa.5.014.i.i.i = phi i64 [ %_19.1, %bb7.i.i ], [ %_46.i.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i ]
  %_45.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.015.i.i.i, i64 4, !dbg !10828
  %_46.i.i.i = add nsw i64 %iter.sroa.5.014.i.i.i, -1, !dbg !10835
  %_0.i.i.i.i = load float, ptr %iter.sroa.0.015.i.i.i, align 4, !dbg !10836, !alias.scope !10839, !noalias !10778, !noundef !11
  %837 = tail call noundef float @llvm.fabs.f32(float %_0.i.i.i.i), !dbg !10844
  %_3.i.i.i.i = fcmp olt float %837, 0x46293E5940000000, !dbg !10847
  %_0.i7.i.i.i = select i1 %_3.i.i.i.i, i32 %ok.sroa.0.016.i.i.i, i32 0, !dbg !10849
  %_40.not.i.i.i = icmp eq i64 %_46.i.i.i, 0, !dbg !10817
  br i1 %_40.not.i.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i, !dbg !10817

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i.i.i
  br label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i, !dbg !10851

_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i
  %ok.sroa.0.016.i41.i.i = phi i32 [ %_0.i7.i48.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i ], [ -1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader ]
  %iter.sroa.0.015.i42.i.i = phi ptr [ %_45.i44.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i ], [ %_20.0, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader ]
  %iter.sroa.5.014.i43.i.i = phi i64 [ %_46.i45.i.i, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i ], [ %_19.1, %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i.preheader ]
  %_45.i44.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.015.i42.i.i, i64 4, !dbg !10855
  %_46.i45.i.i = add nsw i64 %iter.sroa.5.014.i43.i.i, -1, !dbg !10858
  %_0.i.i46.i.i = load float, ptr %iter.sroa.0.015.i42.i.i, align 4, !dbg !10859, !alias.scope !10861, !noalias !10806, !noundef !11
  %838 = tail call noundef float @llvm.fabs.f32(float %_0.i.i46.i.i), !dbg !10866
  %_3.i.i47.i.i = fcmp olt float %838, 0x46293E5940000000, !dbg !10868
  %_0.i7.i48.i.i = select i1 %_3.i.i47.i.i, i32 %ok.sroa.0.016.i41.i.i, i32 0, !dbg !10870
  %_40.not.i49.i.i = icmp eq i64 %_46.i45.i.i, 0, !dbg !10851
  br i1 %_40.not.i49.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECs5xZ4lC6BZIV_20multiband_compressor.exit51.i.i, label %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i, !dbg !10851

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECs5xZ4lC6BZIV_20multiband_compressor.exit51.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane6scalarfNtB4_4Lane4load.exit.i40.i.i
  %839 = getelementptr inbounds nuw i8, ptr %self, i64 112, !dbg !10872
  %840 = and i32 %_0.i7.i.i.i, 1065353216, !dbg !10872
  %841 = and i32 %840, %_0.i7.i48.i.i, !dbg !10872
  %842 = icmp ne i32 %841, 1065353216, !dbg !10872
  %843 = zext i1 %842 to i32, !dbg !10872
  store i32 %843, ptr %839, align 8, !dbg !10872, !alias.scope !10873, !noalias !10874
  %_14.i.i7002 = load i64, ptr %_5, align 8, !dbg !10875, !alias.scope !10873, !noalias !10874, !noundef !11
  %844 = tail call i64 @llvm.uadd.sat.i64(i64 %_14.i.i7002, i64 1), !dbg !10876
  store i64 %844, ptr %_5, align 8, !dbg !10879, !alias.scope !10873, !noalias !10874
  %.idx.i.i.i = shl nuw nsw i64 %_19.1, 2, !dbg !10880
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %_19.0, i8 0, i64 %.idx.i.i.i, i1 false), !dbg !10887, !alias.scope !10888, !noalias !10778
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1) %_20.0, i8 0, i64 %.idx.i.i.i, i1 false), !dbg !10891, !alias.scope !10894, !noalias !10806
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10897), !dbg !10900
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10901), !dbg !10900
  store i64 0, ptr %cursor.i, align 8, !dbg !10903, !alias.scope !10907, !noalias !10908
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10909), !dbg !10912
  %845 = getelementptr inbounds nuw i8, ptr %self, i64 168, !dbg !10915
  %846 = getelementptr inbounds nuw i8, ptr %self, i64 128, !dbg !10917
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %845, i8 0, i64 24, i1 false), !alias.scope !10918, !noalias !10919
  %_39.1.i.i.i.i = load i64, ptr %846, align 8, !dbg !10917, !alias.scope !10920, !noalias !10919, !noundef !11
  %_222.i.i.i.i.i = icmp eq i64 %_39.1.i.i.i.i, 0, !dbg !10921
  br i1 %_222.i.i.i.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i, label %bb14.preheader.i.i.i.i.i, !dbg !10926

bb14.preheader.i.i.i.i.i:                         ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECs5xZ4lC6BZIV_20multiband_compressor.exit51.i.i
  %_39.0.i.i.i.i = load ptr, ptr %sides.i, align 8, !dbg !10917, !alias.scope !10920, !noalias !10919, !nonnull !11, !noundef !11
  %.idx.i.i.i.i.i = shl nuw nsw i64 %_39.1.i.i.i.i, 2, !dbg !10927
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_39.0.i.i.i.i, i8 0, i64 %.idx.i.i.i.i.i, i1 false), !dbg !10931, !alias.scope !10932, !noalias !10935
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i, !dbg !10936

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i: ; preds = %bb14.preheader.i.i.i.i.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskfECs5xZ4lC6BZIV_20multiband_compressor.exit51.i.i
  %847 = getelementptr inbounds nuw i8, ptr %self, i64 144, !dbg !10937
  %_40.1.i.i.i.i = load i64, ptr %847, align 8, !dbg !10937, !alias.scope !10920, !noalias !10919, !noundef !11
  %_222.i3.i.i.i.i = icmp eq i64 %_40.1.i.i.i.i, 0, !dbg !10938
  br i1 %_222.i3.i.i.i.i, label %_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i, label %bb14.preheader.i4.i.i.i.i, !dbg !10943

bb14.preheader.i4.i.i.i.i:                        ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i
  %848 = getelementptr inbounds nuw i8, ptr %self, i64 136, !dbg !10937
  %_40.0.i.i.i.i = load ptr, ptr %848, align 8, !dbg !10937, !alias.scope !10920, !noalias !10919, !nonnull !11, !noundef !11
  %.idx.i5.i.i.i.i = shl nuw nsw i64 %_40.1.i.i.i.i, 2, !dbg !10944
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_40.0.i.i.i.i, i8 0, i64 %.idx.i5.i.i.i.i, i1 false), !dbg !10948, !alias.scope !10949, !noalias !10935
  br label %_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i, !dbg !10952

_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i: ; preds = %bb14.preheader.i4.i.i.i.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.i.i.i
  %iter.sroa.0.0.ptr.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 192, !dbg !10953
  %849 = getelementptr inbounds nuw i8, ptr %self, i64 196, !dbg !10956
  %_38.i.i.i.i = load float, ptr %849, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.i.i.i.i, ptr %iter.sroa.0.0.ptr.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %850 = getelementptr inbounds nuw i8, ptr %self, i64 200, !dbg !10959
  store float 0.000000e+00, ptr %850, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %851 = getelementptr inbounds nuw i8, ptr %self, i64 204, !dbg !10960
  store i32 0, ptr %851, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.1.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 208, !dbg !10953
  %852 = getelementptr inbounds nuw i8, ptr %self, i64 212, !dbg !10956
  %_38.1.i.i.i.i = load float, ptr %852, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.1.i.i.i.i, ptr %iter.sroa.0.0.ptr.1.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %853 = getelementptr inbounds nuw i8, ptr %self, i64 216, !dbg !10959
  store float 0.000000e+00, ptr %853, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %854 = getelementptr inbounds nuw i8, ptr %self, i64 220, !dbg !10960
  store i32 0, ptr %854, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.2.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 224, !dbg !10953
  %855 = getelementptr inbounds nuw i8, ptr %self, i64 228, !dbg !10956
  %_38.2.i.i.i.i = load float, ptr %855, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.2.i.i.i.i, ptr %iter.sroa.0.0.ptr.2.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %856 = getelementptr inbounds nuw i8, ptr %self, i64 232, !dbg !10959
  store float 0.000000e+00, ptr %856, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %857 = getelementptr inbounds nuw i8, ptr %self, i64 236, !dbg !10960
  store i32 0, ptr %857, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.3.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 240, !dbg !10953
  %858 = getelementptr inbounds nuw i8, ptr %self, i64 244, !dbg !10956
  %_38.3.i.i.i.i = load float, ptr %858, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.3.i.i.i.i, ptr %iter.sroa.0.0.ptr.3.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %859 = getelementptr inbounds nuw i8, ptr %self, i64 248, !dbg !10959
  store float 0.000000e+00, ptr %859, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %860 = getelementptr inbounds nuw i8, ptr %self, i64 252, !dbg !10960
  store i32 0, ptr %860, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.4.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 256, !dbg !10953
  %861 = getelementptr inbounds nuw i8, ptr %self, i64 260, !dbg !10956
  %_38.4.i.i.i.i = load float, ptr %861, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.4.i.i.i.i, ptr %iter.sroa.0.0.ptr.4.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %862 = getelementptr inbounds nuw i8, ptr %self, i64 264, !dbg !10959
  store float 0.000000e+00, ptr %862, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %863 = getelementptr inbounds nuw i8, ptr %self, i64 268, !dbg !10960
  store i32 0, ptr %863, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.5.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 272, !dbg !10953
  %864 = getelementptr inbounds nuw i8, ptr %self, i64 276, !dbg !10956
  %_38.5.i.i.i.i = load float, ptr %864, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.5.i.i.i.i, ptr %iter.sroa.0.0.ptr.5.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %865 = getelementptr inbounds nuw i8, ptr %self, i64 280, !dbg !10959
  store float 0.000000e+00, ptr %865, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %866 = getelementptr inbounds nuw i8, ptr %self, i64 284, !dbg !10960
  store i32 0, ptr %866, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.6.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 288, !dbg !10953
  %867 = getelementptr inbounds nuw i8, ptr %self, i64 292, !dbg !10956
  %_38.6.i.i.i.i = load float, ptr %867, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.6.i.i.i.i, ptr %iter.sroa.0.0.ptr.6.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %868 = getelementptr inbounds nuw i8, ptr %self, i64 296, !dbg !10959
  store float 0.000000e+00, ptr %868, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %869 = getelementptr inbounds nuw i8, ptr %self, i64 300, !dbg !10960
  store i32 0, ptr %869, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.7.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 304, !dbg !10953
  %870 = getelementptr inbounds nuw i8, ptr %self, i64 308, !dbg !10956
  %_38.7.i.i.i.i = load float, ptr %870, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.7.i.i.i.i, ptr %iter.sroa.0.0.ptr.7.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %871 = getelementptr inbounds nuw i8, ptr %self, i64 312, !dbg !10959
  store float 0.000000e+00, ptr %871, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %872 = getelementptr inbounds nuw i8, ptr %self, i64 316, !dbg !10960
  store i32 0, ptr %872, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.8.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 320, !dbg !10953
  %873 = getelementptr inbounds nuw i8, ptr %self, i64 324, !dbg !10956
  %_38.8.i.i.i.i = load float, ptr %873, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.8.i.i.i.i, ptr %iter.sroa.0.0.ptr.8.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %874 = getelementptr inbounds nuw i8, ptr %self, i64 328, !dbg !10959
  store float 0.000000e+00, ptr %874, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %875 = getelementptr inbounds nuw i8, ptr %self, i64 332, !dbg !10960
  store i32 0, ptr %875, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  %iter.sroa.0.0.ptr.9.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 336, !dbg !10953
  %876 = getelementptr inbounds nuw i8, ptr %self, i64 340, !dbg !10956
  %_38.9.i.i.i.i = load float, ptr %876, align 4, !dbg !10956, !alias.scope !10920, !noalias !10919, !noundef !11
  store float %_38.9.i.i.i.i, ptr %iter.sroa.0.0.ptr.9.i.i.i.i, align 4, !dbg !10958, !alias.scope !10920, !noalias !10919
  %877 = getelementptr inbounds nuw i8, ptr %self, i64 344, !dbg !10959
  store float 0.000000e+00, ptr %877, align 4, !dbg !10959, !alias.scope !10920, !noalias !10919
  %878 = getelementptr inbounds nuw i8, ptr %self, i64 348, !dbg !10960
  store i32 0, ptr %878, align 4, !dbg !10960, !alias.scope !10920, !noalias !10919
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10961), !dbg !10912
  %879 = getelementptr inbounds nuw i8, ptr %self, i64 528, !dbg !10915
  %880 = getelementptr inbounds nuw i8, ptr %self, i64 488, !dbg !10917
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %879, i8 0, i64 24, i1 false), !dbg !10915, !alias.scope !10918, !noalias !10919
  %_39.1.i.1.i.i.i = load i64, ptr %880, align 8, !dbg !10917, !alias.scope !10963, !noalias !10919, !noundef !11
  %_222.i.i.1.i.i.i = icmp eq i64 %_39.1.i.1.i.i.i, 0, !dbg !10921
  br i1 %_222.i.i.1.i.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i, label %bb14.preheader.i.i.1.i.i.i, !dbg !10926

bb14.preheader.i.i.1.i.i.i:                       ; preds = %_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i
  %iter1.sroa.0.0.ptr.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 480, !dbg !10964
  %_39.0.i.1.i.i.i = load ptr, ptr %iter1.sroa.0.0.ptr.1.i.i.i, align 8, !dbg !10917, !alias.scope !10963, !noalias !10919, !nonnull !11, !noundef !11
  %.idx.i.i.1.i.i.i = shl nuw nsw i64 %_39.1.i.1.i.i.i, 2, !dbg !10927
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_39.0.i.1.i.i.i, i8 0, i64 %.idx.i.i.1.i.i.i, i1 false), !dbg !10931, !alias.scope !10932, !noalias !10972
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i, !dbg !10936

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i: ; preds = %bb14.preheader.i.i.1.i.i.i, %_RNvMs0_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_4SidefKj1_E19discontinuity_resetB5_.exit.i.i.i
  %881 = getelementptr inbounds nuw i8, ptr %self, i64 504, !dbg !10937
  %_40.1.i.1.i.i.i = load i64, ptr %881, align 8, !dbg !10937, !alias.scope !10963, !noalias !10919, !noundef !11
  %_222.i3.i.1.i.i.i = icmp eq i64 %_40.1.i.1.i.i.i, 0, !dbg !10938
  br i1 %_222.i3.i.1.i.i.i, label %bb6.i7003, label %bb14.preheader.i4.i.1.i.i.i, !dbg !10943

bb14.preheader.i4.i.1.i.i.i:                      ; preds = %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i
  %882 = getelementptr inbounds nuw i8, ptr %self, i64 496, !dbg !10937
  %_40.0.i.1.i.i.i = load ptr, ptr %882, align 8, !dbg !10937, !alias.scope !10963, !noalias !10919, !nonnull !11, !noundef !11
  %.idx.i5.i.1.i.i.i = shl nuw nsw i64 %_40.1.i.1.i.i.i, 2, !dbg !10944
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_40.0.i.1.i.i.i, i8 0, i64 %.idx.i5.i.1.i.i.i, i1 false), !dbg !10948, !alias.scope !10949, !noalias !10972
  br label %bb6.i7003, !dbg !10952

bb6.i7003:                                        ; preds = %bb14.preheader.i4.i.1.i.i.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCs5xZ4lC6BZIV_20multiband_compressor.exit.i.1.i.i.i
  %iter.sroa.0.0.ptr.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 552, !dbg !10953
  %883 = getelementptr inbounds nuw i8, ptr %self, i64 556, !dbg !10956
  %_38.i.1.i.i.i = load float, ptr %883, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %884 = getelementptr inbounds nuw i8, ptr %self, i64 560, !dbg !10959
  store float 0.000000e+00, ptr %884, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %885 = getelementptr inbounds nuw i8, ptr %self, i64 564, !dbg !10960
  store i32 0, ptr %885, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.1.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 568, !dbg !10953
  %886 = getelementptr inbounds nuw i8, ptr %self, i64 572, !dbg !10956
  %_38.1.i.1.i.i.i = load float, ptr %886, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.1.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.1.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %887 = getelementptr inbounds nuw i8, ptr %self, i64 576, !dbg !10959
  store float 0.000000e+00, ptr %887, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %888 = getelementptr inbounds nuw i8, ptr %self, i64 580, !dbg !10960
  store i32 0, ptr %888, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.2.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 584, !dbg !10953
  %889 = getelementptr inbounds nuw i8, ptr %self, i64 588, !dbg !10956
  %_38.2.i.1.i.i.i = load float, ptr %889, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.2.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.2.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %890 = getelementptr inbounds nuw i8, ptr %self, i64 592, !dbg !10959
  store float 0.000000e+00, ptr %890, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %891 = getelementptr inbounds nuw i8, ptr %self, i64 596, !dbg !10960
  store i32 0, ptr %891, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.3.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 600, !dbg !10953
  %892 = getelementptr inbounds nuw i8, ptr %self, i64 604, !dbg !10956
  %_38.3.i.1.i.i.i = load float, ptr %892, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.3.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.3.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %893 = getelementptr inbounds nuw i8, ptr %self, i64 608, !dbg !10959
  store float 0.000000e+00, ptr %893, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %894 = getelementptr inbounds nuw i8, ptr %self, i64 612, !dbg !10960
  store i32 0, ptr %894, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.4.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 616, !dbg !10953
  %895 = getelementptr inbounds nuw i8, ptr %self, i64 620, !dbg !10956
  %_38.4.i.1.i.i.i = load float, ptr %895, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.4.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.4.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %896 = getelementptr inbounds nuw i8, ptr %self, i64 624, !dbg !10959
  store float 0.000000e+00, ptr %896, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %897 = getelementptr inbounds nuw i8, ptr %self, i64 628, !dbg !10960
  store i32 0, ptr %897, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.5.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 632, !dbg !10953
  %898 = getelementptr inbounds nuw i8, ptr %self, i64 636, !dbg !10956
  %_38.5.i.1.i.i.i = load float, ptr %898, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.5.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.5.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %899 = getelementptr inbounds nuw i8, ptr %self, i64 640, !dbg !10959
  store float 0.000000e+00, ptr %899, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %900 = getelementptr inbounds nuw i8, ptr %self, i64 644, !dbg !10960
  store i32 0, ptr %900, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.6.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 648, !dbg !10953
  %901 = getelementptr inbounds nuw i8, ptr %self, i64 652, !dbg !10956
  %_38.6.i.1.i.i.i = load float, ptr %901, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.6.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.6.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %902 = getelementptr inbounds nuw i8, ptr %self, i64 656, !dbg !10959
  store float 0.000000e+00, ptr %902, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %903 = getelementptr inbounds nuw i8, ptr %self, i64 660, !dbg !10960
  store i32 0, ptr %903, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.7.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 664, !dbg !10953
  %904 = getelementptr inbounds nuw i8, ptr %self, i64 668, !dbg !10956
  %_38.7.i.1.i.i.i = load float, ptr %904, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.7.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.7.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %905 = getelementptr inbounds nuw i8, ptr %self, i64 672, !dbg !10959
  store float 0.000000e+00, ptr %905, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %906 = getelementptr inbounds nuw i8, ptr %self, i64 676, !dbg !10960
  store i32 0, ptr %906, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.8.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 680, !dbg !10953
  %907 = getelementptr inbounds nuw i8, ptr %self, i64 684, !dbg !10956
  %_38.8.i.1.i.i.i = load float, ptr %907, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.8.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.8.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %908 = getelementptr inbounds nuw i8, ptr %self, i64 688, !dbg !10959
  store float 0.000000e+00, ptr %908, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %909 = getelementptr inbounds nuw i8, ptr %self, i64 692, !dbg !10960
  store i32 0, ptr %909, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %iter.sroa.0.0.ptr.9.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 696, !dbg !10953
  %910 = getelementptr inbounds nuw i8, ptr %self, i64 700, !dbg !10956
  %_38.9.i.1.i.i.i = load float, ptr %910, align 4, !dbg !10956, !alias.scope !10963, !noalias !10919, !noundef !11
  store float %_38.9.i.1.i.i.i, ptr %iter.sroa.0.0.ptr.9.i.1.i.i.i, align 4, !dbg !10958, !alias.scope !10963, !noalias !10919
  %911 = getelementptr inbounds nuw i8, ptr %self, i64 704, !dbg !10959
  store float 0.000000e+00, ptr %911, align 4, !dbg !10959, !alias.scope !10963, !noalias !10919
  %912 = getelementptr inbounds nuw i8, ptr %self, i64 708, !dbg !10960
  store i32 0, ptr %912, align 4, !dbg !10960, !alias.scope !10963, !noalias !10919
  %mask.i = load i32, ptr %839, align 8, !dbg !10973, !alias.scope !10721, !noalias !10975, !noundef !11
  %_21.i = and i32 %mask.i, 1, !dbg !10976
  %913 = icmp eq i32 %_21.i, 0, !dbg !10976
  %spec.select = select i1 %913, i64 0, i64 %_19.1, !dbg !10976
  br label %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E6finishB5_.exit, !dbg !10976

_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E6finishB5_.exit: ; preds = %bb6.i7003, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i
  %reports.sroa.6.0 = phi i64 [ 0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockfECs5xZ4lC6BZIV_20multiband_compressor.exit.i.i ], [ %spec.select, %bb6.i7003 ], !dbg !10981
  %reports.sroa.0.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16, !dbg !10982
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, i8 0, i64 16, i1 false), !dbg !10982
  store i64 %report.sroa.7.2, ptr %reports.sroa.0.sroa.5.0._0.sroa_idx, align 8, !dbg !10982
  %reports.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24, !dbg !10982
  store i64 %reports.sroa.6.0, ptr %reports.sroa.4.0._0.sroa_idx, align 8, !dbg !10982
  %reports.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32, !dbg !10982
  store i64 %reports.sroa.6.0, ptr %reports.sroa.6.0._0.sroa_idx, align 8, !dbg !10982
  br label %bb7, !dbg !3808

bb7:                                              ; preds = %bb4, %_RNvMs3_Cs5xZ4lC6BZIV_20multiband_compressorINtB5_8InstancefKj1_E6finishB5_.exit
  ret void, !dbg !3808
}
