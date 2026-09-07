define internal fastcc void @_RINvMsf_CsdvPQf9CMsz3_17true_peak_limiterINtB6_27PreparedTruePeakLimiterBankNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_bank_innerKb1_EB6_(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(328) %_0, ptr noalias noundef nonnull align 32 dereferenceable(2304) %self, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(112) %block) unnamed_addr #0 personality ptr @rust_eh_personality !dbg !5860 {
start:
  %_71.i930.i = alloca [736 x i8], align 32
  %peaks_left.i941.i = alloca [1024 x i8], align 4
  %scratch.i942.i = alloca [32 x i8], align 4
  %hot_left.i948.i = alloca [736 x i8], align 32
  %peaks_left.i635.i = alloca [1024 x i8], align 4
  %scratch.i.i = alloca [32 x i8], align 4
  %hot_left.i641.i = alloca [736 x i8], align 32
  %left_prefix.i228.i = alloca [32 x i8], align 32
  %uniform_left.i249.i = alloca [128 x i8], align 32
  %peaks_left.i250.i = alloca [1024 x i8], align 4
  %hot_left.i256.i = alloca [736 x i8], align 32
  %left_prefix.i.i = alloca [32 x i8], align 32
  %uniform_left.i.i = alloca [128 x i8], align 32
  %peaks_left.i.i = alloca [1024 x i8], align 4
  %hot_left.i.i = alloca [736 x i8], align 32
  %shape.i = alloca [24 x i8], align 8
  %report = alloca [328 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %block, i64 32, !dbg !5861
  %_37.0 = load ptr, ptr %0, align 8, !dbg !5861, !nonnull !12, !align !14, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %block, i64 40, !dbg !5861
  %_37.1 = load i64, ptr %1, align 8, !dbg !5861, !noundef !12
  %2 = icmp eq i64 %_37.1, 0, !dbg !5861
  br i1 %2, label %bb2, label %bb1, !dbg !5861

bb2:                                              ; preds = %bb1, %start
  call void @llvm.lifetime.start.p0(ptr nonnull %report), !dbg !5862
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 2288, !dbg !5863
  %4 = load i8, ptr %3, align 16, !dbg !5863, !range !17, !noundef !12
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(320) %report, i8 0, i64 320, i1 false)
  %5 = getelementptr inbounds nuw i8, ptr %report, i64 320, !dbg !5864
  store i8 %4, ptr %5, align 8, !dbg !5864
  %6 = getelementptr inbounds nuw i8, ptr %block, i64 48
  %_38.0 = load ptr, ptr %6, align 8, !nonnull !12, !align !24, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %block, i64 56
  %_38.1 = load i64, ptr %7, align 8, !noundef !12
  %_26 = getelementptr inbounds nuw i8, ptr %self, i64 1848
  %_25 = getelementptr inbounds nuw i8, ptr %self, i64 1648
  %8 = getelementptr inbounds nuw i8, ptr %block, i64 96
  %_24 = load i64, ptr %8, align 8
  %_23 = getelementptr inbounds nuw i8, ptr %self, i64 2048
  %9 = call i64 @llvm.usub.sat.i64(i64 %_38.1, i64 1), !dbg !5867
  %exitcond.not = icmp eq i64 %_38.1, 0, !dbg !5875
  br i1 %exitcond.not, label %panic, label %bb4, !dbg !5875

bb1:                                              ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 2152, !dbg !5877
  store i8 0, ptr %10, align 8, !dbg !5877
  br label %bb2, !dbg !5878

start.bb11.thread_crit_edge.i:                    ; preds = %bb6.7
  %_19.0.i.pre.pre.i = load ptr, ptr %1182, align 8, !dbg !5879, !alias.scope !5888, !noalias !5893
  br label %bb11.thread.i, !dbg !5899

bb1.i:                                            ; preds = %bb6.7
  %_68.0.i = load ptr, ptr %1182, align 16, !dbg !5900, !alias.scope !5901, !noalias !5902, !nonnull !12, !noundef !12
  %_8.i3733.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_68.0.i, i64 %_68.1.i, !dbg !5903
  br label %bb1.i.i3734.i, !dbg !5908

bb1.i.i3734.i:                                    ; preds = %bb13.i.i3736.i, %bb1.i
  %_221.i.i.i = phi ptr [ %_22.i.i3737.i, %bb13.i.i3736.i ], [ %_68.0.i, %bb1.i ]
  %_12.i.i3735.i = icmp eq ptr %_221.i.i.i, %_8.i3733.i, !dbg !5910
  br i1 %_12.i.i3735.i, label %bb3.i, label %bb13.i.i3736.i, !dbg !5913

bb13.i.i3736.i:                                   ; preds = %bb1.i.i3734.i
  %_22.i.i3737.i = getelementptr inbounds nuw i8, ptr %_221.i.i.i, i64 16, !dbg !5914
  %11 = getelementptr inbounds nuw i8, ptr %_221.i.i.i, i64 12, !dbg !5916
  %_3.i.i.i.i = load i32, ptr %11, align 4, !dbg !5916, !alias.scope !5918, !noalias !5923, !noundef !12
  %12 = icmp eq i32 %_3.i.i.i.i, 0, !dbg !5916
  %_51.i.i.i.i = load i32, ptr %_221.i.i.i, align 4, !dbg !5916, !alias.scope !5918, !noalias !5923
  %13 = getelementptr inbounds nuw i8, ptr %_221.i.i.i, i64 4, !dbg !5916
  %_72.i.i.i.i = load i32, ptr %13, align 4, !dbg !5916, !alias.scope !5918, !noalias !5923
  %14 = icmp eq i32 %_51.i.i.i.i, %_72.i.i.i.i, !dbg !5916
  %_0.sroa.0.0.i.i.i.i = select i1 %12, i1 %14, i1 false, !dbg !5916
  br i1 %_0.sroa.0.0.i.i.i.i, label %bb1.i.i3734.i, label %bb11.thread.i, !dbg !5926

bb3.i:                                            ; preds = %bb1.i.i3734.i
  %15 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !5927
  %_69.0.i = load ptr, ptr %15, align 16, !dbg !5927, !alias.scope !5901, !noalias !5902, !nonnull !12, !noundef !12
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !5927
  %_69.1.i = load i64, ptr %16, align 8, !dbg !5927, !alias.scope !5901, !noalias !5902, !noundef !12
  %_8.i3738.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_69.0.i, i64 %_69.1.i, !dbg !5928
  br label %bb1.i.i3739.i, !dbg !5933

bb1.i.i3739.i:                                    ; preds = %bb13.i.i3742.i, %bb3.i
  %_221.i.i3740.i = phi ptr [ %_22.i.i3743.i, %bb13.i.i3742.i ], [ %_69.0.i, %bb3.i ]
  %_12.i.i3741.i = icmp eq ptr %_221.i.i3740.i, %_8.i3738.i, !dbg !5935
  br i1 %_12.i.i3741.i, label %bb5.i, label %bb13.i.i3742.i, !dbg !5938

bb13.i.i3742.i:                                   ; preds = %bb1.i.i3739.i
  %_22.i.i3743.i = getelementptr inbounds nuw i8, ptr %_221.i.i3740.i, i64 16, !dbg !5939
  %17 = getelementptr inbounds nuw i8, ptr %_221.i.i3740.i, i64 12, !dbg !5941
  %_3.i.i.i3744.i = load i32, ptr %17, align 4, !dbg !5941, !alias.scope !5943, !noalias !5948, !noundef !12
  %18 = icmp eq i32 %_3.i.i.i3744.i, 0, !dbg !5941
  %_51.i.i.i3745.i = load i32, ptr %_221.i.i3740.i, align 4, !dbg !5941, !alias.scope !5943, !noalias !5948
  %19 = getelementptr inbounds nuw i8, ptr %_221.i.i3740.i, i64 4, !dbg !5941
  %_72.i.i.i3746.i = load i32, ptr %19, align 4, !dbg !5941, !alias.scope !5943, !noalias !5948
  %20 = icmp eq i32 %_51.i.i.i3745.i, %_72.i.i.i3746.i, !dbg !5941
  %_0.sroa.0.0.i.i.i3747.i = select i1 %18, i1 %20, i1 false, !dbg !5941
  br i1 %_0.sroa.0.0.i.i.i3747.i, label %bb1.i.i3739.i, label %bb11.thread.i, !dbg !5951

bb5.i:                                            ; preds = %bb1.i.i3739.i
  %_51.not.i = icmp samesign ugt i64 %words.i, %_39.1
  br i1 %_51.not.i, label %bb35.i, label %bb1.i3749.i, !dbg !5952, !prof !165

bb11.thread.i:                                    ; preds = %bb13.i.i3736.i, %bb13.i.i3742.i, %start.bb11.thread_crit_edge.i
  %_19.0.i.pre.i = phi ptr [ %_19.0.i.pre.pre.i, %start.bb11.thread_crit_edge.i ], [ %_68.0.i, %bb13.i.i3742.i ], [ %_68.0.i, %bb13.i.i3736.i ], !dbg !5879
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb16.i, !dbg !5961

bb11.i:                                           ; preds = %bb1.i3749.i
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  %23 = load i8, ptr %22, align 8, !range !17, !alias.scope !5901, !noalias !5902
  %_15.i = trunc nuw i8 %23 to i1
  br i1 %_15.i, label %bb13.i, label %bb16.i, !dbg !5961

bb1.i3749.i:                                      ; preds = %bb5.i, %bb10.i3756.i
  %iter.sroa.6.0.i.i = phi i64 [ %len.i.i.i.i.i, %bb10.i3756.i ], [ %words.i, %bb5.i ], !dbg !5962
  %iter.sroa.0.0.i3750.i = phi ptr [ %data.i.i.i.i.i, %bb10.i3756.i ], [ %_39.0, %bb5.i ], !dbg !5962
  %24 = icmp eq i64 %iter.sroa.6.0.i.i, 0, !dbg !5964
  br i1 %24, label %bb11.i, label %bb11.preheader.i.i, !dbg !5964

bb11.preheader.i.i:                               ; preds = %bb1.i3749.i
  %..i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i.i, i64 32), !dbg !5966
  %_18.idx.i.i = shl nuw nsw i64 %..i.i.i.i, 2, !dbg !5969
  %_18.i3751.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i3750.i, i64 %_18.idx.i.i, !dbg !5969
  br label %bb11.i3752.i, !dbg !5974

bb11.i3752.i:                                     ; preds = %bb11.i3752.i, %bb11.preheader.i.i
  %iter1.sroa.0.014.i.i = phi ptr [ %_31.i3753.i, %bb11.i3752.i ], [ %iter.sroa.0.0.i3750.i, %bb11.preheader.i.i ]
  %bits.sroa.0.013.i.i = phi i32 [ %25, %bb11.i3752.i ], [ 0, %bb11.preheader.i.i ]
  %_31.i3753.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i.i, i64 4, !dbg !5976
  %_134.i3754.i = load i32, ptr %iter1.sroa.0.014.i.i, align 4, !dbg !5978, !alias.scope !5979, !noalias !5901, !noundef !12
  %25 = or i32 %_134.i3754.i, %bits.sroa.0.013.i.i, !dbg !5982
  %_25.i3755.i = icmp eq ptr %_31.i3753.i, %_18.i3751.i, !dbg !5983
  br i1 %_25.i3755.i, label %bb10.i3756.i, label %bb11.i3752.i, !dbg !5974

bb10.i3756.i:                                     ; preds = %bb11.i3752.i
  %data.i.i.i.i.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i3750.i, i64 %..i.i.i.i, !dbg !5985
  %len.i.i.i.i.i = sub nuw nsw i64 %iter.sroa.6.0.i.i, %..i.i.i.i, !dbg !5990
  %26 = icmp eq i32 %25, 0, !dbg !5991
  br i1 %26, label %bb1.i3749.i, label %bb11.thread6733.i, !dbg !5991

bb11.thread6733.i:                                ; preds = %bb10.i3756.i
  %27 = getelementptr inbounds nuw i8, ptr %self, i64 2152
  br label %bb16.i, !dbg !5961

bb35.i:                                           ; preds = %bb5.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b63e7acf20e2df0969a284a5c2277b6c) #30, !dbg !5992, !noalias !5902
  unreachable, !dbg !5992

bb16.i:                                           ; preds = %bb11.thread6733.i, %bb11.i, %bb11.thread.i
  %_19.0.i.i = phi ptr [ %_19.0.i.pre.i, %bb11.thread.i ], [ %_68.0.i, %bb11.i ], [ %_68.0.i, %bb11.thread6733.i ], !dbg !5879
  %28 = phi ptr [ %21, %bb11.thread.i ], [ %22, %bb11.i ], [ %27, %bb11.thread6733.i ]
  %quiet.sroa.0.06732.i = phi i1 [ false, %bb11.thread.i ], [ true, %bb11.i ], [ false, %bb11.thread6733.i ]
  %_23.i = getelementptr inbounds nuw i8, ptr %self, i64 1616, !dbg !5993
  %_25.i = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !5994
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5995), !dbg !5996
  %_8.i3757.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_19.0.i.i, i64 %_68.1.i, !dbg !5997
  br label %bb1.i.i3758.i, !dbg !6002

bb1.i.i3758.i:                                    ; preds = %bb13.i.i3761.i, %bb16.i
  %_221.i.i3759.i = phi ptr [ %_22.i.i3762.i, %bb13.i.i3761.i ], [ %_19.0.i.i, %bb16.i ]
  %_12.i.i3760.i = icmp eq ptr %_221.i.i3759.i, %_8.i3757.i, !dbg !6004
  br i1 %_12.i.i3760.i, label %bb12.i.i, label %bb13.i.i3761.i, !dbg !6007

bb13.i.i3761.i:                                   ; preds = %bb1.i.i3758.i
  %_22.i.i3762.i = getelementptr inbounds nuw i8, ptr %_221.i.i3759.i, i64 16, !dbg !6008
  %29 = getelementptr inbounds nuw i8, ptr %_221.i.i3759.i, i64 12, !dbg !6010
  %_3.i.i.i3763.i = load i32, ptr %29, align 4, !dbg !6010, !alias.scope !6012, !noalias !6017, !noundef !12
  %30 = icmp eq i32 %_3.i.i.i3763.i, 0, !dbg !6010
  %_51.i.i.i3764.i = load i32, ptr %_221.i.i3759.i, align 4, !dbg !6010, !alias.scope !6012, !noalias !6017
  %31 = getelementptr inbounds nuw i8, ptr %_221.i.i3759.i, i64 4, !dbg !6010
  %_72.i.i.i3765.i = load i32, ptr %31, align 4, !dbg !6010, !alias.scope !6012, !noalias !6017
  %32 = icmp eq i32 %_51.i.i.i3764.i, %_72.i.i.i3765.i, !dbg !6010
  %_0.sroa.0.0.i.i.i3766.i = select i1 %30, i1 %32, i1 false, !dbg !6010
  br i1 %_0.sroa.0.0.i.i.i3766.i, label %bb1.i.i3758.i, label %bb13.i.i, !dbg !6020

bb13.i.i:                                         ; preds = %bb13.i.i3761.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6021), !dbg !6024
  %33 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !6026
  %_31.0.i.i = load ptr, ptr %33, align 8, !dbg !6026, !alias.scope !6028, !noalias !5893, !nonnull !12, !noundef !12
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !6026
  %_31.1.i.i = load i64, ptr %34, align 8, !dbg !6026, !alias.scope !6028, !noalias !5893, !noundef !12
  %_17.idx.i.i = mul nuw nsw i64 %_31.1.i.i, 12, !dbg !6029
  %_17.i3768.i = getelementptr inbounds nuw i8, ptr %_31.0.i.i, i64 %_17.idx.i.i, !dbg !6029
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6033), !dbg !6036, !noalias !6037
  %_5.not.i.i.i.i = icmp eq i64 %_31.1.i.i, 0
  %35 = getelementptr inbounds nuw i8, ptr %_31.0.i.i, i64 4
  %36 = getelementptr inbounds nuw i8, ptr %_31.0.i.i, i64 8
  br i1 %_5.not.i.i.i.i, label %bb2.i3779.i, label %bb1.i.i3769.i

bb1.i.i3769.i:                                    ; preds = %bb13.i.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i
  %_224.i.i.i = phi ptr [ %_22.i.i3772.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i ], [ %_31.0.i.i, %bb13.i.i ]
  %_12.i.i3770.i = icmp eq ptr %_224.i.i.i, %_17.i3768.i, !dbg !6038
  br i1 %_12.i.i3770.i, label %bb2.i3779.i, label %bb13.i.i3771.i, !dbg !6042

bb13.i.i3771.i:                                   ; preds = %bb1.i.i3769.i
  %_22.i.i3772.i = getelementptr inbounds nuw i8, ptr %_224.i.i.i, i64 12, !dbg !6043
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6045), !dbg !6048, !noalias !6037
  %_9.i.i.i3773.i = load i32, ptr %_224.i.i.i, align 4, !dbg !6049, !alias.scope !6045, !noalias !6052, !noundef !12
  %_10.i.i.i.i = load i32, ptr %_31.0.i.i, align 4, !dbg !6049, !alias.scope !6033, !noalias !6054, !noundef !12
  %_8.i.i.i.i = icmp eq i32 %_9.i.i.i3773.i, %_10.i.i.i.i, !dbg !6049
  br i1 %_8.i.i.i.i, label %bb2.i.i.i.i, label %bb8.i.i, !dbg !6049

bb2.i.i.i.i:                                      ; preds = %bb13.i.i3771.i
  %37 = getelementptr inbounds nuw i8, ptr %_224.i.i.i, i64 4, !dbg !6049
  %_12.i.i.i3775.i = load i32, ptr %37, align 4, !dbg !6049, !alias.scope !6045, !noalias !6052, !noundef !12
  %_13.i.i.i3776.i = load i32, ptr %35, align 4, !dbg !6049, !alias.scope !6033, !noalias !6054, !noundef !12
  %_11.i.i.i.i = icmp eq i32 %_12.i.i.i3775.i, %_13.i.i.i3776.i, !dbg !6049
  br i1 %_11.i.i.i.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i, label %bb8.i.i, !dbg !6049

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i: ; preds = %bb2.i.i.i.i
  %38 = getelementptr inbounds nuw i8, ptr %_224.i.i.i, i64 8, !dbg !6049
  %_14.i.i.i3777.i = load i32, ptr %38, align 4, !dbg !6049, !alias.scope !6045, !noalias !6052, !noundef !12
  %_15.i.i.i3778.i = load i32, ptr %36, align 4, !dbg !6049, !alias.scope !6033, !noalias !6054, !noundef !12
  %39 = icmp eq i32 %_14.i.i.i3777.i, %_15.i.i.i3778.i, !dbg !6049
  br i1 %39, label %bb1.i.i3769.i, label %bb8.i.i, !dbg !6048

bb2.i3779.i:                                      ; preds = %bb1.i.i3769.i, %bb13.i.i
  %40 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !6055
  %_32.0.i.i = load ptr, ptr %40, align 8, !dbg !6055, !alias.scope !6028, !noalias !5893, !nonnull !12, !noundef !12
  %41 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !6055
  %_32.1.i.i = load i64, ptr %41, align 8, !dbg !6055, !alias.scope !6028, !noalias !5893, !noundef !12
  %_26.idx.i.i = shl nuw nsw i64 %_32.1.i.i, 2, !dbg !6056
  %_26.i3780.i = getelementptr inbounds nuw i8, ptr %_32.0.i.i, i64 %_26.idx.i.i, !dbg !6056
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6060), !dbg !6063, !noalias !6037
  %_6.not.i.i.i.i = icmp eq i64 %_32.1.i.i, 0
  br i1 %_6.not.i.i.i.i, label %bb4.i.i, label %bb1.i3.i.i

bb1.i3.i.i:                                       ; preds = %bb2.i3779.i, %bb13.i5.i.i
  %_223.i.i.i = phi ptr [ %_22.i6.i.i, %bb13.i5.i.i ], [ %_32.0.i.i, %bb2.i3779.i ]
  %_12.i4.i.i = icmp eq ptr %_223.i.i.i, %_26.i3780.i, !dbg !6064
  br i1 %_12.i4.i.i, label %bb4.i.i, label %bb13.i5.i.i, !dbg !6068

bb13.i5.i.i:                                      ; preds = %bb1.i3.i.i
  %_22.i6.i.i = getelementptr inbounds nuw i8, ptr %_223.i.i.i, i64 4, !dbg !6069
  %ptr.val.i.i.i = load i32, ptr %_223.i.i.i, align 4, !dbg !6071, !noalias !6072
  %_4.i.i.i3781.i = load i32, ptr %_32.0.i.i, align 4, !dbg !6074, !alias.scope !6060, !noalias !6076, !noundef !12
  %_0.i.i.i.i = icmp eq i32 %ptr.val.i.i.i, %_4.i.i.i3781.i, !dbg !6077
  br i1 %_0.i.i.i.i, label %bb1.i3.i.i, label %bb8.i.i, !dbg !6071

bb12.i.i:                                         ; preds = %bb1.i.i3758.i
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 1792, !dbg !6078
  %_20.0.i.i = load ptr, ptr %42, align 8, !dbg !6078, !alias.scope !5888, !noalias !5893, !nonnull !12, !noundef !12
  %43 = getelementptr inbounds nuw i8, ptr %self, i64 1800, !dbg !6078
  %_20.1.i.i = load i64, ptr %43, align 8, !dbg !6078, !alias.scope !5888, !noalias !5893, !noundef !12
  %_8.i3782.i = getelementptr inbounds nuw %"effect_runtime::ramp::LinearRamp", ptr %_20.0.i.i, i64 %_20.1.i.i, !dbg !6079
  br label %bb1.i.i3783.i, !dbg !6084

bb1.i.i3783.i:                                    ; preds = %bb13.i.i3786.i, %bb12.i.i
  %_221.i.i3784.i = phi ptr [ %_22.i.i3787.i, %bb13.i.i3786.i ], [ %_20.0.i.i, %bb12.i.i ]
  %_12.i.i3785.i = icmp eq ptr %_221.i.i3784.i, %_8.i3782.i, !dbg !6086
  br i1 %_12.i.i3785.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3792.i, label %bb13.i.i3786.i, !dbg !6089

bb13.i.i3786.i:                                   ; preds = %bb1.i.i3783.i
  %_22.i.i3787.i = getelementptr inbounds nuw i8, ptr %_221.i.i3784.i, i64 16, !dbg !6090
  %44 = getelementptr inbounds nuw i8, ptr %_221.i.i3784.i, i64 12, !dbg !6092
  %_3.i.i.i3788.i = load i32, ptr %44, align 4, !dbg !6092, !alias.scope !6094, !noalias !6099, !noundef !12
  %45 = icmp eq i32 %_3.i.i.i3788.i, 0, !dbg !6092
  %_51.i.i.i3789.i = load i32, ptr %_221.i.i3784.i, align 4, !dbg !6092, !alias.scope !6094, !noalias !6099
  %46 = getelementptr inbounds nuw i8, ptr %_221.i.i3784.i, i64 4, !dbg !6092
  %_72.i.i.i3790.i = load i32, ptr %46, align 4, !dbg !6092, !alias.scope !6094, !noalias !6099
  %47 = icmp eq i32 %_51.i.i.i3789.i, %_72.i.i.i3790.i, !dbg !6092
  %_0.sroa.0.0.i.i.i3791.i = select i1 %45, i1 %47, i1 false, !dbg !6092
  br i1 %_0.sroa.0.0.i.i.i3791.i, label %bb1.i.i3783.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3792.i, !dbg !6102

_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3792.i: ; preds = %bb13.i.i3786.i, %bb1.i.i3783.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6103), !dbg !6024
  %48 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !6106
  %_31.0.i3793.i = load ptr, ptr %48, align 8, !dbg !6106, !alias.scope !6108, !noalias !5893, !nonnull !12, !noundef !12
  %49 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !6106
  %_31.1.i3794.i = load i64, ptr %49, align 8, !dbg !6106, !alias.scope !6108, !noalias !5893, !noundef !12
  %_17.idx.i3795.i = mul nuw nsw i64 %_31.1.i3794.i, 12, !dbg !6109
  %_17.i3796.i = getelementptr inbounds nuw i8, ptr %_31.0.i3793.i, i64 %_17.idx.i3795.i, !dbg !6109
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6113), !dbg !6116, !noalias !6037
  %_5.not.i.i.i3797.i = icmp eq i64 %_31.1.i3794.i, 0
  %50 = getelementptr inbounds nuw i8, ptr %_31.0.i3793.i, i64 4
  %51 = getelementptr inbounds nuw i8, ptr %_31.0.i3793.i, i64 8
  br i1 %_5.not.i.i.i3797.i, label %bb2.i3815.i, label %bb1.i.i3798.i

bb1.i.i3798.i:                                    ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3792.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3812.i
  %_224.i.i3799.i = phi ptr [ %_22.i.i3802.i, %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3812.i ], [ %_31.0.i3793.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3792.i ]
  %_12.i.i3800.i = icmp eq ptr %_224.i.i3799.i, %_17.i3796.i, !dbg !6117
  br i1 %_12.i.i3800.i, label %bb2.i3815.i, label %bb13.i.i3801.i, !dbg !6121

bb13.i.i3801.i:                                   ; preds = %bb1.i.i3798.i
  %_22.i.i3802.i = getelementptr inbounds nuw i8, ptr %_224.i.i3799.i, i64 12, !dbg !6122
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6124), !dbg !6127, !noalias !6037
  %_9.i.i.i3803.i = load i32, ptr %_224.i.i3799.i, align 4, !dbg !6128, !alias.scope !6124, !noalias !6131, !noundef !12
  %_10.i.i.i3804.i = load i32, ptr %_31.0.i3793.i, align 4, !dbg !6128, !alias.scope !6113, !noalias !6133, !noundef !12
  %_8.i.i.i3805.i = icmp eq i32 %_9.i.i.i3803.i, %_10.i.i.i3804.i, !dbg !6128
  br i1 %_8.i.i.i3805.i, label %bb2.i.i.i3808.i, label %bb6.i.i, !dbg !6128

bb2.i.i.i3808.i:                                  ; preds = %bb13.i.i3801.i
  %52 = getelementptr inbounds nuw i8, ptr %_224.i.i3799.i, i64 4, !dbg !6128
  %_12.i.i.i3809.i = load i32, ptr %52, align 4, !dbg !6128, !alias.scope !6124, !noalias !6131, !noundef !12
  %_13.i.i.i3810.i = load i32, ptr %50, align 4, !dbg !6128, !alias.scope !6113, !noalias !6133, !noundef !12
  %_11.i.i.i3811.i = icmp eq i32 %_12.i.i.i3809.i, %_13.i.i.i3810.i, !dbg !6128
  br i1 %_11.i.i.i3811.i, label %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3812.i, label %bb6.i.i, !dbg !6128

_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3812.i: ; preds = %bb2.i.i.i3808.i
  %53 = getelementptr inbounds nuw i8, ptr %_224.i.i3799.i, i64 8, !dbg !6128
  %_14.i.i.i3813.i = load i32, ptr %53, align 4, !dbg !6128, !alias.scope !6124, !noalias !6131, !noundef !12
  %_15.i.i.i3814.i = load i32, ptr %51, align 4, !dbg !6128, !alias.scope !6113, !noalias !6133, !noundef !12
  %54 = icmp eq i32 %_14.i.i.i3813.i, %_15.i.i.i3814.i, !dbg !6128
  br i1 %54, label %bb1.i.i3798.i, label %bb6.i.i, !dbg !6127

bb2.i3815.i:                                      ; preds = %bb1.i.i3798.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter20ramps_are_stationary.exit3792.i
  %55 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !6134
  %_32.0.i3816.i = load ptr, ptr %55, align 8, !dbg !6134, !alias.scope !6108, !noalias !5893, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !6134
  %_32.1.i3817.i = load i64, ptr %56, align 8, !dbg !6134, !alias.scope !6108, !noalias !5893, !noundef !12
  %_26.idx.i3818.i = shl nuw nsw i64 %_32.1.i3817.i, 2, !dbg !6135
  %_26.i3819.i = getelementptr inbounds nuw i8, ptr %_32.0.i3816.i, i64 %_26.idx.i3818.i, !dbg !6135
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6139), !dbg !6142, !noalias !6037
  %_6.not.i.i.i3820.i = icmp eq i64 %_32.1.i3817.i, 0
  br i1 %_6.not.i.i.i3820.i, label %bb2.i.i, label %bb1.i3.i3821.i

bb1.i3.i3821.i:                                   ; preds = %bb2.i3815.i, %bb13.i5.i3824.i
  %_223.i.i3822.i = phi ptr [ %_22.i6.i3825.i, %bb13.i5.i3824.i ], [ %_32.0.i3816.i, %bb2.i3815.i ]
  %_12.i4.i3823.i = icmp eq ptr %_223.i.i3822.i, %_26.i3819.i, !dbg !6143
  br i1 %_12.i4.i3823.i, label %bb2.i.i, label %bb13.i5.i3824.i, !dbg !6147

bb13.i5.i3824.i:                                  ; preds = %bb1.i3.i3821.i
  %_22.i6.i3825.i = getelementptr inbounds nuw i8, ptr %_223.i.i3822.i, i64 4, !dbg !6148
  %ptr.val.i.i3826.i = load i32, ptr %_223.i.i3822.i, align 4, !dbg !6150, !noalias !6151
  %_4.i.i.i3827.i = load i32, ptr %_32.0.i3816.i, align 4, !dbg !6153, !alias.scope !6139, !noalias !6155, !noundef !12
  %_0.i.i.i3828.i = icmp eq i32 %ptr.val.i.i3826.i, %_4.i.i.i3827.i, !dbg !6156
  br i1 %_0.i.i.i3828.i, label %bb1.i3.i3821.i, label %bb6.i.i, !dbg !6150

bb8.i.i:                                          ; preds = %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i.i, %bb2.i.i.i.i, %bb13.i.i3771.i, %bb13.i5.i.i, %bb6.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6157), !dbg !6160
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6161), !dbg !6160
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6163), !dbg !6160
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i948.i), !dbg !6165, !noalias !6169
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i948.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !6172, !noalias !6173
  %57 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !6174
  %58 = load i8, ptr %57, align 32, !dbg !6174, !range !17, !alias.scope !6178, !noalias !6179, !noundef !12
  %59 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !6180
  %60 = load i8, ptr %59, align 1, !dbg !6180, !range !17, !alias.scope !6178, !noalias !6179, !noundef !12
  %_24.i955.i = load i32, ptr %_25.i, align 4, !dbg !6182, !alias.scope !6184, !noalias !6185, !noundef !12
  %61 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !6186
  %_26.i956.i = load i32, ptr %61, align 4, !dbg !6186, !alias.scope !6184, !noalias !6185, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i942.i), !dbg !6188, !noalias !6169
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i942.i, i8 0, i64 32, i1 false), !noalias !6169
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i941.i), !dbg !6190, !noalias !6169
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i941.i, i8 0, i64 1024, i1 false), !noalias !6169
  %62 = add nuw nsw i64 %_31, 31, !dbg !6192
  %yield_count.sroa.0.0.i.i.i = lshr i64 %62, 5, !dbg !6192
  %_75.not.i9677953.i = icmp eq i64 %yield_count.sroa.0.0.i.i.i, 0, !dbg !6199
  br i1 %_75.not.i9677953.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb29.i968.lr.ph.i, !dbg !6199

bb29.i968.lr.ph.i:                                ; preds = %bb8.i.i
  %63 = zext i32 %_26.i956.i to i64, !dbg !6186
  %64 = zext i32 %_24.i955.i to i64, !dbg !6182
  %_22.i952.i = trunc nuw i8 %60 to i1, !dbg !6180
  %_21.i949.i = trunc nuw i8 %58 to i1, !dbg !6174
  %65 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !6208
  %66 = bitcast <8 x float> %65 to <8 x i32>, !dbg !6229
  %67 = xor <8 x i32> %66, splat (i32 -1), !dbg !6244
  %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 32
  %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 64
  %history.i.i894.sroa.16.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 96
  %history.i.i894.sroa.19.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 128
  %history.i.i894.sroa.22.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 160
  %history.i.i894.sroa.25.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 192
  %history.i.i894.sroa.29.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 224
  %history.i.i894.sroa.32.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 256
  %history.i.i894.sroa.35.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 288
  %history.i.i894.sroa.38.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 320
  %history.i.i894.sroa.41.0.hot_left.i948.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 352
  %68 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %69 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %70 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i1071.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %71 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %72 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %73 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i1072.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %74 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %75 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %76 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i1073.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %77 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %78 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %79 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i1074.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %82 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i1075.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %83 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %84 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %85 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i1076.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %86 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %87 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %88 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i1077.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %89 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %90 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %91 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i1078.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %92 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i1079.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %97 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i1080.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %98 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %99 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %100 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i1081.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %101 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %103 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %_47.i984.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 384
  %_48.i985.i = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 512
  %104 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 480
  %105 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 448
  %106 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 416
  %107 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 608
  %108 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 576
  %109 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 544
  %110 = select i1 %_21.i949.i, <8 x i32> %66, <8 x i32> %67
  %111 = icmp slt <8 x i32> %110, zeroinitializer
  %112 = getelementptr inbounds nuw i8, ptr %self, i64 1624
  %113 = getelementptr inbounds nuw i8, ptr %self, i64 1840
  %114 = getelementptr inbounds nuw i8, ptr %self, i64 1688
  %115 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %116 = getelementptr inbounds nuw i8, ptr %self, i64 1832
  %117 = getelementptr inbounds nuw i8, ptr %self, i64 1824
  %118 = getelementptr inbounds nuw i8, ptr %self, i64 1768
  %119 = getelementptr inbounds nuw i8, ptr %self, i64 1760
  %120 = getelementptr inbounds nuw i8, ptr %self, i64 1736
  %121 = getelementptr inbounds nuw i8, ptr %self, i64 1728
  %122 = getelementptr inbounds nuw i8, ptr %self, i64 1704
  %123 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %124 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 672
  %125 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 704
  %126 = getelementptr inbounds nuw i8, ptr %hot_left.i948.i, i64 640
  %127 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %128 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %129 = select i1 %_22.i952.i, <8 x i32> %66, <8 x i32> %67
  %130 = icmp slt <8 x i32> %129, zeroinitializer
  %131 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %_13.i1422.sroa.0.0.copyload.i = load <8 x float>, ptr %106, align 32, !noalias !6249
  %_13.i1414.sroa.0.0.copyload.i = load <8 x float>, ptr %109, align 32, !noalias !6249
  %_64.i.i910.sroa.0.0.copyload.i = load <8 x float>, ptr %125, align 32, !noalias !6249
  %iter.i.i916.sroa.0.0.ptr7705.1.i = getelementptr inbounds nuw i8, ptr %scratch.i942.i, i64 4
  %iter.i.i916.sroa.0.0.ptr7705.2.i = getelementptr inbounds nuw i8, ptr %scratch.i942.i, i64 8
  %iter.i.i916.sroa.0.0.ptr7705.3.i = getelementptr inbounds nuw i8, ptr %scratch.i942.i, i64 12
  %iter.i.i916.sroa.0.0.ptr7705.4.i = getelementptr inbounds nuw i8, ptr %scratch.i942.i, i64 16
  %iter.i.i916.sroa.0.0.ptr7705.5.i = getelementptr inbounds nuw i8, ptr %scratch.i942.i, i64 20
  %iter.i.i916.sroa.0.0.ptr7705.6.i = getelementptr inbounds nuw i8, ptr %scratch.i942.i, i64 24
  %iter.i.i916.sroa.0.0.ptr7705.7.i = getelementptr inbounds nuw i8, ptr %scratch.i942.i, i64 28
  %hot_left.i948.promoted.i = load <8 x float>, ptr %hot_left.i948.i, align 32, !noalias !6250
  %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.16.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.16.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.19.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.19.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.22.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.22.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.25.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.25.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.29.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.29.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.32.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.32.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.35.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.35.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.38.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.38.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %history.i.i894.sroa.41.0.hot_left.i948.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i894.sroa.41.0.hot_left.i948.sroa_idx.i, align 32, !noalias !6250
  %.promoted10732.i = load <8 x float>, ptr %104, align 32, !noalias !6249
  %_47.i984.promoted10735.i = load <8 x float>, ptr %_47.i984.i, align 32, !noalias !6249
  %.promoted.i = load <8 x float>, ptr %105, align 32, !noalias !6249
  %.promoted10740.i = load <8 x float>, ptr %107, align 32, !noalias !6249
  %_48.i985.promoted10743.i = load <8 x float>, ptr %_48.i985.i, align 32, !noalias !6249
  %.promoted10746.i = load <8 x float>, ptr %108, align 32, !noalias !6249
  %.promoted10749.i = load <8 x float>, ptr %124, align 32, !noalias !6249
  %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1
  %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1
  br label %bb29.i968.i, !dbg !6199

bb14.i976.bb12.i962.loopexit_crit_edge.i:         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i
  store <8 x float> %294, ptr %124, align 1, !dbg !6255, !noalias !6249
  store <8 x float> %246, ptr %104, align 32, !dbg !6273, !noalias !6279
  store <8 x float> %251, ptr %_47.i984.i, align 32, !dbg !6285, !noalias !6279
  store <8 x float> %252, ptr %105, align 32, !dbg !6287, !noalias !6279
  store <8 x float> %254, ptr %107, align 32, !dbg !6288, !noalias !6290
  store <8 x float> %259, ptr %_48.i985.i, align 32, !dbg !6293, !noalias !6290
  store <8 x float> %260, ptr %108, align 32, !dbg !6294, !noalias !6290
  br label %bb12.i962.loopexit.i, !dbg !6295

bb12.i962.loopexit.i:                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i, %bb14.i976.bb12.i962.loopexit_crit_edge.i
  %.lcssa1070110750.i = phi <8 x float> [ %294, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %.lcssa1070110751.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ]
  %.lcssa1050010747.i = phi <8 x float> [ %260, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %.lcssa1050010748.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ]
  %.lcssa1051810744.i = phi <8 x float> [ %259, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %.lcssa1051810745.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ]
  %.lcssa1053610741.i = phi <8 x float> [ %254, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %.lcssa1053610742.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ]
  %.lcssa1055410738.i = phi <8 x float> [ %252, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %.lcssa1055410739.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ]
  %.lcssa1057210736.i = phi <8 x float> [ %251, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %.lcssa1057210737.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ]
  %.lcssa1059010733.i = phi <8 x float> [ %246, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %.lcssa1059010734.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ]
  %ring_cursor.sroa.0.1.i978.lcssa.i = phi i64 [ %spec.store.select9.i1058.i, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %ring_cursor.sroa.0.0.i9657956.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ], !dbg !6301
  %main_cursor.sroa.0.1.i979.lcssa.i = phi i64 [ %spec.store.select.i1056.i, %bb14.i976.bb12.i962.loopexit_crit_edge.i ], [ %main_cursor.sroa.0.0.i9667957.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i ], !dbg !6302
  %_75.not.i967.i = icmp eq i64 %134, 0, !dbg !6199
  %indvars.iv.next.i = add nsw i64 %indvars.iv.i, -32, !dbg !6199
  br i1 %_75.not.i967.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, label %bb29.i968.i, !dbg !6199

bb29.i968.i:                                      ; preds = %bb12.i962.loopexit.i, %bb29.i968.lr.ph.i
  %history.i.i894.sroa.13.sroa.0.0.lcssa.i2771 = phi <8 x float> [ %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i.promoted, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.13.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.10.sroa.0.0.lcssa.i2752 = phi <8 x float> [ %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i.promoted, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.10.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %.lcssa1070110751.i = phi <8 x float> [ %.promoted10749.i, %bb29.i968.lr.ph.i ], [ %.lcssa1070110750.i, %bb12.i962.loopexit.i ]
  %.lcssa1050010748.i = phi <8 x float> [ %.promoted10746.i, %bb29.i968.lr.ph.i ], [ %.lcssa1050010747.i, %bb12.i962.loopexit.i ]
  %.lcssa1051810745.i = phi <8 x float> [ %_48.i985.promoted10743.i, %bb29.i968.lr.ph.i ], [ %.lcssa1051810744.i, %bb12.i962.loopexit.i ]
  %.lcssa1053610742.i = phi <8 x float> [ %.promoted10740.i, %bb29.i968.lr.ph.i ], [ %.lcssa1053610741.i, %bb12.i962.loopexit.i ]
  %.lcssa1055410739.i = phi <8 x float> [ %.promoted.i, %bb29.i968.lr.ph.i ], [ %.lcssa1055410738.i, %bb12.i962.loopexit.i ]
  %.lcssa1057210737.i = phi <8 x float> [ %_47.i984.promoted10735.i, %bb29.i968.lr.ph.i ], [ %.lcssa1057210736.i, %bb12.i962.loopexit.i ]
  %.lcssa1059010734.i = phi <8 x float> [ %.promoted10732.i, %bb29.i968.lr.ph.i ], [ %.lcssa1059010733.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.41.sroa.0.0.lcssa10731.i = phi <8 x float> [ %history.i.i894.sroa.41.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.41.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.38.sroa.0.0.lcssa10730.i = phi <8 x float> [ %history.i.i894.sroa.38.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.38.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.35.sroa.0.0.lcssa10729.i = phi <8 x float> [ %history.i.i894.sroa.35.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.35.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.32.sroa.0.0.lcssa10728.i = phi <8 x float> [ %history.i.i894.sroa.32.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.32.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.29.sroa.0.0.lcssa10727.i = phi <8 x float> [ %history.i.i894.sroa.29.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.29.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.25.sroa.0.0.lcssa10726.i = phi <8 x float> [ %history.i.i894.sroa.25.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.25.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.22.sroa.0.0.lcssa10725.i = phi <8 x float> [ %history.i.i894.sroa.22.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.22.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.19.sroa.0.0.lcssa10724.i = phi <8 x float> [ %history.i.i894.sroa.19.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.19.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.16.sroa.0.0.lcssa10723.i = phi <8 x float> [ %history.i.i894.sroa.16.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.16.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.13.sroa.0.0.lcssa10722.i = phi <8 x float> [ %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.13.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.10.sroa.0.0.lcssa10721.i = phi <8 x float> [ %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.10.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %history.i.i894.sroa.0.0.lcssa10702.i = phi <8 x float> [ %hot_left.i948.promoted.i, %bb29.i968.lr.ph.i ], [ %history.i.i894.sroa.0.0.lcssa.i, %bb12.i962.loopexit.i ]
  %indvars.iv.i = phi i64 [ %_31, %bb29.i968.lr.ph.i ], [ %indvars.iv.next.i, %bb12.i962.loopexit.i ]
  %main_cursor.sroa.0.0.i9667957.i = phi i64 [ %64, %bb29.i968.lr.ph.i ], [ %main_cursor.sroa.0.1.i979.lcssa.i, %bb12.i962.loopexit.i ]
  %ring_cursor.sroa.0.0.i9657956.i = phi i64 [ %63, %bb29.i968.lr.ph.i ], [ %ring_cursor.sroa.0.1.i978.lcssa.i, %bb12.i962.loopexit.i ]
  %iter3.sroa.0.0.i9647955.i = phi i64 [ %yield_count.sroa.0.0.i.i.i, %bb29.i968.lr.ph.i ], [ %134, %bb12.i962.loopexit.i ]
  %iter.sroa.0.0.i9637954.i = phi i64 [ 0, %bb29.i968.lr.ph.i ], [ %133, %bb12.i962.loopexit.i ]
  %132 = tail call i64 @llvm.umax.i64(i64 %indvars.iv.i, i64 1), !dbg !6303
  %umax9592.i = tail call i64 @llvm.umin.i64(i64 %132, i64 32), !dbg !6303
  %133 = add nuw nsw i64 %iter.sroa.0.0.i9637954.i, 32, !dbg !6303
  %134 = add nsw i64 %iter3.sroa.0.0.i9647955.i, -1, !dbg !6307
  %_20.i.i9747671.not.i = icmp eq i64 %iter.sroa.0.0.i9637954.i, %_31, !dbg !6308
  br i1 %_20.i.i9747671.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i, label %bb5.i.i1066.lr.ph.i, !dbg !6321

bb5.i.i1066.lr.ph.i:                              ; preds = %bb29.i968.i
  %_5.i2469.i = load <8 x float>, ptr %self, align 32, !alias.scope !5901, !noalias !5902
  %_14.i.i.i.i859.sroa.0.0.copyload.i = load <8 x float>, ptr %68, align 32, !alias.scope !5901, !noalias !5902
  %_17.i.i.i.i856.sroa.0.0.copyload.i = load <8 x float>, ptr %69, align 32, !alias.scope !5901, !noalias !5902
  %_20.i.i.i.i853.sroa.0.0.copyload.i = load <8 x float>, ptr %70, align 32, !alias.scope !5901, !noalias !5902
  %_25.i.i.i.i849.sroa.0.0.copyload.i = load <8 x float>, ptr %row12.i.i.i.i1071.i, align 32, !alias.scope !5901, !noalias !5902
  %_28.i.i.i.i846.sroa.0.0.copyload.i = load <8 x float>, ptr %71, align 32, !alias.scope !5901, !noalias !5902
  %_31.i.i.i.i843.sroa.0.0.copyload.i = load <8 x float>, ptr %72, align 32, !alias.scope !5901, !noalias !5902
  %_34.i.i.i.i840.sroa.0.0.copyload.i = load <8 x float>, ptr %73, align 32, !alias.scope !5901, !noalias !5902
  %_39.i.i.i.i836.sroa.0.0.copyload.i = load <8 x float>, ptr %row13.i.i.i.i1072.i, align 32, !alias.scope !5901, !noalias !5902
  %_42.i.i.i.i833.sroa.0.0.copyload.i = load <8 x float>, ptr %74, align 32, !alias.scope !5901, !noalias !5902
  %_45.i.i.i.i830.sroa.0.0.copyload.i = load <8 x float>, ptr %75, align 32, !alias.scope !5901, !noalias !5902
  %_48.i.i.i.i827.sroa.0.0.copyload.i = load <8 x float>, ptr %76, align 32, !alias.scope !5901, !noalias !5902
  %_53.i.i.i.i823.sroa.0.0.copyload.i = load <8 x float>, ptr %row14.i.i.i.i1073.i, align 32, !alias.scope !5901, !noalias !5902
  %_56.i.i.i.i820.sroa.0.0.copyload.i = load <8 x float>, ptr %77, align 32, !alias.scope !5901, !noalias !5902
  %_59.i.i.i.i817.sroa.0.0.copyload.i = load <8 x float>, ptr %78, align 32, !alias.scope !5901, !noalias !5902
  %_62.i.i.i.i814.sroa.0.0.copyload.i = load <8 x float>, ptr %79, align 32, !alias.scope !5901, !noalias !5902
  %_67.i.i.i.i810.sroa.0.0.copyload.i = load <8 x float>, ptr %row15.i.i.i.i1074.i, align 32, !alias.scope !5901, !noalias !5902
  %_70.i.i.i.i807.sroa.0.0.copyload.i = load <8 x float>, ptr %80, align 32, !alias.scope !5901, !noalias !5902
  %_73.i.i.i.i804.sroa.0.0.copyload.i = load <8 x float>, ptr %81, align 32, !alias.scope !5901, !noalias !5902
  %_76.i.i.i.i801.sroa.0.0.copyload.i = load <8 x float>, ptr %82, align 32, !alias.scope !5901, !noalias !5902
  %_81.i.i.i.i797.sroa.0.0.copyload.i = load <8 x float>, ptr %row16.i.i.i.i1075.i, align 32, !alias.scope !5901, !noalias !5902
  %_84.i.i.i.i794.sroa.0.0.copyload.i = load <8 x float>, ptr %83, align 32, !alias.scope !5901, !noalias !5902
  %_87.i.i.i.i791.sroa.0.0.copyload.i = load <8 x float>, ptr %84, align 32, !alias.scope !5901, !noalias !5902
  %_90.i.i.i.i788.sroa.0.0.copyload.i = load <8 x float>, ptr %85, align 32, !alias.scope !5901, !noalias !5902
  %_95.i.i.i.i784.sroa.0.0.copyload.i = load <8 x float>, ptr %row17.i.i.i.i1076.i, align 32, !alias.scope !5901, !noalias !5902
  %_98.i.i.i.i781.sroa.0.0.copyload.i = load <8 x float>, ptr %86, align 32, !alias.scope !5901, !noalias !5902
  %_101.i.i.i.i778.sroa.0.0.copyload.i = load <8 x float>, ptr %87, align 32, !alias.scope !5901, !noalias !5902
  %_104.i.i.i.i775.sroa.0.0.copyload.i = load <8 x float>, ptr %88, align 32, !alias.scope !5901, !noalias !5902
  %_109.i.i.i.i771.sroa.0.0.copyload.i = load <8 x float>, ptr %row18.i.i.i.i1077.i, align 32, !alias.scope !5901, !noalias !5902
  %_112.i.i.i.i768.sroa.0.0.copyload.i = load <8 x float>, ptr %89, align 32, !alias.scope !5901, !noalias !5902
  %_115.i.i.i.i765.sroa.0.0.copyload.i = load <8 x float>, ptr %90, align 32, !alias.scope !5901, !noalias !5902
  %_118.i.i.i.i762.sroa.0.0.copyload.i = load <8 x float>, ptr %91, align 32, !alias.scope !5901, !noalias !5902
  %_123.i.i.i.i758.sroa.0.0.copyload.i = load <8 x float>, ptr %row19.i.i.i.i1078.i, align 32, !alias.scope !5901, !noalias !5902
  %_126.i.i.i.i755.sroa.0.0.copyload.i = load <8 x float>, ptr %92, align 32, !alias.scope !5901, !noalias !5902
  %_129.i.i.i.i752.sroa.0.0.copyload.i = load <8 x float>, ptr %93, align 32, !alias.scope !5901, !noalias !5902
  %_132.i.i.i.i749.sroa.0.0.copyload.i = load <8 x float>, ptr %94, align 32, !alias.scope !5901, !noalias !5902
  %_137.i.i.i.i745.sroa.0.0.copyload.i = load <8 x float>, ptr %row20.i.i.i.i1079.i, align 32, !alias.scope !5901, !noalias !5902
  %_140.i.i.i.i742.sroa.0.0.copyload.i = load <8 x float>, ptr %95, align 32, !alias.scope !5901, !noalias !5902
  %_143.i.i.i.i739.sroa.0.0.copyload.i = load <8 x float>, ptr %96, align 32, !alias.scope !5901, !noalias !5902
  %_146.i.i.i.i736.sroa.0.0.copyload.i = load <8 x float>, ptr %97, align 32, !alias.scope !5901, !noalias !5902
  %_151.i.i.i.i732.sroa.0.0.copyload.i = load <8 x float>, ptr %row21.i.i.i.i1080.i, align 32, !alias.scope !5901, !noalias !5902
  %_154.i.i.i.i729.sroa.0.0.copyload.i = load <8 x float>, ptr %98, align 32, !alias.scope !5901, !noalias !5902
  %_157.i.i.i.i726.sroa.0.0.copyload.i = load <8 x float>, ptr %99, align 32, !alias.scope !5901, !noalias !5902
  %_160.i.i.i.i723.sroa.0.0.copyload.i = load <8 x float>, ptr %100, align 32, !alias.scope !5901, !noalias !5902
  %_165.i.i.i.i719.sroa.0.0.copyload.i = load <8 x float>, ptr %row22.i.i.i.i1081.i, align 32, !alias.scope !5901, !noalias !5902
  %_168.i.i.i.i716.sroa.0.0.copyload.i = load <8 x float>, ptr %101, align 32, !alias.scope !5901, !noalias !5902
  %_171.i.i.i.i713.sroa.0.0.copyload.i = load <8 x float>, ptr %102, align 32, !alias.scope !5901, !noalias !5902
  %_174.i.i.i.i710.sroa.0.0.copyload.i = load <8 x float>, ptr %103, align 32, !alias.scope !5901, !noalias !5902
  br label %bb5.i.i1066.i, !dbg !6321

bb5.i.i1066.i:                                    ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i, %bb5.i.i1066.lr.ph.i
  %iter.sroa.0.0.i.i9737683.i = phi i64 [ 0, %bb5.i.i1066.lr.ph.i ], [ %135, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.10.sroa.0.07682.i = phi <8 x float> [ %history.i.i894.sroa.10.sroa.0.0.lcssa10721.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.0.07672.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.13.sroa.0.07681.i = phi <8 x float> [ %history.i.i894.sroa.13.sroa.0.0.lcssa10722.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.10.sroa.0.07682.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.16.sroa.0.07680.i = phi <8 x float> [ %history.i.i894.sroa.16.sroa.0.0.lcssa10723.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.13.sroa.0.07681.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.19.sroa.0.07679.i = phi <8 x float> [ %history.i.i894.sroa.19.sroa.0.0.lcssa10724.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.16.sroa.0.07680.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.22.sroa.0.07678.i = phi <8 x float> [ %history.i.i894.sroa.22.sroa.0.0.lcssa10725.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.19.sroa.0.07679.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.38.sroa.0.07677.i = phi <8 x float> [ %history.i.i894.sroa.38.sroa.0.0.lcssa10730.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.35.sroa.0.07676.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.35.sroa.0.07676.i = phi <8 x float> [ %history.i.i894.sroa.35.sroa.0.0.lcssa10729.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.32.sroa.0.07675.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.32.sroa.0.07675.i = phi <8 x float> [ %history.i.i894.sroa.32.sroa.0.0.lcssa10728.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.29.sroa.0.07674.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.29.sroa.0.07674.i = phi <8 x float> [ %history.i.i894.sroa.29.sroa.0.0.lcssa10727.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.25.sroa.0.07673.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.25.sroa.0.07673.i = phi <8 x float> [ %history.i.i894.sroa.25.sroa.0.0.lcssa10726.i, %bb5.i.i1066.lr.ph.i ], [ %history.i.i894.sroa.22.sroa.0.07678.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %history.i.i894.sroa.0.07672.i = phi <8 x float> [ %history.i.i894.sroa.0.0.lcssa10702.i, %bb5.i.i1066.lr.ph.i ], [ %lanes.i3092.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ]
  %135 = add nuw nsw i64 %iter.sroa.0.0.i.i9737683.i, 1, !dbg !6322
  %_11.i19.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i9737683.i, %iter.sroa.0.0.i9637954.i, !dbg !6328
  %base.i.i1067.i = shl i64 %_11.i19.i.i, 3, !dbg !6328
  %_24.i.i1068.i = icmp samesign ugt i64 %base.i.i1067.i, %_39.1, !dbg !6330
  br i1 %_24.i.i1068.i, label %bb7.i.i1094.i, label %bb8.i.i1069.i, !dbg !6330, !prof !639

bb8.i.i1069.i:                                    ; preds = %bb5.i.i1066.i
  %_27.i.i1070.i = sub nuw nsw i64 %_39.1, %base.i.i1067.i, !dbg !6336
  %_8.i3095.i = icmp samesign ugt i64 %_27.i.i1070.i, 7, !dbg !6337
  br i1 %_8.i3095.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i, label %bb2.i3096.i, !dbg !6337, !prof !651

bb2.i3096.i:                                      ; preds = %bb8.i.i1069.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i2752, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i2771, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i1070.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !6348, !noalias !6349
  unreachable, !dbg !6348

bb7.i.i1094.i:                                    ; preds = %bb5.i.i1066.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i2752, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i2771, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i1067.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !6354, !noalias !6355
  unreachable, !dbg !6354

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i: ; preds = %bb8.i.i1069.i
  %_31.i20.i.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i.i1067.i, !dbg !6356
  %lanes.i3092.sroa.0.0.copyload.i = load <8 x float>, ptr %_31.i20.i.i, align 4, !dbg !6361, !alias.scope !6366, !noalias !6370
  %136 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i894.sroa.22.sroa.0.07678.i), !dbg !6372
  %137 = fmul <8 x float> %_5.i2469.i, %lanes.i3092.sroa.0.0.copyload.i, !dbg !6390
  %138 = fadd <8 x float> %137, zeroinitializer, !dbg !6409
  %139 = fmul <8 x float> %_14.i.i.i.i859.sroa.0.0.copyload.i, %lanes.i3092.sroa.0.0.copyload.i, !dbg !6419
  %140 = fadd <8 x float> %139, zeroinitializer, !dbg !6424
  %141 = fmul <8 x float> %_17.i.i.i.i856.sroa.0.0.copyload.i, %lanes.i3092.sroa.0.0.copyload.i, !dbg !6429
  %142 = fadd <8 x float> %141, zeroinitializer, !dbg !6434
  %143 = fmul <8 x float> %_20.i.i.i.i853.sroa.0.0.copyload.i, %lanes.i3092.sroa.0.0.copyload.i, !dbg !6439
  %144 = fadd <8 x float> %143, zeroinitializer, !dbg !6444
  %145 = fmul <8 x float> %_25.i.i.i.i849.sroa.0.0.copyload.i, %history.i.i894.sroa.0.07672.i, !dbg !6449
  %146 = fadd <8 x float> %145, %138, !dbg !6456
  %147 = fmul <8 x float> %_28.i.i.i.i846.sroa.0.0.copyload.i, %history.i.i894.sroa.0.07672.i, !dbg !6461
  %148 = fadd <8 x float> %147, %140, !dbg !6466
  %149 = fmul <8 x float> %_31.i.i.i.i843.sroa.0.0.copyload.i, %history.i.i894.sroa.0.07672.i, !dbg !6471
  %150 = fadd <8 x float> %149, %142, !dbg !6476
  %151 = fmul <8 x float> %_34.i.i.i.i840.sroa.0.0.copyload.i, %history.i.i894.sroa.0.07672.i, !dbg !6481
  %152 = fadd <8 x float> %151, %144, !dbg !6486
  %153 = fmul <8 x float> %_39.i.i.i.i836.sroa.0.0.copyload.i, %history.i.i894.sroa.10.sroa.0.07682.i, !dbg !6491
  %154 = fadd <8 x float> %153, %146, !dbg !6498
  %155 = fmul <8 x float> %_42.i.i.i.i833.sroa.0.0.copyload.i, %history.i.i894.sroa.10.sroa.0.07682.i, !dbg !6503
  %156 = fadd <8 x float> %155, %148, !dbg !6508
  %157 = fmul <8 x float> %_45.i.i.i.i830.sroa.0.0.copyload.i, %history.i.i894.sroa.10.sroa.0.07682.i, !dbg !6513
  %158 = fadd <8 x float> %157, %150, !dbg !6518
  %159 = fmul <8 x float> %_48.i.i.i.i827.sroa.0.0.copyload.i, %history.i.i894.sroa.10.sroa.0.07682.i, !dbg !6523
  %160 = fadd <8 x float> %159, %152, !dbg !6528
  %161 = fmul <8 x float> %_53.i.i.i.i823.sroa.0.0.copyload.i, %history.i.i894.sroa.13.sroa.0.07681.i, !dbg !6533
  %162 = fadd <8 x float> %161, %154, !dbg !6540
  %163 = fmul <8 x float> %_56.i.i.i.i820.sroa.0.0.copyload.i, %history.i.i894.sroa.13.sroa.0.07681.i, !dbg !6545
  %164 = fadd <8 x float> %163, %156, !dbg !6550
  %165 = fmul <8 x float> %_59.i.i.i.i817.sroa.0.0.copyload.i, %history.i.i894.sroa.13.sroa.0.07681.i, !dbg !6555
  %166 = fadd <8 x float> %165, %158, !dbg !6560
  %167 = fmul <8 x float> %_62.i.i.i.i814.sroa.0.0.copyload.i, %history.i.i894.sroa.13.sroa.0.07681.i, !dbg !6565
  %168 = fadd <8 x float> %167, %160, !dbg !6570
  %169 = fmul <8 x float> %_67.i.i.i.i810.sroa.0.0.copyload.i, %history.i.i894.sroa.16.sroa.0.07680.i, !dbg !6575
  %170 = fadd <8 x float> %169, %162, !dbg !6582
  %171 = fmul <8 x float> %_70.i.i.i.i807.sroa.0.0.copyload.i, %history.i.i894.sroa.16.sroa.0.07680.i, !dbg !6587
  %172 = fadd <8 x float> %171, %164, !dbg !6592
  %173 = fmul <8 x float> %_73.i.i.i.i804.sroa.0.0.copyload.i, %history.i.i894.sroa.16.sroa.0.07680.i, !dbg !6597
  %174 = fadd <8 x float> %173, %166, !dbg !6602
  %175 = fmul <8 x float> %_76.i.i.i.i801.sroa.0.0.copyload.i, %history.i.i894.sroa.16.sroa.0.07680.i, !dbg !6607
  %176 = fadd <8 x float> %175, %168, !dbg !6612
  %177 = fmul <8 x float> %_81.i.i.i.i797.sroa.0.0.copyload.i, %history.i.i894.sroa.19.sroa.0.07679.i, !dbg !6617
  %178 = fadd <8 x float> %177, %170, !dbg !6624
  %179 = fmul <8 x float> %_84.i.i.i.i794.sroa.0.0.copyload.i, %history.i.i894.sroa.19.sroa.0.07679.i, !dbg !6629
  %180 = fadd <8 x float> %179, %172, !dbg !6634
  %181 = fmul <8 x float> %_87.i.i.i.i791.sroa.0.0.copyload.i, %history.i.i894.sroa.19.sroa.0.07679.i, !dbg !6639
  %182 = fadd <8 x float> %181, %174, !dbg !6644
  %183 = fmul <8 x float> %_90.i.i.i.i788.sroa.0.0.copyload.i, %history.i.i894.sroa.19.sroa.0.07679.i, !dbg !6649
  %184 = fadd <8 x float> %183, %176, !dbg !6654
  %185 = fmul <8 x float> %_95.i.i.i.i784.sroa.0.0.copyload.i, %history.i.i894.sroa.22.sroa.0.07678.i, !dbg !6659
  %186 = fadd <8 x float> %185, %178, !dbg !6666
  %187 = fmul <8 x float> %_98.i.i.i.i781.sroa.0.0.copyload.i, %history.i.i894.sroa.22.sroa.0.07678.i, !dbg !6671
  %188 = fadd <8 x float> %187, %180, !dbg !6676
  %189 = fmul <8 x float> %_101.i.i.i.i778.sroa.0.0.copyload.i, %history.i.i894.sroa.22.sroa.0.07678.i, !dbg !6681
  %190 = fadd <8 x float> %189, %182, !dbg !6686
  %191 = fmul <8 x float> %_104.i.i.i.i775.sroa.0.0.copyload.i, %history.i.i894.sroa.22.sroa.0.07678.i, !dbg !6691
  %192 = fadd <8 x float> %191, %184, !dbg !6696
  %193 = fmul <8 x float> %_109.i.i.i.i771.sroa.0.0.copyload.i, %history.i.i894.sroa.25.sroa.0.07673.i, !dbg !6701
  %194 = fadd <8 x float> %193, %186, !dbg !6708
  %195 = fmul <8 x float> %_112.i.i.i.i768.sroa.0.0.copyload.i, %history.i.i894.sroa.25.sroa.0.07673.i, !dbg !6713
  %196 = fadd <8 x float> %195, %188, !dbg !6718
  %197 = fmul <8 x float> %_115.i.i.i.i765.sroa.0.0.copyload.i, %history.i.i894.sroa.25.sroa.0.07673.i, !dbg !6723
  %198 = fadd <8 x float> %197, %190, !dbg !6728
  %199 = fmul <8 x float> %_118.i.i.i.i762.sroa.0.0.copyload.i, %history.i.i894.sroa.25.sroa.0.07673.i, !dbg !6733
  %200 = fadd <8 x float> %199, %192, !dbg !6738
  %201 = fmul <8 x float> %_123.i.i.i.i758.sroa.0.0.copyload.i, %history.i.i894.sroa.29.sroa.0.07674.i, !dbg !6743
  %202 = fadd <8 x float> %201, %194, !dbg !6750
  %203 = fmul <8 x float> %_126.i.i.i.i755.sroa.0.0.copyload.i, %history.i.i894.sroa.29.sroa.0.07674.i, !dbg !6755
  %204 = fadd <8 x float> %203, %196, !dbg !6760
  %205 = fmul <8 x float> %_129.i.i.i.i752.sroa.0.0.copyload.i, %history.i.i894.sroa.29.sroa.0.07674.i, !dbg !6765
  %206 = fadd <8 x float> %205, %198, !dbg !6770
  %207 = fmul <8 x float> %_132.i.i.i.i749.sroa.0.0.copyload.i, %history.i.i894.sroa.29.sroa.0.07674.i, !dbg !6775
  %208 = fadd <8 x float> %207, %200, !dbg !6780
  %209 = fmul <8 x float> %_137.i.i.i.i745.sroa.0.0.copyload.i, %history.i.i894.sroa.32.sroa.0.07675.i, !dbg !6785
  %210 = fadd <8 x float> %209, %202, !dbg !6792
  %211 = fmul <8 x float> %_140.i.i.i.i742.sroa.0.0.copyload.i, %history.i.i894.sroa.32.sroa.0.07675.i, !dbg !6797
  %212 = fadd <8 x float> %211, %204, !dbg !6802
  %213 = fmul <8 x float> %_143.i.i.i.i739.sroa.0.0.copyload.i, %history.i.i894.sroa.32.sroa.0.07675.i, !dbg !6807
  %214 = fadd <8 x float> %213, %206, !dbg !6812
  %215 = fmul <8 x float> %_146.i.i.i.i736.sroa.0.0.copyload.i, %history.i.i894.sroa.32.sroa.0.07675.i, !dbg !6817
  %216 = fadd <8 x float> %215, %208, !dbg !6822
  %217 = fmul <8 x float> %_151.i.i.i.i732.sroa.0.0.copyload.i, %history.i.i894.sroa.35.sroa.0.07676.i, !dbg !6827
  %218 = fadd <8 x float> %217, %210, !dbg !6834
  %219 = fmul <8 x float> %_154.i.i.i.i729.sroa.0.0.copyload.i, %history.i.i894.sroa.35.sroa.0.07676.i, !dbg !6839
  %220 = fadd <8 x float> %219, %212, !dbg !6844
  %221 = fmul <8 x float> %_157.i.i.i.i726.sroa.0.0.copyload.i, %history.i.i894.sroa.35.sroa.0.07676.i, !dbg !6849
  %222 = fadd <8 x float> %221, %214, !dbg !6854
  %223 = fmul <8 x float> %_160.i.i.i.i723.sroa.0.0.copyload.i, %history.i.i894.sroa.35.sroa.0.07676.i, !dbg !6859
  %224 = fadd <8 x float> %223, %216, !dbg !6864
  %225 = fmul <8 x float> %_165.i.i.i.i719.sroa.0.0.copyload.i, %history.i.i894.sroa.38.sroa.0.07677.i, !dbg !6869
  %226 = fadd <8 x float> %225, %218, !dbg !6876
  %227 = fmul <8 x float> %_168.i.i.i.i716.sroa.0.0.copyload.i, %history.i.i894.sroa.38.sroa.0.07677.i, !dbg !6881
  %228 = fadd <8 x float> %227, %220, !dbg !6886
  %229 = fmul <8 x float> %_171.i.i.i.i713.sroa.0.0.copyload.i, %history.i.i894.sroa.38.sroa.0.07677.i, !dbg !6891
  %230 = fadd <8 x float> %229, %222, !dbg !6896
  %231 = fmul <8 x float> %_174.i.i.i.i710.sroa.0.0.copyload.i, %history.i.i894.sroa.38.sroa.0.07677.i, !dbg !6901
  %232 = fadd <8 x float> %231, %224, !dbg !6906
  %233 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %226), !dbg !6911
  %234 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %136, <8 x float> %233), !dbg !6919
  %235 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %228), !dbg !6911
  %236 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %234, <8 x float> %235), !dbg !6919
  %237 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %230), !dbg !6911
  %238 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %236, <8 x float> %237), !dbg !6919
  %239 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %232), !dbg !6911
  %240 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %238, <8 x float> %239), !dbg !6919
  %_39.i.i1091.idx.i = shl i64 %iter.sroa.0.0.i.i9737683.i, 5, !dbg !6928
  %_39.i.i1091.i = getelementptr inbounds nuw i8, ptr %peaks_left.i941.i, i64 %_39.i.i1091.idx.i, !dbg !6928
  store <8 x float> %240, ptr %_39.i.i1091.i, align 4, !dbg !6939, !alias.scope !6946, !noalias !6950
  %exitcond.not.i = icmp eq i64 %135, %umax9592.i, !dbg !6308
  br i1 %exitcond.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i, label %bb5.i.i1066.i, !dbg !6321

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i, %bb29.i968.i
  %history.i.i894.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.0.0.lcssa10702.i, %bb29.i968.i ], [ %lanes.i3092.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.25.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.25.sroa.0.0.lcssa10726.i, %bb29.i968.i ], [ %history.i.i894.sroa.22.sroa.0.07678.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.29.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.29.sroa.0.0.lcssa10727.i, %bb29.i968.i ], [ %history.i.i894.sroa.25.sroa.0.07673.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.32.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.32.sroa.0.0.lcssa10728.i, %bb29.i968.i ], [ %history.i.i894.sroa.29.sroa.0.07674.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.35.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.35.sroa.0.0.lcssa10729.i, %bb29.i968.i ], [ %history.i.i894.sroa.32.sroa.0.07675.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.38.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.38.sroa.0.0.lcssa10730.i, %bb29.i968.i ], [ %history.i.i894.sroa.35.sroa.0.07676.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.41.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.41.sroa.0.0.lcssa10731.i, %bb29.i968.i ], [ %history.i.i894.sroa.38.sroa.0.07677.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.22.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.22.sroa.0.0.lcssa10725.i, %bb29.i968.i ], [ %history.i.i894.sroa.19.sroa.0.07679.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.19.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.19.sroa.0.0.lcssa10724.i, %bb29.i968.i ], [ %history.i.i894.sroa.16.sroa.0.07680.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.16.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.16.sroa.0.0.lcssa10723.i, %bb29.i968.i ], [ %history.i.i894.sroa.13.sroa.0.07681.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.13.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.13.sroa.0.0.lcssa10722.i, %bb29.i968.i ], [ %history.i.i894.sroa.10.sroa.0.07682.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  %history.i.i894.sroa.10.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i894.sroa.10.sroa.0.0.lcssa10721.i, %bb29.i968.i ], [ %history.i.i894.sroa.0.07672.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit.i ], !dbg !6954
  store <8 x float> %history.i.i894.sroa.16.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.16.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  store <8 x float> %history.i.i894.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.19.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  store <8 x float> %history.i.i894.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.22.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  store <8 x float> %history.i.i894.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.25.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  store <8 x float> %history.i.i894.sroa.29.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.29.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  store <8 x float> %history.i.i894.sroa.32.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.32.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  store <8 x float> %history.i.i894.sroa.35.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.35.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  store <8 x float> %history.i.i894.sroa.38.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.38.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  store <8 x float> %history.i.i894.sroa.41.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.41.0.hot_left.i948.sroa_idx.i, align 32, !dbg !6347, !noalias !6250
  br i1 %_20.i.i9747671.not.i, label %bb12.i962.loopexit.i, label %bb32.i981.lr.ph.i, !dbg !6295

bb32.i981.lr.ph.i:                                ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i975.i
  %_60.i995.i = load i64, ptr %112, align 8, !alias.scope !5901, !noalias !5902
  %_67.i1054.i = load i64, ptr %131, align 8, !alias.scope !5901, !noalias !5902
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3140.i, !dbg !6295

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3140.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i, %bb32.i981.lr.ph.i
  %241 = phi <8 x float> [ %.lcssa1070110751.i, %bb32.i981.lr.ph.i ], [ %294, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %_12.i1415.sroa.0.0.copyload7911.i = phi <8 x float> [ %.lcssa1050010748.i, %bb32.i981.lr.ph.i ], [ %260, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %_11.i.sroa.0.0.copyload7871.i = phi <8 x float> [ %.lcssa1051810745.i, %bb32.i981.lr.ph.i ], [ %259, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %242 = phi <8 x float> [ %.lcssa1053610742.i, %bb32.i981.lr.ph.i ], [ %254, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %_12.i1423.sroa.0.0.copyload7791.i = phi <8 x float> [ %.lcssa1055410739.i, %bb32.i981.lr.ph.i ], [ %252, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %_11.i1424.sroa.0.0.copyload7751.i = phi <8 x float> [ %.lcssa1057210737.i, %bb32.i981.lr.ph.i ], [ %251, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %243 = phi <8 x float> [ %.lcssa1059010734.i, %bb32.i981.lr.ph.i ], [ %246, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %main_cursor.sroa.0.1.i9797709.i = phi i64 [ %main_cursor.sroa.0.0.i9667957.i, %bb32.i981.lr.ph.i ], [ %spec.store.select.i1056.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %ring_cursor.sroa.0.1.i9787708.i = phi i64 [ %ring_cursor.sroa.0.0.i9657956.i, %bb32.i981.lr.ph.i ], [ %spec.store.select9.i1058.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %iter2.sroa.0.0.i9777707.i = phi i64 [ 0, %bb32.i981.lr.ph.i ], [ %244, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i ]
  %244 = add nuw nsw i64 %iter2.sroa.0.0.i9777707.i, 1, !dbg !6955
  %_43.i982.i = add nuw nsw i64 %iter2.sroa.0.0.i9777707.i, %iter.sroa.0.0.i9637954.i, !dbg !6961
  %base.i983.i = shl i64 %_43.i982.i, 3, !dbg !6961
  %245 = fadd <8 x float> %243, splat (float -1.000000e+00), !dbg !6962
  %246 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %245, <8 x float> zeroinitializer), !dbg !6972
  %247 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %246, <8 x float> zeroinitializer, i8 30), !dbg !6977
  %248 = fadd <8 x float> %_12.i1423.sroa.0.0.copyload7791.i, %_11.i1424.sroa.0.0.copyload7751.i, !dbg !6989
  %249 = bitcast <8 x float> %247 to <8 x i32>, !dbg !6994
  %250 = icmp slt <8 x i32> %249, zeroinitializer, !dbg !7001
  %251 = select <8 x i1> %250, <8 x float> %248, <8 x float> %_13.i1422.sroa.0.0.copyload.i, !dbg !7001
  %252 = select <8 x i1> %250, <8 x float> %_12.i1423.sroa.0.0.copyload7791.i, <8 x float> zeroinitializer, !dbg !7005
  %253 = fadd <8 x float> %242, splat (float -1.000000e+00), !dbg !7010
  %254 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %253, <8 x float> zeroinitializer), !dbg !7015
  %255 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %254, <8 x float> zeroinitializer, i8 30), !dbg !7020
  %256 = fadd <8 x float> %_12.i1415.sroa.0.0.copyload7911.i, %_11.i.sroa.0.0.copyload7871.i, !dbg !7026
  %257 = bitcast <8 x float> %255 to <8 x i32>, !dbg !7031
  %258 = icmp slt <8 x i32> %257, zeroinitializer, !dbg !7035
  %259 = select <8 x i1> %258, <8 x float> %256, <8 x float> %_13.i1414.sroa.0.0.copyload.i, !dbg !7035
  %260 = select <8 x i1> %258, <8 x float> %_12.i1415.sroa.0.0.copyload7911.i, <8 x float> zeroinitializer, !dbg !7037
  %_92.i990.idx.i = shl i64 %iter2.sroa.0.0.i9777707.i, 5, !dbg !7042
  %_92.i990.i = getelementptr inbounds nuw i8, ptr %peaks_left.i941.i, i64 %_92.i990.idx.i, !dbg !7042
  %lanes.i3133.sroa.0.0.copyload.i = load <8 x float>, ptr %_92.i990.i, align 4, !dbg !7053, !alias.scope !7058, !noalias !7062
  %261 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i3133.sroa.0.0.copyload.i, <8 x float> %lanes.i3133.sroa.0.0.copyload.i), !dbg !7066
  %262 = select <8 x i1> %111, <8 x float> %261, <8 x float> %lanes.i3133.sroa.0.0.copyload.i, !dbg !7071
  %_93.i991.i = icmp samesign ugt i64 %base.i983.i, %_39.1, !dbg !7076
  br i1 %_93.i991.i, label %bb36.i1064.i, label %bb37.i992.i, !dbg !7076, !prof !639

bb37.i992.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3140.i
  %_96.i993.i = sub nuw nsw i64 %_39.1, %base.i983.i, !dbg !7080
  %_100.i994.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i983.i, !dbg !7081
  %_8.i3127.i = icmp samesign ugt i64 %_96.i993.i, 7, !dbg !7086
  br i1 %_8.i3127.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3131.i, label %bb2.i3128.i, !dbg !7086, !prof !651

bb2.i3128.i:                                      ; preds = %bb37.i992.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_96.i993.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7091, !noalias !7092
  unreachable, !dbg !7091

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3131.i: ; preds = %bb37.i992.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7096), !dbg !7099
  %width.i.i996.i = load i64, ptr %113, align 8, !dbg !7100, !alias.scope !7101, !noalias !7102, !noundef !12
  %263 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %262, <8 x float> %251, i8 30), !dbg !7111
  %264 = fdiv <8 x float> %251, %262, !dbg !7117
  %265 = bitcast <8 x float> %263 to <8 x i32>, !dbg !7127
  %266 = icmp slt <8 x i32> %265, zeroinitializer, !dbg !7131
  %267 = select <8 x i1> %266, <8 x float> %264, <8 x float> splat (float 1.000000e+00), !dbg !7131
  %_144.1.i.i997.i = load i64, ptr %114, align 8, !dbg !7133, !alias.scope !7101, !noalias !7102, !noundef !12
  %_22.i.i998.i = mul i64 %width.i.i996.i, %ring_cursor.sroa.0.1.i9787708.i, !dbg !7134
  %_92.i.i999.i = icmp ugt i64 %_22.i.i998.i, %_144.1.i.i997.i, !dbg !7135
  br i1 %_92.i.i999.i, label %bb37.i.i1063.i, label %bb38.i.i1000.i, !dbg !7135, !prof !639

bb38.i.i1000.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3131.i
  %_95.i.i1002.i = sub nuw i64 %_144.1.i.i997.i, %_22.i.i998.i, !dbg !7140
  %_8.i3524.i = icmp samesign ugt i64 %_95.i.i1002.i, 7, !dbg !7141
  br i1 %_8.i3524.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3527.i, label %bb2.i3525.i, !dbg !7141, !prof !651

bb2.i3525.i:                                      ; preds = %bb38.i.i1000.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i1002.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7149, !noalias !7150
  unreachable, !dbg !7149

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3527.i: ; preds = %bb38.i.i1000.i
  %_144.0.i.i1001.i = load ptr, ptr %115, align 8, !dbg !7133, !alias.scope !7101, !noalias !7102, !nonnull !12, !noundef !12
  %_99.i.i1003.i = getelementptr inbounds nuw float, ptr %_144.0.i.i1001.i, i64 %_22.i.i998.i, !dbg !7154
  store <8 x float> %267, ptr %_99.i.i1003.i, align 4, !dbg !7159, !alias.scope !7163, !noalias !7167
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7169), !dbg !7172
  %width.i.i = load i64, ptr %113, align 8, !dbg !7173, !alias.scope !7175, !noalias !7176, !noundef !12
  %268 = icmp eq i64 %width.i.i, 0, !dbg !7178
  br i1 %268, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i, label %bb32.i1211.lr.ph.i, !dbg !7178

bb32.i1211.lr.ph.i:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3527.i
  %_112.1.i.i = load i64, ptr %116, align 8, !alias.scope !7175, !noalias !7176, !noundef !12
  %_112.0.i.i = load ptr, ptr %117, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %269 = add i64 %ring_cursor.sroa.0.1.i9787708.i, 1
  %_23.not.i.i = icmp ult i64 %269, %_60.i995.i
  %270 = select i1 %_23.not.i.i, i64 0, i64 %_60.i995.i
  %start1.sroa.0.0.i.i = sub nuw i64 %269, %270
  %_114.1.i.i = load i64, ptr %114, align 8, !alias.scope !5901, !noalias !5902
  %_114.0.i.i = load ptr, ptr %115, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %_116.1.i.i = load i64, ptr %118, align 8, !alias.scope !5901, !noalias !5902
  %_116.0.i.i = load ptr, ptr %119, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %_118.1.i.i = load i64, ptr %120, align 8, !alias.scope !5901, !noalias !5902
  %_118.0.i.i = load ptr, ptr %121, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %_45.i.i = mul i64 %width.i.i, %start1.sroa.0.0.i.i
  br label %bb32.i1211.i, !dbg !7178

bb32.i1211.i:                                     ; preds = %bb31.i.i, %bb32.i1211.lr.ph.i
  %iter.i1209.sroa.10.07700.i = phi i64 [ %width.i.i, %bb32.i1211.lr.ph.i ], [ %271, %bb31.i.i ]
  %iter.i1209.sroa.7.07699.i = phi i64 [ 0, %bb32.i1211.lr.ph.i ], [ %_9.0.i.i, %bb31.i.i ]
  %iter.i1209.sroa.0.0.idx7698.i = phi i64 [ 0, %bb32.i1211.lr.ph.i ], [ %iter.i1209.sroa.0.0.add.i, %bb31.i.i ]
  %iter.i1209.sroa.0.0.ptr7701.i = getelementptr inbounds nuw i8, ptr %scratch.i942.i, i64 %iter.i1209.sroa.0.0.idx7698.i, !dbg !7180
  %271 = add i64 %iter.i1209.sroa.10.07700.i, -1, !dbg !7180
  %_7.i.i.i = icmp eq i64 %iter.i1209.sroa.0.0.idx7698.i, 32, !dbg !7181
  br i1 %_7.i.i.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i, label %bb3.i1213.i, !dbg !7185

bb3.i1213.i:                                      ; preds = %bb32.i1211.i
  %iter.i1209.sroa.0.0.add.i = add nuw nsw i64 %iter.i1209.sroa.0.0.idx7698.i, 4, !dbg !7186
  %_9.0.i.i = add nuw nsw i64 %iter.i1209.sroa.7.07699.i, 1, !dbg !7188
  %exitcond9583.not.i = icmp eq i64 %iter.i1209.sroa.7.07699.i, %_112.1.i.i, !dbg !7189
  br i1 %exitcond9583.not.i, label %panic.i.i, label %bb5.i.i, !dbg !7189

bb5.i.i:                                          ; preds = %bb3.i1213.i
  %272 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i.i, i64 %iter.i1209.sroa.7.07699.i, !dbg !7189
  %shape.i.i = load i32, ptr %272, align 4, !dbg !7189, !noalias !7190, !noundef !12
  %273 = getelementptr inbounds nuw i8, ptr %272, i64 4, !dbg !7189
  %shape3.i.i = load i32, ptr %273, align 4, !dbg !7189, !noalias !7190, !noundef !12
  %window.i.i = zext i32 %shape.i.i to i64, !dbg !7191
  %_19.i.i = zext i32 %shape3.i.i to i64, !dbg !7192
  %274 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_19.i.i, !dbg !7193
  %_20.not.i.i = icmp ult i64 %274, %_60.i995.i, !dbg !7194
  %275 = select i1 %_20.not.i.i, i64 0, i64 %_60.i995.i, !dbg !7194
  %spec.select.i.i = sub nuw i64 %274, %275, !dbg !7194
  %_27.i1215.i = mul i64 %spec.select.i.i, %width.i.i, !dbg !7195
  %_26.i1216.i = add i64 %_27.i1215.i, %iter.i1209.sroa.7.07699.i, !dbg !7195
  %_30.i.i = icmp ult i64 %_26.i1216.i, %_114.1.i.i, !dbg !7196
  br i1 %_30.i.i, label %bb12.i1217.i, label %panic5.i.i, !dbg !7196

panic.i.i:                                        ; preds = %bb3.i1213.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i.i, i64 noundef %_112.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8013bf8450ffb218032f1e27d334efa7) #30, !dbg !7189, !noalias !7190
  unreachable, !dbg !7189

bb12.i1217.i:                                     ; preds = %bb5.i.i
  %276 = getelementptr inbounds nuw float, ptr %_114.0.i.i, i64 %_26.i1216.i, !dbg !7196
  %277 = load float, ptr %276, align 4, !dbg !7196, !noalias !7190, !noundef !12
  %exitcond9584.not.i = icmp eq i64 %iter.i1209.sroa.7.07699.i, %_116.1.i.i, !dbg !7197
  br i1 %exitcond9584.not.i, label %panic6.i.i, label %bb13.i1219.i, !dbg !7197

panic5.i.i:                                       ; preds = %bb5.i.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1216.i, i64 noundef %_114.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d297cbdfce2474defb1d7f11394e8cb6) #30, !dbg !7196, !noalias !7190
  unreachable, !dbg !7196

bb13.i1219.i:                                     ; preds = %bb12.i1217.i
  %278 = getelementptr inbounds nuw i32, ptr %_116.0.i.i, i64 %iter.i1209.sroa.7.07699.i, !dbg !7197
  %_32.i.i = load i32, ptr %278, align 4, !dbg !7197, !noalias !7190, !noundef !12
  %position.i.i = zext i32 %_32.i.i to i64, !dbg !7197
  %279 = icmp eq i32 %_32.i.i, 0, !dbg !7198
  br i1 %279, label %bb17.i1221.i, label %bb15.i.i, !dbg !7198

panic6.i.i:                                       ; preds = %bb12.i1217.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i.i, i64 noundef %_116.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ac016620dad255f0d323022a5b7f5883) #30, !dbg !7197, !noalias !7190
  unreachable, !dbg !7197

bb15.i.i:                                         ; preds = %bb13.i1219.i
  %_37.i.i = icmp ult i64 %iter.i1209.sroa.7.07699.i, %_118.1.i.i, !dbg !7199
  br i1 %_37.i.i, label %bb16.i1220.i, label %panic7.i.i, !dbg !7199

bb17.i1221.i:                                     ; preds = %bb35.i1236.i, %bb16.i1220.i, %bb13.i1219.i
  %newest.sroa.0.0.i.i = phi float [ %277, %bb13.i1219.i ], [ %_35.i.i, %bb35.i1236.i ], [ %277, %bb16.i1220.i ], !dbg !7200
  %exitcond9585.not.i = icmp eq i64 %iter.i1209.sroa.7.07699.i, %_118.1.i.i, !dbg !7201
  br i1 %exitcond9585.not.i, label %panic8.i.i, label %bb18.i.i, !dbg !7201

bb16.i1220.i:                                     ; preds = %bb15.i.i
  %280 = getelementptr inbounds nuw float, ptr %_118.0.i.i, i64 %iter.i1209.sroa.7.07699.i, !dbg !7199
  %_35.i.i = load float, ptr %280, align 4, !dbg !7199, !noalias !7190, !noundef !12
  %_102.i.i = fcmp olt float %_35.i.i, %277, !dbg !7202
  br i1 %_102.i.i, label %bb35.i1236.i, label %bb17.i1221.i, !dbg !7202

panic7.i.i:                                       ; preds = %bb15.i.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1209.sroa.7.07699.i, i64 noundef %_118.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5355958980a78279123102c3494b10a) #30, !dbg !7199, !noalias !7190
  unreachable, !dbg !7199

bb35.i1236.i:                                     ; preds = %bb16.i1220.i
  br label %bb17.i1221.i, !dbg !7204

bb18.i.i:                                         ; preds = %bb17.i1221.i
  %281 = getelementptr inbounds nuw float, ptr %_118.0.i.i, i64 %iter.i1209.sroa.7.07699.i, !dbg !7201
  store float %newest.sroa.0.0.i.i, ptr %281, align 4, !dbg !7201, !noalias !7190
  %_42.i.i = add nuw nsw i64 %position.i.i, 1, !dbg !7205
  %complete.i.i = icmp eq i64 %_42.i.i, %window.i.i, !dbg !7205
  br i1 %complete.i.i, label %bb22.i1224.i, label %bb20.i.i, !dbg !7206

panic8.i.i:                                       ; preds = %bb17.i1221.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i.i, i64 noundef %_118.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2d2a28b8cb03afaaebfea48ffa77c925) #30, !dbg !7201, !noalias !7190
  unreachable, !dbg !7201

bb20.i.i:                                         ; preds = %bb18.i.i
  %_44.i.i = add i64 %iter.i1209.sroa.7.07699.i, %_45.i.i, !dbg !7207
  %_47.i1222.i = icmp ult i64 %_44.i.i, %_114.1.i.i, !dbg !7208
  br i1 %_47.i1222.i, label %bb30.i.i, label %panic9.i.i, !dbg !7208

panic9.i.i:                                       ; preds = %bb20.i.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i.i, i64 noundef %_114.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_53d3a5c3ecf31ea5f14c0a7f8bf40921) #30, !dbg !7208, !noalias !7190
  unreachable, !dbg !7208

bb30.i.i:                                         ; preds = %bb20.i.i
  %282 = getelementptr inbounds nuw float, ptr %_114.0.i.i, i64 %_44.i.i, !dbg !7208
  %_43.i1223.i = load float, ptr %282, align 4, !dbg !7208, !noalias !7190, !noundef !12
  %_103.i.i = fcmp olt float %_43.i1223.i, %newest.sroa.0.0.i.i, !dbg !7209
  %newest.sroa.0.1.i.i = select i1 %_103.i.i, float %_43.i1223.i, float %newest.sroa.0.0.i.i, !dbg !7209
  store float %newest.sroa.0.1.i.i, ptr %iter.i1209.sroa.0.0.ptr7701.i, align 4, !dbg !7211, !noalias !7212
  %283 = trunc i64 %_42.i.i to i32, !dbg !7213
  br label %bb31.i.i, !dbg !7214

bb31.i.i:                                         ; preds = %bb25.i1234.i, %bb30.i.i
  %storemerge.i = phi i32 [ %283, %bb30.i.i ], [ 0, %bb25.i1234.i ], !dbg !7215
  store i32 %storemerge.i, ptr %278, align 4, !dbg !7215, !noalias !7190
  %284 = icmp eq i64 %271, 0, !dbg !7178
  br i1 %284, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i, label %bb32.i1211.i, !dbg !7178

bb22.i1224.i:                                     ; preds = %bb18.i.i
  store float %newest.sroa.0.0.i.i, ptr %iter.i1209.sroa.0.0.ptr7701.i, align 4, !dbg !7211, !noalias !7212
  %285 = load float, ptr %276, align 4, !dbg !7216, !noalias !7190, !noundef !12
  br label %bb41.i1228.i, !dbg !7217

bb41.i1228.i:                                     ; preds = %bb25.i1234.i, %bb22.i1224.i
  %iter2.sroa.0.0.i12257697.i = phi i64 [ 0, %bb22.i1224.i ], [ %_105.i.i, %bb25.i1234.i ]
  %suffix.sroa.0.0.i7696.i = phi float [ %285, %bb22.i1224.i ], [ %suffix.sroa.0.1.i.i, %bb25.i1234.i ]
  %end.sroa.0.1.i7695.i = phi i64 [ %spec.select.i.i, %bb22.i1224.i ], [ %288, %bb25.i1234.i ]
  %_56.i1229.i = mul i64 %end.sroa.0.1.i7695.i, %width.i.i, !dbg !7220
  %_55.i1230.i = add i64 %_56.i1229.i, %iter.i1209.sroa.7.07699.i, !dbg !7220
  %_59.i1231.i = icmp ult i64 %_55.i1230.i, %_114.1.i.i, !dbg !7221
  br i1 %_59.i1231.i, label %bb25.i1234.i, label %panic13.i.i, !dbg !7221

panic13.i.i:                                      ; preds = %bb41.i1228.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1230.i, i64 noundef %_114.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b22b5c926aed02a79660ac772e5ad40d) #30, !dbg !7221, !noalias !7190
  unreachable, !dbg !7221

bb25.i1234.i:                                     ; preds = %bb41.i1228.i
  %_105.i.i = add nuw nsw i64 %iter2.sroa.0.0.i12257697.i, 1, !dbg !7222
  %286 = getelementptr inbounds nuw float, ptr %_114.0.i.i, i64 %_55.i1230.i, !dbg !7221
  %_54.i.i = load float, ptr %286, align 4, !dbg !7221, !noalias !7190, !noundef !12
  %_107.i.i = fcmp olt float %suffix.sroa.0.0.i7696.i, %_54.i.i, !dbg !7225
  %suffix.sroa.0.1.i.i = select i1 %_107.i.i, float %suffix.sroa.0.0.i7696.i, float %_54.i.i, !dbg !7225
  store float %suffix.sroa.0.1.i.i, ptr %286, align 4, !dbg !7227, !noalias !7190
  %287 = icmp eq i64 %end.sroa.0.1.i7695.i, 0, !dbg !7228
  %spec.store.select.i1235.i = select i1 %287, i64 %_60.i995.i, i64 %end.sroa.0.1.i7695.i, !dbg !7228
  %288 = add i64 %spec.store.select.i1235.i, -1, !dbg !7229
  %exitcond9582.not.i = icmp eq i64 %_105.i.i, %window.i.i, !dbg !7230
  br i1 %exitcond9582.not.i, label %bb31.i.i, label %bb41.i1228.i, !dbg !7217

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i: ; preds = %bb31.i.i, %bb32.i1211.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3527.i
  %lanes.i3117.sroa.0.0.copyload.i = load <8 x float>, ptr %scratch.i942.i, align 4, !dbg !7232, !alias.scope !7237, !noalias !7241
  %289 = fmul <8 x float> %lanes.i3117.sroa.0.0.copyload.i, splat (float 1.638400e+04), !dbg !7245
  %290 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %289), !dbg !7250
  %291 = fmul <8 x float> %290, splat (float 0x3F10000000000000), !dbg !7259
  %292 = icmp eq i64 %width.i.i996.i, 0, !dbg !7264
  %_149.1.i.i1031.pre.i = load i64, ptr %122, align 8, !dbg !7269, !alias.scope !7101, !noalias !7102
  br i1 %292, label %bb16.i.i1030.i, label %bb39.i.i1010.lr.ph.i, !dbg !7264

bb39.i.i1010.lr.ph.i:                             ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i
  %_145.1.i.i1013.i = load i64, ptr %116, align 8, !alias.scope !7101, !noalias !7102, !noundef !12
  %_145.0.i.i1017.i = load ptr, ptr %117, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %_147.0.i.i1028.i = load ptr, ptr %123, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %exitcond9588.not.i = icmp eq i64 %_145.1.i.i1013.i, 0, !dbg !7270
  br i1 %exitcond9588.not.i, label %panic.i.i1015.i, label %bb17.i.i1016.i, !dbg !7270

bb37.i.i1063.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3131.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i998.i, i64 noundef %_144.1.i.i997.i, i64 noundef %_144.1.i.i997.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_913d17a5751fc2956adecdab98dac09f) #30, !dbg !7272, !noalias !7273
  unreachable, !dbg !7272

bb16.i.i1030.loopexit.i:                          ; preds = %bb21.i.i1027.7.i, %bb21.i.i1027.6.i, %bb21.i.i1027.5.i, %bb21.i.i1027.4.i, %bb21.i.i1027.3.i, %bb21.i.i1027.2.i, %bb21.i.i1027.1.i, %bb21.i.i1027.i
  %lanes.i3110.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %scratch.i942.i, align 4, !dbg !7274, !alias.scope !7279, !noalias !7283
  br label %bb16.i.i1030.i, !dbg !7287

bb16.i.i1030.i:                                   ; preds = %bb16.i.i1030.loopexit.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i
  %lanes.i3110.sroa.0.0.copyload.i = phi <8 x float> [ %lanes.i3110.sroa.0.0.copyload.pre.i, %bb16.i.i1030.loopexit.i ], [ %lanes.i3117.sroa.0.0.copyload.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit.i ], !dbg !7274
  %293 = fadd <8 x float> %241, %291, !dbg !7288
  %294 = fsub <8 x float> %293, %lanes.i3110.sroa.0.0.copyload.i, !dbg !7293
  %_109.i.i1032.i = icmp ugt i64 %_22.i.i998.i, %_149.1.i.i1031.pre.i, !dbg !7298
  br i1 %_109.i.i1032.i, label %bb42.i.i1062.i, label %bb43.i.i1033.i, !dbg !7298, !prof !639

bb43.i.i1033.i:                                   ; preds = %bb16.i.i1030.i
  %_112.i.i1035.i = sub nuw i64 %_149.1.i.i1031.pre.i, %_22.i.i998.i, !dbg !7302
  %_8.i3519.i = icmp samesign ugt i64 %_112.i.i1035.i, 7, !dbg !7303
  br i1 %_8.i3519.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3522.i, label %bb2.i3520.i, !dbg !7303, !prof !651

bb2.i3520.i:                                      ; preds = %bb43.i.i1033.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i1035.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7308, !noalias !7309
  unreachable, !dbg !7308

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3522.i: ; preds = %bb43.i.i1033.i
  %_149.0.i.i1034.i = load ptr, ptr %123, align 8, !dbg !7269, !alias.scope !7101, !noalias !7102, !nonnull !12, !noundef !12
  %_116.i.i1036.i = getelementptr inbounds nuw float, ptr %_149.0.i.i1034.i, i64 %_22.i.i998.i, !dbg !7313
  store <8 x float> %291, ptr %_116.i.i1036.i, align 4, !dbg !7318, !alias.scope !7322, !noalias !7326
  %_68.i.i906.sroa.0.0.copyload.i = load <8 x float>, ptr %126, align 32, !dbg !7328, !noalias !7331
  %295 = fdiv <8 x float> %294, %_64.i.i910.sroa.0.0.copyload.i, !dbg !7332
  %296 = fsub <8 x float> splat (float 1.000000e+00), %295, !dbg !7337
  %297 = fsub <8 x float> %296, %_68.i.i906.sroa.0.0.copyload.i, !dbg !7342
  %298 = fmul <8 x float> %259, %297, !dbg !7347
  %299 = fadd <8 x float> %_68.i.i906.sroa.0.0.copyload.i, %298, !dbg !7355
  %300 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %296, <8 x float> %299), !dbg !7362
  %301 = bitcast <8 x float> %300 to <8 x i32>, !dbg !7368
  %302 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %300), !dbg !7375
  %303 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %302, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !7377
  %304 = bitcast <8 x float> %303 to <8 x i32>, !dbg !7389
  %305 = xor <8 x i32> %304, splat (i32 -1), !dbg !7401
  %306 = and <8 x i32> %305, %301, !dbg !7403
  %307 = bitcast <8 x i32> %306 to <8 x float>, !dbg !7409
  store <8 x i32> %306, ptr %126, align 32, !dbg !7410, !noalias !7331
  %308 = fsub <8 x float> splat (float 1.000000e+00), %307, !dbg !7411
  %_150.1.i.i1037.i = load i64, ptr %127, align 8, !dbg !7416, !alias.scope !7101, !noalias !7102, !noundef !12
  %_76.i.i1038.i = mul i64 %width.i.i996.i, %main_cursor.sroa.0.1.i9797709.i, !dbg !7418
  %_120.i.i1039.i = icmp ugt i64 %_76.i.i1038.i, %_150.1.i.i1037.i, !dbg !7419
  br i1 %_120.i.i1039.i, label %bb48.i.i1061.i, label %bb49.i.i1040.i, !dbg !7419, !prof !639

bb42.i.i1062.i:                                   ; preds = %bb16.i.i1030.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i998.i, i64 noundef %_149.1.i.i1031.pre.i, i64 noundef %_149.1.i.i1031.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a0dcf875eae79f6708bcf79c3cbff55) #30, !dbg !7424, !noalias !7425
  unreachable, !dbg !7424

bb49.i.i1040.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3522.i
  %_123.i.i1042.i = sub nuw i64 %_150.1.i.i1037.i, %_76.i.i1038.i, !dbg !7426
  %_8.i3104.i = icmp samesign ugt i64 %_123.i.i1042.i, 7, !dbg !7427
  br i1 %_8.i3104.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i, label %bb2.i3105.i, !dbg !7427, !prof !651

bb2.i3105.i:                                      ; preds = %bb49.i.i1040.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i1042.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7432, !noalias !7433
  unreachable, !dbg !7432

bb48.i.i1061.i:                                   ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3522.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i1038.i, i64 noundef %_150.1.i.i1037.i, i64 noundef %_150.1.i.i1037.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5e0b8406fbb9ac3f5f26ac6469c25f7) #30, !dbg !7437, !noalias !7425
  unreachable, !dbg !7437

bb17.i.i1016.i:                                   ; preds = %bb39.i.i1010.lr.ph.i
  %309 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1017.i, i64 8, !dbg !7270
  %_44.i.i1018.i = load i32, ptr %309, align 4, !dbg !7270, !noalias !7425, !noundef !12
  %_43.i.i1019.i = zext i32 %_44.i.i1018.i to i64, !dbg !7270
  %310 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_43.i.i1019.i, !dbg !7438
  %_47.not.i.i1020.i = icmp ult i64 %310, %_60.i995.i, !dbg !7439
  %311 = select i1 %_47.not.i.i1020.i, i64 0, i64 %_60.i995.i, !dbg !7439
  %spec.select.i.i1021.i = sub nuw i64 %310, %311, !dbg !7439
  %_51.i.i1022.i = mul i64 %spec.select.i.i1021.i, %width.i.i996.i, !dbg !7441
  %_53.i.i1025.i = icmp ult i64 %_51.i.i1022.i, %_149.1.i.i1031.pre.i, !dbg !7442
  br i1 %_53.i.i1025.i, label %bb21.i.i1027.i, label %panic1.i.i1026.i, !dbg !7442

panic.i.i1015.i:                                  ; preds = %bb39.i.i1010.7.i, %bb39.i.i1010.6.i, %bb39.i.i1010.5.i, %bb39.i.i1010.4.i, %bb39.i.i1010.3.i, %bb39.i.i1010.2.i, %bb39.i.i1010.1.i, %bb39.i.i1010.lr.ph.i
  %_145.1.i.i1013.lcssa.ph.i = phi i64 [ 7, %bb39.i.i1010.7.i ], [ 6, %bb39.i.i1010.6.i ], [ 5, %bb39.i.i1010.5.i ], [ 4, %bb39.i.i1010.4.i ], [ 3, %bb39.i.i1010.3.i ], [ 2, %bb39.i.i1010.2.i ], [ 1, %bb39.i.i1010.1.i ], [ 0, %bb39.i.i1010.lr.ph.i ]
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i1013.lcssa.ph.i, i64 noundef %_145.1.i.i1013.lcssa.ph.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ff2ed40a8d2df224a47a35441e5e2fe) #30, !dbg !7270, !noalias !7425
  unreachable, !dbg !7270

bb21.i.i1027.i:                                   ; preds = %bb17.i.i1016.i
  %312 = getelementptr inbounds nuw float, ptr %_147.0.i.i1028.i, i64 %_51.i.i1022.i, !dbg !7442
  %_49.i.i1029.i = load float, ptr %312, align 4, !dbg !7442, !noalias !7425, !noundef !12
  store float %_49.i.i1029.i, ptr %scratch.i942.i, align 4, !dbg !7443, !noalias !7444
  %313 = icmp eq i64 %width.i.i996.i, 1, !dbg !7264
  br i1 %313, label %bb16.i.i1030.loopexit.i, label %bb39.i.i1010.1.i, !dbg !7264

bb39.i.i1010.1.i:                                 ; preds = %bb21.i.i1027.i
  %exitcond9588.1.not.i = icmp eq i64 %_145.1.i.i1013.i, 1, !dbg !7270
  br i1 %exitcond9588.1.not.i, label %panic.i.i1015.i, label %bb17.i.i1016.1.i, !dbg !7270

bb17.i.i1016.1.i:                                 ; preds = %bb39.i.i1010.1.i
  %314 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1017.i, i64 20, !dbg !7270
  %_44.i.i1018.1.i = load i32, ptr %314, align 4, !dbg !7270, !noalias !7425, !noundef !12
  %_43.i.i1019.1.i = zext i32 %_44.i.i1018.1.i to i64, !dbg !7270
  %315 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_43.i.i1019.1.i, !dbg !7438
  %_47.not.i.i1020.1.i = icmp ult i64 %315, %_60.i995.i, !dbg !7439
  %316 = select i1 %_47.not.i.i1020.1.i, i64 0, i64 %_60.i995.i, !dbg !7439
  %spec.select.i.i1021.1.i = sub nuw i64 %315, %316, !dbg !7439
  %_51.i.i1022.1.i = mul i64 %spec.select.i.i1021.1.i, %width.i.i996.i, !dbg !7441
  %_50.i.i1023.1.i = add i64 %_51.i.i1022.1.i, 1, !dbg !7441
  %_53.i.i1025.1.i = icmp ult i64 %_50.i.i1023.1.i, %_149.1.i.i1031.pre.i, !dbg !7442
  br i1 %_53.i.i1025.1.i, label %bb21.i.i1027.1.i, label %panic1.i.i1026.i, !dbg !7442

bb21.i.i1027.1.i:                                 ; preds = %bb17.i.i1016.1.i
  %317 = getelementptr inbounds nuw float, ptr %_147.0.i.i1028.i, i64 %_50.i.i1023.1.i, !dbg !7442
  %_49.i.i1029.1.i = load float, ptr %317, align 4, !dbg !7442, !noalias !7425, !noundef !12
  store float %_49.i.i1029.1.i, ptr %iter.i.i916.sroa.0.0.ptr7705.1.i, align 4, !dbg !7443, !noalias !7444
  %318 = icmp eq i64 %width.i.i996.i, 2, !dbg !7264
  br i1 %318, label %bb16.i.i1030.loopexit.i, label %bb39.i.i1010.2.i, !dbg !7264

bb39.i.i1010.2.i:                                 ; preds = %bb21.i.i1027.1.i
  %exitcond9588.2.not.i = icmp eq i64 %_145.1.i.i1013.i, 2, !dbg !7270
  br i1 %exitcond9588.2.not.i, label %panic.i.i1015.i, label %bb17.i.i1016.2.i, !dbg !7270

bb17.i.i1016.2.i:                                 ; preds = %bb39.i.i1010.2.i
  %319 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1017.i, i64 32, !dbg !7270
  %_44.i.i1018.2.i = load i32, ptr %319, align 4, !dbg !7270, !noalias !7425, !noundef !12
  %_43.i.i1019.2.i = zext i32 %_44.i.i1018.2.i to i64, !dbg !7270
  %320 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_43.i.i1019.2.i, !dbg !7438
  %_47.not.i.i1020.2.i = icmp ult i64 %320, %_60.i995.i, !dbg !7439
  %321 = select i1 %_47.not.i.i1020.2.i, i64 0, i64 %_60.i995.i, !dbg !7439
  %spec.select.i.i1021.2.i = sub nuw i64 %320, %321, !dbg !7439
  %_51.i.i1022.2.i = mul i64 %spec.select.i.i1021.2.i, %width.i.i996.i, !dbg !7441
  %_50.i.i1023.2.i = add i64 %_51.i.i1022.2.i, 2, !dbg !7441
  %_53.i.i1025.2.i = icmp ult i64 %_50.i.i1023.2.i, %_149.1.i.i1031.pre.i, !dbg !7442
  br i1 %_53.i.i1025.2.i, label %bb21.i.i1027.2.i, label %panic1.i.i1026.i, !dbg !7442

bb21.i.i1027.2.i:                                 ; preds = %bb17.i.i1016.2.i
  %322 = getelementptr inbounds nuw float, ptr %_147.0.i.i1028.i, i64 %_50.i.i1023.2.i, !dbg !7442
  %_49.i.i1029.2.i = load float, ptr %322, align 4, !dbg !7442, !noalias !7425, !noundef !12
  store float %_49.i.i1029.2.i, ptr %iter.i.i916.sroa.0.0.ptr7705.2.i, align 4, !dbg !7443, !noalias !7444
  %323 = icmp eq i64 %width.i.i996.i, 3, !dbg !7264
  br i1 %323, label %bb16.i.i1030.loopexit.i, label %bb39.i.i1010.3.i, !dbg !7264

bb39.i.i1010.3.i:                                 ; preds = %bb21.i.i1027.2.i
  %exitcond9588.3.not.i = icmp eq i64 %_145.1.i.i1013.i, 3, !dbg !7270
  br i1 %exitcond9588.3.not.i, label %panic.i.i1015.i, label %bb17.i.i1016.3.i, !dbg !7270

bb17.i.i1016.3.i:                                 ; preds = %bb39.i.i1010.3.i
  %324 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1017.i, i64 44, !dbg !7270
  %_44.i.i1018.3.i = load i32, ptr %324, align 4, !dbg !7270, !noalias !7425, !noundef !12
  %_43.i.i1019.3.i = zext i32 %_44.i.i1018.3.i to i64, !dbg !7270
  %325 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_43.i.i1019.3.i, !dbg !7438
  %_47.not.i.i1020.3.i = icmp ult i64 %325, %_60.i995.i, !dbg !7439
  %326 = select i1 %_47.not.i.i1020.3.i, i64 0, i64 %_60.i995.i, !dbg !7439
  %spec.select.i.i1021.3.i = sub nuw i64 %325, %326, !dbg !7439
  %_51.i.i1022.3.i = mul i64 %spec.select.i.i1021.3.i, %width.i.i996.i, !dbg !7441
  %_50.i.i1023.3.i = add i64 %_51.i.i1022.3.i, 3, !dbg !7441
  %_53.i.i1025.3.i = icmp ult i64 %_50.i.i1023.3.i, %_149.1.i.i1031.pre.i, !dbg !7442
  br i1 %_53.i.i1025.3.i, label %bb21.i.i1027.3.i, label %panic1.i.i1026.i, !dbg !7442

bb21.i.i1027.3.i:                                 ; preds = %bb17.i.i1016.3.i
  %327 = getelementptr inbounds nuw float, ptr %_147.0.i.i1028.i, i64 %_50.i.i1023.3.i, !dbg !7442
  %_49.i.i1029.3.i = load float, ptr %327, align 4, !dbg !7442, !noalias !7425, !noundef !12
  store float %_49.i.i1029.3.i, ptr %iter.i.i916.sroa.0.0.ptr7705.3.i, align 4, !dbg !7443, !noalias !7444
  %328 = icmp eq i64 %width.i.i996.i, 4, !dbg !7264
  br i1 %328, label %bb16.i.i1030.loopexit.i, label %bb39.i.i1010.4.i, !dbg !7264

bb39.i.i1010.4.i:                                 ; preds = %bb21.i.i1027.3.i
  %exitcond9588.4.not.i = icmp eq i64 %_145.1.i.i1013.i, 4, !dbg !7270
  br i1 %exitcond9588.4.not.i, label %panic.i.i1015.i, label %bb17.i.i1016.4.i, !dbg !7270

bb17.i.i1016.4.i:                                 ; preds = %bb39.i.i1010.4.i
  %329 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1017.i, i64 56, !dbg !7270
  %_44.i.i1018.4.i = load i32, ptr %329, align 4, !dbg !7270, !noalias !7425, !noundef !12
  %_43.i.i1019.4.i = zext i32 %_44.i.i1018.4.i to i64, !dbg !7270
  %330 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_43.i.i1019.4.i, !dbg !7438
  %_47.not.i.i1020.4.i = icmp ult i64 %330, %_60.i995.i, !dbg !7439
  %331 = select i1 %_47.not.i.i1020.4.i, i64 0, i64 %_60.i995.i, !dbg !7439
  %spec.select.i.i1021.4.i = sub nuw i64 %330, %331, !dbg !7439
  %_51.i.i1022.4.i = mul i64 %spec.select.i.i1021.4.i, %width.i.i996.i, !dbg !7441
  %_50.i.i1023.4.i = add i64 %_51.i.i1022.4.i, 4, !dbg !7441
  %_53.i.i1025.4.i = icmp ult i64 %_50.i.i1023.4.i, %_149.1.i.i1031.pre.i, !dbg !7442
  br i1 %_53.i.i1025.4.i, label %bb21.i.i1027.4.i, label %panic1.i.i1026.i, !dbg !7442

bb21.i.i1027.4.i:                                 ; preds = %bb17.i.i1016.4.i
  %332 = getelementptr inbounds nuw float, ptr %_147.0.i.i1028.i, i64 %_50.i.i1023.4.i, !dbg !7442
  %_49.i.i1029.4.i = load float, ptr %332, align 4, !dbg !7442, !noalias !7425, !noundef !12
  store float %_49.i.i1029.4.i, ptr %iter.i.i916.sroa.0.0.ptr7705.4.i, align 4, !dbg !7443, !noalias !7444
  %333 = icmp eq i64 %width.i.i996.i, 5, !dbg !7264
  br i1 %333, label %bb16.i.i1030.loopexit.i, label %bb39.i.i1010.5.i, !dbg !7264

bb39.i.i1010.5.i:                                 ; preds = %bb21.i.i1027.4.i
  %exitcond9588.5.not.i = icmp eq i64 %_145.1.i.i1013.i, 5, !dbg !7270
  br i1 %exitcond9588.5.not.i, label %panic.i.i1015.i, label %bb17.i.i1016.5.i, !dbg !7270

bb17.i.i1016.5.i:                                 ; preds = %bb39.i.i1010.5.i
  %334 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1017.i, i64 68, !dbg !7270
  %_44.i.i1018.5.i = load i32, ptr %334, align 4, !dbg !7270, !noalias !7425, !noundef !12
  %_43.i.i1019.5.i = zext i32 %_44.i.i1018.5.i to i64, !dbg !7270
  %335 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_43.i.i1019.5.i, !dbg !7438
  %_47.not.i.i1020.5.i = icmp ult i64 %335, %_60.i995.i, !dbg !7439
  %336 = select i1 %_47.not.i.i1020.5.i, i64 0, i64 %_60.i995.i, !dbg !7439
  %spec.select.i.i1021.5.i = sub nuw i64 %335, %336, !dbg !7439
  %_51.i.i1022.5.i = mul i64 %spec.select.i.i1021.5.i, %width.i.i996.i, !dbg !7441
  %_50.i.i1023.5.i = add i64 %_51.i.i1022.5.i, 5, !dbg !7441
  %_53.i.i1025.5.i = icmp ult i64 %_50.i.i1023.5.i, %_149.1.i.i1031.pre.i, !dbg !7442
  br i1 %_53.i.i1025.5.i, label %bb21.i.i1027.5.i, label %panic1.i.i1026.i, !dbg !7442

bb21.i.i1027.5.i:                                 ; preds = %bb17.i.i1016.5.i
  %337 = getelementptr inbounds nuw float, ptr %_147.0.i.i1028.i, i64 %_50.i.i1023.5.i, !dbg !7442
  %_49.i.i1029.5.i = load float, ptr %337, align 4, !dbg !7442, !noalias !7425, !noundef !12
  store float %_49.i.i1029.5.i, ptr %iter.i.i916.sroa.0.0.ptr7705.5.i, align 4, !dbg !7443, !noalias !7444
  %338 = icmp eq i64 %width.i.i996.i, 6, !dbg !7264
  br i1 %338, label %bb16.i.i1030.loopexit.i, label %bb39.i.i1010.6.i, !dbg !7264

bb39.i.i1010.6.i:                                 ; preds = %bb21.i.i1027.5.i
  %exitcond9588.6.not.i = icmp eq i64 %_145.1.i.i1013.i, 6, !dbg !7270
  br i1 %exitcond9588.6.not.i, label %panic.i.i1015.i, label %bb17.i.i1016.6.i, !dbg !7270

bb17.i.i1016.6.i:                                 ; preds = %bb39.i.i1010.6.i
  %339 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1017.i, i64 80, !dbg !7270
  %_44.i.i1018.6.i = load i32, ptr %339, align 4, !dbg !7270, !noalias !7425, !noundef !12
  %_43.i.i1019.6.i = zext i32 %_44.i.i1018.6.i to i64, !dbg !7270
  %340 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_43.i.i1019.6.i, !dbg !7438
  %_47.not.i.i1020.6.i = icmp ult i64 %340, %_60.i995.i, !dbg !7439
  %341 = select i1 %_47.not.i.i1020.6.i, i64 0, i64 %_60.i995.i, !dbg !7439
  %spec.select.i.i1021.6.i = sub nuw i64 %340, %341, !dbg !7439
  %_51.i.i1022.6.i = mul i64 %spec.select.i.i1021.6.i, %width.i.i996.i, !dbg !7441
  %_50.i.i1023.6.i = add i64 %_51.i.i1022.6.i, 6, !dbg !7441
  %_53.i.i1025.6.i = icmp ult i64 %_50.i.i1023.6.i, %_149.1.i.i1031.pre.i, !dbg !7442
  br i1 %_53.i.i1025.6.i, label %bb21.i.i1027.6.i, label %panic1.i.i1026.i, !dbg !7442

bb21.i.i1027.6.i:                                 ; preds = %bb17.i.i1016.6.i
  %342 = getelementptr inbounds nuw float, ptr %_147.0.i.i1028.i, i64 %_50.i.i1023.6.i, !dbg !7442
  %_49.i.i1029.6.i = load float, ptr %342, align 4, !dbg !7442, !noalias !7425, !noundef !12
  store float %_49.i.i1029.6.i, ptr %iter.i.i916.sroa.0.0.ptr7705.6.i, align 4, !dbg !7443, !noalias !7444
  %343 = icmp eq i64 %width.i.i996.i, 7, !dbg !7264
  br i1 %343, label %bb16.i.i1030.loopexit.i, label %bb39.i.i1010.7.i, !dbg !7264

bb39.i.i1010.7.i:                                 ; preds = %bb21.i.i1027.6.i
  %exitcond9588.7.not.i = icmp eq i64 %_145.1.i.i1013.i, 7, !dbg !7270
  br i1 %exitcond9588.7.not.i, label %panic.i.i1015.i, label %bb17.i.i1016.7.i, !dbg !7270

bb17.i.i1016.7.i:                                 ; preds = %bb39.i.i1010.7.i
  %344 = getelementptr inbounds nuw i8, ptr %_145.0.i.i1017.i, i64 92, !dbg !7270
  %_44.i.i1018.7.i = load i32, ptr %344, align 4, !dbg !7270, !noalias !7425, !noundef !12
  %_43.i.i1019.7.i = zext i32 %_44.i.i1018.7.i to i64, !dbg !7270
  %345 = add i64 %ring_cursor.sroa.0.1.i9787708.i, %_43.i.i1019.7.i, !dbg !7438
  %_47.not.i.i1020.7.i = icmp ult i64 %345, %_60.i995.i, !dbg !7439
  %346 = select i1 %_47.not.i.i1020.7.i, i64 0, i64 %_60.i995.i, !dbg !7439
  %spec.select.i.i1021.7.i = sub nuw i64 %345, %346, !dbg !7439
  %_51.i.i1022.7.i = mul i64 %spec.select.i.i1021.7.i, %width.i.i996.i, !dbg !7441
  %_50.i.i1023.7.i = add i64 %_51.i.i1022.7.i, 7, !dbg !7441
  %_53.i.i1025.7.i = icmp ult i64 %_50.i.i1023.7.i, %_149.1.i.i1031.pre.i, !dbg !7442
  br i1 %_53.i.i1025.7.i, label %bb21.i.i1027.7.i, label %panic1.i.i1026.i, !dbg !7442

bb21.i.i1027.7.i:                                 ; preds = %bb17.i.i1016.7.i
  %347 = getelementptr inbounds nuw float, ptr %_147.0.i.i1028.i, i64 %_50.i.i1023.7.i, !dbg !7442
  %_49.i.i1029.7.i = load float, ptr %347, align 4, !dbg !7442, !noalias !7425, !noundef !12
  store float %_49.i.i1029.7.i, ptr %iter.i.i916.sroa.0.0.ptr7705.7.i, align 4, !dbg !7443, !noalias !7444
  br label %bb16.i.i1030.loopexit.i, !dbg !7264

panic1.i.i1026.i:                                 ; preds = %bb17.i.i1016.7.i, %bb17.i.i1016.6.i, %bb17.i.i1016.5.i, %bb17.i.i1016.4.i, %bb17.i.i1016.3.i, %bb17.i.i1016.2.i, %bb17.i.i1016.1.i, %bb17.i.i1016.i
  %_50.i.i1023.lcssa.ph.i = phi i64 [ %_50.i.i1023.7.i, %bb17.i.i1016.7.i ], [ %_50.i.i1023.6.i, %bb17.i.i1016.6.i ], [ %_50.i.i1023.5.i, %bb17.i.i1016.5.i ], [ %_50.i.i1023.4.i, %bb17.i.i1016.4.i ], [ %_50.i.i1023.3.i, %bb17.i.i1016.3.i ], [ %_50.i.i1023.2.i, %bb17.i.i1016.2.i ], [ %_50.i.i1023.1.i, %bb17.i.i1016.1.i ], [ %_51.i.i1022.i, %bb17.i.i1016.i ]
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i1023.lcssa.ph.i, i64 noundef %_149.1.i.i1031.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7d7f2b4ff3f37cf08b84adcf6762221c) #30, !dbg !7442, !noalias !7425
  unreachable, !dbg !7442

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3512.i: ; preds = %bb49.i.i1040.i
  %_150.0.i.i1041.i = load ptr, ptr %128, align 8, !dbg !7416, !alias.scope !7101, !noalias !7102, !nonnull !12, !noundef !12
  %_127.i.i1043.i = getelementptr inbounds nuw float, ptr %_150.0.i.i1041.i, i64 %_76.i.i1038.i, !dbg !7445
  %lanes.i3101.sroa.0.0.copyload.i = load <8 x float>, ptr %_127.i.i1043.i, align 4, !dbg !7450, !alias.scope !7454, !noalias !7458
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i1043.i, ptr noundef nonnull align 4 dereferenceable(32) %_100.i994.i, i64 32, i1 false), !dbg !7460
  %348 = fmul <8 x float> %308, %lanes.i3101.sroa.0.0.copyload.i, !dbg !7466
  %349 = select <8 x i1> %130, <8 x float> %lanes.i3101.sroa.0.0.copyload.i, <8 x float> %348, !dbg !7471
  store <8 x float> %349, ptr %_100.i994.i, align 4, !dbg !7476, !alias.scope !7481, !noalias !7485
  %350 = add i64 %main_cursor.sroa.0.1.i9797709.i, 1, !dbg !7489
  %_65.i1055.i = icmp eq i64 %350, %_67.i1054.i, !dbg !7490
  %spec.store.select.i1056.i = select i1 %_65.i1055.i, i64 0, i64 %350, !dbg !7490
  %351 = add i64 %ring_cursor.sroa.0.1.i9787708.i, 1, !dbg !7491
  %_68.i1057.i = icmp eq i64 %351, %_60.i995.i, !dbg !7492
  %spec.store.select9.i1058.i = select i1 %_68.i1057.i, i64 0, i64 %351, !dbg !7492
  %exitcond9593.not.i = icmp eq i64 %244, %umax9592.i, !dbg !7493
  br i1 %exitcond9593.not.i, label %bb14.i976.bb12.i962.loopexit_crit_edge.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3140.i, !dbg !6295

bb36.i1064.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3140.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i983.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_beae6f21d5ab5b7a10c8cf24995b1244) #30, !dbg !7496, !noalias !7497
  unreachable, !dbg !7496

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i: ; preds = %bb12.i962.loopexit.i
  store <8 x float> %history.i.i894.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.10.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i894.sroa.13.0.hot_left.i948.sroa_idx.i, align 1, !dbg !6347
  store <8 x float> %history.i.i894.sroa.0.0.lcssa.i, ptr %hot_left.i948.i, align 32, !dbg !6954, !noalias !6249
  %352 = trunc i64 %main_cursor.sroa.0.1.i979.lcssa.i to i32, !dbg !7498
  %353 = trunc i64 %ring_cursor.sroa.0.1.i978.lcssa.i to i32, !dbg !7499
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !7500

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, %bb8.i.i
  %ring_cursor.sroa.0.0.i965.lcssa.i = phi i32 [ %_26.i956.i, %bb8.i.i ], [ %353, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !6186
  %main_cursor.sroa.0.0.i966.lcssa.i = phi i32 [ %_24.i955.i, %bb8.i.i ], [ %352, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !6182
  call void @llvm.lifetime.start.p0(ptr nonnull %_71.i930.i), !dbg !7500, !noalias !6169
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(736) %_71.i930.i, ptr noundef nonnull align 32 dereferenceable(736) %hot_left.i948.i, i64 736, i1 false), !dbg !7500, !noalias !6169
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %_71.i930.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25) #31, !dbg !7501, !noalias !7497
  call void @llvm.lifetime.end.p0(ptr nonnull %_71.i930.i), !dbg !7502, !noalias !6169
  store i32 %main_cursor.sroa.0.0.i966.lcssa.i, ptr %_25.i, align 4, !dbg !7498, !alias.scope !6184, !noalias !6185
  store i32 %ring_cursor.sroa.0.0.i965.lcssa.i, ptr %61, align 4, !dbg !7499, !alias.scope !6184, !noalias !6185
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i941.i), !dbg !7503, !noalias !6169
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i942.i), !dbg !7504, !noalias !6169
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i948.i), !dbg !7505, !noalias !6169
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !6160

bb4.i.i:                                          ; preds = %bb1.i3.i.i, %bb2.i.i, %bb2.i3779.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7506), !dbg !7509
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7510), !dbg !7509
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7512), !dbg !7509
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7514), !dbg !7509
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i256.i), !dbg !7516, !noalias !7520
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i256.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !7522, !noalias !7523
  %354 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !7524
  %355 = load i8, ptr %354, align 32, !dbg !7524, !range !17, !alias.scope !7528, !noalias !7529, !noundef !12
  %356 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !7530
  %357 = load i8, ptr %356, align 1, !dbg !7530, !range !17, !alias.scope !7528, !noalias !7529, !noundef !12
  %358 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !7532
  %ring.i263.i = load i64, ptr %358, align 8, !dbg !7532, !alias.scope !7534, !noalias !7535, !noundef !12
  %359 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !7536
  %main.i264.i = load i64, ptr %359, align 8, !dbg !7536, !alias.scope !7534, !noalias !7535, !noundef !12
  %_25.i265.i = load i32, ptr %_25.i, align 4, !dbg !7538, !alias.scope !7540, !noalias !7541, !noundef !12
  %360 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !7542
  %_26.i266.i = load i32, ptr %360, align 4, !dbg !7542, !alias.scope !7540, !noalias !7541, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i250.i), !dbg !7544, !noalias !7520
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i250.i, i8 0, i64 1024, i1 false), !noalias !7520
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i249.i), !dbg !7546, !noalias !7520
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i249.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, i64 %ring.i263.i, i64 %main.i264.i) #31, !dbg !7548, !noalias !5902
  %361 = add nuw nsw i64 %_31, 31, !dbg !7549
  %yield_count.sroa.0.0.i.i3865.i = lshr i64 %361, 5, !dbg !7549
  %_107.not.i2758269.i = icmp eq i64 %yield_count.sroa.0.0.i.i3865.i, 0, !dbg !7556
  br i1 %_107.not.i2758269.i, label %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge, label %bb36.i276.lr.ph.i, !dbg !7556

bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge: ; preds = %bb4.i.i
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 104
  %left_phase.i420.i.pre = load i32, ptr %.phi.trans.insert, align 8, !dbg !7565, !noalias !7520
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !7556

bb36.i276.lr.ph.i:                                ; preds = %bb4.i.i
  %362 = zext i32 %_26.i266.i to i64, !dbg !7542
  %363 = zext i32 %_25.i265.i to i64, !dbg !7538
  %_22.i260.i = trunc nuw i8 %357 to i1, !dbg !7530
  %_21.i257.i = trunc nuw i8 %355 to i1, !dbg !7524
  %364 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !7566
  %365 = bitcast <8 x float> %364 to <8 x i32>, !dbg !7572
  %366 = xor <8 x i32> %365, splat (i32 -1), !dbg !7578
  %history.i.i225.sroa.10.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 32
  %history.i.i225.sroa.13.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 64
  %history.i.i225.sroa.16.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 96
  %history.i.i225.sroa.19.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 128
  %history.i.i225.sroa.22.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 160
  %history.i.i225.sroa.25.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 192
  %history.i.i225.sroa.29.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 224
  %history.i.i225.sroa.32.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 256
  %history.i.i225.sroa.35.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 288
  %history.i.i225.sroa.38.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 320
  %history.i.i225.sroa.41.0.hot_left.i256.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 352
  %367 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %368 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %369 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i396.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %370 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %371 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %372 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i397.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %373 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %374 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %375 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i398.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %376 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %377 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %378 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i399.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %379 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %380 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %381 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i400.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %382 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %383 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %384 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i401.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %385 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %386 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %387 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i402.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %388 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %389 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %390 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i403.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %391 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %392 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %393 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i404.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %394 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %395 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %396 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i405.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %397 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %398 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %399 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i406.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %400 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %401 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %402 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %403 = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 80
  %_48.i246.sroa.3.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 88
  %_48.i246.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 96
  %_78.i319.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 384
  %_79.i320.i = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 512
  %404 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 480
  %405 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 448
  %406 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 416
  %407 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 608
  %408 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 576
  %409 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 544
  %410 = select i1 %_21.i257.i, <8 x i32> %365, <8 x i32> %366
  %411 = icmp slt <8 x i32> %410, zeroinitializer
  %412 = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 32
  %413 = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 40
  %_22.i.i343.i = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 104
  %414 = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 48
  %415 = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 56
  %416 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 672
  %417 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 704
  %418 = getelementptr inbounds nuw i8, ptr %hot_left.i256.i, i64 640
  %419 = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 72
  %420 = getelementptr inbounds nuw i8, ptr %uniform_left.i249.i, i64 64
  %421 = select i1 %_22.i260.i, <8 x i32> %365, <8 x i32> %366
  %422 = icmp slt <8 x i32> %421, zeroinitializer
  %_48.i246.sroa.3.0.copyload.pre.i = load i64, ptr %_48.i246.sroa.3.0..sroa_idx.i, align 8, !noalias !6249
  %_48.i246.sroa.4.0.copyload.pre.i = load i64, ptr %_48.i246.sroa.4.0..sroa_idx.i, align 16, !noalias !6249
  %_54.0.i.i331.pre.i = load ptr, ptr %412, align 32, !noalias !6249
  %_54.1.i.i332.pre.i = load i64, ptr %413, align 8, !noalias !6249
  %_18.i22.i.i = load i64, ptr %403, align 16, !noalias !6249
  %_56.0.i.i349.i = load ptr, ptr %414, align 16, !noalias !6249, !nonnull !12, !align !24
  %_56.1.i.i350.i = load i64, ptr %415, align 8, !noalias !6249
  %_58.1.i.i362.i = load i64, ptr %419, align 8, !noalias !6249
  %_58.0.i.i361.i = load ptr, ptr %420, align 32, !noalias !6249, !nonnull !12, !align !24
  %uniform_left.i249.promoted.i = load <8 x float>, ptr %uniform_left.i249.i, align 1, !noalias !6249
  %_22.i.i343.promoted.i = load i32, ptr %_22.i.i343.i, align 4, !noalias !6249
  %history.i.i225.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %hot_left.i256.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.10.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.10.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.13.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.13.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.16.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.16.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.19.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.19.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.22.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.22.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.25.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.25.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.29.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.29.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.32.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.32.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.35.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.35.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.38.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.38.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  %history.i.i225.sroa.41.sroa.0.0.copyload.i.pre = load <8 x float>, ptr %history.i.i225.sroa.41.0.hot_left.i256.sroa_idx.i, align 32, !dbg !7580, !noalias !7584
  br label %bb36.i276.i, !dbg !7556

bb16.i284.bb13.i270.loopexit_crit_edge.i:         ; preds = %bb25.i382.i
  store <8 x float> %.lcssa22242847, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802857, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722866, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642876, ptr %408, align 1, !dbg !7615
  store <8 x float> %.lcssa1018310822.i, ptr %404, align 32, !noalias !6249
  store <8 x float> %.lcssa1017510833.i, ptr %_78.i319.i, align 32, !noalias !6249
  store <8 x float> %.lcssa1016710844.i, ptr %405, align 32, !noalias !6249
  br label %bb13.i270.loopexit.i, !dbg !7616

bb13.i270.loopexit.i:                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i, %bb16.i284.bb13.i270.loopexit_crit_edge.i
  %storemerge.i.i348.lcssa82258250.lcssa10877.i = phi i32 [ %storemerge.i.i348.lcssa82258250.i, %bb16.i284.bb13.i270.loopexit_crit_edge.i ], [ %storemerge.i.i348.lcssa82258250.lcssa10878.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i ]
  %minimum.i.i36.sroa.0.08107.lcssa8231.lcssa.i = phi <8 x float> [ %minimum.i.i36.sroa.0.08107.lcssa.i, %bb16.i284.bb13.i270.loopexit_crit_edge.i ], [ %minimum.i.i36.sroa.0.08107.lcssa8231.lcssa10866.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i ]
  %ring_cursor.sroa.0.1.i285.lcssa.i = phi i64 [ %ring_cursor.sroa.0.2.i385.i, %bb16.i284.bb13.i270.loopexit_crit_edge.i ], [ %ring_cursor.sroa.0.0.i2718270.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i ], !dbg !7617
  %main_cursor.sroa.0.1.i286.lcssa.i = phi i64 [ %main_cursor.sroa.0.2.i388.i, %bb16.i284.bb13.i270.loopexit_crit_edge.i ], [ %main_cursor.sroa.0.0.i2728271.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i ], !dbg !7618
  %_107.not.i275.i = icmp eq i64 %424, 0, !dbg !7556
  %indvars.iv.next9618.i = add nsw i64 %indvars.iv9617.i, -32, !dbg !7556
  br i1 %_107.not.i275.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, label %bb36.i276.i, !dbg !7556

bb36.i276.i:                                      ; preds = %bb13.i270.loopexit.i, %bb36.i276.lr.ph.i
  %history.i.i225.sroa.41.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.41.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.41.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.38.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.38.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.38.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.35.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.35.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.35.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.32.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.32.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.32.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.29.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.29.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.29.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.25.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.25.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.25.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.22.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.22.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.22.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.19.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.19.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.19.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.16.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.16.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.16.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.13.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.13.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.13.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.10.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.10.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.10.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %history.i.i225.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i225.sroa.0.0.copyload.i.pre, %bb36.i276.lr.ph.i ], [ %history.i.i225.sroa.0.0.lcssa.i, %bb13.i270.loopexit.i ], !dbg !7580
  %storemerge.i.i348.lcssa82258250.lcssa10878.i = phi i32 [ %_22.i.i343.promoted.i, %bb36.i276.lr.ph.i ], [ %storemerge.i.i348.lcssa82258250.lcssa10877.i, %bb13.i270.loopexit.i ]
  %minimum.i.i36.sroa.0.08107.lcssa8231.lcssa10866.i = phi <8 x float> [ %uniform_left.i249.promoted.i, %bb36.i276.lr.ph.i ], [ %minimum.i.i36.sroa.0.08107.lcssa8231.lcssa.i, %bb13.i270.loopexit.i ]
  %indvars.iv9617.i = phi i64 [ %_31, %bb36.i276.lr.ph.i ], [ %indvars.iv.next9618.i, %bb13.i270.loopexit.i ]
  %iter3.sroa.0.0.i2748273.i = phi i64 [ %yield_count.sroa.0.0.i.i3865.i, %bb36.i276.lr.ph.i ], [ %424, %bb13.i270.loopexit.i ]
  %iter2.sroa.0.0.i2738272.i = phi i64 [ 0, %bb36.i276.lr.ph.i ], [ %423, %bb13.i270.loopexit.i ]
  %main_cursor.sroa.0.0.i2728271.i = phi i64 [ %363, %bb36.i276.lr.ph.i ], [ %main_cursor.sroa.0.1.i286.lcssa.i, %bb13.i270.loopexit.i ]
  %ring_cursor.sroa.0.0.i2718270.i = phi i64 [ %362, %bb36.i276.lr.ph.i ], [ %ring_cursor.sroa.0.1.i285.lcssa.i, %bb13.i270.loopexit.i ]
  %umin9634.i = tail call i64 @llvm.umin.i64(i64 %indvars.iv9617.i, i64 32), !dbg !7619
  %umax9620.i = tail call i64 @llvm.umax.i64(i64 %umin9634.i, i64 1), !dbg !7619
  %423 = add nuw nsw i64 %iter2.sroa.0.0.i2738272.i, 32, !dbg !7619
  %424 = add nsw i64 %iter3.sroa.0.0.i2748273.i, -1, !dbg !7623
  %_34.i278.i = sub nsw i64 %_31, %iter2.sroa.0.0.i2738272.i, !dbg !7624
  %..i3866.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i278.i, i64 32), !dbg !7625
  %_20.i.i2828078.not.i = icmp eq i64 %iter2.sroa.0.0.i2738272.i, %_31, !dbg !7629
  br i1 %_20.i.i2828078.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i, label %bb5.i.i390.lr.ph.i, !dbg !7633

bb5.i.i390.lr.ph.i:                               ; preds = %bb36.i276.i
  %_5.i2775.i = load <8 x float>, ptr %self, align 32, !alias.scope !5901, !noalias !5902
  %_14.i.i.i.i190.sroa.0.0.copyload.i = load <8 x float>, ptr %367, align 32, !alias.scope !5901, !noalias !5902
  %_17.i.i.i.i187.sroa.0.0.copyload.i = load <8 x float>, ptr %368, align 32, !alias.scope !5901, !noalias !5902
  %_20.i.i.i.i184.sroa.0.0.copyload.i = load <8 x float>, ptr %369, align 32, !alias.scope !5901, !noalias !5902
  %_25.i.i.i.i180.sroa.0.0.copyload.i = load <8 x float>, ptr %row12.i.i.i.i396.i, align 32, !alias.scope !5901, !noalias !5902
  %_28.i.i.i.i177.sroa.0.0.copyload.i = load <8 x float>, ptr %370, align 32, !alias.scope !5901, !noalias !5902
  %_31.i.i.i.i174.sroa.0.0.copyload.i = load <8 x float>, ptr %371, align 32, !alias.scope !5901, !noalias !5902
  %_34.i.i.i.i171.sroa.0.0.copyload.i = load <8 x float>, ptr %372, align 32, !alias.scope !5901, !noalias !5902
  %_39.i.i.i.i167.sroa.0.0.copyload.i = load <8 x float>, ptr %row13.i.i.i.i397.i, align 32, !alias.scope !5901, !noalias !5902
  %_42.i.i.i.i164.sroa.0.0.copyload.i = load <8 x float>, ptr %373, align 32, !alias.scope !5901, !noalias !5902
  %_45.i.i.i.i161.sroa.0.0.copyload.i = load <8 x float>, ptr %374, align 32, !alias.scope !5901, !noalias !5902
  %_48.i.i.i.i158.sroa.0.0.copyload.i = load <8 x float>, ptr %375, align 32, !alias.scope !5901, !noalias !5902
  %_53.i.i.i.i154.sroa.0.0.copyload.i = load <8 x float>, ptr %row14.i.i.i.i398.i, align 32, !alias.scope !5901, !noalias !5902
  %_56.i.i.i.i151.sroa.0.0.copyload.i = load <8 x float>, ptr %376, align 32, !alias.scope !5901, !noalias !5902
  %_59.i.i.i.i148.sroa.0.0.copyload.i = load <8 x float>, ptr %377, align 32, !alias.scope !5901, !noalias !5902
  %_62.i.i.i.i145.sroa.0.0.copyload.i = load <8 x float>, ptr %378, align 32, !alias.scope !5901, !noalias !5902
  %_67.i.i.i.i141.sroa.0.0.copyload.i = load <8 x float>, ptr %row15.i.i.i.i399.i, align 32, !alias.scope !5901, !noalias !5902
  %_70.i.i.i.i138.sroa.0.0.copyload.i = load <8 x float>, ptr %379, align 32, !alias.scope !5901, !noalias !5902
  %_73.i.i.i.i135.sroa.0.0.copyload.i = load <8 x float>, ptr %380, align 32, !alias.scope !5901, !noalias !5902
  %_76.i.i.i.i132.sroa.0.0.copyload.i = load <8 x float>, ptr %381, align 32, !alias.scope !5901, !noalias !5902
  %_81.i.i.i.i128.sroa.0.0.copyload.i = load <8 x float>, ptr %row16.i.i.i.i400.i, align 32, !alias.scope !5901, !noalias !5902
  %_84.i.i.i.i125.sroa.0.0.copyload.i = load <8 x float>, ptr %382, align 32, !alias.scope !5901, !noalias !5902
  %_87.i.i.i.i122.sroa.0.0.copyload.i = load <8 x float>, ptr %383, align 32, !alias.scope !5901, !noalias !5902
  %_90.i.i.i.i119.sroa.0.0.copyload.i = load <8 x float>, ptr %384, align 32, !alias.scope !5901, !noalias !5902
  %_95.i.i.i.i115.sroa.0.0.copyload.i = load <8 x float>, ptr %row17.i.i.i.i401.i, align 32, !alias.scope !5901, !noalias !5902
  %_98.i.i.i.i112.sroa.0.0.copyload.i = load <8 x float>, ptr %385, align 32, !alias.scope !5901, !noalias !5902
  %_101.i.i.i.i109.sroa.0.0.copyload.i = load <8 x float>, ptr %386, align 32, !alias.scope !5901, !noalias !5902
  %_104.i.i.i.i106.sroa.0.0.copyload.i = load <8 x float>, ptr %387, align 32, !alias.scope !5901, !noalias !5902
  %_109.i.i.i.i102.sroa.0.0.copyload.i = load <8 x float>, ptr %row18.i.i.i.i402.i, align 32, !alias.scope !5901, !noalias !5902
  %_112.i.i.i.i99.sroa.0.0.copyload.i = load <8 x float>, ptr %388, align 32, !alias.scope !5901, !noalias !5902
  %_115.i.i.i.i96.sroa.0.0.copyload.i = load <8 x float>, ptr %389, align 32, !alias.scope !5901, !noalias !5902
  %_118.i.i.i.i93.sroa.0.0.copyload.i = load <8 x float>, ptr %390, align 32, !alias.scope !5901, !noalias !5902
  %_123.i.i.i.i89.sroa.0.0.copyload.i = load <8 x float>, ptr %row19.i.i.i.i403.i, align 32, !alias.scope !5901, !noalias !5902
  %_126.i.i.i.i86.sroa.0.0.copyload.i = load <8 x float>, ptr %391, align 32, !alias.scope !5901, !noalias !5902
  %_129.i.i.i.i83.sroa.0.0.copyload.i = load <8 x float>, ptr %392, align 32, !alias.scope !5901, !noalias !5902
  %_132.i.i.i.i80.sroa.0.0.copyload.i = load <8 x float>, ptr %393, align 32, !alias.scope !5901, !noalias !5902
  %_137.i.i.i.i76.sroa.0.0.copyload.i = load <8 x float>, ptr %row20.i.i.i.i404.i, align 32, !alias.scope !5901, !noalias !5902
  %_140.i.i.i.i73.sroa.0.0.copyload.i = load <8 x float>, ptr %394, align 32, !alias.scope !5901, !noalias !5902
  %_143.i.i.i.i70.sroa.0.0.copyload.i = load <8 x float>, ptr %395, align 32, !alias.scope !5901, !noalias !5902
  %_146.i.i.i.i67.sroa.0.0.copyload.i = load <8 x float>, ptr %396, align 32, !alias.scope !5901, !noalias !5902
  %_151.i.i.i.i63.sroa.0.0.copyload.i = load <8 x float>, ptr %row21.i.i.i.i405.i, align 32, !alias.scope !5901, !noalias !5902
  %_154.i.i.i.i60.sroa.0.0.copyload.i = load <8 x float>, ptr %397, align 32, !alias.scope !5901, !noalias !5902
  %_157.i.i.i.i57.sroa.0.0.copyload.i = load <8 x float>, ptr %398, align 32, !alias.scope !5901, !noalias !5902
  %_160.i.i.i.i54.sroa.0.0.copyload.i = load <8 x float>, ptr %399, align 32, !alias.scope !5901, !noalias !5902
  %_165.i.i.i.i50.sroa.0.0.copyload.i = load <8 x float>, ptr %row22.i.i.i.i406.i, align 32, !alias.scope !5901, !noalias !5902
  %_168.i.i.i.i47.sroa.0.0.copyload.i = load <8 x float>, ptr %400, align 32, !alias.scope !5901, !noalias !5902
  %_171.i.i.i.i44.sroa.0.0.copyload.i = load <8 x float>, ptr %401, align 32, !alias.scope !5901, !noalias !5902
  %_174.i.i.i.i41.sroa.0.0.copyload.i = load <8 x float>, ptr %402, align 32, !alias.scope !5901, !noalias !5902
  br label %bb5.i.i390.i, !dbg !7633

bb5.i.i390.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i, %bb5.i.i390.lr.ph.i
  %iter.sroa.0.0.i.i2818090.i = phi i64 [ 0, %bb5.i.i390.lr.ph.i ], [ %425, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.10.sroa.0.08089.i = phi <8 x float> [ %history.i.i225.sroa.10.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.0.08079.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.13.sroa.0.08088.i = phi <8 x float> [ %history.i.i225.sroa.13.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.10.sroa.0.08089.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.16.sroa.0.08087.i = phi <8 x float> [ %history.i.i225.sroa.16.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.13.sroa.0.08088.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.19.sroa.0.08086.i = phi <8 x float> [ %history.i.i225.sroa.19.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.16.sroa.0.08087.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.22.sroa.0.08085.i = phi <8 x float> [ %history.i.i225.sroa.22.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.19.sroa.0.08086.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.38.sroa.0.08084.i = phi <8 x float> [ %history.i.i225.sroa.38.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.35.sroa.0.08083.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.35.sroa.0.08083.i = phi <8 x float> [ %history.i.i225.sroa.35.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.32.sroa.0.08082.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.32.sroa.0.08082.i = phi <8 x float> [ %history.i.i225.sroa.32.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.29.sroa.0.08081.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.29.sroa.0.08081.i = phi <8 x float> [ %history.i.i225.sroa.29.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.25.sroa.0.08080.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.25.sroa.0.08080.i = phi <8 x float> [ %history.i.i225.sroa.25.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %history.i.i225.sroa.22.sroa.0.08085.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %history.i.i225.sroa.0.08079.i = phi <8 x float> [ %history.i.i225.sroa.0.0.copyload.i, %bb5.i.i390.lr.ph.i ], [ %lanes.i3192.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ]
  %425 = add nuw nsw i64 %iter.sroa.0.0.i.i2818090.i, 1, !dbg !7634
  %_11.i17.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i2818090.i, %iter2.sroa.0.0.i2738272.i, !dbg !7637
  %base.i.i391.i = shl i64 %_11.i17.i.i, 3, !dbg !7637
  %_24.i.i392.i = icmp samesign ugt i64 %base.i.i391.i, %_39.1, !dbg !7638
  br i1 %_24.i.i392.i, label %bb7.i.i419.i, label %bb8.i.i393.i, !dbg !7638, !prof !639

bb8.i.i393.i:                                     ; preds = %bb5.i.i390.i
  %_27.i.i394.i = sub nuw nsw i64 %_39.1, %base.i.i391.i, !dbg !7641
  %_8.i3195.i = icmp samesign ugt i64 %_27.i.i394.i, 7, !dbg !7642
  br i1 %_8.i3195.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i, label %bb2.i3196.i, !dbg !7642, !prof !651

bb2.i3196.i:                                      ; preds = %bb8.i.i393.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i394.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !7647, !noalias !7648
  unreachable, !dbg !7647

bb7.i.i419.i:                                     ; preds = %bb5.i.i390.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i391.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !7653, !noalias !7654
  unreachable, !dbg !7653

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i: ; preds = %bb8.i.i393.i
  %_31.i.i395.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i.i391.i, !dbg !7655
  %lanes.i3192.sroa.0.0.copyload.i = load <8 x float>, ptr %_31.i.i395.i, align 4, !dbg !7657, !alias.scope !7661, !noalias !7665
  %426 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i225.sroa.22.sroa.0.08085.i), !dbg !7667
  %427 = fmul <8 x float> %_5.i2775.i, %lanes.i3192.sroa.0.0.copyload.i, !dbg !7674
  %428 = fadd <8 x float> %427, zeroinitializer, !dbg !7680
  %429 = fmul <8 x float> %_14.i.i.i.i190.sroa.0.0.copyload.i, %lanes.i3192.sroa.0.0.copyload.i, !dbg !7685
  %430 = fadd <8 x float> %429, zeroinitializer, !dbg !7690
  %431 = fmul <8 x float> %_17.i.i.i.i187.sroa.0.0.copyload.i, %lanes.i3192.sroa.0.0.copyload.i, !dbg !7695
  %432 = fadd <8 x float> %431, zeroinitializer, !dbg !7700
  %433 = fmul <8 x float> %_20.i.i.i.i184.sroa.0.0.copyload.i, %lanes.i3192.sroa.0.0.copyload.i, !dbg !7705
  %434 = fadd <8 x float> %433, zeroinitializer, !dbg !7710
  %435 = fmul <8 x float> %_25.i.i.i.i180.sroa.0.0.copyload.i, %history.i.i225.sroa.0.08079.i, !dbg !7715
  %436 = fadd <8 x float> %435, %428, !dbg !7720
  %437 = fmul <8 x float> %_28.i.i.i.i177.sroa.0.0.copyload.i, %history.i.i225.sroa.0.08079.i, !dbg !7725
  %438 = fadd <8 x float> %437, %430, !dbg !7730
  %439 = fmul <8 x float> %_31.i.i.i.i174.sroa.0.0.copyload.i, %history.i.i225.sroa.0.08079.i, !dbg !7735
  %440 = fadd <8 x float> %439, %432, !dbg !7740
  %441 = fmul <8 x float> %_34.i.i.i.i171.sroa.0.0.copyload.i, %history.i.i225.sroa.0.08079.i, !dbg !7745
  %442 = fadd <8 x float> %441, %434, !dbg !7750
  %443 = fmul <8 x float> %_39.i.i.i.i167.sroa.0.0.copyload.i, %history.i.i225.sroa.10.sroa.0.08089.i, !dbg !7755
  %444 = fadd <8 x float> %443, %436, !dbg !7760
  %445 = fmul <8 x float> %_42.i.i.i.i164.sroa.0.0.copyload.i, %history.i.i225.sroa.10.sroa.0.08089.i, !dbg !7765
  %446 = fadd <8 x float> %445, %438, !dbg !7770
  %447 = fmul <8 x float> %_45.i.i.i.i161.sroa.0.0.copyload.i, %history.i.i225.sroa.10.sroa.0.08089.i, !dbg !7775
  %448 = fadd <8 x float> %447, %440, !dbg !7780
  %449 = fmul <8 x float> %_48.i.i.i.i158.sroa.0.0.copyload.i, %history.i.i225.sroa.10.sroa.0.08089.i, !dbg !7785
  %450 = fadd <8 x float> %449, %442, !dbg !7790
  %451 = fmul <8 x float> %_53.i.i.i.i154.sroa.0.0.copyload.i, %history.i.i225.sroa.13.sroa.0.08088.i, !dbg !7795
  %452 = fadd <8 x float> %451, %444, !dbg !7800
  %453 = fmul <8 x float> %_56.i.i.i.i151.sroa.0.0.copyload.i, %history.i.i225.sroa.13.sroa.0.08088.i, !dbg !7805
  %454 = fadd <8 x float> %453, %446, !dbg !7810
  %455 = fmul <8 x float> %_59.i.i.i.i148.sroa.0.0.copyload.i, %history.i.i225.sroa.13.sroa.0.08088.i, !dbg !7815
  %456 = fadd <8 x float> %455, %448, !dbg !7820
  %457 = fmul <8 x float> %_62.i.i.i.i145.sroa.0.0.copyload.i, %history.i.i225.sroa.13.sroa.0.08088.i, !dbg !7825
  %458 = fadd <8 x float> %457, %450, !dbg !7830
  %459 = fmul <8 x float> %_67.i.i.i.i141.sroa.0.0.copyload.i, %history.i.i225.sroa.16.sroa.0.08087.i, !dbg !7835
  %460 = fadd <8 x float> %459, %452, !dbg !7840
  %461 = fmul <8 x float> %_70.i.i.i.i138.sroa.0.0.copyload.i, %history.i.i225.sroa.16.sroa.0.08087.i, !dbg !7845
  %462 = fadd <8 x float> %461, %454, !dbg !7850
  %463 = fmul <8 x float> %_73.i.i.i.i135.sroa.0.0.copyload.i, %history.i.i225.sroa.16.sroa.0.08087.i, !dbg !7855
  %464 = fadd <8 x float> %463, %456, !dbg !7860
  %465 = fmul <8 x float> %_76.i.i.i.i132.sroa.0.0.copyload.i, %history.i.i225.sroa.16.sroa.0.08087.i, !dbg !7865
  %466 = fadd <8 x float> %465, %458, !dbg !7870
  %467 = fmul <8 x float> %_81.i.i.i.i128.sroa.0.0.copyload.i, %history.i.i225.sroa.19.sroa.0.08086.i, !dbg !7875
  %468 = fadd <8 x float> %467, %460, !dbg !7880
  %469 = fmul <8 x float> %_84.i.i.i.i125.sroa.0.0.copyload.i, %history.i.i225.sroa.19.sroa.0.08086.i, !dbg !7885
  %470 = fadd <8 x float> %469, %462, !dbg !7890
  %471 = fmul <8 x float> %_87.i.i.i.i122.sroa.0.0.copyload.i, %history.i.i225.sroa.19.sroa.0.08086.i, !dbg !7895
  %472 = fadd <8 x float> %471, %464, !dbg !7900
  %473 = fmul <8 x float> %_90.i.i.i.i119.sroa.0.0.copyload.i, %history.i.i225.sroa.19.sroa.0.08086.i, !dbg !7905
  %474 = fadd <8 x float> %473, %466, !dbg !7910
  %475 = fmul <8 x float> %_95.i.i.i.i115.sroa.0.0.copyload.i, %history.i.i225.sroa.22.sroa.0.08085.i, !dbg !7915
  %476 = fadd <8 x float> %475, %468, !dbg !7920
  %477 = fmul <8 x float> %_98.i.i.i.i112.sroa.0.0.copyload.i, %history.i.i225.sroa.22.sroa.0.08085.i, !dbg !7925
  %478 = fadd <8 x float> %477, %470, !dbg !7930
  %479 = fmul <8 x float> %_101.i.i.i.i109.sroa.0.0.copyload.i, %history.i.i225.sroa.22.sroa.0.08085.i, !dbg !7935
  %480 = fadd <8 x float> %479, %472, !dbg !7940
  %481 = fmul <8 x float> %_104.i.i.i.i106.sroa.0.0.copyload.i, %history.i.i225.sroa.22.sroa.0.08085.i, !dbg !7945
  %482 = fadd <8 x float> %481, %474, !dbg !7950
  %483 = fmul <8 x float> %_109.i.i.i.i102.sroa.0.0.copyload.i, %history.i.i225.sroa.25.sroa.0.08080.i, !dbg !7955
  %484 = fadd <8 x float> %483, %476, !dbg !7960
  %485 = fmul <8 x float> %_112.i.i.i.i99.sroa.0.0.copyload.i, %history.i.i225.sroa.25.sroa.0.08080.i, !dbg !7965
  %486 = fadd <8 x float> %485, %478, !dbg !7970
  %487 = fmul <8 x float> %_115.i.i.i.i96.sroa.0.0.copyload.i, %history.i.i225.sroa.25.sroa.0.08080.i, !dbg !7975
  %488 = fadd <8 x float> %487, %480, !dbg !7980
  %489 = fmul <8 x float> %_118.i.i.i.i93.sroa.0.0.copyload.i, %history.i.i225.sroa.25.sroa.0.08080.i, !dbg !7985
  %490 = fadd <8 x float> %489, %482, !dbg !7990
  %491 = fmul <8 x float> %_123.i.i.i.i89.sroa.0.0.copyload.i, %history.i.i225.sroa.29.sroa.0.08081.i, !dbg !7995
  %492 = fadd <8 x float> %491, %484, !dbg !8000
  %493 = fmul <8 x float> %_126.i.i.i.i86.sroa.0.0.copyload.i, %history.i.i225.sroa.29.sroa.0.08081.i, !dbg !8005
  %494 = fadd <8 x float> %493, %486, !dbg !8010
  %495 = fmul <8 x float> %_129.i.i.i.i83.sroa.0.0.copyload.i, %history.i.i225.sroa.29.sroa.0.08081.i, !dbg !8015
  %496 = fadd <8 x float> %495, %488, !dbg !8020
  %497 = fmul <8 x float> %_132.i.i.i.i80.sroa.0.0.copyload.i, %history.i.i225.sroa.29.sroa.0.08081.i, !dbg !8025
  %498 = fadd <8 x float> %497, %490, !dbg !8030
  %499 = fmul <8 x float> %_137.i.i.i.i76.sroa.0.0.copyload.i, %history.i.i225.sroa.32.sroa.0.08082.i, !dbg !8035
  %500 = fadd <8 x float> %499, %492, !dbg !8040
  %501 = fmul <8 x float> %_140.i.i.i.i73.sroa.0.0.copyload.i, %history.i.i225.sroa.32.sroa.0.08082.i, !dbg !8045
  %502 = fadd <8 x float> %501, %494, !dbg !8050
  %503 = fmul <8 x float> %_143.i.i.i.i70.sroa.0.0.copyload.i, %history.i.i225.sroa.32.sroa.0.08082.i, !dbg !8055
  %504 = fadd <8 x float> %503, %496, !dbg !8060
  %505 = fmul <8 x float> %_146.i.i.i.i67.sroa.0.0.copyload.i, %history.i.i225.sroa.32.sroa.0.08082.i, !dbg !8065
  %506 = fadd <8 x float> %505, %498, !dbg !8070
  %507 = fmul <8 x float> %_151.i.i.i.i63.sroa.0.0.copyload.i, %history.i.i225.sroa.35.sroa.0.08083.i, !dbg !8075
  %508 = fadd <8 x float> %507, %500, !dbg !8080
  %509 = fmul <8 x float> %_154.i.i.i.i60.sroa.0.0.copyload.i, %history.i.i225.sroa.35.sroa.0.08083.i, !dbg !8085
  %510 = fadd <8 x float> %509, %502, !dbg !8090
  %511 = fmul <8 x float> %_157.i.i.i.i57.sroa.0.0.copyload.i, %history.i.i225.sroa.35.sroa.0.08083.i, !dbg !8095
  %512 = fadd <8 x float> %511, %504, !dbg !8100
  %513 = fmul <8 x float> %_160.i.i.i.i54.sroa.0.0.copyload.i, %history.i.i225.sroa.35.sroa.0.08083.i, !dbg !8105
  %514 = fadd <8 x float> %513, %506, !dbg !8110
  %515 = fmul <8 x float> %_165.i.i.i.i50.sroa.0.0.copyload.i, %history.i.i225.sroa.38.sroa.0.08084.i, !dbg !8115
  %516 = fadd <8 x float> %515, %508, !dbg !8120
  %517 = fmul <8 x float> %_168.i.i.i.i47.sroa.0.0.copyload.i, %history.i.i225.sroa.38.sroa.0.08084.i, !dbg !8125
  %518 = fadd <8 x float> %517, %510, !dbg !8130
  %519 = fmul <8 x float> %_171.i.i.i.i44.sroa.0.0.copyload.i, %history.i.i225.sroa.38.sroa.0.08084.i, !dbg !8135
  %520 = fadd <8 x float> %519, %512, !dbg !8140
  %521 = fmul <8 x float> %_174.i.i.i.i41.sroa.0.0.copyload.i, %history.i.i225.sroa.38.sroa.0.08084.i, !dbg !8145
  %522 = fadd <8 x float> %521, %514, !dbg !8150
  %523 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %516), !dbg !8155
  %524 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %426, <8 x float> %523), !dbg !8161
  %525 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %518), !dbg !8155
  %526 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %524, <8 x float> %525), !dbg !8161
  %527 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %520), !dbg !8155
  %528 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %526, <8 x float> %527), !dbg !8161
  %529 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %522), !dbg !8155
  %530 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %528, <8 x float> %529), !dbg !8161
  %_39.i.i416.idx.i = shl i64 %iter.sroa.0.0.i.i2818090.i, 5, !dbg !8166
  %_39.i.i416.i = getelementptr inbounds nuw i8, ptr %peaks_left.i250.i, i64 %_39.i.i416.idx.i, !dbg !8166
  store <8 x float> %530, ptr %_39.i.i416.i, align 4, !dbg !8171, !alias.scope !8176, !noalias !8180
  %exitcond9621.not.i = icmp eq i64 %425, %umax9620.i, !dbg !7629
  br i1 %exitcond9621.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i, label %bb5.i.i390.i, !dbg !7633

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i, %bb36.i276.i
  %history.i.i225.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.0.0.copyload.i, %bb36.i276.i ], [ %lanes.i3192.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.25.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.25.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.22.sroa.0.08085.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.29.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.29.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.25.sroa.0.08080.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.32.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.32.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.29.sroa.0.08081.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.35.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.35.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.32.sroa.0.08082.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.38.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.38.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.35.sroa.0.08083.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.41.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.41.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.38.sroa.0.08084.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.22.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.22.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.19.sroa.0.08086.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.19.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.19.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.16.sroa.0.08087.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.16.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.16.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.13.sroa.0.08088.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.13.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.13.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.10.sroa.0.08089.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  %history.i.i225.sroa.10.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i225.sroa.10.sroa.0.0.copyload.i, %bb36.i276.i ], [ %history.i.i225.sroa.0.08079.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3562.i ], !dbg !8184
  store <8 x float> %history.i.i225.sroa.0.0.lcssa.i, ptr %hot_left.i256.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.10.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.13.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.16.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.16.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.19.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.22.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.25.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.29.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.29.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.32.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.32.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.35.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.35.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.38.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.38.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  store <8 x float> %history.i.i225.sroa.41.sroa.0.0.lcssa.i, ptr %history.i.i225.sroa.41.0.hot_left.i256.sroa_idx.i, align 32, !dbg !8185, !noalias !7584
  br i1 %_20.i.i2828078.not.i, label %bb13.i270.loopexit.i, label %bb17.i289.lr.ph.i, !dbg !7616

bb17.i289.lr.ph.i:                                ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i283.i
  %_13.i1450.sroa.0.0.copyload.i = load <8 x float>, ptr %406, align 32, !noalias !6249
  %_13.i1436.sroa.0.0.copyload.i = load <8 x float>, ptr %409, align 32, !noalias !6249
  %_37.i.i28.sroa.0.0.copyload.i = load <8 x float>, ptr %417, align 32, !noalias !6249
  %.promoted10821.i = load <8 x float>, ptr %404, align 32, !noalias !6249
  %_78.i319.promoted10832.i = load <8 x float>, ptr %_78.i319.i, align 32, !noalias !6249
  %.promoted10843.i = load <8 x float>, ptr %405, align 32, !noalias !6249
  %.promoted10854.i = load <8 x float>, ptr %407, align 32, !noalias !6249
  %_79.i320.promoted10857.i = load <8 x float>, ptr %_79.i320.i, align 32, !noalias !6249
  %.promoted10860.i = load <8 x float>, ptr %408, align 32, !noalias !6249
  %.promoted10863.i = load <8 x float>, ptr %416, align 32, !noalias !6249
  br label %bb17.i289.i, !dbg !7616

bb17.i289.i:                                      ; preds = %bb25.i382.i, %bb17.i289.lr.ph.i
  %.lcssa21642877 = phi <8 x float> [ %.promoted10860.i, %bb17.i289.lr.ph.i ], [ %.lcssa21642876, %bb25.i382.i ]
  %.lcssa21722867 = phi <8 x float> [ %_79.i320.promoted10857.i, %bb17.i289.lr.ph.i ], [ %.lcssa21722866, %bb25.i382.i ]
  %.lcssa21802858 = phi <8 x float> [ %.promoted10854.i, %bb17.i289.lr.ph.i ], [ %.lcssa21802857, %bb25.i382.i ]
  %.lcssa22242848 = phi <8 x float> [ %.promoted10863.i, %bb17.i289.lr.ph.i ], [ %.lcssa22242847, %bb25.i382.i ]
  %.lcssa1082010865.i = phi <8 x float> [ %.promoted10863.i, %bb17.i289.lr.ph.i ], [ %.lcssa1082010864.i, %bb25.i382.i ]
  %.lcssa1014310862.i = phi <8 x float> [ %.promoted10860.i, %bb17.i289.lr.ph.i ], [ %.lcssa1014310861.i, %bb25.i382.i ]
  %.lcssa1015110859.i = phi <8 x float> [ %_79.i320.promoted10857.i, %bb17.i289.lr.ph.i ], [ %.lcssa1015110858.i, %bb25.i382.i ]
  %.lcssa1015910856.i = phi <8 x float> [ %.promoted10854.i, %bb17.i289.lr.ph.i ], [ %.lcssa1015910855.i, %bb25.i382.i ]
  %.lcssa1016710845.i = phi <8 x float> [ %.promoted10843.i, %bb17.i289.lr.ph.i ], [ %.lcssa1016710844.i, %bb25.i382.i ]
  %.lcssa1017510834.i = phi <8 x float> [ %_78.i319.promoted10832.i, %bb17.i289.lr.ph.i ], [ %.lcssa1017510833.i, %bb25.i382.i ]
  %.lcssa1018310823.i = phi <8 x float> [ %.promoted10821.i, %bb17.i289.lr.ph.i ], [ %.lcssa1018310822.i, %bb25.i382.i ]
  %storemerge.i.i348.lcssa82258251.i = phi i32 [ %storemerge.i.i348.lcssa82258250.lcssa10878.i, %bb17.i289.lr.ph.i ], [ %storemerge.i.i348.lcssa82258250.i, %bb25.i382.i ]
  %frame.sroa.0.0.i2878245.i = phi i64 [ 0, %bb17.i289.lr.ph.i ], [ %_62.i302.i, %bb25.i382.i ]
  %main_cursor.sroa.0.1.i2868244.i = phi i64 [ %main_cursor.sroa.0.0.i2728271.i, %bb17.i289.lr.ph.i ], [ %main_cursor.sroa.0.2.i388.i, %bb25.i382.i ]
  %ring_cursor.sroa.0.1.i2858243.i = phi i64 [ %ring_cursor.sroa.0.0.i2718270.i, %bb17.i289.lr.ph.i ], [ %ring_cursor.sroa.0.2.i385.i, %bb25.i382.i ]
  %minimum.i.i36.sroa.0.08107.lcssa82318242.i = phi <8 x float> [ %minimum.i.i36.sroa.0.08107.lcssa8231.lcssa10866.i, %bb17.i289.lr.ph.i ], [ %minimum.i.i36.sroa.0.08107.lcssa.i, %bb25.i382.i ]
  %_46.i290.i = sub nuw nsw i64 %..i3866.i, %frame.sroa.0.0.i2878245.i, !dbg !8186
  %ring.i1348.i = load i64, ptr %358, align 8, !dbg !8187, !alias.scope !8189, !noalias !8192, !noundef !12
  %main.i1349.i = load i64, ptr %359, align 8, !dbg !8196, !alias.scope !8189, !noalias !8192, !noundef !12
  %_10.i.i = add i64 %ring_cursor.sroa.0.1.i2858243.i, 1, !dbg !8197
  %_45.not.i.i = icmp ult i64 %_10.i.i, %ring.i1348.i, !dbg !8198
  %531 = select i1 %_45.not.i.i, i64 0, i64 %ring.i1348.i, !dbg !8198
  %start1.sroa.0.0.i1350.i = sub nuw i64 %_10.i.i, %531, !dbg !8198
  %_12.i1351.i = add i64 %ring_cursor.sroa.0.1.i2858243.i, %_48.i246.sroa.3.0.copyload.pre.i, !dbg !8200
  %_46.not.i.i = icmp ult i64 %_12.i1351.i, %ring.i1348.i, !dbg !8201
  %532 = select i1 %_46.not.i.i, i64 0, i64 %ring.i1348.i, !dbg !8201
  %left_end.sroa.0.0.i.i = sub nuw i64 %_12.i1351.i, %532, !dbg !8201
  %_18.i1355.i = add i64 %ring_cursor.sroa.0.1.i2858243.i, %_48.i246.sroa.4.0.copyload.pre.i, !dbg !8203
  %_48.not.i.i = icmp ult i64 %_18.i1355.i, %ring.i1348.i, !dbg !8204
  %533 = select i1 %_48.not.i.i, i64 0, i64 %ring.i1348.i, !dbg !8204
  %left_expiring.sroa.0.0.i.i = sub nuw i64 %_18.i1355.i, %533, !dbg !8204
  %_30.i1358.i = sub i64 %ring.i1348.i, %ring_cursor.sroa.0.1.i2858243.i, !dbg !8206
  %..i3879.i = tail call noundef i64 @llvm.umin.i64(i64 %_30.i1358.i, i64 %_46.i290.i), !dbg !8207
  %_31.i.i = sub i64 %main.i1349.i, %main_cursor.sroa.0.1.i2868244.i, !dbg !8209
  %..i3880.i = tail call noundef i64 @llvm.umin.i64(i64 %_31.i.i, i64 %..i3879.i), !dbg !8210
  %_32.i1361.i = sub i64 %ring.i1348.i, %start1.sroa.0.0.i1350.i, !dbg !8212
  %..i3881.i = tail call noundef i64 @llvm.umin.i64(i64 %_32.i1361.i, i64 %..i3880.i), !dbg !8213
  %_34.i1363.i = sub i64 %ring.i1348.i, %left_end.sroa.0.0.i.i, !dbg !8215
  %..i3882.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1363.i, i64 %..i3881.i), !dbg !8216
  %_38.i.i = sub i64 %ring.i1348.i, %left_expiring.sroa.0.0.i.i, !dbg !8218
  %..i3884.i = tail call noundef i64 @llvm.umin.i64(i64 %_38.i.i, i64 %..i3882.i), !dbg !8219
  %_52.i292.i = add i64 %frame.sroa.0.0.i2878245.i, %iter2.sroa.0.0.i2738272.i, !dbg !8221
  %base.i293.i = shl i64 %_52.i292.i, 3, !dbg !8221
  %base.i2936803.i = add i64 %..i3884.i, %_52.i292.i, !dbg !8222
  %_56.i295.i = shl i64 %base.i2936803.i, 3, !dbg !8222
  %_119.i296.i = icmp ult i64 %_56.i295.i, %base.i293.i, !dbg !8223
  %_113.not.i297.i = icmp ugt i64 %_56.i295.i, %_39.1
  %or.cond.i298.i = or i1 %_119.i296.i, %_113.not.i297.i, !dbg !8223
  br i1 %or.cond.i298.i, label %bb41.i389.i, label %bb39.i299.i, !dbg !8223, !prof !165

bb39.i299.i:                                      ; preds = %bb17.i289.i
  %_122.i300.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i293.i, !dbg !8230
  %_62.i302.i = add nuw nsw i64 %..i3884.i, %frame.sroa.0.0.i2878245.i, !dbg !8234
  %_131.i310.i.idx = shl nuw nsw i64 %frame.sroa.0.0.i2878245.i, 5, !dbg !8235
  %_131.i310.i = getelementptr inbounds nuw i8, ptr %peaks_left.i250.i, i64 %_131.i310.i.idx, !dbg !8235
  %_2.i.i.i8114.not.i = icmp eq i64 %..i3884.i, 0, !dbg !8245
  br i1 %_2.i.i.i8114.not.i, label %bb25.i382.i, label %bb24.i314.lr.ph.i, !dbg !8245

bb41.i389.i:                                      ; preds = %bb17.i289.i
  store <8 x float> %.lcssa22242848, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802858, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722867, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642877, ptr %408, align 1, !dbg !7615
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i293.i, i64 noundef %_56.i295.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2cc46ad3620fe9ec5bf0f181ee950489) #30, !dbg !8249, !noalias !8250
  unreachable, !dbg !8249

bb24.i314.lr.ph.i:                                ; preds = %bb39.i299.i
  %umin9630.i = tail call i64 @llvm.umin.i64(i64 %_34.i1363.i, i64 %_38.i.i), !dbg !8245
  %umin9631.i = tail call i64 @llvm.umin.i64(i64 %umin9630.i, i64 %_32.i1361.i), !dbg !8245
  %umin9632.i = tail call i64 @llvm.umin.i64(i64 %umin9631.i, i64 %_30.i1358.i), !dbg !8245
  %umin9633.i = tail call i64 @llvm.umin.i64(i64 %umin9632.i, i64 %_31.i.i), !dbg !8245
  %534 = sub nsw i64 %umin9634.i, %frame.sroa.0.0.i2878245.i, !dbg !8245
  %umin9635.i = tail call i64 @llvm.umin.i64(i64 %umin9633.i, i64 %534), !dbg !8245
  %535 = and i64 %umin9635.i, 2305843009213693951, !dbg !8245
  br label %bb24.i314.i, !dbg !8245

bb24.i314.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i, %bb24.i314.lr.ph.i
  %536 = phi <8 x float> [ %.lcssa1082010865.i, %bb24.i314.lr.ph.i ], [ %578, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %storemerge.i.i3488215.i = phi i32 [ %storemerge.i.i348.lcssa82258251.i, %bb24.i314.lr.ph.i ], [ %storemerge.i.i348.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %_12.i1437.sroa.0.0.copyload8199.i = phi <8 x float> [ %.lcssa1014310862.i, %bb24.i314.lr.ph.i ], [ %554, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %_11.i1438.sroa.0.0.copyload8183.i = phi <8 x float> [ %.lcssa1015110859.i, %bb24.i314.lr.ph.i ], [ %553, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %537 = phi <8 x float> [ %.lcssa1015910856.i, %bb24.i314.lr.ph.i ], [ %548, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %_12.i1451.sroa.0.0.copyload8151.i = phi <8 x float> [ %.lcssa1016710845.i, %bb24.i314.lr.ph.i ], [ %546, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %_11.i1452.sroa.0.0.copyload8135.i = phi <8 x float> [ %.lcssa1017510834.i, %bb24.i314.lr.ph.i ], [ %545, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %538 = phi <8 x float> [ %.lcssa1018310823.i, %bb24.i314.lr.ph.i ], [ %540, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %iter.i240.sroa.21.08117.i = phi i64 [ 0, %bb24.i314.lr.ph.i ], [ %_9.0.i3899.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %minimum.i.i36.sroa.0.081078115.i = phi <8 x float> [ %minimum.i.i36.sroa.0.08107.lcssa82318242.i, %bb24.i314.lr.ph.i ], [ %minimum.i.i36.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %start1.i.i.i.i.i = shl i64 %iter.i240.sroa.21.08117.i, 3, !dbg !8251
  %data.i.i.i.i3896.i = getelementptr inbounds nuw float, ptr %_122.i300.i, i64 %start1.i.i.i.i.i, !dbg !8253
  %_9.0.i3899.i = add nuw nsw i64 %iter.i240.sroa.21.08117.i, 1, !dbg !8255
  %539 = fadd <8 x float> %538, splat (float -1.000000e+00), !dbg !8256
  %540 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %539, <8 x float> zeroinitializer), !dbg !8262
  %541 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %540, <8 x float> zeroinitializer, i8 30), !dbg !8267
  %542 = fadd <8 x float> %_12.i1451.sroa.0.0.copyload8151.i, %_11.i1452.sroa.0.0.copyload8135.i, !dbg !8273
  %543 = bitcast <8 x float> %541 to <8 x i32>, !dbg !8278
  %544 = icmp slt <8 x i32> %543, zeroinitializer, !dbg !8282
  %545 = select <8 x i1> %544, <8 x float> %542, <8 x float> %_13.i1450.sroa.0.0.copyload.i, !dbg !8282
  %546 = select <8 x i1> %544, <8 x float> %_12.i1451.sroa.0.0.copyload8151.i, <8 x float> zeroinitializer, !dbg !8284
  %547 = fadd <8 x float> %537, splat (float -1.000000e+00), !dbg !8289
  %548 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %547, <8 x float> zeroinitializer), !dbg !8294
  %549 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %548, <8 x float> zeroinitializer, i8 30), !dbg !8299
  %550 = fadd <8 x float> %_12.i1437.sroa.0.0.copyload8199.i, %_11.i1438.sroa.0.0.copyload8183.i, !dbg !8305
  %551 = bitcast <8 x float> %549 to <8 x i32>, !dbg !8310
  %552 = icmp slt <8 x i32> %551, zeroinitializer, !dbg !8314
  %553 = select <8 x i1> %552, <8 x float> %550, <8 x float> %_13.i1436.sroa.0.0.copyload.i, !dbg !8314
  %554 = select <8 x i1> %552, <8 x float> %_12.i1437.sroa.0.0.copyload8199.i, <8 x float> zeroinitializer, !dbg !8316
  %_133.i326.i = add i64 %iter.i240.sroa.21.08117.i, %ring_cursor.sroa.0.1.i2858243.i, !dbg !8321
  %_134.i327.i = add i64 %iter.i240.sroa.21.08117.i, %main_cursor.sroa.0.1.i2868244.i, !dbg !8324
  %_135.i328.i = add i64 %iter.i240.sroa.21.08117.i, %left_end.sroa.0.0.i.i, !dbg !8325
  %_136.i329.i = add i64 %iter.i240.sroa.21.08117.i, %start1.sroa.0.0.i1350.i, !dbg !8326
  %_137.i330.i = add i64 %iter.i240.sroa.21.08117.i, %left_expiring.sroa.0.0.i.i, !dbg !8327
  %base.i9.i.i334.i = shl i64 %_133.i326.i, 3, !dbg !8328
  %_7.i10.i.i335.i = add i64 %base.i9.i.i334.i, 8, !dbg !8331
  %555 = or disjoint i64 %base.i9.i.i334.i, 7, !dbg !8333
  %or.cond.i13.i.i338.not.i = icmp ult i64 %555, %_54.1.i.i332.pre.i, !dbg !8333
  br i1 %or.cond.i13.i.i338.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i339.i, label %bb4.i15.i.i381.i, !dbg !8333, !prof !2723

bb4.i15.i.i381.i:                                 ; preds = %bb24.i314.i
  store <8 x float> %.lcssa22242848, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802858, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722867, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642877, ptr %408, align 1, !dbg !7615
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i334.i, i64 noundef %_7.i10.i.i335.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i332.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !8340, !noalias !8341
  unreachable, !dbg !8340

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i339.i: ; preds = %bb24.i314.i
  %data.i4.i.i.i.i = getelementptr inbounds nuw float, ptr %_131.i310.i, i64 %start1.i.i.i.i.i, !dbg !8355
  %lanes.i3210.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i4.i.i.i.i, align 4, !dbg !8358, !alias.scope !8363, !noalias !8367
  %556 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i3210.sroa.0.0.copyload.i, <8 x float> %lanes.i3210.sroa.0.0.copyload.i), !dbg !8371
  %557 = select <8 x i1> %411, <8 x float> %556, <8 x float> %lanes.i3210.sroa.0.0.copyload.i, !dbg !8376
  %558 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %557, <8 x float> %545, i8 30), !dbg !8381
  %559 = bitcast <8 x float> %558 to <8 x i32>, !dbg !8387
  %560 = icmp slt <8 x i32> %559, zeroinitializer, !dbg !8391
  %561 = fdiv <8 x float> %545, %557, !dbg !8393
  %562 = select <8 x i1> %560, <8 x float> %561, <8 x float> splat (float 1.000000e+00), !dbg !8391
  %_17.i14.i.i340.i = getelementptr inbounds nuw float, ptr %_54.0.i.i331.pre.i, i64 %base.i9.i.i334.i, !dbg !8398
  store <8 x float> %562, ptr %_17.i14.i.i340.i, align 4, !dbg !8402, !alias.scope !8407, !noalias !8411
  %base.i1138.i = shl i64 %_135.i328.i, 3, !dbg !8415
  %563 = or disjoint i64 %base.i1138.i, 7, !dbg !8420
  %or.cond.i1142.not.i = icmp ult i64 %563, %_54.1.i.i332.pre.i, !dbg !8420
  br i1 %or.cond.i1142.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1146.i, label %bb4.i1145.i, !dbg !8420, !prof !2723

bb4.i1145.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i339.i
  store <8 x float> %.lcssa22242848, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802858, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722867, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642877, ptr %408, align 1, !dbg !7615
  %_5.i1139.i = add i64 %base.i1138.i, 8, !dbg !8428
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1138.i, i64 noundef %_5.i1139.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i332.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !8429, !noalias !8430
  unreachable, !dbg !8429

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1146.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i339.i
  %_15.i1144.i = getelementptr inbounds nuw float, ptr %_54.0.i.i331.pre.i, i64 %base.i1138.i, !dbg !8438
  %lanes.i3050.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1144.i, align 4, !dbg !8442, !alias.scope !8447, !noalias !8451
  %position.i.i344.i = zext i32 %storemerge.i.i3488215.i to i64, !dbg !8455
  %564 = icmp eq i32 %storemerge.i.i3488215.i, 0, !dbg !8457
  br i1 %564, label %bb5.i33.i.i, label %bb3.i.i345.i, !dbg !8457

bb3.i.i345.i:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1146.i
  %565 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i36.sroa.0.081078115.i, <8 x float> %lanes.i3050.sroa.0.0.copyload.i), !dbg !8459
  br label %bb5.i33.i.i, !dbg !8468

bb5.i33.i.i:                                      ; preds = %bb3.i.i345.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1146.i
  %minimum.i.i36.sroa.0.0.i = phi <8 x float> [ %565, %bb3.i.i345.i ], [ %lanes.i3050.sroa.0.0.copyload.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1146.i ], !dbg !8469
  %_15.i34.i.i = add nuw nsw i64 %position.i.i344.i, 1, !dbg !8470
  %complete.i.i346.i = icmp eq i64 %_15.i34.i.i, %_18.i22.i.i, !dbg !8470
  br i1 %complete.i.i346.i, label %bb19.i.i377.i, label %bb7.i35.i.i, !dbg !8472

bb7.i35.i.i:                                      ; preds = %bb5.i33.i.i
  %base.i1129.i = shl i64 %_136.i329.i, 3, !dbg !8474
  %566 = or disjoint i64 %base.i1129.i, 7, !dbg !8476
  %or.cond.i1133.not.i = icmp ult i64 %566, %_54.1.i.i332.pre.i, !dbg !8476
  br i1 %or.cond.i1133.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1137.i, label %bb4.i1136.i, !dbg !8476, !prof !2723

bb4.i1136.i:                                      ; preds = %bb7.i35.i.i
  store <8 x float> %.lcssa22242848, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802858, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722867, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642877, ptr %408, align 1, !dbg !7615
  %_5.i1130.i = add i64 %base.i1129.i, 8, !dbg !8480
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1129.i, i64 noundef %_5.i1130.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i332.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !8481, !noalias !8482
  unreachable, !dbg !8481

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1137.i: ; preds = %bb7.i35.i.i
  %_15.i1135.i = getelementptr inbounds nuw float, ptr %_54.0.i.i331.pre.i, i64 %base.i1129.i, !dbg !8486
  %lanes.i3057.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1135.i, align 4, !dbg !8488, !alias.scope !8493, !noalias !8497
  %567 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i3057.sroa.0.0.copyload.i, <8 x float> %minimum.i.i36.sroa.0.0.i), !dbg !8501
  %568 = trunc i64 %_15.i34.i.i to i32, !dbg !8506
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i347.i, !dbg !8508

bb19.i.i377.i:                                    ; preds = %bb5.i33.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %end.sroa.0.0.i.i3758106.i = phi i64 [ %572, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], [ %_135.i328.i, %bb5.i33.i.i ]
  %iter.sroa.0.0.i36.i8105.i = phi i64 [ %_30.i37.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], [ 0, %bb5.i33.i.i ]
  %suffix.i.i10.sroa.0.08104.i = phi <8 x float> [ %570, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], [ %lanes.i3050.sroa.0.0.copyload.i, %bb5.i33.i.i ]
  %base.i1098.i = shl i64 %end.sroa.0.0.i.i3758106.i, 3, !dbg !8509
  %569 = or disjoint i64 %base.i1098.i, 7, !dbg !8514
  %or.cond.i1099.not.i = icmp ult i64 %569, %_54.1.i.i332.pre.i, !dbg !8514
  br i1 %or.cond.i1099.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb4.i1101.i, !dbg !8514, !prof !2723

bb4.i1101.i:                                      ; preds = %bb19.i.i377.i
  store <8 x float> %.lcssa22242848, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802858, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722867, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642877, ptr %408, align 1, !dbg !7615
  %_5.i.i = add i64 %base.i1098.i, 8, !dbg !8518
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1098.i, i64 noundef %_5.i.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i332.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !8519, !noalias !8520
  unreachable, !dbg !8519

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %bb19.i.i377.i
  %_30.i37.i.i = add nuw i64 %iter.sroa.0.0.i36.i8105.i, 1, !dbg !8524
  %_15.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i331.pre.i, i64 %base.i1098.i, !dbg !8535
  %lanes.i3085.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i.i, align 4, !dbg !8537, !alias.scope !8542, !noalias !8546
  %570 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i.i10.sroa.0.08104.i, <8 x float> %lanes.i3085.sroa.0.0.copyload.i), !dbg !8550
  store <8 x float> %570, ptr %_15.i.i, align 4, !dbg !8555, !alias.scope !8561, !noalias !8565
  %571 = icmp eq i64 %end.sroa.0.0.i.i3758106.i, 0, !dbg !8569
  %spec.store.select.i.i379.i = select i1 %571, i64 %ring.i263.i, i64 %end.sroa.0.0.i.i3758106.i, !dbg !8569
  %572 = add i64 %spec.store.select.i.i379.i, -1, !dbg !8570
  %exitcond9625.not.i = icmp eq i64 %_30.i37.i.i, %_18.i22.i.i, !dbg !8571
  br i1 %exitcond9625.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i347.i, label %bb19.i.i377.i, !dbg !8574

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i347.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1137.i
  %minimum.i.i36.sroa.0.1.i = phi <8 x float> [ %567, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1137.i ], [ %minimum.i.i36.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !8469
  %storemerge.i.i348.i = phi i32 [ %568, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1137.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !8575
  %573 = fmul <8 x float> %minimum.i.i36.sroa.0.1.i, splat (float 1.638400e+04), !dbg !8576
  %574 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %573), !dbg !8581
  %575 = fmul <8 x float> %574, splat (float 0x3F10000000000000), !dbg !8586
  %base.i1120.i = shl i64 %_137.i330.i, 3, !dbg !8591
  %576 = or disjoint i64 %base.i1120.i, 7, !dbg !8593
  %or.cond.i1124.not.i = icmp ult i64 %576, %_56.1.i.i350.i, !dbg !8593
  br i1 %or.cond.i1124.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1128.i, label %bb4.i1127.i, !dbg !8593, !prof !2723

bb4.i1127.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i347.i
  store <8 x float> %.lcssa22242848, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802858, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722867, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642877, ptr %408, align 1, !dbg !7615
  %_5.i1121.i = add i64 %base.i1120.i, 8, !dbg !8597
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1120.i, i64 noundef %_5.i1121.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i350.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !8598, !noalias !8599
  unreachable, !dbg !8598

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1128.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i347.i
  %_15.i1126.i = getelementptr inbounds nuw float, ptr %_56.0.i.i349.i, i64 %base.i1120.i, !dbg !8603
  %lanes.i3064.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1126.i, align 4, !dbg !8605, !alias.scope !8610, !noalias !8614
  %577 = fadd <8 x float> %536, %575, !dbg !8618
  %578 = fsub <8 x float> %577, %lanes.i3064.sroa.0.0.copyload.i, !dbg !8623
  %_8.not.i4.i.i357.i = icmp ugt i64 %_7.i10.i.i335.i, %_56.1.i.i350.i
  br i1 %_8.not.i4.i.i357.i, label %bb4.i7.i.i372.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i359.i, !dbg !8628, !prof !165

bb4.i7.i.i372.i:                                  ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1128.i
  store <8 x float> %.lcssa22242848, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802858, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722867, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642877, ptr %408, align 1, !dbg !7615
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i334.i, i64 noundef %_7.i10.i.i335.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i350.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !8633, !noalias !8634
  unreachable, !dbg !8633

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i359.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1128.i
  %_17.i6.i.i360.i = getelementptr inbounds nuw float, ptr %_56.0.i.i349.i, i64 %base.i9.i.i334.i, !dbg !8638
  store <8 x float> %575, ptr %_17.i6.i.i360.i, align 4, !dbg !8640, !alias.scope !8645, !noalias !8649
  %_41.i.i24.sroa.0.0.copyload.i = load <8 x float>, ptr %418, align 32, !dbg !8653, !noalias !8656
  %579 = fdiv <8 x float> %578, %_37.i.i28.sroa.0.0.copyload.i, !dbg !8657
  %580 = fsub <8 x float> splat (float 1.000000e+00), %579, !dbg !8662
  %581 = fsub <8 x float> %580, %_41.i.i24.sroa.0.0.copyload.i, !dbg !8667
  %582 = fmul <8 x float> %553, %581, !dbg !8672
  %583 = fadd <8 x float> %_41.i.i24.sroa.0.0.copyload.i, %582, !dbg !8677
  %584 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %580, <8 x float> %583), !dbg !8681
  %585 = bitcast <8 x float> %584 to <8 x i32>, !dbg !8687
  %586 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %584), !dbg !8694
  %587 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %586, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !8696
  %588 = bitcast <8 x float> %587 to <8 x i32>, !dbg !8702
  %589 = xor <8 x i32> %588, splat (i32 -1), !dbg !8708
  %590 = and <8 x i32> %589, %585, !dbg !8710
  store <8 x i32> %590, ptr %418, align 32, !dbg !8714, !noalias !8656
  %base.i1111.i = shl i64 %_134.i327.i, 3, !dbg !8715
  %591 = or disjoint i64 %base.i1111.i, 7, !dbg !8718
  %or.cond.i1115.not.i = icmp ult i64 %591, %_58.1.i.i362.i, !dbg !8718
  br i1 %or.cond.i1115.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i, label %bb4.i1118.i, !dbg !8718, !prof !2723

bb4.i1118.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i359.i
  store <8 x float> %.lcssa22242848, ptr %416, align 1, !dbg !7589
  store <8 x float> %.lcssa21802858, ptr %407, align 1, !dbg !7611
  store <8 x float> %.lcssa21722867, ptr %_79.i320.i, align 1, !dbg !7614
  store <8 x float> %.lcssa21642877, ptr %408, align 1, !dbg !7615
  %_5.i1112.i = add i64 %base.i1111.i, 8, !dbg !8722
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1111.i, i64 noundef %_5.i1112.i, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i362.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !8723, !noalias !8724
  unreachable, !dbg !8723

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i359.i
  %592 = bitcast <8 x i32> %590 to <8 x float>, !dbg !8728
  %593 = fsub <8 x float> splat (float 1.000000e+00), %592, !dbg !8729
  %_15.i1117.i = getelementptr inbounds nuw float, ptr %_58.0.i.i361.i, i64 %base.i1111.i, !dbg !8734
  %lanes.i3071.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1117.i, align 4, !dbg !8736, !alias.scope !8741, !noalias !8745
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i1117.i, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i3896.i, i64 32, i1 false), !dbg !8749
  %594 = fmul <8 x float> %593, %lanes.i3071.sroa.0.0.copyload.i, !dbg !8756
  %595 = select <8 x i1> %422, <8 x float> %lanes.i3071.sroa.0.0.copyload.i, <8 x float> %594, !dbg !8761
  store <8 x float> %595, ptr %data.i.i.i.i3896.i, align 4, !dbg !8766, !alias.scope !8771, !noalias !8775
  %exitcond9636.not.i = icmp eq i64 %_9.0.i3899.i, %535, !dbg !8245
  br i1 %exitcond9636.not.i, label %bb25.i382.i, label %bb24.i314.i, !dbg !8245

bb25.i382.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i, %bb39.i299.i
  %.lcssa21642876 = phi <8 x float> [ %.lcssa21642877, %bb39.i299.i ], [ %554, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa21722866 = phi <8 x float> [ %.lcssa21722867, %bb39.i299.i ], [ %553, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa21802857 = phi <8 x float> [ %.lcssa21802858, %bb39.i299.i ], [ %548, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa22242847 = phi <8 x float> [ %.lcssa22242848, %bb39.i299.i ], [ %578, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa1082010864.i = phi <8 x float> [ %.lcssa1082010865.i, %bb39.i299.i ], [ %578, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa1014310861.i = phi <8 x float> [ %.lcssa1014310862.i, %bb39.i299.i ], [ %554, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa1015110858.i = phi <8 x float> [ %.lcssa1015110859.i, %bb39.i299.i ], [ %553, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa1015910855.i = phi <8 x float> [ %.lcssa1015910856.i, %bb39.i299.i ], [ %548, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa1016710844.i = phi <8 x float> [ %.lcssa1016710845.i, %bb39.i299.i ], [ %546, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa1017510833.i = phi <8 x float> [ %.lcssa1017510834.i, %bb39.i299.i ], [ %545, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %.lcssa1018310822.i = phi <8 x float> [ %.lcssa1018310823.i, %bb39.i299.i ], [ %540, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %storemerge.i.i348.lcssa82258250.i = phi i32 [ %storemerge.i.i348.lcssa82258251.i, %bb39.i299.i ], [ %storemerge.i.i348.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %minimum.i.i36.sroa.0.08107.lcssa.i = phi <8 x float> [ %minimum.i.i36.sroa.0.08107.lcssa82318242.i, %bb39.i299.i ], [ %minimum.i.i36.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1119.i ]
  %_92.i383.i = add i64 %..i3884.i, %ring_cursor.sroa.0.1.i2858243.i, !dbg !8779
  %_132.not.i384.i = icmp ult i64 %_92.i383.i, %ring.i263.i, !dbg !8780
  %596 = select i1 %_132.not.i384.i, i64 0, i64 %ring.i263.i, !dbg !8780
  %ring_cursor.sroa.0.2.i385.i = sub nuw i64 %_92.i383.i, %596, !dbg !8780
  %_94.i386.i = add i64 %..i3884.i, %main_cursor.sroa.0.1.i2868244.i, !dbg !8783
  %_138.not.i387.i = icmp ult i64 %_94.i386.i, %main.i264.i, !dbg !8784
  %597 = select i1 %_138.not.i387.i, i64 0, i64 %main.i264.i, !dbg !8784
  %main_cursor.sroa.0.2.i388.i = sub nuw i64 %_94.i386.i, %597, !dbg !8784
  %_41.i288.i = icmp ult i64 %_62.i302.i, %..i3866.i, !dbg !7616
  br i1 %_41.i288.i, label %bb17.i289.i, label %bb16.i284.bb13.i270.loopexit_crit_edge.i, !dbg !7616

_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i: ; preds = %bb13.i270.loopexit.i
  store <8 x float> %minimum.i.i36.sroa.0.08107.lcssa8231.lcssa.i, ptr %uniform_left.i249.i, align 1, !noalias !6249
  %598 = trunc i64 %main_cursor.sroa.0.1.i286.lcssa.i to i32, !dbg !8786
  %599 = trunc i64 %ring_cursor.sroa.0.1.i285.lcssa.i to i32, !dbg !8788
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !8789

_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i
  %left_phase.i420.i = phi i32 [ %left_phase.i420.i.pre, %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge ], [ %storemerge.i.i348.lcssa82258250.lcssa10877.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !7565
  %ring_cursor.sroa.0.0.i271.lcssa.i = phi i32 [ %_26.i266.i, %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge ], [ %599, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !7542
  %main_cursor.sroa.0.0.i272.lcssa.i = phi i32 [ %_25.i265.i, %bb4.i.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i_crit_edge ], [ %598, %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !7538
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i228.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i249.i, i64 32, i1 false), !dbg !8789, !noalias !6249
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i249.i), !dbg !8790, !noalias !7520
  %600 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !8791
  %_147.1.i422.i = load i64, ptr %600, align 8, !dbg !8791, !alias.scope !8792, !noalias !7523, !noundef !12
  %_8.i3554.i = icmp samesign ugt i64 %_147.1.i422.i, 7, !dbg !8793
  br i1 %_8.i3554.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3557.i, label %bb2.i3555.i, !dbg !8793, !prof !651

bb2.i3555.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_147.1.i422.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !8798, !noalias !8799
  unreachable, !dbg !8798

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3557.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %601 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !8791
  %_147.0.i421.i = load ptr, ptr %601, align 8, !dbg !8791, !alias.scope !8792, !noalias !7523, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_147.0.i421.i, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i228.i, i64 32, i1 false), !dbg !8803, !noalias !5902
  %602 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !8807
  %_148.0.i423.i = load ptr, ptr %602, align 8, !dbg !8807, !alias.scope !8792, !noalias !7523, !nonnull !12, !noundef !12
  %603 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !8807
  %_148.1.i424.i = load i64, ptr %603, align 8, !dbg !8807, !alias.scope !8792, !noalias !7523, !noundef !12
  %604 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i420.i), !dbg !8808
  br i1 %604, label %bb2.i3919.i, label %bb6.i3914.i, !dbg !8808

bb6.i3914.i:                                      ; preds = %bb2.i3919.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3557.i
  %end_or_len.idx.i.i = shl nuw nsw i64 %_148.1.i424.i, 2, !dbg !8812
  %end_or_len.i.i = getelementptr inbounds nuw i8, ptr %_148.0.i423.i, i64 %end_or_len.idx.i.i, !dbg !8812
  %_293.i.i = icmp eq i64 %_148.1.i424.i, 0, !dbg !8816
  br i1 %_293.i.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i, label %bb10.i3915.i, !dbg !8819

bb2.i3919.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3557.i
  %bytes1.sroa.0.0.zext.i.i = and i32 %left_phase.i420.i, 255, !dbg !8820
  %bytes1.sroa.0.0.isplat.i.i = mul nuw i32 %bytes1.sroa.0.0.zext.i.i, 16843009, !dbg !8820
  %_5.i3920.i = icmp eq i32 %left_phase.i420.i, %bytes1.sroa.0.0.isplat.i.i, !dbg !8821
  br i1 %_5.i3920.i, label %bb3.i3921.i, label %bb6.i3914.i, !dbg !8821

bb3.i3921.i:                                      ; preds = %bb2.i3919.i
  %bytes.sroa.0.0.extract.trunc.i.i = trunc i32 %left_phase.i420.i to i8, !dbg !8822
  %605 = shl nuw nsw i64 %_148.1.i424.i, 2, !dbg !8824
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_148.0.i423.i, i8 %bytes.sroa.0.0.extract.trunc.i.i, i64 %605, i1 false), !dbg !8824, !alias.scope !8825, !noalias !8250
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i, !dbg !8828

bb10.i3915.i:                                     ; preds = %bb6.i3914.i, %bb10.i3915.i
  %iter.sroa.0.04.i.i = phi ptr [ %_38.i3916.i, %bb10.i3915.i ], [ %_148.0.i423.i, %bb6.i3914.i ]
  %_38.i3916.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i.i, i64 4, !dbg !8829
  store i32 %left_phase.i420.i, ptr %iter.sroa.0.04.i.i, align 4, !dbg !8831, !alias.scope !8825, !noalias !8250
  %_29.i3917.i = icmp eq ptr %_38.i3916.i, %end_or_len.i.i, !dbg !8816
  br i1 %_29.i3917.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i, label %bb10.i3915.i, !dbg !8819

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i: ; preds = %bb10.i3915.i, %bb3.i3921.i, %bb6.i3914.i
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i256.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25) #31, !dbg !8832, !noalias !8250
  store i32 %main_cursor.sroa.0.0.i272.lcssa.i, ptr %_25.i, align 4, !dbg !8786, !alias.scope !7540, !noalias !7541
  store i32 %ring_cursor.sroa.0.0.i271.lcssa.i, ptr %360, align 4, !dbg !8788, !alias.scope !7540, !noalias !7541
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i250.i), !dbg !8833, !noalias !7520
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i256.i), !dbg !8834, !noalias !7520
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !7509

bb6.i.i:                                          ; preds = %_RNCNvCsdvPQf9CMsz3_17true_peak_limiter13lanes_uniform0B3_.exit.i.i3812.i, %bb2.i.i.i3808.i, %bb13.i.i3801.i, %bb13.i5.i3824.i
  br i1 %_12.i.i3785.i, label %bb7.i.i, label %bb8.i.i, !dbg !8835

bb2.i.i:                                          ; preds = %bb1.i3.i3821.i, %bb2.i3815.i
  br i1 %_12.i.i3785.i, label %bb3.i.i, label %bb4.i.i, !dbg !8836

bb7.i.i:                                          ; preds = %bb6.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8837), !dbg !8840
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8841), !dbg !8840
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8843), !dbg !8840
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i641.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !8845, !noalias !5902
  %606 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !8849
  %607 = load i8, ptr %606, align 32, !dbg !8849, !range !17, !alias.scope !8853, !noalias !8854, !noundef !12
  %608 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !8857
  %609 = load i8, ptr %608, align 1, !dbg !8857, !range !17, !alias.scope !8853, !noalias !8854, !noundef !12
  %_24.i.i = load i32, ptr %_25.i, align 4, !dbg !8859, !alias.scope !8861, !noalias !8862, !noundef !12
  %610 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !8863
  %_26.i648.i = load i32, ptr %610, align 4, !dbg !8863, !alias.scope !8861, !noalias !8862, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %scratch.i.i), !dbg !8865, !noalias !8867
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %scratch.i.i, i8 0, i64 32, i1 false), !noalias !8867
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i635.i), !dbg !8868, !noalias !8867
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i635.i, i8 0, i64 1024, i1 false), !noalias !8867
  %611 = add nuw nsw i64 %_31, 31, !dbg !8870
  %yield_count.sroa.0.0.i.i3925.i = lshr i64 %611, 5, !dbg !8870
  %hot_left.i641.promoted.i = load <8 x float>, ptr %hot_left.i641.i, align 1, !noalias !6249
  %_75.not.i8067.i = icmp eq i64 %yield_count.sroa.0.0.i.i3925.i, 0, !dbg !8877
  br i1 %_75.not.i8067.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, label %bb29.i.lr.ph.i, !dbg !8877

bb29.i.lr.ph.i:                                   ; preds = %bb7.i.i
  %612 = zext i32 %_26.i648.i to i64, !dbg !8863
  %613 = zext i32 %_24.i.i to i64, !dbg !8859
  %_22.i645.i = trunc nuw i8 %609 to i1, !dbg !8857
  %_21.i642.i = trunc nuw i8 %607 to i1, !dbg !8849
  %614 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !8886
  %615 = bitcast <8 x float> %614 to <8 x i32>, !dbg !8892
  %616 = xor <8 x i32> %615, splat (i32 -1), !dbg !8898
  %history.i.i611.sroa.10.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 32
  %history.i.i611.sroa.13.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 64
  %history.i.i611.sroa.16.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 96
  %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 128
  %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 160
  %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 192
  %history.i.i611.sroa.29.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 224
  %history.i.i611.sroa.32.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 256
  %history.i.i611.sroa.35.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 288
  %history.i.i611.sroa.38.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 320
  %history.i.i611.sroa.41.0.hot_left.i641.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 352
  %617 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %618 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %619 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i685.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %620 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %621 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %622 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i686.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %623 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %624 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %625 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i687.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %626 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %627 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %628 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i688.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %629 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %630 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %631 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i689.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %632 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %633 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %634 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i690.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %635 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %636 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %637 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i691.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %638 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %639 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %640 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i692.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %641 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %642 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %643 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i693.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %644 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %645 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %646 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i694.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %647 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %648 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %649 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i695.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %650 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %651 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %652 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %_47.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 384
  %_48.i666.i = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 512
  %653 = select i1 %_21.i642.i, <8 x i32> %615, <8 x i32> %616
  %654 = icmp slt <8 x i32> %653, zeroinitializer
  %655 = getelementptr inbounds nuw i8, ptr %self, i64 1624
  %656 = getelementptr inbounds nuw i8, ptr %self, i64 1840
  %657 = getelementptr inbounds nuw i8, ptr %self, i64 1688
  %658 = getelementptr inbounds nuw i8, ptr %self, i64 1680
  %659 = getelementptr inbounds nuw i8, ptr %self, i64 1768
  %660 = getelementptr inbounds nuw i8, ptr %self, i64 1760
  %661 = getelementptr inbounds nuw i8, ptr %self, i64 1736
  %662 = getelementptr inbounds nuw i8, ptr %self, i64 1728
  %663 = getelementptr inbounds nuw i8, ptr %self, i64 1704
  %664 = getelementptr inbounds nuw i8, ptr %self, i64 1696
  %665 = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 672
  %666 = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 704
  %667 = getelementptr inbounds nuw i8, ptr %hot_left.i641.i, i64 640
  %668 = getelementptr inbounds nuw i8, ptr %self, i64 1672
  %669 = getelementptr inbounds nuw i8, ptr %self, i64 1664
  %670 = select i1 %_22.i645.i, <8 x i32> %615, <8 x i32> %616
  %671 = icmp slt <8 x i32> %670, zeroinitializer
  %672 = getelementptr inbounds nuw i8, ptr %self, i64 1632
  %history.i.i611.sroa.10.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.10.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.13.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.13.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.16.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.16.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.19.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.22.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.25.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.29.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.29.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.32.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.32.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.35.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.35.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.38.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.38.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %history.i.i611.sroa.41.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %history.i.i611.sroa.41.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8900, !noalias !6249
  %_8.i.i627.sroa.0.0.copyload.i = load <8 x float>, ptr %_47.i.i, align 32, !noalias !6249
  %_9.i.i626.sroa.0.0.copyload.i = load <8 x float>, ptr %_48.i666.i, align 32, !noalias !6249
  %_64.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %666, align 32, !noalias !6249
  %iter.i.i.sroa.0.0.ptr7997.1.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 4
  %iter.i.i.sroa.0.0.ptr7997.2.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 8
  %iter.i.i.sroa.0.0.ptr7997.3.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 12
  %iter.i.i.sroa.0.0.ptr7997.4.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 16
  %iter.i.i.sroa.0.0.ptr7997.5.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 20
  %iter.i.i.sroa.0.0.ptr7997.6.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 24
  %iter.i.i.sroa.0.0.ptr7997.7.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 28
  %.promoted10809.i = load <8 x float>, ptr %665, align 32, !noalias !6249
  %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1
  %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1
  %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1
  br label %bb29.i.i, !dbg !8877

bb14.i.bb12.i651.loopexit_crit_edge.i:            ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3537.i
  store <8 x float> %816, ptr %665, align 32, !dbg !8904, !noalias !6249
  br label %bb12.i651.loopexit.i, !dbg !8913

bb12.i651.loopexit.i:                             ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i661.i, %bb14.i.bb12.i651.loopexit_crit_edge.i
  %.lcssa1039510810.i = phi <8 x float> [ %816, %bb14.i.bb12.i651.loopexit_crit_edge.i ], [ %.lcssa1039510811.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i661.i ]
  %ring_cursor.sroa.0.1.i663.lcssa.i = phi i64 [ %spec.store.select9.i.i, %bb14.i.bb12.i651.loopexit_crit_edge.i ], [ %ring_cursor.sroa.0.0.i6538071.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i661.i ], !dbg !8919
  %main_cursor.sroa.0.1.i664.lcssa.i = phi i64 [ %spec.store.select.i.i, %bb14.i.bb12.i651.loopexit_crit_edge.i ], [ %main_cursor.sroa.0.0.i6548072.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i661.i ], !dbg !8920
  %_75.not.i.i = icmp eq i64 %675, 0, !dbg !8877
  %indvars.iv.next9597.i = add nsw i64 %indvars.iv9596.i, -32, !dbg !8877
  br i1 %_75.not.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, label %bb29.i.i, !dbg !8877

bb29.i.i:                                         ; preds = %bb12.i651.loopexit.i, %bb29.i.lr.ph.i
  %history.i.i611.sroa.25.sroa.0.0.lcssa.i2828 = phi <8 x float> [ %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i.promoted, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.25.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ]
  %history.i.i611.sroa.22.sroa.0.0.lcssa.i2809 = phi <8 x float> [ %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i.promoted, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.22.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ]
  %history.i.i611.sroa.19.sroa.0.0.lcssa.i2790 = phi <8 x float> [ %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i.promoted, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.19.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ]
  %.lcssa1039510811.i = phi <8 x float> [ %.promoted10809.i, %bb29.i.lr.ph.i ], [ %.lcssa1039510810.i, %bb12.i651.loopexit.i ]
  %history.i.i611.sroa.16.sroa.0.0.lcssa10790.i = phi <8 x float> [ %history.i.i611.sroa.16.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.16.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ]
  %history.i.i611.sroa.13.sroa.0.0.lcssa10771.i = phi <8 x float> [ %history.i.i611.sroa.13.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.13.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ]
  %history.i.i611.sroa.10.sroa.0.0.lcssa10752.i = phi <8 x float> [ %history.i.i611.sroa.10.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.10.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ]
  %history.i.i611.sroa.41.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i611.sroa.41.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.41.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ], !dbg !8900
  %history.i.i611.sroa.38.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i611.sroa.38.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.38.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ], !dbg !8900
  %history.i.i611.sroa.35.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i611.sroa.35.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.35.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ], !dbg !8900
  %history.i.i611.sroa.32.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i611.sroa.32.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.32.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ], !dbg !8900
  %history.i.i611.sroa.29.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i611.sroa.29.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.29.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ], !dbg !8900
  %history.i.i611.sroa.25.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i611.sroa.25.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.25.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ], !dbg !8900
  %history.i.i611.sroa.22.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i611.sroa.22.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.22.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ], !dbg !8900
  %history.i.i611.sroa.19.sroa.0.0.copyload.i = phi <8 x float> [ %history.i.i611.sroa.19.sroa.0.0.copyload.pre.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.19.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ], !dbg !8900
  %indvars.iv9596.i = phi i64 [ %_31, %bb29.i.lr.ph.i ], [ %indvars.iv.next9597.i, %bb12.i651.loopexit.i ]
  %main_cursor.sroa.0.0.i6548072.i = phi i64 [ %613, %bb29.i.lr.ph.i ], [ %main_cursor.sroa.0.1.i664.lcssa.i, %bb12.i651.loopexit.i ]
  %ring_cursor.sroa.0.0.i6538071.i = phi i64 [ %612, %bb29.i.lr.ph.i ], [ %ring_cursor.sroa.0.1.i663.lcssa.i, %bb12.i651.loopexit.i ]
  %iter3.sroa.0.0.i6528070.i = phi i64 [ %yield_count.sroa.0.0.i.i3925.i, %bb29.i.lr.ph.i ], [ %675, %bb12.i651.loopexit.i ]
  %iter.sroa.0.0.i8069.i = phi i64 [ 0, %bb29.i.lr.ph.i ], [ %674, %bb12.i651.loopexit.i ]
  %history.i.i611.sroa.0.0.lcssa80448068.i = phi <8 x float> [ %hot_left.i641.promoted.i, %bb29.i.lr.ph.i ], [ %history.i.i611.sroa.0.0.lcssa.i, %bb12.i651.loopexit.i ]
  %673 = tail call i64 @llvm.umax.i64(i64 %indvars.iv9596.i, i64 1), !dbg !8921
  %umax9613.i = tail call i64 @llvm.umin.i64(i64 %673, i64 32), !dbg !8921
  %674 = add nuw nsw i64 %iter.sroa.0.0.i8069.i, 32, !dbg !8921
  %675 = add nsw i64 %iter3.sroa.0.0.i6528070.i, -1, !dbg !8925
  %_20.i.i6607962.not.i = icmp eq i64 %iter.sroa.0.0.i8069.i, %_31, !dbg !8926
  br i1 %_20.i.i6607962.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i661.i, label %bb5.i.i678.lr.ph.i, !dbg !8930

bb5.i.i678.lr.ph.i:                               ; preds = %bb29.i.i
  %_5.i2622.i = load <8 x float>, ptr %self, align 32, !alias.scope !5901, !noalias !5902
  %_14.i.i.i.i576.sroa.0.0.copyload.i = load <8 x float>, ptr %617, align 32, !alias.scope !5901, !noalias !5902
  %_17.i.i.i.i573.sroa.0.0.copyload.i = load <8 x float>, ptr %618, align 32, !alias.scope !5901, !noalias !5902
  %_20.i.i.i.i570.sroa.0.0.copyload.i = load <8 x float>, ptr %619, align 32, !alias.scope !5901, !noalias !5902
  %_25.i.i.i.i566.sroa.0.0.copyload.i = load <8 x float>, ptr %row12.i.i.i.i685.i, align 32, !alias.scope !5901, !noalias !5902
  %_28.i.i.i.i563.sroa.0.0.copyload.i = load <8 x float>, ptr %620, align 32, !alias.scope !5901, !noalias !5902
  %_31.i.i.i.i560.sroa.0.0.copyload.i = load <8 x float>, ptr %621, align 32, !alias.scope !5901, !noalias !5902
  %_34.i.i.i.i557.sroa.0.0.copyload.i = load <8 x float>, ptr %622, align 32, !alias.scope !5901, !noalias !5902
  %_39.i.i.i.i553.sroa.0.0.copyload.i = load <8 x float>, ptr %row13.i.i.i.i686.i, align 32, !alias.scope !5901, !noalias !5902
  %_42.i.i.i.i550.sroa.0.0.copyload.i = load <8 x float>, ptr %623, align 32, !alias.scope !5901, !noalias !5902
  %_45.i.i.i.i547.sroa.0.0.copyload.i = load <8 x float>, ptr %624, align 32, !alias.scope !5901, !noalias !5902
  %_48.i.i.i.i544.sroa.0.0.copyload.i = load <8 x float>, ptr %625, align 32, !alias.scope !5901, !noalias !5902
  %_53.i.i.i.i540.sroa.0.0.copyload.i = load <8 x float>, ptr %row14.i.i.i.i687.i, align 32, !alias.scope !5901, !noalias !5902
  %_56.i.i.i.i537.sroa.0.0.copyload.i = load <8 x float>, ptr %626, align 32, !alias.scope !5901, !noalias !5902
  %_59.i.i.i.i534.sroa.0.0.copyload.i = load <8 x float>, ptr %627, align 32, !alias.scope !5901, !noalias !5902
  %_62.i.i.i.i531.sroa.0.0.copyload.i = load <8 x float>, ptr %628, align 32, !alias.scope !5901, !noalias !5902
  %_67.i.i.i.i527.sroa.0.0.copyload.i = load <8 x float>, ptr %row15.i.i.i.i688.i, align 32, !alias.scope !5901, !noalias !5902
  %_70.i.i.i.i524.sroa.0.0.copyload.i = load <8 x float>, ptr %629, align 32, !alias.scope !5901, !noalias !5902
  %_73.i.i.i.i521.sroa.0.0.copyload.i = load <8 x float>, ptr %630, align 32, !alias.scope !5901, !noalias !5902
  %_76.i.i.i.i518.sroa.0.0.copyload.i = load <8 x float>, ptr %631, align 32, !alias.scope !5901, !noalias !5902
  %_81.i.i.i.i514.sroa.0.0.copyload.i = load <8 x float>, ptr %row16.i.i.i.i689.i, align 32, !alias.scope !5901, !noalias !5902
  %_84.i.i.i.i511.sroa.0.0.copyload.i = load <8 x float>, ptr %632, align 32, !alias.scope !5901, !noalias !5902
  %_87.i.i.i.i508.sroa.0.0.copyload.i = load <8 x float>, ptr %633, align 32, !alias.scope !5901, !noalias !5902
  %_90.i.i.i.i505.sroa.0.0.copyload.i = load <8 x float>, ptr %634, align 32, !alias.scope !5901, !noalias !5902
  %_95.i.i.i.i501.sroa.0.0.copyload.i = load <8 x float>, ptr %row17.i.i.i.i690.i, align 32, !alias.scope !5901, !noalias !5902
  %_98.i.i.i.i498.sroa.0.0.copyload.i = load <8 x float>, ptr %635, align 32, !alias.scope !5901, !noalias !5902
  %_101.i.i.i.i495.sroa.0.0.copyload.i = load <8 x float>, ptr %636, align 32, !alias.scope !5901, !noalias !5902
  %_104.i.i.i.i492.sroa.0.0.copyload.i = load <8 x float>, ptr %637, align 32, !alias.scope !5901, !noalias !5902
  %_109.i.i.i.i488.sroa.0.0.copyload.i = load <8 x float>, ptr %row18.i.i.i.i691.i, align 32, !alias.scope !5901, !noalias !5902
  %_112.i.i.i.i485.sroa.0.0.copyload.i = load <8 x float>, ptr %638, align 32, !alias.scope !5901, !noalias !5902
  %_115.i.i.i.i482.sroa.0.0.copyload.i = load <8 x float>, ptr %639, align 32, !alias.scope !5901, !noalias !5902
  %_118.i.i.i.i479.sroa.0.0.copyload.i = load <8 x float>, ptr %640, align 32, !alias.scope !5901, !noalias !5902
  %_123.i.i.i.i475.sroa.0.0.copyload.i = load <8 x float>, ptr %row19.i.i.i.i692.i, align 32, !alias.scope !5901, !noalias !5902
  %_126.i.i.i.i472.sroa.0.0.copyload.i = load <8 x float>, ptr %641, align 32, !alias.scope !5901, !noalias !5902
  %_129.i.i.i.i469.sroa.0.0.copyload.i = load <8 x float>, ptr %642, align 32, !alias.scope !5901, !noalias !5902
  %_132.i.i.i.i466.sroa.0.0.copyload.i = load <8 x float>, ptr %643, align 32, !alias.scope !5901, !noalias !5902
  %_137.i.i.i.i462.sroa.0.0.copyload.i = load <8 x float>, ptr %row20.i.i.i.i693.i, align 32, !alias.scope !5901, !noalias !5902
  %_140.i.i.i.i459.sroa.0.0.copyload.i = load <8 x float>, ptr %644, align 32, !alias.scope !5901, !noalias !5902
  %_143.i.i.i.i456.sroa.0.0.copyload.i = load <8 x float>, ptr %645, align 32, !alias.scope !5901, !noalias !5902
  %_146.i.i.i.i453.sroa.0.0.copyload.i = load <8 x float>, ptr %646, align 32, !alias.scope !5901, !noalias !5902
  %_151.i.i.i.i449.sroa.0.0.copyload.i = load <8 x float>, ptr %row21.i.i.i.i694.i, align 32, !alias.scope !5901, !noalias !5902
  %_154.i.i.i.i446.sroa.0.0.copyload.i = load <8 x float>, ptr %647, align 32, !alias.scope !5901, !noalias !5902
  %_157.i.i.i.i443.sroa.0.0.copyload.i = load <8 x float>, ptr %648, align 32, !alias.scope !5901, !noalias !5902
  %_160.i.i.i.i440.sroa.0.0.copyload.i = load <8 x float>, ptr %649, align 32, !alias.scope !5901, !noalias !5902
  %_165.i.i.i.i436.sroa.0.0.copyload.i = load <8 x float>, ptr %row22.i.i.i.i695.i, align 32, !alias.scope !5901, !noalias !5902
  %_168.i.i.i.i433.sroa.0.0.copyload.i = load <8 x float>, ptr %650, align 32, !alias.scope !5901, !noalias !5902
  %_171.i.i.i.i430.sroa.0.0.copyload.i = load <8 x float>, ptr %651, align 32, !alias.scope !5901, !noalias !5902
  %_174.i.i.i.i427.sroa.0.0.copyload.i = load <8 x float>, ptr %652, align 32, !alias.scope !5901, !noalias !5902
  br label %bb5.i.i678.i, !dbg !8930

bb5.i.i678.i:                                     ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i, %bb5.i.i678.lr.ph.i
  %iter.sroa.0.0.i.i6597974.i = phi i64 [ 0, %bb5.i.i678.lr.ph.i ], [ %676, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.10.sroa.0.07973.i = phi <8 x float> [ %history.i.i611.sroa.10.sroa.0.0.lcssa10752.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.0.07963.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.13.sroa.0.07972.i = phi <8 x float> [ %history.i.i611.sroa.13.sroa.0.0.lcssa10771.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.10.sroa.0.07973.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.16.sroa.0.07971.i = phi <8 x float> [ %history.i.i611.sroa.16.sroa.0.0.lcssa10790.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.13.sroa.0.07972.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.19.sroa.0.07970.i = phi <8 x float> [ %history.i.i611.sroa.19.sroa.0.0.copyload.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.16.sroa.0.07971.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.22.sroa.0.07969.i = phi <8 x float> [ %history.i.i611.sroa.22.sroa.0.0.copyload.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.19.sroa.0.07970.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.38.sroa.0.07968.i = phi <8 x float> [ %history.i.i611.sroa.38.sroa.0.0.copyload.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.35.sroa.0.07967.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.35.sroa.0.07967.i = phi <8 x float> [ %history.i.i611.sroa.35.sroa.0.0.copyload.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.32.sroa.0.07966.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.32.sroa.0.07966.i = phi <8 x float> [ %history.i.i611.sroa.32.sroa.0.0.copyload.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.29.sroa.0.07965.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.29.sroa.0.07965.i = phi <8 x float> [ %history.i.i611.sroa.29.sroa.0.0.copyload.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.25.sroa.0.07964.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.25.sroa.0.07964.i = phi <8 x float> [ %history.i.i611.sroa.25.sroa.0.0.copyload.i, %bb5.i.i678.lr.ph.i ], [ %history.i.i611.sroa.22.sroa.0.07969.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %history.i.i611.sroa.0.07963.i = phi <8 x float> [ %history.i.i611.sroa.0.0.lcssa80448068.i, %bb5.i.i678.lr.ph.i ], [ %lanes.i3142.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ]
  %676 = add nuw nsw i64 %iter.sroa.0.0.i.i6597974.i, 1, !dbg !8931
  %_11.i.i679.i = add nuw nsw i64 %iter.sroa.0.0.i.i6597974.i, %iter.sroa.0.0.i8069.i, !dbg !8934
  %base.i.i680.i = shl i64 %_11.i.i679.i, 3, !dbg !8934
  %_24.i.i681.i = icmp samesign ugt i64 %base.i.i680.i, %_39.1, !dbg !8935
  br i1 %_24.i.i681.i, label %bb7.i.i707.i, label %bb8.i.i682.i, !dbg !8935, !prof !639

bb8.i.i682.i:                                     ; preds = %bb5.i.i678.i
  %_27.i.i683.i = sub nuw nsw i64 %_39.1, %base.i.i680.i, !dbg !8938
  %_8.i3145.i = icmp samesign ugt i64 %_27.i.i683.i, 7, !dbg !8939
  br i1 %_8.i3145.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i, label %bb2.i3146.i, !dbg !8939, !prof !651

bb2.i3146.i:                                      ; preds = %bb8.i.i682.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i2790, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i2809, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i2828, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i683.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !8945, !noalias !8946
  unreachable, !dbg !8945

bb7.i.i707.i:                                     ; preds = %bb5.i.i678.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i2790, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i2809, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i2828, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i680.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !8953, !noalias !8954
  unreachable, !dbg !8953

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i: ; preds = %bb8.i.i682.i
  %_31.i19.i684.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i.i680.i, !dbg !8955
  %lanes.i3142.sroa.0.0.copyload.i = load <8 x float>, ptr %_31.i19.i684.i, align 4, !dbg !8957, !alias.scope !8961, !noalias !8965
  %677 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i611.sroa.22.sroa.0.07969.i), !dbg !8967
  %678 = fmul <8 x float> %_5.i2622.i, %lanes.i3142.sroa.0.0.copyload.i, !dbg !8974
  %679 = fadd <8 x float> %678, zeroinitializer, !dbg !8980
  %680 = fmul <8 x float> %_14.i.i.i.i576.sroa.0.0.copyload.i, %lanes.i3142.sroa.0.0.copyload.i, !dbg !8985
  %681 = fadd <8 x float> %680, zeroinitializer, !dbg !8990
  %682 = fmul <8 x float> %_17.i.i.i.i573.sroa.0.0.copyload.i, %lanes.i3142.sroa.0.0.copyload.i, !dbg !8995
  %683 = fadd <8 x float> %682, zeroinitializer, !dbg !9000
  %684 = fmul <8 x float> %_20.i.i.i.i570.sroa.0.0.copyload.i, %lanes.i3142.sroa.0.0.copyload.i, !dbg !9005
  %685 = fadd <8 x float> %684, zeroinitializer, !dbg !9010
  %686 = fmul <8 x float> %_25.i.i.i.i566.sroa.0.0.copyload.i, %history.i.i611.sroa.0.07963.i, !dbg !9015
  %687 = fadd <8 x float> %686, %679, !dbg !9020
  %688 = fmul <8 x float> %_28.i.i.i.i563.sroa.0.0.copyload.i, %history.i.i611.sroa.0.07963.i, !dbg !9025
  %689 = fadd <8 x float> %688, %681, !dbg !9030
  %690 = fmul <8 x float> %_31.i.i.i.i560.sroa.0.0.copyload.i, %history.i.i611.sroa.0.07963.i, !dbg !9035
  %691 = fadd <8 x float> %690, %683, !dbg !9040
  %692 = fmul <8 x float> %_34.i.i.i.i557.sroa.0.0.copyload.i, %history.i.i611.sroa.0.07963.i, !dbg !9045
  %693 = fadd <8 x float> %692, %685, !dbg !9050
  %694 = fmul <8 x float> %_39.i.i.i.i553.sroa.0.0.copyload.i, %history.i.i611.sroa.10.sroa.0.07973.i, !dbg !9055
  %695 = fadd <8 x float> %694, %687, !dbg !9060
  %696 = fmul <8 x float> %_42.i.i.i.i550.sroa.0.0.copyload.i, %history.i.i611.sroa.10.sroa.0.07973.i, !dbg !9065
  %697 = fadd <8 x float> %696, %689, !dbg !9070
  %698 = fmul <8 x float> %_45.i.i.i.i547.sroa.0.0.copyload.i, %history.i.i611.sroa.10.sroa.0.07973.i, !dbg !9075
  %699 = fadd <8 x float> %698, %691, !dbg !9080
  %700 = fmul <8 x float> %_48.i.i.i.i544.sroa.0.0.copyload.i, %history.i.i611.sroa.10.sroa.0.07973.i, !dbg !9085
  %701 = fadd <8 x float> %700, %693, !dbg !9090
  %702 = fmul <8 x float> %_53.i.i.i.i540.sroa.0.0.copyload.i, %history.i.i611.sroa.13.sroa.0.07972.i, !dbg !9095
  %703 = fadd <8 x float> %702, %695, !dbg !9100
  %704 = fmul <8 x float> %_56.i.i.i.i537.sroa.0.0.copyload.i, %history.i.i611.sroa.13.sroa.0.07972.i, !dbg !9105
  %705 = fadd <8 x float> %704, %697, !dbg !9110
  %706 = fmul <8 x float> %_59.i.i.i.i534.sroa.0.0.copyload.i, %history.i.i611.sroa.13.sroa.0.07972.i, !dbg !9115
  %707 = fadd <8 x float> %706, %699, !dbg !9120
  %708 = fmul <8 x float> %_62.i.i.i.i531.sroa.0.0.copyload.i, %history.i.i611.sroa.13.sroa.0.07972.i, !dbg !9125
  %709 = fadd <8 x float> %708, %701, !dbg !9130
  %710 = fmul <8 x float> %_67.i.i.i.i527.sroa.0.0.copyload.i, %history.i.i611.sroa.16.sroa.0.07971.i, !dbg !9135
  %711 = fadd <8 x float> %710, %703, !dbg !9140
  %712 = fmul <8 x float> %_70.i.i.i.i524.sroa.0.0.copyload.i, %history.i.i611.sroa.16.sroa.0.07971.i, !dbg !9145
  %713 = fadd <8 x float> %712, %705, !dbg !9150
  %714 = fmul <8 x float> %_73.i.i.i.i521.sroa.0.0.copyload.i, %history.i.i611.sroa.16.sroa.0.07971.i, !dbg !9155
  %715 = fadd <8 x float> %714, %707, !dbg !9160
  %716 = fmul <8 x float> %_76.i.i.i.i518.sroa.0.0.copyload.i, %history.i.i611.sroa.16.sroa.0.07971.i, !dbg !9165
  %717 = fadd <8 x float> %716, %709, !dbg !9170
  %718 = fmul <8 x float> %_81.i.i.i.i514.sroa.0.0.copyload.i, %history.i.i611.sroa.19.sroa.0.07970.i, !dbg !9175
  %719 = fadd <8 x float> %718, %711, !dbg !9180
  %720 = fmul <8 x float> %_84.i.i.i.i511.sroa.0.0.copyload.i, %history.i.i611.sroa.19.sroa.0.07970.i, !dbg !9185
  %721 = fadd <8 x float> %720, %713, !dbg !9190
  %722 = fmul <8 x float> %_87.i.i.i.i508.sroa.0.0.copyload.i, %history.i.i611.sroa.19.sroa.0.07970.i, !dbg !9195
  %723 = fadd <8 x float> %722, %715, !dbg !9200
  %724 = fmul <8 x float> %_90.i.i.i.i505.sroa.0.0.copyload.i, %history.i.i611.sroa.19.sroa.0.07970.i, !dbg !9205
  %725 = fadd <8 x float> %724, %717, !dbg !9210
  %726 = fmul <8 x float> %_95.i.i.i.i501.sroa.0.0.copyload.i, %history.i.i611.sroa.22.sroa.0.07969.i, !dbg !9215
  %727 = fadd <8 x float> %726, %719, !dbg !9220
  %728 = fmul <8 x float> %_98.i.i.i.i498.sroa.0.0.copyload.i, %history.i.i611.sroa.22.sroa.0.07969.i, !dbg !9225
  %729 = fadd <8 x float> %728, %721, !dbg !9230
  %730 = fmul <8 x float> %_101.i.i.i.i495.sroa.0.0.copyload.i, %history.i.i611.sroa.22.sroa.0.07969.i, !dbg !9235
  %731 = fadd <8 x float> %730, %723, !dbg !9240
  %732 = fmul <8 x float> %_104.i.i.i.i492.sroa.0.0.copyload.i, %history.i.i611.sroa.22.sroa.0.07969.i, !dbg !9245
  %733 = fadd <8 x float> %732, %725, !dbg !9250
  %734 = fmul <8 x float> %_109.i.i.i.i488.sroa.0.0.copyload.i, %history.i.i611.sroa.25.sroa.0.07964.i, !dbg !9255
  %735 = fadd <8 x float> %734, %727, !dbg !9260
  %736 = fmul <8 x float> %_112.i.i.i.i485.sroa.0.0.copyload.i, %history.i.i611.sroa.25.sroa.0.07964.i, !dbg !9265
  %737 = fadd <8 x float> %736, %729, !dbg !9270
  %738 = fmul <8 x float> %_115.i.i.i.i482.sroa.0.0.copyload.i, %history.i.i611.sroa.25.sroa.0.07964.i, !dbg !9275
  %739 = fadd <8 x float> %738, %731, !dbg !9280
  %740 = fmul <8 x float> %_118.i.i.i.i479.sroa.0.0.copyload.i, %history.i.i611.sroa.25.sroa.0.07964.i, !dbg !9285
  %741 = fadd <8 x float> %740, %733, !dbg !9290
  %742 = fmul <8 x float> %_123.i.i.i.i475.sroa.0.0.copyload.i, %history.i.i611.sroa.29.sroa.0.07965.i, !dbg !9295
  %743 = fadd <8 x float> %742, %735, !dbg !9300
  %744 = fmul <8 x float> %_126.i.i.i.i472.sroa.0.0.copyload.i, %history.i.i611.sroa.29.sroa.0.07965.i, !dbg !9305
  %745 = fadd <8 x float> %744, %737, !dbg !9310
  %746 = fmul <8 x float> %_129.i.i.i.i469.sroa.0.0.copyload.i, %history.i.i611.sroa.29.sroa.0.07965.i, !dbg !9315
  %747 = fadd <8 x float> %746, %739, !dbg !9320
  %748 = fmul <8 x float> %_132.i.i.i.i466.sroa.0.0.copyload.i, %history.i.i611.sroa.29.sroa.0.07965.i, !dbg !9325
  %749 = fadd <8 x float> %748, %741, !dbg !9330
  %750 = fmul <8 x float> %_137.i.i.i.i462.sroa.0.0.copyload.i, %history.i.i611.sroa.32.sroa.0.07966.i, !dbg !9335
  %751 = fadd <8 x float> %750, %743, !dbg !9340
  %752 = fmul <8 x float> %_140.i.i.i.i459.sroa.0.0.copyload.i, %history.i.i611.sroa.32.sroa.0.07966.i, !dbg !9345
  %753 = fadd <8 x float> %752, %745, !dbg !9350
  %754 = fmul <8 x float> %_143.i.i.i.i456.sroa.0.0.copyload.i, %history.i.i611.sroa.32.sroa.0.07966.i, !dbg !9355
  %755 = fadd <8 x float> %754, %747, !dbg !9360
  %756 = fmul <8 x float> %_146.i.i.i.i453.sroa.0.0.copyload.i, %history.i.i611.sroa.32.sroa.0.07966.i, !dbg !9365
  %757 = fadd <8 x float> %756, %749, !dbg !9370
  %758 = fmul <8 x float> %_151.i.i.i.i449.sroa.0.0.copyload.i, %history.i.i611.sroa.35.sroa.0.07967.i, !dbg !9375
  %759 = fadd <8 x float> %758, %751, !dbg !9380
  %760 = fmul <8 x float> %_154.i.i.i.i446.sroa.0.0.copyload.i, %history.i.i611.sroa.35.sroa.0.07967.i, !dbg !9385
  %761 = fadd <8 x float> %760, %753, !dbg !9390
  %762 = fmul <8 x float> %_157.i.i.i.i443.sroa.0.0.copyload.i, %history.i.i611.sroa.35.sroa.0.07967.i, !dbg !9395
  %763 = fadd <8 x float> %762, %755, !dbg !9400
  %764 = fmul <8 x float> %_160.i.i.i.i440.sroa.0.0.copyload.i, %history.i.i611.sroa.35.sroa.0.07967.i, !dbg !9405
  %765 = fadd <8 x float> %764, %757, !dbg !9410
  %766 = fmul <8 x float> %_165.i.i.i.i436.sroa.0.0.copyload.i, %history.i.i611.sroa.38.sroa.0.07968.i, !dbg !9415
  %767 = fadd <8 x float> %766, %759, !dbg !9420
  %768 = fmul <8 x float> %_168.i.i.i.i433.sroa.0.0.copyload.i, %history.i.i611.sroa.38.sroa.0.07968.i, !dbg !9425
  %769 = fadd <8 x float> %768, %761, !dbg !9430
  %770 = fmul <8 x float> %_171.i.i.i.i430.sroa.0.0.copyload.i, %history.i.i611.sroa.38.sroa.0.07968.i, !dbg !9435
  %771 = fadd <8 x float> %770, %763, !dbg !9440
  %772 = fmul <8 x float> %_174.i.i.i.i427.sroa.0.0.copyload.i, %history.i.i611.sroa.38.sroa.0.07968.i, !dbg !9445
  %773 = fadd <8 x float> %772, %765, !dbg !9450
  %774 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %767), !dbg !9455
  %775 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %677, <8 x float> %774), !dbg !9461
  %776 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %769), !dbg !9455
  %777 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %775, <8 x float> %776), !dbg !9461
  %778 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %771), !dbg !9455
  %779 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %777, <8 x float> %778), !dbg !9461
  %780 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %773), !dbg !9455
  %781 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %779, <8 x float> %780), !dbg !9461
  %_39.i.i704.idx.i = shl i64 %iter.sroa.0.0.i.i6597974.i, 5, !dbg !9466
  %_39.i.i704.i = getelementptr inbounds nuw i8, ptr %peaks_left.i635.i, i64 %_39.i.i704.idx.i, !dbg !9466
  store <8 x float> %781, ptr %_39.i.i704.i, align 4, !dbg !9471, !alias.scope !9476, !noalias !9480
  %exitcond9600.not.i = icmp eq i64 %676, %umax9613.i, !dbg !8926
  br i1 %exitcond9600.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i661.i, label %bb5.i.i678.i, !dbg !8930

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i661.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i, %bb29.i.i
  %history.i.i611.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.0.0.lcssa80448068.i, %bb29.i.i ], [ %lanes.i3142.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.25.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.25.sroa.0.0.copyload.i, %bb29.i.i ], [ %history.i.i611.sroa.22.sroa.0.07969.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.29.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.29.sroa.0.0.copyload.i, %bb29.i.i ], [ %history.i.i611.sroa.25.sroa.0.07964.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.32.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.32.sroa.0.0.copyload.i, %bb29.i.i ], [ %history.i.i611.sroa.29.sroa.0.07965.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.35.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.35.sroa.0.0.copyload.i, %bb29.i.i ], [ %history.i.i611.sroa.32.sroa.0.07966.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.38.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.38.sroa.0.0.copyload.i, %bb29.i.i ], [ %history.i.i611.sroa.35.sroa.0.07967.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.41.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.41.sroa.0.0.copyload.i, %bb29.i.i ], [ %history.i.i611.sroa.38.sroa.0.07968.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.22.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.22.sroa.0.0.copyload.i, %bb29.i.i ], [ %history.i.i611.sroa.19.sroa.0.07970.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.19.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.19.sroa.0.0.copyload.i, %bb29.i.i ], [ %history.i.i611.sroa.16.sroa.0.07971.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.16.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.16.sroa.0.0.lcssa10790.i, %bb29.i.i ], [ %history.i.i611.sroa.13.sroa.0.07972.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.13.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.13.sroa.0.0.lcssa10771.i, %bb29.i.i ], [ %history.i.i611.sroa.10.sroa.0.07973.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  %history.i.i611.sroa.10.sroa.0.0.lcssa.i = phi <8 x float> [ %history.i.i611.sroa.10.sroa.0.0.lcssa10752.i, %bb29.i.i ], [ %history.i.i611.sroa.0.07963.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3532.i ], !dbg !9484
  store <8 x float> %history.i.i611.sroa.29.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.29.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8944, !noalias !6249
  store <8 x float> %history.i.i611.sroa.32.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.32.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8944, !noalias !6249
  store <8 x float> %history.i.i611.sroa.35.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.35.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8944, !noalias !6249
  store <8 x float> %history.i.i611.sroa.38.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.38.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8944, !noalias !6249
  store <8 x float> %history.i.i611.sroa.41.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.41.0.hot_left.i641.sroa_idx.i, align 32, !dbg !8944, !noalias !6249
  br i1 %_20.i.i6607962.not.i, label %bb12.i651.loopexit.i, label %bb32.i.lr.ph.i, !dbg !8913

bb32.i.lr.ph.i:                                   ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i661.i
  %_60.i.i = load i64, ptr %655, align 8, !alias.scope !5901, !noalias !5902
  %_67.i675.i = load i64, ptr %672, align 8, !alias.scope !5901, !noalias !5902
  br label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3190.i, !dbg !8913

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3190.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3537.i, %bb32.i.lr.ph.i
  %_58.i.i.sroa.0.0.copyload8005.i = phi <8 x float> [ %.lcssa1039510811.i, %bb32.i.lr.ph.i ], [ %816, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3537.i ]
  %main_cursor.sroa.0.1.i6648001.i = phi i64 [ %main_cursor.sroa.0.0.i6548072.i, %bb32.i.lr.ph.i ], [ %spec.store.select.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3537.i ]
  %ring_cursor.sroa.0.1.i6638000.i = phi i64 [ %ring_cursor.sroa.0.0.i6538071.i, %bb32.i.lr.ph.i ], [ %spec.store.select9.i.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3537.i ]
  %iter2.sroa.0.0.i6627999.i = phi i64 [ 0, %bb32.i.lr.ph.i ], [ %782, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3537.i ]
  %782 = add nuw nsw i64 %iter2.sroa.0.0.i6627999.i, 1, !dbg !9485
  %_43.i.i = add nuw nsw i64 %iter2.sroa.0.0.i6627999.i, %iter.sroa.0.0.i8069.i, !dbg !9491
  %base.i665.i = shl i64 %_43.i.i, 3, !dbg !9491
  %_92.i668.idx.i = shl i64 %iter2.sroa.0.0.i6627999.i, 5, !dbg !9492
  %_92.i668.i = getelementptr inbounds nuw i8, ptr %peaks_left.i635.i, i64 %_92.i668.idx.i, !dbg !9492
  %lanes.i3183.sroa.0.0.copyload.i = load <8 x float>, ptr %_92.i668.i, align 4, !dbg !9503, !alias.scope !9508, !noalias !9512
  %783 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i3183.sroa.0.0.copyload.i, <8 x float> %lanes.i3183.sroa.0.0.copyload.i), !dbg !9516
  %784 = select <8 x i1> %654, <8 x float> %783, <8 x float> %lanes.i3183.sroa.0.0.copyload.i, !dbg !9521
  %_93.i.i = icmp samesign ugt i64 %base.i665.i, %_39.1, !dbg !9526
  br i1 %_93.i.i, label %bb36.i677.i, label %bb37.i.i, !dbg !9526, !prof !639

bb37.i.i:                                         ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3190.i
  %_96.i.i = sub nuw nsw i64 %_39.1, %base.i665.i, !dbg !9530
  %_100.i.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i665.i, !dbg !9531
  %_8.i3177.i = icmp samesign ugt i64 %_96.i.i, 7, !dbg !9536
  br i1 %_8.i3177.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3181.i, label %bb2.i3178.i, !dbg !9536, !prof !651

bb2.i3178.i:                                      ; preds = %bb37.i.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_96.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !9541, !noalias !9542
  unreachable, !dbg !9541

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3181.i: ; preds = %bb37.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9546), !dbg !9549
  %width.i.i.i = load i64, ptr %656, align 8, !dbg !9550, !alias.scope !9551, !noalias !9552, !noundef !12
  %785 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %784, <8 x float> %_8.i.i627.sroa.0.0.copyload.i, i8 30), !dbg !9561
  %786 = fdiv <8 x float> %_8.i.i627.sroa.0.0.copyload.i, %784, !dbg !9567
  %787 = bitcast <8 x float> %785 to <8 x i32>, !dbg !9572
  %788 = icmp slt <8 x i32> %787, zeroinitializer, !dbg !9576
  %789 = select <8 x i1> %788, <8 x float> %786, <8 x float> splat (float 1.000000e+00), !dbg !9576
  %_144.1.i.i.i = load i64, ptr %657, align 8, !dbg !9578, !alias.scope !9551, !noalias !9552, !noundef !12
  %_22.i.i669.i = mul i64 %width.i.i.i, %ring_cursor.sroa.0.1.i6638000.i, !dbg !9579
  %_92.i.i.i = icmp ugt i64 %_22.i.i669.i, %_144.1.i.i.i, !dbg !9580
  br i1 %_92.i.i.i, label %bb37.i.i.i, label %bb38.i.i.i, !dbg !9580, !prof !639

bb38.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3181.i
  %_95.i.i.i = sub nuw i64 %_144.1.i.i.i, %_22.i.i669.i, !dbg !9583
  %_8.i3549.i = icmp samesign ugt i64 %_95.i.i.i, 7, !dbg !9584
  br i1 %_8.i3549.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3552.i, label %bb2.i3550.i, !dbg !9584, !prof !651

bb2.i3550.i:                                      ; preds = %bb38.i.i.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_95.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !9589, !noalias !9590
  unreachable, !dbg !9589

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3552.i: ; preds = %bb38.i.i.i
  %_144.0.i.i.i = load ptr, ptr %658, align 8, !dbg !9578, !alias.scope !9551, !noalias !9552, !nonnull !12, !noundef !12
  %_99.i.i.i = getelementptr inbounds nuw float, ptr %_144.0.i.i.i, i64 %_22.i.i669.i, !dbg !9594
  store <8 x float> %789, ptr %_99.i.i.i, align 4, !dbg !9596, !alias.scope !9600, !noalias !9604
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9606), !dbg !9609
  %width.i1239.i = load i64, ptr %656, align 8, !dbg !9610, !alias.scope !9612, !noalias !9613, !noundef !12
  %790 = icmp eq i64 %width.i1239.i, 0, !dbg !9615
  br i1 %790, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1347.i, label %bb32.i1246.lr.ph.i, !dbg !9615

bb32.i1246.lr.ph.i:                               ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3552.i
  %_112.1.i1249.i = load i64, ptr %49, align 8, !alias.scope !9612, !noalias !9613, !noundef !12
  %_112.0.i1253.i = load ptr, ptr %48, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %791 = add i64 %ring_cursor.sroa.0.1.i6638000.i, 1
  %_23.not.i1260.i = icmp ult i64 %791, %_60.i.i
  %792 = select i1 %_23.not.i1260.i, i64 0, i64 %_60.i.i
  %start1.sroa.0.0.i1261.i = sub nuw i64 %791, %792
  %_114.1.i1264.i = load i64, ptr %657, align 8, !alias.scope !5901, !noalias !5902
  %_114.0.i1268.i = load ptr, ptr %658, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %_116.1.i1269.i = load i64, ptr %659, align 8, !alias.scope !5901, !noalias !5902
  %_116.0.i1273.i = load ptr, ptr %660, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %_118.1.i1277.i = load i64, ptr %661, align 8, !alias.scope !5901, !noalias !5902
  %_118.0.i1281.i = load ptr, ptr %662, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %_45.i1294.i = mul i64 %width.i1239.i, %start1.sroa.0.0.i1261.i
  br label %bb32.i1246.i, !dbg !9615

bb32.i1246.i:                                     ; preds = %bb31.i1309.i, %bb32.i1246.lr.ph.i
  %iter.i1238.sroa.10.07992.i = phi i64 [ %width.i1239.i, %bb32.i1246.lr.ph.i ], [ %793, %bb31.i1309.i ]
  %iter.i1238.sroa.7.07991.i = phi i64 [ 0, %bb32.i1246.lr.ph.i ], [ %_9.0.i3948.i, %bb31.i1309.i ]
  %iter.i1238.sroa.0.0.idx7990.i = phi i64 [ 0, %bb32.i1246.lr.ph.i ], [ %iter.i1238.sroa.0.0.add.i, %bb31.i1309.i ]
  %iter.i1238.sroa.0.0.ptr7993.i = getelementptr inbounds nuw i8, ptr %scratch.i.i, i64 %iter.i1238.sroa.0.0.idx7990.i, !dbg !9617
  %793 = add i64 %iter.i1238.sroa.10.07992.i, -1, !dbg !9617
  %_7.i.i3944.i = icmp eq i64 %iter.i1238.sroa.0.0.idx7990.i, 32, !dbg !9618
  br i1 %_7.i.i3944.i, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1347.i, label %bb3.i1248.i, !dbg !9622

bb3.i1248.i:                                      ; preds = %bb32.i1246.i
  %iter.i1238.sroa.0.0.add.i = add nuw nsw i64 %iter.i1238.sroa.0.0.idx7990.i, 4, !dbg !9623
  %_9.0.i3948.i = add nuw nsw i64 %iter.i1238.sroa.7.07991.i, 1, !dbg !9625
  %exitcond9604.not.i = icmp eq i64 %iter.i1238.sroa.7.07991.i, %_112.1.i1249.i, !dbg !9626
  br i1 %exitcond9604.not.i, label %panic.i1251.i, label %bb5.i1252.i, !dbg !9626

bb5.i1252.i:                                      ; preds = %bb3.i1248.i
  %794 = getelementptr inbounds nuw %LaneShape, ptr %_112.0.i1253.i, i64 %iter.i1238.sroa.7.07991.i, !dbg !9626
  %shape.i1254.i = load i32, ptr %794, align 4, !dbg !9626, !noalias !9627, !noundef !12
  %795 = getelementptr inbounds nuw i8, ptr %794, i64 4, !dbg !9626
  %shape3.i1255.i = load i32, ptr %795, align 4, !dbg !9626, !noalias !9627, !noundef !12
  %window.i1256.i = zext i32 %shape.i1254.i to i64, !dbg !9628
  %_19.i1257.i = zext i32 %shape3.i1255.i to i64, !dbg !9629
  %796 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_19.i1257.i, !dbg !9630
  %_20.not.i1258.i = icmp ult i64 %796, %_60.i.i, !dbg !9631
  %797 = select i1 %_20.not.i1258.i, i64 0, i64 %_60.i.i, !dbg !9631
  %spec.select.i1259.i = sub nuw i64 %796, %797, !dbg !9631
  %_27.i1262.i = mul i64 %spec.select.i1259.i, %width.i1239.i, !dbg !9632
  %_26.i1263.i = add i64 %_27.i1262.i, %iter.i1238.sroa.7.07991.i, !dbg !9632
  %_30.i1265.i = icmp ult i64 %_26.i1263.i, %_114.1.i1264.i, !dbg !9633
  br i1 %_30.i1265.i, label %bb12.i1267.i, label %panic5.i1266.i, !dbg !9633

panic.i1251.i:                                    ; preds = %bb3.i1248.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_112.1.i1249.i, i64 noundef %_112.1.i1249.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8013bf8450ffb218032f1e27d334efa7) #30, !dbg !9626, !noalias !9627
  unreachable, !dbg !9626

bb12.i1267.i:                                     ; preds = %bb5.i1252.i
  %798 = getelementptr inbounds nuw float, ptr %_114.0.i1268.i, i64 %_26.i1263.i, !dbg !9633
  %799 = load float, ptr %798, align 4, !dbg !9633, !noalias !9627, !noundef !12
  %exitcond9605.not.i = icmp eq i64 %iter.i1238.sroa.7.07991.i, %_116.1.i1269.i, !dbg !9634
  br i1 %exitcond9605.not.i, label %panic6.i1271.i, label %bb13.i1272.i, !dbg !9634

panic5.i1266.i:                                   ; preds = %bb5.i1252.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_26.i1263.i, i64 noundef %_114.1.i1264.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d297cbdfce2474defb1d7f11394e8cb6) #30, !dbg !9633, !noalias !9627
  unreachable, !dbg !9633

bb13.i1272.i:                                     ; preds = %bb12.i1267.i
  %800 = getelementptr inbounds nuw i32, ptr %_116.0.i1273.i, i64 %iter.i1238.sroa.7.07991.i, !dbg !9634
  %_32.i1274.i = load i32, ptr %800, align 4, !dbg !9634, !noalias !9627, !noundef !12
  %position.i1275.i = zext i32 %_32.i1274.i to i64, !dbg !9634
  %801 = icmp eq i32 %_32.i1274.i, 0, !dbg !9635
  br i1 %801, label %bb17.i1284.i, label %bb15.i1276.i, !dbg !9635

panic6.i1271.i:                                   ; preds = %bb12.i1267.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_116.1.i1269.i, i64 noundef %_116.1.i1269.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ac016620dad255f0d323022a5b7f5883) #30, !dbg !9634, !noalias !9627
  unreachable, !dbg !9634

bb15.i1276.i:                                     ; preds = %bb13.i1272.i
  %_37.i1278.i = icmp ult i64 %iter.i1238.sroa.7.07991.i, %_118.1.i1277.i, !dbg !9636
  br i1 %_37.i1278.i, label %bb16.i1280.i, label %panic7.i1279.i, !dbg !9636

bb17.i1284.i:                                     ; preds = %bb35.i1345.i, %bb16.i1280.i, %bb13.i1272.i
  %newest.sroa.0.0.i1285.i = phi float [ %799, %bb13.i1272.i ], [ %_35.i1282.i, %bb35.i1345.i ], [ %799, %bb16.i1280.i ], !dbg !9637
  %exitcond9606.not.i = icmp eq i64 %iter.i1238.sroa.7.07991.i, %_118.1.i1277.i, !dbg !9638
  br i1 %exitcond9606.not.i, label %panic8.i1288.i, label %bb18.i1289.i, !dbg !9638

bb16.i1280.i:                                     ; preds = %bb15.i1276.i
  %802 = getelementptr inbounds nuw float, ptr %_118.0.i1281.i, i64 %iter.i1238.sroa.7.07991.i, !dbg !9636
  %_35.i1282.i = load float, ptr %802, align 4, !dbg !9636, !noalias !9627, !noundef !12
  %_102.i1283.i = fcmp olt float %_35.i1282.i, %799, !dbg !9639
  br i1 %_102.i1283.i, label %bb35.i1345.i, label %bb17.i1284.i, !dbg !9639

panic7.i1279.i:                                   ; preds = %bb15.i1276.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %iter.i1238.sroa.7.07991.i, i64 noundef %_118.1.i1277.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5355958980a78279123102c3494b10a) #30, !dbg !9636, !noalias !9627
  unreachable, !dbg !9636

bb35.i1345.i:                                     ; preds = %bb16.i1280.i
  br label %bb17.i1284.i, !dbg !9641

bb18.i1289.i:                                     ; preds = %bb17.i1284.i
  %803 = getelementptr inbounds nuw float, ptr %_118.0.i1281.i, i64 %iter.i1238.sroa.7.07991.i, !dbg !9638
  store float %newest.sroa.0.0.i1285.i, ptr %803, align 4, !dbg !9638, !noalias !9627
  %_42.i1291.i = add nuw nsw i64 %position.i1275.i, 1, !dbg !9642
  %complete.i1292.i = icmp eq i64 %_42.i1291.i, %window.i1256.i, !dbg !9642
  br i1 %complete.i1292.i, label %bb22.i1314.i, label %bb20.i1293.i, !dbg !9643

panic8.i1288.i:                                   ; preds = %bb17.i1284.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_118.1.i1277.i, i64 noundef %_118.1.i1277.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2d2a28b8cb03afaaebfea48ffa77c925) #30, !dbg !9638, !noalias !9627
  unreachable, !dbg !9638

bb20.i1293.i:                                     ; preds = %bb18.i1289.i
  %_44.i1295.i = add i64 %iter.i1238.sroa.7.07991.i, %_45.i1294.i, !dbg !9644
  %_47.i1297.i = icmp ult i64 %_44.i1295.i, %_114.1.i1264.i, !dbg !9645
  br i1 %_47.i1297.i, label %bb30.i1307.i, label %panic9.i1298.i, !dbg !9645

panic9.i1298.i:                                   ; preds = %bb20.i1293.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_44.i1295.i, i64 noundef %_114.1.i1264.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_53d3a5c3ecf31ea5f14c0a7f8bf40921) #30, !dbg !9645, !noalias !9627
  unreachable, !dbg !9645

bb30.i1307.i:                                     ; preds = %bb20.i1293.i
  %804 = getelementptr inbounds nuw float, ptr %_114.0.i1268.i, i64 %_44.i1295.i, !dbg !9645
  %_43.i1301.i = load float, ptr %804, align 4, !dbg !9645, !noalias !9627, !noundef !12
  %_103.i1302.i = fcmp olt float %_43.i1301.i, %newest.sroa.0.0.i1285.i, !dbg !9646
  %newest.sroa.0.1.i1303.i = select i1 %_103.i1302.i, float %_43.i1301.i, float %newest.sroa.0.0.i1285.i, !dbg !9646
  store float %newest.sroa.0.1.i1303.i, ptr %iter.i1238.sroa.0.0.ptr7993.i, align 4, !dbg !9648, !noalias !9649
  %805 = trunc i64 %_42.i1291.i to i32, !dbg !9650
  br label %bb31.i1309.i, !dbg !9651

bb31.i1309.i:                                     ; preds = %bb25.i1342.i, %bb30.i1307.i
  %storemerge6797.i = phi i32 [ %805, %bb30.i1307.i ], [ 0, %bb25.i1342.i ], !dbg !9652
  store i32 %storemerge6797.i, ptr %800, align 4, !dbg !9652, !noalias !9627
  %806 = icmp eq i64 %793, 0, !dbg !9615
  br i1 %806, label %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1347.i, label %bb32.i1246.i, !dbg !9615

bb22.i1314.i:                                     ; preds = %bb18.i1289.i
  store float %newest.sroa.0.0.i1285.i, ptr %iter.i1238.sroa.0.0.ptr7993.i, align 4, !dbg !9648, !noalias !9649
  %807 = load float, ptr %798, align 4, !dbg !9653, !noalias !9627, !noundef !12
  br label %bb41.i1327.i, !dbg !9654

bb41.i1327.i:                                     ; preds = %bb25.i1342.i, %bb22.i1314.i
  %iter2.sroa.0.0.i13197989.i = phi i64 [ 0, %bb22.i1314.i ], [ %_105.i1328.i, %bb25.i1342.i ]
  %suffix.sroa.0.0.i13187988.i = phi float [ %807, %bb22.i1314.i ], [ %suffix.sroa.0.1.i1338.i, %bb25.i1342.i ]
  %end.sroa.0.1.i13177987.i = phi i64 [ %spec.select.i1259.i, %bb22.i1314.i ], [ %810, %bb25.i1342.i ]
  %_56.i1329.i = mul i64 %end.sroa.0.1.i13177987.i, %width.i1239.i, !dbg !9657
  %_55.i1330.i = add i64 %_56.i1329.i, %iter.i1238.sroa.7.07991.i, !dbg !9657
  %_59.i1332.i = icmp ult i64 %_55.i1330.i, %_114.1.i1264.i, !dbg !9658
  br i1 %_59.i1332.i, label %bb25.i1342.i, label %panic13.i1333.i, !dbg !9658

panic13.i1333.i:                                  ; preds = %bb41.i1327.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_55.i1330.i, i64 noundef %_114.1.i1264.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b22b5c926aed02a79660ac772e5ad40d) #30, !dbg !9658, !noalias !9627
  unreachable, !dbg !9658

bb25.i1342.i:                                     ; preds = %bb41.i1327.i
  %_105.i1328.i = add nuw nsw i64 %iter2.sroa.0.0.i13197989.i, 1, !dbg !9659
  %808 = getelementptr inbounds nuw float, ptr %_114.0.i1268.i, i64 %_55.i1330.i, !dbg !9658
  %_54.i1336.i = load float, ptr %808, align 4, !dbg !9658, !noalias !9627, !noundef !12
  %_107.i1337.i = fcmp olt float %suffix.sroa.0.0.i13187988.i, %_54.i1336.i, !dbg !9662
  %suffix.sroa.0.1.i1338.i = select i1 %_107.i1337.i, float %suffix.sroa.0.0.i13187988.i, float %_54.i1336.i, !dbg !9662
  store float %suffix.sroa.0.1.i1338.i, ptr %808, align 4, !dbg !9664, !noalias !9627
  %809 = icmp eq i64 %end.sroa.0.1.i13177987.i, 0, !dbg !9665
  %spec.store.select.i1344.i = select i1 %809, i64 %_60.i.i, i64 %end.sroa.0.1.i13177987.i, !dbg !9665
  %810 = add i64 %spec.store.select.i1344.i, -1, !dbg !9666
  %exitcond9603.not.i = icmp eq i64 %_105.i1328.i, %window.i1256.i, !dbg !9667
  br i1 %exitcond9603.not.i, label %bb31.i1309.i, label %bb41.i1327.i, !dbg !9654

_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1347.i: ; preds = %bb31.i1309.i, %bb32.i1246.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3552.i
  %lanes.i3167.sroa.0.0.copyload.i = load <8 x float>, ptr %scratch.i.i, align 4, !dbg !9669, !alias.scope !9674, !noalias !9678
  %811 = fmul <8 x float> %lanes.i3167.sroa.0.0.copyload.i, splat (float 1.638400e+04), !dbg !9682
  %812 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %811), !dbg !9687
  %813 = fmul <8 x float> %812, splat (float 0x3F10000000000000), !dbg !9692
  %814 = icmp eq i64 %width.i.i.i, 0, !dbg !9697
  %_149.1.i.i.pre.i = load i64, ptr %663, align 8, !dbg !9699, !alias.scope !9551, !noalias !9552
  br i1 %814, label %bb16.i.i.i, label %bb39.i.i.lr.ph.i, !dbg !9697

bb39.i.i.lr.ph.i:                                 ; preds = %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1347.i
  %_145.1.i.i.i = load i64, ptr %49, align 8, !alias.scope !9551, !noalias !9552, !noundef !12
  %_145.0.i.i.i = load ptr, ptr %48, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %_147.0.i.i.i = load ptr, ptr %664, align 8, !alias.scope !5901, !noalias !5902, !nonnull !12
  %exitcond9609.not.i = icmp eq i64 %_145.1.i.i.i, 0, !dbg !9700
  br i1 %exitcond9609.not.i, label %panic.i.i.i, label %bb17.i.i.i, !dbg !9700

bb37.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3181.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i669.i, i64 noundef %_144.1.i.i.i, i64 noundef %_144.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_913d17a5751fc2956adecdab98dac09f) #30, !dbg !9701, !noalias !9702
  unreachable, !dbg !9701

bb16.i.i.loopexit.i:                              ; preds = %bb21.i.i.7.i, %bb21.i.i.6.i, %bb21.i.i.5.i, %bb21.i.i.4.i, %bb21.i.i.3.i, %bb21.i.i.2.i, %bb21.i.i.1.i, %bb21.i.i.i
  %lanes.i3160.sroa.0.0.copyload.pre.i = load <8 x float>, ptr %scratch.i.i, align 4, !dbg !9703, !alias.scope !9708, !noalias !9712
  br label %bb16.i.i.i, !dbg !9716

bb16.i.i.i:                                       ; preds = %bb16.i.i.loopexit.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1347.i
  %lanes.i3160.sroa.0.0.copyload.i = phi <8 x float> [ %lanes.i3160.sroa.0.0.copyload.pre.i, %bb16.i.i.loopexit.i ], [ %lanes.i3167.sroa.0.0.copyload.i, %_RNvCsdvPQf9CMsz3_17true_peak_limiter15sliding_minimum.exit1347.i ], !dbg !9703
  %815 = fadd <8 x float> %_58.i.i.sroa.0.0.copyload8005.i, %813, !dbg !9717
  %816 = fsub <8 x float> %815, %lanes.i3160.sroa.0.0.copyload.i, !dbg !9722
  %_109.i.i.i = icmp ugt i64 %_22.i.i669.i, %_149.1.i.i.pre.i, !dbg !9727
  br i1 %_109.i.i.i, label %bb42.i.i.i, label %bb43.i.i.i, !dbg !9727, !prof !639

bb43.i.i.i:                                       ; preds = %bb16.i.i.i
  %_112.i.i.i = sub nuw i64 %_149.1.i.i.pre.i, %_22.i.i669.i, !dbg !9730
  %_8.i3544.i = icmp samesign ugt i64 %_112.i.i.i, 7, !dbg !9731
  br i1 %_8.i3544.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3547.i, label %bb2.i3545.i, !dbg !9731, !prof !651

bb2.i3545.i:                                      ; preds = %bb43.i.i.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_112.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !9736, !noalias !9737
  unreachable, !dbg !9736

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3547.i: ; preds = %bb43.i.i.i
  %_149.0.i.i.i = load ptr, ptr %664, align 8, !dbg !9699, !alias.scope !9551, !noalias !9552, !nonnull !12, !noundef !12
  %_116.i.i.i = getelementptr inbounds nuw float, ptr %_149.0.i.i.i, i64 %_22.i.i669.i, !dbg !9741
  store <8 x float> %813, ptr %_116.i.i.i, align 4, !dbg !9743, !alias.scope !9747, !noalias !9751
  %_68.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %667, align 32, !dbg !9753, !noalias !6249
  %817 = fdiv <8 x float> %816, %_64.i.i.sroa.0.0.copyload.i, !dbg !9754
  %818 = fsub <8 x float> splat (float 1.000000e+00), %817, !dbg !9759
  %819 = fsub <8 x float> %818, %_68.i.i.sroa.0.0.copyload.i, !dbg !9764
  %820 = fmul <8 x float> %_9.i.i626.sroa.0.0.copyload.i, %819, !dbg !9769
  %821 = fadd <8 x float> %_68.i.i.sroa.0.0.copyload.i, %820, !dbg !9774
  %822 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %818, <8 x float> %821), !dbg !9778
  %823 = bitcast <8 x float> %822 to <8 x i32>, !dbg !9783
  %824 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %822), !dbg !9789
  %825 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %824, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !9791
  %826 = bitcast <8 x float> %825 to <8 x i32>, !dbg !9797
  %827 = xor <8 x i32> %826, splat (i32 -1), !dbg !9803
  %828 = and <8 x i32> %827, %823, !dbg !9805
  %829 = bitcast <8 x i32> %828 to <8 x float>, !dbg !9809
  store <8 x i32> %828, ptr %667, align 32, !dbg !9810, !noalias !6249
  %830 = fsub <8 x float> splat (float 1.000000e+00), %829, !dbg !9811
  %_150.1.i.i.i = load i64, ptr %668, align 8, !dbg !9816, !alias.scope !9551, !noalias !9552, !noundef !12
  %_76.i.i.i = mul i64 %width.i.i.i, %main_cursor.sroa.0.1.i6648001.i, !dbg !9817
  %_120.i.i.i = icmp ugt i64 %_76.i.i.i, %_150.1.i.i.i, !dbg !9818
  br i1 %_120.i.i.i, label %bb48.i.i.i, label %bb49.i.i.i, !dbg !9818, !prof !639

bb42.i.i.i:                                       ; preds = %bb16.i.i.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_22.i.i669.i, i64 noundef %_149.1.i.i.pre.i, i64 noundef %_149.1.i.i.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a0dcf875eae79f6708bcf79c3cbff55) #30, !dbg !9821, !noalias !9822
  unreachable, !dbg !9821

bb49.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3547.i
  %_123.i.i.i = sub nuw i64 %_150.1.i.i.i, %_76.i.i.i, !dbg !9823
  %_8.i3154.i = icmp samesign ugt i64 %_123.i.i.i, 7, !dbg !9824
  br i1 %_8.i3154.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3537.i, label %bb2.i3155.i, !dbg !9824, !prof !651

bb2.i3155.i:                                      ; preds = %bb49.i.i.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_123.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !9829, !noalias !9830
  unreachable, !dbg !9829

bb48.i.i.i:                                       ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3547.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %_76.i.i.i, i64 noundef %_150.1.i.i.i, i64 noundef %_150.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5e0b8406fbb9ac3f5f26ac6469c25f7) #30, !dbg !9834, !noalias !9822
  unreachable, !dbg !9834

bb17.i.i.i:                                       ; preds = %bb39.i.i.lr.ph.i
  %831 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 8, !dbg !9700
  %_44.i.i671.i = load i32, ptr %831, align 4, !dbg !9700, !noalias !9822, !noundef !12
  %_43.i.i672.i = zext i32 %_44.i.i671.i to i64, !dbg !9700
  %832 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_43.i.i672.i, !dbg !9835
  %_47.not.i.i.i = icmp ult i64 %832, %_60.i.i, !dbg !9836
  %833 = select i1 %_47.not.i.i.i, i64 0, i64 %_60.i.i, !dbg !9836
  %spec.select.i.i.i = sub nuw i64 %832, %833, !dbg !9836
  %_51.i.i.i = mul i64 %spec.select.i.i.i, %width.i.i.i, !dbg !9837
  %_53.i.i673.i = icmp ult i64 %_51.i.i.i, %_149.1.i.i.pre.i, !dbg !9838
  br i1 %_53.i.i673.i, label %bb21.i.i.i, label %panic1.i.i.i, !dbg !9838

panic.i.i.i:                                      ; preds = %bb39.i.i.7.i, %bb39.i.i.6.i, %bb39.i.i.5.i, %bb39.i.i.4.i, %bb39.i.i.3.i, %bb39.i.i.2.i, %bb39.i.i.1.i, %bb39.i.i.lr.ph.i
  %_145.1.i.i.lcssa.ph.i = phi i64 [ 7, %bb39.i.i.7.i ], [ 6, %bb39.i.i.6.i ], [ 5, %bb39.i.i.5.i ], [ 4, %bb39.i.i.4.i ], [ 3, %bb39.i.i.3.i ], [ 2, %bb39.i.i.2.i ], [ 1, %bb39.i.i.1.i ], [ 0, %bb39.i.i.lr.ph.i ]
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_145.1.i.i.lcssa.ph.i, i64 noundef %_145.1.i.i.lcssa.ph.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ff2ed40a8d2df224a47a35441e5e2fe) #30, !dbg !9700, !noalias !9822
  unreachable, !dbg !9700

bb21.i.i.i:                                       ; preds = %bb17.i.i.i
  %834 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_51.i.i.i, !dbg !9838
  %_49.i.i674.i = load float, ptr %834, align 4, !dbg !9838, !noalias !9822, !noundef !12
  store float %_49.i.i674.i, ptr %scratch.i.i, align 4, !dbg !9839, !noalias !9840
  %835 = icmp eq i64 %width.i.i.i, 1, !dbg !9697
  br i1 %835, label %bb16.i.i.loopexit.i, label %bb39.i.i.1.i, !dbg !9697

bb39.i.i.1.i:                                     ; preds = %bb21.i.i.i
  %exitcond9609.1.not.i = icmp eq i64 %_145.1.i.i.i, 1, !dbg !9700
  br i1 %exitcond9609.1.not.i, label %panic.i.i.i, label %bb17.i.i.1.i, !dbg !9700

bb17.i.i.1.i:                                     ; preds = %bb39.i.i.1.i
  %836 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 20, !dbg !9700
  %_44.i.i671.1.i = load i32, ptr %836, align 4, !dbg !9700, !noalias !9822, !noundef !12
  %_43.i.i672.1.i = zext i32 %_44.i.i671.1.i to i64, !dbg !9700
  %837 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_43.i.i672.1.i, !dbg !9835
  %_47.not.i.i.1.i = icmp ult i64 %837, %_60.i.i, !dbg !9836
  %838 = select i1 %_47.not.i.i.1.i, i64 0, i64 %_60.i.i, !dbg !9836
  %spec.select.i.i.1.i = sub nuw i64 %837, %838, !dbg !9836
  %_51.i.i.1.i = mul i64 %spec.select.i.i.1.i, %width.i.i.i, !dbg !9837
  %_50.i.i.1.i = add i64 %_51.i.i.1.i, 1, !dbg !9837
  %_53.i.i673.1.i = icmp ult i64 %_50.i.i.1.i, %_149.1.i.i.pre.i, !dbg !9838
  br i1 %_53.i.i673.1.i, label %bb21.i.i.1.i, label %panic1.i.i.i, !dbg !9838

bb21.i.i.1.i:                                     ; preds = %bb17.i.i.1.i
  %839 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.1.i, !dbg !9838
  %_49.i.i674.1.i = load float, ptr %839, align 4, !dbg !9838, !noalias !9822, !noundef !12
  store float %_49.i.i674.1.i, ptr %iter.i.i.sroa.0.0.ptr7997.1.i, align 4, !dbg !9839, !noalias !9840
  %840 = icmp eq i64 %width.i.i.i, 2, !dbg !9697
  br i1 %840, label %bb16.i.i.loopexit.i, label %bb39.i.i.2.i, !dbg !9697

bb39.i.i.2.i:                                     ; preds = %bb21.i.i.1.i
  %exitcond9609.2.not.i = icmp eq i64 %_145.1.i.i.i, 2, !dbg !9700
  br i1 %exitcond9609.2.not.i, label %panic.i.i.i, label %bb17.i.i.2.i, !dbg !9700

bb17.i.i.2.i:                                     ; preds = %bb39.i.i.2.i
  %841 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 32, !dbg !9700
  %_44.i.i671.2.i = load i32, ptr %841, align 4, !dbg !9700, !noalias !9822, !noundef !12
  %_43.i.i672.2.i = zext i32 %_44.i.i671.2.i to i64, !dbg !9700
  %842 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_43.i.i672.2.i, !dbg !9835
  %_47.not.i.i.2.i = icmp ult i64 %842, %_60.i.i, !dbg !9836
  %843 = select i1 %_47.not.i.i.2.i, i64 0, i64 %_60.i.i, !dbg !9836
  %spec.select.i.i.2.i = sub nuw i64 %842, %843, !dbg !9836
  %_51.i.i.2.i = mul i64 %spec.select.i.i.2.i, %width.i.i.i, !dbg !9837
  %_50.i.i.2.i = add i64 %_51.i.i.2.i, 2, !dbg !9837
  %_53.i.i673.2.i = icmp ult i64 %_50.i.i.2.i, %_149.1.i.i.pre.i, !dbg !9838
  br i1 %_53.i.i673.2.i, label %bb21.i.i.2.i, label %panic1.i.i.i, !dbg !9838

bb21.i.i.2.i:                                     ; preds = %bb17.i.i.2.i
  %844 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.2.i, !dbg !9838
  %_49.i.i674.2.i = load float, ptr %844, align 4, !dbg !9838, !noalias !9822, !noundef !12
  store float %_49.i.i674.2.i, ptr %iter.i.i.sroa.0.0.ptr7997.2.i, align 4, !dbg !9839, !noalias !9840
  %845 = icmp eq i64 %width.i.i.i, 3, !dbg !9697
  br i1 %845, label %bb16.i.i.loopexit.i, label %bb39.i.i.3.i, !dbg !9697

bb39.i.i.3.i:                                     ; preds = %bb21.i.i.2.i
  %exitcond9609.3.not.i = icmp eq i64 %_145.1.i.i.i, 3, !dbg !9700
  br i1 %exitcond9609.3.not.i, label %panic.i.i.i, label %bb17.i.i.3.i, !dbg !9700

bb17.i.i.3.i:                                     ; preds = %bb39.i.i.3.i
  %846 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 44, !dbg !9700
  %_44.i.i671.3.i = load i32, ptr %846, align 4, !dbg !9700, !noalias !9822, !noundef !12
  %_43.i.i672.3.i = zext i32 %_44.i.i671.3.i to i64, !dbg !9700
  %847 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_43.i.i672.3.i, !dbg !9835
  %_47.not.i.i.3.i = icmp ult i64 %847, %_60.i.i, !dbg !9836
  %848 = select i1 %_47.not.i.i.3.i, i64 0, i64 %_60.i.i, !dbg !9836
  %spec.select.i.i.3.i = sub nuw i64 %847, %848, !dbg !9836
  %_51.i.i.3.i = mul i64 %spec.select.i.i.3.i, %width.i.i.i, !dbg !9837
  %_50.i.i.3.i = add i64 %_51.i.i.3.i, 3, !dbg !9837
  %_53.i.i673.3.i = icmp ult i64 %_50.i.i.3.i, %_149.1.i.i.pre.i, !dbg !9838
  br i1 %_53.i.i673.3.i, label %bb21.i.i.3.i, label %panic1.i.i.i, !dbg !9838

bb21.i.i.3.i:                                     ; preds = %bb17.i.i.3.i
  %849 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.3.i, !dbg !9838
  %_49.i.i674.3.i = load float, ptr %849, align 4, !dbg !9838, !noalias !9822, !noundef !12
  store float %_49.i.i674.3.i, ptr %iter.i.i.sroa.0.0.ptr7997.3.i, align 4, !dbg !9839, !noalias !9840
  %850 = icmp eq i64 %width.i.i.i, 4, !dbg !9697
  br i1 %850, label %bb16.i.i.loopexit.i, label %bb39.i.i.4.i, !dbg !9697

bb39.i.i.4.i:                                     ; preds = %bb21.i.i.3.i
  %exitcond9609.4.not.i = icmp eq i64 %_145.1.i.i.i, 4, !dbg !9700
  br i1 %exitcond9609.4.not.i, label %panic.i.i.i, label %bb17.i.i.4.i, !dbg !9700

bb17.i.i.4.i:                                     ; preds = %bb39.i.i.4.i
  %851 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 56, !dbg !9700
  %_44.i.i671.4.i = load i32, ptr %851, align 4, !dbg !9700, !noalias !9822, !noundef !12
  %_43.i.i672.4.i = zext i32 %_44.i.i671.4.i to i64, !dbg !9700
  %852 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_43.i.i672.4.i, !dbg !9835
  %_47.not.i.i.4.i = icmp ult i64 %852, %_60.i.i, !dbg !9836
  %853 = select i1 %_47.not.i.i.4.i, i64 0, i64 %_60.i.i, !dbg !9836
  %spec.select.i.i.4.i = sub nuw i64 %852, %853, !dbg !9836
  %_51.i.i.4.i = mul i64 %spec.select.i.i.4.i, %width.i.i.i, !dbg !9837
  %_50.i.i.4.i = add i64 %_51.i.i.4.i, 4, !dbg !9837
  %_53.i.i673.4.i = icmp ult i64 %_50.i.i.4.i, %_149.1.i.i.pre.i, !dbg !9838
  br i1 %_53.i.i673.4.i, label %bb21.i.i.4.i, label %panic1.i.i.i, !dbg !9838

bb21.i.i.4.i:                                     ; preds = %bb17.i.i.4.i
  %854 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.4.i, !dbg !9838
  %_49.i.i674.4.i = load float, ptr %854, align 4, !dbg !9838, !noalias !9822, !noundef !12
  store float %_49.i.i674.4.i, ptr %iter.i.i.sroa.0.0.ptr7997.4.i, align 4, !dbg !9839, !noalias !9840
  %855 = icmp eq i64 %width.i.i.i, 5, !dbg !9697
  br i1 %855, label %bb16.i.i.loopexit.i, label %bb39.i.i.5.i, !dbg !9697

bb39.i.i.5.i:                                     ; preds = %bb21.i.i.4.i
  %exitcond9609.5.not.i = icmp eq i64 %_145.1.i.i.i, 5, !dbg !9700
  br i1 %exitcond9609.5.not.i, label %panic.i.i.i, label %bb17.i.i.5.i, !dbg !9700

bb17.i.i.5.i:                                     ; preds = %bb39.i.i.5.i
  %856 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 68, !dbg !9700
  %_44.i.i671.5.i = load i32, ptr %856, align 4, !dbg !9700, !noalias !9822, !noundef !12
  %_43.i.i672.5.i = zext i32 %_44.i.i671.5.i to i64, !dbg !9700
  %857 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_43.i.i672.5.i, !dbg !9835
  %_47.not.i.i.5.i = icmp ult i64 %857, %_60.i.i, !dbg !9836
  %858 = select i1 %_47.not.i.i.5.i, i64 0, i64 %_60.i.i, !dbg !9836
  %spec.select.i.i.5.i = sub nuw i64 %857, %858, !dbg !9836
  %_51.i.i.5.i = mul i64 %spec.select.i.i.5.i, %width.i.i.i, !dbg !9837
  %_50.i.i.5.i = add i64 %_51.i.i.5.i, 5, !dbg !9837
  %_53.i.i673.5.i = icmp ult i64 %_50.i.i.5.i, %_149.1.i.i.pre.i, !dbg !9838
  br i1 %_53.i.i673.5.i, label %bb21.i.i.5.i, label %panic1.i.i.i, !dbg !9838

bb21.i.i.5.i:                                     ; preds = %bb17.i.i.5.i
  %859 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.5.i, !dbg !9838
  %_49.i.i674.5.i = load float, ptr %859, align 4, !dbg !9838, !noalias !9822, !noundef !12
  store float %_49.i.i674.5.i, ptr %iter.i.i.sroa.0.0.ptr7997.5.i, align 4, !dbg !9839, !noalias !9840
  %860 = icmp eq i64 %width.i.i.i, 6, !dbg !9697
  br i1 %860, label %bb16.i.i.loopexit.i, label %bb39.i.i.6.i, !dbg !9697

bb39.i.i.6.i:                                     ; preds = %bb21.i.i.5.i
  %exitcond9609.6.not.i = icmp eq i64 %_145.1.i.i.i, 6, !dbg !9700
  br i1 %exitcond9609.6.not.i, label %panic.i.i.i, label %bb17.i.i.6.i, !dbg !9700

bb17.i.i.6.i:                                     ; preds = %bb39.i.i.6.i
  %861 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 80, !dbg !9700
  %_44.i.i671.6.i = load i32, ptr %861, align 4, !dbg !9700, !noalias !9822, !noundef !12
  %_43.i.i672.6.i = zext i32 %_44.i.i671.6.i to i64, !dbg !9700
  %862 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_43.i.i672.6.i, !dbg !9835
  %_47.not.i.i.6.i = icmp ult i64 %862, %_60.i.i, !dbg !9836
  %863 = select i1 %_47.not.i.i.6.i, i64 0, i64 %_60.i.i, !dbg !9836
  %spec.select.i.i.6.i = sub nuw i64 %862, %863, !dbg !9836
  %_51.i.i.6.i = mul i64 %spec.select.i.i.6.i, %width.i.i.i, !dbg !9837
  %_50.i.i.6.i = add i64 %_51.i.i.6.i, 6, !dbg !9837
  %_53.i.i673.6.i = icmp ult i64 %_50.i.i.6.i, %_149.1.i.i.pre.i, !dbg !9838
  br i1 %_53.i.i673.6.i, label %bb21.i.i.6.i, label %panic1.i.i.i, !dbg !9838

bb21.i.i.6.i:                                     ; preds = %bb17.i.i.6.i
  %864 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.6.i, !dbg !9838
  %_49.i.i674.6.i = load float, ptr %864, align 4, !dbg !9838, !noalias !9822, !noundef !12
  store float %_49.i.i674.6.i, ptr %iter.i.i.sroa.0.0.ptr7997.6.i, align 4, !dbg !9839, !noalias !9840
  %865 = icmp eq i64 %width.i.i.i, 7, !dbg !9697
  br i1 %865, label %bb16.i.i.loopexit.i, label %bb39.i.i.7.i, !dbg !9697

bb39.i.i.7.i:                                     ; preds = %bb21.i.i.6.i
  %exitcond9609.7.not.i = icmp eq i64 %_145.1.i.i.i, 7, !dbg !9700
  br i1 %exitcond9609.7.not.i, label %panic.i.i.i, label %bb17.i.i.7.i, !dbg !9700

bb17.i.i.7.i:                                     ; preds = %bb39.i.i.7.i
  %866 = getelementptr inbounds nuw i8, ptr %_145.0.i.i.i, i64 92, !dbg !9700
  %_44.i.i671.7.i = load i32, ptr %866, align 4, !dbg !9700, !noalias !9822, !noundef !12
  %_43.i.i672.7.i = zext i32 %_44.i.i671.7.i to i64, !dbg !9700
  %867 = add i64 %ring_cursor.sroa.0.1.i6638000.i, %_43.i.i672.7.i, !dbg !9835
  %_47.not.i.i.7.i = icmp ult i64 %867, %_60.i.i, !dbg !9836
  %868 = select i1 %_47.not.i.i.7.i, i64 0, i64 %_60.i.i, !dbg !9836
  %spec.select.i.i.7.i = sub nuw i64 %867, %868, !dbg !9836
  %_51.i.i.7.i = mul i64 %spec.select.i.i.7.i, %width.i.i.i, !dbg !9837
  %_50.i.i.7.i = add i64 %_51.i.i.7.i, 7, !dbg !9837
  %_53.i.i673.7.i = icmp ult i64 %_50.i.i.7.i, %_149.1.i.i.pre.i, !dbg !9838
  br i1 %_53.i.i673.7.i, label %bb21.i.i.7.i, label %panic1.i.i.i, !dbg !9838

bb21.i.i.7.i:                                     ; preds = %bb17.i.i.7.i
  %869 = getelementptr inbounds nuw float, ptr %_147.0.i.i.i, i64 %_50.i.i.7.i, !dbg !9838
  %_49.i.i674.7.i = load float, ptr %869, align 4, !dbg !9838, !noalias !9822, !noundef !12
  store float %_49.i.i674.7.i, ptr %iter.i.i.sroa.0.0.ptr7997.7.i, align 4, !dbg !9839, !noalias !9840
  br label %bb16.i.i.loopexit.i, !dbg !9697

panic1.i.i.i:                                     ; preds = %bb17.i.i.7.i, %bb17.i.i.6.i, %bb17.i.i.5.i, %bb17.i.i.4.i, %bb17.i.i.3.i, %bb17.i.i.2.i, %bb17.i.i.1.i, %bb17.i.i.i
  %_50.i.i.lcssa.ph.i = phi i64 [ %_50.i.i.7.i, %bb17.i.i.7.i ], [ %_50.i.i.6.i, %bb17.i.i.6.i ], [ %_50.i.i.5.i, %bb17.i.i.5.i ], [ %_50.i.i.4.i, %bb17.i.i.4.i ], [ %_50.i.i.3.i, %bb17.i.i.3.i ], [ %_50.i.i.2.i, %bb17.i.i.2.i ], [ %_50.i.i.1.i, %bb17.i.i.1.i ], [ %_51.i.i.i, %bb17.i.i.i ]
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_50.i.i.lcssa.ph.i, i64 noundef %_149.1.i.i.pre.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7d7f2b4ff3f37cf08b84adcf6762221c) #30, !dbg !9838, !noalias !9822
  unreachable, !dbg !9838

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3537.i: ; preds = %bb49.i.i.i
  %_150.0.i.i.i = load ptr, ptr %669, align 8, !dbg !9816, !alias.scope !9551, !noalias !9552, !nonnull !12, !noundef !12
  %_127.i.i.i = getelementptr inbounds nuw float, ptr %_150.0.i.i.i, i64 %_76.i.i.i, !dbg !9841
  %lanes.i3151.sroa.0.0.copyload.i = load <8 x float>, ptr %_127.i.i.i, align 4, !dbg !9843, !alias.scope !9847, !noalias !9851
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_127.i.i.i, ptr noundef nonnull align 4 dereferenceable(32) %_100.i.i, i64 32, i1 false), !dbg !9853
  %870 = fmul <8 x float> %830, %lanes.i3151.sroa.0.0.copyload.i, !dbg !9858
  %871 = select <8 x i1> %671, <8 x float> %lanes.i3151.sroa.0.0.copyload.i, <8 x float> %870, !dbg !9863
  store <8 x float> %871, ptr %_100.i.i, align 4, !dbg !9868, !alias.scope !9873, !noalias !9877
  %872 = add i64 %main_cursor.sroa.0.1.i6648001.i, 1, !dbg !9881
  %_65.i676.i = icmp eq i64 %872, %_67.i675.i, !dbg !9882
  %spec.store.select.i.i = select i1 %_65.i676.i, i64 0, i64 %872, !dbg !9882
  %873 = add i64 %ring_cursor.sroa.0.1.i6638000.i, 1, !dbg !9883
  %_68.i.i = icmp eq i64 %873, %_60.i.i, !dbg !9884
  %spec.store.select9.i.i = select i1 %_68.i.i, i64 0, i64 %873, !dbg !9884
  %exitcond9614.not.i = icmp eq i64 %782, %umax9613.i, !dbg !9885
  br i1 %exitcond9614.not.i, label %bb14.i.bb12.i651.loopexit_crit_edge.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3190.i, !dbg !8913

bb36.i677.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane4load.exit3190.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i665.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_beae6f21d5ab5b7a10c8cf24995b1244) #30, !dbg !9888, !noalias !9889
  unreachable, !dbg !9888

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i: ; preds = %bb12.i651.loopexit.i
  store <8 x float> %history.i.i611.sroa.19.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.19.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.22.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.22.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.25.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.25.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944
  store <8 x float> %history.i.i611.sroa.10.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.10.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944, !noalias !6249
  store <8 x float> %history.i.i611.sroa.13.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.13.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944, !noalias !6249
  store <8 x float> %history.i.i611.sroa.16.sroa.0.0.lcssa.i, ptr %history.i.i611.sroa.16.0.hot_left.i641.sroa_idx.i, align 1, !dbg !8944, !noalias !6249
  %874 = trunc i64 %main_cursor.sroa.0.1.i664.lcssa.i to i32, !dbg !9890
  %875 = trunc i64 %ring_cursor.sroa.0.1.i663.lcssa.i to i32, !dbg !9891
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !9484

_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i, %bb7.i.i
  %history.i.i611.sroa.0.0.lcssa8044.lcssa.i = phi <8 x float> [ %hot_left.i641.promoted.i, %bb7.i.i ], [ %history.i.i611.sroa.0.0.lcssa.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ]
  %ring_cursor.sroa.0.0.i653.lcssa.i = phi i32 [ %_26.i648.i, %bb7.i.i ], [ %875, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !8863
  %main_cursor.sroa.0.0.i654.lcssa.i = phi i32 [ %_24.i.i, %bb7.i.i ], [ %874, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.loopexit.i ], !dbg !8859
  store <8 x float> %history.i.i611.sroa.0.0.lcssa8044.lcssa.i, ptr %hot_left.i641.i, align 1, !dbg !9484, !noalias !6249
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i641.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25) #31, !dbg !9892, !noalias !5902
  store i32 %main_cursor.sroa.0.0.i654.lcssa.i, ptr %_25.i, align 4, !dbg !9890, !alias.scope !8861, !noalias !8862
  store i32 %ring_cursor.sroa.0.0.i653.lcssa.i, ptr %610, align 4, !dbg !9891, !alias.scope !8861, !noalias !8862
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i635.i), !dbg !9893, !noalias !8867
  call void @llvm.lifetime.end.p0(ptr nonnull %scratch.i.i), !dbg !9894, !noalias !8867
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !8840

bb3.i.i:                                          ; preds = %bb2.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9895), !dbg !9898
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9899), !dbg !9898
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9901), !dbg !9898
  tail call void @llvm.experimental.noalias.scope.decl(metadata !9903), !dbg !9898
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::load
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E4loadB5_(ptr noalias noundef align 32 captures(none) dereferenceable(736) %hot_left.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !9905, !noalias !5902
  %876 = getelementptr inbounds nuw i8, ptr %self, i64 1536, !dbg !9909
  %877 = load i8, ptr %876, align 32, !dbg !9909, !range !17, !alias.scope !9913, !noalias !9914, !noundef !12
  %878 = getelementptr inbounds nuw i8, ptr %self, i64 1537, !dbg !9916
  %879 = load i8, ptr %878, align 1, !dbg !9916, !range !17, !alias.scope !9913, !noalias !9914, !noundef !12
  %880 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !9918
  %ring.i.i = load i64, ptr %880, align 8, !dbg !9918, !alias.scope !9920, !noalias !9921, !noundef !12
  %881 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !9922
  %main.i.i = load i64, ptr %881, align 8, !dbg !9922, !alias.scope !9920, !noalias !9921, !noundef !12
  %_25.i.i = load i32, ptr %_25.i, align 4, !dbg !9924, !alias.scope !9926, !noalias !9927, !noundef !12
  %882 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !9928
  %_26.i.i = load i32, ptr %882, align 4, !dbg !9928, !alias.scope !9926, !noalias !9927, !noundef !12
  call void @llvm.lifetime.start.p0(ptr nonnull %peaks_left.i.i), !dbg !9930, !noalias !9932
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(1024) %peaks_left.i.i, i8 0, i64 1024, i1 false), !noalias !9932
  call void @llvm.lifetime.start.p0(ptr nonnull %uniform_left.i.i), !dbg !9933, !noalias !9932
; call <true_peak_limiter::UniformHot<wide::f32x8_::f32x8>>::new
  call fastcc void @_RNvMs6_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10UniformHotNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E3newB5_(ptr noalias noundef align 32 captures(none) dereferenceable(128) %uniform_left.i.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, i64 %ring.i.i, i64 %main.i.i) #31, !dbg !9935, !noalias !5902
  %883 = add nuw nsw i64 %_31, 31, !dbg !9936
  %yield_count.sroa.0.0.i.i3973.i = lshr i64 %883, 5, !dbg !9936
  %hot_left.i.promoted.i = load <8 x float>, ptr %hot_left.i.i, align 1, !noalias !6249
  %uniform_left.i.promoted.i = load <8 x float>, ptr %uniform_left.i.i, align 1, !noalias !6249
  %_107.not.i8434.i = icmp eq i64 %yield_count.sroa.0.0.i.i3973.i, 0, !dbg !9943
  br i1 %_107.not.i8434.i, label %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i, label %bb36.i.lr.ph.i, !dbg !9943

bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i: ; preds = %bb3.i.i
  %.phi.trans.insert9933.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 104
  %left_phase.i.pre.i = load i32, ptr %.phi.trans.insert9933.i, align 8, !dbg !9952, !noalias !9932
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !9943

bb36.i.lr.ph.i:                                   ; preds = %bb3.i.i
  %884 = zext i32 %_26.i.i to i64, !dbg !9928
  %885 = zext i32 %_25.i.i to i64, !dbg !9924
  %_22.i.i = trunc nuw i8 %879 to i1, !dbg !9916
  %_21.i.i = trunc nuw i8 %877 to i1, !dbg !9909
  %886 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !9953
  %887 = bitcast <8 x float> %886 to <8 x i32>, !dbg !9959
  %888 = xor <8 x i32> %887, splat (i32 -1), !dbg !9965
  %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 32
  %history.i.i.sroa.13.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 64
  %history.i.i.sroa.16.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 96
  %history.i.i.sroa.19.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 128
  %history.i.i.sroa.22.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 160
  %history.i.i.sroa.25.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 192
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 224
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 256
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 288
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 320
  %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 352
  %889 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %890 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %891 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %row12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %892 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %893 = getelementptr inbounds nuw i8, ptr %self, i64 192
  %894 = getelementptr inbounds nuw i8, ptr %self, i64 224
  %row13.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 256
  %895 = getelementptr inbounds nuw i8, ptr %self, i64 288
  %896 = getelementptr inbounds nuw i8, ptr %self, i64 320
  %897 = getelementptr inbounds nuw i8, ptr %self, i64 352
  %row14.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 384
  %898 = getelementptr inbounds nuw i8, ptr %self, i64 416
  %899 = getelementptr inbounds nuw i8, ptr %self, i64 448
  %900 = getelementptr inbounds nuw i8, ptr %self, i64 480
  %row15.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 512
  %901 = getelementptr inbounds nuw i8, ptr %self, i64 544
  %902 = getelementptr inbounds nuw i8, ptr %self, i64 576
  %903 = getelementptr inbounds nuw i8, ptr %self, i64 608
  %row16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 640
  %904 = getelementptr inbounds nuw i8, ptr %self, i64 672
  %905 = getelementptr inbounds nuw i8, ptr %self, i64 704
  %906 = getelementptr inbounds nuw i8, ptr %self, i64 736
  %row17.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 768
  %907 = getelementptr inbounds nuw i8, ptr %self, i64 800
  %908 = getelementptr inbounds nuw i8, ptr %self, i64 832
  %909 = getelementptr inbounds nuw i8, ptr %self, i64 864
  %row18.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 896
  %910 = getelementptr inbounds nuw i8, ptr %self, i64 928
  %911 = getelementptr inbounds nuw i8, ptr %self, i64 960
  %912 = getelementptr inbounds nuw i8, ptr %self, i64 992
  %row19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1024
  %913 = getelementptr inbounds nuw i8, ptr %self, i64 1056
  %914 = getelementptr inbounds nuw i8, ptr %self, i64 1088
  %915 = getelementptr inbounds nuw i8, ptr %self, i64 1120
  %row20.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1152
  %916 = getelementptr inbounds nuw i8, ptr %self, i64 1184
  %917 = getelementptr inbounds nuw i8, ptr %self, i64 1216
  %918 = getelementptr inbounds nuw i8, ptr %self, i64 1248
  %row21.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1280
  %919 = getelementptr inbounds nuw i8, ptr %self, i64 1312
  %920 = getelementptr inbounds nuw i8, ptr %self, i64 1344
  %921 = getelementptr inbounds nuw i8, ptr %self, i64 1376
  %row22.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 1408
  %922 = getelementptr inbounds nuw i8, ptr %self, i64 1440
  %923 = getelementptr inbounds nuw i8, ptr %self, i64 1472
  %924 = getelementptr inbounds nuw i8, ptr %self, i64 1504
  %925 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 80
  %_48.i.sroa.3.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 88
  %_48.i.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 96
  %_78.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 384
  %_79.i.i = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 512
  %926 = select i1 %_21.i.i, <8 x i32> %887, <8 x i32> %888
  %927 = icmp slt <8 x i32> %926, zeroinitializer
  %928 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 32
  %929 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 40
  %_22.i.i.i = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 104
  %930 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 48
  %931 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 56
  %932 = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 672
  %933 = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 704
  %934 = getelementptr inbounds nuw i8, ptr %hot_left.i.i, i64 640
  %935 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 72
  %936 = getelementptr inbounds nuw i8, ptr %uniform_left.i.i, i64 64
  %937 = select i1 %_22.i.i, <8 x i32> %887, <8 x i32> %888
  %938 = icmp slt <8 x i32> %937, zeroinitializer
  %history.i.i.sroa.10.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.13.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.16.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.19.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.22.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.25.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.25.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %history.i.i.sroa.41.0.hot_left.i.sroa_idx.promoted.i = load <8 x float>, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 32, !noalias !6249
  %_22.i.i.promoted.i = load i32, ptr %_22.i.i.i, align 4, !noalias !6249
  %.promoted8482.i = load <8 x float>, ptr %932, align 32, !noalias !6249
  %_48.i.sroa.3.0.copyload.i = load i64, ptr %_48.i.sroa.3.0..sroa_idx.i, align 8, !noalias !6249
  %_48.i.sroa.4.0.copyload.i = load i64, ptr %_48.i.sroa.4.0..sroa_idx.i, align 16, !noalias !6249
  %_8.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %_78.i.i, align 32, !noalias !6249
  %_9.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %_79.i.i, align 32, !noalias !6249
  %_54.0.i.i.i = load ptr, ptr %928, align 32, !noalias !6249, !nonnull !12, !align !24
  %_54.1.i.i.i = load i64, ptr %929, align 8, !noalias !6249
  %_18.i21.i.i = load i64, ptr %925, align 16, !noalias !6249
  %_56.0.i.i.i = load ptr, ptr %930, align 16, !noalias !6249, !nonnull !12, !align !24
  %_56.1.i.i.i = load i64, ptr %931, align 8, !noalias !6249
  %_37.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %933, align 32, !noalias !6249
  %_58.1.i.i.i = load i64, ptr %935, align 8, !noalias !6249
  %_58.0.i.i.i = load ptr, ptr %936, align 32, !noalias !6249, !nonnull !12, !align !24
  %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1
  %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1
  %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1
  %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1
  %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i.promoted = load <8 x float>, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1
  br label %bb36.i.i, !dbg !9943

bb16.i.bb13.i7.loopexit_crit_edge.i:              ; preds = %bb25.i.i
  store <8 x float> %.lcssa83458388.i, ptr %932, align 32, !noalias !6249
  br label %bb13.i7.loopexit.i, !dbg !9967

bb13.i7.loopexit.i:                               ; preds = %bb36.i.i, %bb16.i.bb13.i7.loopexit_crit_edge.i
  %history.i.i.sroa.0.0.lcssa.i31 = phi <8 x float> [ %lanes.i3219.sroa.0.0.copyload.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.0.0.lcssa84078436.i, %bb36.i.i ]
  %history.i.i.sroa.25.sroa.0.0.lcssa.i30 = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.08283.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.25.sroa.0.0.lcssa10938.i, %bb36.i.i ]
  %history.i.i.sroa.29.sroa.0.0.lcssa.i29 = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.08288.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.29.sroa.0.0.lcssa8463.i, %bb36.i.i ]
  %history.i.i.sroa.32.sroa.0.0.lcssa.i28 = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.08287.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.32.sroa.0.0.lcssa8464.i, %bb36.i.i ]
  %history.i.i.sroa.35.sroa.0.0.lcssa.i27 = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.08286.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.35.sroa.0.0.lcssa8465.i, %bb36.i.i ]
  %history.i.i.sroa.38.sroa.0.0.lcssa.i26 = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.08285.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.38.sroa.0.0.lcssa8466.i, %bb36.i.i ]
  %history.i.i.sroa.41.sroa.0.0.lcssa.i25 = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.08284.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.41.sroa.0.0.lcssa8467.i, %bb36.i.i ]
  %history.i.i.sroa.22.sroa.0.0.lcssa.i24 = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.08282.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.22.sroa.0.0.lcssa10926.i, %bb36.i.i ]
  %history.i.i.sroa.19.sroa.0.0.lcssa.i23 = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.08281.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.19.sroa.0.0.lcssa10914.i, %bb36.i.i ]
  %history.i.i.sroa.16.sroa.0.0.lcssa.i22 = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.08280.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.16.sroa.0.0.lcssa10902.i, %bb36.i.i ]
  %history.i.i.sroa.13.sroa.0.0.lcssa.i21 = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.08279.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.13.sroa.0.0.lcssa10890.i, %bb36.i.i ]
  %history.i.i.sroa.10.sroa.0.0.lcssa.i20 = phi <8 x float> [ %history.i.i.sroa.0.08289.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %history.i.i.sroa.10.sroa.0.0.lcssa8445.i, %bb36.i.i ]
  %.lcssa83458388.lcssa8483.i = phi <8 x float> [ %.lcssa83458388.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %.lcssa83458388.lcssa8484.i, %bb36.i.i ]
  %storemerge.i.i.lcssa83298368.lcssa8468.i = phi i32 [ %storemerge.i.i.lcssa83298368.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %storemerge.i.i.lcssa83298368.lcssa8469.i, %bb36.i.i ]
  %minimum.i.i.sroa.0.08307.lcssa8349.lcssa.i = phi <8 x float> [ %minimum.i.i.sroa.0.08307.lcssa.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %minimum.i.i.sroa.0.08307.lcssa8349.lcssa84208435.i, %bb36.i.i ]
  %ring_cursor.sroa.0.1.i.lcssa.i = phi i64 [ %ring_cursor.sroa.0.2.i.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %ring_cursor.sroa.0.0.i8437.i, %bb36.i.i ], !dbg !9971
  %main_cursor.sroa.0.1.i.lcssa.i = phi i64 [ %main_cursor.sroa.0.2.i.i, %bb16.i.bb13.i7.loopexit_crit_edge.i ], [ %main_cursor.sroa.0.0.i8438.i, %bb36.i.i ], !dbg !9972
  %_107.not.i.i = icmp eq i64 %940, 0, !dbg !9943
  %indvars.iv.next9640.i = add nsw i64 %indvars.iv9639.i, -32, !dbg !9943
  br i1 %_107.not.i.i, label %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i, label %bb36.i.i, !dbg !9943

bb36.i.i:                                         ; preds = %bb13.i7.loopexit.i, %bb36.i.lr.ph.i
  %history.i.i.sroa.41.sroa.0.0.lcssa8467.i2921 = phi <8 x float> [ %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.41.sroa.0.0.lcssa.i25, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.38.sroa.0.0.lcssa8466.i2912 = phi <8 x float> [ %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.38.sroa.0.0.lcssa.i26, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.35.sroa.0.0.lcssa8465.i2903 = phi <8 x float> [ %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.35.sroa.0.0.lcssa.i27, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.32.sroa.0.0.lcssa8464.i2894 = phi <8 x float> [ %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.32.sroa.0.0.lcssa.i28, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.29.sroa.0.0.lcssa8463.i2885 = phi <8 x float> [ %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i.promoted, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.29.sroa.0.0.lcssa.i29, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.25.sroa.0.0.lcssa10938.i = phi <8 x float> [ %history.i.i.sroa.25.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.25.sroa.0.0.lcssa.i30, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.22.sroa.0.0.lcssa10926.i = phi <8 x float> [ %history.i.i.sroa.22.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.22.sroa.0.0.lcssa.i24, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.19.sroa.0.0.lcssa10914.i = phi <8 x float> [ %history.i.i.sroa.19.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.19.sroa.0.0.lcssa.i23, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.16.sroa.0.0.lcssa10902.i = phi <8 x float> [ %history.i.i.sroa.16.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.16.sroa.0.0.lcssa.i22, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.13.sroa.0.0.lcssa10890.i = phi <8 x float> [ %history.i.i.sroa.13.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.13.sroa.0.0.lcssa.i21, %bb13.i7.loopexit.i ]
  %indvars.iv9639.i = phi i64 [ %_31, %bb36.i.lr.ph.i ], [ %indvars.iv.next9640.i, %bb13.i7.loopexit.i ]
  %.lcssa83458388.lcssa8484.i = phi <8 x float> [ %.promoted8482.i, %bb36.i.lr.ph.i ], [ %.lcssa83458388.lcssa8483.i, %bb13.i7.loopexit.i ]
  %storemerge.i.i.lcssa83298368.lcssa8469.i = phi i32 [ %_22.i.i.promoted.i, %bb36.i.lr.ph.i ], [ %storemerge.i.i.lcssa83298368.lcssa8468.i, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.41.sroa.0.0.lcssa8467.i = phi <8 x float> [ %history.i.i.sroa.41.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.41.sroa.0.0.lcssa.i25, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.38.sroa.0.0.lcssa8466.i = phi <8 x float> [ %history.i.i.sroa.38.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.38.sroa.0.0.lcssa.i26, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.35.sroa.0.0.lcssa8465.i = phi <8 x float> [ %history.i.i.sroa.35.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.35.sroa.0.0.lcssa.i27, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.32.sroa.0.0.lcssa8464.i = phi <8 x float> [ %history.i.i.sroa.32.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.32.sroa.0.0.lcssa.i28, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.29.sroa.0.0.lcssa8463.i = phi <8 x float> [ %history.i.i.sroa.29.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.29.sroa.0.0.lcssa.i29, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.10.sroa.0.0.lcssa8445.i = phi <8 x float> [ %history.i.i.sroa.10.0.hot_left.i.sroa_idx.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.10.sroa.0.0.lcssa.i20, %bb13.i7.loopexit.i ]
  %iter3.sroa.0.0.i8440.i = phi i64 [ %yield_count.sroa.0.0.i.i3973.i, %bb36.i.lr.ph.i ], [ %940, %bb13.i7.loopexit.i ]
  %iter2.sroa.0.0.i8439.i = phi i64 [ 0, %bb36.i.lr.ph.i ], [ %939, %bb13.i7.loopexit.i ]
  %main_cursor.sroa.0.0.i8438.i = phi i64 [ %885, %bb36.i.lr.ph.i ], [ %main_cursor.sroa.0.1.i.lcssa.i, %bb13.i7.loopexit.i ]
  %ring_cursor.sroa.0.0.i8437.i = phi i64 [ %884, %bb36.i.lr.ph.i ], [ %ring_cursor.sroa.0.1.i.lcssa.i, %bb13.i7.loopexit.i ]
  %history.i.i.sroa.0.0.lcssa84078436.i = phi <8 x float> [ %hot_left.i.promoted.i, %bb36.i.lr.ph.i ], [ %history.i.i.sroa.0.0.lcssa.i31, %bb13.i7.loopexit.i ]
  %minimum.i.i.sroa.0.08307.lcssa8349.lcssa84208435.i = phi <8 x float> [ %uniform_left.i.promoted.i, %bb36.i.lr.ph.i ], [ %minimum.i.i.sroa.0.08307.lcssa8349.lcssa.i, %bb13.i7.loopexit.i ]
  %umin9654.i = tail call i64 @llvm.umin.i64(i64 %indvars.iv9639.i, i64 32), !dbg !9973
  %umax9642.i = tail call i64 @llvm.umax.i64(i64 %umin9654.i, i64 1), !dbg !9973
  %939 = add nuw nsw i64 %iter2.sroa.0.0.i8439.i, 32, !dbg !9973
  %940 = add nsw i64 %iter3.sroa.0.0.i8440.i, -1, !dbg !9977
  %_34.i.i = sub nsw i64 %_31, %iter2.sroa.0.0.i8439.i, !dbg !9978
  %..i3974.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i.i, i64 32), !dbg !9979
  %_20.i.i8278.not.i = icmp eq i64 %iter2.sroa.0.0.i8439.i, %_31, !dbg !9983
  br i1 %_20.i.i8278.not.i, label %bb13.i7.loopexit.i, label %bb5.i.i.lr.ph.i, !dbg !9988

bb5.i.i.lr.ph.i:                                  ; preds = %bb36.i.i
  %_5.i2928.i = load <8 x float>, ptr %self, align 32, !alias.scope !5901, !noalias !5902
  %_14.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %889, align 32, !alias.scope !5901, !noalias !5902
  %_17.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %890, align 32, !alias.scope !5901, !noalias !5902
  %_20.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %891, align 32, !alias.scope !5901, !noalias !5902
  %_25.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row12.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_28.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %892, align 32, !alias.scope !5901, !noalias !5902
  %_31.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %893, align 32, !alias.scope !5901, !noalias !5902
  %_34.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %894, align 32, !alias.scope !5901, !noalias !5902
  %_39.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row13.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_42.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %895, align 32, !alias.scope !5901, !noalias !5902
  %_45.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %896, align 32, !alias.scope !5901, !noalias !5902
  %_48.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %897, align 32, !alias.scope !5901, !noalias !5902
  %_53.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row14.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_56.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %898, align 32, !alias.scope !5901, !noalias !5902
  %_59.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %899, align 32, !alias.scope !5901, !noalias !5902
  %_62.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %900, align 32, !alias.scope !5901, !noalias !5902
  %_67.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row15.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_70.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %901, align 32, !alias.scope !5901, !noalias !5902
  %_73.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %902, align 32, !alias.scope !5901, !noalias !5902
  %_76.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %903, align 32, !alias.scope !5901, !noalias !5902
  %_81.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row16.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_84.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %904, align 32, !alias.scope !5901, !noalias !5902
  %_87.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %905, align 32, !alias.scope !5901, !noalias !5902
  %_90.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %906, align 32, !alias.scope !5901, !noalias !5902
  %_95.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row17.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_98.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %907, align 32, !alias.scope !5901, !noalias !5902
  %_101.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %908, align 32, !alias.scope !5901, !noalias !5902
  %_104.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %909, align 32, !alias.scope !5901, !noalias !5902
  %_109.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row18.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_112.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %910, align 32, !alias.scope !5901, !noalias !5902
  %_115.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %911, align 32, !alias.scope !5901, !noalias !5902
  %_118.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %912, align 32, !alias.scope !5901, !noalias !5902
  %_123.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row19.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_126.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %913, align 32, !alias.scope !5901, !noalias !5902
  %_129.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %914, align 32, !alias.scope !5901, !noalias !5902
  %_132.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %915, align 32, !alias.scope !5901, !noalias !5902
  %_137.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row20.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_140.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %916, align 32, !alias.scope !5901, !noalias !5902
  %_143.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %917, align 32, !alias.scope !5901, !noalias !5902
  %_146.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %918, align 32, !alias.scope !5901, !noalias !5902
  %_151.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row21.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_154.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %919, align 32, !alias.scope !5901, !noalias !5902
  %_157.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %920, align 32, !alias.scope !5901, !noalias !5902
  %_160.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %921, align 32, !alias.scope !5901, !noalias !5902
  %_165.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %row22.i.i.i.i.i, align 32, !alias.scope !5901, !noalias !5902
  %_168.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %922, align 32, !alias.scope !5901, !noalias !5902
  %_171.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %923, align 32, !alias.scope !5901, !noalias !5902
  %_174.i.i.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %924, align 32, !alias.scope !5901, !noalias !5902
  br label %bb5.i.i.i, !dbg !9988

bb5.i.i.i:                                        ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i, %bb5.i.i.lr.ph.i
  %iter.sroa.0.0.i.i8290.i = phi i64 [ 0, %bb5.i.i.lr.ph.i ], [ %941, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.0.08289.i = phi <8 x float> [ %history.i.i.sroa.0.0.lcssa84078436.i, %bb5.i.i.lr.ph.i ], [ %lanes.i3219.sroa.0.0.copyload.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.25.sroa.0.08288.i = phi <8 x float> [ %history.i.i.sroa.25.sroa.0.0.lcssa10938.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.22.sroa.0.08283.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.29.sroa.0.08287.i = phi <8 x float> [ %history.i.i.sroa.29.sroa.0.0.lcssa8463.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.25.sroa.0.08288.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.32.sroa.0.08286.i = phi <8 x float> [ %history.i.i.sroa.32.sroa.0.0.lcssa8464.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.29.sroa.0.08287.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.35.sroa.0.08285.i = phi <8 x float> [ %history.i.i.sroa.35.sroa.0.0.lcssa8465.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.32.sroa.0.08286.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.38.sroa.0.08284.i = phi <8 x float> [ %history.i.i.sroa.38.sroa.0.0.lcssa8466.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.35.sroa.0.08285.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.22.sroa.0.08283.i = phi <8 x float> [ %history.i.i.sroa.22.sroa.0.0.lcssa10926.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.19.sroa.0.08282.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.19.sroa.0.08282.i = phi <8 x float> [ %history.i.i.sroa.19.sroa.0.0.lcssa10914.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.16.sroa.0.08281.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.16.sroa.0.08281.i = phi <8 x float> [ %history.i.i.sroa.16.sroa.0.0.lcssa10902.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.13.sroa.0.08280.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.13.sroa.0.08280.i = phi <8 x float> [ %history.i.i.sroa.13.sroa.0.0.lcssa10890.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.10.sroa.0.08279.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %history.i.i.sroa.10.sroa.0.08279.i = phi <8 x float> [ %history.i.i.sroa.10.sroa.0.0.lcssa8445.i, %bb5.i.i.lr.ph.i ], [ %history.i.i.sroa.0.08289.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i ]
  %941 = add nuw nsw i64 %iter.sroa.0.0.i.i8290.i, 1, !dbg !9989
  %_11.i.i.i = add nuw nsw i64 %iter.sroa.0.0.i.i8290.i, %iter2.sroa.0.0.i8439.i, !dbg !9992
  %base.i.i.i = shl i64 %_11.i.i.i, 3, !dbg !9992
  %_24.i.i.i = icmp samesign ugt i64 %base.i.i.i, %_39.1, !dbg !9993
  br i1 %_24.i.i.i, label %bb7.i.i.i, label %bb8.i.i.i, !dbg !9993, !prof !639

bb8.i.i.i:                                        ; preds = %bb5.i.i.i
  %_27.i.i.i = sub nuw nsw i64 %_39.1, %base.i.i.i, !dbg !9996
  %_8.i3222.i = icmp samesign ugt i64 %_27.i.i.i, 7, !dbg !9997
  br i1 %_8.i3222.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i, label %bb2.i3223.i, !dbg !9997, !prof !651

bb2.i3223.i:                                      ; preds = %bb8.i.i.i
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa8463.i2885, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa8464.i2894, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa8465.i2903, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa8466.i2912, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa8467.i2921, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_27.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !10003, !noalias !10004
  unreachable, !dbg !10003

bb7.i.i.i:                                        ; preds = %bb5.i.i.i
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa8463.i2885, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa8464.i2894, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa8465.i2903, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa8466.i2912, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa8467.i2921, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc26f793d85338b5649d38df0c19e7e0) #30, !dbg !10011, !noalias !10012
  unreachable, !dbg !10011

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i: ; preds = %bb8.i.i.i
  %_31.i.i.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i.i.i, !dbg !10013
  %lanes.i3219.sroa.0.0.copyload.i = load <8 x float>, ptr %_31.i.i.i, align 4, !dbg !10015, !alias.scope !10019, !noalias !10023
  %942 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %history.i.i.sroa.22.sroa.0.08283.i), !dbg !10025
  %943 = fmul <8 x float> %_5.i2928.i, %lanes.i3219.sroa.0.0.copyload.i, !dbg !10032
  %944 = fadd <8 x float> %943, zeroinitializer, !dbg !10038
  %945 = fmul <8 x float> %_14.i.i.i.i.sroa.0.0.copyload.i, %lanes.i3219.sroa.0.0.copyload.i, !dbg !10043
  %946 = fadd <8 x float> %945, zeroinitializer, !dbg !10048
  %947 = fmul <8 x float> %_17.i.i.i.i.sroa.0.0.copyload.i, %lanes.i3219.sroa.0.0.copyload.i, !dbg !10053
  %948 = fadd <8 x float> %947, zeroinitializer, !dbg !10058
  %949 = fmul <8 x float> %_20.i.i.i.i.sroa.0.0.copyload.i, %lanes.i3219.sroa.0.0.copyload.i, !dbg !10063
  %950 = fadd <8 x float> %949, zeroinitializer, !dbg !10068
  %951 = fmul <8 x float> %_25.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.0.08289.i, !dbg !10073
  %952 = fadd <8 x float> %951, %944, !dbg !10078
  %953 = fmul <8 x float> %_28.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.0.08289.i, !dbg !10083
  %954 = fadd <8 x float> %953, %946, !dbg !10088
  %955 = fmul <8 x float> %_31.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.0.08289.i, !dbg !10093
  %956 = fadd <8 x float> %955, %948, !dbg !10098
  %957 = fmul <8 x float> %_34.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.0.08289.i, !dbg !10103
  %958 = fadd <8 x float> %957, %950, !dbg !10108
  %959 = fmul <8 x float> %_39.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.10.sroa.0.08279.i, !dbg !10113
  %960 = fadd <8 x float> %959, %952, !dbg !10118
  %961 = fmul <8 x float> %_42.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.10.sroa.0.08279.i, !dbg !10123
  %962 = fadd <8 x float> %961, %954, !dbg !10128
  %963 = fmul <8 x float> %_45.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.10.sroa.0.08279.i, !dbg !10133
  %964 = fadd <8 x float> %963, %956, !dbg !10138
  %965 = fmul <8 x float> %_48.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.10.sroa.0.08279.i, !dbg !10143
  %966 = fadd <8 x float> %965, %958, !dbg !10148
  %967 = fmul <8 x float> %_53.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.13.sroa.0.08280.i, !dbg !10153
  %968 = fadd <8 x float> %967, %960, !dbg !10158
  %969 = fmul <8 x float> %_56.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.13.sroa.0.08280.i, !dbg !10163
  %970 = fadd <8 x float> %969, %962, !dbg !10168
  %971 = fmul <8 x float> %_59.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.13.sroa.0.08280.i, !dbg !10173
  %972 = fadd <8 x float> %971, %964, !dbg !10178
  %973 = fmul <8 x float> %_62.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.13.sroa.0.08280.i, !dbg !10183
  %974 = fadd <8 x float> %973, %966, !dbg !10188
  %975 = fmul <8 x float> %_67.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.16.sroa.0.08281.i, !dbg !10193
  %976 = fadd <8 x float> %975, %968, !dbg !10198
  %977 = fmul <8 x float> %_70.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.16.sroa.0.08281.i, !dbg !10203
  %978 = fadd <8 x float> %977, %970, !dbg !10208
  %979 = fmul <8 x float> %_73.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.16.sroa.0.08281.i, !dbg !10213
  %980 = fadd <8 x float> %979, %972, !dbg !10218
  %981 = fmul <8 x float> %_76.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.16.sroa.0.08281.i, !dbg !10223
  %982 = fadd <8 x float> %981, %974, !dbg !10228
  %983 = fmul <8 x float> %_81.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.19.sroa.0.08282.i, !dbg !10233
  %984 = fadd <8 x float> %983, %976, !dbg !10238
  %985 = fmul <8 x float> %_84.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.19.sroa.0.08282.i, !dbg !10243
  %986 = fadd <8 x float> %985, %978, !dbg !10248
  %987 = fmul <8 x float> %_87.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.19.sroa.0.08282.i, !dbg !10253
  %988 = fadd <8 x float> %987, %980, !dbg !10258
  %989 = fmul <8 x float> %_90.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.19.sroa.0.08282.i, !dbg !10263
  %990 = fadd <8 x float> %989, %982, !dbg !10268
  %991 = fmul <8 x float> %_95.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.22.sroa.0.08283.i, !dbg !10273
  %992 = fadd <8 x float> %991, %984, !dbg !10278
  %993 = fmul <8 x float> %_98.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.22.sroa.0.08283.i, !dbg !10283
  %994 = fadd <8 x float> %993, %986, !dbg !10288
  %995 = fmul <8 x float> %_101.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.22.sroa.0.08283.i, !dbg !10293
  %996 = fadd <8 x float> %995, %988, !dbg !10298
  %997 = fmul <8 x float> %_104.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.22.sroa.0.08283.i, !dbg !10303
  %998 = fadd <8 x float> %997, %990, !dbg !10308
  %999 = fmul <8 x float> %_109.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.25.sroa.0.08288.i, !dbg !10313
  %1000 = fadd <8 x float> %999, %992, !dbg !10318
  %1001 = fmul <8 x float> %_112.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.25.sroa.0.08288.i, !dbg !10323
  %1002 = fadd <8 x float> %1001, %994, !dbg !10328
  %1003 = fmul <8 x float> %_115.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.25.sroa.0.08288.i, !dbg !10333
  %1004 = fadd <8 x float> %1003, %996, !dbg !10338
  %1005 = fmul <8 x float> %_118.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.25.sroa.0.08288.i, !dbg !10343
  %1006 = fadd <8 x float> %1005, %998, !dbg !10348
  %1007 = fmul <8 x float> %_123.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.29.sroa.0.08287.i, !dbg !10353
  %1008 = fadd <8 x float> %1007, %1000, !dbg !10358
  %1009 = fmul <8 x float> %_126.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.29.sroa.0.08287.i, !dbg !10363
  %1010 = fadd <8 x float> %1009, %1002, !dbg !10368
  %1011 = fmul <8 x float> %_129.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.29.sroa.0.08287.i, !dbg !10373
  %1012 = fadd <8 x float> %1011, %1004, !dbg !10378
  %1013 = fmul <8 x float> %_132.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.29.sroa.0.08287.i, !dbg !10383
  %1014 = fadd <8 x float> %1013, %1006, !dbg !10388
  %1015 = fmul <8 x float> %_137.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.32.sroa.0.08286.i, !dbg !10393
  %1016 = fadd <8 x float> %1015, %1008, !dbg !10398
  %1017 = fmul <8 x float> %_140.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.32.sroa.0.08286.i, !dbg !10403
  %1018 = fadd <8 x float> %1017, %1010, !dbg !10408
  %1019 = fmul <8 x float> %_143.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.32.sroa.0.08286.i, !dbg !10413
  %1020 = fadd <8 x float> %1019, %1012, !dbg !10418
  %1021 = fmul <8 x float> %_146.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.32.sroa.0.08286.i, !dbg !10423
  %1022 = fadd <8 x float> %1021, %1014, !dbg !10428
  %1023 = fmul <8 x float> %_151.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.35.sroa.0.08285.i, !dbg !10433
  %1024 = fadd <8 x float> %1023, %1016, !dbg !10438
  %1025 = fmul <8 x float> %_154.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.35.sroa.0.08285.i, !dbg !10443
  %1026 = fadd <8 x float> %1025, %1018, !dbg !10448
  %1027 = fmul <8 x float> %_157.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.35.sroa.0.08285.i, !dbg !10453
  %1028 = fadd <8 x float> %1027, %1020, !dbg !10458
  %1029 = fmul <8 x float> %_160.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.35.sroa.0.08285.i, !dbg !10463
  %1030 = fadd <8 x float> %1029, %1022, !dbg !10468
  %1031 = fmul <8 x float> %_165.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.38.sroa.0.08284.i, !dbg !10473
  %1032 = fadd <8 x float> %1031, %1024, !dbg !10478
  %1033 = fmul <8 x float> %_168.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.38.sroa.0.08284.i, !dbg !10483
  %1034 = fadd <8 x float> %1033, %1026, !dbg !10488
  %1035 = fmul <8 x float> %_171.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.38.sroa.0.08284.i, !dbg !10493
  %1036 = fadd <8 x float> %1035, %1028, !dbg !10498
  %1037 = fmul <8 x float> %_174.i.i.i.i.sroa.0.0.copyload.i, %history.i.i.sroa.38.sroa.0.08284.i, !dbg !10503
  %1038 = fadd <8 x float> %1037, %1030, !dbg !10508
  %1039 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1032), !dbg !10513
  %1040 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %942, <8 x float> %1039), !dbg !10519
  %1041 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1034), !dbg !10513
  %1042 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1040, <8 x float> %1041), !dbg !10519
  %1043 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1036), !dbg !10513
  %1044 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1042, <8 x float> %1043), !dbg !10519
  %1045 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1038), !dbg !10513
  %1046 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1044, <8 x float> %1045), !dbg !10519
  %_39.i.i.idx.i = shl i64 %iter.sroa.0.0.i.i8290.i, 5, !dbg !10524
  %_39.i.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i.i, i64 %_39.i.i.idx.i, !dbg !10524
  store <8 x float> %1046, ptr %_39.i.i.i, align 4, !dbg !10529, !alias.scope !10534, !noalias !10538
  %exitcond9643.not.i = icmp eq i64 %941, %umax9642.i, !dbg !9983
  br i1 %exitcond9643.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, label %bb5.i.i.i, !dbg !9988

_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i: ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3585.i
  br label %bb17.i.i, !dbg !9967

bb17.i.i:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, %bb25.i.i
  %.lcssa83458389.i = phi <8 x float> [ %.lcssa83458388.i, %bb25.i.i ], [ %.lcssa83458388.lcssa8484.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %storemerge.i.i.lcssa83298369.i = phi i32 [ %storemerge.i.i.lcssa83298368.i, %bb25.i.i ], [ %storemerge.i.i.lcssa83298368.lcssa8469.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %frame.sroa.0.0.i8363.i = phi i64 [ %_62.i.i, %bb25.i.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %main_cursor.sroa.0.1.i8362.i = phi i64 [ %main_cursor.sroa.0.2.i.i, %bb25.i.i ], [ %main_cursor.sroa.0.0.i8438.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %ring_cursor.sroa.0.1.i8361.i = phi i64 [ %ring_cursor.sroa.0.2.i.i, %bb25.i.i ], [ %ring_cursor.sroa.0.0.i8437.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %minimum.i.i.sroa.0.08307.lcssa83498360.i = phi <8 x float> [ %minimum.i.i.sroa.0.08307.lcssa.i, %bb25.i.i ], [ %minimum.i.i.sroa.0.08307.lcssa8349.lcssa84208435.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter14detector_chunkNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i ]
  %_46.i.i = sub nuw nsw i64 %..i3974.i, %frame.sroa.0.0.i8363.i, !dbg !10542
  %ring.i1369.i = load i64, ptr %880, align 8, !dbg !10543, !alias.scope !10545, !noalias !10548, !noundef !12
  %main.i1370.i = load i64, ptr %881, align 8, !dbg !10552, !alias.scope !10545, !noalias !10548, !noundef !12
  %_10.i1371.i = add i64 %ring_cursor.sroa.0.1.i8361.i, 1, !dbg !10553
  %_45.not.i1372.i = icmp ult i64 %_10.i1371.i, %ring.i1369.i, !dbg !10554
  %1047 = select i1 %_45.not.i1372.i, i64 0, i64 %ring.i1369.i, !dbg !10554
  %start1.sroa.0.0.i1373.i = sub nuw i64 %_10.i1371.i, %1047, !dbg !10554
  %_12.i1375.i = add i64 %ring_cursor.sroa.0.1.i8361.i, %_48.i.sroa.3.0.copyload.i, !dbg !10556
  %_46.not.i1376.i = icmp ult i64 %_12.i1375.i, %ring.i1369.i, !dbg !10557
  %1048 = select i1 %_46.not.i1376.i, i64 0, i64 %ring.i1369.i, !dbg !10557
  %left_end.sroa.0.0.i1377.i = sub nuw i64 %_12.i1375.i, %1048, !dbg !10557
  %_18.i1383.i = add i64 %ring_cursor.sroa.0.1.i8361.i, %_48.i.sroa.4.0.copyload.i, !dbg !10559
  %_48.not.i1384.i = icmp ult i64 %_18.i1383.i, %ring.i1369.i, !dbg !10560
  %1049 = select i1 %_48.not.i1384.i, i64 0, i64 %ring.i1369.i, !dbg !10560
  %left_expiring.sroa.0.0.i1385.i = sub nuw i64 %_18.i1383.i, %1049, !dbg !10560
  %_30.i1390.i = sub i64 %ring.i1369.i, %ring_cursor.sroa.0.1.i8361.i, !dbg !10562
  %..i3987.i = tail call noundef i64 @llvm.umin.i64(i64 %_30.i1390.i, i64 %_46.i.i), !dbg !10563
  %_31.i1392.i = sub i64 %main.i1370.i, %main_cursor.sroa.0.1.i8362.i, !dbg !10565
  %..i3988.i = tail call noundef i64 @llvm.umin.i64(i64 %_31.i1392.i, i64 %..i3987.i), !dbg !10566
  %_32.i1394.i = sub i64 %ring.i1369.i, %start1.sroa.0.0.i1373.i, !dbg !10568
  %..i3989.i = tail call noundef i64 @llvm.umin.i64(i64 %_32.i1394.i, i64 %..i3988.i), !dbg !10569
  %_34.i1396.i = sub i64 %ring.i1369.i, %left_end.sroa.0.0.i1377.i, !dbg !10571
  %..i3990.i = tail call noundef i64 @llvm.umin.i64(i64 %_34.i1396.i, i64 %..i3989.i), !dbg !10572
  %_38.i1400.i = sub i64 %ring.i1369.i, %left_expiring.sroa.0.0.i1385.i, !dbg !10574
  %..i3992.i = tail call noundef i64 @llvm.umin.i64(i64 %_38.i1400.i, i64 %..i3990.i), !dbg !10575
  %_52.i.i = add i64 %frame.sroa.0.0.i8363.i, %iter2.sroa.0.0.i8439.i, !dbg !10577
  %base.i.i = shl i64 %_52.i.i, 3, !dbg !10577
  %base.i6809.i = add i64 %..i3992.i, %_52.i.i, !dbg !10580
  %_56.i.i = shl i64 %base.i6809.i, 3, !dbg !10580
  %_119.i.i = icmp ult i64 %_56.i.i, %base.i.i, !dbg !10583
  %_113.not.i.i = icmp ugt i64 %_56.i.i, %_39.1
  %or.cond.i.i = or i1 %_119.i.i, %_113.not.i.i, !dbg !10583
  br i1 %or.cond.i.i, label %bb41.i.i, label %bb39.i.i, !dbg !10583, !prof !165

bb39.i.i:                                         ; preds = %bb17.i.i
  %_122.i.i = getelementptr inbounds nuw float, ptr %_39.0, i64 %base.i.i, !dbg !10590
  %_62.i.i = add nuw nsw i64 %..i3992.i, %frame.sroa.0.0.i8363.i, !dbg !10594
  %_131.i.i.idx = shl nuw nsw i64 %frame.sroa.0.0.i8363.i, 5, !dbg !10596
  %_131.i.i = getelementptr inbounds nuw i8, ptr %peaks_left.i.i, i64 %_131.i.i.idx, !dbg !10596
  %_2.i.i.i40268314.not.i = icmp eq i64 %..i3992.i, 0, !dbg !10606
  br i1 %_2.i.i.i40268314.not.i, label %bb25.i.i, label %bb24.i.preheader.i, !dbg !10606

bb41.i.i:                                         ; preds = %bb17.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08288.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.29.sroa.0.08287.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.08286.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.08285.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.08284.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i.i, i64 noundef %_56.i.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_2cc46ad3620fe9ec5bf0f181ee950489) #30, !dbg !10612, !noalias !10613
  unreachable, !dbg !10612

bb24.i.preheader.i:                               ; preds = %bb39.i.i
  %umin9650.i = tail call i64 @llvm.umin.i64(i64 %_34.i1396.i, i64 %_38.i1400.i)
  %umin9651.i = tail call i64 @llvm.umin.i64(i64 %umin9650.i, i64 %_32.i1394.i)
  %umin9652.i = tail call i64 @llvm.umin.i64(i64 %umin9651.i, i64 %_30.i1390.i)
  %umin9653.i = tail call i64 @llvm.umin.i64(i64 %umin9652.i, i64 %_31.i1392.i)
  %1050 = sub nsw i64 %umin9654.i, %frame.sroa.0.0.i8363.i
  %umin9655.i = tail call i64 @llvm.umin.i64(i64 %umin9653.i, i64 %1050)
  %1051 = and i64 %umin9655.i, 2305843009213693951
  br label %bb24.i.i

bb24.i.i:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i, %bb24.i.preheader.i
  %_33.i17.i.sroa.0.0.copyload8333.i = phi <8 x float> [ %.lcssa83458389.i, %bb24.i.preheader.i ], [ %1076, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ]
  %storemerge.i.i8319.i = phi i32 [ %storemerge.i.i.lcssa83298369.i, %bb24.i.preheader.i ], [ %storemerge.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ]
  %iter.i.sroa.16.08317.i = phi i64 [ 0, %bb24.i.preheader.i ], [ %1052, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ]
  %minimum.i.i.sroa.0.083078315.i = phi <8 x float> [ %minimum.i.i.sroa.0.08307.lcssa83498360.i, %bb24.i.preheader.i ], [ %minimum.i.i.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ]
  %start1.i.i.i.i4034.i = shl i64 %iter.i.sroa.16.08317.i, 3, !dbg !10614
  %data.i.i.i.i4035.i = getelementptr inbounds nuw float, ptr %_122.i.i, i64 %start1.i.i.i.i4034.i, !dbg !10616
  %1052 = add nuw nsw i64 %iter.i.sroa.16.08317.i, 1, !dbg !10618
  %_133.i.i = add i64 %iter.i.sroa.16.08317.i, %ring_cursor.sroa.0.1.i8361.i, !dbg !10619
  %_134.i.i = add i64 %iter.i.sroa.16.08317.i, %main_cursor.sroa.0.1.i8362.i, !dbg !10628
  %_135.i.i = add i64 %iter.i.sroa.16.08317.i, %left_end.sroa.0.0.i1377.i, !dbg !10629
  %_136.i.i = add i64 %iter.i.sroa.16.08317.i, %start1.sroa.0.0.i1373.i, !dbg !10630
  %_137.i.i = add i64 %iter.i.sroa.16.08317.i, %left_expiring.sroa.0.0.i1385.i, !dbg !10631
  %base.i9.i.i.i = shl i64 %_133.i.i, 3, !dbg !10632
  %_7.i10.i.i.i = add i64 %base.i9.i.i.i, 8, !dbg !10635
  %1053 = or disjoint i64 %base.i9.i.i.i, 7, !dbg !10636
  %or.cond.i13.i.i.not.i = icmp ult i64 %1053, %_54.1.i.i.i, !dbg !10636
  br i1 %or.cond.i13.i.i.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i, label %bb4.i15.i.i.i, !dbg !10636, !prof !2723

bb4.i15.i.i.i:                                    ; preds = %bb24.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08288.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.29.sroa.0.08287.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.08286.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.08285.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.08284.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i.i, i64 noundef %_7.i10.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !10640, !noalias !10641
  unreachable, !dbg !10640

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i: ; preds = %bb24.i.i
  %data.i4.i.i.i4040.i = getelementptr inbounds nuw float, ptr %_131.i.i, i64 %start1.i.i.i.i4034.i, !dbg !10655
  %lanes.i3237.sroa.0.0.copyload.i = load <8 x float>, ptr %data.i4.i.i.i4040.i, align 4, !dbg !10658, !alias.scope !10663, !noalias !10667
  %1054 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %lanes.i3237.sroa.0.0.copyload.i, <8 x float> %lanes.i3237.sroa.0.0.copyload.i), !dbg !10671
  %1055 = select <8 x i1> %927, <8 x float> %1054, <8 x float> %lanes.i3237.sroa.0.0.copyload.i, !dbg !10676
  %1056 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1055, <8 x float> %_8.i.i.sroa.0.0.copyload.i, i8 30), !dbg !10681
  %1057 = bitcast <8 x float> %1056 to <8 x i32>, !dbg !10687
  %1058 = icmp slt <8 x i32> %1057, zeroinitializer, !dbg !10691
  %1059 = fdiv <8 x float> %_8.i.i.sroa.0.0.copyload.i, %1055, !dbg !10693
  %1060 = select <8 x i1> %1058, <8 x float> %1059, <8 x float> splat (float 1.000000e+00), !dbg !10691
  %_17.i14.i.i.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i, i64 %base.i9.i.i.i, !dbg !10698
  store <8 x float> %1060, ptr %_17.i14.i.i.i, align 4, !dbg !10700, !alias.scope !10705, !noalias !10709
  %base.i1192.i = shl i64 %_135.i.i, 3, !dbg !10713
  %1061 = or disjoint i64 %base.i1192.i, 7, !dbg !10716
  %or.cond.i1196.not.i = icmp ult i64 %1061, %_54.1.i.i.i, !dbg !10716
  br i1 %or.cond.i1196.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1200.i, label %bb4.i1199.i, !dbg !10716, !prof !2723

bb4.i1199.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08288.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.29.sroa.0.08287.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.08286.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.08285.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.08284.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  %_5.i1193.i = add i64 %base.i1192.i, 8, !dbg !10720
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1192.i, i64 noundef %_5.i1193.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !10721, !noalias !10722
  unreachable, !dbg !10721

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1200.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit16.i.i.i
  %_15.i1198.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i, i64 %base.i1192.i, !dbg !10730
  %lanes.i3008.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1198.i, align 4, !dbg !10732, !alias.scope !10737, !noalias !10741
  %position.i.i.i = zext i32 %storemerge.i.i8319.i to i64, !dbg !10745
  %1062 = icmp eq i32 %storemerge.i.i8319.i, 0, !dbg !10746
  br i1 %1062, label %bb5.i32.i.i, label %bb3.i.i.i, !dbg !10746

bb3.i.i.i:                                        ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1200.i
  %1063 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %minimum.i.i.sroa.0.083078315.i, <8 x float> %lanes.i3008.sroa.0.0.copyload.i), !dbg !10747
  br label %bb5.i32.i.i, !dbg !10752

bb5.i32.i.i:                                      ; preds = %bb3.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1200.i
  %minimum.i.i.sroa.0.0.i = phi <8 x float> [ %1063, %bb3.i.i.i ], [ %lanes.i3008.sroa.0.0.copyload.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1200.i ], !dbg !10753
  %_15.i33.i.i = add nuw nsw i64 %position.i.i.i, 1, !dbg !10754
  %complete.i.i.i = icmp eq i64 %_15.i33.i.i, %_18.i21.i.i, !dbg !10754
  br i1 %complete.i.i.i, label %bb19.i.i.i, label %bb7.i34.i.i, !dbg !10755

bb7.i34.i.i:                                      ; preds = %bb5.i32.i.i
  %base.i1183.i = shl i64 %_136.i.i, 3, !dbg !10756
  %1064 = or disjoint i64 %base.i1183.i, 7, !dbg !10758
  %or.cond.i1187.not.i = icmp ult i64 %1064, %_54.1.i.i.i, !dbg !10758
  br i1 %or.cond.i1187.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i, label %bb4.i1190.i, !dbg !10758, !prof !2723

bb4.i1190.i:                                      ; preds = %bb7.i34.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08288.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.29.sroa.0.08287.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.08286.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.08285.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.08284.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  %_5.i1184.i = add i64 %base.i1183.i, 8, !dbg !10762
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1183.i, i64 noundef %_5.i1184.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !10763, !noalias !10764
  unreachable, !dbg !10763

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i: ; preds = %bb7.i34.i.i
  %_15.i1189.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i, i64 %base.i1183.i, !dbg !10768
  %lanes.i3015.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1189.i, align 4, !dbg !10770, !alias.scope !10775, !noalias !10779
  %1065 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %lanes.i3015.sroa.0.0.copyload.i, <8 x float> %minimum.i.i.sroa.0.0.i), !dbg !10783
  %1066 = trunc i64 %_15.i33.i.i to i32, !dbg !10788
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, !dbg !10789

bb19.i.i.i:                                       ; preds = %bb5.i32.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i
  %end.sroa.0.0.i.i8306.i = phi i64 [ %1070, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ], [ %_135.i.i, %bb5.i32.i.i ]
  %iter.sroa.0.0.i35.i8305.i = phi i64 [ %_30.i36.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ], [ 0, %bb5.i32.i.i ]
  %suffix.i.i.sroa.0.08304.i = phi <8 x float> [ %1068, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ], [ %lanes.i3008.sroa.0.0.copyload.i, %bb5.i32.i.i ]
  %base.i1147.i = shl i64 %end.sroa.0.0.i.i8306.i, 3, !dbg !10790
  %1067 = or disjoint i64 %base.i1147.i, 7, !dbg !10792
  %or.cond.i1151.not.i = icmp ult i64 %1067, %_54.1.i.i.i, !dbg !10792
  br i1 %or.cond.i1151.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i, label %bb4.i1154.i, !dbg !10792, !prof !2723

bb4.i1154.i:                                      ; preds = %bb19.i.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08288.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.29.sroa.0.08287.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.08286.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.08285.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.08284.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  %_5.i1148.i = add i64 %base.i1147.i, 8, !dbg !10796
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1147.i, i64 noundef %_5.i1148.i, i64 noundef range(i64 0, 2305843009213693952) %_54.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !10797, !noalias !10798
  unreachable, !dbg !10797

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i: ; preds = %bb19.i.i.i
  %_30.i36.i.i = add nuw i64 %iter.sroa.0.0.i35.i8305.i, 1, !dbg !10802
  %_15.i1153.i = getelementptr inbounds nuw float, ptr %_54.0.i.i.i, i64 %base.i1147.i, !dbg !10807
  %lanes.i3043.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1153.i, align 4, !dbg !10809, !alias.scope !10814, !noalias !10818
  %1068 = tail call <8 x float> @llvm.x86.avx.min.ps.256(<8 x float> %suffix.i.i.sroa.0.08304.i, <8 x float> %lanes.i3043.sroa.0.0.copyload.i), !dbg !10822
  store <8 x float> %1068, ptr %_15.i1153.i, align 4, !dbg !10827, !alias.scope !10833, !noalias !10837
  %1069 = icmp eq i64 %end.sroa.0.0.i.i8306.i, 0, !dbg !10841
  %spec.store.select.i.i.i = select i1 %1069, i64 %ring.i.i, i64 %end.sroa.0.0.i.i8306.i, !dbg !10841
  %1070 = add i64 %spec.store.select.i.i.i, -1, !dbg !10842
  %exitcond9645.not.i = icmp eq i64 %_30.i36.i.i, %_18.i21.i.i, !dbg !10843
  br i1 %exitcond9645.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i, label %bb19.i.i.i, !dbg !10845

_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i
  %minimum.i.i.sroa.0.1.i = phi <8 x float> [ %1065, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i ], [ %minimum.i.i.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ], !dbg !10753
  %storemerge.i.i.i = phi i32 [ %1066, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1191.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1155.i ], !dbg !10846
  %1071 = fmul <8 x float> %minimum.i.i.sroa.0.1.i, splat (float 1.638400e+04), !dbg !10847
  %1072 = tail call <8 x float> @llvm.floor.v8f32(<8 x float> %1071), !dbg !10852
  %1073 = fmul <8 x float> %1072, splat (float 0x3F10000000000000), !dbg !10857
  %base.i1174.i = shl i64 %_137.i.i, 3, !dbg !10862
  %1074 = or disjoint i64 %base.i1174.i, 7, !dbg !10864
  %or.cond.i1178.not.i = icmp ult i64 %1074, %_56.1.i.i.i, !dbg !10864
  br i1 %or.cond.i1178.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i, label %bb4.i1181.i, !dbg !10864, !prof !2723

bb4.i1181.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08288.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.29.sroa.0.08287.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.08286.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.08285.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.08284.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  %_5.i1175.i = add i64 %base.i1174.i, 8, !dbg !10868
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1174.i, i64 noundef %_5.i1175.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !10869, !noalias !10870
  unreachable, !dbg !10869

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter23sliding_minimum_uniformNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i.i
  %_15.i1180.i = getelementptr inbounds nuw float, ptr %_56.0.i.i.i, i64 %base.i1174.i, !dbg !10874
  %lanes.i3022.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1180.i, align 4, !dbg !10876, !alias.scope !10881, !noalias !10885
  %1075 = fadd <8 x float> %_33.i17.i.sroa.0.0.copyload8333.i, %1073, !dbg !10889
  %1076 = fsub <8 x float> %1075, %lanes.i3022.sroa.0.0.copyload.i, !dbg !10894
  %_8.not.i4.i.i.i = icmp ugt i64 %_7.i10.i.i.i, %_56.1.i.i.i
  br i1 %_8.not.i4.i.i.i, label %bb4.i7.i.i.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i, !dbg !10899, !prof !165

bb4.i7.i.i.i:                                     ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08288.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.29.sroa.0.08287.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.08286.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.08285.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.08284.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i9.i.i.i, i64 noundef %_7.i10.i.i.i, i64 noundef range(i64 0, 2305843009213693952) %_56.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8421dc8ec9e43c41e5b11981eccec9a3) #30, !dbg !10904, !noalias !10905
  unreachable, !dbg !10904

_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1182.i
  %_17.i6.i.i.i = getelementptr inbounds nuw float, ptr %_56.0.i.i.i, i64 %base.i9.i.i.i, !dbg !10909
  store <8 x float> %1073, ptr %_17.i6.i.i.i, align 4, !dbg !10911, !alias.scope !10916, !noalias !10920
  %_41.i.i.sroa.0.0.copyload.i = load <8 x float>, ptr %934, align 32, !dbg !10924, !noalias !6249
  %1077 = fdiv <8 x float> %1076, %_37.i.i.sroa.0.0.copyload.i, !dbg !10925
  %1078 = fsub <8 x float> splat (float 1.000000e+00), %1077, !dbg !10930
  %1079 = fsub <8 x float> %1078, %_41.i.i.sroa.0.0.copyload.i, !dbg !10935
  %1080 = fmul <8 x float> %_9.i.i.sroa.0.0.copyload.i, %1079, !dbg !10940
  %1081 = fadd <8 x float> %_41.i.i.sroa.0.0.copyload.i, %1080, !dbg !10945
  %1082 = tail call <8 x float> @llvm.x86.avx.max.ps.256(<8 x float> %1078, <8 x float> %1081), !dbg !10949
  %1083 = bitcast <8 x float> %1082 to <8 x i32>, !dbg !10954
  %1084 = tail call <8 x float> @llvm.fabs.v8f32(<8 x float> %1082), !dbg !10960
  %1085 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1084, <8 x float> splat (float 0x3BC79CA100000000), i8 17), !dbg !10962
  %1086 = bitcast <8 x float> %1085 to <8 x i32>, !dbg !10968
  %1087 = xor <8 x i32> %1086, splat (i32 -1), !dbg !10974
  %1088 = and <8 x i32> %1087, %1083, !dbg !10976
  store <8 x i32> %1088, ptr %934, align 32, !dbg !10980, !noalias !6249
  %base.i1165.i = shl i64 %_134.i.i, 3, !dbg !10981
  %1089 = or disjoint i64 %base.i1165.i, 7, !dbg !10983
  %or.cond.i1169.not.i = icmp ult i64 %1089, %_58.1.i.i.i, !dbg !10983
  br i1 %or.cond.i1169.not.i, label %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i, label %bb4.i1172.i, !dbg !10983, !prof !2723

bb4.i1172.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i
  store <8 x float> %history.i.i.sroa.25.sroa.0.08288.i, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.29.sroa.0.08287.i, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.08286.i, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.08285.i, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.08284.i, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  %_5.i1166.i = add i64 %base.i1165.i, 8, !dbg !10987
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %base.i1165.i, i64 noundef %_5.i1166.i, i64 noundef range(i64 0, 2305843009213693952) %_58.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ed527a45f08d183090a42adb6b2bde3) #30, !dbg !10988, !noalias !10989
  unreachable, !dbg !10988

_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter15store_ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit8.i.i.i
  %1090 = bitcast <8 x i32> %1088 to <8 x float>, !dbg !10993
  %1091 = fsub <8 x float> splat (float 1.000000e+00), %1090, !dbg !10994
  %_15.i1171.i = getelementptr inbounds nuw float, ptr %_58.0.i.i.i, i64 %base.i1165.i, !dbg !10999
  %lanes.i3029.sroa.0.0.copyload.i = load <8 x float>, ptr %_15.i1171.i, align 4, !dbg !11001, !alias.scope !11006, !noalias !11010
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_15.i1171.i, ptr noundef nonnull align 4 dereferenceable(32) %data.i.i.i.i4035.i, i64 32, i1 false), !dbg !11014
  %1092 = fmul <8 x float> %1091, %lanes.i3029.sroa.0.0.copyload.i, !dbg !11020
  %1093 = select <8 x i1> %938, <8 x float> %lanes.i3029.sroa.0.0.copyload.i, <8 x float> %1092, !dbg !11025
  store <8 x float> %1093, ptr %data.i.i.i.i4035.i, align 4, !dbg !11030, !alias.scope !11035, !noalias !11039
  %exitcond9656.not.i = icmp eq i64 %1052, %1051, !dbg !10606
  br i1 %exitcond9656.not.i, label %bb25.i.i, label %bb24.i.i, !dbg !10606

bb25.i.i:                                         ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i, %bb39.i.i
  %.lcssa83458388.i = phi <8 x float> [ %.lcssa83458389.i, %bb39.i.i ], [ %1076, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ]
  %storemerge.i.i.lcssa83298368.i = phi i32 [ %storemerge.i.i.lcssa83298369.i, %bb39.i.i ], [ %storemerge.i.i.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ]
  %minimum.i.i.sroa.0.08307.lcssa.i = phi <8 x float> [ %minimum.i.i.sroa.0.08307.lcssa83498360.i, %bb39.i.i ], [ %minimum.i.i.sroa.0.0.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter9ring_laneNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit1173.i ]
  %_92.i.i = add i64 %..i3992.i, %ring_cursor.sroa.0.1.i8361.i, !dbg !11043
  %_132.not.i.i = icmp ult i64 %_92.i.i, %ring.i.i, !dbg !11044
  %1094 = select i1 %_132.not.i.i, i64 0, i64 %ring.i.i, !dbg !11044
  %ring_cursor.sroa.0.2.i.i = sub nuw i64 %_92.i.i, %1094, !dbg !11044
  %_94.i.i = add i64 %..i3992.i, %main_cursor.sroa.0.1.i8362.i, !dbg !11047
  %_138.not.i.i = icmp ult i64 %_94.i.i, %main.i.i, !dbg !11048
  %1095 = select i1 %_138.not.i.i, i64 0, i64 %main.i.i, !dbg !11048
  %main_cursor.sroa.0.2.i.i = sub nuw i64 %_94.i.i, %1095, !dbg !11048
  %_41.i.i = icmp ult i64 %_62.i.i, %..i3974.i, !dbg !9967
  br i1 %_41.i.i, label %bb17.i.i, label %bb16.i.bb13.i7.loopexit_crit_edge.i, !dbg !9967

bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i: ; preds = %bb13.i7.loopexit.i
  store <8 x float> %history.i.i.sroa.29.sroa.0.0.lcssa.i29, ptr %history.i.i.sroa.29.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.32.sroa.0.0.lcssa.i28, ptr %history.i.i.sroa.32.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.35.sroa.0.0.lcssa.i27, ptr %history.i.i.sroa.35.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.38.sroa.0.0.lcssa.i26, ptr %history.i.i.sroa.38.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.41.sroa.0.0.lcssa.i25, ptr %history.i.i.sroa.41.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002
  store <8 x float> %history.i.i.sroa.13.sroa.0.0.lcssa.i21, ptr %history.i.i.sroa.13.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002, !noalias !6249
  store <8 x float> %history.i.i.sroa.16.sroa.0.0.lcssa.i22, ptr %history.i.i.sroa.16.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002, !noalias !6249
  store <8 x float> %history.i.i.sroa.19.sroa.0.0.lcssa.i23, ptr %history.i.i.sroa.19.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002, !noalias !6249
  store <8 x float> %history.i.i.sroa.22.sroa.0.0.lcssa.i24, ptr %history.i.i.sroa.22.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002, !noalias !6249
  store <8 x float> %history.i.i.sroa.25.sroa.0.0.lcssa.i30, ptr %history.i.i.sroa.25.0.hot_left.i.sroa_idx.i, align 1, !dbg !10002, !noalias !6249
  store <8 x float> %history.i.i.sroa.10.sroa.0.0.lcssa.i20, ptr %history.i.i.sroa.10.0.hot_left.i.sroa_idx.i, align 32, !dbg !11050, !noalias !6249
  %1096 = trunc i64 %main_cursor.sroa.0.1.i.lcssa.i to i32, !dbg !11051
  %1097 = trunc i64 %ring_cursor.sroa.0.1.i.lcssa.i to i32, !dbg !11053
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !9943

_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i
  %left_phase.i.i = phi i32 [ %storemerge.i.i.lcssa83298368.lcssa8468.i, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %left_phase.i.pre.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], !dbg !9952
  %minimum.i.i.sroa.0.08307.lcssa8349.lcssa8420.lcssa.i = phi <8 x float> [ %minimum.i.i.sroa.0.08307.lcssa8349.lcssa.i, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %uniform_left.i.promoted.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ]
  %history.i.i.sroa.0.0.lcssa8407.lcssa.i = phi <8 x float> [ %history.i.i.sroa.0.0.lcssa.i31, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %hot_left.i.promoted.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ]
  %ring_cursor.sroa.0.0.i.lcssa.i = phi i32 [ %1097, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %_26.i.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], !dbg !9928
  %main_cursor.sroa.0.0.i.lcssa.i = phi i32 [ %1096, %bb13.i7._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], [ %_25.i.i, %bb3.i._RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit_crit_edge.i ], !dbg !9924
  store <8 x float> %history.i.i.sroa.0.0.lcssa8407.lcssa.i, ptr %hot_left.i.i, align 1, !dbg !11050, !noalias !6249
  store <8 x float> %minimum.i.i.sroa.0.08307.lcssa8349.lcssa8420.lcssa.i, ptr %uniform_left.i.i, align 1, !noalias !6249
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i.i, ptr noundef nonnull align 32 dereferenceable(32) %uniform_left.i.i, i64 32, i1 false), !dbg !11054, !noalias !6249
  call void @llvm.lifetime.end.p0(ptr nonnull %uniform_left.i.i), !dbg !11055, !noalias !9932
  %1098 = getelementptr inbounds nuw i8, ptr %self, i64 1736, !dbg !11056
  %_147.1.i.i = load i64, ptr %1098, align 8, !dbg !11056, !alias.scope !11057, !noalias !11058, !noundef !12
  %_8.i3577.i = icmp samesign ugt i64 %_147.1.i.i, 7, !dbg !11059
  br i1 %_8.i3577.i, label %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3580.i, label %bb2.i3578.i, !dbg !11059, !prof !651

bb2.i3578.i:                                      ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 8, i64 noundef range(i64 0, 2305843009213693952) %_147.1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae05af65618c8d83c932596374a1d80) #30, !dbg !11064, !noalias !11065
  unreachable, !dbg !11064

_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3580.i: ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter26limiter_block_uniform_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %1099 = getelementptr inbounds nuw i8, ptr %self, i64 1728, !dbg !11056
  %_147.0.i.i = load ptr, ptr %1099, align 8, !dbg !11056, !alias.scope !11057, !noalias !11058, !nonnull !12, !noundef !12
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(32) %_147.0.i.i, ptr noundef nonnull align 32 dereferenceable(32) %left_prefix.i.i, i64 32, i1 false), !dbg !11069, !noalias !5902
  %_148.0.i.i = load ptr, ptr %55, align 8, !dbg !11073, !alias.scope !11057, !noalias !11058, !nonnull !12, !noundef !12
  %_148.1.i.i = load i64, ptr %56, align 8, !dbg !11073, !alias.scope !11057, !noalias !11058, !noundef !12
  %1100 = tail call i1 @llvm.is.constant.i32(i32 %left_phase.i.i), !dbg !11074
  br i1 %1100, label %bb2.i4071.i, label %bb6.i4062.i, !dbg !11074

bb6.i4062.i:                                      ; preds = %bb2.i4071.i, %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3580.i
  %end_or_len.idx.i4063.i = shl nuw nsw i64 %_148.1.i.i, 2, !dbg !11078
  %end_or_len.i4064.i = getelementptr inbounds nuw i8, ptr %_148.0.i.i, i64 %end_or_len.idx.i4063.i, !dbg !11078
  %_293.i4065.i = icmp eq i64 %_148.1.i.i, 0, !dbg !11082
  br i1 %_293.i4065.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4077.i, label %bb10.i4066.i, !dbg !11085

bb2.i4071.i:                                      ; preds = %_RNvXNtCs1miJJANcyja_4lane5simd8NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8NtB4_4Lane5store.exit3580.i
  %bytes1.sroa.0.0.zext.i4072.i = and i32 %left_phase.i.i, 255, !dbg !11086
  %bytes1.sroa.0.0.isplat.i4073.i = mul nuw i32 %bytes1.sroa.0.0.zext.i4072.i, 16843009, !dbg !11086
  %_5.i4074.i = icmp eq i32 %left_phase.i.i, %bytes1.sroa.0.0.isplat.i4073.i, !dbg !11087
  br i1 %_5.i4074.i, label %bb3.i4075.i, label %bb6.i4062.i, !dbg !11087

bb3.i4075.i:                                      ; preds = %bb2.i4071.i
  %bytes.sroa.0.0.extract.trunc.i4076.i = trunc i32 %left_phase.i.i to i8, !dbg !11088
  %1101 = shl nuw nsw i64 %_148.1.i.i, 2, !dbg !11090
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_148.0.i.i, i8 %bytes.sroa.0.0.extract.trunc.i4076.i, i64 %1101, i1 false), !dbg !11090, !alias.scope !11091, !noalias !10613
  br label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4077.i, !dbg !11094

bb10.i4066.i:                                     ; preds = %bb6.i4062.i, %bb10.i4066.i
  %iter.sroa.0.04.i4067.i = phi ptr [ %_38.i4068.i, %bb10.i4066.i ], [ %_148.0.i.i, %bb6.i4062.i ]
  %_38.i4068.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.04.i4067.i, i64 4, !dbg !11095
  store i32 %left_phase.i.i, ptr %iter.sroa.0.04.i4067.i, align 4, !dbg !11097, !alias.scope !11091, !noalias !10613
  %_29.i4069.i = icmp eq ptr %_38.i4068.i, %end_or_len.i4064.i, !dbg !11082
  br i1 %_29.i4069.i, label %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4077.i, label %bb10.i4066.i, !dbg !11085

_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4077.i: ; preds = %bb10.i4066.i, %bb3.i4075.i, %bb6.i4062.i
; call <true_peak_limiter::HotChannel<wide::f32x8_::f32x8>>::store
  call fastcc void @_RNvMs5_CsdvPQf9CMsz3_17true_peak_limiterINtB5_10HotChannelNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E5storeB5_(ptr noalias noundef readonly align 32 captures(none) dereferenceable(736) %hot_left.i.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25) #31, !dbg !11098, !noalias !5902
  store i32 %main_cursor.sroa.0.0.i.lcssa.i, ptr %_25.i, align 4, !dbg !11051, !alias.scope !9926, !noalias !9927
  store i32 %ring_cursor.sroa.0.0.i.lcssa.i, ptr %882, align 4, !dbg !11053, !alias.scope !9926, !noalias !9927
  call void @llvm.lifetime.end.p0(ptr nonnull %peaks_left.i.i), !dbg !11099, !noalias !9932
  br label %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, !dbg !9898

_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i: ; preds = %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit4077.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh1_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i, %_RNvXs4_NtNtCs4NRVxsYgnAr_4core5slice10specializeSmINtB5_8SpecFillmE9spec_fill.exit.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter27limiter_block_per_lane_monoKh2_NtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  br i1 %quiet.sroa.0.06732.i, label %bb18.i, label %bb24.i, !dbg !11100

bb13.i:                                           ; preds = %bb11.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11101), !dbg !11104
  %1102 = getelementptr inbounds nuw i8, ptr %self, i64 1760, !dbg !11105
  %_40.0.i.i = load ptr, ptr %1102, align 8, !dbg !11105, !alias.scope !11107, !noalias !5902, !nonnull !12, !noundef !12
  %1103 = getelementptr inbounds nuw i8, ptr %self, i64 1768, !dbg !11105
  %_40.1.i.i = load i64, ptr %1103, align 8, !dbg !11105, !alias.scope !11107, !noalias !5902, !noundef !12
  %1104 = getelementptr inbounds nuw i8, ptr %self, i64 1824, !dbg !11108
  %_41.0.i.i = load ptr, ptr %1104, align 8, !dbg !11108, !alias.scope !11107, !noalias !5902, !nonnull !12, !noundef !12
  %1105 = getelementptr inbounds nuw i8, ptr %self, i64 1832, !dbg !11108
  %_41.1.i.i = load i64, ptr %1105, align 8, !dbg !11108, !alias.scope !11107, !noalias !5902, !noundef !12
  %..i.i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 %_41.1.i.i, i64 %_40.1.i.i), !dbg !11109
  %_2.i6.not.i.i = icmp eq i64 %..i.i.i.i.i, 0, !dbg !11115
  br i1 %_2.i6.not.i.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i, label %bb4.i4078.i, !dbg !11115

bb4.i4078.i:                                      ; preds = %bb13.i, %bb6.i4079.i
  %iter.sroa.8.07.i.i = phi i64 [ %1106, %bb6.i4079.i ], [ 0, %bb13.i ]
  %_3.i1.i.i.i = getelementptr inbounds nuw %LaneShape, ptr %_41.0.i.i, i64 %iter.sroa.8.07.i.i, !dbg !11118
  %_14.i.i = load i32, ptr %_3.i1.i.i.i, align 4, !dbg !11121, !noalias !11122, !noundef !12
  %_20.i.i = icmp eq i32 %_14.i.i, 0, !dbg !11123
  br i1 %_20.i.i, label %panic.i4088.i, label %bb6.i4079.i, !dbg !11123

bb6.i4079.i:                                      ; preds = %bb4.i4078.i
  %_3.i.i.i4080.i = getelementptr inbounds nuw i32, ptr %_40.0.i.i, i64 %iter.sroa.8.07.i.i, !dbg !11124
  %1106 = add nuw i64 %iter.sroa.8.07.i.i, 1, !dbg !11127
  %window.i4081.i = zext i32 %_14.i.i to i64, !dbg !11121
  %_18.i4082.i = load i32, ptr %_3.i.i.i4080.i, align 4, !dbg !11128, !noalias !11122, !noundef !12
  %_17.i4083.i = zext i32 %_18.i4082.i to i64, !dbg !11128
  %_19.i4084.i = urem i64 %_31, %window.i4081.i, !dbg !11123
  %_16.i4085.i = add nuw nsw i64 %_19.i4084.i, %_17.i4083.i, !dbg !11129
  %_15.i4086.i = urem i64 %_16.i4085.i, %window.i4081.i, !dbg !11130
  %1107 = trunc nuw i64 %_15.i4086.i to i32, !dbg !11131
  store i32 %1107, ptr %_3.i.i.i4080.i, align 4, !dbg !11131, !noalias !11122
  %exitcond.not.i.i = icmp eq i64 %1106, %..i.i.i.i.i, !dbg !11115
  br i1 %exitcond.not.i.i, label %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i, label %bb4.i4078.i, !dbg !11115

panic.i4088.i:                                    ; preds = %bb4.i4078.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bac57976a2bdbfad4a3a85d5d1c7648c) #30, !dbg !11123, !noalias !11122
  unreachable, !dbg !11123

_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i: ; preds = %bb6.i4079.i, %bb13.i
  %1108 = getelementptr inbounds nuw i8, ptr %self, i64 1624, !dbg !11132
  %_20.val.i = load i64, ptr %1108, align 8, !dbg !11132, !alias.scope !5901, !noalias !5902
  %1109 = getelementptr inbounds nuw i8, ptr %self, i64 1632, !dbg !11132
  %_20.val3729.i = load i64, ptr %1109, align 8, !dbg !11132, !alias.scope !5901, !noalias !5902, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11133), !dbg !11132
  %_10.i4089.i = icmp eq i64 %_20.val3729.i, 0, !dbg !11136
  br i1 %_10.i4089.i, label %panic.i4103.i, label %bb1.i4090.i, !dbg !11136

bb1.i4090.i:                                      ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i
  %_19.i = getelementptr inbounds nuw i8, ptr %self, i64 1640, !dbg !11138
  %_7.i4091.i = load i32, ptr %_19.i, align 4, !dbg !11139, !alias.scope !11140, !noalias !5902, !noundef !12
  %_6.i4092.i = zext i32 %_7.i4091.i to i64, !dbg !11139
  %_8.i4093.i = urem i64 %_31, %_20.val3729.i, !dbg !11136
  %_5.i4094.i = add nuw nsw i64 %_8.i4093.i, %_6.i4092.i, !dbg !11141
  %_4.i4095.i = urem i64 %_5.i4094.i, %_20.val3729.i, !dbg !11142
  %1110 = trunc i64 %_4.i4095.i to i32, !dbg !11143
  store i32 %1110, ptr %_19.i, align 4, !dbg !11143, !alias.scope !11140, !noalias !5902
  %_17.i4096.i = icmp eq i64 %_20.val.i, 0, !dbg !11144
  br i1 %_17.i4096.i, label %panic2.i.i, label %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i, !dbg !11144

panic.i4103.i:                                    ; preds = %_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState18advance_rest_phase.exit.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f0ee36f67d9a332211aa5518dd2ebfd5) #30, !dbg !11136, !noalias !11145
  unreachable, !dbg !11136

panic2.i.i:                                       ; preds = %bb1.i4090.i
; call core::panicking::panic_const::panic_const_rem_by_zero
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core9panicking11panic_const23panic_const_rem_by_zero(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33d4d33e0a850133578789055882dcf9) #30, !dbg !11144, !noalias !11145
  unreachable, !dbg !11144

_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i: ; preds = %bb1.i4090.i
  %1111 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !11146
  %_14.i4098.i = load i32, ptr %1111, align 4, !dbg !11146, !alias.scope !11140, !noalias !5902, !noundef !12
  %_13.i4099.i = zext i32 %_14.i4098.i to i64, !dbg !11146
  %_15.i4100.i = urem i64 %_31, %_20.val.i, !dbg !11144
  %_12.i4101.i = add nuw nsw i64 %_15.i4100.i, %_13.i4099.i, !dbg !11147
  %_11.i4102.i = urem i64 %_12.i4101.i, %_20.val.i, !dbg !11148
  %1112 = trunc i64 %_11.i4102.i to i32, !dbg !11149
  store i32 %1112, ptr %1111, align 4, !dbg !11149, !alias.scope !11140, !noalias !5902
  br label %_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit, !dbg !11150

bb18.i:                                           ; preds = %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
; call <true_peak_limiter::ChannelState>::is_at_silent_rest
  %_27.i = tail call noundef zeroext i1 @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17is_at_silent_rest(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(200) %_25) #31, !dbg !11151, !noalias !5902
  br i1 %_27.i, label %bb20.i, label %bb24.i, !dbg !11152

bb20.i:                                           ; preds = %bb18.i
  %_59.not.i = icmp samesign ugt i64 %words.i, %_39.1
  br i1 %_59.not.i, label %bb40.i, label %bb1.i4109.i, !dbg !11153, !prof !165

bb24.i:                                           ; preds = %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4125.i, %bb18.i, %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i
  %_26.sroa.0.0.i = phi i8 [ %1128, %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4125.i ], [ 0, %bb18.i ], [ 0, %_RINvCsdvPQf9CMsz3_17true_peak_limiter18limiter_block_monoNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8EB2_.exit.i ], !dbg !11161
  store i8 %_26.sroa.0.0.i, ptr %28, align 8, !dbg !11162, !alias.scope !5901, !noalias !5902
  %1113 = load i8, ptr %1180, align 32, !dbg !11163, !range !17, !alias.scope !5901, !noalias !5902, !noundef !12
  store i8 %1113, ptr %1178, align 1, !dbg !11164, !alias.scope !5901, !noalias !5902
  %1114 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> zeroinitializer, <8 x float> zeroinitializer, i8 0), !dbg !11165
  %fst_len.i4105.i = and i64 %_39.1, 2305843009213693944, !dbg !11174
  %1115 = bitcast <8 x float> %1114 to <8 x i32>, !dbg !11178
  %_22.not.i8485.i = icmp eq i64 %fst_len.i4105.i, 0, !dbg !11188
  br i1 %_22.not.i8485.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb13.i1206.i, !dbg !11188

bb13.i1206.i:                                     ; preds = %bb24.i, %bb13.i1206.i
  %iter.sroa.0.0.i12058488.i = phi ptr [ %_27.i.i, %bb13.i1206.i ], [ %_39.0, %bb24.i ]
  %iter.sroa.5.0.i8487.i = phi i64 [ %_28.i.i, %bb13.i1206.i ], [ %fst_len.i4105.i, %bb24.i ]
  %ok.i.sroa.0.08486.i = phi <8 x i32> [ %1120, %bb13.i1206.i ], [ %1115, %bb24.i ]
  %lanes.i.sroa.0.0.copyload.i = load <8 x i32>, ptr %iter.sroa.0.0.i12058488.i, align 4, !dbg !11194, !alias.scope !11199, !noalias !11203
  %_27.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i12058488.i, i64 32, !dbg !11207
  %_28.i.i = add i64 %iter.sroa.5.0.i8487.i, -8, !dbg !11214
  %1116 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i, splat (i32 2147483647), !dbg !11215
  %1117 = bitcast <8 x i32> %1116 to <8 x float>, !dbg !11221
  %1118 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1117, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !11222
  %1119 = bitcast <8 x float> %1118 to <8 x i32>, !dbg !11178
  %1120 = and <8 x i32> %ok.i.sroa.0.08486.i, %1119, !dbg !11228
  %_22.not.i.i = icmp eq i64 %_28.i.i, 0, !dbg !11188
  br i1 %_22.not.i.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread, label %bb13.i1206.i, !dbg !11188

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb24.i
  %1121 = icmp sgt <8 x i32> %1115, splat (i32 -1), !dbg !11230
  %1122 = bitcast <8 x i1> %1121 to i8, !dbg !11230
  %_0.i3704.not.i = icmp eq i8 %1122, 0, !dbg !11240
  br i1 %_0.i3704.not.i, label %_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit, !dbg !11241

_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread: ; preds = %bb13.i1206.i
  %1123 = icmp sgt <8 x i32> %1120, splat (i32 -1), !dbg !11230
  %1124 = bitcast <8 x i1> %1123 to i8, !dbg !11230
  %_0.i3704.not.i2033 = icmp eq i8 %1124, 0, !dbg !11240
  br i1 %_0.i3704.not.i2033, label %_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit, label %bb23.i, !dbg !11241

bb1.i4109.i:                                      ; preds = %bb20.i, %bb10.i4124.i
  %iter.sroa.6.0.i4110.i = phi i64 [ %len.i.i.i.i4115.i, %bb10.i4124.i ], [ %words.i, %bb20.i ], !dbg !11242
  %iter.sroa.0.0.i4111.i = phi ptr [ %data.i.i.i.i4114.i, %bb10.i4124.i ], [ %_39.0, %bb20.i ], !dbg !11242
  %1125 = icmp eq i64 %iter.sroa.6.0.i4110.i, 0, !dbg !11244
  br i1 %1125, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4125.i, label %bb11.preheader.i4112.i, !dbg !11244

bb11.preheader.i4112.i:                           ; preds = %bb1.i4109.i
  %..i.i.i4113.i = tail call noundef i64 @llvm.umin.i64(i64 %iter.sroa.6.0.i4110.i, i64 32), !dbg !11246
  %_18.idx.i4116.i = shl nuw nsw i64 %..i.i.i4113.i, 2, !dbg !11249
  %_18.i4117.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i4111.i, i64 %_18.idx.i4116.i, !dbg !11249
  br label %bb11.i4118.i, !dbg !11254

bb11.i4118.i:                                     ; preds = %bb11.i4118.i, %bb11.preheader.i4112.i
  %iter1.sroa.0.014.i4119.i = phi ptr [ %_31.i4121.i, %bb11.i4118.i ], [ %iter.sroa.0.0.i4111.i, %bb11.preheader.i4112.i ]
  %bits.sroa.0.013.i4120.i = phi i32 [ %1126, %bb11.i4118.i ], [ 0, %bb11.preheader.i4112.i ]
  %_31.i4121.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.014.i4119.i, i64 4, !dbg !11256
  %_134.i4122.i = load i32, ptr %iter1.sroa.0.014.i4119.i, align 4, !dbg !11258, !alias.scope !11259, !noalias !5901, !noundef !12
  %1126 = or i32 %_134.i4122.i, %bits.sroa.0.013.i4120.i, !dbg !11262
  %_25.i4123.i = icmp eq ptr %_31.i4121.i, %_18.i4117.i, !dbg !11263
  br i1 %_25.i4123.i, label %bb10.i4124.i, label %bb11.i4118.i, !dbg !11254

bb10.i4124.i:                                     ; preds = %bb11.i4118.i
  %data.i.i.i.i4114.i = getelementptr inbounds nuw float, ptr %iter.sroa.0.0.i4111.i, i64 %..i.i.i4113.i, !dbg !11265
  %len.i.i.i.i4115.i = sub nuw nsw i64 %iter.sroa.6.0.i4110.i, %..i.i.i4113.i, !dbg !11270
  %1127 = icmp eq i32 %1126, 0, !dbg !11271
  br i1 %1127, label %bb1.i4109.i, label %_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4125.i, !dbg !11271

_RNvNtCseSJggkR25Fr_14effect_runtime4bank22block_is_positive_zero.exit4125.i: ; preds = %bb10.i4124.i, %bb1.i4109.i
  %1128 = zext i1 %1125 to i8, !dbg !11162
  br label %bb24.i, !dbg !11100

bb40.i:                                           ; preds = %bb20.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %words.i, i64 noundef range(i64 0, 2305843009213693952) %_39.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_dceebfa5c8edf99352a4c26a81a69889) #30, !dbg !11272, !noalias !5902
  unreachable, !dbg !11272

bb23.i:                                           ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread, %bb23.i
  %iter.sroa.0.074.i = phi ptr [ %_45.i, %bb23.i ], [ %_39.0, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread ]
  %iter.sroa.5.073.i = phi i64 [ %_46.i, %bb23.i ], [ %fst_len.i4105.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread ]
  %ok.sroa.0.072.i = phi <8 x i32> [ %1133, %bb23.i ], [ %1115, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread ]
  %lanes.i.sroa.0.0.copyload.i6 = load <8 x i32>, ptr %iter.sroa.0.074.i, align 4, !dbg !11273, !alias.scope !11284, !noalias !11290
  %_45.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074.i, i64 32, !dbg !11294
  %_46.i = add i64 %iter.sroa.5.073.i, -8, !dbg !11306
  %1129 = and <8 x i32> %lanes.i.sroa.0.0.copyload.i6, splat (i32 2147483647), !dbg !11307
  %1130 = bitcast <8 x i32> %1129 to <8 x float>, !dbg !11314
  %1131 = tail call <8 x float> @llvm.x86.avx.cmp.ps.256(<8 x float> %1130, <8 x float> splat (float 0x46293E5940000000), i8 17), !dbg !11315
  %1132 = bitcast <8 x float> %1131 to <8 x i32>, !dbg !11321
  %1133 = and <8 x i32> %ok.sroa.0.072.i, %1132, !dbg !11325
  %_40.not.i = icmp eq i64 %_46.i, 0, !dbg !11327
  br i1 %_40.not.i, label %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit, label %bb23.i, !dbg !11327

_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit: ; preds = %bb23.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i
  %ok.sroa.0.0.lcssa.i = phi <8 x i32> [ %1115, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i ], [ %1133, %bb23.i ], !dbg !11328
  %1134 = icmp slt <8 x i32> %ok.sroa.0.0.lcssa.i, zeroinitializer, !dbg !11329
  %bc.i = select <8 x i1> %1134, <8 x i32> zeroinitializer, <8 x i32> splat (i32 1065353216), !dbg !11329
  %1135 = extractelement <8 x i32> %bc.i, i64 0, !dbg !11335
  %1136 = icmp ne i32 %1135, 0, !dbg !11335
  %1137 = zext i1 %1136 to i32, !dbg !11335
  %1138 = extractelement <8 x i32> %bc.i, i64 1, !dbg !11335
  %1139 = icmp eq i32 %1138, 0, !dbg !11335
  %1140 = select i1 %1139, i32 0, i32 2, !dbg !11335
  %mask.sroa.0.1.1.i = or disjoint i32 %1140, %1137, !dbg !11335
  %1141 = extractelement <8 x i32> %bc.i, i64 2, !dbg !11335
  %1142 = icmp eq i32 %1141, 0, !dbg !11335
  %1143 = select i1 %1142, i32 0, i32 4, !dbg !11335
  %mask.sroa.0.1.2.i = or disjoint i32 %mask.sroa.0.1.1.i, %1143, !dbg !11335
  %1144 = extractelement <8 x i32> %bc.i, i64 3, !dbg !11335
  %1145 = icmp eq i32 %1144, 0, !dbg !11335
  %1146 = select i1 %1145, i32 0, i32 8, !dbg !11335
  %mask.sroa.0.1.3.i = or disjoint i32 %mask.sroa.0.1.2.i, %1146, !dbg !11335
  %1147 = extractelement <8 x i32> %bc.i, i64 4, !dbg !11335
  %1148 = icmp eq i32 %1147, 0, !dbg !11335
  %1149 = select i1 %1148, i32 0, i32 16, !dbg !11335
  %mask.sroa.0.1.4.i = or disjoint i32 %mask.sroa.0.1.3.i, %1149, !dbg !11335
  %1150 = extractelement <8 x i32> %bc.i, i64 5, !dbg !11335
  %1151 = icmp eq i32 %1150, 0, !dbg !11335
  %1152 = select i1 %1151, i32 0, i32 32, !dbg !11335
  %mask.sroa.0.1.5.i = or disjoint i32 %mask.sroa.0.1.4.i, %1152, !dbg !11335
  %1153 = extractelement <8 x i32> %bc.i, i64 6, !dbg !11335
  %1154 = icmp eq i32 %1153, 0, !dbg !11335
  %1155 = select i1 %1154, i32 0, i32 64, !dbg !11335
  %mask.sroa.0.1.6.i = or i32 %mask.sroa.0.1.5.i, %1155, !dbg !11335
  %1156 = extractelement <8 x i32> %bc.i, i64 7, !dbg !11335
  %1157 = icmp eq i32 %1156, 0, !dbg !11335
  %1158 = select i1 %1157, i32 0, i32 128, !dbg !11335
  %mask.sroa.0.1.7.i = or i32 %mask.sroa.0.1.6.i, %1158, !dbg !11335
  %1159 = getelementptr inbounds nuw i8, ptr %self, i64 1568, !dbg !11339
  %1160 = getelementptr inbounds nuw i8, ptr %self, i64 1576, !dbg !11339
  store i32 %mask.sroa.0.1.7.i, ptr %1160, align 8, !dbg !11339, !alias.scope !5901, !noalias !5902
  %_36.i = load i64, ptr %1159, align 32, !dbg !11340, !alias.scope !5901, !noalias !5902, !noundef !12
  %1161 = tail call i64 @llvm.uadd.sat.i64(i64 %_36.i, i64 1), !dbg !11341
  store i64 %1161, ptr %1159, align 32, !dbg !11344, !alias.scope !5901, !noalias !5902
  %_222.i.i = icmp eq i64 %_39.1, 0, !dbg !11345
  br i1 %_222.i.i, label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, label %bb14.i4126.preheader.i, !dbg !11351

bb14.i4126.preheader.i:                           ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit
  %.idx.i.i = shl nuw nsw i64 %_39.1, 2, !dbg !11352
  tail call void @llvm.memset.p0.i64(ptr nonnull align 4 %_39.0, i8 0, i64 %.idx.i.i, i1 false), !dbg !11356, !alias.scope !11357, !noalias !5901
  br label %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i, !dbg !11360

_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i: ; preds = %bb14.i4126.preheader.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank19nonfinite_lane_maskNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit
  call void @llvm.lifetime.start.p0(ptr nonnull %shape.i), !dbg !11360, !noalias !6249
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %shape.i, ptr noundef nonnull align 16 dereferenceable(24) %_23.i, i64 24, i1 false), !dbg !11361, !noalias !5902
  %1162 = getelementptr inbounds nuw i8, ptr %self, i64 2120, !dbg !11362
  %rate.i = load i32, ptr %1162, align 8, !dbg !11362, !alias.scope !5901, !noalias !5902, !noundef !12
  %1163 = getelementptr inbounds nuw i8, ptr %self, i64 1584, !dbg !11364
  %_70.0.i = load ptr, ptr %1163, align 16, !dbg !11364, !alias.scope !5901, !noalias !5902, !nonnull !12, !noundef !12
  %1164 = getelementptr inbounds nuw i8, ptr %self, i64 1592, !dbg !11364
  %_70.1.i = load i64, ptr %1164, align 8, !dbg !11364, !alias.scope !5901, !noalias !5902, !noundef !12
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_70.0.i, i64 noundef %_70.1.i, i32 noundef %rate.i) #31, !dbg !11366, !noalias !5902
  %1165 = getelementptr inbounds nuw i8, ptr %self, i64 1600, !dbg !11367
  %_71.0.i = load ptr, ptr %1165, align 32, !dbg !11367, !alias.scope !5901, !noalias !5902, !nonnull !12, !noundef !12
  %1166 = getelementptr inbounds nuw i8, ptr %self, i64 1608, !dbg !11367
  %_71.1.i = load i64, ptr %1166, align 8, !dbg !11367, !alias.scope !5901, !noalias !5902, !noundef !12
; call <true_peak_limiter::ChannelState>::reset_to_defaults
  call void @_RNvMs2_CsdvPQf9CMsz3_17true_peak_limiterNtB5_12ChannelState17reset_to_defaults(ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %shape.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) %_71.0.i, i64 noundef %_71.1.i, i32 noundef %rate.i) #31, !dbg !11368, !noalias !5902
  store i32 0, ptr %_25.i, align 8, !dbg !11369, !alias.scope !5901, !noalias !5902
  %1167 = getelementptr inbounds nuw i8, ptr %self, i64 1644, !dbg !11369
  store i32 0, ptr %1167, align 4, !dbg !11369, !alias.scope !5901, !noalias !5902
  call void @llvm.lifetime.end.p0(ptr nonnull %shape.i), !dbg !11370, !noalias !6249
  br label %_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit, !dbg !11371

_RNvMs9_CsdvPQf9CMsz3_17true_peak_limiterINtB5_11LimiterCoreNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8E18process_block_monoB5_.exit: ; preds = %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i.thread, %_RNvMs1_CsdvPQf9CMsz3_17true_peak_limiterNtB5_7Cursors7advance.exit.i, %_RINvNtCseSJggkR25Fr_14effect_runtime4bank11check_blockNtNtCsdw0XaoyMD7p_4wide6f32x8_5f32x8ECsdvPQf9CMsz3_17true_peak_limiter.exit.i, %_RNvXs_NtNtCs4NRVxsYgnAr_4core5slice10specializeSfINtB4_8SpecFillfE9spec_fillCsdvPQf9CMsz3_17true_peak_limiter.exit.i
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i641.i), !dbg !11371
  call void @llvm.lifetime.end.p0(ptr nonnull %left_prefix.i228.i), !dbg !11371
  call void @llvm.lifetime.end.p0(ptr nonnull %left_prefix.i.i), !dbg !11371
  call void @llvm.lifetime.end.p0(ptr nonnull %hot_left.i.i), !dbg !11371
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(328) %_0, ptr noundef nonnull align 8 dereferenceable(328) %report, i64 328, i1 false), !dbg !11372
  call void @llvm.lifetime.end.p0(ptr nonnull %report), !dbg !11373
  ret void, !dbg !11374

bb4:                                              ; preds = %bb2
  %_14 = load i32, ptr %_38.0, align 4, !dbg !5875, !noundef !12
  %start1 = zext i32 %_14 to i64, !dbg !5875
  %exitcond2001.not = icmp eq i64 %_38.1, 1, !dbg !11375
  br i1 %exitcond2001.not, label %panic2, label %bb5, !dbg !11375

panic:                                            ; preds = %bb6.6, %bb6.5, %bb6.4, %bb6.3, %bb6.2, %bb6.1, %bb2
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %_38.1, i64 noundef %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8790d798703aa1c903d88be7cb1df9ea) #30, !dbg !5875
  unreachable, !dbg !5875

bb5:                                              ; preds = %bb4
  %1168 = getelementptr inbounds nuw i8, ptr %_38.0, i64 4, !dbg !11375
  %_18 = load i32, ptr %1168, align 4, !dbg !11375, !noundef !12
  %end = zext i32 %_18 to i64, !dbg !11375
  %_53 = icmp ult i32 %_18, %_14, !dbg !11377
  %_49.not = icmp ult i64 %_37.1, %end
  %or.cond = or i1 %_53, %_49.not, !dbg !11377
  br i1 %or.cond, label %bb16, label %bb4.1, !dbg !11377, !prof !165

panic2:                                           ; preds = %bb4.7, %bb4.6, %bb4.5, %bb4.4, %bb4.3, %bb4.2, %bb4.1, %bb4
  %.lcssa1992 = phi i64 [ 1, %bb4 ], [ 2, %bb4.1 ], [ 3, %bb4.2 ], [ 4, %bb4.3 ], [ 5, %bb4.4 ], [ 6, %bb4.5 ], [ 7, %bb4.6 ], [ 8, %bb4.7 ], !dbg !11385
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCs4NRVxsYgnAr_4core9panicking18panic_bounds_check(i64 noundef %.lcssa1992, i64 noundef %_38.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_55e533af89cecaf7e999abe53b851675) #30, !dbg !11375
  unreachable, !dbg !11375

bb16:                                             ; preds = %bb5.7, %bb5.6, %bb5.5, %bb5.4, %bb5.3, %bb5.2, %bb5.1, %bb5
  %end.lcssa = phi i64 [ %end, %bb5 ], [ %end.1, %bb5.1 ], [ %end.2, %bb5.2 ], [ %end.3, %bb5.3 ], [ %end.4, %bb5.4 ], [ %end.5, %bb5.5 ], [ %end.6, %bb5.6 ], [ %end.7, %bb5.7 ], !dbg !11375
  %start1.lcssa1998 = phi i64 [ %start1, %bb5 ], [ %start1.1, %bb5.1 ], [ %start1.2, %bb5.2 ], [ %start1.3, %bb5.3 ], [ %start1.4, %bb5.4 ], [ %start1.5, %bb5.5 ], [ %start1.6, %bb5.6 ], [ %start1.7, %bb5.7 ], !dbg !5875
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCs4NRVxsYgnAr_4core5slice5index16slice_index_fail(i64 noundef %start1.lcssa1998, i64 noundef %end.lcssa, i64 noundef %_37.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cdadbf7cf6c6abfb3682542d463ff330) #30, !dbg !11391
  unreachable, !dbg !11391

bb4.1:                                            ; preds = %bb5
  %_54 = sub nuw nsw i64 %end, %start1, !dbg !11392
  %_56 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1, !dbg !11393
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56, i64 noundef %_54, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 0, ptr noalias noundef nonnull align 8 dereferenceable(40) %report) #31, !dbg !11397
  %_14.1 = load i32, ptr %1168, align 4, !dbg !5875, !noundef !12
  %start1.1 = zext i32 %_14.1 to i64, !dbg !5875
  %exitcond2001.1.not = icmp eq i64 %9, 1, !dbg !11375
  br i1 %exitcond2001.1.not, label %panic2, label %bb5.1, !dbg !11375

bb5.1:                                            ; preds = %bb4.1
  %1169 = getelementptr inbounds nuw i8, ptr %_38.0, i64 8, !dbg !11375
  %_18.1 = load i32, ptr %1169, align 4, !dbg !11375, !noundef !12
  %end.1 = zext i32 %_18.1 to i64, !dbg !11375
  %_53.1 = icmp ult i32 %_18.1, %_14.1, !dbg !11377
  %_49.not.1 = icmp ult i64 %_37.1, %end.1
  %or.cond.1 = or i1 %_53.1, %_49.not.1, !dbg !11377
  br i1 %or.cond.1, label %bb16, label %bb6.1, !dbg !11377, !prof !165

bb6.1:                                            ; preds = %bb5.1
  %_54.1 = sub nuw nsw i64 %end.1, %start1.1, !dbg !11392
  %_56.1 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.1, !dbg !11393
  %_27.1 = getelementptr inbounds nuw i8, ptr %report, i64 40, !dbg !11398
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.1, i64 noundef %_54.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 1, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.1) #31, !dbg !11397
  %exitcond.2.not = icmp eq i64 %_38.1, 2, !dbg !5875
  br i1 %exitcond.2.not, label %panic, label %bb4.2, !dbg !5875

bb4.2:                                            ; preds = %bb6.1
  %_14.2 = load i32, ptr %1169, align 4, !dbg !5875, !noundef !12
  %start1.2 = zext i32 %_14.2 to i64, !dbg !5875
  %exitcond2001.2.not = icmp eq i64 %9, 2, !dbg !11375
  br i1 %exitcond2001.2.not, label %panic2, label %bb5.2, !dbg !11375

bb5.2:                                            ; preds = %bb4.2
  %1170 = getelementptr inbounds nuw i8, ptr %_38.0, i64 12, !dbg !11375
  %_18.2 = load i32, ptr %1170, align 4, !dbg !11375, !noundef !12
  %end.2 = zext i32 %_18.2 to i64, !dbg !11375
  %_53.2 = icmp ult i32 %_18.2, %_14.2, !dbg !11377
  %_49.not.2 = icmp ult i64 %_37.1, %end.2
  %or.cond.2 = or i1 %_53.2, %_49.not.2, !dbg !11377
  br i1 %or.cond.2, label %bb16, label %bb6.2, !dbg !11377, !prof !165

bb6.2:                                            ; preds = %bb5.2
  %_54.2 = sub nuw nsw i64 %end.2, %start1.2, !dbg !11392
  %_56.2 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.2, !dbg !11393
  %_27.2 = getelementptr inbounds nuw i8, ptr %report, i64 80, !dbg !11398
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.2, i64 noundef %_54.2, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 2, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.2) #31, !dbg !11397
  %exitcond.3.not = icmp eq i64 %_38.1, 3, !dbg !5875
  br i1 %exitcond.3.not, label %panic, label %bb4.3, !dbg !5875

bb4.3:                                            ; preds = %bb6.2
  %_14.3 = load i32, ptr %1170, align 4, !dbg !5875, !noundef !12
  %start1.3 = zext i32 %_14.3 to i64, !dbg !5875
  %exitcond2001.3.not = icmp eq i64 %9, 3, !dbg !11375
  br i1 %exitcond2001.3.not, label %panic2, label %bb5.3, !dbg !11375

bb5.3:                                            ; preds = %bb4.3
  %1171 = getelementptr inbounds nuw i8, ptr %_38.0, i64 16, !dbg !11375
  %_18.3 = load i32, ptr %1171, align 4, !dbg !11375, !noundef !12
  %end.3 = zext i32 %_18.3 to i64, !dbg !11375
  %_53.3 = icmp ult i32 %_18.3, %_14.3, !dbg !11377
  %_49.not.3 = icmp ult i64 %_37.1, %end.3
  %or.cond.3 = or i1 %_53.3, %_49.not.3, !dbg !11377
  br i1 %or.cond.3, label %bb16, label %bb6.3, !dbg !11377, !prof !165

bb6.3:                                            ; preds = %bb5.3
  %_54.3 = sub nuw nsw i64 %end.3, %start1.3, !dbg !11392
  %_56.3 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.3, !dbg !11393
  %_27.3 = getelementptr inbounds nuw i8, ptr %report, i64 120, !dbg !11398
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.3, i64 noundef %_54.3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 3, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.3) #31, !dbg !11397
  %exitcond.4.not = icmp eq i64 %_38.1, 4, !dbg !5875
  br i1 %exitcond.4.not, label %panic, label %bb4.4, !dbg !5875

bb4.4:                                            ; preds = %bb6.3
  %_14.4 = load i32, ptr %1171, align 4, !dbg !5875, !noundef !12
  %start1.4 = zext i32 %_14.4 to i64, !dbg !5875
  %exitcond2001.4.not = icmp eq i64 %9, 4, !dbg !11375
  br i1 %exitcond2001.4.not, label %panic2, label %bb5.4, !dbg !11375

bb5.4:                                            ; preds = %bb4.4
  %1172 = getelementptr inbounds nuw i8, ptr %_38.0, i64 20, !dbg !11375
  %_18.4 = load i32, ptr %1172, align 4, !dbg !11375, !noundef !12
  %end.4 = zext i32 %_18.4 to i64, !dbg !11375
  %_53.4 = icmp ult i32 %_18.4, %_14.4, !dbg !11377
  %_49.not.4 = icmp ult i64 %_37.1, %end.4
  %or.cond.4 = or i1 %_53.4, %_49.not.4, !dbg !11377
  br i1 %or.cond.4, label %bb16, label %bb6.4, !dbg !11377, !prof !165

bb6.4:                                            ; preds = %bb5.4
  %_54.4 = sub nuw nsw i64 %end.4, %start1.4, !dbg !11392
  %_56.4 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.4, !dbg !11393
  %_27.4 = getelementptr inbounds nuw i8, ptr %report, i64 160, !dbg !11398
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.4, i64 noundef %_54.4, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 4, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.4) #31, !dbg !11397
  %exitcond.5.not = icmp eq i64 %_38.1, 5, !dbg !5875
  br i1 %exitcond.5.not, label %panic, label %bb4.5, !dbg !5875

bb4.5:                                            ; preds = %bb6.4
  %_14.5 = load i32, ptr %1172, align 4, !dbg !5875, !noundef !12
  %start1.5 = zext i32 %_14.5 to i64, !dbg !5875
  %exitcond2001.5.not = icmp eq i64 %9, 5, !dbg !11375
  br i1 %exitcond2001.5.not, label %panic2, label %bb5.5, !dbg !11375

bb5.5:                                            ; preds = %bb4.5
  %1173 = getelementptr inbounds nuw i8, ptr %_38.0, i64 24, !dbg !11375
  %_18.5 = load i32, ptr %1173, align 4, !dbg !11375, !noundef !12
  %end.5 = zext i32 %_18.5 to i64, !dbg !11375
  %_53.5 = icmp ult i32 %_18.5, %_14.5, !dbg !11377
  %_49.not.5 = icmp ult i64 %_37.1, %end.5
  %or.cond.5 = or i1 %_53.5, %_49.not.5, !dbg !11377
  br i1 %or.cond.5, label %bb16, label %bb6.5, !dbg !11377, !prof !165

bb6.5:                                            ; preds = %bb5.5
  %_54.5 = sub nuw nsw i64 %end.5, %start1.5, !dbg !11392
  %_56.5 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.5, !dbg !11393
  %_27.5 = getelementptr inbounds nuw i8, ptr %report, i64 200, !dbg !11398
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.5, i64 noundef %_54.5, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 5, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.5) #31, !dbg !11397
  %exitcond.6.not = icmp eq i64 %_38.1, 6, !dbg !5875
  br i1 %exitcond.6.not, label %panic, label %bb4.6, !dbg !5875

bb4.6:                                            ; preds = %bb6.5
  %_14.6 = load i32, ptr %1173, align 4, !dbg !5875, !noundef !12
  %start1.6 = zext i32 %_14.6 to i64, !dbg !5875
  %exitcond2001.6.not = icmp eq i64 %9, 6, !dbg !11375
  br i1 %exitcond2001.6.not, label %panic2, label %bb5.6, !dbg !11375

bb5.6:                                            ; preds = %bb4.6
  %1174 = getelementptr inbounds nuw i8, ptr %_38.0, i64 28, !dbg !11375
  %_18.6 = load i32, ptr %1174, align 4, !dbg !11375, !noundef !12
  %end.6 = zext i32 %_18.6 to i64, !dbg !11375
  %_53.6 = icmp ult i32 %_18.6, %_14.6, !dbg !11377
  %_49.not.6 = icmp ult i64 %_37.1, %end.6
  %or.cond.6 = or i1 %_53.6, %_49.not.6, !dbg !11377
  br i1 %or.cond.6, label %bb16, label %bb6.6, !dbg !11377, !prof !165

bb6.6:                                            ; preds = %bb5.6
  %_54.6 = sub nuw nsw i64 %end.6, %start1.6, !dbg !11392
  %_56.6 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.6, !dbg !11393
  %_27.6 = getelementptr inbounds nuw i8, ptr %report, i64 240, !dbg !11398
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.6, i64 noundef %_54.6, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 6, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.6) #31, !dbg !11397
  %exitcond.7.not = icmp eq i64 %_38.1, 7, !dbg !5875
  br i1 %exitcond.7.not, label %panic, label %bb4.7, !dbg !5875

bb4.7:                                            ; preds = %bb6.6
  %_14.7 = load i32, ptr %1174, align 4, !dbg !5875, !noundef !12
  %start1.7 = zext i32 %_14.7 to i64, !dbg !5875
  %exitcond2001.7.not = icmp eq i64 %9, 7, !dbg !11375
  br i1 %exitcond2001.7.not, label %panic2, label %bb5.7, !dbg !11375

bb5.7:                                            ; preds = %bb4.7
  %1175 = getelementptr inbounds nuw i8, ptr %_38.0, i64 32, !dbg !11375
  %_18.7 = load i32, ptr %1175, align 4, !dbg !11375, !noundef !12
  %end.7 = zext i32 %_18.7 to i64, !dbg !11375
  %_53.7 = icmp ult i32 %_18.7, %_14.7, !dbg !11377
  %_49.not.7 = icmp ult i64 %_37.1, %end.7
  %or.cond.7 = or i1 %_53.7, %_49.not.7, !dbg !11377
  br i1 %or.cond.7, label %bb16, label %bb6.7, !dbg !11377, !prof !165

bb6.7:                                            ; preds = %bb5.7
  %_54.7 = sub nuw nsw i64 %end.7, %start1.7, !dbg !11392
  %_56.7 = getelementptr inbounds nuw %"effect_contract::PreparedAutomationSpan", ptr %_37.0, i64 %start1.7, !dbg !11393
  %_27.7 = getelementptr inbounds nuw i8, ptr %report, i64 280, !dbg !11398
; call true_peak_limiter::apply_automation
  call void @_RNvCsdvPQf9CMsz3_17true_peak_limiter16apply_automation(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) %_56.7, i64 noundef %_54.7, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(104) %_23, i64 noundef %_24, ptr noalias noundef nonnull align 8 dereferenceable(200) %_25, ptr noalias noundef nonnull align 8 dereferenceable(200) %_26, i64 noundef 7, ptr noalias noundef nonnull align 8 dereferenceable(40) %_27.7) #31, !dbg !11397
  %_39.0 = load ptr, ptr %block, align 8, !dbg !11399, !nonnull !12, !align !24, !noundef !12
  %1176 = getelementptr inbounds nuw i8, ptr %block, i64 8, !dbg !11399
  %_39.1 = load i64, ptr %1176, align 8, !dbg !11399, !noundef !12
  %1177 = getelementptr inbounds nuw i8, ptr %block, i64 104, !dbg !11400
  %_32 = load i32, ptr %1177, align 8, !dbg !11400, !noundef !12
  %_31 = zext i32 %_32 to i64, !dbg !11400
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5901), !dbg !11401
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5902), !dbg !11401
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i641.i), !dbg !11402
  call void @llvm.lifetime.start.p0(ptr nonnull %left_prefix.i228.i), !dbg !11402
  call void @llvm.lifetime.start.p0(ptr nonnull %left_prefix.i.i), !dbg !11402
  call void @llvm.lifetime.start.p0(ptr nonnull %hot_left.i.i), !dbg !11402
  %words.i = shl nuw nsw i64 %_31, 3, !dbg !11402
  %1178 = getelementptr inbounds nuw i8, ptr %self, i64 2153, !dbg !5899
  %1179 = load i8, ptr %1178, align 1, !dbg !5899, !range !17, !alias.scope !5901, !noalias !5902, !noundef !12
  %1180 = getelementptr inbounds nuw i8, ptr %self, i64 2144, !dbg !11403
  %1181 = load i8, ptr %1180, align 32, !dbg !11403, !range !17, !alias.scope !5901, !noalias !5902, !noundef !12
  %_6.i = icmp eq i8 %1179, %1181, !dbg !5899
  %1182 = getelementptr inbounds nuw i8, ptr %self, i64 1776
  %1183 = getelementptr inbounds nuw i8, ptr %self, i64 1784
  %_68.1.i = load i64, ptr %1183, align 8, !dbg !11404, !alias.scope !5901, !noalias !5902
  br i1 %_6.i, label %bb1.i, label %start.bb11.thread_crit_edge.i, !dbg !5899
}
